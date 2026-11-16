# Review: Dutch (nl-NL)

Reviewer: independent machine pass, 2026-11-15. Translator's form of address: informal *je/jouw*, with imperatives for
actions ("Kies", "Sluit", "Voer … in") and infinitives for buttons, menu items and progress lines ("Installeren",
"Map openen", "Bestanden kopiëren…"); kept, because that is how Dutch Windows 10 and 11 and Microsoft's Dutch style
guide address the user. Sentence case and lowercase "pc" ("Deze pc") also kept, as Windows writes them.

## Brought up to date

30 messages checked against the current English. 22 needed new words:

- `app-scan-failed`: the reason now follows a colon, not a full stop ("Deze app kan niet worden bekeken: …"). The
  Dutch scan errors that fill it already start in lowercase.
- `client-pair-failed`: "kan niet koppelen" (cannot pair) is now "kon niet koppelen" (could not pair), since it reports
  one attempt that failed.
- `core-footprint-licenses-left-out`, `core-review-note-licenses`, `core-secret-why-license-option-off`,
  `core-warning-scan-license-data`: "de optie Licentie" is now "de optie Licentie- en activeringsgegevens", the same
  words as the checkbox `app-option-license`.
- `core-warning-build-recorded-folder-left-out`, `core-warning-build-near-part-left-out` `[unreadable]`: was "it may
  not be readable"; now the safety rule: "het pad ervan het station verlaat of via een koppeling loopt, zodat die niet
  veilig kan worden gelezen" (subordinate order after "omdat"), and "het pad ervan verlaat het station of loopt via een
  koppeling, dus het kan niet veilig worden gelezen" (main-clause order after the colon).
- `core-review-start-boot`, `-system`, `-automatic`, `-delayed`, `-manual`, `-disabled`, `-unknown` (new): "met
  Windows, vóór al het andere", "met Windows, tijdens het opstarten", "telkens wanneer deze pc opstart", "telkens kort
  nadat deze pc is opgestart", "alleen wanneer iets erom vraagt", "nooit, omdat het is uitgeschakeld", "op een manier
  die Windows niet vermeldt". The first five match the wording already used in `core-review-service-detail`'s own
  variants. The frames they fill, `core-review-service-detail` `[other]` and `core-review-driver-detail-starts`, said
  "starttype { $start }" (written for a raw value such as "boot"); both now say "start { $start }", so the line reads
  "Laadt disk.sys in Windows zelf, start met Windows, vóór al het andere".
- `core-review-network-domain`, `-private`, `-public` (new): "domein", "privé", "openbaar", as Windows Defender
  Firewall met geavanceerde beveiliging names its profiles (Domeinprofiel, Privéprofiel, Openbaar profiel), lowercase
  because they fall mid-sentence: "…voor app.exe, op netwerken van het type domein, privé". The sentence frame in
  `core-review-firewall-detail` reads correctly with them and was kept.
- `core-review-com-support-detail`: was "Respectievelijk 2, 5 en 1 instellingen" (respectively); each count now sits
  after its own kind, as in English: "Instellingen voor de typebibliotheken (2), interfaces (5) en AppID's (1), die bij
  de klassen horen".
- `installer-license-detail`: "en -sleutels" (could read as product keys) is now "en de registerwaarden waarin
  licenties zijn opgeslagen".
- `installer-no-programdata`, `installer-cli-no-programdata`: was "so the install could not be undone"; now "dus er is
  niets geïnstalleerd: een installatie bewaart daar het installatieverslag waarmee ze ongedaan wordt gemaakt".

8 already said what the current English says: `app-footprint-licenses-still-go` (already used the full option name),
`client-builder-done`, `client-wrong-code` ("Een andere pc" already started the sentence), `core-rollback-setting-put-back`,
`core-rollback-list-entries-removed`, `core-rollback-list-entries-not-removed`, `core-rollback-firewall-removed` (the
English fix was a lowercase "the"; Dutch never had the capital), `ui-disclosure` ("uitgevouwen" / "samengevouwen", as
Narrator says them in Dutch).

The `#` comments of these messages, and of `core-error-io`, `core-rollback-count-kept`, `core-rollback-count-left-behind`
(whose English comments also changed), now match English; the stale "Look Again" comment above `app-scan-stopped` is gone.

## Sample (back-translation)

77 messages. Back-translated from the Dutch before looking at the English.

| Message | Back-translation | Verdict |
| --- | --- | --- |
| ui-clear-search | Clear search query | OK |
| ui-browse | Browse | OK (Windows' "Bladeren") |
| ui-browse-tooltip | Choose a folder for { $field } | OK |
| ui-disclosure | { $title }, { $summary }, expanded / collapsed | OK (Narrator's words) |
| ui-language | Language | OK |
| ui-machine-translated | This translation was made automatically and may contain errors. | OK |
| ui-report-translation-problem | Report a translation problem | OK |
| ui-needs-direct3d-12 | This program needs Direct3D 12 to draw the window. An old graphics driver or a virtual machine may not have that. | OK |
| size-bytes | { $value } byte / bytes | OK |
| size-kb | { $value } KB | OK |
| size-mb | { $value } MB | OK |
| size-gb | { $value } GB | OK |
| size-tb | { $value } TB | OK |
| list-separator | ", " | OK |
| installer-window-title | WinAppMigrator installer | OK |
| installer-window-title-app | Install { $app_name } | OK |
| installer-tagline | Reinstalls an app that was captured on another PC | OK |
| installer-unnamed-app | App | OK (terms.md: app for application) |
| installer-window-failed | The installer's window cannot be opened: { $error } | OK |
| installer-window-failed-without-one | Installing also works without a window: run this file from a command prompt as administrator with --apply, and with --target-root followed by a folder to choose the location yourself. | OK |
| installer-checking | Checking the installer… | OK |
| installer-stub-no-package | This is a WinAppMigrator installer stub and it contains no app package. Make an installer with WinAppMigrator and run the .exe file made here. | OK |
| installer-check-stopped | The check of the installer has stopped. | OK |
| installer-locate-failed | Cannot find the installer: { $error } | OK |
| installer-read-failed | Cannot read the installer: { $error } | OK |
| installer-cannot-be-used-here | The package of this installer cannot be used on this PC: { $error } | OK |
| installer-captured-from | Comes from | OK |
| installer-install-location | Install location | OK |
| installer-choose-location | Choose the install location | OK |
| installer-folder-has-files | This folder already contains files. Files with the same name are replaced and come back if you remove the app. | OK |
| installer-room-needed | Space needed | OK |
| installer-room-short | There may not be enough space here: { $shortfall }. The package can be unpacked on another drive, but the app's files still have to fit here. | OK |
| installer-interrupted | An earlier installation of { $app_name } was never completed. Installing undoes it first. | OK |
| installer-signature | Installer | OK |
| installer-signature-checking | Checking signature… | OK |
| installer-signature-unsigned | Not signed: publisher unknown | OK |
| installer-signature-signed-by | Signed by { $publisher } | OK |
| installer-signature-unnamed | Signed by a nameless publisher | OK |
| installer-signature-invalid | The signature of this installer is not valid ({ $reason }). It may have been changed after it was signed. | OK |
| installer-more-left-out | { $count } more item was / items were left out | OK (plural) |
| uninstaller-plan-coming-back | { $count } file(s) this installation replaced come(s) back, or is placed beside a file that has changed since. | OK (plural) |
| uninstaller-remove-user-data-detail | { $count } item(s) of the app's settings, data and license. It stays / they stay unless this is on. What it/they replaced comes back, or is placed beside it/them if changed since. | OK (plural) |
| installer-dependency-winget | Install with winget: { $id } | Fixed: read as an instruction or button beside "Handmatig installeren"; it says how the dependency is provided, so now "Wordt geïnstalleerd met winget: { $id }" (is installed with winget) |
| app-column-name | Name | OK |
| app-column-publisher | Publisher | OK |
| app-column-version | Version | OK |
| app-column-scope | Scope | OK |
| app-column-arch | Arch. | OK (short, abbreviation marked) |
| app-column-arch-sort | Sort by architecture | OK |
| app-list-search | Search by name or publisher  (Ctrl+F) | OK (Dutch Windows writes Ctrl) |
| app-source | Scan source | OK |
| app-option-license | License and activation data | OK |
| app-option-user-data | User data | OK |
| app-option-whose-data | Whose data | OK |
| app-output-folder | Output folder | OK |
| app-build-create | Create installer | OK |
| app-build-cancel | Cancel | OK |
| app-scan-look-again | Look again | OK |
| app-scan-try-again | Try again | OK |
| app-result-open-folder | Open folder | OK |
| app-result-copy-path | Copy path | OK |
| app-result-copy-error | Copy wrongly / copy error | Fixed: "Fout kopiëren" also reads as "copy incorrectly"; now "Foutmelding kopiëren" (copy error message) |
| app-network-connect | Connect | OK |
| core-warning-shortcuts-unreadable | { $count } shortcut(s) on this drive point to no file, so it could not be determined whether it/they open this app. | OK |
| core-rollback-file-stays-not-beside | { $path } stays, because something that stays still uses it / it changed since install / removal was not asked, and the file it replaced cannot be put beside it: { $error }. | OK |
| core-rollback-firewall-says-otherwise | The firewall rule { $name } stays, because the rule with that name contains something else. | OK |
| core-runtime-feature-timed-out | enabling the Windows feature { $feature } did not finish within the time-out and was ended | OK |
| core-profile-several-desktops | more than one account has a desktop in this session: { $accounts } | OK |
| core-scan-program-not-on-the-drive | { $path } goes through a link or is not on the connected drive, so it was not read. | OK |
| core-archive-data-file-wrong | { $name } is not this installer's data file, or it is damaged or changed after it was made, so nothing was read from it. Copy both files again from where they were made. | OK |
| core-error-io | I/O failed for { $path }: { $error } | OK (reads right with "het installatieprogramma" too) |
| core-restore-hash-check-failed-because | the package failed the hash check and nothing was installed: { $detail } | OK |
| core-warning-restore-commit-failed | The install record cannot be completed: { $error } | OK |
| core-review-cannot-run-here | this app cannot run on this PC, so nothing was changed: { $reason } | OK |
| core-warning-review-documents-unknown | { $count } file(s) from Documents go(es) under { $target } instead, in a folder named _external, because where this PC's Documents folder is could not be determined. To use it/them, move it/them to your Documents folder yourself. | OK |
| core-verify-driver-image-unchecked | Driver file cannot be checked: { $error } | OK |
| core-review-path-title | Adds to { $name } | OK |

Whole-language checks: the `app-column-*` headers are short (Naam, Uitgever, Versie, Bereik, Arch.). Button labels are
the usual Dutch infinitives; the longest, "Installatieprogramma opnieuw maken", is natural Dutch and cannot be cut
without losing sense. A scan of every message for English words found none left, apart from command lines and file
names that must stay (`winget install`, `dism …`, `winappmigrator-installer.exe`).

## Other fixes

- `installer-dependency-winget` and `app-result-copy-error`, as in the table.
- `terms.md`: added the firewall profile names (domein, privé, openbaar) and the service start words.
- Comments brought in line with English, as listed above.

## Open questions

- Firewall profiles: Windows Defender Firewall met geavanceerde beveiliging says "Domein, Privé, Openbaar", but the
  Settings app and Windows-beveiliging say "Particulier netwerk" / "Openbaar netwerk". "privé" follows the brief (the
  firewall's own names); a native speaker should confirm it reads well in "op netwerken van het type domein, privé".
- `core-review-start-boot` and `-system` describe driver start types that Windows shows in no Dutch window, so they are
  descriptions ("met Windows, vóór al het andere"), not Windows labels. Worth a native read.
- "Installatieprogramma opnieuw maken" and "Ook gebruikersgegevens en licentiegegevens verwijderen" are much longer than
  the English; check they are not cut off in the window.
- The `installer-cli-*` and `uninstaller-cli-*` lines are printed in English whatever the language (scripts read them),
  so their Dutch is only seen if that changes.

Checker: nl-NL: 0 errors, 2 warnings (`core-review-com-title` "COM in { $server }" and `core-review-certificate`
"{ } in { }": the Dutch really is the same as the English).
