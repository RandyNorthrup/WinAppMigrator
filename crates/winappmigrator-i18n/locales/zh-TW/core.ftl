# Words the core writes for people: warnings, refusals, errors, and the installer's list of what it sets up.
# Message names start with core-.


## Warnings a package records when it is built, which an installer shows later, maybe on another PC, in that PC's
## language.

# { $count } is how many shortcuts.
core-warning-shortcuts-unreadable =
    { $count ->
       *[other] 此磁碟機上有 { $count } 個捷徑未指向任何檔案，因此無法判斷它們是否會開啟此應用程式。
    }

## Undoing an install: what an uninstall, or a failed install, says it put back, left, or could not undo. Each line is
## shown on its own, in a list.

# { $app_name } is the app's name.
core-rollback-record-damaged = { $app_name } 的部分記錄無法讀取，因此其中描述的任何項目都仍在原處，而它所取代的檔案可能無法恢復。
core-rollback-settings-entry-stays = 此應用程式在「設定」中的項目會保留，以便從該處再次執行此作業。
# { $name } is a runtime's name, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-rollback-prerequisite-stays = { $name } 已安裝（或可能已安裝），並會保留，因為其他程式可能正在使用它。
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-drive-not-connected = 此安裝放在 { $folder } 中的內容位於未連接的磁碟機上。請連接該磁碟機，即可完成此作業。
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-record-part-unread = 此安裝放在 { $folder } 中的內容會保留，因為該部分的記錄無法讀取。

## Why a file or folder was left, shown after "was left:" in the messages below them. Each is part of a sentence, so
## starts with a small letter and has no full stop.

# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-being-changed = 執行期間有程式正在變更 { $folder }，因此未在其中進行任何其他作業。請關閉正在變更它的程式，然後再次執行
# { $folder } is a folder; { $error } is what Windows said went wrong.
core-rollback-reason-not-opened = 無法開啟 { $folder }：{ $error }
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-no-record = 沒有 { $folder } 原本位置的記錄
# { $folder } is a folder; { $actual } is the folder it now leads to.
core-rollback-reason-moved = { $folder } 現在指向 { $actual }，而不是安裝期間的位置
# Why a service was not stopped.
core-rollback-reason-did-not-stop = 它未及時停止

## Files and folders

# { $path } is a folder; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-folder-left = 資料夾 { $path } 未移除：{ $reason }。
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-removed = 已移除資料夾 { $path }
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-holds-others = 資料夾 { $path } 會保留，因為其中有不是此安裝放入的檔案。
# { $path } is a folder; { $error } is what Windows said went wrong.
core-rollback-folder-not-removed = 無法移除資料夾 { $path }：{ $error }。
# { $path } is a file; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-file-left = 檔案 { $path } 未移除：{ $reason }。
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-folder-gone = 不會恢復 { $path }，因為它原本所在的資料夾已不存在。
# { $path } is a file; { $error } is what went wrong.
core-rollback-partial-not-removed = 無法移除不完整的檔案：{ $path }（{ $error }）。
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-checked = 無法檢查 { $path }：{ $error }。
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-removed = 已移除 { $path }
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-removed = 無法移除 { $path }：{ $error }。
# { $path } is a file this install had replaced, and has put back.
core-rollback-file-put-back = 已恢復 { $path }
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-put-back = 無法恢復 { $path }：{ $error }。
# Why a file could not be put back.
core-rollback-backup-gone = 備份已不存在
# { $path } is a file.
core-rollback-no-parent-folder = { $path } 沒有上層資料夾

## A file that stays. { $why } says why: still-used (something that stays still uses it), changed (it has changed since
## it was installed), or not-asked (it is someone's data, and they did not ask for it to go). { $path } is the file.

core-rollback-file-stays =
    { $why ->
        [still-used] { $path } 會保留，因為保留下來的項目仍在使用它。
        [changed] { $path } 會保留，因為它在安裝後已變更。
       *[not-asked] { $path } 會保留，因為沒有要求移除它。
    }
# The file it replaced was to be put beside it, under another name, and every name was taken.
core-rollback-file-stays-no-name-beside =
    { $why ->
        [still-used] { $path } 會保留，因為保留下來的項目仍在使用它，且無法將它所取代的檔案放在旁邊：該檔案可用的名稱都已被使用。
        [changed] { $path } 會保留，因為它在安裝後已變更，且無法將它所取代的檔案放在旁邊：該檔案可用的名稱都已被使用。
       *[not-asked] { $path } 會保留，因為沒有要求移除它，且無法將它所取代的檔案放在旁邊：該檔案可用的名稱都已被使用。
    }
# { $name } is the name the file it replaced now has, such as settings.ini.before-install.
core-rollback-file-stays-beside =
    { $why ->
        [still-used] { $path } 會保留，因為保留下來的項目仍在使用它。它所取代的檔案已放在旁邊，名稱為 { $name }。
        [changed] { $path } 會保留，因為它在安裝後已變更。它所取代的檔案已放在旁邊，名稱為 { $name }。
       *[not-asked] { $path } 會保留，因為沒有要求移除它。它所取代的檔案已放在旁邊，名稱為 { $name }。
    }
# { $error } is what went wrong.
core-rollback-file-stays-not-beside =
    { $why ->
        [still-used] { $path } 會保留，因為保留下來的項目仍在使用它，且無法將它所取代的檔案放在旁邊：{ $error }。
        [changed] { $path } 會保留，因為它在安裝後已變更，且無法將它所取代的檔案放在旁邊：{ $error }。
       *[not-asked] { $path } 會保留，因為沒有要求移除它，且無法將它所取代的檔案放在旁邊：{ $error }。
    }

## The registry and environment variables. { $kind } says what { $name } is: key (a registry key, such as
## HKLM\Software\App), value (a registry value, such as HKLM\Software\App\Version), user-variable or machine-variable
## (an environment variable of the user's or of the whole PC, such as APP_HOME), or user-list or machine-list (a
## variable that is a list of folders, such as Path). { $error } is what Windows said went wrong.

# How a registry key's unnamed value is shown, as Registry Editor shows it.
core-registry-default-value-name = (預設值)
core-rollback-setting-other-account =
    { $kind ->
        [key] 機碼 { $name } 會保留，因為它屬於安裝此項目的帳戶，而該帳戶目前未登入。
        [user-variable] 使用者環境變數 { $name } 會保留，因為它屬於安裝此項目的帳戶，而該帳戶目前未登入。
        [machine-variable] 系統環境變數 { $name } 會保留，因為它屬於安裝此項目的帳戶，而該帳戶目前未登入。
        [user-list] 使用者的 { $name } 會保留，因為它屬於安裝此項目的帳戶，而該帳戶目前未登入。
        [machine-list] 系統的 { $name } 會保留，因為它屬於安裝此項目的帳戶，而該帳戶目前未登入。
       *[value] 登錄值 { $name } 會保留，因為它屬於安裝此項目的帳戶，而該帳戶目前未登入。
    }
core-rollback-setting-not-reached =
    { $kind ->
        [key] 無法存取機碼 { $name }：{ $error }。
        [user-variable] 無法存取使用者環境變數 { $name }：{ $error }。
        [machine-variable] 無法存取系統環境變數 { $name }：{ $error }。
        [user-list] 無法存取使用者的 { $name }：{ $error }。
        [machine-list] 無法存取系統的 { $name }：{ $error }。
       *[value] 無法存取登錄值 { $name }：{ $error }。
    }
core-rollback-setting-not-read =
    { $kind ->
        [key] 無法讀取機碼 { $name }：{ $error }。
        [user-variable] 無法讀取使用者環境變數 { $name }：{ $error }。
        [machine-variable] 無法讀取系統環境變數 { $name }：{ $error }。
        [user-list] 無法讀取使用者的 { $name }：{ $error }。
        [machine-list] 無法讀取系統的 { $name }：{ $error }。
       *[value] 無法讀取登錄值 { $name }：{ $error }。
    }
core-rollback-setting-changed =
    { $kind ->
        [user-variable] 使用者環境變數 { $name } 會保留，因為它在安裝後已變更。
        [machine-variable] 系統環境變數 { $name } 會保留，因為它在安裝後已變更。
        [user-list] 使用者的 { $name } 會保留，因為它在安裝後已變更。
        [machine-list] 系統的 { $name } 會保留，因為它在安裝後已變更。
       *[value] 登錄值 { $name } 會保留，因為它在安裝後已變更。
    }
# Said when it is back as it was before the install.
core-rollback-setting-put-back =
    { $kind ->
        [user-variable] 已恢復使用者環境變數 { $name }
        [machine-variable] 已恢復系統環境變數 { $name }
        [user-list] 已恢復使用者的 { $name }
        [machine-list] 已恢復系統的 { $name }
       *[value] 已恢復登錄值 { $name }
    }
core-rollback-setting-not-put-back =
    { $kind ->
        [user-variable] 無法恢復使用者環境變數 { $name }：{ $error }。
        [machine-variable] 無法恢復系統環境變數 { $name }：{ $error }。
        [user-list] 無法恢復使用者的 { $name }：{ $error }。
        [machine-list] 無法恢復系統的 { $name }：{ $error }。
       *[value] 無法恢復登錄值 { $name }：{ $error }。
    }
# A registry value the install wrote, which is the user's data, and stays unless they asked for it to go.
core-rollback-value-kept-by-request = 登錄值 { $name } 會保留，因為沒有要求移除它。
core-rollback-key-holds-others = 機碼 { $name } 會保留，因為其中有不是此安裝放入的內容。
core-rollback-key-removed = 已移除機碼 { $name }
core-rollback-key-not-removed = 無法移除機碼 { $name }：{ $error }。
# { $kind } is user-list or machine-list.
core-rollback-list-entries-removed =
    { $kind ->
        [user-list] 已從使用者的 { $name } 中移除此安裝的項目
       *[machine-list] 已從系統的 { $name } 中移除此安裝的項目
    }
core-rollback-list-entries-not-removed =
    { $kind ->
        [user-list] 無法從使用者的 { $name } 中移除此安裝的項目：{ $error }。
       *[machine-list] 無法從系統的 { $name } 中移除此安裝的項目：{ $error }。
    }

## Services and drivers. { $kind } is service or driver; { $name } is its name, such as AppUpdater. { $error } and
## { $reason } are what Windows said went wrong.

core-rollback-service-not-read =
    { $kind ->
        [driver] 無法讀取驅動程式 { $name }：{ $error }。
       *[service] 無法讀取服務 { $name }：{ $error }。
    }
core-rollback-service-changed =
    { $kind ->
        [driver] 驅動程式 { $name } 會保留，因為它在安裝後已變更。
       *[service] 服務 { $name } 會保留，因為它在安裝後已變更。
    }
core-rollback-service-not-stopped =
    { $kind ->
        [driver] 無法停止驅動程式 { $name }（{ $reason }），因此它正在使用的項目可能會保留到電腦重新啟動為止。
       *[service] 無法停止服務 { $name }（{ $reason }），因此它正在使用的項目可能會保留到電腦重新啟動為止。
    }
core-rollback-service-stopped-for-this =
    { $kind ->
        [driver] 已停止驅動程式 { $name } 以進行此作業。需要時請再次啟動它。
       *[service] 已停止服務 { $name } 以進行此作業。需要時請再次啟動它。
    }
core-rollback-service-not-removed =
    { $kind ->
        [driver] 無法移除驅動程式 { $name }：{ $reason }。
       *[service] 無法移除服務 { $name }：{ $reason }。
    }
core-rollback-service-marked-for-removal =
    { $kind ->
        [driver] 驅動程式 { $name } 已標示為待移除，會在電腦重新啟動或開啟它的程式將它釋放時移除。
       *[service] 服務 { $name } 已標示為待移除，會在電腦重新啟動或開啟它的程式將它釋放時移除。
    }
core-rollback-service-removed =
    { $kind ->
        [driver] 已移除驅動程式 { $name }
       *[service] 已移除服務 { $name }
    }
core-rollback-service-not-restored =
    { $kind ->
        [driver] 無法將驅動程式 { $name } 恢復原狀：{ $reason }。
       *[service] 無法將服務 { $name } 恢復原狀：{ $reason }。
    }
core-rollback-service-restored =
    { $kind ->
        [driver] 已恢復驅動程式 { $name } 的設定
       *[service] 已恢復服務 { $name } 的設定
    }
core-rollback-service-not-restarted = 服務 { $name } 在此安裝之前正在執行，但未再次啟動：{ $reason }。
# { $account } is the account the service ran as, such as CONTOSO\backup.
core-rollback-service-account-again = 服務 { $name } 原本以 { $account } 身分執行。其密碼並未保存，因此請在「服務」中重新設定其帳戶。

## Firewall rules. { $name } is the rule's name; { $error } is what went wrong.

core-rollback-firewall-not-read = 無法讀取防火牆規則 { $name }：{ $error }。
core-rollback-firewall-says-otherwise = 防火牆規則 { $name } 會保留，因為現有的同名規則內容不同。
core-rollback-firewall-changed = 防火牆規則 { $name } 會保留，因為它在安裝後已變更。
# The capital "The" is as the app has always said it.
core-rollback-firewall-removed = 已移除防火牆規則 { $name }
core-rollback-firewall-not-removed = 無法移除防火牆規則 { $name }：{ $error }。

## Scheduled tasks. { $path } is a folder of tasks, and { $name } a task with its folder, such as \App\Nightly Update.
## { $error } and { $reason } are what went wrong.

core-rollback-task-folder-holds-others = 工作資料夾 { $path } 會保留，因為其中有不是此安裝放入的工作。
core-rollback-task-folder-not-removed = 無法移除工作資料夾 { $path }：{ $error }。
core-rollback-task-not-read = 無法讀取工作 { $name }：{ $error }。
core-rollback-task-changed = 工作 { $name } 會保留，因為它在安裝後已變更。
core-rollback-task-removed = 已移除工作 { $name }
core-rollback-task-not-removed = 無法移除工作 { $name }：{ $reason }。
core-rollback-task-put-back = 已恢復工作 { $name }
core-rollback-task-not-put-back = 無法恢復工作 { $name }：{ $reason }。

## When an install cannot be undone at all

# { $error } is what went wrong.
core-rollback-record-unreadable = 無法讀取此安裝的記錄，因此未復原任何項目：{ $error }。
# { $error } is what went wrong.
core-rollback-nothing-undone = 未復原任何項目：{ $error }。
# One app's part of the message below. { $app_name } is the app's name; { $failures } is what could not be undone, a
# sentence or more; { $folder } is the folder that holds the install's record.
core-rollback-earlier-install-stuck = { $app_name }：{ $failures }如果它始終無法復原，系統管理員可以移除 { $folder } 來放棄它。
# Why an install did not start. { $details } is one or more of the message above.
core-rollback-earlier-installs-not-undone = 先前未完成的安裝無法完全復原，因此未安裝任何內容。請關閉正在使用這些項目的程式，然後再試一次。{ $details }
core-rollback-install-busy = 另一個安裝程式或解除安裝程式正在變更此安裝

## The summary an uninstall writes on the command line, one line each, with what each says below it. { $count } is how
## many.

core-rollback-count-undone = 已復原：{ $count }
core-rollback-count-failed = 無法復原：{ $count }
# What was left on purpose: it has changed since the install, or it is the user's.
core-rollback-count-kept = 保留原狀：{ $count }
# What no uninstall can take back, such as a runtime that was installed, which other programs may use.
core-rollback-count-left-behind = 遺留：{ $count }

## Which Windows a PC or drive has, as the app shows it beside the drive.

# { $name } is the Windows' name, such as Windows 7 Professional Service Pack 1.
core-system-windows-32-bit = { $name }，32 位元
# { $name } is the Windows' name, such as Windows 11 Pro.
core-system-windows-64-bit = { $name }，64 位元
# A drive's Windows, when it only says it is older than Vista.
core-system-windows-xp-or-older = Windows XP 或更舊版本

## Reading a connected drive, or another account's settings. Each is why it could not be done, and some start with a
## small letter, as they follow other words.

core-system-needs-administrator = 讀取連接的磁碟機或其他帳戶的設定需要系統管理員權限
core-system-no-program-data = 找不到這台電腦的 ProgramData 資料夾
# { $path } is a file, such as D:\Windows\System32\config\SOFTWARE.
core-system-not-a-hive-file = 不是登錄區檔案：{ $path }
# { $path } is a file; { $size } is its size in bytes.
core-system-hive-too-large = { $path } 有 { $size } 個位元組，以登錄區而言太大，或超過此磁碟機的登錄區可能的總大小
# { $key } is a registry key, such as Software\Microsoft\Windows\CurrentVersion.
core-system-hive-no-mapping = 離線登錄區沒有 HKLM\{ $key } 的對應
# { $key } is where a drive's registry was to be loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said.
# "reg load" is a command's name.
core-system-hive-not-loaded = reg load { $key } 失敗：{ $error }
# { $key } is where a drive's registry was loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said. "reg
# unload" is a command's name.
core-system-hive-not-unloaded = reg unload { $key } 失敗：{ $error }
# { $drive } is a drive, such as D:\.
core-system-not-a-system-drive = { $drive } 不是 Windows 系統磁碟機（沒有 \Windows 資料夾）。
# { $drive } is a drive, such as C:\. "This PC" is the name of a choice in the app.
core-system-own-system-drive = { $drive } 是這台電腦本身的系統磁碟機。若要讀取這台電腦，請選擇「本機」。
# { $drive } is a drive, such as D:\.
core-system-registry-through-link = { $drive } 不是 Windows 系統磁碟機：通往其登錄的路徑經過連結。

## Warnings a scan records about whose data it read.

# { $account } is an account's name or id, such as alice or S-1-5-21-...-1001.
core-warning-scan-account-not-on-record = 這裡沒有帳戶 { $account } 的記錄，因此未包含任何人的個別使用者資料或設定。
# { $account } is an account's name, such as CONTOSO\alice.
core-warning-scan-signed-in-account-no-profile = 在這裡登入的帳戶 { $account } 沒有設定檔記錄，因此未包含任何人的個別使用者資料或設定。
# { $reason } is why Windows could not say.
core-warning-scan-signed-in-account-unknown = 無法判斷目前登入的使用者（{ $reason }），因此未包含任何人的個別使用者資料或設定。請選擇要包含誰的資料。
core-warning-scan-last-account-unknown = 此磁碟機未記錄最後登入的使用者，且有多個帳戶，因此未包含任何人的個別使用者資料或設定。請選擇要包含誰的資料。
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-settings-unreadable = 無法讀取 { $account } 的設定，因此未包含：{ $error }
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-classes-unreadable = 無法讀取為 { $account } 註冊的 COM 類別和檔案類型，因此未包含：{ $error }

## Paths a scan did not follow. { $count } is how many; { $paths } is some of them, separated by semicolons; { $more } is
## how many more there are, not shown.

core-warning-scan-drive-paths-not-followed =
    { $count ->
       *[other] 此磁碟機上記錄的路徑無法讀取，因此未包含其中的任何內容。這些路徑都位於其他磁碟上、不是完整路徑，或經過會離開此磁碟機的連結：{ $paths }
    }
core-warning-scan-drive-paths-not-followed-more = 此磁碟機上記錄的部分路徑無法讀取，因此未包含其中的任何內容。這些路徑都位於其他磁碟上、不是完整路徑，或經過會離開此磁碟機的連結：{ $paths } 以及其他 { $more } 個
core-warning-scan-network-paths-not-followed =
    { $count ->
       *[other] 這台電腦上記錄的路徑無法讀取，因此未包含其中的任何內容。這些路徑都不在這台電腦的磁碟上（查看它們會連線到該處），或其名稱會開啟其他項目：{ $paths }
    }
core-warning-scan-network-paths-not-followed-more = 這台電腦上記錄的部分路徑無法讀取，因此未包含其中的任何內容。這些路徑都不在這台電腦的磁碟上（查看它們會連線到該處），或其名稱會開啟其他項目：{ $paths } 以及其他 { $more } 個

## What a scan with none of an account's data leaves out of a package, in the app's list of what was left out. Each
## has a name, and below it why.

core-account-left-out-shortcuts = 此帳戶自己資料夾中的捷徑
# { $count } is how many shortcuts.
core-account-left-out-shortcuts-why =
    { $count ->
       *[other] { $count } 個捷徑，只會隨「使用者資料」選項包含
    }
core-account-left-out-variables = 此帳戶的環境變數
# { $count } is how many variables.
core-account-left-out-variables-why =
    { $count ->
       *[other] { $count } 個變數，只會隨「使用者資料」選項包含
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-account-left-out-task = 排定的工作 { $task }
core-account-task-works-on-account-folder = 它處理帳戶資料夾中的某個位置，或以該位置命名，只會隨「使用者資料」選項包含
core-account-task-runs-from-account-folder = 它另外從帳戶資料夾執行的項目，只會隨「使用者資料」選項包含
# { $service } is a service's name, such as AppUpdater.
core-account-left-out-service = 服務 { $service }
core-account-service-works-on-account-folder = 它處理帳戶資料夾中的某個位置，只會隨「使用者資料」選項包含

## What a package was found to still say of whose PC it came from, after a build with none of an account's data.

# The parts of a package, as the warning below names them.
core-account-part-files = 應用程式本身檔案的路徑
core-account-part-registry = 登錄值
core-account-part-settings-entry = 應用程式在「設定」中的項目
core-account-part-main-program = 其主程式的位置
core-account-part-shortcuts = 捷徑
core-account-part-tasks = 排定的工作
core-account-part-services = 服務
core-account-part-variables = 環境變數
# One place in the list below. { $place } is one of the parts above, or a file's name; { $count } is how many times.
core-account-mention-place = { $place }（{ $count }）
# { $kind } is what was found: account (an account's name), folder (where an account's folder is), computer (the PC's
# name), or id (an account's id). { $places } is a list of where, from the message above.
core-warning-scan-package-mentions =
    { $kind ->
        [account] 此套件的以下位置提及帳戶名稱：{ $places }。此套件建置時未包含任何帳戶的資料，而任何擁有此安裝程式的人都能讀取這些內容。
        [folder] 此套件的以下位置提及帳戶資料夾的位置：{ $places }。此套件建置時未包含任何帳戶的資料，而任何擁有此安裝程式的人都能讀取這些內容。
        [computer] 此套件的以下位置提及電腦名稱：{ $places }。此套件建置時未包含任何帳戶的資料，而任何擁有此安裝程式的人都能讀取這些內容。
       *[id] 此套件的以下位置提及帳戶識別碼：{ $places }。此套件建置時未包含任何帳戶的資料，而任何擁有此安裝程式的人都能讀取這些內容。
    }
# { $count } is how many folders.
core-warning-scan-folders-of-others-unreadable =
    { $count ->
       *[other] 應用程式以外的 { $count } 個資料夾無法讀取，因此可能缺少應用程式在該處的資料。
    }

## What a scan leaves out because it looks like a credential, or is a license with the License option off, in the app's
## list of what was left out. Each has a name, and below it why.

core-secret-why-credential = 它看起來像認證
core-secret-why-license-option-off = 「授權與啟用資料」選項已關閉
# { $count } is how many values of a registry key.
core-secret-why-account-values =
    { $count ->
       *[other] { $count } 個值，只會隨「使用者資料」選項包含
    }
# Shown in place of a registry value's name when its name itself looks like a password or a key.
core-secret-name-holds-credential = （名稱含有認證的值）
# Shown in place of an option's name, for an argument on a command line that has no name.
core-secret-unnamed-argument = 含有認證的引數
# { $option } is an option taken out of a command line, such as /password, or the message above. { $place } says
# where the command line was: service, task, shortcut, opener (what opens a file type), opener-unnamed (the same, of a
# type with no name), or uninstall. { $name } is the service, task, shortcut, file type, or app.
core-secret-taken-from =
    { $place ->
        [service] { $name } 執行方式中的 { $option }
        [task] 工作 { $name } 執行方式中的 { $option }
        [shortcut] 捷徑 { $name } 中的 { $option }
        [opener] 開啟 { $name } 之程式中的 { $option }
        [opener-unnamed] 開啟某檔案類型之程式中的 { $option }
       *[uninstall] { $name } 解除安裝方式中的 { $option }
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-secret-task-definition-unreadable = 工作 { $task } 的定義，無法讀取以進行檢查
# { $name } is an environment variable's name, such as APP_TOKEN.
core-secret-left-out-variable = 環境變數 { $name }
# { $class } is a COM class's id, such as {00000000-0000-0000-0000-000000000000}.
core-secret-left-out-class-server = 類別 { $class } 宣稱執行的項目
# { $rule } is a firewall rule's name.
core-secret-left-out-firewall-description = 防火牆規則 { $rule } 的描述

## Licenses a scan found, as the package's report describes each.

# { $name } is a file's name, such as license.lic.
core-license-detail-file = 偵測到授權檔案：{ $name }
# { $framework } is a licensing product's name, such as FlexLM / FlexNet.
core-license-detail-store = 有 { $framework } 授權存放區
# { $name } is an environment variable's name, such as LM_LICENSE_FILE.
core-license-detail-variable = 授權環境變數 { $name }
# { $name } is a registry value's name, such as SerialNumber.
core-license-detail-registry-value = 登錄值疑似含有授權資料：{ $name }
# Said of a registry value in an account's key, which is not named.
core-license-detail-registry-value-of-an-account = 登錄值疑似含有授權資料
# { $key } is a registry key, such as HKLM\Software\Contoso\License.
core-license-detail-registry-key = 登錄機碼疑似含有授權資料：{ $key }

## Warnings a scan records about licenses.

core-warning-scan-license-data = 偵測到授權/啟用資料。開啟「授權與啟用資料」選項時，安裝程式會還原可移轉的內容；整台電腦範圍的值和與硬體綁定的存放區只會回報。WinAppMigrator 絕不會解密、略過啟用或模擬硬體鎖。
core-warning-scan-license-machine-bound = 部分授權與電腦綁定（硬體鎖、鎖定電腦的啟用或經過加密）。安裝程式會盡可能還原；這些授權在新電腦上仍可能需要重新啟用。

## Warnings a scan records about the registry. What could not be read is not named in them: it is kept beside them.

# { $error } is what Windows said went wrong. App Paths is a registry key's name.
core-warning-scan-app-paths-key-unreadable = 無法讀取 App Paths 子機碼：{ $error }
core-warning-scan-registry-limit = 登錄擷取已達設定的安全上限。
core-warning-scan-registry-subtree-limit = 登錄子樹擷取已達設定的安全上限。
# { $error } is what Windows said went wrong.
core-warning-scan-registry-key-unreadable = 無法讀取登錄機碼：{ $error }
core-warning-scan-registry-value-too-large = 登錄值超過設定的擷取大小上限。

## Scheduled tasks

core-warning-scan-tasks-folder-through-link = 此磁碟機上排定的工作資料夾經過連結，因此未讀取其中的工作。
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-warning-scan-task-left-out = 排定的工作 { $task } 已排除：它以其他帳戶執行，而不是要包含其設定的帳戶；或系統管理員以外的人可以變更其檔案；或此應用程式的工作太多，無法全部保留。
# { $error } is what Windows said went wrong.
core-warning-scan-tasks-folder-unreadable = 無法讀取部分排定的工作：{ $error }
# { $path } is a task's file, such as C:\Windows\System32\Tasks\App\Nightly Sync; { $reason } is one of the reasons
# below, or what Windows said went wrong.
core-warning-scan-task-unreadable = 無法讀取排定的工作 { $path }：{ $reason }。如果它會執行此應用程式，則未包含在內。
# Why a task's file could not be read. { $size } is its size in bytes.
core-task-too-large = 它有 { $size } 個位元組，以工作而言太大
# Why a task's file could not be read.
core-task-ends-mid-character = 它在字元中途結束
# Why a task's file could not be read. { $error } says where.
core-task-not-utf16 = 它不是有效的 UTF-16：{ $error }
# Why a task's file could not be read. { $error } says where.
core-task-not-text = 它不是有效的 UTF-8 或 UTF-16：{ $error }
# What a package records of how a task is put back. Register-ScheduledTask is a command's name.
core-task-restore-instructions = 還原時會使用 Register-ScheduledTask 搭配封存的 XML 定義。

## Reading a program's file. Each is why it could not be read, after the program's name.

# { $reason } is what went wrong.
core-pe-unreadable = 無法讀取：{ $reason }
core-pe-headers-misaligned = 其標頭未對齊程式應有的邊界
# { $size } is the file's size in bytes.
core-pe-too-large = 它有 { $size } 個位元組，以程式而言太大
# PE is the name of the format Windows programs are in.
core-pe-no-headers = 它沒有 PE 標頭

## Warnings a scan records about what a program needs. Which program is kept beside each.

# { $error } is what went wrong.
core-warning-scan-imports-unreadable = 無法讀取匯入表：{ $error }
# { $error } is what went wrong.
core-warning-scan-import-name-unreadable = 無法讀取匯入描述元名稱：{ $error }
# { $count } is how many were skipped.
core-warning-scan-legacy-delay-imports =
    { $count ->
       *[other] 已略過 { $count } 個舊版（v1）延遲載入描述元；只會剖析新式以 RVA 為基礎的延遲匯入。
    }

## The DLLs a program needs. Which DLL or program is kept beside each warning.

# { $path } is a program, such as C:\Program Files\App\app.exe.
core-dll-program-has-no-folder = 執行檔沒有上層目錄：{ $path }
# Why a DLL is among the app's files, in the package's list of them.
core-dll-beside-program = 位於所選執行檔旁邊的 DLL
# { $error } is what went wrong.
core-warning-scan-dependency-unscanned = 無法掃描遞迴相依性：{ $error }
# { $name } is a DLL's name, such as d3dx9_43.dll.
core-warning-scan-dependency-not-in-newer-windows = { $name } 位於此應用程式原電腦的 Windows 資料夾中，而 Windows 10 和 11 並未內建它。它是由其他項目安裝在該處，因此不屬於此應用程式本身的檔案，不會一併帶過去。在安裝提供它的項目之前，此應用程式可能無法啟動。
# { $error } is what went wrong.
core-warning-scan-dependency-not-hashed = 無法計算相依性的雜湊：{ $error }
# SystemRoot and windir are the names of environment variables.
core-warning-scan-windows-folder-unknown = 無法取得 SystemRoot/windir 環境變數；系統 DLL 搜尋受到限制。
# { $error } is what went wrong.
core-warning-scan-dependency-architecture-unreadable = 無法讀取相依性的架構：{ $error }

## Services, drivers, shortcuts, and environment variables a scan reads. Where it was reading is kept beside each
## warning.

# { $error } is what Windows said went wrong.
core-warning-scan-services-unreadable = 無法讀取服務的登錄機碼：{ $error }
# { $error } is what Windows said went wrong.
core-warning-scan-drivers-unreadable = 無法讀取用於探索驅動程式的服務登錄機碼：{ $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-queried = 無法查詢捷徑：{ $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-parsed = 無法剖析捷徑查詢的輸出：{ $error }
# { $error } is what Windows said went wrong.
core-warning-scan-environment-unreadable = 無法讀取系統環境變數的登錄機碼：{ $error }

## What a package records of how a driver is put back, which an installer shows. { $kind } is the driver's type as
## Windows names it (kernel, file-system, recognizer, or unknown-type); { $name } is its service's name; { $path } is
## its file. { $signed } is yes when { $signer } names who signed it, and no when that is not known.

core-service-driver-bundled =
    { $signed ->
        [yes] 應用程式擁有的 { $kind } 驅動程式「{ $name }」（{ $path }，由 { $signer } 簽署）。安裝程式會隨附其 .sys，並自動重新註冊驅動程式服務（sc create，需要提高權限），然後確認服務已註冊。
       *[no] 應用程式擁有的 { $kind } 驅動程式「{ $name }」（{ $path }，簽章不明）。安裝程式會隨附其 .sys，並自動重新註冊驅動程式服務（sc create，需要提高權限），然後確認服務已註冊。
    }
core-service-driver-from-vendor =
    { $signed ->
        [yes] 廠商/內建的 { $kind } 驅動程式「{ $name }」（{ $path }，由 { $signer } 簽署）。WinAppMigrator 不會轉散發協力廠商驅動程式；請在啟動應用程式之前，在目標電腦上使用廠商已簽署的驅動程式套件重新安裝。
       *[no] 廠商/內建的 { $kind } 驅動程式「{ $name }」（{ $path }，簽章不明）。WinAppMigrator 不會轉散發協力廠商驅動程式；請在啟動應用程式之前，在目標電腦上使用廠商已簽署的驅動程式套件重新安裝。
    }

## Runtimes an app needs, such as the Visual C++ runtime or .NET: how each is provided, as the Dependencies list says.

core-runtime-handling-built-in = 內建於 Windows
core-runtime-handling-ships-with-app = 隨應用程式提供
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-handling-windows-feature = Windows 功能：{ $feature }
core-runtime-handling-included = 已包含
# { $id } is the runtime's name in winget, such as Microsoft.VCRedist.2015+.x64.
core-runtime-handling-winget = 透過 winget 安裝：{ $id }
core-runtime-handling-manual = 手動安裝

## Downloading a runtime's installer, when an installer is built. Each is why it could not be, and starts with a small
## letter, as it follows other words. { $url } is a web address; { $error } is what went wrong.

core-runtime-download-not-https = 拒絕下載 { $url }：它不是 HTTPS
# TLS is what keeps a download private.
core-runtime-tls-failed = TLS 初始化失敗：{ $error }
core-runtime-download-failed = 下載 { $url } 失敗：{ $error }
core-runtime-download-unreadable = 讀取 { $url } 失敗：{ $error }
core-runtime-download-empty = 下載 { $url } 時未傳回任何資料
# { $came } and { $expected } are numbers of bytes.
core-runtime-download-cut-short = 下載 { $url } 時中斷：{ $expected } 個位元組中只收到 { $came } 個

## Installing a runtime: why it was not installed. Each starts with a small letter, as it follows other words.

core-runtime-no-known-installer = 此執行階段沒有已知的安裝程式
# { $path } is where in the package, such as prerequisites\vc_redist.x64.exe.
core-runtime-payload-misplaced = 套件指定 { $path } 為此執行階段的安裝程式，但該位置不是它應在的位置，因此未執行
# { $path } is a file.
core-runtime-payload-missing = 缺少隨附的安裝程式：{ $path }
# { $error } is what went wrong.
core-runtime-payload-path-invalid = 隨附的安裝程式路徑無效：{ $error }
# { $url } is the web address it can be downloaded from. winget is a program's name.
core-runtime-no-installer-route = 沒有隨附的安裝程式或 winget 識別碼；請從 { $url } 取得
# { $error } is what went wrong.
core-runtime-no-temp-folder = 無法建立供它解壓縮的資料夾：{ $error }
# { $reason } is what the check found.
core-runtime-payload-signature-failed = 隨附的安裝程式未通過簽章檢查，因此未執行：{ $reason }
core-runtime-payload-timed-out = 隨附的安裝程式未在逾時前完成，已被終止
# { $error } is what went wrong.
core-runtime-payload-not-started = 無法啟動隨附的安裝程式：{ $error }
core-runtime-installer-killed = 安裝程式已被終止，沒有結束代碼
# { $code } is the number it ended with, such as 1603.
core-runtime-installer-failed = 安裝程式結束，代碼為 { $code }
# { $feature } is a Windows feature's name, such as NetFx3; { $code } is the number DISM ended with. Windows Features is
# the name of a Windows window. Keep the command as it is.
core-runtime-feature-failed = DISM 無法啟用 { $feature } Windows 功能（結束代碼 { $code }）；可能無法下載所需的承載。請透過「Windows 功能」啟用，或執行 `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`。
# The same, when DISM gave no number.
core-runtime-feature-failed-no-code = DISM 無法啟用 { $feature } Windows 功能（結束代碼不明）；可能無法下載所需的承載。請透過「Windows 功能」啟用，或執行 `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`。
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-feature-timed-out = 啟用 { $feature } Windows 功能未在逾時前完成，已被終止
# { $feature } is a Windows feature's name; { $error } is what went wrong.
core-runtime-feature-no-dism = 無法使用 DISM 來啟用 { $feature } Windows 功能：{ $error }

## Installing a runtime with winget. { $id } is its name in winget, such as Microsoft.VCRedist.2015+.x64.

# { $error } is what went wrong.
core-runtime-winget-not-looked-for = 無法尋找 winget：{ $error }
core-runtime-winget-not-installed = 未安裝 winget
# { $path } is where winget is.
core-runtime-winget-misplaced = { $path } 不是 Windows 存放其自身套件的位置，因此未執行
# { $path } is where winget is; { $reason } is what the check found.
core-runtime-winget-signature-failed = { $path } 未通過簽章檢查，因此未執行：{ $reason }
# { $reason } is one of the two messages above, or what went wrong.
core-runtime-winget-unusable = 無法使用 winget 安裝 { $id }：{ $reason }
# { $code } is the number winget ended with.
core-runtime-winget-failed = winget install { $id } 結束，代碼為 { $code }
core-runtime-winget-failed-no-code = winget install { $id } 結束，代碼不明
core-runtime-winget-timed-out = winget install { $id } 未在逾時前完成，已被終止
# { $error } is what went wrong.
core-runtime-winget-not-started = 無法取得 winget 來安裝 { $id }：{ $error }

## What told a scan how an app was installed, as the package's report lists it. The names of installers and files are
## as their makers write them.

# { $path } is an installer's record of what it installed, such as C:\Program Files\App\unins000.dat.
core-source-record-not-believed = 未使用 { $path }：系統管理員以外的人可以變更它
# { $count } is how many Windows Installer packages the bundle holds.
core-source-burn-bundle =
    { $count ->
       *[other] Burn 組合套件，含 { $count } 個 Windows Installer 套件
    }
# { $count } is how many paths Windows Installer recorded.
core-source-msi-database =
    { $count ->
       *[other] Windows Installer 資料庫：{ $count } 個元件索引鍵路徑
    }
core-source-inno-log = Inno Setup 解除安裝記錄（unins000.dat）
# { $count } is how many paths the log records.
core-source-inno-log-paths =
    { $count ->
       *[other] Inno Setup 解除安裝記錄：{ $count } 個記錄的路徑
    }
core-source-nsis = NSIS (Nullsoft) 解除安裝程式特徵
# { $count } is how many paths the log records.
core-source-nsis-log-paths =
    { $count ->
       *[other] NSIS AdvUninstLog uninstall.log：{ $count } 個記錄的路徑
    }
core-source-squirrel = Squirrel 個別使用者配置（Update.exe / app-*）
# { $version } is a folder's name that says the version, such as app-1.2.3.
core-source-squirrel-version = Squirrel 目前版本：{ $version }
core-source-chocolatey = Chocolatey 套件中繼資料
core-source-msix = MSIX / Appx 套件
# { $identity } is a package's name, such as Contoso.App.
core-source-appx-identity = AppxManifest 身分識別：{ $identity }
# { $count } is how many folders.
core-source-msix-data =
    { $count ->
       *[other] MSIX 個別使用者套件資料：{ $count } 個位置
    }
# App Paths is a registry key's name.
core-source-app-paths = App Paths 登錄

## Why a runtime is needed, as the package's report says it.

# { $program } is a program's file name, such as app.exe; { $dll } is a DLL's, such as msvcp140.dll.
core-runtime-program-imports = { $program } 匯入 { $dll }

## What else of an app's a scan finds in the registry: fonts and certificates, and why each is the app's.

# Why a font is among the app's files, in the package's list of them.
core-registry-font-recorded = 應用程式安裝記錄中指定的字型
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-named-for-the-app = 它以此應用程式或其發行者命名
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-recorded = 應用程式的安裝記錄中指定了它

## Who is signed in, for the Whose Data choice. Each of the first six is why it could not be told, and starts with a
## small letter, as it is shown inside other words. An account is shown by its id, such as S-1-5-21-...-1001.

# { $error } is what Windows said.
core-profile-desktop-owner-not-said = Windows 未提供此桌面的擁有者：{ $error }
# { $error } is what went wrong.
core-profile-desktop-owner-not-asked = 無法向 Windows 查詢此桌面的擁有者：{ $error }
core-profile-signed-in-account-no-profile = 在這裡登入的帳戶 { $account } 沒有設定檔記錄
core-profile-desktop-not-a-person = 這裡的桌面屬於 { $account }，而這不是個人帳戶
core-profile-nobody-signed-in-running-account-no-profile = 這裡沒有人登入，而執行此程式的帳戶 { $account } 沒有設定檔記錄
core-profile-nobody-signed-in-running-not-a-person = 這裡沒有人登入，而執行此程式的 { $account } 不是個人帳戶
# SID is the name Windows gives an account's id.
core-profile-nobody-signed-in-no-sid = 這裡沒有人登入，且此帳戶沒有 SID
# { $accounts } is a list of accounts.
core-profile-several-desktops = 此工作階段中有多個帳戶擁有桌面：{ $accounts }
# Under the Whose Data choice. { $reason } is one of the messages above.
core-profile-signed-in-unknown = 無法判斷目前登入的使用者（{ $reason }）。
# Under the Whose Data choice, when there are accounts to choose from. { $reason } is one of the messages above.
core-profile-signed-in-unknown-choose = 無法判斷目前登入的使用者（{ $reason }）。請選擇要包含誰的資料。
