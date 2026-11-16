# Words the core writes for people: warnings, refusals, errors, and the installer's list of what it sets up.
# Message names start with core-.


## Warnings a package records when it is built, which an installer shows later, maybe on another PC, in that PC's
## language.

# { $count } is how many shortcuts.
core-warning-shortcuts-unreadable =
    { $count ->
       *[other] 此驱动器上有 { $count } 个快捷方式未指向任何文件，因此无法确定它们是否会打开此应用。
    }

## Undoing an install: what an uninstall, or a failed install, says it put back, left, or could not undo. Each line is
## shown on its own, in a list.

# { $app_name } is the app's name.
core-rollback-record-damaged = { $app_name } 的部分记录无法读取，因此其中描述的任何内容仍保留在原处，被替换的文件可能无法恢复。
core-rollback-settings-entry-stays = 此应用在“设置”中的条目将保留，以便可以从那里再次运行此操作。
# { $name } is a runtime's name, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-rollback-prerequisite-stays = { $name } 已安装（或可能已安装），并将保留，因为其他程序可能正在使用它。
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-drive-not-connected = 此安装放在 { $folder } 中的内容位于未连接的驱动器上。连接该驱动器后，即可完成此操作。
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-record-part-unread = 此安装放在 { $folder } 中的内容将保留，因为记录中的那一部分无法读取。

## Why a file or folder was left, shown after "was left:" in the messages below them. Each is part of a sentence, so
## starts with a small letter and has no full stop.

# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-being-changed = 运行期间有程序在更改 { $folder }，因此未在其中执行进一步操作。请关闭正在执行此更改的程序，然后再次运行
# { $folder } is a folder; { $error } is what Windows said went wrong.
core-rollback-reason-not-opened = 无法打开 { $folder }：{ $error }
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-no-record = 没有记录 { $folder } 原来的位置
# { $folder } is a folder; { $actual } is the folder it now leads to.
core-rollback-reason-moved = { $folder } 现在指向 { $actual }，而不是安装期间的位置
# Why a service was not stopped.
core-rollback-reason-did-not-stop = 它未能及时停止

## Files and folders

# { $path } is a folder; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-folder-left = 已保留文件夹 { $path }：{ $reason }。
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-removed = 已删除文件夹 { $path }
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-holds-others = 文件夹 { $path } 将保留，因为其中包含不是此安装放入的文件。
# { $path } is a folder; { $error } is what Windows said went wrong.
core-rollback-folder-not-removed = 无法删除文件夹 { $path }：{ $error }。
# { $path } is a file; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-file-left = 已保留文件 { $path }：{ $reason }。
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-folder-gone = 不会恢复 { $path }，因为它原来所在的文件夹已不存在。
# { $path } is a file; { $error } is what went wrong.
core-rollback-partial-not-removed = 无法删除不完整的文件：{ $path }（{ $error }）。
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-checked = 无法检查 { $path }：{ $error }。
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-removed = 已删除 { $path }
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-removed = 无法删除 { $path }：{ $error }。
# { $path } is a file this install had replaced, and has put back.
core-rollback-file-put-back = 已恢复 { $path }
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-put-back = 无法恢复 { $path }：{ $error }。
# Why a file could not be put back.
core-rollback-backup-gone = 备份已不存在
# { $path } is a file.
core-rollback-no-parent-folder = { $path } 没有父文件夹

## A file that stays. { $why } says why: still-used (something that stays still uses it), changed (it has changed since
## it was installed), or not-asked (it is someone's data, and they did not ask for it to go). { $path } is the file.

core-rollback-file-stays =
    { $why ->
        [still-used] { $path } 将保留，因为某项保留的内容仍在使用它。
        [changed] { $path } 将保留，因为它在安装后已被更改。
       *[not-asked] { $path } 将保留，因为没有要求删除它。
    }
# The file it replaced was to be put beside it, under another name, and every name was taken.
core-rollback-file-stays-no-name-beside =
    { $why ->
        [still-used] { $path } 将保留，因为某项保留的内容仍在使用它，且它所替换的文件无法放在它旁边：可用的名称都已被占用。
        [changed] { $path } 将保留，因为它在安装后已被更改，且它所替换的文件无法放在它旁边：可用的名称都已被占用。
       *[not-asked] { $path } 将保留，因为没有要求删除它，且它所替换的文件无法放在它旁边：可用的名称都已被占用。
    }
# { $name } is the name the file it replaced now has, such as settings.ini.before-install.
core-rollback-file-stays-beside =
    { $why ->
        [still-used] { $path } 将保留，因为某项保留的内容仍在使用它。它所替换的文件位于它旁边，名为 { $name }。
        [changed] { $path } 将保留，因为它在安装后已被更改。它所替换的文件位于它旁边，名为 { $name }。
       *[not-asked] { $path } 将保留，因为没有要求删除它。它所替换的文件位于它旁边，名为 { $name }。
    }
# { $error } is what went wrong.
core-rollback-file-stays-not-beside =
    { $why ->
        [still-used] { $path } 将保留，因为某项保留的内容仍在使用它，且它所替换的文件无法放在它旁边：{ $error }。
        [changed] { $path } 将保留，因为它在安装后已被更改，且它所替换的文件无法放在它旁边：{ $error }。
       *[not-asked] { $path } 将保留，因为没有要求删除它，且它所替换的文件无法放在它旁边：{ $error }。
    }

## The registry and environment variables. { $kind } says what { $name } is: key (a registry key, such as
## HKLM\Software\App), value (a registry value, such as HKLM\Software\App\Version), user-variable or machine-variable
## (an environment variable of the user's or of the whole PC, such as APP_HOME), or user-list or machine-list (a
## variable that is a list of folders, such as Path). { $error } is what Windows said went wrong.

# How a registry key's unnamed value is shown, as Registry Editor shows it.
core-registry-default-value-name = (默认)
core-rollback-setting-other-account =
    { $kind ->
        [key] 注册表项 { $name } 将保留，因为它属于安装此内容的帐户，而该帐户当前未登录。
        [user-variable] 用户环境变量 { $name } 将保留，因为它属于安装此内容的帐户，而该帐户当前未登录。
        [machine-variable] 系统环境变量 { $name } 将保留，因为它属于安装此内容的帐户，而该帐户当前未登录。
        [user-list] 用户的 { $name } 将保留，因为它属于安装此内容的帐户，而该帐户当前未登录。
        [machine-list] 系统的 { $name } 将保留，因为它属于安装此内容的帐户，而该帐户当前未登录。
       *[value] 注册表值 { $name } 将保留，因为它属于安装此内容的帐户，而该帐户当前未登录。
    }
core-rollback-setting-not-reached =
    { $kind ->
        [key] 无法访问注册表项 { $name }：{ $error }。
        [user-variable] 无法访问用户环境变量 { $name }：{ $error }。
        [machine-variable] 无法访问系统环境变量 { $name }：{ $error }。
        [user-list] 无法访问用户的 { $name }：{ $error }。
        [machine-list] 无法访问系统的 { $name }：{ $error }。
       *[value] 无法访问注册表值 { $name }：{ $error }。
    }
core-rollback-setting-not-read =
    { $kind ->
        [key] 无法读取注册表项 { $name }：{ $error }。
        [user-variable] 无法读取用户环境变量 { $name }：{ $error }。
        [machine-variable] 无法读取系统环境变量 { $name }：{ $error }。
        [user-list] 无法读取用户的 { $name }：{ $error }。
        [machine-list] 无法读取系统的 { $name }：{ $error }。
       *[value] 无法读取注册表值 { $name }：{ $error }。
    }
core-rollback-setting-changed =
    { $kind ->
        [user-variable] 用户环境变量 { $name } 将保留，因为它在安装后已被更改。
        [machine-variable] 系统环境变量 { $name } 将保留，因为它在安装后已被更改。
        [user-list] 用户的 { $name } 将保留，因为它在安装后已被更改。
        [machine-list] 系统的 { $name } 将保留，因为它在安装后已被更改。
       *[value] 注册表值 { $name } 将保留，因为它在安装后已被更改。
    }
# Said when it is back as it was before the install.
core-rollback-setting-put-back =
    { $kind ->
        [user-variable] 已恢复用户环境变量 { $name }
        [machine-variable] 已恢复系统环境变量 { $name }
        [user-list] 已恢复用户的 { $name }
        [machine-list] 已恢复系统的 { $name }
       *[value] 已恢复注册表值 { $name }
    }
core-rollback-setting-not-put-back =
    { $kind ->
        [user-variable] 无法恢复用户环境变量 { $name }：{ $error }。
        [machine-variable] 无法恢复系统环境变量 { $name }：{ $error }。
        [user-list] 无法恢复用户的 { $name }：{ $error }。
        [machine-list] 无法恢复系统的 { $name }：{ $error }。
       *[value] 无法恢复注册表值 { $name }：{ $error }。
    }
# A registry value the install wrote, which is the user's data, and stays unless they asked for it to go.
core-rollback-value-kept-by-request = 注册表值 { $name } 将保留，因为没有要求删除它。
core-rollback-key-holds-others = 注册表项 { $name } 将保留，因为其中包含不是此安装放入的内容。
core-rollback-key-removed = 已删除注册表项 { $name }
core-rollback-key-not-removed = 无法删除注册表项 { $name }：{ $error }。
# { $kind } is user-list or machine-list.
core-rollback-list-entries-removed =
    { $kind ->
        [user-list] 已从用户的 { $name } 中删除此安装的条目
       *[machine-list] 已从系统的 { $name } 中删除此安装的条目
    }
core-rollback-list-entries-not-removed =
    { $kind ->
        [user-list] 无法从用户的 { $name } 中删除此安装的条目：{ $error }。
       *[machine-list] 无法从系统的 { $name } 中删除此安装的条目：{ $error }。
    }

## Services and drivers. { $kind } is service or driver; { $name } is its name, such as AppUpdater. { $error } and
## { $reason } are what Windows said went wrong.

core-rollback-service-not-read =
    { $kind ->
        [driver] 无法读取驱动程序 { $name }：{ $error }。
       *[service] 无法读取服务 { $name }：{ $error }。
    }
core-rollback-service-changed =
    { $kind ->
        [driver] 驱动程序 { $name } 将保留，因为它在安装后已被更改。
       *[service] 服务 { $name } 将保留，因为它在安装后已被更改。
    }
core-rollback-service-not-stopped =
    { $kind ->
        [driver] 无法停止驱动程序 { $name }（{ $reason }），因此它正在使用的内容可能会保留，直到电脑重启。
       *[service] 无法停止服务 { $name }（{ $reason }），因此它正在使用的内容可能会保留，直到电脑重启。
    }
core-rollback-service-stopped-for-this =
    { $kind ->
        [driver] 为执行此操作，已停止驱动程序 { $name }。需要时请再次启动它。
       *[service] 为执行此操作，已停止服务 { $name }。需要时请再次启动它。
    }
core-rollback-service-not-removed =
    { $kind ->
        [driver] 无法删除驱动程序 { $name }：{ $reason }。
       *[service] 无法删除服务 { $name }：{ $reason }。
    }
core-rollback-service-marked-for-removal =
    { $kind ->
        [driver] 驱动程序 { $name } 已标记为待删除，将在电脑重启或打开它的程序将其释放时删除。
       *[service] 服务 { $name } 已标记为待删除，将在电脑重启或打开它的程序将其释放时删除。
    }
core-rollback-service-removed =
    { $kind ->
        [driver] 已删除驱动程序 { $name }
       *[service] 已删除服务 { $name }
    }
core-rollback-service-not-restored =
    { $kind ->
        [driver] 无法将驱动程序 { $name } 恢复原样：{ $reason }。
       *[service] 无法将服务 { $name } 恢复原样：{ $reason }。
    }
core-rollback-service-restored =
    { $kind ->
        [driver] 已恢复驱动程序 { $name } 的设置
       *[service] 已恢复服务 { $name } 的设置
    }
core-rollback-service-not-restarted = 服务 { $name } 在此安装之前正在运行，但未能再次启动：{ $reason }。
# { $account } is the account the service ran as, such as CONTOSO\backup.
core-rollback-service-account-again = 服务 { $name } 原先以 { $account } 身份运行。其密码未保存，因此请在“服务”中重新设置其帐户。

## Firewall rules. { $name } is the rule's name; { $error } is what went wrong.

core-rollback-firewall-not-read = 无法读取防火墙规则 { $name }：{ $error }。
core-rollback-firewall-says-otherwise = 防火墙规则 { $name } 将保留，因为使用该名称的现有规则内容不同。
core-rollback-firewall-changed = 防火墙规则 { $name } 将保留，因为它在安装后已被更改。
# The capital "The" is as the app has always said it.
core-rollback-firewall-removed = 已删除防火墙规则 { $name }
core-rollback-firewall-not-removed = 无法删除防火墙规则 { $name }：{ $error }。

## Scheduled tasks. { $path } is a folder of tasks, and { $name } a task with its folder, such as \App\Nightly Update.
## { $error } and { $reason } are what went wrong.

core-rollback-task-folder-holds-others = 任务文件夹 { $path } 将保留，因为其中包含不是此安装放入的任务。
core-rollback-task-folder-not-removed = 无法删除任务文件夹 { $path }：{ $error }。
core-rollback-task-not-read = 无法读取任务 { $name }：{ $error }。
core-rollback-task-changed = 任务 { $name } 将保留，因为它在安装后已被更改。
core-rollback-task-removed = 已删除任务 { $name }
core-rollback-task-not-removed = 无法删除任务 { $name }：{ $reason }。
core-rollback-task-put-back = 已恢复任务 { $name }
core-rollback-task-not-put-back = 无法恢复任务 { $name }：{ $reason }。

## When an install cannot be undone at all

# { $error } is what went wrong.
core-rollback-record-unreadable = 无法读取此安装的记录，因此未撤消任何内容：{ $error }。
# { $error } is what went wrong.
core-rollback-nothing-undone = 未撤消任何内容：{ $error }。
# One app's part of the message below. { $app_name } is the app's name; { $failures } is what could not be undone, a
# sentence or more; { $folder } is the folder that holds the install's record.
core-rollback-earlier-install-stuck = { $app_name }：{ $failures }如果它永远无法撤消，管理员可以删除 { $folder } 来放弃它。
# Why an install did not start. { $details } is one or more of the message above.
core-rollback-earlier-installs-not-undone = 之前未完成的安装无法完全撤消，因此未安装任何内容。请关闭正在使用这些内容的程序，然后重试。{ $details }
core-rollback-install-busy = 此安装正被另一个安装程序或卸载程序更改

## The summary an uninstall writes on the command line, one line each, with what each says below it. { $count } is how
## many.

core-rollback-count-undone = 已撤消：{ $count }
core-rollback-count-failed = 无法撤消：{ $count }
# What was left on purpose: it has changed since the install, or it is the user's.
core-rollback-count-kept = 保留原处：{ $count }
# What no uninstall can take back, such as a runtime that was installed, which other programs may use.
core-rollback-count-left-behind = 遗留：{ $count }

## Which Windows a PC or drive has, as the app shows it beside the drive.

# { $name } is the Windows' name, such as Windows 7 Professional Service Pack 1.
core-system-windows-32-bit = { $name }，32 位
# { $name } is the Windows' name, such as Windows 11 Pro.
core-system-windows-64-bit = { $name }，64 位
# A drive's Windows, when it only says it is older than Vista.
core-system-windows-xp-or-older = Windows XP 或更早版本

## Reading a connected drive, or another account's settings. Each is why it could not be done, and some start with a
## small letter, as they follow other words.

core-system-needs-administrator = 读取已连接的驱动器或其他帐户的设置需要管理员权限
core-system-no-program-data = 找不到此电脑的 ProgramData 文件夹
# { $path } is a file, such as D:\Windows\System32\config\SOFTWARE.
core-system-not-a-hive-file = 不是配置单元文件：{ $path }
# { $path } is a file; { $size } is its size in bytes.
core-system-hive-too-large = { $path } 有 { $size } 字节，太大，不可能是注册表配置单元，或者超出了此驱动器上所有配置单元可能的总大小
# { $key } is a registry key, such as Software\Microsoft\Windows\CurrentVersion.
core-system-hive-no-mapping = 脱机配置单元没有 HKLM\{ $key } 的映射
# { $key } is where a drive's registry was to be loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said.
# "reg load" is a command's name.
core-system-hive-not-loaded = reg load { $key } 失败：{ $error }
# { $key } is where a drive's registry was loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said. "reg
# unload" is a command's name.
core-system-hive-not-unloaded = reg unload { $key } 失败：{ $error }
# { $drive } is a drive, such as D:\.
core-system-not-a-system-drive = { $drive } 不是 Windows 系统驱动器（没有 \Windows 文件夹）。
# { $drive } is a drive, such as C:\. "This PC" is the name of a choice in the app.
core-system-own-system-drive = { $drive } 是此电脑自己的系统驱动器。若要读取此电脑，请选择“此电脑”。
# { $drive } is a drive, such as D:\.
core-system-registry-through-link = { $drive } 不是 Windows 系统驱动器：通向其注册表的路径经过链接。

## Warnings a scan records about whose data it read.

# { $account } is an account's name or id, such as alice or S-1-5-21-...-1001.
core-warning-scan-account-not-on-record = 此处没有帐户 { $account } 的记录，因此未包含任何人的按用户数据或设置。
# { $account } is an account's name, such as CONTOSO\alice.
core-warning-scan-signed-in-account-no-profile = 在此登录的帐户 { $account } 没有配置文件记录，因此未包含任何人的按用户数据或设置。
# { $reason } is why Windows could not say.
core-warning-scan-signed-in-account-unknown = 无法确定谁已登录（{ $reason }），因此未包含任何人的按用户数据或设置。请选择要包含谁的数据。
core-warning-scan-last-account-unknown = 此驱动器未记录最后登录的用户，且其中有多个帐户，因此未包含任何人的按用户数据或设置。请选择要包含谁的数据。
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-settings-unreadable = 无法读取 { $account } 的设置，因此未包含它们：{ $error }
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-classes-unreadable = 无法读取为 { $account } 注册的 COM 类和文件类型，因此未包含它们：{ $error }

## Paths a scan did not follow. { $count } is how many; { $paths } is some of them, separated by semicolons; { $more } is
## how many more there are, not shown.

core-warning-scan-drive-paths-not-followed =
    { $count ->
       *[other] 此驱动器上记录的以下路径无法读取，因此未包含该处的任何内容。每个路径要么位于其他磁盘上，要么不是完整路径，要么经过会离开此驱动器的链接：{ $paths }
    }
core-warning-scan-drive-paths-not-followed-more = 此驱动器上记录的以下路径无法读取，因此未包含该处的任何内容。每个路径要么位于其他磁盘上，要么不是完整路径，要么经过会离开此驱动器的链接：{ $paths }，另有 { $more } 个
core-warning-scan-network-paths-not-followed =
    { $count ->
       *[other] 此电脑上记录的以下路径无法读取，因此未包含该处的任何内容。每个路径要么不在此电脑的磁盘上（查看它就会连接过去），要么其名称会打开其他内容：{ $paths }
    }
core-warning-scan-network-paths-not-followed-more = 此电脑上记录的以下路径无法读取，因此未包含该处的任何内容。每个路径要么不在此电脑的磁盘上（查看它就会连接过去），要么其名称会打开其他内容：{ $paths }，另有 { $more } 个

## What a scan with none of an account's data leaves out of a package, in the app's list of what was left out. Each
## has a name, and below it why.

core-account-left-out-shortcuts = 此帐户自己文件夹中的快捷方式
# { $count } is how many shortcuts.
core-account-left-out-shortcuts-why =
    { $count ->
       *[other] { $count } 个快捷方式，仅在使用“用户数据”选项时才会包含
    }
core-account-left-out-variables = 此帐户的环境变量
# { $count } is how many variables.
core-account-left-out-variables-why =
    { $count ->
       *[other] { $count } 个变量，仅在使用“用户数据”选项时才会包含
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-account-left-out-task = 计划任务 { $task }
core-account-task-works-on-account-folder = 它处理帐户文件夹中的某个位置，或以其命名，仅在使用“用户数据”选项时才会包含
core-account-task-runs-from-account-folder = 它还会从帐户文件夹运行的内容，仅在使用“用户数据”选项时才会包含
# { $service } is a service's name, such as AppUpdater.
core-account-left-out-service = 服务 { $service }
core-account-service-works-on-account-folder = 它处理帐户文件夹中的某个位置，仅在使用“用户数据”选项时才会包含

## What a package was found to still say of whose PC it came from, after a build with none of an account's data.

# The parts of a package, as the warning below names them.
core-account-part-files = 应用自身文件的路径
core-account-part-registry = 注册表值
core-account-part-settings-entry = 应用在“设置”中的条目
core-account-part-main-program = 其主程序的位置
core-account-part-shortcuts = 快捷方式
core-account-part-tasks = 计划任务
core-account-part-services = 服务
core-account-part-variables = 环境变量
# One place in the list below. { $place } is one of the parts above, or a file's name; { $count } is how many times.
core-account-mention-place = { $place } ({ $count })
# { $kind } is what was found: account (an account's name), folder (where an account's folder is), computer (the PC's
# name), or id (an account's id). { $places } is a list of where, from the message above.
core-warning-scan-package-mentions =
    { $kind ->
        [account] 此程序包的以下位置提到了某个帐户的名称：{ $places }。它在生成时未包含任何帐户的数据，任何拥有此安装程序的人都可以读取此信息。
        [folder] 此程序包的以下位置提到了某个帐户文件夹的位置：{ $places }。它在生成时未包含任何帐户的数据，任何拥有此安装程序的人都可以读取此信息。
        [computer] 此程序包的以下位置提到了电脑的名称：{ $places }。它在生成时未包含任何帐户的数据，任何拥有此安装程序的人都可以读取此信息。
       *[id] 此程序包的以下位置提到了某个帐户的 ID：{ $places }。它在生成时未包含任何帐户的数据，任何拥有此安装程序的人都可以读取此信息。
    }
# { $count } is how many folders.
core-warning-scan-folders-of-others-unreadable =
    { $count ->
       *[other] 应用之外的 { $count } 个文件夹无法读取，因此其中属于此应用的数据可能缺失。
    }

## What a scan leaves out because it looks like a credential, or is a license with the License option off, in the app's
## list of what was left out. Each has a name, and below it why.

core-secret-why-credential = 它看起来像凭据
core-secret-why-license-option-off = “许可证和激活数据”选项已关闭
# { $count } is how many values of a registry key.
core-secret-why-account-values =
    { $count ->
       *[other] { $count } 个值，仅在使用“用户数据”选项时才会包含
    }
# Shown in place of a registry value's name when its name itself looks like a password or a key.
core-secret-name-holds-credential = （名称中包含凭据的值）
# Shown in place of an option's name, for an argument on a command line that has no name.
core-secret-unnamed-argument = 包含凭据的参数
# { $option } is an option taken out of a command line, such as /password, or the message above. { $place } says
# where the command line was: service, task, shortcut, opener (what opens a file type), opener-unnamed (the same, of a
# type with no name), or uninstall. { $name } is the service, task, shortcut, file type, or app.
core-secret-taken-from =
    { $place ->
        [service] { $name } 的运行命令中的 { $option }
        [task] 任务 { $name } 的运行命令中的 { $option }
        [shortcut] 快捷方式 { $name } 中的 { $option }
        [opener] 打开 { $name } 的命令中的 { $option }
        [opener-unnamed] 打开某个文件类型的命令中的 { $option }
       *[uninstall] { $name } 的卸载命令中的 { $option }
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-secret-task-definition-unreadable = 任务 { $task } 的定义，无法读取以进行检查
# { $name } is an environment variable's name, such as APP_TOKEN.
core-secret-left-out-variable = 环境变量 { $name }
# { $class } is a COM class's id, such as {00000000-0000-0000-0000-000000000000}.
core-secret-left-out-class-server = 类 { $class } 声明其运行的内容
# { $rule } is a firewall rule's name.
core-secret-left-out-firewall-description = 防火墙规则 { $rule } 的描述

## Licenses a scan found, as the package's report describes each.

# { $name } is a file's name, such as license.lic.
core-license-detail-file = 检测到许可证文件：{ $name }
# { $framework } is a licensing product's name, such as FlexLM / FlexNet.
core-license-detail-store = 存在 { $framework } 许可证存储
# { $name } is an environment variable's name, such as LM_LICENSE_FILE.
core-license-detail-variable = 许可证环境变量 { $name }
# { $name } is a registry value's name, such as SerialNumber.
core-license-detail-registry-value = 注册表值表明存在许可数据：{ $name }
# Said of a registry value in an account's key, which is not named.
core-license-detail-registry-value-of-an-account = 注册表值表明存在许可数据
# { $key } is a registry key, such as HKLM\Software\Contoso\License.
core-license-detail-registry-key = 注册表项表明存在许可数据：{ $key }

## Warnings a scan records about licenses.

core-warning-scan-license-data = 检测到许可/激活数据。打开“许可证和激活数据”选项后，安装程序会还原可以迁移的内容；适用于整台电脑的值和与硬件绑定的存储仅作报告。WinAppMigrator 从不解密、绕过激活或模拟加密狗。
core-warning-scan-license-machine-bound = 部分许可证与电脑绑定（硬件加密狗、锁定到电脑的激活或已加密）。安装程序会尽可能还原；这些许可证在新电脑上可能仍需重新激活。

## Warnings a scan records about the registry. What could not be read is not named in them: it is kept beside them.

# { $error } is what Windows said went wrong. App Paths is a registry key's name.
core-warning-scan-app-paths-key-unreadable = 无法读取 App Paths 子项：{ $error }
core-warning-scan-registry-limit = 注册表捕获已达到配置的安全上限。
core-warning-scan-registry-subtree-limit = 注册表子树捕获已达到配置的安全上限。
# { $error } is what Windows said went wrong.
core-warning-scan-registry-key-unreadable = 无法读取注册表项：{ $error }
core-warning-scan-registry-value-too-large = 注册表值超过了配置的捕获大小上限。

## Scheduled tasks

core-warning-scan-tasks-folder-through-link = 此驱动器上的计划任务文件夹经过链接，因此未读取其中的任务。
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-warning-scan-task-left-out = 计划任务 { $task } 已被排除：它以另一个帐户（而不是正在包含其设置的帐户）身份运行，或管理员以外的人可以更改其文件，或者此应用的任务太多，无法全部保留。
# { $error } is what Windows said went wrong.
core-warning-scan-tasks-folder-unreadable = 无法读取部分计划任务：{ $error }
# { $path } is a task's file, such as C:\Windows\System32\Tasks\App\Nightly Sync; { $reason } is one of the reasons
# below, or what Windows said went wrong.
core-warning-scan-task-unreadable = 无法读取计划任务 { $path }：{ $reason }。如果它运行此应用，则未包含它。
# Why a task's file could not be read. { $size } is its size in bytes.
core-task-too-large = 它有 { $size } 字节，太大，不可能是任务
# Why a task's file could not be read.
core-task-ends-mid-character = 它在某个字符的中间结束
# Why a task's file could not be read. { $error } says where.
core-task-not-utf16 = 它不是有效的 UTF-16：{ $error }
# Why a task's file could not be read. { $error } says where.
core-task-not-text = 它不是有效的 UTF-8 或 UTF-16：{ $error }
# What a package records of how a task is put back. Register-ScheduledTask is a command's name.
core-task-restore-instructions = 还原时使用 Register-ScheduledTask 和存档的 XML 定义。

## Reading a program's file. Each is why it could not be read, after the program's name.

# { $reason } is what went wrong.
core-pe-unreadable = 无法读取它：{ $reason }
core-pe-headers-misaligned = 它的标头未按程序标头所需的边界对齐
# { $size } is the file's size in bytes.
core-pe-too-large = 它有 { $size } 字节，太大，不可能是程序
# PE is the name of the format Windows programs are in.
core-pe-no-headers = 它没有 PE 标头

## Warnings a scan records about what a program needs. Which program is kept beside each.

# { $error } is what went wrong.
core-warning-scan-imports-unreadable = 无法读取导入表：{ $error }
# { $error } is what went wrong.
core-warning-scan-import-name-unreadable = 无法读取导入描述符名称：{ $error }
# { $count } is how many were skipped.
core-warning-scan-legacy-delay-imports =
    { $count ->
       *[other] 已跳过 { $count } 个旧版 (v1) 延迟加载描述符；仅解析基于 RVA 的新版延迟导入。
    }

## The DLLs a program needs. Which DLL or program is kept beside each warning.

# { $path } is a program, such as C:\Program Files\App\app.exe.
core-dll-program-has-no-folder = 可执行文件没有父目录：{ $path }
# Why a DLL is among the app's files, in the package's list of them.
core-dll-beside-program = 位于所选可执行文件旁边的 DLL
# { $error } is what went wrong.
core-warning-scan-dependency-unscanned = 无法扫描递归依赖项：{ $error }
# { $name } is a DLL's name, such as d3dx9_43.dll.
core-warning-scan-dependency-not-in-newer-windows = { $name } 位于此应用原电脑的 Windows 文件夹中，而 Windows 10 和 11 不自带它。它是由其他程序安装到那里的，因此不属于此应用自身的文件，不会一并包含。在安装提供它的程序之前，应用可能无法启动。
# { $error } is what went wrong.
core-warning-scan-dependency-not-hashed = 无法计算依赖项的哈希：{ $error }
# SystemRoot and windir are the names of environment variables.
core-warning-scan-windows-folder-unknown = SystemRoot/windir 环境变量不可用；系统 DLL 搜索受限。
# { $error } is what went wrong.
core-warning-scan-dependency-architecture-unreadable = 无法读取依赖项的体系结构：{ $error }

## Services, drivers, shortcuts, and environment variables a scan reads. Where it was reading is kept beside each
## warning.

# { $error } is what Windows said went wrong.
core-warning-scan-services-unreadable = 无法读取服务注册表项：{ $error }
# { $error } is what Windows said went wrong.
core-warning-scan-drivers-unreadable = 无法读取用于发现驱动程序的服务注册表项：{ $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-queried = 无法查询快捷方式：{ $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-parsed = 无法解析快捷方式查询输出：{ $error }
# { $error } is what Windows said went wrong.
core-warning-scan-environment-unreadable = 无法读取系统环境变量注册表项：{ $error }

## What a package records of how a driver is put back, which an installer shows. { $kind } is the driver's type as
## Windows names it (kernel, file-system, recognizer, or unknown-type); { $name } is its service's name; { $path } is
## its file. { $signed } is yes when { $signer } names who signed it, and no when that is not known.

core-service-driver-bundled =
    { $signed ->
        [yes] 应用自有的 { $kind } 驱动程序“{ $name }”（{ $path }，由 { $signer } 签名）。安装程序会捆绑其 .sys 并自动重新注册驱动程序服务（sc create，需要提升权限），然后确认该服务已注册。
       *[no] 应用自有的 { $kind } 驱动程序“{ $name }”（{ $path }，签名未知）。安装程序会捆绑其 .sys 并自动重新注册驱动程序服务（sc create，需要提升权限），然后确认该服务已注册。
    }
core-service-driver-from-vendor =
    { $signed ->
        [yes] 供应商或 Windows 自带的 { $kind } 驱动程序“{ $name }”（{ $path }，由 { $signer } 签名）。WinAppMigrator 不会再分发第三方驱动程序；请在启动此应用之前，在目标电脑上使用供应商已签名的驱动程序包重新安装它。
       *[no] 供应商或 Windows 自带的 { $kind } 驱动程序“{ $name }”（{ $path }，签名未知）。WinAppMigrator 不会再分发第三方驱动程序；请在启动此应用之前，在目标电脑上使用供应商已签名的驱动程序包重新安装它。
    }

## Runtimes an app needs, such as the Visual C++ runtime or .NET: how each is provided, as the Dependencies list says.

core-runtime-handling-built-in = Windows 内置
core-runtime-handling-ships-with-app = 随应用提供
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-handling-windows-feature = Windows 功能：{ $feature }
core-runtime-handling-included = 已包含
# { $id } is the runtime's name in winget, such as Microsoft.VCRedist.2015+.x64.
core-runtime-handling-winget = 通过 winget 安装：{ $id }
core-runtime-handling-manual = 手动安装

## Downloading a runtime's installer, when an installer is built. Each is why it could not be, and starts with a small
## letter, as it follows other words. { $url } is a web address; { $error } is what went wrong.

core-runtime-download-not-https = 拒绝下载 { $url }：它不是 HTTPS
# TLS is what keeps a download private.
core-runtime-tls-failed = TLS 初始化失败：{ $error }
core-runtime-download-failed = 下载 { $url } 失败：{ $error }
core-runtime-download-unreadable = 读取 { $url } 失败：{ $error }
core-runtime-download-empty = 下载 { $url } 未返回任何数据
# { $came } and { $expected } are numbers of bytes.
core-runtime-download-cut-short = 下载 { $url } 时中途中断：只收到 { $came } 字节，共 { $expected } 字节

## Installing a runtime: why it was not installed. Each starts with a small letter, as it follows other words.

core-runtime-no-known-installer = 此运行时没有已知的安装程序
# { $path } is where in the package, such as prerequisites\vc_redist.x64.exe.
core-runtime-payload-misplaced = 程序包将 { $path } 指定为此运行时的安装程序，但它不在应在的位置，因此未运行
# { $path } is a file.
core-runtime-payload-missing = 缺少捆绑的安装程序：{ $path }
# { $error } is what went wrong.
core-runtime-payload-path-invalid = 捆绑的安装程序路径无效：{ $error }
# { $url } is the web address it can be downloaded from. winget is a program's name.
core-runtime-no-installer-route = 没有捆绑的安装程序或 winget ID；请从 { $url } 获取
# { $error } is what went wrong.
core-runtime-no-temp-folder = 无法创建用于解压缩它的文件夹：{ $error }
# { $reason } is what the check found.
core-runtime-payload-signature-failed = 捆绑的安装程序未通过签名检查，因此未运行：{ $reason }
core-runtime-payload-timed-out = 捆绑的安装程序未在超时时间内完成，已被终止
# { $error } is what went wrong.
core-runtime-payload-not-started = 无法启动捆绑的安装程序：{ $error }
core-runtime-installer-killed = 安装程序已被终止，没有退出代码
# { $code } is the number it ended with, such as 1603.
core-runtime-installer-failed = 安装程序已退出，退出代码为 { $code }
# { $feature } is a Windows feature's name, such as NetFx3; { $code } is the number DISM ended with. Windows Features is
# the name of a Windows window. Keep the command as it is.
core-runtime-feature-failed = DISM 无法启用 Windows 功能 { $feature }（退出代码 { $code }）；可能是无法下载有效负载。请通过“Windows 功能”启用它，或运行 `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`。
# The same, when DISM gave no number.
core-runtime-feature-failed-no-code = DISM 无法启用 Windows 功能 { $feature }（退出代码未知）；可能是无法下载有效负载。请通过“Windows 功能”启用它，或运行 `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`。
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-feature-timed-out = 启用 Windows 功能 { $feature } 未在超时时间内完成，已被终止
# { $feature } is a Windows feature's name; { $error } is what went wrong.
core-runtime-feature-no-dism = DISM 不可用，无法启用 Windows 功能 { $feature }：{ $error }

## Installing a runtime with winget. { $id } is its name in winget, such as Microsoft.VCRedist.2015+.x64.

# { $error } is what went wrong.
core-runtime-winget-not-looked-for = 无法查找 winget：{ $error }
core-runtime-winget-not-installed = 未安装 winget
# { $path } is where winget is.
core-runtime-winget-misplaced = { $path } 不在 Windows 存放其自身程序包的位置，因此未运行
# { $path } is where winget is; { $reason } is what the check found.
core-runtime-winget-signature-failed = { $path } 未通过签名检查，因此未运行：{ $reason }
# { $reason } is one of the two messages above, or what went wrong.
core-runtime-winget-unusable = 无法使用 winget 安装 { $id }：{ $reason }
# { $code } is the number winget ended with.
core-runtime-winget-failed = winget install { $id } 已退出，退出代码为 { $code }
core-runtime-winget-failed-no-code = winget install { $id } 已退出，退出代码未知
core-runtime-winget-timed-out = winget install { $id } 未在超时时间内完成，已被终止
# { $error } is what went wrong.
core-runtime-winget-not-started = winget 不可用，无法安装 { $id }：{ $error }

## What told a scan how an app was installed, as the package's report lists it. The names of installers and files are
## as their makers write them.

# { $path } is an installer's record of what it installed, such as C:\Program Files\App\unins000.dat.
core-source-record-not-believed = 未使用 { $path }：管理员以外的人可以更改它
# { $count } is how many Windows Installer packages the bundle holds.
core-source-burn-bundle =
    { $count ->
       *[other] 包含 { $count } 个 Windows Installer 程序包的 Burn 捆绑包
    }
# { $count } is how many paths Windows Installer recorded.
core-source-msi-database =
    { $count ->
       *[other] Windows Installer 数据库：{ $count } 个组件键路径
    }
core-source-inno-log = Inno Setup 卸载日志 (unins000.dat)
# { $count } is how many paths the log records.
core-source-inno-log-paths =
    { $count ->
       *[other] Inno Setup 卸载日志：记录了 { $count } 个路径
    }
core-source-nsis = NSIS (Nullsoft) 卸载程序特征
# { $count } is how many paths the log records.
core-source-nsis-log-paths =
    { $count ->
       *[other] NSIS AdvUninstLog uninstall.log：记录了 { $count } 个路径
    }
core-source-squirrel = Squirrel 按用户布局 (Update.exe / app-*)
# { $version } is a folder's name that says the version, such as app-1.2.3.
core-source-squirrel-version = Squirrel 当前版本：{ $version }
core-source-chocolatey = Chocolatey 程序包元数据
core-source-msix = MSIX / Appx 包
# { $identity } is a package's name, such as Contoso.App.
core-source-appx-identity = AppxManifest 标识：{ $identity }
# { $count } is how many folders.
core-source-msix-data =
    { $count ->
       *[other] MSIX 按用户程序包数据：{ $count } 个位置
    }
# App Paths is a registry key's name.
core-source-app-paths = App Paths 注册

## Why a runtime is needed, as the package's report says it.

# { $program } is a program's file name, such as app.exe; { $dll } is a DLL's, such as msvcp140.dll.
core-runtime-program-imports = { $program } 导入 { $dll }

## What else of an app's a scan finds in the registry: fonts and certificates, and why each is the app's.

# Why a font is among the app's files, in the package's list of them.
core-registry-font-recorded = 应用的安装记录中指定的字体
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-named-for-the-app = 它以此应用或其发布者命名
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-recorded = 应用的安装记录中指定了它

## Who is signed in, for the Whose Data choice. Each of the first six is why it could not be told, and starts with a
## small letter, as it is shown inside other words. An account is shown by its id, such as S-1-5-21-...-1001.

# { $error } is what Windows said.
core-profile-desktop-owner-not-said = Windows 未说明此桌面属于谁：{ $error }
# { $error } is what went wrong.
core-profile-desktop-owner-not-asked = 无法询问 Windows 此桌面属于谁：{ $error }
core-profile-signed-in-account-no-profile = 在此登录的帐户 { $account } 没有配置文件记录
core-profile-desktop-not-a-person = 此处的桌面属于 { $account }，而它不是个人的帐户
core-profile-nobody-signed-in-running-account-no-profile = 此处没有人登录，且运行此程序的帐户 { $account } 没有配置文件记录
core-profile-nobody-signed-in-running-not-a-person = 此处没有人登录，且运行此程序的 { $account } 不是个人的帐户
# SID is the name Windows gives an account's id.
core-profile-nobody-signed-in-no-sid = 此处没有人登录，且此帐户没有 SID
# { $accounts } is a list of accounts.
core-profile-several-desktops = 此会话中有多个帐户拥有桌面：{ $accounts }
# Under the Whose Data choice. { $reason } is one of the messages above.
core-profile-signed-in-unknown = 无法确定谁已登录（{ $reason }）。
# Under the Whose Data choice, when there are accounts to choose from. { $reason } is one of the messages above.
core-profile-signed-in-unknown-choose = 无法确定谁已登录（{ $reason }）。请选择要包含谁的数据。
