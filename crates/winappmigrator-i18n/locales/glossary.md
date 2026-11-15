# Glossary for translators

Every translation follows these, so that the same thing has the same name everywhere, and matches what Windows calls it.

## Never translated

- **Product and program names:** WinAppMigrator, WinAppMigrator Client, Windows, Windows 7, Windows 10, Windows 11,
  Microsoft, Visual C++, .NET, PowerShell, Hyper-V, GitHub, winget, and the name of any app the program shows.
- **Folder and file names on disk:** `Program Files`, `Program Files (x86)`, `ProgramData`, `AppData`, `System32`,
  `Windows`, `C:\`, file extensions (`.exe`, `.vhdx`, `.wamdata`). They are the same in every language of Windows, even
  where Explorer shows a translated name.
- **What the program fills in:** every `{ $placeholder }`.
- **Codes and technical names:** SHA-256, UAC, PATH, the names of registry keys and values, service names, command-line
  flags (`--force`, `--target-root`, `--lang`), and units with a standard symbol where your language uses the same symbol.

## Use Windows' own word in your language

Where a message names a part of Windows, use exactly the name Windows shows in that language, as found in Windows itself or
the [Microsoft Language Portal](https://www.microsoft.com/language), not a translation of the English:

| English | What it is |
| --- | --- |
| Settings | The Settings app |
| Apps, Installed apps | Settings, Apps, Installed apps, where a program is uninstalled |
| Accessibility | Settings, Accessibility |
| Text size, Animation effects, Contrast themes | Settings, Accessibility |
| Explorer | File Explorer |
| administrator rights, administrator | An account that can change the whole PC |
| User Account Control | The prompt asking to allow changes |
| service | A Windows service |
| scheduled task | A Task Scheduler task |
| shortcut | A `.lnk` shortcut |
| registry | The Windows registry |
| firewall rule | A Windows Defender Firewall rule |
| network share | A shared folder on another PC |
| Documents, Saved Games | The user's known folders, by Windows' name |

## WinAppMigrator's own words

Translate these, and use one word for each everywhere:

| English | What it means |
| --- | --- |
| installer | The single `.exe` WinAppMigrator builds, which reinstalls an app on another PC |
| package | What an installer carries: the app's files and settings |
| Install Location | The folder the app is installed into |
| User Data | The option to include, and restore, an account's settings, documents, and saved games for the app |
| License | The option to include an app's activation and license data |
| Preview Changes | Working out what an install would do, without doing it |
| uninstall | Taking an install back out, which puts back what it replaced |
| runtime | A Microsoft library an app needs, such as Visual C++ |
| the old PC, the new PC | The PC an app is moved from, and the one it is moved to |
| Network PC | Moving an app from another PC over the network, through WinAppMigrator Client |
| pairing code | The six-digit code WinAppMigrator Client shows, typed into the app to connect |

## Style

- **Address the reader as Windows does in your language:** the same politeness and form of "you" that Windows itself uses
  (for example *Sie* in German, *vous* in French, *tu* in Italian, polite forms in Japanese and Korean).
- **Capitalization:** English titles, labels, and buttons use title case. Follow your language's own rules instead,
  usually sentence case.
- **Sentences** say plainly what happened and what to do, as the English does. Do not add or drop meaning; if the English
  is unclear, report it rather than guessing.
- **Digits** stay 0 to 9, as they appear beside paths, versions, and sizes.
- **Quotation marks and punctuation** follow your language (« », „ “, 「」).
