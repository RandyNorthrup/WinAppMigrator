# Words of the parts every window shares: the search box, folder fields, and the Language menu.


## Search and folder fields

ui-clear-search = Effacer la recherche
ui-browse = Parcourir
# A tooltip on Browse. { $field } is the field's own label, such as "Install Location".
ui-browse-tooltip = Choisir un dossier pour « { $field } »

## Headings that open and close what is below them

# What a screen reader says for such a heading. { $title } is the heading, { $summary } a short count of what is below,
# such as "3 of 12 chosen"; $state is "open" or "closed", and only the words for each are translated.
ui-disclosure =
    { $state ->
        [open] { $title }, { $summary }, développé
       *[closed] { $title }, { $summary }, réduit
    }

## The Language menu. Its list shows each language by its own name, which is not translated.

ui-language = Langue
ui-machine-translated = Cette traduction a été réalisée automatiquement et peut contenir des erreurs.
ui-report-translation-problem = Signaler un problème de traduction

## When a window cannot open

# Shown in a message box when the window cannot be drawn.
ui-needs-direct3d-12 = Ce programme a besoin de Direct3D 12 pour afficher sa fenêtre. Un ancien pilote graphique ou une machine virtuelle peut ne pas le prendre en charge.
