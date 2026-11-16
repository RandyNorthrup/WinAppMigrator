# Words of the installer that every built installer carries, which runs on the PC an app is moved to and speaks its
# language, and of the uninstaller it leaves behind for Settings. Message names start with installer- for the
# installer's window, installer-cli- for its command line, and uninstaller- for the uninstaller.
#
# The command lines (installer-cli-, uninstaller-cli-) are always shown in English, as scripts read what they print.
# Translate them all the same. Keep their command-line flags, such as --apply and --target-root, exactly as they are.


## The installer's window: its title and header

installer-window-title = WinAppMigrator-Installationsprogramm
# The window's title in the taskbar. { $app_name } is the name of the app it installs.
installer-window-title-app = { $app_name } installieren
installer-tagline = Installiert eine Anwendung erneut, die auf einem anderen PC erfasst wurde
# What an app is called when its package gives it no name.
installer-unnamed-app = Anwendung

## When the installer's window cannot open. Each is a paragraph of a message box.

# { $error } is what went wrong, as Windows or the graphics driver says it.
installer-window-failed = Das Fenster des Installationsprogramms konnte nicht geöffnet werden: { $error }
installer-window-failed-without-one = Die Installation ist auch ohne Fenster möglich: Führen Sie diese Datei in einer Eingabeaufforderung mit Administratorrechten mit --apply aus. Mit --target-root und einem Ordner legen Sie fest, wohin installiert wird.

## Checking the installer when it opens

installer-checking = Installationsprogramm wird überprüft…
installer-stub-no-package = Dies ist ein Installationsprogramm-Stub von WinAppMigrator, der kein Anwendungspaket enthält. Erstellen Sie mit WinAppMigrator ein Installationsprogramm, und führen Sie dann die erstellte .exe-Datei hier aus.
installer-check-stopped = Die Überprüfung des Installationsprogramms wurde abgebrochen.
# { $error } is what went wrong, as Windows says it.
installer-locate-failed = Das Installationsprogramm wurde nicht gefunden: { $error }
# { $error } is what went wrong reading it.
installer-read-failed = Das Installationsprogramm konnte nicht gelesen werden: { $error }
# { $error } is why, such as that the app needs a newer Windows.
installer-cannot-be-used-here = Das Paket dieses Installationsprogramms kann auf diesem PC nicht verwendet werden: { $error }

## The app, where it goes, and whether the installer is signed

installer-captured-from = Erfasst auf
installer-install-location = Installationsort
# The title of the folder picker.
installer-choose-location = Installationsort auswählen
installer-folder-has-files = Dieser Ordner enthält bereits Dateien. Dateien mit gleichem Namen werden ersetzt und bei einer Deinstallation wiederhergestellt.
installer-room-needed = Benötigter Speicherplatz
# { $shortfall } says which drive is short of room, such as "C: needs up to 824.0 MB for the app's files, and has 300.0 MB free".
installer-room-short = Hier ist möglicherweise nicht genügend Speicherplatz vorhanden: { $shortfall }. Das Paket kann auf einem anderen Laufwerk entpackt werden, aber die Dateien der App müssen trotzdem hierher passen.
# { $app_name } is the name of an app whose earlier install was cut short.
installer-interrupted = Eine frühere Installation von { $app_name } wurde nie abgeschlossen. „Installieren“ macht sie zuerst rückgängig.
# Beside "Installer", for whether it proves who made it.
installer-signature = Installationsprogramm
installer-signature-checking = Signatur wird überprüft…
installer-signature-unsigned = Nicht signiert: Herausgeber unbekannt
# { $publisher } is the name on the installer's certificate, such as "Fabrikam, Inc.".
installer-signature-signed-by = Signiert von { $publisher }
installer-signature-unnamed = Von einem unbenannten Herausgeber signiert
# { $reason } is Windows' own word for what is wrong, such as HashMismatch.
installer-signature-invalid = Die Signatur dieses Installationsprogramms ist ungültig ({ $reason }). Es wurde möglicherweise nach dem Signieren geändert.

## Why the install location cannot be used. Install and Preview Changes stay off while one of these is shown.

# { $app_name } is the name of the app being installed.
installer-location-app-already-there = { $app_name } ist bereits in diesem Ordner installiert, daher kann diese Kopie nicht hier installiert werden. Um sie hier zu installieren, deinstallieren Sie zuerst die Kopie auf diesem PC in den Windows-Einstellungen unter „Apps“, und öffnen Sie dieses Installationsprogramm dann erneut. Oder geben Sie oben einen neuen, leeren Ordner ein.
# { $other_app } is the name of another app Windows lists as installed in the folder. { $app_name } is the name of the app being installed.
installer-location-other-app-there = { $other_app } ist in diesem Ordner installiert, daher kann { $app_name } nicht hier installiert werden. Geben Sie oben einen neuen, leeren Ordner ein. Oder deinstallieren Sie { $other_app } zuerst in den Windows-Einstellungen unter „Apps“, falls Sie es nicht mehr benötigen, und öffnen Sie dieses Installationsprogramm dann erneut.
installer-location-has-files = Dieser Ordner enthält bereits Dateien, die nicht von WinAppMigrator installiert wurden. Diese Dateien gehören möglicherweise zu einer anderen App, daher wird nichts darüber installiert. Geben Sie oben einen neuen, leeren Ordner ein.
# { $reason } is why, such as that it cannot be the Windows folder.
installer-location-refused = Dieser Ordner kann nicht verwendet werden: { $reason }. Geben Sie oben einen anderen Ordner ein.

## What the install sets up beyond its files

installer-beyond-files = Was hierbei außer Dateien eingerichtet wird
# Under the heading above. It is part of what a person agrees to when they click Install.
installer-beyond-files-agreed = All dies geschieht mit Administratorrechten. Wenn die Installation fehlschlägt, werden ihre Änderungen rückgängig gemacht, und Sie werden über alles informiert, was sich nicht rückgängig machen ließ. Sie können die Installation auch später in den Einstellungen unter „Installierte Apps“ rückgängig machen. Ersetzte Dateien werden dabei wiederhergestellt, installierte Laufzeitkomponenten bleiben erhalten.
installer-beyond-files-none = Nichts: Es werden nur Dateien kopiert sowie Einstellungen in den eigenen Registrierungsschlüsseln der App.
# { $problem } is why, such as that the app cannot run on this PC.
installer-cannot-go-here = Dieses Paket kann nicht an diesem Ort installiert werden: { $problem }
installer-working-out = Es wird ermittelt, was hierbei an diesem Ort eingerichtet würde…
# Said as the reason after "This package cannot go to this location:".
installer-list-stopped = die Ermittlung, was dabei geschehen würde, wurde abgebrochen
# A heading over a list of what else the package asked for and will not get. { $count } is how many.
installer-more-left-out =
    { $count ->
        [one] { $count } weiteres Element wurde ausgelassen
       *[other] { $count } weitere Elemente wurden ausgelassen
    }

## Options

installer-options = Optionen
installer-user-data = Benutzerdaten
installer-user-data-detail = Ein: Einstellungen, Dokumente und Gespeicherte Spiele kommen in die üblichen Ordner des angemeldeten Benutzers. Aus: Alles wird unter dem Installationsordner abgelegt, und nichts davon wird so eingerichtet, dass es von dort ausgeführt wird.
# { $account } is the name of the account on the other PC whose data this is.
installer-user-data-whose = Dies sind die Daten von { $account }.
installer-license = Lizenz
installer-license-detail = Aktivierungs- und Lizenzdateien sowie die Registrierungswerte, in denen Lizenzen gespeichert sind. Einige müssen hier möglicherweise erneut aktiviert werden.

## Dependencies: what else the app needs, and how each is provided

installer-dependencies = Abhängigkeiten
installer-dependencies-detail = Was diese App benötigt und wie es jeweils bereitgestellt wird.
installer-dependencies-none = Es wurden keine zusätzlichen Laufzeitabhängigkeiten erkannt.
# { $feature } is the name of an optional part of Windows, such as NetFx3.
installer-dependency-windows-feature = Windows-Feature: { $feature }
installer-dependency-included = Enthalten
# { $id } is the package's name in winget, such as Microsoft.VCRedist.2015+.x64.
installer-dependency-winget = Wird mit winget installiert: { $id }
installer-dependency-manual = Manuell installieren
installer-dependency-built-in = In Windows integriert
installer-dependency-ships-with-app = Wird mit der App geliefert

## Buttons, and what is said beside them

installer-install = Installieren
installer-preview = Änderungsvorschau
installer-cancel = Abbrechen
installer-close = Schließen
installer-open-folder = Installationsordner öffnen
# { $error } is what went wrong, as Windows says it.
installer-open-folder-failed = Der Ordner konnte nicht geöffnet werden: { $error }
installer-location-blocks = „Installieren“ und „Änderungsvorschau“ sind deaktiviert, bis der Installationsort verwendet werden kann. Der Grund wird oben unter dem Installationsort angezeigt.
installer-package-blocks = „Installieren“ ist deaktiviert, da dieses Paket nicht an diesem Ort installiert werden kann. Der Grund wird oben angezeigt.
installer-list-changed = Was die Installation an diesem Ort tun würde, hat sich geändert, daher wurde nichts installiert. Prüfen Sie die Liste unten, und klicken Sie erneut auf „Installieren“, wenn sie Ihren Wünschen entspricht.
installer-cancelled-early = Abgebrochen. Es wurde nichts geändert.

## How far an install has got, beside its progress bar

# { $step } is one of the steps below. { $done } and { $total } are how many of its items are done, and how many there are.
installer-step-counted = { $step }: { $done } von { $total }…
# { $step } is one of the steps below, when it does not count its items.
installer-step-uncounted = { $step }…
installer-step-getting-ready = Wird vorbereitet…
installer-step-cancelling = Wird nach dem aktuellen Schritt abgebrochen…
installer-step-putting-back = Änderungen werden rückgängig gemacht
installer-step-unpacking = Paket wird entpackt
installer-step-checking-package = Paket wird überprüft
installer-step-recovering = Frühere, nie abgeschlossene Installation wird rückgängig gemacht
# What a preview is doing, whatever step it is on.
installer-step-previewing = Mögliche Änderungen werden ermittelt
installer-step-runtimes = Laufzeitkomponenten werden installiert
installer-step-files = Dateien werden kopiert
installer-step-services = Dienste werden eingerichtet
installer-step-drivers = Treiber werden eingerichtet
installer-step-registry = Registrierungseinstellungen werden geschrieben
installer-step-license = Lizenzdaten werden wiederhergestellt
installer-step-tasks = Geplante Aufgaben werden eingerichtet
installer-step-shortcuts = Verknüpfungen werden erstellt
installer-step-environment = Umgebungsvariablen werden festgelegt
installer-step-settings-entry = App wird zu den Einstellungen hinzugefügt
installer-step-verifying = Installation wird überprüft
installer-step-undoing = Wird rückgängig gemacht

## When an install cannot go ahead or stops

installer-carries-no-package = Dieses Installationsprogramm enthält kein Paket.
installer-stopped-unexpectedly = Die Installation wurde unerwartet beendet.
# { $reason } is what went wrong, as the program says it.
installer-stopped-unexpectedly-because = Die Installation wurde unerwartet beendet: { $reason }. Falls sie bereits begonnen hatte, macht ein erneut gestartetes Installationsprogramm zuerst rückgängig, was sie getan hat.
# { $error } is what went wrong reading it.
installer-package-read-failed = Das Paket konnte nicht gelesen werden: { $error }
installer-no-programdata = Der ProgramData-Ordner dieses PCs wurde nicht gefunden, daher wurde nichts installiert: Eine Installation legt dort die Aufzeichnung ab, mit der sie rückgängig gemacht wird.
# { $error } is why, such as that there is nowhere only administrators can get into.
installer-staging-failed = Es konnte kein Bereitstellungsordner erstellt werden: { $error }
# { $error } is what went wrong unpacking it.
installer-unpack-failed = Das Paket konnte nicht entpackt werden: { $error }
installer-staging-nowhere = es gibt keinen Ort zum Entpacken des Pakets, auf den nur Administratoren zugreifen können
# { $failure } is a folder and what went wrong there, such as "C:\ProgramData\WinAppMigrator: Access is denied.".
installer-staging-nowhere-look-at = es gibt keinen Ort zum Entpacken des Pakets, auf den nur Administratoren zugreifen können. Prüfen Sie diesen Ort: { $failure }

## What came of an install, or of a preview

installer-result = Installationsergebnis
installer-result-preview = Vorschau: Noch wurde nichts geändert
installer-result-installed = Die Anwendung wurde installiert und hat die Hashprüfung bestanden.
installer-result-installed-without-left-out = Die Anwendung wurde ohne die unten als ausgelassen aufgeführten Elemente installiert und hat die Hashprüfung bestanden.
installer-result-not-all-put-back = Die Installation ist fehlgeschlagen, und nicht alle Änderungen konnten rückgängig gemacht werden. Was übrig ist, wird unten aufgeführt. Die nächste Installation versucht es erneut.
installer-result-cancelled-put-back = Die Installation wurde abgebrochen, und ihre Änderungen wurden rückgängig gemacht.
installer-result-failed-put-back = Die Installation ist fehlgeschlagen, und ihre Änderungen wurden rückgängig gemacht.
installer-result-not-clean = Die Installation wurde nicht sauber abgeschlossen. Prüfen Sie die Probleme unten.
installer-result-changes-put-back = Rückgängig gemachte Änderungen
installer-result-changes-made = Vorgenommene Änderungen
installer-result-changes-planned = Geplante Änderungen
installer-result-warnings = Warnungen
installer-result-errors = Fehler
installer-result-package-valid = Paket-Hashes gültig
installer-result-package-invalid = Paket-Hashes UNGÜLTIG
installer-result-files-valid = Installierte Dateien gültig
installer-result-files-invalid = Installierte Dateien UNGÜLTIG
installer-result-reboot = Eine Abhängigkeit benötigt einen Neustart, um abgeschlossen zu werden. Starten Sie Windows neu, und starten Sie dann die App.
# A heading over the list of what was changed. { $count } is how many.
installer-result-changes = Änderungen ({ $count })
# After the first part of a long list. { $count } is how many more there are.
installer-result-changes-more = und { $count } weitere.
# Over the path of the install's log file.
installer-log = Protokoll

## The installer's command line. Shown in English: scripts read it. A { $count } is plain digits, with no separators.

installer-cli-stub = WinAppMigrator-Installationsprogramm-Stub – enthält kein Anwendungspaket.
installer-cli-stub-how = Erstellen Sie mit der WinAppMigrator-App ein Installationsprogramm, und führen Sie dann die erstellte .exe-Datei auf dem Zielcomputer aus.
installer-cli-usage-title = Selbstextrahierendes WinAppMigrator-Installationsprogramm.
installer-cli-usage = Ohne Argumente ausführen, um das grafische Installationsprogramm zu öffnen, oder:
installer-cli-usage-preview = Vorschau der Neuinstallation: --dry-run [--target-root <ORDNER>]
installer-cli-usage-apply = Neuinstallation:              --apply [--target-root <ORDNER>]
installer-cli-no-package = das Installationsprogramm enthält kein eingebettetes Paket
installer-cli-no-programdata = der ProgramData-Ordner dieses PCs wurde nicht gefunden, daher wurde nichts installiert: Eine Installation legt dort die Aufzeichnung ab, mit der sie rückgängig gemacht wird
# { $path } is the folder asked for, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-folder-has-files = { $path } kann nicht verwendet werden: { $problem }. Geben Sie --force nur an, wenn Sie wissen, dass es sich um die Dateien dieser App handelt
installer-cli-no-location = dieses Paket gibt nicht an, wo sich seine App befand, geben Sie daher --target-root an
# { $path } is where the app was on the other PC, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-location-refused = die App dieses Pakets befand sich in { $path }, und dieser Ort kann nicht verwendet werden: { $problem }. Geben Sie --target-root an
# { $summary } is how much room it needs where, such as "up to 824.0 MB on C: (356.0 GB free)".
installer-cli-room = Speicherplatz: { $summary }
# { $note } is something to know about the install.
installer-cli-note = Hinweis: { $note }
# { $count } is how many things it sets up beyond its files.
installer-cli-sets-up = Einzurichtende Elemente: { $count }
installer-cli-applied = Neuinstallation ausgeführt.
installer-cli-previewed = Vorschau der Neuinstallation erstellt.
# { $count } is how many changes the install logged.
installer-cli-changes-logged = Protokollierte Änderungen: { $count }
# { $count } is how many warnings.
installer-cli-warnings = Warnungen: { $count }
# { $count } is how many errors.
installer-cli-errors = Fehler: { $count }
# { $count } is how many things the package asked for and did not get.
installer-cli-refused = Abgelehnt: { $count }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-package-valid = Paket-Hashes gültig: { $valid }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-reinstall-valid = Hashes der Neuinstallation gültig: { $valid }
# { $id } is the install's id, such as wam-0f3a….
installer-cli-install-id = Installations-ID: { $id }
installer-cli-put-back = Die Installation ist fehlgeschlagen, daher wurden ihre Änderungen rückgängig gemacht.
# { $path } is the log file.
installer-cli-log = Protokoll: { $path }
# { $path } is the log file. Printed before an error.
installer-cli-error-log = Protokoll: { $path }
# { $error } is what went wrong.
installer-cli-error = Fehler: { $error }

## The uninstaller's window

uninstaller-window-title = Deinstallieren
# The window's title, and its heading. { $app_name } is the name of the app it uninstalls.
uninstaller-window-title-app = { $app_name } deinstallieren
uninstaller-tagline = Macht rückgängig, was das Installationsprogramm getan hat
uninstaller-reading = Aufzeichnung dieser Installation wird gelesen…
uninstaller-stopped-unexpectedly = Die Deinstallation wurde unerwartet beendet.
uninstaller-what-will-be-removed = Was entfernt wird
uninstaller-install-location = Installationsort
uninstaller-files = Dateien
uninstaller-registry-values = Registrierungswerte
uninstaller-services = Dienste
uninstaller-drivers = Treiber
uninstaller-scheduled-tasks = Geplante Aufgaben
uninstaller-environment = Umgebungsvariablen
uninstaller-plan-stays = Dateien, die Sie hinzugefügt oder geändert haben, bleiben erhalten, ebenso die Ordner, in denen sie sich befinden.
# { $count } is how many files the install replaced.
uninstaller-plan-coming-back =
    { $count ->
        [one] { $count } Datei, die diese Installation ersetzt hat, wird wiederhergestellt oder neben einer seitdem geänderten Datei abgelegt.
       *[other] { $count } Dateien, die diese Installation ersetzt hat, werden wiederhergestellt oder neben einer seitdem geänderten Datei abgelegt.
    }
# { $runtimes } is a list of runtimes, such as "Microsoft Visual C++ Redistributable, .NET Desktop Runtime 8".
uninstaller-plan-runtimes = Installierte Laufzeitkomponenten bleiben erhalten, da andere Programme sie möglicherweise verwenden: { $runtimes }.
uninstaller-options = Optionen
uninstaller-remove-user-data = Auch Benutzerdaten und Lizenzdaten entfernen
# { $count } is how many items of the app's settings, data, and license there are.
uninstaller-remove-user-data-detail =
    { $count ->
        [one] { $count } Element mit Einstellungen, Daten und Lizenz der App. Es bleibt erhalten, sofern diese Option nicht aktiviert ist. Was es ersetzt hat, wird wiederhergestellt oder daneben abgelegt, falls es sich seitdem geändert hat.
       *[other] { $count } Elemente mit Einstellungen, Daten und Lizenz der App. Diese bleiben erhalten, sofern diese Option nicht aktiviert ist. Was sie ersetzt haben, wird wiederhergestellt oder daneben abgelegt, falls sie sich seitdem geändert haben.
    }
uninstaller-uninstall = Deinstallieren
uninstaller-cancel = Abbrechen
uninstaller-close = Schließen
# { $done } and { $total } are how many of the install's items are undone, and how many there are.
uninstaller-step-counted = Installation wird rückgängig gemacht: { $done } von { $total }…
uninstaller-step = Installation wird rückgängig gemacht…
uninstaller-done = Die App wurde deinstalliert.
uninstaller-done-except = Die App wurde deinstalliert, mit Ausnahme der unten aufgeführten Elemente.
uninstaller-waits-for-drive = Ein Teil davon befindet sich auf einem nicht angeschlossenen Laufwerk. Schließen Sie es an, und deinstallieren Sie die App erneut über die Einstellungen.
uninstaller-not-all-undone = Nicht alles konnte rückgängig gemacht werden. Schließen Sie, was darauf zugreift, und deinstallieren Sie die App erneut über die Einstellungen.
# Over the path of the uninstall's log file.
uninstaller-log = Protokoll
uninstaller-undone = Rückgängig gemacht
uninstaller-left-in-place = Beibehalten
uninstaller-could-not-be-undone = Nicht rückgängig zu machen
# After the first part of a long list. { $count } is how many more there are.
uninstaller-and-more = und { $count } weitere.
# Said as the reason when there is no record of the install to read.
uninstaller-no-programdata = der ProgramData-Ordner dieses PCs wurde nicht gefunden, daher gibt es keine Aufzeichnung, die gelesen werden könnte

## When the uninstaller's window cannot open. Each is a paragraph of a message box, followed by a command to type.

uninstaller-window-failed-title = WinAppMigrator-Deinstallationsprogramm
# { $error } is what went wrong, as Windows or the graphics driver says it.
uninstaller-window-failed = Das Fenster des Deinstallationsprogramms konnte nicht geöffnet werden: { $error }
uninstaller-window-failed-quiet = Die Deinstallation ist auch ohne Fenster möglich, in einer Eingabeaufforderung mit Administratorrechten:
# Stands in for this program's own path in that command, if it cannot be found.
uninstaller-this-program = dieses Programm

## The uninstaller's command line (--quiet). Shown in English: scripts read it.

# { $app_name } is the name of the app.
uninstaller-cli-uninstalling = { $app_name } wird deinstalliert.
# { $path } is the log file.
uninstaller-cli-log = Protokoll: { $path }
# { $error } is what went wrong.
uninstaller-cli-error = Fehler: { $error }
