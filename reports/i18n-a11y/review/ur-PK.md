# Review: Urdu (ur-PK)

Reviewer: independent machine pass, 2026-11-15. Translator's form of address: آپ, with the polite imperative in ـیں
("منتخب کریں", "دوبارہ کوشش کریں") and passive forms ("نہیں کیا جا سکا", "واپس رکھ دیا گیا") for what happened; kept,
because that is how Windows addresses the user in Urdu. No direction marks anywhere in the text (checked: no U+200E,
U+200F, U+061C, embeddings or isolates); the program lays out right-to-left text and isolates placeholders itself. Urdu
letter forms throughout (ی ے ہ ھ ک; no Arabic ي ك ه ى ة), digits 0 to 9, Urdu "۔" "،" "؛" after Urdu text.

## Brought up to date

30 messages: `app-scan-failed`, `app-footprint-licenses-still-go`, `client-builder-done`, `client-wrong-code`,
`client-pair-failed`, `core-footprint-licenses-left-out`, `core-warning-build-recorded-folder-left-out`,
`core-warning-build-near-part-left-out`, `core-review-start-boot`, `core-review-start-system`,
`core-review-start-automatic`, `core-review-start-delayed`, `core-review-start-manual`, `core-review-start-disabled`,
`core-review-start-unknown`, `core-review-network-domain`, `core-review-network-private`, `core-review-network-public`,
`core-review-com-support-detail`, `core-review-note-licenses`, `core-rollback-setting-put-back`,
`core-rollback-list-entries-removed`, `core-rollback-list-entries-not-removed`, `core-rollback-firewall-removed`,
`core-secret-why-license-option-off`, `core-warning-scan-license-data`, `installer-license-detail`,
`installer-no-programdata`, `installer-cli-no-programdata`, `ui-disclosure`.

All 30 were present, in English's position, and the translator had already folded in the new English. Each was checked
against the current English; 29 already said the same, 1 was reworded:

- `app-scan-failed`: the reason follows a colon ("اس ایپ کا جائزہ نہیں لیا جا سکا: { $error }").
- The option's new name: `app-footprint-licenses-still-go`, `core-footprint-licenses-left-out`,
  `core-review-note-licenses`, `core-secret-why-license-option-off`, `core-warning-scan-license-data` all say
  "لائسنس اور فعال کاری کے ڈیٹا کا اختیار", the words of the checkbox `app-option-license` ("لائسنس اور فعال کاری کا ڈیٹا").
- `[unreadable]` in both left-out warnings is the safety rule: "اس کا پاتھ ڈرائیو سے باہر جاتا ہے یا کسی لنک سے گزرتا ہے،
  اس لیے اسے محفوظ طریقے سے پڑھا نہیں جا سکتا" (its path goes off the drive or through a link, so it cannot safely be read).
- `installer-license-detail`: "وہ رجسٹری اقدار جن میں لائسنس محفوظ ہوتے ہیں" (the registry values in which licenses are
  kept), not product keys.
- `installer-no-programdata`, `installer-cli-no-programdata`: "اس لیے کچھ انسٹال نہیں کیا گیا: تنصیب وہ ریکارڈ وہیں رکھتی ہے
  جس کے ذریعے اسے کالعدم کیا جاتا ہے" (so nothing was installed: an install keeps there the record it is undone by).
- `core-review-start-*`: phrases, filled into `core-review-service-detail` `[other]` and
  `core-review-driver-detail-starts` after "شروع ہوتی ہے:" / "شروع ہوتا ہے:" ("starts:"), feminine for the service and
  masculine for the driver. The colon frame is what lets every phrase read: placed straight before the verb,
  "کبھی نہیں، کیونکہ یہ غیر فعال ہے" (never, as it is disabled) and "صرف اس وقت جب کوئی چیز اسے کہے" would not. Checked
  with each of the seven phrases filled in.
- `core-review-network-*`: ڈومین، نجی، عوامی, as Windows names network profiles in Urdu; filled in they read
  "ڈومین، نجی نیٹ ورکس پر" (on domain, private networks).
- `core-review-com-support-detail`: each count follows its own kind ("ٹائپ لائبریریوں ({ $libraries })…").
- `core-rollback-*` (4): Urdu has no capitals, so the "The" fix needs nothing; wording checked.
- `client-builder-done`, `client-pair-failed`: `[unknown]` "کسی دوسرے PC" starts the sentence, in the oblique form the
  postposition after it needs; Urdu has no capitals.
- `client-wrong-code`: reworded. "نے ایسا کوڈ آزمایا جو یہ کوڈ نہیں ہے" (tried a code that is not this code) is now
  "نے ایسا کوڈ آزمایا جو یہاں دکھائے گئے کوڈ سے مختلف ہے" (tried a code that differs from the code shown here).
- `ui-disclosure`: present; "کھلا ہے" / "بند ہے" (is open / is closed).

## Sample (back-translation)

Back-translated from the Urdu, then compared with the English. 86 messages.

| Message | Back-translation | Verdict |
| --- | --- | --- |
| ui-clear-search | Clear the search | OK |
| ui-browse | Browse | OK |
| ui-browse-tooltip | Choose a folder for { $field } | OK |
| ui-disclosure | { $title }, { $summary }, is open / is closed | OK |
| ui-language | Language | OK |
| ui-machine-translated | This translation was made by machine, and there may be mistakes in it. | OK |
| ui-report-translation-problem | Report a problem of the translation | OK |
| ui-needs-direct3d-12 | To make its window it needs Direct3D 12, which cannot be in an old graphics driver or a virtual machine. | Fixed: "نہیں ہو سکتا" read as "cannot be", not "may not have"; now "شاید موجود نہ ہو" (may not be there) |
| size-bytes | { $value } byte / bytes | OK |
| size-kb | { $value } KB | OK (standard symbol) |
| size-mb | { $value } MB | OK |
| size-gb | { $value } GB | OK |
| size-tb | { $value } TB | OK |
| list-separator | "، " | OK (Urdu comma) |
| installer-window-title | WinAppMigrator installer | OK |
| installer-window-title-app | Install { $app_name } | OK |
| installer-tagline | Reinstalls an application obtained from another PC | OK |
| installer-unnamed-app | Application | OK |
| installer-window-failed | The installer's window could not open: { $error } | OK |
| installer-window-failed-without-one | It can also install without the window: run this file from an administrator's command prompt with --apply, and to choose the place, write a folder after --target-root. | OK |
| installer-checking | The installer is being checked… | OK |
| installer-stub-no-package | This is a WinAppMigrator installer stub, and there is no application package in it. Build an installer with WinAppMigrator, then run the built .exe file here. | OK |
| installer-check-stopped | The check of the installer stopped. | OK |
| installer-locate-failed | Where the installer is could not be found out: { $error } | OK |
| installer-read-failed | The installer could not be read: { $error } | OK |
| installer-cannot-be-used-here | This installer's package cannot be used on this PC: { $error } | OK |
| installer-captured-from | Where it was obtained from | OK |
| installer-install-location | Install location | OK |
| installer-choose-location | Choose the install location | OK |
| installer-folder-has-files | This folder already has files. Files with the same names are replaced, and come back if you uninstall. | OK |
| installer-room-needed | Space needed | OK |
| installer-room-short | There may not be enough space here: { $shortfall }. The package can be unpacked on another drive, but there must still be space here for the app's files. | OK ("here" says where they must fit) |
| installer-interrupted | An earlier install of { $app_name } never finished. On installing, it is undone first. | OK |
| installer-signature | Installer | OK |
| installer-signature-checking | Its signature is being checked… | OK |
| installer-signature-unsigned | Not signed: publisher unknown | OK |
| installer-signature-signed-by | Signer: { $publisher } | OK |
| installer-signature-unnamed | Signed of an unnamed publisher | Fixed: "کے دستخط شدہ" is not grammatical; now "کسی بے نام ناشر کی جانب سے دستخط شدہ" (signed by an unnamed publisher) |
| installer-signature-invalid | This installer's signature is not valid ({ $reason }). It may have been changed after signing. | OK |
| installer-more-left-out | { $count } more thing was / things were left out | OK |
| uninstaller-plan-coming-back | The { $count } file(s) this install replaced come back, or, if the current file has changed since, are put beside it. | OK |
| uninstaller-remove-user-data-detail | { $count } item(s) of its settings, data and license. Unless this option is on, it stays. What it replaced comes back, or if it has changed since, is put beside it. | OK |
| app-list-search | Search by name or publisher (Ctrl+F) | OK (Urdu Windows writes Ctrl) |
| app-list-filter-all | All | OK |
| app-column-name | Name | OK |
| app-column-publisher | Publisher | OK |
| app-column-version | Version | OK |
| app-column-scope | Scope | OK |
| app-column-arch | Architecture | Fixed: about 45 px at the header's 12 px semibold, in a 46 px cell, with 15 px more for the sort arrow; now "آرک" (Arch), abbreviated as the English is; the sort label, which is also what a screen reader says, keeps "آرکیٹیکچر" |
| app-column-arch-sort | Sort by architecture | OK |
| app-source | Source of the scan | OK |
| app-source-network | Network PC… | OK |
| app-source-disconnect | Disconnect | OK |
| app-option-license | License and activation data | OK |
| app-option-user-data | User's data | OK |
| app-option-whose-data | Whose data | OK |
| app-build-create | Make installer | OK |
| app-build-cancel | Cancel | OK |
| app-scan-look-again | Look again | OK |
| app-scan-try-again | Try again | OK |
| app-result-open-folder | Open folder | OK |
| app-result-copy-path | Copy path | OK |
| app-network-connect | Connect | OK |
| app-room-drive-short | To build this, { $drive } needs at most { $needed } of free space, and has { $free }. | OK |
| app-result-files-gone | Installer built, but since the app was looked at { $count } file(s) had gone, so it is / they are not in it. See warnings below. | OK |
| app-footprint-folders-summary | { $included } of { $count } folder(s), { $size } | OK |
| core-rollback-file-left | The file { $path } was left: { $reason }. | OK |
| core-rollback-task-put-back | The task { $name } was put back | OK |
| core-rollback-reason-moved | { $folder } now leads to { $actual }, not to where it was during the install | OK |
| core-task-too-large | Its size is { $size } bytes, which is too big to be a task | OK |
| core-runtime-download-not-https | Downloading { $url } was refused: it is not HTTPS | OK |
| core-package-not-on-the-drive | { $path } is not on the connected drive, so it was not read | OK |
| core-discovery-unsafe-path | Unsafe path: { $path } | OK |
| core-archive-data-file-wrong | { $name } is not this installer's data file, or it is damaged or was changed after being built, so nothing was read from it. Copy both files again from where they were built. | OK |
| core-warning-build-empty-folders-truncated | This app has more than { $count } empty folders, and only that many will be made again. | OK |
| core-restore-location-unusable | That install location cannot be used: { $problem }: "{ $path }" | OK |
| core-restore-added-firewall-rule | Firewall rule { $name } was added for { $program } | OK |
| core-restore-would-restore-task | Scheduled task { $name } will be restored | OK |
| core-warning-restore-prerequisite-pending-reboot | { $name } is not registered yet; after a restart it should become active. | OK |
| core-review-task-detail | Runs with this account: { $account } | OK (reads with both fill-ins, "آپ، جو بھی سائن ان ہو" and "وہ صارف جس نے اسے بنایا") |
| core-review-certificate | { $name } in { $store } | OK |
| core-policy-app-id-started | It would change how { $name } is started, which this package does not install | OK |
| core-warning-verify-registry-value-missing | Registry value is not there or cannot be read: { $error } | OK |
| core-review-service-detail | Runs { $file } as { $account }, starts now and each time this PC starts / … starts: { $start } | OK |
| core-review-driver-detail-starts | Loads { $file } into Windows itself, starts: { $start } | OK |
| core-review-firewall-detail | For { $program }, on { $networks } networks, on port { $ports }, allows incoming connections | Fixed: singular "پورٹ" (port) stood before a list of several ports; added `[one]`, and several now read "پورٹس" (ports) |

## Other fixes

- `terms.md`: the architecture row now records the column header's short form, "آرک", beside "آرکیٹیکچر".
- Checked across the whole language, with nothing more to fix: no Arabic letter forms, direction marks or Eastern
  digits; ھ only for aspiration and ہ elsewhere (all 1,045 distinct words checked, e.g. بھی، رکھ، کھولیں، کہ، یہ); no
  Latin full stops or commas after Urdu text; button labels about as wide as the English (the widest, "USB ڈرائیوز
  دوبارہ تلاش کریں", 5% wider); the other column headers fit their columns. The Latin text that is left is product names,
  paths, flags, codes, and names the glossary keeps (Windows Installer, App Paths, Hyper-V's Edit Disk, the Security log,
  side-by-side).

## Open questions

- `app-column-arch` "آرک" abbreviates the English word; it is not an established Urdu abbreviation. Is it understood
  above x86/x64/ARM64, or is there a short Urdu word that fits about 45 px?
- Firewall profiles ڈومین / نجی / عوامی: taken from how Urdu Windows names network types (نجی نیٹ ورک، عوامی نیٹ ورک), not
  confirmed against an Urdu Windows install; Windows Defender Firewall's own console may stay in English under the Urdu
  language pack.
- `core-review-start-*` in the "شروع ہوتی ہے: …" frame read as a label and its value; a native speaker may prefer a
  frame that puts the phrase before the verb, which would need "disabled" and "manual" reworded.
- Windows' own words in `terms.md` (ترتیبات، رسائی پذیری، منتظم، صارف اکاؤنٹ کنٹرول، فعال کاری، انسٹال کردہ ایپس) could
  not be confirmed against an Urdu Windows install.

Checker: ur-PK: 0 errors, 0 warnings.
