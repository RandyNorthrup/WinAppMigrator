# Words of the parts every window shares: the search box, folder fields, and the Language menu.


## Search and folder fields

ui-clear-search = ล้างการค้นหา
ui-browse = เรียกดู
# A tooltip on Browse. { $field } is the field's own label, such as "Install Location".
ui-browse-tooltip = เลือกโฟลเดอร์สำหรับ { $field }

## Headings that open and close what is below them

# What a screen reader says for such a heading. { $title } is the heading, { $summary } a short count of what is below,
# such as "3 of 12 chosen"; $state is "open" or "closed", and only the words for each are translated.
ui-disclosure =
    { $state ->
        [open] { $title }, { $summary }, ขยายอยู่
       *[closed] { $title }, { $summary }, ยุบอยู่
    }

## The Language menu. Its list shows each language by its own name, which is not translated.

ui-language = ภาษา
ui-machine-translated = คำแปลนี้สร้างขึ้นด้วยเครื่อง และอาจมีข้อผิดพลาด
ui-report-translation-problem = รายงานปัญหาการแปล

## When a window cannot open

# Shown in a message box when the window cannot be drawn.
ui-needs-direct3d-12 = โปรแกรมนี้ต้องใช้ Direct3D 12 ในการแสดงหน้าต่าง ซึ่งไดรเวอร์กราฟิกรุ่นเก่า หรือเครื่องเสมือนอาจไม่มี
