# 0.7.0, released

AC-RELEASE asks that 0.7.0 be tagged and published, that its download be verified by hash, signature and attestation,
that it start in the language of a lab PC, and that a winget update with translated descriptions be opened. This is what
was seen on 2026-11-15, after it happened.

## Published

Tag `v0.7.0` on commit `c12dd9b`, pushed to GitHub; the Release workflow (run 43799382621) built the client for
Windows 7, signed it, built, linted and tested the app, signed it, and published
<https://github.com/RandyNorthrup/WinAppMigrator/releases/tag/v0.7.0> with `WinAppMigrator-v0.7.0.exe`,
`WinAppMigrator-v0.7.0-windows-x64.zip` and `SHA256SUMS.txt`. CI on the code it was built from passed all 18 jobs
(run 43796519343, commit `1601d72`; the commits after it change only reports and the plan).

## The download

[release-0.7.0/download-check.log](release-0.7.0/download-check.log): downloaded into an empty folder,

- both files match `SHA256SUMS.txt`;
- `WinAppMigrator-v0.7.0.exe` is Authenticode-signed by Randy Northrup, valid, and timestamped by Microsoft;
- `gh attestation verify` verifies both files: built by `.github/workflows/release.yml` from `refs/tags/v0.7.0` at
  `c12dd9b`.

## It starts in the lab PC's language

On testpc1 (Windows 11 Pro 25H2), the downloaded exe (same SHA-256) was started by
[release-language-check.ps1](lab-scripts/release-language-check.ps1), with the signed-in account's display-language
list, which Settings writes, set to Italian alone and then to Arabic alone, and no language chosen in the app. Each time
its window was read through UI Automation, an app was picked, and pictures were kept; the list was put back afterward.
[release-0.7.0/language-check.log](release-0.7.0/language-check.log):

- **Italian:** every control's name is Italian ("Ricarica l'elenco delle app", "Cerca per nome o autore (Ctrl+F)",
  "57 app"). [it-IT.png](release-0.7.0/it-IT.png), [it-IT-app.png](release-0.7.0/it-IT-app.png).
- **Arabic:** every name is Arabic, and the window is the mirror image of the English one: "57 تطبيقًا" reads with the
  number where reading starts, the search hint keeps "(Ctrl+F)" whole, and "7-Zip 26.05 (x64)" reads as it is written.
  [ar-SA.png](release-0.7.0/ar-SA.png), [ar-SA-app.png](release-0.7.0/ar-SA-app.png).

### Found on the way, and fixed

The pictures of earlier builds on testpc1's real Windows found three defects that the tests had not:

1. In Arabic, a number inside a phrase was turned around and put on the wrong side: 57 apps read "75". egui joined the
   pieces of a line into one when they looked the same and shaped the joined text right to left. Fixed before the
   release (`1601d72`), with a test that reads the glyphs' places with Windows' own fonts, and its red drill.
2. In Arabic, the search box's hint had its letters reversed and unjoined when egui shortened it. Fixed before the
   release (`1601d72`), with a test and its red drill.
3. In Italian, the heading "Contenuto del programma di installazione" ran under "Analizza di nuovo", in
   [it-IT-app.png](release-0.7.0/it-IT-app.png) of the release itself. **This one is in 0.7.0.** It is fixed on main
   (`1e0f2fd`), with a test that opens the smallest window in all 24 languages, and goes out with the next release.

## winget

[microsoft/winget-pkgs#481256](https://github.com/microsoft/winget-pkgs/pull/481256), "New version:
RandyNorthrup.WinAppMigrator version 0.7.0": the installer manifest with the new URL and SHA-256, the default locale with
one sentence and a tag added, and 23 locale manifests whose `ShortDescription` and `Description` come from each
language's page of the website. `winget validate` passed on MINI-DT. Not install-tested, as for 0.6.1 and 0.6.2.

## The website

The wiki's three commits for 0.7.0 were pushed; the website rebuilt (run 43802508144) and serves the landing page in
English and in each of the 23 languages with their `lang` and `dir` (`ar-SA` right to left), each linking to the 0.7.0
download.
