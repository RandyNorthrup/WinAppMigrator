# Words of the parts every window shares: the search box, folder fields, and the Language menu.


## Search and folder fields

ui-clear-search = Cancella ricerca
ui-browse = Sfoglia
# A tooltip on Browse. { $field } is the field's own label, such as "Install Location".
ui-browse-tooltip = Scegli una cartella per { $field }

## Headings that open and close what is below them

# What a screen reader says for such a heading. { $title } is the heading, { $summary } a short count of what is below,
# such as "3 of 12 chosen"; $state is "open" or "closed", and only the words for each are translated.
ui-disclosure =
    { $state ->
        [open] { $title }, { $summary }, espanso
       *[closed] { $title }, { $summary }, compresso
    }

## The Language menu. Its list shows each language by its own name, which is not translated.

ui-language = Lingua
ui-machine-translated = Questa traduzione è stata eseguita automaticamente e potrebbe contenere errori.
ui-report-translation-problem = Segnala un problema di traduzione

## When a window cannot open

# Shown in a message box when the window cannot be drawn.
ui-needs-direct3d-12 = Per disegnare la finestra è necessario Direct3D 12, che potrebbe non essere disponibile con un driver grafico obsoleto o in una macchina virtuale.
