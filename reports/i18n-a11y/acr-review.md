# Review of the Accessibility Conformance Report

AC-A11Y-ACR asks that [docs/ACCESSIBILITY.md](../../docs/ACCESSIBILITY.md) and the repository's issue forms be reviewed
against every WCAG 2.2 A and AA criterion and EN 301 549 chapter 11, so that each criterion has a status and its
evidence or reason, the report says it is self-assessed, and Accessibility Problem and Translation Problem issue forms
exist. This is that review, made on 2026-11-15 against the code at the commit that carries this file.

## What was checked

1. **Every criterion is there.** A script compared the report's WCAG table with the 55 success criteria at levels A and
   AA in WCAG 2.2 (4.1.1 Parsing, removed in 2.2, is noted under the table). None is missing and none is extra.
2. **Every clause of EN 301 549 chapter 11 is there.** Read against the clause list of v3.2.1: 11.1 to 11.4 (WCAG, as
   above), 11.5.1, 11.5.2.1 to 11.5.2.17, 11.6.1, 11.6.2, 11.7, and 11.8.1 to 11.8.5.
3. **Each "Supports" that says "tested" has a test that would fail.** Each was traced to the check below, and each of
   those checks has a red drill in [drills](drills) that breaks the code and shows the check failing.
4. **Each claim without a test was read against the code**, for what it says the windows do.
5. **The report says it is self-assessed**, and that no third party has evaluated it, in its header table.
6. **The issue forms exist**: [accessibility-problem.yml](../../.github/ISSUE_TEMPLATE/accessibility-problem.yml) and
   [translation-problem.yml](../../.github/ISSUE_TEMPLATE/translation-problem.yml).

## What the review changed

- **EN 301 549 clauses that were missing.** 11.5.1 Closed functionality, 11.5.2.1, 11.5.2.2 and 11.5.2.4 (which apply
  to platforms and assistive technology, not to this) were not listed, and 11.6 was listed as one row, "Not applicable",
  though 11.6.2 No disruption of accessibility features applies to every program. Each now has its own row: 11.6.2
  Supports, the rest Not applicable with the reason.
- **11.8 and 12.2** now name the clauses they stand for (11.8.1 to 11.8.5; 12.2.2 to 12.2.4).
- **The issue forms' labels did not exist.** Both forms label what they open (`accessibility`, `translation`), and GitHub
  drops a label the repository does not have. Both labels were made on GitHub.
- **Axe.Windows in CI was not scanning.** [check-real-windows.ps1](../../.github/scripts/check-real-windows.ps1) passed
  `--showThirdPartyNotices false`; on testpc1 the same flag with a value made the CLI show its notices and scan nothing
  (exit 3, [axe-windows-lab-before-fix.log](axe-windows-lab-before-fix.log) shows the errors the first correct scan
  found). It now passes `--processid`, `--outputdirectory`, `--scanid` and `--alwayssavetestfile`, as the lab run that
  found 0 errors did ([axe-windows-lab.log](axe-windows-lab.log)).

## Where each tested claim is checked

| Criterion | Check |
| --- | --- |
| 1.3.2 Meaningful Sequence, 2.4.3 Focus Order, 2.1.1 Keyboard | `the_tab_key_reaches_every_control_in_reading_order` (app), and the installer's and uninstaller's sweeps; `a_right_to_left_window_is_the_mirror_image` |
| 1.4.3 Contrast (Minimum) | `theme::every_text_color_meets_wcag_aa_on_every_surface` |
| 1.4.11 Non-text Contrast, 2.4.7 Focus Visible | `theme::control_outlines_and_the_focus_ring_meet_non_text_contrast`; `what_has_the_focus_can_be_seen` |
| 1.4.4 Resize Text, 1.4.10 Reflow (the part claimed) | `larger_text_leaves_every_control_within_reach`, at 150% and 225% on 1024 by 768, in the app, the installer and the uninstaller |
| 2.2.2 Pause, Stop, Hide | `widgets::a_progress_bar_of_unknown_length_holds_still_when_nothing_may_move` |
| 2.5.8 Target Size (Minimum) | `nothing_clickable_is_too_small_to_hit`, with `checks::small_targets`, held to `theme::SMALLEST_TARGET` by `targets_are_checked_against_the_size_they_are_made_to` |
| 3.1.1 Language of Page | `language.rs` sets the window's language on its accessibility node; the picker's tests choose a language and read the words back |
| 4.1.2 Name, Role, Value; 1.3.1 Info and Relationships | `every_control_has_a_name_a_role_and_its_state`, in English and in Arabic; Axe.Windows on the real windows: 0 errors in the lab |
| 4.1.3 Status Messages | `progress_and_problems_are_said_as_they_change` |
| 11.7 User preferences (contrast themes) | `theme::a_windows_contrast_theme_is_drawn_in_its_own_colors`; `a_contrast_theme_is_kept_and_not_offered_away` |
| Every word from the catalog (supports 3.1.2 and 3.2.4) | `every_word_on_every_screen_is_from_the_catalog` |

## Claims read against the code, not tested

- 2.4.2 Page Titled: the window titles come from the catalog (`app-name`; the installer's and uninstaller's
  `ViewportCommand::Title`), so they are in the language in use.
- 2.5.2 Pointer Cancellation: egui acts on a button when the pointer is released over it.
- 2.2.1 Timing Adjustable: the only time limit is the client's 15-second wait for the network during pairing.
- 3.3.8 Accessible Authentication: Partially supports is right; the pairing code cannot be pasted between two PCs, and a
  drive or disk image is the way without it.
- 11.6.2 No disruption of accessibility features: the code changes no Windows accessibility setting.

## What stays open

- 1.4.10 Reflow and 1.4.13 Content on Hover or Focus are Partially supports, and say why.
- 11.5.2.6 Row, column, and headers is Partially supports: the list of apps is a list, not a table.
- No one who uses a screen reader, magnification or a contrast theme every day has used it, and no Narrator, NVDA or
  JAWS session has been recorded. The report says so; the Accessibility Problem form is how those reports come in.
