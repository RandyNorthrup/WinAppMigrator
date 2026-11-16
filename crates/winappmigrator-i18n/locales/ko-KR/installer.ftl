# Words of the installer that every built installer carries, which runs on the PC an app is moved to and speaks its
# language, and of the uninstaller it leaves behind for Settings. Message names start with installer- for the
# installer's window, installer-cli- for its command line, and uninstaller- for the uninstaller.
#
# The command lines (installer-cli-, uninstaller-cli-) are always shown in English, as scripts read what they print.
# Translate them all the same. Keep their command-line flags, such as --apply and --target-root, exactly as they are.


## The installer's window: its title and header

installer-window-title = WinAppMigrator 설치 관리자
# The window's title in the taskbar. { $app_name } is the name of the app it installs.
installer-window-title-app = { $app_name } 설치
installer-tagline = 다른 PC에서 캡처한 앱을 다시 설치합니다
# What an app is called when its package gives it no name.
installer-unnamed-app = 앱

## When the installer's window cannot open. Each is a paragraph of a message box.

# { $error } is what went wrong, as Windows or the graphics driver says it.
installer-window-failed = 설치 관리자 창을 열 수 없습니다: { $error }
installer-window-failed-without-one = 창 없이도 설치할 수 있습니다. 관리자 권한 명령 프롬프트에서 이 파일을 --apply 옵션으로 실행하세요. 설치할 위치를 정하려면 --target-root 뒤에 폴더를 지정하세요.

## Checking the installer when it opens

installer-checking = 설치 관리자를 확인하는 중…
installer-stub-no-package = 이 파일은 WinAppMigrator 설치 관리자 스텁이며 앱 패키지가 들어 있지 않습니다. WinAppMigrator로 설치 관리자를 만든 다음, 만들어진 .exe 파일을 여기에서 실행하세요.
installer-check-stopped = 설치 관리자 확인이 중지되었습니다.
# { $error } is what went wrong, as Windows says it.
installer-locate-failed = 설치 관리자를 찾을 수 없습니다: { $error }
# { $error } is what went wrong reading it.
installer-read-failed = 설치 관리자를 읽을 수 없습니다: { $error }
# { $error } is why, such as that the app needs a newer Windows.
installer-cannot-be-used-here = 이 설치 관리자의 패키지는 이 PC에서 사용할 수 없습니다: { $error }

## The app, where it goes, and whether the installer is signed

installer-captured-from = 캡처 원본
installer-install-location = 설치 위치
# The title of the folder picker.
installer-choose-location = 설치 위치 선택
installer-folder-has-files = 이 폴더에 이미 파일이 있습니다. 이름이 같은 파일은 대체되며, 제거하면 원래 파일이 돌아옵니다.
installer-room-needed = 필요한 공간
# { $shortfall } says which drive is short of room, such as "C: needs up to 824.0 MB for the app's files, and has 300.0 MB free".
installer-room-short = 여기에 공간이 부족할 수 있습니다: { $shortfall }. 압축은 다른 드라이브에서 풀 수 있지만, 앱 파일이 들어갈 공간은 있어야 합니다.
# { $app_name } is the name of an app whose earlier install was cut short.
installer-interrupted = 이전에 시작한 { $app_name } 설치가 완료되지 않았습니다. "설치"를 클릭하면 먼저 그 설치를 되돌립니다.
# Beside "Installer", for whether it proves who made it.
installer-signature = 설치 관리자
installer-signature-checking = 서명을 확인하는 중…
installer-signature-unsigned = 서명되지 않음: 알 수 없는 게시자
# { $publisher } is the name on the installer's certificate, such as "Fabrikam, Inc.".
installer-signature-signed-by = 서명자: { $publisher }
installer-signature-unnamed = 이름 없는 게시자가 서명함
# { $reason } is Windows' own word for what is wrong, such as HashMismatch.
installer-signature-invalid = 이 설치 관리자의 서명이 유효하지 않습니다({ $reason }). 서명된 후에 변경되었을 수 있습니다.

## Why the install location cannot be used. Install and Preview Changes stay off while one of these is shown.

# { $app_name } is the name of the app being installed.
installer-location-app-already-there = { $app_name }이(가) 이미 이 폴더에 설치되어 있으므로 이 사본을 여기에 설치할 수 없습니다. 여기에 설치하려면 먼저 Windows 설정의 앱에서 이 PC에 있는 사본을 제거한 다음, 이 설치 관리자를 다시 여세요. 또는 위에 비어 있는 새 폴더를 입력하세요.
# { $other_app } is the name of another app Windows lists as installed in the folder. { $app_name } is the name of the app being installed.
installer-location-other-app-there = 이 폴더에 { $other_app }이(가) 설치되어 있으므로 { $app_name }을(를) 여기에 설치할 수 없습니다. 위에 비어 있는 새 폴더를 입력하세요. 또는 { $other_app }이(가) 더 이상 필요하지 않으면 먼저 Windows 설정의 앱에서 제거한 다음, 이 설치 관리자를 다시 여세요.
installer-location-has-files = 이 폴더에는 WinAppMigrator가 설치하지 않은 파일이 이미 있습니다. 다른 앱의 파일일 수 있으므로 그 위에는 아무것도 설치하지 않습니다. 위에 비어 있는 새 폴더를 입력하세요.
# { $reason } is why, such as that it cannot be the Windows folder.
installer-location-refused = 이 폴더를 사용할 수 없습니다: { $reason }. 위에 다른 폴더를 입력하세요.

## What the install sets up beyond its files

installer-beyond-files = 파일 외에 설정되는 항목
# Under the heading above. It is part of what a person agrees to when they click Install.
installer-beyond-files-agreed = 이 모든 작업은 관리자 권한으로 수행됩니다. 설치에 실패하면 변경된 내용은 원래대로 되돌려지며, 되돌릴 수 없는 항목이 있으면 알려 드립니다. 나중에 설정의 설치된 앱에서 설치를 되돌릴 수도 있습니다. 설치로 대체된 파일은 원래대로 돌아오며, 설치된 런타임은 그대로 유지됩니다.
installer-beyond-files-none = 없음: 파일과 앱 자체 레지스트리 키의 설정만 복사합니다.
# { $problem } is why, such as that the app cannot run on this PC.
installer-cannot-go-here = 이 패키지는 이 위치에 설치할 수 없습니다: { $problem }
installer-working-out = 이 위치에서 설정될 항목을 확인하는 중…
# Said as the reason after "This package cannot go to this location:".
installer-list-stopped = 수행할 작업을 확인하는 과정이 중지되었습니다
# A heading over a list of what else the package asked for and will not get. { $count } is how many.
installer-more-left-out =
    { $count ->
       *[other] 제외된 항목 { $count }개 더
    }

## Options

installer-options = 옵션
installer-user-data = 사용자 데이터
installer-user-data-detail = 켬: 설정, 문서, 저장된 게임이 로그인한 사용자의 해당 폴더로 이동합니다. 끔: 모두 설치 폴더 아래에 저장되며, 거기에서 실행되도록 설정되는 항목은 없습니다.
# { $account } is the name of the account on the other PC whose data this is.
installer-user-data-whose = { $account } 계정의 데이터가 들어 있습니다.
installer-license = 라이선스
installer-license-detail = 정품 인증 및 라이선스 파일과, 라이선스 데이터가 들어 있는 레지스트리 값입니다. 일부는 이 PC에서 다시 정품 인증해야 할 수 있습니다.

## Dependencies: what else the app needs, and how each is provided

installer-dependencies = 종속성
installer-dependencies-detail = 이 앱에 필요한 항목과 각 항목이 제공되는 방법입니다.
installer-dependencies-none = 추가 런타임 종속성이 검색되지 않았습니다.
# { $feature } is the name of an optional part of Windows, such as NetFx3.
installer-dependency-windows-feature = Windows 기능: { $feature }
installer-dependency-included = 포함됨
# { $id } is the package's name in winget, such as Microsoft.VCRedist.2015+.x64.
installer-dependency-winget = winget으로 설치: { $id }
installer-dependency-manual = 수동으로 설치
installer-dependency-built-in = Windows에 기본 제공
installer-dependency-ships-with-app = 앱과 함께 제공

## Buttons, and what is said beside them

installer-install = 설치
installer-preview = 변경 내용 미리 보기
installer-cancel = 취소
installer-close = 닫기
installer-open-folder = 설치 폴더 열기
# { $error } is what went wrong, as Windows says it.
installer-open-folder-failed = 폴더를 열 수 없습니다: { $error }
installer-location-blocks = 설치 위치를 사용할 수 있을 때까지 "설치"와 "변경 내용 미리 보기"는 꺼져 있습니다. 이유는 위의 설치 위치 아래에 표시됩니다.
installer-package-blocks = 이 패키지를 이 위치에 설치할 수 없으므로 "설치"가 꺼져 있습니다. 이유는 위에 표시됩니다.
installer-list-changed = 이 위치에서 수행할 작업이 변경되어 아무것도 설치하지 않았습니다. 아래 목록을 검토하고, 원하는 내용이면 "설치"를 다시 클릭하세요.
installer-cancelled-early = 취소되었습니다. 아무것도 변경되지 않았습니다.

## How far an install has got, beside its progress bar

# { $step } is one of the steps below. { $done } and { $total } are how many of its items are done, and how many there are.
installer-step-counted = { $step }: { $total }개 중 { $done }개…
# { $step } is one of the steps below, when it does not count its items.
installer-step-uncounted = { $step }…
installer-step-getting-ready = 준비하는 중…
installer-step-cancelling = 현재 단계를 마친 후 취소하는 중…
installer-step-putting-back = 변경된 내용을 되돌리는 중
installer-step-unpacking = 압축을 푸는 중
installer-step-checking-package = 패키지를 확인하는 중
installer-step-recovering = 완료되지 않은 이전 설치를 되돌리는 중
# What a preview is doing, whatever step it is on.
installer-step-previewing = 변경될 내용을 확인하는 중
installer-step-runtimes = 런타임을 설치하는 중
installer-step-files = 파일을 복사하는 중
installer-step-services = 서비스를 설정하는 중
installer-step-drivers = 드라이버를 설정하는 중
installer-step-registry = 레지스트리 설정을 쓰는 중
installer-step-license = 라이선스 데이터를 복원하는 중
installer-step-tasks = 예약된 작업을 설정하는 중
installer-step-shortcuts = 바로 가기를 만드는 중
installer-step-environment = 환경 변수를 설정하는 중
installer-step-settings-entry = 설정에 앱을 추가하는 중
installer-step-verifying = 설치된 항목을 확인하는 중
installer-step-undoing = 되돌리는 중

## When an install cannot go ahead or stops

installer-carries-no-package = 이 설치 관리자에는 패키지가 없습니다.
installer-stopped-unexpectedly = 설치가 예기치 않게 중지되었습니다.
# { $reason } is what went wrong, as the program says it.
installer-stopped-unexpectedly-because = 설치가 예기치 않게 중지되었습니다: { $reason }. 설치가 이미 시작되었다면, 설치 관리자를 다시 실행할 때 먼저 그 작업을 되돌립니다.
# { $error } is what went wrong reading it.
installer-package-read-failed = 패키지를 읽을 수 없습니다: { $error }
installer-no-programdata = 이 PC의 ProgramData 폴더를 찾을 수 없어 아무것도 설치하지 않았습니다. 설치를 되돌리는 데 쓰이는 설치 기록을 이 폴더에 보관하기 때문입니다.
# { $error } is why, such as that there is nowhere only administrators can get into.
installer-staging-failed = 준비 폴더를 만들 수 없습니다: { $error }
# { $error } is what went wrong unpacking it.
installer-unpack-failed = 패키지의 압축을 풀 수 없습니다: { $error }
installer-staging-nowhere = 관리자만 액세스할 수 있는 위치 중 패키지의 압축을 풀 수 있는 곳이 없습니다
# { $failure } is a folder and what went wrong there, such as "C:\ProgramData\WinAppMigrator: Access is denied.".
installer-staging-nowhere-look-at = 관리자만 액세스할 수 있는 위치 중 패키지의 압축을 풀 수 있는 곳이 없습니다. 확인할 위치: { $failure }

## What came of an install, or of a preview

installer-result = 설치 결과
installer-result-preview = 미리 보기: 아직 아무것도 변경되지 않았습니다
installer-result-installed = 앱이 설치되었으며 해시 검사를 통과했습니다.
installer-result-installed-without-left-out = 앱이 설치되었으며 해시 검사를 통과했습니다. 단, 아래에 제외된 것으로 표시된 항목은 설치되지 않았습니다.
installer-result-not-all-put-back = 설치에 실패했으며, 변경된 내용을 모두 되돌리지는 못했습니다. 남은 항목은 아래에 표시되며, 다음 설치 때 다시 되돌리려고 시도합니다.
installer-result-cancelled-put-back = 설치가 취소되었으며, 변경된 내용은 원래대로 되돌려졌습니다.
installer-result-failed-put-back = 설치에 실패했으며, 변경된 내용은 원래대로 되돌려졌습니다.
installer-result-not-clean = 설치가 정상적으로 완료되지 않았습니다. 아래 문제를 검토하세요.
installer-result-changes-put-back = 되돌린 변경 내용
installer-result-changes-made = 변경된 내용
installer-result-changes-planned = 예정된 변경 내용
installer-result-warnings = 경고
installer-result-errors = 오류
installer-result-package-valid = 패키지 해시 유효함
installer-result-package-invalid = 패키지 해시 유효하지 않음
installer-result-files-valid = 설치된 파일 유효함
installer-result-files-invalid = 설치된 파일 유효하지 않음
installer-result-reboot = 종속성 설치를 완료하려면 다시 시작해야 합니다. Windows를 다시 시작한 다음 앱을 실행하세요.
# A heading over the list of what was changed. { $count } is how many.
installer-result-changes = 변경 내용({ $count })
# After the first part of a long list. { $count } is how many more there are.
installer-result-changes-more = 외 { $count }개.
# Over the path of the install's log file.
installer-log = 로그

## The installer's command line. Shown in English: scripts read it. A { $count } is plain digits, with no separators.

installer-cli-stub = WinAppMigrator 설치 관리자 스텁 — 앱 패키지가 들어 있지 않습니다.
installer-cli-stub-how = WinAppMigrator 앱으로 설치 관리자를 만든 다음, 만들어진 .exe 파일을 대상 PC에서 실행하세요.
installer-cli-usage-title = WinAppMigrator 자동 압축 풀기 설치 관리자.
installer-cli-usage = 그래픽 설치 관리자를 열려면 인수 없이 실행하세요. 또는 다음과 같이 실행하세요.
installer-cli-usage-preview = 다시 설치 미리 보기: --dry-run [--target-root <DIR>]
installer-cli-usage-apply = 다시 설치:           --apply [--target-root <DIR>]
installer-cli-no-package = 설치 관리자에 포함된 패키지가 없습니다
installer-cli-no-programdata = 이 PC의 ProgramData 폴더를 찾을 수 없어 아무것도 설치하지 않았습니다. 설치를 되돌리는 데 쓰이는 설치 기록을 이 폴더에 보관하기 때문입니다
# { $path } is the folder asked for, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-folder-has-files = { $path }을(를) 사용할 수 없습니다: { $problem }. 이 앱의 폴더임이 확실한 경우에만 --force를 지정하세요
installer-cli-no-location = 이 패키지에는 앱이 있던 위치가 기록되어 있지 않으므로 --target-root를 지정하세요
# { $path } is where the app was on the other PC, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-location-refused = 이 패키지의 앱은 { $path }에 있었지만 이 위치는 사용할 수 없습니다: { $problem }. --target-root를 지정하세요
# { $summary } is how much room it needs where, such as "up to 824.0 MB on C: (356.0 GB free)".
installer-cli-room = 공간: { $summary }
# { $note } is something to know about the install.
installer-cli-note = 참고: { $note }
# { $count } is how many things it sets up beyond its files.
installer-cli-sets-up = 설정 항목: { $count }
installer-cli-applied = 다시 설치를 적용했습니다.
installer-cli-previewed = 다시 설치를 미리 보았습니다.
# { $count } is how many changes the install logged.
installer-cli-changes-logged = 기록된 변경 내용: { $count }
# { $count } is how many warnings.
installer-cli-warnings = 경고: { $count }
# { $count } is how many errors.
installer-cli-errors = 오류: { $count }
# { $count } is how many things the package asked for and did not get.
installer-cli-refused = 거부됨: { $count }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-package-valid = 패키지 해시 유효: { $valid }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-reinstall-valid = 다시 설치 해시 유효: { $valid }
# { $id } is the install's id, such as wam-0f3a….
installer-cli-install-id = 설치 ID: { $id }
installer-cli-put-back = 설치에 실패하여 변경된 내용을 원래대로 되돌렸습니다.
# { $path } is the log file.
installer-cli-log = 로그: { $path }
# { $path } is the log file. Printed before an error.
installer-cli-error-log = 로그: { $path }
# { $error } is what went wrong.
installer-cli-error = 오류: { $error }

## The uninstaller's window

uninstaller-window-title = 제거
# The window's title, and its heading. { $app_name } is the name of the app it uninstalls.
uninstaller-window-title-app = { $app_name } 제거
uninstaller-tagline = 설치 관리자가 수행한 작업을 되돌립니다
uninstaller-reading = 이 설치의 기록을 읽는 중…
uninstaller-stopped-unexpectedly = 제거가 예기치 않게 중지되었습니다.
uninstaller-what-will-be-removed = 제거될 항목
uninstaller-install-location = 설치 위치
uninstaller-files = 파일
uninstaller-registry-values = 레지스트리 값
uninstaller-services = 서비스
uninstaller-drivers = 드라이버
uninstaller-scheduled-tasks = 예약된 작업
uninstaller-environment = 환경 변수
uninstaller-plan-stays = 직접 추가하거나 변경한 파일은 그 파일이 있는 폴더와 함께 그대로 유지됩니다.
# { $count } is how many files the install replaced.
uninstaller-plan-coming-back =
    { $count ->
       *[other] 이 설치로 대체된 파일 { $count }개가 원래대로 돌아옵니다. 그 후 변경된 파일이 있으면 그 옆에 놓입니다.
    }
# { $runtimes } is a list of runtimes, such as "Microsoft Visual C++ Redistributable, .NET Desktop Runtime 8".
uninstaller-plan-runtimes = 설치된 런타임은 다른 프로그램에서 사용할 수 있으므로 그대로 유지됩니다: { $runtimes }.
uninstaller-options = 옵션
uninstaller-remove-user-data = 사용자 데이터 및 라이선스 데이터도 제거
# { $count } is how many items of the app's settings, data, and license there are.
uninstaller-remove-user-data-detail =
    { $count ->
       *[other] 앱의 설정, 데이터, 라이선스 항목 { $count }개입니다. 이 옵션을 켜지 않으면 그대로 유지됩니다. 이 항목이 대체했던 내용은 원래대로 돌아오며, 그 후 이 항목이 변경되었으면 그 옆에 놓입니다.
    }
uninstaller-uninstall = 제거
uninstaller-cancel = 취소
uninstaller-close = 닫기
# { $done } and { $total } are how many of the install's items are undone, and how many there are.
uninstaller-step-counted = 설치를 되돌리는 중: { $total }개 중 { $done }개…
uninstaller-step = 설치를 되돌리는 중…
uninstaller-done = 앱이 제거되었습니다.
uninstaller-done-except = 아래에 표시된 항목을 제외하고 앱이 제거되었습니다.
uninstaller-waits-for-drive = 일부 항목이 연결되지 않은 드라이브에 있습니다. 드라이브를 연결한 다음 설정에서 다시 제거하세요.
uninstaller-not-all-undone = 일부 항목을 되돌릴 수 없습니다. 해당 항목을 사용 중인 프로그램을 닫고 설정에서 다시 제거하세요.
# Over the path of the uninstall's log file.
uninstaller-log = 로그
uninstaller-undone = 되돌린 항목
uninstaller-left-in-place = 그대로 둔 항목
uninstaller-could-not-be-undone = 되돌릴 수 없는 항목
# After the first part of a long list. { $count } is how many more there are.
uninstaller-and-more = 외 { $count }개.
# Said as the reason when there is no record of the install to read.
uninstaller-no-programdata = 이 PC의 ProgramData 폴더를 찾을 수 없어 읽을 기록이 없습니다

## When the uninstaller's window cannot open. Each is a paragraph of a message box, followed by a command to type.

uninstaller-window-failed-title = WinAppMigrator 제거 프로그램
# { $error } is what went wrong, as Windows or the graphics driver says it.
uninstaller-window-failed = 제거 프로그램 창을 열 수 없습니다: { $error }
uninstaller-window-failed-quiet = 창 없이도 관리자 권한 명령 프롬프트에서 다음 명령으로 제거할 수 있습니다:
# Stands in for this program's own path in that command, if it cannot be found.
uninstaller-this-program = 이 프로그램

## The uninstaller's command line (--quiet). Shown in English: scripts read it.

# { $app_name } is the name of the app.
uninstaller-cli-uninstalling = { $app_name } 제거 중.
# { $path } is the log file.
uninstaller-cli-log = 로그: { $path }
# { $error } is what went wrong.
uninstaller-cli-error = 오류: { $error }
