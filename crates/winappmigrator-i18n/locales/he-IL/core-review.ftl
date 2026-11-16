# Words the core says about what an install may do: why an install location or something in a package is refused,
# the installer's list of what an app sets up beyond its files, the checks of a package and of an install, and whether
# there is room for either. Message names start with core-.
#
# Many messages here are a reason, the second half of a sentence that starts "... was left out:" or "that install
# location cannot be used:", so they start with a small letter and have no full stop.


## Why an install location cannot be used. Shown in red under the Install Location field.

core-review-location-state-root = זה המקום שבו התקנות שומרות את הרשומות שלהן
core-review-location-has-files = כבר יש בו קבצים, ואין בו התקנה רשומה, ולכן יש להשתמש בתיקיה ריקה
core-policy-location-not-full-path = הוא חייב להיות נתיב מלא
core-policy-location-parent-step = אסור שיכיל ".."
core-policy-location-not-plain = הוא חייב להיות נתיב פשוט בכונן של מחשב זה
core-policy-location-whole-drive = הוא אינו יכול להיות כונן שלם
core-policy-location-windows = הוא אינו יכול להיות התיקיה Windows
core-policy-location-runs-by-itself = זוהי תיקיה ששייכת ל-Windows, או שהתוכן שלה מופעל או נטען מעצמו
core-policy-location-shared = הוא מכיל דברים אחרים זה לצד זה, ולכן האפליקציה צריכה תיקיה בתוכו

## Whether an app can run on this PC at all, said before anything is installed.

# { $reason } is why, as the second half of a sentence, such as "App.exe is built for Arm64, and this PC is x64, so
# Windows would not start it".
core-review-cannot-run-here = אפליקציה זו אינה יכולה לפעול במחשב זה, ולכן שום דבר לא השתנה: { $reason }
core-policy-architecture-x86 = 32 סיביות (x86)
core-policy-architecture-x64 = x64
core-policy-architecture-arm64 = Arm64
core-policy-architecture-arm32 = 32 סיביות (Arm)
core-policy-architecture-unknown = ארכיטקטורה לא ידועה
# { $code } is the number Windows gives a kind of processor, such as 0x01c4.
core-policy-machine-type = סוג מכונה { $code }
# { $names } is a list of program names, such as "a.exe, b.exe, c.exe", and { $count } how many more there are.
core-policy-names-and-more = { $names }, ועוד { $count }
core-policy-fit-programs-not-said = תוכנית התקנה זו אינה מציינת מה התוכניות שלה צריכות, ולכן לא נבדק אם הן יכולות לפעול במחשב זה.
core-policy-fit-architecture-not-told = לא ניתן היה לקבוע מהי הארכיטקטורה של מחשב זה, ולכן לא נבדק אם התוכניות של אפליקציה זו יכולות לפעול בו.
core-policy-fit-main-not-said = תוכנית התקנה זו אינה מציינת מה התוכנית הראשית שלה צריכה, ולכן מה שנאמר כאן מבוסס על שאר מה שיש בה.
# { $driver } is a driver's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-driver = מנהל ההתקן שלה { $driver } בנוי עבור { $built_for }, ומחשב זה הוא { $here }. מנהל התקן חייב להתאים בדיוק ל-Windows, ולכן אפליקציה זו אינה יכולה לפעול כאן
# { $program } is a program's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-program = { $program } בנויה עבור { $built_for }, ומחשב זה הוא { $here }, ולכן Windows לא יפעיל אותה
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-none-start = אף אחת מהתוכניות שלה אינה יכולה לפעול במחשב זה, שהוא { $here }: { $programs }
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-some-other-architecture = חלק מהתוכניות של אפליקציה זו בנויות עבור ארכיטקטורה אחרת ולא יופעלו במחשב זה, שהוא { $here }: { $programs }. שאר האפליקציה יכולה לפעול, כך שייתכן שזה לא משנה.
# { $here } is a kind of processor, such as "Arm64". { $programs } is a list of program names.
core-policy-fit-emulated = מחשב זה הוא { $here }, ולכן Windows יריץ את אלה באמצעות אמולטור: { $programs }. רוב התוכניות פועלות כך כראוי, וחלקן לא.
# { $program } is a program's file name. { $needs } and { $runs } are versions of Windows, such as 10.0 and 6.1.
core-policy-fit-needs-newer = לפי { $program }, היא זקוקה ל-Windows { $needs } ואילך, ומחשב זה מריץ את { $runs }, ולכן Windows לא יפעיל אותה
# { $programs } is a list of program names.
core-policy-fit-some-need-newer = לפי חלק מהתוכניות של אפליקציה זו, הן זקוקות לגרסה חדשה יותר של Windows, והן לא יופעלו כאן: { $programs }.
# { $from_build } and { $this_build } are Windows build numbers, such as 22631, written as Windows writes them.
core-policy-fit-older-build = אפליקציה זו הייתה ב-Build { $from_build } של Windows, ומחשב זה מריץ את Build { $this_build }. לפי התוכניות שלה, הן יכולות לפעול ב-Windows זה, כך שייתכן שהיא תעבוד, אבל ייתכן שמה שהיא הסתמכה עליו שם אינו קיים כאן.
core-policy-fit-build-not-told = לא ניתן היה לקבוע איזה Build של Windows זה, ולכן הוא לא הושווה ל-Build שאפליקציה זו הייתה בו.

## Room on a drive for a build or an install.

# Each of these ends the sentence core-space-shortfall, after "free for".
core-space-for-unpacking = חילוץ החבילה
core-space-for-install-record = רשומת ההתקנה
core-space-for-app-files = הקבצים של האפליקציה
core-space-for-what-is-replaced = שמירת מה שהיא מחליפה
core-space-for-build-work = קובצי העבודה של הבנייה
core-space-for-installer = תוכנית ההתקנה
core-space-for-package = החבילה
core-space-for-unpacking-and-installing = חילוץ החבילה והתקנתה
# What goes between two of those, as in "unpacking the package and the app's files". Keep the quotes: they keep the
# spaces.
core-space-purposes-joiner = { " ו" }
# { $drive } is a drive, such as C:, or a folder. { $needed } and { $free } are sizes, such as 1.2 GB. { $purposes } is
# what the room is for, such as "unpacking the package and the app's files".
core-space-shortfall = ב-{ $drive } נדרשים עד { $needed } פנויים עבור { $purposes }, ופנויים בו { $free }
# { $drive } is a drive, such as C:, or a folder. { $needed } is a size, such as 1.2 GB.
core-space-not-told = { $drive } לא ציין כמה שטח יש בו, ולכן לא נבדק אם ה-{ $needed } שנדרשים לו ייכנסו
# One drive in a list of where room is needed. { $needed } and { $free } are sizes, and { $drive } a drive, such as C:.
core-space-summary = עד { $needed } ב-{ $drive } ({ $free } פנויים)
# { $needed } is a size, and { $drive } a drive, such as C:.
core-space-summary-not-told = עד { $needed } ב-{ $drive } (לא ניתן היה לקבוע כמה שטח פנוי)
# What goes between two sentences of core-space-shortfall inside core-review-no-room. Keep the quotes: they keep the
# space.
core-review-sentence-joiner = { ". " }
# { $shortfalls } is one or more sentences of core-space-shortfall.
core-review-no-room = אין מספיק שטח לכך, ולכן שום דבר לא השתנה. { $shortfalls }. יש לפנות שם מעט שטח, או לבחור מיקום התקנה בכונן אחר.

## The installer's list of what an app sets up beyond its files: what each kind is called.

core-review-kind-service = שירות
core-review-kind-driver = מנהל התקן
core-review-kind-scheduled-task = משימה מתוזמנת
core-review-kind-autostart = הפעלה אוטומטית
core-review-kind-shortcut = קיצור דרך
core-review-kind-path = PATH
core-review-kind-environment-variable = משתנה סביבה
core-review-kind-shell-extension = הרחבת מעטפת
core-review-kind-menu-command = פקודת תפריט
core-review-kind-link-type = סוג קישור
core-review-kind-file-type = סוג קובץ
core-review-kind-com = COM
core-review-kind-firewall-rule = כלל חומת אש
core-review-kind-event-log-source = מקור יומן אירועים
core-review-kind-font = גופן
core-review-kind-odbc-driver = מנהל התקן ODBC
core-review-kind-odbc-source = מקור נתונים ODBC
core-review-kind-compatibility-setting = הגדרת תאימות
core-review-kind-default-apps = אפליקציות ברירת מחדל
core-review-kind-program-name = שם תוכנית
core-review-kind-other-settings = הגדרות אחרות

## The installer's list of what an app sets up: what each line says of it.

# { $file } is a program's file name. { $account } is the Windows account it runs as, such as LocalSystem. { $starts }
# says when it starts, and is not shown itself; for "other", { $start } is one of the core-review-start- words below.
core-review-service-detail =
    { $starts ->
        [automatic] מריץ את { $file } בתור { $account }, מתחיל לפעול כעת ובכל הפעלה של מחשב זה
        [delayed] מריץ את { $file } בתור { $account }, מתחיל לפעול כעת וזמן קצר אחרי כל הפעלה של מחשב זה
        [manual] מריץ את { $file } בתור { $account }, מתחיל לפעול רק כשמשהו מבקש זאת
        [disabled] מריץ את { $file } בתור { $account }, מושבת
        [other] מריץ את { $file } בתור { $account }, מועד ההפעלה: { $start }
       *[unknown] מריץ את { $file } בתור { $account }
    }
# { $account } is a Windows account, such as LocalService.
core-policy-account-custom = { $account } (חשבון מותאם אישית שאינו משוחזר)
# { $file } is a driver's file name.
core-review-driver-detail = טוען את { $file } לתוך Windows עצמו
# { $file } is a driver's file name. { $start } is when Windows starts it: one of the core-review-start- words below.
core-review-driver-detail-starts = טוען את { $file } לתוך Windows עצמו, מועד ההפעלה: { $start }
# When a service or driver starts, ending a sentence such as "Loads disk.sys into Windows itself, starts …".
core-review-start-boot = יחד עם Windows, לפני כל דבר אחר
core-review-start-system = יחד עם Windows, בזמן האתחול שלו
core-review-start-automatic = בכל הפעלה של מחשב זה
core-review-start-delayed = זמן קצר אחרי כל הפעלה של מחשב זה
core-review-start-manual = רק כשמשהו מבקש זאת
core-review-start-disabled = אף פעם, מכיוון שהוא מושבת
core-review-start-unknown = לא מצוין על ידי Windows
# { $account } is the account a task runs as: an account's name, or core-review-task-runs-as-you, or
# core-review-task-runs-as-creator.
core-review-task-detail = פועלת בתור { $account }
# { $account } is the account a task runs as, as for core-review-task-detail.
core-review-task-detail-highest = פועלת בתור { $account }, עם ההרשאות הגבוהות ביותר שהיא יכולה לקבל
core-review-task-runs-as-you = המשתמש המחובר
core-review-task-runs-as-creator = המשתמש שיצר אותה
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-autostart-detail =
    פועל { $scope ->
        [everyone] בכל כניסה של כל משתמש
       *[you] בכל כניסה שלך
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shortcut-startup-detail =
    קיצור דרך בתיקיית האתחול, { $scope ->
        [everyone] עבור כל המשתמשים במחשב זה
       *[you] עבורך
    }
# { $file } is the file name of what the shortcut opens. { $scope } is whom it is set up for: everyone on this PC,
# or you. It is not shown itself.
core-review-shortcut-detail =
    פותח את { $file }, { $scope ->
        [everyone] עבור כל המשתמשים במחשב זה
       *[you] עבורך
    }
# { $name } is an environment variable's name, such as PATH.
core-review-path-title = מוסיף ל-{ $name }
core-review-path-install-folder = תיקיית ההתקנה
# { $path } is a folder inside the install folder, such as bin\tools.
core-review-path-below-install-folder = תיקיית ההתקנה\{ $path }
# { $value } is the variable's value, as it will be set.
core-review-variable-detail = מוגדר לערך { $value }
# { $count } is how many registry values.
core-review-other-settings-title =
    { $count ->
        [one] הגדרה אחת מחוץ למפתחות של האפליקציה עצמה
        [two] שתי הגדרות מחוץ למפתחות של האפליקציה עצמה
       *[other] { $count } הגדרות מחוץ למפתחות של האפליקציה עצמה
    }
# { $keys } is a list of registry keys, such as HKCU\Software\Vendor.
core-review-other-settings-detail = ב-{ $keys }
# { $keys } is a list of registry keys, and { $count } how many more there are.
core-review-other-settings-detail-more = ב-{ $keys }, ועוד { $count }
# { $way } says whether the rule allows or blocks, and incoming or outgoing connections; it is not shown itself.
# { $ports } is a list of port numbers, and { $port_count } how many. { $program } is the program's file name.
# { $networks } is a list of the kinds of network Windows names in English (domain, private, public), and
# { $network_count } how many.
# The kinds of network a firewall rule is for, as Windows Defender Firewall names its profiles in your language. They
# fill { $networks } in core-review-firewall-detail, joined into a list.
core-review-network-domain = תחום
core-review-network-private = פרטי
core-review-network-public = ציבורי
core-review-firewall-detail =
    { $way ->
        [allow-in] מתיר חיבורים נכנסים
        [allow-out] מתיר חיבורים יוצאים
        [block-in] חוסם חיבורים נכנסים
       *[block-out] חוסם חיבורים יוצאים
    }{ $port_count ->
        [0] { "" }
        [one] { " " }ביציאה { $ports }
        [two] { " " }ביציאות { $ports }
       *[other] { " " }ביציאות { $ports }
    } עבור { $program }, { $network_count ->
        [0] בכל רשת
        [one] ברשת מסוג { $networks }
        [two] ברשתות מסוג { $networks }
       *[other] ברשתות מסוג { $networks }
    }
# { $server } is the file name of what Explorer loads. { $scope } is whom it is set up for: everyone on this PC, or
# you. It is not shown itself.
core-review-shell-extension-detail =
    טוען את { $server } לתוך סייר הקבצים, { $scope ->
        [everyone] עבור כל המשתמשים במחשב זה
       *[you] עבורך
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shell-extension-detail-own-file =
    טוען לתוך סייר הקבצים קובץ שהחבילה מתקינה, { $scope ->
        [everyone] עבור כל המשתמשים במחשב זה
       *[you] עבורך
    }
# The name of a shell extension: { $handler } is its name in the registry, and { $class } the kind of file or folder it
# is for, such as * or Directory.
core-policy-shell-extension-name = { $handler } עבור { $class }
# { $on } is the kind of file or folder whose menu it is, as the registry names it, such as * or
# Directory. { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-menu-command-detail =
    מוסיף פקודה לתפריט של { $on }, { $scope ->
        [everyone] עבור כל המשתמשים במחשב זה
       *[you] עבורך
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-default-verb-detail =
    משנה את מה שקורה בפתיחת פריט מסוג זה, { $scope ->
        [everyone] עבור כל המשתמשים במחשב זה
       *[you] עבורך
    }
core-review-client-entry-detail = ניתן לבחור בו כתוכנית לכך. שום הגדרה אינה מועברת אליו
core-review-com-permissions-title = הרשאות COM
core-review-com-permissions-detail = קובע מי רשאי להפעיל את מחלקות ה-COM שלו, או בתור מה הן פועלות
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-link-type-detail =
    קישורים מסוג זה נפתחים באמצעות האפליקציה, { $scope ->
        [everyone] עבור כל המשתמשים במחשב זה
       *[you] עבורך
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-file-type-detail =
    קבצים מסוג זה נפתחים באמצעות האפליקציה, { $scope ->
        [everyone] עבור כל המשתמשים במחשב זה
       *[you] עבורך
    }
# { $log } is the name of a Windows event log, such as Application.
core-review-event-log-source-detail = יכול לכתוב ליומן { $log }, עם הודעות מהקבצים שלו
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-font-detail =
    נטען מתיקיית ההתקנה בעת הכניסה, { $scope ->
        [everyone] עבור כל המשתמשים במחשב זה
       *[you] עבורך
    }
core-review-odbc-driver-detail = נטען לתוך תוכניות שמשתמשות ב-ODBC כדי לגשת למסד נתונים
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-odbc-source-detail =
    חיבור למסד נתונים, { $scope ->
        [everyone] עבור כל המשתמשים במחשב זה
       *[you] עבורך
    }
core-review-default-apps-detail = ניתן לבחור בו ב"אפליקציות ברירת מחדל". שום הגדרה אינה מועברת אליו
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-program-name-detail =
    "הפעלה" ותוכניות אחרות יכולות למצוא אותו לפי שם זה, { $scope ->
        [everyone] עבור כל המשתמשים במחשב זה
       *[you] עבורך
    }
# { $server } is the file name of what serves the classes.
core-review-com-title = COM ב-{ $server }
core-review-com-title-own-file = COM בקובץ שהחבילה מתקינה
# { $count } is how many registry values.
core-review-com-detail =
    { $count ->
        [one] הגדרה אחת של המחלקות שתוכניות אחרות יכולות להפעיל
        [two] שתי הגדרות של המחלקות שתוכניות אחרות יכולות להפעיל
       *[other] { $count } הגדרות של המחלקות שתוכניות אחרות יכולות להפעיל
    }
core-review-com-support-title = ספריות סוגים, ממשקים ו-AppIDs של COM
# { $libraries }, { $interfaces }, and { $app_ids } are how many registry values there are for its type libraries, its
# interfaces, and its AppIDs.
core-review-com-support-detail = הגדרות שנלוות למחלקות שלו: ספריות סוגים ({ $libraries }), ממשקים ({ $interfaces }) ו-AppIDs ({ $app_ids })
core-review-compatibility-title = האופן שבו התוכניות שלה מופעלות
# { $count } is how many registry values.
core-review-compatibility-detail =
    { $count ->
        [one] הגדרת תאימות אחת לתוכניות שהחבילה מתקינה
        [two] שתי הגדרות תאימות לתוכניות שהחבילה מתקינה
       *[other] { $count } הגדרות תאימות לתוכניות שהחבילה מתקינה
    }

## The installer's notes on whose folders and settings these are, and what is not coming.

# { $account } is the name of a Windows account, such as alice.
core-review-account-signed-in = נתוני המשתמש וההגדרות לכל משתמש יועברו ל-{ $account }, שמחובר כאן, ולא לחשבון מנהל המערכת שתחתיו פועלת תוכנית זו.
# { $account } is the name of a Windows account, such as alice.
core-review-account-no-desktop = אף משתמש לא מחובר להפעלה זו, ולכן נתוני המשתמש וההגדרות לכל משתמש יועברו ל-{ $account }, החשבון שתחתיו פועלת תוכנית זו.
# { $reason } is what Windows said, in its own words.
core-review-account-not-told = לא ניתן היה לקבוע מי מחובר ({ $reason }), ולכן ההגדרות לכל משתמש מושמטות, ונתוני המשתמש מועברים לתוך מיקום ההתקנה.
# { $reason } is what Windows said, in its own words.
core-review-signed-in-unknown = לא ניתן היה לקבוע מי מחובר: { $reason }
# Headings of what is left out, shared by the app's list and the installer's notes.
core-review-left-out-credentials = לא כלולים בתוכנית התקנה זו, מכיוון שהם נראים כמו פרטי כניסה
core-review-left-out-settings = הגדרות והיסטוריה שחשבון שומר עבור האפליקציה, שנכללות רק עם האפשרות "נתוני משתמש"
core-review-left-out-certificates = נראה שהאפליקציה מסתמכת על אישורים שהוגדרו כמהימנים במחשב שלה, והם אינם מותקנים
core-review-left-out-fonts = גופנים אלה נקראים על שם האפליקציה, ושום דבר אינו מציין שהם שלה, ולכן הם אינם מותקנים
# { $names } is a list of names, such as the names of saved passwords.
core-review-note-credentials = לא כלולים בתוכנית התקנה זו, מכיוון שהם נראים כמו פרטי כניסה: { $names }. מה שקיים מהם במחשב זה נשאר כפי שהוא, וייתכן שיהיה צורך להזין אותם שוב.
# { $names } is a list of names of license data.
core-review-note-licenses = לא כלולים בתוכנית התקנה זו, מכיוון שהאפשרות "נתוני רישיון והפעלה" הייתה כבויה כשהיא נבנתה: { $names }.
# { $names } is a list of places an account keeps settings, such as folders.
core-review-note-settings = לא כלולים בתוכנית התקנה זו, מכיוון שהאפשרות "נתוני משתמש" הייתה כבויה כשהיא נבנתה: ההגדרות וההיסטוריה שחשבון שומר עבור האפליקציה, ב-{ $names }. האפליקציה תופעל כאן כמו אצל מי שמשתמש בה לראשונה.
# { $names } is a list of certificates, each as core-review-certificate.
core-review-note-certificates = נראה שהאפליקציה מסתמכת על אישורים שהוגדרו כמהימנים במחשב שלה, והם אינם מותקנים: { $names }. ההחלטה אם לתת בהם אמון כאן היא שלך.
# { $names } is a list of font names.
core-review-note-fonts = גופנים אלה נקראים על שם האפליקציה, ושום דבר אינו מציין שהם שלה, ולכן הם אינם מותקנים: { $names }.
# { $names } is a list of names.
core-review-note-left-out-by-choice = הושמטו על ידי מי שבנה את תוכנית ההתקנה: { $names }.
# { $names } is a list of names.
core-review-note-added-by-choice = נוספו על ידי מי שבנה את תוכנית ההתקנה, מכיוון שהם נחשבו לחלק מהאפליקציה: { $names }.
# { $name } is a certificate's subject, such as CN=Contoso, and { $store } the certificate store it was in, such as
# Root.
core-review-certificate = { $name } ב-{ $store }
# { $names } is a list of names, and { $count } how many more there are.
core-review-names-and-more = { $names }, ועוד { $count }

## Why something in a package is left out. Each is the second half of a sentence of core-warning-review-left-out.

core-review-no-account = הוא שייך להגדרות של חשבון אחד, ולא ניתן היה לקבוע מי מחובר
core-review-name-not-accepted = השם שלו לא התקבל על ידי Windows
# { $error } is what Windows said, in its own words.
core-review-service-unreadable = לא ניתן היה לקרוא את ההגדרות הנוכחיות שלו: { $error }
core-review-service-dependency = הוא תלוי בשירות בעל שם לא חוקי
# { $path } is the program the service runs.
core-review-service-runs-other = הוא מריץ את { $path }, שחבילה זו אינה מתקינה
core-review-service-settings = ההגדרות שלו מציינות משהו שחבילה זו אינה מתקינה, או מפתח שאינו שלו
core-review-driver-no-hash = אין לו גיבוב שלפיו ניתן לבדוק אותו
core-review-license-for-the-machine = רישיון שמאוחסן עבור המחשב כולו רק מדווח
core-review-license-starts-something = אין לרישיון סיבה להפעיל משהו
core-review-task-not-captured = ההגדרה שלה לא נלכדה
# { $error } is what was wrong with the task's XML, in English.
core-review-task-unreadable = לא ניתן היה לקרוא את ההגדרה שלה: { $error }
core-review-task-no-account = היא פועלת בתור מי שמתקין אותה, ולא ניתן היה לקבוע מי מחובר כאן
# { $path } is what the shortcut opens.
core-review-shortcut-opens-other = הוא פותח את { $path }, שחבילה זו אינה מתקינה
# { $service } is the service's name, and { $path } the program it runs now.
core-policy-service-exists = { $service } כבר קיים במחשב זה ומריץ את { $path }, שאינו מהתקנה קודמת כאן. אם זהו עותק ישן יותר של אפליקציה זו, יש להסיר את ההתקנה שלו תחילה
core-policy-key-control-characters = שם המפתח שלו מכיל תווי בקרה
core-policy-value-control-characters = שם הערך שלו מכיל תווי בקרה
core-policy-registry-link = זהו קישור רישום
# { $reason } is why the key it stands for is left out, such as "no app install should change this key".
core-policy-virtual-store = הוא מייצג מפתח של המחשב, ובנוסף { $reason }
core-policy-registry-part = אף התקנה של אפליקציה לא אמורה לכתוב לחלק זה של הרישום
core-policy-key-refused = אף התקנה של אפליקציה לא אמורה לשנות מפתח זה
core-policy-net-debugger = הוא יקבע מאתר באגים לכל תוכנית .NET
core-policy-net-install-root = הוא ישנה את המקום שממנו כל תוכנית .NET טוענת את .NET
core-policy-class-redirects = הוא ישנה את האופן שבו סוג של תוכנית או תיקיה נפתח
core-policy-app-path-name = הוא מציין תוכנית לפי משהו שאינו שם של קובץ
core-policy-app-path-not-installed = הוא ירשום שם תוכנית עבור קובץ שחבילה זו אינה מתקינה
core-policy-shared-default-verb = הוא ישנה את מה שקורה בפתיחה של כל קובץ או תיקיה מסוג מסוים
core-policy-command-runs-other = הוא מוסיף פקודה שמריצה משהו שחבילה זו אינה מתקינה
core-policy-autostart-not-command = הוא מופעל אוטומטית, והערך שלו אינו פקודה
core-policy-autostart-runs-other = הוא מופעל אוטומטית, ומריץ משהו שחבילה זו אינה מתקינה
core-policy-environment-own-rules = משתני סביבה משוחזרים לפי הכללים שלהם
core-policy-service-not-restored = הוא שייך לשירות שחבילה זו אינה משחזרת
core-policy-service-own-rules = ההגדרות של שירות משוחזרות רק לפי הכללים שלו
core-policy-service-dll = ה-ServiceDll שלו אינו קובץ שחבילה זו מתקינה
core-policy-event-log-source-only = ניתן לשחזר רק את הערכים של מקור יומן האירועים עצמו
core-policy-security-log = אף אפליקציה לא כותבת ליומן האבטחה
core-policy-event-log-messages = ההודעות שלו יגיעו מקובץ שחבילה זו אינה מתקינה
core-policy-type-library = ספריית הסוגים שלו אינה קובץ שחבילה זו מתקינה
# { $name } is a program's file name, such as app.exe.
core-policy-application-opened = הוא ישנה את האופן שבו { $name } נפתח, וחבילה זו אינה מתקינה אותו
core-policy-event-log-no-file = מקור יומן אירועים חייב לקחת את ההודעות שלו מקובץ
core-policy-com-runs-other = הוא מריץ משהו שאינו קובץ שחבילה זו מתקינה
core-policy-com-redirect = הוא יהפוך מחלקה זו למשהו שחבילה זו אינה מתקינה
core-policy-com-server = הוא מריץ משהו שאינו קובץ שחבילה זו מתקינה ואינו חלק מ-Windows
core-policy-interface = הוא יקבע את האופן שבו מתבצעות קריאות לממשק, באמצעות משהו שחבילה זו אינה מתקינה
# { $name } is a program's file name, such as app.exe.
core-policy-app-id-started = הוא ישנה את האופן שבו { $name } מופעל, וחבילה זו אינה מתקינה אותו
core-policy-app-id-surrogate = הוא יתארח בתוכנית שחבילה זו אינה מתקינה
core-policy-app-id-service = הוא יפעל בשירות שחבילה זו אינה משחזרת
core-policy-app-id-run-as = הוא יפעל בתור חשבון משלו
core-policy-app-id-link = הוא אינו מציין AppID
core-policy-app-id-remote = הוא יפעל במקום אחר שאינו מחשב זה
core-policy-odbc-own-driver = זוהי הגדרה של ODBC עצמו ולא מנהל התקן
core-policy-odbc-driver-not-installed = מנהל ההתקן שלו אינו קובץ שחבילה זו מתקינה
core-policy-loads-other-file = הוא יטען קובץ שחבילה זו אינה מתקינה
core-policy-odbc-own-source = זוהי הגדרה של ODBC עצמו ולא מקור נתונים
core-policy-odbc-source-missing = הוא מפרט מקור נתונים שאינו קיים בחבילה זו
core-policy-odbc-source-driver = מנהל ההתקן שלו אינו קובץ שחבילה זו מתקינה ואינו חלק מ-Windows
core-policy-shell-extension = הוא יטען לתוך סייר הקבצים הרחבת מעטפת שאינה קובץ שחבילה זו מתקינה
core-policy-not-text = התוכן שלו אינו טקסט מהסוג שבו הוא ייקרא
core-policy-network-place = הוא מציין מיקום ברשת
core-policy-font = הוא ירשום גופן שחבילה זו אינה מתקינה, או גופן שנמצא בתיקיה Windows
core-policy-compatibility = הוא ישנה את אופן הפעולה של תוכנית שחבילה זו אינה מתקינה
core-policy-capabilities = הוא מפרט אפליקציה שהפרטים שלה אינם בחבילה זו
core-policy-default-program = הוא ישנה איזו תוכנית פותחת דברים כברירת מחדל
core-policy-url-protocol = הוא מציין שהוא סוג קישור, ואינו נפתח באמצעות שום דבר שחבילה זו מתקינה
core-policy-command-handler = הוא מוסיף פקודה שמטופלת על ידי משהו שחבילה זו אינה מתקינה
core-policy-installer-command = הוא יריץ משהו שרק Windows Installer יכול לקרוא
core-policy-creates-class = הוא ייצור מחלקה שחבילה זו אינה מתקינה
core-policy-opens-links = הוא פותח קישורים באמצעות משהו שחבילה זו אינה מתקינה
core-policy-opens-files = הוא פותח קבצים באמצעות משהו שחבילה זו אינה מתקינה
core-policy-firewall-name = השם שלו אינו שם שניתן להעביר הלאה
core-policy-firewall-description = התיאור שלו אינו תיאור שניתן להעביר הלאה
core-policy-firewall-every-program = הוא חל על כל התוכניות, וכלל יכול להיות רק עבור תוכנית אחת
# { $path } is the program the rule is for.
core-policy-firewall-program = הוא מיועד ל-{ $path }, שחבילה זו אינה מתקינה
core-policy-firewall-direction = הוא אינו מציין כיוון שניתן להגדיר
core-policy-firewall-action = הוא יעקוף כללים אחרים, ואף כלל של אפליקציה לא אמור לעשות זאת
# { $setting } is a part of a firewall rule, as the scan of its PC described it.
core-policy-firewall-not-carried = הוא מציין { $setting }, שלא ניתן היה להגדיר כפי שהיה, או שהיה מחליש את מה שכללים אחרים עושים
core-policy-firewall-service = הוא מיועד לשירות שחבילה זו אינה משחזרת
core-policy-firewall-parts = חלק ממנו אינו דבר שכלל חומת אש יכול לציין
core-policy-folder-already-there = תיקיה זו כבר הייתה קיימת, מחוץ למיקום ההתקנה, ושום דבר אינו מציין שהיא שייכת לאפליקציה זו, ולכן לא מוצבת בה שום תוכנית
core-policy-variable-name = השם שלו אינו שם משתנה חוקי
core-policy-variable-relied-on = על משתנה זה מסתמכים Windows ותוכניות אחרות
core-policy-task-name = השם שלה אינו שם ש-Windows מקבל
core-policy-task-folder = התיקיה שלה אינה תיקיה ש-Windows מקבל
core-policy-task-windows = היא תחליף אחת מהמשימות של Windows עצמו
core-policy-task-other-action = היא עושה משהו אחר מלבד הפעלת תוכנית
core-policy-task-no-program = היא אינה מציינת תוכנית להפעלה
# { $path } is the program the task runs.
core-policy-task-runs-other = היא מריצה משהו שחבילה זו אינה מתקינה: { $path }
core-policy-task-exists-unreadable = משימה בשם זה כבר קיימת ולא ניתן היה לקרוא אותה
core-policy-task-exists-other = משימה בשם זה כבר קיימת ואינה מפעילה את האפליקציה
# { $path } is the program the task already on this PC runs.
core-policy-task-exists-runs = משימה בשם זה כבר קיימת ומריצה את { $path }, שאינו מהתקנה קודמת כאן
core-policy-shortcut-not-lnk = הוא אינו קיצור דרך מסוג .lnk
core-policy-shortcut-place = הוא אינו בתפריט התחלה או בשולחן העבודה
core-policy-short-name = הנתיב שלו משתמש בשם שלא ניתן לבדוק
core-policy-shortcut-no-folder = במחשב זה אין תיקיה כזו עבורו
core-policy-data-startup = הוא יוצב בתיקיית אתחול, ויופעל בכל כניסה
core-policy-data-powershell = הוא יוצב במקום שבו PowerShell מריץ את מה שהוא מוצא כשהוא מופעל
core-policy-data-windows = הוא יוצב בתיקיה ששייכת ל-Windows
core-policy-license-outside = הוא נמצא מחוץ למיקום ההתקנה ולכל תיקיית נתונים ידועה
core-rewrite-cdata = הוא מכיל מקטע CDATA
core-rewrite-too-much-to-look-through = חבילה זו מכילה בהגדרות שלה יותר מדי מכדי שניתן יהיה לבדוק את מה שהן מציינות

## Warnings a review gives, which the installer shows with what it sets up.

# { $kind } says what was left out, and is not shown itself. { $item } is its path or name, or a registry value such
# as HKLM\Software\App\Name. { $reason } is why, as the second half of the sentence, such as "it cannot be a whole
# drive".
core-warning-review-left-out =
    { $kind ->
        [file] הקובץ { $item } הושמט: { $reason }.
        [folder] התיקיה { $item } הושמטה: { $reason }.
        [service] השירות { $item } הושמט: { $reason }.
        [driver] מנהל ההתקן { $item } הושמט: { $reason }.
        [firewall-rule] כלל חומת האש { $item } הושמט: { $reason }.
        [license-data] נתוני הרישיון ב-{ $item } הושמטו: { $reason }.
        [license-value] ערך הרישיון { $item } הושמט: { $reason }.
        [scheduled-task] המשימה המתוזמנת { $item } הושמטה: { $reason }.
        [shortcut] קיצור הדרך { $item } הושמט: { $reason }.
        [environment-variable] משתנה הסביבה { $item } הושמט: { $reason }.
       *[value] { $item } הושמט: { $reason }.
    }
core-warning-review-package-path-missing = לקובץ שניתן להעתקה אין נתיב יחסי בחבילה.
# { $count } is the most empty folders a package makes.
core-warning-review-too-many-empty-folders = חבילה זו מפרטת יותר מ-{ $count } תיקיות ריקות, ורק מספר זה של תיקיות נוצר.
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Documents folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-documents-elsewhere =
    { $count ->
        [one] קובץ אחד מהתיקיה "מסמכים" מועבר במקום זאת לתוך { $target }, לתיקיה בשם _external, מכיוון שהתיקיה "מסמכים" של מחשב זה נמצאת ב-{ $folder }, ונכתבות רק תיקיות שנמצאות בתוך הפרופיל שלך. כדי להשתמש בו, יש להעביר אותו בעצמך לתיקיה "מסמכים" שלך.
        [two] שני קבצים מהתיקיה "מסמכים" מועברים במקום זאת לתוך { $target }, לתיקיה בשם _external, מכיוון שהתיקיה "מסמכים" של מחשב זה נמצאת ב-{ $folder }, ונכתבות רק תיקיות שנמצאות בתוך הפרופיל שלך. כדי להשתמש בהם, יש להעביר אותם בעצמך לתיקיה "מסמכים" שלך.
       *[other] { $count } קבצים מהתיקיה "מסמכים" מועברים במקום זאת לתוך { $target }, לתיקיה בשם _external, מכיוון שהתיקיה "מסמכים" של מחשב זה נמצאת ב-{ $folder }, ונכתבות רק תיקיות שנמצאות בתוך הפרופיל שלך. כדי להשתמש בהם, יש להעביר אותם בעצמך לתיקיה "מסמכים" שלך.
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-documents-unknown =
    { $count ->
        [one] קובץ אחד מהתיקיה "מסמכים" מועבר במקום זאת לתוך { $target }, לתיקיה בשם _external, מכיוון שלא ניתן היה לקבוע היכן נמצאת התיקיה "מסמכים" של מחשב זה. כדי להשתמש בו, יש להעביר אותו בעצמך לתיקיה "מסמכים" שלך.
        [two] שני קבצים מהתיקיה "מסמכים" מועברים במקום זאת לתוך { $target }, לתיקיה בשם _external, מכיוון שלא ניתן היה לקבוע היכן נמצאת התיקיה "מסמכים" של מחשב זה. כדי להשתמש בהם, יש להעביר אותם בעצמך לתיקיה "מסמכים" שלך.
       *[other] { $count } קבצים מהתיקיה "מסמכים" מועברים במקום זאת לתוך { $target }, לתיקיה בשם _external, מכיוון שלא ניתן היה לקבוע היכן נמצאת התיקיה "מסמכים" של מחשב זה. כדי להשתמש בהם, יש להעביר אותם בעצמך לתיקיה "מסמכים" שלך.
    }
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Saved Games folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-elsewhere =
    { $count ->
        [one] קובץ אחד מהתיקיה "משחקים שמורים" מועבר במקום זאת לתוך { $target }, לתיקיה בשם _external, מכיוון שהתיקיה "משחקים שמורים" של מחשב זה נמצאת ב-{ $folder }, ונכתבות רק תיקיות שנמצאות בתוך הפרופיל שלך. כדי להשתמש בו, יש להעביר אותו בעצמך לתיקיה "משחקים שמורים" שלך.
        [two] שני קבצים מהתיקיה "משחקים שמורים" מועברים במקום זאת לתוך { $target }, לתיקיה בשם _external, מכיוון שהתיקיה "משחקים שמורים" של מחשב זה נמצאת ב-{ $folder }, ונכתבות רק תיקיות שנמצאות בתוך הפרופיל שלך. כדי להשתמש בהם, יש להעביר אותם בעצמך לתיקיה "משחקים שמורים" שלך.
       *[other] { $count } קבצים מהתיקיה "משחקים שמורים" מועברים במקום זאת לתוך { $target }, לתיקיה בשם _external, מכיוון שהתיקיה "משחקים שמורים" של מחשב זה נמצאת ב-{ $folder }, ונכתבות רק תיקיות שנמצאות בתוך הפרופיל שלך. כדי להשתמש בהם, יש להעביר אותם בעצמך לתיקיה "משחקים שמורים" שלך.
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-unknown =
    { $count ->
        [one] קובץ אחד מהתיקיה "משחקים שמורים" מועבר במקום זאת לתוך { $target }, לתיקיה בשם _external, מכיוון שלא ניתן היה לקבוע היכן נמצאת התיקיה "משחקים שמורים" של מחשב זה. כדי להשתמש בו, יש להעביר אותו בעצמך לתיקיה "משחקים שמורים" שלך.
        [two] שני קבצים מהתיקיה "משחקים שמורים" מועברים במקום זאת לתוך { $target }, לתיקיה בשם _external, מכיוון שלא ניתן היה לקבוע היכן נמצאת התיקיה "משחקים שמורים" של מחשב זה. כדי להשתמש בהם, יש להעביר אותם בעצמך לתיקיה "משחקים שמורים" שלך.
       *[other] { $count } קבצים מהתיקיה "משחקים שמורים" מועברים במקום זאת לתוך { $target }, לתיקיה בשם _external, מכיוון שלא ניתן היה לקבוע היכן נמצאת התיקיה "משחקים שמורים" של מחשב זה. כדי להשתמש בהם, יש להעביר אותם בעצמך לתיקיה "משחקים שמורים" שלך.
    }

## Errors when a folder that only administrators may change cannot be made so, or checked.

# { $path } is a folder.
core-privilege-not-a-folder = { $path } הוא קישור או קובץ, ולא תיקיה
# { $path } is a folder. { $error } is what Windows said, in its own words.
core-privilege-not-made-here = התיקיה { $path } לא נוצרה על ידי WinAppMigrator, ואחרים יכולים לשנות את מה שיש בה. מנהל מערכת צריך לבדוק אותה ולהסיר אותה: { $error }
# { $path } is a folder.
core-privilege-others-put-something-in = בתיקיה { $path } יש משהו שלא נוצר על ידי WinAppMigrator, ואחרים יכולים לשנות אותו. מנהל מערכת צריך לבדוק אותה ולהסיר אותה
# { $path } is a folder.
core-privilege-became-a-link-in-use = התיקיה { $path } הפכה לקישור בזמן השימוש בה
# { $path } is a folder.
core-privilege-became-a-link-looked-at = התיקיה { $path } הפכה לקישור בזמן שנבדקה
# { $path } is a file. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-users-cannot-start = לא ניתן היה לאפשר למשתמשים להפעיל את { $path }: { $said }
core-privilege-elevation-unknown = לא ניתן היה לקבוע אם תהליך זה פועל עם הרשאות מוגברות
# { $path } is a folder. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-cannot-restrict = לא ניתן היה להגביל את { $path } למנהלי מערכת: { $said }
# { $path } is a folder. { $said } is the folder's permissions as Windows wrote them, and what else it printed.
core-privilege-not-restricted = לא ניתן היה להגביל את { $path } למנהלי מערכת, וייתכן שהכונן שלה אינו שומר הרשאות: { $said }
# { $path } is a folder.
core-privilege-written-to-first = תהליך אחר כתב אל { $path } לפני שניתן היה להגן עליה
core-privilege-no-program-data = מיקום התיקיה ProgramData אינו מצוין על ידי Windows

## Checks of a package's files, and of what an install put in place. Each is said of one file, value, service, or the
## like, which the report names beside it.

# { $path } is the installer's file.
core-verify-no-package = ב-{ $path } אין חבילה
# { $path } is a folder.
core-verify-not-unpacked = החבילה של תוכנית התקנה זו לא חולצה אל { $path }
core-verify-schema-supported = סכימת המניפסט של החבילה נתמכת.
core-verify-packaged-file-missing = קובץ שנארז בחבילה חסר.
core-verify-no-package-path = לקובץ שניתן להעתקה אין נתיב יחסי בחבילה.
core-verify-folder-there = התיקיה הריקה קיימת.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-unreadable = לא ניתן היה לקרוא אותו כדי לבדוק אותו: { $error }
core-verify-restored-file-missing = הקובץ המשוחזר חסר.
# { $expected } and { $actual } are sizes in bytes.
core-verify-size-mismatch = אי-התאמה בגודל הקובץ. הגודל הצפוי: { $expected } בתים, הגודל שנמצא: { $actual } בתים.
core-verify-no-hash = המניפסט אינו מתעד גיבוב עבור קובץ זה, ולכן לא ניתן לבדוק אותו.
core-verify-hash-mismatch = אי-התאמה בגיבוב SHA-256.
core-verify-file-matches = הגודל והגיבוב של הקובץ תואמים למניפסט.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-payload-failed = אימות מטען הרישיון נכשל: { $error }
core-verify-license-payload-missing = מטען הרישיון שבחבילה חסר.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-failed = אימות מאגר הרישיונות נכשל: { $error }
core-verify-license-store-file-missing = קובץ מאגר הרישיונות שבחבילה חסר.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-path-invalid = נתיב מאגר הרישיונות שבחבילה אינו חוקי: { $error }
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-prerequisite-failed = אימות המטען של הדרישה המוקדמת נכשל: { $error }
core-verify-prerequisite-missing = המטען של הדרישה המוקדמת המצורפת חסר.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-failed = אימות תמונת מנהל ההתקן נכשל: { $error }
core-verify-bundled-driver-missing = תמונת מנהל ההתקן המצורפת חסרה.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-path-invalid = נתיב מנהל ההתקן המצורף אינו חוקי: { $error }
core-verify-prerequisite-present = הדרישה המוקדמת של זמן הריצה קיימת ביעד.
core-verify-prerequisite-not-detected = הדרישה המוקדמת של זמן הריצה לא זוהתה; ייתכן שהיא דורשת הפעלה מחדש או השלמה ידנית (התוצאה הקובעת מופיעה ביומן השחזור).
core-verify-prerequisite-not-probed = לא ניתן לבדוק דרך הרישום אם הדרישה המוקדמת קיימת; ההתקנה בוצעה במידת האפשר.
core-verify-license-file-missing = קובץ הרישיון המשוחזר חסר.
core-verify-license-store-restored-missing = קובץ מאגר הרישיונות המשוחזר חסר.
core-verify-registry-key-there = מפתח הרישום קיים.
core-verify-registry-value-matches = ערך הרישום תואם לנתונים שבארכיון.
core-verify-registry-value-differs = ערך הרישום אינו תואם לנתונים שבארכיון.
core-verify-firewall-rule-there = כלל זה קיים בחומת האש של Windows.
core-verify-variable-matches = ערך משתנה הסביבה תואם לנתונים שבארכיון.
core-verify-variable-differs = ערך משתנה הסביבה אינו תואם לנתונים שבארכיון.
# { $error } is what Windows said, in its own words.
core-verify-variable-unreadable = משתנה הסביבה חסר או לא קריא: { $error }
core-verify-service-runs-other = השירות קיים, אך אינו מריץ את הקובץ הבינארי המשוחזר עם הארגומנטים שלו.
core-verify-service-other-account = השירות קיים, אך אינו פועל בתור החשבון הצפוי.
core-verify-service-matches = השירות קיים, ומריץ את הקובץ הבינארי המשוחזר עם הארגומנטים והחשבון שלו.
core-verify-service-missing = השירות חסר.
# { $error } is what Windows said, in its own words.
core-verify-service-unreadable = לא ניתן לקרוא את השירות: { $error }
core-verify-driver-other-image = שירות מנהל ההתקן אינו מצביע על התמונה המשוחזרת.
core-verify-driver-not-registered = שירות מנהל ההתקן המצורף אינו רשום לאחר השחזור.
# { $error } is what Windows said, in its own words.
core-verify-driver-unreadable = לא ניתן לקרוא את שירות מנהל ההתקן: { $error }
core-verify-driver-image-missing = תמונת מנהל ההתקן חסרה.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-unchecked = לא ניתן היה לבדוק את תמונת מנהל ההתקן: { $error }
core-verify-task-matches = המשימה המתוזמנת קיימת ומריצה את התוכניות המשוחזרות.
core-verify-task-runs-other = המשימה המתוזמנת קיימת, אך אינה מריצה את התוכניות המשוחזרות.
core-verify-task-missing = המשימה המתוזמנת חסרה.
# { $error } is what Windows said, in its own words.
core-verify-task-unreadable = השאילתה על המשימה המתוזמנת נכשלה: { $error }
core-verify-shortcut-matches = קיצור הדרך קיים, ונתיב היעד שלו תואם ליעד המשוחזר.
core-verify-shortcut-opens-other = קיצור הדרך קיים, אך נתיב היעד שלו אינו תואם ליעד המשוחזר.
core-verify-shortcut-missing = קיצור הדרך חסר.
# { $error } is what Windows said, in its own words.
core-verify-shortcut-unreadable = השאילתה על קיצור הדרך נכשלה: { $error }

## Warnings and errors a check gives, which the app and the installer show.

core-warning-verify-schema-unsupported = סכימת המניפסט של החבילה אינה נתמכת.
# { $error } is what went wrong, in Windows' or the program's own words.
core-warning-verify-package-file-failed = אימות קובץ החבילה נכשל: { $error }
core-warning-verify-folder-missing = התיקיה הריקה חסרה.
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-key-missing = מפתח הרישום חסר או לא קריא: { $error }
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-value-missing = ערך הרישום חסר או לא קריא: { $error }
core-warning-verify-registry-kind-missing = סוג ערך הרישום חסר.
core-warning-verify-firewall-rule-missing = כלל זה אינו קיים בחומת האש של Windows.
core-warning-verify-service-scope = אימות השירותים קורא את תצורת השירותים ב-HKLM ואינו מפעיל שירותים.
core-warning-verify-driver-manual = WinAppMigrator אינו מפיץ מחדש מנהל התקן אחד או יותר של ספק או מנהלי התקנים מובנים; יש להתקין אותם מחדש במחשב היעד באמצעות חבילת מנהל ההתקן החתומה של הספק.
core-warning-verify-task-scope = אימות המשימות המתוזמנות בודק רק את הרישום ואת ההפניה לפקודה.

## The verification report written beside a package, verification_report.md. Its headings follow a "#".

core-verify-report-title = דוח אימות של WinAppMigrator
# { $time } is when, as a date and time in UTC, such as 2026-11-15T10:00:00Z.
core-verify-report-verified = מועד האימות (UTC): { $time }
# { $valid } is true or false, which are not translated.
core-verify-report-package-valid = החבילה תקינה: { $valid }
# { $valid } is true or false, which are not translated.
core-verify-report-restore-valid = השחזור תקין: { $valid }
core-verify-report-restore-not-run = השחזור תקין: לא הופעל
core-verify-report-items = פריטים
# { $message } is what a check found, and { $source } the file or setting it looked at.
core-verify-report-item-from = { $message } מקור: { $source }
core-verify-report-warnings = אזהרות
core-verify-report-errors = שגיאות
core-verify-report-none = אין.
