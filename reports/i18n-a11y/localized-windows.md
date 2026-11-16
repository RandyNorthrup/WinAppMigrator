# Windows in Italian: install, uninstall, and a failed install

AC-LOCALIZED-WINDOWS asks that an install, an uninstall, and an install that fails part-way each complete or roll back
with no error caused by Windows' own tools answering in another language. This is that check, run on 2026-11-15.

## Where

testpc1 (Windows 11 Pro 25H2, build 26200), with Windows' Italian language pack installed, and a local administrator,
`WamItaliano`, whose display language and region are Italian (`PreferredUILanguages` = `it-IT`, culture `it-IT`). Run as
that account, Windows' tools answer in Italian, which the log shows: `sc.exe` says "OPERAZIONI NON RIUSCITE 1060: Il
servizio specificato non esiste come servizio installato", and `sc.exe query` labels its fields `NOME_SERVIZIO`,
`STATO`, `CODICE_USCITA_WIN32`. The steps ran elevated, in a scheduled task as that account.

This is Italian on a Windows installed in English, not a Windows installed in Italian (Microsoft's Italian evaluation
media needs a registration this check did without). What differs: the names of built-in accounts and groups, such as
`Administrators`, are those of the language Windows was installed in. WinAppMigrator names accounts and groups only by
SID (`*S-1-5-32-544`, `S-1-5-18`), so it does not depend on them, but they were not seen in Italian here.

## What was moved

Three real apps, from the URLs and hashes the project's CI pins: windows_exporter 0.32.1 (a service and a firewall rule,
plus a scheduled task added for it), Eclipse Mosquitto 2.1.2 (a service), and PuTTY 0.85 (four shortcuts). Each
installer was built with the 0.7.0 app (`target/release`, commit `b414735`) through its real window, in an English
session, and the original apps were then uninstalled.

## What happened

The full log is [localized-windows-italian.log](localized-windows-italian.log); the scripts are in
[lab-scripts](lab-scripts).

| Step | Result |
| --- | --- |
| Install each, `--apply` | Exit 0 each. Hashes valid before and after. Both services running, the firewall rule and the task there, four PuTTY shortcuts. |
| `sc.exe query`, in Italian | `STATO : 4 RUNNING` for both services, which is what the core reads |
| Uninstall each, `--uninstall <id> --quiet` | Exit 0 each: 31, 119 and 67 things undone, none failed or left. Nothing of the three apps left on the PC. |
| Install again with a file it must replace held open | Exit 1. The error carries Windows' Italian words ("Impossibile accedere al file. Il file è utilizzato da un altro processo. (os error 32)"); what it had done was put back (1 undone, 0 failed); no service, rule, or task was left, and the held file was untouched. |

What the installer printed was English, with English number marks (`111.3 MB`, `58.4 GB`), though the account's region
writes `111,3`: a quiet install is read by scripts, and prints the same on every PC.

## What was not covered here

- Building an installer while Windows speaks Italian: the app's scan ran in an English session. The scan reads the
  registry and files directly, and the tools it runs (`whoami`, `icacls`) are read by SID and exit code, which the log of
  Italian tool output above shows unchanged.
- A Windows installed in Italian (see above).
