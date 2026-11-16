# Words the core writes for people while it builds a package from an installed app: what a package records for an
# installer to show later, errors shown in the app, the list of what goes in and what is left out, opening a disk
# image or another PC over the network, and what an installer says when the package it carries cannot be read.
# Message names start with core-.


## Scanning an app, before a package is built. Shown in the app when a scan cannot go on.

# { $path } is the app's program, such as D:\Program Files\Acme\Acme.exe, on a drive taken from another PC.
core-scan-program-not-on-the-drive = { $path }은(는) 링크를 거치거나 연결된 드라이브에 있지 않으므로 읽지 않았습니다.
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-not-on-this-pc = { $path }은(는) 이 PC의 디스크에 있지 않거나 다른 항목을 열게 되는 이름이므로 읽지 않았습니다.
# { $path } is the app's program.
core-scan-program-someone-elses = { $path }은(는) 다른 계정의 폴더에 있거나 이 계정의 것이 아니며, 이 앱 자체의 기록에도 그 위치에 속한다는 내용이 없으므로 읽지 않았습니다.
# { $app_name } is the app's name, as the list of apps calls it.
core-scan-no-executable = { $app_name }의 실행 파일을 찾을 수 없습니다. 드라이버, 런타임 또는 추가 기능에는 패키지로 만들 자체 프로그램이 없을 수 있습니다.
# { $path } is the file that was picked.
core-scan-not-a-file = 검사 대상이 파일이 아닙니다: { $path }
# Names something left out of a package, in a list of what was left out and why. { $name } is the variable's name,
# such as ACME_LICENSE_KEY.
core-scan-environment-variable-item = 환경 변수 { $name }

## Folders and parts that might also be the app's, offered in a list to tick. Each says why it is offered.

core-near-starts-with-app-name = 이름이 이 앱의 이름으로 시작합니다.
core-near-has-app-name = 이름에 이 앱의 이름이 들어 있습니다.
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-starts-with-publisher = 이름이 게시자 이름({ $publisher })으로 시작합니다.
# { $folder } is a folder's name, such as Program Files.
core-near-hidden-part-beside = 앱 목록에 표시되지 않는 구성 요소로, { $folder }에 이 앱과 나란히 설치되어 있습니다.
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-hidden-part-publisher = 앱 목록에 표시되지 않는 구성 요소로, 게시자({ $publisher })가 같습니다.

## What a scan found, as a person chooses from it: the folders that go in, and what is left out, and why

# Why a folder cannot be left out, beside its number of files.
core-footprint-locked-app-folder = 항상 주 프로그램과 함께 포함됩니다
core-footprint-locked-main-program = 주 프로그램이 들어 있으며, 항상 포함됩니다
core-footprint-locked-needed = 주 프로그램을 시작하는 데 꼭 필요한 항목이 들어 있습니다
core-footprint-locked-font = "Windows와 연결되는 항목"에 있는 글꼴과 함께 포함됩니다
# Where a folder is. "AppData" and "ProgramData" are the names of Windows folders; keep them as they are.
core-footprint-place-roaming = 프로필의 설정(AppData)
core-footprint-place-local = 프로필의 이 PC 전용 설정(AppData)
core-footprint-place-program-data = PC의 모든 사용자가 공유하는 데이터(ProgramData)
core-footprint-place-documents = 문서에 있음
core-footprint-place-saved-games = 저장된 게임에 있음
core-footprint-place-public-documents = 공용 문서에 있음
core-footprint-place-app-folder = 앱 폴더에 있음
# { $folder } is a folder, such as C:\Users\Sam\AppData\Roaming.
core-footprint-place-in = { $folder }에 있음
# A row for the files directly in the app's folder. { $folder } is that folder's name, such as Acme.
core-footprint-files-directly-in = { $folder }에 바로 들어 있는 파일
# Headings over what is left out, and why each item is.
core-footprint-licenses-left-out = "라이선스 및 정품 인증 데이터" 옵션이 꺼져 있어 이 설치 관리자에 포함되지 않는 항목
core-footprint-font-stays-behind = 이 글꼴은 옮겨지지 않습니다.
core-footprint-drivers-left-out = Windows 또는 공급업체의 드라이버는 다른 PC로 넘길 수 없으므로 다른 PC에서 다시 설치하세요
core-footprint-driver-reinstall = 공급업체에서 제공하는 패키지로 다시 설치하세요.
# Why something else is left out along with what the person left out. { $service } is a service's name, such as AcmeSync.
core-footprint-needs-left-out = 필요한 { $service } 서비스가 제외되었습니다
core-footprint-told-to-use-left-out = 제외한 폴더에 있는 항목을 사용하도록 설정되어 있습니다
core-footprint-file-in-left-out = 해당 파일이 제외한 폴더에 있습니다
core-footprint-program-in-left-out = 대상 프로그램이 제외한 폴더에 있습니다
core-footprint-names-left-out = 제외한 폴더를 가리킵니다
# Why something is left out, as the installer tells whoever installs it.
core-footprint-builder-left-it-out = 이 설치 관리자를 만든 사람이 제외했습니다
# Something the person left out, by kind and name, such as "Service Acme Updater". { $kind } is the kind, such as
# Service or Scheduled Task; { $title } is its name.
core-footprint-left-out-item = { $kind } { $title }

## Building a package and an installer from a scan. Shown in the app when a build cannot go on.

core-package-scan-of-another-app = 이 검사는 빌드 중인 앱이 아닌 다른 앱에 대한 것이므로 아무것도 빌드하지 않았습니다
core-package-scan-brought-in-more = 이 검사에 이 빌드에서 요청하지 않은 데이터가 포함되어 있어 아무것도 빌드하지 않았습니다
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-package-program-gone = { $path }이(가) 더 이상 없으므로 아무것도 빌드하지 않았습니다. 앱을 다시 검사하세요.
core-package-needs-stub = 자동 압축 풀기 보관 파일에는 설치 관리자 스텁이 필요합니다
# { $reason } is what went wrong.
core-package-staged-file-failed = 준비 폴더의 파일을 패키지에 넣을 수 없습니다: { $reason }
# { $shortfalls } says which drives are short of room and by how much, in sentences of its own.
core-package-not-enough-room = 이 설치 관리자를 빌드할 공간이 부족하여 아무것도 읽거나 쓰지 않았습니다. { $shortfalls }. 공간을 확보하거나 다른 드라이브의 출력 폴더를 선택하세요.
# { $path } is a file's path, on a drive taken from another PC.
core-package-not-read-off-the-drive = { $path }은(는) 링크를 거치거나 연결된 드라이브에 있지 않으므로 읽지 않았습니다
# { $path } is a file's path.
core-package-not-read-not-local = { $path }은(는) 이 PC의 디스크에 있지 않으므로 읽지 않았습니다
# { $path } is a file's path.
core-package-not-read-other-account = { $path }은(는) 다른 계정의 폴더로 이어지므로 읽지 않았습니다
# { $path } is a file's path, as the PC it came from called it.
core-package-not-on-the-drive = { $path }은(는) 연결된 드라이브에 없으므로 읽지 않았습니다
# { $path } is a file's path.
core-package-backup-copy-no-parent = 상위 디렉터리가 없는 파일은 백업 복사할 수 없습니다: { $path }
# { $path } is a file's path.
core-package-backup-copy-destination-no-parent = 백업 복사 대상에 상위 디렉터리가 없습니다: { $path }
# robocopy is the name of a Windows command; keep it as it is. { $code } is how it ended, such as Some(8).
core-package-robocopy-failed = robocopy 백업 모드 복사에 실패했습니다(종료 코드 { $code })
# Stands for a folder where a file's name would be, as in "I/O failed for temporary package folder: …".
core-package-temporary-folder = 임시 패키지 폴더
# Stands for the installer being written where a file's name would be, as in "I/O failed for the installer: …".
core-package-the-installer = 설치 관리자
# { $path } is a file's path, or its name in the package. { $error } is what went wrong with it.
core-package-path-error = { $path }: { $error }
# { $name } is a file's name in the package, such as app/bin/App.exe.
core-package-same-place = 다른 파일이 패키지에서 같은 위치를 차지합니다: { $name }
# { $name } is a file's name in the package.
core-package-changed-in-protected-folder = 패키지를 만드는 동안 보호된 폴더에서 { $name }이(가) 변경되었습니다
core-package-worker-died = 파일을 압축하는 스레드가 예기치 않게 중지되어 설치 관리자를 쓰지 않았습니다
# { $path } is a file's path.
core-package-link-not-read = { $path }은(는) 링크이므로 읽지 않았습니다
# { $path } is a file's path. { $error } is what went wrong.
core-package-not-compressed = { $path }을(를) 압축할 수 없습니다: { $error }
# { $path } is a file's path. { $reason } is what went wrong.
core-package-not-read-back = { $path }을(를) 다시 읽을 수 없습니다: { $reason }

## Where an app's files are, and where an installer may write them. Shown when a path is refused.

# { $path } is a folder, such as C:\Program Files\Acme.
core-discovery-root-not-full-path = 쓰기 대상 폴더는 전체 경로여야 합니다: { $path }
# { $path } is a folder's path.
core-discovery-root-not-a-folder = 쓰기 대상 폴더가 폴더가 아닙니다: { $path }
# { $path } is a file's path.
core-discovery-escapes-root = 경로가 지정된 루트를 벗어납니다: { $path }
# { $path } is the link's path.
core-discovery-refusing-to-write-through-link = 링크를 통해서는 쓰지 않습니다: { $path }
# { $path } is the link's path.
core-discovery-refusing-link = 링크를 거쳐 가지 않습니다: { $path }
# { $error } is what Windows said went wrong.
core-discovery-refusing-link-because = 링크를 거쳐 가지 않습니다: { $error }
# { $path } is the link's path.
core-discovery-link-to-somewhere-else = { $path }은(는) 다른 위치를 가리키는 링크이며, 다른 사용자가 링크 대상을 바꿀 수 있습니다. 링크가 가리키는 위치를 사용하세요
# { $path } is a folder's path.
core-discovery-no-parent-folder = { $path }에 상위 폴더가 없습니다
# { $path } is the install location as it was given. Keep the quotes, or use your language's own.
core-discovery-install-location-not-full-path = 설치 위치는 전체 경로여야 합니다: "{ $path }"
# { $path } is the install location.
core-discovery-install-location-a-link = 설치 위치가 다른 위치를 가리키는 링크입니다: { $path }
# { $path } is a file's path.
core-discovery-device-name = 경로에 Windows 장치용으로 예약된 이름이 사용되었습니다: { $path }
# { $path } is a file's path.
core-discovery-device-name-on-older-windows = { $path }의 이름은 이 Windows 버전에서 파일이 아닌 장치로 열리므로 이 경로에 아무것도 쓰지 않았습니다. Windows 11에서는 설치할 수 있습니다
# { $path } is a file's path.
core-discovery-unsafe-path = 안전하지 않은 경로입니다: { $path }
# { $path } is a file's path.
core-discovery-no-package-path = 파일에 패키지 상대 경로가 없습니다: { $path }

## Writing an installer, and reading the package inside one. Most of these are shown by an installer, on the PC it
## runs on, when it cannot read what it carries. "The package" is the app's files and settings that an installer
## carries; "the stub" is the program part of an installer, before the package; "the data file" is the .wamdata file
## that holds the package of a very large installer, beside it.

# Stands for the package where a file's name would be, as in "I/O failed for installer package: …".
core-archive-installer-package = 설치 관리자 패키지
# Stands for the installer program built into WinAppMigrator, where a file's name would be.
core-archive-embedded-stub = 포함된 설치 관리자 스텁
# { $index } is the number of a file in the package. { $reason } is what went wrong.
core-archive-entry-unreadable = 패키지의 { $index }번 항목을 읽을 수 없습니다: { $reason }
# { $name } is a file's name inside the package, such as app/bin/App.exe.
core-archive-entry-unsafe-path = zip 항목의 경로가 안전하지 않습니다: { $name }
# { $name } is a file's name inside the package.
core-archive-holds-more = { $name }에 표시된 크기보다 많은 데이터가 들어 있습니다
# { $name } is a file's name inside the package.
core-archive-name-too-long = 이 이름은 zip에 사용하기에 너무 깁니다: { $name }
core-archive-changed-while-copied = 패키지를 스텁 뒤에 붙이는 동안 패키지가 변경되어 설치 관리자를 쓰지 않았습니다
core-archive-stub-not-a-program = 이 설치 관리자에서 패키지 앞부분이 프로그램이 아닙니다
core-archive-damaged = 이 설치 관리자가 손상되었거나 빌드된 후 변경되어 패키지에서 아무것도 읽지 않았습니다. 빌드된 위치에서 다시 복사하세요.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-missing = 이 설치 관리자는 같은 폴더의 { $name } 파일에 패키지를 보관하는데, 그 파일이 없습니다. 두 파일은 함께 있어야 합니다. 설치 관리자를 빌드한 위치에서 { $name } 파일을 이곳으로 복사한 다음 다시 실행하세요.
# { $name } is the data file's name, such as App_installer.wamdata. { $error } is what Windows said went wrong.
core-archive-data-file-unopened = 이 설치 관리자의 데이터 파일 { $name }을(를) 열 수 없습니다: { $error }. 두 파일은 이 PC에서 읽을 수 있는 폴더에 함께 있어야 합니다.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-a-link = { $name }은(는) 링크 또는 폴더입니다. 이 설치 관리자의 데이터 파일은 설치 관리자 바로 옆에 있는 독립된 파일이어야 하므로 아무것도 읽지 않았습니다. 실제 파일을 이곳으로 복사하세요.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-wrong = { $name }은(는) 이 설치 관리자의 데이터 파일이 아니거나, 손상되었거나, 빌드된 후 변경되었으므로 아무것도 읽지 않았습니다. 빌드된 위치에서 두 파일을 다시 복사하세요.
core-archive-stub-signed = 설치 관리자 스텁이 서명되어 있습니다. 스텁은 서명되지 않은 상태여야 하므로, 대신 완성된 설치 관리자에 서명하세요
# { $size } is a size, such as 4.2 GB.
core-archive-too-big-to-run = 설치 관리자의 크기는 { $size }인데, Windows에서는 4GB 이상의 실행 파일을 실행할 수 없으므로 아무것도 쓰지 않았습니다
core-archive-outside-signature = 설치 관리자의 서명 범위 밖에 데이터가 있으므로 변조되었을 수 있습니다
core-archive-footer-length = 자동 압축 풀기 보관 파일의 바닥글 길이가 잘못되었습니다
core-archive-seek-outside = 패키지 범위 밖을 탐색했습니다
# { $path } is a file's path.
core-archive-unsafe-package-path = 안전하지 않은 패키지 경로입니다: { $path }
# { $path } is a file's path.
core-archive-empty-package-path = 패키지 경로가 비어 있습니다: { $path }

## Opening a disk image of another PC (a .vhd or .vhdx file) to read apps from it

# { $path } is the disk image file, such as D:\Backups\OldPC.vhdx.
core-image-not-attachable = { $path }은(는) Windows에서 연결할 수 있는 디스크 이미지(.vhd 또는 .vhdx)가 아닙니다
core-image-needs-administrator = 디스크 이미지를 연결하려면 관리자 권한이 필요합니다
# { $path } is the disk image file the person picked.
core-image-not-a-file = { $path }은(는) 파일이 아닙니다
# { $path } is the disk image file. "Edit Disk" is the name of a Hyper-V Manager command.
core-image-differencing = { $path }은(는) 차이점 보관용 디스크입니다. 이 디스크에는 부모 디스크를 기준으로 변경된 내용만 들어 있으며, Windows는 기록된 위치에 있는 부모 디스크도 함께 엽니다. 먼저 Hyper-V의 디스크 편집으로 부모 디스크에 병합한 다음, 부모 디스크를 선택하세요
# { $path } is the disk image file.
core-image-unreadable = { $path }은(는) 읽을 수 있는 .vhd 또는 .vhdx가 아닙니다
# ProgramData is the name of a Windows folder; keep it as it is.
core-image-no-program-data = 이 PC의 ProgramData 폴더를 찾을 수 없습니다
# The reason given in core-image-could-not-attach when Windows already has the image open.
core-image-already-attached = Windows에 이미 연결되어 있으며, 연결한 프로그램이 이 이미지에 쓰고 있을 수 있습니다. 먼저 분리하거나, 해당 드라이브를 연결된 드라이브로 선택하세요
# { $path } is the disk image file. { $reason } is why, such as core-image-already-attached or what Windows said.
core-image-could-not-attach = { $path }을(를) 연결할 수 없습니다: { $reason }
# { $path } is the disk image file.
core-image-no-windows-volume = { $path }에 읽을 수 있는 Windows 볼륨이 없습니다. BitLocker로 암호화된 볼륨은 먼저 잠금을 해제해야 합니다.
# Given as the reason when Windows failed without saying anything.
core-image-windows-did-not-say = Windows에서 이유를 알려 주지 않았습니다

## Reading another PC over the network, through the client running on it. Shown as why something on that PC
## could not be read. "The client" is WinAppMigrator Client, the small program on the other PC.

core-remote-too-slow = 다른 PC가 응답하는 데 시간이 너무 오래 걸렸습니다
core-remote-closed = 다른 PC와의 연결이 닫혔습니다
core-remote-failed = 다른 PC와의 연결에 실패했습니다
core-remote-answered-something-else = 클라이언트가 예상과 다른 응답을 보냈습니다
# { $name } is the PC's name or address as the person typed it, such as OLDPC or 192.168.1.20.
core-remote-not-found = 네트워크에서 { $name }을(를) 찾을 수 없습니다
core-remote-did-not-say-what-it-is = 클라이언트가 자신이 무엇인지 알려 주지 않았습니다
core-remote-ordinary-names-only = 다른 PC의 루트 아래에 있는, 일반 이름으로 된 경로만 요청할 수 있습니다
core-remote-more-but-none = 다른 PC에서 폴더에 항목이 더 있다고 했지만 하나도 보내지 않았습니다
# { $count } is the most things one folder may hold, such as 250,000.
core-remote-too-many-listed = 다른 PC에서 한 폴더에 { $count }개가 넘는 항목을 나열했습니다
core-remote-sent-too-much = 다른 PC에서 요청한 것보다 많은 파일 데이터를 보냈습니다
core-remote-name-not-text = 텍스트가 아닌 이름은 요청할 수 없습니다
core-remote-below-root-only = 다른 PC의 루트 아래에 있는 경로만 요청할 수 있습니다
core-remote-file-ended-early = 다른 PC의 파일이 예상보다 일찍 끝났습니다
core-remote-link-not-followed = 다른 PC의 링크는 따라가지 않습니다
core-remote-only-files-opened = 다른 PC에서는 파일만 열 수 있습니다
core-remote-listing-said-nothing = 목록에 해당 항목에 대한 정보가 없습니다
core-remote-change-time-unknown = 변경된 시간을 알 수 없습니다
core-remote-seek-before-start = 시작 위치 이전으로 탐색했습니다
# { $size } is how many bytes a file is; { $limit } is the most that were expected. Both are numbers.
core-remote-too-large =
    { $size ->
       *[other] 크기가 { $size }바이트이지만 { $limit }바이트 이하로 예상했습니다
    }

## Checking that a download is signed by Microsoft. Each is given as the reason a check failed.

# { $error } is what Windows said went wrong.
core-signature-no-powershell = PowerShell을 시작할 수 없습니다: { $error }
# { $error } is what went wrong reading PowerShell's answer.
core-signature-unreadable = 서명을 읽을 수 없습니다: { $error }
# { $signer } is who signed the file, such as Contoso Ltd.
core-signature-not-microsoft = 서명자가 Microsoft가 아니라 { $signer }입니다
core-signature-not-microsoft-unnamed = Microsoft가 아닌 다른 사람이 서명했습니다
# { $status } is Windows' own word for the signature's state, such as HashMismatch or NotSigned, which is not translated.
core-signature-status = 서명 상태가 { $status }입니다

## Warnings a package records when it is built from an app, which an installer shows later, maybe on another PC, in
## that PC's language. "This app" is the app being packaged.

core-warning-build-program-needs-unreadable = 이 프로그램이 Windows에서 필요로 하는 항목을 읽을 수 없으므로, 다른 PC에서 실행될 수 있는지는 확인하지 않습니다.
core-warning-build-external-manifest = 외부 애플리케이션 매니페스트가 검색되었습니다. side-by-side 종속성 해석은 나중에 처리합니다.
core-warning-build-non-exe = 선택한 파일의 확장자가 .exe가 아닙니다. PE 검사는 계속 진행합니다.
core-warning-build-stand-in-program = 일반 앱 실행 파일을 찾을 수 없어 이 파일을 대신 사용합니다. 아키텍처 및 종속성 결과는 이 파일에 대한 것이며, 앱 전체에 맞지 않을 수 있습니다.
core-warning-build-other-apps-files-unknown = Windows Installer에 있는 다른 앱의 정보를 모두 읽을 수 없었으므로, 이 앱에 기록된 폴더에 다른 앱의 파일이 있어도 여기에서는 알 수 없습니다.
core-warning-build-app-folder-has-another-apps-file = Windows Installer에 따르면 이 앱의 프로그램이 있는 폴더에 다른 앱의 파일이 있습니다. 그 폴더 전체를 가져오므로 이 파일도 함께 포함됩니다.
# "App Paths" is the name of a Windows registry key; keep it as it is.
core-warning-build-app-paths-folder-left-out = 이 앱의 App Paths 항목에 앱 자체 폴더가 아닌 폴더를 검색한다고 되어 있으므로 그 폴더는 포함되지 않습니다. 그 폴더의 내용이 없는 PC에서는 앱이 시작되지 않을 수 있습니다.
core-warning-build-uninstaller-folder-left-out = Windows Installer에 따르면 다른 앱의 파일이 있으므로, 이 앱의 제거 프로그램이 있는 폴더를 제외했습니다.
# { $why } is never shown: it chooses the reason. Translate each reason as the end of the sentence.
core-warning-build-recorded-folder-left-out =
    이 앱에 대해 기록된 항목이 가리키는 폴더를 제외했습니다. { $why ->
        [unreadable] 경로가 드라이브를 벗어나거나 링크를 거치므로 안전하게 읽을 수 없기 때문입니다
        [missing] 폴더가 없기 때문입니다
        [link] 링크를 거치기 때문입니다
        [short-name] 실제 이름이 아닌 다른 형식으로 기록되어 있기 때문입니다
        [another-account] 다른 계정의 폴더에 있기 때문입니다
        [one-account] 이 앱은 한 계정용으로 설치되었는데 이 폴더는 그 계정의 것이 아니기 때문입니다
        [shared] 공유되는 항목이 들어 있거나 Windows의 일부이기 때문입니다
        [generic-name] 이름만으로는 누구의 폴더인지 알 수 없기 때문입니다
        [publisher] 게시자 이름만으로 되어 있어 게시자의 다른 제품이 들어 있을 수 있기 때문입니다
        [holds-the-app] 이 앱 자체 폴더를 포함하고 있어 다른 항목도 들어 있을 수 있기 때문입니다
        [another-apps-name] 다른 앱의 이름으로 되어 있기 때문입니다
        [another-apps-record] 설치 기록이 다른 앱의 것이기 때문입니다
        [in-another-apps-folder] 다른 앱의 폴더 안에 있기 때문입니다
       *[another-apps] 다른 앱의 폴더이기 때문입니다
    }.
core-warning-build-app-folder-holds-another-app = 이 PC의 다른 앱이 이 앱의 폴더 안에 있는 것으로 기록되어 있습니다. 그 폴더 전체를 가져오므로 그 앱도 함께 포함됩니다.
# "Program Files (x86)" is the name of a Windows folder; keep it as Windows calls it.
core-warning-build-from-32-bit-windows = 이 앱은 32비트 Windows에 있었습니다. 레지스트리 키는 64비트 Windows에서 32비트 앱의 키를 보관하는 방식으로 배치되며, 설치 관리자는 이 앱의 위치로 Program Files (x86)을 제안합니다.
# { $dll } is a file's name, such as MSVCP140D.dll.
core-warning-build-debug-runtime = 프로그램에 Visual C++ 런타임의 디버그 빌드인 { $dll }이(가) 필요합니다. Microsoft는 디버그 빌드용 설치 관리자를 제공하지 않으며 재배포를 허용하지 않을 수도 있으므로, Visual Studio가 없는 PC에서는 이 프로그램이 실행되지 않습니다.
# { $program } is a program's file name, such as Tool.exe.
core-warning-build-single-file-program = { $program }은(는) 단일 파일로 묶인 .NET 프로그램으로, 어떤 .NET이 필요한지 나타나 있지 않으므로 .NET을 설치하도록 계획하지 않았습니다. .NET이 필요한 프로그램이라면 해당 .NET이 없는 PC에서는 시작되지 않습니다.
# { $count } is how many of the app's programs.
core-warning-build-programs-not-inspected =
    { $count ->
       *[other] 앱의 프로그램이 너무 많거나 너무 커서, 그중 { $count }개는 필요한 런타임을 확인하지 않았습니다.
    }
# { $runtime } is a runtime's name, such as Java Runtime Environment.
core-warning-build-runtime-no-installer = 이 앱에는 { $runtime }이(가) 필요하지만, 여기에는 그 설치 관리자가 없으므로 실행하지 않습니다. 다른 PC에 직접 설치하세요.
# { $why } is why something was left out, such as "its name says what it holds".
core-warning-build-left-out-of-package = 패키지에서 제외되었습니다: { $why }.
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-missing-dependency = 종속성을 확인할 수 없습니다: { $name }
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-dependency-only-here = { $name }은(는) 이 PC의 검색 경로를 통해서만 찾을 수 있었으며 앱의 일부가 아니므로 포함되지 않습니다. 이 파일이 없는 PC에서는 앱이 시작되지 않을 수 있습니다.
core-warning-build-driver-detected = 드라이버 또는 커널 구성 요소가 검색되었습니다. 앱이 소유한 드라이버는 포함되어 다시 등록되지만, 공급업체 또는 기본 제공 드라이버는 재배포되지 않으므로 서명된 공급업체 패키지로 다시 설치해야 합니다.
core-warning-build-near-no-longer-offered = 앱의 일부일 수도 있는 항목 목록에서 선택한 항목이 더 이상 목록에 없어 추가하지 않았습니다. 목록을 다시 확인하세요.
# { $app_name } is the name of a part of the app, as the list of apps calls it. { $why } is never shown: it chooses the
# reason. Translate each reason as the end of the sentence.
core-warning-build-near-part-left-out =
    { $app_name }이(가) 자신의 것이라고 표시한 항목 중 일부를 제외했습니다. { $why ->
        [unreadable] 경로가 드라이브를 벗어나거나 링크를 거치므로 안전하게 읽을 수 없기 때문입니다
        [missing] 항목이 없기 때문입니다
        [link] 링크를 거치기 때문입니다
        [short-name] 실제 이름이 아닌 다른 형식으로 기록되어 있기 때문입니다
        [another-account] 다른 계정의 폴더에 있기 때문입니다
        [one-account] 이 앱은 한 계정용으로 설치되었는데 이 항목은 그 계정의 것이 아니기 때문입니다
        [shared] 공유되는 항목이 들어 있거나 Windows의 일부이기 때문입니다
        [generic-name] 이름만으로는 누구의 것인지 알 수 없기 때문입니다
        [publisher] 게시자 이름만으로 되어 있어 게시자의 다른 제품이 들어 있을 수 있기 때문입니다
        [holds-the-app] 이 앱 자체 폴더를 포함하고 있어 다른 항목도 들어 있을 수 있기 때문입니다
        [another-apps-name] 다른 앱의 이름으로 되어 있기 때문입니다
        [another-apps-record] 설치 기록이 다른 앱의 것이기 때문입니다
        [in-another-apps-folder] 다른 앱의 폴더 안에 있기 때문입니다
       *[another-apps] 다른 앱의 것이기 때문입니다
    }.
# { $app_name } is the name of a part of the app, as the list of apps calls it.
core-warning-build-near-keys-left-out = { $app_name }의 설치 기록에 나열된 레지스트리 키를 제외했습니다. 그 기록에 다른 앱의 항목도 나와 있기 때문입니다.
# "Windows keeps what an app wrote for one account" is Windows' VirtualStore folder.
core-warning-build-virtual-store-program = 앱이 한 계정을 위해 쓴 내용을 Windows가 보관하는 폴더에 프로그램이 있었습니다. Windows는 이 폴더에 프로그램을 넣지 않으므로 다른 무언가가 넣은 것이며, 이 프로그램은 제외했습니다.
# { $path } is a file or folder, such as C:\Users\Sam\AppData\Roaming\Acme.
core-warning-build-file-left-out = { $path }을(를) 제외했습니다. 연결된 드라이브의 링크를 거치거나, 데이터를 포함하는 계정이 아닌 다른 계정의 항목입니다.
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as D:\Sam\Documents.
core-warning-build-user-folder-not-on-drive = { $user }의 { $folder } 폴더가 이 드라이브에 없는 { $path }에 있었으므로 그 안의 내용은 포함되지 않았습니다.
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as \\server\home\Sam\Documents.
core-warning-build-user-folder-not-on-this-pc = { $user }의 { $folder } 폴더가 이 PC의 디스크에 없는 { $path }에 있었으므로 그 안의 내용은 포함되지 않았습니다.
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-base-unreadable = { $path }을(를) 읽을 수 없습니다({ $error }). 이 위치에 저장된 앱 데이터는 포함되지 않았습니다.
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-folder-unreadable = { $path }을(를) 읽을 수 없습니다({ $error }). 그 안에 있는 앱 데이터가 누락되었을 수 있습니다.
# { $error } is what went wrong.
core-warning-build-entry-unreadable = 파일 시스템 항목을 읽을 수 없습니다: { $error }
# { $count } is how many links. { $names } lists some of them. { $more } is how many more there are, which are not
# named; when it is 0, the sentence ends after the list.
core-warning-build-links-not-followed =
    { $count ->
       *[other] 이 폴더에서 링크 또는 다른 항목을 열게 되는 이름 { $count }개를 따라가지 않았으므로, 그 대상은 설치 관리자에 포함되지 않습니다: { $names }
    }{ $more ->
        [0] .
       *[other] { " " }외 { $more }개.
    }
# { $count } is how many folders. { $depth } is how many levels down a folder can be. { $names } lists some of them.
# { $more } is how many more there are, which are not named; when it is 0, the sentence ends after the list.
core-warning-build-folders-too-deep =
    { $count ->
       *[other] 폴더 { $count }개가 { $depth }단계보다 깊은 곳에 있으므로, 그 안의 내용은 설치 관리자에 포함되지 않습니다: { $names }
    }{ $more ->
        [0] .
       *[other] { " " }외 { $more }개.
    }
# { $error } is what went wrong.
core-warning-build-metadata-unreadable = 파일 메타데이터를 읽을 수 없습니다: { $error }
core-warning-build-reserved-name = 이 파일의 이름은 Windows 장치용으로 예약되어 있으므로 복사하지 않았습니다.
# { $error } is what went wrong.
core-warning-build-hash-failed = 파일의 해시를 계산할 수 없습니다: { $error }
core-warning-build-cloud-placeholder = 온라인 전용 클라우드 파일(예: OneDrive 파일 주문형)에는 로컬 데이터가 없습니다. 백업하기 전에 "항상 이 디바이스에 유지"로 설정하거나 다운로드하세요. 보고만 하고 복사하지 않았습니다.
# { $file_name } is a driver's file name, such as acmeflt.sys.
core-warning-build-driver-file = 드라이버/커널 파일 '{ $file_name }'은(는) 일반 파일로 복사되지 않습니다. 이 앱에 등록된 드라이버 서비스에 사용되는 파일이면 설치 관리자가 이 파일을 포함하고 드라이버를 자동으로 다시 등록합니다. 그렇지 않으면 공급업체의 서명된 드라이버 패키지로 다시 설치하세요.
core-warning-build-file-gone = 앱을 검사할 때는 있었지만 지금은 없으므로 설치 관리자에 포함되지 않았습니다.
# { $error } is what went wrong.
core-warning-build-file-copy-failed = 파일을 패키지에 복사할 수 없습니다: { $error }
# { $count } is how many files. { $names } lists them.
core-warning-build-not-looked-through =
    { $count ->
       *[other] 이 패키지에는 누구의 PC에서 왔는지 알 수 있는 정보가 들어 있는지 확인할 수 없었던 파일이 있습니다: { $names }. 이 파일의 내용에 그런 정보가 들어 있을 수 있습니다.
    }
# { $count } is the most empty folders an installer makes, such as 10,000.
core-warning-build-empty-folders-truncated = 이 앱에는 빈 폴더가 { $count }개보다 많으며, 그중 { $count }개만 다시 만들어집니다.
core-warning-build-credential-by-now = 패키지에서 제외되었습니다: 이제는 자격 증명처럼 보입니다.
core-warning-build-no-package-path = 복사할 수 있는 파일에 패키지 상대 경로가 없습니다.
# { $file_name } is the runtime installer's file name, such as vc_redist.x64.exe. { $error } is what went wrong.
core-warning-build-prerequisite-path-invalid = 포함된 필수 구성 요소 { $file_name }을(를) 배치할 수 없습니다: { $error }
# { $runtime } is a runtime's name, such as Microsoft Visual C++ 2015+ Redistributable (x64). { $error } is what went
# wrong. winget is the name of a Windows command; keep it as it is.
core-warning-build-prerequisite-download-failed = { $runtime }을(를) 다운로드할 수 없습니다({ $error }). 설치 관리자가 대상 PC에서 winget을 사용해 봅니다.
# { $runtime } is a runtime's name. { $reason } is why the check failed, such as "it is signed by Contoso Ltd, not
# Microsoft".
core-warning-build-prerequisite-signature-failed = 다운로드한 { $runtime }이(가) 서명 검사에 실패하여 포함하지 않았습니다: { $reason }. 설치 관리자가 대상 PC에서 winget을 사용해 봅니다.
# { $runtime } is a runtime's name. { $error } is what went wrong.
core-warning-build-prerequisite-hash-failed = { $runtime }을(를) 다운로드했지만 해시를 계산할 수 없습니다({ $error }). 대신 설치 관리자가 대상 PC에서 winget을 사용해 봅니다.
# { $service } is a driver's service name, such as AcmeFlt. { $error } is what went wrong.
core-warning-build-driver-copy-failed = '{ $service }'의 드라이버 이미지를 포함할 수 없습니다: { $error }
core-warning-build-license-not-copied = 라이선스 데이터를 복사할 수 없습니다(없거나 읽을 수 없음). 메타데이터로만 기록했습니다.
core-warning-build-license-file-credential = 라이선스 파일이 자격 증명처럼 보여 제외했습니다.
# { $error } is what went wrong.
core-warning-build-license-store-walk-failed = 라이선스 저장소의 일부를 열거할 수 없습니다: { $error }
core-warning-build-license-store-credential = 라이선스 저장소의 파일이 자격 증명처럼 보여 제외했습니다.
# { $error } is what went wrong.
core-warning-build-license-store-file-skipped = 라이선스 저장소 파일을 건너뛰었습니다: { $error }
