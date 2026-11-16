# Words of WinAppMigrator Client, the console program run on the old PC, the one an app is moving off, so that
# WinAppMigrator on the new PC can read it over the network. Message names start with client-.
#
# Its window is a console, which on Windows 7 is 80 columns wide. Each line break in a message is one on the screen:
# keep every line shorter than 80 characters, and break lines where your language needs to, not only where English
# does.


## Starting

# Shown only in English, in place of a right-to-left language the console cannot show. { $language } is that
# language's name in English, such as Arabic.
client-console-rtl-fallback = 此控制台无法显示 { $language }，因此客户端改用英语。
# The client copies itself to a folder in Program Files before it starts. { $error } is what Windows said went wrong.
client-safe-folder-failed =
    WinAppMigrator Client 无法从只有管理员才能更改的文件夹启动：
    { $error }
client-press-enter-to-close = 按 Enter 关闭此窗口。
# The first line of its screen. { $version } is the client's version, such as 0.6.2.
client-heading = WinAppMigrator Client { $version }
client-intro =
    此程序可让你新电脑上的 WinAppMigrator 通过网络读取此电脑上的应用。
    它不会更改此电脑上的任何内容：它读取 Windows 驱动器的快照，
    并在你停止它时删除该快照。
# { $error } is why: one of the messages under "Why it stopped" below, or what Windows said.
client-stopped = 客户端已停止：{ $error }
# The caretaker is a copy of the client with no window that cleans up after it. { $error } is what Windows said.
client-caretaker-failed =
    如果直接关闭此窗口而不是停止客户端，快照和防火墙规则将保留，
    直到下次在此电脑上运行客户端（{ $error }）。
# Shown while the snapshot is taken, followed on the same line by client-snapshot-taken. Keep the quotes: they keep
# the space before it.
client-snapshot-taking = 正在创建 Windows 驱动器的快照...{ " " }
client-snapshot-taken = 完成。
# The old PC answers the new one's search for it on the network. { $error } is what Windows said went wrong.
client-discovery-failed =
    新电脑不会自动找到此电脑（{ $error }）。
    请在新电脑上改为输入此电脑的名称或地址。

## This PC and its pairing code. The three lines are lined up in two columns, behind two spaces; keep the quotes, which
## keep those spaces.

# { $computer_name } is this PC's name, such as OLD-PC.
client-this-pc = { "  " }此电脑：       { $computer_name }
# { $windows_name } is this PC's Windows, such as Windows 7 Professional Service Pack 1. { $bits } is 32 or 64.
client-windows = { "  " }Windows：      { $windows_name }，{ $bits } 位
# { $code } is six digits in two groups, such as 482 913.
client-pairing-code = { "  " }配对码：       { $code }
# Network PC... is the button of that name in WinAppMigrator: use the name it has there in your language.
client-pairing-steps =
    在新电脑上的 WinAppMigrator 中，选择“网络电脑...”，然后输入此代码。
    请先关闭要迁移的应用，使其文件保持应用退出时的状态。
    若要停止，请在此处按 Enter，或关闭此窗口。

## While it runs. { $from } is the network address of the PC that connected, such as 192.168.1.20, or "unknown" when
## it could not be told, which the [unknown] variant words.

client-paired =
    已与{ $from ->
        [unknown] 另一台电脑
       *[address] { " " }{ $from }{ " " }
    }配对。它正在读取所需的内容...
# { $count } is how many requests it answered.
client-builder-done =
    { $from ->
        [unknown] 另一台电脑
       *[address] { $from }{ " " }
    }已完成（已应答 { $count ->
       *[other] { $count } 个请求
    }）。
# { $error } is what went wrong, often in Windows' words.
client-link-ended =
    与{ $from ->
        [unknown] 另一台电脑
       *[address] { " " }{ $from }{ " " }
    }的连接已断开：{ $error }
client-wrong-code =
    { $from ->
        [unknown] 另一台电脑
       *[address] { $from }{ " " }
    }尝试了与此不符的配对码。
# { $error } is what went wrong, often in Windows' words.
client-pair-failed =
    { $from ->
        [unknown] 另一台电脑
       *[address] { $from }{ " " }
    }无法配对：{ $error }
client-too-many-wrong-codes =
    尝试的错误代码过多，因此现在没有电脑能与此电脑配对。
    你的网络上可能有设备在猜测代码。若要配对，请停止客户端
    并重新启动，然后输入它显示的新代码。
# { $code } is six digits in two groups, such as 482 913.
client-new-code = 错误代码过多。新的配对码为：{ $code }

## Stopping

client-stopping = 正在停止：正在删除快照和防火墙规则...
client-stop-done = 完成。

## Why it stopped, shown after "The client stopped: " (client-stopped), and written to follow it.

client-already-running = 客户端已在此电脑上运行，或者正在清理已关闭的客户端。请使用正在运行的那个，或稍等片刻后再次启动此程序。
# { $drive } is the drive Windows is on, such as C:. { $details } is what Windows said, in its own words.
client-snapshot-failed = Windows 未能创建 { $drive } 的快照：{ $details }
# { $details } is what Windows Firewall said, in its own words.
client-firewall-failed = Windows 防火墙不允许客户端侦听：{ $details }

## What it takes, shown for --help. The options, such as --lang, are typed as they are and are not translated; keep
## their descriptions lined up.

client-usage =
    WinAppMigrator Client 接受以下选项：
    
      --lang <语言>      使用此语言（例如 it-IT，前提是客户端提供该语言），
                         而不是 Windows 的显示语言。
      --code <配对码>    以此六位配对码启动，供同时在新电脑上启动
                         WinAppMigrator 的脚本使用。
      --help             显示此帮助。
