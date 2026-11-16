# Words of WinAppMigrator Client, the console program run on the old PC, the one an app is moving off, so that
# WinAppMigrator on the new PC can read it over the network. Message names start with client-.
#
# Its window is a console, which on Windows 7 is 80 columns wide. Each line break in a message is one on the screen:
# keep every line shorter than 80 characters, and break lines where your language needs to, not only where English
# does.


## Starting

# Shown only in English, in place of a right-to-left language the console cannot show. { $language } is that
# language's name in English, such as Arabic.
client-console-rtl-fallback = 此主控台無法顯示 { $language }，因此用戶端改用英文。
# The client copies itself to a folder in Program Files before it starts. { $error } is what Windows said went wrong.
client-safe-folder-failed =
    WinAppMigrator Client 無法從只有系統管理員能變更的
    資料夾啟動：{ $error }
client-press-enter-to-close = 請按 Enter 關閉此視窗。
# The first line of its screen. { $version } is the client's version, such as 0.6.2.
client-heading = WinAppMigrator Client { $version }
client-intro =
    這可讓您新電腦上的 WinAppMigrator 透過網路讀取這台電腦上的應用程式。
    它不會變更這裡的任何內容：它讀取的是 Windows 磁碟機的快照，
    並會在您停止它時移除快照。
# { $error } is why: one of the messages under "Why it stopped" below, or what Windows said.
client-stopped = 用戶端已停止：{ $error }
# The caretaker is a copy of the client with no window that cleans up after it. { $error } is what Windows said.
client-caretaker-failed =
    如果直接關閉此視窗而不是停止用戶端，快照和防火牆規則會保留，
    直到下次在這裡執行用戶端為止（{ $error }）。
# Shown while the snapshot is taken, followed on the same line by client-snapshot-taken. Keep the quotes: they keep
# the space before it.
client-snapshot-taking = 正在建立 Windows 磁碟機的快照...{ " " }
client-snapshot-taken = 完成。
# The old PC answers the new one's search for it on the network. { $error } is what Windows said went wrong.
client-discovery-failed =
    新電腦無法自動找到這台電腦（{ $error }）。
    請改在新電腦上輸入這台電腦的名稱或位址。

## This PC and its pairing code. The three lines are lined up in two columns, behind two spaces; keep the quotes, which
## keep those spaces.

# { $computer_name } is this PC's name, such as OLD-PC.
client-this-pc = { "  " }電腦名稱：     { $computer_name }
# { $windows_name } is this PC's Windows, such as Windows 7 Professional Service Pack 1. { $bits } is 32 or 64.
client-windows = { "  " }Windows：      { $windows_name }，{ $bits } 位元
# { $code } is six digits in two groups, such as 482 913.
client-pairing-code = { "  " }配對碼：       { $code }
# Network PC... is the button of that name in WinAppMigrator: use the name it has there in your language.
client-pairing-steps =
    在新電腦的 WinAppMigrator 中選擇「網路電腦...」，然後輸入此配對碼。
    請先關閉您要移轉的應用程式，讓它的檔案維持關閉時的狀態。
    若要停止，請在這裡按 Enter，或關閉此視窗。

## While it runs. { $from } is the network address of the PC that connected, such as 192.168.1.20, or "unknown" when
## it could not be told, which the [unknown] variant words.

client-paired =
    已與{ $from ->
        [unknown] 另一台電腦
       *[address] { " " }{ $from }{ " " }
    }配對。它正在讀取所需的資料...
# { $count } is how many requests it answered.
client-builder-done =
    { $from ->
        [unknown] 另一台電腦
       *[address] { $from }{ " " }
    }已完成（已回應 { $count ->
       *[other] { $count } 個要求
    }）。
# { $error } is what went wrong, often in Windows' words.
client-link-ended =
    與{ $from ->
        [unknown] 另一台電腦
       *[address] { " " }{ $from }{ " " }
    }的連線已中斷：{ $error }
client-wrong-code =
    { $from ->
        [unknown] 另一台電腦
       *[address] { $from }{ " " }
    }嘗試的配對碼不是這一組。
# { $error } is what went wrong, often in Windows' words.
client-pair-failed =
    { $from ->
        [unknown] 另一台電腦
       *[address] { $from }{ " " }
    }無法配對：{ $error }
client-too-many-wrong-codes =
    嘗試錯誤配對碼的次數過多，因此目前沒有任何電腦能與這台電腦配對。
    您的網路上可能有東西在猜測配對碼。若要配對，請停止用戶端
    並重新啟動，然後輸入它顯示的新配對碼。
# { $code } is six digits in two groups, such as 482 913.
client-new-code = 錯誤的配對碼過多。新的配對碼為：{ $code }

## Stopping

client-stopping = 正在停止：正在移除快照和防火牆規則...
client-stop-done = 完成。

## Why it stopped, shown after "The client stopped: " (client-stopped), and written to follow it.

client-already-running = 用戶端已在這台電腦上執行，或正在清理已關閉的用戶端。請使用該用戶端，或稍候再重新啟動這個用戶端。
# { $drive } is the drive Windows is on, such as C:. { $details } is what Windows said, in its own words.
client-snapshot-failed = Windows 未能建立 { $drive } 的快照：{ $details }
# { $details } is what Windows Firewall said, in its own words.
client-firewall-failed = Windows 防火牆不允許用戶端接聽連線：{ $details }

## What it takes, shown for --help. The options, such as --lang, are typed as they are and are not translated; keep
## their descriptions lined up.

client-usage =
    WinAppMigrator Client 可使用下列選項：
    
      --lang <語言>      使用此語言（例如 it-IT，若用戶端提供），
                         而不使用 Windows 的顯示語言。
      --code <代碼>      以此六位數配對碼啟動，供同時在新電腦上
                         啟動 WinAppMigrator 的指令碼使用。
      --help             顯示此說明。
