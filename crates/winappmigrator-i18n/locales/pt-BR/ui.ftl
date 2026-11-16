# Words of the parts every window shares: the search box, folder fields, and the Language menu.


## Search and folder fields

ui-clear-search = Limpar pesquisa
ui-browse = Procurar
# A tooltip on Browse. { $field } is the field's own label, such as "Install Location".
ui-browse-tooltip = Escolha uma pasta para { $field }

## Headings that open and close what is below them

# What a screen reader says for such a heading. { $title } is the heading, { $summary } a short count of what is below,
# such as "3 of 12 chosen"; $state is "open" or "closed", and only the words for each are translated.
ui-disclosure =
    { $state ->
        [open] { $title }, { $summary }, expandido
       *[closed] { $title }, { $summary }, recolhido
    }

## The Language menu. Its list shows each language by its own name, which is not translated.

ui-language = Idioma
ui-machine-translated = Esta tradução foi feita automaticamente e pode conter erros.
ui-report-translation-problem = Relatar um problema de tradução

## When a window cannot open

# Shown in a message box when the window cannot be drawn.
ui-needs-direct3d-12 = É necessário o Direct3D 12 para desenhar esta janela, e um driver gráfico antigo ou uma máquina virtual pode não oferecê-lo.
