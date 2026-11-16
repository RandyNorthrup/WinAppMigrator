# Words of the parts every window shares: the search box, folder fields, and the Language menu.


## Search and folder fields

ui-clear-search = Очистить поиск
ui-browse = Обзор
# A tooltip on Browse. { $field } is the field's own label, such as "Install Location".
ui-browse-tooltip = Выберите папку для поля «{ $field }»

## Headings that open and close what is below them

# What a screen reader says for such a heading. { $title } is the heading, { $summary } a short count of what is below,
# such as "3 of 12 chosen"; $state is "open" or "closed", and only the words for each are translated.
ui-disclosure =
    { $state ->
        [open] { $title }, { $summary }, развернуто
       *[closed] { $title }, { $summary }, свернуто
    }

## The Language menu. Its list shows each language by its own name, which is not translated.

ui-language = Язык
ui-machine-translated = Этот перевод выполнен автоматически и может содержать ошибки.
ui-report-translation-problem = Сообщить о проблеме с переводом

## When a window cannot open

# Shown in a message box when the window cannot be drawn.
ui-needs-direct3d-12 = Для отображения окна нужен Direct3D 12, а старый графический драйвер или виртуальная машина могут его не поддерживать.
