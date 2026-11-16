# WinAppMigrator's words in English, the language every other catalog is translated from.
#
# For translators: keep every { $placeholder } exactly as it is, and keep names of programs, files, and Windows
# settings as Windows shows them in your language. Read docs/TRANSLATING.md and locales/glossary.md first.


## Sizes. { $value } is the number already written as this PC writes numbers, such as 824.0 or 824,0.

size-bytes =
    { $count ->
        [zero] { $value } بايت
        [one] { $value } بايت
        [two] { $value } بايت
        [few] { $value } بايت
        [many] { $value } بايت
       *[other] { $value } بايت
    }
size-kb = { $value } كيلوبايت
size-mb = { $value } ميغابايت
size-gb = { $value } غيغابايت
size-tb = { $value } تيرابايت

## Lists

# What goes between the items of a list, such as "services, tasks, shortcuts". Keep the quotes: they keep the space.
list-separator = { "، " }
