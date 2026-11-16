# Translator reports (catalog pass)

## zh-CN: 0 errors 0 warnings. Address 你.
- unclear EN: installer-license-detail ("license files and keys": registry keys vs product keys) -> chose registry keys
- unclear EN: core-warning-build-recorded-folder-left-out / core-warning-build-near-part-left-out [unreadable] "it may not be read" -> chose "cannot be read"
- kept [0] keys on $more/$port_count/$network_count selects

## zh-TW: 0/0. Address 您.
- unclear EN: app-scan-stopped (comment names "Look Again" button, text doesn't mention it)
- unclear EN: installer-license-detail keys; [unreadable] "may not be read"
- core-source-nsis: "signature" = recognizable pattern, not digital signature
- installer stub -> 安裝程式空殼

## id-ID: 0/0. Address Anda.
- unclear EN: [unreadable]; core-review-com-support-detail ({ $libraries } and { $interfaces } and { $app_ids } settings of each: which number goes with which)
- "Arch" column -> "Arsitektur" longer, may need wider column (check pseudo/translated layout)

## es-ES: 0/0. Address tu. Straight quotes as Windows.
- unclear EN: app-scan-stopped comment; com-support-detail
- EN style: client-builder-done/client-wrong-code/client-pair-failed start with lowercase "another PC" (sentence start)
- core-rollback-* capital "The" quirk
- terms: permisos de administrador; record of the install -> historial de la instalación

## ja-JP: 0/0. です/ます, no あなた.
- [unreadable]; app-scan-stopped; com-support-detail
- app-column-arch アーキテクチャ may be too wide

## pt-BR 0/0 (voce). Added [0] variants (pt treats 0 as singular).
- CODE: core-review-firewall-detail { $networks } arrives as English words (domain/private/public)
- CODE: core-review-service-detail [other], core-review-driver-detail-starts { $start } raw Windows value ("boot") in English
- unclear EN: core-policy-not-text; app-source-older-windows "This has" (drive or image?)
## vi-VN 0/0 (ban)
- EN wrong?: core-rollback-setting-other-account [machine-variable]/[machine-list] says PC-wide variable "belongs to the account that installed this"
## nl-NL 0/2 (je) - warnings legit (COM in {}, {} in {})
## ru-RU 0/0 (vy)
- added plural forms for number+bytes in core-verify-size-mismatch, core-system-hive-too-large, core-task-too-large, core-pe-too-large, core-runtime-download-cut-short (EN lacks plural)
- core-error-io placeholder sometimes a phrase ("the installer") not path
- core-policy-architecture-*/machine-type are mid-sentence fragments
## it-IT 0/2 (tu) - legit
- unclear EN: installer-no-programdata "so the install could not be undone"
## de-DE 0/2 (Sie) - legit
- Ctrl+F vs German Strg+F (glossary says keep)
- firewall-detail networks English
## fr-FR 0/2 (vous) legit ("version")
- firewall networks English; core-rollback-count-kept vs -left-behind unclear
## tr-TR 0/0 (siz). Suffixes on nouns after placeholders.
- EN: installer-cli-folder-has-files ends without full stop ("...it is this app's")
- firewall networks English (Turkish Windows: Etki alanı, Özel, Ortak); added [one] for ports
## uk-UA 0/0 (vy)
- EN naming: "License option" vs checkbox "License and Activation Data" (app-footprint-licenses-still-go, core-footprint-licenses-left-out, core-review-note-licenses)
- [one] "a path"/"a file" without count (core-warning-scan-drive-paths-not-followed, -network-paths-not-followed, core-warning-build-not-looked-through): uk added [1]
- installer-license-detail: took LICENSE keys (zh took registry keys) -> clarify EN + delta
## ko-KR 0/0 (-습니다/-하세요)
- Arch column too narrow (아키텍처); installer-no-programdata unclear
## th-TH 0/0. No sentence-final dots (MS Thai style); core-review-sentence-joiner = {" "}. Start words carry own verb.
- "License option" -> used full option name
## ar-SA 0/0. Commands singular. app-output-folder-example path left English.
## hi-IN 0/0 (aap). start lines -> "प्रारंभ होता है: { $start }"
- core-space-for-* fragments; (Default) maybe English in Hindi Windows
## pl-PL 0/0 (singular). core-scan-no-executable lowercase but app-scan-failed inserts after full stop
## bn-BD 0/0 (apni). [unreadable] took as NOT ALLOWED (wrong -> delta). app-footprint-program-files ambiguity

# General points raised by several translators (apply to every language)
- [unreadable] meant a SAFETY rule (path leaves the drive or goes through a link) -> English now says so.
- installer-license-detail "keys" = registry values holding license data.
- installer-no-programdata: nothing was installed.
- "License option" = the "License and Activation Data" option (app-option-license).
- Firewall profiles and service start types are now translated words (core-review-network-*, core-review-start-*).
- client [unknown] "Another PC" starts a sentence.
- app-column-* headers must be short (narrow columns): abbreviate if needed.
- core-review-com-support-detail now names each count's kind.
- English capital "The" mid-sentence in core-rollback-* was a bug, fixed.
- Ctrl+F may be written as Windows writes it in the language (Strg+F in German).
