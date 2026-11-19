<a id="troubleshooting"></a>

# Problemen oplossen

Zoek hieronder wat je ziet, op wat er gebeurt of op de woorden op het scherm. Woorden tussen aanhalingstekens zijn wat het programma toont, soms met een naam of een getal ertussen. Wat een opdrachtregel afdrukt, staat altijd in het Engels en is hier ook zo weergegeven.

Als niets hier helpt, kijk dan bij [Hulp krijgen](#getting-help) aan het eind.

<a id="the-window-could-not-open"></a>

## Het venster kan niet worden geopend

**Wat je ziet.** Een berichtvenster in plaats van het venster:

- van de app: "Het venster kan niet worden geopend", en "Dit programma heeft Direct3D 12 nodig om het venster te tekenen. Een oud grafisch stuurprogramma of een virtuele machine heeft dat mogelijk niet."
- van een installatieprogramma: hetzelfde, en "Installeren kan ook zonder venster: voer dit bestand uit vanaf een opdrachtprompt als administrator met --apply, en met --target-root gevolgd door een map om zelf de locatie te kiezen."
- van een verwijderprogramma: hetzelfde, en "Verwijderen kan ook zonder venster, vanaf een opdrachtprompt als administrator:" met de exacte opdracht.

**Waarom.** De vensters worden getekend met [Direct3D 12](Glossary#direct3d-12), en er is niets anders om ze mee te tekenen.

**Wat je kunt doen.**

- Werk het grafische stuurprogramma van de pc bij.
- Gebruik voor een installatieprogramma of een verwijderprogramma de opdrachtregel. Zie [Opdrachtregel](Command-Line).
- De app heeft geen opdrachtregel. Maak het installatieprogramma op een andere pc: je kunt het station van de oude pc op die pc aansluiten. Zie [Verplaatsen vanaf een aangesloten station](Moving-from-a-Connected-Drive). Die pc kan de oude pc ook via het netwerk lezen. Zie [Verplaatsen vanaf een pc via het netwerk](Moving-over-the-Network).

<a id="starting-the-app"></a>

## De app starten

| Wat je ziet | Wat het betekent | Wat je kunt doen |
| --- | --- | --- |
| De UAC-prompt meldt "Onbekende uitgever" voor de app | Vanaf 0.5.0 is de app ondertekend. Een niet-ondertekende app is niet het releasebestand, of is van vóór 0.5.0. | Download de app opnieuw van de [nieuwste release](https://github.com/RandyNorthrup/WinAppMigrator/releases/latest) en [controleer deze](Getting-Started#check-the-download). |
| SmartScreen meldt "Windows heeft uw pc beschermd" | SmartScreen heeft dat bestand niet vaak gezien | Kies **Meer informatie**, controleer of de uitgever Randy Northrup is en kies daarna **Toch uitvoeren**. |
| Het label bovenaan meldt **Laden mislukt** en de lijst meldt "Kan de apps niet weergeven" | De lijst met apps kan niet worden gelezen | Klik op de ronde pijl om opnieuw te laden. Controleer bij een aangesloten station of het nog is aangesloten. |
| De app die je wilt, staat niet in de lijst | Het is een Microsoft Store-app, een verborgen onderdeel van een andere app, of de app heeft helemaal geen vermelding in Instellingen, zoals een draagbare app | Store-apps kunnen niet worden verplaatst. Een verborgen onderdeel wordt mogelijk aangeboden onder **Hoort mogelijk ook bij de app** wanneer je de hoofdapp kiest. Een app zonder vermelding in Instellingen kan niet worden gekozen. |
| "Geen overeenkomende apps" | De zoekopdracht of het filter verbergt alles | Maak het zoekvak leeg en kies **Alle**. |

<a id="building"></a>

## Het installatieprogramma maken

<a id="picking-a-drive-or-a-disk-image"></a>

### Een station of een schijfkopie kiezen

| Wat je ziet | Wat je kunt doen |
| --- | --- |
| Het station "lijkt geen Windows-systeemstation te zijn (geen \Windows en \Users). Kies de hoofdmap van het station van de andere pc." | Kies de hoofdmap van het station, zoals `E:\`, niet een map erop. Controleer of het het station is waarop Windows was geïnstalleerd. |
| Een schijfkopie "is geen schijfkopie die Windows kan koppelen (.vhd of .vhdx)" | Kies een `.vhd` of `.vhdx`. Sla een VMware- of VirtualBox-schijf eerst op als een van die twee. Exporteer bij een Hyper-V-controlepunt, `.avhdx`, de schijf van de virtuele machine. |
| Een schijfkopie "kan niet worden gekoppeld: Windows heeft de schijfkopie al gekoppeld, en wat de kopie heeft gekoppeld, schrijft er mogelijk naar." | De schijfkopie is ergens anders geopend, bijvoorbeeld gekoppeld in Verkenner. Kies in plaats daarvan het station ervan onder **Aangesloten station…**, of ontkoppel de schijfkopie en kies deze daarna opnieuw. |
| Een schijfkopie "heeft geen leesbaar volume met Windows erop. Een volume dat met BitLocker is versleuteld, moet eerst worden ontgrendeld." | Controleer of het een kopie is van het station waarop Windows was geïnstalleerd. Als de kopie is vergrendeld met [BitLocker](Glossary#bitlocker), koppel je deze zelf in Windows, ontgrendel je deze en kies je het station ervan onder **Aangesloten station…**. |
| Een schijfkopie "is een differentiërende schijf, die alleen bevat wat er is gewijzigd sinds de schijf waarvan deze is gemaakt" | Voeg deze samen met de schijf waarvan deze is gemaakt, met Schijf bewerken in Hyper-V, en kies die schijf. |
| Een schijfkopie "kan niet worden gekoppeld:" en een andere reden | De reden komt van Windows zelf. Lees deze, controleer of niets anders de schijfkopie gebruikt en probeer het opnieuw. |

<a id="reading-a-pc-over-the-network"></a>

### Een pc via het netwerk lezen

De volledige lijst, inclusief wat de client op de oude pc meldt, staat in [Verplaatsen vanaf een pc via het netwerk](Moving-over-the-Network#if-something-goes-wrong).

| Wat je ziet | Wat je kunt doen |
| --- | --- |
| De oude pc staat niet onder **2. Kies de oude pc** | Controleer of de client daar wordt uitgevoerd en een code weergeeft, en of beide pc's in hetzelfde netwerk zitten. Typ dan de naam of het adres. Typ via een router het adres. Een andere firewall dan die van Windows moet TCP 47732 en UDP 47731 doorlaten. |
| Een naam "is niet gevonden in het netwerk" | Typ in plaats daarvan het adres van de oude pc. |
| "de oude pc kan niet worden bereikt:" en een reden | Controleer of de client nog wordt uitgevoerd en of de naam of het adres juist is. |
| "dat is niet de code die de oude pc weergeeft. Controleer de code en probeer het opnieuw." | Typ de code die de client nu weergeeft. Na vijf verkeerde codes maakt de client een nieuwe. Na 20 laat de client niets meer koppelen totdat je deze opnieuw start. |
| Op de oude pc meldt Windows dat de uitgever van de client onbekend is | Dat is alleen te verwachten op een Windows 7 zonder het basiscertificaat van Microsoft uit 2020, die de handtekening van de client niet kan controleren. Kies daar alleen **Ja** voor een client die je zelf vanuit de app hebt opgeslagen. Overal elders moet de prompt Randy Northrup noemen. Als dat niet zo is, voer dat exemplaar dan niet uit. |
| De app kan de oude pc na een lange pauze niet meer lezen | De client verbreekt een verbinding waarover de app 30 minuten niets heeft gevraagd. Klik op **Verbinding verbreken** en koppel opnieuw. |

<a id="looking-at-the-app"></a>

### De app bekijken

| Wat je ziet | Wat het betekent | Wat je kunt doen |
| --- | --- | --- |
| "Deze app kan niet worden bekeken:" en een reden | Het bekijken is mislukt. | Klik op **Opnieuw proberen**. Als het op dezelfde manier mislukt, is de vermelding mogelijk geen app die WinAppMigrator kan verplaatsen. |
| "is er niet meer, hoewel het in de installatiegegevens van deze app als het programma van de app wordt vermeld" | De vermelding van de app in de lijst met apps noemt een programma dat is verplaatst of verwijderd | Herstel de app op deze pc of installeer deze opnieuw, en kies de app daarna weer. |
| "Er is geen eigen programma van" een app "gevonden, dus er is niets om in een installatieprogramma te zetten" | De vermelding heeft geen eigen programma, zoals vaak het geval is bij een stuurprogrammapakket, bijvoorbeeld een grafisch stuurprogramma, bij een runtime of bij een invoegtoepassing | Installeer op de nieuwe pc een stuurprogramma via de fabrikant of Windows Update, en een runtime of invoegtoepassing samen met de app die deze gebruikt. |
| "is een web-app die een browser vanaf een website heeft geïnstalleerd" | Een website die als app is geïnstalleerd, zoals Chrome Remote Desktop, zonder eigen programma | Open op de nieuwe pc de website in dezelfde browser en installeer de app vanaf daar. |
| "Gestopt. Er kan een installatieprogramma worden gemaakt zodra deze app is bekeken." | Je hebt op **Stoppen** geklikt | Klik op **Opnieuw bekijken**. |
| Het bekijken duurt lang | Een grote app, een traag of defect station, of veel accounts | Als het station mogelijk defect raakt, klik je op **Stoppen**, maak je er een [schijfkopie](Glossary#disk-image) van en kies je die in plaats daarvan, omdat lezen een station dat het begeeft de genadeslag kan geven. Vanaf 0.6.0 kies je de kopie met **Schijfkopie…**. Met 0.5.0 koppel je de kopie en kies je het station ervan. Wacht anders gewoon. Je kunt intussen een andere app kiezen. |
| Een rij die je wilt, is grijs en aangevinkt | Deze bevat het hoofdprogramma, of wat nodig is om het te starten | Deze gaat altijd mee. Zie [de rijen die je niet kunt uitvinken](Building-an-Installer#the-groups). |
| Een rij meldt "Ook weggelaten:" | Iets wat je hebt uitgevinkt, heeft dit meegenomen | Vink dat weer aan als je dit wilt. |
| "Wat er over is, vormt geen pakket" | Wat je hebt uitgevinkt, laat niets over waarvan een installatieprogramma kan worden gemaakt | Vink iets weer aan. |
| "Licentiegegevens in wat je hebt weggelaten, gaan toch mee" | Licenties volgen alleen de optie **Licentie- en activeringsgegevens** | Vink die optie uit als je ze eruit wilt hebben. |

<a id="create-installer-stays-greyed-out"></a>

### Installatieprogramma maken blijft grijs

De knop wacht totdat al het volgende waar is:

- het bekijken van de app is voltooid, voor de opties zoals ze nu zijn ("Bepalen wat daarbij hoort…" betekent dat nog wordt bepaald wat je laatste uitvinken meeneemt);
- de **Uitvoermap** is een volledig pad, zoals `D:\Installers` ("Voer een volledig mappad in, zoals D:\Installatieprogramma's.");
- er is ruimte op dat station.

<a id="room"></a>

### Ruimte

| Wat je ziet | Wat je kunt doen |
| --- | --- |
| "Er is niet genoeg ruimte." en hoeveel ruimte een station nodig heeft en heeft | Kies een uitvoermap op een ander station, of laat iets weg. Het getal is het maximum dat het maken kan innemen, vóór compressie, dus het kan meer zijn dan het installatieprogramma uiteindelijk wordt. |
| Een station "geeft niet aan hoeveel ruimte er is" | Het maken gaat door zonder dat station te controleren. Zorg dat er ruimte op is. |
| Een FAT32-station en een grote app | FAT32 kan geen bestand van 4 GiB bevatten. Gebruik een NTFS- of exFAT-station. |

<a id="the-done-screen"></a>

### Het eindscherm

| Wat je ziet | Wat je kunt doen |
| --- | --- |
| "Installatieprogramma gemaakt, maar" sommige bestanden "zijn verdwenen sinds de app is bekeken" | Klik op **Opnieuw bekijken** en daarna op **Installatieprogramma opnieuw maken**, als die bestanden belangrijk zijn. |
| "Installatieprogramma gemaakt, maar" sommige bestanden "konden niet worden vastgelegd, meestal omdat ze vergrendeld of onleesbaar waren." | Sluit de app, en al het andere dat de bestanden ervan gebruikt, en maak het installatieprogramma opnieuw. **Waarschuwingen** vermeldt welke bestanden. |
| "Installatieprogramma gemaakt, maar het vermeldt nog iets over van wie het afkomstig is." | Lees de oranje meldingen eronder. Zie [de laatste controle](Building-an-Installer#the-last-look). |
| **Waarschuwingen** meldt "Kan" een runtime "niet downloaden", en "Het installatieprogramma probeert winget op de doelcomputer." | Die runtime zit niet in het installatieprogramma, dus de nieuwe pc heeft er internet voor nodig. Maak het installatieprogramma opnieuw terwijl je online bent. Zie [Runtimes](Runtimes#at-build-time). |
| "Gebruik dit installatieprogramma niet: het heeft de hashcontrole niet doorstaan." | Gebruik het niet. Maak het opnieuw. Als het weer gebeurt, raakt het station mogelijk defect. |
| "Het installatieprogramma is geschreven, maar de hashcontrole kon niet worden uitgevoerd" | Gebruik het niet. Maak het opnieuw. |
| "Maken geannuleerd. Er is geen installatieprogramma geschreven." | Je hebt op **Annuleren** geklikt. |
| "Het installatieprogramma kan niet worden gemaakt." en een reden | Lees de reden. **Foutmelding kopiëren** kopieert deze, en **Logboek** laat zien waar het logboek staat. |
| "Kan de uitvoermap" met een pad, "niet maken" | Kies een map waarin je kunt schrijven. |
| "Kan geen beveiligde faseringsmap maken, dus er is niets gemaakt" | Het maken kon geen eigen werkmap maken naast de uitvoermap, of in de tijdelijke map van Windows. Kies een uitvoermap op een ander station. |
| "Het maken is onverwacht gestopt" | Er is iets misgegaan wat niemand had verwacht. "Als er een installatieprogramma in de uitvoermap is achtergebleven, gebruik dat dan niet." [Meld het](#getting-help) alsjeblieft. |

<a id="installing"></a>

## Installeren

<a id="when-the-installer-opens"></a>

### Wanneer het installatieprogramma wordt geopend

| Wat je ziet | Wat het betekent | Wat je kunt doen |
| --- | --- | --- |
| "Het installatieprogramma controleren…" duurt lang | Het leest het hele pakket om de hash te controleren | Wacht. Een groot installatieprogramma heeft even nodig. |
| "Kan het installatieprogramma niet lezen:" en "dit installatieprogramma is beschadigd of is gewijzigd nadat het is gemaakt, dus er is niets uit het pakket gelezen. Kopieer het opnieuw van de plek waar het is gemaakt." | Het bestand is afgekapt of gewijzigd | Kopieer het opnieuw. |
| "dit installatieprogramma bewaart het pakket in een bestand met de naam" een `.wamdata`-bestand, "in dezelfde map, en dat bestand is er niet" | Een installatieprogramma van twee bestanden mist het gegevensbestand | Kopieer het `.wamdata`-bestand naar dezelfde map. Zie [Grote apps](Big-Apps). |
| het gegevensbestand "is niet het gegevensbestand van dit installatieprogramma, of het is beschadigd of gewijzigd nadat het is gemaakt" | Het gegevensbestand komt niet overeen | Kopieer beide bestanden opnieuw, van de plek waar ze zijn gemaakt. |
| "Het pakket van dit installatieprogramma kan niet op deze pc worden gebruikt:" en "deze app kan niet op deze pc worden uitgevoerd, dus er is niets gewijzigd" | De programma's van de app zijn gemaakt voor een andere processor, of een programma geeft aan dat het een nieuwere Windows nodig heeft | Installeer op een pc zoals de oude. Er is niets gewijzigd. |
| "Dit is een installatiestub van WinAppMigrator en deze bevat geen app-pakket." | Dit is de kale stub, geen gemaakt installatieprogramma | Maak een installatieprogramma met de app en voer dat uit. |
| "De handtekening van dit installatieprogramma is niet geldig", met de reden van Windows | Het is ondertekend, maar Windows accepteert de handtekening niet. Het is mogelijk sindsdien gewijzigd, of het certificaat wordt op deze pc niet vertrouwd. | Installeer het niet. Overleg met degene die het heeft gemaakt. |
| De UAC-prompt meldt "Onbekende uitgever" | Het installatieprogramma is niet ondertekend. Dat is normaal voor een installatieprogramma dat je zelf hebt gemaakt. | Voer het alleen uit als je vertrouwt waar het vandaan komt. |

<a id="the-install-location"></a>

### De installatielocatie

Als **Installeren** en **Voorbeeld van wijzigingen** grijs zijn, staat er onder **Installatielocatie** een rode melding met de reden. De meest voorkomende reden is dat de app op deze pc al in die map is geïnstalleerd, zoals vaak het geval is bij een webbrowser, een mediaspeler of Office: verwijder die kopie eerst, via Instellingen van Windows onder **Apps**, en open het installatieprogramma opnieuw, of typ een nieuwe, lege map. De redenen, en wat je kunt doen, staan in [Installeren op de nieuwe pc](Installing-on-the-New-PC#when-the-location-cannot-be-used).

<a id="before-or-during-the-install"></a>

### Voor of tijdens de installatie

| Wat je ziet | Wat het betekent | Wat je kunt doen |
| --- | --- | --- |
| "Er is hier mogelijk niet genoeg ruimte" | Het station lijkt te weinig ruimte te hebben | Maak ruimte vrij, of kies een locatie op een ander station. |
| "er is hiervoor niet genoeg ruimte, dus er is niets gewijzigd" | Een station waarnaar het schrijft, is te vol | Maak daar ruimte vrij, of kies een installatielocatie op een ander station. |
| "Wat dit hier zou doen, is gewijzigd, dus er is niets geïnstalleerd." | Er is iets op de pc gewijzigd sinds de lijst werd bepaald | Lees de lijst opnieuw en klik opnieuw op **Installeren** als dit is wat je wilt. |
| "Een eerdere installatie van" een app "is nooit voltooid. Installeren maakt die eerst ongedaan." | Een vorige installatie is afgebroken | Klik op **Installeren**. Dat maakt die eerst ongedaan. |
| "er wordt al een andere installatie of verwijdering uitgevoerd. Wacht tot die is voltooid en probeer het dan opnieuw" | Er kan er maar één tegelijk worden uitgevoerd | Wacht en probeer het dan opnieuw. |
| Een bestand "heeft een naam die deze versie van Windows als apparaat opent en niet als bestand" | De app heeft een bestand met een naam zoals `aux.h`, dat Windows vóór Windows 11 niet kan bevatten | Installeer op Windows 11. |
| "Kan geen faseringsmap maken" | Er is geen privéplek met genoeg ruimte om het pakket uit te pakken | Maak ruimte vrij op het station van de installatielocatie of op het systeemstation. |
| Er wordt gemeld dat een gegevensmap zoals `%AppData%` een koppeling is | Het installatieprogramma schrijft niet via een [koppeling](Glossary#link) | Vink **Gebruikersgegevens** en **Licentie** uit en installeer zonder die gegevens. |
| "De map ProgramData van deze pc is niet gevonden, dus er is niets geïnstalleerd: een installatie bewaart daar het installatieverslag waarmee ze ongedaan wordt gemaakt." | De eigen gegevens van Windows over waar ProgramData staat, kunnen niet worden gelezen | Het installatieprogramma installeert niets zonder een veilige plek om het installatieverslag te bewaren. |

<a id="the-result"></a>

### Het resultaat

| Wat je ziet | Wat je kunt doen |
| --- | --- |
| "De installatie is mislukt en wat is gewijzigd, is teruggezet." | Lees in de rode meldingen waarom. De pc is zoals hij was, behalve een runtime die is geïnstalleerd. |
| "De installatie is mislukt en niet alles wat is gewijzigd, kon worden teruggezet." | Lees wat er wordt vermeld. Sluit wat die bestanden mogelijk gebruikt en voer een installatieprogramma opnieuw uit: het probeert het eerst opnieuw. |
| "Kan" een runtime "niet installeren" | De runtime kan niet worden geïnstalleerd, dus de installatie is teruggezet. Voor de DirectX End-User Runtime heeft de nieuwe pc internet nodig. Zie [Runtimes](Runtimes#with-no-internet-on-the-new-pc). |
| "Voor een afhankelijkheid moet de pc opnieuw worden opgestart om de installatie te voltooien. Start Windows opnieuw op en start daarna de app." | Start opnieuw op en start daarna de app. |
| De service "werd op de bron-pc uitgevoerd als" een account. "Het wachtwoord zit niet in het installatieprogramma, dus de service wordt ingesteld als LocalService." | Open Services en stel het account en wachtwoord van de service in, als de service niet werkt zoals hij nu is. |
| Sommige bestanden "komen in plaats daarvan onder" de installatiemap, "in een map met de naam _external" | Deze pc bewaart Documenten of Opgeslagen spellen buiten je profiel. Verplaats die bestanden zelf naar die map. |
| Iets is "weggelaten", met een reden | De regels hebben het geweigerd, of het is hier al van een andere app. De rest is geïnstalleerd. Als de app het nodig heeft, bekijk dan de reden. |
| **Pakkethashes ONGELDIG** of **Geïnstalleerde bestanden ONGELDIG** | Iets kwam niet overeen. Kopieer het installatieprogramma opnieuw en probeer het nog een keer. |

<a id="after-installing"></a>

### Na het installeren

| Wat je ziet | Wat je kunt doen |
| --- | --- |
| De app start niet | Kijk bij **Afhankelijkheden** of er een runtime is met **Handmatig installeren**, en installeer die. Bekijk de waarschuwingen in het resultaat. |
| Een app van Windows 7 start niet, en een waarschuwing bij het maken noemde een DLL waarbij stond "Windows 10 en 11 worden er niet mee geleverd" | Die DLL is door iets anders in de eigen mappen van de oude Windows geplaatst, dus deze is niet meegegaan. Installeer wat de DLL levert en start de app daarna opnieuw. Zie [Apps van Windows 7, Vista, 8 of 8.1](Moving-from-a-Connected-Drive#apps-from-windows-7-vista-8-or-81). |
| De app start alsof deze nieuw is | Dat is te verwachten bij een installatieprogramma dat is gemaakt met **Gebruikersgegevens** uitgeschakeld. Het gebeurt ook als **Gebruikersgegevens** bij het installeren was uitgevinkt, omdat de gegevens dan in de installatiemap komen, en als het installatieprogramma niet kon vaststellen wie er was aangemeld, wat het dan ook meldt. |
| De app vraagt om activering | Een licentie die aan de oude pc is gebonden, moet opnieuw worden geactiveerd. |
| De eigen updater van de app denkt dat de app niet is geïnstalleerd | Windows Installer heeft hier geen gegevens over de app. Zie [Beperkingen](Limitations). |
| Een bestandstype of koppelingstype wordt niet met de app geopend | Een andere app was hier al eigenaar, dus het is met rust gelaten. Kies de app in Instellingen, onder Standaard-apps. |
| Een nieuwe vermelding in `Path` wordt niet gezien | Open een nieuw PowerShell-venster of een nieuwe opdrachtprompt. Sommige programma's zien een wijziging pas nadat je je hebt afgemeld en weer hebt aangemeld. |

<a id="uninstalling"></a>

## Verwijderen

| Wat je ziet | Wat je kunt doen |
| --- | --- |
| "Een deel ervan staat op een station dat niet is aangesloten." | Sluit dat station aan en verwijder de app opnieuw via Instellingen. |
| "Niet alles kon ongedaan worden gemaakt. Sluit wat het gebruikt en verwijder de app opnieuw via Instellingen." | Sluit de app en alles wat de bestanden ervan gebruikt. Als een service niet stopt, start je de pc opnieuw op. Verwijder de app daarna opnieuw. |
| "this is not an install's id" | De id die aan `--uninstall` is gegeven, is onjuist. Kopieer de id uit de vermelding in Instellingen. Zie [Opdrachtregel](Command-Line#finding-the-id). |
| Er is een bestand met de naam `.before-install` verschenen | Het is het origineel van een bestand dat de installatie heeft vervangen en dat jij sindsdien had gewijzigd. Bewaar de versie die je wilt. |
| Er is nog een runtime geïnstalleerd | Runtimes blijven staan, omdat andere programma's ze mogelijk gebruiken. Laat de runtime staan, tenzij je zeker weet dat niets anders deze gebruikt. |

<a id="scripts"></a>

## Scripts

| Wat je ziet | Wat je kunt doen |
| --- | --- |
| "The requested operation requires elevation", of op een Nederlandse Windows de melding dat voor de bewerking verhoogde bevoegdheden nodig zijn | Voer PowerShell uit als administrator. |
| Er wordt niets afgedrukt en `$LASTEXITCODE` is niet ingesteld | Leid de uitvoer van het installatieprogramma door, bijvoorbeeld naar `Out-String`. Zie [Opdrachtregel](Command-Line). |
| Afsluitcode `2` | Een vlag is onjuist, of `--apply` en `--dry-run` zijn samen gebruikt. |
| "this package does not say where its app was, so give a --target-root" | Voeg `--target-root` toe met een volledig pad. |
| De app "was in" een map, "and that cannot be used" | Voeg `--target-root` toe met een andere map. |
| Een map "cannot be used: it already has files" "Give --force only if you know it is this app's" | Kies een lege map. Gebruik `--force` alleen voor een map waarvan je weet dat die van deze app is. |

<a id="language-and-text"></a>

## Taal en tekst

- **De verkeerde taal.** Kies een andere onder **Taal**, rechtsboven in de app of in het eerste scherm van een installatieprogramma. De app onthoudt de keuze; een installatieprogramma houdt deze vast voor die keer. Zonder keuze gebruikt elk programma de eerste weergavetaal van Windows die het heeft, en anders Engels.
- **Een vertaling klopt niet.** Alles behalve Engels is automatisch vertaald. Kies onder **Taal** de optie **Een vertaalprobleem melden**. Die opent een formulier waarin je taal al is ingevuld. Vermeld wat het scherm toont en wat er zou moeten staan.
- **Letters worden weergegeven als lege vakjes.** Het venster gebruikt de lettertypen die Windows voor elk schrift meelevert, uit `C:\Windows\Fonts`. Een Windows waaruit sommige daarvan zijn verwijderd, tekent die letters als vakjes; de namen van andere apps in de lijst kunnen ze ook nodig hebben.
- **Woorden worden afgekapt.** Een label dat niet past, eindigt op `…`; beweeg de muisaanwijzer erover, of ga ernaartoe met de Tab-toets, om de hele tekst te zien. Meld het met de taal en het scherm, zodat het venster passend kan worden gemaakt.
- **WinAppMigrator Client drukt Engels af op een Windows die van rechts naar links schrijft.** De console waarin de client wordt uitgevoerd, kan Arabische of Hebreeuwse letters niet verbinden of in de juiste volgorde zetten, dus de client drukt Engels af en vermeldt waarom. Met `--lang` kies je elke taal die de client heeft.

<a id="getting-help"></a>

## Hulp krijgen

1. Zoek het [logboek](Glossary#log). Het pad wordt bij een fout weergegeven als **Logboek**, of vanaf een opdrachtregel afgedrukt als `Log:`. Logboeken staan in `%ProgramData%\WinAppMigrator\logs`. Ze bevatten paden en sleutelnamen, en nooit wat een instelling, licentiebestand of omgevingsvariabele bevat. Ze kunnen wel je mapnamen bevatten, dus lees een logboek voordat je het deelt.
2. [Open een issue](https://github.com/RandyNorthrup/WinAppMigrator/issues) met de versie, wat je hebt gedaan en wat er op het scherm stond. **Foutmelding kopiëren** in het eindscherm van de app kopieert de melding.
3. **Voeg nooit een installatieprogramma toe aan een issue.** Het kan privégegevens bevatten.
4. Meld een beveiligingsprobleem in plaats daarvan privé, via de [beveiligingsadviezen van GitHub](https://github.com/RandyNorthrup/WinAppMigrator/security/advisories/new).
