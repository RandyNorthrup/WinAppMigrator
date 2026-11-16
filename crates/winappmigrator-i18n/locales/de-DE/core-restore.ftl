# Words the core writes for people as it installs an app from a package: why an install will not start, the list of
# what it did (or for a preview, what it would do) that the installer window shows, its warnings, the record it keeps of
# each install so that it can be undone, and the core's errors. Message names start with core-.
#
# The list of what an install did is short and plain, one line for each thing, as the installer window lists them:
# "copied", "restored", "would copy". Words such as --force and --apply are command-line options, and LocalService,
# SYSTEM, ProgramData, and %AppData% are as Windows names them: keep them as they are.


## Errors: what went wrong, when the core cannot go on. Each is shown on its own, or after words that say what was being
## done.

core-error-list-changed = was diese Installation tun würde, hat sich seit der Zustimmung geändert, daher wurde nichts installiert
# { $path } is a file or folder, such as C:\Program Files\App\app.exe, or words that name one, such as "the installer".
# { $error } is what Windows said went wrong.
core-error-io = E/A-Vorgang fehlgeschlagen ({ $path }): { $error }
# { $detail } is what went wrong.
core-error-registry = Lesen der Registrierung fehlgeschlagen: { $detail }
# { $path } is a program file. { $detail } is what was wrong with it.
core-error-pe = PE-Analyse von { $path } fehlgeschlagen: { $detail }
# { $detail } is what was wrong, already in words.
core-error-invalid-input = ungültige Eingabe: { $detail }
# { $detail } is what went wrong.
core-error-serialization = Serialisierung des Berichts fehlgeschlagen: { $detail }
# { $detail } is what went wrong.
core-error-archive = Archivvorgang fehlgeschlagen: { $detail }
# { $detail } is what went wrong.
core-error-network = Netzwerkvorgang fehlgeschlagen: { $detail }
# { $detail } is what went wrong.
core-error-time-format = Zeitformatierung fehlgeschlagen: { $detail }
core-error-cancelled = der Vorgang wurde abgebrochen
# What is said when part of the program stopped unexpectedly and gave no reason.
core-error-panic-no-reason = es wurde kein Grund angegeben

## The record of each install, kept so that it can be undone.

# { $kind } is a number that says what type of registry value it is.
core-journal-unknown-value-type = unbekannter Registrierungswerttyp { $kind }
core-journal-not-elevated = nicht mit erhöhten Rechten ausgeführt
core-journal-needs-admin = für die Installation sind Administratorrechte erforderlich, damit ihre Aufzeichnung an einem Ort liegt, den andere nicht ändern können
# { $path } is a folder.
core-journal-not-an-install-folder = dies ist nicht der Ordner einer Installation: { $path }
# { $path } is the file the record is kept in.
core-journal-not-written = die Aufzeichnung dieser Installation konnte nicht geschrieben werden: { $path }
# { $path } is a folder.
core-journal-path-escapes-root = der Pfad führt aus dem vorgesehenen Stammordner heraus: { $path }
# { $path } is a folder.
core-journal-nothing-exists = nichts davon ist auf diesem PC vorhanden: { $path }
core-journal-busy = eine andere Installation oder Deinstallation wird ausgeführt. Warten Sie, bis sie abgeschlossen ist, und versuchen Sie es dann erneut
# Said of a file that was to get back the permissions it had.
core-journal-no-folder = die Datei hat keinen Ordner
# Said of the permissions a file had, which were kept to give back to it.
core-journal-permissions-unreadable = die gespeicherten Berechtigungen konnten nicht gelesen werden
# { $folder } is the folder of one install's record.
core-journal-unreadable-empty = die Aufzeichnung der Installation in { $folder } kann nicht verwendet werden, da sie leer ist
# { $folder } is the folder of one install's record.
core-journal-unreadable-no-header = die Aufzeichnung der Installation in { $folder } kann nicht verwendet werden, da sie nicht mit der Angabe beginnt, worauf sie sich bezieht
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-version = die Aufzeichnung der Installation in { $folder } kann nicht verwendet werden, da eine andere Version dieses Programms sie geschrieben hat
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-install = die Aufzeichnung der Installation in { $folder } kann nicht verwendet werden, da sie laut eigener Angabe zu einer anderen Installation gehört
# { $id } is an install's id, such as wam-1a2b3c.
core-journal-not-an-install-id = dies ist keine Installations-ID: { $id }

## Why an install will not start, or stopped before it changed anything.

core-restore-apply-and-dry-run = --apply und --dry-run können nicht gleichzeitig angegeben werden
# { $path } is the folder that was chosen to install in.
core-restore-location-not-full-path = der Installationsort muss ein vollständiger Pfad sein: „{ $path }“
# { $problem } says what is wrong with the folder, already in words. { $path } is the folder.
core-restore-location-unusable = dieser Installationsort kann nicht verwendet werden: { $problem }: „{ $path }“
core-restore-needs-admin-plan = der Wiederherstellungsplan enthält Änderungen an der Registrierung, an Diensten, geplanten Aufgaben, Verknüpfungen oder geschützten Zielen, die Administratorrechte erfordern
# { $path } is the folder that was chosen to install in.
core-restore-needs-admin-target = für den Zielordner sind erhöhte Rechte erforderlich: { $path }
core-restore-needs-admin-user-data = zum Wiederherstellen von Benutzerdaten in ProgramData oder an anderen geschützten Orten sind erhöhte Rechte erforderlich
# { $error } is why the install stopped. { $failed } is what could not be put back, as sentences of their own.
core-restore-not-all-put-back = { $error }. Nicht alles bereits Ausgeführte konnte rückgängig gemacht werden: { $failed }
# { $error } is what went wrong.
core-restore-nowhere-to-keep-record = es gibt keinen sicheren Ort für die Aufzeichnung, mit der sich diese Installation rückgängig machen ließe, daher wurde nichts installiert: { $error }
# { $path } is a file the app would have put there.
core-restore-already-exists = { $path } ist bereits vorhanden, daher wurde nichts installiert. Verwenden Sie --force, um die Datei zu ersetzen
# { $root } is a folder the app has files for. { $link } is that folder, or one it is in.
core-restore-root-under-link = diese App hat Elemente, die in { $root } wiederhergestellt werden sollen, und { $link } ist ein Link an einen anderen Ort, den andere möglicherweise ändern können, daher wurde nichts installiert. Wenn es sich dabei um Benutzerdaten oder Lizenzdaten handelt, installieren Sie die App ohne ihre Benutzerdaten und Lizenzdaten. Wenn es eine Verknüpfung in einem Autostart-Ordner ist, verschieben Sie diesen Ordner wieder an seinen ursprünglichen Ort
core-restore-hash-check-failed = das Paket hat die Hashprüfung nicht bestanden, und es wurde nichts installiert
# { $detail } is the first thing found wrong, already in words.
core-restore-hash-check-failed-because = das Paket hat die Hashprüfung nicht bestanden, und es wurde nichts installiert: { $detail }
# { $path } is a file or folder.
core-restore-refuse-records-folder = Schreiben an dem Ort, an dem Installationen ihre Aufzeichnungen ablegen, wird verweigert: { $path }
# { $path } is a file or folder.
core-restore-refuse-outside = Schreiben außerhalb des Installationsorts und der bekannten Datenordner wird verweigert: { $path }
# { $path } is a file.
core-restore-no-parent-folder = das Ziel hat keinen übergeordneten Ordner: { $path }
# { $path } is where a file was to go.
core-restore-folder-in-the-way = dort, wo diese Datei hin muss, befindet sich ein Ordner: { $path }
# { $path } is a file in the package.
core-restore-file-changed = { $path } entspricht nicht mehr dem Inhalt des Pakets, daher wurde die Datei nicht installiert
# { $path } is a file in the package.
core-restore-source-is-link = { $path } ist ein Link, dem nicht gefolgt wird
# { $path } is a file in the package.
core-restore-source-became-link = { $path } wurde während des Öffnens zu einem Link
# Why a service did not start, after "did not start:".
core-restore-service-stopped-by-itself = er wurde von selbst wieder beendet
# Why part of the app was left out, after "was left out:".
core-restore-com-class-someone-elses = diese COM-Klasse ist auf diesem PC bereits vorhanden und gehört zu etwas anderem

## What an install did, one line for each thing, or for a preview, what it would do. { $source } is a file in the
## package, and { $destination } and { $path } are where it goes on this PC. { $key } is a registry key, such as
## HKLM\Software\App.

# { $package_valid } is true or false, and { $restore_valid } is true, false, or not_run: keep them as they are.
core-restore-verification-result = Überprüfung nach der Wiederherstellung package_valid={ $package_valid } restore_valid={ $restore_valid }
# { $name } is a service's name.
core-restore-started-service = Dienst gestartet: { $name }
core-restore-would-copy = Würde kopieren: { $source } -> { $destination }
core-restore-copied = Kopiert: { $source } -> { $destination }
core-restore-would-create-empty-folder = Würde leeren Ordner erstellen: { $path }
core-restore-created-empty-folder = Leerer Ordner erstellt: { $path }
# { $value } is the name of a value in the key.
core-restore-would-restore-registry-value = Würde Registrierungswert wiederherstellen: { $key } { $value }
core-restore-would-restore-registry-key = Würde Registrierungsschlüssel wiederherstellen: { $key }
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-would-add-firewall-rule = Würde Firewallregel hinzufügen: { $name } für { $program }
# { $name } is a firewall rule's name.
core-restore-firewall-rule-already-there = Firewallregel bereits vorhanden: { $name }
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-added-firewall-rule = Firewallregel hinzugefügt: { $name } für { $program }
# { $name } is a service's name, { $command } what it runs, and { $account } the account it runs as, such as LocalService.
core-restore-would-restore-service = Würde Dienst wiederherstellen: { $name } -> { $command } (ausgeführt als { $account })
# { $name } is a runtime, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-restore-prerequisite-present = Erforderliche Komponente bereits vorhanden: { $name }
# { $name } is a runtime that Windows has as a Windows feature.
core-restore-would-install-prerequisite-feature = Würde erforderliche Komponente installieren: { $name } (Windows-Feature)
# { $name } is a runtime whose installer is in the package.
core-restore-would-install-prerequisite-bundled = Würde erforderliche Komponente installieren: { $name } (gebündeltes Installationsprogramm)
# { $name } is a runtime that would come through winget, which is a program's name.
core-restore-would-install-prerequisite-winget = Würde erforderliche Komponente installieren: { $name } (winget)
# { $name } is a runtime.
core-restore-installed-prerequisite = Erforderliche Komponente installiert: { $name }
# { $name } is a runtime.
core-restore-installed-prerequisite-reboot = Erforderliche Komponente installiert: { $name } (Neustart erforderlich)
core-restore-would-restore-license = Würde Lizenz wiederherstellen: { $source } -> { $destination }
core-restore-restored-license = Lizenz wiederhergestellt: { $source } -> { $destination }
core-restore-would-restore-license-registry = Würde Lizenz in der Registrierung wiederherstellen: { $key }
# { $name } is a driver's name.
core-restore-driver-not-restored = Treiber nicht automatisch wiederhergestellt: { $name }
# { $name } is a driver's name.
core-restore-would-install-driver = Würde Treiber installieren: { $name } -> { $destination }
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-would-restore-task = Würde geplante Aufgabe wiederherstellen: { $name }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-startup-shortcut = Würde Autostart-Verknüpfung wiederherstellen: { $shortcut } -> { $target }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-shortcut = Würde Verknüpfung wiederherstellen: { $shortcut } -> { $target }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-would-restore-variable =
    { $scope ->
        [user] Würde Benutzerumgebungsvariable wiederherstellen: { $name }
       *[machine] Würde Systemumgebungsvariable wiederherstellen: { $name }
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-left-variable =
    { $scope ->
        [user] Benutzerumgebungsvariable unverändert gelassen: { $name }, da nichts davon zu dieser App gehört
       *[machine] Systemumgebungsvariable unverändert gelassen: { $name }, da nichts davon zu dieser App gehört
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-restored-variable =
    { $scope ->
        [user] Benutzerumgebungsvariable wiederhergestellt: { $name }
       *[machine] Systemumgebungsvariable wiederhergestellt: { $name }
    }
core-restore-set-uninstaller = Deinstallationsprogramm festgelegt: { $key }
core-restore-placed-driver-image = Treiberdatei abgelegt: { $source } -> { $destination }
# { $name } is a driver's name.
core-restore-configured-driver = Treiber konfiguriert: { $name }
# { $name } is a driver's name.
core-restore-created-driver = Treiber erstellt: { $name }
core-restore-ensured-registry-key = Registrierungsschlüssel sichergestellt: { $key }
# { $value } is the name of a value in the key.
core-restore-restored-registry-value = Registrierungswert wiederhergestellt: { $key } { $value }
# "(Default)" is how Windows names the value a key has with no name: write it as Registry Editor does.
core-restore-restored-registry-default = Registrierungswert wiederhergestellt: { $key } (Standard)
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-configured-service = Dienst konfiguriert: { $name } (ausgeführt als { $account })
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-created-service = Dienst erstellt: { $name } (ausgeführt als { $account })
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-restored-task = Geplante Aufgabe wiederhergestellt: { $name }
core-restore-restored-shortcut = Verknüpfung wiederhergestellt: { $path }

## Warnings an install gives, and why it failed. They are shown after it, on the PC it ran on.

core-warning-restore-earlier-not-trusted = Eine Aufzeichnung einer nie abgeschlossenen Installation wurde nicht angetastet, da andere sie ändern können. Was diese Installation geändert hat, ist noch vorhanden.
# { $app_name } is the app's name. { $count } is how many changes were put back.
core-warning-restore-earlier-undone =
    { $count ->
        [one] Eine frühere Installation von { $app_name } wurde nie abgeschlossen und daher zuerst rückgängig gemacht. { $count } Änderung wurde zurückgenommen.
       *[other] Eine frühere Installation von { $app_name } wurde nie abgeschlossen und daher zuerst rückgängig gemacht. { $count } Änderungen wurden zurückgenommen.
    }
# { $app_name } is the app's name.
core-warning-restore-earlier-never-finished = Eine frühere Installation von { $app_name } wurde nie abgeschlossen. Bei der Installation wird sie zuerst rückgängig gemacht.
core-warning-restore-user-data-real-locations = Benutzerdaten werden an den tatsächlichen Speicherorten %AppData%/%ProgramData% dieses Computers wiederhergestellt statt unter dem Zielordner.
# { $error } is what went wrong.
core-warning-restore-commit-failed = Die Aufzeichnung dieser Installation konnte nicht abgeschlossen werden: { $error }
# { $name } is a service's name.
core-warning-restore-service-still-starting = Der Dienst { $name } sollte gestartet werden und ist noch nicht vollständig gestartet. Das kann noch geschehen. Er startet außerdem von selbst, wenn dieser PC das nächste Mal startet.
# { $name } is a service's name. { $reason } is why it did not start, as Windows said it.
core-warning-restore-service-not-started = Der Dienst { $name } ist installiert, wurde aber nicht gestartet: { $reason }. Die App funktioniert möglicherweise erst, wenn er läuft. Windows versucht es beim nächsten Start dieses PCs erneut.
# { $name } is a service's name. { $reason } is why it could not be stopped.
core-warning-restore-service-not-stopped = Der Dienst { $name } konnte vorher nicht beendet werden: { $reason }. Er führt möglicherweise bis zum nächsten Start dieses PCs weiterhin das alte Programm aus.
core-warning-restore-install-cancelled = Die Installation wurde abgebrochen.
core-warning-restore-preview-cancelled = Die Vorschau wurde abgebrochen.
# { $item } is what was left out, such as a file or a service's name. { $reason } is why, already in words.
core-warning-restore-left-out = { $item } wurde ausgelassen: { $reason }.
# { $error } is what went wrong.
core-warning-restore-verification-failed = Die Überprüfung nach der Wiederherstellung konnte nicht ausgeführt werden: { $error }
# { $runner } says what relies on the file: a service, a scheduled task, a firewall rule, a shortcut, or a registry value.
# { $path } is the file. { $name } is the name of what relies on it. { $refusal } is why the file was not replaced,
# already in words.
core-warning-restore-program-would-run =
    { $runner ->
        [service] { $path } darf von diesem Paket nicht ersetzt werden, und der Dienst { $name } ist darauf angewiesen, daher wurde nichts installiert: { $refusal }
        [task] { $path } darf von diesem Paket nicht ersetzt werden, und die geplante Aufgabe { $name } ist darauf angewiesen, daher wurde nichts installiert: { $refusal }
        [firewall] { $path } darf von diesem Paket nicht ersetzt werden, und die Firewallregel { $name } ist darauf angewiesen, daher wurde nichts installiert: { $refusal }
        [shortcut] { $path } darf von diesem Paket nicht ersetzt werden, und die Verknüpfung { $name } ist darauf angewiesen, daher wurde nichts installiert: { $refusal }
       *[registry] { $path } darf von diesem Paket nicht ersetzt werden, und der Registrierungswert { $name } ist darauf angewiesen, daher wurde nichts installiert: { $refusal }
    }
core-warning-restore-destination-exists = Das Ziel ist bereits vorhanden. Verwenden Sie --force, um es zu überschreiben.
# { $path } is a file. { $error } is what went wrong.
core-warning-restore-file-not-written = { $path } konnte nicht geschrieben werden: { $error }
core-warning-restore-not-a-folder = Wo dieser leere Ordner hin soll, befindet sich etwas anderes als ein Ordner.
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-not-looked-at = { $path } konnte nicht geprüft werden: { $error }
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-folder-not-made = { $path } konnte nicht erstellt werden: { $error }
# Why part of the app was left out, after "was left out:".
core-warning-restore-registration-exists = dies ist auf diesem PC bereits mit einer anderen Einstellung vorhanden, die eine Installation nicht übernimmt
core-warning-restore-registry-review = Registrierungswerte wurden aus den archivierten strukturierten Daten wiederhergestellt, wobei bekannte App-Pfade auf den Zielordner umgeschrieben wurden.
# { $error } is what went wrong.
core-warning-restore-firewall-unreadable = Die Regeln der Windows-Firewall konnten nicht gelesen werden: { $error }
# Why a firewall rule was left out, after "was left out:".
core-warning-restore-firewall-rule-exists = eine Regel mit diesem Namen für dieses Programm ist bereits mit anderen Einstellungen vorhanden, die eine Installation nicht ändert
# { $name } is a runtime.
core-warning-restore-prerequisite-unknown = { $name }: Für diese Laufzeitkomponente ist kein Installationsprogramm bekannt.
# { $name } is a runtime.
core-warning-restore-prerequisite-needs-admin = Die Installation von { $name } erfordert erhöhte Rechte. Führen Sie das Installationsprogramm erneut als Administrator aus.
# { $name } is a runtime.
core-warning-restore-prerequisite-reboot = { $name } wurde installiert, aber zum Abschließen ist ein Neustart erforderlich.
# { $error } is what went wrong.
core-warning-restore-skipped-not-recorded = Eine übersprungene Laufzeitkomponente konnte nicht aufgezeichnet werden: { $error }
# { $name } is a runtime. { $reason } is why it was skipped, already in words.
core-warning-restore-prerequisite-skipped = { $name }: { $reason }
# { $name } is a runtime. { $reason } is why it could not be installed. { $url } is a web address to get it from.
core-warning-restore-prerequisite-failed = { $name } konnte nicht installiert werden: { $reason }. Beziehen Sie die Komponente von { $url }.
# { $error } is what went wrong.
core-warning-restore-runtime-not-recorded = Eine installierte Laufzeitkomponente konnte nicht aufgezeichnet werden: { $error }
# { $error } is what went wrong.
core-warning-restore-license-source-invalid = Der Pfad der gepackten Lizenz ist ungültig: { $error }
core-warning-restore-license-needs-admin = Das Wiederherstellen von Lizenzdaten an einem geschützten Ort erfordert erhöhte Rechte.
core-warning-restore-license-exists = Das Lizenzziel ist bereits vorhanden. Verwenden Sie --force, um es zu überschreiben.
# { $error } is what went wrong.
core-warning-restore-license-failed = Lizenzdaten konnten nicht wiederhergestellt werden: { $error }
core-warning-restore-driver-needs-admin = Die Treiberregistrierung erfordert erhöhte Rechte. Führen Sie das Installationsprogramm erneut als Administrator aus.
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-warning-restore-task-runs-as-system = Die geplante Aufgabe { $name } wird als SYSTEM ausgeführt, mit uneingeschränkten Rechten auf diesem PC.
core-warning-restore-shortcut-exists = Die Verknüpfung ist bereits vorhanden. Verwenden Sie --force, um sie zu überschreiben.
core-warning-restore-shortcut-all-users = Mindestens eine wiederhergestellte Verknüpfung liegt auf dem öffentlichen Desktop oder im Startmenü für alle Benutzer.
core-warning-restore-environment-refresh = Änderungen an Umgebungsvariablen werden möglicherweise erst in einem neuen Prozess, einer neuen Shell oder nach einer erneuten Anmeldung wirksam.
# { $name } is a service's name. { $account } is the account it ran as on the PC it came from.
core-warning-restore-service-account = Der Dienst { $name } wurde auf dem Quell-PC als { $account } ausgeführt. Sein Kennwort ist nicht im Installationsprogramm enthalten, daher wird der Dienst als LocalService eingerichtet. Wenn er nicht funktioniert, legen Sie Konto und Kennwort unter „Dienste“ fest.
# { $name } is a runtime.
core-warning-restore-prerequisite-pending-reboot = { $name } ist noch nicht registriert und sollte nach einem Neustart aktiv sein.
# { $name } is a runtime.
core-warning-restore-prerequisite-not-verified = { $name } hat einen Erfolg gemeldet, wurde aber nicht registriert. Die Installation ist möglicherweise fehlgeschlagen.
# { $error } is what went wrong.
core-warning-restore-driver-source-invalid = Der Pfad des gebündelten Treibers ist ungültig: { $error }
# { $error } is what went wrong.
core-warning-restore-driver-not-read = Die aktuellen Einstellungen des Treibers konnten nicht gelesen werden: { $error }
core-warning-restore-driver-exists = Der Treiberdienst ist bereits vorhanden. Verwenden Sie --force, um ihn neu zu konfigurieren.
core-warning-restore-driver-image-exists = Die Treiberdatei ist bereits vorhanden. Verwenden Sie --force, um sie zu überschreiben.
core-warning-restore-driver-hash-bundled = Die gebündelte Treiberdatei entspricht nicht ihrem aufgezeichneten Hash und wurde daher nicht installiert.
# { $error } is what went wrong.
core-warning-restore-driver-not-placed = Die Treiberdatei konnte nicht abgelegt werden: { $error }
core-warning-restore-driver-hash-placed = Die Treiberdatei wurde nach dem Ablegen geändert und daher nicht registriert.
# { $error } is what went wrong.
core-warning-restore-key-not-created = Der Registrierungsschlüssel konnte nicht erstellt werden: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-read = Der Wert, der ersetzt würde, konnte nicht gelesen werden: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-written = Der Registrierungswert konnte nicht geschrieben werden: { $error }
core-warning-restore-value-exists = Der Registrierungswert ist bereits vorhanden. Verwenden Sie --force, um ihn zu überschreiben.
# { $error } is what went wrong.
core-warning-restore-environment-key-not-created = Der Registrierungsschlüssel für Umgebungsvariablen konnte nicht erstellt werden: { $error }
core-warning-restore-variable-exists = Die Umgebungsvariable ist bereits vorhanden. Verwenden Sie --force, um sie zu überschreiben.
# { $error } is what went wrong.
core-warning-restore-variable-not-written = Die Umgebungsvariable konnte nicht geschrieben werden: { $error }
core-warning-restore-value-kind-missing = Der Typ des Registrierungswerts fehlt.
# { $error } is what went wrong.
core-warning-restore-service-not-read = Die aktuellen Einstellungen des Dienstes konnten nicht gelesen werden: { $error }
core-warning-restore-service-exists = Der Dienst ist bereits vorhanden. Verwenden Sie --force, um ihn neu zu konfigurieren.
# { $error } is what went wrong.
core-warning-restore-task-not-queried = Die geplante Aufgabe konnte nicht abgefragt werden: { $error }
core-warning-restore-task-exists = Die geplante Aufgabe ist bereits vorhanden. Verwenden Sie --force, um sie zu überschreiben.
# { $error } is what went wrong.
core-warning-restore-task-folder-not-looked-at = Der Ordner der Aufgabe konnte nicht geprüft werden: { $error }
# { $error } is what went wrong.
core-warning-restore-shortcut-not-put-back = Die Verknüpfung konnte nicht erstellt werden, und die von ihr ersetzte Verknüpfung konnte nicht wiederhergestellt werden: { $error }
# { $error } is what went wrong.
core-warning-restore-command-not-started = Der Befehl konnte nicht gestartet werden: { $error }
# { $output } is what the command wrote, as it wrote it.
core-warning-restore-command-failed = Befehl fehlgeschlagen: { $output }
# { $error } is what went wrong.
core-warning-restore-powershell-not-started = PowerShell konnte nicht gestartet werden: { $error }
