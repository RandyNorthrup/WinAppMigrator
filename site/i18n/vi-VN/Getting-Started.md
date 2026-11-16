<a id="getting-started"></a>

# Bắt đầu sử dụng

Trang này hướng dẫn bạn từ lúc tải xuống cho đến màn hình đầu tiên của ứng dụng.

<a id="what-you-need"></a>

## Những gì bạn cần

- **Windows 10 hoặc Windows 11, 64 bit.** Ứng dụng là một chương trình x64. Windows 11 trên Arm chạy ứng dụng thông qua [mô phỏng](Glossary#emulation). Windows 10 trên Arm không thể chạy chương trình x64, còn Windows 32 bit hoàn toàn không thể chạy ứng dụng này. Windows 10 và 11 dành cho máy tính để bàn trên x64 chưa được chạy trong các bài kiểm tra của dự án, vốn dùng Windows Server 2022 và 2025 cùng Windows 11 trên Arm. Xem [Yêu cầu và kiểm thử](Requirements-and-Testing).
- **Quyền quản trị viên** trên PC. Ứng dụng luôn yêu cầu quyền này khi khởi động. Xem [Quyền quản trị viên](Glossary#administrator-rights).
- **Direct3D 12**, thứ Windows dùng để vẽ cửa sổ của ứng dụng. Windows 10 và 11 thường có sẵn. Trình điều khiển đồ họa cũ hoặc máy ảo có thể không có. Xem [Direct3D 12](Glossary#direct3d-12).
- **Dung lượng trống trên một ổ đĩa** cho trình cài đặt mà bạn tạo. Ứng dụng cho bạn biết cần bao nhiêu trước khi bắt đầu.
- **Internet trên PC cũ là không bắt buộc.** Nếu có internet khi bạn tạo trình cài đặt, ứng dụng sẽ tải xuống các [thư viện thời gian chạy](Glossary#runtime) của Microsoft mà nó có thể đưa vào, rồi đặt chúng vào bên trong trình cài đặt. Xem [Thư viện thời gian chạy](Runtimes).

Thông tin chi tiết hơn, cùng những gì đã và chưa được kiểm thử, có trong [Yêu cầu và kiểm thử](Requirements-and-Testing).

<a id="download"></a>

## Tải xuống

Hãy truy cập [bản phát hành mới nhất](https://github.com/RandyNorthrup/WinAppMigrator/releases/latest). Bản phát hành có ba tệp:

| Tệp | Đây là gì | Chọn tệp này nếu |
| --- | --- | --- |
| `WinAppMigrator-v0.7.1.exe` | Chính ứng dụng. Một tệp duy nhất, không cần cài đặt. | Bạn chỉ muốn sử dụng. Đây là tệp mà hầu hết mọi người cần. |
| `WinAppMigrator-v0.7.1-windows-x64.zip` | Cùng ứng dụng đó, dưới dạng `winappmigrator.exe`, kèm README, nhật ký thay đổi, giấy phép và giấy phép của bên thứ ba | Bạn cũng muốn có các giấy tờ đó |
| `SHA256SUMS.txt` | Tổng kiểm tra của hai tệp ở trên | Bạn muốn kiểm tra bản tải xuống |

Phần trình cài đặt nằm bên trong mọi trình cài đặt, tức [phần khởi chạy của trình cài đặt (stub)](Glossary#installer-stub), không bao giờ được phát hành riêng. Phần này đã nằm sẵn bên trong ứng dụng.

<a id="check-the-download"></a>

## Kiểm tra bản tải xuống

Việc này không bắt buộc, nhưng đáng làm. Mỗi bước kiểm tra dưới đây là độc lập. Hãy chạy chúng trong PowerShell, trong thư mục mà bạn đã lưu tệp.

**1. Tổng kiểm tra.** [Hàm băm](Glossary#hash) giống như dấu vân tay của một tệp. Chỉ cần một byte thay đổi, hàm băm cũng thay đổi.

```powershell
Get-FileHash .\WinAppMigrator-v0.7.1.exe -Algorithm SHA256
Get-Content .\SHA256SUMS.txt
```

Dãy số dài mà `Get-FileHash` in ra phải khớp với dãy trên dòng của tệp đó. `Get-FileHash` in chữ hoa, còn tệp dùng chữ thường. Điều đó không sao: hãy so sánh các chữ cái và chữ số.

**2. Chữ ký.** [Chữ ký](Glossary#signature) cho biết ai đã tạo ra tệp, và tệp chưa bị thay đổi kể từ đó.

```powershell
$signature = Get-AuthenticodeSignature .\WinAppMigrator-v0.7.1.exe
$signature.Status                     # should say Valid
$signature.SignerCertificate.Subject  # should include CN=Randy Northrup
```

Bạn cũng có thể bấm chuột phải vào tệp, chọn **Thuộc tính** và xem tab **Chữ ký số**. Phiên bản 0.5.0 là bản phát hành đầu tiên có ứng dụng được ký. Các bản tải xuống từ những bản phát hành trước đó không được ký.

**3. Nơi tệp được tạo.** Mỗi bản phát hành từ 0.5.0 trở đi đều ghi lại rằng chính quy trình phát hành của GitHub đã tạo ra nó, từ kho mã này. Bản ghi này được gọi là [chứng thực](Glossary#attestation). Nếu bạn có [GitHub CLI](https://cli.github.com/), hãy chạy:

```powershell
gh attestation verify .\WinAppMigrator-v0.7.1.exe --repo RandyNorthrup/WinAppMigrator
```

Cách này cũng dùng được cho tệp zip.

<a id="start-it"></a>

## Khởi động ứng dụng

1. **Bấm đúp** vào `WinAppMigrator-v0.7.1.exe`. Không cần cài đặt gì cả. Ứng dụng chạy từ bất cứ nơi nào bạn đã lưu nó.
2. **SmartScreen.** [SmartScreen](Glossary#smartscreen) của Windows có thể hiển thị màn hình “Windows đã bảo vệ PC của bạn” đối với một tệp mà nó ít gặp. Nếu vậy, hãy chọn **Thêm thông tin** (More info) và kiểm tra rằng nhà phát hành là Randy Northrup trước khi chọn **Vẫn chạy** (Run anyway).
3. **Lời nhắc quản trị viên.** Windows hỏi có cho phép ứng dụng thực hiện thay đổi hay không. Đây là [lời nhắc UAC](Glossary#uac-prompt). Ứng dụng luôn yêu cầu, vì việc đọc ổ đĩa của PC khác, và các tệp mà chỉ quản trị viên mới được đọc, đều cần quyền quản trị viên. Với bản tải xuống đã được ký, lời nhắc ghi **Randy Northrup** là nhà phát hành đã xác minh. Nếu lời nhắc ghi “Nhà phát hành không xác định” cho tệp này, thì đó không phải tệp phát hành đã được ký, nên đừng chạy nó.
4. **Cửa sổ mở ra.** Nếu thay vào đó bạn thấy thông báo rằng không thể mở cửa sổ và cần Direct3D 12, hãy xem [Khắc phục sự cố](Troubleshooting#the-window-could-not-open). Ứng dụng không có cách nào chạy mà không có cửa sổ.

<a id="first-run"></a>

## Lần chạy đầu tiên

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/app-pick-dark.png">
    <img alt="Ứng dụng với Fabrikam Data Sync đã được chọn" src="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/app-pick-light.png" width="860">
  </picture>
</p>

- Danh sách các ứng dụng đã cài đặt nằm ở bên trái. Trong khi danh sách đang tải, nhãn nhỏ ở góc trên bên phải ghi **Đang tải**, rồi sau đó là số ứng dụng tìm thấy, chẳng hạn **5 ứng dụng**.
- Cửa sổ khởi động ở chế độ sáng hoặc tối, theo chế độ mà Windows đang dùng. Nút mặt trời hoặc mặt trăng ở góc trên bên phải dùng để chuyển đổi, và ứng dụng ghi nhớ lựa chọn của bạn. Xem [Các tệp ứng dụng lưu trên PC](Files-It-Keeps-on-a-PC). Khi Windows dùng chủ đề tương phản, ứng dụng dùng màu của chủ đề đó và không có nút này.
- Ứng dụng dùng ngôn ngữ hiển thị của Windows nếu đó là một trong 24 ngôn ngữ của nó, nếu không thì dùng tiếng Anh. **Ngôn ngữ**, ở góc trên bên phải, liệt kê các ngôn ngữ theo tên riêng của chúng và thay đổi cửa sổ ngay lập tức; ứng dụng ghi nhớ lựa chọn của bạn. Tiếng Ả Rập, tiếng Ba Tư, tiếng Urdu và tiếng Do Thái được trình bày từ phải sang trái. Xem [Ngôn ngữ và trợ năng](Languages-and-Accessibility).
- Hãy bấm vào một ứng dụng. Phía bên phải cho thấy những gì sẽ được đưa vào trình cài đặt của ứng dụng đó, và bắt đầu xác định ngay lập tức. Không có gì được tạo cho đến khi bạn bấm **Tạo trình cài đặt**.

Tiếp theo: [Tạo trình cài đặt](Building-an-Installer).
