# Review: Arabic (ar-SA)

Reviewer: independent machine pass, 2026-11-15. Translator's form of address: the second person singular, with
imperatives for actions ("اختر", "أدخل", "انقر فوق") and passive or impersonal forms ("تم", "تعذر", "جارٍ") for what
happened; kept, because that is how Windows addresses the user in Arabic ("هل تريد السماح لهذا التطبيق بإجراء تغييرات
على جهازك؟"). No direction marks anywhere in the text (checked: no U+200E, U+200F, U+061C, embeddings or isolates); the
program lays out right-to-left text and isolates placeholders itself.

## Brought up to date

All 30 messages in the English delta were compared with the current English. 12 needed new words, and 2 sentences that
the new words fill needed reshaping:

- `app-scan-failed`: the reason now follows a colon, not a full stop ("تعذر فحص هذا التطبيق: { $error }").
- `app-footprint-licenses-still-go`, `core-footprint-licenses-left-out`, `core-review-note-licenses`,
  `core-secret-why-license-option-off`, `core-warning-scan-license-data`: خيار "الترخيص" is now
  خيار "بيانات الترخيص والتنشيط", the words of the checkbox `app-option-license`.
- `core-warning-build-recorded-folder-left-out`, `core-warning-build-near-part-left-out` `[unreadable]`: was "it cannot
  be read" (لا يمكن قراءته); now the safety rule: "مساره يؤدي إلى خارج محرك الأقراص أو يمر عبر ارتباط، لذا لا يمكن
  قراءته بأمان" (its path leads outside the drive or goes through a link, so it cannot be read safely).
- `installer-license-detail`: "ملفات ومفاتيح التنشيط والترخيص" (activation and license files and keys, which read as
  product keys) is now "ملفات التنشيط والترخيص، وقيم السجل التي تحتفظ بالتراخيص" (and the registry values that hold
  licenses).
- `installer-no-programdata`, `installer-cli-no-programdata`: was "so the install could not be undone"; now "لذا لم يتم
  تثبيت أي شيء: يحتفظ التثبيت هناك بالسجل الذي يُستخدم للتراجع عنه" (so nothing was installed: an install keeps there
  the record it is undone by).
- `core-review-start-disabled`: was "أبدًا، لأنه معطل" ("never, as it is disabled"), which after the verb of the sentence
  read "ويبدأ أبدًا", "it starts never": Arabic says "never" only with a negated verb. So that every `core-review-start-*`
  phrase reads correctly where it goes, `core-review-driver-detail-starts` and `core-review-service-detail` `[other]` now
  end "ويكون بدء تشغيله/ها { $start }" ("and its start is …") instead of "ويبدأ/وتبدأ { $start }", and
  `core-review-start-disabled` is "معطّلًا، فلا يحدث أبدًا" ("disabled, so it never happens"). The other six start
  phrases read correctly in the new frame unchanged ("ويكون بدء تشغيله مع Windows، قبل أي شيء آخر"). The phrases now
  agree with "بدء" (start), not with the service (feminine) or driver (masculine), so one set fits both sentences.

18 already said what the current English says:

- `client-builder-done`, `client-wrong-code`, `client-pair-failed`: the Arabic sentences open with the verb ("انتهى
  كمبيوتر آخر من عمله", "جرّب كمبيوتر آخر رمزًا…", "تعذر على كمبيوتر آخر الاقتران"), and Arabic has no capitals, so
  "Another PC" at the start of the English needs nothing.
- `core-review-start-boot`, `-system`, `-automatic`, `-delayed`, `-manual`, `-unknown` (already translated as phrases).
- `core-review-network-domain`, `-private`, `-public`: "شبكة المجال", "الشبكة الخاصة", "الشبكة العامة", as in
  `terms.md`. They fill "على { $networks }" in `core-review-firewall-detail`, which reads correctly with them ("… للبرنامج
  app.exe، على شبكة المجال، الشبكة الخاصة"). See open questions.
- `core-review-com-support-detail`: already names each count's kind ("إعدادات ترافق فئاته: 3 لمكتبات الأنواع، و5
  للواجهات، و1 لمعرّفات AppID").
- `core-rollback-setting-put-back`, `core-rollback-list-entries-removed`, `core-rollback-list-entries-not-removed`,
  `core-rollback-firewall-removed`: Arabic never had the capital "The".
- `ui-disclosure`: "موسّع" / "مطوي" (expanded / collapsed), as Narrator says them.

The files were then run through `ftl-tool.py merge`, which brought the `#` comments in 4 files up to the current English;
no message text moved or changed (checked by comparing every message before and after).

## Sample (back-translation)

88 messages: all of `ui.ftl` (8) and `winappmigrator.ftl` (6), the first 25 of `installer.ftl`, its 3 plural messages,
27 from `app.ftl` (buttons, all column headers, option names), and 19 from the `core*.ftl` files with placeholders.

| Message | Back-translation | Verdict |
| --- | --- | --- |
| ui-clear-search | Clear search | OK |
| ui-browse | Browse | OK (Windows' "استعراض") |
| ui-browse-tooltip | Choosing a folder for the "{field}" field | OK |
| ui-disclosure | {title}, {summary}, expanded / collapsed | OK |
| ui-language | Language | OK |
| ui-machine-translated | This translation was done by machine and may contain mistakes. | OK |
| ui-report-translation-problem | Reporting a problem in the translation | OK |
| ui-needs-direct3d-12 | This needs Direct3D 12 to draw its window, and that may not be available with an old graphics driver or on a virtual machine. | OK |
| size-bytes | {value} bytes (every plural form) | OK: "بايت" does not change after a number written in digits |
| size-kb | {value} kilobytes | OK (see open questions) |
| size-mb | {value} megabytes | OK |
| size-gb | {value} gigabytes | OK |
| size-tb | {value} terabytes | OK |
| list-separator | "، " (Arabic comma) | OK |
| installer-window-title | WinAppMigrator installer | OK |
| installer-window-title-app | Install {app} | OK |
| installer-tagline | Reinstalls an app that was captured on another PC | OK |
| installer-unnamed-app | App | OK |
| installer-window-failed | Opening the installer window failed: {error} | OK |
| installer-window-failed-without-one | It can still install without it: run this file from a command prompt running as administrator with --apply, and with --target-root followed by a folder to choose where to install. | OK |
| installer-checking | Checking the installer… | OK |
| installer-stub-no-package | This is a WinAppMigrator installer stub, and it carries no app package. Create an installer with WinAppMigrator, then run the resulting .exe file here. | OK |
| installer-check-stopped | The check of the installer stopped. | OK |
| installer-locate-failed | Locating the installer failed: {error} | OK |
| installer-read-failed | Reading the installer failed: {error} | OK |
| installer-cannot-be-used-here | This installer's package cannot be used on this PC: {error} | OK |
| installer-captured-from | Captured from | OK |
| installer-install-location | Install location | OK |
| installer-choose-location | Choosing the install location | OK (title of the folder picker) |
| installer-folder-has-files | This folder already contains files. Files with the same names are replaced, and come back if you uninstall. | OK |
| installer-room-needed | Space needed | OK |
| installer-room-short | There may not be enough space here: {shortfall}. The package can be unpacked on another drive, but the app's files must still fit here. | OK |
| installer-interrupted | An earlier install of {app} never finished. It is undone first when you click "Install". | OK (names the button, as the English means) |
| installer-signature | Installer | OK |
| installer-signature-checking | Checking its signature… | OK |
| installer-signature-unsigned | Not signed: publisher unknown | OK |
| installer-signature-signed-by | Signed by {publisher} | OK |
| installer-signature-unnamed | Signed by an unnamed publisher | OK |
| installer-signature-invalid | This installer's signature is not valid ({reason}). It may have been changed after it was signed. | OK |
| installer-more-left-out | {n} more item(s) were left out (zero/one/two/few/many/other) | OK: dual "عنصرين آخرين", 3-10 "عناصر أخرى", 11-99 "عنصرًا آخر" all correct |
| uninstaller-plan-coming-back | {n} file(s) this install replaced are put back, or each is placed beside the current file if it has changed since. | OK, all six forms |
| uninstaller-remove-user-data-detail | {n} item(s) of its settings, data and license. They stay unless this option is on. What they replaced comes back, or is placed beside them if they have changed since. | OK, verbs and pronouns agree in each form (dual "يبقيان … استبدلاه … بجانبهما") |
| app-tagline | Turning an installed app into a self-extracting installer | OK |
| app-list-count | {n} apps (one: "one app", two: "two apps") | OK |
| app-list-search | Search by name or publisher (Ctrl+F) | OK (Arabic Windows writes Ctrl+F) |
| app-scope-per-user | Per user | OK |
| app-column-name | Name | OK, short |
| app-column-publisher | Publisher | OK, short |
| app-column-version | Version | OK, short |
| app-column-scope | Scope | OK, short |
| app-column-arch | Architecture | OK: "البنية" is 6 letters, as short as Arabic allows |
| app-column-arch-sort | Sort by architecture | OK |
| app-source | Scan source | OK |
| app-source-network | PC on the network… | OK |
| app-option-license | License and activation data | OK |
| app-option-user-data | User data | OK |
| app-option-whose-data | Data of which account | OK |
| app-output-folder | Output folder | OK |
| app-build-create | Create the installer | OK |
| app-build-cancel | Cancel | OK (Windows' "إلغاء الأمر") |
| app-scan-look-again | Scan again | OK, as in terms.md |
| app-scan-stop | Stop | OK |
| app-scan-try-again | Try again | OK |
| app-scan-failed | Scanning this app failed. {error} | Fixed: the reason follows a colon now, as in English |
| app-result-open-folder | Open folder | OK |
| app-result-copy-path | Copy path | OK |
| app-network-connect | Connect | OK |
| app-room-drive-short | {drive} needs free space of up to {needed} to create this, and has {free} | OK |
| app-footprint-licenses-still-go | License data in what you left out is still included, because it goes by the "License" option: {list}. | Fixed: option named as its checkbox, "بيانات الترخيص والتنشيط" |
| core-rollback-prerequisite-stays | {name} was installed, or may have been, and will stay, because other programs may use it. | OK |
| core-license-detail-file | A license file was detected: {name} | OK |
| core-runtime-feature-failed | DISM could not enable the Windows feature {feature} (exit code {code}); the payload most likely could not be downloaded. Enable it from "Windows Features", or run `dism …`. | OK, command kept |
| core-profile-desktop-not-a-person | The desktop here belongs to {account}, which is not a person's account | OK |
| core-scan-no-executable | Could not find an executable for {app}. A driver, runtime or add-in may have no program of its own to package. | OK, reads after app-scan-failed's colon |
| core-discovery-refusing-link | Going through a link was refused: {path} | OK |
| core-error-io | Input/output failed for {path}: {error} | OK, also with a phrase such as "برنامج التثبيت" in {path} |
| core-error-pe | PE parsing failed for {path}: {detail} | OK |
| core-restore-location-not-full-path | The install location must be a full path: "{path}" | OK |
| core-restore-would-create-empty-folder | The empty folder {path} will be created | OK |
| core-restore-would-restore-variable | The user's / the machine's environment variable {name} will be restored | OK |
| core-warning-restore-environment-key-not-created | Creating the environment registry key failed: {error} | OK |
| core-review-account-not-told | Could not tell who is signed in ({reason}), so per-user settings were left out, and user data goes to the install location. | OK |
| core-verify-not-unpacked | This installer's package was not unpacked to {path} | OK |
| core-verify-task-unreadable | Querying the scheduled task failed: {error} | OK |
| core-policy-fit-program | {program} is made for the {built_for} architecture, and this PC's architecture is {here}, so Windows will not start it | OK, also with "غير معروفة" and "ذات رمز المعالج {code}" filled in |
| core-review-firewall-detail | Allows incoming connections on ports {ports} for the program {program}, on {networks} / on every network | OK with the new network words |
| core-review-service-detail | Runs {file} as {account}, and starts {start} | Fixed: `[other]` is now "ويكون بدء تشغيلها { $start }", so it reads with every start phrase |
| core-review-driver-detail-starts | Loads {file} into Windows itself, and starts never, as it is disabled | Fixed: was ungrammatical with `core-review-start-disabled`; now "ويكون بدء تشغيله { $start }" with "معطّلًا، فلا يحدث أبدًا" |

Across the whole language: the `app-column-*` headers are one word each ("الاسم", "الناشر", "الإصدار", "النطاق",
"البنية"); button labels are one to three words; nothing is left in English except what stays English everywhere
(product names, command lines and flags, file and folder names, registry names, `_external`, `OLD-PC`, and the example
path `C:\Users\You\WinAppMigrator`). The `installer-cli-*` and `uninstaller-cli-*` messages are translated, though the
program prints its command lines in English.

## Other fixes

- `terms.md`: added a row for the app's option, "License and Activation Data" = "بيانات الترخيص والتنشيط", so every
  message that names the option uses the checkbox's words.
- `#` comments brought up to the current English by `ftl-tool.py merge` (no message text changed).

## Open questions

- Firewall profiles: "شبكة المجال", "الشبكة الخاصة", "الشبكة العامة" could not be checked against the Profile column of
  Arabic Windows Defender Firewall, which may show the bare "المجال" / "خاص" / "عام", while Windows Security says "شبكة
  المجال" / "شبكة خاصة" / "شبكة عامة". A native speaker with Arabic Windows should confirm. The list is joined with "، "
  only, with no "و" before the last item, because the program joins lists with `list-separator` alone.
- Service and driver start: the new frame "ويكون بدء تشغيله { $start }" reads correctly with all seven phrases, but with
  `core-review-start-system` it says "بدء تشغيله" twice ("ويكون بدء تشغيله مع Windows، أثناء بدء تشغيله", first the
  driver's start, then Windows'). Clear, but a native speaker may prefer other words.
- "السجل" is the registry (as `terms.md` has it), but the same word is in "سجل التثبيت" (the install's record), "ملف
  السجل" (log file), and "سجل الأحداث" (event log); each use is qualified. Arabic Windows titles Registry Editor "محرر
  التسجيل", so confirm that "السجل" is what Windows calls the registry in its messages.
- Sizes are spelled out ("كيلوبايت", "ميغابايت", "غيغابايت"), as Arabic Windows Explorer writes them; confirm against
  Windows 11.
- `app-output-folder-example` keeps `C:\Users\You\WinAppMigrator` in English: an Arabic word for "You" inside a
  left-to-right path would break up in a right-to-left field. A native speaker may prefer a name.

Checker: ar-SA: 0 errors, 0 warnings.
