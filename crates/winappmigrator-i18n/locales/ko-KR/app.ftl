# Words of the app, WinAppMigrator itself: its one window, with the installed apps listed on the left and, on the
# right, what goes in an installer of the one picked and the button that builds it; and the steps to read another PC
# through WinAppMigrator Client. Message names start with app-.


## The window and its header

# The program's name, in the window's title bar and at the top of the window. Keep it as it is unless your language
# writes product names in a script of its own.
app-name = WinAppMigrator
app-tagline = 설치된 앱을 자동 압축 풀기 설치 관리자로 만듭니다
app-theme-to-light = 밝은 테마로 전환
app-theme-to-dark = 어두운 테마로 전환
app-reload-list = 앱 목록 새로 고침
app-list-loading = 로드 중
# { $count } is how many apps are listed.
app-list-count =
    { $count ->
       *[other] 앱 { $count }개
    }
app-list-load-failed = 로드 실패
# Shown when the window cannot open at all. { $error } is what went wrong, in the words of Windows or a graphics driver.
app-window-could-not-open = 창을 열 수 없습니다: { $error }

## The list of installed apps

app-list-search = 이름 또는 게시자로 검색  (Ctrl+F)
app-list-filter-all = 모두
# Whether an app is installed for one account or for everyone on the PC.
app-scope-per-user = 사용자별
app-scope-per-machine = 컴퓨터별
app-scope-unknown = 알 수 없음
# An app whose processor architecture (x86, x64, ARM64) is not known.
app-arch-unknown = 알 수 없음
app-list-loading-apps = 설치된 앱을 로드하는 중…
# { $error } is why the apps could not be listed.
app-list-could-not-list = 앱 목록을 가져올 수 없습니다: { $error }
app-list-no-matches = 일치하는 앱 없음
app-list-no-matches-hint = 검색어나 범위 필터를 조정하세요.
app-list-worker-disconnected = 앱 목록 작업자의 연결이 끊어졌습니다.
# What a screen reader says of an app's row. { $name } is the app's name, { $publisher } who made it, and { $version } its
# version, such as 11.4.2, or — when either is not known.
app-list-row = { $name }, { $publisher }, 버전 { $version }

## The list's columns, and what a screen reader says each column's heading does when clicked

app-column-name = 이름
app-column-name-sort = 이름으로 정렬
app-column-publisher = 게시자
app-column-publisher-sort = 게시자로 정렬
app-column-version = 버전
app-column-version-sort = 버전으로 정렬
app-column-scope = 범위
app-column-scope-sort = 범위로 정렬
# Short for architecture: x86, x64, or ARM64. The column is narrow.
app-column-arch = 플랫폼
app-column-arch-sort = 플랫폼으로 정렬

## Where apps are read from: this PC, a drive from another PC, a disk image of one, or another PC on the network

app-source = 검사 원본
app-source-this-pc = 이 PC
app-source-drive = 연결된 드라이브…
app-source-image = 디스크 이미지…
app-source-network = 네트워크 PC…
app-source-attaching-image = 이미지를 읽기 전용으로 연결하는 중…
# { $computer } is the other PC's name, such as OLD-PC, and { $address } its network address, such as 192.168.1.20.
app-source-network-pc = 네트워크의 { $computer }({ $address })
app-source-disconnect = 연결 끊기
app-source-change = 변경…
# The title of the window that picks a disk image. .vhd and .vhdx are file extensions.
app-source-pick-image-title = 다른 PC의 디스크 이미지 선택(.vhd 또는 .vhdx)
app-source-pick-image-filter = 디스크 이미지
# { $path } is the disk image file.
app-source-image-has-no-windows = { $path }에 Windows가 있는 볼륨이 없습니다.
app-source-image-not-attached = 디스크 이미지를 연결할 수 없습니다.
app-source-pick-drive-title = 연결된 드라이브의 루트 선택(예: E:\)
# { $path } is the folder picked, such as E:\. \Windows and \Users are folder names, as Windows writes them.
app-source-not-a-windows-drive = { $path }은(는) Windows 시스템 드라이브가 아닌 것 같습니다(\Windows 및 \Users 없음). 다른 PC에서 가져온 드라이브의 루트를 선택하세요.
# Said of a drive or disk image with a Windows older than Windows 10. { $windows } is its name, such as "Windows 7
# Professional Service Pack 1, 32-bit". { $is_32_bit } is "yes" when that Windows is 32-bit, which adds the last
# sentence, and "no" when it is not.
app-source-older-windows =
    여기에는 { $windows }이(가) 있습니다. 여기에서 만든 설치 관리자는 Windows 10 및 11용입니다. 사용자 데이터를 포함하면 Windows가 오래된 앱을 위해 따로 보관해 둔 설정도 함께 옮겨지며, Windows 11에 없는 런타임(예: DirectX 9)은 앱과 함께 설치됩니다.{ $is_32_bit ->
        [yes] { " " }이 Windows의 앱은 32비트이므로, 64비트 Windows가 32비트 앱을 두는 Program Files (x86)에 설치됩니다.
       *[no] { "" }
    }
# The same, of a Windows whose name could not be read. { $is_32_bit } is as above.
app-source-older-windows-unnamed =
    여기에는 이전 버전의 Windows가 있습니다. 여기에서 만든 설치 관리자는 Windows 10 및 11용입니다. 사용자 데이터를 포함하면 Windows가 오래된 앱을 위해 따로 보관해 둔 설정도 함께 옮겨지며, Windows 11에 없는 런타임(예: DirectX 9)은 앱과 함께 설치됩니다.{ $is_32_bit ->
        [yes] { " " }이 Windows의 앱은 32비트이므로, 64비트 Windows가 32비트 앱을 두는 Program Files (x86)에 설치됩니다.
       *[no] { "" }
    }

## The details panel, before an app is picked, and the app picked

app-details-pair-first = 이전 PC와 페어링
app-details-pair-first-hint = 이전 PC에서 WinAppMigrator Client를 실행하고 표시된 코드를 입력하면, 그 PC의 앱이 여기에 표시됩니다. 이 PC의 앱과 같은 방법으로 설치 관리자를 만들 수 있습니다.
app-details-pick-an-app = 앱 선택
app-details-pick-an-app-hint = 목록에서 앱을 선택하여 포함할 항목을 고르고 설치 관리자를 만드세요.
# { $publisher } is who made the app, and { $version } its version, such as 11.4.2.
app-details-publisher-and-version = { $publisher }, 버전 { $version }
# { $version } is the app's version, such as 11.4.2.
app-details-unknown-publisher-and-version = 알 수 없는 게시자, 버전 { $version }
app-details-unknown-publisher = 알 수 없는 게시자

## What goes in besides the program

app-option-license = 라이선스 및 정품 인증 데이터
app-option-license-hint = 이 PC에 종속되어 있어 새 PC에서 다시 정품 인증해야 할 수 있습니다.
app-option-user-data = 사용자 데이터
# Documents, Saved Games, and ProgramData are folders of Windows: use the names Windows gives them in your language,
# except ProgramData, which keeps its name.
app-option-user-data-hint = 설정 및 기록, 문서, 저장된 게임, ProgramData.
app-option-whose-data = 데이터 소유자
app-option-whose-data-hint = 설치 관리자에는 한 계정의 데이터만 들어갑니다. 이 데이터는 설치 관리자를 실행하는 PC에 로그인한 사용자에게 적용됩니다.

## Building the installer

app-output-folder = 출력 폴더
# An example of a folder, shown in the empty field. "You" stands for the name of an account.
app-output-folder-example = C:\Users\사용자\WinAppMigrator
app-output-folder-not-full = 전체 폴더 경로를 입력하세요(예: D:\Installers).
app-build-create = 설치 관리자 만들기
app-build-create-again = 설치 관리자 다시 만들기
app-build-cancel = 취소
app-build-cancelling = 현재 단계를 마친 후 취소하는 중…
app-build-cancelled = 빌드가 취소되었습니다. 설치 관리자를 만들지 않았습니다.
# winappmigrator-installer.exe is a file name.
app-build-no-stub = 이 앱과 같은 위치에서 설치 관리자 스텁(winappmigrator-installer.exe)을 찾을 수 없어 자동 압축 풀기 설치 관리자를 만들 수 없습니다.
app-build-select-an-app-first = 먼저 옮길 앱을 선택하세요.
app-build-look-again-first = 이 앱을 빌드하려면 먼저 다시 검사해야 합니다.
# { $reason } is what went wrong, in the program's own words, often in English.
app-build-stopped-unexpectedly = 빌드가 예기치 않게 중지되었습니다: { $reason }. 출력 폴더에 설치 관리자가 남아 있으면 사용하지 마세요.
app-build-worker-disconnected = 마이그레이션 작업자의 연결이 끊어졌습니다.
# { $path } is the output folder, and { $error } what Windows said went wrong.
app-build-no-output-folder = 출력 폴더 { $path }을(를) 만들 수 없습니다: { $error }
# { $error } is what went wrong.
app-build-no-staging-folder = 보호된 준비 폴더를 만들 수 없어 아무것도 빌드하지 않았습니다: { $error }
# { $error } is what went wrong, as a sentence.
app-build-could-not-build = 설치 관리자를 만들 수 없습니다. { $error }
app-build-no-installer = 빌드가 완료되었지만 설치 관리자가 없습니다.
# { $error } is what went wrong.
app-build-hash-check-did-not-run = 설치 관리자를 만들었지만 해시 검사를 실행할 수 없습니다: { $error }. 이 설치 관리자를 사용하지 마세요.

## How far a build has got

app-progress-preparing = 포함할 항목을 확인하는 중…
app-progress-checking-room = 공간이 충분한지 확인하는 중…
app-progress-bundling = 런타임과 드라이버를 묶는 중…
# { $done } is how many files have been read so far, and { $total } how many there are.
app-progress-packing = 파일을 읽고 압축하는 중: { $total }개 중 { $done }개
# { $done } is how many of the installer's records have been looked over so far, and { $total } how many there are.
app-progress-looking-over = 설치 관리자의 내용을 검토하는 중: { $total }개 중 { $done }개…
app-progress-writing = 설치 관리자를 쓰는 중…
app-progress-verifying = 해시를 확인하는 중…

## What a build made

app-result-failed-hash-check = 이 설치 관리자를 사용하지 마세요. 해시 검사에 실패했습니다.
# { $count } is how many files were gone when the build came to read them.
app-result-files-gone =
    { $count ->
       *[other] 설치 관리자를 만들었지만, 앱을 검사한 후 파일 { $count }개가 없어져 포함되지 않았습니다. 아래 경고를 참조하세요.
    }
# { $count } is how many files could not be read into the installer.
app-result-files-not-captured =
    { $count ->
       *[other] 설치 관리자를 만들었지만, 파일 { $count }개를 캡처할 수 없었습니다. 대개 파일이 잠겨 있거나 읽을 수 없기 때문입니다. 이 파일이 없으면 앱이 작동하지 않을 수 있습니다. 아래 경고를 참조하세요.
    }
app-result-mentions-who = 설치 관리자를 만들었지만, 이 설치 관리자에는 누구의 PC에서 만들어졌는지 알 수 있는 정보가 아직 일부 남아 있습니다. 다른 사람에게 주기 전에 아래 내용을 읽어 보세요.
app-result-ready = 설치 관리자가 준비되었습니다. 패키지가 해시 검사를 통과했습니다.
app-result-files = 파일
app-result-installer-size = 설치 관리자 크기
app-result-warnings = 경고
app-result-log = 로그
app-result-copy-error = 오류 복사
app-result-installer = 설치 관리자
app-result-data-file = 데이터 파일
app-result-open-folder = 폴더 열기
app-result-copy-path = 경로 복사
# { $error } is what Windows said went wrong.
app-result-could-not-open-folder = 폴더를 열 수 없습니다: { $error }
# { $installer } and { $data_file } are file names, such as App_installer.exe and App_installer.wamdata.
app-result-two-files = 이 앱은 실행 파일 하나에 담기에는 너무 커서, 설치 관리자가 { $installer }와(과) { $data_file }의 두 파일로 되어 있습니다. 두 파일은 함께 있어야 합니다. 두 파일을 모두 다른 PC의 같은 폴더에 이 이름 그대로 복사하세요. 설치 관리자에 서명하면 그 서명은 데이터 파일에도 적용됩니다.
# { $app_name } is the app's name. "Unknown publisher" is what Windows shows when it asks to run an unsigned program: use
# Windows' own words for it in your language.
app-result-run-it = 다른 PC에서 실행하여 { $app_name }을(를) 다시 설치하세요. 설치 관리자는 SHA-256으로 파일을 확인합니다. 직접 만든 설치 관리자는 서명되어 있지 않으므로 Windows에 "알 수 없는 게시자"가 표시됩니다.
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-verification-errors = 확인 오류({ $count })
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-warnings-heading = 경고({ $count })
# After the first of a long list of warnings. { $count } is how many more there are.
app-warnings-and-more = 외 { $count }개.

## Looking at an app, to see what an installer of it would hold

app-scan-getting-ready = 이 앱을 검사할 준비를 하는 중…
app-scan-install-record = 설치된 방법을 읽는 중…
app-scan-near-matches = 이 앱의 일부일 수도 있는 항목을 찾는 중…
app-scan-system-items = 서비스, 작업, 바로 가기를 찾는 중…
app-scan-files = 파일을 살펴보는 중…
app-scan-registry = Windows와 연결되는 항목을 읽는 중…
app-scan-finishing = 마무리하는 중…
app-scan-look-again = 다시 검사
app-scan-stop = 중지
app-scan-running-hint = 검사하는 동안 다른 앱을 선택하거나 포함할 항목을 변경할 수 있습니다.
# { $error } is why, in words that follow a colon.
app-scan-failed = 이 앱을 검사할 수 없습니다: { $error }
app-scan-try-again = 다시 시도
app-scan-stopped = 중지되었습니다. 이 앱을 검사해야 설치 관리자를 만들 수 있습니다.
# { $reason } is what went wrong, in the program's own words, often in English.
app-scan-stopped-unexpectedly = 예기치 않게 중지되었습니다: { $reason }.

## How big an installer can be, and whether there is room for it

app-room-files = 파일
# Over how big the installer can be: up to that size.
app-room-up-to = 최대
app-room-free = 여유 공간
# { $drive } is a drive, such as C:.
app-room-free-on = { $drive }의 여유 공간
# How much room a drive has, when it would not say.
app-room-free-unknown = 알 수 없음
app-room-most-it-could-need = 압축 전 기준으로 필요할 수 있는 최대 크기입니다.
# { $drive } is a drive, such as C:, { $needed } how much room it needs, such as 1.2 GB, and { $free } how much it has.
app-room-drive-short = 이 설치 관리자를 빌드하려면 { $drive }에 최대 { $needed }의 여유 공간이 필요하지만 { $free }만 있습니다.
# { $drives } is one sentence like the one above for each drive without the room, one after another.
app-room-not-enough = 공간이 부족합니다. { $drives } 다른 드라이브의 출력 폴더를 선택하거나 일부 항목을 제외하세요.
# { $note } is a sentence, without its full stop, about a drive whose room could not be checked. Use your language's
# full stop.
app-room-could-not-tell = { $note }.

## What goes in the installer, part by part

app-footprint-heading = 설치 관리자에 포함되는 항목
app-footprint-working-out = 함께 포함될 항목을 확인하는 중…
# { $error } is why.
app-footprint-would-not-make-a-package = 남은 항목으로는 패키지를 만들 수 없습니다: { $error }
# { $list } is the names of what else is left out, such as a service and a shortcut.
app-footprint-also-left-out = 이로 인해 { $list }도 제외됩니다. 실행하거나 가리키는 대상이 없게 되기 때문입니다. 이 항목은 다른 PC에 설정되지 않습니다.
# { $list } is the names of the first few of what else is left out, and { $count } how many more there are.
app-footprint-also-left-out-and-more = 이로 인해 { $list } 외 { $count }개 항목도 제외됩니다. 실행하거나 가리키는 대상이 없게 되기 때문입니다. 이 항목은 다른 PC에 설정되지 않습니다.
# { $list } is the names of license files. The option is the one named "License and Activation Data".
app-footprint-licenses-still-go = 제외한 항목에 있는 라이선스 데이터는 "라이선스 및 정품 인증 데이터" 옵션을 따르므로 그대로 포함됩니다: { $list }.
# { $elapsed } is how long ago, in minutes and seconds such as 45:00, or hours, minutes, and seconds such as 1:05:00.
# "Look Again" names the button of that name.
app-footprint-looked-at-long-ago = 이 앱은 { $elapsed } 전에 검사했습니다. 파일은 빌드할 때 읽으므로 변경 내용이 반영되며, 그때 없어진 항목은 목록에 표시됩니다. 새로 생긴 항목을 찾으려면 "다시 검사"를 클릭하세요.
# Headings of the parts of the list, each with a line that says what is in it.
app-footprint-program-files = 프로그램 파일
app-footprint-data-folders = 데이터 폴더
# What could go in with the User Data option, when nothing else is in this part. { $size } is how much, such as 1.2 GB.
app-footprint-folders-with-user-data = 사용자 데이터 옵션 사용 시 { $size }
# { $included } is how many folders go in, { $count } how many there are, and { $size } how much those that go in hold.
app-footprint-folders-summary =
    { $count ->
       *[other] 폴더 { $count }개 중 { $included }개, { $size }
    }
# { $size } is how much, such as 1.2 GB, and { $count } in how many files.
app-footprint-data-only-with-user-data =
    { $count ->
       *[other] 이 앱의 데이터 파일 { $count }개({ $size })는 사용자 데이터 옵션을 켠 경우에만 포함됩니다.
    }
# A folder's row. { $count } is how many files it holds, and { $detail } where it is or why it cannot be left out.
app-footprint-folder-row =
    { $count ->
       *[other] 파일 { $count }개. { $detail }
    }
app-footprint-services = 서비스
app-footprint-startup = 자동 시작
app-footprint-tasks = 예약된 작업
app-footprint-shortcuts = 바로 가기
# { $included } is how many of the part's things go in, and { $count } how many there are.
app-footprint-items-summary = { $count }개 중 { $included }개
# { $why } is what else was left out that this goes with.
app-footprint-left-out-too = 함께 제외됨: { $why }
# A row's detail. { $kind } is what it is, such as Service, and { $detail } what it does.
app-footprint-change-detail = { $kind }. { $detail }
app-footprint-integrations = Windows와 연결되는 항목
# { $included } is how many kinds go in, such as file types and firewall rules, and { $count } how many kinds there are.
app-footprint-integrations-summary =
    { $count ->
       *[other] 종류 { $count }개 중 { $included }개
    }
app-footprint-integration-nothing-left = 제외한 항목 때문에 남은 항목 없음
# { $count } is how many things of this kind the installer sets up.
app-footprint-integration-things =
    { $count ->
       *[other] 설정되는 항목 { $count }개
    }
app-footprint-prerequisites = 실행에 필요한 항목
# { $count } is how many runtimes, such as Visual C++ Redistributable.
app-footprint-prerequisites-summary =
    { $count ->
       *[other] 런타임 { $count }개
    }
app-footprint-near-matches = 앱의 일부일 수도 있는 항목
# { $ticked } is how many were ticked, and { $count } how many there are.
app-footprint-near-matches-summary = { $count }개 중 { $ticked }개 추가됨
app-footprint-near-matches-hint = 선택하지 않으면 설치 관리자에 포함되지 않습니다. 항목을 선택하면 나머지 항목과 같은 검사로 앱을 다시 검사하므로, 암호처럼 보이는 항목은 계속 제외됩니다.
# { $why } is why it might be part of the app, as a sentence.
app-footprint-near-match-with-user-data = 사용자 데이터 옵션과 함께 포함됩니다. { $why }
# How much a folder holds, when not all of it could be measured. { $size } is how much was, such as 1.2 GB.
app-footprint-near-match-over = { $size } 이상
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-folder = 폴더: { $place }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-hidden = 앱의 숨겨진 부분: { $place }
app-footprint-nothing-left-out = 암호, 라이선스, 인증서처럼 보여서 제외된 항목은 없습니다.
app-footprint-left-out = 제외된 항목 및 이유
# { $count } is how many things were left out.
app-footprint-left-out-summary =
    { $count ->
       *[other] 항목 { $count }개
    }
# Above a list of things left out for one reason. { $title } is the reason.
app-footprint-left-out-group = { $title }:
app-footprint-refused = 어떤 설치 관리자도 다음 항목을 설정할 수 없습니다:
# A heading over a list that opens and closes. { $count } is how many are in it.
app-footprint-worth-knowing = 참고 사항({ $count })
# { $list } is what kinds of things were looked for and none found, from the five below.
app-none-found = 찾은 항목 없음: { $list }.
app-none-found-services = 서비스
app-none-found-startup = 시작 프로그램
app-none-found-tasks = 예약된 작업
app-none-found-shortcuts = 바로 가기
app-none-found-integrations = Windows와 연결되는 항목

## Network PC: reading another PC through WinAppMigrator Client, in three steps. "WinAppMigrator Client" is the name of a
## program, and the old PC is the one the apps are moved off.

app-network-step-client = 1. 이전 PC에 클라이언트 복사
app-network-step-client-hint = WinAppMigrator Client는 앱이 현재 있는 PC(Windows 7 이상)에서 실행됩니다. 클라이언트에 표시되는 코드를 통해 이 PC만 그 PC를 암호화된 연결로 읽을 수 있습니다.
# { $drive } is a USB drive, such as E:\.
app-network-save-to-usb = USB 드라이브 { $drive }에 저장
# { $drive } is a USB drive, such as E:\, and { $name } the name it was given, such as BACKUP.
app-network-save-to-named-usb = USB 드라이브 { $drive }({ $name })에 저장
app-network-save-to-folder = 폴더에 저장…
app-network-look-for-usb-again = USB 드라이브 다시 찾기
app-network-no-usb = 연결된 USB 드라이브가 없습니다. USB 드라이브를 연결하거나, 이전 PC에서 액세스할 수 있는 폴더(예: 네트워크의 공유 폴더)에 저장하세요.
# { $path } is where the client was saved, such as E:\WinAppMigrator Client.exe.
app-network-client-saved = 다음 위치에 저장했습니다: { $path }. 이전 PC에서 옮기려는 앱을 닫은 다음, 이 파일을 두 번 클릭하고 변경을 허용하세요. 코드가 표시됩니다.
app-network-step-pick = 2. 이전 PC 선택
app-network-looking = 네트워크에서 클라이언트를 실행 중인 PC를 찾는 중…
app-network-type-name = 또는 이름이나 주소 입력
# An example, shown in the empty field: a PC's name, or its network address.
app-network-type-name-example = OLD-PC 또는 192.168.1.20
app-network-step-code = 3. 표시된 코드 입력
app-network-code = 페어링 코드
app-network-connect = 연결
app-network-pairing = 페어링하는 중…
# Why pairing did not work, shown on its own.
app-network-pairing-stopped = 페어링이 완료되기 전에 중지되었습니다
app-network-wrong-code = 이전 PC에 표시된 코드가 아닙니다. 확인 후 다시 시도하세요.
# { $error } is what went wrong.
app-network-not-reached = 이전 PC에 연결할 수 없습니다: { $error }
app-network-pick-folder-title = WinAppMigrator Client를 저장할 폴더 선택
# { $path } is the folder, and { $error } what Windows said went wrong.
app-client-could-not-save = 클라이언트를 { $path }에 저장할 수 없습니다: { $error }
# scripts\build-client.ps1 is a file name.
app-client-not-carried = 이 WinAppMigrator 빌드에는 클라이언트가 포함되어 있지 않습니다. scripts\build-client.ps1로 클라이언트를 빌드한 다음 앱을 다시 빌드하세요.
# { $error } is what Windows said went wrong.
app-client-could-not-save-note = 클라이언트 옆에 메모를 저장할 수 없습니다: { $error }
# A note saved as a text file beside the client, for whoever runs it on the old PC. Keep its lines short, as it is read in
# Notepad. "WinAppMigrator Client.exe" is a file name. "Network PC..." names the Scan Source button: use its words in your
# language.
app-client-note =
    WinAppMigrator Client
    
    앱을 옮기려는 이전 PC(Windows 7, 8, 10 또는 11)에서 실행하세요.
    
    1. 옮기려는 앱을 닫으세요.
    2. "WinAppMigrator Client.exe"를 두 번 클릭하고 변경을 허용하세요.
    3. 6자리 페어링 코드가 표시됩니다. 새 PC의 WinAppMigrator에서
       "네트워크 PC..."를 선택하고, 이 PC를 고른 다음 코드를 입력하세요.
    4. 새 PC에서 작업이 끝나면 클라이언트 창에서 Enter 키를 누르거나
       창을 닫으세요.
    
    클라이언트는 읽기만 합니다. 실행되는 동안 Windows 드라이브의 스냅숏을
    유지하고, 로컬 네트워크에서 클라이언트 자체의 포트 2개로 들어오는
    연결을 Windows 방화벽에서 허용하며, 중지되면 둘 다 제거합니다.
    네트워크로 전송되는 내용은 암호화되며, 코드를 아는 PC만 내용을
    읽을 수 있습니다.
