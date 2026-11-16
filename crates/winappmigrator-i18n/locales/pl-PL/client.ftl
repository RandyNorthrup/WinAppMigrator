# Words of WinAppMigrator Client, the console program run on the old PC, the one an app is moving off, so that
# WinAppMigrator on the new PC can read it over the network. Message names start with client-.
#
# Its window is a console, which on Windows 7 is 80 columns wide. Each line break in a message is one on the screen:
# keep every line shorter than 80 characters, and break lines where your language needs to, not only where English
# does.


## Starting

# Shown only in English, in place of a right-to-left language the console cannot show. { $language } is that
# language's name in English, such as Arabic.
client-console-rtl-fallback = Ta konsola nie może wyświetlić języka { $language }, więc klient używa języka angielskiego.
# The client copies itself to a folder in Program Files before it starts. { $error } is what Windows said went wrong.
client-safe-folder-failed =
    Nie można uruchomić programu WinAppMigrator Client z folderu, który mogą
    zmieniać tylko administratorzy: { $error }
client-press-enter-to-close = Naciśnij klawisz Enter, aby zamknąć to okno.
# The first line of its screen. { $version } is the client's version, such as 0.6.2.
client-heading = WinAppMigrator Client { $version }
client-intro =
    Dzięki temu program WinAppMigrator na nowym komputerze może przez sieć
    odczytać aplikacje z tego komputera. Klient niczego tu nie zmienia: odczytuje
    migawkę dysku systemu Windows, którą usuwa po zatrzymaniu.
# { $error } is why: one of the messages under "Why it stopped" below, or what Windows said.
client-stopped = Działanie klienta zostało zatrzymane: { $error }
# The caretaker is a copy of the client with no window that cleans up after it. { $error } is what Windows said.
client-caretaker-failed =
    Jeśli zamkniesz to okno, zamiast zatrzymać klienta, migawka i reguły zapory
    pozostaną do następnego uruchomienia klienta na tym komputerze ({ $error }).
# Shown while the snapshot is taken, followed on the same line by client-snapshot-taken. Keep the quotes: they keep
# the space before it.
client-snapshot-taking = Tworzenie migawki dysku systemu Windows...{ " " }
client-snapshot-taken = gotowe.
# The old PC answers the new one's search for it on the network. { $error } is what Windows said went wrong.
client-discovery-failed =
    Ten komputer nie będzie widoczny automatycznie ({ $error }).
    Zamiast tego na nowym komputerze wpisz nazwę lub adres tego komputera.

## This PC and its pairing code. The three lines are lined up in two columns, behind two spaces; keep the quotes, which
## keep those spaces.

# { $computer_name } is this PC's name, such as OLD-PC.
client-this-pc = { "  " }Ten komputer:   { $computer_name }
# { $windows_name } is this PC's Windows, such as Windows 7 Professional Service Pack 1. { $bits } is 32 or 64.
client-windows = { "  " }Windows:        { $windows_name }, { $bits }-bitowy
# { $code } is six digits in two groups, such as 482 913.
client-pairing-code = { "  " }Kod parowania:  { $code }
# Network PC... is the button of that name in WinAppMigrator: use the name it has there in your language.
client-pairing-steps =
    Na nowym komputerze w programie WinAppMigrator wybierz opcję
    Komputer w sieci... i wpisz ten kod. Najpierw zamknij przenoszoną aplikację,
    aby jej pliki pozostały w takim stanie, w jakim je zostawiła.
    Aby zatrzymać, naciśnij tutaj klawisz Enter lub zamknij to okno.

## While it runs. { $from } is the network address of the PC that connected, such as 192.168.1.20, or "unknown" when
## it could not be told, which the [unknown] variant words.

client-paired =
    Sparowano z { $from ->
        [unknown] innym komputerem
       *[address] komputerem { $from }
    }. Trwa odczytywanie potrzebnych danych...
# { $count } is how many requests it answered.
client-builder-done =
    { $from ->
        [unknown] Inny komputer
       *[address] Komputer { $from }
    } zakończył pracę (odpowiedziano na { $count ->
        [one] { $count } żądanie
        [few] { $count } żądania
        [many] { $count } żądań
       *[other] { $count } żądań
    }).
# { $error } is what went wrong, often in Windows' words.
client-link-ended =
    Połączenie z { $from ->
        [unknown] innym komputerem
       *[address] komputerem { $from }
    } zostało zakończone: { $error }
client-wrong-code =
    { $from ->
        [unknown] Inny komputer
       *[address] Komputer { $from }
    } próbował użyć nieprawidłowego kodu.
# { $error } is what went wrong, often in Windows' words.
client-pair-failed =
    Parowanie z { $from ->
        [unknown] innym komputerem
       *[address] komputerem { $from }
    } nie powiodło się: { $error }
client-too-many-wrong-codes =
    Wypróbowano zbyt wiele nieprawidłowych kodów, więc teraz żaden komputer
    nie może sparować się z tym komputerem. Coś w sieci może próbować odgadnąć
    kod. Aby sparować, zatrzymaj klienta, uruchom go ponownie i wpisz nowy kod,
    który zostanie wyświetlony.
# { $code } is six digits in two groups, such as 482 913.
client-new-code = Zbyt wiele nieprawidłowych kodów. Nowy kod parowania: { $code }

## Stopping

client-stopping = Zatrzymywanie: usuwanie migawki i reguł zapory...
client-stop-done = Gotowe.

## Why it stopped, shown after "The client stopped: " (client-stopped), and written to follow it.

client-already-running = na tym komputerze klient już działa albo kończy porządkowanie po kliencie, którego okno zamknięto. Użyj tamtego klienta albo zaczekaj chwilę i uruchom ten ponownie.
# { $drive } is the drive Windows is on, such as C:. { $details } is what Windows said, in its own words.
client-snapshot-failed = system Windows nie utworzył migawki dysku { $drive }: { $details }
# { $details } is what Windows Firewall said, in its own words.
client-firewall-failed = Zapora systemu Windows nie zezwoliła klientowi na nasłuchiwanie: { $details }

## What it takes, shown for --help. The options, such as --lang, are typed as they are and are not translated; keep
## their descriptions lined up.

client-usage =
    Program WinAppMigrator Client obsługuje następujące opcje:
    
      --lang <język>     Używaj języka (np. it-IT), jeśli klient go obsługuje,
                         zamiast języka wyświetlania systemu Windows.
      --code <kod>       Użyj tego sześciocyfrowego kodu parowania, gdy skrypt
                         uruchamia też WinAppMigrator na nowym komputerze.
      --help             Wyświetl tę pomoc.
