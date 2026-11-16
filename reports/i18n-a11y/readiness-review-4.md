# Readiness re-review 4: the plan as the work turned out (WORK-I18N-A11Y)

2026-11-15, before the 0.7.0 tag. The plan was read again against what was built, and against what CI and the lab found
on the way. No requirement, acceptance criterion, or task order changed; these did:

- **Change lists.** Each task now lists every file it touched, taken from git since `c883b86` by owner rules kept beside
  the plan. Reports are evidence, not changes, so none is listed; a receipt's artifact is never one of its inputs.
- **Catalog files.** A language has nine files, not six: `core.ftl` was split by stage into `core.ftl`, `core-build.ftl`,
  `core-restore.ftl` and `core-review.ftl`, with `terms.md` beside them. Message names still start with their part.
- **Accessibility checks are test-only.** `checks.rs` is included by path in the app's and the installer's tests, so no
  shipped crate depends on egui_kittest, even optionally (cargo-about and cargo-deny counted it).
- **T-UNICODE** now also owns reading and making shortcuts whose names or paths are outside Windows' ANSI code page.
  CI's round trip of "WAM E2E Über 日本 Café" found that WScript.Shell reads such a shortcut as empty and cannot make one;
  `AC-UNICODE` already asks that build, install and uninstall succeed for such names, so the criterion is unchanged and
  the task's scope grew to meet it. The plan's risks say so.
- **AC-A11Y-AXE's red drill** breaks the code that gives each panel its bounds, builds the app and installer, and scans
  them on testpc1. The earlier "before and after a fix" logs are kept as what was found, not as a drill.
- **AC-A11Y-TARGET's red drill** shrinks the header's icon buttons; a narrower scroll bar is let through by WCAG 2.5.8's
  spacing exception, as it should be.

Checked: every criterion still has an owning task, every task's dependencies are tasks that come before it, and no path
is owned by two tasks that are active at once. Evidence made before these changes is kept as history and made again.

Result: ready for verification and release.
