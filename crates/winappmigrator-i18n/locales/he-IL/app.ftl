# Words of the app, WinAppMigrator itself: its one window, with the installed apps listed on the left and, on the
# right, what goes in an installer of the one picked and the button that builds it; and the steps to read another PC
# through WinAppMigrator Client. Message names start with app-.


## The window and its header

# The program's name, in the window's title bar and at the top of the window. Keep it as it is unless your language
# writes product names in a script of its own.
app-name = WinAppMigrator
app-tagline = הפיכת אפליקציה מותקנת לתוכנית התקנה עם חילוץ עצמי
app-theme-to-light = מעבר לערכת נושא בהירה
app-theme-to-dark = מעבר לערכת נושא כהה
app-reload-list = טעינה מחדש של רשימת האפליקציות
app-list-loading = בטעינה
# { $count } is how many apps are listed.
app-list-count =
    { $count ->
        [one] אפליקציה אחת
        [two] שתי אפליקציות
       *[other] { $count } אפליקציות
    }
app-list-load-failed = הטעינה נכשלה
# Shown when the window cannot open at all. { $error } is what went wrong, in the words of Windows or a graphics driver.
app-window-could-not-open = לא ניתן היה לפתוח את החלון: { $error }

## The list of installed apps

app-list-search = חיפוש לפי שם או מפרסם  (Ctrl+F)
app-list-filter-all = הכול
# Whether an app is installed for one account or for everyone on the PC.
app-scope-per-user = משתמש יחיד
app-scope-per-machine = כל המחשב
app-scope-unknown = לא ידוע
# An app whose processor architecture (x86, x64, ARM64) is not known.
app-arch-unknown = לא ידוע
app-list-loading-apps = טעינת האפליקציות המותקנות…
# { $error } is why the apps could not be listed.
app-list-could-not-list = לא ניתן היה להציג את רשימת האפליקציות: { $error }
app-list-no-matches = אין אפליקציות תואמות
app-list-no-matches-hint = יש לשנות את טקסט החיפוש או את מסנן ההיקף.
app-list-worker-disconnected = התהליך שאוסף את רשימת האפליקציות התנתק.
# What a screen reader says of an app's row. { $name } is the app's name, { $publisher } who made it, and { $version } its
# version, such as 11.4.2, or — when either is not known.
app-list-row = { $name }, { $publisher }, גרסה { $version }

## The list's columns, and what a screen reader says each column's heading does when clicked

app-column-name = שם
app-column-name-sort = מיון לפי שם
app-column-publisher = מפרסם
app-column-publisher-sort = מיון לפי מפרסם
app-column-version = גרסה
app-column-version-sort = מיון לפי גרסה
app-column-scope = היקף
app-column-scope-sort = מיון לפי היקף
# Short for architecture: x86, x64, or ARM64. The column is narrow.
app-column-arch = מעבד
app-column-arch-sort = מיון לפי מעבד

## Where apps are read from: this PC, a drive from another PC, a disk image of one, or another PC on the network

app-source = מקור הסריקה
app-source-this-pc = מחשב זה
app-source-drive = כונן מחובר…
app-source-image = תמונת דיסק…
app-source-network = מחשב ברשת…
app-source-attaching-image = צירוף התמונה, לקריאה בלבד…
# { $computer } is the other PC's name, such as OLD-PC, and { $address } its network address, such as 192.168.1.20.
app-source-network-pc = { $computer } דרך הרשת ({ $address })
app-source-disconnect = ניתוק
app-source-change = שינוי…
# The title of the window that picks a disk image. .vhd and .vhdx are file extensions.
app-source-pick-image-title = בחירת תמונת דיסק של המחשב האחר (.vhd או .vhdx)
app-source-pick-image-filter = תמונות דיסק
# { $path } is the disk image file.
app-source-image-has-no-windows = ב-{ $path } אין אמצעי אחסון שמותקן בו Windows.
app-source-image-not-attached = לא ניתן היה לצרף את תמונת הדיסק.
app-source-pick-drive-title = בחירת השורש של הכונן המחובר (לדוגמה, E:\)
# { $path } is the folder picked, such as E:\. \Windows and \Users are folder names, as Windows writes them.
app-source-not-a-windows-drive = { $path } אינו נראה ככונן מערכת של Windows (אין בו \Windows ו-\Users). יש לבחור את השורש של הכונן מהמחשב האחר.
# Said of a drive or disk image with a Windows older than Windows 10. { $windows } is its name, such as "Windows 7
# Professional Service Pack 1, 32-bit". { $is_32_bit } is "yes" when that Windows is 32-bit, which adds the last
# sentence, and "no" when it is not.
app-source-older-windows =
    כאן מותקן { $windows }. תוכניות התקנה שנבנות ממנו מיועדות ל-Windows 10 ו-11. עם "נתוני משתמש", מועברות גם הגדרות ש-Windows שמר בצד בשקט עבור אפליקציה ישנה, ורכיבי זמן ריצה ש-Windows 11 אינו כולל, כגון DirectX 9, מותקנים יחד עם האפליקציה.{ $is_32_bit ->
        [yes] { " " }האפליקציות שבו הן של 32 סיביות, ולכן הן מועברות אל Program Files (x86), שבה Windows של 64 סיביות שומר אותן.
       *[no] { "" }
    }
# The same, of a Windows whose name could not be read. { $is_32_bit } is as above.
app-source-older-windows-unnamed =
    כאן מותקנת גרסה ישנה יותר של Windows. תוכניות התקנה שנבנות ממנה מיועדות ל-Windows 10 ו-11. עם "נתוני משתמש", מועברות גם הגדרות ש-Windows שמר בצד בשקט עבור אפליקציה ישנה, ורכיבי זמן ריצה ש-Windows 11 אינו כולל, כגון DirectX 9, מותקנים יחד עם האפליקציה.{ $is_32_bit ->
        [yes] { " " }האפליקציות שבה הן של 32 סיביות, ולכן הן מועברות אל Program Files (x86), שבה Windows של 64 סיביות שומר אותן.
       *[no] { "" }
    }

## The details panel, before an app is picked, and the app picked

app-details-pair-first = התאמה עם המחשב הישן
app-details-pair-first-hint = לאחר ש-WinAppMigrator Client פועל שם והקלדת את הקוד שלו, האפליקציות שבמחשב הישן יופיעו כאן, וניתן לבנות תוכנית התקנה מאחת מהן כמו ממחשב זה.
app-details-pick-an-app = בחירת אפליקציה
app-details-pick-an-app-hint = יש לבחור אפליקציה מהרשימה כדי לבחור מה לכלול ולבנות את תוכנית ההתקנה שלה.
# { $publisher } is who made the app, and { $version } its version, such as 11.4.2.
app-details-publisher-and-version = { $publisher }, גרסה { $version }
# { $version } is the app's version, such as 11.4.2.
app-details-unknown-publisher-and-version = מפרסם לא ידוע, גרסה { $version }
app-details-unknown-publisher = מפרסם לא ידוע

## What goes in besides the program

app-option-license = נתוני רישיון והפעלה
app-option-license-hint = ייתכן שהם קשורים למחשב זה ויחייבו הפעלה חוזרת במחשב החדש.
app-option-user-data = נתוני משתמש
# Documents, Saved Games, and ProgramData are folders of Windows: use the names Windows gives them in your language,
# except ProgramData, which keeps its name.
app-option-user-data-hint = הגדרות והיסטוריה, מסמכים, משחקים שמורים ו-ProgramData.
app-option-whose-data = של מי הנתונים
app-option-whose-data-hint = תוכנית התקנה מכילה את הנתונים של חשבון אחד. הם מועברים למשתמש המחובר במחשב שבו היא מופעלת.

## Building the installer

app-output-folder = תיקיית פלט
# An example of a folder, shown in the empty field. "You" stands for the name of an account.
app-output-folder-example = C:\Users\You\WinAppMigrator
app-output-folder-not-full = יש להזין נתיב מלא של תיקיה, כגון D:\Installers.
app-build-create = יצירת תוכנית התקנה
app-build-create-again = יצירת תוכנית ההתקנה שוב
app-build-cancel = ביטול
app-build-cancelling = ביטול לאחר השלב הנוכחי…
app-build-cancelled = הבנייה בוטלה. לא נכתבה תוכנית התקנה.
# winappmigrator-installer.exe is a file name.
app-build-no-stub = שלד תוכנית ההתקנה (winappmigrator-installer.exe) לא נמצא לצד אפליקציה זו, ולכן לא ניתן לבנות תוכנית התקנה עם חילוץ עצמי.
app-build-select-an-app-first = יש לבחור תחילה אפליקציה להעברה.
app-build-look-again-first = יש לסרוק שוב אפליקציה זו לפני שניתן יהיה לבנות אותה.
# { $reason } is what went wrong, in the program's own words, often in English.
app-build-stopped-unexpectedly = הבנייה נעצרה באופן לא צפוי: { $reason }. אם היא השאירה תוכנית התקנה בתיקיית הפלט, אין להשתמש בה.
app-build-worker-disconnected = תהליך ההעברה התנתק.
# { $path } is the output folder, and { $error } what Windows said went wrong.
app-build-no-output-folder = לא ניתן היה ליצור את תיקיית הפלט { $path }: { $error }
# { $error } is what went wrong.
app-build-no-staging-folder = לא ניתן היה ליצור תיקיית ביניים מוגנת, ולכן שום דבר לא נבנה: { $error }
# { $error } is what went wrong, as a sentence.
app-build-could-not-build = לא ניתן היה לבנות את תוכנית ההתקנה. { $error }
app-build-no-installer = הבנייה הסתיימה ללא תוכנית התקנה.
# { $error } is what went wrong.
app-build-hash-check-did-not-run = תוכנית ההתקנה נכתבה, אך לא ניתן היה להריץ את בדיקת הגיבוב שלה: { $error }. אין להשתמש בה.

## How far a build has got

app-progress-preparing = בירור מה ייכלל…
app-progress-checking-room = בדיקה שיש מספיק שטח…
app-progress-bundling = צירוף רכיבי זמן ריצה ומנהלי התקנים…
# { $done } is how many files have been read so far, and { $total } how many there are.
app-progress-packing = קריאה ודחיסה של קבצים: { $done } מתוך { $total }
# { $done } is how many of the installer's records have been looked over so far, and { $total } how many there are.
app-progress-looking-over = בדיקת מה שתוכנית ההתקנה אומרת: { $done } מתוך { $total }…
app-progress-writing = כתיבת תוכנית ההתקנה…
app-progress-verifying = אימות ערכי הגיבוב…

## What a build made

app-result-failed-hash-check = אין להשתמש בתוכנית התקנה זו: היא נכשלה בבדיקת הגיבוב.
# { $count } is how many files were gone when the build came to read them.
app-result-files-gone =
    { $count ->
        [one] תוכנית ההתקנה נבנתה, אך קובץ אחד נעלם מאז שהאפליקציה נסרקה, ולכן הוא אינו כלול בה. יש לעיין באזהרות שלמטה.
        [two] תוכנית ההתקנה נבנתה, אך שני קבצים נעלמו מאז שהאפליקציה נסרקה, ולכן הם אינם כלולים בה. יש לעיין באזהרות שלמטה.
       *[other] תוכנית ההתקנה נבנתה, אך { $count } קבצים נעלמו מאז שהאפליקציה נסרקה, ולכן הם אינם כלולים בה. יש לעיין באזהרות שלמטה.
    }
# { $count } is how many files could not be read into the installer.
app-result-files-not-captured =
    { $count ->
        [one] תוכנית ההתקנה נבנתה, אך לא ניתן היה ללכוד קובץ אחד, בדרך כלל מכיוון שהוא היה נעול או לא קריא. ייתכן שהאפליקציה לא תפעל בלעדיו. יש לעיין באזהרות שלמטה.
        [two] תוכנית ההתקנה נבנתה, אך לא ניתן היה ללכוד שני קבצים, בדרך כלל מכיוון שהם היו נעולים או לא קריאים. ייתכן שהאפליקציה לא תפעל בלעדיהם. יש לעיין באזהרות שלמטה.
       *[other] תוכנית ההתקנה נבנתה, אך לא ניתן היה ללכוד { $count } קבצים, בדרך כלל מכיוון שהם היו נעולים או לא קריאים. ייתכן שהאפליקציה לא תפעל בלעדיהם. יש לעיין באזהרות שלמטה.
    }
app-result-mentions-who = תוכנית ההתקנה נבנתה, ועדיין יש בה מידע מסוים על מי שהיא הגיעה ממנו. יש לקרוא את מה שכתוב למטה לפני שמעבירים אותה למישהו.
app-result-ready = תוכנית ההתקנה מוכנה. החבילה שלה עברה את בדיקת הגיבוב.
app-result-files = קבצים
app-result-installer-size = גודל תוכנית ההתקנה
app-result-warnings = אזהרות
app-result-log = יומן
app-result-copy-error = העתקת השגיאה
app-result-installer = תוכנית ההתקנה
app-result-data-file = קובץ הנתונים
app-result-open-folder = פתיחת התיקיה
app-result-copy-path = העתקת הנתיב
# { $error } is what Windows said went wrong.
app-result-could-not-open-folder = לא ניתן היה לפתוח את התיקיה: { $error }
# { $installer } and { $data_file } are file names, such as App_installer.exe and App_installer.wamdata.
app-result-two-files = אפליקציה זו גדולה מדי עבור קובץ הפעלה אחד, ולכן תוכנית ההתקנה שלה מורכבת משני קבצים, { $installer } ו-{ $data_file }. הם חייבים להישאר יחד: יש להעתיק את שניהם למחשב האחר, לאותה תיקיה, בשמות אלה. אם חותמים על תוכנית ההתקנה, החתימה מכסה גם את קובץ הנתונים.
# { $app_name } is the app's name. "Unknown publisher" is what Windows shows when it asks to run an unsigned program: use
# Windows' own words for it in your language.
app-result-run-it = יש להפעיל אותה במחשב האחר כדי להתקין מחדש את { $app_name }. היא בודקת את הקבצים שלה באמצעות SHA-256. Windows מציג "מפרסם לא ידוע" מכיוון שתוכניות התקנה שבנית אינן חתומות.
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-verification-errors = שגיאות אימות ({ $count })
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-warnings-heading = אזהרות ({ $count })
# After the first of a long list of warnings. { $count } is how many more there are.
app-warnings-and-more = ועוד { $count }.

## Looking at an app, to see what an installer of it would hold

app-scan-getting-ready = הכנה לסריקת אפליקציה זו…
app-scan-install-record = קריאת האופן שבו היא הותקנה…
app-scan-near-matches = חיפוש של מה שעשוי גם להיות חלק ממנה…
app-scan-system-items = חיפוש השירותים, המשימות וקיצורי הדרך שלה…
app-scan-files = מעבר על הקבצים שלה…
app-scan-registry = קריאת מה שמקשר אותה ל-Windows…
app-scan-finishing = סיום…
app-scan-look-again = סריקה חוזרת
app-scan-stop = עצירה
app-scan-running-hint = ניתן לבחור אפליקציה אחרת או לשנות את מה שנכלל בזמן שהסריקה פועלת.
# { $error } is why, in words that follow a colon.
app-scan-failed = לא ניתן היה לסרוק אפליקציה זו: { $error }
app-scan-try-again = ניסיון חוזר
app-scan-stopped = נעצר. ניתן יהיה לבנות תוכנית התקנה לאחר שאפליקציה זו תיסרק.
# { $reason } is what went wrong, in the program's own words, often in English.
app-scan-stopped-unexpectedly = הסריקה נעצרה באופן לא צפוי: { $reason }.

## How big an installer can be, and whether there is room for it

app-room-files = קבצים
# Over how big the installer can be: up to that size.
app-room-up-to = עד
app-room-free = פנוי
# { $drive } is a drive, such as C:.
app-room-free-on = פנוי ב-{ $drive }
# How much room a drive has, when it would not say.
app-room-free-unknown = לא ידוע
app-room-most-it-could-need = השטח המרבי שעשוי להידרש, לפני דחיסה.
# { $drive } is a drive, such as C:, { $needed } how much room it needs, such as 1.2 GB, and { $free } how much it has.
app-room-drive-short = בכונן { $drive } נדרשים עד { $needed } פנויים כדי לבנות זאת, ופנויים בו { $free }.
# { $drives } is one sentence like the one above for each drive without the room, one after another.
app-room-not-enough = אין מספיק שטח. { $drives } יש לבחור תיקיית פלט בכונן אחר, או להשמיט משהו.
# { $note } is a sentence, without its full stop, about a drive whose room could not be checked. Use your language's
# full stop.
app-room-could-not-tell = { $note }.

## What goes in the installer, part by part

app-footprint-heading = מה נכלל בתוכנית ההתקנה
app-footprint-working-out = בירור מה נלווה לכך…
# { $error } is why.
app-footprint-would-not-make-a-package = ממה שנשאר לא ניתן ליצור חבילה: { $error }
# { $list } is the names of what else is left out, such as a service and a shortcut.
app-footprint-also-left-out = פעולה זו משמיטה גם את { $list }, מכיוון שמה שהם מפעילים או מציינים לא יהיה שם. הם לא יוגדרו במחשב האחר.
# { $list } is the names of the first few of what else is left out, and { $count } how many more there are.
app-footprint-also-left-out-and-more = פעולה זו משמיטה גם את { $list } ועוד { $count }, מכיוון שמה שהם מפעילים או מציינים לא יהיה שם. הם לא יוגדרו במחשב האחר.
# { $list } is the names of license files. The option is the one named "License and Activation Data".
app-footprint-licenses-still-go = נתוני רישיון במה שהשמטת עדיין נכללים, מכיוון שהם נקבעים לפי האפשרות "נתוני רישיון והפעלה": { $list }.
# { $elapsed } is how long ago, in minutes and seconds such as 45:00, or hours, minutes, and seconds such as 1:05:00.
# "Look Again" names the button of that name.
app-footprint-looked-at-long-ago = אפליקציה זו נסרקה לפני { $elapsed }. הקבצים נקראים בזמן הבנייה, כך ששינוי ייקלט, וכל מה שנעלם עד אז יופיע ברשימה. כדי למצוא דברים חדשים, יש ללחוץ על "סריקה חוזרת".
# Headings of the parts of the list, each with a line that says what is in it.
app-footprint-program-files = קובצי התוכנית
app-footprint-data-folders = תיקיות נתונים
# What could go in with the User Data option, when nothing else is in this part. { $size } is how much, such as 1.2 GB.
app-footprint-folders-with-user-data = { $size } עם האפשרות "נתוני משתמש"
# { $included } is how many folders go in, { $count } how many there are, and { $size } how much those that go in hold.
app-footprint-folders-summary =
    { $count ->
        [one] { $included } מתוך תיקיה אחת, { $size }
        [two] { $included } מתוך שתי תיקיות, { $size }
       *[other] { $included } מתוך { $count } תיקיות, { $size }
    }
# { $size } is how much, such as 1.2 GB, and { $count } in how many files.
app-footprint-data-only-with-user-data =
    { $count ->
        [one] { $size } בקובץ אחד של נתוני אפליקציה זו ייכללו רק עם האפשרות "נתוני משתמש".
        [two] { $size } בשני קבצים של נתוני אפליקציה זו ייכללו רק עם האפשרות "נתוני משתמש".
       *[other] { $size } ב-{ $count } קבצים של נתוני אפליקציה זו ייכללו רק עם האפשרות "נתוני משתמש".
    }
# A folder's row. { $count } is how many files it holds, and { $detail } where it is or why it cannot be left out.
app-footprint-folder-row =
    { $count ->
        [one] קובץ אחד. { $detail }
        [two] שני קבצים. { $detail }
       *[other] { $count } קבצים. { $detail }
    }
app-footprint-services = שירותים
app-footprint-startup = הפעלה אוטומטית
app-footprint-tasks = משימות מתוזמנות
app-footprint-shortcuts = קיצורי דרך
# { $included } is how many of the part's things go in, and { $count } how many there are.
app-footprint-items-summary = { $included } מתוך { $count }
# { $why } is what else was left out that this goes with.
app-footprint-left-out-too = הושמט גם כן: { $why }
# A row's detail. { $kind } is what it is, such as Service, and { $detail } what it does.
app-footprint-change-detail = { $kind }. { $detail }
app-footprint-integrations = מה שמקשר אותה ל-Windows
# { $included } is how many kinds go in, such as file types and firewall rules, and { $count } how many kinds there are.
app-footprint-integrations-summary =
    { $count ->
        [one] { $included } מתוך סוג אחד
        [two] { $included } מתוך שני סוגים
       *[other] { $included } מתוך { $count } סוגים
    }
app-footprint-integration-nothing-left = לא נשאר ממנו דבר, בגלל מה שהשמטת
# { $count } is how many things of this kind the installer sets up.
app-footprint-integration-things =
    { $count ->
        [one] פריט אחד מוגדר
        [two] שני פריטים מוגדרים
       *[other] { $count } פריטים מוגדרים
    }
app-footprint-prerequisites = מה שנדרש לה כדי לפעול
# { $count } is how many runtimes, such as Visual C++ Redistributable.
app-footprint-prerequisites-summary =
    { $count ->
        [one] רכיב זמן ריצה אחד
        [two] שני רכיבי זמן ריצה
       *[other] { $count } רכיבי זמן ריצה
    }
app-footprint-near-matches = עשוי גם להיות חלק ממנה
# { $ticked } is how many were ticked, and { $count } how many there are.
app-footprint-near-matches-summary = נוספו { $ticked } מתוך { $count }
app-footprint-near-matches-hint = פריטים אלה אינם נכללים בתוכנית ההתקנה אלא אם מסמנים אותם. סימון פריט סורק שוב את האפליקציה, עם אותן בדיקות כמו לשאר, כך שמה שנראה כמו סיסמה עדיין נשאר בחוץ.
# { $why } is why it might be part of the app, as a sentence.
app-footprint-near-match-with-user-data = מגיע עם האפשרות "נתוני משתמש". { $why }
# How much a folder holds, when not all of it could be measured. { $size } is how much was, such as 1.2 GB.
app-footprint-near-match-over = יותר מ-{ $size }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-folder = תיקיה: { $place }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-hidden = חלק מוסתר של אפליקציה: { $place }
app-footprint-nothing-left-out = שום דבר לא הושמט בשל דמיון לסיסמה, לרישיון או לאישור.
app-footprint-left-out = מה הושמט, ומדוע
# { $count } is how many things were left out.
app-footprint-left-out-summary =
    { $count ->
        [one] פריט אחד
        [two] שני פריטים
       *[other] { $count } פריטים
    }
# Above a list of things left out for one reason. { $title } is the reason.
app-footprint-left-out-group = { $title }:
app-footprint-refused = אף תוכנית התקנה לא תורשה להגדיר את אלה:
# A heading over a list that opens and closes. { $count } is how many are in it.
app-footprint-worth-knowing = כדאי לדעת ({ $count })
# { $list } is what kinds of things were looked for and none found, from the five below.
app-none-found = לא נמצאו: { $list }.
app-none-found-services = שירותים
app-none-found-startup = פריטי אתחול
app-none-found-tasks = משימות מתוזמנות
app-none-found-shortcuts = קיצורי דרך
app-none-found-integrations = פריטים שמקשרים אותה ל-Windows

## Network PC: reading another PC through WinAppMigrator Client, in three steps. "WinAppMigrator Client" is the name of a
## program, and the old PC is the one the apps are moved off.

app-network-step-client = 1. העתקת הלקוח למחשב הישן
app-network-step-client-hint = WinAppMigrator Client פועל במחשב שבו האפליקציה נמצאת כעת, עם Windows 7 ואילך. הוא רק מאפשר למחשב זה לקרוא אותו, בהצפנה, באמצעות קוד שהוא מציג.
# { $drive } is a USB drive, such as E:\.
app-network-save-to-usb = שמירה בכונן USB { $drive }
# { $drive } is a USB drive, such as E:\, and { $name } the name it was given, such as BACKUP.
app-network-save-to-named-usb = שמירה בכונן USB { $drive } ({ $name })
app-network-save-to-folder = שמירה בתיקיה…
app-network-look-for-usb-again = חיפוש חוזר של כונני USB
app-network-no-usb = לא מחובר כונן USB. ניתן לחבר כונן, או לשמור בתיקיה: תיקיה שניתן לגשת אליה מהמחשב הישן, כגון תיקיה משותפת ברשת.
# { $path } is where the client was saved, such as E:\WinAppMigrator Client.exe.
app-network-client-saved = נשמר ב-{ $path }. במחשב הישן, יש לסגור את האפליקציה שמעבירים, ואז ללחוץ על הקובץ לחיצה כפולה ולאפשר לו לבצע שינויים. הוא יציג קוד.
app-network-step-pick = 2. בחירת המחשב הישן
app-network-looking = חיפוש ברשת אחר מחשבים שהלקוח פועל בהם…
app-network-type-name = או הקלדת השם או הכתובת שלו
# An example, shown in the empty field: a PC's name, or its network address.
app-network-type-name-example = OLD-PC או 192.168.1.20
app-network-step-code = 3. הקלדת הקוד שהוא מציג
app-network-code = קוד התאמה
app-network-connect = התחברות
app-network-pairing = ביצוע התאמה…
# Why pairing did not work, shown on its own.
app-network-pairing-stopped = ההתאמה נעצרה לפני שהסתיימה
app-network-wrong-code = זה אינו הקוד שהמחשב הישן מציג. יש לבדוק אותו ולנסות שוב.
# { $error } is what went wrong.
app-network-not-reached = לא ניתן היה להגיע למחשב הישן: { $error }
app-network-pick-folder-title = בחירת תיקיה עבור WinAppMigrator Client
# { $path } is the folder, and { $error } what Windows said went wrong.
app-client-could-not-save = לא ניתן היה לשמור את הלקוח ב-{ $path }: { $error }
# scripts\build-client.ps1 is a file name.
app-client-not-carried = גרסה זו של WinAppMigrator אינה כוללת את הלקוח. יש לבנות אותו באמצעות scripts\build-client.ps1, ולאחר מכן לבנות שוב את האפליקציה.
# { $error } is what Windows said went wrong.
app-client-could-not-save-note = לא ניתן היה לשמור את ההערה שלצידו: { $error }
# A note saved as a text file beside the client, for whoever runs it on the old PC. Keep its lines short, as it is read in
# Notepad. "WinAppMigrator Client.exe" is a file name. "Network PC..." names the Scan Source button: use its words in your
# language.
app-client-note =
    WinAppMigrator Client
    
    יש להפעיל קובץ זה במחשב שממנו מעבירים את האפליקציות: Windows 7, 8, 10 או 11.
    
    1. יש לסגור את האפליקציה שמעבירים.
    2. יש ללחוץ לחיצה כפולה על "WinAppMigrator Client.exe" ולאפשר לו לבצע
       שינויים.
    3. הוא יציג קוד התאמה בן 6 ספרות. ב-WinAppMigrator במחשב החדש, יש לבחור
       "מחשב ברשת...", לבחור מחשב זה ולהקליד את הקוד.
    4. כשהמחשב החדש מסיים, יש להקיש Enter בחלון של הלקוח, או לסגור אותו.
    
    הוא רק קורא. בזמן שהוא פועל, הוא שומר תמונת מצב של כונן Windows ומאפשר
    לשתי היציאות שלו לעבור דרך חומת האש של Windows מהרשת המקומית, והוא מסיר
    את שניהם כשהוא נעצר. מה שעובר ברשת מוצפן, ורק מחשב שיש לו את הקוד יכול
    לקרוא משהו.
