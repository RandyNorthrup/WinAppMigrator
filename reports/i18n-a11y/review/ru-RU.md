# Review: Russian (ru-RU)

Reviewer: independent machine pass, 2026-11-15. Translator's form of address: "вы", lowercase mid-sentence, with plain
imperatives for actions (Выберите, Закройте, Перезагрузите компьютер); kept, because that is how Russian Windows 10 and
11 address the user (UAC: "Разрешить этому приложению вносить изменения на вашем устройстве?").

## Brought up to date

30 messages checked against the current English. 17 needed new or added words:

- `app-scan-failed`: the reason now follows a colon, not a full stop ("Не удалось проверить это приложение: …").
  `core-scan-no-executable` and `core-scan-not-a-file`, which fill it, already start with a small letter.
- `core-warning-build-recorded-folder-left-out`, `core-warning-build-near-part-left-out` `[unreadable]`: was "ее нельзя
  читать" (it may not be read); now the safety rule: "путь к ней выходит за пределы диска или проходит через ссылку,
  поэтому читать ее небезопасно".
- `core-review-start-boot`, `-system`, `-automatic`, `-delayed`, `-manual`, `-disabled`, `-unknown` (added). Russian
  cannot say "запускается никогда" (starts never), so each start word carries its own verb ("запускается при каждом
  запуске компьютера", "никогда не запускается: запуск отключен"), and `core-review-service-detail` `[other]` and
  `core-review-driver-detail-starts` now end ", { $start }" instead of "тип запуска: { $start }". "запуск отключен" is
  neuter, so it fits a service (feminine) and a driver (masculine) alike. The automatic, delayed, and manual words are
  the same as the `[automatic]`, `[delayed]`, and `[manual]` variants of `core-review-service-detail`.
- `core-review-network-domain`, `-private`, `-public` (added): "доменных", "частных", "общедоступных", the adjectives
  of Windows' "Сеть домена", "Частная сеть", "Общедоступная сеть", in the case the sentence needs.
  `core-review-firewall-detail` now ends "в { $networks } сетях" ("в частных, общедоступных сетях"); it said "в сетях
  типа { $networks }", which would have shown the English words.
- `core-review-com-support-detail`: each count now follows its own kind ("Параметры его библиотек типов (3),
  интерфейсов (12) и AppID (1), относящиеся к его классам"); it said "Параметров каждого вида: 3, 12 и 1".
- `installer-license-detail`: "Файлы и ключи активации и лицензии" (could be read as product keys) is now "Файлы
  активации и лицензий, а также параметры реестра, в которых хранятся лицензии".
- `installer-no-programdata`, `installer-cli-no-programdata`: was "so the install could not be undone"; now "поэтому
  ничего не установлено: в ней хранится запись, по которой установку можно отменить".

13 already said what the current English says: `app-footprint-licenses-still-go`, `client-builder-done`,
`client-wrong-code`, `client-pair-failed`, `core-footprint-licenses-left-out`, `core-review-note-licenses`,
`core-rollback-setting-put-back`, `core-rollback-list-entries-removed`, `core-rollback-list-entries-not-removed`,
`core-rollback-firewall-removed`, `core-secret-why-license-option-off`, `core-warning-scan-license-data`, `ui-disclosure`.
"The License and Activation Data option" is «Данные лицензии и активации» everywhere, the words of `app-option-license`.
The `client-*` `[unknown]` variants start their sentences with a capital ("Другой компьютер", "Другому компьютеру").

## Sample (back-translation)

| Message | Back-translation | Verdict |
| --- | --- | --- |
| ui-clear-search | Clear search | OK |
| ui-browse | Browse | OK (Windows' button word) |
| ui-browse-tooltip | Choose a folder for the field «X» | OK |
| ui-disclosure | X, Y, expanded / X, Y, collapsed | OK (Narrator's words) |
| ui-language | Language | OK |
| ui-machine-translated | This translation was made automatically and may contain errors. | OK |
| ui-report-translation-problem | Report a problem with the translation | OK |
| ui-needs-direct3d-12 | To display the window Direct3D 12 is needed, and an old graphics driver or a virtual machine may not support it. | OK |
| size-bytes | N byte / N bytes (one, few, many, fractional forms) | OK |
| size-kb | N KB (КБ) | OK |
| size-mb | N MB (МБ) | OK |
| size-gb | N GB (ГБ) | OK |
| size-tb | N TB (ТБ) | OK |
| list-separator | ", " | OK |
| installer-window-title | WinAppMigrator installer | OK |
| installer-window-title-app | Installation of X | OK |
| installer-tagline | Reinstalls an application packed on another computer | OK |
| installer-unnamed-app | Application | OK |
| installer-window-failed | Could not open the installer's window: E | OK |
| installer-window-failed-without-one | The app can be installed without the window too: run this file from a command prompt as administrator with --apply, and to choose the folder add --target-root and its path. | OK |
| installer-checking | Checking the installer… | OK |
| installer-stub-no-package | This is a WinAppMigrator installer stub, and it has no application package. Create an installer in WinAppMigrator, then run the resulting .exe file here. | OK |
| installer-check-stopped | The check of the installer was interrupted. | OK |
| installer-locate-failed | Could not find the installer: E | OK |
| installer-read-failed | Could not read the installer: E | OK |
| installer-cannot-be-used-here | This installer's package cannot be used on this computer: E | OK |
| installer-captured-from | Source | OK (terms.md) |
| installer-install-location | Install folder | OK |
| installer-choose-location | Choose the install folder | OK |
| installer-folder-has-files | This folder already has files. Files with the same names will be replaced, and come back if you uninstall the app. | OK |
| installer-room-needed | Space required | OK |
| installer-room-short | There may not be enough space here: S. The package can be unpacked on another drive, but the app's files still have to fit. | OK (read with `core-space-shortfall` filled in) |
| installer-interrupted | The previous installation of X was not finished. It will be undone before installing. | OK |
| installer-signature | Installer | OK |
| installer-signature-checking | Checking the signature… | OK |
| installer-signature-unsigned | Not signed: publisher unknown | OK |
| installer-signature-signed-by | Signed: P | OK |
| installer-signature-unnamed | Signed by a publisher without a name | OK |
| installer-signature-invalid | This installer's signature is invalid (R). Perhaps it was changed after signing. | OK |
| installer-more-left-out | N more item(s) left out (one/few/many forms) | OK |
| uninstaller-plan-coming-back | N file(s) replaced by this install will be put back, or placed beside a file that has changed since. | OK (all forms) |
| uninstaller-remove-user-data-detail | N item(s) of the app's settings, data and license. They stay unless this option is on. What they replaced is put back, or placed beside them if they have changed since. | OK (all forms) |
| installer-install | Install | OK |
| installer-preview | Preview of changes | OK (terms.md; long for a button, see Open questions) |
| installer-user-data-detail | On: … Off: all of it goes into the install folder, and nothing is set up to start from there. | Fixed: "на работу оттуда" (to work from there) became "на запуск оттуда" (to start from there) |
| installer-license-detail | Activation and license files, and registry values that hold licenses. Some may need reactivation on this computer. | Fixed: brought up to date (registry values, not keys) |
| installer-no-programdata | This computer's ProgramData folder was not found, so nothing was installed: it holds the record by which the install can be undone. | Fixed: brought up to date |
| app-column-name | Name | OK |
| app-column-publisher | Publisher | OK |
| app-column-version | Version | OK |
| app-column-scope | Scope | OK |
| app-column-arch | Arch. | OK (abbreviated, fits) |
| app-option-license | License and activation data | OK |
| app-option-user-data | User data | OK |
| app-option-whose-data | Whose data | OK |
| app-build-create | Create installer | OK |
| app-build-cancel | Cancel | OK |
| app-scan-look-again | Check again | OK (terms.md) |
| app-scan-stop | Stop | OK |
| app-network-connect | Connect | OK |
| app-result-copy-path | Copy path | OK |
| app-source | Scan source | OK |
| app-list-count | N app(s) (one/few/many forms) | OK |
| app-footprint-folders-summary | I of N folder(s), S | OK (genitive after "из" right in every form) |
| app-scan-failed | Could not check this app: E | Fixed: brought up to date (colon) |
| core-space-shortfall | C: — needs up to N of free space for the app's files, and N is available | OK |
| core-policy-fit-program | The program P is built for architecture A, and this computer's architecture is B, so Windows will not start it | OK |
| core-runtime-download-cut-short | Download of U cut short: received A of B bytes | OK (genitive after "из" right in every form) |
| core-warning-restore-earlier-undone | The previous install of X was not finished, so it was undone first. N change(s) put back. | OK |
| core-rollback-file-stays-beside | The file P stays, because something that stays uses it. The file it replaced is beside it under the name N. | OK |
| core-error-io | I/O error (P): E | OK (reads right when P is a phrase such as "установщик") |
| core-review-service-detail | Runs F as A, starts with Windows before everything else | Fixed: `[other]` now takes the start words, which carry their verb |
| core-review-driver-detail-starts | Loads F into Windows itself, never starts: startup is disabled | Fixed: same |
| core-review-firewall-detail | Allows incoming connections through ports 80, 443 for app.exe in private, public networks | Fixed: network words fit the sentence; 21, 31 … ports no longer get "порт" (see Other fixes) |
| core-review-com-support-detail | Settings of its type libraries (3), interfaces (12) and AppIDs (1), which go with its classes | Fixed: brought up to date |
| core-warning-build-recorded-folder-left-out | … was left out, because its path leaves the drive or goes through a link, so reading it is not safe. | Fixed: `[unreadable]` brought up to date |
| core-warning-scan-drive-paths-not-followed | Could not read the path / some paths recorded on this drive … | Fixed: see Other fixes |
| core-warning-build-not-looked-through | This package has a file / files that could not be checked for who the package came from … | Fixed: see Other fixes |
| core-warning-scan-package-mentions | In this package the account's name is mentioned in these places: L. … | Fixed: word order ("упоминается имя учетной записи, в следующих местах" had a stray comma) |
| core-restore-ensured-registry-key | Made sure the registry key K exists | Fixed: was "проверено наличие" (checked that it exists); "ensured" means made sure, creating it if needed |

80 messages sampled, 13 fixed.

## Other fixes

- Russian's `[one]` category also covers 21, 31, 101 …, so a `[one]` variant that does not show the number must not
  say "one". `core-warning-build-not-looked-through`, `core-warning-scan-drive-paths-not-followed`, and
  `core-warning-scan-network-paths-not-followed` said "a file" / "a path" for 21 of them, and
  `core-review-firewall-detail` said "через порт" (through the port) before a list of 21 ports. Each now has an exact
  `[1]` variant with the singular words, and `[one]` uses the plural words.
- Ran `ftl-tool.py merge` so the files carry the current English comments (several still described `{ $start }` and
  `{ $networks }` as raw English words, and `app-scan-failed`'s `{ $error }` as "a sentence"). No message text changed
  by that.
- `terms.md`: added the firewall profile words and how the start words are built.

## Open questions

- Firewall profiles: "доменных, частных, общедоступных" follow Windows Security's "Сеть домена", "Частная сеть",
  "Общедоступная сеть". The rule list of "Монитор брандмауэра Защитника Windows" may show the profiles as "Домен",
  "Частный", "Общий" instead; a native speaker with Russian Windows should confirm which reads more naturally here.
- Start words: the English is descriptive, so these are too. Windows' own start types ("Автоматически", "Автоматически
  (отложенный запуск)", "Вручную", "Отключена") are not used; a native speaker may prefer them.
- The Scope column is 76 px of text. "Для пользователя" and "Для всех пользователей" (`app-scope-per-user`,
  `app-scope-per-machine`) will be cut to "Для польз…" and "Для всех п…". They are still told apart, but the column
  could be made wider for Russian, or the values could be "Пользователь" and "Компьютер". Left as it is: a layout call.
- `installer-preview` "Предварительный просмотр изменений" (the terms.md word for Preview Changes) is long for a
  button beside "Установить" and "Отмена"; check it fits in the real window.
- `core-policy-architecture-unknown` and `core-policy-machine-type` ("неизвестного типа", "с кодом типа машины …") read
  well after "создан для архитектуры", less well after "архитектура этого компьютера —".

Checker: ru-RU: 0 errors, 0 warnings.
