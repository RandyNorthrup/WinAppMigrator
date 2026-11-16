# Words of the installer that every built installer carries, which runs on the PC an app is moved to and speaks its
# language, and of the uninstaller it leaves behind for Settings. Message names start with installer- for the
# installer's window, installer-cli- for its command line, and uninstaller- for the uninstaller.
#
# The command lines (installer-cli-, uninstaller-cli-) are always shown in English, as scripts read what they print.
# Translate them all the same. Keep their command-line flags, such as --apply and --target-root, exactly as they are.


## The installer's window: its title and header

installer-window-title = תוכנית ההתקנה של WinAppMigrator
# The window's title in the taskbar. { $app_name } is the name of the app it installs.
installer-window-title-app = התקנת { $app_name }
installer-tagline = התקנה מחדש של אפליקציה שנלכדה במחשב אחר
# What an app is called when its package gives it no name.
installer-unnamed-app = אפליקציה

## When the installer's window cannot open. Each is a paragraph of a message box.

# { $error } is what went wrong, as Windows or the graphics driver says it.
installer-window-failed = לא ניתן היה לפתוח את החלון של תוכנית ההתקנה: { $error }
installer-window-failed-without-one = עדיין ניתן להתקין בלי החלון: יש להפעיל קובץ זה משורת פקודה של מנהל מערכת עם --apply, ולבחירת המיקום, עם --target-root ואחריו תיקיה.

## Checking the installer when it opens

installer-checking = בדיקת תוכנית ההתקנה…
installer-stub-no-package = זהו שלד תוכנית התקנה של WinAppMigrator, והוא אינו מכיל חבילת אפליקציה. יש לבנות תוכנית התקנה באמצעות WinAppMigrator, ולאחר מכן להפעיל כאן את קובץ ה-.exe שנוצר.
installer-check-stopped = בדיקת תוכנית ההתקנה נעצרה.
# { $error } is what went wrong, as Windows says it.
installer-locate-failed = לא ניתן היה לאתר את תוכנית ההתקנה: { $error }
# { $error } is what went wrong reading it.
installer-read-failed = לא ניתן היה לקרוא את תוכנית ההתקנה: { $error }
# { $error } is why, such as that the app needs a newer Windows.
installer-cannot-be-used-here = לא ניתן להשתמש בחבילה של תוכנית התקנה זו במחשב זה: { $error }

## The app, where it goes, and whether the installer is signed

installer-captured-from = מקור הלכידה
installer-install-location = מיקום ההתקנה
# The title of the folder picker.
installer-choose-location = בחירת מיקום ההתקנה
installer-folder-has-files = תיקיה זו כבר מכילה קבצים. קבצים בעלי אותו שם יוחלפו, ויחזרו אם ההתקנה תוסר.
installer-room-needed = שטח נדרש
# { $shortfall } says which drive is short of room, such as "C: needs up to 824.0 MB for the app's files, and has 300.0 MB free".
installer-room-short = ייתכן שאין כאן מספיק שטח: { $shortfall }. החילוץ יכול להתבצע בכונן אחר, אך עדיין צריך להיות מקום לקבצים של האפליקציה.
# { $app_name } is the name of an app whose earlier install was cut short.
installer-interrupted = התקנה קודמת של { $app_name } לא הסתיימה. ההתקנה תבטל אותה תחילה.
# Beside "Installer", for whether it proves who made it.
installer-signature = תוכנית ההתקנה
installer-signature-checking = בדיקת החתימה…
installer-signature-unsigned = לא חתומה: מפרסם לא ידוע
# { $publisher } is the name on the installer's certificate, such as "Fabrikam, Inc.".
installer-signature-signed-by = חתומה על ידי { $publisher }
installer-signature-unnamed = חתומה על ידי מפרסם ללא שם
# { $reason } is Windows' own word for what is wrong, such as HashMismatch.
installer-signature-invalid = החתימה של תוכנית התקנה זו אינה חוקית ({ $reason }). ייתכן שהיא שונתה מאז שנחתמה.

## Why the install location cannot be used. Install and Preview Changes stay off while one of these is shown.

# { $app_name } is the name of the app being installed.
installer-location-app-already-there = האפליקציה { $app_name } כבר מותקנת בתיקיה זו, ולכן לא ניתן להתקין כאן עותק זה. כדי להתקין אותה כאן, יש להסיר תחילה את ההתקנה של העותק שבמחשב זה, בהגדרות של Windows תחת "אפליקציות", ולאחר מכן לפתוח שוב את תוכנית ההתקנה. לחלופין, יש להזין למעלה תיקיה חדשה וריקה.
# { $other_app } is the name of another app Windows lists as installed in the folder. { $app_name } is the name of the app being installed.
installer-location-other-app-there = האפליקציה { $other_app } מותקנת בתיקיה זו, ולכן לא ניתן להתקין כאן את { $app_name }. יש להזין למעלה תיקיה חדשה וריקה. לחלופין, אם אין עוד צורך ב-{ $other_app }, יש להסיר תחילה את ההתקנה שלה, בהגדרות של Windows תחת "אפליקציות", ולאחר מכן לפתוח שוב את תוכנית ההתקנה.
installer-location-has-files = תיקיה זו כבר מכילה קבצים שלא הותקנו על ידי WinAppMigrator. ייתכן שהם שייכים לאפליקציה אחרת, ולכן שום דבר לא יותקן מעליהם. יש להזין למעלה תיקיה חדשה וריקה.
# { $reason } is why, such as that it cannot be the Windows folder.
installer-location-refused = לא ניתן להשתמש בתיקיה זו: { $reason }. יש להזין למעלה תיקיה אחרת.

## What the install sets up beyond its files

installer-beyond-files = מה מוגדר מלבד קבצים
# Under the heading above. It is part of what a person agrees to when they click Install.
installer-beyond-files-agreed = כל זה מתבצע עם הרשאות מנהל מערכת. אם ההתקנה נכשלת, השינויים שביצעה מוחזרים, ותוצג הודעה על כל מה שלא ניתן היה להחזיר. ניתן גם לבטל את ההתקנה מאוחר יותר בהגדרות, תחת "אפליקציות מותקנות". קבצים שהיא מחליפה יחזרו, ורכיבי זמן ריצה שהיא מתקינה יישארו.
installer-beyond-files-none = שום דבר: ההתקנה רק מעתיקה קבצים, והגדרות במפתחות הרישום של האפליקציה עצמה.
# { $problem } is why, such as that the app cannot run on this PC.
installer-cannot-go-here = לא ניתן להתקין חבילה זו במיקום זה: { $problem }
installer-working-out = בירור מה יוגדר במיקום זה…
# Said as the reason after "This package cannot go to this location:".
installer-list-stopped = הבירור של מה שההתקנה תעשה נעצר
# A heading over a list of what else the package asked for and will not get. { $count } is how many.
installer-more-left-out =
    { $count ->
        [one] הושמט עוד פריט אחד
        [two] הושמטו עוד שני פריטים
       *[other] הושמטו עוד { $count } פריטים
    }

## Options

installer-options = אפשרויות
installer-user-data = נתוני משתמש
installer-user-data-detail = מופעל: הגדרות, מסמכים ומשחקים שמורים מועברים לתיקיות הרגילות של המשתמש המחובר. כבוי: כל אלה מועברים לתיקיית ההתקנה, ושום דבר לא מוגדר לפעול מתוכם.
# { $account } is the name of the account on the other PC whose data this is.
installer-user-data-whose = כאן נמצאים הנתונים של { $account }.
installer-license = רישיון
installer-license-detail = קבצי הפעלה ורישיון, וערכי הרישום שמכילים רישיונות. ייתכן שחלקם יחייבו הפעלה חוזרת כאן.

## Dependencies: what else the app needs, and how each is provided

installer-dependencies = תלויות
installer-dependencies-detail = מה האפליקציה צריכה, ואיך כל רכיב מסופק.
installer-dependencies-none = לא זוהו תלויות נוספות של זמן ריצה.
# { $feature } is the name of an optional part of Windows, such as NetFx3.
installer-dependency-windows-feature = תכונת Windows: { $feature }
installer-dependency-included = כלול
# { $id } is the package's name in winget, such as Microsoft.VCRedist.2015+.x64.
installer-dependency-winget = מותקן באמצעות winget: { $id }
installer-dependency-manual = התקנה ידנית
installer-dependency-built-in = מובנה ב-Windows
installer-dependency-ships-with-app = מגיע עם האפליקציה

## Buttons, and what is said beside them

installer-install = התקנה
installer-preview = תצוגה מקדימה של השינויים
installer-cancel = ביטול
installer-close = סגירה
installer-open-folder = פתיחת תיקיית ההתקנה
# { $error } is what went wrong, as Windows says it.
installer-open-folder-failed = לא ניתן היה לפתוח את התיקיה: { $error }
installer-location-blocks = הלחצנים "התקנה" ו"תצוגה מקדימה של השינויים" אינם זמינים עד שניתן יהיה להשתמש במיקום ההתקנה. הסיבה מופיעה למעלה, מתחת למיקום.
installer-package-blocks = הלחצן "התקנה" אינו זמין מכיוון שלא ניתן להתקין חבילה זו במיקום זה. הסיבה מופיעה למעלה.
installer-list-changed = מה שההתקנה תעשה כאן השתנה, ולכן שום דבר לא הותקן. יש לעבור על הרשימה שלמטה, וללחוץ שוב על "התקנה" אם זה מה שרצוי.
installer-cancelled-early = בוטל. שום דבר לא השתנה.

## How far an install has got, beside its progress bar

# { $step } is one of the steps below. { $done } and { $total } are how many of its items are done, and how many there are.
installer-step-counted = { $step }: { $done } מתוך { $total }…
# { $step } is one of the steps below, when it does not count its items.
installer-step-uncounted = { $step }…
installer-step-getting-ready = הכנה…
installer-step-cancelling = ביטול לאחר השלב הנוכחי…
installer-step-putting-back = החזרת השינויים שבוצעו
installer-step-unpacking = חילוץ
installer-step-checking-package = בדיקת החבילה
installer-step-recovering = ביטול התקנה קודמת שלא הסתיימה
# What a preview is doing, whatever step it is on.
installer-step-previewing = בירור מה ישתנה
installer-step-runtimes = התקנת רכיבי זמן ריצה
installer-step-files = העתקת קבצים
installer-step-services = הגדרת שירותים
installer-step-drivers = הגדרת מנהלי התקנים
installer-step-registry = כתיבת הגדרות רישום
installer-step-license = שחזור נתוני רישיון
installer-step-tasks = הגדרת משימות מתוזמנות
installer-step-shortcuts = יצירת קיצורי דרך
installer-step-environment = הגדרת משתני סביבה
installer-step-settings-entry = הוספת האפליקציה להגדרות
installer-step-verifying = בדיקת מה שהותקן
installer-step-undoing = ביטול

## When an install cannot go ahead or stops

installer-carries-no-package = תוכנית התקנה זו אינה מכילה חבילה.
installer-stopped-unexpectedly = ההתקנה נעצרה באופן לא צפוי.
# { $reason } is what went wrong, as the program says it.
installer-stopped-unexpectedly-because = ההתקנה נעצרה באופן לא צפוי: { $reason }. אם היא כבר התחילה, הפעלה חוזרת של תוכנית ההתקנה תבטל תחילה את מה שהיא עשתה.
# { $error } is what went wrong reading it.
installer-package-read-failed = לא ניתן היה לקרוא את החבילה: { $error }
installer-no-programdata = לא ניתן היה למצוא את התיקיה ProgramData של מחשב זה, ולכן לא הותקן דבר: כל התקנה שומרת שם את הרשומה שמשמשת לביטולה.
# { $error } is why, such as that there is nowhere only administrators can get into.
installer-staging-failed = לא ניתן היה ליצור תיקיית ביניים: { $error }
# { $error } is what went wrong unpacking it.
installer-unpack-failed = לא ניתן היה לחלץ את החבילה: { $error }
installer-staging-nowhere = אין מקום לחילוץ החבילה שרק מנהלי מערכת יכולים לגשת אליו
# { $failure } is a folder and what went wrong there, such as "C:\ProgramData\WinAppMigrator: Access is denied.".
installer-staging-nowhere-look-at = אין מקום לחילוץ החבילה שרק מנהלי מערכת יכולים לגשת אליו. המקום שיש לבדוק הוא { $failure }

## What came of an install, or of a preview

installer-result = תוצאת ההתקנה
installer-result-preview = תצוגה מקדימה: שום דבר עדיין לא השתנה
installer-result-installed = האפליקציה הותקנה ועברה את בדיקת הגיבוב.
installer-result-installed-without-left-out = האפליקציה הותקנה ועברה את בדיקת הגיבוב, ללא מה שמופיע למטה כמושמט.
installer-result-not-all-put-back = ההתקנה נכשלה, ולא ניתן היה להחזיר את כל השינויים שביצעה. מה שנשאר מפורט למטה, וההתקנה הבאה תנסה שוב.
installer-result-cancelled-put-back = ההתקנה בוטלה, והשינויים שביצעה הוחזרו.
installer-result-failed-put-back = ההתקנה נכשלה, והשינויים שביצעה הוחזרו.
installer-result-not-clean = ההתקנה לא הסתיימה באופן תקין. יש לעיין בבעיות שלמטה.
installer-result-changes-put-back = שינויים שהוחזרו
installer-result-changes-made = שינויים שבוצעו
installer-result-changes-planned = שינויים מתוכננים
installer-result-warnings = אזהרות
installer-result-errors = שגיאות
installer-result-package-valid = ערכי הגיבוב של החבילה תקינים
installer-result-package-invalid = ערכי הגיבוב של החבילה אינם תקינים
installer-result-files-valid = הקבצים המותקנים תקינים
installer-result-files-invalid = הקבצים המותקנים אינם תקינים
installer-result-reboot = אחת התלויות דורשת הפעלה מחדש כדי להשלים את התקנתה. יש להפעיל מחדש את Windows, ולאחר מכן לפתוח את האפליקציה.
# A heading over the list of what was changed. { $count } is how many.
installer-result-changes = שינויים ({ $count })
# After the first part of a long list. { $count } is how many more there are.
installer-result-changes-more = ועוד { $count }.
# Over the path of the install's log file.
installer-log = יומן

## The installer's command line. Shown in English: scripts read it. A { $count } is plain digits, with no separators.

installer-cli-stub = שלד תוכנית התקנה של WinAppMigrator — הוא אינו מכיל חבילת אפליקציה.
installer-cli-stub-how = יש לבנות תוכנית התקנה באמצעות האפליקציה WinAppMigrator, ולאחר מכן להפעיל במחשב היעד את קובץ ה-.exe שנוצר.
installer-cli-usage-title = תוכנית התקנה של WinAppMigrator עם חילוץ עצמי.
installer-cli-usage = יש להפעיל אותה ללא ארגומנטים כדי לפתוח את תוכנית ההתקנה הגרפית, או:
installer-cli-usage-preview = תצוגה מקדימה של ההתקנה מחדש: --dry-run [--target-root <DIR>]
installer-cli-usage-apply = התקנה מחדש:                  --apply [--target-root <DIR>]
installer-cli-no-package = תוכנית ההתקנה אינה מכילה חבילה מוטמעת
installer-cli-no-programdata = לא ניתן היה למצוא את התיקיה ProgramData של מחשב זה, ולכן לא הותקן דבר: כל התקנה שומרת שם את הרשומה שמשמשת לביטולה
# { $path } is the folder asked for, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-folder-has-files = לא ניתן להשתמש ב-{ $path }: { $problem }. יש לציין --force רק אם ידוע שהתיקיה שייכת לאפליקציה זו
installer-cli-no-location = חבילה זו אינה מציינת היכן הייתה האפליקציה שלה, ולכן יש לציין --target-root
# { $path } is where the app was on the other PC, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-location-refused = האפליקציה של חבילה זו הייתה ב-{ $path }, ולא ניתן להשתמש במיקום זה: { $problem }. יש לציין --target-root
# { $summary } is how much room it needs where, such as "up to 824.0 MB on C: (356.0 GB free)".
installer-cli-room = שטח: { $summary }
# { $note } is something to know about the install.
installer-cli-note = הערה: { $note }
# { $count } is how many things it sets up beyond its files.
installer-cli-sets-up = פריטים שמוגדרים: { $count }
installer-cli-applied = ההתקנה מחדש בוצעה.
installer-cli-previewed = בוצעה תצוגה מקדימה של ההתקנה מחדש.
# { $count } is how many changes the install logged.
installer-cli-changes-logged = שינויים שנרשמו: { $count }
# { $count } is how many warnings.
installer-cli-warnings = אזהרות: { $count }
# { $count } is how many errors.
installer-cli-errors = שגיאות: { $count }
# { $count } is how many things the package asked for and did not get.
installer-cli-refused = נדחו: { $count }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-package-valid = ערכי הגיבוב של החבילה תקינים: { $valid }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-reinstall-valid = ערכי הגיבוב של ההתקנה מחדש תקינים: { $valid }
# { $id } is the install's id, such as wam-0f3a….
installer-cli-install-id = מזהה ההתקנה: { $id }
installer-cli-put-back = ההתקנה נכשלה, ולכן השינויים שביצעה הוחזרו.
# { $path } is the log file.
installer-cli-log = יומן: { $path }
# { $path } is the log file. Printed before an error.
installer-cli-error-log = יומן: { $path }
# { $error } is what went wrong.
installer-cli-error = שגיאה: { $error }

## The uninstaller's window

uninstaller-window-title = הסרת התקנה
# The window's title, and its heading. { $app_name } is the name of the app it uninstalls.
uninstaller-window-title-app = הסרת ההתקנה של { $app_name }
uninstaller-tagline = ביטול מה שתוכנית ההתקנה עשתה
uninstaller-reading = קריאת רשומת ההתקנה…
uninstaller-stopped-unexpectedly = הסרת ההתקנה נעצרה באופן לא צפוי.
uninstaller-what-will-be-removed = מה יוסר
uninstaller-install-location = מיקום ההתקנה
uninstaller-files = קבצים
uninstaller-registry-values = ערכי רישום
uninstaller-services = שירותים
uninstaller-drivers = מנהלי התקנים
uninstaller-scheduled-tasks = משימות מתוזמנות
uninstaller-environment = משתני סביבה
uninstaller-plan-stays = קבצים שהוספת או ששינית נשארים, יחד עם התיקיות שבהן הם נמצאים.
# { $count } is how many files the install replaced.
uninstaller-plan-coming-back =
    { $count ->
        [one] קובץ אחד שהתקנה זו החליפה יחזור, או יוצב לצד הקובץ אם זה השתנה מאז.
        [two] שני קבצים שהתקנה זו החליפה יחזרו, או יוצבו לצד קובץ שהשתנה מאז.
       *[other] { $count } קבצים שהתקנה זו החליפה יחזרו, או יוצבו לצד קובץ שהשתנה מאז.
    }
# { $runtimes } is a list of runtimes, such as "Microsoft Visual C++ Redistributable, .NET Desktop Runtime 8".
uninstaller-plan-runtimes = רכיבי זמן הריצה שהיא התקינה נשארים, כי ייתכן שתוכניות אחרות משתמשות בהם: { $runtimes }.
uninstaller-options = אפשרויות
uninstaller-remove-user-data = הסרה גם של נתוני משתמש ונתוני רישיון
# { $count } is how many items of the app's settings, data, and license there are.
uninstaller-remove-user-data-detail =
    { $count ->
        [one] פריט אחד של ההגדרות, הנתונים והרישיון של האפליקציה. הוא נשאר אלא אם אפשרות זו מופעלת. מה שהוא החליף יחזור, או יוצב לצידו אם הפריט השתנה מאז.
        [two] שני פריטים של ההגדרות, הנתונים והרישיון של האפליקציה. הם נשארים אלא אם אפשרות זו מופעלת. מה שהם החליפו יחזור, או יוצב לצידם אם הם השתנו מאז.
       *[other] { $count } פריטים של ההגדרות, הנתונים והרישיון של האפליקציה. הם נשארים אלא אם אפשרות זו מופעלת. מה שהם החליפו יחזור, או יוצב לצידם אם הם השתנו מאז.
    }
uninstaller-uninstall = הסרת התקנה
uninstaller-cancel = ביטול
uninstaller-close = סגירה
# { $done } and { $total } are how many of the install's items are undone, and how many there are.
uninstaller-step-counted = ביטול ההתקנה: { $done } מתוך { $total }…
uninstaller-step = ביטול ההתקנה…
uninstaller-done = ההתקנה של האפליקציה הוסרה.
uninstaller-done-except = ההתקנה של האפליקציה הוסרה, פרט למה שמופיע למטה.
uninstaller-waits-for-drive = חלק ממנה נמצא בכונן שאינו מחובר. יש לחבר את הכונן, ולהסיר שוב את ההתקנה דרך "הגדרות".
uninstaller-not-all-undone = לא ניתן היה לבטל את כל ההתקנה. יש לסגור את מה שמשתמש בה, ולהסיר שוב את ההתקנה דרך "הגדרות".
# Over the path of the uninstall's log file.
uninstaller-log = יומן
uninstaller-undone = בוטלו
uninstaller-left-in-place = נשארו במקומם
uninstaller-could-not-be-undone = לא ניתן היה לבטל
# After the first part of a long list. { $count } is how many more there are.
uninstaller-and-more = ועוד { $count }.
# Said as the reason when there is no record of the install to read.
uninstaller-no-programdata = לא ניתן היה למצוא את התיקיה ProgramData של מחשב זה, ולכן אין רשומה לקריאה

## When the uninstaller's window cannot open. Each is a paragraph of a message box, followed by a command to type.

uninstaller-window-failed-title = תוכנית הסרת ההתקנה של WinAppMigrator
# { $error } is what went wrong, as Windows or the graphics driver says it.
uninstaller-window-failed = לא ניתן היה לפתוח את החלון של תוכנית הסרת ההתקנה: { $error }
uninstaller-window-failed-quiet = עדיין ניתן להסיר את ההתקנה בלי החלון, משורת פקודה של מנהל מערכת:
# Stands in for this program's own path in that command, if it cannot be found.
uninstaller-this-program = תוכנית זו

## The uninstaller's command line (--quiet). Shown in English: scripts read it.

# { $app_name } is the name of the app.
uninstaller-cli-uninstalling = הסרת ההתקנה של { $app_name }.
# { $path } is the log file.
uninstaller-cli-log = יומן: { $path }
# { $error } is what went wrong.
uninstaller-cli-error = שגיאה: { $error }
