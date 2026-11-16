# Brief: translate WinAppMigrator's catalogs into one language

WinAppMigrator (a Windows program: an app that builds self-extracting installers from installed apps, an installer that
reinstalls the app on a new PC, and a small console client for old PCs) is getting its first translations. There are no
native speakers on the project, so your translation is what people in your language will read. Make it read as if
Microsoft's own translators wrote it for Windows: natural, plain, precise, consistent. A person moving their apps to a
new PC is relying on these words to know what will happen to their files.

## What you write

Repository: `C:\Users\Randy\Coding\win-app-migrator`. English source: `crates\winappmigrator-i18n\locales\en-US\*.ftl`
(9 files, about 1,300 messages). You write `crates\winappmigrator-i18n\locales\<TAG>\` with the same 9 file names, plus
one short `terms.md`. Touch nothing else in the repository. Other translators are writing other language folders at the
same time; leave them alone.

Do not run cargo, git, or any build. Use only the checker below.

## Read first

1. `crates\winappmigrator-i18n\locales\glossary.md`: what is never translated, where Windows' own word must be used,
   WinAppMigrator's own terms, and style (form of address, capitalization, punctuation, digits).
2. `docs\TRANSLATING.md`: the Fluent rules in short.
3. Each English file in full before you translate it. The `#` comments say what fills each placeholder and where the
   words appear (a button, a title, a fragment inserted into another sentence). Use them.

## Steps

1. Write `locales\<TAG>\terms.md`: a short table, English term → your term, for every term in the glossary's two tables
   ("Use Windows' own word" and "WinAppMigrator's own words"), plus any other word you find repeated across the files
   (for example: app, folder, drive, scan, build, preview, restore, settings, warning, record of the install). Use
   Windows' real UI term in your language where one exists. Then use exactly those terms everywhere.
2. Translate file by file, smallest first: `winappmigrator.ftl`, `ui.ftl`, `client.ftl`, `installer.ftl`, `app.ftl`,
   `core-restore.ftl`, `core-build.ftl`, `core-review.ftl`, `core.ftl`.
   - Write drafts that hold **only messages** (no comments needed): `id = translated text`, with attributes and
     multi-line values exactly as Fluent needs.
   - A file with more than about 120 messages goes in pieces: `locales\<TAG>\drafts\<stem>.1.ftl`,
     `...\drafts\<stem>.2.ftl`, and so on (for `core.ftl`, the stem is `core`). Small files can be written straight as
     `locales\<TAG>\<name>.ftl`.
   - Then merge, which puts your messages in English's order under English's comments, writes
     `locales\<TAG>\<name>.ftl`, and deletes the pieces:
     `C:\Users\Randy\AppData\Local\Temp\claude\9b6bfcf6d6737e8e\28316fa3-07c3-42a6-9952-23884d58bf11\scratchpad\tools\ftlvenv\Scripts\python.exe scripts\ftl-tool.py merge crates\winappmigrator-i18n\locales <TAG> --file <name>.ftl`
     (run from the repository folder).
   - Check: the same command with `check` instead of `merge`. Fix every ERROR. Fix every WARN about a missing plural
     form. A WARN that text is "still English" is right only where the English really is the same in your language (a
     product name, a code, a key); translate the rest.
   - To fix a message after merging, edit `locales\<TAG>\<name>.ftl` directly, then check again.
3. When all 9 files check with 0 errors, run `check` once with no `--file`, and re-read your whole translation once for
   consistency with `terms.md`, mistranslations, and anything that reads like a machine. Fix what you find.

## Fluent rules that matter

- Keep every message name and attribute name (`.tooltip` and the like) exactly. Translate only the text.
- Keep every `{ $placeholder }` exactly: the same names, none added, none dropped. Move them wherever your grammar needs.
- A value that continues over several lines: every line after the first is indented (4 spaces). No blank line inside a
  message.
- `{` and `}` are special. A literal brace is `{"{"}`. A value that must start or end with a space keeps it with a
  string literal, as English's `list-separator = {", "}` does.
- **Select expressions** (`{ $x -> [a] ... *[b] ... }`):
  - **Plural** selects (keys such as `one`, `few`, `many`, `other`, on a count): use your language's own CLDR plural
    categories, with `*[other]` as the default. Polish, Russian, Ukrainian need `one`, `few`, `many`; Arabic needs
    `zero`, `one`, `two`, `few`, `many`; Hebrew uses `one`, `two`, `other`. Languages with one form (Chinese, Japanese,
    Korean, Indonesian, Vietnamese, Thai) keep the select with only `*[other]`, so that the placeholder set stays the
    same as English's:
    `size-bytes = { $count ->\n   *[other] { $value } 字节\n}`
  - **Other** selects (keys such as `[service]`, `[task]`, `[true]`, `[file]`): the keys are values the program passes.
    Keep every key and the default exactly; translate each variant's text.
- Keyboard shortcuts in text (`Ctrl+F`), command-line flags (`--force`), registry paths, `%AppData%`, `LocalService`,
  `SYSTEM`, file and folder names, and product names stay as they are (see the glossary).
- English title case (on buttons, titles, labels) becomes your language's own capitalization.
- A message that the comments say is a fragment inserted into another sentence stays a fragment: same case and
  punctuation conventions at its start and end as the sentence it goes into needs in your language. Check the message it
  goes into so they read as one sentence.
- Do not add bidirectional marks or HTML entities. Write plain Unicode. Digits stay 0 to 9.

## When you finish

Reply with: the language, `check` result for all files (errors and warnings counts), the form of address you used and
why, any English message that was unclear or that you think is wrong (name and why), and any term you were unsure of.
Keep the reply under 250 words.
