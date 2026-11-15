# Translating WinAppMigrator

WinAppMigrator speaks 24 languages. English is written by hand; the other 23 were first made by machine, then reviewed
by a second machine pass, because the project had no native speakers to ask. They will have mistakes. If you speak one of
these languages, you can make it better, and every fix helps everyone who reads it.

## Report a problem

The quickest way: in the app or the installer, open **Language** and choose **Report a Translation Problem**. It opens the
[Translation Problem form](https://github.com/RandyNorthrup/WinAppMigrator/issues/new?template=translation-problem.yml)
with your language filled in. Say what the screen shows, what it should say, and where you saw it. A screenshot helps.

## Fix it yourself

Each language is a folder in [`crates/winappmigrator-i18n/locales`](../crates/winappmigrator-i18n/locales), named by its
tag (`it-IT`, `ar-SA`), holding the same files as `en-US`:

| File | What it holds |
| --- | --- |
| `winappmigrator.ftl` | Words shared by everything: sizes, lists |
| `ui.ftl` | Parts every window has: the search box, folder fields, the Language menu |
| `app.ftl` | The app, which builds installers |
| `installer.ftl` | The installer and uninstaller, which run on the new PC |
| `core*.ftl` | What the core says: warnings, refusals, errors, the list of what an install sets up |
| `client.ftl` | WinAppMigrator Client, the console program on the old PC |

The files use [Fluent](https://projectfluent.org/fluent/guide/). A message is a name, `=`, and the words:

```ftl
# { $path } is a folder, such as C:\Program Files\App.
installer-location-in-use = This folder cannot be used: { $path }
```

- **Change only what comes after `=`.** Keep the name, and keep every `{ $placeholder }` exactly: the program fills it in.
  You may move a placeholder anywhere in your sentence.
- **Plurals:** a message that depends on a count has one line per plural form. Use your language's forms (Polish and
  Russian have `one`, `few`, `many`, and `other`; Arabic also `zero` and `two`; Chinese, Japanese, and Korean only `other`):

  ```ftl
  core-files = { $count ->
      [one] { $count } file
      [few] { $count } pliki
     *[other] { $count } files
  }
  ```

  The line starting with `*` is the one used when no other matches, and must stay.
- **The `#` lines** explain a message: what fills each placeholder, and where the words appear. Do not translate them.
- Read [`locales/glossary.md`](../crates/winappmigrator-i18n/locales/glossary.md) first: it says which words stay as they
  are, and how Windows itself names what the app refers to.

Then open a pull request with your change. The checks fail if a message is missing, a placeholder is lost or added, or a
file does not parse, and say which.

## Trying your change

Build and run the app (see [CONTRIBUTING.md](../CONTRIBUTING.md)), then choose your language under **Language**. Look for
words cut off at the edge of a button or a box: German and Russian often need more room than English, and the windows
must be made to fit, not the words shortened past sense.

## Adding a language

A new language needs its row in `LANGUAGES` in
[`crates/winappmigrator-i18n/src/languages.rs`](../crates/winappmigrator-i18n/src/languages.rs) (its tag, its own name,
its direction, its script, and how it writes numbers), a folder of all the files, and, for a script Segoe UI does not draw,
its Windows font in `script_fonts` in [`crates/winappmigrator-ui/src/theme.rs`](../crates/winappmigrator-ui/src/theme.rs).
Open an issue first, so the work is not done twice.
