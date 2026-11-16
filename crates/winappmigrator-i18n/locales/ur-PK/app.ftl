# Words of the app, WinAppMigrator itself: its one window, with the installed apps listed on the left and, on the
# right, what goes in an installer of the one picked and the button that builds it; and the steps to read another PC
# through WinAppMigrator Client. Message names start with app-.


## The window and its header

# The program's name, in the window's title bar and at the top of the window. Keep it as it is unless your language
# writes product names in a script of its own.
app-name = WinAppMigrator
app-tagline = انسٹال کردہ ایپ کو خود نکالنے والے انسٹالر میں تبدیل کریں
app-theme-to-light = ہلکی تھیم پر سوئچ کریں
app-theme-to-dark = گہری تھیم پر سوئچ کریں
app-reload-list = ایپلیکیشنز کی فہرست دوبارہ لوڈ کریں
app-list-loading = لوڈ ہو رہا ہے
# { $count } is how many apps are listed.
app-list-count =
    { $count ->
        [one] { $count } ایپ
       *[other] { $count } ایپس
    }
app-list-load-failed = لوڈ نہیں ہو سکا
# Shown when the window cannot open at all. { $error } is what went wrong, in the words of Windows or a graphics driver.
app-window-could-not-open = ونڈو نہیں کھل سکی: { $error }

## The list of installed apps

app-list-search = نام یا ناشر کے لحاظ سے تلاش کریں  (Ctrl+F)
app-list-filter-all = تمام
# Whether an app is installed for one account or for everyone on the PC.
app-scope-per-user = فی صارف
app-scope-per-machine = فی مشین
app-scope-unknown = نامعلوم
# An app whose processor architecture (x86, x64, ARM64) is not known.
app-arch-unknown = نامعلوم
app-list-loading-apps = انسٹال کردہ ایپلیکیشنز لوڈ ہو رہی ہیں…
# { $error } is why the apps could not be listed.
app-list-could-not-list = ایپلیکیشنز کی فہرست نہیں بن سکی: { $error }
app-list-no-matches = کوئی مماثل ایپلیکیشن نہیں
app-list-no-matches-hint = تلاش کا متن یا دائرہ کار کا فلٹر تبدیل کریں۔
app-list-worker-disconnected = فہرست بنانے والا ورکر منقطع ہو گیا۔
# What a screen reader says of an app's row. { $name } is the app's name, { $publisher } who made it, and { $version } its
# version, such as 11.4.2, or — when either is not known.
app-list-row = { $name }، { $publisher }، ورژن { $version }

## The list's columns, and what a screen reader says each column's heading does when clicked

app-column-name = نام
app-column-name-sort = نام کے لحاظ سے ترتیب دیں
app-column-publisher = ناشر
app-column-publisher-sort = ناشر کے لحاظ سے ترتیب دیں
app-column-version = ورژن
app-column-version-sort = ورژن کے لحاظ سے ترتیب دیں
app-column-scope = دائرہ کار
app-column-scope-sort = دائرہ کار کے لحاظ سے ترتیب دیں
# Short for architecture: x86, x64, or ARM64. The column is narrow.
app-column-arch = آرک
app-column-arch-sort = آرکیٹیکچر کے لحاظ سے ترتیب دیں

## Where apps are read from: this PC, a drive from another PC, a disk image of one, or another PC on the network

app-source = اسکین کا ماخذ
app-source-this-pc = یہ PC
app-source-drive = منسلک ڈرائیو…
app-source-image = ڈسک امیج…
app-source-network = نیٹ ورک PC…
app-source-attaching-image = امیج کو صرف پڑھنے کے لیے منسلک کیا جا رہا ہے…
# { $computer } is the other PC's name, such as OLD-PC, and { $address } its network address, such as 192.168.1.20.
app-source-network-pc = نیٹ ورک کے ذریعے { $computer } ({ $address })
app-source-disconnect = منقطع کریں
app-source-change = تبدیل کریں…
# The title of the window that picks a disk image. .vhd and .vhdx are file extensions.
app-source-pick-image-title = دوسرے PC کی ڈسک امیج منتخب کریں (.vhd یا .vhdx)
app-source-pick-image-filter = ڈسک امیجز
# { $path } is the disk image file.
app-source-image-has-no-windows = { $path } میں ایسا کوئی والیوم نہیں جس پر Windows ہو۔
app-source-image-not-attached = ڈسک امیج منسلک نہیں کی جا سکی۔
app-source-pick-drive-title = منسلک ڈرائیو کا روٹ منتخب کریں (مثلاً E:\)
# { $path } is the folder picked, such as E:\. \Windows and \Users are folder names, as Windows writes them.
app-source-not-a-windows-drive = { $path } Windows سسٹم ڈرائیو نہیں لگتی (اس میں \Windows اور \Users نہیں ہیں)۔ دوسرے PC کی ڈرائیو کا روٹ منتخب کریں۔
# Said of a drive or disk image with a Windows older than Windows 10. { $windows } is its name, such as "Windows 7
# Professional Service Pack 1, 32-bit". { $is_32_bit } is "yes" when that Windows is 32-bit, which adds the last
# sentence, and "no" when it is not.
app-source-older-windows =
    اس میں { $windows } ہے۔ اس سے بنائے گئے انسٹالرز Windows 10 اور 11 کے لیے ہوتے ہیں۔ صارف کا ڈیٹا شامل کرنے پر وہ ترتیبات بھی ساتھ آتی ہیں جو Windows نے کسی پرانی ایپ کے لیے خاموشی سے الگ رکھی تھیں، اور جو رن ٹائمز Windows 11 میں نہیں ہیں، جیسے DirectX 9، وہ ایپ کے ساتھ انسٹال کیے جاتے ہیں۔{ $is_32_bit ->
        [yes] { " " }اس کی ایپس 32 بٹ ہیں، اس لیے وہ Program Files (x86) میں جاتی ہیں، جہاں 64 بٹ Windows انہیں رکھتا ہے۔
       *[no] { "" }
    }
# The same, of a Windows whose name could not be read. { $is_32_bit } is as above.
app-source-older-windows-unnamed =
    اس میں Windows کا ایک پرانا ورژن ہے۔ اس سے بنائے گئے انسٹالرز Windows 10 اور 11 کے لیے ہوتے ہیں۔ صارف کا ڈیٹا شامل کرنے پر وہ ترتیبات بھی ساتھ آتی ہیں جو Windows نے کسی پرانی ایپ کے لیے خاموشی سے الگ رکھی تھیں، اور جو رن ٹائمز Windows 11 میں نہیں ہیں، جیسے DirectX 9، وہ ایپ کے ساتھ انسٹال کیے جاتے ہیں۔{ $is_32_bit ->
        [yes] { " " }اس کی ایپس 32 بٹ ہیں، اس لیے وہ Program Files (x86) میں جاتی ہیں، جہاں 64 بٹ Windows انہیں رکھتا ہے۔
       *[no] { "" }
    }

## The details panel, before an app is picked, and the app picked

app-details-pair-first = پرانے PC کے ساتھ جوڑا بنائیں
app-details-pair-first-hint = جب WinAppMigrator Client وہاں چل رہا ہو اور آپ اس کا کوڈ ٹائپ کر چکے ہوں، تو اس کی ایپس یہاں درج ہو جاتی ہیں، اور آپ ان میں سے کسی کا انسٹالر اسی طرح بناتے ہیں جیسے اس PC کی ایپ کا۔
app-details-pick-an-app = ایک ایپلیکیشن منتخب کریں
app-details-pick-an-app-hint = کیا شامل کرنا ہے یہ چننے اور اس کا انسٹالر بنانے کے لیے فہرست سے ایک ایپ منتخب کریں۔
# { $publisher } is who made the app, and { $version } its version, such as 11.4.2.
app-details-publisher-and-version = { $publisher }، ورژن { $version }
# { $version } is the app's version, such as 11.4.2.
app-details-unknown-publisher-and-version = نامعلوم ناشر، ورژن { $version }
app-details-unknown-publisher = نامعلوم ناشر

## What goes in besides the program

app-option-license = لائسنس اور فعال کاری کا ڈیٹا
app-option-license-hint = ہو سکتا ہے یہ اس PC سے بندھا ہو اور نئے PC پر دوبارہ فعال کاری کی ضرورت ہو۔
app-option-user-data = صارف کا ڈیٹا
# Documents, Saved Games, and ProgramData are folders of Windows: use the names Windows gives them in your language,
# except ProgramData, which keeps its name.
app-option-user-data-hint = ترتیبات اور سرگزشت، دستاویزات، محفوظ کردہ گیمز، اور ProgramData۔
app-option-whose-data = کس کا ڈیٹا
app-option-whose-data-hint = ایک انسٹالر میں ایک اکاؤنٹ کا ڈیٹا ہوتا ہے۔ یہ جہاں چلایا جائے، وہاں جو بھی سائن ان ہو، اسی کے پاس جاتا ہے۔

## Building the installer

app-output-folder = آؤٹ پٹ فولڈر
# An example of a folder, shown in the empty field. "You" stands for the name of an account.
app-output-folder-example = C:\Users\You\WinAppMigrator
app-output-folder-not-full = مکمل فولڈر پاتھ درج کریں، جیسے D:\Installers۔
app-build-create = انسٹالر بنائیں
app-build-create-again = انسٹالر دوبارہ بنائیں
app-build-cancel = منسوخ کریں
app-build-cancelling = موجودہ مرحلے کے بعد منسوخ کیا جا رہا ہے…
app-build-cancelled = انسٹالر بنانا منسوخ کر دیا گیا۔ کوئی انسٹالر نہیں لکھا گیا۔
# winappmigrator-installer.exe is a file name.
app-build-no-stub = انسٹالر اسٹب (winappmigrator-installer.exe) اس ایپ کے ساتھ نہیں ملا، اس لیے خود نکالنے والا انسٹالر نہیں بنایا جا سکتا۔
app-build-select-an-app-first = پہلے منتقل کرنے کے لیے ایک ایپلیکیشن منتخب کریں۔
app-build-look-again-first = انسٹالر بنانے سے پہلے اس ایپ کا دوبارہ جائزہ لینا ضروری ہے۔
# { $reason } is what went wrong, in the program's own words, often in English.
app-build-stopped-unexpectedly = انسٹالر بنانے کا عمل غیر متوقع طور پر رک گیا: { $reason }۔ اگر اس نے آؤٹ پٹ فولڈر میں کوئی انسٹالر چھوڑا ہے تو اسے استعمال نہ کریں۔
app-build-worker-disconnected = منتقلی کا ورکر منقطع ہو گیا۔
# { $path } is the output folder, and { $error } what Windows said went wrong.
app-build-no-output-folder = آؤٹ پٹ فولڈر { $path } نہیں بنایا جا سکا: { $error }
# { $error } is what went wrong.
app-build-no-staging-folder = محفوظ اسٹیجنگ فولڈر نہیں بنایا جا سکا، اس لیے کچھ نہیں بنایا گیا: { $error }
# { $error } is what went wrong, as a sentence.
app-build-could-not-build = انسٹالر نہیں بنایا جا سکا۔ { $error }
app-build-no-installer = انسٹالر بنانے کا عمل کسی انسٹالر کے بغیر ختم ہو گیا۔
# { $error } is what went wrong.
app-build-hash-check-did-not-run = انسٹالر لکھ دیا گیا، لیکن اس کی ہیش کی جانچ نہیں چل سکی: { $error }۔ اسے استعمال نہ کریں۔

## How far a build has got

app-progress-preparing = معلوم کیا جا رہا ہے کہ اس میں کیا شامل ہوگا…
app-progress-checking-room = جانچ کی جا رہی ہے کہ جگہ موجود ہے…
app-progress-bundling = رن ٹائمز اور ڈرائیورز شامل کیے جا رہے ہیں…
# { $done } is how many files have been read so far, and { $total } how many there are.
app-progress-packing = فائلیں پڑھی اور کمپریس کی جا رہی ہیں: { $total } میں سے { $done }
# { $done } is how many of the installer's records have been looked over so far, and { $total } how many there are.
app-progress-looking-over = انسٹالر کے مندرجات کا جائزہ لیا جا رہا ہے: { $total } میں سے { $done }…
app-progress-writing = انسٹالر لکھا جا رہا ہے…
app-progress-verifying = ہیشز کی تصدیق کی جا رہی ہے…

## What a build made

app-result-failed-hash-check = یہ انسٹالر استعمال نہ کریں: یہ اپنی ہیش کی جانچ میں ناکام رہا۔
# { $count } is how many files were gone when the build came to read them.
app-result-files-gone =
    { $count ->
        [one] انسٹالر بن گیا، لیکن ایپ کا جائزہ لینے کے بعد سے { $count } فائل غائب ہو چکی تھی، اس لیے وہ اس میں شامل نہیں ہے۔ نیچے انتباہات دیکھیں۔
       *[other] انسٹالر بن گیا، لیکن ایپ کا جائزہ لینے کے بعد سے { $count } فائلیں غائب ہو چکی تھیں، اس لیے وہ اس میں شامل نہیں ہیں۔ نیچے انتباہات دیکھیں۔
    }
# { $count } is how many files could not be read into the installer.
app-result-files-not-captured =
    { $count ->
        [one] انسٹالر بن گیا، لیکن { $count } فائل حاصل نہیں کی جا سکی، عموماً اس لیے کہ وہ لاک تھی یا پڑھی نہیں جا سکتی تھی۔ ہو سکتا ہے ایپ اس کے بغیر کام نہ کرے۔ نیچے انتباہات دیکھیں۔
       *[other] انسٹالر بن گیا، لیکن { $count } فائلیں حاصل نہیں کی جا سکیں، عموماً اس لیے کہ وہ لاک تھیں یا پڑھی نہیں جا سکتی تھیں۔ ہو سکتا ہے ایپ ان کے بغیر کام نہ کرے۔ نیچے انتباہات دیکھیں۔
    }
app-result-mentions-who = انسٹالر بن گیا، اور اس میں اب بھی کچھ ایسا درج ہے جس سے پتا چلتا ہے کہ یہ کس کے پاس سے آیا ہے۔ کسی کو دینے سے پہلے نیچے پڑھ لیں کہ اس میں کیا لکھا ہے۔
app-result-ready = انسٹالر تیار ہے۔ اس کا پیکیج اپنی ہیش کی جانچ میں کامیاب رہا۔
app-result-files = فائلیں
app-result-installer-size = انسٹالر کا سائز
app-result-warnings = انتباہات
app-result-log = لاگ
app-result-copy-error = خرابی کاپی کریں
app-result-installer = انسٹالر
app-result-data-file = ڈیٹا فائل
app-result-open-folder = فولڈر کھولیں
app-result-copy-path = پاتھ کاپی کریں
# { $error } is what Windows said went wrong.
app-result-could-not-open-folder = فولڈر نہیں کھل سکا: { $error }
# { $installer } and { $data_file } are file names, such as App_installer.exe and App_installer.wamdata.
app-result-two-files = یہ ایپ ایک قابل عمل فائل کے لیے بہت بڑی ہے، اس لیے اس کا انسٹالر دو فائلوں پر مشتمل ہے، { $installer } اور { $data_file }۔ انہیں ایک ساتھ رہنا چاہیے: دونوں کو دوسرے PC پر، ایک ہی فولڈر میں، انہی ناموں کے ساتھ کاپی کریں۔ اگر آپ انسٹالر پر دستخط کرتے ہیں تو وہ دستخط ڈیٹا فائل کا بھی احاطہ کرتے ہیں۔
# { $app_name } is the app's name. "Unknown publisher" is what Windows shows when it asks to run an unsigned program: use
# Windows' own words for it in your language.
app-result-run-it = { $app_name } کو دوبارہ انسٹال کرنے کے لیے اسے دوسرے PC پر چلائیں۔ یہ SHA-256 کے ذریعے اپنی فائلوں کی جانچ کرتا ہے۔ Windows "نامعلوم ناشر" دکھاتا ہے کیونکہ آپ کے بنائے ہوئے انسٹالرز دستخط شدہ نہیں ہوتے۔
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-verification-errors = تصدیق کی خرابیاں ({ $count })
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-warnings-heading = انتباہات ({ $count })
# After the first of a long list of warnings. { $count } is how many more there are.
app-warnings-and-more = اور مزید { $count }۔

## Looking at an app, to see what an installer of it would hold

app-scan-getting-ready = اس ایپ کا جائزہ لینے کی تیاری ہو رہی ہے…
app-scan-install-record = پڑھا جا رہا ہے کہ اسے کیسے انسٹال کیا گیا تھا…
app-scan-near-matches = تلاش کیا جا رہا ہے کہ اور کیا اس کا حصہ ہو سکتا ہے…
app-scan-system-items = اس کی سروسز، ٹاسکس اور شارٹ کٹس تلاش کیے جا رہے ہیں…
app-scan-files = اس کی فائلوں کا جائزہ لیا جا رہا ہے…
app-scan-registry = پڑھا جا رہا ہے کہ یہ Windows سے کیسے جڑی ہے…
app-scan-finishing = مکمل کیا جا رہا ہے…
app-scan-look-again = دوبارہ جائزہ لیں
app-scan-stop = روکیں
app-scan-running-hint = جب یہ چل رہا ہو، آپ کوئی دوسری ایپ منتخب کر سکتے ہیں یا شامل کی جانے والی چیزیں تبدیل کر سکتے ہیں۔
# { $error } is why, as a sentence.
app-scan-failed = اس ایپ کا جائزہ نہیں لیا جا سکا: { $error }
app-scan-try-again = دوبارہ کوشش کریں
# "Look Again" names the button of that name.
app-scan-stopped = روک دیا گیا۔ اس ایپ کا جائزہ مکمل ہونے کے بعد انسٹالر بنایا جا سکتا ہے۔
# { $reason } is what went wrong, in the program's own words, often in English.
app-scan-stopped-unexpectedly = یہ غیر متوقع طور پر رک گیا: { $reason }۔

## How big an installer can be, and whether there is room for it

app-room-files = فائلیں
# Over how big the installer can be: up to that size.
app-room-up-to = زیادہ سے زیادہ
app-room-free = خالی
# { $drive } is a drive, such as C:.
app-room-free-on = { $drive } پر خالی
# How much room a drive has, when it would not say.
app-room-free-unknown = نامعلوم
app-room-most-it-could-need = کمپریس کرنے سے پہلے، اسے زیادہ سے زیادہ اتنی جگہ درکار ہو سکتی ہے۔
# { $drive } is a drive, such as C:, { $needed } how much room it needs, such as 1.2 GB, and { $free } how much it has.
app-room-drive-short = اسے بنانے کے لیے { $drive } پر زیادہ سے زیادہ { $needed } خالی جگہ درکار ہے، اور اس پر { $free } ہے۔
# { $drives } is one sentence like the one above for each drive without the room, one after another.
app-room-not-enough = کافی جگہ نہیں ہے۔ { $drives } کسی دوسری ڈرائیو پر آؤٹ پٹ فولڈر منتخب کریں، یا کچھ چیزیں خارج کریں۔
# { $note } is a sentence, without its full stop, about a drive whose room could not be checked. Use your language's
# full stop.
app-room-could-not-tell = { $note }۔

## What goes in the installer, part by part

app-footprint-heading = انسٹالر میں کیا شامل ہوتا ہے
app-footprint-working-out = معلوم کیا جا رہا ہے کہ اس کے ساتھ اور کیا جاتا ہے…
# { $error } is why.
app-footprint-would-not-make-a-package = جو باقی بچا ہے اس سے پیکیج نہیں بنے گا: { $error }
# { $list } is the names of what else is left out, such as a service and a shortcut.
app-footprint-also-left-out = اس سے { $list } بھی خارج ہو جاتے ہیں، کیونکہ جو چیز وہ چلاتے ہیں یا جس کا وہ نام لیتے ہیں وہ وہاں نہیں ہوگی۔ انہیں دوسرے PC پر سیٹ اپ نہیں کیا جائے گا۔
# { $list } is the names of the first few of what else is left out, and { $count } how many more there are.
app-footprint-also-left-out-and-more = اس سے { $list }، اور مزید { $count } بھی خارج ہو جاتے ہیں، کیونکہ جو چیز وہ چلاتے ہیں یا جس کا وہ نام لیتے ہیں وہ وہاں نہیں ہوگی۔ انہیں دوسرے PC پر سیٹ اپ نہیں کیا جائے گا۔
# { $list } is the names of license files. "License" names the License and Activation Data option.
app-footprint-licenses-still-go = آپ کی خارج کردہ چیزوں میں موجود لائسنس کا ڈیٹا پھر بھی شامل ہوتا ہے، کیونکہ اس کا فیصلہ لائسنس اور فعال کاری کے ڈیٹا کے اختیار سے ہوتا ہے: { $list }۔
# { $elapsed } is how long ago, in minutes and seconds such as 45:00, or hours, minutes, and seconds such as 1:05:00.
# "Look Again" names the button of that name.
app-footprint-looked-at-long-ago = اس کا جائزہ { $elapsed } پہلے لیا گیا تھا۔ فائلیں انسٹالر بناتے وقت پڑھی جاتی ہیں، اس لیے کوئی تبدیلی شامل ہو جاتی ہے، اور جو چیز تب تک غائب ہو چکی ہو وہ درج کر دی جاتی ہے۔ کسی نئی چیز کے لیے "دوبارہ جائزہ لیں" منتخب کریں۔
# Headings of the parts of the list, each with a line that says what is in it.
app-footprint-program-files = پروگرام فائلیں
app-footprint-data-folders = ڈیٹا فولڈرز
# What could go in with the User Data option, when nothing else is in this part. { $size } is how much, such as 1.2 GB.
app-footprint-folders-with-user-data = صارف کے ڈیٹا کے اختیار کے ساتھ { $size }
# { $included } is how many folders go in, { $count } how many there are, and { $size } how much those that go in hold.
app-footprint-folders-summary =
    { $count ->
        [one] { $count } میں سے { $included } فولڈر، { $size }
       *[other] { $count } میں سے { $included } فولڈرز، { $size }
    }
# { $size } is how much, such as 1.2 GB, and { $count } in how many files.
app-footprint-data-only-with-user-data =
    { $count ->
        [one] اس ایپ کے ڈیٹا کی { $count } فائل میں موجود { $size } صرف صارف کے ڈیٹا کے اختیار کے ساتھ شامل ہوگا۔
       *[other] اس ایپ کے ڈیٹا کی { $count } فائلوں میں موجود { $size } صرف صارف کے ڈیٹا کے اختیار کے ساتھ شامل ہوگا۔
    }
# A folder's row. { $count } is how many files it holds, and { $detail } where it is or why it cannot be left out.
app-footprint-folder-row =
    { $count ->
        [one] { $count } فائل۔ { $detail }
       *[other] { $count } فائلیں۔ { $detail }
    }
app-footprint-services = سروسز
app-footprint-startup = خود بخود شروع ہوتا ہے
app-footprint-tasks = شیڈول کردہ ٹاسکس
app-footprint-shortcuts = شارٹ کٹس
# { $included } is how many of the part's things go in, and { $count } how many there are.
app-footprint-items-summary = { $count } میں سے { $included }
# { $why } is what else was left out that this goes with.
app-footprint-left-out-too = یہ بھی خارج: { $why }
# A row's detail. { $kind } is what it is, such as Service, and { $detail } what it does.
app-footprint-change-detail = { $kind }۔ { $detail }
app-footprint-integrations = یہ Windows سے کیسے جڑی ہے
# { $included } is how many kinds go in, such as file types and firewall rules, and { $count } how many kinds there are.
app-footprint-integrations-summary =
    { $count ->
        [one] { $count } میں سے { $included } قسم
       *[other] { $count } میں سے { $included } اقسام
    }
app-footprint-integration-nothing-left = آپ کی خارج کردہ چیزوں کی وجہ سے اس میں سے کچھ باقی نہیں رہا
# { $count } is how many things of this kind the installer sets up.
app-footprint-integration-things =
    { $count ->
        [one] { $count } چیز سیٹ اپ کی جاتی ہے
       *[other] { $count } چیزیں سیٹ اپ کی جاتی ہیں
    }
app-footprint-prerequisites = اسے چلنے کے لیے کیا درکار ہے
# { $count } is how many runtimes, such as Visual C++ Redistributable.
app-footprint-prerequisites-summary =
    { $count ->
        [one] { $count } رن ٹائم
       *[other] { $count } رن ٹائمز
    }
app-footprint-near-matches = یہ بھی اس کا حصہ ہو سکتا ہے
# { $ticked } is how many were ticked, and { $count } how many there are.
app-footprint-near-matches-summary = { $count } میں سے { $ticked } شامل کیے گئے
app-footprint-near-matches-hint = جب تک آپ ان پر نشان نہ لگائیں، یہ انسٹالر میں شامل نہیں ہوتے۔ کسی پر نشان لگانے سے ایپ کا دوبارہ جائزہ لیا جاتا ہے، باقی سب جیسی جانچ کے ساتھ، اس لیے جو چیز پاس ورڈ جیسی لگے وہ پھر بھی باہر رہتی ہے۔
# { $why } is why it might be part of the app, as a sentence.
app-footprint-near-match-with-user-data = صارف کے ڈیٹا کے اختیار کے ساتھ آتا ہے۔ { $why }
# How much a folder holds, when not all of it could be measured. { $size } is how much was, such as 1.2 GB.
app-footprint-near-match-over = { $size } سے زیادہ
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-folder = فولڈر: { $place }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-hidden = ایپ کا پوشیدہ حصہ: { $place }
app-footprint-nothing-left-out = پاس ورڈ، لائسنس یا سرٹیفکیٹ جیسا لگنے کی وجہ سے کچھ بھی خارج نہیں کیا گیا۔
app-footprint-left-out = خارج کردہ، اور کیوں
# { $count } is how many things were left out.
app-footprint-left-out-summary =
    { $count ->
        [one] { $count } چیز
       *[other] { $count } چیزیں
    }
# Above a list of things left out for one reason. { $title } is the reason.
app-footprint-left-out-group = { $title }:
app-footprint-refused = کسی بھی انسٹالر کو انہیں سیٹ اپ کرنے کی اجازت نہیں ہوگی:
# A heading over a list that opens and closes. { $count } is how many are in it.
app-footprint-worth-knowing = جاننے کے لائق ({ $count })
# { $list } is what kinds of things were looked for and none found, from the five below.
app-none-found = کچھ نہیں ملا: { $list }۔
app-none-found-services = سروسز
app-none-found-startup = اسٹارٹ اپ آئٹمز
app-none-found-tasks = شیڈول کردہ ٹاسکس
app-none-found-shortcuts = شارٹ کٹس
app-none-found-integrations = کوئی ایسی چیز جو اسے Windows سے جوڑتی ہو

## Network PC: reading another PC through WinAppMigrator Client, in three steps. "WinAppMigrator Client" is the name of a
## program, and the old PC is the one the apps are moved off.

app-network-step-client = 1. پرانے PC پر کلائنٹ رکھیں
app-network-step-client-hint = WinAppMigrator Client اس PC پر چلتا ہے جس پر ایپ ابھی موجود ہے، Windows 7 یا بعد کے ورژن پر۔ یہ صرف اس PC کو، مرموز طریقے سے اور اپنے دکھائے ہوئے کوڈ کے ذریعے، اسے پڑھنے دیتا ہے۔
# { $drive } is a USB drive, such as E:\.
app-network-save-to-usb = USB ڈرائیو { $drive } میں محفوظ کریں
# { $drive } is a USB drive, such as E:\, and { $name } the name it was given, such as BACKUP.
app-network-save-to-named-usb = USB ڈرائیو { $drive } ({ $name }) میں محفوظ کریں
app-network-save-to-folder = کسی فولڈر میں محفوظ کریں…
app-network-look-for-usb-again = USB ڈرائیوز دوبارہ تلاش کریں
app-network-no-usb = کوئی USB ڈرائیو لگی ہوئی نہیں ہے۔ ایک لگائیں، یا کسی فولڈر میں محفوظ کریں: ایسا فولڈر جس تک پرانے PC سے پہنچا جا سکے، جیسے آپ کے نیٹ ورک پر کوئی مشترکہ فولڈر۔
# { $path } is where the client was saved, such as E:\WinAppMigrator Client.exe.
app-network-client-saved = { $path } کے طور پر محفوظ کیا گیا۔ پرانے PC پر، جو ایپ آپ منتقل کر رہے ہیں اسے بند کریں، پھر اس فائل پر ڈبل کلک کریں اور اسے تبدیلیاں کرنے دیں۔ یہ ایک کوڈ دکھاتا ہے۔
app-network-step-pick = 2. پرانا PC منتخب کریں
app-network-looking = آپ کے نیٹ ورک پر ایسے PCs تلاش کیے جا رہے ہیں جن پر کلائنٹ چل رہا ہے…
app-network-type-name = یا اس کا نام یا پتہ ٹائپ کریں
# An example, shown in the empty field: a PC's name, or its network address.
app-network-type-name-example = OLD-PC یا 192.168.1.20
app-network-step-code = 3. اس کا دکھایا ہوا کوڈ ٹائپ کریں
app-network-code = جوڑا بنانے کا کوڈ
app-network-connect = مربوط کریں
app-network-pairing = جوڑا بنایا جا رہا ہے…
# Why pairing did not work, shown on its own.
app-network-pairing-stopped = جوڑا بنانے کا عمل مکمل ہونے سے پہلے رک گیا
app-network-wrong-code = یہ وہ کوڈ نہیں ہے جو پرانا PC دکھاتا ہے۔ اسے چیک کریں اور دوبارہ کوشش کریں۔
# { $error } is what went wrong.
app-network-not-reached = پرانے PC تک نہیں پہنچا جا سکا: { $error }
app-network-pick-folder-title = WinAppMigrator Client کے لیے فولڈر منتخب کریں
# { $path } is the folder, and { $error } what Windows said went wrong.
app-client-could-not-save = کلائنٹ کو { $path } میں محفوظ نہیں کیا جا سکا: { $error }
# scripts\build-client.ps1 is a file name.
app-client-not-carried = WinAppMigrator کے اس بلڈ میں کلائنٹ شامل نہیں ہے۔ اسے scripts\build-client.ps1 سے بنائیں، پھر ایپ دوبارہ بنائیں۔
# { $error } is what Windows said went wrong.
app-client-could-not-save-note = اس کے ساتھ والا نوٹ محفوظ نہیں کیا جا سکا: { $error }
# A note saved as a text file beside the client, for whoever runs it on the old PC. Keep its lines short, as it is read in
# Notepad. "WinAppMigrator Client.exe" is a file name. "Network PC..." names the Scan Source button: use its words in your
# language.
app-client-note =
    WinAppMigrator Client
    
    یہ اس PC پر چلائیں جس سے آپ ایپس منتقل کر رہے ہیں: Windows 7، 8، 10 یا 11۔
    
    1. جو ایپ آپ منتقل کر رہے ہیں اسے بند کریں۔
    2. "WinAppMigrator Client.exe" پر ڈبل کلک کریں اور اسے تبدیلیاں کرنے دیں۔
    3. یہ 6 ہندسوں کا جوڑا بنانے کا کوڈ دکھاتا ہے۔ اپنے نئے PC پر WinAppMigrator
       میں نیٹ ورک PC... منتخب کریں، یہ PC چنیں، اور کوڈ ٹائپ کریں۔
    4. جب نئے PC کا کام ہو جائے تو کلائنٹ کی ونڈو میں Enter دبائیں، یا اسے
       بند کریں۔
    
    یہ صرف پڑھتا ہے۔ چلتے وقت یہ Windows ڈرائیو کا ایک اسنیپ شاٹ رکھتا ہے اور
    آپ کے مقامی نیٹ ورک سے اپنی دو پورٹس کو Windows فائر وال سے گزرنے دیتا
    ہے، اور رکنے پر دونوں کو ہٹا دیتا ہے۔ نیٹ ورک پر جانے والا ڈیٹا مرموز ہوتا
    ہے، اور صرف وہی PC کچھ پڑھ سکتا ہے جس کے پاس کوڈ ہو۔
