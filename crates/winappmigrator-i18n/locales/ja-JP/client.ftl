# Words of WinAppMigrator Client, the console program run on the old PC, the one an app is moving off, so that
# WinAppMigrator on the new PC can read it over the network. Message names start with client-.
#
# Its window is a console, which on Windows 7 is 80 columns wide. Each line break in a message is one on the screen:
# keep every line shorter than 80 characters, and break lines where your language needs to, not only where English
# does.


## Starting

# Shown only in English, in place of a right-to-left language the console cannot show. { $language } is that
# language's name in English, such as Arabic.
client-console-rtl-fallback = このコンソールでは { $language } を表示できないため、クライアントは英語で表示します。
# The client copies itself to a folder in Program Files before it starts. { $error } is what Windows said went wrong.
client-safe-folder-failed =
    WinAppMigrator Client を管理者だけが変更できるフォルダーから
    開始できませんでした：{ $error }
client-press-enter-to-close = このウィンドウを閉じるには、Enter キーを押してください。
# The first line of its screen. { $version } is the client's version, such as 0.6.2.
client-heading = WinAppMigrator Client { $version }
client-intro =
    新しい PC の WinAppMigrator が、ネットワーク経由でこの PC の
    アプリを読み取れるようにします。この PC では何も変更しません。
    読み取るのは Windows ドライブのスナップショットで、停止すると
    削除されます。
# { $error } is why: one of the messages under "Why it stopped" below, or what Windows said.
client-stopped = クライアントが停止しました：{ $error }
# The caretaker is a copy of the client with no window that cleans up after it. { $error } is what Windows said.
client-caretaker-failed =
    停止せずにこのウィンドウを閉じた場合、スナップショットと
    ファイアウォール規則は、次にこの PC でクライアントを実行するまで
    残ります（{ $error }）。
# Shown while the snapshot is taken, followed on the same line by client-snapshot-taken. Keep the quotes: they keep
# the space before it.
client-snapshot-taking = Windows ドライブのスナップショットを作成しています...{ " " }
client-snapshot-taken = 完了しました。
# The old PC answers the new one's search for it on the network. { $error } is what Windows said went wrong.
client-discovery-failed =
    この PC は自動的には表示されません（{ $error }）。
    新しい PC で、この PC の名前またはアドレスを入力してください。

## This PC and its pairing code. The three lines are lined up in two columns, behind two spaces; keep the quotes, which
## keep those spaces.

# { $computer_name } is this PC's name, such as OLD-PC.
client-this-pc = { "  " }この PC：            { $computer_name }
# { $windows_name } is this PC's Windows, such as Windows 7 Professional Service Pack 1. { $bits } is 32 or 64.
client-windows = { "  " }Windows：            { $windows_name }、{ $bits } ビット
# { $code } is six digits in two groups, such as 482 913.
client-pairing-code = { "  " }ペアリング コード：  { $code }
# Network PC... is the button of that name in WinAppMigrator: use the name it has there in your language.
client-pairing-steps =
    新しい PC の WinAppMigrator で「ネットワーク PC...」を選び、
    このコードを入力してください。
    移行するアプリを先に閉じて、ファイルを保存された状態にしてください。
    停止するには、ここで Enter キーを押すか、このウィンドウを閉じてください。

## While it runs. { $from } is the network address of the PC that connected, such as 192.168.1.20, or "unknown" when
## it could not be told, which the [unknown] variant words.

client-paired =
    { $from ->
        [unknown] 別の PC
       *[address] { $from }
    } とペアリングしました。必要な情報を読み取っています...
# { $count } is how many requests it answered.
client-builder-done =
    { $from ->
        [unknown] 別の PC
       *[address] { $from }
    } の処理が完了しました（{ $count ->
       *[other] { $count } 件の要求に応答
    }）。
# { $error } is what went wrong, often in Windows' words.
client-link-ended =
    { $from ->
        [unknown] 別の PC
       *[address] { $from }
    } との接続が終了しました：{ $error }
client-wrong-code =
    { $from ->
        [unknown] 別の PC
       *[address] { $from }
    } が、この PC のものではないコードを試しました。
# { $error } is what went wrong, often in Windows' words.
client-pair-failed =
    { $from ->
        [unknown] 別の PC
       *[address] { $from }
    } はペアリングできませんでした：{ $error }
client-too-many-wrong-codes =
    誤ったコードが何度も試されたため、現在この PC とペアリングできる
    PC はありません。ネットワーク上の何かがコードを推測している可能性が
    あります。ペアリングするには、クライアントを停止してから再度起動し、
    表示される新しいコードを入力してください。
# { $code } is six digits in two groups, such as 482 913.
client-new-code = 誤ったコードが多すぎます。新しいペアリング コード：{ $code }

## Stopping

client-stopping = 停止しています：スナップショットとファイアウォール規則を削除しています...
client-stop-done = 完了しました。

## Why it stopped, shown after "The client stopped: " (client-stopped), and written to follow it.

client-already-running = この PC では既にクライアントが実行中か、閉じられたクライアントの後処理中です。そちらを使用するか、しばらく待ってからもう一度起動してください。
# { $drive } is the drive Windows is on, such as C:. { $details } is what Windows said, in its own words.
client-snapshot-failed = Windows は { $drive } のスナップショットを作成できませんでした：{ $details }
# { $details } is what Windows Firewall said, in its own words.
client-firewall-failed = Windows ファイアウォールにより、クライアントの待ち受けが許可されませんでした：{ $details }

## What it takes, shown for --help. The options, such as --lang, are typed as they are and are not translated; keep
## their descriptions lined up.

client-usage =
    WinAppMigrator Client では次のオプションを使用できます：
    
      --lang <言語>    この言語で表示します（例：it-IT）。クライアントが
                       この言語に対応している場合に、Windows の表示言語の
                       代わりに使用されます。
      --code <コード>  この 6 桁のペアリング コードで開始します。新しい PC の
                       WinAppMigrator も起動するスクリプト向けです。
      --help           この説明を表示します。
