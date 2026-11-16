<a id="troubleshooting"></a>

# Khắc phục sự cố

Hãy tìm điều bạn thấy ở bên dưới, theo những gì xảy ra hoặc theo các từ trên màn hình. Các từ trong dấu ngoặc kép là những gì chương trình hiển thị, đôi khi có tên hoặc con số ở giữa.

Nếu không có gì ở đây giúp được bạn, hãy xem [Nhận trợ giúp](#getting-help) ở cuối trang.

<a id="the-window-could-not-open"></a>

## Không thể mở cửa sổ

**Bạn thấy gì.** Một hộp thông báo thay vì cửa sổ:

- từ ứng dụng: “Không thể mở cửa sổ”, và “Chương trình này cần Direct3D 12 để vẽ cửa sổ, nhưng trình điều khiển đồ họa cũ hoặc máy ảo có thể không hỗ trợ.”
- từ trình cài đặt: nội dung tương tự, và “Trình cài đặt vẫn có thể cài đặt mà không cần cửa sổ: hãy chạy tệp này từ dấu nhắc lệnh với quyền quản trị viên kèm --apply, và kèm --target-root theo sau là một thư mục để chọn nơi cài đặt.”
- từ trình gỡ cài đặt: nội dung tương tự, và “Trình gỡ cài đặt vẫn có thể gỡ cài đặt mà không cần cửa sổ, từ dấu nhắc lệnh với quyền quản trị viên:” kèm lệnh chính xác.

**Lý do.** Các cửa sổ được vẽ bằng [Direct3D 12](Glossary#direct3d-12), và không có cách nào khác để vẽ chúng.

**Cách xử lý.**

- Cập nhật trình điều khiển đồ họa của PC.
- Với trình cài đặt hoặc trình gỡ cài đặt, hãy dùng dòng lệnh. Xem [Dòng lệnh](Command-Line).
- Ứng dụng không có dòng lệnh. Hãy tạo trình cài đặt trên một PC khác: bạn có thể kết nối ổ đĩa của PC cũ với PC đó. Xem [Chuyển từ ổ đĩa đã kết nối](Moving-from-a-Connected-Drive). PC đó cũng có thể đọc PC cũ qua mạng. Xem [Chuyển từ một PC qua mạng](Moving-over-the-Network).

<a id="starting-the-app"></a>

## Khởi động ứng dụng

| Bạn thấy gì | Ý nghĩa | Cách xử lý |
| --- | --- | --- |
| Lời nhắc UAC ghi “Nhà phát hành không xác định” cho ứng dụng | Từ 0.5.0 trở đi, ứng dụng được ký. Ứng dụng không được ký thì không phải tệp phát hành, hoặc có từ trước 0.5.0. | Hãy tải xuống lại từ [bản phát hành mới nhất](https://github.com/RandyNorthrup/WinAppMigrator/releases/latest), và [kiểm tra tệp](Getting-Started#check-the-download). |
| SmartScreen ghi “Windows đã bảo vệ PC của bạn” | SmartScreen ít gặp tệp đó | Chọn **Thêm thông tin** (More info), kiểm tra rằng nhà phát hành là Randy Northrup, rồi chọn **Vẫn chạy** (Run anyway). |
| Nhãn ở trên cùng ghi **Tải không thành công**, và danh sách ghi “Không thể liệt kê ứng dụng” | Không thể đọc danh sách ứng dụng | Bấm vào mũi tên vòng tròn để tải lại. Với ổ đĩa đã kết nối, hãy kiểm tra rằng ổ đĩa vẫn được kết nối. |
| Ứng dụng bạn muốn không có trong danh sách | Đó là ứng dụng Microsoft Store, phần ẩn của một ứng dụng khác, hoặc hoàn toàn không có mục nào trong Cài đặt, chẳng hạn ứng dụng chạy trực tiếp không cần cài đặt (portable) | Không thể chuyển ứng dụng Store. Phần ẩn có thể được đề xuất trong **Có thể cũng là một phần của ứng dụng** khi bạn chọn ứng dụng chính. Không thể chọn ứng dụng không có mục trong Cài đặt. |
| “Không có ứng dụng phù hợp” | Nội dung tìm kiếm hoặc bộ lọc đang ẩn mọi thứ | Xóa nội dung ô tìm kiếm, và chọn **Tất cả**. |

<a id="building"></a>

## Tạo trình cài đặt

<a id="picking-a-drive-or-a-disk-image"></a>

### Chọn ổ đĩa hoặc ảnh đĩa

| Bạn thấy gì | Cách xử lý |
| --- | --- |
| Ổ đĩa “không giống ổ đĩa hệ thống Windows (không có \Windows và \Users). Hãy chọn thư mục gốc của ổ đĩa lấy từ PC kia.” | Hãy chọn cấp trên cùng của ổ đĩa, chẳng hạn `E:\`, chứ không phải một thư mục trên đó. Kiểm tra rằng đó là ổ đĩa mà Windows đã được cài đặt. |
| Ảnh đĩa “không phải là ảnh đĩa mà Windows có thể gắn (.vhd hoặc .vhdx)” | Hãy chọn tệp `.vhd` hoặc `.vhdx`. Trước tiên, hãy lưu đĩa VMware hoặc VirtualBox thành một trong hai định dạng đó. Với điểm kiểm tra Hyper-V, `.avhdx`, hãy xuất đĩa của máy ảo. |
| “không thể gắn” ảnh đĩa: “Windows đã gắn ảnh đĩa này, và thứ đã gắn nó có thể đang ghi vào đó.” | Ảnh đĩa đang được mở ở nơi khác, ví dụ đã được gắn trong Chương trình Quản lý Tệp. Thay vào đó, hãy chọn ổ đĩa của nó trong **Ổ đĩa đã kết nối…**, hoặc tháo ảnh đĩa, rồi chọn lại. |
| Ảnh đĩa “không có ổ đĩa nào chứa Windows có thể đọc được. Ổ đĩa được mã hóa bằng BitLocker phải được mở khóa trước.” | Kiểm tra rằng đó là ảnh của ổ đĩa mà Windows đã được cài đặt. Nếu ảnh bị khóa bằng [BitLocker](Glossary#bitlocker), hãy tự gắn nó trong Windows, mở khóa, rồi chọn ổ đĩa của nó trong **Ổ đĩa đã kết nối…**. |
| Ảnh đĩa “là đĩa khác biệt, chỉ chứa những gì đã thay đổi kể từ đĩa gốc của nó” | Hãy hợp nhất nó vào đĩa gốc bằng Chỉnh sửa đĩa (Edit Disk) của Hyper-V, rồi chọn đĩa gốc. |
| “không thể gắn” ảnh đĩa, kèm một lý do khác | Lý do là của chính Windows. Hãy đọc lý do, kiểm tra rằng không có gì khác đang dùng ảnh đĩa, rồi thử lại. |

<a id="reading-a-pc-over-the-network"></a>

### Đọc một PC qua mạng

Danh sách đầy đủ, bao gồm những gì ứng dụng khách hiển thị trên PC cũ, có trong [Chuyển từ một PC qua mạng](Moving-over-the-Network#if-something-goes-wrong).

| Bạn thấy gì | Cách xử lý |
| --- | --- |
| PC cũ không được liệt kê trong **2. Chọn PC cũ** | Kiểm tra rằng ứng dụng khách đang chạy trên PC đó và hiển thị một mã, và cả hai PC ở trên cùng một mạng. Sau đó nhập tên hoặc địa chỉ của PC đó. Nếu đi qua bộ định tuyến, hãy nhập địa chỉ. Tường lửa không phải của Windows phải cho phép TCP 47732 và UDP 47731 đi qua. |
| “không tìm thấy” một tên “trên mạng” | Hãy nhập địa chỉ của PC cũ thay vào đó. |
| “không thể kết nối tới PC cũ:” và một lý do | Kiểm tra rằng ứng dụng khách vẫn đang chạy, và tên hoặc địa chỉ là đúng. |
| “đó không phải là mã mà PC cũ hiển thị. Hãy kiểm tra rồi thử lại.” | Hãy nhập mã mà ứng dụng khách đang hiển thị. Sau năm lần nhập sai mã, ứng dụng khách tạo mã mới. Sau 20 lần, nó không cho ghép nối nữa cho đến khi bạn khởi động lại nó. |
| Trên PC cũ, Windows ghi nhà phát hành của ứng dụng khách là không xác định | Điều đó chỉ bình thường trên Windows 7 không có chứng chỉ gốc năm 2020 của Microsoft, vốn không thể kiểm tra chữ ký của ứng dụng khách. Ở đó, chỉ chọn **Có** cho ứng dụng khách mà bạn đã tự lưu từ ứng dụng. Ở bất kỳ nơi nào khác, lời nhắc phải ghi tên Randy Northrup. Nếu không, đừng chạy bản sao đó. |
| Ứng dụng không thể đọc PC cũ nữa, sau một thời gian dài không hoạt động | Ứng dụng khách ngắt kết nối nếu ứng dụng không yêu cầu gì trong 30 phút. Hãy bấm **Ngắt kết nối**, rồi ghép nối lại. |

<a id="looking-at-the-app"></a>

### Xem xét ứng dụng

| Bạn thấy gì | Ý nghĩa | Cách xử lý |
| --- | --- | --- |
| “Không thể xem xét ứng dụng này” và một lý do | Việc xem xét đã thất bại. Một lý do thường gặp là mục hoàn toàn không có chương trình nào, chẳng hạn gói trình điều khiển. | Bấm **Thử lại**. Nếu lỗi lặp lại như cũ, mục đó có thể không phải là ứng dụng mà WinAppMigrator có thể chuyển. |
| “Đã dừng. Có thể tạo trình cài đặt sau khi ứng dụng này đã được xem xét.” | Bạn đã bấm **Dừng** | Bấm **Xem xét lại**. |
| Việc xem xét mất nhiều thời gian | Ứng dụng lớn, ổ đĩa chậm hoặc sắp hỏng, hoặc nhiều tài khoản | Nếu ổ đĩa có thể sắp hỏng, hãy bấm **Dừng**, tạo [ảnh đĩa](Glossary#disk-image) của nó, rồi chọn ảnh đĩa đó thay vào, vì việc đọc một ổ đĩa sắp hỏng có thể khiến nó hỏng hẳn. Từ 0.6.0, hãy chọn ảnh đĩa bằng **Ảnh đĩa…**. Với 0.5.0, hãy gắn ảnh đĩa rồi chọn ổ đĩa của nó. Nếu không, hãy chờ. Trong lúc đó, bạn có thể chọn ứng dụng khác. |
| Một hàng bạn muốn bị làm mờ và đã được chọn | Hàng đó chứa chương trình chính, hoặc những gì chương trình cần để khởi động | Hàng đó luôn được đưa vào. Xem [các hàng bạn không thể bỏ chọn](Building-an-Installer#the-groups). |
| Một hàng ghi “Cũng bị loại trừ:” | Một thứ bạn đã bỏ chọn đã kéo theo hàng này | Hãy chọn lại thứ đó, nếu bạn muốn giữ hàng này. |
| “Phần còn lại không thể tạo thành một gói” | Những gì bạn bỏ chọn khiến không còn gì có thể tạo thành gói | Hãy chọn lại một mục nào đó. |
| “Dữ liệu giấy phép trong những gì bạn đã loại trừ vẫn được đưa vào” | Giấy phép chỉ tuân theo tùy chọn **Dữ liệu giấy phép và kích hoạt** | Bỏ chọn tùy chọn đó nếu bạn muốn loại trừ chúng. |

<a id="create-installer-stays-greyed-out"></a>

### Nút Tạo trình cài đặt vẫn bị làm mờ

Nút này chờ cho đến khi tất cả các điều sau đều đúng:

- việc xem xét ứng dụng đã hoàn tất, với các tùy chọn như hiện tại (“Đang xác định những gì đi kèm với mục đó…” nghĩa là ứng dụng vẫn đang xác định những gì mà lần bỏ chọn gần nhất của bạn kéo theo);
- **Thư mục đầu ra** là đường dẫn đầy đủ, chẳng hạn `D:\Installers` (“Hãy nhập đường dẫn thư mục đầy đủ, chẳng hạn D:\Installers.”);
- ổ đĩa đó còn đủ dung lượng.

<a id="room"></a>

### Dung lượng

| Bạn thấy gì | Cách xử lý |
| --- | --- |
| “Không đủ dung lượng.” và một ổ đĩa cần bao nhiêu, còn bao nhiêu | Hãy chọn thư mục đầu ra trên ổ đĩa khác, hoặc bỏ bớt một số mục. Con số này là mức tối đa mà một lần tạo có thể cần, trước khi nén, nên có thể lớn hơn kích thước thực của trình cài đặt. |
| Một ổ đĩa “không cho biết còn bao nhiêu dung lượng” | Quá trình tạo vẫn tiếp tục mà không kiểm tra ổ đĩa đó. Hãy đảm bảo ổ đĩa còn đủ chỗ. |
| Ổ đĩa FAT32 và một ứng dụng lớn | FAT32 không thể chứa tệp 4 GiB. Hãy dùng ổ đĩa NTFS hoặc exFAT. |

<a id="the-done-screen"></a>

### Màn hình hoàn tất

| Bạn thấy gì | Cách xử lý |
| --- | --- |
| “Đã tạo trình cài đặt, nhưng” một số tệp “đã biến mất kể từ khi ứng dụng được xem xét” | Bấm **Xem xét lại**, rồi **Tạo lại trình cài đặt**, nếu các tệp đó quan trọng. |
| “Đã tạo trình cài đặt, nhưng không thể thu thập” một số tệp, “thường là do tệp bị khóa hoặc không đọc được.” | Hãy đóng ứng dụng, và mọi thứ khác đang dùng tệp của nó, rồi tạo lại. **Cảnh báo** liệt kê những tệp nào. |
| “Đã tạo trình cài đặt, nhưng trình cài đặt vẫn còn chứa thông tin cho biết nó đến từ ai.” | Hãy đọc các thông báo màu hổ phách bên dưới. Xem [lần xem xét cuối cùng](Building-an-Installer#the-last-look). |
| **Cảnh báo** ghi “Không thể tải xuống” một thư viện thời gian chạy, và “trình cài đặt sẽ thử dùng winget trên máy đích.” | Thư viện thời gian chạy đó không nằm trong trình cài đặt, nên PC mới sẽ cần internet cho nó. Hãy tạo lại khi có kết nối internet. Xem [Thư viện thời gian chạy](Runtimes#at-build-time). |
| “Đừng dùng trình cài đặt này: trình cài đặt không vượt qua kiểm tra hàm băm.” | Đừng dùng nó. Hãy tạo lại. Nếu điều này xảy ra lần nữa, ổ đĩa có thể sắp hỏng. |
| “Trình cài đặt đã được ghi, nhưng không thể chạy kiểm tra hàm băm cho trình cài đặt” | Đừng dùng nó. Hãy tạo lại. |
| “Đã hủy việc tạo. Không có trình cài đặt nào được ghi.” | Bạn đã bấm **Hủy**. |
| “Không thể tạo trình cài đặt.” và một lý do | Hãy đọc lý do. **Sao chép thông báo lỗi** sẽ sao chép lý do, và **Nhật ký** cho biết nhật ký nằm ở đâu. |
| “Không thể tạo thư mục đầu ra” | Hãy chọn thư mục mà bạn có thể ghi vào. |
| “Không thể tạo thư mục trung gian được bảo vệ, nên không có gì được tạo” | Quá trình tạo không thể tạo thư mục làm việc riêng bên cạnh thư mục đầu ra, hoặc trong thư mục tạm của Windows. Hãy chọn thư mục đầu ra trên ổ đĩa khác. |
| “Quá trình tạo đã dừng đột ngột” | Đã xảy ra sự cố mà không gì lường trước được. “Nếu quá trình này để lại trình cài đặt trong thư mục đầu ra, đừng dùng trình cài đặt đó.” Vui lòng [báo cáo sự cố](#getting-help). |

<a id="installing"></a>

## Cài đặt

<a id="when-the-installer-opens"></a>

### Khi trình cài đặt mở ra

| Bạn thấy gì | Ý nghĩa | Cách xử lý |
| --- | --- | --- |
| “Đang kiểm tra trình cài đặt…” trong thời gian dài | Trình cài đặt đọc toàn bộ gói để kiểm tra hàm băm | Hãy chờ. Trình cài đặt lớn sẽ mất một lúc. |
| “Không thể đọc trình cài đặt:” và “trình cài đặt này bị hỏng hoặc đã bị thay đổi sau khi được tạo, nên không có gì được đọc từ gói của nó. Hãy sao chép lại trình cài đặt từ nơi nó được tạo.” | Tệp bị cắt ngắn hoặc đã bị thay đổi | Hãy sao chép lại. |
| “trình cài đặt này lưu gói của nó trong một tệp tên là” một tệp `.wamdata`, “trong cùng thư mục, và tệp đó không có ở đây” | Trình cài đặt gồm hai tệp đang thiếu tệp dữ liệu | Hãy sao chép tệp `.wamdata` vào cùng thư mục. Xem [Ứng dụng lớn](Big-Apps). |
| tệp dữ liệu “không phải là tệp dữ liệu của trình cài đặt này, hoặc tệp bị hỏng hay đã bị thay đổi sau khi được tạo” | Tệp dữ liệu không khớp | Hãy sao chép lại cả hai tệp, từ nơi chúng được tạo. |
| “Không thể dùng gói của trình cài đặt này trên PC này:” và “ứng dụng này không thể chạy trên PC này, nên không có gì bị thay đổi” | Các chương trình của ứng dụng được tạo cho bộ xử lý khác, hoặc một chương trình cho biết nó cần phiên bản Windows mới hơn | Hãy cài đặt trên một PC giống với PC cũ. Chưa có gì bị thay đổi. |
| “Đây là phần khởi chạy (stub) của trình cài đặt WinAppMigrator và không chứa gói ứng dụng nào.” | Đây là phần khởi chạy trống, không phải trình cài đặt đã được tạo | Hãy tạo trình cài đặt bằng ứng dụng, rồi chạy trình cài đặt đó. |
| “Chữ ký của trình cài đặt này không hợp lệ”, kèm lý do của Windows | Trình cài đặt đã được ký, nhưng Windows không chấp nhận chữ ký. Trình cài đặt có thể đã bị thay đổi sau đó, hoặc chứng chỉ của nó không được tin cậy trên PC này. | Đừng cài đặt. Hãy hỏi lại người đã tạo trình cài đặt. |
| Lời nhắc UAC ghi “Nhà phát hành không xác định” | Trình cài đặt chưa được ký. Điều đó là bình thường với trình cài đặt do bạn tạo. | Chỉ chạy nếu bạn tin cậy nguồn gốc của nó. |

<a id="the-install-location"></a>

### Vị trí cài đặt

Khi **Cài đặt** và **Xem trước thay đổi** bị làm mờ, một thông báo màu đỏ bên dưới **Vị trí cài đặt** sẽ cho biết lý do. Lý do thường gặp nhất là ứng dụng đã được cài đặt trong thư mục đó trên PC này, như thường thấy với trình duyệt web, trình phát đa phương tiện hoặc Office: hãy gỡ cài đặt bản sao đó trước, trong Cài đặt của Windows, mục **Ứng dụng**, rồi mở lại trình cài đặt, hoặc nhập một thư mục mới, trống. Các lý do, và cách xử lý, có trong [Cài đặt trên PC mới](Installing-on-the-New-PC#when-the-location-cannot-be-used).

<a id="before-or-during-the-install"></a>

### Trước hoặc trong khi cài đặt

| Bạn thấy gì | Ý nghĩa | Cách xử lý |
| --- | --- | --- |
| “Có thể không đủ dung lượng ở đây” | Ổ đĩa có vẻ thiếu dung lượng | Hãy giải phóng dung lượng, hoặc chọn vị trí trên ổ đĩa khác. |
| “không đủ dung lượng cho việc này, nên không có gì bị thay đổi” | Một ổ đĩa mà trình cài đặt ghi vào đã quá đầy | Hãy giải phóng dung lượng ở đó, hoặc chọn vị trí cài đặt trên ổ đĩa khác. |
| “Những gì trình cài đặt sẽ làm ở đây đã thay đổi, nên không có gì được cài đặt.” | Có gì đó trên PC đã thay đổi kể từ khi danh sách được xác định | Hãy đọc lại danh sách, và bấm **Cài đặt** lần nữa nếu đó là điều bạn muốn. |
| “Một lần cài đặt” một ứng dụng “trước đây chưa hoàn tất. Thao tác Cài đặt sẽ hoàn tác lần đó trước.” | Một lần cài đặt trước đó đã bị gián đoạn | Bấm **Cài đặt**. Trình cài đặt sẽ hoàn tác lần đó trước. |
| “một lần cài đặt hoặc gỡ cài đặt khác đang chạy. Hãy đợi cho đến khi hoàn tất, rồi thử lại” | Mỗi lần chỉ chạy được một | Hãy chờ, rồi thử lại. |
| Một tệp “có tên mà phiên bản Windows này mở như một thiết bị chứ không phải tệp” | Ứng dụng có một tệp tên giống `aux.h`, mà Windows trước Windows 11 không thể chứa | Hãy cài đặt trên Windows 11. |
| “Không thể tạo thư mục trung gian” | Không có nơi riêng tư nào còn đủ dung lượng để giải nén gói | Hãy giải phóng dung lượng trên ổ đĩa của vị trí cài đặt hoặc trên ổ đĩa hệ thống. |
| Một thư mục dữ liệu như `%AppData%` được cho là một liên kết | Trình cài đặt sẽ không ghi xuyên qua [liên kết](Glossary#link) | Hãy bỏ chọn **Dữ liệu người dùng** và **Giấy phép**, rồi cài đặt mà không có chúng. |
| “Không tìm thấy thư mục ProgramData của PC này, nên không có gì được cài đặt.” | Không thể đọc bản ghi của chính Windows về vị trí của ProgramData | Trình cài đặt sẽ không cài đặt nếu không có nơi an toàn để lưu bản ghi của nó. |

<a id="the-result"></a>

### Kết quả

| Bạn thấy gì | Cách xử lý |
| --- | --- |
| “Cài đặt thất bại và những gì đã thay đổi đã được khôi phục lại.” | Hãy đọc các thông báo màu đỏ để biết lý do. PC vẫn như trước, ngoại trừ thư viện thời gian chạy đã cài đặt. |
| “Cài đặt thất bại và không phải mọi thay đổi đều có thể được khôi phục lại.” | Hãy đọc những gì được liệt kê. Đóng những gì có thể đang dùng các tệp đó, rồi chạy lại một trình cài đặt: trình cài đặt sẽ thử khôi phục lại trước. |
| “Không thể cài đặt” một thư viện thời gian chạy | Không thể cài đặt thư viện thời gian chạy, nên lần cài đặt đã được khôi phục lại. Với DirectX End-User Runtime, PC mới cần có internet. Xem [Thư viện thời gian chạy](Runtimes#with-no-internet-on-the-new-pc). |
| “Một phần phụ thuộc cần khởi động lại để hoàn tất. Hãy khởi động lại Windows, rồi mở ứng dụng.” | Hãy khởi động lại, rồi mở ứng dụng. |
| Dịch vụ “đã chạy dưới tài khoản” một tài khoản “trên máy nguồn. Mật khẩu của tài khoản đó không có trong trình cài đặt, nên dịch vụ được thiết lập để chạy dưới LocalService.” | Hãy mở Dịch vụ, và đặt tài khoản cùng mật khẩu cho dịch vụ, nếu dịch vụ không hoạt động như hiện tại. |
| Một số tệp “được đặt trong” thư mục cài đặt “thay vào đó, trong một thư mục tên là _external” | PC này lưu Tài liệu hoặc Trò chơi đã lưu ngoài hồ sơ người dùng của bạn. Hãy tự di chuyển các tệp đó vào thư mục ấy. |
| Một mục “bị loại trừ”, kèm lý do | Các quy tắc đã từ chối mục đó, hoặc nó đã thuộc về một ứng dụng khác ở đây. Phần còn lại đã được cài đặt. Nếu ứng dụng cần mục đó, hãy xem lý do. |
| **Hàm băm của gói KHÔNG HỢP LỆ** hoặc **Tệp đã cài đặt KHÔNG HỢP LỆ** | Có gì đó không khớp. Hãy sao chép lại trình cài đặt, và thử thêm một lần nữa. |

<a id="after-installing"></a>

### Sau khi cài đặt

| Bạn thấy gì | Cách xử lý |
| --- | --- |
| Ứng dụng không khởi động | Hãy kiểm tra **Phần phụ thuộc** xem có thư viện thời gian chạy nào được đánh dấu **Cài đặt thủ công** không, và cài đặt nó. Hãy kiểm tra các cảnh báo trong kết quả. |
| Ứng dụng từ Windows 7 không khởi động, và một cảnh báo khi tạo trình cài đặt đã nêu tên một DLL mà “Windows 10 và 11 không đi kèm” | DLL đó đã được một chương trình khác đặt vào các thư mục riêng của Windows cũ, nên nó không được mang theo. Hãy cài đặt thứ cung cấp DLL đó, rồi khởi động lại ứng dụng. Xem [Ứng dụng từ Windows 7, Vista, 8 hoặc 8.1](Moving-from-a-Connected-Drive#apps-from-windows-7-vista-8-or-81). |
| Ứng dụng khởi động như mới | Điều đó là bình thường với trình cài đặt được tạo khi **Dữ liệu người dùng** tắt. Điều này cũng xảy ra khi **Dữ liệu người dùng** bị bỏ chọn lúc cài đặt, vì khi đó dữ liệu được đặt trong thư mục cài đặt, và khi trình cài đặt không xác định được ai đang đăng nhập, điều mà trình cài đặt sẽ cho biết. |
| Ứng dụng yêu cầu kích hoạt | Giấy phép gắn với PC cũ cần được kích hoạt lại. |
| Trình cập nhật riêng của ứng dụng cho rằng ứng dụng chưa được cài đặt | Windows Installer không có bản ghi nào về ứng dụng ở đây. Xem [Hạn chế](Limitations). |
| Một loại tệp hoặc loại liên kết không mở bằng ứng dụng | Một ứng dụng khác đã sở hữu nó ở đây, nên nó được để nguyên. Hãy chọn ứng dụng trong Cài đặt, mục Ứng dụng mặc định. |
| Mục `Path` mới không được nhận | Hãy mở một cửa sổ PowerShell hoặc dấu nhắc lệnh mới. Một số chương trình chỉ nhận thay đổi sau khi bạn đăng xuất rồi đăng nhập lại. |

<a id="uninstalling"></a>

## Gỡ cài đặt

| Bạn thấy gì | Cách xử lý |
| --- | --- |
| “Một phần của ứng dụng nằm trên ổ đĩa chưa được kết nối.” | Hãy kết nối ổ đĩa đó, rồi gỡ cài đặt lại từ Cài đặt. |
| “Không thể hoàn tác tất cả. Hãy đóng những gì đang sử dụng ứng dụng, rồi gỡ cài đặt lại từ Cài đặt.” | Hãy đóng ứng dụng và mọi thứ đang dùng tệp của nó. Nếu một dịch vụ không dừng, hãy khởi động lại PC. Sau đó gỡ cài đặt lại. |
| “this is not an install's id” | ID được cung cấp cho `--uninstall` không đúng. Hãy sao chép ID từ mục trong Cài đặt. Xem [Dòng lệnh](Command-Line#finding-the-id). |
| Một tệp tên `.before-install` xuất hiện | Đó là bản gốc của một tệp mà lần cài đặt đã thay thế, trong khi bạn đã thay đổi tệp được cài đặt kể từ đó. Hãy giữ bản nào bạn muốn. |
| Một thư viện thời gian chạy vẫn được cài đặt | Thư viện thời gian chạy được giữ lại, vì các chương trình khác có thể dùng chúng. Hãy để nguyên trừ khi bạn chắc chắn không có gì khác dùng nó. |

<a id="scripts"></a>

## Tập lệnh

| Bạn thấy gì | Cách xử lý |
| --- | --- |
| “The requested operation requires elevation”, hoặc thông báo tương tự bằng ngôn ngữ của Windows | Hãy chạy PowerShell với quyền quản trị viên. |
| Không có gì được in ra, và `$LASTEXITCODE` không được đặt | Hãy chuyển đầu ra của trình cài đặt qua đường ống, ví dụ tới `Out-String`. Xem [Dòng lệnh](Command-Line). |
| Mã thoát `2` | Một cờ không đúng, hoặc `--apply` và `--dry-run` được dùng cùng nhau. |
| “this package does not say where its app was, so give a --target-root” | Hãy thêm `--target-root` với một đường dẫn đầy đủ. |
| Ứng dụng “was in” một thư mục, “and that cannot be used” | Hãy thêm `--target-root` với một thư mục khác. |
| Một thư mục “cannot be used: it already has files” “Give --force only if you know it is this app's” | Hãy chọn một thư mục trống. Chỉ dùng `--force` cho thư mục mà bạn biết là của ứng dụng này. |

<a id="language-and-text"></a>

## Ngôn ngữ và văn bản

- **Sai ngôn ngữ.** Hãy chọn ngôn ngữ khác trong **Ngôn ngữ**, ở góc trên bên phải của ứng dụng hoặc của màn hình đầu tiên của trình cài đặt. Ứng dụng ghi nhớ lựa chọn đó; trình cài đặt giữ lựa chọn cho lần chạy đó. Khi chưa có lựa chọn nào, mỗi chương trình dùng ngôn ngữ đầu tiên trong số các ngôn ngữ hiển thị của Windows mà nó có, nếu không thì dùng tiếng Anh.
- **Bản dịch đọc không đúng.** Mọi ngôn ngữ ngoài tiếng Anh đều được dịch bằng máy. Trong **Ngôn ngữ**, hãy chọn **Báo cáo lỗi bản dịch**, mục này mở một biểu mẫu đã điền sẵn ngôn ngữ của bạn. Hãy cho biết màn hình hiển thị gì và đáng lẽ phải ghi gì.
- **Chữ hiển thị thành ô trống.** Cửa sổ dùng các phông chữ đi kèm Windows cho từng hệ chữ viết, từ `C:\Windows\Fonts`. Windows đã bị xóa một số phông chữ trong đó sẽ vẽ các chữ ấy thành ô trống; tên của các ứng dụng khác trong danh sách cũng có thể cần đến các phông chữ đó.
- **Chữ bị cắt.** Nhãn không vừa sẽ kết thúc bằng `…`; hãy di chuột lên nhãn, hoặc di chuyển tới nhãn bằng phím Tab, để xem toàn bộ. Hãy báo cáo kèm ngôn ngữ và màn hình, để cửa sổ có thể được chỉnh cho vừa.
- **WinAppMigrator Client in tiếng Anh trên Windows có hướng viết từ phải sang trái.** Cửa sổ dòng lệnh mà ứng dụng khách chạy trong đó không thể nối hoặc sắp xếp chữ Ả Rập hay chữ Do Thái, nên nó in tiếng Anh và cho biết lý do. `--lang` chọn bất kỳ ngôn ngữ nào mà ứng dụng khách có.

<a id="getting-help"></a>

## Nhận trợ giúp

1. Tìm [nhật ký](Glossary#log). Đường dẫn của nhật ký được hiển thị dưới dạng **Nhật ký** khi có lỗi, hoặc được in ra dưới dạng `Log:` từ dòng lệnh. Nhật ký nằm trong `%ProgramData%\WinAppMigrator\logs`. Nhật ký chứa đường dẫn và tên khóa, và không bao giờ chứa nội dung của thiết đặt, tệp giấy phép hay biến môi trường. Tuy vậy, nhật ký vẫn có thể chứa tên thư mục của bạn, nên hãy đọc trước khi chia sẻ.
2. [Mở một issue](https://github.com/RandyNorthrup/WinAppMigrator/issues) kèm phiên bản, những gì bạn đã làm, và những gì màn hình hiển thị. **Sao chép thông báo lỗi** trên màn hình hoàn tất của ứng dụng sẽ sao chép thông báo.
3. **Không bao giờ đính kèm trình cài đặt vào issue.** Trình cài đặt có thể chứa dữ liệu riêng tư.
4. Với vấn đề bảo mật, hãy báo cáo riêng thay vào đó, qua [cảnh báo bảo mật của GitHub](https://github.com/RandyNorthrup/WinAppMigrator/security/advisories/new).
