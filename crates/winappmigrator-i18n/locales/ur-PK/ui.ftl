# Words of the parts every window shares: the search box, folder fields, and the Language menu.


## Search and folder fields

ui-clear-search = تلاش صاف کریں
ui-browse = براؤز کریں
# A tooltip on Browse. { $field } is the field's own label, such as "Install Location".
ui-browse-tooltip = { $field } کے لیے فولڈر منتخب کریں

## Headings that open and close what is below them

# What a screen reader says for such a heading. { $title } is the heading, { $summary } a short count of what is below,
# such as "3 of 12 chosen"; $state is "open" or "closed", and only the words for each are translated.
ui-disclosure =
    { $state ->
        [open] { $title }، { $summary }، کھلا ہے
       *[closed] { $title }، { $summary }، بند ہے
    }

## The Language menu. Its list shows each language by its own name, which is not translated.

ui-language = زبان
ui-machine-translated = یہ ترجمہ مشین سے کیا گیا ہے اور اس میں غلطیاں ہو سکتی ہیں۔
ui-report-translation-problem = ترجمے کے مسئلے کی اطلاع دیں

## When a window cannot open

# Shown in a message box when the window cannot be drawn.
ui-needs-direct3d-12 = اپنی ونڈو بنانے کے لیے اسے Direct3D 12 درکار ہے، جو کسی پرانے گرافکس ڈرائیور یا ورچوئل مشین میں شاید موجود نہ ہو۔
