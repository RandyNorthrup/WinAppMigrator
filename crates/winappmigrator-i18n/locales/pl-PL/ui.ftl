# Words of the parts every window shares: the search box, folder fields, and the Language menu.


## Search and folder fields

ui-clear-search = Wyczyść wyszukiwanie
ui-browse = Przeglądaj
# A tooltip on Browse. { $field } is the field's own label, such as "Install Location".
ui-browse-tooltip = Wybierz folder dla pola { $field }

## Headings that open and close what is below them

# What a screen reader says for such a heading. { $title } is the heading, { $summary } a short count of what is below,
# such as "3 of 12 chosen"; $state is "open" or "closed", and only the words for each are translated.
ui-disclosure =
    { $state ->
        [open] { $title }, { $summary }, rozwinięty
       *[closed] { $title }, { $summary }, zwinięty
    }

## The Language menu. Its list shows each language by its own name, which is not translated.

ui-language = Język
ui-machine-translated = To tłumaczenie zostało wykonane maszynowo i może zawierać błędy.
ui-report-translation-problem = Zgłoś problem z tłumaczeniem

## When a window cannot open

# Shown in a message box when the window cannot be drawn.
ui-needs-direct3d-12 = Do wyświetlenia okna jest wymagany interfejs Direct3D 12, którego może nie obsługiwać stary sterownik karty graficznej lub maszyna wirtualna.
