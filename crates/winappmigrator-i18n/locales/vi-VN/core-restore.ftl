# Words the core writes for people as it installs an app from a package: why an install will not start, the list of
# what it did (or for a preview, what it would do) that the installer window shows, its warnings, the record it keeps of
# each install so that it can be undone, and the core's errors. Message names start with core-.
#
# The list of what an install did is short and plain, one line for each thing, as the installer window lists them:
# "copied", "restored", "would copy". Words such as --force and --apply are command-line options, and LocalService,
# SYSTEM, ProgramData, and %AppData% are as Windows names them: keep them as they are.


## Errors: what went wrong, when the core cannot go on. Each is shown on its own, or after words that say what was being
## done.

core-error-list-changed = những gì lần cài đặt này sẽ làm đã thay đổi kể từ khi được đồng ý, nên không có gì được cài đặt
# { $path } is a file or folder, such as C:\Program Files\App\app.exe, or words that name one, such as "the installer".
# { $error } is what Windows said went wrong.
core-error-io = thao tác I/O không thành công với { $path }: { $error }
# { $detail } is what went wrong.
core-error-registry = đọc sổ đăng ký không thành công: { $detail }
# { $path } is a program file. { $detail } is what was wrong with it.
core-error-pe = phân tích PE không thành công với { $path }: { $detail }
# { $detail } is what was wrong, already in words.
core-error-invalid-input = dữ liệu đầu vào không hợp lệ: { $detail }
# { $detail } is what went wrong.
core-error-serialization = tuần tự hóa báo cáo không thành công: { $detail }
# { $detail } is what went wrong.
core-error-archive = thao tác lưu trữ không thành công: { $detail }
# { $detail } is what went wrong.
core-error-network = thao tác mạng không thành công: { $detail }
# { $detail } is what went wrong.
core-error-time-format = định dạng thời gian không thành công: { $detail }
core-error-cancelled = thao tác đã bị hủy
# What is said when part of the program stopped unexpectedly and gave no reason.
core-error-panic-no-reason = không có lý do nào được đưa ra

## The record of each install, kept so that it can be undone.

# { $kind } is a number that says what type of registry value it is.
core-journal-unknown-value-type = loại giá trị sổ đăng ký không xác định { $kind }
core-journal-not-elevated = chưa được nâng quyền
core-journal-needs-admin = việc cài đặt cần quyền quản trị viên, để lưu bản ghi cài đặt ở nơi người khác không thể thay đổi
# { $path } is a folder.
core-journal-not-an-install-folder = đây không phải thư mục của một lần cài đặt: { $path }
# { $path } is the file the record is kept in.
core-journal-not-written = không thể ghi bản ghi của lần cài đặt này: { $path }
# { $path } is a folder.
core-journal-path-escapes-root = đường dẫn vượt ra ngoài thư mục gốc dự kiến: { $path }
# { $path } is a folder.
core-journal-nothing-exists = không mục nào trong số này tồn tại trên PC này: { $path }
core-journal-busy = một lần cài đặt hoặc gỡ cài đặt khác đang chạy. Hãy đợi cho đến khi hoàn tất, rồi thử lại
# Said of a file that was to get back the permissions it had.
core-journal-no-folder = tệp không có thư mục chứa
# Said of the permissions a file had, which were kept to give back to it.
core-journal-permissions-unreadable = không thể đọc những gì đã được lưu giữ
# { $folder } is the folder of one install's record.
core-journal-unreadable-empty = không thể dùng bản ghi của lần cài đặt trong { $folder }: bản ghi trống
# { $folder } is the folder of one install's record.
core-journal-unreadable-no-header = không thể dùng bản ghi của lần cài đặt trong { $folder }: bản ghi không bắt đầu bằng phần cho biết nó nói về điều gì
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-version = không thể dùng bản ghi của lần cài đặt trong { $folder }: một phiên bản khác của chương trình này đã ghi bản ghi đó
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-install = không thể dùng bản ghi của lần cài đặt trong { $folder }: bản ghi cho biết nó thuộc về một lần cài đặt khác
# { $id } is an install's id, such as wam-1a2b3c.
core-journal-not-an-install-id = đây không phải ID của một lần cài đặt: { $id }

## Why an install will not start, or stopped before it changed anything.

core-restore-apply-and-dry-run = không thể chỉ định cùng lúc cả --apply và --dry-run
# { $path } is the folder that was chosen to install in.
core-restore-location-not-full-path = vị trí cài đặt phải là đường dẫn đầy đủ: “{ $path }”
# { $problem } says what is wrong with the folder, already in words. { $path } is the folder.
core-restore-location-unusable = không thể dùng vị trí cài đặt đó: { $problem }: “{ $path }”
core-restore-needs-admin-plan = kế hoạch khôi phục bao gồm các thay đổi chỉ quản trị viên mới được thực hiện đối với sổ đăng ký, dịch vụ, tác vụ đã lên lịch, lối tắt hoặc đích được bảo vệ
# { $path } is the folder that was chosen to install in.
core-restore-needs-admin-target = thư mục gốc đích cần nâng quyền: { $path }
core-restore-needs-admin-user-data = việc khôi phục dữ liệu người dùng vào ProgramData hoặc các vị trí được bảo vệ khác cần nâng quyền
# { $error } is why the install stopped. { $failed } is what could not be put back, as sentences of their own.
core-restore-not-all-put-back = { $error }. Không thể khôi phục lại tất cả những gì đã làm: { $failed }
# { $error } is what went wrong.
core-restore-nowhere-to-keep-record = không có nơi an toàn để lưu bản ghi cho phép hoàn tác lần cài đặt này, nên không có gì được cài đặt: { $error }
# { $path } is a file the app would have put there.
core-restore-already-exists = { $path } đã tồn tại, nên không có gì được cài đặt. Hãy dùng --force để thay thế
# { $root } is a folder the app has files for. { $link } is that folder, or one it is in.
core-restore-root-under-link = ứng dụng này có nội dung cần khôi phục vào { $root }, và { $link } là liên kết tới nơi khác mà người khác có thể thay đổi, nên không có gì được cài đặt. Nếu nội dung đó là dữ liệu người dùng hoặc dữ liệu giấy phép, hãy cài đặt mà không kèm dữ liệu người dùng và dữ liệu giấy phép. Nếu đó là lối tắt trong thư mục Khởi động, hãy đưa thư mục đó về vị trí cũ
core-restore-hash-check-failed = gói không vượt qua kiểm tra hàm băm và không có gì được cài đặt
# { $detail } is the first thing found wrong, already in words.
core-restore-hash-check-failed-because = gói không vượt qua kiểm tra hàm băm và không có gì được cài đặt: { $detail }
# { $path } is a file or folder.
core-restore-refuse-records-folder = từ chối ghi vào nơi các lần cài đặt lưu bản ghi: { $path }
# { $path } is a file or folder.
core-restore-refuse-outside = từ chối ghi ra ngoài vị trí cài đặt và các thư mục dữ liệu đã biết: { $path }
# { $path } is a file.
core-restore-no-parent-folder = đích không có thư mục cha: { $path }
# { $path } is where a file was to go.
core-restore-folder-in-the-way = có một thư mục nằm ở vị trí mà tệp này cần được đặt: { $path }
# { $path } is a file in the package.
core-restore-file-changed = { $path } không còn giống với nội dung trong gói, nên không được cài đặt
# { $path } is a file in the package.
core-restore-source-is-link = { $path } là liên kết và không được đi theo
# { $path } is a file in the package.
core-restore-source-became-link = { $path } đã trở thành liên kết trong khi đang được mở
# Why a service did not start, after "did not start:".
core-restore-service-stopped-by-itself = dịch vụ đã tự dừng lại
# Why part of the app was left out, after "was left out:".
core-restore-com-class-someone-elses = lớp COM này đã có trên PC này và thuộc về một thành phần khác

## What an install did, one line for each thing, or for a preview, what it would do. { $source } is a file in the
## package, and { $destination } and { $path } are where it goes on this PC. { $key } is a registry key, such as
## HKLM\Software\App.

# { $package_valid } is true or false, and { $restore_valid } is true, false, or not_run: keep them as they are.
core-restore-verification-result = xác minh sau khôi phục package_valid={ $package_valid } restore_valid={ $restore_valid }
# { $name } is a service's name.
core-restore-started-service = đã khởi động dịch vụ { $name }
core-restore-would-copy = sẽ sao chép { $source } -> { $destination }
core-restore-copied = đã sao chép { $source } -> { $destination }
core-restore-would-create-empty-folder = sẽ tạo thư mục trống { $path }
core-restore-created-empty-folder = đã tạo thư mục trống { $path }
# { $value } is the name of a value in the key.
core-restore-would-restore-registry-value = sẽ khôi phục sổ đăng ký { $key } { $value }
core-restore-would-restore-registry-key = sẽ khôi phục sổ đăng ký { $key } (khóa)
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-would-add-firewall-rule = sẽ thêm quy tắc tường lửa { $name } cho { $program }
# { $name } is a firewall rule's name.
core-restore-firewall-rule-already-there = quy tắc tường lửa { $name } đã có sẵn
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-added-firewall-rule = đã thêm quy tắc tường lửa { $name } cho { $program }
# { $name } is a service's name, { $command } what it runs, and { $account } the account it runs as, such as LocalService.
core-restore-would-restore-service = sẽ khôi phục dịch vụ { $name } -> { $command } (chạy dưới tài khoản { $account })
# { $name } is a runtime, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-restore-prerequisite-present = thành phần tiên quyết đã có sẵn: { $name }
# { $name } is a runtime that Windows has as a Windows feature.
core-restore-would-install-prerequisite-feature = sẽ cài đặt thành phần tiên quyết { $name } (tính năng Windows)
# { $name } is a runtime whose installer is in the package.
core-restore-would-install-prerequisite-bundled = sẽ cài đặt thành phần tiên quyết { $name } (trình cài đặt đi kèm)
# { $name } is a runtime that would come through winget, which is a program's name.
core-restore-would-install-prerequisite-winget = sẽ cài đặt thành phần tiên quyết { $name } (winget)
# { $name } is a runtime.
core-restore-installed-prerequisite = đã cài đặt thành phần tiên quyết { $name }
# { $name } is a runtime.
core-restore-installed-prerequisite-reboot = đã cài đặt thành phần tiên quyết { $name } (cần khởi động lại)
core-restore-would-restore-license = sẽ khôi phục giấy phép { $source } -> { $destination }
core-restore-restored-license = đã khôi phục giấy phép { $source } -> { $destination }
core-restore-would-restore-license-registry = sẽ khôi phục sổ đăng ký giấy phép { $key }
# { $name } is a driver's name.
core-restore-driver-not-restored = trình điều khiển không được tự động khôi phục: { $name }
# { $name } is a driver's name.
core-restore-would-install-driver = sẽ cài đặt trình điều khiển { $name } -> { $destination }
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-would-restore-task = sẽ khôi phục tác vụ đã lên lịch { $name }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-startup-shortcut = sẽ khôi phục lối tắt khởi động { $shortcut } -> { $target }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-shortcut = sẽ khôi phục lối tắt { $shortcut } -> { $target }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-would-restore-variable =
    { $scope ->
        [user] sẽ khôi phục biến môi trường người dùng { $name }
       *[machine] sẽ khôi phục biến môi trường hệ thống { $name }
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-left-variable =
    { $scope ->
        [user] đã giữ nguyên biến môi trường người dùng { $name }: không phần nào của biến này thuộc về ứng dụng này
       *[machine] đã giữ nguyên biến môi trường hệ thống { $name }: không phần nào của biến này thuộc về ứng dụng này
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-restored-variable =
    { $scope ->
        [user] đã khôi phục biến môi trường người dùng { $name }
       *[machine] đã khôi phục biến môi trường hệ thống { $name }
    }
core-restore-set-uninstaller = đã đặt trình gỡ cài đặt cho { $key }
core-restore-placed-driver-image = đã đặt tệp trình điều khiển { $source } -> { $destination }
# { $name } is a driver's name.
core-restore-configured-driver = đã cấu hình trình điều khiển { $name }
# { $name } is a driver's name.
core-restore-created-driver = đã tạo trình điều khiển { $name }
# { $name } is a driver's name.
core-restore-started-driver = đã khởi động trình điều khiển { $name }
core-restore-ensured-registry-key = đã đảm bảo có khóa sổ đăng ký { $key }
# { $value } is the name of a value in the key.
core-restore-restored-registry-value = đã khôi phục sổ đăng ký { $key } { $value }
# "(Default)" is how Windows names the value a key has with no name: write it as Registry Editor does.
core-restore-restored-registry-default = đã khôi phục sổ đăng ký { $key } (Mặc định)
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-configured-service = đã cấu hình dịch vụ { $name } (chạy dưới tài khoản { $account })
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-created-service = đã tạo dịch vụ { $name } (chạy dưới tài khoản { $account })
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-restored-task = đã khôi phục tác vụ đã lên lịch { $name }
core-restore-restored-shortcut = đã khôi phục lối tắt { $path }

## Warnings an install gives, and why it failed. They are shown after it, on the PC it ran on.

core-warning-restore-earlier-not-trusted = Một bản ghi của lần cài đặt chưa hoàn tất đã được giữ nguyên, vì người khác có thể thay đổi bản ghi đó. Mọi thay đổi mà lần cài đặt đó đã thực hiện vẫn còn nguyên.
# { $app_name } is the app's name. { $count } is how many changes were put back.
core-warning-restore-earlier-undone =
    { $count ->
       *[other] Một lần cài đặt { $app_name } trước đây chưa hoàn tất, nên đã được hoàn tác trước. Đã khôi phục lại { $count } thay đổi.
    }
# { $app_name } is the app's name.
core-warning-restore-earlier-never-finished = Một lần cài đặt { $app_name } trước đây chưa hoàn tất. Việc cài đặt sẽ hoàn tác lần đó trước.
core-warning-restore-user-data-real-locations = Dữ liệu người dùng được khôi phục vào các vị trí %AppData%/%ProgramData% thực của máy này thay vì thư mục gốc đích.
# { $error } is what went wrong.
core-warning-restore-commit-failed = Không thể hoàn tất bản ghi của lần cài đặt này: { $error }
# { $name } is a service's name.
core-warning-restore-service-still-starting = Dịch vụ { $name } đã được yêu cầu khởi động nhưng chưa khởi động xong. Dịch vụ vẫn có thể khởi động sau đó. Dịch vụ cũng tự khởi động vào lần tới PC này khởi động.
# { $name } is a service's name. { $reason } is why it did not start, as Windows said it.
core-warning-restore-service-not-started = Dịch vụ { $name } đã được cài đặt nhưng không khởi động: { $reason }. Ứng dụng có thể không hoạt động cho đến khi dịch vụ khởi động. Windows sẽ thử lại vào lần tới PC này khởi động.
# { $name } is a service's name. { $reason } is why it could not be stopped.
core-warning-restore-service-not-stopped = Không thể dừng dịch vụ { $name } trước: { $reason }. Dịch vụ có thể tiếp tục chạy chương trình cũ cho đến lần tới PC này khởi động.
core-warning-restore-install-cancelled = Quá trình cài đặt đã bị hủy.
core-warning-restore-preview-cancelled = Quá trình xem trước đã bị hủy.
# { $item } is what was left out, such as a file or a service's name. { $reason } is why, already in words.
core-warning-restore-left-out = { $item } đã bị loại trừ: { $reason }.
# { $error } is what went wrong.
core-warning-restore-verification-failed = Không thể chạy xác minh sau khôi phục: { $error }
# { $runner } says what relies on the file: a service, a scheduled task, a firewall rule, a shortcut, or a registry value.
# { $path } is the file. { $name } is the name of what relies on it. { $refusal } is why the file was not replaced,
# already in words.
core-warning-restore-program-would-run =
    { $runner ->
        [service] { $path } không phải tệp mà gói này được phép thay thế, và dịch vụ { $name } phụ thuộc vào tệp đó, nên không có gì được cài đặt: { $refusal }
        [task] { $path } không phải tệp mà gói này được phép thay thế, và tác vụ đã lên lịch { $name } phụ thuộc vào tệp đó, nên không có gì được cài đặt: { $refusal }
        [firewall] { $path } không phải tệp mà gói này được phép thay thế, và quy tắc tường lửa { $name } phụ thuộc vào tệp đó, nên không có gì được cài đặt: { $refusal }
        [shortcut] { $path } không phải tệp mà gói này được phép thay thế, và lối tắt { $name } phụ thuộc vào tệp đó, nên không có gì được cài đặt: { $refusal }
       *[registry] { $path } không phải tệp mà gói này được phép thay thế, và giá trị sổ đăng ký { $name } phụ thuộc vào tệp đó, nên không có gì được cài đặt: { $refusal }
    }
core-warning-restore-destination-exists = Đích đã tồn tại; hãy dùng --force để ghi đè.
# { $path } is a file. { $error } is what went wrong.
core-warning-restore-file-not-written = Không thể ghi { $path }: { $error }
core-warning-restore-not-a-folder = Có một mục không phải thư mục nằm ở vị trí của thư mục trống này.
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-not-looked-at = Không thể kiểm tra { $path }: { $error }
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-folder-not-made = Không thể tạo { $path }: { $error }
# Why part of the app was left out, after "was left out:".
core-warning-restore-registration-exists = PC này đã có mục này với thiết đặt khác, và việc cài đặt không tiếp quản mục đó
core-warning-restore-registry-review = Các giá trị sổ đăng ký đã được khôi phục từ dữ liệu có cấu trúc đã lưu trữ, trong đó các đường dẫn ứng dụng đã biết được viết lại theo thư mục gốc đích.
# { $error } is what went wrong.
core-warning-restore-firewall-unreadable = Không thể đọc các quy tắc của Tường lửa Windows: { $error }
# Why a firewall rule was left out, after "was left out:".
core-warning-restore-firewall-rule-exists = đã có một quy tắc cùng tên cho chương trình này với thiết đặt khác, và việc cài đặt không thay đổi quy tắc đó
# { $name } is a runtime.
core-warning-restore-prerequisite-unknown = { $name }: không có trình cài đặt nào đã biết cho thư viện thời gian chạy này.
# { $name } is a runtime.
core-warning-restore-prerequisite-needs-admin = Việc cài đặt { $name } cần nâng quyền; hãy chạy lại trình cài đặt với tư cách quản trị viên.
# { $name } is a runtime.
core-warning-restore-prerequisite-reboot = { $name } đã được cài đặt nhưng cần khởi động lại để hoàn tất.
# { $error } is what went wrong.
core-warning-restore-skipped-not-recorded = Không thể ghi lại một thư viện thời gian chạy đã bị bỏ qua: { $error }
# { $name } is a runtime. { $reason } is why it was skipped, already in words.
core-warning-restore-prerequisite-skipped = { $name }: { $reason }
# { $name } is a runtime. { $reason } is why it could not be installed. { $url } is a web address to get it from.
core-warning-restore-prerequisite-failed = Không thể cài đặt { $name }: { $reason }. Hãy tải về từ { $url }.
# { $error } is what went wrong.
core-warning-restore-runtime-not-recorded = Không thể ghi lại một thư viện thời gian chạy đã cài đặt: { $error }
# { $error } is what went wrong.
core-warning-restore-license-source-invalid = Đường dẫn giấy phép trong gói không hợp lệ: { $error }
core-warning-restore-license-needs-admin = Việc khôi phục dữ liệu giấy phép vào vị trí được bảo vệ cần nâng quyền.
core-warning-restore-license-exists = Đích của giấy phép đã tồn tại; hãy dùng --force để ghi đè.
# { $error } is what went wrong.
core-warning-restore-license-failed = Không thể khôi phục dữ liệu giấy phép: { $error }
core-warning-restore-driver-needs-admin = Việc đăng ký trình điều khiển cần nâng quyền; hãy chạy lại trình cài đặt với tư cách quản trị viên.
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-warning-restore-task-runs-as-system = Tác vụ đã lên lịch { $name } chạy dưới tài khoản SYSTEM, với toàn quyền trên PC này.
core-warning-restore-shortcut-exists = Lối tắt đã tồn tại; hãy dùng --force để ghi đè.
core-warning-restore-shortcut-all-users = Một hoặc nhiều lối tắt đã khôi phục trỏ tới màn hình nền công cộng hoặc vị trí menu Bắt đầu dùng chung.
core-warning-restore-environment-refresh = Thay đổi biến môi trường có thể chỉ có hiệu lực trong tiến trình, shell hoặc phiên đăng nhập mới.
# { $name } is a service's name. { $account } is the account it ran as on the PC it came from.
core-warning-restore-service-account = Dịch vụ { $name } đã chạy dưới tài khoản { $account } trên máy nguồn. Mật khẩu của tài khoản đó không có trong trình cài đặt, nên dịch vụ được thiết lập để chạy dưới LocalService. Nếu dịch vụ không hoạt động, hãy đặt tài khoản và mật khẩu cho dịch vụ trong Dịch vụ.
# { $name } is a runtime.
core-warning-restore-prerequisite-pending-reboot = { $name } chưa được đăng ký; thành phần này sẽ hoạt động sau khi khởi động lại.
# { $name } is a runtime.
core-warning-restore-prerequisite-not-verified = { $name } báo cáo thành công nhưng chưa được đăng ký; quá trình cài đặt có thể đã thất bại.
# { $error } is what went wrong.
core-warning-restore-driver-source-invalid = Đường dẫn trình điều khiển đi kèm không hợp lệ: { $error }
# { $error } is what went wrong.
core-warning-restore-driver-not-read = Không thể đọc thiết đặt hiện tại của trình điều khiển: { $error }
core-warning-restore-driver-exists = Dịch vụ trình điều khiển đã tồn tại; hãy dùng --force để cấu hình lại.
core-warning-restore-driver-image-exists = Tệp trình điều khiển đã tồn tại; hãy dùng --force để ghi đè.
core-warning-restore-driver-hash-bundled = Tệp trình điều khiển đi kèm không khớp với hàm băm đã ghi, nên không được cài đặt.
# { $error } is what went wrong.
core-warning-restore-driver-not-placed = Không thể đặt tệp trình điều khiển: { $error }
core-warning-restore-driver-hash-placed = Tệp trình điều khiển đã thay đổi sau khi được đặt vào, nên không được đăng ký.
# { $error } is what went wrong.
core-warning-restore-key-not-created = Không thể tạo khóa sổ đăng ký: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-read = Không thể đọc giá trị mà thao tác này sẽ thay thế: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-written = Không thể ghi giá trị sổ đăng ký: { $error }
core-warning-restore-value-exists = Giá trị sổ đăng ký đã tồn tại; hãy dùng --force để ghi đè.
# { $error } is what went wrong.
core-warning-restore-environment-key-not-created = Không thể tạo khóa sổ đăng ký môi trường: { $error }
core-warning-restore-variable-exists = Biến môi trường đã tồn tại; hãy dùng --force để ghi đè.
# { $error } is what went wrong.
core-warning-restore-variable-not-written = Không thể ghi biến môi trường: { $error }
core-warning-restore-value-kind-missing = Thiếu loại của giá trị sổ đăng ký.
# { $error } is what went wrong.
core-warning-restore-service-not-read = Không thể đọc thiết đặt hiện tại của dịch vụ: { $error }
core-warning-restore-service-exists = Dịch vụ đã tồn tại; hãy dùng --force để cấu hình lại.
# { $error } is what went wrong.
core-warning-restore-task-not-queried = Không thể truy vấn tác vụ đã lên lịch: { $error }
core-warning-restore-task-exists = Tác vụ đã lên lịch đã tồn tại; hãy dùng --force để ghi đè.
# { $error } is what went wrong.
core-warning-restore-task-folder-not-looked-at = Không thể kiểm tra thư mục của tác vụ: { $error }
# { $error } is what went wrong.
core-warning-restore-shortcut-not-put-back = Không thể tạo lối tắt, và không thể khôi phục lại lối tắt đã bị thay thế: { $error }
# { $error } is what went wrong.
core-warning-restore-command-not-started = Không thể khởi động lệnh: { $error }
# { $output } is what the command wrote, as it wrote it.
core-warning-restore-command-failed = Lệnh không thành công: { $output }
# { $error } is what went wrong.
core-warning-restore-powershell-not-started = Không thể khởi động PowerShell: { $error }
