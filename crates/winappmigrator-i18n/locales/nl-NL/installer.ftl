# Words of the installer that every built installer carries, which runs on the PC an app is moved to and speaks its
# language, and of the uninstaller it leaves behind for Settings. Message names start with installer- for the
# installer's window, installer-cli- for its command line, and uninstaller- for the uninstaller.
#
# The command lines (installer-cli-, uninstaller-cli-) are always shown in English, as scripts read what they print.
# Translate them all the same. Keep their command-line flags, such as --apply and --target-root, exactly as they are.


## The installer's window: its title and header

installer-window-title = WinAppMigrator-installatieprogramma
# The window's title in the taskbar. { $app_name } is the name of the app it installs.
installer-window-title-app = { $app_name } installeren
installer-tagline = Installeert een app opnieuw die op een andere pc is vastgelegd
# What an app is called when its package gives it no name.
installer-unnamed-app = App

## When the installer's window cannot open. Each is a paragraph of a message box.

# { $error } is what went wrong, as Windows or the graphics driver says it.
installer-window-failed = Het venster van het installatieprogramma kan niet worden geopend: { $error }
installer-window-failed-without-one = Installeren kan ook zonder venster: voer dit bestand uit vanaf een opdrachtprompt als administrator met --apply, en met --target-root gevolgd door een map om zelf de locatie te kiezen.

## Checking the installer when it opens

installer-checking = Het installatieprogramma controleren…
installer-stub-no-package = Dit is een installatiestub van WinAppMigrator en deze bevat geen app-pakket. Maak een installatieprogramma met WinAppMigrator en voer het gemaakte .exe-bestand hier uit.
installer-check-stopped = De controle van het installatieprogramma is gestopt.
# { $error } is what went wrong, as Windows says it.
installer-locate-failed = Kan het installatieprogramma niet vinden: { $error }
# { $error } is what went wrong reading it.
installer-read-failed = Kan het installatieprogramma niet lezen: { $error }
# { $error } is why, such as that the app needs a newer Windows.
installer-cannot-be-used-here = Het pakket van dit installatieprogramma kan niet op deze pc worden gebruikt: { $error }

## The app, where it goes, and whether the installer is signed

installer-captured-from = Afkomstig van
installer-install-location = Installatielocatie
# The title of the folder picker.
installer-choose-location = De installatielocatie kiezen
installer-folder-has-files = Deze map bevat al bestanden. Bestanden met dezelfde naam worden vervangen en komen terug als je de app verwijdert.
installer-room-needed = Benodigde ruimte
# { $shortfall } says which drive is short of room, such as "C: needs up to 824.0 MB for the app's files, and has 300.0 MB free".
installer-room-short = Er is hier mogelijk niet genoeg ruimte: { $shortfall }. Het pakket kan op een ander station worden uitgepakt, maar de bestanden van de app moeten hier nog steeds passen.
# { $app_name } is the name of an app whose earlier install was cut short.
installer-interrupted = Een eerdere installatie van { $app_name } is nooit voltooid. Installeren maakt die eerst ongedaan.
# Beside "Installer", for whether it proves who made it.
installer-signature = Installatieprogramma
installer-signature-checking = Handtekening controleren…
installer-signature-unsigned = Niet ondertekend: uitgever onbekend
# { $publisher } is the name on the installer's certificate, such as "Fabrikam, Inc.".
installer-signature-signed-by = Ondertekend door { $publisher }
installer-signature-unnamed = Ondertekend door een naamloze uitgever
# { $reason } is Windows' own word for what is wrong, such as HashMismatch.
installer-signature-invalid = De handtekening van dit installatieprogramma is niet geldig ({ $reason }). Het is mogelijk gewijzigd nadat het is ondertekend.

## Why the install location cannot be used. Install and Preview Changes stay off while one of these is shown.

# { $app_name } is the name of the app being installed.
installer-location-app-already-there = { $app_name } is al in deze map geïnstalleerd, dus deze kopie kan hier niet worden geplaatst. Als je de app hier wilt installeren, verwijder je eerst de kopie op deze pc via Instellingen van Windows onder Apps en open je dit installatieprogramma daarna opnieuw. Of voer hierboven een nieuwe, lege map in.
# { $other_app } is the name of another app Windows lists as installed in the folder. { $app_name } is the name of the app being installed.
installer-location-other-app-there = { $other_app } is in deze map geïnstalleerd, dus { $app_name } kan hier niet worden geplaatst. Voer hierboven een nieuwe, lege map in. Of, als je { $other_app } niet meer nodig hebt, verwijder je die eerst via Instellingen van Windows onder Apps en open je dit installatieprogramma daarna opnieuw.
installer-location-has-files = Deze map bevat al bestanden die niet door WinAppMigrator zijn geïnstalleerd. Ze kunnen van een andere app zijn, dus er wordt niets overheen geïnstalleerd. Voer hierboven een nieuwe, lege map in.
# { $reason } is why, such as that it cannot be the Windows folder.
installer-location-refused = Deze map kan niet worden gebruikt: { $reason }. Voer hierboven een andere map in.

## What the install sets up beyond its files

installer-beyond-files = Wat dit naast bestanden instelt
# Under the heading above. It is part of what a person agrees to when they click Install.
installer-beyond-files-agreed = Dit gebeurt allemaal met administratorrechten. Als de installatie mislukt, wordt teruggezet wat is gewijzigd en krijg je te horen wat niet kon worden teruggezet. Je kunt de installatie later ook ongedaan maken via Instellingen, onder Geïnstalleerde apps. Bestanden die worden vervangen, komen dan terug, en runtimes die worden geïnstalleerd, blijven staan.
installer-beyond-files-none = Niets: er worden alleen bestanden gekopieerd, en instellingen in de eigen registersleutels van de app.
# { $problem } is why, such as that the app cannot run on this PC.
installer-cannot-go-here = Dit pakket kan niet op deze locatie worden geplaatst: { $problem }
installer-working-out = Bepalen wat dit op deze locatie zou instellen…
# Said as the reason after "This package cannot go to this location:".
installer-list-stopped = het bepalen van wat het zou doen, is gestopt
# A heading over a list of what else the package asked for and will not get. { $count } is how many.
installer-more-left-out =
    { $count ->
        [one] Nog { $count } item is weggelaten
       *[other] Nog { $count } items zijn weggelaten
    }

## Options

installer-options = Opties
installer-user-data = Gebruikersgegevens
installer-user-data-detail = Aan: instellingen, Documenten en Opgeslagen spellen gaan naar de gebruikelijke mappen van wie er is aangemeld. Uit: alles komt in de installatiemap terecht en er wordt niets ingesteld om van daaruit te worden uitgevoerd.
# { $account } is the name of the account on the other PC whose data this is.
installer-user-data-whose = Dit bevat de gegevens van { $account }.
installer-license = Licentie
installer-license-detail = Activerings- en licentiebestanden, en de registerwaarden waarin licenties zijn opgeslagen. Sommige moeten hier mogelijk opnieuw worden geactiveerd.

## Dependencies: what else the app needs, and how each is provided

installer-dependencies = Afhankelijkheden
installer-dependencies-detail = Wat deze app nodig heeft en hoe elk onderdeel wordt geleverd.
installer-dependencies-none = Er zijn geen extra runtime-afhankelijkheden gevonden.
# { $feature } is the name of an optional part of Windows, such as NetFx3.
installer-dependency-windows-feature = Windows-onderdeel: { $feature }
installer-dependency-included = Inbegrepen
# { $id } is the package's name in winget, such as Microsoft.VCRedist.2015+.x64.
installer-dependency-winget = Wordt geïnstalleerd met winget: { $id }
installer-dependency-manual = Handmatig installeren
installer-dependency-built-in = Ingebouwd in Windows
installer-dependency-ships-with-app = Wordt met de app meegeleverd

## Buttons, and what is said beside them

installer-install = Installeren
installer-preview = Voorbeeld van wijzigingen
installer-cancel = Annuleren
installer-close = Sluiten
installer-open-folder = Installatiemap openen
# { $error } is what went wrong, as Windows says it.
installer-open-folder-failed = Kan de map niet openen: { $error }
installer-location-blocks = Installeren en Voorbeeld van wijzigingen zijn uitgeschakeld totdat de installatielocatie kan worden gebruikt. De reden staat hierboven, onder de locatie.
installer-package-blocks = Installeren is uitgeschakeld omdat dit pakket niet op deze locatie kan worden geplaatst. De reden staat hierboven.
installer-list-changed = Wat dit hier zou doen, is gewijzigd, dus er is niets geïnstalleerd. Bekijk de lijst hieronder en klik opnieuw op Installeren als dit is wat je wilt.
installer-cancelled-early = Geannuleerd. Er is niets gewijzigd.

## How far an install has got, beside its progress bar

# { $step } is one of the steps below. { $done } and { $total } are how many of its items are done, and how many there are.
installer-step-counted = { $step }: { $done } van { $total }…
# { $step } is one of the steps below, when it does not count its items.
installer-step-uncounted = { $step }…
installer-step-getting-ready = Voorbereiden…
installer-step-cancelling = Annuleren na de huidige stap…
installer-step-putting-back = Terugzetten wat is gewijzigd
installer-step-unpacking = Uitpakken
installer-step-checking-package = Het pakket controleren
installer-step-recovering = Een eerdere installatie ongedaan maken die nooit is voltooid
# What a preview is doing, whatever step it is on.
installer-step-previewing = Bepalen wat er zou veranderen
installer-step-runtimes = Runtimes installeren
installer-step-files = Bestanden kopiëren
installer-step-services = Services instellen
installer-step-drivers = Stuurprogramma's instellen
installer-step-registry = Registerinstellingen schrijven
installer-step-license = Licentiegegevens herstellen
installer-step-tasks = Geplande taken instellen
installer-step-shortcuts = Snelkoppelingen maken
installer-step-environment = Omgevingsvariabelen instellen
installer-step-settings-entry = De app toevoegen aan Instellingen
installer-step-verifying = Controleren wat is geïnstalleerd
installer-step-undoing = Ongedaan maken

## When an install cannot go ahead or stops

installer-carries-no-package = Dit installatieprogramma bevat geen pakket.
installer-stopped-unexpectedly = De installatie is onverwacht gestopt.
# { $reason } is what went wrong, as the program says it.
installer-stopped-unexpectedly-because = De installatie is onverwacht gestopt: { $reason }. Als de installatie al was begonnen, maakt een installatieprogramma dat opnieuw wordt uitgevoerd, eerst ongedaan wat er is gedaan.
# { $error } is what went wrong reading it.
installer-package-read-failed = Kan het pakket niet lezen: { $error }
installer-no-programdata = De map ProgramData van deze pc is niet gevonden, dus er is niets geïnstalleerd: een installatie bewaart daar het installatieverslag waarmee ze ongedaan wordt gemaakt.
# { $error } is why, such as that there is nowhere only administrators can get into.
installer-staging-failed = Kan geen faseringsmap maken: { $error }
# { $error } is what went wrong unpacking it.
installer-unpack-failed = Kan het pakket niet uitpakken: { $error }
installer-staging-nowhere = er is geen plek om het pakket uit te pakken waar alleen administrators bij kunnen
# { $failure } is a folder and what went wrong there, such as "C:\ProgramData\WinAppMigrator: Access is denied.".
installer-staging-nowhere-look-at = er is geen plek om het pakket uit te pakken waar alleen administrators bij kunnen. Controleer vooral { $failure }

## What came of an install, or of a preview

installer-result = Installatieresultaat
installer-result-preview = Voorbeeld: er is nog niets gewijzigd
installer-result-installed = De app is geïnstalleerd en heeft de hashcontrole doorstaan.
installer-result-installed-without-left-out = De app is geïnstalleerd en heeft de hashcontrole doorstaan, zonder wat hieronder als weggelaten wordt vermeld.
installer-result-not-all-put-back = De installatie is mislukt en niet alles wat is gewijzigd, kon worden teruggezet. Wat er nog over is, staat hieronder. De volgende installatie probeert het opnieuw.
installer-result-cancelled-put-back = De installatie is geannuleerd en wat is gewijzigd, is teruggezet.
installer-result-failed-put-back = De installatie is mislukt en wat is gewijzigd, is teruggezet.
installer-result-not-clean = De installatie is niet zonder problemen voltooid. Bekijk de problemen hieronder.
installer-result-changes-put-back = Teruggezette wijzigingen
installer-result-changes-made = Aangebrachte wijzigingen
installer-result-changes-planned = Geplande wijzigingen
installer-result-warnings = Waarschuwingen
installer-result-errors = Fouten
installer-result-package-valid = Pakkethashes geldig
installer-result-package-invalid = Pakkethashes ONGELDIG
installer-result-files-valid = Geïnstalleerde bestanden geldig
installer-result-files-invalid = Geïnstalleerde bestanden ONGELDIG
installer-result-reboot = Voor een afhankelijkheid moet de pc opnieuw worden opgestart om de installatie te voltooien. Start Windows opnieuw op en start daarna de app.
# A heading over the list of what was changed. { $count } is how many.
installer-result-changes = Wijzigingen ({ $count })
# After the first part of a long list. { $count } is how many more there are.
installer-result-changes-more = en nog { $count }.
# Over the path of the install's log file.
installer-log = Logboek

## The installer's command line. Shown in English: scripts read it. A { $count } is plain digits, with no separators.

installer-cli-stub = WinAppMigrator-installatiestub — deze bevat geen app-pakket.
installer-cli-stub-how = Maak een installatieprogramma met de app WinAppMigrator en voer het gemaakte .exe-bestand uit op de doelcomputer.
installer-cli-usage-title = Zelfuitpakkend installatieprogramma van WinAppMigrator.
installer-cli-usage = Voer het uit zonder argumenten om het grafische installatieprogramma te openen, of:
installer-cli-usage-preview = voorbeeld van herinstallatie: --dry-run [--target-root <MAP>]
installer-cli-usage-apply = herinstalleren:               --apply [--target-root <MAP>]
installer-cli-no-package = installatieprogramma bevat geen ingesloten pakket
installer-cli-no-programdata = de map ProgramData van deze pc is niet gevonden, dus er is niets geïnstalleerd: een installatie bewaart daar het installatieverslag waarmee ze ongedaan wordt gemaakt
# { $path } is the folder asked for, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-folder-has-files = { $path } kan niet worden gebruikt: { $problem }. Geef --force alleen op als je weet dat het van deze app is
installer-cli-no-location = dit pakket vermeldt niet waar de app stond, dus geef een --target-root op
# { $path } is where the app was on the other PC, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-location-refused = de app van dit pakket stond in { $path }, en dat kan niet worden gebruikt: { $problem }. Geef een --target-root op
# { $summary } is how much room it needs where, such as "up to 824.0 MB on C: (356.0 GB free)".
installer-cli-room = Ruimte: { $summary }
# { $note } is something to know about the install.
installer-cli-note = Opmerking: { $note }
# { $count } is how many things it sets up beyond its files.
installer-cli-sets-up = Stelt in: { $count }
installer-cli-applied = Herinstallatie toegepast.
installer-cli-previewed = Voorbeeld van herinstallatie gemaakt.
# { $count } is how many changes the install logged.
installer-cli-changes-logged = Vastgelegde wijzigingen: { $count }
# { $count } is how many warnings.
installer-cli-warnings = Waarschuwingen: { $count }
# { $count } is how many errors.
installer-cli-errors = Fouten: { $count }
# { $count } is how many things the package asked for and did not get.
installer-cli-refused = Geweigerd: { $count }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-package-valid = Pakkethashes geldig: { $valid }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-reinstall-valid = Hashes van herinstallatie geldig: { $valid }
# { $id } is the install's id, such as wam-0f3a….
installer-cli-install-id = Installatie-id: { $id }
installer-cli-put-back = De installatie is mislukt, dus wat is gewijzigd, is teruggezet.
# { $path } is the log file.
installer-cli-log = Logboek: { $path }
# { $path } is the log file. Printed before an error.
installer-cli-error-log = logboek: { $path }
# { $error } is what went wrong.
installer-cli-error = fout: { $error }

## The uninstaller's window

uninstaller-window-title = Verwijderen
# The window's title, and its heading. { $app_name } is the name of the app it uninstalls.
uninstaller-window-title-app = { $app_name } verwijderen
uninstaller-tagline = Maakt ongedaan wat het installatieprogramma heeft gedaan
uninstaller-reading = Het installatieverslag lezen…
uninstaller-stopped-unexpectedly = Het verwijderen is onverwacht gestopt.
uninstaller-what-will-be-removed = Wat wordt verwijderd
uninstaller-install-location = Installatielocatie
uninstaller-files = Bestanden
uninstaller-registry-values = Registerwaarden
uninstaller-services = Services
uninstaller-drivers = Stuurprogramma's
uninstaller-scheduled-tasks = Geplande taken
uninstaller-environment = Omgeving
uninstaller-plan-stays = Bestanden die je hebt toegevoegd of gewijzigd, blijven staan, samen met de mappen waarin ze staan.
# { $count } is how many files the install replaced.
uninstaller-plan-coming-back =
    { $count ->
        [one] { $count } bestand dat deze installatie heeft vervangen, komt terug of wordt naast een bestand geplaatst dat sindsdien is gewijzigd.
       *[other] { $count } bestanden die deze installatie heeft vervangen, komen terug of worden naast een bestand geplaatst dat sindsdien is gewijzigd.
    }
# { $runtimes } is a list of runtimes, such as "Microsoft Visual C++ Redistributable, .NET Desktop Runtime 8".
uninstaller-plan-runtimes = Runtimes die zijn geïnstalleerd, blijven staan omdat andere programma's ze mogelijk gebruiken: { $runtimes }.
uninstaller-options = Opties
uninstaller-remove-user-data = Ook gebruikersgegevens en licentiegegevens verwijderen
# { $count } is how many items of the app's settings, data, and license there are.
uninstaller-remove-user-data-detail =
    { $count ->
        [one] { $count } item van de instellingen, gegevens en licentie van de app. Het blijft staan, tenzij dit is ingeschakeld. Wat het heeft vervangen, komt terug, of wordt ernaast geplaatst als het sindsdien is gewijzigd.
       *[other] { $count } items van de instellingen, gegevens en licentie van de app. Ze blijven staan, tenzij dit is ingeschakeld. Wat ze hebben vervangen, komt terug, of wordt ernaast geplaatst als ze sindsdien zijn gewijzigd.
    }
uninstaller-uninstall = Verwijderen
uninstaller-cancel = Annuleren
uninstaller-close = Sluiten
# { $done } and { $total } are how many of the install's items are undone, and how many there are.
uninstaller-step-counted = De installatie ongedaan maken: { $done } van { $total }…
uninstaller-step = De installatie ongedaan maken…
uninstaller-done = De app is verwijderd.
uninstaller-done-except = De app is verwijderd, behalve wat hieronder wordt vermeld.
uninstaller-waits-for-drive = Een deel ervan staat op een station dat niet is aangesloten. Sluit het station aan en verwijder de app opnieuw via Instellingen.
uninstaller-not-all-undone = Niet alles kon ongedaan worden gemaakt. Sluit wat het gebruikt en verwijder de app opnieuw via Instellingen.
# Over the path of the uninstall's log file.
uninstaller-log = Logboek
uninstaller-undone = Ongedaan gemaakt
uninstaller-left-in-place = Blijft staan
uninstaller-could-not-be-undone = Kon niet ongedaan worden gemaakt
# After the first part of a long list. { $count } is how many more there are.
uninstaller-and-more = en nog { $count }.
# Said as the reason when there is no record of the install to read.
uninstaller-no-programdata = de map ProgramData van deze pc is niet gevonden, dus er is geen installatieverslag om te lezen

## When the uninstaller's window cannot open. Each is a paragraph of a message box, followed by a command to type.

uninstaller-window-failed-title = WinAppMigrator-verwijderprogramma
# { $error } is what went wrong, as Windows or the graphics driver says it.
uninstaller-window-failed = Het venster van het verwijderprogramma kan niet worden geopend: { $error }
uninstaller-window-failed-quiet = Verwijderen kan ook zonder venster, vanaf een opdrachtprompt als administrator:
# Stands in for this program's own path in that command, if it cannot be found.
uninstaller-this-program = dit programma

## The uninstaller's command line (--quiet). Shown in English: scripts read it.

# { $app_name } is the name of the app.
uninstaller-cli-uninstalling = { $app_name } wordt verwijderd.
# { $path } is the log file.
uninstaller-cli-log = Logboek: { $path }
# { $error } is what went wrong.
uninstaller-cli-error = fout: { $error }
