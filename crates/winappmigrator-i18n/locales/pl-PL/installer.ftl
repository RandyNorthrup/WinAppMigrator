# Words of the installer that every built installer carries, which runs on the PC an app is moved to and speaks its
# language, and of the uninstaller it leaves behind for Settings. Message names start with installer- for the
# installer's window, installer-cli- for its command line, and uninstaller- for the uninstaller.
#
# The command lines (installer-cli-, uninstaller-cli-) are always shown in English, as scripts read what they print.
# Translate them all the same. Keep their command-line flags, such as --apply and --target-root, exactly as they are.


## The installer's window: its title and header

installer-window-title = Instalator WinAppMigrator
# The window's title in the taskbar. { $app_name } is the name of the app it installs.
installer-window-title-app = Instalacja aplikacji { $app_name }
installer-tagline = Ponownie instaluje aplikację przechwyconą na innym komputerze
# What an app is called when its package gives it no name.
installer-unnamed-app = Aplikacja

## When the installer's window cannot open. Each is a paragraph of a message box.

# { $error } is what went wrong, as Windows or the graphics driver says it.
installer-window-failed = Nie można otworzyć okna instalatora: { $error }
installer-window-failed-without-one = Instalacja jest możliwa również bez okna: uruchom ten plik w wierszu polecenia administratora z opcją --apply, a jeśli chcesz wybrać lokalizację, dodaj opcję --target-root i ścieżkę folderu.

## Checking the installer when it opens

installer-checking = Sprawdzanie instalatora…
installer-stub-no-package = To jest moduł startowy instalatora WinAppMigrator i nie zawiera on pakietu aplikacji. Utwórz instalator za pomocą programu WinAppMigrator, a następnie uruchom tutaj utworzony plik .exe.
installer-check-stopped = Sprawdzanie instalatora zostało przerwane.
# { $error } is what went wrong, as Windows says it.
installer-locate-failed = Nie można zlokalizować instalatora: { $error }
# { $error } is what went wrong reading it.
installer-read-failed = Nie można odczytać instalatora: { $error }
# { $error } is why, such as that the app needs a newer Windows.
installer-cannot-be-used-here = Pakietu tego instalatora nie można użyć na tym komputerze: { $error }

## The app, where it goes, and whether the installer is signed

installer-captured-from = Przechwycono z
installer-install-location = Lokalizacja instalacji
# The title of the folder picker.
installer-choose-location = Wybierz lokalizację instalacji
installer-folder-has-files = Ten folder zawiera już pliki. Pliki o takich samych nazwach zostaną zastąpione i wrócą na miejsce, jeśli odinstalujesz aplikację.
installer-room-needed = Wymagane miejsce
# { $shortfall } says which drive is short of room, such as "C: needs up to 824.0 MB for the app's files, and has 300.0 MB free".
installer-room-short = W tej lokalizacji może brakować miejsca: { $shortfall }. Pakiet można rozpakować na innym dysku, ale pliki aplikacji i tak muszą się zmieścić.
# { $app_name } is the name of an app whose earlier install was cut short.
installer-interrupted = Wcześniejsza instalacja aplikacji { $app_name } nie została ukończona. Instalacja najpierw ją cofnie.
# Beside "Installer", for whether it proves who made it.
installer-signature = Instalator
installer-signature-checking = Sprawdzanie podpisu…
installer-signature-unsigned = Niepodpisany: nieznany wydawca
# { $publisher } is the name on the installer's certificate, such as "Fabrikam, Inc.".
installer-signature-signed-by = Podpisany przez: { $publisher }
installer-signature-unnamed = Podpisany przez nienazwanego wydawcę
# { $reason } is Windows' own word for what is wrong, such as HashMismatch.
installer-signature-invalid = Podpis tego instalatora jest nieprawidłowy ({ $reason }). Instalator mógł zostać zmieniony po podpisaniu.

## Why the install location cannot be used. Install and Preview Changes stay off while one of these is shown.

# { $app_name } is the name of the app being installed.
installer-location-app-already-there = Aplikacja { $app_name } jest już zainstalowana w tym folderze, więc tej kopii nie można w nim umieścić. Aby zainstalować ją tutaj, najpierw odinstaluj kopię z tego komputera w Ustawieniach systemu Windows w sekcji Aplikacje, a następnie ponownie otwórz ten instalator. Możesz też wpisać powyżej nowy, pusty folder.
# { $other_app } is the name of another app Windows lists as installed in the folder. { $app_name } is the name of the app being installed.
installer-location-other-app-there = W tym folderze jest zainstalowana aplikacja { $other_app }, więc nie można w nim umieścić aplikacji { $app_name }. Wpisz powyżej nowy, pusty folder. Jeśli aplikacja { $other_app } nie jest już potrzebna, możesz też najpierw ją odinstalować w Ustawieniach systemu Windows w sekcji Aplikacje, a następnie ponownie otworzyć ten instalator.
installer-location-has-files = Ten folder zawiera już pliki, które nie zostały zainstalowane przez program WinAppMigrator. Mogą one należeć do innej aplikacji, więc nic nie zostanie na nich zainstalowane. Wpisz powyżej nowy, pusty folder.
# { $reason } is why, such as that it cannot be the Windows folder.
installer-location-refused = Nie można użyć tego folderu: { $reason }. Wpisz powyżej inny folder.

## What the install sets up beyond its files

installer-beyond-files = Co ta instalacja konfiguruje oprócz plików
# Under the heading above. It is part of what a person agrees to when they click Install.
installer-beyond-files-agreed = Wszystkie te czynności są wykonywane z uprawnieniami administratora. Jeśli instalacja się nie powiedzie, wprowadzone zmiany zostaną cofnięte, a otrzymasz informację o wszystkim, czego nie udało się cofnąć. Instalację możesz też cofnąć później w Ustawieniach, w sekcji Zainstalowane aplikacje. Zastąpione pliki wrócą wtedy na miejsce, a zainstalowane środowiska uruchomieniowe pozostaną.
installer-beyond-files-none = Nic: kopiowane są tylko pliki oraz ustawienia we własnych kluczach rejestru aplikacji.
# { $problem } is why, such as that the app cannot run on this PC.
installer-cannot-go-here = Tego pakietu nie można zainstalować w tej lokalizacji: { $problem }
installer-working-out = Ustalanie, co zostałoby skonfigurowane w tej lokalizacji…
# Said as the reason after "This package cannot go to this location:".
installer-list-stopped = ustalanie, co zostałoby wykonane, zostało przerwane
# A heading over a list of what else the package asked for and will not get. { $count } is how many.
installer-more-left-out =
    { $count ->
        [one] Pominięto jeszcze { $count } element
        [few] Pominięto jeszcze { $count } elementy
        [many] Pominięto jeszcze { $count } elementów
       *[other] Pominięto jeszcze { $count } elementów
    }

## Options

installer-options = Opcje
installer-user-data = Dane użytkownika
installer-user-data-detail = Włączone: ustawienia, Dokumenty i Zapisane gry trafiają do standardowych folderów osoby, która jest zalogowana. Wyłączone: wszystkie te dane trafiają do folderu instalacji i nic nie jest konfigurowane do uruchamiania z nich.
# { $account } is the name of the account on the other PC whose data this is.
installer-user-data-whose = Zawiera dane konta { $account }.
installer-license = Licencja
installer-license-detail = Pliki aktywacji i licencji oraz wartości rejestru przechowujące licencje. Niektóre mogą wymagać ponownej aktywacji na tym komputerze.

## Dependencies: what else the app needs, and how each is provided

installer-dependencies = Zależności
installer-dependencies-detail = Czego ta aplikacja potrzebuje i jak zapewniany jest każdy z tych elementów.
installer-dependencies-none = Nie wykryto dodatkowych zależności od środowisk uruchomieniowych.
# { $feature } is the name of an optional part of Windows, such as NetFx3.
installer-dependency-windows-feature = Funkcja systemu Windows: { $feature }
installer-dependency-included = Dołączone
# { $id } is the package's name in winget, such as Microsoft.VCRedist.2015+.x64.
installer-dependency-winget = Instalowane za pomocą winget: { $id }
installer-dependency-manual = Zainstaluj ręcznie
installer-dependency-built-in = Wbudowane w system Windows
installer-dependency-ships-with-app = Dostarczane z aplikacją

## Buttons, and what is said beside them

installer-install = Zainstaluj
installer-preview = Podgląd zmian
installer-cancel = Anuluj
installer-close = Zamknij
installer-open-folder = Otwórz folder instalacji
# { $error } is what went wrong, as Windows says it.
installer-open-folder-failed = Nie można otworzyć folderu: { $error }
installer-location-blocks = Przyciski Zainstaluj i Podgląd zmian są niedostępne, dopóki nie będzie można użyć lokalizacji instalacji. Przyczyna jest podana powyżej, pod tym polem.
installer-package-blocks = Przycisk Zainstaluj jest niedostępny, ponieważ tego pakietu nie można zainstalować w tej lokalizacji. Przyczyna jest podana powyżej.
installer-list-changed = To, co ta instalacja wykonałaby w tej lokalizacji, uległo zmianie, więc niczego nie zainstalowano. Przejrzyj poniższą listę i jeśli wszystko się zgadza, ponownie kliknij przycisk Zainstaluj.
installer-cancelled-early = Anulowano. Niczego nie zmieniono.

## How far an install has got, beside its progress bar

# { $step } is one of the steps below. { $done } and { $total } are how many of its items are done, and how many there are.
installer-step-counted = { $step }: { $done } z { $total }…
# { $step } is one of the steps below, when it does not count its items.
installer-step-uncounted = { $step }…
installer-step-getting-ready = Przygotowywanie…
installer-step-cancelling = Anulowanie po zakończeniu bieżącego kroku…
installer-step-putting-back = Cofanie wprowadzonych zmian
installer-step-unpacking = Rozpakowywanie
installer-step-checking-package = Sprawdzanie pakietu
installer-step-recovering = Cofanie wcześniejszej, nieukończonej instalacji
# What a preview is doing, whatever step it is on.
installer-step-previewing = Ustalanie, co by się zmieniło
installer-step-runtimes = Instalowanie środowisk uruchomieniowych
installer-step-files = Kopiowanie plików
installer-step-services = Konfigurowanie usług
installer-step-drivers = Konfigurowanie sterowników
installer-step-registry = Zapisywanie ustawień rejestru
installer-step-license = Przywracanie danych licencji
installer-step-tasks = Konfigurowanie zaplanowanych zadań
installer-step-shortcuts = Tworzenie skrótów
installer-step-environment = Ustawianie zmiennych środowiskowych
installer-step-settings-entry = Dodawanie aplikacji do Ustawień
installer-step-verifying = Sprawdzanie zainstalowanych elementów
installer-step-undoing = Cofanie

## When an install cannot go ahead or stops

installer-carries-no-package = Ten instalator nie zawiera pakietu.
installer-stopped-unexpectedly = Instalacja nieoczekiwanie się zatrzymała.
# { $reason } is what went wrong, as the program says it.
installer-stopped-unexpectedly-because = Instalacja nieoczekiwanie się zatrzymała: { $reason }. Jeśli instalacja już się rozpoczęła, ponowne uruchomienie instalatora najpierw cofnie jej zmiany.
# { $error } is what went wrong reading it.
installer-package-read-failed = Nie można odczytać pakietu: { $error }
installer-no-programdata = Nie można odnaleźć folderu ProgramData na tym komputerze, więc niczego nie zainstalowano: instalacja przechowuje w nim zapis, dzięki któremu można ją cofnąć.
# { $error } is why, such as that there is nowhere only administrators can get into.
installer-staging-failed = Nie można utworzyć folderu przejściowego: { $error }
# { $error } is what went wrong unpacking it.
installer-unpack-failed = Nie można rozpakować pakietu: { $error }
installer-staging-nowhere = nie ma miejsca na rozpakowanie pakietu, do którego dostęp mają tylko administratorzy
# { $failure } is a folder and what went wrong there, such as "C:\ProgramData\WinAppMigrator: Access is denied.".
installer-staging-nowhere-look-at = nie ma miejsca na rozpakowanie pakietu, do którego dostęp mają tylko administratorzy. Sprawdź: { $failure }

## What came of an install, or of a preview

installer-result = Wynik instalacji
installer-result-preview = Podgląd: jeszcze niczego nie zmieniono
installer-result-installed = Aplikacja została zainstalowana i pomyślnie przeszła weryfikację skrótów.
installer-result-installed-without-left-out = Aplikacja została zainstalowana i pomyślnie przeszła weryfikację skrótów, bez elementów wymienionych poniżej jako pominięte.
installer-result-not-all-put-back = Instalacja nie powiodła się i nie wszystkie wprowadzone zmiany udało się cofnąć. Pozostałe elementy są wymienione poniżej. Następna instalacja spróbuje je cofnąć ponownie.
installer-result-cancelled-put-back = Instalacja została anulowana, a wprowadzone zmiany zostały cofnięte.
installer-result-failed-put-back = Instalacja nie powiodła się, a wprowadzone zmiany zostały cofnięte.
installer-result-not-clean = Instalacja nie zakończyła się bez problemów. Przejrzyj poniższe problemy.
installer-result-changes-put-back = Cofnięte zmiany
installer-result-changes-made = Wprowadzone zmiany
installer-result-changes-planned = Planowane zmiany
installer-result-warnings = Ostrzeżenia
installer-result-errors = Błędy
installer-result-package-valid = Skróty pakietu są prawidłowe
installer-result-package-invalid = Skróty pakietu są NIEPRAWIDŁOWE
installer-result-files-valid = Zainstalowane pliki są prawidłowe
installer-result-files-invalid = Zainstalowane pliki są NIEPRAWIDŁOWE
installer-result-reboot = Jedna z zależności wymaga ponownego uruchomienia, aby zakończyć instalację. Uruchom ponownie system Windows, a następnie uruchom aplikację.
# A heading over the list of what was changed. { $count } is how many.
installer-result-changes = Zmiany ({ $count })
# After the first part of a long list. { $count } is how many more there are.
installer-result-changes-more = i jeszcze { $count }.
# Over the path of the install's log file.
installer-log = Dziennik

## The installer's command line. Shown in English: scripts read it. A { $count } is plain digits, with no separators.

installer-cli-stub = Moduł startowy instalatora WinAppMigrator — nie zawiera pakietu aplikacji.
installer-cli-stub-how = Utwórz instalator za pomocą aplikacji WinAppMigrator, a następnie uruchom utworzony plik .exe na komputerze docelowym.
installer-cli-usage-title = Samorozpakowujący się instalator WinAppMigrator.
installer-cli-usage = Uruchom go bez argumentów, aby otworzyć instalator graficzny, lub:
installer-cli-usage-preview = podgląd ponownej instalacji: --dry-run [--target-root <FOLDER>]
installer-cli-usage-apply = ponowna instalacja:          --apply [--target-root <FOLDER>]
installer-cli-no-package = instalator nie zawiera osadzonego pakietu
installer-cli-no-programdata = nie można odnaleźć folderu ProgramData na tym komputerze, więc niczego nie zainstalowano: instalacja przechowuje w nim zapis, dzięki któremu można ją cofnąć
# { $path } is the folder asked for, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-folder-has-files = Nie można użyć folderu { $path }: { $problem }. Podaj opcję --force tylko wtedy, gdy wiesz, że należy on do tej aplikacji
installer-cli-no-location = ten pakiet nie określa, gdzie znajdowała się jego aplikacja, więc podaj opcję --target-root
# { $path } is where the app was on the other PC, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-location-refused = aplikacja z tego pakietu znajdowała się w folderze { $path }, a tej lokalizacji nie można użyć: { $problem }. Podaj opcję --target-root
# { $summary } is how much room it needs where, such as "up to 824.0 MB on C: (356.0 GB free)".
installer-cli-room = Miejsce: { $summary }
# { $note } is something to know about the install.
installer-cli-note = Uwaga: { $note }
# { $count } is how many things it sets up beyond its files.
installer-cli-sets-up = Konfigurowane elementy: { $count }
installer-cli-applied = Ponowna instalacja została wykonana.
installer-cli-previewed = Podgląd ponownej instalacji został wykonany.
# { $count } is how many changes the install logged.
installer-cli-changes-logged = Zmiany zapisane w dzienniku: { $count }
# { $count } is how many warnings.
installer-cli-warnings = Ostrzeżenia: { $count }
# { $count } is how many errors.
installer-cli-errors = Błędy: { $count }
# { $count } is how many things the package asked for and did not get.
installer-cli-refused = Odrzucone: { $count }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-package-valid = Skróty pakietu prawidłowe: { $valid }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-reinstall-valid = Skróty ponownej instalacji prawidłowe: { $valid }
# { $id } is the install's id, such as wam-0f3a….
installer-cli-install-id = Identyfikator instalacji: { $id }
installer-cli-put-back = Instalacja nie powiodła się, więc wprowadzone zmiany zostały cofnięte.
# { $path } is the log file.
installer-cli-log = Dziennik: { $path }
# { $path } is the log file. Printed before an error.
installer-cli-error-log = dziennik: { $path }
# { $error } is what went wrong.
installer-cli-error = błąd: { $error }

## The uninstaller's window

uninstaller-window-title = Dezinstalacja
# The window's title, and its heading. { $app_name } is the name of the app it uninstalls.
uninstaller-window-title-app = Dezinstalacja aplikacji { $app_name }
uninstaller-tagline = Cofa zmiany wprowadzone przez instalator
uninstaller-reading = Odczytywanie zapisu tej instalacji…
uninstaller-stopped-unexpectedly = Dezinstalacja nieoczekiwanie się zatrzymała.
uninstaller-what-will-be-removed = Co zostanie usunięte
uninstaller-install-location = Lokalizacja instalacji
uninstaller-files = Pliki
uninstaller-registry-values = Wartości rejestru
uninstaller-services = Usługi
uninstaller-drivers = Sterowniki
uninstaller-scheduled-tasks = Zaplanowane zadania
uninstaller-environment = Zmienne środowiskowe
uninstaller-plan-stays = Pliki dodane lub zmienione po instalacji pozostaną wraz z folderami, w których się znajdują.
# { $count } is how many files the install replaced.
uninstaller-plan-coming-back =
    { $count ->
        [one] { $count } plik zastąpiony przez tę instalację wróci na miejsce albo trafi obok pliku, który od tego czasu się zmienił.
        [few] { $count } pliki zastąpione przez tę instalację wrócą na miejsce albo trafią obok plików, które od tego czasu się zmieniły.
        [many] { $count } plików zastąpionych przez tę instalację wróci na miejsce albo trafi obok plików, które od tego czasu się zmieniły.
       *[other] { $count } plików zastąpionych przez tę instalację wróci na miejsce albo trafi obok plików, które od tego czasu się zmieniły.
    }
# { $runtimes } is a list of runtimes, such as "Microsoft Visual C++ Redistributable, .NET Desktop Runtime 8".
uninstaller-plan-runtimes = Zainstalowane środowiska uruchomieniowe pozostaną, ponieważ mogą z nich korzystać inne programy: { $runtimes }.
uninstaller-options = Opcje
uninstaller-remove-user-data = Usuń również dane użytkownika i dane licencji
# { $count } is how many items of the app's settings, data, and license there are.
uninstaller-remove-user-data-detail =
    { $count ->
        [one] { $count } element ustawień, danych i licencji aplikacji. Pozostanie, chyba że ta opcja jest włączona. To, co ten element zastąpił, wróci na miejsce albo trafi obok niego, jeśli od tego czasu się zmienił.
        [few] { $count } elementy ustawień, danych i licencji aplikacji. Pozostaną, chyba że ta opcja jest włączona. To, co te elementy zastąpiły, wróci na miejsce albo trafi obok nich, jeśli od tego czasu się zmieniły.
        [many] { $count } elementów ustawień, danych i licencji aplikacji. Pozostaną, chyba że ta opcja jest włączona. To, co te elementy zastąpiły, wróci na miejsce albo trafi obok nich, jeśli od tego czasu się zmieniły.
       *[other] { $count } elementów ustawień, danych i licencji aplikacji. Pozostaną, chyba że ta opcja jest włączona. To, co te elementy zastąpiły, wróci na miejsce albo trafi obok nich, jeśli od tego czasu się zmieniły.
    }
uninstaller-uninstall = Odinstaluj
uninstaller-cancel = Anuluj
uninstaller-close = Zamknij
# { $done } and { $total } are how many of the install's items are undone, and how many there are.
uninstaller-step-counted = Cofanie instalacji: { $done } z { $total }…
uninstaller-step = Cofanie instalacji…
uninstaller-done = Aplikacja została odinstalowana.
uninstaller-done-except = Aplikacja została odinstalowana, z wyjątkiem elementów wymienionych poniżej.
uninstaller-waits-for-drive = Część aplikacji znajduje się na dysku, który nie jest podłączony. Podłącz go i ponownie odinstaluj aplikację w Ustawieniach.
uninstaller-not-all-undone = Nie udało się cofnąć wszystkiego. Zamknij programy, które korzystają z tych elementów, i ponownie odinstaluj aplikację w Ustawieniach.
# Over the path of the uninstall's log file.
uninstaller-log = Dziennik
uninstaller-undone = Cofnięte
uninstaller-left-in-place = Zachowane
uninstaller-could-not-be-undone = Nie można cofnąć
# After the first part of a long list. { $count } is how many more there are.
uninstaller-and-more = i jeszcze { $count }.
# Said as the reason when there is no record of the install to read.
uninstaller-no-programdata = nie można odnaleźć folderu ProgramData na tym komputerze, więc nie ma zapisu instalacji do odczytania

## When the uninstaller's window cannot open. Each is a paragraph of a message box, followed by a command to type.

uninstaller-window-failed-title = Dezinstalator WinAppMigrator
# { $error } is what went wrong, as Windows or the graphics driver says it.
uninstaller-window-failed = Nie można otworzyć okna dezinstalatora: { $error }
uninstaller-window-failed-quiet = Dezinstalacja jest możliwa również bez okna, w wierszu polecenia administratora:
# Stands in for this program's own path in that command, if it cannot be found.
uninstaller-this-program = ten program

## The uninstaller's command line (--quiet). Shown in English: scripts read it.

# { $app_name } is the name of the app.
uninstaller-cli-uninstalling = Odinstalowywanie aplikacji { $app_name }.
# { $path } is the log file.
uninstaller-cli-log = Dziennik: { $path }
# { $error } is what went wrong.
uninstaller-cli-error = błąd: { $error }
