# Words of the installer that every built installer carries, which runs on the PC an app is moved to and speaks its
# language, and of the uninstaller it leaves behind for Settings. Message names start with installer- for the
# installer's window, installer-cli- for its command line, and uninstaller- for the uninstaller.
#
# The command lines (installer-cli-, uninstaller-cli-) are always shown in English, as scripts read what they print.
# Translate them all the same. Keep their command-line flags, such as --apply and --target-root, exactly as they are.


## The installer's window: its title and header

installer-window-title = WinAppMigrator 安裝程式
# The window's title in the taskbar. { $app_name } is the name of the app it installs.
installer-window-title-app = 安裝 { $app_name }
installer-tagline = 重新安裝在另一台電腦上擷取的應用程式
# What an app is called when its package gives it no name.
installer-unnamed-app = 應用程式

## When the installer's window cannot open. Each is a paragraph of a message box.

# { $error } is what went wrong, as Windows or the graphics driver says it.
installer-window-failed = 無法開啟安裝程式的視窗：{ $error }
installer-window-failed-without-one = 仍可在沒有視窗的情況下安裝：請在系統管理員命令提示字元中以 --apply 執行此檔案；若要選擇安裝位置，請加上 --target-root 並在其後指定資料夾。

## Checking the installer when it opens

installer-checking = 正在檢查安裝程式…
installer-stub-no-package = 這是 WinAppMigrator 的安裝程式空殼，不含任何應用程式套件。請使用 WinAppMigrator 建置安裝程式，然後在這裡執行產生的 .exe。
installer-check-stopped = 安裝程式的檢查已停止。
# { $error } is what went wrong, as Windows says it.
installer-locate-failed = 找不到安裝程式：{ $error }
# { $error } is what went wrong reading it.
installer-read-failed = 無法讀取安裝程式：{ $error }
# { $error } is why, such as that the app needs a newer Windows.
installer-cannot-be-used-here = 此安裝程式的套件無法在這台電腦上使用：{ $error }

## The app, where it goes, and whether the installer is signed

installer-captured-from = 擷取來源
installer-install-location = 安裝位置
# The title of the folder picker.
installer-choose-location = 選擇安裝位置
installer-folder-has-files = 此資料夾中已有檔案。同名的檔案會被取代，並會在您解除安裝時恢復。
installer-room-needed = 所需空間
# { $shortfall } says which drive is short of room, such as "C: needs up to 824.0 MB for the app's files, and has 300.0 MB free".
installer-room-short = 這裡的空間可能不足：{ $shortfall }。它可以在其他磁碟機上解壓縮，但應用程式的檔案仍必須放得下。
# { $app_name } is the name of an app whose earlier install was cut short.
installer-interrupted = 先前安裝 { $app_name } 的作業未完成。按「安裝」時會先將它復原。
# Beside "Installer", for whether it proves who made it.
installer-signature = 安裝程式
installer-signature-checking = 正在檢查簽章…
installer-signature-unsigned = 未簽署：未知的發行者
# { $publisher } is the name on the installer's certificate, such as "Fabrikam, Inc.".
installer-signature-signed-by = 由 { $publisher } 簽署
installer-signature-unnamed = 由未具名的發行者簽署
# { $reason } is Windows' own word for what is wrong, such as HashMismatch.
installer-signature-invalid = 此安裝程式的簽章無效（{ $reason }）。它可能在簽署後遭到變更。

## Why the install location cannot be used. Install and Preview Changes stay off while one of these is shown.

# { $app_name } is the name of the app being installed.
installer-location-app-already-there = { $app_name } 已安裝在此資料夾中，因此這份複本無法安裝到這裡。若要安裝在這裡，請先在 Windows「設定」的「應用程式」中，解除安裝這台電腦上的複本，然後重新開啟此安裝程式。或者，在上方輸入新的空白資料夾。
# { $other_app } is the name of another app Windows lists as installed in the folder. { $app_name } is the name of the app being installed.
installer-location-other-app-there = { $other_app } 已安裝在此資料夾中，因此 { $app_name } 無法安裝到這裡。請在上方輸入新的空白資料夾。或者，如果您不再需要 { $other_app }，請先在 Windows「設定」的「應用程式」中將它解除安裝，然後重新開啟此安裝程式。
installer-location-has-files = 此資料夾中已有不是由 WinAppMigrator 安裝的檔案。這些檔案可能屬於其他應用程式，因此不會在其上安裝任何內容。請在上方輸入新的空白資料夾。
# { $reason } is why, such as that it cannot be the Windows folder.
installer-location-refused = 無法使用此資料夾：{ $reason }。請在上方輸入其他資料夾。

## What the install sets up beyond its files

installer-beyond-files = 檔案以外會設定的項目
# Under the heading above. It is part of what a person agrees to when they click Install.
installer-beyond-files-agreed = 這些作業全都會以系統管理員權限執行。如果安裝失敗，所做的變更會恢復原狀，任何無法恢復的項目都會告知您。您之後也可以從「設定」的「已安裝的應用程式」中復原此安裝。被取代的檔案會恢復，而它安裝的執行階段則會保留。
installer-beyond-files-none = 無：它只會複製檔案，以及應用程式本身登錄機碼中的設定。
# { $problem } is why, such as that the app cannot run on this PC.
installer-cannot-go-here = 此套件無法安裝到此位置：{ $problem }
installer-working-out = 正在判斷在此位置會設定哪些項目…
# Said as the reason after "This package cannot go to this location:".
installer-list-stopped = 判斷要執行的作業時已停止
# A heading over a list of what else the package asked for and will not get. { $count } is how many.
installer-more-left-out =
    { $count ->
       *[other] 另有 { $count } 個項目已排除
    }

## Options

installer-options = 選項
installer-user-data = 使用者資料
installer-user-data-detail = 開啟：設定、文件和儲存的遊戲會放到目前登入者的對應資料夾中。關閉：全部放在安裝資料夾下，且不會設定任何從中執行的項目。
# { $account } is the name of the account on the other PC whose data this is.
installer-user-data-whose = 這包含 { $account } 的資料。
installer-license = 授權
installer-license-detail = 啟用與授權檔案，以及存放授權資料的登錄值。部分項目可能需要在這裡重新啟用。

## Dependencies: what else the app needs, and how each is provided

installer-dependencies = 相依性
installer-dependencies-detail = 此應用程式需要的項目，以及各項目的提供方式。
installer-dependencies-none = 未偵測到其他執行階段相依性。
# { $feature } is the name of an optional part of Windows, such as NetFx3.
installer-dependency-windows-feature = Windows 功能：{ $feature }
installer-dependency-included = 已包含
# { $id } is the package's name in winget, such as Microsoft.VCRedist.2015+.x64.
installer-dependency-winget = 透過 winget 安裝：{ $id }
installer-dependency-manual = 手動安裝
installer-dependency-built-in = 內建於 Windows
installer-dependency-ships-with-app = 隨應用程式提供

## Buttons, and what is said beside them

installer-install = 安裝
installer-preview = 預覽變更
installer-cancel = 取消
installer-close = 關閉
installer-open-folder = 開啟安裝資料夾
# { $error } is what went wrong, as Windows says it.
installer-open-folder-failed = 無法開啟資料夾：{ $error }
installer-location-blocks = 在安裝位置可以使用之前，「安裝」和「預覽變更」都會停用。原因顯示在上方的安裝位置下。
installer-package-blocks = 由於此套件無法安裝到此位置，「安裝」已停用。原因顯示在上方。
installer-list-changed = 在這裡要執行的作業已有變更，因此未安裝任何內容。請檢閱下方清單，如果內容符合您的需求，請再按一次「安裝」。
installer-cancelled-early = 已取消。未做任何變更。

## How far an install has got, beside its progress bar

# { $step } is one of the steps below. { $done } and { $total } are how many of its items are done, and how many there are.
installer-step-counted = { $step }：{ $done } / { $total }…
# { $step } is one of the steps below, when it does not count its items.
installer-step-uncounted = { $step }…
installer-step-getting-ready = 正在準備…
installer-step-cancelling = 將在目前步驟完成後取消…
installer-step-putting-back = 正在恢復已變更的項目
installer-step-unpacking = 正在解壓縮
installer-step-checking-package = 正在檢查套件
installer-step-recovering = 正在復原先前未完成的安裝
# What a preview is doing, whatever step it is on.
installer-step-previewing = 正在判斷會變更的項目
installer-step-runtimes = 正在安裝執行階段
installer-step-files = 正在複製檔案
installer-step-services = 正在設定服務
installer-step-drivers = 正在設定驅動程式
installer-step-registry = 正在寫入登錄設定
installer-step-license = 正在還原授權資料
installer-step-tasks = 正在設定排定的工作
installer-step-shortcuts = 正在建立捷徑
installer-step-environment = 正在設定環境變數
installer-step-settings-entry = 正在將應用程式新增至「設定」
installer-step-verifying = 正在檢查已安裝的項目
installer-step-undoing = 正在復原

## When an install cannot go ahead or stops

installer-carries-no-package = 此安裝程式不含套件。
installer-stopped-unexpectedly = 安裝意外停止。
# { $reason } is what went wrong, as the program says it.
installer-stopped-unexpectedly-because = 安裝意外停止：{ $reason }。如果安裝已經開始，再次執行安裝程式時會先復原它所做的變更。
# { $error } is what went wrong reading it.
installer-package-read-failed = 無法讀取套件：{ $error }
installer-no-programdata = 找不到這台電腦的 ProgramData 資料夾，因此未安裝任何內容：安裝會將日後用來復原它的記錄保存在該資料夾中。
# { $error } is why, such as that there is nowhere only administrators can get into.
installer-staging-failed = 無法建立暫存資料夾：{ $error }
# { $error } is what went wrong unpacking it.
installer-unpack-failed = 無法解壓縮套件：{ $error }
installer-staging-nowhere = 找不到只有系統管理員能存取的位置來解壓縮套件
# { $failure } is a folder and what went wrong there, such as "C:\ProgramData\WinAppMigrator: Access is denied.".
installer-staging-nowhere-look-at = 找不到只有系統管理員能存取的位置來解壓縮套件。需要檢查的是 { $failure }

## What came of an install, or of a preview

installer-result = 安裝結果
installer-result-preview = 預覽：尚未做任何變更
installer-result-installed = 應用程式已安裝，並通過雜湊檢查。
installer-result-installed-without-left-out = 應用程式已安裝，並通過雜湊檢查，但不含下方列為已排除的項目。
installer-result-not-all-put-back = 安裝失敗，且所做的變更無法全部恢復。剩下的項目列在下方，下次安裝時會再試一次。
installer-result-cancelled-put-back = 安裝已取消，所做的變更已恢復原狀。
installer-result-failed-put-back = 安裝失敗，所做的變更已恢復原狀。
installer-result-not-clean = 安裝未順利完成。請檢閱下方的問題。
installer-result-changes-put-back = 已恢復的變更
installer-result-changes-made = 已做的變更
installer-result-changes-planned = 預計的變更
installer-result-warnings = 警告
installer-result-errors = 錯誤
installer-result-package-valid = 套件雜湊有效
installer-result-package-invalid = 套件雜湊無效
installer-result-files-valid = 已安裝的檔案有效
installer-result-files-invalid = 已安裝的檔案無效
installer-result-reboot = 有相依元件需要重新啟動才能完成。請重新啟動 Windows，然後啟動應用程式。
# A heading over the list of what was changed. { $count } is how many.
installer-result-changes = 變更（{ $count }）
# After the first part of a long list. { $count } is how many more there are.
installer-result-changes-more = 以及其他 { $count } 項。
# Over the path of the install's log file.
installer-log = 記錄檔

## The installer's command line. Shown in English: scripts read it. A { $count } is plain digits, with no separators.

installer-cli-stub = WinAppMigrator 安裝程式空殼：不含任何應用程式套件。
installer-cli-stub-how = 請使用 WinAppMigrator 應用程式建置安裝程式，然後在目標電腦上執行產生的 .exe。
installer-cli-usage-title = WinAppMigrator 自解壓縮安裝程式。
installer-cli-usage = 不加任何引數執行即可開啟圖形化安裝程式，或使用：
installer-cli-usage-preview = 預覽重新安裝： --dry-run [--target-root <資料夾>]
installer-cli-usage-apply = 重新安裝：     --apply [--target-root <資料夾>]
installer-cli-no-package = 安裝程式不含內嵌的套件
installer-cli-no-programdata = 找不到這台電腦的 ProgramData 資料夾，因此未安裝任何內容：安裝會將日後用來復原它的記錄保存在該資料夾中
# { $path } is the folder asked for, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-folder-has-files = 無法使用 { $path }：{ $problem }。只有在您確定它屬於此應用程式時，才指定 --force
installer-cli-no-location = 此套件未說明其應用程式原本的位置，因此請指定 --target-root
# { $path } is where the app was on the other PC, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-location-refused = 此套件的應用程式原本位於 { $path }，但該位置無法使用：{ $problem }。請指定 --target-root
# { $summary } is how much room it needs where, such as "up to 824.0 MB on C: (356.0 GB free)".
installer-cli-room = 空間：{ $summary }
# { $note } is something to know about the install.
installer-cli-note = 注意：{ $note }
# { $count } is how many things it sets up beyond its files.
installer-cli-sets-up = 設定項目：{ $count }
installer-cli-applied = 已套用重新安裝。
installer-cli-previewed = 已預覽重新安裝。
# { $count } is how many changes the install logged.
installer-cli-changes-logged = 已記錄的變更：{ $count }
# { $count } is how many warnings.
installer-cli-warnings = 警告：{ $count }
# { $count } is how many errors.
installer-cli-errors = 錯誤：{ $count }
# { $count } is how many things the package asked for and did not get.
installer-cli-refused = 已拒絕：{ $count }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-package-valid = 套件雜湊有效：{ $valid }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-reinstall-valid = 重新安裝雜湊有效：{ $valid }
# { $id } is the install's id, such as wam-0f3a….
installer-cli-install-id = 安裝識別碼：{ $id }
installer-cli-put-back = 安裝失敗，因此所做的變更已恢復原狀。
# { $path } is the log file.
installer-cli-log = 記錄檔：{ $path }
# { $path } is the log file. Printed before an error.
installer-cli-error-log = 記錄檔：{ $path }
# { $error } is what went wrong.
installer-cli-error = 錯誤：{ $error }

## The uninstaller's window

uninstaller-window-title = 解除安裝
# The window's title, and its heading. { $app_name } is the name of the app it uninstalls.
uninstaller-window-title-app = 解除安裝 { $app_name }
uninstaller-tagline = 復原其安裝程式所做的變更
uninstaller-reading = 正在讀取此安裝的記錄…
uninstaller-stopped-unexpectedly = 解除安裝意外停止。
uninstaller-what-will-be-removed = 將移除的項目
uninstaller-install-location = 安裝位置
uninstaller-files = 檔案
uninstaller-registry-values = 登錄值
uninstaller-services = 服務
uninstaller-drivers = 驅動程式
uninstaller-scheduled-tasks = 排定的工作
uninstaller-environment = 環境變數
uninstaller-plan-stays = 您新增或變更的檔案會保留，其所在的資料夾也會保留。
# { $count } is how many files the install replaced.
uninstaller-plan-coming-back =
    { $count ->
       *[other] 此安裝所取代的 { $count } 個檔案會恢復；如果原處的檔案在安裝後已變更，則會放在該檔案旁邊。
    }
# { $runtimes } is a list of runtimes, such as "Microsoft Visual C++ Redistributable, .NET Desktop Runtime 8".
uninstaller-plan-runtimes = 它安裝的執行階段會保留，因為其他程式可能會使用：{ $runtimes }。
uninstaller-options = 選項
uninstaller-remove-user-data = 同時移除使用者資料和授權資料
# { $count } is how many items of the app's settings, data, and license there are.
uninstaller-remove-user-data-detail =
    { $count ->
       *[other] 其設定、資料和授權共 { $count } 個項目。除非開啟此選項，否則會保留這些項目。被它們取代的項目會恢復；如果這些項目在安裝後已變更，則會放在其旁邊。
    }
uninstaller-uninstall = 解除安裝
uninstaller-cancel = 取消
uninstaller-close = 關閉
# { $done } and { $total } are how many of the install's items are undone, and how many there are.
uninstaller-step-counted = 正在復原安裝：{ $done } / { $total }…
uninstaller-step = 正在復原安裝…
uninstaller-done = 應用程式已解除安裝。
uninstaller-done-except = 應用程式已解除安裝，但下方列出的項目除外。
uninstaller-waits-for-drive = 有部分內容位於未連接的磁碟機上。請連接該磁碟機，然後從「設定」再次解除安裝。
uninstaller-not-all-undone = 無法全部復原。請關閉正在使用它的程式，然後從「設定」再次解除安裝。
# Over the path of the uninstall's log file.
uninstaller-log = 記錄檔
uninstaller-undone = 已復原
uninstaller-left-in-place = 保留原狀
uninstaller-could-not-be-undone = 無法復原
# After the first part of a long list. { $count } is how many more there are.
uninstaller-and-more = 以及其他 { $count } 項。
# Said as the reason when there is no record of the install to read.
uninstaller-no-programdata = 找不到這台電腦的 ProgramData 資料夾，因此沒有可讀取的記錄

## When the uninstaller's window cannot open. Each is a paragraph of a message box, followed by a command to type.

uninstaller-window-failed-title = WinAppMigrator 解除安裝程式
# { $error } is what went wrong, as Windows or the graphics driver says it.
uninstaller-window-failed = 無法開啟解除安裝程式的視窗：{ $error }
uninstaller-window-failed-quiet = 仍可在沒有視窗的情況下解除安裝，請在系統管理員命令提示字元中執行：
# Stands in for this program's own path in that command, if it cannot be found.
uninstaller-this-program = 此程式

## The uninstaller's command line (--quiet). Shown in English: scripts read it.

# { $app_name } is the name of the app.
uninstaller-cli-uninstalling = 正在解除安裝 { $app_name }。
# { $path } is the log file.
uninstaller-cli-log = 記錄檔：{ $path }
# { $error } is what went wrong.
uninstaller-cli-error = 錯誤：{ $error }
