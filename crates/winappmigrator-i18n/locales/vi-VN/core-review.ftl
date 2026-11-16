# Words the core says about what an install may do: why an install location or something in a package is refused,
# the installer's list of what an app sets up beyond its files, the checks of a package and of an install, and whether
# there is room for either. Message names start with core-.
#
# Many messages here are a reason, the second half of a sentence that starts "... was left out:" or "that install
# location cannot be used:", so they start with a small letter and have no full stop.


## Why an install location cannot be used. Shown in red under the Install Location field.

core-review-location-state-root = đây là nơi các lần cài đặt lưu bản ghi
core-review-location-has-files = thư mục này đã có tệp và không có lần cài đặt nào được ghi nhận ở đây, nên hãy dùng một thư mục trống
core-policy-location-not-full-path = vị trí này phải là đường dẫn đầy đủ
core-policy-location-parent-step = vị trí này không được chứa “..”
core-policy-location-not-plain = vị trí này phải là một đường dẫn thông thường trên một ổ đĩa của PC này
core-policy-location-whole-drive = vị trí này không thể là toàn bộ ổ đĩa
core-policy-location-windows = vị trí này không thể là thư mục Windows
core-policy-location-runs-by-itself = đây là thư mục thuộc về Windows, hoặc là thư mục có nội dung tự chạy hoặc tự được tải
core-policy-location-shared = thư mục này chứa nhiều thứ khác nằm cạnh nhau, nên ứng dụng cần một thư mục bên trong nó

## Whether an app can run on this PC at all, said before anything is installed.

# { $reason } is why, as the second half of a sentence, such as "App.exe is built for Arm64, and this PC is x64, so
# Windows would not start it".
core-review-cannot-run-here = ứng dụng này không thể chạy trên PC này, nên không có gì bị thay đổi: { $reason }
core-policy-architecture-x86 = x86 32 bit
core-policy-architecture-x64 = x64
core-policy-architecture-arm64 = Arm64
core-policy-architecture-arm32 = Arm 32 bit
core-policy-architecture-unknown = một kiến trúc không xác định
# { $code } is the number Windows gives a kind of processor, such as 0x01c4.
core-policy-machine-type = loại máy { $code }
# { $names } is a list of program names, such as "a.exe, b.exe, c.exe", and { $count } how many more there are.
core-policy-names-and-more = { $names } và { $count } mục khác
core-policy-fit-programs-not-said = Trình cài đặt này không cho biết các chương trình của nó cần gì, nên chưa kiểm tra liệu chúng có thể chạy trên PC này hay không.
core-policy-fit-architecture-not-told = Không xác định được kiến trúc của PC này, nên chưa kiểm tra liệu các chương trình của ứng dụng này có thể chạy trên PC này hay không.
core-policy-fit-main-not-said = Trình cài đặt này không cho biết chương trình chính của nó cần gì, nên thông tin ở đây dựa trên phần còn lại mà nó có.
# { $driver } is a driver's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-driver = trình điều khiển { $driver } của ứng dụng dành cho { $built_for }, còn PC này là { $here }. Trình điều khiển phải khớp chính xác với Windows, nên ứng dụng này không thể chạy ở đây
# { $program } is a program's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-program = { $program } dành cho { $built_for }, còn PC này là { $here }, nên Windows sẽ không khởi động chương trình đó
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-none-start = không chương trình nào của ứng dụng có thể khởi động trên PC này, vốn là { $here }: { $programs }
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-some-other-architecture = Một số chương trình của ứng dụng này dành cho kiến trúc khác và sẽ không khởi động trên PC này, vốn là { $here }: { $programs }. Phần còn lại của ứng dụng có thể khởi động, nên điều này có thể không quan trọng.
# { $here } is a kind of processor, such as "Arm64". { $programs } is a list of program names.
core-policy-fit-emulated = PC này là { $here }, nên Windows sẽ chạy các chương trình sau thông qua trình mô phỏng: { $programs }. Hầu hết các chương trình đều chạy ổn theo cách này, nhưng một số thì không.
# { $program } is a program's file name. { $needs } and { $runs } are versions of Windows, such as 10.0 and 6.1.
core-policy-fit-needs-newer = { $program } cho biết cần Windows { $needs } trở lên, còn PC này chạy { $runs }, nên Windows sẽ không khởi động chương trình đó
# { $programs } is a list of program names.
core-policy-fit-some-need-newer = Một số chương trình của ứng dụng này cho biết cần phiên bản Windows mới hơn phiên bản này và sẽ không khởi động ở đây: { $programs }.
# { $from_build } and { $this_build } are Windows build numbers, such as 22631, written as Windows writes them.
core-policy-fit-older-build = Ứng dụng này từng nằm trên Windows bản dựng { $from_build }, còn PC này chạy bản dựng { $this_build }. Các chương trình của ứng dụng cho biết chúng có thể khởi động trên phiên bản Windows này, nên ứng dụng có thể hoạt động, nhưng những gì ứng dụng dựa vào ở đó có thể không có ở đây.
core-policy-fit-build-not-told = Không xác định được bản dựng Windows này, nên chưa so sánh với bản dựng mà ứng dụng này từng chạy.

## Room on a drive for a build or an install.

# Each of these ends the sentence core-space-shortfall, after "free for".
core-space-for-unpacking = giải nén gói
core-space-for-install-record = bản ghi cài đặt
core-space-for-app-files = các tệp của ứng dụng
core-space-for-what-is-replaced = lưu giữ những gì bị thay thế
core-space-for-build-work = dữ liệu làm việc khi tạo trình cài đặt
core-space-for-installer = trình cài đặt
core-space-for-package = gói
core-space-for-unpacking-and-installing = giải nén và cài đặt gói
# What goes between two of those, as in "unpacking the package and the app's files". Keep the quotes: they keep the
# spaces.
core-space-purposes-joiner = { " và " }
# { $drive } is a drive, such as C:, or a folder. { $needed } and { $free } are sizes, such as 1.2 GB. { $purposes } is
# what the room is for, such as "unpacking the package and the app's files".
core-space-shortfall = { $drive } cần tối đa { $needed } dung lượng trống cho { $purposes }, nhưng chỉ còn { $free }
# { $drive } is a drive, such as C:, or a folder. { $needed } is a size, such as 1.2 GB.
core-space-not-told = { $drive } không cho biết còn bao nhiêu dung lượng, nên chưa kiểm tra liệu { $needed } cần thiết có đủ chỗ hay không
# One drive in a list of where room is needed. { $needed } and { $free } are sizes, and { $drive } a drive, such as C:.
core-space-summary = tối đa { $needed } trên { $drive } (còn trống { $free })
# { $needed } is a size, and { $drive } a drive, such as C:.
core-space-summary-not-told = tối đa { $needed } trên { $drive } (không xác định được dung lượng còn trống)
# What goes between two sentences of core-space-shortfall inside core-review-no-room. Keep the quotes: they keep the
# space.
core-review-sentence-joiner = { ". " }
# { $shortfalls } is one or more sentences of core-space-shortfall.
core-review-no-room = không đủ dung lượng cho việc này, nên không có gì bị thay đổi. { $shortfalls }. Hãy giải phóng dung lượng ở đó, hoặc chọn vị trí cài đặt trên ổ đĩa khác.

## The installer's list of what an app sets up beyond its files: what each kind is called.

core-review-kind-service = Dịch vụ
core-review-kind-driver = Trình điều khiển
core-review-kind-scheduled-task = Tác vụ đã lên lịch
core-review-kind-autostart = Tự khởi động
core-review-kind-shortcut = Lối tắt
core-review-kind-path = PATH
core-review-kind-environment-variable = Biến môi trường
core-review-kind-shell-extension = Phần mở rộng shell
core-review-kind-menu-command = Lệnh menu
core-review-kind-link-type = Loại liên kết
core-review-kind-file-type = Loại tệp
core-review-kind-com = COM
core-review-kind-firewall-rule = Quy tắc tường lửa
core-review-kind-event-log-source = Nguồn nhật ký sự kiện
core-review-kind-font = Phông chữ
core-review-kind-odbc-driver = Trình điều khiển ODBC
core-review-kind-odbc-source = Nguồn dữ liệu ODBC
core-review-kind-compatibility-setting = Thiết đặt tương thích
core-review-kind-default-apps = Ứng dụng mặc định
core-review-kind-program-name = Tên chương trình
core-review-kind-other-settings = Thiết đặt khác

## The installer's list of what an app sets up: what each line says of it.

# { $file } is a program's file name. { $account } is the Windows account it runs as, such as LocalSystem. { $starts }
# says when it starts, and is not shown itself; for "other", { $start } is one of the core-review-start- words below.
core-review-service-detail =
    { $starts ->
        [automatic] Chạy { $file } dưới tài khoản { $account }, khởi động ngay và mỗi khi PC này khởi động
        [delayed] Chạy { $file } dưới tài khoản { $account }, khởi động ngay và một lúc sau mỗi lần PC này khởi động
        [manual] Chạy { $file } dưới tài khoản { $account }, chỉ khởi động khi có thứ gì đó yêu cầu
        [disabled] Chạy { $file } dưới tài khoản { $account }, đã bị tắt
        [other] Chạy { $file } dưới tài khoản { $account }, { $start }
       *[unknown] Chạy { $file } dưới tài khoản { $account }
    }
# { $account } is a Windows account, such as LocalService.
core-policy-account-custom = { $account } (tài khoản tùy chỉnh không được khôi phục)
# { $file } is a driver's file name.
core-review-driver-detail = Tải { $file } vào chính Windows
# { $file } is a driver's file name. { $start } is when Windows starts it: one of the core-review-start- words below.
core-review-driver-detail-starts = Tải { $file } vào chính Windows, { $start }
# When a service or driver starts, ending a sentence such as "Loads disk.sys into Windows itself, starts …".
core-review-start-boot = khởi động cùng Windows, trước mọi thứ khác
core-review-start-system = khởi động cùng Windows, trong lúc Windows đang khởi động
core-review-start-automatic = khởi động mỗi khi PC này khởi động
core-review-start-delayed = khởi động một lúc sau mỗi lần PC này khởi động
core-review-start-manual = chỉ khởi động khi có thứ gì đó yêu cầu
core-review-start-disabled = không bao giờ khởi động, vì đã bị tắt
core-review-start-unknown = khởi động theo cách mà Windows không cho biết
# { $account } is the account a task runs as: an account's name, or core-review-task-runs-as-you, or
# core-review-task-runs-as-creator.
core-review-task-detail = Chạy với tư cách { $account }
# { $account } is the account a task runs as, as for core-review-task-detail.
core-review-task-detail-highest = Chạy với tư cách { $account }, với quyền cao nhất có thể
core-review-task-runs-as-you = bạn, tức người đang đăng nhập
core-review-task-runs-as-creator = người dùng đã tạo tác vụ
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-autostart-detail =
    Chạy { $scope ->
        [everyone] khi bất kỳ ai đăng nhập
       *[you] khi bạn đăng nhập
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shortcut-startup-detail =
    Một lối tắt trong thư mục Khởi động, { $scope ->
        [everyone] cho mọi người trên PC này
       *[you] cho bạn
    }
# { $file } is the file name of what the shortcut opens. { $scope } is whom it is set up for: everyone on this PC,
# or you. It is not shown itself.
core-review-shortcut-detail =
    Mở { $file }, { $scope ->
        [everyone] cho mọi người trên PC này
       *[you] cho bạn
    }
# { $name } is an environment variable's name, such as PATH.
core-review-path-title = Thêm vào { $name }
core-review-path-install-folder = Thư mục cài đặt
# { $path } is a folder inside the install folder, such as bin\tools.
core-review-path-below-install-folder = Thư mục cài đặt\{ $path }
# { $value } is the variable's value, as it will be set.
core-review-variable-detail = Đặt thành { $value }
# { $count } is how many registry values.
core-review-other-settings-title =
    { $count ->
       *[other] { $count } thiết đặt nằm ngoài các khóa riêng của ứng dụng
    }
# { $keys } is a list of registry keys, such as HKCU\Software\Vendor.
core-review-other-settings-detail = Trong { $keys }
# { $keys } is a list of registry keys, and { $count } how many more there are.
core-review-other-settings-detail-more = Trong { $keys } và { $count } khóa khác
# { $way } says whether the rule allows or blocks, and incoming or outgoing connections; it is not shown itself.
# { $ports } is a list of port numbers, and { $port_count } how many. { $program } is the program's file name.
# { $networks } is a list of the kinds of network Windows names in English (domain, private, public), and
# { $network_count } how many.
# The kinds of network a firewall rule is for, as Windows Defender Firewall names its profiles in your language. They
# fill { $networks } in core-review-firewall-detail, joined into a list.
core-review-network-domain = miền
core-review-network-private = riêng tư
core-review-network-public = công cộng
core-review-firewall-detail =
    { $way ->
        [allow-in] Cho phép kết nối đến
        [allow-out] Cho phép kết nối đi
        [block-in] Chặn kết nối đến
       *[block-out] Chặn kết nối đi
    }{ $port_count ->
        [0] { "" }
       *[other] { " " }trên cổng { $ports }
    } cho { $program }, trên { $network_count ->
        [0] mọi mạng
       *[other] các mạng { $networks }
    }
# { $server } is the file name of what Explorer loads. { $scope } is whom it is set up for: everyone on this PC, or
# you. It is not shown itself.
core-review-shell-extension-detail =
    Tải { $server } vào Chương trình Quản lý Tệp, { $scope ->
        [everyone] cho mọi người trên PC này
       *[you] cho bạn
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shell-extension-detail-own-file =
    Tải một tệp mà ứng dụng cài đặt vào Chương trình Quản lý Tệp, { $scope ->
        [everyone] cho mọi người trên PC này
       *[you] cho bạn
    }
# The name of a shell extension: { $handler } is its name in the registry, and { $class } the kind of file or folder it
# is for, such as * or Directory.
core-policy-shell-extension-name = { $handler } trên { $class }
# { $on } is the kind of file or folder whose menu it is, as the registry names it, such as * or
# Directory. { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-menu-command-detail =
    Thêm một lệnh vào menu của { $on }, { $scope ->
        [everyone] cho mọi người trên PC này
       *[you] cho bạn
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-default-verb-detail =
    Thay đổi điều xảy ra khi mở một mục thuộc loại này, { $scope ->
        [everyone] cho mọi người trên PC này
       *[you] cho bạn
    }
core-review-client-entry-detail = Có thể được chọn làm chương trình cho việc này. Không có gì được chuyển sang dùng chương trình này
core-review-com-permissions-title = Quyền COM
core-review-com-permissions-detail = Quy định ai có thể khởi động các lớp COM của ứng dụng, hoặc các lớp đó chạy với tư cách gì
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-link-type-detail =
    Các liên kết thuộc loại này được mở bằng ứng dụng, { $scope ->
        [everyone] cho mọi người trên PC này
       *[you] cho bạn
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-file-type-detail =
    Các tệp thuộc loại này được mở bằng ứng dụng, { $scope ->
        [everyone] cho mọi người trên PC này
       *[you] cho bạn
    }
# { $log } is the name of a Windows event log, such as Application.
core-review-event-log-source-detail = Có thể ghi vào nhật ký { $log }, với thông báo lấy từ các tệp riêng của ứng dụng
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-font-detail =
    Được tải từ thư mục cài đặt khi bạn đăng nhập, { $scope ->
        [everyone] cho mọi người trên PC này
       *[you] cho bạn
    }
core-review-odbc-driver-detail = Được tải vào các chương trình dùng ODBC để kết nối cơ sở dữ liệu
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-odbc-source-detail =
    Một kết nối cơ sở dữ liệu, { $scope ->
        [everyone] cho mọi người trên PC này
       *[you] cho bạn
    }
core-review-default-apps-detail = Có thể được chọn trong Ứng dụng mặc định. Không có gì được chuyển sang dùng ứng dụng này
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-program-name-detail =
    Hộp thoại Chạy và các chương trình khác có thể tìm thấy ứng dụng theo tên này, { $scope ->
        [everyone] cho mọi người trên PC này
       *[you] cho bạn
    }
# { $server } is the file name of what serves the classes.
core-review-com-title = COM trong { $server }
core-review-com-title-own-file = COM trong một tệp mà ứng dụng cài đặt
# { $count } is how many registry values.
core-review-com-detail =
    { $count ->
       *[other] { $count } thiết đặt của các lớp mà chương trình khác có thể khởi động
    }
core-review-com-support-title = Thư viện kiểu, giao diện và AppID của COM
# { $libraries }, { $interfaces }, and { $app_ids } are how many registry values there are for its type libraries, its
# interfaces, and its AppIDs.
core-review-com-support-detail = Thiết đặt cho các thư viện kiểu ({ $libraries }), giao diện ({ $interfaces }) và AppID ({ $app_ids }) đi kèm các lớp của ứng dụng
core-review-compatibility-title = Cách các chương trình của ứng dụng khởi động
# { $count } is how many registry values.
core-review-compatibility-detail =
    { $count ->
       *[other] { $count } thiết đặt tương thích cho các chương trình mà ứng dụng cài đặt
    }

## The installer's notes on whose folders and settings these are, and what is not coming.

# { $account } is the name of a Windows account, such as alice.
core-review-account-signed-in = Dữ liệu người dùng và thiết đặt theo người dùng được chuyển cho { $account }, người đang đăng nhập ở đây, chứ không phải cho tài khoản quản trị viên đang chạy chương trình này.
# { $account } is the name of a Windows account, such as alice.
core-review-account-no-desktop = Không có ai đăng nhập vào phiên này, nên dữ liệu người dùng và thiết đặt theo người dùng được chuyển cho { $account }, tài khoản đang chạy chương trình này.
# { $reason } is what Windows said, in its own words.
core-review-account-not-told = Không xác định được ai đang đăng nhập ({ $reason }), nên các thiết đặt theo người dùng bị loại trừ, và dữ liệu người dùng được đặt trong vị trí cài đặt.
# { $reason } is what Windows said, in its own words.
core-review-signed-in-unknown = không xác định được ai đang đăng nhập: { $reason }
# Headings of what is left out, shared by the app's list and the installer's notes.
core-review-left-out-credentials = Không có trong trình cài đặt này, vì trông giống thông tin xác thực
core-review-left-out-settings = Thiết đặt và lịch sử mà một tài khoản lưu cho ứng dụng, chỉ được đưa vào khi có tùy chọn Dữ liệu người dùng
core-review-left-out-certificates = Ứng dụng có vẻ dựa vào các chứng chỉ đã được tin cậy trên PC của nó, và các chứng chỉ đó không được cài đặt
core-review-left-out-fonts = Các phông chữ này mang tên của ứng dụng, nhưng không có gì cho biết chúng thuộc về ứng dụng, nên chúng không được cài đặt
# { $names } is a list of names, such as the names of saved passwords.
core-review-note-credentials = Không có trong trình cài đặt này, vì trông giống thông tin xác thực: { $names }. Những gì của chúng trên PC này vẫn được giữ nguyên, và bạn có thể phải nhập lại chúng.
# { $names } is a list of names of license data.
core-review-note-licenses = Không có trong trình cài đặt này, vì tùy chọn Dữ liệu giấy phép và kích hoạt đã tắt khi trình cài đặt được tạo: { $names }.
# { $names } is a list of places an account keeps settings, such as folders.
core-review-note-settings = Không có trong trình cài đặt này, vì tùy chọn Dữ liệu người dùng đã tắt khi trình cài đặt được tạo: thiết đặt và lịch sử mà một tài khoản lưu cho ứng dụng, trong { $names }. Ứng dụng sẽ khởi động ở đây như với người mới dùng lần đầu.
# { $names } is a list of certificates, each as core-review-certificate.
core-review-note-certificates = Ứng dụng có vẻ dựa vào các chứng chỉ đã được tin cậy trên PC của nó, và các chứng chỉ đó không được cài đặt: { $names }. Việc có tin cậy chúng ở đây hay không là do bạn quyết định.
# { $names } is a list of font names.
core-review-note-fonts = Các phông chữ này mang tên của ứng dụng, nhưng không có gì cho biết chúng thuộc về ứng dụng, nên chúng không được cài đặt: { $names }.
# { $names } is a list of names.
core-review-note-left-out-by-choice = Bị loại trừ bởi người tạo trình cài đặt này: { $names }.
# { $names } is a list of names.
core-review-note-added-by-choice = Được thêm bởi người tạo trình cài đặt này, vì họ cho rằng đây là một phần của ứng dụng: { $names }.
# { $name } is a certificate's subject, such as CN=Contoso, and { $store } the certificate store it was in, such as
# Root.
core-review-certificate = { $name } trong { $store }
# { $names } is a list of names, and { $count } how many more there are.
core-review-names-and-more = { $names } và { $count } mục khác

## Why something in a package is left out. Each is the second half of a sentence of core-warning-review-left-out.

core-review-no-account = mục này thuộc về thiết đặt của một tài khoản, và không xác định được ai đang đăng nhập
core-review-name-not-accepted = Windows không chấp nhận tên của mục này
# { $error } is what Windows said, in its own words.
core-review-service-unreadable = không thể đọc thiết đặt hiện tại của dịch vụ: { $error }
core-review-service-dependency = dịch vụ phụ thuộc vào một dịch vụ có tên không hợp lệ
# { $path } is the program the service runs.
core-review-service-runs-other = dịch vụ chạy { $path }, mà gói này không cài đặt
core-review-service-settings = thiết đặt của dịch vụ tham chiếu thứ mà gói này không cài đặt, hoặc một khóa không thuộc về dịch vụ
core-review-driver-no-hash = trình điều khiển không có hàm băm để kiểm tra
core-review-license-for-the-machine = giấy phép được lưu cho toàn bộ máy chỉ được báo cáo
core-review-license-starts-something = giấy phép không có lý do gì để khởi động bất cứ thứ gì
core-review-task-not-captured = định nghĩa của tác vụ chưa được thu thập
# { $error } is what was wrong with the task's XML, in English.
core-review-task-unreadable = không thể đọc định nghĩa của tác vụ: { $error }
core-review-task-no-account = tác vụ chạy với tư cách người cài đặt, và không xác định được ai đang đăng nhập ở đây
# { $path } is what the shortcut opens.
core-review-shortcut-opens-other = lối tắt mở { $path }, mà gói này không cài đặt
# { $service } is the service's name, and { $path } the program it runs now.
core-policy-service-exists = { $service } đã tồn tại trên PC này và chạy { $path }, không phải từ một lần cài đặt trước đây ở đây. Nếu đó là bản sao cũ hơn của ứng dụng này, hãy gỡ cài đặt bản đó trước
core-policy-key-control-characters = tên khóa của mục này chứa ký tự điều khiển
core-policy-value-control-characters = tên giá trị của mục này chứa ký tự điều khiển
core-policy-registry-link = mục này là một liên kết sổ đăng ký
# { $reason } is why the key it stands for is left out, such as "no app install should change this key".
core-policy-virtual-store = mục này đại diện cho một khóa của PC, và { $reason }
core-policy-registry-part = không lần cài đặt ứng dụng nào được ghi vào phần này của sổ đăng ký
core-policy-key-refused = không lần cài đặt ứng dụng nào được thay đổi khóa này
core-policy-net-debugger = mục này sẽ chỉ định trình gỡ lỗi cho mọi chương trình .NET
core-policy-net-install-root = mục này sẽ thay đổi nơi mọi chương trình .NET tải .NET
core-policy-class-redirects = mục này sẽ thay đổi cách mở một loại chương trình hoặc thư mục
core-policy-app-path-name = mục này đặt tên cho chương trình bằng thứ khác với tên tệp
core-policy-app-path-not-installed = mục này sẽ đăng ký tên chương trình cho một tệp mà gói này không cài đặt
core-policy-shared-default-verb = mục này sẽ thay đổi điều xảy ra khi mở mọi tệp hoặc thư mục thuộc một loại
core-policy-command-runs-other = mục này thêm một lệnh chạy thứ mà gói này không cài đặt
core-policy-autostart-not-command = mục này tự khởi động, và giá trị của nó không phải là lệnh
core-policy-autostart-runs-other = mục này tự khởi động và chạy thứ mà gói này không cài đặt
core-policy-environment-own-rules = biến môi trường được khôi phục theo quy tắc riêng của chúng
core-policy-service-not-restored = mục này thuộc về một dịch vụ mà gói này không khôi phục
core-policy-service-own-rules = thiết đặt riêng của dịch vụ chỉ được khôi phục theo quy tắc của dịch vụ
core-policy-service-dll = ServiceDll của mục này không phải là tệp mà gói này cài đặt
core-policy-event-log-source-only = chỉ có thể khôi phục các giá trị riêng của nguồn nhật ký sự kiện
core-policy-security-log = không ứng dụng nào ghi vào nhật ký Bảo mật
core-policy-event-log-messages = thông báo của mục này sẽ đến từ một tệp mà gói này không cài đặt
core-policy-type-library = thư viện kiểu của mục này không phải là tệp mà gói này cài đặt
# { $name } is a program's file name, such as app.exe.
core-policy-application-opened = mục này sẽ thay đổi cách mở { $name }, mà gói này không cài đặt
core-policy-event-log-no-file = nguồn nhật ký sự kiện phải lấy thông báo từ một tệp
core-policy-com-runs-other = mục này chạy thứ không phải là tệp mà gói này cài đặt
core-policy-com-redirect = mục này sẽ biến lớp này thành thứ mà gói này không cài đặt
core-policy-com-server = mục này chạy thứ không phải là tệp mà gói này cài đặt, cũng không phải là một phần của Windows
core-policy-interface = mục này sẽ quyết định cách thực hiện lệnh gọi tới một giao diện, bằng thứ mà gói này không cài đặt
# { $name } is a program's file name, such as app.exe.
core-policy-app-id-started = mục này sẽ thay đổi cách khởi động { $name }, mà gói này không cài đặt
core-policy-app-id-surrogate = mục này sẽ được chạy bên trong một chương trình chủ mà gói này không cài đặt
core-policy-app-id-service = mục này sẽ chạy trong một dịch vụ mà gói này không khôi phục
core-policy-app-id-run-as = mục này sẽ chạy dưới một tài khoản riêng
core-policy-app-id-link = mục này không chỉ định AppID nào
core-policy-app-id-remote = mục này sẽ chạy ở nơi khác ngoài PC này
core-policy-odbc-own-driver = đó là thiết đặt riêng của ODBC, không phải trình điều khiển
core-policy-odbc-driver-not-installed = trình điều khiển của mục này không phải là tệp mà gói này cài đặt
core-policy-loads-other-file = mục này sẽ tải một tệp mà gói này không cài đặt
core-policy-odbc-own-source = đó là thiết đặt riêng của ODBC, không phải nguồn dữ liệu
core-policy-odbc-source-missing = mục này liệt kê một nguồn dữ liệu mà gói này không có
core-policy-odbc-source-driver = trình điều khiển của mục này không phải là tệp mà gói này cài đặt, cũng không phải là một phần của Windows
core-policy-shell-extension = mục này sẽ tải vào Chương trình Quản lý Tệp một phần mở rộng shell không phải là tệp mà gói này cài đặt
core-policy-not-text = nội dung của mục này không phải là văn bản như cách nó sẽ được đọc
core-policy-network-place = mục này trỏ tới một vị trí trên mạng
core-policy-font = mục này sẽ đăng ký một phông chữ mà gói này không cài đặt, hoặc một phông chữ trong thư mục Windows
core-policy-compatibility = mục này sẽ thay đổi cách chạy của một chương trình mà gói này không cài đặt
core-policy-capabilities = mục này liệt kê một ứng dụng mà thông tin chi tiết không có trong gói này
core-policy-default-program = mục này sẽ thay đổi chương trình mặc định dùng để mở các mục
core-policy-url-protocol = mục này tự nhận là một loại liên kết, và không mở bằng thứ gì mà gói này cài đặt
core-policy-command-handler = mục này thêm một lệnh được xử lý bởi thứ mà gói này không cài đặt
core-policy-installer-command = mục này sẽ chạy thứ mà chỉ Windows Installer mới đọc được
core-policy-creates-class = mục này sẽ tạo một lớp mà gói này không cài đặt
core-policy-opens-links = mục này mở liên kết bằng thứ mà gói này không cài đặt
core-policy-opens-files = mục này mở tệp bằng thứ mà gói này không cài đặt
core-policy-firewall-name = tên của quy tắc này không phải là tên có thể chuyển giao
core-policy-firewall-description = mô tả của quy tắc này không phải là mô tả có thể chuyển giao
core-policy-firewall-every-program = quy tắc này áp dụng cho mọi chương trình, trong khi một quy tắc chỉ có thể dành cho một chương trình
# { $path } is the program the rule is for.
core-policy-firewall-program = quy tắc này dành cho { $path }, mà gói này không cài đặt
core-policy-firewall-direction = quy tắc này không chỉ rõ một hướng có thể đặt
core-policy-firewall-action = quy tắc này sẽ ghi đè các quy tắc khác, điều mà không quy tắc nào của ứng dụng được phép làm
# { $setting } is a part of a firewall rule, as the scan of its PC described it.
core-policy-firewall-not-carried = quy tắc này có { $setting }, vốn không thể đặt đúng như ban đầu, hoặc sẽ làm suy yếu tác dụng của các quy tắc khác
core-policy-firewall-service = quy tắc này dành cho một dịch vụ mà gói này không khôi phục
core-policy-firewall-parts = một phần của quy tắc này không phải là điều mà quy tắc tường lửa có thể biểu thị
core-policy-folder-already-there = thư mục đó đã có sẵn, nằm ngoài vị trí cài đặt, và không có gì cho biết nó thuộc về ứng dụng này, nên không có chương trình nào được đặt vào đó
core-policy-variable-name = tên của biến này không phải là tên biến hợp lệ
core-policy-variable-relied-on = Windows và các chương trình khác phụ thuộc vào biến này
core-policy-task-name = tên của tác vụ không phải là tên mà Windows chấp nhận
core-policy-task-folder = thư mục của tác vụ không phải là thư mục mà Windows chấp nhận
core-policy-task-windows = tác vụ này sẽ thay thế một trong các tác vụ riêng của Windows
core-policy-task-other-action = tác vụ này làm việc khác ngoài việc chạy một chương trình
core-policy-task-no-program = tác vụ này không chỉ định chương trình nào để chạy
# { $path } is the program the task runs.
core-policy-task-runs-other = tác vụ này chạy thứ mà gói này không cài đặt: { $path }
core-policy-task-exists-unreadable = đã có một tác vụ cùng tên và không thể đọc tác vụ đó
core-policy-task-exists-other = đã có một tác vụ cùng tên và tác vụ đó không chạy ứng dụng
# { $path } is the program the task already on this PC runs.
core-policy-task-exists-runs = đã có một tác vụ cùng tên và tác vụ đó chạy { $path }, không phải từ một lần cài đặt trước đây ở đây
core-policy-shortcut-not-lnk = đây không phải là lối tắt .lnk
core-policy-shortcut-place = lối tắt này không nằm trong menu Bắt đầu hoặc trên màn hình nền
core-policy-short-name = đường dẫn của mục này dùng một tên không thể kiểm tra
core-policy-shortcut-no-folder = PC này không có thư mục như vậy cho lối tắt này
core-policy-data-startup = mục này sẽ được đặt vào thư mục Khởi động và chạy mỗi lần đăng nhập
core-policy-data-powershell = mục này sẽ được đặt vào nơi PowerShell chạy những gì nó tìm thấy khi khởi động
core-policy-data-windows = mục này sẽ được đặt vào một thư mục thuộc về Windows
core-policy-license-outside = mục này nằm ngoài vị trí cài đặt và mọi thư mục dữ liệu đã biết
core-rewrite-cdata = mục này chứa một phần CDATA
core-rewrite-too-much-to-look-through = gói này chứa quá nhiều thiết đặt để có thể kiểm tra những gì chúng tham chiếu

## Warnings a review gives, which the installer shows with what it sets up.

# { $kind } says what was left out, and is not shown itself. { $item } is its path or name, or a registry value such
# as HKLM\Software\App\Name. { $reason } is why, as the second half of the sentence, such as "it cannot be a whole
# drive".
core-warning-review-left-out =
    { $kind ->
        [file] Tệp { $item } đã bị loại trừ: { $reason }.
        [folder] Thư mục { $item } đã bị loại trừ: { $reason }.
        [service] Dịch vụ { $item } đã bị loại trừ: { $reason }.
        [driver] Trình điều khiển { $item } đã bị loại trừ: { $reason }.
        [firewall-rule] Quy tắc tường lửa { $item } đã bị loại trừ: { $reason }.
        [license-data] Dữ liệu giấy phép tại { $item } đã bị loại trừ: { $reason }.
        [license-value] Giá trị giấy phép { $item } đã bị loại trừ: { $reason }.
        [scheduled-task] Tác vụ đã lên lịch { $item } đã bị loại trừ: { $reason }.
        [shortcut] Lối tắt { $item } đã bị loại trừ: { $reason }.
        [environment-variable] Biến môi trường { $item } đã bị loại trừ: { $reason }.
       *[value] { $item } đã bị loại trừ: { $reason }.
    }
core-warning-review-package-path-missing = Một tệp có thể sao chép không có đường dẫn tương đối trong gói.
# { $count } is the most empty folders a package makes.
core-warning-review-too-many-empty-folders = Gói này liệt kê hơn { $count } thư mục trống, và chỉ chừng đó thư mục được tạo.
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Documents folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-documents-elsewhere =
    { $count ->
       *[other] { $count } tệp từ Tài liệu được đặt trong { $target } thay vào đó, trong một thư mục tên là _external, vì thư mục Tài liệu của PC này nằm tại { $folder }, và chỉ thư mục bên trong hồ sơ người dùng của bạn mới được ghi vào. Để sử dụng các tệp này, hãy tự di chuyển chúng vào thư mục Tài liệu của bạn.
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-documents-unknown =
    { $count ->
       *[other] { $count } tệp từ Tài liệu được đặt trong { $target } thay vào đó, trong một thư mục tên là _external, vì không xác định được thư mục Tài liệu của PC này nằm ở đâu. Để sử dụng các tệp này, hãy tự di chuyển chúng vào thư mục Tài liệu của bạn.
    }
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Saved Games folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-elsewhere =
    { $count ->
       *[other] { $count } tệp từ Trò chơi đã lưu được đặt trong { $target } thay vào đó, trong một thư mục tên là _external, vì thư mục Trò chơi đã lưu của PC này nằm tại { $folder }, và chỉ thư mục bên trong hồ sơ người dùng của bạn mới được ghi vào. Để sử dụng các tệp này, hãy tự di chuyển chúng vào thư mục Trò chơi đã lưu của bạn.
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-unknown =
    { $count ->
       *[other] { $count } tệp từ Trò chơi đã lưu được đặt trong { $target } thay vào đó, trong một thư mục tên là _external, vì không xác định được thư mục Trò chơi đã lưu của PC này nằm ở đâu. Để sử dụng các tệp này, hãy tự di chuyển chúng vào thư mục Trò chơi đã lưu của bạn.
    }

## Errors when a folder that only administrators may change cannot be made so, or checked.

# { $path } is a folder.
core-privilege-not-a-folder = { $path } là liên kết hoặc tệp, không phải thư mục
# { $path } is a folder. { $error } is what Windows said, in its own words.
core-privilege-not-made-here = { $path } không do WinAppMigrator tạo, và người khác có thể thay đổi nội dung bên trong. Quản trị viên nên kiểm tra và xóa thư mục này: { $error }
# { $path } is a folder.
core-privilege-others-put-something-in = { $path } chứa nội dung không do WinAppMigrator tạo, và người khác có thể thay đổi nội dung đó. Quản trị viên nên kiểm tra và xóa thư mục này
# { $path } is a folder.
core-privilege-became-a-link-in-use = { $path } đã trở thành liên kết trong khi đang được sử dụng
# { $path } is a folder.
core-privilege-became-a-link-looked-at = { $path } đã trở thành liên kết trong khi đang được kiểm tra
# { $path } is a file. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-users-cannot-start = không thể cho phép người dùng khởi động { $path }: { $said }
core-privilege-elevation-unknown = không xác định được tiến trình này đã được nâng quyền hay chưa
# { $path } is a folder. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-cannot-restrict = không thể giới hạn { $path } chỉ cho quản trị viên: { $said }
# { $path } is a folder. { $said } is the folder's permissions as Windows wrote them, and what else it printed.
core-privilege-not-restricted = không thể giới hạn { $path } chỉ cho quản trị viên, và ổ đĩa chứa thư mục này có thể không lưu giữ quyền: { $said }
# { $path } is a folder.
core-privilege-written-to-first = một tiến trình khác đã ghi vào { $path } trước khi thư mục này có thể được bảo vệ
core-privilege-no-program-data = Windows không cho biết ProgramData nằm ở đâu

## Checks of a package's files, and of what an install put in place. Each is said of one file, value, service, or the
## like, which the report names beside it.

# { $path } is the installer's file.
core-verify-no-package = { $path } không chứa gói nào
# { $path } is a folder.
core-verify-not-unpacked = gói của trình cài đặt này chưa được giải nén vào { $path }
core-verify-schema-supported = Lược đồ tệp kê khai của gói được hỗ trợ.
core-verify-packaged-file-missing = Thiếu tệp trong gói.
core-verify-no-package-path = Tệp có thể sao chép không có đường dẫn tương đối trong gói.
core-verify-folder-there = Thư mục trống có tồn tại.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-unreadable = Không thể đọc mục này để kiểm tra: { $error }
core-verify-restored-file-missing = Thiếu tệp đã khôi phục.
# { $expected } and { $actual } are sizes in bytes.
core-verify-size-mismatch = Kích thước tệp không khớp. Dự kiến { $expected } byte, thực tế { $actual } byte.
core-verify-no-hash = Tệp kê khai không ghi hàm băm cho tệp này, nên không thể kiểm tra tệp.
core-verify-hash-mismatch = Hàm băm SHA-256 không khớp.
core-verify-file-matches = Kích thước tệp và hàm băm khớp với tệp kê khai.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-payload-failed = Xác minh dữ liệu giấy phép không thành công: { $error }
core-verify-license-payload-missing = Thiếu dữ liệu giấy phép trong gói.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-failed = Xác minh kho giấy phép không thành công: { $error }
core-verify-license-store-file-missing = Thiếu tệp kho giấy phép trong gói.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-path-invalid = Đường dẫn kho giấy phép trong gói không hợp lệ: { $error }
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-prerequisite-failed = Xác minh dữ liệu thành phần tiên quyết không thành công: { $error }
core-verify-prerequisite-missing = Thiếu dữ liệu thành phần tiên quyết đi kèm.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-failed = Xác minh tệp trình điều khiển không thành công: { $error }
core-verify-bundled-driver-missing = Thiếu tệp trình điều khiển đi kèm.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-path-invalid = Đường dẫn trình điều khiển đi kèm không hợp lệ: { $error }
core-verify-prerequisite-present = Thành phần tiên quyết thời gian chạy có trên máy đích.
core-verify-prerequisite-not-detected = Không phát hiện thành phần tiên quyết thời gian chạy; có thể cần khởi động lại hoặc hoàn tất thủ công (xem nhật ký khôi phục để biết kết quả chính xác).
core-verify-prerequisite-not-probed = Không thể dò sự có mặt của thành phần tiên quyết qua sổ đăng ký; việc cài đặt được thực hiện theo khả năng tốt nhất.
core-verify-license-file-missing = Thiếu tệp giấy phép đã khôi phục.
core-verify-license-store-restored-missing = Thiếu tệp kho giấy phép đã khôi phục.
core-verify-registry-key-there = Khóa sổ đăng ký có tồn tại.
core-verify-registry-value-matches = Giá trị sổ đăng ký khớp với dữ liệu đã lưu trữ.
core-verify-registry-value-differs = Giá trị sổ đăng ký không khớp với dữ liệu đã lưu trữ.
core-verify-firewall-rule-there = Tường lửa Windows có quy tắc này.
core-verify-variable-matches = Giá trị biến môi trường khớp với dữ liệu đã lưu trữ.
core-verify-variable-differs = Giá trị biến môi trường không khớp với dữ liệu đã lưu trữ.
# { $error } is what Windows said, in its own words.
core-verify-variable-unreadable = Biến môi trường bị thiếu hoặc không đọc được: { $error }
core-verify-service-runs-other = Dịch vụ có tồn tại nhưng không chạy tệp nhị phân đã khôi phục với các đối số của nó.
core-verify-service-other-account = Dịch vụ có tồn tại nhưng không chạy dưới tài khoản dự kiến.
core-verify-service-matches = Dịch vụ có tồn tại và chạy tệp nhị phân đã khôi phục với đúng đối số và tài khoản.
core-verify-service-missing = Thiếu dịch vụ.
# { $error } is what Windows said, in its own words.
core-verify-service-unreadable = Không thể đọc dịch vụ: { $error }
core-verify-driver-other-image = Dịch vụ trình điều khiển không trỏ tới tệp đã khôi phục.
core-verify-driver-not-registered = Dịch vụ trình điều khiển đi kèm chưa được đăng ký sau khi khôi phục.
# { $error } is what Windows said, in its own words.
core-verify-driver-unreadable = Không thể đọc dịch vụ trình điều khiển: { $error }
core-verify-driver-image-missing = Thiếu tệp trình điều khiển.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-unchecked = Không thể kiểm tra tệp trình điều khiển: { $error }
core-verify-task-matches = Tác vụ đã lên lịch có tồn tại và chạy các chương trình đã khôi phục.
core-verify-task-runs-other = Tác vụ đã lên lịch có tồn tại nhưng không chạy các chương trình đã khôi phục.
core-verify-task-missing = Thiếu tác vụ đã lên lịch.
# { $error } is what Windows said, in its own words.
core-verify-task-unreadable = Truy vấn tác vụ đã lên lịch không thành công: { $error }
core-verify-shortcut-matches = Lối tắt có tồn tại và đường dẫn đích khớp với đích đã khôi phục.
core-verify-shortcut-opens-other = Lối tắt có tồn tại nhưng đường dẫn đích không khớp với đích đã khôi phục.
core-verify-shortcut-missing = Thiếu lối tắt.
# { $error } is what Windows said, in its own words.
core-verify-shortcut-unreadable = Truy vấn lối tắt không thành công: { $error }

## Warnings and errors a check gives, which the app and the installer show.

core-warning-verify-schema-unsupported = Lược đồ tệp kê khai của gói không được hỗ trợ.
# { $error } is what went wrong, in Windows' or the program's own words.
core-warning-verify-package-file-failed = Xác minh tệp trong gói không thành công: { $error }
core-warning-verify-folder-missing = Thiếu thư mục trống.
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-key-missing = Khóa sổ đăng ký bị thiếu hoặc không đọc được: { $error }
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-value-missing = Giá trị sổ đăng ký bị thiếu hoặc không đọc được: { $error }
core-warning-verify-registry-kind-missing = Thiếu loại của giá trị sổ đăng ký.
core-warning-verify-firewall-rule-missing = Tường lửa Windows không có quy tắc này.
core-warning-verify-service-scope = Việc xác minh dịch vụ chỉ đọc cấu hình dịch vụ trong HKLM và không khởi động dịch vụ.
core-warning-verify-driver-manual = WinAppMigrator không phân phối lại một hoặc nhiều trình điều khiển của nhà cung cấp hoặc có sẵn trong Windows; hãy cài đặt lại chúng bằng gói trình điều khiển đã ký của nhà cung cấp trên máy đích.
core-warning-verify-task-scope = Việc xác minh tác vụ đã lên lịch chỉ kiểm tra việc đăng ký và lệnh được tham chiếu.

## The verification report written beside a package, verification_report.md. Its headings follow a "#".

core-verify-report-title = Báo cáo xác minh WinAppMigrator
# { $time } is when, as a date and time in UTC, such as 2026-11-15T10:00:00Z.
core-verify-report-verified = Thời điểm xác minh (UTC): { $time }
# { $valid } is true or false, which are not translated.
core-verify-report-package-valid = Gói hợp lệ: { $valid }
# { $valid } is true or false, which are not translated.
core-verify-report-restore-valid = Khôi phục hợp lệ: { $valid }
core-verify-report-restore-not-run = Khôi phục hợp lệ: chưa chạy
core-verify-report-items = Mục
# { $message } is what a check found, and { $source } the file or setting it looked at.
core-verify-report-item-from = { $message } Nguồn: { $source }
core-verify-report-warnings = Cảnh báo
core-verify-report-errors = Lỗi
core-verify-report-none = Không có.
