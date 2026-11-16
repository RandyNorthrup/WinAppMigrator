# Words the core writes for people while it builds a package from an installed app: what a package records for an
# installer to show later, errors shown in the app, the list of what goes in and what is left out, opening a disk
# image or another PC over the network, and what an installer says when the package it carries cannot be read.
# Message names start with core-.


## Scanning an app, before a package is built. Shown in the app when a scan cannot go on.

# { $path } is the app's program, such as D:\Program Files\Acme\Acme.exe, on a drive taken from another PC.
core-scan-program-not-on-the-drive = Ścieżka programu { $path } prowadzi przez link lub nie znajduje się na podłączonym dysku, więc program nie został odczytany.
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-not-on-this-pc = Program { $path } nie znajduje się na dysku tego komputera lub ma nazwę, która otworzyłaby coś innego, więc nie został odczytany.
# { $path } is the app's program.
core-scan-program-someone-elses = Program { $path } znajduje się w folderach innego konta lub nie należy do tego konta, a zapis instalacji tej aplikacji nie wskazuje, że powinien się tam znajdować, więc nie został odczytany.
# { $app_name } is the app's name, as the list of apps calls it.
core-scan-no-executable = nie można znaleźć pliku wykonywalnego aplikacji { $app_name }. Sterownik, środowisko uruchomieniowe lub dodatek mogą nie mieć własnego programu, który można spakować.
# { $path } is the file that was picked.
core-scan-not-a-file = element wybrany do analizy nie jest plikiem: { $path }
# Names something left out of a package, in a list of what was left out and why. { $name } is the variable's name,
# such as ACME_LICENSE_KEY.
core-scan-environment-variable-item = Zmienna środowiskowa { $name }

## Folders and parts that might also be the app's, offered in a list to tick. Each says why it is offered.

core-near-starts-with-app-name = Nazwa zaczyna się od nazwy tej aplikacji.
core-near-has-app-name = Nazwa zawiera nazwę tej aplikacji.
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-starts-with-publisher = Nazwa zaczyna się od nazwy wydawcy: { $publisher }.
# { $folder } is a folder's name, such as Program Files.
core-near-hidden-part-beside = Ukryta część, której nie ma na liście aplikacji, zainstalowana obok tej aplikacji w folderze { $folder }.
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-hidden-part-publisher = Ukryta część, której nie ma na liście aplikacji, od tego samego wydawcy: { $publisher }.

## What a scan found, as a person chooses from it: the folders that go in, and what is left out, and why

# Why a folder cannot be left out, beside its number of files.
core-footprint-locked-app-folder = Te pliki zawsze trafiają do instalatora razem z programem głównym
core-footprint-locked-main-program = Zawiera program główny, który zawsze trafia do instalatora
core-footprint-locked-needed = Zawiera elementy, bez których program główny się nie uruchomi
core-footprint-locked-font = Trafia do instalatora razem ze swoją czcionką, wymienioną w sekcji Powiązania z systemem Windows
# Where a folder is. "AppData" and "ProgramData" are the names of Windows folders; keep them as they are.
core-footprint-place-roaming = Ustawienia w profilu (AppData)
core-footprint-place-local = Ustawienia dla tego komputera w profilu (AppData)
core-footprint-place-program-data = Wspólne dla wszystkich użytkowników komputera (ProgramData)
core-footprint-place-documents = W folderze Dokumenty
core-footprint-place-saved-games = W folderze Zapisane gry
core-footprint-place-public-documents = W folderze Dokumenty publiczne
core-footprint-place-app-folder = W folderze aplikacji
# { $folder } is a folder, such as C:\Users\Sam\AppData\Roaming.
core-footprint-place-in = W folderze { $folder }
# A row for the files directly in the app's folder. { $folder } is that folder's name, such as Acme.
core-footprint-files-directly-in = Pliki bezpośrednio w folderze { $folder }
# Headings over what is left out, and why each item is.
core-footprint-licenses-left-out = Nie ma ich w tym instalatorze, ponieważ opcja Dane licencji i aktywacji jest wyłączona
core-footprint-font-stays-behind = Nie zostanie przeniesiona.
core-footprint-drivers-left-out = Sterowniki należące do systemu Windows lub dostawcy, których nie można rozpowszechniać, więc zainstaluj je ponownie na drugim komputerze
core-footprint-driver-reinstall = Zainstaluj go ponownie z własnego pakietu dostawcy.
# Why something else is left out along with what the person left out. { $service } is a service's name, such as AcmeSync.
core-footprint-needs-left-out = wymaga usługi { $service }, która jest pominięta
core-footprint-told-to-use-left-out = ma używać czegoś z folderu, który został pominięty
core-footprint-file-in-left-out = jego plik znajduje się w pominiętym folderze
core-footprint-program-in-left-out = program, którego dotyczy, znajduje się w pominiętym folderze
core-footprint-names-left-out = wskazuje pominięty folder
# Why something is left out, as the installer tells whoever installs it.
core-footprint-builder-left-it-out = osoba, która utworzyła ten instalator, pominęła ten element
# Something the person left out, by kind and name, such as "Service Acme Updater". { $kind } is the kind, such as
# Service or Scheduled Task; { $title } is its name.
core-footprint-left-out-item = { $kind } { $title }

## Building a package and an installer from a scan. Shown in the app when a build cannot go on.

core-package-scan-of-another-app = ta analiza dotyczy innej aplikacji niż tworzona, więc niczego nie utworzono
core-package-scan-brought-in-more = ta analiza zawiera dane, których dołączenia nie wybrano dla tego instalatora, więc niczego nie utworzono
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-package-program-gone = program { $path } już nie istnieje, więc niczego nie utworzono. Przeanalizuj aplikację ponownie.
core-package-needs-stub = samorozpakowujące się archiwum wymaga modułu startowego instalatora
# { $reason } is what went wrong.
core-package-staged-file-failed = nie można dodać pliku z folderu przejściowego do pakietu: { $reason }
# { $shortfalls } says which drives are short of room and by how much, in sentences of its own.
core-package-not-enough-room = za mało miejsca na utworzenie tego instalatora, więc niczego nie odczytano ani nie zapisano. { $shortfalls }. Zwolnij miejsce na tym dysku lub wybierz folder wyjściowy na innym dysku.
# { $path } is a file's path, on a drive taken from another PC.
core-package-not-read-off-the-drive = ścieżka { $path } prowadzi przez link lub nie znajduje się na podłączonym dysku, więc nie została odczytana
# { $path } is a file's path.
core-package-not-read-not-local = plik { $path } nie znajduje się na dysku tego komputera, więc nie został odczytany
# { $path } is a file's path.
core-package-not-read-other-account = ścieżka { $path } prowadzi do folderów innego konta, więc nie została odczytana
# { $path } is a file's path, as the PC it came from called it.
core-package-not-on-the-drive = plik { $path } nie znajduje się na podłączonym dysku, więc nie został odczytany
# { $path } is a file's path.
core-package-backup-copy-no-parent = nie można skopiować w trybie kopii zapasowej pliku bez folderu nadrzędnego: { $path }
# { $path } is a file's path.
core-package-backup-copy-destination-no-parent = miejsce docelowe kopiowania w trybie kopii zapasowej nie ma folderu nadrzędnego: { $path }
# robocopy is the name of a Windows command; keep it as it is. { $code } is how it ended, such as Some(8).
core-package-robocopy-failed = kopiowanie w trybie kopii zapasowej za pomocą polecenia robocopy nie powiodło się (kod zakończenia { $code })
# Stands for a folder where a file's name would be, as in "I/O failed for temporary package folder: …".
core-package-temporary-folder = tymczasowy folder pakietu
# Stands for the installer being written where a file's name would be, as in "I/O failed for the installer: …".
core-package-the-installer = instalator
# { $path } is a file's path, or its name in the package. { $error } is what went wrong with it.
core-package-path-error = { $path }: { $error }
# { $name } is a file's name in the package, such as app/bin/App.exe.
core-package-same-place = inny plik zajmuje to samo miejsce w pakiecie: { $name }
# { $name } is a file's name in the package.
core-package-changed-in-protected-folder = plik { $name } zmienił się w chronionym folderze podczas tworzenia pakietu
core-package-worker-died = wątek kompresujący pliki nieoczekiwanie się zatrzymał, więc nie zapisano żadnego instalatora
# { $path } is a file's path.
core-package-link-not-read = element { $path } jest linkiem, więc nie został odczytany
# { $path } is a file's path. { $error } is what went wrong.
core-package-not-compressed = nie można skompresować pliku { $path }: { $error }
# { $path } is a file's path. { $reason } is what went wrong.
core-package-not-read-back = nie można ponownie odczytać pliku { $path }: { $reason }

## Where an app's files are, and where an installer may write them. Shown when a path is refused.

# { $path } is a folder, such as C:\Program Files\Acme.
core-discovery-root-not-full-path = folder, w którym ma nastąpić zapis, musi być pełną ścieżką: { $path }
# { $path } is a folder's path.
core-discovery-root-not-a-folder = element, w którym ma nastąpić zapis, nie jest folderem: { $path }
# { $path } is a file's path.
core-discovery-escapes-root = ścieżka wykracza poza zamierzony katalog główny: { $path }
# { $path } is the link's path.
core-discovery-refusing-to-write-through-link = odmowa zapisu przez link: { $path }
# { $path } is the link's path.
core-discovery-refusing-link = odmowa przejścia przez link: { $path }
# { $error } is what Windows said went wrong.
core-discovery-refusing-link-because = odmowa przejścia przez link: { $error }
# { $path } is the link's path.
core-discovery-link-to-somewhere-else = element { $path } jest linkiem do innego miejsca, a inni mogą zmienić, dokąd prowadzi. Użyj miejsca, do którego prowadzi ten link
# { $path } is a folder's path.
core-discovery-no-parent-folder = element { $path } nie ma folderu nadrzędnego
# { $path } is the install location as it was given. Keep the quotes, or use your language's own.
core-discovery-install-location-not-full-path = lokalizacja instalacji musi być pełną ścieżką: „{ $path }”
# { $path } is the install location.
core-discovery-install-location-a-link = lokalizacja instalacji jest linkiem do innego miejsca: { $path }
# { $path } is a file's path.
core-discovery-device-name = ścieżka zawiera nazwę zarezerwowaną dla urządzenia systemu Windows: { $path }
# { $path } is a file's path.
core-discovery-device-name-on-older-windows = ścieżka { $path } ma nazwę, którą ta wersja systemu Windows otwiera jako urządzenie, a nie jako plik, więc niczego w niej nie zapisano. Tę aplikację można zainstalować w systemie Windows 11
# { $path } is a file's path.
core-discovery-unsafe-path = niebezpieczna ścieżka: { $path }
# { $path } is a file's path.
core-discovery-no-package-path = plik nie ma ścieżki względnej w pakiecie: { $path }

## Writing an installer, and reading the package inside one. Most of these are shown by an installer, on the PC it
## runs on, when it cannot read what it carries. "The package" is the app's files and settings that an installer
## carries; "the stub" is the program part of an installer, before the package; "the data file" is the .wamdata file
## that holds the package of a very large installer, beside it.

# Stands for the package where a file's name would be, as in "I/O failed for installer package: …".
core-archive-installer-package = pakiet instalatora
# Stands for the installer program built into WinAppMigrator, where a file's name would be.
core-archive-embedded-stub = osadzony moduł startowy instalatora
# { $index } is the number of a file in the package. { $reason } is what went wrong.
core-archive-entry-unreadable = nie można odczytać pozycji { $index } pakietu: { $reason }
# { $name } is a file's name inside the package, such as app/bin/App.exe.
core-archive-entry-unsafe-path = pozycja archiwum zip ma niebezpieczną ścieżkę: { $name }
# { $name } is a file's name inside the package.
core-archive-holds-more = plik { $name } zawiera więcej danych, niż deklaruje
# { $name } is a file's name inside the package.
core-archive-name-too-long = ta nazwa jest zbyt długa dla archiwum zip: { $name }
core-archive-changed-while-copied = pakiet zmienił się podczas dołączania go za modułem startowym, więc nie zapisano żadnego instalatora
core-archive-stub-not-a-program = część tego instalatora poprzedzająca pakiet nie jest programem
core-archive-damaged = ten instalator jest uszkodzony lub został zmieniony po utworzeniu, więc niczego nie odczytano z jego pakietu. Skopiuj go ponownie z miejsca, w którym został utworzony.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-missing = ten instalator przechowuje swój pakiet w pliku o nazwie { $name } w tym samym folderze, a tego pliku tam nie ma. Oba pliki muszą pozostać razem: skopiuj tutaj plik { $name } z miejsca, w którym utworzono instalator, a następnie uruchom go ponownie.
# { $name } is the data file's name, such as App_installer.wamdata. { $error } is what Windows said went wrong.
core-archive-data-file-unopened = nie można otworzyć pliku danych tego instalatora, { $name }: { $error }. Oba pliki muszą pozostać razem, w folderze, który ten komputer może odczytać.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-a-link = element { $name } jest linkiem lub folderem, a plik danych tego instalatora musi być osobnym plikiem znajdującym się tuż obok niego, więc niczego z niego nie odczytano. Skopiuj tutaj rzeczywisty plik.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-wrong = plik { $name } nie jest plikiem danych tego instalatora albo jest uszkodzony lub został zmieniony po utworzeniu, więc niczego z niego nie odczytano. Skopiuj ponownie oba pliki z miejsca, w którym zostały utworzone.
core-archive-stub-signed = moduł startowy instalatora jest podpisany; musi pozostać niepodpisany, więc podpisz zamiast niego gotowy instalator
# { $size } is a size, such as 4.2 GB.
core-archive-too-big-to-run = instalator miałby rozmiar { $size }, a system Windows nie może uruchomić pliku wykonywalnego o rozmiarze 4 GB lub większym; niczego nie zapisano
core-archive-outside-signature = instalator zawiera dane poza swoim podpisem i mógł zostać zmodyfikowany
core-archive-footer-length = długość stopki samorozpakowującego się archiwum jest nieprawidłowa
core-archive-seek-outside = próba przejścia do pozycji poza pakietem
# { $path } is a file's path.
core-archive-unsafe-package-path = niebezpieczna ścieżka w pakiecie: { $path }
# { $path } is a file's path.
core-archive-empty-package-path = pusta ścieżka w pakiecie: { $path }

## Opening a disk image of another PC (a .vhd or .vhdx file) to read apps from it

# { $path } is the disk image file, such as D:\Backups\OldPC.vhdx.
core-image-not-attachable = plik { $path } nie jest obrazem dysku, który system Windows może dołączyć (.vhd lub .vhdx)
core-image-needs-administrator = dołączenie obrazu dysku wymaga uprawnień administratora
# { $path } is the disk image file the person picked.
core-image-not-a-file = element { $path } nie jest plikiem
# { $path } is the disk image file. "Edit Disk" is the name of a Hyper-V Manager command.
core-image-differencing = plik { $path } jest dyskiem różnicowym, który zawiera tylko zmiany wprowadzone od utworzenia jego dysku nadrzędnego, a system Windows otworzyłby również tamten dysk, gdziekolwiek według zapisu się znajduje. Najpierw scal go z tamtym dyskiem za pomocą polecenia Edytuj dysk w Menedżerze funkcji Hyper-V, a następnie wybierz tamten dysk
# { $path } is the disk image file.
core-image-unreadable = plik { $path } nie jest plikiem .vhd ani .vhdx, który można odczytać
# ProgramData is the name of a Windows folder; keep it as it is.
core-image-no-program-data = nie można odnaleźć folderu ProgramData na tym komputerze
# The reason given in core-image-could-not-attach when Windows already has the image open.
core-image-already-attached = system Windows ma już ten obraz dołączony, a to, co go dołączyło, może w nim zapisywać dane. Najpierw go odłącz albo wybierz jego dysk jako podłączony dysk
# { $path } is the disk image file. { $reason } is why, such as core-image-already-attached or what Windows said.
core-image-could-not-attach = nie można dołączyć obrazu { $path }: { $reason }
# { $path } is the disk image file.
core-image-no-windows-volume = obraz { $path } nie zawiera woluminu z systemem Windows, który można odczytać. Wolumin zaszyfrowany funkcją BitLocker trzeba najpierw odblokować.
# Given as the reason when Windows failed without saying anything.
core-image-windows-did-not-say = system Windows nie podał przyczyny

## Reading another PC over the network, through the client running on it. Shown as why something on that PC
## could not be read. "The client" is WinAppMigrator Client, the small program on the other PC.

core-remote-too-slow = drugi komputer zbyt długo nie odpowiadał
core-remote-closed = połączenie z drugim komputerem jest zamknięte
core-remote-failed = połączenie z drugim komputerem nie powiodło się
core-remote-answered-something-else = klient udzielił innej odpowiedzi niż oczekiwana
# { $name } is the PC's name or address as the person typed it, such as OLDPC or 192.168.1.20.
core-remote-not-found = nie znaleziono komputera { $name } w sieci
core-remote-did-not-say-what-it-is = klient nie podał, czym jest
core-remote-ordinary-names-only = można pytać tylko o ścieżkę poniżej katalogu głównego drugiego komputera, złożoną ze zwykłych nazw
core-remote-more-but-none = drugi komputer podał, że folder zawiera więcej elementów, ale nie przesłał żadnego z nich
# { $count } is the most things one folder may hold, such as 250,000.
core-remote-too-many-listed = drugi komputer wymienił w jednym folderze więcej niż { $count } elementów
core-remote-sent-too-much = drugi komputer przesłał większą część pliku, niż o nią poproszono
core-remote-name-not-text = nie można pytać o nazwę, która nie jest tekstem
core-remote-below-root-only = można pytać tylko o ścieżkę poniżej katalogu głównego drugiego komputera
core-remote-file-ended-early = plik na drugim komputerze skończył się przedwcześnie
core-remote-link-not-followed = za linkami na innym komputerze nie można podążać
core-remote-only-files-opened = na innym komputerze można otwierać tylko pliki
core-remote-listing-said-nothing = lista zawartości folderu nie zawierała informacji o tym elemencie
core-remote-change-time-unknown = czas modyfikacji jest nieznany
core-remote-seek-before-start = próba przejścia do pozycji przed początkiem
# { $size } is how many bytes a file is; { $limit } is the most that were expected. Both are numbers.
core-remote-too-large =
    { $size ->
        [one] ma { $size } bajt, a oczekiwano najwyżej { $limit }
        [few] ma { $size } bajty, a oczekiwano najwyżej { $limit }
        [many] ma { $size } bajtów, a oczekiwano najwyżej { $limit }
       *[other] ma { $size } bajtów, a oczekiwano najwyżej { $limit }
    }

## Checking that a download is signed by Microsoft. Each is given as the reason a check failed.

# { $error } is what Windows said went wrong.
core-signature-no-powershell = nie można uruchomić programu PowerShell: { $error }
# { $error } is what went wrong reading PowerShell's answer.
core-signature-unreadable = nie można odczytać podpisu: { $error }
# { $signer } is who signed the file, such as Contoso Ltd.
core-signature-not-microsoft = plik jest podpisany przez { $signer }, a nie przez firmę Microsoft
core-signature-not-microsoft-unnamed = plik jest podpisany przez kogoś innego, a nie przez firmę Microsoft
# { $status } is Windows' own word for the signature's state, such as HashMismatch or NotSigned, which is not translated.
core-signature-status = stan podpisu to { $status }

## Warnings a package records when it is built from an app, which an installer shows later, maybe on another PC, in
## that PC's language. "This app" is the app being packaged.

core-warning-build-program-needs-unreadable = Nie można odczytać, czego ten program potrzebuje od systemu Windows, więc nie zostanie sprawdzone, czy może działać na drugim komputerze.
core-warning-build-external-manifest = Wykryto zewnętrzny manifest aplikacji; interpretacja zależności side-by-side została odłożona.
core-warning-build-non-exe = Wybrany plik nie ma rozszerzenia .exe; analiza formatu PE będzie kontynuowana.
core-warning-build-stand-in-program = Nie znaleziono zwykłego pliku wykonywalnego aplikacji, więc zastępuje go ten plik. Wyniki dotyczące architektury i zależności opisują ten plik i mogą nie opisywać aplikacji.
core-warning-build-other-apps-files-unknown = Nie wszystkie informacje Instalatora Windows o innych aplikacjach dało się odczytać, więc folder zapisany dla tej aplikacji może zawierać pliki innej aplikacji, a nic tutaj o tym nie poinformuje.
core-warning-build-app-folder-has-another-apps-file = Według Instalatora Windows w folderze programu tej aplikacji znajduje się plik innej aplikacji. Zostanie on dołączony, ponieważ cały ten folder jest przenoszony.
# "App Paths" is the name of a Windows registry key; keep it as it is.
core-warning-build-app-paths-folder-left-out = Wpis App Paths tej aplikacji wskazuje, że szuka ona plików w folderze, który do niej nie należy, więc ten folder nie zostanie dołączony. Aplikacja może się nie uruchomić na komputerze bez jego zawartości.
core-warning-build-uninstaller-folder-left-out = Folder dezinstalatora tej aplikacji został pominięty, ponieważ według Instalatora Windows znajduje się w nim plik innej aplikacji.
# { $why } is never shown: it chooses the reason. Translate each reason as the end of the sentence.
core-warning-build-recorded-folder-left-out =
    Element zapisany dla tej aplikacji wskazuje folder, który został pominięty, ponieważ { $why ->
        [unreadable] jego ścieżka wychodzi poza dysk lub prowadzi przez link, więc nie można go bezpiecznie odczytać
        [missing] nie istnieje
        [link] prowadzi przez link
        [short-name] jest zapisany pod skróconą nazwą, a nie pod rzeczywistą
        [another-account] znajduje się w folderach innego konta
        [one-account] ta aplikacja została zainstalowana dla jednego konta, a ten folder nie należy do tego konta
        [shared] zawiera elementy współdzielone lub jest częścią systemu Windows
        [generic-name] jego nazwa nie wskazuje, do kogo należy
        [publisher] jego nazwa odpowiada tylko wydawcy, więc może zawierać inne produkty tego wydawcy
        [holds-the-app] zawiera własny folder tej aplikacji i może zawierać więcej
        [another-apps-name] ma nazwę innej aplikacji
        [another-apps-record] jego zapis instalacji należy do innej aplikacji
        [in-another-apps-folder] znajduje się w folderze innej aplikacji
       *[another-apps] należy do innej aplikacji
    }.
core-warning-build-app-folder-holds-another-app = Według zapisów inna aplikacja na tym komputerze znajduje się w folderze tej aplikacji i zostanie dołączona razem z nią, ponieważ cały ten folder jest przenoszony.
# "Program Files (x86)" is the name of a Windows folder; keep it as Windows calls it.
core-warning-build-from-32-bit-windows = Ta aplikacja pochodzi z 32-bitowego systemu Windows. Jej klucze rejestru są rozmieszczone tak, jak 64-bitowy system Windows przechowuje klucze aplikacji 32-bitowych, a instalator proponuje dla niej folder Program Files (x86).
# { $dll } is a file's name, such as MSVCP140D.dll.
core-warning-build-debug-runtime = Program wymaga pliku { $dll }, czyli wersji debugowania środowiska uruchomieniowego Visual C++. Firma Microsoft nie udostępnia dla takich plików instalatora i może nie zezwalać na ich rozpowszechnianie, więc program nie uruchomi się na komputerze bez programu Visual Studio.
# { $program } is a program's file name, such as Tool.exe.
core-warning-build-single-file-program = Program { $program } to program .NET spakowany do jednego pliku, który nie określa, jakiej wersji platformy .NET potrzebuje, więc żadna nie została dla niego zaplanowana. Jeśli wymaga którejś z nich, nie uruchomi się na komputerze, na którym jej nie ma.
# { $count } is how many of the app's programs.
core-warning-build-programs-not-inspected =
    { $count ->
        [one] { $count } program aplikacji nie został sprawdzony pod kątem wymaganych środowisk uruchomieniowych, ponieważ aplikacja ma tak wiele programów lub są one tak duże.
        [few] { $count } programy aplikacji nie zostały sprawdzone pod kątem wymaganych środowisk uruchomieniowych, ponieważ aplikacja ma tak wiele programów lub są one tak duże.
        [many] { $count } programów aplikacji nie zostało sprawdzonych pod kątem wymaganych środowisk uruchomieniowych, ponieważ aplikacja ma tak wiele programów lub są one tak duże.
       *[other] { $count } programów aplikacji nie zostało sprawdzonych pod kątem wymaganych środowisk uruchomieniowych, ponieważ aplikacja ma tak wiele programów lub są one tak duże.
    }
# { $runtime } is a runtime's name, such as Java Runtime Environment.
core-warning-build-runtime-no-installer = Aplikacja wymaga środowiska uruchomieniowego { $runtime }, a nie ma tu dla niego instalatora, więc żaden nie zostanie uruchomiony. Zainstaluj je samodzielnie na drugim komputerze.
# { $why } is why something was left out, such as "its name says what it holds".
core-warning-build-left-out-of-package = Pominięto w pakiecie: { $why }.
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-missing-dependency = Nie można rozpoznać zależności: { $name }
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-dependency-only-here = Plik { $name } został znaleziony tylko w ścieżce wyszukiwania tego komputera i nie należy do aplikacji, więc nie zostanie dołączony. Aplikacja może się nie uruchomić na komputerze, na którym go nie ma.
core-warning-build-driver-detected = Wykryto sterownik lub składnik jądra. Sterownik należący do aplikacji jest dołączany i ponownie rejestrowany; sterownik dostawcy lub wbudowany w system nie jest rozpowszechniany i należy go ponownie zainstalować z podpisanego pakietu dostawcy.
core-warning-build-near-no-longer-offered = Element zaznaczony na liście elementów, które mogą również należeć do tej aplikacji, nie znajduje się już na tej liście, więc nie został dodany. Przejrzyj listę ponownie.
# { $app_name } is the name of a part of the app, as the list of apps calls it. { $why } is never shown: it chooses the
# reason. Translate each reason as the end of the sentence.
core-warning-build-near-part-left-out =
    Pominięto element, który według aplikacji { $app_name } do niej należy: { $why ->
        [unreadable] jego ścieżka wychodzi poza dysk lub prowadzi przez link, więc nie można go bezpiecznie odczytać
        [missing] nie istnieje
        [link] prowadzi przez link
        [short-name] jest zapisany pod skróconą nazwą, a nie pod rzeczywistą
        [another-account] znajduje się w folderach innego konta
        [one-account] ta aplikacja została zainstalowana dla jednego konta, a ten element nie należy do tego konta
        [shared] zawiera elementy współdzielone lub jest częścią systemu Windows
        [generic-name] jego nazwa nie wskazuje, do kogo należy
        [publisher] jego nazwa odpowiada tylko wydawcy, więc może zawierać inne produkty tego wydawcy
        [holds-the-app] zawiera własny folder tej aplikacji i może zawierać więcej
        [another-apps-name] ma nazwę innej aplikacji
        [another-apps-record] jego zapis instalacji należy do innej aplikacji
        [in-another-apps-folder] znajduje się w folderze innej aplikacji
       *[another-apps] należy do innej aplikacji
    }.
# { $app_name } is the name of a part of the app, as the list of apps calls it.
core-warning-build-near-keys-left-out = Pominięto klucze rejestru wymienione w zapisie instalacji aplikacji { $app_name }, ponieważ ten zapis wskazuje również elementy innej aplikacji.
# "Windows keeps what an app wrote for one account" is Windows' VirtualStore folder.
core-warning-build-virtual-store-program = Program znajdował się w folderze, w którym system Windows przechowuje dane zapisane przez aplikację dla jednego konta. System Windows nigdy nie umieszcza tam programów, więc umieściło go tam coś innego. Program został pominięty.
# { $path } is a file or folder, such as C:\Users\Sam\AppData\Roaming\Acme.
core-warning-build-file-left-out = Pominięto element { $path }: prowadzi przez link na podłączonym dysku lub należy do innego konta niż to, którego dane są dołączane.
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as D:\Sam\Documents.
core-warning-build-user-folder-not-on-drive = Folder { $folder } konta { $user } znajdował się w lokalizacji { $path }, która nie jest na tym dysku, więc jego zawartość nie została dołączona.
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as \\server\home\Sam\Documents.
core-warning-build-user-folder-not-on-this-pc = Folder { $folder } konta { $user } znajdował się w lokalizacji { $path }, która nie jest na dysku tego komputera, więc jego zawartość nie została dołączona.
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-base-unreadable = Nie można odczytać folderu { $path } ({ $error }); przechowywane w nim dane aplikacji nie zostały dołączone.
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-folder-unreadable = Nie można odczytać folderu { $path } ({ $error }); może brakować zagnieżdżonych w nim danych aplikacji.
# { $error } is what went wrong.
core-warning-build-entry-unreadable = Nie można odczytać elementu systemu plików: { $error }
# { $count } is how many links. { $names } lists some of them. { $more } is how many more there are, which are not
# named; when it is 0, the sentence ends after the list.
core-warning-build-links-not-followed =
    { $count ->
        [one] W tym folderze pominięto { $count } link lub nazwę, która otworzyłaby coś innego, więc to, co wskazuje, nie trafiło do instalatora: { $names }
        [few] W tym folderze pominięto { $count } linki lub nazwy, które otworzyłyby coś innego, więc to, co wskazują, nie trafiło do instalatora: { $names }
        [many] W tym folderze pominięto { $count } linków lub nazw, które otworzyłyby coś innego, więc to, co wskazują, nie trafiło do instalatora: { $names }
       *[other] W tym folderze pominięto { $count } linków lub nazw, które otworzyłyby coś innego, więc to, co wskazują, nie trafiło do instalatora: { $names }
    }{ $more ->
        [0] .
        [one] { " " }i jeszcze { $more }.
        [few] { " " }i jeszcze { $more }.
        [many] { " " }i jeszcze { $more }.
       *[other] { " " }i jeszcze { $more }.
    }
# { $count } is how many folders. { $depth } is how many levels down a folder can be. { $names } lists some of them.
# { $more } is how many more there are, which are not named; when it is 0, the sentence ends after the list.
core-warning-build-folders-too-deep =
    { $count ->
        [one] { $count } folder przekracza maksymalną głębokość zagnieżdżenia ({ $depth }), więc jego zawartość nie trafiła do instalatora: { $names }
        [few] { $count } foldery przekraczają maksymalną głębokość zagnieżdżenia ({ $depth }), więc ich zawartość nie trafiła do instalatora: { $names }
        [many] { $count } folderów przekracza maksymalną głębokość zagnieżdżenia ({ $depth }), więc ich zawartość nie trafiła do instalatora: { $names }
       *[other] { $count } folderów przekracza maksymalną głębokość zagnieżdżenia ({ $depth }), więc ich zawartość nie trafiła do instalatora: { $names }
    }{ $more ->
        [0] .
        [one] { " " }i jeszcze { $more }.
        [few] { " " }i jeszcze { $more }.
        [many] { " " }i jeszcze { $more }.
       *[other] { " " }i jeszcze { $more }.
    }
# { $error } is what went wrong.
core-warning-build-metadata-unreadable = Nie można odczytać metadanych pliku: { $error }
core-warning-build-reserved-name = Nazwa tego pliku jest zarezerwowana dla urządzenia systemu Windows, więc plik nie został skopiowany.
# { $error } is what went wrong.
core-warning-build-hash-failed = Nie można obliczyć skrótu pliku: { $error }
core-warning-build-cloud-placeholder = Plik w chmurze dostępny tylko online (np. Pliki na żądanie usługi OneDrive) nie ma danych lokalnych; przed utworzeniem kopii zapasowej przypnij go lub pobierz. Zgłoszono, ale nie skopiowano.
# { $file_name } is a driver's file name, such as acmeflt.sys.
core-warning-build-driver-file = Plik sterownika lub jądra „{ $file_name }” nie jest kopiowany jako zwykły plik. Jeśli obsługuje zarejestrowaną usługę sterownika tej aplikacji, instalator dołącza go i automatycznie ponownie rejestruje sterownik; w przeciwnym razie zainstaluj go ponownie z podpisanego pakietu sterowników dostawcy.
core-warning-build-file-gone = Ten element istniał podczas analizy aplikacji, ale już go nie ma, więc nie trafił do instalatora.
# { $error } is what went wrong.
core-warning-build-file-copy-failed = Nie można skopiować pliku do pakietu: { $error }
# { $count } is how many files. { $names } lists them.
core-warning-build-not-looked-through =
    { $count ->
        [one] Ten pakiet zawiera plik, którego nie można było przejrzeć pod kątem informacji o tym, od kogo pochodzi: { $names }. Jego zawartość może to ujawniać.
        [few] Ten pakiet zawiera pliki, których nie można było przejrzeć pod kątem informacji o tym, od kogo pochodzą: { $names }. Ich zawartość może to ujawniać.
        [many] Ten pakiet zawiera pliki, których nie można było przejrzeć pod kątem informacji o tym, od kogo pochodzą: { $names }. Ich zawartość może to ujawniać.
       *[other] Ten pakiet zawiera pliki, których nie można było przejrzeć pod kątem informacji o tym, od kogo pochodzą: { $names }. Ich zawartość może to ujawniać.
    }
# { $count } is the most empty folders an installer makes, such as 10,000.
core-warning-build-empty-folders-truncated = Ta aplikacja ma więcej niż { $count } pustych folderów, a odtworzonych zostanie tylko tyle.
core-warning-build-credential-by-now = Pominięto w pakiecie: element wygląda teraz na poświadczenie.
core-warning-build-no-package-path = Plik do skopiowania nie miał ścieżki względnej w pakiecie.
# { $file_name } is the runtime installer's file name, such as vc_redist.x64.exe. { $error } is what went wrong.
core-warning-build-prerequisite-path-invalid = Nie można umieścić dołączonego wymagania wstępnego { $file_name }: { $error }
# { $runtime } is a runtime's name, such as Microsoft Visual C++ 2015+ Redistributable (x64). { $error } is what went
# wrong. winget is the name of a Windows command; keep it as it is.
core-warning-build-prerequisite-download-failed = Nie można pobrać środowiska uruchomieniowego { $runtime } ({ $error }); instalator spróbuje użyć narzędzia winget na komputerze docelowym.
# { $runtime } is a runtime's name. { $reason } is why the check failed, such as "it is signed by Contoso Ltd, not
# Microsoft".
core-warning-build-prerequisite-signature-failed = Pobrany plik środowiska uruchomieniowego { $runtime } nie przeszedł weryfikacji podpisu, więc nie został dołączony: { $reason }. Instalator spróbuje użyć narzędzia winget na komputerze docelowym.
# { $runtime } is a runtime's name. { $error } is what went wrong.
core-warning-build-prerequisite-hash-failed = Pobrano środowisko uruchomieniowe { $runtime }, ale nie można obliczyć jego skrótu ({ $error }); instalator spróbuje zamiast tego użyć narzędzia winget na komputerze docelowym.
# { $service } is a driver's service name, such as AcmeFlt. { $error } is what went wrong.
core-warning-build-driver-copy-failed = Nie można dołączyć obrazu sterownika dla usługi „{ $service }”: { $error }
core-warning-build-license-not-copied = Nie można skopiować danych licencji (brak danych lub są nieczytelne); zapisano tylko metadane.
core-warning-build-license-file-credential = Plik licencji wygląda na poświadczenie, więc został pominięty.
# { $error } is what went wrong.
core-warning-build-license-store-walk-failed = Nie można wyliczyć części magazynu licencji: { $error }
core-warning-build-license-store-credential = Plik w magazynie licencji wygląda na poświadczenie, więc został pominięty.
# { $error } is what went wrong.
core-warning-build-license-store-file-skipped = Pominięto plik magazynu licencji: { $error }
