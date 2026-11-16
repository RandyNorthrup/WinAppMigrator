# Words of the parts every window shares: the search box, folder fields, and the Language menu.


## Search and folder fields

ui-clear-search = Xóa tìm kiếm
ui-browse = Duyệt
# A tooltip on Browse. { $field } is the field's own label, such as "Install Location".
ui-browse-tooltip = Chọn thư mục cho { $field }

## Headings that open and close what is below them

# What a screen reader says for such a heading. { $title } is the heading, { $summary } a short count of what is below,
# such as "3 of 12 chosen"; $state is "open" or "closed", and only the words for each are translated.
ui-disclosure =
    { $state ->
        [open] { $title }, { $summary }, đang mở
       *[closed] { $title }, { $summary }, đang đóng
    }

## The Language menu. Its list shows each language by its own name, which is not translated.

ui-language = Ngôn ngữ
ui-machine-translated = Bản dịch này do máy thực hiện và có thể có lỗi.
ui-report-translation-problem = Báo cáo lỗi bản dịch

## When a window cannot open

# Shown in a message box when the window cannot be drawn.
ui-needs-direct3d-12 = Chương trình này cần Direct3D 12 để vẽ cửa sổ, nhưng trình điều khiển đồ họa cũ hoặc máy ảo có thể không hỗ trợ.
