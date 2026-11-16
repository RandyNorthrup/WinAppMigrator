# Words of the installer that every built installer carries, which runs on the PC an app is moved to and speaks its
# language, and of the uninstaller it leaves behind for Settings. Message names start with installer- for the
# installer's window, installer-cli- for its command line, and uninstaller- for the uninstaller.
#
# The command lines (installer-cli-, uninstaller-cli-) are always shown in English, as scripts read what they print.
# Translate them all the same. Keep their command-line flags, such as --apply and --target-root, exactly as they are.


## The installer's window: its title and header

installer-window-title = Trình cài đặt WinAppMigrator
# The window's title in the taskbar. { $app_name } is the name of the app it installs.
installer-window-title-app = Cài đặt { $app_name }
installer-tagline = Cài đặt lại một ứng dụng đã được thu thập trên PC khác
# What an app is called when its package gives it no name.
installer-unnamed-app = Ứng dụng

## When the installer's window cannot open. Each is a paragraph of a message box.

# { $error } is what went wrong, as Windows or the graphics driver says it.
installer-window-failed = Không thể mở cửa sổ của trình cài đặt: { $error }
installer-window-failed-without-one = Trình cài đặt vẫn có thể cài đặt mà không cần cửa sổ: hãy chạy tệp này từ dấu nhắc lệnh với quyền quản trị viên kèm --apply, và kèm --target-root theo sau là một thư mục để chọn nơi cài đặt.

## Checking the installer when it opens

installer-checking = Đang kiểm tra trình cài đặt…
installer-stub-no-package = Đây là phần khởi chạy (stub) của trình cài đặt WinAppMigrator và không chứa gói ứng dụng nào. Hãy tạo trình cài đặt bằng WinAppMigrator, rồi chạy tệp .exe được tạo ra tại đây.
installer-check-stopped = Quá trình kiểm tra trình cài đặt đã dừng.
# { $error } is what went wrong, as Windows says it.
installer-locate-failed = Không thể xác định vị trí của trình cài đặt: { $error }
# { $error } is what went wrong reading it.
installer-read-failed = Không thể đọc trình cài đặt: { $error }
# { $error } is why, such as that the app needs a newer Windows.
installer-cannot-be-used-here = Không thể dùng gói của trình cài đặt này trên PC này: { $error }

## The app, where it goes, and whether the installer is signed

installer-captured-from = Thu thập từ
installer-install-location = Vị trí cài đặt
# The title of the folder picker.
installer-choose-location = Chọn vị trí cài đặt
installer-folder-has-files = Thư mục này đã có tệp. Các tệp trùng tên sẽ bị thay thế và sẽ được khôi phục nếu bạn gỡ cài đặt.
installer-room-needed = Dung lượng cần thiết
# { $shortfall } says which drive is short of room, such as "C: needs up to 824.0 MB for the app's files, and has 300.0 MB free".
installer-room-short = Có thể không đủ dung lượng ở đây: { $shortfall }. Gói có thể được giải nén trên ổ đĩa khác, nhưng vẫn phải đủ chỗ cho các tệp của ứng dụng.
# { $app_name } is the name of an app whose earlier install was cut short.
installer-interrupted = Một lần cài đặt { $app_name } trước đây chưa hoàn tất. Thao tác Cài đặt sẽ hoàn tác lần đó trước.
# Beside "Installer", for whether it proves who made it.
installer-signature = Trình cài đặt
installer-signature-checking = Đang kiểm tra chữ ký…
installer-signature-unsigned = Chưa được ký: nhà phát hành không xác định
# { $publisher } is the name on the installer's certificate, such as "Fabrikam, Inc.".
installer-signature-signed-by = Được ký bởi { $publisher }
installer-signature-unnamed = Được ký bởi một nhà phát hành không có tên
# { $reason } is Windows' own word for what is wrong, such as HashMismatch.
installer-signature-invalid = Chữ ký của trình cài đặt này không hợp lệ ({ $reason }). Trình cài đặt có thể đã bị thay đổi sau khi được ký.

## Why the install location cannot be used. Install and Preview Changes stay off while one of these is shown.

# { $app_name } is the name of the app being installed.
installer-location-app-already-there = { $app_name } đã được cài đặt trong thư mục này, nên không thể đặt bản sao này ở đây. Để cài đặt ở đây, trước tiên hãy gỡ cài đặt bản sao trên PC này trong Cài đặt của Windows, mục Ứng dụng, rồi mở lại trình cài đặt này. Hoặc nhập một thư mục mới, trống ở trên.
# { $other_app } is the name of another app Windows lists as installed in the folder. { $app_name } is the name of the app being installed.
installer-location-other-app-there = { $other_app } đã được cài đặt trong thư mục này, nên không thể đặt { $app_name } ở đây. Hãy nhập một thư mục mới, trống ở trên. Hoặc, nếu bạn không còn cần { $other_app }, trước tiên hãy gỡ cài đặt ứng dụng đó trong Cài đặt của Windows, mục Ứng dụng, rồi mở lại trình cài đặt này.
installer-location-has-files = Thư mục này đã có các tệp không do WinAppMigrator cài đặt. Các tệp đó có thể thuộc về ứng dụng khác, nên không có gì được cài đặt đè lên chúng. Hãy nhập một thư mục mới, trống ở trên.
# { $reason } is why, such as that it cannot be the Windows folder.
installer-location-refused = Không thể dùng thư mục này: { $reason }. Hãy nhập một thư mục khác ở trên.

## What the install sets up beyond its files

installer-beyond-files = Những gì được thiết lập ngoài tệp
# Under the heading above. It is part of what a person agrees to when they click Install.
installer-beyond-files-agreed = Tất cả những việc này được thực hiện với quyền quản trị viên. Nếu cài đặt thất bại, những gì đã thay đổi sẽ được khôi phục lại, và bạn sẽ được thông báo về bất kỳ điều gì không thể khôi phục lại. Bạn cũng có thể hoàn tác lần cài đặt này sau đó trong Cài đặt, mục Ứng dụng đã cài đặt. Các tệp bị thay thế sẽ được khôi phục, còn các thư viện thời gian chạy đã cài đặt thì được giữ lại.
installer-beyond-files-none = Không có gì: trình cài đặt chỉ sao chép tệp và các thiết đặt trong khóa sổ đăng ký riêng của ứng dụng.
# { $problem } is why, such as that the app cannot run on this PC.
installer-cannot-go-here = Không thể đặt gói này tại vị trí này: { $problem }
installer-working-out = Đang xác định những gì sẽ được thiết lập tại vị trí này…
# Said as the reason after "This package cannot go to this location:".
installer-list-stopped = quá trình xác định những việc sẽ làm đã dừng
# A heading over a list of what else the package asked for and will not get. { $count } is how many.
installer-more-left-out =
    { $count ->
       *[other] Còn { $count } mục khác bị loại trừ
    }

## Options

installer-options = Tùy chọn
installer-user-data = Dữ liệu người dùng
installer-user-data-detail = Bật: thiết đặt, Tài liệu và Trò chơi đã lưu được đưa vào các thư mục thông thường của người đang đăng nhập. Tắt: tất cả được đặt trong thư mục cài đặt, và không có gì được thiết lập để chạy từ đó.
# { $account } is the name of the account on the other PC whose data this is.
installer-user-data-whose = Đây là dữ liệu của { $account }.
installer-license = Giấy phép
installer-license-detail = Các tệp kích hoạt và giấy phép, cùng các giá trị sổ đăng ký chứa giấy phép. Một số có thể cần được kích hoạt lại ở đây.

## Dependencies: what else the app needs, and how each is provided

installer-dependencies = Phần phụ thuộc
installer-dependencies-detail = Những gì ứng dụng này cần và cách cung cấp từng thành phần.
installer-dependencies-none = Không phát hiện phần phụ thuộc thời gian chạy bổ sung nào.
# { $feature } is the name of an optional part of Windows, such as NetFx3.
installer-dependency-windows-feature = Tính năng Windows: { $feature }
installer-dependency-included = Đã bao gồm
# { $id } is the package's name in winget, such as Microsoft.VCRedist.2015+.x64.
installer-dependency-winget = Được cài đặt bằng winget: { $id }
installer-dependency-manual = Cài đặt thủ công
installer-dependency-built-in = Có sẵn trong Windows
installer-dependency-ships-with-app = Đi kèm ứng dụng

## Buttons, and what is said beside them

installer-install = Cài đặt
installer-preview = Xem trước thay đổi
installer-cancel = Hủy
installer-close = Đóng
installer-open-folder = Mở thư mục cài đặt
# { $error } is what went wrong, as Windows says it.
installer-open-folder-failed = Không thể mở thư mục: { $error }
installer-location-blocks = Cài đặt và Xem trước thay đổi bị tắt cho đến khi có thể dùng Vị trí cài đặt. Lý do nằm ở trên, bên dưới trường đó.
installer-package-blocks = Cài đặt bị tắt vì không thể đặt gói này tại vị trí này. Lý do nằm ở trên.
installer-list-changed = Những gì trình cài đặt sẽ làm ở đây đã thay đổi, nên không có gì được cài đặt. Hãy xem lại danh sách bên dưới, rồi bấm Cài đặt lần nữa nếu đó là điều bạn muốn.
installer-cancelled-early = Đã hủy. Không có gì bị thay đổi.

## How far an install has got, beside its progress bar

# { $step } is one of the steps below. { $done } and { $total } are how many of its items are done, and how many there are.
installer-step-counted = { $step }: { $done } trên { $total }…
# { $step } is one of the steps below, when it does not count its items.
installer-step-uncounted = { $step }…
installer-step-getting-ready = Đang chuẩn bị…
installer-step-cancelling = Đang hủy sau bước hiện tại…
installer-step-putting-back = Đang khôi phục lại những gì đã thay đổi
installer-step-unpacking = Đang giải nén
installer-step-checking-package = Đang kiểm tra gói
installer-step-recovering = Đang hoàn tác một lần cài đặt trước chưa hoàn tất
# What a preview is doing, whatever step it is on.
installer-step-previewing = Đang xác định những gì sẽ thay đổi
installer-step-runtimes = Đang cài đặt thư viện thời gian chạy
installer-step-files = Đang sao chép tệp
installer-step-services = Đang thiết lập dịch vụ
installer-step-drivers = Đang thiết lập trình điều khiển
installer-step-registry = Đang ghi thiết đặt sổ đăng ký
installer-step-license = Đang khôi phục dữ liệu giấy phép
installer-step-tasks = Đang thiết lập tác vụ đã lên lịch
installer-step-shortcuts = Đang tạo lối tắt
installer-step-environment = Đang đặt biến môi trường
installer-step-settings-entry = Đang thêm ứng dụng vào Cài đặt
installer-step-verifying = Đang kiểm tra những gì đã cài đặt
installer-step-undoing = Đang hoàn tác

## When an install cannot go ahead or stops

installer-carries-no-package = Trình cài đặt này không chứa gói nào.
installer-stopped-unexpectedly = Quá trình cài đặt đã dừng đột ngột.
# { $reason } is what went wrong, as the program says it.
installer-stopped-unexpectedly-because = Quá trình cài đặt đã dừng đột ngột: { $reason }. Nếu quá trình đã bắt đầu, việc chạy lại trình cài đặt sẽ hoàn tác những gì nó đã làm trước.
# { $error } is what went wrong reading it.
installer-package-read-failed = Không thể đọc gói: { $error }
installer-no-programdata = Không tìm thấy thư mục ProgramData của PC này, nên không có gì được cài đặt: mỗi lần cài đặt đều lưu tại đó bản ghi dùng để hoàn tác lần cài đặt ấy.
# { $error } is why, such as that there is nowhere only administrators can get into.
installer-staging-failed = Không thể tạo thư mục trung gian: { $error }
# { $error } is what went wrong unpacking it.
installer-unpack-failed = Không thể giải nén gói: { $error }
installer-staging-nowhere = không có nơi nào chỉ quản trị viên mới truy cập được để giải nén gói
# { $failure } is a folder and what went wrong there, such as "C:\ProgramData\WinAppMigrator: Access is denied.".
installer-staging-nowhere-look-at = không có nơi nào chỉ quản trị viên mới truy cập được để giải nén gói. Nơi cần kiểm tra là { $failure }

## What came of an install, or of a preview

installer-result = Kết quả cài đặt
installer-result-preview = Xem trước: chưa có gì bị thay đổi
installer-result-installed = Ứng dụng đã được cài đặt và đã vượt qua kiểm tra hàm băm.
installer-result-installed-without-left-out = Ứng dụng đã được cài đặt và đã vượt qua kiểm tra hàm băm, ngoại trừ những mục được liệt kê là bị loại trừ bên dưới.
installer-result-not-all-put-back = Cài đặt thất bại và không phải mọi thay đổi đều có thể được khôi phục lại. Những gì còn lại được liệt kê bên dưới, và lần cài đặt tiếp theo sẽ thử lại.
installer-result-cancelled-put-back = Cài đặt đã bị hủy và những gì đã thay đổi đã được khôi phục lại.
installer-result-failed-put-back = Cài đặt thất bại và những gì đã thay đổi đã được khôi phục lại.
installer-result-not-clean = Quá trình cài đặt chưa hoàn tất trọn vẹn. Hãy xem lại các sự cố bên dưới.
installer-result-changes-put-back = Thay đổi đã được khôi phục lại
installer-result-changes-made = Thay đổi đã thực hiện
installer-result-changes-planned = Thay đổi dự kiến
installer-result-warnings = Cảnh báo
installer-result-errors = Lỗi
installer-result-package-valid = Hàm băm của gói hợp lệ
installer-result-package-invalid = Hàm băm của gói KHÔNG HỢP LỆ
installer-result-files-valid = Tệp đã cài đặt hợp lệ
installer-result-files-invalid = Tệp đã cài đặt KHÔNG HỢP LỆ
installer-result-reboot = Một phần phụ thuộc cần khởi động lại để hoàn tất. Hãy khởi động lại Windows, rồi mở ứng dụng.
# A heading over the list of what was changed. { $count } is how many.
installer-result-changes = Thay đổi ({ $count })
# After the first part of a long list. { $count } is how many more there are.
installer-result-changes-more = và { $count } mục khác.
# Over the path of the install's log file.
installer-log = Nhật ký

## The installer's command line. Shown in English: scripts read it. A { $count } is plain digits, with no separators.

installer-cli-stub = Phần khởi chạy (stub) của trình cài đặt WinAppMigrator — không chứa gói ứng dụng nào.
installer-cli-stub-how = Hãy tạo trình cài đặt bằng ứng dụng WinAppMigrator, rồi chạy tệp .exe được tạo ra trên máy đích.
installer-cli-usage-title = Trình cài đặt tự giải nén WinAppMigrator.
installer-cli-usage = Chạy không kèm đối số để mở trình cài đặt đồ họa, hoặc:
installer-cli-usage-preview = xem trước việc cài đặt lại: --dry-run [--target-root <DIR>]
installer-cli-usage-apply = cài đặt lại:                --apply [--target-root <DIR>]
installer-cli-no-package = trình cài đặt không chứa gói nhúng
installer-cli-no-programdata = không tìm thấy thư mục ProgramData của PC này, nên không có gì được cài đặt: mỗi lần cài đặt đều lưu tại đó bản ghi dùng để hoàn tác lần cài đặt ấy
# { $path } is the folder asked for, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-folder-has-files = không thể dùng { $path }: { $problem }. Chỉ dùng --force nếu bạn biết đó là thư mục của ứng dụng này
installer-cli-no-location = gói này không cho biết ứng dụng của nó từng nằm ở đâu, nên hãy chỉ định --target-root
# { $path } is where the app was on the other PC, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-location-refused = ứng dụng của gói này từng nằm trong { $path }, và không thể dùng vị trí đó: { $problem }. Hãy chỉ định --target-root
# { $summary } is how much room it needs where, such as "up to 824.0 MB on C: (356.0 GB free)".
installer-cli-room = Dung lượng: { $summary }
# { $note } is something to know about the install.
installer-cli-note = Lưu ý: { $note }
# { $count } is how many things it sets up beyond its files.
installer-cli-sets-up = Thiết lập: { $count }
installer-cli-applied = Đã áp dụng việc cài đặt lại.
installer-cli-previewed = Đã xem trước việc cài đặt lại.
# { $count } is how many changes the install logged.
installer-cli-changes-logged = Thay đổi đã ghi vào nhật ký: { $count }
# { $count } is how many warnings.
installer-cli-warnings = Cảnh báo: { $count }
# { $count } is how many errors.
installer-cli-errors = Lỗi: { $count }
# { $count } is how many things the package asked for and did not get.
installer-cli-refused = Bị từ chối: { $count }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-package-valid = Hàm băm của gói hợp lệ: { $valid }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-reinstall-valid = Hàm băm của bản cài đặt lại hợp lệ: { $valid }
# { $id } is the install's id, such as wam-0f3a….
installer-cli-install-id = ID cài đặt: { $id }
installer-cli-put-back = Cài đặt thất bại, nên những gì đã thay đổi đã được khôi phục lại.
# { $path } is the log file.
installer-cli-log = Nhật ký: { $path }
# { $path } is the log file. Printed before an error.
installer-cli-error-log = nhật ký: { $path }
# { $error } is what went wrong.
installer-cli-error = lỗi: { $error }

## The uninstaller's window

uninstaller-window-title = Gỡ cài đặt
# The window's title, and its heading. { $app_name } is the name of the app it uninstalls.
uninstaller-window-title-app = Gỡ cài đặt { $app_name }
uninstaller-tagline = Hoàn tác những gì trình cài đặt đã làm
uninstaller-reading = Đang đọc bản ghi của lần cài đặt này…
uninstaller-stopped-unexpectedly = Quá trình gỡ cài đặt đã dừng đột ngột.
uninstaller-what-will-be-removed = Những gì sẽ bị xóa
uninstaller-install-location = Vị trí cài đặt
uninstaller-files = Tệp
uninstaller-registry-values = Giá trị sổ đăng ký
uninstaller-services = Dịch vụ
uninstaller-drivers = Trình điều khiển
uninstaller-scheduled-tasks = Tác vụ đã lên lịch
uninstaller-environment = Môi trường
uninstaller-plan-stays = Các tệp bạn đã thêm hoặc thay đổi sẽ được giữ lại, cùng với các thư mục chứa chúng.
# { $count } is how many files the install replaced.
uninstaller-plan-coming-back =
    { $count ->
       *[other] { $count } tệp mà lần cài đặt này đã thay thế sẽ được khôi phục, hoặc được đặt bên cạnh tệp đã thay đổi kể từ đó.
    }
# { $runtimes } is a list of runtimes, such as "Microsoft Visual C++ Redistributable, .NET Desktop Runtime 8".
uninstaller-plan-runtimes = Các thư viện thời gian chạy mà lần cài đặt này đã cài đặt sẽ được giữ lại, vì các chương trình khác có thể dùng chúng: { $runtimes }.
uninstaller-options = Tùy chọn
uninstaller-remove-user-data = Đồng thời xóa dữ liệu người dùng và dữ liệu giấy phép
# { $count } is how many items of the app's settings, data, and license there are.
uninstaller-remove-user-data-detail =
    { $count ->
       *[other] { $count } mục thiết đặt, dữ liệu và giấy phép của ứng dụng. Các mục này được giữ lại trừ khi bạn bật tùy chọn này. Những gì chúng đã thay thế sẽ được khôi phục, hoặc được đặt bên cạnh chúng nếu chúng đã thay đổi kể từ đó.
    }
uninstaller-uninstall = Gỡ cài đặt
uninstaller-cancel = Hủy
uninstaller-close = Đóng
# { $done } and { $total } are how many of the install's items are undone, and how many there are.
uninstaller-step-counted = Đang hoàn tác lần cài đặt: { $done } trên { $total }…
uninstaller-step = Đang hoàn tác lần cài đặt…
uninstaller-done = Ứng dụng đã được gỡ cài đặt.
uninstaller-done-except = Ứng dụng đã được gỡ cài đặt, ngoại trừ những mục được liệt kê bên dưới.
uninstaller-waits-for-drive = Một phần của ứng dụng nằm trên ổ đĩa chưa được kết nối. Hãy kết nối ổ đĩa đó, rồi gỡ cài đặt lại từ Cài đặt.
uninstaller-not-all-undone = Không thể hoàn tác tất cả. Hãy đóng những gì đang sử dụng ứng dụng, rồi gỡ cài đặt lại từ Cài đặt.
# Over the path of the uninstall's log file.
uninstaller-log = Nhật ký
uninstaller-undone = Đã hoàn tác
uninstaller-left-in-place = Được giữ nguyên
uninstaller-could-not-be-undone = Không thể hoàn tác
# After the first part of a long list. { $count } is how many more there are.
uninstaller-and-more = và { $count } mục khác.
# Said as the reason when there is no record of the install to read.
uninstaller-no-programdata = không tìm thấy thư mục ProgramData của PC này, nên không có bản ghi nào để đọc

## When the uninstaller's window cannot open. Each is a paragraph of a message box, followed by a command to type.

uninstaller-window-failed-title = Trình gỡ cài đặt WinAppMigrator
# { $error } is what went wrong, as Windows or the graphics driver says it.
uninstaller-window-failed = Không thể mở cửa sổ của trình gỡ cài đặt: { $error }
uninstaller-window-failed-quiet = Trình gỡ cài đặt vẫn có thể gỡ cài đặt mà không cần cửa sổ, từ dấu nhắc lệnh với quyền quản trị viên:
# Stands in for this program's own path in that command, if it cannot be found.
uninstaller-this-program = chương trình này

## The uninstaller's command line (--quiet). Shown in English: scripts read it.

# { $app_name } is the name of the app.
uninstaller-cli-uninstalling = Đang gỡ cài đặt { $app_name }.
# { $path } is the log file.
uninstaller-cli-log = Nhật ký: { $path }
# { $error } is what went wrong.
uninstaller-cli-error = lỗi: { $error }
