# Words the core writes for people while it builds a package from an installed app: what a package records for an
# installer to show later, errors shown in the app, the list of what goes in and what is left out, opening a disk
# image or another PC over the network, and what an installer says when the package it carries cannot be read.
# Message names start with core-.


## Scanning an app, before a package is built. Shown in the app when a scan cannot go on.

# { $path } is the app's program, such as D:\Program Files\Acme\Acme.exe, on a drive taken from another PC.
core-scan-program-not-on-the-drive = { $path } 经过链接或不在已连接的驱动器上，因此未读取。
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-not-on-this-pc = { $path } 不在此电脑的磁盘上，或其名称会打开其他内容，因此未读取。
# { $path } is the app's program.
core-scan-program-someone-elses = { $path } 位于另一个帐户的文件夹中或不属于此帐户，而此应用自身的记录并未表明它属于那里，因此未读取。
# { $app_name } is the app's name, as the list of apps calls it.
core-scan-nothing-to-package = 找不到 { $app_name } 自己的程序，因此没有可放入安装程序的内容。驱动程序、运行时和加载项通常如此：在新电脑上，请从制造商或 Windows 更新安装驱动程序，并将运行时或加载项与使用它的应用一起安装。
# { $app_name } is the app's name, as the list of apps calls it. A web app is a website installed as an app, with
# a window and a Start menu entry of its own.
core-scan-web-app = { $app_name } 是浏览器从网站安装的 Web 应用。它没有自己的程序，因此没有可放入安装程序的内容。请在新电脑上用同一浏览器打开该网站，并从那里安装此应用。
# { $path } is the program the app's own record names, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-missing = 此应用的记录将 { $path } 列为应用的程序，但它已不在那里。该应用可能已被移动或部分删除：在此电脑上修复或重新安装该应用应能将其恢复。
# { $path } is the file that was picked.
core-scan-not-a-file = 扫描目标不是文件：{ $path }
# Names something left out of a package, in a list of what was left out and why. { $name } is the variable's name,
# such as ACME_LICENSE_KEY.
core-scan-environment-variable-item = 环境变量 { $name }

## Folders and parts that might also be the app's, offered in a list to tick. Each says why it is offered.

core-near-starts-with-app-name = 其名称以此应用的名称开头。
core-near-has-app-name = 其名称包含此应用的名称。
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-starts-with-publisher = 其名称以发布者的名称 { $publisher } 开头。
# { $folder } is a folder's name, such as Program Files.
core-near-hidden-part-beside = 应用列表中隐藏的一个部分，与此应用一起安装在 { $folder } 中。
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-hidden-part-publisher = 应用列表中隐藏的一个部分，来自同一发布者 { $publisher }。

## What a scan found, as a person chooses from it: the folders that go in, and what is left out, and why

# Why a folder cannot be left out, beside its number of files.
core-footprint-locked-app-folder = 这些始终与主程序一起包含
core-footprint-locked-main-program = 包含主程序，主程序始终会被包含
core-footprint-locked-needed = 包含主程序启动所必需的内容
core-footprint-locked-font = 随其字体一起包含（字体列在“与 Windows 的集成”下）
# Where a folder is. "AppData" and "ProgramData" are the names of Windows folders; keep them as they are.
core-footprint-place-roaming = 你的配置文件中的设置 (AppData)
core-footprint-place-local = 你的配置文件中此电脑专用的设置 (AppData)
core-footprint-place-program-data = 由此电脑上的所有人共享 (ProgramData)
core-footprint-place-documents = 在你的“文档”中
core-footprint-place-saved-games = 在你的“保存的游戏”中
core-footprint-place-public-documents = 在“公用文档”中
core-footprint-place-app-folder = 在应用的文件夹中
# { $folder } is a folder, such as C:\Users\Sam\AppData\Roaming.
core-footprint-place-in = 在 { $folder } 中
# A row for the files directly in the app's folder. { $folder } is that folder's name, such as Acme.
core-footprint-files-directly-in = 直接位于 { $folder } 中的文件
# Headings over what is left out, and why each item is.
core-footprint-licenses-left-out = 未包含在此安装程序中，因为“许可证和激活数据”选项已关闭
core-footprint-font-stays-behind = 它会留在原处。
core-footprint-drivers-left-out = 属于 Windows 或供应商的驱动程序，无法再分发，因此请在另一台电脑上重新安装它们
core-footprint-driver-reinstall = 请使用其供应商自己的程序包重新安装它。
# Why something else is left out along with what the person left out. { $service } is a service's name, such as AcmeSync.
core-footprint-needs-left-out = 它需要 { $service }，而该服务已被排除
core-footprint-told-to-use-left-out = 它被设置为使用你排除的文件夹中的内容
core-footprint-file-in-left-out = 它的文件位于你排除的文件夹中
core-footprint-program-in-left-out = 它所对应的程序位于你排除的文件夹中
core-footprint-names-left-out = 它指向你排除的文件夹
# Why something is left out, as the installer tells whoever installs it.
core-footprint-builder-left-it-out = 此安装程序的生成者排除了它
# Something the person left out, by kind and name, such as "Service Acme Updater". { $kind } is the kind, such as
# Service or Scheduled Task; { $title } is its name.
core-footprint-left-out-item = { $kind } { $title }

## Building a package and an installer from a scan. Shown in the app when a build cannot go on.

core-package-scan-of-another-app = 此扫描针对的应用不是正在生成的应用，因此未生成任何内容
core-package-scan-brought-in-more = 此扫描带入了此次生成未要求的数据，因此未生成任何内容
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-package-program-gone = { $path } 已不存在，因此未生成任何内容。请重新查看此应用。
core-package-needs-stub = 自解压存档需要安装程序存根
# { $reason } is what went wrong.
core-package-staged-file-failed = 暂存文件夹中的某个文件无法放入程序包：{ $reason }
# { $shortfalls } says which drives are short of room and by how much, in sentences of its own.
core-package-not-enough-room = 空间不足，无法生成此安装程序，因此未读取或写入任何内容。{ $shortfalls }。请在那里释放一些空间，或选择其他驱动器上的输出文件夹。
# { $path } is a file's path, on a drive taken from another PC.
core-package-not-read-off-the-drive = { $path } 经过链接或不在已连接的驱动器上，因此未读取
# { $path } is a file's path.
core-package-not-read-not-local = { $path } 不在此电脑的磁盘上，因此未读取
# { $path } is a file's path.
core-package-not-read-other-account = { $path } 通向另一个帐户的文件夹，因此未读取
# { $path } is a file's path, as the PC it came from called it.
core-package-not-on-the-drive = { $path } 不在已连接的驱动器上，因此未读取
# { $path } is a file's path.
core-package-backup-copy-no-parent = 无法备份复制没有父目录的文件：{ $path }
# { $path } is a file's path.
core-package-backup-copy-destination-no-parent = 备份复制的目标没有父目录：{ $path }
# robocopy is the name of a Windows command; keep it as it is. { $code } is how it ended, such as Some(8).
core-package-robocopy-failed = robocopy 备份模式复制失败（退出代码 { $code }）
# Stands for a folder where a file's name would be, as in "I/O failed for temporary package folder: …".
core-package-temporary-folder = 临时程序包文件夹
# Stands for the installer being written where a file's name would be, as in "I/O failed for the installer: …".
core-package-the-installer = 安装程序
# { $path } is a file's path, or its name in the package. { $error } is what went wrong with it.
core-package-path-error = { $path }：{ $error }
# { $name } is a file's name in the package, such as app/bin/App.exe.
core-package-same-place = 另一个文件在程序包中的位置与此相同：{ $name }
# { $name } is a file's name in the package.
core-package-changed-in-protected-folder = 生成程序包期间，{ $name } 在受保护的文件夹中发生了更改
core-package-worker-died = 压缩文件的线程意外停止，因此未写入安装程序
# { $path } is a file's path.
core-package-link-not-read = { $path } 是链接，因此未读取
# { $path } is a file's path. { $error } is what went wrong.
core-package-not-compressed = 无法压缩 { $path }：{ $error }
# { $path } is a file's path. { $reason } is what went wrong.
core-package-not-read-back = 无法回读 { $path }：{ $reason }

## Where an app's files are, and where an installer may write them. Shown when a path is refused.

# { $path } is a folder, such as C:\Program Files\Acme.
core-discovery-root-not-full-path = 要写入的文件夹必须是完整路径：{ $path }
# { $path } is a folder's path.
core-discovery-root-not-a-folder = 要写入的文件夹不是文件夹：{ $path }
# { $path } is a file's path.
core-discovery-escapes-root = 路径超出了预期的根目录：{ $path }
# { $path } is the link's path.
core-discovery-refusing-to-write-through-link = 拒绝通过链接写入：{ $path }
# { $path } is the link's path.
core-discovery-refusing-link = 拒绝经过链接：{ $path }
# { $error } is what Windows said went wrong.
core-discovery-refusing-link-because = 拒绝经过链接：{ $error }
# { $path } is the link's path.
core-discovery-link-to-somewhere-else = { $path } 是指向其他位置的链接，其他人可能能够更改它指向的位置。请使用它指向的位置
# { $path } is a folder's path.
core-discovery-no-parent-folder = { $path } 没有父文件夹
# { $path } is the install location as it was given. Keep the quotes, or use your language's own.
core-discovery-install-location-not-full-path = 安装位置必须是完整路径：“{ $path }”
# { $path } is the install location.
core-discovery-install-location-a-link = 安装位置是指向其他位置的链接：{ $path }
# { $path } is a file's path.
core-discovery-device-name = 路径使用了为 Windows 设备保留的名称：{ $path }
# { $path } is a file's path.
core-discovery-device-name-on-older-windows = { $path } 的名称会被此版本的 Windows 当作设备而不是文件打开，因此未向其写入任何内容。它可以安装在 Windows 11 上
# { $path } is a file's path.
core-discovery-unsafe-path = 不安全的路径：{ $path }
# { $path } is a file's path.
core-discovery-no-package-path = 文件没有程序包相对路径：{ $path }

## Writing an installer, and reading the package inside one. Most of these are shown by an installer, on the PC it
## runs on, when it cannot read what it carries. "The package" is the app's files and settings that an installer
## carries; "the stub" is the program part of an installer, before the package; "the data file" is the .wamdata file
## that holds the package of a very large installer, beside it.

# Stands for the package where a file's name would be, as in "I/O failed for installer package: …".
core-archive-installer-package = 安装程序的程序包
# Stands for the installer program built into WinAppMigrator, where a file's name would be.
core-archive-embedded-stub = 嵌入的安装程序存根
# { $index } is the number of a file in the package. { $reason } is what went wrong.
core-archive-entry-unreadable = 无法读取程序包的第 { $index } 个条目：{ $reason }
# { $name } is a file's name inside the package, such as app/bin/App.exe.
core-archive-entry-unsafe-path = zip 条目的路径不安全：{ $name }
# { $name } is a file's name inside the package.
core-archive-holds-more = { $name } 包含的内容多于其声明的内容
# { $name } is a file's name inside the package.
core-archive-name-too-long = 此名称对于 zip 来说太长：{ $name }
core-archive-changed-while-copied = 将程序包追加到存根时，程序包发生了更改，因此未写入安装程序
core-archive-stub-not-a-program = 此安装程序中位于程序包之前的部分不是程序
core-archive-damaged = 此安装程序已损坏，或在生成后被更改，因此未从其程序包中读取任何内容。请从生成它的位置重新复制。
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-missing = 此安装程序将其程序包保存在同一文件夹中名为 { $name } 的文件中，但该文件不存在。这两个文件必须放在一起：请从生成安装程序的位置将 { $name } 复制到此处，然后重新运行。
# { $name } is the data file's name, such as App_installer.wamdata. { $error } is what Windows said went wrong.
core-archive-data-file-unopened = 无法打开此安装程序的数据文件 { $name }：{ $error }。这两个文件必须放在一起，并位于此电脑可以读取的文件夹中。
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-a-link = { $name } 是链接或文件夹，而此安装程序的数据文件必须是独立的文件，且紧挨着安装程序，因此未从中读取任何内容。请将实际文件复制到此处。
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-wrong = { $name } 不是此安装程序的数据文件，或者已损坏，或在生成后被更改，因此未从中读取任何内容。请从生成它们的位置重新复制这两个文件。
core-archive-stub-signed = 安装程序存根已签名；它必须保持未签名状态，因此请改为对生成的安装程序签名
# { $size } is a size, such as 4.2 GB.
core-archive-too-big-to-run = 安装程序将为 { $size }，而 Windows 无法运行 4 GB 或更大的可执行文件；未写入任何内容
core-archive-outside-signature = 安装程序在其签名之外包含数据，可能已被篡改
core-archive-footer-length = 自解压存档的尾部长度无效
core-archive-seek-outside = 定位到程序包之外
# { $path } is a file's path.
core-archive-unsafe-package-path = 不安全的程序包路径：{ $path }
# { $path } is a file's path.
core-archive-empty-package-path = 程序包路径为空：{ $path }

## Opening a disk image of another PC (a .vhd or .vhdx file) to read apps from it

# { $path } is the disk image file, such as D:\Backups\OldPC.vhdx.
core-image-not-attachable = { $path } 不是 Windows 可以附加的磁盘映像 (.vhd 或 .vhdx)
core-image-needs-administrator = 附加磁盘映像需要管理员权限
# { $path } is the disk image file the person picked.
core-image-not-a-file = { $path } 不是文件
# { $path } is the disk image file. "Edit Disk" is the name of a Hyper-V Manager command.
core-image-differencing = { $path } 是差异磁盘，只包含自其父磁盘创建以来的更改，而 Windows 也会打开父磁盘，无论它记录的父磁盘位置在哪里。请先使用 Hyper-V 的“编辑磁盘”将其合并到父磁盘，然后选择父磁盘
# { $path } is the disk image file.
core-image-unreadable = { $path } 不是可读取的 .vhd 或 .vhdx
# ProgramData is the name of a Windows folder; keep it as it is.
core-image-no-program-data = 找不到此电脑的 ProgramData 文件夹
# The reason given in core-image-could-not-attach when Windows already has the image open.
core-image-already-attached = Windows 已附加此映像，附加它的程序可能正在向其写入。请先将其分离，或将其驱动器作为已连接的驱动器选择
# { $path } is the disk image file. { $reason } is why, such as core-image-already-attached or what Windows said.
core-image-could-not-attach = 无法附加 { $path }：{ $reason }
# { $path } is the disk image file.
core-image-no-windows-volume = { $path } 中没有可读取的装有 Windows 的卷。使用 BitLocker 加密的卷必须先解锁。
# Given as the reason when Windows failed without saying anything.
core-image-windows-did-not-say = Windows 未说明原因

## Reading another PC over the network, through the client running on it. Shown as why something on that PC
## could not be read. "The client" is WinAppMigrator Client, the small program on the other PC.

core-remote-too-slow = 另一台电脑响应时间过长
core-remote-closed = 与另一台电脑的连接已关闭
core-remote-failed = 与另一台电脑的连接失败
core-remote-answered-something-else = 客户端返回了意外的应答
# { $name } is the PC's name or address as the person typed it, such as OLDPC or 192.168.1.20.
core-remote-not-found = 在网络上找不到 { $name }
core-remote-did-not-say-what-it-is = 客户端未说明自己是什么
core-remote-ordinary-names-only = 只能查询另一台电脑根目录下使用普通名称的路径
core-remote-more-but-none = 另一台电脑表示某个文件夹中还有更多内容，但未提供其中任何内容
# { $count } is the most things one folder may hold, such as 250,000.
core-remote-too-many-listed = 另一台电脑在一个文件夹中列出了超过 { $count } 项
core-remote-sent-too-much = 另一台电脑发送的文件内容超出了请求的范围
core-remote-name-not-text = 无法查询不是文本的名称
core-remote-below-root-only = 只能查询另一台电脑根目录下的路径
core-remote-file-ended-early = 另一台电脑上的文件提前结束
core-remote-link-not-followed = 不跟随另一台电脑上的链接
core-remote-only-files-opened = 在另一台电脑上只能打开文件
core-remote-listing-said-nothing = 列表中没有它的任何信息
core-remote-change-time-unknown = 不知道它的更改时间
core-remote-seek-before-start = 定位到开头之前
# { $size } is how many bytes a file is; { $limit } is the most that were expected. Both are numbers.
core-remote-too-large =
    { $size ->
       *[other] 它有 { $size } 字节，而预期不超过 { $limit } 字节
    }

## Checking that a download is signed by Microsoft. Each is given as the reason a check failed.

# { $error } is what Windows said went wrong.
core-signature-no-powershell = 无法启动 PowerShell：{ $error }
# { $error } is what went wrong reading PowerShell's answer.
core-signature-unreadable = 无法读取签名：{ $error }
# { $signer } is who signed the file, such as Contoso Ltd.
core-signature-not-microsoft = 它由 { $signer } 签名，而不是 Microsoft
core-signature-not-microsoft-unnamed = 它由其他人签名，而不是 Microsoft
# { $status } is Windows' own word for the signature's state, such as HashMismatch or NotSigned, which is not translated.
core-signature-status = 其签名状态为 { $status }

## Warnings a package records when it is built from an app, which an installer shows later, maybe on another PC, in
## that PC's language. "This app" is the app being packaged.

core-warning-build-program-needs-unreadable = 无法读取此程序对 Windows 的要求，因此不会检查它能否在另一台电脑上运行。
core-warning-build-external-manifest = 检测到外部应用程序清单；并行 (side-by-side) 依赖项的解析将推迟进行。
core-warning-build-non-exe = 所选文件不使用 .exe 扩展名；将继续进行 PE 扫描。
core-warning-build-stand-in-program = 未找到普通的应用可执行文件，因此由此文件代替。体系结构和依赖项结果描述的是此文件，不一定能描述该应用。
core-warning-build-other-apps-files-unknown = 无法读取 Windows Installer 中有关其他应用的全部信息，因此此应用记录的文件夹中可能包含其他应用的文件，而这里没有任何信息能表明这一点。
core-warning-build-app-folder-has-another-apps-file = Windows Installer 表明另一个应用在此应用的程序所在的文件夹中有一个文件，该文件会一并包含，因为整个文件夹都会被包含。
# "App Paths" is the name of a Windows registry key; keep it as it is.
core-warning-build-app-paths-folder-left-out = 此应用的 App Paths 条目表明它会在一个不属于它的文件夹中查找，因此该文件夹不会一并包含。在没有该文件夹内容的电脑上，它可能无法启动。
core-warning-build-uninstaller-folder-left-out = 此应用的卸载程序所在的文件夹已被排除，因为 Windows Installer 表明另一个应用在那里有文件。
# { $why } is never shown: it chooses the reason. Translate each reason as the end of the sentence.
core-warning-build-recorded-folder-left-out =
    此应用记录中的某项指向一个已排除的文件夹，因为{ $why ->
        [unreadable] 它的路径会离开该驱动器或经过链接，因此无法安全读取
        [missing] 它不存在
        [link] 它经过链接
        [short-name] 它的写法不是其真实名称
        [another-account] 它位于另一个帐户的文件夹中
        [one-account] 此应用是为一个帐户安装的，而这不是该帐户的文件夹
        [shared] 它包含共享的内容，或属于 Windows 的一部分
        [generic-name] 从它的名称看不出它属于谁
        [publisher] 它仅以发布者命名，可能包含该发布者的其他产品
        [holds-the-app] 它包含此应用自己的文件夹，还可能包含更多内容
        [another-apps-name] 它使用另一个应用的名称
        [another-apps-record] 它的安装记录属于另一个应用
        [in-another-apps-folder] 它位于另一个应用的文件夹中
       *[another-apps] 它属于另一个应用
    }。
core-warning-build-app-folder-holds-another-app = 此电脑上的另一个应用记录为位于此应用的文件夹中，它会一并包含，因为整个文件夹都会被包含。
# "Program Files (x86)" is the name of a Windows folder; keep it as Windows calls it.
core-warning-build-from-32-bit-windows = 此应用原先位于 32 位 Windows 上。其注册表项按 64 位 Windows 存放 32 位应用的方式排列，安装程序会为其提供 Program Files (x86)。
# { $dll } is a file's name, such as MSVCP140D.dll.
core-warning-build-debug-runtime = 某个程序需要 { $dll }，这是 Visual C++ 运行时的调试版本。Microsoft 没有此类版本的安装程序，并且可能不允许再分发它们，因此在未安装 Visual Studio 的电脑上它将无法运行。
# { $program } is a program's file name, such as Tool.exe.
core-warning-build-single-file-program = { $program } 是打包成单个文件的 .NET 程序，它没有说明需要哪个 .NET，因此未为其计划任何 .NET。如果它需要某个 .NET，在没有该 .NET 的电脑上将无法启动。
# { $count } is how many of the app's programs.
core-warning-build-programs-not-inspected =
    { $count ->
       *[other] 此应用有 { $count } 个程序未检查其所需的运行时，因为程序太多或体积太大。
    }
# { $runtime } is a runtime's name, such as Java Runtime Environment.
core-warning-build-runtime-no-installer = 此应用需要 { $runtime }，但此处没有它的安装程序，因此不会运行任何安装程序。请在另一台电脑上自行安装。
# { $why } is why something was left out, such as "its name says what it holds".
core-warning-build-left-out-of-package = 已从程序包中排除：{ $why }。
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-missing-dependency = 无法解析依赖项：{ $name }
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-dependency-only-here = { $name } 只能通过此电脑的搜索路径找到，并且不属于此应用，因此不会一并包含。在没有它的电脑上，应用可能无法启动。
core-warning-build-driver-detected = 检测到驱动程序或内核组件。应用自有的驱动程序会被捆绑并重新注册；供应商驱动程序或 Windows 自带的驱动程序不会再分发，必须使用其已签名的供应商程序包重新安装。
core-warning-build-near-no-longer-offered = 在“可能也属于此应用”列表中勾选的某项已不在该列表中，因此未添加。请重新查看该列表。
# { $app_name } is the name of a part of the app, as the list of apps calls it. { $why } is never shown: it chooses the
# reason. Translate each reason as the end of the sentence.
core-warning-build-near-part-left-out =
    { $app_name } 声明属于其自身的部分内容已被排除：{ $why ->
        [unreadable] 它的路径会离开该驱动器或经过链接，因此无法安全读取
        [missing] 它不存在
        [link] 它经过链接
        [short-name] 它的写法不是其真实名称
        [another-account] 它位于另一个帐户的文件夹中
        [one-account] 此应用是为一个帐户安装的，而这不是该帐户的文件夹
        [shared] 它包含共享的内容，或属于 Windows 的一部分
        [generic-name] 从它的名称看不出它属于谁
        [publisher] 它仅以发布者命名，可能包含该发布者的其他产品
        [holds-the-app] 它包含此应用自己的文件夹，还可能包含更多内容
        [another-apps-name] 它使用另一个应用的名称
        [another-apps-record] 它的安装记录属于另一个应用
        [in-another-apps-folder] 它位于另一个应用的文件夹中
       *[another-apps] 它属于另一个应用
    }。
# { $app_name } is the name of a part of the app, as the list of apps calls it.
core-warning-build-near-keys-left-out = { $app_name } 的安装记录中列出的注册表项已被排除，因为该记录还指向了属于另一个应用的内容。
# "Windows keeps what an app wrote for one account" is Windows' VirtualStore folder.
core-warning-build-virtual-store-program = 在 Windows 存放应用为单个帐户所写内容的文件夹中有一个程序。Windows 从不在那里放置程序，因此是其他程序放置的，该程序已被排除。
# { $path } is a file or folder, such as C:\Users\Sam\AppData\Roaming\Acme.
core-warning-build-file-left-out = { $path } 已被排除：它经过已连接驱动器上的链接，或者属于另一个帐户，而不是正在包含其数据的帐户。
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as D:\Sam\Documents.
core-warning-build-user-folder-not-on-drive = { $user } 的 { $folder } 文件夹位于 { $path }，不在此驱动器上，因此未包含其中的内容。
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as \\server\home\Sam\Documents.
core-warning-build-user-folder-not-on-this-pc = { $user } 的 { $folder } 文件夹位于 { $path }，不在此电脑的磁盘上，因此未包含其中的内容。
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-base-unreadable = 无法读取 { $path }（{ $error }）；未包含存储在那里的应用数据。
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-folder-unreadable = 无法读取 { $path }（{ $error }）；其中嵌套的应用数据可能缺失。
# { $error } is what went wrong.
core-warning-build-entry-unreadable = 无法读取文件系统条目：{ $error }
# { $count } is how many links. { $names } lists some of them. { $more } is how many more there are, which are not
# named; when it is 0, the sentence ends after the list.
core-warning-build-links-not-followed =
    { $count ->
       *[other] 此文件夹中有 { $count } 个链接或会打开其他内容的名称未被跟随，因此它们指向的内容不在安装程序中：{ $names }
    }{ $more ->
        [0] 。
       *[other] ，另有 { $more } 个。
    }
# { $count } is how many folders. { $depth } is how many levels down a folder can be. { $names } lists some of them.
# { $more } is how many more there are, which are not named; when it is 0, the sentence ends after the list.
core-warning-build-folders-too-deep =
    { $count ->
       *[other] 有 { $count } 个文件夹的深度超过 { $depth } 级，因此其中的内容不在安装程序中：{ $names }
    }{ $more ->
        [0] 。
       *[other] ，另有 { $more } 个。
    }
# { $error } is what went wrong.
core-warning-build-metadata-unreadable = 无法读取文件元数据：{ $error }
core-warning-build-reserved-name = 此文件的名称为 Windows 设备保留，因此未复制。
# { $error } is what went wrong.
core-warning-build-hash-failed = 无法计算文件哈希：{ $error }
core-warning-build-cloud-placeholder = 仅联机可用的云文件（例如 OneDrive 文件随选）没有本地数据；请在备份前将其设为“始终保留在此设备上”或下载它。已报告，但未复制。
# { $file_name } is a driver's file name, such as acmeflt.sys.
core-warning-build-driver-file = 驱动程序/内核文件“{ $file_name }”不会作为普通文件复制。如果它支持此应用已注册的驱动程序服务，安装程序会将其捆绑并自动重新注册该驱动程序；否则，请使用供应商已签名的驱动程序包重新安装它。
core-warning-build-file-gone = 查看此应用时此项存在，但现在已不存在，因此不在安装程序中。
# { $error } is what went wrong.
core-warning-build-file-copy-failed = 无法将文件复制到程序包中：{ $error }
# { $count } is how many files. { $names } lists them.
core-warning-build-not-looked-through =
    { $count ->
       *[other] 此程序包中有无法检查是否透露其来源的文件：{ $names }。这些文件的内容可能会透露来源。
    }
# { $count } is the most empty folders an installer makes, such as 10,000.
core-warning-build-empty-folders-truncated = 此应用有超过 { $count } 个空文件夹，只会重新创建其中的 { $count } 个。
core-warning-build-credential-by-now = 已从程序包中排除：它现在看起来像凭据。
core-warning-build-no-package-path = 可复制的文件没有程序包相对路径。
# { $file_name } is the runtime installer's file name, such as vc_redist.x64.exe. { $error } is what went wrong.
core-warning-build-prerequisite-path-invalid = 无法放置捆绑的必备组件 { $file_name }：{ $error }
# { $runtime } is a runtime's name, such as Microsoft Visual C++ 2015+ Redistributable (x64). { $error } is what went
# wrong. winget is the name of a Windows command; keep it as it is.
core-warning-build-prerequisite-download-failed = 无法下载 { $runtime }（{ $error }）；安装程序将在目标电脑上尝试使用 winget。
# { $runtime } is a runtime's name. { $reason } is why the check failed, such as "it is signed by Contoso Ltd, not
# Microsoft".
core-warning-build-prerequisite-signature-failed = 下载的 { $runtime } 未通过签名检查，因此未捆绑：{ $reason }。安装程序将在目标电脑上尝试使用 winget。
# { $runtime } is a runtime's name. { $error } is what went wrong.
core-warning-build-prerequisite-hash-failed = 已下载 { $runtime }，但无法计算其哈希（{ $error }）；安装程序将改为在目标电脑上尝试使用 winget。
# { $service } is a driver's service name, such as AcmeFlt. { $error } is what went wrong.
core-warning-build-driver-copy-failed = 无法为“{ $service }”捆绑驱动程序映像：{ $error }
core-warning-build-license-not-copied = 无法复制许可证数据（缺失或无法读取）；仅记录为元数据。
core-warning-build-license-file-credential = 某个许可证文件看起来像凭据，已被排除。
# { $error } is what went wrong.
core-warning-build-license-store-walk-failed = 无法枚举许可证存储的部分内容：{ $error }
core-warning-build-license-store-credential = 许可证存储中的某个文件看起来像凭据，已被排除。
# { $error } is what went wrong.
core-warning-build-license-store-file-skipped = 已跳过一个许可证存储文件：{ $error }
