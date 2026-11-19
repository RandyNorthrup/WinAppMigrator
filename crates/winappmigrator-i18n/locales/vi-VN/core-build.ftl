# Words the core writes for people while it builds a package from an installed app: what a package records for an
# installer to show later, errors shown in the app, the list of what goes in and what is left out, opening a disk
# image or another PC over the network, and what an installer says when the package it carries cannot be read.
# Message names start with core-.


## Scanning an app, before a package is built. Shown in the app when a scan cannot go on.

# { $path } is the app's program, such as D:\Program Files\Acme\Acme.exe, on a drive taken from another PC.
core-scan-program-not-on-the-drive = { $path } đi qua một liên kết hoặc không nằm trên ổ đĩa đã kết nối, nên không được đọc.
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-not-on-this-pc = { $path } không nằm trên đĩa của PC này, hoặc có tên sẽ mở một thứ khác, nên không được đọc.
# { $path } is the app's program.
core-scan-program-someone-elses = { $path } nằm trong thư mục của tài khoản khác hoặc không thuộc tài khoản này, và bản ghi của chính ứng dụng này không cho biết tệp đó thuộc về nơi đó, nên không được đọc.
# { $app_name } is the app's name, as the list of apps calls it.
core-scan-nothing-to-package = Không tìm thấy chương trình riêng nào của { $app_name }, nên không có gì để đưa vào trình cài đặt. Trình điều khiển, thư viện thời gian chạy và phần bổ trợ thường như vậy: trên PC mới, hãy cài đặt trình điều khiển từ nhà sản xuất hoặc Windows Update, và cài đặt thư viện thời gian chạy hoặc phần bổ trợ cùng với ứng dụng sử dụng nó.
# { $app_name } is the app's name, as the list of apps calls it. A web app is a website installed as an app, with
# a window and a Start menu entry of its own.
core-scan-web-app = { $app_name } là một ứng dụng web mà trình duyệt đã cài đặt từ một trang web. Ứng dụng này không có chương trình riêng, nên không có gì để đưa vào trình cài đặt. Trên PC mới, hãy mở trang web đó trong cùng trình duyệt và cài đặt ứng dụng từ đó.
# { $path } is the program the app's own record names, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-missing = { $path } không còn ở đó nữa, dù bản ghi của ứng dụng này ghi nó là chương trình của ứng dụng. Ứng dụng có thể đã bị di chuyển hoặc bị gỡ một phần: sửa chữa hoặc cài đặt lại ứng dụng trên PC này sẽ đưa nó trở lại.
# { $path } is the file that was picked.
core-scan-not-a-file = đích quét không phải là tệp: { $path }
# Names something left out of a package, in a list of what was left out and why. { $name } is the variable's name,
# such as ACME_LICENSE_KEY.
core-scan-environment-variable-item = Biến môi trường { $name }

## Folders and parts that might also be the app's, offered in a list to tick. Each says why it is offered.

core-near-starts-with-app-name = Tên của mục này bắt đầu bằng tên của ứng dụng này.
core-near-has-app-name = Tên của mục này có chứa tên của ứng dụng này.
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-starts-with-publisher = Tên của mục này bắt đầu bằng tên nhà phát hành, { $publisher }.
# { $folder } is a folder's name, such as Program Files.
core-near-hidden-part-beside = Một phần bị ẩn khỏi danh sách ứng dụng, được cài đặt bên cạnh ứng dụng này trong { $folder }.
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-hidden-part-publisher = Một phần bị ẩn khỏi danh sách ứng dụng, của cùng nhà phát hành, { $publisher }.

## What a scan found, as a person chooses from it: the folders that go in, and what is left out, and why

# Why a folder cannot be left out, beside its number of files.
core-footprint-locked-app-folder = Những mục này luôn đi kèm chương trình chính
core-footprint-locked-main-program = Chứa chương trình chính, luôn được đưa vào
core-footprint-locked-needed = Chứa những gì chương trình chính cần để khởi động
core-footprint-locked-font = Đi kèm phông chữ của nó, trong mục Những gì gắn ứng dụng với Windows
# Where a folder is. "AppData" and "ProgramData" are the names of Windows folders; keep them as they are.
core-footprint-place-roaming = Thiết đặt trong hồ sơ của bạn (AppData)
core-footprint-place-local = Thiết đặt cho PC này trong hồ sơ của bạn (AppData)
core-footprint-place-program-data = Dùng chung cho mọi người trên PC (ProgramData)
core-footprint-place-documents = Trong Tài liệu của bạn
core-footprint-place-saved-games = Trong Trò chơi đã lưu của bạn
core-footprint-place-public-documents = Trong Tài liệu công cộng
core-footprint-place-app-folder = Trong thư mục của ứng dụng
# { $folder } is a folder, such as C:\Users\Sam\AppData\Roaming.
core-footprint-place-in = Trong { $folder }
# A row for the files directly in the app's folder. { $folder } is that folder's name, such as Acme.
core-footprint-files-directly-in = Các tệp nằm trực tiếp trong { $folder }
# Headings over what is left out, and why each item is.
core-footprint-licenses-left-out = Không có trong trình cài đặt này, vì tùy chọn Dữ liệu giấy phép và kích hoạt đang tắt
core-footprint-font-stays-behind = Mục này không được mang theo.
core-footprint-drivers-left-out = Các trình điều khiển thuộc về Windows hoặc nhà cung cấp, không thể chuyển giao, nên hãy cài đặt lại chúng trên PC kia
core-footprint-driver-reinstall = Hãy cài đặt lại trình điều khiển này từ gói của chính nhà cung cấp.
# Why something else is left out along with what the person left out. { $service } is a service's name, such as AcmeSync.
core-footprint-needs-left-out = mục này cần { $service }, vốn đã bị loại trừ
core-footprint-told-to-use-left-out = mục này được thiết lập để dùng một thứ trong thư mục bạn đã loại trừ
core-footprint-file-in-left-out = tệp của mục này nằm trong thư mục bạn đã loại trừ
core-footprint-program-in-left-out = chương trình mà mục này dành cho nằm trong thư mục bạn đã loại trừ
core-footprint-names-left-out = mục này tham chiếu một thư mục bạn đã loại trừ
# Why something is left out, as the installer tells whoever installs it.
core-footprint-builder-left-it-out = người tạo trình cài đặt này đã loại trừ mục này
# Something the person left out, by kind and name, such as "Service Acme Updater". { $kind } is the kind, such as
# Service or Scheduled Task; { $title } is its name.
core-footprint-left-out-item = { $kind } { $title }

## Building a package and an installer from a scan. Shown in the app when a build cannot go on.

core-package-scan-of-another-app = lần quét này là của một ứng dụng khác với ứng dụng đang được tạo trình cài đặt, nên không có gì được tạo
core-package-scan-brought-in-more = lần quét này đưa vào dữ liệu mà lần tạo này không yêu cầu, nên không có gì được tạo
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-package-program-gone = { $path } không còn ở đó nữa, nên không có gì được tạo. Hãy xem xét lại ứng dụng.
core-package-needs-stub = kho lưu trữ tự giải nén cần phần khởi chạy (stub) của trình cài đặt
# { $reason } is what went wrong.
core-package-staged-file-failed = một tệp trong thư mục trung gian không thể được đưa vào gói: { $reason }
# { $shortfalls } says which drives are short of room and by how much, in sentences of its own.
core-package-not-enough-room = không đủ dung lượng để tạo trình cài đặt này, nên không có gì được đọc hoặc ghi. { $shortfalls }. Hãy giải phóng dung lượng ở đó, hoặc chọn thư mục đầu ra trên ổ đĩa khác.
# { $path } is a file's path, on a drive taken from another PC.
core-package-not-read-off-the-drive = { $path } đi qua một liên kết hoặc không nằm trên ổ đĩa đã kết nối, nên không được đọc
# { $path } is a file's path.
core-package-not-read-not-local = { $path } không nằm trên đĩa của PC này, nên không được đọc
# { $path } is a file's path.
core-package-not-read-other-account = { $path } dẫn vào thư mục của tài khoản khác, nên không được đọc
# { $path } is a file's path, as the PC it came from called it.
core-package-not-on-the-drive = { $path } không nằm trên ổ đĩa đã kết nối, nên không được đọc
# { $path } is a file's path.
core-package-backup-copy-no-parent = không thể sao chép dự phòng một tệp không có thư mục cha: { $path }
# { $path } is a file's path.
core-package-backup-copy-destination-no-parent = đích sao chép dự phòng không có thư mục cha: { $path }
# robocopy is the name of a Windows command; keep it as it is. { $code } is how it ended, such as Some(8).
core-package-robocopy-failed = sao chép ở chế độ sao lưu bằng robocopy không thành công (mã thoát { $code })
# Stands for a folder where a file's name would be, as in "I/O failed for temporary package folder: …".
core-package-temporary-folder = thư mục gói tạm thời
# Stands for the installer being written where a file's name would be, as in "I/O failed for the installer: …".
core-package-the-installer = trình cài đặt
# { $path } is a file's path, or its name in the package. { $error } is what went wrong with it.
core-package-path-error = { $path }: { $error }
# { $name } is a file's name in the package, such as app/bin/App.exe.
core-package-same-place = một tệp khác có cùng vị trí trong gói: { $name }
# { $name } is a file's name in the package.
core-package-changed-in-protected-folder = { $name } đã thay đổi trong thư mục được bảo vệ trong khi gói đang được tạo
core-package-worker-died = một luồng nén tệp đã dừng đột ngột, nên không có trình cài đặt nào được ghi
# { $path } is a file's path.
core-package-link-not-read = { $path } là liên kết, nên không được đọc
# { $path } is a file's path. { $error } is what went wrong.
core-package-not-compressed = không thể nén { $path }: { $error }
# { $path } is a file's path. { $reason } is what went wrong.
core-package-not-read-back = không thể đọc lại { $path }: { $reason }

## Where an app's files are, and where an installer may write them. Shown when a path is refused.

# { $path } is a folder, such as C:\Program Files\Acme.
core-discovery-root-not-full-path = thư mục để ghi vào phải là đường dẫn đầy đủ: { $path }
# { $path } is a folder's path.
core-discovery-root-not-a-folder = thư mục để ghi vào không phải là thư mục: { $path }
# { $path } is a file's path.
core-discovery-escapes-root = đường dẫn vượt ra ngoài thư mục gốc dự kiến: { $path }
# { $path } is the link's path.
core-discovery-refusing-to-write-through-link = từ chối ghi qua liên kết: { $path }
# { $path } is the link's path.
core-discovery-refusing-link = từ chối đi qua liên kết: { $path }
# { $error } is what Windows said went wrong.
core-discovery-refusing-link-because = từ chối đi qua liên kết: { $error }
# { $path } is the link's path.
core-discovery-link-to-somewhere-else = { $path } là liên kết tới nơi khác, và người khác có thể thay đổi nơi liên kết trỏ tới. Hãy dùng chính nơi mà liên kết trỏ tới
# { $path } is a folder's path.
core-discovery-no-parent-folder = { $path } không có thư mục cha
# { $path } is the install location as it was given. Keep the quotes, or use your language's own.
core-discovery-install-location-not-full-path = vị trí cài đặt phải là đường dẫn đầy đủ: “{ $path }”
# { $path } is the install location.
core-discovery-install-location-a-link = vị trí cài đặt là liên kết tới nơi khác: { $path }
# { $path } is a file's path.
core-discovery-device-name = đường dẫn dùng tên dành riêng cho thiết bị Windows: { $path }
# { $path } is a file's path.
core-discovery-device-name-on-older-windows = { $path } có tên mà phiên bản Windows này mở như một thiết bị chứ không phải tệp, nên không có gì được ghi vào đó. Có thể cài đặt tệp này trên Windows 11
# { $path } is a file's path.
core-discovery-unsafe-path = đường dẫn không an toàn: { $path }
# { $path } is a file's path.
core-discovery-no-package-path = tệp không có đường dẫn tương đối trong gói: { $path }

## Writing an installer, and reading the package inside one. Most of these are shown by an installer, on the PC it
## runs on, when it cannot read what it carries. "The package" is the app's files and settings that an installer
## carries; "the stub" is the program part of an installer, before the package; "the data file" is the .wamdata file
## that holds the package of a very large installer, beside it.

# Stands for the package where a file's name would be, as in "I/O failed for installer package: …".
core-archive-installer-package = gói của trình cài đặt
# Stands for the installer program built into WinAppMigrator, where a file's name would be.
core-archive-embedded-stub = phần khởi chạy (stub) của trình cài đặt được nhúng
# { $index } is the number of a file in the package. { $reason } is what went wrong.
core-archive-entry-unreadable = không thể đọc mục { $index } của gói: { $reason }
# { $name } is a file's name inside the package, such as app/bin/App.exe.
core-archive-entry-unsafe-path = mục zip có đường dẫn không an toàn: { $name }
# { $name } is a file's name inside the package.
core-archive-holds-more = { $name } chứa nhiều dữ liệu hơn mức nó khai báo
# { $name } is a file's name inside the package.
core-archive-name-too-long = tên này quá dài đối với tệp zip: { $name }
core-archive-changed-while-copied = gói đã thay đổi trong khi đang được ghi tiếp sau phần khởi chạy, nên không có trình cài đặt nào được ghi
core-archive-stub-not-a-program = phần đứng trước gói trong trình cài đặt này không phải là chương trình
core-archive-damaged = trình cài đặt này bị hỏng hoặc đã bị thay đổi sau khi được tạo, nên không có gì được đọc từ gói của nó. Hãy sao chép lại trình cài đặt từ nơi nó được tạo.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-missing = trình cài đặt này lưu gói của nó trong một tệp tên là { $name }, trong cùng thư mục, và tệp đó không có ở đây. Hai tệp này phải luôn đi cùng nhau: hãy sao chép { $name } vào đây từ nơi trình cài đặt được tạo, rồi chạy lại.
# { $name } is the data file's name, such as App_installer.wamdata. { $error } is what Windows said went wrong.
core-archive-data-file-unopened = không thể mở tệp dữ liệu của trình cài đặt này, { $name }: { $error }. Hai tệp này phải luôn đi cùng nhau, trong một thư mục mà PC này có thể đọc.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-a-link = { $name } là liên kết hoặc thư mục, trong khi tệp dữ liệu của trình cài đặt này phải là một tệp thực sự, nằm ngay bên cạnh, nên không có gì được đọc từ đó. Hãy sao chép tệp thật vào đây.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-wrong = { $name } không phải là tệp dữ liệu của trình cài đặt này, hoặc tệp bị hỏng hay đã bị thay đổi sau khi được tạo, nên không có gì được đọc từ đó. Hãy sao chép lại cả hai tệp từ nơi chúng được tạo.
core-archive-stub-signed = phần khởi chạy của trình cài đặt đã được ký; phần này phải để ở trạng thái chưa ký, vì vậy hãy ký trình cài đặt hoàn chỉnh thay vào đó
# { $size } is a size, such as 4.2 GB.
core-archive-too-big-to-run = trình cài đặt sẽ có kích thước { $size }, và Windows không thể chạy tệp thực thi từ 4 GB trở lên; không có gì được ghi
core-archive-outside-signature = trình cài đặt có dữ liệu nằm ngoài chữ ký và có thể đã bị can thiệp
core-archive-footer-length = độ dài phần cuối của kho lưu trữ tự giải nén không hợp lệ
core-archive-seek-outside = di chuyển vị trí đọc ra ngoài gói
# { $path } is a file's path.
core-archive-unsafe-package-path = đường dẫn gói không an toàn: { $path }
# { $path } is a file's path.
core-archive-empty-package-path = đường dẫn gói trống: { $path }

## Opening a disk image of another PC (a .vhd or .vhdx file) to read apps from it

# { $path } is the disk image file, such as D:\Backups\OldPC.vhdx.
core-image-not-attachable = { $path } không phải là ảnh đĩa mà Windows có thể gắn (.vhd hoặc .vhdx)
core-image-needs-administrator = việc gắn ảnh đĩa cần quyền quản trị viên
# { $path } is the disk image file the person picked.
core-image-not-a-file = { $path } không phải là tệp
# { $path } is the disk image file. "Edit Disk" is the name of a Hyper-V Manager command.
core-image-differencing = { $path } là đĩa khác biệt, chỉ chứa những gì đã thay đổi kể từ đĩa gốc của nó, và Windows cũng sẽ mở đĩa gốc đó, ở bất cứ nơi nào đĩa khác biệt cho biết. Trước tiên, hãy hợp nhất đĩa này vào đĩa gốc bằng Chỉnh sửa đĩa của Hyper-V, rồi chọn đĩa gốc
# { $path } is the disk image file.
core-image-unreadable = { $path } không phải là tệp .vhd hoặc .vhdx có thể đọc được
# ProgramData is the name of a Windows folder; keep it as it is.
core-image-no-program-data = không tìm thấy thư mục ProgramData của PC này
# The reason given in core-image-could-not-attach when Windows already has the image open.
core-image-already-attached = Windows đã gắn ảnh đĩa này, và thứ đã gắn nó có thể đang ghi vào đó. Hãy tháo ảnh đĩa trước, hoặc chọn ổ đĩa của nó làm ổ đĩa đã kết nối
# { $path } is the disk image file. { $reason } is why, such as core-image-already-attached or what Windows said.
core-image-could-not-attach = không thể gắn { $path }: { $reason }
# { $path } is the disk image file.
core-image-no-windows-volume = { $path } không có ổ đĩa nào chứa Windows có thể đọc được. Ổ đĩa được mã hóa bằng BitLocker phải được mở khóa trước.
# Given as the reason when Windows failed without saying anything.
core-image-windows-did-not-say = Windows không cho biết lý do

## Reading another PC over the network, through the client running on it. Shown as why something on that PC
## could not be read. "The client" is WinAppMigrator Client, the small program on the other PC.

core-remote-too-slow = PC kia phản hồi quá lâu
core-remote-closed = kết nối với PC kia đã đóng
core-remote-failed = kết nối với PC kia không thành công
core-remote-answered-something-else = ứng dụng khách đã trả lời một nội dung khác
# { $name } is the PC's name or address as the person typed it, such as OLDPC or 192.168.1.20.
core-remote-not-found = không tìm thấy { $name } trên mạng
core-remote-did-not-say-what-it-is = ứng dụng khách không cho biết nó là gì
core-remote-ordinary-names-only = chỉ có thể hỏi về đường dẫn nằm dưới thư mục gốc của PC kia, với tên thông thường
core-remote-more-but-none = PC kia cho biết một thư mục còn nhiều nội dung hơn, nhưng không gửi phần nào
# { $count } is the most things one folder may hold, such as 250,000.
core-remote-too-many-listed = PC kia đã liệt kê hơn { $count } mục trong một thư mục
core-remote-sent-too-much = PC kia đã gửi nhiều dữ liệu của một tệp hơn mức được yêu cầu
core-remote-name-not-text = không thể hỏi về một tên không phải là văn bản
core-remote-below-root-only = chỉ có thể hỏi về đường dẫn nằm dưới thư mục gốc của PC kia
core-remote-file-ended-early = tệp của PC kia kết thúc sớm
core-remote-link-not-followed = liên kết trên PC khác không được đi theo
core-remote-only-files-opened = chỉ có tệp mới được mở trên PC khác
core-remote-listing-said-nothing = danh sách không cho biết gì về mục này
core-remote-change-time-unknown = không rõ thời điểm mục này được thay đổi
core-remote-seek-before-start = di chuyển vị trí đọc tới trước điểm bắt đầu
# { $size } is how many bytes a file is; { $limit } is the most that were expected. Both are numbers.
core-remote-too-large =
    { $size ->
       *[other] mục này có { $size } byte, trong khi dự kiến không quá { $limit } byte
    }

## Checking that a download is signed by Microsoft. Each is given as the reason a check failed.

# { $error } is what Windows said went wrong.
core-signature-no-powershell = không thể khởi động PowerShell: { $error }
# { $error } is what went wrong reading PowerShell's answer.
core-signature-unreadable = không thể đọc chữ ký: { $error }
# { $signer } is who signed the file, such as Contoso Ltd.
core-signature-not-microsoft = tệp được ký bởi { $signer }, không phải Microsoft
core-signature-not-microsoft-unnamed = tệp được ký bởi bên khác, không phải Microsoft
# { $status } is Windows' own word for the signature's state, such as HashMismatch or NotSigned, which is not translated.
core-signature-status = trạng thái chữ ký của tệp là { $status }

## Warnings a package records when it is built from an app, which an installer shows later, maybe on another PC, in
## that PC's language. "This app" is the app being packaged.

core-warning-build-program-needs-unreadable = Không thể đọc những gì chương trình này cần từ Windows, nên sẽ không kiểm tra liệu nó có thể chạy trên PC kia hay không.
core-warning-build-external-manifest = Đã phát hiện tệp kê khai ứng dụng bên ngoài; việc diễn giải phần phụ thuộc side-by-side được hoãn lại.
core-warning-build-non-exe = Tệp đã chọn không dùng phần mở rộng .exe; quá trình quét PE sẽ tiếp tục.
core-warning-build-stand-in-program = Không tìm thấy tệp thực thi ứng dụng thông thường nào, nên tệp này được dùng thay thế. Kết quả về kiến trúc và phần phụ thuộc mô tả tệp này, và có thể không mô tả ứng dụng.
core-warning-build-other-apps-files-unknown = Không thể đọc toàn bộ thông tin mà Windows Installer có về các ứng dụng khác, nên một thư mục được ghi nhận cho ứng dụng này có thể chứa tệp của ứng dụng khác mà không có gì ở đây cho biết điều đó.
core-warning-build-app-folder-has-another-apps-file = Windows Installer cho biết một ứng dụng khác có tệp trong thư mục chứa chương trình của ứng dụng này, và tệp đó cũng được mang theo, vì toàn bộ thư mục đó được lấy.
# "App Paths" is the name of a Windows registry key; keep it as it is.
core-warning-build-app-paths-folder-left-out = Mục App Paths của ứng dụng này cho biết ứng dụng tìm trong một thư mục không phải của nó, nên thư mục đó không được mang theo. Ứng dụng có thể không khởi động trên PC không có nội dung của thư mục đó.
core-warning-build-uninstaller-folder-left-out = Thư mục chứa trình gỡ cài đặt của ứng dụng này đã bị loại trừ, vì Windows Installer cho biết một ứng dụng khác có tệp ở đó.
# { $why } is never shown: it chooses the reason. Translate each reason as the end of the sentence.
core-warning-build-recorded-folder-left-out =
    Một mục được ghi nhận cho ứng dụng này tham chiếu một thư mục đã bị loại trừ, vì { $why ->
        [unreadable] đường dẫn của thư mục đó dẫn ra ngoài ổ đĩa hoặc đi qua một liên kết, nên không thể đọc một cách an toàn
        [missing] thư mục đó không tồn tại
        [link] thư mục đó đi qua một liên kết
        [short-name] thư mục đó không được viết theo tên thật của nó
        [another-account] thư mục đó nằm trong thư mục của tài khoản khác
        [one-account] ứng dụng này được cài đặt cho một tài khoản, và thư mục đó không thuộc tài khoản ấy
        [shared] thư mục đó chứa nội dung dùng chung, hoặc là một phần của Windows
        [generic-name] tên của thư mục đó không cho biết nó thuộc về ai
        [publisher] thư mục đó chỉ mang tên nhà phát hành, và có thể chứa các sản phẩm khác của họ
        [holds-the-app] thư mục đó chứa chính thư mục của ứng dụng này, và có thể chứa nhiều hơn thế
        [another-apps-name] thư mục đó mang tên của ứng dụng khác
        [another-apps-record] bản ghi cài đặt của thư mục đó thuộc về ứng dụng khác
        [in-another-apps-folder] thư mục đó nằm trong thư mục của ứng dụng khác
       *[another-apps] thư mục đó thuộc về ứng dụng khác
    }.
core-warning-build-app-folder-holds-another-app = Một ứng dụng khác trên PC này được ghi nhận là nằm bên trong thư mục của ứng dụng này, và được mang theo cùng ứng dụng, vì toàn bộ thư mục đó được lấy.
# "Program Files (x86)" is the name of a Windows folder; keep it as Windows calls it.
core-warning-build-from-32-bit-windows = Ứng dụng này nằm trên Windows 32 bit. Các khóa sổ đăng ký của ứng dụng được sắp xếp theo cách Windows 64 bit lưu giữ khóa của ứng dụng 32 bit, và trình cài đặt đề xuất Program Files (x86) cho ứng dụng.
# { $dll } is a file's name, such as MSVCP140D.dll.
core-warning-build-debug-runtime = Một chương trình cần { $dll }, bản dựng gỡ lỗi của thư viện thời gian chạy Visual C++. Microsoft không có trình cài đặt cho các tệp này và có thể không cho phép phân phối lại chúng, nên chương trình sẽ không chạy trên PC không có Visual Studio.
# { $program } is a program's file name, such as Tool.exe.
core-warning-build-single-file-program = { $program } là chương trình .NET được đóng gói thành một tệp, không cho biết cần phiên bản .NET nào, nên không có .NET nào được lên kế hoạch cho chương trình này. Nếu chương trình cần .NET, nó sẽ không khởi động trên PC không có .NET đó.
# { $count } is how many of the app's programs.
core-warning-build-programs-not-inspected =
    { $count ->
       *[other] { $count } chương trình của ứng dụng không được kiểm tra về các thư viện thời gian chạy cần thiết, vì ứng dụng có quá nhiều chương trình hoặc chúng quá lớn.
    }
# { $runtime } is a runtime's name, such as Java Runtime Environment.
core-warning-build-runtime-no-installer = Ứng dụng cần { $runtime }, và ở đây không có trình cài đặt cho thành phần này, nên không có gì được chạy. Hãy tự cài đặt thành phần này trên PC kia.
# { $why } is why something was left out, such as "its name says what it holds".
core-warning-build-left-out-of-package = Đã loại trừ khỏi gói: { $why }.
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-missing-dependency = Không thể xác định phần phụ thuộc: { $name }
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-dependency-only-here = { $name } chỉ được tìm thấy qua đường dẫn tìm kiếm của PC này và không phải là một phần của ứng dụng, nên không được mang theo. Ứng dụng có thể không khởi động trên PC không có tệp này.
core-warning-build-driver-detected = Đã phát hiện trình điều khiển hoặc thành phần nhân. Trình điều khiển thuộc về ứng dụng sẽ được đóng gói và đăng ký lại; trình điều khiển của nhà cung cấp hoặc có sẵn trong Windows sẽ không được phân phối lại và phải được cài đặt lại từ gói đã ký của nhà cung cấp.
core-warning-build-near-no-longer-offered = Một mục đã chọn trong danh sách những gì có thể cũng thuộc ứng dụng này không còn nằm trong danh sách đó, nên không được thêm vào. Hãy xem lại danh sách.
# { $app_name } is the name of a part of the app, as the list of apps calls it. { $why } is never shown: it chooses the
# reason. Translate each reason as the end of the sentence.
core-warning-build-near-part-left-out =
    Một phần mà { $app_name } cho là của mình đã bị loại trừ: { $why ->
        [unreadable] đường dẫn của phần đó dẫn ra ngoài ổ đĩa hoặc đi qua một liên kết, nên không thể đọc một cách an toàn
        [missing] phần đó không tồn tại
        [link] phần đó đi qua một liên kết
        [short-name] phần đó không được viết theo tên thật của nó
        [another-account] phần đó nằm trong thư mục của tài khoản khác
        [one-account] ứng dụng này được cài đặt cho một tài khoản, và phần đó không thuộc tài khoản ấy
        [shared] phần đó chứa nội dung dùng chung, hoặc là một phần của Windows
        [generic-name] tên của phần đó không cho biết nó thuộc về ai
        [publisher] phần đó chỉ mang tên nhà phát hành, và có thể chứa các sản phẩm khác của họ
        [holds-the-app] phần đó chứa chính thư mục của ứng dụng này, và có thể chứa nhiều hơn thế
        [another-apps-name] phần đó mang tên của ứng dụng khác
        [another-apps-record] bản ghi cài đặt của phần đó thuộc về ứng dụng khác
        [in-another-apps-folder] phần đó nằm trong thư mục của ứng dụng khác
       *[another-apps] phần đó thuộc về ứng dụng khác
    }.
# { $app_name } is the name of a part of the app, as the list of apps calls it.
core-warning-build-near-keys-left-out = Các khóa sổ đăng ký mà bản ghi cài đặt của { $app_name } liệt kê đã bị loại trừ, vì bản ghi đó cũng tham chiếu những gì thuộc về ứng dụng khác.
# "Windows keeps what an app wrote for one account" is Windows' VirtualStore folder.
core-warning-build-virtual-store-program = Có một chương trình nằm trong thư mục nơi Windows lưu những gì ứng dụng đã ghi cho một tài khoản. Windows không bao giờ đặt chương trình vào đó, nên một thứ khác đã làm việc này, và chương trình đó đã bị loại trừ.
# { $path } is a file or folder, such as C:\Users\Sam\AppData\Roaming\Acme.
core-warning-build-file-left-out = { $path } đã bị loại trừ: mục này đi qua một liên kết trên ổ đĩa đã kết nối, hoặc thuộc về tài khoản khác với tài khoản có dữ liệu đang được đưa vào.
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as D:\Sam\Documents.
core-warning-build-user-folder-not-on-drive = Thư mục { $folder } của { $user } nằm tại { $path }, không thuộc ổ đĩa này, nên nội dung ở đó không được đưa vào.
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as \\server\home\Sam\Documents.
core-warning-build-user-folder-not-on-this-pc = Thư mục { $folder } của { $user } nằm tại { $path }, không nằm trên đĩa của PC này, nên nội dung ở đó không được đưa vào.
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-base-unreadable = Không thể đọc { $path } ({ $error }); dữ liệu ứng dụng lưu ở đó không được đưa vào.
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-folder-unreadable = Không thể đọc { $path } ({ $error }); dữ liệu ứng dụng lồng bên trong có thể bị thiếu.
# { $error } is what went wrong.
core-warning-build-entry-unreadable = Không thể đọc một mục trong hệ thống tệp: { $error }
# { $count } is how many links. { $names } lists some of them. { $more } is how many more there are, which are not
# named; when it is 0, the sentence ends after the list.
core-warning-build-links-not-followed =
    { $count ->
       *[other] { $count } liên kết, hoặc tên sẽ mở một thứ khác, đã không được đi theo trong thư mục này, nên những gì chúng trỏ tới không có trong trình cài đặt: { $names }
    }{ $more ->
        [0] .
       *[other] , và { $more } mục khác.
    }
# { $count } is how many folders. { $depth } is how many levels down a folder can be. { $names } lists some of them.
# { $more } is how many more there are, which are not named; when it is 0, the sentence ends after the list.
core-warning-build-folders-too-deep =
    { $count ->
       *[other] { $count } thư mục nằm sâu hơn { $depth } cấp, nên nội dung bên trong không có trong trình cài đặt: { $names }
    }{ $more ->
        [0] .
       *[other] , và { $more } mục khác.
    }
# { $error } is what went wrong.
core-warning-build-metadata-unreadable = Không thể đọc siêu dữ liệu của tệp: { $error }
core-warning-build-reserved-name = Tên của tệp này được dành riêng cho thiết bị Windows, nên tệp không được sao chép.
# { $error } is what went wrong.
core-warning-build-hash-failed = Không thể tính hàm băm của tệp: { $error }
core-warning-build-cloud-placeholder = Tệp đám mây chỉ có trực tuyến (ví dụ: Tệp theo yêu cầu của OneDrive) không có dữ liệu cục bộ; hãy ghim hoặc tải tệp xuống trước khi sao lưu. Đã báo cáo nhưng không sao chép.
# { $file_name } is a driver's file name, such as acmeflt.sys.
core-warning-build-driver-file = Tệp trình điều khiển/nhân “{ $file_name }” không được sao chép như một tệp thông thường. Nếu tệp này phục vụ một dịch vụ trình điều khiển đã đăng ký của ứng dụng này, trình cài đặt sẽ đóng gói tệp và tự động đăng ký lại trình điều khiển; nếu không, hãy cài đặt lại tệp bằng gói trình điều khiển đã ký của nhà cung cấp.
core-warning-build-file-gone = Mục này có ở đó khi ứng dụng được xem xét nhưng giờ đã biến mất, nên không có trong trình cài đặt.
# { $error } is what went wrong.
core-warning-build-file-copy-failed = Không thể sao chép tệp vào gói: { $error }
# { $count } is how many files. { $names } lists them.
core-warning-build-not-looked-through =
    { $count ->
       *[other] Gói này có tệp không thể được kiểm tra xem có tiết lộ nguồn gốc hay không: { $names }. Nội dung của các tệp đó có thể tiết lộ điều này.
    }
# { $count } is the most empty folders an installer makes, such as 10,000.
core-warning-build-empty-folders-truncated = Ứng dụng này có hơn { $count } thư mục trống, và chỉ chừng đó thư mục sẽ được tạo lại.
core-warning-build-credential-by-now = Đã loại trừ khỏi gói: giờ đây mục này trông giống thông tin xác thực.
core-warning-build-no-package-path = Tệp có thể sao chép không có đường dẫn tương đối trong gói.
# { $file_name } is the runtime installer's file name, such as vc_redist.x64.exe. { $error } is what went wrong.
core-warning-build-prerequisite-path-invalid = Không thể đặt thành phần tiên quyết đi kèm { $file_name }: { $error }
# { $runtime } is a runtime's name, such as Microsoft Visual C++ 2015+ Redistributable (x64). { $error } is what went
# wrong. winget is the name of a Windows command; keep it as it is.
core-warning-build-prerequisite-download-failed = Không thể tải xuống { $runtime } ({ $error }); trình cài đặt sẽ thử dùng winget trên máy đích.
# { $runtime } is a runtime's name. { $reason } is why the check failed, such as "it is signed by Contoso Ltd, not
# Microsoft".
core-warning-build-prerequisite-signature-failed = Bản tải xuống của { $runtime } không vượt qua kiểm tra chữ ký, nên không được đóng gói kèm: { $reason }. Trình cài đặt sẽ thử dùng winget trên máy đích.
# { $runtime } is a runtime's name. { $error } is what went wrong.
core-warning-build-prerequisite-hash-failed = Đã tải xuống { $runtime } nhưng không thể tính hàm băm ({ $error }); thay vào đó, trình cài đặt sẽ thử dùng winget trên máy đích.
# { $service } is a driver's service name, such as AcmeFlt. { $error } is what went wrong.
core-warning-build-driver-copy-failed = Không thể đóng gói tệp trình điều khiển cho “{ $service }”: { $error }
core-warning-build-license-not-copied = Không thể sao chép dữ liệu giấy phép (bị thiếu hoặc không đọc được); chỉ được ghi lại dưới dạng siêu dữ liệu.
core-warning-build-license-file-credential = Một tệp giấy phép trông giống thông tin xác thực và đã bị loại trừ.
# { $error } is what went wrong.
core-warning-build-license-store-walk-failed = Không thể liệt kê một phần của kho giấy phép: { $error }
core-warning-build-license-store-credential = Một tệp trong kho giấy phép trông giống thông tin xác thực và đã bị loại trừ.
# { $error } is what went wrong.
core-warning-build-license-store-file-skipped = Đã bỏ qua một tệp trong kho giấy phép: { $error }
