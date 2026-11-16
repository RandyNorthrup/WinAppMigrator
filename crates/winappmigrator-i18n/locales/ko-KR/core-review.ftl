# Words the core says about what an install may do: why an install location or something in a package is refused,
# the installer's list of what an app sets up beyond its files, the checks of a package and of an install, and whether
# there is room for either. Message names start with core-.
#
# Many messages here are a reason, the second half of a sentence that starts "... was left out:" or "that install
# location cannot be used:", so they start with a small letter and have no full stop.


## Why an install location cannot be used. Shown in red under the Install Location field.

core-review-location-state-root = 설치 기록을 보관하는 위치입니다
core-review-location-has-files = 이미 파일이 있고 이 위치에 기록된 설치가 없으므로 빈 폴더를 사용하세요
core-policy-location-not-full-path = 전체 경로여야 합니다
core-policy-location-parent-step = ".." 요소를 포함할 수 없습니다
core-policy-location-not-plain = 이 PC 드라이브의 일반 경로여야 합니다
core-policy-location-whole-drive = 드라이브 전체일 수 없습니다
core-policy-location-windows = Windows 폴더일 수 없습니다
core-policy-location-runs-by-itself = Windows에 속한 폴더이거나, 폴더 내용이 자동으로 실행되거나 로드되는 폴더입니다
core-policy-location-shared = 다른 항목이 함께 들어 있는 폴더이므로 앱은 그 안에 별도의 폴더가 필요합니다

## Whether an app can run on this PC at all, said before anything is installed.

# { $reason } is why, as the second half of a sentence, such as "App.exe is built for Arm64, and this PC is x64, so
# Windows would not start it".
core-review-cannot-run-here = 이 앱은 이 PC에서 실행할 수 없으므로 아무것도 변경하지 않았습니다: { $reason }
core-policy-architecture-x86 = 32비트 x86
core-policy-architecture-x64 = x64
core-policy-architecture-arm64 = Arm64
core-policy-architecture-arm32 = 32비트 Arm
core-policy-architecture-unknown = 알 수 없는 아키텍처
# { $code } is the number Windows gives a kind of processor, such as 0x01c4.
core-policy-machine-type = 컴퓨터 유형 { $code }
# { $names } is a list of program names, such as "a.exe, b.exe, c.exe", and { $count } how many more there are.
core-policy-names-and-more = { $names } 외 { $count }개
core-policy-fit-programs-not-said = 이 설치 관리자에는 프로그램에 필요한 항목이 나와 있지 않으므로, 이 PC에서 실행될 수 있는지 확인하지 않았습니다.
core-policy-fit-architecture-not-told = 이 PC의 아키텍처를 확인할 수 없으므로, 이 앱의 프로그램이 이 PC에서 실행될 수 있는지 확인하지 않았습니다.
core-policy-fit-main-not-said = 이 설치 관리자에는 주 프로그램에 필요한 항목이 나와 있지 않으므로, 여기에 표시되는 내용은 나머지 정보를 기준으로 합니다.
# { $driver } is a driver's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-driver = 드라이버 { $driver }은(는) { $built_for }용으로 빌드되었는데 이 PC는 { $here }입니다. 드라이버는 Windows와 정확히 일치해야 하므로 이 앱은 여기에서 실행할 수 없습니다
# { $program } is a program's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-program = { $program }은(는) { $built_for }용으로 빌드되었는데 이 PC는 { $here }이므로 Windows에서 시작하지 않습니다
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-none-start = 이 앱의 프로그램 중 이 PC({ $here })에서 시작할 수 있는 프로그램이 없습니다: { $programs }
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-some-other-architecture = 이 앱의 일부 프로그램은 다른 아키텍처용으로 빌드되어 이 PC({ $here })에서 시작되지 않습니다: { $programs }. 앱의 나머지 부분은 시작할 수 있으므로 문제가 되지 않을 수도 있습니다.
# { $here } is a kind of processor, such as "Arm64". { $programs } is a list of program names.
core-policy-fit-emulated = 이 PC는 { $here }이므로 Windows에서 다음 프로그램을 에뮬레이터를 통해 실행합니다: { $programs }. 대부분의 프로그램은 이렇게 실행해도 문제가 없지만 일부는 그렇지 않습니다.
# { $program } is a program's file name. { $needs } and { $runs } are versions of Windows, such as 10.0 and 6.1.
core-policy-fit-needs-newer = { $program }에는 Windows { $needs } 이상이 필요하다고 되어 있는데 이 PC의 Windows 버전은 { $runs }이므로 Windows에서 시작하지 않습니다
# { $programs } is a list of program names.
core-policy-fit-some-need-newer = 이 앱의 일부 프로그램은 이보다 최신 버전의 Windows가 필요하다고 되어 있어 여기에서 시작되지 않습니다: { $programs }.
# { $from_build } and { $this_build } are Windows build numbers, such as 22631, written as Windows writes them.
core-policy-fit-older-build = 이 앱은 Windows 빌드 { $from_build }에 있었고, 이 PC의 빌드는 { $this_build }입니다. 프로그램은 이 Windows에서 시작할 수 있다고 되어 있으므로 작동할 수도 있지만, 이전 PC에서 사용하던 항목이 여기에는 없을 수 있습니다.
core-policy-fit-build-not-told = 이 Windows의 빌드를 확인할 수 없어, 이 앱이 있던 빌드와 비교하지 않았습니다.

## Room on a drive for a build or an install.

# Each of these ends the sentence core-space-shortfall, after "free for".
core-space-for-unpacking = 패키지 압축 풀기
core-space-for-install-record = 설치 기록
core-space-for-app-files = 앱 파일
core-space-for-what-is-replaced = 대체되는 항목 보관
core-space-for-build-work = 빌드 작업 파일
core-space-for-installer = 설치 관리자
core-space-for-package = 패키지
core-space-for-unpacking-and-installing = 패키지 압축 풀기 및 설치
# What goes between two of those, as in "unpacking the package and the app's files". Keep the quotes: they keep the
# spaces.
core-space-purposes-joiner = { " 및 " }
# { $drive } is a drive, such as C:, or a folder. { $needed } and { $free } are sizes, such as 1.2 GB. { $purposes } is
# what the room is for, such as "unpacking the package and the app's files".
core-space-shortfall = { $purposes }에 필요한 여유 공간이 { $drive }에 최대 { $needed } 있어야 하지만 { $free }만 있습니다
# { $drive } is a drive, such as C:, or a folder. { $needed } is a size, such as 1.2 GB.
core-space-not-told = { $drive }에서 여유 공간을 알려 주지 않아, 필요한 공간({ $needed })이 충분한지 확인하지 않았습니다
# One drive in a list of where room is needed. { $needed } and { $free } are sizes, and { $drive } a drive, such as C:.
core-space-summary = { $drive }에 최대 { $needed }(여유 공간 { $free })
# { $needed } is a size, and { $drive } a drive, such as C:.
core-space-summary-not-told = { $drive }에 최대 { $needed }(여유 공간을 확인할 수 없음)
# What goes between two sentences of core-space-shortfall inside core-review-no-room. Keep the quotes: they keep the
# space.
core-review-sentence-joiner = { ". " }
# { $shortfalls } is one or more sentences of core-space-shortfall.
core-review-no-room = 공간이 부족하여 아무것도 변경하지 않았습니다. { $shortfalls }. 공간을 확보하거나 다른 드라이브의 설치 위치를 선택하세요.

## The installer's list of what an app sets up beyond its files: what each kind is called.

core-review-kind-service = 서비스
core-review-kind-driver = 드라이버
core-review-kind-scheduled-task = 예약된 작업
core-review-kind-autostart = 자동 시작
core-review-kind-shortcut = 바로 가기
core-review-kind-path = PATH
core-review-kind-environment-variable = 환경 변수
core-review-kind-shell-extension = 셸 확장
core-review-kind-menu-command = 메뉴 명령
core-review-kind-link-type = 링크 유형
core-review-kind-file-type = 파일 형식
core-review-kind-com = COM
core-review-kind-firewall-rule = 방화벽 규칙
core-review-kind-event-log-source = 이벤트 로그 원본
core-review-kind-font = 글꼴
core-review-kind-odbc-driver = ODBC 드라이버
core-review-kind-odbc-source = ODBC 데이터 원본
core-review-kind-compatibility-setting = 호환성 설정
core-review-kind-default-apps = 기본 앱
core-review-kind-program-name = 프로그램 이름
core-review-kind-other-settings = 기타 설정

## The installer's list of what an app sets up: what each line says of it.

# { $file } is a program's file name. { $account } is the Windows account it runs as, such as LocalSystem. { $starts }
# says when it starts, and is not shown itself; for "other", { $start } is one of the core-review-start- words below.
core-review-service-detail =
    { $starts ->
        [automatic] { $file }을(를) { $account } 계정으로 실행하며, 지금 시작되고 이 PC가 시작될 때마다 시작됩니다
        [delayed] { $file }을(를) { $account } 계정으로 실행하며, 지금 시작되고 이 PC가 시작될 때마다 잠시 후 시작됩니다
        [manual] { $file }을(를) { $account } 계정으로 실행하며, 무언가가 요청할 때만 시작됩니다
        [disabled] { $file }을(를) { $account } 계정으로 실행하며, 사용 안 함 상태입니다
        [other] { $file }을(를) { $account } 계정으로 실행하며, { $start }
       *[unknown] { $file }을(를) { $account } 계정으로 실행합니다
    }
# { $account } is a Windows account, such as LocalService.
core-policy-account-custom = { $account }(사용자 지정 계정은 복원되지 않음)
# { $file } is a driver's file name.
core-review-driver-detail = { $file }을(를) Windows 자체에 로드합니다
# { $file } is a driver's file name. { $start } is when Windows starts it: one of the core-review-start- words below.
core-review-driver-detail-starts = { $file }을(를) Windows 자체에 로드하며, { $start }
# When a service or driver starts, ending a sentence such as "Loads disk.sys into Windows itself, starts …".
core-review-start-boot = Windows와 함께 가장 먼저 시작됩니다
core-review-start-system = Windows가 시작될 때 함께 시작됩니다
core-review-start-automatic = 이 PC가 시작될 때마다 시작됩니다
core-review-start-delayed = 이 PC가 시작될 때마다 잠시 후 시작됩니다
core-review-start-manual = 무언가가 요청할 때만 시작됩니다
core-review-start-disabled = 사용 안 함 상태이므로 시작되지 않습니다
core-review-start-unknown = 시작 방식은 Windows에서 알려 주지 않습니다
# { $account } is the account a task runs as: an account's name, or core-review-task-runs-as-you, or
# core-review-task-runs-as-creator.
core-review-task-detail = { $account } 계정으로 실행합니다
# { $account } is the account a task runs as, as for core-review-task-detail.
core-review-task-detail-highest = { $account } 계정으로, 가장 높은 수준의 권한으로 실행합니다
core-review-task-runs-as-you = 로그인한 사용자
core-review-task-runs-as-creator = 작업을 만든 사용자
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-autostart-detail =
    { $scope ->
        [everyone] 모든 사용자가 로그인할 때
       *[you] 현재 사용자가 로그인할 때
    } 실행됩니다
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shortcut-startup-detail =
    { $scope ->
        [everyone] 이 PC의 모든 사용자용
       *[you] 현재 사용자용
    } 시작프로그램 폴더의 바로 가기
# { $file } is the file name of what the shortcut opens. { $scope } is whom it is set up for: everyone on this PC,
# or you. It is not shown itself.
core-review-shortcut-detail =
    { $scope ->
        [everyone] 이 PC의 모든 사용자에 대해
       *[you] 현재 사용자에 대해
    } { $file }을(를) 엽니다
# { $name } is an environment variable's name, such as PATH.
core-review-path-title = { $name }에 추가
core-review-path-install-folder = 설치 폴더
# { $path } is a folder inside the install folder, such as bin\tools.
core-review-path-below-install-folder = 설치 폴더\{ $path }
# { $value } is the variable's value, as it will be set.
core-review-variable-detail = 설정할 값: { $value }
# { $count } is how many registry values.
core-review-other-settings-title =
    { $count ->
       *[other] 앱 자체 키 밖의 설정 { $count }개
    }
# { $keys } is a list of registry keys, such as HKCU\Software\Vendor.
core-review-other-settings-detail = 위치: { $keys }
# { $keys } is a list of registry keys, and { $count } how many more there are.
core-review-other-settings-detail-more = 위치: { $keys } 외 { $count }개
# { $way } says whether the rule allows or blocks, and incoming or outgoing connections; it is not shown itself.
# { $ports } is a list of port numbers, and { $port_count } how many. { $program } is the program's file name.
# { $networks } is a list of the kinds of network Windows names in English (domain, private, public), and
# { $network_count } how many.
# The kinds of network a firewall rule is for, as Windows Defender Firewall names its profiles in your language. They
# fill { $networks } in core-review-firewall-detail, joined into a list.
core-review-network-domain = 도메인
core-review-network-private = 개인
core-review-network-public = 공용
core-review-firewall-detail =
    { $program }에 대해 { $network_count ->
        [0] 모든 네트워크
       *[other] { $networks } 네트워크
    }에서 { $port_count ->
        [0] { "" }
       *[other] 포트 { $ports }의{ " " }
    }{ $way ->
        [allow-in] 들어오는 연결을 허용합니다
        [allow-out] 나가는 연결을 허용합니다
        [block-in] 들어오는 연결을 차단합니다
       *[block-out] 나가는 연결을 차단합니다
    }
# { $server } is the file name of what Explorer loads. { $scope } is whom it is set up for: everyone on this PC, or
# you. It is not shown itself.
core-review-shell-extension-detail =
    { $scope ->
        [everyone] 이 PC의 모든 사용자에 대해
       *[you] 현재 사용자에 대해
    } 파일 탐색기에 { $server }을(를) 로드합니다
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shell-extension-detail-own-file =
    { $scope ->
        [everyone] 이 PC의 모든 사용자에 대해
       *[you] 현재 사용자에 대해
    } 설치하는 파일을 파일 탐색기에 로드합니다
# The name of a shell extension: { $handler } is its name in the registry, and { $class } the kind of file or folder it
# is for, such as * or Directory.
core-policy-shell-extension-name = { $class }의 { $handler }
# { $on } is the kind of file or folder whose menu it is, as the registry names it, such as * or
# Directory. { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-menu-command-detail =
    { $scope ->
        [everyone] 이 PC의 모든 사용자에 대해
       *[you] 현재 사용자에 대해
    } { $on }의 메뉴에 명령을 추가합니다
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-default-verb-detail =
    { $scope ->
        [everyone] 이 PC의 모든 사용자에 대해
       *[you] 현재 사용자에 대해
    } 이 형식의 항목을 열 때 수행되는 동작을 변경합니다
core-review-client-entry-detail = 이 용도의 프로그램으로 선택할 수 있습니다. 기본 프로그램이 이 프로그램으로 바뀌지는 않습니다
core-review-com-permissions-title = COM 권한
core-review-com-permissions-detail = COM 클래스를 시작할 수 있는 사용자 또는 클래스의 실행 계정을 지정합니다
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-link-type-detail =
    { $scope ->
        [everyone] 이 PC의 모든 사용자에 대해
       *[you] 현재 사용자에 대해
    } 이 유형의 링크를 앱으로 엽니다
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-file-type-detail =
    { $scope ->
        [everyone] 이 PC의 모든 사용자에 대해
       *[you] 현재 사용자에 대해
    } 이 형식의 파일을 앱으로 엽니다
# { $log } is the name of a Windows event log, such as Application.
core-review-event-log-source-detail = 앱 자체 파일의 메시지를 사용하여 { $log } 로그에 쓸 수 있습니다
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-font-detail =
    { $scope ->
        [everyone] 이 PC의 모든 사용자에 대해
       *[you] 현재 사용자에 대해
    } 로그인할 때 설치 폴더에서 로드됩니다
core-review-odbc-driver-detail = ODBC를 사용하여 데이터베이스에 연결하는 프로그램에 로드됩니다
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-odbc-source-detail =
    { $scope ->
        [everyone] 이 PC의 모든 사용자용
       *[you] 현재 사용자용
    } 데이터베이스 연결
core-review-default-apps-detail = 기본 앱에서 선택할 수 있습니다. 기본 앱이 이 앱으로 바뀌지는 않습니다
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-program-name-detail =
    { $scope ->
        [everyone] 이 PC의 모든 사용자에 대해
       *[you] 현재 사용자에 대해
    } "실행" 및 다른 프로그램에서 이 이름으로 찾을 수 있습니다
# { $server } is the file name of what serves the classes.
core-review-com-title = { $server }의 COM
core-review-com-title-own-file = 설치하는 파일의 COM
# { $count } is how many registry values.
core-review-com-detail =
    { $count ->
       *[other] 다른 프로그램이 시작할 수 있는 클래스의 설정 { $count }개
    }
core-review-com-support-title = COM 형식 라이브러리, 인터페이스 및 AppID
# { $libraries }, { $interfaces }, and { $app_ids } are how many registry values there are for its type libraries, its
# interfaces, and its AppIDs.
core-review-com-support-detail = 클래스와 함께 사용되는 설정: 형식 라이브러리 { $libraries }개, 인터페이스 { $interfaces }개, AppID { $app_ids }개
core-review-compatibility-title = 프로그램 시작 방식
# { $count } is how many registry values.
core-review-compatibility-detail =
    { $count ->
       *[other] 설치하는 프로그램에 대한 호환성 설정 { $count }개
    }

## The installer's notes on whose folders and settings these are, and what is not coming.

# { $account } is the name of a Windows account, such as alice.
core-review-account-signed-in = 사용자 데이터와 사용자별 설정은 이 설치를 실행 중인 관리자가 아니라, 여기에 로그인한 { $account }에게 적용됩니다.
# { $account } is the name of a Windows account, such as alice.
core-review-account-no-desktop = 이 세션에 로그인한 사용자가 없으므로, 사용자 데이터와 사용자별 설정은 이 설치를 실행 중인 계정인 { $account }에게 적용됩니다.
# { $reason } is what Windows said, in its own words.
core-review-account-not-told = 로그인한 사용자를 확인할 수 없으므로({ $reason }) 사용자별 설정은 제외되고, 사용자 데이터는 설치 위치 아래에 저장됩니다.
# { $reason } is what Windows said, in its own words.
core-review-signed-in-unknown = 로그인한 사용자를 확인할 수 없습니다: { $reason }
# Headings of what is left out, shared by the app's list and the installer's notes.
core-review-left-out-credentials = 자격 증명처럼 보여 이 설치 관리자에 포함되지 않는 항목
core-review-left-out-settings = 한 계정에서 앱에 대해 보관하는 설정 및 기록으로, 사용자 데이터 옵션을 켠 경우에만 포함되는 항목
core-review-left-out-certificates = 이전 PC에서 신뢰하던 인증서로, 앱이 사용하는 것으로 보이지만 설치되지 않는 항목
core-review-left-out-fonts = 앱 이름으로 되어 있지만 앱 자체 글꼴이라는 근거가 없어 설치되지 않는 글꼴
# { $names } is a list of names, such as the names of saved passwords.
core-review-note-credentials = 자격 증명처럼 보여 이 설치 관리자에 포함되지 않았습니다: { $names }. 이 PC에 있는 해당 항목은 그대로 유지되며, 다시 입력해야 할 수 있습니다.
# { $names } is a list of names of license data.
core-review-note-licenses = 이 설치 관리자를 만들 때 "라이선스 및 정품 인증 데이터" 옵션이 꺼져 있어 포함되지 않았습니다: { $names }.
# { $names } is a list of places an account keeps settings, such as folders.
core-review-note-settings = 설치 관리자를 만들 때 사용자 데이터 옵션이 꺼져 있어, 한 계정에서 앱에 대해 보관하는 설정 및 기록({ $names })이 포함되지 않았습니다. 이 PC에서는 앱을 처음 사용하는 것처럼 시작됩니다.
# { $names } is a list of certificates, each as core-review-certificate.
core-review-note-certificates = 앱이 이전 PC에서 신뢰하던 인증서를 사용하는 것으로 보이지만, 이 인증서는 설치되지 않습니다: { $names }. 이 PC에서 이 인증서를 신뢰할지는 직접 결정하세요.
# { $names } is a list of font names.
core-review-note-fonts = 다음 글꼴은 앱 이름으로 되어 있지만 앱 자체 글꼴이라는 근거가 없어 설치되지 않습니다: { $names }.
# { $names } is a list of names.
core-review-note-left-out-by-choice = 이 설치 관리자를 만든 사람이 제외한 항목: { $names }.
# { $names } is a list of names.
core-review-note-added-by-choice = 이 설치 관리자를 만든 사람이 앱의 일부로 판단하여 추가한 항목: { $names }.
# { $name } is a certificate's subject, such as CN=Contoso, and { $store } the certificate store it was in, such as
# Root.
core-review-certificate = { $store }의 { $name }
# { $names } is a list of names, and { $count } how many more there are.
core-review-names-and-more = { $names } 외 { $count }개

## Why something in a package is left out. Each is the second half of a sentence of core-warning-review-left-out.

core-review-no-account = 한 계정의 설정에 속하는데, 로그인한 사용자를 확인할 수 없습니다
core-review-name-not-accepted = Windows에서 이 이름을 허용하지 않습니다
# { $error } is what Windows said, in its own words.
core-review-service-unreadable = 현재 설정을 읽을 수 없습니다: { $error }
core-review-service-dependency = 이름이 잘못된 서비스에 종속되어 있습니다
# { $path } is the program the service runs.
core-review-service-runs-other = 이 패키지에서 설치하지 않는 { $path }을(를) 실행합니다
core-review-service-settings = 설정에 이 패키지에서 설치하지 않는 항목이나, 서비스 자체 키가 아닌 키가 지정되어 있습니다
core-review-driver-no-hash = 확인에 사용할 해시가 없습니다
core-review-license-for-the-machine = 컴퓨터 전체에 저장된 라이선스는 보고만 합니다
core-review-license-starts-something = 라이선스가 무언가를 시작할 이유가 없습니다
core-review-task-not-captured = 작업 정의가 캡처되지 않았습니다
# { $error } is what was wrong with the task's XML, in English.
core-review-task-unreadable = 작업 정의를 읽을 수 없습니다: { $error }
core-review-task-no-account = 설치하는 사용자로 실행되는 작업인데, 여기에 로그인한 사용자를 확인할 수 없습니다
# { $path } is what the shortcut opens.
core-review-shortcut-opens-other = 이 패키지에서 설치하지 않는 { $path }을(를) 엽니다
# { $service } is the service's name, and { $path } the program it runs now.
core-policy-service-exists = { $service } 서비스가 이미 이 PC에 있으며, 이전에 여기에서 설치한 것이 아닌 { $path }을(를) 실행합니다. 이 앱의 이전 사본이라면 먼저 제거하세요
core-policy-key-control-characters = 키 이름에 제어 문자가 들어 있습니다
core-policy-value-control-characters = 값 이름에 제어 문자가 들어 있습니다
core-policy-registry-link = 레지스트리 링크입니다
# { $reason } is why the key it stands for is left out, such as "no app install should change this key".
core-policy-virtual-store = PC의 키를 대신하는 키이며, { $reason }
core-policy-registry-part = 앱 설치는 레지스트리의 이 부분에 쓰면 안 됩니다
core-policy-key-refused = 앱 설치는 이 키를 변경하면 안 됩니다
core-policy-net-debugger = 모든 .NET 프로그램의 디버거를 지정하게 됩니다
core-policy-net-install-root = 모든 .NET 프로그램이 .NET을 로드하는 위치를 변경하게 됩니다
core-policy-class-redirects = 한 종류의 프로그램 또는 폴더를 여는 방식을 변경하게 됩니다
core-policy-app-path-name = 파일 이름이 아닌 다른 이름으로 프로그램을 지정합니다
core-policy-app-path-not-installed = 이 패키지에서 설치하지 않는 파일에 대해 프로그램 이름을 등록하게 됩니다
core-policy-shared-default-verb = 한 종류의 모든 파일 또는 폴더를 열 때의 동작을 변경하게 됩니다
core-policy-command-runs-other = 이 패키지에서 설치하지 않는 항목을 실행하는 명령을 추가합니다
core-policy-autostart-not-command = 자동으로 시작되는데, 값이 명령이 아닙니다
core-policy-autostart-runs-other = 자동으로 시작되며, 이 패키지에서 설치하지 않는 항목을 실행합니다
core-policy-environment-own-rules = 환경 변수는 별도의 규칙에 따라 복원됩니다
core-policy-service-not-restored = 이 패키지에서 복원하지 않는 서비스에 속합니다
core-policy-service-own-rules = 서비스 자체 설정은 서비스 규칙을 통해서만 복원됩니다
core-policy-service-dll = ServiceDll이 이 패키지에서 설치하는 파일이 아닙니다
core-policy-event-log-source-only = 이벤트 로그 원본 자체의 값만 복원할 수 있습니다
core-policy-security-log = 보안 로그에 쓰는 앱은 없습니다
core-policy-event-log-messages = 메시지를 이 패키지에서 설치하지 않는 파일에서 가져오게 됩니다
core-policy-type-library = 형식 라이브러리가 이 패키지에서 설치하는 파일이 아닙니다
# { $name } is a program's file name, such as app.exe.
core-policy-application-opened = 이 패키지에서 설치하지 않는 { $name }을(를) 여는 방식을 변경하게 됩니다
core-policy-event-log-no-file = 이벤트 로그 원본은 파일에서 메시지를 가져와야 합니다
core-policy-com-runs-other = 이 패키지에서 설치하는 파일이 아닌 항목을 실행합니다
core-policy-com-redirect = 이 클래스를 이 패키지에서 설치하지 않는 항목으로 바꾸게 됩니다
core-policy-com-server = 이 패키지에서 설치하는 파일도 아니고 Windows의 일부도 아닌 항목을 실행합니다
core-policy-interface = 이 패키지에서 설치하지 않는 항목으로 인터페이스 호출 방식을 결정하게 됩니다
# { $name } is a program's file name, such as app.exe.
core-policy-app-id-started = 이 패키지에서 설치하지 않는 { $name }의 시작 방식을 변경하게 됩니다
core-policy-app-id-surrogate = 이 패키지에서 설치하지 않는 프로그램에서 호스트되게 됩니다
core-policy-app-id-service = 이 패키지에서 복원하지 않는 서비스에서 실행되게 됩니다
core-policy-app-id-run-as = 별도의 계정으로 실행되게 됩니다
core-policy-app-id-link = AppID를 지정하지 않습니다
core-policy-app-id-remote = 이 PC가 아닌 다른 곳에서 실행되게 됩니다
core-policy-odbc-own-driver = ODBC 자체 설정이며 드라이버가 아닙니다
core-policy-odbc-driver-not-installed = 드라이버가 이 패키지에서 설치하는 파일이 아닙니다
core-policy-loads-other-file = 이 패키지에서 설치하지 않는 파일을 로드하게 됩니다
core-policy-odbc-own-source = ODBC 자체 설정이며 데이터 원본이 아닙니다
core-policy-odbc-source-missing = 이 패키지에 없는 데이터 원본을 나열합니다
core-policy-odbc-source-driver = 드라이버가 이 패키지에서 설치하는 파일도 아니고 Windows의 일부도 아닙니다
core-policy-shell-extension = 이 패키지에서 설치하는 파일이 아닌 셸 확장을 파일 탐색기에 로드하게 됩니다
core-policy-not-text = 들어 있는 내용이 읽어야 할 형식의 텍스트가 아닙니다
core-policy-network-place = 네트워크상의 위치를 지정합니다
core-policy-font = 이 패키지에서 설치하지 않는 글꼴이나 Windows 폴더에 있는 글꼴을 등록하게 됩니다
core-policy-compatibility = 이 패키지에서 설치하지 않는 프로그램의 실행 방식을 변경하게 됩니다
core-policy-capabilities = 세부 정보가 이 패키지에 없는 앱을 나열합니다
core-policy-default-program = 기본적으로 항목을 여는 프로그램을 변경하게 됩니다
core-policy-url-protocol = 링크 유형이라고 되어 있지만, 이 패키지에서 설치하는 항목으로 열리지 않습니다
core-policy-command-handler = 이 패키지에서 설치하지 않는 항목이 처리하는 명령을 추가합니다
core-policy-installer-command = Windows Installer만 읽을 수 있는 내용을 실행하게 됩니다
core-policy-creates-class = 이 패키지에서 설치하지 않는 클래스를 만들게 됩니다
core-policy-opens-links = 이 패키지에서 설치하지 않는 항목으로 링크를 엽니다
core-policy-opens-files = 이 패키지에서 설치하지 않는 항목으로 파일을 엽니다
core-policy-firewall-name = 이름을 그대로 옮길 수 없습니다
core-policy-firewall-description = 설명을 그대로 옮길 수 없습니다
core-policy-firewall-every-program = 모든 프로그램에 적용되는데, 규칙은 한 프로그램에만 적용될 수 있습니다
# { $path } is the program the rule is for.
core-policy-firewall-program = 이 패키지에서 설치하지 않는 { $path }에 대한 규칙입니다
core-policy-firewall-direction = 설정할 수 있는 방향이 지정되어 있지 않습니다
core-policy-firewall-action = 다른 규칙보다 우선하게 되는데, 앱의 규칙은 그래서는 안 됩니다
# { $setting } is a part of a firewall rule, as the scan of its PC described it.
core-policy-firewall-not-carried = { $setting } 설정이 지정되어 있는데, 원래대로 설정할 수 없거나 다른 규칙의 효과를 약화시키게 됩니다
core-policy-firewall-service = 이 패키지에서 복원하지 않는 서비스에 대한 규칙입니다
core-policy-firewall-parts = 일부 내용이 방화벽 규칙에서 지정할 수 있는 내용이 아닙니다
core-policy-folder-already-there = 이 폴더는 설치 위치 밖에 이미 있었으며 이 앱의 폴더라는 근거가 없으므로, 그 안에 프로그램을 넣지 않습니다
core-policy-variable-name = 이름이 유효한 변수 이름이 아닙니다
core-policy-variable-relied-on = Windows와 다른 프로그램이 이 변수를 사용합니다
core-policy-task-name = Windows에서 허용하는 이름이 아닙니다
core-policy-task-folder = Windows에서 허용하는 폴더가 아닙니다
core-policy-task-windows = Windows 자체 작업 중 하나를 대체하게 됩니다
core-policy-task-other-action = 프로그램 실행 외의 다른 작업을 수행합니다
core-policy-task-no-program = 실행할 프로그램이 지정되어 있지 않습니다
# { $path } is the program the task runs.
core-policy-task-runs-other = 이 패키지에서 설치하지 않는 항목을 실행합니다: { $path }
core-policy-task-exists-unreadable = 이 이름의 작업이 이미 있으며, 읽을 수 없습니다
core-policy-task-exists-other = 이 이름의 작업이 이미 있으며, 이 앱을 실행하지 않습니다
# { $path } is the program the task already on this PC runs.
core-policy-task-exists-runs = 이 이름의 작업이 이미 있으며, 이전에 여기에서 설치한 것이 아닌 { $path }을(를) 실행합니다
core-policy-shortcut-not-lnk = .lnk 바로 가기가 아닙니다
core-policy-shortcut-place = 시작 메뉴나 바탕 화면에 있지 않습니다
core-policy-short-name = 경로에 확인할 수 없는 이름이 사용되었습니다
core-policy-shortcut-no-folder = 이 PC에 해당 폴더가 없습니다
core-policy-data-startup = 시작프로그램 폴더에 들어가 로그인할 때마다 실행되게 됩니다
core-policy-data-powershell = PowerShell이 시작할 때 찾은 항목을 실행하는 위치에 들어가게 됩니다
core-policy-data-windows = Windows에 속한 폴더에 들어가게 됩니다
core-policy-license-outside = 설치 위치와 모든 알려진 데이터 폴더 밖에 있습니다
core-rewrite-cdata = CDATA 섹션이 들어 있습니다
core-rewrite-too-much-to-look-through = 이 패키지의 설정에 포함된 내용이 너무 많아 설정이 가리키는 대상을 확인할 수 없습니다

## Warnings a review gives, which the installer shows with what it sets up.

# { $kind } says what was left out, and is not shown itself. { $item } is its path or name, or a registry value such
# as HKLM\Software\App\Name. { $reason } is why, as the second half of the sentence, such as "it cannot be a whole
# drive".
core-warning-review-left-out =
    { $kind ->
        [file] { $item } 파일이 제외되었습니다: { $reason }.
        [folder] { $item } 폴더가 제외되었습니다: { $reason }.
        [service] { $item } 서비스가 제외되었습니다: { $reason }.
        [driver] { $item } 드라이버가 제외되었습니다: { $reason }.
        [firewall-rule] { $item } 방화벽 규칙이 제외되었습니다: { $reason }.
        [license-data] { $item }의 라이선스 데이터가 제외되었습니다: { $reason }.
        [license-value] { $item } 라이선스 값이 제외되었습니다: { $reason }.
        [scheduled-task] { $item } 예약된 작업이 제외되었습니다: { $reason }.
        [shortcut] { $item } 바로 가기가 제외되었습니다: { $reason }.
        [environment-variable] { $item } 환경 변수가 제외되었습니다: { $reason }.
       *[value] { $item }이(가) 제외되었습니다: { $reason }.
    }
core-warning-review-package-path-missing = 복사할 수 있는 파일에 패키지 상대 경로가 없습니다.
# { $count } is the most empty folders a package makes.
core-warning-review-too-many-empty-folders = 이 패키지에는 빈 폴더가 { $count }개보다 많이 나열되어 있으며, 그중 { $count }개만 만들어집니다.
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Documents folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-documents-elsewhere =
    { $count ->
       *[other] 문서의 파일 { $count }개는 대신 { $target } 아래의 _external 폴더에 저장됩니다. 이 PC의 문서 폴더가 { $folder }에 있는데, 프로필 안의 폴더에만 쓰기 때문입니다. 이 파일을 사용하려면 직접 문서 폴더로 옮기세요.
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-documents-unknown =
    { $count ->
       *[other] 문서의 파일 { $count }개는 대신 { $target } 아래의 _external 폴더에 저장됩니다. 이 PC의 문서 폴더 위치를 확인할 수 없기 때문입니다. 이 파일을 사용하려면 직접 문서 폴더로 옮기세요.
    }
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Saved Games folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-elsewhere =
    { $count ->
       *[other] 저장된 게임의 파일 { $count }개는 대신 { $target } 아래의 _external 폴더에 저장됩니다. 이 PC의 저장된 게임 폴더가 { $folder }에 있는데, 프로필 안의 폴더에만 쓰기 때문입니다. 이 파일을 사용하려면 직접 저장된 게임 폴더로 옮기세요.
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-unknown =
    { $count ->
       *[other] 저장된 게임의 파일 { $count }개는 대신 { $target } 아래의 _external 폴더에 저장됩니다. 이 PC의 저장된 게임 폴더 위치를 확인할 수 없기 때문입니다. 이 파일을 사용하려면 직접 저장된 게임 폴더로 옮기세요.
    }

## Errors when a folder that only administrators may change cannot be made so, or checked.

# { $path } is a folder.
core-privilege-not-a-folder = { $path }은(는) 폴더가 아니라 링크 또는 파일입니다
# { $path } is a folder. { $error } is what Windows said, in its own words.
core-privilege-not-made-here = { $path }은(는) WinAppMigrator가 만든 폴더가 아니며, 다른 사용자가 그 내용을 변경할 수 있습니다. 관리자가 확인한 후 제거해야 합니다: { $error }
# { $path } is a folder.
core-privilege-others-put-something-in = { $path }에 WinAppMigrator가 만들지 않은 항목이 있으며, 다른 사용자가 이 항목을 변경할 수 있습니다. 관리자가 확인한 후 제거해야 합니다
# { $path } is a folder.
core-privilege-became-a-link-in-use = 사용하는 동안 { $path }이(가) 링크로 바뀌었습니다
# { $path } is a folder.
core-privilege-became-a-link-looked-at = 확인하는 동안 { $path }이(가) 링크로 바뀌었습니다
# { $path } is a file. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-users-cannot-start = 사용자가 { $path }을(를) 시작할 수 있도록 설정할 수 없습니다: { $said }
core-privilege-elevation-unknown = 이 프로세스가 관리자 권한으로 실행 중인지 확인할 수 없습니다
# { $path } is a folder. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-cannot-restrict = { $path }에 대한 액세스를 관리자로 제한할 수 없습니다: { $said }
# { $path } is a folder. { $said } is the folder's permissions as Windows wrote them, and what else it printed.
core-privilege-not-restricted = { $path }에 대한 액세스를 관리자로 제한할 수 없으며, 해당 드라이브에서 사용 권한이 유지되지 않을 수 있습니다: { $said }
# { $path } is a folder.
core-privilege-written-to-first = { $path }을(를) 보호하기 전에 다른 프로세스가 이 폴더에 썼습니다
core-privilege-no-program-data = Windows에서 ProgramData의 위치를 알려 주지 않습니다

## Checks of a package's files, and of what an install put in place. Each is said of one file, value, service, or the
## like, which the report names beside it.

# { $path } is the installer's file.
core-verify-no-package = { $path }에 패키지가 없습니다
# { $path } is a folder.
core-verify-not-unpacked = 이 설치 관리자의 패키지가 { $path }에 압축 해제되지 않았습니다
core-verify-schema-supported = 패키지 매니페스트 스키마가 지원됩니다.
core-verify-packaged-file-missing = 패키지된 파일이 없습니다.
core-verify-no-package-path = 복사할 수 있는 파일에 패키지 상대 경로가 없습니다.
core-verify-folder-there = 빈 폴더가 있습니다.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-unreadable = 확인하기 위해 읽을 수 없습니다: { $error }
core-verify-restored-file-missing = 복원된 파일이 없습니다.
# { $expected } and { $actual } are sizes in bytes.
core-verify-size-mismatch = 파일 크기가 일치하지 않습니다. 예상 크기는 { $expected }바이트이지만 실제 크기는 { $actual }바이트입니다.
core-verify-no-hash = 매니페스트에 이 파일의 해시가 기록되어 있지 않으므로 확인할 수 없습니다.
core-verify-hash-mismatch = SHA-256 해시가 일치하지 않습니다.
core-verify-file-matches = 파일 크기와 해시가 매니페스트와 일치합니다.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-payload-failed = 라이선스 페이로드를 확인하지 못했습니다: { $error }
core-verify-license-payload-missing = 패키지된 라이선스 페이로드가 없습니다.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-failed = 라이선스 저장소를 확인하지 못했습니다: { $error }
core-verify-license-store-file-missing = 패키지된 라이선스 저장소 파일이 없습니다.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-path-invalid = 패키지된 라이선스 저장소 경로가 잘못되었습니다: { $error }
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-prerequisite-failed = 필수 구성 요소 페이로드를 확인하지 못했습니다: { $error }
core-verify-prerequisite-missing = 포함된 필수 구성 요소 페이로드가 없습니다.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-failed = 드라이버 이미지를 확인하지 못했습니다: { $error }
core-verify-bundled-driver-missing = 포함된 드라이버 이미지가 없습니다.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-path-invalid = 포함된 드라이버 경로가 잘못되었습니다: { $error }
core-verify-prerequisite-present = 런타임 필수 구성 요소가 대상 PC에 있습니다.
core-verify-prerequisite-not-detected = 런타임 필수 구성 요소가 검색되지 않았습니다. 다시 시작하거나 수동으로 완료해야 할 수 있습니다(정확한 결과는 복원 로그를 참조하세요).
core-verify-prerequisite-not-probed = 레지스트리로는 필수 구성 요소가 있는지 확인할 수 없습니다. 설치는 가능한 범위에서 수행되었습니다.
core-verify-license-file-missing = 복원된 라이선스 파일이 없습니다.
core-verify-license-store-restored-missing = 복원된 라이선스 저장소 파일이 없습니다.
core-verify-registry-key-there = 레지스트리 키가 있습니다.
core-verify-registry-value-matches = 레지스트리 값이 보관된 데이터와 일치합니다.
core-verify-registry-value-differs = 레지스트리 값이 보관된 데이터와 일치하지 않습니다.
core-verify-firewall-rule-there = Windows 방화벽에 이 규칙이 있습니다.
core-verify-variable-matches = 환경 변수 값이 보관된 데이터와 일치합니다.
core-verify-variable-differs = 환경 변수 값이 보관된 데이터와 일치하지 않습니다.
# { $error } is what Windows said, in its own words.
core-verify-variable-unreadable = 환경 변수가 없거나 읽을 수 없습니다: { $error }
core-verify-service-runs-other = 서비스가 있지만 복원된 바이너리를 해당 인수로 실행하지 않습니다.
core-verify-service-other-account = 서비스가 있지만 예상한 계정으로 실행되지 않습니다.
core-verify-service-matches = 서비스가 있으며, 복원된 바이너리를 해당 인수와 계정으로 실행합니다.
core-verify-service-missing = 서비스가 없습니다.
# { $error } is what Windows said, in its own words.
core-verify-service-unreadable = 서비스를 읽을 수 없습니다: { $error }
core-verify-driver-other-image = 드라이버 서비스가 복원된 이미지를 가리키지 않습니다.
core-verify-driver-not-registered = 복원 후 포함된 드라이버 서비스가 등록되지 않았습니다.
# { $error } is what Windows said, in its own words.
core-verify-driver-unreadable = 드라이버 서비스를 읽을 수 없습니다: { $error }
core-verify-driver-image-missing = 드라이버 이미지가 없습니다.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-unchecked = 드라이버 이미지를 확인할 수 없습니다: { $error }
core-verify-task-matches = 예약된 작업이 있으며 복원된 프로그램을 실행합니다.
core-verify-task-runs-other = 예약된 작업이 있지만 복원된 프로그램을 실행하지 않습니다.
core-verify-task-missing = 예약된 작업이 없습니다.
# { $error } is what Windows said, in its own words.
core-verify-task-unreadable = 예약된 작업 쿼리에 실패했습니다: { $error }
core-verify-shortcut-matches = 바로 가기가 있으며 대상 경로가 복원된 대상과 일치합니다.
core-verify-shortcut-opens-other = 바로 가기가 있지만 대상 경로가 복원된 대상과 일치하지 않습니다.
core-verify-shortcut-missing = 바로 가기가 없습니다.
# { $error } is what Windows said, in its own words.
core-verify-shortcut-unreadable = 바로 가기 쿼리에 실패했습니다: { $error }

## Warnings and errors a check gives, which the app and the installer show.

core-warning-verify-schema-unsupported = 패키지 매니페스트 스키마가 지원되지 않습니다.
# { $error } is what went wrong, in Windows' or the program's own words.
core-warning-verify-package-file-failed = 패키지 파일을 확인하지 못했습니다: { $error }
core-warning-verify-folder-missing = 빈 폴더가 없습니다.
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-key-missing = 레지스트리 키가 없거나 읽을 수 없습니다: { $error }
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-value-missing = 레지스트리 값이 없거나 읽을 수 없습니다: { $error }
core-warning-verify-registry-kind-missing = 레지스트리 값 형식이 없습니다.
core-warning-verify-firewall-rule-missing = Windows 방화벽에 이 규칙이 없습니다.
core-warning-verify-service-scope = 서비스 확인은 HKLM 서비스 구성을 읽기만 하며 서비스를 시작하지 않습니다.
core-warning-verify-driver-manual = 하나 이상의 공급업체/기본 제공 드라이버는 WinAppMigrator에서 재배포하지 않습니다. 대상 PC에서 공급업체의 서명된 드라이버 패키지로 다시 설치하세요.
core-warning-verify-task-scope = 예약된 작업 확인은 등록 여부와 명령 참조만 검사합니다.

## The verification report written beside a package, verification_report.md. Its headings follow a "#".

core-verify-report-title = WinAppMigrator 확인 보고서
# { $time } is when, as a date and time in UTC, such as 2026-11-15T10:00:00Z.
core-verify-report-verified = 확인 시각(UTC): { $time }
# { $valid } is true or false, which are not translated.
core-verify-report-package-valid = 패키지 유효: { $valid }
# { $valid } is true or false, which are not translated.
core-verify-report-restore-valid = 복원 유효: { $valid }
core-verify-report-restore-not-run = 복원 유효: 실행 안 함
core-verify-report-items = 항목
# { $message } is what a check found, and { $source } the file or setting it looked at.
core-verify-report-item-from = { $message } 원본: { $source }
core-verify-report-warnings = 경고
core-verify-report-errors = 오류
core-verify-report-none = 없음.
