# Words of the installer that every built installer carries, which runs on the PC an app is moved to and speaks its
# language, and of the uninstaller it leaves behind for Settings. Message names start with installer- for the
# installer's window, installer-cli- for its command line, and uninstaller- for the uninstaller.
#
# The command lines (installer-cli-, uninstaller-cli-) are always shown in English, as scripts read what they print.
# Translate them all the same. Keep their command-line flags, such as --apply and --target-root, exactly as they are.


## The installer's window: its title and header

installer-window-title = WinAppMigrator インストーラー
# The window's title in the taskbar. { $app_name } is the name of the app it installs.
installer-window-title-app = { $app_name } のインストール
installer-tagline = 別の PC で取り込んだアプリケーションを再インストールします
# What an app is called when its package gives it no name.
installer-unnamed-app = アプリケーション

## When the installer's window cannot open. Each is a paragraph of a message box.

# { $error } is what went wrong, as Windows or the graphics driver says it.
installer-window-failed = インストーラーのウィンドウを開けませんでした：{ $error }
installer-window-failed-without-one = ウィンドウがなくてもインストールできます。管理者として実行したコマンド プロンプトから、--apply を付けてこのファイルを実行してください。インストール先を選ぶには、--target-root の後にフォルダーを指定します。

## Checking the installer when it opens

installer-checking = インストーラーを確認しています…
installer-stub-no-package = これは WinAppMigrator のインストーラー スタブで、アプリケーション パッケージは含まれていません。WinAppMigrator でインストーラーを作成し、作成された .exe をここで実行してください。
installer-check-stopped = インストーラーの確認が停止しました。
# { $error } is what went wrong, as Windows says it.
installer-locate-failed = インストーラーの場所を特定できませんでした：{ $error }
# { $error } is what went wrong reading it.
installer-read-failed = インストーラーを読み取れませんでした：{ $error }
# { $error } is why, such as that the app needs a newer Windows.
installer-cannot-be-used-here = このインストーラーのパッケージは、この PC では使用できません：{ $error }

## The app, where it goes, and whether the installer is signed

installer-captured-from = 取り込み元
installer-install-location = インストール先
# The title of the folder picker.
installer-choose-location = インストール先の選択
installer-folder-has-files = このフォルダーには既にファイルがあります。同じ名前のファイルは置き換えられ、アンインストールすると元に戻ります。
installer-room-needed = 必要な空き領域
# { $shortfall } says which drive is short of room, such as "C: needs up to 824.0 MB for the app's files, and has 300.0 MB free".
installer-room-short = ここには十分な空き領域がない可能性があります：{ $shortfall }。展開は別のドライブで行えますが、アプリのファイルはここに収まる必要があります。
# { $app_name } is the name of an app whose earlier install was cut short.
installer-interrupted = 以前の { $app_name } のインストールが完了していません。「インストール」を実行すると、最初にそれを元に戻します。
# Beside "Installer", for whether it proves who made it.
installer-signature = インストーラー
installer-signature-checking = 署名を確認しています…
installer-signature-unsigned = 署名なし：発行元は不明です
# { $publisher } is the name on the installer's certificate, such as "Fabrikam, Inc.".
installer-signature-signed-by = 署名者：{ $publisher }
installer-signature-unnamed = 名前のない発行元による署名
# { $reason } is Windows' own word for what is wrong, such as HashMismatch.
installer-signature-invalid = このインストーラーの署名は無効です（{ $reason }）。署名後に変更された可能性があります。

## Why the install location cannot be used. Install and Preview Changes stay off while one of these is shown.

# { $app_name } is the name of the app being installed.
installer-location-app-already-there = { $app_name } はこのフォルダーに既にインストールされているため、このコピーをここにインストールすることはできません。ここにインストールするには、まず Windows の「設定」の「アプリ」で、この PC 上のコピーをアンインストールしてから、このインストーラーをもう一度開いてください。または、上で空の新しいフォルダーを入力してください。
# { $other_app } is the name of another app Windows lists as installed in the folder. { $app_name } is the name of the app being installed.
installer-location-other-app-there = このフォルダーには { $other_app } がインストールされているため、{ $app_name } をここにインストールすることはできません。上で空の新しいフォルダーを入力してください。または、{ $other_app } が不要になった場合は、まず Windows の「設定」の「アプリ」でアンインストールしてから、このインストーラーをもう一度開いてください。
installer-location-has-files = このフォルダーには、WinAppMigrator でインストールされたものではないファイルが既にあります。別のアプリのファイルである可能性があるため、その上には何もインストールしません。上で空の新しいフォルダーを入力してください。
# { $reason } is why, such as that it cannot be the Windows folder.
installer-location-refused = このフォルダーは使用できません：{ $reason }。上で別のフォルダーを入力してください。

## What the install sets up beyond its files

installer-beyond-files = ファイル以外に設定される内容
# Under the heading above. It is part of what a person agrees to when they click Install.
installer-beyond-files-agreed = これらはすべて管理者権限で行われます。インストールに失敗した場合は変更が元に戻され、元に戻せなかったものがあればお知らせします。インストールは後から「設定」の「インストールされているアプリ」で元に戻すこともできます。その場合、置き換えられたファイルは元に戻り、インストールされたランタイムはそのまま残ります。
installer-beyond-files-none = なし：ファイルと、アプリ自身のレジストリ キー内の設定をコピーするだけです。
# { $problem } is why, such as that the app cannot run on this PC.
installer-cannot-go-here = このパッケージは、このインストール先には配置できません：{ $problem }
installer-working-out = このインストール先で設定される内容を確認しています…
# Said as the reason after "This package cannot go to this location:".
installer-list-stopped = 実行内容の確認が停止しました
# A heading over a list of what else the package asked for and will not get. { $count } is how many.
installer-more-left-out =
    { $count ->
       *[other] ほかに { $count } 件が除外されました
    }

## Options

installer-options = オプション
installer-user-data = ユーザー データ
installer-user-data-detail = オン：設定、ドキュメント、保存したゲームは、サインインしているユーザーの通常のフォルダーに配置されます。オフ：すべてインストール フォルダーの下に配置され、そこから実行されるものは何も設定されません。
# { $account } is the name of the account on the other PC whose data this is.
installer-user-data-whose = これには { $account } のデータが含まれています。
installer-license = ライセンス
installer-license-detail = ライセンス認証とライセンスのファイル、およびライセンスを保持するレジストリ値。この PC で再度ライセンス認証が必要になるものもあります。

## Dependencies: what else the app needs, and how each is provided

installer-dependencies = 依存関係
installer-dependencies-detail = このアプリに必要なものと、それぞれの提供方法です。
installer-dependencies-none = 追加のランタイム依存関係は検出されませんでした。
# { $feature } is the name of an optional part of Windows, such as NetFx3.
installer-dependency-windows-feature = Windows の機能：{ $feature }
installer-dependency-included = 同梱
# { $id } is the package's name in winget, such as Microsoft.VCRedist.2015+.x64.
installer-dependency-winget = winget でインストール：{ $id }
installer-dependency-manual = 手動でインストール
installer-dependency-built-in = Windows に組み込み
installer-dependency-ships-with-app = アプリに付属

## Buttons, and what is said beside them

installer-install = インストール
installer-preview = 変更のプレビュー
installer-cancel = キャンセル
installer-close = 閉じる
installer-open-folder = インストール フォルダーを開く
# { $error } is what went wrong, as Windows says it.
installer-open-folder-failed = フォルダーを開けませんでした：{ $error }
installer-location-blocks = インストール先が使用できるようになるまで、「インストール」と「変更のプレビュー」は使用できません。理由は、上のインストール先の下に表示されています。
installer-package-blocks = このパッケージはこのインストール先に配置できないため、「インストール」は使用できません。理由は上に表示されています。
installer-list-changed = ここで実行される内容が変わったため、何もインストールされませんでした。下の一覧を確認し、問題がなければもう一度「インストール」をクリックしてください。
installer-cancelled-early = キャンセルしました。何も変更されていません。

## How far an install has got, beside its progress bar

# { $step } is one of the steps below. { $done } and { $total } are how many of its items are done, and how many there are.
installer-step-counted = { $step }：{ $total } 件中 { $done } 件…
# { $step } is one of the steps below, when it does not count its items.
installer-step-uncounted = { $step }…
installer-step-getting-ready = 準備しています…
installer-step-cancelling = 現在の手順が終わり次第キャンセルします…
installer-step-putting-back = 変更を元に戻しています
installer-step-unpacking = 展開しています
installer-step-checking-package = パッケージを確認しています
installer-step-recovering = 完了しなかった以前のインストールを元に戻しています
# What a preview is doing, whatever step it is on.
installer-step-previewing = 変更内容を確認しています
installer-step-runtimes = ランタイムをインストールしています
installer-step-files = ファイルをコピーしています
installer-step-services = サービスを設定しています
installer-step-drivers = ドライバーを設定しています
installer-step-registry = レジストリ設定を書き込んでいます
installer-step-license = ライセンス データを復元しています
installer-step-tasks = スケジュールされたタスクを設定しています
installer-step-shortcuts = ショートカットを作成しています
installer-step-environment = 環境変数を設定しています
installer-step-settings-entry = アプリを「設定」に追加しています
installer-step-verifying = インストールされた内容を確認しています
installer-step-undoing = 元に戻しています

## When an install cannot go ahead or stops

installer-carries-no-package = このインストーラーにはパッケージが含まれていません。
installer-stopped-unexpectedly = インストールが予期せず停止しました。
# { $reason } is what went wrong, as the program says it.
installer-stopped-unexpectedly-because = インストールが予期せず停止しました：{ $reason }。インストールが始まっていた場合は、インストーラーをもう一度実行すると、最初にその変更が元に戻されます。
# { $error } is what went wrong reading it.
installer-package-read-failed = パッケージを読み取れませんでした：{ $error }
installer-no-programdata = この PC の ProgramData フォルダーが見つからなかったため、何もインストールされませんでした。インストールを元に戻すための記録は、このフォルダーに保存されます。
# { $error } is why, such as that there is nowhere only administrators can get into.
installer-staging-failed = ステージング フォルダーを作成できませんでした：{ $error }
# { $error } is what went wrong unpacking it.
installer-unpack-failed = パッケージを展開できませんでした：{ $error }
installer-staging-nowhere = 管理者だけがアクセスできる、パッケージの展開場所がありません
# { $failure } is a folder and what went wrong there, such as "C:\ProgramData\WinAppMigrator: Access is denied.".
installer-staging-nowhere-look-at = 管理者だけがアクセスできる、パッケージの展開場所がありません。確認が必要な場所：{ $failure }

## What came of an install, or of a preview

installer-result = インストール結果
installer-result-preview = プレビュー：まだ何も変更されていません
installer-result-installed = アプリケーションがインストールされ、ハッシュ チェックに合格しました。
installer-result-installed-without-left-out = アプリケーションがインストールされ、ハッシュ チェックに合格しました。ただし、下に除外として表示されているものは含まれません。
installer-result-not-all-put-back = インストールに失敗し、変更の一部を元に戻せませんでした。残っているものは下に表示されています。次回のインストール時に再試行されます。
installer-result-cancelled-put-back = インストールはキャンセルされ、変更は元に戻されました。
installer-result-failed-put-back = インストールに失敗し、変更は元に戻されました。
installer-result-not-clean = インストールは正常に完了しませんでした。下の問題を確認してください。
installer-result-changes-put-back = 元に戻した変更
installer-result-changes-made = 行った変更
installer-result-changes-planned = 予定されている変更
installer-result-warnings = 警告
installer-result-errors = エラー
installer-result-package-valid = パッケージのハッシュ：有効
installer-result-package-invalid = パッケージのハッシュ：無効
installer-result-files-valid = インストールされたファイル：有効
installer-result-files-invalid = インストールされたファイル：無効
installer-result-reboot = 依存関係の処理を完了するには再起動が必要です。Windows を再起動してから、アプリを起動してください。
# A heading over the list of what was changed. { $count } is how many.
installer-result-changes = 変更（{ $count }）
# After the first part of a long list. { $count } is how many more there are.
installer-result-changes-more = ほか { $count } 件。
# Over the path of the install's log file.
installer-log = ログ

## The installer's command line. Shown in English: scripts read it. A { $count } is plain digits, with no separators.

installer-cli-stub = WinAppMigrator インストーラー スタブ — アプリケーション パッケージは含まれていません。
installer-cli-stub-how = WinAppMigrator アプリでインストーラーを作成し、作成された .exe を対象の PC で実行してください。
installer-cli-usage-title = WinAppMigrator 自己解凍形式インストーラー。
installer-cli-usage = 引数なしで実行するとグラフィカル インストーラーが開きます。または次のように実行します：
installer-cli-usage-preview = 再インストールのプレビュー： --dry-run [--target-root <DIR>]
installer-cli-usage-apply = 再インストール：             --apply [--target-root <DIR>]
installer-cli-no-package = インストーラーに埋め込みパッケージが含まれていません
installer-cli-no-programdata = この PC の ProgramData フォルダーが見つからなかったため、何もインストールされませんでした（インストールを元に戻すための記録は、このフォルダーに保存されます）
# { $path } is the folder asked for, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-folder-has-files = { $path } は使用できません：{ $problem }。このアプリのものだとわかっている場合にのみ --force を指定してください
installer-cli-no-location = このパッケージにはアプリの元の場所が記録されていないため、--target-root を指定してください
# { $path } is where the app was on the other PC, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-location-refused = このパッケージのアプリは { $path } にありましたが、その場所は使用できません：{ $problem }。--target-root を指定してください
# { $summary } is how much room it needs where, such as "up to 824.0 MB on C: (356.0 GB free)".
installer-cli-room = 空き領域：{ $summary }
# { $note } is something to know about the install.
installer-cli-note = 注意：{ $note }
# { $count } is how many things it sets up beyond its files.
installer-cli-sets-up = 設定する項目：{ $count }
installer-cli-applied = 再インストールを適用しました。
installer-cli-previewed = 再インストールをプレビューしました。
# { $count } is how many changes the install logged.
installer-cli-changes-logged = 記録された変更：{ $count }
# { $count } is how many warnings.
installer-cli-warnings = 警告：{ $count }
# { $count } is how many errors.
installer-cli-errors = エラー：{ $count }
# { $count } is how many things the package asked for and did not get.
installer-cli-refused = 拒否された項目：{ $count }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-package-valid = パッケージのハッシュが有効：{ $valid }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-reinstall-valid = 再インストールのハッシュが有効：{ $valid }
# { $id } is the install's id, such as wam-0f3a….
installer-cli-install-id = インストール ID：{ $id }
installer-cli-put-back = インストールに失敗したため、変更は元に戻されました。
# { $path } is the log file.
installer-cli-log = ログ：{ $path }
# { $path } is the log file. Printed before an error.
installer-cli-error-log = ログ：{ $path }
# { $error } is what went wrong.
installer-cli-error = エラー：{ $error }

## The uninstaller's window

uninstaller-window-title = アンインストール
# The window's title, and its heading. { $app_name } is the name of the app it uninstalls.
uninstaller-window-title-app = { $app_name } のアンインストール
uninstaller-tagline = インストーラーが行った変更を元に戻します
uninstaller-reading = このインストールの記録を読み取っています…
uninstaller-stopped-unexpectedly = アンインストールが予期せず停止しました。
uninstaller-what-will-be-removed = 削除される内容
uninstaller-install-location = インストール先
uninstaller-files = ファイル
uninstaller-registry-values = レジストリ値
uninstaller-services = サービス
uninstaller-drivers = ドライバー
uninstaller-scheduled-tasks = スケジュールされたタスク
uninstaller-environment = 環境変数
uninstaller-plan-stays = 追加または変更したファイルは、それを含むフォルダーと共に残ります。
# { $count } is how many files the install replaced.
uninstaller-plan-coming-back =
    { $count ->
       *[other] このインストールで置き換えられた { $count } 個のファイルは元に戻ります。ただし、その後変更されたファイルがある場合は、その横に配置されます。
    }
# { $runtimes } is a list of runtimes, such as "Microsoft Visual C++ Redistributable, .NET Desktop Runtime 8".
uninstaller-plan-runtimes = インストールされたランタイムは、ほかのプログラムが使用している可能性があるため残ります：{ $runtimes }。
uninstaller-options = オプション
uninstaller-remove-user-data = ユーザー データとライセンス データも削除する
# { $count } is how many items of the app's settings, data, and license there are.
uninstaller-remove-user-data-detail =
    { $count ->
       *[other] 設定、データ、ライセンスの項目が { $count } 個あります。これをオンにしない限り残ります。これらが置き換えたものは元に戻ります。ただし、その後変更されている場合は、その横に配置されます。
    }
uninstaller-uninstall = アンインストール
uninstaller-cancel = キャンセル
uninstaller-close = 閉じる
# { $done } and { $total } are how many of the install's items are undone, and how many there are.
uninstaller-step-counted = インストールを元に戻しています：{ $total } 件中 { $done } 件…
uninstaller-step = インストールを元に戻しています…
uninstaller-done = アプリはアンインストールされました。
uninstaller-done-except = 下に表示されているものを除き、アプリはアンインストールされました。
uninstaller-waits-for-drive = 一部が、接続されていないドライブ上にあります。ドライブを接続し、「設定」からもう一度アンインストールしてください。
uninstaller-not-all-undone = 一部を元に戻せませんでした。使用中のものを閉じて、「設定」からもう一度アンインストールしてください。
# Over the path of the uninstall's log file.
uninstaller-log = ログ
uninstaller-undone = 元に戻した項目
uninstaller-left-in-place = そのまま残した項目
uninstaller-could-not-be-undone = 元に戻せなかった項目
# After the first part of a long list. { $count } is how many more there are.
uninstaller-and-more = ほか { $count } 件。
# Said as the reason when there is no record of the install to read.
uninstaller-no-programdata = この PC の ProgramData フォルダーが見つからないため、読み取る記録がありません

## When the uninstaller's window cannot open. Each is a paragraph of a message box, followed by a command to type.

uninstaller-window-failed-title = WinAppMigrator アンインストーラー
# { $error } is what went wrong, as Windows or the graphics driver says it.
uninstaller-window-failed = アンインストーラーのウィンドウを開けませんでした：{ $error }
uninstaller-window-failed-quiet = ウィンドウがなくても、管理者として実行したコマンド プロンプトから次のコマンドでアンインストールできます：
# Stands in for this program's own path in that command, if it cannot be found.
uninstaller-this-program = このプログラム

## The uninstaller's command line (--quiet). Shown in English: scripts read it.

# { $app_name } is the name of the app.
uninstaller-cli-uninstalling = { $app_name } をアンインストールしています。
# { $path } is the log file.
uninstaller-cli-log = ログ：{ $path }
# { $error } is what went wrong.
uninstaller-cli-error = エラー：{ $error }
