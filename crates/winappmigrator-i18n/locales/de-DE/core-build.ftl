# Words the core writes for people while it builds a package from an installed app: what a package records for an
# installer to show later, errors shown in the app, the list of what goes in and what is left out, opening a disk
# image or another PC over the network, and what an installer says when the package it carries cannot be read.
# Message names start with core-.


## Scanning an app, before a package is built. Shown in the app when a scan cannot go on.

# { $path } is the app's program, such as D:\Program Files\Acme\Acme.exe, on a drive taken from another PC.
core-scan-program-not-on-the-drive = { $path } führt über einen Link oder befindet sich nicht auf dem angeschlossenen Laufwerk, daher wurde die Datei nicht gelesen.
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-not-on-this-pc = { $path } befindet sich nicht auf einem Datenträger dieses PCs oder hat einen Namen, der etwas anderes öffnen würde, daher wurde die Datei nicht gelesen.
# { $path } is the app's program.
core-scan-program-someone-elses = { $path } befindet sich in den Ordnern eines anderen Kontos oder gehört nicht zu diesem Konto, und der eigene Installationseintrag dieser App gibt nicht an, dass die Datei dorthin gehört, daher wurde sie nicht gelesen.
# { $app_name } is the app's name, as the list of apps calls it.
core-scan-no-executable = für { $app_name } wurde keine ausführbare Datei gefunden. Ein Treiber, eine Laufzeitkomponente oder ein Add-In hat möglicherweise kein eigenes Programm, das gepackt werden kann.
# { $path } is the file that was picked.
core-scan-not-a-file = das Analyseziel ist keine Datei: { $path }
# Names something left out of a package, in a list of what was left out and why. { $name } is the variable's name,
# such as ACME_LICENSE_KEY.
core-scan-environment-variable-item = Die Umgebungsvariable { $name }

## Folders and parts that might also be the app's, offered in a list to tick. Each says why it is offered.

core-near-starts-with-app-name = Der Name beginnt mit dem Namen dieser App.
core-near-has-app-name = Der Name enthält den Namen dieser App.
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-starts-with-publisher = Der Name beginnt mit dem Namen des Herausgebers, { $publisher }.
# { $folder } is a folder's name, such as Program Files.
core-near-hidden-part-beside = Ein in der App-Liste ausgeblendeter Teil, der neben dieser App in { $folder } installiert ist.
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-hidden-part-publisher = Ein in der App-Liste ausgeblendeter Teil desselben Herausgebers, { $publisher }.

## What a scan found, as a person chooses from it: the folders that go in, and what is left out, and why

# Why a folder cannot be left out, beside its number of files.
core-footprint-locked-app-folder = Gehören immer zum Hauptprogramm
core-footprint-locked-main-program = Enthält das Hauptprogramm, das immer aufgenommen wird
core-footprint-locked-needed = Enthält, was das Hauptprogramm zum Starten benötigt
core-footprint-locked-font = Gehört zur Schriftart unter „Einbindung in Windows“
# Where a folder is. "AppData" and "ProgramData" are the names of Windows folders; keep them as they are.
core-footprint-place-roaming = Einstellungen in Ihrem Profil (AppData)
core-footprint-place-local = Einstellungen für diesen PC in Ihrem Profil (AppData)
core-footprint-place-program-data = Von allen auf diesem PC gemeinsam genutzt (ProgramData)
core-footprint-place-documents = Im Ordner „Dokumente“
core-footprint-place-saved-games = Im Ordner „Gespeicherte Spiele“
core-footprint-place-public-documents = Im Ordner „Öffentliche Dokumente“
core-footprint-place-app-folder = Im Ordner der App
# { $folder } is a folder, such as C:\Users\Sam\AppData\Roaming.
core-footprint-place-in = In { $folder }
# A row for the files directly in the app's folder. { $folder } is that folder's name, such as Acme.
core-footprint-files-directly-in = Dateien direkt in { $folder }
# Headings over what is left out, and why each item is.
core-footprint-licenses-left-out = Nicht in diesem Installationsprogramm, da die Option „Lizenz- und Aktivierungsdaten“ deaktiviert ist
core-footprint-font-stays-behind = Die Schriftart bleibt zurück.
core-footprint-drivers-left-out = Treiber, die zu Windows oder einem Hersteller gehören und nicht weitergegeben werden können. Installieren Sie sie daher auf dem anderen PC neu
core-footprint-driver-reinstall = Installieren Sie ihn mit dem eigenen Paket des Herstellers neu.
# Why something else is left out along with what the person left out. { $service } is a service's name, such as AcmeSync.
core-footprint-needs-left-out = es benötigt { $service }, und dieser Dienst wird ausgelassen
core-footprint-told-to-use-left-out = es soll etwas in einem Ordner verwenden, den Sie ausgelassen haben
core-footprint-file-in-left-out = seine Datei befindet sich in einem Ordner, den Sie ausgelassen haben
core-footprint-program-in-left-out = das zugehörige Programm befindet sich in einem Ordner, den Sie ausgelassen haben
core-footprint-names-left-out = es nennt einen Ordner, den Sie ausgelassen haben
# Why something is left out, as the installer tells whoever installs it.
core-footprint-builder-left-it-out = die Person, die dieses Installationsprogramm erstellt hat, hat es ausgelassen
# Something the person left out, by kind and name, such as "Service Acme Updater". { $kind } is the kind, such as
# Service or Scheduled Task; { $title } is its name.
core-footprint-left-out-item = { $kind } { $title }

## Building a package and an installer from a scan. Shown in the app when a build cannot go on.

core-package-scan-of-another-app = diese Analyse betrifft eine andere App als die, die erstellt wird, daher wurde nichts erstellt
core-package-scan-brought-in-more = diese Analyse enthält Daten, die für diese Erstellung nicht angefordert wurden, daher wurde nichts erstellt
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-package-program-gone = { $path } ist nicht mehr vorhanden, daher wurde nichts erstellt. Analysieren Sie die App erneut.
core-package-needs-stub = ein selbstextrahierendes Archiv erfordert einen Installationsprogramm-Stub
# { $reason } is what went wrong.
core-package-staged-file-failed = eine Datei im Bereitstellungsordner konnte nicht in das Paket aufgenommen werden: { $reason }
# { $shortfalls } says which drives are short of room and by how much, in sentences of its own.
core-package-not-enough-room = es ist nicht genügend Speicherplatz zum Erstellen dieses Installationsprogramms vorhanden, daher wurde nichts gelesen oder geschrieben. { $shortfalls }. Geben Sie dort Speicherplatz frei, oder wählen Sie einen Ausgabeordner auf einem anderen Laufwerk aus.
# { $path } is a file's path, on a drive taken from another PC.
core-package-not-read-off-the-drive = { $path } führt über einen Link oder befindet sich nicht auf dem angeschlossenen Laufwerk, daher wurde die Datei nicht gelesen
# { $path } is a file's path.
core-package-not-read-not-local = { $path } befindet sich nicht auf einem Datenträger dieses PCs, daher wurde die Datei nicht gelesen
# { $path } is a file's path.
core-package-not-read-other-account = { $path } führt in die Ordner eines anderen Kontos, daher wurde die Datei nicht gelesen
# { $path } is a file's path, as the PC it came from called it.
core-package-not-on-the-drive = { $path } befindet sich nicht auf dem angeschlossenen Laufwerk, daher wurde die Datei nicht gelesen
# { $path } is a file's path.
core-package-backup-copy-no-parent = eine Datei ohne übergeordneten Ordner kann nicht als Sicherung kopiert werden: { $path }
# { $path } is a file's path.
core-package-backup-copy-destination-no-parent = das Ziel der Sicherungskopie hat keinen übergeordneten Ordner: { $path }
# robocopy is the name of a Windows command; keep it as it is. { $code } is how it ended, such as Some(8).
core-package-robocopy-failed = Kopieren mit robocopy im Sicherungsmodus fehlgeschlagen (Exitcode { $code })
# Stands for a folder where a file's name would be, as in "I/O failed for temporary package folder: …".
core-package-temporary-folder = temporärer Paketordner
# Stands for the installer being written where a file's name would be, as in "I/O failed for the installer: …".
core-package-the-installer = das Installationsprogramm
# { $path } is a file's path, or its name in the package. { $error } is what went wrong with it.
core-package-path-error = { $path }: { $error }
# { $name } is a file's name in the package, such as app/bin/App.exe.
core-package-same-place = eine andere Datei hat denselben Platz im Paket: { $name }
# { $name } is a file's name in the package.
core-package-changed-in-protected-folder = { $name } wurde im geschützten Ordner geändert, während das Paket erstellt wurde
core-package-worker-died = ein Thread, der Dateien komprimiert, wurde unerwartet beendet, daher wurde kein Installationsprogramm geschrieben
# { $path } is a file's path.
core-package-link-not-read = { $path } ist ein Link und wurde daher nicht gelesen
# { $path } is a file's path. { $error } is what went wrong.
core-package-not-compressed = { $path } konnte nicht komprimiert werden: { $error }
# { $path } is a file's path. { $reason } is what went wrong.
core-package-not-read-back = { $path } konnte nicht zurückgelesen werden: { $reason }

## Where an app's files are, and where an installer may write them. Shown when a path is refused.

# { $path } is a folder, such as C:\Program Files\Acme.
core-discovery-root-not-full-path = der Ordner, unter den geschrieben wird, muss ein vollständiger Pfad sein: { $path }
# { $path } is a folder's path.
core-discovery-root-not-a-folder = der Ordner, unter den geschrieben wird, ist kein Ordner: { $path }
# { $path } is a file's path.
core-discovery-escapes-root = der Pfad führt aus dem vorgesehenen Stammordner heraus: { $path }
# { $path } is the link's path.
core-discovery-refusing-to-write-through-link = Schreiben über einen Link wird verweigert: { $path }
# { $path } is the link's path.
core-discovery-refusing-link = Zugriff über einen Link wird verweigert: { $path }
# { $error } is what Windows said went wrong.
core-discovery-refusing-link-because = Zugriff über einen Link wird verweigert: { $error }
# { $path } is the link's path.
core-discovery-link-to-somewhere-else = { $path } ist ein Link an einen anderen Ort, und andere können möglicherweise ändern, wohin er führt. Verwenden Sie den Ort, zu dem er führt
# { $path } is a folder's path.
core-discovery-no-parent-folder = { $path } hat keinen übergeordneten Ordner
# { $path } is the install location as it was given. Keep the quotes, or use your language's own.
core-discovery-install-location-not-full-path = der Installationsort muss ein vollständiger Pfad sein: „{ $path }“
# { $path } is the install location.
core-discovery-install-location-a-link = der Installationsort ist ein Link an einen anderen Ort: { $path }
# { $path } is a file's path.
core-discovery-device-name = der Pfad verwendet einen Namen, der für ein Windows-Gerät reserviert ist: { $path }
# { $path } is a file's path.
core-discovery-device-name-on-older-windows = { $path } hat einen Namen, den diese Windows-Version als Gerät und nicht als Datei öffnet, daher wurde nichts hineingeschrieben. Die Installation ist unter Windows 11 möglich
# { $path } is a file's path.
core-discovery-unsafe-path = unsicherer Pfad: { $path }
# { $path } is a file's path.
core-discovery-no-package-path = die Datei hat keinen relativen Pfad im Paket: { $path }

## Writing an installer, and reading the package inside one. Most of these are shown by an installer, on the PC it
## runs on, when it cannot read what it carries. "The package" is the app's files and settings that an installer
## carries; "the stub" is the program part of an installer, before the package; "the data file" is the .wamdata file
## that holds the package of a very large installer, beside it.

# Stands for the package where a file's name would be, as in "I/O failed for installer package: …".
core-archive-installer-package = Paket des Installationsprogramms
# Stands for the installer program built into WinAppMigrator, where a file's name would be.
core-archive-embedded-stub = eingebetteter Installationsprogramm-Stub
# { $index } is the number of a file in the package. { $reason } is what went wrong.
core-archive-entry-unreadable = Eintrag { $index } des Pakets konnte nicht gelesen werden: { $reason }
# { $name } is a file's name inside the package, such as app/bin/App.exe.
core-archive-entry-unsafe-path = ein ZIP-Eintrag hat einen unsicheren Pfad: { $name }
# { $name } is a file's name inside the package.
core-archive-holds-more = { $name } enthält mehr, als angegeben ist
# { $name } is a file's name inside the package.
core-archive-name-too-long = dieser Name ist für ein ZIP-Archiv zu lang: { $name }
core-archive-changed-while-copied = das Paket wurde geändert, während es hinter den Stub geschrieben wurde, daher wurde kein Installationsprogramm geschrieben
core-archive-stub-not-a-program = der Teil dieses Installationsprogramms vor seinem Paket ist kein Programm
core-archive-damaged = dieses Installationsprogramm ist beschädigt oder wurde nach der Erstellung geändert, daher wurde nichts aus seinem Paket gelesen. Kopieren Sie es erneut von dem Ort, an dem es erstellt wurde.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-missing = dieses Installationsprogramm speichert sein Paket in einer Datei namens { $name } im selben Ordner, und diese Datei ist nicht vorhanden. Die beiden Dateien müssen zusammenbleiben: Kopieren Sie { $name } von dem Ort, an dem das Installationsprogramm erstellt wurde, hierher, und führen Sie es dann erneut aus.
# { $name } is the data file's name, such as App_installer.wamdata. { $error } is what Windows said went wrong.
core-archive-data-file-unopened = die Datendatei dieses Installationsprogramms, { $name }, konnte nicht geöffnet werden: { $error }. Die beiden Dateien müssen zusammen in einem Ordner bleiben, den dieser PC lesen kann.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-a-link = { $name } ist ein Link oder ein Ordner, aber die Datendatei dieses Installationsprogramms muss eine eigenständige Datei direkt daneben sein, daher wurde nichts daraus gelesen. Kopieren Sie die echte Datei hierher.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-wrong = { $name } ist nicht die Datendatei dieses Installationsprogramms oder ist beschädigt oder wurde nach der Erstellung geändert, daher wurde nichts daraus gelesen. Kopieren Sie beide Dateien erneut von dem Ort, an dem sie erstellt wurden.
core-archive-stub-signed = der Installationsprogramm-Stub ist signiert. Er muss unsigniert bleiben, signieren Sie daher stattdessen das fertige Installationsprogramm
# { $size } is a size, such as 4.2 GB.
core-archive-too-big-to-run = das Installationsprogramm wäre { $size } groß, und Windows kann keine ausführbare Datei mit 4 GB oder mehr ausführen. Es wurde nichts geschrieben
core-archive-outside-signature = das Installationsprogramm enthält Daten außerhalb seiner Signatur und wurde möglicherweise manipuliert
core-archive-footer-length = die Länge der Fußzeile des selbstextrahierenden Archivs ist ungültig
core-archive-seek-outside = Positionierung außerhalb des Pakets
# { $path } is a file's path.
core-archive-unsafe-package-path = unsicherer Paketpfad: { $path }
# { $path } is a file's path.
core-archive-empty-package-path = leerer Paketpfad: { $path }

## Opening a disk image of another PC (a .vhd or .vhdx file) to read apps from it

# { $path } is the disk image file, such as D:\Backups\OldPC.vhdx.
core-image-not-attachable = { $path } ist kein Datenträgerabbild, das Windows anfügen kann (.vhd oder .vhdx)
core-image-needs-administrator = zum Anfügen eines Datenträgerabbilds sind Administratorrechte erforderlich
# { $path } is the disk image file the person picked.
core-image-not-a-file = { $path } ist keine Datei
# { $path } is the disk image file. "Edit Disk" is the name of a Hyper-V Manager command.
core-image-differencing = { $path } ist ein differenzierender Datenträger, der nur die Änderungen seit dem Datenträger enthält, aus dem er erstellt wurde, und Windows würde auch diesen Datenträger öffnen, wo immer er laut Angabe liegt. Führen Sie ihn zuerst mit „Datenträger bearbeiten“ in Hyper-V mit diesem Datenträger zusammen, und wählen Sie dann diesen aus
# { $path } is the disk image file.
core-image-unreadable = { $path } ist keine lesbare .vhd- oder .vhdx-Datei
# ProgramData is the name of a Windows folder; keep it as it is.
core-image-no-program-data = der ProgramData-Ordner dieses PCs wurde nicht gefunden
# The reason given in core-image-could-not-attach when Windows already has the image open.
core-image-already-attached = Windows hat das Abbild bereits angefügt, und was es angefügt hat, schreibt möglicherweise darauf. Trennen Sie es zuerst, oder wählen Sie sein Laufwerk als angeschlossenes Laufwerk aus
# { $path } is the disk image file. { $reason } is why, such as core-image-already-attached or what Windows said.
core-image-could-not-attach = { $path } konnte nicht angefügt werden: { $reason }
# { $path } is the disk image file.
core-image-no-windows-volume = { $path } enthält kein lesbares Volume mit Windows. Ein mit BitLocker verschlüsseltes Volume muss zuerst entsperrt werden.
# Given as the reason when Windows failed without saying anything.
core-image-windows-did-not-say = Windows hat keinen Grund angegeben

## Reading another PC over the network, through the client running on it. Shown as why something on that PC
## could not be read. "The client" is WinAppMigrator Client, the small program on the other PC.

core-remote-too-slow = der andere PC hat zu lange nicht geantwortet
core-remote-closed = die Verbindung zum anderen PC ist geschlossen
core-remote-failed = die Verbindung zum anderen PC ist fehlgeschlagen
core-remote-answered-something-else = der Client hat etwas anderes geantwortet
# { $name } is the PC's name or address as the person typed it, such as OLDPC or 192.168.1.20.
core-remote-not-found = { $name } wurde im Netzwerk nicht gefunden
core-remote-did-not-say-what-it-is = der Client hat nicht angegeben, was er ist
core-remote-ordinary-names-only = abgefragt wird nur ein Pfad unterhalb des Stammverzeichnisses des anderen PCs, mit gewöhnlichen Namen
core-remote-more-but-none = der andere PC hat gemeldet, dass ein Ordner weitere Inhalte hat, aber keine davon übermittelt
# { $count } is the most things one folder may hold, such as 250,000.
core-remote-too-many-listed = der andere PC hat mehr als { $count } Elemente in einem Ordner aufgelistet
core-remote-sent-too-much = der andere PC hat mehr von einer Datei gesendet als angefordert
core-remote-name-not-text = ein Name, der kein Text ist, kann nicht abgefragt werden
core-remote-below-root-only = nur ein Pfad unterhalb des Stammverzeichnisses des anderen PCs kann abgefragt werden
core-remote-file-ended-early = die Datei des anderen PCs endete vorzeitig
core-remote-link-not-followed = ein Link auf einem anderen PC wird nicht verfolgt
core-remote-only-files-opened = auf einem anderen PC werden nur Dateien geöffnet
core-remote-listing-said-nothing = die Auflistung enthielt keine Angaben dazu
core-remote-change-time-unknown = der Zeitpunkt der letzten Änderung ist unbekannt
core-remote-seek-before-start = eine Positionierung vor den Anfang
# { $size } is how many bytes a file is; { $limit } is the most that were expected. Both are numbers.
core-remote-too-large =
    { $size ->
        [one] die Datei ist { $size } Byte groß, und es wurden höchstens { $limit } erwartet
       *[other] die Datei ist { $size } Bytes groß, und es wurden höchstens { $limit } erwartet
    }

## Checking that a download is signed by Microsoft. Each is given as the reason a check failed.

# { $error } is what Windows said went wrong.
core-signature-no-powershell = PowerShell konnte nicht gestartet werden: { $error }
# { $error } is what went wrong reading PowerShell's answer.
core-signature-unreadable = die Signatur konnte nicht gelesen werden: { $error }
# { $signer } is who signed the file, such as Contoso Ltd.
core-signature-not-microsoft = die Datei ist von { $signer } signiert, nicht von Microsoft
core-signature-not-microsoft-unnamed = die Datei ist von jemand anderem signiert, nicht von Microsoft
# { $status } is Windows' own word for the signature's state, such as HashMismatch or NotSigned, which is not translated.
core-signature-status = der Signaturstatus lautet { $status }

## Warnings a package records when it is built from an app, which an installer shows later, maybe on another PC, in
## that PC's language. "This app" is the app being packaged.

core-warning-build-program-needs-unreadable = Was dieses Programm von Windows benötigt, konnte nicht gelesen werden, daher wird nicht geprüft, ob es auf dem anderen PC ausgeführt werden kann.
core-warning-build-external-manifest = Externes Anwendungsmanifest erkannt. Die Auswertung von Side-by-Side-Abhängigkeiten wird zurückgestellt.
core-warning-build-non-exe = Die ausgewählte Datei hat nicht die Erweiterung .exe. Die PE-Analyse wird fortgesetzt.
core-warning-build-stand-in-program = Es wurde keine gewöhnliche ausführbare Datei der Anwendung gefunden, daher steht diese Datei stellvertretend dafür. Die Ergebnisse zu Architektur und Abhängigkeiten beschreiben diese Datei und treffen möglicherweise nicht auf die Anwendung zu.
core-warning-build-other-apps-files-unknown = Nicht alles, was Windows Installer über andere Apps verzeichnet hat, konnte gelesen werden. Ein Ordner, der für diese App verzeichnet ist, kann daher Dateien einer anderen App enthalten, ohne dass hier etwas darauf hinweist.
core-warning-build-app-folder-has-another-apps-file = Laut Windows Installer hat eine andere App eine Datei in dem Ordner, in dem sich das Programm dieser App befindet. Diese Datei wird mit übernommen, da der gesamte Ordner übernommen wird.
# "App Paths" is the name of a Windows registry key; keep it as it is.
core-warning-build-app-paths-folder-left-out = Der Eintrag dieser App unter App Paths gibt an, dass sie in einem Ordner sucht, der nicht ihr eigener ist, daher wird dieser Ordner nicht übernommen. Auf einem PC ohne dessen Inhalt startet die App möglicherweise nicht.
core-warning-build-uninstaller-folder-left-out = Der Ordner, in dem sich das Deinstallationsprogramm dieser App befindet, wurde ausgelassen, da laut Windows Installer eine andere App dort eine Datei hat.
# { $why } is never shown: it chooses the reason. Translate each reason as the end of the sentence.
core-warning-build-recorded-folder-left-out =
    Ein für diese App verzeichnetes Element nennt einen Ordner, der ausgelassen wurde, da { $why ->
        [unreadable] sein Pfad das Laufwerk verlässt oder über einen Link führt und er daher nicht gefahrlos gelesen werden kann
        [missing] er nicht vorhanden ist
        [link] er über einen Link führt
        [short-name] er nicht so geschrieben ist, wie er tatsächlich heißt
        [another-account] er sich in den Ordnern eines anderen Kontos befindet
        [one-account] diese App für ein einzelnes Konto installiert wurde und dieser Ordner nicht zu diesem Konto gehört
        [shared] er gemeinsam genutzte Inhalte enthält oder zu Windows gehört
        [generic-name] sein Name nichts darüber aussagt, wem er gehört
        [publisher] er nur nach dem Herausgeber benannt ist und dessen andere Produkte enthalten kann
        [holds-the-app] er den eigenen Ordner dieser App enthält und mehr enthalten kann
        [another-apps-name] er den Namen einer anderen App trägt
        [another-apps-record] sein Installationseintrag zu einer anderen App gehört
        [in-another-apps-folder] er sich im Ordner einer anderen App befindet
       *[another-apps] er zu einer anderen App gehört
    }.
core-warning-build-app-folder-holds-another-app = Eine andere App auf diesem PC ist als im Ordner dieser App befindlich verzeichnet und wird mit übernommen, da der gesamte Ordner übernommen wird.
# "Program Files (x86)" is the name of a Windows folder; keep it as Windows calls it.
core-warning-build-from-32-bit-windows = Diese App befand sich auf einem 32-Bit-Windows. Ihre Registrierungsschlüssel werden so angeordnet, wie ein 64-Bit-Windows die Schlüssel einer 32-Bit-App ablegt, und das Installationsprogramm schlägt Program Files (x86) dafür vor.
# { $dll } is a file's name, such as MSVCP140D.dll.
core-warning-build-debug-runtime = Ein Programm benötigt { $dll }, eine Debugversion der Visual C++-Laufzeitkomponente. Microsoft bietet dafür kein Installationsprogramm an und erlaubt die Weitergabe möglicherweise nicht, daher wird das Programm auf einem PC ohne Visual Studio nicht ausgeführt.
# { $program } is a program's file name, such as Tool.exe.
core-warning-build-single-file-program = { $program } ist ein .NET-Programm, das in eine einzige Datei gepackt ist und nicht angibt, welches .NET es benötigt, daher ist keines dafür vorgesehen. Falls es eines benötigt, startet es auf einem PC ohne dieses .NET nicht.
# { $count } is how many of the app's programs.
core-warning-build-programs-not-inspected =
    { $count ->
        [one] { $count } Programm der App wurde nicht auf benötigte Laufzeitkomponenten untersucht, da die App so viele oder so große Programme hat.
       *[other] { $count } Programme der App wurden nicht auf benötigte Laufzeitkomponenten untersucht, da die App so viele oder so große Programme hat.
    }
# { $runtime } is a runtime's name, such as Java Runtime Environment.
core-warning-build-runtime-no-installer = Die App benötigt { $runtime }, und hier ist kein Installationsprogramm dafür vorhanden, daher wird keines ausgeführt. Installieren Sie die Komponente auf dem anderen PC selbst.
# { $why } is why something was left out, such as "its name says what it holds".
core-warning-build-left-out-of-package = Nicht in das Paket aufgenommen: { $why }.
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-missing-dependency = Abhängigkeit konnte nicht aufgelöst werden: { $name }
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-dependency-only-here = { $name } wurde nur über den Suchpfad dieses PCs gefunden und gehört nicht zur App, daher wird die Datei nicht übernommen. Auf einem PC ohne diese Datei startet die App möglicherweise nicht.
core-warning-build-driver-detected = Treiber oder Kernelkomponente erkannt. Ein Treiber, der zur App gehört, wird gebündelt und neu registriert. Ein Treiber eines Herstellers oder von Windows wird nicht weitergegeben und muss mit dem signierten Paket des Herstellers neu installiert werden.
core-warning-build-near-no-longer-offered = Ein Element, das in der Liste möglicher weiterer Bestandteile dieser App aktiviert war, ist nicht mehr in dieser Liste und wurde daher nicht hinzugefügt. Sehen Sie sich die Liste erneut an.
# { $app_name } is the name of a part of the app, as the list of apps calls it. { $why } is never shown: it chooses the
# reason. Translate each reason as the end of the sentence.
core-warning-build-near-part-left-out =
    Ein Teil dessen, was { $app_name } als zugehörig angibt, wurde ausgelassen: { $why ->
        [unreadable] Sein Pfad verlässt das Laufwerk oder führt über einen Link, daher kann er nicht gefahrlos gelesen werden
        [missing] Er ist nicht vorhanden
        [link] Er führt über einen Link
        [short-name] Er ist nicht so geschrieben, wie er tatsächlich heißt
        [another-account] Er befindet sich in den Ordnern eines anderen Kontos
        [one-account] Diese App wurde für ein einzelnes Konto installiert, und dieser Teil gehört nicht zu diesem Konto
        [shared] Er enthält gemeinsam genutzte Inhalte oder gehört zu Windows
        [generic-name] Sein Name sagt nichts darüber aus, wem er gehört
        [publisher] Er ist nur nach dem Herausgeber benannt und kann dessen andere Produkte enthalten
        [holds-the-app] Er enthält den eigenen Ordner dieser App und kann mehr enthalten
        [another-apps-name] Er trägt den Namen einer anderen App
        [another-apps-record] Sein Installationseintrag gehört zu einer anderen App
        [in-another-apps-folder] Er befindet sich im Ordner einer anderen App
       *[another-apps] Er gehört zu einer anderen App
    }.
# { $app_name } is the name of a part of the app, as the list of apps calls it.
core-warning-build-near-keys-left-out = Die Registrierungsschlüssel, die im Installationseintrag von { $app_name } aufgeführt sind, wurden ausgelassen, da dieser Eintrag auch Elemente einer anderen App nennt.
# "Windows keeps what an app wrote for one account" is Windows' VirtualStore folder.
core-warning-build-virtual-store-program = Ein Programm befand sich in dem Ordner, in dem Windows ablegt, was eine App für ein einzelnes Konto geschrieben hat. Windows legt dort nie ein Programm ab, also hat etwas anderes es dort abgelegt, und es wurde ausgelassen.
# { $path } is a file or folder, such as C:\Users\Sam\AppData\Roaming\Acme.
core-warning-build-file-left-out = { $path } wurde ausgelassen: Der Pfad führt über einen Link auf einem angeschlossenen Laufwerk oder gehört zu einem anderen Konto als dem, dessen Daten aufgenommen werden.
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as D:\Sam\Documents.
core-warning-build-user-folder-not-on-drive = Der Ordner „{ $folder }“ von { $user } befand sich unter { $path }. Dieser Ort liegt nicht auf diesem Laufwerk, daher wurde der Inhalt nicht aufgenommen.
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as \\server\home\Sam\Documents.
core-warning-build-user-folder-not-on-this-pc = Der Ordner „{ $folder }“ von { $user } befand sich unter { $path }. Dieser Ort liegt nicht auf einem Datenträger dieses PCs, daher wurde der Inhalt nicht aufgenommen.
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-base-unreadable = { $path } konnte nicht gelesen werden ({ $error }). Dort gespeicherte Anwendungsdaten wurden nicht aufgenommen.
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-folder-unreadable = { $path } konnte nicht gelesen werden ({ $error }). Darin enthaltene Anwendungsdaten fehlen möglicherweise.
# { $error } is what went wrong.
core-warning-build-entry-unreadable = Ein Dateisystemeintrag konnte nicht gelesen werden: { $error }
# { $count } is how many links. { $names } lists some of them. { $more } is how many more there are, which are not
# named; when it is 0, the sentence ends after the list.
core-warning-build-links-not-followed =
    { $count ->
        [one] { $count } Link oder Name, der etwas anderes öffnen würde, wurde in diesem Ordner nicht verfolgt, daher ist sein Ziel nicht im Installationsprogramm enthalten: { $names }
       *[other] { $count } Links oder Namen, die etwas anderes öffnen würden, wurden in diesem Ordner nicht verfolgt, daher sind ihre Ziele nicht im Installationsprogramm enthalten: { $names }
    }{ $more ->
        [0] .
       *[other] { " " }und { $more } weitere.
    }
# { $count } is how many folders. { $depth } is how many levels down a folder can be. { $names } lists some of them.
# { $more } is how many more there are, which are not named; when it is 0, the sentence ends after the list.
core-warning-build-folders-too-deep =
    { $count ->
        [one] { $count } Ordner liegt mehr als { $depth } Ebenen tief, daher ist sein Inhalt nicht im Installationsprogramm enthalten: { $names }
       *[other] { $count } Ordner liegen mehr als { $depth } Ebenen tief, daher ist ihr Inhalt nicht im Installationsprogramm enthalten: { $names }
    }{ $more ->
        [0] .
       *[other] { " " }und { $more } weitere.
    }
# { $error } is what went wrong.
core-warning-build-metadata-unreadable = Dateimetadaten konnten nicht gelesen werden: { $error }
core-warning-build-reserved-name = Der Name dieser Datei ist für ein Windows-Gerät reserviert, daher wurde sie nicht kopiert.
# { $error } is what went wrong.
core-warning-build-hash-failed = Für die Datei konnte kein Hash berechnet werden: { $error }
core-warning-build-cloud-placeholder = Nur online verfügbare Clouddatei (z. B. OneDrive-Dateien bei Bedarf) ohne lokale Daten. Wählen Sie vor der Sicherung „Immer auf diesem Gerät beibehalten“, oder laden Sie die Datei herunter. Gemeldet, aber nicht kopiert.
# { $file_name } is a driver's file name, such as acmeflt.sys.
core-warning-build-driver-file = Die Treiber- oder Kerneldatei „{ $file_name }“ wird nicht als normale Datei kopiert. Wenn sie zu einem registrierten Treiberdienst dieser App gehört, bündelt das Installationsprogramm sie und registriert den Treiber automatisch neu. Andernfalls installieren Sie sie mit dem signierten Treiberpaket des Herstellers neu.
core-warning-build-file-gone = Dies war bei der Analyse der App vorhanden und ist jetzt nicht mehr vorhanden, daher ist es nicht im Installationsprogramm enthalten.
# { $error } is what went wrong.
core-warning-build-file-copy-failed = Die Datei konnte nicht in das Paket kopiert werden: { $error }
# { $count } is how many files. { $names } lists them.
core-warning-build-not-looked-through =
    { $count ->
        [one] Dieses Paket enthält eine Datei, die nicht auf Hinweise durchsucht werden konnte, von wem das Paket stammt: { $names }. Ihr Inhalt könnte darauf hinweisen.
       *[other] Dieses Paket enthält Dateien, die nicht auf Hinweise durchsucht werden konnten, von wem das Paket stammt: { $names }. Ihr Inhalt könnte darauf hinweisen.
    }
# { $count } is the most empty folders an installer makes, such as 10,000.
core-warning-build-empty-folders-truncated = Diese App hat mehr als { $count } leere Ordner, und nur so viele werden neu erstellt.
core-warning-build-credential-by-now = Nicht in das Paket aufgenommen: Es sieht inzwischen wie Anmeldeinformationen aus.
core-warning-build-no-package-path = Die kopierbare Datei hatte keinen relativen Pfad im Paket.
# { $file_name } is the runtime installer's file name, such as vc_redist.x64.exe. { $error } is what went wrong.
core-warning-build-prerequisite-path-invalid = Die gebündelte erforderliche Komponente { $file_name } konnte nicht abgelegt werden: { $error }
# { $runtime } is a runtime's name, such as Microsoft Visual C++ 2015+ Redistributable (x64). { $error } is what went
# wrong. winget is the name of a Windows command; keep it as it is.
core-warning-build-prerequisite-download-failed = { $runtime } konnte nicht heruntergeladen werden ({ $error }). Das Installationsprogramm versucht es auf dem Zielcomputer mit winget.
# { $runtime } is a runtime's name. { $reason } is why the check failed, such as "it is signed by Contoso Ltd, not
# Microsoft".
core-warning-build-prerequisite-signature-failed = Der Download von { $runtime } hat die Signaturprüfung nicht bestanden und wurde daher nicht gebündelt: { $reason }. Das Installationsprogramm versucht es auf dem Zielcomputer mit winget.
# { $runtime } is a runtime's name. { $error } is what went wrong.
core-warning-build-prerequisite-hash-failed = { $runtime } wurde heruntergeladen, aber es konnte kein Hash berechnet werden ({ $error }). Das Installationsprogramm versucht es stattdessen auf dem Zielcomputer mit winget.
# { $service } is a driver's service name, such as AcmeFlt. { $error } is what went wrong.
core-warning-build-driver-copy-failed = Die Treiberdatei für „{ $service }“ konnte nicht gebündelt werden: { $error }
core-warning-build-license-not-copied = Lizenzdaten konnten nicht kopiert werden (fehlen oder nicht lesbar). Nur als Metadaten erfasst.
core-warning-build-license-file-credential = Eine Lizenzdatei sieht wie Anmeldeinformationen aus und wurde ausgelassen.
# { $error } is what went wrong.
core-warning-build-license-store-walk-failed = Ein Teil eines Lizenzspeichers konnte nicht aufgelistet werden: { $error }
core-warning-build-license-store-credential = Eine Datei in einem Lizenzspeicher sieht wie Anmeldeinformationen aus und wurde ausgelassen.
# { $error } is what went wrong.
core-warning-build-license-store-file-skipped = Eine Datei eines Lizenzspeichers wurde übersprungen: { $error }
