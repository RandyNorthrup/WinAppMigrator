# Words of the parts every window shares: the search box, folder fields, and the Language menu.


## Search and folder fields

ui-clear-search = Suche löschen
ui-browse = Durchsuchen
# A tooltip on Browse. { $field } is the field's own label, such as "Install Location".
ui-browse-tooltip = Ordner für „{ $field }“ auswählen

## Headings that open and close what is below them

# What a screen reader says for such a heading. { $title } is the heading, { $summary } a short count of what is below,
# such as "3 of 12 chosen"; $state is "open" or "closed", and only the words for each are translated.
ui-disclosure =
    { $state ->
        [open] { $title }, { $summary }, geöffnet
       *[closed] { $title }, { $summary }, geschlossen
    }

## The Language menu. Its list shows each language by its own name, which is not translated.

ui-language = Sprache
ui-machine-translated = Diese Übersetzung wurde maschinell erstellt und kann Fehler enthalten.
ui-report-translation-problem = Übersetzungsproblem melden

## When a window cannot open

# Shown in a message box when the window cannot be drawn.
ui-needs-direct3d-12 = Zum Darstellen des Fensters wird Direct3D 12 benötigt. Ein alter Grafiktreiber oder ein virtueller Computer bietet das möglicherweise nicht.
