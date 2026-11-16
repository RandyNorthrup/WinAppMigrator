# Review: Hebrew (he-IL)

Reviewer: independent machine pass, 2026-11-15. Translator's form of address: gender-neutral, as Microsoft writes Hebrew
for Windows 11: instructions are "יש ל..." with an infinitive ("יש לבחור", "יש להזין"), what the reader may do is "ניתן",
buttons and commands are action nouns ("התקנה", "ביטול", "סגירה", "עיון"), and what happened is passive ("הושמט", "לא
ניתן היה"); kept, because it is how Windows addresses the reader in Hebrew and it reads the same to everyone. A search of
every file for gendered imperatives, futures, and presents addressed to the reader ("בחר", "לחץ", "הזן", "תוכל",
"אתה", …) found none; past forms such as "הקלדת" and "שהשמטת" are the same for everyone without vowels. No direction marks
anywhere (checked: no U+200E, U+200F, U+061C, embeddings, isolates, zero-width characters, or no-break spaces), and no
vowel points.

## Brought up to date

All 30 messages in the English delta were compared with the current English. 11 needed new words:

- `app-scan-failed`: the reason now follows a colon, not a full stop ("לא ניתן היה לסרוק אפליקציה זו: { $error }"). It
  reads with the reasons it is given, such as `core-scan-no-executable`.
- `core-warning-build-recorded-folder-left-out`, `core-warning-build-near-part-left-out` `[unreadable]`: was "שלא ניתן
  לקרוא אותה" / "לא ניתן לקרוא אותו" (it cannot be read), a read error. Now the safety rule: "(ש)הנתיב שלה/שלו מוביל אל
  מחוץ לכונן או עובר דרך קישור, ולכן לא בטוח לקרוא אותה/אותו" (its path leads outside the drive or goes through a link,
  so it is not safe to read it). Gender agrees with each sentence (the folder, feminine; the part, masculine), and the
  first sentence keeps the "ש" that follows "מכיוון" in every reason.
- `core-review-note-licenses`, `core-secret-why-license-option-off`, `core-warning-scan-license-data`: "האפשרות של נתוני
  הרישיון" (the option of the license data) is now האפשרות "נתוני רישיון והפעלה", the checkbox's own words
  (`app-option-license`), as `app-footprint-licenses-still-go` and `core-footprint-licenses-left-out` already had it.
- `installer-license-detail`: "קבצים ומפתחות של הפעלה ורישיון" (activation and license files and keys, which reads as
  product keys) is now "קבצי הפעלה ורישיון, וערכי הרישום שמכילים רישיונות" (and the registry values that hold licenses).
- `installer-no-programdata`, `installer-cli-no-programdata`: was "so the install could not be undone" / "so an install
  will not be able to be undone"; now "ולכן לא הותקן דבר: כל התקנה שומרת שם את הרשומה שמשמשת לביטולה" (so nothing was
  installed: every install keeps there the record that is used to undo it). The command-line one stays without a full
  stop, as in English.
- `core-review-com-support-detail`: was the three kinds, then "בהתאמה" (respectively) and the three counts. Now each
  count follows its kind, as the English does: "הגדרות שנלוות למחלקות שלו: ספריות סוגים (3), ממשקים (5) ו-AppIDs (1)"
  (settings that go with its classes: type libraries (3), interfaces (5) and AppIDs (1)).
- `core-review-start-unknown`: was "באופן ש-Windows אינו מציין" (in a way Windows does not say). The start words fill
  "מועד ההפעלה: { $start }" (when it starts: …) in `core-review-service-detail` `[other]` and
  `core-review-driver-detail-starts`, and a manner does not answer "when". Now "לא מצוין על ידי Windows" (not stated by
  Windows).

19 already said what the current English says:

- `app-footprint-licenses-still-go`, `core-footprint-licenses-left-out`: already named the option in the checkbox's words.
- `client-builder-done`, `client-wrong-code`, `client-pair-failed`: Hebrew has no capitals. "מחשב אחר" (another PC) is
  the subject of "סיים" and "ניסה" and agrees with them (masculine); `client-pair-failed` says "לא ניתן היה לבצע התאמה
  עם מחשב אחר" (pairing with another PC could not be done), the same fact.
- `core-review-start-boot`, `-system`, `-automatic`, `-delayed`, `-manual`, `-disabled`: each reads correctly after
  "מועד ההפעלה:" in both sentences, for a service and for a driver (both masculine, so "הוא" in "אף פעם, מכיוון שהוא
  מושבת" fits both). The frame is a label and its value, so "אף פעם" (never) needs no negated verb.
- `core-review-network-domain`, `-private`, `-public`: "תחום", "פרטי", "ציבורי", as `terms.md` has Windows' profile
  names. They fill "ברשת/ברשתות מסוג { $networks }" (on networks of the kind …) in `core-review-firewall-detail`, where
  they agree with "סוג" and read correctly alone or as a list ("ברשתות מסוג תחום, פרטי"). See open questions.
- `core-rollback-setting-put-back`, `core-rollback-list-entries-removed`, `core-rollback-list-entries-not-removed`,
  `core-rollback-firewall-removed`: Hebrew never had the capital "The".
- `ui-disclosure`: "מורחב" / "מכווץ" (expanded / collapsed), the words Narrator says for these states in Hebrew.

Every phrase that fills a placeholder in these sentences was checked to start with a Hebrew letter (see "Other fixes"
for why).

## Sample (back-translation)

92 messages: all of `ui.ftl` (8) and `winappmigrator.ftl` (6), the first 25 of `installer.ftl`, its 3 plural messages,
29 from `app.ftl` (its tagline, buttons, every column header, scopes, and option names), and 21 from the `core*.ftl`
files with placeholders.

| Message | Back-translation | Verdict |
| --- | --- | --- |
| ui-clear-search | Clearing the search | OK |
| ui-browse | Browse | OK (Windows' "עיון") |
| ui-browse-tooltip | Choosing a folder for {field} | OK |
| ui-disclosure | {title}, {summary}, expanded / collapsed | OK (see above) |
| ui-language | Language | OK |
| ui-machine-translated | This translation was made by machine and may contain errors. | OK |
| ui-report-translation-problem | Reporting a problem in the translation | OK |
| ui-needs-direct3d-12 | This program needs Direct3D 12 to draw its window, and an old graphics driver or a virtual machine may not support it. | OK |
| size-bytes | {value} byte / bytes (one, two, other) | OK |
| size-kb | {value} KB | OK: Hebrew Windows writes KB, MB, GB, TB in Latin letters |
| size-mb | {value} MB | OK |
| size-gb | {value} GB | OK |
| size-tb | {value} TB | OK |
| list-separator | ", " | OK |
| installer-window-title | The installer of WinAppMigrator | OK |
| installer-window-title-app | Installing {app} | OK (window titles are action nouns) |
| installer-tagline | Reinstalling an app that was captured on another PC | OK |
| installer-unnamed-app | App | OK |
| installer-window-failed | The installer's window could not be opened: {error} | OK |
| installer-window-failed-without-one | It is still possible to install without the window: run this file from an administrator's command prompt with --apply, and, to choose the location, with --target-root followed by a folder. | OK |
| installer-checking | Checking the installer… | OK |
| installer-stub-no-package | This is a WinAppMigrator installer stub, and it holds no app package. Build an installer with WinAppMigrator, then run here the .exe file that was produced. | OK |
| installer-check-stopped | The check of the installer stopped. | OK |
| installer-locate-failed | The installer could not be located: {error} | OK |
| installer-read-failed | The installer could not be read: {error} | OK |
| installer-cannot-be-used-here | This installer's package cannot be used on this PC: {error} | OK |
| installer-captured-from | Source of the capture | OK |
| installer-install-location | Install location | OK |
| installer-choose-location | Choosing the install location | OK (the folder picker's title) |
| installer-folder-has-files | This folder already holds files. Files with the same name will be replaced, and will come back if the install is removed. | OK |
| installer-room-needed | Space needed | OK |
| installer-room-short | There may not be enough space here: {shortfall}. Unpacking can happen on another drive, but there still has to be room for the app's files. | OK |
| installer-interrupted | An earlier install of {app} did not finish. The install will undo it first. | OK |
| installer-signature | The installer | OK |
| installer-signature-checking | Checking the signature… | OK |
| installer-signature-unsigned | Not signed: unknown publisher | OK (Windows' "מפרסם לא ידוע") |
| installer-signature-signed-by | Signed by {publisher} | OK |
| installer-signature-unnamed | Signed by a publisher without a name | OK |
| installer-signature-invalid | This installer's signature is not valid ({reason}). It may have been changed since it was signed. | OK |
| installer-more-left-out | One more item was left out / two more items / {n} more items were left out | OK, each form |
| uninstaller-plan-coming-back | One file / two files / {n} files this install replaced will come back, or be placed beside a file that has changed since. | OK, each form |
| uninstaller-remove-user-data-detail | One item / two items / {n} items of the app's settings, data and license. It stays / they stay unless this option is on. What it / they replaced comes back, or is placed beside it / them if it / they changed since. | OK: verbs and pronouns agree in each form |
| app-tagline | Turning an installed app into a self-extracting installer | OK |
| app-list-count | One app / two apps / {n} apps | OK |
| app-list-search | Search by name or publisher (Ctrl+F) | OK (Hebrew Windows writes Ctrl) |
| app-scope-per-user | Single user | OK |
| app-scope-per-machine | Whole PC | OK |
| app-column-name | Name | OK, short |
| app-column-publisher | Publisher | OK, short |
| app-column-version | Version | OK, short |
| app-column-scope | Scope | OK, short |
| app-column-arch | Processor | OK: "ארכיטקטורה" (10 letters) does not fit the 58-pixel column; "מעבד" (4) does, and its values are processor kinds (x86, x64, ARM64). See open questions |
| app-column-arch-sort | Sort by processor | OK, matches its header |
| app-source | Scan source | OK |
| app-source-network | PC on the network… | OK (Network PC, as `terms.md` has it) |
| app-option-license | License and activation data | OK |
| app-option-user-data | User data | OK |
| app-option-whose-data | Whose data | OK |
| app-output-folder | Output folder | OK |
| app-build-create | Creating an installer | OK |
| app-build-cancel | Cancel | OK (Windows' "ביטול") |
| app-scan-look-again | Scan again | OK |
| app-scan-stop | Stop | OK |
| app-scan-try-again | Retry | OK |
| app-scan-failed | This app could not be scanned: {error} | Fixed: the reason follows a colon, as the English now has it (delta) |
| app-result-open-folder | Open the folder | OK |
| app-result-copy-path | Copy the path | OK |
| app-room-drive-short | On drive {drive}, up to {needed} free is needed to build this, and {free} is free on it. | OK |
| app-network-connect | Connect | OK |
| app-footprint-folders-summary | {included} of one folder / two folders / {n} folders, {size} | OK |
| app-footprint-licenses-still-go | License data in what you left out is still included, because it goes by the option "License and activation data": {list}. | OK (already up to date) |
| core-rollback-prerequisite-stays | {name} was installed, or may have been, and it stays, because other programs may use it. | OK |
| core-license-detail-file | A license file was detected: {name} | OK |
| core-runtime-feature-failed | The DISM tool could not enable the Windows feature {feature} (exit code {code}); most likely the payload could not be downloaded. Enable it through "Windows Features", or run `dism …`. | Fixed: started with "DISM", so where it is inserted it would be laid out left to right (see Other fixes). Command kept as is |
| core-profile-desktop-not-a-person | The desktop here belongs to {account}, which is not a person's account | OK |
| core-scan-no-executable | No executable was found for {app}. A driver, runtime or add-in may have no program of its own to package. | OK, also after `app-scan-failed`'s colon |
| core-discovery-refusing-link | Going through a link is not allowed: {path} | OK |
| core-error-io | Input/output failed for {path}: {error} | OK, also with "תוכנית ההתקנה" (the installer) in {path} |
| core-error-pe | PE parsing failed for {path}: {detail} | OK |
| core-restore-location-not-full-path | The install location must be a full path: "{path}" | OK |
| core-restore-would-create-empty-folder | The empty folder {path} will be created | OK |
| core-restore-would-restore-variable | The user's / the machine's environment variable {name} will be restored | OK |
| core-review-account-not-told | Who is signed in could not be determined ({reason}), so per-user settings are left out, and user data goes into the install location. | OK |
| core-verify-not-unpacked | This installer's package was not unpacked to {path} | OK |
| core-policy-fit-program | {program} is built for {built_for}, and this PC is {here}, so Windows will not start it | OK; its fill words for 32-bit x86 and Arm fixed (see Other fixes) |
| core-warning-build-left-out-of-package | Left out of the package: {why}. | OK, also with the License option's reason |
| core-archive-entry-unreadable | Item {index} in the package could not be read: {reason} | OK |
| core-warning-build-base-unreadable | {path} could not be read ({error}); app data stored there was not included. | OK |
| core-secret-why-account-values | One value / two values / {n} values, included only with the option "User data" | OK |
| core-review-firewall-detail | Allows incoming connections on port(s) {ports} for {program}, on networks of the kind {networks} / on every network | OK with the new network words |
| core-review-service-detail | Runs {file} as {account}; when it starts: {start} (`[other]`) | OK with every start phrase, after `core-review-start-unknown` was fixed |
| core-review-driver-detail-starts | Loads {file} into Windows itself; when it starts: {start} | OK, same |

Across the whole language: the `app-column-*` headers are one short word each ("שם", "מפרסם", "גרסה", "היקף", "מעבד");
button labels are one to three words; nothing is left in English except what stays English everywhere (product and
program names, commands and flags, file, folder and registry names, `_external`, `OLD-PC`, `Ctrl+F`, `KB`…`TB`, the
example path `C:\Users\You\WinAppMigrator`, and "Build", which `terms.md` takes from Hebrew Windows). The `#` comments
and the order of messages already matched the current English in every file.

## Other fixes

- Words the program puts into another message must start with a Hebrew letter. The program sets off every placeholder
  (and every select) with a first-strong isolate, so a phrase that starts with a Latin word, such as "Windows" or "DISM",
  is laid out left to right inside the Hebrew sentence and reads in the wrong order. 19 such phrases were reworded,
  each keeping its meaning:
  - Reasons and errors: `core-image-already-attached` ("התמונה כבר צורפה על ידי Windows, …", the image is already
    attached by Windows), `core-image-windows-did-not-say` ("הסיבה לא צוינה על ידי Windows", the reason was not stated
    by Windows), `core-review-name-not-accepted` ("השם שלו לא התקבל על ידי Windows"), `core-policy-variable-relied-on`
    ("על משתנה זה מסתמכים Windows ותוכניות אחרות"), `core-privilege-no-program-data` ("מיקום התיקיה ProgramData אינו
    מצוין על ידי Windows"), `core-profile-desktop-owner-not-said` ("לא ניתן היה לקבל מ-Windows את הבעלים של שולחן עבודה
    זה: { $error }").
  - Commands and tools: "הכלי DISM" (the DISM tool) in `core-runtime-feature-failed`, `-failed-no-code`, `-no-dism`;
    "הכלי winget" in `core-runtime-winget-not-installed`, `-not-started`; "הפקודה winget install" (the command …) in
    `core-runtime-winget-failed`, `-failed-no-code`, `-timed-out`; "הפקודה reg load / reg unload" in
    `core-system-hive-not-loaded`, `-not-unloaded`. Verbs now agree with "הפקודה" (feminine).
  - Kinds of processor, which fill { $built_for } and { $here }: `core-policy-architecture-x86` "32 סיביות (x86)" and
    `-arm32` "32 סיביות (Arm)" (were "x86 של 32 סיביות", "Arm של 32 סיביות").
  - The Windows name for an old drive, which fills { $windows } in `app-source-older-windows` ("כאן מותקן { $windows }"):
    `core-system-windows-xp-or-older` "לכל היותר Windows XP" (Windows XP at most), was "Windows XP או גרסה ישנה יותר".

  Phrases that start with a Latin word but are only shown on their own (titles such as `core-review-com-title`, hints,
  the client's console text) were left as they are: there the window's right-to-left direction lays them out.
- `terms.md`: added the rule above, the option's name in quotes (האפשרות "נתוני רישיון והפעלה"), and "32 סיביות (x86)"
  / "32 סיביות (Arm)".

## Open questions

- Firewall profiles: "תחום", "פרטי", "ציבורי" (from `terms.md`) could not be checked against the Profile column of
  Hebrew Windows Defender Firewall; Windows Security may say "רשת תחום", "רשת פרטית", "רשת ציבורית". If Windows uses
  the longer forms, `core-review-firewall-detail` would need "ברשתות: { $networks }" instead of "ברשתות מסוג
  { $networks }". A native speaker with Hebrew Windows should confirm.
- "מועד ההפעלה" (when it starts) for a service or driver: "הפעלה" is also this program's word for activation ("נתוני
  רישיון והפעלה") and for running a program. The Services console may call the start type "סוג אתחול"; confirm the
  words fit.
- "לכל היותר Windows XP" (Windows XP at most) for "Windows XP or older", and "32 סיביות (x86)" for "32-bit x86", were
  chosen so the phrase starts with a Hebrew word or a digit. A native speaker may prefer other words that also do.
- `app-column-arch` is "מעבד" (processor), not "ארכיטקטורה" (architecture, too wide for the column). Confirm it reads as
  the kind of program, not the PC's processor.
- `ui-disclosure` says "מורחב" / "מכווץ" (expanded / collapsed) where the English says "open" / "closed"; confirm with
  Narrator in Hebrew that this is what a listener expects.
- "תוכנית התקנה" for installer and "הסרת התקנה" for uninstall follow Hebrew Windows, but "התקנה" also means an install;
  sentences such as `installer-interrupted` ("ההתקנה תבטל אותה תחילה", the install will undo it first) rely on context.

Checker: he-IL: 0 errors, 0 warnings.
