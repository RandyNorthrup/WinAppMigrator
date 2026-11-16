# Words the core writes for people: warnings, refusals, errors, and the installer's list of what it sets up.
# Message names start with core-.


## Warnings a package records when it is built, which an installer shows later, maybe on another PC, in that PC's
## language.

# { $count } is how many shortcuts.
core-warning-shortcuts-unreadable =
    { $count ->
       *[other] 이 드라이브의 바로 가기 { $count }개에 파일이 지정되어 있지 않아, 이 앱을 여는 바로 가기인지 확인할 수 없습니다.
    }

## Undoing an install: what an uninstall, or a failed install, says it put back, left, or could not undo. Each line is
## shown on its own, in a list.

# { $app_name } is the app's name.
core-rollback-record-damaged = { $app_name } 기록의 일부를 읽을 수 없으므로, 그 부분에 기록된 항목은 그대로 남아 있으며 대체된 파일이 원래대로 돌아오지 않을 수 있습니다.
core-rollback-settings-entry-stays = 설정에서 이 작업을 다시 실행할 수 있도록 설정의 앱 항목은 그대로 유지됩니다.
# { $name } is a runtime's name, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-rollback-prerequisite-stays = { $name }은(는) 설치되었거나 설치되었을 수 있으며, 다른 프로그램에서 사용 중일 수 있으므로 그대로 유지됩니다.
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-drive-not-connected = 이 설치가 { $folder }에 넣은 항목은 연결되지 않은 드라이브에 있습니다. 드라이브를 연결하면 이 작업을 완료할 수 있습니다.
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-record-part-unread = 기록의 해당 부분을 읽을 수 없으므로, 이 설치가 { $folder }에 넣은 항목은 그대로 유지됩니다.

## Why a file or folder was left, shown after "was left:" in the messages below them. Each is part of a sentence, so
## starts with a small letter and has no full stop.

# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-being-changed = 이 작업이 실행되는 동안 무언가가 { $folder } 폴더를 변경하고 있어 그 폴더에서는 더 이상 작업하지 않았습니다. 변경하고 있는 프로그램을 닫고 다시 실행하세요
# { $folder } is a folder; { $error } is what Windows said went wrong.
core-rollback-reason-not-opened = { $folder } 폴더를 열 수 없습니다: { $error }
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-no-record = { $folder } 폴더가 있던 위치에 대한 기록이 없습니다
# { $folder } is a folder; { $actual } is the folder it now leads to.
core-rollback-reason-moved = { $folder } 폴더가 이제 설치 당시의 위치가 아닌 { $actual }을(를) 가리킵니다
# Why a service was not stopped.
core-rollback-reason-did-not-stop = 제시간에 중지되지 않았습니다

## Files and folders

# { $path } is a folder; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-folder-left = { $path } 폴더를 그대로 두었습니다: { $reason }.
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-removed = { $path } 폴더를 제거했습니다
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-holds-others = { $path } 폴더에는 이 설치가 넣지 않은 파일이 있으므로 그대로 유지됩니다.
# { $path } is a folder; { $error } is what Windows said went wrong.
core-rollback-folder-not-removed = { $path } 폴더를 제거할 수 없습니다: { $error }.
# { $path } is a file; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-file-left = { $path } 파일을 그대로 두었습니다: { $reason }.
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-folder-gone = 파일이 있던 폴더가 없어졌으므로 { $path }을(를) 되돌리지 않습니다.
# { $path } is a file; { $error } is what went wrong.
core-rollback-partial-not-removed = 일부만 기록된 파일을 제거할 수 없습니다: { $path }({ $error }).
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-checked = { $path }을(를) 확인할 수 없습니다: { $error }.
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-removed = { $path }을(를) 제거했습니다
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-removed = { $path }을(를) 제거할 수 없습니다: { $error }.
# { $path } is a file this install had replaced, and has put back.
core-rollback-file-put-back = { $path }을(를) 원래대로 되돌렸습니다
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-put-back = { $path }을(를) 원래대로 되돌릴 수 없습니다: { $error }.
# Why a file could not be put back.
core-rollback-backup-gone = 백업이 없어졌습니다
# { $path } is a file.
core-rollback-no-parent-folder = { $path }에 상위 폴더가 없습니다

## A file that stays. { $why } says why: still-used (something that stays still uses it), changed (it has changed since
## it was installed), or not-asked (it is someone's data, and they did not ask for it to go). { $path } is the file.

core-rollback-file-stays =
    { $why ->
        [still-used] 그대로 유지되는 다른 항목이 아직 사용하므로 { $path }은(는) 그대로 유지됩니다.
        [changed] 설치된 후 변경되었으므로 { $path }은(는) 그대로 유지됩니다.
       *[not-asked] 제거 요청이 없었으므로 { $path }은(는) 그대로 유지됩니다.
    }
# The file it replaced was to be put beside it, under another name, and every name was taken.
core-rollback-file-stays-no-name-beside =
    { $why ->
        [still-used] 그대로 유지되는 다른 항목이 아직 사용하므로 { $path }은(는) 그대로 유지되며, 이 파일이 대체한 파일을 그 옆에 놓을 수 없습니다: 사용할 수 있는 이름이 모두 사용 중입니다.
        [changed] 설치된 후 변경되었으므로 { $path }은(는) 그대로 유지되며, 이 파일이 대체한 파일을 그 옆에 놓을 수 없습니다: 사용할 수 있는 이름이 모두 사용 중입니다.
       *[not-asked] 제거 요청이 없었으므로 { $path }은(는) 그대로 유지되며, 이 파일이 대체한 파일을 그 옆에 놓을 수 없습니다: 사용할 수 있는 이름이 모두 사용 중입니다.
    }
# { $name } is the name the file it replaced now has, such as settings.ini.before-install.
core-rollback-file-stays-beside =
    { $why ->
        [still-used] 그대로 유지되는 다른 항목이 아직 사용하므로 { $path }은(는) 그대로 유지됩니다. 이 파일이 대체한 파일은 그 옆에 있습니다(이름: { $name }).
        [changed] 설치된 후 변경되었으므로 { $path }은(는) 그대로 유지됩니다. 이 파일이 대체한 파일은 그 옆에 있습니다(이름: { $name }).
       *[not-asked] 제거 요청이 없었으므로 { $path }은(는) 그대로 유지됩니다. 이 파일이 대체한 파일은 그 옆에 있습니다(이름: { $name }).
    }
# { $error } is what went wrong.
core-rollback-file-stays-not-beside =
    { $why ->
        [still-used] 그대로 유지되는 다른 항목이 아직 사용하므로 { $path }은(는) 그대로 유지되며, 이 파일이 대체한 파일을 그 옆에 놓을 수 없습니다: { $error }.
        [changed] 설치된 후 변경되었으므로 { $path }은(는) 그대로 유지되며, 이 파일이 대체한 파일을 그 옆에 놓을 수 없습니다: { $error }.
       *[not-asked] 제거 요청이 없었으므로 { $path }은(는) 그대로 유지되며, 이 파일이 대체한 파일을 그 옆에 놓을 수 없습니다: { $error }.
    }

## The registry and environment variables. { $kind } says what { $name } is: key (a registry key, such as
## HKLM\Software\App), value (a registry value, such as HKLM\Software\App\Version), user-variable or machine-variable
## (an environment variable of the user's or of the whole PC, such as APP_HOME), or user-list or machine-list (a
## variable that is a list of folders, such as Path). { $error } is what Windows said went wrong.

# How a registry key's unnamed value is shown, as Registry Editor shows it.
core-registry-default-value-name = (기본값)
core-rollback-setting-other-account =
    { $kind ->
        [key] { $name } 키는 이 앱을 설치한 계정에 속하는데 그 계정이 로그인되어 있지 않으므로 그대로 유지됩니다.
        [user-variable] { $name } 사용자 환경 변수는 이 앱을 설치한 계정에 속하는데 그 계정이 로그인되어 있지 않으므로 그대로 유지됩니다.
        [machine-variable] { $name } 시스템 환경 변수는 이 앱을 설치한 계정에 속하는데 그 계정이 로그인되어 있지 않으므로 그대로 유지됩니다.
        [user-list] 사용자 { $name } 변수는 이 앱을 설치한 계정에 속하는데 그 계정이 로그인되어 있지 않으므로 그대로 유지됩니다.
        [machine-list] 시스템 { $name } 변수는 이 앱을 설치한 계정에 속하는데 그 계정이 로그인되어 있지 않으므로 그대로 유지됩니다.
       *[value] { $name } 레지스트리 값은 이 앱을 설치한 계정에 속하는데 그 계정이 로그인되어 있지 않으므로 그대로 유지됩니다.
    }
core-rollback-setting-not-reached =
    { $kind ->
        [key] { $name } 키에 액세스할 수 없습니다: { $error }.
        [user-variable] { $name } 사용자 환경 변수에 액세스할 수 없습니다: { $error }.
        [machine-variable] { $name } 시스템 환경 변수에 액세스할 수 없습니다: { $error }.
        [user-list] 사용자 { $name } 변수에 액세스할 수 없습니다: { $error }.
        [machine-list] 시스템 { $name } 변수에 액세스할 수 없습니다: { $error }.
       *[value] { $name } 레지스트리 값에 액세스할 수 없습니다: { $error }.
    }
core-rollback-setting-not-read =
    { $kind ->
        [key] { $name } 키를 읽을 수 없습니다: { $error }.
        [user-variable] { $name } 사용자 환경 변수를 읽을 수 없습니다: { $error }.
        [machine-variable] { $name } 시스템 환경 변수를 읽을 수 없습니다: { $error }.
        [user-list] 사용자 { $name } 변수를 읽을 수 없습니다: { $error }.
        [machine-list] 시스템 { $name } 변수를 읽을 수 없습니다: { $error }.
       *[value] { $name } 레지스트리 값을 읽을 수 없습니다: { $error }.
    }
core-rollback-setting-changed =
    { $kind ->
        [user-variable] { $name } 사용자 환경 변수는 설치된 후 변경되었으므로 그대로 유지됩니다.
        [machine-variable] { $name } 시스템 환경 변수는 설치된 후 변경되었으므로 그대로 유지됩니다.
        [user-list] 사용자 { $name } 변수는 설치된 후 변경되었으므로 그대로 유지됩니다.
        [machine-list] 시스템 { $name } 변수는 설치된 후 변경되었으므로 그대로 유지됩니다.
       *[value] { $name } 레지스트리 값은 설치된 후 변경되었으므로 그대로 유지됩니다.
    }
# Said when it is back as it was before the install.
core-rollback-setting-put-back =
    { $kind ->
        [user-variable] { $name } 사용자 환경 변수를 원래대로 되돌렸습니다
        [machine-variable] { $name } 시스템 환경 변수를 원래대로 되돌렸습니다
        [user-list] 사용자 { $name } 변수를 원래대로 되돌렸습니다
        [machine-list] 시스템 { $name } 변수를 원래대로 되돌렸습니다
       *[value] { $name } 레지스트리 값을 원래대로 되돌렸습니다
    }
core-rollback-setting-not-put-back =
    { $kind ->
        [user-variable] { $name } 사용자 환경 변수를 원래대로 되돌릴 수 없습니다: { $error }.
        [machine-variable] { $name } 시스템 환경 변수를 원래대로 되돌릴 수 없습니다: { $error }.
        [user-list] 사용자 { $name } 변수를 원래대로 되돌릴 수 없습니다: { $error }.
        [machine-list] 시스템 { $name } 변수를 원래대로 되돌릴 수 없습니다: { $error }.
       *[value] { $name } 레지스트리 값을 원래대로 되돌릴 수 없습니다: { $error }.
    }
# A registry value the install wrote, which is the user's data, and stays unless they asked for it to go.
core-rollback-value-kept-by-request = 제거 요청이 없었으므로 { $name } 레지스트리 값은 그대로 유지됩니다.
core-rollback-key-holds-others = { $name } 키에는 이 설치가 넣지 않은 항목이 있으므로 그대로 유지됩니다.
core-rollback-key-removed = { $name } 키를 제거했습니다
core-rollback-key-not-removed = { $name } 키를 제거할 수 없습니다: { $error }.
# { $kind } is user-list or machine-list.
core-rollback-list-entries-removed =
    { $kind ->
        [user-list] 사용자 { $name } 변수에서 이 설치의 항목을 제거했습니다
       *[machine-list] 시스템 { $name } 변수에서 이 설치의 항목을 제거했습니다
    }
core-rollback-list-entries-not-removed =
    { $kind ->
        [user-list] 사용자 { $name } 변수에서 이 설치의 항목을 제거할 수 없습니다: { $error }.
       *[machine-list] 시스템 { $name } 변수에서 이 설치의 항목을 제거할 수 없습니다: { $error }.
    }

## Services and drivers. { $kind } is service or driver; { $name } is its name, such as AppUpdater. { $error } and
## { $reason } are what Windows said went wrong.

core-rollback-service-not-read =
    { $kind ->
        [driver] { $name } 드라이버를 읽을 수 없습니다: { $error }.
       *[service] { $name } 서비스를 읽을 수 없습니다: { $error }.
    }
core-rollback-service-changed =
    { $kind ->
        [driver] { $name } 드라이버는 설치된 후 변경되었으므로 그대로 유지됩니다.
       *[service] { $name } 서비스는 설치된 후 변경되었으므로 그대로 유지됩니다.
    }
core-rollback-service-not-stopped =
    { $kind ->
        [driver] { $name } 드라이버를 중지할 수 없으므로({ $reason }), PC를 다시 시작할 때까지 이 드라이버가 사용 중인 항목이 남아 있을 수 있습니다.
       *[service] { $name } 서비스를 중지할 수 없으므로({ $reason }), PC를 다시 시작할 때까지 이 서비스가 사용 중인 항목이 남아 있을 수 있습니다.
    }
core-rollback-service-stopped-for-this =
    { $kind ->
        [driver] 이 작업을 위해 { $name } 드라이버를 중지했습니다. 필요할 때 다시 시작하세요.
       *[service] 이 작업을 위해 { $name } 서비스를 중지했습니다. 필요할 때 다시 시작하세요.
    }
core-rollback-service-not-removed =
    { $kind ->
        [driver] { $name } 드라이버를 제거할 수 없습니다: { $reason }.
       *[service] { $name } 서비스를 제거할 수 없습니다: { $reason }.
    }
core-rollback-service-marked-for-removal =
    { $kind ->
        [driver] { $name } 드라이버가 제거 대상으로 표시되었으며, PC를 다시 시작하거나 이 드라이버를 열고 있는 프로그램이 해제하면 제거됩니다.
       *[service] { $name } 서비스가 제거 대상으로 표시되었으며, PC를 다시 시작하거나 이 서비스를 열고 있는 프로그램이 해제하면 제거됩니다.
    }
core-rollback-service-removed =
    { $kind ->
        [driver] { $name } 드라이버를 제거했습니다
       *[service] { $name } 서비스를 제거했습니다
    }
core-rollback-service-not-restored =
    { $kind ->
        [driver] { $name } 드라이버를 원래 상태로 되돌릴 수 없습니다: { $reason }.
       *[service] { $name } 서비스를 원래 상태로 되돌릴 수 없습니다: { $reason }.
    }
core-rollback-service-restored =
    { $kind ->
        [driver] { $name } 드라이버의 설정을 원래대로 되돌렸습니다
       *[service] { $name } 서비스의 설정을 원래대로 되돌렸습니다
    }
core-rollback-service-not-restarted = { $name } 서비스는 이 설치 전에 실행 중이었지만 다시 시작되지 않았습니다: { $reason }.
# { $account } is the account the service ran as, such as CONTOSO\backup.
core-rollback-service-account-again = { $name } 서비스는 { $account } 계정으로 실행되었습니다. 암호는 보관되지 않으므로 "서비스"에서 계정을 다시 설정하세요.

## Firewall rules. { $name } is the rule's name; { $error } is what went wrong.

core-rollback-firewall-not-read = { $name } 방화벽 규칙을 읽을 수 없습니다: { $error }.
core-rollback-firewall-says-otherwise = 그 이름으로 있는 규칙의 내용이 다르므로 { $name } 방화벽 규칙은 그대로 유지됩니다.
core-rollback-firewall-changed = { $name } 방화벽 규칙은 설치된 후 변경되었으므로 그대로 유지됩니다.
# The capital "The" is as the app has always said it.
core-rollback-firewall-removed = { $name } 방화벽 규칙을 제거했습니다
core-rollback-firewall-not-removed = { $name } 방화벽 규칙을 제거할 수 없습니다: { $error }.

## Scheduled tasks. { $path } is a folder of tasks, and { $name } a task with its folder, such as \App\Nightly Update.
## { $error } and { $reason } are what went wrong.

core-rollback-task-folder-holds-others = { $path } 작업 폴더에는 이 설치가 넣지 않은 작업이 있으므로 그대로 유지됩니다.
core-rollback-task-folder-not-removed = { $path } 작업 폴더를 제거할 수 없습니다: { $error }.
core-rollback-task-not-read = { $name } 작업을 읽을 수 없습니다: { $error }.
core-rollback-task-changed = { $name } 작업은 설치된 후 변경되었으므로 그대로 유지됩니다.
core-rollback-task-removed = { $name } 작업을 제거했습니다
core-rollback-task-not-removed = { $name } 작업을 제거할 수 없습니다: { $reason }.
core-rollback-task-put-back = { $name } 작업을 원래대로 되돌렸습니다
core-rollback-task-not-put-back = { $name } 작업을 원래대로 되돌릴 수 없습니다: { $reason }.

## When an install cannot be undone at all

# { $error } is what went wrong.
core-rollback-record-unreadable = 이 설치의 기록을 읽을 수 없어 아무것도 되돌리지 않았습니다: { $error }.
# { $error } is what went wrong.
core-rollback-nothing-undone = 아무것도 되돌리지 않았습니다: { $error }.
# One app's part of the message below. { $app_name } is the app's name; { $failures } is what could not be undone, a
# sentence or more; { $folder } is the folder that holds the install's record.
core-rollback-earlier-install-stuck = { $app_name }: { $failures } 이 설치를 끝내 되돌릴 수 없다면, 관리자가 { $folder } 폴더를 제거하여 되돌리기를 포기할 수 있습니다.
# Why an install did not start. { $details } is one or more of the message above.
core-rollback-earlier-installs-not-undone = 완료되지 않은 이전 설치를 완전히 되돌릴 수 없어 아무것도 설치하지 않았습니다. 이 항목을 사용 중인 프로그램을 닫고 다시 시도하세요. { $details }
core-rollback-install-busy = 다른 설치 관리자 또는 제거 프로그램이 이 설치를 변경하고 있습니다

## The summary an uninstall writes on the command line, one line each, with what each says below it. { $count } is how
## many.

core-rollback-count-undone = 되돌림: { $count }
core-rollback-count-failed = 되돌릴 수 없음: { $count }
# What was left on purpose: it has changed since the install, or it is the user's.
core-rollback-count-kept = 그대로 둠: { $count }
# What no uninstall can take back, such as a runtime that was installed, which other programs may use.
core-rollback-count-left-behind = 남음: { $count }

## Which Windows a PC or drive has, as the app shows it beside the drive.

# { $name } is the Windows' name, such as Windows 7 Professional Service Pack 1.
core-system-windows-32-bit = { $name }, 32비트
# { $name } is the Windows' name, such as Windows 11 Pro.
core-system-windows-64-bit = { $name }, 64비트
# A drive's Windows, when it only says it is older than Vista.
core-system-windows-xp-or-older = Windows XP 또는 이전 버전

## Reading a connected drive, or another account's settings. Each is why it could not be done, and some start with a
## small letter, as they follow other words.

core-system-needs-administrator = 연결된 드라이브나 다른 계정의 설정을 읽으려면 관리자 권한이 필요합니다
core-system-no-program-data = 이 PC의 ProgramData 폴더를 찾을 수 없습니다
# { $path } is a file, such as D:\Windows\System32\config\SOFTWARE.
core-system-not-a-hive-file = 하이브 파일이 아닙니다: { $path }
# { $path } is a file; { $size } is its size in bytes.
core-system-hive-too-large = { $path }의 크기는 { $size }바이트로, 레지스트리 하이브라고 하기에는 너무 크거나 이 드라이브의 하이브를 모두 합한 크기보다 큽니다
# { $key } is a registry key, such as Software\Microsoft\Windows\CurrentVersion.
core-system-hive-no-mapping = 오프라인 하이브에 HKLM\{ $key }에 대한 매핑이 없습니다
# { $key } is where a drive's registry was to be loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said.
# "reg load" is a command's name.
core-system-hive-not-loaded = reg load { $key } 명령이 실패했습니다: { $error }
# { $key } is where a drive's registry was loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said. "reg
# unload" is a command's name.
core-system-hive-not-unloaded = reg unload { $key } 명령이 실패했습니다: { $error }
# { $drive } is a drive, such as D:\.
core-system-not-a-system-drive = { $drive } 드라이브는 Windows 시스템 드라이브가 아닙니다(\Windows 폴더 없음).
# { $drive } is a drive, such as C:\. "This PC" is the name of a choice in the app.
core-system-own-system-drive = { $drive } 드라이브는 이 PC의 시스템 드라이브입니다. 이 PC를 읽으려면 "이 PC"를 선택하세요.
# { $drive } is a drive, such as D:\.
core-system-registry-through-link = { $drive } 드라이브는 Windows 시스템 드라이브가 아닙니다: 레지스트리로 가는 경로가 링크를 거칩니다.

## Warnings a scan records about whose data it read.

# { $account } is an account's name or id, such as alice or S-1-5-21-...-1001.
core-warning-scan-account-not-on-record = { $account } 계정이 여기에 기록되어 있지 않으므로 어떤 사용자의 사용자별 데이터나 설정도 포함되지 않았습니다.
# { $account } is an account's name, such as CONTOSO\alice.
core-warning-scan-signed-in-account-no-profile = 여기에 로그인한 계정 { $account }에 기록된 프로필이 없으므로 어떤 사용자의 사용자별 데이터나 설정도 포함되지 않았습니다.
# { $reason } is why Windows could not say.
core-warning-scan-signed-in-account-unknown = 로그인한 사용자를 확인할 수 없으므로({ $reason }) 어떤 사용자의 사용자별 데이터나 설정도 포함되지 않았습니다. 포함할 계정을 선택하세요.
core-warning-scan-last-account-unknown = 이 드라이브에는 마지막으로 로그인한 사용자가 기록되어 있지 않고 계정이 두 개 이상 있으므로, 어떤 사용자의 사용자별 데이터나 설정도 포함되지 않았습니다. 포함할 계정을 선택하세요.
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-settings-unreadable = { $account }의 설정을 읽을 수 없어 포함하지 않았습니다: { $error }
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-classes-unreadable = { $account }에 대해 등록된 COM 클래스와 파일 형식을 읽을 수 없어 포함하지 않았습니다: { $error }

## Paths a scan did not follow. { $count } is how many; { $paths } is some of them, separated by semicolons; { $more } is
## how many more there are, not shown.

core-warning-scan-drive-paths-not-followed =
    { $count ->
       *[other] 이 드라이브에 기록된 경로 중 읽을 수 없는 경로가 있어 해당 위치의 항목은 포함되지 않았습니다. 각 경로는 다른 디스크에 있거나, 전체 경로가 아니거나, 드라이브 밖으로 이어지는 링크를 거칩니다: { $paths }
    }
core-warning-scan-drive-paths-not-followed-more = 이 드라이브에 기록된 경로 중 읽을 수 없는 경로가 있어 해당 위치의 항목은 포함되지 않았습니다. 각 경로는 다른 디스크에 있거나, 전체 경로가 아니거나, 드라이브 밖으로 이어지는 링크를 거칩니다: { $paths } 외 { $more }개
core-warning-scan-network-paths-not-followed =
    { $count ->
       *[other] 이 PC에 기록된 경로 중 읽을 수 없는 경로가 있어 해당 위치의 항목은 포함되지 않았습니다. 각 경로는 이 PC의 디스크에 있지 않아 확인하려면 그 위치에 연결해야 하거나, 다른 항목을 열게 되는 이름입니다: { $paths }
    }
core-warning-scan-network-paths-not-followed-more = 이 PC에 기록된 경로 중 읽을 수 없는 경로가 있어 해당 위치의 항목은 포함되지 않았습니다. 각 경로는 이 PC의 디스크에 있지 않아 확인하려면 그 위치에 연결해야 하거나, 다른 항목을 열게 되는 이름입니다: { $paths } 외 { $more }개

## What a scan with none of an account's data leaves out of a package, in the app's list of what was left out. Each
## has a name, and below it why.

core-account-left-out-shortcuts = 이 계정 자체 폴더에 있는 바로 가기
# { $count } is how many shortcuts.
core-account-left-out-shortcuts-why =
    { $count ->
       *[other] 바로 가기 { $count }개로, 사용자 데이터 옵션을 켠 경우에만 포함됩니다
    }
core-account-left-out-variables = 이 계정의 환경 변수
# { $count } is how many variables.
core-account-left-out-variables-why =
    { $count ->
       *[other] 변수 { $count }개로, 사용자 데이터 옵션을 켠 경우에만 포함됩니다
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-account-left-out-task = { $task } 예약된 작업
core-account-task-works-on-account-folder = 계정 폴더 안의 위치를 대상으로 작업하거나 그 위치의 이름이 붙어 있으며, 사용자 데이터 옵션을 켠 경우에만 포함됩니다
core-account-task-runs-from-account-folder = 이 작업이 계정 폴더에서 함께 실행하는 항목은 사용자 데이터 옵션을 켠 경우에만 포함됩니다
# { $service } is a service's name, such as AppUpdater.
core-account-left-out-service = { $service } 서비스
core-account-service-works-on-account-folder = 계정 폴더 안의 위치를 대상으로 작업하며, 사용자 데이터 옵션을 켠 경우에만 포함됩니다

## What a package was found to still say of whose PC it came from, after a build with none of an account's data.

# The parts of a package, as the warning below names them.
core-account-part-files = 앱 자체 파일의 경로
core-account-part-registry = 레지스트리 값
core-account-part-settings-entry = 설정의 앱 항목
core-account-part-main-program = 주 프로그램의 위치
core-account-part-shortcuts = 바로 가기
core-account-part-tasks = 예약된 작업
core-account-part-services = 서비스
core-account-part-variables = 환경 변수
# One place in the list below. { $place } is one of the parts above, or a file's name; { $count } is how many times.
core-account-mention-place = { $place }({ $count })
# { $kind } is what was found: account (an account's name), folder (where an account's folder is), computer (the PC's
# name), or id (an account's id). { $places } is a list of where, from the message above.
core-warning-scan-package-mentions =
    { $kind ->
        [account] 이 패키지의 { $places }에 계정 이름이 나와 있습니다. 이 패키지는 계정 데이터 없이 빌드되었지만, 설치 관리자를 가진 사람은 누구나 이 내용을 읽을 수 있습니다.
        [folder] 이 패키지의 { $places }에 계정 폴더의 위치가 나와 있습니다. 이 패키지는 계정 데이터 없이 빌드되었지만, 설치 관리자를 가진 사람은 누구나 이 내용을 읽을 수 있습니다.
        [computer] 이 패키지의 { $places }에 PC 이름이 나와 있습니다. 이 패키지는 계정 데이터 없이 빌드되었지만, 설치 관리자를 가진 사람은 누구나 이 내용을 읽을 수 있습니다.
       *[id] 이 패키지의 { $places }에 계정 ID가 나와 있습니다. 이 패키지는 계정 데이터 없이 빌드되었지만, 설치 관리자를 가진 사람은 누구나 이 내용을 읽을 수 있습니다.
    }
# { $count } is how many folders.
core-warning-scan-folders-of-others-unreadable =
    { $count ->
       *[other] 앱 밖에 있는 폴더 { $count }개를 읽을 수 없어, 그곳에 있는 앱 데이터가 누락되었을 수 있습니다.
    }

## What a scan leaves out because it looks like a credential, or is a license with the License option off, in the app's
## list of what was left out. Each has a name, and below it why.

core-secret-why-credential = 자격 증명처럼 보입니다
core-secret-why-license-option-off = "라이선스 및 정품 인증 데이터" 옵션이 꺼져 있습니다
# { $count } is how many values of a registry key.
core-secret-why-account-values =
    { $count ->
       *[other] 값 { $count }개로, 사용자 데이터 옵션을 켠 경우에만 포함됩니다
    }
# Shown in place of a registry value's name when its name itself looks like a password or a key.
core-secret-name-holds-credential = (이름에 자격 증명이 포함된 값)
# Shown in place of an option's name, for an argument on a command line that has no name.
core-secret-unnamed-argument = 자격 증명이 포함된 인수
# { $option } is an option taken out of a command line, such as /password, or the message above. { $place } says
# where the command line was: service, task, shortcut, opener (what opens a file type), opener-unnamed (the same, of a
# type with no name), or uninstall. { $name } is the service, task, shortcut, file type, or app.
core-secret-taken-from =
    { $place ->
        [service] { $name }의 실행 방식에 포함된 { $option }
        [task] { $name } 작업의 실행 방식에 포함된 { $option }
        [shortcut] { $name } 바로 가기에 포함된 { $option }
        [opener] { $name }을(를) 여는 항목에 포함된 { $option }
        [opener-unnamed] 파일 형식을 여는 항목에 포함된 { $option }
       *[uninstall] { $name }의 제거 방식에 포함된 { $option }
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-secret-task-definition-unreadable = 확인하기 위해 읽을 수 없었던 { $task } 작업의 정의
# { $name } is an environment variable's name, such as APP_TOKEN.
core-secret-left-out-variable = { $name } 환경 변수
# { $class } is a COM class's id, such as {00000000-0000-0000-0000-000000000000}.
core-secret-left-out-class-server = { $class } 클래스가 실행한다고 지정한 항목
# { $rule } is a firewall rule's name.
core-secret-left-out-firewall-description = { $rule } 방화벽 규칙의 설명

## Licenses a scan found, as the package's report describes each.

# { $name } is a file's name, such as license.lic.
core-license-detail-file = 라이선스 파일 검색됨: { $name }
# { $framework } is a licensing product's name, such as FlexLM / FlexNet.
core-license-detail-store = { $framework } 라이선스 저장소 있음
# { $name } is an environment variable's name, such as LM_LICENSE_FILE.
core-license-detail-variable = 라이선스 환경 변수 { $name }
# { $name } is a registry value's name, such as SerialNumber.
core-license-detail-registry-value = 라이선스 데이터로 보이는 레지스트리 값: { $name }
# Said of a registry value in an account's key, which is not named.
core-license-detail-registry-value-of-an-account = 라이선스 데이터로 보이는 레지스트리 값
# { $key } is a registry key, such as HKLM\Software\Contoso\License.
core-license-detail-registry-key = 라이선스 데이터로 보이는 레지스트리 키: { $key }

## Warnings a scan records about licenses.

core-warning-scan-license-data = 라이선스/정품 인증 데이터가 검색되었습니다. "라이선스 및 정품 인증 데이터" 옵션을 켜면 설치 관리자가 옮길 수 있는 항목을 복원합니다. 컴퓨터 전체 값과 하드웨어에 종속된 저장소는 보고만 합니다. WinAppMigrator는 암호를 해독하거나, 정품 인증을 우회하거나, 동글을 에뮬레이션하지 않습니다.
core-warning-scan-license-machine-bound = 일부 라이선스는 컴퓨터에 종속되어 있습니다(하드웨어 동글, 컴퓨터 고정 정품 인증 또는 암호화). 설치 관리자가 가능한 항목을 복원하지만, 이 라이선스는 새 PC에서 다시 정품 인증해야 할 수 있습니다.

## Warnings a scan records about the registry. What could not be read is not named in them: it is kept beside them.

# { $error } is what Windows said went wrong. App Paths is a registry key's name.
core-warning-scan-app-paths-key-unreadable = App Paths 하위 키를 읽을 수 없습니다: { $error }
core-warning-scan-registry-limit = 레지스트리 캡처가 구성된 안전 한도에 도달했습니다.
core-warning-scan-registry-subtree-limit = 레지스트리 하위 트리 캡처가 구성된 안전 한도에 도달했습니다.
# { $error } is what Windows said went wrong.
core-warning-scan-registry-key-unreadable = 레지스트리 키를 읽을 수 없습니다: { $error }
core-warning-scan-registry-value-too-large = 레지스트리 값이 구성된 캡처 크기 한도를 초과했습니다.

## Scheduled tasks

core-warning-scan-tasks-folder-through-link = 이 드라이브의 예약된 작업 폴더가 링크를 거치므로 작업을 읽지 않았습니다.
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-warning-scan-task-left-out = { $task } 예약된 작업을 제외했습니다. 설정을 포함하는 계정이 아닌 다른 계정으로 실행되거나, 관리자가 아닌 사용자가 작업 파일을 변경할 수 있거나, 앱의 작업이 너무 많아 모두 보관할 수 없습니다.
# { $error } is what Windows said went wrong.
core-warning-scan-tasks-folder-unreadable = 예약된 작업의 일부를 읽을 수 없습니다: { $error }
# { $path } is a task's file, such as C:\Windows\System32\Tasks\App\Nightly Sync; { $reason } is one of the reasons
# below, or what Windows said went wrong.
core-warning-scan-task-unreadable = { $path } 예약된 작업을 읽을 수 없습니다: { $reason }. 이 작업이 앱을 실행한다면 포함되지 않았습니다.
# Why a task's file could not be read. { $size } is its size in bytes.
core-task-too-large = 크기가 { $size }바이트로, 작업 파일이라고 하기에는 너무 큽니다
# Why a task's file could not be read.
core-task-ends-mid-character = 문자 중간에서 끝납니다
# Why a task's file could not be read. { $error } says where.
core-task-not-utf16 = 유효한 UTF-16이 아닙니다: { $error }
# Why a task's file could not be read. { $error } says where.
core-task-not-text = 유효한 UTF-8 또는 UTF-16이 아닙니다: { $error }
# What a package records of how a task is put back. Register-ScheduledTask is a command's name.
core-task-restore-instructions = 복원 시 보관된 XML 정의로 Register-ScheduledTask를 사용합니다.

## Reading a program's file. Each is why it could not be read, after the program's name.

# { $reason } is what went wrong.
core-pe-unreadable = 읽을 수 없습니다: { $reason }
core-pe-headers-misaligned = 헤더가 프로그램 헤더에 필요한 경계에 맞춰져 있지 않습니다
# { $size } is the file's size in bytes.
core-pe-too-large = 크기가 { $size }바이트로, 프로그램이라고 하기에는 너무 큽니다
# PE is the name of the format Windows programs are in.
core-pe-no-headers = PE 헤더가 없습니다

## Warnings a scan records about what a program needs. Which program is kept beside each.

# { $error } is what went wrong.
core-warning-scan-imports-unreadable = 가져오기 테이블을 읽을 수 없습니다: { $error }
# { $error } is what went wrong.
core-warning-scan-import-name-unreadable = 가져오기 설명자 이름을 읽을 수 없습니다: { $error }
# { $count } is how many were skipped.
core-warning-scan-legacy-delay-imports =
    { $count ->
       *[other] 레거시(v1) 지연 로드 설명자 { $count }개를 건너뛰었습니다. 최신 RVA 기반 지연 가져오기만 구문 분석합니다.
    }

## The DLLs a program needs. Which DLL or program is kept beside each warning.

# { $path } is a program, such as C:\Program Files\App\app.exe.
core-dll-program-has-no-folder = 실행 파일에 상위 디렉터리가 없습니다: { $path }
# Why a DLL is among the app's files, in the package's list of them.
core-dll-beside-program = 선택한 실행 파일 옆에 있는 DLL
# { $error } is what went wrong.
core-warning-scan-dependency-unscanned = 재귀 종속성을 검사할 수 없습니다: { $error }
# { $name } is a DLL's name, such as d3dx9_43.dll.
core-warning-scan-dependency-not-in-newer-windows = { $name }은(는) 이 앱이 있던 PC의 Windows 폴더에 있었지만, Windows 10 및 11에는 기본으로 포함되어 있지 않습니다. 다른 무언가가 그 위치에 설치한 파일이므로 앱 자체 파일이 아니며 포함되지 않습니다. 이 파일을 제공하는 항목을 설치할 때까지 앱이 시작되지 않을 수 있습니다.
# { $error } is what went wrong.
core-warning-scan-dependency-not-hashed = 종속성의 해시를 계산할 수 없습니다: { $error }
# SystemRoot and windir are the names of environment variables.
core-warning-scan-windows-folder-unknown = SystemRoot/windir 환경 변수를 사용할 수 없습니다. 시스템 DLL 검색이 제한됩니다.
# { $error } is what went wrong.
core-warning-scan-dependency-architecture-unreadable = 종속성 아키텍처를 읽을 수 없습니다: { $error }

## Services, drivers, shortcuts, and environment variables a scan reads. Where it was reading is kept beside each
## warning.

# { $error } is what Windows said went wrong.
core-warning-scan-services-unreadable = 서비스 레지스트리 키를 읽을 수 없습니다: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-drivers-unreadable = 드라이버를 검색하기 위한 서비스 레지스트리 키를 읽을 수 없습니다: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-queried = 바로 가기를 쿼리할 수 없습니다: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-parsed = 바로 가기 쿼리 출력을 구문 분석할 수 없습니다: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-environment-unreadable = 시스템 환경 변수 레지스트리 키를 읽을 수 없습니다: { $error }

## What a package records of how a driver is put back, which an installer shows. { $kind } is the driver's type as
## Windows names it (kernel, file-system, recognizer, or unknown-type); { $name } is its service's name; { $path } is
## its file. { $signed } is yes when { $signer } names who signed it, and no when that is not known.

core-service-driver-bundled =
    { $signed ->
        [yes] 앱 소유 { $kind } 드라이버 '{ $name }'({ $path }, 서명자: { $signer }). 설치 관리자가 .sys 파일을 포함하고 드라이버 서비스를 자동으로 다시 등록한 다음(sc create, 관리자 권한 필요), 서비스가 등록되었는지 확인합니다.
       *[no] 앱 소유 { $kind } 드라이버 '{ $name }'({ $path }, 서명 알 수 없음). 설치 관리자가 .sys 파일을 포함하고 드라이버 서비스를 자동으로 다시 등록한 다음(sc create, 관리자 권한 필요), 서비스가 등록되었는지 확인합니다.
    }
core-service-driver-from-vendor =
    { $signed ->
        [yes] 공급업체/기본 제공 { $kind } 드라이버 '{ $name }'({ $path }, 서명자: { $signer }). WinAppMigrator는 타사 드라이버를 재배포하지 않습니다. 앱을 실행하기 전에 대상 PC에서 공급업체의 서명된 드라이버 패키지로 다시 설치하세요.
       *[no] 공급업체/기본 제공 { $kind } 드라이버 '{ $name }'({ $path }, 서명 알 수 없음). WinAppMigrator는 타사 드라이버를 재배포하지 않습니다. 앱을 실행하기 전에 대상 PC에서 공급업체의 서명된 드라이버 패키지로 다시 설치하세요.
    }

## Runtimes an app needs, such as the Visual C++ runtime or .NET: how each is provided, as the Dependencies list says.

core-runtime-handling-built-in = Windows에 기본 제공
core-runtime-handling-ships-with-app = 앱과 함께 제공
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-handling-windows-feature = Windows 기능: { $feature }
core-runtime-handling-included = 포함됨
# { $id } is the runtime's name in winget, such as Microsoft.VCRedist.2015+.x64.
core-runtime-handling-winget = winget으로 설치: { $id }
core-runtime-handling-manual = 수동으로 설치

## Downloading a runtime's installer, when an installer is built. Each is why it could not be, and starts with a small
## letter, as it follows other words. { $url } is a web address; { $error } is what went wrong.

core-runtime-download-not-https = { $url }을(를) 다운로드하지 않습니다: HTTPS가 아닙니다
# TLS is what keeps a download private.
core-runtime-tls-failed = TLS 초기화에 실패했습니다: { $error }
core-runtime-download-failed = { $url } 다운로드에 실패했습니다: { $error }
core-runtime-download-unreadable = { $url }을(를) 읽지 못했습니다: { $error }
core-runtime-download-empty = { $url } 다운로드에서 데이터가 반환되지 않았습니다
# { $came } and { $expected } are numbers of bytes.
core-runtime-download-cut-short = { $url } 다운로드가 중간에 끊겼습니다: { $expected }바이트 중 { $came }바이트만 받았습니다

## Installing a runtime: why it was not installed. Each starts with a small letter, as it follows other words.

core-runtime-no-known-installer = 이 런타임에 대해 알려진 설치 관리자가 없습니다
# { $path } is where in the package, such as prerequisites\vc_redist.x64.exe.
core-runtime-payload-misplaced = 패키지에서 { $path }을(를) 이 런타임의 설치 관리자로 지정하고 있지만 올바른 위치가 아니므로 실행하지 않았습니다
# { $path } is a file.
core-runtime-payload-missing = 포함된 설치 관리자가 없습니다: { $path }
# { $error } is what went wrong.
core-runtime-payload-path-invalid = 포함된 설치 관리자 경로가 잘못되었습니다: { $error }
# { $url } is the web address it can be downloaded from. winget is a program's name.
core-runtime-no-installer-route = 포함된 설치 관리자나 winget ID가 없습니다. { $url }에서 받으세요
# { $error } is what went wrong.
core-runtime-no-temp-folder = 압축을 풀 폴더를 만들 수 없습니다: { $error }
# { $reason } is what the check found.
core-runtime-payload-signature-failed = 포함된 설치 관리자가 서명 검사에 실패하여 실행하지 않았습니다: { $reason }
core-runtime-payload-timed-out = 포함된 설치 관리자가 제한 시간 내에 완료되지 않아 종료되었습니다
# { $error } is what went wrong.
core-runtime-payload-not-started = 포함된 설치 관리자를 시작할 수 없습니다: { $error }
core-runtime-installer-killed = 설치 관리자가 종료 코드 없이 종료되었습니다
# { $code } is the number it ended with, such as 1603.
core-runtime-installer-failed = 설치 관리자가 종료되었습니다(종료 코드 { $code })
# { $feature } is a Windows feature's name, such as NetFx3; { $code } is the number DISM ended with. Windows Features is
# the name of a Windows window. Keep the command as it is.
core-runtime-feature-failed = DISM에서 { $feature } Windows 기능을 사용하도록 설정할 수 없습니다(종료 코드 { $code }). 페이로드를 다운로드하지 못했을 가능성이 큽니다. "Windows 기능"에서 사용하도록 설정하거나 `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`를 실행하세요.
# The same, when DISM gave no number.
core-runtime-feature-failed-no-code = DISM에서 { $feature } Windows 기능을 사용하도록 설정할 수 없습니다(종료 코드 알 수 없음). 페이로드를 다운로드하지 못했을 가능성이 큽니다. "Windows 기능"에서 사용하도록 설정하거나 `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`를 실행하세요.
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-feature-timed-out = { $feature } Windows 기능을 사용하도록 설정하는 작업이 제한 시간 내에 완료되지 않아 종료되었습니다
# { $feature } is a Windows feature's name; { $error } is what went wrong.
core-runtime-feature-no-dism = { $feature } Windows 기능을 사용하도록 설정하는 데 필요한 DISM을 사용할 수 없습니다: { $error }

## Installing a runtime with winget. { $id } is its name in winget, such as Microsoft.VCRedist.2015+.x64.

# { $error } is what went wrong.
core-runtime-winget-not-looked-for = winget을 찾을 수 없습니다: { $error }
core-runtime-winget-not-installed = winget이 설치되어 있지 않습니다
# { $path } is where winget is.
core-runtime-winget-misplaced = { $path }은(는) Windows가 자체 패키지를 보관하는 위치가 아니므로 실행하지 않았습니다
# { $path } is where winget is; { $reason } is what the check found.
core-runtime-winget-signature-failed = { $path }이(가) 서명 검사에 실패하여 실행하지 않았습니다: { $reason }
# { $reason } is one of the two messages above, or what went wrong.
core-runtime-winget-unusable = winget을 사용하여 { $id }을(를) 설치할 수 없습니다: { $reason }
# { $code } is the number winget ended with.
core-runtime-winget-failed = winget install { $id } 명령이 종료되었습니다(종료 코드 { $code })
core-runtime-winget-failed-no-code = winget install { $id } 명령이 알 수 없는 코드로 종료되었습니다
core-runtime-winget-timed-out = winget install { $id } 명령이 제한 시간 내에 완료되지 않아 종료되었습니다
# { $error } is what went wrong.
core-runtime-winget-not-started = { $id }을(를) 설치하는 데 필요한 winget을 사용할 수 없습니다: { $error }

## What told a scan how an app was installed, as the package's report lists it. The names of installers and files are
## as their makers write them.

# { $path } is an installer's record of what it installed, such as C:\Program Files\App\unins000.dat.
core-source-record-not-believed = { $path }을(를) 사용하지 않았습니다: 관리자가 아닌 사용자가 변경할 수 있습니다
# { $count } is how many Windows Installer packages the bundle holds.
core-source-burn-bundle =
    { $count ->
       *[other] Windows Installer 패키지 { $count }개로 구성된 Burn 번들
    }
# { $count } is how many paths Windows Installer recorded.
core-source-msi-database =
    { $count ->
       *[other] Windows Installer 데이터베이스: 구성 요소 키 경로 { $count }개
    }
core-source-inno-log = Inno Setup 제거 로그(unins000.dat)
# { $count } is how many paths the log records.
core-source-inno-log-paths =
    { $count ->
       *[other] Inno Setup 제거 로그: 기록된 경로 { $count }개
    }
core-source-nsis = NSIS(Nullsoft) 제거 프로그램 서명
# { $count } is how many paths the log records.
core-source-nsis-log-paths =
    { $count ->
       *[other] NSIS AdvUninstLog uninstall.log: 기록된 경로 { $count }개
    }
core-source-squirrel = Squirrel 사용자별 레이아웃(Update.exe / app-*)
# { $version } is a folder's name that says the version, such as app-1.2.3.
core-source-squirrel-version = Squirrel 현재 버전: { $version }
core-source-chocolatey = Chocolatey 패키지 메타데이터
core-source-msix = MSIX / Appx 패키지
# { $identity } is a package's name, such as Contoso.App.
core-source-appx-identity = AppxManifest ID: { $identity }
# { $count } is how many folders.
core-source-msix-data =
    { $count ->
       *[other] MSIX 사용자별 패키지 데이터: 위치 { $count }개
    }
# App Paths is a registry key's name.
core-source-app-paths = App Paths 등록

## Why a runtime is needed, as the package's report says it.

# { $program } is a program's file name, such as app.exe; { $dll } is a DLL's, such as msvcp140.dll.
core-runtime-program-imports = { $program }이(가) { $dll }을(를) 가져옵니다

## What else of an app's a scan finds in the registry: fonts and certificates, and why each is the app's.

# Why a font is among the app's files, in the package's list of them.
core-registry-font-recorded = 앱의 설치 기록에 나와 있는 글꼴
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-named-for-the-app = 앱 또는 앱 게시자의 이름이 붙어 있습니다
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-recorded = 앱의 설치 기록에 나와 있습니다

## Who is signed in, for the Whose Data choice. Each of the first six is why it could not be told, and starts with a
## small letter, as it is shown inside other words. An account is shown by its id, such as S-1-5-21-...-1001.

# { $error } is what Windows said.
core-profile-desktop-owner-not-said = Windows에서 이 데스크톱의 소유자를 알려 주지 않았습니다: { $error }
# { $error } is what went wrong.
core-profile-desktop-owner-not-asked = Windows에 이 데스크톱의 소유자를 물어볼 수 없습니다: { $error }
core-profile-signed-in-account-no-profile = 여기에 로그인한 계정 { $account }에 기록된 프로필이 없습니다
core-profile-desktop-not-a-person = 이 데스크톱은 { $account } 계정의 것인데, 이 계정은 사람이 사용하는 계정이 아닙니다
core-profile-nobody-signed-in-running-account-no-profile = 여기에 로그인한 사용자가 없으며, 이 프로그램을 실행 중인 계정 { $account }에 기록된 프로필이 없습니다
core-profile-nobody-signed-in-running-not-a-person = 여기에 로그인한 사용자가 없으며, 이 프로그램을 실행 중인 { $account } 계정은 사람이 사용하는 계정이 아닙니다
# SID is the name Windows gives an account's id.
core-profile-nobody-signed-in-no-sid = 여기에 로그인한 사용자가 없으며, 이 계정에는 SID가 없습니다
# { $accounts } is a list of accounts.
core-profile-several-desktops = 이 세션에 데스크톱이 있는 계정이 둘 이상입니다: { $accounts }
# Under the Whose Data choice. { $reason } is one of the messages above.
core-profile-signed-in-unknown = 로그인한 사용자를 확인할 수 없습니다({ $reason }).
# Under the Whose Data choice, when there are accounts to choose from. { $reason } is one of the messages above.
core-profile-signed-in-unknown-choose = 로그인한 사용자를 확인할 수 없습니다({ $reason }). 포함할 데이터의 계정을 선택하세요.
