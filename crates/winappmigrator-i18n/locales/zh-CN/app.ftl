# Words of the app, WinAppMigrator itself: its one window, with the installed apps listed on the left and, on the
# right, what goes in an installer of the one picked and the button that builds it; and the steps to read another PC
# through WinAppMigrator Client. Message names start with app-.


## The window and its header

# The program's name, in the window's title bar and at the top of the window. Keep it as it is unless your language
# writes product names in a script of its own.
app-name = WinAppMigrator
app-tagline = 将已安装的应用转换为自解压安装程序
app-theme-to-light = 切换到浅色主题
app-theme-to-dark = 切换到深色主题
app-reload-list = 重新加载应用列表
app-list-loading = 正在加载
# { $count } is how many apps are listed.
app-list-count =
    { $count ->
       *[other] { $count } 个应用
    }
app-list-load-failed = 加载失败
# Shown when the window cannot open at all. { $error } is what went wrong, in the words of Windows or a graphics driver.
app-window-could-not-open = 无法打开窗口：{ $error }

## The list of installed apps

app-list-search = 按名称或发布者搜索  (Ctrl+F)
app-list-filter-all = 全部
# Whether an app is installed for one account or for everyone on the PC.
app-scope-per-user = 按用户
app-scope-per-machine = 按计算机
app-scope-unknown = 未知
# An app whose processor architecture (x86, x64, ARM64) is not known.
app-arch-unknown = 未知
app-list-loading-apps = 正在加载已安装的应用…
# { $error } is why the apps could not be listed.
app-list-could-not-list = 无法列出应用：{ $error }
app-list-no-matches = 没有匹配的应用
app-list-no-matches-hint = 请调整搜索文本或范围筛选器。
app-list-worker-disconnected = 读取应用列表的工作进程已断开连接。
# What a screen reader says of an app's row. { $name } is the app's name, { $publisher } who made it, and { $version } its
# version, such as 11.4.2, or — when either is not known.
app-list-row = { $name }，{ $publisher }，版本 { $version }

## The list's columns, and what a screen reader says each column's heading does when clicked

app-column-name = 名称
app-column-name-sort = 按名称排序
app-column-publisher = 发布者
app-column-publisher-sort = 按发布者排序
app-column-version = 版本
app-column-version-sort = 按版本排序
app-column-scope = 范围
app-column-scope-sort = 按范围排序
# Short for architecture: x86, x64, or ARM64. The column is narrow.
app-column-arch = 架构
app-column-arch-sort = 按体系结构排序

## Where apps are read from: this PC, a drive from another PC, a disk image of one, or another PC on the network

app-source = 扫描源
app-source-this-pc = 此电脑
app-source-drive = 已连接的驱动器…
app-source-image = 磁盘映像…
app-source-network = 网络电脑…
app-source-attaching-image = 正在以只读方式附加映像…
# { $computer } is the other PC's name, such as OLD-PC, and { $address } its network address, such as 192.168.1.20.
app-source-network-pc = { $computer }（通过网络连接，{ $address }）
app-source-disconnect = 断开连接
app-source-change = 更改…
# The title of the window that picks a disk image. .vhd and .vhdx are file extensions.
app-source-pick-image-title = 选择另一台电脑的磁盘映像 (.vhd 或 .vhdx)
app-source-pick-image-filter = 磁盘映像
# { $path } is the disk image file.
app-source-image-has-no-windows = { $path } 中没有装有 Windows 的卷。
app-source-image-not-attached = 无法附加磁盘映像。
app-source-pick-drive-title = 选择已连接驱动器的根目录（例如 E:\）
# { $path } is the folder picked, such as E:\. \Windows and \Users are folder names, as Windows writes them.
app-source-not-a-windows-drive = { $path } 看起来不像 Windows 系统驱动器（没有 \Windows 和 \Users）。请选择来自另一台电脑的驱动器的根目录。
# Said of a drive or disk image with a Windows older than Windows 10. { $windows } is its name, such as "Windows 7
# Professional Service Pack 1, 32-bit". { $is_32_bit } is "yes" when that Windows is 32-bit, which adds the last
# sentence, and "no" when it is not.
app-source-older-windows =
    此处装有 { $windows }。由此生成的安装程序适用于 Windows 10 和 11。使用“用户数据”时，Windows 为旧版应用悄悄另存的设置会一并包含，Windows 11 没有的运行时（例如 DirectX 9）会随应用一起安装。{ $is_32_bit ->
        [yes] 其中的应用为 32 位，因此会放入 Program Files (x86)，64 位 Windows 将此类应用存放在该文件夹中。
       *[no] { "" }
    }
# The same, of a Windows whose name could not be read. { $is_32_bit } is as above.
app-source-older-windows-unnamed =
    此处装有较旧版本的 Windows。由此生成的安装程序适用于 Windows 10 和 11。使用“用户数据”时，Windows 为旧版应用悄悄另存的设置会一并包含，Windows 11 没有的运行时（例如 DirectX 9）会随应用一起安装。{ $is_32_bit ->
        [yes] 其中的应用为 32 位，因此会放入 Program Files (x86)，64 位 Windows 将此类应用存放在该文件夹中。
       *[no] { "" }
    }

## The details panel, before an app is picked, and the app picked

app-details-pair-first = 与旧电脑配对
app-details-pair-first-hint = 当 WinAppMigrator Client 在旧电脑上运行并且你已输入其代码后，旧电脑上的应用会列在此处，你可以像在此电脑上一样从中选择一个来生成安装程序。
app-details-pick-an-app = 选择一个应用
app-details-pick-an-app-hint = 从列表中选择一个应用，以选择要包含的内容并生成其安装程序。
# { $publisher } is who made the app, and { $version } its version, such as 11.4.2.
app-details-publisher-and-version = { $publisher }，版本 { $version }
# { $version } is the app's version, such as 11.4.2.
app-details-unknown-publisher-and-version = 未知发布者，版本 { $version }
app-details-unknown-publisher = 未知发布者

## What goes in besides the program

app-option-license = 许可证和激活数据
app-option-license-hint = 可能与此电脑绑定，在新电脑上需要重新激活。
app-option-user-data = 用户数据
# Documents, Saved Games, and ProgramData are folders of Windows: use the names Windows gives them in your language,
# except ProgramData, which keeps its name.
app-option-user-data-hint = 设置和历史记录、文档、保存的游戏以及 ProgramData。
app-option-whose-data = 数据所属帐户
app-option-whose-data-hint = 一个安装程序只包含一个帐户的数据。这些数据将归属于运行安装程序时登录的用户。

## Building the installer

app-output-folder = 输出文件夹
# An example of a folder, shown in the empty field. "You" stands for the name of an account.
app-output-folder-example = C:\Users\用户名\WinAppMigrator
app-output-folder-not-full = 请输入完整的文件夹路径，例如 D:\Installers。
app-build-create = 创建安装程序
app-build-create-again = 再次创建安装程序
app-build-cancel = 取消
app-build-cancelling = 将在当前步骤完成后取消…
app-build-cancelled = 已取消生成。未写入任何安装程序。
# winappmigrator-installer.exe is a file name.
app-build-no-stub = 在此应用旁边找不到安装程序存根 (winappmigrator-installer.exe)，因此无法生成自解压安装程序。
app-build-select-an-app-first = 请先选择要迁移的应用。
app-build-look-again-first = 必须重新查看此应用，然后才能生成。
# { $reason } is what went wrong, in the program's own words, often in English.
app-build-stopped-unexpectedly = 生成意外停止：{ $reason }。如果它在输出文件夹中留下了安装程序，请不要使用该安装程序。
app-build-worker-disconnected = 迁移工作进程已断开连接。
# { $path } is the output folder, and { $error } what Windows said went wrong.
app-build-no-output-folder = 无法创建输出文件夹 { $path }：{ $error }
# { $error } is what went wrong.
app-build-no-staging-folder = 无法创建受保护的暂存文件夹，因此未生成任何内容：{ $error }
# { $error } is what went wrong, as a sentence.
app-build-could-not-build = 无法生成安装程序。{ $error }
app-build-no-installer = 生成已完成，但未生成安装程序。
# { $error } is what went wrong.
app-build-hash-check-did-not-run = 安装程序已写入，但无法运行其哈希检查：{ $error }。请不要使用它。

## How far a build has got

app-progress-preparing = 正在确定要包含的内容…
app-progress-checking-room = 正在检查空间是否足够…
app-progress-bundling = 正在捆绑运行时和驱动程序…
# { $done } is how many files have been read so far, and { $total } how many there are.
app-progress-packing = 正在读取和压缩文件：{ $done }/{ $total }
# { $done } is how many of the installer's records have been looked over so far, and { $total } how many there are.
app-progress-looking-over = 正在检查安装程序记录的内容：{ $done }/{ $total }…
app-progress-writing = 正在写入安装程序…
app-progress-verifying = 正在验证哈希…

## What a build made

app-result-failed-hash-check = 请不要使用此安装程序：它未通过哈希检查。
# { $count } is how many files were gone when the build came to read them.
app-result-files-gone =
    { $count ->
       *[other] 安装程序已生成，但自查看此应用以来已有 { $count } 个文件不复存在，因此未包含在其中。请参阅下面的“警告”。
    }
# { $count } is how many files could not be read into the installer.
app-result-files-not-captured =
    { $count ->
       *[other] 安装程序已生成，但有 { $count } 个文件无法捕获，通常是因为它们被锁定或无法读取。缺少这些文件，应用可能无法正常工作。请参阅下面的“警告”。
    }
app-result-mentions-who = 安装程序已生成，但其中仍包含有关其来源的信息。在将其提供给任何人之前，请阅读下面的说明。
app-result-ready = 安装程序已就绪。其程序包已通过哈希检查。
app-result-files = 文件
app-result-installer-size = 安装程序大小
app-result-warnings = 警告
app-result-log = 日志
app-result-copy-error = 复制错误信息
app-result-installer = 安装程序
app-result-data-file = 数据文件
app-result-open-folder = 打开文件夹
app-result-copy-path = 复制路径
# { $error } is what Windows said went wrong.
app-result-could-not-open-folder = 无法打开文件夹：{ $error }
# { $installer } and { $data_file } are file names, such as App_installer.exe and App_installer.wamdata.
app-result-two-files = 此应用太大，无法放入一个可执行文件，因此其安装程序由两个文件组成：{ $installer } 和 { $data_file }。它们必须放在一起：将两者以原有名称复制到另一台电脑上的同一个文件夹中。如果你对安装程序进行签名，该签名也涵盖数据文件。
# { $app_name } is the app's name. "Unknown publisher" is what Windows shows when it asks to run an unsigned program: use
# Windows' own words for it in your language.
app-result-run-it = 在另一台电脑上运行它以重新安装 { $app_name }。它会通过 SHA-256 检查其文件。Windows 会显示“未知发布者”，因为你生成的安装程序未签名。
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-verification-errors = 验证错误 ({ $count })
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-warnings-heading = 警告 ({ $count })
# After the first of a long list of warnings. { $count } is how many more there are.
app-warnings-and-more = 另有 { $count } 项。

## Looking at an app, to see what an installer of it would hold

app-scan-getting-ready = 正在准备查看此应用…
app-scan-install-record = 正在读取其安装方式…
app-scan-near-matches = 正在查找可能也属于它的内容…
app-scan-system-items = 正在查找其服务、任务和快捷方式…
app-scan-files = 正在检查其文件…
app-scan-registry = 正在读取它与 Windows 的集成…
app-scan-finishing = 即将完成…
app-scan-look-again = 重新查看
app-scan-stop = 停止
app-scan-running-hint = 在此过程中，你可以选择其他应用或更改要包含的内容。
# { $error } is why, in words that follow a colon.
app-scan-failed = 无法查看此应用：{ $error }
app-scan-try-again = 重试
app-scan-stopped = 已停止。查看完此应用后才能生成安装程序。
# { $reason } is what went wrong, in the program's own words, often in English.
app-scan-stopped-unexpectedly = 查看过程意外停止：{ $reason }。

## How big an installer can be, and whether there is room for it

app-room-files = 文件
# Over how big the installer can be: up to that size.
app-room-up-to = 最多
app-room-free = 可用
# { $drive } is a drive, such as C:.
app-room-free-on = { $drive } 上可用
# How much room a drive has, when it would not say.
app-room-free-unknown = 未知
app-room-most-it-could-need = 压缩前可能需要的最大空间。
# { $drive } is a drive, such as C:, { $needed } how much room it needs, such as 1.2 GB, and { $free } how much it has.
app-room-drive-short = { $drive } 最多需要 { $needed } 的可用空间才能生成此安装程序，现有 { $free }。
# { $drives } is one sentence like the one above for each drive without the room, one after another.
app-room-not-enough = 空间不足。{ $drives }请选择其他驱动器上的输出文件夹，或排除某些内容。
# { $note } is a sentence, without its full stop, about a drive whose room could not be checked. Use your language's
# full stop.
app-room-could-not-tell = { $note }。

## What goes in the installer, part by part

app-footprint-heading = 安装程序包含的内容
app-footprint-working-out = 正在确定与之相关的内容…
# { $error } is why.
app-footprint-would-not-make-a-package = 剩余的内容无法构成程序包：{ $error }
# { $list } is the names of what else is left out, such as a service and a shortcut.
app-footprint-also-left-out = 这也会排除 { $list }，因为它们运行或指向的内容将不存在。它们不会在另一台电脑上设置。
# { $list } is the names of the first few of what else is left out, and { $count } how many more there are.
app-footprint-also-left-out-and-more = 这也会排除 { $list } 以及另外 { $count } 项，因为它们运行或指向的内容将不存在。它们不会在另一台电脑上设置。
# { $list } is the names of license files. The option is the one named "License and Activation Data".
app-footprint-licenses-still-go = 你排除的内容中的许可证数据仍会包含在内，因为这由“许可证和激活数据”选项决定：{ $list }。
# { $elapsed } is how long ago, in minutes and seconds such as 45:00, or hours, minutes, and seconds such as 1:05:00.
# "Look Again" names the button of that name.
app-footprint-looked-at-long-ago = 此应用是在 { $elapsed } 前查看的。文件会在生成时读取，因此会纳入更改，届时已不存在的内容会列出。如有新增内容，请单击“重新查看”。
# Headings of the parts of the list, each with a line that says what is in it.
app-footprint-program-files = 程序文件
app-footprint-data-folders = 数据文件夹
# What could go in with the User Data option, when nothing else is in this part. { $size } is how much, such as 1.2 GB.
app-footprint-folders-with-user-data = 使用“用户数据”选项时可包含 { $size }
# { $included } is how many folders go in, { $count } how many there are, and { $size } how much those that go in hold.
app-footprint-folders-summary =
    { $count ->
       *[other] { $count } 个文件夹中的 { $included } 个，{ $size }
    }
# { $size } is how much, such as 1.2 GB, and { $count } in how many files.
app-footprint-data-only-with-user-data =
    { $count ->
       *[other] 此应用的数据中有 { $count } 个文件（{ $size }）仅在使用“用户数据”选项时才会包含。
    }
# A folder's row. { $count } is how many files it holds, and { $detail } where it is or why it cannot be left out.
app-footprint-folder-row =
    { $count ->
       *[other] { $count } 个文件。{ $detail }
    }
app-footprint-services = 服务
app-footprint-startup = 自动启动
app-footprint-tasks = 计划任务
app-footprint-shortcuts = 快捷方式
# { $included } is how many of the part's things go in, and { $count } how many there are.
app-footprint-items-summary = { $count } 项中的 { $included } 项
# { $why } is what else was left out that this goes with.
app-footprint-left-out-too = 也已排除：{ $why }
# A row's detail. { $kind } is what it is, such as Service, and { $detail } what it does.
app-footprint-change-detail = { $kind }：{ $detail }
app-footprint-integrations = 与 Windows 的集成
# { $included } is how many kinds go in, such as file types and firewall rules, and { $count } how many kinds there are.
app-footprint-integrations-summary =
    { $count ->
       *[other] { $count } 类中的 { $included } 类
    }
app-footprint-integration-nothing-left = 由于你排除的内容，此类已不剩任何内容
# { $count } is how many things of this kind the installer sets up.
app-footprint-integration-things =
    { $count ->
       *[other] 将设置 { $count } 项
    }
app-footprint-prerequisites = 运行所需的内容
# { $count } is how many runtimes, such as Visual C++ Redistributable.
app-footprint-prerequisites-summary =
    { $count ->
       *[other] { $count } 个运行时
    }
app-footprint-near-matches = 可能也属于此应用
# { $ticked } is how many were ticked, and { $count } how many there are.
app-footprint-near-matches-summary = { $count } 项中已添加 { $ticked } 项
app-footprint-near-matches-hint = 除非你勾选，否则它们不会包含在安装程序中。勾选某项会重新查看应用，并执行与其他内容相同的检查，因此看起来像密码的内容仍会被排除。
# { $why } is why it might be part of the app, as a sentence.
app-footprint-near-match-with-user-data = 随“用户数据”选项一起包含。{ $why }
# How much a folder holds, when not all of it could be measured. { $size } is how much was, such as 1.2 GB.
app-footprint-near-match-over = 超过 { $size }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-folder = 文件夹：{ $place }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-hidden = 应用的隐藏部分：{ $place }
app-footprint-nothing-left-out = 没有内容因看起来像密码、许可证或证书而被排除。
app-footprint-left-out = 排除的内容及原因
# { $count } is how many things were left out.
app-footprint-left-out-summary =
    { $count ->
       *[other] { $count } 项
    }
# Above a list of things left out for one reason. { $title } is the reason.
app-footprint-left-out-group = { $title }：
app-footprint-refused = 任何安装程序都不得设置以下内容：
# A heading over a list that opens and closes. { $count } is how many are in it.
app-footprint-worth-knowing = 注意事项 ({ $count })
# { $list } is what kinds of things were looked for and none found, from the five below.
app-none-found = 未找到：{ $list }。
app-none-found-services = 服务
app-none-found-startup = 启动项
app-none-found-tasks = 计划任务
app-none-found-shortcuts = 快捷方式
app-none-found-integrations = 与 Windows 的任何集成

## Network PC: reading another PC through WinAppMigrator Client, in three steps. "WinAppMigrator Client" is the name of a
## program, and the old PC is the one the apps are moved off.

app-network-step-client = 1. 将客户端放到旧电脑上
app-network-step-client-hint = WinAppMigrator Client 在应用当前所在的电脑上运行，需要 Windows 7 或更高版本。它仅允许此电脑使用它所显示的代码，以加密方式读取那台电脑。
# { $drive } is a USB drive, such as E:\.
app-network-save-to-usb = 保存到 USB 驱动器 { $drive }
# { $drive } is a USB drive, such as E:\, and { $name } the name it was given, such as BACKUP.
app-network-save-to-named-usb = 保存到 USB 驱动器 { $drive } ({ $name })
app-network-save-to-folder = 保存到文件夹…
app-network-look-for-usb-again = 重新查找 USB 驱动器
app-network-no-usb = 未插入 USB 驱动器。请插入一个，或保存到可以从旧电脑访问的文件夹，例如网络上的共享文件夹。
# { $path } is where the client was saved, such as E:\WinAppMigrator Client.exe.
app-network-client-saved = 已保存为 { $path }。在旧电脑上，先关闭要迁移的应用，然后双击该文件，并允许它对设备进行更改。它会显示一个代码。
app-network-step-pick = 2. 选择旧电脑
app-network-looking = 正在网络上查找运行客户端的电脑…
app-network-type-name = 或输入其名称或地址
# An example, shown in the empty field: a PC's name, or its network address.
app-network-type-name-example = OLD-PC 或 192.168.1.20
app-network-step-code = 3. 输入它显示的代码
app-network-code = 配对码
app-network-connect = 连接
app-network-pairing = 正在配对…
# Why pairing did not work, shown on its own.
app-network-pairing-stopped = 配对在完成前已停止
app-network-wrong-code = 这不是旧电脑显示的代码。请检查后重试。
# { $error } is what went wrong.
app-network-not-reached = 无法连接到旧电脑：{ $error }
app-network-pick-folder-title = 选择用于保存 WinAppMigrator Client 的文件夹
# { $path } is the folder, and { $error } what Windows said went wrong.
app-client-could-not-save = 无法将客户端保存到 { $path }：{ $error }
# scripts\build-client.ps1 is a file name.
app-client-not-carried = 此版本的 WinAppMigrator 不包含客户端。请使用 scripts\build-client.ps1 生成客户端，然后重新生成应用。
# { $error } is what Windows said went wrong.
app-client-could-not-save-note = 无法保存客户端旁边的说明文件：{ $error }
# A note saved as a text file beside the client, for whoever runs it on the old PC. Keep its lines short, as it is read in
# Notepad. "WinAppMigrator Client.exe" is a file name. "Network PC..." names the Scan Source button: use its words in your
# language.
app-client-note =
    WinAppMigrator Client
    
    在你要迁出应用的电脑上运行此程序：Windows 7、8、10 或 11。
    
    1. 关闭要迁移的应用。
    2. 双击“WinAppMigrator Client.exe”，并允许它对设备进行更改。
    3. 它会显示一个 6 位配对码。在新电脑上的 WinAppMigrator 中，
       选择“网络电脑...”，选择此电脑，然后输入该代码。
    4. 新电脑完成后，在客户端窗口中按 Enter，或关闭该窗口。
    
    它只读取数据。运行期间，它会保留 Windows 驱动器的快照，
    并允许本地网络通过 Windows 防火墙访问它自己的两个端口；
    停止时会删除这两项。通过网络传输的内容均已加密，
    只有拥有该代码的电脑才能读取任何内容。
