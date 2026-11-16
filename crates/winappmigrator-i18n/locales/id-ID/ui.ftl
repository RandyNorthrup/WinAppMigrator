# Words of the parts every window shares: the search box, folder fields, and the Language menu.


## Search and folder fields

ui-clear-search = Hapus pencarian
ui-browse = Telusuri
# A tooltip on Browse. { $field } is the field's own label, such as "Install Location".
ui-browse-tooltip = Pilih folder untuk { $field }

## Headings that open and close what is below them

# What a screen reader says for such a heading. { $title } is the heading, { $summary } a short count of what is below,
# such as "3 of 12 chosen"; $state is "open" or "closed", and only the words for each are translated.
ui-disclosure =
    { $state ->
        [open] { $title }, { $summary }, terbuka
       *[closed] { $title }, { $summary }, tertutup
    }

## The Language menu. Its list shows each language by its own name, which is not translated.

ui-language = Bahasa
ui-machine-translated = Terjemahan ini dibuat oleh mesin dan mungkin mengandung kesalahan.
ui-report-translation-problem = Laporkan masalah terjemahan

## When a window cannot open

# Shown in a message box when the window cannot be drawn.
ui-needs-direct3d-12 = Program ini memerlukan Direct3D 12 untuk menggambar jendelanya, yang mungkin tidak dimiliki oleh driver grafis lama atau mesin virtual.
