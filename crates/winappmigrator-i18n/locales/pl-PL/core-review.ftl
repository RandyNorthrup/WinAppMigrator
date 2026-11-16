# Words the core says about what an install may do: why an install location or something in a package is refused,
# the installer's list of what an app sets up beyond its files, the checks of a package and of an install, and whether
# there is room for either. Message names start with core-.
#
# Many messages here are a reason, the second half of a sentence that starts "... was left out:" or "that install
# location cannot be used:", so they start with a small letter and have no full stop.


## Why an install location cannot be used. Shown in red under the Install Location field.

core-review-location-state-root = jest to miejsce, w którym instalacje przechowują swoje zapisy
core-review-location-has-files = zawiera już pliki, a nie ma tu zapisanej żadnej instalacji, więc użyj pustego folderu
core-policy-location-not-full-path = musi to być pełna ścieżka
core-policy-location-parent-step = ścieżka nie może zawierać „..”
core-policy-location-not-plain = musi to być zwykła ścieżka na dysku tego komputera
core-policy-location-whole-drive = nie może to być cały dysk
core-policy-location-windows = nie może to być folder Windows
core-policy-location-runs-by-itself = jest to folder należący do systemu Windows lub folder, którego zawartość jest automatycznie uruchamiana lub ładowana
core-policy-location-shared = zawiera obok siebie inne elementy, więc aplikacja musi mieć w nim własny podfolder

## Whether an app can run on this PC at all, said before anything is installed.

# { $reason } is why, as the second half of a sentence, such as "App.exe is built for Arm64, and this PC is x64, so
# Windows would not start it".
core-review-cannot-run-here = ta aplikacja nie może działać na tym komputerze, więc niczego nie zmieniono: { $reason }
core-policy-architecture-x86 = 32-bitowa x86
core-policy-architecture-x64 = x64
core-policy-architecture-arm64 = Arm64
core-policy-architecture-arm32 = 32-bitowa Arm
core-policy-architecture-unknown = nieznana architektura
# { $code } is the number Windows gives a kind of processor, such as 0x01c4.
core-policy-machine-type = typ maszyny { $code }
# { $names } is a list of program names, such as "a.exe, b.exe, c.exe", and { $count } how many more there are.
core-policy-names-and-more = { $names } i jeszcze { $count }
core-policy-fit-programs-not-said = Ten instalator nie określa, czego potrzebują jego programy, więc nie sprawdzono, czy mogą one działać na tym komputerze.
core-policy-fit-architecture-not-told = Nie można ustalić architektury tego komputera, więc nie sprawdzono, czy programy tej aplikacji mogą na nim działać.
core-policy-fit-main-not-said = Ten instalator nie określa, czego potrzebuje jego program główny, więc podane tu informacje opierają się na pozostałej zawartości instalatora.
# { $driver } is a driver's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-driver = architektura jej sterownika { $driver } to { $built_for }, a architektura tego komputera to { $here }. Sterownik musi dokładnie pasować do systemu Windows, więc ta aplikacja nie może tu działać
# { $program } is a program's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-program = architektura programu { $program } to { $built_for }, a architektura tego komputera to { $here }, więc system Windows go nie uruchomi
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-none-start = żaden z jej programów nie może się uruchomić na tym komputerze, którego architektura to { $here }: { $programs }
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-some-other-architecture = Niektóre programy tej aplikacji są przeznaczone dla innej architektury i nie uruchomią się na tym komputerze, którego architektura to { $here }: { $programs }. Pozostała część aplikacji może działać, więc może to nie mieć znaczenia.
# { $here } is a kind of processor, such as "Arm64". { $programs } is a list of program names.
core-policy-fit-emulated = Architektura tego komputera to { $here }, więc system Windows uruchamiałby te programy przy użyciu emulacji: { $programs }. Większość programów działa w ten sposób poprawnie, ale niektóre nie.
# { $program } is a program's file name. { $needs } and { $runs } are versions of Windows, such as 10.0 and 6.1.
core-policy-fit-needs-newer = program { $program } wymaga systemu Windows { $needs } lub nowszego, a na tym komputerze działa system Windows { $runs }, więc system Windows go nie uruchomi
# { $programs } is a list of program names.
core-policy-fit-some-need-newer = Niektóre programy tej aplikacji wymagają nowszej wersji systemu Windows niż ta i nie uruchomią się tutaj: { $programs }.
# { $from_build } and { $this_build } are Windows build numbers, such as 22631, written as Windows writes them.
core-policy-fit-older-build = Ta aplikacja działała w systemie Windows w kompilacji { $from_build }, a na tym komputerze działa kompilacja { $this_build }. Jej programy deklarują, że mogą się uruchomić w tej wersji systemu Windows, więc aplikacja może działać, ale elementy, na których polegała tam, mogą tu nie być dostępne.
core-policy-fit-build-not-told = Nie można ustalić kompilacji tego systemu Windows, więc nie porównano jej z kompilacją, w której działała ta aplikacja.

## Room on a drive for a build or an install.

# Each of these ends the sentence core-space-shortfall, after "free for".
core-space-for-unpacking = rozpakowanie pakietu
core-space-for-install-record = zapis instalacji
core-space-for-app-files = pliki aplikacji
core-space-for-what-is-replaced = zachowanie zastępowanych elementów
core-space-for-build-work = pliki robocze tworzenia instalatora
core-space-for-installer = instalator
core-space-for-package = pakiet
core-space-for-unpacking-and-installing = rozpakowanie i zainstalowanie pakietu
# What goes between two of those, as in "unpacking the package and the app's files". Keep the quotes: they keep the
# spaces.
core-space-purposes-joiner = { " i " }
# { $drive } is a drive, such as C:, or a folder. { $needed } and { $free } are sizes, such as 1.2 GB. { $purposes } is
# what the room is for, such as "unpacking the package and the app's files".
core-space-shortfall = { $drive } potrzebuje do { $needed } wolnego miejsca na { $purposes }, a ma { $free }
# { $drive } is a drive, such as C:, or a folder. { $needed } is a size, such as 1.2 GB.
core-space-not-told = { $drive } nie podaje, ile ma wolnego miejsca, więc nie sprawdzono, czy zmieści się potrzebne { $needed }
# One drive in a list of where room is needed. { $needed } and { $free } are sizes, and { $drive } a drive, such as C:.
core-space-summary = do { $needed } na { $drive } (wolne: { $free })
# { $needed } is a size, and { $drive } a drive, such as C:.
core-space-summary-not-told = do { $needed } na { $drive } (nie można ustalić ilości wolnego miejsca)
# What goes between two sentences of core-space-shortfall inside core-review-no-room. Keep the quotes: they keep the
# space.
core-review-sentence-joiner = { ". " }
# { $shortfalls } is one or more sentences of core-space-shortfall.
core-review-no-room = za mało miejsca, więc niczego nie zmieniono. { $shortfalls }. Zwolnij miejsce na tym dysku lub wybierz lokalizację instalacji na innym dysku.

## The installer's list of what an app sets up beyond its files: what each kind is called.

core-review-kind-service = Usługa
core-review-kind-driver = Sterownik
core-review-kind-scheduled-task = Zaplanowane zadanie
core-review-kind-autostart = Autostart
core-review-kind-shortcut = Skrót
core-review-kind-path = PATH
core-review-kind-environment-variable = Zmienna środowiskowa
core-review-kind-shell-extension = Rozszerzenie powłoki
core-review-kind-menu-command = Polecenie menu
core-review-kind-link-type = Typ linku
core-review-kind-file-type = Typ pliku
core-review-kind-com = COM
core-review-kind-firewall-rule = Reguła zapory
core-review-kind-event-log-source = Źródło dziennika zdarzeń
core-review-kind-font = Czcionka
core-review-kind-odbc-driver = Sterownik ODBC
core-review-kind-odbc-source = Źródło danych ODBC
core-review-kind-compatibility-setting = Ustawienie zgodności
core-review-kind-default-apps = Aplikacje domyślne
core-review-kind-program-name = Nazwa programu
core-review-kind-other-settings = Inne ustawienia

## The installer's list of what an app sets up: what each line says of it.

# { $file } is a program's file name. { $account } is the Windows account it runs as, such as LocalSystem. { $starts }
# says when it starts, and is not shown itself; for "other", { $start } is one of the core-review-start- words below.
core-review-service-detail =
    { $starts ->
        [automatic] Uruchamia plik { $file } jako { $account }; uruchamianie: od razu i przy każdym uruchomieniu komputera
        [delayed] Uruchamia plik { $file } jako { $account }; uruchamianie: od razu i wkrótce po każdym uruchomieniu komputera
        [manual] Uruchamia plik { $file } jako { $account }; uruchamianie: tylko na żądanie
        [disabled] Uruchamia plik { $file } jako { $account }; uruchamianie: wyłączone
        [other] Uruchamia plik { $file } jako { $account }; uruchamianie: { $start }
       *[unknown] Uruchamia plik { $file } jako { $account }
    }
# { $account } is a Windows account, such as LocalService.
core-policy-account-custom = { $account } (konto niestandardowe nie zostanie przywrócone)
# { $file } is a driver's file name.
core-review-driver-detail = Ładuje plik { $file } bezpośrednio do systemu Windows
# { $file } is a driver's file name. { $start } is when Windows starts it: one of the core-review-start- words below.
core-review-driver-detail-starts = Ładuje plik { $file } bezpośrednio do systemu Windows; uruchamianie: { $start }
# When a service or driver starts, ending a sentence such as "Loads disk.sys into Windows itself, starts …".
core-review-start-boot = razem z systemem Windows, przed wszystkim innym
core-review-start-system = razem z systemem Windows, podczas jego uruchamiania
core-review-start-automatic = przy każdym uruchomieniu komputera
core-review-start-delayed = wkrótce po każdym uruchomieniu komputera
core-review-start-manual = tylko na żądanie
core-review-start-disabled = nigdy, ponieważ jest wyłączone
core-review-start-unknown = w sposób, którego system Windows nie określa
# { $account } is the account a task runs as: an account's name, or core-review-task-runs-as-you, or
# core-review-task-runs-as-creator.
core-review-task-detail = Działa jako { $account }
# { $account } is the account a task runs as, as for core-review-task-detail.
core-review-task-detail-highest = Działa jako { $account }, z najwyższymi możliwymi uprawnieniami
core-review-task-runs-as-you = ty, czyli zalogowany użytkownik
core-review-task-runs-as-creator = użytkownik, który je utworzył
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-autostart-detail =
    Uruchamia się { $scope ->
        [everyone] przy logowaniu każdego użytkownika
       *[you] przy logowaniu bieżącego użytkownika
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shortcut-startup-detail =
    Skrót w folderze Autostart, { $scope ->
        [everyone] dla wszystkich użytkowników tego komputera
       *[you] tylko dla bieżącego użytkownika
    }
# { $file } is the file name of what the shortcut opens. { $scope } is whom it is set up for: everyone on this PC,
# or you. It is not shown itself.
core-review-shortcut-detail =
    Otwiera plik { $file }, { $scope ->
        [everyone] dla wszystkich użytkowników tego komputera
       *[you] tylko dla bieżącego użytkownika
    }
# { $name } is an environment variable's name, such as PATH.
core-review-path-title = Dodaje do zmiennej { $name }
core-review-path-install-folder = Folder instalacji
# { $path } is a folder inside the install folder, such as bin\tools.
core-review-path-below-install-folder = Folder instalacji\{ $path }
# { $value } is the variable's value, as it will be set.
core-review-variable-detail = Wartość: { $value }
# { $count } is how many registry values.
core-review-other-settings-title =
    { $count ->
        [one] { $count } ustawienie poza własnymi kluczami aplikacji
        [few] { $count } ustawienia poza własnymi kluczami aplikacji
        [many] { $count } ustawień poza własnymi kluczami aplikacji
       *[other] { $count } ustawień poza własnymi kluczami aplikacji
    }
# { $keys } is a list of registry keys, such as HKCU\Software\Vendor.
core-review-other-settings-detail = Klucze: { $keys }
# { $keys } is a list of registry keys, and { $count } how many more there are.
core-review-other-settings-detail-more = Klucze: { $keys } i jeszcze { $count }
# { $way } says whether the rule allows or blocks, and incoming or outgoing connections; it is not shown itself.
# { $ports } is a list of port numbers, and { $port_count } how many. { $program } is the program's file name.
# { $networks } is a list of the kinds of network Windows names in English (domain, private, public), and
# { $network_count } how many.
# The kinds of network a firewall rule is for, as Windows Defender Firewall names its profiles in your language. They
# fill { $networks } in core-review-firewall-detail, joined into a list.
core-review-network-domain = Domena
core-review-network-private = Prywatny
core-review-network-public = Publiczny
core-review-firewall-detail =
    { $way ->
        [allow-in] Zezwala na przychodzące
        [allow-out] Zezwala na wychodzące
        [block-in] Blokuje przychodzące
       *[block-out] Blokuje wychodzące
    } { $port_count ->
        [0] połączenia
        [one] połączenia na porcie { $ports }
        [few] połączenia na portach { $ports }
        [many] połączenia na portach { $ports }
       *[other] połączenia na portach { $ports }
    } programu { $program } { $network_count ->
        [0] w każdej sieci
        [one] w profilu sieci { $networks }
        [few] w profilach sieci { $networks }
        [many] w profilach sieci { $networks }
       *[other] w profilach sieci { $networks }
    }
# { $server } is the file name of what Explorer loads. { $scope } is whom it is set up for: everyone on this PC, or
# you. It is not shown itself.
core-review-shell-extension-detail =
    Ładuje plik { $server } do Eksploratora plików, { $scope ->
        [everyone] dla wszystkich użytkowników tego komputera
       *[you] tylko dla bieżącego użytkownika
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shell-extension-detail-own-file =
    Ładuje do Eksploratora plików instalowany przez siebie plik, { $scope ->
        [everyone] dla wszystkich użytkowników tego komputera
       *[you] tylko dla bieżącego użytkownika
    }
# The name of a shell extension: { $handler } is its name in the registry, and { $class } the kind of file or folder it
# is for, such as * or Directory.
core-policy-shell-extension-name = { $handler } dla { $class }
# { $on } is the kind of file or folder whose menu it is, as the registry names it, such as * or
# Directory. { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-menu-command-detail =
    Dodaje polecenie do menu elementów typu { $on }, { $scope ->
        [everyone] dla wszystkich użytkowników tego komputera
       *[you] tylko dla bieżącego użytkownika
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-default-verb-detail =
    Zmienia akcję wykonywaną przy otwieraniu takich elementów, { $scope ->
        [everyone] dla wszystkich użytkowników tego komputera
       *[you] tylko dla bieżącego użytkownika
    }
core-review-client-entry-detail = Można go wybrać jako program do tego celu. Ustawienia domyślne nie są zmieniane
core-review-com-permissions-title = Uprawnienia COM
core-review-com-permissions-detail = Określa, kto może uruchamiać klasy COM aplikacji lub jako kto one działają
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-link-type-detail =
    Linki tego typu są otwierane w aplikacji, { $scope ->
        [everyone] dla wszystkich użytkowników tego komputera
       *[you] tylko dla bieżącego użytkownika
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-file-type-detail =
    Pliki tego typu są otwierane w aplikacji, { $scope ->
        [everyone] dla wszystkich użytkowników tego komputera
       *[you] tylko dla bieżącego użytkownika
    }
# { $log } is the name of a Windows event log, such as Application.
core-review-event-log-source-detail = Może zapisywać w dzienniku { $log } komunikaty pochodzące z własnych plików
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-font-detail =
    Ładowana z folderu instalacji podczas logowania, { $scope ->
        [everyone] dla wszystkich użytkowników tego komputera
       *[you] tylko dla bieżącego użytkownika
    }
core-review-odbc-driver-detail = Ładowany do programów, które łączą się z bazą danych za pomocą ODBC
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-odbc-source-detail =
    Połączenie z bazą danych, { $scope ->
        [everyone] dla wszystkich użytkowników tego komputera
       *[you] tylko dla bieżącego użytkownika
    }
core-review-default-apps-detail = Można ją wybrać w obszarze Aplikacje domyślne. Ustawienia domyślne nie są zmieniane
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-program-name-detail =
    Okno Uruchamianie i inne programy mogą znaleźć program pod tą nazwą, { $scope ->
        [everyone] dla wszystkich użytkowników tego komputera
       *[you] tylko dla bieżącego użytkownika
    }
# { $server } is the file name of what serves the classes.
core-review-com-title = COM w pliku { $server }
core-review-com-title-own-file = COM w pliku instalowanym przez aplikację
# { $count } is how many registry values.
core-review-com-detail =
    { $count ->
        [one] { $count } ustawienie klas, które mogą uruchamiać inne programy
        [few] { $count } ustawienia klas, które mogą uruchamiać inne programy
        [many] { $count } ustawień klas, które mogą uruchamiać inne programy
       *[other] { $count } ustawień klas, które mogą uruchamiać inne programy
    }
core-review-com-support-title = Biblioteki typów, interfejsy i identyfikatory AppID modelu COM
# { $libraries }, { $interfaces }, and { $app_ids } are how many registry values there are for its type libraries, its
# interfaces, and its AppIDs.
core-review-com-support-detail = Ustawienia bibliotek typów ({ $libraries }), interfejsów ({ $interfaces }) i identyfikatorów AppID ({ $app_ids }), powiązane z klasami aplikacji
core-review-compatibility-title = Sposób uruchamiania programów
# { $count } is how many registry values.
core-review-compatibility-detail =
    { $count ->
        [one] { $count } ustawienie zgodności dla instalowanych programów
        [few] { $count } ustawienia zgodności dla instalowanych programów
        [many] { $count } ustawień zgodności dla instalowanych programów
       *[other] { $count } ustawień zgodności dla instalowanych programów
    }

## The installer's notes on whose folders and settings these are, and what is not coming.

# { $account } is the name of a Windows account, such as alice.
core-review-account-signed-in = Dane użytkownika i ustawienia poszczególnych użytkowników trafią do konta { $account }, które jest tu zalogowane, a nie do konta administratora, na którym działa ten program.
# { $account } is the name of a Windows account, such as alice.
core-review-account-no-desktop = W tej sesji nikt nie jest zalogowany, więc dane użytkownika i ustawienia poszczególnych użytkowników trafią do konta { $account }, na którym działa ten program.
# { $reason } is what Windows said, in its own words.
core-review-account-not-told = Nie można ustalić, kto jest zalogowany ({ $reason }), więc ustawienia poszczególnych użytkowników zostaną pominięte, a dane użytkownika trafią do lokalizacji instalacji.
# { $reason } is what Windows said, in its own words.
core-review-signed-in-unknown = nie można ustalić, kto jest zalogowany: { $reason }
# Headings of what is left out, shared by the app's list and the installer's notes.
core-review-left-out-credentials = Nie ma ich w tym instalatorze, ponieważ wyglądają na poświadczenia
core-review-left-out-settings = Ustawienia i historia, które konto przechowuje dla aplikacji; trafiają do instalatora tylko z opcją Dane użytkownika
core-review-left-out-certificates = Aplikacja prawdopodobnie korzysta z certyfikatów, które były zaufane na jej komputerze, a nie zostaną one zainstalowane
core-review-left-out-fonts = Te czcionki mają nazwę aplikacji, ale nic nie wskazuje, że do niej należą, więc nie zostaną zainstalowane
# { $names } is a list of names, such as the names of saved passwords.
core-review-note-credentials = Nie ma ich w tym instalatorze, ponieważ wyglądają na poświadczenia: { $names }. Ich odpowiedniki na tym komputerze pozostaną bez zmian; być może trzeba będzie wprowadzić je ponownie.
# { $names } is a list of names of license data.
core-review-note-licenses = Nie ma ich w tym instalatorze, ponieważ podczas jego tworzenia opcja Dane licencji i aktywacji była wyłączona: { $names }.
# { $names } is a list of places an account keeps settings, such as folders.
core-review-note-settings = Nie ma ich w tym instalatorze, ponieważ podczas jego tworzenia opcja Dane użytkownika była wyłączona: ustawienia i historia, które konto przechowuje dla aplikacji, w lokalizacjach { $names }. Aplikacja uruchomi się tutaj tak, jak u nowego użytkownika.
# { $names } is a list of certificates, each as core-review-certificate.
core-review-note-certificates = Aplikacja prawdopodobnie korzysta z certyfikatów, które były zaufane na jej komputerze, a nie zostaną one zainstalowane: { $names }. Zdecyduj, czy chcesz im tu zaufać.
# { $names } is a list of font names.
core-review-note-fonts = Te czcionki mają nazwę aplikacji, ale nic nie wskazuje, że do niej należą, więc nie zostaną zainstalowane: { $names }.
# { $names } is a list of names.
core-review-note-left-out-by-choice = Pominięte przez osobę, która utworzyła ten instalator: { $names }.
# { $names } is a list of names.
core-review-note-added-by-choice = Dodane przez osobę, która utworzyła ten instalator, ponieważ uznała je za część aplikacji: { $names }.
# { $name } is a certificate's subject, such as CN=Contoso, and { $store } the certificate store it was in, such as
# Root.
core-review-certificate = { $name } w magazynie { $store }
# { $names } is a list of names, and { $count } how many more there are.
core-review-names-and-more = { $names } i jeszcze { $count }

## Why something in a package is left out. Each is the second half of a sentence of core-warning-review-left-out.

core-review-no-account = należy do ustawień jednego konta, a nie można ustalić, kto jest zalogowany
core-review-name-not-accepted = system Windows nie zaakceptowałby tej nazwy
# { $error } is what Windows said, in its own words.
core-review-service-unreadable = nie można odczytać bieżących ustawień: { $error }
core-review-service-dependency = zależy od usługi o nieprawidłowej nazwie
# { $path } is the program the service runs.
core-review-service-runs-other = uruchamia program { $path }, którego ten pakiet nie instaluje
core-review-service-settings = ustawienia wskazują coś, czego ten pakiet nie instaluje, lub klucz, który nie należy do tej usługi
core-review-driver-no-hash = brak skrótu, za pomocą którego można by go sprawdzić
core-review-license-for-the-machine = licencja przechowywana dla całego komputera jest tylko zgłaszana
core-review-license-starts-something = licencja nie ma powodu niczego uruchamiać
core-review-task-not-captured = nie przechwycono definicji zadania
# { $error } is what was wrong with the task's XML, in English.
core-review-task-unreadable = nie można odczytać definicji zadania: { $error }
core-review-task-no-account = zadanie działa jako osoba, która je instaluje, a nie można ustalić, kto jest tu zalogowany
# { $path } is what the shortcut opens.
core-review-shortcut-opens-other = otwiera element { $path }, którego ten pakiet nie instaluje
# { $service } is the service's name, and { $path } the program it runs now.
core-policy-service-exists = usługa { $service } już istnieje na tym komputerze i uruchamia program { $path }, który nie pochodzi z wcześniejszej instalacji na tym komputerze. Jeśli jest to starsza kopia tej aplikacji, najpierw ją odinstaluj
core-policy-key-control-characters = nazwa klucza zawiera znaki sterujące
core-policy-value-control-characters = nazwa wartości zawiera znaki sterujące
core-policy-registry-link = jest to link rejestru
# { $reason } is why the key it stands for is left out, such as "no app install should change this key".
core-policy-virtual-store = zastępuje klucz komputera, a { $reason }
core-policy-registry-part = żadna instalacja aplikacji nie powinna zapisywać w tej części rejestru
core-policy-key-refused = żadna instalacja aplikacji nie powinna zmieniać tego klucza
core-policy-net-debugger = ustawiłoby to debuger dla każdego programu .NET
core-policy-net-install-root = zmieniłoby to miejsce, z którego każdy program .NET ładuje platformę .NET
core-policy-class-redirects = zmieniłoby to sposób otwierania pewnego rodzaju programów lub folderów
core-policy-app-path-name = wskazuje program za pomocą czegoś innego niż nazwa pliku
core-policy-app-path-not-installed = zarejestrowałoby to nazwę programu dla pliku, którego ten pakiet nie instaluje
core-policy-shared-default-verb = zmieniłoby to akcję otwierania wszystkich plików lub folderów danego rodzaju
core-policy-command-runs-other = dodaje polecenie, które uruchamia coś, czego ten pakiet nie instaluje
core-policy-autostart-not-command = uruchamia się automatycznie, a wartość nie jest poleceniem
core-policy-autostart-runs-other = uruchamia się automatycznie i uruchamia coś, czego ten pakiet nie instaluje
core-policy-environment-own-rules = zmienne środowiskowe są przywracane według własnych reguł
core-policy-service-not-restored = należy do usługi, której ten pakiet nie przywraca
core-policy-service-own-rules = własne ustawienia usługi są przywracane tylko według jej reguł
core-policy-service-dll = wartość ServiceDll nie wskazuje pliku instalowanego przez ten pakiet
core-policy-event-log-source-only = można przywrócić tylko własne wartości źródła dziennika zdarzeń
core-policy-security-log = żadna aplikacja nie zapisuje w dzienniku Zabezpieczenia
core-policy-event-log-messages = komunikaty pochodziłyby z pliku, którego ten pakiet nie instaluje
core-policy-type-library = biblioteka typów nie jest plikiem instalowanym przez ten pakiet
# { $name } is a program's file name, such as app.exe.
core-policy-application-opened = zmieniłoby to sposób otwierania programu { $name }, którego ten pakiet nie instaluje
core-policy-event-log-no-file = źródło dziennika zdarzeń musi pobierać komunikaty z pliku
core-policy-com-runs-other = uruchamia coś, co nie jest plikiem instalowanym przez ten pakiet
core-policy-com-redirect = sprawiłoby to, że ta klasa stałaby się czymś, czego ten pakiet nie instaluje
core-policy-com-server = uruchamia coś, co nie jest ani plikiem instalowanym przez ten pakiet, ani częścią systemu Windows
core-policy-interface = decydowałoby to o sposobie wywoływania interfejsu za pomocą czegoś, czego ten pakiet nie instaluje
# { $name } is a program's file name, such as app.exe.
core-policy-app-id-started = zmieniłoby to sposób uruchamiania programu { $name }, którego ten pakiet nie instaluje
core-policy-app-id-surrogate = byłoby to hostowane przez program, którego ten pakiet nie instaluje
core-policy-app-id-service = działałoby to w usłudze, której ten pakiet nie przywraca
core-policy-app-id-run-as = działałoby to na własnym koncie
core-policy-app-id-link = nie wskazuje identyfikatora AppID
core-policy-app-id-remote = działałoby to na innym komputerze niż ten
core-policy-odbc-own-driver = jest to własne ustawienie ODBC, a nie sterownik
core-policy-odbc-driver-not-installed = sterownik nie jest plikiem instalowanym przez ten pakiet
core-policy-loads-other-file = ładowałoby to plik, którego ten pakiet nie instaluje
core-policy-odbc-own-source = jest to własne ustawienie ODBC, a nie źródło danych
core-policy-odbc-source-missing = wymienia źródło danych, którego ten pakiet nie zawiera
core-policy-odbc-source-driver = sterownik nie jest ani plikiem instalowanym przez ten pakiet, ani częścią systemu Windows
core-policy-shell-extension = ładowałoby to do Eksploratora plików rozszerzenie powłoki, które nie jest plikiem instalowanym przez ten pakiet
core-policy-not-text = zawartość nie jest tekstem, jako który miałaby zostać odczytana
core-policy-network-place = wskazuje lokalizację w sieci
core-policy-font = zarejestrowałoby to czcionkę, której ten pakiet nie instaluje, lub czcionkę z folderu Windows
core-policy-compatibility = zmieniłoby to sposób działania programu, którego ten pakiet nie instaluje
core-policy-capabilities = wymienia aplikację, której szczegółów nie ma w tym pakiecie
core-policy-default-program = zmieniłoby to program, który domyślnie otwiera elementy
core-policy-url-protocol = deklaruje typ linku, ale nie otwiera go nic, co instaluje ten pakiet
core-policy-command-handler = dodaje polecenie obsługiwane przez coś, czego ten pakiet nie instaluje
core-policy-installer-command = uruchamiałoby to coś, co może odczytać tylko Instalator Windows
core-policy-creates-class = utworzyłoby to klasę, której ten pakiet nie instaluje
core-policy-opens-links = otwiera linki za pomocą czegoś, czego ten pakiet nie instaluje
core-policy-opens-files = otwiera pliki za pomocą czegoś, czego ten pakiet nie instaluje
core-policy-firewall-name = nazwy tej reguły nie można przenieść
core-policy-firewall-description = opisu tej reguły nie można przenieść
core-policy-firewall-every-program = dotyczy wszystkich programów, a reguła może dotyczyć tylko jednego
# { $path } is the program the rule is for.
core-policy-firewall-program = dotyczy programu { $path }, którego ten pakiet nie instaluje
core-policy-firewall-direction = nie określa kierunku, który można ustawić
core-policy-firewall-action = zastępowałaby inne reguły, czego nie powinna robić reguła żadnej aplikacji
# { $setting } is a part of a firewall rule, as the scan of its PC described it.
core-policy-firewall-not-carried = zawiera ustawienie „{ $setting }”, którego nie można odtworzyć w takiej postaci lub które osłabiłoby działanie innych reguł
core-policy-firewall-service = dotyczy usługi, której ten pakiet nie przywraca
core-policy-firewall-parts = część reguły wykracza poza to, co może określać reguła zapory
core-policy-folder-already-there = ten folder już istniał poza lokalizacją instalacji i nic nie wskazuje, że należy do tej aplikacji, więc nie trafia do niego żaden program
core-policy-variable-name = nazwa nie jest prawidłową nazwą zmiennej
core-policy-variable-relied-on = system Windows i inne programy zależą od tej zmiennej
core-policy-task-name = system Windows nie akceptuje tej nazwy zadania
core-policy-task-folder = system Windows nie akceptuje folderu tego zadania
core-policy-task-windows = zastąpiłoby to jedno z własnych zadań systemu Windows
core-policy-task-other-action = wykonuje inną czynność niż uruchomienie programu
core-policy-task-no-program = nie wskazuje programu do uruchomienia
# { $path } is the program the task runs.
core-policy-task-runs-other = uruchamia coś, czego ten pakiet nie instaluje: { $path }
core-policy-task-exists-unreadable = zadanie o tej nazwie już istnieje i nie można go odczytać
core-policy-task-exists-other = zadanie o tej nazwie już istnieje i nie uruchamia tej aplikacji
# { $path } is the program the task already on this PC runs.
core-policy-task-exists-runs = zadanie o tej nazwie już istnieje i uruchamia program { $path }, który nie pochodzi z wcześniejszej instalacji na tym komputerze
core-policy-shortcut-not-lnk = nie jest to skrót .lnk
core-policy-shortcut-place = nie znajduje się w menu Start ani na pulpicie
core-policy-short-name = ścieżka zawiera nazwę, której nie można sprawdzić
core-policy-shortcut-no-folder = na tym komputerze nie ma odpowiedniego folderu
core-policy-data-startup = trafiłoby to do folderu Autostart i uruchamiało się przy każdym logowaniu
core-policy-data-powershell = trafiłoby to do miejsca, z którego program PowerShell przy starcie uruchamia znalezione pliki
core-policy-data-windows = trafiłoby to do folderu należącego do systemu Windows
core-policy-license-outside = znajduje się poza lokalizacją instalacji i wszystkimi znanymi folderami danych
core-rewrite-cdata = zawiera sekcję CDATA
core-rewrite-too-much-to-look-through = ustawienia tego pakietu zawierają zbyt wiele danych, aby można było sprawdzić to, co wskazują

## Warnings a review gives, which the installer shows with what it sets up.

# { $kind } says what was left out, and is not shown itself. { $item } is its path or name, or a registry value such
# as HKLM\Software\App\Name. { $reason } is why, as the second half of the sentence, such as "it cannot be a whole
# drive".
core-warning-review-left-out =
    { $kind ->
        [file] Pominięto plik { $item }: { $reason }.
        [folder] Pominięto folder { $item }: { $reason }.
        [service] Pominięto usługę { $item }: { $reason }.
        [driver] Pominięto sterownik { $item }: { $reason }.
        [firewall-rule] Pominięto regułę zapory { $item }: { $reason }.
        [license-data] Pominięto dane licencji w lokalizacji { $item }: { $reason }.
        [license-value] Pominięto wartość licencji { $item }: { $reason }.
        [scheduled-task] Pominięto zaplanowane zadanie { $item }: { $reason }.
        [shortcut] Pominięto skrót { $item }: { $reason }.
        [environment-variable] Pominięto zmienną środowiskową { $item }: { $reason }.
       *[value] Pominięto element { $item }: { $reason }.
    }
core-warning-review-package-path-missing = Plik do skopiowania nie ma ścieżki względnej w pakiecie.
# { $count } is the most empty folders a package makes.
core-warning-review-too-many-empty-folders = Ten pakiet zawiera więcej niż { $count } pustych folderów, a utworzonych zostanie tylko tyle.
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Documents folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-documents-elsewhere =
    { $count ->
        [one] { $count } plik z folderu Dokumenty trafi zamiast tego do folderu _external w lokalizacji { $target }, ponieważ folder Dokumenty na tym komputerze znajduje się w lokalizacji { $folder }, a zapisywane są tylko foldery wewnątrz profilu. Aby go użyć, przenieś go samodzielnie do swojego folderu Dokumenty.
        [few] { $count } pliki z folderu Dokumenty trafią zamiast tego do folderu _external w lokalizacji { $target }, ponieważ folder Dokumenty na tym komputerze znajduje się w lokalizacji { $folder }, a zapisywane są tylko foldery wewnątrz profilu. Aby ich użyć, przenieś je samodzielnie do swojego folderu Dokumenty.
        [many] { $count } plików z folderu Dokumenty trafi zamiast tego do folderu _external w lokalizacji { $target }, ponieważ folder Dokumenty na tym komputerze znajduje się w lokalizacji { $folder }, a zapisywane są tylko foldery wewnątrz profilu. Aby ich użyć, przenieś je samodzielnie do swojego folderu Dokumenty.
       *[other] { $count } plików z folderu Dokumenty trafi zamiast tego do folderu _external w lokalizacji { $target }, ponieważ folder Dokumenty na tym komputerze znajduje się w lokalizacji { $folder }, a zapisywane są tylko foldery wewnątrz profilu. Aby ich użyć, przenieś je samodzielnie do swojego folderu Dokumenty.
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-documents-unknown =
    { $count ->
        [one] { $count } plik z folderu Dokumenty trafi zamiast tego do folderu _external w lokalizacji { $target }, ponieważ nie można ustalić, gdzie na tym komputerze znajduje się folder Dokumenty. Aby go użyć, przenieś go samodzielnie do swojego folderu Dokumenty.
        [few] { $count } pliki z folderu Dokumenty trafią zamiast tego do folderu _external w lokalizacji { $target }, ponieważ nie można ustalić, gdzie na tym komputerze znajduje się folder Dokumenty. Aby ich użyć, przenieś je samodzielnie do swojego folderu Dokumenty.
        [many] { $count } plików z folderu Dokumenty trafi zamiast tego do folderu _external w lokalizacji { $target }, ponieważ nie można ustalić, gdzie na tym komputerze znajduje się folder Dokumenty. Aby ich użyć, przenieś je samodzielnie do swojego folderu Dokumenty.
       *[other] { $count } plików z folderu Dokumenty trafi zamiast tego do folderu _external w lokalizacji { $target }, ponieważ nie można ustalić, gdzie na tym komputerze znajduje się folder Dokumenty. Aby ich użyć, przenieś je samodzielnie do swojego folderu Dokumenty.
    }
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Saved Games folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-elsewhere =
    { $count ->
        [one] { $count } plik z folderu Zapisane gry trafi zamiast tego do folderu _external w lokalizacji { $target }, ponieważ folder Zapisane gry na tym komputerze znajduje się w lokalizacji { $folder }, a zapisywane są tylko foldery wewnątrz profilu. Aby go użyć, przenieś go samodzielnie do swojego folderu Zapisane gry.
        [few] { $count } pliki z folderu Zapisane gry trafią zamiast tego do folderu _external w lokalizacji { $target }, ponieważ folder Zapisane gry na tym komputerze znajduje się w lokalizacji { $folder }, a zapisywane są tylko foldery wewnątrz profilu. Aby ich użyć, przenieś je samodzielnie do swojego folderu Zapisane gry.
        [many] { $count } plików z folderu Zapisane gry trafi zamiast tego do folderu _external w lokalizacji { $target }, ponieważ folder Zapisane gry na tym komputerze znajduje się w lokalizacji { $folder }, a zapisywane są tylko foldery wewnątrz profilu. Aby ich użyć, przenieś je samodzielnie do swojego folderu Zapisane gry.
       *[other] { $count } plików z folderu Zapisane gry trafi zamiast tego do folderu _external w lokalizacji { $target }, ponieważ folder Zapisane gry na tym komputerze znajduje się w lokalizacji { $folder }, a zapisywane są tylko foldery wewnątrz profilu. Aby ich użyć, przenieś je samodzielnie do swojego folderu Zapisane gry.
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-unknown =
    { $count ->
        [one] { $count } plik z folderu Zapisane gry trafi zamiast tego do folderu _external w lokalizacji { $target }, ponieważ nie można ustalić, gdzie na tym komputerze znajduje się folder Zapisane gry. Aby go użyć, przenieś go samodzielnie do swojego folderu Zapisane gry.
        [few] { $count } pliki z folderu Zapisane gry trafią zamiast tego do folderu _external w lokalizacji { $target }, ponieważ nie można ustalić, gdzie na tym komputerze znajduje się folder Zapisane gry. Aby ich użyć, przenieś je samodzielnie do swojego folderu Zapisane gry.
        [many] { $count } plików z folderu Zapisane gry trafi zamiast tego do folderu _external w lokalizacji { $target }, ponieważ nie można ustalić, gdzie na tym komputerze znajduje się folder Zapisane gry. Aby ich użyć, przenieś je samodzielnie do swojego folderu Zapisane gry.
       *[other] { $count } plików z folderu Zapisane gry trafi zamiast tego do folderu _external w lokalizacji { $target }, ponieważ nie można ustalić, gdzie na tym komputerze znajduje się folder Zapisane gry. Aby ich użyć, przenieś je samodzielnie do swojego folderu Zapisane gry.
    }

## Errors when a folder that only administrators may change cannot be made so, or checked.

# { $path } is a folder.
core-privilege-not-a-folder = element { $path } jest linkiem lub plikiem, a nie folderem
# { $path } is a folder. { $error } is what Windows said, in its own words.
core-privilege-not-made-here = folder { $path } nie został utworzony przez program WinAppMigrator, a inni mogą zmieniać jego zawartość. Administrator powinien go sprawdzić i usunąć: { $error }
# { $path } is a folder.
core-privilege-others-put-something-in = folder { $path } zawiera coś, co nie zostało utworzone przez program WinAppMigrator, a inni mogą to zmieniać. Administrator powinien go sprawdzić i usunąć
# { $path } is a folder.
core-privilege-became-a-link-in-use = folder { $path } stał się linkiem podczas używania
# { $path } is a folder.
core-privilege-became-a-link-looked-at = folder { $path } stał się linkiem podczas sprawdzania
# { $path } is a file. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-users-cannot-start = nie można zezwolić użytkownikom na uruchamianie pliku { $path }: { $said }
core-privilege-elevation-unknown = nie można ustalić, czy ten proces ma podwyższone uprawnienia
# { $path } is a folder. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-cannot-restrict = nie można ograniczyć dostępu do folderu { $path } tylko do administratorów: { $said }
# { $path } is a folder. { $said } is the folder's permissions as Windows wrote them, and what else it printed.
core-privilege-not-restricted = nie można ograniczyć dostępu do folderu { $path } tylko do administratorów, a jego dysk może nie obsługiwać uprawnień: { $said }
# { $path } is a folder.
core-privilege-written-to-first = inny proces zapisał dane w folderze { $path }, zanim można było go zabezpieczyć
core-privilege-no-program-data = system Windows nie podaje lokalizacji folderu ProgramData

## Checks of a package's files, and of what an install put in place. Each is said of one file, value, service, or the
## like, which the report names beside it.

# { $path } is the installer's file.
core-verify-no-package = plik { $path } nie zawiera pakietu
# { $path } is a folder.
core-verify-not-unpacked = pakiet tego instalatora nie został rozpakowany do folderu { $path }
core-verify-schema-supported = Schemat manifestu pakietu jest obsługiwany.
core-verify-packaged-file-missing = Brak spakowanego pliku.
core-verify-no-package-path = Plik do skopiowania nie ma ścieżki względnej w pakiecie.
core-verify-folder-there = Pusty folder istnieje.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-unreadable = Nie można było odczytać elementu w celu sprawdzenia: { $error }
core-verify-restored-file-missing = Brak przywróconego pliku.
# { $expected } and { $actual } are sizes in bytes.
core-verify-size-mismatch = Niezgodność rozmiaru pliku. Oczekiwany rozmiar (w bajtach): { $expected }, znaleziony: { $actual }.
core-verify-no-hash = Manifest nie zawiera skrótu tego pliku, więc nie można go sprawdzić.
core-verify-hash-mismatch = Niezgodność skrótu SHA-256.
core-verify-file-matches = Rozmiar i skrót pliku są zgodne z manifestem.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-payload-failed = Weryfikacja danych licencji nie powiodła się: { $error }
core-verify-license-payload-missing = Brak spakowanych danych licencji.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-failed = Weryfikacja magazynu licencji nie powiodła się: { $error }
core-verify-license-store-file-missing = Brak spakowanego pliku magazynu licencji.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-path-invalid = Ścieżka spakowanego magazynu licencji jest nieprawidłowa: { $error }
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-prerequisite-failed = Weryfikacja danych wymagania wstępnego nie powiodła się: { $error }
core-verify-prerequisite-missing = Brak dołączonych danych wymagania wstępnego.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-failed = Weryfikacja obrazu sterownika nie powiodła się: { $error }
core-verify-bundled-driver-missing = Brak dołączonego obrazu sterownika.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-path-invalid = Ścieżka dołączonego sterownika jest nieprawidłowa: { $error }
core-verify-prerequisite-present = Wymagane środowisko uruchomieniowe jest obecne na komputerze docelowym.
core-verify-prerequisite-not-detected = Nie wykryto wymaganego środowiska uruchomieniowego; może być wymagane ponowne uruchomienie lub ręczne dokończenie instalacji (rozstrzygający wynik znajduje się w dzienniku przywracania).
core-verify-prerequisite-not-probed = Obecności wymagania wstępnego nie można sprawdzić w rejestrze; instalacja została przeprowadzona w miarę możliwości.
core-verify-license-file-missing = Brak przywróconego pliku licencji.
core-verify-license-store-restored-missing = Brak przywróconego pliku magazynu licencji.
core-verify-registry-key-there = Klucz rejestru istnieje.
core-verify-registry-value-matches = Wartość rejestru jest zgodna z zarchiwizowanymi danymi.
core-verify-registry-value-differs = Wartość rejestru nie jest zgodna z zarchiwizowanymi danymi.
core-verify-firewall-rule-there = Zapora systemu Windows zawiera tę regułę.
core-verify-variable-matches = Wartość zmiennej środowiskowej jest zgodna z zarchiwizowanymi danymi.
core-verify-variable-differs = Wartość zmiennej środowiskowej nie jest zgodna z zarchiwizowanymi danymi.
# { $error } is what Windows said, in its own words.
core-verify-variable-unreadable = Brak zmiennej środowiskowej lub nie można jej odczytać: { $error }
core-verify-service-runs-other = Usługa istnieje, ale nie uruchamia przywróconego pliku binarnego z jego argumentami.
core-verify-service-other-account = Usługa istnieje, ale nie działa na oczekiwanym koncie.
core-verify-service-matches = Usługa istnieje i uruchamia przywrócony plik binarny z jego argumentami na oczekiwanym koncie.
core-verify-service-missing = Brak usługi.
# { $error } is what Windows said, in its own words.
core-verify-service-unreadable = Nie można odczytać usługi: { $error }
core-verify-driver-other-image = Usługa sterownika nie wskazuje przywróconego obrazu.
core-verify-driver-not-registered = Dołączona usługa sterownika nie jest zarejestrowana po przywróceniu.
# { $error } is what Windows said, in its own words.
core-verify-driver-unreadable = Nie można odczytać usługi sterownika: { $error }
core-verify-driver-image-missing = Brak obrazu sterownika.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-unchecked = Nie można sprawdzić obrazu sterownika: { $error }
core-verify-task-matches = Zaplanowane zadanie istnieje i uruchamia przywrócone programy.
core-verify-task-runs-other = Zaplanowane zadanie istnieje, ale nie uruchamia przywróconych programów.
core-verify-task-missing = Brak zaplanowanego zadania.
# { $error } is what Windows said, in its own words.
core-verify-task-unreadable = Zapytanie o zaplanowane zadanie nie powiodło się: { $error }
core-verify-shortcut-matches = Skrót istnieje, a jego ścieżka docelowa jest zgodna z przywróconym elementem docelowym.
core-verify-shortcut-opens-other = Skrót istnieje, ale jego ścieżka docelowa nie jest zgodna z przywróconym elementem docelowym.
core-verify-shortcut-missing = Brak skrótu.
# { $error } is what Windows said, in its own words.
core-verify-shortcut-unreadable = Zapytanie o skrót nie powiodło się: { $error }

## Warnings and errors a check gives, which the app and the installer show.

core-warning-verify-schema-unsupported = Schemat manifestu pakietu nie jest obsługiwany.
# { $error } is what went wrong, in Windows' or the program's own words.
core-warning-verify-package-file-failed = Weryfikacja pliku pakietu nie powiodła się: { $error }
core-warning-verify-folder-missing = Brak pustego folderu.
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-key-missing = Brak klucza rejestru lub nie można go odczytać: { $error }
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-value-missing = Brak wartości rejestru lub nie można jej odczytać: { $error }
core-warning-verify-registry-kind-missing = Brak typu wartości rejestru.
core-warning-verify-firewall-rule-missing = Zapora systemu Windows nie zawiera tej reguły.
core-warning-verify-service-scope = Weryfikacja usług odczytuje konfigurację usług z klucza HKLM i nie uruchamia usług.
core-warning-verify-driver-manual = Program WinAppMigrator nie rozpowszechnia co najmniej jednego sterownika dostawcy lub wbudowanego w system; zainstaluj go ponownie na komputerze docelowym z podpisanego pakietu sterowników dostawcy.
core-warning-verify-task-scope = Weryfikacja zaplanowanych zadań sprawdza tylko rejestrację i odwołanie do polecenia.

## The verification report written beside a package, verification_report.md. Its headings follow a "#".

core-verify-report-title = Raport weryfikacji WinAppMigrator
# { $time } is when, as a date and time in UTC, such as 2026-11-15T10:00:00Z.
core-verify-report-verified = Zweryfikowano (UTC): { $time }
# { $valid } is true or false, which are not translated.
core-verify-report-package-valid = Pakiet prawidłowy: { $valid }
# { $valid } is true or false, which are not translated.
core-verify-report-restore-valid = Przywracanie prawidłowe: { $valid }
core-verify-report-restore-not-run = Przywracanie prawidłowe: nie uruchomiono
core-verify-report-items = Elementy
# { $message } is what a check found, and { $source } the file or setting it looked at.
core-verify-report-item-from = { $message } Źródło: { $source }
core-verify-report-warnings = Ostrzeżenia
core-verify-report-errors = Błędy
core-verify-report-none = Brak.
