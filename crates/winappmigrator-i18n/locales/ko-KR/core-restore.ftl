# Words the core writes for people as it installs an app from a package: why an install will not start, the list of
# what it did (or for a preview, what it would do) that the installer window shows, its warnings, the record it keeps of
# each install so that it can be undone, and the core's errors. Message names start with core-.
#
# The list of what an install did is short and plain, one line for each thing, as the installer window lists them:
# "copied", "restored", "would copy". Words such as --force and --apply are command-line options, and LocalService,
# SYSTEM, ProgramData, and %AppData% are as Windows names them: keep them as they are.


## Errors: what went wrong, when the core cannot go on. Each is shown on its own, or after words that say what was being
## done.

core-error-list-changed = 동의한 이후 이 설치에서 수행할 작업이 변경되어 아무것도 설치하지 않았습니다
# { $path } is a file or folder, such as C:\Program Files\App\app.exe, or words that name one, such as "the installer".
# { $error } is what Windows said went wrong.
core-error-io = { $path }에 대한 I/O 작업에 실패했습니다: { $error }
# { $detail } is what went wrong.
core-error-registry = 레지스트리를 읽지 못했습니다: { $detail }
# { $path } is a program file. { $detail } is what was wrong with it.
core-error-pe = { $path }의 PE 구문 분석에 실패했습니다: { $detail }
# { $detail } is what was wrong, already in words.
core-error-invalid-input = 입력이 잘못되었습니다: { $detail }
# { $detail } is what went wrong.
core-error-serialization = 보고서를 직렬화하지 못했습니다: { $detail }
# { $detail } is what went wrong.
core-error-archive = 보관 파일 작업에 실패했습니다: { $detail }
# { $detail } is what went wrong.
core-error-network = 네트워크 작업에 실패했습니다: { $detail }
# { $detail } is what went wrong.
core-error-time-format = 시간 형식을 지정하지 못했습니다: { $detail }
core-error-cancelled = 취소되었습니다
# What is said when part of the program stopped unexpectedly and gave no reason.
core-error-panic-no-reason = 이유가 제공되지 않았습니다

## The record of each install, kept so that it can be undone.

# { $kind } is a number that says what type of registry value it is.
core-journal-unknown-value-type = 알 수 없는 레지스트리 값 형식입니다: { $kind }
core-journal-not-elevated = 관리자 권한으로 실행되고 있지 않습니다
core-journal-needs-admin = 설치하려면 관리자 권한이 필요합니다. 다른 사용자가 변경할 수 없는 위치에 설치 기록을 보관하기 위해서입니다
# { $path } is a folder.
core-journal-not-an-install-folder = 이 폴더는 설치 기록 폴더가 아닙니다: { $path }
# { $path } is the file the record is kept in.
core-journal-not-written = 이 설치의 기록을 쓸 수 없습니다: { $path }
# { $path } is a folder.
core-journal-path-escapes-root = 경로가 지정된 루트를 벗어납니다: { $path }
# { $path } is a folder.
core-journal-nothing-exists = 이 PC에 이 항목이 하나도 없습니다: { $path }
core-journal-busy = 다른 설치 또는 제거가 실행 중입니다. 완료될 때까지 기다린 후 다시 시도하세요
# Said of a file that was to get back the permissions it had.
core-journal-no-folder = 폴더가 없습니다
# Said of the permissions a file had, which were kept to give back to it.
core-journal-permissions-unreadable = 보관된 사용 권한을 읽을 수 없습니다
# { $folder } is the folder of one install's record.
core-journal-unreadable-empty = { $folder }의 설치 기록을 사용할 수 없습니다: 기록이 비어 있습니다
# { $folder } is the folder of one install's record.
core-journal-unreadable-no-header = { $folder }의 설치 기록을 사용할 수 없습니다: 무엇에 대한 기록인지 나타내는 내용으로 시작하지 않습니다
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-version = { $folder }의 설치 기록을 사용할 수 없습니다: 이 프로그램의 다른 버전에서 작성한 기록입니다
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-install = { $folder }의 설치 기록을 사용할 수 없습니다: 다른 설치의 기록이라고 표시되어 있습니다
# { $id } is an install's id, such as wam-1a2b3c.
core-journal-not-an-install-id = 설치 ID가 아닙니다: { $id }

## Why an install will not start, or stopped before it changed anything.

core-restore-apply-and-dry-run = --apply와 --dry-run을 함께 지정할 수 없습니다
# { $path } is the folder that was chosen to install in.
core-restore-location-not-full-path = 설치 위치는 전체 경로여야 합니다: "{ $path }"
# { $problem } says what is wrong with the folder, already in words. { $path } is the folder.
core-restore-location-unusable = 해당 설치 위치를 사용할 수 없습니다: { $problem }: "{ $path }"
core-restore-needs-admin-plan = 복원 계획에 관리자만 수행할 수 있는 레지스트리, 서비스, 예약된 작업, 바로 가기 또는 보호된 대상의 변경이 포함되어 있습니다
# { $path } is the folder that was chosen to install in.
core-restore-needs-admin-target = 대상 루트에 관리자 권한이 필요합니다: { $path }
core-restore-needs-admin-user-data = 사용자 데이터를 ProgramData 또는 기타 보호된 위치로 복원하려면 관리자 권한이 필요합니다
# { $error } is why the install stopped. { $failed } is what could not be put back, as sentences of their own.
core-restore-not-all-put-back = { $error }. 수행된 작업을 모두 되돌리지는 못했습니다: { $failed }
# { $error } is what went wrong.
core-restore-nowhere-to-keep-record = 이 설치를 되돌리는 데 필요한 기록을 안전하게 보관할 위치가 없어 아무것도 설치하지 않았습니다: { $error }
# { $path } is a file the app would have put there.
core-restore-already-exists = { $path }이(가) 이미 있으므로 아무것도 설치하지 않았습니다. 바꾸려면 --force를 사용하세요
# { $root } is a folder the app has files for. { $link } is that folder, or one it is in.
core-restore-root-under-link = 이 앱에는 { $root }에 복원할 항목이 있는데, { $link }은(는) 다른 위치를 가리키는 링크이며 다른 사용자가 변경할 수 있으므로 아무것도 설치하지 않았습니다. 그 위치에 들어갈 항목이 사용자 데이터나 라이선스 데이터라면 사용자 데이터와 라이선스 데이터 없이 설치하세요. 시작프로그램 폴더의 바로 가기라면 그 폴더를 원래 위치로 되돌리세요
core-restore-hash-check-failed = 패키지가 해시 검사에 실패하여 아무것도 설치하지 않았습니다
# { $detail } is the first thing found wrong, already in words.
core-restore-hash-check-failed-because = 패키지가 해시 검사에 실패하여 아무것도 설치하지 않았습니다: { $detail }
# { $path } is a file or folder.
core-restore-refuse-records-folder = 설치 기록을 보관하는 위치에는 쓰지 않습니다: { $path }
# { $path } is a file or folder.
core-restore-refuse-outside = 설치 위치와 알려진 데이터 폴더 밖에는 쓰지 않습니다: { $path }
# { $path } is a file.
core-restore-no-parent-folder = 대상에 상위 폴더가 없습니다: { $path }
# { $path } is where a file was to go.
core-restore-folder-in-the-way = 이 파일이 들어갈 위치에 폴더가 있습니다: { $path }
# { $path } is a file in the package.
core-restore-file-changed = { $path }이(가) 더 이상 패키지에 들어 있는 내용과 같지 않아 설치하지 않았습니다
# { $path } is a file in the package.
core-restore-source-is-link = { $path }은(는) 링크이며, 링크를 따라가지 않습니다
# { $path } is a file in the package.
core-restore-source-became-link = 여는 동안 { $path }이(가) 링크로 바뀌었습니다
# Why a service did not start, after "did not start:".
core-restore-service-stopped-by-itself = 서비스가 스스로 다시 중지되었습니다
# Why part of the app was left out, after "was left out:".
core-restore-com-class-someone-elses = 이 COM 클래스는 이미 이 PC에 다른 항목의 클래스로 등록되어 있습니다

## What an install did, one line for each thing, or for a preview, what it would do. { $source } is a file in the
## package, and { $destination } and { $path } are where it goes on this PC. { $key } is a registry key, such as
## HKLM\Software\App.

# { $package_valid } is true or false, and { $restore_valid } is true, false, or not_run: keep them as they are.
core-restore-verification-result = 복원 후 확인 package_valid={ $package_valid } restore_valid={ $restore_valid }
# { $name } is a service's name.
core-restore-started-service = 서비스 시작함: { $name }
core-restore-would-copy = 복사 예정: { $source } -> { $destination }
core-restore-copied = 복사함: { $source } -> { $destination }
core-restore-would-create-empty-folder = 빈 폴더 만들기 예정: { $path }
core-restore-created-empty-folder = 빈 폴더 만듦: { $path }
# { $value } is the name of a value in the key.
core-restore-would-restore-registry-value = 레지스트리 복원 예정: { $key } { $value }
core-restore-would-restore-registry-key = 레지스트리 복원 예정: { $key } (키)
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-would-add-firewall-rule = 방화벽 규칙 추가 예정: { $name } (대상: { $program })
# { $name } is a firewall rule's name.
core-restore-firewall-rule-already-there = 방화벽 규칙이 이미 있음: { $name }
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-added-firewall-rule = 방화벽 규칙 추가함: { $name } (대상: { $program })
# { $name } is a service's name, { $command } what it runs, and { $account } the account it runs as, such as LocalService.
core-restore-would-restore-service = 서비스 복원 예정: { $name } -> { $command } (실행 계정: { $account })
# { $name } is a runtime, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-restore-prerequisite-present = 필수 구성 요소가 이미 있음: { $name }
# { $name } is a runtime that Windows has as a Windows feature.
core-restore-would-install-prerequisite-feature = 필수 구성 요소 설치 예정: { $name } (Windows 기능)
# { $name } is a runtime whose installer is in the package.
core-restore-would-install-prerequisite-bundled = 필수 구성 요소 설치 예정: { $name } (포함된 설치 관리자)
# { $name } is a runtime that would come through winget, which is a program's name.
core-restore-would-install-prerequisite-winget = 필수 구성 요소 설치 예정: { $name } (winget)
# { $name } is a runtime.
core-restore-installed-prerequisite = 필수 구성 요소 설치함: { $name }
# { $name } is a runtime.
core-restore-installed-prerequisite-reboot = 필수 구성 요소 설치함: { $name } (다시 시작 필요)
core-restore-would-restore-license = 라이선스 복원 예정: { $source } -> { $destination }
core-restore-restored-license = 라이선스 복원함: { $source } -> { $destination }
core-restore-would-restore-license-registry = 라이선스 레지스트리 복원 예정: { $key }
# { $name } is a driver's name.
core-restore-driver-not-restored = 드라이버를 자동으로 복원하지 않음: { $name }
# { $name } is a driver's name.
core-restore-would-install-driver = 드라이버 설치 예정: { $name } -> { $destination }
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-would-restore-task = 예약된 작업 복원 예정: { $name }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-startup-shortcut = 시작프로그램 바로 가기 복원 예정: { $shortcut } -> { $target }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-shortcut = 바로 가기 복원 예정: { $shortcut } -> { $target }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-would-restore-variable =
    { $scope ->
        [user] 사용자 환경 변수 복원 예정: { $name }
       *[machine] 시스템 환경 변수 복원 예정: { $name }
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-left-variable =
    { $scope ->
        [user] 사용자 환경 변수를 그대로 둠: { $name } (이 앱에 속한 부분이 없음)
       *[machine] 시스템 환경 변수를 그대로 둠: { $name } (이 앱에 속한 부분이 없음)
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-restored-variable =
    { $scope ->
        [user] 사용자 환경 변수 복원함: { $name }
       *[machine] 시스템 환경 변수 복원함: { $name }
    }
core-restore-set-uninstaller = 제거 프로그램 설정함: { $key }
core-restore-placed-driver-image = 드라이버 이미지 배치함: { $source } -> { $destination }
# { $name } is a driver's name.
core-restore-configured-driver = 드라이버 구성함: { $name }
# { $name } is a driver's name.
core-restore-created-driver = 드라이버 만듦: { $name }
core-restore-ensured-registry-key = 레지스트리 키 준비함: { $key }
# { $value } is the name of a value in the key.
core-restore-restored-registry-value = 레지스트리 복원함: { $key } { $value }
# "(Default)" is how Windows names the value a key has with no name: write it as Registry Editor does.
core-restore-restored-registry-default = 레지스트리 복원함: { $key } (기본값)
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-configured-service = 서비스 구성함: { $name } (실행 계정: { $account })
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-created-service = 서비스 만듦: { $name } (실행 계정: { $account })
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-restored-task = 예약된 작업 복원함: { $name }
core-restore-restored-shortcut = 바로 가기 복원함: { $path }

## Warnings an install gives, and why it failed. They are shown after it, on the PC it ran on.

core-warning-restore-earlier-not-trusted = 완료되지 않은 설치의 기록을 다른 사용자가 변경할 수 있으므로 그 기록은 건드리지 않았습니다. 그 설치에서 변경한 내용은 그대로 남아 있습니다.
# { $app_name } is the app's name. { $count } is how many changes were put back.
core-warning-restore-earlier-undone =
    { $count ->
       *[other] 이전에 시작한 { $app_name } 설치가 완료되지 않아 먼저 되돌렸습니다. 변경 내용 { $count }개를 되돌렸습니다.
    }
# { $app_name } is the app's name.
core-warning-restore-earlier-never-finished = 이전에 시작한 { $app_name } 설치가 완료되지 않았습니다. 설치하면 먼저 그 설치를 되돌립니다.
core-warning-restore-user-data-real-locations = 사용자 데이터는 대상 루트가 아니라 이 PC의 실제 %AppData%/%ProgramData% 위치로 복원됩니다.
# { $error } is what went wrong.
core-warning-restore-commit-failed = 이 설치의 기록을 완료할 수 없습니다: { $error }
# { $name } is a service's name.
core-warning-restore-service-still-starting = { $name } 서비스에 시작을 요청했지만 아직 시작이 완료되지 않았습니다. 곧 시작될 수 있습니다. 다음에 이 PC가 시작될 때도 자동으로 시작됩니다.
# { $name } is a service's name. { $reason } is why it did not start, as Windows said it.
core-warning-restore-service-not-started = { $name } 서비스가 설치되었지만 시작되지 않았습니다: { $reason }. 서비스가 시작될 때까지 앱이 작동하지 않을 수 있습니다. 다음에 이 PC가 시작될 때 Windows에서 다시 시도합니다.
# { $name } is a service's name. { $reason } is why it could not be stopped.
core-warning-restore-service-not-stopped = { $name } 서비스를 먼저 중지할 수 없습니다: { $reason }. 다음에 이 PC가 시작될 때까지 이전 프로그램이 계속 실행될 수 있습니다.
core-warning-restore-install-cancelled = 설치가 취소되었습니다.
core-warning-restore-preview-cancelled = 미리 보기가 취소되었습니다.
# { $item } is what was left out, such as a file or a service's name. { $reason } is why, already in words.
core-warning-restore-left-out = { $item }이(가) 제외되었습니다: { $reason }.
# { $error } is what went wrong.
core-warning-restore-verification-failed = 복원 후 확인을 실행할 수 없습니다: { $error }
# { $runner } says what relies on the file: a service, a scheduled task, a firewall rule, a shortcut, or a registry value.
# { $path } is the file. { $name } is the name of what relies on it. { $refusal } is why the file was not replaced,
# already in words.
core-warning-restore-program-would-run =
    { $runner ->
        [service] { $path }은(는) 이 패키지가 대체할 수 있는 파일이 아니며 { $name } 서비스가 이 파일을 사용하므로 아무것도 설치하지 않았습니다: { $refusal }
        [task] { $path }은(는) 이 패키지가 대체할 수 있는 파일이 아니며 { $name } 예약된 작업이 이 파일을 사용하므로 아무것도 설치하지 않았습니다: { $refusal }
        [firewall] { $path }은(는) 이 패키지가 대체할 수 있는 파일이 아니며 { $name } 방화벽 규칙이 이 파일을 사용하므로 아무것도 설치하지 않았습니다: { $refusal }
        [shortcut] { $path }은(는) 이 패키지가 대체할 수 있는 파일이 아니며 { $name } 바로 가기가 이 파일을 사용하므로 아무것도 설치하지 않았습니다: { $refusal }
       *[registry] { $path }은(는) 이 패키지가 대체할 수 있는 파일이 아니며 { $name } 레지스트리 값이 이 파일을 사용하므로 아무것도 설치하지 않았습니다: { $refusal }
    }
core-warning-restore-destination-exists = 대상이 이미 있습니다. 덮어쓰려면 --force를 사용하세요.
# { $path } is a file. { $error } is what went wrong.
core-warning-restore-file-not-written = { $path }을(를) 쓸 수 없습니다: { $error }
core-warning-restore-not-a-folder = 이 빈 폴더가 들어갈 위치에 폴더가 아닌 다른 항목이 있습니다.
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-not-looked-at = { $path }을(를) 확인할 수 없습니다: { $error }
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-folder-not-made = { $path }을(를) 만들 수 없습니다: { $error }
# Why part of the app was left out, after "was left out:".
core-warning-restore-registration-exists = 이 PC에 이미 같은 항목이 다른 값으로 설정되어 있으며, 설치 시 이를 대신 차지하지 않습니다
core-warning-restore-registry-review = 레지스트리 값은 보관된 구조화 데이터에서 복원되었으며, 알려진 앱 경로는 대상 루트에 맞게 다시 작성되었습니다.
# { $error } is what went wrong.
core-warning-restore-firewall-unreadable = Windows 방화벽 규칙을 읽을 수 없습니다: { $error }
# Why a firewall rule was left out, after "was left out:".
core-warning-restore-firewall-rule-exists = 이 프로그램에 대해 같은 이름의 규칙이 이미 다른 설정으로 있으며, 설치 시 이를 변경하지 않습니다
# { $name } is a runtime.
core-warning-restore-prerequisite-unknown = { $name }: 이 런타임에 대해 알려진 설치 관리자가 없습니다.
# { $name } is a runtime.
core-warning-restore-prerequisite-needs-admin = { $name }을(를) 설치하려면 관리자 권한이 필요합니다. 관리자 권한으로 설치 관리자를 다시 실행하세요.
# { $name } is a runtime.
core-warning-restore-prerequisite-reboot = { $name }이(가) 설치되었지만 설치를 완료하려면 다시 시작해야 합니다.
# { $error } is what went wrong.
core-warning-restore-skipped-not-recorded = 건너뛴 런타임을 기록할 수 없습니다: { $error }
# { $name } is a runtime. { $reason } is why it was skipped, already in words.
core-warning-restore-prerequisite-skipped = { $name }: { $reason }
# { $name } is a runtime. { $reason } is why it could not be installed. { $url } is a web address to get it from.
core-warning-restore-prerequisite-failed = { $name }을(를) 설치할 수 없습니다: { $reason }. { $url }에서 받으세요.
# { $error } is what went wrong.
core-warning-restore-runtime-not-recorded = 설치된 런타임을 기록할 수 없습니다: { $error }
# { $error } is what went wrong.
core-warning-restore-license-source-invalid = 패키지에 포함된 라이선스 경로가 잘못되었습니다: { $error }
core-warning-restore-license-needs-admin = 라이선스 데이터를 보호된 위치로 복원하려면 관리자 권한이 필요합니다.
core-warning-restore-license-exists = 라이선스 대상이 이미 있습니다. 덮어쓰려면 --force를 사용하세요.
# { $error } is what went wrong.
core-warning-restore-license-failed = 라이선스 데이터를 복원할 수 없습니다: { $error }
core-warning-restore-driver-needs-admin = 드라이버를 등록하려면 관리자 권한이 필요합니다. 관리자 권한으로 설치 관리자를 다시 실행하세요.
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-warning-restore-task-runs-as-system = { $name } 예약된 작업은 이 PC에 대한 모든 권한을 가진 SYSTEM으로 실행됩니다.
core-warning-restore-shortcut-exists = 바로 가기가 이미 있습니다. 덮어쓰려면 --force를 사용하세요.
core-warning-restore-shortcut-all-users = 복원된 바로 가기 중 하나 이상이 공용 바탕 화면이나 공용 시작 메뉴 위치를 대상으로 합니다.
core-warning-restore-environment-refresh = 환경 변수 변경 내용을 적용하려면 새 프로세스, 셸 또는 로그인 세션이 필요할 수 있습니다.
# { $name } is a service's name. { $account } is the account it ran as on the PC it came from.
core-warning-restore-service-account = { $name } 서비스는 이전 PC에서 { $account } 계정으로 실행되었습니다. 암호는 설치 관리자에 들어 있지 않으므로 이 서비스는 LocalService로 설정됩니다. 서비스가 작동하지 않으면 "서비스"에서 계정과 암호를 설정하세요.
# { $name } is a runtime.
core-warning-restore-prerequisite-pending-reboot = { $name }이(가) 아직 등록되지 않았습니다. 다시 시작하면 활성화될 것입니다.
# { $name } is a runtime.
core-warning-restore-prerequisite-not-verified = { $name }이(가) 성공했다고 보고했지만 등록되지 않았습니다. 설치에 실패했을 수 있습니다.
# { $error } is what went wrong.
core-warning-restore-driver-source-invalid = 포함된 드라이버 경로가 잘못되었습니다: { $error }
# { $error } is what went wrong.
core-warning-restore-driver-not-read = 드라이버의 현재 설정을 읽을 수 없습니다: { $error }
core-warning-restore-driver-exists = 드라이버 서비스가 이미 있습니다. 다시 구성하려면 --force를 사용하세요.
core-warning-restore-driver-image-exists = 드라이버 이미지가 이미 있습니다. 덮어쓰려면 --force를 사용하세요.
core-warning-restore-driver-hash-bundled = 포함된 드라이버 이미지가 기록된 해시와 일치하지 않아 설치하지 않았습니다.
# { $error } is what went wrong.
core-warning-restore-driver-not-placed = 드라이버 이미지를 배치할 수 없습니다: { $error }
core-warning-restore-driver-hash-placed = 드라이버 이미지가 배치된 후 변경되어 등록하지 않았습니다.
# { $error } is what went wrong.
core-warning-restore-key-not-created = 레지스트리 키를 만들 수 없습니다: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-read = 대체될 값을 읽을 수 없습니다: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-written = 레지스트리 값을 쓸 수 없습니다: { $error }
core-warning-restore-value-exists = 레지스트리 값이 이미 있습니다. 덮어쓰려면 --force를 사용하세요.
# { $error } is what went wrong.
core-warning-restore-environment-key-not-created = 환경 변수 레지스트리 키를 만들 수 없습니다: { $error }
core-warning-restore-variable-exists = 환경 변수가 이미 있습니다. 덮어쓰려면 --force를 사용하세요.
# { $error } is what went wrong.
core-warning-restore-variable-not-written = 환경 변수를 쓸 수 없습니다: { $error }
core-warning-restore-value-kind-missing = 레지스트리 값 형식이 없습니다.
# { $error } is what went wrong.
core-warning-restore-service-not-read = 서비스의 현재 설정을 읽을 수 없습니다: { $error }
core-warning-restore-service-exists = 서비스가 이미 있습니다. 다시 구성하려면 --force를 사용하세요.
# { $error } is what went wrong.
core-warning-restore-task-not-queried = 예약된 작업을 쿼리하지 못했습니다: { $error }
core-warning-restore-task-exists = 예약된 작업이 이미 있습니다. 덮어쓰려면 --force를 사용하세요.
# { $error } is what went wrong.
core-warning-restore-task-folder-not-looked-at = 작업 폴더를 확인할 수 없습니다: { $error }
# { $error } is what went wrong.
core-warning-restore-shortcut-not-put-back = 바로 가기를 만들 수 없으며, 대체되었던 바로 가기도 되돌릴 수 없습니다: { $error }
# { $error } is what went wrong.
core-warning-restore-command-not-started = 명령을 시작하지 못했습니다: { $error }
# { $output } is what the command wrote, as it wrote it.
core-warning-restore-command-failed = 명령이 실패했습니다: { $output }
# { $error } is what went wrong.
core-warning-restore-powershell-not-started = PowerShell을 시작하지 못했습니다: { $error }
