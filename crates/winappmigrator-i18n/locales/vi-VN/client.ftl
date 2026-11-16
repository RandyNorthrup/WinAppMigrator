# Words of WinAppMigrator Client, the console program run on the old PC, the one an app is moving off, so that
# WinAppMigrator on the new PC can read it over the network. Message names start with client-.
#
# Its window is a console, which on Windows 7 is 80 columns wide. Each line break in a message is one on the screen:
# keep every line shorter than 80 characters, and break lines where your language needs to, not only where English
# does.


## Starting

# Shown only in English, in place of a right-to-left language the console cannot show. { $language } is that
# language's name in English, such as Arabic.
client-console-rtl-fallback = Cửa sổ dòng lệnh này không thể hiển thị { $language }, nên ứng dụng khách dùng tiếng Anh.
# The client copies itself to a folder in Program Files before it starts. { $error } is what Windows said went wrong.
client-safe-folder-failed =
    WinAppMigrator Client không thể khởi động từ thư mục mà chỉ quản trị viên
    mới có thể thay đổi: { $error }
client-press-enter-to-close = Nhấn Enter để đóng cửa sổ này.
# The first line of its screen. { $version } is the client's version, such as 0.6.2.
client-heading = WinAppMigrator Client { $version }
client-intro =
    Chương trình này cho phép WinAppMigrator trên PC mới của bạn đọc các ứng dụng
    trên PC này qua mạng. Nó không thay đổi gì ở đây: nó đọc một ảnh chụp nhanh
    của ổ đĩa Windows và sẽ xóa ảnh chụp nhanh đó khi bạn dừng nó.
# { $error } is why: one of the messages under "Why it stopped" below, or what Windows said.
client-stopped = Ứng dụng khách đã dừng: { $error }
# The caretaker is a copy of the client with no window that cleans up after it. { $error } is what Windows said.
client-caretaker-failed =
    Nếu cửa sổ này bị đóng thay vì được dừng, ảnh chụp nhanh và các quy tắc
    tường lửa sẽ còn lại cho đến lần tới ứng dụng khách chạy ở đây ({ $error }).
# Shown while the snapshot is taken, followed on the same line by client-snapshot-taken. Keep the quotes: they keep
# the space before it.
client-snapshot-taking = Đang tạo ảnh chụp nhanh của ổ đĩa Windows...{ " " }
client-snapshot-taken = xong.
# The old PC answers the new one's search for it on the network. { $error } is what Windows said went wrong.
client-discovery-failed =
    PC này sẽ không tự hiện ra trên mạng ({ $error }).
    Trên PC mới, hãy nhập tên hoặc địa chỉ của PC này.

## This PC and its pairing code. The three lines are lined up in two columns, behind two spaces; keep the quotes, which
## keep those spaces.

# { $computer_name } is this PC's name, such as OLD-PC.
client-this-pc = { "  " }PC này:        { $computer_name }
# { $windows_name } is this PC's Windows, such as Windows 7 Professional Service Pack 1. { $bits } is 32 or 64.
client-windows = { "  " }Windows:       { $windows_name }, { $bits } bit
# { $code } is six digits in two groups, such as 482 913.
client-pairing-code = { "  " }Mã ghép nối:   { $code }
# Network PC... is the button of that name in WinAppMigrator: use the name it has there in your language.
client-pairing-steps =
    Trên PC mới, trong WinAppMigrator, hãy chọn PC qua mạng... và nhập mã này.
    Trước tiên, hãy đóng ứng dụng bạn đang chuyển, để các tệp của ứng dụng đó
    giữ nguyên như lúc nó được đóng.
    Để dừng, hãy nhấn Enter tại đây hoặc đóng cửa sổ này.

## While it runs. { $from } is the network address of the PC that connected, such as 192.168.1.20, or "unknown" when
## it could not be told, which the [unknown] variant words.

client-paired =
    Đã ghép nối với { $from ->
        [unknown] một PC khác
       *[address] { $from }
    }. Đang đọc những gì cần thiết...
# { $count } is how many requests it answered.
client-builder-done =
    { $from ->
        [unknown] Một PC khác
       *[address] { $from }
    } đã xong (đã trả lời { $count ->
       *[other] { $count } yêu cầu
    }).
# { $error } is what went wrong, often in Windows' words.
client-link-ended =
    Kết nối với { $from ->
        [unknown] một PC khác
       *[address] { $from }
    } đã kết thúc: { $error }
client-wrong-code =
    { $from ->
        [unknown] Một PC khác
       *[address] { $from }
    } đã thử một mã không phải mã này.
# { $error } is what went wrong, often in Windows' words.
client-pair-failed =
    { $from ->
        [unknown] Một PC khác
       *[address] { $from }
    } không thể ghép nối: { $error }
client-too-many-wrong-codes =
    Đã có quá nhiều mã sai được thử, nên hiện không PC nào có thể ghép nối với
    PC này. Có thể có thứ gì đó trên mạng của bạn đang dò mã. Để ghép nối, hãy
    dừng ứng dụng khách rồi khởi động lại, sau đó nhập mã mới mà nó hiển thị.
# { $code } is six digits in two groups, such as 482 913.
client-new-code = Quá nhiều mã sai. Mã ghép nối mới là: { $code }

## Stopping

client-stopping = Đang dừng: đang xóa ảnh chụp nhanh và các quy tắc tường lửa...
client-stop-done = Xong.

## Why it stopped, shown after "The client stopped: " (client-stopped), and written to follow it.

client-already-running = ứng dụng khách đang chạy trên PC này, hoặc đang dọn dẹp sau một phiên đã bị đóng. Hãy dùng phiên đó, hoặc đợi một lát rồi khởi động lại phiên này.
# { $drive } is the drive Windows is on, such as C:. { $details } is what Windows said, in its own words.
client-snapshot-failed = Windows không tạo được ảnh chụp nhanh của { $drive }: { $details }
# { $details } is what Windows Firewall said, in its own words.
client-firewall-failed = Tường lửa Windows không cho phép ứng dụng khách nhận kết nối: { $details }

## What it takes, shown for --help. The options, such as --lang, are typed as they are and are not translated; keep
## their descriptions lined up.

client-usage =
    WinAppMigrator Client nhận các tùy chọn sau:
    
      --lang <ngôn ngữ>  Dùng ngôn ngữ này, chẳng hạn it-IT, nếu ứng dụng khách
                         có ngôn ngữ đó, thay cho ngôn ngữ hiển thị của Windows.
      --code <mã>        Bắt đầu với mã ghép nối gồm sáu chữ số này, dành cho tập
                         lệnh cũng khởi động WinAppMigrator trên PC mới.
      --help             Hiển thị nội dung này.
