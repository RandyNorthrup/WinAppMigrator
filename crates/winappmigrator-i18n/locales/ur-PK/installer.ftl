# Words of the installer that every built installer carries, which runs on the PC an app is moved to and speaks its
# language, and of the uninstaller it leaves behind for Settings. Message names start with installer- for the
# installer's window, installer-cli- for its command line, and uninstaller- for the uninstaller.
#
# The command lines (installer-cli-, uninstaller-cli-) are always shown in English, as scripts read what they print.
# Translate them all the same. Keep their command-line flags, such as --apply and --target-root, exactly as they are.


## The installer's window: its title and header

installer-window-title = WinAppMigrator انسٹالر
# The window's title in the taskbar. { $app_name } is the name of the app it installs.
installer-window-title-app = { $app_name } انسٹال کریں
installer-tagline = کسی دوسرے PC سے حاصل کی گئی ایپلیکیشن کو دوبارہ انسٹال کرتا ہے
# What an app is called when its package gives it no name.
installer-unnamed-app = ایپلیکیشن

## When the installer's window cannot open. Each is a paragraph of a message box.

# { $error } is what went wrong, as Windows or the graphics driver says it.
installer-window-failed = انسٹالر کی ونڈو نہیں کھل سکی: { $error }
installer-window-failed-without-one = یہ ونڈو کے بغیر بھی انسٹال کر سکتا ہے: اس فائل کو منتظم کے کمانڈ پرامپٹ سے --apply کے ساتھ چلائیں، اور مقام منتخب کرنے کے لیے --target-root کے بعد کوئی فولڈر لکھیں۔

## Checking the installer when it opens

installer-checking = انسٹالر کی جانچ کی جا رہی ہے…
installer-stub-no-package = یہ WinAppMigrator انسٹالر اسٹب ہے، اور اس میں کوئی ایپلیکیشن پیکیج نہیں ہے۔ WinAppMigrator سے انسٹالر بنائیں، پھر بنائی گئی .exe فائل یہاں چلائیں۔
installer-check-stopped = انسٹالر کی جانچ رک گئی۔
# { $error } is what went wrong, as Windows says it.
installer-locate-failed = انسٹالر کا مقام معلوم نہیں ہو سکا: { $error }
# { $error } is what went wrong reading it.
installer-read-failed = انسٹالر پڑھا نہیں جا سکا: { $error }
# { $error } is why, such as that the app needs a newer Windows.
installer-cannot-be-used-here = اس انسٹالر کا پیکیج اس PC پر استعمال نہیں کیا جا سکتا: { $error }

## The app, where it goes, and whether the installer is signed

installer-captured-from = کہاں سے حاصل کیا گیا
installer-install-location = تنصیب کا مقام
# The title of the folder picker.
installer-choose-location = تنصیب کا مقام منتخب کریں
installer-folder-has-files = اس فولڈر میں پہلے سے فائلیں موجود ہیں۔ ایک جیسے نام والی فائلیں تبدیل کر دی جاتی ہیں، اور اگر آپ ان انسٹال کریں تو واپس آ جاتی ہیں۔
installer-room-needed = درکار جگہ
# { $shortfall } says which drive is short of room, such as "C: needs up to 824.0 MB for the app's files, and has 300.0 MB free".
installer-room-short = یہاں شاید کافی جگہ نہ ہو: { $shortfall }۔ پیکیج کسی دوسری ڈرائیو پر نکالا جا سکتا ہے، لیکن ایپ کی فائلوں کے لیے پھر بھی یہاں جگہ ہونی چاہیے۔
# { $app_name } is the name of an app whose earlier install was cut short.
installer-interrupted = { $app_name } کی ایک پچھلی تنصیب کبھی مکمل نہیں ہوئی۔ انسٹال کرنے پر پہلے اسے کالعدم کیا جاتا ہے۔
# Beside "Installer", for whether it proves who made it.
installer-signature = انسٹالر
installer-signature-checking = اس کے دستخط کی جانچ کی جا رہی ہے…
installer-signature-unsigned = دستخط شدہ نہیں: ناشر نامعلوم
# { $publisher } is the name on the installer's certificate, such as "Fabrikam, Inc.".
installer-signature-signed-by = دستخط کنندہ: { $publisher }
installer-signature-unnamed = کسی بے نام ناشر کی جانب سے دستخط شدہ
# { $reason } is Windows' own word for what is wrong, such as HashMismatch.
installer-signature-invalid = اس انسٹالر کے دستخط درست نہیں ہیں ({ $reason })۔ ہو سکتا ہے دستخط کے بعد اسے تبدیل کیا گیا ہو۔

## Why the install location cannot be used. Install and Preview Changes stay off while one of these is shown.

# { $app_name } is the name of the app being installed.
installer-location-app-already-there = { $app_name } اس فولڈر میں پہلے سے انسٹال ہے، اس لیے یہ کاپی یہاں نہیں جا سکتی۔ اسے یہاں انسٹال کرنے کے لیے پہلے اس PC پر موجود کاپی کو Windows ترتیبات میں ایپس کے تحت ان انسٹال کریں، اور پھر یہ انسٹالر دوبارہ کھولیں۔ یا اوپر ایک نیا، خالی فولڈر درج کریں۔
# { $other_app } is the name of another app Windows lists as installed in the folder. { $app_name } is the name of the app being installed.
installer-location-other-app-there = { $other_app } اس فولڈر میں انسٹال ہے، اس لیے { $app_name } یہاں نہیں جا سکتی۔ اوپر ایک نیا، خالی فولڈر درج کریں۔ یا، اگر آپ کو اب { $other_app } کی ضرورت نہیں ہے، تو پہلے اسے Windows ترتیبات میں ایپس کے تحت ان انسٹال کریں، اور پھر یہ انسٹالر دوبارہ کھولیں۔
installer-location-has-files = اس فولڈر میں پہلے سے ایسی فائلیں ہیں جو WinAppMigrator نے انسٹال نہیں کیں۔ ہو سکتا ہے وہ کسی دوسری ایپ کی ہوں، اس لیے ان کے اوپر کچھ انسٹال نہیں کیا جاتا۔ اوپر ایک نیا، خالی فولڈر درج کریں۔
# { $reason } is why, such as that it cannot be the Windows folder.
installer-location-refused = یہ فولڈر استعمال نہیں کیا جا سکتا: { $reason }۔ اوپر کوئی دوسرا فولڈر درج کریں۔

## What the install sets up beyond its files

installer-beyond-files = یہ فائلوں کے علاوہ کیا سیٹ اپ کرتا ہے
# Under the heading above. It is part of what a person agrees to when they click Install.
installer-beyond-files-agreed = یہ سب منتظم کے حقوق کے ساتھ کیا جاتا ہے۔ اگر تنصیب ناکام ہو جائے تو اس نے جو کچھ تبدیل کیا وہ واپس رکھ دیا جاتا ہے، اور جو واپس نہ رکھا جا سکے اس کے بارے میں آپ کو بتایا جاتا ہے۔ آپ بعد میں ترتیبات میں انسٹال کردہ ایپس کے تحت بھی تنصیب کو کالعدم کر سکتے ہیں۔ جن فائلوں کو یہ تبدیل کرتا ہے وہ واپس آ جاتی ہیں، اور جو رن ٹائمز یہ انسٹال کرتا ہے وہ باقی رہتے ہیں۔
installer-beyond-files-none = کچھ نہیں: یہ صرف فائلیں، اور ایپ کی اپنی رجسٹری کلیدوں میں ترتیبات کاپی کرتا ہے۔
# { $problem } is why, such as that the app cannot run on this PC.
installer-cannot-go-here = یہ پیکیج اس مقام پر نہیں جا سکتا: { $problem }
installer-working-out = معلوم کیا جا رہا ہے کہ یہ اس مقام پر کیا سیٹ اپ کرے گا…
# Said as the reason after "This package cannot go to this location:".
installer-list-stopped = یہ معلوم کرنے کا عمل رک گیا کہ یہ کیا کرے گا
# A heading over a list of what else the package asked for and will not get. { $count } is how many.
installer-more-left-out =
    { $count ->
        [one] مزید { $count } چیز خارج کر دی گئی
       *[other] مزید { $count } چیزیں خارج کر دی گئیں
    }

## Options

installer-options = اختیارات
installer-user-data = صارف کا ڈیٹا
installer-user-data-detail = آن: ترتیبات، دستاویزات اور محفوظ کردہ گیمز اس شخص کے معمول کے فولڈرز میں جاتے ہیں جو سائن ان ہو۔ آف: یہ سب تنصیب کے فولڈر کے تحت جاتا ہے، اور اس سے چلنے کے لیے کچھ سیٹ اپ نہیں کیا جاتا۔
# { $account } is the name of the account on the other PC whose data this is.
installer-user-data-whose = اس میں { $account } کا ڈیٹا ہے۔
installer-license = لائسنس
installer-license-detail = فعال کاری اور لائسنس کی فائلیں، اور وہ رجسٹری اقدار جن میں لائسنس محفوظ ہوتے ہیں۔ کچھ کو یہاں دوبارہ فعال کاری کی ضرورت ہو سکتی ہے۔

## Dependencies: what else the app needs, and how each is provided

installer-dependencies = انحصارات
installer-dependencies-detail = اس ایپ کو کیا درکار ہے، اور ہر ایک کیسے فراہم کیا جاتا ہے۔
installer-dependencies-none = کوئی اضافی رن ٹائم انحصارات نہیں ملے۔
# { $feature } is the name of an optional part of Windows, such as NetFx3.
installer-dependency-windows-feature = Windows خصوصیت: { $feature }
installer-dependency-included = شامل ہے
# { $id } is the package's name in winget, such as Microsoft.VCRedist.2015+.x64.
installer-dependency-winget = winget سے انسٹال کیا جاتا ہے: { $id }
installer-dependency-manual = دستی طور پر انسٹال کریں
installer-dependency-built-in = Windows میں شامل ہے
installer-dependency-ships-with-app = ایپ کے ساتھ آتا ہے

## Buttons, and what is said beside them

installer-install = انسٹال کریں
installer-preview = تبدیلیوں کا پیش منظر
installer-cancel = منسوخ کریں
installer-close = بند کریں
installer-open-folder = تنصیب کا فولڈر کھولیں
# { $error } is what went wrong, as Windows says it.
installer-open-folder-failed = فولڈر نہیں کھل سکا: { $error }
installer-location-blocks = جب تک تنصیب کا مقام استعمال کے قابل نہ ہو، "انسٹال کریں" اور "تبدیلیوں کا پیش منظر" بند رہیں گے۔ وجہ اوپر، اس کے نیچے درج ہے۔
installer-package-blocks = "انسٹال کریں" بند ہے کیونکہ یہ پیکیج اس مقام پر نہیں جا سکتا۔ وجہ اوپر درج ہے۔
installer-list-changed = یہاں یہ جو کچھ کرے گا وہ بدل گیا ہے، اس لیے کچھ انسٹال نہیں کیا گیا۔ نیچے دی گئی فہرست دیکھیں، اور اگر یہی آپ چاہتے ہیں تو دوبارہ "انسٹال کریں" پر کلک کریں۔
installer-cancelled-early = منسوخ کر دیا گیا۔ کچھ تبدیل نہیں کیا گیا۔

## How far an install has got, beside its progress bar

# { $step } is one of the steps below. { $done } and { $total } are how many of its items are done, and how many there are.
installer-step-counted = { $step }: { $total } میں سے { $done }…
# { $step } is one of the steps below, when it does not count its items.
installer-step-uncounted = { $step }…
installer-step-getting-ready = تیاری ہو رہی ہے…
installer-step-cancelling = موجودہ مرحلے کے بعد منسوخ کیا جا رہا ہے…
installer-step-putting-back = جو تبدیل کیا گیا تھا اسے واپس رکھا جا رہا ہے
installer-step-unpacking = پیکیج نکالا جا رہا ہے
installer-step-checking-package = پیکیج کی جانچ کی جا رہی ہے
installer-step-recovering = ایک پچھلی تنصیب کالعدم کی جا رہی ہے جو کبھی مکمل نہیں ہوئی
# What a preview is doing, whatever step it is on.
installer-step-previewing = معلوم کیا جا رہا ہے کہ کیا تبدیل ہوگا
installer-step-runtimes = رن ٹائمز انسٹال کیے جا رہے ہیں
installer-step-files = فائلیں کاپی کی جا رہی ہیں
installer-step-services = سروسز سیٹ اپ کی جا رہی ہیں
installer-step-drivers = ڈرائیورز سیٹ اپ کیے جا رہے ہیں
installer-step-registry = رجسٹری ترتیبات لکھی جا رہی ہیں
installer-step-license = لائسنس کا ڈیٹا بحال کیا جا رہا ہے
installer-step-tasks = شیڈول کردہ ٹاسکس سیٹ اپ کیے جا رہے ہیں
installer-step-shortcuts = شارٹ کٹس بنائے جا رہے ہیں
installer-step-environment = ماحولیاتی متغیرات سیٹ کیے جا رہے ہیں
installer-step-settings-entry = ایپ کو ترتیبات میں شامل کیا جا رہا ہے
installer-step-verifying = جو انسٹال کیا گیا اس کی جانچ کی جا رہی ہے
installer-step-undoing = کالعدم کیا جا رہا ہے

## When an install cannot go ahead or stops

installer-carries-no-package = اس انسٹالر میں کوئی پیکیج نہیں ہے۔
installer-stopped-unexpectedly = تنصیب غیر متوقع طور پر رک گئی۔
# { $reason } is what went wrong, as the program says it.
installer-stopped-unexpectedly-because = تنصیب غیر متوقع طور پر رک گئی: { $reason }۔ اگر یہ شروع ہو چکی تھی، تو انسٹالر دوبارہ چلانے پر پہلے وہ کالعدم ہو جاتا ہے جو اس نے کیا تھا۔
# { $error } is what went wrong reading it.
installer-package-read-failed = پیکیج پڑھا نہیں جا سکا: { $error }
installer-no-programdata = اس PC کا ProgramData فولڈر نہیں مل سکا، اس لیے کچھ انسٹال نہیں کیا گیا: تنصیب وہ ریکارڈ وہیں رکھتی ہے جس کے ذریعے اسے کالعدم کیا جاتا ہے۔
# { $error } is why, such as that there is nowhere only administrators can get into.
installer-staging-failed = اسٹیجنگ فولڈر نہیں بنایا جا سکا: { $error }
# { $error } is what went wrong unpacking it.
installer-unpack-failed = پیکیج نکالا نہیں جا سکا: { $error }
installer-staging-nowhere = پیکیج نکالنے کے لیے کوئی ایسی جگہ نہیں ہے جہاں صرف منتظمین رسائی حاصل کر سکیں
# { $failure } is a folder and what went wrong there, such as "C:\ProgramData\WinAppMigrator: Access is denied.".
installer-staging-nowhere-look-at = پیکیج نکالنے کے لیے کوئی ایسی جگہ نہیں ہے جہاں صرف منتظمین رسائی حاصل کر سکیں۔ جسے دیکھنا چاہیے وہ یہ ہے: { $failure }

## What came of an install, or of a preview

installer-result = تنصیب کا نتیجہ
installer-result-preview = پیش منظر: ابھی تک کچھ تبدیل نہیں کیا گیا
installer-result-installed = ایپلیکیشن انسٹال ہو گئی اور اپنی ہیش کی جانچ میں کامیاب رہی۔
installer-result-installed-without-left-out = ایپلیکیشن انسٹال ہو گئی اور اپنی ہیش کی جانچ میں کامیاب رہی، ان چیزوں کے بغیر جو نیچے خارج کردہ کے طور پر درج ہیں۔
installer-result-not-all-put-back = تنصیب ناکام ہو گئی، اور اس نے جو کچھ تبدیل کیا تھا وہ سب واپس نہیں رکھا جا سکا۔ جو باقی ہے وہ نیچے درج ہے، اور اگلی تنصیب دوبارہ کوشش کرے گی۔
installer-result-cancelled-put-back = تنصیب منسوخ کر دی گئی، اور اس نے جو کچھ تبدیل کیا تھا وہ واپس رکھ دیا گیا۔
installer-result-failed-put-back = تنصیب ناکام ہو گئی، اور اس نے جو کچھ تبدیل کیا تھا وہ واپس رکھ دیا گیا۔
installer-result-not-clean = تنصیب صاف طور پر مکمل نہیں ہوئی۔ نیچے دیے گئے مسائل کا جائزہ لیں۔
installer-result-changes-put-back = واپس رکھی گئی تبدیلیاں
installer-result-changes-made = کی گئی تبدیلیاں
installer-result-changes-planned = منصوبہ بند تبدیلیاں
installer-result-warnings = انتباہات
installer-result-errors = خرابیاں
installer-result-package-valid = پیکیج کے ہیش درست ہیں
installer-result-package-invalid = پیکیج کے ہیش درست نہیں ہیں
installer-result-files-valid = انسٹال کردہ فائلیں درست ہیں
installer-result-files-invalid = انسٹال کردہ فائلیں درست نہیں ہیں
installer-result-reboot = ایک انحصار کو مکمل ہونے کے لیے دوبارہ شروع کرنا درکار ہے۔ Windows کو دوبارہ شروع کریں، پھر ایپ چلائیں۔
# A heading over the list of what was changed. { $count } is how many.
installer-result-changes = تبدیلیاں ({ $count })
# After the first part of a long list. { $count } is how many more there are.
installer-result-changes-more = اور مزید { $count }۔
# Over the path of the install's log file.
installer-log = لاگ

## The installer's command line. Shown in English: scripts read it. A { $count } is plain digits, with no separators.

installer-cli-stub = WinAppMigrator انسٹالر اسٹب — اس میں کوئی ایپلیکیشن پیکیج نہیں ہے۔
installer-cli-stub-how = WinAppMigrator ایپ سے انسٹالر بنائیں، پھر بنائی گئی .exe فائل ہدف مشین پر چلائیں۔
installer-cli-usage-title = WinAppMigrator خود نکالنے والا انسٹالر۔
installer-cli-usage = گرافیکل انسٹالر کھولنے کے لیے اسے کسی آرگیومنٹ کے بغیر چلائیں، یا:
installer-cli-usage-preview = دوبارہ تنصیب کا پیش منظر: --dry-run [--target-root <DIR>]
installer-cli-usage-apply = دوبارہ انسٹال کریں:        --apply [--target-root <DIR>]
installer-cli-no-package = انسٹالر میں کوئی شامل شدہ پیکیج نہیں ہے
installer-cli-no-programdata = اس PC کا ProgramData فولڈر نہیں مل سکا، اس لیے کچھ انسٹال نہیں کیا گیا: تنصیب وہ ریکارڈ وہیں رکھتی ہے جس کے ذریعے اسے کالعدم کیا جاتا ہے
# { $path } is the folder asked for, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-folder-has-files = { $path } استعمال نہیں کیا جا سکتا: { $problem }۔ --force صرف اسی صورت میں دیں جب آپ کو معلوم ہو کہ یہ اسی ایپ کا ہے
installer-cli-no-location = یہ پیکیج نہیں بتاتا کہ اس کی ایپ کہاں تھی، اس لیے --target-root دیں
# { $path } is where the app was on the other PC, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-location-refused = اس پیکیج کی ایپ { $path } میں تھی، اور اسے استعمال نہیں کیا جا سکتا: { $problem }۔ --target-root دیں
# { $summary } is how much room it needs where, such as "up to 824.0 MB on C: (356.0 GB free)".
installer-cli-room = جگہ: { $summary }
# { $note } is something to know about the install.
installer-cli-note = نوٹ: { $note }
# { $count } is how many things it sets up beyond its files.
installer-cli-sets-up = سیٹ اپ کرتا ہے: { $count }
installer-cli-applied = دوبارہ تنصیب لاگو کر دی گئی۔
installer-cli-previewed = دوبارہ تنصیب کا پیش منظر تیار ہو گیا۔
# { $count } is how many changes the install logged.
installer-cli-changes-logged = لاگ کی گئی تبدیلیاں: { $count }
# { $count } is how many warnings.
installer-cli-warnings = انتباہات: { $count }
# { $count } is how many errors.
installer-cli-errors = خرابیاں: { $count }
# { $count } is how many things the package asked for and did not get.
installer-cli-refused = مسترد: { $count }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-package-valid = پیکیج کے ہیش درست: { $valid }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-reinstall-valid = دوبارہ تنصیب کے ہیش درست: { $valid }
# { $id } is the install's id, such as wam-0f3a….
installer-cli-install-id = تنصیب کی ID: { $id }
installer-cli-put-back = تنصیب ناکام ہو گئی، اس لیے اس نے جو کچھ تبدیل کیا تھا وہ واپس رکھ دیا گیا۔
# { $path } is the log file.
installer-cli-log = لاگ: { $path }
# { $path } is the log file. Printed before an error.
installer-cli-error-log = لاگ: { $path }
# { $error } is what went wrong.
installer-cli-error = خرابی: { $error }

## The uninstaller's window

uninstaller-window-title = ان انسٹال کریں
# The window's title, and its heading. { $app_name } is the name of the app it uninstalls.
uninstaller-window-title-app = { $app_name } ان انسٹال کریں
uninstaller-tagline = وہ کالعدم کرتا ہے جو اس کے انسٹالر نے کیا تھا
uninstaller-reading = اس تنصیب کا ریکارڈ پڑھا جا رہا ہے…
uninstaller-stopped-unexpectedly = ان انسٹال کا عمل غیر متوقع طور پر رک گیا۔
uninstaller-what-will-be-removed = کیا ہٹایا جائے گا
uninstaller-install-location = تنصیب کا مقام
uninstaller-files = فائلیں
uninstaller-registry-values = رجسٹری اقدار
uninstaller-services = سروسز
uninstaller-drivers = ڈرائیورز
uninstaller-scheduled-tasks = شیڈول کردہ ٹاسکس
uninstaller-environment = ماحولیاتی متغیرات
uninstaller-plan-stays = آپ کی شامل یا تبدیل کردہ فائلیں، ان کے فولڈرز سمیت، باقی رہتی ہیں۔
# { $count } is how many files the install replaced.
uninstaller-plan-coming-back =
    { $count ->
        [one] اس تنصیب نے جو { $count } فائل تبدیل کی تھی وہ واپس آ جاتی ہے، یا اگر موجودہ فائل تب سے تبدیل ہو چکی ہو تو اس کے ساتھ رکھ دی جاتی ہے۔
       *[other] اس تنصیب نے جو { $count } فائلیں تبدیل کی تھیں وہ واپس آ جاتی ہیں، یا اگر موجودہ فائلیں تب سے تبدیل ہو چکی ہوں تو ان کے ساتھ رکھ دی جاتی ہیں۔
    }
# { $runtimes } is a list of runtimes, such as "Microsoft Visual C++ Redistributable, .NET Desktop Runtime 8".
uninstaller-plan-runtimes = اس کے انسٹال کردہ رن ٹائمز باقی رہتے ہیں، کیونکہ دوسرے پروگرام انہیں استعمال کر سکتے ہیں: { $runtimes }۔
uninstaller-options = اختیارات
uninstaller-remove-user-data = صارف کا ڈیٹا اور لائسنس کا ڈیٹا بھی ہٹائیں
# { $count } is how many items of the app's settings, data, and license there are.
uninstaller-remove-user-data-detail =
    { $count ->
        [one] اس کی ترتیبات، ڈیٹا اور لائسنس کا { $count } آئٹم۔ جب تک یہ اختیار آن نہ ہو، یہ باقی رہتا ہے۔ اس نے جو تبدیل کیا تھا وہ واپس آ جاتا ہے، یا اگر یہ تب سے تبدیل ہو چکا ہو تو اس کے ساتھ رکھ دیا جاتا ہے۔
       *[other] اس کی ترتیبات، ڈیٹا اور لائسنس کے { $count } آئٹمز۔ جب تک یہ اختیار آن نہ ہو، یہ باقی رہتے ہیں۔ انہوں نے جو تبدیل کیا تھا وہ واپس آ جاتا ہے، یا اگر یہ تب سے تبدیل ہو چکے ہوں تو ان کے ساتھ رکھ دیا جاتا ہے۔
    }
uninstaller-uninstall = ان انسٹال کریں
uninstaller-cancel = منسوخ کریں
uninstaller-close = بند کریں
# { $done } and { $total } are how many of the install's items are undone, and how many there are.
uninstaller-step-counted = تنصیب کالعدم کی جا رہی ہے: { $total } میں سے { $done }…
uninstaller-step = تنصیب کالعدم کی جا رہی ہے…
uninstaller-done = ایپ ان انسٹال ہو گئی۔
uninstaller-done-except = ایپ ان انسٹال ہو گئی، سوائے ان چیزوں کے جو نیچے درج ہیں۔
uninstaller-waits-for-drive = اس کا کچھ حصہ ایسی ڈرائیو پر ہے جو منسلک نہیں ہے۔ اسے منسلک کریں، اور ترتیبات سے دوبارہ ان انسٹال کریں۔
uninstaller-not-all-undone = اس کا سب کچھ کالعدم نہیں کیا جا سکا۔ جو چیز اسے استعمال کر رہی ہے اسے بند کریں، اور ترتیبات سے دوبارہ ان انسٹال کریں۔
# Over the path of the uninstall's log file.
uninstaller-log = لاگ
uninstaller-undone = کالعدم کیا گیا
uninstaller-left-in-place = اپنی جگہ چھوڑ دیا گیا
uninstaller-could-not-be-undone = کالعدم نہیں کیا جا سکا
# After the first part of a long list. { $count } is how many more there are.
uninstaller-and-more = اور مزید { $count }۔
# Said as the reason when there is no record of the install to read.
uninstaller-no-programdata = اس PC کا ProgramData فولڈر نہیں مل سکا، اس لیے پڑھنے کے لیے کوئی ریکارڈ نہیں ہے

## When the uninstaller's window cannot open. Each is a paragraph of a message box, followed by a command to type.

uninstaller-window-failed-title = WinAppMigrator ان انسٹالر
# { $error } is what went wrong, as Windows or the graphics driver says it.
uninstaller-window-failed = ان انسٹالر کی ونڈو نہیں کھل سکی: { $error }
uninstaller-window-failed-quiet = یہ ونڈو کے بغیر بھی، منتظم کے کمانڈ پرامپٹ سے ان انسٹال کر سکتا ہے:
# Stands in for this program's own path in that command, if it cannot be found.
uninstaller-this-program = یہ پروگرام

## The uninstaller's command line (--quiet). Shown in English: scripts read it.

# { $app_name } is the name of the app.
uninstaller-cli-uninstalling = { $app_name } ان انسٹال کی جا رہی ہے۔
# { $path } is the log file.
uninstaller-cli-log = لاگ: { $path }
# { $error } is what went wrong.
uninstaller-cli-error = خرابی: { $error }
