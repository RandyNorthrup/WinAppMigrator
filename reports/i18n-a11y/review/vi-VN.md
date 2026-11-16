# Review: Vietnamese (vi-VN)

Reviewer: independent machine pass, 2026-11-15. Translator's form of address: **bạn**, with "hãy …" for instructions.
Kept, because that is how Windows 10 and 11 in Vietnamese address the user (UAC: "Bạn có muốn cho phép ứng dụng này thực
hiện thay đổi đối với thiết bị của bạn không?"). The translator was consistent throughout: no "quý vị" or bare commands.

## Brought up to date
30 messages from the English delta, plus 2 sentences that take the new start words:

- **Rewritten to match the new English (12):** `app-scan-failed` (now a colon before `{ $error }`, and the core
  errors it carries already start in lower case); `app-footprint-licenses-still-go`, `core-footprint-licenses-left-out`,
  `core-review-note-licenses`, `core-secret-why-license-option-off`, `core-warning-scan-license-data` ("tùy chọn Giấy
  phép" became "tùy chọn Dữ liệu giấy phép và kích hoạt", the words of `app-option-license`);
  `core-warning-build-recorded-folder-left-out`, `core-warning-build-near-part-left-out` (`[unreadable]` said "may not
  be readable", a read error; it now says the path leads off the drive or goes through a link, so it cannot be read
  safely); `core-review-com-support-detail` (each count now beside its kind: "thư viện kiểu (…), giao diện (…) và AppID
  (…)"); `installer-license-detail` (it said "activation and license files and keys"; now "files … and the registry
  values that hold licenses"); `installer-no-programdata`, `installer-cli-no-programdata` (they said "so the install
  cannot be undone"; now "so nothing was installed: every install keeps there the record that undoes it").
- **Added (10):** `core-review-start-boot`, `-system`, `-automatic`, `-delayed`, `-manual`, `-disabled`, `-unknown`;
  `core-review-network-domain`, `-private`, `-public`.
- **Changed so the new words fit (2, not in the delta):** `core-review-service-detail` `[other]` and
  `core-review-driver-detail-starts` said "kiểu khởi động { $start }" (startup type: raw value). Vietnamese cannot put
  "never" after the verb ("khởi động không bao giờ" is wrong), so each start word now carries its own verb and the
  sentences end in `, { $start }`: "…, khởi động cùng Windows, trước mọi thứ khác", "…, chỉ khởi động khi có thứ gì đó
  yêu cầu", "…, không bao giờ khởi động, vì đã bị tắt". `core-review-firewall-detail` ("trên các mạng { $networks }")
  already reads correctly with "miền", "riêng tư", "công cộng" and was left as it was.
- **Already right, checked and left alone (8):** `client-builder-done`, `client-wrong-code`, `client-pair-failed`
  (already start with "Một PC khác"); `core-rollback-setting-put-back`, `core-rollback-list-entries-removed`,
  `core-rollback-list-entries-not-removed`, `core-rollback-firewall-removed` (the English only fixed a capital "The");
  `ui-disclosure` (already present: "đang mở" / "đang đóng").

The files were then run through `ftl-tool.py merge`, which only refreshed the `#` comments to English's current ones
(for example the comment on `app-scan-failed` and on the two start sentences); no translation changed by it.

## Sample (back-translation)
72 messages: all of `ui.ftl` and `winappmigrator.ftl`, the first 25 of `installer.ftl`, 17 from `app.ftl`, every
plural in `installer.ftl`, and 13 from `core*.ftl` with placeholders.

| Message | Back-translation | Verdict |
| --- | --- | --- |
| ui-clear-search | Clear search | OK |
| ui-browse | Browse | OK (Windows' own "Duyệt") |
| ui-browse-tooltip | Choose a folder for { $field } | OK |
| ui-disclosure | { $title }, { $summary }, open (now) / closed (now) | OK (see Open questions) |
| ui-language | Language | OK |
| ui-machine-translated | This translation was made by machine and may have errors. | OK |
| ui-report-translation-problem | Report a translation error | OK |
| ui-needs-direct3d-12 | This program needs Direct3D 12 to draw the window, but an old graphics driver or a virtual machine may not support it. | OK |
| size-bytes | { $value } byte (one form: Vietnamese has no plural) | OK |
| size-kb, size-mb, size-gb, size-tb | { $value } KB / MB / GB / TB | OK (4 messages) |
| list-separator | ", " | OK |
| installer-window-title | WinAppMigrator installer | OK |
| installer-window-title-app | Install { $app_name } | OK |
| installer-tagline | Reinstalls an app that was captured on another PC | OK |
| installer-unnamed-app | Application | OK |
| installer-window-failed | Cannot open the installer's window: { $error } | OK |
| installer-window-failed-without-one | The installer can still install without a window: run this file from a command prompt with administrator rights with --apply, and with --target-root followed by a folder to choose where. | OK |
| installer-checking | Checking the installer… | OK |
| installer-stub-no-package | This is the launcher part (stub) of a WinAppMigrator installer and holds no app package. Create an installer with WinAppMigrator, then run the .exe it makes here. | OK |
| installer-check-stopped | The check of the installer stopped. | OK |
| installer-locate-failed | Cannot determine where the installer is: { $error } | OK |
| installer-read-failed | Cannot read the installer: { $error } | OK |
| installer-cannot-be-used-here | This installer's package cannot be used on this PC: { $error } | OK |
| installer-captured-from | Captured from | OK |
| installer-install-location | Install location | OK |
| installer-choose-location | Choose the install location | OK |
| installer-folder-has-files | This folder already has files. Files with the same names will be replaced, and will be put back if you uninstall. | OK |
| installer-room-needed | Space needed | OK |
| installer-room-short | There may not be enough space here: { $shortfall }. The package can be unpacked on another drive, but there must still be room for the app's files. | OK (read with a filled `core-space-shortfall`) |
| installer-interrupted | An earlier install of { $app_name } did not finish. Install will undo it first. | OK |
| installer-signature | Installer | OK |
| installer-signature-checking | Checking the signature… | OK |
| installer-signature-unsigned | Not signed: publisher unknown | OK ("Nhà phát hành không xác định", Windows' words) |
| installer-signature-signed-by | Signed by { $publisher } | OK |
| installer-signature-unnamed | Signed by a publisher with no name | OK |
| installer-signature-invalid | This installer's signature is not valid ({ $reason }). It may have been changed after it was signed. | OK |
| installer-more-left-out | { $count } more items left out | OK (plural, `*[other]` only, as Vietnamese) |
| uninstaller-plan-coming-back | { $count } files this install replaced will be put back, or placed beside a file that has changed since. | OK (plural) |
| uninstaller-remove-user-data-detail | { $count } items of the app's settings, data and license. They stay unless you turn this on. What they replaced is put back, or placed beside them if they changed since. | OK (plural) |
| app-build-create | Create installer | OK |
| app-build-create-again | Create the installer again | OK |
| app-build-cancel | Cancel | OK |
| app-scan-look-again | Look again | OK |
| app-result-copy-error | Copy the error message | OK |
| app-source | Scan source | OK |
| app-source-network | PC over the network… | OK |
| app-column-name | Name | OK |
| app-column-publisher | Publisher | OK |
| app-column-version | Version | OK (54 px in a 72 px cell) |
| app-column-scope | Scope | OK |
| app-column-arch | Architecture | OK, kept (see Open questions: 48.6 px in a 46 px cell) |
| app-option-license | License and activation data | OK |
| app-option-user-data | User data | OK |
| app-option-whose-data | Whose data | OK |
| app-scope-per-user | By user | Fixed: "Theo người dùng" is 99 px in the 76 px Scope cell, so every per-user row showed "Theo người…"; now "Người dùng" (70 px), paired with "Toàn máy" |
| app-network-client-saved | Saved as { $path }. On the old PC, close the app you are moving, then double-click this file and let the file make changes. The file will show a code. | Fixed: "the file shows a code" is unnatural; now "cho phép chương trình thực hiện thay đổi. Chương trình sẽ hiển thị một mã" (the program) |
| core-rollback-earlier-install-stuck | { $app_name }: { $failures } If this install can never be undone, an administrator can give up on it by deleting { $folder }. | OK |
| core-rollback-service-not-stopped | Cannot stop the service { $name } ({ $reason }), so what it uses may remain until the PC restarts. | OK |
| core-warning-scan-network-paths-not-followed | Paths recorded on this PC could not be read … Each path is not on a disk of this PC, where checking the path would connect there, or has a name that opens something else: { $paths } | Fixed: "nơi" (where) attached to "a disk of this PC", saying the opposite; now "Mỗi đường dẫn hoặc không nằm trên đĩa của PC này (khi đó việc kiểm tra đường dẫn sẽ kết nối tới nơi đó), hoặc …" |
| core-warning-build-user-folder-not-on-drive | { $user }'s { $folder } folder was at { $path }, not on this drive, so what is there was not included. | OK |
| core-package-not-enough-room | Not enough space to create this installer, so nothing was read or written. { $shortfalls }. Free up space there, or choose an output folder on another drive. | OK |
| core-space-shortfall | { $drive } needs up to { $needed } free space for { $purposes }, but has only { $free } | OK |
| core-image-differencing | { $path } is a differencing disk, holding only what changed since its parent disk, and Windows would also open that parent disk, wherever it says it is. First merge it into the parent disk with Hyper-V's Edit Disk, then pick the parent disk | OK (see Open questions) |
| core-restore-root-under-link | This app has things to restore into { $root }, and { $link } is a link to somewhere else that others may be able to change, so nothing was installed. … | OK |
| core-warning-restore-service-account | The service { $name } ran as { $account } on the source machine. Its password is not in the installer, so the service is set up to run as LocalService. … | OK |
| core-policy-fit-older-build | This app was on Windows build { $from_build }, while this PC runs build { $this_build }. … | OK |
| core-warning-review-documents-elsewhere | { $count } files from Documents go into { $target } instead, in a folder named _external, because this PC's Documents folder is at { $folder } … | OK |
| core-review-firewall-detail | Allows incoming connections on port { $ports } for { $program }, on the { $networks } networks | OK (filled with the new words: "trên các mạng riêng tư, công cộng") |
| core-review-service-detail | Runs { $file } as { $account }, starts now and right after each time this PC starts (`[delayed]`) | Fixed: "ngay sau" means "immediately after", the opposite of a delayed start; now "một lúc sau" (a while after). `[other]` now ends in `, { $start }` (see above) |

## Other fixes
- `app-scope-unknown`, `app-arch-unknown`: "Không xác định" (89 px) was cut to "Không xác…" in the 76 px Scope cell; now
  "Không rõ" (53 px), which fits there. In the 46 px Arch cell it is still cut, as English "Unknown" (55 px) is.
- `core-warning-scan-network-paths-not-followed-more`: the same "nơi" fix as its sample sibling.
- `installer-cli-folder-has-files`: began with a capital "Không" although it is printed inside an error sentence
  ("lỗi: dữ liệu đầu vào không hợp lệ: …"); now lower case, like the other command-line reasons.
- `terms.md`: added the app's "License and Activation Data" option, the firewall profile words, the short scope labels,
  and how the start words carry their own verb.

Checked across the language: no text left in English beyond product names, command names, and terms the glossary keeps
(`hive`, `stub` in brackets after its Vietnamese term). Buttons are short. `Ctrl+F` is as Vietnamese Windows writes it.
Plural messages all use only `*[other]`, as Vietnamese CLDR has.

## Open questions
- **Firewall profiles:** "miền", "riêng tư", "công cộng" are what I believe Windows 11 in Vietnamese shows for the
  Domain, Private and Public profiles (Settings shows "Công cộng" / "Riêng tư" for a network's profile type). A native
  speaker should confirm against Windows Defender Firewall's own pages, which may say "Mạng riêng" instead.
- **`app-column-arch` "Kiến trúc":** 48.6 px at Segoe UI Semibold 12 against a 46 px cell. It spills 2.6 px into the
  12 px gutter after the last column, which is harmless, but when the list is sorted by that column the chevron ends
  about 6 px past the list's edge. Vietnamese has no customary short form ("K.trúc" would be unusual); widening
  `ARCH_COLUMN` from 58 to about 66 px would be cleaner than an abbreviation.
- **`ui-disclosure`:** "đang mở" / "đang đóng" kept. Vietnamese Narrator may say "đã mở rộng" / "đã thu gọn" for
  expanded and collapsed, and "đang đóng" can also be heard as "closing".
- **Hyper-V's "Edit Disk" (`core-image-differencing`):** translated as "Chỉnh sửa đĩa". Hyper-V Manager may stay in
  English on Vietnamese Windows (Vietnamese is a language interface pack), in which case "Edit Disk" is what people see.
- **`core-source-nsis` "Chữ ký":** the English "signature" means a recognisable pattern, not a digital signature;
  "chữ ký" carries the same ambiguity. "Dấu hiệu nhận dạng" would be unambiguous.
- **`app-scope-per-user` "Người dùng":** chosen to fit the Scope column; a native speaker may prefer another short
  form that still pairs with "Toàn máy".
- English, raised by the translator: `core-rollback-setting-other-account` `[machine-variable]` / `[machine-list]` say
  a machine-wide variable "belongs to the account that installed this". Translated as written.

Checker: vi-VN: 0 errors, 0 warnings.
