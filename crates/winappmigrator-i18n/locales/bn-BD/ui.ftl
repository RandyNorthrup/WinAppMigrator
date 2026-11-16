# Words of the parts every window shares: the search box, folder fields, and the Language menu.


## Search and folder fields

ui-clear-search = অনুসন্ধান মুছুন
ui-browse = ব্রাউজ করুন
# A tooltip on Browse. { $field } is the field's own label, such as "Install Location".
ui-browse-tooltip = { $field }-এর জন্য একটি ফোল্ডার বেছে নিন

## Headings that open and close what is below them

# What a screen reader says for such a heading. { $title } is the heading, { $summary } a short count of what is below,
# such as "3 of 12 chosen"; $state is "open" or "closed", and only the words for each are translated.
ui-disclosure =
    { $state ->
        [open] { $title }, { $summary }, খোলা
       *[closed] { $title }, { $summary }, বন্ধ
    }

## The Language menu. Its list shows each language by its own name, which is not translated.

ui-language = ভাষা
ui-machine-translated = এই অনুবাদটি মেশিনে করা হয়েছে, তাই এতে ভুল থাকতে পারে।
ui-report-translation-problem = অনুবাদের সমস্যা জানান

## When a window cannot open

# Shown in a message box when the window cannot be drawn.
ui-needs-direct3d-12 = এই প্রোগ্রামের উইন্ডো দেখাতে Direct3D 12 প্রয়োজন, যা পুরোনো গ্রাফিক্স ড্রাইভারে বা ভার্চুয়াল মেশিনে না-ও থাকতে পারে।
