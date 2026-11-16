# Review: Spanish, Spain (es-ES)

Reviewer: independent machine pass, 2026-11-15. Translator's form of address: **tú**. Kept, because Windows 10 and 11
in Spanish (Spain) use tú ("¿Quieres permitir que esta aplicación haga cambios en el dispositivo?"). Two sentences used a
bare "puede", which a reader could take as the formal usted. They are now impersonal (see Other fixes).

## Brought up to date
30 messages from the English delta, plus the 2 sentences that take the new start words:

- **Rewritten to match the new English (13):** `app-scan-failed` (a colon now comes before `{ $error }`, which starts
  in lower case); `core-footprint-licenses-left-out`, `core-review-note-licenses`, `core-secret-why-license-option-off`,
  `core-warning-scan-license-data` ("la opción Licencia" became "la opción Datos de licencia y activación", the same
  words as `app-option-license`); `core-warning-build-recorded-folder-left-out`,
  `core-warning-build-near-part-left-out` (`[unreadable]` now says the path leaves the drive or goes through a link, so
  it cannot safely be read: a safety rule, not a read error); `core-review-com-support-detail` (each count now has its
  kind); `installer-license-detail` (registry values that hold licenses, not product keys); `installer-no-programdata`,
  `installer-cli-no-programdata` (nothing was installed, because the record that undoes an install is kept in
  ProgramData).
- **Added (10):** `core-review-start-boot`, `-system`, `-automatic`, `-delayed`, `-manual`, `-disabled`, `-unknown`;
  `core-review-network-domain`, `-private`, `-public`.
- **Changed so the new words fit (2, not in the delta):** `core-review-service-detail` `[other]` and
  `core-review-driver-detail-starts` used to print the raw value after "tipo de inicio:". They now end in
  `{ $start }`, and each start word carries its own verb ("…, se inicia con Windows, antes que todo lo demás"). The verb
  is in the start words because "se inicia nunca" is not correct Spanish. Disabled reads "no se inicia nunca, porque
  está deshabilitado". `core-review-firewall-detail` now says "en el perfil { $networks }" for 1 profile and
  "en los perfiles { $networks }" for more, so the profile names Windows uses read correctly.
- **Already right, checked and left alone (8):** `app-footprint-licenses-still-go`; `client-builder-done`,
  `client-wrong-code`, `client-pair-failed` (the translator had already started each with "Otro PC");
  `core-rollback-setting-put-back`, `core-rollback-list-entries-removed`, `core-rollback-list-entries-not-removed`,
  `core-rollback-firewall-removed` (the English only fixed a capital "The"); `ui-disclosure` (already
  "expandido"/"contraído", as Narrator says them).

## Sample (back-translation)
| Message | Back-translation | Verdict |
| --- | --- | --- |
| ui-clear-search | Clear search | OK |
| ui-browse | Browse | OK |
| ui-browse-tooltip | Choose a folder for { $field } | OK |
| ui-disclosure | { $title }, { $summary }, expanded / collapsed | OK (Narrator's words) |
| ui-language | Language | OK |
| ui-machine-translated | This translation was made automatically and may contain errors. | OK |
| ui-report-translation-problem | Report a translation problem | OK |
| ui-needs-direct3d-12 | Direct3D 12 is needed to draw the window, and an old graphics driver or a virtual machine may not have it. | OK |
| size-bytes | { $value } byte / bytes | OK |
| size-kb, size-mb, size-gb, size-tb | { $value } KB / MB / GB / TB | OK (4 messages) |
| list-separator | ", " | OK |
| installer-window-title | WinAppMigrator Installer | OK |
| installer-window-title-app | Install { $app_name } | OK |
| installer-tagline | Reinstalls an app captured on another PC | OK |
| installer-unnamed-app | Application | OK |
| installer-window-failed | The installer window could not be opened: { $error } | OK |
| installer-window-failed-without-one | Even so, you (formal?) can install without it: run this file from an administrator command prompt with --apply and, to choose where, with --target-root followed by a folder. | Fixed: "puede" could be read as usted beside the tú imperative; now impersonal "se puede instalar" |
| installer-checking | Checking the installer… | OK |
| installer-stub-no-package | This is the base program of a WinAppMigrator installer and holds no app package. Create an installer with WinAppMigrator and run the .exe it makes here. | OK |
| installer-check-stopped | The installer check stopped. | OK |
| installer-locate-failed | The installer could not be found: { $error } | OK |
| installer-read-failed | The installer could not be read: { $error } | OK |
| installer-cannot-be-used-here | This installer's package cannot be used on this PC: { $error } | OK |
| installer-captured-from | Captured on | OK |
| installer-install-location | Install location | OK |
| installer-choose-location | Choose the install location | OK |
| installer-folder-has-files | This folder already has files. Files with the same name are replaced, and go back to their place if you uninstall. | OK |
| installer-room-needed | Space needed | OK |
| installer-room-short | There may not be enough space here: { $shortfall }. It can be unpacked on another drive, but the app's files still have to fit. | OK |
| installer-interrupted | An earlier install of { $app_name } did not finish. Install undoes it first. | OK |
| installer-signature | Installer | OK |
| installer-signature-checking | Checking its signature… | OK |
| installer-signature-unsigned | Unsigned: unknown publisher | OK ("Editor desconocido" is Windows' own) |
| installer-signature-signed-by | Signed by { $publisher } | OK |
| installer-signature-unnamed | Signed by an unnamed publisher | OK |
| installer-signature-invalid | This installer's signature is not valid ({ $reason }). It may have been changed after it was signed. | OK |
| installer-location-app-already-there | { $app_name } is already installed in this folder, so this copy cannot go here. To install it here, first uninstall the copy on this PC in Windows Settings, under Apps, and open this installer again. Or type a new, empty folder above. | OK |
| installer-location-other-app-there | { $other_app } is installed in this folder, so { $app_name } cannot go here. Type a new, empty folder above. Or, if you no longer need { $other_app }, uninstall it first… | OK |
| installer-location-has-files | This folder already has files WinAppMigrator did not install. They may belong to another app, so nothing is installed over them. Type a new, empty folder above. | OK |
| installer-location-refused | This folder cannot be used: { $reason }. Type another folder above. | OK |
| installer-more-left-out (plural) | [one] { $count } more item was left out / [other] { $count } more items were left out | OK |
| uninstaller-plan-coming-back (plural) | { $count } file(s) this install replaced go back to their place, or are put beside a file that has changed since. | OK |
| uninstaller-remove-user-data-detail (plural) | { $count } item(s) of its settings, data and license. It stays / They stay unless this is on. What it/they replaced goes back, or is put beside it/them if changed since. | OK |
| uninstaller-window-failed-quiet | Even so, you (formal?) can uninstall without it, from an administrator command prompt: | Fixed: same as installer-window-failed-without-one |
| app-column-name | Name | OK |
| app-column-publisher | Publisher | OK |
| app-column-version | Version | OK |
| app-column-scope | Scope | OK |
| app-column-arch | Arch. | OK (short) |
| app-option-license | License and activation data | OK |
| app-option-user-data | User data | OK |
| app-option-whose-data | Whose the data are | OK |
| app-build-create | Create installer | OK |
| app-build-create-again | Create the installer again | OK |
| app-build-cancel | Cancel | OK |
| app-scan-look-again | Analyze again | OK (matches terms.md "Volver a analizar") |
| app-scan-stop | Stop | OK |
| app-scan-try-again | Try again | OK |
| app-result-copy-error | Copy error | OK |
| app-result-open-folder | Open folder | OK |
| app-result-copy-path | Copy path | OK |
| app-network-connect | Connect | OK |
| app-footprint-refused | No installer could set this up: | Fixed: "podría" said no installer *could*; English says no installer *would be allowed to*. Now "Ningún instalador tendría permiso para configurar esto:" |
| app-footprint-integration-nothing-left | Nothing is left, so you have left out (ambiguous "por lo que") | Fixed: now "debido a lo que has excluido" (because of what you left out) |
| core-rollback-file-left | The file { $path } was left: { $reason }. | OK |
| core-rollback-value-kept-by-request | The registry value { $name } stays, since it was not asked to be removed. | OK |
| core-rollback-task-not-put-back | The task { $name } could not be put back: { $reason }. | OK |
| core-system-registry-through-link | { $drive } is not a Windows system drive: the way to its registry goes through a link. | OK |
| core-runtime-tls-failed | error initializing TLS: { $error } | OK |
| core-profile-signed-in-unknown | Could not tell who is signed in ({ $reason }). | OK |
| core-discovery-root-not-a-folder | the folder that is written in is not a folder: { $path } | OK |
| core-image-unreadable | { $path } is not a .vhd or .vhdx file that can be read | OK |
| core-warning-build-user-folder-not-on-this-pc | { $user }'s { $folder } folder was at { $path }, which is not on a disk of this PC, so what is there was not included. | OK |
| core-warning-build-prerequisite-path-invalid | The included prerequisite { $file_name } could not be placed: { $error } | OK |
| core-restore-not-all-put-back | { $error }. Not everything that had been done could be put back: { $failed } | OK |
| core-restore-file-changed | { $path } is no longer what the package holds, so it was not installed | OK |
| core-restore-would-install-prerequisite-winget | the prerequisite { $name } would be installed (winget) | OK |
| core-warning-restore-left-out | { $item } was left out: { $reason }. | OK |
| core-warning-restore-prerequisite-not-verified | { $name } said it finished successfully but was not registered; the install may have failed. | OK |
| core-space-not-told | { $drive } does not say how much space it has, so whether the { $needed } it needs fit was not checked | OK |
| core-review-account-signed-in | User data and per-user settings go to { $account }, who is signed in here, not to the administrator this runs as. | OK |
| core-review-note-left-out-by-choice | Left out by whoever created this installer: { $names }. | OK |
| core-privilege-not-made-here | The folder { $path } was not created by WinAppMigrator, and others can change its contents. An administrator should check it and remove it: { $error } | OK |
| core-verify-variable-unreadable | The environment variable is missing or cannot be read: { $error } | OK |
| core-review-no-room | there is not enough space for this, so nothing was changed. { $shortfalls }. Free up space there or choose an install location on another drive. | OK |

89 messages in all (size-kb to size-tb counted as 4): 4 fixed, 85 OK. Fixes in the brought-up-to-date list are not
counted here.

Whole-language checks: the `app-column-*` headers are short ("Arq." for Arch). Button labels are short; the longest is
"Volver a crear el instalador". Nothing is left in English except product names, units and pure placeholders.
"Ctrl+F" stays, as Spanish Windows writes it. No usted forms remain.

## Other fixes
- `client-safe-folder-failed`: "no pudo iniciarse" changed to "no ha podido iniciarse", the pretérito perfecto the
  catalog uses everywhere else for what just happened (terms.md).
- `core-rollback-count-left-behind`: "Dejado atrás" was a calque. It is now "No reversible: { $count }" (what no
  uninstall can take back), which also stays clearly apart from `core-rollback-count-kept` ("Se queda en su lugar").
- Ran `ftl-tool.py merge` on es-ES so the `#` comments match the current English (for example, the `{ $start }` and
  `{ $path }` notes in `core-error-io`). No translation changed except the edits listed here.
- `terms.md`: added "deshabilitado" (a service's start type), the firewall profile names, and a structure note on the
  start words and the firewall sentence.

## Open questions
- **Firewall profile names:** "Dominio", "Privado", "Público" are taken from the Profiles box under Advanced
  options (Opciones avanzadas) in a Windows Defender Firewall rule, and from its Profile column. The sentence reads
  "…para app.exe, en los perfiles Dominio, Privado". A native speaker should confirm the names, and say whether
  "en redes privadas"-style wording would read more naturally to non-technical users.
- Start words: "se inicia con Windows, durante el arranque" (system) and "antes que todo lo demás" (boot) are
  descriptive phrasings, not Windows labels. Check that they sound natural.
- "Entrar" for the Enter key (`client-*`, `app-client-note`) follows Microsoft's Spanish style, but Spanish keyboards
  print "Intro".
- The client note and the client say "Firewall de Windows" (the Windows 7 name). Windows 10 and 11 say "Firewall de
  Windows Defender". This is acceptable because the client also runs on Windows 7, but worth a look.
- `core-warning-scan-license-data`: "llaves de hardware" for dongles. "mochilas" is also common in Spain.

Checker: es-ES: 0 errors, 0 warnings.
