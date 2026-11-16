# Words of WinAppMigrator Client, the console program run on the old PC, the one an app is moving off, so that
# WinAppMigrator on the new PC can read it over the network. Message names start with client-.
#
# Its window is a console, which on Windows 7 is 80 columns wide. Each line break in a message is one on the screen:
# keep every line shorter than 80 characters, and break lines where your language needs to, not only where English
# does.


## Starting

# Shown only in English, in place of a right-to-left language the console cannot show. { $language } is that
# language's name in English, such as Arabic.
client-console-rtl-fallback = یہ کنسول { $language } نہیں دکھا سکتا، اس لیے کلائنٹ انگریزی استعمال کرتا ہے۔
# The client copies itself to a folder in Program Files before it starts. { $error } is what Windows said went wrong.
client-safe-folder-failed =
    WinAppMigrator Client کسی ایسے فولڈر سے شروع نہیں ہو سکا جسے صرف منتظمین
    تبدیل کر سکتے ہیں: { $error }
client-press-enter-to-close = یہ ونڈو بند کرنے کے لیے Enter دبائیں۔
# The first line of its screen. { $version } is the client's version, such as 0.6.2.
client-heading = WinAppMigrator Client { $version }
client-intro =
    اس کی مدد سے آپ کے نئے PC پر WinAppMigrator آپ کے نیٹ ورک کے ذریعے اس PC
    کی ایپس پڑھ سکتا ہے۔ یہ یہاں کچھ تبدیل نہیں کرتا: یہ Windows ڈرائیو کا ایک
    اسنیپ شاٹ پڑھتا ہے، جسے آپ کے روکنے پر ہٹا دیتا ہے۔
# { $error } is why: one of the messages under "Why it stopped" below, or what Windows said.
client-stopped = کلائنٹ رک گیا: { $error }
# The caretaker is a copy of the client with no window that cleans up after it. { $error } is what Windows said.
client-caretaker-failed =
    اگر یہ ونڈو روکنے کے بجائے بند کر دی گئی تو اسنیپ شاٹ اور فائر وال قواعد
    اس وقت تک رہیں گے جب تک کلائنٹ اگلی بار یہاں نہ چلایا جائے ({ $error })۔
# Shown while the snapshot is taken, followed on the same line by client-snapshot-taken. Keep the quotes: they keep
# the space before it.
client-snapshot-taking = Windows ڈرائیو کا اسنیپ شاٹ لیا جا رہا ہے...{ " " }
client-snapshot-taken = ہو گیا۔
# The old PC answers the new one's search for it on the network. { $error } is what Windows said went wrong.
client-discovery-failed =
    یہ PC خود بخود نظر نہیں آئے گا ({ $error })۔
    اس کے بجائے نئے PC پر اس PC کا نام یا پتہ ٹائپ کریں۔

## This PC and its pairing code. The three lines are lined up in two columns, behind two spaces; keep the quotes, which
## keep those spaces.

# { $computer_name } is this PC's name, such as OLD-PC.
client-this-pc = { "  " }یہ PC:              { $computer_name }
# { $windows_name } is this PC's Windows, such as Windows 7 Professional Service Pack 1. { $bits } is 32 or 64.
client-windows = { "  " }Windows:            { $windows_name }، { $bits } بٹ
# { $code } is six digits in two groups, such as 482 913.
client-pairing-code = { "  " }جوڑا بنانے کا کوڈ:  { $code }
# Network PC... is the button of that name in WinAppMigrator: use the name it has there in your language.
client-pairing-steps =
    اپنے نئے PC پر WinAppMigrator میں نیٹ ورک PC... منتخب کریں اور یہ
    کوڈ ٹائپ کریں۔
    جو ایپ آپ منتقل کر رہے ہیں اسے پہلے بند کریں، تاکہ اس کی فائلیں ویسی ہی
    رہیں جیسی اس نے چھوڑی تھیں۔
    روکنے کے لیے یہاں Enter دبائیں یا یہ ونڈو بند کریں۔

## While it runs. { $from } is the network address of the PC that connected, such as 192.168.1.20, or "unknown" when
## it could not be told, which the [unknown] variant words.

client-paired =
    { $from ->
        [unknown] کسی دوسرے PC
       *[address] { $from }
    } کے ساتھ جوڑا بن گیا۔ یہ اپنی ضرورت کی چیزیں پڑھ رہا ہے...
# { $count } is how many requests it answered.
client-builder-done =
    { $from ->
        [unknown] کسی دوسرے PC
       *[address] { $from }
    } کا کام مکمل ہو گیا ({ $count ->
        [one] { $count } درخواست
       *[other] { $count } درخواستوں
    } کا جواب دیا گیا)۔
# { $error } is what went wrong, often in Windows' words.
client-link-ended =
    { $from ->
        [unknown] کسی دوسرے PC
       *[address] { $from }
    } سے رابطہ ختم ہو گیا: { $error }
client-wrong-code =
    { $from ->
        [unknown] کسی دوسرے PC
       *[address] { $from }
    } نے ایسا کوڈ آزمایا جو یہاں دکھائے گئے کوڈ سے مختلف ہے۔
# { $error } is what went wrong, often in Windows' words.
client-pair-failed =
    { $from ->
        [unknown] کسی دوسرے PC
       *[address] { $from }
    } کے ساتھ جوڑا نہیں بن سکا: { $error }
client-too-many-wrong-codes =
    بہت زیادہ غلط کوڈ آزمائے گئے، اس لیے اب کوئی PC اس کے ساتھ جوڑا نہیں بنا
    سکتا۔ ہو سکتا ہے آپ کے نیٹ ورک پر کوئی چیز کوڈ کا اندازہ لگا رہی ہو۔ جوڑا
    بنانے کے لیے کلائنٹ کو روکیں اور دوبارہ شروع کریں، اور جو نیا کوڈ دکھائے
    وہ ٹائپ کریں۔
# { $code } is six digits in two groups, such as 482 913.
client-new-code = بہت زیادہ غلط کوڈ۔ نیا جوڑا بنانے کا کوڈ یہ ہے: { $code }

## Stopping

client-stopping = روکا جا رہا ہے: اسنیپ شاٹ اور فائر وال قواعد ہٹائے جا رہے ہیں...
client-stop-done = ہو گیا۔

## Why it stopped, shown after "The client stopped: " (client-stopped), and written to follow it.

client-already-running = کلائنٹ اس PC پر پہلے سے چل رہا ہے، یا بند کیے گئے کلائنٹ کے بعد صفائی کر رہا ہے۔ وہی استعمال کریں، یا کچھ دیر انتظار کریں اور اسے دوبارہ شروع کریں۔
# { $drive } is the drive Windows is on, such as C:. { $details } is what Windows said, in its own words.
client-snapshot-failed = Windows نے { $drive } کا اسنیپ شاٹ نہیں بنایا: { $details }
# { $details } is what Windows Firewall said, in its own words.
client-firewall-failed = Windows فائر وال نے کلائنٹ کو سننے کی اجازت نہیں دی: { $details }

## What it takes, shown for --help. The options, such as --lang, are typed as they are and are not translated; keep
## their descriptions lined up.

client-usage =
    WinAppMigrator Client یہ اختیارات لیتا ہے:
    
      --lang <language>  Windows کی ڈسپلے زبان کے بجائے یہ زبان، جیسے it-IT،
                         استعمال کریں، اگر کلائنٹ میں موجود ہو۔
      --code <code>      چھ ہندسوں کے اس جوڑا بنانے کے کوڈ سے شروع کریں، ایسی
                         اسکرپٹ کے لیے جو نئے PC پر WinAppMigrator بھی شروع کرے۔
      --help             یہ دکھائیں۔
