# Words of the parts every window shares: the search box, folder fields, and the Language menu.


## Search and folder fields

ui-clear-search = Zoekopdracht wissen
ui-browse = Bladeren
# A tooltip on Browse. { $field } is the field's own label, such as "Install Location".
ui-browse-tooltip = Een map kiezen voor { $field }

## Headings that open and close what is below them

# What a screen reader says for such a heading. { $title } is the heading, { $summary } a short count of what is below,
# such as "3 of 12 chosen"; $state is "open" or "closed", and only the words for each are translated.
ui-disclosure =
    { $state ->
        [open] { $title }, { $summary }, uitgevouwen
       *[closed] { $title }, { $summary }, samengevouwen
    }

## The Language menu. Its list shows each language by its own name, which is not translated.

ui-language = Taal
ui-machine-translated = Deze vertaling is automatisch gemaakt en kan fouten bevatten.
ui-report-translation-problem = Een vertaalprobleem melden

## When a window cannot open

# Shown in a message box when the window cannot be drawn.
ui-needs-direct3d-12 = Dit programma heeft Direct3D 12 nodig om het venster te tekenen. Een oud grafisch stuurprogramma of een virtuele machine heeft dat mogelijk niet.
