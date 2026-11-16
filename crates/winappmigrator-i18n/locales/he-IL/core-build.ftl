# Words the core writes for people while it builds a package from an installed app: what a package records for an
# installer to show later, errors shown in the app, the list of what goes in and what is left out, opening a disk
# image or another PC over the network, and what an installer says when the package it carries cannot be read.
# Message names start with core-.


## Scanning an app, before a package is built. Shown in the app when a scan cannot go on.

# { $path } is the app's program, such as D:\Program Files\Acme\Acme.exe, on a drive taken from another PC.
core-scan-program-not-on-the-drive = { $path } עובר דרך קישור או אינו נמצא בכונן המחובר, ולכן הוא לא נקרא.
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-not-on-this-pc = { $path } אינו נמצא בדיסק של מחשב זה, או שיש לו שם שיפתח משהו אחר, ולכן הוא לא נקרא.
# { $path } is the app's program.
core-scan-program-someone-elses = { $path } נמצא בתיקיות של חשבון אחר או אינו שייך לחשבון זה, והרשומה של אפליקציה זו עצמה אינה מציינת שמקומו שם, ולכן הוא לא נקרא.
# { $app_name } is the app's name, as the list of apps calls it.
core-scan-no-executable = לא נמצא קובץ הפעלה עבור { $app_name }. ייתכן שלמנהל התקן, לרכיב זמן ריצה או לתוסף אין תוכנית משלו לאריזה.
# { $path } is the file that was picked.
core-scan-not-a-file = יעד הסריקה אינו קובץ: { $path }
# Names something left out of a package, in a list of what was left out and why. { $name } is the variable's name,
# such as ACME_LICENSE_KEY.
core-scan-environment-variable-item = משתנה הסביבה { $name }

## Folders and parts that might also be the app's, offered in a list to tick. Each says why it is offered.

core-near-starts-with-app-name = השם שלו מתחיל בשם של אפליקציה זו.
core-near-has-app-name = השם שלו מכיל את השם של אפליקציה זו.
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-starts-with-publisher = השם שלו מתחיל בשם המפרסם, { $publisher }.
# { $folder } is a folder's name, such as Program Files.
core-near-hidden-part-beside = חלק שמוסתר מרשימת האפליקציות, ומותקן לצד אפליקציה זו ב-{ $folder }.
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-hidden-part-publisher = חלק שמוסתר מרשימת האפליקציות, מאותו מפרסם, { $publisher }.

## What a scan found, as a person chooses from it: the folders that go in, and what is left out, and why

# Why a folder cannot be left out, beside its number of files.
core-footprint-locked-app-folder = אלה תמיד נכללים עם התוכנית הראשית
core-footprint-locked-main-program = מכילה את התוכנית הראשית, שתמיד נכללת
core-footprint-locked-needed = מכילה את מה שהתוכנית הראשית אינה יכולה לפעול בלעדיו
core-footprint-locked-font = נכללת יחד עם הגופן שבה, תחת "מה שמקשר אותה ל-Windows"
# Where a folder is. "AppData" and "ProgramData" are the names of Windows folders; keep them as they are.
core-footprint-place-roaming = הגדרות בפרופיל שלך (AppData)
core-footprint-place-local = הגדרות עבור מחשב זה בפרופיל שלך (AppData)
core-footprint-place-program-data = משותף לכל המשתמשים במחשב (ProgramData)
core-footprint-place-documents = בתיקיה "מסמכים" שלך
core-footprint-place-saved-games = בתיקיה "משחקים שמורים" שלך
core-footprint-place-public-documents = בתיקיה "מסמכים ציבוריים"
core-footprint-place-app-folder = בתיקיה של האפליקציה
# { $folder } is a folder, such as C:\Users\Sam\AppData\Roaming.
core-footprint-place-in = ב-{ $folder }
# A row for the files directly in the app's folder. { $folder } is that folder's name, such as Acme.
core-footprint-files-directly-in = קבצים שנמצאים ישירות ב-{ $folder }
# Headings over what is left out, and why each item is.
core-footprint-licenses-left-out = לא כלולים בתוכנית התקנה זו, מכיוון שהאפשרות "נתוני רישיון והפעלה" כבויה
core-footprint-font-stays-behind = הגופן נשאר מאחור.
core-footprint-drivers-left-out = מנהלי התקנים ששייכים ל-Windows או לספק, שאי אפשר להעביר הלאה, ולכן יש להתקין אותם מחדש במחשב האחר
core-footprint-driver-reinstall = יש להתקין אותו מחדש מהחבילה של הספק שלו.
# Why something else is left out along with what the person left out. { $service } is a service's name, such as AcmeSync.
core-footprint-needs-left-out = הוא זקוק ל-{ $service }, שהושמט
core-footprint-told-to-use-left-out = הוא מוגדר להשתמש במשהו שנמצא בתיקיה שהשמטת
core-footprint-file-in-left-out = הקובץ שלו נמצא בתיקיה שהשמטת
core-footprint-program-in-left-out = התוכנית שעבורה הוא נועד נמצאת בתיקיה שהשמטת
core-footprint-names-left-out = הוא מציין תיקיה שהשמטת
# Why something is left out, as the installer tells whoever installs it.
core-footprint-builder-left-it-out = הוא הושמט על ידי מי שבנה את תוכנית ההתקנה
# Something the person left out, by kind and name, such as "Service Acme Updater". { $kind } is the kind, such as
# Service or Scheduled Task; { $title } is its name.
core-footprint-left-out-item = { $kind } { $title }

## Building a package and an installer from a scan. Shown in the app when a build cannot go on.

core-package-scan-of-another-app = סריקה זו היא של אפליקציה אחרת מזו שנבנית, ולכן שום דבר לא נבנה
core-package-scan-brought-in-more = סריקה זו הביאה נתונים שבנייה זו לא התבקשה לכלול, ולכן שום דבר לא נבנה
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-package-program-gone = { $path } כבר אינו קיים, ולכן שום דבר לא נבנה. יש לסרוק שוב את האפליקציה.
core-package-needs-stub = ארכיון עם חילוץ עצמי דורש שלד תוכנית התקנה
# { $reason } is what went wrong.
core-package-staged-file-failed = לא ניתן היה להכניס לחבילה קובץ מתיקיית הביניים: { $reason }
# { $shortfalls } says which drives are short of room and by how much, in sentences of its own.
core-package-not-enough-room = אין מספיק שטח לבניית תוכנית התקנה זו, ולכן שום דבר לא נקרא או נכתב. { $shortfalls }. יש לפנות שם מעט שטח, או לבחור תיקיית פלט בכונן אחר.
# { $path } is a file's path, on a drive taken from another PC.
core-package-not-read-off-the-drive = { $path } עובר דרך קישור או אינו נמצא בכונן המחובר, ולכן הוא לא נקרא
# { $path } is a file's path.
core-package-not-read-not-local = { $path } אינו נמצא בדיסק של מחשב זה, ולכן הוא לא נקרא
# { $path } is a file's path.
core-package-not-read-other-account = { $path } מוביל לתיקיות של חשבון אחר, ולכן הוא לא נקרא
# { $path } is a file's path, as the PC it came from called it.
core-package-not-on-the-drive = { $path } אינו נמצא בכונן המחובר, ולכן הוא לא נקרא
# { $path } is a file's path.
core-package-backup-copy-no-parent = לא ניתן ליצור עותק גיבוי של קובץ ללא תיקיית אב: { $path }
# { $path } is a file's path.
core-package-backup-copy-destination-no-parent = ליעד של עותק הגיבוי אין תיקיית אב: { $path }
# robocopy is the name of a Windows command; keep it as it is. { $code } is how it ended, such as Some(8).
core-package-robocopy-failed = העתקה במצב גיבוי באמצעות robocopy נכשלה (קוד יציאה { $code })
# Stands for a folder where a file's name would be, as in "I/O failed for temporary package folder: …".
core-package-temporary-folder = תיקיית החבילה הזמנית
# Stands for the installer being written where a file's name would be, as in "I/O failed for the installer: …".
core-package-the-installer = תוכנית ההתקנה
# { $path } is a file's path, or its name in the package. { $error } is what went wrong with it.
core-package-path-error = { $path }: { $error }
# { $name } is a file's name in the package, such as app/bin/App.exe.
core-package-same-place = לקובץ אחר יש אותו מקום בחבילה: { $name }
# { $name } is a file's name in the package.
core-package-changed-in-protected-folder = { $name } השתנה בתיקיה המוגנת בזמן שהחבילה נוצרה
core-package-worker-died = הליך משנה שדחס קבצים נעצר באופן לא צפוי, ולכן לא נכתבה תוכנית התקנה
# { $path } is a file's path.
core-package-link-not-read = { $path } הוא קישור, ולכן הוא לא נקרא
# { $path } is a file's path. { $error } is what went wrong.
core-package-not-compressed = לא ניתן היה לדחוס את { $path }: { $error }
# { $path } is a file's path. { $reason } is what went wrong.
core-package-not-read-back = לא ניתן היה לקרוא בחזרה את { $path }: { $reason }

## Where an app's files are, and where an installer may write them. Shown when a path is refused.

# { $path } is a folder, such as C:\Program Files\Acme.
core-discovery-root-not-full-path = התיקיה שאליה כותבים חייבת להיות נתיב מלא: { $path }
# { $path } is a folder's path.
core-discovery-root-not-a-folder = התיקיה שאליה כותבים אינה תיקיה: { $path }
# { $path } is a file's path.
core-discovery-escapes-root = הנתיב יוצא מחוץ לשורש המיועד: { $path }
# { $path } is the link's path.
core-discovery-refusing-to-write-through-link = כתיבה דרך קישור אינה מותרת: { $path }
# { $path } is the link's path.
core-discovery-refusing-link = מעבר דרך קישור אינו מותר: { $path }
# { $error } is what Windows said went wrong.
core-discovery-refusing-link-because = מעבר דרך קישור אינו מותר: { $error }
# { $path } is the link's path.
core-discovery-link-to-somewhere-else = { $path } הוא קישור למקום אחר, ואחרים עשויים להיות מסוגלים לשנות את היעד שלו. יש להשתמש במקום שאליו הוא מוביל
# { $path } is a folder's path.
core-discovery-no-parent-folder = ל-{ $path } אין תיקיית אב
# { $path } is the install location as it was given. Keep the quotes, or use your language's own.
core-discovery-install-location-not-full-path = מיקום ההתקנה חייב להיות נתיב מלא: "{ $path }"
# { $path } is the install location.
core-discovery-install-location-a-link = מיקום ההתקנה הוא קישור למקום אחר: { $path }
# { $path } is a file's path.
core-discovery-device-name = הנתיב משתמש בשם ששמור להתקן של Windows: { $path }
# { $path } is a file's path.
core-discovery-device-name-on-older-windows = ל-{ $path } יש שם שגרסה זו של Windows פותחת כהתקן ולא כקובץ, ולכן שום דבר לא נכתב אליו. ניתן להתקין אותו ב-Windows 11
# { $path } is a file's path.
core-discovery-unsafe-path = נתיב לא בטוח: { $path }
# { $path } is a file's path.
core-discovery-no-package-path = לקובץ אין נתיב יחסי בחבילה: { $path }

## Writing an installer, and reading the package inside one. Most of these are shown by an installer, on the PC it
## runs on, when it cannot read what it carries. "The package" is the app's files and settings that an installer
## carries; "the stub" is the program part of an installer, before the package; "the data file" is the .wamdata file
## that holds the package of a very large installer, beside it.

# Stands for the package where a file's name would be, as in "I/O failed for installer package: …".
core-archive-installer-package = חבילת תוכנית ההתקנה
# Stands for the installer program built into WinAppMigrator, where a file's name would be.
core-archive-embedded-stub = שלד תוכנית ההתקנה המוטמע
# { $index } is the number of a file in the package. { $reason } is what went wrong.
core-archive-entry-unreadable = לא ניתן היה לקרוא את פריט { $index } בחבילה: { $reason }
# { $name } is a file's name inside the package, such as app/bin/App.exe.
core-archive-entry-unsafe-path = לפריט ה-zip יש נתיב לא בטוח: { $name }
# { $name } is a file's name inside the package.
core-archive-holds-more = { $name } מכיל יותר ממה שהוא מצהיר
# { $name } is a file's name inside the package.
core-archive-name-too-long = שם זה ארוך מדי עבור zip: { $name }
core-archive-changed-while-copied = החבילה השתנתה בזמן שהוצבה אחרי השלד, ולכן לא נכתבה תוכנית התקנה
core-archive-stub-not-a-program = החלק של תוכנית התקנה זו שלפני החבילה שלה אינו תוכנית
core-archive-damaged = תוכנית התקנה זו פגומה או ששונתה לאחר שנבנתה, ולכן שום דבר לא נקרא מהחבילה שלה. יש להעתיק אותה שוב מהמקום שבו היא נבנתה.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-missing = תוכנית התקנה זו שומרת את החבילה שלה בקובץ בשם { $name }, באותה תיקיה, והקובץ הזה אינו נמצא שם. שני הקבצים חייבים להישאר יחד: יש להעתיק לכאן את { $name } מהמקום שבו נבנתה תוכנית ההתקנה, ולאחר מכן להפעיל אותה שוב.
# { $name } is the data file's name, such as App_installer.wamdata. { $error } is what Windows said went wrong.
core-archive-data-file-unopened = לא ניתן היה לפתוח את קובץ הנתונים של תוכנית התקנה זו, { $name }: { $error }. שני הקבצים חייבים להישאר יחד, בתיקיה שמחשב זה יכול לקרוא.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-a-link = { $name } הוא קישור או תיקיה, וקובץ הנתונים של תוכנית התקנה זו חייב להיות קובץ עצמאי, ממש לצידה, ולכן שום דבר לא נקרא ממנו. יש להעתיק לכאן את הקובץ האמיתי.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-wrong = { $name } אינו קובץ הנתונים של תוכנית התקנה זו, או שהוא פגום או ששונה לאחר שנבנה, ולכן שום דבר לא נקרא ממנו. יש להעתיק שוב את שני הקבצים מהמקום שבו הם נבנו.
core-archive-stub-signed = שלד תוכנית ההתקנה חתום; הוא חייב להישאר לא חתום, ולכן יש לחתום במקומו על תוכנית ההתקנה המוגמרת
# { $size } is a size, such as 4.2 GB.
core-archive-too-big-to-run = גודל תוכנית ההתקנה יהיה { $size }, ו-Windows אינו יכול להפעיל קובץ הפעלה בגודל 4 GB ומעלה; שום דבר לא נכתב
core-archive-outside-signature = בתוכנית ההתקנה יש נתונים מחוץ לחתימה שלה, וייתכן ששונתה ללא הרשאה
core-archive-footer-length = אורך הכותרת התחתונה של הארכיון עם החילוץ העצמי אינו חוקי
core-archive-seek-outside = ניסיון לגשת למיקום מחוץ לחבילה
# { $path } is a file's path.
core-archive-unsafe-package-path = נתיב חבילה לא בטוח: { $path }
# { $path } is a file's path.
core-archive-empty-package-path = נתיב חבילה ריק: { $path }

## Opening a disk image of another PC (a .vhd or .vhdx file) to read apps from it

# { $path } is the disk image file, such as D:\Backups\OldPC.vhdx.
core-image-not-attachable = { $path } אינו תמונת דיסק ש-Windows יכול לצרף (.vhd או .vhdx)
core-image-needs-administrator = צירוף תמונת דיסק דורש הרשאות מנהל מערכת
# { $path } is the disk image file the person picked.
core-image-not-a-file = { $path } אינו קובץ
# { $path } is the disk image file. "Edit Disk" is the name of a Hyper-V Manager command.
core-image-differencing = { $path } הוא דיסק הבדלים, שמכיל רק את מה שהשתנה מאז הדיסק שממנו הוא נוצר, ו-Windows יפתח גם את הדיסק הזה, בכל מיקום שמצוין בו. יש למזג אותו תחילה לתוך הדיסק הזה, באמצעות "עריכת דיסק" של Hyper-V, ולבחור בדיסק הזה
# { $path } is the disk image file.
core-image-unreadable = { $path } אינו קובץ .vhd או .vhdx שניתן היה לקרוא
# ProgramData is the name of a Windows folder; keep it as it is.
core-image-no-program-data = לא ניתן היה למצוא את התיקיה ProgramData של מחשב זה
# The reason given in core-image-could-not-attach when Windows already has the image open.
core-image-already-attached = התמונה כבר צורפה על ידי Windows, ומה שצירף אותה עשוי לכתוב אליה. יש לנתק אותה תחילה, או לבחור את הכונן שלה ככונן מחובר
# { $path } is the disk image file. { $reason } is why, such as core-image-already-attached or what Windows said.
core-image-could-not-attach = לא ניתן היה לצרף את { $path }: { $reason }
# { $path } is the disk image file.
core-image-no-windows-volume = ב-{ $path } אין אמצעי אחסון שמותקן בו Windows שניתן היה לקרוא. יש לבטל תחילה את הנעילה של אמצעי אחסון שמוצפן באמצעות BitLocker.
# Given as the reason when Windows failed without saying anything.
core-image-windows-did-not-say = הסיבה לא צוינה על ידי Windows

## Reading another PC over the network, through the client running on it. Shown as why something on that PC
## could not be read. "The client" is WinAppMigrator Client, the small program on the other PC.

core-remote-too-slow = המחשב האחר התעכב יותר מדי בתשובה
core-remote-closed = החיבור למחשב האחר סגור
core-remote-failed = החיבור למחשב האחר נכשל
core-remote-answered-something-else = הלקוח ענה משהו אחר
# { $name } is the PC's name or address as the person typed it, such as OLDPC or 192.168.1.20.
core-remote-not-found = { $name } לא נמצא ברשת
core-remote-did-not-say-what-it-is = הלקוח לא ציין מה הוא
core-remote-ordinary-names-only = ניתן לשאול רק על נתיב שמתחת לשורש של המחשב האחר, בשמות רגילים
core-remote-more-but-none = המחשב האחר ציין שבתיקיה יש פריטים נוספים, ולא שלח אף אחד מהם
# { $count } is the most things one folder may hold, such as 250,000.
core-remote-too-many-listed = המחשב האחר פירט יותר מ-{ $count } פריטים בתיקיה אחת
core-remote-sent-too-much = המחשב האחר שלח מקובץ יותר ממה שהתבקש
core-remote-name-not-text = לא ניתן לשאול על שם שאינו טקסט
core-remote-below-root-only = ניתן לשאול רק על נתיב שמתחת לשורש של המחשב האחר
core-remote-file-ended-early = הקובץ של המחשב האחר הסתיים מוקדם מהצפוי
core-remote-link-not-followed = אין מעקב אחר קישור במחשב אחר
core-remote-only-files-opened = במחשב אחר נפתחים רק קבצים
core-remote-listing-said-nothing = הרשימה לא ציינה עליו דבר
core-remote-change-time-unknown = מועד השינוי שלו אינו ידוע
core-remote-seek-before-start = ניסיון לגשת למיקום שלפני ההתחלה
# { $size } is how many bytes a file is; { $limit } is the most that were expected. Both are numbers.
core-remote-too-large =
    { $size ->
        [one] גודלו { $size } בית, ולא היו צפויים יותר מ-{ $limit }
        [two] גודלו { $size } בתים, ולא היו צפויים יותר מ-{ $limit }
       *[other] גודלו { $size } בתים, ולא היו צפויים יותר מ-{ $limit }
    }

## Checking that a download is signed by Microsoft. Each is given as the reason a check failed.

# { $error } is what Windows said went wrong.
core-signature-no-powershell = לא ניתן היה להפעיל את PowerShell: { $error }
# { $error } is what went wrong reading PowerShell's answer.
core-signature-unreadable = לא ניתן היה לקרוא את החתימה: { $error }
# { $signer } is who signed the file, such as Contoso Ltd.
core-signature-not-microsoft = הוא חתום על ידי { $signer }, ולא על ידי Microsoft
core-signature-not-microsoft-unnamed = הוא חתום על ידי גורם אחר, ולא על ידי Microsoft
# { $status } is Windows' own word for the signature's state, such as HashMismatch or NotSigned, which is not translated.
core-signature-status = מצב החתימה שלו הוא { $status }

## Warnings a package records when it is built from an app, which an installer shows later, maybe on another PC, in
## that PC's language. "This app" is the app being packaged.

core-warning-build-program-needs-unreadable = לא ניתן היה לקרוא מה תוכנית זו צריכה מ-Windows, ולכן לא ייבדק אם היא יכולה לפעול במחשב האחר.
core-warning-build-external-manifest = זוהה מניפסט אפליקציה חיצוני; הפענוח של תלויות זו לצד זו נדחה לשלב מאוחר יותר.
core-warning-build-non-exe = הקובץ שנבחר אינו משתמש בסיומת .exe; סריקת ה-PE תימשך.
core-warning-build-stand-in-program = לא נמצא קובץ הפעלה רגיל של האפליקציה, ולכן קובץ זה משמש במקומו. תוצאות הארכיטקטורה והתלויות מתארות קובץ זה, וייתכן שאינן מתארות את האפליקציה.
core-warning-build-other-apps-files-unknown = לא ניתן היה לקרוא את כל מה ש-Windows Installer מחזיק לגבי אפליקציות אחרות, ולכן תיקיה שרשומה עבור אפליקציה זו עשויה להכיל קבצים של אפליקציה אחרת בלי שדבר כאן יציין זאת.
core-warning-build-app-folder-has-another-apps-file = לפי Windows Installer, לאפליקציה אחרת יש קובץ בתיקיה שבה נמצאת התוכנית של אפליקציה זו, והוא מועבר יחד איתה, מכיוון שהתיקיה כולה נלקחת.
# "App Paths" is the name of a Windows registry key; keep it as it is.
core-warning-build-app-paths-folder-left-out = לפי הרשומה של אפליקציה זו ב-App Paths, היא מחפשת בתיקיה שאינה שלה, ולכן תיקיה זו אינה מועברת. ייתכן שהאפליקציה לא תופעל במחשב שאין בו את מה שנמצא שם.
core-warning-build-uninstaller-folder-left-out = התיקיה שבה נמצאת תוכנית הסרת ההתקנה של אפליקציה זו הושמטה, מכיוון שלפי Windows Installer יש שם קובץ של אפליקציה אחרת.
# { $why } is never shown: it chooses the reason. Translate each reason as the end of the sentence.
core-warning-build-recorded-folder-left-out =
    משהו שרשום עבור אפליקציה זו מציין תיקיה שהושמטה, מכיוון { $why ->
        [unreadable] שהנתיב שלה מוביל אל מחוץ לכונן או עובר דרך קישור, ולכן לא בטוח לקרוא אותה
        [missing] שהיא אינה קיימת
        [link] שהיא עוברת דרך קישור
        [short-name] שהיא אינה כתובה בשם האמיתי שלה
        [another-account] שהיא נמצאת בתיקיות של חשבון אחר
        [one-account] שאפליקציה זו הותקנה עבור חשבון אחד, ותיקיה זו אינה שייכת לחשבון הזה
        [shared] שהיא מכילה פריטים משותפים, או שהיא חלק מ-Windows
        [generic-name] שהשם שלה אינו מעיד למי היא שייכת
        [publisher] שהיא נקראת רק על שם המפרסם, ועשויה להכיל מוצרים אחרים שלו
        [holds-the-app] שהיא מכילה את התיקיה של אפליקציה זו עצמה, ועשויה להכיל יותר
        [another-apps-name] שהיא נקראת בשם של אפליקציה אחרת
        [another-apps-record] שרשומת ההתקנה שלה שייכת לאפליקציה אחרת
        [in-another-apps-folder] שהיא נמצאת בתיקיה של אפליקציה אחרת
       *[another-apps] שהיא שייכת לאפליקציה אחרת
    }.
core-warning-build-app-folder-holds-another-app = אפליקציה אחרת במחשב זה רשומה כנמצאת בתוך התיקיה של אפליקציה זו, והיא מועברת יחד איתה, מכיוון שהתיקיה כולה נלקחת.
# "Program Files (x86)" is the name of a Windows folder; keep it as Windows calls it.
core-warning-build-from-32-bit-windows = אפליקציה זו הייתה ב-Windows של 32 סיביות. מפתחות הרישום שלה מסודרים כפי ש-Windows של 64 סיביות שומר מפתחות של אפליקציה של 32 סיביות, ותוכנית ההתקנה מציעה עבורה את Program Files (x86).
# { $dll } is a file's name, such as MSVCP140D.dll.
core-warning-build-debug-runtime = תוכנית זקוקה ל-{ $dll }, גרסת ניפוי באגים של רכיב זמן הריצה של Visual C++. ל-Microsoft אין תוכנית התקנה עבור גרסאות כאלה, וייתכן שאסור להפיץ אותן, ולכן התוכנית לא תפעל במחשב שאין בו Visual Studio.
# { $program } is a program's file name, such as Tool.exe.
core-warning-build-single-file-program = { $program } היא תוכנית .NET שארוזה בקובץ אחד, שאינה מציינת לאיזו גרסה של .NET היא זקוקה, ולכן לא מתוכננת עבורה התקנה של .NET. אם היא זקוקה לגרסה כלשהי, היא לא תופעל במחשב שאין בו אותה.
# { $count } is how many of the app's programs.
core-warning-build-programs-not-inspected =
    { $count ->
        [one] תוכנית אחת של האפליקציה לא נבדקה לגבי רכיבי זמן הריצה שהיא צריכה, מכיוון שלאפליקציה יש תוכניות רבות כל כך או שהן גדולות כל כך.
        [two] שתיים מהתוכניות של האפליקציה לא נבדקו לגבי רכיבי זמן הריצה שהן צריכות, מכיוון שלאפליקציה יש תוכניות רבות כל כך או שהן גדולות כל כך.
       *[other] { $count } מהתוכניות של האפליקציה לא נבדקו לגבי רכיבי זמן הריצה שהן צריכות, מכיוון שלאפליקציה יש תוכניות רבות כל כך או שהן גדולות כל כך.
    }
# { $runtime } is a runtime's name, such as Java Runtime Environment.
core-warning-build-runtime-no-installer = האפליקציה זקוקה ל-{ $runtime }, ואין כאן תוכנית התקנה עבורו, ולכן שום תוכנית התקנה לא תופעל. יש להתקין אותו בעצמך במחשב האחר.
# { $why } is why something was left out, such as "its name says what it holds".
core-warning-build-left-out-of-package = הושמט מהחבילה: { $why }.
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-missing-dependency = לא ניתן היה לפתור תלות: { $name }
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-dependency-only-here = { $name } נמצא רק דרך נתיב החיפוש של מחשב זה, ואינו חלק מהאפליקציה, ולכן הוא אינו מועבר. ייתכן שהאפליקציה לא תופעל במחשב שאין בו קובץ זה.
core-warning-build-driver-detected = זוהה מנהל התקן או רכיב ליבה. מנהל התקן שבבעלות האפליקציה מצורף ונרשם מחדש; מנהל התקן של ספק או מנהל התקן מובנה אינו מופץ מחדש, ויש להתקין אותו מחדש מחבילת מנהל ההתקן החתומה של הספק.
core-warning-build-near-no-longer-offered = פריט שסומן ברשימה של מה שעשוי גם להיות שייך לאפליקציה זו כבר אינו ברשימה, ולכן הוא לא נוסף. יש לעיין שוב ברשימה.
# { $app_name } is the name of a part of the app, as the list of apps calls it. { $why } is never shown: it chooses the
# reason. Translate each reason as the end of the sentence.
core-warning-build-near-part-left-out =
    חלק ממה שרשום כשייך ל-{ $app_name } הושמט: { $why ->
        [unreadable] הנתיב שלו מוביל אל מחוץ לכונן או עובר דרך קישור, ולכן לא בטוח לקרוא אותו
        [missing] הוא אינו קיים
        [link] הוא עובר דרך קישור
        [short-name] הוא אינו כתוב בשם האמיתי שלו
        [another-account] הוא נמצא בתיקיות של חשבון אחר
        [one-account] אפליקציה זו הותקנה עבור חשבון אחד, והוא אינו שייך לחשבון הזה
        [shared] הוא מכיל פריטים משותפים, או שהוא חלק מ-Windows
        [generic-name] השם שלו אינו מעיד למי הוא שייך
        [publisher] הוא נקרא רק על שם המפרסם, ועשוי להכיל מוצרים אחרים שלו
        [holds-the-app] הוא מכיל את התיקיה של אפליקציה זו עצמה, ועשוי להכיל יותר
        [another-apps-name] הוא נקרא בשם של אפליקציה אחרת
        [another-apps-record] רשומת ההתקנה שלו שייכת לאפליקציה אחרת
        [in-another-apps-folder] הוא נמצא בתיקיה של אפליקציה אחרת
       *[another-apps] הוא שייך לאפליקציה אחרת
    }.
# { $app_name } is the name of a part of the app, as the list of apps calls it.
core-warning-build-near-keys-left-out = מפתחות הרישום שרשומת ההתקנה של { $app_name } מפרטת הושמטו, מכיוון שרשומה זו מציינת גם פריטים של אפליקציה אחרת.
# "Windows keeps what an app wrote for one account" is Windows' VirtualStore folder.
core-warning-build-virtual-store-program = נמצאה תוכנית בתיקיה שבה Windows שומר את מה שאפליקציה כתבה עבור חשבון אחד. Windows אף פעם לא מציב תוכנית שם, ולכן משהו אחר עשה זאת, והיא הושמטה.
# { $path } is a file or folder, such as C:\Users\Sam\AppData\Roaming\Acme.
core-warning-build-file-left-out = { $path } הושמט: הוא עובר דרך קישור בכונן מחובר, או שהוא שייך לחשבון אחר מזה שהנתונים שלו נכללים.
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as D:\Sam\Documents.
core-warning-build-user-folder-not-on-drive = התיקיה { $folder } של { $user } הייתה ב-{ $path }, שאינו בכונן זה, ולכן מה שנמצא שם לא נכלל.
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as \\server\home\Sam\Documents.
core-warning-build-user-folder-not-on-this-pc = התיקיה { $folder } של { $user } הייתה ב-{ $path }, שאינו בדיסק של מחשב זה, ולכן מה שנמצא שם לא נכלל.
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-base-unreadable = לא ניתן היה לקרוא את { $path } ({ $error }); נתוני אפליקציה שמאוחסנים שם לא נכללו.
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-folder-unreadable = לא ניתן היה לקרוא את { $path } ({ $error }); ייתכן שחסרים נתוני אפליקציה שנמצאים בתיקיות המשנה שלה.
# { $error } is what went wrong.
core-warning-build-entry-unreadable = לא ניתן היה לקרוא פריט במערכת הקבצים: { $error }
# { $count } is how many links. { $names } lists some of them. { $more } is how many more there are, which are not
# named; when it is 0, the sentence ends after the list.
core-warning-build-links-not-followed =
    { $count ->
        [one] לא בוצע מעקב אחר קישור אחד בתיקיה זו, או אחר שם שיפתח משהו אחר, ולכן מה שהוא מצביע עליו אינו בתוכנית ההתקנה: { $names }
        [two] לא בוצע מעקב אחר שני קישורים בתיקיה זו, או אחר שמות שיפתחו משהו אחר, ולכן מה שהם מצביעים עליו אינו בתוכנית ההתקנה: { $names }
       *[other] לא בוצע מעקב אחר { $count } קישורים בתיקיה זו, או אחר שמות שיפתחו משהו אחר, ולכן מה שהם מצביעים עליו אינו בתוכנית ההתקנה: { $names }
    }{ $more ->
        [0] .
       *[other] , ועוד { $more }.
    }
# { $count } is how many folders. { $depth } is how many levels down a folder can be. { $names } lists some of them.
# { $more } is how many more there are, which are not named; when it is 0, the sentence ends after the list.
core-warning-build-folders-too-deep =
    { $count ->
        [one] תיקיה אחת נמצאת יותר מ-{ $depth } רמות למטה, ולכן מה שבתוכה אינו בתוכנית ההתקנה: { $names }
        [two] שתי תיקיות נמצאות יותר מ-{ $depth } רמות למטה, ולכן מה שבתוכן אינו בתוכנית ההתקנה: { $names }
       *[other] { $count } תיקיות נמצאות יותר מ-{ $depth } רמות למטה, ולכן מה שבתוכן אינו בתוכנית ההתקנה: { $names }
    }{ $more ->
        [0] .
       *[other] , ועוד { $more }.
    }
# { $error } is what went wrong.
core-warning-build-metadata-unreadable = לא ניתן היה לקרוא את המטא-נתונים של הקובץ: { $error }
core-warning-build-reserved-name = שם הקובץ הזה שמור להתקן של Windows, ולכן הוא לא הועתק.
# { $error } is what went wrong.
core-warning-build-hash-failed = לא ניתן היה לחשב גיבוב לקובץ: { $error }
core-warning-build-cloud-placeholder = לקובץ ענן שזמין באינטרנט בלבד (לדוגמה, קבצים לפי דרישה של OneDrive) אין נתונים מקומיים; יש להצמיד או להוריד אותו לפני הגיבוי. הוא דווח אך לא הועתק.
# { $file_name } is a driver's file name, such as acmeflt.sys.
core-warning-build-driver-file = קובץ מנהל התקן/ליבה "{ $file_name }" אינו מועתק כקובץ רגיל. אם הוא משמש שירות מנהל התקן רשום של אפליקציה זו, תוכנית ההתקנה מצרפת אותו ורושמת מחדש את מנהל ההתקן באופן אוטומטי; אחרת, יש להתקין אותו מחדש באמצעות חבילת מנהל ההתקן החתומה של הספק.
core-warning-build-file-gone = פריט זה היה קיים כשהאפליקציה נסרקה, וכעת הוא אינו קיים, ולכן הוא אינו בתוכנית ההתקנה.
# { $error } is what went wrong.
core-warning-build-file-copy-failed = לא ניתן היה להעתיק את הקובץ לחבילה: { $error }
# { $count } is how many files. { $names } lists them.
core-warning-build-not-looked-through =
    { $count ->
        [one] בחבילה זו יש קובץ שלא ניתן היה לבדוק אם הוא מעיד ממי היא הגיעה: { $names }. ייתכן שהתוכן שלו מעיד על כך.
        [two] בחבילה זו יש שני קבצים שלא ניתן היה לבדוק אם הם מעידים ממי היא הגיעה: { $names }. ייתכן שהתוכן שלהם מעיד על כך.
       *[other] בחבילה זו יש קבצים שלא ניתן היה לבדוק אם הם מעידים ממי היא הגיעה: { $names }. ייתכן שהתוכן שלהם מעיד על כך.
    }
# { $count } is the most empty folders an installer makes, such as 10,000.
core-warning-build-empty-folders-truncated = לאפליקציה זו יש יותר מ-{ $count } תיקיות ריקות, ורק מספר זה של תיקיות ייווצר מחדש.
core-warning-build-credential-by-now = הושמט מהחבילה: כעת הוא נראה כמו פרטי כניסה.
core-warning-build-no-package-path = לקובץ שניתן להעתקה לא היה נתיב יחסי בחבילה.
# { $file_name } is the runtime installer's file name, such as vc_redist.x64.exe. { $error } is what went wrong.
core-warning-build-prerequisite-path-invalid = לא ניתן היה להציב את הדרישה המוקדמת המצורפת { $file_name }: { $error }
# { $runtime } is a runtime's name, such as Microsoft Visual C++ 2015+ Redistributable (x64). { $error } is what went
# wrong. winget is the name of a Windows command; keep it as it is.
core-warning-build-prerequisite-download-failed = לא ניתן היה להוריד את { $runtime } ({ $error }); תוכנית ההתקנה תנסה להשתמש ב-winget במחשב היעד.
# { $runtime } is a runtime's name. { $reason } is why the check failed, such as "it is signed by Contoso Ltd, not
# Microsoft".
core-warning-build-prerequisite-signature-failed = ההורדה של { $runtime } נכשלה בבדיקת החתימה, ולכן היא לא צורפה: { $reason }. תוכנית ההתקנה תנסה להשתמש ב-winget במחשב היעד.
# { $runtime } is a runtime's name. { $error } is what went wrong.
core-warning-build-prerequisite-hash-failed = { $runtime } הורד, אך לא ניתן היה לחשב לו גיבוב ({ $error }); תוכנית ההתקנה תנסה במקום זאת להשתמש ב-winget במחשב היעד.
# { $service } is a driver's service name, such as AcmeFlt. { $error } is what went wrong.
core-warning-build-driver-copy-failed = לא ניתן היה לצרף את תמונת מנהל ההתקן עבור "{ $service }": { $error }
core-warning-build-license-not-copied = לא ניתן היה להעתיק את נתוני הרישיון (חסרים או לא קריאים); הם נרשמו כמטא-נתונים בלבד.
core-warning-build-license-file-credential = קובץ רישיון נראה כמו פרטי כניסה, והוא הושמט.
# { $error } is what went wrong.
core-warning-build-license-store-walk-failed = לא ניתן היה למנות חלק ממאגר רישיונות: { $error }
core-warning-build-license-store-credential = קובץ במאגר רישיונות נראה כמו פרטי כניסה, והוא הושמט.
# { $error } is what went wrong.
core-warning-build-license-store-file-skipped = דולג על קובץ במאגר רישיונות: { $error }
