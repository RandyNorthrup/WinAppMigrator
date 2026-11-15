# Readiness review: languages and accessibility (WORK-I18N-A11Y)

Reviewed 2026-11-15 against `docs/plans/i18n-a11y.md` (scope revision 1), `docs/PLAN.md`, `CONTRIBUTING.md`, and
`docs/gui.md` at commit `c883b86`, with the source scanned for what each requirement touches. This reviews the requirements,
not any code: nothing here says an implementation works.

## What was checked in the source first

- **Text surface.** Person-facing text lives in `winappmigrator-gui` (about 360 capitalized literals), `winappmigrator-installer`
  (about 310, including the uninstaller), `winappmigrator-ui` (26), `winappmigrator-client` (about 100), and core: 137
  `InvalidInput` error sites and about 480 `format!` sites across 30 files, many of them messages a person reads (refusals,
  the installer's list of what it sets up, room checks). Core's `Warning` already has a `code`, which the package requirement
  builds on.
- **Rendering.** egui/epaint 0.37 shapes text with `harfrust`, so Devanagari, Bengali, and Thai join correctly, and it breaks
  CJK lines between characters. It sets each shaped segment's direction from its script and has no bidirectional algorithm
  and no mirrored layout. That is why REQ-RTL is real work, not a setting.
- **Fonts.** `theme::install_fonts` loads Segoe UI and Consolas from the Windows font folder, and `add_fallback_fonts` already
  loads Korean, Japanese, or Chinese fonts by script for app names. Every font in the language table is part of a stock Windows
  10 and 11 install.
- **Accessibility today.** eframe is built with `accesskit`; custom-painted controls report themselves with `widget_info`; a
  test in `theme.rs` holds every text color to WCAG AA on every surface. The UIA tree of the real windows (read on the lab PCs
  during issue #2) showed names and roles, and one control below 24 by 24 points: the search box's Clear Search button, 20 by 20.
  egui exposes `accesskit_node_builder`, so live regions and a window's language can be set.
- **`unsafe` is denied** workspace-wide. Windows' language list, separators, contrast theme, text scale, and animation setting
  are all readable from the registry with `winreg`, which the workspace already uses.
- **Localized Windows.** Core reads `whoami` by SID and `sc.exe` states by number, which suggests care, but nothing has run on
  a non-English Windows.

## Findings and what was done

| # | Area | Finding | Resolution |
|---|---|---|---|
| 1 | Coverage | `AC-I18N-CATALOG` was listed under two requirements; each criterion belongs to one. | Added `AC-I18N-COMPLETE` for REQ-I18N-SHIP: exactly the 24 table languages ship, each fully translated. |
| 2 | Coverage | `T-TRANSLATE` and `T-WEB` named no files for the 23 catalogs or the translated pages. | Listed every catalog (`locales/<tag>/winappmigrator.ftl`) and every page and landing file (`site/i18n/...`) as exact paths. |
| 3 | Coverage | `T-LOCALIZED-WINDOWS` had no change path; its fixes cannot be known before it runs. | Its findings go in `docs/discovery.md`, which it always changes; code it fixes is added to the task when found. |
| 4 | Clarity | `AC-I18N-NOHARDCODE` said "not data" without saying how a test knows. | The test exempts only the data values it put on screen itself (paths, names, versions, digits). |
| 5 | Failure | Nothing said what happens when a catalog is corrupt or a saved language stops shipping. | `AC-I18N-FALLBACK` covers a catalog that fails to parse (English is used); `AC-I18N-NEGOTIATE` covers a saved choice that no longer ships. |
| 6 | Scope | The pseudo languages could leak into the Language menu. | `AC-I18N-NEGOTIATE` says they are never chosen from Windows' list; they are for tests only. |
| 7 | Clarity | "Numbers" in REQ-I18N-FORMAT had no example beyond sizes. | `AC-I18N-SIZE` adds a count with grouping (12,345 in English, 12.345 in Italian). |
| 8 | Conflict | WCAG 1.4.4 at 225% text against the app's fixed table columns, which shorten text with an ellipsis by design. | `AC-A11Y-TEXT-SIZE` allows shortening only where the full text is offered on hover, on focus, and to screen readers, which keeps the content available. |
| 9 | Conflict | CONTRIBUTING and docs/gui.md say not to drive live windows with scripted clicks; `AC-A11Y-AXE` runs on real windows. | No conflict: Axe.Windows reads the UI Automation tree and clicks nothing, like the existing Real windows job that "only looks". The lab runs for issue #2 were a reproduction the owner asked for. |
| 10 | Terminology | WCAG states target size in CSS pixels; egui works in points. | One egui point is one device-independent pixel, the same as a CSS pixel at 100%, so 24 points is the WCAG 2.2 2.5.8 minimum. |
| 11 | Scope | Digits in Arabic and Persian: CLDR's default for some regions is native digits. | Decided: digits stay 0-9 everywhere, as paths, versions, and sizes mix with text, and Windows' own default for these regions in its UI is the same. Recorded in the plan's decisions through `AC-I18N-SIZE`'s formats. |
| 12 | Scope | The uninstaller has no Language menu in the requirements. | Intended: it follows Windows' language, opened from Settings by whoever uninstalls. The installer's choice is for that run only, since the installer saves nothing of a person's. |
| 13 | Risk | An Italian Windows is needed for `AC-LOCALIZED-WINDOWS`; none is in the lab. | A Hyper-V VM on testpc2 from Microsoft's evaluation media, within the owner's standing permission to download and install what testing needs. Recorded as manual with a reason. |
| 14 | Risk | Every installer carries every catalog. | Catalogs embedded compressed, with a 2 MB budget on what they add to the stub, checked when the stub is built. |

## Dependency order

`T-I18N-CRATE` first. `T-UI-TEXT`, `T-CORE-TEXT`, `T-CLIENT-TEXT`, and `T-UNICODE` can then proceed; `T-APP-TEXT` and
`T-INSTALLER-TEXT` need the UI and core changes; `T-RTL` needs every screen's text moved first, so its mirroring is checked on
real strings; translation waits for all text to be extracted, so translators see the final catalog; the website waits for
translation; the release waits for everything. No cycles.

## Open after this review

- Exact core files for `T-CORE-TEXT` beyond the ten listed are expected to grow as messages are moved; each is added to the task
  when touched, which re-binds its evidence.
- Whether egui's `TextEdit` can be told a direction is unknown; paths and search stay left to right regardless, as the plan says.

Result: ready to start `T-I18N-CRATE`.
