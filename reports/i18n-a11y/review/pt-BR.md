# Review: Portuguese, Brazil (pt-BR)

Reviewer: independent machine pass, 2026-11-15. Translator's form of address: "você", with commands in the third-person
imperative ("Feche", "Digite", "Escolha") and buttons in the infinitive ("Instalar", "Cancelar"); kept, because that is
how Windows 10 and 11 in Brazilian Portuguese address the user. Sentence case and “ ” quotes, as Windows pt-BR does.

## Brought up to date

30 messages checked against the current English. 22 needed new or changed words:

- `app-scan-failed`: the reason now follows a colon ("Não foi possível analisar este aplicativo: …"), not a full stop.
- `app-footprint-licenses-still-go`, `core-footprint-licenses-left-out`, `core-review-note-licenses`,
  `core-secret-why-license-option-off`, `core-warning-scan-license-data`: "a opção Licença" is now "a opção Dados de
  licença e ativação", the words of the checkbox `app-option-license`. In `core-warning-scan-license-data`, "Com a opção
  Dados de licença e ativação, …" (as `app-source-older-windows` says "Com a opção Dados do usuário"), which avoids
  "ativação ativada".
- `core-warning-build-recorded-folder-left-out`, `core-warning-build-near-part-left-out` `[unreadable]`: was "ela não
  pode ser lida" (a read error); now the safety rule: "o caminho dela sai da unidade ou passa por um link, então não é
  seguro lê-la".
- `core-review-start-boot`, `-system`, `-automatic`, `-delayed`, `-manual`, `-disabled`, `-unknown` (new): "com o
  Windows, antes de qualquer outra coisa", "com o Windows, durante a inicialização dele", "sempre que este PC é
  iniciado", "logo após cada inicialização deste PC", "só quando algo o solicita", "nunca, pois está desativado", "de um
  modo que o Windows não informa". They fill "…, início: { $start }" in `core-review-service-detail` `[other]` and
  `core-review-driver-detail-starts`, which reads correctly with every one of them, including "nunca" (a "…, inicia
  { $start }" frame would not: "inicia nunca" is not Portuguese). The phrases match the wording of the service's own
  `[automatic]`, `[delayed]`, and `[manual]` lines.
- `core-review-network-domain`, `-private`, `-public` (new): "Domínio", "Privado", "Público", the profile names of
  Windows Defender Firewall in pt-BR. `core-review-firewall-detail` now has a `[one]` line, "no perfil de rede
  { $networks }", beside "nos perfis de rede { $networks }", so a single profile is not plural.
- `core-review-com-support-detail`: was "{ $libraries }, { $interfaces } e { $app_ids } configurações,
  respectivamente, …"; now each count follows its kind: "Configurações das bibliotecas de tipos ({ $libraries }), das
  interfaces ({ $interfaces }) e dos AppIDs ({ $app_ids }) que acompanham as classes dele".
- `installer-license-detail`: "chaves" (which could read as product keys) is now "os valores do Registro que guardam
  licenças".
- `installer-no-programdata`, `installer-cli-no-programdata`: was "so the install could not be undone"; now "então nada
  foi instalado: é nela que uma instalação guarda o histórico usado para desfazê-la".

Already current, no change needed (8): `client-builder-done`, `client-wrong-code`, `client-pair-failed` (the
translation already began the sentence with "Outro PC"); `core-rollback-setting-put-back`,
`core-rollback-list-entries-removed`, `core-rollback-list-entries-not-removed`, `core-rollback-firewall-removed` (the
translation never copied the capital "The" quirk); `ui-disclosure` (already present).

The files were then run through `ftl-tool.py merge`, which only refreshed the English `#` comments to the current
English and put the three network words in English's position.

## Sample (back-translation)

72 messages: all of `ui.ftl` (8) and `winappmigrator.ftl` (6), the first 25 of `installer.ftl`, its 3 plural messages,
17 from `app.ftl`, and 13 from `core*.ftl` with placeholders. 5 fixed, 67 OK. Plural messages carry a `[0]` line (CLDR puts 0 in pt's
`one`, but Brazilian Portuguese says "0 itens"); exact-number keys match before plural categories, so that is correct.

| Message | Back-translation | Verdict |
| --- | --- | --- |
| ui-clear-search | Clear search | OK |
| ui-browse | Browse | OK (Windows' "Procurar") |
| ui-browse-tooltip | Choose a folder for { $field } | OK |
| ui-disclosure | { $title }, { $summary }, expanded / collapsed | OK (Narrator's own pt-BR state words) |
| ui-language | Language | OK |
| ui-machine-translated | This translation was made automatically and may contain errors. | OK |
| ui-report-translation-problem | Report a translation problem | OK |
| ui-needs-direct3d-12 | Direct3D 12 is needed to draw this window, and an old graphics driver or a virtual machine may not offer it. | OK |
| size-bytes | { $value } byte / { $value } bytes (0 bytes) | OK |
| size-kb, size-mb, size-gb, size-tb | { $value } KB / MB / GB / TB | OK (4 messages) |
| list-separator | ", " | OK |
| installer-window-title | WinAppMigrator's installer | OK |
| installer-window-title-app | Install { $app_name } | OK |
| installer-tagline | Reinstalls an app captured on another PC | OK |
| installer-unnamed-app | App | OK |
| installer-window-failed | It was not possible to open the installer's window: { $error } | OK |
| installer-window-failed-without-one | It is still possible to install without it: run this file in an administrator command prompt with --apply and, to choose where, with --target-root followed by a folder. | OK |
| installer-checking | Checking the installer… | OK |
| installer-stub-no-package | This is a WinAppMigrator installer stub and holds no app package. Create an installer with WinAppMigrator and run the generated .exe here. | OK |
| installer-check-stopped | The check of the installer was interrupted. | OK |
| installer-locate-failed | It was not possible to locate the installer: { $error } | OK |
| installer-read-failed | It was not possible to read the installer: { $error } | OK |
| installer-cannot-be-used-here | This installer's package cannot be used on this PC: { $error } | OK |
| installer-captured-from | Captured from | OK |
| installer-install-location | Install location | OK |
| installer-choose-location | Choose the install location | OK (dialog title, infinitive as Windows titles are) |
| installer-folder-has-files | This folder already has files. Files with the same names are replaced and come back if you uninstall. | OK |
| installer-room-needed | Space needed | OK |
| installer-room-short | Maybe there is not enough space here: { $shortfall }. The package can be unpacked on another drive, but the app's files still have to fit. | OK |
| installer-interrupted | An earlier install of { $app_name } was not finished. When installing, it is undone first. | OK |
| installer-signature | Installer | OK |
| installer-signature-checking | Checking the signature… | OK |
| installer-signature-unsigned | Not signed: unknown publisher | OK |
| installer-signature-signed-by | Signed by { $publisher } | OK |
| installer-signature-unnamed | Signed by a publisher without a name | OK |
| installer-signature-invalid | This installer's signature is not valid ({ $reason }). It may have been changed after being signed. | OK |
| installer-more-left-out | 1 more item was left out / { $count } more items were left out | OK |
| uninstaller-plan-coming-back | { $count } file(s) this install replaced come back or are put beside a file that has been changed since. | OK |
| uninstaller-remove-user-data-detail | { $count } item(s) of the app's settings, data, and license. It stays unless this option is on. What it replaced comes back, or is put beside it if it has been changed since. | OK |
| app-build-create | Create installer | OK |
| app-build-cancel | Cancel | OK |
| app-scan-look-again | Analyze again | OK (terms.md: Look Again) |
| app-scan-try-again | Try again | OK |
| app-result-copy-path | Copy path | OK |
| app-network-connect | Connect | OK |
| app-column-name | Name | OK |
| app-column-publisher | Publisher | OK (Windows' "Editor") |
| app-column-version | Version | OK |
| app-column-scope | Scope | OK |
| app-column-arch | Arch. | OK (abbreviated, fits the narrow column) |
| app-option-license | License and activation data | OK |
| app-option-user-data | User data | OK |
| app-option-whose-data | Data of whom | OK (terms.md; label above a list of accounts) |
| app-source | Source of the scan | OK |
| app-list-search | Search by name or publisher  (Ctrl+F) | OK (pt-BR Windows also writes Ctrl) |
| app-scan-failed | It was not possible to analyze this app: { $error } | Fixed: brought up to date (colon) |
| core-error-io | I/O failure in { $path }: { $error } | Fixed: { $path } can be "o instalador", and "em o instalador" is wrong Portuguese (must contract to "no"); now "falha de E/S ({ $path }): { $error }" |
| core-space-shortfall | { $drive } needs up to { $needed } free for { $purposes } and has { $free } | Fixed: the purposes mixed verbs and nouns, so "para descompactar o pacote e o histórico da instalação" read as "to unpack the package and the record"; `core-space-for-unpacking`, `-what-is-replaced`, `-unpacking-and-installing` are now noun phrases ("a descompactação do pacote", "o backup do que a instalação substitui", "a descompactação e a instalação do pacote") |
| core-review-service-detail | Runs { $file } as { $account }, starts now and each time this PC starts / … / is disabled / start: { $start } | Fixed: `[disabled]` "desabilitado" is now "desativado", the word Services uses for that startup type, matching the new start word |
| core-review-driver-detail-starts | Loads { $file } into Windows itself, start: with Windows, before anything else | OK (with the new words) |
| core-review-firewall-detail | Allows incoming connections on ports { $ports } for { $program }, in the network profiles { $networks } | Fixed: added `[one]` "no perfil de rede { $networks }" so one profile does not read as plural |
| core-policy-fit-needs-newer | { $program } says it needs Windows { $needs } or newer, and this PC runs version { $runs }, so Windows would not start it | OK |
| core-rollback-file-stays-beside | { $path } stays, because something that stays still uses it. The file it replaced is beside it, as { $name }. | OK |
| core-warning-review-documents-elsewhere | { $count } file(s) from Documents go instead to { $target }, in a folder named _external, because this PC's Documents folder is at { $folder }, and files are only written in folders inside your profile. To use them, move them yourself to your Documents folder. | OK |
| core-restore-root-under-link | this app has items to restore in { $root }, and { $link } is a link to somewhere else that others may be able to change, so nothing was installed. If what goes there is user data or license data, install it without its user and license data. If it is a shortcut in a Startup folder, return that folder to where it was | OK |
| core-warning-build-links-not-followed | { $count } link(s), or name(s) that would open something else, were not followed in this folder, so what they point at is not in the installer: { $names }, and { $more } more. | OK |
| core-runtime-download-cut-short | the download of { $url } was interrupted: { $came } of { $expected } bytes arrived | OK |
| core-warning-scan-package-mentions | This package mentions an account's name in { $places }. It was created without any account's data, and anyone who has the installer can read this. | OK |
| core-image-differencing | { $path } is a differencing disk, which only holds what changed since the disk it was created from, and Windows would open that disk too, wherever it says it is. Merge it into that disk first, with Hyper-V's Edit Disk, and choose that disk | OK |

Whole-language checks: the `app-column-*` headers are short (Nome, Editor, Versão, Escopo, Arq.); button labels are
short infinitives; nothing is left in English that should not be (the checker reports no "still English" warnings);
Windows words match `terms.md` (Configurações, Aplicativos instalados, Explorador de Arquivos, Serviços, pasta
Inicializar, Registro, (Padrão), Jogos Salvos).

## Other fixes

- `core-error-io`: "falha de E/S em { $path }" became "falha de E/S ({ $path })", because the path is sometimes the
  words "o instalador" or "pasta temporária do pacote", and "em o instalador" is ungrammatical.
- `core-space-for-unpacking`, `core-space-for-what-is-replaced`, `core-space-for-unpacking-and-installing`: now noun
  phrases, so that joined with " e " after "para" in `core-space-shortfall` (shown in the installer's Room Needed line)
  they cannot be misread.
- `core-review-service-detail` `[disabled]`: "está desabilitado" became "está desativado", Windows' word in Services.
- `core-review-firewall-detail`: `[one]` added for `$network_count`.
- `terms.md`: added the firewall profiles (Domínio / Privado / Público) and the startup type "Desativado", and noted
  that the license option is cited in running text as "a opção Dados de licença e ativação".
- All files: `#` comments refreshed to the current English by `ftl-tool.py merge` (no translated text changed by it).

## Open questions

- Firewall profile names: "Domínio", "Privado", "Público" are taken as the Profile column of Windows Defender Firewall
  with Advanced Security in pt-BR Windows 10 and 11, and capitalized as names ("no perfil de rede Privado"). Could not
  be confirmed against a pt-BR Windows here; older Windows (7) said "particular" in some places.
- `ui-disclosure`: "expandido" / "recolhido" are the words Narrator uses in pt-BR for an expandable item, rather than a
  literal "aberto" / "fechado". A native screen-reader user may prefer one or the other.
- The start words read as "…, início: com o Windows, antes de qualquer outra coisa". Natural enough in a list line, but
  a native speaker may find a better frame that still works for "nunca, pois está desativado".
- `core-source-nsis`: "Assinatura do desinstalador NSIS" means a recognizable pattern in the file, not a digital
  signature; Portuguese uses "assinatura" for both, which may confuse beside the installer's signature check.
- `app-footprint-program-files` is "Arquivos do programa" (the app's own program files, a heading beside "Pastas de
  dados"), not the Program Files folder; the English heading is ambiguous.
- Client and verify messages say "Firewall do Windows" (the Windows 7 name; Windows 10 and 11 say "Firewall do Windows
  Defender"). Kept, as the client runs on Windows 7 and later.

Checker: pt-BR: 0 errors, 0 warnings.
