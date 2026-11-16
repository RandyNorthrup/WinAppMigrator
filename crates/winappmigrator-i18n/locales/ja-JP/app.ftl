# Words of the app, WinAppMigrator itself: its one window, with the installed apps listed on the left and, on the
# right, what goes in an installer of the one picked and the button that builds it; and the steps to read another PC
# through WinAppMigrator Client. Message names start with app-.


## The window and its header

# The program's name, in the window's title bar and at the top of the window. Keep it as it is unless your language
# writes product names in a script of its own.
app-name = WinAppMigrator
app-tagline = インストール済みのアプリを自己解凍形式のインストーラーに変換します
app-theme-to-light = ライト テーマに切り替え
app-theme-to-dark = ダーク テーマに切り替え
app-reload-list = アプリケーションの一覧を再読み込み
app-list-loading = 読み込み中
# { $count } is how many apps are listed.
app-list-count =
    { $count ->
       *[other] { $count } 個のアプリ
    }
app-list-load-failed = 読み込みに失敗しました
# Shown when the window cannot open at all. { $error } is what went wrong, in the words of Windows or a graphics driver.
app-window-could-not-open = ウィンドウを開けませんでした：{ $error }

## The list of installed apps

app-list-search = 名前または発行元で検索（Ctrl+F）
app-list-filter-all = すべて
# Whether an app is installed for one account or for everyone on the PC.
app-scope-per-user = ユーザー別
app-scope-per-machine = PC 全体
app-scope-unknown = 不明
# An app whose processor architecture (x86, x64, ARM64) is not known.
app-arch-unknown = 不明
app-list-loading-apps = インストールされているアプリケーションを読み込んでいます…
# { $error } is why the apps could not be listed.
app-list-could-not-list = アプリケーションの一覧を取得できませんでした：{ $error }
app-list-no-matches = 一致するアプリケーションはありません
app-list-no-matches-hint = 検索文字列または範囲のフィルターを調整してください。
app-list-worker-disconnected = 一覧を取得するワーカーとの接続が切断されました。
# What a screen reader says of an app's row. { $name } is the app's name, { $publisher } who made it, and { $version } its
# version, such as 11.4.2, or — when either is not known.
app-list-row = { $name }、{ $publisher }、バージョン { $version }

## The list's columns, and what a screen reader says each column's heading does when clicked

app-column-name = 名前
app-column-name-sort = 名前で並べ替え
app-column-publisher = 発行元
app-column-publisher-sort = 発行元で並べ替え
app-column-version = バージョン
app-column-version-sort = バージョンで並べ替え
app-column-scope = 範囲
app-column-scope-sort = 範囲で並べ替え
# Short for architecture: x86, x64, or ARM64. The column is narrow.
app-column-arch = アーキ
app-column-arch-sort = アーキテクチャで並べ替え

## Where apps are read from: this PC, a drive from another PC, a disk image of one, or another PC on the network

app-source = スキャン元
app-source-this-pc = この PC
app-source-drive = 接続したドライブ…
app-source-image = ディスク イメージ…
app-source-network = ネットワーク PC…
app-source-attaching-image = イメージを読み取り専用でアタッチしています…
# { $computer } is the other PC's name, such as OLD-PC, and { $address } its network address, such as 192.168.1.20.
app-source-network-pc = ネットワーク経由の { $computer }（{ $address }）
app-source-disconnect = 切断
app-source-change = 変更…
# The title of the window that picks a disk image. .vhd and .vhdx are file extensions.
app-source-pick-image-title = 別の PC のディスク イメージを選択（.vhd または .vhdx）
app-source-pick-image-filter = ディスク イメージ
# { $path } is the disk image file.
app-source-image-has-no-windows = { $path } には Windows を含むボリュームがありません。
app-source-image-not-attached = ディスク イメージをアタッチできませんでした。
app-source-pick-drive-title = 接続したドライブのルートを選択（例：E:\）
# { $path } is the folder picked, such as E:\. \Windows and \Users are folder names, as Windows writes them.
app-source-not-a-windows-drive = { $path } は Windows のシステム ドライブではないようです（\Windows と \Users がありません）。別の PC のドライブのルートを選択してください。
# Said of a drive or disk image with a Windows older than Windows 10. { $windows } is its name, such as "Windows 7
# Professional Service Pack 1, 32-bit". { $is_32_bit } is "yes" when that Windows is 32-bit, which adds the last
# sentence, and "no" when it is not.
app-source-older-windows =
    これには { $windows } が入っています。ここから作成したインストーラーは Windows 10 および 11 用です。ユーザー データを含めると、Windows が古いアプリ用に別の場所に保存していた設定も移行され、DirectX 9 など Windows 11 にないランタイムがアプリと共にインストールされます。{ $is_32_bit ->
        [yes] ここにあるアプリは 32 ビットのため、64 ビット版 Windows で 32 ビット アプリが置かれる Program Files (x86) に配置されます。
       *[no] { "" }
    }
# The same, of a Windows whose name could not be read. { $is_32_bit } is as above.
app-source-older-windows-unnamed =
    これには古い Windows が入っています。ここから作成したインストーラーは Windows 10 および 11 用です。ユーザー データを含めると、Windows が古いアプリ用に別の場所に保存していた設定も移行され、DirectX 9 など Windows 11 にないランタイムがアプリと共にインストールされます。{ $is_32_bit ->
        [yes] ここにあるアプリは 32 ビットのため、64 ビット版 Windows で 32 ビット アプリが置かれる Program Files (x86) に配置されます。
       *[no] { "" }
    }

## The details panel, before an app is picked, and the app picked

app-details-pair-first = 古い PC とペアリング
app-details-pair-first-hint = 古い PC で WinAppMigrator Client を実行し、表示されたコードを入力すると、その PC のアプリがここに表示されます。この PC のアプリと同じように、そこからインストーラーを作成できます。
app-details-pick-an-app = アプリケーションの選択
app-details-pick-an-app-hint = 一覧からアプリを選び、含める内容を選択してインストーラーを作成します。
# { $publisher } is who made the app, and { $version } its version, such as 11.4.2.
app-details-publisher-and-version = { $publisher }、バージョン { $version }
# { $version } is the app's version, such as 11.4.2.
app-details-unknown-publisher-and-version = 不明な発行元、バージョン { $version }
app-details-unknown-publisher = 不明な発行元

## What goes in besides the program

app-option-license = ライセンスとライセンス認証のデータ
app-option-license-hint = この PC に関連付けられている場合があり、新しい PC で再度ライセンス認証が必要になることがあります。
app-option-user-data = ユーザー データ
# Documents, Saved Games, and ProgramData are folders of Windows: use the names Windows gives them in your language,
# except ProgramData, which keeps its name.
app-option-user-data-hint = 設定と履歴、ドキュメント、保存したゲーム、ProgramData。
app-option-whose-data = 対象のアカウント
app-option-whose-data-hint = インストーラーに含められるのは 1 つのアカウントのデータです。データは、インストーラーを実行する PC でサインインしているユーザーのアカウントに配置されます。

## Building the installer

app-output-folder = 出力フォルダー
# An example of a folder, shown in the empty field. "You" stands for the name of an account.
app-output-folder-example = C:\Users\ユーザー名\WinAppMigrator
app-output-folder-not-full = D:\Installers のように、フォルダーの完全なパスを入力してください。
app-build-create = インストーラーの作成
app-build-create-again = インストーラーの再作成
app-build-cancel = キャンセル
app-build-cancelling = 現在の手順が終わり次第キャンセルします…
app-build-cancelled = 作成をキャンセルしました。インストーラーは書き込まれていません。
# winappmigrator-installer.exe is a file name.
app-build-no-stub = インストーラー スタブ（winappmigrator-installer.exe）がこのアプリと同じ場所に見つからないため、自己解凍形式のインストーラーを作成できません。
app-build-select-an-app-first = 最初に、移行するアプリケーションを選択してください。
app-build-look-again-first = 作成する前に、このアプリをもう一度調べる必要があります。
# { $reason } is what went wrong, in the program's own words, often in English.
app-build-stopped-unexpectedly = 作成が予期せず停止しました：{ $reason }。出力フォルダーにインストーラーが残っている場合は、使用しないでください。
app-build-worker-disconnected = 移行処理を行うワーカーとの接続が切断されました。
# { $path } is the output folder, and { $error } what Windows said went wrong.
app-build-no-output-folder = 出力フォルダー { $path } を作成できませんでした：{ $error }
# { $error } is what went wrong.
app-build-no-staging-folder = 保護されたステージング フォルダーを作成できなかったため、何も作成されませんでした：{ $error }
# { $error } is what went wrong, as a sentence.
app-build-could-not-build = インストーラーを作成できませんでした。{ $error }
app-build-no-installer = 作成は完了しましたが、インストーラーはありません。
# { $error } is what went wrong.
app-build-hash-check-did-not-run = インストーラーは書き込まれましたが、ハッシュ チェックを実行できませんでした：{ $error }。このインストーラーは使用しないでください。

## How far a build has got

app-progress-preparing = 含める内容を確認しています…
app-progress-checking-room = 空き領域を確認しています…
app-progress-bundling = ランタイムとドライバーをまとめています…
# { $done } is how many files have been read so far, and { $total } how many there are.
app-progress-packing = ファイルを読み取って圧縮しています：{ $total } 個中 { $done } 個
# { $done } is how many of the installer's records have been looked over so far, and { $total } how many there are.
app-progress-looking-over = インストーラーの内容を確認しています：{ $total } 件中 { $done } 件…
app-progress-writing = インストーラーを書き込んでいます…
app-progress-verifying = ハッシュを検証しています…

## What a build made

app-result-failed-hash-check = このインストーラーは使用しないでください：ハッシュ チェックに失敗しました。
# { $count } is how many files were gone when the build came to read them.
app-result-files-gone =
    { $count ->
       *[other] インストーラーは作成されましたが、アプリを調べた後に { $count } 個のファイルがなくなったため、それらは含まれていません。下の「警告」を参照してください。
    }
# { $count } is how many files could not be read into the installer.
app-result-files-not-captured =
    { $count ->
       *[other] インストーラーは作成されましたが、{ $count } 個のファイルを取り込めませんでした。通常は、ファイルがロックされていたか、読み取れなかったことが原因です。これらがないと、アプリが動作しない可能性があります。下の「警告」を参照してください。
    }
app-result-mentions-who = インストーラーは作成されましたが、作成元に関する情報がまだ含まれています。誰かに渡す前に、下の内容を確認してください。
app-result-ready = インストーラーの準備ができました。パッケージはハッシュ チェックに合格しました。
app-result-files = ファイル
app-result-installer-size = インストーラーのサイズ
app-result-warnings = 警告
app-result-log = ログ
app-result-copy-error = エラーのコピー
app-result-installer = インストーラー
app-result-data-file = データ ファイル
app-result-open-folder = フォルダーを開く
app-result-copy-path = パスのコピー
# { $error } is what Windows said went wrong.
app-result-could-not-open-folder = フォルダーを開けませんでした：{ $error }
# { $installer } and { $data_file } are file names, such as App_installer.exe and App_installer.wamdata.
app-result-two-files = このアプリは 1 つの実行可能ファイルに収まらないほど大きいため、インストーラーは { $installer } と { $data_file } の 2 つのファイルで構成されます。この 2 つは一緒にしておく必要があります。両方をこの名前のまま、別の PC の同じフォルダーにコピーしてください。インストーラーに署名すると、その署名はデータ ファイルにも適用されます。
# { $app_name } is the app's name. "Unknown publisher" is what Windows shows when it asks to run an unsigned program: use
# Windows' own words for it in your language.
app-result-run-it = 別の PC でこれを実行すると、{ $app_name } が再インストールされます。ファイルは SHA-256 で確認されます。作成したインストーラーは署名されていないため、Windows には「不明な発行元」と表示されます。
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-verification-errors = 検証エラー（{ $count }）
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-warnings-heading = 警告（{ $count }）
# After the first of a long list of warnings. { $count } is how many more there are.
app-warnings-and-more = ほか { $count } 件。

## Looking at an app, to see what an installer of it would hold

app-scan-getting-ready = このアプリを調べる準備をしています…
app-scan-install-record = インストール方法を読み取っています…
app-scan-near-matches = このアプリの一部である可能性があるものを探しています…
app-scan-system-items = サービス、タスク、ショートカットを探しています…
app-scan-files = ファイルを調べています…
app-scan-registry = Windows との連携を読み取っています…
app-scan-finishing = 最終処理をしています…
app-scan-look-again = もう一度調べる
app-scan-stop = 停止
app-scan-running-hint = 実行中に、別のアプリを選んだり、含める内容を変更したりできます。
# { $error } is why, in words that follow a colon.
app-scan-failed = このアプリを調べることができませんでした：{ $error }
app-scan-try-again = 再試行
app-scan-stopped = 停止しました。このアプリを調べ終えると、インストーラーを作成できます。
# { $reason } is what went wrong, in the program's own words, often in English.
app-scan-stopped-unexpectedly = 予期せず停止しました：{ $reason }。

## How big an installer can be, and whether there is room for it

app-room-files = ファイル
# Over how big the installer can be: up to that size.
app-room-up-to = 最大
app-room-free = 空き
# { $drive } is a drive, such as C:.
app-room-free-on = { $drive } の空き
# How much room a drive has, when it would not say.
app-room-free-unknown = 不明
app-room-most-it-could-need = 圧縮前の、必要になる可能性がある最大のサイズです。
# { $drive } is a drive, such as C:, { $needed } how much room it needs, such as 1.2 GB, and { $free } how much it has.
app-room-drive-short = { $drive } では、これを作成するのに最大 { $needed } の空き領域が必要ですが、空きは { $free } です。
# { $drives } is one sentence like the one above for each drive without the room, one after another.
app-room-not-enough = 空き領域が不足しています。{ $drives }別のドライブの出力フォルダーを選択するか、含める内容を減らしてください。
# { $note } is a sentence, without its full stop, about a drive whose room could not be checked. Use your language's
# full stop.
app-room-could-not-tell = { $note }。

## What goes in the installer, part by part

app-footprint-heading = インストーラーに含まれる内容
app-footprint-working-out = それに伴う内容を確認しています…
# { $error } is why.
app-footprint-would-not-make-a-package = 残りの内容ではパッケージを作成できません：{ $error }
# { $list } is the names of what else is left out, such as a service and a shortcut.
app-footprint-also-left-out = これにより { $list } も除外されます。これらが実行または参照するものが含まれなくなるためです。別の PC では設定されません。
# { $list } is the names of the first few of what else is left out, and { $count } how many more there are.
app-footprint-also-left-out-and-more = これにより { $list } と、ほか { $count } 件も除外されます。これらが実行または参照するものが含まれなくなるためです。別の PC では設定されません。
# { $list } is the names of license files. The option is the one named "License and Activation Data".
app-footprint-licenses-still-go = 除外した内容に含まれるライセンス データは、「ライセンスとライセンス認証のデータ」オプションに従うため、引き続き含まれます：{ $list }。
# { $elapsed } is how long ago, in minutes and seconds such as 45:00, or hours, minutes, and seconds such as 1:05:00.
# "Look Again" names the button of that name.
app-footprint-looked-at-long-ago = このアプリを調べたのは { $elapsed } 前です。ファイルは作成時に読み取られるため、変更は反映され、その時点でなくなっているものは一覧に表示されます。新しく追加されたものを含めるには、「もう一度調べる」を使用してください。
# Headings of the parts of the list, each with a line that says what is in it.
app-footprint-program-files = プログラム ファイル
app-footprint-data-folders = データ フォルダー
# What could go in with the User Data option, when nothing else is in this part. { $size } is how much, such as 1.2 GB.
app-footprint-folders-with-user-data = 「ユーザー データ」オプションを使用すると { $size }
# { $included } is how many folders go in, { $count } how many there are, and { $size } how much those that go in hold.
app-footprint-folders-summary =
    { $count ->
       *[other] { $count } 個中 { $included } 個のフォルダー、{ $size }
    }
# { $size } is how much, such as 1.2 GB, and { $count } in how many files.
app-footprint-data-only-with-user-data =
    { $count ->
       *[other] このアプリのデータのうち、{ $count } 個のファイル（{ $size }）は「ユーザー データ」オプションを使用した場合にのみ含まれます。
    }
# A folder's row. { $count } is how many files it holds, and { $detail } where it is or why it cannot be left out.
app-footprint-folder-row =
    { $count ->
       *[other] { $count } 個のファイル。{ $detail }
    }
app-footprint-services = サービス
app-footprint-startup = 自動起動
app-footprint-tasks = スケジュールされたタスク
app-footprint-shortcuts = ショートカット
# { $included } is how many of the part's things go in, and { $count } how many there are.
app-footprint-items-summary = { $count } 個中 { $included } 個
# { $why } is what else was left out that this goes with.
app-footprint-left-out-too = 併せて除外：{ $why }
# A row's detail. { $kind } is what it is, such as Service, and { $detail } what it does.
app-footprint-change-detail = { $kind }。{ $detail }
app-footprint-integrations = Windows との連携
# { $included } is how many kinds go in, such as file types and firewall rules, and { $count } how many kinds there are.
app-footprint-integrations-summary =
    { $count ->
       *[other] { $count } 種類中 { $included } 種類
    }
app-footprint-integration-nothing-left = 除外した内容により、何も残っていません
# { $count } is how many things of this kind the installer sets up.
app-footprint-integration-things =
    { $count ->
       *[other] { $count } 件を設定
    }
app-footprint-prerequisites = 実行に必要なもの
# { $count } is how many runtimes, such as Visual C++ Redistributable.
app-footprint-prerequisites-summary =
    { $count ->
       *[other] { $count } 個のランタイム
    }
app-footprint-near-matches = このアプリの一部である可能性があるもの
# { $ticked } is how many were ticked, and { $count } how many there are.
app-footprint-near-matches-summary = { $count } 個中 { $ticked } 個を追加
app-footprint-near-matches-hint = 選択しない限り、インストーラーには含まれません。選択すると、ほかと同じ確認を行ってアプリをもう一度調べるため、パスワードのように見えるものは引き続き除外されます。
# { $why } is why it might be part of the app, as a sentence.
app-footprint-near-match-with-user-data = 「ユーザー データ」オプションを使用すると含まれます。{ $why }
# How much a folder holds, when not all of it could be measured. { $size } is how much was, such as 1.2 GB.
app-footprint-near-match-over = { $size } 以上
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-folder = フォルダー：{ $place }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-hidden = アプリの非表示の部分：{ $place }
app-footprint-nothing-left-out = パスワード、ライセンス、証明書のように見えるために除外されたものはありません。
app-footprint-left-out = 除外したものとその理由
# { $count } is how many things were left out.
app-footprint-left-out-summary =
    { $count ->
       *[other] { $count } 件
    }
# Above a list of things left out for one reason. { $title } is the reason.
app-footprint-left-out-group = { $title }：
app-footprint-refused = 次の項目は、どのインストーラーでも設定が許可されません：
# A heading over a list that opens and closes. { $count } is how many are in it.
app-footprint-worth-knowing = 参考情報（{ $count }）
# { $list } is what kinds of things were looked for and none found, from the five below.
app-none-found = 見つかりませんでした：{ $list }。
app-none-found-services = サービス
app-none-found-startup = スタートアップ項目
app-none-found-tasks = スケジュールされたタスク
app-none-found-shortcuts = ショートカット
app-none-found-integrations = Windows との連携

## Network PC: reading another PC through WinAppMigrator Client, in three steps. "WinAppMigrator Client" is the name of a
## program, and the old PC is the one the apps are moved off.

app-network-step-client = 1. 古い PC にクライアントを用意
app-network-step-client-hint = WinAppMigrator Client は、現在アプリがある PC（Windows 7 以降）で実行します。クライアントに表示されるコードを使って、この PC からの暗号化された読み取りだけを許可します。
# { $drive } is a USB drive, such as E:\.
app-network-save-to-usb = USB ドライブ { $drive } に保存
# { $drive } is a USB drive, such as E:\, and { $name } the name it was given, such as BACKUP.
app-network-save-to-named-usb = USB ドライブ { $drive }（{ $name }）に保存
app-network-save-to-folder = フォルダーに保存…
app-network-look-for-usb-again = USB ドライブを再検索
app-network-no-usb = USB ドライブが接続されていません。USB ドライブを接続するか、フォルダーに保存してください。保存先は、ネットワーク上の共有フォルダーなど、古い PC からアクセスできるフォルダーにしてください。
# { $path } is where the client was saved, such as E:\WinAppMigrator Client.exe.
app-network-client-saved = { $path } として保存しました。古い PC で、移行するアプリを閉じてから、このファイルをダブルクリックし、変更を許可してください。コードが表示されます。
app-network-step-pick = 2. 古い PC を選択
app-network-looking = クライアントを実行している PC をネットワーク上で探しています…
app-network-type-name = または、名前かアドレスを入力
# An example, shown in the empty field: a PC's name, or its network address.
app-network-type-name-example = OLD-PC または 192.168.1.20
app-network-step-code = 3. 表示されたコードを入力
app-network-code = ペアリング コード
app-network-connect = 接続
app-network-pairing = ペアリングしています…
# Why pairing did not work, shown on its own.
app-network-pairing-stopped = ペアリングが完了する前に停止しました
app-network-wrong-code = 古い PC に表示されているコードと一致しません。確認して、もう一度お試しください。
# { $error } is what went wrong.
app-network-not-reached = 古い PC に接続できませんでした：{ $error }
app-network-pick-folder-title = WinAppMigrator Client の保存先フォルダーの選択
# { $path } is the folder, and { $error } what Windows said went wrong.
app-client-could-not-save = クライアントを { $path } に保存できませんでした：{ $error }
# scripts\build-client.ps1 is a file name.
app-client-not-carried = この WinAppMigrator のビルドにはクライアントが含まれていません。scripts\build-client.ps1 でクライアントをビルドしてから、アプリをもう一度ビルドしてください。
# { $error } is what Windows said went wrong.
app-client-could-not-save-note = クライアントの横に置くメモを保存できませんでした：{ $error }
# A note saved as a text file beside the client, for whoever runs it on the old PC. Keep its lines short, as it is read in
# Notepad. "WinAppMigrator Client.exe" is a file name. "Network PC..." names the Scan Source button: use its words in your
# language.
app-client-note =
    WinAppMigrator Client
    
    アプリの移行元の PC（Windows 7、8、10、11）でこれを実行してください。
    
    1. 移行するアプリを閉じます。
    2. 「WinAppMigrator Client.exe」をダブルクリックし、変更を許可します。
    3. 6 桁のペアリング コードが表示されます。新しい PC の WinAppMigrator で
       「ネットワーク PC...」を選び、この PC を選択してコードを入力します。
    4. 新しい PC での作業が終わったら、クライアントのウィンドウで Enter キーを
       押すか、ウィンドウを閉じます。
    
    このプログラムは読み取りだけを行います。実行中は、Windows ドライブの
    スナップショットを保持し、ローカル ネットワークからの通信用に専用の
    2 つのポートを Windows ファイアウォールで許可します。どちらも停止時に
    削除されます。ネットワーク上の通信は暗号化され、コードを知っている PC
    だけが内容を読み取れます。
