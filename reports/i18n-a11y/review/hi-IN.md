# Review: Hindi (hi-IN)

Reviewer: independent machine pass, 2026-11-15. Translator's form of address: आप, with the polite imperative
(चुनें, करें, चलाएँ) on buttons and in instructions; kept, because that is how Windows in Hindi addresses the user.

## Brought up to date

30 messages checked against the current English. 12 needed new or changed words:

- `app-scan-failed`: the reason now follows a colon, not a full stop ("इस ऐप को स्कैन नहीं किया जा सका: …").
- `app-footprint-licenses-still-go`, `core-footprint-licenses-left-out`, `core-review-note-licenses`,
  `core-secret-why-license-option-off`, `core-warning-scan-license-data`: "लाइसेंस विकल्प" (the License option) is now
  "लाइसेंस और सक्रियण डेटा" विकल्प, the words of the checkbox `app-option-license`. The name is in quotes so that it
  reads as one name, not as "license, and the activation data option".
- `core-warning-build-recorded-folder-left-out`, `core-warning-build-near-part-left-out` `[unreadable]`: was "it cannot
  be read"; now the safety rule: "इसका पथ ड्राइव से बाहर जाता है या किसी लिंक से होकर जाता है, इसलिए इसे सुरक्षित रूप से
  पढ़ा नहीं जा सकता" ("ड्राइव से बाहर" as `core-warning-scan-drive-paths-not-followed` already says it).
- `core-review-com-support-detail`: was "क्रमशः 3, 5 और 1 सेटिंग्स" ("respectively 3, 5, and 1 settings", with nothing
  for "respectively" to refer to); now each count follows its own kind: "इसकी टाइप लाइब्रेरी (3), इंटरफ़ेस (5) और
  AppID (1) की सेटिंग्स, जो इसकी क्लास के साथ जाती हैं".
- `installer-license-detail`: "कुंजियाँ" (keys, which could read as product keys) is now "लाइसेंस रखने वाले रजिस्ट्री मान"
  (registry values that hold licenses).
- `installer-no-programdata`, `installer-cli-no-programdata`: was "so the install could not be undone"; now "इसलिए कुछ
  भी इंस्टॉल नहीं किया गया: इंस्टॉल को पूर्ववत करने के लिए उसका रिकॉर्ड वहीं रखा जाता है" (so nothing was installed: an
  install's record, for undoing it, is kept there). The CLI line keeps English's lack of a full stop.

18 already said what the current English says, and were checked and left as they are:

- `client-builder-done`, `client-wrong-code`, `client-pair-failed` `[unknown]`: "किसी अन्य PC" (another PC, in the form
  that goes before का/ने/से) opens the sentence correctly; Devanagari has no capitals, so English's change needs nothing
  here.
- `core-review-start-boot`, `-system`, `-automatic`, `-delayed`, `-manual`, `-disabled`, `-unknown`: "Windows के साथ,
  बाकी सब से पहले", "Windows के साथ, उसके प्रारंभ होते समय", "हर बार यह PC प्रारंभ होने पर", "हर बार यह PC प्रारंभ होने के
  कुछ देर बाद", "केवल तब, जब कोई चीज़ इसके लिए कहती है", "कभी नहीं, क्योंकि यह अक्षम है" (अक्षम: Windows' "Disabled"
  startup type), "ऐसे तरीके से जिसे Windows नहीं बताता". They fill "…, प्रारंभ होती है: { $start }" (service, feminine)
  in `core-review-service-detail` `[other]` and "…, प्रारंभ होता है: { $start }" (driver, masculine) in
  `core-review-driver-detail-starts`. Hindi puts the verb last, so the words cannot simply end the sentence as in
  English, and "कभी नहीं" (never) cannot sit before an affirmative "प्रारंभ होता है"; the colon keeps every one of the
  seven correct with both genders. Checked each filled in; kept.
- `core-review-network-domain`, `-private`, `-public`: "डोमेन", "निजी", "सार्वजनिक", Windows Defender Firewall's profile
  names in Hindi. They fill "{ $program } के लिए, { $networks } नेटवर्क पर …" in `core-review-firewall-detail`, joined by
  `list-separator`, and read correctly ("app.exe के लिए, डोमेन, निजी नेटवर्क पर …").
- `core-rollback-setting-put-back`, `core-rollback-list-entries-removed`, `core-rollback-list-entries-not-removed`,
  `core-rollback-firewall-removed`: English's change was its stray capital "The"; the Hindi had no such fault.
- `ui-disclosure`: present, with only "खुला" (open) and "बंद" (closed) translated.

## Sample (back-translation)

111 messages. The side-by-side listing used for most of them showed the English beside the Hindi, so those
back-translations were made from the Hindi line read first, not fully blind; the last 14 (marked "blind") were
back-translated from a Hindi-only listing before the English was opened.

| Message | Back-translation | Verdict |
| --- | --- | --- |
| ui-clear-search | Clear search | OK |
| ui-browse | Browse | OK |
| ui-browse-tooltip | Choose a folder for { $field } | OK |
| ui-disclosure | { $title }, { $summary }, open / closed | OK |
| ui-language | Language | OK |
| ui-machine-translated | This translation was done by machine and may contain mistakes. | OK |
| ui-report-translation-problem | Report a problem with the translation | OK |
| ui-needs-direct3d-12 | To show its window it needs Direct3D 12, which an old graphics driver or a virtual machine cannot have. | Fixed: "नहीं हो सकता" read as "cannot have"; now "शायद न हो" (may not have) |
| size-bytes | { $value } byte / bytes (Hindi uses one form) | OK |
| size-kb | { $value } KB | OK |
| size-mb | { $value } MB | OK |
| size-gb | { $value } GB | OK |
| size-tb | { $value } TB | OK |
| list-separator | ", " | OK |
| installer-window-title | WinAppMigrator Installer | OK |
| installer-window-title-app | Install { $app_name } | OK |
| installer-tagline | Reinstalls an application captured on another PC | OK |
| installer-unnamed-app | Application | OK |
| installer-window-failed | The installer's window could not open: { $error } | OK |
| installer-window-failed-without-one | It can install without the window too: run this file from an administrator command prompt with --apply, and to choose the place give a folder after --target-root. | OK |
| installer-checking | The installer is being checked… | OK |
| installer-stub-no-package | This is a WinAppMigrator installer stub, and it has no application package. Make an installer with WinAppMigrator, then run the made .exe file here. | OK |
| installer-check-stopped | The installer's check stopped. | OK |
| installer-locate-failed | The installer could not be located: { $error } | OK |
| installer-read-failed | The installer could not be read: { $error } | OK |
| installer-cannot-be-used-here | This installer's package cannot be used on this PC: { $error } | OK |
| installer-captured-from | Source of the capture | OK |
| installer-install-location | Install location | OK |
| installer-choose-location | Choose install location | OK |
| installer-folder-has-files | This folder already has files. Files with the same names are replaced, and if you uninstall, they come back. | OK |
| installer-room-needed | Disk space needed | OK |
| installer-room-short | There cannot be enough space here: { $shortfall }. The package can unpack on another drive, but there must still be enough space here for the app's files. | Fixed: "पर्याप्त स्थान नहीं हो सकता है" also reads as "cannot be"; now "हो सकता है कि यहाँ पर्याप्त स्थान न हो" (there may not be) |
| installer-interrupted | An earlier install of { $app_name } never finished. On installing, it is undone first. | OK |
| installer-signature | Installer | OK |
| installer-signature-checking | Its signature is being checked… | OK |
| installer-signature-unsigned | Not signed: publisher unknown | OK |
| installer-signature-signed-by | Signed by { $publisher } | OK |
| installer-signature-unnamed | Signed by an unnamed publisher | OK |
| installer-signature-invalid | This installer's signature is not valid ({ $reason }). It may have been changed after it was signed. | OK |
| installer-more-left-out | { $count } more item was / items were left out | OK |
| uninstaller-plan-coming-back | { $count } file(s) this install replaced come back, or, if the file in their place has changed since, are put beside it. | OK |
| uninstaller-remove-user-data-detail | { $count } item(s) of its settings, data, and license. Unless this option is on, they stay. What they replaced comes back, or, if they have changed since, is put beside them. | OK |
| app-tagline | Turn an installed app into a self-extracting installer | OK |
| app-theme-to-light | Switch to light theme | OK |
| app-theme-to-dark | Switch to dark theme | OK |
| app-reload-list | Load the application list again | OK |
| app-list-count | { $count } app / apps | OK |
| app-list-search | Search by name or publisher  (Ctrl+F) | OK (Hindi Windows writes Ctrl) |
| app-list-filter-all | All | OK |
| app-scope-per-user | Per-user | OK |
| app-scope-per-machine | Per-machine | OK |
| app-list-row | { $name }, { $publisher }, version { $version } | OK |
| app-column-name | Name | OK |
| app-column-name-sort | Sort by name | OK |
| app-column-publisher | Publisher | OK |
| app-column-version | Version | OK |
| app-column-scope | Scope | OK |
| app-column-arch | Arch (transliterated) | OK, see open questions |
| app-column-arch-sort | Sort by architecture | OK |
| app-source | Scan source | OK |
| app-source-this-pc | This PC | OK |
| app-source-drive | Connected drive… | OK |
| app-source-image | Disk image… | OK |
| app-source-network | Network PC… | OK |
| app-option-license | License and activation data | OK |
| app-option-license-hint | It may be tied to this PC, and may need activating again on the new PC. | OK |
| app-option-user-data | User data | OK |
| app-option-user-data-hint | Settings and history, Documents, Saved Games, and ProgramData. | OK |
| app-option-whose-data | Whose data | OK |
| app-option-whose-data-hint | An installer holds one account's data. Wherever the installer runs, it goes to whoever is signed in there. | OK |
| app-output-folder | Output folder | OK |
| app-build-create | Create installer | OK |
| app-build-cancel | Cancel | OK |
| app-result-open-folder | Open folder | OK |
| app-result-copy-path | Copy path | OK |
| app-scan-look-again | Scan again | OK (terms.md: look at an app = स्कैन करना) |
| app-scan-stop | Stop | OK |
| app-network-code | Pairing code | OK |
| app-network-connect | Connect | OK |
| core-rollback-drive-not-connected | What this install put in { $folder } is on a drive that is not connected. Connect it, and this can be finished. | OK |
| core-rollback-file-stays-not-beside | { $path } stays, because something that stays still uses it / it has changed since it was installed / it was not asked to be removed, and the file it replaced could not be put beside it: { $error }. | OK |
| core-rollback-task-not-put-back | The task { $name } could not be put back as it was: { $reason }. | OK |
| core-warning-scan-last-account-unknown | This drive does not say who last signed in, and it has more than one account, so no one's per-user data or settings were included. Choose whose data to include. | OK |
| core-runtime-payload-timed-out | The bundled installer did not finish within the time limit and was ended | OK |
| core-scan-not-a-file | The scan target is not a file: { $path } | OK |
| core-package-robocopy-failed | robocopy backup-mode copy failed (exit { $code }) | OK |
| core-warning-build-file-gone | This was there when the app was scanned, but is not now, so it is not in the installer. | OK |
| core-warning-build-user-folder-not-on-drive | { $user }'s { $folder } folder was at { $path }, which is not on this drive, so what is there was not included. | OK |
| core-restore-not-all-put-back | { $error }. What was done could not all be put back as it was: { $failed } | OK |
| core-restore-would-install-driver | Driver { $name } -> { $destination } will be installed | OK |
| core-warning-restore-service-not-started | Service { $name } is installed but did not start: { $reason }. Until it starts, the app may not work. When this PC next starts, Windows will try again. | OK |
| core-warning-restore-driver-needs-admin | Registering the driver needs administrator rights; run the installer again as administrator. | OK |
| core-space-not-told | { $drive } did not say how much space it has, so whether the { $needed } it needs fits was not checked | OK |
| core-review-task-detail-highest | Runs, with the highest rights it can have, as: { $account } | OK (colon because { $account } may be a phrase) |
| core-privilege-users-cannot-start | Users could not be allowed to start { $path }: { $said } | OK |
| core-verify-file-matches | The file's size and hash match the manifest. | OK |
| core-warning-verify-package-file-failed | Verification of the package file failed: { $error } | OK |
| core-rollback-reason-no-record (blind) | There is no record of where { $folder } was | OK |
| core-rollback-backup-gone (blind) | The backup no longer exists | OK |
| core-rollback-key-not-removed (blind) | The key { $name } could not be removed: { $error }. | OK |
| core-rollback-firewall-says-otherwise (blind) | The firewall rule { $name } stays, because the rule there by that name says something else. | OK |
| core-rollback-earlier-installs-not-undone (blind) | An earlier install that never finished could not be fully undone, so nothing was installed. Close whatever is using these and try again. { $details } | OK |
| core-system-own-system-drive (blind) | { $drive } is this PC's own system drive. To read this PC, choose "This PC". | OK |
| core-account-left-out-variables-why (blind) | { $count } variable(s), which only go in with the User Data option | OK |
| core-secret-left-out-firewall-description (blind) | The description of the firewall rule { $rule } | OK |
| core-task-too-large (blind) | It is { $size } bytes, which is too large for a task | OK |
| core-warning-scan-dependency-not-in-newer-windows (blind) | { $name } was in the Windows folder of the PC this app was on, and does not come with Windows 10 and 11. Something else installed it there, so it is not part of the app's own files and does not come along. The app may not start until what provides it is installed. | OK |
| core-runtime-handling-manual (blind) | Install manually | OK |
| core-runtime-payload-not-started (blind) | The bundled installer could not be started: { $error } | OK |
| core-runtime-winget-not-started (blind) | winget is not available to install { $id }: { $error } | OK |
| core-registry-certificate-recorded (blind) | The app's install record names it | OK |

Whole-language checks: the `app-column-*` headers are नाम, प्रकाशक, संस्करण, दायरा, आर्क, all short; button labels are
two or three words in the polite imperative, as Windows in Hindi writes them ("इंस्टॉलर बनाएँ", "रद्द करें"). A scan of
every message for Latin-script words found only what the glossary keeps (product, file, folder, command, and flag names,
units, `Ctrl+F`, the folder name `_external`, "Edit Disk" as terms.md says) and the examples `OLD-PC` and `D:\Installers`.

## Other fixes

- Comments brought into line with the current English by `ftl-tool.py merge` (17 comment lines in `app.ftl`,
  `core.ftl`, `core-restore.ftl`, `core-review.ftl`; checked on a copy first that it changed no message text).
- `terms.md`: added the app's "License and Activation Data" option and how a sentence names it, kept apart from the
  installer's "License" option, and "अक्षम" for the Disabled startup type.

## Open questions

- Service and driver start: "…, प्रारंभ होती है: { $start }" keeps all seven start words correct for a service
  (feminine) and a driver (masculine), but reads like a label after the colon. A native speaker may prefer the start
  words reworded so they can stand before the verb ("…बाकी सब से पहले प्रारंभ होती है"), which needs a new wording for
  `core-review-start-disabled` and `-manual`.
- Firewall profile names "डोमेन", "निजी", "सार्वजनिक": the usual Hindi Windows words, not confirmed against Hindi Windows
  Defender Firewall itself.
- `app-column-arch` "आर्क" is a transliteration, not a Hindi abbreviation; the sort tooltip spells out "आर्किटेक्चर".
  A native speaker might prefer "आर्कि." if the column allows.
- The translator's question stands: whether Hindi Registry Editor shows "(Default)" as "(डिफ़ॉल्ट)", as terms.md has it.
- "हर बार यह PC प्रारंभ होने पर" is everyday Hindi; the more formal "हर बार इस PC के प्रारंभ होने पर" may read better to some.
- `app-output-folder-example` uses "आप" (you) as the stand-in account name in `C:\Users\आप\WinAppMigrator`, as English
  uses "You"; real Hindi account names are often in Latin letters.

Checker: hi-IN: 0 errors, 0 warnings.
