# Review: French, France (fr-FR)

Reviewer: independent machine pass, 2026-11-15. Translator's form of address: **vous**, with imperatives in the vous
form. Kept, because Windows 10 and 11 in French use vous ("Voulez-vous autoriser cette application à apporter des
modifications à votre appareil ?"). The typography was already French: a no-break space before `:` `;` `?` `!` and
inside `« »`, and units as Windows writes them (octet, Ko, Mo, Go, To). The new text follows the same rules.

## Brought up to date
30 messages from the English delta, plus the 2 sentences that take the new start words:

- **Rewritten to match the new English (11):** `app-scan-failed` (a colon now comes before `{ $error }`, which starts
  in lower case, so `core-scan-no-executable` now reads correctly after it); `core-footprint-licenses-left-out`,
  `core-review-note-licenses`, `core-secret-why-license-option-off`, `core-warning-scan-license-data` ("l'option
  Licence" became "l'option Données de licence et d'activation", the same words as `app-option-license`);
  `core-warning-build-recorded-folder-left-out`, `core-warning-build-near-part-left-out` (`[unreadable]` said only "it
  cannot be read". It now says "son chemin sort du lecteur ou passe par un lien, ce qui ne permet pas de le/la lire en
  toute sécurité": a safety rule, not a read error); `core-review-com-support-detail` (was "respectively N, N and N
  settings"; each count now names its kind: type libraries, interfaces, AppIDs); `installer-license-detail` ("clés",
  which reads as product keys, became "valeurs de Registre qui contiennent des licences"); `installer-no-programdata`,
  `installer-cli-no-programdata` (said the install could not be undone; now say nothing was installed, because an
  install keeps the record that undoes it in ProgramData).
- **Added (10):** `core-review-start-boot`, `-system`, `-automatic`, `-delayed`, `-manual`, `-disabled`, `-unknown`;
  `core-review-network-domain`, `-private`, `-public` (domaine, privé, public).
- **Changed so the new words fit (2, not in the delta):** `core-review-service-detail` `[other]` and
  `core-review-driver-detail-starts` used to print the raw value after "type de démarrage :". They now end in
  `{ $start }`, and each start word carries its own verb: "…, démarre avec Windows, avant tout le reste", "…, se lance
  à chaque démarrage de ce PC", "…, ne démarre jamais, car il est désactivé". The verb is in the start words because
  French needs *ne* before *jamais* ("démarre jamais" is wrong), so it cannot stay in the sentence. `il` fits both
  a service and a driver (both masculine). `core-review-firewall-detail` already said "sur les réseaux de type
  { $networks }", which reads correctly with the new words ("sur les réseaux de type privé, public").
- **Already right, checked and left alone (9):** `app-footprint-licenses-still-go` (already named the full option);
  `client-builder-done`, `client-wrong-code`, `client-pair-failed` (already started with "Un autre PC");
  `core-rollback-setting-put-back`, `core-rollback-list-entries-removed`, `core-rollback-list-entries-not-removed`,
  `core-rollback-firewall-removed` (the English only fixed a capital "The"; the French uses past-participle phrases
  and was never affected); `ui-disclosure` (already present, with Narrator's "développé" and "réduit").

All rendered with sample values (each start word in both sentences; firewall rules with 0, 1, and 3 profiles and 0, 1,
and 2 ports) and read through.

## Sample (back-translation)
77 messages: all of `ui.ftl` (8) and `winappmigrator.ftl` (6), the first 25 of `installer.ftl`, its 4 plural
messages, 19 from `app.ftl`, and 15 from the `core*.ftl` files.

| Message | Back-translation | Verdict |
| --- | --- | --- |
| ui-clear-search | Clear the search | OK |
| ui-browse | Browse | OK (Windows' word) |
| ui-browse-tooltip | Choose a folder for « { $field } » | OK |
| ui-disclosure | { $title }, { $summary }, expanded / collapsed | OK (Narrator's words for the two states) |
| ui-language | Language | OK |
| ui-machine-translated | This translation was produced automatically and may contain errors. | OK |
| ui-report-translation-problem | Report a translation problem | OK |
| ui-needs-direct3d-12 | This program needs Direct3D 12 to display its window. An old graphics driver or a virtual machine may not support it. | OK |
| size-bytes | { $value } byte / { $value } bytes | OK (French `one` covers 0 and 1, so "0 octet" is right) |
| size-kb | { $value } KB (Ko) | OK |
| size-mb | { $value } MB (Mo) | OK |
| size-gb | { $value } GB (Go) | OK |
| size-tb | { $value } TB (To) | OK |
| list-separator | ", " | OK |
| installer-window-title | WinAppMigrator's installation program | OK |
| installer-window-title-app | Install { $app_name } | OK |
| installer-tagline | Reinstalls an application captured on another PC | OK |
| installer-unnamed-app | Application | OK |
| installer-window-failed | The installation program's window could not open: { $error } | OK |
| installer-window-failed-without-one | It can still install without a window: run this file from a command prompt opened as administrator with --apply, and --target-root followed by a folder to choose the location. | OK |
| installer-checking | Checking the installation program… | OK |
| installer-stub-no-package | This is a WinAppMigrator installation program stub, and it contains no application package. Build an installation program with WinAppMigrator, then run the produced .exe here. | OK |
| installer-check-stopped | The check of the installation program stopped. | OK |
| installer-locate-failed | Unable to locate the installation program: { $error } | OK |
| installer-read-failed | Unable to read the installation program: { $error } | OK |
| installer-cannot-be-used-here | This installation program's package cannot be used on this PC: { $error } | OK |
| installer-captured-from | Captured on | OK (feminine, agreeing with the app the card is about) |
| installer-install-location | Install location | OK |
| installer-choose-location | Choose the install location | OK |
| installer-folder-has-files | This folder already contains files. Files with the same name are replaced, and are put back if you uninstall. | OK |
| installer-room-needed | Space required | OK |
| installer-room-short | Space may be insufficient here: { $shortfall }. The package can be unpacked on another drive, but the app's files must still fit here. | OK |
| installer-interrupted | An earlier install of { $app_name } never completed. It will be undone first when you click Install. | OK |
| installer-signature | Installation program | OK |
| installer-signature-checking | Checking its signature… | OK |
| installer-signature-unsigned | Not signed: unknown publisher | OK ("Éditeur inconnu", as Windows says) |
| installer-signature-signed-by | Signed by { $publisher } | OK |
| installer-signature-unnamed | Signed by a publisher with no name | OK |
| installer-signature-invalid | This installation program's signature is not valid ({ $reason }). It may have been modified since it was signed. | OK |
| installer-more-left-out | { $count } other item was left out / { $count } other items were left out | OK |
| uninstaller-plan-coming-back | { $count } file(s) replaced by this install is/are put back, or placed beside a file that has changed since. | OK |
| uninstaller-remove-user-data-detail | { $count } item(s) of its settings, data and license. It stays / they stay unless this option is on. What it/they replaced is put back, or placed beside it/them if it/they changed since. | OK |
| installer-result-changes-more | and { $count } other. / and { $count } others. | OK (French adds the plural English lacks) |
| app-reload-list | Reload the list of applications | OK |
| app-list-search | Search by name or publisher  (Ctrl+F) | OK (French Windows also writes Ctrl) |
| app-column-name | Name | OK |
| app-column-publisher | Publisher | OK |
| app-column-version | Version | OK |
| app-column-scope | Scope | OK |
| app-column-arch | Arch. | OK (abbreviated, as short as English) |
| app-source | Source of the scan | OK |
| app-option-license | License and activation data | OK |
| app-option-user-data | User data | OK |
| app-option-whose-data | Account of the data | OK |
| app-output-folder | Output folder | OK |
| app-build-create | Create the installation program | OK (longer than English, but it is a full-width button) |
| app-build-cancel | Cancel | OK |
| app-scan-look-again | Re-examine | OK |
| app-scan-try-again | Try again | OK |
| app-network-connect | Connect | OK |
| app-result-copy-path | Copy the path | OK |
| app-scan-failed | This application could not be examined. { $error } | Fixed: delta; colon before `{ $error }`, as the English now has |
| core-rollback-drive-not-connected | What this install placed in { $folder } is on a drive that is not connected. Connect it, and this operation can be finished. | OK |
| core-rollback-folder-not-removed | The folder { $path } could not be deleted: { $error }. | OK |
| core-rollback-service-changed | The driver/service { $name } stays in place, because it has changed since it was installed. | OK |
| core-scan-no-executable | unable to find an executable for { $app_name }. A driver, runtime, or add-in may have no program of its own to package. | OK (lower case, follows the colon of `app-scan-failed`) |
| core-discovery-root-not-a-folder | the folder to write in is not a folder: { $path } | OK |
| core-error-invalid-input | invalid input: { $detail } | OK |
| core-journal-nothing-exists | nothing of this exists on this PC: { $path } | Fixed: "rien de ceci" is unnatural French; now "rien de tout cela" |
| core-restore-not-all-put-back | { $error }. What had been done could not be entirely put back: { $failed } | OK |
| core-restore-file-changed | { $path } no longer matches the package's content, so it was not installed | OK |
| core-restore-would-restore-registry-value | would restore the registry value { $key } { $value } | OK |
| core-policy-fit-driver | its driver { $driver } is built for { $built_for }, and this PC is of type { $here }. A driver has to match Windows exactly, so this app cannot run here | OK |
| core-space-not-told | { $drive } did not give its free space, so the room for the { $needed } needed was not checked | OK |
| core-review-event-log-source-detail | Can write to the { $log } log, with messages from its own files | OK |
| core-review-task-detail-highest | Runs as { $account }, with the highest privileges | OK ("autorisations maximales", Task Scheduler's own words) |
| core-review-other-settings-detail-more | In { $keys } and { $count } other(s) | OK |

Also checked across the whole language: the `app-column-*` headers are 3 to 7 letters ("Arch." abbreviated); the
`core-review-kind-*` labels use Windows' names ("Source de données ODBC", "Applications par défaut"); nothing is left
in English except the two lines that are the same in French; "Registre", "Pare-feu Windows", "programme
d'installation", "PC du réseau", "code d'association", and "Réexaminer" are used the same way everywhere; no plain
space before French punctuation remains.

## Other fixes
- `core-journal-nothing-exists`: "rien de ceci n'existe" became "rien de tout cela n'existe", the natural phrasing.
- Comments brought in line with English by running `ftl-tool.py merge` (the start-word, firewall, COM, rollback, and
  `core-error-io` notes). No translated text was changed by it.
- `terms.md`: added the firewall profile words, and a note on why the start words carry their own verb.

## Open questions
- **Firewall profile names:** "domaine", "privé", "public" are taken as the Profile column of Windows Defender Firewall
  with Advanced Security shows them; the Windows Security app says "Réseau avec domaine", "Réseau privé", "Réseau
  public". A native speaker with French Windows should confirm, and that "sur les réseaux de type domaine, privé,
  public" (joined by commas, no "et") reads well.
- **Start words:** check that "se lance à chaque démarrage de ce PC" and "démarre avec Windows, pendant son lancement"
  read naturally after "Exécute svc.exe en tant que LocalSystem, …" and "Charge disk.sys dans Windows même, …".
- `core-rollback-count-kept` "Laissés en place" and `core-rollback-count-left-behind` "Restants" (the translator
  flagged these too): the English comments now say kept means left on purpose and left behind means no uninstall can
  take it back (such as a runtime). "Restants" is vague next to "Laissés en place"; a native speaker may want a
  clearer pair, such as "Restent sur ce PC".
- Long labels to look at in the window: `app-network-look-for-usb-again` "Rechercher à nouveau les lecteurs USB" and
  the uninstaller checkbox `uninstaller-remove-user-data` "Supprimer aussi les données utilisateur et les données de
  licence" are about 1.5 to 1.7 times the English.
- A placeholder cannot take elision: "ce que { $app_name } déclare" gives "ce que Acme", where French would write
  "ce qu'Acme". Unavoidable without the code knowing the name's first letter.
- For the English (raised by vi-VN): `core-rollback-setting-other-account` `[machine-variable]` and `[machine-list]`
  say a PC-wide variable "belongs to the account that installed this". The French says the same; it follows the
  English if that changes.

Checker: fr-FR: 0 errors, 2 warnings (`app-list-row` and `app-details-publisher-and-version`, "{ }, version { }",
which is the same in French).
