# Words the core writes for people as it installs an app from a package: why an install will not start, the list of
# what it did (or for a preview, what it would do) that the installer window shows, its warnings, the record it keeps of
# each install so that it can be undone, and the core's errors. Message names start with core-.
#
# The list of what an install did is short and plain, one line for each thing, as the installer window lists them:
# "copied", "restored", "would copy". Words such as --force and --apply are command-line options, and LocalService,
# SYSTEM, ProgramData, and %AppData% are as Windows names them: keep them as they are.


## Errors: what went wrong, when the core cannot go on. Each is shown on its own, or after words that say what was being
## done.

core-error-list-changed = 此安装将执行的操作在获得同意后已发生变化，因此未安装任何内容
# { $path } is a file or folder, such as C:\Program Files\App\app.exe, or words that name one, such as "the installer".
# { $error } is what Windows said went wrong.
core-error-io = I/O 操作失败（{ $path }）：{ $error }
# { $detail } is what went wrong.
core-error-registry = 注册表读取失败：{ $detail }
# { $path } is a program file. { $detail } is what was wrong with it.
core-error-pe = PE 解析失败（{ $path }）：{ $detail }
# { $detail } is what was wrong, already in words.
core-error-invalid-input = 输入无效：{ $detail }
# { $detail } is what went wrong.
core-error-serialization = 报告序列化失败：{ $detail }
# { $detail } is what went wrong.
core-error-archive = 存档操作失败：{ $detail }
# { $detail } is what went wrong.
core-error-network = 网络操作失败：{ $detail }
# { $detail } is what went wrong.
core-error-time-format = 时间格式化失败：{ $detail }
core-error-cancelled = 操作已取消
# What is said when part of the program stopped unexpectedly and gave no reason.
core-error-panic-no-reason = 未提供原因

## The record of each install, kept so that it can be undone.

# { $kind } is a number that says what type of registry value it is.
core-journal-unknown-value-type = 未知的注册表值类型 { $kind }
core-journal-not-elevated = 未提升权限
core-journal-needs-admin = 安装需要管理员权限，以便将其记录保存在其他人无法更改的位置
# { $path } is a folder.
core-journal-not-an-install-folder = 这不是安装的文件夹：{ $path }
# { $path } is the file the record is kept in.
core-journal-not-written = 无法写入此安装的记录：{ $path }
# { $path } is a folder.
core-journal-path-escapes-root = 路径超出了预期的根目录：{ $path }
# { $path } is a folder.
core-journal-nothing-exists = 此电脑上不存在其中任何内容：{ $path }
core-journal-busy = 另一个安装或卸载正在运行。请等待它完成后重试
# Said of a file that was to get back the permissions it had.
core-journal-no-folder = 它没有文件夹
# Said of the permissions a file had, which were kept to give back to it.
core-journal-permissions-unreadable = 无法读取所保存的内容
# { $folder } is the folder of one install's record.
core-journal-unreadable-empty = { $folder } 中的安装记录无法使用：记录为空
# { $folder } is the folder of one install's record.
core-journal-unreadable-no-header = { $folder } 中的安装记录无法使用：其开头没有说明它所涉及的内容
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-version = { $folder } 中的安装记录无法使用：它是由此程序的其他版本写入的
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-install = { $folder } 中的安装记录无法使用：它表明自己属于另一个安装
# { $id } is an install's id, such as wam-1a2b3c.
core-journal-not-an-install-id = 这不是安装的 ID：{ $id }

## Why an install will not start, or stopped before it changed anything.

core-restore-apply-and-dry-run = 不能同时提供 --apply 和 --dry-run
# { $path } is the folder that was chosen to install in.
core-restore-location-not-full-path = 安装位置必须是完整路径：“{ $path }”
# { $problem } says what is wrong with the folder, already in words. { $path } is the folder.
core-restore-location-unusable = 无法使用该安装位置：{ $problem }：“{ $path }”
core-restore-needs-admin-plan = 还原计划包含仅限管理员执行的注册表、服务、计划任务、快捷方式或受保护目标更改
# { $path } is the folder that was chosen to install in.
core-restore-needs-admin-target = 目标根目录需要提升的权限：{ $path }
core-restore-needs-admin-user-data = 将用户数据还原到 ProgramData 或其他受保护的位置需要提升的权限
# { $error } is why the install stopped. { $failed } is what could not be put back, as sentences of their own.
core-restore-not-all-put-back = { $error }。已完成的操作未能全部恢复：{ $failed }
# { $error } is what went wrong.
core-restore-nowhere-to-keep-record = 没有安全的位置来保存可用于撤消此安装的记录，因此未安装任何内容：{ $error }
# { $path } is a file the app would have put there.
core-restore-already-exists = { $path } 已存在，因此未安装任何内容。使用 --force 可替换它
# { $root } is a folder the app has files for. { $link } is that folder, or one it is in.
core-restore-root-under-link = 此应用有内容要还原到 { $root }，而 { $link } 是指向其他位置的链接，其他人可能能够更改它，因此未安装任何内容。如果要放到那里的是用户数据或许可证数据，请在不包含用户数据和许可证数据的情况下安装。如果是“启动”文件夹中的快捷方式，请将该文件夹移回原来的位置
core-restore-hash-check-failed = 程序包未通过哈希检查，未安装任何内容
# { $detail } is the first thing found wrong, already in words.
core-restore-hash-check-failed-because = 程序包未通过哈希检查，未安装任何内容：{ $detail }
# { $path } is a file or folder.
core-restore-refuse-records-folder = 拒绝写入安装保存其记录的位置：{ $path }
# { $path } is a file or folder.
core-restore-refuse-outside = 拒绝写入安装位置和已知数据文件夹之外的位置：{ $path }
# { $path } is a file.
core-restore-no-parent-folder = 目标没有父文件夹：{ $path }
# { $path } is where a file was to go.
core-restore-folder-in-the-way = 此文件要放置的位置上已有一个文件夹：{ $path }
# { $path } is a file in the package.
core-restore-file-changed = { $path } 已不再是程序包中的内容，因此未安装
# { $path } is a file in the package.
core-restore-source-is-link = { $path } 是一个链接，不应跟随
# { $path } is a file in the package.
core-restore-source-became-link = { $path } 在打开期间变成了链接
# Why a service did not start, after "did not start:".
core-restore-service-stopped-by-itself = 它又自行停止了
# Why part of the app was left out, after "was left out:".
core-restore-com-class-someone-elses = 此电脑上已有此 COM 类，属于其他程序

## What an install did, one line for each thing, or for a preview, what it would do. { $source } is a file in the
## package, and { $destination } and { $path } are where it goes on this PC. { $key } is a registry key, such as
## HKLM\Software\App.

# { $package_valid } is true or false, and { $restore_valid } is true, false, or not_run: keep them as they are.
core-restore-verification-result = 还原后验证 package_valid={ $package_valid } restore_valid={ $restore_valid }
# { $name } is a service's name.
core-restore-started-service = 已启动服务 { $name }
core-restore-would-copy = 将复制 { $source } -> { $destination }
core-restore-copied = 已复制 { $source } -> { $destination }
core-restore-would-create-empty-folder = 将创建空文件夹 { $path }
core-restore-created-empty-folder = 已创建空文件夹 { $path }
# { $value } is the name of a value in the key.
core-restore-would-restore-registry-value = 将还原注册表 { $key } { $value }
core-restore-would-restore-registry-key = 将还原注册表 { $key }（项）
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-would-add-firewall-rule = 将为 { $program } 添加防火墙规则 { $name }
# { $name } is a firewall rule's name.
core-restore-firewall-rule-already-there = 防火墙规则 { $name } 已存在
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-added-firewall-rule = 已为 { $program } 添加防火墙规则 { $name }
# { $name } is a service's name, { $command } what it runs, and { $account } the account it runs as, such as LocalService.
core-restore-would-restore-service = 将还原服务 { $name } -> { $command }（以 { $account } 身份运行）
# { $name } is a runtime, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-restore-prerequisite-present = 必备组件已存在：{ $name }
# { $name } is a runtime that Windows has as a Windows feature.
core-restore-would-install-prerequisite-feature = 将安装必备组件 { $name }（Windows 功能）
# { $name } is a runtime whose installer is in the package.
core-restore-would-install-prerequisite-bundled = 将安装必备组件 { $name }（捆绑的安装程序）
# { $name } is a runtime that would come through winget, which is a program's name.
core-restore-would-install-prerequisite-winget = 将安装必备组件 { $name } (winget)
# { $name } is a runtime.
core-restore-installed-prerequisite = 已安装必备组件 { $name }
# { $name } is a runtime.
core-restore-installed-prerequisite-reboot = 已安装必备组件 { $name }（需要重启）
core-restore-would-restore-license = 将还原许可证 { $source } -> { $destination }
core-restore-restored-license = 已还原许可证 { $source } -> { $destination }
core-restore-would-restore-license-registry = 将还原许可证注册表 { $key }
# { $name } is a driver's name.
core-restore-driver-not-restored = 未自动还原驱动程序：{ $name }
# { $name } is a driver's name.
core-restore-would-install-driver = 将安装驱动程序 { $name } -> { $destination }
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-would-restore-task = 将还原计划任务 { $name }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-startup-shortcut = 将还原启动快捷方式 { $shortcut } -> { $target }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-shortcut = 将还原快捷方式 { $shortcut } -> { $target }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-would-restore-variable =
    { $scope ->
        [user] 将还原用户环境变量 { $name }
       *[machine] 将还原系统环境变量 { $name }
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-left-variable =
    { $scope ->
        [user] 用户环境变量 { $name } 保持不变：其中没有任何内容属于此应用
       *[machine] 系统环境变量 { $name } 保持不变：其中没有任何内容属于此应用
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-restored-variable =
    { $scope ->
        [user] 已还原用户环境变量 { $name }
       *[machine] 已还原系统环境变量 { $name }
    }
core-restore-set-uninstaller = 已为 { $key } 设置卸载程序
core-restore-placed-driver-image = 已放置驱动程序映像 { $source } -> { $destination }
# { $name } is a driver's name.
core-restore-configured-driver = 已配置驱动程序 { $name }
# { $name } is a driver's name.
core-restore-created-driver = 已创建驱动程序 { $name }
# { $name } is a driver's name.
core-restore-started-driver = 已启动驱动程序 { $name }
core-restore-ensured-registry-key = 已确保注册表项 { $key } 存在
# { $value } is the name of a value in the key.
core-restore-restored-registry-value = 已还原注册表 { $key } { $value }
# "(Default)" is how Windows names the value a key has with no name: write it as Registry Editor does.
core-restore-restored-registry-default = 已还原注册表 { $key } (默认)
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-configured-service = 已配置服务 { $name }（以 { $account } 身份运行）
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-created-service = 已创建服务 { $name }（以 { $account } 身份运行）
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-restored-task = 已还原计划任务 { $name }
core-restore-restored-shortcut = 已还原快捷方式 { $path }

## Warnings an install gives, and why it failed. They are shown after it, on the PC it ran on.

core-warning-restore-earlier-not-trusted = 一条未完成安装的记录未被处理，因为其他人可以更改它。该安装所做的任何更改仍然存在。
# { $app_name } is the app's name. { $count } is how many changes were put back.
core-warning-restore-earlier-undone =
    { $count ->
       *[other] { $app_name } 之前的一次安装未完成，因此已先将其撤消。已恢复 { $count } 项更改。
    }
# { $app_name } is the app's name.
core-warning-restore-earlier-never-finished = { $app_name } 之前的一次安装未完成。安装时会先撤消该次安装。
core-warning-restore-user-data-real-locations = 用户数据将还原到此电脑实际的 %AppData%/%ProgramData% 位置，而不是目标根目录。
# { $error } is what went wrong.
core-warning-restore-commit-failed = 无法完成此安装的记录：{ $error }
# { $name } is a service's name.
core-warning-restore-service-still-starting = 已请求启动服务 { $name }，但它尚未完成启动。它可能稍后会启动。下次此电脑启动时，它也会自行启动。
# { $name } is a service's name. { $reason } is why it did not start, as Windows said it.
core-warning-restore-service-not-started = 服务 { $name } 已安装，但未启动：{ $reason }。在它启动之前，应用可能无法正常工作。下次此电脑启动时，Windows 会再次尝试。
# { $name } is a service's name. { $reason } is why it could not be stopped.
core-warning-restore-service-not-stopped = 无法先停止服务 { $name }：{ $reason }。在此电脑下次启动之前，它可能会继续运行旧程序。
core-warning-restore-install-cancelled = 安装已取消。
core-warning-restore-preview-cancelled = 预览已取消。
# { $item } is what was left out, such as a file or a service's name. { $reason } is why, already in words.
core-warning-restore-left-out = { $item } 已排除：{ $reason }。
# { $error } is what went wrong.
core-warning-restore-verification-failed = 无法运行还原后验证：{ $error }
# { $runner } says what relies on the file: a service, a scheduled task, a firewall rule, a shortcut, or a registry value.
# { $path } is the file. { $name } is the name of what relies on it. { $refusal } is why the file was not replaced,
# already in words.
core-warning-restore-program-would-run =
    { $runner ->
        [service] 此程序包无权替换 { $path }，而服务 { $name } 依赖于它，因此未安装任何内容：{ $refusal }
        [task] 此程序包无权替换 { $path }，而计划任务 { $name } 依赖于它，因此未安装任何内容：{ $refusal }
        [firewall] 此程序包无权替换 { $path }，而防火墙规则 { $name } 依赖于它，因此未安装任何内容：{ $refusal }
        [shortcut] 此程序包无权替换 { $path }，而快捷方式 { $name } 依赖于它，因此未安装任何内容：{ $refusal }
       *[registry] 此程序包无权替换 { $path }，而注册表值 { $name } 依赖于它，因此未安装任何内容：{ $refusal }
    }
core-warning-restore-destination-exists = 目标已存在；使用 --force 可覆盖。
# { $path } is a file. { $error } is what went wrong.
core-warning-restore-file-not-written = 无法写入 { $path }：{ $error }
core-warning-restore-not-a-folder = 此空文件夹所在的位置上已有其他内容，而不是文件夹。
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-not-looked-at = 无法查看 { $path }：{ $error }
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-folder-not-made = 无法创建 { $path }：{ $error }
# Why part of the app was left out, after "was left out:".
core-warning-restore-registration-exists = 此电脑上已有此项，且设置为其他内容，安装不会接管它
core-warning-restore-registry-review = 注册表值已从存档的结构化数据中还原，已知的应用路径已重写为目标根目录。
# { $error } is what went wrong.
core-warning-restore-firewall-unreadable = 无法读取 Windows 防火墙的规则：{ $error }
# Why a firewall rule was left out, after "was left out:".
core-warning-restore-firewall-rule-exists = 此程序已有同名规则，且设置为其他内容，安装不会更改它
# { $name } is a runtime.
core-warning-restore-prerequisite-unknown = { $name }：此运行时没有已知的安装程序。
# { $name } is a runtime.
core-warning-restore-prerequisite-needs-admin = 安装 { $name } 需要提升权限；请以管理员身份重新运行安装程序。
# { $name } is a runtime.
core-warning-restore-prerequisite-reboot = { $name } 已安装，但需要重启才能完成。
# { $error } is what went wrong.
core-warning-restore-skipped-not-recorded = 无法记录已跳过的运行时：{ $error }
# { $name } is a runtime. { $reason } is why it was skipped, already in words.
core-warning-restore-prerequisite-skipped = { $name }：{ $reason }
# { $name } is a runtime. { $reason } is why it could not be installed. { $url } is a web address to get it from.
core-warning-restore-prerequisite-failed = 无法安装 { $name }：{ $reason }。请从 { $url } 获取。
# { $error } is what went wrong.
core-warning-restore-runtime-not-recorded = 无法记录已安装的运行时：{ $error }
# { $error } is what went wrong.
core-warning-restore-license-source-invalid = 打包的许可证路径无效：{ $error }
core-warning-restore-license-needs-admin = 将许可证数据还原到受保护的位置需要提升权限。
core-warning-restore-license-exists = 许可证目标已存在；使用 --force 可覆盖。
# { $error } is what went wrong.
core-warning-restore-license-failed = 无法还原许可证数据：{ $error }
core-warning-restore-driver-needs-admin = 注册驱动程序需要提升权限；请以管理员身份重新运行安装程序。
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-warning-restore-task-runs-as-system = 计划任务 { $name } 以 SYSTEM 身份运行，拥有此电脑的完全权限。
core-warning-restore-shortcut-exists = 快捷方式已存在；使用 --force 可覆盖。
core-warning-restore-shortcut-all-users = 一个或多个已还原的快捷方式指向公用桌面或公用“开始”菜单位置。
core-warning-restore-environment-refresh = 环境变量的更改可能需要在新的进程、命令行界面或登录会话中才能生效。
# { $name } is a service's name. { $account } is the account it ran as on the PC it came from.
core-warning-restore-service-account = 服务 { $name } 在源电脑上以 { $account } 身份运行。其密码不在安装程序中，因此该服务设置为以 LocalService 身份运行。如果它无法正常工作，请在“服务”中设置其帐户和密码。
# { $name } is a runtime.
core-warning-restore-prerequisite-pending-reboot = { $name } 尚未注册；重启后应会生效。
# { $name } is a runtime.
core-warning-restore-prerequisite-not-verified = { $name } 报告成功，但未注册；安装可能已失败。
# { $error } is what went wrong.
core-warning-restore-driver-source-invalid = 捆绑的驱动程序路径无效：{ $error }
# { $error } is what went wrong.
core-warning-restore-driver-not-read = 无法读取驱动程序的当前设置：{ $error }
core-warning-restore-driver-exists = 驱动程序服务已存在；使用 --force 可重新配置。
core-warning-restore-driver-image-exists = 驱动程序映像已存在；使用 --force 可覆盖。
core-warning-restore-driver-hash-bundled = 捆绑的驱动程序映像与其记录的哈希不匹配，因此未安装。
# { $error } is what went wrong.
core-warning-restore-driver-not-placed = 无法放置驱动程序映像：{ $error }
core-warning-restore-driver-hash-placed = 驱动程序映像在放置后发生了更改，因此未注册。
# { $error } is what went wrong.
core-warning-restore-key-not-created = 无法创建注册表项：{ $error }
# { $error } is what went wrong.
core-warning-restore-value-not-read = 无法读取此操作将替换的值：{ $error }
# { $error } is what went wrong.
core-warning-restore-value-not-written = 无法写入注册表值：{ $error }
core-warning-restore-value-exists = 注册表值已存在；使用 --force 可覆盖。
# { $error } is what went wrong.
core-warning-restore-environment-key-not-created = 无法创建环境变量注册表项：{ $error }
core-warning-restore-variable-exists = 环境变量已存在；使用 --force 可覆盖。
# { $error } is what went wrong.
core-warning-restore-variable-not-written = 无法写入环境变量：{ $error }
core-warning-restore-value-kind-missing = 缺少注册表值类型。
# { $error } is what went wrong.
core-warning-restore-service-not-read = 无法读取服务的当前设置：{ $error }
core-warning-restore-service-exists = 服务已存在；使用 --force 可重新配置。
# { $error } is what went wrong.
core-warning-restore-task-not-queried = 无法查询计划任务：{ $error }
core-warning-restore-task-exists = 计划任务已存在；使用 --force 可覆盖。
# { $error } is what went wrong.
core-warning-restore-task-folder-not-looked-at = 无法查看任务的文件夹：{ $error }
# { $error } is what went wrong.
core-warning-restore-shortcut-not-put-back = 无法创建快捷方式，并且无法恢复它所替换的快捷方式：{ $error }
# { $error } is what went wrong.
core-warning-restore-command-not-started = 无法启动命令：{ $error }
# { $output } is what the command wrote, as it wrote it.
core-warning-restore-command-failed = 命令失败：{ $output }
# { $error } is what went wrong.
core-warning-restore-powershell-not-started = 无法启动 PowerShell：{ $error }
