# Words the core writes for people as it installs an app from a package: why an install will not start, the list of
# what it did (or for a preview, what it would do) that the installer window shows, its warnings, the record it keeps of
# each install so that it can be undone, and the core's errors. Message names start with core-.
#
# The list of what an install did is short and plain, one line for each thing, as the installer window lists them:
# "copied", "restored", "would copy". Words such as --force and --apply are command-line options, and LocalService,
# SYSTEM, ProgramData, and %AppData% are as Windows names them: keep them as they are.


## Errors: what went wrong, when the core cannot go on. Each is shown on its own, or after words that say what was being
## done.

core-error-list-changed = wat deze installatie zou doen, is gewijzigd sinds ermee is ingestemd, dus er is niets geïnstalleerd
# { $path } is a file or folder, such as C:\Program Files\App\app.exe, or words that name one, such as "the installer".
# { $error } is what Windows said went wrong.
core-error-io = I/O mislukt voor { $path }: { $error }
# { $detail } is what went wrong.
core-error-registry = lezen van register mislukt: { $detail }
# { $path } is a program file. { $detail } is what was wrong with it.
core-error-pe = PE-parsering mislukt voor { $path }: { $detail }
# { $detail } is what was wrong, already in words.
core-error-invalid-input = ongeldige invoer: { $detail }
# { $detail } is what went wrong.
core-error-serialization = serialisatie van rapport mislukt: { $detail }
# { $detail } is what went wrong.
core-error-archive = archiefbewerking mislukt: { $detail }
# { $detail } is what went wrong.
core-error-network = netwerkbewerking mislukt: { $detail }
# { $detail } is what went wrong.
core-error-time-format = tijdnotatie mislukt: { $detail }
core-error-cancelled = het is geannuleerd
# What is said when part of the program stopped unexpectedly and gave no reason.
core-error-panic-no-reason = er is geen reden opgegeven

## The record of each install, kept so that it can be undone.

# { $kind } is a number that says what type of registry value it is.
core-journal-unknown-value-type = onbekend type registerwaarde { $kind }
core-journal-not-elevated = niet met verhoogde bevoegdheden
core-journal-needs-admin = voor installeren zijn administratorrechten nodig, zodat het installatieverslag wordt bewaard waar anderen het niet kunnen wijzigen
# { $path } is a folder.
core-journal-not-an-install-folder = dit is geen map van een installatie: { $path }
# { $path } is the file the record is kept in.
core-journal-not-written = het installatieverslag kan niet worden geschreven: { $path }
# { $path } is a folder.
core-journal-path-escapes-root = pad valt buiten de bedoelde hoofdmap: { $path }
# { $path } is a folder.
core-journal-nothing-exists = niets hiervan bestaat op deze pc: { $path }
core-journal-busy = er wordt al een andere installatie of verwijdering uitgevoerd. Wacht tot die is voltooid en probeer het dan opnieuw
# Said of a file that was to get back the permissions it had.
core-journal-no-folder = het heeft geen map
# Said of the permissions a file had, which were kept to give back to it.
core-journal-permissions-unreadable = wat is bewaard, kan niet worden gelezen
# { $folder } is the folder of one install's record.
core-journal-unreadable-empty = het installatieverslag in { $folder } kan niet worden gebruikt: het is leeg
# { $folder } is the folder of one install's record.
core-journal-unreadable-no-header = het installatieverslag in { $folder } kan niet worden gebruikt: het begint niet met waar het over gaat
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-version = het installatieverslag in { $folder } kan niet worden gebruikt: het is geschreven door een andere versie van dit programma
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-install = het installatieverslag in { $folder } kan niet worden gebruikt: volgens het verslag hoort het bij een andere installatie
# { $id } is an install's id, such as wam-1a2b3c.
core-journal-not-an-install-id = dit is geen id van een installatie: { $id }

## Why an install will not start, or stopped before it changed anything.

core-restore-apply-and-dry-run = --apply en --dry-run kunnen niet allebei worden opgegeven
# { $path } is the folder that was chosen to install in.
core-restore-location-not-full-path = de installatielocatie moet een volledig pad zijn: "{ $path }"
# { $problem } says what is wrong with the folder, already in words. { $path } is the folder.
core-restore-location-unusable = die installatielocatie kan niet worden gebruikt: { $problem }: "{ $path }"
core-restore-needs-admin-plan = het herstelplan bevat wijzigingen in het register, services, geplande taken, snelkoppelingen of beveiligde doelen waarvoor administratorrechten nodig zijn
# { $path } is the folder that was chosen to install in.
core-restore-needs-admin-target = voor de doelmap zijn verhoogde bevoegdheden vereist: { $path }
core-restore-needs-admin-user-data = voor het herstellen van gebruikersgegevens naar ProgramData of andere beveiligde locaties zijn verhoogde bevoegdheden vereist
# { $error } is why the install stopped. { $failed } is what could not be put back, as sentences of their own.
core-restore-not-all-put-back = { $error }. Niet alles wat al was gedaan, kon worden teruggezet: { $failed }
# { $error } is what went wrong.
core-restore-nowhere-to-keep-record = er is geen veilige plek voor het installatieverslag waarmee deze installatie ongedaan kan worden gemaakt, dus er is niets geïnstalleerd: { $error }
# { $path } is a file the app would have put there.
core-restore-already-exists = { $path } bestaat al, dus er is niets geïnstalleerd. Gebruik --force om het te vervangen
# { $root } is a folder the app has files for. { $link } is that folder, or one it is in.
core-restore-root-under-link = deze app heeft items om te herstellen naar { $root }, en { $link } is een koppeling naar een andere plek, die anderen mogelijk kunnen wijzigen, dus er is niets geïnstalleerd. Als wat daar komt gebruikersgegevens of licentiegegevens zijn, installeer de app dan zonder gebruikersgegevens en licentiegegevens. Als het een snelkoppeling in een map Opstarten is, zet die map dan terug waar deze stond
core-restore-hash-check-failed = het pakket heeft de hashcontrole niet doorstaan en er is niets geïnstalleerd
# { $detail } is the first thing found wrong, already in words.
core-restore-hash-check-failed-because = het pakket heeft de hashcontrole niet doorstaan en er is niets geïnstalleerd: { $detail }
# { $path } is a file or folder.
core-restore-refuse-records-folder = schrijven geweigerd op de plek waar installaties hun verslagen bewaren: { $path }
# { $path } is a file or folder.
core-restore-refuse-outside = schrijven buiten de installatielocatie en de bekende gegevensmappen geweigerd: { $path }
# { $path } is a file.
core-restore-no-parent-folder = doel heeft geen bovenliggende map: { $path }
# { $path } is where a file was to go.
core-restore-folder-in-the-way = er staat een map op de plek waar dit bestand moet komen: { $path }
# { $path } is a file in the package.
core-restore-file-changed = { $path } is niet meer wat het pakket bevat, dus het is niet geïnstalleerd
# { $path } is a file in the package.
core-restore-source-is-link = { $path } is een koppeling, en die wordt niet gevolgd
# { $path } is a file in the package.
core-restore-source-became-link = { $path } is een koppeling geworden terwijl het werd geopend
# Why a service did not start, after "did not start:".
core-restore-service-stopped-by-itself = de service is vanzelf weer gestopt
# Why part of the app was left out, after "was left out:".
core-restore-com-class-someone-elses = deze COM-klasse staat al op deze pc, als onderdeel van iets anders

## What an install did, one line for each thing, or for a preview, what it would do. { $source } is a file in the
## package, and { $destination } and { $path } are where it goes on this PC. { $key } is a registry key, such as
## HKLM\Software\App.

# { $package_valid } is true or false, and { $restore_valid } is true, false, or not_run: keep them as they are.
core-restore-verification-result = verificatie na herstel package_valid={ $package_valid } restore_valid={ $restore_valid }
# { $name } is a service's name.
core-restore-started-service = service { $name } gestart
core-restore-would-copy = zou { $source } kopiëren naar { $destination }
core-restore-copied = { $source } gekopieerd naar { $destination }
core-restore-would-create-empty-folder = zou de lege map { $path } maken
core-restore-created-empty-folder = lege map { $path } gemaakt
# { $value } is the name of a value in the key.
core-restore-would-restore-registry-value = zou register { $key } { $value } herstellen
core-restore-would-restore-registry-key = zou register { $key } (sleutel) herstellen
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-would-add-firewall-rule = zou firewallregel { $name } toevoegen voor { $program }
# { $name } is a firewall rule's name.
core-restore-firewall-rule-already-there = firewallregel { $name } bestaat al
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-added-firewall-rule = firewallregel { $name } toegevoegd voor { $program }
# { $name } is a service's name, { $command } what it runs, and { $account } the account it runs as, such as LocalService.
core-restore-would-restore-service = zou service { $name } herstellen met { $command } (wordt uitgevoerd als { $account })
# { $name } is a runtime, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-restore-prerequisite-present = vereiste is al aanwezig: { $name }
# { $name } is a runtime that Windows has as a Windows feature.
core-restore-would-install-prerequisite-feature = zou vereiste { $name } installeren (Windows-onderdeel)
# { $name } is a runtime whose installer is in the package.
core-restore-would-install-prerequisite-bundled = zou vereiste { $name } installeren (meegeleverd installatieprogramma)
# { $name } is a runtime that would come through winget, which is a program's name.
core-restore-would-install-prerequisite-winget = zou vereiste { $name } installeren (winget)
# { $name } is a runtime.
core-restore-installed-prerequisite = vereiste { $name } geïnstalleerd
# { $name } is a runtime.
core-restore-installed-prerequisite-reboot = vereiste { $name } geïnstalleerd (opnieuw opstarten vereist)
core-restore-would-restore-license = zou licentie { $source } herstellen naar { $destination }
core-restore-restored-license = licentie { $source } hersteld naar { $destination }
core-restore-would-restore-license-registry = zou licentiegegevens in register { $key } herstellen
# { $name } is a driver's name.
core-restore-driver-not-restored = stuurprogramma niet automatisch hersteld: { $name }
# { $name } is a driver's name.
core-restore-would-install-driver = zou stuurprogramma { $name } installeren in { $destination }
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-would-restore-task = zou geplande taak { $name } herstellen
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-startup-shortcut = zou opstartsnelkoppeling { $shortcut } naar { $target } herstellen
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-shortcut = zou snelkoppeling { $shortcut } naar { $target } herstellen
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-would-restore-variable =
    { $scope ->
        [user] zou gebruikersvariabele { $name } herstellen
       *[machine] zou systeemvariabele { $name } herstellen
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-left-variable =
    { $scope ->
        [user] gebruikersvariabele { $name } ongewijzigd gelaten: niets ervan hoort bij deze app
       *[machine] systeemvariabele { $name } ongewijzigd gelaten: niets ervan hoort bij deze app
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-restored-variable =
    { $scope ->
        [user] gebruikersvariabele { $name } hersteld
       *[machine] systeemvariabele { $name } hersteld
    }
core-restore-set-uninstaller = verwijderprogramma ingesteld voor { $key }
core-restore-placed-driver-image = stuurprogrammabestand { $source } geplaatst in { $destination }
# { $name } is a driver's name.
core-restore-configured-driver = stuurprogramma { $name } geconfigureerd
# { $name } is a driver's name.
core-restore-created-driver = stuurprogramma { $name } gemaakt
# { $name } is a driver's name.
core-restore-started-driver = stuurprogramma { $name } gestart
core-restore-ensured-registry-key = registersleutel { $key } gemaakt of al aanwezig
# { $value } is the name of a value in the key.
core-restore-restored-registry-value = register { $key } { $value } hersteld
# "(Default)" is how Windows names the value a key has with no name: write it as Registry Editor does.
core-restore-restored-registry-default = register { $key } (Standaard) hersteld
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-configured-service = service { $name } geconfigureerd (wordt uitgevoerd als { $account })
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-created-service = service { $name } gemaakt (wordt uitgevoerd als { $account })
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-restored-task = geplande taak { $name } hersteld
core-restore-restored-shortcut = snelkoppeling { $path } hersteld

## Warnings an install gives, and why it failed. They are shown after it, on the PC it ran on.

core-warning-restore-earlier-not-trusted = Een verslag van een installatie die nooit is voltooid, is ongemoeid gelaten, omdat anderen het kunnen wijzigen. Wat die installatie heeft gewijzigd, is er nog steeds.
# { $app_name } is the app's name. { $count } is how many changes were put back.
core-warning-restore-earlier-undone =
    { $count ->
        [one] Een eerdere installatie van { $app_name } is nooit voltooid, dus die is eerst ongedaan gemaakt. { $count } wijziging is teruggezet.
       *[other] Een eerdere installatie van { $app_name } is nooit voltooid, dus die is eerst ongedaan gemaakt. { $count } wijzigingen zijn teruggezet.
    }
# { $app_name } is the app's name.
core-warning-restore-earlier-never-finished = Een eerdere installatie van { $app_name } is nooit voltooid. Installeren maakt die eerst ongedaan.
core-warning-restore-user-data-real-locations = Gebruikersgegevens worden hersteld naar de echte locaties %AppData%/%ProgramData% van deze computer in plaats van naar de doelmap.
# { $error } is what went wrong.
core-warning-restore-commit-failed = Het installatieverslag kan niet worden voltooid: { $error }
# { $name } is a service's name.
core-warning-restore-service-still-starting = De service { $name } is gevraagd te starten en is nog niet klaar met starten. Dat kan alsnog gebeuren. De service start ook vanzelf wanneer deze pc de volgende keer opstart.
# { $name } is a service's name. { $reason } is why it did not start, as Windows said it.
core-warning-restore-service-not-started = De service { $name } is geïnstalleerd, maar is niet gestart: { $reason }. De app werkt mogelijk pas wanneer de service wel start. Windows probeert het opnieuw wanneer deze pc de volgende keer opstart.
# { $name } is a service's name. { $reason } is why it could not be stopped.
core-warning-restore-service-not-stopped = De service { $name } kan niet eerst worden gestopt: { $reason }. Mogelijk blijft het oude programma actief totdat deze pc de volgende keer opstart.
core-warning-restore-install-cancelled = De installatie is geannuleerd.
core-warning-restore-preview-cancelled = Het voorbeeld is geannuleerd.
# { $item } is what was left out, such as a file or a service's name. { $reason } is why, already in words.
core-warning-restore-left-out = { $item } is weggelaten: { $reason }.
# { $error } is what went wrong.
core-warning-restore-verification-failed = Verificatie na herstel kan niet worden uitgevoerd: { $error }
# { $runner } says what relies on the file: a service, a scheduled task, a firewall rule, a shortcut, or a registry value.
# { $path } is the file. { $name } is the name of what relies on it. { $refusal } is why the file was not replaced,
# already in words.
core-warning-restore-program-would-run =
    { $runner ->
        [service] { $path } mag niet door dit pakket worden vervangen, en de service { $name } is ervan afhankelijk, dus er is niets geïnstalleerd: { $refusal }
        [task] { $path } mag niet door dit pakket worden vervangen, en de geplande taak { $name } is ervan afhankelijk, dus er is niets geïnstalleerd: { $refusal }
        [firewall] { $path } mag niet door dit pakket worden vervangen, en de firewallregel { $name } is ervan afhankelijk, dus er is niets geïnstalleerd: { $refusal }
        [shortcut] { $path } mag niet door dit pakket worden vervangen, en de snelkoppeling { $name } is ervan afhankelijk, dus er is niets geïnstalleerd: { $refusal }
       *[registry] { $path } mag niet door dit pakket worden vervangen, en de registerwaarde { $name } is ervan afhankelijk, dus er is niets geïnstalleerd: { $refusal }
    }
core-warning-restore-destination-exists = Doel bestaat al. Gebruik --force om het te overschrijven.
# { $path } is a file. { $error } is what went wrong.
core-warning-restore-file-not-written = Kan { $path } niet schrijven: { $error }
core-warning-restore-not-a-folder = Op de plek van deze lege map staat iets anders dan een map.
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-not-looked-at = Kan { $path } niet bekijken: { $error }
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-folder-not-made = Kan { $path } niet maken: { $error }
# Why part of the app was left out, after "was left out:".
core-warning-restore-registration-exists = deze pc heeft dit al, ingesteld op iets anders, en een installatie neemt dat niet over
core-warning-restore-registry-review = Registerwaarden zijn hersteld uit de gearchiveerde gestructureerde gegevens, waarbij bekende app-paden zijn herschreven naar de doelmap.
# { $error } is what went wrong.
core-warning-restore-firewall-unreadable = De regels van Windows Firewall kunnen niet worden gelezen: { $error }
# Why a firewall rule was left out, after "was left out:".
core-warning-restore-firewall-rule-exists = er is al een regel met deze naam voor dit programma, ingesteld op iets anders, en een installatie wijzigt die niet
# { $name } is a runtime.
core-warning-restore-prerequisite-unknown = { $name }: er is geen bekend installatieprogramma voor deze runtime.
# { $name } is a runtime.
core-warning-restore-prerequisite-needs-admin = Voor het installeren van { $name } zijn verhoogde bevoegdheden vereist. Voer het installatieprogramma opnieuw uit als administrator.
# { $name } is a runtime.
core-warning-restore-prerequisite-reboot = { $name } is geïnstalleerd, maar de pc moet opnieuw worden opgestart om de installatie te voltooien.
# { $error } is what went wrong.
core-warning-restore-skipped-not-recorded = Kan een overgeslagen runtime niet vastleggen: { $error }
# { $name } is a runtime. { $reason } is why it was skipped, already in words.
core-warning-restore-prerequisite-skipped = { $name }: { $reason }
# { $name } is a runtime. { $reason } is why it could not be installed. { $url } is a web address to get it from.
core-warning-restore-prerequisite-failed = Kan { $name } niet installeren: { $reason }. Download het via { $url }.
# { $error } is what went wrong.
core-warning-restore-runtime-not-recorded = Kan een geïnstalleerde runtime niet vastleggen: { $error }
# { $error } is what went wrong.
core-warning-restore-license-source-invalid = Pad van verpakte licentie is ongeldig: { $error }
core-warning-restore-license-needs-admin = Voor het herstellen van licentiegegevens naar een beveiligde locatie zijn verhoogde bevoegdheden vereist.
core-warning-restore-license-exists = Licentiedoel bestaat al. Gebruik --force om het te overschrijven.
# { $error } is what went wrong.
core-warning-restore-license-failed = Kan licentiegegevens niet herstellen: { $error }
core-warning-restore-driver-needs-admin = Voor het registreren van stuurprogramma's zijn verhoogde bevoegdheden vereist. Voer het installatieprogramma opnieuw uit als administrator.
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-warning-restore-task-runs-as-system = De geplande taak { $name } wordt uitgevoerd als SYSTEM, met volledige rechten op deze pc.
core-warning-restore-shortcut-exists = Snelkoppeling bestaat al. Gebruik --force om deze te overschrijven.
core-warning-restore-shortcut-all-users = Een of meer herstelde snelkoppelingen verwijzen naar een openbaar bureaublad of een gedeelde locatie in het menu Start.
core-warning-restore-environment-refresh = Wijzigingen in omgevingsvariabelen worden mogelijk pas zichtbaar in een nieuw proces, een nieuwe shell of een nieuwe aanmeldingssessie.
# { $name } is a service's name. { $account } is the account it ran as on the PC it came from.
core-warning-restore-service-account = De service { $name } werd op de bron-pc uitgevoerd als { $account }. Het wachtwoord zit niet in het installatieprogramma, dus de service wordt ingesteld als LocalService. Als de service niet werkt, stel je het account en wachtwoord in via Services.
# { $name } is a runtime.
core-warning-restore-prerequisite-pending-reboot = { $name } is nog niet geregistreerd. Na opnieuw opstarten zou het actief moeten zijn.
# { $name } is a runtime.
core-warning-restore-prerequisite-not-verified = { $name } heeft succes gemeld, maar is niet geregistreerd. De installatie is mogelijk mislukt.
# { $error } is what went wrong.
core-warning-restore-driver-source-invalid = Pad van meegeleverd stuurprogramma is ongeldig: { $error }
# { $error } is what went wrong.
core-warning-restore-driver-not-read = Kan de huidige instellingen van het stuurprogramma niet lezen: { $error }
core-warning-restore-driver-exists = Stuurprogrammaservice bestaat al. Gebruik --force om deze opnieuw te configureren.
core-warning-restore-driver-image-exists = Stuurprogrammabestand bestaat al. Gebruik --force om het te overschrijven.
core-warning-restore-driver-hash-bundled = Het meegeleverde stuurprogrammabestand komt niet overeen met de vastgelegde hash, dus het is niet geïnstalleerd.
# { $error } is what went wrong.
core-warning-restore-driver-not-placed = Kan stuurprogrammabestand niet plaatsen: { $error }
core-warning-restore-driver-hash-placed = Het stuurprogrammabestand is gewijzigd nadat het is geplaatst, dus het is niet geregistreerd.
# { $error } is what went wrong.
core-warning-restore-key-not-created = Kan registersleutel niet maken: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-read = Kan de waarde die hiermee zou worden vervangen, niet lezen: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-written = Kan registerwaarde niet schrijven: { $error }
core-warning-restore-value-exists = Registerwaarde bestaat al. Gebruik --force om deze te overschrijven.
# { $error } is what went wrong.
core-warning-restore-environment-key-not-created = Kan registersleutel voor omgevingsvariabelen niet maken: { $error }
core-warning-restore-variable-exists = Omgevingsvariabele bestaat al. Gebruik --force om deze te overschrijven.
# { $error } is what went wrong.
core-warning-restore-variable-not-written = Kan omgevingsvariabele niet schrijven: { $error }
core-warning-restore-value-kind-missing = Type van registerwaarde ontbreekt.
# { $error } is what went wrong.
core-warning-restore-service-not-read = Kan de huidige instellingen van de service niet lezen: { $error }
core-warning-restore-service-exists = Service bestaat al. Gebruik --force om deze opnieuw te configureren.
# { $error } is what went wrong.
core-warning-restore-task-not-queried = Kan geplande taak niet opvragen: { $error }
core-warning-restore-task-exists = Geplande taak bestaat al. Gebruik --force om deze te overschrijven.
# { $error } is what went wrong.
core-warning-restore-task-folder-not-looked-at = Kan de map van de taak niet bekijken: { $error }
# { $error } is what went wrong.
core-warning-restore-shortcut-not-put-back = De snelkoppeling kan niet worden gemaakt en de snelkoppeling die werd vervangen, kan niet worden teruggezet: { $error }
# { $error } is what went wrong.
core-warning-restore-command-not-started = Kan opdracht niet starten: { $error }
# { $output } is what the command wrote, as it wrote it.
core-warning-restore-command-failed = Opdracht mislukt: { $output }
# { $error } is what went wrong.
core-warning-restore-powershell-not-started = Kan PowerShell niet starten: { $error }
