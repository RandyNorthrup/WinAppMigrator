# Words the core writes for people: warnings, refusals, errors, and the installer's list of what it sets up.
# Message names start with core-.


## Warnings a package records when it is built, which an installer shows later, maybe on another PC, in that PC's
## language.

# { $count } is how many shortcuts.
core-warning-shortcuts-unreadable =
    { $count ->
       *[other] { $count } lối tắt trên ổ đĩa này không trỏ tới tệp nào, nên không xác định được liệu chúng có mở ứng dụng này hay không.
    }

## Undoing an install: what an uninstall, or a failed install, says it put back, left, or could not undo. Each line is
## shown on its own, in a list.

# { $app_name } is the app's name.
core-rollback-record-damaged = Không thể đọc một phần bản ghi của { $app_name }, nên mọi thứ mà phần đó mô tả vẫn còn nguyên, và tệp đã bị thay thế có thể không được khôi phục.
core-rollback-settings-entry-stays = Mục của ứng dụng trong Cài đặt vẫn được giữ lại, để có thể chạy lại thao tác này từ đó.
# { $name } is a runtime's name, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-rollback-prerequisite-stays = { $name } đã được cài đặt, hoặc có thể đã được cài đặt, và được giữ lại vì các chương trình khác có thể đang dùng thành phần này.
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-drive-not-connected = Những gì lần cài đặt này đã đặt vào { $folder } nằm trên ổ đĩa chưa được kết nối. Hãy kết nối ổ đĩa đó để có thể hoàn tất việc này.
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-record-part-unread = Những gì lần cài đặt này đã đặt vào { $folder } vẫn được giữ lại, vì không thể đọc phần đó của bản ghi.

## Why a file or folder was left, shown after "was left:" in the messages below them. Each is part of a sentence, so
## starts with a small letter and has no full stop.

# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-being-changed = có thứ gì đó đã thay đổi { $folder } trong khi thao tác này chạy, nên không có gì thêm được thực hiện trong thư mục đó. Hãy đóng thứ đang làm việc đó, rồi chạy lại thao tác này
# { $folder } is a folder; { $error } is what Windows said went wrong.
core-rollback-reason-not-opened = không thể mở { $folder }: { $error }
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-no-record = không có bản ghi về vị trí trước đây của { $folder }
# { $folder } is a folder; { $actual } is the folder it now leads to.
core-rollback-reason-moved = { $folder } hiện trỏ tới { $actual }, không phải vị trí của nó trong lúc cài đặt
# Why a service was not stopped.
core-rollback-reason-did-not-stop = dịch vụ không dừng kịp thời

## Files and folders

# { $path } is a folder; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-folder-left = Thư mục { $path } được giữ lại: { $reason }.
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-removed = Đã xóa thư mục { $path }
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-holds-others = Thư mục { $path } được giữ lại, vì thư mục này chứa các tệp không do lần cài đặt này đặt vào.
# { $path } is a folder; { $error } is what Windows said went wrong.
core-rollback-folder-not-removed = Không thể xóa thư mục { $path }: { $error }.
# { $path } is a file; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-file-left = Tệp { $path } được giữ lại: { $reason }.
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-folder-gone = { $path } sẽ không được khôi phục, vì thư mục chứa tệp này đã không còn.
# { $path } is a file; { $error } is what went wrong.
core-rollback-partial-not-removed = Không thể xóa một tệp chưa hoàn chỉnh: { $path } ({ $error }).
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-checked = Không thể kiểm tra { $path }: { $error }.
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-removed = Đã xóa { $path }
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-removed = Không thể xóa { $path }: { $error }.
# { $path } is a file this install had replaced, and has put back.
core-rollback-file-put-back = Đã khôi phục { $path }
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-put-back = Không thể khôi phục { $path }: { $error }.
# Why a file could not be put back.
core-rollback-backup-gone = bản sao lưu đã không còn
# { $path } is a file.
core-rollback-no-parent-folder = { $path } không có thư mục cha

## A file that stays. { $why } says why: still-used (something that stays still uses it), changed (it has changed since
## it was installed), or not-asked (it is someone's data, and they did not ask for it to go). { $path } is the file.

core-rollback-file-stays =
    { $why ->
        [still-used] { $path } được giữ lại, vì một thành phần được giữ lại vẫn đang dùng tệp này.
        [changed] { $path } được giữ lại, vì tệp đã thay đổi kể từ khi được cài đặt.
       *[not-asked] { $path } được giữ lại, vì không có yêu cầu xóa tệp này.
    }
# The file it replaced was to be put beside it, under another name, and every name was taken.
core-rollback-file-stays-no-name-beside =
    { $why ->
        [still-used] { $path } được giữ lại, vì một thành phần được giữ lại vẫn đang dùng tệp này, và không thể đặt tệp mà nó đã thay thế bên cạnh: mọi tên cho tệp đó đều đã được dùng.
        [changed] { $path } được giữ lại, vì tệp đã thay đổi kể từ khi được cài đặt, và không thể đặt tệp mà nó đã thay thế bên cạnh: mọi tên cho tệp đó đều đã được dùng.
       *[not-asked] { $path } được giữ lại, vì không có yêu cầu xóa tệp này, và không thể đặt tệp mà nó đã thay thế bên cạnh: mọi tên cho tệp đó đều đã được dùng.
    }
# { $name } is the name the file it replaced now has, such as settings.ini.before-install.
core-rollback-file-stays-beside =
    { $why ->
        [still-used] { $path } được giữ lại, vì một thành phần được giữ lại vẫn đang dùng tệp này. Tệp mà nó đã thay thế nằm bên cạnh, với tên { $name }.
        [changed] { $path } được giữ lại, vì tệp đã thay đổi kể từ khi được cài đặt. Tệp mà nó đã thay thế nằm bên cạnh, với tên { $name }.
       *[not-asked] { $path } được giữ lại, vì không có yêu cầu xóa tệp này. Tệp mà nó đã thay thế nằm bên cạnh, với tên { $name }.
    }
# { $error } is what went wrong.
core-rollback-file-stays-not-beside =
    { $why ->
        [still-used] { $path } được giữ lại, vì một thành phần được giữ lại vẫn đang dùng tệp này, và không thể đặt tệp mà nó đã thay thế bên cạnh: { $error }.
        [changed] { $path } được giữ lại, vì tệp đã thay đổi kể từ khi được cài đặt, và không thể đặt tệp mà nó đã thay thế bên cạnh: { $error }.
       *[not-asked] { $path } được giữ lại, vì không có yêu cầu xóa tệp này, và không thể đặt tệp mà nó đã thay thế bên cạnh: { $error }.
    }

## The registry and environment variables. { $kind } says what { $name } is: key (a registry key, such as
## HKLM\Software\App), value (a registry value, such as HKLM\Software\App\Version), user-variable or machine-variable
## (an environment variable of the user's or of the whole PC, such as APP_HOME), or user-list or machine-list (a
## variable that is a list of folders, such as Path). { $error } is what Windows said went wrong.

# How a registry key's unnamed value is shown, as Registry Editor shows it.
core-registry-default-value-name = (Mặc định)
core-rollback-setting-other-account =
    { $kind ->
        [key] Khóa { $name } được giữ lại, vì khóa này thuộc về tài khoản đã thực hiện cài đặt, và tài khoản đó hiện không đăng nhập.
        [user-variable] Biến môi trường người dùng { $name } được giữ lại, vì biến này thuộc về tài khoản đã thực hiện cài đặt, và tài khoản đó hiện không đăng nhập.
        [machine-variable] Biến môi trường hệ thống { $name } được giữ lại, vì biến này thuộc về tài khoản đã thực hiện cài đặt, và tài khoản đó hiện không đăng nhập.
        [user-list] { $name } của người dùng được giữ lại, vì biến này thuộc về tài khoản đã thực hiện cài đặt, và tài khoản đó hiện không đăng nhập.
        [machine-list] { $name } của hệ thống được giữ lại, vì biến này thuộc về tài khoản đã thực hiện cài đặt, và tài khoản đó hiện không đăng nhập.
       *[value] Giá trị sổ đăng ký { $name } được giữ lại, vì giá trị này thuộc về tài khoản đã thực hiện cài đặt, và tài khoản đó hiện không đăng nhập.
    }
core-rollback-setting-not-reached =
    { $kind ->
        [key] Không thể truy cập khóa { $name }: { $error }.
        [user-variable] Không thể truy cập biến môi trường người dùng { $name }: { $error }.
        [machine-variable] Không thể truy cập biến môi trường hệ thống { $name }: { $error }.
        [user-list] Không thể truy cập { $name } của người dùng: { $error }.
        [machine-list] Không thể truy cập { $name } của hệ thống: { $error }.
       *[value] Không thể truy cập giá trị sổ đăng ký { $name }: { $error }.
    }
core-rollback-setting-not-read =
    { $kind ->
        [key] Không thể đọc khóa { $name }: { $error }.
        [user-variable] Không thể đọc biến môi trường người dùng { $name }: { $error }.
        [machine-variable] Không thể đọc biến môi trường hệ thống { $name }: { $error }.
        [user-list] Không thể đọc { $name } của người dùng: { $error }.
        [machine-list] Không thể đọc { $name } của hệ thống: { $error }.
       *[value] Không thể đọc giá trị sổ đăng ký { $name }: { $error }.
    }
core-rollback-setting-changed =
    { $kind ->
        [user-variable] Biến môi trường người dùng { $name } được giữ lại, vì biến này đã thay đổi kể từ khi được cài đặt.
        [machine-variable] Biến môi trường hệ thống { $name } được giữ lại, vì biến này đã thay đổi kể từ khi được cài đặt.
        [user-list] { $name } của người dùng được giữ lại, vì biến này đã thay đổi kể từ khi được cài đặt.
        [machine-list] { $name } của hệ thống được giữ lại, vì biến này đã thay đổi kể từ khi được cài đặt.
       *[value] Giá trị sổ đăng ký { $name } được giữ lại, vì giá trị này đã thay đổi kể từ khi được cài đặt.
    }
# Said when it is back as it was before the install.
core-rollback-setting-put-back =
    { $kind ->
        [user-variable] Đã khôi phục biến môi trường người dùng { $name }
        [machine-variable] Đã khôi phục biến môi trường hệ thống { $name }
        [user-list] Đã khôi phục { $name } của người dùng
        [machine-list] Đã khôi phục { $name } của hệ thống
       *[value] Đã khôi phục giá trị sổ đăng ký { $name }
    }
core-rollback-setting-not-put-back =
    { $kind ->
        [user-variable] Không thể khôi phục biến môi trường người dùng { $name }: { $error }.
        [machine-variable] Không thể khôi phục biến môi trường hệ thống { $name }: { $error }.
        [user-list] Không thể khôi phục { $name } của người dùng: { $error }.
        [machine-list] Không thể khôi phục { $name } của hệ thống: { $error }.
       *[value] Không thể khôi phục giá trị sổ đăng ký { $name }: { $error }.
    }
# A registry value the install wrote, which is the user's data, and stays unless they asked for it to go.
core-rollback-value-kept-by-request = Giá trị sổ đăng ký { $name } được giữ lại, vì không có yêu cầu xóa giá trị này.
core-rollback-key-holds-others = Khóa { $name } được giữ lại, vì khóa này chứa những gì không do lần cài đặt này đặt vào.
core-rollback-key-removed = Đã xóa khóa { $name }
core-rollback-key-not-removed = Không thể xóa khóa { $name }: { $error }.
# { $kind } is user-list or machine-list.
core-rollback-list-entries-removed =
    { $kind ->
        [user-list] Đã gỡ các mục của lần cài đặt này khỏi { $name } của người dùng
       *[machine-list] Đã gỡ các mục của lần cài đặt này khỏi { $name } của hệ thống
    }
core-rollback-list-entries-not-removed =
    { $kind ->
        [user-list] Không thể gỡ các mục của lần cài đặt này khỏi { $name } của người dùng: { $error }.
       *[machine-list] Không thể gỡ các mục của lần cài đặt này khỏi { $name } của hệ thống: { $error }.
    }

## Services and drivers. { $kind } is service or driver; { $name } is its name, such as AppUpdater. { $error } and
## { $reason } are what Windows said went wrong.

core-rollback-service-not-read =
    { $kind ->
        [driver] Không thể đọc trình điều khiển { $name }: { $error }.
       *[service] Không thể đọc dịch vụ { $name }: { $error }.
    }
core-rollback-service-changed =
    { $kind ->
        [driver] Trình điều khiển { $name } được giữ lại, vì trình điều khiển này đã thay đổi kể từ khi được cài đặt.
       *[service] Dịch vụ { $name } được giữ lại, vì dịch vụ này đã thay đổi kể từ khi được cài đặt.
    }
core-rollback-service-not-stopped =
    { $kind ->
        [driver] Không thể dừng trình điều khiển { $name } ({ $reason }), nên những gì nó đang dùng có thể còn lại cho đến khi PC khởi động lại.
       *[service] Không thể dừng dịch vụ { $name } ({ $reason }), nên những gì nó đang dùng có thể còn lại cho đến khi PC khởi động lại.
    }
core-rollback-service-stopped-for-this =
    { $kind ->
        [driver] Trình điều khiển { $name } đã được dừng để thực hiện việc này. Hãy khởi động lại khi bạn cần.
       *[service] Dịch vụ { $name } đã được dừng để thực hiện việc này. Hãy khởi động lại khi bạn cần.
    }
core-rollback-service-not-removed =
    { $kind ->
        [driver] Không thể xóa trình điều khiển { $name }: { $reason }.
       *[service] Không thể xóa dịch vụ { $name }: { $reason }.
    }
core-rollback-service-marked-for-removal =
    { $kind ->
        [driver] Trình điều khiển { $name } đã được đánh dấu để xóa, và sẽ bị xóa khi PC khởi động lại hoặc khi thứ đang mở nó giải phóng nó.
       *[service] Dịch vụ { $name } đã được đánh dấu để xóa, và sẽ bị xóa khi PC khởi động lại hoặc khi thứ đang mở nó giải phóng nó.
    }
core-rollback-service-removed =
    { $kind ->
        [driver] Đã xóa trình điều khiển { $name }
       *[service] Đã xóa dịch vụ { $name }
    }
core-rollback-service-not-restored =
    { $kind ->
        [driver] Không thể khôi phục trình điều khiển { $name } về như cũ: { $reason }.
       *[service] Không thể khôi phục dịch vụ { $name } về như cũ: { $reason }.
    }
core-rollback-service-restored =
    { $kind ->
        [driver] Đã khôi phục thiết đặt của trình điều khiển { $name }
       *[service] Đã khôi phục thiết đặt của dịch vụ { $name }
    }
core-rollback-service-not-restarted = Dịch vụ { $name } đang chạy trước lần cài đặt này, và đã không khởi động lại: { $reason }.
# { $account } is the account the service ran as, such as CONTOSO\backup.
core-rollback-service-account-again = Dịch vụ { $name } đã chạy dưới tài khoản { $account }. Mật khẩu của tài khoản không được lưu, nên hãy đặt lại tài khoản cho dịch vụ trong Dịch vụ.

## Firewall rules. { $name } is the rule's name; { $error } is what went wrong.

core-rollback-firewall-not-read = Không thể đọc quy tắc tường lửa { $name }: { $error }.
core-rollback-firewall-says-otherwise = Quy tắc tường lửa { $name } được giữ lại, vì quy tắc hiện có mang tên đó có nội dung khác.
core-rollback-firewall-changed = Quy tắc tường lửa { $name } được giữ lại, vì quy tắc này đã thay đổi kể từ khi được cài đặt.
# The capital "The" is as the app has always said it.
core-rollback-firewall-removed = Đã xóa quy tắc tường lửa { $name }
core-rollback-firewall-not-removed = Không thể xóa quy tắc tường lửa { $name }: { $error }.

## Scheduled tasks. { $path } is a folder of tasks, and { $name } a task with its folder, such as \App\Nightly Update.
## { $error } and { $reason } are what went wrong.

core-rollback-task-folder-holds-others = Thư mục tác vụ { $path } được giữ lại, vì thư mục này chứa các tác vụ không do lần cài đặt này đặt vào.
core-rollback-task-folder-not-removed = Không thể xóa thư mục tác vụ { $path }: { $error }.
core-rollback-task-not-read = Không thể đọc tác vụ { $name }: { $error }.
core-rollback-task-changed = Tác vụ { $name } được giữ lại, vì tác vụ này đã thay đổi kể từ khi được cài đặt.
core-rollback-task-removed = Đã xóa tác vụ { $name }
core-rollback-task-not-removed = Không thể xóa tác vụ { $name }: { $reason }.
core-rollback-task-put-back = Đã khôi phục tác vụ { $name }
core-rollback-task-not-put-back = Không thể khôi phục tác vụ { $name }: { $reason }.

## When an install cannot be undone at all

# { $error } is what went wrong.
core-rollback-record-unreadable = Không thể đọc bản ghi của lần cài đặt này, nên không có gì được hoàn tác: { $error }.
# { $error } is what went wrong.
core-rollback-nothing-undone = Không có gì được hoàn tác: { $error }.
# One app's part of the message below. { $app_name } is the app's name; { $failures } is what could not be undone, a
# sentence or more; { $folder } is the folder that holds the install's record.
core-rollback-earlier-install-stuck = { $app_name }: { $failures } Nếu không bao giờ hoàn tác được lần cài đặt này, quản trị viên có thể từ bỏ nó bằng cách xóa { $folder }.
# Why an install did not start. { $details } is one or more of the message above.
core-rollback-earlier-installs-not-undone = không thể hoàn tác hoàn toàn một lần cài đặt trước đây chưa hoàn tất, nên không có gì được cài đặt. Hãy đóng mọi thứ đang dùng các mục này rồi thử lại. { $details }
core-rollback-install-busy = lần cài đặt này đang bị một trình cài đặt hoặc trình gỡ cài đặt khác thay đổi

## The summary an uninstall writes on the command line, one line each, with what each says below it. { $count } is how
## many.

core-rollback-count-undone = Đã hoàn tác: { $count }
core-rollback-count-failed = Không thể hoàn tác: { $count }
# What was left on purpose: it has changed since the install, or it is the user's.
core-rollback-count-kept = Được giữ nguyên: { $count }
# What no uninstall can take back, such as a runtime that was installed, which other programs may use.
core-rollback-count-left-behind = Còn sót lại: { $count }

## Which Windows a PC or drive has, as the app shows it beside the drive.

# { $name } is the Windows' name, such as Windows 7 Professional Service Pack 1.
core-system-windows-32-bit = { $name }, 32 bit
# { $name } is the Windows' name, such as Windows 11 Pro.
core-system-windows-64-bit = { $name }, 64 bit
# A drive's Windows, when it only says it is older than Vista.
core-system-windows-xp-or-older = Windows XP hoặc cũ hơn

## Reading a connected drive, or another account's settings. Each is why it could not be done, and some start with a
## small letter, as they follow other words.

core-system-needs-administrator = việc đọc ổ đĩa đã kết nối hoặc thiết đặt của tài khoản khác cần quyền quản trị viên
core-system-no-program-data = không tìm thấy thư mục ProgramData của PC này
# { $path } is a file, such as D:\Windows\System32\config\SOFTWARE.
core-system-not-a-hive-file = không phải là tệp hive sổ đăng ký: { $path }
# { $path } is a file; { $size } is its size in bytes.
core-system-hive-too-large = { $path } có kích thước { $size } byte, quá lớn để là một hive sổ đăng ký, hoặc lớn hơn tổng kích thước các hive của ổ đĩa này
# { $key } is a registry key, such as Software\Microsoft\Windows\CurrentVersion.
core-system-hive-no-mapping = hive ngoại tuyến không có ánh xạ cho HKLM\{ $key }
# { $key } is where a drive's registry was to be loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said.
# "reg load" is a command's name.
core-system-hive-not-loaded = reg load { $key } không thành công: { $error }
# { $key } is where a drive's registry was loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said. "reg
# unload" is a command's name.
core-system-hive-not-unloaded = reg unload { $key } không thành công: { $error }
# { $drive } is a drive, such as D:\.
core-system-not-a-system-drive = { $drive } không phải là ổ đĩa hệ thống Windows (không có thư mục \Windows).
# { $drive } is a drive, such as C:\. "This PC" is the name of a choice in the app.
core-system-own-system-drive = { $drive } là ổ đĩa hệ thống của chính PC này. Hãy chọn PC này để đọc PC này.
# { $drive } is a drive, such as D:\.
core-system-registry-through-link = { $drive } không phải là ổ đĩa hệ thống Windows: đường dẫn tới sổ đăng ký của ổ đĩa này đi qua một liên kết.

## Warnings a scan records about whose data it read.

# { $account } is an account's name or id, such as alice or S-1-5-21-...-1001.
core-warning-scan-account-not-on-record = Tài khoản { $account } không được ghi nhận ở đây, nên không có dữ liệu hay thiết đặt theo người dùng nào được đưa vào.
# { $account } is an account's name, such as CONTOSO\alice.
core-warning-scan-signed-in-account-no-profile = Tài khoản đang đăng nhập ở đây, { $account }, không có hồ sơ người dùng được ghi nhận, nên không có dữ liệu hay thiết đặt theo người dùng nào được đưa vào.
# { $reason } is why Windows could not say.
core-warning-scan-signed-in-account-unknown = Không xác định được ai đang đăng nhập ({ $reason }), nên không có dữ liệu hay thiết đặt theo người dùng nào được đưa vào. Hãy chọn dữ liệu của ai cần đưa vào.
core-warning-scan-last-account-unknown = Ổ đĩa này không cho biết ai đăng nhập lần cuối, và có nhiều hơn một tài khoản, nên không có dữ liệu hay thiết đặt theo người dùng nào được đưa vào. Hãy chọn dữ liệu của ai cần đưa vào.
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-settings-unreadable = Không thể đọc thiết đặt của { $account }, nên các thiết đặt đó không được đưa vào: { $error }
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-classes-unreadable = Không thể đọc các lớp COM và loại tệp đã đăng ký cho { $account }, nên chúng không được đưa vào: { $error }

## Paths a scan did not follow. { $count } is how many; { $paths } is some of them, separated by semicolons; { $more } is
## how many more there are, not shown.

core-warning-scan-drive-paths-not-followed =
    { $count ->
       *[other] Không thể đọc đường dẫn được ghi nhận trên ổ đĩa này, nên không có gì ở đó được đưa vào. Mỗi đường dẫn nằm trên đĩa khác, không phải là đường dẫn đầy đủ, hoặc đi qua một liên kết dẫn ra ngoài ổ đĩa: { $paths }
    }
core-warning-scan-drive-paths-not-followed-more = Không thể đọc một số đường dẫn được ghi nhận trên ổ đĩa này, nên không có gì ở đó được đưa vào. Mỗi đường dẫn nằm trên đĩa khác, không phải là đường dẫn đầy đủ, hoặc đi qua một liên kết dẫn ra ngoài ổ đĩa: { $paths } và { $more } đường dẫn khác
core-warning-scan-network-paths-not-followed =
    { $count ->
       *[other] Không thể đọc đường dẫn được ghi nhận trên PC này, nên không có gì ở đó được đưa vào. Mỗi đường dẫn hoặc không nằm trên đĩa của PC này (khi đó việc kiểm tra đường dẫn sẽ kết nối tới nơi đó), hoặc có tên sẽ mở một thứ khác: { $paths }
    }
core-warning-scan-network-paths-not-followed-more = Không thể đọc một số đường dẫn được ghi nhận trên PC này, nên không có gì ở đó được đưa vào. Mỗi đường dẫn hoặc không nằm trên đĩa của PC này (khi đó việc kiểm tra đường dẫn sẽ kết nối tới nơi đó), hoặc có tên sẽ mở một thứ khác: { $paths } và { $more } đường dẫn khác

## What a scan with none of an account's data leaves out of a package, in the app's list of what was left out. Each
## has a name, and below it why.

core-account-left-out-shortcuts = Lối tắt trong các thư mục riêng của tài khoản này
# { $count } is how many shortcuts.
core-account-left-out-shortcuts-why =
    { $count ->
       *[other] { $count } lối tắt, chỉ được đưa vào khi có tùy chọn Dữ liệu người dùng
    }
core-account-left-out-variables = Biến môi trường của tài khoản này
# { $count } is how many variables.
core-account-left-out-variables-why =
    { $count ->
       *[other] { $count } biến, chỉ được đưa vào khi có tùy chọn Dữ liệu người dùng
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-account-left-out-task = Tác vụ đã lên lịch { $task }
core-account-task-works-on-account-folder = Tác vụ này làm việc trên một vị trí trong thư mục của một tài khoản, hoặc được đặt tên theo vị trí đó, và chỉ được đưa vào khi có tùy chọn Dữ liệu người dùng
core-account-task-runs-from-account-folder = Những gì tác vụ này cũng chạy từ thư mục của một tài khoản chỉ được đưa vào khi có tùy chọn Dữ liệu người dùng
# { $service } is a service's name, such as AppUpdater.
core-account-left-out-service = Dịch vụ { $service }
core-account-service-works-on-account-folder = Dịch vụ này làm việc trên một vị trí trong thư mục của một tài khoản, và chỉ được đưa vào khi có tùy chọn Dữ liệu người dùng

## What a package was found to still say of whose PC it came from, after a build with none of an account's data.

# The parts of a package, as the warning below names them.
core-account-part-files = đường dẫn của các tệp riêng của ứng dụng
core-account-part-registry = giá trị sổ đăng ký
core-account-part-settings-entry = mục của ứng dụng trong Cài đặt
core-account-part-main-program = vị trí chương trình chính của ứng dụng
core-account-part-shortcuts = lối tắt
core-account-part-tasks = tác vụ đã lên lịch
core-account-part-services = dịch vụ
core-account-part-variables = biến môi trường
# One place in the list below. { $place } is one of the parts above, or a file's name; { $count } is how many times.
core-account-mention-place = { $place } ({ $count })
# { $kind } is what was found: account (an account's name), folder (where an account's folder is), computer (the PC's
# name), or id (an account's id). { $places } is a list of where, from the message above.
core-warning-scan-package-mentions =
    { $kind ->
        [account] Gói này nhắc đến tên của một tài khoản trong { $places }. Gói được tạo mà không có dữ liệu của tài khoản nào, và bất kỳ ai có trình cài đặt đều có thể đọc thông tin này.
        [folder] Gói này nhắc đến vị trí thư mục của một tài khoản trong { $places }. Gói được tạo mà không có dữ liệu của tài khoản nào, và bất kỳ ai có trình cài đặt đều có thể đọc thông tin này.
        [computer] Gói này nhắc đến tên của PC trong { $places }. Gói được tạo mà không có dữ liệu của tài khoản nào, và bất kỳ ai có trình cài đặt đều có thể đọc thông tin này.
       *[id] Gói này nhắc đến ID của một tài khoản trong { $places }. Gói được tạo mà không có dữ liệu của tài khoản nào, và bất kỳ ai có trình cài đặt đều có thể đọc thông tin này.
    }
# { $count } is how many folders.
core-warning-scan-folders-of-others-unreadable =
    { $count ->
       *[other] Không thể đọc { $count } thư mục nằm ngoài ứng dụng, nên dữ liệu của ứng dụng ở đó có thể bị thiếu.
    }

## What a scan leaves out because it looks like a credential, or is a license with the License option off, in the app's
## list of what was left out. Each has a name, and below it why.

core-secret-why-credential = mục này trông giống thông tin xác thực
core-secret-why-license-option-off = tùy chọn Dữ liệu giấy phép và kích hoạt đang tắt
# { $count } is how many values of a registry key.
core-secret-why-account-values =
    { $count ->
       *[other] { $count } giá trị, chỉ được đưa vào khi có tùy chọn Dữ liệu người dùng
    }
# Shown in place of a registry value's name when its name itself looks like a password or a key.
core-secret-name-holds-credential = (một giá trị có tên chứa thông tin xác thực)
# Shown in place of an option's name, for an argument on a command line that has no name.
core-secret-unnamed-argument = một đối số chứa thông tin xác thực
# { $option } is an option taken out of a command line, such as /password, or the message above. { $place } says
# where the command line was: service, task, shortcut, opener (what opens a file type), opener-unnamed (the same, of a
# type with no name), or uninstall. { $name } is the service, task, shortcut, file type, or app.
core-secret-taken-from =
    { $place ->
        [service] { $option } trong cách chạy { $name }
        [task] { $option } trong cách chạy tác vụ { $name }
        [shortcut] { $option } trong lối tắt { $name }
        [opener] { $option } trong chương trình mở { $name }
        [opener-unnamed] { $option } trong chương trình mở một loại tệp
       *[uninstall] { $option } trong cách gỡ cài đặt { $name }
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-secret-task-definition-unreadable = Định nghĩa của tác vụ { $task }, không thể đọc để kiểm tra
# { $name } is an environment variable's name, such as APP_TOKEN.
core-secret-left-out-variable = Biến môi trường { $name }
# { $class } is a COM class's id, such as {00000000-0000-0000-0000-000000000000}.
core-secret-left-out-class-server = Những gì lớp { $class } cho biết là nó chạy
# { $rule } is a firewall rule's name.
core-secret-left-out-firewall-description = Mô tả của quy tắc tường lửa { $rule }

## Licenses a scan found, as the package's report describes each.

# { $name } is a file's name, such as license.lic.
core-license-detail-file = Đã phát hiện tệp giấy phép: { $name }
# { $framework } is a licensing product's name, such as FlexLM / FlexNet.
core-license-detail-store = Có kho giấy phép { $framework }
# { $name } is an environment variable's name, such as LM_LICENSE_FILE.
core-license-detail-variable = Biến môi trường giấy phép { $name }
# { $name } is a registry value's name, such as SerialNumber.
core-license-detail-registry-value = Giá trị sổ đăng ký có vẻ chứa dữ liệu giấy phép: { $name }
# Said of a registry value in an account's key, which is not named.
core-license-detail-registry-value-of-an-account = Giá trị sổ đăng ký có vẻ chứa dữ liệu giấy phép
# { $key } is a registry key, such as HKLM\Software\Contoso\License.
core-license-detail-registry-key = Khóa sổ đăng ký có vẻ chứa dữ liệu giấy phép: { $key }

## Warnings a scan records about licenses.

core-warning-scan-license-data = Đã phát hiện dữ liệu giấy phép/kích hoạt. Khi bật tùy chọn Dữ liệu giấy phép và kích hoạt, trình cài đặt sẽ khôi phục những gì có thể di chuyển; các giá trị toàn máy và kho gắn với phần cứng chỉ được báo cáo. WinAppMigrator không bao giờ giải mã, bỏ qua kích hoạt hay giả lập khóa cứng (dongle).
core-warning-scan-license-machine-bound = Một số giấy phép gắn với máy (khóa cứng, kích hoạt khóa theo máy hoặc được mã hóa). Trình cài đặt khôi phục những gì có thể; các giấy phép này vẫn có thể cần kích hoạt lại trên PC mới.

## Warnings a scan records about the registry. What could not be read is not named in them: it is kept beside them.

# { $error } is what Windows said went wrong. App Paths is a registry key's name.
core-warning-scan-app-paths-key-unreadable = Không thể đọc khóa con của App Paths: { $error }
core-warning-scan-registry-limit = Việc thu thập sổ đăng ký đã đạt giới hạn an toàn đã cấu hình.
core-warning-scan-registry-subtree-limit = Việc thu thập cây con của sổ đăng ký đã đạt giới hạn an toàn đã cấu hình.
# { $error } is what Windows said went wrong.
core-warning-scan-registry-key-unreadable = Không thể đọc khóa sổ đăng ký: { $error }
core-warning-scan-registry-value-too-large = Giá trị sổ đăng ký vượt quá giới hạn kích thước thu thập đã cấu hình.

## Scheduled tasks

core-warning-scan-tasks-folder-through-link = Thư mục tác vụ đã lên lịch trên ổ đĩa này đi qua một liên kết, nên các tác vụ trong đó không được đọc.
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-warning-scan-task-left-out = Tác vụ đã lên lịch { $task } đã bị loại trừ: tác vụ này chạy dưới tài khoản khác với tài khoản có thiết đặt đang được đưa vào, người khác ngoài quản trị viên có thể thay đổi tệp của tác vụ, hoặc ứng dụng có quá nhiều tác vụ để giữ lại.
# { $error } is what Windows said went wrong.
core-warning-scan-tasks-folder-unreadable = Không thể đọc một phần các tác vụ đã lên lịch: { $error }
# { $path } is a task's file, such as C:\Windows\System32\Tasks\App\Nightly Sync; { $reason } is one of the reasons
# below, or what Windows said went wrong.
core-warning-scan-task-unreadable = Không thể đọc tác vụ đã lên lịch { $path }: { $reason }. Nếu tác vụ này chạy ứng dụng, nó đã không được đưa vào.
# Why a task's file could not be read. { $size } is its size in bytes.
core-task-too-large = tệp có kích thước { $size } byte, quá lớn để là một tác vụ
# Why a task's file could not be read.
core-task-ends-mid-character = tệp kết thúc giữa chừng một ký tự
# Why a task's file could not be read. { $error } says where.
core-task-not-utf16 = tệp không phải là UTF-16 hợp lệ: { $error }
# Why a task's file could not be read. { $error } says where.
core-task-not-text = tệp không phải là UTF-8 hoặc UTF-16 hợp lệ: { $error }
# What a package records of how a task is put back. Register-ScheduledTask is a command's name.
core-task-restore-instructions = Việc khôi phục dùng Register-ScheduledTask với định nghĩa XML đã lưu trữ.

## Reading a program's file. Each is why it could not be read, after the program's name.

# { $reason } is what went wrong.
core-pe-unreadable = không thể đọc tệp: { $reason }
core-pe-headers-misaligned = phần đầu của tệp không nằm trên ranh giới mà phần đầu của một chương trình phải tuân theo
# { $size } is the file's size in bytes.
core-pe-too-large = tệp có kích thước { $size } byte, quá lớn để là một chương trình
# PE is the name of the format Windows programs are in.
core-pe-no-headers = tệp không có phần đầu PE

## Warnings a scan records about what a program needs. Which program is kept beside each.

# { $error } is what went wrong.
core-warning-scan-imports-unreadable = Không thể đọc bảng nhập: { $error }
# { $error } is what went wrong.
core-warning-scan-import-name-unreadable = Không thể đọc tên của một bộ mô tả nhập: { $error }
# { $count } is how many were skipped.
core-warning-scan-legacy-delay-imports =
    { $count ->
       *[other] Đã bỏ qua { $count } bộ mô tả tải trễ kiểu cũ (v1); chỉ các mục nhập tải trễ hiện đại dựa trên RVA mới được phân tích.
    }

## The DLLs a program needs. Which DLL or program is kept beside each warning.

# { $path } is a program, such as C:\Program Files\App\app.exe.
core-dll-program-has-no-folder = tệp thực thi không có thư mục cha: { $path }
# Why a DLL is among the app's files, in the package's list of them.
core-dll-beside-program = DLL nằm bên cạnh tệp thực thi đã chọn
# { $error } is what went wrong.
core-warning-scan-dependency-unscanned = Không thể quét phần phụ thuộc đệ quy: { $error }
# { $name } is a DLL's name, such as d3dx9_43.dll.
core-warning-scan-dependency-not-in-newer-windows = { $name } nằm trong thư mục Windows của PC mà ứng dụng này từng chạy, và Windows 10 và 11 không đi kèm tệp này. Một chương trình nào đó đã cài đặt tệp vào đó, nên tệp không thuộc các tệp riêng của ứng dụng và không được mang theo. Ứng dụng có thể không khởi động cho đến khi thứ cung cấp tệp này được cài đặt.
# { $error } is what went wrong.
core-warning-scan-dependency-not-hashed = Không thể tính hàm băm của phần phụ thuộc: { $error }
# SystemRoot and windir are the names of environment variables.
core-warning-scan-windows-folder-unknown = Không có biến môi trường SystemRoot/windir; việc tìm kiếm DLL hệ thống bị hạn chế.
# { $error } is what went wrong.
core-warning-scan-dependency-architecture-unreadable = Không thể đọc kiến trúc của phần phụ thuộc: { $error }

## Services, drivers, shortcuts, and environment variables a scan reads. Where it was reading is kept beside each
## warning.

# { $error } is what Windows said went wrong.
core-warning-scan-services-unreadable = Không thể đọc khóa sổ đăng ký dịch vụ: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-drivers-unreadable = Không thể đọc khóa sổ đăng ký dịch vụ để phát hiện trình điều khiển: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-queried = Không thể truy vấn lối tắt: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-parsed = Không thể phân tích kết quả truy vấn lối tắt: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-environment-unreadable = Không thể đọc khóa sổ đăng ký môi trường của máy: { $error }

## What a package records of how a driver is put back, which an installer shows. { $kind } is the driver's type as
## Windows names it (kernel, file-system, recognizer, or unknown-type); { $name } is its service's name; { $path } is
## its file. { $signed } is yes when { $signer } names who signed it, and no when that is not known.

core-service-driver-bundled =
    { $signed ->
        [yes] Trình điều khiển { $kind } thuộc về ứng dụng “{ $name }” ({ $path }, được ký bởi { $signer }). Trình cài đặt đóng gói tệp .sys của trình điều khiển và tự động đăng ký lại dịch vụ trình điều khiển (sc create, cần nâng quyền), sau đó xác nhận dịch vụ đã được đăng ký.
       *[no] Trình điều khiển { $kind } thuộc về ứng dụng “{ $name }” ({ $path }, chữ ký không xác định). Trình cài đặt đóng gói tệp .sys của trình điều khiển và tự động đăng ký lại dịch vụ trình điều khiển (sc create, cần nâng quyền), sau đó xác nhận dịch vụ đã được đăng ký.
    }
core-service-driver-from-vendor =
    { $signed ->
        [yes] Trình điều khiển { $kind } của nhà cung cấp/có sẵn trong Windows “{ $name }” ({ $path }, được ký bởi { $signer }). WinAppMigrator không phân phối lại trình điều khiển của bên thứ ba; hãy cài đặt lại trình điều khiển này bằng gói trình điều khiển đã ký của nhà cung cấp trên máy đích trước khi mở ứng dụng.
       *[no] Trình điều khiển { $kind } của nhà cung cấp/có sẵn trong Windows “{ $name }” ({ $path }, chữ ký không xác định). WinAppMigrator không phân phối lại trình điều khiển của bên thứ ba; hãy cài đặt lại trình điều khiển này bằng gói trình điều khiển đã ký của nhà cung cấp trên máy đích trước khi mở ứng dụng.
    }

## Runtimes an app needs, such as the Visual C++ runtime or .NET: how each is provided, as the Dependencies list says.

core-runtime-handling-built-in = Có sẵn trong Windows
core-runtime-handling-ships-with-app = Đi kèm ứng dụng
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-handling-windows-feature = Tính năng Windows: { $feature }
core-runtime-handling-included = Đã bao gồm
# { $id } is the runtime's name in winget, such as Microsoft.VCRedist.2015+.x64.
core-runtime-handling-winget = Được cài đặt bằng winget: { $id }
core-runtime-handling-manual = Cài đặt thủ công

## Downloading a runtime's installer, when an installer is built. Each is why it could not be, and starts with a small
## letter, as it follows other words. { $url } is a web address; { $error } is what went wrong.

core-runtime-download-not-https = từ chối tải xuống { $url }: địa chỉ không dùng HTTPS
# TLS is what keeps a download private.
core-runtime-tls-failed = khởi tạo TLS không thành công: { $error }
core-runtime-download-failed = tải xuống { $url } không thành công: { $error }
core-runtime-download-unreadable = đọc { $url } không thành công: { $error }
core-runtime-download-empty = tải xuống { $url } không trả về dữ liệu nào
# { $came } and { $expected } are numbers of bytes.
core-runtime-download-cut-short = tải xuống { $url } bị ngắt giữa chừng: chỉ nhận được { $came } trên { $expected } byte

## Installing a runtime: why it was not installed. Each starts with a small letter, as it follows other words.

core-runtime-no-known-installer = không có trình cài đặt nào đã biết cho thư viện thời gian chạy này
# { $path } is where in the package, such as prerequisites\vc_redist.x64.exe.
core-runtime-payload-misplaced = gói chỉ định { $path } là trình cài đặt của thư viện thời gian chạy này, nhưng đó không phải vị trí đúng của nó, nên trình cài đặt không được chạy
# { $path } is a file.
core-runtime-payload-missing = thiếu trình cài đặt đi kèm: { $path }
# { $error } is what went wrong.
core-runtime-payload-path-invalid = đường dẫn trình cài đặt đi kèm không hợp lệ: { $error }
# { $url } is the web address it can be downloaded from. winget is a program's name.
core-runtime-no-installer-route = không có trình cài đặt đi kèm hay ID winget; hãy tải về từ { $url }
# { $error } is what went wrong.
core-runtime-no-temp-folder = không thể tạo thư mục để giải nén trình cài đặt: { $error }
# { $reason } is what the check found.
core-runtime-payload-signature-failed = trình cài đặt đi kèm không vượt qua kiểm tra chữ ký và không được chạy: { $reason }
core-runtime-payload-timed-out = trình cài đặt đi kèm không hoàn tất trong thời gian chờ và đã bị dừng
# { $error } is what went wrong.
core-runtime-payload-not-started = không thể khởi động trình cài đặt đi kèm: { $error }
core-runtime-installer-killed = trình cài đặt đã bị dừng mà không có mã thoát
# { $code } is the number it ended with, such as 1603.
core-runtime-installer-failed = trình cài đặt đã thoát với mã { $code }
# { $feature } is a Windows feature's name, such as NetFx3; { $code } is the number DISM ended with. Windows Features is
# the name of a Windows window. Keep the command as it is.
core-runtime-feature-failed = DISM không thể bật tính năng Windows { $feature } (mã thoát { $code }); có thể không tải xuống được dữ liệu cần thiết. Hãy bật tính năng này qua Tính năng của Windows, hoặc chạy `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`.
# The same, when DISM gave no number.
core-runtime-feature-failed-no-code = DISM không thể bật tính năng Windows { $feature } (mã thoát không xác định); có thể không tải xuống được dữ liệu cần thiết. Hãy bật tính năng này qua Tính năng của Windows, hoặc chạy `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`.
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-feature-timed-out = việc bật tính năng Windows { $feature } không hoàn tất trong thời gian chờ và đã bị dừng
# { $feature } is a Windows feature's name; { $error } is what went wrong.
core-runtime-feature-no-dism = không có DISM để bật tính năng Windows { $feature }: { $error }

## Installing a runtime with winget. { $id } is its name in winget, such as Microsoft.VCRedist.2015+.x64.

# { $error } is what went wrong.
core-runtime-winget-not-looked-for = không thể tìm winget: { $error }
core-runtime-winget-not-installed = winget chưa được cài đặt
# { $path } is where winget is.
core-runtime-winget-misplaced = { $path } không nằm ở nơi Windows lưu các gói riêng của mình, nên không được chạy
# { $path } is where winget is; { $reason } is what the check found.
core-runtime-winget-signature-failed = { $path } không vượt qua kiểm tra chữ ký và không được chạy: { $reason }
# { $reason } is one of the two messages above, or what went wrong.
core-runtime-winget-unusable = không thể dùng winget để cài đặt { $id }: { $reason }
# { $code } is the number winget ended with.
core-runtime-winget-failed = winget install { $id } đã thoát với mã { $code }
core-runtime-winget-failed-no-code = winget install { $id } đã thoát với mã không xác định
core-runtime-winget-timed-out = winget install { $id } không hoàn tất trong thời gian chờ và đã bị dừng
# { $error } is what went wrong.
core-runtime-winget-not-started = không có winget để cài đặt { $id }: { $error }

## What told a scan how an app was installed, as the package's report lists it. The names of installers and files are
## as their makers write them.

# { $path } is an installer's record of what it installed, such as C:\Program Files\App\unins000.dat.
core-source-record-not-believed = { $path } không được dùng: người khác ngoài quản trị viên có thể thay đổi tệp này
# { $count } is how many Windows Installer packages the bundle holds.
core-source-burn-bundle =
    { $count ->
       *[other] Gói Burn gồm { $count } gói Windows Installer
    }
# { $count } is how many paths Windows Installer recorded.
core-source-msi-database =
    { $count ->
       *[other] Cơ sở dữ liệu Windows Installer: { $count } đường dẫn khóa thành phần
    }
core-source-inno-log = Nhật ký gỡ cài đặt Inno Setup (unins000.dat)
# { $count } is how many paths the log records.
core-source-inno-log-paths =
    { $count ->
       *[other] Nhật ký gỡ cài đặt Inno Setup: { $count } đường dẫn được ghi lại
    }
core-source-nsis = Chữ ký trình gỡ cài đặt NSIS (Nullsoft)
# { $count } is how many paths the log records.
core-source-nsis-log-paths =
    { $count ->
       *[other] NSIS AdvUninstLog uninstall.log: { $count } đường dẫn được ghi lại
    }
core-source-squirrel = Bố cục theo người dùng của Squirrel (Update.exe / app-*)
# { $version } is a folder's name that says the version, such as app-1.2.3.
core-source-squirrel-version = Phiên bản hiện tại của Squirrel: { $version }
core-source-chocolatey = Siêu dữ liệu gói Chocolatey
core-source-msix = Gói MSIX / Appx
# { $identity } is a package's name, such as Contoso.App.
core-source-appx-identity = Danh tính AppxManifest: { $identity }
# { $count } is how many folders.
core-source-msix-data =
    { $count ->
       *[other] Dữ liệu gói MSIX theo người dùng: { $count } vị trí
    }
# App Paths is a registry key's name.
core-source-app-paths = Đăng ký App Paths

## Why a runtime is needed, as the package's report says it.

# { $program } is a program's file name, such as app.exe; { $dll } is a DLL's, such as msvcp140.dll.
core-runtime-program-imports = { $program } nhập { $dll }

## What else of an app's a scan finds in the registry: fonts and certificates, and why each is the app's.

# Why a font is among the app's files, in the package's list of them.
core-registry-font-recorded = Một phông chữ mà bản ghi cài đặt của ứng dụng tham chiếu
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-named-for-the-app = chứng chỉ mang tên ứng dụng hoặc tên nhà phát hành
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-recorded = bản ghi cài đặt của ứng dụng tham chiếu chứng chỉ này

## Who is signed in, for the Whose Data choice. Each of the first six is why it could not be told, and starts with a
## small letter, as it is shown inside other words. An account is shown by its id, such as S-1-5-21-...-1001.

# { $error } is what Windows said.
core-profile-desktop-owner-not-said = Windows không cho biết màn hình nền này của ai: { $error }
# { $error } is what went wrong.
core-profile-desktop-owner-not-asked = không thể hỏi Windows màn hình nền này của ai: { $error }
core-profile-signed-in-account-no-profile = tài khoản đang đăng nhập ở đây, { $account }, không có hồ sơ người dùng được ghi nhận
core-profile-desktop-not-a-person = màn hình nền ở đây thuộc về { $account }, không phải tài khoản của một người
core-profile-nobody-signed-in-running-account-no-profile = không có ai đăng nhập ở đây, và tài khoản đang chạy chương trình này, { $account }, không có hồ sơ người dùng được ghi nhận
core-profile-nobody-signed-in-running-not-a-person = không có ai đăng nhập ở đây, và { $account }, tài khoản đang chạy chương trình này, không phải tài khoản của một người
# SID is the name Windows gives an account's id.
core-profile-nobody-signed-in-no-sid = không có ai đăng nhập ở đây, và tài khoản này không có SID
# { $accounts } is a list of accounts.
core-profile-several-desktops = có nhiều hơn một tài khoản có màn hình nền trong phiên này: { $accounts }
# Under the Whose Data choice. { $reason } is one of the messages above.
core-profile-signed-in-unknown = Không xác định được ai đang đăng nhập ({ $reason }).
# Under the Whose Data choice, when there are accounts to choose from. { $reason } is one of the messages above.
core-profile-signed-in-unknown-choose = Không xác định được ai đang đăng nhập ({ $reason }). Hãy chọn dữ liệu của ai cần đưa vào.
