# Review: Ukrainian (uk-UA)

Reviewer: independent machine pass, 2026-11-15. Translator's form of address: plural "ви", lowercase, with plain
imperatives on buttons and in steps (Виберіть, Закрийте, Перезавантажте); kept, because that is how Ukrainian Windows 10
and 11 address the user.

Genuine Ukrainian, not Russian-influenced: the whole language was scanned for Russian-only letters (ы, э, ъ, ё), Russian
function words, active participles in -ючий/-ячий, "при …" and "по …" calques, and about 60 known Russianisms (являється,
на протязі, слідуючий, співпадати, в залежності від, включаючи, даний, існуючий, відмінити, по замовчуванню, ключ
реєстру, задача, вірно, and others). The text is clean: "включити" is used only for "include", on and off are
"увімкнути"/"вимкнути", registry keys and values are "розділ"/"параметр" as in Ukrainian Windows, and "інсталювати" is
Microsoft's Ukrainian word. One calque was found and fixed: `core-error-io` "вводу-виводу" (from Russian "ввод-вывод")
is now the standard "введення-виведення".

## Brought up to date

30 messages checked against the current English; 24 needed changes, and 6 already said the same.

- `app-scan-failed`: the reason now follows a colon ("Не вдалося просканувати цю програму: …"). The Ukrainian errors
  that fill it already start in lowercase.
- `app-footprint-licenses-still-go`, `core-footprint-licenses-left-out`, `core-review-note-licenses`,
  `core-secret-why-license-option-off`, `core-warning-scan-license-data`: "параметр «Ліцензія»" is now "параметр «Дані
  ліцензії та активації»", the words of `app-option-license`.
- `core-warning-build-recorded-folder-left-out`, `core-warning-build-near-part-left-out` `[unreadable]`: was "її не можна
  прочитати" (it cannot be read); now the safety rule: "її шлях виводить за межі диска або веде через посилання, тому
  читати її небезпечно".
- `core-review-start-boot`, `-system`, `-automatic`, `-delayed`, `-manual`, `-disabled`, `-unknown` (added): each
  carries its own verb, "запускається …", and the disabled one is "ніколи не запускається (вимкнуто)", because
  Ukrainian needs "не" with "ніколи". `core-review-service-detail` `[other]` and `core-review-driver-detail-starts` now
  end "…, { $start }" instead of "тип запуску: { $start }", which had shown Windows' raw English value ("boot").
- `core-review-network-domain`, `-private`, `-public` (added): "доменна", "приватна", "загальнодоступна", agreeing with
  "мережа". `core-review-firewall-detail` now reads "…для { $program }, тип мережі: приватна" (or "типи мереж: доменна,
  приватна"; "в усіх мережах" when there are none). The old "у мережах { $networks }" would not read well with the
  adjectives.
- `core-review-com-support-detail`: each count now follows its kind: "Параметри її бібліотек типів ({ $libraries }),
  інтерфейсів ({ $interfaces }) і AppID ({ $app_ids }), пов'язані з її класами".
- `installer-license-detail`: "Файли й ключі" (could read as product keys) is now "Файли активації та ліцензій, а також
  параметри реєстру, у яких зберігаються ліцензії", and "Деякі програми" is now "Деякі ліцензії".
- `installer-no-programdata`, `installer-cli-no-programdata`: was "so the install could not be undone"; now "тому
  нічого не інстальовано: саме там інсталяція зберігає запис, за яким її скасовують".
- `core-rollback-setting-put-back`, `core-rollback-list-entries-removed`: comment only (the "capital The" note is gone in
  English); the Ukrainian never had the capital.
- Already the same, so not changed: `client-builder-done`, `client-wrong-code`, `client-pair-failed` (in Ukrainian
  "іншим ПК" sits mid-sentence, so lowercase is right), `core-rollback-list-entries-not-removed`,
  `core-rollback-firewall-removed`, `ui-disclosure`.

Comments of four messages also changed in English without being on the list. They now match English: `app-scan-stopped`,
`core-error-io`, `core-rollback-count-kept`, and `core-rollback-count-left-behind`. The last one's new note showed that
its text was too vague (see the sample).

## Sample (back-translation)

| Message | Back-translation | Verdict |
| --- | --- | --- |
| ui-clear-search | Clear search | OK |
| ui-browse | Browse (Windows' "Огляд") | OK |
| ui-browse-tooltip | Choose a folder for the "{field}" field | OK |
| ui-disclosure | {title}, {summary}, expanded / collapsed | OK (Narrator's words for open and closed) |
| ui-language | Language | OK |
| ui-machine-translated | This translation was done automatically, so it may contain errors. | OK |
| ui-report-translation-problem | Report a problem with the translation | OK |
| ui-needs-direct3d-12 | Displaying the window needs Direct3D 12, which an old graphics driver or a virtual machine may not have. | OK |
| size-bytes | {value} byte / bytes (one, few, many; other for fractions) | OK |
| size-kb | {value} KB (КБ) | OK |
| size-mb | {value} MB (МБ) | OK |
| size-gb | {value} GB (ГБ) | OK |
| size-tb | {value} TB (ТБ) | OK |
| list-separator | ", " | OK |
| installer-window-title | WinAppMigrator installer | OK |
| installer-window-title-app | Installation of the program {app_name} | OK |
| installer-tagline | Reinstalls a program obtained from another PC | OK |
| installer-unnamed-app | Program | OK |
| installer-window-failed | Could not open the installer's window: {error} | OK |
| installer-window-failed-without-one | You can install without it too: run this file from a command prompt opened as administrator with --apply, and to choose a folder add --target-root and its path. | OK |
| installer-checking | Checking the installer… | OK |
| installer-stub-no-package | This is a WinAppMigrator installer stub and holds no app package. Create an installer in WinAppMigrator, then run here the created .exe. | Fixed: "запустіть тут створений файл" could read as "run the file created here"; now "запустіть тут файл .exe, який він створить" |
| installer-check-stopped | Checking the installer was stopped. | OK |
| installer-locate-failed | Could not find the installer: {error} | OK |
| installer-read-failed | Could not read the installer: {error} | OK |
| installer-cannot-be-used-here | This installer's package cannot be used on this PC: {error} | OK |
| installer-captured-from | Obtained from | OK |
| installer-install-location | Install location | OK |
| installer-choose-location | Choose the install location | OK |
| installer-folder-has-files | This folder already has files. Files with the same names will be replaced, and if you uninstall the program they come back. | OK |
| installer-room-needed | Space needed | OK |
| installer-room-short | There may not be enough space here. {shortfall}. The package can be unpacked on another drive too, but the program's files still have to fit here. | OK (a full stop, not English's colon: the Ukrainian `core-space-shortfall` is a whole sentence starting "На …") |
| installer-interrupted | The earlier install of {app_name} was not finished. It will be undone before installing. | OK |
| installer-signature | Installer | OK |
| installer-signature-checking | Checking the signature… | OK |
| installer-signature-unsigned | Not signed: publisher unknown | OK |
| installer-signature-signed-by | Signed: {publisher} | OK |
| installer-signature-unnamed | Signed by a publisher with no name | OK |
| installer-signature-invalid | This installer's signature is invalid ({reason}). The installer may have been changed after signing. | OK |
| installer-more-left-out | {count} more item(s) not included (one, few, many) | OK |
| uninstaller-plan-coming-back | {count} file(s) replaced by this install will come back or be placed beside the file(s) that changed since. | OK |
| uninstaller-remove-user-data-detail | {count} item(s) of the program's settings, data, and license. It/they stay unless this option is on. What it/they replaced comes back or, if changed since, is placed beside it/them. | OK |
| uninstaller-not-all-undone | Not everything could be undone. Close what "this" uses, and uninstall again in Settings. | Fixed: "те, що це використовує" was garbled; now "те, що використовує програму" |
| app-tagline | Turn an installed program into a self-extracting installer | OK |
| app-reload-list | Refresh the program list | OK |
| app-list-count | {count} program(s) | OK |
| app-list-search | Search by name or publisher  (Ctrl+F) | OK (Ukrainian Windows writes Ctrl+F) |
| app-column-name | Name | OK |
| app-column-publisher | Publisher | OK |
| app-column-version | Version | OK |
| app-column-scope | Scope | OK |
| app-column-arch | Arch. | OK, short |
| app-scope-per-machine | For the whole PC | OK |
| app-option-license | License and activation data | OK |
| app-option-user-data | User data | OK |
| app-option-whose-data | Whose data | OK |
| app-output-folder | Folder to save to | OK |
| app-build-create | Create installer | OK |
| app-scan-look-again | Scan again | OK |
| app-result-copy-path | Copy path | OK |
| app-network-connect | Connect (something) | Fixed: transitive "Підключити"; now "Підключитися", as on Windows' own Connect buttons |
| app-scan-failed | Could not scan this program: {error} | Fixed (brought up to date) |
| core-rollback-record-damaged | Part of the record of {app_name} could not be read, so everything it describes stayed in place, and a file the install replaced may not come back. | OK |
| core-rollback-folder-left | Folder {path} was left: {reason}. | OK |
| core-rollback-key-not-removed | Could not remove the key {name}: {error}. | OK |
| core-rollback-task-folder-holds-others | Task folder {path} stays, as it holds tasks not added by this install. | OK |
| core-rollback-count-left-behind | Remained: {count} | Fixed: vague next to "Залишено на місці"; now "Залишається в системі" (stays on the system), as English's new note means |
| core-system-own-system-drive | {drive} is this PC's own system drive. To read this PC, choose "This PC". | OK |
| core-warning-scan-network-paths-not-followed-more | Some paths recorded on this PC could not be read, so nothing from there was included. Each points off this PC's disks (looking would connect) or has a name that would open something else: {paths} and {more} more | OK |
| core-runtime-winget-misplaced | {path} is not where Windows keeps its own packages, so it was not run | OK |
| core-profile-nobody-signed-in-running-not-a-person | nobody is signed in here, and {account}, under which this runs, is not a person's account | OK |
| core-space-shortfall | {drive} needs up to {needed} free for {purposes}, and {free} is free | OK |
| core-policy-fit-older-build | This program ran on Windows build {from_build}, and this PC runs build {this_build}. Its executables say they can start on this Windows, so it may work, but what it relied on there may not be here. | OK |
| core-warning-restore-prerequisite-reboot | {name} was installed, but the computer has to restart to finish. | OK |
| core-image-already-attached | the image is already attached in Windows, and whatever attached it can write to it. Detach it first, or choose its drive as a connected drive | OK |
| core-image-differencing | {path} is a differencing disk holding only the changes against its parent, which Windows would open too, wherever it says it is. Merge it with Hyper-V's Edit Disk first, and choose that disk | OK |
| core-error-io | I/O error ("вводу-виводу") — {path}: {error} | Fixed: Russian calque; now "введення-виведення" |
| core-review-firewall-detail | Allows incoming connections on port {ports} for {program}, network type: {networks} | Fixed (brought up to date, see above) |

78 messages, 7 fixed (two of them as part of bringing the catalog up to date).

Whole-language checks: the `app-column-*` headers are short (Назва, Видавець, Версія, Область, Арх.); button labels are
short; nothing is left in English that should not be (the checker reports no "still English" warnings); key names are
written Ctrl+F, as Ukrainian Windows writes them.

## Other fixes

- `uninstaller-not-all-undone`: garbled object (see the sample).
- Comments of `app-scan-stopped`, `core-error-io`, `core-rollback-count-kept`, `core-rollback-count-left-behind`,
  `core-rollback-setting-put-back`, `core-rollback-list-entries-removed`, `core-review-service-detail`,
  `core-review-driver-detail-starts`, and `app-footprint-licenses-still-go` now match English. Every comment in the
  language now matches English's.

## Open questions

- Firewall profiles: I could not confirm Windows Defender Firewall's Ukrainian profile names. I used "доменна /
  приватна / загальнодоступна" (мережа), after Windows 11 Settings ("Тип мережевого профілю": "Приватна мережа",
  "Загальнодоступна мережа"). If wf.msc says "Домен / Приватний / Загальнодоступний" (профіль), a native speaker may
  prefer those, with the frame changed to "профілі: …".
- `core-review-start-*` in Ukrainian include the verb "запускається" (and "ніколи не запускається (вимкнуто)"), and their
  frames have none. Whoever edits them must keep the verb inside each one.
- "(За замовчуванням)" for (Default), and "Програми за замовчуванням": these are Windows' own Ukrainian words, though
  purists prefer "типово". I kept them, following the glossary.
- I kept "Служба захисту користувачів" for User Account Control, as Ukrainian Windows names it to my knowledge. It should
  be confirmed.
- `app-output-folder-example` translates the user name in the path, "C:\Users\Ви\WinAppMigrator". It is readable, but a
  native speaker may prefer a placeholder name.

Checker: uk-UA: 0 errors, 0 warnings.
