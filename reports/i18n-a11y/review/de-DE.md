# Review: German (de-DE)

Reviewer: independent machine pass, 2026-11-15. Translator's form of address: „Sie“, with imperatives in the
„Wählen Sie …“ form and infinitive labels on buttons („Installationsprogramm erstellen“); kept, because that is how German
Windows 10 and 11 address the user.

## Brought up to date

30 messages checked against the current English. 22 needed new or changed words:

- `app-scan-failed`: the reason now follows a colon, not a full stop („Diese App konnte nicht analysiert werden: …“).
- `app-footprint-licenses-still-go`, `core-footprint-licenses-left-out`, `core-review-note-licenses`,
  `core-secret-why-license-option-off`, `core-warning-scan-license-data`: „die Option „Lizenz““ is now
  „die Option „Lizenz- und Aktivierungsdaten““, the words of the checkbox `app-option-license`.
- `core-warning-build-recorded-folder-left-out`, `core-warning-build-near-part-left-out` `[unreadable]`: was "it cannot
  be read"; now the safety rule: „sein Pfad verlässt das Laufwerk oder führt über einen Link, daher kann er nicht
  gefahrlos gelesen werden“ („gefahrlos“, not „sicher“, which would also read as "certainly").
- `core-review-start-boot`, `-system`, `-automatic`, `-delayed`, `-manual`, `-disabled`, `-unknown` (new): „mit Windows,
  vor allem anderen“, „mit Windows, während des Hochfahrens“, „bei jedem Start dieses PCs“, „kurz nach jedem Start dieses
  PCs“, „nur, wenn er angefordert wird“, „nie, da er deaktiviert ist“, „auf eine Weise, die Windows nicht angibt“. So that
  they read as the end of the sentence, `core-review-service-detail` `[other]` and `core-review-driver-detail-starts` now
  end „…, startet { $start }“ instead of „…, Start: { $start }“ (which was built for a raw Windows value such as "boot").
- `core-review-network-domain`, `-private`, `-public` (new): „Domäne“, „Privat“, „Öffentlich“, as the Profile column of
  Windows Defender Firewall shows them. They fill „in Netzwerken vom Typ { $networks }“ in `core-review-firewall-detail`,
  which reads correctly with them („… für app.exe, in Netzwerken vom Typ Domäne, Privat“).
- `core-review-com-support-detail`: each count now follows its own kind, as in English („Einstellungen für die
  Typbibliotheken (3), Schnittstellen (5) und AppIDs (1), die zu den Klassen gehören“), instead of „3, 5 bzw. 1
  Einstellungen“.
- `installer-license-detail`: „-schlüssel“ (which could read as product keys) is now „die Registrierungswerte, in denen
  Lizenzen gespeichert sind“.
- `installer-no-programdata`, `installer-cli-no-programdata`: was "so the install could not be undone"; now „daher wurde
  nichts installiert: Eine Installation legt dort die Aufzeichnung ab, mit der sie rückgängig gemacht wird“.

8 already said what the current English says: `client-builder-done`, `client-wrong-code`, `client-pair-failed` („Ein
anderer PC“ already starts the sentence with a capital), `core-rollback-setting-put-back`,
`core-rollback-list-entries-removed`, `core-rollback-list-entries-not-removed`, `core-rollback-firewall-removed` (German
never had the capital "The"), `ui-disclosure`.

The files were then run through `ftl-tool.py merge`, which changed only `#` comments, bringing them up to the current
English ones; no message text moved or changed.

## Sample (back-translation)

75 messages: all of `ui.ftl` and `winappmigrator.ftl`, the first 25 of `installer.ftl`, its 3 plural messages, 20 from
`app.ftl`, and 13 from the `core*.ftl` files with placeholders.

| Message | Back-translation | Verdict |
| --- | --- | --- |
| ui-clear-search | Clear search | OK |
| ui-browse | Browse | OK |
| ui-browse-tooltip | Choose folder for "{field}" | OK |
| ui-disclosure | {title}, {summary}, opened / closed | OK (see open questions) |
| ui-language | Language | OK |
| ui-machine-translated | This translation was created by machine and may contain errors. | OK |
| ui-report-translation-problem | Report translation problem | OK |
| ui-needs-direct3d-12 | Direct3D 12 is needed to draw the window. An old graphics driver or a virtual machine may not offer it. | OK |
| size-bytes | {value} byte / {value} bytes | OK |
| size-kb, size-mb, size-gb, size-tb | {value} KB / MB / GB / TB | OK (same symbols in German) |
| list-separator | ", " | OK |
| installer-window-title | WinAppMigrator installer | OK |
| installer-window-title-app | Install {app} | OK |
| installer-tagline | Reinstalls an application that was captured on another PC | OK |
| installer-unnamed-app | Application | OK |
| installer-window-failed | The installer's window could not be opened: {error} | OK |
| installer-window-failed-without-one | Installing also works without a window: run this file with --apply in a command prompt with administrator rights. With --target-root and a folder you set where it installs. | OK |
| installer-checking | Checking the installer… | OK |
| installer-stub-no-package | This is a WinAppMigrator installer stub that holds no application package. Create an installer with WinAppMigrator, then run the created .exe file here. | OK |
| installer-check-stopped | The check of the installer was stopped. | OK |
| installer-locate-failed | The installer was not found: {error} | OK |
| installer-read-failed | The installer could not be read: {error} | OK |
| installer-cannot-be-used-here | This installer's package cannot be used on this PC: {error} | OK |
| installer-captured-from | Captured on | OK |
| installer-install-location | Install location | OK |
| installer-choose-location | Choose install location | OK |
| installer-folder-has-files | This folder already holds files. Files with the same name are replaced, and restored by an uninstall. | OK |
| installer-room-needed | Space needed | OK |
| installer-room-short | There may not be enough space here: {shortfall}. The package can be unpacked on another drive, but the app's files still have to fit here. | OK |
| installer-interrupted | An earlier install of {app} was never finished. "Install" undoes it first. | OK |
| installer-signature | Installer | OK |
| installer-signature-checking | Checking signature… | OK |
| installer-signature-unsigned | Not signed: publisher unknown | OK |
| installer-signature-signed-by | Signed by {publisher} | OK |
| installer-signature-unnamed | Signed by an unnamed publisher | OK |
| installer-signature-invalid | This installer's signature is invalid ({reason}). It may have been changed after signing. | OK |
| installer-more-left-out | [one] {n} more item was left out / [other] {n} more items were left out | OK |
| uninstaller-plan-coming-back | [one] {n} file this install replaced is restored, or put beside a file changed since / [other] same, plural | OK |
| uninstaller-remove-user-data-detail | [one] {n} item of the app's settings, data, and license. It stays unless this option is on. What it replaced is restored, or put beside it if it has changed since. / [other] same, plural | OK |
| app-column-name | Name | OK |
| app-column-publisher | Publisher | OK (Windows' „Herausgeber“) |
| app-column-version | Version | OK |
| app-column-scope | Scope | OK, short |
| app-column-arch | Arch. | OK, abbreviated for the narrow column |
| app-column-arch-sort | Sort by architecture | OK |
| app-list-search | Search by name or publisher (Ctrl+F) | Fixed: „Ctrl+F“ is now „Strg+F“, as German keyboards and Windows label the key |
| app-option-license | License and activation data | OK |
| app-option-user-data | User data | OK |
| app-option-whose-data | Whose data | OK |
| app-build-create | Create installer | OK |
| app-build-create-again | Create installer again | OK |
| app-build-cancel | Cancel | OK |
| app-scan-look-again | Analyze again | OK (terms.md) |
| app-scan-stop | Stop | OK |
| app-scan-try-again | Try again | OK |
| app-result-copy-error | Copy error | OK |
| app-result-open-folder | Open folder | OK |
| app-network-connect | Connect | OK |
| app-scan-failed | This app could not be analyzed. {error} | Fixed: colon before the reason, as the current English (the reasons start in lower case) |
| core-space-shortfall | {drive} needs up to {needed} free space for {purposes} and has {free} | OK, also with two purposes joined by „und“ |
| core-review-service-detail | Runs {file} as {account}, starts now and at every start of this PC / …, starts only when something requests it / …, Start: {start} | Fixed: `[other]` now „startet { $start }“ to take the new start words; `[manual]` „wenn etwas es anfordert“ had the wrong pronoun for „Dienst“, now „wenn er angefordert wird“ |
| core-review-driver-detail-starts | Loads {file} into Windows itself, Start: {start} | Fixed: „startet { $start }“, as above |
| core-review-firewall-detail | Allows incoming connections on port {ports} for {program}, in networks of type {networks} | OK with the new profile words |
| core-review-com-support-detail | 3, 5, and 1 settings respectively that belong to the classes | Fixed (brought up to date, above) |
| core-warning-build-recorded-folder-left-out | Something on record for this app names a folder that was left out, because {reason} | Fixed `[unreadable]` (above); the other reasons OK |
| core-warning-build-near-part-left-out | Part of what {app} states as "own" was left out: {reason} | Fixed: „als eigen angibt“ is not idiomatic, now „als zugehörig angibt“; each reason after the colon is a full sentence, so it now starts with a capital, as elsewhere in the catalog; `[unreadable]` updated |
| core-rollback-file-stays-beside | {path} stays, because something that also stays still uses the file. The file it replaced lies beside it as {name}. | OK |
| core-warning-restore-service-account | The service {name} ran as {account} on the source PC. Its password is not in the installer, so the service is set up as LocalService. If it does not work, set account and password under "Services". | OK |
| core-policy-fit-needs-newer | {program} says it needs Windows {needs} or later, and this PC runs Windows {runs}, so Windows would not start the program | OK |
| core-warning-scan-package-mentions | This package mentions an account's name at these places: {places}. It was built with none of an account's data, and anyone who has the installer can read this. | OK |
| core-restore-left-variable | User environment variable left unchanged: {name}, since none of it belongs to this app | OK |
| core-warning-review-documents-elsewhere | [one] {n} file from "Documents" is put under {target} instead, in a folder named _external, because this PC's "Documents" folder is at {folder} and only a folder inside your profile is written to. To use it, move it to your "Documents" folder yourself. | OK |

## Other fixes

- `core-warning-build-credential-by-now`: „Nicht in das Paket aufgenommen: es sieht …“ → „… : Es sieht …“; a full
  sentence after a colon takes a capital, as everywhere else in the catalog.
- `terms.md`: the License row now separates the installer's checkbox („Lizenz“) from the app's option, which every text
  names „Option „Lizenz- und Aktivierungsdaten““; added Strg / Strg+F and the firewall profiles Domäne, Privat,
  Öffentlich.
- Whole-language checks: the `app-column-*` headers are short (Name, Herausgeber, Version, Bereich, Arch.); button labels
  are short infinitives; nothing is left in English except product names, flags, `ProgramData`/`Program Files (x86)`,
  and the two checker warnings below, which are the same in German.

## Open questions

- Firewall profiles: „Domäne“, „Privat“, „Öffentlich“ are the Profile column words of Windows Defender Firewall with
  Advanced Security, from memory, not confirmed on a German Windows. A native speaker may prefer the network-profile form
  („im Netzwerkprofil Privat“, „in den Profilen Domäne, Privat“) over „in Netzwerken vom Typ Domäne, Privat“.
- `ui-disclosure`: German Narrator announces expand/collapse state as „erweitert“ / „reduziert“. The translation follows
  the English "open"/"closed" with „geöffnet“ / „geschlossen“; worth a native check against what Narrator says for other
  disclosure headings.
- Service and driver start words: „nur, wenn er angefordert wird“ and „nie, da er deaktiviert ist“ rely on the line
  standing under „Dienst“ or „Treiber“ (both masculine). That holds for the two kinds that use them.
- Several app messages that English starts in lower case (`app-network-wrong-code`, `app-network-not-reached`,
  `app-client-could-not-save`, `app-client-not-carried`) start with a capital in German. They are shown on their own, so
  that is right; but `terms.md` says inserted reasons start lower case, so check them if the app ever puts them after a
  colon.

Checker: de-DE: 0 errors, 2 warnings (`core-review-com-title` „COM in { $server }“ and `core-review-certificate`
„{ $name } in { $store }“, which are the same in German).
