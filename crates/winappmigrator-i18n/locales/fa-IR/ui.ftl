# Words of the parts every window shares: the search box, folder fields, and the Language menu.


## Search and folder fields

ui-clear-search = پاک کردن جستجو
ui-browse = مرور
# A tooltip on Browse. { $field } is the field's own label, such as "Install Location".
ui-browse-tooltip = یک پوشه برای «{ $field }» انتخاب کنید

## Headings that open and close what is below them

# What a screen reader says for such a heading. { $title } is the heading, { $summary } a short count of what is below,
# such as "3 of 12 chosen"; $state is "open" or "closed", and only the words for each are translated.
ui-disclosure =
    { $state ->
        [open] { $title }، { $summary }، بازشده
       *[closed] { $title }، { $summary }، جمع‌شده
    }

## The Language menu. Its list shows each language by its own name, which is not translated.

ui-language = زبان
ui-machine-translated = این ترجمه به‌صورت ماشینی انجام شده است و ممکن است اشتباهاتی داشته باشد.
ui-report-translation-problem = گزارش مشکل ترجمه

## When a window cannot open

# Shown in a message box when the window cannot be drawn.
ui-needs-direct3d-12 = این برنامه برای ترسیم پنجره خود به Direct3D 12 نیاز دارد، که ممکن است درایور گرافیکی قدیمی یا ماشین مجازی آن را نداشته باشد.
