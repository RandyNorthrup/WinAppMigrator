# Words the core says about what an install may do: why an install location or something in a package is refused,
# the installer's list of what an app sets up beyond its files, the checks of a package and of an install, and whether
# there is room for either. Message names start with core-.
#
# Many messages here are a reason, the second half of a sentence that starts "... was left out:" or "that install
# location cannot be used:", so they start with a small letter and have no full stop.


## Why an install location cannot be used. Shown in red under the Install Location field.

core-review-location-state-root = 這是安裝存放其記錄的位置
core-review-location-has-files = 其中已有檔案，且此處沒有安裝記錄，因此請使用空白資料夾
core-policy-location-not-full-path = 它必須是完整路徑
core-policy-location-parent-step = 它不得包含「..」
core-policy-location-not-plain = 它必須是這台電腦磁碟機上的一般路徑
core-policy-location-whole-drive = 它不能是整個磁碟機
core-policy-location-windows = 它不能是 Windows 資料夾
core-policy-location-runs-by-itself = 它是屬於 Windows 的資料夾，或其內容會自行執行或載入
core-policy-location-shared = 它並列存放了其他項目，因此應用程式需要其中的子資料夾

## Whether an app can run on this PC at all, said before anything is installed.

# { $reason } is why, as the second half of a sentence, such as "App.exe is built for Arm64, and this PC is x64, so
# Windows would not start it".
core-review-cannot-run-here = 此應用程式無法在這台電腦上執行，因此未做任何變更：{ $reason }
core-policy-architecture-x86 = 32 位元 x86
core-policy-architecture-x64 = x64
core-policy-architecture-arm64 = Arm64
core-policy-architecture-arm32 = 32 位元 Arm
core-policy-architecture-unknown = 未知的架構
# { $code } is the number Windows gives a kind of processor, such as 0x01c4.
core-policy-machine-type = 機器類型 { $code }
# { $names } is a list of program names, such as "a.exe, b.exe, c.exe", and { $count } how many more there are.
core-policy-names-and-more = { $names }，以及其他 { $count } 個
core-policy-fit-programs-not-said = 此安裝程式未說明其程式的需求，因此未檢查它們能否在這台電腦上執行。
core-policy-fit-architecture-not-told = 無法判斷這台電腦的架構，因此未檢查此應用程式的程式能否在其上執行。
core-policy-fit-main-not-said = 此安裝程式未說明其主程式的需求，因此這裡的說明是根據它的其他內容。
# { $driver } is a driver's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-driver = 其驅動程式 { $driver } 是為 { $built_for } 建置的，而這台電腦是 { $here }。驅動程式必須與 Windows 完全相符，因此此應用程式無法在這裡執行
# { $program } is a program's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-program = { $program } 是為 { $built_for } 建置的，而這台電腦是 { $here }，因此 Windows 不會啟動它
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-none-start = 它的程式都無法在這台電腦（{ $here }）上啟動：{ $programs }
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-some-other-architecture = 此應用程式的部分程式是為其他架構建置的，無法在這台電腦（{ $here }）上啟動：{ $programs }。應用程式的其餘部分可以啟動，因此這可能不會有影響。
# { $here } is a kind of processor, such as "Arm64". { $programs } is a list of program names.
core-policy-fit-emulated = 這台電腦是 { $here }，因此 Windows 會透過模擬執行這些程式：{ $programs }。大部分程式這樣執行都沒有問題，但有些不行。
# { $program } is a program's file name. { $needs } and { $runs } are versions of Windows, such as 10.0 and 6.1.
core-policy-fit-needs-newer = { $program } 表示它需要 Windows { $needs } 或更新版本，而這台電腦執行的是 { $runs }，因此 Windows 不會啟動它
# { $programs } is a list of program names.
core-policy-fit-some-need-newer = 此應用程式的部分程式表示它們需要比這更新的 Windows，因此無法在這裡啟動：{ $programs }。
# { $from_build } and { $this_build } are Windows build numbers, such as 22631, written as Windows writes them.
core-policy-fit-older-build = 此應用程式原本位於 Windows 組建 { $from_build }，而這台電腦執行的是組建 { $this_build }。其程式表示可以在此 Windows 上啟動，因此可能可以運作，但它在原電腦上依賴的項目可能不在這裡。
core-policy-fit-build-not-told = 無法判斷此 Windows 的組建，因此未與此應用程式原本所在的組建比較。

## Room on a drive for a build or an install.

# Each of these ends the sentence core-space-shortfall, after "free for".
core-space-for-unpacking = 解壓縮套件
core-space-for-install-record = 安裝記錄
core-space-for-app-files = 應用程式的檔案
core-space-for-what-is-replaced = 保留被取代的項目
core-space-for-build-work = 建置作業的工作檔案
core-space-for-installer = 安裝程式
core-space-for-package = 套件
core-space-for-unpacking-and-installing = 解壓縮並安裝套件
# What goes between two of those, as in "unpacking the package and the app's files". Keep the quotes: they keep the
# spaces.
core-space-purposes-joiner = 、
# { $drive } is a drive, such as C:, or a folder. { $needed } and { $free } are sizes, such as 1.2 GB. { $purposes } is
# what the room is for, such as "unpacking the package and the app's files".
core-space-shortfall = { $drive } 最多需要 { $needed } 的可用空間，用於{ $purposes }，目前只有 { $free }
# { $drive } is a drive, such as C:, or a folder. { $needed } is a size, such as 1.2 GB.
core-space-not-told = { $drive } 未提供其可用空間，因此未檢查它所需的 { $needed } 是否放得下
# One drive in a list of where room is needed. { $needed } and { $free } are sizes, and { $drive } a drive, such as C:.
core-space-summary = { $drive } 上最多 { $needed }（可用 { $free }）
# { $needed } is a size, and { $drive } a drive, such as C:.
core-space-summary-not-told = { $drive } 上最多 { $needed }（無法得知可用空間）
# What goes between two sentences of core-space-shortfall inside core-review-no-room. Keep the quotes: they keep the
# space.
core-review-sentence-joiner = 。
# { $shortfalls } is one or more sentences of core-space-shortfall.
core-review-no-room = 空間不足，因此未做任何變更。{ $shortfalls }。請在該處釋放一些空間，或選擇其他磁碟機上的安裝位置。

## The installer's list of what an app sets up beyond its files: what each kind is called.

core-review-kind-service = 服務
core-review-kind-driver = 驅動程式
core-review-kind-scheduled-task = 排定的工作
core-review-kind-autostart = 自動啟動
core-review-kind-shortcut = 捷徑
core-review-kind-path = PATH
core-review-kind-environment-variable = 環境變數
core-review-kind-shell-extension = 殼層擴充功能
core-review-kind-menu-command = 功能表命令
core-review-kind-link-type = 連結類型
core-review-kind-file-type = 檔案類型
core-review-kind-com = COM
core-review-kind-firewall-rule = 防火牆規則
core-review-kind-event-log-source = 事件記錄檔來源
core-review-kind-font = 字型
core-review-kind-odbc-driver = ODBC 驅動程式
core-review-kind-odbc-source = ODBC 資料來源
core-review-kind-compatibility-setting = 相容性設定
core-review-kind-default-apps = 預設應用程式
core-review-kind-program-name = 程式名稱
core-review-kind-other-settings = 其他設定

## The installer's list of what an app sets up: what each line says of it.

# { $file } is a program's file name. { $account } is the Windows account it runs as, such as LocalSystem. { $starts }
# says when it starts, and is not shown itself; for "other", { $start } is one of the core-review-start- words below.
core-review-service-detail =
    { $starts ->
        [automatic] 以 { $account } 身分執行 { $file }，立即啟動，並在這台電腦每次啟動時啟動
        [delayed] 以 { $account } 身分執行 { $file }，立即啟動，並在這台電腦每次啟動後不久啟動
        [manual] 以 { $account } 身分執行 { $file }，只在被要求時才啟動
        [disabled] 以 { $account } 身分執行 { $file }，已停用
        [other] 以 { $account } 身分執行 { $file }，{ $start }
       *[unknown] 以 { $account } 身分執行 { $file }
    }
# { $account } is a Windows account, such as LocalService.
core-policy-account-custom = { $account }（未還原自訂帳戶）
# { $file } is a driver's file name.
core-review-driver-detail = 將 { $file } 載入 Windows 本身
# { $file } is a driver's file name. { $start } is when Windows starts it: one of the core-review-start- words below.
core-review-driver-detail-starts = 將 { $file } 載入 Windows 本身，{ $start }
# When a service or driver starts, ending a sentence such as "Loads disk.sys into Windows itself, starts …".
core-review-start-boot = 隨 Windows 一起啟動，且早於其他所有項目
core-review-start-system = 在 Windows 啟動過程中隨之啟動
core-review-start-automatic = 在這台電腦每次啟動時啟動
core-review-start-delayed = 在這台電腦每次啟動後不久啟動
core-review-start-manual = 只在被要求時才啟動
core-review-start-disabled = 已停用，因此永遠不會啟動
core-review-start-unknown = 以 Windows 未說明的方式啟動
# { $account } is the account a task runs as: an account's name, or core-review-task-runs-as-you, or
# core-review-task-runs-as-creator.
core-review-task-detail = 執行身分：{ $account }
# { $account } is the account a task runs as, as for core-review-task-detail.
core-review-task-detail-highest = 執行身分：{ $account }，使用其可擁有的最高權限
core-review-task-runs-as-you = 您（目前登入的使用者）
core-review-task-runs-as-creator = 建立它的使用者
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-autostart-detail =
    { $scope ->
        [everyone] 在任何人登入時執行
       *[you] 在您登入時執行
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shortcut-startup-detail =
    「啟動」資料夾中的捷徑，{ $scope ->
        [everyone] 適用於這台電腦上的所有人
       *[you] 適用於您
    }
# { $file } is the file name of what the shortcut opens. { $scope } is whom it is set up for: everyone on this PC,
# or you. It is not shown itself.
core-review-shortcut-detail =
    開啟 { $file }，{ $scope ->
        [everyone] 適用於這台電腦上的所有人
       *[you] 適用於您
    }
# { $name } is an environment variable's name, such as PATH.
core-review-path-title = 新增至 { $name }
core-review-path-install-folder = 安裝資料夾
# { $path } is a folder inside the install folder, such as bin\tools.
core-review-path-below-install-folder = 安裝資料夾\{ $path }
# { $value } is the variable's value, as it will be set.
core-review-variable-detail = 設定為 { $value }
# { $count } is how many registry values.
core-review-other-settings-title =
    { $count ->
       *[other] 應用程式本身機碼以外的 { $count } 項設定
    }
# { $keys } is a list of registry keys, such as HKCU\Software\Vendor.
core-review-other-settings-detail = 位於 { $keys }
# { $keys } is a list of registry keys, and { $count } how many more there are.
core-review-other-settings-detail-more = 位於 { $keys }，以及其他 { $count } 個
# { $way } says whether the rule allows or blocks, and incoming or outgoing connections; it is not shown itself.
# { $ports } is a list of port numbers, and { $port_count } how many. { $program } is the program's file name.
# { $networks } is a list of the kinds of network Windows names in English (domain, private, public), and
# { $network_count } how many.
# The kinds of network a firewall rule is for, as Windows Defender Firewall names its profiles in your language. They
# fill { $networks } in core-review-firewall-detail, joined into a list.
core-review-network-domain = 網域
core-review-network-private = 私人
core-review-network-public = 公用
core-review-firewall-detail =
    { $way ->
        [allow-in] 允許輸入連線
        [allow-out] 允許輸出連線
        [block-in] 封鎖輸入連線
       *[block-out] 封鎖輸出連線
    }{ $port_count ->
        [0] { "" }
       *[other] （連接埠 { $ports }）
    }，對象為 { $program }，套用於{ $network_count ->
        [0] 所有網路
       *[other] { $networks }網路
    }
# { $server } is the file name of what Explorer loads. { $scope } is whom it is set up for: everyone on this PC, or
# you. It is not shown itself.
core-review-shell-extension-detail =
    將 { $server } 載入檔案總管，{ $scope ->
        [everyone] 適用於這台電腦上的所有人
       *[you] 適用於您
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shell-extension-detail-own-file =
    將它安裝的檔案載入檔案總管，{ $scope ->
        [everyone] 適用於這台電腦上的所有人
       *[you] 適用於您
    }
# The name of a shell extension: { $handler } is its name in the registry, and { $class } the kind of file or folder it
# is for, such as * or Directory.
core-policy-shell-extension-name = { $class } 上的 { $handler }
# { $on } is the kind of file or folder whose menu it is, as the registry names it, such as * or
# Directory. { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-menu-command-detail =
    在 { $on } 的功能表中新增命令，{ $scope ->
        [everyone] 適用於這台電腦上的所有人
       *[you] 適用於您
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-default-verb-detail =
    變更開啟這類項目時的動作，{ $scope ->
        [everyone] 適用於這台電腦上的所有人
       *[you] 適用於您
    }
core-review-client-entry-detail = 可被選為處理此項目的程式。不會將任何設定改為使用它
core-review-com-permissions-title = COM 權限
core-review-com-permissions-detail = 指定誰可以啟動其 COM 類別，或這些類別以什麼身分執行
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-link-type-detail =
    這類連結會以此應用程式開啟，{ $scope ->
        [everyone] 適用於這台電腦上的所有人
       *[you] 適用於您
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-file-type-detail =
    這類檔案會以此應用程式開啟，{ $scope ->
        [everyone] 適用於這台電腦上的所有人
       *[you] 適用於您
    }
# { $log } is the name of a Windows event log, such as Application.
core-review-event-log-source-detail = 可寫入 { $log } 記錄檔，訊息來自它自己的檔案
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-font-detail =
    在登入時從安裝資料夾載入，{ $scope ->
        [everyone] 適用於這台電腦上的所有人
       *[you] 適用於您
    }
core-review-odbc-driver-detail = 載入使用 ODBC 連線資料庫的程式中
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-odbc-source-detail =
    資料庫連線，{ $scope ->
        [everyone] 適用於這台電腦上的所有人
       *[you] 適用於您
    }
core-review-default-apps-detail = 可在「預設應用程式」中選擇。不會將任何設定改為使用它
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-program-name-detail =
    「執行」和其他程式可以透過此名稱找到它，{ $scope ->
        [everyone] 適用於這台電腦上的所有人
       *[you] 適用於您
    }
# { $server } is the file name of what serves the classes.
core-review-com-title = { $server } 中的 COM
core-review-com-title-own-file = 它安裝之檔案中的 COM
# { $count } is how many registry values.
core-review-com-detail =
    { $count ->
       *[other] 其他程式可以啟動之類別的 { $count } 項設定
    }
core-review-com-support-title = COM 型別程式庫、介面和 AppID
# { $libraries }, { $interfaces }, and { $app_ids } are how many registry values there are for its type libraries, its
# interfaces, and its AppIDs.
core-review-com-support-detail = 其類別附帶的型別程式庫（{ $libraries }）、介面（{ $interfaces }）和 AppID（{ $app_ids }）設定
core-review-compatibility-title = 其程式的啟動方式
# { $count } is how many registry values.
core-review-compatibility-detail =
    { $count ->
       *[other] 它安裝之程式的 { $count } 項相容性設定
    }

## The installer's notes on whose folders and settings these are, and what is not coming.

# { $account } is the name of a Windows account, such as alice.
core-review-account-signed-in = 使用者資料和個別使用者設定會套用到在這裡登入的 { $account }，而不是執行此程式的系統管理員。
# { $account } is the name of a Windows account, such as alice.
core-review-account-no-desktop = 此工作階段中沒有人登入，因此使用者資料和個別使用者設定會套用到執行此程式的帳戶 { $account }。
# { $reason } is what Windows said, in its own words.
core-review-account-not-told = 無法判斷目前登入的使用者（{ $reason }），因此已排除個別使用者設定，而使用者資料會放在安裝位置下。
# { $reason } is what Windows said, in its own words.
core-review-signed-in-unknown = 無法判斷目前登入的使用者：{ $reason }
# Headings of what is left out, shared by the app's list and the installer's notes.
core-review-left-out-credentials = 未包含在此安裝程式中，因為它們看起來像認證
core-review-left-out-settings = 帳戶為此應用程式保存的設定和歷程記錄，只會在開啟「使用者資料」選項時包含
core-review-left-out-certificates = 此應用程式似乎依賴在其原電腦上受信任的憑證，而這些憑證不會安裝
core-review-left-out-fonts = 這些字型以此應用程式的名稱命名，但沒有任何資訊表示它們屬於此應用程式，因此不會安裝
# { $names } is a list of names, such as the names of saved passwords.
core-review-note-credentials = 未包含在此安裝程式中，因為它們看起來像認證：{ $names }。這台電腦上的相關內容會維持原狀，您可能需要重新輸入。
# { $names } is a list of names of license data.
core-review-note-licenses = 未包含在此安裝程式中，因為建置時「授權與啟用資料」選項已關閉：{ $names }。
# { $names } is a list of places an account keeps settings, such as folders.
core-review-note-settings = 未包含在此安裝程式中，因為建置時「使用者資料」選項已關閉：帳戶為此應用程式保存的設定和歷程記錄，位於 { $names }。此應用程式在這裡會像新使用者第一次使用一樣啟動。
# { $names } is a list of certificates, each as core-review-certificate.
core-review-note-certificates = 此應用程式似乎依賴在其原電腦上受信任的憑證，而這些憑證不會安裝：{ $names }。是否要在這裡信任它們，由您決定。
# { $names } is a list of font names.
core-review-note-fonts = 這些字型以此應用程式的名稱命名，但沒有任何資訊表示它們屬於此應用程式，因此不會安裝：{ $names }。
# { $names } is a list of names.
core-review-note-left-out-by-choice = 建置此安裝程式的人已排除：{ $names }。
# { $names } is a list of names.
core-review-note-added-by-choice = 建置此安裝程式的人認為這些屬於此應用程式，因此已新增：{ $names }。
# { $name } is a certificate's subject, such as CN=Contoso, and { $store } the certificate store it was in, such as
# Root.
core-review-certificate = { $store } 中的 { $name }
# { $names } is a list of names, and { $count } how many more there are.
core-review-names-and-more = { $names }，以及其他 { $count } 個

## Why something in a package is left out. Each is the second half of a sentence of core-warning-review-left-out.

core-review-no-account = 它屬於單一帳戶的設定，但無法判斷目前登入的使用者
core-review-name-not-accepted = Windows 不接受其名稱
# { $error } is what Windows said, in its own words.
core-review-service-unreadable = 無法讀取其目前的設定：{ $error }
core-review-service-dependency = 它依賴名稱無效的服務
# { $path } is the program the service runs.
core-review-service-runs-other = 它執行的 { $path } 不是此套件安裝的
core-review-service-settings = 其設定指向此套件未安裝的項目，或不屬於它自己的機碼
core-review-driver-no-hash = 它沒有可用來檢查的雜湊
core-review-license-for-the-machine = 為整台電腦存放的授權只會回報
core-review-license-starts-something = 授權沒有理由啟動任何項目
core-review-task-not-captured = 未擷取其定義
# { $error } is what was wrong with the task's XML, in English.
core-review-task-unreadable = 無法讀取其定義：{ $error }
core-review-task-no-account = 它以安裝者的身分執行，但無法判斷這裡目前登入的使用者
# { $path } is what the shortcut opens.
core-review-shortcut-opens-other = 它開啟的 { $path } 不是此套件安裝的
# { $service } is the service's name, and { $path } the program it runs now.
core-policy-service-exists = { $service } 已存在於這台電腦上並執行 { $path }，而這不是來自先前在這裡的安裝。如果它是此應用程式的舊複本，請先將它解除安裝
core-policy-key-control-characters = 其機碼名稱包含控制字元
core-policy-value-control-characters = 其值的名稱包含控制字元
core-policy-registry-link = 它是登錄連結
# { $reason } is why the key it stands for is left out, such as "no app install should change this key".
core-policy-virtual-store = 它代表電腦層級的機碼，而{ $reason }
core-policy-registry-part = 任何應用程式安裝都不應寫入登錄的這個部分
core-policy-key-refused = 任何應用程式安裝都不應變更此機碼
core-policy-net-debugger = 它會為每個 .NET 程式指定偵錯工具
core-policy-net-install-root = 它會變更每個 .NET 程式載入 .NET 的位置
core-policy-class-redirects = 它會變更某類程式或資料夾的開啟方式
core-policy-app-path-name = 它以檔案名稱以外的名稱指定程式
core-policy-app-path-not-installed = 它會為此套件未安裝的檔案註冊程式名稱
core-policy-shared-default-verb = 它會變更開啟某類所有檔案或資料夾時的動作
core-policy-command-runs-other = 它新增的命令會執行此套件未安裝的項目
core-policy-autostart-not-command = 它會自動啟動，且其值不是命令
core-policy-autostart-runs-other = 它會自動啟動，並執行此套件未安裝的項目
core-policy-environment-own-rules = 環境變數依其專屬規則還原
core-policy-service-not-restored = 它屬於此套件未還原的服務
core-policy-service-own-rules = 服務本身的設定只能透過其規則還原
core-policy-service-dll = 其 ServiceDll 不是此套件安裝的檔案
core-policy-event-log-source-only = 只能還原事件記錄檔來源本身的值
core-policy-security-log = 沒有任何應用程式會寫入「安全性」記錄檔
core-policy-event-log-messages = 其訊息會來自此套件未安裝的檔案
core-policy-type-library = 其型別程式庫不是此套件安裝的檔案
# { $name } is a program's file name, such as app.exe.
core-policy-application-opened = 它會變更 { $name } 的開啟方式，而此套件並未安裝該程式
core-policy-event-log-no-file = 事件記錄檔來源必須從檔案取得其訊息
core-policy-com-runs-other = 它執行的項目不是此套件安裝的檔案
core-policy-com-redirect = 它會讓此類別變成此套件未安裝的項目
core-policy-com-server = 它執行的項目既不是此套件安裝的檔案，也不屬於 Windows
core-policy-interface = 它會以此套件未安裝的項目決定介面呼叫的方式
# { $name } is a program's file name, such as app.exe.
core-policy-app-id-started = 它會變更 { $name } 的啟動方式，而此套件並未安裝該程式
core-policy-app-id-surrogate = 它會由此套件未安裝的程式代管
core-policy-app-id-service = 它會在此套件未還原的服務中執行
core-policy-app-id-run-as = 它會以專屬帳戶執行
core-policy-app-id-link = 它未指定 AppID
core-policy-app-id-remote = 它會在這台電腦以外的地方執行
core-policy-odbc-own-driver = 這是 ODBC 本身的設定，而不是驅動程式
core-policy-odbc-driver-not-installed = 其驅動程式不是此套件安裝的檔案
core-policy-loads-other-file = 它會載入此套件未安裝的檔案
core-policy-odbc-own-source = 這是 ODBC 本身的設定，而不是資料來源
core-policy-odbc-source-missing = 它列出了此套件沒有的資料來源
core-policy-odbc-source-driver = 其驅動程式既不是此套件安裝的檔案，也不屬於 Windows
core-policy-shell-extension = 它會將不是此套件安裝之檔案的殼層擴充功能載入檔案總管
core-policy-not-text = 其內容不是它應被讀取的文字格式
core-policy-network-place = 它指向網路上的位置
core-policy-font = 它會註冊此套件未安裝的字型，或 Windows 資料夾中的字型
core-policy-compatibility = 它會變更此套件未安裝之程式的執行方式
core-policy-capabilities = 它列出的應用程式，其詳細資料不在此套件中
core-policy-default-program = 它會變更預設用來開啟項目的程式
core-policy-url-protocol = 它表示自己是連結類型，但不是以此套件安裝的項目開啟
core-policy-command-handler = 它新增的命令由此套件未安裝的項目處理
core-policy-installer-command = 它會執行只有 Windows Installer 才能讀取的內容
core-policy-creates-class = 它會建立此套件未安裝的類別
core-policy-opens-links = 它會以此套件未安裝的項目開啟連結
core-policy-opens-files = 它會以此套件未安裝的項目開啟檔案
core-policy-firewall-name = 其名稱無法照原樣帶過去
core-policy-firewall-description = 其描述無法照原樣帶過去
core-policy-firewall-every-program = 它套用到所有程式，但規則只能針對一個程式
# { $path } is the program the rule is for.
core-policy-firewall-program = 它是針對 { $path }，而此套件並未安裝該程式
core-policy-firewall-direction = 它未指定可設定的方向
core-policy-firewall-action = 它會覆寫其他規則，而任何應用程式的規則都不應如此
# { $setting } is a part of a firewall rule, as the scan of its PC described it.
core-policy-firewall-not-carried = 它指定了 { $setting }，但無法照原樣設定，或會削弱其他規則的作用
core-policy-firewall-service = 它是針對此套件未還原的服務
core-policy-firewall-parts = 其中有部分內容不是防火牆規則可以指定的
core-policy-folder-already-there = 該資料夾原本就已存在於安裝位置之外，且沒有任何資訊表示它屬於此應用程式，因此不會在其中放入任何程式
core-policy-variable-name = 其名稱不是有效的變數名稱
core-policy-variable-relied-on = Windows 和其他程式依賴此變數
core-policy-task-name = 其名稱不是 Windows 接受的名稱
core-policy-task-folder = 其資料夾不是 Windows 接受的資料夾
core-policy-task-windows = 它會取代 Windows 本身的工作
core-policy-task-other-action = 它執行的動作不是執行程式
core-policy-task-no-program = 它未指定要執行的程式
# { $path } is the program the task runs.
core-policy-task-runs-other = 它執行的項目不是此套件安裝的：{ $path }
core-policy-task-exists-unreadable = 已有同名的工作，且無法讀取
core-policy-task-exists-other = 已有同名的工作，且它不會執行此應用程式
# { $path } is the program the task already on this PC runs.
core-policy-task-exists-runs = 已有同名的工作並執行 { $path }，而這不是來自先前在這裡的安裝
core-policy-shortcut-not-lnk = 它不是 .lnk 捷徑
core-policy-shortcut-place = 它不在「開始」功能表或桌面上
core-policy-short-name = 其路徑使用了無法檢查的名稱
core-policy-shortcut-no-folder = 這台電腦沒有它所需的這類資料夾
core-policy-data-startup = 它會放在「啟動」資料夾中，並在每次登入時執行
core-policy-data-powershell = 它會放在 PowerShell 啟動時會執行其中內容的位置
core-policy-data-windows = 它會放在屬於 Windows 的資料夾中
core-policy-license-outside = 它位於安裝位置和所有已知資料資料夾之外
core-rewrite-cdata = 它包含 CDATA 區段
core-rewrite-too-much-to-look-through = 此套件的設定內容太多，無法檢查其中指向的項目

## Warnings a review gives, which the installer shows with what it sets up.

# { $kind } says what was left out, and is not shown itself. { $item } is its path or name, or a registry value such
# as HKLM\Software\App\Name. { $reason } is why, as the second half of the sentence, such as "it cannot be a whole
# drive".
core-warning-review-left-out =
    { $kind ->
        [file] 檔案 { $item } 已排除：{ $reason }。
        [folder] 資料夾 { $item } 已排除：{ $reason }。
        [service] 服務 { $item } 已排除：{ $reason }。
        [driver] 驅動程式 { $item } 已排除：{ $reason }。
        [firewall-rule] 防火牆規則 { $item } 已排除：{ $reason }。
        [license-data] 位於 { $item } 的授權資料已排除：{ $reason }。
        [license-value] 授權值 { $item } 已排除：{ $reason }。
        [scheduled-task] 排定的工作 { $item } 已排除：{ $reason }。
        [shortcut] 捷徑 { $item } 已排除：{ $reason }。
        [environment-variable] 環境變數 { $item } 已排除：{ $reason }。
       *[value] { $item } 已排除：{ $reason }。
    }
core-warning-review-package-path-missing = 可複製的檔案沒有套件相對路徑。
# { $count } is the most empty folders a package makes.
core-warning-review-too-many-empty-folders = 此套件列出了超過 { $count } 個空資料夾，只會建立其中的 { $count } 個。
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Documents folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-documents-elsewhere =
    { $count ->
       *[other] 來自「文件」的 { $count } 個檔案會改放在 { $target } 下名為 _external 的資料夾中，因為這台電腦的「文件」資料夾位於 { $folder }，而只會寫入您設定檔內的資料夾。若要使用這些檔案，請自行將它們移到您的「文件」資料夾。
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-documents-unknown =
    { $count ->
       *[other] 來自「文件」的 { $count } 個檔案會改放在 { $target } 下名為 _external 的資料夾中，因為無法判斷這台電腦的「文件」資料夾位於何處。若要使用這些檔案，請自行將它們移到您的「文件」資料夾。
    }
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Saved Games folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-elsewhere =
    { $count ->
       *[other] 來自「儲存的遊戲」的 { $count } 個檔案會改放在 { $target } 下名為 _external 的資料夾中，因為這台電腦的「儲存的遊戲」資料夾位於 { $folder }，而只會寫入您設定檔內的資料夾。若要使用這些檔案，請自行將它們移到您的「儲存的遊戲」資料夾。
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-unknown =
    { $count ->
       *[other] 來自「儲存的遊戲」的 { $count } 個檔案會改放在 { $target } 下名為 _external 的資料夾中，因為無法判斷這台電腦的「儲存的遊戲」資料夾位於何處。若要使用這些檔案，請自行將它們移到您的「儲存的遊戲」資料夾。
    }

## Errors when a folder that only administrators may change cannot be made so, or checked.

# { $path } is a folder.
core-privilege-not-a-folder = { $path } 是連結或檔案，不是資料夾
# { $path } is a folder. { $error } is what Windows said, in its own words.
core-privilege-not-made-here = { $path } 不是由 WinAppMigrator 建立的，且其他人可以變更其中的內容。系統管理員應檢查並移除它：{ $error }
# { $path } is a folder.
core-privilege-others-put-something-in = { $path } 中有不是由 WinAppMigrator 建立的內容，且其他人可以變更它。系統管理員應檢查並移除它
# { $path } is a folder.
core-privilege-became-a-link-in-use = { $path } 在使用期間變成了連結
# { $path } is a folder.
core-privilege-became-a-link-looked-at = { $path } 在檢查期間變成了連結
# { $path } is a file. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-users-cannot-start = 無法讓使用者啟動 { $path }：{ $said }
core-privilege-elevation-unknown = 無法判斷此處理程序是否已提高權限
# { $path } is a folder. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-cannot-restrict = 無法將 { $path } 限制為僅系統管理員可存取：{ $said }
# { $path } is a folder. { $said } is the folder's permissions as Windows wrote them, and what else it printed.
core-privilege-not-restricted = 無法將 { $path } 限制為僅系統管理員可存取，其磁碟機可能不會保留權限：{ $said }
# { $path } is a folder.
core-privilege-written-to-first = 在 { $path } 受到保護之前，有其他處理程序寫入了它
core-privilege-no-program-data = Windows 未提供 ProgramData 的位置

## Checks of a package's files, and of what an install put in place. Each is said of one file, value, service, or the
## like, which the report names beside it.

# { $path } is the installer's file.
core-verify-no-package = { $path } 中沒有套件
# { $path } is a folder.
core-verify-not-unpacked = 此安裝程式的套件未解壓縮到 { $path }
core-verify-schema-supported = 支援套件資訊清單的結構描述。
core-verify-packaged-file-missing = 缺少套件中的檔案。
core-verify-no-package-path = 可複製的檔案沒有套件相對路徑。
core-verify-folder-there = 空資料夾存在。
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-unreadable = 無法讀取以進行檢查：{ $error }
core-verify-restored-file-missing = 缺少還原的檔案。
# { $expected } and { $actual } are sizes in bytes.
core-verify-size-mismatch = 檔案大小不符。預期為 { $expected } 個位元組，實際為 { $actual } 個位元組。
core-verify-no-hash = 資訊清單未記錄此檔案的雜湊，因此無法檢查。
core-verify-hash-mismatch = SHA-256 雜湊不符。
core-verify-file-matches = 檔案大小和雜湊與資訊清單相符。
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-payload-failed = 授權承載驗證失敗：{ $error }
core-verify-license-payload-missing = 缺少套件中的授權承載。
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-failed = 授權存放區驗證失敗：{ $error }
core-verify-license-store-file-missing = 缺少套件中的授權存放區檔案。
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-path-invalid = 套件中的授權存放區路徑無效：{ $error }
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-prerequisite-failed = 必要元件承載驗證失敗：{ $error }
core-verify-prerequisite-missing = 缺少隨附的必要元件承載。
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-failed = 驅動程式映像驗證失敗：{ $error }
core-verify-bundled-driver-missing = 缺少隨附的驅動程式映像。
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-path-invalid = 隨附的驅動程式路徑無效：{ $error }
core-verify-prerequisite-present = 目標電腦上已有執行階段必要元件。
core-verify-prerequisite-not-detected = 未偵測到執行階段必要元件；可能需要重新啟動或手動完成（確切結果請參閱還原記錄檔）。
core-verify-prerequisite-not-probed = 無法透過登錄偵測此必要元件是否存在；已盡可能安裝。
core-verify-license-file-missing = 缺少還原的授權檔案。
core-verify-license-store-restored-missing = 缺少還原的授權存放區檔案。
core-verify-registry-key-there = 登錄機碼存在。
core-verify-registry-value-matches = 登錄值與封存的資料相符。
core-verify-registry-value-differs = 登錄值與封存的資料不符。
core-verify-firewall-rule-there = Windows 防火牆有此規則。
core-verify-variable-matches = 環境變數值與封存的資料相符。
core-verify-variable-differs = 環境變數值與封存的資料不符。
# { $error } is what Windows said, in its own words.
core-verify-variable-unreadable = 環境變數不存在或無法讀取：{ $error }
core-verify-service-runs-other = 服務存在，但未以其引數執行還原的二進位檔。
core-verify-service-other-account = 服務存在，但未以預期的帳戶執行。
core-verify-service-matches = 服務存在，並以其引數和帳戶執行還原的二進位檔。
core-verify-service-missing = 服務不存在。
# { $error } is what Windows said, in its own words.
core-verify-service-unreadable = 無法讀取服務：{ $error }
core-verify-driver-other-image = 驅動程式服務未指向還原的映像。
core-verify-driver-not-registered = 還原後，隨附的驅動程式服務未註冊。
# { $error } is what Windows said, in its own words.
core-verify-driver-unreadable = 無法讀取驅動程式服務：{ $error }
core-verify-driver-image-missing = 驅動程式映像不存在。
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-unchecked = 無法檢查驅動程式映像：{ $error }
core-verify-task-matches = 排定的工作存在，並執行還原的程式。
core-verify-task-runs-other = 排定的工作存在，但未執行還原的程式。
core-verify-task-missing = 排定的工作不存在。
# { $error } is what Windows said, in its own words.
core-verify-task-unreadable = 查詢排定的工作失敗：{ $error }
core-verify-shortcut-matches = 捷徑存在，且目標路徑與還原的目標相符。
core-verify-shortcut-opens-other = 捷徑存在，但目標路徑與還原的目標不符。
core-verify-shortcut-missing = 捷徑不存在。
# { $error } is what Windows said, in its own words.
core-verify-shortcut-unreadable = 查詢捷徑失敗：{ $error }

## Warnings and errors a check gives, which the app and the installer show.

core-warning-verify-schema-unsupported = 不支援套件資訊清單的結構描述。
# { $error } is what went wrong, in Windows' or the program's own words.
core-warning-verify-package-file-failed = 套件檔案驗證失敗：{ $error }
core-warning-verify-folder-missing = 空資料夾不存在。
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-key-missing = 登錄機碼不存在或無法讀取：{ $error }
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-value-missing = 登錄值不存在或無法讀取：{ $error }
core-warning-verify-registry-kind-missing = 缺少登錄值類型。
core-warning-verify-firewall-rule-missing = Windows 防火牆沒有此規則。
core-warning-verify-service-scope = 服務驗證會讀取 HKLM 中的服務設定，但不會啟動服務。
core-warning-verify-driver-manual = WinAppMigrator 不會轉散發一或多個廠商/內建驅動程式；請在目標電腦上使用廠商已簽署的驅動程式套件重新安裝。
core-warning-verify-task-scope = 排定的工作驗證只會檢查註冊和命令參照。

## The verification report written beside a package, verification_report.md. Its headings follow a "#".

core-verify-report-title = WinAppMigrator 驗證報告
# { $time } is when, as a date and time in UTC, such as 2026-11-15T10:00:00Z.
core-verify-report-verified = 驗證時間（UTC）：{ $time }
# { $valid } is true or false, which are not translated.
core-verify-report-package-valid = 套件有效：{ $valid }
# { $valid } is true or false, which are not translated.
core-verify-report-restore-valid = 還原有效：{ $valid }
core-verify-report-restore-not-run = 還原有效：未執行
core-verify-report-items = 項目
# { $message } is what a check found, and { $source } the file or setting it looked at.
core-verify-report-item-from = { $message } 來源：{ $source }
core-verify-report-warnings = 警告
core-verify-report-errors = 錯誤
core-verify-report-none = 無。
