<a id="installing-on-the-new-pc"></a>

# Instalowanie na nowym komputerze

Instalator to jedno okno. Sprawdza sam siebie, pokazuje, gdzie zainstaluje aplikację, i wszystko, co skonfiguruje, a instaluje dopiero po kliknięciu przycisku **Zainstaluj**. Ta strona omawia okno od góry do dołu, używając słów widocznych na ekranie.

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/installer-dark.png">
    <img alt="Instalator aplikacji Fabrikam Data Sync z informacją, że nie jest podpisany, lokalizacją instalacji, wymaganym miejscem i listą tego, co konfiguruje oprócz plików" src="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/installer-light.png" width="600">
  </picture>
</p>

<a id="before-you-start"></a>

## Zanim zaczniesz

- **Skopiuj cały instalator.** Jeśli podczas tworzenia oprócz pliku `.exe` powstał również plik `.wamdata`, skopiuj oba do tego samego folderu i nie zmieniaj ich nazw. Zobacz [Duże aplikacje](Big-Apps).
- **Uruchamiaj tylko zaufane instalatory.** Instalator działa z pełnymi [uprawnieniami administratora](Glossary#administrator-rights). Traktuj go jak każdy inny program: uruchamiaj go tylko wtedy, gdy został utworzony samodzielnie lub pochodzi od zaufanej osoby. Zobacz [Prywatność i bezpieczeństwo](Privacy-and-Security).
- **Potrzebne są uprawnienia administratora** na tym komputerze oraz [Direct3D 12](Glossary#direct3d-12) do wyświetlenia okna. Bez Direct3D 12 nadal można zainstalować aplikację z wiersza polecenia. Zobacz [Wiersz polecenia](Command-Line).

<a id="open-it"></a>

## Otwieranie instalatora

Kliknij dwukrotnie instalator. System Windows wyświetli [monit Kontroli konta użytkownika](Glossary#uac-prompt). Instalator nie jest podpisany, chyba że podpisała go osoba, która go utworzyła, więc monit zwykle wyświetla **Nieznany wydawca**. To normalne. Zobacz [Prywatność i bezpieczeństwo](Privacy-and-Security#signing).

Okno „Instalator WinAppMigrator” z podtytułem „Ponownie instaluje aplikację przechwyconą na innym komputerze” otwiera się od razu, niezależnie od rozmiaru instalatora. Wyświetla „Sprawdzanie instalatora…”, gdy sprawdza [skrót](Glossary#hash) całego swojego pakietu. Wymaga to odczytania całego pakietu, więc w przypadku dużego instalatora trwa to chwilę. Od tego momentu instalator utrzymuje otwarty własny plik, a także swój plik danych, jeśli go ma, więc żadnego z nich nie można zmienić ani podmienić, dopóki okno jest otwarte.

Sprawdzanie może zakończyć się w tym miejscu czerwonym komunikatem. Najczęstsze z nich to:

- Instalator jest uszkodzony lub został zmieniony po utworzeniu: skopiuj go ponownie z miejsca, w którym został utworzony.
- Brakuje jego pliku danych albo nie jest to właściwy plik: skopiuj ponownie oba pliki do jednego folderu.
- „Pakietu tego instalatora nie można użyć na tym komputerze”, na przykład dlatego, że programy aplikacji są przeznaczone dla innego rodzaju procesora albo wymagają nowszej wersji systemu Windows. Niczego nie zmieniono.

Wszystkie te przypadki opisano na stronie [Rozwiązywanie problemów](Troubleshooting#installing).

Gdy sprawdzanie zakończy się pomyślnie, tytuł okna zmienia się na „Instalacja aplikacji” i nazwę aplikacji.

<a id="its-language"></a>

### Język instalatora

Instalator używa języka komputera, na którym jest uruchamiany, a nie tego, na którym go utworzono: języka wyświetlania systemu Windows, jeśli jest to jeden z 24 języków programu WinAppMigrator, a w przeciwnym razie angielskiego. Menu **Język** w prawym górnym rogu zmienia język na czas tego uruchomienia. Informacje o aplikacji, w tym uwagi zapisane przez program podczas tworzenia instalatora, również są wyświetlane w tym języku. Instalator utworzony za pomocą programu WinAppMigrator 0.6 lub starszego wyświetla te uwagi po angielsku, tak jak zostały zapisane. [Instalacja w trybie cichym z wiersza polecenia](Command-Line) zawsze odpowiada po angielsku, aby skrypty mogły odczytać odpowiedź.

<a id="the-app-and-where-it-goes"></a>

## Aplikacja i miejsce jej instalacji

Pierwsza karta zawiera nazwę aplikacji, a pod nią wydawcę i wersję, na przykład „Fabrikam Inc. · 1.7.3”. Dalej znajdują się wiersze:

| Wiersz | Co zawiera |
| --- | --- |
| **Przechwycono z** | Nazwa komputera, z którego pochodzi aplikacja. Pojawia się tylko wtedy, gdy instalator został utworzony z włączoną opcją **Dane użytkownika**. |
| **Instalator** | Czy ten plik instalatora jest [podpisany](Glossary#signature): „Sprawdzanie podpisu…”, a następnie „Niepodpisany: nieznany wydawca” albo „Podpisany przez:” i nazwa podpisującego. Jeśli podpis jest nieważny, zamiast tego pojawia się czerwony komunikat: „Podpis tego instalatora jest nieprawidłowy” z przyczyną podaną przez system Windows oraz „Instalator mógł zostać zmieniony po podpisaniu.” |
| **Lokalizacja instalacji** | Folder, do którego trafi aplikacja, z przyciskiem **Przeglądaj** do wybrania innego. Początkowo jest to folder, w którym aplikacja znajdowała się na starym komputerze. Aplikacja z 32-bitowego systemu Windows trafia początkowo do folderu `Program Files (x86)`, w którym 64-bitowy system Windows przechowuje aplikacje 32-bitowe. Jeśli tej lokalizacji nie można tu użyć, czerwony komunikat pod nią podaje przyczynę. |
| **Wymagane miejsce** | Ile miejsca na tym dysku wymaga instalacja i ile jest wolnego, na przykład „do 824,0 MB na C: (wolne: 356,0 GB)”. Uwzględnia zarówno rozpakowany pakiet, jak i pliki aplikacji. |

Nazwa wydawcy w pierwszym wierszu pochodzi z pakietu, więc mógł ją wpisać każdy. Wiersz **Instalator** pochodzi od systemu Windows i to jemu należy ufać.

<a id="when-the-location-cannot-be-used"></a>

### Gdy nie można użyć lokalizacji

Jeśli nie można użyć folderu podanego w polu **Lokalizacja instalacji**, czerwony komunikat pod nim podaje przyczynę. Przyciski **Zainstaluj** i **Podgląd zmian** pozostają wyszarzone, dopóki nie wybierzesz innego folderu, a wiersz obok nich informuje: „Przyciski Zainstaluj i Podgląd zmian są niedostępne, dopóki nie będzie można użyć lokalizacji instalacji.”

Najczęstszą przyczyną jest to, że folder zawiera już pliki. Folder z plikami jest używany tylko wtedy, gdy umieściła je tam wcześniejsza instalacja z programu WinAppMigrator, dzięki czemu pliki innej aplikacji nigdy nie zostaną zastąpione:

| Komunikat | Co oznacza | Co zrobić |
| --- | --- | --- |
| „Aplikacja”, jej nazwa, a następnie „jest już zainstalowana w tym folderze, więc tej kopii nie można w nim umieścić.” | Aplikacja jest już zainstalowana na tym komputerze, często przez jej własny instalator: jej główny program znajduje się w tym folderze albo lista zainstalowanych aplikacji systemu Windows wskazuje ten folder. Zdarza się to w przypadku aplikacji, które ma już wiele komputerów, takich jak przeglądarka internetowa, odtwarzacz multimediów lub pakiet Office. | Najpierw odinstaluj kopię z tego komputera w Ustawieniach systemu Windows w sekcji **Aplikacje**, a następnie ponownie otwórz instalator. Możesz też wpisać w polu **Lokalizacja instalacji** nowy, pusty folder, aby zainstalować tę kopię obok. |
| „W tym folderze jest zainstalowana aplikacja”, nazwa innej aplikacji i informacja, że tej aplikacji „nie można w nim umieścić”. | Lista zainstalowanych aplikacji systemu Windows wskazuje w tym folderze inną aplikację, na przykład inną wersję pakietu Office. | Wpisz w polu **Lokalizacja instalacji** nowy, pusty folder. Jeśli tamta aplikacja nie jest już potrzebna, możesz też najpierw ją odinstalować w Ustawieniach systemu Windows w sekcji **Aplikacje**, a następnie ponownie otworzyć instalator. |
| „Ten folder zawiera już pliki, które nie zostały zainstalowane przez program WinAppMigrator.” | W folderze jest coś innego, a lista nie wskazuje w nim żadnej zainstalowanej aplikacji. Może to należeć do innej aplikacji. | Wpisz w polu **Lokalizacja instalacji** nowy, pusty folder. Jeśli pliki pozostały po odinstalowanej aplikacji, możesz usunąć folder, a następnie ponownie otworzyć instalator. |

Z każdej innej przyczyny komunikat brzmi „Nie można użyć tego folderu:”, po czym następuje przyczyna i „Wpisz powyżej inny folder.” Możliwe przyczyny to:

| Przyczyna na ekranie | Co zrobić |
| --- | --- |
| „musi to być pełna ścieżka” | Wpisz ścieżkę zaczynającą się od litery dysku, na przykład `C:\Program Files\Fabrikam Data Sync`. |
| „nie może to być cały dysk” | Użyj folderu na dysku, a nie samego dysku. |
| „nie może to być folder Windows” | Wybierz folder poza `C:\Windows`. |
| „zawiera obok siebie inne elementy, więc aplikacja musi mieć w nim własny podfolder” | Dodaj folder dla aplikacji, na przykład `C:\Program Files\Fabrikam Data Sync` zamiast `C:\Program Files`. |
| „jest to folder należący do systemu Windows lub folder, którego zawartość jest automatycznie uruchamiana lub ładowana” | Wybierz zwykły folder. Na przykład folder Autostart zostanie odrzucony. |
| „musi to być zwykła ścieżka na dysku tego komputera” | [Folder udostępniony](Glossary#network-share) zostanie odrzucony, podobnie jak nazwa, którą system Windows odczytałby jako coś innego. |
| informacja, że folder „jest linkiem do innego miejsca, a inni mogą zmienić, dokąd prowadzi” | Użyj folderu, do którego prowadzi [link](Glossary#link). |
| „jest to miejsce, w którym instalacje przechowują swoje zapisy” | Wybierz folder poza `%ProgramData%\WinAppMigrator`. |

Jeśli folder zawiera pliki i **jest** w nim zapisana wcześniejsza instalacja tego projektu, instalator go użyje i ostrzeże: „Ten folder zawiera już pliki. Pliki o takich samych nazwach zostaną zastąpione i wrócą na miejsce, jeśli odinstalujesz aplikację.”

Po zaakceptowaniu folderu okno zastępuje plik, wartość rejestru, usługę, zadanie, skrót lub zmienną o tej samej nazwie, tak jak opcja `--force` w [wierszu polecenia](Command-Line#what---force-does). To, co już należy do innej aplikacji, na przykład jej usługa lub typ pliku, nadal pozostaje własnością tamtej aplikacji.

Jeśli na dysku może brakować miejsca, bursztynowa uwaga informuje: „W tej lokalizacji może brakować miejsca” i podaje, ile go potrzeba. Pakiet można rozpakować na innym dysku, ale pliki aplikacji nadal muszą zmieścić się na tym. Sama instalacja przed rozpoczęciem ponownie sprawdza dostępne miejsce.

<a id="an-earlier-install-that-never-finished"></a>

### Nieukończona wcześniejsza instalacja

Jeśli poprzednia instalacja wykonywana przez którykolwiek z tych instalatorów została przerwana, na przykład przez awarię lub zanik zasilania, bursztynowa uwaga informuje: „Wcześniejsza instalacja aplikacji”, nazwa aplikacji, „nie została ukończona. Instalacja najpierw ją cofnie.” Dzieje się to automatycznie po kliknięciu przycisku **Zainstaluj**.

<a id="what-this-sets-up-beyond-files"></a>

## Co ta instalacja konfiguruje oprócz plików

Ta karta to lista, na którą wyrażasz zgodę, klikając przycisk **Zainstaluj**. Pod jej tytułem znajduje się tekst:

> Wszystkie te czynności są wykonywane z uprawnieniami administratora. Jeśli instalacja się nie powiedzie, wprowadzone zmiany zostaną cofnięte, a otrzymasz informację o wszystkim, czego nie udało się cofnąć. Instalację możesz też cofnąć później w Ustawieniach, w sekcji Zainstalowane aplikacje. Zastąpione pliki wrócą wtedy na miejsce, a zainstalowane środowiska uruchomieniowe pozostaną.

Każdy wiersz zawiera nazwę, etykietę po prawej stronie określającą rodzaj elementu oraz wiersz opisujący, co ten element robi. Etykieta w zwykłym niebieskim kolorze oznacza zwykły element. Etykieta w kolorze **bursztynowym** oznacza coś, co działa z pełnymi uprawnieniami, na przykład usługę działającą jako LocalSystem, sterownik lub regułę zapory zezwalającą na połączenia przychodzące. Takim elementom przyjrzyj się dwa razy.

Jeśli lista jest pusta, wyświetlany jest tekst „Nic: kopiowane są tylko pliki oraz ustawienia we własnych kluczach rejestru aplikacji.”

Lista jest ustalana dla lokalizacji podanej w polu. Jeśli zmienisz lokalizację, przez chwilę wyświetlany jest tekst „Ustalanie, co zostałoby skonfigurowane w tej lokalizacji…”, a przycisk **Zainstaluj** czeka.

<a id="what-each-label-means"></a>

### Znaczenie etykiet

| Etykieta | Co to jest | Przykład wiersza pod nią |
| --- | --- | --- |
| **Usługa** | [Usługa](Glossary#service): program uruchamiany przez system Windows w tle | „Uruchamia plik FabrikamSync.exe jako NT AUTHORITY\LocalService; uruchamianie: od razu i przy każdym uruchomieniu komputera”. W innych na końcu jest „od razu i wkrótce po każdym uruchomieniu komputera”, „tylko na żądanie” lub „wyłączone”. |
| **Sterownik** | [Sterownik](Glossary#driver) należący do aplikacji | „Ładuje plik” i jego nazwa „bezpośrednio do systemu Windows”. Zawsze bursztynowa. Sterownik uruchamiany razem z systemem Windows jest uruchamiany zaraz po instalacji. Sterownik programu VeraCrypt został w ten sposób załadowany ręcznie na komputerze testowym; CI nie ładuje żadnego sterownika. |
| **Zaplanowane zadanie** | [Zaplanowane zadanie](Glossary#scheduled-task) | „Działa jako SYSTEM” lub jako konto. Tekst „z najwyższymi możliwymi uprawnieniami” jest oznaczony na bursztynowo. Zadanie, które na starym komputerze działało jako konkretna osoba, działa jako osoba, która je instaluje. |
| **Autostart** | [Element autostartu](Glossary#startup-item) | „Uruchamia się przy logowaniu bieżącego użytkownika”, „Uruchamia się przy logowaniu każdego użytkownika” lub „Skrót w folderze Autostart, tylko dla bieżącego użytkownika” |
| **Skrót** | [Skrót](Glossary#shortcut) na Pulpicie lub w menu Start | „Otwiera plik Fabrikam.exe, dla wszystkich użytkowników tego komputera” lub „tylko dla bieżącego użytkownika” |
| **PATH** | Wpis dodany do [zmiennej Path](Glossary#path-variable) | „Dodaje do zmiennej Path” i „Folder instalacji\bin”. Wpis jest dodawany do dotychczasowej zawartości, nigdy jej nie zastępuje. |
| **Zmienna środowiskowa** | [Zmienna środowiskowa](Glossary#environment-variable) | „Wartość:” i jej wartość |
| **Rozszerzenie powłoki** | Kod ładowany do Eksploratora plików | „Ładuje plik” i jego nazwa „do Eksploratora plików, dla wszystkich użytkowników tego komputera”. Bursztynowa. |
| **Polecenie menu** | Polecenie w menu wyświetlanym po kliknięciu prawym przyciskiem myszy | „Dodaje polecenie do menu elementów typu” i rodzaj pliku. Tekst „Zmienia akcję wykonywaną przy otwieraniu takich elementów” jest bursztynowy. |
| **Typ linku** | [Protokół linków](Glossary#link-protocol), na przykład `fabrikam:` | „Linki tego typu są otwierane w aplikacji” |
| **Typ pliku** | [Typ pliku](Glossary#file-type), na przykład `.fds` | „Pliki tego typu są otwierane w aplikacji” |
| **COM** | [Klasy COM](Glossary#com-class), które mogą uruchamiać inne programy | „COM w pliku” i nazwa pliku, z liczbą ustawień. „Uprawnienia COM” są bursztynowe. |
| **Reguła zapory** | [Reguła zapory](Glossary#firewall-rule) dla jednego z programów aplikacji | „Zezwala na przychodzące połączenia na porcie 8080 programu FabrikamSync.exe w każdej sieci”. Zezwolenie na połączenia przychodzące jest bursztynowe. |
| **Źródło dziennika zdarzeń** | Nazwa, pod którą aplikacja zapisuje w dziennikach zdarzeń systemu Windows | „Może zapisywać w dzienniku Application komunikaty pochodzące z własnych plików” |
| **Czcionka** | Czcionka w folderze instalacji | „Ładowana z folderu instalacji podczas logowania” |
| **Sterownik ODBC** | Sterownik bazy danych dla [ODBC](Glossary#odbc) | „Ładowany do programów, które łączą się z bazą danych za pomocą ODBC”. Bursztynowa. |
| **Źródło danych ODBC** | Zapisane połączenie z bazą danych | „Połączenie z bazą danych, dla wszystkich użytkowników tego komputera” |
| **Ustawienie zgodności** | Sposób, w jaki system Windows uruchamia niektóre programy aplikacji | „Sposób uruchamiania programów” |
| **Aplikacje domyślne** | Wpis, dzięki któremu aplikację można wybrać w obszarze Aplikacje domyślne | „Można ją wybrać w obszarze Aplikacje domyślne. Ustawienia domyślne nie są zmieniane” |
| **Nazwa programu** | Nazwa, pod którą okno Uruchamianie i inne programy mogą znaleźć aplikację | „Okno Uruchamianie i inne programy mogą znaleźć program pod tą nazwą” |
| **Inne ustawienia** | Wartości [rejestru](Glossary#registry) poza własnymi kluczami aplikacji i rodzajami wymienionymi powyżej | „3 ustawienia poza własnymi kluczami aplikacji”, „Klucze:” i lista kluczy. Bursztynowa, jeśli którekolwiek z nich znajdują się w kluczach firmy Microsoft. |

Niektóre elementy można ocenić dopiero wtedy, gdy instalacja do nich dotrze, na przykład usługę o tej samej nazwie, która już należy do innej aplikacji. Takie elementy pozostają na liście w zażądanej postaci. Gdy instalacja do nich dotrze, pozostawia je bez zmian i informuje o tym.

<a id="notes-and-what-is-left-out"></a>

### Uwagi i pominięte elementy

Pod wierszami bursztynowe uwagi informują o tym, co warto wiedzieć, na przykład:

- że poświadczenia nie zostały umieszczone w instalatorze, więc może być konieczne ponowne ich wprowadzenie;
- że podczas tworzenia instalatora opcja **Licencja** lub **Dane użytkownika** była wyłączona, więc tych danych w nim nie ma;
- na których [certyfikatach](Glossary#certificate) aplikacja zdawała się polegać, choć certyfikaty nigdy nie są instalowane;
- co osoba tworząca instalator pominęła, a co dodała;
- że aplikacja może tu nie działać: „Ta aplikacja działała w systemie Windows w kompilacji” nowszej niż na tym komputerze albo system Windows uruchamiałby niektóre jej programy „przy użyciu emulacji”;
- że pliki z folderów Dokumenty lub Zapisane gry „trafią zamiast tego do folderu _external w lokalizacji” folderu instalacji, ponieważ odpowiedni folder na tym komputerze znajduje się poza profilem.

Następnie, na bursztynowo, każdy element pominięty przez reguły bezpieczeństwa wraz z przyczyną. Pierwszych 20 jest wyświetlanych w całości. Pozostałe są dostępne pod pozycją taką jak „Pominięto jeszcze 5 elementów”, którą można rozwinąć. Reszta instalacji przebiega bez nich. Co jest odrzucane i dlaczego, opisano na stronie [Co trafia do instalatora](What-Goes-in-an-Installer#what-the-installer-refuses).

Jeśli pakietu nie można w ogóle zainstalować w wybranej lokalizacji, czerwony komunikat informuje: „Tego pakietu nie można zainstalować w tej lokalizacji” i podaje przyczynę.

<a id="options"></a>

## Opcje

Ta karta pojawia się tylko wtedy, gdy instalator zawiera dane użytkownika lub dane licencji. Każde pole jest początkowo zaznaczone.

| Opcja | Opis na ekranie |
| --- | --- |
| **Dane użytkownika** | „Włączone: ustawienia, Dokumenty i Zapisane gry trafiają do standardowych folderów osoby, która jest zalogowana. Wyłączone: wszystkie te dane trafiają do folderu instalacji i nic nie jest konfigurowane do uruchamiania z nich.” Jeśli instalator podaje, czyje dane zawiera, wiersz pod opcją brzmi „Zawiera dane konta” i nazwa konta. |
| **Licencja** | „Pliki aktywacji i licencji oraz wartości rejestru przechowujące licencje. Niektóre mogą wymagać ponownej aktywacji na tym komputerze.” |

Zmiana pola od razu odpowiednio zmienia powyższą listę.

Dane użytkownika zawsze trafiają do **osoby zalogowanej** w systemie Windows, nawet jeśli w monicie UAC wpisano nazwę innego administratora. Jeśli instalator nie może ustalić, kto to jest, informuje o tym, pomija ustawienia poszczególnych użytkowników i zamiast zgadywać, umieszcza dane użytkownika w folderze instalacji. Jeśli w tej sesji nikt nie jest zalogowany, dane użytkownika i ustawienia poszczególnych użytkowników trafiają do konta, jako które działa instalator, o czym również informuje.

<a id="dependencies"></a>

## Zależności

„Czego ta aplikacja potrzebuje i jak zapewniany jest każdy z tych elementów.” Każde [środowisko uruchomieniowe](Glossary#runtime) ma etykietę:

| Etykieta | Znaczenie |
| --- | --- |
| **Dołączone** | W instalatorze. Instaluje się bez internetu, z wyjątkiem środowiska DirectX End-User Runtime, którego instalator pobiera je od firmy Microsoft. Spośród nich tylko środowisko uruchomieniowe Visual C++ x64 zostało naprawdę zainstalowane w testach projektu. |
| **Instalowane za pomocą winget:** i identyfikator | Nie udało się go pobrać podczas tworzenia instalatora, więc instalator używa narzędzia [winget](Glossary#winget), które wymaga internetu. Ta ścieżka nie była uruchamiana w testach projektu. |
| **Funkcja systemu Windows: NetFx3** | .NET Framework 3.5, włączany jako [funkcja systemu Windows](Glossary#windows-feature). Może to wymagać internetu. Ta ścieżka nie była uruchamiana w testach projektu. |
| **Wbudowane w system Windows** | .NET Framework 4, który systemy Windows 10 i 11 już mają |
| **Dostarczane z aplikacją** | Aplikacja zawiera własną kopię |
| **Zainstaluj ręcznie** | Nie ma tu dla niego instalatora. Zainstaluj je samodzielnie. |

Jeśli nie ma żadnych, wyświetlany jest tekst „Nie wykryto dodatkowych zależności od środowisk uruchomieniowych.” Zobacz [Środowiska uruchomieniowe](Runtimes).

<a id="preview-changes"></a>

## Podgląd zmian

Przycisk **Podgląd zmian** ustala dokładnie, co zrobiłaby instalacja, i niczego nie zmienia. Sprawdza, czy jest miejsce, rozpakowuje pakiet do prywatnego folderu, ustala zmiany i ponownie usuwa ten folder. Karta wyniku ma nagłówek „Podgląd: jeszcze niczego nie zmieniono”, zawiera kafelek **Planowane zmiany** oraz listę zmian, z usługami, zadaniami i skrótami na początku. Okno wyświetla pierwsze 300, a następnie podaje, ile jest pozostałych.

Podgląd nie sprawdza skrótu każdego pliku. Skrót całego pakietu został sprawdzony przy otwieraniu okna.

<a id="install"></a>

## Instalacja

Przycisk **Zainstaluj** pozostaje wyszarzony, dopóki nie można użyć lokalizacji, a lista nie zostanie ustalona dla tej lokalizacji. Jeśli pakietu nie można w ogóle zainstalować w tej lokalizacji, wiersz obok przycisku informuje: „Przycisk Zainstaluj jest niedostępny, ponieważ tego pakietu nie można zainstalować w tej lokalizacji”, a przyczyna jest podana na czerwono w karcie **Co ta instalacja konfiguruje oprócz plików**. Po kliknięciu przycisku:

1. Instalator jeszcze raz ustala listę. Jeśli to, co by zrobił, zmieniło się od czasu przejrzenia listy, niczego nie instaluje, wyświetla nową listę i informuje: „To, co ta instalacja wykonałaby w tej lokalizacji, uległo zmianie, więc niczego nie zainstalowano. Przejrzyj poniższą listę i jeśli wszystko się zgadza, ponownie kliknij przycisk Zainstaluj.”
2. Sprawdza, czy na każdym dysku, na którym zapisuje, jest miejsce.
3. Rozpakowuje pakiet do folderu, z którego mogą korzystać tylko administratorzy, sprawdzając skrót każdego rozpakowywanego pliku.
4. Instaluje, najpierw zapisując każdą zmianę w [dzienniku zmian](Glossary#journal), aby można ją było cofnąć.
5. Odczytuje każdy zainstalowany plik i sprawdza jego skrót.
6. Uruchamia usługi, które mają uruchamiać się automatycznie.

Na dole widać, co instalator robi, licznik, jeśli jest dostępny, czas trwania, pasek postępu i przycisk **Anuluj**. Możliwe kroki to: „Przygotowywanie…”, „Rozpakowywanie”, „Sprawdzanie pakietu”, „Cofanie wcześniejszej, nieukończonej instalacji”, „Instalowanie środowisk uruchomieniowych”, „Kopiowanie plików”, „Konfigurowanie usług”, „Konfigurowanie sterowników”, „Zapisywanie ustawień rejestru”, „Przywracanie danych licencji”, „Konfigurowanie zaplanowanych zadań”, „Tworzenie skrótów”, „Ustawianie zmiennych środowiskowych”, „Dodawanie aplikacji do Ustawień” i „Sprawdzanie zainstalowanych elementów”.

Przycisk **Anuluj** wyświetla „Anulowanie po zakończeniu bieżącego kroku…”, a następnie „Cofanie wprowadzonych zmian”. Zamknięcie okna podczas instalacji działa tak samo jak przycisk **Anuluj**: okno pozostaje otwarte, dopóki wszystko nie zostanie cofnięte. Jeśli jeszcze niczego nie zmieniono, wyświetlany jest tekst „Anulowano. Niczego nie zmieniono.”

<a id="the-hash-check"></a>

## Weryfikacja skrótów

Instalator sprawdza [skróty](Glossary#hash) trzy razy:

1. **Przy otwieraniu okna**: skrót całego pakietu. Obcięta kopia lub kopia zmieniona po utworzeniu jest odrzucana, zanim cokolwiek zostanie z niej odczytane.
2. **Podczas rozpakowywania**: skrót każdego pliku, zanim cokolwiek zostanie zapisane w docelowym miejscu.
3. **Po instalacji**: instalator odczytuje każdy zainstalowany plik i ponownie go sprawdza.

Skróty wykrywają uszkodzenia oraz zmiany wprowadzone przez kogoś, kto nie zaktualizował również skrótów. Nie dowodzą, kto utworzył instalator. Dowodzi tego tylko [podpis](Glossary#signature).

<a id="the-result"></a>

## Wynik

Karta wyniku pojawia się u góry, z nagłówkiem „Wynik instalacji”.

| Komunikat | Co oznacza |
| --- | --- |
| „Aplikacja została zainstalowana i pomyślnie przeszła weryfikację skrótów.” | Wszystko w porządku. Jeśli reguły coś pominęły, komunikat zawiera dodatkowo „bez elementów wymienionych poniżej jako pominięte”. |
| „Instalacja została anulowana, a wprowadzone zmiany zostały cofnięte.” | Instalacja została anulowana. |
| „Instalacja nie powiodła się, a wprowadzone zmiany zostały cofnięte.” | Coś się nie powiodło. Wprowadzone zmiany zostały cofnięte, z wyjątkiem zainstalowanych środowisk uruchomieniowych. To, co jest wymienione pod komunikatem na bursztynowo, zostało pozostawione, a przyczyna jest podana, na przykład usługa, która zniknie dopiero po ponownym uruchomieniu komputera. |
| „Instalacja nie powiodła się i nie wszystkie wprowadzone zmiany udało się cofnąć. Pozostałe elementy są wymienione poniżej. Następna instalacja spróbuje je cofnąć ponownie.” | Przeczytaj listę. Ponowne uruchomienie instalatora jeszcze raz spróbuje je cofnąć. |
| „Instalacja nie zakończyła się bez problemów. Przejrzyj poniższe problemy.” | Przeczytaj komunikaty pod nim. |

Następnie widać trzy kafelki: **Wprowadzone zmiany** (lub **Cofnięte zmiany** po niepowodzeniu), **Ostrzeżenia** i **Błędy**. Etykiety informują: **Skróty pakietu są prawidłowe** i **Zainstalowane pliki są prawidłowe** albo, na czerwono, **NIEPRAWIDŁOWE**. Do pięciu ostrzeżeń i wszystkie błędy są wyświetlane jako komunikaty. Komunikat „Jedna z zależności wymaga ponownego uruchomienia, aby zakończyć instalację. Uruchom ponownie system Windows, a następnie uruchom aplikację.” oznacza, że środowisko uruchomieniowe zażądało ponownego uruchomienia.

Przycisk **Zmiany** z liczbą otwiera listę wykonanych zmian, maksymalnie 300. Jeśli instalacja nie zakończyła się bez problemów, ścieżka jej [dziennika](Glossary#log) jest wyświetlana jako **Dziennik**.

Po pomyślnej instalacji przyciski na dole zmieniają się na **Zamknij** i **Otwórz folder instalacji**.

<a id="where-things-end-up"></a>

## Gdzie trafiają poszczególne elementy

- **Pliki programu** trafiają do lokalizacji instalacji.
- **Dane użytkownika**, gdy to pole jest zaznaczone, trafiają do folderów AppData, Dokumenty i Zapisane gry zalogowanego konta oraz do folderu ProgramData. Folder Dokumenty lub Zapisane gry, który ten komputer przechowuje poza profilem, nie jest zapisywany. Te pliki trafiają do lokalizacji instalacji, do folderu `_external`, a lista informuje o tym na początku. Gdy pole nie jest zaznaczone, wszystkie dane użytkownika trafiają do lokalizacji instalacji.
- **Skróty** trafiają na Pulpit i do menu Start tego komputera: bieżącego użytkownika albo wszystkich użytkowników, jeśli tam się wcześniej znajdowały.
- Do zmiennej **Path** są dodawane tylko wpisy aplikacji. Nic innego się w niej nie zmienia.
- **To, co inna aplikacja już tu ma, pozostaje jej własnością.** Typ pliku, typ linku, klasa COM, usługa, źródło dziennika zdarzeń, czcionka lub podobny element, który już należy do czegoś innego, pozostaje bez zmian, a instalator o tym informuje. Na przykład aplikację, która miała otwierać linki `http`, trzeba ponownie ustawić jako domyślną w Ustawieniach.
- W **Ustawieniach** pojawia się wpis aplikacji, dzięki czemu można ją odinstalować jak każdą inną. Zobacz [Cofanie instalacji](Undoing-an-Install).

<a id="common-reasons-it-refuses"></a>

## Najczęstsze przyczyny odmowy

| Co widać | Dlaczego | Co zrobić |
| --- | --- | --- |
| „ta aplikacja nie może działać na tym komputerze, więc niczego nie zmieniono” | Jej programy są przeznaczone dla innego rodzaju procesora albo wymagają nowszej wersji systemu Windows | Zainstaluj ją na komputerze podobnym do starego |
| „za mało miejsca, więc niczego nie zmieniono” | Za mało wolnego miejsca na dysku, na którym instalator zapisuje | Zwolnij miejsce lub wybierz lokalizację na innym dysku |
| „trwa inna instalacja lub dezinstalacja” | Jednocześnie może działać tylko jedna | Zaczekaj na jej zakończenie, a następnie spróbuj ponownie |
| Plik „ma nazwę, którą ta wersja systemu Windows otwiera jako urządzenie, a nie jako plik” | Aplikacja zawiera plik o nazwie podobnej do `aux.h`, którego wersje systemu Windows starsze niż Windows 11 nie mogą przechowywać | Zainstaluj aplikację w systemie Windows 11 |
| Folder danych, taki jak `%AppData%`, znajduje się pod linkiem | Instalator nie zapisuje przez link | Usuń zaznaczenie pól **Dane użytkownika** i **Licencja** i zainstaluj aplikację bez nich |

Więcej przyczyn opisano na stronie [Rozwiązywanie problemów](Troubleshooting#installing).
