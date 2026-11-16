# Words the core writes for people: warnings, refusals, errors, and the installer's list of what it sets up.
# Message names start with core-.


## Warnings a package records when it is built, which an installer shows later, maybe on another PC, in that PC's
## language.

# { $count } is how many shortcuts.
core-warning-shortcuts-unreadable =
    { $count ->
        [one] { $count } snelkoppeling op dit station verwijst naar geen enkel bestand, dus er kon niet worden vastgesteld of die snelkoppeling deze app opent.
       *[other] { $count } snelkoppelingen op dit station verwijzen naar geen enkel bestand, dus er kon niet worden vastgesteld of ze deze app openen.
    }

## Undoing an install: what an uninstall, or a failed install, says it put back, left, or could not undo. Each line is
## shown on its own, in a list.

# { $app_name } is the app's name.
core-rollback-record-damaged = Een deel van het installatieverslag van { $app_name } kan niet worden gelezen, dus alles wat daarin werd beschreven, staat er nog, en een bestand dat werd vervangen, komt mogelijk niet terug.
core-rollback-settings-entry-stays = Het item van de app in Instellingen blijft staan, zodat dit daar opnieuw kan worden uitgevoerd.
# { $name } is a runtime's name, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-rollback-prerequisite-stays = { $name } is geïnstalleerd, of mogelijk geïnstalleerd, en blijft staan, omdat andere programma's het mogelijk gebruiken.
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-drive-not-connected = Wat deze installatie in { $folder } heeft geplaatst, staat op een station dat niet is aangesloten. Sluit het station aan, dan kan dit worden voltooid.
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-record-part-unread = Wat deze installatie in { $folder } heeft geplaatst, blijft staan, omdat dat deel van het installatieverslag niet kon worden gelezen.

## Why a file or folder was left, shown after "was left:" in the messages below them. Each is part of a sentence, so
## starts with a small letter and has no full stop.

# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-being-changed = iets wijzigde { $folder } terwijl dit werd uitgevoerd, dus er is verder niets in gedaan. Sluit wat dat doet en voer dit opnieuw uit
# { $folder } is a folder; { $error } is what Windows said went wrong.
core-rollback-reason-not-opened = { $folder } kan niet worden geopend: { $error }
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-no-record = er is niet vastgelegd waar { $folder } stond
# { $folder } is a folder; { $actual } is the folder it now leads to.
core-rollback-reason-moved = { $folder } leidt nu naar { $actual }, niet naar waar het tijdens de installatie naartoe leidde
# Why a service was not stopped.
core-rollback-reason-did-not-stop = het is niet op tijd gestopt

## Files and folders

# { $path } is a folder; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-folder-left = De map { $path } is blijven staan: { $reason }.
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-removed = Map { $path } verwijderd
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-holds-others = De map { $path } blijft staan, omdat er bestanden in staan die deze installatie er niet in heeft geplaatst.
# { $path } is a folder; { $error } is what Windows said went wrong.
core-rollback-folder-not-removed = De map { $path } kan niet worden verwijderd: { $error }.
# { $path } is a file; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-file-left = Het bestand { $path } is blijven staan: { $reason }.
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-folder-gone = { $path } wordt niet teruggezet, omdat de map waarin het stond, is verdwenen.
# { $path } is a file; { $error } is what went wrong.
core-rollback-partial-not-removed = Een onvolledig bestand kan niet worden verwijderd: { $path } ({ $error }).
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-checked = { $path } kan niet worden gecontroleerd: { $error }.
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-removed = { $path } verwijderd
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-removed = { $path } kan niet worden verwijderd: { $error }.
# { $path } is a file this install had replaced, and has put back.
core-rollback-file-put-back = { $path } teruggezet
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-put-back = { $path } kan niet worden teruggezet: { $error }.
# Why a file could not be put back.
core-rollback-backup-gone = de back-up is verdwenen
# { $path } is a file.
core-rollback-no-parent-folder = { $path } heeft geen bovenliggende map

## A file that stays. { $why } says why: still-used (something that stays still uses it), changed (it has changed since
## it was installed), or not-asked (it is someone's data, and they did not ask for it to go). { $path } is the file.

core-rollback-file-stays =
    { $why ->
        [still-used] { $path } blijft staan, omdat iets wat blijft staan het nog gebruikt.
        [changed] { $path } blijft staan, omdat het is gewijzigd sinds het is geïnstalleerd.
       *[not-asked] { $path } blijft staan, omdat niet is gevraagd het te verwijderen.
    }
# The file it replaced was to be put beside it, under another name, and every name was taken.
core-rollback-file-stays-no-name-beside =
    { $why ->
        [still-used] { $path } blijft staan, omdat iets wat blijft staan het nog gebruikt, en het bestand dat het verving, kan er niet naast worden geplaatst: elke naam ervoor is al in gebruik.
        [changed] { $path } blijft staan, omdat het is gewijzigd sinds het is geïnstalleerd, en het bestand dat het verving, kan er niet naast worden geplaatst: elke naam ervoor is al in gebruik.
       *[not-asked] { $path } blijft staan, omdat niet is gevraagd het te verwijderen, en het bestand dat het verving, kan er niet naast worden geplaatst: elke naam ervoor is al in gebruik.
    }
# { $name } is the name the file it replaced now has, such as settings.ini.before-install.
core-rollback-file-stays-beside =
    { $why ->
        [still-used] { $path } blijft staan, omdat iets wat blijft staan het nog gebruikt. Het bestand dat het verving, staat ernaast, als { $name }.
        [changed] { $path } blijft staan, omdat het is gewijzigd sinds het is geïnstalleerd. Het bestand dat het verving, staat ernaast, als { $name }.
       *[not-asked] { $path } blijft staan, omdat niet is gevraagd het te verwijderen. Het bestand dat het verving, staat ernaast, als { $name }.
    }
# { $error } is what went wrong.
core-rollback-file-stays-not-beside =
    { $why ->
        [still-used] { $path } blijft staan, omdat iets wat blijft staan het nog gebruikt, en het bestand dat het verving, kan er niet naast worden geplaatst: { $error }.
        [changed] { $path } blijft staan, omdat het is gewijzigd sinds het is geïnstalleerd, en het bestand dat het verving, kan er niet naast worden geplaatst: { $error }.
       *[not-asked] { $path } blijft staan, omdat niet is gevraagd het te verwijderen, en het bestand dat het verving, kan er niet naast worden geplaatst: { $error }.
    }

## The registry and environment variables. { $kind } says what { $name } is: key (a registry key, such as
## HKLM\Software\App), value (a registry value, such as HKLM\Software\App\Version), user-variable or machine-variable
## (an environment variable of the user's or of the whole PC, such as APP_HOME), or user-list or machine-list (a
## variable that is a list of folders, such as Path). { $error } is what Windows said went wrong.

# How a registry key's unnamed value is shown, as Registry Editor shows it.
core-registry-default-value-name = (Standaard)
core-rollback-setting-other-account =
    { $kind ->
        [key] De registersleutel { $name } blijft staan, omdat deze hoort bij het account waarmee dit is geïnstalleerd, en dat is niet aangemeld.
        [user-variable] De gebruikersvariabele { $name } blijft staan, omdat deze hoort bij het account waarmee dit is geïnstalleerd, en dat is niet aangemeld.
        [machine-variable] De systeemvariabele { $name } blijft staan, omdat deze hoort bij het account waarmee dit is geïnstalleerd, en dat is niet aangemeld.
        [user-list] De gebruikersvariabele { $name } blijft staan, omdat deze hoort bij het account waarmee dit is geïnstalleerd, en dat is niet aangemeld.
        [machine-list] De systeemvariabele { $name } blijft staan, omdat deze hoort bij het account waarmee dit is geïnstalleerd, en dat is niet aangemeld.
       *[value] De registerwaarde { $name } blijft staan, omdat deze hoort bij het account waarmee dit is geïnstalleerd, en dat is niet aangemeld.
    }
core-rollback-setting-not-reached =
    { $kind ->
        [key] De registersleutel { $name } kan niet worden bereikt: { $error }.
        [user-variable] De gebruikersvariabele { $name } kan niet worden bereikt: { $error }.
        [machine-variable] De systeemvariabele { $name } kan niet worden bereikt: { $error }.
        [user-list] De gebruikersvariabele { $name } kan niet worden bereikt: { $error }.
        [machine-list] De systeemvariabele { $name } kan niet worden bereikt: { $error }.
       *[value] De registerwaarde { $name } kan niet worden bereikt: { $error }.
    }
core-rollback-setting-not-read =
    { $kind ->
        [key] De registersleutel { $name } kan niet worden gelezen: { $error }.
        [user-variable] De gebruikersvariabele { $name } kan niet worden gelezen: { $error }.
        [machine-variable] De systeemvariabele { $name } kan niet worden gelezen: { $error }.
        [user-list] De gebruikersvariabele { $name } kan niet worden gelezen: { $error }.
        [machine-list] De systeemvariabele { $name } kan niet worden gelezen: { $error }.
       *[value] De registerwaarde { $name } kan niet worden gelezen: { $error }.
    }
core-rollback-setting-changed =
    { $kind ->
        [user-variable] De gebruikersvariabele { $name } blijft staan, omdat deze is gewijzigd sinds de installatie.
        [machine-variable] De systeemvariabele { $name } blijft staan, omdat deze is gewijzigd sinds de installatie.
        [user-list] De gebruikersvariabele { $name } blijft staan, omdat deze is gewijzigd sinds de installatie.
        [machine-list] De systeemvariabele { $name } blijft staan, omdat deze is gewijzigd sinds de installatie.
       *[value] De registerwaarde { $name } blijft staan, omdat deze is gewijzigd sinds de installatie.
    }
# Said when it is back as it was before the install.
core-rollback-setting-put-back =
    { $kind ->
        [user-variable] Gebruikersvariabele { $name } teruggezet
        [machine-variable] Systeemvariabele { $name } teruggezet
        [user-list] Gebruikersvariabele { $name } teruggezet
        [machine-list] Systeemvariabele { $name } teruggezet
       *[value] Registerwaarde { $name } teruggezet
    }
core-rollback-setting-not-put-back =
    { $kind ->
        [user-variable] De gebruikersvariabele { $name } kan niet worden teruggezet: { $error }.
        [machine-variable] De systeemvariabele { $name } kan niet worden teruggezet: { $error }.
        [user-list] De gebruikersvariabele { $name } kan niet worden teruggezet: { $error }.
        [machine-list] De systeemvariabele { $name } kan niet worden teruggezet: { $error }.
       *[value] De registerwaarde { $name } kan niet worden teruggezet: { $error }.
    }
# A registry value the install wrote, which is the user's data, and stays unless they asked for it to go.
core-rollback-value-kept-by-request = De registerwaarde { $name } blijft staan, omdat niet is gevraagd deze te verwijderen.
core-rollback-key-holds-others = De registersleutel { $name } blijft staan, omdat deze iets bevat wat deze installatie er niet in heeft geplaatst.
core-rollback-key-removed = Registersleutel { $name } verwijderd
core-rollback-key-not-removed = De registersleutel { $name } kan niet worden verwijderd: { $error }.
# { $kind } is user-list or machine-list.
core-rollback-list-entries-removed =
    { $kind ->
        [user-list] Items van deze installatie verwijderd uit gebruikersvariabele { $name }
       *[machine-list] Items van deze installatie verwijderd uit systeemvariabele { $name }
    }
core-rollback-list-entries-not-removed =
    { $kind ->
        [user-list] De items van deze installatie kunnen niet worden verwijderd uit de gebruikersvariabele { $name }: { $error }.
       *[machine-list] De items van deze installatie kunnen niet worden verwijderd uit de systeemvariabele { $name }: { $error }.
    }

## Services and drivers. { $kind } is service or driver; { $name } is its name, such as AppUpdater. { $error } and
## { $reason } are what Windows said went wrong.

core-rollback-service-not-read =
    { $kind ->
        [driver] Het stuurprogramma { $name } kan niet worden gelezen: { $error }.
       *[service] De service { $name } kan niet worden gelezen: { $error }.
    }
core-rollback-service-changed =
    { $kind ->
        [driver] Het stuurprogramma { $name } blijft staan, omdat het is gewijzigd sinds de installatie.
       *[service] De service { $name } blijft staan, omdat deze is gewijzigd sinds de installatie.
    }
core-rollback-service-not-stopped =
    { $kind ->
        [driver] Het stuurprogramma { $name } kan niet worden gestopt ({ $reason }), dus wat het gebruikt, blijft mogelijk in gebruik tot de pc opnieuw wordt opgestart.
       *[service] De service { $name } kan niet worden gestopt ({ $reason }), dus wat deze gebruikt, blijft mogelijk in gebruik tot de pc opnieuw wordt opgestart.
    }
core-rollback-service-stopped-for-this =
    { $kind ->
        [driver] Het stuurprogramma { $name } is hiervoor gestopt. Start het opnieuw wanneer je het nodig hebt.
       *[service] De service { $name } is hiervoor gestopt. Start deze opnieuw wanneer je deze nodig hebt.
    }
core-rollback-service-not-removed =
    { $kind ->
        [driver] Het stuurprogramma { $name } kan niet worden verwijderd: { $reason }.
       *[service] De service { $name } kan niet worden verwijderd: { $reason }.
    }
core-rollback-service-marked-for-removal =
    { $kind ->
        [driver] Het stuurprogramma { $name } is gemarkeerd voor verwijdering en verdwijnt wanneer de pc opnieuw wordt opgestart of wanneer wat het open heeft, het loslaat.
       *[service] De service { $name } is gemarkeerd voor verwijdering en verdwijnt wanneer de pc opnieuw wordt opgestart of wanneer wat deze open heeft, deze loslaat.
    }
core-rollback-service-removed =
    { $kind ->
        [driver] Stuurprogramma { $name } verwijderd
       *[service] Service { $name } verwijderd
    }
core-rollback-service-not-restored =
    { $kind ->
        [driver] Het stuurprogramma { $name } kan niet worden teruggezet zoals het was: { $reason }.
       *[service] De service { $name } kan niet worden teruggezet zoals deze was: { $reason }.
    }
core-rollback-service-restored =
    { $kind ->
        [driver] Instellingen van stuurprogramma { $name } teruggezet
       *[service] Instellingen van service { $name } teruggezet
    }
core-rollback-service-not-restarted = De service { $name } werd uitgevoerd vóór deze installatie en is niet opnieuw gestart: { $reason }.
# { $account } is the account the service ran as, such as CONTOSO\backup.
core-rollback-service-account-again = De service { $name } werd uitgevoerd als { $account }. Het wachtwoord is niet bewaard, dus stel het account opnieuw in via Services.

## Firewall rules. { $name } is the rule's name; { $error } is what went wrong.

core-rollback-firewall-not-read = De firewallregel { $name } kan niet worden gelezen: { $error }.
core-rollback-firewall-says-otherwise = De firewallregel { $name } blijft staan, omdat de regel met die naam iets anders bevat.
core-rollback-firewall-changed = De firewallregel { $name } blijft staan, omdat deze is gewijzigd sinds de installatie.
# The capital "The" is as the app has always said it.
core-rollback-firewall-removed = Firewallregel { $name } verwijderd
core-rollback-firewall-not-removed = De firewallregel { $name } kan niet worden verwijderd: { $error }.

## Scheduled tasks. { $path } is a folder of tasks, and { $name } a task with its folder, such as \App\Nightly Update.
## { $error } and { $reason } are what went wrong.

core-rollback-task-folder-holds-others = De taakmap { $path } blijft staan, omdat er taken in staan die deze installatie er niet in heeft geplaatst.
core-rollback-task-folder-not-removed = De taakmap { $path } kan niet worden verwijderd: { $error }.
core-rollback-task-not-read = De taak { $name } kan niet worden gelezen: { $error }.
core-rollback-task-changed = De taak { $name } blijft staan, omdat deze is gewijzigd sinds de installatie.
core-rollback-task-removed = Taak { $name } verwijderd
core-rollback-task-not-removed = De taak { $name } kan niet worden verwijderd: { $reason }.
core-rollback-task-put-back = Taak { $name } teruggezet
core-rollback-task-not-put-back = De taak { $name } kan niet worden teruggezet: { $reason }.

## When an install cannot be undone at all

# { $error } is what went wrong.
core-rollback-record-unreadable = Het installatieverslag kan niet worden gelezen, dus er is niets ongedaan gemaakt: { $error }.
# { $error } is what went wrong.
core-rollback-nothing-undone = Er is niets ongedaan gemaakt: { $error }.
# One app's part of the message below. { $app_name } is the app's name; { $failures } is what could not be undone, a
# sentence or more; { $folder } is the folder that holds the install's record.
core-rollback-earlier-install-stuck = { $app_name }: { $failures } Als dit nooit ongedaan kan worden gemaakt, kan een administrator het opgeven door { $folder } te verwijderen.
# Why an install did not start. { $details } is one or more of the message above.
core-rollback-earlier-installs-not-undone = een eerdere installatie die nooit is voltooid, kan niet volledig ongedaan worden gemaakt, dus er is niets geïnstalleerd. Sluit alles wat deze bestanden gebruikt en probeer het opnieuw. { $details }
core-rollback-install-busy = deze installatie wordt gewijzigd door een ander installatie- of verwijderprogramma

## The summary an uninstall writes on the command line, one line each, with what each says below it. { $count } is how
## many.

core-rollback-count-undone = Ongedaan gemaakt: { $count }
core-rollback-count-failed = Kon niet ongedaan worden gemaakt: { $count }
# What was left on purpose: it has changed since the install, or it is the user's.
core-rollback-count-kept = Blijft staan: { $count }
# What no uninstall can take back, such as a runtime that was installed, which other programs may use.
core-rollback-count-left-behind = Achtergebleven: { $count }

## Which Windows a PC or drive has, as the app shows it beside the drive.

# { $name } is the Windows' name, such as Windows 7 Professional Service Pack 1.
core-system-windows-32-bit = { $name }, 32-bits
# { $name } is the Windows' name, such as Windows 11 Pro.
core-system-windows-64-bit = { $name }, 64-bits
# A drive's Windows, when it only says it is older than Vista.
core-system-windows-xp-or-older = Windows XP of ouder

## Reading a connected drive, or another account's settings. Each is why it could not be done, and some start with a
## small letter, as they follow other words.

core-system-needs-administrator = voor het lezen van een aangesloten station of de instellingen van een ander account zijn administratorrechten nodig
core-system-no-program-data = de map ProgramData van deze pc is niet gevonden
# { $path } is a file, such as D:\Windows\System32\config\SOFTWARE.
core-system-not-a-hive-file = geen registercomponentbestand: { $path }
# { $path } is a file; { $size } is its size in bytes.
core-system-hive-too-large = { $path } is { $size } bytes, te groot voor een registercomponent, of groter dan alle registercomponenten van dit station samen kunnen zijn
# { $key } is a registry key, such as Software\Microsoft\Windows\CurrentVersion.
core-system-hive-no-mapping = offline registercomponent heeft geen toewijzing voor HKLM\{ $key }
# { $key } is where a drive's registry was to be loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said.
# "reg load" is a command's name.
core-system-hive-not-loaded = reg load { $key } mislukt: { $error }
# { $key } is where a drive's registry was loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said. "reg
# unload" is a command's name.
core-system-hive-not-unloaded = reg unload { $key } mislukt: { $error }
# { $drive } is a drive, such as D:\.
core-system-not-a-system-drive = { $drive } is geen Windows-systeemstation (geen map \Windows).
# { $drive } is a drive, such as C:\. "This PC" is the name of a choice in the app.
core-system-own-system-drive = { $drive } is het eigen systeemstation van deze pc. Kies Deze pc om deze pc te lezen.
# { $drive } is a drive, such as D:\.
core-system-registry-through-link = { $drive } is geen Windows-systeemstation: de weg naar het register loopt via een koppeling.

## Warnings a scan records about whose data it read.

# { $account } is an account's name or id, such as alice or S-1-5-21-...-1001.
core-warning-scan-account-not-on-record = Het account { $account } is hier niet geregistreerd, dus er zijn geen gegevens of instellingen per gebruiker opgenomen.
# { $account } is an account's name, such as CONTOSO\alice.
core-warning-scan-signed-in-account-no-profile = Het account dat hier is aangemeld, { $account }, heeft geen geregistreerd profiel, dus er zijn geen gegevens of instellingen per gebruiker opgenomen.
# { $reason } is why Windows could not say.
core-warning-scan-signed-in-account-unknown = Kan niet vaststellen wie er is aangemeld ({ $reason }), dus er zijn geen gegevens of instellingen per gebruiker opgenomen. Kies wiens gegevens je wilt opnemen.
core-warning-scan-last-account-unknown = Dit station vermeldt niet wie zich het laatst heeft aangemeld en heeft meer dan één account, dus er zijn geen gegevens of instellingen per gebruiker opgenomen. Kies wiens gegevens je wilt opnemen.
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-settings-unreadable = De instellingen van { $account } kunnen niet worden gelezen, dus ze zijn niet opgenomen: { $error }
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-classes-unreadable = De COM-klassen en bestandstypen die voor { $account } zijn geregistreerd, kunnen niet worden gelezen, dus ze zijn niet opgenomen: { $error }

## Paths a scan did not follow. { $count } is how many; { $paths } is some of them, separated by semicolons; { $more } is
## how many more there are, not shown.

core-warning-scan-drive-paths-not-followed =
    { $count ->
        [one] Een pad dat op dit station is vastgelegd, kan niet worden gelezen, dus wat daar staat, is niet opgenomen. Elk pad staat op een andere schijf, is geen volledig pad of loopt via een koppeling, en zou dus buiten het station leiden: { $paths }
       *[other] Sommige paden die op dit station zijn vastgelegd, kunnen niet worden gelezen, dus wat daar staat, is niet opgenomen. Elk pad staat op een andere schijf, is geen volledig pad of loopt via een koppeling, en zou dus buiten het station leiden: { $paths }
    }
core-warning-scan-drive-paths-not-followed-more = Sommige paden die op dit station zijn vastgelegd, kunnen niet worden gelezen, dus wat daar staat, is niet opgenomen. Elk pad staat op een andere schijf, is geen volledig pad of loopt via een koppeling, en zou dus buiten het station leiden: { $paths } en nog { $more }
core-warning-scan-network-paths-not-followed =
    { $count ->
        [one] Een pad dat op deze pc is vastgelegd, kan niet worden gelezen, dus wat daar staat, is niet opgenomen. Elk pad staat niet op een schijf van deze pc, zodat bij het bekijken verbinding zou worden gemaakt, of heeft een naam die iets anders zou openen: { $paths }
       *[other] Sommige paden die op deze pc zijn vastgelegd, kunnen niet worden gelezen, dus wat daar staat, is niet opgenomen. Elk pad staat niet op een schijf van deze pc, zodat bij het bekijken verbinding zou worden gemaakt, of heeft een naam die iets anders zou openen: { $paths }
    }
core-warning-scan-network-paths-not-followed-more = Sommige paden die op deze pc zijn vastgelegd, kunnen niet worden gelezen, dus wat daar staat, is niet opgenomen. Elk pad staat niet op een schijf van deze pc, zodat bij het bekijken verbinding zou worden gemaakt, of heeft een naam die iets anders zou openen: { $paths } en nog { $more }

## What a scan with none of an account's data leaves out of a package, in the app's list of what was left out. Each
## has a name, and below it why.

core-account-left-out-shortcuts = Snelkoppelingen in de eigen mappen van dit account
# { $count } is how many shortcuts.
core-account-left-out-shortcuts-why =
    { $count ->
        [one] { $count } snelkoppeling, die alleen meegaat met de optie Gebruikersgegevens
       *[other] { $count } snelkoppelingen, die alleen meegaan met de optie Gebruikersgegevens
    }
core-account-left-out-variables = Omgevingsvariabelen van dit account
# { $count } is how many variables.
core-account-left-out-variables-why =
    { $count ->
        [one] { $count } variabele, die alleen meegaat met de optie Gebruikersgegevens
       *[other] { $count } variabelen, die alleen meegaan met de optie Gebruikersgegevens
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-account-left-out-task = De geplande taak { $task }
core-account-task-works-on-account-folder = Deze werkt met een plek in de mappen van een account, of is daarnaar genoemd, en gaat alleen mee met de optie Gebruikersgegevens
core-account-task-runs-from-account-folder = Wat deze taak ook vanuit de mappen van een account uitvoert, gaat alleen mee met de optie Gebruikersgegevens
# { $service } is a service's name, such as AppUpdater.
core-account-left-out-service = De service { $service }
core-account-service-works-on-account-folder = Deze werkt met een plek in de mappen van een account en gaat alleen mee met de optie Gebruikersgegevens

## What a package was found to still say of whose PC it came from, after a build with none of an account's data.

# The parts of a package, as the warning below names them.
core-account-part-files = de paden van de eigen bestanden van de app
core-account-part-registry = registerwaarden
core-account-part-settings-entry = het item van de app in Instellingen
core-account-part-main-program = waar het hoofdprogramma staat
core-account-part-shortcuts = snelkoppelingen
core-account-part-tasks = geplande taken
core-account-part-services = services
core-account-part-variables = omgevingsvariabelen
# One place in the list below. { $place } is one of the parts above, or a file's name; { $count } is how many times.
core-account-mention-place = { $place } ({ $count })
# { $kind } is what was found: account (an account's name), folder (where an account's folder is), computer (the PC's
# name), or id (an account's id). { $places } is a list of where, from the message above.
core-warning-scan-package-mentions =
    { $kind ->
        [account] Dit pakket vermeldt de naam van een account in { $places }. Het is gemaakt zonder gegevens van een account, en iedereen die het installatieprogramma heeft, kan dit lezen.
        [folder] Dit pakket vermeldt waar de map van een account staat, in { $places }. Het is gemaakt zonder gegevens van een account, en iedereen die het installatieprogramma heeft, kan dit lezen.
        [computer] Dit pakket vermeldt de naam van de pc in { $places }. Het is gemaakt zonder gegevens van een account, en iedereen die het installatieprogramma heeft, kan dit lezen.
       *[id] Dit pakket vermeldt de id van een account in { $places }. Het is gemaakt zonder gegevens van een account, en iedereen die het installatieprogramma heeft, kan dit lezen.
    }
# { $count } is how many folders.
core-warning-scan-folders-of-others-unreadable =
    { $count ->
        [one] { $count } map buiten de app kan niet worden gelezen, dus gegevens van de app daarin ontbreken mogelijk.
       *[other] { $count } mappen buiten de app kunnen niet worden gelezen, dus gegevens van de app daarin ontbreken mogelijk.
    }

## What a scan leaves out because it looks like a credential, or is a license with the License option off, in the app's
## list of what was left out. Each has a name, and below it why.

core-secret-why-credential = het lijkt op een referentie
core-secret-why-license-option-off = de optie Licentie- en activeringsgegevens is uitgeschakeld
# { $count } is how many values of a registry key.
core-secret-why-account-values =
    { $count ->
        [one] { $count } waarde, die alleen meegaat met de optie Gebruikersgegevens
       *[other] { $count } waarden, die alleen meegaan met de optie Gebruikersgegevens
    }
# Shown in place of a registry value's name when its name itself looks like a password or a key.
core-secret-name-holds-credential = (een waarde waarvan de naam een referentie bevat)
# Shown in place of an option's name, for an argument on a command line that has no name.
core-secret-unnamed-argument = een argument met een referentie
# { $option } is an option taken out of a command line, such as /password, or the message above. { $place } says
# where the command line was: service, task, shortcut, opener (what opens a file type), opener-unnamed (the same, of a
# type with no name), or uninstall. { $name } is the service, task, shortcut, file type, or app.
core-secret-taken-from =
    { $place ->
        [service] { $option } in de opdracht waarmee { $name } wordt uitgevoerd
        [task] { $option } in de opdracht waarmee de taak { $name } wordt uitgevoerd
        [shortcut] { $option } in de snelkoppeling { $name }
        [opener] { $option } in de opdracht waarmee { $name } wordt geopend
        [opener-unnamed] { $option } in de opdracht waarmee een bestandstype wordt geopend
       *[uninstall] { $option } in de opdracht waarmee { $name } wordt verwijderd
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-secret-task-definition-unreadable = De definitie van de taak { $task }, die niet kon worden gelezen om deze te controleren
# { $name } is an environment variable's name, such as APP_TOKEN.
core-secret-left-out-variable = De omgevingsvariabele { $name }
# { $class } is a COM class's id, such as {00000000-0000-0000-0000-000000000000}.
core-secret-left-out-class-server = Wat de klasse { $class } volgens eigen opgave uitvoert
# { $rule } is a firewall rule's name.
core-secret-left-out-firewall-description = De beschrijving van de firewallregel { $rule }

## Licenses a scan found, as the package's report describes each.

# { $name } is a file's name, such as license.lic.
core-license-detail-file = Licentiebestand gevonden: { $name }
# { $framework } is a licensing product's name, such as FlexLM / FlexNet.
core-license-detail-store = Licentiearchief van { $framework } aanwezig
# { $name } is an environment variable's name, such as LM_LICENSE_FILE.
core-license-detail-variable = Omgevingsvariabele voor licentie: { $name }
# { $name } is a registry value's name, such as SerialNumber.
core-license-detail-registry-value = Registerwaarde wijst op licentiegegevens: { $name }
# Said of a registry value in an account's key, which is not named.
core-license-detail-registry-value-of-an-account = Registerwaarde wijst op licentiegegevens
# { $key } is a registry key, such as HKLM\Software\Contoso\License.
core-license-detail-registry-key = Registersleutel wijst op licentiegegevens: { $key }

## Warnings a scan records about licenses.

core-warning-scan-license-data = Er zijn licentie- of activeringsgegevens gevonden. Als de optie Licentie- en activeringsgegevens is ingeschakeld, herstelt het installatieprogramma wat kan worden verplaatst. Waarden voor de hele computer en archieven die aan hardware zijn gebonden, worden alleen gemeld. WinAppMigrator ontsleutelt nooit iets, omzeilt nooit activering en emuleert nooit dongles.
core-warning-scan-license-machine-bound = Sommige licenties zijn aan de computer gebonden (hardwaredongle, activering die aan de computer is gekoppeld, of versleuteld). Het installatieprogramma herstelt wat het kan; deze licenties moeten mogelijk nog steeds opnieuw worden geactiveerd op de nieuwe pc.

## Warnings a scan records about the registry. What could not be read is not named in them: it is kept beside them.

# { $error } is what Windows said went wrong. App Paths is a registry key's name.
core-warning-scan-app-paths-key-unreadable = Kan subsleutel van App Paths niet lezen: { $error }
core-warning-scan-registry-limit = Het vastleggen van het register heeft de ingestelde veiligheidslimiet bereikt.
core-warning-scan-registry-subtree-limit = Het vastleggen van een deel van het register heeft de ingestelde veiligheidslimiet bereikt.
# { $error } is what Windows said went wrong.
core-warning-scan-registry-key-unreadable = Kan registersleutel niet lezen: { $error }
core-warning-scan-registry-value-too-large = Registerwaarde is groter dan de ingestelde maximale grootte voor vastleggen.

## Scheduled tasks

core-warning-scan-tasks-folder-through-link = De map met geplande taken op dit station loopt via een koppeling, dus de taken zijn niet gelezen.
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-warning-scan-task-left-out = De geplande taak { $task } is weggelaten: deze wordt uitgevoerd als een ander account dan het account waarvan de instellingen worden opgenomen, anderen dan administrators kunnen het bestand ervan wijzigen, of de app heeft er te veel om te bewaren.
# { $error } is what Windows said went wrong.
core-warning-scan-tasks-folder-unreadable = Kan een deel van de geplande taken niet lezen: { $error }
# { $path } is a task's file, such as C:\Windows\System32\Tasks\App\Nightly Sync; { $reason } is one of the reasons
# below, or what Windows said went wrong.
core-warning-scan-task-unreadable = Kan de geplande taak { $path } niet lezen: { $reason }. Als deze de app uitvoert, is deze niet opgenomen.
# Why a task's file could not be read. { $size } is its size in bytes.
core-task-too-large = het is { $size } bytes, te groot voor een taak
# Why a task's file could not be read.
core-task-ends-mid-character = het eindigt halverwege een teken
# Why a task's file could not be read. { $error } says where.
core-task-not-utf16 = het is geen geldige UTF-16: { $error }
# Why a task's file could not be read. { $error } says where.
core-task-not-text = het is geen geldige UTF-8 of UTF-16: { $error }
# What a package records of how a task is put back. Register-ScheduledTask is a command's name.
core-task-restore-instructions = Bij het herstellen wordt Register-ScheduledTask gebruikt met de gearchiveerde XML-definitie.

## Reading a program's file. Each is why it could not be read, after the program's name.

# { $reason } is what went wrong.
core-pe-unreadable = het kan niet worden gelezen: { $reason }
core-pe-headers-misaligned = de headers staan niet op de grenzen waarop die van een programma moeten staan
# { $size } is the file's size in bytes.
core-pe-too-large = het is { $size } bytes, te groot voor een programma
# PE is the name of the format Windows programs are in.
core-pe-no-headers = het heeft geen PE-headers

## Warnings a scan records about what a program needs. Which program is kept beside each.

# { $error } is what went wrong.
core-warning-scan-imports-unreadable = Importtabel kan niet worden gelezen: { $error }
# { $error } is what went wrong.
core-warning-scan-import-name-unreadable = De naam van een importdescriptor kan niet worden gelezen: { $error }
# { $count } is how many were skipped.
core-warning-scan-legacy-delay-imports =
    { $count ->
        [one] { $count } verouderde (v1) descriptor voor vertraagd laden is overgeslagen; alleen moderne, op RVA gebaseerde vertraagde imports worden geparseerd.
       *[other] { $count } verouderde (v1) descriptors voor vertraagd laden zijn overgeslagen; alleen moderne, op RVA gebaseerde vertraagde imports worden geparseerd.
    }

## The DLLs a program needs. Which DLL or program is kept beside each warning.

# { $path } is a program, such as C:\Program Files\App\app.exe.
core-dll-program-has-no-folder = uitvoerbaar bestand heeft geen bovenliggende map: { $path }
# Why a DLL is among the app's files, in the package's list of them.
core-dll-beside-program = DLL gevonden naast het geselecteerde uitvoerbare bestand
# { $error } is what went wrong.
core-warning-scan-dependency-unscanned = Kan recursieve afhankelijkheid niet scannen: { $error }
# { $name } is a DLL's name, such as d3dx9_43.dll.
core-warning-scan-dependency-not-in-newer-windows = { $name } stond in de Windows-map van de pc waarop deze app stond, en Windows 10 en 11 worden er niet mee geleverd. Iets anders heeft het daar geïnstalleerd, dus het hoort niet bij de eigen bestanden van de app en gaat niet mee. De app start mogelijk pas wanneer is geïnstalleerd wat dit bestand levert.
# { $error } is what went wrong.
core-warning-scan-dependency-not-hashed = Kan geen hash van afhankelijkheid berekenen: { $error }
# SystemRoot and windir are the names of environment variables.
core-warning-scan-windows-folder-unknown = De omgevingsvariabelen SystemRoot/windir zijn niet beschikbaar; het zoeken naar systeem-DLL's is beperkt.
# { $error } is what went wrong.
core-warning-scan-dependency-architecture-unreadable = Kan architectuur van afhankelijkheid niet lezen: { $error }

## Services, drivers, shortcuts, and environment variables a scan reads. Where it was reading is kept beside each
## warning.

# { $error } is what Windows said went wrong.
core-warning-scan-services-unreadable = Kan registersleutel van services niet lezen: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-drivers-unreadable = Kan registersleutel van services niet lezen om stuurprogramma's te zoeken: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-queried = Kan snelkoppelingen niet opvragen: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-parsed = Kan de uitvoer van het opvragen van snelkoppelingen niet parseren: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-environment-unreadable = Kan registersleutel van systeemomgevingsvariabelen niet lezen: { $error }

## What a package records of how a driver is put back, which an installer shows. { $kind } is the driver's type as
## Windows names it (kernel, file-system, recognizer, or unknown-type); { $name } is its service's name; { $path } is
## its file. { $signed } is yes when { $signer } names who signed it, and no when that is not known.

core-service-driver-bundled =
    { $signed ->
        [yes] Stuurprogramma '{ $name }' van de app zelf, type { $kind } ({ $path }, ondertekend door { $signer }). Het installatieprogramma bundelt het .sys-bestand en registreert de stuurprogrammaservice automatisch opnieuw (sc create, vereist verhoogde bevoegdheden), en controleert daarna of de service is geregistreerd.
       *[no] Stuurprogramma '{ $name }' van de app zelf, type { $kind } ({ $path }, handtekening onbekend). Het installatieprogramma bundelt het .sys-bestand en registreert de stuurprogrammaservice automatisch opnieuw (sc create, vereist verhoogde bevoegdheden), en controleert daarna of de service is geregistreerd.
    }
core-service-driver-from-vendor =
    { $signed ->
        [yes] Stuurprogramma '{ $name }' van een leverancier of van Windows, type { $kind } ({ $path }, ondertekend door { $signer }). WinAppMigrator geeft geen stuurprogramma's van derden door. Installeer het opnieuw op de doelcomputer met het ondertekende stuurprogrammapakket van de leverancier voordat je de app start.
       *[no] Stuurprogramma '{ $name }' van een leverancier of van Windows, type { $kind } ({ $path }, handtekening onbekend). WinAppMigrator geeft geen stuurprogramma's van derden door. Installeer het opnieuw op de doelcomputer met het ondertekende stuurprogrammapakket van de leverancier voordat je de app start.
    }

## Runtimes an app needs, such as the Visual C++ runtime or .NET: how each is provided, as the Dependencies list says.

core-runtime-handling-built-in = Ingebouwd in Windows
core-runtime-handling-ships-with-app = Wordt met de app meegeleverd
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-handling-windows-feature = Windows-onderdeel: { $feature }
core-runtime-handling-included = Inbegrepen
# { $id } is the runtime's name in winget, such as Microsoft.VCRedist.2015+.x64.
core-runtime-handling-winget = Installeren met winget: { $id }
core-runtime-handling-manual = Handmatig installeren

## Downloading a runtime's installer, when an installer is built. Each is why it could not be, and starts with a small
## letter, as it follows other words. { $url } is a web address; { $error } is what went wrong.

core-runtime-download-not-https = downloaden van { $url } geweigerd: het is geen HTTPS
# TLS is what keeps a download private.
core-runtime-tls-failed = TLS-initialisatie mislukt: { $error }
core-runtime-download-failed = downloaden van { $url } mislukt: { $error }
core-runtime-download-unreadable = lezen van { $url } mislukt: { $error }
core-runtime-download-empty = downloaden van { $url } heeft geen gegevens opgeleverd
# { $came } and { $expected } are numbers of bytes.
core-runtime-download-cut-short = downloaden van { $url } is afgebroken: { $came } van { $expected } bytes ontvangen

## Installing a runtime: why it was not installed. Each starts with a small letter, as it follows other words.

core-runtime-no-known-installer = er is geen bekend installatieprogramma voor deze runtime
# { $path } is where in the package, such as prerequisites\vc_redist.x64.exe.
core-runtime-payload-misplaced = het pakket noemt { $path } als installatieprogramma van deze runtime, maar daar hoort het niet te staan, dus het is niet uitgevoerd
# { $path } is a file.
core-runtime-payload-missing = meegeleverd installatieprogramma ontbreekt: { $path }
# { $error } is what went wrong.
core-runtime-payload-path-invalid = pad van meegeleverd installatieprogramma is ongeldig: { $error }
# { $url } is the web address it can be downloaded from. winget is a program's name.
core-runtime-no-installer-route = geen meegeleverd installatieprogramma of winget-id; download het via { $url }
# { $error } is what went wrong.
core-runtime-no-temp-folder = kan geen map maken om het in uit te pakken: { $error }
# { $reason } is what the check found.
core-runtime-payload-signature-failed = het meegeleverde installatieprogramma heeft de handtekeningcontrole niet doorstaan en is niet uitgevoerd: { $reason }
core-runtime-payload-timed-out = meegeleverd installatieprogramma is niet binnen de time-out voltooid en is beëindigd
# { $error } is what went wrong.
core-runtime-payload-not-started = kan meegeleverd installatieprogramma niet starten: { $error }
core-runtime-installer-killed = installatieprogramma is beëindigd zonder afsluitcode
# { $code } is the number it ended with, such as 1603.
core-runtime-installer-failed = installatieprogramma is afgesloten met code { $code }
# { $feature } is a Windows feature's name, such as NetFx3; { $code } is the number DISM ended with. Windows Features is
# the name of a Windows window. Keep the command as it is.
core-runtime-feature-failed = DISM kan het Windows-onderdeel { $feature } niet inschakelen (afsluitcode { $code }); de benodigde bestanden konden waarschijnlijk niet worden gedownload. Schakel het in via Windows-onderdelen, of voer `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess` uit.
# The same, when DISM gave no number.
core-runtime-feature-failed-no-code = DISM kan het Windows-onderdeel { $feature } niet inschakelen (afsluitcode onbekend); de benodigde bestanden konden waarschijnlijk niet worden gedownload. Schakel het in via Windows-onderdelen, of voer `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess` uit.
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-feature-timed-out = het inschakelen van het Windows-onderdeel { $feature } is niet binnen de time-out voltooid en is beëindigd
# { $feature } is a Windows feature's name; { $error } is what went wrong.
core-runtime-feature-no-dism = DISM is niet beschikbaar om het Windows-onderdeel { $feature } in te schakelen: { $error }

## Installing a runtime with winget. { $id } is its name in winget, such as Microsoft.VCRedist.2015+.x64.

# { $error } is what went wrong.
core-runtime-winget-not-looked-for = kan niet naar winget zoeken: { $error }
core-runtime-winget-not-installed = winget is niet geïnstalleerd
# { $path } is where winget is.
core-runtime-winget-misplaced = { $path } is niet de plek waar Windows de eigen pakketten bewaart, dus het is niet uitgevoerd
# { $path } is where winget is; { $reason } is what the check found.
core-runtime-winget-signature-failed = { $path } heeft de handtekeningcontrole niet doorstaan en is niet uitgevoerd: { $reason }
# { $reason } is one of the two messages above, or what went wrong.
core-runtime-winget-unusable = kan winget niet gebruiken om { $id } te installeren: { $reason }
# { $code } is the number winget ended with.
core-runtime-winget-failed = winget install { $id } is afgesloten met { $code }
core-runtime-winget-failed-no-code = winget install { $id } is afgesloten met een onbekende code
core-runtime-winget-timed-out = winget install { $id } is niet binnen de time-out voltooid en is beëindigd
# { $error } is what went wrong.
core-runtime-winget-not-started = winget is niet beschikbaar om { $id } te installeren: { $error }

## What told a scan how an app was installed, as the package's report lists it. The names of installers and files are
## as their makers write them.

# { $path } is an installer's record of what it installed, such as C:\Program Files\App\unins000.dat.
core-source-record-not-believed = { $path } is niet gebruikt: anderen dan administrators kunnen het wijzigen
# { $count } is how many Windows Installer packages the bundle holds.
core-source-burn-bundle =
    { $count ->
        [one] Burn-bundel met { $count } Windows Installer-pakket
       *[other] Burn-bundel met { $count } Windows Installer-pakketten
    }
# { $count } is how many paths Windows Installer recorded.
core-source-msi-database =
    { $count ->
        [one] Windows Installer-database: { $count } sleutelpad van onderdeel
       *[other] Windows Installer-database: { $count } sleutelpaden van onderdelen
    }
core-source-inno-log = Verwijderlogboek van Inno Setup (unins000.dat)
# { $count } is how many paths the log records.
core-source-inno-log-paths =
    { $count ->
        [one] Verwijderlogboek van Inno Setup: { $count } vastgelegd pad
       *[other] Verwijderlogboek van Inno Setup: { $count } vastgelegde paden
    }
core-source-nsis = Handtekening van NSIS-verwijderprogramma (Nullsoft)
# { $count } is how many paths the log records.
core-source-nsis-log-paths =
    { $count ->
        [one] NSIS AdvUninstLog uninstall.log: { $count } vastgelegd pad
       *[other] NSIS AdvUninstLog uninstall.log: { $count } vastgelegde paden
    }
core-source-squirrel = Squirrel-indeling per gebruiker (Update.exe / app-*)
# { $version } is a folder's name that says the version, such as app-1.2.3.
core-source-squirrel-version = Huidige versie van Squirrel: { $version }
core-source-chocolatey = Pakketmetagegevens van Chocolatey
core-source-msix = MSIX-/Appx-pakket
# { $identity } is a package's name, such as Contoso.App.
core-source-appx-identity = Identiteit in AppxManifest: { $identity }
# { $count } is how many folders.
core-source-msix-data =
    { $count ->
        [one] MSIX-pakketgegevens per gebruiker: { $count } locatie
       *[other] MSIX-pakketgegevens per gebruiker: { $count } locaties
    }
# App Paths is a registry key's name.
core-source-app-paths = Registratie in App Paths

## Why a runtime is needed, as the package's report says it.

# { $program } is a program's file name, such as app.exe; { $dll } is a DLL's, such as msvcp140.dll.
core-runtime-program-imports = { $program } importeert { $dll }

## What else of an app's a scan finds in the registry: fonts and certificates, and why each is the app's.

# Why a font is among the app's files, in the package's list of them.
core-registry-font-recorded = Een lettertype dat in de installatiegegevens van de app wordt genoemd
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-named-for-the-app = het is genoemd naar de app of de uitgever ervan
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-recorded = de installatiegegevens van de app noemen het

## Who is signed in, for the Whose Data choice. Each of the first six is why it could not be told, and starts with a
## small letter, as it is shown inside other words. An account is shown by its id, such as S-1-5-21-...-1001.

# { $error } is what Windows said.
core-profile-desktop-owner-not-said = Windows geeft niet aan van wie dit bureaublad is: { $error }
# { $error } is what went wrong.
core-profile-desktop-owner-not-asked = Windows kan niet worden gevraagd van wie dit bureaublad is: { $error }
core-profile-signed-in-account-no-profile = het account dat hier is aangemeld, { $account }, heeft geen geregistreerd profiel
core-profile-desktop-not-a-person = het bureaublad hier is van { $account }, en dat is geen account van een persoon
core-profile-nobody-signed-in-running-account-no-profile = er is hier niemand aangemeld, en het account waarmee dit wordt uitgevoerd, { $account }, heeft geen geregistreerd profiel
core-profile-nobody-signed-in-running-not-a-person = er is hier niemand aangemeld, en { $account }, waarmee dit wordt uitgevoerd, is geen account van een persoon
# SID is the name Windows gives an account's id.
core-profile-nobody-signed-in-no-sid = er is hier niemand aangemeld, en dit account heeft geen SID
# { $accounts } is a list of accounts.
core-profile-several-desktops = meer dan één account heeft een bureaublad in deze sessie: { $accounts }
# Under the Whose Data choice. { $reason } is one of the messages above.
core-profile-signed-in-unknown = Kan niet vaststellen wie er is aangemeld ({ $reason }).
# Under the Whose Data choice, when there are accounts to choose from. { $reason } is one of the messages above.
core-profile-signed-in-unknown-choose = Kan niet vaststellen wie er is aangemeld ({ $reason }). Kies wiens gegevens je wilt opnemen.
