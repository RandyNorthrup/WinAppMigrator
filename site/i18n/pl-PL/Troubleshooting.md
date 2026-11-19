<a id="troubleshooting"></a>

# Rozwiązywanie problemów

Znajdź poniżej to, co widzisz, według tego, co się dzieje, lub według słów na ekranie. Słowa w cudzysłowie to tekst wyświetlany przez program, czasem z nazwą lub liczbą w środku.

Jeśli nic tutaj nie pomoże, zobacz sekcję [Uzyskiwanie pomocy](#getting-help) na końcu strony.

<a id="the-window-could-not-open"></a>

## Nie można otworzyć okna

**Co widać.** Okno komunikatu zamiast okna programu:

- w programie: „Nie można otworzyć okna” oraz „Do wyświetlenia okna jest wymagany interfejs Direct3D 12, którego może nie obsługiwać stary sterownik karty graficznej lub maszyna wirtualna.”
- w instalatorze: to samo oraz „Instalacja jest możliwa również bez okna: uruchom ten plik w wierszu polecenia administratora z opcją --apply, a jeśli chcesz wybrać lokalizację, dodaj opcję --target-root i ścieżkę folderu.”
- w dezinstalatorze: to samo oraz „Dezinstalacja jest możliwa również bez okna, w wierszu polecenia administratora:” z dokładnym poleceniem.

**Dlaczego.** Okna są rysowane za pomocą interfejsu [Direct3D 12](Glossary#direct3d-12) i nie ma innego sposobu ich rysowania.

**Co zrobić.**

- Zaktualizuj sterownik karty graficznej komputera.
- W przypadku instalatora lub dezinstalatora użyj wiersza polecenia. Zobacz [Wiersz polecenia](Command-Line).
- Program nie ma wiersza polecenia. Utwórz instalator na innym komputerze: możesz podłączyć do niego dysk starego komputera. Zobacz [Przenoszenie z podłączonego dysku](Moving-from-a-Connected-Drive). Tamten komputer może też odczytać stary komputer przez sieć. Zobacz [Przenoszenie z komputera przez sieć](Moving-over-the-Network).

<a id="starting-the-app"></a>

## Uruchamianie programu

| Co widać | Co to oznacza | Co zrobić |
| --- | --- | --- |
| Monit UAC wyświetla dla programu „Nieznany wydawca” | Od wersji 0.5.0 program jest podpisany. Niepodpisany program nie jest plikiem wydania albo pochodzi sprzed wersji 0.5.0. | Pobierz go ponownie z [najnowszego wydania](https://github.com/RandyNorthrup/WinAppMigrator/releases/latest) i [sprawdź go](Getting-Started#check-the-download). |
| SmartScreen wyświetla „System Windows ochronił ten komputer” | Filtr SmartScreen rzadko spotykał się z tym plikiem | Wybierz **Więcej informacji**, sprawdź, czy wydawcą jest Randy Northrup, a następnie wybierz **Uruchom mimo to**. |
| Etykieta u góry wyświetla **Ładowanie nie powiodło się**, a lista „Nie można wyświetlić listy aplikacji” | Nie można odczytać listy aplikacji | Kliknij okrągłą strzałkę, aby ponownie załadować listę. W przypadku podłączonego dysku sprawdź, czy jest nadal podłączony. |
| Potrzebnej aplikacji nie ma na liście | Jest to aplikacja ze sklepu Microsoft Store, ukryta część innej aplikacji albo aplikacja, która w ogóle nie ma wpisu w Ustawieniach, na przykład aplikacja przenośna | Aplikacji ze sklepu Microsoft Store nie można przenosić. Ukryta część może zostać zaproponowana w sekcji **Może również należeć do aplikacji** po wybraniu głównej aplikacji. Aplikacji bez wpisu w Ustawieniach nie można wybrać. |
| „Brak pasujących aplikacji” | Wyszukiwanie lub filtr ukrywa wszystkie aplikacje | Wyczyść pole wyszukiwania i wybierz **Wszystkie**. |

<a id="building"></a>

## Tworzenie instalatora

<a id="picking-a-drive-or-a-disk-image"></a>

### Wybieranie dysku lub obrazu dysku

| Co widać | Co zrobić |
| --- | --- |
| Dysk „nie wygląda na dysk systemowy Windows (brak folderów \Windows i \Users). Wybierz katalog główny dysku z innego komputera.” | Wybierz najwyższy poziom dysku, na przykład `E:\`, a nie folder na nim. Sprawdź, czy jest to dysk, na którym był zainstalowany system Windows. |
| Obraz „nie jest obrazem dysku, który system Windows może dołączyć (.vhd lub .vhdx)” | Wybierz plik `.vhd` lub `.vhdx`. Dysk VMware lub VirtualBox najpierw zapisz w jednym z tych formatów. W przypadku punktu kontrolnego funkcji Hyper-V, `.avhdx`, wyeksportuj dysk maszyny wirtualnej. |
| „nie można dołączyć obrazu”, jego ścieżka i „system Windows ma już ten obraz dołączony, a to, co go dołączyło, może w nim zapisywać dane.” | Obraz jest otwarty gdzie indziej, na przykład zamontowany w Eksploratorze plików. Zamiast tego wybierz jego dysk za pomocą opcji **Podłączony dysk…** albo odłącz obraz i wybierz go ponownie. |
| Obraz „nie zawiera woluminu z systemem Windows, który można odczytać. Wolumin zaszyfrowany funkcją BitLocker trzeba najpierw odblokować.” | Sprawdź, czy jest to obraz dysku, na którym był zainstalowany system Windows. Jeśli jest zablokowany funkcją [BitLocker](Glossary#bitlocker), samodzielnie zamontuj go w systemie Windows, odblokuj i wybierz jego dysk za pomocą opcji **Podłączony dysk…**. |
| Obraz „jest dyskiem różnicowym, który zawiera tylko zmiany wprowadzone od utworzenia jego dysku nadrzędnego” | Scal go z dyskiem nadrzędnym za pomocą polecenia Edytuj dysk w Menedżerze funkcji Hyper-V i wybierz tamten dysk. |
| „nie można dołączyć obrazu”, jego ścieżka i inna przyczyna | Przyczynę podaje sam system Windows. Przeczytaj ją, sprawdź, czy nic innego nie korzysta z obrazu, i spróbuj ponownie. |

<a id="reading-a-pc-over-the-network"></a>

### Odczytywanie komputera przez sieć

Pełna lista, w tym komunikaty klienta na starym komputerze, znajduje się na stronie [Przenoszenie z komputera przez sieć](Moving-over-the-Network#if-something-goes-wrong).

| Co widać | Co zrobić |
| --- | --- |
| Starego komputera nie ma na liście w sekcji **2. Wybierz stary komputer** | Sprawdź, czy klient na nim działa i wyświetla kod oraz czy oba komputery są w tej samej sieci. Następnie wpisz jego nazwę lub adres. Jeśli komputery rozdziela router, wpisz adres. Zapora inna niż wbudowana Zapora systemu Windows musi przepuszczać ruch TCP 47732 i UDP 47731. |
| „nie znaleziono komputera”, jego nazwa i „w sieci” | Zamiast nazwy wpisz adres starego komputera. |
| „nie można połączyć się ze starym komputerem:” i przyczyna | Sprawdź, czy klient nadal działa oraz czy nazwa lub adres są poprawne. |
| „to nie jest kod wyświetlany na starym komputerze. Sprawdź go i spróbuj ponownie.” | Wpisz kod, który klient wyświetla teraz. Po pięciu błędnych kodach klient generuje nowy. Po 20 nie pozwala na żadne parowanie, dopóki nie zostanie ponownie uruchomiony. |
| Na starym komputerze system Windows informuje, że wydawca klienta jest nieznany | Jest to normalne tylko w systemie Windows 7 bez certyfikatu głównego firmy Microsoft z 2020 roku, który nie może sprawdzić podpisu klienta. Tam wybierz **Tak** tylko w przypadku klienta zapisanego samodzielnie z programu. W każdym innym przypadku monit powinien wskazywać Randy Northrup. Jeśli tak nie jest, nie uruchamiaj tej kopii. |
| Po długiej przerwie program nie może już odczytać starego komputera | Klient kończy połączenie, przez które program nie wysłał żadnego zapytania przez 30 minut. Kliknij **Rozłącz** i sparuj komputery ponownie. |

<a id="looking-at-the-app"></a>

### Analizowanie aplikacji

| Co widać | Co to oznacza | Co zrobić |
| --- | --- | --- |
| „Nie można przeanalizować tej aplikacji:” i przyczyna | Analiza się nie powiodła. | Kliknij **Spróbuj ponownie**. Jeśli analiza znów zakończy się tak samo, wpis może nie być aplikacją, którą WinAppMigrator może przenieść. |
| „już nie istnieje, choć zapis instalacji tej aplikacji wskazuje go jako jej program” | Wpis aplikacji na liście aplikacji wskazuje program, który został przeniesiony lub usunięty | Napraw aplikację na tym komputerze lub zainstaluj ją ponownie, a następnie wybierz ją jeszcze raz. |
| „Nie znaleziono własnego programu aplikacji”, jej nazwa i „więc nie ma nic do umieszczenia w instalatorze” | Wpis nie ma własnego programu, jak to często bywa w przypadku pakietu sterownika, takiego jak sterownik karty graficznej, a także środowiska uruchomieniowego lub dodatku | Na nowym komputerze zainstaluj sterownik od producenta lub z usługi Windows Update, a środowisko uruchomieniowe lub dodatek razem z aplikacją, która z niego korzysta. |
| „to aplikacja internetowa, którą przeglądarka zainstalowała z witryny internetowej” | Witryna internetowa zainstalowana jako aplikacja, taka jak Chrome Remote Desktop, bez własnego programu | Na nowym komputerze otwórz tę witrynę w tej samej przeglądarce i zainstaluj aplikację stamtąd. |
| „Zatrzymano. Instalator będzie można utworzyć po przeanalizowaniu tej aplikacji.” | Kliknięto przycisk **Zatrzymaj** | Kliknij **Analizuj ponownie**. |
| Analiza trwa długo | Duża aplikacja, wolny lub uszkodzony dysk albo wiele kont | Jeśli dysk może być uszkodzony, kliknij **Zatrzymaj**, utwórz [obraz dysku](Glossary#disk-image) i wybierz go zamiast dysku, ponieważ odczytywanie uszkodzonego dysku może go ostatecznie zniszczyć. Od wersji 0.6.0 wybierz go za pomocą opcji **Obraz dysku…**. W wersji 0.5.0 zamontuj go i wybierz jego dysk. W przeciwnym razie zaczekaj. W tym czasie możesz wybrać inną aplikację. |
| Potrzebny wiersz jest wyszarzony i zaznaczony | Zawiera główny program lub to, czego program potrzebuje do uruchomienia | Zawsze trafia do instalatora. Zobacz [wiersze, których nie można odznaczyć](Building-an-Installer#the-groups). |
| Wiersz zawiera tekst „Również pominięte:” | Pociągnął go za sobą element, którego zaznaczenie usunięto | Jeśli ten wiersz jest potrzebny, zaznacz ponownie tamten element. |
| „Z pozostałych elementów nie można utworzyć pakietu” | Po usunięciu zaznaczeń nie pozostało nic, z czego można utworzyć instalator | Zaznacz ponownie któryś element. |
| „Dane licencji z pominiętych elementów zostaną mimo to dołączone” | O licencjach decyduje wyłącznie opcja **Dane licencji i aktywacji** | Jeśli licencje mają zostać pominięte, usuń zaznaczenie tej opcji. |

<a id="create-installer-stays-greyed-out"></a>

### Przycisk Utwórz instalator pozostaje wyszarzony

Przycisk czeka, aż zostaną spełnione wszystkie poniższe warunki:

- analiza aplikacji została zakończona dla bieżących ustawień opcji (tekst „Ustalanie, co jest z tym powiązane…” oznacza, że program nadal ustala, co pociąga za sobą ostatnie usunięcie zaznaczenia);
- pole **Folder wyjściowy** zawiera pełną ścieżkę, na przykład `D:\Installers` („Wpisz pełną ścieżkę folderu, na przykład D:\Instalatory.”);
- na tym dysku jest wystarczająco dużo miejsca.

<a id="room"></a>

### Miejsce

| Co widać | Co zrobić |
| --- | --- |
| „Za mało miejsca.” i informacja, ile miejsca dysk potrzebuje, a ile ma | Wybierz folder wyjściowy na innym dysku lub pomiń niektóre elementy. Podana wartość to maksimum, jakiego może wymagać tworzenie instalatora przed kompresją, więc może być większa niż sam instalator. |
| Dysk „nie podaje, ile ma wolnego miejsca” | Tworzenie instalatora jest kontynuowane bez sprawdzania tego dysku. Upewnij się, że jest na nim miejsce. |
| Dysk FAT32 i duża aplikacja | System plików FAT32 nie może przechowywać pliku o rozmiarze 4 GiB. Użyj dysku NTFS lub exFAT. |

<a id="the-done-screen"></a>

### Ekran zakończenia

| Co widać | Co zrobić |
| --- | --- |
| „Instalator został utworzony, ale 2 pliki zniknęły od czasu analizy aplikacji” | Jeśli te pliki są ważne, kliknij **Analizuj ponownie**, a następnie **Utwórz instalator ponownie**. |
| „Instalator został utworzony, ale 2 pliki nie mogły zostać przechwycone, zwykle dlatego, że były zablokowane lub nieczytelne.” | Zamknij aplikację i wszystko inne, co korzysta z jej plików, a następnie ponownie utwórz instalator. Sekcja **Ostrzeżenia** wymienia te pliki. |
| „Instalator został utworzony, ale nadal zawiera pewne informacje o tym, od kogo pochodzi.” | Przeczytaj bursztynowe komunikaty pod nim. Zobacz [ostatni przegląd](Building-an-Installer#the-last-look). |
| Sekcja **Ostrzeżenia** zawiera „Nie można pobrać środowiska uruchomieniowego” z jego nazwą oraz „instalator spróbuje użyć narzędzia winget na komputerze docelowym.” | Tego środowiska uruchomieniowego nie ma w instalatorze, więc nowy komputer będzie potrzebował do niego internetu. Utwórz instalator ponownie z dostępem do internetu. Zobacz [Środowiska uruchomieniowe](Runtimes#at-build-time). |
| „Nie używaj tego instalatora: nie przeszedł weryfikacji skrótów.” | Nie używaj go. Utwórz instalator ponownie. Jeśli to się powtórzy, dysk może być uszkodzony. |
| „Instalator został zapisany, ale nie można było przeprowadzić weryfikacji jego skrótów” | Nie używaj go. Utwórz instalator ponownie. |
| „Anulowano tworzenie. Nie zapisano żadnego instalatora.” | Kliknięto przycisk **Anuluj**. |
| „Nie można utworzyć instalatora.” i przyczyna | Przeczytaj przyczynę. Przycisk **Kopiuj błąd** kopiuje ją, a **Dziennik** pokazuje, gdzie jest dziennik. |
| „Nie można utworzyć folderu wyjściowego” | Wybierz folder, w którym możesz zapisywać. |
| „Nie można utworzyć chronionego folderu przejściowego, więc niczego nie utworzono” | Nie udało się utworzyć prywatnego folderu roboczego obok folderu wyjściowego ani w folderze tymczasowym systemu Windows. Wybierz folder wyjściowy na innym dysku. |
| „Tworzenie nieoczekiwanie się zatrzymało” | Wystąpił błąd, którego nic nie przewidziało. „Jeśli w folderze wyjściowym pozostał instalator, nie używaj go.” Prosimy o [zgłoszenie](#getting-help). |

<a id="installing"></a>

## Instalowanie

<a id="when-the-installer-opens"></a>

### Gdy instalator się otwiera

| Co widać | Co to oznacza | Co zrobić |
| --- | --- | --- |
| „Sprawdzanie instalatora…” przez długi czas | Instalator odczytuje cały pakiet, aby sprawdzić jego skrót | Zaczekaj. Sprawdzanie dużego instalatora trwa chwilę. |
| „Nie można odczytać instalatora:” i „ten instalator jest uszkodzony lub został zmieniony po utworzeniu, więc niczego nie odczytano z jego pakietu. Skopiuj go ponownie z miejsca, w którym został utworzony.” | Plik jest obcięty lub zmieniony | Skopiuj go ponownie. |
| „ten instalator przechowuje swój pakiet w pliku o nazwie”, nazwa pliku `.wamdata` i „w tym samym folderze, a tego pliku tam nie ma” | W instalatorze składającym się z dwóch plików brakuje pliku danych | Skopiuj plik `.wamdata` do tego samego folderu. Zobacz [Duże aplikacje](Big-Apps). |
| Plik danych „nie jest plikiem danych tego instalatora albo jest uszkodzony lub został zmieniony po utworzeniu” | Plik danych nie pasuje | Skopiuj ponownie oba pliki z miejsca, w którym zostały utworzone. |
| „Pakietu tego instalatora nie można użyć na tym komputerze:” i „ta aplikacja nie może działać na tym komputerze, więc niczego nie zmieniono” | Programy aplikacji są przeznaczone dla innego procesora albo któryś program deklaruje, że wymaga nowszej wersji systemu Windows | Zainstaluj aplikację na komputerze podobnym do starego. Niczego nie zmieniono. |
| „To jest moduł startowy instalatora WinAppMigrator i nie zawiera on pakietu aplikacji.” | Jest to sam moduł startowy, a nie utworzony instalator | Utwórz instalator za pomocą programu i uruchom go. |
| „Podpis tego instalatora jest nieprawidłowy” z przyczyną podaną przez system Windows | Instalator został podpisany, ale system Windows nie akceptuje podpisu. Mógł zostać zmieniony po podpisaniu albo jego certyfikat nie jest zaufany na tym komputerze. | Nie instaluj go. Skontaktuj się z osobą, która go utworzyła. |
| Monit UAC wyświetla „Nieznany wydawca” | Instalator nie jest podpisany. To normalne w przypadku samodzielnie utworzonego instalatora. | Uruchamiaj go tylko wtedy, gdy ufasz jego źródłu. |

<a id="the-install-location"></a>

### Lokalizacja instalacji

Gdy przyciski **Zainstaluj** i **Podgląd zmian** są wyszarzone, czerwony komunikat pod polem **Lokalizacja instalacji** podaje przyczynę. Najczęstszą przyczyną jest to, że aplikacja jest już zainstalowana w tym folderze na tym komputerze, jak to często bywa z przeglądarką internetową, odtwarzaczem multimediów lub pakietem Office: najpierw odinstaluj tę kopię w Ustawieniach systemu Windows w sekcji **Aplikacje** i ponownie otwórz instalator albo wpisz nowy, pusty folder. Przyczyny i sposoby postępowania opisano na stronie [Instalowanie na nowym komputerze](Installing-on-the-New-PC#when-the-location-cannot-be-used).

<a id="before-or-during-the-install"></a>

### Przed instalacją lub w jej trakcie

| Co widać | Co to oznacza | Co zrobić |
| --- | --- | --- |
| „W tej lokalizacji może brakować miejsca” | Na dysku może brakować miejsca | Zwolnij miejsce lub wybierz lokalizację na innym dysku. |
| „za mało miejsca, więc niczego nie zmieniono” | Dysk, na którym instalator zapisuje, jest zbyt zapełniony | Zwolnij na nim miejsce lub wybierz lokalizację instalacji na innym dysku. |
| „To, co ta instalacja wykonałaby w tej lokalizacji, uległo zmianie, więc niczego nie zainstalowano.” | Coś na komputerze zmieniło się od czasu ustalenia listy | Przejrzyj listę ponownie i jeśli wszystko się zgadza, ponownie kliknij **Zainstaluj**. |
| „Wcześniejsza instalacja aplikacji”, jej nazwa i „nie została ukończona. Instalacja najpierw ją cofnie.” | Poprzednia instalacja została przerwana | Kliknij **Zainstaluj**. Instalacja najpierw cofnie tamtą. |
| „trwa inna instalacja lub dezinstalacja. Zaczekaj na jej zakończenie, a następnie spróbuj ponownie” | Jednocześnie może działać tylko jedna | Zaczekaj, a następnie spróbuj ponownie. |
| Plik „ma nazwę, którą ta wersja systemu Windows otwiera jako urządzenie, a nie jako plik” | Aplikacja zawiera plik o nazwie podobnej do `aux.h`, którego wersje systemu Windows starsze niż Windows 11 nie mogą przechowywać | Zainstaluj aplikację w systemie Windows 11. |
| „Nie można utworzyć folderu przejściowego” | Nie ma prywatnego miejsca z wystarczającą ilością wolnego miejsca do rozpakowania pakietu | Zwolnij miejsce na dysku lokalizacji instalacji lub na dysku systemowym. |
| Folder danych, taki jak `%AppData%`, jest określony jako link | Instalator nie zapisuje przez [link](Glossary#link) | Usuń zaznaczenie pól **Dane użytkownika** i **Licencja** i zainstaluj aplikację bez nich. |
| „Nie można odnaleźć folderu ProgramData na tym komputerze, więc niczego nie zainstalowano” | Nie można odczytać zapisu systemu Windows wskazującego, gdzie znajduje się folder ProgramData | Instalator niczego nie zainstaluje bez bezpiecznego miejsca na przechowanie swojego zapisu. |

<a id="the-result"></a>

### Wynik

| Co widać | Co zrobić |
| --- | --- |
| „Instalacja nie powiodła się, a wprowadzone zmiany zostały cofnięte.” | Przeczytaj czerwone komunikaty, aby poznać przyczynę. Komputer jest w takim stanie jak wcześniej, z wyjątkiem zainstalowanych środowisk uruchomieniowych. |
| „Instalacja nie powiodła się i nie wszystkie wprowadzone zmiany udało się cofnąć.” | Przeczytaj, co zostało wymienione. Zamknij programy, które mogą korzystać z tych plików, i ponownie uruchom instalator: najpierw ponowi on próbę cofnięcia zmian. |
| „Nie można zainstalować środowiska uruchomieniowego” i jego nazwa | Nie udało się zainstalować środowiska uruchomieniowego, więc instalacja została cofnięta. W przypadku środowiska DirectX End-User Runtime nowy komputer potrzebuje internetu. Zobacz [Środowiska uruchomieniowe](Runtimes#with-no-internet-on-the-new-pc). |
| „Jedna z zależności wymaga ponownego uruchomienia, aby zakończyć instalację. Uruchom ponownie system Windows, a następnie uruchom aplikację.” | Uruchom ponownie komputer, a następnie uruchom aplikację. |
| Usługa „działała na komputerze źródłowym jako” konto. „Jej hasła nie ma w instalatorze, więc usługa została skonfigurowana do działania jako LocalService.” | Jeśli usługa nie działa w obecnej postaci, otwórz konsolę Usługi i ustaw konto oraz hasło usługi. |
| Niektóre pliki „trafią zamiast tego do folderu _external w lokalizacji” folderu instalacji | Ten komputer przechowuje folder Dokumenty lub Zapisane gry poza profilem. Przenieś te pliki samodzielnie do tego folderu. |
| „Pominięto element” z podaną przyczyną | Reguły go odrzuciły albo element już należy tu do innej aplikacji. Reszta została zainstalowana. Jeśli aplikacja go potrzebuje, zapoznaj się z przyczyną. |
| **Skróty pakietu są NIEPRAWIDŁOWE** lub **Zainstalowane pliki są NIEPRAWIDŁOWE** | Coś się nie zgadzało. Skopiuj instalator ponownie i spróbuj jeszcze raz. |

<a id="after-installing"></a>

### Po instalacji

| Co widać | Co zrobić |
| --- | --- |
| Aplikacja się nie uruchamia | Sprawdź w sekcji **Zależności**, czy któreś środowisko uruchomieniowe nie jest oznaczone jako **Zainstaluj ręcznie**, i zainstaluj je. Sprawdź ostrzeżenia w wyniku. |
| Aplikacja z systemu Windows 7 się nie uruchamia, a ostrzeżenie podczas tworzenia instalatora wskazało bibliotekę DLL, o której napisano, że „systemy Windows 10 i 11 go nie zawierają” | Ta biblioteka DLL została umieszczona w folderach starego systemu Windows przez coś innego, więc nie została przeniesiona. Zainstaluj to, co ją dostarcza, a następnie ponownie uruchom aplikację. Zobacz [Aplikacje z systemów Windows 7, Vista, 8 lub 8.1](Moving-from-a-Connected-Drive#apps-from-windows-7-vista-8-or-81). |
| Aplikacja uruchamia się jak nowa | To normalne w przypadku instalatora utworzonego z wyłączoną opcją **Dane użytkownika**. Dzieje się tak również wtedy, gdy podczas instalacji usunięto zaznaczenie pola **Dane użytkownika**, ponieważ dane trafiają wtedy do folderu instalacji, oraz wtedy, gdy instalator nie mógł ustalić, kto jest zalogowany, o czym informuje. |
| Aplikacja prosi o aktywację | Licencja powiązana ze starym komputerem wymaga ponownej aktywacji. |
| Własny program aktualizujący aplikacji uważa, że nie jest ona zainstalowana | Instalator Windows nie ma na tym komputerze zapisu o niej. Zobacz [Ograniczenia](Limitations). |
| Typ pliku lub typ linku nie otwiera się w aplikacji | Na tym komputerze należał już do innej aplikacji, więc pozostał bez zmian. Wybierz aplikację w Ustawieniach, w sekcji Aplikacje domyślne. |
| Nowy wpis zmiennej `Path` nie jest widoczny | Otwórz nowe okno programu PowerShell lub wiersza polecenia. Niektóre programy widzą zmianę dopiero po wylogowaniu się i ponownym zalogowaniu. |

<a id="uninstalling"></a>

## Odinstalowywanie

| Co widać | Co zrobić |
| --- | --- |
| „Część aplikacji znajduje się na dysku, który nie jest podłączony.” | Podłącz ten dysk i ponownie odinstaluj aplikację w Ustawieniach. |
| „Nie udało się cofnąć wszystkiego. Zamknij programy, które korzystają z tych elementów, i ponownie odinstaluj aplikację w Ustawieniach.” | Zamknij aplikację i wszystko, co korzysta z jej plików. Jeśli usługa nie chce się zatrzymać, uruchom ponownie komputer. Następnie ponownie odinstaluj aplikację. |
| „this is not an install's id” | Identyfikator podany w opcji `--uninstall` jest nieprawidłowy. Skopiuj go z wpisu w Ustawieniach. Zobacz [Wiersz polecenia](Command-Line#finding-the-id). |
| Pojawił się plik o nazwie `.before-install` | Jest to oryginał pliku zastąpionego przez instalację, który został od tego czasu zmieniony. Zachowaj tę wersję, którą chcesz. |
| Środowisko uruchomieniowe jest nadal zainstalowane | Środowiska uruchomieniowe pozostają, ponieważ mogą z nich korzystać inne programy. Pozostaw je, chyba że masz pewność, że nic innego z nich nie korzysta. |

<a id="scripts"></a>

## Skrypty

| Co widać | Co zrobić |
| --- | --- |
| „Żądana operacja wymaga podniesienia uprawnień” | Uruchom program PowerShell jako administrator. |
| Nic nie jest wyświetlane, a zmienna `$LASTEXITCODE` nie jest ustawiona | Przekaż dane wyjściowe instalatora potokiem, na przykład do polecenia `Out-String`. Zobacz [Wiersz polecenia](Command-Line). |
| Kod zakończenia `2` | Opcja jest nieprawidłowa albo użyto jednocześnie opcji `--apply` i `--dry-run`. |
| „this package does not say where its app was, so give a --target-root” (wiersz polecenia instalatora zawsze odpowiada po angielsku) | Dodaj opcję `--target-root` z pełną ścieżką. |
| „this package's app was in”, nazwa folderu i „and that cannot be used” | Dodaj opcję `--target-root` z innym folderem. |
| Ścieżka folderu, „cannot be used: it already has files” i „Give --force only if you know it is this app's” | Wybierz pusty folder. Używaj opcji `--force` tylko w przypadku folderu, o którym wiesz, że należy do tej aplikacji. |

<a id="language-and-text"></a>

## Język i tekst

- **Niewłaściwy język.** Wybierz inny w menu **Język** w prawym górnym rogu programu lub pierwszego ekranu instalatora. Program zapamiętuje wybór; instalator zachowuje go na czas tego uruchomienia. Jeśli nie dokonano wyboru, każdy z nich używa pierwszego z języków wyświetlania systemu Windows, który obsługuje, a w przeciwnym razie angielskiego.
- **Tłumaczenie brzmi źle.** Wszystkie języki oprócz angielskiego zostały przetłumaczone maszynowo. W menu **Język** wybierz **Zgłoś problem z tłumaczeniem**. Otworzy się formularz z wypełnionym już językiem. Napisz, co wyświetla ekran i co powinien wyświetlać.
- **Litery są wyświetlane jako puste prostokąty.** Okno używa czcionek dostarczanych z systemem Windows dla każdego pisma, z folderu `C:\Windows\Fonts`. System Windows, z którego usunięto niektóre z nich, wyświetla te litery jako prostokąty; mogą ich też wymagać nazwy innych aplikacji na liście.
- **Słowa są obcięte.** Etykieta, która się nie mieści, kończy się znakiem `…`; wskaż ją myszą lub przejdź do niej klawiszem Tab, aby zobaczyć całość. Zgłoś to, podając język i ekran, aby można było dopasować okno.
- **Program WinAppMigrator Client wyświetla tekst po angielsku w systemie Windows z pismem od prawej do lewej.** Konsola, w której działa, nie potrafi łączyć ani porządkować liter arabskich i hebrajskich, więc klient wyświetla tekst po angielsku i wyjaśnia dlaczego. Opcja `--lang` wybiera dowolny z obsługiwanych języków.

<a id="getting-help"></a>

## Uzyskiwanie pomocy

1. Znajdź [dziennik](Glossary#log). Jego ścieżka jest wyświetlana jako **Dziennik** w razie niepowodzenia albo wypisywana jako `Log:` w wierszu polecenia. Dzienniki znajdują się w folderze `%ProgramData%\WinAppMigrator\logs`. Zawierają ścieżki i nazwy kluczy, ale nigdy zawartości ustawień, plików licencji ani zmiennych środowiskowych. Mogą jednak zawierać nazwy folderów, więc przeczytaj dziennik, zanim go udostępnisz.
2. [Otwórz zgłoszenie](https://github.com/RandyNorthrup/WinAppMigrator/issues), podając wersję, wykonane czynności i komunikat wyświetlony na ekranie. Przycisk **Kopiuj błąd** na ekranie zakończenia programu kopiuje komunikat.
3. **Nigdy nie dołączaj instalatora do zgłoszenia.** Może zawierać prywatne dane.
4. Problem z zabezpieczeniami zgłoś prywatnie, za pomocą [porad dotyczących zabezpieczeń w serwisie GitHub](https://github.com/RandyNorthrup/WinAppMigrator/security/advisories/new).
