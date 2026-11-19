# Words the core writes for people while it builds a package from an installed app: what a package records for an
# installer to show later, errors shown in the app, the list of what goes in and what is left out, opening a disk
# image or another PC over the network, and what an installer says when the package it carries cannot be read.
# Message names start with core-.


## Scanning an app, before a package is built. Shown in the app when a scan cannot go on.

# { $path } is the app's program, such as D:\Program Files\Acme\Acme.exe, on a drive taken from another PC.
core-scan-program-not-on-the-drive = { $path } 經過連結，或不在連接的磁碟機上，因此未讀取。
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-not-on-this-pc = { $path } 不在這台電腦的磁碟上，或其名稱會開啟其他項目，因此未讀取。
# { $path } is the app's program.
core-scan-program-someone-elses = { $path } 位於其他帳戶的資料夾中或不屬於此帳戶，且此應用程式本身的記錄並未說明它屬於該處，因此未讀取。
# { $app_name } is the app's name, as the list of apps calls it.
core-scan-nothing-to-package = 找不到 { $app_name } 本身的程式，因此沒有可放入安裝程式的內容。驅動程式、執行階段和增益集通常是這樣：請在新電腦上從製造商或 Windows Update 安裝驅動程式，並將執行階段或增益集與使用它的應用程式一起安裝。
# { $app_name } is the app's name, as the list of apps calls it. A web app is a website installed as an app, with
# a window and a Start menu entry of its own.
core-scan-web-app = { $app_name } 是瀏覽器從網站安裝的 Web 應用程式。它沒有自己的程式，因此沒有可放入安裝程式的內容。請在新電腦上用同一個瀏覽器開啟該網站，並從那裡安裝這個應用程式。
# { $path } is the program the app's own record names, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-missing = 此應用程式的記錄將 { $path } 列為應用程式的程式，但它已不在那裡。此應用程式可能已被移動或部分移除：在這台電腦上修復或重新安裝此應用程式，應該就能復原。
# { $path } is the file that was picked.
core-scan-not-a-file = 掃描目標不是檔案：{ $path }
# Names something left out of a package, in a list of what was left out and why. { $name } is the variable's name,
# such as ACME_LICENSE_KEY.
core-scan-environment-variable-item = 環境變數 { $name }

## Folders and parts that might also be the app's, offered in a list to tick. Each says why it is offered.

core-near-starts-with-app-name = 其名稱以此應用程式的名稱開頭。
core-near-has-app-name = 其名稱包含此應用程式的名稱。
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-starts-with-publisher = 其名稱以發行者的名稱 { $publisher } 開頭。
# { $folder } is a folder's name, such as Program Files.
core-near-hidden-part-beside = 應用程式清單中未顯示的元件，與此應用程式一起安裝在 { $folder } 中。
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-hidden-part-publisher = 應用程式清單中未顯示的元件，來自相同的發行者 { $publisher }。

## What a scan found, as a person chooses from it: the folders that go in, and what is left out, and why

# Why a folder cannot be left out, beside its number of files.
core-footprint-locked-app-folder = 這些一律與主程式一起包含
core-footprint-locked-main-program = 包含主程式，一律會包含在內
core-footprint-locked-needed = 包含主程式啟動所需的項目
core-footprint-locked-font = 隨其字型一起包含，列在「與 Windows 的整合」下
# Where a folder is. "AppData" and "ProgramData" are the names of Windows folders; keep them as they are.
core-footprint-place-roaming = 您設定檔中的設定（AppData）
core-footprint-place-local = 您設定檔中這台電腦專用的設定（AppData）
core-footprint-place-program-data = 這台電腦上所有人共用（ProgramData）
core-footprint-place-documents = 在您的「文件」資料夾中
core-footprint-place-saved-games = 在您的「儲存的遊戲」資料夾中
core-footprint-place-public-documents = 在「公用文件」資料夾中
core-footprint-place-app-folder = 在應用程式的資料夾中
# { $folder } is a folder, such as C:\Users\Sam\AppData\Roaming.
core-footprint-place-in = 在 { $folder } 中
# A row for the files directly in the app's folder. { $folder } is that folder's name, such as Acme.
core-footprint-files-directly-in = 直接位於 { $folder } 中的檔案
# Headings over what is left out, and why each item is.
core-footprint-licenses-left-out = 未包含在此安裝程式中，因為「授權與啟用資料」選項已關閉
core-footprint-font-stays-behind = 它會留在原處。
core-footprint-drivers-left-out = 屬於 Windows 或廠商的驅動程式無法轉散發，請在另一台電腦上重新安裝
core-footprint-driver-reinstall = 請從其廠商自己的套件重新安裝。
# Why something else is left out along with what the person left out. { $service } is a service's name, such as AcmeSync.
core-footprint-needs-left-out = 它需要已排除的 { $service }
core-footprint-told-to-use-left-out = 它被設定為使用您排除之資料夾中的項目
core-footprint-file-in-left-out = 它的檔案位於您排除的資料夾中
core-footprint-program-in-left-out = 它所屬的程式位於您排除的資料夾中
core-footprint-names-left-out = 它指向您排除的資料夾
# Why something is left out, as the installer tells whoever installs it.
core-footprint-builder-left-it-out = 建置此安裝程式的人已將它排除
# Something the person left out, by kind and name, such as "Service Acme Updater". { $kind } is the kind, such as
# Service or Scheduled Task; { $title } is its name.
core-footprint-left-out-item = { $kind } { $title }

## Building a package and an installer from a scan. Shown in the app when a build cannot go on.

core-package-scan-of-another-app = 此掃描的對象與要建置的應用程式不同，因此未建置任何內容
core-package-scan-brought-in-more = 此掃描帶入了此建置未要求的資料，因此未建置任何內容
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-package-program-gone = { $path } 已不存在，因此未建置任何內容。請重新檢查應用程式。
core-package-needs-stub = 自解壓縮封存需要安裝程式空殼
# { $reason } is what went wrong.
core-package-staged-file-failed = 暫存資料夾中的檔案無法加入套件：{ $reason }
# { $shortfalls } says which drives are short of room and by how much, in sentences of its own.
core-package-not-enough-room = 空間不足，無法建置此安裝程式，因此未讀取或寫入任何內容。{ $shortfalls }。請在該處釋放一些空間，或在其他磁碟機上選擇輸出資料夾。
# { $path } is a file's path, on a drive taken from another PC.
core-package-not-read-off-the-drive = { $path } 經過連結，或不在連接的磁碟機上，因此未讀取
# { $path } is a file's path.
core-package-not-read-not-local = { $path } 不在這台電腦的磁碟上，因此未讀取
# { $path } is a file's path.
core-package-not-read-other-account = { $path } 通往其他帳戶的資料夾，因此未讀取
# { $path } is a file's path, as the PC it came from called it.
core-package-not-on-the-drive = { $path } 不在連接的磁碟機上，因此未讀取
# { $path } is a file's path.
core-package-backup-copy-no-parent = 無法以備份方式複製沒有上層目錄的檔案：{ $path }
# { $path } is a file's path.
core-package-backup-copy-destination-no-parent = 備份複製的目的地沒有上層目錄：{ $path }
# robocopy is the name of a Windows command; keep it as it is. { $code } is how it ended, such as Some(8).
core-package-robocopy-failed = robocopy 備份模式複製失敗（結束代碼 { $code }）
# Stands for a folder where a file's name would be, as in "I/O failed for temporary package folder: …".
core-package-temporary-folder = 暫存套件資料夾
# Stands for the installer being written where a file's name would be, as in "I/O failed for the installer: …".
core-package-the-installer = 安裝程式
# { $path } is a file's path, or its name in the package. { $error } is what went wrong with it.
core-package-path-error = { $path }：{ $error }
# { $name } is a file's name in the package, such as app/bin/App.exe.
core-package-same-place = 另一個檔案在套件中的位置相同：{ $name }
# { $name } is a file's name in the package.
core-package-changed-in-protected-folder = 建立套件期間，{ $name } 在受保護的資料夾中遭到變更
core-package-worker-died = 壓縮檔案的執行緒意外停止，因此未寫入任何安裝程式
# { $path } is a file's path.
core-package-link-not-read = { $path } 是連結，因此未讀取
# { $path } is a file's path. { $error } is what went wrong.
core-package-not-compressed = 無法壓縮 { $path }：{ $error }
# { $path } is a file's path. { $reason } is what went wrong.
core-package-not-read-back = 無法讀回 { $path }：{ $reason }

## Where an app's files are, and where an installer may write them. Shown when a path is refused.

# { $path } is a folder, such as C:\Program Files\Acme.
core-discovery-root-not-full-path = 要寫入的資料夾必須是完整路徑：{ $path }
# { $path } is a folder's path.
core-discovery-root-not-a-folder = 要寫入的資料夾不是資料夾：{ $path }
# { $path } is a file's path.
core-discovery-escapes-root = 路徑超出預定的根目錄：{ $path }
# { $path } is the link's path.
core-discovery-refusing-to-write-through-link = 拒絕透過連結寫入：{ $path }
# { $path } is the link's path.
core-discovery-refusing-link = 拒絕經過連結：{ $path }
# { $error } is what Windows said went wrong.
core-discovery-refusing-link-because = 拒絕經過連結：{ $error }
# { $path } is the link's path.
core-discovery-link-to-somewhere-else = { $path } 是指向其他位置的連結，其他人可能可以變更它的指向。請使用它所指向的位置
# { $path } is a folder's path.
core-discovery-no-parent-folder = { $path } 沒有上層資料夾
# { $path } is the install location as it was given. Keep the quotes, or use your language's own.
core-discovery-install-location-not-full-path = 安裝位置必須是完整路徑：「{ $path }」
# { $path } is the install location.
core-discovery-install-location-a-link = 安裝位置是指向其他位置的連結：{ $path }
# { $path } is a file's path.
core-discovery-device-name = 路徑使用了保留給 Windows 裝置的名稱：{ $path }
# { $path } is a file's path.
core-discovery-device-name-on-older-windows = { $path } 的名稱在此版本的 Windows 中會被當成裝置而非檔案開啟，因此未寫入任何內容。它可以安裝在 Windows 11 上
# { $path } is a file's path.
core-discovery-unsafe-path = 不安全的路徑：{ $path }
# { $path } is a file's path.
core-discovery-no-package-path = 檔案沒有套件相對路徑：{ $path }

## Writing an installer, and reading the package inside one. Most of these are shown by an installer, on the PC it
## runs on, when it cannot read what it carries. "The package" is the app's files and settings that an installer
## carries; "the stub" is the program part of an installer, before the package; "the data file" is the .wamdata file
## that holds the package of a very large installer, beside it.

# Stands for the package where a file's name would be, as in "I/O failed for installer package: …".
core-archive-installer-package = 安裝程式套件
# Stands for the installer program built into WinAppMigrator, where a file's name would be.
core-archive-embedded-stub = 內嵌的安裝程式空殼
# { $index } is the number of a file in the package. { $reason } is what went wrong.
core-archive-entry-unreadable = 無法讀取套件的第 { $index } 個項目：{ $reason }
# { $name } is a file's name inside the package, such as app/bin/App.exe.
core-archive-entry-unsafe-path = zip 項目的路徑不安全：{ $name }
# { $name } is a file's name inside the package.
core-archive-holds-more = { $name } 包含的內容比其標示的多
# { $name } is a file's name inside the package.
core-archive-name-too-long = 此名稱對 zip 而言太長：{ $name }
core-archive-changed-while-copied = 套件在附加到安裝程式空殼後方的過程中發生變更，因此未寫入任何安裝程式
core-archive-stub-not-a-program = 此安裝程式在套件之前的部分不是程式
core-archive-damaged = 此安裝程式已損毀，或在建置後遭到變更，因此未從其套件讀取任何內容。請從建置它的位置重新複製。
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-missing = 此安裝程式將其套件存放在同一資料夾中名為 { $name } 的檔案內，但該檔案不存在。這兩個檔案必須放在一起：請從建置安裝程式的位置將 { $name } 複製到這裡，然後再次執行。
# { $name } is the data file's name, such as App_installer.wamdata. { $error } is what Windows said went wrong.
core-archive-data-file-unopened = 無法開啟此安裝程式的資料檔 { $name }：{ $error }。這兩個檔案必須放在一起，且位於這台電腦可以讀取的資料夾中。
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-a-link = { $name } 是連結或資料夾，而此安裝程式的資料檔必須是獨立的檔案，且就在安裝程式旁邊，因此未從中讀取任何內容。請將實際的檔案複製到這裡。
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-wrong = { $name } 不是此安裝程式的資料檔，或已損毀，或在建置後遭到變更，因此未從中讀取任何內容。請從建置它們的位置重新複製這兩個檔案。
core-archive-stub-signed = 安裝程式空殼已簽署；它必須保持未簽署，因此請改為簽署完成的安裝程式
# { $size } is a size, such as 4.2 GB.
core-archive-too-big-to-run = 安裝程式將為 { $size }，而 Windows 無法執行 4 GB 以上的執行檔；未寫入任何內容
core-archive-outside-signature = 安裝程式在其簽章之外含有資料，可能已遭竄改
core-archive-footer-length = 自解壓縮封存的頁尾長度無效
core-archive-seek-outside = 定位超出套件範圍
# { $path } is a file's path.
core-archive-unsafe-package-path = 不安全的套件路徑：{ $path }
# { $path } is a file's path.
core-archive-empty-package-path = 空白的套件路徑：{ $path }

## Opening a disk image of another PC (a .vhd or .vhdx file) to read apps from it

# { $path } is the disk image file, such as D:\Backups\OldPC.vhdx.
core-image-not-attachable = { $path } 不是 Windows 可以掛接的磁碟映像（.vhd 或 .vhdx）
core-image-needs-administrator = 掛接磁碟映像需要系統管理員權限
# { $path } is the disk image file the person picked.
core-image-not-a-file = { $path } 不是檔案
# { $path } is the disk image file. "Edit Disk" is the name of a Hyper-V Manager command.
core-image-differencing = { $path } 是差異磁碟，只包含從其來源磁碟建立以來的變更，而 Windows 也會開啟該來源磁碟，無論它記錄的位置在哪裡。請先使用 Hyper-V 的「編輯磁碟」將它合併到該磁碟，然後選取該磁碟
# { $path } is the disk image file.
core-image-unreadable = { $path } 不是可讀取的 .vhd 或 .vhdx
# ProgramData is the name of a Windows folder; keep it as it is.
core-image-no-program-data = 找不到這台電腦的 ProgramData 資料夾
# The reason given in core-image-could-not-attach when Windows already has the image open.
core-image-already-attached = Windows 已掛接它，而掛接它的程式可能正在寫入。請先取消掛接，或將其磁碟機選為連接的磁碟機
# { $path } is the disk image file. { $reason } is why, such as core-image-already-attached or what Windows said.
core-image-could-not-attach = 無法掛接 { $path }：{ $reason }
# { $path } is the disk image file.
core-image-no-windows-volume = { $path } 中沒有可讀取且安裝了 Windows 的磁碟區。以 BitLocker 加密的磁碟區必須先解除鎖定。
# Given as the reason when Windows failed without saying anything.
core-image-windows-did-not-say = Windows 未說明原因

## Reading another PC over the network, through the client running on it. Shown as why something on that PC
## could not be read. "The client" is WinAppMigrator Client, the small program on the other PC.

core-remote-too-slow = 另一台電腦的回應時間過長
core-remote-closed = 與另一台電腦的連線已關閉
core-remote-failed = 與另一台電腦的連線失敗
core-remote-answered-something-else = 用戶端回應了其他內容
# { $name } is the PC's name or address as the person typed it, such as OLDPC or 192.168.1.20.
core-remote-not-found = 在網路上找不到 { $name }
core-remote-did-not-say-what-it-is = 用戶端未說明它是什麼
core-remote-ordinary-names-only = 只能查詢另一台電腦根目錄下、使用一般名稱的路徑
core-remote-more-but-none = 另一台電腦表示某個資料夾中還有更多內容，卻沒有提供任何內容
# { $count } is the most things one folder may hold, such as 250,000.
core-remote-too-many-listed = 另一台電腦在一個資料夾中列出了超過 { $count } 個項目
core-remote-sent-too-much = 另一台電腦傳送的檔案內容超過要求的量
core-remote-name-not-text = 無法查詢不是文字的名稱
core-remote-below-root-only = 只能查詢另一台電腦根目錄下的路徑
core-remote-file-ended-early = 另一台電腦的檔案提前結束
core-remote-link-not-followed = 不會追蹤另一台電腦上的連結
core-remote-only-files-opened = 在另一台電腦上只能開啟檔案
core-remote-listing-said-nothing = 清單中沒有提到它
core-remote-change-time-unknown = 不知道它的變更時間
core-remote-seek-before-start = 定位到開頭之前的位置
# { $size } is how many bytes a file is; { $limit } is the most that were expected. Both are numbers.
core-remote-too-large =
    { $size ->
       *[other] 它有 { $size } 個位元組，但預期不會超過 { $limit } 個
    }

## Checking that a download is signed by Microsoft. Each is given as the reason a check failed.

# { $error } is what Windows said went wrong.
core-signature-no-powershell = 無法啟動 PowerShell：{ $error }
# { $error } is what went wrong reading PowerShell's answer.
core-signature-unreadable = 無法讀取簽章：{ $error }
# { $signer } is who signed the file, such as Contoso Ltd.
core-signature-not-microsoft = 它是由 { $signer } 簽署，而不是 Microsoft
core-signature-not-microsoft-unnamed = 它是由其他人簽署，而不是 Microsoft
# { $status } is Windows' own word for the signature's state, such as HashMismatch or NotSigned, which is not translated.
core-signature-status = 其簽章狀態為 { $status }

## Warnings a package records when it is built from an app, which an installer shows later, maybe on another PC, in
## that PC's language. "This app" is the app being packaged.

core-warning-build-program-needs-unreadable = 無法讀取此應用程式對 Windows 的需求，因此不會檢查它能否在另一台電腦上執行。
core-warning-build-external-manifest = 偵測到外部應用程式資訊清單；並存相依性的解讀將延後處理。
core-warning-build-non-exe = 所選檔案的副檔名不是 .exe；將繼續進行 PE 掃描。
core-warning-build-stand-in-program = 找不到一般的應用程式執行檔，因此以此檔案代替。架構和相依性結果描述的是此檔案，不一定能描述該應用程式。
core-warning-build-other-apps-files-unknown = 無法讀取 Windows Installer 中其他應用程式的所有資訊，因此此應用程式記錄中的資料夾可能包含其他應用程式的檔案，而這裡無從得知。
core-warning-build-app-folder-has-another-apps-file = Windows Installer 表示另一個應用程式在此應用程式的程式所在資料夾中有檔案，而該檔案會一併帶過去，因為該資料夾會整個納入。
# "App Paths" is the name of a Windows registry key; keep it as it is.
core-warning-build-app-paths-folder-left-out = 此應用程式的 App Paths 項目表示它會在不屬於它的資料夾中尋找，因此該資料夾不會一併帶過去。在沒有該處內容的電腦上，它可能無法啟動。
core-warning-build-uninstaller-folder-left-out = 此應用程式的解除安裝程式所在的資料夾已排除，因為 Windows Installer 表示另一個應用程式在該處有檔案。
# { $why } is never shown: it chooses the reason. Translate each reason as the end of the sentence.
core-warning-build-recorded-folder-left-out =
    此應用程式記錄中的某個項目指向已排除的資料夾，因為{ $why ->
        [unreadable] 它的路徑會通往磁碟機以外的位置或經過連結，因此無法安全地讀取
        [missing] 它不存在
        [link] 它經過連結
        [short-name] 它的寫法與其實際名稱不同
        [another-account] 它位於其他帳戶的資料夾中
        [one-account] 此應用程式是為單一帳戶安裝的，而這不是該帳戶的資料夾
        [shared] 它存放共用的內容，或屬於 Windows 的一部分
        [generic-name] 從它的名稱看不出屬於誰
        [publisher] 它僅以發行者命名，可能存放該發行者的其他產品
        [holds-the-app] 它包含此應用程式本身的資料夾，且可能包含更多內容
        [another-apps-name] 它使用另一個應用程式的名稱
        [another-apps-record] 它的安裝記錄屬於另一個應用程式
        [in-another-apps-folder] 它位於另一個應用程式的資料夾中
       *[another-apps] 它屬於另一個應用程式
    }。
core-warning-build-app-folder-holds-another-app = 這台電腦上的另一個應用程式記錄為位於此應用程式的資料夾內，並會隨之一併帶過去，因為該資料夾會整個納入。
# "Program Files (x86)" is the name of a Windows folder; keep it as Windows calls it.
core-warning-build-from-32-bit-windows = 此應用程式原本位於 32 位元 Windows 上。其登錄機碼已依照 64 位元 Windows 存放 32 位元應用程式的方式配置，而安裝程式會為它提供 Program Files (x86)。
# { $dll } is a file's name, such as MSVCP140D.dll.
core-warning-build-debug-runtime = 有程式需要 { $dll }，這是 Visual C++ 執行階段的偵錯組建。Microsoft 沒有提供這類組建的安裝程式，而且可能不允許轉散發，因此該程式無法在沒有 Visual Studio 的電腦上執行。
# { $program } is a program's file name, such as Tool.exe.
core-warning-build-single-file-program = { $program } 是封裝成單一檔案的 .NET 程式，無法看出它需要哪個 .NET，因此未為它規劃任何 .NET。如果它需要 .NET，在沒有該 .NET 的電腦上將無法啟動。
# { $count } is how many of the app's programs.
core-warning-build-programs-not-inspected =
    { $count ->
       *[other] 此應用程式有 { $count } 個程式未檢查其所需的執行階段，因為程式數量太多或檔案太大。
    }
# { $runtime } is a runtime's name, such as Java Runtime Environment.
core-warning-build-runtime-no-installer = 此應用程式需要 { $runtime }，但這裡沒有它的安裝程式，因此不會執行任何安裝程式。請在另一台電腦上自行安裝。
# { $why } is why something was left out, such as "its name says what it holds".
core-warning-build-left-out-of-package = 已從套件中排除：{ $why }。
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-missing-dependency = 無法解析相依性：{ $name }
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-dependency-only-here = { $name } 只能透過這台電腦的搜尋路徑找到，且不屬於此應用程式，因此不會一併帶過去。在沒有它的電腦上，此應用程式可能無法啟動。
core-warning-build-driver-detected = 偵測到驅動程式或核心元件。應用程式擁有的驅動程式會隨附並重新註冊；廠商或內建的驅動程式不會轉散發，必須從其已簽署的廠商套件重新安裝。
core-warning-build-near-no-longer-offered = 在「可能也屬於此應用程式的項目」清單中勾選的某個項目已不在清單上，因此未新增。請重新查看清單。
# { $app_name } is the name of a part of the app, as the list of apps calls it. { $why } is never shown: it chooses the
# reason. Translate each reason as the end of the sentence.
core-warning-build-near-part-left-out =
    { $app_name } 宣稱屬於它自己的部分內容已排除：{ $why ->
        [unreadable] 它的路徑會通往磁碟機以外的位置或經過連結，因此無法安全地讀取
        [missing] 它不存在
        [link] 它經過連結
        [short-name] 它的寫法與其實際名稱不同
        [another-account] 它位於其他帳戶的資料夾中
        [one-account] 此應用程式是為單一帳戶安裝的，而這不是該帳戶的資料夾
        [shared] 它存放共用的內容，或屬於 Windows 的一部分
        [generic-name] 從它的名稱看不出屬於誰
        [publisher] 它僅以發行者命名，可能存放該發行者的其他產品
        [holds-the-app] 它包含此應用程式本身的資料夾，且可能包含更多內容
        [another-apps-name] 它使用另一個應用程式的名稱
        [another-apps-record] 它的安裝記錄屬於另一個應用程式
        [in-another-apps-folder] 它位於另一個應用程式的資料夾中
       *[another-apps] 它屬於另一個應用程式
    }。
# { $app_name } is the name of a part of the app, as the list of apps calls it.
core-warning-build-near-keys-left-out = { $app_name } 的安裝記錄所列的登錄機碼已排除，因為該記錄也指向屬於另一個應用程式的項目。
# "Windows keeps what an app wrote for one account" is Windows' VirtualStore folder.
core-warning-build-virtual-store-program = 有程式位於 Windows 存放應用程式為單一帳戶寫入之內容的資料夾中。Windows 從不會將程式放在該處，因此是其他程式放的，已將它排除。
# { $path } is a file or folder, such as C:\Users\Sam\AppData\Roaming\Acme.
core-warning-build-file-left-out = { $path } 已排除：它經過連接之磁碟機上的連結，或屬於其他帳戶，而不是要包含其資料的帳戶。
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as D:\Sam\Documents.
core-warning-build-user-folder-not-on-drive = { $user } 的 { $folder } 資料夾位於 { $path }，不在此磁碟機上，因此未包含其中的內容。
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as \\server\home\Sam\Documents.
core-warning-build-user-folder-not-on-this-pc = { $user } 的 { $folder } 資料夾位於 { $path }，不在這台電腦的磁碟上，因此未包含其中的內容。
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-base-unreadable = 無法讀取 { $path }（{ $error }）；未包含存放在該處的應用程式資料。
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-folder-unreadable = 無法讀取 { $path }（{ $error }）；可能缺少該處的巢狀應用程式資料。
# { $error } is what went wrong.
core-warning-build-entry-unreadable = 無法讀取檔案系統項目：{ $error }
# { $count } is how many links. { $names } lists some of them. { $more } is how many more there are, which are not
# named; when it is 0, the sentence ends after the list.
core-warning-build-links-not-followed =
    { $count ->
       *[other] 此資料夾中有 { $count } 個連結或會開啟其他項目的名稱未被追蹤，因此其指向的內容不在安裝程式中：{ $names }
    }{ $more ->
        [0] 。
       *[other] ，以及其他 { $more } 個。
    }
# { $count } is how many folders. { $depth } is how many levels down a folder can be. { $names } lists some of them.
# { $more } is how many more there are, which are not named; when it is 0, the sentence ends after the list.
core-warning-build-folders-too-deep =
    { $count ->
       *[other] 有 { $count } 個資料夾的層級超過 { $depth } 層，因此其中的內容不在安裝程式中：{ $names }
    }{ $more ->
        [0] 。
       *[other] ，以及其他 { $more } 個。
    }
# { $error } is what went wrong.
core-warning-build-metadata-unreadable = 無法讀取檔案中繼資料：{ $error }
core-warning-build-reserved-name = 此檔案的名稱保留給 Windows 裝置使用，因此未複製。
# { $error } is what went wrong.
core-warning-build-hash-failed = 無法計算檔案雜湊：{ $error }
core-warning-build-cloud-placeholder = 僅限線上的雲端檔案（例如 OneDrive 檔案隨選）沒有本機資料；請在備份前將它設為一律保留在此裝置上，或將它下載。已回報但未複製。
# { $file_name } is a driver's file name, such as acmeflt.sys.
core-warning-build-driver-file = 驅動程式/核心檔案「{ $file_name }」不會以一般檔案的方式複製。如果它支援此應用程式已註冊的驅動程式服務，安裝程式會隨附它並自動重新註冊驅動程式；否則請使用廠商已簽署的驅動程式套件重新安裝。
core-warning-build-file-gone = 檢查應用程式時此項目還在，但現在已不存在，因此不在安裝程式中。
# { $error } is what went wrong.
core-warning-build-file-copy-failed = 無法將檔案複製到套件中：{ $error }
# { $count } is how many files. { $names } lists them.
core-warning-build-not-looked-through =
    { $count ->
       *[other] 此套件含有無法檢查是否透露來源的檔案：{ $names }。這些檔案的內容可能會透露來源。
    }
# { $count } is the most empty folders an installer makes, such as 10,000.
core-warning-build-empty-folders-truncated = 此應用程式有超過 { $count } 個空資料夾，只會重新建立其中的 { $count } 個。
core-warning-build-credential-by-now = 已從套件中排除：它現在看起來像認證。
core-warning-build-no-package-path = 可複製的檔案沒有套件相對路徑。
# { $file_name } is the runtime installer's file name, such as vc_redist.x64.exe. { $error } is what went wrong.
core-warning-build-prerequisite-path-invalid = 無法放置隨附的必要元件 { $file_name }：{ $error }
# { $runtime } is a runtime's name, such as Microsoft Visual C++ 2015+ Redistributable (x64). { $error } is what went
# wrong. winget is the name of a Windows command; keep it as it is.
core-warning-build-prerequisite-download-failed = 無法下載 { $runtime }（{ $error }）；安裝程式會在目標電腦上嘗試使用 winget。
# { $runtime } is a runtime's name. { $reason } is why the check failed, such as "it is signed by Contoso Ltd, not
# Microsoft".
core-warning-build-prerequisite-signature-failed = 下載的 { $runtime } 未通過簽章檢查，因此未隨附：{ $reason }。安裝程式會在目標電腦上嘗試使用 winget。
# { $runtime } is a runtime's name. { $error } is what went wrong.
core-warning-build-prerequisite-hash-failed = 已下載 { $runtime }，但無法計算其雜湊（{ $error }）；安裝程式會改為在目標電腦上嘗試使用 winget。
# { $service } is a driver's service name, such as AcmeFlt. { $error } is what went wrong.
core-warning-build-driver-copy-failed = 無法隨附「{ $service }」的驅動程式映像：{ $error }
core-warning-build-license-not-copied = 無法複製授權資料（遺失或無法讀取）；僅記錄為中繼資料。
core-warning-build-license-file-credential = 有授權檔案看起來像認證，已將它排除。
# { $error } is what went wrong.
core-warning-build-license-store-walk-failed = 無法列舉授權存放區的部分內容：{ $error }
core-warning-build-license-store-credential = 授權存放區中有檔案看起來像認證，已將它排除。
# { $error } is what went wrong.
core-warning-build-license-store-file-skipped = 已略過授權存放區檔案：{ $error }
