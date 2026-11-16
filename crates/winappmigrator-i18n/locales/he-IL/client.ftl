# Words of WinAppMigrator Client, the console program run on the old PC, the one an app is moving off, so that
# WinAppMigrator on the new PC can read it over the network. Message names start with client-.
#
# Its window is a console, which on Windows 7 is 80 columns wide. Each line break in a message is one on the screen:
# keep every line shorter than 80 characters, and break lines where your language needs to, not only where English
# does.


## Starting

# Shown only in English, in place of a right-to-left language the console cannot show. { $language } is that
# language's name in English, such as Arabic.
client-console-rtl-fallback = מסוף זה אינו יכול להציג { $language }, ולכן הלקוח משתמש באנגלית.
# The client copies itself to a folder in Program Files before it starts. { $error } is what Windows said went wrong.
client-safe-folder-failed =
    לא ניתן היה להפעיל את WinAppMigrator Client מתיקיה שרק מנהלי מערכת
    יכולים לשנות: { $error }
client-press-enter-to-close = יש להקיש Enter כדי לסגור חלון זה.
# The first line of its screen. { $version } is the client's version, such as 0.6.2.
client-heading = WinAppMigrator Client { $version }
client-intro =
    כלי זה מאפשר ל-WinAppMigrator במחשב החדש לקרוא דרך הרשת את האפליקציות
    שבמחשב זה. הוא אינו משנה דבר כאן: הוא קורא תמונת מצב של כונן Windows,
    ומסיר אותה כשעוצרים אותו.
# { $error } is why: one of the messages under "Why it stopped" below, or what Windows said.
client-stopped = הלקוח נעצר: { $error }
# The caretaker is a copy of the client with no window that cleans up after it. { $error } is what Windows said.
client-caretaker-failed =
    אם חלון זה ייסגר במקום לעצור את הלקוח, תמונת המצב וכללי חומת האש
    יישארו עד להפעלה הבאה של הלקוח כאן ({ $error }).
# Shown while the snapshot is taken, followed on the same line by client-snapshot-taken. Keep the quotes: they keep
# the space before it.
client-snapshot-taking = יצירת תמונת מצב של כונן Windows...{ " " }
client-snapshot-taken = בוצע.
# The old PC answers the new one's search for it on the network. { $error } is what Windows said went wrong.
client-discovery-failed =
    מחשב זה לא יופיע באופן אוטומטי ({ $error }).
    במחשב החדש, יש להקליד במקום זאת את השם או הכתובת של מחשב זה.

## This PC and its pairing code. The three lines are lined up in two columns, behind two spaces; keep the quotes, which
## keep those spaces.

# { $computer_name } is this PC's name, such as OLD-PC.
client-this-pc = { "  " }מחשב זה:      { $computer_name }
# { $windows_name } is this PC's Windows, such as Windows 7 Professional Service Pack 1. { $bits } is 32 or 64.
client-windows = { "  " }Windows:      { $windows_name }, { $bits } סיביות
# { $code } is six digits in two groups, such as 482 913.
client-pairing-code = { "  " }קוד התאמה:    { $code }
# Network PC... is the button of that name in WinAppMigrator: use the name it has there in your language.
client-pairing-steps =
    במחשב החדש, ב-WinAppMigrator, יש לבחור "מחשב ברשת..." ולהקליד קוד זה.
    יש לסגור תחילה את האפליקציה שמעבירים, כדי שקבציה יישארו כפי שהיו.
    כדי לעצור, יש להקיש Enter כאן או לסגור חלון זה.

## While it runs. { $from } is the network address of the PC that connected, such as 192.168.1.20, or "unknown" when
## it could not be told, which the [unknown] variant words.

client-paired =
    בוצעה התאמה עם { $from ->
        [unknown] מחשב אחר
       *[address] { $from }
    }. הוא קורא את מה שנדרש לו...
# { $count } is how many requests it answered.
client-builder-done =
    { $from ->
        [unknown] מחשב אחר
       *[address] { $from }
    } סיים ({ $count ->
        [one] נענתה בקשה אחת
        [two] נענו שתי בקשות
       *[other] נענו { $count } בקשות
    }).
# { $error } is what went wrong, often in Windows' words.
client-link-ended =
    החיבור אל { $from ->
        [unknown] מחשב אחר
       *[address] { $from }
    } הסתיים: { $error }
client-wrong-code =
    { $from ->
        [unknown] מחשב אחר
       *[address] { $from }
    } ניסה קוד שאינו הקוד של מחשב זה.
# { $error } is what went wrong, often in Windows' words.
client-pair-failed =
    לא ניתן היה לבצע התאמה עם { $from ->
        [unknown] מחשב אחר
       *[address] { $from }
    }: { $error }
client-too-many-wrong-codes =
    נוסו יותר מדי קודים שגויים, ולכן כעת אף מחשב לא יכול לבצע התאמה
    עם מחשב זה. ייתכן שמשהו ברשת מנסה לנחש. כדי לבצע התאמה, יש לעצור
    את הלקוח ולהפעיל אותו שוב, ולהקליד את הקוד החדש שהוא מציג.
# { $code } is six digits in two groups, such as 482 913.
client-new-code = יותר מדי קודים שגויים. קוד ההתאמה החדש הוא: { $code }

## Stopping

client-stopping = עצירה: הסרת תמונת המצב וכללי חומת האש...
client-stop-done = בוצע.

## Why it stopped, shown after "The client stopped: " (client-stopped), and written to follow it.

client-already-running = הלקוח כבר פועל במחשב זה, או מנקה אחרי לקוח שנסגר. יש להשתמש בלקוח הפועל, או להמתין רגע ולהפעיל לקוח זה שוב.
# { $drive } is the drive Windows is on, such as C:. { $details } is what Windows said, in its own words.
client-snapshot-failed = Windows לא יצר תמונת מצב של { $drive }: { $details }
# { $details } is what Windows Firewall said, in its own words.
client-firewall-failed = חומת האש של Windows לא אפשרה ללקוח להאזין: { $details }

## What it takes, shown for --help. The options, such as --lang, are typed as they are and are not translated; keep
## their descriptions lined up.

client-usage =
    WinAppMigrator Client מקבל את האפשרויות הבאות:
    
      --lang <שפה>       שימוש בשפה זו, כגון it-IT, אם היא קיימת בלקוח,
                         במקום שפת התצוגה של Windows.
      --code <קוד>       התחלה עם קוד התאמה זה בן שש הספרות, עבור סקריפט
                         שמפעיל גם את WinAppMigrator במחשב החדש.
      --help             הצגת מידע זה.
