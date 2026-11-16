# Words the core says about what an install may do: why an install location or something in a package is refused,
# the installer's list of what an app sets up beyond its files, the checks of a package and of an install, and whether
# there is room for either. Message names start with core-.
#
# Many messages here are a reason, the second half of a sentence that starts "... was left out:" or "that install
# location cannot be used:", so they start with a small letter and have no full stop.


## Why an install location cannot be used. Shown in red under the Install Location field.

core-review-location-state-root = hier bewaren installaties hun verslagen
core-review-location-has-files = er staan al bestanden in en hier is geen installatie geregistreerd, dus gebruik een lege map
core-policy-location-not-full-path = dit moet een volledig pad zijn
core-policy-location-parent-step = dit mag geen ".." bevatten
core-policy-location-not-plain = dit moet een gewoon pad op een station van deze pc zijn
core-policy-location-whole-drive = dit mag geen volledig station zijn
core-policy-location-windows = dit mag niet de Windows-map zijn
core-policy-location-runs-by-itself = dit is een map van Windows, of een map waarvan de inhoud vanzelf wordt uitgevoerd of geladen
core-policy-location-shared = hier staan andere dingen naast elkaar in, dus de app heeft een map hierbinnen nodig

## Whether an app can run on this PC at all, said before anything is installed.

# { $reason } is why, as the second half of a sentence, such as "App.exe is built for Arm64, and this PC is x64, so
# Windows would not start it".
core-review-cannot-run-here = deze app kan niet op deze pc worden uitgevoerd, dus er is niets gewijzigd: { $reason }
core-policy-architecture-x86 = 32-bits x86
core-policy-architecture-x64 = x64
core-policy-architecture-arm64 = Arm64
core-policy-architecture-arm32 = 32-bits Arm
core-policy-architecture-unknown = een onbekende architectuur
# { $code } is the number Windows gives a kind of processor, such as 0x01c4.
core-policy-machine-type = machinetype { $code }
# { $names } is a list of program names, such as "a.exe, b.exe, c.exe", and { $count } how many more there are.
core-policy-names-and-more = { $names } en nog { $count }
core-policy-fit-programs-not-said = Dit installatieprogramma vermeldt niet wat de programma's nodig hebben, dus er is niet gecontroleerd of ze op deze pc kunnen worden uitgevoerd.
core-policy-fit-architecture-not-told = Er kon niet worden vastgesteld welke architectuur deze pc heeft, dus er is niet gecontroleerd of de programma's van deze app erop kunnen worden uitgevoerd.
core-policy-fit-main-not-said = Dit installatieprogramma vermeldt niet wat het hoofdprogramma nodig heeft, dus wat hier staat, is gebaseerd op de rest van wat het bevat.
# { $driver } is a driver's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-driver = het stuurprogramma { $driver } is gemaakt voor { $built_for } en deze pc is { $here }. Een stuurprogramma moet precies bij Windows passen, dus deze app kan hier niet worden uitgevoerd
# { $program } is a program's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-program = { $program } is gemaakt voor { $built_for } en deze pc is { $here }, dus Windows zou het niet starten
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-none-start = geen van de programma's kan starten op deze pc, die { $here } is: { $programs }
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-some-other-architecture = Sommige programma's van deze app zijn gemaakt voor een andere architectuur en starten niet op deze pc, die { $here } is: { $programs }. De rest van de app wel, dus dit is mogelijk geen probleem.
# { $here } is a kind of processor, such as "Arm64". { $programs } is a list of program names.
core-policy-fit-emulated = Deze pc is { $here }, dus Windows zou deze programma's via emulatie uitvoeren: { $programs }. De meeste programma's werken zo prima, sommige niet.
# { $program } is a program's file name. { $needs } and { $runs } are versions of Windows, such as 10.0 and 6.1.
core-policy-fit-needs-newer = { $program } heeft volgens eigen opgave Windows { $needs } of hoger nodig, en op deze pc draait { $runs }, dus Windows zou het niet starten
# { $programs } is a list of program names.
core-policy-fit-some-need-newer = Sommige programma's van deze app hebben volgens eigen opgave een nieuwere versie van Windows nodig dan deze en starten hier niet: { $programs }.
# { $from_build } and { $this_build } are Windows build numbers, such as 22631, written as Windows writes them.
core-policy-fit-older-build = Deze app stond op Windows-build { $from_build } en op deze pc draait build { $this_build }. Volgens de programma's kunnen ze op deze Windows starten, dus het werkt mogelijk, maar wat de app daar gebruikte, is hier misschien niet aanwezig.
core-policy-fit-build-not-told = Er kon niet worden vastgesteld welke build van Windows dit is, dus deze is niet vergeleken met de build waarop deze app stond.

## Room on a drive for a build or an install.

# Each of these ends the sentence core-space-shortfall, after "free for".
core-space-for-unpacking = het uitpakken van het pakket
core-space-for-install-record = het installatieverslag
core-space-for-app-files = de bestanden van de app
core-space-for-what-is-replaced = het bewaren van wat wordt vervangen
core-space-for-build-work = de werkbestanden van het maken
core-space-for-installer = het installatieprogramma
core-space-for-package = het pakket
core-space-for-unpacking-and-installing = het uitpakken en installeren van het pakket
# What goes between two of those, as in "unpacking the package and the app's files". Keep the quotes: they keep the
# spaces.
core-space-purposes-joiner = { " en " }
# { $drive } is a drive, such as C:, or a folder. { $needed } and { $free } are sizes, such as 1.2 GB. { $purposes } is
# what the room is for, such as "unpacking the package and the app's files".
core-space-shortfall = { $drive } heeft maximaal { $needed } vrije ruimte nodig voor { $purposes } en heeft { $free }
# { $drive } is a drive, such as C:, or a folder. { $needed } is a size, such as 1.2 GB.
core-space-not-told = { $drive } geeft niet aan hoeveel ruimte er is, dus er is niet gecontroleerd of de benodigde { $needed } past
# One drive in a list of where room is needed. { $needed } and { $free } are sizes, and { $drive } a drive, such as C:.
core-space-summary = maximaal { $needed } op { $drive } ({ $free } vrij)
# { $needed } is a size, and { $drive } a drive, such as C:.
core-space-summary-not-told = maximaal { $needed } op { $drive } (hoeveel er vrij is, kon niet worden vastgesteld)
# What goes between two sentences of core-space-shortfall inside core-review-no-room. Keep the quotes: they keep the
# space.
core-review-sentence-joiner = { ". " }
# { $shortfalls } is one or more sentences of core-space-shortfall.
core-review-no-room = er is hiervoor niet genoeg ruimte, dus er is niets gewijzigd. { $shortfalls }. Maak daar ruimte vrij of kies een installatielocatie op een ander station.

## The installer's list of what an app sets up beyond its files: what each kind is called.

core-review-kind-service = Service
core-review-kind-driver = Stuurprogramma
core-review-kind-scheduled-task = Geplande taak
core-review-kind-autostart = Start automatisch
core-review-kind-shortcut = Snelkoppeling
core-review-kind-path = PATH
core-review-kind-environment-variable = Omgevingsvariabele
core-review-kind-shell-extension = Shell-extensie
core-review-kind-menu-command = Menuopdracht
core-review-kind-link-type = Koppelingstype
core-review-kind-file-type = Bestandstype
core-review-kind-com = COM
core-review-kind-firewall-rule = Firewallregel
core-review-kind-event-log-source = Bron voor gebeurtenislogboek
core-review-kind-font = Lettertype
core-review-kind-odbc-driver = ODBC-stuurprogramma
core-review-kind-odbc-source = ODBC-gegevensbron
core-review-kind-compatibility-setting = Compatibiliteitsinstelling
core-review-kind-default-apps = Standaard-apps
core-review-kind-program-name = Programmanaam
core-review-kind-other-settings = Overige instellingen

## The installer's list of what an app sets up: what each line says of it.

# { $file } is a program's file name. { $account } is the Windows account it runs as, such as LocalSystem. { $starts }
# says when it starts, and is not shown itself; for "other", { $start } is one of the core-review-start- words below.
core-review-service-detail =
    { $starts ->
        [automatic] Voert { $file } uit als { $account }, start nu en telkens wanneer deze pc opstart
        [delayed] Voert { $file } uit als { $account }, start nu en kort nadat deze pc is opgestart
        [manual] Voert { $file } uit als { $account }, start alleen wanneer iets erom vraagt
        [disabled] Voert { $file } uit als { $account }, is uitgeschakeld
        [other] Voert { $file } uit als { $account }, start { $start }
       *[unknown] Voert { $file } uit als { $account }
    }
# { $account } is a Windows account, such as LocalService.
core-policy-account-custom = { $account } (aangepast account niet hersteld)
# { $file } is a driver's file name.
core-review-driver-detail = Laadt { $file } in Windows zelf
# { $file } is a driver's file name. { $start } is when Windows starts it: one of the core-review-start- words below.
core-review-driver-detail-starts = Laadt { $file } in Windows zelf, start { $start }
# When a service or driver starts, ending a sentence such as "Loads disk.sys into Windows itself, starts …".
core-review-start-boot = met Windows, vóór al het andere
core-review-start-system = met Windows, tijdens het opstarten
core-review-start-automatic = telkens wanneer deze pc opstart
core-review-start-delayed = telkens kort nadat deze pc is opgestart
core-review-start-manual = alleen wanneer iets erom vraagt
core-review-start-disabled = nooit, omdat het is uitgeschakeld
core-review-start-unknown = op een manier die Windows niet vermeldt
# { $account } is the account a task runs as: an account's name, or core-review-task-runs-as-you, or
# core-review-task-runs-as-creator.
core-review-task-detail = Wordt uitgevoerd als { $account }
# { $account } is the account a task runs as, as for core-review-task-detail.
core-review-task-detail-highest = Wordt uitgevoerd als { $account }, met de hoogst mogelijke rechten
core-review-task-runs-as-you = wie er is aangemeld (jij)
core-review-task-runs-as-creator = de gebruiker die de taak heeft gemaakt
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-autostart-detail =
    Wordt uitgevoerd { $scope ->
        [everyone] voor iedereen die zich aanmeldt
       *[you] wanneer je je aanmeldt
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shortcut-startup-detail =
    Een snelkoppeling in de map Opstarten, { $scope ->
        [everyone] voor iedereen op deze pc
       *[you] voor jou
    }
# { $file } is the file name of what the shortcut opens. { $scope } is whom it is set up for: everyone on this PC,
# or you. It is not shown itself.
core-review-shortcut-detail =
    Opent { $file }, { $scope ->
        [everyone] voor iedereen op deze pc
       *[you] voor jou
    }
# { $name } is an environment variable's name, such as PATH.
core-review-path-title = Voegt toe aan { $name }
core-review-path-install-folder = De installatiemap
# { $path } is a folder inside the install folder, such as bin\tools.
core-review-path-below-install-folder = De installatiemap\{ $path }
# { $value } is the variable's value, as it will be set.
core-review-variable-detail = Ingesteld op { $value }
# { $count } is how many registry values.
core-review-other-settings-title =
    { $count ->
        [one] { $count } instelling buiten de eigen sleutels van de app
       *[other] { $count } instellingen buiten de eigen sleutels van de app
    }
# { $keys } is a list of registry keys, such as HKCU\Software\Vendor.
core-review-other-settings-detail = In { $keys }
# { $keys } is a list of registry keys, and { $count } how many more there are.
core-review-other-settings-detail-more = In { $keys } en nog { $count }
# { $way } says whether the rule allows or blocks, and incoming or outgoing connections; it is not shown itself.
# { $ports } is a list of port numbers, and { $port_count } how many. { $program } is the program's file name.
# { $networks } is a list of the kinds of network Windows names in English (domain, private, public), and
# { $network_count } how many.
# The kinds of network a firewall rule is for, as Windows Defender Firewall names its profiles in your language. They
# fill { $networks } in core-review-firewall-detail, joined into a list.
core-review-network-domain = domein
core-review-network-private = privé
core-review-network-public = openbaar
core-review-firewall-detail =
    { $way ->
        [allow-in] Staat inkomende
        [allow-out] Staat uitgaande
        [block-in] Blokkeert inkomende
       *[block-out] Blokkeert uitgaande
    } { $port_count ->
        [0] verbindingen
       *[other] verbindingen op { $ports }
    }{ $way ->
        [allow-in] { " " }toe
        [allow-out] { " " }toe
        [block-in] { "" }
       *[block-out] { "" }
    } voor { $program }, op { $network_count ->
        [0] elk netwerk
       *[other] netwerken van het type { $networks }
    }
# { $server } is the file name of what Explorer loads. { $scope } is whom it is set up for: everyone on this PC, or
# you. It is not shown itself.
core-review-shell-extension-detail =
    Laadt { $server } in Verkenner, { $scope ->
        [everyone] voor iedereen op deze pc
       *[you] voor jou
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shell-extension-detail-own-file =
    Laadt in Verkenner een bestand dat het installeert, { $scope ->
        [everyone] voor iedereen op deze pc
       *[you] voor jou
    }
# The name of a shell extension: { $handler } is its name in the registry, and { $class } the kind of file or folder it
# is for, such as * or Directory.
core-policy-shell-extension-name = { $handler } voor { $class }
# { $on } is the kind of file or folder whose menu it is, as the registry names it, such as * or
# Directory. { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-menu-command-detail =
    Voegt een opdracht toe aan het menu van { $on }, { $scope ->
        [everyone] voor iedereen op deze pc
       *[you] voor jou
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-default-verb-detail =
    Wijzigt wat er gebeurt wanneer je een van deze opent, { $scope ->
        [everyone] voor iedereen op deze pc
       *[you] voor jou
    }
core-review-client-entry-detail = Kan worden gekozen als programma hiervoor. Er wordt niets aan gewijzigd
core-review-com-permissions-title = COM-machtigingen
core-review-com-permissions-detail = Bepaalt wie de COM-klassen mag starten, of als welk account ze worden uitgevoerd
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-link-type-detail =
    Koppelingen van dit type worden met de app geopend, { $scope ->
        [everyone] voor iedereen op deze pc
       *[you] voor jou
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-file-type-detail =
    Bestanden van dit type worden met de app geopend, { $scope ->
        [everyone] voor iedereen op deze pc
       *[you] voor jou
    }
# { $log } is the name of a Windows event log, such as Application.
core-review-event-log-source-detail = Kan schrijven naar het logboek { $log }, met berichten uit de eigen bestanden
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-font-detail =
    Wordt bij het aanmelden geladen vanuit de installatiemap, { $scope ->
        [everyone] voor iedereen op deze pc
       *[you] voor jou
    }
core-review-odbc-driver-detail = Wordt geladen in programma's die ODBC gebruiken om een database te bereiken
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-odbc-source-detail =
    Een databaseverbinding, { $scope ->
        [everyone] voor iedereen op deze pc
       *[you] voor jou
    }
core-review-default-apps-detail = Kan worden gekozen in Standaard-apps. Er wordt niets aan gewijzigd
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-program-name-detail =
    Uitvoeren en andere programma's kunnen het via deze naam vinden, { $scope ->
        [everyone] voor iedereen op deze pc
       *[you] voor jou
    }
# { $server } is the file name of what serves the classes.
core-review-com-title = COM in { $server }
core-review-com-title-own-file = COM in een bestand dat het installeert
# { $count } is how many registry values.
core-review-com-detail =
    { $count ->
        [one] { $count } instelling van de klassen die andere programma's kunnen starten
       *[other] { $count } instellingen van de klassen die andere programma's kunnen starten
    }
core-review-com-support-title = COM-typebibliotheken, -interfaces en AppID's
# { $libraries }, { $interfaces }, and { $app_ids } are how many registry values there are for its type libraries, its
# interfaces, and its AppIDs.
core-review-com-support-detail = Instellingen voor de typebibliotheken ({ $libraries }), interfaces ({ $interfaces }) en AppID's ({ $app_ids }), die bij de klassen horen
core-review-compatibility-title = Hoe de programma's starten
# { $count } is how many registry values.
core-review-compatibility-detail =
    { $count ->
        [one] { $count } compatibiliteitsinstelling voor programma's die het installeert
       *[other] { $count } compatibiliteitsinstellingen voor programma's die het installeert
    }

## The installer's notes on whose folders and settings these are, and what is not coming.

# { $account } is the name of a Windows account, such as alice.
core-review-account-signed-in = Gebruikersgegevens en instellingen per gebruiker gaan naar { $account }, die hier is aangemeld, en niet naar het administratoraccount waarmee dit wordt uitgevoerd.
# { $account } is the name of a Windows account, such as alice.
core-review-account-no-desktop = Er is niemand aangemeld bij deze sessie, dus gebruikersgegevens en instellingen per gebruiker gaan naar { $account }, het account waarmee dit wordt uitgevoerd.
# { $reason } is what Windows said, in its own words.
core-review-account-not-told = Kan niet vaststellen wie er is aangemeld ({ $reason }), dus instellingen per gebruiker worden weggelaten en gebruikersgegevens komen in de installatielocatie terecht.
# { $reason } is what Windows said, in its own words.
core-review-signed-in-unknown = kan niet vaststellen wie er is aangemeld: { $reason }
# Headings of what is left out, shared by the app's list and the installer's notes.
core-review-left-out-credentials = Niet in dit installatieprogramma, omdat ze op referenties lijken
core-review-left-out-settings = Instellingen en geschiedenis die een account voor de app bijhoudt, die alleen meegaan met de optie Gebruikersgegevens
core-review-left-out-certificates = De app lijkt afhankelijk van certificaten die op de oorspronkelijke pc werden vertrouwd, en die worden niet geïnstalleerd
core-review-left-out-fonts = Deze lettertypen hebben de naam van de app, maar niets geeft aan dat ze van de app zijn, dus ze worden niet geïnstalleerd
# { $names } is a list of names, such as the names of saved passwords.
core-review-note-credentials = Niet in dit installatieprogramma, omdat ze op referenties lijken: { $names }. Wat hiervan op deze pc staat, blijft zoals het is, en je moet ze mogelijk opnieuw invoeren.
# { $names } is a list of names of license data.
core-review-note-licenses = Niet in dit installatieprogramma, omdat de optie Licentie- en activeringsgegevens was uitgeschakeld toen het werd gemaakt: { $names }.
# { $names } is a list of places an account keeps settings, such as folders.
core-review-note-settings = Niet in dit installatieprogramma, omdat de optie Gebruikersgegevens was uitgeschakeld toen het werd gemaakt: de instellingen en geschiedenis die een account voor de app bijhoudt, in { $names }. De app start hier zoals voor iemand die de app nog nooit heeft gebruikt.
# { $names } is a list of certificates, each as core-review-certificate.
core-review-note-certificates = De app lijkt afhankelijk van certificaten die op de oorspronkelijke pc werden vertrouwd, en die worden niet geïnstalleerd: { $names }. Of je ze hier vertrouwt, bepaal je zelf.
# { $names } is a list of font names.
core-review-note-fonts = Deze lettertypen hebben de naam van de app, maar niets geeft aan dat ze van de app zijn, dus ze worden niet geïnstalleerd: { $names }.
# { $names } is a list of names.
core-review-note-left-out-by-choice = Weggelaten door wie dit installatieprogramma heeft gemaakt: { $names }.
# { $names } is a list of names.
core-review-note-added-by-choice = Toegevoegd door wie dit installatieprogramma heeft gemaakt, omdat die het als onderdeel van de app beschouwde: { $names }.
# { $name } is a certificate's subject, such as CN=Contoso, and { $store } the certificate store it was in, such as
# Root.
core-review-certificate = { $name } in { $store }
# { $names } is a list of names, and { $count } how many more there are.
core-review-names-and-more = { $names } en nog { $count }

## Why something in a package is left out. Each is the second half of a sentence of core-warning-review-left-out.

core-review-no-account = het hoort bij de instellingen van één account, en er kon niet worden vastgesteld wie er is aangemeld
core-review-name-not-accepted = Windows accepteert de naam niet
# { $error } is what Windows said, in its own words.
core-review-service-unreadable = de huidige instellingen kunnen niet worden gelezen: { $error }
core-review-service-dependency = het is afhankelijk van een service met een ongeldige naam
# { $path } is the program the service runs.
core-review-service-runs-other = het voert { $path } uit, en dat installeert dit pakket niet
core-review-service-settings = de instellingen noemen iets wat dit pakket niet installeert, of een sleutel die niet van de service zelf is
core-review-driver-no-hash = er is geen hash om het mee te controleren
core-review-license-for-the-machine = een licentie die voor de hele computer is opgeslagen, wordt alleen gemeld
core-review-license-starts-something = een licentie heeft geen reden om iets te starten
core-review-task-not-captured = de definitie is niet vastgelegd
# { $error } is what was wrong with the task's XML, in English.
core-review-task-unreadable = de definitie kan niet worden gelezen: { $error }
core-review-task-no-account = het wordt uitgevoerd als wie het installeert, en er kon niet worden vastgesteld wie hier is aangemeld
# { $path } is what the shortcut opens.
core-review-shortcut-opens-other = het opent { $path }, en dat installeert dit pakket niet
# { $service } is the service's name, and { $path } the program it runs now.
core-policy-service-exists = { $service } bestaat al op deze pc en voert { $path } uit, en dat komt niet van een eerdere installatie hier. Als het een oudere kopie van deze app is, verwijder je die eerst
core-policy-key-control-characters = de sleutelnaam bevat besturingstekens
core-policy-value-control-characters = de naam van de waarde bevat besturingstekens
core-policy-registry-link = het is een registerkoppeling
# { $reason } is why the key it stands for is left out, such as "no app install should change this key".
core-policy-virtual-store = het staat voor een sleutel van de pc, en { $reason }
core-policy-registry-part = geen enkele app-installatie hoort naar dit deel van het register te schrijven
core-policy-key-refused = geen enkele app-installatie hoort deze sleutel te wijzigen
core-policy-net-debugger = het zou een debugger instellen voor elk .NET-programma
core-policy-net-install-root = het zou wijzigen waar elk .NET-programma .NET vandaan laadt
core-policy-class-redirects = het zou wijzigen hoe een soort programma of map wordt geopend
core-policy-app-path-name = het noemt een programma met iets anders dan een bestandsnaam
core-policy-app-path-not-installed = het zou een programmanaam registreren voor een bestand dat dit pakket niet installeert
core-policy-shared-default-verb = het zou wijzigen wat er gebeurt wanneer elk bestand of elke map van een bepaald soort wordt geopend
core-policy-command-runs-other = het voegt een opdracht toe die iets uitvoert wat dit pakket niet installeert
core-policy-autostart-not-command = het start automatisch, en de waarde is geen opdracht
core-policy-autostart-runs-other = het start automatisch, en voert iets uit wat dit pakket niet installeert
core-policy-environment-own-rules = omgevingsvariabelen worden volgens hun eigen regels hersteld
core-policy-service-not-restored = het hoort bij een service die dit pakket niet herstelt
core-policy-service-own-rules = de eigen instellingen van een service worden alleen via de regels voor services hersteld
core-policy-service-dll = de ServiceDll is geen bestand dat dit pakket installeert
core-policy-event-log-source-only = alleen de eigen waarden van een bron voor gebeurtenislogboek kunnen worden hersteld
core-policy-security-log = geen enkele app schrijft naar het logboek Beveiliging
core-policy-event-log-messages = de berichten zouden afkomstig zijn uit een bestand dat dit pakket niet installeert
core-policy-type-library = de typebibliotheek is geen bestand dat dit pakket installeert
# { $name } is a program's file name, such as app.exe.
core-policy-application-opened = het zou wijzigen hoe { $name } wordt geopend, en dat installeert dit pakket niet
core-policy-event-log-no-file = een bron voor gebeurtenislogboek moet de berichten uit een bestand halen
core-policy-com-runs-other = het voert iets uit wat geen bestand is dat dit pakket installeert
core-policy-com-redirect = het zou van deze klasse iets maken wat dit pakket niet installeert
core-policy-com-server = het voert iets uit wat geen bestand is dat dit pakket installeert, en ook geen onderdeel van Windows
core-policy-interface = het zou bepalen hoe aanroepen naar een interface worden gedaan, met iets wat dit pakket niet installeert
# { $name } is a program's file name, such as app.exe.
core-policy-app-id-started = het zou wijzigen hoe { $name } wordt gestart, en dat installeert dit pakket niet
core-policy-app-id-surrogate = het zou worden gehost door een programma dat dit pakket niet installeert
core-policy-app-id-service = het zou worden uitgevoerd in een service die dit pakket niet herstelt
core-policy-app-id-run-as = het zou worden uitgevoerd als een eigen account
core-policy-app-id-link = het noemt geen AppID
core-policy-app-id-remote = het zou ergens anders dan op deze pc worden uitgevoerd
core-policy-odbc-own-driver = dat is een eigen instelling van ODBC en geen stuurprogramma
core-policy-odbc-driver-not-installed = het stuurprogramma is geen bestand dat dit pakket installeert
core-policy-loads-other-file = het zou een bestand laden dat dit pakket niet installeert
core-policy-odbc-own-source = dat is een eigen instelling van ODBC en geen gegevensbron
core-policy-odbc-source-missing = het vermeldt een gegevensbron die dit pakket niet heeft
core-policy-odbc-source-driver = het stuurprogramma is geen bestand dat dit pakket installeert, en ook geen onderdeel van Windows
core-policy-shell-extension = het zou in Verkenner een shell-extensie laden die geen bestand is dat dit pakket installeert
core-policy-not-text = wat het bevat, is geen tekst, terwijl het als tekst zou worden gelezen
core-policy-network-place = het noemt een locatie in het netwerk
core-policy-font = het zou een lettertype registreren dat dit pakket niet installeert, of een lettertype in de Windows-map
core-policy-compatibility = het zou wijzigen hoe een programma wordt uitgevoerd dat dit pakket niet installeert
core-policy-capabilities = het vermeldt een app waarvan de gegevens niet in dit pakket staan
core-policy-default-program = het zou wijzigen welk programma dingen standaard opent
core-policy-url-protocol = het geeft aan een koppelingstype te zijn, maar opent met niets wat dit pakket installeert
core-policy-command-handler = het voegt een opdracht toe die wordt afgehandeld door iets wat dit pakket niet installeert
core-policy-installer-command = het zou iets uitvoeren wat alleen Windows Installer kan lezen
core-policy-creates-class = het zou een klasse maken die dit pakket niet installeert
core-policy-opens-links = het opent koppelingen met iets wat dit pakket niet installeert
core-policy-opens-files = het opent bestanden met iets wat dit pakket niet installeert
core-policy-firewall-name = de naam kan niet worden doorgegeven
core-policy-firewall-description = de beschrijving kan niet worden doorgegeven
core-policy-firewall-every-program = het geldt voor elk programma, en een regel kan maar voor één programma gelden
# { $path } is the program the rule is for.
core-policy-firewall-program = het is voor { $path }, en dat installeert dit pakket niet
core-policy-firewall-direction = het vermeldt geen richting die kan worden ingesteld
core-policy-firewall-action = het zou andere regels overschrijven, en de regel van een app hoort dat niet te doen
# { $setting } is a part of a firewall rule, as the scan of its PC described it.
core-policy-firewall-not-carried = het vermeldt { $setting }, en dat kan niet zo worden ingesteld, of zou verzwakken wat andere regels doen
core-policy-firewall-service = het is voor een service die dit pakket niet herstelt
core-policy-firewall-parts = een deel ervan is niet wat een firewallregel kan bevatten
core-policy-folder-already-there = die map bestond al, buiten de installatielocatie, en niets geeft aan dat deze van deze app is, dus er komt geen programma in
core-policy-variable-name = de naam is geen geldige variabelenaam
core-policy-variable-relied-on = Windows en andere programma's zijn afhankelijk van deze variabele
core-policy-task-name = de naam wordt niet geaccepteerd door Windows
core-policy-task-folder = de map wordt niet geaccepteerd door Windows
core-policy-task-windows = het zou een van de eigen taken van Windows vervangen
core-policy-task-other-action = het doet iets anders dan een programma uitvoeren
core-policy-task-no-program = het noemt geen programma om uit te voeren
# { $path } is the program the task runs.
core-policy-task-runs-other = het voert iets uit wat dit pakket niet installeert: { $path }
core-policy-task-exists-unreadable = er bestaat al een taak met deze naam, en die kan niet worden gelezen
core-policy-task-exists-other = er bestaat al een taak met deze naam, en die voert de app niet uit
# { $path } is the program the task already on this PC runs.
core-policy-task-exists-runs = er bestaat al een taak met deze naam, en die voert { $path } uit, wat niet van een eerdere installatie hier komt
core-policy-shortcut-not-lnk = het is geen .lnk-snelkoppeling
core-policy-shortcut-place = het staat niet in een menu Start of op een bureaublad
core-policy-short-name = het pad gebruikt een naam die niet kan worden gecontroleerd
core-policy-shortcut-no-folder = deze pc heeft niet de map waarin het moet komen
core-policy-data-startup = het zou in een map Opstarten komen en bij elke aanmelding worden uitgevoerd
core-policy-data-powershell = het zou op een plek komen waar PowerShell bij het starten uitvoert wat het er vindt
core-policy-data-windows = het zou in een map komen die van Windows is
core-policy-license-outside = het staat buiten de installatielocatie en buiten elke bekende gegevensmap
core-rewrite-cdata = het bevat een CDATA-sectie
core-rewrite-too-much-to-look-through = dit pakket bevat te veel in de instellingen om te controleren wat ze noemen

## Warnings a review gives, which the installer shows with what it sets up.

# { $kind } says what was left out, and is not shown itself. { $item } is its path or name, or a registry value such
# as HKLM\Software\App\Name. { $reason } is why, as the second half of the sentence, such as "it cannot be a whole
# drive".
core-warning-review-left-out =
    { $kind ->
        [file] Het bestand { $item } is weggelaten: { $reason }.
        [folder] De map { $item } is weggelaten: { $reason }.
        [service] De service { $item } is weggelaten: { $reason }.
        [driver] Het stuurprogramma { $item } is weggelaten: { $reason }.
        [firewall-rule] De firewallregel { $item } is weggelaten: { $reason }.
        [license-data] De licentiegegevens op { $item } zijn weggelaten: { $reason }.
        [license-value] De licentiewaarde { $item } is weggelaten: { $reason }.
        [scheduled-task] De geplande taak { $item } is weggelaten: { $reason }.
        [shortcut] De snelkoppeling { $item } is weggelaten: { $reason }.
        [environment-variable] De omgevingsvariabele { $item } is weggelaten: { $reason }.
       *[value] { $item } is weggelaten: { $reason }.
    }
core-warning-review-package-path-missing = Een kopieerbaar bestand heeft geen relatief pad in het pakket.
# { $count } is the most empty folders a package makes.
core-warning-review-too-many-empty-folders = Dit pakket vermeldt meer dan { $count } lege mappen, en er worden er maar zoveel gemaakt.
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Documents folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-documents-elsewhere =
    { $count ->
        [one] { $count } bestand uit Documenten komt in plaats daarvan onder { $target }, in een map met de naam _external, omdat de map Documenten van deze pc op { $folder } staat en er alleen naar een map binnen je profiel wordt geschreven. Als je het wilt gebruiken, verplaats je het zelf naar je map Documenten.
       *[other] { $count } bestanden uit Documenten komen in plaats daarvan onder { $target }, in een map met de naam _external, omdat de map Documenten van deze pc op { $folder } staat en er alleen naar een map binnen je profiel wordt geschreven. Als je ze wilt gebruiken, verplaats je ze zelf naar je map Documenten.
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-documents-unknown =
    { $count ->
        [one] { $count } bestand uit Documenten komt in plaats daarvan onder { $target }, in een map met de naam _external, omdat niet kon worden vastgesteld waar de map Documenten van deze pc staat. Als je het wilt gebruiken, verplaats je het zelf naar je map Documenten.
       *[other] { $count } bestanden uit Documenten komen in plaats daarvan onder { $target }, in een map met de naam _external, omdat niet kon worden vastgesteld waar de map Documenten van deze pc staat. Als je ze wilt gebruiken, verplaats je ze zelf naar je map Documenten.
    }
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Saved Games folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-elsewhere =
    { $count ->
        [one] { $count } bestand uit Opgeslagen spellen komt in plaats daarvan onder { $target }, in een map met de naam _external, omdat de map Opgeslagen spellen van deze pc op { $folder } staat en er alleen naar een map binnen je profiel wordt geschreven. Als je het wilt gebruiken, verplaats je het zelf naar je map Opgeslagen spellen.
       *[other] { $count } bestanden uit Opgeslagen spellen komen in plaats daarvan onder { $target }, in een map met de naam _external, omdat de map Opgeslagen spellen van deze pc op { $folder } staat en er alleen naar een map binnen je profiel wordt geschreven. Als je ze wilt gebruiken, verplaats je ze zelf naar je map Opgeslagen spellen.
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-unknown =
    { $count ->
        [one] { $count } bestand uit Opgeslagen spellen komt in plaats daarvan onder { $target }, in een map met de naam _external, omdat niet kon worden vastgesteld waar de map Opgeslagen spellen van deze pc staat. Als je het wilt gebruiken, verplaats je het zelf naar je map Opgeslagen spellen.
       *[other] { $count } bestanden uit Opgeslagen spellen komen in plaats daarvan onder { $target }, in een map met de naam _external, omdat niet kon worden vastgesteld waar de map Opgeslagen spellen van deze pc staat. Als je ze wilt gebruiken, verplaats je ze zelf naar je map Opgeslagen spellen.
    }

## Errors when a folder that only administrators may change cannot be made so, or checked.

# { $path } is a folder.
core-privilege-not-a-folder = { $path } is een koppeling of een bestand, geen map
# { $path } is a folder. { $error } is what Windows said, in its own words.
core-privilege-not-made-here = { $path } is niet door WinAppMigrator gemaakt, en anderen kunnen wijzigen wat erin staat. Een administrator moet de map controleren en verwijderen: { $error }
# { $path } is a folder.
core-privilege-others-put-something-in = { $path } bevat iets wat niet door WinAppMigrator is gemaakt, en anderen kunnen het wijzigen. Een administrator moet de map controleren en verwijderen
# { $path } is a folder.
core-privilege-became-a-link-in-use = { $path } is een koppeling geworden terwijl de map werd gebruikt
# { $path } is a folder.
core-privilege-became-a-link-looked-at = { $path } is een koppeling geworden terwijl de map werd bekeken
# { $path } is a file. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-users-cannot-start = kan gebruikers niet toestaan { $path } te starten: { $said }
core-privilege-elevation-unknown = kan niet vaststellen of dit proces verhoogde bevoegdheden heeft
# { $path } is a folder. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-cannot-restrict = kan { $path } niet beperken tot administrators: { $said }
# { $path } is a folder. { $said } is the folder's permissions as Windows wrote them, and what else it printed.
core-privilege-not-restricted = { $path } kan niet worden beperkt tot administrators, en het station bewaart mogelijk geen machtigingen: { $said }
# { $path } is a folder.
core-privilege-written-to-first = een ander proces heeft naar { $path } geschreven voordat de map kon worden beveiligd
core-privilege-no-program-data = Windows geeft niet aan waar ProgramData staat

## Checks of a package's files, and of what an install put in place. Each is said of one file, value, service, or the
## like, which the report names beside it.

# { $path } is the installer's file.
core-verify-no-package = { $path } bevat geen pakket
# { $path } is a folder.
core-verify-not-unpacked = het pakket van dit installatieprogramma is niet uitgepakt naar { $path }
core-verify-schema-supported = Schema van pakketmanifest wordt ondersteund.
core-verify-packaged-file-missing = Verpakt bestand ontbreekt.
core-verify-no-package-path = Kopieerbaar bestand heeft geen relatief pad in het pakket.
core-verify-folder-there = Lege map is aanwezig.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-unreadable = Kan het niet lezen om het te controleren: { $error }
core-verify-restored-file-missing = Hersteld bestand ontbreekt.
# { $expected } and { $actual } are sizes in bytes.
core-verify-size-mismatch = Bestandsgrootte komt niet overeen. Verwacht: { $expected } bytes, gevonden: { $actual } bytes.
core-verify-no-hash = Het manifest bevat geen hash voor dit bestand, dus het kan niet worden gecontroleerd.
core-verify-hash-mismatch = SHA-256-hash komt niet overeen.
core-verify-file-matches = Bestandsgrootte en hash komen overeen met het manifest.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-payload-failed = Verificatie van verpakte licentie mislukt: { $error }
core-verify-license-payload-missing = Verpakte licentie ontbreekt.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-failed = Verificatie van licentiearchief mislukt: { $error }
core-verify-license-store-file-missing = Verpakt bestand van licentiearchief ontbreekt.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-path-invalid = Pad van verpakt licentiearchief is ongeldig: { $error }
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-prerequisite-failed = Verificatie van meegeleverde vereiste mislukt: { $error }
core-verify-prerequisite-missing = Meegeleverde vereiste ontbreekt.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-failed = Verificatie van stuurprogrammabestand mislukt: { $error }
core-verify-bundled-driver-missing = Meegeleverd stuurprogrammabestand ontbreekt.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-path-invalid = Pad van meegeleverd stuurprogramma is ongeldig: { $error }
core-verify-prerequisite-present = Vereiste runtime is aanwezig op de doelcomputer.
core-verify-prerequisite-not-detected = Vereiste runtime niet gevonden. Mogelijk is opnieuw opstarten of handmatige voltooiing nodig (zie het herstellogboek voor de definitieve uitkomst).
core-verify-prerequisite-not-probed = Of deze vereiste aanwezig is, kan niet in het register worden gecontroleerd. De installatie is naar beste vermogen uitgevoerd.
core-verify-license-file-missing = Hersteld licentiebestand ontbreekt.
core-verify-license-store-restored-missing = Hersteld bestand van licentiearchief ontbreekt.
core-verify-registry-key-there = Registersleutel bestaat.
core-verify-registry-value-matches = Registerwaarde komt overeen met gearchiveerde gegevens.
core-verify-registry-value-differs = Registerwaarde komt niet overeen met gearchiveerde gegevens.
core-verify-firewall-rule-there = Windows Firewall heeft deze regel.
core-verify-variable-matches = Waarde van omgevingsvariabele komt overeen met gearchiveerde gegevens.
core-verify-variable-differs = Waarde van omgevingsvariabele komt niet overeen met gearchiveerde gegevens.
# { $error } is what Windows said, in its own words.
core-verify-variable-unreadable = Omgevingsvariabele ontbreekt of is onleesbaar: { $error }
core-verify-service-runs-other = Service bestaat, maar voert het herstelde programma niet uit met de bijbehorende argumenten.
core-verify-service-other-account = Service bestaat, maar wordt niet uitgevoerd als het verwachte account.
core-verify-service-matches = Service bestaat en voert het herstelde programma uit met de bijbehorende argumenten en het juiste account.
core-verify-service-missing = Service ontbreekt.
# { $error } is what Windows said, in its own words.
core-verify-service-unreadable = Service is onleesbaar: { $error }
core-verify-driver-other-image = Stuurprogrammaservice verwijst niet naar het herstelde stuurprogrammabestand.
core-verify-driver-not-registered = Meegeleverde stuurprogrammaservice is na herstel niet geregistreerd.
# { $error } is what Windows said, in its own words.
core-verify-driver-unreadable = Stuurprogrammaservice is onleesbaar: { $error }
core-verify-driver-image-missing = Stuurprogrammabestand ontbreekt.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-unchecked = Stuurprogrammabestand kan niet worden gecontroleerd: { $error }
core-verify-task-matches = Geplande taak bestaat en voert de herstelde programma's uit.
core-verify-task-runs-other = Geplande taak bestaat, maar voert de herstelde programma's niet uit.
core-verify-task-missing = Geplande taak ontbreekt.
# { $error } is what Windows said, in its own words.
core-verify-task-unreadable = Opvragen van geplande taak mislukt: { $error }
core-verify-shortcut-matches = Snelkoppeling bestaat en het doelpad komt overeen met het herstelde doel.
core-verify-shortcut-opens-other = Snelkoppeling bestaat, maar het doelpad komt niet overeen met het herstelde doel.
core-verify-shortcut-missing = Snelkoppeling ontbreekt.
# { $error } is what Windows said, in its own words.
core-verify-shortcut-unreadable = Opvragen van snelkoppeling mislukt: { $error }

## Warnings and errors a check gives, which the app and the installer show.

core-warning-verify-schema-unsupported = Schema van pakketmanifest wordt niet ondersteund.
# { $error } is what went wrong, in Windows' or the program's own words.
core-warning-verify-package-file-failed = Verificatie van pakketbestand mislukt: { $error }
core-warning-verify-folder-missing = Lege map ontbreekt.
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-key-missing = Registersleutel ontbreekt of is onleesbaar: { $error }
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-value-missing = Registerwaarde ontbreekt of is onleesbaar: { $error }
core-warning-verify-registry-kind-missing = Type van registerwaarde ontbreekt.
core-warning-verify-firewall-rule-missing = Windows Firewall heeft deze regel niet.
core-warning-verify-service-scope = Bij de verificatie van services wordt de serviceconfiguratie in HKLM gelezen en worden geen services gestart.
core-warning-verify-driver-manual = Een of meer stuurprogramma's van een leverancier of van Windows worden niet door WinAppMigrator doorgegeven. Installeer ze opnieuw op de doelcomputer met het ondertekende stuurprogrammapakket van de leverancier.
core-warning-verify-task-scope = Bij de verificatie van geplande taken worden alleen de registratie en de verwijzing naar de opdracht gecontroleerd.

## The verification report written beside a package, verification_report.md. Its headings follow a "#".

core-verify-report-title = Verificatierapport van WinAppMigrator
# { $time } is when, as a date and time in UTC, such as 2026-11-15T10:00:00Z.
core-verify-report-verified = Geverifieerd (UTC): { $time }
# { $valid } is true or false, which are not translated.
core-verify-report-package-valid = Pakket geldig: { $valid }
# { $valid } is true or false, which are not translated.
core-verify-report-restore-valid = Herstel geldig: { $valid }
core-verify-report-restore-not-run = Herstel geldig: niet uitgevoerd
core-verify-report-items = Items
# { $message } is what a check found, and { $source } the file or setting it looked at.
core-verify-report-item-from = { $message } Bron: { $source }
core-verify-report-warnings = Waarschuwingen
core-verify-report-errors = Fouten
core-verify-report-none = Geen.
