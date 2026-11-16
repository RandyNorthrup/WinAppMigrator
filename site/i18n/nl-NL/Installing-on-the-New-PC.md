<a id="installing-on-the-new-pc"></a>

# Installeren op de nieuwe pc

Het installatieprogramma bestaat uit één venster. Het controleert zichzelf, laat zien waar het installeert en alles wat het instelt, en installeert pas wanneer je op **Installeren** klikt. Deze pagina loopt het venster van boven naar beneden door, met de woorden die op het scherm staan.

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/installer-dark.png">
    <img alt="Het installatieprogramma voor Fabrikam Data Sync, met de melding dat het niet is ondertekend, de installatielocatie, de benodigde ruimte en wat het naast bestanden instelt" src="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/installer-light.png" width="600">
  </picture>
</p>

<a id="before-you-start"></a>

## Voordat je begint

- **Kopieer het hele installatieprogramma.** Als het maken naast het `.exe`-bestand ook een `.wamdata`-bestand heeft opgeleverd, kopieer je beide naar dezelfde map, met dezelfde namen. Zie [Grote apps](Big-Apps).
- **Voer alleen installatieprogramma's uit die je vertrouwt.** Een installatieprogramma wordt uitgevoerd met volledige [administratorrechten](Glossary#administrator-rights). Behandel het zoals elk ander programma: voer het alleen uit als je het zelf hebt gemaakt, of als je het hebt gekregen van iemand die je vertrouwt. Zie [Privacy en beveiliging](Privacy-and-Security).
- **Je hebt administratorrechten nodig** op deze pc, en [Direct3D 12](Glossary#direct3d-12) voor het venster. Zonder Direct3D 12 kun je nog steeds installeren vanaf een opdrachtprompt. Zie [Opdrachtregel](Command-Line).

<a id="open-it"></a>

## Openen

Dubbelklik op het installatieprogramma. Windows toont de [UAC-prompt](Glossary#uac-prompt) van Gebruikersaccountbeheer. Een installatieprogramma is niet ondertekend, tenzij degene die het heeft gemaakt het heeft ondertekend, dus de prompt meldt meestal **Onbekende uitgever**. Dat is te verwachten. Zie [Privacy en beveiliging](Privacy-and-Security#signing).

Het venster, "WinAppMigrator-installatieprogramma", "Installeert een app opnieuw die op een andere pc is vastgelegd", wordt meteen geopend, hoe groot het installatieprogramma ook is. Er staat "Het installatieprogramma controleren…" terwijl het de [hash](Glossary#hash) van het hele pakket controleert. Daarvoor wordt alles gelezen, dus een groot installatieprogramma heeft even nodig. Vanaf dat moment houdt het installatieprogramma zijn eigen bestand open, en ook het gegevensbestand als het er een heeft, zodat geen van beide kan worden gewijzigd of verwisseld zolang het venster open is.

De controle kan hier stoppen met een rode melding. De meest voorkomende zijn:

- Het installatieprogramma is beschadigd of is gewijzigd nadat het is gemaakt: kopieer het opnieuw van de plek waar het is gemaakt.
- Het gegevensbestand ontbreekt of is niet het juiste: kopieer beide bestanden opnieuw, naar één map.
- "Het pakket van dit installatieprogramma kan niet op deze pc worden gebruikt", bijvoorbeeld omdat de programma's van de app voor een ander soort processor zijn gemaakt, of een nieuwere Windows nodig hebben. Er is niets gewijzigd.

Ze worden allemaal uitgelegd in [Problemen oplossen](Troubleshooting#installing).

Als de controle slaagt, wordt de titel van het venster de naam van de app, gevolgd door "installeren".

<a id="its-language"></a>

### De taal

Een installatieprogramma spreekt de taal van de pc waarop het wordt uitgevoerd, niet die van de pc waarop het is gemaakt: de weergavetaal van Windows als dat een van de 24 talen van WinAppMigrator is, en anders Engels. Met **Taal**, rechtsboven, wijzig je de taal voor deze keer. Wat het over de app vermeldt, inclusief de opmerkingen die de app heeft geschreven toen het installatieprogramma werd gemaakt, komt ook in die taal. Een installatieprogramma dat is gemaakt met WinAppMigrator 0.6 of ouder, toont die opmerkingen in het Engels, zoals ze zijn geschreven. Een [stille installatie vanaf een opdrachtregel](Command-Line) antwoordt altijd in het Engels, zodat scripts het kunnen lezen.

<a id="the-app-and-where-it-goes"></a>

## De app en waar deze terechtkomt

De eerste kaart bevat de naam van de app, met de uitgever en de versie eronder, zoals "Fabrikam Inc. · 1.7.3". Daarna:

| Rij | Wat er staat |
| --- | --- |
| **Afkomstig van** | De naam van de pc waar de app vandaan komt. Deze rij staat er alleen als het installatieprogramma is gemaakt met de optie **Gebruikersgegevens** ingeschakeld. |
| **Installatieprogramma** | Of dit installatiebestand is [ondertekend](Glossary#signature): "Handtekening controleren…", daarna "Niet ondertekend: uitgever onbekend", of "Ondertekend door" en de naam van de ondertekenaar. Een handtekening die niet klopt, geeft in plaats daarvan een rode melding: "De handtekening van dit installatieprogramma is niet geldig", met de reden van Windows, en "Het is mogelijk gewijzigd nadat het is ondertekend." |
| **Installatielocatie** | De map waarin de app wordt geplaatst, met **Bladeren** om er een te kiezen. In het begin is dit de map waarin de app op de oude pc stond. Een app van een 32-bits Windows begint in plaats daarvan in `Program Files (x86)`, waar 64-bits Windows een 32-bits app bewaart. Als die map hier niet kan worden gebruikt, staat eronder een rode melding met de reden. |
| **Benodigde ruimte** | Hoeveel ruimte de installatie op dat station nodig heeft en hoeveel er vrij is, zoals "maximaal 824,0 MB op C: (356,0 GB vrij)". Het uitgepakte pakket telt mee, net als de bestanden van de app. |

De naam van de uitgever op de eerste regel komt uit het pakket, dus iedereen kan die hebben ingevuld. De rij **Installatieprogramma** komt van Windows, en die kun je vertrouwen.

<a id="when-the-location-cannot-be-used"></a>

### Als de locatie niet kan worden gebruikt

Als de map in **Installatielocatie** niet kan worden gebruikt, staat eronder een rode melding met de reden. **Installeren** en **Voorbeeld van wijzigingen** blijven grijs totdat je een andere kiest, en ernaast staat "Installeren en Voorbeeld van wijzigingen zijn uitgeschakeld totdat de installatielocatie kan worden gebruikt."

De meest voorkomende reden is dat er al bestanden in de map staan. Een map met bestanden wordt alleen gebruikt als een eerdere installatie van WinAppMigrator ze daar heeft geplaatst, zodat de bestanden van een andere app nooit worden vervangen:

| Melding | Wat het betekent | Wat je kunt doen |
| --- | --- | --- |
| De naam van de app, gevolgd door "is al in deze map geïnstalleerd, dus deze kopie kan hier niet worden geplaatst." | De app is al op deze pc geïnstalleerd, vaak door het eigen installatieprogramma: het hoofdprogramma staat in die map, of de lijst met geïnstalleerde apps van Windows noemt die map. Dit gebeurt bij apps die veel pc's al hebben, zoals een webbrowser, een mediaspeler of Office. | Verwijder eerst de kopie op deze pc, via Instellingen van Windows onder **Apps**, en open het installatieprogramma daarna opnieuw. Of typ een nieuwe, lege map in **Installatielocatie** om deze kopie ernaast te installeren. |
| De naam van een andere app, gevolgd door "is in deze map geïnstalleerd", en dat deze app "kan hier niet worden geplaatst". | De lijst met geïnstalleerde apps van Windows noemt een andere app in die map, zoals een andere editie van Office. | Typ een nieuwe, lege map in **Installatielocatie**. Of, als je de andere app niet meer nodig hebt, verwijder je die eerst, via Instellingen van Windows onder **Apps**, en open je het installatieprogramma daarna opnieuw. |
| "Deze map bevat al bestanden die niet door WinAppMigrator zijn geïnstalleerd." | Er staat iets anders in de map, en er is daar geen geïnstalleerde app vermeld. Het is mogelijk van een andere app. | Typ een nieuwe, lege map in **Installatielocatie**. Als de bestanden zijn overgebleven van een app die je hebt verwijderd, kun je de map verwijderen en het installatieprogramma daarna opnieuw openen. |

Bij elke andere reden staat er "Deze map kan niet worden gebruikt:", de reden, en "Voer hierboven een andere map in." De redenen zijn:

| Reden op het scherm | Wat je kunt doen |
| --- | --- |
| "dit moet een volledig pad zijn" | Typ een pad dat met een stationsletter begint, zoals `C:\Program Files\Fabrikam Data Sync`. |
| "dit mag geen volledig station zijn" | Gebruik een map op het station, niet het station zelf. |
| "dit mag niet de Windows-map zijn" | Kies een map buiten `C:\Windows`. |
| "hier staan andere dingen naast elkaar in, dus de app heeft een map hierbinnen nodig" | Voeg een map voor de app toe, zoals `C:\Program Files\Fabrikam Data Sync` in plaats van `C:\Program Files`. |
| "dit is een map van Windows, of een map waarvan de inhoud vanzelf wordt uitgevoerd of geladen" | Kies een gewone map. Een map Opstarten wordt bijvoorbeeld geweigerd. |
| "dit moet een gewoon pad op een station van deze pc zijn" | Een [gedeelde map in het netwerk](Glossary#network-share) wordt geweigerd, net als een naam die Windows als iets anders zou lezen. |
| de map, gevolgd door "is een koppeling naar een andere plek, en anderen kunnen mogelijk wijzigen waar deze naartoe leidt" | Gebruik de map waar de [koppeling](Glossary#link) naartoe leidt. |
| "hier bewaren installaties hun verslagen" | Kies een map buiten `%ProgramData%\WinAppMigrator`. |

Als de map bestanden bevat en er daar **wel** een eerdere installatie van dit project is geregistreerd, gebruikt het installatieprogramma de map en waarschuwt het: "Deze map bevat al bestanden. Bestanden met dezelfde naam worden vervangen en komen terug als je de app verwijdert."

Zodra een map is geaccepteerd, vervangt het venster een bestand, registerwaarde, service, taak, snelkoppeling of variabele met dezelfde naam, zoals `--force` dat doet op de [opdrachtregel](Command-Line#what---force-does). Wat al van een andere app is, zoals de service of het bestandstype daarvan, blijft van die app.

Als het station te weinig ruimte lijkt te hebben, staat er een oranje opmerking "Er is hier mogelijk niet genoeg ruimte" met wat er nodig is. Het pakket kan op een ander station worden uitgepakt, maar de bestanden van de app moeten nog steeds op dit station passen. De installatie zelf controleert de ruimte opnieuw voordat deze begint.

<a id="an-earlier-install-that-never-finished"></a>

### Een eerdere installatie die nooit is voltooid

Als een vorige installatie door een van deze installatieprogramma's is afgebroken, door een crash of een stroomstoring, staat er een oranje opmerking: "Een eerdere installatie van" die app "is nooit voltooid. Installeren maakt die eerst ongedaan." Dat gebeurt automatisch wanneer je op **Installeren** klikt.

<a id="what-this-sets-up-beyond-files"></a>

## Wat dit naast bestanden instelt

Deze kaart is de lijst waarmee je akkoord gaat wanneer je op **Installeren** klikt. Onder de titel staat:

> Dit gebeurt allemaal met administratorrechten. Als de installatie mislukt, wordt teruggezet wat is gewijzigd en krijg je te horen wat niet kon worden teruggezet. Je kunt de installatie later ook ongedaan maken via Instellingen, onder Geïnstalleerde apps. Bestanden die worden vervangen, komen dan terug, en runtimes die worden geïnstalleerd, blijven staan.

Elke rij heeft een naam, rechts een label dat aangeeft wat voor soort ding het is, en een regel die zegt wat het doet. Een label in het gewone blauw is normaal. Een label in **oranje** markeert iets dat met volledige rechten wordt uitgevoerd, zoals een service die als LocalSystem wordt uitgevoerd, een stuurprogramma of een firewallregel die verbindingen binnenlaat. Kijk daar twee keer naar.

Als de lijst leeg is, staat er "Niets: er worden alleen bestanden gekopieerd, en instellingen in de eigen registersleutels van de app."

De lijst wordt bepaald voor de locatie in het vak. Als je de locatie wijzigt, staat er even "Bepalen wat dit op deze locatie zou instellen…", en wacht **Installeren**.

<a id="what-each-label-means"></a>

### Wat elk label betekent

| Label | Wat het is | Voorbeeld van de regel eronder |
| --- | --- | --- |
| **Service** | Een [service](Glossary#service): een programma dat Windows op de achtergrond uitvoert | "Voert FabrikamSync.exe uit als NT AUTHORITY\LocalService, start nu en telkens wanneer deze pc opstart". Andere zeggen "start nu en kort nadat deze pc is opgestart", "start alleen wanneer iets erom vraagt" of "is uitgeschakeld". |
| **Stuurprogramma** | Een [stuurprogramma](Glossary#driver) dat van de app is | "Laadt" een bestand "in Windows zelf". Altijd oranje. Geen enkele test van het project heeft een stuurprogramma geladen. |
| **Geplande taak** | Een [geplande taak](Glossary#scheduled-task) | "Wordt uitgevoerd als SYSTEM", of als een account. "met de hoogst mogelijke rechten" is oranje gemarkeerd. Een taak die op de oude pc als een persoon werd uitgevoerd, wordt uitgevoerd als degene die installeert. |
| **Start automatisch** | Een [opstartitem](Glossary#startup-item) | "Wordt uitgevoerd wanneer je je aanmeldt", "Wordt uitgevoerd voor iedereen die zich aanmeldt" of "Een snelkoppeling in de map Opstarten, voor jou" |
| **Snelkoppeling** | Een [snelkoppeling](Glossary#shortcut) op het bureaublad of in het menu Start | "Opent Fabrikam.exe, voor iedereen op deze pc" of "voor jou" |
| **PATH** | Een vermelding die wordt toegevoegd aan de [variabele Path](Glossary#path-variable) | "Voegt toe aan Path" en "De installatiemap\bin". Deze wordt toegevoegd aan wat er al staat, en vervangt dat nooit. |
| **Omgevingsvariabele** | Een [omgevingsvariabele](Glossary#environment-variable) | "Ingesteld op" de waarde |
| **Shell-extensie** | Code die in Verkenner wordt geladen | "Laadt" een bestand "in Verkenner, voor iedereen op deze pc". Oranje. |
| **Menuopdracht** | Een opdracht in het snelmenu dat je met de rechtermuisknop opent | "Voegt een opdracht toe aan het menu van" een soort bestand. "Wijzigt wat er gebeurt wanneer je een van deze opent" is oranje. |
| **Koppelingstype** | Een [koppelingsprotocol](Glossary#link-protocol), zoals `fabrikam:` | "Koppelingen van dit type worden met de app geopend" |
| **Bestandstype** | Een [bestandstype](Glossary#file-type), zoals `.fds` | "Bestanden van dit type worden met de app geopend" |
| **COM** | [COM-klassen](Glossary#com-class) die andere programma's kunnen starten | "COM in" een bestand, met hoeveel instellingen. "COM-machtigingen" is oranje. |
| **Firewallregel** | Een [firewallregel](Glossary#firewall-rule) voor een van de programma's | "Staat inkomende verbindingen op 8080 toe voor FabrikamSync.exe, op elk netwerk". Inkomende verbindingen toestaan is oranje. |
| **Bron voor gebeurtenislogboek** | Een naam waaronder de app naar de gebeurtenislogboeken van Windows schrijft | "Kan schrijven naar het logboek Application, met berichten uit de eigen bestanden" |
| **Lettertype** | Een lettertype in de installatiemap | "Wordt bij het aanmelden geladen vanuit de installatiemap" |
| **ODBC-stuurprogramma** | Een databasestuurprogramma voor [ODBC](Glossary#odbc) | "Wordt geladen in programma's die ODBC gebruiken om een database te bereiken". Oranje. |
| **ODBC-gegevensbron** | Een opgeslagen databaseverbinding | "Een databaseverbinding, voor iedereen op deze pc" |
| **Compatibiliteitsinstelling** | Hoe Windows sommige van de programma's start | "Hoe de programma's starten" |
| **Standaard-apps** | Een vermelding waarmee je de app kunt kiezen in Standaard-apps | "Kan worden gekozen in Standaard-apps. Er wordt niets aan gewijzigd" |
| **Programmanaam** | Een naam waaronder Uitvoeren en andere programma's de app kunnen vinden | "Uitvoeren en andere programma's kunnen het via deze naam vinden" |
| **Overige instellingen** | [Register](Glossary#registry)waarden buiten de eigen sleutels van de app en de soorten hierboven | "3 instellingen buiten de eigen sleutels van de app", "In" de sleutels. Oranje als er een in de sleutels van Microsoft staat. |

Sommige dingen kunnen pas worden beoordeeld wanneer de installatie eraan toe is, zoals een service met dezelfde naam die al van een andere app is. Die blijven op de lijst staan zoals gevraagd. Wanneer de installatie daar aankomt, laat ze die met rust en meldt ze dat.

<a id="notes-and-what-is-left-out"></a>

### Opmerkingen en wat is weggelaten

Onder de rijen staan oranje opmerkingen over wat je moet weten, bijvoorbeeld:

- dat referenties niet in het installatieprogramma zijn opgenomen, zodat je ze mogelijk opnieuw moet invoeren;
- dat de optie **Licentie- en activeringsgegevens** of **Gebruikersgegevens** was uitgeschakeld toen het werd gemaakt, zodat die gegevens er niet in zitten;
- op welke [certificaten](Glossary#certificate) de app leek te vertrouwen, die nooit worden geïnstalleerd;
- wat degene die het installatieprogramma heeft gemaakt, heeft weggelaten en wat die heeft toegevoegd;
- dat de app hier mogelijk niet werkt: "Deze app stond op Windows-build" een nieuwere, of dat Windows sommige programma's "via emulatie" zou uitvoeren;
- dat bestanden uit Documenten of Opgeslagen spellen "onder" de installatiemap komen, "in een map met de naam _external", omdat de map van deze pc buiten je profiel staat.

Daarna, in oranje, alles wat de veiligheidsregels weglaten, met de reden. De eerste 20 worden volledig weergegeven. De rest staat onder een regel zoals "Nog 12 items zijn weggelaten", die je kunt openen. De rest van de installatie gaat zonder die items gewoon door. Wat wordt geweigerd, en waarom, staat in [Wat er in een installatieprogramma gaat](What-Goes-in-an-Installer#what-the-installer-refuses).

Als het pakket helemaal niet naar de gekozen locatie kan, staat er een rode melding "Dit pakket kan niet op deze locatie worden geplaatst" met de reden.

<a id="options"></a>

## Opties

Deze kaart staat er alleen als het installatieprogramma gebruikersgegevens of licentiegegevens bevat. Elk vak is in het begin aangevinkt.

| Optie | Wat er staat |
| --- | --- |
| **Gebruikersgegevens** | "Aan: instellingen, Documenten en Opgeslagen spellen gaan naar de gebruikelijke mappen van wie er is aangemeld. Uit: alles komt in de installatiemap terecht en er wordt niets ingesteld om van daaruit te worden uitgevoerd." Als het installatieprogramma vermeldt wiens gegevens het bevat, staat eronder "Dit bevat de gegevens van" dat account. |
| **Licentie** | "Activerings- en licentiebestanden, en de registerwaarden waarin licenties zijn opgeslagen. Sommige moeten hier mogelijk opnieuw worden geactiveerd." |

Als je een vak wijzigt, past de lijst hierboven zich meteen aan.

Gebruikersgegevens gaan altijd naar **wie er is aangemeld** bij Windows, ook als bij de UAC-prompt de naam van een andere administrator is getypt. Als het installatieprogramma niet kan vaststellen wie dat is, meldt het dat, laat het instellingen per gebruiker weg en plaatst het gebruikersgegevens in de installatiemap in plaats van te gokken. Als er niemand bij die sessie is aangemeld, gaan gebruikersgegevens en instellingen per gebruiker naar het account waarmee het installatieprogramma wordt uitgevoerd, en dat meldt het ook.

<a id="dependencies"></a>

## Afhankelijkheden

"Wat deze app nodig heeft en hoe elk onderdeel wordt geleverd." Elke [runtime](Glossary#runtime) heeft een label:

| Label | Betekenis |
| --- | --- |
| **Inbegrepen** | In het installatieprogramma. Wordt zonder internet geïnstalleerd, behalve de DirectX End-User Runtime, waarvan het installatieprogramma de runtime bij Microsoft ophaalt. Hiervan is in de tests van het project alleen de x64 Visual C++-runtime echt geïnstalleerd. |
| **Wordt geïnstalleerd met winget:** en een id | De runtime kon niet worden gedownload toen het installatieprogramma werd gemaakt, dus het installatieprogramma gebruikt [winget](Glossary#winget), waarvoor internet nodig is. Deze route is niet uitgevoerd in de tests van het project. |
| **Windows-onderdeel: NetFx3** | .NET Framework 3.5, ingeschakeld als [Windows-onderdeel](Glossary#windows-feature). Hiervoor kan internet nodig zijn. Deze route is niet uitgevoerd in de tests van het project. |
| **Ingebouwd in Windows** | .NET Framework 4, dat Windows 10 en 11 al hebben |
| **Wordt met de app meegeleverd** | De app heeft een eigen exemplaar |
| **Handmatig installeren** | Hier is er geen installatieprogramma voor. Installeer het zelf. |

Als er geen zijn, staat er "Er zijn geen extra runtime-afhankelijkheden gevonden." Zie [Runtimes](Runtimes).

<a id="preview-changes"></a>

## Voorbeeld van wijzigingen

**Voorbeeld van wijzigingen** bepaalt precies wat een installatie zou doen, en wijzigt niets. Het controleert of er ruimte is, pakt het pakket uit naar een privémap, bepaalt de wijzigingen en verwijdert die map weer. De resultaatkaart heeft als kop "Voorbeeld: er is nog niets gewijzigd", met **Geplande wijzigingen** en een lijst van de wijzigingen, met services, taken en snelkoppelingen eerst. Het venster toont de eerste 300 en vermeldt daarna hoeveel er nog meer zijn.

Een voorbeeld controleert niet de hash van elk bestand. De hash van het hele pakket is al gecontroleerd toen het venster werd geopend.

<a id="install"></a>

## Installeren

**Installeren** blijft grijs totdat de locatie kan worden gebruikt en de lijst voor die locatie geldt. Als het pakket helemaal niet naar die locatie kan, staat ernaast "Installeren is uitgeschakeld omdat dit pakket niet op deze locatie kan worden geplaatst.", en staat de reden in rood bij **Wat dit naast bestanden instelt**. Wanneer je erop klikt, gebeurt het volgende:

1. Het bepaalt de lijst nog één keer. Als wat het zou doen is gewijzigd sinds je ernaar hebt gekeken, installeert het niets, toont het de nieuwe lijst en staat er "Wat dit hier zou doen, is gewijzigd, dus er is niets geïnstalleerd. Bekijk de lijst hieronder en klik opnieuw op Installeren als dit is wat je wilt."
2. Het controleert of er ruimte is op elk station waarnaar het schrijft.
3. Het pakt het pakket uit naar een map die alleen administrators kunnen gebruiken, en controleert de hash van elk bestand dat eruit komt.
4. Het installeert, en houdt eerst een [journaal](Glossary#journal) bij van elke wijziging, zodat die ongedaan kan worden gemaakt.
5. Het leest elk bestand terug dat het heeft geïnstalleerd en controleert de hash.
6. Het start de services die zijn ingesteld om vanzelf te starten.

Onderaan zie je wat het doet, een telling waar die er is, hoelang het al bezig is, een voortgangsbalk en **Annuleren**. De stappen die je kunt zien, zijn "Voorbereiden…", "Uitpakken", "Het pakket controleren", "Een eerdere installatie ongedaan maken die nooit is voltooid", "Runtimes installeren", "Bestanden kopiëren", "Services instellen", "Stuurprogramma's instellen", "Registerinstellingen schrijven", "Licentiegegevens herstellen", "Geplande taken instellen", "Snelkoppelingen maken", "Omgevingsvariabelen instellen", "De app toevoegen aan Instellingen" en "Controleren wat is geïnstalleerd".

**Annuleren** toont "Annuleren na de huidige stap…" en daarna "Terugzetten wat is gewijzigd". Als je het venster sluit tijdens het installeren, gebeurt hetzelfde als bij **Annuleren**: het venster blijft open totdat alles is teruggezet. Als er nog niets was gewijzigd, staat er "Geannuleerd. Er is niets gewijzigd."

<a id="the-hash-check"></a>

## De hashcontrole

Het installatieprogramma controleert [hashes](Glossary#hash) drie keer:

1. **Wanneer het venster wordt geopend**: de hash van het hele pakket. Een afgekapte kopie, of een kopie die is bewerkt nadat deze is gemaakt, wordt geweigerd voordat er iets uit wordt gelezen.
2. **Tijdens het uitpakken**: de hash van elk bestand, voordat er iets op de plaats ervan wordt geschreven.
3. **Na het installeren**: het leest elk bestand terug dat het heeft geïnstalleerd en controleert het opnieuw.

Hashes vangen beschadiging op, en wijzigingen door iedereen die niet ook de hashes bijwerkt. Ze bewijzen niet wie een installatieprogramma heeft gemaakt. Alleen een [handtekening](Glossary#signature) doet dat.

<a id="the-result"></a>

## Het resultaat

De resultaatkaart verschijnt bovenaan, met als kop "Installatieresultaat".

| Melding | Wat het betekent |
| --- | --- |
| "De app is geïnstalleerd en heeft de hashcontrole doorstaan." | Alles is in orde. Als de regels iets hebben weggelaten, staat erbij "zonder wat hieronder als weggelaten wordt vermeld." |
| "De installatie is geannuleerd en wat is gewijzigd, is teruggezet." | Je hebt geannuleerd. |
| "De installatie is mislukt en wat is gewijzigd, is teruggezet." | Er is iets mislukt. Wat is gewijzigd, is teruggezet, behalve een runtime die is geïnstalleerd. Wat eronder in oranje staat, is blijven staan, met de reden, zoals een service die pas verdwijnt wanneer de pc opnieuw opstart. |
| "De installatie is mislukt en niet alles wat is gewijzigd, kon worden teruggezet. Wat er nog over is, staat hieronder. De volgende installatie probeert het opnieuw." | Lees de lijst. Als je een installatieprogramma opnieuw uitvoert, probeert het opnieuw om alles terug te zetten. |
| "De installatie is niet zonder problemen voltooid. Bekijk de problemen hieronder." | Lees de meldingen eronder. |

Daarna volgen drie tegels: **Aangebrachte wijzigingen** (of **Teruggezette wijzigingen** na een mislukking), **Waarschuwingen** en **Fouten**. Labels tonen **Pakkethashes geldig** en **Geïnstalleerde bestanden geldig**, of **ONGELDIG** in rood. Tot vijf waarschuwingen en alle fouten worden als melding weergegeven. "Voor een afhankelijkheid moet de pc opnieuw worden opgestart om de installatie te voltooien. Start Windows opnieuw op en start daarna de app." betekent dat een runtime om opnieuw opstarten heeft gevraagd.

**Wijzigingen**, met een aantal, opent de lijst van wat er is gedaan, tot 300 items. Als de installatie niet zonder problemen is voltooid, wordt het pad van het [logboek](Glossary#log) weergegeven als **Logboek**.

Na een geslaagde installatie worden de knoppen onderaan **Sluiten** en **Installatiemap openen**.

<a id="where-things-end-up"></a>

## Waar alles terechtkomt

- **Programmabestanden** komen in de installatielocatie.
- **Gebruikersgegevens** gaan, met dat vak aangevinkt, naar de eigen mappen AppData, Documenten en Opgeslagen spellen van het aangemelde account, en naar ProgramData. Er wordt niet geschreven naar een map Documenten of Opgeslagen spellen die deze pc buiten je profiel bewaart. Die bestanden komen in de installatielocatie, in `_external`, en de lijst vermeldt dat als eerste. Met het vak uitgevinkt komen alle gebruikersgegevens in de installatielocatie.
- **Snelkoppelingen** komen op het bureaublad en in het menu Start van deze pc: die van jou, of die van iedereen als ze daar stonden.
- Aan **Path** worden alleen de vermeldingen van de app toegevoegd. Verder verandert er niets aan.
- **Wat hier al van een andere app is, blijft van die app.** Een bestandstype, koppelingstype, COM-klasse, service, bron voor gebeurtenislogboek, lettertype of iets dergelijks dat al van iets anders is, wordt met rust gelaten, en het installatieprogramma meldt dat. Een app die verwachtte `http`-koppelingen te openen, moet bijvoorbeeld opnieuw als standaard worden ingesteld in Instellingen.
- **Instellingen** krijgt een vermelding voor de app, zodat je deze kunt verwijderen zoals elke andere app. Zie [Een installatie ongedaan maken](Undoing-an-Install).

<a id="common-reasons-it-refuses"></a>

## Veelvoorkomende redenen om te weigeren

| Wat je ziet | Waarom | Wat je kunt doen |
| --- | --- | --- |
| "deze app kan niet op deze pc worden uitgevoerd, dus er is niets gewijzigd" | De programma's zijn gemaakt voor een ander soort processor, of hebben een nieuwere Windows nodig | Installeer op een pc die overeenkomt met de oude |
| "er is hiervoor niet genoeg ruimte, dus er is niets gewijzigd" | Niet genoeg vrije ruimte op een station waarnaar het schrijft | Maak ruimte vrij, of kies een locatie op een ander station |
| "er wordt al een andere installatie of verwijdering uitgevoerd" | Er kan er maar één tegelijk worden uitgevoerd | Wacht tot die is voltooid en probeer het dan opnieuw |
| Een bestand "heeft een naam die deze versie van Windows als apparaat opent en niet als bestand" | De app heeft een bestand met een naam zoals `aux.h`, dat Windows vóór Windows 11 niet kan bevatten | Installeer op Windows 11 |
| Een gegevensmap zoals `%AppData%` ligt achter een koppeling | Het installatieprogramma schrijft niet via een koppeling | Vink **Gebruikersgegevens** en **Licentie** uit, en installeer zonder die gegevens |

Meer staat in [Problemen oplossen](Troubleshooting#installing).
