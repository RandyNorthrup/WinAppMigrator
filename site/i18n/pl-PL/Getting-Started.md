<a id="getting-started"></a>

# Pierwsze kroki

Ta strona prowadzi od pobrania programu do jego pierwszego ekranu.

<a id="what-you-need"></a>

## Wymagania

- **Windows 10 lub Windows 11, wersja 64-bitowa.** Program jest aplikacją x64. Windows 11 na urządzeniach Arm uruchamia go przy użyciu [emulacji](Glossary#emulation). Windows 10 na urządzeniach Arm nie może uruchamiać programów x64, a 32-bitowy system Windows nie może go uruchomić w ogóle. Klienckie wersje systemów Windows 10 i 11 na platformie x64 nie były uruchamiane w testach projektu, w których używane są systemy Windows Server 2022 i 2025 oraz Windows 11 na urządzeniach Arm. Zobacz [Wymagania i testy](Requirements-and-Testing).
- **Uprawnienia administratora** na komputerze. Program zawsze prosi o nie przy uruchamianiu. Zobacz [Uprawnienia administratora](Glossary#administrator-rights).
- **Direct3D 12**, którego system Windows używa do rysowania okna programu. Systemy Windows 10 i 11 zwykle go mają. Stary sterownik karty graficznej lub maszyna wirtualna mogą go nie obsługiwać. Zobacz [Direct3D 12](Glossary#direct3d-12).
- **Miejsce na dysku** na tworzony instalator. Program poinformuje, ile go potrzeba, zanim zaczniesz.
- **Internet na starym komputerze jest opcjonalny.** Jeśli podczas tworzenia instalatora jest dostępny, program pobiera [środowiska uruchomieniowe](Glossary#runtime) firmy Microsoft, które może dołączyć, i umieszcza je w instalatorze. Zobacz [Środowiska uruchomieniowe](Runtimes).

Więcej szczegółów, w tym informacje o tym, co zostało przetestowane, a co nie, znajdziesz na stronie [Wymagania i testy](Requirements-and-Testing).

<a id="download"></a>

## Pobieranie

Przejdź do [najnowszego wydania](https://github.com/RandyNorthrup/WinAppMigrator/releases/latest). Zawiera ono trzy pliki:

| Plik | Co to jest | Wybierz, jeśli |
| --- | --- | --- |
| `WinAppMigrator-v0.7.2.exe` | Sam program. Jeden plik, którego nie trzeba instalować. | Chcesz po prostu z niego korzystać. Tego pliku potrzebuje większość osób. |
| `WinAppMigrator-v0.7.2-windows-x64.zip` | Ten sam program jako `winappmigrator.exe`, z plikiem README, listą zmian, licencją i licencjami innych firm | Chcesz mieć również dokumenty towarzyszące |
| `SHA256SUMS.txt` | Sumy kontrolne dwóch powyższych plików | Chcesz sprawdzić pobrany plik |

Część umieszczana w każdym instalatorze, czyli [moduł startowy instalatora](Glossary#installer-stub), nigdy nie jest publikowana osobno. Jest już zawarta w programie.

<a id="check-the-download"></a>

## Sprawdzanie pobranego pliku

Ten krok jest opcjonalny, ale warto go wykonać. Każde z poniższych sprawdzeń jest niezależne od pozostałych. Uruchom je w programie PowerShell, w folderze, w którym zapisano plik.

**1. Suma kontrolna.** [Skrót](Glossary#hash) to odcisk palca pliku. Jeśli zmieni się choćby jeden bajt, zmieni się również skrót.

```powershell
Get-FileHash .\WinAppMigrator-v0.7.2.exe -Algorithm SHA256
Get-Content .\SHA256SUMS.txt
```

Długa liczba wyświetlona przez polecenie `Get-FileHash` powinna być taka sama jak liczba w wierszu tego pliku. Polecenie `Get-FileHash` wyświetla wielkie litery, a plik zawiera małe. To nie problem: porównaj litery i cyfry.

**2. Podpis.** [Podpis](Glossary#signature) informuje, kto utworzył plik i że plik od tego czasu się nie zmienił.

```powershell
$signature = Get-AuthenticodeSignature .\WinAppMigrator-v0.7.2.exe
$signature.Status                     # should say Valid
$signature.SignerCertificate.Subject  # should include CN=Randy Northrup
```

Możesz też kliknąć plik prawym przyciskiem myszy, wybrać polecenie **Właściwości** i otworzyć kartę **Podpisy cyfrowe**. Wersja 0.5.0 to pierwsze wydanie, w którym program jest podpisany. Pliki pobrane z wcześniejszych wydań nie są podpisane.

**3. Miejsce utworzenia.** Każde wydanie od wersji 0.5.0 zawiera zapis potwierdzający, że zostało utworzone przez własny przepływ pracy wydań serwisu GitHub, z tego repozytorium. Taki zapis nazywa się [atestacją](Glossary#attestation). Jeśli masz narzędzie [GitHub CLI](https://cli.github.com/), uruchom:

```powershell
gh attestation verify .\WinAppMigrator-v0.7.2.exe --repo RandyNorthrup/WinAppMigrator
```

Tak samo można sprawdzić plik zip.

<a id="start-it"></a>

## Uruchamianie programu

1. **Kliknij dwukrotnie** plik `WinAppMigrator-v0.7.2.exe`. Nie trzeba niczego instalować. Program działa z miejsca, w którym go zapisano.
2. **SmartScreen.** Filtr [SmartScreen](Glossary#smartscreen) systemu Windows może wyświetlić ekran „System Windows ochronił ten komputer” w przypadku pliku, z którym rzadko się spotykał. Jeśli tak się stanie, wybierz **Więcej informacji** i sprawdź, czy wydawcą jest Randy Northrup, zanim wybierzesz **Uruchom mimo to**.
3. **Monit administratora.** System Windows pyta, czy zezwolić programowi na wprowadzanie zmian. Jest to [monit Kontroli konta użytkownika](Glossary#uac-prompt). Program zawsze o to prosi, ponieważ odczyt dysku innego komputera i plików, które mogą odczytywać tylko administratorzy, wymaga uprawnień administratora. W przypadku podpisanego pliku do pobrania monit wskazuje **Randy Northrup** jako zweryfikowanego wydawcę. Jeśli dla tego pliku wyświetla „Nieznany wydawca”, nie jest to podpisany plik wydania, więc go nie uruchamiaj.
4. **Otwiera się okno.** Jeśli zamiast tego pojawi się komunikat, że nie można otworzyć okna i że wymagany jest Direct3D 12, zobacz [Rozwiązywanie problemów](Troubleshooting#the-window-could-not-open). Program nie może działać bez swojego okna.

<a id="first-run"></a>

## Pierwsze uruchomienie

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/app-pick-dark.png">
    <img alt="Program z wybraną aplikacją Fabrikam Data Sync" src="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/app-pick-light.png" width="860">
  </picture>
</p>

- Lista zainstalowanych aplikacji znajduje się po lewej stronie. Podczas jej ładowania mała etykieta w prawym górnym rogu wyświetla **Ładowanie**, a następnie liczbę znalezionych aplikacji, na przykład **5 aplikacji**.
- Okno otwiera się w trybie jasnym lub ciemnym, zależnie od trybu systemu Windows. Przycisk ze słońcem lub księżycem w prawym górnym rogu przełącza tryb, a program zapamiętuje wybór. Zobacz [Pliki przechowywane na komputerze](Files-It-Keeps-on-a-PC). W motywie kontrastu systemu Windows program używa kolorów motywu i nie ma tego przycisku.
- Program używa języka wyświetlania systemu Windows, jeśli jest to jeden z jego 24 języków, a w przeciwnym razie angielskiego. Menu **Język** w prawym górnym rogu wyświetla języki pod ich własnymi nazwami i od razu zmienia język okna; program zapamiętuje wybór. Okna w języku arabskim, perskim, urdu i hebrajskim mają układ od prawej do lewej. Zobacz [Języki i ułatwienia dostępu](Languages-and-Accessibility).
- Kliknij aplikację. Po prawej stronie widać, co trafiłoby do jej instalatora, a program od razu zaczyna to ustalać. Nic nie zostanie utworzone, dopóki nie klikniesz **Utwórz instalator**.

Dalej: [Tworzenie instalatora](Building-an-Installer).
