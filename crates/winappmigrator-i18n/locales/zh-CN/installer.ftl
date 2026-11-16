# Words of the installer that every built installer carries, which runs on the PC an app is moved to and speaks its
# language, and of the uninstaller it leaves behind for Settings. Message names start with installer- for the
# installer's window, installer-cli- for its command line, and uninstaller- for the uninstaller.
#
# The command lines (installer-cli-, uninstaller-cli-) are always shown in English, as scripts read what they print.
# Translate them all the same. Keep their command-line flags, such as --apply and --target-root, exactly as they are.


## The installer's window: its title and header

installer-window-title = WinAppMigrator 安装程序
# The window's title in the taskbar. { $app_name } is the name of the app it installs.
installer-window-title-app = 安装 { $app_name }
installer-tagline = 重新安装在另一台电脑上捕获的应用
# What an app is called when its package gives it no name.
installer-unnamed-app = 应用

## When the installer's window cannot open. Each is a paragraph of a message box.

# { $error } is what went wrong, as Windows or the graphics driver says it.
installer-window-failed = 无法打开安装程序的窗口：{ $error }
installer-window-failed-without-one = 没有窗口也可以安装：在管理员命令提示符中使用 --apply 运行此文件；若要选择安装位置，请再加上 --target-root，后跟一个文件夹。

## Checking the installer when it opens

installer-checking = 正在检查安装程序…
installer-stub-no-package = 这是 WinAppMigrator 安装程序存根，其中不包含任何应用的程序包。请使用 WinAppMigrator 生成安装程序，然后在此处运行生成的 .exe。
installer-check-stopped = 安装程序检查已停止。
# { $error } is what went wrong, as Windows says it.
installer-locate-failed = 找不到安装程序：{ $error }
# { $error } is what went wrong reading it.
installer-read-failed = 无法读取安装程序：{ $error }
# { $error } is why, such as that the app needs a newer Windows.
installer-cannot-be-used-here = 此安装程序的程序包无法在此电脑上使用：{ $error }

## The app, where it goes, and whether the installer is signed

installer-captured-from = 捕获自
installer-install-location = 安装位置
# The title of the folder picker.
installer-choose-location = 选择安装位置
installer-folder-has-files = 此文件夹中已有文件。同名文件将被替换，卸载后会恢复。
installer-room-needed = 所需空间
# { $shortfall } says which drive is short of room, such as "C: needs up to 824.0 MB for the app's files, and has 300.0 MB free".
installer-room-short = 此处可能没有足够的空间：{ $shortfall }。可以在其他驱动器上解压缩，但应用的文件仍须放得下。
# { $app_name } is the name of an app whose earlier install was cut short.
installer-interrupted = { $app_name } 之前的一次安装未完成。安装时会先撤消该次安装。
# Beside "Installer", for whether it proves who made it.
installer-signature = 安装程序
installer-signature-checking = 正在检查其签名…
installer-signature-unsigned = 未签名：发布者未知
# { $publisher } is the name on the installer's certificate, such as "Fabrikam, Inc.".
installer-signature-signed-by = 签名者：{ $publisher }
installer-signature-unnamed = 由未具名的发布者签名
# { $reason } is Windows' own word for what is wrong, such as HashMismatch.
installer-signature-invalid = 此安装程序的签名无效（{ $reason }）。签名后它可能已被更改。

## Why the install location cannot be used. Install and Preview Changes stay off while one of these is shown.

# { $app_name } is the name of the app being installed.
installer-location-app-already-there = { $app_name } 已安装在此文件夹中，因此这个副本不能安装到这里。若要安装到此处，请先在 Windows 设置的“应用”中卸载此电脑上的副本，然后重新打开此安装程序。或者在上方输入一个新的空文件夹。
# { $other_app } is the name of another app Windows lists as installed in the folder. { $app_name } is the name of the app being installed.
installer-location-other-app-there = { $other_app } 已安装在此文件夹中，因此 { $app_name } 不能安装到这里。请在上方输入一个新的空文件夹。或者，如果你不再需要 { $other_app }，请先在 Windows 设置的“应用”中将其卸载，然后重新打开此安装程序。
installer-location-has-files = 此文件夹中已有并非由 WinAppMigrator 安装的文件。它们可能属于其他应用，因此不会在其上安装任何内容。请在上方输入一个新的空文件夹。
# { $reason } is why, such as that it cannot be the Windows folder.
installer-location-refused = 无法使用此文件夹：{ $reason }。请在上方输入其他文件夹。

## What the install sets up beyond its files

installer-beyond-files = 除文件之外还会设置的内容
# Under the heading above. It is part of what a person agrees to when they click Install.
installer-beyond-files-agreed = 所有这些操作都以管理员权限执行。如果安装失败，它所做的更改将被恢复，无法恢复的内容会告知你。你也可以稍后在“设置”的“已安装的应用”中撤消此安装。它替换的文件会恢复，它安装的运行时会保留。
installer-beyond-files-none = 无：它只复制文件，以及应用自身注册表项中的设置。
# { $problem } is why, such as that the app cannot run on this PC.
installer-cannot-go-here = 此程序包无法安装到此位置：{ $problem }
installer-working-out = 正在确定在此位置将设置哪些内容…
# Said as the reason after "This package cannot go to this location:".
installer-list-stopped = 确定其操作的过程已停止
# A heading over a list of what else the package asked for and will not get. { $count } is how many.
installer-more-left-out =
    { $count ->
       *[other] 另有 { $count } 项已排除
    }

## Options

installer-options = 选项
installer-user-data = 用户数据
installer-user-data-detail = 开：设置、文档和保存的游戏将放入当前登录者的常用文件夹。关：全部放在安装文件夹下，且不会设置任何从中运行的内容。
# { $account } is the name of the account on the other PC whose data this is.
installer-user-data-whose = 这里包含 { $account } 的数据。
installer-license = 许可证
installer-license-detail = 激活和许可证文件，以及保存许可证的注册表值。部分可能需要在此重新激活。

## Dependencies: what else the app needs, and how each is provided

installer-dependencies = 依赖项
installer-dependencies-detail = 此应用需要的内容，以及每项的提供方式。
installer-dependencies-none = 未检测到其他运行时依赖项。
# { $feature } is the name of an optional part of Windows, such as NetFx3.
installer-dependency-windows-feature = Windows 功能：{ $feature }
installer-dependency-included = 已包含
# { $id } is the package's name in winget, such as Microsoft.VCRedist.2015+.x64.
installer-dependency-winget = 通过 winget 安装：{ $id }
installer-dependency-manual = 手动安装
installer-dependency-built-in = Windows 内置
installer-dependency-ships-with-app = 随应用提供

## Buttons, and what is said beside them

installer-install = 安装
installer-preview = 预览更改
installer-cancel = 取消
installer-close = 关闭
installer-open-folder = 打开安装文件夹
# { $error } is what went wrong, as Windows says it.
installer-open-folder-failed = 无法打开文件夹：{ $error }
installer-location-blocks = 在安装位置可以使用之前，“安装”和“预览更改”不可用。原因显示在上方的安装位置下。
installer-package-blocks = “安装”不可用，因为此程序包无法安装到此位置。原因显示在上方。
installer-list-changed = 在此处将执行的操作已发生变化，因此未安装任何内容。请查看下面的列表，如果符合你的要求，请再次单击“安装”。
installer-cancelled-early = 已取消。未进行任何更改。

## How far an install has got, beside its progress bar

# { $step } is one of the steps below. { $done } and { $total } are how many of its items are done, and how many there are.
installer-step-counted = { $step }：{ $done }/{ $total }…
# { $step } is one of the steps below, when it does not count its items.
installer-step-uncounted = { $step }…
installer-step-getting-ready = 正在准备…
installer-step-cancelling = 将在当前步骤完成后取消…
installer-step-putting-back = 正在恢复已更改的内容
installer-step-unpacking = 正在解压缩
installer-step-checking-package = 正在检查程序包
installer-step-recovering = 正在撤消之前未完成的安装
# What a preview is doing, whatever step it is on.
installer-step-previewing = 正在确定将要更改的内容
installer-step-runtimes = 正在安装运行时
installer-step-files = 正在复制文件
installer-step-services = 正在设置服务
installer-step-drivers = 正在设置驱动程序
installer-step-registry = 正在写入注册表设置
installer-step-license = 正在还原许可证数据
installer-step-tasks = 正在设置计划任务
installer-step-shortcuts = 正在创建快捷方式
installer-step-environment = 正在设置环境变量
installer-step-settings-entry = 正在将应用添加到“设置”
installer-step-verifying = 正在检查已安装的内容
installer-step-undoing = 正在撤消

## When an install cannot go ahead or stops

installer-carries-no-package = 此安装程序不包含程序包。
installer-stopped-unexpectedly = 安装意外停止。
# { $reason } is what went wrong, as the program says it.
installer-stopped-unexpectedly-because = 安装意外停止：{ $reason }。如果安装已经开始，再次运行安装程序会先撤消它所做的操作。
# { $error } is what went wrong reading it.
installer-package-read-failed = 无法读取程序包：{ $error }
installer-no-programdata = 找不到此电脑的 ProgramData 文件夹，因此未安装任何内容：安装会将用于撤消它的记录保存在该文件夹中。
# { $error } is why, such as that there is nowhere only administrators can get into.
installer-staging-failed = 无法创建暂存文件夹：{ $error }
# { $error } is what went wrong unpacking it.
installer-unpack-failed = 无法解压缩程序包：{ $error }
installer-staging-nowhere = 没有只有管理员才能访问的位置可用于解压缩程序包
# { $failure } is a folder and what went wrong there, such as "C:\ProgramData\WinAppMigrator: Access is denied.".
installer-staging-nowhere-look-at = 没有只有管理员才能访问的位置可用于解压缩程序包。需要检查的是 { $failure }

## What came of an install, or of a preview

installer-result = 安装结果
installer-result-preview = 预览：尚未进行任何更改
installer-result-installed = 应用已安装，并已通过哈希检查。
installer-result-installed-without-left-out = 应用已安装，并已通过哈希检查，但不包括下面列为已排除的内容。
installer-result-not-all-put-back = 安装失败，且其所做的更改未能全部恢复。剩余的内容列在下面，下次安装时会再次尝试。
installer-result-cancelled-put-back = 安装已取消，其所做的更改已恢复。
installer-result-failed-put-back = 安装失败，其所做的更改已恢复。
installer-result-not-clean = 安装未正常完成。请查看下面的问题。
installer-result-changes-put-back = 已恢复的更改
installer-result-changes-made = 已进行的更改
installer-result-changes-planned = 计划的更改
installer-result-warnings = 警告
installer-result-errors = 错误
installer-result-package-valid = 程序包哈希有效
installer-result-package-invalid = 程序包哈希无效
installer-result-files-valid = 已安装的文件有效
installer-result-files-invalid = 已安装的文件无效
installer-result-reboot = 某个依赖项需要重启才能完成。请重启 Windows，然后启动应用。
# A heading over the list of what was changed. { $count } is how many.
installer-result-changes = 更改 ({ $count })
# After the first part of a long list. { $count } is how many more there are.
installer-result-changes-more = 另有 { $count } 项。
# Over the path of the install's log file.
installer-log = 日志

## The installer's command line. Shown in English: scripts read it. A { $count } is plain digits, with no separators.

installer-cli-stub = WinAppMigrator 安装程序存根——其中不包含任何应用的程序包。
installer-cli-stub-how = 请使用 WinAppMigrator 应用生成安装程序，然后在目标电脑上运行生成的 .exe。
installer-cli-usage-title = WinAppMigrator 自解压安装程序。
installer-cli-usage = 不带参数运行可打开图形安装程序，或者：
installer-cli-usage-preview = 预览重新安装：--dry-run [--target-root <DIR>]
installer-cli-usage-apply = 重新安装：    --apply [--target-root <DIR>]
installer-cli-no-package = 安装程序不包含嵌入的程序包
installer-cli-no-programdata = 找不到此电脑的 ProgramData 文件夹，因此未安装任何内容：安装会将用于撤消它的记录保存在该文件夹中
# { $path } is the folder asked for, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-folder-has-files = 无法使用 { $path }：{ $problem }。仅当你确定它属于此应用时才使用 --force
installer-cli-no-location = 此程序包未说明其应用原来的位置，因此请指定 --target-root
# { $path } is where the app was on the other PC, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-location-refused = 此程序包的应用原来位于 { $path }，该位置无法使用：{ $problem }。请指定 --target-root
# { $summary } is how much room it needs where, such as "up to 824.0 MB on C: (356.0 GB free)".
installer-cli-room = 空间：{ $summary }
# { $note } is something to know about the install.
installer-cli-note = 注意：{ $note }
# { $count } is how many things it sets up beyond its files.
installer-cli-sets-up = 将设置：{ $count }
installer-cli-applied = 已应用重新安装。
installer-cli-previewed = 已预览重新安装。
# { $count } is how many changes the install logged.
installer-cli-changes-logged = 已记录的更改：{ $count }
# { $count } is how many warnings.
installer-cli-warnings = 警告：{ $count }
# { $count } is how many errors.
installer-cli-errors = 错误：{ $count }
# { $count } is how many things the package asked for and did not get.
installer-cli-refused = 已拒绝：{ $count }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-package-valid = 程序包哈希有效：{ $valid }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-reinstall-valid = 重新安装哈希有效：{ $valid }
# { $id } is the install's id, such as wam-0f3a….
installer-cli-install-id = 安装 ID：{ $id }
installer-cli-put-back = 安装失败，因此其所做的更改已恢复。
# { $path } is the log file.
installer-cli-log = 日志：{ $path }
# { $path } is the log file. Printed before an error.
installer-cli-error-log = 日志：{ $path }
# { $error } is what went wrong.
installer-cli-error = 错误：{ $error }

## The uninstaller's window

uninstaller-window-title = 卸载
# The window's title, and its heading. { $app_name } is the name of the app it uninstalls.
uninstaller-window-title-app = 卸载 { $app_name }
uninstaller-tagline = 撤消其安装程序所做的操作
uninstaller-reading = 正在读取此次安装的记录…
uninstaller-stopped-unexpectedly = 卸载意外停止。
uninstaller-what-will-be-removed = 将删除的内容
uninstaller-install-location = 安装位置
uninstaller-files = 文件
uninstaller-registry-values = 注册表值
uninstaller-services = 服务
uninstaller-drivers = 驱动程序
uninstaller-scheduled-tasks = 计划任务
uninstaller-environment = 环境变量
uninstaller-plan-stays = 你添加或更改的文件及其所在的文件夹将保留。
# { $count } is how many files the install replaced.
uninstaller-plan-coming-back =
    { $count ->
       *[other] 此安装替换的 { $count } 个文件将恢复；如果原位置的文件此后已更改，则放在该文件旁边。
    }
# { $runtimes } is a list of runtimes, such as "Microsoft Visual C++ Redistributable, .NET Desktop Runtime 8".
uninstaller-plan-runtimes = 它安装的运行时将保留，因为其他程序可能会使用它们：{ $runtimes }。
uninstaller-options = 选项
uninstaller-remove-user-data = 同时删除用户数据和许可证数据
# { $count } is how many items of the app's settings, data, and license there are.
uninstaller-remove-user-data-detail =
    { $count ->
       *[other] 其设置、数据和许可证共 { $count } 项。除非打开此选项，否则它们将保留。它们所替换的内容将恢复；如果它们此后已更改，则放在它们旁边。
    }
uninstaller-uninstall = 卸载
uninstaller-cancel = 取消
uninstaller-close = 关闭
# { $done } and { $total } are how many of the install's items are undone, and how many there are.
uninstaller-step-counted = 正在撤消安装：{ $done }/{ $total }…
uninstaller-step = 正在撤消安装…
uninstaller-done = 应用已卸载。
uninstaller-done-except = 应用已卸载，下面列出的内容除外。
uninstaller-waits-for-drive = 部分内容位于未连接的驱动器上。请连接该驱动器，然后从“设置”再次卸载。
uninstaller-not-all-undone = 并非所有内容都能撤消。请关闭正在使用它的程序，然后从“设置”再次卸载。
# Over the path of the uninstall's log file.
uninstaller-log = 日志
uninstaller-undone = 已撤消
uninstaller-left-in-place = 保留原处
uninstaller-could-not-be-undone = 无法撤消
# After the first part of a long list. { $count } is how many more there are.
uninstaller-and-more = 另有 { $count } 项。
# Said as the reason when there is no record of the install to read.
uninstaller-no-programdata = 找不到此电脑的 ProgramData 文件夹，因此没有可读取的记录

## When the uninstaller's window cannot open. Each is a paragraph of a message box, followed by a command to type.

uninstaller-window-failed-title = WinAppMigrator 卸载程序
# { $error } is what went wrong, as Windows or the graphics driver says it.
uninstaller-window-failed = 无法打开卸载程序的窗口：{ $error }
uninstaller-window-failed-quiet = 没有窗口也可以卸载，在管理员命令提示符中运行：
# Stands in for this program's own path in that command, if it cannot be found.
uninstaller-this-program = 此程序

## The uninstaller's command line (--quiet). Shown in English: scripts read it.

# { $app_name } is the name of the app.
uninstaller-cli-uninstalling = 正在卸载 { $app_name }。
# { $path } is the log file.
uninstaller-cli-log = 日志：{ $path }
# { $error } is what went wrong.
uninstaller-cli-error = 错误：{ $error }
