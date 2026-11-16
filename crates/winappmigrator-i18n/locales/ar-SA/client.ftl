# Words of WinAppMigrator Client, the console program run on the old PC, the one an app is moving off, so that
# WinAppMigrator on the new PC can read it over the network. Message names start with client-.
#
# Its window is a console, which on Windows 7 is 80 columns wide. Each line break in a message is one on the screen:
# keep every line shorter than 80 characters, and break lines where your language needs to, not only where English
# does.


## Starting

# Shown only in English, in place of a right-to-left language the console cannot show. { $language } is that
# language's name in English, such as Arabic.
client-console-rtl-fallback = لا يمكن لوحدة التحكم هذه عرض { $language }، لذا يستخدم العميل اللغة الإنجليزية.
# The client copies itself to a folder in Program Files before it starts. { $error } is what Windows said went wrong.
client-safe-folder-failed =
    تعذر بدء WinAppMigrator Client من مجلد لا يمكن لغير المسؤولين
    تغييره: { $error }
client-press-enter-to-close = اضغط على Enter لإغلاق هذه النافذة.
# The first line of its screen. { $version } is the client's version, such as 0.6.2.
client-heading = WinAppMigrator Client { $version }
client-intro =
    يتيح هذا لـ WinAppMigrator على الكمبيوتر الجديد قراءة التطبيقات الموجودة
    على هذا الكمبيوتر عبر الشبكة. لا يغيّر شيئًا هنا: فهو يقرأ لقطة لمحرك
    أقراص Windows، ويزيلها عند إيقافه.
# { $error } is why: one of the messages under "Why it stopped" below, or what Windows said.
client-stopped = توقف العميل: { $error }
# The caretaker is a copy of the client with no window that cleans up after it. { $error } is what Windows said.
client-caretaker-failed =
    إذا أُغلقت هذه النافذة بدلاً من إيقاف العميل، فستبقى اللقطة وقواعد جدار
    الحماية حتى التشغيل التالي للعميل هنا ({ $error }).
# Shown while the snapshot is taken, followed on the same line by client-snapshot-taken. Keep the quotes: they keep
# the space before it.
client-snapshot-taking = جارٍ أخذ لقطة لمحرك أقراص Windows...{ " " }
client-snapshot-taken = تم.
# The old PC answers the new one's search for it on the network. { $error } is what Windows said went wrong.
client-discovery-failed =
    لن يظهر هذا الكمبيوتر على الشبكة تلقائيًا ({ $error }).
    على الكمبيوتر الجديد، اكتب اسم هذا الكمبيوتر أو عنوانه بدلاً من ذلك.

## This PC and its pairing code. The three lines are lined up in two columns, behind two spaces; keep the quotes, which
## keep those spaces.

# { $computer_name } is this PC's name, such as OLD-PC.
client-this-pc = { "  " }هذا الكمبيوتر:  { $computer_name }
# { $windows_name } is this PC's Windows, such as Windows 7 Professional Service Pack 1. { $bits } is 32 or 64.
client-windows = { "  " }Windows:        { $windows_name }، { $bits } بت
# { $code } is six digits in two groups, such as 482 913.
client-pairing-code = { "  " }رمز الاقتران:   { $code }
# Network PC... is the button of that name in WinAppMigrator: use the name it has there in your language.
client-pairing-steps =
    على الكمبيوتر الجديد، في WinAppMigrator، اختر "كمبيوتر على الشبكة..."
    واكتب هذا الرمز. أغلق التطبيق الذي تنقله أولاً، حتى تبقى ملفاته كما تركها.
    للإيقاف، اضغط على Enter هنا أو أغلق هذه النافذة.

## While it runs. { $from } is the network address of the PC that connected, such as 192.168.1.20, or "unknown" when
## it could not be told, which the [unknown] variant words.

client-paired =
    تم الاقتران مع { $from ->
        [unknown] كمبيوتر آخر
       *[address] { $from }
    }. وهو يقرأ الآن ما يحتاج إليه...
# { $count } is how many requests it answered.
client-builder-done =
    انتهى { $from ->
        [unknown] كمبيوتر آخر
       *[address] { $from }
    } من عمله (تمت الإجابة عن { $count ->
        [zero] { $count } طلب
        [one] طلب واحد
        [two] طلبين
        [few] { $count } طلبات
        [many] { $count } طلبًا
       *[other] { $count } طلب
    }).
# { $error } is what went wrong, often in Windows' words.
client-link-ended =
    انتهى الاتصال مع { $from ->
        [unknown] كمبيوتر آخر
       *[address] { $from }
    }: { $error }
client-wrong-code =
    جرّب { $from ->
        [unknown] كمبيوتر آخر
       *[address] { $from }
    } رمزًا لا يطابق رمز هذا الكمبيوتر.
# { $error } is what went wrong, often in Windows' words.
client-pair-failed =
    تعذر على { $from ->
        [unknown] كمبيوتر آخر
       *[address] { $from }
    } الاقتران: { $error }
client-too-many-wrong-codes =
    تمت تجربة عدد كبير جدًا من الرموز الخاطئة، لذا لا يمكن لأي كمبيوتر الاقتران
    بهذا الكمبيوتر الآن. قد يكون هناك شيء على الشبكة يحاول التخمين. للاقتران،
    أوقف العميل ثم ابدأ تشغيله مرة أخرى، واكتب الرمز الجديد الذي يعرضه.
# { $code } is six digits in two groups, such as 482 913.
client-new-code = عدد كبير جدًا من الرموز الخاطئة. رمز الاقتران الجديد هو: { $code }

## Stopping

client-stopping = جارٍ الإيقاف: إزالة اللقطة وقواعد جدار الحماية...
client-stop-done = تم.

## Why it stopped, shown after "The client stopped: " (client-stopped), and written to follow it.

client-already-running = العميل قيد التشغيل بالفعل على هذا الكمبيوتر، أو يجري التنظيف بعد نسخة أُغلقت. استخدم تلك النسخة، أو انتظر لحظة ثم ابدأ تشغيل هذه النسخة مرة أخرى.
# { $drive } is the drive Windows is on, such as C:. { $details } is what Windows said, in its own words.
client-snapshot-failed = لم يأخذ Windows لقطة لـ { $drive }: { $details }
# { $details } is what Windows Firewall said, in its own words.
client-firewall-failed = لم يسمح جدار حماية Windows للعميل بالاستماع: { $details }

## What it takes, shown for --help. The options, such as --lang, are typed as they are and are not translated; keep
## their descriptions lined up.

client-usage =
    يقبل WinAppMigrator Client هذه الخيارات:
    
      --lang <language>  استخدام هذه اللغة، مثل it-IT، إن كانت متوفرة في العميل،
                         بدلاً من لغة العرض في Windows.
      --code <code>      البدء برمز الاقتران هذا المكون من ستة أرقام، لبرنامج نصي
                         يبدأ تشغيل WinAppMigrator على الكمبيوتر الجديد أيضًا.
      --help             عرض هذه المعلومات.
