# Words the core writes for people as it installs an app from a package: why an install will not start, the list of
# what it did (or for a preview, what it would do) that the installer window shows, its warnings, the record it keeps of
# each install so that it can be undone, and the core's errors. Message names start with core-.
#
# The list of what an install did is short and plain, one line for each thing, as the installer window lists them:
# "copied", "restored", "would copy". Words such as --force and --apply are command-line options, and LocalService,
# SYSTEM, ProgramData, and %AppData% are as Windows names them: keep them as they are.


## Errors: what went wrong, when the core cannot go on. Each is shown on its own, or after words that say what was being
## done.

core-error-list-changed = اس تنصیب پر اتفاق کے بعد سے یہ جو کچھ کرے گی وہ بدل گیا ہے، اس لیے کچھ انسٹال نہیں کیا گیا
# { $path } is a file or folder, such as C:\Program Files\App\app.exe, or words that name one, such as "the installer".
# { $error } is what Windows said went wrong.
core-error-io = { $path } کے لیے I/O ناکام ہو گیا: { $error }
# { $detail } is what went wrong.
core-error-registry = رجسٹری پڑھنا ناکام ہو گیا: { $detail }
# { $path } is a program file. { $detail } is what was wrong with it.
core-error-pe = { $path } کے لیے PE پارس کرنا ناکام ہو گیا: { $detail }
# { $detail } is what was wrong, already in words.
core-error-invalid-input = غلط ان پٹ: { $detail }
# { $detail } is what went wrong.
core-error-serialization = رپورٹ کو سیریلائز کرنا ناکام ہو گیا: { $detail }
# { $detail } is what went wrong.
core-error-archive = آرکائیو کا عمل ناکام ہو گیا: { $detail }
# { $detail } is what went wrong.
core-error-network = نیٹ ورک کا عمل ناکام ہو گیا: { $detail }
# { $detail } is what went wrong.
core-error-time-format = وقت کی فارمیٹنگ ناکام ہو گئی: { $detail }
core-error-cancelled = اسے منسوخ کر دیا گیا
# What is said when part of the program stopped unexpectedly and gave no reason.
core-error-panic-no-reason = کوئی وجہ نہیں بتائی گئی

## The record of each install, kept so that it can be undone.

# { $kind } is a number that says what type of registry value it is.
core-journal-unknown-value-type = نامعلوم رجسٹری قدر کی قسم { $kind }
core-journal-not-elevated = منتظم کے حقوق کے ساتھ نہیں چل رہا
core-journal-needs-admin = انسٹال کرنے کے لیے منتظم کے حقوق درکار ہیں، تاکہ اس کا ریکارڈ ایسی جگہ رکھا جائے جہاں دوسرے اسے تبدیل نہ کر سکیں
# { $path } is a folder.
core-journal-not-an-install-folder = یہ کسی تنصیب کا فولڈر نہیں ہے: { $path }
# { $path } is the file the record is kept in.
core-journal-not-written = اس تنصیب کا ریکارڈ نہیں لکھا جا سکا: { $path }
# { $path } is a folder.
core-journal-path-escapes-root = پاتھ مطلوبہ روٹ سے باہر جاتا ہے: { $path }
# { $path } is a folder.
core-journal-nothing-exists = اس میں سے کچھ بھی اس PC پر موجود نہیں ہے: { $path }
core-journal-busy = کوئی اور تنصیب یا ان انسٹال چل رہا ہے۔ اس کے ختم ہونے کا انتظار کریں، پھر دوبارہ کوشش کریں
# Said of a file that was to get back the permissions it had.
core-journal-no-folder = اس کا کوئی فولڈر نہیں ہے
# Said of the permissions a file had, which were kept to give back to it.
core-journal-permissions-unreadable = جو محفوظ رکھا گیا تھا وہ پڑھا نہیں جا سکا
# { $folder } is the folder of one install's record.
core-journal-unreadable-empty = { $folder } میں موجود تنصیب کا ریکارڈ استعمال نہیں کیا جا سکتا: یہ خالی ہے
# { $folder } is the folder of one install's record.
core-journal-unreadable-no-header = { $folder } میں موجود تنصیب کا ریکارڈ استعمال نہیں کیا جا سکتا: اس کے شروع میں یہ درج نہیں کہ یہ کس بارے میں ہے
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-version = { $folder } میں موجود تنصیب کا ریکارڈ استعمال نہیں کیا جا سکتا: اسے اس پروگرام کے کسی دوسرے ورژن نے لکھا ہے
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-install = { $folder } میں موجود تنصیب کا ریکارڈ استعمال نہیں کیا جا سکتا: اس کے مطابق یہ کسی دوسری تنصیب کا ہے
# { $id } is an install's id, such as wam-1a2b3c.
core-journal-not-an-install-id = یہ کسی تنصیب کی ID نہیں ہے: { $id }

## Why an install will not start, or stopped before it changed anything.

core-restore-apply-and-dry-run = --apply اور --dry-run دونوں ایک ساتھ نہیں دیے جا سکتے
# { $path } is the folder that was chosen to install in.
core-restore-location-not-full-path = تنصیب کا مقام مکمل پاتھ ہونا چاہیے: "{ $path }"
# { $problem } says what is wrong with the folder, already in words. { $path } is the folder.
core-restore-location-unusable = تنصیب کا وہ مقام استعمال نہیں کیا جا سکتا: { $problem }: "{ $path }"
core-restore-needs-admin-plan = بحالی کے منصوبے میں رجسٹری، سروس، شیڈول کردہ ٹاسک، شارٹ کٹ یا محفوظ مقام کی ایسی تبدیلیاں شامل ہیں جن کے لیے منتظم کے حقوق درکار ہیں
# { $path } is the folder that was chosen to install in.
core-restore-needs-admin-target = ہدف روٹ کے لیے منتظم کے حقوق درکار ہیں: { $path }
core-restore-needs-admin-user-data = صارف کا ڈیٹا ProgramData یا دیگر محفوظ مقامات میں بحال کرنے کے لیے منتظم کے حقوق درکار ہیں
# { $error } is why the install stopped. { $failed } is what could not be put back, as sentences of their own.
core-restore-not-all-put-back = { $error }۔ جو کچھ کیا جا چکا تھا وہ سب واپس نہیں رکھا جا سکا: { $failed }
# { $error } is what went wrong.
core-restore-nowhere-to-keep-record = وہ ریکارڈ رکھنے کے لیے کوئی محفوظ جگہ نہیں ہے جس سے اس تنصیب کو کالعدم کیا جا سکے، اس لیے کچھ انسٹال نہیں کیا گیا: { $error }
# { $path } is a file the app would have put there.
core-restore-already-exists = { $path } پہلے سے موجود ہے، اس لیے کچھ انسٹال نہیں کیا گیا۔ اسے تبدیل کرنے کے لیے --force استعمال کریں
# { $root } is a folder the app has files for. { $link } is that folder, or one it is in.
core-restore-root-under-link = اس ایپ کے پاس { $root } میں بحال کرنے کے لیے چیزیں ہیں، اور { $link } کسی دوسری جگہ کا لنک ہے، جسے شاید دوسرے تبدیل کر سکیں، اس لیے کچھ انسٹال نہیں کیا گیا۔ اگر وہاں جانے والی چیز صارف کا ڈیٹا یا لائسنس کا ڈیٹا ہے، تو اسے صارف کے ڈیٹا اور لائسنس کے ڈیٹا کے بغیر انسٹال کریں۔ اگر یہ کسی اسٹارٹ اپ فولڈر میں شارٹ کٹ ہے، تو اس فولڈر کو واپس وہیں رکھیں جہاں وہ تھا
core-restore-hash-check-failed = پیکیج اپنی ہیش کی جانچ میں ناکام رہا اور کچھ انسٹال نہیں کیا گیا
# { $detail } is the first thing found wrong, already in words.
core-restore-hash-check-failed-because = پیکیج اپنی ہیش کی جانچ میں ناکام رہا اور کچھ انسٹال نہیں کیا گیا: { $detail }
# { $path } is a file or folder.
core-restore-refuse-records-folder = وہاں لکھنے سے انکار کیا گیا جہاں تنصیبات اپنے ریکارڈ رکھتی ہیں: { $path }
# { $path } is a file or folder.
core-restore-refuse-outside = تنصیب کے مقام اور معلوم ڈیٹا فولڈرز سے باہر لکھنے سے انکار کیا گیا: { $path }
# { $path } is a file.
core-restore-no-parent-folder = منزل کا کوئی پیرنٹ فولڈر نہیں ہے: { $path }
# { $path } is where a file was to go.
core-restore-folder-in-the-way = جہاں اس فائل کو جانا ہے وہاں ایک فولڈر موجود ہے: { $path }
# { $path } is a file in the package.
core-restore-file-changed = { $path } اب وہ نہیں رہا جو پیکیج میں ہے، اس لیے اسے انسٹال نہیں کیا گیا
# { $path } is a file in the package.
core-restore-source-is-link = { $path } ایک لنک ہے، اور اس کی پیروی نہیں کی جاتی
# { $path } is a file in the package.
core-restore-source-became-link = { $path } کھولے جانے کے دوران لنک بن گیا
# Why a service did not start, after "did not start:".
core-restore-service-stopped-by-itself = یہ خود ہی دوبارہ رک گئی
# Why part of the app was left out, after "was left out:".
core-restore-com-class-someone-elses = یہ COM کلاس اس PC پر پہلے سے کسی اور چیز کی ملکیت کے طور پر موجود ہے

## What an install did, one line for each thing, or for a preview, what it would do. { $source } is a file in the
## package, and { $destination } and { $path } are where it goes on this PC. { $key } is a registry key, such as
## HKLM\Software\App.

# { $package_valid } is true or false, and { $restore_valid } is true, false, or not_run: keep them as they are.
core-restore-verification-result = بحالی کے بعد کی تصدیق package_valid={ $package_valid } restore_valid={ $restore_valid }
# { $name } is a service's name.
core-restore-started-service = سروس { $name } شروع کی گئی
core-restore-would-copy = { $source } کو { $destination } میں کاپی کیا جائے گا
core-restore-copied = { $source } کو { $destination } میں کاپی کیا گیا
core-restore-would-create-empty-folder = خالی فولڈر { $path } بنایا جائے گا
core-restore-created-empty-folder = خالی فولڈر { $path } بنایا گیا
# { $value } is the name of a value in the key.
core-restore-would-restore-registry-value = رجسٹری قدر { $key } { $value } بحال کی جائے گی
core-restore-would-restore-registry-key = رجسٹری کلید { $key } بحال کی جائے گی
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-would-add-firewall-rule = { $program } کے لیے فائر وال قاعدہ { $name } شامل کیا جائے گا
# { $name } is a firewall rule's name.
core-restore-firewall-rule-already-there = فائر وال قاعدہ { $name } پہلے سے موجود ہے
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-added-firewall-rule = { $program } کے لیے فائر وال قاعدہ { $name } شامل کیا گیا
# { $name } is a service's name, { $command } what it runs, and { $account } the account it runs as, such as LocalService.
core-restore-would-restore-service = سروس { $name } بحال کی جائے گی، جو { $command } چلاتی ہے ({ $account } کے طور پر)
# { $name } is a runtime, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-restore-prerequisite-present = پیشگی ضرورت پہلے سے موجود ہے: { $name }
# { $name } is a runtime that Windows has as a Windows feature.
core-restore-would-install-prerequisite-feature = پیشگی ضرورت { $name } انسٹال کی جائے گی (Windows خصوصیت)
# { $name } is a runtime whose installer is in the package.
core-restore-would-install-prerequisite-bundled = پیشگی ضرورت { $name } انسٹال کی جائے گی (شامل کردہ انسٹالر)
# { $name } is a runtime that would come through winget, which is a program's name.
core-restore-would-install-prerequisite-winget = پیشگی ضرورت { $name } انسٹال کی جائے گی (winget)
# { $name } is a runtime.
core-restore-installed-prerequisite = پیشگی ضرورت { $name } انسٹال کی گئی
# { $name } is a runtime.
core-restore-installed-prerequisite-reboot = پیشگی ضرورت { $name } انسٹال کی گئی (دوبارہ شروع کرنا درکار ہے)
core-restore-would-restore-license = لائسنس { $source } کو { $destination } میں بحال کیا جائے گا
core-restore-restored-license = لائسنس { $source } کو { $destination } میں بحال کیا گیا
core-restore-would-restore-license-registry = لائسنس کی رجسٹری { $key } بحال کی جائے گی
# { $name } is a driver's name.
core-restore-driver-not-restored = ڈرائیور خود بخود بحال نہیں کیا گیا: { $name }
# { $name } is a driver's name.
core-restore-would-install-driver = ڈرائیور { $name } کو { $destination } میں انسٹال کیا جائے گا
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-would-restore-task = شیڈول کردہ ٹاسک { $name } بحال کیا جائے گا
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-startup-shortcut = اسٹارٹ اپ شارٹ کٹ { $shortcut } بحال کیا جائے گا، جو { $target } کھولتا ہے
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-shortcut = شارٹ کٹ { $shortcut } بحال کیا جائے گا، جو { $target } کھولتا ہے
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-would-restore-variable =
    { $scope ->
        [user] صارف کا ماحولیاتی متغیر { $name } بحال کیا جائے گا
       *[machine] مشین کا ماحولیاتی متغیر { $name } بحال کیا جائے گا
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-left-variable =
    { $scope ->
        [user] صارف کا ماحولیاتی متغیر { $name } جیسا تھا ویسا چھوڑ دیا گیا: اس کا کوئی حصہ اس ایپ کا نہیں ہے
       *[machine] مشین کا ماحولیاتی متغیر { $name } جیسا تھا ویسا چھوڑ دیا گیا: اس کا کوئی حصہ اس ایپ کا نہیں ہے
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-restored-variable =
    { $scope ->
        [user] صارف کا ماحولیاتی متغیر { $name } بحال کیا گیا
       *[machine] مشین کا ماحولیاتی متغیر { $name } بحال کیا گیا
    }
core-restore-set-uninstaller = { $key } کے لیے ان انسٹالر سیٹ کیا گیا
core-restore-placed-driver-image = ڈرائیور امیج { $source } کو { $destination } میں رکھا گیا
# { $name } is a driver's name.
core-restore-configured-driver = ڈرائیور { $name } کنفیگر کیا گیا
# { $name } is a driver's name.
core-restore-created-driver = ڈرائیور { $name } بنایا گیا
# { $name } is a driver's name.
core-restore-started-driver = ڈرائیور { $name } شروع کیا گیا
core-restore-ensured-registry-key = رجسٹری کلید { $key } کی موجودگی یقینی بنائی گئی
# { $value } is the name of a value in the key.
core-restore-restored-registry-value = رجسٹری قدر { $key } { $value } بحال کی گئی
# "(Default)" is how Windows names the value a key has with no name: write it as Registry Editor does.
core-restore-restored-registry-default = رجسٹری قدر { $key } (ڈیفالٹ) بحال کی گئی
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-configured-service = سروس { $name } کنفیگر کی گئی ({ $account } کے طور پر چلتی ہے)
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-created-service = سروس { $name } بنائی گئی ({ $account } کے طور پر چلتی ہے)
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-restored-task = شیڈول کردہ ٹاسک { $name } بحال کیا گیا
core-restore-restored-shortcut = شارٹ کٹ { $path } بحال کیا گیا

## Warnings an install gives, and why it failed. They are shown after it, on the PC it ran on.

core-warning-restore-earlier-not-trusted = ایک ایسی تنصیب کا ریکارڈ، جو کبھی مکمل نہیں ہوئی، جوں کا توں چھوڑ دیا گیا، کیونکہ دوسرے اسے تبدیل کر سکتے ہیں۔ اس تنصیب نے جو کچھ بھی تبدیل کیا تھا وہ ابھی تک موجود ہے۔
# { $app_name } is the app's name. { $count } is how many changes were put back.
core-warning-restore-earlier-undone =
    { $count ->
        [one] { $app_name } کی ایک پچھلی تنصیب کبھی مکمل نہیں ہوئی، اس لیے پہلے اسے کالعدم کیا گیا۔ { $count } تبدیلی واپس رکھی گئی۔
       *[other] { $app_name } کی ایک پچھلی تنصیب کبھی مکمل نہیں ہوئی، اس لیے پہلے اسے کالعدم کیا گیا۔ { $count } تبدیلیاں واپس رکھی گئیں۔
    }
# { $app_name } is the app's name.
core-warning-restore-earlier-never-finished = { $app_name } کی ایک پچھلی تنصیب کبھی مکمل نہیں ہوئی۔ انسٹال کرنے پر پہلے اسے کالعدم کیا جاتا ہے۔
core-warning-restore-user-data-real-locations = صارف کا ڈیٹا ہدف روٹ کے بجائے اس مشین کے اصل %AppData%/%ProgramData% مقامات پر بحال کیا جاتا ہے۔
# { $error } is what went wrong.
core-warning-restore-commit-failed = اس تنصیب کا ریکارڈ مکمل نہیں کیا جا سکا: { $error }
# { $name } is a service's name.
core-warning-restore-service-still-starting = سروس { $name } کو شروع ہونے کے لیے کہا گیا، اور وہ ابھی تک پوری طرح شروع نہیں ہوئی۔ ممکن ہے ابھی ہو جائے۔ یہ اگلی بار یہ PC شروع ہونے پر بھی خود بخود شروع ہوتی ہے۔
# { $name } is a service's name. { $reason } is why it did not start, as Windows said it.
core-warning-restore-service-not-started = سروس { $name } انسٹال ہو گئی ہے لیکن شروع نہیں ہوئی: { $reason }۔ جب تک یہ شروع نہ ہو، ہو سکتا ہے ایپ کام نہ کرے۔ Windows اگلی بار یہ PC شروع ہونے پر دوبارہ کوشش کرتا ہے۔
# { $name } is a service's name. { $reason } is why it could not be stopped.
core-warning-restore-service-not-stopped = سروس { $name } کو پہلے روکا نہیں جا سکا: { $reason }۔ ہو سکتا ہے یہ اگلی بار یہ PC شروع ہونے تک پرانا پروگرام چلاتی رہے۔
core-warning-restore-install-cancelled = تنصیب منسوخ کر دی گئی۔
core-warning-restore-preview-cancelled = پیش منظر منسوخ کر دیا گیا۔
# { $item } is what was left out, such as a file or a service's name. { $reason } is why, already in words.
core-warning-restore-left-out = { $item } کو خارج کر دیا گیا: { $reason }۔
# { $error } is what went wrong.
core-warning-restore-verification-failed = بحالی کے بعد کی تصدیق نہیں چل سکی: { $error }
# { $runner } says what relies on the file: a service, a scheduled task, a firewall rule, a shortcut, or a registry value.
# { $path } is the file. { $name } is the name of what relies on it. { $refusal } is why the file was not replaced,
# already in words.
core-warning-restore-program-would-run =
    { $runner ->
        [service] { $path } کو تبدیل کرنا اس پیکیج کا کام نہیں ہے، اور سروس { $name } اس پر انحصار کرتی ہے، اس لیے کچھ انسٹال نہیں کیا گیا: { $refusal }
        [task] { $path } کو تبدیل کرنا اس پیکیج کا کام نہیں ہے، اور شیڈول کردہ ٹاسک { $name } اس پر انحصار کرتا ہے، اس لیے کچھ انسٹال نہیں کیا گیا: { $refusal }
        [firewall] { $path } کو تبدیل کرنا اس پیکیج کا کام نہیں ہے، اور فائر وال قاعدہ { $name } اس پر انحصار کرتا ہے، اس لیے کچھ انسٹال نہیں کیا گیا: { $refusal }
        [shortcut] { $path } کو تبدیل کرنا اس پیکیج کا کام نہیں ہے، اور شارٹ کٹ { $name } اس پر انحصار کرتا ہے، اس لیے کچھ انسٹال نہیں کیا گیا: { $refusal }
       *[registry] { $path } کو تبدیل کرنا اس پیکیج کا کام نہیں ہے، اور رجسٹری قدر { $name } اس پر انحصار کرتی ہے، اس لیے کچھ انسٹال نہیں کیا گیا: { $refusal }
    }
core-warning-restore-destination-exists = منزل پہلے سے موجود ہے؛ اوور رائٹ کرنے کے لیے --force استعمال کریں۔
# { $path } is a file. { $error } is what went wrong.
core-warning-restore-file-not-written = { $path } لکھا نہیں جا سکا: { $error }
core-warning-restore-not-a-folder = جہاں یہ خالی فولڈر جانا ہے وہاں فولڈر کے علاوہ کوئی اور چیز موجود ہے۔
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-not-looked-at = { $path } کو دیکھا نہیں جا سکا: { $error }
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-folder-not-made = { $path } بنایا نہیں جا سکا: { $error }
# Why part of the app was left out, after "was left out:".
core-warning-restore-registration-exists = اس PC پر یہ پہلے سے کسی اور چیز پر سیٹ موجود ہے، اور کوئی تنصیب اسے اپنے قبضے میں نہیں لیتی
core-warning-restore-registry-review = رجسٹری اقدار محفوظ شدہ منظم ڈیٹا سے بحال کی گئیں، اور ایپ کے معلوم پاتھ ہدف روٹ کے مطابق دوبارہ لکھے گئے۔
# { $error } is what went wrong.
core-warning-restore-firewall-unreadable = Windows فائر وال کے قواعد پڑھے نہیں جا سکے: { $error }
# Why a firewall rule was left out, after "was left out:".
core-warning-restore-firewall-rule-exists = اس پروگرام کے لیے اس نام کا ایک قاعدہ پہلے سے موجود ہے، جو کسی اور طرح سیٹ ہے، اور کوئی تنصیب اسے تبدیل نہیں کرتی
# { $name } is a runtime.
core-warning-restore-prerequisite-unknown = { $name }: اس رن ٹائم کا کوئی معلوم انسٹالر نہیں ہے۔
# { $name } is a runtime.
core-warning-restore-prerequisite-needs-admin = { $name } انسٹال کرنے کے لیے منتظم کے حقوق درکار ہیں؛ انسٹالر کو بطور منتظم دوبارہ چلائیں۔
# { $name } is a runtime.
core-warning-restore-prerequisite-reboot = { $name } انسٹال ہو گیا لیکن اسے مکمل کرنے کے لیے دوبارہ شروع کرنا درکار ہے۔
# { $error } is what went wrong.
core-warning-restore-skipped-not-recorded = چھوڑا گیا رن ٹائم ریکارڈ نہیں کیا جا سکا: { $error }
# { $name } is a runtime. { $reason } is why it was skipped, already in words.
core-warning-restore-prerequisite-skipped = { $name }: { $reason }
# { $name } is a runtime. { $reason } is why it could not be installed. { $url } is a web address to get it from.
core-warning-restore-prerequisite-failed = { $name } انسٹال نہیں کیا جا سکا: { $reason }۔ اسے { $url } سے حاصل کریں۔
# { $error } is what went wrong.
core-warning-restore-runtime-not-recorded = انسٹال کردہ رن ٹائم ریکارڈ نہیں کیا جا سکا: { $error }
# { $error } is what went wrong.
core-warning-restore-license-source-invalid = پیکیج میں لائسنس کا پاتھ غلط ہے: { $error }
core-warning-restore-license-needs-admin = لائسنس کا ڈیٹا کسی محفوظ مقام پر بحال کرنے کے لیے منتظم کے حقوق درکار ہیں۔
core-warning-restore-license-exists = لائسنس کی منزل پہلے سے موجود ہے؛ اوور رائٹ کرنے کے لیے --force استعمال کریں۔
# { $error } is what went wrong.
core-warning-restore-license-failed = لائسنس کا ڈیٹا بحال نہیں کیا جا سکا: { $error }
core-warning-restore-driver-needs-admin = ڈرائیور کی رجسٹریشن کے لیے منتظم کے حقوق درکار ہیں؛ انسٹالر کو بطور منتظم دوبارہ چلائیں۔
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-warning-restore-task-runs-as-system = شیڈول کردہ ٹاسک { $name } بطور SYSTEM چلتا ہے، اس PC پر مکمل حقوق کے ساتھ۔
core-warning-restore-shortcut-exists = شارٹ کٹ پہلے سے موجود ہے؛ اوور رائٹ کرنے کے لیے --force استعمال کریں۔
core-warning-restore-shortcut-all-users = ایک یا زیادہ بحال کردہ شارٹ کٹس کا ہدف عوامی ڈیسک ٹاپ یا مشترکہ اسٹارٹ مینیو کا مقام ہے۔
core-warning-restore-environment-refresh = ماحولیاتی متغیرات کی تبدیلیاں نظر آنے کے لیے نیا پروسیس، شیل یا سائن ان سیشن درکار ہو سکتا ہے۔
# { $name } is a service's name. { $account } is the account it ran as on the PC it came from.
core-warning-restore-service-account = سروس { $name } ماخذ پر { $account } کے طور پر چلتی تھی۔ اس کا پاس ورڈ انسٹالر میں نہیں ہے، اس لیے سروس LocalService کے طور پر سیٹ اپ کی گئی ہے۔ اگر یہ کام نہ کرے، تو سروسز میں اس کا اکاؤنٹ اور پاس ورڈ سیٹ کریں۔
# { $name } is a runtime.
core-warning-restore-prerequisite-pending-reboot = { $name } ابھی تک رجسٹر نہیں ہوا؛ دوبارہ شروع کرنے کے بعد اسے فعال ہو جانا چاہیے۔
# { $name } is a runtime.
core-warning-restore-prerequisite-not-verified = { $name } نے کامیابی کی اطلاع دی لیکن رجسٹر نہیں ہوا؛ ممکن ہے تنصیب ناکام ہو گئی ہو۔
# { $error } is what went wrong.
core-warning-restore-driver-source-invalid = شامل کردہ ڈرائیور کا پاتھ غلط ہے: { $error }
# { $error } is what went wrong.
core-warning-restore-driver-not-read = ڈرائیور کی موجودہ ترتیبات پڑھی نہیں جا سکیں: { $error }
core-warning-restore-driver-exists = ڈرائیور سروس پہلے سے موجود ہے؛ اسے دوبارہ کنفیگر کرنے کے لیے --force استعمال کریں۔
core-warning-restore-driver-image-exists = ڈرائیور امیج پہلے سے موجود ہے؛ اوور رائٹ کرنے کے لیے --force استعمال کریں۔
core-warning-restore-driver-hash-bundled = شامل کردہ ڈرائیور امیج اپنے ریکارڈ شدہ ہیش سے مماثل نہیں ہے، اس لیے اسے انسٹال نہیں کیا گیا۔
# { $error } is what went wrong.
core-warning-restore-driver-not-placed = ڈرائیور امیج نہیں رکھی جا سکی: { $error }
core-warning-restore-driver-hash-placed = ڈرائیور امیج رکھے جانے کے بعد تبدیل ہو گئی، اس لیے اسے رجسٹر نہیں کیا گیا۔
# { $error } is what went wrong.
core-warning-restore-key-not-created = رجسٹری کلید نہیں بنائی جا سکی: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-read = وہ قدر نہیں پڑھی جا سکی جس کی جگہ یہ لے گی: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-written = رجسٹری قدر نہیں لکھی جا سکی: { $error }
core-warning-restore-value-exists = رجسٹری قدر پہلے سے موجود ہے؛ اوور رائٹ کرنے کے لیے --force استعمال کریں۔
# { $error } is what went wrong.
core-warning-restore-environment-key-not-created = ماحولیاتی متغیرات کی رجسٹری کلید نہیں بنائی جا سکی: { $error }
core-warning-restore-variable-exists = ماحولیاتی متغیر پہلے سے موجود ہے؛ اوور رائٹ کرنے کے لیے --force استعمال کریں۔
# { $error } is what went wrong.
core-warning-restore-variable-not-written = ماحولیاتی متغیر نہیں لکھا جا سکا: { $error }
core-warning-restore-value-kind-missing = رجسٹری قدر کی قسم موجود نہیں ہے۔
# { $error } is what went wrong.
core-warning-restore-service-not-read = سروس کی موجودہ ترتیبات پڑھی نہیں جا سکیں: { $error }
core-warning-restore-service-exists = سروس پہلے سے موجود ہے؛ اسے دوبارہ کنفیگر کرنے کے لیے --force استعمال کریں۔
# { $error } is what went wrong.
core-warning-restore-task-not-queried = شیڈول کردہ ٹاسک کے بارے میں معلومات حاصل نہیں کی جا سکیں: { $error }
core-warning-restore-task-exists = شیڈول کردہ ٹاسک پہلے سے موجود ہے؛ اوور رائٹ کرنے کے لیے --force استعمال کریں۔
# { $error } is what went wrong.
core-warning-restore-task-folder-not-looked-at = ٹاسک کے فولڈر کو دیکھا نہیں جا سکا: { $error }
# { $error } is what went wrong.
core-warning-restore-shortcut-not-put-back = شارٹ کٹ نہیں بنایا جا سکا، اور جس شارٹ کٹ کی جگہ اس نے لی تھی اسے واپس نہیں رکھا جا سکا: { $error }
# { $error } is what went wrong.
core-warning-restore-command-not-started = کمانڈ شروع نہیں کی جا سکی: { $error }
# { $output } is what the command wrote, as it wrote it.
core-warning-restore-command-failed = کمانڈ ناکام ہو گئی: { $output }
# { $error } is what went wrong.
core-warning-restore-powershell-not-started = PowerShell شروع نہیں کیا جا سکا: { $error }
