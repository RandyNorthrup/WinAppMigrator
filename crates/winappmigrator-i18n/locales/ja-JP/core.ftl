# Words the core writes for people: warnings, refusals, errors, and the installer's list of what it sets up.
# Message names start with core-.


## Warnings a package records when it is built, which an installer shows later, maybe on another PC, in that PC's
## language.

# { $count } is how many shortcuts.
core-warning-shortcuts-unreadable =
    { $count ->
       *[other] このドライブ上の { $count } 個のショートカットにはファイルが指定されていないため、このアプリを開くものかどうかを判別できませんでした。
    }

## Undoing an install: what an uninstall, or a failed install, says it put back, left, or could not undo. Each line is
## shown on its own, in a list.

# { $app_name } is the app's name.
core-rollback-record-damaged = { $app_name } の記録の一部を読み取れなかったため、そこに記載されていたものはそのまま残っています。また、置き換えられたファイルが元に戻らない可能性があります。
core-rollback-settings-entry-stays = 「設定」にあるアプリのエントリは残ります。そこからこの処理をもう一度実行できるようにするためです。
# { $name } is a runtime's name, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-rollback-prerequisite-stays = { $name } はインストールされたか、インストールされた可能性があります。ほかのプログラムが使用している可能性があるため、残ります。
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-drive-not-connected = このインストールで { $folder } に配置したものは、接続されていないドライブ上にあります。ドライブを接続すると、この処理を完了できます。
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-record-part-unread = 記録のその部分を読み取れなかったため、このインストールで { $folder } に配置したものは残ります。

## Why a file or folder was left, shown after "was left:" in the messages below them. Each is part of a sentence, so
## starts with a small letter and has no full stop.

# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-being-changed = この処理の実行中に何かが { $folder } を変更していたため、そこではそれ以上何も行いませんでした。変更しているものを閉じて、もう一度実行してください
# { $folder } is a folder; { $error } is what Windows said went wrong.
core-rollback-reason-not-opened = { $folder } を開けませんでした：{ $error }
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-no-record = { $folder } の場所の記録がありません
# { $folder } is a folder; { $actual } is the folder it now leads to.
core-rollback-reason-moved = { $folder } のリンク先が { $actual } になっており、インストール時とは異なります
# Why a service was not stopped.
core-rollback-reason-did-not-stop = 時間内に停止しませんでした

## Files and folders

# { $path } is a folder; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-folder-left = フォルダー { $path } はそのまま残しました：{ $reason }。
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-removed = フォルダー { $path } を削除しました
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-holds-others = フォルダー { $path } には、このインストールで配置していないファイルが含まれているため、残ります。
# { $path } is a folder; { $error } is what Windows said went wrong.
core-rollback-folder-not-removed = フォルダー { $path } を削除できませんでした：{ $error }。
# { $path } is a file; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-file-left = ファイル { $path } はそのまま残しました：{ $reason }。
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-folder-gone = { $path } は、元のフォルダーがなくなったため、元に戻しません。
# { $path } is a file; { $error } is what went wrong.
core-rollback-partial-not-removed = 書き込みが途中のファイルを削除できませんでした：{ $path }（{ $error }）。
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-checked = { $path } を確認できませんでした：{ $error }。
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-removed = { $path } を削除しました
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-removed = { $path } を削除できませんでした：{ $error }。
# { $path } is a file this install had replaced, and has put back.
core-rollback-file-put-back = { $path } を元に戻しました
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-put-back = { $path } を元に戻せませんでした：{ $error }。
# Why a file could not be put back.
core-rollback-backup-gone = バックアップがなくなっています
# { $path } is a file.
core-rollback-no-parent-folder = { $path } に親フォルダーがありません

## A file that stays. { $why } says why: still-used (something that stays still uses it), changed (it has changed since
## it was installed), or not-asked (it is someone's data, and they did not ask for it to go). { $path } is the file.

core-rollback-file-stays =
    { $why ->
        [still-used] { $path } は、残るものがまだ使用しているため残ります。
        [changed] { $path } は、インストール後に変更されているため残ります。
       *[not-asked] { $path } は、削除が要求されていないため残ります。
    }
# The file it replaced was to be put beside it, under another name, and every name was taken.
core-rollback-file-stays-no-name-beside =
    { $why ->
        [still-used] { $path } は、残るものがまだ使用しているため残ります。置き換えられたファイルは、使用できる名前がすべて使われているため、横に配置できませんでした。
        [changed] { $path } は、インストール後に変更されているため残ります。置き換えられたファイルは、使用できる名前がすべて使われているため、横に配置できませんでした。
       *[not-asked] { $path } は、削除が要求されていないため残ります。置き換えられたファイルは、使用できる名前がすべて使われているため、横に配置できませんでした。
    }
# { $name } is the name the file it replaced now has, such as settings.ini.before-install.
core-rollback-file-stays-beside =
    { $why ->
        [still-used] { $path } は、残るものがまだ使用しているため残ります。置き換えられたファイルは、{ $name } として横に配置されています。
        [changed] { $path } は、インストール後に変更されているため残ります。置き換えられたファイルは、{ $name } として横に配置されています。
       *[not-asked] { $path } は、削除が要求されていないため残ります。置き換えられたファイルは、{ $name } として横に配置されています。
    }
# { $error } is what went wrong.
core-rollback-file-stays-not-beside =
    { $why ->
        [still-used] { $path } は、残るものがまだ使用しているため残ります。置き換えられたファイルを横に配置できませんでした：{ $error }。
        [changed] { $path } は、インストール後に変更されているため残ります。置き換えられたファイルを横に配置できませんでした：{ $error }。
       *[not-asked] { $path } は、削除が要求されていないため残ります。置き換えられたファイルを横に配置できませんでした：{ $error }。
    }

## The registry and environment variables. { $kind } says what { $name } is: key (a registry key, such as
## HKLM\Software\App), value (a registry value, such as HKLM\Software\App\Version), user-variable or machine-variable
## (an environment variable of the user's or of the whole PC, such as APP_HOME), or user-list or machine-list (a
## variable that is a list of folders, such as Path). { $error } is what Windows said went wrong.

# How a registry key's unnamed value is shown, as Registry Editor shows it.
core-registry-default-value-name = (既定)
core-rollback-setting-other-account =
    { $kind ->
        [key] キー { $name } は、インストールしたアカウントに属しており、そのアカウントがサインインしていないため残ります。
        [user-variable] ユーザー環境変数 { $name } は、インストールしたアカウントに属しており、そのアカウントがサインインしていないため残ります。
        [machine-variable] システム環境変数 { $name } は、インストールしたアカウントに属しており、そのアカウントがサインインしていないため残ります。
        [user-list] ユーザーの { $name } は、インストールしたアカウントに属しており、そのアカウントがサインインしていないため残ります。
        [machine-list] システムの { $name } は、インストールしたアカウントに属しており、そのアカウントがサインインしていないため残ります。
       *[value] レジストリ値 { $name } は、インストールしたアカウントに属しており、そのアカウントがサインインしていないため残ります。
    }
core-rollback-setting-not-reached =
    { $kind ->
        [key] キー { $name } にアクセスできませんでした：{ $error }。
        [user-variable] ユーザー環境変数 { $name } にアクセスできませんでした：{ $error }。
        [machine-variable] システム環境変数 { $name } にアクセスできませんでした：{ $error }。
        [user-list] ユーザーの { $name } にアクセスできませんでした：{ $error }。
        [machine-list] システムの { $name } にアクセスできませんでした：{ $error }。
       *[value] レジストリ値 { $name } にアクセスできませんでした：{ $error }。
    }
core-rollback-setting-not-read =
    { $kind ->
        [key] キー { $name } を読み取れませんでした：{ $error }。
        [user-variable] ユーザー環境変数 { $name } を読み取れませんでした：{ $error }。
        [machine-variable] システム環境変数 { $name } を読み取れませんでした：{ $error }。
        [user-list] ユーザーの { $name } を読み取れませんでした：{ $error }。
        [machine-list] システムの { $name } を読み取れませんでした：{ $error }。
       *[value] レジストリ値 { $name } を読み取れませんでした：{ $error }。
    }
core-rollback-setting-changed =
    { $kind ->
        [user-variable] ユーザー環境変数 { $name } は、インストール後に変更されているため残ります。
        [machine-variable] システム環境変数 { $name } は、インストール後に変更されているため残ります。
        [user-list] ユーザーの { $name } は、インストール後に変更されているため残ります。
        [machine-list] システムの { $name } は、インストール後に変更されているため残ります。
       *[value] レジストリ値 { $name } は、インストール後に変更されているため残ります。
    }
# Said when it is back as it was before the install.
core-rollback-setting-put-back =
    { $kind ->
        [user-variable] ユーザー環境変数 { $name } を元に戻しました
        [machine-variable] システム環境変数 { $name } を元に戻しました
        [user-list] ユーザーの { $name } を元に戻しました
        [machine-list] システムの { $name } を元に戻しました
       *[value] レジストリ値 { $name } を元に戻しました
    }
core-rollback-setting-not-put-back =
    { $kind ->
        [user-variable] ユーザー環境変数 { $name } を元に戻せませんでした：{ $error }。
        [machine-variable] システム環境変数 { $name } を元に戻せませんでした：{ $error }。
        [user-list] ユーザーの { $name } を元に戻せませんでした：{ $error }。
        [machine-list] システムの { $name } を元に戻せませんでした：{ $error }。
       *[value] レジストリ値 { $name } を元に戻せませんでした：{ $error }。
    }
# A registry value the install wrote, which is the user's data, and stays unless they asked for it to go.
core-rollback-value-kept-by-request = レジストリ値 { $name } は、削除が要求されていないため残ります。
core-rollback-key-holds-others = キー { $name } には、このインストールで配置していないものが含まれているため、残ります。
core-rollback-key-removed = キー { $name } を削除しました
core-rollback-key-not-removed = キー { $name } を削除できませんでした：{ $error }。
# { $kind } is user-list or machine-list.
core-rollback-list-entries-removed =
    { $kind ->
        [user-list] ユーザーの { $name } から、このインストールのエントリを削除しました
       *[machine-list] システムの { $name } から、このインストールのエントリを削除しました
    }
core-rollback-list-entries-not-removed =
    { $kind ->
        [user-list] ユーザーの { $name } から、このインストールのエントリを削除できませんでした：{ $error }。
       *[machine-list] システムの { $name } から、このインストールのエントリを削除できませんでした：{ $error }。
    }

## Services and drivers. { $kind } is service or driver; { $name } is its name, such as AppUpdater. { $error } and
## { $reason } are what Windows said went wrong.

core-rollback-service-not-read =
    { $kind ->
        [driver] ドライバー { $name } を読み取れませんでした：{ $error }。
       *[service] サービス { $name } を読み取れませんでした：{ $error }。
    }
core-rollback-service-changed =
    { $kind ->
        [driver] ドライバー { $name } は、インストール後に変更されているため残ります。
       *[service] サービス { $name } は、インストール後に変更されているため残ります。
    }
core-rollback-service-not-stopped =
    { $kind ->
        [driver] ドライバー { $name } を停止できませんでした（{ $reason }）。使用中のものは、PC を再起動するまで残る可能性があります。
       *[service] サービス { $name } を停止できませんでした（{ $reason }）。使用中のものは、PC を再起動するまで残る可能性があります。
    }
core-rollback-service-stopped-for-this =
    { $kind ->
        [driver] ドライバー { $name } は、この処理のために停止しました。必要なときにもう一度開始してください。
       *[service] サービス { $name } は、この処理のために停止しました。必要なときにもう一度開始してください。
    }
core-rollback-service-not-removed =
    { $kind ->
        [driver] ドライバー { $name } を削除できませんでした：{ $reason }。
       *[service] サービス { $name } を削除できませんでした：{ $reason }。
    }
core-rollback-service-marked-for-removal =
    { $kind ->
        [driver] ドライバー { $name } は削除対象としてマークされました。PC を再起動したとき、またはドライバーを開いているものがそれを解放したときに削除されます。
       *[service] サービス { $name } は削除対象としてマークされました。PC を再起動したとき、またはサービスを開いているものがそれを解放したときに削除されます。
    }
core-rollback-service-removed =
    { $kind ->
        [driver] ドライバー { $name } を削除しました
       *[service] サービス { $name } を削除しました
    }
core-rollback-service-not-restored =
    { $kind ->
        [driver] ドライバー { $name } を元の状態に戻せませんでした：{ $reason }。
       *[service] サービス { $name } を元の状態に戻せませんでした：{ $reason }。
    }
core-rollback-service-restored =
    { $kind ->
        [driver] ドライバー { $name } の設定を元に戻しました
       *[service] サービス { $name } の設定を元に戻しました
    }
core-rollback-service-not-restarted = サービス { $name } はこのインストールの前に実行されていましたが、再び開始されませんでした：{ $reason }。
# { $account } is the account the service ran as, such as CONTOSO\backup.
core-rollback-service-account-again = サービス { $name } は { $account } として実行されていました。パスワードは保存されていないため、「サービス」でアカウントをもう一度設定してください。

## Firewall rules. { $name } is the rule's name; { $error } is what went wrong.

core-rollback-firewall-not-read = ファイアウォール規則 { $name } を読み取れませんでした：{ $error }。
core-rollback-firewall-says-otherwise = ファイアウォール規則 { $name } は、その名前で存在する規則の内容が異なるため残ります。
core-rollback-firewall-changed = ファイアウォール規則 { $name } は、インストール後に変更されているため残ります。
# The capital "The" is as the app has always said it.
core-rollback-firewall-removed = ファイアウォール規則 { $name } を削除しました
core-rollback-firewall-not-removed = ファイアウォール規則 { $name } を削除できませんでした：{ $error }。

## Scheduled tasks. { $path } is a folder of tasks, and { $name } a task with its folder, such as \App\Nightly Update.
## { $error } and { $reason } are what went wrong.

core-rollback-task-folder-holds-others = タスク フォルダー { $path } には、このインストールで配置していないタスクが含まれているため、残ります。
core-rollback-task-folder-not-removed = タスク フォルダー { $path } を削除できませんでした：{ $error }。
core-rollback-task-not-read = タスク { $name } を読み取れませんでした：{ $error }。
core-rollback-task-changed = タスク { $name } は、インストール後に変更されているため残ります。
core-rollback-task-removed = タスク { $name } を削除しました
core-rollback-task-not-removed = タスク { $name } を削除できませんでした：{ $reason }。
core-rollback-task-put-back = タスク { $name } を元に戻しました
core-rollback-task-not-put-back = タスク { $name } を元に戻せませんでした：{ $reason }。

## When an install cannot be undone at all

# { $error } is what went wrong.
core-rollback-record-unreadable = このインストールの記録を読み取れなかったため、何も元に戻していません：{ $error }。
# { $error } is what went wrong.
core-rollback-nothing-undone = 何も元に戻していません：{ $error }。
# One app's part of the message below. { $app_name } is the app's name; { $failures } is what could not be undone, a
# sentence or more; { $folder } is the folder that holds the install's record.
core-rollback-earlier-install-stuck = { $app_name }：{ $failures }どうしても元に戻せない場合は、管理者が { $folder } を削除して、このインストールを破棄できます。
# Why an install did not start. { $details } is one or more of the message above.
core-rollback-earlier-installs-not-undone = 完了しなかった以前のインストールを完全には元に戻せなかったため、何もインストールされませんでした。これらを使用しているものを閉じて、もう一度お試しください。{ $details }
core-rollback-install-busy = このインストールは、別のインストーラーまたはアンインストーラーによって変更中です

## The summary an uninstall writes on the command line, one line each, with what each says below it. { $count } is how
## many.

core-rollback-count-undone = 元に戻した項目：{ $count }
core-rollback-count-failed = 元に戻せなかった項目：{ $count }
# What was left on purpose: it has changed since the install, or it is the user's.
core-rollback-count-kept = そのまま残した項目：{ $count }
# What no uninstall can take back, such as a runtime that was installed, which other programs may use.
core-rollback-count-left-behind = 残った項目：{ $count }

## Which Windows a PC or drive has, as the app shows it beside the drive.

# { $name } is the Windows' name, such as Windows 7 Professional Service Pack 1.
core-system-windows-32-bit = { $name }、32 ビット
# { $name } is the Windows' name, such as Windows 11 Pro.
core-system-windows-64-bit = { $name }、64 ビット
# A drive's Windows, when it only says it is older than Vista.
core-system-windows-xp-or-older = Windows XP 以前

## Reading a connected drive, or another account's settings. Each is why it could not be done, and some start with a
## small letter, as they follow other words.

core-system-needs-administrator = 接続したドライブや別のアカウントの設定を読み取るには、管理者権限が必要です
core-system-no-program-data = この PC の ProgramData フォルダーが見つかりませんでした
# { $path } is a file, such as D:\Windows\System32\config\SOFTWARE.
core-system-not-a-hive-file = ハイブ ファイルではありません：{ $path }
# { $path } is a file; { $size } is its size in bytes.
core-system-hive-too-large = { $path } は { $size } バイトあり、レジストリ ハイブとしては大きすぎるか、このドライブのハイブの合計として想定される上限を超えています
# { $key } is a registry key, such as Software\Microsoft\Windows\CurrentVersion.
core-system-hive-no-mapping = オフライン ハイブに HKLM\{ $key } のマッピングがありません
# { $key } is where a drive's registry was to be loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said.
# "reg load" is a command's name.
core-system-hive-not-loaded = reg load { $key } に失敗しました：{ $error }
# { $key } is where a drive's registry was loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said. "reg
# unload" is a command's name.
core-system-hive-not-unloaded = reg unload { $key } に失敗しました：{ $error }
# { $drive } is a drive, such as D:\.
core-system-not-a-system-drive = { $drive } は Windows のシステム ドライブではありません（\Windows フォルダーがありません）。
# { $drive } is a drive, such as C:\. "This PC" is the name of a choice in the app.
core-system-own-system-drive = { $drive } はこの PC 自身のシステム ドライブです。この PC を読み取るには、「この PC」を選択してください。
# { $drive } is a drive, such as D:\.
core-system-registry-through-link = { $drive } は Windows のシステム ドライブではありません：レジストリへのパスがリンクを経由しています。

## Warnings a scan records about whose data it read.

# { $account } is an account's name or id, such as alice or S-1-5-21-...-1001.
core-warning-scan-account-not-on-record = アカウント { $account } はここに記録されていないため、ユーザーごとのデータや設定は含まれていません。
# { $account } is an account's name, such as CONTOSO\alice.
core-warning-scan-signed-in-account-no-profile = ここにサインインしているアカウント { $account } のプロファイルが記録されていないため、ユーザーごとのデータや設定は含まれていません。
# { $reason } is why Windows could not say.
core-warning-scan-signed-in-account-unknown = サインインしているユーザーを判別できなかったため（{ $reason }）、ユーザーごとのデータや設定は含まれていません。含めるアカウントを選択してください。
core-warning-scan-last-account-unknown = このドライブには最後にサインインしたユーザーが記録されておらず、複数のアカウントがあるため、ユーザーごとのデータや設定は含まれていません。含めるアカウントを選択してください。
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-settings-unreadable = { $account } の設定を読み取れなかったため、含まれていません：{ $error }
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-classes-unreadable = { $account } 用に登録されている COM クラスとファイルの種類を読み取れなかったため、含まれていません：{ $error }

## Paths a scan did not follow. { $count } is how many; { $paths } is some of them, separated by semicolons; { $more } is
## how many more there are, not shown.

core-warning-scan-drive-paths-not-followed =
    { $count ->
       *[other] このドライブに記録されているパスを読み取れなかったため、そこにあるものは含まれていません。いずれも、別のディスク上にあるか、完全なパスではないか、ドライブの外につながるリンクを経由しています：{ $paths }
    }
core-warning-scan-drive-paths-not-followed-more = このドライブに記録されているパスを読み取れなかったため、そこにあるものは含まれていません。いずれも、別のディスク上にあるか、完全なパスではないか、ドライブの外につながるリンクを経由しています：{ $paths }、ほか { $more } 件
core-warning-scan-network-paths-not-followed =
    { $count ->
       *[other] この PC に記録されているパスを読み取れなかったため、そこにあるものは含まれていません。いずれも、この PC のディスク上になく、確認するとそこに接続することになるか、別のものを開く名前が付いています：{ $paths }
    }
core-warning-scan-network-paths-not-followed-more = この PC に記録されているパスを読み取れなかったため、そこにあるものは含まれていません。いずれも、この PC のディスク上になく、確認するとそこに接続することになるか、別のものを開く名前が付いています：{ $paths }、ほか { $more } 件

## What a scan with none of an account's data leaves out of a package, in the app's list of what was left out. Each
## has a name, and below it why.

core-account-left-out-shortcuts = このアカウント自身のフォルダー内のショートカット
# { $count } is how many shortcuts.
core-account-left-out-shortcuts-why =
    { $count ->
       *[other] { $count } 個のショートカット。「ユーザー データ」オプションを使用した場合にのみ含まれます
    }
core-account-left-out-variables = このアカウントの環境変数
# { $count } is how many variables.
core-account-left-out-variables-why =
    { $count ->
       *[other] { $count } 個の変数。「ユーザー データ」オプションを使用した場合にのみ含まれます
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-account-left-out-task = スケジュールされたタスク { $task }
core-account-task-works-on-account-folder = アカウントのフォルダー内の場所を対象に動作するか、その場所にちなんだ名前が付いているため、「ユーザー データ」オプションを使用した場合にのみ含まれます
core-account-task-runs-from-account-folder = アカウントのフォルダーから実行するものは、「ユーザー データ」オプションを使用した場合にのみ含まれます
# { $service } is a service's name, such as AppUpdater.
core-account-left-out-service = サービス { $service }
core-account-service-works-on-account-folder = アカウントのフォルダー内の場所を対象に動作するため、「ユーザー データ」オプションを使用した場合にのみ含まれます

## What a package was found to still say of whose PC it came from, after a build with none of an account's data.

# The parts of a package, as the warning below names them.
core-account-part-files = アプリ自身のファイルのパス
core-account-part-registry = レジストリ値
core-account-part-settings-entry = 「設定」にあるアプリのエントリ
core-account-part-main-program = メイン プログラムの場所
core-account-part-shortcuts = ショートカット
core-account-part-tasks = スケジュールされたタスク
core-account-part-services = サービス
core-account-part-variables = 環境変数
# One place in the list below. { $place } is one of the parts above, or a file's name; { $count } is how many times.
core-account-mention-place = { $place }（{ $count }）
# { $kind } is what was found: account (an account's name), folder (where an account's folder is), computer (the PC's
# name), or id (an account's id). { $places } is a list of where, from the message above.
core-warning-scan-package-mentions =
    { $kind ->
        [account] このパッケージの { $places } に、アカウントの名前が含まれています。アカウントのデータを含めずに作成されましたが、インストーラーを持っている人は誰でもこれを読み取ることができます。
        [folder] このパッケージの { $places } に、アカウントのフォルダーの場所が含まれています。アカウントのデータを含めずに作成されましたが、インストーラーを持っている人は誰でもこれを読み取ることができます。
        [computer] このパッケージの { $places } に、PC の名前が含まれています。アカウントのデータを含めずに作成されましたが、インストーラーを持っている人は誰でもこれを読み取ることができます。
       *[id] このパッケージの { $places } に、アカウントの ID が含まれています。アカウントのデータを含めずに作成されましたが、インストーラーを持っている人は誰でもこれを読み取ることができます。
    }
# { $count } is how many folders.
core-warning-scan-folders-of-others-unreadable =
    { $count ->
       *[other] アプリ外の { $count } 個のフォルダーを読み取れなかったため、そこにあるアプリのデータが欠けている可能性があります。
    }

## What a scan leaves out because it looks like a credential, or is a license with the License option off, in the app's
## list of what was left out. Each has a name, and below it why.

core-secret-why-credential = 資格情報のように見えます
core-secret-why-license-option-off = 「ライセンスとライセンス認証のデータ」オプションがオフです
# { $count } is how many values of a registry key.
core-secret-why-account-values =
    { $count ->
       *[other] { $count } 個の値。「ユーザー データ」オプションを使用した場合にのみ含まれます
    }
# Shown in place of a registry value's name when its name itself looks like a password or a key.
core-secret-name-holds-credential = （名前に資格情報を含む値）
# Shown in place of an option's name, for an argument on a command line that has no name.
core-secret-unnamed-argument = 資格情報を含む引数
# { $option } is an option taken out of a command line, such as /password, or the message above. { $place } says
# where the command line was: service, task, shortcut, opener (what opens a file type), opener-unnamed (the same, of a
# type with no name), or uninstall. { $name } is the service, task, shortcut, file type, or app.
core-secret-taken-from =
    { $place ->
        [service] { $name } の実行方法に含まれる { $option }
        [task] タスク { $name } の実行方法に含まれる { $option }
        [shortcut] ショートカット { $name } に含まれる { $option }
        [opener] { $name } を開くものに含まれる { $option }
        [opener-unnamed] ファイルの種類を開くものに含まれる { $option }
       *[uninstall] { $name } のアンインストール方法に含まれる { $option }
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-secret-task-definition-unreadable = タスク { $task } の定義（確認のために読み取ることができませんでした）
# { $name } is an environment variable's name, such as APP_TOKEN.
core-secret-left-out-variable = 環境変数 { $name }
# { $class } is a COM class's id, such as {00000000-0000-0000-0000-000000000000}.
core-secret-left-out-class-server = クラス { $class } が実行するとしているもの
# { $rule } is a firewall rule's name.
core-secret-left-out-firewall-description = ファイアウォール規則 { $rule } の説明

## Licenses a scan found, as the package's report describes each.

# { $name } is a file's name, such as license.lic.
core-license-detail-file = ライセンス ファイルが検出されました：{ $name }
# { $framework } is a licensing product's name, such as FlexLM / FlexNet.
core-license-detail-store = { $framework } のライセンス ストアがあります
# { $name } is an environment variable's name, such as LM_LICENSE_FILE.
core-license-detail-variable = ライセンスの環境変数 { $name }
# { $name } is a registry value's name, such as SerialNumber.
core-license-detail-registry-value = ライセンス データと思われるレジストリ値：{ $name }
# Said of a registry value in an account's key, which is not named.
core-license-detail-registry-value-of-an-account = ライセンス データと思われるレジストリ値
# { $key } is a registry key, such as HKLM\Software\Contoso\License.
core-license-detail-registry-key = ライセンス データと思われるレジストリ キー：{ $key }

## Warnings a scan records about licenses.

core-warning-scan-license-data = ライセンス/ライセンス認証のデータが検出されました。「ライセンスとライセンス認証のデータ」オプションをオンにすると、インストーラーは移行可能なものを復元します。PC 全体の値やハードウェアに関連付けられたストアは報告のみ行います。WinAppMigrator が暗号化を解除したり、ライセンス認証を回避したり、ドングルをエミュレートしたりすることはありません。
core-warning-scan-license-machine-bound = 一部のライセンスは PC に関連付けられています（ハードウェア ドングル、PC 固定のライセンス認証、または暗号化）。インストーラーは可能なものを復元しますが、新しい PC で再度ライセンス認証が必要になる場合があります。

## Warnings a scan records about the registry. What could not be read is not named in them: it is kept beside them.

# { $error } is what Windows said went wrong. App Paths is a registry key's name.
core-warning-scan-app-paths-key-unreadable = App Paths の子キーを読み取れませんでした：{ $error }
core-warning-scan-registry-limit = レジストリの取り込みが、設定された安全上の上限に達しました。
core-warning-scan-registry-subtree-limit = レジストリのサブツリーの取り込みが、設定された安全上の上限に達しました。
# { $error } is what Windows said went wrong.
core-warning-scan-registry-key-unreadable = レジストリ キーを読み取れませんでした：{ $error }
core-warning-scan-registry-value-too-large = レジストリ値が、設定された取り込みサイズの上限を超えました。

## Scheduled tasks

core-warning-scan-tasks-folder-through-link = このドライブのスケジュールされたタスクのフォルダーはリンクを経由しているため、タスクは読み取られていません。
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-warning-scan-task-left-out = スケジュールされたタスク { $task } は除外されました：設定を含める対象とは別のアカウントとして実行されるか、管理者以外がそのファイルを変更できるか、アプリのタスクが多すぎて保持できません。
# { $error } is what Windows said went wrong.
core-warning-scan-tasks-folder-unreadable = スケジュールされたタスクの一部を読み取れませんでした：{ $error }
# { $path } is a task's file, such as C:\Windows\System32\Tasks\App\Nightly Sync; { $reason } is one of the reasons
# below, or what Windows said went wrong.
core-warning-scan-task-unreadable = スケジュールされたタスク { $path } を読み取れませんでした：{ $reason }。アプリを実行するタスクだった場合、そのタスクは含まれていません。
# Why a task's file could not be read. { $size } is its size in bytes.
core-task-too-large = { $size } バイトあり、タスクとしては大きすぎます
# Why a task's file could not be read.
core-task-ends-mid-character = 文字の途中で終わっています
# Why a task's file could not be read. { $error } says where.
core-task-not-utf16 = 有効な UTF-16 ではありません：{ $error }
# Why a task's file could not be read. { $error } says where.
core-task-not-text = 有効な UTF-8 または UTF-16 ではありません：{ $error }
# What a package records of how a task is put back. Register-ScheduledTask is a command's name.
core-task-restore-instructions = 復元では、アーカイブされた XML 定義を使用して Register-ScheduledTask を実行します。

## Reading a program's file. Each is why it could not be read, after the program's name.

# { $reason } is what went wrong.
core-pe-unreadable = 読み取れませんでした：{ $reason }
core-pe-headers-misaligned = ヘッダーが、プログラムで必要な境界に配置されていません
# { $size } is the file's size in bytes.
core-pe-too-large = { $size } バイトあり、プログラムとしては大きすぎます
# PE is the name of the format Windows programs are in.
core-pe-no-headers = PE ヘッダーがありません

## Warnings a scan records about what a program needs. Which program is kept beside each.

# { $error } is what went wrong.
core-warning-scan-imports-unreadable = インポート テーブルを読み取れませんでした：{ $error }
# { $error } is what went wrong.
core-warning-scan-import-name-unreadable = インポート記述子の名前を読み取れませんでした：{ $error }
# { $count } is how many were skipped.
core-warning-scan-legacy-delay-imports =
    { $count ->
       *[other] { $count } 個のレガシ（v1）遅延読み込み記述子をスキップしました。解析されるのは、RVA ベースの新しい形式の遅延インポートだけです。
    }

## The DLLs a program needs. Which DLL or program is kept beside each warning.

# { $path } is a program, such as C:\Program Files\App\app.exe.
core-dll-program-has-no-folder = 実行可能ファイルに親ディレクトリがありません：{ $path }
# Why a DLL is among the app's files, in the package's list of them.
core-dll-beside-program = 選択した実行可能ファイルと同じ場所にある DLL
# { $error } is what went wrong.
core-warning-scan-dependency-unscanned = 再帰的な依存関係をスキャンできませんでした：{ $error }
# { $name } is a DLL's name, such as d3dx9_43.dll.
core-warning-scan-dependency-not-in-newer-windows = { $name } はこのアプリがあった PC の Windows フォルダーにありましたが、Windows 10 および 11 には含まれていません。何かがそこにインストールしたもので、アプリ自身のファイルではないため、含まれません。それを提供するものがインストールされるまで、アプリが起動しない可能性があります。
# { $error } is what went wrong.
core-warning-scan-dependency-not-hashed = 依存関係のハッシュを計算できませんでした：{ $error }
# SystemRoot and windir are the names of environment variables.
core-warning-scan-windows-folder-unknown = SystemRoot/windir 環境変数を使用できないため、システム DLL の検索が制限されます。
# { $error } is what went wrong.
core-warning-scan-dependency-architecture-unreadable = 依存関係のアーキテクチャを読み取れませんでした：{ $error }

## Services, drivers, shortcuts, and environment variables a scan reads. Where it was reading is kept beside each
## warning.

# { $error } is what Windows said went wrong.
core-warning-scan-services-unreadable = サービスのレジストリ キーを読み取れませんでした：{ $error }
# { $error } is what Windows said went wrong.
core-warning-scan-drivers-unreadable = ドライバーを検出するためにサービスのレジストリ キーを読み取ることができませんでした：{ $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-queried = ショートカットを照会できませんでした：{ $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-parsed = ショートカットの照会結果を解析できませんでした：{ $error }
# { $error } is what Windows said went wrong.
core-warning-scan-environment-unreadable = システム環境変数のレジストリ キーを読み取れませんでした：{ $error }

## What a package records of how a driver is put back, which an installer shows. { $kind } is the driver's type as
## Windows names it (kernel, file-system, recognizer, or unknown-type); { $name } is its service's name; { $path } is
## its file. { $signed } is yes when { $signer } names who signed it, and no when that is not known.

core-service-driver-bundled =
    { $signed ->
        [yes] アプリケーションが所有する { $kind } ドライバー「{ $name }」（{ $path }、署名者：{ $signer }）。インストーラーは .sys を同梱し、ドライバー サービスを自動的に再登録してから（sc create、管理者権限が必要）、サービスが登録されたことを確認します。
       *[no] アプリケーションが所有する { $kind } ドライバー「{ $name }」（{ $path }、署名は不明）。インストーラーは .sys を同梱し、ドライバー サービスを自動的に再登録してから（sc create、管理者権限が必要）、サービスが登録されたことを確認します。
    }
core-service-driver-from-vendor =
    { $signed ->
        [yes] ベンダー製または Windows 標準の { $kind } ドライバー「{ $name }」（{ $path }、署名者：{ $signer }）。WinAppMigrator はサードパーティのドライバーを再配布しません。アプリを起動する前に、対象の PC でベンダーの署名済みドライバー パッケージを使用して再インストールしてください。
       *[no] ベンダー製または Windows 標準の { $kind } ドライバー「{ $name }」（{ $path }、署名は不明）。WinAppMigrator はサードパーティのドライバーを再配布しません。アプリを起動する前に、対象の PC でベンダーの署名済みドライバー パッケージを使用して再インストールしてください。
    }

## Runtimes an app needs, such as the Visual C++ runtime or .NET: how each is provided, as the Dependencies list says.

core-runtime-handling-built-in = Windows に組み込み
core-runtime-handling-ships-with-app = アプリに付属
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-handling-windows-feature = Windows の機能：{ $feature }
core-runtime-handling-included = 同梱
# { $id } is the runtime's name in winget, such as Microsoft.VCRedist.2015+.x64.
core-runtime-handling-winget = winget でインストール：{ $id }
core-runtime-handling-manual = 手動でインストール

## Downloading a runtime's installer, when an installer is built. Each is why it could not be, and starts with a small
## letter, as it follows other words. { $url } is a web address; { $error } is what went wrong.

core-runtime-download-not-https = { $url } のダウンロードを拒否しました：HTTPS ではありません
# TLS is what keeps a download private.
core-runtime-tls-failed = TLS の初期化に失敗しました：{ $error }
core-runtime-download-failed = { $url } のダウンロードに失敗しました：{ $error }
core-runtime-download-unreadable = { $url } の読み取りに失敗しました：{ $error }
core-runtime-download-empty = { $url } のダウンロードでデータが返されませんでした
# { $came } and { $expected } are numbers of bytes.
core-runtime-download-cut-short = { $url } のダウンロードが途中で終わりました：{ $expected } バイト中 { $came } バイトを受信

## Installing a runtime: why it was not installed. Each starts with a small letter, as it follows other words.

core-runtime-no-known-installer = このランタイムの既知のインストーラーはありません
# { $path } is where in the package, such as prerequisites\vc_redist.x64.exe.
core-runtime-payload-misplaced = パッケージでは { $path } がこのランタイムのインストーラーとして指定されていますが、本来の場所ではないため、実行しませんでした
# { $path } is a file.
core-runtime-payload-missing = 同梱のインストーラーがありません：{ $path }
# { $error } is what went wrong.
core-runtime-payload-path-invalid = 同梱のインストーラーのパスが無効です：{ $error }
# { $url } is the web address it can be downloaded from. winget is a program's name.
core-runtime-no-installer-route = 同梱のインストーラーも winget の ID もありません。{ $url } から入手してください
# { $error } is what went wrong.
core-runtime-no-temp-folder = 展開用のフォルダーを作成できませんでした：{ $error }
# { $reason } is what the check found.
core-runtime-payload-signature-failed = 同梱のインストーラーは署名チェックに失敗したため、実行しませんでした：{ $reason }
core-runtime-payload-timed-out = 同梱のインストーラーがタイムアウトまでに完了しなかったため、終了させました
# { $error } is what went wrong.
core-runtime-payload-not-started = 同梱のインストーラーを開始できませんでした：{ $error }
core-runtime-installer-killed = インストーラーは終了コードなしで終了しました
# { $code } is the number it ended with, such as 1603.
core-runtime-installer-failed = インストーラーはコード { $code } で終了しました
# { $feature } is a Windows feature's name, such as NetFx3; { $code } is the number DISM ended with. Windows Features is
# the name of a Windows window. Keep the command as it is.
core-runtime-feature-failed = DISM で Windows の機能 { $feature } を有効にできませんでした（終了コード { $code }）。ペイロードをダウンロードできなかった可能性があります。「Windows の機能」で有効にするか、`dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess` を実行してください。
# The same, when DISM gave no number.
core-runtime-feature-failed-no-code = DISM で Windows の機能 { $feature } を有効にできませんでした（終了コード不明）。ペイロードをダウンロードできなかった可能性があります。「Windows の機能」で有効にするか、`dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess` を実行してください。
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-feature-timed-out = Windows の機能 { $feature } の有効化がタイムアウトまでに完了しなかったため、終了させました
# { $feature } is a Windows feature's name; { $error } is what went wrong.
core-runtime-feature-no-dism = Windows の機能 { $feature } を有効にするための DISM を使用できません：{ $error }

## Installing a runtime with winget. { $id } is its name in winget, such as Microsoft.VCRedist.2015+.x64.

# { $error } is what went wrong.
core-runtime-winget-not-looked-for = winget を検索できませんでした：{ $error }
core-runtime-winget-not-installed = winget がインストールされていません
# { $path } is where winget is.
core-runtime-winget-misplaced = { $path } は Windows が自身のパッケージを保存する場所ではないため、実行しませんでした
# { $path } is where winget is; { $reason } is what the check found.
core-runtime-winget-signature-failed = { $path } は署名チェックに失敗したため、実行しませんでした：{ $reason }
# { $reason } is one of the two messages above, or what went wrong.
core-runtime-winget-unusable = winget を使用して { $id } をインストールできませんでした：{ $reason }
# { $code } is the number winget ended with.
core-runtime-winget-failed = winget install { $id } はコード { $code } で終了しました
core-runtime-winget-failed-no-code = winget install { $id } は不明なコードで終了しました
core-runtime-winget-timed-out = winget install { $id } がタイムアウトまでに完了しなかったため、終了させました
# { $error } is what went wrong.
core-runtime-winget-not-started = { $id } をインストールするための winget を使用できません：{ $error }

## What told a scan how an app was installed, as the package's report lists it. The names of installers and files are
## as their makers write them.

# { $path } is an installer's record of what it installed, such as C:\Program Files\App\unins000.dat.
core-source-record-not-believed = { $path } は使用しませんでした：管理者以外が変更できます
# { $count } is how many Windows Installer packages the bundle holds.
core-source-burn-bundle =
    { $count ->
       *[other] { $count } 個の Windows Installer パッケージを含む Burn バンドル
    }
# { $count } is how many paths Windows Installer recorded.
core-source-msi-database =
    { $count ->
       *[other] Windows Installer データベース：{ $count } 個のコンポーネント キー パス
    }
core-source-inno-log = Inno Setup のアンインストール ログ（unins000.dat）
# { $count } is how many paths the log records.
core-source-inno-log-paths =
    { $count ->
       *[other] Inno Setup のアンインストール ログ：記録されたパス { $count } 個
    }
core-source-nsis = NSIS（Nullsoft）アンインストーラーの署名
# { $count } is how many paths the log records.
core-source-nsis-log-paths =
    { $count ->
       *[other] NSIS AdvUninstLog uninstall.log：記録されたパス { $count } 個
    }
core-source-squirrel = Squirrel のユーザーごとのレイアウト（Update.exe / app-*）
# { $version } is a folder's name that says the version, such as app-1.2.3.
core-source-squirrel-version = Squirrel の現在のバージョン：{ $version }
core-source-chocolatey = Chocolatey パッケージのメタデータ
core-source-msix = MSIX / Appx パッケージ
# { $identity } is a package's name, such as Contoso.App.
core-source-appx-identity = AppxManifest の ID：{ $identity }
# { $count } is how many folders.
core-source-msix-data =
    { $count ->
       *[other] MSIX のユーザーごとのパッケージ データ：{ $count } か所
    }
# App Paths is a registry key's name.
core-source-app-paths = App Paths の登録

## Why a runtime is needed, as the package's report says it.

# { $program } is a program's file name, such as app.exe; { $dll } is a DLL's, such as msvcp140.dll.
core-runtime-program-imports = { $program } は { $dll } をインポートしています

## What else of an app's a scan finds in the registry: fonts and certificates, and why each is the app's.

# Why a font is among the app's files, in the package's list of them.
core-registry-font-recorded = アプリのインストールの記録に記載されているフォント
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-named-for-the-app = アプリまたは発行元の名前が付いています
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-recorded = アプリのインストールの記録に記載されています

## Who is signed in, for the Whose Data choice. Each of the first six is why it could not be told, and starts with a
## small letter, as it is shown inside other words. An account is shown by its id, such as S-1-5-21-...-1001.

# { $error } is what Windows said.
core-profile-desktop-owner-not-said = Windows からこのデスクトップの所有者が示されませんでした：{ $error }
# { $error } is what went wrong.
core-profile-desktop-owner-not-asked = このデスクトップの所有者を Windows に問い合わせることができませんでした：{ $error }
core-profile-signed-in-account-no-profile = ここにサインインしているアカウント { $account } のプロファイルが記録されていません
core-profile-desktop-not-a-person = ここのデスクトップは { $account } のものですが、これは個人のアカウントではありません
core-profile-nobody-signed-in-running-account-no-profile = ここには誰もサインインしておらず、これを実行しているアカウント { $account } のプロファイルが記録されていません
core-profile-nobody-signed-in-running-not-a-person = ここには誰もサインインしておらず、これを実行している { $account } は個人のアカウントではありません
# SID is the name Windows gives an account's id.
core-profile-nobody-signed-in-no-sid = ここには誰もサインインしておらず、このアカウントには SID がありません
# { $accounts } is a list of accounts.
core-profile-several-desktops = このセッションでは、複数のアカウントがデスクトップを持っています：{ $accounts }
# Under the Whose Data choice. { $reason } is one of the messages above.
core-profile-signed-in-unknown = サインインしているユーザーを判別できませんでした（{ $reason }）。
# Under the Whose Data choice, when there are accounts to choose from. { $reason } is one of the messages above.
core-profile-signed-in-unknown-choose = サインインしているユーザーを判別できませんでした（{ $reason }）。データを含めるアカウントを選択してください。
