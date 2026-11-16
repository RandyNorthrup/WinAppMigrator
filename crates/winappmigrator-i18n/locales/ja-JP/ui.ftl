# Words of the parts every window shares: the search box, folder fields, and the Language menu.


## Search and folder fields

ui-clear-search = 検索をクリア
ui-browse = 参照
# A tooltip on Browse. { $field } is the field's own label, such as "Install Location".
ui-browse-tooltip = 「{ $field }」に使うフォルダーを選択します

## Headings that open and close what is below them

# What a screen reader says for such a heading. { $title } is the heading, { $summary } a short count of what is below,
# such as "3 of 12 chosen"; $state is "open" or "closed", and only the words for each are translated.
ui-disclosure =
    { $state ->
        [open] { $title }、{ $summary }、展開
       *[closed] { $title }、{ $summary }、折りたたみ
    }

## The Language menu. Its list shows each language by its own name, which is not translated.

ui-language = 言語
ui-machine-translated = この翻訳は機械翻訳によるもので、誤りが含まれている可能性があります。
ui-report-translation-problem = 翻訳の問題を報告

## When a window cannot open

# Shown in a message box when the window cannot be drawn.
ui-needs-direct3d-12 = ウィンドウの描画には Direct3D 12 が必要です。古いグラフィックス ドライバーや仮想マシンでは使用できない場合があります。
