# Words of the parts every window shares: the search box, folder fields, and the Language menu.


## Search and folder fields

ui-clear-search = Aramayı temizle
ui-browse = Gözat
# A tooltip on Browse. { $field } is the field's own label, such as "Install Location".
ui-browse-tooltip = { $field } için bir klasör seçin

## Headings that open and close what is below them

# What a screen reader says for such a heading. { $title } is the heading, { $summary } a short count of what is below,
# such as "3 of 12 chosen"; $state is "open" or "closed", and only the words for each are translated.
ui-disclosure =
    { $state ->
        [open] { $title }, { $summary }, genişletilmiş
       *[closed] { $title }, { $summary }, daraltılmış
    }

## The Language menu. Its list shows each language by its own name, which is not translated.

ui-language = Dil
ui-machine-translated = Bu çeviri makine tarafından yapıldı ve hatalar içerebilir.
ui-report-translation-problem = Çeviri sorunu bildir

## When a window cannot open

# Shown in a message box when the window cannot be drawn.
ui-needs-direct3d-12 = Bu programın penceresini çizmek için Direct3D 12 gerekir. Eski bir grafik sürücüsü veya bir sanal makine bunu sağlamayabilir.
