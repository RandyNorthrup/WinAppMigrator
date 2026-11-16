# Words the core writes for people: warnings, refusals, errors, and the installer's list of what it sets up.
# Message names start with core-.


## Warnings a package records when it is built, which an installer shows later, maybe on another PC, in that PC's
## language.

# { $count } is how many shortcuts.
core-warning-shortcuts-unreadable =
    { $count ->
        [one] קיצור דרך אחד בכונן זה אינו מציין קובץ, ולכן לא ניתן היה לקבוע אם הוא פותח אפליקציה זו.
        [two] שני קיצורי דרך בכונן זה אינם מציינים קובץ, ולכן לא ניתן היה לקבוע אם הם פותחים אפליקציה זו.
       *[other] { $count } קיצורי דרך בכונן זה אינם מציינים קובץ, ולכן לא ניתן היה לקבוע אם הם פותחים אפליקציה זו.
    }

## Undoing an install: what an uninstall, or a failed install, says it put back, left, or could not undo. Each line is
## shown on its own, in a list.

# { $app_name } is the app's name.
core-rollback-record-damaged = לא ניתן היה לקרוא חלק מהרשומה של { $app_name }, ולכן כל מה שהחלק הזה תיאר עדיין נמצא במקומו, וייתכן שקובץ שההתקנה החליפה לא יחזור.
core-rollback-settings-entry-stays = הערך של האפליקציה בהגדרות נשאר, כדי שניתן יהיה להפעיל פעולה זו שוב משם.
# { $name } is a runtime's name, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-rollback-prerequisite-stays = { $name } הותקן, או ייתכן שהותקן, והוא נשאר, מכיוון שייתכן שתוכניות אחרות משתמשות בו.
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-drive-not-connected = מה שהתקנה זו הציבה ב-{ $folder } נמצא בכונן שאינו מחובר. יש לחבר אותו, ואז ניתן יהיה להשלים פעולה זו.
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-record-part-unread = מה שהתקנה זו הציבה ב-{ $folder } נשאר, מכיוון שלא ניתן היה לקרוא את החלק הזה של הרשומה.

## Why a file or folder was left, shown after "was left:" in the messages below them. Each is part of a sentence, so
## starts with a small letter and has no full stop.

# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-being-changed = משהו שינה את { $folder } בזמן שפעולה זו רצה, ולכן לא בוצע בה דבר נוסף. יש לסגור את מה שעושה זאת, ולהפעיל פעולה זו שוב
# { $folder } is a folder; { $error } is what Windows said went wrong.
core-rollback-reason-not-opened = לא ניתן היה לפתוח את { $folder }: { $error }
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-no-record = אין תיעוד של המיקום שבו הייתה התיקיה { $folder }
# { $folder } is a folder; { $actual } is the folder it now leads to.
core-rollback-reason-moved = התיקיה { $folder } מובילה כעת אל { $actual }, ולא למקום שבו הייתה במהלך ההתקנה
# Why a service was not stopped.
core-rollback-reason-did-not-stop = הוא לא נעצר בזמן

## Files and folders

# { $path } is a folder; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-folder-left = התיקיה { $path } נשארה: { $reason }.
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-removed = הוסרה התיקיה { $path }
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-holds-others = התיקיה { $path } נשארת, מכיוון שהיא מכילה קבצים שהתקנה זו לא הציבה שם.
# { $path } is a folder; { $error } is what Windows said went wrong.
core-rollback-folder-not-removed = לא ניתן היה להסיר את התיקיה { $path }: { $error }.
# { $path } is a file; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-file-left = הקובץ { $path } נשאר: { $reason }.
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-folder-gone = { $path } אינו מוחזר, מכיוון שהתיקיה שבה הוא היה כבר אינה קיימת.
# { $path } is a file; { $error } is what went wrong.
core-rollback-partial-not-removed = לא ניתן היה להסיר קובץ חלקי: { $path } ({ $error }).
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-checked = לא ניתן היה לבדוק את { $path }: { $error }.
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-removed = הוסר { $path }
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-removed = לא ניתן היה להסיר את { $path }: { $error }.
# { $path } is a file this install had replaced, and has put back.
core-rollback-file-put-back = הוחזר { $path }
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-put-back = לא ניתן היה להחזיר את { $path }: { $error }.
# Why a file could not be put back.
core-rollback-backup-gone = הגיבוי כבר אינו קיים
# { $path } is a file.
core-rollback-no-parent-folder = ל-{ $path } אין תיקיית אב

## A file that stays. { $why } says why: still-used (something that stays still uses it), changed (it has changed since
## it was installed), or not-asked (it is someone's data, and they did not ask for it to go). { $path } is the file.

core-rollback-file-stays =
    { $why ->
        [still-used] { $path } נשאר, מכיוון שמשהו שנשאר עדיין משתמש בו.
        [changed] { $path } נשאר, מכיוון שהוא השתנה מאז שהותקן.
       *[not-asked] { $path } נשאר, מכיוון שלא התבקשה הסרתו.
    }
# The file it replaced was to be put beside it, under another name, and every name was taken.
core-rollback-file-stays-no-name-beside =
    { $why ->
        [still-used] { $path } נשאר, מכיוון שמשהו שנשאר עדיין משתמש בו, ולא ניתן היה להציב לצידו את הקובץ שהוא החליף: כל השמות האפשריים עבורו תפוסים.
        [changed] { $path } נשאר, מכיוון שהוא השתנה מאז שהותקן, ולא ניתן היה להציב לצידו את הקובץ שהוא החליף: כל השמות האפשריים עבורו תפוסים.
       *[not-asked] { $path } נשאר, מכיוון שלא התבקשה הסרתו, ולא ניתן היה להציב לצידו את הקובץ שהוא החליף: כל השמות האפשריים עבורו תפוסים.
    }
# { $name } is the name the file it replaced now has, such as settings.ini.before-install.
core-rollback-file-stays-beside =
    { $why ->
        [still-used] { $path } נשאר, מכיוון שמשהו שנשאר עדיין משתמש בו. הקובץ שהוא החליף נמצא לצידו, בשם { $name }.
        [changed] { $path } נשאר, מכיוון שהוא השתנה מאז שהותקן. הקובץ שהוא החליף נמצא לצידו, בשם { $name }.
       *[not-asked] { $path } נשאר, מכיוון שלא התבקשה הסרתו. הקובץ שהוא החליף נמצא לצידו, בשם { $name }.
    }
# { $error } is what went wrong.
core-rollback-file-stays-not-beside =
    { $why ->
        [still-used] { $path } נשאר, מכיוון שמשהו שנשאר עדיין משתמש בו, ולא ניתן היה להציב לצידו את הקובץ שהוא החליף: { $error }.
        [changed] { $path } נשאר, מכיוון שהוא השתנה מאז שהותקן, ולא ניתן היה להציב לצידו את הקובץ שהוא החליף: { $error }.
       *[not-asked] { $path } נשאר, מכיוון שלא התבקשה הסרתו, ולא ניתן היה להציב לצידו את הקובץ שהוא החליף: { $error }.
    }

## The registry and environment variables. { $kind } says what { $name } is: key (a registry key, such as
## HKLM\Software\App), value (a registry value, such as HKLM\Software\App\Version), user-variable or machine-variable
## (an environment variable of the user's or of the whole PC, such as APP_HOME), or user-list or machine-list (a
## variable that is a list of folders, such as Path). { $error } is what Windows said went wrong.

# How a registry key's unnamed value is shown, as Registry Editor shows it.
core-registry-default-value-name = (ברירת מחדל)
core-rollback-setting-other-account =
    { $kind ->
        [key] המפתח { $name } נשאר, מכיוון שהוא שייך לחשבון שהתקין זאת, שאינו מחובר.
        [user-variable] משתנה הסביבה של המשתמש { $name } נשאר, מכיוון שהוא שייך לחשבון שהתקין זאת, שאינו מחובר.
        [machine-variable] משתנה הסביבה של המחשב { $name } נשאר, מכיוון שהוא שייך לחשבון שהתקין זאת, שאינו מחובר.
        [user-list] המשתנה { $name } של המשתמש נשאר, מכיוון שהוא שייך לחשבון שהתקין זאת, שאינו מחובר.
        [machine-list] המשתנה { $name } של המחשב נשאר, מכיוון שהוא שייך לחשבון שהתקין זאת, שאינו מחובר.
       *[value] ערך הרישום { $name } נשאר, מכיוון שהוא שייך לחשבון שהתקין זאת, שאינו מחובר.
    }
core-rollback-setting-not-reached =
    { $kind ->
        [key] לא ניתן היה לגשת אל המפתח { $name }: { $error }.
        [user-variable] לא ניתן היה לגשת אל משתנה הסביבה של המשתמש { $name }: { $error }.
        [machine-variable] לא ניתן היה לגשת אל משתנה הסביבה של המחשב { $name }: { $error }.
        [user-list] לא ניתן היה לגשת אל המשתנה { $name } של המשתמש: { $error }.
        [machine-list] לא ניתן היה לגשת אל המשתנה { $name } של המחשב: { $error }.
       *[value] לא ניתן היה לגשת אל ערך הרישום { $name }: { $error }.
    }
core-rollback-setting-not-read =
    { $kind ->
        [key] לא ניתן היה לקרוא את המפתח { $name }: { $error }.
        [user-variable] לא ניתן היה לקרוא את משתנה הסביבה של המשתמש { $name }: { $error }.
        [machine-variable] לא ניתן היה לקרוא את משתנה הסביבה של המחשב { $name }: { $error }.
        [user-list] לא ניתן היה לקרוא את המשתנה { $name } של המשתמש: { $error }.
        [machine-list] לא ניתן היה לקרוא את המשתנה { $name } של המחשב: { $error }.
       *[value] לא ניתן היה לקרוא את ערך הרישום { $name }: { $error }.
    }
core-rollback-setting-changed =
    { $kind ->
        [user-variable] משתנה הסביבה של המשתמש { $name } נשאר, מכיוון שהוא השתנה מאז שהותקן.
        [machine-variable] משתנה הסביבה של המחשב { $name } נשאר, מכיוון שהוא השתנה מאז שהותקן.
        [user-list] המשתנה { $name } של המשתמש נשאר, מכיוון שהוא השתנה מאז שהותקן.
        [machine-list] המשתנה { $name } של המחשב נשאר, מכיוון שהוא השתנה מאז שהותקן.
       *[value] ערך הרישום { $name } נשאר, מכיוון שהוא השתנה מאז שהותקן.
    }
# Said when it is back as it was before the install.
core-rollback-setting-put-back =
    { $kind ->
        [user-variable] הוחזר משתנה הסביבה של המשתמש { $name }
        [machine-variable] הוחזר משתנה הסביבה של המחשב { $name }
        [user-list] הוחזר המשתנה { $name } של המשתמש
        [machine-list] הוחזר המשתנה { $name } של המחשב
       *[value] הוחזר ערך הרישום { $name }
    }
core-rollback-setting-not-put-back =
    { $kind ->
        [user-variable] לא ניתן היה להחזיר את משתנה הסביבה של המשתמש { $name }: { $error }.
        [machine-variable] לא ניתן היה להחזיר את משתנה הסביבה של המחשב { $name }: { $error }.
        [user-list] לא ניתן היה להחזיר את המשתנה { $name } של המשתמש: { $error }.
        [machine-list] לא ניתן היה להחזיר את המשתנה { $name } של המחשב: { $error }.
       *[value] לא ניתן היה להחזיר את ערך הרישום { $name }: { $error }.
    }
# A registry value the install wrote, which is the user's data, and stays unless they asked for it to go.
core-rollback-value-kept-by-request = ערך הרישום { $name } נשאר, מכיוון שלא התבקשה הסרתו.
core-rollback-key-holds-others = המפתח { $name } נשאר, מכיוון שהוא מכיל פריטים שהתקנה זו לא הציבה שם.
core-rollback-key-removed = הוסר המפתח { $name }
core-rollback-key-not-removed = לא ניתן היה להסיר את המפתח { $name }: { $error }.
# { $kind } is user-list or machine-list.
core-rollback-list-entries-removed =
    { $kind ->
        [user-list] הערכים של התקנה זו הוצאו מהמשתנה { $name } של המשתמש
       *[machine-list] הערכים של התקנה זו הוצאו מהמשתנה { $name } של המחשב
    }
core-rollback-list-entries-not-removed =
    { $kind ->
        [user-list] לא ניתן היה להוציא את הערכים של התקנה זו מהמשתנה { $name } של המשתמש: { $error }.
       *[machine-list] לא ניתן היה להוציא את הערכים של התקנה זו מהמשתנה { $name } של המחשב: { $error }.
    }

## Services and drivers. { $kind } is service or driver; { $name } is its name, such as AppUpdater. { $error } and
## { $reason } are what Windows said went wrong.

core-rollback-service-not-read =
    { $kind ->
        [driver] לא ניתן היה לקרוא את מנהל ההתקן { $name }: { $error }.
       *[service] לא ניתן היה לקרוא את השירות { $name }: { $error }.
    }
core-rollback-service-changed =
    { $kind ->
        [driver] מנהל ההתקן { $name } נשאר, מכיוון שהוא השתנה מאז שהותקן.
       *[service] השירות { $name } נשאר, מכיוון שהוא השתנה מאז שהותקן.
    }
core-rollback-service-not-stopped =
    { $kind ->
        [driver] לא ניתן היה לעצור את מנהל ההתקן { $name } ({ $reason }), ולכן ייתכן שמה שהוא משתמש בו יישאר עד להפעלה מחדש של המחשב.
       *[service] לא ניתן היה לעצור את השירות { $name } ({ $reason }), ולכן ייתכן שמה שהוא משתמש בו יישאר עד להפעלה מחדש של המחשב.
    }
core-rollback-service-stopped-for-this =
    { $kind ->
        [driver] מנהל ההתקן { $name } נעצר לצורך פעולה זו. יש להפעיל אותו שוב כשיהיה בו צורך.
       *[service] השירות { $name } נעצר לצורך פעולה זו. יש להפעיל אותו שוב כשיהיה בו צורך.
    }
core-rollback-service-not-removed =
    { $kind ->
        [driver] לא ניתן היה להסיר את מנהל ההתקן { $name }: { $reason }.
       *[service] לא ניתן היה להסיר את השירות { $name }: { $reason }.
    }
core-rollback-service-marked-for-removal =
    { $kind ->
        [driver] מנהל ההתקן { $name } סומן להסרה, והוא יוסר כשהמחשב יופעל מחדש או כשמה שמחזיק אותו פתוח ישחרר אותו.
       *[service] השירות { $name } סומן להסרה, והוא יוסר כשהמחשב יופעל מחדש או כשמה שמחזיק אותו פתוח ישחרר אותו.
    }
core-rollback-service-removed =
    { $kind ->
        [driver] הוסר מנהל ההתקן { $name }
       *[service] הוסר השירות { $name }
    }
core-rollback-service-not-restored =
    { $kind ->
        [driver] לא ניתן היה להחזיר את מנהל ההתקן { $name } למצבו הקודם: { $reason }.
       *[service] לא ניתן היה להחזיר את השירות { $name } למצבו הקודם: { $reason }.
    }
core-rollback-service-restored =
    { $kind ->
        [driver] הוחזרו ההגדרות של מנהל ההתקן { $name }
       *[service] הוחזרו ההגדרות של השירות { $name }
    }
core-rollback-service-not-restarted = השירות { $name } פעל לפני התקנה זו, ולא הופעל שוב: { $reason }.
# { $account } is the account the service ran as, such as CONTOSO\backup.
core-rollback-service-account-again = השירות { $name } פעל בתור { $account }. הסיסמה שלו אינה נשמרת, ולכן יש להגדיר שוב את החשבון שלו ב"שירותים".

## Firewall rules. { $name } is the rule's name; { $error } is what went wrong.

core-rollback-firewall-not-read = לא ניתן היה לקרוא את כלל חומת האש { $name }: { $error }.
core-rollback-firewall-says-otherwise = כלל חומת האש { $name } נשאר, מכיוון שהכלל הקיים בשם זה מציין משהו אחר.
core-rollback-firewall-changed = כלל חומת האש { $name } נשאר, מכיוון שהוא השתנה מאז שהותקן.
# The capital "The" is as the app has always said it.
core-rollback-firewall-removed = הוסר כלל חומת האש { $name }
core-rollback-firewall-not-removed = לא ניתן היה להסיר את כלל חומת האש { $name }: { $error }.

## Scheduled tasks. { $path } is a folder of tasks, and { $name } a task with its folder, such as \App\Nightly Update.
## { $error } and { $reason } are what went wrong.

core-rollback-task-folder-holds-others = תיקיית המשימות { $path } נשארת, מכיוון שהיא מכילה משימות שהתקנה זו לא הציבה שם.
core-rollback-task-folder-not-removed = לא ניתן היה להסיר את תיקיית המשימות { $path }: { $error }.
core-rollback-task-not-read = לא ניתן היה לקרוא את המשימה { $name }: { $error }.
core-rollback-task-changed = המשימה { $name } נשארת, מכיוון שהיא השתנתה מאז שהותקנה.
core-rollback-task-removed = הוסרה המשימה { $name }
core-rollback-task-not-removed = לא ניתן היה להסיר את המשימה { $name }: { $reason }.
core-rollback-task-put-back = הוחזרה המשימה { $name }
core-rollback-task-not-put-back = לא ניתן היה להחזיר את המשימה { $name }: { $reason }.

## When an install cannot be undone at all

# { $error } is what went wrong.
core-rollback-record-unreadable = לא ניתן היה לקרוא את רשומת ההתקנה, ולכן שום דבר לא בוטל: { $error }.
# { $error } is what went wrong.
core-rollback-nothing-undone = שום דבר לא בוטל: { $error }.
# One app's part of the message below. { $app_name } is the app's name; { $failures } is what could not be undone, a
# sentence or more; { $folder } is the folder that holds the install's record.
core-rollback-earlier-install-stuck = { $app_name }: { $failures } אם לעולם לא ניתן יהיה לבטל אותה, מנהל מערכת יכול לוותר עליה על ידי הסרת { $folder }.
# Why an install did not start. { $details } is one or more of the message above.
core-rollback-earlier-installs-not-undone = לא ניתן היה לבטל במלואה התקנה קודמת שלא הסתיימה, ולכן שום דבר לא הותקן. יש לסגור את מה שמשתמש בפריטים אלה ולנסות שוב. { $details }
core-rollback-install-busy = התקנה זו משתנה כעת על ידי תוכנית התקנה או תוכנית הסרת התקנה אחרת

## The summary an uninstall writes on the command line, one line each, with what each says below it. { $count } is how
## many.

core-rollback-count-undone = בוטלו: { $count }
core-rollback-count-failed = לא ניתן היה לבטל: { $count }
# What was left on purpose: it has changed since the install, or it is the user's.
core-rollback-count-kept = נשארו במקומם: { $count }
# What no uninstall can take back, such as a runtime that was installed, which other programs may use.
core-rollback-count-left-behind = נותרו מאחור: { $count }

## Which Windows a PC or drive has, as the app shows it beside the drive.

# { $name } is the Windows' name, such as Windows 7 Professional Service Pack 1.
core-system-windows-32-bit = { $name }, 32 סיביות
# { $name } is the Windows' name, such as Windows 11 Pro.
core-system-windows-64-bit = { $name }, 64 סיביות
# A drive's Windows, when it only says it is older than Vista.
core-system-windows-xp-or-older = לכל היותר Windows XP

## Reading a connected drive, or another account's settings. Each is why it could not be done, and some start with a
## small letter, as they follow other words.

core-system-needs-administrator = קריאה של כונן מחובר או של הגדרות של חשבון אחר דורשת הרשאות מנהל מערכת
core-system-no-program-data = לא ניתן היה למצוא את התיקיה ProgramData של מחשב זה
# { $path } is a file, such as D:\Windows\System32\config\SOFTWARE.
core-system-not-a-hive-file = אינו קובץ כוורת: { $path }
# { $path } is a file; { $size } is its size in bytes.
core-system-hive-too-large = הגודל של { $path } הוא { $size } בתים, גדול מדי עבור כוורת רישום, או יותר ממה שכל הכוורות של כונן זה יכולות להגיע אליו יחד
# { $key } is a registry key, such as Software\Microsoft\Windows\CurrentVersion.
core-system-hive-no-mapping = לכוורת הלא מקוונת אין מיפוי עבור HKLM\{ $key }
# { $key } is where a drive's registry was to be loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said.
# "reg load" is a command's name.
core-system-hive-not-loaded = הפקודה reg load { $key } נכשלה: { $error }
# { $key } is where a drive's registry was loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said. "reg
# unload" is a command's name.
core-system-hive-not-unloaded = הפקודה reg unload { $key } נכשלה: { $error }
# { $drive } is a drive, such as D:\.
core-system-not-a-system-drive = { $drive } אינו כונן מערכת של Windows (אין בו תיקיה \Windows).
# { $drive } is a drive, such as C:\. "This PC" is the name of a choice in the app.
core-system-own-system-drive = { $drive } הוא כונן המערכת של מחשב זה. כדי לקרוא את מחשב זה, יש לבחור "מחשב זה".
# { $drive } is a drive, such as D:\.
core-system-registry-through-link = { $drive } אינו כונן מערכת של Windows: הדרך לרישום שלו עוברת דרך קישור.

## Warnings a scan records about whose data it read.

# { $account } is an account's name or id, such as alice or S-1-5-21-...-1001.
core-warning-scan-account-not-on-record = החשבון { $account } אינו רשום כאן, ולכן לא נכללו נתונים או הגדרות לכל משתמש של אף אחד.
# { $account } is an account's name, such as CONTOSO\alice.
core-warning-scan-signed-in-account-no-profile = לחשבון המחובר כאן, { $account }, אין פרופיל רשום, ולכן לא נכללו נתונים או הגדרות לכל משתמש של אף אחד.
# { $reason } is why Windows could not say.
core-warning-scan-signed-in-account-unknown = לא ניתן היה לקבוע מי מחובר ({ $reason }), ולכן לא נכללו נתונים או הגדרות לכל משתמש של אף אחד. יש לבחור של מי הנתונים שייכללו.
core-warning-scan-last-account-unknown = כונן זה אינו מציין מי נכנס אחרון, ויש בו יותר מחשבון אחד, ולכן לא נכללו נתונים או הגדרות לכל משתמש של אף אחד. יש לבחור של מי הנתונים שייכללו.
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-settings-unreadable = לא ניתן היה לקרוא את ההגדרות של { $account }, ולכן הן לא נכללו: { $error }
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-classes-unreadable = לא ניתן היה לקרוא את מחלקות ה-COM ואת סוגי הקבצים שרשומים עבור { $account }, ולכן הם לא נכללו: { $error }

## Paths a scan did not follow. { $count } is how many; { $paths } is some of them, separated by semicolons; { $more } is
## how many more there are, not shown.

core-warning-scan-drive-paths-not-followed =
    { $count ->
        [one] לא ניתן היה לקרוא נתיב שרשום בכונן זה, ולכן שום דבר משם לא נכלל. הוא נמצא בדיסק אחר, אינו נתיב מלא, או עובר דרך קישור שיוביל אל מחוץ לכונן: { $paths }
        [two] לא ניתן היה לקרוא כמה נתיבים שרשומים בכונן זה, ולכן שום דבר משם לא נכלל. כל אחד מהם נמצא בדיסק אחר, אינו נתיב מלא, או עובר דרך קישור שיוביל אל מחוץ לכונן: { $paths }
       *[other] לא ניתן היה לקרוא כמה נתיבים שרשומים בכונן זה, ולכן שום דבר משם לא נכלל. כל אחד מהם נמצא בדיסק אחר, אינו נתיב מלא, או עובר דרך קישור שיוביל אל מחוץ לכונן: { $paths }
    }
core-warning-scan-drive-paths-not-followed-more = לא ניתן היה לקרוא כמה נתיבים שרשומים בכונן זה, ולכן שום דבר משם לא נכלל. כל אחד מהם נמצא בדיסק אחר, אינו נתיב מלא, או עובר דרך קישור שיוביל אל מחוץ לכונן: { $paths } ועוד { $more }
core-warning-scan-network-paths-not-followed =
    { $count ->
        [one] לא ניתן היה לקרוא נתיב שרשום במחשב זה, ולכן שום דבר משם לא נכלל. הוא אינו נמצא בדיסק של מחשב זה, ובדיקה שלו הייתה יוצרת אליו חיבור, או שיש לו שם שיפתח משהו אחר: { $paths }
        [two] לא ניתן היה לקרוא כמה נתיבים שרשומים במחשב זה, ולכן שום דבר משם לא נכלל. כל אחד מהם אינו נמצא בדיסק של מחשב זה, ובדיקה שלו הייתה יוצרת אליו חיבור, או שיש לו שם שיפתח משהו אחר: { $paths }
       *[other] לא ניתן היה לקרוא כמה נתיבים שרשומים במחשב זה, ולכן שום דבר משם לא נכלל. כל אחד מהם אינו נמצא בדיסק של מחשב זה, ובדיקה שלו הייתה יוצרת אליו חיבור, או שיש לו שם שיפתח משהו אחר: { $paths }
    }
core-warning-scan-network-paths-not-followed-more = לא ניתן היה לקרוא כמה נתיבים שרשומים במחשב זה, ולכן שום דבר משם לא נכלל. כל אחד מהם אינו נמצא בדיסק של מחשב זה, ובדיקה שלו הייתה יוצרת אליו חיבור, או שיש לו שם שיפתח משהו אחר: { $paths } ועוד { $more }

## What a scan with none of an account's data leaves out of a package, in the app's list of what was left out. Each
## has a name, and below it why.

core-account-left-out-shortcuts = קיצורי דרך בתיקיות של חשבון זה עצמו
# { $count } is how many shortcuts.
core-account-left-out-shortcuts-why =
    { $count ->
        [one] קיצור דרך אחד, שנכלל רק עם האפשרות "נתוני משתמש"
        [two] שני קיצורי דרך, שנכללים רק עם האפשרות "נתוני משתמש"
       *[other] { $count } קיצורי דרך, שנכללים רק עם האפשרות "נתוני משתמש"
    }
core-account-left-out-variables = משתני סביבה של חשבון זה
# { $count } is how many variables.
core-account-left-out-variables-why =
    { $count ->
        [one] משתנה אחד, שנכלל רק עם האפשרות "נתוני משתמש"
        [two] שני משתנים, שנכללים רק עם האפשרות "נתוני משתמש"
       *[other] { $count } משתנים, שנכללים רק עם האפשרות "נתוני משתמש"
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-account-left-out-task = המשימה המתוזמנת { $task }
core-account-task-works-on-account-folder = היא פועלת על מיקום בתיקיות של חשבון, או נקראת על שם מיקום כזה, ונכללת רק עם האפשרות "נתוני משתמש"
core-account-task-runs-from-account-folder = מה שהיא מריצה בנוסף מתוך התיקיות של חשבון נכלל רק עם האפשרות "נתוני משתמש"
# { $service } is a service's name, such as AppUpdater.
core-account-left-out-service = השירות { $service }
core-account-service-works-on-account-folder = הוא פועל על מיקום בתיקיות של חשבון, ונכלל רק עם האפשרות "נתוני משתמש"

## What a package was found to still say of whose PC it came from, after a build with none of an account's data.

# The parts of a package, as the warning below names them.
core-account-part-files = הנתיבים של הקבצים של האפליקציה עצמה
core-account-part-registry = ערכי רישום
core-account-part-settings-entry = הערך של האפליקציה בהגדרות
core-account-part-main-program = המיקום של התוכנית הראשית שלה
core-account-part-shortcuts = קיצורי דרך
core-account-part-tasks = משימות מתוזמנות
core-account-part-services = שירותים
core-account-part-variables = משתני סביבה
# One place in the list below. { $place } is one of the parts above, or a file's name; { $count } is how many times.
core-account-mention-place = { $place } ({ $count })
# { $kind } is what was found: account (an account's name), folder (where an account's folder is), computer (the PC's
# name), or id (an account's id). { $places } is a list of where, from the message above.
core-warning-scan-package-mentions =
    { $kind ->
        [account] חבילה זו מציינת שם של חשבון במקומות הבאים: { $places }. היא נבנתה ללא נתונים של אף חשבון, וכל מי שיש בידיו את תוכנית ההתקנה יכול לקרוא זאת.
        [folder] חבילה זו מציינת את המיקום של תיקיה של חשבון במקומות הבאים: { $places }. היא נבנתה ללא נתונים של אף חשבון, וכל מי שיש בידיו את תוכנית ההתקנה יכול לקרוא זאת.
        [computer] חבילה זו מציינת את שם המחשב במקומות הבאים: { $places }. היא נבנתה ללא נתונים של אף חשבון, וכל מי שיש בידיו את תוכנית ההתקנה יכול לקרוא זאת.
       *[id] חבילה זו מציינת מזהה של חשבון במקומות הבאים: { $places }. היא נבנתה ללא נתונים של אף חשבון, וכל מי שיש בידיו את תוכנית ההתקנה יכול לקרוא זאת.
    }
# { $count } is how many folders.
core-warning-scan-folders-of-others-unreadable =
    { $count ->
        [one] לא ניתן היה לקרוא תיקיה אחת מחוץ לאפליקציה, ולכן ייתכן שחסרים נתונים של האפליקציה שנמצאים בה.
        [two] לא ניתן היה לקרוא שתי תיקיות מחוץ לאפליקציה, ולכן ייתכן שחסרים נתונים של האפליקציה שנמצאים בהן.
       *[other] לא ניתן היה לקרוא { $count } תיקיות מחוץ לאפליקציה, ולכן ייתכן שחסרים נתונים של האפליקציה שנמצאים בהן.
    }

## What a scan leaves out because it looks like a credential, or is a license with the License option off, in the app's
## list of what was left out. Each has a name, and below it why.

core-secret-why-credential = הוא נראה כמו פרטי כניסה
core-secret-why-license-option-off = האפשרות "נתוני רישיון והפעלה" כבויה
# { $count } is how many values of a registry key.
core-secret-why-account-values =
    { $count ->
        [one] ערך אחד, שנכלל רק עם האפשרות "נתוני משתמש"
        [two] שני ערכים, שנכללים רק עם האפשרות "נתוני משתמש"
       *[other] { $count } ערכים, שנכללים רק עם האפשרות "נתוני משתמש"
    }
# Shown in place of a registry value's name when its name itself looks like a password or a key.
core-secret-name-holds-credential = (ערך ששמו מכיל פרטי כניסה)
# Shown in place of an option's name, for an argument on a command line that has no name.
core-secret-unnamed-argument = ארגומנט שמכיל פרטי כניסה
# { $option } is an option taken out of a command line, such as /password, or the message above. { $place } says
# where the command line was: service, task, shortcut, opener (what opens a file type), opener-unnamed (the same, of a
# type with no name), or uninstall. { $name } is the service, task, shortcut, file type, or app.
core-secret-taken-from =
    { $place ->
        [service] { $option } באופן שבו { $name } מופעל
        [task] { $option } באופן שבו המשימה { $name } מופעלת
        [shortcut] { $option } בקיצור הדרך { $name }
        [opener] { $option } במה שפותח את { $name }
        [opener-unnamed] { $option } במה שפותח סוג קובץ
       *[uninstall] { $option } באופן שבו מוסרת ההתקנה של { $name }
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-secret-task-definition-unreadable = ההגדרה של המשימה { $task }, שלא ניתן היה לקרוא כדי לבדוק אותה
# { $name } is an environment variable's name, such as APP_TOKEN.
core-secret-left-out-variable = משתנה הסביבה { $name }
# { $class } is a COM class's id, such as {00000000-0000-0000-0000-000000000000}.
core-secret-left-out-class-server = מה שהמחלקה { $class } מציינת שהיא מריצה
# { $rule } is a firewall rule's name.
core-secret-left-out-firewall-description = התיאור של כלל חומת האש { $rule }

## Licenses a scan found, as the package's report describes each.

# { $name } is a file's name, such as license.lic.
core-license-detail-file = זוהה קובץ רישיון: { $name }
# { $framework } is a licensing product's name, such as FlexLM / FlexNet.
core-license-detail-store = קיים מאגר רישיונות של { $framework }
# { $name } is an environment variable's name, such as LM_LICENSE_FILE.
core-license-detail-variable = משתנה סביבה של רישיון { $name }
# { $name } is a registry value's name, such as SerialNumber.
core-license-detail-registry-value = ערך רישום שמרמז על נתוני רישוי: { $name }
# Said of a registry value in an account's key, which is not named.
core-license-detail-registry-value-of-an-account = ערך רישום שמרמז על נתוני רישוי
# { $key } is a registry key, such as HKLM\Software\Contoso\License.
core-license-detail-registry-key = מפתח רישום שמרמז על נתוני רישוי: { $key }

## Warnings a scan records about licenses.

core-warning-scan-license-data = זוהו נתוני רישוי/הפעלה. כשהאפשרות "נתוני רישיון והפעלה" מופעלת, תוכנית ההתקנה משחזרת את מה שניתן להעביר; ערכים ברמת המחשב ומאגרים שקשורים לחומרה רק מדווחים. WinAppMigrator לעולם אינו מפענח הצפנה, עוקף הפעלה או מדמה מפתחות חומרה.
core-warning-scan-license-machine-bound = חלק מהרישיונות קשורים למחשב (מפתח חומרה, הפעלה שנעולה למחשב, או הצפנה). תוכנית ההתקנה משחזרת את מה שהיא יכולה; ייתכן שעדיין תידרש עבורם הפעלה חוזרת במחשב החדש.

## Warnings a scan records about the registry. What could not be read is not named in them: it is kept beside them.

# { $error } is what Windows said went wrong. App Paths is a registry key's name.
core-warning-scan-app-paths-key-unreadable = לא ניתן היה לקרוא מפתח משנה של App Paths: { $error }
core-warning-scan-registry-limit = לכידת הרישום הגיעה למגבלת הבטיחות שהוגדרה.
core-warning-scan-registry-subtree-limit = לכידת עץ המשנה של הרישום הגיעה למגבלת הבטיחות שהוגדרה.
# { $error } is what Windows said went wrong.
core-warning-scan-registry-key-unreadable = לא ניתן היה לקרוא את מפתח הרישום: { $error }
core-warning-scan-registry-value-too-large = ערך הרישום חרג ממגבלת הגודל ללכידה שהוגדרה.

## Scheduled tasks

core-warning-scan-tasks-folder-through-link = תיקיית המשימות המתוזמנות בכונן זה עוברת דרך קישור, ולכן המשימות שבה לא נקראו.
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-warning-scan-task-left-out = המשימה המתוזמנת { $task } הושמטה: היא פועלת בתור חשבון אחר מזה שההגדרות שלו נכללות, משתמשים שאינם מנהלי מערכת יכולים לשנות את הקובץ שלה, או שלאפליקציה יש יותר מדי משימות כדי לשמור את כולן.
# { $error } is what Windows said went wrong.
core-warning-scan-tasks-folder-unreadable = לא ניתן היה לקרוא חלק מהמשימות המתוזמנות: { $error }
# { $path } is a task's file, such as C:\Windows\System32\Tasks\App\Nightly Sync; { $reason } is one of the reasons
# below, or what Windows said went wrong.
core-warning-scan-task-unreadable = לא ניתן היה לקרוא את המשימה המתוזמנת { $path }: { $reason }. אם היא מפעילה את האפליקציה, היא לא נכללה.
# Why a task's file could not be read. { $size } is its size in bytes.
core-task-too-large = גודלו { $size } בתים, גדול מדי עבור משימה
# Why a task's file could not be read.
core-task-ends-mid-character = הוא מסתיים באמצע תו
# Why a task's file could not be read. { $error } says where.
core-task-not-utf16 = הוא אינו UTF-16 חוקי: { $error }
# Why a task's file could not be read. { $error } says where.
core-task-not-text = הוא אינו UTF-8 או UTF-16 חוקי: { $error }
# What a package records of how a task is put back. Register-ScheduledTask is a command's name.
core-task-restore-instructions = השחזור משתמש ב-Register-ScheduledTask עם הגדרת ה-XML שבארכיון.

## Reading a program's file. Each is why it could not be read, after the program's name.

# { $reason } is what went wrong.
core-pe-unreadable = לא ניתן היה לקרוא אותו: { $reason }
core-pe-headers-misaligned = הכותרות שלו אינן ממוקמות בגבולות שבהם כותרות של תוכנית חייבות להיות
# { $size } is the file's size in bytes.
core-pe-too-large = גודלו { $size } בתים, גדול מדי עבור תוכנית
# PE is the name of the format Windows programs are in.
core-pe-no-headers = אין לו כותרות PE

## Warnings a scan records about what a program needs. Which program is kept beside each.

# { $error } is what went wrong.
core-warning-scan-imports-unreadable = לא ניתן היה לקרוא את טבלת הייבוא: { $error }
# { $error } is what went wrong.
core-warning-scan-import-name-unreadable = לא ניתן היה לקרוא שם של מתאר ייבוא: { $error }
# { $count } is how many were skipped.
core-warning-scan-legacy-delay-imports =
    { $count ->
        [one] דולג על מתאר אחד של טעינה מושהית מדור קודם (v1); רק ייבוא מושהה מודרני מבוסס RVA מנותח.
        [two] דולגו שני מתארים של טעינה מושהית מדור קודם (v1); רק ייבוא מושהה מודרני מבוסס RVA מנותח.
       *[other] דולגו { $count } מתארים של טעינה מושהית מדור קודם (v1); רק ייבוא מושהה מודרני מבוסס RVA מנותח.
    }

## The DLLs a program needs. Which DLL or program is kept beside each warning.

# { $path } is a program, such as C:\Program Files\App\app.exe.
core-dll-program-has-no-folder = לקובץ ההפעלה אין תיקיית אב: { $path }
# Why a DLL is among the app's files, in the package's list of them.
core-dll-beside-program = DLL שנמצא לצד קובץ ההפעלה שנבחר
# { $error } is what went wrong.
core-warning-scan-dependency-unscanned = לא ניתן היה לסרוק תלות רקורסיבית: { $error }
# { $name } is a DLL's name, such as d3dx9_43.dll.
core-warning-scan-dependency-not-in-newer-windows = { $name } היה בתיקיה Windows של המחשב שבו הייתה אפליקציה זו, ו-Windows 10 ו-11 אינם כוללים אותו. משהו התקין אותו שם, ולכן הוא אינו חלק מהקבצים של האפליקציה עצמה ואינו מועבר. ייתכן שהאפליקציה לא תופעל עד שיותקן מה שמספק אותו.
# { $error } is what went wrong.
core-warning-scan-dependency-not-hashed = לא ניתן היה לחשב גיבוב לתלות: { $error }
# SystemRoot and windir are the names of environment variables.
core-warning-scan-windows-folder-unknown = משתני הסביבה SystemRoot/windir אינם זמינים; חיפוש קובצי DLL של המערכת מוגבל.
# { $error } is what went wrong.
core-warning-scan-dependency-architecture-unreadable = לא ניתן היה לקרוא את הארכיטקטורה של התלות: { $error }

## Services, drivers, shortcuts, and environment variables a scan reads. Where it was reading is kept beside each
## warning.

# { $error } is what Windows said went wrong.
core-warning-scan-services-unreadable = לא ניתן היה לקרוא את מפתח הרישום של השירותים: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-drivers-unreadable = לא ניתן היה לקרוא את מפתח הרישום של השירותים לצורך גילוי מנהלי התקנים: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-queried = השאילתה על קיצורי הדרך נכשלה: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-parsed = לא ניתן היה לנתח את הפלט של השאילתה על קיצורי הדרך: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-environment-unreadable = לא ניתן היה לקרוא את מפתח הרישום של סביבת המחשב: { $error }

## What a package records of how a driver is put back, which an installer shows. { $kind } is the driver's type as
## Windows names it (kernel, file-system, recognizer, or unknown-type); { $name } is its service's name; { $path } is
## its file. { $signed } is yes when { $signer } names who signed it, and no when that is not known.

core-service-driver-bundled =
    { $signed ->
        [yes] מנהל התקן מסוג { $kind } בבעלות האפליקציה, "{ $name }" ({ $path }, חתום על ידי { $signer }). תוכנית ההתקנה מצרפת את קובץ ה-.sys שלו ורושמת מחדש את שירות מנהל ההתקן באופן אוטומטי (sc create, דורש הרשאות מוגברות), ולאחר מכן מוודאת שהשירות רשום.
       *[no] מנהל התקן מסוג { $kind } בבעלות האפליקציה, "{ $name }" ({ $path }, החתימה אינה ידועה). תוכנית ההתקנה מצרפת את קובץ ה-.sys שלו ורושמת מחדש את שירות מנהל ההתקן באופן אוטומטי (sc create, דורש הרשאות מוגברות), ולאחר מכן מוודאת שהשירות רשום.
    }
core-service-driver-from-vendor =
    { $signed ->
        [yes] מנהל התקן מסוג { $kind } של ספק או מובנה, "{ $name }" ({ $path }, חתום על ידי { $signer }). WinAppMigrator אינו מפיץ מחדש מנהלי התקנים של צד שלישי; יש להתקין אותו מחדש במחשב היעד באמצעות חבילת מנהל ההתקן החתומה של הספק, לפני הפעלת האפליקציה.
       *[no] מנהל התקן מסוג { $kind } של ספק או מובנה, "{ $name }" ({ $path }, החתימה אינה ידועה). WinAppMigrator אינו מפיץ מחדש מנהלי התקנים של צד שלישי; יש להתקין אותו מחדש במחשב היעד באמצעות חבילת מנהל ההתקן החתומה של הספק, לפני הפעלת האפליקציה.
    }

## Runtimes an app needs, such as the Visual C++ runtime or .NET: how each is provided, as the Dependencies list says.

core-runtime-handling-built-in = מובנה ב-Windows
core-runtime-handling-ships-with-app = מגיע עם האפליקציה
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-handling-windows-feature = תכונת Windows: { $feature }
core-runtime-handling-included = כלול
# { $id } is the runtime's name in winget, such as Microsoft.VCRedist.2015+.x64.
core-runtime-handling-winget = מותקן באמצעות winget: { $id }
core-runtime-handling-manual = התקנה ידנית

## Downloading a runtime's installer, when an installer is built. Each is why it could not be, and starts with a small
## letter, as it follows other words. { $url } is a web address; { $error } is what went wrong.

core-runtime-download-not-https = ההורדה של { $url } נדחתה: הכתובת אינה HTTPS
# TLS is what keeps a download private.
core-runtime-tls-failed = אתחול TLS נכשל: { $error }
core-runtime-download-failed = ההורדה של { $url } נכשלה: { $error }
core-runtime-download-unreadable = הקריאה של { $url } נכשלה: { $error }
core-runtime-download-empty = ההורדה של { $url } לא החזירה נתונים
# { $came } and { $expected } are numbers of bytes.
core-runtime-download-cut-short = ההורדה של { $url } נקטעה: התקבלו { $came } מתוך { $expected } בתים

## Installing a runtime: why it was not installed. Each starts with a small letter, as it follows other words.

core-runtime-no-known-installer = אין תוכנית התקנה ידועה עבור רכיב זמן ריצה זה
# { $path } is where in the package, such as prerequisites\vc_redist.x64.exe.
core-runtime-payload-misplaced = החבילה מציינת את { $path } כתוכנית ההתקנה של רכיב זמן ריצה זה, אך זה אינו המקום שבו היא אמורה להיות, ולכן היא לא הופעלה
# { $path } is a file.
core-runtime-payload-missing = תוכנית ההתקנה המצורפת חסרה: { $path }
# { $error } is what went wrong.
core-runtime-payload-path-invalid = הנתיב של תוכנית ההתקנה המצורפת אינו חוקי: { $error }
# { $url } is the web address it can be downloaded from. winget is a program's name.
core-runtime-no-installer-route = אין תוכנית התקנה מצורפת או מזהה winget; ניתן להשיג אותו מ-{ $url }
# { $error } is what went wrong.
core-runtime-no-temp-folder = לא ניתן היה ליצור תיקיה לחילוץ שלו: { $error }
# { $reason } is what the check found.
core-runtime-payload-signature-failed = תוכנית ההתקנה המצורפת נכשלה בבדיקת החתימה ולא הופעלה: { $reason }
core-runtime-payload-timed-out = תוכנית ההתקנה המצורפת לא הסתיימה בזמן הקצוב והופסקה
# { $error } is what went wrong.
core-runtime-payload-not-started = לא ניתן היה להפעיל את תוכנית ההתקנה המצורפת: { $error }
core-runtime-installer-killed = תוכנית ההתקנה הופסקה ללא קוד יציאה
# { $code } is the number it ended with, such as 1603.
core-runtime-installer-failed = תוכנית ההתקנה הסתיימה עם הקוד { $code }
# { $feature } is a Windows feature's name, such as NetFx3; { $code } is the number DISM ended with. Windows Features is
# the name of a Windows window. Keep the command as it is.
core-runtime-feature-failed = הכלי DISM לא הצליח להפעיל את תכונת Windows { $feature } (קוד יציאה { $code }); ככל הנראה לא ניתן היה להוריד את המטען. יש להפעיל אותה דרך "תכונות Windows", או להריץ `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`.
# The same, when DISM gave no number.
core-runtime-feature-failed-no-code = הכלי DISM לא הצליח להפעיל את תכונת Windows { $feature } (קוד יציאה לא ידוע); ככל הנראה לא ניתן היה להוריד את המטען. יש להפעיל אותה דרך "תכונות Windows", או להריץ `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`.
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-feature-timed-out = הפעלת תכונת Windows { $feature } לא הסתיימה בזמן הקצוב והופסקה
# { $feature } is a Windows feature's name; { $error } is what went wrong.
core-runtime-feature-no-dism = הכלי DISM אינו זמין להפעלת תכונת Windows { $feature }: { $error }

## Installing a runtime with winget. { $id } is its name in winget, such as Microsoft.VCRedist.2015+.x64.

# { $error } is what went wrong.
core-runtime-winget-not-looked-for = לא ניתן היה לחפש את winget: { $error }
core-runtime-winget-not-installed = הכלי winget אינו מותקן
# { $path } is where winget is.
core-runtime-winget-misplaced = { $path } אינו במקום שבו Windows שומר את החבילות שלו, ולכן הוא לא הופעל
# { $path } is where winget is; { $reason } is what the check found.
core-runtime-winget-signature-failed = { $path } נכשל בבדיקת החתימה ולא הופעל: { $reason }
# { $reason } is one of the two messages above, or what went wrong.
core-runtime-winget-unusable = לא ניתן היה להשתמש ב-winget כדי להתקין את { $id }: { $reason }
# { $code } is the number winget ended with.
core-runtime-winget-failed = הפקודה winget install { $id } הסתיימה עם הקוד { $code }
core-runtime-winget-failed-no-code = הפקודה winget install { $id } הסתיימה עם קוד לא ידוע
core-runtime-winget-timed-out = הפקודה winget install { $id } לא הסתיימה בזמן הקצוב והופסקה
# { $error } is what went wrong.
core-runtime-winget-not-started = הכלי winget אינו זמין להתקנת { $id }: { $error }

## What told a scan how an app was installed, as the package's report lists it. The names of installers and files are
## as their makers write them.

# { $path } is an installer's record of what it installed, such as C:\Program Files\App\unins000.dat.
core-source-record-not-believed = לא נעשה שימוש ב-{ $path }: משתמשים שאינם מנהלי מערכת יכולים לשנות אותו
# { $count } is how many Windows Installer packages the bundle holds.
core-source-burn-bundle =
    { $count ->
        [one] צרור Burn של חבילת Windows Installer אחת
        [two] צרור Burn של שתי חבילות Windows Installer
       *[other] צרור Burn של { $count } חבילות Windows Installer
    }
# { $count } is how many paths Windows Installer recorded.
core-source-msi-database =
    { $count ->
        [one] מסד הנתונים של Windows Installer: נתיב מפתח אחד של רכיב
        [two] מסד הנתונים של Windows Installer: שני נתיבי מפתח של רכיבים
       *[other] מסד הנתונים של Windows Installer: { $count } נתיבי מפתח של רכיבים
    }
core-source-inno-log = יומן הסרת ההתקנה של Inno Setup (unins000.dat)
# { $count } is how many paths the log records.
core-source-inno-log-paths =
    { $count ->
        [one] יומן הסרת ההתקנה של Inno Setup: נתיב רשום אחד
        [two] יומן הסרת ההתקנה של Inno Setup: שני נתיבים רשומים
       *[other] יומן הסרת ההתקנה של Inno Setup: { $count } נתיבים רשומים
    }
core-source-nsis = חתימה של תוכנית הסרת התקנה של NSIS (Nullsoft)
# { $count } is how many paths the log records.
core-source-nsis-log-paths =
    { $count ->
        [one] NSIS AdvUninstLog uninstall.log: נתיב רשום אחד
        [two] NSIS AdvUninstLog uninstall.log: שני נתיבים רשומים
       *[other] NSIS AdvUninstLog uninstall.log: { $count } נתיבים רשומים
    }
core-source-squirrel = פריסה לכל משתמש של Squirrel (Update.exe / app-*)
# { $version } is a folder's name that says the version, such as app-1.2.3.
core-source-squirrel-version = הגרסה הנוכחית של Squirrel: { $version }
core-source-chocolatey = מטא-נתונים של חבילת Chocolatey
core-source-msix = חבילת MSIX / Appx
# { $identity } is a package's name, such as Contoso.App.
core-source-appx-identity = זהות AppxManifest: { $identity }
# { $count } is how many folders.
core-source-msix-data =
    { $count ->
        [one] נתוני חבילת MSIX לכל משתמש: מיקום אחד
        [two] נתוני חבילת MSIX לכל משתמש: שני מיקומים
       *[other] נתוני חבילת MSIX לכל משתמש: { $count } מיקומים
    }
# App Paths is a registry key's name.
core-source-app-paths = רישום ב-App Paths

## Why a runtime is needed, as the package's report says it.

# { $program } is a program's file name, such as app.exe; { $dll } is a DLL's, such as msvcp140.dll.
core-runtime-program-imports = { $program } מייבא את { $dll }

## What else of an app's a scan finds in the registry: fonts and certificates, and why each is the app's.

# Why a font is among the app's files, in the package's list of them.
core-registry-font-recorded = גופן שרשומת ההתקנה של האפליקציה מציינת
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-named-for-the-app = הוא נקרא על שם האפליקציה או על שם המפרסם שלה
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-recorded = רשומת ההתקנה של האפליקציה מציינת אותו

## Who is signed in, for the Whose Data choice. Each of the first six is why it could not be told, and starts with a
## small letter, as it is shown inside other words. An account is shown by its id, such as S-1-5-21-...-1001.

# { $error } is what Windows said.
core-profile-desktop-owner-not-said = לא ניתן היה לקבל מ-Windows את הבעלים של שולחן עבודה זה: { $error }
# { $error } is what went wrong.
core-profile-desktop-owner-not-asked = לא ניתן היה לשאול את Windows של מי שולחן עבודה זה: { $error }
core-profile-signed-in-account-no-profile = לחשבון המחובר כאן, { $account }, אין פרופיל רשום
core-profile-desktop-not-a-person = שולחן העבודה כאן שייך ל-{ $account }, שאינו חשבון של אדם
core-profile-nobody-signed-in-running-account-no-profile = אף משתמש אינו מחובר כאן, ולחשבון שמריץ תוכנית זו, { $account }, אין פרופיל רשום
core-profile-nobody-signed-in-running-not-a-person = אף משתמש אינו מחובר כאן, ו-{ $account }, שמריץ תוכנית זו, אינו חשבון של אדם
# SID is the name Windows gives an account's id.
core-profile-nobody-signed-in-no-sid = אף משתמש אינו מחובר כאן, ולחשבון זה אין SID
# { $accounts } is a list of accounts.
core-profile-several-desktops = ליותר מחשבון אחד יש שולחן עבודה בהפעלה זו: { $accounts }
# Under the Whose Data choice. { $reason } is one of the messages above.
core-profile-signed-in-unknown = לא ניתן היה לקבוע מי מחובר ({ $reason }).
# Under the Whose Data choice, when there are accounts to choose from. { $reason } is one of the messages above.
core-profile-signed-in-unknown-choose = לא ניתן היה לקבוע מי מחובר ({ $reason }). יש לבחור של מי הנתונים שייכללו.
