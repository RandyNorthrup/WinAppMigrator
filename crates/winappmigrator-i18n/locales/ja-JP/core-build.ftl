# Words the core writes for people while it builds a package from an installed app: what a package records for an
# installer to show later, errors shown in the app, the list of what goes in and what is left out, opening a disk
# image or another PC over the network, and what an installer says when the package it carries cannot be read.
# Message names start with core-.


## Scanning an app, before a package is built. Shown in the app when a scan cannot go on.

# { $path } is the app's program, such as D:\Program Files\Acme\Acme.exe, on a drive taken from another PC.
core-scan-program-not-on-the-drive = { $path } はリンクを経由しているか、接続したドライブ上にないため、読み取りませんでした。
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-not-on-this-pc = { $path } はこの PC のディスク上にないか、別のものを開く名前が付いているため、読み取りませんでした。
# { $path } is the app's program.
core-scan-program-someone-elses = { $path } は別のアカウントのフォルダーにあるか、このアカウントのものではなく、このアプリ自身の記録にもそこに属するとは記載されていないため、読み取りませんでした。
# { $app_name } is the app's name, as the list of apps calls it.
core-scan-no-executable = { $app_name } の実行可能ファイルが見つかりませんでした。ドライバー、ランタイム、アドインには、パッケージにできる独自のプログラムがない場合があります。
# { $path } is the file that was picked.
core-scan-not-a-file = スキャン対象がファイルではありません：{ $path }
# Names something left out of a package, in a list of what was left out and why. { $name } is the variable's name,
# such as ACME_LICENSE_KEY.
core-scan-environment-variable-item = 環境変数 { $name }

## Folders and parts that might also be the app's, offered in a list to tick. Each says why it is offered.

core-near-starts-with-app-name = 名前がこのアプリの名前で始まっています。
core-near-has-app-name = 名前にこのアプリの名前が含まれています。
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-starts-with-publisher = 名前が発行元の名前（{ $publisher }）で始まっています。
# { $folder } is a folder's name, such as Program Files.
core-near-hidden-part-beside = アプリの一覧に表示されない部分で、このアプリと並んで { $folder } にインストールされています。
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-hidden-part-publisher = アプリの一覧に表示されない部分で、同じ発行元（{ $publisher }）のものです。

## What a scan found, as a person chooses from it: the folders that go in, and what is left out, and why

# Why a folder cannot be left out, beside its number of files.
core-footprint-locked-app-folder = これらは常にメイン プログラムと共に含まれます
core-footprint-locked-main-program = メイン プログラムを含んでいるため、常に含まれます
core-footprint-locked-needed = メイン プログラムの起動に必要なものを含んでいます
core-footprint-locked-font = フォントと共に「Windows との連携」に含まれます
# Where a folder is. "AppData" and "ProgramData" are the names of Windows folders; keep them as they are.
core-footprint-place-roaming = プロファイル内の設定（AppData）
core-footprint-place-local = プロファイル内のこの PC 用の設定（AppData）
core-footprint-place-program-data = この PC のすべてのユーザーで共有（ProgramData）
core-footprint-place-documents = ドキュメント内
core-footprint-place-saved-games = 保存したゲーム内
core-footprint-place-public-documents = パブリックのドキュメント内
core-footprint-place-app-folder = アプリのフォルダー内
# { $folder } is a folder, such as C:\Users\Sam\AppData\Roaming.
core-footprint-place-in = { $folder } 内
# A row for the files directly in the app's folder. { $folder } is that folder's name, such as Acme.
core-footprint-files-directly-in = { $folder } 直下のファイル
# Headings over what is left out, and why each item is.
core-footprint-licenses-left-out = 「ライセンスとライセンス認証のデータ」オプションがオフのため、このインストーラーには含まれません
core-footprint-font-stays-behind = 移行されずに残ります。
core-footprint-drivers-left-out = Windows またはベンダーに属するドライバーです。再配布できないため、別の PC で再インストールしてください
core-footprint-driver-reinstall = ベンダー独自のパッケージから再インストールしてください。
# Why something else is left out along with what the person left out. { $service } is a service's name, such as AcmeSync.
core-footprint-needs-left-out = 除外された { $service } が必要です
core-footprint-told-to-use-left-out = 除外したフォルダー内のものを使用するよう設定されています
core-footprint-file-in-left-out = そのファイルが、除外したフォルダー内にあります
core-footprint-program-in-left-out = 対象のプログラムが、除外したフォルダー内にあります
core-footprint-names-left-out = 除外したフォルダーを参照しています
# Why something is left out, as the installer tells whoever installs it.
core-footprint-builder-left-it-out = このインストーラーの作成者が除外しました
# Something the person left out, by kind and name, such as "Service Acme Updater". { $kind } is the kind, such as
# Service or Scheduled Task; { $title } is its name.
core-footprint-left-out-item = { $kind }：{ $title }

## Building a package and an installer from a scan. Shown in the app when a build cannot go on.

core-package-scan-of-another-app = このスキャンは作成中のアプリとは別のアプリのものであるため、何も作成されませんでした
core-package-scan-brought-in-more = このスキャンで、今回の作成では要求されていないデータが取り込まれたため、何も作成されませんでした
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-package-program-gone = { $path } が見つからなくなったため、何も作成されませんでした。アプリをもう一度調べてください。
core-package-needs-stub = 自己解凍形式のアーカイブにはインストーラー スタブが必要です
# { $reason } is what went wrong.
core-package-staged-file-failed = ステージング フォルダー内のファイルをパッケージに含めることができませんでした：{ $reason }
# { $shortfalls } says which drives are short of room and by how much, in sentences of its own.
core-package-not-enough-room = このインストーラーを作成するための空き領域が不足しているため、何も読み取りや書き込みをしていません。{ $shortfalls }。そのドライブの空き領域を増やすか、別のドライブの出力フォルダーを選択してください。
# { $path } is a file's path, on a drive taken from another PC.
core-package-not-read-off-the-drive = { $path } はリンクを経由しているか、接続したドライブ上にないため、読み取りませんでした
# { $path } is a file's path.
core-package-not-read-not-local = { $path } はこの PC のディスク上にないため、読み取りませんでした
# { $path } is a file's path.
core-package-not-read-other-account = { $path } は別のアカウントのフォルダーにつながっているため、読み取りませんでした
# { $path } is a file's path, as the PC it came from called it.
core-package-not-on-the-drive = { $path } は接続したドライブ上にないため、読み取りませんでした
# { $path } is a file's path.
core-package-backup-copy-no-parent = 親ディレクトリのないファイルはバックアップ コピーできません：{ $path }
# { $path } is a file's path.
core-package-backup-copy-destination-no-parent = バックアップ コピー先に親ディレクトリがありません：{ $path }
# robocopy is the name of a Windows command; keep it as it is. { $code } is how it ended, such as Some(8).
core-package-robocopy-failed = robocopy によるバックアップ モードのコピーに失敗しました（終了コード { $code }）
# Stands for a folder where a file's name would be, as in "I/O failed for temporary package folder: …".
core-package-temporary-folder = 一時パッケージ フォルダー
# Stands for the installer being written where a file's name would be, as in "I/O failed for the installer: …".
core-package-the-installer = インストーラー
# { $path } is a file's path, or its name in the package. { $error } is what went wrong with it.
core-package-path-error = { $path }：{ $error }
# { $name } is a file's name in the package, such as app/bin/App.exe.
core-package-same-place = パッケージ内の同じ場所に別のファイルがあります：{ $name }
# { $name } is a file's name in the package.
core-package-changed-in-protected-folder = パッケージの作成中に、保護されたフォルダー内の { $name } が変更されました
core-package-worker-died = ファイルを圧縮するスレッドが予期せず停止したため、インストーラーは書き込まれませんでした
# { $path } is a file's path.
core-package-link-not-read = { $path } はリンクのため、読み取りませんでした
# { $path } is a file's path. { $error } is what went wrong.
core-package-not-compressed = { $path } を圧縮できませんでした：{ $error }
# { $path } is a file's path. { $reason } is what went wrong.
core-package-not-read-back = { $path } を読み戻せませんでした：{ $reason }

## Where an app's files are, and where an installer may write them. Shown when a path is refused.

# { $path } is a folder, such as C:\Program Files\Acme.
core-discovery-root-not-full-path = 書き込み先のフォルダーは完全なパスである必要があります：{ $path }
# { $path } is a folder's path.
core-discovery-root-not-a-folder = 書き込み先のフォルダーがフォルダーではありません：{ $path }
# { $path } is a file's path.
core-discovery-escapes-root = パスが想定したルートの外を指しています：{ $path }
# { $path } is the link's path.
core-discovery-refusing-to-write-through-link = リンクを経由した書き込みを拒否しました：{ $path }
# { $path } is the link's path.
core-discovery-refusing-link = リンクを経由したアクセスを拒否しました：{ $path }
# { $error } is what Windows said went wrong.
core-discovery-refusing-link-because = リンクを経由したアクセスを拒否しました：{ $error }
# { $path } is the link's path.
core-discovery-link-to-somewhere-else = { $path } は別の場所へのリンクで、リンク先がほかのユーザーによって変更される可能性があります。リンク先の場所を使用してください
# { $path } is a folder's path.
core-discovery-no-parent-folder = { $path } に親フォルダーがありません
# { $path } is the install location as it was given. Keep the quotes, or use your language's own.
core-discovery-install-location-not-full-path = インストール先は完全なパスである必要があります：「{ $path }」
# { $path } is the install location.
core-discovery-install-location-a-link = インストール先が別の場所へのリンクです：{ $path }
# { $path } is a file's path.
core-discovery-device-name = パスに Windows のデバイス用に予約された名前が使用されています：{ $path }
# { $path } is a file's path.
core-discovery-device-name-on-older-windows = { $path } の名前は、このバージョンの Windows ではファイルではなくデバイスとして開かれるため、何も書き込みませんでした。Windows 11 にはインストールできます
# { $path } is a file's path.
core-discovery-unsafe-path = 安全でないパス：{ $path }
# { $path } is a file's path.
core-discovery-no-package-path = ファイルにパッケージ内の相対パスがありません：{ $path }

## Writing an installer, and reading the package inside one. Most of these are shown by an installer, on the PC it
## runs on, when it cannot read what it carries. "The package" is the app's files and settings that an installer
## carries; "the stub" is the program part of an installer, before the package; "the data file" is the .wamdata file
## that holds the package of a very large installer, beside it.

# Stands for the package where a file's name would be, as in "I/O failed for installer package: …".
core-archive-installer-package = インストーラー パッケージ
# Stands for the installer program built into WinAppMigrator, where a file's name would be.
core-archive-embedded-stub = 埋め込みのインストーラー スタブ
# { $index } is the number of a file in the package. { $reason } is what went wrong.
core-archive-entry-unreadable = パッケージのエントリ { $index } を読み取れませんでした：{ $reason }
# { $name } is a file's name inside the package, such as app/bin/App.exe.
core-archive-entry-unsafe-path = zip エントリのパスが安全ではありません：{ $name }
# { $name } is a file's name inside the package.
core-archive-holds-more = { $name } には記載されているより多くのデータが含まれています
# { $name } is a file's name inside the package.
core-archive-name-too-long = この名前は zip には長すぎます：{ $name }
core-archive-changed-while-copied = スタブの後ろに配置している間にパッケージが変更されたため、インストーラーは書き込まれませんでした
core-archive-stub-not-a-program = このインストーラーのパッケージより前の部分がプログラムではありません
core-archive-damaged = このインストーラーは破損しているか、作成後に変更されたため、パッケージから何も読み取りませんでした。作成された場所からもう一度コピーしてください。
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-missing = このインストーラーは、パッケージを同じフォルダーにある { $name } という名前のファイルに保存していますが、そのファイルがありません。2 つのファイルは一緒にしておく必要があります。インストーラーが作成された場所から { $name } をここにコピーしてから、もう一度実行してください。
# { $name } is the data file's name, such as App_installer.wamdata. { $error } is what Windows said went wrong.
core-archive-data-file-unopened = このインストーラーのデータ ファイル { $name } を開けませんでした：{ $error }。2 つのファイルは、この PC から読み取れるフォルダーに一緒に置いておく必要があります。
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-a-link = { $name } はリンクまたはフォルダーです。このインストーラーのデータ ファイルは、インストーラーのすぐ横にある独立したファイルである必要があるため、何も読み取りませんでした。実際のファイルをここにコピーしてください。
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-wrong = { $name } はこのインストーラーのデータ ファイルではないか、破損しているか、作成後に変更されたため、何も読み取りませんでした。作成された場所から両方のファイルをもう一度コピーしてください。
core-archive-stub-signed = インストーラー スタブが署名されています。スタブは署名なしのままにする必要があるため、代わりに完成したインストーラーに署名してください
# { $size } is a size, such as 4.2 GB.
core-archive-too-big-to-run = インストーラーのサイズは { $size } になりますが、Windows では 4 GB 以上の実行可能ファイルを実行できないため、何も書き込みませんでした
core-archive-outside-signature = インストーラーの署名の範囲外にデータがあり、改ざんされている可能性があります
core-archive-footer-length = 自己解凍形式アーカイブのフッターの長さが無効です
core-archive-seek-outside = パッケージの範囲外へのシーク
# { $path } is a file's path.
core-archive-unsafe-package-path = 安全でないパッケージ パス：{ $path }
# { $path } is a file's path.
core-archive-empty-package-path = 空のパッケージ パス：{ $path }

## Opening a disk image of another PC (a .vhd or .vhdx file) to read apps from it

# { $path } is the disk image file, such as D:\Backups\OldPC.vhdx.
core-image-not-attachable = { $path } は、Windows でアタッチできるディスク イメージ（.vhd または .vhdx）ではありません
core-image-needs-administrator = ディスク イメージをアタッチするには管理者権限が必要です
# { $path } is the disk image file the person picked.
core-image-not-a-file = { $path } はファイルではありません
# { $path } is the disk image file. "Edit Disk" is the name of a Hyper-V Manager command.
core-image-differencing = { $path } は差分ディスクです。差分ディスクには作成元のディスクからの変更しか含まれず、Windows は作成元のディスクも、記録されている場所から開きます。まず Hyper-V の「ディスクの編集」でそのディスクにマージしてから、マージ先のディスクを選択してください
# { $path } is the disk image file.
core-image-unreadable = { $path } は読み取り可能な .vhd または .vhdx ではありません
# ProgramData is the name of a Windows folder; keep it as it is.
core-image-no-program-data = この PC の ProgramData フォルダーが見つかりませんでした
# The reason given in core-image-could-not-attach when Windows already has the image open.
core-image-already-attached = Windows で既にアタッチされており、アタッチしたものが書き込み中である可能性があります。まずデタッチするか、そのドライブを接続したドライブとして選択してください
# { $path } is the disk image file. { $reason } is why, such as core-image-already-attached or what Windows said.
core-image-could-not-attach = { $path } をアタッチできませんでした：{ $reason }
# { $path } is the disk image file.
core-image-no-windows-volume = { $path } には、読み取り可能な Windows のボリュームがありません。BitLocker で暗号化されたボリュームは、先にロックを解除する必要があります。
# Given as the reason when Windows failed without saying anything.
core-image-windows-did-not-say = Windows から理由は示されませんでした

## Reading another PC over the network, through the client running on it. Shown as why something on that PC
## could not be read. "The client" is WinAppMigrator Client, the small program on the other PC.

core-remote-too-slow = 別の PC からの応答に時間がかかりすぎました
core-remote-closed = 別の PC との接続が閉じられています
core-remote-failed = 別の PC との接続に失敗しました
core-remote-answered-something-else = クライアントから想定外の応答がありました
# { $name } is the PC's name or address as the person typed it, such as OLDPC or 192.168.1.20.
core-remote-not-found = ネットワーク上で { $name } が見つかりませんでした
core-remote-did-not-say-what-it-is = クライアントが自身の種類を示しませんでした
core-remote-ordinary-names-only = 照会できるのは、別の PC のルートより下にある、通常の名前のパスだけです
core-remote-more-but-none = 別の PC は、フォルダーにまだ項目があると示しましたが、何も送信しませんでした
# { $count } is the most things one folder may hold, such as 250,000.
core-remote-too-many-listed = 別の PC が 1 つのフォルダーに { $count } を超える項目を列挙しました
core-remote-sent-too-much = 別の PC が、要求されたより多くのファイル データを送信しました
core-remote-name-not-text = テキストではない名前は照会できません
core-remote-below-root-only = 照会できるのは、別の PC のルートより下にあるパスだけです
core-remote-file-ended-early = 別の PC のファイルが途中で終わりました
core-remote-link-not-followed = 別の PC 上のリンクはたどりません
core-remote-only-files-opened = 別の PC で開けるのはファイルだけです
core-remote-listing-said-nothing = 一覧にその情報がありませんでした
core-remote-change-time-unknown = 変更日時が不明です
core-remote-seek-before-start = 先頭より前へのシーク
# { $size } is how many bytes a file is; { $limit } is the most that were expected. Both are numbers.
core-remote-too-large =
    { $size ->
       *[other] サイズが { $size } バイトありますが、想定は最大 { $limit } バイトでした
    }

## Checking that a download is signed by Microsoft. Each is given as the reason a check failed.

# { $error } is what Windows said went wrong.
core-signature-no-powershell = PowerShell を開始できませんでした：{ $error }
# { $error } is what went wrong reading PowerShell's answer.
core-signature-unreadable = 署名を読み取れませんでした：{ $error }
# { $signer } is who signed the file, such as Contoso Ltd.
core-signature-not-microsoft = Microsoft ではなく { $signer } によって署名されています
core-signature-not-microsoft-unnamed = Microsoft ではなく、別の署名者によって署名されています
# { $status } is Windows' own word for the signature's state, such as HashMismatch or NotSigned, which is not translated.
core-signature-status = 署名の状態は { $status } です

## Warnings a package records when it is built from an app, which an installer shows later, maybe on another PC, in
## that PC's language. "This app" is the app being packaged.

core-warning-build-program-needs-unreadable = このプログラムが Windows に必要とするものを読み取れなかったため、別の PC で実行できるかどうかは確認されません。
core-warning-build-external-manifest = 外部アプリケーション マニフェストが検出されました。サイドバイサイドの依存関係の解釈は後で行われます。
core-warning-build-non-exe = 選択したファイルの拡張子は .exe ではありません。PE のスキャンを続行します。
core-warning-build-stand-in-program = 通常のアプリケーションの実行可能ファイルが見つからなかったため、このファイルで代用します。アーキテクチャと依存関係の結果はこのファイルについてのものであり、アプリケーションには当てはまらない場合があります。
core-warning-build-other-apps-files-unknown = Windows Installer が保持しているほかのアプリの情報をすべて読み取ることができなかったため、このアプリの記録にあるフォルダーにほかのアプリのファイルが含まれていても、ここではそれを示せません。
core-warning-build-app-folder-has-another-apps-file = Windows Installer によると、このアプリのプログラムがあるフォルダーに別のアプリのファイルがあります。そのフォルダーはすべて含まれるため、そのファイルも含まれます。
# "App Paths" is the name of a Windows registry key; keep it as it is.
core-warning-build-app-paths-folder-left-out = このアプリの App Paths エントリが、自身のものではないフォルダーを参照しているため、そのフォルダーは含まれません。そこにあるものがない PC では、アプリが起動しない可能性があります。
core-warning-build-uninstaller-folder-left-out = Windows Installer によると別のアプリのファイルがあるため、このアプリのアンインストーラーがあるフォルダーは除外されました。
# { $why } is never shown: it chooses the reason. Translate each reason as the end of the sentence.
core-warning-build-recorded-folder-left-out =
    このアプリの記録にあるものが参照しているフォルダーは、{ $why ->
        [unreadable] パスがドライブの外を指しているか、リンクを経由していて、安全に読み取れない
        [missing] 存在しない
        [link] リンクを経由している
        [short-name] 実際の名前で記載されていない
        [another-account] 別のアカウントのフォルダーにある
        [one-account] このアプリが 1 つのアカウント用にインストールされていて、そのアカウントのものではない
        [shared] 共有されているものか、Windows の一部を含んでいる
        [generic-name] 名前から所有者がわからない
        [publisher] 発行元の名前だけが付いていて、その発行元のほかの製品を含んでいる可能性がある
        [holds-the-app] このアプリ自身のフォルダーを含んでいて、ほかのものも含んでいる可能性がある
        [another-apps-name] 別のアプリの名前が付いている
        [another-apps-record] インストールの記録が別のアプリのものである
        [in-another-apps-folder] 別のアプリのフォルダー内にある
       *[another-apps] 別のアプリのものである
    }ため、除外されました。
core-warning-build-app-folder-holds-another-app = この PC 上の別のアプリが、このアプリのフォルダー内にあると記録されています。そのフォルダーはすべて含まれるため、そのアプリも一緒に含まれます。
# "Program Files (x86)" is the name of a Windows folder; keep it as Windows calls it.
core-warning-build-from-32-bit-windows = このアプリは 32 ビット版 Windows 上にありました。レジストリ キーは 64 ビット版 Windows で 32 ビット アプリのキーが置かれる形で配置され、インストーラーではインストール先として Program Files (x86) が示されます。
# { $dll } is a file's name, such as MSVCP140D.dll.
core-warning-build-debug-runtime = プログラムに、Visual C++ ランタイムのデバッグ ビルドである { $dll } が必要です。Microsoft はこれらのインストーラーを提供しておらず、再配布も許可していない場合があるため、Visual Studio がない PC では実行できません。
# { $program } is a program's file name, such as Tool.exe.
core-warning-build-single-file-program = { $program } は 1 つのファイルにまとめられた .NET プログラムで、必要な .NET の種類が示されていないため、.NET のインストールは予定されていません。.NET が必要な場合、それがない PC では起動しません。
# { $count } is how many of the app's programs.
core-warning-build-programs-not-inspected =
    { $count ->
       *[other] アプリのプログラムの数が多いか、サイズが大きいため、{ $count } 個のプログラムについて、必要なランタイムを確認していません。
    }
# { $runtime } is a runtime's name, such as Java Runtime Environment.
core-warning-build-runtime-no-installer = このアプリには { $runtime } が必要ですが、ここにはそのインストーラーがないため、何も実行されません。別の PC に手動でインストールしてください。
# { $why } is why something was left out, such as "its name says what it holds".
core-warning-build-left-out-of-package = パッケージから除外されました：{ $why }。
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-missing-dependency = 依存関係を解決できませんでした：{ $name }
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-dependency-only-here = { $name } はこの PC の検索パスからしか見つからず、アプリの一部ではないため、含まれません。これがない PC では、アプリが起動しない可能性があります。
core-warning-build-driver-detected = ドライバーまたはカーネル コンポーネントが検出されました。アプリが所有するドライバーは同梱されて再登録されますが、ベンダー製または Windows 標準のドライバーは再配布されないため、ベンダーの署名済みパッケージから再インストールする必要があります。
core-warning-build-near-no-longer-offered = このアプリの一部である可能性があるものの一覧で選択した項目が、一覧に表示されなくなったため、追加されませんでした。一覧をもう一度確認してください。
# { $app_name } is the name of a part of the app, as the list of apps calls it. { $why } is never shown: it chooses the
# reason. Translate each reason as the end of the sentence.
core-warning-build-near-part-left-out =
    { $app_name } が自身のものとしている部分の一部は、{ $why ->
        [unreadable] パスがドライブの外を指しているか、リンクを経由していて、安全に読み取れない
        [missing] 存在しない
        [link] リンクを経由している
        [short-name] 実際の名前で記載されていない
        [another-account] 別のアカウントのフォルダーにある
        [one-account] このアプリが 1 つのアカウント用にインストールされていて、そのアカウントのものではない
        [shared] 共有されているものか、Windows の一部を含んでいる
        [generic-name] 名前から所有者がわからない
        [publisher] 発行元の名前だけが付いていて、その発行元のほかの製品を含んでいる可能性がある
        [holds-the-app] このアプリ自身のフォルダーを含んでいて、ほかのものも含んでいる可能性がある
        [another-apps-name] 別のアプリの名前が付いている
        [another-apps-record] インストールの記録が別のアプリのものである
        [in-another-apps-folder] 別のアプリのフォルダー内にある
       *[another-apps] 別のアプリのものである
    }ため、除外されました。
# { $app_name } is the name of a part of the app, as the list of apps calls it.
core-warning-build-near-keys-left-out = { $app_name } のインストールの記録に記載されているレジストリ キーは、その記録に別のアプリのものも含まれているため、除外されました。
# "Windows keeps what an app wrote for one account" is Windows' VirtualStore folder.
core-warning-build-virtual-store-program = Windows がアカウントごとにアプリの書き込み内容を保存するフォルダーに、プログラムがありました。Windows がそこにプログラムを置くことはないため、ほかの何かが置いたものと判断し、除外しました。
# { $path } is a file or folder, such as C:\Users\Sam\AppData\Roaming\Acme.
core-warning-build-file-left-out = { $path } は除外されました：接続したドライブ上のリンクを経由しているか、データを含める対象とは別のアカウントのものです。
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as D:\Sam\Documents.
core-warning-build-user-folder-not-on-drive = { $user } の { $folder } フォルダーは { $path } にあり、このドライブ上にないため、その内容は含まれていません。
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as \\server\home\Sam\Documents.
core-warning-build-user-folder-not-on-this-pc = { $user } の { $folder } フォルダーは { $path } にあり、この PC のディスク上にないため、その内容は含まれていません。
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-base-unreadable = { $path } を読み取れませんでした（{ $error }）。そこに保存されているアプリケーション データは含まれていません。
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-folder-unreadable = { $path } を読み取れませんでした（{ $error }）。その中にあるアプリケーション データが欠けている可能性があります。
# { $error } is what went wrong.
core-warning-build-entry-unreadable = ファイル システムのエントリを読み取れませんでした：{ $error }
# { $count } is how many links. { $names } lists some of them. { $more } is how many more there are, which are not
# named; when it is 0, the sentence ends after the list.
core-warning-build-links-not-followed =
    { $count ->
       *[other] このフォルダー内の { $count } 個のリンク（または別のものを開く名前）をたどらなかったため、そのリンク先はインストーラーに含まれていません：{ $names }
    }{ $more ->
        [0] 。
       *[other] 、ほか { $more } 件。
    }
# { $count } is how many folders. { $depth } is how many levels down a folder can be. { $names } lists some of them.
# { $more } is how many more there are, which are not named; when it is 0, the sentence ends after the list.
core-warning-build-folders-too-deep =
    { $count ->
       *[other] { $count } 個のフォルダーが { $depth } 階層より深い場所にあるため、その内容はインストーラーに含まれていません：{ $names }
    }{ $more ->
        [0] 。
       *[other] 、ほか { $more } 件。
    }
# { $error } is what went wrong.
core-warning-build-metadata-unreadable = ファイルのメタデータを読み取れませんでした：{ $error }
core-warning-build-reserved-name = このファイルの名前は Windows のデバイス用に予約されているため、コピーされませんでした。
# { $error } is what went wrong.
core-warning-build-hash-failed = ファイルのハッシュを計算できませんでした：{ $error }
core-warning-build-cloud-placeholder = オンライン専用のクラウド ファイル（OneDrive のファイル オンデマンドなど）には、ローカル データがありません。バックアップの前に、「常にこのデバイス上に保持する」を選択するか、ダウンロードしてください。報告のみ行い、コピーはしません。
# { $file_name } is a driver's file name, such as acmeflt.sys.
core-warning-build-driver-file = ドライバー/カーネル ファイル「{ $file_name }」は、通常のファイルとしてはコピーされません。このアプリの登録済みドライバー サービスで使用されている場合は、インストーラーに同梱され、ドライバーが自動的に再登録されます。それ以外の場合は、ベンダーの署名済みドライバー パッケージで再インストールしてください。
core-warning-build-file-gone = アプリを調べたときには存在していましたが、現在はないため、インストーラーには含まれていません。
# { $error } is what went wrong.
core-warning-build-file-copy-failed = ファイルをパッケージにコピーできませんでした：{ $error }
# { $count } is how many files. { $names } lists them.
core-warning-build-not-looked-through =
    { $count ->
       *[other] このパッケージには、作成元に関する情報が含まれていないかを確認できなかったファイルがあります：{ $names }。これらの内容に作成元に関する情報が含まれている可能性があります。
    }
# { $count } is the most empty folders an installer makes, such as 10,000.
core-warning-build-empty-folders-truncated = このアプリには { $count } 個を超える空のフォルダーがあり、再作成されるのはその数までです。
core-warning-build-credential-by-now = パッケージから除外されました：現時点で資格情報のように見えます。
core-warning-build-no-package-path = コピー可能なファイルにパッケージ内の相対パスがありませんでした。
# { $file_name } is the runtime installer's file name, such as vc_redist.x64.exe. { $error } is what went wrong.
core-warning-build-prerequisite-path-invalid = 同梱の前提条件 { $file_name } を配置できませんでした：{ $error }
# { $runtime } is a runtime's name, such as Microsoft Visual C++ 2015+ Redistributable (x64). { $error } is what went
# wrong. winget is the name of a Windows command; keep it as it is.
core-warning-build-prerequisite-download-failed = { $runtime } をダウンロードできませんでした（{ $error }）。インストーラーは対象の PC で winget を試します。
# { $runtime } is a runtime's name. { $reason } is why the check failed, such as "it is signed by Contoso Ltd, not
# Microsoft".
core-warning-build-prerequisite-signature-failed = { $runtime } のダウンロードが署名チェックに失敗したため、同梱されませんでした：{ $reason }。インストーラーは対象の PC で winget を試します。
# { $runtime } is a runtime's name. { $error } is what went wrong.
core-warning-build-prerequisite-hash-failed = { $runtime } をダウンロードしましたが、ハッシュを計算できませんでした（{ $error }）。インストーラーは代わりに対象の PC で winget を試します。
# { $service } is a driver's service name, such as AcmeFlt. { $error } is what went wrong.
core-warning-build-driver-copy-failed = 「{ $service }」のドライバー イメージを同梱できませんでした：{ $error }
core-warning-build-license-not-copied = ライセンス データをコピーできませんでした（存在しないか、読み取れません）。メタデータとしてのみ記録されました。
core-warning-build-license-file-credential = ライセンス ファイルが資格情報のように見えるため、除外されました。
# { $error } is what went wrong.
core-warning-build-license-store-walk-failed = ライセンス ストアの一部を列挙できませんでした：{ $error }
core-warning-build-license-store-credential = ライセンス ストア内のファイルが資格情報のように見えるため、除外されました。
# { $error } is what went wrong.
core-warning-build-license-store-file-skipped = ライセンス ストアのファイルをスキップしました：{ $error }
