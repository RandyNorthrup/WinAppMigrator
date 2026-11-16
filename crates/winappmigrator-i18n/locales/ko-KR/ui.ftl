# Words of the parts every window shares: the search box, folder fields, and the Language menu.


## Search and folder fields

ui-clear-search = 검색 지우기
ui-browse = 찾아보기
# A tooltip on Browse. { $field } is the field's own label, such as "Install Location".
ui-browse-tooltip = { $field }에 사용할 폴더 선택

## Headings that open and close what is below them

# What a screen reader says for such a heading. { $title } is the heading, { $summary } a short count of what is below,
# such as "3 of 12 chosen"; $state is "open" or "closed", and only the words for each are translated.
ui-disclosure =
    { $state ->
        [open] { $title }, { $summary }, 확장됨
       *[closed] { $title }, { $summary }, 축소됨
    }

## The Language menu. Its list shows each language by its own name, which is not translated.

ui-language = 언어
ui-machine-translated = 이 번역은 기계 번역이며 오류가 있을 수 있습니다.
ui-report-translation-problem = 번역 문제 보고

## When a window cannot open

# Shown in a message box when the window cannot be drawn.
ui-needs-direct3d-12 = 창을 그리려면 Direct3D 12가 필요합니다. 오래된 그래픽 드라이버나 가상 머신에서는 지원되지 않을 수 있습니다.
