# Words of the parts every window shares: the search box, folder fields, and the Language menu.


## Search and folder fields

ui-clear-search = 清除搜索
ui-browse = 浏览
# A tooltip on Browse. { $field } is the field's own label, such as "Install Location".
ui-browse-tooltip = 为“{ $field }”选择文件夹

## Headings that open and close what is below them

# What a screen reader says for such a heading. { $title } is the heading, { $summary } a short count of what is below,
# such as "3 of 12 chosen"; $state is "open" or "closed", and only the words for each are translated.
ui-disclosure =
    { $state ->
        [open] { $title }，{ $summary }，已展开
       *[closed] { $title }，{ $summary }，已折叠
    }

## The Language menu. Its list shows each language by its own name, which is not translated.

ui-language = 语言
ui-machine-translated = 此翻译由机器生成，可能存在错误。
ui-report-translation-problem = 报告翻译问题

## When a window cannot open

# Shown in a message box when the window cannot be drawn.
ui-needs-direct3d-12 = 此程序需要 Direct3D 12 来绘制窗口，而旧版显卡驱动程序或虚拟机可能不支持它。
