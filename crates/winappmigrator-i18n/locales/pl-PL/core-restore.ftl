# Words the core writes for people as it installs an app from a package: why an install will not start, the list of
# what it did (or for a preview, what it would do) that the installer window shows, its warnings, the record it keeps of
# each install so that it can be undone, and the core's errors. Message names start with core-.
#
# The list of what an install did is short and plain, one line for each thing, as the installer window lists them:
# "copied", "restored", "would copy". Words such as --force and --apply are command-line options, and LocalService,
# SYSTEM, ProgramData, and %AppData% are as Windows names them: keep them as they are.


## Errors: what went wrong, when the core cannot go on. Each is shown on its own, or after words that say what was being
## done.

core-error-list-changed = to, co ta instalacja miała wykonać, zmieniło się od czasu jej zatwierdzenia, więc niczego nie zainstalowano
# { $path } is a file or folder, such as C:\Program Files\App\app.exe, or words that name one, such as "the installer".
# { $error } is what Windows said went wrong.
core-error-io = operacja wejścia/wyjścia nie powiodła się ({ $path }): { $error }
# { $detail } is what went wrong.
core-error-registry = odczyt rejestru nie powiódł się: { $detail }
# { $path } is a program file. { $detail } is what was wrong with it.
core-error-pe = analiza pliku PE nie powiodła się ({ $path }): { $detail }
# { $detail } is what was wrong, already in words.
core-error-invalid-input = nieprawidłowe dane wejściowe: { $detail }
# { $detail } is what went wrong.
core-error-serialization = serializacja raportu nie powiodła się: { $detail }
# { $detail } is what went wrong.
core-error-archive = operacja na archiwum nie powiodła się: { $detail }
# { $detail } is what went wrong.
core-error-network = operacja sieciowa nie powiodła się: { $detail }
# { $detail } is what went wrong.
core-error-time-format = formatowanie czasu nie powiodło się: { $detail }
core-error-cancelled = operacja została anulowana
# What is said when part of the program stopped unexpectedly and gave no reason.
core-error-panic-no-reason = nie podano przyczyny

## The record of each install, kept so that it can be undone.

# { $kind } is a number that says what type of registry value it is.
core-journal-unknown-value-type = nieznany typ wartości rejestru { $kind }
core-journal-not-elevated = brak podwyższonych uprawnień
core-journal-needs-admin = instalacja wymaga uprawnień administratora, aby jej zapis był przechowywany w miejscu, którego inni nie mogą zmienić
# { $path } is a folder.
core-journal-not-an-install-folder = to nie jest folder instalacji: { $path }
# { $path } is the file the record is kept in.
core-journal-not-written = nie można utworzyć zapisu tej instalacji: { $path }
# { $path } is a folder.
core-journal-path-escapes-root = ścieżka wykracza poza zamierzony katalog główny: { $path }
# { $path } is a folder.
core-journal-nothing-exists = nic z tego nie istnieje na tym komputerze: { $path }
core-journal-busy = trwa inna instalacja lub dezinstalacja. Zaczekaj na jej zakończenie, a następnie spróbuj ponownie
# Said of a file that was to get back the permissions it had.
core-journal-no-folder = nie ma folderu nadrzędnego
# Said of the permissions a file had, which were kept to give back to it.
core-journal-permissions-unreadable = nie można odczytać zachowanych uprawnień
# { $folder } is the folder of one install's record.
core-journal-unreadable-empty = nie można użyć zapisu instalacji w folderze { $folder }: zapis jest pusty
# { $folder } is the folder of one install's record.
core-journal-unreadable-no-header = nie można użyć zapisu instalacji w folderze { $folder }: zapis nie zaczyna się od informacji o tym, czego dotyczy
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-version = nie można użyć zapisu instalacji w folderze { $folder }: zapis został utworzony przez inną wersję tego programu
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-install = nie można użyć zapisu instalacji w folderze { $folder }: według zapisu należy on do innej instalacji
# { $id } is an install's id, such as wam-1a2b3c.
core-journal-not-an-install-id = to nie jest identyfikator instalacji: { $id }

## Why an install will not start, or stopped before it changed anything.

core-restore-apply-and-dry-run = nie można jednocześnie podać opcji --apply i --dry-run
# { $path } is the folder that was chosen to install in.
core-restore-location-not-full-path = lokalizacja instalacji musi być pełną ścieżką: „{ $path }”
# { $problem } says what is wrong with the folder, already in words. { $path } is the folder.
core-restore-location-unusable = nie można użyć tej lokalizacji instalacji: { $problem }: „{ $path }”
core-restore-needs-admin-plan = plan przywracania obejmuje zmiany rejestru, usług, zaplanowanych zadań, skrótów lub chronionych lokalizacji docelowych, które wymagają uprawnień administratora
# { $path } is the folder that was chosen to install in.
core-restore-needs-admin-target = docelowy katalog główny wymaga podwyższonych uprawnień: { $path }
core-restore-needs-admin-user-data = przywracanie danych użytkownika do folderu ProgramData lub innych chronionych lokalizacji wymaga podwyższonych uprawnień
# { $error } is why the install stopped. { $failed } is what could not be put back, as sentences of their own.
core-restore-not-all-put-back = { $error }. Nie wszystko, co zostało już wykonane, udało się cofnąć: { $failed }
# { $error } is what went wrong.
core-restore-nowhere-to-keep-record = nie ma bezpiecznego miejsca na zapis, który umożliwiłby cofnięcie tej instalacji, więc niczego nie zainstalowano: { $error }
# { $path } is a file the app would have put there.
core-restore-already-exists = plik { $path } już istnieje, więc niczego nie zainstalowano. Użyj opcji --force, aby go zastąpić
# { $root } is a folder the app has files for. { $link } is that folder, or one it is in.
core-restore-root-under-link = ta aplikacja ma elementy do przywrócenia w lokalizacji { $root }, a { $link } jest linkiem do innego miejsca, które inni mogą zmieniać, więc niczego nie zainstalowano. Jeśli trafiają tam dane użytkownika lub dane licencji, zainstaluj aplikację bez danych użytkownika i danych licencji. Jeśli jest to skrót w folderze Autostart, przywróć ten folder do jego pierwotnej lokalizacji
core-restore-hash-check-failed = pakiet nie przeszedł weryfikacji skrótów i niczego nie zainstalowano
# { $detail } is the first thing found wrong, already in words.
core-restore-hash-check-failed-because = pakiet nie przeszedł weryfikacji skrótów i niczego nie zainstalowano: { $detail }
# { $path } is a file or folder.
core-restore-refuse-records-folder = odmowa zapisu w miejscu, w którym instalacje przechowują swoje zapisy: { $path }
# { $path } is a file or folder.
core-restore-refuse-outside = odmowa zapisu poza lokalizacją instalacji i znanymi folderami danych: { $path }
# { $path } is a file.
core-restore-no-parent-folder = miejsce docelowe nie ma folderu nadrzędnego: { $path }
# { $path } is where a file was to go.
core-restore-folder-in-the-way = w miejscu, w którym ma się znaleźć ten plik, jest folder: { $path }
# { $path } is a file in the package.
core-restore-file-changed = plik { $path } nie jest już taki sam jak w pakiecie, więc nie został zainstalowany
# { $path } is a file in the package.
core-restore-source-is-link = element { $path } jest linkiem, za którym nie należy podążać
# { $path } is a file in the package.
core-restore-source-became-link = element { $path } stał się linkiem podczas otwierania
# Why a service did not start, after "did not start:".
core-restore-service-stopped-by-itself = ponownie zatrzymała się sama
# Why part of the app was left out, after "was left out:".
core-restore-com-class-someone-elses = ta klasa COM już istnieje na tym komputerze i należy do czegoś innego

## What an install did, one line for each thing, or for a preview, what it would do. { $source } is a file in the
## package, and { $destination } and { $path } are where it goes on this PC. { $key } is a registry key, such as
## HKLM\Software\App.

# { $package_valid } is true or false, and { $restore_valid } is true, false, or not_run: keep them as they are.
core-restore-verification-result = weryfikacja po przywróceniu: package_valid={ $package_valid } restore_valid={ $restore_valid }
# { $name } is a service's name.
core-restore-started-service = uruchomiono usługę { $name }
core-restore-would-copy = do skopiowania: { $source } -> { $destination }
core-restore-copied = skopiowano { $source } -> { $destination }
core-restore-would-create-empty-folder = do utworzenia: pusty folder { $path }
core-restore-created-empty-folder = utworzono pusty folder { $path }
# { $value } is the name of a value in the key.
core-restore-would-restore-registry-value = do przywrócenia: wartość rejestru { $key } { $value }
core-restore-would-restore-registry-key = do przywrócenia: klucz rejestru { $key }
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-would-add-firewall-rule = do dodania: reguła zapory { $name } dla programu { $program }
# { $name } is a firewall rule's name.
core-restore-firewall-rule-already-there = reguła zapory { $name } już istnieje
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-added-firewall-rule = dodano regułę zapory { $name } dla programu { $program }
# { $name } is a service's name, { $command } what it runs, and { $account } the account it runs as, such as LocalService.
core-restore-would-restore-service = do przywrócenia: usługa { $name } -> { $command } (działa jako { $account })
# { $name } is a runtime, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-restore-prerequisite-present = wymaganie wstępne jest już spełnione: { $name }
# { $name } is a runtime that Windows has as a Windows feature.
core-restore-would-install-prerequisite-feature = do zainstalowania: wymaganie wstępne { $name } (funkcja systemu Windows)
# { $name } is a runtime whose installer is in the package.
core-restore-would-install-prerequisite-bundled = do zainstalowania: wymaganie wstępne { $name } (dołączony instalator)
# { $name } is a runtime that would come through winget, which is a program's name.
core-restore-would-install-prerequisite-winget = do zainstalowania: wymaganie wstępne { $name } (winget)
# { $name } is a runtime.
core-restore-installed-prerequisite = zainstalowano wymaganie wstępne { $name }
# { $name } is a runtime.
core-restore-installed-prerequisite-reboot = zainstalowano wymaganie wstępne { $name } (wymagane ponowne uruchomienie)
core-restore-would-restore-license = do przywrócenia: licencja { $source } -> { $destination }
core-restore-restored-license = przywrócono licencję { $source } -> { $destination }
core-restore-would-restore-license-registry = do przywrócenia: licencja w rejestrze { $key }
# { $name } is a driver's name.
core-restore-driver-not-restored = sterownik nie jest przywracany automatycznie: { $name }
# { $name } is a driver's name.
core-restore-would-install-driver = do zainstalowania: sterownik { $name } -> { $destination }
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-would-restore-task = do przywrócenia: zaplanowane zadanie { $name }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-startup-shortcut = do przywrócenia: skrót autostartu { $shortcut } -> { $target }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-shortcut = do przywrócenia: skrót { $shortcut } -> { $target }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-would-restore-variable =
    { $scope ->
        [user] do przywrócenia: zmienna środowiskowa użytkownika { $name }
       *[machine] do przywrócenia: systemowa zmienna środowiskowa { $name }
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-left-variable =
    { $scope ->
        [user] pozostawiono bez zmian zmienną środowiskową użytkownika { $name }: żadna jej część nie należy do tej aplikacji
       *[machine] pozostawiono bez zmian systemową zmienną środowiskową { $name }: żadna jej część nie należy do tej aplikacji
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-restored-variable =
    { $scope ->
        [user] przywrócono zmienną środowiskową użytkownika { $name }
       *[machine] przywrócono systemową zmienną środowiskową { $name }
    }
core-restore-set-uninstaller = ustawiono dezinstalator dla klucza { $key }
core-restore-placed-driver-image = umieszczono obraz sterownika { $source } -> { $destination }
# { $name } is a driver's name.
core-restore-configured-driver = skonfigurowano sterownik { $name }
# { $name } is a driver's name.
core-restore-created-driver = utworzono sterownik { $name }
# { $name } is a driver's name.
core-restore-started-driver = uruchomiono sterownik { $name }
core-restore-ensured-registry-key = zapewniono istnienie klucza rejestru { $key }
# { $value } is the name of a value in the key.
core-restore-restored-registry-value = przywrócono wartość rejestru { $key } { $value }
# "(Default)" is how Windows names the value a key has with no name: write it as Registry Editor does.
core-restore-restored-registry-default = przywrócono wartość rejestru { $key } (Domyślna)
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-configured-service = skonfigurowano usługę { $name } (działa jako { $account })
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-created-service = utworzono usługę { $name } (działa jako { $account })
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-restored-task = przywrócono zaplanowane zadanie { $name }
core-restore-restored-shortcut = przywrócono skrót { $path }

## Warnings an install gives, and why it failed. They are shown after it, on the PC it ran on.

core-warning-restore-earlier-not-trusted = Zapis instalacji, która nie została ukończona, pozostawiono bez zmian, ponieważ inni mogą go zmieniać. Wszystko, co zmieniła tamta instalacja, nadal pozostaje na miejscu.
# { $app_name } is the app's name. { $count } is how many changes were put back.
core-warning-restore-earlier-undone =
    { $count ->
        [one] Wcześniejsza instalacja aplikacji { $app_name } nie została ukończona, więc najpierw ją cofnięto. Cofnięto { $count } zmianę.
        [few] Wcześniejsza instalacja aplikacji { $app_name } nie została ukończona, więc najpierw ją cofnięto. Cofnięto { $count } zmiany.
        [many] Wcześniejsza instalacja aplikacji { $app_name } nie została ukończona, więc najpierw ją cofnięto. Cofnięto { $count } zmian.
       *[other] Wcześniejsza instalacja aplikacji { $app_name } nie została ukończona, więc najpierw ją cofnięto. Cofnięto { $count } zmian.
    }
# { $app_name } is the app's name.
core-warning-restore-earlier-never-finished = Wcześniejsza instalacja aplikacji { $app_name } nie została ukończona. Instalacja najpierw ją cofnie.
core-warning-restore-user-data-real-locations = Dane użytkownika są przywracane do rzeczywistych lokalizacji %AppData%/%ProgramData% na tym komputerze, a nie do docelowego katalogu głównego.
# { $error } is what went wrong.
core-warning-restore-commit-failed = Nie można ukończyć zapisu tej instalacji: { $error }
# { $name } is a service's name.
core-warning-restore-service-still-starting = Usługa { $name } otrzymała polecenie uruchomienia, ale jeszcze się nie uruchomiła. Może to jeszcze nastąpić. Usługa uruchomi się też sama przy następnym uruchomieniu komputera.
# { $name } is a service's name. { $reason } is why it did not start, as Windows said it.
core-warning-restore-service-not-started = Usługa { $name } została zainstalowana, ale się nie uruchomiła: { $reason }. Aplikacja może nie działać, dopóki usługa się nie uruchomi. System Windows spróbuje ponownie przy następnym uruchomieniu komputera.
# { $name } is a service's name. { $reason } is why it could not be stopped.
core-warning-restore-service-not-stopped = Nie można było najpierw zatrzymać usługi { $name }: { $reason }. Do następnego uruchomienia komputera może ona nadal uruchamiać stary program.
core-warning-restore-install-cancelled = Instalacja została anulowana.
core-warning-restore-preview-cancelled = Podgląd został anulowany.
# { $item } is what was left out, such as a file or a service's name. { $reason } is why, already in words.
core-warning-restore-left-out = Pominięto element { $item }: { $reason }.
# { $error } is what went wrong.
core-warning-restore-verification-failed = Nie można przeprowadzić weryfikacji po przywróceniu: { $error }
# { $runner } says what relies on the file: a service, a scheduled task, a firewall rule, a shortcut, or a registry value.
# { $path } is the file. { $name } is the name of what relies on it. { $refusal } is why the file was not replaced,
# already in words.
core-warning-restore-program-would-run =
    { $runner ->
        [service] Ten pakiet nie może zastąpić pliku { $path }, a zależy od niego usługa { $name }, więc niczego nie zainstalowano: { $refusal }
        [task] Ten pakiet nie może zastąpić pliku { $path }, a zależy od niego zaplanowane zadanie { $name }, więc niczego nie zainstalowano: { $refusal }
        [firewall] Ten pakiet nie może zastąpić pliku { $path }, a zależy od niego reguła zapory { $name }, więc niczego nie zainstalowano: { $refusal }
        [shortcut] Ten pakiet nie może zastąpić pliku { $path }, a zależy od niego skrót { $name }, więc niczego nie zainstalowano: { $refusal }
       *[registry] Ten pakiet nie może zastąpić pliku { $path }, a zależy od niego wartość rejestru { $name }, więc niczego nie zainstalowano: { $refusal }
    }
core-warning-restore-destination-exists = Miejsce docelowe już istnieje. Użyj opcji --force, aby je zastąpić.
# { $path } is a file. { $error } is what went wrong.
core-warning-restore-file-not-written = Nie można zapisać pliku { $path }: { $error }
core-warning-restore-not-a-folder = W miejscu, w którym ma się znaleźć ten pusty folder, znajduje się coś innego niż folder.
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-not-looked-at = Nie można sprawdzić folderu { $path }: { $error }
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-folder-not-made = Nie można utworzyć folderu { $path }: { $error }
# Why part of the app was left out, after "was left out:".
core-warning-restore-registration-exists = ten element już istnieje na tym komputerze z innymi ustawieniami, a instalacja go nie przejmuje
core-warning-restore-registry-review = Wartości rejestru zostały przywrócone z zarchiwizowanych danych strukturalnych, a znane ścieżki aplikacji zostały zmienione tak, aby wskazywały docelowy katalog główny.
# { $error } is what went wrong.
core-warning-restore-firewall-unreadable = Nie można odczytać reguł Zapory systemu Windows: { $error }
# Why a firewall rule was left out, after "was left out:".
core-warning-restore-firewall-rule-exists = reguła o tej nazwie dla tego programu już istnieje z innymi ustawieniami, a instalacja jej nie zmienia
# { $name } is a runtime.
core-warning-restore-prerequisite-unknown = { $name }: brak znanego instalatora tego środowiska uruchomieniowego.
# { $name } is a runtime.
core-warning-restore-prerequisite-needs-admin = Instalacja środowiska uruchomieniowego { $name } wymaga podwyższonych uprawnień; uruchom instalator ponownie jako administrator.
# { $name } is a runtime.
core-warning-restore-prerequisite-reboot = Środowisko uruchomieniowe { $name } zostało zainstalowane, ale do ukończenia instalacji wymagane jest ponowne uruchomienie.
# { $error } is what went wrong.
core-warning-restore-skipped-not-recorded = Nie można zapisać informacji o pominiętym środowisku uruchomieniowym: { $error }
# { $name } is a runtime. { $reason } is why it was skipped, already in words.
core-warning-restore-prerequisite-skipped = { $name }: { $reason }
# { $name } is a runtime. { $reason } is why it could not be installed. { $url } is a web address to get it from.
core-warning-restore-prerequisite-failed = Nie można zainstalować środowiska uruchomieniowego { $name }: { $reason }. Pobierz je ze strony { $url }.
# { $error } is what went wrong.
core-warning-restore-runtime-not-recorded = Nie można zapisać informacji o zainstalowanym środowisku uruchomieniowym: { $error }
# { $error } is what went wrong.
core-warning-restore-license-source-invalid = Ścieżka licencji w pakiecie jest nieprawidłowa: { $error }
core-warning-restore-license-needs-admin = Przywracanie danych licencji do chronionej lokalizacji wymaga podwyższonych uprawnień.
core-warning-restore-license-exists = Miejsce docelowe licencji już istnieje. Użyj opcji --force, aby je zastąpić.
# { $error } is what went wrong.
core-warning-restore-license-failed = Nie można przywrócić danych licencji: { $error }
core-warning-restore-driver-needs-admin = Rejestracja sterownika wymaga podwyższonych uprawnień; uruchom instalator ponownie jako administrator.
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-warning-restore-task-runs-as-system = Zaplanowane zadanie { $name } działa jako SYSTEM, z pełnymi uprawnieniami do tego komputera.
core-warning-restore-shortcut-exists = Skrót już istnieje. Użyj opcji --force, aby go zastąpić.
core-warning-restore-shortcut-all-users = Co najmniej jeden przywrócony skrót znajduje się na pulpicie publicznym lub we wspólnym menu Start.
core-warning-restore-environment-refresh = Zmiany zmiennych środowiskowych mogą być widoczne dopiero w nowym procesie, nowej powłoce lub po ponownym zalogowaniu.
# { $name } is a service's name. { $account } is the account it ran as on the PC it came from.
core-warning-restore-service-account = Usługa { $name } działała na komputerze źródłowym jako { $account }. Jej hasła nie ma w instalatorze, więc usługa została skonfigurowana do działania jako LocalService. Jeśli nie działa, ustaw jej konto i hasło w przystawce Usługi.
# { $name } is a runtime.
core-warning-restore-prerequisite-pending-reboot = Środowisko uruchomieniowe { $name } nie jest jeszcze zarejestrowane. Powinno stać się aktywne po ponownym uruchomieniu.
# { $name } is a runtime.
core-warning-restore-prerequisite-not-verified = Instalator środowiska uruchomieniowego { $name } zgłosił powodzenie, ale środowisko nie zostało zarejestrowane. Instalacja mogła się nie powieść.
# { $error } is what went wrong.
core-warning-restore-driver-source-invalid = Ścieżka dołączonego sterownika jest nieprawidłowa: { $error }
# { $error } is what went wrong.
core-warning-restore-driver-not-read = Nie można odczytać bieżących ustawień sterownika: { $error }
core-warning-restore-driver-exists = Usługa sterownika już istnieje. Użyj opcji --force, aby skonfigurować ją ponownie.
core-warning-restore-driver-image-exists = Obraz sterownika już istnieje. Użyj opcji --force, aby go zastąpić.
core-warning-restore-driver-hash-bundled = Dołączony obraz sterownika nie odpowiada zapisanemu skrótowi, więc nie został zainstalowany.
# { $error } is what went wrong.
core-warning-restore-driver-not-placed = Nie można umieścić obrazu sterownika: { $error }
core-warning-restore-driver-hash-placed = Obraz sterownika zmienił się po umieszczeniu, więc nie został zarejestrowany.
# { $error } is what went wrong.
core-warning-restore-key-not-created = Nie można utworzyć klucza rejestru: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-read = Nie można odczytać wartości, którą ta zmiana miałaby zastąpić: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-written = Nie można zapisać wartości rejestru: { $error }
core-warning-restore-value-exists = Wartość rejestru już istnieje. Użyj opcji --force, aby ją zastąpić.
# { $error } is what went wrong.
core-warning-restore-environment-key-not-created = Nie można utworzyć klucza rejestru zmiennych środowiskowych: { $error }
core-warning-restore-variable-exists = Zmienna środowiskowa już istnieje. Użyj opcji --force, aby ją zastąpić.
# { $error } is what went wrong.
core-warning-restore-variable-not-written = Nie można zapisać zmiennej środowiskowej: { $error }
core-warning-restore-value-kind-missing = Brak typu wartości rejestru.
# { $error } is what went wrong.
core-warning-restore-service-not-read = Nie można odczytać bieżących ustawień usługi: { $error }
core-warning-restore-service-exists = Usługa już istnieje. Użyj opcji --force, aby skonfigurować ją ponownie.
# { $error } is what went wrong.
core-warning-restore-task-not-queried = Nie można odczytać zaplanowanego zadania: { $error }
core-warning-restore-task-exists = Zaplanowane zadanie już istnieje. Użyj opcji --force, aby je zastąpić.
# { $error } is what went wrong.
core-warning-restore-task-folder-not-looked-at = Nie można sprawdzić folderu zadania: { $error }
# { $error } is what went wrong.
core-warning-restore-shortcut-not-put-back = Nie można utworzyć skrótu ani przywrócić skrótu, który miał on zastąpić: { $error }
# { $error } is what went wrong.
core-warning-restore-command-not-started = Nie można uruchomić polecenia: { $error }
# { $output } is what the command wrote, as it wrote it.
core-warning-restore-command-failed = Polecenie nie powiodło się: { $output }
# { $error } is what went wrong.
core-warning-restore-powershell-not-started = Nie można uruchomić programu PowerShell: { $error }
