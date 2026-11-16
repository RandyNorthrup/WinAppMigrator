# Review: Japanese (ja-JP)

Reviewer: independent machine pass, 2026-11-15. Translator's form of address: polite です/ます sentences with no あなた,
noun-form labels and buttons (「インストーラーの作成」「パスのコピー」), requests as 「…してください」; kept, because that is
how Japanese Windows 10 and 11 address the user. Quotation marks 「」, full-width 。、：（）, and a half-width space between
Japanese and Latin text or digits, as Microsoft's Japanese style does.

## Brought up to date

30 messages checked against the current English. 21 needed new or changed words, and two sentences that the new words
fill were rebuilt:

- `app-scan-failed`: the reason now follows a colon, not a full stop (「このアプリを調べることができませんでした：{ $error }」).
- `app-footprint-licenses-still-go`, `core-footprint-licenses-left-out`, `core-review-note-licenses`,
  `core-secret-why-license-option-off`, `core-warning-scan-license-data`: 「「ライセンス」オプション」 is now
  「「ライセンスとライセンス認証のデータ」オプション」, the words of the checkbox `app-option-license`.
- `core-warning-build-recorded-folder-left-out`, `core-warning-build-near-part-left-out` `[unreadable]`: was 「読み取れない」
  ("cannot be read"); now the safety rule: 「パスがドライブの外を指しているか、リンクを経由していて、安全に読み取れない」,
  which ends the sentence as 「…ため、除外されました。」
- `core-review-start-boot`, `-system`, `-automatic`, `-delayed`, `-manual`, `-disabled`, `-unknown` (new): 「Windows の起動時に、
  ほかのすべてより先に開始します」「Windows の起動中に開始します」「この PC の起動時に毎回開始します」「この PC の起動後に毎回少し
  遅れて開始します」「要求があったときにのみ開始します」「無効になっているため、開始しません」「開始方法は Windows で示されていません」.
  Japanese puts the verb last, and "never, as it is disabled" cannot end in 「開始します」, so each is a whole clause, phrased
  as the `[automatic]`/`[delayed]`/`[manual]` variants already were. `core-review-service-detail` `[other]` and
  `core-review-driver-detail-starts` used to end 「{ $start } で開始します」, built for a raw Windows value such as "boot";
  they now end 「…。{ $start }」 (「disk.sys を Windows 本体に読み込みます。Windows の起動時に、ほかのすべてより先に開始します」).
- `core-review-network-domain`, `-private`, `-public` (new): 「ドメイン」「プライベート」「パブリック」, as the Profile column of Windows
  Defender Firewall shows them. They fill 「（{ $networks } ネットワーク）」 in `core-review-firewall-detail`, which reads
  correctly with them (「app.exe のポート 8080 での受信接続を許可します（ドメイン、プライベート ネットワーク）」).
- `installer-license-detail`: 「ライセンス認証とライセンスのファイルおよびキー」 ("keys" could read as product keys) is now
  「ライセンス認証とライセンスのファイル、およびライセンスを保持するレジストリ値」.
- `installer-no-programdata`: said "so the install could not be undone"; now "so nothing was installed: the record an
  install is undone by is kept in that folder" (「…見つからなかったため、何もインストールされませんでした。インストールを元に戻す
  ための記録は、このフォルダーに保存されます。」).
- `installer-cli-no-programdata`: the same, as one sentence with the reason in parentheses and no full stop, as the
  other command-line reasons.

No change needed (checked):

- `client-builder-done`, `client-wrong-code`, `client-pair-failed`: Japanese has no capitals; 「別の PC」 already starts
  the sentence correctly.
- `core-review-com-support-detail`: already named each count's kind (「クラスに付随する設定：タイプ ライブラリ { $libraries } 件、
  インターフェイス { $interfaces } 件、AppID { $app_ids } 件」); comment updated.
- `core-rollback-setting-put-back`, `core-rollback-list-entries-removed`, `core-rollback-list-entries-not-removed`,
  `core-rollback-firewall-removed`: the English capital "The" never existed in Japanese; stale comments removed.
- `ui-disclosure`: already present, with the right placeholders.

## Sample (back-translation)

| Message | Back-translation | Verdict |
| --- | --- | --- |
| ui-clear-search | Clear search | OK |
| ui-browse | Browse | OK (Windows' 参照) |
| ui-browse-tooltip | Choose the folder to use for "{ $field }" | OK |
| ui-disclosure | { $title }, { $summary }, expanded / collapsed | OK |
| ui-language | Language | OK |
| ui-machine-translated | This translation was made by machine translation and may contain mistakes. | OK |
| ui-report-translation-problem | Report a translation problem | OK |
| ui-needs-direct3d-12 | Drawing the window needs Direct3D 12. Old graphics drivers or virtual machines may not be able to use it. | OK |
| size-bytes | { $value } bytes (`*[other]` only, as Japanese has no plural) | OK |
| size-kb | { $value } KB | OK |
| size-mb | { $value } MB | OK |
| size-gb | { $value } GB | OK |
| size-tb | { $value } TB | OK |
| list-separator | 、 (ideographic comma, no space) | OK |
| installer-window-title | WinAppMigrator installer | OK |
| installer-window-title-app | Installation of { $app_name } | OK |
| installer-tagline | Reinstalls an application captured on another PC | OK |
| installer-unnamed-app | Application | OK |
| installer-window-failed | Could not open the installer's window: { $error } | OK |
| installer-window-failed-without-one | It can install without a window. From a command prompt run as administrator, run this file with --apply. To choose the install location, give a folder after --target-root. | OK |
| installer-checking | Checking the installer… | OK |
| installer-stub-no-package | This is a WinAppMigrator installer stub and holds no application package. Create an installer with WinAppMigrator and run the created .exe here. | OK |
| installer-check-stopped | The check of the installer stopped. | OK |
| installer-locate-failed | Could not determine where the installer is: { $error } | OK |
| installer-read-failed | Could not read the installer: { $error } | OK |
| installer-cannot-be-used-here | This installer's package cannot be used on this PC: { $error } | OK |
| installer-captured-from | Captured from | OK |
| installer-install-location | Install location | OK |
| installer-choose-location | Choose the install location | OK |
| installer-folder-has-files | This folder already has files. Files with the same names are replaced, and come back when you uninstall. | OK |
| installer-room-needed | Free space needed | OK |
| installer-room-short | There may not be enough free space here: { $shortfall }. Unpacking can be done on another drive, but the app's files must fit here. | OK |
| installer-interrupted | An earlier install of { $app_name } did not finish. Running "Install" undoes it first. | OK |
| installer-signature | Installer | OK |
| installer-signature-checking | Checking the signature… | OK |
| installer-signature-unsigned | Not signed: the publisher is unknown | OK |
| installer-signature-signed-by | Signer: { $publisher } | OK |
| installer-signature-unnamed | Signed by a publisher with no name | OK |
| installer-signature-invalid | This installer's signature is not valid ({ $reason }). It may have been changed after it was signed. | OK |
| installer-more-left-out | { $count } more were left out | OK |
| uninstaller-plan-coming-back | The { $count } files this install replaced come back; where a file has changed since, they are put beside it. | OK |
| uninstaller-remove-user-data-detail | There are { $count } items of settings, data, and license. They stay unless this is on. What they replaced comes back, or, if it changed since, is put beside it. | OK |
| app-tagline | Turns an installed app into a self-extracting installer | OK |
| app-reload-list | Reload the application list | OK |
| app-list-count | { $count } apps | OK |
| app-list-search | Search by name or publisher (Ctrl+F) | OK (Japanese Windows writes Ctrl) |
| app-scope-per-user | Per user | Fixed: 「ユーザーごと」, six full-width characters, is 78 px at the row's 13 px and the Scope column gives 76 px, so it would be cut; now 「ユーザー別」 (65 px). The prose keeps 「ユーザーごと」 |
| app-column-name | Name | OK |
| app-column-publisher | Publisher | OK |
| app-column-version | Version | OK |
| app-column-scope | Scope | OK |
| app-column-arch | Arch | Fixed: 「アーキテクチャ」 is 84 px at the header's 12 px and the column gives 46 px; now the abbreviation 「アーキ」 (36 px) |
| app-column-arch-sort | Sort by architecture | OK (screen reader; keeps the full word) |
| app-source | Scan source | OK |
| app-option-license | License and license-activation data | OK |
| app-option-user-data | User data | OK |
| app-option-whose-data | Account it is for | OK |
| app-build-create | Create installer (noun form, as Windows buttons) | OK |
| app-scan-look-again | Look again | OK |
| app-result-copy-path | Copy path (Explorer's 「パスのコピー」) | OK |
| app-network-connect | Connect | OK |
| app-result-run-it | Run this on the other PC and { $app_name } is reinstalled. Files are checked with SHA-256. Installers you create are not signed, so Windows shows "Unknown publisher". | OK (不明な発行元 is Windows' wording) |
| core-rollback-reason-being-changed | while this ran, something was changing { $folder }, so nothing more was done there. Close what is changing it and run again | OK |
| core-rollback-key-not-removed | Could not delete the key { $name }: { $error }. | OK |
| core-rollback-earlier-installs-not-undone | An earlier install that did not finish could not be fully undone, so nothing was installed. Close what is using these and try again. { $details } | OK |
| core-warning-scan-folders-of-others-unreadable | { $count } folders outside the app could not be read, so the app's data there may be missing. | OK |
| core-warning-scan-services-unreadable | Could not read the services' registry key: { $error } | OK |
| core-runtime-winget-unusable | Could not install { $id } using winget: { $reason } | OK |
| core-scan-environment-variable-item | Environment variable { $name } | OK |
| core-discovery-root-not-a-folder | The folder to write to is not a folder: { $path } | OK |
| core-image-not-a-file | { $path } is not a file | OK |
| core-warning-build-base-unreadable | Could not read { $path } ({ $error }). Application data stored there is not included. | OK |
| core-error-archive | Archive operation failed: { $detail } | OK |
| core-error-io | I/O for { $path } failed: { $error } | OK (reads also when { $path } is a phrase such as 「インストーラー」) |
| core-restore-file-changed | { $path } differs from what the package holds, so it was not installed | OK |
| core-restore-would-install-driver | Will install driver: { $name } -> { $destination } | OK |
| core-warning-restore-verification-failed | Could not run the verification after restoring: { $error } | OK |
| core-warning-restore-environment-key-not-created | Could not create the environment variables' registry key: { $error } | OK |
| core-policy-fit-none-start | None of its programs can start on this PC ({ $here }): { $programs } | OK |
| core-review-font-detail | Loads from the install folder at sign-in (for everyone on this PC / only for the current user) | OK |
| core-warning-review-documents-unknown | { $count } files from Documents are put instead in a folder named _external under { $target }, because where this PC's Documents folder is could not be told. To use them, move them to the Documents folder yourself. | OK |
| core-review-service-detail | Runs { $file } as { $account }. Starts now, and then each time this PC starts / … / `[other]` { $start } | Fixed: `[other]` ended 「{ $start } で開始します」, made for a raw Windows value; the start sentence now follows directly |
| core-review-driver-detail-starts | Loads { $file } into Windows itself. { $start } | Fixed: as above |
| core-review-firewall-detail | Allows incoming connections of { $program } on ports { $ports } (domain, private networks) | OK with the new profile words |

84 messages: 80 OK, 4 fixed.

## Other fixes

- Comments brought in line with the current English (they explain each message to the next translator):
  `app-footprint-licenses-still-go`, `app-scan-failed`, `core-review-com-support-detail`, `core-review-service-detail`,
  `core-review-driver-detail-starts`, the profile note above `core-review-network-domain`, `core-rollback-count-kept`,
  `core-rollback-count-left-behind`, `core-error-io`; the stale "Look Again" note on `app-scan-stopped` and the capital
  "The" notes on `core-rollback-setting-put-back` and `core-rollback-list-entries-removed` removed. Every ja-JP comment
  now matches en-US, and every message is in English's order.
- `terms.md`: added the firewall profile names, the column abbreviation 「アーキ」, the short scope label 「ユーザー別」, how the
  start words are built, and that the option is always named 「ライセンスとライセンス認証のデータ」 (the installer's own
  heading `installer-license` stays 「ライセンス」, as English's "License").
- Checked across the language: no English left that should not be; button labels short; 「Ctrl+F」 as Japanese Windows
  writes it; plurals use only `*[other]`, as CLDR gives Japanese; command-line usage columns stay aligned in a
  double-width console.

## Open questions

- `app-column-arch` 「アーキ」 is an informal abbreviation (Task Manager writes 「アーキテクチャ」 in a wider column). A native
  speaker may prefer 「CPU」, or the column could be widened for the full word.
- `app-scope-per-user` 「ユーザー別」 in the narrow column and the filter, while sentences say 「ユーザーごと」: confirm the pair
  reads naturally beside 「PC 全体」, or widen the Scope column (88 px) by a few pixels and go back to 「ユーザーごと」.
- `ui-disclosure` 「展開」「折りたたみ」: confirm these match what Japanese Narrator says for expanded and collapsed headings.
- `core-rollback-count-kept` 「そのまま残した項目」 (left on purpose) and `core-rollback-count-left-behind` 「残った項目」 (no
  uninstall can take it back) are close in Japanese; a clearer pair may exist.
- 「Windows ファイアウォール」 follows the English "Windows Firewall" and Windows 7, where the client runs; Windows 10 and
  11 call it 「Windows Defender ファイアウォール」. Not confirmed against Windows itself.
- The profile names 「ドメイン」「プライベート」「パブリック」 are from Windows Defender Firewall with Advanced Security, as I
  know it; not confirmed against a Japanese Windows 11.

Checker: ja-JP: 0 errors, 0 warnings.
