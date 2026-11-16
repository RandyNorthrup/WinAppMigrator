# Words the core says about what an install may do: why an install location or something in a package is refused,
# the installer's list of what an app sets up beyond its files, the checks of a package and of an install, and whether
# there is room for either. Message names start with core-.
#
# Many messages here are a reason, the second half of a sentence that starts "... was left out:" or "that install
# location cannot be used:", so they start with a small letter and have no full stop.


## Why an install location cannot be used. Shown in red under the Install Location field.

core-review-location-state-root = インストールの記録を保存する場所です
core-review-location-has-files = 既にファイルがあり、ここにはインストールの記録がないため、空のフォルダーを使用してください
core-policy-location-not-full-path = 完全なパスである必要があります
core-policy-location-parent-step = 「..」を含めることはできません
core-policy-location-not-plain = この PC のドライブ上の通常のパスである必要があります
core-policy-location-whole-drive = ドライブ全体は指定できません
core-policy-location-windows = Windows フォルダーは指定できません
core-policy-location-runs-by-itself = Windows に属するフォルダー、または内容が自動的に実行されたり読み込まれたりするフォルダーです
core-policy-location-shared = ほかのものと並べて保持するフォルダーのため、アプリにはその中のフォルダーが必要です

## Whether an app can run on this PC at all, said before anything is installed.

# { $reason } is why, as the second half of a sentence, such as "App.exe is built for Arm64, and this PC is x64, so
# Windows would not start it".
core-review-cannot-run-here = このアプリはこの PC では実行できないため、何も変更されませんでした：{ $reason }
core-policy-architecture-x86 = 32 ビット x86
core-policy-architecture-x64 = x64
core-policy-architecture-arm64 = Arm64
core-policy-architecture-arm32 = 32 ビット Arm
core-policy-architecture-unknown = 不明なアーキテクチャ
# { $code } is the number Windows gives a kind of processor, such as 0x01c4.
core-policy-machine-type = マシンの種類 { $code }
# { $names } is a list of program names, such as "a.exe, b.exe, c.exe", and { $count } how many more there are.
core-policy-names-and-more = { $names }、ほか { $count } 件
core-policy-fit-programs-not-said = このインストーラーにはプログラムに必要なものが記載されていないため、この PC で実行できるかどうかは確認されていません。
core-policy-fit-architecture-not-told = この PC のアーキテクチャを判別できなかったため、このアプリのプログラムを実行できるかどうかは確認されていません。
core-policy-fit-main-not-said = このインストーラーにはメイン プログラムに必要なものが記載されていないため、ここに示す内容はそれ以外の情報に基づいています。
# { $driver } is a driver's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-driver = ドライバー { $driver } は { $built_for } 用にビルドされていますが、この PC は { $here } です。ドライバーは Windows と完全に一致する必要があるため、このアプリはここでは実行できません
# { $program } is a program's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-program = { $program } は { $built_for } 用にビルドされていますが、この PC は { $here } のため、Windows はこれを起動しません
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-none-start = どのプログラムも、この PC（{ $here }）では起動できません：{ $programs }
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-some-other-architecture = このアプリの一部のプログラムは別のアーキテクチャ用にビルドされており、この PC（{ $here }）では起動しません：{ $programs }。アプリのほかの部分は起動できるため、問題にならない可能性があります。
# { $here } is a kind of processor, such as "Arm64". { $programs } is a list of program names.
core-policy-fit-emulated = この PC は { $here } のため、Windows は次のプログラムをエミュレーションで実行します：{ $programs }。ほとんどのプログラムはその方法で問題なく動作しますが、動作しないものもあります。
# { $program } is a program's file name. { $needs } and { $runs } are versions of Windows, such as 10.0 and 6.1.
core-policy-fit-needs-newer = { $program } には Windows { $needs } 以降が必要ですが、この PC は { $runs } のため、Windows はこれを起動しません
# { $programs } is a list of program names.
core-policy-fit-some-need-newer = このアプリの一部のプログラムには、この Windows より新しいバージョンが必要なため、ここでは起動しません：{ $programs }。
# { $from_build } and { $this_build } are Windows build numbers, such as 22631, written as Windows writes them.
core-policy-fit-older-build = このアプリは Windows のビルド { $from_build } 上にあり、この PC はビルド { $this_build } です。プログラムはこの Windows で起動できると示しているため動作する可能性がありますが、移行元の PC で使用していたものがここにはない可能性があります。
core-policy-fit-build-not-told = この Windows のビルドを判別できなかったため、このアプリがあった PC のビルドとは比較していません。

## Room on a drive for a build or an install.

# Each of these ends the sentence core-space-shortfall, after "free for".
core-space-for-unpacking = パッケージの展開
core-space-for-install-record = インストールの記録
core-space-for-app-files = アプリのファイル
core-space-for-what-is-replaced = 置き換えられるものの保存
core-space-for-build-work = 作成時の作業データ
core-space-for-installer = インストーラー
core-space-for-package = パッケージ
core-space-for-unpacking-and-installing = パッケージの展開とインストール
# What goes between two of those, as in "unpacking the package and the app's files". Keep the quotes: they keep the
# spaces.
core-space-purposes-joiner = { "と" }
# { $drive } is a drive, such as C:, or a folder. { $needed } and { $free } are sizes, such as 1.2 GB. { $purposes } is
# what the room is for, such as "unpacking the package and the app's files".
core-space-shortfall = { $drive } では、{ $purposes }のために最大 { $needed } の空き領域が必要ですが、空きは { $free } です
# { $drive } is a drive, such as C:, or a folder. { $needed } is a size, such as 1.2 GB.
core-space-not-told = { $drive } の空き領域を取得できなかったため、必要な { $needed } が収まるかどうかは確認されていません
# One drive in a list of where room is needed. { $needed } and { $free } are sizes, and { $drive } a drive, such as C:.
core-space-summary = { $drive } に最大 { $needed }（空き { $free }）
# { $needed } is a size, and { $drive } a drive, such as C:.
core-space-summary-not-told = { $drive } に最大 { $needed }（空き領域は不明）
# What goes between two sentences of core-space-shortfall inside core-review-no-room. Keep the quotes: they keep the
# space.
core-review-sentence-joiner = { "。" }
# { $shortfalls } is one or more sentences of core-space-shortfall.
core-review-no-room = 空き領域が不足しているため、何も変更されませんでした。{ $shortfalls }。そのドライブの空き領域を増やすか、別のドライブのインストール先を選択してください。

## The installer's list of what an app sets up beyond its files: what each kind is called.

core-review-kind-service = サービス
core-review-kind-driver = ドライバー
core-review-kind-scheduled-task = スケジュールされたタスク
core-review-kind-autostart = 自動起動
core-review-kind-shortcut = ショートカット
core-review-kind-path = PATH
core-review-kind-environment-variable = 環境変数
core-review-kind-shell-extension = シェル拡張
core-review-kind-menu-command = メニュー コマンド
core-review-kind-link-type = リンクの種類
core-review-kind-file-type = ファイルの種類
core-review-kind-com = COM
core-review-kind-firewall-rule = ファイアウォール規則
core-review-kind-event-log-source = イベント ログのソース
core-review-kind-font = フォント
core-review-kind-odbc-driver = ODBC ドライバー
core-review-kind-odbc-source = ODBC データ ソース
core-review-kind-compatibility-setting = 互換性の設定
core-review-kind-default-apps = 既定のアプリ
core-review-kind-program-name = プログラム名
core-review-kind-other-settings = その他の設定

## The installer's list of what an app sets up: what each line says of it.

# { $file } is a program's file name. { $account } is the Windows account it runs as, such as LocalSystem. { $starts }
# says when it starts, and is not shown itself; for "other", { $start } is one of the core-review-start- words below.
core-review-service-detail =
    { $starts ->
        [automatic] { $file } を { $account } として実行します。今すぐ開始し、以後この PC の起動時に毎回開始します
        [delayed] { $file } を { $account } として実行します。今すぐ開始し、以後この PC の起動後に毎回少し遅れて開始します
        [manual] { $file } を { $account } として実行します。要求があったときにのみ開始します
        [disabled] { $file } を { $account } として実行します。無効になっています
        [other] { $file } を { $account } として実行します。{ $start }
       *[unknown] { $file } を { $account } として実行します
    }
# { $account } is a Windows account, such as LocalService.
core-policy-account-custom = { $account }（カスタム アカウントは復元されません）
# { $file } is a driver's file name.
core-review-driver-detail = { $file } を Windows 本体に読み込みます
# { $file } is a driver's file name. { $start } is when Windows starts it: one of the core-review-start- words below.
core-review-driver-detail-starts = { $file } を Windows 本体に読み込みます。{ $start }
# When a service or driver starts, ending a sentence such as "Loads disk.sys into Windows itself, starts …".
core-review-start-boot = Windows の起動時に、ほかのすべてより先に開始します
core-review-start-system = Windows の起動中に開始します
core-review-start-automatic = この PC の起動時に毎回開始します
core-review-start-delayed = この PC の起動後に毎回少し遅れて開始します
core-review-start-manual = 要求があったときにのみ開始します
core-review-start-disabled = 無効になっているため、開始しません
core-review-start-unknown = 開始方法は Windows で示されていません
# { $account } is the account a task runs as: an account's name, or core-review-task-runs-as-you, or
# core-review-task-runs-as-creator.
core-review-task-detail = { $account } として実行します
# { $account } is the account a task runs as, as for core-review-task-detail.
core-review-task-detail-highest = { $account } として、最上位の特権で実行します
core-review-task-runs-as-you = サインインしているユーザー
core-review-task-runs-as-creator = タスクを作成したユーザー
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-autostart-detail =
    { $scope ->
        [everyone] すべてのユーザーのサインイン時に実行します
       *[you] 現在のユーザーのサインイン時に実行します
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shortcut-startup-detail =
    スタートアップ フォルダー内のショートカット（{ $scope ->
        [everyone] この PC のすべてのユーザーが対象
       *[you] 現在のユーザーのみが対象
    }）
# { $file } is the file name of what the shortcut opens. { $scope } is whom it is set up for: everyone on this PC,
# or you. It is not shown itself.
core-review-shortcut-detail =
    { $file } を開きます（{ $scope ->
        [everyone] この PC のすべてのユーザーが対象
       *[you] 現在のユーザーのみが対象
    }）
# { $name } is an environment variable's name, such as PATH.
core-review-path-title = { $name } に追加
core-review-path-install-folder = インストール フォルダー
# { $path } is a folder inside the install folder, such as bin\tools.
core-review-path-below-install-folder = インストール フォルダー\{ $path }
# { $value } is the variable's value, as it will be set.
core-review-variable-detail = { $value } に設定
# { $count } is how many registry values.
core-review-other-settings-title =
    { $count ->
       *[other] アプリ自身のキー以外の設定 { $count } 件
    }
# { $keys } is a list of registry keys, such as HKCU\Software\Vendor.
core-review-other-settings-detail = 場所：{ $keys }
# { $keys } is a list of registry keys, and { $count } how many more there are.
core-review-other-settings-detail-more = 場所：{ $keys }、ほか { $count } 件
# { $way } says whether the rule allows or blocks, and incoming or outgoing connections; it is not shown itself.
# { $ports } is a list of port numbers, and { $port_count } how many. { $program } is the program's file name.
# { $networks } is a list of the kinds of network Windows names in English (domain, private, public), and
# { $network_count } how many.
# The kinds of network a firewall rule is for, as Windows Defender Firewall names its profiles in your language. They
# fill { $networks } in core-review-firewall-detail, joined into a list.
core-review-network-domain = ドメイン
core-review-network-private = プライベート
core-review-network-public = パブリック
core-review-firewall-detail =
    { $program } の{ $port_count ->
        [0] { "" }
       *[other] ポート { $ports } での
    }{ $way ->
        [allow-in] 受信接続を許可します
        [allow-out] 送信接続を許可します
        [block-in] 受信接続をブロックします
       *[block-out] 送信接続をブロックします
    }（{ $network_count ->
        [0] すべてのネットワーク
       *[other] { $networks } ネットワーク
    }）
# { $server } is the file name of what Explorer loads. { $scope } is whom it is set up for: everyone on this PC, or
# you. It is not shown itself.
core-review-shell-extension-detail =
    { $server } をエクスプローラーに読み込みます（{ $scope ->
        [everyone] この PC のすべてのユーザーが対象
       *[you] 現在のユーザーのみが対象
    }）
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shell-extension-detail-own-file =
    インストールするファイルをエクスプローラーに読み込みます（{ $scope ->
        [everyone] この PC のすべてのユーザーが対象
       *[you] 現在のユーザーのみが対象
    }）
# The name of a shell extension: { $handler } is its name in the registry, and { $class } the kind of file or folder it
# is for, such as * or Directory.
core-policy-shell-extension-name = { $handler }（{ $class } 用）
# { $on } is the kind of file or folder whose menu it is, as the registry names it, such as * or
# Directory. { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-menu-command-detail =
    { $on } のメニューにコマンドを追加します（{ $scope ->
        [everyone] この PC のすべてのユーザーが対象
       *[you] 現在のユーザーのみが対象
    }）
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-default-verb-detail =
    この種類のものを開いたときの動作を変更します（{ $scope ->
        [everyone] この PC のすべてのユーザーが対象
       *[you] 現在のユーザーのみが対象
    }）
core-review-client-entry-detail = これに使うプログラムとして選択できるようになります。既定はこのプログラムに変更されません
core-review-com-permissions-title = COM のアクセス許可
core-review-com-permissions-detail = COM クラスを開始できるユーザー、またはクラスを実行するアカウントを指定します
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-link-type-detail =
    この種類のリンクをアプリで開きます（{ $scope ->
        [everyone] この PC のすべてのユーザーが対象
       *[you] 現在のユーザーのみが対象
    }）
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-file-type-detail =
    この種類のファイルをアプリで開きます（{ $scope ->
        [everyone] この PC のすべてのユーザーが対象
       *[you] 現在のユーザーのみが対象
    }）
# { $log } is the name of a Windows event log, such as Application.
core-review-event-log-source-detail = 独自のファイルのメッセージを使用して、{ $log } ログに書き込めます
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-font-detail =
    サインイン時にインストール フォルダーから読み込みます（{ $scope ->
        [everyone] この PC のすべてのユーザーが対象
       *[you] 現在のユーザーのみが対象
    }）
core-review-odbc-driver-detail = ODBC を使用してデータベースに接続するプログラムに読み込まれます
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-odbc-source-detail =
    データベース接続（{ $scope ->
        [everyone] この PC のすべてのユーザーが対象
       *[you] 現在のユーザーのみが対象
    }）
core-review-default-apps-detail = 「既定のアプリ」で選択できるようになります。既定はこのアプリに変更されません
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-program-name-detail =
    「ファイル名を指定して実行」やほかのプログラムから、この名前で見つけられるようになります（{ $scope ->
        [everyone] この PC のすべてのユーザーが対象
       *[you] 現在のユーザーのみが対象
    }）
# { $server } is the file name of what serves the classes.
core-review-com-title = { $server } の COM
core-review-com-title-own-file = インストールするファイルの COM
# { $count } is how many registry values.
core-review-com-detail =
    { $count ->
       *[other] ほかのプログラムが開始できるクラスの設定 { $count } 件
    }
core-review-com-support-title = COM タイプ ライブラリ、インターフェイス、AppID
# { $libraries }, { $interfaces }, and { $app_ids } are how many registry values there are for its type libraries, its
# interfaces, and its AppIDs.
core-review-com-support-detail = クラスに付随する設定：タイプ ライブラリ { $libraries } 件、インターフェイス { $interfaces } 件、AppID { $app_ids } 件
core-review-compatibility-title = プログラムの起動方法
# { $count } is how many registry values.
core-review-compatibility-detail =
    { $count ->
       *[other] インストールするプログラムの互換性の設定 { $count } 件
    }

## The installer's notes on whose folders and settings these are, and what is not coming.

# { $account } is the name of a Windows account, such as alice.
core-review-account-signed-in = ユーザー データとユーザーごとの設定は、これを実行している管理者ではなく、ここにサインインしている { $account } のアカウントに配置されます。
# { $account } is the name of a Windows account, such as alice.
core-review-account-no-desktop = このセッションには誰もサインインしていないため、ユーザー データとユーザーごとの設定は、これを実行しているアカウント { $account } に配置されます。
# { $reason } is what Windows said, in its own words.
core-review-account-not-told = サインインしているユーザーを判別できなかったため（{ $reason }）、ユーザーごとの設定は除外され、ユーザー データはインストール先の下に配置されます。
# { $reason } is what Windows said, in its own words.
core-review-signed-in-unknown = サインインしているユーザーを判別できませんでした：{ $reason }
# Headings of what is left out, shared by the app's list and the installer's notes.
core-review-left-out-credentials = 資格情報のように見えるため、このインストーラーには含まれません
core-review-left-out-settings = アカウントがこのアプリ用に保持している設定と履歴。「ユーザー データ」オプションを使用した場合にのみ含まれます
core-review-left-out-certificates = アプリは移行元の PC で信頼されていた証明書を使用しているようですが、それらはインストールされません
core-review-left-out-fonts = これらのフォントにはアプリの名前が付いていますが、アプリ自身のものであることを示す情報がないため、インストールされません
# { $names } is a list of names, such as the names of saved passwords.
core-review-note-credentials = 資格情報のように見えるため、このインストーラーには含まれていません：{ $names }。この PC 上にある該当の情報はそのまま残ります。再度入力が必要になる場合があります。
# { $names } is a list of names of license data.
core-review-note-licenses = 作成時に「ライセンスとライセンス認証のデータ」オプションがオフだったため、このインストーラーには含まれていません：{ $names }。
# { $names } is a list of places an account keeps settings, such as folders.
core-review-note-settings = 作成時に「ユーザー データ」オプションがオフだったため、このインストーラーには含まれていません：アカウントがこのアプリ用に保持している設定と履歴（{ $names }）。この PC では、アプリを初めて使う場合と同じ状態で起動します。
# { $names } is a list of certificates, each as core-review-certificate.
core-review-note-certificates = アプリは移行元の PC で信頼されていた証明書を使用しているようですが、それらはインストールされません：{ $names }。この PC でそれらを信頼するかどうかを判断してください。
# { $names } is a list of font names.
core-review-note-fonts = これらのフォントにはアプリの名前が付いていますが、アプリ自身のものであることを示す情報がないため、インストールされません：{ $names }。
# { $names } is a list of names.
core-review-note-left-out-by-choice = このインストーラーの作成者が除外しました：{ $names }。
# { $names } is a list of names.
core-review-note-added-by-choice = このインストーラーの作成者が、アプリの一部と判断して追加しました：{ $names }。
# { $name } is a certificate's subject, such as CN=Contoso, and { $store } the certificate store it was in, such as
# Root.
core-review-certificate = { $name }（{ $store }）
# { $names } is a list of names, and { $count } how many more there are.
core-review-names-and-more = { $names }、ほか { $count } 件

## Why something in a package is left out. Each is the second half of a sentence of core-warning-review-left-out.

core-review-no-account = 1 つのアカウントの設定に属していますが、サインインしているユーザーを判別できませんでした
core-review-name-not-accepted = Windows がその名前を受け付けません
# { $error } is what Windows said, in its own words.
core-review-service-unreadable = 現在の設定を読み取れませんでした：{ $error }
core-review-service-dependency = 無効な名前のサービスに依存しています
# { $path } is the program the service runs.
core-review-service-runs-other = このパッケージでインストールされない { $path } を実行します
core-review-service-settings = 設定が、このパッケージでインストールされないもの、または自身のものではないキーを参照しています
core-review-driver-no-hash = 確認に使用するハッシュがありません
core-review-license-for-the-machine = PC 全体に保存されたライセンスは報告のみ行います
core-review-license-starts-something = ライセンスが何かを起動する理由はありません
core-review-task-not-captured = 定義が取り込まれていません
# { $error } is what was wrong with the task's XML, in English.
core-review-task-unreadable = 定義を読み取れませんでした：{ $error }
core-review-task-no-account = インストールしたユーザーとして実行されますが、ここでサインインしているユーザーを判別できませんでした
# { $path } is what the shortcut opens.
core-review-shortcut-opens-other = このパッケージでインストールされない { $path } を開きます
# { $service } is the service's name, and { $path } the program it runs now.
core-policy-service-exists = { $service } はこの PC に既に存在し、{ $path } を実行していますが、これはここでの以前のインストールによるものではありません。このアプリの古いコピーの場合は、先にそれをアンインストールしてください
core-policy-key-control-characters = キー名に制御文字が含まれています
core-policy-value-control-characters = 値の名前に制御文字が含まれています
core-policy-registry-link = レジストリ リンクです
# { $reason } is why the key it stands for is left out, such as "no app install should change this key".
core-policy-virtual-store = PC 全体のキーの代わりとなるもので、{ $reason }
core-policy-registry-part = アプリのインストールでレジストリのこの部分に書き込むべきではありません
core-policy-key-refused = アプリのインストールでこのキーを変更すべきではありません
core-policy-net-debugger = すべての .NET プログラムのデバッガーを指定することになります
core-policy-net-install-root = すべての .NET プログラムが .NET を読み込む場所を変更することになります
core-policy-class-redirects = ある種類のプログラムやフォルダーの開き方を変更することになります
core-policy-app-path-name = ファイル名以外の名前でプログラムを指定しています
core-policy-app-path-not-installed = このパッケージでインストールされないファイルのプログラム名を登録することになります
core-policy-shared-default-verb = ある種類のすべてのファイルやフォルダーを開いたときの動作を変更することになります
core-policy-command-runs-other = このパッケージでインストールされないものを実行するコマンドを追加します
core-policy-autostart-not-command = 自動的に起動しますが、その値がコマンドではありません
core-policy-autostart-runs-other = 自動的に起動し、このパッケージでインストールされないものを実行します
core-policy-environment-own-rules = 環境変数は専用の規則で復元されます
core-policy-service-not-restored = このパッケージで復元されないサービスに属しています
core-policy-service-own-rules = サービス自身の設定は、サービス用の規則によってのみ復元されます
core-policy-service-dll = ServiceDll が、このパッケージでインストールされるファイルではありません
core-policy-event-log-source-only = 復元できるのはイベント ログのソース自身の値だけです
core-policy-security-log = アプリがセキュリティ ログに書き込むことはありません
core-policy-event-log-messages = メッセージが、このパッケージでインストールされないファイルから取得されることになります
core-policy-type-library = タイプ ライブラリが、このパッケージでインストールされるファイルではありません
# { $name } is a program's file name, such as app.exe.
core-policy-application-opened = このパッケージでインストールされない { $name } の開き方を変更することになります
core-policy-event-log-no-file = イベント ログのソースは、ファイルからメッセージを取得する必要があります
core-policy-com-runs-other = このパッケージでインストールされるファイル以外のものを実行します
core-policy-com-redirect = このクラスを、このパッケージでインストールされないものに変えることになります
core-policy-com-server = このパッケージでインストールされるファイルでも Windows の一部でもないものを実行します
core-policy-interface = このパッケージでインストールされないものによって、インターフェイスの呼び出し方法を決めることになります
# { $name } is a program's file name, such as app.exe.
core-policy-app-id-started = このパッケージでインストールされない { $name } の起動方法を変更することになります
core-policy-app-id-surrogate = このパッケージでインストールされないプログラムによってホストされることになります
core-policy-app-id-service = このパッケージで復元されないサービス内で実行されることになります
core-policy-app-id-run-as = 独自のアカウントとして実行されることになります
core-policy-app-id-link = AppID を指定していません
core-policy-app-id-remote = この PC 以外の場所で実行されることになります
core-policy-odbc-own-driver = これは ODBC 自身の設定であり、ドライバーではありません
core-policy-odbc-driver-not-installed = ドライバーが、このパッケージでインストールされるファイルではありません
core-policy-loads-other-file = このパッケージでインストールされないファイルを読み込むことになります
core-policy-odbc-own-source = これは ODBC 自身の設定であり、データ ソースではありません
core-policy-odbc-source-missing = このパッケージにないデータ ソースが記載されています
core-policy-odbc-source-driver = ドライバーが、このパッケージでインストールされるファイルでも Windows の一部でもありません
core-policy-shell-extension = このパッケージでインストールされるファイルではないシェル拡張を、エクスプローラーに読み込むことになります
core-policy-not-text = 保持している内容が、読み取られる形式のテキストではありません
core-policy-network-place = ネットワーク上の場所を指定しています
core-policy-font = このパッケージでインストールされないフォント、または Windows フォルダー内のフォントを登録することになります
core-policy-compatibility = このパッケージでインストールされないプログラムの実行方法を変更することになります
core-policy-capabilities = 詳細がこのパッケージに含まれていないアプリが記載されています
core-policy-default-program = 既定で開くプログラムを変更することになります
core-policy-url-protocol = リンクの種類であるとしていますが、このパッケージでインストールされるものでは開かれません
core-policy-command-handler = このパッケージでインストールされないものが処理するコマンドを追加します
core-policy-installer-command = Windows Installer でしか読み取れないものを実行することになります
core-policy-creates-class = このパッケージでインストールされないクラスを作成することになります
core-policy-opens-links = このパッケージでインストールされないものでリンクを開きます
core-policy-opens-files = このパッケージでインストールされないものでファイルを開きます
core-policy-firewall-name = 名前が、引き継ぐことのできるものではありません
core-policy-firewall-description = 説明が、引き継ぐことのできるものではありません
core-policy-firewall-every-program = すべてのプログラムに適用されますが、規則は 1 つのプログラムに対してのみ作成できます
# { $path } is the program the rule is for.
core-policy-firewall-program = このパッケージでインストールされない { $path } が対象です
core-policy-firewall-direction = 設定できる方向が指定されていません
core-policy-firewall-action = ほかの規則より優先されることになりますが、アプリの規則でそうすべきではありません
# { $setting } is a part of a firewall rule, as the scan of its PC described it.
core-policy-firewall-not-carried = { $setting } が指定されていますが、元のとおりに設定できないか、ほかの規則の効果を弱めることになります
core-policy-firewall-service = このパッケージで復元されないサービスが対象です
core-policy-firewall-parts = 一部が、ファイアウォール規則で指定できる内容ではありません
core-policy-folder-already-there = そのフォルダーはインストール先の外に既にあり、このアプリのものであることを示す情報がないため、プログラムは配置されません
core-policy-variable-name = 名前が有効な変数名ではありません
core-policy-variable-relied-on = Windows やほかのプログラムがこの変数に依存しています
core-policy-task-name = 名前が、Windows で受け付けられるものではありません
core-policy-task-folder = フォルダーが、Windows で受け付けられるものではありません
core-policy-task-windows = Windows 自身のタスクを置き換えることになります
core-policy-task-other-action = プログラムの実行以外の操作を行います
core-policy-task-no-program = 実行するプログラムが指定されていません
# { $path } is the program the task runs.
core-policy-task-runs-other = このパッケージでインストールされないものを実行します：{ $path }
core-policy-task-exists-unreadable = この名前のタスクが既に存在し、読み取れませんでした
core-policy-task-exists-other = この名前のタスクが既に存在し、そのタスクはアプリを実行しません
# { $path } is the program the task already on this PC runs.
core-policy-task-exists-runs = この名前のタスクが既に存在し、{ $path } を実行していますが、これはここでの以前のインストールによるものではありません
core-policy-shortcut-not-lnk = .lnk ショートカットではありません
core-policy-shortcut-place = スタート メニューまたはデスクトップ上にありません
core-policy-short-name = パスに確認できない名前が使用されています
core-policy-shortcut-no-folder = この PC にはそのためのフォルダーがありません
core-policy-data-startup = スタートアップ フォルダーに配置され、サインインのたびに実行されることになります
core-policy-data-powershell = PowerShell が起動時に見つけたものを実行する場所に配置されることになります
core-policy-data-windows = Windows に属するフォルダーに配置されることになります
core-policy-license-outside = インストール先と既知のすべてのデータ フォルダーの外にあります
core-rewrite-cdata = CDATA セクションが含まれています
core-rewrite-too-much-to-look-through = このパッケージの設定に含まれる内容が多すぎるため、参照先を確認できません

## Warnings a review gives, which the installer shows with what it sets up.

# { $kind } says what was left out, and is not shown itself. { $item } is its path or name, or a registry value such
# as HKLM\Software\App\Name. { $reason } is why, as the second half of the sentence, such as "it cannot be a whole
# drive".
core-warning-review-left-out =
    { $kind ->
        [file] ファイル { $item } は除外されました：{ $reason }。
        [folder] フォルダー { $item } は除外されました：{ $reason }。
        [service] サービス { $item } は除外されました：{ $reason }。
        [driver] ドライバー { $item } は除外されました：{ $reason }。
        [firewall-rule] ファイアウォール規則 { $item } は除外されました：{ $reason }。
        [license-data] { $item } のライセンス データは除外されました：{ $reason }。
        [license-value] ライセンス値 { $item } は除外されました：{ $reason }。
        [scheduled-task] スケジュールされたタスク { $item } は除外されました：{ $reason }。
        [shortcut] ショートカット { $item } は除外されました：{ $reason }。
        [environment-variable] 環境変数 { $item } は除外されました：{ $reason }。
       *[value] { $item } は除外されました：{ $reason }。
    }
core-warning-review-package-path-missing = コピー可能なファイルにパッケージ内の相対パスがありません。
# { $count } is the most empty folders a package makes.
core-warning-review-too-many-empty-folders = このパッケージには { $count } 個を超える空のフォルダーが記載されており、作成されるのはその数までです。
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Documents folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-documents-elsewhere =
    { $count ->
       *[other] ドキュメントの { $count } 個のファイルは、代わりに { $target } の下の _external という名前のフォルダーに配置されます。この PC のドキュメント フォルダーは { $folder } にあり、書き込まれるのはプロファイル内のフォルダーだけであるためです。使用するには、手動でドキュメント フォルダーに移動してください。
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-documents-unknown =
    { $count ->
       *[other] ドキュメントの { $count } 個のファイルは、代わりに { $target } の下の _external という名前のフォルダーに配置されます。この PC のドキュメント フォルダーの場所を判別できなかったためです。使用するには、手動でドキュメント フォルダーに移動してください。
    }
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Saved Games folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-elsewhere =
    { $count ->
       *[other] 保存したゲームの { $count } 個のファイルは、代わりに { $target } の下の _external という名前のフォルダーに配置されます。この PC の保存したゲーム フォルダーは { $folder } にあり、書き込まれるのはプロファイル内のフォルダーだけであるためです。使用するには、手動で保存したゲーム フォルダーに移動してください。
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-unknown =
    { $count ->
       *[other] 保存したゲームの { $count } 個のファイルは、代わりに { $target } の下の _external という名前のフォルダーに配置されます。この PC の保存したゲーム フォルダーの場所を判別できなかったためです。使用するには、手動で保存したゲーム フォルダーに移動してください。
    }

## Errors when a folder that only administrators may change cannot be made so, or checked.

# { $path } is a folder.
core-privilege-not-a-folder = { $path } はフォルダーではなく、リンクまたはファイルです
# { $path } is a folder. { $error } is what Windows said, in its own words.
core-privilege-not-made-here = { $path } は WinAppMigrator が作成したものではなく、ほかのユーザーが内容を変更できます。管理者が確認して削除する必要があります：{ $error }
# { $path } is a folder.
core-privilege-others-put-something-in = { $path } には WinAppMigrator が作成したものではないものが含まれており、ほかのユーザーが変更できます。管理者が確認して削除する必要があります
# { $path } is a folder.
core-privilege-became-a-link-in-use = { $path } は、使用中にリンクに変わりました
# { $path } is a folder.
core-privilege-became-a-link-looked-at = { $path } は、確認中にリンクに変わりました
# { $path } is a file. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-users-cannot-start = ユーザーが { $path } を起動できるように設定できませんでした：{ $said }
core-privilege-elevation-unknown = このプロセスが管理者として実行されているかどうかを判別できませんでした
# { $path } is a folder. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-cannot-restrict = { $path } へのアクセスを管理者のみに制限できませんでした：{ $said }
# { $path } is a folder. { $said } is the folder's permissions as Windows wrote them, and what else it printed.
core-privilege-not-restricted = { $path } へのアクセスを管理者のみに制限できませんでした。このドライブではアクセス許可が保持されない可能性があります：{ $said }
# { $path } is a folder.
core-privilege-written-to-first = 保護される前に、別のプロセスが { $path } に書き込みました
core-privilege-no-program-data = Windows から ProgramData の場所が示されません

## Checks of a package's files, and of what an install put in place. Each is said of one file, value, service, or the
## like, which the report names beside it.

# { $path } is the installer's file.
core-verify-no-package = { $path } にはパッケージが含まれていません
# { $path } is a folder.
core-verify-not-unpacked = このインストーラーのパッケージは { $path } に展開されていません
core-verify-schema-supported = パッケージ マニフェストのスキーマはサポートされています。
core-verify-packaged-file-missing = パッケージ内のファイルがありません。
core-verify-no-package-path = コピー可能なファイルにパッケージ内の相対パスがありません。
core-verify-folder-there = 空のフォルダーが存在します。
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-unreadable = 確認のために読み取ることができませんでした：{ $error }
core-verify-restored-file-missing = 復元されたファイルがありません。
# { $expected } and { $actual } are sizes in bytes.
core-verify-size-mismatch = ファイル サイズが一致しません。想定は { $expected } バイトですが、実際は { $actual } バイトです。
core-verify-no-hash = マニフェストにこのファイルのハッシュが記録されていないため、確認できません。
core-verify-hash-mismatch = SHA-256 ハッシュが一致しません。
core-verify-file-matches = ファイル サイズとハッシュはマニフェストと一致しています。
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-payload-failed = ライセンス ペイロードの検証に失敗しました：{ $error }
core-verify-license-payload-missing = パッケージ内のライセンス ペイロードがありません。
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-failed = ライセンス ストアの検証に失敗しました：{ $error }
core-verify-license-store-file-missing = パッケージ内のライセンス ストア ファイルがありません。
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-path-invalid = パッケージ内のライセンス ストアのパスが無効です：{ $error }
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-prerequisite-failed = 前提条件のペイロードの検証に失敗しました：{ $error }
core-verify-prerequisite-missing = 同梱の前提条件のペイロードがありません。
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-failed = ドライバー イメージの検証に失敗しました：{ $error }
core-verify-bundled-driver-missing = 同梱のドライバー イメージがありません。
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-path-invalid = 同梱のドライバーのパスが無効です：{ $error }
core-verify-prerequisite-present = ランタイムの前提条件は対象の PC に存在します。
core-verify-prerequisite-not-detected = ランタイムの前提条件が検出されませんでした。再起動または手動での完了が必要な場合があります（正式な結果は復元ログを参照してください）。
core-verify-prerequisite-not-probed = 前提条件の有無はレジストリで確認できないため、インストールはベスト エフォートで行われました。
core-verify-license-file-missing = 復元されたライセンス ファイルがありません。
core-verify-license-store-restored-missing = 復元されたライセンス ストア ファイルがありません。
core-verify-registry-key-there = レジストリ キーが存在します。
core-verify-registry-value-matches = レジストリ値はアーカイブされたデータと一致しています。
core-verify-registry-value-differs = レジストリ値がアーカイブされたデータと一致しません。
core-verify-firewall-rule-there = Windows ファイアウォールにこの規則があります。
core-verify-variable-matches = 環境変数の値はアーカイブされたデータと一致しています。
core-verify-variable-differs = 環境変数の値がアーカイブされたデータと一致しません。
# { $error } is what Windows said, in its own words.
core-verify-variable-unreadable = 環境変数が存在しないか、読み取れません：{ $error }
core-verify-service-runs-other = サービスは存在しますが、復元されたバイナリを引数付きで実行していません。
core-verify-service-other-account = サービスは存在しますが、想定したアカウントとして実行されていません。
core-verify-service-matches = サービスが存在し、復元されたバイナリを想定どおりの引数とアカウントで実行しています。
core-verify-service-missing = サービスがありません。
# { $error } is what Windows said, in its own words.
core-verify-service-unreadable = サービスを読み取れません：{ $error }
core-verify-driver-other-image = ドライバー サービスが、復元されたイメージを指していません。
core-verify-driver-not-registered = 同梱のドライバー サービスが、復元後に登録されていません。
# { $error } is what Windows said, in its own words.
core-verify-driver-unreadable = ドライバー サービスを読み取れません：{ $error }
core-verify-driver-image-missing = ドライバー イメージがありません。
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-unchecked = ドライバー イメージを確認できませんでした：{ $error }
core-verify-task-matches = スケジュールされたタスクが存在し、復元されたプログラムを実行します。
core-verify-task-runs-other = スケジュールされたタスクは存在しますが、復元されたプログラムを実行しません。
core-verify-task-missing = スケジュールされたタスクがありません。
# { $error } is what Windows said, in its own words.
core-verify-task-unreadable = スケジュールされたタスクの照会に失敗しました：{ $error }
core-verify-shortcut-matches = ショートカットが存在し、リンク先のパスは復元されたリンク先と一致しています。
core-verify-shortcut-opens-other = ショートカットは存在しますが、リンク先のパスが復元されたリンク先と一致しません。
core-verify-shortcut-missing = ショートカットがありません。
# { $error } is what Windows said, in its own words.
core-verify-shortcut-unreadable = ショートカットの照会に失敗しました：{ $error }

## Warnings and errors a check gives, which the app and the installer show.

core-warning-verify-schema-unsupported = パッケージ マニフェストのスキーマはサポートされていません。
# { $error } is what went wrong, in Windows' or the program's own words.
core-warning-verify-package-file-failed = パッケージ ファイルの検証に失敗しました：{ $error }
core-warning-verify-folder-missing = 空のフォルダーがありません。
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-key-missing = レジストリ キーが存在しないか、読み取れません：{ $error }
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-value-missing = レジストリ値が存在しないか、読み取れません：{ $error }
core-warning-verify-registry-kind-missing = レジストリ値の種類がありません。
core-warning-verify-firewall-rule-missing = Windows ファイアウォールにこの規則がありません。
core-warning-verify-service-scope = サービスの検証では HKLM のサービス構成を読み取りますが、サービスは開始しません。
core-warning-verify-driver-manual = 1 つ以上のベンダー製または Windows 標準のドライバーは、WinAppMigrator では再配布されません。対象の PC で、ベンダーの署名済みドライバー パッケージを使用して再インストールしてください。
core-warning-verify-task-scope = スケジュールされたタスクの検証では、登録とコマンドの参照のみを確認します。

## The verification report written beside a package, verification_report.md. Its headings follow a "#".

core-verify-report-title = WinAppMigrator 検証レポート
# { $time } is when, as a date and time in UTC, such as 2026-11-15T10:00:00Z.
core-verify-report-verified = 検証日時（UTC）：{ $time }
# { $valid } is true or false, which are not translated.
core-verify-report-package-valid = パッケージが有効：{ $valid }
# { $valid } is true or false, which are not translated.
core-verify-report-restore-valid = 復元が有効：{ $valid }
core-verify-report-restore-not-run = 復元が有効：未実行
core-verify-report-items = 項目
# { $message } is what a check found, and { $source } the file or setting it looked at.
core-verify-report-item-from = { $message } ソース：{ $source }
core-verify-report-warnings = 警告
core-verify-report-errors = エラー
core-verify-report-none = なし。
