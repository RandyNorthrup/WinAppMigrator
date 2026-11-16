# Words of WinAppMigrator Client, the console program run on the old PC, the one an app is moving off, so that
# WinAppMigrator on the new PC can read it over the network. Message names start with client-.
#
# Its window is a console, which on Windows 7 is 80 columns wide. Each line break in a message is one on the screen:
# keep every line shorter than 80 characters, and break lines where your language needs to, not only where English
# does.


## Starting

# Shown only in English, in place of a right-to-left language the console cannot show. { $language } is that
# language's name in English, such as Arabic.
client-console-rtl-fallback = 이 콘솔에서는 { $language }을(를) 표시할 수 없으므로 클라이언트가 영어를 사용합니다.
# The client copies itself to a folder in Program Files before it starts. { $error } is what Windows said went wrong.
client-safe-folder-failed =
    관리자만 변경할 수 있는 폴더에서 WinAppMigrator Client를 시작할 수
    없습니다: { $error }
client-press-enter-to-close = 이 창을 닫으려면 Enter 키를 누르세요.
# The first line of its screen. { $version } is the client's version, such as 0.6.2.
client-heading = WinAppMigrator Client { $version }
client-intro =
    새 PC의 WinAppMigrator가 네트워크를 통해 이 PC의 앱을 읽을 수 있게
    합니다. 이 PC에서는 아무것도 변경하지 않습니다. Windows 드라이브의
    스냅숏을 읽으며, 클라이언트를 중지하면 스냅숏을 제거합니다.
# { $error } is why: one of the messages under "Why it stopped" below, or what Windows said.
client-stopped = 클라이언트가 중지되었습니다: { $error }
# The caretaker is a copy of the client with no window that cleans up after it. { $error } is what Windows said.
client-caretaker-failed =
    이 창을 중지하지 않고 닫으면, 다음에 이 PC에서 클라이언트를 실행할
    때까지 스냅숏과 방화벽 규칙이 남아 있습니다({ $error }).
# Shown while the snapshot is taken, followed on the same line by client-snapshot-taken. Keep the quotes: they keep
# the space before it.
client-snapshot-taking = Windows 드라이브의 스냅숏을 만드는 중...{ " " }
client-snapshot-taken = 완료.
# The old PC answers the new one's search for it on the network. { $error } is what Windows said went wrong.
client-discovery-failed =
    이 PC는 네트워크에서 자동으로 검색되지 않습니다({ $error }).
    새 PC에서 이 PC의 이름이나 주소를 직접 입력하세요.

## This PC and its pairing code. The three lines are lined up in two columns, behind two spaces; keep the quotes, which
## keep those spaces.

# { $computer_name } is this PC's name, such as OLD-PC.
client-this-pc = { "  " }이 PC:         { $computer_name }
# { $windows_name } is this PC's Windows, such as Windows 7 Professional Service Pack 1. { $bits } is 32 or 64.
client-windows = { "  " }Windows:       { $windows_name }, { $bits }비트
# { $code } is six digits in two groups, such as 482 913.
client-pairing-code = { "  " }페어링 코드:   { $code }
# Network PC... is the button of that name in WinAppMigrator: use the name it has there in your language.
client-pairing-steps =
    새 PC의 WinAppMigrator에서 "네트워크 PC..."를 선택하고 이 코드를
    입력하세요. 앱의 파일이 마지막 상태 그대로 있도록, 옮기려는 앱을
    먼저 닫으세요.
    중지하려면 여기에서 Enter 키를 누르거나 이 창을 닫으세요.

## While it runs. { $from } is the network address of the PC that connected, such as 192.168.1.20, or "unknown" when
## it could not be told, which the [unknown] variant words.

client-paired =
    { $from ->
        [unknown] 다른 PC와
       *[address] { $from }와(과)
    } 페어링되었습니다. 필요한 항목을 읽는 중...
# { $count } is how many requests it answered.
client-builder-done =
    { $from ->
        [unknown] 다른 PC에서
       *[address] { $from }에서
    } 작업을 마쳤습니다(요청 { $count ->
       *[other] { $count }개
    }에 응답함).
# { $error } is what went wrong, often in Windows' words.
client-link-ended =
    { $from ->
        [unknown] 다른 PC와의
       *[address] { $from }와(과)의
    } 연결이 끊어졌습니다: { $error }
client-wrong-code =
    { $from ->
        [unknown] 다른 PC에서
       *[address] { $from }에서
    } 이 PC의 코드가 아닌 코드를 입력했습니다.
# { $error } is what went wrong, often in Windows' words.
client-pair-failed =
    { $from ->
        [unknown] 다른 PC에서
       *[address] { $from }에서
    } 페어링하지 못했습니다: { $error }
client-too-many-wrong-codes =
    잘못된 코드 입력이 너무 많아, 이제 어떤 PC도 이 PC와 페어링할 수
    없습니다. 네트워크의 무언가가 코드를 추측하고 있을 수 있습니다.
    페어링하려면 클라이언트를 중지한 후 다시 시작하고, 새로 표시되는
    코드를 입력하세요.
# { $code } is six digits in two groups, such as 482 913.
client-new-code = 잘못된 코드가 너무 많습니다. 새 페어링 코드: { $code }

## Stopping

client-stopping = 중지하는 중: 스냅숏과 방화벽 규칙을 제거하는 중...
client-stop-done = 완료.

## Why it stopped, shown after "The client stopped: " (client-stopped), and written to follow it.

client-already-running = 이 PC에서 클라이언트가 이미 실행 중이거나, 닫힌 클라이언트의 정리 작업이 진행 중입니다. 실행 중인 클라이언트를 사용하거나, 잠시 기다린 후 이 클라이언트를 다시 시작하세요.
# { $drive } is the drive Windows is on, such as C:. { $details } is what Windows said, in its own words.
client-snapshot-failed = Windows에서 { $drive }의 스냅숏을 만들지 못했습니다: { $details }
# { $details } is what Windows Firewall said, in its own words.
client-firewall-failed = Windows 방화벽에서 클라이언트의 수신 대기를 허용하지 않았습니다: { $details }

## What it takes, shown for --help. The options, such as --lang, are typed as they are and are not translated; keep
## their descriptions lined up.

client-usage =
    WinAppMigrator Client에서 사용할 수 있는 옵션:
    
      --lang <언어>      이 언어(예: it-IT)를 사용합니다. 클라이언트에 이
                         언어가 있으면 Windows 표시 언어 대신 사용합니다.
      --code <코드>      이 6자리 페어링 코드로 시작합니다. 새 PC에서
                         WinAppMigrator도 함께 시작하는 스크립트용입니다.
      --help             이 도움말을 표시합니다.
