# Words of the parts every window shares: the search box, folder fields, and the Language menu.


## Search and folder fields

ui-clear-search = ניקוי החיפוש
ui-browse = עיון
# A tooltip on Browse. { $field } is the field's own label, such as "Install Location".
ui-browse-tooltip = בחירת תיקיה עבור { $field }

## Headings that open and close what is below them

# What a screen reader says for such a heading. { $title } is the heading, { $summary } a short count of what is below,
# such as "3 of 12 chosen"; $state is "open" or "closed", and only the words for each are translated.
ui-disclosure =
    { $state ->
        [open] { $title }, { $summary }, מורחב
       *[closed] { $title }, { $summary }, מכווץ
    }

## The Language menu. Its list shows each language by its own name, which is not translated.

ui-language = שפה
ui-machine-translated = תרגום זה נוצר על ידי מכונה ועשוי לכלול שגיאות.
ui-report-translation-problem = דיווח על בעיה בתרגום

## When a window cannot open

# Shown in a message box when the window cannot be drawn.
ui-needs-direct3d-12 = תוכנית זו זקוקה ל-Direct3D 12 כדי לצייר את החלון שלה, ומנהל התקן גרפי ישן או מחשב וירטואלי עשויים שלא לתמוך בו.
