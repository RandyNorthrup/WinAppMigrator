# Review: Traditional Chinese, Taiwan (zh-TW)

Reviewer: independent machine pass, 2026-11-15. Translator's form of address: 您, with requests as 「請…」 and plain
noun labels and buttons (「建立安裝程式」「複製路徑」); kept, because that is how Windows 10 and 11 in Taiwan address the
user. Full-width punctuation (，。：；（）「」), 、 between list items, and a half-width space between Chinese and Latin
text, digits, or a placeholder, as Microsoft's Taiwan style does. The only half-width spaces next to full-width
punctuation are the column padding in `client-this-pc`/`-windows`/`-pairing-code` and `installer-cli-usage-*`, which
keep the console columns aligned (each CJK character is two columns wide).

## Mainland-Chinese check

The whole catalog, not only the sample, was checked for terms from mainland China:

- Every character in every message encodes in Big5 (cp950), so no simplified character slipped in.
- A search for about 250 mainland computing words (软件/軟件, 程序 as "program", 文件 as "file", 文件夾, 默認, 設置, 網絡,
  信息, 數據, 用戶, 鏈接, 磁盤, 卸載, 打開, 保存 as "save", 菜單, 窗口, 屏幕, 運行, 註冊表, 快捷方式, 進程, 線程, 字體,
  支持, 兼容, 賬戶, 創建, 激活, 許可證, 簽名, 證書, 哈希, 緩存, 內存, 界面, 自定義, 搜索, 加載, 重啟, 文檔, 鏡像, 掛載,
  字符, 端口, 協議, 地址, 日誌, 列表, 對話框, 客戶端, 共享, 禁用, 臨時, 圖標, 粘貼, 撤銷, 查找, 文本, 模塊, 變量, 調試,
  構建, 操作系統, 計算機, 互聯網, 數據庫, 擴展名, 檢測, 密鑰, 會話, 字節, 補丁, 命令行, 腳本, 獲取, 訪問, 只讀, and
  others) found none. The few hits were Taiwan usage: 用戶端 (client), 處理程序 (process), 身分, 結束代碼 (exit code),
  登錄 (the registry).
- Reading every message of `core*.ftl`, `client.ftl`, `app.ftl`, `installer.ftl`, and `ui.ftl` turned up Taiwan forms
  throughout: 程式, 檔案, 資料夾, 預設, 設定, 網路, 資料, 使用者, 連結, 磁碟機, 解除安裝, 開啟, 視窗, 登錄/機碼/登錄區,
  捷徑, 處理程序, 執行緒, 字型, 支援, 相容性, 帳戶, 建立, 啟用, 授權, 簽章, 憑證, 雜湊, 介面, 搜尋, 載入, 映像, 掛接,
  字元, 連接埠, 位址, 記錄檔, 清單, 共用, 停用, 暫存, 剖析, 引數, 指令碼, 逾時, 協力廠商, 增益集, 硬體鎖, 文件 only
  for the Documents folder.

## Brought up to date

30 messages checked against the current English. 22 needed new or changed words, and three sentences the new words fill
were rebuilt:

- `app-scan-failed`: the reason now follows a colon, not a full stop (「無法檢查此應用程式：{ $error }」).
- `app-footprint-licenses-still-go`, `core-footprint-licenses-left-out`, `core-review-note-licenses`,
  `core-secret-why-license-option-off`, `core-warning-scan-license-data`: 「「授權」選項」 is now 「「授權與啟用資料」選項」,
  the words of the checkbox `app-option-license`.
- `core-warning-build-recorded-folder-left-out`, `core-warning-build-near-part-left-out` `[unreadable]`: was 「它無法讀取」
  ("it cannot be read"); now the safety rule: 「它的路徑會通往磁碟機以外的位置或經過連結，因此無法安全地讀取」.
- `core-review-start-boot`, `-system`, `-automatic`, `-delayed`, `-manual`, `-disabled`, `-unknown` (new):
  「隨 Windows 一起啟動，且早於其他所有項目」「在 Windows 啟動過程中隨之啟動」「在這台電腦每次啟動時啟動」
  「在這台電腦每次啟動後不久啟動」「只在被要求時才啟動」「已停用，因此永遠不會啟動」「以 Windows 未說明的方式啟動」.
  "never, as it is disabled" cannot sit after a shared verb, so each phrase carries its own 啟動, worded as the
  `[automatic]` and `[delayed]` variants already were. `core-review-service-detail` `[other]` and
  `core-review-driver-detail-starts` used to end 「啟動方式：{ $start }」, built for a raw value such as "boot"; they now end
  「，{ $start }」 (「將 disk.sys 載入 Windows 本身，隨 Windows 一起啟動，且早於其他所有項目」).
- `core-review-network-domain`, `-private`, `-public` (new): 「網域」「私人」「公用」, as Windows Defender Firewall names its
  profiles. In `core-review-firewall-detail` the space in 「{ $networks } 網路」 was removed, so it reads
  「套用於網域、私人網路」.
- `core-review-com-support-detail`: now names each count's kind:
  「其類別附帶的型別程式庫（{ $libraries }）、介面（{ $interfaces }）和 AppID（{ $app_ids }）設定」.
- `installer-license-detail`: 「啟用與授權的檔案和機碼」 ("keys" read as registry keys, loosely) is now
  「啟用與授權檔案，以及存放授權資料的登錄值」.
- `installer-no-programdata`, `installer-cli-no-programdata`: said "so the install cannot be undone"; now "so nothing was
  installed: an install keeps the record it is undone by there" (「…因此未安裝任何內容：安裝會將日後用來復原它的記錄保存在
  該資料夾中」).

Already right, no change needed: `client-builder-done`, `client-wrong-code`, `client-pair-failed` (「另一台電腦…」 starts
the sentence; Chinese has no capital), `core-rollback-setting-put-back`, `core-rollback-list-entries-removed`,
`core-rollback-list-entries-not-removed`, `core-rollback-firewall-removed` (the English capital "The" never reached
Chinese), `ui-disclosure` (present, with 「已展開」/「已摺疊」 as Narrator says them).

The `#` comments of these messages, and of `core-error-io`, `core-rollback-count-kept`, and
`core-rollback-count-left-behind`, were brought in line with English, so every message's comment now matches.

## Sample (back-translation)

| Message | Back-translation | Verdict |
| --- | --- | --- |
| ui-clear-search | Clear search | OK |
| ui-browse | Browse | OK |
| ui-browse-tooltip | Choose the folder for "{field}" | OK |
| ui-disclosure | {title}, {summary}, expanded / collapsed | OK |
| ui-language | Language | OK |
| ui-machine-translated | This translation was produced by machine and may have errors. | OK |
| ui-report-translation-problem | Report a translation problem | OK |
| ui-needs-direct3d-12 | This program needs Direct3D 12 to draw its window, which an old display-card driver or a virtual machine may not support. | Fixed: began 「這需要」 ("this needs"), which in a lone message box does not say what "this" is; now 「此程式需要」 |
| size-bytes | {value} bytes | OK (Chinese has only `other`) |
| size-kb / size-mb / size-gb / size-tb | {value} KB / MB / GB / TB | OK |
| list-separator | 、 | OK (Chinese enumeration comma) |
| installer-window-title | WinAppMigrator Installer | OK |
| installer-window-title-app | Install {app} | OK |
| installer-tagline | Reinstalls an application captured on another PC | OK |
| installer-unnamed-app | Application | OK |
| installer-window-failed | Could not open the installer's window: {error} | OK |
| installer-window-failed-without-one | It can still install without a window: run this file with --apply in an administrator command prompt; to choose the install location, add --target-root followed by a folder. | OK |
| installer-checking | Checking the installer… | OK |
| installer-stub-no-package | This is a WinAppMigrator installer stub and contains no application package. Build an installer with WinAppMigrator, then run the produced .exe here. | OK |
| installer-check-stopped | The installer's check has stopped. | OK |
| installer-locate-failed | Cannot find the installer: {error} | OK |
| installer-read-failed | Cannot read the installer: {error} | OK |
| installer-cannot-be-used-here | This installer's package cannot be used on this PC: {error} | OK |
| installer-captured-from | Capture source | OK |
| installer-install-location | Install location | OK |
| installer-choose-location | Choose the install location | OK |
| installer-folder-has-files | This folder already has files. Files with the same names are replaced, and are restored when you uninstall. | OK |
| installer-room-needed | Space needed | OK |
| installer-room-short | There may not be enough space here: {shortfall}. It can unpack on another drive, but the app's files must still fit. | OK |
| installer-interrupted | The earlier install of {app} did not finish. Clicking Install undoes it first. | OK |
| installer-signature | Installer | OK |
| installer-signature-checking | Checking the signature… | OK |
| installer-signature-unsigned | Not signed: Unknown publisher | OK (Windows' 「未知的發行者」) |
| installer-signature-signed-by | Signed by {publisher} | OK |
| installer-signature-unnamed | Signed by an unnamed publisher | OK |
| installer-signature-invalid | This installer's signature is invalid ({reason}). It may have been changed after it was signed. | OK |
| installer-more-left-out | {count} more items were left out | OK |
| uninstaller-plan-coming-back | The {count} files this install replaced are restored; if the file in their place has changed since the install, they go beside it. | OK |
| uninstaller-remove-user-data-detail | {count} items of its settings, data, and license. They stay unless this option is on. What they replaced is restored; if they have changed since the install, it goes beside them. | OK |
| installer-user-data-detail | On: settings, Documents, and Saved Games go to the matching folders of whoever is signed in. Off: everything goes under the install folder, and nothing is set up to run from it. | Fixed: 「一般資料夾」 reads as "general folders"; now 「對應資料夾」, the folders they belong in |
| installer-license-detail | Activation and license files, and the registry values that hold license data. Some may need reactivation here. | Fixed: brought up to date (see above) |
| app-tagline | Turn an installed application into a self-extracting installer | OK |
| app-list-search | Search by name or publisher  (Ctrl+F) | OK (Taiwan Windows writes Ctrl+F as is) |
| app-column-name | Name | OK |
| app-column-publisher | Publisher | OK |
| app-column-version | Version | OK |
| app-column-scope | Scope | OK |
| app-column-arch | Architecture (two characters) | OK |
| app-source | Scan source | OK |
| app-option-license | License and activation data | OK |
| app-option-user-data | User data | OK |
| app-option-whose-data | Account the data belongs to | OK |
| app-build-create | Create installer | OK |
| app-build-cancel | Cancel | OK |
| app-scan-look-again | Check again | OK |
| app-scan-try-again | Try once more | OK |
| app-result-open-folder | Open folder | OK |
| app-result-copy-path | Copy path | OK |
| app-result-copy-error | Copy error / copying failed | Fixed: 「複製錯誤」 also reads "copy error" as in a failed copy; now 「複製錯誤訊息」 (copy the error message) |
| app-network-connect | Connect | OK |
| core-scan-no-executable | Cannot find {app}'s executable. A driver, runtime, or add-in may have no standalone program to package. | OK |
| core-package-not-enough-room | Not enough space to build this installer, so nothing was read or written. {shortfalls}. Free some space there, or choose an output folder on another drive. | OK |
| core-warning-build-user-folder-not-on-drive | {user}'s {folder} folder is at {path}, not on this drive, so what is in it was not included. | OK |
| core-archive-data-file-missing | This installer stores its package in a file named {name} in the same folder, but that file is not there. The two files must stay together: copy {name} here from where the installer was built, then run again. | OK |
| core-archive-seek-outside | Search position outside the package | Fixed: 「搜尋」 is "search"; a file seek is 「定位」 (also `core-remote-seek-before-start`) |
| core-warning-restore-service-not-started | Service {name} is installed but did not start: {reason}. Until it starts the app may not work. When this PC next starts, Windows tries again. | OK |
| core-restore-already-exists | {path} already exists, so nothing was installed. Use --force to replace it | OK |
| core-warning-restore-earlier-undone | The earlier install of {app} did not finish, so it was undone first. {count} changes were put back. | OK |
| core-space-shortfall | {drive} needs up to {needed} free, for {purposes}, and has only {free} | OK |
| core-policy-fit-needs-newer | {program} says it needs Windows {needs} or newer, and this PC runs {runs}, so Windows would not start it | OK |
| core-review-no-room | Not enough space, so nothing was changed. {shortfalls}. Free some space there, or choose an install location on another drive. | OK |
| core-review-service-detail | Runs {file} as {account}, starts now and each time this PC starts / … / starts only when asked | Fixed: `[other]` rebuilt for the new start words; `[manual]` 「只在有項目要求時啟動」 ("when an item requests") now 「只在被要求時才啟動」, the same words as `core-review-start-manual` |
| core-review-firewall-detail | Allows incoming connections (port {ports}), for {program}, applied to {networks} networks | Fixed: no space before 網路 now that the profiles are Chinese words |
| core-review-com-support-detail | The type libraries ({n}), interfaces ({n}), and AppIDs ({n}) settings that come with its classes | Fixed: brought up to date |
| core-rollback-file-stays-beside | {path} stays, because something that stays still uses it. The file it replaced was put beside it, named {name}. | OK |
| core-warning-scan-task-unreadable | Cannot read scheduled task {path}: {reason}. If it runs this app, it was not included. | OK |
| core-runtime-download-cut-short | Download of {url} broke off: of {expected} bytes only {came} arrived | OK |

80 messages sampled (the four size units counted one each); 8 fixed, 4 of them as part of bringing the catalog up to
date with English.

Whole-language checks: the `app-column-*` headers are two or three characters (名稱, 發行者, 版本, 範圍, 架構); button labels are
two to six characters; nothing that should be translated is left in English (the Latin text left is product, file,
command, flag, and registry names, and the `installer-cli-` usage lines' flags).

## Other fixes

- `core-remote-seek-before-start`: 「搜尋位置在開頭之前」 is now 「定位到開頭之前的位置」, as for `core-archive-seek-outside`.
- `terms.md`: added the firewall profile names (網域、私人、公用), how the service start words are built, and 定位 for a file
  seek. No existing term was changed.

## Open questions

- Firewall profiles 「網域」「私人」「公用」 are given as Windows Defender Firewall with Advanced Security shows them in its
  Profile column; this pass could not open a Taiwan Windows to confirm them, nor that 「套用於網域、私人網路」 reads as
  naturally to a Taiwan reader as 「套用於網域、私人設定檔」 would.
- "Attach" a disk image is 「掛接」 throughout (Explorer's word for Mount). Disk Management says 「附加 VHD」 for Attach VHD.
  Both are Taiwan usage; a native speaker may prefer one.
- The service start words follow the English descriptions, not the Services console's own 「自動」「自動 (延遲開始)」
  「手動」「已停用」; that matches the English intent, but a reader who knows the console might expect those.
- `core-error-io` puts a space between `{ $path }` and 的; when the program fills it with words such as 「安裝程式」 rather
  than a path, the space is cosmetic only.

Checker: zh-TW: 0 errors, 0 warnings.
