# Review: Polish (pl-PL)

Reviewer: independent machine pass, 2026-11-15. Translator's form of address: second person singular, with imperatives
for actions (Wybierz, Zamknij, Uruchom ponownie) and "Czy chcesz…" for questions, no personal pronouns; kept, because
that is how Polish Windows 10 and 11 address the user (for example UAC's "Czy chcesz zezwolić tej aplikacji na
wprowadzanie zmian…").

## Brought up to date

30 messages checked against the current English. 7 needed new words:

- `app-scan-failed`: the reason now follows a colon, not a full stop ("Nie można przeanalizować tej aplikacji: …").
- `core-warning-build-recorded-folder-left-out`, `core-warning-build-near-part-left-out` `[unreadable]`: was "it cannot
  be read"; now the safety rule: "jego ścieżka wychodzi poza dysk lub prowadzi przez link, więc nie można go bezpiecznie
  odczytać".
- `core-review-com-support-detail`: each count now sits after its own kind, as in English, instead of "odpowiednio: …".
- `installer-license-detail`: "klucze" (could read as product keys) is now "wartości rejestru przechowujące licencje".
- `installer-no-programdata`, `installer-cli-no-programdata`: was "so the install could not be undone"; now "więc
  niczego nie zainstalowano: instalacja przechowuje w nim zapis, dzięki któremu można ją cofnąć".

23 already said what the current English says: `app-footprint-licenses-still-go`, `client-builder-done`,
`client-wrong-code`, `client-pair-failed`, `core-footprint-licenses-left-out`, `core-review-start-boot`,
`core-review-start-system`, `core-review-start-automatic`, `core-review-start-delayed`, `core-review-start-manual`,
`core-review-start-disabled`, `core-review-start-unknown`, `core-review-network-domain`, `core-review-network-private`,
`core-review-network-public`, `core-review-note-licenses`, `core-rollback-setting-put-back`,
`core-rollback-list-entries-removed`, `core-rollback-list-entries-not-removed`, `core-rollback-firewall-removed`,
`core-secret-why-license-option-off`, `core-warning-scan-license-data`, `ui-disclosure`.

Notes on these:

- "the License and Activation Data option" is "opcja Dane licencji i aktywacji" everywhere, the same words as
  `app-option-license`.
- The `core-review-start-*` words were read inside the sentences they fill. Both `core-review-service-detail` `[other]`
  and `core-review-driver-detail-starts` end with "uruchamianie: { $start }", and every start word reads correctly
  after it. "nigdy, ponieważ jest wyłączone" is neuter because it agrees with "uruchamianie", which avoids any clash
  between "usługa" (feminine) and "sterownik" (masculine).
- `core-review-network-*` are Domena, Prywatny, Publiczny. As far as this review could tell, those are the profile names
  Windows Defender Firewall shows for a rule. They read correctly in `core-review-firewall-detail` ("w profilu sieci
  Prywatny", "w profilach sieci Domena, Prywatny").
- The `client-*` `[unknown]` variants start their sentences with a capital ("Inny komputer"). `client-pair-failed`
  starts with "Parowanie z innym komputerem", which is natural Polish and keeps the meaning.

## Sample (back-translation)

| Message | Back-translation | Verdict |
| --- | --- | --- |
| ui-clear-search | Clear search | OK |
| ui-browse | Browse | OK |
| ui-browse-tooltip | Choose a folder for the field { $field } | OK |
| ui-disclosure | { $title }, { $summary }, expanded / collapsed | OK |
| ui-language | Language | OK |
| ui-machine-translated | This translation was made by machine and may contain errors. | OK |
| ui-report-translation-problem | Report a problem with the translation | OK |
| ui-needs-direct3d-12 | Showing the window requires the Direct3D 12 interface, which an old graphics card driver or a virtual machine may not support. | OK |
| size-bytes | { $value } byte / bytes (one, few, many, other) | OK |
| size-kb | { $value } KB | OK |
| size-mb | { $value } MB | OK |
| size-gb | { $value } GB | OK |
| size-tb | { $value } TB | OK |
| list-separator | ", " | OK |
| installer-window-title | WinAppMigrator Installer | OK |
| installer-window-title-app | Installation of the app { $app_name } | OK |
| installer-tagline | Reinstalls an app captured on another computer | OK |
| installer-unnamed-app | Application | OK |
| installer-window-failed | The installer's window cannot be opened: { $error } | OK |
| installer-window-failed-without-one | Installing also works without a window: run this file in an administrator command prompt with --apply, and to choose the location add --target-root and a folder path. | OK |
| installer-checking | Checking the installer… | OK |
| installer-stub-no-package | This is the WinAppMigrator installer stub, and it holds no app package. Create an installer with WinAppMigrator, then run the created .exe here. | OK |
| installer-check-stopped | Checking the installer was interrupted. | OK |
| installer-locate-failed | The installer cannot be located: { $error } | OK |
| installer-read-failed | The installer cannot be read: { $error } | OK |
| installer-cannot-be-used-here | This installer's package cannot be used on this computer: { $error } | OK |
| installer-captured-from | Captured from | OK |
| installer-install-location | Install location | OK |
| installer-choose-location | Choose the install location | OK |
| installer-folder-has-files | This folder already contains files. Files with the same names will be replaced, and will come back if you uninstall the app. | OK |
| installer-room-needed | Space needed | OK |
| installer-room-short | There may not be enough space in this location: { $shortfall }. The package can be unpacked on another drive, but the app's files still have to fit. | OK |
| installer-interrupted | An earlier installation of { $app_name } was not finished. The installation will undo it first. | OK |
| installer-signature | Installer | OK |
| installer-signature-checking | Checking the signature… | OK |
| installer-signature-unsigned | Not signed: unknown publisher | OK |
| installer-signature-signed-by | Signed by: { $publisher } | OK |
| installer-signature-unnamed | Signed by an unnamed publisher | OK |
| installer-signature-invalid | This installer's signature is invalid ({ $reason }). The installer may have been changed after it was signed. | OK |
| installer-more-left-out | { $count } more item(s) were left out (one, few, many, other) | OK |
| installer-result-changes | Changes ({ $count }) | OK |
| installer-result-changes-more | and { $count } more. | OK |
| installer-cli-sets-up | Items configured: { $count } | OK |
| installer-cli-changes-logged | Changes written to the log: { $count } | OK |
| installer-cli-warnings | Warnings: { $count } | OK |
| installer-cli-errors | Errors: { $count } | OK |
| installer-cli-refused | Refused: { $count } | OK |
| uninstaller-plan-coming-back | { $count } file(s) replaced by this install will go back, or go beside the file(s) that changed since then (one, few, many, other) | OK |
| uninstaller-remove-user-data-detail | { $count } item(s) of the app's settings, data, and license. They stay unless this option is on. What they replaced goes back, or goes beside them if they changed since. | OK |
| uninstaller-and-more | and { $count } more. | OK |
| app-column-name | Name | OK |
| app-column-publisher | Publisher | OK |
| app-column-version | Version | OK |
| app-column-scope | Scope | OK |
| app-column-arch | Arch. (short for architecture) | OK |
| app-build-create | Create installer | OK |
| app-build-cancel | Cancel | OK |
| app-scan-look-again | Analyze again | OK |
| app-scan-try-again | Try again | OK |
| app-result-open-folder | Open folder | OK |
| app-option-license | License and activation data | OK |
| app-option-user-data | User data | OK |
| app-option-whose-data | Whose data | OK |
| app-output-folder | Output folder | OK |
| app-source | Analysis source | OK |
| app-list-count | { $count } app(s) (one, few, many, other) | OK |
| app-scan-failed | This app cannot be analyzed. { $error } | Fixed: brought up to date. The reason now follows a colon, as English and the lowercase `core-scan-*` reasons expect. |
| app-footprint-data-only-with-user-data | { $size } of this app's data in { $count } file(s) will be included … | Fixed: "{ $size } danych … zostanie dołączone" did not agree with a size like "824,0 MB" (that needs "dołączonych"). Now "Dane tej aplikacji ({ $size } w { $count } plikach) zostaną dołączone tylko z opcją Dane użytkownika", which is grammatical for any size. |
| core-rollback-drive-not-connected | What this install put in folder { $folder } is on a drive that is not connected. Connect it so this can be finished. | OK |
| core-rollback-firewall-not-read | The firewall rule { $name } cannot be read: { $error }. | OK |
| core-system-not-a-hive-file | this is not a registry hive file: { $path } | OK |
| core-warning-scan-folders-of-others-unreadable | { $count } folder(s) outside the app cannot be read, so the app's data there may be missing. | OK |
| core-runtime-payload-missing | the bundled installer is missing: { $path } | OK |
| core-discovery-install-location-not-full-path | the install location must be a full path: „{ $path }” | OK |
| core-image-not-attachable | the file { $path } is not a disk image Windows can attach (.vhd or .vhdx) | OK |
| core-warning-build-runtime-no-installer | The app needs the runtime { $runtime }, and there is no installer for it here, so none will be run. Install it yourself on the other computer. | OK |
| core-restore-needs-admin-target | the target root requires elevated permissions: { $path } | OK |
| core-restore-would-create-empty-folder | to be created: empty folder { $path } | OK |
| core-warning-restore-prerequisite-reboot | The runtime { $name } was installed, but a restart is needed to finish the installation. | OK |
| core-policy-fit-driver | the architecture of its driver { $driver } is { $built_for }, and this computer's is { $here }. A driver must match Windows exactly, so this app cannot run here | OK |
| core-review-autostart-detail | Starts when any user signs in / when the current user signs in | OK |
| core-review-service-detail | Runs file { $file } as { $account }; start: now and at every computer start / … / { $start } | OK |
| core-review-driver-detail-starts | Loads file { $file } directly into Windows; start: { $start } | OK |
| core-review-firewall-detail | Allows incoming connections on port(s) { $ports } of program { $program } in network profile(s) { $networks } / in every network | OK |
| core-review-com-support-detail | Settings of type libraries, interfaces, and AppIDs (respectively: …), linked to the app's classes | Fixed: brought up to date, each count now follows its own kind. |
| core-privilege-not-a-folder | the item { $path } is a link or a file, not a folder | OK |

86 messages sampled, 3 fixed (2 because the English had changed, 1 for grammar).

Checked across the whole language: the `app-column-*` headers are short (Nazwa, Wydawca, Wersja, Zakres, Arch.); button
labels are short (Utwórz instalator, Anuluj, Zainstaluj, Podgląd zmian, Analizuj ponownie, Spróbuj ponownie); nothing is
left in English that should not be. `Ctrl+F` is kept, as Polish Windows writes it.

## Other fixes

- Comments in all nine files resynced with the English comments using `scripts\ftl-tool.py merge`. Only comments changed,
  and the diff against the pre-review copy was checked to confirm it. Stale comments are gone, for example "{ $error } is
  why, as a sentence" on `app-scan-failed`, the capital "The" note on `core-rollback-*`, and the old count note on
  `core-review-com-support-detail`. The new English notes are now present, for example on `core-error-io` and
  `core-rollback-count-kept` / `-left-behind`. Their Polish already fit them: "Zachowane" for what is kept on purpose,
  "Pozostawione" for what no uninstall can take back.
- `terms.md` is unchanged. The new words use its terms: "wartość rejestru", "zapis instalacji", "cofnąć", "link".

## Open questions

- `core-review-network-domain/private/public` (Domena, Prywatny, Publiczny): this review believes these are the profile
  names on a firewall rule's Advanced tab in Polish Windows Defender Firewall, but could not confirm them on a Polish
  Windows. Please also check that "w profilach sieci Domena, Prywatny" reads well.
- `ui-disclosure`: "rozwinięty" / "zwinięty". A native screen-reader user may prefer the neuter state words
  "rozwinięte" / "zwinięte" that some Polish screen readers say.
- `core-space-shortfall` (fills `installer-room-short`): "C: potrzebuje do 824,0 MB wolnego miejsca na …, a ma 300,0 MB"
  uses the drive letter as the subject, following English. It is understandable, but "Na dysku C: potrzeba…" may read
  more naturally.
- `core-review-firewall-detail`: "połączenia na porcie 80 programu app.exe" could be misread as "port 80 of app.exe".
  Kept, because it follows English's order and the meaning is clear in context.

Checker: pl-PL: 0 errors, 0 warnings.
