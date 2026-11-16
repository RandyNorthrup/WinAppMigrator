# Review: Indonesian (id-ID)

Reviewer: independent machine pass, 2026-11-15. Translator's form of address: "Anda", in sentence case; kept, because
Windows in Indonesian addresses the reader as "Anda" and writes labels in sentence case.

## Brought up to date

30 messages from the English delta list.

- Rewritten to match the new English (12): `app-scan-failed` (a colon now, as the error words follow one),
  `app-footprint-licenses-still-go`, `core-footprint-licenses-left-out`, `core-review-note-licenses`,
  `core-secret-why-license-option-off`, `core-warning-scan-license-data` (all now name the option as
  "opsi Data lisensi dan aktivasi", the same words as `app-option-license`), `core-warning-build-recorded-folder-left-out`
  and `core-warning-build-near-part-left-out` (`[unreadable]` is now the safety rule: "jalurnya keluar dari drive atau
  melewati tautan, sehingga tidak aman untuk dibaca"), `core-review-com-support-detail` (each count now named by its
  kind), `installer-license-detail` (registry values that hold licenses, "nilai registri yang menyimpan lisensi", not
  keys), `installer-no-programdata` and `installer-cli-no-programdata` (nothing was installed, because the install keeps
  its undo record there).
- Added in English's position (10): `core-review-start-boot`, `-system`, `-automatic`, `-delayed`, `-manual`, `-disabled`,
  `-unknown`; `core-review-network-domain`, `-private`, `-public` ("domain", "pribadi", "publik").
- Already right, no text change (8): `client-builder-done`, `client-wrong-code`, `client-pair-failed` ("PC lain" already
  starts the sentence with a capital), `core-rollback-setting-put-back`, `core-rollback-list-entries-removed`,
  `core-rollback-list-entries-not-removed`, `core-rollback-firewall-removed` (the English capital "The" quirk never
  existed in Indonesian), `ui-disclosure` (already translated).

## Sample (back-translation)

| Message | Back-translation | Verdict |
| --- | --- | --- |
| ui-clear-search | Clear search | OK |
| ui-browse | Browse | OK |
| ui-browse-tooltip | Choose a folder for { $field } | OK |
| ui-disclosure | { $title }, { $summary }, open / closed | OK |
| ui-language | Language | OK |
| ui-machine-translated | This translation was made by machine and may contain mistakes. | OK |
| ui-report-translation-problem | Report a translation problem | OK |
| ui-needs-direct3d-12 | This program needs Direct3D 12 to draw its window, which an old graphics driver or a virtual machine may not have. | OK |
| size-bytes | { $value } byte (one form: Indonesian has only `other`) | OK |
| size-kb | { $value } KB | OK |
| size-mb | { $value } MB | OK |
| size-gb | { $value } GB | OK |
| size-tb | { $value } TB | OK |
| list-separator | ", " | OK |
| installer-window-title | WinAppMigrator Installer | OK |
| installer-window-title-app | Install { $app_name } | OK |
| installer-tagline | Reinstalls an app taken from another PC | OK |
| installer-unnamed-app | Application | OK |
| installer-window-failed | The installer window could not be opened: { $error } | OK |
| installer-window-failed-without-one | The installer can still install without a window: run this file from an administrator command prompt with --apply, and --target-root followed by a folder to choose its location. | OK |
| installer-checking | Checking the installer… | OK |
| installer-stub-no-package | This is a WinAppMigrator installer stub, and it carries no app package. Build an installer with WinAppMigrator, then run the resulting .exe here. | OK |
| installer-check-stopped | The installer check stopped. | OK |
| installer-locate-failed | Cannot find the installer: { $error } | OK |
| installer-read-failed | Cannot read the installer: { $error } | OK |
| installer-cannot-be-used-here | This installer package cannot be used on this PC: { $error } | OK |
| installer-captured-from | Taken from | OK |
| installer-install-location | Install location | OK |
| installer-choose-location | Choose install location | OK |
| installer-folder-has-files | This folder already contains files. Files with the same name will be replaced, and returned if you uninstall. | OK |
| installer-room-needed | Space needed | OK |
| installer-room-short | Space here may not be enough: { $shortfall }. The package can be extracted on another drive, but the app's files must still fit. | OK |
| installer-interrupted | An earlier install of { $app_name } never finished. Install will undo that install first. | OK |
| installer-signature | Installer | OK |
| installer-signature-checking | Checking its signature… | OK |
| installer-signature-unsigned | Not signed: unknown publisher | OK |
| installer-signature-signed-by | Signed by { $publisher } | OK |
| installer-signature-unnamed | Signed by an unnamed publisher | OK |
| installer-signature-invalid | This installer's signature is not valid ({ $reason }). The installer may have been changed since it was signed. | OK |
| installer-more-left-out (plural) | { $count } others not included | OK |
| uninstaller-plan-coming-back (plural) | { $count } files this install replaced will be returned, or placed beside a file that has changed since. | OK |
| uninstaller-remove-user-data-detail (plural) | { $count } items of its settings, data, and license. They stay unless this option is on. What they replaced will be returned, or placed beside them if they have changed since. | OK |
| app-list-search | Search by name or publisher (Ctrl+F) | OK |
| app-column-name | Name | OK |
| app-column-publisher | Publisher | OK |
| app-column-version | Version | OK |
| app-column-scope | Scope | OK |
| app-column-arch | Architecture | Fixed: "Arsitektur" is about 52 px in a 46 px header cell; now "Arsit." |
| app-column-arch-sort | Sort by architecture | OK (spoken, keeps the full word) |
| app-option-license | License and activation data | OK |
| app-option-user-data | User data | OK |
| app-option-whose-data | Whose data | OK |
| app-build-create | Create installer | OK |
| app-build-create-again | Create installer again | OK |
| app-build-cancel | Cancel | OK |
| app-scan-look-again | Scan again | OK |
| app-scan-stop | Stop | OK |
| app-scan-try-again | Try again | OK |
| app-result-copy-path | Copy path | OK |
| app-source-disconnect | Disconnect | OK |
| app-network-connect | Connect | OK |
| app-scan-failed | This app could not be scanned: { $error } | Fixed (delta): a full stop before the lowercase error words became a colon |
| core-rollback-reason-being-changed | something was changing { $folder } while this ran, so nothing more was done in it. Close what is doing it, then run this again | OK |
| core-rollback-service-account-again | Service { $name } runs as { $account }. Its password is not stored, so set the account again in Services. | OK |
| core-system-windows-32-bit | { $name }, 32-bit | OK |
| core-secret-left-out-class-server | What class { $class } says it runs | OK |
| core-warning-scan-dependency-not-hashed | Could not compute the dependency's hash: { $error } | OK |
| core-runtime-installer-failed | the installer exited with code { $code } | OK |
| core-profile-nobody-signed-in-running-not-a-person | nobody is signed in here, and { $account }, which runs this, is not a person's account | OK |
| core-scan-not-a-file | scan target is not a file: { $path } | OK |
| core-package-link-not-read | { $path } is a link, so it was not read | OK |
| core-archive-data-file-unopened | this installer's data file, { $name }, could not be opened: { $error }. Both files must stay together, in a folder this PC can read. | OK |
| core-warning-build-missing-dependency | Dependency could not be found: { $name } | OK |
| core-error-serialization | report serialization failed: { $detail } | OK |
| core-restore-refuse-records-folder | refusing to write to where installs keep their records: { $path } | OK |
| core-restore-would-install-prerequisite-winget | would install prerequisite { $name } (winget) | OK |
| core-warning-restore-earlier-never-finished | An earlier install of { $app_name } never finished. Installing will undo it first. | OK |
| core-warning-restore-prerequisite-pending-reboot | { $name } is not registered yet; it should be active after the PC restarts. | OK |
| core-policy-fit-program | { $program } is made for { $built_for }, while this PC is { $here }, so Windows will not start it | OK |
| core-policy-shell-extension-name | { $handler } on { $class } | OK |
| core-policy-virtual-store | this stands for a key belonging to the PC, and { $reason } | OK |
| core-verify-license-store-failed | License store verification failed: { $error } | OK |
| core-review-service-detail `[other]` | Runs { $file } as { $account }, startup type: { $start } | Fixed: "jenis startup:" dropped, so the new start words finish the sentence |
| core-review-driver-detail-starts | Loads { $file } into Windows itself, startup type: { $start } | Fixed: as above |

84 messages, 4 fixed (2 of them delta messages).

## Other fixes

- `core-review-service-detail` `[other]` and `core-review-driver-detail-starts` now end in `{ $start }` alone, and each
  `core-review-start-*` phrase carries its own verb ("dimulai bersama Windows, sebelum apa pun yang lain", "tidak pernah
  dimulai, karena dinonaktifkan"), because Indonesian cannot say "dimulai tidak pernah" (starts never). The phrases are
  used only in these two sentences (`start_words` in `restore_review.rs`).
- Translator comments in every file refreshed from current English with `ftl-tool.py merge` (comments only; no text
  changed by it).
- `terms.md`: added the firewall profile names, the start-phrase rule, "opsi Data lisensi dan aktivasi" for the app's
  option (separate from the installer's own "Lisensi" option), and the "Arsit." column header.
- Whole-language checks: no English left in any message; button labels are short; `Ctrl+F` kept, as Indonesian Windows
  writes it.

## Open questions

- Firewall profiles: "pribadi" for Private. Indonesian Windows has used both "Jaringan pribadi" and "Jaringan privat";
  could not confirm which Windows Defender Firewall shows today. "domain" and "publik" are certain.
- "Arsit." as the Arch column header: a native speaker may prefer another short form, or the column may be widened.
- "Tidak diketahui" (`app-scope-unknown`, `app-arch-unknown`) is wider than the Scope and Arch cells; "Tak diketahui" is
  shorter but the English "Unknown" already overflows the Arch cell, so it was left for a layout check.
- `core-review-start-system`: "saat Windows sedang dimuat" (while Windows is loading) for "as it starts"; confirm it
  reads naturally after "Memuat … ke dalam Windows itu sendiri,".
- `ui-disclosure`: "terbuka"/"tertutup" follow the English; Narrator in Indonesian says "diperluas"/"diciutkan" for its
  own expand state.
- Done-action log lines across `core.ftl` ("Menghapus { $path }", "Mengembalikan …") read as present tense in
  Indonesian; consistent, so kept, but "{ $path } dihapus" may read better to a native speaker.

Checker: id-ID: 0 errors, 0 warnings.
