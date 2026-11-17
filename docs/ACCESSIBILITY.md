# Accessibility Conformance Report

**WinAppMigrator 0.7.2**: the app (`WinAppMigrator-v0.7.2.exe`), the installers it builds (with their uninstaller), and
WinAppMigrator Client. The website, <https://winappmigrator.com/>, has its own section at the end.

| | |
| --- | --- |
| Standards | [WCAG 2.2](https://www.w3.org/TR/WCAG22/) levels A and AA, applied to software as [WCAG2ICT](https://www.w3.org/TR/wcag2ict-22/) describes; [EN 301 549 v3.2.1](https://www.etsi.org/deliver/etsi_en/301500_301599/301549/03.02.01_60/en_301549v030201p.pdf) chapters 11 (software) and 12 (documentation and support) |
| Date | November 2026 |
| How it was evaluated | Self-assessed by the project, with the automated checks below and a review of each criterion. **No third party has evaluated or certified it.** |
| Contact | [Report an accessibility problem](https://github.com/RandyNorthrup/WinAppMigrator/issues/new?template=accessibility-problem.yml) |

**Terms.** *Supports*: the product meets the criterion. *Partially supports*: some of it does not. *Does not support*: most
of it does not. *Not applicable*: the criterion has nothing to apply to here.

## How it was checked

- **The accessibility tree, on every screen.** UI tests draw each screen of the app, the installer and the uninstaller
  (each step of a build, an install and an uninstall, done and failed) and walk the tree a screen reader is given
  (`crates/winappmigrator-ui/src/checks.rs`, `ui_tests_a11y.rs`, and the installer's and uninstaller's tests):
  - every control has a name, a role, and its state, in English and in Arabic;
  - every target is at least 24 by 24 points, or has that much room around it;
  - the Tab key reaches every button;
  - progress is a polite live region and a problem an assertive one;
  - in a test language that marks every word from the catalog, no word on any screen comes from anywhere else;
  - in a right-to-left test language, the windows are mirrored.
- **Axe.Windows** (Microsoft's rules for UI Automation) scans the real app, installer and uninstaller windows on
  Windows Server 2022, Windows Server 2025 and Windows 11 on Arm in CI, and fails the build on any error.
- **Colors.** A test holds every text color to 4.5:1 on every surface it is drawn on, and control outlines, filled
  controls and the focus outline to 3:1, in the light and dark themes, and checks that each of Windows 11's four contrast
  themes is drawn in its own system colors.
- **Windows settings.** Tests open each window with Text size at 150% and 225% on a 1024 by 768 screen, in a contrast
  theme, and with Animation effects off.
- **Not yet done:** no one who relies on a screen reader, magnification, or a contrast theme day to day has used it, and
  no session with Narrator, NVDA or JAWS has been recorded. Reports from people who do are what will find what the checks
  above cannot.

## WCAG 2.2, as WCAG2ICT applies it to software

| Criterion | Level | Conformance | Remarks |
| --- | --- | --- | --- |
| 1.1.1 Non-text Content | A | Supports | Every icon button has a name, which is also its tooltip. The logo is decorative. Pictures in the guide have text alternatives. |
| 1.2.1 Audio-only and Video-only (Prerecorded) | A | Not applicable | There is no audio or video. |
| 1.2.2 Captions (Prerecorded) | A | Not applicable | |
| 1.2.3 Audio Description or Media Alternative (Prerecorded) | A | Not applicable | |
| 1.2.4 Captions (Live) | AA | Not applicable | |
| 1.2.5 Audio Description (Prerecorded) | AA | Not applicable | |
| 1.3.1 Info and Relationships | A | Supports | Controls have their roles (button, checkbox, radio button, combo box, text field, scroll bar); section headings have the heading role; a checkbox's title and detail are its name. |
| 1.3.2 Meaningful Sequence | A | Supports | The tree is in reading order, also in right-to-left languages, where the layout is mirrored but the order of the tree is not. |
| 1.3.3 Sensory Characteristics | A | Supports | Instructions name controls by their labels. |
| 1.3.4 Orientation | AA | Supports | The windows work at any shape the screen has. |
| 1.3.5 Identify Input Purpose | AA | Not applicable | No field asks for information about the user. |
| 1.4.1 Use of Color | A | Supports | Status is said in words and shown with an icon as well as a color. |
| 1.4.2 Audio Control | A | Not applicable | There is no sound. |
| 1.4.3 Contrast (Minimum) | AA | Supports | Tested: every text color on every surface is at least 4.5:1. |
| 1.4.4 Resize Text | AA | Supports | Windows' Text size, up to 225%, enlarges everything in the windows by that factor. |
| 1.4.5 Images of Text | AA | Supports | No text is drawn as a picture. |
| 1.4.10 Reflow | AA | Partially supports | At 225% on a 1024 by 768 screen, each window's main action can be reached by scrolling up and down (tested). That nothing needs scrolling sideways at that size has not been checked on every screen. |
| 1.4.11 Non-text Contrast | AA | Supports | Tested: control outlines, filled controls and the focus outline are at least 3:1 against what is around them. |
| 1.4.12 Text Spacing | AA | Not applicable | WCAG2ICT: the windows offer no way for a user to change text spacing, and Windows has no setting that would. |
| 1.4.13 Content on Hover or Focus | AA | Partially supports | Tooltips stay while the pointer is over the control or it has the keyboard's focus, and Escape takes the focus, and the tooltip, away. A tooltip can close when the pointer moves onto it. What a tooltip says is also the control's accessible name, or is said on the screen. |
| 2.1.1 Keyboard | A | Supports | Tested: Tab reaches every control; Enter and Space work buttons and checkboxes; arrows move through the list of apps; Ctrl+F goes to the search box. Folder choosers are Windows' own. |
| 2.1.2 No Keyboard Trap | A | Supports | Tab goes round each window. |
| 2.1.4 Character Key Shortcuts | A | Supports | The only shortcut, Ctrl+F, uses a modifier key. |
| 2.2.1 Timing Adjustable | A | Supports | Nothing a person does is timed. The client ends a pairing that takes more than 15 seconds, which is the network's time, not the person's: typing the code has no limit. |
| 2.2.2 Pause, Stop, Hide | A | Supports | The only moving thing is the bar of a step whose length is unknown, which is still with Animation effects off. |
| 2.3.1 Three Flashes or Below Threshold | A | Supports | Nothing flashes. |
| 2.4.1 Bypass Blocks | A | Not applicable | WCAG2ICT: applies to sets of software, not to a single window. |
| 2.4.2 Page Titled | A | Supports | Each window's title says what it is, in the language in use: "WinAppMigrator", "Install" and the app's name, "Uninstall". |
| 2.4.3 Focus Order | A | Supports | The focus moves in reading order. |
| 2.4.4 Link Purpose (In Context) | A | Supports | |
| 2.4.5 Multiple Ways | AA | Not applicable | WCAG2ICT: applies to sets of software. |
| 2.4.6 Headings and Labels | AA | Supports | |
| 2.4.7 Focus Visible | AA | Supports | The control with the focus has a 2-point outline in the accent color, at least 3:1 against what is around it. |
| 2.4.11 Focus Not Obscured (Minimum) | AA | Supports | The panels at the top and bottom of a window do not cover what scrolls between them; a scroll area brings the focused control into view. |
| 2.5.1 Pointer Gestures | A | Supports | Everything works with a single click. |
| 2.5.2 Pointer Cancellation | A | Supports | Controls act when the button is released over them. |
| 2.5.3 Label in Name | A | Supports | Each control's name starts with its visible label. |
| 2.5.4 Motion Actuation | A | Not applicable | |
| 2.5.7 Dragging Movements | AA | Supports | Dragging a scroll bar can be replaced by the wheel, the keyboard, or a click on the bar. |
| 2.5.8 Target Size (Minimum) | AA | Supports | Tested on every screen. Scroll bars are 24 points wide where they are grabbed. |
| 3.1.1 Language of Page | A | Supports | Each window tells screen readers the language it is in. |
| 3.1.2 Language of Parts | AA | Supports | The names of apps, paths and versions are shown as they are. The Language menu lists each language by its own name. |
| 3.2.1 On Focus | A | Supports | |
| 3.2.2 On Input | A | Supports | Choosing a language changes the words of the window at once, which is what the menu is for, and nothing else. |
| 3.2.3 Consistent Navigation | AA | Not applicable | WCAG2ICT: applies to sets of software. |
| 3.2.4 Consistent Identification | AA | Supports | The same controls have the same names in the app, the installer and the uninstaller. |
| 3.2.6 Consistent Help | A | Not applicable | WCAG2ICT: applies to sets of software. |
| 3.3.1 Error Identification | A | Supports | A problem is said in words, next to an error icon, and announced. |
| 3.3.2 Labels or Instructions | A | Supports | |
| 3.3.3 Error Suggestion | AA | Supports | Messages say what to do, such as which folder to choose instead. |
| 3.3.4 Error Prevention (Legal, Financial, Data) | AA | Supports | Before installing, the installer lists everything it will change; Preview Changes shows it without changing anything; every install can be undone. |
| 3.3.7 Redundant Entry | A | Supports | Nothing has to be typed twice. |
| 3.3.8 Accessible Authentication (Minimum) | AA | Partially supports | Pairing with WinAppMigrator Client means typing a six-digit code shown on the other PC's screen, which cannot be pasted. A drive or a disk image can be read without pairing. |
| 4.1.2 Name, Role, Value | A | Supports | Through UI Automation, by AccessKit. Tested on every screen. |
| 4.1.3 Status Messages | AA | Supports | Progress is a polite live region; problems an assertive one. |

4.1.1 Parsing is no longer part of WCAG 2.2.

**WinAppMigrator Client** is a console program. Its text is in the console, which screen readers read, in Windows'
display language, except right-to-left languages, which a Windows 7 console cannot show in order, where it is in English
and says so. It asks for nothing but to be closed.

## EN 301 549, chapters 11 and 12

| Clause | Conformance | Remarks |
| --- | --- | --- |
| 11.1 to 11.4 | As WCAG above | |
| 11.5.1 Closed functionality | Not applicable | Screen readers and other assistive technology can be used with every window. |
| 11.5.2.1 Platform accessibility service support for software that provides a user interface | Not applicable | Applies to platform software, such as Windows. |
| 11.5.2.2 Platform accessibility service support for assistive technologies | Not applicable | Applies to platform software. |
| 11.5.2.3 Use of accessibility services | Supports | UI Automation, through AccessKit. |
| 11.5.2.4 Assistive technology | Not applicable | It is not assistive technology. |
| 11.5.2.5 Object information | Supports | Name, role, state, and bounds of each control. |
| 11.5.2.6 Row, column, and headers | Partially supports | The list of apps is exposed as a list of rows, each named by its app, publisher and version; its columns are not exposed as a table. |
| 11.5.2.7 Values | Supports | |
| 11.5.2.8 Label relationships | Supports | A field's label is part of its name. |
| 11.5.2.9 Parent-child relationships | Supports | |
| 11.5.2.10 Text | Supports | |
| 11.5.2.11 List of available actions | Supports | Click and focus. |
| 11.5.2.12 Execution of available actions | Supports | |
| 11.5.2.13 Tracking of focus and selection attributes | Supports | |
| 11.5.2.14 Modification of focus and selection attributes | Supports | |
| 11.5.2.15 Change notification | Supports | |
| 11.5.2.16 Modifications of states and properties | Supports | |
| 11.5.2.17 Modifications of values and text | Supports | |
| 11.6.1 User control of accessibility features | Not applicable | Applies to platform software. |
| 11.6.2 No disruption of accessibility features | Supports | It changes none of Windows' accessibility settings, and takes the keyboard and pointer through Windows' ordinary messages, after Sticky Keys and Filter Keys have done their work. Contrast themes, Text size and Animation effects are followed (tested). |
| 11.7 User preferences | Partially supports | Contrast themes, Text size, Animation effects, and light or dark mode are followed. The windows use Segoe UI and the fonts Windows has for each script, not a font chosen by the user. |
| 11.8.1 to 11.8.5 Authoring tools | Not applicable | An installer it builds has the same accessible windows as everything else here; nothing else it produces is content for people. |
| 12.1.1 Accessibility and compatibility features | Supports | [Languages and accessibility](https://github.com/RandyNorthrup/WinAppMigrator/wiki/Languages-and-Accessibility) |
| 12.1.2 Accessible documentation | Supports | The website is held to WCAG 2.2 AA (below). |
| 12.2.2 to 12.2.4 Support services | Supports | Issue forms for accessibility and for translation problems, which can be filled in with the keyboard and a screen reader on GitHub. |

## The website

Every page of the website is held to WCAG 2.2 AA. On every build, [axe-core](https://github.com/dequelabs/axe-core) checks
each page, in each language, and the build fails on any violation. Each page has a link that skips to its content,
states its language and direction, keeps visible focus outlines, and has text alternatives for its pictures; code blocks
that scroll can be reached with the keyboard. What axe-core cannot judge (whether a page reads well, and what a
screen reader user makes of it) has not been checked by a person who relies on one.
