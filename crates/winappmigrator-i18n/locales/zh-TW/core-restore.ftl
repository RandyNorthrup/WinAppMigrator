# Words the core writes for people as it installs an app from a package: why an install will not start, the list of
# what it did (or for a preview, what it would do) that the installer window shows, its warnings, the record it keeps of
# each install so that it can be undone, and the core's errors. Message names start with core-.
#
# The list of what an install did is short and plain, one line for each thing, as the installer window lists them:
# "copied", "restored", "would copy". Words such as --force and --apply are command-line options, and LocalService,
# SYSTEM, ProgramData, and %AppData% are as Windows names them: keep them as they are.


## Errors: what went wrong, when the core cannot go on. Each is shown on its own, or after words that say what was being
## done.

core-error-list-changed = 此安裝要執行的作業在您同意後已有變更，因此未安裝任何內容
# { $path } is a file or folder, such as C:\Program Files\App\app.exe, or words that name one, such as "the installer".
# { $error } is what Windows said went wrong.
core-error-io = { $path } 的 I/O 失敗：{ $error }
# { $detail } is what went wrong.
core-error-registry = 讀取登錄失敗：{ $detail }
# { $path } is a program file. { $detail } is what was wrong with it.
core-error-pe = 剖析 { $path } 的 PE 失敗：{ $detail }
# { $detail } is what was wrong, already in words.
core-error-invalid-input = 輸入無效：{ $detail }
# { $detail } is what went wrong.
core-error-serialization = 報告序列化失敗：{ $detail }
# { $detail } is what went wrong.
core-error-archive = 封存作業失敗：{ $detail }
# { $detail } is what went wrong.
core-error-network = 網路作業失敗：{ $detail }
# { $detail } is what went wrong.
core-error-time-format = 時間格式化失敗：{ $detail }
core-error-cancelled = 作業已取消
# What is said when part of the program stopped unexpectedly and gave no reason.
core-error-panic-no-reason = 未提供原因

## The record of each install, kept so that it can be undone.

# { $kind } is a number that says what type of registry value it is.
core-journal-unknown-value-type = 未知的登錄值類型 { $kind }
core-journal-not-elevated = 未提高權限
core-journal-needs-admin = 安裝需要系統管理員權限，才能將其記錄存放在其他人無法變更的位置
# { $path } is a folder.
core-journal-not-an-install-folder = 這不是安裝的資料夾：{ $path }
# { $path } is the file the record is kept in.
core-journal-not-written = 無法寫入此安裝的記錄：{ $path }
# { $path } is a folder.
core-journal-path-escapes-root = 路徑超出預定的根目錄：{ $path }
# { $path } is a folder.
core-journal-nothing-exists = 這台電腦上沒有這些項目：{ $path }
core-journal-busy = 另一個安裝或解除安裝作業正在執行。請等它完成後再試一次
# Said of a file that was to get back the permissions it had.
core-journal-no-folder = 它沒有資料夾
# Said of the permissions a file had, which were kept to give back to it.
core-journal-permissions-unreadable = 無法讀取保存的內容
# { $folder } is the folder of one install's record.
core-journal-unreadable-empty = 無法使用 { $folder } 中的安裝記錄：記錄是空的
# { $folder } is the folder of one install's record.
core-journal-unreadable-no-header = 無法使用 { $folder } 中的安裝記錄：其開頭沒有說明它所屬的安裝
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-version = 無法使用 { $folder } 中的安裝記錄：它是由此程式的其他版本寫入的
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-install = 無法使用 { $folder } 中的安裝記錄：記錄表示它屬於其他安裝
# { $id } is an install's id, such as wam-1a2b3c.
core-journal-not-an-install-id = 這不是安裝的識別碼：{ $id }

## Why an install will not start, or stopped before it changed anything.

core-restore-apply-and-dry-run = 不能同時指定 --apply 和 --dry-run
# { $path } is the folder that was chosen to install in.
core-restore-location-not-full-path = 安裝位置必須是完整路徑：「{ $path }」
# { $problem } says what is wrong with the folder, already in words. { $path } is the folder.
core-restore-location-unusable = 無法使用該安裝位置：{ $problem }：「{ $path }」
core-restore-needs-admin-plan = 還原計畫包含僅限系統管理員的登錄、服務、排定的工作、捷徑或受保護目標的變更
# { $path } is the folder that was chosen to install in.
core-restore-needs-admin-target = 目標根目錄需要提高權限：{ $path }
core-restore-needs-admin-user-data = 將使用者資料還原到 ProgramData 或其他受保護的位置需要提高權限
# { $error } is why the install stopped. { $failed } is what could not be put back, as sentences of their own.
core-restore-not-all-put-back = { $error }。已完成的作業無法全部恢復：{ $failed }
# { $error } is what went wrong.
core-restore-nowhere-to-keep-record = 找不到安全的位置來存放可復原此安裝的記錄，因此未安裝任何內容：{ $error }
# { $path } is a file the app would have put there.
core-restore-already-exists = { $path } 已存在，因此未安裝任何內容。請使用 --force 取代它
# { $root } is a folder the app has files for. { $link } is that folder, or one it is in.
core-restore-root-under-link = 此應用程式有要還原到 { $root } 的內容，而 { $link } 是指向其他位置的連結，其他人可能可以變更它，因此未安裝任何內容。如果要放在該處的是使用者資料或授權資料，請在安裝時不包含其使用者資料和授權資料。如果是「啟動」資料夾中的捷徑，請將該資料夾移回原本的位置
core-restore-hash-check-failed = 套件未通過雜湊檢查，因此未安裝任何內容
# { $detail } is the first thing found wrong, already in words.
core-restore-hash-check-failed-because = 套件未通過雜湊檢查，因此未安裝任何內容：{ $detail }
# { $path } is a file or folder.
core-restore-refuse-records-folder = 拒絕寫入安裝存放記錄的位置：{ $path }
# { $path } is a file or folder.
core-restore-refuse-outside = 拒絕寫入安裝位置和已知資料資料夾以外的位置：{ $path }
# { $path } is a file.
core-restore-no-parent-folder = 目的地沒有上層資料夾：{ $path }
# { $path } is where a file was to go.
core-restore-folder-in-the-way = 此檔案要放置的位置有一個資料夾：{ $path }
# { $path } is a file in the package.
core-restore-file-changed = { $path } 已不是套件中的內容，因此未安裝
# { $path } is a file in the package.
core-restore-source-is-link = { $path } 是連結，不應追蹤
# { $path } is a file in the package.
core-restore-source-became-link = { $path } 在開啟期間變成了連結
# Why a service did not start, after "did not start:".
core-restore-service-stopped-by-itself = 它又自行停止了
# Why part of the app was left out, after "was left out:".
core-restore-com-class-someone-elses = 這台電腦上已有此 COM 類別，且屬於其他項目

## What an install did, one line for each thing, or for a preview, what it would do. { $source } is a file in the
## package, and { $destination } and { $path } are where it goes on this PC. { $key } is a registry key, such as
## HKLM\Software\App.

# { $package_valid } is true or false, and { $restore_valid } is true, false, or not_run: keep them as they are.
core-restore-verification-result = 還原後驗證 package_valid={ $package_valid } restore_valid={ $restore_valid }
# { $name } is a service's name.
core-restore-started-service = 已啟動服務 { $name }
core-restore-would-copy = 將複製 { $source } -> { $destination }
core-restore-copied = 已複製 { $source } -> { $destination }
core-restore-would-create-empty-folder = 將建立空資料夾 { $path }
core-restore-created-empty-folder = 已建立空資料夾 { $path }
# { $value } is the name of a value in the key.
core-restore-would-restore-registry-value = 將還原登錄 { $key } { $value }
core-restore-would-restore-registry-key = 將還原登錄 { $key }（機碼）
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-would-add-firewall-rule = 將為 { $program } 新增防火牆規則 { $name }
# { $name } is a firewall rule's name.
core-restore-firewall-rule-already-there = 防火牆規則 { $name } 已存在
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-added-firewall-rule = 已為 { $program } 新增防火牆規則 { $name }
# { $name } is a service's name, { $command } what it runs, and { $account } the account it runs as, such as LocalService.
core-restore-would-restore-service = 將還原服務 { $name } -> { $command }（以 { $account } 身分執行）
# { $name } is a runtime, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-restore-prerequisite-present = 必要元件已存在：{ $name }
# { $name } is a runtime that Windows has as a Windows feature.
core-restore-would-install-prerequisite-feature = 將安裝必要元件 { $name }（Windows 功能）
# { $name } is a runtime whose installer is in the package.
core-restore-would-install-prerequisite-bundled = 將安裝必要元件 { $name }（隨附的安裝程式）
# { $name } is a runtime that would come through winget, which is a program's name.
core-restore-would-install-prerequisite-winget = 將安裝必要元件 { $name }（winget）
# { $name } is a runtime.
core-restore-installed-prerequisite = 已安裝必要元件 { $name }
# { $name } is a runtime.
core-restore-installed-prerequisite-reboot = 已安裝必要元件 { $name }（需要重新啟動）
core-restore-would-restore-license = 將還原授權 { $source } -> { $destination }
core-restore-restored-license = 已還原授權 { $source } -> { $destination }
core-restore-would-restore-license-registry = 將還原授權登錄 { $key }
# { $name } is a driver's name.
core-restore-driver-not-restored = 未自動還原驅動程式：{ $name }
# { $name } is a driver's name.
core-restore-would-install-driver = 將安裝驅動程式 { $name } -> { $destination }
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-would-restore-task = 將還原排定的工作 { $name }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-startup-shortcut = 將還原啟動捷徑 { $shortcut } -> { $target }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-shortcut = 將還原捷徑 { $shortcut } -> { $target }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-would-restore-variable =
    { $scope ->
        [user] 將還原使用者環境變數 { $name }
       *[machine] 將還原系統環境變數 { $name }
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-left-variable =
    { $scope ->
        [user] 使用者環境變數 { $name } 維持原狀：其中沒有任何內容屬於此應用程式
       *[machine] 系統環境變數 { $name } 維持原狀：其中沒有任何內容屬於此應用程式
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-restored-variable =
    { $scope ->
        [user] 已還原使用者環境變數 { $name }
       *[machine] 已還原系統環境變數 { $name }
    }
core-restore-set-uninstaller = 已設定 { $key } 的解除安裝程式
core-restore-placed-driver-image = 已放置驅動程式映像 { $source } -> { $destination }
# { $name } is a driver's name.
core-restore-configured-driver = 已設定驅動程式 { $name }
# { $name } is a driver's name.
core-restore-created-driver = 已建立驅動程式 { $name }
# { $name } is a driver's name.
core-restore-started-driver = 已啟動驅動程式 { $name }
core-restore-ensured-registry-key = 已確認登錄機碼 { $key } 存在
# { $value } is the name of a value in the key.
core-restore-restored-registry-value = 已還原登錄 { $key } { $value }
# "(Default)" is how Windows names the value a key has with no name: write it as Registry Editor does.
core-restore-restored-registry-default = 已還原登錄 { $key } (預設值)
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-configured-service = 已設定服務 { $name }（以 { $account } 身分執行）
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-created-service = 已建立服務 { $name }（以 { $account } 身分執行）
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-restored-task = 已還原排定的工作 { $name }
core-restore-restored-shortcut = 已還原捷徑 { $path }

## Warnings an install gives, and why it failed. They are shown after it, on the PC it ran on.

core-warning-restore-earlier-not-trusted = 有一份未完成之安裝的記錄未被處理，因為其他人可以變更它。該安裝所做的任何變更仍然存在。
# { $app_name } is the app's name. { $count } is how many changes were put back.
core-warning-restore-earlier-undone =
    { $count ->
       *[other] 先前安裝 { $app_name } 的作業未完成，因此已先將它復原。已恢復 { $count } 項變更。
    }
# { $app_name } is the app's name.
core-warning-restore-earlier-never-finished = 先前安裝 { $app_name } 的作業未完成。安裝時會先將它復原。
core-warning-restore-user-data-real-locations = 使用者資料會還原到這台電腦實際的 %AppData%/%ProgramData% 位置，而不是目標根目錄。
# { $error } is what went wrong.
core-warning-restore-commit-failed = 無法完成此安裝的記錄：{ $error }
# { $name } is a service's name.
core-warning-restore-service-still-starting = 已要求啟動服務 { $name }，但它尚未完成啟動。它可能稍後就會啟動。這台電腦下次啟動時，它也會自動啟動。
# { $name } is a service's name. { $reason } is why it did not start, as Windows said it.
core-warning-restore-service-not-started = 服務 { $name } 已安裝但未啟動：{ $reason }。在它啟動之前，應用程式可能無法運作。這台電腦下次啟動時，Windows 會再試一次。
# { $name } is a service's name. { $reason } is why it could not be stopped.
core-warning-restore-service-not-stopped = 無法先停止服務 { $name }：{ $reason }。在這台電腦下次啟動之前，它可能會繼續執行舊的程式。
core-warning-restore-install-cancelled = 安裝已取消。
core-warning-restore-preview-cancelled = 預覽已取消。
# { $item } is what was left out, such as a file or a service's name. { $reason } is why, already in words.
core-warning-restore-left-out = { $item } 已排除：{ $reason }。
# { $error } is what went wrong.
core-warning-restore-verification-failed = 無法執行還原後驗證：{ $error }
# { $runner } says what relies on the file: a service, a scheduled task, a firewall rule, a shortcut, or a registry value.
# { $path } is the file. { $name } is the name of what relies on it. { $refusal } is why the file was not replaced,
# already in words.
core-warning-restore-program-would-run =
    { $runner ->
        [service] 此套件無權取代 { $path }，而服務 { $name } 依賴它，因此未安裝任何內容：{ $refusal }
        [task] 此套件無權取代 { $path }，而排定的工作 { $name } 依賴它，因此未安裝任何內容：{ $refusal }
        [firewall] 此套件無權取代 { $path }，而防火牆規則 { $name } 依賴它，因此未安裝任何內容：{ $refusal }
        [shortcut] 此套件無權取代 { $path }，而捷徑 { $name } 依賴它，因此未安裝任何內容：{ $refusal }
       *[registry] 此套件無權取代 { $path }，而登錄值 { $name } 依賴它，因此未安裝任何內容：{ $refusal }
    }
core-warning-restore-destination-exists = 目的地已存在；請使用 --force 覆寫。
# { $path } is a file. { $error } is what went wrong.
core-warning-restore-file-not-written = 無法寫入 { $path }：{ $error }
core-warning-restore-not-a-folder = 此空資料夾要放置的位置有其他不是資料夾的項目。
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-not-looked-at = 無法檢查 { $path }：{ $error }
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-folder-not-made = 無法建立 { $path }：{ $error }
# Why part of the app was left out, after "was left out:".
core-warning-restore-registration-exists = 這台電腦上已有此項目，且設定為其他內容，安裝不會接管它
core-warning-restore-registry-review = 已從封存的結構化資料還原登錄值，並將已知的應用程式路徑改寫為目標根目錄。
# { $error } is what went wrong.
core-warning-restore-firewall-unreadable = 無法讀取 Windows 防火牆的規則：{ $error }
# Why a firewall rule was left out, after "was left out:".
core-warning-restore-firewall-rule-exists = 此程式已有同名的規則，且設定為其他內容，安裝不會變更它
# { $name } is a runtime.
core-warning-restore-prerequisite-unknown = { $name }：此執行階段沒有已知的安裝程式。
# { $name } is a runtime.
core-warning-restore-prerequisite-needs-admin = 安裝 { $name } 需要提高權限；請以系統管理員身分重新執行安裝程式。
# { $name } is a runtime.
core-warning-restore-prerequisite-reboot = { $name } 已安裝，但需要重新啟動才能完成。
# { $error } is what went wrong.
core-warning-restore-skipped-not-recorded = 無法記錄已略過的執行階段：{ $error }
# { $name } is a runtime. { $reason } is why it was skipped, already in words.
core-warning-restore-prerequisite-skipped = { $name }：{ $reason }
# { $name } is a runtime. { $reason } is why it could not be installed. { $url } is a web address to get it from.
core-warning-restore-prerequisite-failed = 無法安裝 { $name }：{ $reason }。請從 { $url } 取得。
# { $error } is what went wrong.
core-warning-restore-runtime-not-recorded = 無法記錄已安裝的執行階段：{ $error }
# { $error } is what went wrong.
core-warning-restore-license-source-invalid = 套件中的授權路徑無效：{ $error }
core-warning-restore-license-needs-admin = 將授權資料還原到受保護的位置需要提高權限。
core-warning-restore-license-exists = 授權目的地已存在；請使用 --force 覆寫。
# { $error } is what went wrong.
core-warning-restore-license-failed = 無法還原授權資料：{ $error }
core-warning-restore-driver-needs-admin = 註冊驅動程式需要提高權限；請以系統管理員身分重新執行安裝程式。
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-warning-restore-task-runs-as-system = 排定的工作 { $name } 以 SYSTEM 身分執行，擁有這台電腦的完整權限。
core-warning-restore-shortcut-exists = 捷徑已存在；請使用 --force 覆寫。
core-warning-restore-shortcut-all-users = 一或多個還原的捷徑指向公用桌面或所有使用者的「開始」功能表位置。
core-warning-restore-environment-refresh = 環境變數的變更可能需要新的處理程序、殼層或登入工作階段才會生效。
# { $name } is a service's name. { $account } is the account it ran as on the PC it came from.
core-warning-restore-service-account = 服務 { $name } 在來源電腦上以 { $account } 身分執行。其密碼不在安裝程式中，因此服務會設定為以 LocalService 身分執行。如果無法運作，請在「服務」中設定其帳戶和密碼。
# { $name } is a runtime.
core-warning-restore-prerequisite-pending-reboot = { $name } 尚未註冊；重新啟動後應該就會生效。
# { $name } is a runtime.
core-warning-restore-prerequisite-not-verified = { $name } 回報成功但未註冊；安裝可能已失敗。
# { $error } is what went wrong.
core-warning-restore-driver-source-invalid = 隨附的驅動程式路徑無效：{ $error }
# { $error } is what went wrong.
core-warning-restore-driver-not-read = 無法讀取驅動程式目前的設定：{ $error }
core-warning-restore-driver-exists = 驅動程式服務已存在；請使用 --force 重新設定。
core-warning-restore-driver-image-exists = 驅動程式映像已存在；請使用 --force 覆寫。
core-warning-restore-driver-hash-bundled = 隨附的驅動程式映像與其記錄的雜湊不符，因此未安裝。
# { $error } is what went wrong.
core-warning-restore-driver-not-placed = 無法放置驅動程式映像：{ $error }
core-warning-restore-driver-hash-placed = 驅動程式映像在放置後已變更，因此未註冊。
# { $error } is what went wrong.
core-warning-restore-key-not-created = 無法建立登錄機碼：{ $error }
# { $error } is what went wrong.
core-warning-restore-value-not-read = 無法讀取此作業要取代的值：{ $error }
# { $error } is what went wrong.
core-warning-restore-value-not-written = 無法寫入登錄值：{ $error }
core-warning-restore-value-exists = 登錄值已存在；請使用 --force 覆寫。
# { $error } is what went wrong.
core-warning-restore-environment-key-not-created = 無法建立環境變數的登錄機碼：{ $error }
core-warning-restore-variable-exists = 環境變數已存在；請使用 --force 覆寫。
# { $error } is what went wrong.
core-warning-restore-variable-not-written = 無法寫入環境變數：{ $error }
core-warning-restore-value-kind-missing = 缺少登錄值類型。
# { $error } is what went wrong.
core-warning-restore-service-not-read = 無法讀取服務目前的設定：{ $error }
core-warning-restore-service-exists = 服務已存在；請使用 --force 重新設定。
# { $error } is what went wrong.
core-warning-restore-task-not-queried = 查詢排定的工作失敗：{ $error }
core-warning-restore-task-exists = 排定的工作已存在；請使用 --force 覆寫。
# { $error } is what went wrong.
core-warning-restore-task-folder-not-looked-at = 無法檢查工作的資料夾：{ $error }
# { $error } is what went wrong.
core-warning-restore-shortcut-not-put-back = 無法建立捷徑，也無法恢復它所取代的捷徑：{ $error }
# { $error } is what went wrong.
core-warning-restore-command-not-started = 無法啟動命令：{ $error }
# { $output } is what the command wrote, as it wrote it.
core-warning-restore-command-failed = 命令失敗：{ $output }
# { $error } is what went wrong.
core-warning-restore-powershell-not-started = 無法啟動 PowerShell：{ $error }
