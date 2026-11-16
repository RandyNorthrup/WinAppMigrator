# Independent review of the machine translations

Each of the 23 languages was translated by one machine pass, then reviewed by a second, independent one, which had not
translated it. The reviewer back-translated a sample of at least 60 messages into English without looking at the English
first (all of `ui.ftl` and `winappmigrator.ftl`, the installer's first screen, the app's buttons, column headers and
options, every plural message in `installer.ftl`, and messages with placeholders from each part of the core), compared
each with the English, fixed what was wrong, brought the language up to date with the English that changed while it was
translated, and checked it with `scripts/ftl-tool.py`. Each report lists every sampled message with its back-translation
and verdict, the other fixes, and what a native speaker should look at.

This is a judgement of language made by machine, for a project with no native speakers yet: it lowers the error rate,
and does not replace a person who reads the language.

| Language | Tag | Messages sampled | Fixed in the sample | Checker |
| --- | --- | --- | --- | --- |
| [Arabic](ar-SA.md) | `ar-SA` | 88 | 4 | ar-SA: 0 errors, 0 warnings |
| [Bengali](bn-BD.md) | `bn-BD` | 78 | 7 | bn-BD: 0 errors, 0 warnings |
| [German](de-DE.md) | `de-DE` | 72 | 7 | de-DE: 0 errors, 2 warnings |
| [Spanish](es-ES.md) | `es-ES` | 85 | 4 | es-ES: 0 errors, 0 warnings |
| [Persian](fa-IR.md) | `fa-IR` | 76 | 1 | fa-IR: 0 errors, 0 warnings |
| [French](fr-FR.md) | `fr-FR` | 77 | 2 | fr-FR: 0 errors, 2 warnings |
| [Hebrew](he-IL.md) | `he-IL` | 92 | 2 | he-IL: 0 errors, 0 warnings |
| [Hindi](hi-IN.md) | `hi-IN` | 111 | 2 | hi-IN: 0 errors, 0 warnings |
| [Indonesian](id-ID.md) | `id-ID` | 84 | 4 | id-ID: 0 errors, 0 warnings |
| [Italian](it-IT.md) | `it-IT` | 73 | 5 | it-IT: 0 errors, 2 warnings |
| [Japanese](ja-JP.md) | `ja-JP` | 84 | 4 | ja-JP: 0 errors, 0 warnings |
| [Korean](ko-KR.md) | `ko-KR` | 77 | 9 | ko-KR: 0 errors, 0 warnings |
| [Dutch](nl-NL.md) | `nl-NL` | 77 | 2 | nl-NL: 0 errors, 2 warnings |
| [Polish](pl-PL.md) | `pl-PL` | 86 | 3 | pl-PL: 0 errors, 0 warnings |
| [Portuguese (Brazil)](pt-BR.md) | `pt-BR` | 69 | 5 | pt-BR: 0 errors, 0 warnings |
| [Russian](ru-RU.md) | `ru-RU` | 80 | 13 | ru-RU: 0 errors, 0 warnings |
| [Thai](th-TH.md) | `th-TH` | 85 | 6 | th-TH: 0 errors, 0 warnings |
| [Turkish](tr-TR.md) | `tr-TR` | 81 | 1 | tr-TR: 0 errors, 0 warnings |
| [Ukrainian](uk-UA.md) | `uk-UA` | 78 | 7 | uk-UA: 0 errors, 0 warnings |
| [Urdu](ur-PK.md) | `ur-PK` | 86 | 4 | ur-PK: 0 errors, 0 warnings |
| [Vietnamese](vi-VN.md) | `vi-VN` | 69 | 4 | vi-VN: 0 errors, 0 warnings |
| [Chinese (Simplified)](zh-CN.md) | `zh-CN` | 73 | 4 | zh-CN: 0 errors, 0 warnings |
| [Chinese (Traditional)](zh-TW.md) | `zh-TW` | 77 | 8 | zh-TW: 0 errors, 0 warnings |

The Hindi reviewer reports that it back-translated only the last 14 of its 111 messages before reading their English;
the rest were compared with the English in view. Its fixes stand, and Hindi is the first to give a native speaker.

"Fixed in the sample" counts sampled messages whose verdict was a fix; each report also lists fixes found outside the
sample, and the messages brought up to date. The checker's warnings left are messages that read the same as the English
in that language, such as "COM in { $server }".

The briefs both passes worked from, and the translators' notes, are in [../translation](../translation).
