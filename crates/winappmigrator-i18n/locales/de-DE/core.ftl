# Words the core writes for people: warnings, refusals, errors, and the installer's list of what it sets up.
# Message names start with core-.


## Warnings a package records when it is built, which an installer shows later, maybe on another PC, in that PC's
## language.

# { $count } is how many shortcuts.
core-warning-shortcuts-unreadable =
    { $count ->
        [one] { $count } Verknüpfung auf diesem Laufwerk nennt keine Datei, daher konnte nicht ermittelt werden, ob sie diese App öffnet.
       *[other] { $count } Verknüpfungen auf diesem Laufwerk nennen keine Datei, daher konnte nicht ermittelt werden, ob sie diese App öffnen.
    }

## Undoing an install: what an uninstall, or a failed install, says it put back, left, or could not undo. Each line is
## shown on its own, in a list.

# { $app_name } is the app's name.
core-rollback-record-damaged = Ein Teil der Aufzeichnung von { $app_name } konnte nicht gelesen werden, daher ist alles, was darin beschrieben war, noch vorhanden, und eine ersetzte Datei wird möglicherweise nicht wiederhergestellt.
core-rollback-settings-entry-stays = Der Eintrag der App in den Einstellungen bleibt erhalten, damit dies von dort aus erneut ausgeführt werden kann.
# { $name } is a runtime's name, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-rollback-prerequisite-stays = { $name } wurde installiert oder möglicherweise installiert und bleibt erhalten, da andere Programme die Komponente verwenden könnten.
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-drive-not-connected = Was diese Installation in { $folder } abgelegt hat, befindet sich auf einem nicht angeschlossenen Laufwerk. Schließen Sie es an, dann kann dies abgeschlossen werden.
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-record-part-unread = Was diese Installation in { $folder } abgelegt hat, bleibt erhalten, da dieser Teil der Aufzeichnung nicht gelesen werden konnte.

## Why a file or folder was left, shown after "was left:" in the messages below them. Each is part of a sentence, so
## starts with a small letter and has no full stop.

# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-being-changed = etwas hat { $folder } während dieses Vorgangs geändert, daher wurde darin nichts weiter unternommen. Schließen Sie, was das verursacht, und führen Sie dies erneut aus
# { $folder } is a folder; { $error } is what Windows said went wrong.
core-rollback-reason-not-opened = { $folder } konnte nicht geöffnet werden: { $error }
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-no-record = es ist nicht aufgezeichnet, wo sich { $folder } befand
# { $folder } is a folder; { $actual } is the folder it now leads to.
core-rollback-reason-moved = { $folder } führt jetzt zu { $actual } und nicht dorthin, wo der Ordner während der Installation war
# Why a service was not stopped.
core-rollback-reason-did-not-stop = er wurde nicht rechtzeitig beendet

## Files and folders

# { $path } is a folder; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-folder-left = Der Ordner { $path } blieb erhalten: { $reason }.
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-removed = Ordner entfernt: { $path }
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-holds-others = Der Ordner { $path } bleibt erhalten, da er Dateien enthält, die nicht von dieser Installation stammen.
# { $path } is a folder; { $error } is what Windows said went wrong.
core-rollback-folder-not-removed = Der Ordner { $path } konnte nicht entfernt werden: { $error }.
# { $path } is a file; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-file-left = Die Datei { $path } blieb erhalten: { $reason }.
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-folder-gone = { $path } wird nicht wiederhergestellt, da der Ordner, in dem sich die Datei befand, nicht mehr vorhanden ist.
# { $path } is a file; { $error } is what went wrong.
core-rollback-partial-not-removed = Eine unvollständige Datei konnte nicht entfernt werden: { $path } ({ $error }).
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-checked = { $path } konnte nicht geprüft werden: { $error }.
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-removed = Entfernt: { $path }
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-removed = { $path } konnte nicht entfernt werden: { $error }.
# { $path } is a file this install had replaced, and has put back.
core-rollback-file-put-back = Wiederhergestellt: { $path }
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-put-back = { $path } konnte nicht wiederhergestellt werden: { $error }.
# Why a file could not be put back.
core-rollback-backup-gone = die Sicherung ist nicht mehr vorhanden
# { $path } is a file.
core-rollback-no-parent-folder = { $path } hat keinen übergeordneten Ordner

## A file that stays. { $why } says why: still-used (something that stays still uses it), changed (it has changed since
## it was installed), or not-asked (it is someone's data, and they did not ask for it to go). { $path } is the file.

core-rollback-file-stays =
    { $why ->
        [still-used] { $path } bleibt erhalten, da etwas, das ebenfalls erhalten bleibt, die Datei noch verwendet.
        [changed] { $path } bleibt erhalten, da die Datei seit der Installation geändert wurde.
       *[not-asked] { $path } bleibt erhalten, da ihre Entfernung nicht angefordert wurde.
    }
# The file it replaced was to be put beside it, under another name, and every name was taken.
core-rollback-file-stays-no-name-beside =
    { $why ->
        [still-used] { $path } bleibt erhalten, da etwas, das ebenfalls erhalten bleibt, die Datei noch verwendet, und die von ihr ersetzte Datei konnte nicht daneben abgelegt werden, da alle möglichen Namen dafür vergeben sind.
        [changed] { $path } bleibt erhalten, da die Datei seit der Installation geändert wurde, und die von ihr ersetzte Datei konnte nicht daneben abgelegt werden, da alle möglichen Namen dafür vergeben sind.
       *[not-asked] { $path } bleibt erhalten, da ihre Entfernung nicht angefordert wurde, und die von ihr ersetzte Datei konnte nicht daneben abgelegt werden, da alle möglichen Namen dafür vergeben sind.
    }
# { $name } is the name the file it replaced now has, such as settings.ini.before-install.
core-rollback-file-stays-beside =
    { $why ->
        [still-used] { $path } bleibt erhalten, da etwas, das ebenfalls erhalten bleibt, die Datei noch verwendet. Die von ihr ersetzte Datei liegt daneben als { $name }.
        [changed] { $path } bleibt erhalten, da die Datei seit der Installation geändert wurde. Die von ihr ersetzte Datei liegt daneben als { $name }.
       *[not-asked] { $path } bleibt erhalten, da ihre Entfernung nicht angefordert wurde. Die von ihr ersetzte Datei liegt daneben als { $name }.
    }
# { $error } is what went wrong.
core-rollback-file-stays-not-beside =
    { $why ->
        [still-used] { $path } bleibt erhalten, da etwas, das ebenfalls erhalten bleibt, die Datei noch verwendet, und die von ihr ersetzte Datei konnte nicht daneben abgelegt werden: { $error }.
        [changed] { $path } bleibt erhalten, da die Datei seit der Installation geändert wurde, und die von ihr ersetzte Datei konnte nicht daneben abgelegt werden: { $error }.
       *[not-asked] { $path } bleibt erhalten, da ihre Entfernung nicht angefordert wurde, und die von ihr ersetzte Datei konnte nicht daneben abgelegt werden: { $error }.
    }

## The registry and environment variables. { $kind } says what { $name } is: key (a registry key, such as
## HKLM\Software\App), value (a registry value, such as HKLM\Software\App\Version), user-variable or machine-variable
## (an environment variable of the user's or of the whole PC, such as APP_HOME), or user-list or machine-list (a
## variable that is a list of folders, such as Path). { $error } is what Windows said went wrong.

# How a registry key's unnamed value is shown, as Registry Editor shows it.
core-registry-default-value-name = (Standard)
core-rollback-setting-other-account =
    { $kind ->
        [key] Der Schlüssel { $name } bleibt erhalten, da er zum Konto gehört, unter dem dies installiert wurde, und dieses Konto nicht angemeldet ist.
        [user-variable] Die Benutzerumgebungsvariable { $name } bleibt erhalten, da sie zum Konto gehört, unter dem dies installiert wurde, und dieses Konto nicht angemeldet ist.
        [machine-variable] Die Systemumgebungsvariable { $name } bleibt erhalten, da sie zum Konto gehört, unter dem dies installiert wurde, und dieses Konto nicht angemeldet ist.
        [user-list] Die Benutzervariable { $name } bleibt erhalten, da sie zum Konto gehört, unter dem dies installiert wurde, und dieses Konto nicht angemeldet ist.
        [machine-list] Die Systemvariable { $name } bleibt erhalten, da sie zum Konto gehört, unter dem dies installiert wurde, und dieses Konto nicht angemeldet ist.
       *[value] Der Registrierungswert { $name } bleibt erhalten, da er zum Konto gehört, unter dem dies installiert wurde, und dieses Konto nicht angemeldet ist.
    }
core-rollback-setting-not-reached =
    { $kind ->
        [key] Auf den Schlüssel { $name } konnte nicht zugegriffen werden: { $error }.
        [user-variable] Auf die Benutzerumgebungsvariable { $name } konnte nicht zugegriffen werden: { $error }.
        [machine-variable] Auf die Systemumgebungsvariable { $name } konnte nicht zugegriffen werden: { $error }.
        [user-list] Auf die Benutzervariable { $name } konnte nicht zugegriffen werden: { $error }.
        [machine-list] Auf die Systemvariable { $name } konnte nicht zugegriffen werden: { $error }.
       *[value] Auf den Registrierungswert { $name } konnte nicht zugegriffen werden: { $error }.
    }
core-rollback-setting-not-read =
    { $kind ->
        [key] Der Schlüssel { $name } konnte nicht gelesen werden: { $error }.
        [user-variable] Die Benutzerumgebungsvariable { $name } konnte nicht gelesen werden: { $error }.
        [machine-variable] Die Systemumgebungsvariable { $name } konnte nicht gelesen werden: { $error }.
        [user-list] Die Benutzervariable { $name } konnte nicht gelesen werden: { $error }.
        [machine-list] Die Systemvariable { $name } konnte nicht gelesen werden: { $error }.
       *[value] Der Registrierungswert { $name } konnte nicht gelesen werden: { $error }.
    }
core-rollback-setting-changed =
    { $kind ->
        [user-variable] Die Benutzerumgebungsvariable { $name } bleibt erhalten, da sie seit der Installation geändert wurde.
        [machine-variable] Die Systemumgebungsvariable { $name } bleibt erhalten, da sie seit der Installation geändert wurde.
        [user-list] Die Benutzervariable { $name } bleibt erhalten, da sie seit der Installation geändert wurde.
        [machine-list] Die Systemvariable { $name } bleibt erhalten, da sie seit der Installation geändert wurde.
       *[value] Der Registrierungswert { $name } bleibt erhalten, da er seit der Installation geändert wurde.
    }
# Said when it is back as it was before the install.
core-rollback-setting-put-back =
    { $kind ->
        [user-variable] Wiederhergestellt: Benutzerumgebungsvariable { $name }
        [machine-variable] Wiederhergestellt: Systemumgebungsvariable { $name }
        [user-list] Wiederhergestellt: Benutzervariable { $name }
        [machine-list] Wiederhergestellt: Systemvariable { $name }
       *[value] Wiederhergestellt: Registrierungswert { $name }
    }
core-rollback-setting-not-put-back =
    { $kind ->
        [user-variable] Die Benutzerumgebungsvariable { $name } konnte nicht wiederhergestellt werden: { $error }.
        [machine-variable] Die Systemumgebungsvariable { $name } konnte nicht wiederhergestellt werden: { $error }.
        [user-list] Die Benutzervariable { $name } konnte nicht wiederhergestellt werden: { $error }.
        [machine-list] Die Systemvariable { $name } konnte nicht wiederhergestellt werden: { $error }.
       *[value] Der Registrierungswert { $name } konnte nicht wiederhergestellt werden: { $error }.
    }
# A registry value the install wrote, which is the user's data, and stays unless they asked for it to go.
core-rollback-value-kept-by-request = Der Registrierungswert { $name } bleibt erhalten, da seine Entfernung nicht angefordert wurde.
core-rollback-key-holds-others = Der Schlüssel { $name } bleibt erhalten, da er Elemente enthält, die nicht von dieser Installation stammen.
core-rollback-key-removed = Schlüssel entfernt: { $name }
core-rollback-key-not-removed = Der Schlüssel { $name } konnte nicht entfernt werden: { $error }.
# { $kind } is user-list or machine-list.
core-rollback-list-entries-removed =
    { $kind ->
        [user-list] Einträge dieser Installation aus der Benutzervariablen { $name } entfernt
       *[machine-list] Einträge dieser Installation aus der Systemvariablen { $name } entfernt
    }
core-rollback-list-entries-not-removed =
    { $kind ->
        [user-list] Die Einträge dieser Installation konnten nicht aus der Benutzervariablen { $name } entfernt werden: { $error }.
       *[machine-list] Die Einträge dieser Installation konnten nicht aus der Systemvariablen { $name } entfernt werden: { $error }.
    }

## Services and drivers. { $kind } is service or driver; { $name } is its name, such as AppUpdater. { $error } and
## { $reason } are what Windows said went wrong.

core-rollback-service-not-read =
    { $kind ->
        [driver] Der Treiber { $name } konnte nicht gelesen werden: { $error }.
       *[service] Der Dienst { $name } konnte nicht gelesen werden: { $error }.
    }
core-rollback-service-changed =
    { $kind ->
        [driver] Der Treiber { $name } bleibt erhalten, da er seit der Installation geändert wurde.
       *[service] Der Dienst { $name } bleibt erhalten, da er seit der Installation geändert wurde.
    }
core-rollback-service-not-stopped =
    { $kind ->
        [driver] Der Treiber { $name } konnte nicht beendet werden ({ $reason }), daher bleibt, was er verwendet, möglicherweise bis zum Neustart des PCs bestehen.
       *[service] Der Dienst { $name } konnte nicht beendet werden ({ $reason }), daher bleibt, was er verwendet, möglicherweise bis zum Neustart des PCs bestehen.
    }
core-rollback-service-stopped-for-this =
    { $kind ->
        [driver] Der Treiber { $name } wurde dafür beendet. Starten Sie ihn erneut, wenn Sie ihn benötigen.
       *[service] Der Dienst { $name } wurde dafür beendet. Starten Sie ihn erneut, wenn Sie ihn benötigen.
    }
core-rollback-service-not-removed =
    { $kind ->
        [driver] Der Treiber { $name } konnte nicht entfernt werden: { $reason }.
       *[service] Der Dienst { $name } konnte nicht entfernt werden: { $reason }.
    }
core-rollback-service-marked-for-removal =
    { $kind ->
        [driver] Der Treiber { $name } ist zum Entfernen markiert und wird entfernt, wenn der PC neu gestartet wird oder sobald ihn nichts mehr geöffnet hat.
       *[service] Der Dienst { $name } ist zum Entfernen markiert und wird entfernt, wenn der PC neu gestartet wird oder sobald ihn nichts mehr geöffnet hat.
    }
core-rollback-service-removed =
    { $kind ->
        [driver] Treiber entfernt: { $name }
       *[service] Dienst entfernt: { $name }
    }
core-rollback-service-not-restored =
    { $kind ->
        [driver] Der Treiber { $name } konnte nicht in den vorherigen Zustand zurückversetzt werden: { $reason }.
       *[service] Der Dienst { $name } konnte nicht in den vorherigen Zustand zurückversetzt werden: { $reason }.
    }
core-rollback-service-restored =
    { $kind ->
        [driver] Einstellungen des Treibers wiederhergestellt: { $name }
       *[service] Einstellungen des Dienstes wiederhergestellt: { $name }
    }
core-rollback-service-not-restarted = Der Dienst { $name } lief vor dieser Installation und wurde nicht erneut gestartet: { $reason }.
# { $account } is the account the service ran as, such as CONTOSO\backup.
core-rollback-service-account-again = Der Dienst { $name } wurde als { $account } ausgeführt. Sein Kennwort wird nicht gespeichert, legen Sie das Konto daher unter „Dienste“ erneut fest.

## Firewall rules. { $name } is the rule's name; { $error } is what went wrong.

core-rollback-firewall-not-read = Die Firewallregel { $name } konnte nicht gelesen werden: { $error }.
core-rollback-firewall-says-otherwise = Die Firewallregel { $name } bleibt erhalten, da die vorhandene Regel mit diesem Namen etwas anderes festlegt.
core-rollback-firewall-changed = Die Firewallregel { $name } bleibt erhalten, da sie seit der Installation geändert wurde.
# The capital "The" is as the app has always said it.
core-rollback-firewall-removed = Firewallregel entfernt: { $name }
core-rollback-firewall-not-removed = Die Firewallregel { $name } konnte nicht entfernt werden: { $error }.

## Scheduled tasks. { $path } is a folder of tasks, and { $name } a task with its folder, such as \App\Nightly Update.
## { $error } and { $reason } are what went wrong.

core-rollback-task-folder-holds-others = Der Aufgabenordner { $path } bleibt erhalten, da er Aufgaben enthält, die nicht von dieser Installation stammen.
core-rollback-task-folder-not-removed = Der Aufgabenordner { $path } konnte nicht entfernt werden: { $error }.
core-rollback-task-not-read = Die Aufgabe { $name } konnte nicht gelesen werden: { $error }.
core-rollback-task-changed = Die Aufgabe { $name } bleibt erhalten, da sie seit der Installation geändert wurde.
core-rollback-task-removed = Aufgabe entfernt: { $name }
core-rollback-task-not-removed = Die Aufgabe { $name } konnte nicht entfernt werden: { $reason }.
core-rollback-task-put-back = Aufgabe wiederhergestellt: { $name }
core-rollback-task-not-put-back = Die Aufgabe { $name } konnte nicht wiederhergestellt werden: { $reason }.

## When an install cannot be undone at all

# { $error } is what went wrong.
core-rollback-record-unreadable = Die Aufzeichnung dieser Installation konnte nicht gelesen werden, daher wurde nichts rückgängig gemacht: { $error }.
# { $error } is what went wrong.
core-rollback-nothing-undone = Es wurde nichts rückgängig gemacht: { $error }.
# One app's part of the message below. { $app_name } is the app's name; { $failures } is what could not be undone, a
# sentence or more; { $folder } is the folder that holds the install's record.
core-rollback-earlier-install-stuck = { $app_name }: { $failures } Falls sich dies nie rückgängig machen lässt, kann ein Administrator die Installation aufgeben, indem er { $folder } entfernt.
# Why an install did not start. { $details } is one or more of the message above.
core-rollback-earlier-installs-not-undone = eine frühere, nie abgeschlossene Installation konnte nicht vollständig rückgängig gemacht werden, daher wurde nichts installiert. Schließen Sie alles, was diese Elemente verwendet, und versuchen Sie es erneut. { $details }
core-rollback-install-busy = diese Installation wird gerade von einem anderen Installations- oder Deinstallationsprogramm geändert

## The summary an uninstall writes on the command line, one line each, with what each says below it. { $count } is how
## many.

core-rollback-count-undone = Rückgängig gemacht: { $count }
core-rollback-count-failed = Nicht rückgängig zu machen: { $count }
# What was left on purpose: it has changed since the install, or it is the user's.
core-rollback-count-kept = Beibehalten: { $count }
# What no uninstall can take back, such as a runtime that was installed, which other programs may use.
core-rollback-count-left-behind = Zurückgelassen: { $count }

## Which Windows a PC or drive has, as the app shows it beside the drive.

# { $name } is the Windows' name, such as Windows 7 Professional Service Pack 1.
core-system-windows-32-bit = { $name }, 32-Bit
# { $name } is the Windows' name, such as Windows 11 Pro.
core-system-windows-64-bit = { $name }, 64-Bit
# A drive's Windows, when it only says it is older than Vista.
core-system-windows-xp-or-older = Windows XP oder älter

## Reading a connected drive, or another account's settings. Each is why it could not be done, and some start with a
## small letter, as they follow other words.

core-system-needs-administrator = zum Lesen eines angeschlossenen Laufwerks oder der Einstellungen eines anderen Kontos sind Administratorrechte erforderlich
core-system-no-program-data = der ProgramData-Ordner dieses PCs wurde nicht gefunden
# { $path } is a file, such as D:\Windows\System32\config\SOFTWARE.
core-system-not-a-hive-file = keine Registrierungsstrukturdatei: { $path }
# { $path } is a file; { $size } is its size in bytes.
core-system-hive-too-large = { $path } ist { $size } Bytes groß und damit zu groß für eine Registrierungsstruktur oder größer, als die Strukturen dieses Laufwerks insgesamt sein können
# { $key } is a registry key, such as Software\Microsoft\Windows\CurrentVersion.
core-system-hive-no-mapping = die Offlinestruktur hat keine Zuordnung für HKLM\{ $key }
# { $key } is where a drive's registry was to be loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said.
# "reg load" is a command's name.
core-system-hive-not-loaded = reg load { $key } fehlgeschlagen: { $error }
# { $key } is where a drive's registry was loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said. "reg
# unload" is a command's name.
core-system-hive-not-unloaded = reg unload { $key } fehlgeschlagen: { $error }
# { $drive } is a drive, such as D:\.
core-system-not-a-system-drive = { $drive } ist kein Windows-Systemlaufwerk (kein Ordner \Windows).
# { $drive } is a drive, such as C:\. "This PC" is the name of a choice in the app.
core-system-own-system-drive = { $drive } ist das eigene Systemlaufwerk dieses PCs. Wählen Sie „Dieser PC“, um diesen PC zu lesen.
# { $drive } is a drive, such as D:\.
core-system-registry-through-link = { $drive } ist kein Windows-Systemlaufwerk: Der Weg zu seiner Registrierung führt über einen Link.

## Warnings a scan records about whose data it read.

# { $account } is an account's name or id, such as alice or S-1-5-21-...-1001.
core-warning-scan-account-not-on-record = Das Konto { $account } ist hier nicht verzeichnet, daher wurden keine benutzerspezifischen Daten oder Einstellungen aufgenommen.
# { $account } is an account's name, such as CONTOSO\alice.
core-warning-scan-signed-in-account-no-profile = Das hier angemeldete Konto { $account } hat kein verzeichnetes Profil, daher wurden keine benutzerspezifischen Daten oder Einstellungen aufgenommen.
# { $reason } is why Windows could not say.
core-warning-scan-signed-in-account-unknown = Es konnte nicht ermittelt werden, wer angemeldet ist ({ $reason }), daher wurden keine benutzerspezifischen Daten oder Einstellungen aufgenommen. Wählen Sie aus, wessen Daten aufgenommen werden sollen.
core-warning-scan-last-account-unknown = Dieses Laufwerk gibt nicht an, wer zuletzt angemeldet war, und es hat mehr als ein Konto, daher wurden keine benutzerspezifischen Daten oder Einstellungen aufgenommen. Wählen Sie aus, wessen Daten aufgenommen werden sollen.
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-settings-unreadable = Die Einstellungen von { $account } konnten nicht gelesen werden und wurden daher nicht aufgenommen: { $error }
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-classes-unreadable = Die für { $account } registrierten COM-Klassen und Dateitypen konnten nicht gelesen werden und wurden daher nicht aufgenommen: { $error }

## Paths a scan did not follow. { $count } is how many; { $paths } is some of them, separated by semicolons; { $more } is
## how many more there are, not shown.

core-warning-scan-drive-paths-not-followed =
    { $count ->
        [one] Ein auf diesem Laufwerk verzeichneter Pfad konnte nicht gelesen werden, daher wurde von dort nichts aufgenommen. Der Pfad liegt auf einem anderen Datenträger, ist kein vollständiger Pfad oder führt über einen Link, der vom Laufwerk wegführen würde: { $paths }
       *[other] Einige auf diesem Laufwerk verzeichnete Pfade konnten nicht gelesen werden, daher wurde von dort nichts aufgenommen. Jeder davon liegt auf einem anderen Datenträger, ist kein vollständiger Pfad oder führt über einen Link, der vom Laufwerk wegführen würde: { $paths }
    }
core-warning-scan-drive-paths-not-followed-more = Einige auf diesem Laufwerk verzeichnete Pfade konnten nicht gelesen werden, daher wurde von dort nichts aufgenommen. Jeder davon liegt auf einem anderen Datenträger, ist kein vollständiger Pfad oder führt über einen Link, der vom Laufwerk wegführen würde: { $paths } und { $more } weitere
core-warning-scan-network-paths-not-followed =
    { $count ->
        [one] Ein auf diesem PC verzeichneter Pfad konnte nicht gelesen werden, daher wurde von dort nichts aufgenommen. Der Pfad liegt nicht auf einem Datenträger dieses PCs, sodass beim Zugriff eine Verbindung hergestellt würde, oder er hat einen Namen, der etwas anderes öffnen würde: { $paths }
       *[other] Einige auf diesem PC verzeichnete Pfade konnten nicht gelesen werden, daher wurde von dort nichts aufgenommen. Jeder davon liegt nicht auf einem Datenträger dieses PCs, sodass beim Zugriff eine Verbindung hergestellt würde, oder hat einen Namen, der etwas anderes öffnen würde: { $paths }
    }
core-warning-scan-network-paths-not-followed-more = Einige auf diesem PC verzeichnete Pfade konnten nicht gelesen werden, daher wurde von dort nichts aufgenommen. Jeder davon liegt nicht auf einem Datenträger dieses PCs, sodass beim Zugriff eine Verbindung hergestellt würde, oder hat einen Namen, der etwas anderes öffnen würde: { $paths } und { $more } weitere

## What a scan with none of an account's data leaves out of a package, in the app's list of what was left out. Each
## has a name, and below it why.

core-account-left-out-shortcuts = Verknüpfungen in den eigenen Ordnern dieses Kontos
# { $count } is how many shortcuts.
core-account-left-out-shortcuts-why =
    { $count ->
        [one] { $count } Verknüpfung, die nur mit der Option „Benutzerdaten“ aufgenommen wird
       *[other] { $count } Verknüpfungen, die nur mit der Option „Benutzerdaten“ aufgenommen werden
    }
core-account-left-out-variables = Umgebungsvariablen dieses Kontos
# { $count } is how many variables.
core-account-left-out-variables-why =
    { $count ->
        [one] { $count } Variable, die nur mit der Option „Benutzerdaten“ aufgenommen wird
       *[other] { $count } Variablen, die nur mit der Option „Benutzerdaten“ aufgenommen werden
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-account-left-out-task = Die geplante Aufgabe { $task }
core-account-task-works-on-account-folder = Die Aufgabe arbeitet mit einem Ort in den Ordnern eines Kontos oder ist nach einem solchen benannt und wird nur mit der Option „Benutzerdaten“ aufgenommen
core-account-task-runs-from-account-folder = Was die Aufgabe zusätzlich aus den Ordnern eines Kontos ausführt, wird nur mit der Option „Benutzerdaten“ aufgenommen
# { $service } is a service's name, such as AppUpdater.
core-account-left-out-service = Der Dienst { $service }
core-account-service-works-on-account-folder = Der Dienst arbeitet mit einem Ort in den Ordnern eines Kontos und wird nur mit der Option „Benutzerdaten“ aufgenommen

## What a package was found to still say of whose PC it came from, after a build with none of an account's data.

# The parts of a package, as the warning below names them.
core-account-part-files = die Pfade der eigenen Dateien der App
core-account-part-registry = Registrierungswerte
core-account-part-settings-entry = der Eintrag der App in den Einstellungen
core-account-part-main-program = der Speicherort des Hauptprogramms
core-account-part-shortcuts = Verknüpfungen
core-account-part-tasks = geplante Aufgaben
core-account-part-services = Dienste
core-account-part-variables = Umgebungsvariablen
# One place in the list below. { $place } is one of the parts above, or a file's name; { $count } is how many times.
core-account-mention-place = { $place } ({ $count })
# { $kind } is what was found: account (an account's name), folder (where an account's folder is), computer (the PC's
# name), or id (an account's id). { $places } is a list of where, from the message above.
core-warning-scan-package-mentions =
    { $kind ->
        [account] Dieses Paket erwähnt an folgenden Stellen den Namen eines Kontos: { $places }. Es wurde ohne die Daten eines Kontos erstellt, und jeder, der das Installationsprogramm hat, kann dies lesen.
        [folder] Dieses Paket erwähnt an folgenden Stellen, wo sich der Ordner eines Kontos befindet: { $places }. Es wurde ohne die Daten eines Kontos erstellt, und jeder, der das Installationsprogramm hat, kann dies lesen.
        [computer] Dieses Paket erwähnt an folgenden Stellen den Namen des PCs: { $places }. Es wurde ohne die Daten eines Kontos erstellt, und jeder, der das Installationsprogramm hat, kann dies lesen.
       *[id] Dieses Paket erwähnt an folgenden Stellen die ID eines Kontos: { $places }. Es wurde ohne die Daten eines Kontos erstellt, und jeder, der das Installationsprogramm hat, kann dies lesen.
    }
# { $count } is how many folders.
core-warning-scan-folders-of-others-unreadable =
    { $count ->
        [one] { $count } Ordner außerhalb der App konnte nicht gelesen werden, daher fehlen dort möglicherweise Daten der App.
       *[other] { $count } Ordner außerhalb der App konnten nicht gelesen werden, daher fehlen dort möglicherweise Daten der App.
    }

## What a scan leaves out because it looks like a credential, or is a license with the License option off, in the app's
## list of what was left out. Each has a name, and below it why.

core-secret-why-credential = es sieht wie Anmeldeinformationen aus
core-secret-why-license-option-off = die Option „Lizenz- und Aktivierungsdaten“ ist deaktiviert
# { $count } is how many values of a registry key.
core-secret-why-account-values =
    { $count ->
        [one] { $count } Wert, der nur mit der Option „Benutzerdaten“ aufgenommen wird
       *[other] { $count } Werte, die nur mit der Option „Benutzerdaten“ aufgenommen werden
    }
# Shown in place of a registry value's name when its name itself looks like a password or a key.
core-secret-name-holds-credential = (ein Wert, dessen Name Anmeldeinformationen enthält)
# Shown in place of an option's name, for an argument on a command line that has no name.
core-secret-unnamed-argument = ein Argument mit Anmeldeinformationen
# { $option } is an option taken out of a command line, such as /password, or the message above. { $place } says
# where the command line was: service, task, shortcut, opener (what opens a file type), opener-unnamed (the same, of a
# type with no name), or uninstall. { $name } is the service, task, shortcut, file type, or app.
core-secret-taken-from =
    { $place ->
        [service] { $option } in der Befehlszeile des Dienstes { $name }
        [task] { $option } in der Befehlszeile der Aufgabe { $name }
        [shortcut] { $option } in der Verknüpfung { $name }
        [opener] { $option } in dem, was { $name } öffnet
        [opener-unnamed] { $option } in dem, was einen Dateityp öffnet
       *[uninstall] { $option } im Deinstallationsbefehl von { $name }
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-secret-task-definition-unreadable = Die Definition der Aufgabe { $task }, die zur Prüfung nicht gelesen werden konnte
# { $name } is an environment variable's name, such as APP_TOKEN.
core-secret-left-out-variable = Die Umgebungsvariable { $name }
# { $class } is a COM class's id, such as {00000000-0000-0000-0000-000000000000}.
core-secret-left-out-class-server = Was die Klasse { $class } nach eigener Angabe ausführt
# { $rule } is a firewall rule's name.
core-secret-left-out-firewall-description = Die Beschreibung der Firewallregel { $rule }

## Licenses a scan found, as the package's report describes each.

# { $name } is a file's name, such as license.lic.
core-license-detail-file = Lizenzdatei erkannt: { $name }
# { $framework } is a licensing product's name, such as FlexLM / FlexNet.
core-license-detail-store = Lizenzspeicher von { $framework } vorhanden
# { $name } is an environment variable's name, such as LM_LICENSE_FILE.
core-license-detail-variable = Lizenzumgebungsvariable { $name }
# { $name } is a registry value's name, such as SerialNumber.
core-license-detail-registry-value = Registrierungswert deutet auf Lizenzdaten hin: { $name }
# Said of a registry value in an account's key, which is not named.
core-license-detail-registry-value-of-an-account = Registrierungswert deutet auf Lizenzdaten hin
# { $key } is a registry key, such as HKLM\Software\Contoso\License.
core-license-detail-registry-key = Registrierungsschlüssel deutet auf Lizenzdaten hin: { $key }

## Warnings a scan records about licenses.

core-warning-scan-license-data = Lizenz- bzw. Aktivierungsdaten wurden erkannt. Mit aktivierter Option „Lizenz- und Aktivierungsdaten“ stellt das Installationsprogramm wieder her, was übertragen werden kann. Computerweite Werte und hardwaregebundene Speicher werden nur gemeldet. WinAppMigrator entschlüsselt nichts, umgeht keine Aktivierung und emuliert keine Dongles.
core-warning-scan-license-machine-bound = Einige Lizenzen sind an den Computer gebunden (Hardware-Dongle, computergebundene Aktivierung oder Verschlüsselung). Das Installationsprogramm stellt wieder her, was möglich ist. Diese Lizenzen müssen auf dem neuen PC möglicherweise trotzdem erneut aktiviert werden.

## Warnings a scan records about the registry. What could not be read is not named in them: it is kept beside them.

# { $error } is what Windows said went wrong. App Paths is a registry key's name.
core-warning-scan-app-paths-key-unreadable = Ein Unterschlüssel von App Paths konnte nicht gelesen werden: { $error }
core-warning-scan-registry-limit = Die Erfassung der Registrierung hat das konfigurierte Sicherheitslimit erreicht.
core-warning-scan-registry-subtree-limit = Die Erfassung einer Registrierungsteilstruktur hat das konfigurierte Sicherheitslimit erreicht.
# { $error } is what Windows said went wrong.
core-warning-scan-registry-key-unreadable = Ein Registrierungsschlüssel konnte nicht gelesen werden: { $error }
core-warning-scan-registry-value-too-large = Ein Registrierungswert hat die konfigurierte Größengrenze für die Erfassung überschritten.

## Scheduled tasks

core-warning-scan-tasks-folder-through-link = Der Ordner der geplanten Aufgaben auf diesem Laufwerk führt über einen Link, daher wurden seine Aufgaben nicht gelesen.
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-warning-scan-task-left-out = Die geplante Aufgabe { $task } wurde ausgelassen, da sie unter einem anderen Konto ausgeführt wird als dem, dessen Einstellungen aufgenommen werden, da andere als Administratoren ihre Datei ändern können oder da die App zu viele Aufgaben hat, um alle zu übernehmen.
# { $error } is what Windows said went wrong.
core-warning-scan-tasks-folder-unreadable = Ein Teil der geplanten Aufgaben konnte nicht gelesen werden: { $error }
# { $path } is a task's file, such as C:\Windows\System32\Tasks\App\Nightly Sync; { $reason } is one of the reasons
# below, or what Windows said went wrong.
core-warning-scan-task-unreadable = Die geplante Aufgabe { $path } konnte nicht gelesen werden: { $reason }. Falls sie die App ausführt, wurde sie nicht aufgenommen.
# Why a task's file could not be read. { $size } is its size in bytes.
core-task-too-large = die Datei ist { $size } Bytes groß und damit zu groß für eine Aufgabe
# Why a task's file could not be read.
core-task-ends-mid-character = die Datei endet mitten in einem Zeichen
# Why a task's file could not be read. { $error } says where.
core-task-not-utf16 = die Datei ist kein gültiges UTF-16: { $error }
# Why a task's file could not be read. { $error } says where.
core-task-not-text = die Datei ist weder gültiges UTF-8 noch gültiges UTF-16: { $error }
# What a package records of how a task is put back. Register-ScheduledTask is a command's name.
core-task-restore-instructions = Die Wiederherstellung verwendet Register-ScheduledTask mit der archivierten XML-Definition.

## Reading a program's file. Each is why it could not be read, after the program's name.

# { $reason } is what went wrong.
core-pe-unreadable = die Datei konnte nicht gelesen werden: { $reason }
core-pe-headers-misaligned = ihre Header liegen nicht an den Grenzen, die für ein Programm vorgeschrieben sind
# { $size } is the file's size in bytes.
core-pe-too-large = die Datei ist { $size } Bytes groß und damit zu groß für ein Programm
# PE is the name of the format Windows programs are in.
core-pe-no-headers = die Datei hat keine PE-Header

## Warnings a scan records about what a program needs. Which program is kept beside each.

# { $error } is what went wrong.
core-warning-scan-imports-unreadable = Die Importtabelle konnte nicht gelesen werden: { $error }
# { $error } is what went wrong.
core-warning-scan-import-name-unreadable = Der Name eines Importdeskriptors konnte nicht gelesen werden: { $error }
# { $count } is how many were skipped.
core-warning-scan-legacy-delay-imports =
    { $count ->
        [one] { $count } veralteter (v1) Delay-Load-Deskriptor wurde übersprungen. Nur moderne RVA-basierte verzögerte Importe werden analysiert.
       *[other] { $count } veraltete (v1) Delay-Load-Deskriptoren wurden übersprungen. Nur moderne RVA-basierte verzögerte Importe werden analysiert.
    }

## The DLLs a program needs. Which DLL or program is kept beside each warning.

# { $path } is a program, such as C:\Program Files\App\app.exe.
core-dll-program-has-no-folder = die ausführbare Datei hat keinen übergeordneten Ordner: { $path }
# Why a DLL is among the app's files, in the package's list of them.
core-dll-beside-program = DLL neben der ausgewählten ausführbaren Datei gefunden
# { $error } is what went wrong.
core-warning-scan-dependency-unscanned = Eine rekursive Abhängigkeit konnte nicht analysiert werden: { $error }
# { $name } is a DLL's name, such as d3dx9_43.dll.
core-warning-scan-dependency-not-in-newer-windows = { $name } befand sich im Windows-Ordner des PCs, auf dem sich diese App befand, und ist in Windows 10 und 11 nicht enthalten. Etwas anderes hat die Datei dort installiert, daher gehört sie nicht zu den eigenen Dateien der App und wird nicht übernommen. Die App startet möglicherweise erst, wenn installiert ist, was die Datei bereitstellt.
# { $error } is what went wrong.
core-warning-scan-dependency-not-hashed = Für eine Abhängigkeit konnte kein Hash berechnet werden: { $error }
# SystemRoot and windir are the names of environment variables.
core-warning-scan-windows-folder-unknown = Die Umgebungsvariablen SystemRoot/windir sind nicht verfügbar. Die Suche nach System-DLLs ist eingeschränkt.
# { $error } is what went wrong.
core-warning-scan-dependency-architecture-unreadable = Die Architektur einer Abhängigkeit konnte nicht gelesen werden: { $error }

## Services, drivers, shortcuts, and environment variables a scan reads. Where it was reading is kept beside each
## warning.

# { $error } is what Windows said went wrong.
core-warning-scan-services-unreadable = Der Registrierungsschlüssel der Dienste konnte nicht gelesen werden: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-drivers-unreadable = Der Registrierungsschlüssel der Dienste konnte für die Treibererkennung nicht gelesen werden: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-queried = Verknüpfungen konnten nicht abgefragt werden: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-parsed = Die Ausgabe der Verknüpfungsabfrage konnte nicht analysiert werden: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-environment-unreadable = Der Registrierungsschlüssel der Systemumgebungsvariablen konnte nicht gelesen werden: { $error }

## What a package records of how a driver is put back, which an installer shows. { $kind } is the driver's type as
## Windows names it (kernel, file-system, recognizer, or unknown-type); { $name } is its service's name; { $path } is
## its file. { $signed } is yes when { $signer } names who signed it, and no when that is not known.

core-service-driver-bundled =
    { $signed ->
        [yes] App-eigener Treiber „{ $name }“ vom Typ { $kind } ({ $path }, signiert von { $signer }). Das Installationsprogramm bündelt seine .sys-Datei und registriert den Treiberdienst automatisch neu (sc create, erfordert erhöhte Rechte). Anschließend wird bestätigt, dass der Dienst registriert ist.
       *[no] App-eigener Treiber „{ $name }“ vom Typ { $kind } ({ $path }, Signatur unbekannt). Das Installationsprogramm bündelt seine .sys-Datei und registriert den Treiberdienst automatisch neu (sc create, erfordert erhöhte Rechte). Anschließend wird bestätigt, dass der Dienst registriert ist.
    }
core-service-driver-from-vendor =
    { $signed ->
        [yes] Hersteller- oder Windows-Treiber „{ $name }“ vom Typ { $kind } ({ $path }, signiert von { $signer }). WinAppMigrator gibt keine Treiber von Drittanbietern weiter. Installieren Sie ihn auf dem Zielcomputer mit dem signierten Treiberpaket des Herstellers neu, bevor Sie die App starten.
       *[no] Hersteller- oder Windows-Treiber „{ $name }“ vom Typ { $kind } ({ $path }, Signatur unbekannt). WinAppMigrator gibt keine Treiber von Drittanbietern weiter. Installieren Sie ihn auf dem Zielcomputer mit dem signierten Treiberpaket des Herstellers neu, bevor Sie die App starten.
    }

## Runtimes an app needs, such as the Visual C++ runtime or .NET: how each is provided, as the Dependencies list says.

core-runtime-handling-built-in = In Windows integriert
core-runtime-handling-ships-with-app = Wird mit der App geliefert
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-handling-windows-feature = Windows-Feature: { $feature }
core-runtime-handling-included = Enthalten
# { $id } is the runtime's name in winget, such as Microsoft.VCRedist.2015+.x64.
core-runtime-handling-winget = Wird mit winget installiert: { $id }
core-runtime-handling-manual = Manuell installieren

## Downloading a runtime's installer, when an installer is built. Each is why it could not be, and starts with a small
## letter, as it follows other words. { $url } is a web address; { $error } is what went wrong.

core-runtime-download-not-https = Download von { $url } wird verweigert: keine HTTPS-Adresse
# TLS is what keeps a download private.
core-runtime-tls-failed = TLS-Initialisierung fehlgeschlagen: { $error }
core-runtime-download-failed = Download von { $url } fehlgeschlagen: { $error }
core-runtime-download-unreadable = Lesen von { $url } fehlgeschlagen: { $error }
core-runtime-download-empty = Download von { $url } lieferte keine Daten
# { $came } and { $expected } are numbers of bytes.
core-runtime-download-cut-short = Download von { $url } wurde vorzeitig beendet: { $came } von { $expected } Bytes empfangen

## Installing a runtime: why it was not installed. Each starts with a small letter, as it follows other words.

core-runtime-no-known-installer = für diese Laufzeitkomponente ist kein Installationsprogramm bekannt
# { $path } is where in the package, such as prerequisites\vc_redist.x64.exe.
core-runtime-payload-misplaced = das Paket gibt { $path } als Installationsprogramm dieser Laufzeitkomponente an, aber dort gehört es nicht hin, daher wurde es nicht ausgeführt
# { $path } is a file.
core-runtime-payload-missing = das gebündelte Installationsprogramm fehlt: { $path }
# { $error } is what went wrong.
core-runtime-payload-path-invalid = der Pfad des gebündelten Installationsprogramms ist ungültig: { $error }
# { $url } is the web address it can be downloaded from. winget is a program's name.
core-runtime-no-installer-route = kein gebündeltes Installationsprogramm und keine winget-ID. Beziehen Sie die Komponente von { $url }
# { $error } is what went wrong.
core-runtime-no-temp-folder = es konnte kein Ordner zum Entpacken erstellt werden: { $error }
# { $reason } is what the check found.
core-runtime-payload-signature-failed = das gebündelte Installationsprogramm hat die Signaturprüfung nicht bestanden und wurde nicht ausgeführt: { $reason }
core-runtime-payload-timed-out = das gebündelte Installationsprogramm wurde nicht innerhalb des Zeitlimits abgeschlossen und daher beendet
# { $error } is what went wrong.
core-runtime-payload-not-started = das gebündelte Installationsprogramm konnte nicht gestartet werden: { $error }
core-runtime-installer-killed = das Installationsprogramm wurde ohne Exitcode beendet
# { $code } is the number it ended with, such as 1603.
core-runtime-installer-failed = das Installationsprogramm wurde mit Code { $code } beendet
# { $feature } is a Windows feature's name, such as NetFx3; { $code } is the number DISM ended with. Windows Features is
# the name of a Windows window. Keep the command as it is.
core-runtime-feature-failed = DISM konnte das Windows-Feature { $feature } nicht aktivieren (Exitcode { $code }). Die Nutzdaten konnten wahrscheinlich nicht heruntergeladen werden. Aktivieren Sie es über „Windows-Features“, oder führen Sie `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess` aus.
# The same, when DISM gave no number.
core-runtime-feature-failed-no-code = DISM konnte das Windows-Feature { $feature } nicht aktivieren (Exitcode unbekannt). Die Nutzdaten konnten wahrscheinlich nicht heruntergeladen werden. Aktivieren Sie es über „Windows-Features“, oder führen Sie `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess` aus.
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-feature-timed-out = das Aktivieren des Windows-Features { $feature } wurde nicht innerhalb des Zeitlimits abgeschlossen und daher beendet
# { $feature } is a Windows feature's name; { $error } is what went wrong.
core-runtime-feature-no-dism = DISM ist zum Aktivieren des Windows-Features { $feature } nicht verfügbar: { $error }

## Installing a runtime with winget. { $id } is its name in winget, such as Microsoft.VCRedist.2015+.x64.

# { $error } is what went wrong.
core-runtime-winget-not-looked-for = es konnte nicht nach winget gesucht werden: { $error }
core-runtime-winget-not-installed = winget ist nicht installiert
# { $path } is where winget is.
core-runtime-winget-misplaced = { $path } befindet sich nicht dort, wo Windows seine eigenen Pakete ablegt, daher wurde es nicht ausgeführt
# { $path } is where winget is; { $reason } is what the check found.
core-runtime-winget-signature-failed = { $path } hat die Signaturprüfung nicht bestanden und wurde nicht ausgeführt: { $reason }
# { $reason } is one of the two messages above, or what went wrong.
core-runtime-winget-unusable = winget konnte nicht zum Installieren von { $id } verwendet werden: { $reason }
# { $code } is the number winget ended with.
core-runtime-winget-failed = winget install { $id } wurde mit { $code } beendet
core-runtime-winget-failed-no-code = winget install { $id } wurde mit einem unbekannten Code beendet
core-runtime-winget-timed-out = winget install { $id } wurde nicht innerhalb des Zeitlimits abgeschlossen und daher beendet
# { $error } is what went wrong.
core-runtime-winget-not-started = winget ist zum Installieren von { $id } nicht verfügbar: { $error }

## What told a scan how an app was installed, as the package's report lists it. The names of installers and files are
## as their makers write them.

# { $path } is an installer's record of what it installed, such as C:\Program Files\App\unins000.dat.
core-source-record-not-believed = { $path } wurde nicht verwendet, da andere als Administratoren die Datei ändern können
# { $count } is how many Windows Installer packages the bundle holds.
core-source-burn-bundle =
    { $count ->
        [one] Burn-Bundle mit { $count } Windows Installer-Paket
       *[other] Burn-Bundle mit { $count } Windows Installer-Paketen
    }
# { $count } is how many paths Windows Installer recorded.
core-source-msi-database =
    { $count ->
        [one] Windows Installer-Datenbank: { $count } Komponentenschlüsselpfad
       *[other] Windows Installer-Datenbank: { $count } Komponentenschlüsselpfade
    }
core-source-inno-log = Inno Setup-Deinstallationsprotokoll (unins000.dat)
# { $count } is how many paths the log records.
core-source-inno-log-paths =
    { $count ->
        [one] Inno Setup-Deinstallationsprotokoll: { $count } aufgezeichneter Pfad
       *[other] Inno Setup-Deinstallationsprotokoll: { $count } aufgezeichnete Pfade
    }
core-source-nsis = Signatur eines NSIS-Deinstallationsprogramms (Nullsoft)
# { $count } is how many paths the log records.
core-source-nsis-log-paths =
    { $count ->
        [one] NSIS AdvUninstLog uninstall.log: { $count } aufgezeichneter Pfad
       *[other] NSIS AdvUninstLog uninstall.log: { $count } aufgezeichnete Pfade
    }
core-source-squirrel = Squirrel-Struktur pro Benutzer (Update.exe / app-*)
# { $version } is a folder's name that says the version, such as app-1.2.3.
core-source-squirrel-version = Aktuelle Squirrel-Version: { $version }
core-source-chocolatey = Chocolatey-Paketmetadaten
core-source-msix = MSIX-/Appx-Paket
# { $identity } is a package's name, such as Contoso.App.
core-source-appx-identity = AppxManifest-Identität: { $identity }
# { $count } is how many folders.
core-source-msix-data =
    { $count ->
        [one] MSIX-Paketdaten pro Benutzer: { $count } Speicherort
       *[other] MSIX-Paketdaten pro Benutzer: { $count } Speicherorte
    }
# App Paths is a registry key's name.
core-source-app-paths = Registrierung unter App Paths

## Why a runtime is needed, as the package's report says it.

# { $program } is a program's file name, such as app.exe; { $dll } is a DLL's, such as msvcp140.dll.
core-runtime-program-imports = { $program } importiert { $dll }

## What else of an app's a scan finds in the registry: fonts and certificates, and why each is the app's.

# Why a font is among the app's files, in the package's list of them.
core-registry-font-recorded = Eine Schriftart, die im Installationseintrag der App genannt wird
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-named-for-the-app = es ist nach der App oder ihrem Herausgeber benannt
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-recorded = der Installationseintrag der App nennt es

## Who is signed in, for the Whose Data choice. Each of the first six is why it could not be told, and starts with a
## small letter, as it is shown inside other words. An account is shown by its id, such as S-1-5-21-...-1001.

# { $error } is what Windows said.
core-profile-desktop-owner-not-said = Windows hat nicht angegeben, wem dieser Desktop gehört: { $error }
# { $error } is what went wrong.
core-profile-desktop-owner-not-asked = Windows konnte nicht gefragt werden, wem dieser Desktop gehört: { $error }
core-profile-signed-in-account-no-profile = das hier angemeldete Konto { $account } hat kein verzeichnetes Profil
core-profile-desktop-not-a-person = der Desktop hier gehört { $account }, und das ist nicht das Konto einer Person
core-profile-nobody-signed-in-running-account-no-profile = hier ist niemand angemeldet, und das Konto, unter dem dies ausgeführt wird, { $account }, hat kein verzeichnetes Profil
core-profile-nobody-signed-in-running-not-a-person = hier ist niemand angemeldet, und { $account }, unter dem dies ausgeführt wird, ist nicht das Konto einer Person
# SID is the name Windows gives an account's id.
core-profile-nobody-signed-in-no-sid = hier ist niemand angemeldet, und dieses Konto hat keine SID
# { $accounts } is a list of accounts.
core-profile-several-desktops = mehr als ein Konto hat in dieser Sitzung einen Desktop: { $accounts }
# Under the Whose Data choice. { $reason } is one of the messages above.
core-profile-signed-in-unknown = Es konnte nicht ermittelt werden, wer angemeldet ist ({ $reason }).
# Under the Whose Data choice, when there are accounts to choose from. { $reason } is one of the messages above.
core-profile-signed-in-unknown-choose = Es konnte nicht ermittelt werden, wer angemeldet ist ({ $reason }). Wählen Sie aus, wessen Daten aufgenommen werden sollen.
