# Words of the parts every window shares: the search box, folder fields, and the Language menu.


## Search and folder fields

ui-clear-search = खोज साफ़ करें
ui-browse = ब्राउज़ करें
# A tooltip on Browse. { $field } is the field's own label, such as "Install Location".
ui-browse-tooltip = { $field } के लिए एक फ़ोल्डर चुनें

## Headings that open and close what is below them

# What a screen reader says for such a heading. { $title } is the heading, { $summary } a short count of what is below,
# such as "3 of 12 chosen"; $state is "open" or "closed", and only the words for each are translated.
ui-disclosure =
    { $state ->
        [open] { $title }, { $summary }, खुला
       *[closed] { $title }, { $summary }, बंद
    }

## The Language menu. Its list shows each language by its own name, which is not translated.

ui-language = भाषा
ui-machine-translated = यह अनुवाद मशीन द्वारा किया गया है और इसमें गलतियाँ हो सकती हैं।
ui-report-translation-problem = अनुवाद की समस्या की रिपोर्ट करें

## When a window cannot open

# Shown in a message box when the window cannot be drawn.
ui-needs-direct3d-12 = अपनी विंडो दिखाने के लिए इसे Direct3D 12 की आवश्यकता है, जो किसी पुराने ग्राफ़िक्स ड्राइवर या वर्चुअल मशीन में शायद न हो।
