# Review: Korean (ko-KR)

Reviewer: independent machine pass, 2026-11-15. Translator's form of address: Windows 11 Korean style, with statements
in 합니다체 (-습니다/-ㅂ니다), instructions in -하세요, and noun phrases for titles, labels and buttons (설치 관리자 만들기,
경로 복사). The reader is never called 당신. Kept, because Korean Windows addresses the user this way. Particles after a
placeholder use Microsoft's 을(를), 이(가), 은(는), 와(과) forms, or a noun goes after the placeholder (`{ $name }
서비스`, `파일 { $count }개`). A scan of every placeholder in all nine files found no bare 을/를/이/가/은/는/로 after a
placeholder. Particles after a closing parenthesis agree with the word before it (이름(…)으로, 게시자(…)가, 공간(…)이).

## Brought up to date

30 messages checked against the current English. 22 needed new or changed words, and two sentences that the new words
fill were rebuilt:

- `app-scan-failed`: the reason now follows a colon, not a full stop (이 앱을 검사할 수 없습니다: { $error }).
- `app-footprint-licenses-still-go`, `core-footprint-licenses-left-out`, `core-review-note-licenses`,
  `core-secret-why-license-option-off`, `core-warning-scan-license-data`: "라이선스" 옵션 and 라이선스 옵션 are now
  "라이선스 및 정품 인증 데이터" 옵션, the words of the checkbox `app-option-license`. The name is quoted because it is
  long and contains 및 and 데이터, which otherwise run into the sentence.
- `core-warning-build-recorded-folder-left-out`, `core-warning-build-near-part-left-out` `[unreadable]`: was 읽을 수 없는
  폴더/항목이기 때문입니다 ("because it cannot be read"). It now gives the safety rule: 경로가 드라이브를 벗어나거나 링크를
  거치므로 안전하게 읽을 수 없기 때문입니다.
- `core-review-start-boot`, `-system`, `-automatic`, `-delayed`, `-manual`, `-disabled`, `-unknown` (new). Each is a
  whole clause, as the service variants already were: Windows와 함께 가장 먼저 시작됩니다, Windows가 시작될 때 함께
  시작됩니다, 이 PC가 시작될 때마다 시작됩니다, 이 PC가 시작될 때마다 잠시 후 시작됩니다, 무언가가 요청할 때만
  시작됩니다, 사용 안 함 상태이므로 시작되지 않습니다 (사용 안 함 is the start type's name in Services), 시작 방식은
  Windows에서 알려 주지 않습니다.
- `core-review-service-detail` `[other]` and `core-review-driver-detail-starts` were rebuilt to take those clauses. They
  had said 시작 유형은 { $start }입니다 ("its start type is …"), which reads wrongly with the new phrases. They now say
  …하며, { $start }. For example: disk.sys을(를) Windows 자체에 로드하며, Windows와 함께 가장 먼저 시작됩니다.
- `core-review-network-domain`, `-private`, `-public` (new): 도메인, 개인, 공용, the profile names Windows Defender
  Firewall uses. They fill `core-review-firewall-detail`, which already said { $networks } 네트워크에서. For example: app.exe에
  대해 도메인, 개인 네트워크에서 포트 8080, 8081의 들어오는 연결을 허용합니다.
- `core-review-com-support-detail`: was 각각 N개, N개, N개의 설정 ("N, N and N settings respectively"), which did not say
  which count was which. Now: 클래스와 함께 사용되는 설정: 형식 라이브러리 N개, 인터페이스 N개, AppID N개.
- `installer-license-detail`: the "keys" were 키 (which could be read as product keys). Now 라이선스 데이터가 들어 있는
  레지스트리 값 (registry values that hold license data).
- `installer-no-programdata`, `installer-cli-no-programdata`: had said "so the install cannot be undone". Now they say
  that nothing was installed, because the install record it is undone by is kept there: 이 PC의 ProgramData 폴더를 찾을
  수 없어 아무것도 설치하지 않았습니다. 설치를 되돌리는 데 쓰이는 설치 기록을 이 폴더에 보관하기 때문입니다. The CLI
  form has no final full stop, like the other CLI reasons.

These 8 already said what the current English says and were left alone:

- `client-builder-done`, `client-wrong-code`, `client-pair-failed`: Korean has no capital letters, and 다른 PC에서 already
  starts the sentence.
- `core-rollback-setting-put-back`, `core-rollback-list-entries-removed`, `core-rollback-list-entries-not-removed`,
  `core-rollback-firewall-removed`: the English change was only the stray capital "The".
- `ui-disclosure`: the translation was already present and right: 확장됨/축소됨, the words Narrator uses for these states.

`merge` was run to put the new messages in English's place, which also brought the English comments in every ko-KR file
up to date. No translation changed except those listed in this report.

## Sample (back-translation)

| Message | Back-translation | Verdict |
| --- | --- | --- |
| ui-clear-search | Clear search | OK |
| ui-browse | Browse | OK |
| ui-browse-tooltip | Select the folder to use for { $field } | OK |
| ui-disclosure | { $title }, { $summary }, expanded / collapsed | OK |
| ui-language | Language | OK |
| ui-machine-translated | This translation was made by machine translation and may have errors | Fixed: said "translation … by machine translation". Now 이 번역은 기계 번역이며 오류가 있을 수 있습니다 |
| ui-report-translation-problem | Report translation problem | OK |
| ui-needs-direct3d-12 | Direct3D 12 is needed to draw the window. Old graphics drivers or virtual machines may not support it. | OK |
| size-bytes | { $value } bytes (other only) | OK |
| size-kb | { $value }KB | OK (no space, as Korean Explorer writes sizes) |
| size-mb | { $value }MB | OK |
| size-gb | { $value }GB | OK |
| size-tb | { $value }TB | OK |
| list-separator | ", " | OK |
| installer-window-title | WinAppMigrator installer | OK |
| installer-window-title-app | { $app_name } install | OK |
| installer-tagline | Reinstalls an app captured on another PC | OK |
| installer-unnamed-app | App | OK |
| installer-window-failed | Cannot open the installer window: { $error } | OK |
| installer-window-failed-without-one | It can install without a window too. Run this file with the --apply option from an administrator command prompt. To choose where, give a folder after --target-root. | OK |
| installer-checking | Checking the installer… | OK |
| installer-stub-no-package | This file is a WinAppMigrator installer stub and has no app package. Make an installer with WinAppMigrator, then run the .exe it made here. | OK |
| installer-check-stopped | The installer check stopped. | OK |
| installer-locate-failed | Cannot find the installer: { $error } | OK |
| installer-read-failed | Cannot read the installer: { $error } | OK |
| installer-cannot-be-used-here | This installer's package cannot be used on this PC: { $error } | OK |
| installer-captured-from | Capture source | OK |
| installer-install-location | Install location | OK |
| installer-choose-location | Choose install location | OK |
| installer-folder-has-files | This folder already has files. Files with the same name are replaced, and if you uninstall, the original files come back. | OK |
| installer-room-needed | Space needed | OK |
| installer-room-short | There may not be enough space here: { $shortfall }. Unpacking can be done on another drive, but there must be room for the app's files. | OK (read with core-space-shortfall filled in) |
| installer-interrupted | The install of { $app_name } started earlier did not finish. Clicking "Install" first undoes that install. | OK |
| installer-signature | Installer | OK |
| installer-signature-checking | Checking the signature… | OK |
| installer-signature-unsigned | Not signed: Unknown publisher | OK (Windows' 알 수 없는 게시자) |
| installer-signature-signed-by | Signer: { $publisher } | OK |
| installer-signature-unnamed | Signed by a publisher with no name | OK |
| installer-signature-invalid | This installer's signature is not valid ({ $reason }). It may have been changed after it was signed. | OK |
| installer-more-left-out | { $count } more excluded items | OK (plural, other only) |
| uninstaller-plan-coming-back | { $count } files replaced by this install go back to how they were. If a file has changed since, they are put beside it. | OK (plural) |
| uninstaller-remove-user-data-detail | { $count } items of the app's settings, data, and license. Unless this option is on, they stay. What they replaced comes back, or goes beside them if they have changed since. | OK (plural) |
| app-column-name | Name | OK |
| app-column-publisher | Publisher | OK |
| app-column-version | Version | OK |
| app-column-scope | Scope | OK |
| app-column-arch | Architecture | Fixed: 아키텍처 is 48 px in 12 px Malgun Gothic, and the cell is 46 px (58 less the gutter), plus a 10 px sort arrow. Now 플랫폼 (36 px), which Visual Studio uses for x86/x64/ARM64 and the Windows 10 Task Manager used for this column. `app-column-arch-sort` now says 플랫폼으로 정렬 to match |
| app-build-create | Create installer | OK |
| app-build-cancel | Cancel | OK |
| app-scan-look-again | Scan again | OK |
| app-scan-stop | Stop | OK |
| app-result-copy-error | Copy error | OK |
| app-result-open-folder | Open folder | OK |
| app-option-license | License and activation data | OK |
| app-option-user-data | User data | OK |
| app-option-user-data-hint | Settings and history, Documents, Saved Games, ProgramData. | OK |
| app-option-whose-data | Data owner | OK |
| app-list-search | Search by name or publisher  (Ctrl+F) | OK |
| app-room-drive-short | To build this installer, { $drive } needs up to { $needed } of free space, but has only { $free }. | OK |
| app-scan-failed | This app cannot be scanned. { $error } | Fixed (delta): colon before the reason |
| app-footprint-licenses-still-go | License data in what you excluded follows the "License" option, so it still goes in: { $list }. | Fixed (delta): the option's full name |
| core-review-service-detail | Runs { $file } as the { $account } account, and its start type is { $start } | Fixed (delta): `[other]` now takes the new clause-shaped start words |
| core-review-driver-detail-starts | Loads { $file } into Windows itself, and its start type is { $start } | Fixed (delta), as above |
| core-review-firewall-detail | For { $program }, on { $networks } networks, allows incoming connections on ports { $ports } | OK (read with 도메인, 개인 filled in) |
| core-review-com-support-detail | N, N, and N settings respectively, used with the classes | Fixed (delta): names each count's kind |
| core-space-shortfall | For { $purposes }, { $drive } must have up to { $needed } free, but has only { $free } | OK |
| core-policy-fit-program | { $program } was built for { $built_for }, and this PC is { $here }, so Windows does not start it | OK |
| core-policy-fit-needs-newer | { $program } says it needs Windows { $needs } or later, and this PC's Windows version is { $runs }, so Windows does not start it | OK |
| core-warning-review-left-out | The { $item } file / folder / service … was excluded: { $reason }. | OK (the reasons are clauses with no full stop of their own) |
| core-review-account-signed-in | User data and per-user settings go to { $account }, signed in here, not to the administrator running this install. | OK |
| core-verify-size-mismatch | The file size does not match. Expected { $expected } bytes, but it is { $actual } bytes. | OK |
| core-rollback-file-stays | Because something that stays still uses it / it changed after install / no removal was asked, { $path } stays. | OK |
| core-restore-root-under-link | This app has items to restore to { $root }, but { $link } is a link to another place that others can change, so nothing was installed… | OK |
| core-near-starts-with-publisher | Its name starts with the publisher's name ({ $publisher }). | OK |
| core-secret-taken-from | { $option } included in how { $name } is run / in the shortcut { $name } / … | OK |
| core-warning-build-recorded-folder-left-out | A folder named by something recorded for this app was excluded, because it is a folder that cannot be read. | Fixed (delta): `[unreadable]` now gives the safety rule |
| core-scan-no-executable | Cannot find the executable of { $app_name }. A driver, runtime, or add-in may have no program of its own to package | Fixed: added the full stop the English has. It ends `app-scan-failed`, which adds none |

77 messages sampled: 68 OK and 9 fixed. Six of the fixes were delta items that would have been made anyway.

## Other fixes

- `ui-machine-translated`: removed the doubled "translation … machine translation" (shown on every screen in Korean).
- `app-column-arch`, `app-column-arch-sort`: 아키텍처 → 플랫폼 so the header fits its column (see the sample).
- `core-scan-no-executable`: final full stop, as in English.
- `terms.md`: added the firewall profiles (도메인, 개인, 공용) and the start type 사용 안 함. Also added the full option name
  "라이선스 및 정품 인증 데이터" 옵션, 플랫폼 for the Arch column, and the clause shape of the start words.

Whole-language checks: every column header is two or three Hangul characters (이름, 게시자, 버전, 범위, 플랫폼), and the
Name and Publisher columns are wide anyway. Buttons are short noun phrases (설치, 취소, 닫기, 다시 검사, 설치 관리자 만들기, 변경 내용 미리 보기). No English
is left that should be translated: what remains is product names, paths, registry and command names (App Paths, Windows
Installer, DISM, robocopy), and format names. Ctrl+F stays Ctrl+F, as Korean Windows writes it. The console alignment of
`installer-cli-usage-preview` and `installer-cli-usage-apply` was checked counting Hangul as two columns, and it lines up.

## Open questions

- The start words are plain clauses that match the English, not the Services start-type names (자동, 자동(지연된 시작),
  수동, 사용 안 함). A native speaker may prefer the Windows names, for example 시작 유형: 자동.
- `app-column-arch` 플랫폼: Windows 11 Task Manager calls this column 아키텍처, which is 2 px too wide for the 46 px cell.
  If a native speaker prefers 아키텍처, widen `ARCH_COLUMN` in `crates/winappmigrator-gui/src/views.rs` from 58 to about
  66 px.
- The profile names 도메인, 개인, 공용 are the ones Windows Defender Firewall with Advanced Security and Windows Security
  use, as I know them. They were not checked against a Korean Windows 11.
- The License option name is now in quotes in sentences, while the User Data option (사용자 데이터 옵션) is not, as the
  translator had it. A native speaker may want one style for both.
- English, for the maintainers: the comment above `core-rollback-firewall-removed` in `en-US/core.ftl` still says
  "The capital "The" is as the app has always said it.", but the message no longer has "The".

Checker: ko-KR: 0 errors, 0 warnings.
