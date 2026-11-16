# Words of the parts every window shares: the search box, folder fields, and the Language menu.


## Search and folder fields

ui-clear-search = Borrar búsqueda
ui-browse = Examinar
# A tooltip on Browse. { $field } is the field's own label, such as "Install Location".
ui-browse-tooltip = Elige una carpeta para { $field }

## Headings that open and close what is below them

# What a screen reader says for such a heading. { $title } is the heading, { $summary } a short count of what is below,
# such as "3 of 12 chosen"; $state is "open" or "closed", and only the words for each are translated.
ui-disclosure =
    { $state ->
        [open] { $title }, { $summary }, expandido
       *[closed] { $title }, { $summary }, contraído
    }

## The Language menu. Its list shows each language by its own name, which is not translated.

ui-language = Idioma
ui-machine-translated = Esta traducción se ha hecho de forma automática y puede contener errores.
ui-report-translation-problem = Notificar un problema de traducción

## When a window cannot open

# Shown in a message box when the window cannot be drawn.
ui-needs-direct3d-12 = Se necesita Direct3D 12 para dibujar la ventana, y es posible que un controlador de gráficos antiguo o una máquina virtual no lo tengan.
