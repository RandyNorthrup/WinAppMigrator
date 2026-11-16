# Words the core writes for people as it installs an app from a package: why an install will not start, the list of
# what it did (or for a preview, what it would do) that the installer window shows, its warnings, the record it keeps of
# each install so that it can be undone, and the core's errors. Message names start with core-.
#
# The list of what an install did is short and plain, one line for each thing, as the installer window lists them:
# "copied", "restored", "would copy". Words such as --force and --apply are command-line options, and LocalService,
# SYSTEM, ProgramData, and %AppData% are as Windows names them: keep them as they are.


## Errors: what went wrong, when the core cannot go on. Each is shown on its own, or after words that say what was being
## done.

core-error-list-changed = מה שההתקנה הזו תעשה השתנה מאז שאושר, ולכן שום דבר לא הותקן
# { $path } is a file or folder, such as C:\Program Files\App\app.exe, or words that name one, such as "the installer".
# { $error } is what Windows said went wrong.
core-error-io = פעולת קלט/פלט נכשלה עבור { $path }: { $error }
# { $detail } is what went wrong.
core-error-registry = קריאת הרישום נכשלה: { $detail }
# { $path } is a program file. { $detail } is what was wrong with it.
core-error-pe = ניתוח PE נכשל עבור { $path }: { $detail }
# { $detail } is what was wrong, already in words.
core-error-invalid-input = קלט לא חוקי: { $detail }
# { $detail } is what went wrong.
core-error-serialization = עריכה בסידרה של הדוח נכשלה: { $detail }
# { $detail } is what went wrong.
core-error-archive = פעולת הארכיון נכשלה: { $detail }
# { $detail } is what went wrong.
core-error-network = פעולת הרשת נכשלה: { $detail }
# { $detail } is what went wrong.
core-error-time-format = עיצוב השעה נכשל: { $detail }
core-error-cancelled = הפעולה בוטלה
# What is said when part of the program stopped unexpectedly and gave no reason.
core-error-panic-no-reason = לא ניתנה סיבה

## The record of each install, kept so that it can be undone.

# { $kind } is a number that says what type of registry value it is.
core-journal-unknown-value-type = סוג ערך רישום לא ידוע { $kind }
core-journal-not-elevated = ללא הרשאות מוגברות
core-journal-needs-admin = ההתקנה דורשת הרשאות מנהל מערכת, כדי לשמור את הרשומה שלה במקום שאחרים אינם יכולים לשנות
# { $path } is a folder.
core-journal-not-an-install-folder = זו אינה תיקיה של התקנה: { $path }
# { $path } is the file the record is kept in.
core-journal-not-written = לא ניתן היה לכתוב את רשומת ההתקנה: { $path }
# { $path } is a folder.
core-journal-path-escapes-root = הנתיב יוצא מחוץ לשורש המיועד: { $path }
# { $path } is a folder.
core-journal-nothing-exists = אף אחד מאלה אינו קיים במחשב זה: { $path }
core-journal-busy = התקנה או הסרת התקנה אחרת פועלת. יש להמתין לסיומה, ולאחר מכן לנסות שוב
# Said of a file that was to get back the permissions it had.
core-journal-no-folder = אין לו תיקיה
# Said of the permissions a file had, which were kept to give back to it.
core-journal-permissions-unreadable = לא ניתן היה לקרוא את מה שנשמר
# { $folder } is the folder of one install's record.
core-journal-unreadable-empty = לא ניתן להשתמש ברשומת ההתקנה ב-{ $folder }: היא ריקה
# { $folder } is the folder of one install's record.
core-journal-unreadable-no-header = לא ניתן להשתמש ברשומת ההתקנה ב-{ $folder }: היא אינה מתחילה בתיאור של מה שהיא עוסקת בו
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-version = לא ניתן להשתמש ברשומת ההתקנה ב-{ $folder }: היא נכתבה על ידי גרסה אחרת של תוכנית זו
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-install = לא ניתן להשתמש ברשומת ההתקנה ב-{ $folder }: לפי מה שכתוב בה, היא שייכת להתקנה אחרת
# { $id } is an install's id, such as wam-1a2b3c.
core-journal-not-an-install-id = זה אינו מזהה של התקנה: { $id }

## Why an install will not start, or stopped before it changed anything.

core-restore-apply-and-dry-run = לא ניתן לציין גם --apply וגם --dry-run
# { $path } is the folder that was chosen to install in.
core-restore-location-not-full-path = מיקום ההתקנה חייב להיות נתיב מלא: "{ $path }"
# { $problem } says what is wrong with the folder, already in words. { $path } is the folder.
core-restore-location-unusable = לא ניתן להשתמש במיקום התקנה זה: { $problem }: "{ $path }"
core-restore-needs-admin-plan = תוכנית השחזור כוללת שינויים ברישום, בשירותים, במשימות מתוזמנות, בקיצורי דרך או ביעדים מוגנים, שמותרים רק למנהל מערכת
# { $path } is the folder that was chosen to install in.
core-restore-needs-admin-target = שורש היעד דורש הרשאות מוגברות: { $path }
core-restore-needs-admin-user-data = שחזור נתוני משתמש אל ProgramData או אל מיקומים מוגנים אחרים דורש הרשאות מוגברות
# { $error } is why the install stopped. { $failed } is what could not be put back, as sentences of their own.
core-restore-not-all-put-back = { $error }. לא ניתן היה להחזיר את כל מה שבוצע: { $failed }
# { $error } is what went wrong.
core-restore-nowhere-to-keep-record = אין מקום בטוח לשמירת הרשומה שתאפשר לבטל התקנה זו, ולכן שום דבר לא הותקן: { $error }
# { $path } is a file the app would have put there.
core-restore-already-exists = { $path } כבר קיים, ולכן שום דבר לא הותקן. כדי להחליף אותו, יש להשתמש באפשרות --force
# { $root } is a folder the app has files for. { $link } is that folder, or one it is in.
core-restore-root-under-link = לאפליקציה זו יש פריטים לשחזור אל { $root }, ו-{ $link } הוא קישור למקום אחר, שאחרים עשויים להיות מסוגלים לשנות, ולכן שום דבר לא הותקן. אם מה שמגיע לשם הוא נתוני משתמש או נתוני רישיון, יש להתקין אותה ללא נתוני המשתמש ונתוני הרישיון שלה. אם זהו קיצור דרך בתיקיית אתחול, יש להחזיר את התיקיה למקום שבו הייתה
core-restore-hash-check-failed = החבילה נכשלה בבדיקת הגיבוב, ושום דבר לא הותקן
# { $detail } is the first thing found wrong, already in words.
core-restore-hash-check-failed-because = החבילה נכשלה בבדיקת הגיבוב, ושום דבר לא הותקן: { $detail }
# { $path } is a file or folder.
core-restore-refuse-records-folder = כתיבה למקום שבו התקנות שומרות את הרשומות שלהן אינה מותרת: { $path }
# { $path } is a file or folder.
core-restore-refuse-outside = כתיבה מחוץ למיקום ההתקנה ולתיקיות הנתונים הידועות אינה מותרת: { $path }
# { $path } is a file.
core-restore-no-parent-folder = ליעד אין תיקיית אב: { $path }
# { $path } is where a file was to go.
core-restore-folder-in-the-way = במקום שבו צריך להיות קובץ זה נמצאת תיקיה: { $path }
# { $path } is a file in the package.
core-restore-file-changed = { $path } כבר אינו זהה למה שהחבילה מכילה, ולכן הוא לא הותקן
# { $path } is a file in the package.
core-restore-source-is-link = { $path } הוא קישור, ואין לעקוב אחריו
# { $path } is a file in the package.
core-restore-source-became-link = { $path } הפך לקישור בזמן שנפתח
# Why a service did not start, after "did not start:".
core-restore-service-stopped-by-itself = הוא נעצר שוב מעצמו
# Why part of the app was left out, after "was left out:".
core-restore-com-class-someone-elses = מחלקת COM זו כבר קיימת במחשב זה, כשייכת למשהו אחר

## What an install did, one line for each thing, or for a preview, what it would do. { $source } is a file in the
## package, and { $destination } and { $path } are where it goes on this PC. { $key } is a registry key, such as
## HKLM\Software\App.

# { $package_valid } is true or false, and { $restore_valid } is true, false, or not_run: keep them as they are.
core-restore-verification-result = אימות לאחר השחזור package_valid={ $package_valid } restore_valid={ $restore_valid }
# { $name } is a service's name.
core-restore-started-service = הופעל השירות { $name }
core-restore-would-copy = יועתק { $source } -> { $destination }
core-restore-copied = הועתק { $source } -> { $destination }
core-restore-would-create-empty-folder = תיווצר התיקיה הריקה { $path }
core-restore-created-empty-folder = נוצרה התיקיה הריקה { $path }
# { $value } is the name of a value in the key.
core-restore-would-restore-registry-value = ישוחזר ערך הרישום { $key } { $value }
core-restore-would-restore-registry-key = ישוחזר מפתח הרישום { $key }
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-would-add-firewall-rule = יתווסף כלל חומת האש { $name } עבור { $program }
# { $name } is a firewall rule's name.
core-restore-firewall-rule-already-there = כלל חומת האש { $name } כבר קיים
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-added-firewall-rule = נוסף כלל חומת האש { $name } עבור { $program }
# { $name } is a service's name, { $command } what it runs, and { $account } the account it runs as, such as LocalService.
core-restore-would-restore-service = ישוחזר השירות { $name } -> { $command } (פועל בתור { $account })
# { $name } is a runtime, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-restore-prerequisite-present = הדרישה המוקדמת כבר קיימת: { $name }
# { $name } is a runtime that Windows has as a Windows feature.
core-restore-would-install-prerequisite-feature = תותקן הדרישה המוקדמת { $name } (תכונת Windows)
# { $name } is a runtime whose installer is in the package.
core-restore-would-install-prerequisite-bundled = תותקן הדרישה המוקדמת { $name } (תוכנית התקנה מצורפת)
# { $name } is a runtime that would come through winget, which is a program's name.
core-restore-would-install-prerequisite-winget = תותקן הדרישה המוקדמת { $name } (winget)
# { $name } is a runtime.
core-restore-installed-prerequisite = הותקנה הדרישה המוקדמת { $name }
# { $name } is a runtime.
core-restore-installed-prerequisite-reboot = הותקנה הדרישה המוקדמת { $name } (נדרשת הפעלה מחדש)
core-restore-would-restore-license = ישוחזר הרישיון { $source } -> { $destination }
core-restore-restored-license = שוחזר הרישיון { $source } -> { $destination }
core-restore-would-restore-license-registry = ישוחזר רישיון ברישום { $key }
# { $name } is a driver's name.
core-restore-driver-not-restored = מנהל ההתקן לא שוחזר אוטומטית: { $name }
# { $name } is a driver's name.
core-restore-would-install-driver = יותקן מנהל ההתקן { $name } -> { $destination }
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-would-restore-task = תשוחזר המשימה המתוזמנת { $name }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-startup-shortcut = ישוחזר קיצור הדרך לאתחול { $shortcut } -> { $target }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-shortcut = ישוחזר קיצור הדרך { $shortcut } -> { $target }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-would-restore-variable =
    { $scope ->
        [user] ישוחזר משתנה הסביבה של המשתמש { $name }
       *[machine] ישוחזר משתנה הסביבה של המחשב { $name }
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-left-variable =
    { $scope ->
        [user] משתנה הסביבה של המשתמש { $name } נשאר כפי שהוא: שום חלק ממנו אינו שייך לאפליקציה זו
       *[machine] משתנה הסביבה של המחשב { $name } נשאר כפי שהוא: שום חלק ממנו אינו שייך לאפליקציה זו
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-restored-variable =
    { $scope ->
        [user] שוחזר משתנה הסביבה של המשתמש { $name }
       *[machine] שוחזר משתנה הסביבה של המחשב { $name }
    }
core-restore-set-uninstaller = הוגדרה תוכנית הסרת ההתקנה עבור { $key }
core-restore-placed-driver-image = הוצבה תמונת מנהל ההתקן { $source } -> { $destination }
# { $name } is a driver's name.
core-restore-configured-driver = הוגדר מנהל ההתקן { $name }
# { $name } is a driver's name.
core-restore-created-driver = נוצר מנהל ההתקן { $name }
core-restore-ensured-registry-key = אומת קיומו של מפתח הרישום { $key }
# { $value } is the name of a value in the key.
core-restore-restored-registry-value = שוחזר ערך הרישום { $key } { $value }
# "(Default)" is how Windows names the value a key has with no name: write it as Registry Editor does.
core-restore-restored-registry-default = שוחזר ערך הרישום { $key } (ברירת מחדל)
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-configured-service = הוגדר השירות { $name } (פועל בתור { $account })
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-created-service = נוצר השירות { $name } (פועל בתור { $account })
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-restored-task = שוחזרה המשימה המתוזמנת { $name }
core-restore-restored-shortcut = שוחזר קיצור הדרך { $path }

## Warnings an install gives, and why it failed. They are shown after it, on the PC it ran on.

core-warning-restore-earlier-not-trusted = רשומה של התקנה שלא הסתיימה לא טופלה, מכיוון שאחרים יכולים לשנות אותה. מה שאותה התקנה שינתה עדיין נמצא במקומו.
# { $app_name } is the app's name. { $count } is how many changes were put back.
core-warning-restore-earlier-undone =
    { $count ->
        [one] התקנה קודמת של { $app_name } לא הסתיימה, ולכן היא בוטלה תחילה. שינוי אחד הוחזר.
        [two] התקנה קודמת של { $app_name } לא הסתיימה, ולכן היא בוטלה תחילה. שני שינויים הוחזרו.
       *[other] התקנה קודמת של { $app_name } לא הסתיימה, ולכן היא בוטלה תחילה. { $count } שינויים הוחזרו.
    }
# { $app_name } is the app's name.
core-warning-restore-earlier-never-finished = התקנה קודמת של { $app_name } לא הסתיימה. ההתקנה תבטל אותה תחילה.
core-warning-restore-user-data-real-locations = נתוני המשתמש משוחזרים למיקומים האמיתיים של %AppData%/%ProgramData% במחשב זה, במקום לשורש היעד.
# { $error } is what went wrong.
core-warning-restore-commit-failed = לא ניתן היה להשלים את רשומת ההתקנה: { $error }
# { $name } is a service's name.
core-warning-restore-service-still-starting = התקבלה בקשה להפעיל את השירות { $name }, והוא עדיין לא סיים לעלות. ייתכן שעוד יעלה. הוא גם יופעל מעצמו בהפעלה הבאה של מחשב זה.
# { $name } is a service's name. { $reason } is why it did not start, as Windows said it.
core-warning-restore-service-not-started = השירות { $name } מותקן ולא הופעל: { $reason }. ייתכן שהאפליקציה לא תפעל עד שהוא יופעל. Windows ינסה שוב בהפעלה הבאה של מחשב זה.
# { $name } is a service's name. { $reason } is why it could not be stopped.
core-warning-restore-service-not-stopped = לא ניתן היה לעצור תחילה את השירות { $name }: { $reason }. ייתכן שהוא ימשיך להריץ את התוכנית הישנה עד להפעלה הבאה של מחשב זה.
core-warning-restore-install-cancelled = ההתקנה בוטלה.
core-warning-restore-preview-cancelled = התצוגה המקדימה בוטלה.
# { $item } is what was left out, such as a file or a service's name. { $reason } is why, already in words.
core-warning-restore-left-out = { $item } הושמט: { $reason }.
# { $error } is what went wrong.
core-warning-restore-verification-failed = לא ניתן היה להריץ את האימות לאחר השחזור: { $error }
# { $runner } says what relies on the file: a service, a scheduled task, a firewall rule, a shortcut, or a registry value.
# { $path } is the file. { $name } is the name of what relies on it. { $refusal } is why the file was not replaced,
# already in words.
core-warning-restore-program-would-run =
    { $runner ->
        [service] { $path } אינו קובץ שחבילה זו רשאית להחליף, והשירות { $name } מסתמך עליו, ולכן שום דבר לא הותקן: { $refusal }
        [task] { $path } אינו קובץ שחבילה זו רשאית להחליף, והמשימה המתוזמנת { $name } מסתמכת עליו, ולכן שום דבר לא הותקן: { $refusal }
        [firewall] { $path } אינו קובץ שחבילה זו רשאית להחליף, וכלל חומת האש { $name } מסתמך עליו, ולכן שום דבר לא הותקן: { $refusal }
        [shortcut] { $path } אינו קובץ שחבילה זו רשאית להחליף, וקיצור הדרך { $name } מסתמך עליו, ולכן שום דבר לא הותקן: { $refusal }
       *[registry] { $path } אינו קובץ שחבילה זו רשאית להחליף, וערך הרישום { $name } מסתמך עליו, ולכן שום דבר לא הותקן: { $refusal }
    }
core-warning-restore-destination-exists = היעד קיים; כדי להחליף אותו, יש להשתמש באפשרות --force.
# { $path } is a file. { $error } is what went wrong.
core-warning-restore-file-not-written = לא ניתן היה לכתוב את { $path }: { $error }
core-warning-restore-not-a-folder = במקום שבו צריכה להיות תיקיה ריקה זו נמצא משהו שאינו תיקיה.
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-not-looked-at = לא ניתן היה לבדוק את { $path }: { $error }
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-folder-not-made = לא ניתן היה ליצור את { $path }: { $error }
# Why part of the app was left out, after "was left out:".
core-warning-restore-registration-exists = פריט זה כבר קיים במחשב זה, עם הגדרה אחרת, והתקנה אינה משתלטת עליו
core-warning-restore-registry-review = ערכי הרישום שוחזרו מהנתונים המובנים שבארכיון, ונתיבים ידועים של האפליקציה שוכתבו לשורש היעד.
# { $error } is what went wrong.
core-warning-restore-firewall-unreadable = לא ניתן היה לקרוא את הכללים של חומת האש של Windows: { $error }
# Why a firewall rule was left out, after "was left out:".
core-warning-restore-firewall-rule-exists = כלל בשם זה עבור תוכנית זו כבר קיים, עם הגדרות אחרות, והתקנה אינה משנה אותו
# { $name } is a runtime.
core-warning-restore-prerequisite-unknown = { $name }: אין תוכנית התקנה ידועה עבור רכיב זמן ריצה זה.
# { $name } is a runtime.
core-warning-restore-prerequisite-needs-admin = התקנת { $name } דורשת הרשאות מוגברות; יש להפעיל שוב את תוכנית ההתקנה כמנהל מערכת.
# { $name } is a runtime.
core-warning-restore-prerequisite-reboot = { $name } הותקן, אך נדרשת הפעלה מחדש כדי להשלים את ההתקנה.
# { $error } is what went wrong.
core-warning-restore-skipped-not-recorded = לא ניתן היה לרשום רכיב זמן ריצה שדולג: { $error }
# { $name } is a runtime. { $reason } is why it was skipped, already in words.
core-warning-restore-prerequisite-skipped = { $name }: { $reason }
# { $name } is a runtime. { $reason } is why it could not be installed. { $url } is a web address to get it from.
core-warning-restore-prerequisite-failed = לא ניתן היה להתקין את { $name }: { $reason }. ניתן להשיג אותו מ-{ $url }.
# { $error } is what went wrong.
core-warning-restore-runtime-not-recorded = לא ניתן היה לרשום רכיב זמן ריצה שהותקן: { $error }
# { $error } is what went wrong.
core-warning-restore-license-source-invalid = נתיב הרישיון בחבילה אינו חוקי: { $error }
core-warning-restore-license-needs-admin = שחזור נתוני רישיון למיקום מוגן דורש הרשאות מוגברות.
core-warning-restore-license-exists = יעד הרישיון קיים; כדי להחליף אותו, יש להשתמש באפשרות --force.
# { $error } is what went wrong.
core-warning-restore-license-failed = לא ניתן היה לשחזר את נתוני הרישיון: { $error }
core-warning-restore-driver-needs-admin = רישום מנהל ההתקן דורש הרשאות מוגברות; יש להפעיל שוב את תוכנית ההתקנה כמנהל מערכת.
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-warning-restore-task-runs-as-system = המשימה המתוזמנת { $name } פועלת בתור SYSTEM, עם הרשאות מלאות במחשב זה.
core-warning-restore-shortcut-exists = קיצור הדרך קיים; כדי להחליף אותו, יש להשתמש באפשרות --force.
core-warning-restore-shortcut-all-users = קיצור דרך משוחזר אחד או יותר מפנים לשולחן העבודה הציבורי או למיקום משותף בתפריט התחלה.
core-warning-restore-environment-refresh = ייתכן שכדי לראות את השינויים במשתני הסביבה, יידרשו תהליך חדש, מעטפת חדשה או כניסה מחדש.
# { $name } is a service's name. { $account } is the account it ran as on the PC it came from.
core-warning-restore-service-account = השירות { $name } פעל במחשב המקור בתור { $account }. הסיסמה שלו אינה בתוכנית ההתקנה, ולכן השירות מוגדר לפעול בתור LocalService. אם הוא אינו פועל כראוי, יש להגדיר את החשבון והסיסמה שלו ב"שירותים".
# { $name } is a runtime.
core-warning-restore-prerequisite-pending-reboot = { $name } עדיין לא רשום; הוא אמור להיות פעיל לאחר הפעלה מחדש.
# { $name } is a runtime.
core-warning-restore-prerequisite-not-verified = { $name } דיווח על הצלחה אך לא נרשם; ייתכן שההתקנה נכשלה.
# { $error } is what went wrong.
core-warning-restore-driver-source-invalid = נתיב מנהל ההתקן המצורף אינו חוקי: { $error }
# { $error } is what went wrong.
core-warning-restore-driver-not-read = לא ניתן היה לקרוא את ההגדרות הנוכחיות של מנהל ההתקן: { $error }
core-warning-restore-driver-exists = שירות מנהל ההתקן קיים; כדי להגדיר אותו מחדש, יש להשתמש באפשרות --force.
core-warning-restore-driver-image-exists = תמונת מנהל ההתקן קיימת; כדי להחליף אותה, יש להשתמש באפשרות --force.
core-warning-restore-driver-hash-bundled = תמונת מנהל ההתקן המצורפת אינה תואמת לגיבוב שנרשם עבורה, ולכן היא לא הותקנה.
# { $error } is what went wrong.
core-warning-restore-driver-not-placed = לא ניתן היה להציב את תמונת מנהל ההתקן: { $error }
core-warning-restore-driver-hash-placed = תמונת מנהל ההתקן השתנתה לאחר שהוצבה, ולכן היא לא נרשמה.
# { $error } is what went wrong.
core-warning-restore-key-not-created = לא ניתן היה ליצור את מפתח הרישום: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-read = לא ניתן היה לקרוא את הערך שפעולה זו תחליף: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-written = לא ניתן היה לכתוב את ערך הרישום: { $error }
core-warning-restore-value-exists = ערך הרישום קיים; כדי להחליף אותו, יש להשתמש באפשרות --force.
# { $error } is what went wrong.
core-warning-restore-environment-key-not-created = לא ניתן היה ליצור את מפתח הרישום של הסביבה: { $error }
core-warning-restore-variable-exists = משתנה הסביבה קיים; כדי להחליף אותו, יש להשתמש באפשרות --force.
# { $error } is what went wrong.
core-warning-restore-variable-not-written = לא ניתן היה לכתוב את משתנה הסביבה: { $error }
core-warning-restore-value-kind-missing = סוג ערך הרישום חסר.
# { $error } is what went wrong.
core-warning-restore-service-not-read = לא ניתן היה לקרוא את ההגדרות הנוכחיות של השירות: { $error }
core-warning-restore-service-exists = השירות קיים; כדי להגדיר אותו מחדש, יש להשתמש באפשרות --force.
# { $error } is what went wrong.
core-warning-restore-task-not-queried = השאילתה על המשימה המתוזמנת נכשלה: { $error }
core-warning-restore-task-exists = המשימה המתוזמנת קיימת; כדי להחליף אותה, יש להשתמש באפשרות --force.
# { $error } is what went wrong.
core-warning-restore-task-folder-not-looked-at = לא ניתן היה לבדוק את התיקיה של המשימה: { $error }
# { $error } is what went wrong.
core-warning-restore-shortcut-not-put-back = לא ניתן היה ליצור את קיצור הדרך, וגם לא ניתן היה להחזיר את קיצור הדרך שהוא החליף: { $error }
# { $error } is what went wrong.
core-warning-restore-command-not-started = הפעלת הפקודה נכשלה: { $error }
# { $output } is what the command wrote, as it wrote it.
core-warning-restore-command-failed = הפקודה נכשלה: { $output }
# { $error } is what went wrong.
core-warning-restore-powershell-not-started = הפעלת PowerShell נכשלה: { $error }
