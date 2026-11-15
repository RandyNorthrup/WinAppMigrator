# Words of the parts every window shares: the search box, folder fields, and the Language menu.

## Search and folder fields

ui-clear-search = Clear Search
ui-browse = Browse
# A tooltip on Browse. { $field } is the field's own label, such as "Install Location".
ui-browse-tooltip = Choose a folder for { $field }

## Headings that open and close what is below them

# What a screen reader says for such a heading. { $title } is the heading, { $summary } a short count of what is below,
# such as "3 of 12 chosen"; $state is "open" or "closed", and only the words for each are translated.
ui-disclosure = { $state ->
        [open] { $title }, { $summary }, open
       *[closed] { $title }, { $summary }, closed
    }

## The Language menu. Its list shows each language by its own name, which is not translated.

ui-language = Language
ui-machine-translated = This translation was made by machine and may have mistakes.
ui-report-translation-problem = Report a Translation Problem

## When a window cannot open

# Shown in a message box when the window cannot be drawn.
ui-needs-direct3d-12 = This needs Direct3D 12 to draw its window, which an old graphics driver or a virtual machine may not have.
