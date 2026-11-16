# Words of the parts every window shares: the search box, folder fields, and the Language menu.


## Search and folder fields

ui-clear-search = مسح البحث
ui-browse = استعراض
# A tooltip on Browse. { $field } is the field's own label, such as "Install Location".
ui-browse-tooltip = اختيار مجلد لحقل "{ $field }"

## Headings that open and close what is below them

# What a screen reader says for such a heading. { $title } is the heading, { $summary } a short count of what is below,
# such as "3 of 12 chosen"; $state is "open" or "closed", and only the words for each are translated.
ui-disclosure =
    { $state ->
        [open] { $title }، { $summary }، موسّع
       *[closed] { $title }، { $summary }، مطوي
    }

## The Language menu. Its list shows each language by its own name, which is not translated.

ui-language = اللغة
ui-machine-translated = تمت هذه الترجمة آليًا وقد تحتوي على أخطاء.
ui-report-translation-problem = الإبلاغ عن مشكلة في الترجمة

## When a window cannot open

# Shown in a message box when the window cannot be drawn.
ui-needs-direct3d-12 = يحتاج هذا إلى Direct3D 12 لرسم نافذته، وقد لا يتوفر ذلك مع برنامج تشغيل رسومات قديم أو على جهاز ظاهري.
