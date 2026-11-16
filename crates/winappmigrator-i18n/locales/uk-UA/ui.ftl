# Words of the parts every window shares: the search box, folder fields, and the Language menu.


## Search and folder fields

ui-clear-search = Очистити пошук
ui-browse = Огляд
# A tooltip on Browse. { $field } is the field's own label, such as "Install Location".
ui-browse-tooltip = Вибрати папку для поля «{ $field }»

## Headings that open and close what is below them

# What a screen reader says for such a heading. { $title } is the heading, { $summary } a short count of what is below,
# such as "3 of 12 chosen"; $state is "open" or "closed", and only the words for each are translated.
ui-disclosure =
    { $state ->
        [open] { $title }, { $summary }, розгорнуто
       *[closed] { $title }, { $summary }, згорнуто
    }

## The Language menu. Its list shows each language by its own name, which is not translated.

ui-language = Мова
ui-machine-translated = Цей переклад виконано автоматично, тому він може містити помилки.
ui-report-translation-problem = Повідомити про проблему з перекладом

## When a window cannot open

# Shown in a message box when the window cannot be drawn.
ui-needs-direct3d-12 = Для відображення вікна потрібен Direct3D 12, якого може не бути в старому графічному драйвері або у віртуальній машині.
