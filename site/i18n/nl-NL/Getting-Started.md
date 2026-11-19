<a id="getting-started"></a>

# Aan de slag

Deze pagina brengt je van de download tot het eerste scherm van de app.

<a id="what-you-need"></a>

## Wat je nodig hebt

- **Windows 10 of Windows 11, 64-bits.** De app is een x64-programma. Windows 11 op Arm voert het uit via [emulatie](Glossary#emulation). Windows 10 op Arm kan geen x64-programma's uitvoeren, en 32-bits Windows kan het helemaal niet uitvoeren. De desktopversies van Windows 10 en 11 op x64 zijn niet uitgevoerd in de tests van het project, die Windows Server 2022 en 2025 en Windows 11 op Arm gebruiken. Zie [Vereisten en tests](Requirements-and-Testing).
- **Administratorrechten** op de pc. De app vraagt er altijd om wanneer deze start. Zie [Administratorrechten](Glossary#administrator-rights).
- **Direct3D 12**, waarmee Windows het venster van de app tekent. Windows 10 en 11 hebben dit normaal gesproken. Een oud grafisch stuurprogramma of een virtuele machine heeft het mogelijk niet. Zie [Direct3D 12](Glossary#direct3d-12).
- **Ruimte op een station** voor het installatieprogramma dat je maakt. De app laat je vooraf weten hoeveel.
- **Internet op de oude pc is optioneel.** Als er internet is wanneer je het installatieprogramma maakt, downloadt de app de Microsoft-[runtimes](Glossary#runtime) die erin kunnen en plaatst deze in het installatieprogramma. Zie [Runtimes](Runtimes).

Meer details, en wat wel en niet is getest, staan in [Vereisten en tests](Requirements-and-Testing).

<a id="download"></a>

## Downloaden

Ga naar de [nieuwste release](https://github.com/RandyNorthrup/WinAppMigrator/releases/latest). Die heeft drie bestanden:

| Bestand | Wat het is | Neem het als |
| --- | --- | --- |
| `WinAppMigrator-v0.7.3.exe` | De app zelf. Eén bestand, er hoeft niets te worden geïnstalleerd. | Je de app gewoon wilt gebruiken. Dit is het bestand dat de meeste mensen willen. |
| `WinAppMigrator-v0.7.3-windows-x64.zip` | Dezelfde app, als `winappmigrator.exe`, met de README, het wijzigingslogboek, de licentie en de licenties van derden | Je ook de documenten erbij wilt |
| `SHA256SUMS.txt` | De controlesommen van de twee bestanden hierboven | Je je download wilt controleren |

Het deel dat in elk installatieprogramma zit, de [installatiestub](Glossary#installer-stub), wordt nooit apart gepubliceerd. Het zit al in de app.

<a id="check-the-download"></a>

## De download controleren

Dit is optioneel, maar de moeite waard. Elke controle hieronder staat op zichzelf. Voer ze uit in PowerShell, in de map waarin je het bestand hebt opgeslagen.

**1. De controlesom.** Een [hash](Glossary#hash) is een vingerafdruk van een bestand. Als er ook maar één byte verandert, verandert de hash.

```powershell
Get-FileHash .\WinAppMigrator-v0.7.3.exe -Algorithm SHA256
Get-Content .\SHA256SUMS.txt
```

Het lange getal van `Get-FileHash` moet overeenkomen met dat op de regel voor dat bestand. `Get-FileHash` drukt hoofdletters af en het bestand gebruikt kleine letters. Dat is geen probleem: vergelijk de letters en cijfers.

**2. De handtekening.** Een [handtekening](Glossary#signature) laat zien wie het bestand heeft gemaakt, en dat het sindsdien niet is gewijzigd.

```powershell
$signature = Get-AuthenticodeSignature .\WinAppMigrator-v0.7.3.exe
$signature.Status                     # should say Valid
$signature.SignerCertificate.Subject  # should include CN=Randy Northrup
```

Je kunt ook met de rechtermuisknop op het bestand klikken, **Eigenschappen** kiezen en naar het tabblad **Digitale handtekeningen** kijken. Versie 0.5.0 is de eerste release waarvan de app is ondertekend. Downloads van eerdere releases zijn niet ondertekend.

**3. Waar het is gemaakt.** Van elke release vanaf 0.5.0 is vastgelegd dat de eigen releaseworkflow van GitHub deze heeft gemaakt, vanuit deze repository. Die vastlegging heet een [attestatie](Glossary#attestation). Als je de [GitHub CLI](https://cli.github.com/) hebt, voer je dit uit:

```powershell
gh attestation verify .\WinAppMigrator-v0.7.3.exe --repo RandyNorthrup/WinAppMigrator
```

Hetzelfde werkt voor het zip-bestand.

<a id="start-it"></a>

## Starten

1. **Dubbelklik** op `WinAppMigrator-v0.7.3.exe`. Er hoeft niets te worden geïnstalleerd. De app wordt uitgevoerd vanaf de plek waar je deze hebt opgeslagen.
2. **SmartScreen.** Windows [SmartScreen](Glossary#smartscreen) kan het scherm "Windows heeft uw pc beschermd" weergeven voor een bestand dat het niet vaak heeft gezien. Kies in dat geval **Meer informatie** en controleer of de uitgever Randy Northrup is voordat je **Toch uitvoeren** kiest.
3. **De vraag om administratorrechten.** Windows vraagt of de app wijzigingen mag aanbrengen. Dit is de [UAC-prompt](Glossary#uac-prompt) van Gebruikersaccountbeheer. De app vraagt het altijd, omdat voor het lezen van het station van een andere pc, en van bestanden die alleen administrators mogen lezen, administratorrechten nodig zijn. Bij de ondertekende download noemt de prompt **Randy Northrup** als geverifieerde uitgever. Als er voor dit bestand "Onbekende uitgever" staat, is het niet het ondertekende releasebestand. Voer het dan niet uit.
4. **Het venster wordt geopend.** Als je in plaats daarvan een melding ziet dat het venster niet kan worden geopend en dat Direct3D 12 nodig is, kijk dan in [Problemen oplossen](Troubleshooting#the-window-could-not-open). De app kan niet zonder venster worden uitgevoerd.

<a id="first-run"></a>

## De eerste keer

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/app-pick-dark.png">
    <img alt="De app met Fabrikam Data Sync geselecteerd" src="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/app-pick-light.png" width="860">
  </picture>
</p>

- De lijst met geïnstalleerde apps staat links. Tijdens het laden staat in het kleine label rechtsboven **Laden** en daarna hoeveel apps er zijn gevonden, zoals **5 apps**.
- Het venster start in de lichte of donkere modus, net als Windows. De knop met de zon of de maan rechtsboven schakelt tussen beide, en de app onthoudt je keuze. Zie [Bestanden die het op een pc bewaart](Files-It-Keeps-on-a-PC). In een contrastthema van Windows gebruikt de app de kleuren van het thema en is er geen knop.
- De app gebruikt de weergavetaal van Windows als dat een van de 24 talen van de app is, en anders Engels. **Taal**, rechtsboven, toont de talen in hun eigen naam en wijzigt het venster meteen; de app onthoudt je keuze. Arabisch, Perzisch, Urdu en Hebreeuws worden van rechts naar links weergegeven. Zie [Talen en toegankelijkheid](Languages-and-Accessibility).
- Klik op een app. Rechts zie je wat er in het installatieprogramma zou gaan, en de app begint dat meteen te bepalen. Er wordt niets gemaakt totdat je op **Installatieprogramma maken** klikt.

Volgende: [Een installatieprogramma maken](Building-an-Installer).
