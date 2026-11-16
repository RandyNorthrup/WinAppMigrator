# Words of the app, WinAppMigrator itself: its one window, with the installed apps listed on the left and, on the
# right, what goes in an installer of the one picked and the button that builds it; and the steps to read another PC
# through WinAppMigrator Client. Message names start with app-.


## The window and its header

# The program's name, in the window's title bar and at the top of the window. Keep it as it is unless your language
# writes product names in a script of its own.
app-name = WinAppMigrator
app-tagline = Przekształć zainstalowaną aplikację w samorozpakowujący się instalator
app-theme-to-light = Przełącz na motyw jasny
app-theme-to-dark = Przełącz na motyw ciemny
app-reload-list = Załaduj ponownie listę aplikacji
app-list-loading = Ładowanie
# { $count } is how many apps are listed.
app-list-count =
    { $count ->
        [one] { $count } aplikacja
        [few] { $count } aplikacje
        [many] { $count } aplikacji
       *[other] { $count } aplikacji
    }
app-list-load-failed = Ładowanie nie powiodło się
# Shown when the window cannot open at all. { $error } is what went wrong, in the words of Windows or a graphics driver.
app-window-could-not-open = Nie można otworzyć okna: { $error }

## The list of installed apps

app-list-search = Wyszukaj według nazwy lub wydawcy  (Ctrl+F)
app-list-filter-all = Wszystkie
# Whether an app is installed for one account or for everyone on the PC.
app-scope-per-user = Dla użytkownika
app-scope-per-machine = Dla komputera
app-scope-unknown = Nieznany
# An app whose processor architecture (x86, x64, ARM64) is not known.
app-arch-unknown = Nieznana
app-list-loading-apps = Ładowanie zainstalowanych aplikacji…
# { $error } is why the apps could not be listed.
app-list-could-not-list = Nie można wyświetlić listy aplikacji: { $error }
app-list-no-matches = Brak pasujących aplikacji
app-list-no-matches-hint = Zmień tekst wyszukiwania lub filtr zakresu.
app-list-worker-disconnected = Rozłączono proces roboczy inwentaryzacji.
# What a screen reader says of an app's row. { $name } is the app's name, { $publisher } who made it, and { $version } its
# version, such as 11.4.2, or — when either is not known.
app-list-row = { $name }, { $publisher }, wersja { $version }

## The list's columns, and what a screen reader says each column's heading does when clicked

app-column-name = Nazwa
app-column-name-sort = Sortuj według nazwy
app-column-publisher = Wydawca
app-column-publisher-sort = Sortuj według wydawcy
app-column-version = Wersja
app-column-version-sort = Sortuj według wersji
app-column-scope = Zakres
app-column-scope-sort = Sortuj według zakresu
# Short for architecture: x86, x64, or ARM64. The column is narrow.
app-column-arch = Arch.
app-column-arch-sort = Sortuj według architektury

## Where apps are read from: this PC, a drive from another PC, a disk image of one, or another PC on the network

app-source = Źródło analizy
app-source-this-pc = Ten komputer
app-source-drive = Podłączony dysk…
app-source-image = Obraz dysku…
app-source-network = Komputer w sieci…
app-source-attaching-image = Dołączanie obrazu w trybie tylko do odczytu…
# { $computer } is the other PC's name, such as OLD-PC, and { $address } its network address, such as 192.168.1.20.
app-source-network-pc = { $computer } przez sieć ({ $address })
app-source-disconnect = Rozłącz
app-source-change = Zmień…
# The title of the window that picks a disk image. .vhd and .vhdx are file extensions.
app-source-pick-image-title = Wybierz obraz dysku innego komputera (.vhd lub .vhdx)
app-source-pick-image-filter = Obrazy dysków
# { $path } is the disk image file.
app-source-image-has-no-windows = Obraz { $path } nie zawiera woluminu z systemem Windows.
app-source-image-not-attached = Nie można dołączyć obrazu dysku.
app-source-pick-drive-title = Wybierz katalog główny podłączonego dysku (np. E:\)
# { $path } is the folder picked, such as E:\. \Windows and \Users are folder names, as Windows writes them.
app-source-not-a-windows-drive = Lokalizacja { $path } nie wygląda na dysk systemowy Windows (brak folderów \Windows i \Users). Wybierz katalog główny dysku z innego komputera.
# Said of a drive or disk image with a Windows older than Windows 10. { $windows } is its name, such as "Windows 7
# Professional Service Pack 1, 32-bit". { $is_32_bit } is "yes" when that Windows is 32-bit, which adds the last
# sentence, and "no" when it is not.
app-source-older-windows =
    Zawiera system { $windows }. Instalatory utworzone z tego źródła są przeznaczone dla systemów Windows 10 i 11. Z opcją Dane użytkownika przenoszone są też ustawienia, które system Windows po cichu przechowywał osobno dla starszej aplikacji, a środowiska uruchomieniowe, których nie ma w systemie Windows 11, takie jak DirectX 9, są instalowane razem z aplikacją.{ $is_32_bit ->
        [yes] { " " }Aplikacje z tego systemu są 32-bitowe, więc trafiają do folderu Program Files (x86), w którym 64-bitowy system Windows przechowuje takie aplikacje.
       *[no] { "" }
    }
# The same, of a Windows whose name could not be read. { $is_32_bit } is as above.
app-source-older-windows-unnamed =
    Zawiera starszą wersję systemu Windows. Instalatory utworzone z tego źródła są przeznaczone dla systemów Windows 10 i 11. Z opcją Dane użytkownika przenoszone są też ustawienia, które system Windows po cichu przechowywał osobno dla starszej aplikacji, a środowiska uruchomieniowe, których nie ma w systemie Windows 11, takie jak DirectX 9, są instalowane razem z aplikacją.{ $is_32_bit ->
        [yes] { " " }Aplikacje z tego systemu są 32-bitowe, więc trafiają do folderu Program Files (x86), w którym 64-bitowy system Windows przechowuje takie aplikacje.
       *[no] { "" }
    }

## The details panel, before an app is picked, and the app picked

app-details-pair-first = Sparuj ze starym komputerem
app-details-pair-first-hint = Gdy program WinAppMigrator Client będzie tam działać i wpiszesz wyświetlony przez niego kod, aplikacje tamtego komputera pojawią się tutaj, a instalator dowolnej z nich utworzysz tak samo jak na tym komputerze.
app-details-pick-an-app = Wybierz aplikację
app-details-pick-an-app-hint = Wybierz aplikację z listy, aby określić, co ma zostać dołączone, i utworzyć jej instalator.
# { $publisher } is who made the app, and { $version } its version, such as 11.4.2.
app-details-publisher-and-version = { $publisher }, wersja { $version }
# { $version } is the app's version, such as 11.4.2.
app-details-unknown-publisher-and-version = Nieznany wydawca, wersja { $version }
app-details-unknown-publisher = Nieznany wydawca

## What goes in besides the program

app-option-license = Dane licencji i aktywacji
app-option-license-hint = Mogą być powiązane z tym komputerem i wymagać ponownej aktywacji na nowym.
app-option-user-data = Dane użytkownika
# Documents, Saved Games, and ProgramData are folders of Windows: use the names Windows gives them in your language,
# except ProgramData, which keeps its name.
app-option-user-data-hint = Ustawienia i historia, Dokumenty, Zapisane gry oraz ProgramData.
app-option-whose-data = Czyje dane
app-option-whose-data-hint = Instalator zawiera dane jednego konta. Trafiają one do osoby zalogowanej na komputerze, na którym zostanie uruchomiony.

## Building the installer

app-output-folder = Folder wyjściowy
# An example of a folder, shown in the empty field. "You" stands for the name of an account.
app-output-folder-example = C:\Users\Użytkownik\WinAppMigrator
app-output-folder-not-full = Wpisz pełną ścieżkę folderu, na przykład D:\Instalatory.
app-build-create = Utwórz instalator
app-build-create-again = Utwórz instalator ponownie
app-build-cancel = Anuluj
app-build-cancelling = Anulowanie po zakończeniu bieżącego kroku…
app-build-cancelled = Anulowano tworzenie. Nie zapisano żadnego instalatora.
# winappmigrator-installer.exe is a file name.
app-build-no-stub = Nie znaleziono modułu startowego instalatora (winappmigrator-installer.exe) obok tej aplikacji, więc nie można utworzyć samorozpakowującego się instalatora.
app-build-select-an-app-first = Najpierw wybierz aplikację do migracji.
app-build-look-again-first = Zanim będzie można utworzyć instalator tej aplikacji, trzeba ją ponownie przeanalizować.
# { $reason } is what went wrong, in the program's own words, often in English.
app-build-stopped-unexpectedly = Tworzenie nieoczekiwanie się zatrzymało: { $reason }. Jeśli w folderze wyjściowym pozostał instalator, nie używaj go.
app-build-worker-disconnected = Rozłączono proces roboczy migracji.
# { $path } is the output folder, and { $error } what Windows said went wrong.
app-build-no-output-folder = Nie można utworzyć folderu wyjściowego { $path }: { $error }
# { $error } is what went wrong.
app-build-no-staging-folder = Nie można utworzyć chronionego folderu przejściowego, więc niczego nie utworzono: { $error }
# { $error } is what went wrong, as a sentence.
app-build-could-not-build = Nie można utworzyć instalatora. { $error }
app-build-no-installer = Tworzenie zakończyło się bez instalatora.
# { $error } is what went wrong.
app-build-hash-check-did-not-run = Instalator został zapisany, ale nie można było przeprowadzić weryfikacji jego skrótów: { $error }. Nie używaj go.

## How far a build has got

app-progress-preparing = Ustalanie zawartości…
app-progress-checking-room = Sprawdzanie dostępnego miejsca…
app-progress-bundling = Dołączanie środowisk uruchomieniowych i sterowników…
# { $done } is how many files have been read so far, and { $total } how many there are.
app-progress-packing = Odczytywanie i kompresowanie plików: { $done } z { $total }
# { $done } is how many of the installer's records have been looked over so far, and { $total } how many there are.
app-progress-looking-over = Przeglądanie zawartości instalatora: { $done } z { $total }…
app-progress-writing = Zapisywanie instalatora…
app-progress-verifying = Weryfikowanie skrótów…

## What a build made

app-result-failed-hash-check = Nie używaj tego instalatora: nie przeszedł weryfikacji skrótów.
# { $count } is how many files were gone when the build came to read them.
app-result-files-gone =
    { $count ->
        [one] Instalator został utworzony, ale { $count } plik zniknął od czasu analizy aplikacji, więc nie ma go w instalatorze. Zobacz poniżej sekcję Ostrzeżenia.
        [few] Instalator został utworzony, ale { $count } pliki zniknęły od czasu analizy aplikacji, więc nie ma ich w instalatorze. Zobacz poniżej sekcję Ostrzeżenia.
        [many] Instalator został utworzony, ale { $count } plików zniknęło od czasu analizy aplikacji, więc nie ma ich w instalatorze. Zobacz poniżej sekcję Ostrzeżenia.
       *[other] Instalator został utworzony, ale { $count } plików zniknęło od czasu analizy aplikacji, więc nie ma ich w instalatorze. Zobacz poniżej sekcję Ostrzeżenia.
    }
# { $count } is how many files could not be read into the installer.
app-result-files-not-captured =
    { $count ->
        [one] Instalator został utworzony, ale { $count } plik nie mógł zostać przechwycony, zwykle dlatego, że był zablokowany lub nieczytelny. Aplikacja może bez niego nie działać. Zobacz poniżej sekcję Ostrzeżenia.
        [few] Instalator został utworzony, ale { $count } pliki nie mogły zostać przechwycone, zwykle dlatego, że były zablokowane lub nieczytelne. Aplikacja może bez nich nie działać. Zobacz poniżej sekcję Ostrzeżenia.
        [many] Instalator został utworzony, ale { $count } plików nie mogło zostać przechwyconych, zwykle dlatego, że były zablokowane lub nieczytelne. Aplikacja może bez nich nie działać. Zobacz poniżej sekcję Ostrzeżenia.
       *[other] Instalator został utworzony, ale { $count } plików nie mogło zostać przechwyconych, zwykle dlatego, że były zablokowane lub nieczytelne. Aplikacja może bez nich nie działać. Zobacz poniżej sekcję Ostrzeżenia.
    }
app-result-mentions-who = Instalator został utworzony, ale nadal zawiera pewne informacje o tym, od kogo pochodzi. Zanim przekażesz go komukolwiek, przeczytaj poniżej, co zawiera.
app-result-ready = Instalator jest gotowy. Jego pakiet pomyślnie przeszedł weryfikację skrótów.
app-result-files = Pliki
app-result-installer-size = Rozmiar instalatora
app-result-warnings = Ostrzeżenia
app-result-log = Dziennik
app-result-copy-error = Kopiuj błąd
app-result-installer = Instalator
app-result-data-file = Plik danych
app-result-open-folder = Otwórz folder
app-result-copy-path = Kopiuj ścieżkę
# { $error } is what Windows said went wrong.
app-result-could-not-open-folder = Nie można otworzyć folderu: { $error }
# { $installer } and { $data_file } are file names, such as App_installer.exe and App_installer.wamdata.
app-result-two-files = Ta aplikacja jest zbyt duża na jeden plik wykonywalny, więc jej instalator składa się z dwóch plików: { $installer } i { $data_file }. Muszą one pozostać razem: skopiuj oba na drugi komputer do tego samego folderu, bez zmieniania nazw. Jeśli podpiszesz instalator, podpis obejmie również plik danych.
# { $app_name } is the app's name. "Unknown publisher" is what Windows shows when it asks to run an unsigned program: use
# Windows' own words for it in your language.
app-result-run-it = Uruchom go na drugim komputerze, aby ponownie zainstalować aplikację { $app_name }. Instalator sprawdza swoje pliki za pomocą skrótów SHA-256. System Windows wyświetla komunikat „Nieznany wydawca”, ponieważ tworzone tu instalatory nie są podpisane.
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-verification-errors = Błędy weryfikacji ({ $count })
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-warnings-heading = Ostrzeżenia ({ $count })
# After the first of a long list of warnings. { $count } is how many more there are.
app-warnings-and-more = i jeszcze { $count }.

## Looking at an app, to see what an installer of it would hold

app-scan-getting-ready = Przygotowywanie do analizy tej aplikacji…
app-scan-install-record = Odczytywanie sposobu instalacji…
app-scan-near-matches = Wyszukiwanie elementów, które również mogą należeć do aplikacji…
app-scan-system-items = Wyszukiwanie usług, zadań i skrótów aplikacji…
app-scan-files = Przeglądanie plików aplikacji…
app-scan-registry = Odczytywanie powiązań z systemem Windows…
app-scan-finishing = Kończenie…
app-scan-look-again = Analizuj ponownie
app-scan-stop = Zatrzymaj
app-scan-running-hint = W trakcie analizy możesz wybrać inną aplikację lub zmienić dołączane elementy.
# { $error } is why, in words that follow a colon.
app-scan-failed = Nie można przeanalizować tej aplikacji: { $error }
app-scan-try-again = Spróbuj ponownie
app-scan-stopped = Zatrzymano. Instalator będzie można utworzyć po przeanalizowaniu tej aplikacji.
# { $reason } is what went wrong, in the program's own words, often in English.
app-scan-stopped-unexpectedly = Analiza nieoczekiwanie się zatrzymała: { $reason }.

## How big an installer can be, and whether there is room for it

app-room-files = Pliki
# Over how big the installer can be: up to that size.
app-room-up-to = Maksymalnie
app-room-free = Wolne
# { $drive } is a drive, such as C:.
app-room-free-on = Wolne na dysku { $drive }
# How much room a drive has, when it would not say.
app-room-free-unknown = Nieznane
app-room-most-it-could-need = Maksymalna ilość miejsca, jaka może być potrzebna, przed kompresją.
# { $drive } is a drive, such as C:, { $needed } how much room it needs, such as 1.2 GB, and { $free } how much it has.
app-room-drive-short = Dysk { $drive } potrzebuje do { $needed } wolnego miejsca, aby utworzyć ten instalator, a ma { $free }.
# { $drives } is one sentence like the one above for each drive without the room, one after another.
app-room-not-enough = Za mało miejsca. { $drives } Wybierz folder wyjściowy na innym dysku lub pomiń niektóre elementy.
# { $note } is a sentence, without its full stop, about a drive whose room could not be checked. Use your language's
# full stop.
app-room-could-not-tell = { $note }.

## What goes in the installer, part by part

app-footprint-heading = Zawartość instalatora
app-footprint-working-out = Ustalanie, co jest z tym powiązane…
# { $error } is why.
app-footprint-would-not-make-a-package = Z pozostałych elementów nie można utworzyć pakietu: { $error }
# { $list } is the names of what else is left out, such as a service and a shortcut.
app-footprint-also-left-out = Spowoduje to również pominięcie elementów: { $list }, ponieważ to, co uruchamiają lub wskazują, nie byłoby dostępne. Nie zostaną one skonfigurowane na drugim komputerze.
# { $list } is the names of the first few of what else is left out, and { $count } how many more there are.
app-footprint-also-left-out-and-more = Spowoduje to również pominięcie elementów: { $list } i jeszcze { $count }, ponieważ to, co uruchamiają lub wskazują, nie byłoby dostępne. Nie zostaną one skonfigurowane na drugim komputerze.
# { $list } is the names of license files. The option is the one named "License and Activation Data".
app-footprint-licenses-still-go = Dane licencji z pominiętych elementów zostaną mimo to dołączone, ponieważ decyduje o nich opcja Dane licencji i aktywacji: { $list }.
# { $elapsed } is how long ago, in minutes and seconds such as 45:00, or hours, minutes, and seconds such as 1:05:00.
# "Look Again" names the button of that name.
app-footprint-looked-at-long-ago = Aplikacja została przeanalizowana { $elapsed } temu. Pliki są odczytywane podczas tworzenia instalatora, więc zmiany zostaną uwzględnione, a wszystko, co do tego czasu zniknie, zostanie wymienione. Aby uwzględnić nowe elementy, kliknij przycisk Analizuj ponownie.
# Headings of the parts of the list, each with a line that says what is in it.
app-footprint-program-files = Pliki programu
app-footprint-data-folders = Foldery danych
# What could go in with the User Data option, when nothing else is in this part. { $size } is how much, such as 1.2 GB.
app-footprint-folders-with-user-data = { $size } z opcją Dane użytkownika
# { $included } is how many folders go in, { $count } how many there are, and { $size } how much those that go in hold.
app-footprint-folders-summary =
    { $count ->
        [one] { $included } z { $count } folderu, { $size }
        [few] { $included } z { $count } folderów, { $size }
        [many] { $included } z { $count } folderów, { $size }
       *[other] { $included } z { $count } folderów, { $size }
    }
# { $size } is how much, such as 1.2 GB, and { $count } in how many files.
app-footprint-data-only-with-user-data =
    { $count ->
        [one] Dane tej aplikacji ({ $size } w { $count } pliku) zostaną dołączone tylko z opcją Dane użytkownika.
        [few] Dane tej aplikacji ({ $size } w { $count } plikach) zostaną dołączone tylko z opcją Dane użytkownika.
        [many] Dane tej aplikacji ({ $size } w { $count } plikach) zostaną dołączone tylko z opcją Dane użytkownika.
       *[other] Dane tej aplikacji ({ $size } w { $count } plikach) zostaną dołączone tylko z opcją Dane użytkownika.
    }
# A folder's row. { $count } is how many files it holds, and { $detail } where it is or why it cannot be left out.
app-footprint-folder-row =
    { $count ->
        [one] { $count } plik. { $detail }
        [few] { $count } pliki. { $detail }
        [many] { $count } plików. { $detail }
       *[other] { $count } plików. { $detail }
    }
app-footprint-services = Usługi
app-footprint-startup = Autostart
app-footprint-tasks = Zaplanowane zadania
app-footprint-shortcuts = Skróty
# { $included } is how many of the part's things go in, and { $count } how many there are.
app-footprint-items-summary = { $included } z { $count }
# { $why } is what else was left out that this goes with.
app-footprint-left-out-too = Również pominięte: { $why }
# A row's detail. { $kind } is what it is, such as Service, and { $detail } what it does.
app-footprint-change-detail = { $kind }. { $detail }
app-footprint-integrations = Powiązania z systemem Windows
# { $included } is how many kinds go in, such as file types and firewall rules, and { $count } how many kinds there are.
app-footprint-integrations-summary =
    { $count ->
        [one] { $included } z { $count } rodzaju
        [few] { $included } z { $count } rodzajów
        [many] { $included } z { $count } rodzajów
       *[other] { $included } z { $count } rodzajów
    }
app-footprint-integration-nothing-left = Nic nie pozostało z powodu pominiętych elementów
# { $count } is how many things of this kind the installer sets up.
app-footprint-integration-things =
    { $count ->
        [one] { $count } element do skonfigurowania
        [few] { $count } elementy do skonfigurowania
        [many] { $count } elementów do skonfigurowania
       *[other] { $count } elementów do skonfigurowania
    }
app-footprint-prerequisites = Wymagane do działania
# { $count } is how many runtimes, such as Visual C++ Redistributable.
app-footprint-prerequisites-summary =
    { $count ->
        [one] { $count } środowisko uruchomieniowe
        [few] { $count } środowiska uruchomieniowe
        [many] { $count } środowisk uruchomieniowych
       *[other] { $count } środowisk uruchomieniowych
    }
app-footprint-near-matches = Może również należeć do aplikacji
# { $ticked } is how many were ticked, and { $count } how many there are.
app-footprint-near-matches-summary = Dodano { $ticked } z { $count }
app-footprint-near-matches-hint = Te elementy nie trafią do instalatora, jeśli ich nie zaznaczysz. Zaznaczenie elementu powoduje ponowną analizę aplikacji z tymi samymi kontrolami co reszta, więc wszystko, co wygląda na hasło, nadal zostanie pominięte.
# { $why } is why it might be part of the app, as a sentence.
app-footprint-near-match-with-user-data = Dołączane z opcją Dane użytkownika. { $why }
# How much a folder holds, when not all of it could be measured. { $size } is how much was, such as 1.2 GB.
app-footprint-near-match-over = ponad { $size }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-folder = Folder: { $place }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-hidden = Ukryta część aplikacji: { $place }
app-footprint-nothing-left-out = Nie pominięto niczego, co wyglądałoby na hasło, licencję lub certyfikat.
app-footprint-left-out = Pominięte elementy i przyczyny
# { $count } is how many things were left out.
app-footprint-left-out-summary =
    { $count ->
        [one] { $count } element
        [few] { $count } elementy
        [many] { $count } elementów
       *[other] { $count } elementów
    }
# Above a list of things left out for one reason. { $title } is the reason.
app-footprint-left-out-group = { $title }:
app-footprint-refused = Żaden instalator nie miałby pozwolenia na skonfigurowanie tych elementów:
# A heading over a list that opens and closes. { $count } is how many are in it.
app-footprint-worth-knowing = Warto wiedzieć ({ $count })
# { $list } is what kinds of things were looked for and none found, from the five below.
app-none-found = Nie znaleziono elementów tego typu: { $list }.
app-none-found-services = usługi
app-none-found-startup = elementy autostartu
app-none-found-tasks = zaplanowane zadania
app-none-found-shortcuts = skróty
app-none-found-integrations = powiązania z systemem Windows

## Network PC: reading another PC through WinAppMigrator Client, in three steps. "WinAppMigrator Client" is the name of a
## program, and the old PC is the one the apps are moved off.

app-network-step-client = 1. Umieść klienta na starym komputerze
app-network-step-client-hint = Program WinAppMigrator Client działa na komputerze, na którym aplikacja znajduje się teraz (Windows 7 lub nowszy). Pozwala tylko temu komputerowi odczytywać swoje dane, w postaci zaszyfrowanej, za pomocą wyświetlanego kodu.
# { $drive } is a USB drive, such as E:\.
app-network-save-to-usb = Zapisz na dysku USB { $drive }
# { $drive } is a USB drive, such as E:\, and { $name } the name it was given, such as BACKUP.
app-network-save-to-named-usb = Zapisz na dysku USB { $drive } ({ $name })
app-network-save-to-folder = Zapisz w folderze…
app-network-look-for-usb-again = Wyszukaj ponownie dyski USB
app-network-no-usb = Nie podłączono dysku USB. Podłącz dysk albo zapisz klienta w folderze dostępnym ze starego komputera, na przykład w folderze udostępnionym w sieci.
# { $path } is where the client was saved, such as E:\WinAppMigrator Client.exe.
app-network-client-saved = Zapisano jako { $path }. Na starym komputerze zamknij przenoszoną aplikację, a następnie kliknij dwukrotnie ten plik i zezwól mu na wprowadzanie zmian. Zostanie wyświetlony kod.
app-network-step-pick = 2. Wybierz stary komputer
app-network-looking = Wyszukiwanie w sieci komputerów, na których działa klient…
app-network-type-name = Albo wpisz jego nazwę lub adres
# An example, shown in the empty field: a PC's name, or its network address.
app-network-type-name-example = STARY-PC lub 192.168.1.20
app-network-step-code = 3. Wpisz wyświetlany kod
app-network-code = Kod parowania
app-network-connect = Połącz
app-network-pairing = Parowanie…
# Why pairing did not work, shown on its own.
app-network-pairing-stopped = parowanie zostało przerwane przed zakończeniem
app-network-wrong-code = to nie jest kod wyświetlany na starym komputerze. Sprawdź go i spróbuj ponownie.
# { $error } is what went wrong.
app-network-not-reached = nie można połączyć się ze starym komputerem: { $error }
app-network-pick-folder-title = Wybierz folder dla programu WinAppMigrator Client
# { $path } is the folder, and { $error } what Windows said went wrong.
app-client-could-not-save = nie można zapisać klienta w folderze { $path }: { $error }
# scripts\build-client.ps1 is a file name.
app-client-not-carried = ta kompilacja programu WinAppMigrator nie zawiera klienta. Skompiluj go za pomocą skryptu scripts\build-client.ps1, a następnie ponownie skompiluj aplikację.
# { $error } is what Windows said went wrong.
app-client-could-not-save-note = nie można zapisać notatki obok klienta: { $error }
# A note saved as a text file beside the client, for whoever runs it on the old PC. Keep its lines short, as it is read in
# Notepad. "WinAppMigrator Client.exe" is a file name. "Network PC..." names the Scan Source button: use its words in your
# language.
app-client-note =
    WinAppMigrator Client
    
    Uruchom ten program na komputerze, z którego przenosisz aplikacje:
    Windows 7, 8, 10 lub 11.
    
    1. Zamknij przenoszoną aplikację.
    2. Kliknij dwukrotnie plik „WinAppMigrator Client.exe” i zezwól mu
       na wprowadzanie zmian.
    3. Zostanie wyświetlony 6-cyfrowy kod parowania. W programie WinAppMigrator
       na nowym komputerze wybierz opcję Komputer w sieci..., wybierz ten
       komputer i wpisz kod.
    4. Gdy nowy komputer skończy pracę, naciśnij klawisz Enter w oknie klienta
       lub zamknij to okno.
    
    Program tylko odczytuje dane. Podczas działania przechowuje migawkę dysku
    systemu Windows i przepuszcza przez Zaporę systemu Windows ruch z sieci
    lokalnej na swoich dwóch portach, a po zatrzymaniu usuwa jedno i drugie.
    Dane przesyłane przez sieć są szyfrowane i tylko komputer, który zna kod,
    może cokolwiek odczytać.
