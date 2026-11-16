# Words of the app, WinAppMigrator itself: its one window, with the installed apps listed on the left and, on the
# right, what goes in an installer of the one picked and the button that builds it; and the steps to read another PC
# through WinAppMigrator Client. Message names start with app-.


## The window and its header

# The program's name, in the window's title bar and at the top of the window. Keep it as it is unless your language
# writes product names in a script of its own.
app-name = WinAppMigrator
app-tagline = 將已安裝的應用程式變成自解壓縮安裝程式
app-theme-to-light = 切換為淺色模式
app-theme-to-dark = 切換為深色模式
app-reload-list = 重新載入應用程式清單
app-list-loading = 正在載入
# { $count } is how many apps are listed.
app-list-count =
    { $count ->
       *[other] { $count } 個應用程式
    }
app-list-load-failed = 載入失敗
# Shown when the window cannot open at all. { $error } is what went wrong, in the words of Windows or a graphics driver.
app-window-could-not-open = 無法開啟視窗：{ $error }

## The list of installed apps

app-list-search = 依名稱或發行者搜尋  (Ctrl+F)
app-list-filter-all = 全部
# Whether an app is installed for one account or for everyone on the PC.
app-scope-per-user = 單一使用者
app-scope-per-machine = 所有使用者
app-scope-unknown = 未知
# An app whose processor architecture (x86, x64, ARM64) is not known.
app-arch-unknown = 未知
app-list-loading-apps = 正在載入已安裝的應用程式…
# { $error } is why the apps could not be listed.
app-list-could-not-list = 無法列出應用程式：{ $error }
app-list-no-matches = 沒有相符的應用程式
app-list-no-matches-hint = 請調整搜尋文字或範圍篩選條件。
app-list-worker-disconnected = 應用程式清查背景工作已中斷連線。
# What a screen reader says of an app's row. { $name } is the app's name, { $publisher } who made it, and { $version } its
# version, such as 11.4.2, or — when either is not known.
app-list-row = { $name }，{ $publisher }，版本 { $version }

## The list's columns, and what a screen reader says each column's heading does when clicked

app-column-name = 名稱
app-column-name-sort = 依名稱排序
app-column-publisher = 發行者
app-column-publisher-sort = 依發行者排序
app-column-version = 版本
app-column-version-sort = 依版本排序
app-column-scope = 範圍
app-column-scope-sort = 依範圍排序
# Short for architecture: x86, x64, or ARM64. The column is narrow.
app-column-arch = 架構
app-column-arch-sort = 依架構排序

## Where apps are read from: this PC, a drive from another PC, a disk image of one, or another PC on the network

app-source = 掃描來源
app-source-this-pc = 本機
app-source-drive = 連接的磁碟機…
app-source-image = 磁碟映像…
app-source-network = 網路電腦…
app-source-attaching-image = 正在以唯讀方式掛接映像…
# { $computer } is the other PC's name, such as OLD-PC, and { $address } its network address, such as 192.168.1.20.
app-source-network-pc = 透過網路連線的 { $computer }（{ $address }）
app-source-disconnect = 中斷連線
app-source-change = 變更…
# The title of the window that picks a disk image. .vhd and .vhdx are file extensions.
app-source-pick-image-title = 選取另一台電腦的磁碟映像（.vhd 或 .vhdx）
app-source-pick-image-filter = 磁碟映像
# { $path } is the disk image file.
app-source-image-has-no-windows = { $path } 中沒有安裝 Windows 的磁碟區。
app-source-image-not-attached = 無法掛接磁碟映像。
app-source-pick-drive-title = 選取連接之磁碟機的根目錄（例如 E:\）
# { $path } is the folder picked, such as E:\. \Windows and \Users are folder names, as Windows writes them.
app-source-not-a-windows-drive = { $path } 看起來不像 Windows 系統磁碟機（沒有 \Windows 和 \Users）。請選取來自另一台電腦之磁碟機的根目錄。
# Said of a drive or disk image with a Windows older than Windows 10. { $windows } is its name, such as "Windows 7
# Professional Service Pack 1, 32-bit". { $is_32_bit } is "yes" when that Windows is 32-bit, which adds the last
# sentence, and "no" when it is not.
app-source-older-windows =
    其中的系統是 { $windows }。從中建置的安裝程式適用於 Windows 10 和 11。開啟「使用者資料」時，Windows 為舊版應用程式悄悄另外保存的設定也會一併帶過去，而 Windows 11 沒有的執行階段（例如 DirectX 9）會隨應用程式一起安裝。{ $is_32_bit ->
        [yes] 其應用程式為 32 位元，因此會安裝到 Program Files (x86)，也就是 64 位元 Windows 存放這類應用程式的位置。
       *[no] { "" }
    }
# The same, of a Windows whose name could not be read. { $is_32_bit } is as above.
app-source-older-windows-unnamed =
    其中的系統是舊版 Windows。從中建置的安裝程式適用於 Windows 10 和 11。開啟「使用者資料」時，Windows 為舊版應用程式悄悄另外保存的設定也會一併帶過去，而 Windows 11 沒有的執行階段（例如 DirectX 9）會隨應用程式一起安裝。{ $is_32_bit ->
        [yes] 其應用程式為 32 位元，因此會安裝到 Program Files (x86)，也就是 64 位元 Windows 存放這類應用程式的位置。
       *[no] { "" }
    }

## The details panel, before an app is picked, and the app picked

app-details-pair-first = 與舊電腦配對
app-details-pair-first-hint = 當 WinAppMigrator Client 在舊電腦上執行，且您已輸入它顯示的配對碼後，舊電腦的應用程式就會列在這裡，您可以像處理這台電腦的應用程式一樣，為其中之一建置安裝程式。
app-details-pick-an-app = 選擇應用程式
app-details-pick-an-app-hint = 從清單中選取應用程式，以選擇要包含的內容並建置其安裝程式。
# { $publisher } is who made the app, and { $version } its version, such as 11.4.2.
app-details-publisher-and-version = { $publisher }，版本 { $version }
# { $version } is the app's version, such as 11.4.2.
app-details-unknown-publisher-and-version = 未知的發行者，版本 { $version }
app-details-unknown-publisher = 未知的發行者

## What goes in besides the program

app-option-license = 授權與啟用資料
app-option-license-hint = 可能與這台電腦綁定，需要在新電腦上重新啟用。
app-option-user-data = 使用者資料
# Documents, Saved Games, and ProgramData are folders of Windows: use the names Windows gives them in your language,
# except ProgramData, which keeps its name.
app-option-user-data-hint = 設定和歷程記錄、文件、儲存的遊戲，以及 ProgramData。
app-option-whose-data = 資料所屬帳戶
app-option-whose-data-hint = 一個安裝程式只包含一個帳戶的資料。資料會放到執行安裝程式時登入的帳戶中。

## Building the installer

app-output-folder = 輸出資料夾
# An example of a folder, shown in the empty field. "You" stands for the name of an account.
app-output-folder-example = C:\Users\使用者\WinAppMigrator
app-output-folder-not-full = 請輸入完整的資料夾路徑，例如 D:\Installers。
app-build-create = 建立安裝程式
app-build-create-again = 再次建立安裝程式
app-build-cancel = 取消
app-build-cancelling = 將在目前步驟完成後取消…
app-build-cancelled = 已取消建置。未寫入任何安裝程式。
# winappmigrator-installer.exe is a file name.
app-build-no-stub = 在此應用程式旁找不到安裝程式空殼（winappmigrator-installer.exe），因此無法建置自解壓縮安裝程式。
app-build-select-an-app-first = 請先選取要移轉的應用程式。
app-build-look-again-first = 必須重新檢查此應用程式，才能建置。
# { $reason } is what went wrong, in the program's own words, often in English.
app-build-stopped-unexpectedly = 建置意外停止：{ $reason }。如果它在輸出資料夾中留下了安裝程式，請勿使用。
app-build-worker-disconnected = 移轉背景工作已中斷連線。
# { $path } is the output folder, and { $error } what Windows said went wrong.
app-build-no-output-folder = 無法建立輸出資料夾 { $path }：{ $error }
# { $error } is what went wrong.
app-build-no-staging-folder = 無法建立受保護的暫存資料夾，因此未建置任何內容：{ $error }
# { $error } is what went wrong, as a sentence.
app-build-could-not-build = 無法建置安裝程式。{ $error }
app-build-no-installer = 建置已完成，但沒有產生安裝程式。
# { $error } is what went wrong.
app-build-hash-check-did-not-run = 安裝程式已寫入，但無法執行其雜湊檢查：{ $error }。請勿使用。

## How far a build has got

app-progress-preparing = 正在判斷要包含的內容…
app-progress-checking-room = 正在檢查空間是否足夠…
app-progress-bundling = 正在加入隨附的執行階段和驅動程式…
# { $done } is how many files have been read so far, and { $total } how many there are.
app-progress-packing = 正在讀取並壓縮檔案：{ $done } / { $total }
# { $done } is how many of the installer's records have been looked over so far, and { $total } how many there are.
app-progress-looking-over = 正在檢閱安裝程式的內容：{ $done } / { $total }…
app-progress-writing = 正在寫入安裝程式…
app-progress-verifying = 正在驗證雜湊…

## What a build made

app-result-failed-hash-check = 請勿使用此安裝程式：它未通過雜湊檢查。
# { $count } is how many files were gone when the build came to read them.
app-result-files-gone =
    { $count ->
       *[other] 安裝程式已建置，但有 { $count } 個檔案在檢查應用程式後已不存在，因此未包含在內。請參閱下方的「警告」。
    }
# { $count } is how many files could not be read into the installer.
app-result-files-not-captured =
    { $count ->
       *[other] 安裝程式已建置，但有 { $count } 個檔案無法擷取，通常是因為檔案已鎖定或無法讀取。缺少這些檔案，應用程式可能無法運作。請參閱下方的「警告」。
    }
app-result-mentions-who = 安裝程式已建置，但其中仍含有關於其來源的資訊。在將它提供給任何人之前，請先閱讀下方的說明。
app-result-ready = 安裝程式已就緒。其套件已通過雜湊檢查。
app-result-files = 檔案
app-result-installer-size = 安裝程式大小
app-result-warnings = 警告
app-result-log = 記錄檔
app-result-copy-error = 複製錯誤訊息
app-result-installer = 安裝程式
app-result-data-file = 資料檔
app-result-open-folder = 開啟資料夾
app-result-copy-path = 複製路徑
# { $error } is what Windows said went wrong.
app-result-could-not-open-folder = 無法開啟資料夾：{ $error }
# { $installer } and { $data_file } are file names, such as App_installer.exe and App_installer.wamdata.
app-result-two-files = 此應用程式太大，無法放入單一執行檔，因此其安裝程式分為兩個檔案：{ $installer } 和 { $data_file }。這兩個檔案必須放在一起：請將兩者以相同的名稱複製到另一台電腦上的同一個資料夾中。如果您簽署安裝程式，該簽章也會涵蓋資料檔。
# { $app_name } is the app's name. "Unknown publisher" is what Windows shows when it asks to run an unsigned program: use
# Windows' own words for it in your language.
app-result-run-it = 在另一台電腦上執行它，即可重新安裝 { $app_name }。它會以 SHA-256 檢查其檔案。由於您建置的安裝程式未經簽署，Windows 會顯示「未知的發行者」。
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-verification-errors = 驗證錯誤（{ $count }）
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-warnings-heading = 警告（{ $count }）
# After the first of a long list of warnings. { $count } is how many more there are.
app-warnings-and-more = 以及其他 { $count } 項。

## Looking at an app, to see what an installer of it would hold

app-scan-getting-ready = 正在準備檢查此應用程式…
app-scan-install-record = 正在讀取其安裝方式…
app-scan-near-matches = 正在尋找可能也屬於它的項目…
app-scan-system-items = 正在尋找其服務、排定的工作和捷徑…
app-scan-files = 正在檢查其檔案…
app-scan-registry = 正在讀取它與 Windows 的整合項目…
app-scan-finishing = 即將完成…
app-scan-look-again = 重新檢查
app-scan-stop = 停止
app-scan-running-hint = 執行期間，您可以選擇其他應用程式，或變更要包含的內容。
# { $error } is why, in words that follow a colon.
app-scan-failed = 無法檢查此應用程式：{ $error }
app-scan-try-again = 再試一次
app-scan-stopped = 已停止。此應用程式檢查完成後，才能建置安裝程式。
# { $reason } is what went wrong, in the program's own words, often in English.
app-scan-stopped-unexpectedly = 它意外停止：{ $reason }。

## How big an installer can be, and whether there is room for it

app-room-files = 檔案
# Over how big the installer can be: up to that size.
app-room-up-to = 最多
app-room-free = 可用
# { $drive } is a drive, such as C:.
app-room-free-on = { $drive } 的可用空間
# How much room a drive has, when it would not say.
app-room-free-unknown = 未知
app-room-most-it-could-need = 壓縮前可能需要的最大空間。
# { $drive } is a drive, such as C:, { $needed } how much room it needs, such as 1.2 GB, and { $free } how much it has.
app-room-drive-short = { $drive } 最多需要 { $needed } 的可用空間才能建置此安裝程式，目前只有 { $free }。
# { $drives } is one sentence like the one above for each drive without the room, one after another.
app-room-not-enough = 空間不足。{ $drives }請在其他磁碟機上選擇輸出資料夾，或排除部分項目。
# { $note } is a sentence, without its full stop, about a drive whose room could not be checked. Use your language's
# full stop.
app-room-could-not-tell = { $note }。

## What goes in the installer, part by part

app-footprint-heading = 安裝程式包含的內容
app-footprint-working-out = 正在判斷與其相關的項目…
# { $error } is why.
app-footprint-would-not-make-a-package = 剩下的內容無法構成套件：{ $error }
# { $list } is the names of what else is left out, such as a service and a shortcut.
app-footprint-also-left-out = 這也會排除 { $list }，因為它們執行或指向的項目將不存在。這些項目不會在另一台電腦上設定。
# { $list } is the names of the first few of what else is left out, and { $count } how many more there are.
app-footprint-also-left-out-and-more = 這也會排除 { $list } 以及其他 { $count } 項，因為它們執行或指向的項目將不存在。這些項目不會在另一台電腦上設定。
# { $list } is the names of license files. The option is the one named "License and Activation Data".
app-footprint-licenses-still-go = 您排除的內容中的授權資料仍會包含在內，因為這由「授權與啟用資料」選項決定：{ $list }。
# { $elapsed } is how long ago, in minutes and seconds such as 45:00, or hours, minutes, and seconds such as 1:05:00.
# "Look Again" names the button of that name.
app-footprint-looked-at-long-ago = 此應用程式是在 { $elapsed } 前檢查的。檔案會在您建置時讀取，因此變更會被納入，屆時已不存在的項目也會列出。若要納入任何新項目，請按「重新檢查」。
# Headings of the parts of the list, each with a line that says what is in it.
app-footprint-program-files = 程式檔案
app-footprint-data-folders = 資料存放位置
# What could go in with the User Data option, when nothing else is in this part. { $size } is how much, such as 1.2 GB.
app-footprint-folders-with-user-data = 開啟「使用者資料」選項時為 { $size }
# { $included } is how many folders go in, { $count } how many there are, and { $size } how much those that go in hold.
app-footprint-folders-summary =
    { $count ->
       *[other] { $included } / { $count } 個資料夾，{ $size }
    }
# { $size } is how much, such as 1.2 GB, and { $count } in how many files.
app-footprint-data-only-with-user-data =
    { $count ->
       *[other] 此應用程式的資料中，有 { $size }（{ $count } 個檔案）只會在開啟「使用者資料」選項時包含。
    }
# A folder's row. { $count } is how many files it holds, and { $detail } where it is or why it cannot be left out.
app-footprint-folder-row =
    { $count ->
       *[other] { $count } 個檔案。{ $detail }
    }
app-footprint-services = 服務
app-footprint-startup = 自動啟動
app-footprint-tasks = 排定的工作
app-footprint-shortcuts = 捷徑
# { $included } is how many of the part's things go in, and { $count } how many there are.
app-footprint-items-summary = { $included } / { $count }
# { $why } is what else was left out that this goes with.
app-footprint-left-out-too = 也已排除：{ $why }
# A row's detail. { $kind } is what it is, such as Service, and { $detail } what it does.
app-footprint-change-detail = { $kind }。{ $detail }
app-footprint-integrations = 與 Windows 的整合
# { $included } is how many kinds go in, such as file types and firewall rules, and { $count } how many kinds there are.
app-footprint-integrations-summary =
    { $count ->
       *[other] { $included } / { $count } 種
    }
app-footprint-integration-nothing-left = 由於您排除的內容，已無剩餘項目
# { $count } is how many things of this kind the installer sets up.
app-footprint-integration-things =
    { $count ->
       *[other] 設定 { $count } 個項目
    }
app-footprint-prerequisites = 執行所需的項目
# { $count } is how many runtimes, such as Visual C++ Redistributable.
app-footprint-prerequisites-summary =
    { $count ->
       *[other] { $count } 個執行階段
    }
app-footprint-near-matches = 可能也屬於它的項目
# { $ticked } is how many were ticked, and { $count } how many there are.
app-footprint-near-matches-summary = 已新增 { $ticked } / { $count } 個
app-footprint-near-matches-hint = 除非您勾選，否則不會包含在安裝程式中。勾選一個項目會重新檢查應用程式，並套用與其他項目相同的檢查，因此看起來像密碼的內容仍會被排除。
# { $why } is why it might be part of the app, as a sentence.
app-footprint-near-match-with-user-data = 隨「使用者資料」選項包含。{ $why }
# How much a folder holds, when not all of it could be measured. { $size } is how much was, such as 1.2 GB.
app-footprint-near-match-over = 超過 { $size }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-folder = 資料夾：{ $place }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-hidden = 應用程式的隱藏部分：{ $place }
app-footprint-nothing-left-out = 沒有任何項目因看起來像密碼、授權或憑證而被排除。
app-footprint-left-out = 排除的項目及原因
# { $count } is how many things were left out.
app-footprint-left-out-summary =
    { $count ->
       *[other] { $count } 個項目
    }
# Above a list of things left out for one reason. { $title } is the reason.
app-footprint-left-out-group = { $title }：
app-footprint-refused = 任何安裝程式都不得設定這些項目：
# A heading over a list that opens and closes. { $count } is how many are in it.
app-footprint-worth-knowing = 注意事項（{ $count }）
# { $list } is what kinds of things were looked for and none found, from the five below.
app-none-found = 找不到：{ $list }。
app-none-found-services = 服務
app-none-found-startup = 啟動項目
app-none-found-tasks = 排定的工作
app-none-found-shortcuts = 捷徑
app-none-found-integrations = 與 Windows 整合的任何項目

## Network PC: reading another PC through WinAppMigrator Client, in three steps. "WinAppMigrator Client" is the name of a
## program, and the old PC is the one the apps are moved off.

app-network-step-client = 1. 將用戶端放到舊電腦上
app-network-step-client-hint = WinAppMigrator Client 在應用程式目前所在的電腦上執行，需要 Windows 7 或更新版本。它只允許這台電腦憑它顯示的配對碼，以加密方式讀取該電腦。
# { $drive } is a USB drive, such as E:\.
app-network-save-to-usb = 儲存到 USB 磁碟機 { $drive }
# { $drive } is a USB drive, such as E:\, and { $name } the name it was given, such as BACKUP.
app-network-save-to-named-usb = 儲存到 USB 磁碟機 { $drive }（{ $name }）
app-network-save-to-folder = 儲存到資料夾…
app-network-look-for-usb-again = 重新尋找 USB 磁碟機
app-network-no-usb = 未插入 USB 磁碟機。請插入一個，或儲存到可從舊電腦存取的資料夾，例如網路上的共用資料夾。
# { $path } is where the client was saved, such as E:\WinAppMigrator Client.exe.
app-network-client-saved = 已儲存為 { $path }。在舊電腦上，請關閉您要移轉的應用程式，然後按兩下該檔案並允許它進行變更。它會顯示配對碼。
app-network-step-pick = 2. 選擇舊電腦
app-network-looking = 正在網路上尋找執行用戶端的電腦…
app-network-type-name = 或輸入其名稱或位址
# An example, shown in the empty field: a PC's name, or its network address.
app-network-type-name-example = OLD-PC 或 192.168.1.20
app-network-step-code = 3. 輸入它顯示的配對碼
app-network-code = 配對碼
app-network-connect = 連線
app-network-pairing = 正在配對…
# Why pairing did not work, shown on its own.
app-network-pairing-stopped = 配對在完成前已停止
app-network-wrong-code = 這不是舊電腦顯示的配對碼。請檢查後再試一次。
# { $error } is what went wrong.
app-network-not-reached = 無法連線到舊電腦：{ $error }
app-network-pick-folder-title = 選擇要存放 WinAppMigrator Client 的資料夾
# { $path } is the folder, and { $error } what Windows said went wrong.
app-client-could-not-save = 無法將用戶端儲存到 { $path }：{ $error }
# scripts\build-client.ps1 is a file name.
app-client-not-carried = 此版本的 WinAppMigrator 未包含用戶端。請使用 scripts\build-client.ps1 建置用戶端，然後重新建置應用程式。
# { $error } is what Windows said went wrong.
app-client-could-not-save-note = 無法儲存用戶端旁邊的說明檔：{ $error }
# A note saved as a text file beside the client, for whoever runs it on the old PC. Keep its lines short, as it is read in
# Notepad. "WinAppMigrator Client.exe" is a file name. "Network PC..." names the Scan Source button: use its words in your
# language.
app-client-note =
    WinAppMigrator Client
    
    請在您要移出應用程式的電腦上執行此程式：Windows 7、8、10 或 11。
    
    1. 關閉您要移轉的應用程式。
    2. 按兩下「WinAppMigrator Client.exe」，並允許它進行變更。
    3. 它會顯示 6 位數的配對碼。在新電腦的 WinAppMigrator 中選擇
       「網路電腦...」，選取這台電腦，然後輸入配對碼。
    4. 新電腦完成後，請在用戶端的視窗中按 Enter，或將視窗關閉。
    
    它只會讀取資料。執行期間，它會保留 Windows 磁碟機的快照，並讓
    它自己的兩個連接埠透過 Windows 防火牆接收來自區域網路的連線；
    停止時會將兩者都移除。透過網路傳輸的內容皆經過加密，只有
    擁有配對碼的電腦才能讀取任何內容。
