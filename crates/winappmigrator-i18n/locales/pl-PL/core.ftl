# Words the core writes for people: warnings, refusals, errors, and the installer's list of what it sets up.
# Message names start with core-.


## Warnings a package records when it is built, which an installer shows later, maybe on another PC, in that PC's
## language.

# { $count } is how many shortcuts.
core-warning-shortcuts-unreadable =
    { $count ->
        [one] { $count } skrót na tym dysku nie wskazuje żadnego pliku, więc nie można ustalić, czy otwiera tę aplikację.
        [few] { $count } skróty na tym dysku nie wskazują żadnego pliku, więc nie można ustalić, czy otwierają tę aplikację.
        [many] { $count } skrótów na tym dysku nie wskazuje żadnego pliku, więc nie można ustalić, czy otwierają tę aplikację.
       *[other] { $count } skrótów na tym dysku nie wskazuje żadnego pliku, więc nie można ustalić, czy otwierają tę aplikację.
    }

## Undoing an install: what an uninstall, or a failed install, says it put back, left, or could not undo. Each line is
## shown on its own, in a list.

# { $app_name } is the app's name.
core-rollback-record-damaged = Nie można odczytać części zapisu aplikacji { $app_name }, więc wszystko, co ta część opisywała, nadal pozostaje na miejscu, a zastąpiony plik może nie wrócić na miejsce.
core-rollback-settings-entry-stays = Wpis aplikacji w Ustawieniach pozostaje, aby można było z niego ponownie uruchomić dezinstalację.
# { $name } is a runtime's name, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-rollback-prerequisite-stays = Środowisko uruchomieniowe { $name } zostało zainstalowane (lub mogło zostać zainstalowane) i pozostaje, ponieważ mogą z niego korzystać inne programy.
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-drive-not-connected = Elementy umieszczone przez tę instalację w folderze { $folder } znajdują się na dysku, który nie jest podłączony. Podłącz go, aby można było dokończyć tę operację.
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-record-part-unread = Elementy umieszczone przez tę instalację w folderze { $folder } pozostają, ponieważ nie można odczytać tej części zapisu.

## Why a file or folder was left, shown after "was left:" in the messages below them. Each is part of a sentence, so
## starts with a small letter and has no full stop.

# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-being-changed = coś zmieniało folder { $folder } w trakcie tej operacji, więc nic więcej w nim nie zrobiono. Zamknij program, który to robi, i uruchom tę operację ponownie
# { $folder } is a folder; { $error } is what Windows said went wrong.
core-rollback-reason-not-opened = nie można otworzyć folderu { $folder }: { $error }
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-no-record = brak zapisu o tym, gdzie znajdował się folder { $folder }
# { $folder } is a folder; { $actual } is the folder it now leads to.
core-rollback-reason-moved = folder { $folder } prowadzi teraz do lokalizacji { $actual }, a nie tam, gdzie prowadził podczas instalacji
# Why a service was not stopped.
core-rollback-reason-did-not-stop = zatrzymywanie trwało zbyt długo

## Files and folders

# { $path } is a folder; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-folder-left = Folder { $path } został pozostawiony: { $reason }.
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-removed = Usunięto folder { $path }
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-holds-others = Folder { $path } pozostaje, ponieważ zawiera pliki, których ta instalacja w nim nie umieściła.
# { $path } is a folder; { $error } is what Windows said went wrong.
core-rollback-folder-not-removed = Nie można usunąć folderu { $path }: { $error }.
# { $path } is a file; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-file-left = Plik { $path } został pozostawiony: { $reason }.
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-folder-gone = Plik { $path } nie zostanie przywrócony, ponieważ folder, w którym się znajdował, już nie istnieje.
# { $path } is a file; { $error } is what went wrong.
core-rollback-partial-not-removed = Nie można usunąć częściowo zapisanego pliku: { $path } ({ $error }).
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-checked = Nie można sprawdzić pliku { $path }: { $error }.
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-removed = Usunięto plik { $path }
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-removed = Nie można usunąć pliku { $path }: { $error }.
# { $path } is a file this install had replaced, and has put back.
core-rollback-file-put-back = Przywrócono plik { $path }
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-put-back = Nie można przywrócić pliku { $path }: { $error }.
# Why a file could not be put back.
core-rollback-backup-gone = kopia zapasowa już nie istnieje
# { $path } is a file.
core-rollback-no-parent-folder = plik { $path } nie ma folderu nadrzędnego

## A file that stays. { $why } says why: still-used (something that stays still uses it), changed (it has changed since
## it was installed), or not-asked (it is someone's data, and they did not ask for it to go). { $path } is the file.

core-rollback-file-stays =
    { $why ->
        [still-used] Plik { $path } pozostaje, ponieważ nadal korzysta z niego coś, co pozostaje.
        [changed] Plik { $path } pozostaje, ponieważ zmienił się od czasu instalacji.
       *[not-asked] Plik { $path } pozostaje, ponieważ nie zażądano jego usunięcia.
    }
# The file it replaced was to be put beside it, under another name, and every name was taken.
core-rollback-file-stays-no-name-beside =
    { $why ->
        [still-used] Plik { $path } pozostaje, ponieważ nadal korzysta z niego coś, co pozostaje, a zastąpionego przez niego pliku nie można umieścić obok: wszystkie możliwe nazwy są zajęte.
        [changed] Plik { $path } pozostaje, ponieważ zmienił się od czasu instalacji, a zastąpionego przez niego pliku nie można umieścić obok: wszystkie możliwe nazwy są zajęte.
       *[not-asked] Plik { $path } pozostaje, ponieważ nie zażądano jego usunięcia, a zastąpionego przez niego pliku nie można umieścić obok: wszystkie możliwe nazwy są zajęte.
    }
# { $name } is the name the file it replaced now has, such as settings.ini.before-install.
core-rollback-file-stays-beside =
    { $why ->
        [still-used] Plik { $path } pozostaje, ponieważ nadal korzysta z niego coś, co pozostaje. Zastąpiony przez niego plik znajduje się obok jako { $name }.
        [changed] Plik { $path } pozostaje, ponieważ zmienił się od czasu instalacji. Zastąpiony przez niego plik znajduje się obok jako { $name }.
       *[not-asked] Plik { $path } pozostaje, ponieważ nie zażądano jego usunięcia. Zastąpiony przez niego plik znajduje się obok jako { $name }.
    }
# { $error } is what went wrong.
core-rollback-file-stays-not-beside =
    { $why ->
        [still-used] Plik { $path } pozostaje, ponieważ nadal korzysta z niego coś, co pozostaje, a zastąpionego przez niego pliku nie można umieścić obok: { $error }.
        [changed] Plik { $path } pozostaje, ponieważ zmienił się od czasu instalacji, a zastąpionego przez niego pliku nie można umieścić obok: { $error }.
       *[not-asked] Plik { $path } pozostaje, ponieważ nie zażądano jego usunięcia, a zastąpionego przez niego pliku nie można umieścić obok: { $error }.
    }

## The registry and environment variables. { $kind } says what { $name } is: key (a registry key, such as
## HKLM\Software\App), value (a registry value, such as HKLM\Software\App\Version), user-variable or machine-variable
## (an environment variable of the user's or of the whole PC, such as APP_HOME), or user-list or machine-list (a
## variable that is a list of folders, such as Path). { $error } is what Windows said went wrong.

# How a registry key's unnamed value is shown, as Registry Editor shows it.
core-registry-default-value-name = (Domyślna)
core-rollback-setting-other-account =
    { $kind ->
        [key] Klucz { $name } pozostaje, ponieważ należy do konta, z którego przeprowadzono tę instalację, a to konto nie jest zalogowane.
        [user-variable] Zmienna środowiskowa użytkownika { $name } pozostaje, ponieważ należy do konta, z którego przeprowadzono tę instalację, a to konto nie jest zalogowane.
        [machine-variable] Systemowa zmienna środowiskowa { $name } pozostaje, ponieważ należy do konta, z którego przeprowadzono tę instalację, a to konto nie jest zalogowane.
        [user-list] Zmienna { $name } użytkownika pozostaje, ponieważ należy do konta, z którego przeprowadzono tę instalację, a to konto nie jest zalogowane.
        [machine-list] Systemowa zmienna { $name } pozostaje, ponieważ należy do konta, z którego przeprowadzono tę instalację, a to konto nie jest zalogowane.
       *[value] Wartość rejestru { $name } pozostaje, ponieważ należy do konta, z którego przeprowadzono tę instalację, a to konto nie jest zalogowane.
    }
core-rollback-setting-not-reached =
    { $kind ->
        [key] Nie można uzyskać dostępu do klucza { $name }: { $error }.
        [user-variable] Nie można uzyskać dostępu do zmiennej środowiskowej użytkownika { $name }: { $error }.
        [machine-variable] Nie można uzyskać dostępu do systemowej zmiennej środowiskowej { $name }: { $error }.
        [user-list] Nie można uzyskać dostępu do zmiennej { $name } użytkownika: { $error }.
        [machine-list] Nie można uzyskać dostępu do systemowej zmiennej { $name }: { $error }.
       *[value] Nie można uzyskać dostępu do wartości rejestru { $name }: { $error }.
    }
core-rollback-setting-not-read =
    { $kind ->
        [key] Nie można odczytać klucza { $name }: { $error }.
        [user-variable] Nie można odczytać zmiennej środowiskowej użytkownika { $name }: { $error }.
        [machine-variable] Nie można odczytać systemowej zmiennej środowiskowej { $name }: { $error }.
        [user-list] Nie można odczytać zmiennej { $name } użytkownika: { $error }.
        [machine-list] Nie można odczytać systemowej zmiennej { $name }: { $error }.
       *[value] Nie można odczytać wartości rejestru { $name }: { $error }.
    }
core-rollback-setting-changed =
    { $kind ->
        [user-variable] Zmienna środowiskowa użytkownika { $name } pozostaje, ponieważ zmieniła się od czasu instalacji.
        [machine-variable] Systemowa zmienna środowiskowa { $name } pozostaje, ponieważ zmieniła się od czasu instalacji.
        [user-list] Zmienna { $name } użytkownika pozostaje, ponieważ zmieniła się od czasu instalacji.
        [machine-list] Systemowa zmienna { $name } pozostaje, ponieważ zmieniła się od czasu instalacji.
       *[value] Wartość rejestru { $name } pozostaje, ponieważ zmieniła się od czasu instalacji.
    }
# Said when it is back as it was before the install.
core-rollback-setting-put-back =
    { $kind ->
        [user-variable] Przywrócono zmienną środowiskową użytkownika { $name }
        [machine-variable] Przywrócono systemową zmienną środowiskową { $name }
        [user-list] Przywrócono zmienną { $name } użytkownika
        [machine-list] Przywrócono systemową zmienną { $name }
       *[value] Przywrócono wartość rejestru { $name }
    }
core-rollback-setting-not-put-back =
    { $kind ->
        [user-variable] Nie można przywrócić zmiennej środowiskowej użytkownika { $name }: { $error }.
        [machine-variable] Nie można przywrócić systemowej zmiennej środowiskowej { $name }: { $error }.
        [user-list] Nie można przywrócić zmiennej { $name } użytkownika: { $error }.
        [machine-list] Nie można przywrócić systemowej zmiennej { $name }: { $error }.
       *[value] Nie można przywrócić wartości rejestru { $name }: { $error }.
    }
# A registry value the install wrote, which is the user's data, and stays unless they asked for it to go.
core-rollback-value-kept-by-request = Wartość rejestru { $name } pozostaje, ponieważ nie zażądano jej usunięcia.
core-rollback-key-holds-others = Klucz { $name } pozostaje, ponieważ zawiera elementy, których ta instalacja w nim nie umieściła.
core-rollback-key-removed = Usunięto klucz { $name }
core-rollback-key-not-removed = Nie można usunąć klucza { $name }: { $error }.
# { $kind } is user-list or machine-list.
core-rollback-list-entries-removed =
    { $kind ->
        [user-list] Usunięto wpisy tej instalacji ze zmiennej { $name } użytkownika
       *[machine-list] Usunięto wpisy tej instalacji z systemowej zmiennej { $name }
    }
core-rollback-list-entries-not-removed =
    { $kind ->
        [user-list] Nie można usunąć wpisów tej instalacji ze zmiennej { $name } użytkownika: { $error }.
       *[machine-list] Nie można usunąć wpisów tej instalacji z systemowej zmiennej { $name }: { $error }.
    }

## Services and drivers. { $kind } is service or driver; { $name } is its name, such as AppUpdater. { $error } and
## { $reason } are what Windows said went wrong.

core-rollback-service-not-read =
    { $kind ->
        [driver] Nie można odczytać sterownika { $name }: { $error }.
       *[service] Nie można odczytać usługi { $name }: { $error }.
    }
core-rollback-service-changed =
    { $kind ->
        [driver] Sterownik { $name } pozostaje, ponieważ zmienił się od czasu instalacji.
       *[service] Usługa { $name } pozostaje, ponieważ zmieniła się od czasu instalacji.
    }
core-rollback-service-not-stopped =
    { $kind ->
        [driver] Nie można zatrzymać sterownika { $name } ({ $reason }), więc używane przez niego elementy mogą pozostać do ponownego uruchomienia komputera.
       *[service] Nie można zatrzymać usługi { $name } ({ $reason }), więc używane przez nią elementy mogą pozostać do ponownego uruchomienia komputera.
    }
core-rollback-service-stopped-for-this =
    { $kind ->
        [driver] Sterownik { $name } został zatrzymany na potrzeby tej operacji. Uruchom go ponownie, gdy będzie potrzebny.
       *[service] Usługa { $name } została zatrzymana na potrzeby tej operacji. Uruchom ją ponownie, gdy będzie potrzebna.
    }
core-rollback-service-not-removed =
    { $kind ->
        [driver] Nie można usunąć sterownika { $name }: { $reason }.
       *[service] Nie można usunąć usługi { $name }: { $reason }.
    }
core-rollback-service-marked-for-removal =
    { $kind ->
        [driver] Sterownik { $name } został oznaczony do usunięcia i zostanie usunięty po ponownym uruchomieniu komputera lub gdy zwolni go program, który go używa.
       *[service] Usługa { $name } została oznaczona do usunięcia i zostanie usunięta po ponownym uruchomieniu komputera lub gdy zwolni ją program, który jej używa.
    }
core-rollback-service-removed =
    { $kind ->
        [driver] Usunięto sterownik { $name }
       *[service] Usunięto usługę { $name }
    }
core-rollback-service-not-restored =
    { $kind ->
        [driver] Nie można przywrócić poprzednich ustawień sterownika { $name }: { $reason }.
       *[service] Nie można przywrócić poprzednich ustawień usługi { $name }: { $reason }.
    }
core-rollback-service-restored =
    { $kind ->
        [driver] Przywrócono ustawienia sterownika { $name }
       *[service] Przywrócono ustawienia usługi { $name }
    }
core-rollback-service-not-restarted = Usługa { $name } działała przed tą instalacją, ale nie uruchomiła się ponownie: { $reason }.
# { $account } is the account the service ran as, such as CONTOSO\backup.
core-rollback-service-account-again = Usługa { $name } działała jako { $account }. Jej hasło nie jest przechowywane, więc ponownie ustaw jej konto w przystawce Usługi.

## Firewall rules. { $name } is the rule's name; { $error } is what went wrong.

core-rollback-firewall-not-read = Nie można odczytać reguły zapory { $name }: { $error }.
core-rollback-firewall-says-otherwise = Reguła zapory { $name } pozostaje, ponieważ istniejąca reguła o tej nazwie ma inne ustawienia.
core-rollback-firewall-changed = Reguła zapory { $name } pozostaje, ponieważ zmieniła się od czasu instalacji.
# The capital "The" is as the app has always said it.
core-rollback-firewall-removed = Usunięto regułę zapory { $name }
core-rollback-firewall-not-removed = Nie można usunąć reguły zapory { $name }: { $error }.

## Scheduled tasks. { $path } is a folder of tasks, and { $name } a task with its folder, such as \App\Nightly Update.
## { $error } and { $reason } are what went wrong.

core-rollback-task-folder-holds-others = Folder zadań { $path } pozostaje, ponieważ zawiera zadania, których ta instalacja w nim nie umieściła.
core-rollback-task-folder-not-removed = Nie można usunąć folderu zadań { $path }: { $error }.
core-rollback-task-not-read = Nie można odczytać zadania { $name }: { $error }.
core-rollback-task-changed = Zadanie { $name } pozostaje, ponieważ zmieniło się od czasu instalacji.
core-rollback-task-removed = Usunięto zadanie { $name }
core-rollback-task-not-removed = Nie można usunąć zadania { $name }: { $reason }.
core-rollback-task-put-back = Przywrócono zadanie { $name }
core-rollback-task-not-put-back = Nie można przywrócić zadania { $name }: { $reason }.

## When an install cannot be undone at all

# { $error } is what went wrong.
core-rollback-record-unreadable = Nie można odczytać zapisu tej instalacji, więc niczego nie cofnięto: { $error }.
# { $error } is what went wrong.
core-rollback-nothing-undone = Niczego nie cofnięto: { $error }.
# One app's part of the message below. { $app_name } is the app's name; { $failures } is what could not be undone, a
# sentence or more; { $folder } is the folder that holds the install's record.
core-rollback-earlier-install-stuck = { $app_name }: { $failures } Jeśli tej instalacji nie da się cofnąć, administrator może z niej zrezygnować, usuwając folder { $folder }.
# Why an install did not start. { $details } is one or more of the message above.
core-rollback-earlier-installs-not-undone = nie można było w pełni cofnąć wcześniejszej, nieukończonej instalacji, więc niczego nie zainstalowano. Zamknij wszystko, co korzysta z tych elementów, i spróbuj ponownie. { $details }
core-rollback-install-busy = ta instalacja jest właśnie zmieniana przez inny instalator lub dezinstalator

## The summary an uninstall writes on the command line, one line each, with what each says below it. { $count } is how
## many.

core-rollback-count-undone = Cofnięte: { $count }
core-rollback-count-failed = Nie można cofnąć: { $count }
# What was left on purpose: it has changed since the install, or it is the user's.
core-rollback-count-kept = Zachowane: { $count }
# What no uninstall can take back, such as a runtime that was installed, which other programs may use.
core-rollback-count-left-behind = Pozostawione: { $count }

## Which Windows a PC or drive has, as the app shows it beside the drive.

# { $name } is the Windows' name, such as Windows 7 Professional Service Pack 1.
core-system-windows-32-bit = { $name }, 32-bitowy
# { $name } is the Windows' name, such as Windows 11 Pro.
core-system-windows-64-bit = { $name }, 64-bitowy
# A drive's Windows, when it only says it is older than Vista.
core-system-windows-xp-or-older = Windows XP lub starszy

## Reading a connected drive, or another account's settings. Each is why it could not be done, and some start with a
## small letter, as they follow other words.

core-system-needs-administrator = odczyt podłączonego dysku lub ustawień innego konta wymaga uprawnień administratora
core-system-no-program-data = nie można odnaleźć folderu ProgramData na tym komputerze
# { $path } is a file, such as D:\Windows\System32\config\SOFTWARE.
core-system-not-a-hive-file = to nie jest plik gałęzi rejestru: { $path }
# { $path } is a file; { $size } is its size in bytes.
core-system-hive-too-large = plik { $path } ma rozmiar { $size } B, za duży dla gałęzi rejestru lub większy niż łączny możliwy rozmiar gałęzi tego dysku
# { $key } is a registry key, such as Software\Microsoft\Windows\CurrentVersion.
core-system-hive-no-mapping = gałąź offline nie ma mapowania dla klucza HKLM\{ $key }
# { $key } is where a drive's registry was to be loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said.
# "reg load" is a command's name.
core-system-hive-not-loaded = polecenie reg load { $key } nie powiodło się: { $error }
# { $key } is where a drive's registry was loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said. "reg
# unload" is a command's name.
core-system-hive-not-unloaded = polecenie reg unload { $key } nie powiodło się: { $error }
# { $drive } is a drive, such as D:\.
core-system-not-a-system-drive = Dysk { $drive } nie jest dyskiem systemowym Windows (brak folderu \Windows).
# { $drive } is a drive, such as C:\. "This PC" is the name of a choice in the app.
core-system-own-system-drive = Dysk { $drive } jest dyskiem systemowym tego komputera. Aby odczytać ten komputer, wybierz opcję Ten komputer.
# { $drive } is a drive, such as D:\.
core-system-registry-through-link = Dysk { $drive } nie jest dyskiem systemowym Windows: droga do jego rejestru prowadzi przez link.

## Warnings a scan records about whose data it read.

# { $account } is an account's name or id, such as alice or S-1-5-21-...-1001.
core-warning-scan-account-not-on-record = Na tym komputerze nie ma informacji o koncie { $account }, więc nie dołączono niczyich danych ani ustawień poszczególnych użytkowników.
# { $account } is an account's name, such as CONTOSO\alice.
core-warning-scan-signed-in-account-no-profile = Konto zalogowane tutaj, { $account }, nie ma zapisanego profilu, więc nie dołączono niczyich danych ani ustawień poszczególnych użytkowników.
# { $reason } is why Windows could not say.
core-warning-scan-signed-in-account-unknown = Nie można ustalić, kto jest zalogowany ({ $reason }), więc nie dołączono niczyich danych ani ustawień poszczególnych użytkowników. Wybierz, czyje dane dołączyć.
core-warning-scan-last-account-unknown = Ten dysk nie zawiera informacji o tym, kto ostatnio się logował, a ma więcej niż jedno konto, więc nie dołączono niczyich danych ani ustawień poszczególnych użytkowników. Wybierz, czyje dane dołączyć.
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-settings-unreadable = Nie można odczytać ustawień konta { $account }, więc nie zostały one dołączone: { $error }
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-classes-unreadable = Nie można odczytać klas COM i typów plików zarejestrowanych dla konta { $account }, więc nie zostały one dołączone: { $error }

## Paths a scan did not follow. { $count } is how many; { $paths } is some of them, separated by semicolons; { $more } is
## how many more there are, not shown.

core-warning-scan-drive-paths-not-followed =
    { $count ->
        [one] Nie można odczytać ścieżki zapisanej na tym dysku, więc nic z tej lokalizacji nie zostało dołączone. Taka ścieżka znajduje się na innym dysku, nie jest pełną ścieżką lub prowadzi przez link, który wyprowadziłby poza ten dysk: { $paths }
        [few] Nie można odczytać niektórych ścieżek zapisanych na tym dysku, więc nic z tych lokalizacji nie zostało dołączone. Każda z nich znajduje się na innym dysku, nie jest pełną ścieżką lub prowadzi przez link, który wyprowadziłby poza ten dysk: { $paths }
        [many] Nie można odczytać niektórych ścieżek zapisanych na tym dysku, więc nic z tych lokalizacji nie zostało dołączone. Każda z nich znajduje się na innym dysku, nie jest pełną ścieżką lub prowadzi przez link, który wyprowadziłby poza ten dysk: { $paths }
       *[other] Nie można odczytać niektórych ścieżek zapisanych na tym dysku, więc nic z tych lokalizacji nie zostało dołączone. Każda z nich znajduje się na innym dysku, nie jest pełną ścieżką lub prowadzi przez link, który wyprowadziłby poza ten dysk: { $paths }
    }
core-warning-scan-drive-paths-not-followed-more = Nie można odczytać niektórych ścieżek zapisanych na tym dysku, więc nic z tych lokalizacji nie zostało dołączone. Każda z nich znajduje się na innym dysku, nie jest pełną ścieżką lub prowadzi przez link, który wyprowadziłby poza ten dysk: { $paths } i jeszcze { $more }
core-warning-scan-network-paths-not-followed =
    { $count ->
        [one] Nie można odczytać ścieżki zapisanej na tym komputerze, więc nic z tej lokalizacji nie zostało dołączone. Taka ścieżka nie znajduje się na dysku tego komputera (sprawdzenie jej spowodowałoby nawiązanie połączenia) lub ma nazwę, która otworzyłaby coś innego: { $paths }
        [few] Nie można odczytać niektórych ścieżek zapisanych na tym komputerze, więc nic z tych lokalizacji nie zostało dołączone. Każda z nich nie znajduje się na dysku tego komputera (sprawdzenie jej spowodowałoby nawiązanie połączenia) lub ma nazwę, która otworzyłaby coś innego: { $paths }
        [many] Nie można odczytać niektórych ścieżek zapisanych na tym komputerze, więc nic z tych lokalizacji nie zostało dołączone. Każda z nich nie znajduje się na dysku tego komputera (sprawdzenie jej spowodowałoby nawiązanie połączenia) lub ma nazwę, która otworzyłaby coś innego: { $paths }
       *[other] Nie można odczytać niektórych ścieżek zapisanych na tym komputerze, więc nic z tych lokalizacji nie zostało dołączone. Każda z nich nie znajduje się na dysku tego komputera (sprawdzenie jej spowodowałoby nawiązanie połączenia) lub ma nazwę, która otworzyłaby coś innego: { $paths }
    }
core-warning-scan-network-paths-not-followed-more = Nie można odczytać niektórych ścieżek zapisanych na tym komputerze, więc nic z tych lokalizacji nie zostało dołączone. Każda z nich nie znajduje się na dysku tego komputera (sprawdzenie jej spowodowałoby nawiązanie połączenia) lub ma nazwę, która otworzyłaby coś innego: { $paths } i jeszcze { $more }

## What a scan with none of an account's data leaves out of a package, in the app's list of what was left out. Each
## has a name, and below it why.

core-account-left-out-shortcuts = Skróty w folderach tego konta
# { $count } is how many shortcuts.
core-account-left-out-shortcuts-why =
    { $count ->
        [one] { $count } skrót, dołączany tylko z opcją Dane użytkownika
        [few] { $count } skróty, dołączane tylko z opcją Dane użytkownika
        [many] { $count } skrótów, dołączanych tylko z opcją Dane użytkownika
       *[other] { $count } skrótów, dołączanych tylko z opcją Dane użytkownika
    }
core-account-left-out-variables = Zmienne środowiskowe tego konta
# { $count } is how many variables.
core-account-left-out-variables-why =
    { $count ->
        [one] { $count } zmienna, dołączana tylko z opcją Dane użytkownika
        [few] { $count } zmienne, dołączane tylko z opcją Dane użytkownika
        [many] { $count } zmiennych, dołączanych tylko z opcją Dane użytkownika
       *[other] { $count } zmiennych, dołączanych tylko z opcją Dane użytkownika
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-account-left-out-task = Zaplanowane zadanie { $task }
core-account-task-works-on-account-folder = Operuje na lokalizacji w folderach konta lub jego nazwa się do niej odnosi; jest dołączane tylko z opcją Dane użytkownika
core-account-task-runs-from-account-folder = To, co dodatkowo uruchamia z folderów konta, jest dołączane tylko z opcją Dane użytkownika
# { $service } is a service's name, such as AppUpdater.
core-account-left-out-service = Usługa { $service }
core-account-service-works-on-account-folder = Operuje na lokalizacji w folderach konta i jest dołączana tylko z opcją Dane użytkownika

## What a package was found to still say of whose PC it came from, after a build with none of an account's data.

# The parts of a package, as the warning below names them.
core-account-part-files = ścieżki własnych plików aplikacji
core-account-part-registry = wartości rejestru
core-account-part-settings-entry = wpis aplikacji w Ustawieniach
core-account-part-main-program = lokalizacja programu głównego
core-account-part-shortcuts = skróty
core-account-part-tasks = zaplanowane zadania
core-account-part-services = usługi
core-account-part-variables = zmienne środowiskowe
# One place in the list below. { $place } is one of the parts above, or a file's name; { $count } is how many times.
core-account-mention-place = { $place } ({ $count })
# { $kind } is what was found: account (an account's name), folder (where an account's folder is), computer (the PC's
# name), or id (an account's id). { $places } is a list of where, from the message above.
core-warning-scan-package-mentions =
    { $kind ->
        [account] Ten pakiet zawiera nazwę konta w następujących miejscach: { $places }. Został utworzony bez danych żadnego konta, a każdy, kto ma instalator, może to odczytać.
        [folder] Ten pakiet zawiera lokalizację folderu konta w następujących miejscach: { $places }. Został utworzony bez danych żadnego konta, a każdy, kto ma instalator, może to odczytać.
        [computer] Ten pakiet zawiera nazwę komputera w następujących miejscach: { $places }. Został utworzony bez danych żadnego konta, a każdy, kto ma instalator, może to odczytać.
       *[id] Ten pakiet zawiera identyfikator konta w następujących miejscach: { $places }. Został utworzony bez danych żadnego konta, a każdy, kto ma instalator, może to odczytać.
    }
# { $count } is how many folders.
core-warning-scan-folders-of-others-unreadable =
    { $count ->
        [one] Nie można odczytać { $count } folderu poza aplikacją, więc może brakować znajdujących się tam danych aplikacji.
        [few] Nie można odczytać { $count } folderów poza aplikacją, więc może brakować znajdujących się tam danych aplikacji.
        [many] Nie można odczytać { $count } folderów poza aplikacją, więc może brakować znajdujących się tam danych aplikacji.
       *[other] Nie można odczytać { $count } folderów poza aplikacją, więc może brakować znajdujących się tam danych aplikacji.
    }

## What a scan leaves out because it looks like a credential, or is a license with the License option off, in the app's
## list of what was left out. Each has a name, and below it why.

core-secret-why-credential = wygląda na poświadczenie
core-secret-why-license-option-off = opcja Dane licencji i aktywacji jest wyłączona
# { $count } is how many values of a registry key.
core-secret-why-account-values =
    { $count ->
        [one] { $count } wartość, dołączana tylko z opcją Dane użytkownika
        [few] { $count } wartości, dołączane tylko z opcją Dane użytkownika
        [many] { $count } wartości, dołączanych tylko z opcją Dane użytkownika
       *[other] { $count } wartości, dołączanych tylko z opcją Dane użytkownika
    }
# Shown in place of a registry value's name when its name itself looks like a password or a key.
core-secret-name-holds-credential = (wartość, której nazwa zawiera poświadczenie)
# Shown in place of an option's name, for an argument on a command line that has no name.
core-secret-unnamed-argument = argument zawierający poświadczenie
# { $option } is an option taken out of a command line, such as /password, or the message above. { $place } says
# where the command line was: service, task, shortcut, opener (what opens a file type), opener-unnamed (the same, of a
# type with no name), or uninstall. { $name } is the service, task, shortcut, file type, or app.
core-secret-taken-from =
    { $place ->
        [service] { $option } w poleceniu uruchamiającym usługę { $name }
        [task] { $option } w poleceniu uruchamiającym zadanie { $name }
        [shortcut] { $option } w skrócie { $name }
        [opener] { $option } w poleceniu otwierającym pliki typu { $name }
        [opener-unnamed] { $option } w poleceniu otwierającym pliki pewnego typu
       *[uninstall] { $option } w poleceniu odinstalowującym aplikację { $name }
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-secret-task-definition-unreadable = Definicja zadania { $task }, której nie można było odczytać w celu sprawdzenia
# { $name } is an environment variable's name, such as APP_TOKEN.
core-secret-left-out-variable = Zmienna środowiskowa { $name }
# { $class } is a COM class's id, such as {00000000-0000-0000-0000-000000000000}.
core-secret-left-out-class-server = Element, który według klasy { $class } jest przez nią uruchamiany
# { $rule } is a firewall rule's name.
core-secret-left-out-firewall-description = Opis reguły zapory { $rule }

## Licenses a scan found, as the package's report describes each.

# { $name } is a file's name, such as license.lic.
core-license-detail-file = Wykryto plik licencji: { $name }
# { $framework } is a licensing product's name, such as FlexLM / FlexNet.
core-license-detail-store = Wykryto magazyn licencji { $framework }
# { $name } is an environment variable's name, such as LM_LICENSE_FILE.
core-license-detail-variable = Zmienna środowiskowa licencji { $name }
# { $name } is a registry value's name, such as SerialNumber.
core-license-detail-registry-value = Wartość rejestru wskazuje na dane licencji: { $name }
# Said of a registry value in an account's key, which is not named.
core-license-detail-registry-value-of-an-account = Wartość rejestru wskazuje na dane licencji
# { $key } is a registry key, such as HKLM\Software\Contoso\License.
core-license-detail-registry-key = Klucz rejestru wskazuje na dane licencji: { $key }

## Warnings a scan records about licenses.

core-warning-scan-license-data = Wykryto dane licencji lub aktywacji. Gdy opcja Dane licencji i aktywacji jest włączona, instalator przywraca to, co można przenieść; wartości dotyczące całego komputera i magazyny powiązane ze sprzętem są tylko zgłaszane. Program WinAppMigrator nigdy nie odszyfrowuje danych, nie obchodzi aktywacji ani nie emuluje kluczy sprzętowych.
core-warning-scan-license-machine-bound = Niektóre licencje są powiązane z komputerem (klucz sprzętowy, aktywacja przypisana do komputera lub szyfrowanie). Instalator przywraca to, co może; te licencje mogą nadal wymagać ponownej aktywacji na nowym komputerze.

## Warnings a scan records about the registry. What could not be read is not named in them: it is kept beside them.

# { $error } is what Windows said went wrong. App Paths is a registry key's name.
core-warning-scan-app-paths-key-unreadable = Nie można odczytać podklucza App Paths: { $error }
core-warning-scan-registry-limit = Przechwytywanie rejestru osiągnęło skonfigurowany limit bezpieczeństwa.
core-warning-scan-registry-subtree-limit = Przechwytywanie poddrzewa rejestru osiągnęło skonfigurowany limit bezpieczeństwa.
# { $error } is what Windows said went wrong.
core-warning-scan-registry-key-unreadable = Nie można odczytać klucza rejestru: { $error }
core-warning-scan-registry-value-too-large = Wartość rejestru przekroczyła skonfigurowany limit rozmiaru przechwytywania.

## Scheduled tasks

core-warning-scan-tasks-folder-through-link = Folder zaplanowanych zadań na tym dysku prowadzi przez link, więc jego zadania nie zostały odczytane.
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-warning-scan-task-left-out = Pominięto zaplanowane zadanie { $task }: działa jako inne konto niż to, którego ustawienia są dołączane, jego plik mogą zmieniać nie tylko administratorzy lub aplikacja ma zbyt wiele zadań, aby je zachować.
# { $error } is what Windows said went wrong.
core-warning-scan-tasks-folder-unreadable = Nie można odczytać części zaplanowanych zadań: { $error }
# { $path } is a task's file, such as C:\Windows\System32\Tasks\App\Nightly Sync; { $reason } is one of the reasons
# below, or what Windows said went wrong.
core-warning-scan-task-unreadable = Nie można odczytać zaplanowanego zadania { $path }: { $reason }. Jeśli uruchamia ono aplikację, nie zostało dołączone.
# Why a task's file could not be read. { $size } is its size in bytes.
core-task-too-large = ma rozmiar { $size } B, za duży dla zadania
# Why a task's file could not be read.
core-task-ends-mid-character = kończy się w połowie znaku
# Why a task's file could not be read. { $error } says where.
core-task-not-utf16 = nie jest prawidłowym tekstem UTF-16: { $error }
# Why a task's file could not be read. { $error } says where.
core-task-not-text = nie jest prawidłowym tekstem UTF-8 ani UTF-16: { $error }
# What a package records of how a task is put back. Register-ScheduledTask is a command's name.
core-task-restore-instructions = Przywracanie korzysta z polecenia Register-ScheduledTask i zarchiwizowanej definicji XML.

## Reading a program's file. Each is why it could not be read, after the program's name.

# { $reason } is what went wrong.
core-pe-unreadable = nie można go odczytać: { $reason }
core-pe-headers-misaligned = jego nagłówki nie są wyrównane do granic wymaganych dla programu
# { $size } is the file's size in bytes.
core-pe-too-large = ma rozmiar { $size } B, za duży dla programu
# PE is the name of the format Windows programs are in.
core-pe-no-headers = nie ma nagłówków PE

## Warnings a scan records about what a program needs. Which program is kept beside each.

# { $error } is what went wrong.
core-warning-scan-imports-unreadable = Nie można odczytać tabeli importów: { $error }
# { $error } is what went wrong.
core-warning-scan-import-name-unreadable = Nie można odczytać nazwy deskryptora importu: { $error }
# { $count } is how many were skipped.
core-warning-scan-legacy-delay-imports =
    { $count ->
        [one] Pominięto { $count } starszy deskryptor importu z opóźnionym ładowaniem (v1); analizowane są tylko nowoczesne importy z opóźnionym ładowaniem oparte na adresach RVA.
        [few] Pominięto { $count } starsze deskryptory importu z opóźnionym ładowaniem (v1); analizowane są tylko nowoczesne importy z opóźnionym ładowaniem oparte na adresach RVA.
        [many] Pominięto { $count } starszych deskryptorów importu z opóźnionym ładowaniem (v1); analizowane są tylko nowoczesne importy z opóźnionym ładowaniem oparte na adresach RVA.
       *[other] Pominięto { $count } starszych deskryptorów importu z opóźnionym ładowaniem (v1); analizowane są tylko nowoczesne importy z opóźnionym ładowaniem oparte na adresach RVA.
    }

## The DLLs a program needs. Which DLL or program is kept beside each warning.

# { $path } is a program, such as C:\Program Files\App\app.exe.
core-dll-program-has-no-folder = plik wykonywalny nie ma folderu nadrzędnego: { $path }
# Why a DLL is among the app's files, in the package's list of them.
core-dll-beside-program = Biblioteka DLL znajduje się obok wybranego pliku wykonywalnego
# { $error } is what went wrong.
core-warning-scan-dependency-unscanned = Nie można przeanalizować zależności rekursywnej: { $error }
# { $name } is a DLL's name, such as d3dx9_43.dll.
core-warning-scan-dependency-not-in-newer-windows = Plik { $name } znajdował się w folderze Windows komputera, na którym była ta aplikacja, a systemy Windows 10 i 11 go nie zawierają. Umieściło go tam coś innego, więc nie należy do plików aplikacji i nie zostanie dołączony. Aplikacja może się nie uruchomić, dopóki nie zostanie zainstalowane to, co go dostarcza.
# { $error } is what went wrong.
core-warning-scan-dependency-not-hashed = Nie można obliczyć skrótu zależności: { $error }
# SystemRoot and windir are the names of environment variables.
core-warning-scan-windows-folder-unknown = Zmienne środowiskowe SystemRoot/windir są niedostępne; wyszukiwanie systemowych bibliotek DLL jest ograniczone.
# { $error } is what went wrong.
core-warning-scan-dependency-architecture-unreadable = Nie można odczytać architektury zależności: { $error }

## Services, drivers, shortcuts, and environment variables a scan reads. Where it was reading is kept beside each
## warning.

# { $error } is what Windows said went wrong.
core-warning-scan-services-unreadable = Nie można odczytać klucza rejestru usług: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-drivers-unreadable = Nie można odczytać klucza rejestru usług w celu wykrycia sterowników: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-queried = Zapytanie o skróty nie powiodło się: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-parsed = Nie można przeanalizować wyniku zapytania o skróty: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-environment-unreadable = Nie można odczytać klucza rejestru systemowych zmiennych środowiskowych: { $error }

## What a package records of how a driver is put back, which an installer shows. { $kind } is the driver's type as
## Windows names it (kernel, file-system, recognizer, or unknown-type); { $name } is its service's name; { $path } is
## its file. { $signed } is yes when { $signer } names who signed it, and no when that is not known.

core-service-driver-bundled =
    { $signed ->
        [yes] Sterownik typu { $kind } „{ $name }” należący do aplikacji ({ $path }, podpisany przez { $signer }). Instalator dołącza jego plik .sys i automatycznie ponownie rejestruje usługę sterownika (sc create, wymaga podwyższonych uprawnień), a następnie potwierdza, że usługa jest zarejestrowana.
       *[no] Sterownik typu { $kind } „{ $name }” należący do aplikacji ({ $path }, podpis nieznany). Instalator dołącza jego plik .sys i automatycznie ponownie rejestruje usługę sterownika (sc create, wymaga podwyższonych uprawnień), a następnie potwierdza, że usługa jest zarejestrowana.
    }
core-service-driver-from-vendor =
    { $signed ->
        [yes] Sterownik typu { $kind } „{ $name }” dostawcy lub wbudowany w system ({ $path }, podpisany przez { $signer }). Program WinAppMigrator nie rozpowszechnia sterowników innych firm; przed uruchomieniem aplikacji zainstaluj go ponownie na komputerze docelowym z podpisanego pakietu sterowników dostawcy.
       *[no] Sterownik typu { $kind } „{ $name }” dostawcy lub wbudowany w system ({ $path }, podpis nieznany). Program WinAppMigrator nie rozpowszechnia sterowników innych firm; przed uruchomieniem aplikacji zainstaluj go ponownie na komputerze docelowym z podpisanego pakietu sterowników dostawcy.
    }

## Runtimes an app needs, such as the Visual C++ runtime or .NET: how each is provided, as the Dependencies list says.

core-runtime-handling-built-in = Wbudowane w system Windows
core-runtime-handling-ships-with-app = Dostarczane z aplikacją
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-handling-windows-feature = Funkcja systemu Windows: { $feature }
core-runtime-handling-included = Dołączone
# { $id } is the runtime's name in winget, such as Microsoft.VCRedist.2015+.x64.
core-runtime-handling-winget = Instalowane za pomocą winget: { $id }
core-runtime-handling-manual = Zainstaluj ręcznie

## Downloading a runtime's installer, when an installer is built. Each is why it could not be, and starts with a small
## letter, as it follows other words. { $url } is a web address; { $error } is what went wrong.

core-runtime-download-not-https = odmowa pobrania z adresu { $url }: to nie jest adres HTTPS
# TLS is what keeps a download private.
core-runtime-tls-failed = inicjowanie protokołu TLS nie powiodło się: { $error }
core-runtime-download-failed = pobieranie z adresu { $url } nie powiodło się: { $error }
core-runtime-download-unreadable = odczyt z adresu { $url } nie powiódł się: { $error }
core-runtime-download-empty = pobieranie z adresu { $url } nie zwróciło żadnych danych
# { $came } and { $expected } are numbers of bytes.
core-runtime-download-cut-short = pobieranie z adresu { $url } zostało przerwane: odebrano { $came } z { $expected } B

## Installing a runtime: why it was not installed. Each starts with a small letter, as it follows other words.

core-runtime-no-known-installer = brak znanego instalatora tego środowiska uruchomieniowego
# { $path } is where in the package, such as prerequisites\vc_redist.x64.exe.
core-runtime-payload-misplaced = pakiet wskazuje plik { $path } jako instalator tego środowiska uruchomieniowego, ale plik ten nie znajduje się we właściwym miejscu, więc nie został uruchomiony
# { $path } is a file.
core-runtime-payload-missing = brak dołączonego instalatora: { $path }
# { $error } is what went wrong.
core-runtime-payload-path-invalid = ścieżka dołączonego instalatora jest nieprawidłowa: { $error }
# { $url } is the web address it can be downloaded from. winget is a program's name.
core-runtime-no-installer-route = brak dołączonego instalatora i identyfikatora winget; pobierz środowisko ze strony { $url }
# { $error } is what went wrong.
core-runtime-no-temp-folder = nie można utworzyć folderu, w którym zostałby rozpakowany: { $error }
# { $reason } is what the check found.
core-runtime-payload-signature-failed = dołączony instalator nie przeszedł weryfikacji podpisu i nie został uruchomiony: { $reason }
core-runtime-payload-timed-out = dołączony instalator nie zakończył działania w wyznaczonym czasie i został zakończony
# { $error } is what went wrong.
core-runtime-payload-not-started = nie można uruchomić dołączonego instalatora: { $error }
core-runtime-installer-killed = instalator został zakończony bez kodu zakończenia
# { $code } is the number it ended with, such as 1603.
core-runtime-installer-failed = instalator zakończył działanie z kodem { $code }
# { $feature } is a Windows feature's name, such as NetFx3; { $code } is the number DISM ended with. Windows Features is
# the name of a Windows window. Keep the command as it is.
core-runtime-feature-failed = narzędzie DISM nie mogło włączyć funkcji systemu Windows { $feature } (kod zakończenia { $code }); prawdopodobnie nie można było pobrać jej danych. Włącz ją w oknie Funkcje systemu Windows lub uruchom polecenie `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`.
# The same, when DISM gave no number.
core-runtime-feature-failed-no-code = narzędzie DISM nie mogło włączyć funkcji systemu Windows { $feature } (kod zakończenia nieznany); prawdopodobnie nie można było pobrać jej danych. Włącz ją w oknie Funkcje systemu Windows lub uruchom polecenie `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`.
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-feature-timed-out = włączanie funkcji systemu Windows { $feature } nie zakończyło się w wyznaczonym czasie i zostało przerwane
# { $feature } is a Windows feature's name; { $error } is what went wrong.
core-runtime-feature-no-dism = narzędzie DISM jest niedostępne, więc nie można włączyć funkcji systemu Windows { $feature }: { $error }

## Installing a runtime with winget. { $id } is its name in winget, such as Microsoft.VCRedist.2015+.x64.

# { $error } is what went wrong.
core-runtime-winget-not-looked-for = nie można wyszukać narzędzia winget: { $error }
core-runtime-winget-not-installed = narzędzie winget nie jest zainstalowane
# { $path } is where winget is.
core-runtime-winget-misplaced = plik { $path } nie znajduje się w miejscu, w którym system Windows przechowuje własne pakiety, więc nie został uruchomiony
# { $path } is where winget is; { $reason } is what the check found.
core-runtime-winget-signature-failed = plik { $path } nie przeszedł weryfikacji podpisu i nie został uruchomiony: { $reason }
# { $reason } is one of the two messages above, or what went wrong.
core-runtime-winget-unusable = nie można użyć narzędzia winget do zainstalowania pakietu { $id }: { $reason }
# { $code } is the number winget ended with.
core-runtime-winget-failed = polecenie winget install { $id } zakończyło działanie z kodem { $code }
core-runtime-winget-failed-no-code = polecenie winget install { $id } zakończyło działanie z nieznanym kodem
core-runtime-winget-timed-out = polecenie winget install { $id } nie zakończyło działania w wyznaczonym czasie i zostało przerwane
# { $error } is what went wrong.
core-runtime-winget-not-started = narzędzie winget jest niedostępne, więc nie można zainstalować pakietu { $id }: { $error }

## What told a scan how an app was installed, as the package's report lists it. The names of installers and files are
## as their makers write them.

# { $path } is an installer's record of what it installed, such as C:\Program Files\App\unins000.dat.
core-source-record-not-believed = nie użyto pliku { $path }: mogą go zmieniać nie tylko administratorzy
# { $count } is how many Windows Installer packages the bundle holds.
core-source-burn-bundle =
    { $count ->
        [one] Pakiet zbiorczy Burn zawierający { $count } pakiet Instalatora Windows
        [few] Pakiet zbiorczy Burn zawierający { $count } pakiety Instalatora Windows
        [many] Pakiet zbiorczy Burn zawierający { $count } pakietów Instalatora Windows
       *[other] Pakiet zbiorczy Burn zawierający { $count } pakietów Instalatora Windows
    }
# { $count } is how many paths Windows Installer recorded.
core-source-msi-database =
    { $count ->
        [one] Baza danych Instalatora Windows: { $count } ścieżka klucza składnika
        [few] Baza danych Instalatora Windows: { $count } ścieżki kluczy składników
        [many] Baza danych Instalatora Windows: { $count } ścieżek kluczy składników
       *[other] Baza danych Instalatora Windows: { $count } ścieżek kluczy składników
    }
core-source-inno-log = Dziennik dezinstalacji Inno Setup (unins000.dat)
# { $count } is how many paths the log records.
core-source-inno-log-paths =
    { $count ->
        [one] Dziennik dezinstalacji Inno Setup: { $count } zapisana ścieżka
        [few] Dziennik dezinstalacji Inno Setup: { $count } zapisane ścieżki
        [many] Dziennik dezinstalacji Inno Setup: { $count } zapisanych ścieżek
       *[other] Dziennik dezinstalacji Inno Setup: { $count } zapisanych ścieżek
    }
core-source-nsis = Sygnatura dezinstalatora NSIS (Nullsoft)
# { $count } is how many paths the log records.
core-source-nsis-log-paths =
    { $count ->
        [one] Plik uninstall.log NSIS AdvUninstLog: { $count } zapisana ścieżka
        [few] Plik uninstall.log NSIS AdvUninstLog: { $count } zapisane ścieżki
        [many] Plik uninstall.log NSIS AdvUninstLog: { $count } zapisanych ścieżek
       *[other] Plik uninstall.log NSIS AdvUninstLog: { $count } zapisanych ścieżek
    }
core-source-squirrel = Układ Squirrel dla użytkownika (Update.exe / app-*)
# { $version } is a folder's name that says the version, such as app-1.2.3.
core-source-squirrel-version = Bieżąca wersja Squirrel: { $version }
core-source-chocolatey = Metadane pakietu Chocolatey
core-source-msix = Pakiet MSIX / Appx
# { $identity } is a package's name, such as Contoso.App.
core-source-appx-identity = Tożsamość w pliku AppxManifest: { $identity }
# { $count } is how many folders.
core-source-msix-data =
    { $count ->
        [one] Dane pakietu MSIX dla użytkownika: { $count } lokalizacja
        [few] Dane pakietu MSIX dla użytkownika: { $count } lokalizacje
        [many] Dane pakietu MSIX dla użytkownika: { $count } lokalizacji
       *[other] Dane pakietu MSIX dla użytkownika: { $count } lokalizacji
    }
# App Paths is a registry key's name.
core-source-app-paths = Rejestracja w kluczu App Paths

## Why a runtime is needed, as the package's report says it.

# { $program } is a program's file name, such as app.exe; { $dll } is a DLL's, such as msvcp140.dll.
core-runtime-program-imports = Program { $program } importuje bibliotekę { $dll }

## What else of an app's a scan finds in the registry: fonts and certificates, and why each is the app's.

# Why a font is among the app's files, in the package's list of them.
core-registry-font-recorded = Czcionka wymieniona w zapisie instalacji aplikacji
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-named-for-the-app = jego nazwa wskazuje aplikację lub jej wydawcę
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-recorded = jest wymieniony w zapisie instalacji aplikacji

## Who is signed in, for the Whose Data choice. Each of the first six is why it could not be told, and starts with a
## small letter, as it is shown inside other words. An account is shown by its id, such as S-1-5-21-...-1001.

# { $error } is what Windows said.
core-profile-desktop-owner-not-said = system Windows nie podał, czyj jest ten pulpit: { $error }
# { $error } is what went wrong.
core-profile-desktop-owner-not-asked = nie można zapytać systemu Windows, czyj jest ten pulpit: { $error }
core-profile-signed-in-account-no-profile = konto zalogowane tutaj, { $account }, nie ma zapisanego profilu
core-profile-desktop-not-a-person = pulpit należy tutaj do konta { $account }, które nie jest kontem osoby
core-profile-nobody-signed-in-running-account-no-profile = nikt nie jest tu zalogowany, a konto, na którym działa ten program, { $account }, nie ma zapisanego profilu
core-profile-nobody-signed-in-running-not-a-person = nikt nie jest tu zalogowany, a konto { $account }, na którym działa ten program, nie jest kontem osoby
# SID is the name Windows gives an account's id.
core-profile-nobody-signed-in-no-sid = nikt nie jest tu zalogowany, a to konto nie ma identyfikatora SID
# { $accounts } is a list of accounts.
core-profile-several-desktops = więcej niż jedno konto ma pulpit w tej sesji: { $accounts }
# Under the Whose Data choice. { $reason } is one of the messages above.
core-profile-signed-in-unknown = Nie można ustalić, kto jest zalogowany ({ $reason }).
# Under the Whose Data choice, when there are accounts to choose from. { $reason } is one of the messages above.
core-profile-signed-in-unknown-choose = Nie można ustalić, kto jest zalogowany ({ $reason }). Wybierz, czyje dane dołączyć.
