# Review: Bengali, Bangladesh (bn-BD)

Reviewer: independent machine pass, 2026-11-15. Translator's form of address: আপনি, with the polite command form
(করুন, বেছে নিন) on buttons and in instructions; kept, because it is how Windows and software made for Bangladesh speak
to the reader, and it is used the same way in every file.

## Brought up to date
30 messages. 11 changed:

- app-scan-failed: the error now follows a colon, as the English says (it followed a full stop).
- app-footprint-licenses-still-go, core-footprint-licenses-left-out, core-review-note-licenses,
  core-secret-why-license-option-off, core-warning-scan-license-data: "the License option" became the option's full name,
  লাইসেন্স ও অ্যাক্টিভেশন ডেটা, as on the checkbox (app-option-license). core-warning-scan-license-data also said
  "what can be removed" (যা সরানো যায়); it now says "what can be moved" (যা স্থানান্তর করা যায়).
- core-warning-build-recorded-folder-left-out, core-warning-build-near-part-left-out `[unreadable]`: was "there is no
  permission to read it"; now "its path leaves the drive or goes through a link, so it cannot safely be read".
- installer-license-detail: "license files and keys" became "license files, and the registry values that hold licenses".
- installer-no-programdata, installer-cli-no-programdata: was "so the install could not have been undone"; now "so
  nothing was installed: the record an install is undone by is kept there".

19 checked and already right: client-builder-done, client-wrong-code, client-pair-failed (Bengali has no capitals, and
অন্য একটি পিসি already starts the sentence); core-review-start-boot, -system, -automatic, -delayed, -manual, -disabled,
-unknown (read correctly after শুরু হয়: in core-review-service-detail and core-review-driver-detail-starts);
core-review-network-domain, -private, -public (read correctly before নেটওয়ার্কে in core-review-firewall-detail);
core-review-com-support-detail (already names each count's kind); core-rollback-setting-put-back,
core-rollback-list-entries-removed, core-rollback-list-entries-not-removed, core-rollback-firewall-removed (the English
change was a capital letter); ui-disclosure.

## Sample (back-translation)
| Message | Back-translation | Verdict |
| --- | --- | --- |
| ui-clear-search | Clear search | OK |
| ui-browse | Browse | OK |
| ui-browse-tooltip | Choose a folder for { $field } | OK |
| ui-disclosure | { $title }, { $summary }, open / closed | OK |
| ui-language | Language | OK |
| ui-machine-translated | This translation was done by machine, so it may have mistakes. | OK |
| ui-report-translation-problem | Report a translation problem | OK |
| ui-needs-direct3d-12 | Its window needs Direct3D 12 to show, which an old graphics driver or a virtual machine may not have. | Fixed: began with "its" and had no subject; now "this program's window" |
| size-bytes | { $value } bytes | OK |
| size-kb | { $value } KB | OK |
| size-mb | { $value } MB | OK |
| size-gb | { $value } GB | OK |
| size-tb | { $value } TB | OK |
| list-separator | ", " | OK |
| installer-window-title | WinAppMigrator installer | OK |
| installer-window-title-app | Install { $app_name } | OK |
| installer-tagline | Reinstalls an application collected from another PC | OK |
| installer-unnamed-app | Application | OK |
| installer-window-failed | The installer's window could not be opened: { $error } | OK |
| installer-window-failed-without-one | It can install without a window too: run this file from an administrator's command prompt with --apply, and give a folder after --target-root to choose where it installs. | OK |
| installer-checking | Checking the installer… | OK |
| installer-stub-no-package | This is a WinAppMigrator installer stub and has no application package. Make an installer with WinAppMigrator, then run the .exe it makes here. | OK |
| installer-check-stopped | The installer's check has stopped. | OK |
| installer-locate-failed | The installer could not be found: { $error } | OK |
| installer-read-failed | The installer could not be read: { $error } | OK |
| installer-cannot-be-used-here | This installer's package cannot be used on this PC: { $error } | OK |
| installer-captured-from | Where it was collected from | OK |
| installer-install-location | Install location | OK |
| installer-choose-location | Choose the install location | OK |
| installer-folder-has-files | This folder already has files. Files with the same names will be replaced, and come back if you uninstall. | OK |
| installer-room-needed | Room needed | OK |
| installer-room-short | There may not be enough room here: { $shortfall }. The package can unpack on another drive, but there must be room here for the app's files. | OK |
| installer-interrupted | An earlier install of { $app_name } never finished. Installing will undo it first. | OK |
| installer-signature | Installer | OK |
| installer-signature-checking | Checking its signature… | OK |
| installer-signature-unsigned | Not signed: publisher unknown | OK |
| installer-signature-signed-by | Signed by { $publisher } | OK |
| installer-signature-unnamed | Signed by an unnamed publisher | OK |
| installer-signature-invalid | This installer's signature is not valid ({ $reason }). It may have been changed after it was signed. | OK |
| installer-more-left-out | { $count } more were left out (one, other) | OK |
| uninstaller-plan-coming-back | The { $count } file(s) this install replaced will come back, or, if the file has changed meanwhile, be put beside it. | OK |
| uninstaller-remove-user-data-detail | { $count } item(s) of its settings, data, and license. Unless this option is on, they stay. What they replaced comes back, or is put beside them if they have changed meanwhile. | OK |
| app-column-name | Name | OK |
| app-column-publisher | Publisher | OK |
| app-column-version | Version | OK |
| app-column-scope | Scope | OK |
| app-column-arch | Architecture | Fixed: 74 to 83 px wide in a 46 px cell, so it was cut off; now আর্কি. (35 to 39 px) |
| app-scope-per-user | Per-user | Fixed: 94 px in the 76 px Scope cell, cut off; now ব্যবহারকারী (user) |
| app-scope-per-machine | Per-machine | Fixed: shortened to মেশিন (machine) to pair with the above |
| app-option-license | License and activation data | OK |
| app-option-user-data | User's data | OK |
| app-option-whose-data | Whose data | OK |
| app-build-create | Create installer | OK |
| app-build-cancel | Cancel | OK |
| app-result-copy-path | Copy path | OK |
| app-scan-look-again | Scan again | OK |
| app-network-connect | Connect | OK |
| app-list-search | Search by name or publisher (Ctrl+F) | OK |
| app-result-files-not-captured | Installer made, but { $count } file(s) could not be collected, usually because locked or unreadable. The app may not work without them. See "Warnings" below. | OK |
| app-footprint-nothing-left-out | Nothing was left out for looking like a password, license, or certificate. | Fixed: certificate was শংসাপত্র, a West Bengal word; now সার্টিফিকেট |
| core-rollback-drive-not-connected | What this install put in { $folder } is on a drive that is not connected. Connect the drive, and it can be finished. | OK |
| core-rollback-setting-not-reached | Could not reach the key / the user's environment variable / … { $name }: { $error }. | OK |
| core-rollback-service-changed | The driver / service { $name } stays, because it changed after it was installed. | Fixed: service was পরিষেবা; now সেবা (see below) |
| core-warning-scan-network-paths-not-followed | A path recorded on this PC could not be read, so nothing there was included. Each such path is either not on a disk of this PC, so looking at it would connect there, or has a name that would open something else: { $paths } | OK |
| core-warning-scan-task-unreadable | The scheduled task { $path } could not be read: { $reason }. If it runs the app, that was not included. | OK |
| core-runtime-installer-failed | The installer ended with code { $code } | OK |
| core-signature-not-microsoft | It is not Microsoft, signed by { $signer } | Fixed: read as "it is not Microsoft"; now "signed not by Microsoft but by { $signer }" (and the -unnamed one too) |
| core-archive-data-file-wrong | { $name } is not this installer's data file, or it is damaged or changed after it was made, so nothing was read from it. Copy both files again from where they were made. | OK |
| core-warning-build-base-unreadable | { $path } could not be read ({ $error }); application data kept there was not included. | OK |
| core-restore-needs-admin-target | The target root needs administrator rights: { $path } | OK |
| core-warning-restore-task-runs-as-system | The scheduled task { $name } runs as SYSTEM, with full rights over this PC. | OK |
| core-review-account-not-told | Could not tell who is signed in ({ $reason }), so per-user settings are left out, and user data goes inside the install location. | OK |
| core-policy-service-exists | { $service } is already on this PC and runs { $path }, which did not come from an earlier install here. If it is an older copy of this app, uninstall it first | OK |
| core-privilege-not-made-here | WinAppMigrator did not make { $path }, and others can change what is in it. An administrator should check and remove it: { $error } | OK |
| core-review-firewall-detail | For { $program }, on { $networks } networks (or every network), on { $ports } ports, allows/blocks incoming/outgoing connections | OK |
| core-review-service-detail | Runs { $file } as { $account }, starts: { $start } (and the fixed start types) | OK |
| core-rollback-file-stays-no-name-beside | { $path } stays, because something else that stays still uses it / it changed / it was not asked to be removed, and the file it replaced could not be put beside it: every possible name is taken. | OK |
| core-policy-fit-some-need-newer | Some of this app's programs say they need a newer Windows than this, so they will not start here: { $programs }. | OK |

78 messages; 7 fixed (2 for wording, 3 for column width, 2 for Bangladesh terms).

## Other fixes
- **Written for Bangladesh.** The translation used West Bengal (Kolkata) forms that read as Indian Bengali in
  Bangladesh. Changed everywhere, and recorded in `terms.md`:
  - plural ending -গুলি to -গুলো (198 places: সেগুলো, ফাইলগুলো, নথিগুলো…), as Bangla Academy spelling, school
    textbooks and the press in Bangladesh write it;
  - পরিষেবা to সেবা for a Windows service (59 places), the word Bangladesh uses for a service;
  - শংসাপত্র to সার্টিফিকেট for a certificate (3 places), as Bangladesh's own ICT law and everyday use say it;
  - এখনও, তখনও to এখনো, তখনো (11 places), to match the catalog's own কোনো and কখনো in Bangla Academy spelling.
- Narrow columns (measured in Nirmala UI at 13 px against the app's column widths): the Arch header and both Scope values
  overflowed and would be cut off with "…". The Scope filter above the list uses the same two words, so it is shorter too.
- core-signature-not-microsoft-unnamed: the same clarity fix as core-signature-not-microsoft.

## Open questions
- **Windows' own Bengali (Bangladesh) names could not be checked against Windows.** They are now in Bangladesh spelling,
  which may not be what the bn-BD language pack shows: the Services window (সেবাগুলো), Documents and Public Documents
  (নথিগুলো, সর্বজনীন নথিগুলো), Saved Games (সংরক্ষিত গেমগুলো), Windows Features (Windows বৈশিষ্ট্যগুলো), Accessibility
  (অ্যাক্সেসযোগ্যতা; Bangladesh's own word is often অভিগম্যতা). A native speaker with Windows in Bangla (Bangladesh) should
  compare these with Windows and correct them in `terms.md` and the catalogs.
- সেবা and পরিষেবা: if the bn-BD language pack calls a Windows service পরিষেবা or সার্ভিস, use its word.
- Firewall profiles ডোমেন, ব্যক্তিগত, সর্বজনীন (core-review-network-*): plausible, but unconfirmed against Windows Defender
  Firewall in Bengali; Bangladeshi users may know them as পাবলিক and প্রাইভেট.
- নেটওয়ার্কে ভাগ করা ফোল্ডার (shared folder): Bangladeshi readers usually say শেয়ার করা ফোল্ডার; kept, as Windows'
  word is not known.
- আর্কি. for the Arch column is an unusual abbreviation in Bengali; its sort button reads the whole word, আর্কিটেকচার.
- app-footprint-program-files (the translator's question): the English heading "Program Files" can be read as the folder
  or as "the program's files"; প্রোগ্রামের ফাইল (the program's files) fits the list it heads.

Checker: bn-BD: 0 errors, 0 warnings.
