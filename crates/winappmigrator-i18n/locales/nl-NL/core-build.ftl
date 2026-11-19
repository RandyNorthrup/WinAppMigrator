# Words the core writes for people while it builds a package from an installed app: what a package records for an
# installer to show later, errors shown in the app, the list of what goes in and what is left out, opening a disk
# image or another PC over the network, and what an installer says when the package it carries cannot be read.
# Message names start with core-.


## Scanning an app, before a package is built. Shown in the app when a scan cannot go on.

# { $path } is the app's program, such as D:\Program Files\Acme\Acme.exe, on a drive taken from another PC.
core-scan-program-not-on-the-drive = { $path } loopt via een koppeling of staat niet op het aangesloten station, dus het is niet gelezen.
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-not-on-this-pc = { $path } staat niet op een schijf van deze pc, of heeft een naam die iets anders zou openen, dus het is niet gelezen.
# { $path } is the app's program.
core-scan-program-someone-elses = { $path } staat in de mappen van een ander account of is niet van dit account, en volgens de eigen installatiegegevens van deze app hoort het daar niet, dus het is niet gelezen.
# { $app_name } is the app's name, as the list of apps calls it.
core-scan-nothing-to-package = Er is geen eigen programma van { $app_name } gevonden, dus er is niets om in een installatieprogramma te zetten. Bij stuurprogramma's, runtimes en invoegtoepassingen is dat vaak zo: installeer op de nieuwe pc een stuurprogramma via de fabrikant of Windows Update, en een runtime of invoegtoepassing samen met de app die deze gebruikt.
# { $app_name } is the app's name, as the list of apps calls it. A web app is a website installed as an app, with
# a window and a Start menu entry of its own.
core-scan-web-app = { $app_name } is een web-app die een browser vanaf een website heeft geïnstalleerd. Deze heeft geen eigen programma, dus er is niets om in een installatieprogramma te zetten. Open op de nieuwe pc de website in die browser en installeer de app vanaf daar.
# { $path } is the program the app's own record names, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-missing = { $path } is er niet meer, hoewel het in de installatiegegevens van deze app als het programma van de app wordt vermeld. De app is misschien verplaatst of deels verwijderd: als je de app op deze pc herstelt of opnieuw installeert, zou het terug moeten komen.
# { $path } is the file that was picked.
core-scan-not-a-file = scandoel is geen bestand: { $path }
# Names something left out of a package, in a list of what was left out and why. { $name } is the variable's name,
# such as ACME_LICENSE_KEY.
core-scan-environment-variable-item = De omgevingsvariabele { $name }

## Folders and parts that might also be the app's, offered in a list to tick. Each says why it is offered.

core-near-starts-with-app-name = De naam begint met de naam van deze app.
core-near-has-app-name = De naam bevat de naam van deze app.
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-starts-with-publisher = De naam begint met de naam van de uitgever, { $publisher }.
# { $folder } is a folder's name, such as Program Files.
core-near-hidden-part-beside = Een onderdeel dat niet in de lijst met apps staat, geïnstalleerd naast deze app in { $folder }.
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-hidden-part-publisher = Een onderdeel dat niet in de lijst met apps staat, van dezelfde uitgever, { $publisher }.

## What a scan found, as a person chooses from it: the folders that go in, and what is left out, and why

# Why a folder cannot be left out, beside its number of files.
core-footprint-locked-app-folder = Deze gaan altijd mee met het hoofdprogramma
core-footprint-locked-main-program = Bevat het hoofdprogramma, dat altijd meegaat
core-footprint-locked-needed = Bevat wat het hoofdprogramma nodig heeft om te kunnen starten
core-footprint-locked-font = Gaat mee met het lettertype, onder Wat de app met Windows verbindt
# Where a folder is. "AppData" and "ProgramData" are the names of Windows folders; keep them as they are.
core-footprint-place-roaming = Instellingen in je profiel (AppData)
core-footprint-place-local = Instellingen voor deze pc in je profiel (AppData)
core-footprint-place-program-data = Gedeeld door iedereen op de pc (ProgramData)
core-footprint-place-documents = In je Documenten
core-footprint-place-saved-games = In je Opgeslagen spellen
core-footprint-place-public-documents = In Openbare documenten
core-footprint-place-app-folder = In de map van de app
# { $folder } is a folder, such as C:\Users\Sam\AppData\Roaming.
core-footprint-place-in = In { $folder }
# A row for the files directly in the app's folder. { $folder } is that folder's name, such as Acme.
core-footprint-files-directly-in = Bestanden direct in { $folder }
# Headings over what is left out, and why each item is.
core-footprint-licenses-left-out = Niet in dit installatieprogramma, omdat de optie Licentie- en activeringsgegevens is uitgeschakeld
core-footprint-font-stays-behind = Het blijft achter.
core-footprint-drivers-left-out = Stuurprogramma's van Windows of van een leverancier, die niet mogen worden doorgegeven, dus installeer ze opnieuw op de andere pc
core-footprint-driver-reinstall = Installeer het opnieuw met het eigen pakket van de leverancier.
# Why something else is left out along with what the person left out. { $service } is a service's name, such as AcmeSync.
core-footprint-needs-left-out = het heeft { $service } nodig, en dat is weggelaten
core-footprint-told-to-use-left-out = het moet iets gebruiken in een map die je hebt weggelaten
core-footprint-file-in-left-out = het bestand ervan staat in een map die je hebt weggelaten
core-footprint-program-in-left-out = het programma waarvoor het is, staat in een map die je hebt weggelaten
core-footprint-names-left-out = het noemt een map die je hebt weggelaten
# Why something is left out, as the installer tells whoever installs it.
core-footprint-builder-left-it-out = wie dit installatieprogramma heeft gemaakt, heeft het weggelaten
# Something the person left out, by kind and name, such as "Service Acme Updater". { $kind } is the kind, such as
# Service or Scheduled Task; { $title } is its name.
core-footprint-left-out-item = { $kind } { $title }

## Building a package and an installer from a scan. Shown in the app when a build cannot go on.

core-package-scan-of-another-app = deze scan is van een andere app dan de app waarvan het installatieprogramma wordt gemaakt, dus er is niets gemaakt
core-package-scan-brought-in-more = deze scan heeft gegevens opgenomen waar bij het maken niet om is gevraagd, dus er is niets gemaakt
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-package-program-gone = { $path } is er niet meer, dus er is niets gemaakt. Bekijk de app opnieuw.
core-package-needs-stub = voor een zelfuitpakkend archief is een installatiestub vereist
# { $reason } is what went wrong.
core-package-staged-file-failed = een bestand in de faseringsmap kan niet in het pakket worden opgenomen: { $reason }
# { $shortfalls } says which drives are short of room and by how much, in sentences of its own.
core-package-not-enough-room = er is niet genoeg ruimte om dit installatieprogramma te maken, dus er is niets gelezen of geschreven. { $shortfalls }. Maak daar ruimte vrij of kies een uitvoermap op een ander station.
# { $path } is a file's path, on a drive taken from another PC.
core-package-not-read-off-the-drive = { $path } loopt via een koppeling of staat niet op het aangesloten station, dus het is niet gelezen
# { $path } is a file's path.
core-package-not-read-not-local = { $path } staat niet op een schijf van deze pc, dus het is niet gelezen
# { $path } is a file's path.
core-package-not-read-other-account = { $path } leidt naar de mappen van een ander account, dus het is niet gelezen
# { $path } is a file's path, as the PC it came from called it.
core-package-not-on-the-drive = { $path } staat niet op het aangesloten station, dus het is niet gelezen
# { $path } is a file's path.
core-package-backup-copy-no-parent = kan geen back-upkopie maken van een bestand zonder bovenliggende map: { $path }
# { $path } is a file's path.
core-package-backup-copy-destination-no-parent = doel van back-upkopie heeft geen bovenliggende map: { $path }
# robocopy is the name of a Windows command; keep it as it is. { $code } is how it ended, such as Some(8).
core-package-robocopy-failed = kopiëren met robocopy in back-upmodus mislukt (afsluitcode { $code })
# Stands for a folder where a file's name would be, as in "I/O failed for temporary package folder: …".
core-package-temporary-folder = tijdelijke pakketmap
# Stands for the installer being written where a file's name would be, as in "I/O failed for the installer: …".
core-package-the-installer = het installatieprogramma
# { $path } is a file's path, or its name in the package. { $error } is what went wrong with it.
core-package-path-error = { $path }: { $error }
# { $name } is a file's name in the package, such as app/bin/App.exe.
core-package-same-place = een ander bestand heeft dezelfde plek in het pakket: { $name }
# { $name } is a file's name in the package.
core-package-changed-in-protected-folder = { $name } is gewijzigd in de beveiligde map terwijl het pakket werd gemaakt
core-package-worker-died = een thread die bestanden comprimeerde, is onverwacht gestopt, dus er is geen installatieprogramma geschreven
# { $path } is a file's path.
core-package-link-not-read = { $path } is een koppeling, dus het is niet gelezen
# { $path } is a file's path. { $error } is what went wrong.
core-package-not-compressed = { $path } kan niet worden gecomprimeerd: { $error }
# { $path } is a file's path. { $reason } is what went wrong.
core-package-not-read-back = { $path } kan niet worden teruggelezen: { $reason }

## Where an app's files are, and where an installer may write them. Shown when a path is refused.

# { $path } is a folder, such as C:\Program Files\Acme.
core-discovery-root-not-full-path = de map waaronder moet worden geschreven, moet een volledig pad zijn: { $path }
# { $path } is a folder's path.
core-discovery-root-not-a-folder = de map waaronder moet worden geschreven, is geen map: { $path }
# { $path } is a file's path.
core-discovery-escapes-root = pad valt buiten de bedoelde hoofdmap: { $path }
# { $path } is the link's path.
core-discovery-refusing-to-write-through-link = schrijven via een koppeling geweigerd: { $path }
# { $path } is the link's path.
core-discovery-refusing-link = volgen van een koppeling geweigerd: { $path }
# { $error } is what Windows said went wrong.
core-discovery-refusing-link-because = volgen van een koppeling geweigerd: { $error }
# { $path } is the link's path.
core-discovery-link-to-somewhere-else = { $path } is een koppeling naar een andere plek, en anderen kunnen mogelijk wijzigen waar deze naartoe leidt. Gebruik de plek waar deze naartoe leidt
# { $path } is a folder's path.
core-discovery-no-parent-folder = { $path } heeft geen bovenliggende map
# { $path } is the install location as it was given. Keep the quotes, or use your language's own.
core-discovery-install-location-not-full-path = de installatielocatie moet een volledig pad zijn: "{ $path }"
# { $path } is the install location.
core-discovery-install-location-a-link = de installatielocatie is een koppeling naar een andere plek: { $path }
# { $path } is a file's path.
core-discovery-device-name = pad gebruikt een naam die is gereserveerd voor een Windows-apparaat: { $path }
# { $path } is a file's path.
core-discovery-device-name-on-older-windows = { $path } heeft een naam die deze versie van Windows als apparaat opent en niet als bestand, dus er is niets naar geschreven. Het kan wel worden geïnstalleerd op Windows 11
# { $path } is a file's path.
core-discovery-unsafe-path = onveilig pad: { $path }
# { $path } is a file's path.
core-discovery-no-package-path = bestand heeft geen relatief pad in het pakket: { $path }

## Writing an installer, and reading the package inside one. Most of these are shown by an installer, on the PC it
## runs on, when it cannot read what it carries. "The package" is the app's files and settings that an installer
## carries; "the stub" is the program part of an installer, before the package; "the data file" is the .wamdata file
## that holds the package of a very large installer, beside it.

# Stands for the package where a file's name would be, as in "I/O failed for installer package: …".
core-archive-installer-package = installatiepakket
# Stands for the installer program built into WinAppMigrator, where a file's name would be.
core-archive-embedded-stub = ingesloten installatiestub
# { $index } is the number of a file in the package. { $reason } is what went wrong.
core-archive-entry-unreadable = item { $index } van het pakket kan niet worden gelezen: { $reason }
# { $name } is a file's name inside the package, such as app/bin/App.exe.
core-archive-entry-unsafe-path = zip-item heeft een onveilig pad: { $name }
# { $name } is a file's name inside the package.
core-archive-holds-more = { $name } bevat meer dan het aangeeft
# { $name } is a file's name inside the package.
core-archive-name-too-long = deze naam is te lang voor een zip-bestand: { $name }
core-archive-changed-while-copied = het pakket is gewijzigd terwijl het achter de stub werd geplaatst, dus er is geen installatieprogramma geschreven
core-archive-stub-not-a-program = het deel van dit installatieprogramma vóór het pakket is geen programma
core-archive-damaged = dit installatieprogramma is beschadigd of is gewijzigd nadat het is gemaakt, dus er is niets uit het pakket gelezen. Kopieer het opnieuw van de plek waar het is gemaakt.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-missing = dit installatieprogramma bewaart het pakket in een bestand met de naam { $name }, in dezelfde map, en dat bestand is er niet. De twee bestanden moeten bij elkaar blijven: kopieer { $name } hierheen van de plek waar het installatieprogramma is gemaakt en voer dit daarna opnieuw uit.
# { $name } is the data file's name, such as App_installer.wamdata. { $error } is what Windows said went wrong.
core-archive-data-file-unopened = het gegevensbestand van dit installatieprogramma, { $name }, kan niet worden geopend: { $error }. De twee bestanden moeten bij elkaar blijven, in een map die deze pc kan lezen.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-a-link = { $name } is een koppeling of een map, en het gegevensbestand van dit installatieprogramma moet een eigen bestand zijn, direct ernaast, dus er is niets uit gelezen. Kopieer het echte bestand hierheen.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-wrong = { $name } is niet het gegevensbestand van dit installatieprogramma, of het is beschadigd of gewijzigd nadat het is gemaakt, dus er is niets uit gelezen. Kopieer beide bestanden opnieuw van de plek waar ze zijn gemaakt.
core-archive-stub-signed = de installatiestub is ondertekend, maar moet niet-ondertekend blijven, dus onderteken in plaats daarvan het voltooide installatieprogramma
# { $size } is a size, such as 4.2 GB.
core-archive-too-big-to-run = het installatieprogramma zou { $size } groot worden, en Windows kan geen uitvoerbaar bestand van 4 GB of groter uitvoeren; er is niets geschreven
core-archive-outside-signature = installatieprogramma bevat gegevens buiten de handtekening en is mogelijk gemanipuleerd
core-archive-footer-length = de lengte van het afsluitblok van het zelfuitpakkende archief is ongeldig
core-archive-seek-outside = zoekpositie buiten het pakket
# { $path } is a file's path.
core-archive-unsafe-package-path = onveilig pakketpad: { $path }
# { $path } is a file's path.
core-archive-empty-package-path = leeg pakketpad: { $path }

## Opening a disk image of another PC (a .vhd or .vhdx file) to read apps from it

# { $path } is the disk image file, such as D:\Backups\OldPC.vhdx.
core-image-not-attachable = { $path } is geen schijfkopie die Windows kan koppelen (.vhd of .vhdx)
core-image-needs-administrator = voor het koppelen van een schijfkopie zijn administratorrechten nodig
# { $path } is the disk image file the person picked.
core-image-not-a-file = { $path } is geen bestand
# { $path } is the disk image file. "Edit Disk" is the name of a Hyper-V Manager command.
core-image-differencing = { $path } is een differentiërende schijf, die alleen bevat wat er is gewijzigd sinds de schijf waarvan deze is gemaakt, en Windows zou die schijf ook openen, waar die ook staat. Voeg deze eerst samen met die schijf, met Schijf bewerken in Hyper-V, en kies dan die schijf
# { $path } is the disk image file.
core-image-unreadable = { $path } is geen .vhd of .vhdx die kan worden gelezen
# ProgramData is the name of a Windows folder; keep it as it is.
core-image-no-program-data = de map ProgramData van deze pc is niet gevonden
# The reason given in core-image-could-not-attach when Windows already has the image open.
core-image-already-attached = Windows heeft de schijfkopie al gekoppeld, en wat de kopie heeft gekoppeld, schrijft er mogelijk naar. Ontkoppel de kopie eerst of kies het station ervan als aangesloten station
# { $path } is the disk image file. { $reason } is why, such as core-image-already-attached or what Windows said.
core-image-could-not-attach = { $path } kan niet worden gekoppeld: { $reason }
# { $path } is the disk image file.
core-image-no-windows-volume = { $path } heeft geen leesbaar volume met Windows erop. Een volume dat met BitLocker is versleuteld, moet eerst worden ontgrendeld.
# Given as the reason when Windows failed without saying anything.
core-image-windows-did-not-say = Windows heeft geen reden opgegeven

## Reading another PC over the network, through the client running on it. Shown as why something on that PC
## could not be read. "The client" is WinAppMigrator Client, the small program on the other PC.

core-remote-too-slow = het duurde te lang voordat de andere pc antwoordde
core-remote-closed = de verbinding met de andere pc is gesloten
core-remote-failed = de verbinding met de andere pc is mislukt
core-remote-answered-something-else = de client gaf een ander antwoord dan verwacht
# { $name } is the PC's name or address as the person typed it, such as OLDPC or 192.168.1.20.
core-remote-not-found = { $name } is niet gevonden in het netwerk
core-remote-did-not-say-what-it-is = de client heeft niet gezegd wat het is
core-remote-ordinary-names-only = er wordt alleen gevraagd naar een pad onder de hoofdmap van de andere pc, met gewone namen
core-remote-more-but-none = de andere pc meldde dat een map meer bevatte, maar gaf daar niets van door
# { $count } is the most things one folder may hold, such as 250,000.
core-remote-too-many-listed = de andere pc heeft meer dan { $count } items in één map vermeld
core-remote-sent-too-much = de andere pc heeft meer van een bestand verzonden dan was gevraagd
core-remote-name-not-text = er kan niet worden gevraagd naar een naam die geen tekst is
core-remote-below-root-only = er kan alleen worden gevraagd naar een pad onder de hoofdmap van de andere pc
core-remote-file-ended-early = het bestand van de andere pc hield te vroeg op
core-remote-link-not-followed = een koppeling op een andere pc wordt niet gevolgd
core-remote-only-files-opened = op een andere pc wordt alleen een bestand geopend
core-remote-listing-said-nothing = de lijst vermeldde er niets over
core-remote-change-time-unknown = wanneer het is gewijzigd, is niet bekend
core-remote-seek-before-start = een zoekpositie vóór het begin
# { $size } is how many bytes a file is; { $limit } is the most that were expected. Both are numbers.
core-remote-too-large =
    { $size ->
        [one] het is { $size } byte, en er werd niet meer dan { $limit } verwacht
       *[other] het is { $size } bytes, en er werd niet meer dan { $limit } verwacht
    }

## Checking that a download is signed by Microsoft. Each is given as the reason a check failed.

# { $error } is what Windows said went wrong.
core-signature-no-powershell = kan PowerShell niet starten: { $error }
# { $error } is what went wrong reading PowerShell's answer.
core-signature-unreadable = kan de handtekening niet lezen: { $error }
# { $signer } is who signed the file, such as Contoso Ltd.
core-signature-not-microsoft = het is ondertekend door { $signer }, niet door Microsoft
core-signature-not-microsoft-unnamed = het is ondertekend door iemand anders, niet door Microsoft
# { $status } is Windows' own word for the signature's state, such as HashMismatch or NotSigned, which is not translated.
core-signature-status = de status van de handtekening is { $status }

## Warnings a package records when it is built from an app, which an installer shows later, maybe on another PC, in
## that PC's language. "This app" is the app being packaged.

core-warning-build-program-needs-unreadable = Wat dit van Windows nodig heeft, kan niet worden gelezen, dus er wordt niet gecontroleerd of het op de andere pc kan worden uitgevoerd.
core-warning-build-external-manifest = Extern toepassingsmanifest gevonden. De interpretatie van side-by-side-afhankelijkheden wordt uitgesteld.
core-warning-build-non-exe = Het geselecteerde bestand heeft niet de extensie .exe. De PE-scan gaat door.
core-warning-build-stand-in-program = Er is geen gewoon uitvoerbaar bestand van de app gevonden, dus dit bestand wordt in plaats daarvan gebruikt. De resultaten voor architectuur en afhankelijkheden beschrijven dit bestand en beschrijven mogelijk niet de app.
core-warning-build-other-apps-files-unknown = Niet alles wat Windows Installer over andere apps heeft, kon worden gelezen, dus een map die voor deze app is geregistreerd, kan bestanden van een andere app bevatten zonder dat iets hier dat aangeeft.
core-warning-build-app-folder-has-another-apps-file = Volgens Windows Installer heeft een andere app een bestand in de map waarin het programma van deze app staat, en dat gaat mee, omdat de hele map wordt meegenomen.
# "App Paths" is the name of a Windows registry key; keep it as it is.
core-warning-build-app-paths-folder-left-out = Volgens het App Paths-item van deze app zoekt de app in een map die niet van de app zelf is, dus die map gaat niet mee. De app start mogelijk niet op een pc zonder wat daar staat.
core-warning-build-uninstaller-folder-left-out = De map waarin het verwijderprogramma van deze app staat, is weggelaten, omdat er volgens Windows Installer een bestand van een andere app in staat.
# { $why } is never shown: it chooses the reason. Translate each reason as the end of the sentence.
core-warning-build-recorded-folder-left-out =
    Iets wat voor deze app is geregistreerd, noemt een map die is weggelaten, omdat { $why ->
        [unreadable] het pad ervan het station verlaat of via een koppeling loopt, zodat die niet veilig kan worden gelezen
        [missing] die er niet is
        [link] die via een koppeling loopt
        [short-name] die niet is geschreven zoals die echt heet
        [another-account] die in de mappen van een ander account staat
        [one-account] deze app voor één account is geïnstalleerd en de map niet van dat account is
        [shared] die gedeelde inhoud bevat of deel uitmaakt van Windows
        [generic-name] de naam niets zegt over van wie de map is
        [publisher] die alleen naar de uitgever is genoemd en ook andere producten van de uitgever kan bevatten
        [holds-the-app] die de eigen map van deze app bevat, en mogelijk meer
        [another-apps-name] die de naam van een andere app heeft
        [another-apps-record] de installatiegegevens ervan van een andere app zijn
        [in-another-apps-folder] die in de map van een andere app staat
       *[another-apps] die van een andere app is
    }.
core-warning-build-app-folder-holds-another-app = Een andere app op deze pc is geregistreerd in de map van deze app en gaat mee, omdat de hele map wordt meegenomen.
# "Program Files (x86)" is the name of a Windows folder; keep it as Windows calls it.
core-warning-build-from-32-bit-windows = Deze app stond op 32-bits Windows. De registersleutels zijn ingedeeld zoals 64-bits Windows die van een 32-bits app bewaart, en het installatieprogramma biedt Program Files (x86) aan voor de app.
# { $dll } is a file's name, such as MSVCP140D.dll.
core-warning-build-debug-runtime = Een programma heeft { $dll } nodig, een debugversie van de Visual C++-runtime. Microsoft heeft daar geen installatieprogramma voor en staat mogelijk niet toe dat deze worden doorgegeven, dus het programma wordt niet uitgevoerd op een pc zonder Visual Studio.
# { $program } is a program's file name, such as Tool.exe.
core-warning-build-single-file-program = { $program } is een .NET-programma dat in één bestand is verpakt en niet vermeldt welke .NET-versie het nodig heeft, dus er is geen .NET voor gepland. Als het er een nodig heeft, start het niet op een pc waarop die ontbreekt.
# { $count } is how many of the app's programs.
core-warning-build-programs-not-inspected =
    { $count ->
        [one] { $count } van de programma's van de app is niet gecontroleerd op de runtimes die het nodig heeft, omdat de app er zo veel heeft of ze zo groot zijn.
       *[other] { $count } van de programma's van de app zijn niet gecontroleerd op de runtimes die ze nodig hebben, omdat de app er zo veel heeft of ze zo groot zijn.
    }
# { $runtime } is a runtime's name, such as Java Runtime Environment.
core-warning-build-runtime-no-installer = De app heeft { $runtime } nodig, en daar is hier geen installatieprogramma voor, dus er wordt niets uitgevoerd. Installeer het zelf op de andere pc.
# { $why } is why something was left out, such as "its name says what it holds".
core-warning-build-left-out-of-package = Weggelaten uit het pakket: { $why }.
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-missing-dependency = Afhankelijkheid kan niet worden gevonden: { $name }
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-dependency-only-here = { $name } is alleen gevonden via het zoekpad van deze pc en hoort niet bij de app, dus het gaat niet mee. De app start mogelijk niet op een pc zonder dit bestand.
core-warning-build-driver-detected = Stuurprogramma of kernelonderdeel gevonden. Een stuurprogramma van de app zelf wordt meegeleverd en opnieuw geregistreerd. Een stuurprogramma van een leverancier of van Windows wordt niet doorgegeven en moet opnieuw worden geïnstalleerd met het ondertekende pakket van de leverancier.
core-warning-build-near-no-longer-offered = Iets wat is aangevinkt in de lijst met wat mogelijk ook bij deze app hoort, staat niet meer in die lijst, dus het is niet toegevoegd. Bekijk de lijst opnieuw.
# { $app_name } is the name of a part of the app, as the list of apps calls it. { $why } is never shown: it chooses the
# reason. Translate each reason as the end of the sentence.
core-warning-build-near-part-left-out =
    Een deel van wat { $app_name } als eigen onderdeel opgeeft, is weggelaten: { $why ->
        [unreadable] het pad ervan verlaat het station of loopt via een koppeling, dus het kan niet veilig worden gelezen
        [missing] het is er niet
        [link] het loopt via een koppeling
        [short-name] het is niet geschreven zoals het echt heet
        [another-account] het staat in de mappen van een ander account
        [one-account] deze app is voor één account geïnstalleerd en dit is niet van dat account
        [shared] het bevat gedeelde inhoud of maakt deel uit van Windows
        [generic-name] de naam zegt niets over van wie het is
        [publisher] het is alleen naar de uitgever genoemd en kan ook andere producten van de uitgever bevatten
        [holds-the-app] het bevat de eigen map van deze app, en mogelijk meer
        [another-apps-name] het heeft de naam van een andere app
        [another-apps-record] de installatiegegevens ervan zijn van een andere app
        [in-another-apps-folder] het staat in de map van een andere app
       *[another-apps] het is van een andere app
    }.
# { $app_name } is the name of a part of the app, as the list of apps calls it.
core-warning-build-near-keys-left-out = De registersleutels die in de installatiegegevens van { $app_name } staan, zijn weggelaten, omdat die gegevens ook iets noemen wat van een andere app is.
# "Windows keeps what an app wrote for one account" is Windows' VirtualStore folder.
core-warning-build-virtual-store-program = Er stond een programma in de map waarin Windows bewaart wat een app voor één account heeft geschreven. Windows zet daar nooit een programma neer, dus iets anders heeft dat gedaan, en het is weggelaten.
# { $path } is a file or folder, such as C:\Users\Sam\AppData\Roaming\Acme.
core-warning-build-file-left-out = { $path } is weggelaten: het loopt via een koppeling op een aangesloten station, of het hoort bij een ander account dan het account waarvan de gegevens worden opgenomen.
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as D:\Sam\Documents.
core-warning-build-user-folder-not-on-drive = De map { $folder } van { $user } stond op { $path }, en dat is niet op dit station, dus wat daar staat, is niet opgenomen.
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as \\server\home\Sam\Documents.
core-warning-build-user-folder-not-on-this-pc = De map { $folder } van { $user } stond op { $path }, en dat is niet op een schijf van deze pc, dus wat daar staat, is niet opgenomen.
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-base-unreadable = Kan { $path } niet lezen ({ $error }). App-gegevens die daar zijn opgeslagen, zijn niet opgenomen.
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-folder-unreadable = Kan { $path } niet lezen ({ $error }). Geneste app-gegevens daarin ontbreken mogelijk.
# { $error } is what went wrong.
core-warning-build-entry-unreadable = Kan een item in het bestandssysteem niet lezen: { $error }
# { $count } is how many links. { $names } lists some of them. { $more } is how many more there are, which are not
# named; when it is 0, the sentence ends after the list.
core-warning-build-links-not-followed =
    { $count ->
        [one] { $count } koppeling, of naam die iets anders zou openen, is in deze map niet gevolgd, dus waar die naar verwijst, zit niet in het installatieprogramma: { $names }
       *[other] { $count } koppelingen, of namen die iets anders zouden openen, zijn in deze map niet gevolgd, dus waar ze naar verwijzen, zit niet in het installatieprogramma: { $names }
    }{ $more ->
        [0] .
       *[other] { " " }en nog { $more }.
    }
# { $count } is how many folders. { $depth } is how many levels down a folder can be. { $names } lists some of them.
# { $more } is how many more there are, which are not named; when it is 0, the sentence ends after the list.
core-warning-build-folders-too-deep =
    { $count ->
        [one] { $count } map ligt meer dan { $depth } niveaus diep, dus wat erin staat, zit niet in het installatieprogramma: { $names }
       *[other] { $count } mappen liggen meer dan { $depth } niveaus diep, dus wat erin staat, zit niet in het installatieprogramma: { $names }
    }{ $more ->
        [0] .
       *[other] { " " }en nog { $more }.
    }
# { $error } is what went wrong.
core-warning-build-metadata-unreadable = Kan metagegevens van bestand niet lezen: { $error }
core-warning-build-reserved-name = De naam van dit bestand is gereserveerd voor een Windows-apparaat, dus het is niet gekopieerd.
# { $error } is what went wrong.
core-warning-build-hash-failed = Kan geen hash van het bestand berekenen: { $error }
core-warning-build-cloud-placeholder = Cloudbestand dat alleen online beschikbaar is (bijv. OneDrive Bestanden op aanvraag) heeft geen lokale gegevens. Zet het vast op dit apparaat of download het vóór de back-up. Gemeld, maar niet gekopieerd.
# { $file_name } is a driver's file name, such as acmeflt.sys.
core-warning-build-driver-file = Stuurprogramma- of kernelbestand '{ $file_name }' wordt niet als gewoon bestand gekopieerd. Als het hoort bij een geregistreerde stuurprogrammaservice van deze app, neemt het installatieprogramma het bestand op en wordt het stuurprogramma automatisch opnieuw geregistreerd. Installeer het anders opnieuw met het ondertekende stuurprogrammapakket van de leverancier.
core-warning-build-file-gone = Dit was er toen de app werd bekeken en is nu verdwenen, dus het zit niet in het installatieprogramma.
# { $error } is what went wrong.
core-warning-build-file-copy-failed = Kan bestand niet naar het pakket kopiëren: { $error }
# { $count } is how many files. { $names } lists them.
core-warning-build-not-looked-through =
    { $count ->
        [one] Dit pakket bevat een bestand dat niet kon worden doorzocht op vermeldingen van wie het afkomstig is: { $names }. De inhoud kan dat wel vermelden.
       *[other] Dit pakket bevat bestanden die niet konden worden doorzocht op vermeldingen van wie ze afkomstig zijn: { $names }. De inhoud kan dat wel vermelden.
    }
# { $count } is the most empty folders an installer makes, such as 10,000.
core-warning-build-empty-folders-truncated = Deze app heeft meer dan { $count } lege mappen, en er worden er maar zoveel opnieuw gemaakt.
core-warning-build-credential-by-now = Weggelaten uit het pakket: het lijkt inmiddels op een referentie.
core-warning-build-no-package-path = Kopieerbaar bestand had geen relatief pad in het pakket.
# { $file_name } is the runtime installer's file name, such as vc_redist.x64.exe. { $error } is what went wrong.
core-warning-build-prerequisite-path-invalid = Kan meegeleverde vereiste { $file_name } niet plaatsen: { $error }
# { $runtime } is a runtime's name, such as Microsoft Visual C++ 2015+ Redistributable (x64). { $error } is what went
# wrong. winget is the name of a Windows command; keep it as it is.
core-warning-build-prerequisite-download-failed = Kan { $runtime } niet downloaden ({ $error }). Het installatieprogramma probeert winget op de doelcomputer.
# { $runtime } is a runtime's name. { $reason } is why the check failed, such as "it is signed by Contoso Ltd, not
# Microsoft".
core-warning-build-prerequisite-signature-failed = De download van { $runtime } heeft de handtekeningcontrole niet doorstaan, dus deze is niet meegeleverd: { $reason }. Het installatieprogramma probeert winget op de doelcomputer.
# { $runtime } is a runtime's name. { $error } is what went wrong.
core-warning-build-prerequisite-hash-failed = { $runtime } is gedownload, maar er kan geen hash van worden berekend ({ $error }). Het installatieprogramma probeert in plaats daarvan winget op de doelcomputer.
# { $service } is a driver's service name, such as AcmeFlt. { $error } is what went wrong.
core-warning-build-driver-copy-failed = Kan stuurprogrammabestand voor '{ $service }' niet bundelen: { $error }
core-warning-build-license-not-copied = Licentiegegevens kunnen niet worden gekopieerd (ontbreken of onleesbaar). Alleen vastgelegd als metagegevens.
core-warning-build-license-file-credential = Een licentiebestand lijkt op een referentie en is weggelaten.
# { $error } is what went wrong.
core-warning-build-license-store-walk-failed = Kan een deel van een licentiearchief niet opsommen: { $error }
core-warning-build-license-store-credential = Een bestand in een licentiearchief lijkt op een referentie en is weggelaten.
# { $error } is what went wrong.
core-warning-build-license-store-file-skipped = Bestand in licentiearchief overgeslagen: { $error }
