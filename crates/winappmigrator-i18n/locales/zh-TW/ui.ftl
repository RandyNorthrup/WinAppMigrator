# Words of the parts every window shares: the search box, folder fields, and the Language menu.


## Search and folder fields

ui-clear-search = 清除搜尋
ui-browse = 瀏覽
# A tooltip on Browse. { $field } is the field's own label, such as "Install Location".
ui-browse-tooltip = 選擇「{ $field }」的資料夾

## Headings that open and close what is below them

# What a screen reader says for such a heading. { $title } is the heading, { $summary } a short count of what is below,
# such as "3 of 12 chosen"; $state is "open" or "closed", and only the words for each are translated.
ui-disclosure =
    { $state ->
        [open] { $title }，{ $summary }，已展開
       *[closed] { $title }，{ $summary }，已摺疊
    }

## The Language menu. Its list shows each language by its own name, which is not translated.

ui-language = 語言
ui-machine-translated = 此翻譯由機器產生，可能有錯誤。
ui-report-translation-problem = 回報翻譯問題

## When a window cannot open

# Shown in a message box when the window cannot be drawn.
ui-needs-direct3d-12 = 此程式需要 Direct3D 12 才能繪製視窗，而舊的顯示卡驅動程式或虛擬機器可能不支援。
