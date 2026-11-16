<a id="installing-on-the-new-pc"></a>

# Cài đặt trên PC mới

Trình cài đặt là một cửa sổ duy nhất. Trình cài đặt tự kiểm tra, cho bạn thấy nơi nó sẽ cài đặt và mọi thứ nó sẽ thiết lập, và chỉ cài đặt khi bạn bấm **Cài đặt**. Trang này đi qua cửa sổ đó từ trên xuống dưới, dùng đúng các từ trên màn hình.

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/installer-dark.png">
    <img alt="Trình cài đặt cho Fabrikam Data Sync, cho biết trình cài đặt chưa được ký, vị trí cài đặt, dung lượng cần thiết, và những gì được thiết lập ngoài tệp" src="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/installer-light.png" width="600">
  </picture>
</p>

<a id="before-you-start"></a>

## Trước khi bắt đầu

- **Sao chép toàn bộ trình cài đặt.** Nếu quá trình tạo cho bạn một tệp `.wamdata` cùng với tệp `.exe`, hãy sao chép cả hai vào cùng một thư mục và giữ nguyên tên của chúng. Xem [Ứng dụng lớn](Big-Apps).
- **Chỉ chạy những trình cài đặt mà bạn tin cậy.** Trình cài đặt chạy với toàn bộ [quyền quản trị viên](Glossary#administrator-rights). Hãy đối xử với trình cài đặt như mọi chương trình khác: chỉ chạy nếu bạn đã tự tạo nó, hoặc nhận nó từ người mà bạn tin cậy. Xem [Quyền riêng tư và bảo mật](Privacy-and-Security).
- **Bạn cần quyền quản trị viên** trên PC này, và [Direct3D 12](Glossary#direct3d-12) cho cửa sổ. Không có Direct3D 12, bạn vẫn có thể cài đặt từ dấu nhắc lệnh. Xem [Dòng lệnh](Command-Line).

<a id="open-it"></a>

## Mở trình cài đặt

Bấm đúp vào trình cài đặt. Windows hiển thị [lời nhắc UAC](Glossary#uac-prompt). Trình cài đặt không được ký trừ khi người tạo ra nó đã ký, nên lời nhắc thường ghi **Nhà phát hành không xác định**. Điều đó là bình thường. Xem [Quyền riêng tư và bảo mật](Privacy-and-Security#signing).

Cửa sổ “Trình cài đặt WinAppMigrator”, “Cài đặt lại một ứng dụng đã được thu thập trên PC khác”, mở ra ngay lập tức, dù trình cài đặt lớn đến đâu. Cửa sổ hiển thị “Đang kiểm tra trình cài đặt…” trong khi kiểm tra [hàm băm](Glossary#hash) của toàn bộ gói. Việc này đọc toàn bộ gói, nên trình cài đặt lớn sẽ mất một lúc. Từ thời điểm này, trình cài đặt giữ mở chính tệp của nó, và cả tệp dữ liệu nếu có, nên không tệp nào có thể bị thay đổi hay tráo đổi trong khi cửa sổ đang mở.

Quá trình kiểm tra có thể dừng ở đây, với một thông báo màu đỏ. Các thông báo thường gặp là:

- Trình cài đặt bị hỏng hoặc đã bị thay đổi sau khi được tạo: hãy sao chép lại từ nơi nó được tạo.
- Tệp dữ liệu của trình cài đặt bị thiếu, hoặc không phải tệp đúng: hãy sao chép lại cả hai tệp, vào cùng một thư mục.
- “Không thể dùng gói của trình cài đặt này trên PC này”, chẳng hạn vì các chương trình của ứng dụng được tạo cho loại bộ xử lý khác, hoặc cần phiên bản Windows mới hơn. Chưa có gì bị thay đổi.

Tất cả các trường hợp này đều được giải thích trong [Khắc phục sự cố](Troubleshooting#installing).

Khi kiểm tra thành công, tiêu đề của cửa sổ trở thành “Cài đặt” và tên của ứng dụng.

<a id="its-language"></a>

### Ngôn ngữ của trình cài đặt

Trình cài đặt dùng ngôn ngữ của PC mà nó chạy trên đó, chứ không phải của PC đã tạo ra nó: ngôn ngữ hiển thị của Windows nếu đó là một trong 24 ngôn ngữ mà WinAppMigrator có, nếu không thì dùng tiếng Anh. **Ngôn ngữ**, ở góc trên bên phải, thay đổi ngôn ngữ cho lần chạy này. Những gì trình cài đặt liệt kê về ứng dụng, kể cả các ghi chú mà WinAppMigrator đã viết khi tạo trình cài đặt, cũng hiển thị bằng ngôn ngữ đó. Trình cài đặt được tạo bằng WinAppMigrator 0.6 trở về trước hiển thị các ghi chú đó bằng tiếng Anh, như khi chúng được viết. [Cài đặt im lặng từ dòng lệnh](Command-Line) luôn trả lời bằng tiếng Anh, để tập lệnh đọc được.

<a id="the-app-and-where-it-goes"></a>

## Ứng dụng và nơi cài đặt

Thẻ đầu tiên có tên của ứng dụng, với nhà phát hành và phiên bản ở bên dưới, chẳng hạn “Fabrikam Inc. · 1.7.3”. Tiếp theo là:

| Hàng | Nội dung |
| --- | --- |
| **Thu thập từ** | Tên của PC mà ứng dụng được lấy từ đó. Hàng này chỉ xuất hiện khi trình cài đặt được tạo với tùy chọn **Dữ liệu người dùng** được bật. |
| **Trình cài đặt** | Tệp trình cài đặt này có được [ký](Glossary#signature) hay không: “Đang kiểm tra chữ ký…”, rồi “Chưa được ký: nhà phát hành không xác định”, hoặc “Được ký bởi” và tên người ký. Chữ ký không hợp lệ sẽ hiển thị một thông báo màu đỏ thay vào đó: “Chữ ký của trình cài đặt này không hợp lệ”, kèm lý do của Windows, và “Trình cài đặt có thể đã bị thay đổi sau khi được ký.” |
| **Vị trí cài đặt** | Thư mục mà ứng dụng được cài vào, với **Duyệt** để chọn thư mục. Ban đầu đây là thư mục chứa ứng dụng trên PC cũ. Ứng dụng từ Windows 32 bit thì ban đầu nằm trong `Program Files (x86)`, nơi Windows 64 bit lưu ứng dụng 32 bit. Nếu không thể dùng thư mục đó ở đây, một thông báo màu đỏ bên dưới sẽ cho biết lý do. |
| **Dung lượng cần thiết** | Lần cài đặt cần bao nhiêu dung lượng trên ổ đĩa đó, và còn trống bao nhiêu, chẳng hạn “tối đa 824,0 MB trên C: (còn trống 356,0 GB)”. Con số này tính cả gói khi được giải nén lẫn các tệp của ứng dụng. |

Tên nhà phát hành ở dòng đầu tiên lấy từ gói, nên bất kỳ ai cũng có thể đã viết nó. Hàng **Trình cài đặt** lấy từ Windows, và đó là hàng đáng tin cậy.

<a id="when-the-location-cannot-be-used"></a>

### Khi không thể dùng vị trí cài đặt

Nếu không thể dùng thư mục trong **Vị trí cài đặt**, một thông báo màu đỏ bên dưới sẽ cho biết lý do. **Cài đặt** và **Xem trước thay đổi** bị làm mờ cho đến khi bạn chọn thư mục khác, và một dòng bên cạnh các nút đó ghi “Cài đặt và Xem trước thay đổi bị tắt cho đến khi có thể dùng Vị trí cài đặt.”

Lý do thường gặp nhất là thư mục đã có tệp. Một thư mục đã có tệp chỉ được dùng nếu một lần cài đặt trước đó từ WinAppMigrator đã đặt các tệp đó vào, để tệp của ứng dụng khác không bao giờ bị thay thế:

| Thông báo | Ý nghĩa | Cách xử lý |
| --- | --- | --- |
| Tên của ứng dụng, rồi “đã được cài đặt trong thư mục này, nên không thể đặt bản sao này ở đây.” | Ứng dụng đã được cài đặt trên PC này, thường là bởi trình cài đặt riêng của nó: chương trình chính của ứng dụng nằm trong thư mục đó, hoặc danh sách ứng dụng đã cài đặt của Windows ghi ứng dụng ở đó. Điều này xảy ra với những ứng dụng mà nhiều PC đã có sẵn, chẳng hạn trình duyệt web, trình phát đa phương tiện hoặc Office. | Trước tiên, hãy gỡ cài đặt bản sao trên PC này trong Cài đặt của Windows, mục **Ứng dụng**, rồi mở lại trình cài đặt. Hoặc nhập một thư mục mới, trống vào **Vị trí cài đặt** để cài đặt bản sao này bên cạnh bản kia. |
| Tên của một ứng dụng khác, rồi “đã được cài đặt trong thư mục này”, và rằng “không thể đặt” ứng dụng này “ở đây”. | Danh sách ứng dụng đã cài đặt của Windows ghi một ứng dụng khác ở thư mục đó, chẳng hạn một phiên bản Office khác. | Hãy nhập một thư mục mới, trống vào **Vị trí cài đặt**. Hoặc, nếu bạn không còn cần ứng dụng kia, trước tiên hãy gỡ cài đặt nó trong Cài đặt của Windows, mục **Ứng dụng**, rồi mở lại trình cài đặt. |
| “Thư mục này đã có các tệp không do WinAppMigrator cài đặt.” | Có thứ khác trong thư mục, và không có ứng dụng đã cài đặt nào được ghi ở đó. Đó có thể là tệp của ứng dụng khác. | Hãy nhập một thư mục mới, trống vào **Vị trí cài đặt**. Nếu các tệp là phần còn sót lại từ một ứng dụng bạn đã gỡ cài đặt, bạn có thể xóa thư mục, rồi mở lại trình cài đặt. |

Với bất kỳ lý do nào khác, thông báo ghi “Không thể dùng thư mục này:”, lý do, và “Hãy nhập một thư mục khác ở trên.” Các lý do là:

| Lý do trên màn hình | Cách xử lý |
| --- | --- |
| “vị trí này phải là đường dẫn đầy đủ” | Hãy nhập một đường dẫn bắt đầu bằng ký tự ổ đĩa, chẳng hạn `C:\Program Files\Fabrikam Data Sync`. |
| “vị trí này không thể là toàn bộ ổ đĩa” | Hãy dùng một thư mục trên ổ đĩa, chứ không phải chính ổ đĩa. |
| “vị trí này không thể là thư mục Windows” | Hãy chọn một thư mục nằm ngoài `C:\Windows`. |
| “thư mục này chứa nhiều thứ khác nằm cạnh nhau, nên ứng dụng cần một thư mục bên trong nó” | Hãy thêm một thư mục cho ứng dụng, chẳng hạn `C:\Program Files\Fabrikam Data Sync` thay vì `C:\Program Files`. |
| “đây là thư mục thuộc về Windows, hoặc là thư mục có nội dung tự chạy hoặc tự được tải” | Hãy chọn một thư mục thông thường. Ví dụ, thư mục Khởi động sẽ bị từ chối. |
| “vị trí này phải là một đường dẫn thông thường trên một ổ đĩa của PC này” | [Thư mục dùng chung trên mạng](Glossary#network-share) bị từ chối, và tên mà Windows sẽ hiểu thành thứ khác cũng vậy. |
| rằng thư mục “là liên kết tới nơi khác, và người khác có thể thay đổi nơi liên kết trỏ tới” | Hãy dùng thư mục mà [liên kết](Glossary#link) trỏ tới. |
| “đây là nơi các lần cài đặt lưu bản ghi” | Hãy chọn một thư mục nằm ngoài `%ProgramData%\WinAppMigrator`. |

Nếu thư mục có tệp và một lần cài đặt trước đó từ WinAppMigrator **có** được ghi nhận ở đó, trình cài đặt sẽ dùng thư mục đó, và cảnh báo: “Thư mục này đã có tệp. Các tệp trùng tên sẽ bị thay thế và sẽ được khôi phục nếu bạn gỡ cài đặt.”

Khi một thư mục được chấp nhận, cửa sổ sẽ thay thế tệp, giá trị sổ đăng ký, dịch vụ, tác vụ, lối tắt hoặc biến trùng tên, như `--force` làm trên [dòng lệnh](Command-Line#what---force-does). Những gì đã thuộc về một ứng dụng khác, chẳng hạn dịch vụ hoặc loại tệp của ứng dụng đó, vẫn thuộc về ứng dụng đó.

Nếu ổ đĩa có vẻ thiếu dung lượng, một ghi chú màu hổ phách sẽ ghi “Có thể không đủ dung lượng ở đây” và dung lượng cần thiết. Gói có thể được giải nén trên ổ đĩa khác, nhưng các tệp của ứng dụng vẫn phải vừa trên ổ đĩa này. Bản thân quá trình cài đặt sẽ kiểm tra lại dung lượng trước khi bắt đầu.

<a id="an-earlier-install-that-never-finished"></a>

### Lần cài đặt trước chưa hoàn tất

Nếu một lần cài đặt trước đó bởi bất kỳ trình cài đặt nào trong số này bị gián đoạn, do sự cố hoặc mất điện, một ghi chú màu hổ phách sẽ ghi “Một lần cài đặt” ứng dụng đó “trước đây chưa hoàn tất. Thao tác Cài đặt sẽ hoàn tác lần đó trước.” Việc đó tự động diễn ra khi bạn bấm **Cài đặt**.

<a id="what-this-sets-up-beyond-files"></a>

## Những gì được thiết lập ngoài tệp

Thẻ này là danh sách mà bạn đồng ý khi bấm **Cài đặt**. Bên dưới tiêu đề, thẻ ghi:

> Tất cả những việc này được thực hiện với quyền quản trị viên. Nếu cài đặt thất bại, những gì đã thay đổi sẽ được khôi phục lại, và bạn sẽ được thông báo về bất kỳ điều gì không thể khôi phục lại. Bạn cũng có thể hoàn tác lần cài đặt này sau đó trong Cài đặt, mục Ứng dụng đã cài đặt. Các tệp bị thay thế sẽ được khôi phục, còn các thư viện thời gian chạy đã cài đặt thì được giữ lại.

Mỗi hàng có một tên, một nhãn ở bên phải cho biết đó là loại gì, và một dòng cho biết nó làm gì. Nhãn có màu xanh lam thông thường là bình thường. Nhãn màu **hổ phách** đánh dấu thứ chạy với toàn quyền, chẳng hạn dịch vụ chạy dưới LocalSystem, trình điều khiển, hoặc quy tắc tường lửa cho phép kết nối đến. Hãy xem kỹ những mục đó.

Nếu danh sách trống, thẻ ghi “Không có gì: trình cài đặt chỉ sao chép tệp và các thiết đặt trong khóa sổ đăng ký riêng của ứng dụng.”

Danh sách được xác định cho vị trí trong ô. Nếu bạn thay đổi vị trí, thẻ sẽ ghi “Đang xác định những gì sẽ được thiết lập tại vị trí này…” trong giây lát, và **Cài đặt** sẽ chờ.

<a id="what-each-label-means"></a>

### Ý nghĩa của từng nhãn

| Nhãn | Đây là gì | Ví dụ về dòng bên dưới |
| --- | --- | --- |
| **Dịch vụ** | Một [dịch vụ](Glossary#service): chương trình mà Windows chạy trong nền | “Chạy FabrikamSync.exe dưới tài khoản NT AUTHORITY\LocalService, khởi động ngay và mỗi khi PC này khởi động”. Các dịch vụ khác ghi “khởi động ngay và một lúc sau mỗi lần PC này khởi động”, “chỉ khởi động khi có thứ gì đó yêu cầu”, hoặc “đã bị tắt”. |
| **Trình điều khiển** | Một [trình điều khiển](Glossary#driver) thuộc về ứng dụng | “Tải” một tệp “vào chính Windows”. Luôn có màu hổ phách. Chưa có bài kiểm tra nào của dự án tải trình điều khiển. |
| **Tác vụ đã lên lịch** | Một [tác vụ đã lên lịch](Glossary#scheduled-task) | “Chạy với tư cách SYSTEM”, hoặc với tư cách một tài khoản. “với quyền cao nhất có thể” được đánh dấu màu hổ phách. Tác vụ từng chạy với tư cách một người trên PC cũ sẽ chạy với tư cách người cài đặt nó. |
| **Tự khởi động** | Một [mục khởi động](Glossary#startup-item) | “Chạy khi bạn đăng nhập”, “Chạy khi bất kỳ ai đăng nhập”, hoặc “Một lối tắt trong thư mục Khởi động, cho bạn” |
| **Lối tắt** | Một [lối tắt](Glossary#shortcut) trên màn hình nền hoặc trong menu Bắt đầu | “Mở Fabrikam.exe, cho mọi người trên PC này” hoặc “cho bạn” |
| **PATH** | Một mục được thêm vào [biến Path](Glossary#path-variable) | “Thêm vào Path” và “Thư mục cài đặt\bin”. Mục này được thêm vào những gì đã có, không bao giờ thay thế chúng. |
| **Biến môi trường** | Một [biến môi trường](Glossary#environment-variable) | “Đặt thành” giá trị của biến |
| **Phần mở rộng shell** | Mã được tải vào Chương trình Quản lý Tệp | “Tải” một tệp “vào Chương trình Quản lý Tệp, cho mọi người trên PC này”. Màu hổ phách. |
| **Lệnh menu** | Một lệnh trên menu chuột phải | “Thêm một lệnh vào menu của” một loại tệp. “Thay đổi điều xảy ra khi mở một mục thuộc loại này” có màu hổ phách. |
| **Loại liên kết** | Một [giao thức liên kết](Glossary#link-protocol), chẳng hạn `fabrikam:` | “Các liên kết thuộc loại này được mở bằng ứng dụng” |
| **Loại tệp** | Một [loại tệp](Glossary#file-type), chẳng hạn `.fds` | “Các tệp thuộc loại này được mở bằng ứng dụng” |
| **COM** | Các [lớp COM](Glossary#com-class) mà chương trình khác có thể khởi động | “COM trong” một tệp, kèm số lượng thiết đặt. “Quyền COM” có màu hổ phách. |
| **Quy tắc tường lửa** | Một [quy tắc tường lửa](Glossary#firewall-rule) cho một trong các chương trình của ứng dụng | “Cho phép kết nối đến trên cổng 8080 cho FabrikamSync.exe, trên mọi mạng”. Việc cho phép kết nối đến có màu hổ phách. |
| **Nguồn nhật ký sự kiện** | Một tên mà ứng dụng dùng để ghi vào nhật ký sự kiện của Windows | “Có thể ghi vào nhật ký Application, với thông báo lấy từ các tệp riêng của ứng dụng” |
| **Phông chữ** | Một phông chữ trong thư mục cài đặt | “Được tải từ thư mục cài đặt khi bạn đăng nhập” |
| **Trình điều khiển ODBC** | Một trình điều khiển cơ sở dữ liệu cho [ODBC](Glossary#odbc) | “Được tải vào các chương trình dùng ODBC để kết nối cơ sở dữ liệu”. Màu hổ phách. |
| **Nguồn dữ liệu ODBC** | Một kết nối cơ sở dữ liệu đã lưu | “Một kết nối cơ sở dữ liệu, cho mọi người trên PC này” |
| **Thiết đặt tương thích** | Cách Windows khởi động một số chương trình của ứng dụng | “Cách các chương trình của ứng dụng khởi động” |
| **Ứng dụng mặc định** | Một mục cho phép bạn chọn ứng dụng trong Ứng dụng mặc định | “Có thể được chọn trong Ứng dụng mặc định. Không có gì được chuyển sang dùng ứng dụng này” |
| **Tên chương trình** | Một tên mà hộp thoại Chạy và các chương trình khác có thể dùng để tìm ứng dụng | “Hộp thoại Chạy và các chương trình khác có thể tìm thấy ứng dụng theo tên này” |
| **Thiết đặt khác** | Các giá trị [sổ đăng ký](Glossary#registry) nằm ngoài các khóa riêng của ứng dụng và ngoài các loại ở trên | “3 thiết đặt nằm ngoài các khóa riêng của ứng dụng”, “Trong” các khóa. Màu hổ phách nếu có giá trị nào nằm trong khóa của Microsoft. |

Một số thứ chỉ có thể được đánh giá khi quá trình cài đặt đến bước đó, chẳng hạn một dịch vụ cùng tên đã thuộc về ứng dụng khác. Những mục đó vẫn nằm trong danh sách như đã yêu cầu. Khi quá trình cài đặt đến đó, trình cài đặt sẽ để nguyên chúng, và cho biết điều đó.

<a id="notes-and-what-is-left-out"></a>

### Ghi chú và những gì bị loại trừ

Bên dưới các hàng, các ghi chú màu hổ phách cho biết những điều bạn nên biết, ví dụ:

- rằng thông tin xác thực không được đưa vào trình cài đặt, nên bạn có thể phải nhập lại;
- rằng tùy chọn **Dữ liệu giấy phép và kích hoạt** hoặc **Dữ liệu người dùng** đã tắt khi trình cài đặt được tạo, nên dữ liệu đó không có trong trình cài đặt;
- những [chứng chỉ](Glossary#certificate) mà ứng dụng có vẻ dựa vào, vốn không bao giờ được cài đặt;
- những gì người tạo trình cài đặt đã loại trừ, và những gì họ đã thêm vào;
- rằng ứng dụng có thể không hoạt động ở đây: “Ứng dụng này từng nằm trên Windows bản dựng” mới hơn, hoặc rằng Windows sẽ chạy một số chương trình của ứng dụng “thông qua trình mô phỏng”;
- rằng các tệp từ Tài liệu hoặc Trò chơi đã lưu sẽ “được đặt trong” thư mục cài đặt “thay vào đó, trong một thư mục tên là _external”, vì thư mục đó của PC này nằm ngoài hồ sơ người dùng của bạn.

Tiếp theo, bằng màu hổ phách, là từng thứ mà các quy tắc an toàn loại trừ, kèm lý do. 20 mục đầu tiên được hiển thị đầy đủ. Các mục còn lại nằm dưới “… mục khác bị loại trừ”, mà bạn có thể mở ra. Phần còn lại của lần cài đặt vẫn tiếp tục mà không có chúng. Những gì bị từ chối, và lý do, có trong [Những gì được đưa vào trình cài đặt](What-Goes-in-an-Installer#what-the-installer-refuses).

Nếu hoàn toàn không thể đặt gói vào vị trí đã chọn, một thông báo màu đỏ sẽ ghi “Không thể đặt gói này tại vị trí này” và lý do.

<a id="options"></a>

## Tùy chọn

Thẻ này chỉ xuất hiện khi trình cài đặt có chứa dữ liệu người dùng hoặc dữ liệu giấy phép. Mỗi ô ban đầu đều được chọn.

| Tùy chọn | Nội dung |
| --- | --- |
| **Dữ liệu người dùng** | “Bật: thiết đặt, Tài liệu và Trò chơi đã lưu được đưa vào các thư mục thông thường của người đang đăng nhập. Tắt: tất cả được đặt trong thư mục cài đặt, và không có gì được thiết lập để chạy từ đó.” Nếu trình cài đặt cho biết nó chứa dữ liệu của ai, một dòng bên dưới sẽ ghi “Đây là dữ liệu của” tài khoản đó. |
| **Giấy phép** | “Các tệp kích hoạt và giấy phép, cùng các giá trị sổ đăng ký chứa giấy phép. Một số có thể cần được kích hoạt lại ở đây.” |

Khi bạn thay đổi một ô, danh sách ở trên thay đổi theo ngay lập tức.

Dữ liệu người dùng luôn được chuyển cho **người đang đăng nhập** vào Windows, kể cả khi tên của một quản trị viên khác đã được nhập tại lời nhắc UAC. Nếu trình cài đặt không xác định được đó là ai, nó sẽ cho biết điều đó, loại trừ các thiết đặt theo người dùng, và đặt dữ liệu người dùng trong thư mục cài đặt thay vì đoán. Nếu không có ai đăng nhập vào phiên đó, dữ liệu người dùng và thiết đặt theo người dùng được chuyển cho tài khoản mà trình cài đặt đang chạy dưới đó, và trình cài đặt cũng cho biết điều này.

<a id="dependencies"></a>

## Phần phụ thuộc

“Những gì ứng dụng này cần và cách cung cấp từng thành phần.” Mỗi [thư viện thời gian chạy](Glossary#runtime) có một nhãn:

| Nhãn | Ý nghĩa |
| --- | --- |
| **Đã bao gồm** | Nằm bên trong trình cài đặt. Được cài đặt mà không cần internet, ngoại trừ DirectX End-User Runtime, vì trình cài đặt của nó tải thư viện về từ Microsoft. Trong số này, chỉ thư viện thời gian chạy Visual C++ x64 đã thực sự được cài đặt trong các bài kiểm tra của dự án. |
| **Được cài đặt bằng winget:** và một id | Không thể tải xuống thư viện này khi trình cài đặt được tạo, nên trình cài đặt dùng [winget](Glossary#winget), vốn cần internet. Cách này chưa được chạy trong các bài kiểm tra của dự án. |
| **Tính năng Windows: NetFx3** | .NET Framework 3.5, được bật dưới dạng [tính năng Windows](Glossary#windows-feature). Việc này có thể cần internet. Cách này chưa được chạy trong các bài kiểm tra của dự án. |
| **Có sẵn trong Windows** | .NET Framework 4, vốn đã có sẵn trong Windows 10 và 11 |
| **Đi kèm ứng dụng** | Ứng dụng mang theo bản sao riêng |
| **Cài đặt thủ công** | Ở đây không có trình cài đặt cho thư viện này. Hãy tự cài đặt. |

Nếu không có thư viện nào, thẻ ghi “Không phát hiện phần phụ thuộc thời gian chạy bổ sung nào.” Xem [Thư viện thời gian chạy](Runtimes).

<a id="preview-changes"></a>

## Xem trước thay đổi

**Xem trước thay đổi** xác định chính xác những gì một lần cài đặt sẽ làm, mà không thay đổi gì. Thao tác này kiểm tra dung lượng, giải nén gói vào một thư mục riêng, xác định các thay đổi, rồi xóa thư mục đó. Thẻ kết quả có tiêu đề “Xem trước: chưa có gì bị thay đổi”, với **Thay đổi dự kiến**, và danh sách các thay đổi, trong đó dịch vụ, tác vụ và lối tắt đứng đầu. Cửa sổ liệt kê 300 mục đầu tiên, rồi cho biết còn bao nhiêu mục nữa.

Bản xem trước không kiểm tra hàm băm của từng tệp. Hàm băm của toàn bộ gói đã được kiểm tra khi cửa sổ mở ra.

<a id="install"></a>

## Cài đặt

**Cài đặt** bị làm mờ cho đến khi có thể dùng vị trí và danh sách là cho vị trí đó. Nếu hoàn toàn không thể đặt gói vào vị trí đó, một dòng bên cạnh nút ghi “Cài đặt bị tắt vì không thể đặt gói này tại vị trí này”, và lý do được hiển thị màu đỏ trong **Những gì được thiết lập ngoài tệp**. Khi bạn bấm nút này:

1. Trình cài đặt xác định danh sách thêm một lần nữa. Nếu những gì nó sẽ làm đã thay đổi kể từ khi bạn xem, nó không cài đặt gì, hiển thị danh sách mới và ghi “Những gì trình cài đặt sẽ làm ở đây đã thay đổi, nên không có gì được cài đặt. Hãy xem lại danh sách bên dưới, rồi bấm Cài đặt lần nữa nếu đó là điều bạn muốn.”
2. Trình cài đặt kiểm tra dung lượng trống trên mọi ổ đĩa mà nó ghi vào.
3. Trình cài đặt giải nén gói vào một thư mục mà chỉ quản trị viên mới dùng được, kiểm tra hàm băm của từng tệp khi giải nén.
4. Trình cài đặt cài đặt, ghi từng thay đổi vào [bản ghi cài đặt](Glossary#journal) trước khi thực hiện, để có thể hoàn tác.
5. Trình cài đặt đọc lại mọi tệp đã cài đặt và kiểm tra hàm băm.
6. Trình cài đặt khởi động các dịch vụ được đặt để tự khởi động.

Ở dưới cùng, bạn thấy trình cài đặt đang làm gì, số đếm nếu có, thời gian đã mất, thanh tiến trình và **Hủy**. Các bước bạn có thể thấy là “Đang chuẩn bị…”, “Đang giải nén”, “Đang kiểm tra gói”, “Đang hoàn tác một lần cài đặt trước chưa hoàn tất”, “Đang cài đặt thư viện thời gian chạy”, “Đang sao chép tệp”, “Đang thiết lập dịch vụ”, “Đang thiết lập trình điều khiển”, “Đang ghi thiết đặt sổ đăng ký”, “Đang khôi phục dữ liệu giấy phép”, “Đang thiết lập tác vụ đã lên lịch”, “Đang tạo lối tắt”, “Đang đặt biến môi trường”, “Đang thêm ứng dụng vào Cài đặt” và “Đang kiểm tra những gì đã cài đặt”.

**Hủy** hiển thị “Đang hủy sau bước hiện tại…”, rồi “Đang khôi phục lại những gì đã thay đổi”. Đóng cửa sổ trong khi đang cài đặt có tác dụng giống như **Hủy**: cửa sổ vẫn mở cho đến khi mọi thứ được khôi phục lại. Nếu chưa có gì bị thay đổi, cửa sổ ghi “Đã hủy. Không có gì bị thay đổi.”

<a id="the-hash-check"></a>

## Kiểm tra hàm băm

Trình cài đặt kiểm tra [hàm băm](Glossary#hash) ba lần:

1. **Khi cửa sổ mở ra**: hàm băm của toàn bộ gói. Bản sao bị cắt ngắn, hoặc bị chỉnh sửa sau khi được tạo, sẽ bị từ chối trước khi bất cứ thứ gì bên trong được đọc.
2. **Khi giải nén**: hàm băm của từng tệp, trước khi bất cứ thứ gì được ghi vào vị trí của nó.
3. **Sau khi cài đặt**: trình cài đặt đọc lại từng tệp đã cài đặt và kiểm tra lần nữa.

Hàm băm phát hiện hư hỏng, và các thay đổi của bất kỳ ai không đồng thời cập nhật hàm băm. Hàm băm không chứng minh ai đã tạo ra trình cài đặt. Chỉ [chữ ký](Glossary#signature) mới làm được điều đó.

<a id="the-result"></a>

## Kết quả

Thẻ kết quả xuất hiện ở trên cùng, với tiêu đề “Kết quả cài đặt”.

| Thông báo | Ý nghĩa |
| --- | --- |
| “Ứng dụng đã được cài đặt và đã vượt qua kiểm tra hàm băm.” | Mọi thứ đều ổn. Nếu có gì bị các quy tắc loại trừ, thông báo sẽ thêm “ngoại trừ những mục được liệt kê là bị loại trừ bên dưới.” |
| “Cài đặt đã bị hủy và những gì đã thay đổi đã được khôi phục lại.” | Bạn đã hủy. |
| “Cài đặt thất bại và những gì đã thay đổi đã được khôi phục lại.” | Đã có lỗi. Những gì đã thay đổi đã được khôi phục lại, ngoại trừ thư viện thời gian chạy đã cài đặt. Mọi thứ được liệt kê màu hổ phách bên dưới đều được để lại, kèm lý do, chẳng hạn một dịch vụ chỉ biến mất khi PC khởi động lại. |
| “Cài đặt thất bại và không phải mọi thay đổi đều có thể được khôi phục lại. Những gì còn lại được liệt kê bên dưới, và lần cài đặt tiếp theo sẽ thử lại.” | Hãy đọc danh sách. Chạy lại một trình cài đặt sẽ thử khôi phục lại lần nữa. |
| “Quá trình cài đặt chưa hoàn tất trọn vẹn. Hãy xem lại các sự cố bên dưới.” | Hãy đọc các thông báo bên dưới. |

Tiếp theo là ba ô: **Thay đổi đã thực hiện** (hoặc **Thay đổi đã được khôi phục lại** sau khi thất bại), **Cảnh báo** và **Lỗi**. Các nhãn ghi **Hàm băm của gói hợp lệ** và **Tệp đã cài đặt hợp lệ**, hoặc **KHÔNG HỢP LỆ** màu đỏ. Tối đa năm cảnh báo và mọi lỗi được hiển thị dưới dạng thông báo. “Một phần phụ thuộc cần khởi động lại để hoàn tất. Hãy khởi động lại Windows, rồi mở ứng dụng.” có nghĩa là một thư viện thời gian chạy đã yêu cầu khởi động lại.

**Thay đổi**, kèm số lượng, mở danh sách những gì đã thực hiện, tối đa 300 mục. Nếu quá trình cài đặt chưa hoàn tất trọn vẹn, đường dẫn của [nhật ký](Glossary#log) được hiển thị dưới dạng **Nhật ký**.

Sau khi cài đặt thành công, các nút ở dưới cùng trở thành **Đóng** và **Mở thư mục cài đặt**.

<a id="where-things-end-up"></a>

## Mọi thứ được đặt ở đâu

- **Tệp chương trình** được đặt trong vị trí cài đặt.
- **Dữ liệu người dùng**, khi ô đó được chọn, được đưa vào AppData, Tài liệu, Trò chơi đã lưu của chính tài khoản đang đăng nhập, và vào ProgramData. Thư mục Tài liệu hoặc Trò chơi đã lưu mà PC này lưu ngoài hồ sơ người dùng của bạn sẽ không được ghi vào. Các tệp đó được đặt trong vị trí cài đặt, trong `_external`, và danh sách cho biết điều này trước tiên. Khi ô đó không được chọn, toàn bộ dữ liệu người dùng được đặt trong vị trí cài đặt.
- **Lối tắt** được đặt trên màn hình nền và trong menu Bắt đầu của PC này: của bạn, hoặc của mọi người nếu trước đây chúng nằm ở đó.
- **Path** chỉ được thêm các mục của ứng dụng. Không có gì khác trong đó thay đổi.
- **Những gì đã thuộc về ứng dụng khác ở đây vẫn thuộc về ứng dụng đó.** Loại tệp, loại liên kết, lớp COM, dịch vụ, nguồn nhật ký sự kiện, phông chữ hoặc bất cứ thứ gì tương tự đã thuộc về thứ khác sẽ được để nguyên, và trình cài đặt cho biết điều đó. Ví dụ, ứng dụng muốn mở liên kết `http` sẽ phải được đặt lại làm mặc định trong Cài đặt.
- **Cài đặt** có thêm một mục cho ứng dụng, để bạn có thể gỡ cài đặt nó như mọi ứng dụng khác. Xem [Hoàn tác một lần cài đặt](Undoing-an-Install).

<a id="common-reasons-it-refuses"></a>

## Các lý do từ chối thường gặp

| Bạn thấy gì | Lý do | Cách xử lý |
| --- | --- | --- |
| “ứng dụng này không thể chạy trên PC này, nên không có gì bị thay đổi” | Các chương trình của ứng dụng được tạo cho loại bộ xử lý khác, hoặc cần phiên bản Windows mới hơn | Hãy cài đặt trên một PC giống với PC cũ |
| “không đủ dung lượng cho việc này, nên không có gì bị thay đổi” | Không đủ dung lượng trống trên một ổ đĩa mà trình cài đặt ghi vào | Hãy giải phóng dung lượng, hoặc chọn vị trí trên ổ đĩa khác |
| “một lần cài đặt hoặc gỡ cài đặt khác đang chạy” | Mỗi lần chỉ chạy được một | Hãy đợi cho đến khi hoàn tất, rồi thử lại |
| Một tệp “có tên mà phiên bản Windows này mở như một thiết bị chứ không phải tệp” | Ứng dụng có một tệp tên giống `aux.h`, mà Windows trước Windows 11 không thể chứa | Hãy cài đặt trên Windows 11 |
| Một thư mục dữ liệu như `%AppData%` nằm dưới một liên kết | Trình cài đặt sẽ không ghi xuyên qua liên kết | Hãy bỏ chọn **Dữ liệu người dùng** và **Giấy phép**, rồi cài đặt mà không có chúng |

Các lý do khác có trong [Khắc phục sự cố](Troubleshooting#installing).
