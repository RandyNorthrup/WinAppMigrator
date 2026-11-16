# Words the core says about what an install may do: why an install location or something in a package is refused,
# the installer's list of what an app sets up beyond its files, the checks of a package and of an install, and whether
# there is room for either. Message names start with core-.
#
# Many messages here are a reason, the second half of a sentence that starts "... was left out:" or "that install
# location cannot be used:", so they start with a small letter and have no full stop.


## Why an install location cannot be used. Shown in red under the Install Location field.

core-review-location-state-root = 它是安装保存其记录的位置
core-review-location-has-files = 它已包含文件，且此处没有安装记录，因此请使用空文件夹
core-policy-location-not-full-path = 它必须是完整路径
core-policy-location-parent-step = 它不得包含“..”
core-policy-location-not-plain = 它必须是此电脑驱动器上的普通路径
core-policy-location-whole-drive = 它不能是整个驱动器
core-policy-location-windows = 它不能是 Windows 文件夹
core-policy-location-runs-by-itself = 它是属于 Windows 的文件夹，或者其内容会自行运行或加载
core-policy-location-shared = 其中并列存放着其他内容，因此应用需要使用其中的一个子文件夹

## Whether an app can run on this PC at all, said before anything is installed.

# { $reason } is why, as the second half of a sentence, such as "App.exe is built for Arm64, and this PC is x64, so
# Windows would not start it".
core-review-cannot-run-here = 此应用无法在此电脑上运行，因此未进行任何更改：{ $reason }
core-policy-architecture-x86 = 32 位 x86
core-policy-architecture-x64 = x64
core-policy-architecture-arm64 = Arm64
core-policy-architecture-arm32 = 32 位 Arm
core-policy-architecture-unknown = 未知体系结构
# { $code } is the number Windows gives a kind of processor, such as 0x01c4.
core-policy-machine-type = 计算机类型 { $code }
# { $names } is a list of program names, such as "a.exe, b.exe, c.exe", and { $count } how many more there are.
core-policy-names-and-more = { $names }，另有 { $count } 个
core-policy-fit-programs-not-said = 此安装程序未说明其程序的要求，因此未检查它们能否在此电脑上运行。
core-policy-fit-architecture-not-told = 无法确定此电脑的体系结构，因此未检查此应用的程序能否在其上运行。
core-policy-fit-main-not-said = 此安装程序未说明其主程序的要求，因此此处的说明依据的是它拥有的其余信息。
# { $driver } is a driver's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-driver = 其驱动程序 { $driver } 是为 { $built_for } 生成的，而此电脑是 { $here }。驱动程序必须与 Windows 完全匹配，因此此应用无法在此运行
# { $program } is a program's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-program = { $program } 是为 { $built_for } 生成的，而此电脑是 { $here }，因此 Windows 不会启动它
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-none-start = 它的所有程序都无法在此电脑 ({ $here }) 上启动：{ $programs }
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-some-other-architecture = 此应用的部分程序是为其他体系结构生成的，无法在此电脑 ({ $here }) 上启动：{ $programs }。应用的其余部分可以启动，因此这可能无关紧要。
# { $here } is a kind of processor, such as "Arm64". { $programs } is a list of program names.
core-policy-fit-emulated = 此电脑是 { $here }，因此 Windows 会通过仿真运行以下程序：{ $programs }。大多数程序以这种方式运行没有问题，但有些程序不行。
# { $program } is a program's file name. { $needs } and { $runs } are versions of Windows, such as 10.0 and 6.1.
core-policy-fit-needs-newer = { $program } 表明它需要 Windows { $needs } 或更高版本，而此电脑运行的是 { $runs }，因此 Windows 不会启动它
# { $programs } is a list of program names.
core-policy-fit-some-need-newer = 此应用的部分程序表明它们需要比此版本更新的 Windows，无法在此启动：{ $programs }。
# { $from_build } and { $this_build } are Windows build numbers, such as 22631, written as Windows writes them.
core-policy-fit-older-build = 此应用原先位于 Windows 内部版本 { $from_build } 上，而此电脑运行的是内部版本 { $this_build }。其程序表明它们可以在此 Windows 上启动，因此它可能可以正常工作，但它在原电脑上依赖的内容在这里可能不存在。
core-policy-fit-build-not-told = 无法确定此 Windows 的内部版本，因此未将其与此应用原先所在的版本进行比较。

## Room on a drive for a build or an install.

# Each of these ends the sentence core-space-shortfall, after "free for".
core-space-for-unpacking = 解压缩程序包
core-space-for-install-record = 安装记录
core-space-for-app-files = 应用的文件
core-space-for-what-is-replaced = 保留其替换的内容
core-space-for-build-work = 生成过程所用的内容
core-space-for-installer = 安装程序
core-space-for-package = 程序包
core-space-for-unpacking-and-installing = 解压缩并安装程序包
# What goes between two of those, as in "unpacking the package and the app's files". Keep the quotes: they keep the
# spaces.
core-space-purposes-joiner = 和
# { $drive } is a drive, such as C:, or a folder. { $needed } and { $free } are sizes, such as 1.2 GB. { $purposes } is
# what the room is for, such as "unpacking the package and the app's files".
core-space-shortfall = { $drive } 最多需要 { $needed } 的可用空间用于{ $purposes }，现有 { $free }
# { $drive } is a drive, such as C:, or a folder. { $needed } is a size, such as 1.2 GB.
core-space-not-told = { $drive } 未报告其可用空间，因此未检查能否容纳所需的 { $needed }
# One drive in a list of where room is needed. { $needed } and { $free } are sizes, and { $drive } a drive, such as C:.
core-space-summary = { $drive } 上最多 { $needed }（可用 { $free }）
# { $needed } is a size, and { $drive } a drive, such as C:.
core-space-summary-not-told = { $drive } 上最多 { $needed }（无法确定可用空间）
# What goes between two sentences of core-space-shortfall inside core-review-no-room. Keep the quotes: they keep the
# space.
core-review-sentence-joiner = 。
# { $shortfalls } is one or more sentences of core-space-shortfall.
core-review-no-room = 空间不足，因此未进行任何更改。{ $shortfalls }。请在那里释放一些空间，或选择其他驱动器上的安装位置。

## The installer's list of what an app sets up beyond its files: what each kind is called.

core-review-kind-service = 服务
core-review-kind-driver = 驱动程序
core-review-kind-scheduled-task = 计划任务
core-review-kind-autostart = 自动启动
core-review-kind-shortcut = 快捷方式
core-review-kind-path = PATH
core-review-kind-environment-variable = 环境变量
core-review-kind-shell-extension = 外壳扩展
core-review-kind-menu-command = 菜单命令
core-review-kind-link-type = 链接类型
core-review-kind-file-type = 文件类型
core-review-kind-com = COM
core-review-kind-firewall-rule = 防火墙规则
core-review-kind-event-log-source = 事件日志源
core-review-kind-font = 字体
core-review-kind-odbc-driver = ODBC 驱动程序
core-review-kind-odbc-source = ODBC 数据源
core-review-kind-compatibility-setting = 兼容性设置
core-review-kind-default-apps = 默认应用
core-review-kind-program-name = 程序名称
core-review-kind-other-settings = 其他设置

## The installer's list of what an app sets up: what each line says of it.

# { $file } is a program's file name. { $account } is the Windows account it runs as, such as LocalSystem. { $starts }
# says when it starts, and is not shown itself; for "other", { $start } is one of the core-review-start- words below.
core-review-service-detail =
    { $starts ->
        [automatic] 以 { $account } 身份运行 { $file }，立即启动，并在此电脑每次启动时启动
        [delayed] 以 { $account } 身份运行 { $file }，立即启动，并在此电脑每次启动后不久启动
        [manual] 以 { $account } 身份运行 { $file }，仅在有程序请求时启动
        [disabled] 以 { $account } 身份运行 { $file }，已禁用
        [other] 以 { $account } 身份运行 { $file }，{ $start }
       *[unknown] 以 { $account } 身份运行 { $file }
    }
# { $account } is a Windows account, such as LocalService.
core-policy-account-custom = { $account }（未还原自定义帐户）
# { $file } is a driver's file name.
core-review-driver-detail = 将 { $file } 加载到 Windows 内核中
# { $file } is a driver's file name. { $start } is when Windows starts it: one of the core-review-start- words below.
core-review-driver-detail-starts = 将 { $file } 加载到 Windows 内核中，{ $start }
# When a service or driver starts, ending a sentence such as "Loads disk.sys into Windows itself, starts …".
core-review-start-boot = 随 Windows 启动，先于其他任何内容
core-review-start-system = 在 Windows 启动过程中启动
core-review-start-automatic = 在此电脑每次启动时启动
core-review-start-delayed = 在此电脑每次启动后不久启动
core-review-start-manual = 仅在有程序请求时启动
core-review-start-disabled = 从不启动，因为它已被禁用
core-review-start-unknown = 以 Windows 未说明的方式启动
# { $account } is the account a task runs as: an account's name, or core-review-task-runs-as-you, or
# core-review-task-runs-as-creator.
core-review-task-detail = 以 { $account } 身份运行
# { $account } is the account a task runs as, as for core-review-task-detail.
core-review-task-detail-highest = 以 { $account } 身份运行，并使用其可拥有的最高权限
core-review-task-runs-as-you = 你（当前登录的用户）
core-review-task-runs-as-creator = 创建它的用户
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-autostart-detail =
    { $scope ->
        [everyone] 在任何人登录时运行
       *[you] 在你登录时运行
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shortcut-startup-detail =
    “启动”文件夹中的快捷方式，{ $scope ->
        [everyone] 适用于此电脑上的所有人
       *[you] 适用于你
    }
# { $file } is the file name of what the shortcut opens. { $scope } is whom it is set up for: everyone on this PC,
# or you. It is not shown itself.
core-review-shortcut-detail =
    打开 { $file }，{ $scope ->
        [everyone] 适用于此电脑上的所有人
       *[you] 适用于你
    }
# { $name } is an environment variable's name, such as PATH.
core-review-path-title = 添加到 { $name }
core-review-path-install-folder = 安装文件夹
# { $path } is a folder inside the install folder, such as bin\tools.
core-review-path-below-install-folder = 安装文件夹\{ $path }
# { $value } is the variable's value, as it will be set.
core-review-variable-detail = 设置为 { $value }
# { $count } is how many registry values.
core-review-other-settings-title =
    { $count ->
       *[other] 应用自身注册表项之外的 { $count } 项设置
    }
# { $keys } is a list of registry keys, such as HKCU\Software\Vendor.
core-review-other-settings-detail = 位于 { $keys }
# { $keys } is a list of registry keys, and { $count } how many more there are.
core-review-other-settings-detail-more = 位于 { $keys }，另有 { $count } 个
# { $way } says whether the rule allows or blocks, and incoming or outgoing connections; it is not shown itself.
# { $ports } is a list of port numbers, and { $port_count } how many. { $program } is the program's file name.
# { $networks } is a list of the kinds of network Windows names in English (domain, private, public), and
# { $network_count } how many.
# The kinds of network a firewall rule is for, as Windows Defender Firewall names its profiles in your language. They
# fill { $networks } in core-review-firewall-detail, joined into a list.
core-review-network-domain = 域
core-review-network-private = 专用
core-review-network-public = 公用
core-review-firewall-detail =
    { $way ->
        [allow-in] 允许入站
        [allow-out] 允许出站
        [block-in] 阻止入站
       *[block-out] 阻止出站
    }{ $port_count ->
        [0] 连接
       *[other] 连接（端口：{ $ports }）
    }，用于 { $program }，{ $network_count ->
        [0] 适用于所有网络
       *[other] 适用于{ $networks }网络
    }
# { $server } is the file name of what Explorer loads. { $scope } is whom it is set up for: everyone on this PC, or
# you. It is not shown itself.
core-review-shell-extension-detail =
    将 { $server } 加载到文件资源管理器中，{ $scope ->
        [everyone] 适用于此电脑上的所有人
       *[you] 适用于你
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shell-extension-detail-own-file =
    将它安装的一个文件加载到文件资源管理器中，{ $scope ->
        [everyone] 适用于此电脑上的所有人
       *[you] 适用于你
    }
# The name of a shell extension: { $handler } is its name in the registry, and { $class } the kind of file or folder it
# is for, such as * or Directory.
core-policy-shell-extension-name = { $class } 上的 { $handler }
# { $on } is the kind of file or folder whose menu it is, as the registry names it, such as * or
# Directory. { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-menu-command-detail =
    向 { $on } 的菜单添加命令，{ $scope ->
        [everyone] 适用于此电脑上的所有人
       *[you] 适用于你
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-default-verb-detail =
    更改打开此类项时执行的操作，{ $scope ->
        [everyone] 适用于此电脑上的所有人
       *[you] 适用于你
    }
core-review-client-entry-detail = 可选择它作为执行此操作的程序。不会将任何默认设置更改为它
core-review-com-permissions-title = COM 权限
core-review-com-permissions-detail = 规定谁可以启动其 COM 类，或这些类以什么身份运行
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-link-type-detail =
    此类链接使用此应用打开，{ $scope ->
        [everyone] 适用于此电脑上的所有人
       *[you] 适用于你
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-file-type-detail =
    此类型的文件使用此应用打开，{ $scope ->
        [everyone] 适用于此电脑上的所有人
       *[you] 适用于你
    }
# { $log } is the name of a Windows event log, such as Application.
core-review-event-log-source-detail = 可以使用其自身文件中的消息写入 { $log } 日志
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-font-detail =
    在你登录时从安装文件夹加载，{ $scope ->
        [everyone] 适用于此电脑上的所有人
       *[you] 适用于你
    }
core-review-odbc-driver-detail = 加载到使用 ODBC 访问数据库的程序中
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-odbc-source-detail =
    数据库连接，{ $scope ->
        [everyone] 适用于此电脑上的所有人
       *[you] 适用于你
    }
core-review-default-apps-detail = 可在“默认应用”中选择。不会将任何默认设置更改为它
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-program-name-detail =
    “运行”和其他程序可以通过此名称找到它，{ $scope ->
        [everyone] 适用于此电脑上的所有人
       *[you] 适用于你
    }
# { $server } is the file name of what serves the classes.
core-review-com-title = { $server } 中的 COM
core-review-com-title-own-file = 它安装的文件中的 COM
# { $count } is how many registry values.
core-review-com-detail =
    { $count ->
       *[other] 其他程序可以启动的类的 { $count } 项设置
    }
core-review-com-support-title = COM 类型库、接口和 AppID
# { $libraries }, { $interfaces }, and { $app_ids } are how many registry values there are for its type libraries, its
# interfaces, and its AppIDs.
core-review-com-support-detail = 其类型库（{ $libraries } 项）、接口（{ $interfaces } 项）和 AppID（{ $app_ids } 项）的设置，与其类配套使用
core-review-compatibility-title = 其程序的启动方式
# { $count } is how many registry values.
core-review-compatibility-detail =
    { $count ->
       *[other] 针对它安装的程序的 { $count } 项兼容性设置
    }

## The installer's notes on whose folders and settings these are, and what is not coming.

# { $account } is the name of a Windows account, such as alice.
core-review-account-signed-in = 用户数据和按用户的设置将归属于在此登录的 { $account }，而不是运行此程序的管理员。
# { $account } is the name of a Windows account, such as alice.
core-review-account-no-desktop = 此会话中没有人登录，因此用户数据和按用户的设置将归属于运行此程序的帐户 { $account }。
# { $reason } is what Windows said, in its own words.
core-review-account-not-told = 无法确定谁已登录（{ $reason }），因此已排除按用户的设置，用户数据将放在安装位置下。
# { $reason } is what Windows said, in its own words.
core-review-signed-in-unknown = 无法确定谁已登录：{ $reason }
# Headings of what is left out, shared by the app's list and the installer's notes.
core-review-left-out-credentials = 未包含在此安装程序中，因为它们看起来像凭据
core-review-left-out-settings = 帐户为此应用保存的设置和历史记录，仅在使用“用户数据”选项时才会包含
core-review-left-out-certificates = 此应用似乎依赖其原电脑上受信任的证书，这些证书不会被安装
core-review-left-out-fonts = 这些字体以此应用的名称命名，但没有任何信息表明它们属于此应用，因此不会被安装
# { $names } is a list of names, such as the names of saved passwords.
core-review-note-credentials = 未包含在此安装程序中，因为它们看起来像凭据：{ $names }。此电脑上的相应内容保持不变，你可能需要重新输入它们。
# { $names } is a list of names of license data.
core-review-note-licenses = 未包含在此安装程序中，因为生成时“许可证和激活数据”选项处于关闭状态：{ $names }。
# { $names } is a list of places an account keeps settings, such as folders.
core-review-note-settings = 未包含在此安装程序中，因为生成时“用户数据”选项处于关闭状态：帐户为此应用保存的设置和历史记录，位于 { $names }。此应用在这里启动时就像初次使用一样。
# { $names } is a list of certificates, each as core-review-certificate.
core-review-note-certificates = 此应用似乎依赖其原电脑上受信任的证书，这些证书不会被安装：{ $names }。是否在此信任它们由你决定。
# { $names } is a list of font names.
core-review-note-fonts = 这些字体以此应用的名称命名，但没有任何信息表明它们属于此应用，因此不会被安装：{ $names }。
# { $names } is a list of names.
core-review-note-left-out-by-choice = 已被此安装程序的生成者排除：{ $names }。
# { $names } is a list of names.
core-review-note-added-by-choice = 已被此安装程序的生成者添加，因为生成者认为它们属于此应用：{ $names }。
# { $name } is a certificate's subject, such as CN=Contoso, and { $store } the certificate store it was in, such as
# Root.
core-review-certificate = { $store } 中的 { $name }
# { $names } is a list of names, and { $count } how many more there are.
core-review-names-and-more = { $names }，另有 { $count } 个

## Why something in a package is left out. Each is the second half of a sentence of core-warning-review-left-out.

core-review-no-account = 它属于某个帐户的设置，但无法确定谁已登录
core-review-name-not-accepted = Windows 不接受其名称
# { $error } is what Windows said, in its own words.
core-review-service-unreadable = 无法读取其当前设置：{ $error }
core-review-service-dependency = 它依赖于名称无效的服务
# { $path } is the program the service runs.
core-review-service-runs-other = 它运行 { $path }，而此程序包不安装该程序
core-review-service-settings = 它的设置指向此程序包不安装的内容，或指向不属于它的注册表项
core-review-driver-no-hash = 它没有可用于检查的哈希
core-review-license-for-the-machine = 为整台电脑存储的许可证仅作报告
core-review-license-starts-something = 许可证没有理由启动任何内容
core-review-task-not-captured = 未捕获其定义
# { $error } is what was wrong with the task's XML, in English.
core-review-task-unreadable = 无法读取其定义：{ $error }
core-review-task-no-account = 它以安装它的用户身份运行，但无法确定此处谁已登录
# { $path } is what the shortcut opens.
core-review-shortcut-opens-other = 它打开 { $path }，而此程序包不安装该项
# { $service } is the service's name, and { $path } the program it runs now.
core-policy-service-exists = 此电脑上已存在 { $service }，它运行的 { $path } 并非来自此处之前的安装。如果它是此应用的旧副本，请先将其卸载
core-policy-key-control-characters = 其注册表项名称包含控制字符
core-policy-value-control-characters = 其值的名称包含控制字符
core-policy-registry-link = 它是注册表链接
# { $reason } is why the key it stands for is left out, such as "no app install should change this key".
core-policy-virtual-store = 它代表此电脑的一个注册表项，而{ $reason }
core-policy-registry-part = 任何应用安装都不应写入注册表的这一部分
core-policy-key-refused = 任何应用安装都不应更改此注册表项
core-policy-net-debugger = 它会为每个 .NET 程序指定调试器
core-policy-net-install-root = 它会更改每个 .NET 程序加载 .NET 的位置
core-policy-class-redirects = 它会更改某类程序或文件夹的打开方式
core-policy-app-path-name = 它用文件名以外的名称指定程序
core-policy-app-path-not-installed = 它会为此程序包不安装的文件注册程序名称
core-policy-shared-default-verb = 它会更改打开某一类所有文件或文件夹时执行的操作
core-policy-command-runs-other = 它添加的命令会运行此程序包不安装的内容
core-policy-autostart-not-command = 它会自动启动，但其值不是命令
core-policy-autostart-runs-other = 它会自动启动，并运行此程序包不安装的内容
core-policy-environment-own-rules = 环境变量按其自身的规则还原
core-policy-service-not-restored = 它属于此程序包不还原的服务
core-policy-service-own-rules = 服务自身的设置只能按其规则还原
core-policy-service-dll = 其 ServiceDll 不是此程序包安装的文件
core-policy-event-log-source-only = 只能还原事件日志源自身的值
core-policy-security-log = 任何应用都不会写入“安全”日志
core-policy-event-log-messages = 其消息将来自此程序包不安装的文件
core-policy-type-library = 其类型库不是此程序包安装的文件
# { $name } is a program's file name, such as app.exe.
core-policy-application-opened = 它会更改 { $name } 的打开方式，而此程序包不安装该程序
core-policy-event-log-no-file = 事件日志源必须从文件中获取其消息
core-policy-com-runs-other = 它运行的内容不是此程序包安装的文件
core-policy-com-redirect = 它会使此类变成此程序包不安装的内容
core-policy-com-server = 它运行的内容既不是此程序包安装的文件，也不属于 Windows
core-policy-interface = 它会使用此程序包不安装的内容来决定如何调用接口
# { $name } is a program's file name, such as app.exe.
core-policy-app-id-started = 它会更改 { $name } 的启动方式，而此程序包不安装该程序
core-policy-app-id-surrogate = 它将由此程序包不安装的程序托管
core-policy-app-id-service = 它将在此程序包不还原的服务中运行
core-policy-app-id-run-as = 它将以其自己的帐户运行
core-policy-app-id-link = 它没有指定 AppID
core-policy-app-id-remote = 它将在此电脑以外的位置运行
core-policy-odbc-own-driver = 那是 ODBC 自身的设置，而不是驱动程序
core-policy-odbc-driver-not-installed = 其驱动程序不是此程序包安装的文件
core-policy-loads-other-file = 它会加载此程序包不安装的文件
core-policy-odbc-own-source = 那是 ODBC 自身的设置，而不是数据源
core-policy-odbc-source-missing = 它列出了此程序包中没有的数据源
core-policy-odbc-source-driver = 其驱动程序既不是此程序包安装的文件，也不属于 Windows
core-policy-shell-extension = 它会将一个并非此程序包所安装文件的外壳扩展加载到文件资源管理器中
core-policy-not-text = 它包含的内容不是其应被读取为的文本
core-policy-network-place = 它指向网络上的某个位置
core-policy-font = 它会注册此程序包不安装的字体，或 Windows 文件夹中的字体
core-policy-compatibility = 它会更改此程序包不安装的程序的运行方式
core-policy-capabilities = 它列出的应用的详细信息不在此程序包中
core-policy-default-program = 它会更改默认用于打开内容的程序
core-policy-url-protocol = 它声明自己是链接类型，但打开时不使用此程序包安装的任何内容
core-policy-command-handler = 它添加的命令由此程序包不安装的内容处理
core-policy-installer-command = 它会运行只有 Windows Installer 才能读取的内容
core-policy-creates-class = 它会创建此程序包不安装的类
core-policy-opens-links = 它使用此程序包不安装的内容打开链接
core-policy-opens-files = 它使用此程序包不安装的内容打开文件
core-policy-firewall-name = 其名称无法原样沿用
core-policy-firewall-description = 其描述无法原样沿用
core-policy-firewall-every-program = 它适用于所有程序，而一条规则只能针对一个程序
# { $path } is the program the rule is for.
core-policy-firewall-program = 它针对 { $path }，而此程序包不安装该程序
core-policy-firewall-direction = 它没有指明可设置的方向
core-policy-firewall-action = 它会覆盖其他规则，任何应用的规则都不应如此
# { $setting } is a part of a firewall rule, as the scan of its PC described it.
core-policy-firewall-not-carried = 它规定了 { $setting }，该设置无法按原样设置，或会削弱其他规则的作用
core-policy-firewall-service = 它针对此程序包不还原的服务
core-policy-firewall-parts = 它的一部分不是防火墙规则可以表达的内容
core-policy-folder-already-there = 该文件夹在安装位置之外已存在，且没有任何信息表明它属于此应用，因此不会在其中放置任何程序
core-policy-variable-name = 其名称不是有效的变量名
core-policy-variable-relied-on = Windows 和其他程序依赖于此变量
core-policy-task-name = 其名称不是 Windows 接受的名称
core-policy-task-folder = 其文件夹不是 Windows 接受的文件夹
core-policy-task-windows = 它会替换 Windows 自己的某个任务
core-policy-task-other-action = 它执行的操作不是运行程序
core-policy-task-no-program = 它没有指定要运行的程序
# { $path } is the program the task runs.
core-policy-task-runs-other = 它运行此程序包不安装的内容：{ $path }
core-policy-task-exists-unreadable = 已存在同名任务，且无法读取
core-policy-task-exists-other = 已存在同名任务，且它不运行此应用
# { $path } is the program the task already on this PC runs.
core-policy-task-exists-runs = 已存在同名任务，它运行的 { $path } 并非来自此处之前的安装
core-policy-shortcut-not-lnk = 它不是 .lnk 快捷方式
core-policy-shortcut-place = 它不在“开始”菜单或桌面上
core-policy-short-name = 其路径使用了无法检查的名称
core-policy-shortcut-no-folder = 此电脑上没有供它使用的此类文件夹
core-policy-data-startup = 它会放入“启动”文件夹，并在每次登录时运行
core-policy-data-powershell = 它会放入 PowerShell 启动时会运行其中内容的位置
core-policy-data-windows = 它会放入属于 Windows 的文件夹
core-policy-license-outside = 它位于安装位置和所有已知数据文件夹之外
core-rewrite-cdata = 它包含 CDATA 节
core-rewrite-too-much-to-look-through = 此程序包的设置中包含的内容太多，无法检查它们指向的内容

## Warnings a review gives, which the installer shows with what it sets up.

# { $kind } says what was left out, and is not shown itself. { $item } is its path or name, or a registry value such
# as HKLM\Software\App\Name. { $reason } is why, as the second half of the sentence, such as "it cannot be a whole
# drive".
core-warning-review-left-out =
    { $kind ->
        [file] 文件 { $item } 已被排除：{ $reason }。
        [folder] 文件夹 { $item } 已被排除：{ $reason }。
        [service] 服务 { $item } 已被排除：{ $reason }。
        [driver] 驱动程序 { $item } 已被排除：{ $reason }。
        [firewall-rule] 防火墙规则 { $item } 已被排除：{ $reason }。
        [license-data] 位于 { $item } 的许可证数据已被排除：{ $reason }。
        [license-value] 许可证值 { $item } 已被排除：{ $reason }。
        [scheduled-task] 计划任务 { $item } 已被排除：{ $reason }。
        [shortcut] 快捷方式 { $item } 已被排除：{ $reason }。
        [environment-variable] 环境变量 { $item } 已被排除：{ $reason }。
       *[value] { $item } 已被排除：{ $reason }。
    }
core-warning-review-package-path-missing = 可复制的文件没有程序包相对路径。
# { $count } is the most empty folders a package makes.
core-warning-review-too-many-empty-folders = 此程序包列出了超过 { $count } 个空文件夹，只会创建其中的 { $count } 个。
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Documents folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-documents-elsewhere =
    { $count ->
       *[other] 来自“文档”的 { $count } 个文件将改为放在 { $target } 下名为 _external 的文件夹中，因为此电脑的“文档”文件夹位于 { $folder }，而只会写入你的配置文件内的文件夹。若要使用这些文件，请自行将它们移到你的“文档”文件夹中。
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-documents-unknown =
    { $count ->
       *[other] 来自“文档”的 { $count } 个文件将改为放在 { $target } 下名为 _external 的文件夹中，因为无法确定此电脑的“文档”文件夹所在的位置。若要使用这些文件，请自行将它们移到你的“文档”文件夹中。
    }
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Saved Games folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-elsewhere =
    { $count ->
       *[other] 来自“保存的游戏”的 { $count } 个文件将改为放在 { $target } 下名为 _external 的文件夹中，因为此电脑的“保存的游戏”文件夹位于 { $folder }，而只会写入你的配置文件内的文件夹。若要使用这些文件，请自行将它们移到你的“保存的游戏”文件夹中。
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-unknown =
    { $count ->
       *[other] 来自“保存的游戏”的 { $count } 个文件将改为放在 { $target } 下名为 _external 的文件夹中，因为无法确定此电脑的“保存的游戏”文件夹所在的位置。若要使用这些文件，请自行将它们移到你的“保存的游戏”文件夹中。
    }

## Errors when a folder that only administrators may change cannot be made so, or checked.

# { $path } is a folder.
core-privilege-not-a-folder = { $path } 是链接或文件，而不是文件夹
# { $path } is a folder. { $error } is what Windows said, in its own words.
core-privilege-not-made-here = { $path } 不是由 WinAppMigrator 创建的，其他人可以更改其中的内容。管理员应检查并删除它：{ $error }
# { $path } is a folder.
core-privilege-others-put-something-in = { $path } 中有并非由 WinAppMigrator 创建的内容，其他人可以更改它。管理员应检查并删除它
# { $path } is a folder.
core-privilege-became-a-link-in-use = { $path } 在使用期间变成了链接
# { $path } is a folder.
core-privilege-became-a-link-looked-at = { $path } 在查看期间变成了链接
# { $path } is a file. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-users-cannot-start = 无法允许用户启动 { $path }：{ $said }
core-privilege-elevation-unknown = 无法确定此进程是否已提升权限
# { $path } is a folder. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-cannot-restrict = 无法将 { $path } 限制为仅管理员可访问：{ $said }
# { $path } is a folder. { $said } is the folder's permissions as Windows wrote them, and what else it printed.
core-privilege-not-restricted = 无法将 { $path } 限制为仅管理员可访问，其所在驱动器可能不保留权限：{ $said }
# { $path } is a folder.
core-privilege-written-to-first = 在 { $path } 受到保护之前，另一个进程已向其写入
core-privilege-no-program-data = Windows 未说明 ProgramData 的位置

## Checks of a package's files, and of what an install put in place. Each is said of one file, value, service, or the
## like, which the report names beside it.

# { $path } is the installer's file.
core-verify-no-package = { $path } 中没有程序包
# { $path } is a folder.
core-verify-not-unpacked = 此安装程序的程序包未解压缩到 { $path }
core-verify-schema-supported = 程序包清单架构受支持。
core-verify-packaged-file-missing = 缺少打包的文件。
core-verify-no-package-path = 可复制的文件没有程序包相对路径。
core-verify-folder-there = 空文件夹存在。
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-unreadable = 无法读取它以进行检查：{ $error }
core-verify-restored-file-missing = 缺少已还原的文件。
# { $expected } and { $actual } are sizes in bytes.
core-verify-size-mismatch = 文件大小不匹配。预期为 { $expected } 字节，实际为 { $actual } 字节。
core-verify-no-hash = 清单未记录此文件的哈希，因此无法检查。
core-verify-hash-mismatch = SHA-256 哈希不匹配。
core-verify-file-matches = 文件大小和哈希与清单匹配。
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-payload-failed = 许可证有效负载验证失败：{ $error }
core-verify-license-payload-missing = 缺少打包的许可证有效负载。
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-failed = 许可证存储验证失败：{ $error }
core-verify-license-store-file-missing = 缺少打包的许可证存储文件。
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-path-invalid = 打包的许可证存储路径无效：{ $error }
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-prerequisite-failed = 必备组件有效负载验证失败：{ $error }
core-verify-prerequisite-missing = 缺少捆绑的必备组件有效负载。
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-failed = 驱动程序映像验证失败：{ $error }
core-verify-bundled-driver-missing = 缺少捆绑的驱动程序映像。
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-path-invalid = 捆绑的驱动程序路径无效：{ $error }
core-verify-prerequisite-present = 目标电脑上存在运行时必备组件。
core-verify-prerequisite-not-detected = 未检测到运行时必备组件；它可能需要重启或手动完成（有关确切结果，请参阅还原日志）。
core-verify-prerequisite-not-probed = 无法通过注册表探测必备组件是否存在；安装已尽力完成。
core-verify-license-file-missing = 缺少已还原的许可证文件。
core-verify-license-store-restored-missing = 缺少已还原的许可证存储文件。
core-verify-registry-key-there = 注册表项存在。
core-verify-registry-value-matches = 注册表值与存档数据匹配。
core-verify-registry-value-differs = 注册表值与存档数据不匹配。
core-verify-firewall-rule-there = Windows 防火墙中有此规则。
core-verify-variable-matches = 环境变量值与存档数据匹配。
core-verify-variable-differs = 环境变量值与存档数据不匹配。
# { $error } is what Windows said, in its own words.
core-verify-variable-unreadable = 环境变量缺失或无法读取：{ $error }
core-verify-service-runs-other = 服务存在，但未使用其参数运行已还原的二进制文件。
core-verify-service-other-account = 服务存在，但未以预期的帐户运行。
core-verify-service-matches = 服务存在，并以其参数和帐户运行已还原的二进制文件。
core-verify-service-missing = 缺少服务。
# { $error } is what Windows said, in its own words.
core-verify-service-unreadable = 无法读取服务：{ $error }
core-verify-driver-other-image = 驱动程序服务未指向已还原的映像。
core-verify-driver-not-registered = 还原后未注册捆绑的驱动程序服务。
# { $error } is what Windows said, in its own words.
core-verify-driver-unreadable = 无法读取驱动程序服务：{ $error }
core-verify-driver-image-missing = 缺少驱动程序映像。
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-unchecked = 无法检查驱动程序映像：{ $error }
core-verify-task-matches = 计划任务存在，并运行已还原的程序。
core-verify-task-runs-other = 计划任务存在，但不运行已还原的程序。
core-verify-task-missing = 缺少计划任务。
# { $error } is what Windows said, in its own words.
core-verify-task-unreadable = 计划任务查询失败：{ $error }
core-verify-shortcut-matches = 快捷方式存在，且目标路径与已还原的目标匹配。
core-verify-shortcut-opens-other = 快捷方式存在，但目标路径与已还原的目标不匹配。
core-verify-shortcut-missing = 缺少快捷方式。
# { $error } is what Windows said, in its own words.
core-verify-shortcut-unreadable = 快捷方式查询失败：{ $error }

## Warnings and errors a check gives, which the app and the installer show.

core-warning-verify-schema-unsupported = 程序包清单架构不受支持。
# { $error } is what went wrong, in Windows' or the program's own words.
core-warning-verify-package-file-failed = 程序包文件验证失败：{ $error }
core-warning-verify-folder-missing = 缺少空文件夹。
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-key-missing = 注册表项缺失或无法读取：{ $error }
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-value-missing = 注册表值缺失或无法读取：{ $error }
core-warning-verify-registry-kind-missing = 缺少注册表值类型。
core-warning-verify-firewall-rule-missing = Windows 防火墙中没有此规则。
core-warning-verify-service-scope = 服务验证读取 HKLM 服务配置，不会启动服务。
core-warning-verify-driver-manual = WinAppMigrator 不会再分发一个或多个供应商驱动程序或 Windows 自带的驱动程序；请在目标电脑上使用供应商已签名的驱动程序包重新安装它们。
core-warning-verify-task-scope = 计划任务验证仅检查注册情况和命令引用。

## The verification report written beside a package, verification_report.md. Its headings follow a "#".

core-verify-report-title = WinAppMigrator 验证报告
# { $time } is when, as a date and time in UTC, such as 2026-11-15T10:00:00Z.
core-verify-report-verified = 验证时间 (UTC)：{ $time }
# { $valid } is true or false, which are not translated.
core-verify-report-package-valid = 程序包有效：{ $valid }
# { $valid } is true or false, which are not translated.
core-verify-report-restore-valid = 还原有效：{ $valid }
core-verify-report-restore-not-run = 还原有效：未运行
core-verify-report-items = 项目
# { $message } is what a check found, and { $source } the file or setting it looked at.
core-verify-report-item-from = { $message } 来源：{ $source }
core-verify-report-warnings = 警告
core-verify-report-errors = 错误
core-verify-report-none = 无。
