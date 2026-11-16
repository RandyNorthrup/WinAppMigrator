# Words the core says about what an install may do: why an install location or something in a package is refused,
# the installer's list of what an app sets up beyond its files, the checks of a package and of an install, and whether
# there is room for either. Message names start with core-.
#
# Many messages here are a reason, the second half of a sentence that starts "... was left out:" or "that install
# location cannot be used:", so they start with a small letter and have no full stop.


## Why an install location cannot be used. Shown in red under the Install Location field.

core-review-location-state-root = an diesem Ort legen Installationen ihre Aufzeichnungen ab
core-review-location-has-files = er enthält bereits Dateien, und hier ist keine Installation verzeichnet. Verwenden Sie daher einen leeren Ordner
core-policy-location-not-full-path = er muss ein vollständiger Pfad sein
core-policy-location-parent-step = er darf kein „..“ enthalten
core-policy-location-not-plain = er muss ein einfacher Pfad auf einem Laufwerk dieses PCs sein
core-policy-location-whole-drive = er kann kein ganzes Laufwerk sein
core-policy-location-windows = er kann nicht der Windows-Ordner sein
core-policy-location-runs-by-itself = er gehört zu Windows, oder sein Inhalt wird automatisch ausgeführt oder geladen
core-policy-location-shared = er enthält andere Dinge nebeneinander, daher benötigt die App einen eigenen Ordner darin

## Whether an app can run on this PC at all, said before anything is installed.

# { $reason } is why, as the second half of a sentence, such as "App.exe is built for Arm64, and this PC is x64, so
# Windows would not start it".
core-review-cannot-run-here = diese App kann auf diesem PC nicht ausgeführt werden, daher wurde nichts geändert: { $reason }
core-policy-architecture-x86 = 32-Bit-x86
core-policy-architecture-x64 = x64
core-policy-architecture-arm64 = Arm64
core-policy-architecture-arm32 = 32-Bit-Arm
core-policy-architecture-unknown = eine unbekannte Architektur
# { $code } is the number Windows gives a kind of processor, such as 0x01c4.
core-policy-machine-type = Maschinentyp { $code }
# { $names } is a list of program names, such as "a.exe, b.exe, c.exe", and { $count } how many more there are.
core-policy-names-and-more = { $names } und { $count } weitere
core-policy-fit-programs-not-said = Dieses Installationsprogramm gibt nicht an, was seine Programme benötigen, daher wurde nicht geprüft, ob sie auf diesem PC ausgeführt werden können.
core-policy-fit-architecture-not-told = Die Architektur dieses PCs konnte nicht ermittelt werden, daher wurde nicht geprüft, ob die Programme dieser App darauf ausgeführt werden können.
core-policy-fit-main-not-said = Dieses Installationsprogramm gibt nicht an, was sein Hauptprogramm benötigt, daher stützen sich die Angaben hier auf den Rest seines Inhalts.
# { $driver } is a driver's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-driver = ihr Treiber { $driver } ist für { $built_for } erstellt, und dieser PC verwendet { $here }. Ein Treiber muss genau zu Windows passen, daher kann diese App hier nicht ausgeführt werden
# { $program } is a program's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-program = { $program } ist für { $built_for } erstellt, und dieser PC verwendet { $here }, daher würde Windows das Programm nicht starten
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-none-start = keines ihrer Programme kann auf diesem PC ({ $here }) starten: { $programs }
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-some-other-architecture = Einige Programme dieser App sind für eine andere Architektur erstellt und starten nicht auf diesem PC ({ $here }): { $programs }. Der Rest der App kann starten, daher spielt das möglicherweise keine Rolle.
# { $here } is a kind of processor, such as "Arm64". { $programs } is a list of program names.
core-policy-fit-emulated = Dieser PC verwendet { $here }, daher würde Windows diese Programme über einen Emulator ausführen: { $programs }. Die meisten Programme funktionieren so problemlos, einige nicht.
# { $program } is a program's file name. { $needs } and { $runs } are versions of Windows, such as 10.0 and 6.1.
core-policy-fit-needs-newer = { $program } benötigt laut eigener Angabe Windows { $needs } oder höher, und auf diesem PC läuft Windows { $runs }, daher würde Windows das Programm nicht starten
# { $programs } is a list of program names.
core-policy-fit-some-need-newer = Einige Programme dieser App benötigen laut eigener Angabe ein neueres Windows als dieses und starten hier nicht: { $programs }.
# { $from_build } and { $this_build } are Windows build numbers, such as 22631, written as Windows writes them.
core-policy-fit-older-build = Diese App befand sich auf Windows-Build { $from_build }, und auf diesem PC läuft Build { $this_build }. Ihre Programme können laut eigener Angabe unter diesem Windows starten, daher funktioniert sie möglicherweise, aber worauf sie sich dort verlassen hat, ist hier möglicherweise nicht vorhanden.
core-policy-fit-build-not-told = Der Build dieses Windows konnte nicht ermittelt werden, daher wurde er nicht mit dem verglichen, auf dem sich diese App befand.

## Room on a drive for a build or an install.

# Each of these ends the sentence core-space-shortfall, after "free for".
core-space-for-unpacking = das Entpacken des Pakets
core-space-for-install-record = die Aufzeichnung der Installation
core-space-for-app-files = die Dateien der App
core-space-for-what-is-replaced = das Aufbewahren dessen, was ersetzt wird
core-space-for-build-work = die Arbeitsdateien einer Erstellung
core-space-for-installer = das Installationsprogramm
core-space-for-package = das Paket
core-space-for-unpacking-and-installing = das Entpacken und Installieren des Pakets
# What goes between two of those, as in "unpacking the package and the app's files". Keep the quotes: they keep the
# spaces.
core-space-purposes-joiner = { " und " }
# { $drive } is a drive, such as C:, or a folder. { $needed } and { $free } are sizes, such as 1.2 GB. { $purposes } is
# what the room is for, such as "unpacking the package and the app's files".
core-space-shortfall = { $drive } benötigt bis zu { $needed } freien Speicherplatz für { $purposes } und hat { $free }
# { $drive } is a drive, such as C:, or a folder. { $needed } is a size, such as 1.2 GB.
core-space-not-told = { $drive } gibt nicht an, wie viel Speicherplatz frei ist, daher wurde nicht geprüft, ob die benötigten { $needed } passen
# One drive in a list of where room is needed. { $needed } and { $free } are sizes, and { $drive } a drive, such as C:.
core-space-summary = bis zu { $needed } auf { $drive } ({ $free } frei)
# { $needed } is a size, and { $drive } a drive, such as C:.
core-space-summary-not-told = bis zu { $needed } auf { $drive } (freier Speicherplatz konnte nicht ermittelt werden)
# What goes between two sentences of core-space-shortfall inside core-review-no-room. Keep the quotes: they keep the
# space.
core-review-sentence-joiner = { ". " }
# { $shortfalls } is one or more sentences of core-space-shortfall.
core-review-no-room = es ist nicht genügend Speicherplatz dafür vorhanden, daher wurde nichts geändert. { $shortfalls }. Geben Sie dort Speicherplatz frei, oder wählen Sie einen Installationsort auf einem anderen Laufwerk aus.

## The installer's list of what an app sets up beyond its files: what each kind is called.

core-review-kind-service = Dienst
core-review-kind-driver = Treiber
core-review-kind-scheduled-task = Geplante Aufgabe
core-review-kind-autostart = Autostart
core-review-kind-shortcut = Verknüpfung
core-review-kind-path = PATH
core-review-kind-environment-variable = Umgebungsvariable
core-review-kind-shell-extension = Shellerweiterung
core-review-kind-menu-command = Menübefehl
core-review-kind-link-type = Linktyp
core-review-kind-file-type = Dateityp
core-review-kind-com = COM
core-review-kind-firewall-rule = Firewallregel
core-review-kind-event-log-source = Ereignisprotokollquelle
core-review-kind-font = Schriftart
core-review-kind-odbc-driver = ODBC-Treiber
core-review-kind-odbc-source = ODBC-Datenquelle
core-review-kind-compatibility-setting = Kompatibilitätseinstellung
core-review-kind-default-apps = Standard-Apps
core-review-kind-program-name = Programmname
core-review-kind-other-settings = Weitere Einstellungen

## The installer's list of what an app sets up: what each line says of it.

# { $file } is a program's file name. { $account } is the Windows account it runs as, such as LocalSystem. { $starts }
# says when it starts, and is not shown itself; for "other", { $start } is one of the core-review-start- words below.
core-review-service-detail =
    { $starts ->
        [automatic] Führt { $file } als { $account } aus, startet jetzt und bei jedem Start dieses PCs
        [delayed] Führt { $file } als { $account } aus, startet jetzt und kurz nach jedem Start dieses PCs
        [manual] Führt { $file } als { $account } aus, startet nur, wenn er angefordert wird
        [disabled] Führt { $file } als { $account } aus, ist deaktiviert
        [other] Führt { $file } als { $account } aus, startet { $start }
       *[unknown] Führt { $file } als { $account } aus
    }
# { $account } is a Windows account, such as LocalService.
core-policy-account-custom = { $account } (benutzerdefiniertes Konto nicht wiederhergestellt)
# { $file } is a driver's file name.
core-review-driver-detail = Lädt { $file } in Windows selbst
# { $file } is a driver's file name. { $start } is when Windows starts it: one of the core-review-start- words below.
core-review-driver-detail-starts = Lädt { $file } in Windows selbst, startet { $start }
# When a service or driver starts, ending a sentence such as "Loads disk.sys into Windows itself, starts …".
core-review-start-boot = mit Windows, vor allem anderen
core-review-start-system = mit Windows, während des Hochfahrens
core-review-start-automatic = bei jedem Start dieses PCs
core-review-start-delayed = kurz nach jedem Start dieses PCs
core-review-start-manual = nur, wenn er angefordert wird
core-review-start-disabled = nie, da er deaktiviert ist
core-review-start-unknown = auf eine Weise, die Windows nicht angibt
# { $account } is the account a task runs as: an account's name, or core-review-task-runs-as-you, or
# core-review-task-runs-as-creator.
core-review-task-detail = Ausgeführt als: { $account }
# { $account } is the account a task runs as, as for core-review-task-detail.
core-review-task-detail-highest = Ausgeführt als: { $account }, mit höchsten Privilegien
core-review-task-runs-as-you = Sie, also der jeweils angemeldete Benutzer
core-review-task-runs-as-creator = der Benutzer, der sie erstellt hat
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-autostart-detail =
    Wird { $scope ->
        [everyone] bei der Anmeldung jedes Benutzers ausgeführt
       *[you] bei Ihrer Anmeldung ausgeführt
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shortcut-startup-detail =
    Eine Verknüpfung im Autostart-Ordner, { $scope ->
        [everyone] für alle auf diesem PC
       *[you] für Sie
    }
# { $file } is the file name of what the shortcut opens. { $scope } is whom it is set up for: everyone on this PC,
# or you. It is not shown itself.
core-review-shortcut-detail =
    Öffnet { $file }, { $scope ->
        [everyone] für alle auf diesem PC
       *[you] für Sie
    }
# { $name } is an environment variable's name, such as PATH.
core-review-path-title = Ergänzt { $name }
core-review-path-install-folder = Der Installationsordner
# { $path } is a folder inside the install folder, such as bin\tools.
core-review-path-below-install-folder = Der Installationsordner\{ $path }
# { $value } is the variable's value, as it will be set.
core-review-variable-detail = Wird auf { $value } festgelegt
# { $count } is how many registry values.
core-review-other-settings-title =
    { $count ->
        [one] { $count } Einstellung außerhalb der eigenen Schlüssel der App
       *[other] { $count } Einstellungen außerhalb der eigenen Schlüssel der App
    }
# { $keys } is a list of registry keys, such as HKCU\Software\Vendor.
core-review-other-settings-detail = In { $keys }
# { $keys } is a list of registry keys, and { $count } how many more there are.
core-review-other-settings-detail-more = In { $keys } und { $count } weiteren
# { $way } says whether the rule allows or blocks, and incoming or outgoing connections; it is not shown itself.
# { $ports } is a list of port numbers, and { $port_count } how many. { $program } is the program's file name.
# { $networks } is a list of the kinds of network Windows names in English (domain, private, public), and
# { $network_count } how many.
# The kinds of network a firewall rule is for, as Windows Defender Firewall names its profiles in your language. They
# fill { $networks } in core-review-firewall-detail, joined into a list.
core-review-network-domain = Domäne
core-review-network-private = Privat
core-review-network-public = Öffentlich
core-review-firewall-detail =
    { $way ->
        [allow-in] Erlaubt eingehende
        [allow-out] Erlaubt ausgehende
        [block-in] Blockiert eingehende
       *[block-out] Blockiert ausgehende
    } { $port_count ->
        [0] Verbindungen
        [one] Verbindungen an Port { $ports }
       *[other] Verbindungen an den Ports { $ports }
    } für { $program }, { $network_count ->
        [0] in allen Netzwerken
       *[other] in Netzwerken vom Typ { $networks }
    }
# { $server } is the file name of what Explorer loads. { $scope } is whom it is set up for: everyone on this PC, or
# you. It is not shown itself.
core-review-shell-extension-detail =
    Lädt { $server } in den Explorer, { $scope ->
        [everyone] für alle auf diesem PC
       *[you] für Sie
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shell-extension-detail-own-file =
    Lädt eine mitinstallierte Datei in den Explorer, { $scope ->
        [everyone] für alle auf diesem PC
       *[you] für Sie
    }
# The name of a shell extension: { $handler } is its name in the registry, and { $class } the kind of file or folder it
# is for, such as * or Directory.
core-policy-shell-extension-name = { $handler } für { $class }
# { $on } is the kind of file or folder whose menu it is, as the registry names it, such as * or
# Directory. { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-menu-command-detail =
    Fügt dem Menü von { $on } einen Befehl hinzu, { $scope ->
        [everyone] für alle auf diesem PC
       *[you] für Sie
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-default-verb-detail =
    Ändert, was beim Öffnen dieser Elemente geschieht, { $scope ->
        [everyone] für alle auf diesem PC
       *[you] für Sie
    }
core-review-client-entry-detail = Kann als Programm hierfür ausgewählt werden. Es wird nichts darauf umgestellt
core-review-com-permissions-title = COM-Berechtigungen
core-review-com-permissions-detail = Legt fest, wer die COM-Klassen starten darf oder unter welchem Konto sie ausgeführt werden
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-link-type-detail =
    Links dieses Typs werden mit der App geöffnet, { $scope ->
        [everyone] für alle auf diesem PC
       *[you] für Sie
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-file-type-detail =
    Dateien dieses Typs werden mit der App geöffnet, { $scope ->
        [everyone] für alle auf diesem PC
       *[you] für Sie
    }
# { $log } is the name of a Windows event log, such as Application.
core-review-event-log-source-detail = Kann in das Protokoll „{ $log }“ schreiben, mit Meldungen aus eigenen Dateien
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-font-detail =
    Wird bei der Anmeldung aus dem Installationsordner geladen, { $scope ->
        [everyone] für alle auf diesem PC
       *[you] für Sie
    }
core-review-odbc-driver-detail = Wird in Programme geladen, die über ODBC auf eine Datenbank zugreifen
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-odbc-source-detail =
    Eine Datenbankverbindung, { $scope ->
        [everyone] für alle auf diesem PC
       *[you] für Sie
    }
core-review-default-apps-detail = Kann unter „Standard-Apps“ ausgewählt werden. Es wird nichts darauf umgestellt
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-program-name-detail =
    „Ausführen“ und andere Programme können es unter diesem Namen finden, { $scope ->
        [everyone] für alle auf diesem PC
       *[you] für Sie
    }
# { $server } is the file name of what serves the classes.
core-review-com-title = COM in { $server }
core-review-com-title-own-file = COM in einer mitinstallierten Datei
# { $count } is how many registry values.
core-review-com-detail =
    { $count ->
        [one] { $count } Einstellung der Klassen, die andere Programme starten können
       *[other] { $count } Einstellungen der Klassen, die andere Programme starten können
    }
core-review-com-support-title = COM-Typbibliotheken, -Schnittstellen und AppIDs
# { $libraries }, { $interfaces }, and { $app_ids } are how many registry values there are for its type libraries, its
# interfaces, and its AppIDs.
core-review-com-support-detail = Einstellungen für die Typbibliotheken ({ $libraries }), Schnittstellen ({ $interfaces }) und AppIDs ({ $app_ids }), die zu den Klassen gehören
core-review-compatibility-title = Wie die Programme starten
# { $count } is how many registry values.
core-review-compatibility-detail =
    { $count ->
        [one] { $count } Kompatibilitätseinstellung für mitinstallierte Programme
       *[other] { $count } Kompatibilitätseinstellungen für mitinstallierte Programme
    }

## The installer's notes on whose folders and settings these are, and what is not coming.

# { $account } is the name of a Windows account, such as alice.
core-review-account-signed-in = Benutzerdaten und benutzerspezifische Einstellungen gehen an { $account }, das hier angemeldete Konto, und nicht an das Administratorkonto, unter dem dies ausgeführt wird.
# { $account } is the name of a Windows account, such as alice.
core-review-account-no-desktop = In dieser Sitzung ist niemand angemeldet, daher gehen Benutzerdaten und benutzerspezifische Einstellungen an { $account }, das Konto, unter dem dies ausgeführt wird.
# { $reason } is what Windows said, in its own words.
core-review-account-not-told = Es konnte nicht ermittelt werden, wer angemeldet ist ({ $reason }), daher werden benutzerspezifische Einstellungen ausgelassen, und Benutzerdaten werden unter dem Installationsort abgelegt.
# { $reason } is what Windows said, in its own words.
core-review-signed-in-unknown = es konnte nicht ermittelt werden, wer angemeldet ist: { $reason }
# Headings of what is left out, shared by the app's list and the installer's notes.
core-review-left-out-credentials = Nicht in diesem Installationsprogramm, da sie wie Anmeldeinformationen aussehen
core-review-left-out-settings = Einstellungen und Verlauf, die ein Konto für die App speichert und die nur mit der Option „Benutzerdaten“ aufgenommen werden
core-review-left-out-certificates = Die App scheint auf Zertifikate angewiesen zu sein, die auf ihrem PC als vertrauenswürdig galten, und diese werden nicht installiert
core-review-left-out-fonts = Diese Schriftarten tragen den Namen der App, aber nichts weist sie als ihre eigenen aus, daher werden sie nicht installiert
# { $names } is a list of names, such as the names of saved passwords.
core-review-note-credentials = Nicht in diesem Installationsprogramm, da sie wie Anmeldeinformationen aussehen: { $names }. Was davon auf diesem PC vorhanden ist, bleibt unverändert, und Sie müssen diese Angaben möglicherweise erneut eingeben.
# { $names } is a list of names of license data.
core-review-note-licenses = Nicht in diesem Installationsprogramm, da die Option „Lizenz- und Aktivierungsdaten“ bei der Erstellung deaktiviert war: { $names }.
# { $names } is a list of places an account keeps settings, such as folders.
core-review-note-settings = Nicht in diesem Installationsprogramm, da die Option „Benutzerdaten“ bei der Erstellung deaktiviert war: die Einstellungen und der Verlauf, die ein Konto für die App speichert, in { $names }. Die App startet hier wie für jemanden, der sie zum ersten Mal verwendet.
# { $names } is a list of certificates, each as core-review-certificate.
core-review-note-certificates = Die App scheint auf Zertifikate angewiesen zu sein, die auf ihrem PC als vertrauenswürdig galten, und diese werden nicht installiert: { $names }. Ob Sie ihnen hier vertrauen, entscheiden Sie selbst.
# { $names } is a list of font names.
core-review-note-fonts = Diese Schriftarten tragen den Namen der App, aber nichts weist sie als ihre eigenen aus, daher werden sie nicht installiert: { $names }.
# { $names } is a list of names.
core-review-note-left-out-by-choice = Von der Person ausgelassen, die dieses Installationsprogramm erstellt hat: { $names }.
# { $names } is a list of names.
core-review-note-added-by-choice = Von der Person hinzugefügt, die dieses Installationsprogramm erstellt hat, da sie es für einen Teil der App hielt: { $names }.
# { $name } is a certificate's subject, such as CN=Contoso, and { $store } the certificate store it was in, such as
# Root.
core-review-certificate = { $name } in { $store }
# { $names } is a list of names, and { $count } how many more there are.
core-review-names-and-more = { $names } und { $count } weitere

## Why something in a package is left out. Each is the second half of a sentence of core-warning-review-left-out.

core-review-no-account = es gehört zu den Einstellungen eines Kontos, und es konnte nicht ermittelt werden, wer angemeldet ist
core-review-name-not-accepted = Windows würde seinen Namen nicht akzeptieren
# { $error } is what Windows said, in its own words.
core-review-service-unreadable = seine aktuellen Einstellungen konnten nicht gelesen werden: { $error }
core-review-service-dependency = er hängt von einem Dienst mit ungültigem Namen ab
# { $path } is the program the service runs.
core-review-service-runs-other = er führt { $path } aus, und dieses Paket installiert diese Datei nicht
core-review-service-settings = seine Einstellungen nennen etwas, das dieses Paket nicht installiert, oder einen Schlüssel, der nicht sein eigener ist
core-review-driver-no-hash = er hat keinen Hash, anhand dessen er geprüft werden kann
core-review-license-for-the-machine = eine für den gesamten Computer gespeicherte Lizenz wird nur gemeldet
core-review-license-starts-something = eine Lizenz hat keinen Grund, etwas zu starten
core-review-task-not-captured = ihre Definition wurde nicht erfasst
# { $error } is what was wrong with the task's XML, in English.
core-review-task-unreadable = ihre Definition konnte nicht gelesen werden: { $error }
core-review-task-no-account = sie wird als die Person ausgeführt, die installiert, und es konnte nicht ermittelt werden, wer hier angemeldet ist
# { $path } is what the shortcut opens.
core-review-shortcut-opens-other = sie öffnet { $path }, und dieses Paket installiert diese Datei nicht
# { $service } is the service's name, and { $path } the program it runs now.
core-policy-service-exists = { $service } ist auf diesem PC bereits vorhanden und führt { $path } aus, und diese Datei stammt nicht aus einer früheren Installation hier. Wenn es sich um eine ältere Kopie dieser App handelt, deinstallieren Sie diese zuerst
core-policy-key-control-characters = sein Schlüsselname enthält Steuerzeichen
core-policy-value-control-characters = der Name seines Werts enthält Steuerzeichen
core-policy-registry-link = es handelt sich um einen Registrierungslink
# { $reason } is why the key it stands for is left out, such as "no app install should change this key".
core-policy-virtual-store = es steht für einen Schlüssel des PCs, und { $reason }
core-policy-registry-part = keine App-Installation sollte in diesen Teil der Registrierung schreiben
core-policy-key-refused = keine App-Installation sollte diesen Schlüssel ändern
core-policy-net-debugger = es würde einen Debugger für jedes .NET-Programm festlegen
core-policy-net-install-root = es würde ändern, von wo jedes .NET-Programm .NET lädt
core-policy-class-redirects = es würde ändern, wie eine Art von Programm oder Ordner geöffnet wird
core-policy-app-path-name = es nennt ein Programm mit etwas anderem als einem Dateinamen
core-policy-app-path-not-installed = es würde einen Programmnamen für eine Datei registrieren, die dieses Paket nicht installiert
core-policy-shared-default-verb = es würde ändern, was beim Öffnen aller Dateien oder Ordner einer Art geschieht
core-policy-command-runs-other = es fügt einen Befehl hinzu, der etwas ausführt, das dieses Paket nicht installiert
core-policy-autostart-not-command = es startet automatisch, und sein Wert ist kein Befehl
core-policy-autostart-runs-other = es startet automatisch und führt etwas aus, das dieses Paket nicht installiert
core-policy-environment-own-rules = Umgebungsvariablen werden nach eigenen Regeln wiederhergestellt
core-policy-service-not-restored = es gehört zu einem Dienst, den dieses Paket nicht wiederherstellt
core-policy-service-own-rules = die eigenen Einstellungen eines Dienstes werden nur nach dessen Regeln wiederhergestellt
core-policy-service-dll = seine ServiceDll ist keine Datei, die dieses Paket installiert
core-policy-event-log-source-only = nur die eigenen Werte einer Ereignisprotokollquelle können wiederhergestellt werden
core-policy-security-log = keine App schreibt in das Protokoll „Sicherheit“
core-policy-event-log-messages = seine Meldungen kämen aus einer Datei, die dieses Paket nicht installiert
core-policy-type-library = seine Typbibliothek ist keine Datei, die dieses Paket installiert
# { $name } is a program's file name, such as app.exe.
core-policy-application-opened = es würde ändern, wie { $name } geöffnet wird, und dieses Paket installiert diese Datei nicht
core-policy-event-log-no-file = eine Ereignisprotokollquelle muss ihre Meldungen aus einer Datei beziehen
core-policy-com-runs-other = es führt etwas aus, das keine Datei ist, die dieses Paket installiert
core-policy-com-redirect = es würde diese Klasse zu etwas machen, das dieses Paket nicht installiert
core-policy-com-server = es führt etwas aus, das weder eine von diesem Paket installierte Datei noch Teil von Windows ist
core-policy-interface = es würde mit etwas, das dieses Paket nicht installiert, festlegen, wie Aufrufe einer Schnittstelle erfolgen
# { $name } is a program's file name, such as app.exe.
core-policy-app-id-started = es würde ändern, wie { $name } gestartet wird, und dieses Paket installiert diese Datei nicht
core-policy-app-id-surrogate = es würde von einem Programm gehostet, das dieses Paket nicht installiert
core-policy-app-id-service = es würde in einem Dienst ausgeführt, den dieses Paket nicht wiederherstellt
core-policy-app-id-run-as = es würde unter einem eigenen Konto ausgeführt
core-policy-app-id-link = es nennt keine AppID
core-policy-app-id-remote = es würde auf einem anderen Computer als diesem PC ausgeführt
core-policy-odbc-own-driver = das ist eine eigene Einstellung von ODBC und kein Treiber
core-policy-odbc-driver-not-installed = sein Treiber ist keine Datei, die dieses Paket installiert
core-policy-loads-other-file = es würde eine Datei laden, die dieses Paket nicht installiert
core-policy-odbc-own-source = das ist eine eigene Einstellung von ODBC und keine Datenquelle
core-policy-odbc-source-missing = es nennt eine Datenquelle, die dieses Paket nicht enthält
core-policy-odbc-source-driver = sein Treiber ist weder eine von diesem Paket installierte Datei noch Teil von Windows
core-policy-shell-extension = es würde eine Shellerweiterung in den Explorer laden, die keine von diesem Paket installierte Datei ist
core-policy-not-text = sein Inhalt ist nicht der Text, als der er gelesen würde
core-policy-network-place = es nennt einen Ort im Netzwerk
core-policy-font = es würde eine Schriftart registrieren, die dieses Paket nicht installiert, oder eine im Windows-Ordner
core-policy-compatibility = es würde ändern, wie ein Programm ausgeführt wird, das dieses Paket nicht installiert
core-policy-capabilities = es nennt eine App, deren Angaben nicht in diesem Paket enthalten sind
core-policy-default-program = es würde ändern, welches Programm Elemente standardmäßig öffnet
core-policy-url-protocol = es ist als Linktyp angegeben und wird mit nichts geöffnet, was dieses Paket installiert
core-policy-command-handler = es fügt einen Befehl hinzu, der von etwas verarbeitet wird, das dieses Paket nicht installiert
core-policy-installer-command = es würde etwas ausführen, das nur Windows Installer lesen kann
core-policy-creates-class = es würde eine Klasse erstellen, die dieses Paket nicht installiert
core-policy-opens-links = es öffnet Links mit etwas, das dieses Paket nicht installiert
core-policy-opens-files = es öffnet Dateien mit etwas, das dieses Paket nicht installiert
core-policy-firewall-name = ihr Name lässt sich nicht übernehmen
core-policy-firewall-description = ihre Beschreibung lässt sich nicht übernehmen
core-policy-firewall-every-program = sie gilt für alle Programme, und eine Regel kann nur für eines gelten
# { $path } is the program the rule is for.
core-policy-firewall-program = sie gilt für { $path }, und dieses Paket installiert diese Datei nicht
core-policy-firewall-direction = sie gibt keine Richtung an, die hier festgelegt werden kann
core-policy-firewall-action = sie würde andere Regeln außer Kraft setzen, was keine Regel einer App tun sollte
# { $setting } is a part of a firewall rule, as the scan of its PC described it.
core-policy-firewall-not-carried = sie gibt { $setting } an, was sich nicht unverändert festlegen ließ oder die Wirkung anderer Regeln schwächen würde
core-policy-firewall-service = sie gilt für einen Dienst, den dieses Paket nicht wiederherstellt
core-policy-firewall-parts = ein Teil davon lässt sich in einer Firewallregel nicht angeben
core-policy-folder-already-there = dieser Ordner war bereits außerhalb des Installationsorts vorhanden, und nichts weist ihn als Ordner dieser App aus, daher wird kein Programm darin abgelegt
core-policy-variable-name = ihr Name ist kein gültiger Variablenname
core-policy-variable-relied-on = Windows und andere Programme sind auf diese Variable angewiesen
core-policy-task-name = ihr Name wird von Windows nicht akzeptiert
core-policy-task-folder = ihr Ordner wird von Windows nicht akzeptiert
core-policy-task-windows = sie würde eine der eigenen Aufgaben von Windows ersetzen
core-policy-task-other-action = sie tut etwas anderes, als ein Programm auszuführen
core-policy-task-no-program = sie nennt kein auszuführendes Programm
# { $path } is the program the task runs.
core-policy-task-runs-other = sie führt etwas aus, das dieses Paket nicht installiert: { $path }
core-policy-task-exists-unreadable = eine Aufgabe mit diesem Namen ist bereits vorhanden und konnte nicht gelesen werden
core-policy-task-exists-other = eine Aufgabe mit diesem Namen ist bereits vorhanden und führt die App nicht aus
# { $path } is the program the task already on this PC runs.
core-policy-task-exists-runs = eine Aufgabe mit diesem Namen ist bereits vorhanden und führt { $path } aus, und diese Datei stammt nicht aus einer früheren Installation hier
core-policy-shortcut-not-lnk = sie ist keine .lnk-Verknüpfung
core-policy-shortcut-place = sie befindet sich weder in einem Startmenü noch auf einem Desktop
core-policy-short-name = der Pfad verwendet einen Namen, der nicht geprüft werden kann
core-policy-shortcut-no-folder = dieser PC hat keinen solchen Ordner dafür
core-policy-data-startup = es käme in einen Autostart-Ordner und würde bei jeder Anmeldung ausgeführt
core-policy-data-powershell = es käme an einen Ort, an dem PowerShell beim Start ausführt, was es dort findet
core-policy-data-windows = es käme in einen Ordner, der zu Windows gehört
core-policy-license-outside = es befindet sich außerhalb des Installationsorts und aller bekannten Datenordner
core-rewrite-cdata = es enthält einen CDATA-Abschnitt
core-rewrite-too-much-to-look-through = dieses Paket enthält zu viel in seinen Einstellungen, um zu prüfen, was sie nennen

## Warnings a review gives, which the installer shows with what it sets up.

# { $kind } says what was left out, and is not shown itself. { $item } is its path or name, or a registry value such
# as HKLM\Software\App\Name. { $reason } is why, as the second half of the sentence, such as "it cannot be a whole
# drive".
core-warning-review-left-out =
    { $kind ->
        [file] Die Datei { $item } wurde ausgelassen: { $reason }.
        [folder] Der Ordner { $item } wurde ausgelassen: { $reason }.
        [service] Der Dienst { $item } wurde ausgelassen: { $reason }.
        [driver] Der Treiber { $item } wurde ausgelassen: { $reason }.
        [firewall-rule] Die Firewallregel { $item } wurde ausgelassen: { $reason }.
        [license-data] Die Lizenzdaten unter { $item } wurden ausgelassen: { $reason }.
        [license-value] Der Lizenzwert { $item } wurde ausgelassen: { $reason }.
        [scheduled-task] Die geplante Aufgabe { $item } wurde ausgelassen: { $reason }.
        [shortcut] Die Verknüpfung { $item } wurde ausgelassen: { $reason }.
        [environment-variable] Die Umgebungsvariable { $item } wurde ausgelassen: { $reason }.
       *[value] { $item } wurde ausgelassen: { $reason }.
    }
core-warning-review-package-path-missing = Eine kopierbare Datei hat keinen relativen Pfad im Paket.
# { $count } is the most empty folders a package makes.
core-warning-review-too-many-empty-folders = Dieses Paket führt mehr als { $count } leere Ordner auf, und nur so viele werden erstellt.
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Documents folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-documents-elsewhere =
    { $count ->
        [one] { $count } Datei aus „Dokumente“ wird stattdessen unter { $target } in einem Ordner namens _external abgelegt, da sich der Ordner „Dokumente“ dieses PCs unter { $folder } befindet und nur in einen Ordner innerhalb Ihres Profils geschrieben wird. Um die Datei zu verwenden, verschieben Sie sie selbst in Ihren Ordner „Dokumente“.
       *[other] { $count } Dateien aus „Dokumente“ werden stattdessen unter { $target } in einem Ordner namens _external abgelegt, da sich der Ordner „Dokumente“ dieses PCs unter { $folder } befindet und nur in einen Ordner innerhalb Ihres Profils geschrieben wird. Um die Dateien zu verwenden, verschieben Sie sie selbst in Ihren Ordner „Dokumente“.
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-documents-unknown =
    { $count ->
        [one] { $count } Datei aus „Dokumente“ wird stattdessen unter { $target } in einem Ordner namens _external abgelegt, da nicht ermittelt werden konnte, wo sich der Ordner „Dokumente“ dieses PCs befindet. Um die Datei zu verwenden, verschieben Sie sie selbst in Ihren Ordner „Dokumente“.
       *[other] { $count } Dateien aus „Dokumente“ werden stattdessen unter { $target } in einem Ordner namens _external abgelegt, da nicht ermittelt werden konnte, wo sich der Ordner „Dokumente“ dieses PCs befindet. Um die Dateien zu verwenden, verschieben Sie sie selbst in Ihren Ordner „Dokumente“.
    }
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Saved Games folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-elsewhere =
    { $count ->
        [one] { $count } Datei aus „Gespeicherte Spiele“ wird stattdessen unter { $target } in einem Ordner namens _external abgelegt, da sich der Ordner „Gespeicherte Spiele“ dieses PCs unter { $folder } befindet und nur in einen Ordner innerhalb Ihres Profils geschrieben wird. Um die Datei zu verwenden, verschieben Sie sie selbst in Ihren Ordner „Gespeicherte Spiele“.
       *[other] { $count } Dateien aus „Gespeicherte Spiele“ werden stattdessen unter { $target } in einem Ordner namens _external abgelegt, da sich der Ordner „Gespeicherte Spiele“ dieses PCs unter { $folder } befindet und nur in einen Ordner innerhalb Ihres Profils geschrieben wird. Um die Dateien zu verwenden, verschieben Sie sie selbst in Ihren Ordner „Gespeicherte Spiele“.
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-unknown =
    { $count ->
        [one] { $count } Datei aus „Gespeicherte Spiele“ wird stattdessen unter { $target } in einem Ordner namens _external abgelegt, da nicht ermittelt werden konnte, wo sich der Ordner „Gespeicherte Spiele“ dieses PCs befindet. Um die Datei zu verwenden, verschieben Sie sie selbst in Ihren Ordner „Gespeicherte Spiele“.
       *[other] { $count } Dateien aus „Gespeicherte Spiele“ werden stattdessen unter { $target } in einem Ordner namens _external abgelegt, da nicht ermittelt werden konnte, wo sich der Ordner „Gespeicherte Spiele“ dieses PCs befindet. Um die Dateien zu verwenden, verschieben Sie sie selbst in Ihren Ordner „Gespeicherte Spiele“.
    }

## Errors when a folder that only administrators may change cannot be made so, or checked.

# { $path } is a folder.
core-privilege-not-a-folder = { $path } ist ein Link oder eine Datei, kein Ordner
# { $path } is a folder. { $error } is what Windows said, in its own words.
core-privilege-not-made-here = { $path } wurde nicht von WinAppMigrator erstellt, und andere können den Inhalt ändern. Ein Administrator sollte den Ordner prüfen und entfernen: { $error }
# { $path } is a folder.
core-privilege-others-put-something-in = { $path } enthält etwas, das nicht von WinAppMigrator erstellt wurde, und andere können es ändern. Ein Administrator sollte den Ordner prüfen und entfernen
# { $path } is a folder.
core-privilege-became-a-link-in-use = { $path } wurde während der Verwendung zu einem Link
# { $path } is a folder.
core-privilege-became-a-link-looked-at = { $path } wurde während der Prüfung zu einem Link
# { $path } is a file. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-users-cannot-start = Benutzern konnte das Starten von { $path } nicht erlaubt werden: { $said }
core-privilege-elevation-unknown = es konnte nicht ermittelt werden, ob dieser Prozess mit erhöhten Rechten ausgeführt wird
# { $path } is a folder. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-cannot-restrict = { $path } konnte nicht auf Administratoren beschränkt werden: { $said }
# { $path } is a folder. { $said } is the folder's permissions as Windows wrote them, and what else it printed.
core-privilege-not-restricted = { $path } konnte nicht auf Administratoren beschränkt werden, und das Laufwerk speichert möglicherweise keine Berechtigungen: { $said }
# { $path } is a folder.
core-privilege-written-to-first = ein anderer Prozess hat in { $path } geschrieben, bevor der Ordner geschützt werden konnte
core-privilege-no-program-data = Windows gibt nicht an, wo sich ProgramData befindet

## Checks of a package's files, and of what an install put in place. Each is said of one file, value, service, or the
## like, which the report names beside it.

# { $path } is the installer's file.
core-verify-no-package = { $path } enthält kein Paket
# { $path } is a folder.
core-verify-not-unpacked = das Paket dieses Installationsprogramms wurde nicht nach { $path } entpackt
core-verify-schema-supported = Das Schema des Paketmanifests wird unterstützt.
core-verify-packaged-file-missing = Die gepackte Datei fehlt.
core-verify-no-package-path = Die kopierbare Datei hat keinen relativen Pfad im Paket.
core-verify-folder-there = Der leere Ordner ist vorhanden.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-unreadable = Das Element konnte zur Prüfung nicht gelesen werden: { $error }
core-verify-restored-file-missing = Die wiederhergestellte Datei fehlt.
# { $expected } and { $actual } are sizes in bytes.
core-verify-size-mismatch = Die Dateigröße stimmt nicht überein. Erwartet: { $expected } Bytes, gefunden: { $actual } Bytes.
core-verify-no-hash = Das Manifest enthält keinen Hash für diese Datei, daher kann sie nicht geprüft werden.
core-verify-hash-mismatch = Der SHA-256-Hash stimmt nicht überein.
core-verify-file-matches = Dateigröße und Hash stimmen mit dem Manifest überein.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-payload-failed = Die Überprüfung der Lizenznutzdaten ist fehlgeschlagen: { $error }
core-verify-license-payload-missing = Die gepackten Lizenznutzdaten fehlen.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-failed = Die Überprüfung des Lizenzspeichers ist fehlgeschlagen: { $error }
core-verify-license-store-file-missing = Die gepackte Lizenzspeicherdatei fehlt.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-path-invalid = Der Pfad des gepackten Lizenzspeichers ist ungültig: { $error }
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-prerequisite-failed = Die Überprüfung der Nutzdaten der erforderlichen Komponente ist fehlgeschlagen: { $error }
core-verify-prerequisite-missing = Die Nutzdaten der gebündelten erforderlichen Komponente fehlen.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-failed = Die Überprüfung der Treiberdatei ist fehlgeschlagen: { $error }
core-verify-bundled-driver-missing = Die gebündelte Treiberdatei fehlt.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-path-invalid = Der Pfad des gebündelten Treibers ist ungültig: { $error }
core-verify-prerequisite-present = Die erforderliche Laufzeitkomponente ist auf dem Zielcomputer vorhanden.
core-verify-prerequisite-not-detected = Die erforderliche Laufzeitkomponente wurde nicht erkannt. Möglicherweise ist ein Neustart oder ein manueller Abschluss erforderlich (das maßgebliche Ergebnis steht im Wiederherstellungsprotokoll).
core-verify-prerequisite-not-probed = Ob die erforderliche Komponente vorhanden ist, lässt sich nicht über die Registrierung prüfen. Die Installation erfolgte nach bestem Bemühen.
core-verify-license-file-missing = Die wiederhergestellte Lizenzdatei fehlt.
core-verify-license-store-restored-missing = Die wiederhergestellte Lizenzspeicherdatei fehlt.
core-verify-registry-key-there = Der Registrierungsschlüssel ist vorhanden.
core-verify-registry-value-matches = Der Registrierungswert stimmt mit den archivierten Daten überein.
core-verify-registry-value-differs = Der Registrierungswert stimmt nicht mit den archivierten Daten überein.
core-verify-firewall-rule-there = Die Windows-Firewall enthält diese Regel.
core-verify-variable-matches = Der Wert der Umgebungsvariablen stimmt mit den archivierten Daten überein.
core-verify-variable-differs = Der Wert der Umgebungsvariablen stimmt nicht mit den archivierten Daten überein.
# { $error } is what Windows said, in its own words.
core-verify-variable-unreadable = Die Umgebungsvariable fehlt oder ist nicht lesbar: { $error }
core-verify-service-runs-other = Der Dienst ist vorhanden, führt aber nicht die wiederhergestellte Binärdatei mit ihren Argumenten aus.
core-verify-service-other-account = Der Dienst ist vorhanden, wird aber nicht unter dem erwarteten Konto ausgeführt.
core-verify-service-matches = Der Dienst ist vorhanden und führt die wiederhergestellte Binärdatei mit ihren Argumenten unter dem erwarteten Konto aus.
core-verify-service-missing = Der Dienst fehlt.
# { $error } is what Windows said, in its own words.
core-verify-service-unreadable = Der Dienst ist nicht lesbar: { $error }
core-verify-driver-other-image = Der Treiberdienst verweist nicht auf die wiederhergestellte Treiberdatei.
core-verify-driver-not-registered = Der gebündelte Treiberdienst ist nach der Wiederherstellung nicht registriert.
# { $error } is what Windows said, in its own words.
core-verify-driver-unreadable = Der Treiberdienst ist nicht lesbar: { $error }
core-verify-driver-image-missing = Die Treiberdatei fehlt.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-unchecked = Die Treiberdatei konnte nicht geprüft werden: { $error }
core-verify-task-matches = Die geplante Aufgabe ist vorhanden und führt die wiederhergestellten Programme aus.
core-verify-task-runs-other = Die geplante Aufgabe ist vorhanden, führt aber nicht die wiederhergestellten Programme aus.
core-verify-task-missing = Die geplante Aufgabe fehlt.
# { $error } is what Windows said, in its own words.
core-verify-task-unreadable = Die Abfrage der geplanten Aufgabe ist fehlgeschlagen: { $error }
core-verify-shortcut-matches = Die Verknüpfung ist vorhanden, und ihr Zielpfad entspricht dem wiederhergestellten Ziel.
core-verify-shortcut-opens-other = Die Verknüpfung ist vorhanden, aber ihr Zielpfad entspricht nicht dem wiederhergestellten Ziel.
core-verify-shortcut-missing = Die Verknüpfung fehlt.
# { $error } is what Windows said, in its own words.
core-verify-shortcut-unreadable = Die Abfrage der Verknüpfung ist fehlgeschlagen: { $error }

## Warnings and errors a check gives, which the app and the installer show.

core-warning-verify-schema-unsupported = Das Schema des Paketmanifests wird nicht unterstützt.
# { $error } is what went wrong, in Windows' or the program's own words.
core-warning-verify-package-file-failed = Die Überprüfung der Paketdatei ist fehlgeschlagen: { $error }
core-warning-verify-folder-missing = Der leere Ordner fehlt.
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-key-missing = Der Registrierungsschlüssel fehlt oder ist nicht lesbar: { $error }
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-value-missing = Der Registrierungswert fehlt oder ist nicht lesbar: { $error }
core-warning-verify-registry-kind-missing = Der Typ des Registrierungswerts fehlt.
core-warning-verify-firewall-rule-missing = Die Windows-Firewall enthält diese Regel nicht.
core-warning-verify-service-scope = Die Dienstüberprüfung liest die Dienstkonfiguration aus HKLM und startet keine Dienste.
core-warning-verify-driver-manual = Mindestens ein Treiber eines Herstellers oder von Windows wird von WinAppMigrator nicht weitergegeben. Installieren Sie diese Treiber auf dem Zielcomputer mit dem signierten Treiberpaket des Herstellers neu.
core-warning-verify-task-scope = Die Überprüfung geplanter Aufgaben prüft nur die Registrierung und den Befehlsverweis.

## The verification report written beside a package, verification_report.md. Its headings follow a "#".

core-verify-report-title = WinAppMigrator-Überprüfungsbericht
# { $time } is when, as a date and time in UTC, such as 2026-11-15T10:00:00Z.
core-verify-report-verified = Überprüft (UTC): { $time }
# { $valid } is true or false, which are not translated.
core-verify-report-package-valid = Paket gültig: { $valid }
# { $valid } is true or false, which are not translated.
core-verify-report-restore-valid = Wiederherstellung gültig: { $valid }
core-verify-report-restore-not-run = Wiederherstellung gültig: nicht ausgeführt
core-verify-report-items = Elemente
# { $message } is what a check found, and { $source } the file or setting it looked at.
core-verify-report-item-from = { $message } Quelle: { $source }
core-verify-report-warnings = Warnungen
core-verify-report-errors = Fehler
core-verify-report-none = Keine.
