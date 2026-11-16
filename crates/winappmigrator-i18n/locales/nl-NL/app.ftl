# Words of the app, WinAppMigrator itself: its one window, with the installed apps listed on the left and, on the
# right, what goes in an installer of the one picked and the button that builds it; and the steps to read another PC
# through WinAppMigrator Client. Message names start with app-.


## The window and its header

# The program's name, in the window's title bar and at the top of the window. Keep it as it is unless your language
# writes product names in a script of its own.
app-name = WinAppMigrator
app-tagline = Maak van een geïnstalleerde app een zelfuitpakkend installatieprogramma
app-theme-to-light = Overschakelen naar licht thema
app-theme-to-dark = Overschakelen naar donker thema
app-reload-list = De lijst met apps opnieuw laden
app-list-loading = Laden
# { $count } is how many apps are listed.
app-list-count =
    { $count ->
        [one] { $count } app
       *[other] { $count } apps
    }
app-list-load-failed = Laden mislukt
# Shown when the window cannot open at all. { $error } is what went wrong, in the words of Windows or a graphics driver.
app-window-could-not-open = Het venster kan niet worden geopend: { $error }

## The list of installed apps

app-list-search = Zoeken op naam of uitgever  (Ctrl+F)
app-list-filter-all = Alle
# Whether an app is installed for one account or for everyone on the PC.
app-scope-per-user = Per gebruiker
app-scope-per-machine = Per computer
app-scope-unknown = Onbekend
# An app whose processor architecture (x86, x64, ARM64) is not known.
app-arch-unknown = Onbekend
app-list-loading-apps = Geïnstalleerde apps laden…
# { $error } is why the apps could not be listed.
app-list-could-not-list = Kan de apps niet weergeven: { $error }
app-list-no-matches = Geen overeenkomende apps
app-list-no-matches-hint = Pas de zoektekst of het bereikfilter aan.
app-list-worker-disconnected = De verbinding met het inventarisatieproces is verbroken.
# What a screen reader says of an app's row. { $name } is the app's name, { $publisher } who made it, and { $version } its
# version, such as 11.4.2, or — when either is not known.
app-list-row = { $name }, { $publisher }, versie { $version }

## The list's columns, and what a screen reader says each column's heading does when clicked

app-column-name = Naam
app-column-name-sort = Sorteren op naam
app-column-publisher = Uitgever
app-column-publisher-sort = Sorteren op uitgever
app-column-version = Versie
app-column-version-sort = Sorteren op versie
app-column-scope = Bereik
app-column-scope-sort = Sorteren op bereik
# Short for architecture: x86, x64, or ARM64. The column is narrow.
app-column-arch = Arch.
app-column-arch-sort = Sorteren op architectuur

## Where apps are read from: this PC, a drive from another PC, a disk image of one, or another PC on the network

app-source = Scanbron
app-source-this-pc = Deze pc
app-source-drive = Aangesloten station…
app-source-image = Schijfkopie…
app-source-network = Netwerk-pc…
app-source-attaching-image = De schijfkopie koppelen, alleen-lezen…
# { $computer } is the other PC's name, such as OLD-PC, and { $address } its network address, such as 192.168.1.20.
app-source-network-pc = { $computer } via het netwerk ({ $address })
app-source-disconnect = Verbinding verbreken
app-source-change = Wijzigen…
# The title of the window that picks a disk image. .vhd and .vhdx are file extensions.
app-source-pick-image-title = Een schijfkopie van de andere pc selecteren (.vhd of .vhdx)
app-source-pick-image-filter = Schijfkopieën
# { $path } is the disk image file.
app-source-image-has-no-windows = { $path } heeft geen volume met Windows erop.
app-source-image-not-attached = De schijfkopie kan niet worden gekoppeld.
app-source-pick-drive-title = De hoofdmap van het aangesloten station selecteren (bijv. E:\)
# { $path } is the folder picked, such as E:\. \Windows and \Users are folder names, as Windows writes them.
app-source-not-a-windows-drive = { $path } lijkt geen Windows-systeemstation te zijn (geen \Windows en \Users). Kies de hoofdmap van het station van de andere pc.
# Said of a drive or disk image with a Windows older than Windows 10. { $windows } is its name, such as "Windows 7
# Professional Service Pack 1, 32-bit". { $is_32_bit } is "yes" when that Windows is 32-bit, which adds the last
# sentence, and "no" when it is not.
app-source-older-windows =
    Hierop staat { $windows }. Installatieprogramma's die hiervan worden gemaakt, zijn bedoeld voor Windows 10 en 11. Met de optie Gebruikersgegevens gaan instellingen mee die Windows ongemerkt apart heeft gezet voor een oudere app, en runtimes die Windows 11 niet heeft, zoals DirectX 9, worden met de app geïnstalleerd.{ $is_32_bit ->
        [yes] { " " }De apps zijn 32-bits, dus ze worden in Program Files (x86) geplaatst, waar 64-bits Windows ze bewaart.
       *[no] { "" }
    }
# The same, of a Windows whose name could not be read. { $is_32_bit } is as above.
app-source-older-windows-unnamed =
    Hierop staat een oudere versie van Windows. Installatieprogramma's die hiervan worden gemaakt, zijn bedoeld voor Windows 10 en 11. Met de optie Gebruikersgegevens gaan instellingen mee die Windows ongemerkt apart heeft gezet voor een oudere app, en runtimes die Windows 11 niet heeft, zoals DirectX 9, worden met de app geïnstalleerd.{ $is_32_bit ->
        [yes] { " " }De apps zijn 32-bits, dus ze worden in Program Files (x86) geplaatst, waar 64-bits Windows ze bewaart.
       *[no] { "" }
    }

## The details panel, before an app is picked, and the app picked

app-details-pair-first = Koppelen met de oude pc
app-details-pair-first-hint = Zodra WinAppMigrator Client daar wordt uitgevoerd en je de code hebt getypt, worden de apps van die pc hier weergegeven en maak je van een ervan een installatieprogramma, net als op deze pc.
app-details-pick-an-app = Kies een app
app-details-pick-an-app-hint = Selecteer een app in de lijst om te kiezen wat je wilt opnemen en het installatieprogramma te maken.
# { $publisher } is who made the app, and { $version } its version, such as 11.4.2.
app-details-publisher-and-version = { $publisher }, versie { $version }
# { $version } is the app's version, such as 11.4.2.
app-details-unknown-publisher-and-version = Onbekende uitgever, versie { $version }
app-details-unknown-publisher = Onbekende uitgever

## What goes in besides the program

app-option-license = Licentie- en activeringsgegevens
app-option-license-hint = Kunnen aan deze pc gebonden zijn en moeten op de nieuwe pc mogelijk opnieuw worden geactiveerd.
app-option-user-data = Gebruikersgegevens
# Documents, Saved Games, and ProgramData are folders of Windows: use the names Windows gives them in your language,
# except ProgramData, which keeps its name.
app-option-user-data-hint = Instellingen en geschiedenis, Documenten, Opgeslagen spellen en ProgramData.
app-option-whose-data = Wiens gegevens
app-option-whose-data-hint = Een installatieprogramma bevat de gegevens van één account. Die gaan naar wie er is aangemeld op de pc waarop het wordt uitgevoerd.

## Building the installer

app-output-folder = Uitvoermap
# An example of a folder, shown in the empty field. "You" stands for the name of an account.
app-output-folder-example = C:\Users\Naam\WinAppMigrator
app-output-folder-not-full = Voer een volledig mappad in, zoals D:\Installatieprogramma's.
app-build-create = Installatieprogramma maken
app-build-create-again = Installatieprogramma opnieuw maken
app-build-cancel = Annuleren
app-build-cancelling = Annuleren na de huidige stap…
app-build-cancelled = Maken geannuleerd. Er is geen installatieprogramma geschreven.
# winappmigrator-installer.exe is a file name.
app-build-no-stub = De installatiestub (winappmigrator-installer.exe) is niet gevonden naast deze app, dus er kan geen zelfuitpakkend installatieprogramma worden gemaakt.
app-build-select-an-app-first = Selecteer eerst een app om te migreren.
app-build-look-again-first = Deze app moet opnieuw worden bekeken voordat er een installatieprogramma van kan worden gemaakt.
# { $reason } is what went wrong, in the program's own words, often in English.
app-build-stopped-unexpectedly = Het maken is onverwacht gestopt: { $reason }. Als er een installatieprogramma in de uitvoermap is achtergebleven, gebruik dat dan niet.
app-build-worker-disconnected = De verbinding met het migratieproces is verbroken.
# { $path } is the output folder, and { $error } what Windows said went wrong.
app-build-no-output-folder = Kan de uitvoermap { $path } niet maken: { $error }
# { $error } is what went wrong.
app-build-no-staging-folder = Kan geen beveiligde faseringsmap maken, dus er is niets gemaakt: { $error }
# { $error } is what went wrong, as a sentence.
app-build-could-not-build = Het installatieprogramma kan niet worden gemaakt. { $error }
app-build-no-installer = Het maken is voltooid zonder installatieprogramma.
# { $error } is what went wrong.
app-build-hash-check-did-not-run = Het installatieprogramma is geschreven, maar de hashcontrole kon niet worden uitgevoerd: { $error }. Gebruik het niet.

## How far a build has got

app-progress-preparing = Bepalen wat erin gaat…
app-progress-checking-room = Controleren of er ruimte is…
app-progress-bundling = Runtimes en stuurprogramma's bundelen…
# { $done } is how many files have been read so far, and { $total } how many there are.
app-progress-packing = Bestanden lezen en comprimeren: { $done } van { $total }
# { $done } is how many of the installer's records have been looked over so far, and { $total } how many there are.
app-progress-looking-over = Nakijken wat het installatieprogramma vermeldt: { $done } van { $total }…
app-progress-writing = Het installatieprogramma schrijven…
app-progress-verifying = Hashes verifiëren…

## What a build made

app-result-failed-hash-check = Gebruik dit installatieprogramma niet: het heeft de hashcontrole niet doorstaan.
# { $count } is how many files were gone when the build came to read them.
app-result-files-gone =
    { $count ->
        [one] Installatieprogramma gemaakt, maar { $count } bestand is verdwenen sinds de app is bekeken, dus het zit er niet in. Zie Waarschuwingen hieronder.
       *[other] Installatieprogramma gemaakt, maar { $count } bestanden zijn verdwenen sinds de app is bekeken, dus ze zitten er niet in. Zie Waarschuwingen hieronder.
    }
# { $count } is how many files could not be read into the installer.
app-result-files-not-captured =
    { $count ->
        [one] Installatieprogramma gemaakt, maar { $count } bestand kon niet worden vastgelegd, meestal omdat het vergrendeld of onleesbaar was. De app werkt mogelijk niet zonder dit bestand. Zie Waarschuwingen hieronder.
       *[other] Installatieprogramma gemaakt, maar { $count } bestanden konden niet worden vastgelegd, meestal omdat ze vergrendeld of onleesbaar waren. De app werkt mogelijk niet zonder deze bestanden. Zie Waarschuwingen hieronder.
    }
app-result-mentions-who = Installatieprogramma gemaakt, maar het vermeldt nog iets over van wie het afkomstig is. Lees hieronder wat er staat voordat je het aan iemand geeft.
app-result-ready = Installatieprogramma gereed. Het pakket heeft de hashcontrole doorstaan.
app-result-files = Bestanden
app-result-installer-size = Grootte van installatieprogramma
app-result-warnings = Waarschuwingen
app-result-log = Logboek
app-result-copy-error = Foutmelding kopiëren
app-result-installer = Installatieprogramma
app-result-data-file = Gegevensbestand
app-result-open-folder = Map openen
app-result-copy-path = Pad kopiëren
# { $error } is what Windows said went wrong.
app-result-could-not-open-folder = Kan de map niet openen: { $error }
# { $installer } and { $data_file } are file names, such as App_installer.exe and App_installer.wamdata.
app-result-two-files = Deze app is te groot voor één uitvoerbaar bestand, dus het installatieprogramma bestaat uit twee bestanden: { $installer } en { $data_file }. Ze moeten bij elkaar blijven: kopieer beide naar dezelfde map op de andere pc, met deze namen. Als je het installatieprogramma ondertekent, geldt die handtekening ook voor het gegevensbestand.
# { $app_name } is the app's name. "Unknown publisher" is what Windows shows when it asks to run an unsigned program: use
# Windows' own words for it in your language.
app-result-run-it = Voer het uit op de andere pc om { $app_name } opnieuw te installeren. Het controleert de bestanden met SHA-256. Windows toont "Onbekende uitgever" omdat installatieprogramma's die je maakt, niet zijn ondertekend.
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-verification-errors = Verificatiefouten ({ $count })
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-warnings-heading = Waarschuwingen ({ $count })
# After the first of a long list of warnings. { $count } is how many more there are.
app-warnings-and-more = en nog { $count }.

## Looking at an app, to see what an installer of it would hold

app-scan-getting-ready = Voorbereiden om deze app te bekijken…
app-scan-install-record = Lezen hoe de app is geïnstalleerd…
app-scan-near-matches = Zoeken naar wat er mogelijk ook bij hoort…
app-scan-system-items = Zoeken naar services, taken en snelkoppelingen…
app-scan-files = De bestanden doorlopen…
app-scan-registry = Lezen wat de app met Windows verbindt…
app-scan-finishing = Afronden…
app-scan-look-again = Opnieuw bekijken
app-scan-stop = Stoppen
app-scan-running-hint = Je kunt een andere app kiezen of wijzigen wat wordt opgenomen terwijl dit bezig is.
# { $error } is why, in words that follow a colon.
app-scan-failed = Deze app kan niet worden bekeken: { $error }
app-scan-try-again = Opnieuw proberen
app-scan-stopped = Gestopt. Er kan een installatieprogramma worden gemaakt zodra deze app is bekeken.
# { $reason } is what went wrong, in the program's own words, often in English.
app-scan-stopped-unexpectedly = Het bekijken is onverwacht gestopt: { $reason }.

## How big an installer can be, and whether there is room for it

app-room-files = Bestanden
# Over how big the installer can be: up to that size.
app-room-up-to = Maximaal
app-room-free = Vrij
# { $drive } is a drive, such as C:.
app-room-free-on = Vrij op { $drive }
# How much room a drive has, when it would not say.
app-room-free-unknown = Onbekend
app-room-most-it-could-need = Het maximum dat nodig kan zijn, vóór compressie.
# { $drive } is a drive, such as C:, { $needed } how much room it needs, such as 1.2 GB, and { $free } how much it has.
app-room-drive-short = { $drive } heeft maximaal { $needed } vrije ruimte nodig om dit te maken en heeft { $free }.
# { $drives } is one sentence like the one above for each drive without the room, one after another.
app-room-not-enough = Er is niet genoeg ruimte. { $drives } Kies een uitvoermap op een ander station of laat iets weg.
# { $note } is a sentence, without its full stop, about a drive whose room could not be checked. Use your language's
# full stop.
app-room-could-not-tell = { $note }.

## What goes in the installer, part by part

app-footprint-heading = Wat er in het installatieprogramma gaat
app-footprint-working-out = Bepalen wat daarbij hoort…
# { $error } is why.
app-footprint-would-not-make-a-package = Wat er over is, vormt geen pakket: { $error }
# { $list } is the names of what else is left out, such as a service and a shortcut.
app-footprint-also-left-out = Daardoor wordt ook dit weggelaten: { $list }, omdat wat ze uitvoeren of noemen er niet zou zijn. Ze worden niet ingesteld op de andere pc.
# { $list } is the names of the first few of what else is left out, and { $count } how many more there are.
app-footprint-also-left-out-and-more = Daardoor wordt ook dit weggelaten: { $list } en nog { $count }, omdat wat ze uitvoeren of noemen er niet zou zijn. Ze worden niet ingesteld op de andere pc.
# { $list } is the names of license files. The option is the one named "License and Activation Data".
app-footprint-licenses-still-go = Licentiegegevens in wat je hebt weggelaten, gaan toch mee, omdat die door de optie Licentie- en activeringsgegevens worden bepaald: { $list }.
# { $elapsed } is how long ago, in minutes and seconds such as 45:00, or hours, minutes, and seconds such as 1:05:00.
# "Look Again" names the button of that name.
app-footprint-looked-at-long-ago = Dit is { $elapsed } geleden bekeken. Bestanden worden gelezen wanneer je het installatieprogramma maakt, dus een wijziging wordt meegenomen en alles wat dan verdwenen is, wordt vermeld. Kies Opnieuw bekijken om iets nieuws te vinden.
# Headings of the parts of the list, each with a line that says what is in it.
app-footprint-program-files = Programmabestanden
app-footprint-data-folders = Gegevensmappen
# What could go in with the User Data option, when nothing else is in this part. { $size } is how much, such as 1.2 GB.
app-footprint-folders-with-user-data = { $size } met de optie Gebruikersgegevens
# { $included } is how many folders go in, { $count } how many there are, and { $size } how much those that go in hold.
app-footprint-folders-summary =
    { $count ->
        [one] { $included } van { $count } map, { $size }
       *[other] { $included } van { $count } mappen, { $size }
    }
# { $size } is how much, such as 1.2 GB, and { $count } in how many files.
app-footprint-data-only-with-user-data =
    { $count ->
        [one] { $size } in { $count } bestand met gegevens van deze app gaat alleen mee met de optie Gebruikersgegevens.
       *[other] { $size } in { $count } bestanden met gegevens van deze app gaat alleen mee met de optie Gebruikersgegevens.
    }
# A folder's row. { $count } is how many files it holds, and { $detail } where it is or why it cannot be left out.
app-footprint-folder-row =
    { $count ->
        [one] { $count } bestand. { $detail }
       *[other] { $count } bestanden. { $detail }
    }
app-footprint-services = Services
app-footprint-startup = Start automatisch
app-footprint-tasks = Geplande taken
app-footprint-shortcuts = Snelkoppelingen
# { $included } is how many of the part's things go in, and { $count } how many there are.
app-footprint-items-summary = { $included } van { $count }
# { $why } is what else was left out that this goes with.
app-footprint-left-out-too = Ook weggelaten: { $why }
# A row's detail. { $kind } is what it is, such as Service, and { $detail } what it does.
app-footprint-change-detail = { $kind }. { $detail }
app-footprint-integrations = Wat de app met Windows verbindt
# { $included } is how many kinds go in, such as file types and firewall rules, and { $count } how many kinds there are.
app-footprint-integrations-summary =
    { $count ->
        [one] { $included } van { $count } soort
       *[other] { $included } van { $count } soorten
    }
app-footprint-integration-nothing-left = Er is niets van over, door wat je hebt weggelaten
# { $count } is how many things of this kind the installer sets up.
app-footprint-integration-things =
    { $count ->
        [one] { $count } item wordt ingesteld
       *[other] { $count } items worden ingesteld
    }
app-footprint-prerequisites = Wat de app nodig heeft om te worden uitgevoerd
# { $count } is how many runtimes, such as Visual C++ Redistributable.
app-footprint-prerequisites-summary =
    { $count ->
        [one] { $count } runtime
       *[other] { $count } runtimes
    }
app-footprint-near-matches = Hoort mogelijk ook bij de app
# { $ticked } is how many were ticked, and { $count } how many there are.
app-footprint-near-matches-summary = { $ticked } van { $count } toegevoegd
app-footprint-near-matches-hint = Niet in het installatieprogramma, tenzij je ze aanvinkt. Als je er een aanvinkt, wordt de app opnieuw bekeken, met dezelfde controles als de rest, dus wat op een wachtwoord lijkt, blijft er nog steeds buiten.
# { $why } is why it might be part of the app, as a sentence.
app-footprint-near-match-with-user-data = Gaat mee met de optie Gebruikersgegevens. { $why }
# How much a folder holds, when not all of it could be measured. { $size } is how much was, such as 1.2 GB.
app-footprint-near-match-over = meer dan { $size }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-folder = Map: { $place }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-hidden = Verborgen onderdeel van een app: { $place }
app-footprint-nothing-left-out = Er is niets weggelaten omdat het op een wachtwoord, licentie of certificaat leek.
app-footprint-left-out = Weggelaten, en waarom
# { $count } is how many things were left out.
app-footprint-left-out-summary =
    { $count ->
        [one] { $count } item
       *[other] { $count } items
    }
# Above a list of things left out for one reason. { $title } is the reason.
app-footprint-left-out-group = { $title }:
app-footprint-refused = Geen enkel installatieprogramma mag deze instellen:
# A heading over a list that opens and closes. { $count } is how many are in it.
app-footprint-worth-knowing = Goed om te weten ({ $count })
# { $list } is what kinds of things were looked for and none found, from the five below.
app-none-found = Niets gevonden: { $list }.
app-none-found-services = services
app-none-found-startup = opstartitems
app-none-found-tasks = geplande taken
app-none-found-shortcuts = snelkoppelingen
app-none-found-integrations = iets wat de app met Windows verbindt

## Network PC: reading another PC through WinAppMigrator Client, in three steps. "WinAppMigrator Client" is the name of a
## program, and the old PC is the one the apps are moved off.

app-network-step-client = 1. Zet de client op de oude pc
app-network-step-client-hint = WinAppMigrator Client wordt uitgevoerd op de pc waarop de app nu staat, met Windows 7 of hoger. Alleen deze pc kan die pc dan lezen, versleuteld, met een code die de client weergeeft.
# { $drive } is a USB drive, such as E:\.
app-network-save-to-usb = Opslaan op USB-station { $drive }
# { $drive } is a USB drive, such as E:\, and { $name } the name it was given, such as BACKUP.
app-network-save-to-named-usb = Opslaan op USB-station { $drive } ({ $name })
app-network-save-to-folder = Opslaan in een map…
app-network-look-for-usb-again = Opnieuw zoeken naar USB-stations
app-network-no-usb = Er is geen USB-station aangesloten. Sluit er een aan of sla op in een map die je vanaf de oude pc kunt bereiken, zoals een gedeelde map in je netwerk.
# { $path } is where the client was saved, such as E:\WinAppMigrator Client.exe.
app-network-client-saved = Opgeslagen als { $path }. Sluit op de oude pc de app die je verplaatst, dubbelklik dan op dit bestand en sta toe dat het wijzigingen aanbrengt. Er wordt een code weergegeven.
app-network-step-pick = 2. Kies de oude pc
app-network-looking = Zoeken in je netwerk naar pc's waarop de client wordt uitgevoerd…
app-network-type-name = Of typ de naam of het adres
# An example, shown in the empty field: a PC's name, or its network address.
app-network-type-name-example = OUDE-PC of 192.168.1.20
app-network-step-code = 3. Typ de code die wordt weergegeven
app-network-code = Koppelcode
app-network-connect = Verbinden
app-network-pairing = Koppelen…
# Why pairing did not work, shown on its own.
app-network-pairing-stopped = het koppelen is gestopt voordat het klaar was
app-network-wrong-code = dat is niet de code die de oude pc weergeeft. Controleer de code en probeer het opnieuw.
# { $error } is what went wrong.
app-network-not-reached = de oude pc kan niet worden bereikt: { $error }
app-network-pick-folder-title = Een map kiezen voor WinAppMigrator Client
# { $path } is the folder, and { $error } what Windows said went wrong.
app-client-could-not-save = de client kan niet worden opgeslagen in { $path }: { $error }
# scripts\build-client.ps1 is a file name.
app-client-not-carried = deze build van WinAppMigrator bevat de client niet. Maak de client met scripts\build-client.ps1 en maak de app daarna opnieuw.
# { $error } is what Windows said went wrong.
app-client-could-not-save-note = de notitie die ernaast hoort, kan niet worden opgeslagen: { $error }
# A note saved as a text file beside the client, for whoever runs it on the old PC. Keep its lines short, as it is read in
# Notepad. "WinAppMigrator Client.exe" is a file name. "Network PC..." names the Scan Source button: use its words in your
# language.
app-client-note =
    WinAppMigrator Client
    
    Voer dit uit op de pc waar je apps vanaf verplaatst: Windows 7, 8, 10 of 11.
    
    1. Sluit de app die je verplaatst.
    2. Dubbelklik op "WinAppMigrator Client.exe" en sta toe dat het programma
       wijzigingen aanbrengt.
    3. Er wordt een koppelcode van 6 cijfers weergegeven. Kies in WinAppMigrator
       op je nieuwe pc Netwerk-pc..., kies deze pc en typ de code.
    4. Druk op Enter in het venster van de client of sluit het venster wanneer
       de nieuwe pc klaar is.
    
    Het programma leest alleen. Terwijl het wordt uitgevoerd, bewaart het een
    momentopname van het Windows-station en laat het zijn eigen twee poorten
    vanuit je lokale netwerk door Windows Firewall toe. Beide worden verwijderd
    wanneer het stopt. Wat via het netwerk gaat, is versleuteld, en alleen een
    pc die de code heeft, kan iets lezen.
