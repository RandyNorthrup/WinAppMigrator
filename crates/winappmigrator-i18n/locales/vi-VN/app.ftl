# Words of the app, WinAppMigrator itself: its one window, with the installed apps listed on the left and, on the
# right, what goes in an installer of the one picked and the button that builds it; and the steps to read another PC
# through WinAppMigrator Client. Message names start with app-.


## The window and its header

# The program's name, in the window's title bar and at the top of the window. Keep it as it is unless your language
# writes product names in a script of its own.
app-name = WinAppMigrator
app-tagline = Biến một ứng dụng đã cài đặt thành trình cài đặt tự giải nén
app-theme-to-light = Chuyển sang chủ đề sáng
app-theme-to-dark = Chuyển sang chủ đề tối
app-reload-list = Tải lại danh sách ứng dụng
app-list-loading = Đang tải
# { $count } is how many apps are listed.
app-list-count =
    { $count ->
       *[other] { $count } ứng dụng
    }
app-list-load-failed = Tải không thành công
# Shown when the window cannot open at all. { $error } is what went wrong, in the words of Windows or a graphics driver.
app-window-could-not-open = Không thể mở cửa sổ: { $error }

## The list of installed apps

app-list-search = Tìm theo tên hoặc nhà phát hành  (Ctrl+F)
app-list-filter-all = Tất cả
# Whether an app is installed for one account or for everyone on the PC.
app-scope-per-user = Người dùng
app-scope-per-machine = Toàn máy
app-scope-unknown = Không rõ
# An app whose processor architecture (x86, x64, ARM64) is not known.
app-arch-unknown = Không rõ
app-list-loading-apps = Đang tải các ứng dụng đã cài đặt…
# { $error } is why the apps could not be listed.
app-list-could-not-list = Không thể liệt kê ứng dụng: { $error }
app-list-no-matches = Không có ứng dụng phù hợp
app-list-no-matches-hint = Hãy điều chỉnh nội dung tìm kiếm hoặc bộ lọc phạm vi.
app-list-worker-disconnected = Tiến trình kiểm kê ứng dụng đã ngắt kết nối.
# What a screen reader says of an app's row. { $name } is the app's name, { $publisher } who made it, and { $version } its
# version, such as 11.4.2, or — when either is not known.
app-list-row = { $name }, { $publisher }, phiên bản { $version }

## The list's columns, and what a screen reader says each column's heading does when clicked

app-column-name = Tên
app-column-name-sort = Sắp xếp theo tên
app-column-publisher = Nhà phát hành
app-column-publisher-sort = Sắp xếp theo nhà phát hành
app-column-version = Phiên bản
app-column-version-sort = Sắp xếp theo phiên bản
app-column-scope = Phạm vi
app-column-scope-sort = Sắp xếp theo phạm vi
# Short for architecture: x86, x64, or ARM64. The column is narrow.
app-column-arch = Kiến trúc
app-column-arch-sort = Sắp xếp theo kiến trúc

## Where apps are read from: this PC, a drive from another PC, a disk image of one, or another PC on the network

app-source = Nguồn quét
app-source-this-pc = PC này
app-source-drive = Ổ đĩa đã kết nối…
app-source-image = Ảnh đĩa…
app-source-network = PC qua mạng…
app-source-attaching-image = Đang gắn ảnh đĩa ở chế độ chỉ đọc…
# { $computer } is the other PC's name, such as OLD-PC, and { $address } its network address, such as 192.168.1.20.
app-source-network-pc = { $computer } qua mạng ({ $address })
app-source-disconnect = Ngắt kết nối
app-source-change = Thay đổi…
# The title of the window that picks a disk image. .vhd and .vhdx are file extensions.
app-source-pick-image-title = Chọn ảnh đĩa của PC kia (.vhd hoặc .vhdx)
app-source-pick-image-filter = Ảnh đĩa
# { $path } is the disk image file.
app-source-image-has-no-windows = { $path } không có ổ đĩa nào chứa Windows.
app-source-image-not-attached = Không thể gắn ảnh đĩa.
app-source-pick-drive-title = Chọn thư mục gốc của ổ đĩa đã kết nối (ví dụ: E:\)
# { $path } is the folder picked, such as E:\. \Windows and \Users are folder names, as Windows writes them.
app-source-not-a-windows-drive = { $path } không giống ổ đĩa hệ thống Windows (không có \Windows và \Users). Hãy chọn thư mục gốc của ổ đĩa lấy từ PC kia.
# Said of a drive or disk image with a Windows older than Windows 10. { $windows } is its name, such as "Windows 7
# Professional Service Pack 1, 32-bit". { $is_32_bit } is "yes" when that Windows is 32-bit, which adds the last
# sentence, and "no" when it is not.
app-source-older-windows =
    Ổ đĩa này có { $windows }. Trình cài đặt được tạo từ ổ đĩa này dành cho Windows 10 và 11. Khi chọn Dữ liệu người dùng, các thiết đặt mà Windows đã âm thầm lưu riêng cho ứng dụng cũ cũng được đưa theo, và các thư viện thời gian chạy mà Windows 11 không có, chẳng hạn DirectX 9, sẽ được cài đặt cùng ứng dụng.{ $is_32_bit ->
        [yes] { " " }Các ứng dụng trên ổ đĩa này là 32 bit, nên chúng được đưa vào Program Files (x86), nơi Windows 64 bit lưu giữ các ứng dụng như vậy.
       *[no] { "" }
    }
# The same, of a Windows whose name could not be read. { $is_32_bit } is as above.
app-source-older-windows-unnamed =
    Ổ đĩa này có một phiên bản Windows cũ hơn. Trình cài đặt được tạo từ ổ đĩa này dành cho Windows 10 và 11. Khi chọn Dữ liệu người dùng, các thiết đặt mà Windows đã âm thầm lưu riêng cho ứng dụng cũ cũng được đưa theo, và các thư viện thời gian chạy mà Windows 11 không có, chẳng hạn DirectX 9, sẽ được cài đặt cùng ứng dụng.{ $is_32_bit ->
        [yes] { " " }Các ứng dụng trên ổ đĩa này là 32 bit, nên chúng được đưa vào Program Files (x86), nơi Windows 64 bit lưu giữ các ứng dụng như vậy.
       *[no] { "" }
    }

## The details panel, before an app is picked, and the app picked

app-details-pair-first = Ghép nối với PC cũ
app-details-pair-first-hint = Khi WinAppMigrator Client đang chạy trên PC đó và bạn đã nhập mã của nó, các ứng dụng của PC đó sẽ được liệt kê ở đây, và bạn có thể tạo trình cài đặt từ một ứng dụng như với PC này.
app-details-pick-an-app = Chọn một ứng dụng
app-details-pick-an-app-hint = Hãy chọn một ứng dụng trong danh sách để chọn những gì cần đưa vào và tạo trình cài đặt cho ứng dụng đó.
# { $publisher } is who made the app, and { $version } its version, such as 11.4.2.
app-details-publisher-and-version = { $publisher }, phiên bản { $version }
# { $version } is the app's version, such as 11.4.2.
app-details-unknown-publisher-and-version = Nhà phát hành không xác định, phiên bản { $version }
app-details-unknown-publisher = Nhà phát hành không xác định

## What goes in besides the program

app-option-license = Dữ liệu giấy phép và kích hoạt
app-option-license-hint = Có thể gắn với PC này và cần được kích hoạt lại trên PC mới.
app-option-user-data = Dữ liệu người dùng
# Documents, Saved Games, and ProgramData are folders of Windows: use the names Windows gives them in your language,
# except ProgramData, which keeps its name.
app-option-user-data-hint = Thiết đặt và lịch sử, Tài liệu, Trò chơi đã lưu và ProgramData.
app-option-whose-data = Dữ liệu của ai
app-option-whose-data-hint = Một trình cài đặt chứa dữ liệu của một tài khoản. Dữ liệu đó được chuyển cho người đang đăng nhập tại nơi trình cài đặt chạy.

## Building the installer

app-output-folder = Thư mục đầu ra
# An example of a folder, shown in the empty field. "You" stands for the name of an account.
app-output-folder-example = C:\Users\Bạn\WinAppMigrator
app-output-folder-not-full = Hãy nhập đường dẫn thư mục đầy đủ, chẳng hạn D:\Installers.
app-build-create = Tạo trình cài đặt
app-build-create-again = Tạo lại trình cài đặt
app-build-cancel = Hủy
app-build-cancelling = Đang hủy sau bước hiện tại…
app-build-cancelled = Đã hủy việc tạo. Không có trình cài đặt nào được ghi.
# winappmigrator-installer.exe is a file name.
app-build-no-stub = Không tìm thấy phần khởi chạy (stub) của trình cài đặt (winappmigrator-installer.exe) bên cạnh ứng dụng này, nên không thể tạo trình cài đặt tự giải nén.
app-build-select-an-app-first = Trước tiên, hãy chọn một ứng dụng để di chuyển.
app-build-look-again-first = Cần xem xét lại ứng dụng này trước khi có thể tạo trình cài đặt.
# { $reason } is what went wrong, in the program's own words, often in English.
app-build-stopped-unexpectedly = Quá trình tạo đã dừng đột ngột: { $reason }. Nếu quá trình này để lại trình cài đặt trong thư mục đầu ra, đừng dùng trình cài đặt đó.
app-build-worker-disconnected = Tiến trình di chuyển đã ngắt kết nối.
# { $path } is the output folder, and { $error } what Windows said went wrong.
app-build-no-output-folder = Không thể tạo thư mục đầu ra { $path }: { $error }
# { $error } is what went wrong.
app-build-no-staging-folder = Không thể tạo thư mục trung gian được bảo vệ, nên không có gì được tạo: { $error }
# { $error } is what went wrong, as a sentence.
app-build-could-not-build = Không thể tạo trình cài đặt. { $error }
app-build-no-installer = Quá trình tạo đã kết thúc mà không có trình cài đặt nào.
# { $error } is what went wrong.
app-build-hash-check-did-not-run = Trình cài đặt đã được ghi, nhưng không thể chạy kiểm tra hàm băm cho trình cài đặt: { $error }. Đừng dùng trình cài đặt này.

## How far a build has got

app-progress-preparing = Đang xác định những gì được đưa vào…
app-progress-checking-room = Đang kiểm tra dung lượng trống…
app-progress-bundling = Đang đóng gói thư viện thời gian chạy và trình điều khiển…
# { $done } is how many files have been read so far, and { $total } how many there are.
app-progress-packing = Đang đọc và nén tệp: { $done } trên { $total }
# { $done } is how many of the installer's records have been looked over so far, and { $total } how many there are.
app-progress-looking-over = Đang xem xét nội dung của trình cài đặt: { $done } trên { $total }…
app-progress-writing = Đang ghi trình cài đặt…
app-progress-verifying = Đang xác minh hàm băm…

## What a build made

app-result-failed-hash-check = Đừng dùng trình cài đặt này: trình cài đặt không vượt qua kiểm tra hàm băm.
# { $count } is how many files were gone when the build came to read them.
app-result-files-gone =
    { $count ->
       *[other] Đã tạo trình cài đặt, nhưng { $count } tệp đã biến mất kể từ khi ứng dụng được xem xét, nên không có trong trình cài đặt. Hãy xem phần Cảnh báo bên dưới.
    }
# { $count } is how many files could not be read into the installer.
app-result-files-not-captured =
    { $count ->
       *[other] Đã tạo trình cài đặt, nhưng không thể thu thập { $count } tệp, thường là do tệp bị khóa hoặc không đọc được. Ứng dụng có thể không hoạt động nếu thiếu các tệp đó. Hãy xem phần Cảnh báo bên dưới.
    }
app-result-mentions-who = Đã tạo trình cài đặt, nhưng trình cài đặt vẫn còn chứa thông tin cho biết nó đến từ ai. Hãy đọc nội dung bên dưới trước khi đưa trình cài đặt cho bất kỳ ai.
app-result-ready = Trình cài đặt đã sẵn sàng. Gói của trình cài đặt đã vượt qua kiểm tra hàm băm.
app-result-files = Tệp
app-result-installer-size = Kích thước trình cài đặt
app-result-warnings = Cảnh báo
app-result-log = Nhật ký
app-result-copy-error = Sao chép thông báo lỗi
app-result-installer = Trình cài đặt
app-result-data-file = Tệp dữ liệu
app-result-open-folder = Mở thư mục
app-result-copy-path = Sao chép đường dẫn
# { $error } is what Windows said went wrong.
app-result-could-not-open-folder = Không thể mở thư mục: { $error }
# { $installer } and { $data_file } are file names, such as App_installer.exe and App_installer.wamdata.
app-result-two-files = Ứng dụng này quá lớn để nằm trong một tệp thực thi, nên trình cài đặt của ứng dụng gồm hai tệp, { $installer } và { $data_file }. Hai tệp này phải luôn đi cùng nhau: hãy sao chép cả hai sang PC kia, vào cùng một thư mục, với đúng các tên này. Nếu bạn ký trình cài đặt, chữ ký đó cũng bao gồm tệp dữ liệu.
# { $app_name } is the app's name. "Unknown publisher" is what Windows shows when it asks to run an unsigned program: use
# Windows' own words for it in your language.
app-result-run-it = Hãy chạy trình cài đặt trên PC kia để cài đặt lại { $app_name }. Trình cài đặt kiểm tra các tệp của nó bằng SHA-256. Windows hiển thị “Nhà phát hành không xác định” vì trình cài đặt do bạn tạo không được ký.
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-verification-errors = Lỗi xác minh ({ $count })
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-warnings-heading = Cảnh báo ({ $count })
# After the first of a long list of warnings. { $count } is how many more there are.
app-warnings-and-more = và { $count } mục khác.

## Looking at an app, to see what an installer of it would hold

app-scan-getting-ready = Đang chuẩn bị xem xét ứng dụng này…
app-scan-install-record = Đang đọc cách ứng dụng được cài đặt…
app-scan-near-matches = Đang tìm những gì có thể cũng là một phần của ứng dụng…
app-scan-system-items = Đang tìm dịch vụ, tác vụ và lối tắt của ứng dụng…
app-scan-files = Đang duyệt qua các tệp của ứng dụng…
app-scan-registry = Đang đọc những gì gắn ứng dụng với Windows…
app-scan-finishing = Đang hoàn tất…
app-scan-look-again = Xem xét lại
app-scan-stop = Dừng
app-scan-running-hint = Bạn có thể chọn ứng dụng khác hoặc thay đổi những gì được đưa vào trong khi quá trình này chạy.
# { $error } is why, in words that follow a colon.
app-scan-failed = Không thể xem xét ứng dụng này: { $error }
app-scan-try-again = Thử lại
app-scan-stopped = Đã dừng. Có thể tạo trình cài đặt sau khi ứng dụng này đã được xem xét.
# { $reason } is what went wrong, in the program's own words, often in English.
app-scan-stopped-unexpectedly = Quá trình đã dừng đột ngột: { $reason }.

## How big an installer can be, and whether there is room for it

app-room-files = Tệp
# Over how big the installer can be: up to that size.
app-room-up-to = Tối đa
app-room-free = Còn trống
# { $drive } is a drive, such as C:.
app-room-free-on = Còn trống trên { $drive }
# How much room a drive has, when it would not say.
app-room-free-unknown = Không xác định
app-room-most-it-could-need = Dung lượng tối đa có thể cần, trước khi nén.
# { $drive } is a drive, such as C:, { $needed } how much room it needs, such as 1.2 GB, and { $free } how much it has.
app-room-drive-short = { $drive } cần tối đa { $needed } dung lượng trống để tạo trình cài đặt này, nhưng chỉ còn { $free }.
# { $drives } is one sentence like the one above for each drive without the room, one after another.
app-room-not-enough = Không đủ dung lượng. { $drives } Hãy chọn thư mục đầu ra trên ổ đĩa khác, hoặc bỏ bớt một số mục.
# { $note } is a sentence, without its full stop, about a drive whose room could not be checked. Use your language's
# full stop.
app-room-could-not-tell = { $note }.

## What goes in the installer, part by part

app-footprint-heading = Những gì được đưa vào trình cài đặt
app-footprint-working-out = Đang xác định những gì đi kèm với mục đó…
# { $error } is why.
app-footprint-would-not-make-a-package = Phần còn lại không thể tạo thành một gói: { $error }
# { $list } is the names of what else is left out, such as a service and a shortcut.
app-footprint-also-left-out = Việc đó cũng loại trừ { $list }, vì những gì chúng chạy hoặc trỏ tới sẽ không có ở đó. Chúng sẽ không được thiết lập trên PC kia.
# { $list } is the names of the first few of what else is left out, and { $count } how many more there are.
app-footprint-also-left-out-and-more = Việc đó cũng loại trừ { $list } và { $count } mục khác, vì những gì chúng chạy hoặc trỏ tới sẽ không có ở đó. Chúng sẽ không được thiết lập trên PC kia.
# { $list } is the names of license files. The option is the one named "License and Activation Data".
app-footprint-licenses-still-go = Dữ liệu giấy phép trong những gì bạn đã loại trừ vẫn được đưa vào, vì phần đó do tùy chọn Dữ liệu giấy phép và kích hoạt quyết định: { $list }.
# { $elapsed } is how long ago, in minutes and seconds such as 45:00, or hours, minutes, and seconds such as 1:05:00.
# "Look Again" names the button of that name.
app-footprint-looked-at-long-ago = Ứng dụng này đã được xem xét cách đây { $elapsed }. Tệp được đọc khi bạn tạo trình cài đặt, nên mọi thay đổi đều được cập nhật, và những gì đã biến mất lúc đó sẽ được liệt kê. Hãy chọn Xem xét lại để tìm những gì mới.
# Headings of the parts of the list, each with a line that says what is in it.
app-footprint-program-files = Tệp chương trình
app-footprint-data-folders = Thư mục dữ liệu
# What could go in with the User Data option, when nothing else is in this part. { $size } is how much, such as 1.2 GB.
app-footprint-folders-with-user-data = { $size } với tùy chọn Dữ liệu người dùng
# { $included } is how many folders go in, { $count } how many there are, and { $size } how much those that go in hold.
app-footprint-folders-summary =
    { $count ->
       *[other] { $included } trên { $count } thư mục, { $size }
    }
# { $size } is how much, such as 1.2 GB, and { $count } in how many files.
app-footprint-data-only-with-user-data =
    { $count ->
       *[other] { $size } trong { $count } tệp dữ liệu của ứng dụng này chỉ được đưa vào khi có tùy chọn Dữ liệu người dùng.
    }
# A folder's row. { $count } is how many files it holds, and { $detail } where it is or why it cannot be left out.
app-footprint-folder-row =
    { $count ->
       *[other] { $count } tệp. { $detail }
    }
app-footprint-services = Dịch vụ
app-footprint-startup = Tự khởi động
app-footprint-tasks = Tác vụ đã lên lịch
app-footprint-shortcuts = Lối tắt
# { $included } is how many of the part's things go in, and { $count } how many there are.
app-footprint-items-summary = { $included } trên { $count }
# { $why } is what else was left out that this goes with.
app-footprint-left-out-too = Cũng bị loại trừ: { $why }
# A row's detail. { $kind } is what it is, such as Service, and { $detail } what it does.
app-footprint-change-detail = { $kind }. { $detail }
app-footprint-integrations = Những gì gắn ứng dụng với Windows
# { $included } is how many kinds go in, such as file types and firewall rules, and { $count } how many kinds there are.
app-footprint-integrations-summary =
    { $count ->
       *[other] { $included } trên { $count } loại
    }
app-footprint-integration-nothing-left = Không còn gì, do những gì bạn đã loại trừ
# { $count } is how many things of this kind the installer sets up.
app-footprint-integration-things =
    { $count ->
       *[other] { $count } mục được thiết lập
    }
app-footprint-prerequisites = Những gì ứng dụng cần để chạy
# { $count } is how many runtimes, such as Visual C++ Redistributable.
app-footprint-prerequisites-summary =
    { $count ->
       *[other] { $count } thư viện thời gian chạy
    }
app-footprint-near-matches = Có thể cũng là một phần của ứng dụng
# { $ticked } is how many were ticked, and { $count } how many there are.
app-footprint-near-matches-summary = Đã thêm { $ticked } trên { $count }
app-footprint-near-matches-hint = Không được đưa vào trình cài đặt trừ khi bạn chọn. Khi bạn chọn một mục, ứng dụng sẽ được xem xét lại với cùng các bước kiểm tra như phần còn lại, nên những gì trông giống mật khẩu vẫn bị loại trừ.
# { $why } is why it might be part of the app, as a sentence.
app-footprint-near-match-with-user-data = Đi kèm tùy chọn Dữ liệu người dùng. { $why }
# How much a folder holds, when not all of it could be measured. { $size } is how much was, such as 1.2 GB.
app-footprint-near-match-over = hơn { $size }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-folder = Thư mục: { $place }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-hidden = Phần ẩn của một ứng dụng: { $place }
app-footprint-nothing-left-out = Không có gì bị loại trừ vì trông giống mật khẩu, giấy phép hoặc chứng chỉ.
app-footprint-left-out = Mục bị loại trừ và lý do
# { $count } is how many things were left out.
app-footprint-left-out-summary =
    { $count ->
       *[other] { $count } mục
    }
# Above a list of things left out for one reason. { $title } is the reason.
app-footprint-left-out-group = { $title }:
app-footprint-refused = Không trình cài đặt nào được phép thiết lập những mục này:
# A heading over a list that opens and closes. { $count } is how many are in it.
app-footprint-worth-knowing = Thông tin cần biết ({ $count })
# { $list } is what kinds of things were looked for and none found, from the five below.
app-none-found = Không tìm thấy: { $list }.
app-none-found-services = dịch vụ
app-none-found-startup = mục khởi động
app-none-found-tasks = tác vụ đã lên lịch
app-none-found-shortcuts = lối tắt
app-none-found-integrations = bất cứ thứ gì gắn ứng dụng với Windows

## Network PC: reading another PC through WinAppMigrator Client, in three steps. "WinAppMigrator Client" is the name of a
## program, and the old PC is the one the apps are moved off.

app-network-step-client = 1. Đưa ứng dụng khách vào PC cũ
app-network-step-client-hint = WinAppMigrator Client chạy trên PC hiện đang có ứng dụng, từ Windows 7 trở lên. Ứng dụng khách chỉ cho phép PC này đọc PC đó, qua kết nối được mã hóa, bằng mã mà ứng dụng khách hiển thị.
# { $drive } is a USB drive, such as E:\.
app-network-save-to-usb = Lưu vào ổ USB { $drive }
# { $drive } is a USB drive, such as E:\, and { $name } the name it was given, such as BACKUP.
app-network-save-to-named-usb = Lưu vào ổ USB { $drive } ({ $name })
app-network-save-to-folder = Lưu vào thư mục…
app-network-look-for-usb-again = Tìm lại ổ USB
app-network-no-usb = Chưa cắm ổ USB nào. Hãy cắm một ổ USB, hoặc lưu vào một thư mục mà bạn có thể truy cập từ PC cũ, chẳng hạn thư mục dùng chung trên mạng.
# { $path } is where the client was saved, such as E:\WinAppMigrator Client.exe.
app-network-client-saved = Đã lưu thành { $path }. Trên PC cũ, hãy đóng ứng dụng bạn đang chuyển, rồi bấm đúp vào tệp này và cho phép chương trình thực hiện thay đổi. Chương trình sẽ hiển thị một mã.
app-network-step-pick = 2. Chọn PC cũ
app-network-looking = Đang tìm trên mạng các PC đang chạy ứng dụng khách…
app-network-type-name = Hoặc nhập tên hoặc địa chỉ của PC đó
# An example, shown in the empty field: a PC's name, or its network address.
app-network-type-name-example = OLD-PC hoặc 192.168.1.20
app-network-step-code = 3. Nhập mã mà PC đó hiển thị
app-network-code = Mã ghép nối
app-network-connect = Kết nối
app-network-pairing = Đang ghép nối…
# Why pairing did not work, shown on its own.
app-network-pairing-stopped = quá trình ghép nối đã dừng trước khi hoàn tất
app-network-wrong-code = đó không phải là mã mà PC cũ hiển thị. Hãy kiểm tra rồi thử lại.
# { $error } is what went wrong.
app-network-not-reached = không thể kết nối tới PC cũ: { $error }
app-network-pick-folder-title = Chọn thư mục cho WinAppMigrator Client
# { $path } is the folder, and { $error } what Windows said went wrong.
app-client-could-not-save = không thể lưu ứng dụng khách vào { $path }: { $error }
# scripts\build-client.ps1 is a file name.
app-client-not-carried = bản dựng WinAppMigrator này không kèm ứng dụng khách. Hãy biên dịch ứng dụng khách bằng scripts\build-client.ps1, rồi biên dịch lại ứng dụng.
# { $error } is what Windows said went wrong.
app-client-could-not-save-note = không thể lưu tệp ghi chú bên cạnh ứng dụng khách: { $error }
# A note saved as a text file beside the client, for whoever runs it on the old PC. Keep its lines short, as it is read in
# Notepad. "WinAppMigrator Client.exe" is a file name. "Network PC..." names the Scan Source button: use its words in your
# language.
app-client-note =
    WinAppMigrator Client
    
    Hãy chạy chương trình này trên PC mà bạn đang chuyển ứng dụng đi:
    Windows 7, 8, 10 hoặc 11.
    
    1. Đóng ứng dụng bạn đang chuyển.
    2. Bấm đúp vào “WinAppMigrator Client.exe” và cho phép nó thực hiện thay đổi.
    3. Chương trình hiển thị một mã ghép nối gồm 6 chữ số. Trong WinAppMigrator
       trên PC mới, hãy chọn PC qua mạng..., chọn PC này và nhập mã.
    4. Khi PC mới đã xong, hãy nhấn Enter trong cửa sổ của ứng dụng khách, hoặc
       đóng cửa sổ đó.
    
    Chương trình chỉ đọc. Trong khi chạy, chương trình giữ một ảnh chụp nhanh của
    ổ đĩa Windows và mở hai cổng riêng của nó trên Tường lửa Windows cho mạng
    cục bộ của bạn, rồi gỡ bỏ cả hai khi dừng. Dữ liệu truyền qua mạng được
    mã hóa, và chỉ PC có mã mới đọc được bất cứ thứ gì.
