# Brief: review one language's machine translation of WinAppMigrator, and bring it up to date

WinAppMigrator (a Windows program that turns an installed app into an installer for another PC) was just machine-translated
into 23 languages by one agent per language. You are the independent second pass for one language. Nobody on the project
speaks it, so your review is the only check its readers get before release. Fix what is wrong; do not rewrite what is right.

Repository: `C:\Users\Randy\Coding\win-app-migrator`. Write only inside `crates\winappmigrator-i18n\locales\<TAG>\` and
your report file `reports\i18n-a11y\review\<TAG>.md`. Do not run cargo or git. Other reviewers work on other languages at
the same time.

Checker (run from the repository folder):
`C:\Users\Randy\AppData\Local\Temp\claude\9b6bfcf6d6737e8e\28316fa3-07c3-42a6-9952-23884d58bf11\scratchpad\tools\ftlvenv\Scripts\python.exe scripts\ftl-tool.py check crates\winappmigrator-i18n\locales <TAG>`

## Read first

- `crates\winappmigrator-i18n\locales\glossary.md` and `docs\TRANSLATING.md` (Fluent rules: keep message names, attribute
  names, every `{ $placeholder }`, select keys of non-plural selects; plural selects use the language's CLDR categories with
  a `*[other]` default).
- `crates\winappmigrator-i18n\locales\<TAG>\terms.md`: the translator's term choices. Keep them unless one is wrong; if
  you change a term, change it everywhere it appears and update `terms.md`.
- The translator's notes for your language in
  `C:\Users\Randy\AppData\Local\Temp\claude\9b6bfcf6d6737e8e\28316fa3-07c3-42a6-9952-23884d58bf11\scratchpad\translator-notes.md`
  (section for your tag, plus the general points other translators raised).

## 1. Bring it up to date with English

The English changed after translation started. For every message in
`C:\Users\Randy\AppData\Local\Temp\claude\9b6bfcf6d6737e8e\28316fa3-07c3-42a6-9952-23884d58bf11\scratchpad\en-delta-list.tsv`
(file, new or changed, message name), read the current English in `locales\en-US\<file>` and make your language's
message say the same, editing it in place (add it, in English's position, if it is missing). Notable intents:

- `core-warning-build-*-left-out` `[unreadable]`: the path leaves the drive or goes through a link, so it is not *safe*
  to read; it is a safety rule, not a read error.
- `installer-license-detail`: the "keys" are registry values that hold license data, not product keys.
- `installer-no-programdata` / `installer-cli-no-programdata`: nothing was installed, because an install keeps the record
  it is undone by in ProgramData.
- "the License option" is now "the License and Activation Data option" (the checkbox `app-option-license`): use the
  same words as your `app-option-license`.
- `core-review-start-*` fill `{ $start }` in `core-review-service-detail` (`[other]`) and
  `core-review-driver-detail-starts`; `core-review-network-*` fill `{ $networks }` in `core-review-firewall-detail`.
  Use Windows' own words in your language (for firewall profiles, as Windows Defender Firewall names them), and make
  sure the sentences they go into read correctly with them.
- `client-*` `[unknown]` "Another PC" starts a sentence.

## 2. Review a sample, by back-translation

Choose at least 60 messages, including: all of `ui.ftl` and `winappmigrator.ftl`; the first 25 messages of
`installer.ftl` (the installer's main screen, read by every user of an installer); 15 from `app.ftl` including its
buttons, column headers, and option names; every plural message in `installer.ftl`; and at least 10 spread across the
`core*.ftl` files, chosen among those with placeholders. For each:

1. Back-translate your language's text into English without looking at the English first.
2. Compare with the English. Mark it OK, or fix it (meaning lost or changed, wrong term, wrong Windows word, unnatural,
   grammar wrong with the placeholder filled in, inconsistent with `terms.md`).

Also check across the whole language, quickly: the column headers in `app.ftl` (`app-column-*`) are short, since the
columns are narrow (an abbreviation is fine); button labels are short; nothing is left in English that should not be.
Keyboard key names (`Ctrl+F`) may be written as Windows writes them in your language (German `Strg+F`).

Run the checker until it reports 0 errors. A "still English" warning is fine only where the English really is the same.

## 3. Report

Write `reports\i18n-a11y\review\<TAG>.md`:

```markdown
# Review: <Language> (<TAG>)

Reviewer: independent machine pass, <date>. Translator's form of address: …; kept / changed because …

## Brought up to date
<n> messages (list the names).

## Sample (back-translation)
| Message | Back-translation | Verdict |
| --- | --- | --- |
| installer-install | Install | OK |
| … | … | Fixed: <what and why> |

## Other fixes
- …

## Open questions
- … (anything a native speaker should look at; terms you could not confirm against Windows)

Checker: <TAG>: 0 errors, <n> warnings.
```

Reply in under 150 words: the language, how many sample messages, how many fixed, the checker result, and the most
important open question.
