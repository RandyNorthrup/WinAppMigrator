# Words the core writes for people as it installs an app from a package: why an install will not start, the list of
# what it did (or for a preview, what it would do) that the installer window shows, its warnings, the record it keeps of
# each install so that it can be undone, and the core's errors. Message names start with core-.
#
# The list of what an install did is short and plain, one line for each thing, as the installer window lists them:
# "copied", "restored", "would copy". Words such as --force and --apply are command-line options, and LocalService,
# SYSTEM, ProgramData, and %AppData% are as Windows names them: keep them as they are.


## Errors: what went wrong, when the core cannot go on. Each is shown on its own, or after words that say what was being
## done.

core-error-list-changed = 同意した後にこのインストールの実行内容が変わったため、何もインストールされませんでした
# { $path } is a file or folder, such as C:\Program Files\App\app.exe, or words that name one, such as "the installer".
# { $error } is what Windows said went wrong.
core-error-io = { $path } の I/O に失敗しました：{ $error }
# { $detail } is what went wrong.
core-error-registry = レジストリの読み取りに失敗しました：{ $detail }
# { $path } is a program file. { $detail } is what was wrong with it.
core-error-pe = { $path } の PE 解析に失敗しました：{ $detail }
# { $detail } is what was wrong, already in words.
core-error-invalid-input = 入力が無効です：{ $detail }
# { $detail } is what went wrong.
core-error-serialization = レポートのシリアル化に失敗しました：{ $detail }
# { $detail } is what went wrong.
core-error-archive = アーカイブ操作に失敗しました：{ $detail }
# { $detail } is what went wrong.
core-error-network = ネットワーク操作に失敗しました：{ $detail }
# { $detail } is what went wrong.
core-error-time-format = 時刻の書式設定に失敗しました：{ $detail }
core-error-cancelled = キャンセルされました
# What is said when part of the program stopped unexpectedly and gave no reason.
core-error-panic-no-reason = 理由は示されませんでした

## The record of each install, kept so that it can be undone.

# { $kind } is a number that says what type of registry value it is.
core-journal-unknown-value-type = 不明なレジストリ値の種類 { $kind }
core-journal-not-elevated = 管理者として実行されていません
core-journal-needs-admin = インストールには管理者権限が必要です。ほかのユーザーが変更できない場所に記録を保存するためです
# { $path } is a folder.
core-journal-not-an-install-folder = これはインストールのフォルダーではありません：{ $path }
# { $path } is the file the record is kept in.
core-journal-not-written = このインストールの記録を書き込めませんでした：{ $path }
# { $path } is a folder.
core-journal-path-escapes-root = パスが想定したルートの外を指しています：{ $path }
# { $path } is a folder.
core-journal-nothing-exists = この PC にはいずれも存在しません：{ $path }
core-journal-busy = 別のインストールまたはアンインストールが実行中です。完了するまで待ってから、もう一度お試しください
# Said of a file that was to get back the permissions it had.
core-journal-no-folder = フォルダーがありません
# Said of the permissions a file had, which were kept to give back to it.
core-journal-permissions-unreadable = 保存していた内容を読み取れませんでした
# { $folder } is the folder of one install's record.
core-journal-unreadable-empty = { $folder } にあるインストールの記録は使用できません：記録が空です
# { $folder } is the folder of one install's record.
core-journal-unreadable-no-header = { $folder } にあるインストールの記録は使用できません：記録の対象を示す情報で始まっていません
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-version = { $folder } にあるインストールの記録は使用できません：このプログラムの別のバージョンで書き込まれています
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-install = { $folder } にあるインストールの記録は使用できません：別のインストールの記録であると記載されています
# { $id } is an install's id, such as wam-1a2b3c.
core-journal-not-an-install-id = これはインストールの ID ではありません：{ $id }

## Why an install will not start, or stopped before it changed anything.

core-restore-apply-and-dry-run = --apply と --dry-run を同時に指定することはできません
# { $path } is the folder that was chosen to install in.
core-restore-location-not-full-path = インストール先は完全なパスである必要があります：「{ $path }」
# { $problem } says what is wrong with the folder, already in words. { $path } is the folder.
core-restore-location-unusable = そのインストール先は使用できません：{ $problem }：「{ $path }」
core-restore-needs-admin-plan = 復元計画に、管理者のみが行えるレジストリ、サービス、スケジュールされたタスク、ショートカット、または保護された場所への変更が含まれています
# { $path } is the folder that was chosen to install in.
core-restore-needs-admin-target = 対象のルートには管理者権限が必要です：{ $path }
core-restore-needs-admin-user-data = ProgramData などの保護された場所にユーザー データを復元するには、管理者権限が必要です
# { $error } is why the install stopped. { $failed } is what could not be put back, as sentences of their own.
core-restore-not-all-put-back = { $error }。実行済みの内容をすべて元に戻すことはできませんでした：{ $failed }
# { $error } is what went wrong.
core-restore-nowhere-to-keep-record = このインストールを元に戻すための記録を安全に保存できる場所がないため、何もインストールされませんでした：{ $error }
# { $path } is a file the app would have put there.
core-restore-already-exists = { $path } は既に存在するため、何もインストールされませんでした。置き換えるには --force を使用してください
# { $root } is a folder the app has files for. { $link } is that folder, or one it is in.
core-restore-root-under-link = このアプリには { $root } に復元するものがありますが、{ $link } は別の場所へのリンクで、ほかのユーザーが変更できる可能性があるため、何もインストールされませんでした。そこに配置されるものがユーザー データまたはライセンス データの場合は、ユーザー データとライセンス データを含めずにインストールしてください。スタートアップ フォルダー内のショートカットの場合は、そのフォルダーを元の場所に戻してください
core-restore-hash-check-failed = パッケージがハッシュ チェックに失敗したため、何もインストールされませんでした
# { $detail } is the first thing found wrong, already in words.
core-restore-hash-check-failed-because = パッケージがハッシュ チェックに失敗したため、何もインストールされませんでした：{ $detail }
# { $path } is a file or folder.
core-restore-refuse-records-folder = インストールの記録を保存する場所への書き込みを拒否しました：{ $path }
# { $path } is a file or folder.
core-restore-refuse-outside = インストール先と既知のデータ フォルダー以外への書き込みを拒否しました：{ $path }
# { $path } is a file.
core-restore-no-parent-folder = 書き込み先に親フォルダーがありません：{ $path }
# { $path } is where a file was to go.
core-restore-folder-in-the-way = このファイルを配置する場所にフォルダーがあります：{ $path }
# { $path } is a file in the package.
core-restore-file-changed = { $path } はパッケージに含まれている内容と異なるため、インストールされませんでした
# { $path } is a file in the package.
core-restore-source-is-link = { $path } はリンクであり、リンク先はたどりません
# { $path } is a file in the package.
core-restore-source-became-link = { $path } は、開いている間にリンクに変わりました
# Why a service did not start, after "did not start:".
core-restore-service-stopped-by-itself = 自動的に再び停止しました
# Why part of the app was left out, after "was left out:".
core-restore-com-class-someone-elses = この COM クラスは、別のものとして既にこの PC に登録されています

## What an install did, one line for each thing, or for a preview, what it would do. { $source } is a file in the
## package, and { $destination } and { $path } are where it goes on this PC. { $key } is a registry key, such as
## HKLM\Software\App.

# { $package_valid } is true or false, and { $restore_valid } is true, false, or not_run: keep them as they are.
core-restore-verification-result = 復元後の検証 package_valid={ $package_valid } restore_valid={ $restore_valid }
# { $name } is a service's name.
core-restore-started-service = サービスを開始しました：{ $name }
core-restore-would-copy = コピー予定：{ $source } -> { $destination }
core-restore-copied = コピーしました：{ $source } -> { $destination }
core-restore-would-create-empty-folder = 空のフォルダーを作成予定：{ $path }
core-restore-created-empty-folder = 空のフォルダーを作成しました：{ $path }
# { $value } is the name of a value in the key.
core-restore-would-restore-registry-value = レジストリを復元予定：{ $key } { $value }
core-restore-would-restore-registry-key = レジストリを復元予定：{ $key }（キー）
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-would-add-firewall-rule = ファイアウォール規則を追加予定：{ $name }（{ $program } 用）
# { $name } is a firewall rule's name.
core-restore-firewall-rule-already-there = ファイアウォール規則 { $name } は既に存在します
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-added-firewall-rule = ファイアウォール規則を追加しました：{ $name }（{ $program } 用）
# { $name } is a service's name, { $command } what it runs, and { $account } the account it runs as, such as LocalService.
core-restore-would-restore-service = サービスを復元予定：{ $name } -> { $command }（{ $account } として実行）
# { $name } is a runtime, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-restore-prerequisite-present = 前提条件は既に存在します：{ $name }
# { $name } is a runtime that Windows has as a Windows feature.
core-restore-would-install-prerequisite-feature = 前提条件をインストール予定：{ $name }（Windows の機能）
# { $name } is a runtime whose installer is in the package.
core-restore-would-install-prerequisite-bundled = 前提条件をインストール予定：{ $name }（同梱のインストーラー）
# { $name } is a runtime that would come through winget, which is a program's name.
core-restore-would-install-prerequisite-winget = 前提条件をインストール予定：{ $name }（winget）
# { $name } is a runtime.
core-restore-installed-prerequisite = 前提条件をインストールしました：{ $name }
# { $name } is a runtime.
core-restore-installed-prerequisite-reboot = 前提条件をインストールしました：{ $name }（再起動が必要）
core-restore-would-restore-license = ライセンスを復元予定：{ $source } -> { $destination }
core-restore-restored-license = ライセンスを復元しました：{ $source } -> { $destination }
core-restore-would-restore-license-registry = ライセンスのレジストリを復元予定：{ $key }
# { $name } is a driver's name.
core-restore-driver-not-restored = ドライバーは自動的には復元されません：{ $name }
# { $name } is a driver's name.
core-restore-would-install-driver = ドライバーをインストール予定：{ $name } -> { $destination }
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-would-restore-task = スケジュールされたタスクを復元予定：{ $name }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-startup-shortcut = スタートアップのショートカットを復元予定：{ $shortcut } -> { $target }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-shortcut = ショートカットを復元予定：{ $shortcut } -> { $target }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-would-restore-variable =
    { $scope ->
        [user] ユーザー環境変数を復元予定：{ $name }
       *[machine] システム環境変数を復元予定：{ $name }
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-left-variable =
    { $scope ->
        [user] ユーザー環境変数 { $name } は変更しませんでした：このアプリに属する部分はありません
       *[machine] システム環境変数 { $name } は変更しませんでした：このアプリに属する部分はありません
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-restored-variable =
    { $scope ->
        [user] ユーザー環境変数を復元しました：{ $name }
       *[machine] システム環境変数を復元しました：{ $name }
    }
core-restore-set-uninstaller = { $key } のアンインストーラーを設定しました
core-restore-placed-driver-image = ドライバー イメージを配置しました：{ $source } -> { $destination }
# { $name } is a driver's name.
core-restore-configured-driver = ドライバーを構成しました：{ $name }
# { $name } is a driver's name.
core-restore-created-driver = ドライバーを作成しました：{ $name }
# { $name } is a driver's name.
core-restore-started-driver = ドライバーを開始しました：{ $name }
core-restore-ensured-registry-key = レジストリ キーを用意しました：{ $key }
# { $value } is the name of a value in the key.
core-restore-restored-registry-value = レジストリを復元しました：{ $key } { $value }
# "(Default)" is how Windows names the value a key has with no name: write it as Registry Editor does.
core-restore-restored-registry-default = レジストリを復元しました：{ $key } (既定)
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-configured-service = サービスを構成しました：{ $name }（{ $account } として実行）
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-created-service = サービスを作成しました：{ $name }（{ $account } として実行）
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-restored-task = スケジュールされたタスクを復元しました：{ $name }
core-restore-restored-shortcut = ショートカットを復元しました：{ $path }

## Warnings an install gives, and why it failed. They are shown after it, on the PC it ran on.

core-warning-restore-earlier-not-trusted = 完了しなかったインストールの記録は、ほかのユーザーが変更できるため、そのままにしました。そのインストールによる変更は残っています。
# { $app_name } is the app's name. { $count } is how many changes were put back.
core-warning-restore-earlier-undone =
    { $count ->
       *[other] 以前の { $app_name } のインストールが完了していなかったため、最初に元に戻しました。{ $count } 件の変更を元に戻しました。
    }
# { $app_name } is the app's name.
core-warning-restore-earlier-never-finished = 以前の { $app_name } のインストールが完了していません。インストールを実行すると、最初にそれを元に戻します。
core-warning-restore-user-data-real-locations = ユーザー データは、対象のルートではなく、この PC の実際の %AppData%/%ProgramData% の場所に復元されます。
# { $error } is what went wrong.
core-warning-restore-commit-failed = このインストールの記録を完了できませんでした：{ $error }
# { $name } is a service's name.
core-warning-restore-service-still-starting = サービス { $name } の開始を要求しましたが、まだ開始処理が完了していません。この後開始される可能性があります。また、次回この PC を起動したときにも自動的に開始されます。
# { $name } is a service's name. { $reason } is why it did not start, as Windows said it.
core-warning-restore-service-not-started = サービス { $name } はインストールされましたが、開始されませんでした：{ $reason }。開始されるまで、アプリが動作しない可能性があります。次回この PC を起動したときに、Windows が再試行します。
# { $name } is a service's name. { $reason } is why it could not be stopped.
core-warning-restore-service-not-stopped = サービス { $name } を最初に停止できませんでした：{ $reason }。次回この PC を起動するまで、古いプログラムが実行され続ける可能性があります。
core-warning-restore-install-cancelled = インストールはキャンセルされました。
core-warning-restore-preview-cancelled = プレビューはキャンセルされました。
# { $item } is what was left out, such as a file or a service's name. { $reason } is why, already in words.
core-warning-restore-left-out = { $item } は除外されました：{ $reason }。
# { $error } is what went wrong.
core-warning-restore-verification-failed = 復元後の検証を実行できませんでした：{ $error }
# { $runner } says what relies on the file: a service, a scheduled task, a firewall rule, a shortcut, or a registry value.
# { $path } is the file. { $name } is the name of what relies on it. { $refusal } is why the file was not replaced,
# already in words.
core-warning-restore-program-would-run =
    { $runner ->
        [service] { $path } はこのパッケージで置き換えてよいものではなく、サービス { $name } がこれに依存しているため、何もインストールされませんでした：{ $refusal }
        [task] { $path } はこのパッケージで置き換えてよいものではなく、スケジュールされたタスク { $name } がこれに依存しているため、何もインストールされませんでした：{ $refusal }
        [firewall] { $path } はこのパッケージで置き換えてよいものではなく、ファイアウォール規則 { $name } がこれに依存しているため、何もインストールされませんでした：{ $refusal }
        [shortcut] { $path } はこのパッケージで置き換えてよいものではなく、ショートカット { $name } がこれに依存しているため、何もインストールされませんでした：{ $refusal }
       *[registry] { $path } はこのパッケージで置き換えてよいものではなく、レジストリ値 { $name } がこれに依存しているため、何もインストールされませんでした：{ $refusal }
    }
core-warning-restore-destination-exists = 書き込み先が既に存在します。上書きするには --force を使用してください。
# { $path } is a file. { $error } is what went wrong.
core-warning-restore-file-not-written = { $path } を書き込めませんでした：{ $error }
core-warning-restore-not-a-folder = この空のフォルダーを配置する場所に、フォルダー以外のものがあります。
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-not-looked-at = { $path } を確認できませんでした：{ $error }
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-folder-not-made = { $path } を作成できませんでした：{ $error }
# Why part of the app was left out, after "was left out:".
core-warning-restore-registration-exists = この PC には既に別の設定で登録されており、インストールで引き継ぐことはありません
core-warning-restore-registry-review = レジストリ値はアーカイブされた構造化データから復元され、既知のアプリのパスは対象のルートに書き換えられました。
# { $error } is what went wrong.
core-warning-restore-firewall-unreadable = Windows ファイアウォールの規則を読み取れませんでした：{ $error }
# Why a firewall rule was left out, after "was left out:".
core-warning-restore-firewall-rule-exists = このプログラム用の同じ名前の規則が別の設定で既に存在します。インストールでその規則は変更しません
# { $name } is a runtime.
core-warning-restore-prerequisite-unknown = { $name }：このランタイムの既知のインストーラーはありません。
# { $name } is a runtime.
core-warning-restore-prerequisite-needs-admin = { $name } のインストールには管理者権限が必要です。インストーラーを管理者として再実行してください。
# { $name } is a runtime.
core-warning-restore-prerequisite-reboot = { $name } はインストールされましたが、完了するには再起動が必要です。
# { $error } is what went wrong.
core-warning-restore-skipped-not-recorded = スキップしたランタイムを記録できませんでした：{ $error }
# { $name } is a runtime. { $reason } is why it was skipped, already in words.
core-warning-restore-prerequisite-skipped = { $name }：{ $reason }
# { $name } is a runtime. { $reason } is why it could not be installed. { $url } is a web address to get it from.
core-warning-restore-prerequisite-failed = { $name } をインストールできませんでした：{ $reason }。{ $url } から入手してください。
# { $error } is what went wrong.
core-warning-restore-runtime-not-recorded = インストールしたランタイムを記録できませんでした：{ $error }
# { $error } is what went wrong.
core-warning-restore-license-source-invalid = パッケージ内のライセンスのパスが無効です：{ $error }
core-warning-restore-license-needs-admin = 保護された場所にライセンス データを復元するには、管理者権限が必要です。
core-warning-restore-license-exists = ライセンスの書き込み先が既に存在します。上書きするには --force を使用してください。
# { $error } is what went wrong.
core-warning-restore-license-failed = ライセンス データを復元できませんでした：{ $error }
core-warning-restore-driver-needs-admin = ドライバーの登録には管理者権限が必要です。インストーラーを管理者として再実行してください。
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-warning-restore-task-runs-as-system = スケジュールされたタスク { $name } は SYSTEM として実行され、この PC に対するすべての権限を持ちます。
core-warning-restore-shortcut-exists = ショートカットが既に存在します。上書きするには --force を使用してください。
core-warning-restore-shortcut-all-users = 復元したショートカットの 1 つ以上が、パブリック デスクトップまたは共通のスタート メニューの場所にあります。
core-warning-restore-environment-refresh = 環境変数の変更を反映するには、新しいプロセス、シェル、またはサインイン セッションが必要な場合があります。
# { $name } is a service's name. { $account } is the account it ran as on the PC it came from.
core-warning-restore-service-account = サービス { $name } は、移行元の PC で { $account } として実行されていました。パスワードはインストーラーに含まれていないため、サービスは LocalService として設定されます。動作しない場合は、「サービス」でアカウントとパスワードを設定してください。
# { $name } is a runtime.
core-warning-restore-prerequisite-pending-reboot = { $name } はまだ登録されていません。再起動後に有効になるはずです。
# { $name } is a runtime.
core-warning-restore-prerequisite-not-verified = { $name } は成功を報告しましたが、登録されていません。インストールに失敗した可能性があります。
# { $error } is what went wrong.
core-warning-restore-driver-source-invalid = 同梱のドライバーのパスが無効です：{ $error }
# { $error } is what went wrong.
core-warning-restore-driver-not-read = ドライバーの現在の設定を読み取れませんでした：{ $error }
core-warning-restore-driver-exists = ドライバー サービスが既に存在します。再構成するには --force を使用してください。
core-warning-restore-driver-image-exists = ドライバー イメージが既に存在します。上書きするには --force を使用してください。
core-warning-restore-driver-hash-bundled = 同梱のドライバー イメージが記録されたハッシュと一致しないため、インストールされませんでした。
# { $error } is what went wrong.
core-warning-restore-driver-not-placed = ドライバー イメージを配置できませんでした：{ $error }
core-warning-restore-driver-hash-placed = ドライバー イメージが配置後に変更されたため、登録されませんでした。
# { $error } is what went wrong.
core-warning-restore-key-not-created = レジストリ キーを作成できませんでした：{ $error }
# { $error } is what went wrong.
core-warning-restore-value-not-read = 置き換える値を読み取れませんでした：{ $error }
# { $error } is what went wrong.
core-warning-restore-value-not-written = レジストリ値を書き込めませんでした：{ $error }
core-warning-restore-value-exists = レジストリ値が既に存在します。上書きするには --force を使用してください。
# { $error } is what went wrong.
core-warning-restore-environment-key-not-created = 環境変数のレジストリ キーを作成できませんでした：{ $error }
core-warning-restore-variable-exists = 環境変数が既に存在します。上書きするには --force を使用してください。
# { $error } is what went wrong.
core-warning-restore-variable-not-written = 環境変数を書き込めませんでした：{ $error }
core-warning-restore-value-kind-missing = レジストリ値の種類がありません。
# { $error } is what went wrong.
core-warning-restore-service-not-read = サービスの現在の設定を読み取れませんでした：{ $error }
core-warning-restore-service-exists = サービスが既に存在します。再構成するには --force を使用してください。
# { $error } is what went wrong.
core-warning-restore-task-not-queried = スケジュールされたタスクを照会できませんでした：{ $error }
core-warning-restore-task-exists = スケジュールされたタスクが既に存在します。上書きするには --force を使用してください。
# { $error } is what went wrong.
core-warning-restore-task-folder-not-looked-at = タスクのフォルダーを確認できませんでした：{ $error }
# { $error } is what went wrong.
core-warning-restore-shortcut-not-put-back = ショートカットを作成できず、置き換えたショートカットも元に戻せませんでした：{ $error }
# { $error } is what went wrong.
core-warning-restore-command-not-started = コマンドを開始できませんでした：{ $error }
# { $output } is what the command wrote, as it wrote it.
core-warning-restore-command-failed = コマンドが失敗しました：{ $output }
# { $error } is what went wrong.
core-warning-restore-powershell-not-started = PowerShell を開始できませんでした：{ $error }
