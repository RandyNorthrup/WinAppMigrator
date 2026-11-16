# Review: Italian, Italy (it-IT)

Reviewer: independent machine pass, 2026-11-15. Translator's form of address: **tu**, with tu imperatives ("Scegli",
"Immetti", "Riprova"). Kept, because Windows 10 and 11 in Italian address the user with tu ("Vuoi consentire a questa
app di apportare modifiche al dispositivo?"), and the glossary asks for tu. Sentence case, Windows' terms ("Esplora
file", "Registro di sistema", "attività pianificata", "collegamento", "Autore" for Publisher, "INVIO") and straight
quotes were already consistent; new text follows them. The translation was already good: most of the sample needed no
change.

## Brought up to date

30 messages from the English delta, plus 3 sentences changed so the new words read correctly in them.

- **Rewritten to match the new English (12):** `app-scan-failed` (a colon before `{ $error }`, which starts in lower
  case: "Impossibile analizzare questa app: impossibile trovare un eseguibile…"); `app-footprint-licenses-still-go`,
  `core-footprint-licenses-left-out`, `core-review-note-licenses`, `core-secret-why-license-option-off`,
  `core-warning-scan-license-data` ("l'opzione Licenza" became "l'opzione Dati di licenza e attivazione", the words of
  `app-option-license`; in the last one "Se l'opzione Dati di licenza e attivazione è attiva", to avoid "attivazione
  attivata"); `core-warning-build-recorded-folder-left-out`, `core-warning-build-near-part-left-out` (`[unreadable]`
  said only "non può essere letta"; now "il suo percorso esce dall'unità o passa attraverso un collegamento simbolico,
  quindi non può essere letta in modo sicuro", a safety rule); `core-review-com-support-detail` (was "Rispettivamente N,
  N e N impostazioni"; now "Impostazioni delle sue librerie dei tipi (N), interfacce (N) e AppID (N), che accompagnano
  le sue classi"); `installer-license-detail` ("chiavi", read as product keys, became "valori del Registro di sistema
  che contengono licenze"); `installer-no-programdata`, `installer-cli-no-programdata` (said the install could not have
  been undone; now "non è stato installato nulla: è lì che un'installazione conserva il registro che serve ad
  annullarla").
- **Added (10):** `core-review-start-boot`, `-system`, `-automatic`, `-delayed`, `-manual`, `-disabled`, `-unknown`;
  `core-review-network-domain`, `-private`, `-public` (Dominio, Privato, Pubblico, the profile names of Windows
  Defender Firewall's rule wizard and Profile column).
- **Checked, already right, unchanged (8):** `client-builder-done`, `client-wrong-code`, `client-pair-failed` (already
  "Un altro PC" at the start of the sentence); `core-rollback-setting-put-back`, `core-rollback-list-entries-removed`,
  `core-rollback-list-entries-not-removed`, `core-rollback-firewall-removed` (the English fix was a capital "The"
  mid-sentence, which the Italian never had); `ui-disclosure` (already present, "espanso" / "compresso" as Narrator
  says them).
- **Changed so the new words fit (3, not in the delta):** `core-review-service-detail` `[other]` and
  `core-review-driver-detail-starts` ended in "tipo di avvio: { $start }", built for the raw Windows value. They now end
  in `{ $start }`, and each start word carries its own verb ("…, si avvia con Windows, prima di tutto il resto", "…,
  non si avvia mai, perché è disabilitato"), because Italian needs "non" before "mai" and cannot keep the verb in the
  sentence. "disabilitato" agrees with both servizio and driver. `core-review-firewall-detail` said "sulle reti di tipo
  { $networks }", which does not take the masculine profile names; it now selects on `$network_count`: "nel profilo di
  rete Privato" / "nei profili di rete Dominio, Privato", and "su tutte le reti" for none.

## Sample (back-translation)

| Message | Back-translation | Verdict |
| --- | --- | --- |
| ui-clear-search | Clear search | OK |
| ui-browse | Browse | OK |
| ui-browse-tooltip | Choose a folder for { $field } | OK |
| ui-disclosure | { $title }, { $summary }, expanded / collapsed | OK |
| ui-language | Language | OK |
| ui-machine-translated | This translation was made automatically and may contain errors. | OK |
| ui-report-translation-problem | Report a translation problem | OK |
| ui-needs-direct3d-12 | Direct3D 12 is needed to draw the window, and may not be there with an outdated graphics driver or in a virtual machine. | OK |
| size-bytes | { $value } byte (both forms) | OK: "byte" does not change in the plural in Italian |
| size-kb | { $value } KB | OK |
| size-mb | { $value } MB | OK |
| size-gb | { $value } GB | OK |
| size-tb | { $value } TB | OK |
| list-separator | ", " | OK |
| installer-window-title | WinAppMigrator installer | OK |
| installer-window-title-app | Install { $app_name } | OK |
| installer-tagline | Reinstalls an app captured on another PC | OK |
| installer-unnamed-app | Application | OK |
| installer-window-failed | Could not open the installer's window: { $error } | OK |
| installer-window-failed-without-one | You can still install without a window: from a command prompt with administrator rights, run this file with --apply and, to choose the destination, with --target-root followed by a folder. | OK |
| installer-checking | Checking the installer… | OK |
| installer-stub-no-package | This is a WinAppMigrator installer stub and contains no app package. Create an installer with WinAppMigrator, then run the generated .exe file here. | OK |
| installer-check-stopped | The installer check stopped. | OK |
| installer-locate-failed | Could not locate the installer: { $error } | OK |
| installer-read-failed | Could not read the installer: { $error } | OK |
| installer-cannot-be-used-here | This installer's package cannot be used on this PC: { $error } | OK |
| installer-captured-from | Captured from (agrees with "app") | OK |
| installer-install-location | Install location | OK |
| installer-choose-location | Choose the install location | OK |
| installer-folder-has-files | This folder already contains files. Files with the same name are replaced, and the originals are restored if you uninstall the app. | OK |
| installer-room-needed | Space needed | OK |
| installer-room-short | The space here may not be enough: { $shortfall }. The package can be unpacked on another drive, but the app's files must still fit here. | OK |
| installer-interrupted | An earlier install of { $app_name } was never completed. Clicking Install will undo it first. | OK |
| installer-signature | Installer | OK |
| installer-signature-checking | Checking the signature… | OK |
| installer-signature-unsigned | Not signed: unknown publisher | OK |
| installer-signature-signed-by | Signed by { $publisher } | OK |
| installer-signature-unnamed | Signed by an unnamed publisher | OK |
| installer-signature-invalid | The signature of this installer is not valid ({ $reason }). The program may have been changed after signing. | OK |
| installer-more-left-out | [one] 1 more item left out / [other] N more items left out | OK |
| uninstaller-plan-coming-back | N file(s) this install replaced is/are restored, or, if the current file has changed meanwhile, put beside it | OK |
| uninstaller-remove-user-data-detail | N item(s) of settings, data and license. It stays / they stay unless this option is on. What it replaced is restored, or put beside it if it changed meanwhile. | OK |
| app-list-search | Search by name or publisher  (Ctrl+F) | OK ("Autore" is Publisher in Italian Programs and Features) |
| app-column-name | Name | OK |
| app-column-publisher | Publisher | OK |
| app-column-version | Version | OK |
| app-column-scope | Scope | OK |
| app-column-arch | Arch. | OK, short |
| app-option-license | License and activation data | OK |
| app-option-user-data | User data | OK |
| app-option-whose-data | Which account's data | OK |
| app-source | Source of the scan | OK |
| app-build-create | Create installer | OK |
| app-build-create-again | Create the installer again | OK; long (see open questions) |
| app-build-cancel | Cancel | OK |
| app-scan-look-again | Analyze again | OK |
| app-result-open-folder | Open folder | OK |
| app-result-copy-path | Copy path | OK |
| app-network-code | Pairing code | OK |
| app-room-free-on | Free in { $drive } | Fixed: "Libero su { $drive }"; free space is "su C:" in Italian |
| core-space-shortfall | { $drive } needs up to { $needed } of free space for { $purposes }, and has { $free } | OK |
| core-policy-fit-needs-newer | { $program } says it needs Windows { $needs } or later, and this PC has version { $runs }, so Windows would not start it | OK |
| core-review-service-detail | [other] Runs X as Y, start type: { $start } | Fixed: ends in the new start words, which carry their verb |
| core-review-driver-detail-starts | Loads X directly into Windows, start type: { $start } | Fixed: as above |
| core-review-firewall-detail | Allows incoming connections on port P for X, on networks of type { $networks } | Fixed: "nel profilo di rete / nei profili di rete { $networks }" |
| core-review-com-support-detail | Respectively N, N and N settings, which go with its classes | Fixed (delta): each count names its kind |
| core-warning-review-left-out | The file / folder / service … X was left out: reason. (gender agreed per kind) | OK |
| core-rollback-file-stays-beside | X stays, because something that stays still uses it. The file it replaced is beside it, named N. | OK |
| core-warning-build-links-not-followed | N symbolic link(s), or name(s) that would open something else, were not followed in this folder, so what they point to is not in the installer: names and M more. | OK |
| core-archive-data-file-missing | This installer keeps its package in a file named N in the same folder, and that file is not there. The two files must stay together: copy N here from where the installer was created, then run it again. | OK |
| core-restore-root-under-link | This app has things to restore in R, and L is a symbolic link pointing elsewhere that others may change, so nothing was installed. If it is user or license data, install without them; if a shortcut in the Startup folder, put that folder back. | OK |
| core-warning-restore-earlier-undone | An earlier install of X never finished, so it was undone first. N change(s) put back. | OK |
| core-privilege-not-made-here | The folder X was not created by WinAppMigrator and others can change its contents. An administrator should check and remove it: error | OK |

73 messages: all of `ui.ftl` and `winappmigrator.ftl`, the first 25 of `installer.ftl`, its 3 plural messages, 18 of
`app.ftl`, 13 from the `core*.ftl` files with placeholders. 5 fixed in the sample (1 of them in the delta).

## Other fixes

- **Saved Games:** "Partite salvate" became **"Giochi salvati"**, the name Italian Windows gives the known folder
  (C:\Utenti\nome\Giochi salvati; Citrix's Italian profile-management documentation also calls the Saved Games folder
  "Giochi salvati"). Changed everywhere it appears: `app-option-user-data-hint`, `installer-user-data-detail`,
  `core-footprint-place-saved-games`, `core-warning-review-saved-games-elsewhere`,
  `core-warning-review-saved-games-unknown`, and `terms.md`.
- `terms.md`: added the firewall profile names (Dominio, Privato, Pubblico), "disabilitato" for a disabled service, and
  "libreria dei tipi" for a COM type library.
- The `#` comments were refreshed to the current English with `ftl-tool.py merge` (only comment lines changed), so a
  later translator reads the current notes, such as for `core-rollback-count-kept` and `-left-behind`.
- Checked across the whole language: the column headers are short ("Nome", "Autore", "Versione", "Ambito", "Arch.").
  Nothing is left in English that should not be: every message identical to the English is a name, a unit, a
  placeholder-only pattern, "Log" (as Italian Windows says it), "Driver", or "In { … }", which is Italian too.

## Open questions

- **Firewall profiles** (`core-review-network-*`): "Dominio", "Privato", "Pubblico" are the names in Windows Defender
  Firewall con sicurezza avanzata. The Settings app writes the network profile as "Pubblica" / "Privata" (agreeing with
  "rete"). A native speaker should confirm "nei profili di rete Dominio, Privato" reads naturally.
- **"Giochi salvati"** was confirmed from secondary sources, not seen on an Italian Windows. Worth one look at a user
  folder on Italian Windows.
- **Long buttons:** `app-build-create-again` "Crea di nuovo il programma di installazione" (43 characters to the
  English 22) and `app-build-create` "Crea programma di installazione" (31 to 14). "programma di installazione" is
  Windows' own word for installer, so the window should make room rather than the label shrink.
- The start words are descriptive, as in English, not the Services console's labels (Automatico, Automatico (avvio
  ritardato), Manuale, Disabilitato); only "disabilitato" is Windows' word.
- `core-rollback-count-kept` "Lasciati al loro posto" and `core-rollback-count-left-behind` "Rimasti" are as close to
  each other as the English "Left in place" and "Left behind".
- `Ctrl+F` is kept as Italian Windows writes it in menus (Italian documentation writes CTRL+F).
- For the English (raised by vi-VN): `core-rollback-setting-other-account` `[machine-variable]` and `[machine-list]`
  say a PC-wide variable "belongs to the account that installed this". The Italian says the same, and follows the
  English if that changes.

Checker: it-IT: 0 errors, 2 warnings (`core-review-com-title` "COM in { }" and `core-review-certificate` "{ } in { }",
the same in Italian).
