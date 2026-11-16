# Review: Turkish (tr-TR)

Reviewer: independent machine pass, 2026-11-15. Translator's form of address: *siz*, with plural imperatives ("seçin",
"yazın", "bilgisayarınız"); kept, because Windows 11 in Turkish speaks to the reader the same way.

Suffixes and placeholders: I checked every placeholder in all nine files, not only the sample, for a suffix that depends
on a value the translator could not see. None was attached to a placeholder (no `{ $x }'ın`, `{ $x }'e`, or `{ $count }'i`).
Each case ending goes on a noun after the placeholder ("{ $path } klasörüne", "{ $name } hizmetinin",
"{ $drive } sürücüsünde"), on a postposition that takes the bare noun (için, ile, olarak, üzerinde, gibi), or the
placeholder comes after a colon. Placeholders filled with Turkish words (`$purposes`, `$account`, `$start`, `$networks`)
are written in the nominative, and each sentence was checked with them filled in. Apostrophe suffixes on fixed names
(Windows'un, WinAppMigrator'da, Installer'ın, ODBC'nin, 11'de, .NET'i) follow vowel harmony.

## Brought up to date

30 messages. 22 were edited:

- `app-scan-failed`: now ends with a colon, not a full stop.
- `app-footprint-licenses-still-go`, `core-footprint-licenses-left-out`, `core-review-note-licenses`,
  `core-secret-why-license-option-off`, `core-warning-scan-license-data`: "Lisans seçeneği" is now
  "Lisans ve etkinleştirme verileri seçeneği", the same words as `app-option-license`.
- `core-warning-build-recorded-folder-left-out`, `core-warning-build-near-part-left-out`: `[unreadable]` changed from
  "okunamayabilir" (may be unreadable) to a safety rule: "yolu sürücünün dışına çıkıyor veya bir bağlantı üzerinden
  gidiyor, bu nedenle güvenle okunamaz".
- `core-review-start-boot`, `-system`, `-automatic`, `-delayed`, `-manual`, `-disabled`, `-unknown` (new). Each
  carries its own verb ("… başlar", or "hiçbir zaman başlamaz" for disabled), because Turkish "never" needs a negative
  verb. They complete the sentences after a semicolon, like the service line's other variants.
- `core-review-network-domain`, `-private`, `-public` (new): Etki alanı, Özel, Ortak.
- `core-review-com-support-detail`: each count now follows its kind: "tür kitaplığı ({ $libraries }), arabirim
  ({ $interfaces }) ve AppID ({ $app_ids }) ayarları".
- `installer-license-detail`: "anahtarlar" (keys) became "lisans bilgilerini tutan kayıt defteri değerleri".
- `installer-no-programdata`, `installer-cli-no-programdata`: these used to say the install could not be undone. They
  now say nothing was installed, because an install keeps the record it is undone by in ProgramData.

8 already said the same as the English and were left as they were: `client-builder-done`, `client-wrong-code`,
`client-pair-failed` (they already began with a capitalised "Başka bir bilgisayar"), `core-rollback-setting-put-back`,
`core-rollback-list-entries-removed`, `core-rollback-list-entries-not-removed`, `core-rollback-firewall-removed` (the
English fix was only a capital letter mid-sentence), and `ui-disclosure` (already translated).

## Sample (back-translation)

| Message | Back-translation | Verdict |
| --- | --- | --- |
| ui-clear-search | Clear the search | OK |
| ui-browse | Browse | OK (Windows' "Gözat") |
| ui-browse-tooltip | Choose a folder for { $field } | OK |
| ui-disclosure | { $title }, { $summary }, expanded / collapsed | OK (Narrator's words) |
| ui-language | Language | OK |
| ui-machine-translated | This translation was made by machine and may contain errors. | OK |
| ui-report-translation-problem | Report a translation problem | OK |
| ui-needs-direct3d-12 | This program needs Direct3D 12 to draw its window. An old graphics driver or a virtual machine may not provide it. | OK |
| size-bytes | { $value } byte(s) | OK (Turkish does not pluralise after a number) |
| size-kb | { $value } KB | OK |
| size-mb | { $value } MB | OK |
| size-gb | { $value } GB | OK |
| size-tb | { $value } TB | OK |
| list-separator | ", " | OK |
| installer-window-title | WinAppMigrator Installer | OK |
| installer-window-title-app | Install the { $app_name } app | OK (see Open questions) |
| installer-tagline | Reinstalls an app taken from another PC | OK |
| installer-unnamed-app | Application | OK |
| installer-window-failed | The installer's window could not open: { $error } | OK |
| installer-window-failed-without-one | It can also install without a window: run this file from an administrator command prompt with --apply. To choose where, add --target-root and then a folder. | OK |
| installer-checking | Checking the installer… | OK |
| installer-stub-no-package | This is a WinAppMigrator installer stub and carries no app package. Create an installer with WinAppMigrator, then run the created .exe here. | OK |
| installer-check-stopped | The installer's check stopped. | OK |
| installer-locate-failed | The installer could not be found: { $error } | OK |
| installer-read-failed | The installer could not be read: { $error } | OK |
| installer-cannot-be-used-here | This installer's package cannot be used on this PC: { $error } | OK |
| installer-captured-from | The PC it was taken from | OK |
| installer-install-location | Install location | OK |
| installer-choose-location | Choose the install location | OK |
| installer-folder-has-files | This folder already has files. Files with the same name are overwritten; if you uninstall the app, the old files come back. | OK |
| installer-room-needed | Space needed | OK |
| installer-room-short | There may not be enough room here: { $shortfall }. The package can be unpacked on another drive, but the app's files still have to fit. | OK |
| installer-interrupted | An earlier install of the { $app_name } app never finished. When Install is chosen, that install is undone first. | OK |
| installer-signature | Installer | OK |
| installer-signature-checking | Checking its signature… | OK |
| installer-signature-unsigned | Unsigned: publisher unknown | OK |
| installer-signature-signed-by | Signed by: { $publisher } | OK |
| installer-signature-unnamed | Signed by a publisher whose name is not given | OK |
| installer-signature-invalid | This installer's signature is not valid ({ $reason }). It may have been changed after it was signed. | OK |
| installer-more-left-out (plural) | { $count } more items were left out | OK |
| uninstaller-plan-coming-back (plural) | The { $count } file(s) this install overwrote come back, or are put beside the file(s) changed since. | OK |
| uninstaller-remove-user-data-detail (plural) | { $count } item(s) of its settings, data, and license. They stay unless this option is on. What they overwrote comes back; if the items have changed since, it is put beside them. | OK |
| app-build-create | Create installer | OK |
| app-build-create-again | Create the installer again | OK |
| app-build-cancel | Cancel | OK |
| app-scan-look-again | Look again | OK |
| app-scan-stop | Stop | OK |
| app-scan-try-again | Try again | OK |
| app-result-open-folder | Open folder | OK |
| app-result-copy-path | Copy path | OK |
| app-network-connect | Connect | OK |
| app-column-name | Name | OK, short |
| app-column-publisher | Publisher | OK, short |
| app-column-version | Version | OK, short |
| app-column-scope | Scope | OK, short |
| app-column-arch | Architecture | OK, 6 letters ("Mimari"), fits the column |
| app-option-license | License and activation data | OK |
| app-option-user-data | User data | OK |
| app-option-whose-data | Whose data | OK |
| app-list-search | Search by name or publisher  (Ctrl+F) | OK (Turkish keyboards label it Ctrl) |
| app-list-count | { $count } app(s) | OK |
| app-room-drive-short | To build this, up to { $needed } free space is needed on drive { $drive }; the drive has { $free } free. | OK |
| app-result-run-it | To reinstall the { $app_name } app, run this on the other PC. It checks its files with SHA-256. Because installers you build are unsigned, Windows shows an "Unknown publisher" warning. | OK |
| app-scan-failed | This app could not be examined. { $error } | Fixed: colon, as the English now has |
| core-rollback-file-stays | { $path } stays, because something that stays still uses it / because it changed after it was installed / because its removal was not asked for. | OK |
| core-system-registry-through-link | { $drive } is not a Windows system drive: the path to its registry goes through a link. | OK |
| core-warning-scan-drive-paths-not-followed | A path / some paths recorded on this drive could not be read, so nothing there was included. Each is on another disk, is not a full path, or goes through a link that would lead off the drive: { $paths } | OK |
| core-runtime-download-failed | { $url } could not be downloaded: { $error } | OK |
| core-secret-taken-from | { $option } (in the command that runs the service / task { $name }; in the shortcut { $name }; …) | OK |
| core-rollback-earlier-install-stuck | { $app_name }: { $failures } If it can never be undone, an administrator can give up on this install by removing the { $folder } folder. | OK |
| core-runtime-feature-failed | DISM could not enable the { $feature } Windows feature (exit code { $code }); the payload most likely could not be downloaded. Enable it from Windows Features, or run … | OK |
| core-scan-no-executable | No executable was found for { $app_name }. A driver, runtime, or add-in may have no program of its own to package. | OK |
| core-near-hidden-part-beside | A part hidden in the app list, installed beside this app, in the { $folder } folder. | OK |
| core-warning-build-runtime-no-installer | The app needs the { $runtime } runtime, and as there is no installer for it here, none will be run. Install it yourself on the other PC. | OK |
| core-restore-already-exists | { $path } already exists, so nothing was installed. Use --force to overwrite it | OK |
| core-warning-restore-prerequisite-reboot | { $name } was installed, but a restart is needed to complete it. | OK |
| core-restore-location-unusable | this install location cannot be used: { $problem }: "{ $path }" | OK |
| core-review-other-settings-title | { $count } setting(s) outside the app's own keys | OK |
| core-review-shortcut-detail | Opens { $file }, for everyone on this PC / for you | OK |
| core-space-shortfall | On { $drive }, up to { $needed } free space is needed for { $purposes }; { $free } is free now | OK (each `core-space-for-*` value reads correctly before "için") |
| core-policy-fit-needs-newer | { $program } says it needs Windows { $needs } or newer, and this PC runs Windows { $runs }; so Windows would not start it | OK |

81 messages sampled. One sample message needed a fix (`app-scan-failed`, which was also on the delta list). Nothing else
in the sample was wrong.

## Other fixes

- `core-review-service-detail` `[other]` and `core-review-driver-detail-starts`: changed "…; başlatma: { $start }"
  (start: …) to "…; { $start }". The new start words carry their own verb, so the line now reads as a sentence, such
  as "disk.sys dosyasını doğrudan Windows'a yükler; Windows ile birlikte, her şeyden önce başlar".
- `core-review-firewall-detail`: removed the comma after "{ $program } için". The network list brings its own commas
  ("Etki alanı, Özel türündeki ağlarda"), and the extra comma made the line hard to read.
- Updated the `#` comments of the changed messages to the current English comments.
- `terms.md`: added two rows: the firewall profiles (Etki alanı, Özel, Ortak) and "disabled" (devre dışı).

## Open questions

- Firewall profile names: confirm "Etki alanı / Özel / Ortak" and how they are capitalised in Turkish Windows Defender
  Firewall. wf.msc may write "Etki Alanı". Settings and Windows Security show "Etki alanı ağı / Özel ağ / Ortak ağ".
- `core-review-start-*`: a native speaker should confirm the phrasing reads naturally. I kept the English's descriptive
  meaning rather than the startup type names in the Services console (Otomatik, Otomatik (Gecikmeli Başlatma), El ile,
  Devre dışı). Boot and system start have no name shown there.
- "Bilinmeyen yayımcı" (`app-result-run-it`, `app-details-unknown-publisher`): confirm against what Turkish Windows
  shows for an unsigned program. The User Account Control prompt may show "Yayımcı: Bilinmiyor", and the file security
  warning may show "Bilinmeyen Yayımcı".
- `installer-window-title-app`: when a package has no app name, `{ $app_name }` is "Uygulama", and the title becomes
  "Uygulama uygulamasını yükle". It is readable but awkward. The app's code would need a separate unnamed-app title
  to avoid it.
- `app-network-type-name-example` "ESKI-PC" is written without the dotted İ, presumably because computer names are
  usually plain ASCII. Left as it is.

Checker: tr-TR: 0 errors, 0 warnings.
