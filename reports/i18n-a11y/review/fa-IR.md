# Review: Persian (fa-IR)

Reviewer: independent machine pass, 2026-11-15. Translator's form of address: «شما» with the polite plural imperative
(«انتخاب کنید»، «وارد کنید»، «ببندید»); kept, because Windows 11 in Persian addresses the reader the same way.

The translator's notes file has no fa-IR section, so only the general points were checked against this language.

Script and characters, checked across all nine files, not only the sample:

- No direction marks or bidi controls anywhere (no LRM, RLM, ALM, LRE/RLE/PDF, LRO/RLO, LRI/RLI/FSI/PDI). The program
  isolates placeholders and lays out the text itself.
- Persian letters only: no Arabic ي (U+064A), ك (U+0643), ى, or ة in any message. (`terms.md` names ي and ك once, on
  purpose, as the forms not to use.)
- Digits stay 0 to 9; no Persian or Arabic-Indic digits.
- Zero-width non-joiner is in every می‌ / نمی‌ / ‌ها / ‌های / ‌ای / ‌تر / ‌شده position where the letter before it joins.
  The only plural suffixes written without one follow a non-joining letter (ا د ر ز و: «کارها»، «مسیرهای»، «خطاها»),
  where it makes no difference. No ZWNJ next to a space, a brace, a Latin letter, or doubled.
- Persian punctuation throughout (، ؛ « »). No ASCII comma, semicolon, question mark, or double quote next to Persian
  text. No Persian suffix is glued to a placeholder.
- Tanwin appears only on the standard adverbs (مثلاً، مستقیماً، احتمالاً، معمولاً، واقعاً).

## Brought up to date

30 messages. All 30 were already in the file, in English's position, and the translator had already followed the new
English in most of them. I checked each one against the current English. 3 needed an edit:

- `core-warning-build-recorded-folder-left-out`, `core-warning-build-near-part-left-out`: in `[unreadable]`,
  «نمی‌توان آن را با اطمینان خواند» ("cannot be read with confidence", which reads as a reliability problem) became
  «خواندن آن ایمن نیست» ("reading it is not safe"), so the reason reads as a safety rule.
- `core-review-start-unknown`: «به روشی که Windows مشخص نمی‌کند» ("in a way…") became «در زمانی که Windows مشخص
  نمی‌کند» ("at a time Windows does not specify"). The start words follow the label «زمان شروع:» ("start time:") in
  `core-review-service-detail` `[other]` and `core-review-driver-detail-starts`. That colon form is the right choice
  for Persian, because «هرگز» ("never") needs a negative verb and cannot be followed by «شروع می‌شود».

Checked and left as they are:

- `app-scan-failed`: { $error } now follows a colon.
- `app-footprint-licenses-still-go`, `core-footprint-licenses-left-out`, `core-review-note-licenses`,
  `core-secret-why-license-option-off`, `core-warning-scan-license-data`: all name گزینه «داده‌های مجوز و فعال‌سازی»,
  using the same words as `app-option-license`.
- `client-builder-done`, `client-wrong-code`, `client-pair-failed`: `[unknown]` «رایانه‌ای دیگر» starts the sentence
  correctly. (Persian has no capital letters.)
- `core-review-start-boot`, `-system`, `-automatic`, `-delayed`, `-manual`, `-disabled` (new): read correctly after
  «زمان شروع:».
- `core-review-network-domain`, `-private`, `-public` (new): دامنه، خصوصی، عمومی, which is how Windows names these
  network types in Persian. In context: «در شبکه‌های دامنه، خصوصی».
- `core-review-com-support-detail`: each count follows its own kind, as in the English.
- `core-rollback-setting-put-back`, `core-rollback-list-entries-removed`, `core-rollback-list-entries-not-removed`,
  `core-rollback-firewall-removed`: these match the new English (the capital-"The" fix has no effect in Persian).
- `installer-license-detail`: «مقادیر رجیستری که مجوزها را در خود دارند» says registry values, not product keys.
- `installer-no-programdata`, `installer-cli-no-programdata`: «چیزی نصب نشد» ("nothing was installed"), and the
  reason is that every install keeps the record it is undone by there.
- `ui-disclosure` (new): «{ $title }، { $summary }، بازشده / جمع‌شده».

## Sample (back-translation)

76 messages.

| Message | Back-translation | Verdict |
| --- | --- | --- |
| ui-clear-search | Clear search | OK |
| ui-browse | Browse | OK |
| ui-browse-tooltip | Choose a folder for «field» | OK |
| ui-disclosure | { title }, { summary }, opened / collapsed | OK |
| ui-language | Language | OK |
| ui-machine-translated | This translation was done by machine and may have mistakes. | OK |
| ui-report-translation-problem | Report a translation problem | OK |
| ui-needs-direct3d-12 | This program needs Direct3D 12 to draw its window, which an old graphics driver or a virtual machine may not have. | OK |
| size-bytes | { value } byte(s) (noun stays singular after a number, as Persian does) | OK |
| size-kb | { value } kilobytes | OK |
| size-mb | { value } megabytes | OK |
| size-gb | { value } gigabytes | OK |
| size-tb | { value } terabytes | OK |
| list-separator | «، » (Persian comma and space) | OK |
| installer-window-title | WinAppMigrator installer | OK |
| installer-window-title-app | Install { app } | OK |
| installer-tagline | Reinstalls a program that was taken from another PC | OK |
| installer-unnamed-app | Program | OK |
| installer-window-failed | The installer's window did not open: { error } | OK |
| installer-window-failed-without-one | It can still be installed without a window: run this file from a command prompt with administrator rights with --apply, and to choose the place, give --target-root with a folder. | OK |
| installer-checking | Checking the installer… | OK |
| installer-stub-no-package | This is a WinAppMigrator installer stub and holds no program package. Build an installer with WinAppMigrator, then run the built .exe file here. | OK |
| installer-check-stopped | The check of the installer stopped. | OK |
| installer-locate-failed | The installer was not found: { error } | OK |
| installer-read-failed | The installer was not read: { error } | OK |
| installer-cannot-be-used-here | This installer's package cannot be used on this PC: { error } | OK |
| installer-captured-from | Taken from | OK |
| installer-install-location | Install location | OK |
| installer-choose-location | Choose the install location | OK |
| installer-folder-has-files | This folder already has files. Files with the same name are replaced, and if you uninstall, they come back. | OK |
| installer-room-needed | Space needed | OK |
| installer-room-short | There may not be enough space here: { shortfall }. The package can be unpacked on another drive, but the program's files still have to fit. | OK |
| installer-interrupted | The earlier install of { app } never finished. «Install» undoes it first. | OK |
| installer-signature | Installer | OK |
| installer-signature-checking | Checking its signature… | OK |
| installer-signature-unsigned | Not signed: unknown publisher (Windows' «ناشر ناشناخته») | OK |
| installer-signature-signed-by | Signed by { publisher } | OK |
| installer-signature-unnamed | Signed by an unnamed publisher | OK |
| installer-signature-invalid | This installer's signature is not valid ({ reason }). It may have been changed after signing. | OK |
| installer-more-left-out | { n } more items were left out (one / other) | OK |
| uninstaller-plan-coming-back | { n } file(s) this install replaced come back, or, if the file that took its place has changed since, go beside it (one / other) | OK |
| uninstaller-remove-user-data-detail | { n } item(s) of its settings, data, and license. They stay unless this option is on. What they replaced comes back, or, if they have changed since, goes beside them. (one / other) | OK |
| app-tagline | Turn an installed program into a self-extracting installer | OK |
| app-reload-list | Reload the list of programs | OK |
| app-list-search | Search by name or publisher  (Ctrl+F) | OK. Persian Windows writes Ctrl. |
| app-column-name | Name | OK |
| app-column-publisher | Publisher | OK |
| app-column-version | Version | OK |
| app-column-scope | Scope | OK |
| app-column-arch | Architecture | OK. No Persian abbreviation exists, and the word is short (see Open questions). |
| app-scope-per-user | User-specific | OK |
| app-scope-per-machine | Machine-specific | OK |
| app-option-license | License and activation data | OK |
| app-option-user-data | User data | OK |
| app-option-whose-data | Whose data | OK |
| app-output-folder | Output folder | OK |
| app-build-create | Create installer | OK |
| app-build-cancel | Cancel | OK |
| app-scan-look-again | Look again | OK |
| app-result-copy-error | Copy error | OK |
| app-network-connect | Connect | OK |
| app-result-files-not-captured | Installer built, but { n } file(s) were not captured, usually because locked or unreadable. The program may not work without it/them. See «Warnings» below. | OK |
| core-rollback-prerequisite-stays | { name } was installed, or may have been, and stays, because other programs may use it. | OK |
| core-rollback-file-stays-beside | { path } stays, because… The file it had replaced is beside it, named { name }. | OK |
| core-rollback-service-account-again | Service { name } ran as { account }. Its password is not kept, so set its account again in «Services». | OK |
| core-system-hive-too-large | The size of { path } is { size } bytes, which is too large for a registry hive, or more than all this drive's hives together | OK |
| core-warning-scan-dependency-not-in-newer-windows | { name } was in the Windows folder of the PC this program was on, and Windows 10 and 11 do not come with it… | OK |
| core-space-shortfall | { drive } needs up to { needed } free space for { purposes } and has { free } (read with the core-space-for- words filled in) | OK |
| core-policy-fit-needs-newer | { program } says it needs Windows { needs } or newer, and this PC runs { runs }, so Windows will not start it | OK |
| core-policy-fit-emulated | This PC is { here }, so Windows runs these through an emulator: { programs }. Most programs have no problem that way, but some do. | OK |
| core-review-account-signed-in | User data and per-user settings go to { account }, who is signed in here, and not to the administrator whose account this runs as. | OK |
| core-review-firewall-detail | "Permission to incoming connections on port { ports } for { program }, on { networks } networks" | Fixed: «اجازه به» is a bare noun ("permission to"), not parallel with «مسدود کردن» ("blocking"). Now «اجازه دادن به» ("allowing"), Windows Firewall's own wording for Allow the connection. |
| core-archive-data-file-missing | This installer keeps its package in a file named { name }, in this same folder, and that file is not here. The two files have to stay together… | OK |
| core-warning-build-user-folder-not-on-drive | User { user }'s { folder } folder was at { path }, which is not on this drive, so its contents were not included. | OK |
| core-warning-restore-service-account | Service { name } ran as { account } on the source PC. Its password is not in the installer, so the service is set up as LocalService… | OK |
| core-warning-build-links-not-followed | { n } link(s), or names that would open something else, were not followed in this folder, so what they point at is not in the installer: { names }, and { more } more. | OK |

Whole-language checks:

- Column headers (`app-column-*`): نام، ناشر، نسخه، دامنه، معماری. Each is a single short word.
- Buttons are short: لغو، بستن، نصب، توقف، اتصال، تغییر…، مرور، بررسی دوباره، تلاش دوباره.
- Nothing is left in English that should be translated. The Latin text that remains is product names, paths,
  command-line flags, registry and command names, file formats, and the `_external` folder name, as the glossary asks.

## Other fixes

- `installer-dependency-included`, `core-runtime-handling-included`: «گنجانده شده» → «گنجانده‌شده». The ZWNJ makes the
  participle one word, as `terms.md` asks for ‌شده compounds and as the rest of the catalog writes it («گنجانده‌شده»،
  «امضاشده»، «بازیابی‌شده»).

No terms changed. `terms.md` is unchanged.

## Open questions

- **"administrator rights" as «حقوق سرپرستی».** Persian Windows uses «سرپرست» for administrator, but I could not
  confirm whether it says «حقوق»، «امتیازات», or «مجوزهای» for "rights". The term appears 15 times.
- **"firewall rule" as «قانون دیوار آتش»** (15 times). Microsoft's Persian has also used «قاعده» for "rule". Confirm against
  Windows Defender Firewall in Persian. The network profile words دامنه، خصوصی، عمومی should also be confirmed there.
- **`ui-disclosure` «بازشده / جمع‌شده».** Narrator may use its own Persian words for expanded and collapsed; a native
  speaker could align them.
- **`app-column-arch` «معماری» width.** It measures about 37 px in Segoe UI Semibold at 12 px, in a 46 px cell. The
  word fits, but when the list is sorted by this column, the sort chevron (drawn at text width + 10 px) runs about 6 px
  into the gutter. Persian has no usual abbreviation. Worth a look in the running app.
- **Service and driver start lines.** The `core-review-start-*` words follow «زمان شروع:» ("start time:"). A native
  speaker should confirm this reads naturally in a list row.
- **English source comment, not mine to change.** The comment above `core-review-firewall-detail` (copied from en-US)
  still says { $networks } arrives as English words, which is no longer true.

Checker: fa-IR: 0 errors, 0 warnings.
