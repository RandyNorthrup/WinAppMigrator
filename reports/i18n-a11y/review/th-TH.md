# Review: Thai (th-TH)

Reviewer: independent machine pass, 2026-11-15. Translator's form of address: the polite, neutral voice of Thai
Windows 11, with "คุณ" only where a "you" is needed, "โปรด" for requests, no ครับ/ค่ะ, and no full stop at the end of
a sentence. I kept it because that is how Thai Windows talks to the user.

## Brought up to date

I checked all 30 messages on the delta list against the current English. 10 needed new or changed words:

- `app-scan-failed`: the reason now follows a colon: “ไม่สามารถตรวจดูแอปนี้ได้: { $error }”.
- `core-warning-build-recorded-folder-left-out`, `core-warning-build-near-part-left-out` `[unreadable]`: this was
  "it cannot be read" (“ไม่สามารถอ่านได้”). It is now the safety rule: “เส้นทางออกไปนอกไดรฟ์ หรือผ่านลิงก์
  จึงไม่สามารถอ่านได้อย่างปลอดภัย” ("its path goes off the drive or through a link, so it cannot be read safely").
- `core-review-start-system`: “ขณะที่ Windows กำลังเริ่มต้น” ("while Windows is starting"; it was "when Windows starts").
- `core-review-start-delayed`: “เริ่มทำงานไม่นาน หลังจากพีซีเครื่องนี้เริ่มทำงานในแต่ละครั้ง”. The words now say
  "soon after each start", in the same order as `core-review-service-detail` `[delayed]`, which I matched to them.
  The other start words, `-boot`, `-automatic`, `-manual`, `-disabled`, and `-unknown`, already said what the
  English says. Each one carries its own verb (เริ่มทำงาน…), so they read as the end of
  “เรียกใช้ app.exe ในฐานะ LocalSystem { $start }” and “โหลด disk.sys เข้าไปใน Windows โดยตรง { $start }”.
- `core-review-com-support-detail`: was "settings 3, 5 and 1 items respectively". Each count now follows its own kind,
  in the words of `core-review-com-support-title`: “การตั้งค่าสำหรับไลบรารีชนิด ({ $libraries }) อินเทอร์เฟซ
  ({ $interfaces }) และ AppID ({ $app_ids }) ซึ่งไปพร้อมกับคลาสของแอป”.
- `core-warning-scan-license-data`: "the License option" (ตัวเลือกสิทธิ์การใช้งาน) is now
  `ตัวเลือก “ข้อมูลสิทธิ์การใช้งานและการเปิดใช้งาน”`, the words of `app-option-license`.
- `installer-license-detail`: was "files and keys of activation and license". It is now “ไฟล์การเปิดใช้งานและสิทธิ์การใช้งาน
  รวมถึงค่ารีจิสทรีที่เก็บสิทธิ์การใช้งาน” ("…and the registry values that hold licenses").
- `installer-no-programdata`, `installer-cli-no-programdata`: was "so the install cannot be undone". It is now
  “จึงไม่ได้ติดตั้งสิ่งใด เนื่องจากการติดตั้งจะเก็บบันทึก ที่ใช้เลิกทำการติดตั้งไว้ที่นั่น” ("so nothing was
  installed, because an install keeps the record it is undone by there").

20 already said what the current English says:

- The license option: `app-footprint-licenses-still-go`, `core-footprint-licenses-left-out`,
  `core-review-note-licenses`, and `core-secret-why-license-option-off` already used the full option name.
- `client-builder-done`, `client-wrong-code`, `client-pair-failed`: “พีซีเครื่องอื่น” ("another PC"). Thai has no
  capitals, and the words already start the sentence.
- The start words: `core-review-start-boot`, `-automatic` (I added a space between phrases), `-manual`, `-disabled`,
  and `-unknown`.
- `core-review-network-domain`, `-private`, `-public`: “โดเมน”, “ส่วนตัว”, “สาธารณะ”, the profile names of Windows
  Defender Firewall. They fill “…สำหรับ app.exe บนเครือข่าย{ $networks }”, which gives “บนเครือข่ายส่วนตัว”
  ("on private networks"), Windows' own wording, or “บนเครือข่ายโดเมน, ส่วนตัว”.
- `core-rollback-setting-put-back`, `core-rollback-list-entries-removed`, `core-rollback-list-entries-not-removed`,
  `core-rollback-firewall-removed`: Thai never had the stray capital "The".
- `ui-disclosure`: “ขยายอยู่” / “ยุบอยู่” ("expanded" / "collapsed").

I then ran the files through `ftl-tool.py merge`. It changed only `#` comments, bringing them up to the current
English ones. I confirmed on a copy first that no message text moved or changed.

## Sample (back-translation)

| Message | Back-translation | Verdict |
| --- | --- | --- |
| ui-clear-search | Clear search | OK |
| ui-browse | Browse | OK |
| ui-browse-tooltip | Choose a folder for { $field } | OK |
| ui-disclosure | { $title }, { $summary }, expanded / collapsed | OK |
| ui-language | Language | OK |
| ui-machine-translated | This translation was made by machine and may contain errors | OK |
| ui-report-translation-problem | Report a translation problem | OK |
| ui-needs-direct3d-12 | This program needs Direct3D 12 to show its window, which an old graphics driver or a virtual machine may not have | OK; I added a space between phrases |
| size-bytes | { $value } bytes | OK (Thai has only `other`) |
| size-kb | { $value } KB | OK |
| size-mb | { $value } MB | OK |
| size-gb | { $value } GB | OK |
| size-tb | { $value } TB | OK |
| list-separator | ", " | OK |
| installer-window-title | WinAppMigrator installer | OK |
| installer-window-title-app | Install { $app_name } | OK |
| installer-tagline | Install again an application collected from another PC | OK |
| installer-unnamed-app | Application | OK |
| installer-window-failed | Could not open the installer's window: { $error } | OK |
| installer-window-failed-without-one | But it can still install without a window: run this file from a command prompt run as administrator with --apply, and --target-root followed by a folder to choose where | OK; I added a space between phrases |
| installer-checking | Checking the installer… | OK |
| installer-stub-no-package | This is a WinAppMigrator installer stub, with no application package inside. Please build an installer with WinAppMigrator, then run the .exe you get here | OK |
| installer-check-stopped | Checking the installer stopped | OK |
| installer-locate-failed | Could not find where the installer is: { $error } | OK |
| installer-read-failed | Could not read the installer: { $error } | OK |
| installer-cannot-be-used-here | This installer's package cannot be used on this PC: { $error } | OK; I added a space between phrases |
| installer-captured-from | Collected from | OK |
| installer-install-location | Install location | OK |
| installer-choose-location | Choose the install location | OK |
| installer-folder-has-files | This folder already has files. Files with the same name are replaced, and come back if you uninstall | OK |
| installer-room-needed | Space needed | OK |
| installer-room-short | There may not be enough space here: { $shortfall }. The installer can unpack on another drive, but the app's files still need enough space | OK |
| installer-interrupted | An earlier install of { $app_name } did not finish. On install, the system first undoes that install | OK |
| installer-signature | Installer | OK |
| installer-signature-checking | Checking the signature… | OK |
| installer-signature-unsigned | Not signed: publisher unknown | OK |
| installer-signature-signed-by | Signed by { $publisher } | OK |
| installer-signature-unnamed | Signed by a publisher with no name | OK |
| installer-signature-invalid | This installer's signature is not valid ({ $reason }). The installer may have been changed after signing | OK |
| installer-more-left-out | { $count } more items not included | OK |
| uninstaller-plan-coming-back | { $count } files this install replaced come back, or are put beside the original file if that file has changed since | Fixed: “ไฟล์เดิม” ("the original file") read as the replaced file itself. It is now “ไฟล์ปัจจุบัน” ("the current file"), and "since" is now “หลังการติดตั้ง” ("since the install") |
| uninstaller-remove-user-data-detail | { $count } items of the app's settings, data, and license, which stay unless this option is on. What they replaced comes back, or is put beside them if they have changed since | OK; I added a space between phrases |
| app-tagline | Turn an installed app into a self-extracting installer | OK; I added a space between phrases |
| app-reload-list | Reload the list of applications | OK |
| app-list-search | Search by name or publisher  (Ctrl+F) | OK; Thai Windows writes Ctrl |
| app-list-filter-all | All | OK |
| app-scope-per-user | Per user | OK |
| app-scope-per-machine | Per machine | OK |
| app-column-name | Name | OK |
| app-column-publisher | Publisher | OK |
| app-column-version | Version | OK |
| app-column-scope | Scope | OK |
| app-column-arch | Architecture | Fixed: “สถาปัตยกรรม” is 71 px wide at 12 px semibold, and the column has 46 px. It is now “ซีพียู” ("CPU", 25 px), which leaves room for the sort arrow |
| app-column-arch-sort | Sort by architecture | Fixed: “เรียงลำดับตามสถาปัตยกรรมซีพียู” ("sort by CPU architecture"), so the spoken name contains the visible header |
| app-source | Scan source | OK |
| app-source-network | PC on the network… | OK (terms.md: Network PC) |
| app-option-license | License and activation data | OK |
| app-option-user-data | User data | OK |
| app-option-whose-data | Whose data | OK |
| app-output-folder | Output folder | OK |
| app-build-create | Create installer | OK |
| app-build-cancel | Cancel | OK |
| app-result-open-folder | Open folder | OK |
| app-result-copy-path | Copy path | OK |
| app-result-log | Log file | OK |
| app-scan-failed | Could not look at this app: { $error } | Fixed (delta): colon added |
| core-rollback-drive-not-connected | What this install put in { $folder } is on a drive that is not connected. Please connect that drive so this can be finished | OK; I added a space between phrases |
| core-rollback-file-folder-gone | { $path } will not be put back, because the folder the file was in has gone | OK |
| core-rollback-key-not-removed | Could not remove the key { $name }: { $error } | OK |
| core-rollback-task-changed | The task { $name } stays, because it changed after it was installed | OK |
| core-system-registry-through-link | { $drive } is not a Windows system drive: the path to its registry goes through a link | OK |
| core-scan-program-someone-elses | { $path } is in another account's folder or is not this account's, and this app's own record does not say the program should be there, so it was not read | OK; I added a space between phrases |
| core-package-program-gone | { $path } is no longer there, so nothing was built. Please look at the app again | OK |
| core-discovery-link-to-somewhere-else | { $path } is a link pointing elsewhere, and others may change where it leads. Please use the place it points to instead | OK |
| core-error-pe | PE parsing failed for { $path }: { $detail } | OK |
| core-journal-unreadable-empty | The install record in { $folder } cannot be used: the record is empty | OK |
| core-restore-would-copy | Would copy { $source } -> { $destination } | OK |
| core-space-summary | Up to { $needed } on { $drive } ({ $free } free) | OK |
| core-policy-names-and-more | { $names } and { $count } more items | OK |
| core-review-shortcut-opens-other | Opens { $path }, which this package did not install | OK |
| core-privilege-not-made-here | { $path } was not made by WinAppMigrator, and others can change what is inside. An administrator should check and remove this folder: { $error } | OK |
| core-review-service-detail | Runs { $file } as { $account }, starts now and each time this PC starts / …and soon after each time / …only when something asks / is disabled / { $start } | Fixed (delta): `[delayed]` matched to the start words; added spaces between phrases |
| core-review-driver-detail-starts | Loads { $file } directly into Windows, { $start } | OK with the start words filled in |
| core-review-firewall-detail | Allows incoming connections on port 80 for app.exe on private networks | OK with the network words filled in |
| core-review-com-support-detail | Settings for its type libraries (3), interfaces (5), and AppIDs (1), which go with the app's classes | Fixed (delta): each count follows its kind |

84 messages sampled. 6 needed fixes (`uninstaller-plan-coming-back`, `app-column-arch`, `app-column-arch-sort`, and the
3 delta messages `app-scan-failed`, `core-review-service-detail`, and `core-review-com-support-detail`). The rest only
gained spaces between phrases.

Across the whole language: the other column headers fit their columns (Scope “ขอบเขต” 43 px of 76, Version
“เวอร์ชัน” 40 px of 72). Button labels are short. Nothing is left in English except product names, folder and file
names, command-line flags, key names, and codes (COM, ODBC, DISM, RVA, UTF-8), as the glossary says.

## Other fixes

- **Spaces between phrases, so lines can wrap.** Thai writes words without spaces, and the windows break lines only at
  spaces. I measured every unbroken run at 14 px Leelawadee UI, the font the app uses for Thai. 187 runs were wider
  than 250 px, and many were 300–536 px, against a details panel that can be as narrow as about 340 px of text. I
  added a space at a natural pause in 176 of them (about 155 messages): before และ, หรือ, เนื่องจาก, จนกว่า, a relative
  ที่, or the verb after a long subject. The words did not change. A run that appears in several messages got the same
  break in each. 15 runs of 250–290 px stay whole, because they have no natural pause, and they still fit the narrowest
  panel. I left `client.ftl` and the `app-client-note` text file's hand-made line breaks alone: the console and Notepad
  break lines wherever they need to.
- `terms.md`: added the firewall profile names, and a note on spaces between phrases for anyone editing later.

## Open questions

- **The spaces between phrases** were placed by a non-native machine pass. A native speaker should check that each
  added space falls where a Thai writer would pause. Most are before และ / หรือ / ที่ / เนื่องจาก, and a few are before
  ว่า, for example “ไม่สามารถทราบได้ ว่า…”.
- **The Arch column**: “ซีพียู” ("CPU") was chosen because “สถาปัตยกรรม”, which is what Task Manager uses, does not
  fit 46 px. A native speaker may prefer another short word.
- **Firewall profiles**: I could not confirm “โดเมน”, “ส่วนตัว”, “สาธารณะ” against Thai Windows Defender Firewall. A list
  reads “บนเครือข่ายโดเมน, ส่วนตัว”, which should be understood but is a little loose.
- “เก็บมาจาก” for "captured from" (`installer-captured-from`, `installer-tagline`): it is plain and clear, but a native
  speaker may prefer “บันทึกมาจาก”.
- `ui-disclosure`: check “ขยายอยู่” / “ยุบอยู่” against what Thai Narrator says for expanded and collapsed.

Checker: th-TH: 0 errors, 0 warnings.
