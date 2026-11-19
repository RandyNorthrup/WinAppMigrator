# Words the core writes for people while it builds a package from an installed app: what a package records for an
# installer to show later, errors shown in the app, the list of what goes in and what is left out, opening a disk
# image or another PC over the network, and what an installer says when the package it carries cannot be read.
# Message names start with core-.


## Scanning an app, before a package is built. Shown in the app when a scan cannot go on.

# { $path } is the app's program, such as D:\Program Files\Acme\Acme.exe, on a drive taken from another PC.
core-scan-program-not-on-the-drive = { $path } کسی لنک سے گزرتا ہے یا منسلک ڈرائیو پر نہیں ہے، اس لیے اسے پڑھا نہیں گیا۔
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-not-on-this-pc = { $path } اس PC کی کسی ڈسک پر نہیں ہے، یا اس کا نام ایسا ہے جو کچھ اور کھول دے گا، اس لیے اسے پڑھا نہیں گیا۔
# { $path } is the app's program.
core-scan-program-someone-elses = { $path } کسی دوسرے اکاؤنٹ کے فولڈرز میں ہے یا اس اکاؤنٹ کا نہیں ہے، اور اس ایپ کا اپنا ریکارڈ یہ نہیں بتاتا کہ اس کی جگہ وہی ہے، اس لیے اسے پڑھا نہیں گیا۔
# { $app_name } is the app's name, as the list of apps calls it.
core-scan-nothing-to-package = { $app_name } کا اپنا کوئی پروگرام نہیں ملا، اس لیے انسٹالر میں ڈالنے کے لیے کچھ نہیں ہے۔ ڈرائیور، رن ٹائم اور ایڈ ان اکثر ایسے ہی ہوتے ہیں: نئے PC پر ڈرائیور اس کے بنانے والے یا Windows Update سے انسٹال کریں، اور رن ٹائم یا ایڈ ان اسے استعمال کرنے والی ایپ کے ساتھ انسٹال کریں۔
# { $app_name } is the app's name, as the list of apps calls it. A web app is a website installed as an app, with
# a window and a Start menu entry of its own.
core-scan-web-app = { $app_name } ایک ویب ایپ ہے جسے کسی براؤزر نے کسی ویب سائٹ سے انسٹال کیا ہے۔ اس کا اپنا کوئی پروگرام نہیں ہے، اس لیے انسٹالر میں ڈالنے کے لیے کچھ نہیں ہے۔ نئے PC پر اسی براؤزر میں وہ ویب سائٹ کھولیں اور وہیں سے ایپ انسٹال کریں۔
# { $path } is the program the app's own record names, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-missing = { $path } اب وہاں نہیں ہے، اگرچہ اس ایپ کا ریکارڈ اسے ایپ کا پروگرام بتاتا ہے۔ ہو سکتا ہے ایپ کو کہیں منتقل کیا گیا ہو یا جزوی طور پر ہٹا دیا گیا ہو: اس PC پر ایپ کی مرمت کرنے یا اسے دوبارہ انسٹال کرنے سے یہ واپس آ جانا چاہیے۔
# { $path } is the file that was picked.
core-scan-not-a-file = اسکین کا ہدف فائل نہیں ہے: { $path }
# Names something left out of a package, in a list of what was left out and why. { $name } is the variable's name,
# such as ACME_LICENSE_KEY.
core-scan-environment-variable-item = ماحولیاتی متغیر { $name }

## Folders and parts that might also be the app's, offered in a list to tick. Each says why it is offered.

core-near-starts-with-app-name = اس کا نام اس ایپ کے نام سے شروع ہوتا ہے۔
core-near-has-app-name = اس کے نام میں اس ایپ کا نام شامل ہے۔
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-starts-with-publisher = اس کا نام ناشر کے نام، { $publisher }، سے شروع ہوتا ہے۔
# { $folder } is a folder's name, such as Program Files.
core-near-hidden-part-beside = ایپس کی فہرست سے پوشیدہ ایک حصہ، جو { $folder } میں اس ایپ کے ساتھ انسٹال ہے۔
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-hidden-part-publisher = ایپس کی فہرست سے پوشیدہ ایک حصہ، اسی ناشر، { $publisher }، کی طرف سے۔

## What a scan found, as a person chooses from it: the folders that go in, and what is left out, and why

# Why a folder cannot be left out, beside its number of files.
core-footprint-locked-app-folder = یہ ہمیشہ مرکزی پروگرام کے ساتھ جاتی ہیں
core-footprint-locked-main-program = اس میں مرکزی پروگرام ہے، جو ہمیشہ شامل ہوتا ہے
core-footprint-locked-needed = اس میں وہ چیزیں ہیں جن کے بغیر مرکزی پروگرام شروع نہیں ہو سکتا
core-footprint-locked-font = یہ اپنے فونٹ کے ساتھ جاتا ہے، جو "یہ Windows سے کیسے جڑی ہے" کے تحت ہے
# Where a folder is. "AppData" and "ProgramData" are the names of Windows folders; keep them as they are.
core-footprint-place-roaming = آپ کی پروفائل میں ترتیبات (AppData)
core-footprint-place-local = آپ کی پروفائل میں اس PC کے لیے ترتیبات (AppData)
core-footprint-place-program-data = PC پر سب کے لیے مشترکہ (ProgramData)
core-footprint-place-documents = آپ کی دستاویزات میں
core-footprint-place-saved-games = آپ کی محفوظ کردہ گیمز میں
core-footprint-place-public-documents = عوامی دستاویزات میں
core-footprint-place-app-folder = ایپ کے فولڈر میں
# { $folder } is a folder, such as C:\Users\Sam\AppData\Roaming.
core-footprint-place-in = { $folder } میں
# A row for the files directly in the app's folder. { $folder } is that folder's name, such as Acme.
core-footprint-files-directly-in = براہ راست { $folder } میں موجود فائلیں
# Headings over what is left out, and why each item is.
core-footprint-licenses-left-out = اس انسٹالر میں شامل نہیں، کیونکہ لائسنس اور فعال کاری کے ڈیٹا کا اختیار آف ہے
core-footprint-font-stays-behind = یہ پیچھے رہ جاتا ہے۔
core-footprint-drivers-left-out = ڈرائیورز جو Windows یا کسی وینڈر کے ہیں اور آگے منتقل نہیں کیے جا سکتے، اس لیے انہیں دوسرے PC پر دوبارہ انسٹال کریں
core-footprint-driver-reinstall = اسے اس کے وینڈر کے اپنے پیکیج سے دوبارہ انسٹال کریں۔
# Why something else is left out along with what the person left out. { $service } is a service's name, such as AcmeSync.
core-footprint-needs-left-out = اسے { $service } کی ضرورت ہے، جو خارج ہے
core-footprint-told-to-use-left-out = اسے ایسے فولڈر کی کوئی چیز استعمال کرنے کو کہا گیا ہے جو آپ نے خارج کیا
core-footprint-file-in-left-out = اس کی فائل ایسے فولڈر میں ہے جو آپ نے خارج کیا
core-footprint-program-in-left-out = جس پروگرام کے لیے یہ ہے وہ ایسے فولڈر میں ہے جو آپ نے خارج کیا
core-footprint-names-left-out = یہ ایسے فولڈر کا نام لیتا ہے جو آپ نے خارج کیا
# Why something is left out, as the installer tells whoever installs it.
core-footprint-builder-left-it-out = اس انسٹالر کو بنانے والے نے اسے خارج کیا
# Something the person left out, by kind and name, such as "Service Acme Updater". { $kind } is the kind, such as
# Service or Scheduled Task; { $title } is its name.
core-footprint-left-out-item = { $kind } { $title }

## Building a package and an installer from a scan. Shown in the app when a build cannot go on.

core-package-scan-of-another-app = یہ اسکین بنائی جا رہی ایپ کا نہیں بلکہ کسی دوسری ایپ کا ہے، اس لیے کچھ نہیں بنایا گیا
core-package-scan-brought-in-more = یہ اسکین ایسا ڈیٹا لے آیا جسے شامل کرنے کو اس بلڈ میں نہیں کہا گیا تھا، اس لیے کچھ نہیں بنایا گیا
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-package-program-gone = { $path } اب موجود نہیں ہے، اس لیے کچھ نہیں بنایا گیا۔ ایپ کا دوبارہ جائزہ لیں۔
core-package-needs-stub = خود نکالنے والے آرکائیو کے لیے انسٹالر اسٹب درکار ہے
# { $reason } is what went wrong.
core-package-staged-file-failed = اسٹیجنگ فولڈر کی ایک فائل پیکیج میں نہیں جا سکی: { $reason }
# { $shortfalls } says which drives are short of room and by how much, in sentences of its own.
core-package-not-enough-room = یہ انسٹالر بنانے کے لیے کافی جگہ نہیں ہے، اس لیے کچھ پڑھا یا لکھا نہیں گیا۔ { $shortfalls }۔ وہاں کچھ جگہ خالی کریں، یا کسی دوسری ڈرائیو پر آؤٹ پٹ فولڈر منتخب کریں۔
# { $path } is a file's path, on a drive taken from another PC.
core-package-not-read-off-the-drive = { $path } کسی لنک سے گزرتا ہے یا منسلک ڈرائیو پر نہیں ہے، اس لیے اسے پڑھا نہیں گیا
# { $path } is a file's path.
core-package-not-read-not-local = { $path } اس PC کی کسی ڈسک پر نہیں ہے، اس لیے اسے پڑھا نہیں گیا
# { $path } is a file's path.
core-package-not-read-other-account = { $path } کسی دوسرے اکاؤنٹ کے فولڈرز میں لے جاتا ہے، اس لیے اسے پڑھا نہیں گیا
# { $path } is a file's path, as the PC it came from called it.
core-package-not-on-the-drive = { $path } منسلک ڈرائیو پر نہیں ہے، اس لیے اسے پڑھا نہیں گیا
# { $path } is a file's path.
core-package-backup-copy-no-parent = پیرنٹ فولڈر کے بغیر فائل کی بیک اپ کاپی نہیں بنائی جا سکتی: { $path }
# { $path } is a file's path.
core-package-backup-copy-destination-no-parent = بیک اپ کاپی کی منزل کا کوئی پیرنٹ فولڈر نہیں ہے: { $path }
# robocopy is the name of a Windows command; keep it as it is. { $code } is how it ended, such as Some(8).
core-package-robocopy-failed = robocopy کی بیک اپ موڈ کاپی ناکام ہو گئی (ایگزٹ { $code })
# Stands for a folder where a file's name would be, as in "I/O failed for temporary package folder: …".
core-package-temporary-folder = پیکیج کا عارضی فولڈر
# Stands for the installer being written where a file's name would be, as in "I/O failed for the installer: …".
core-package-the-installer = انسٹالر
# { $path } is a file's path, or its name in the package. { $error } is what went wrong with it.
core-package-path-error = { $path }: { $error }
# { $name } is a file's name in the package, such as app/bin/App.exe.
core-package-same-place = پیکیج میں ایک اور فائل کی بھی یہی جگہ ہے: { $name }
# { $name } is a file's name in the package.
core-package-changed-in-protected-folder = پیکیج بنتے وقت محفوظ فولڈر میں { $name } تبدیل ہو گیا
core-package-worker-died = فائلیں کمپریس کرنے والا ایک تھریڈ غیر متوقع طور پر رک گیا، اس لیے کوئی انسٹالر نہیں لکھا گیا
# { $path } is a file's path.
core-package-link-not-read = { $path } ایک لنک ہے، اس لیے اسے پڑھا نہیں گیا
# { $path } is a file's path. { $error } is what went wrong.
core-package-not-compressed = { $path } کو کمپریس نہیں کیا جا سکا: { $error }
# { $path } is a file's path. { $reason } is what went wrong.
core-package-not-read-back = { $path } کو دوبارہ پڑھا نہیں جا سکا: { $reason }

## Where an app's files are, and where an installer may write them. Shown when a path is refused.

# { $path } is a folder, such as C:\Program Files\Acme.
core-discovery-root-not-full-path = جس فولڈر کے تحت لکھنا ہے وہ مکمل پاتھ ہونا چاہیے: { $path }
# { $path } is a folder's path.
core-discovery-root-not-a-folder = جس فولڈر کے تحت لکھنا ہے وہ فولڈر نہیں ہے: { $path }
# { $path } is a file's path.
core-discovery-escapes-root = پاتھ مطلوبہ روٹ سے باہر جاتا ہے: { $path }
# { $path } is the link's path.
core-discovery-refusing-to-write-through-link = لنک کے ذریعے لکھنے سے انکار کیا گیا: { $path }
# { $path } is the link's path.
core-discovery-refusing-link = لنک سے گزرنے سے انکار کیا گیا: { $path }
# { $error } is what Windows said went wrong.
core-discovery-refusing-link-because = لنک سے گزرنے سے انکار کیا گیا: { $error }
# { $path } is the link's path.
core-discovery-link-to-somewhere-else = { $path } کسی دوسری جگہ کا لنک ہے، اور شاید دوسرے یہ تبدیل کر سکیں کہ یہ کہاں لے جاتا ہے۔ وہ جگہ استعمال کریں جہاں یہ لے جاتا ہے
# { $path } is a folder's path.
core-discovery-no-parent-folder = { $path } کا کوئی پیرنٹ فولڈر نہیں ہے
# { $path } is the install location as it was given. Keep the quotes, or use your language's own.
core-discovery-install-location-not-full-path = تنصیب کا مقام مکمل پاتھ ہونا چاہیے: "{ $path }"
# { $path } is the install location.
core-discovery-install-location-a-link = تنصیب کا مقام کسی دوسری جگہ کا لنک ہے: { $path }
# { $path } is a file's path.
core-discovery-device-name = پاتھ میں ایسا نام استعمال ہوا ہے جو Windows ڈیوائس کے لیے مخصوص ہے: { $path }
# { $path } is a file's path.
core-discovery-device-name-on-older-windows = { $path } کا نام ایسا ہے جسے Windows کا یہ ورژن فائل کے بجائے ڈیوائس کے طور پر کھولتا ہے، اس لیے اس میں کچھ نہیں لکھا گیا۔ اسے Windows 11 پر انسٹال کیا جا سکتا ہے
# { $path } is a file's path.
core-discovery-unsafe-path = غیر محفوظ پاتھ: { $path }
# { $path } is a file's path.
core-discovery-no-package-path = فائل کا پیکیج کے اندر کوئی نسبتی پاتھ نہیں ہے: { $path }

## Writing an installer, and reading the package inside one. Most of these are shown by an installer, on the PC it
## runs on, when it cannot read what it carries. "The package" is the app's files and settings that an installer
## carries; "the stub" is the program part of an installer, before the package; "the data file" is the .wamdata file
## that holds the package of a very large installer, beside it.

# Stands for the package where a file's name would be, as in "I/O failed for installer package: …".
core-archive-installer-package = انسٹالر پیکیج
# Stands for the installer program built into WinAppMigrator, where a file's name would be.
core-archive-embedded-stub = شامل شدہ انسٹالر اسٹب
# { $index } is the number of a file in the package. { $reason } is what went wrong.
core-archive-entry-unreadable = پیکیج کا اندراج { $index } پڑھا نہیں جا سکا: { $reason }
# { $name } is a file's name inside the package, such as app/bin/App.exe.
core-archive-entry-unsafe-path = zip اندراج کا پاتھ غیر محفوظ ہے: { $name }
# { $name } is a file's name inside the package.
core-archive-holds-more = { $name } میں اس سے زیادہ ہے جتنا یہ بتاتا ہے
# { $name } is a file's name inside the package.
core-archive-name-too-long = یہ نام zip کے لیے بہت لمبا ہے: { $name }
core-archive-changed-while-copied = اسٹب کے بعد رکھے جانے کے دوران پیکیج تبدیل ہو گیا، اس لیے کوئی انسٹالر نہیں لکھا گیا
core-archive-stub-not-a-program = اس انسٹالر کا وہ حصہ جو اس کے پیکیج سے پہلے ہے، پروگرام نہیں ہے
core-archive-damaged = یہ انسٹالر خراب ہے یا بنائے جانے کے بعد تبدیل کیا گیا ہے، اس لیے اس کے پیکیج سے کچھ نہیں پڑھا گیا۔ اسے وہاں سے دوبارہ کاپی کریں جہاں یہ بنایا گیا تھا۔
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-missing = یہ انسٹالر اپنا پیکیج اسی فولڈر میں { $name } نامی فائل میں رکھتا ہے، اور وہ فائل وہاں موجود نہیں ہے۔ دونوں فائلوں کو ایک ساتھ رہنا چاہیے: { $name } کو یہاں اس جگہ سے کاپی کریں جہاں انسٹالر بنایا گیا تھا، پھر اسے دوبارہ چلائیں۔
# { $name } is the data file's name, such as App_installer.wamdata. { $error } is what Windows said went wrong.
core-archive-data-file-unopened = اس انسٹالر کی ڈیٹا فائل، { $name }، کھولی نہیں جا سکی: { $error }۔ دونوں فائلوں کو ایک ساتھ، ایسے فولڈر میں رہنا چاہیے جسے یہ PC پڑھ سکے۔
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-a-link = { $name } ایک لنک یا فولڈر ہے، جبکہ اس انسٹالر کی ڈیٹا فائل کو اس کے بالکل ساتھ ایک الگ فائل ہونا چاہیے، اس لیے اس سے کچھ نہیں پڑھا گیا۔ اصل فائل یہاں کاپی کریں۔
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-wrong = { $name } اس انسٹالر کی ڈیٹا فائل نہیں ہے، یا یہ خراب ہے یا بنائے جانے کے بعد تبدیل کی گئی ہے، اس لیے اس سے کچھ نہیں پڑھا گیا۔ دونوں فائلیں وہاں سے دوبارہ کاپی کریں جہاں یہ بنائی گئی تھیں۔
core-archive-stub-signed = انسٹالر اسٹب دستخط شدہ ہے؛ اسے غیر دستخط شدہ رہنا چاہیے، اس لیے اس کے بجائے مکمل انسٹالر پر دستخط کریں
# { $size } is a size, such as 4.2 GB.
core-archive-too-big-to-run = انسٹالر { $size } کا ہوگا، اور Windows 4 GB یا اس سے بڑی قابل عمل فائل نہیں چلا سکتا؛ کچھ نہیں لکھا گیا
core-archive-outside-signature = انسٹالر میں اس کے دستخط سے باہر ڈیٹا موجود ہے اور ہو سکتا ہے اس میں چھیڑ چھاڑ کی گئی ہو
core-archive-footer-length = خود نکالنے والے آرکائیو کے فوٹر کی لمبائی غلط ہے
core-archive-seek-outside = پیکیج کی حد سے باہر جانے کی کوشش
# { $path } is a file's path.
core-archive-unsafe-package-path = پیکیج کا غیر محفوظ پاتھ: { $path }
# { $path } is a file's path.
core-archive-empty-package-path = پیکیج کا خالی پاتھ: { $path }

## Opening a disk image of another PC (a .vhd or .vhdx file) to read apps from it

# { $path } is the disk image file, such as D:\Backups\OldPC.vhdx.
core-image-not-attachable = { $path } ایسی ڈسک امیج نہیں ہے جسے Windows منسلک کر سکے (.vhd یا .vhdx)
core-image-needs-administrator = ڈسک امیج منسلک کرنے کے لیے منتظم کے حقوق درکار ہیں
# { $path } is the disk image file the person picked.
core-image-not-a-file = { $path } فائل نہیں ہے
# { $path } is the disk image file. "Edit Disk" is the name of a Hyper-V Manager command.
core-image-differencing = { $path } ایک ڈفرنسنگ ڈسک ہے، جس میں صرف وہ تبدیلیاں ہوتی ہیں جو اس ڈسک کے بعد ہوئیں جس سے یہ بنائی گئی تھی، اور Windows وہ ڈسک بھی کھولے گا، یہ اسے جہاں بھی بتائے۔ پہلے اسے Hyper-V کے Edit Disk کے ذریعے اس ڈسک میں ضم کریں، اور پھر وہ ڈسک منتخب کریں
# { $path } is the disk image file.
core-image-unreadable = { $path } ایسی .vhd یا .vhdx نہیں ہے جسے پڑھا جا سکے
# ProgramData is the name of a Windows folder; keep it as it is.
core-image-no-program-data = اس PC کا ProgramData فولڈر نہیں مل سکا
# The reason given in core-image-could-not-attach when Windows already has the image open.
core-image-already-attached = Windows نے اسے پہلے سے منسلک کیا ہوا ہے، اور جس نے بھی اسے منسلک کیا ہے وہ شاید اس میں لکھ رہا ہو۔ پہلے اسے علیحدہ کریں، یا اس کی ڈرائیو کو منسلک ڈرائیو کے طور پر منتخب کریں
# { $path } is the disk image file. { $reason } is why, such as core-image-already-attached or what Windows said.
core-image-could-not-attach = { $path } منسلک نہیں کی جا سکی: { $reason }
# { $path } is the disk image file.
core-image-no-windows-volume = { $path } میں ایسا کوئی والیوم نہیں جس پر Windows ہو اور جسے پڑھا جا سکے۔ BitLocker سے مرموز والیوم کو پہلے ان لاک کرنا ضروری ہے۔
# Given as the reason when Windows failed without saying anything.
core-image-windows-did-not-say = Windows نے وجہ نہیں بتائی

## Reading another PC over the network, through the client running on it. Shown as why something on that PC
## could not be read. "The client" is WinAppMigrator Client, the small program on the other PC.

core-remote-too-slow = دوسرے PC نے جواب دینے میں بہت دیر لگائی
core-remote-closed = دوسرے PC سے رابطہ بند ہے
core-remote-failed = دوسرے PC سے رابطہ ناکام ہو گیا
core-remote-answered-something-else = کلائنٹ نے کچھ اور جواب دیا
# { $name } is the PC's name or address as the person typed it, such as OLDPC or 192.168.1.20.
core-remote-not-found = { $name } نیٹ ورک پر نہیں ملا
core-remote-did-not-say-what-it-is = کلائنٹ نے یہ نہیں بتایا کہ وہ کیا ہے
core-remote-ordinary-names-only = صرف دوسرے PC کے روٹ کے نیچے، عام ناموں والے پاتھ کے بارے میں پوچھا جاتا ہے
core-remote-more-but-none = دوسرے PC نے کہا کہ ایک فولڈر میں مزید چیزیں ہیں، لیکن ان میں سے کچھ بھی نہیں دیا
# { $count } is the most things one folder may hold, such as 250,000.
core-remote-too-many-listed = دوسرے PC نے ایک فولڈر میں { $count } سے زیادہ چیزیں درج کیں
core-remote-sent-too-much = دوسرے PC نے فائل کا اس سے زیادہ حصہ بھیجا جتنا مانگا گیا تھا
core-remote-name-not-text = ایسے نام کے بارے میں نہیں پوچھا جا سکتا جو متن نہ ہو
core-remote-below-root-only = صرف دوسرے PC کے روٹ کے نیچے والے پاتھ کے بارے میں پوچھا جا سکتا ہے
core-remote-file-ended-early = دوسرے PC کی فائل وقت سے پہلے ختم ہو گئی
core-remote-link-not-followed = کسی دوسرے PC پر لنک کی پیروی نہیں کی جاتی
core-remote-only-files-opened = دوسرے PC پر صرف فائل ہی کھولی جاتی ہے
core-remote-listing-said-nothing = فہرست میں اس کے بارے میں کچھ نہیں تھا
core-remote-change-time-unknown = یہ معلوم نہیں کہ اسے کب تبدیل کیا گیا
core-remote-seek-before-start = آغاز سے پہلے کی جگہ پر جانے کی کوشش
# { $size } is how many bytes a file is; { $limit } is the most that were expected. Both are numbers.
core-remote-too-large =
    { $size ->
        [one] یہ { $size } بائٹ ہے، اور { $limit } سے زیادہ کی توقع نہیں تھی
       *[other] یہ { $size } بائٹس ہے، اور { $limit } سے زیادہ کی توقع نہیں تھی
    }

## Checking that a download is signed by Microsoft. Each is given as the reason a check failed.

# { $error } is what Windows said went wrong.
core-signature-no-powershell = PowerShell شروع نہیں کیا جا سکا: { $error }
# { $error } is what went wrong reading PowerShell's answer.
core-signature-unreadable = دستخط پڑھے نہیں جا سکے: { $error }
# { $signer } is who signed the file, such as Contoso Ltd.
core-signature-not-microsoft = اس پر Microsoft کے بجائے { $signer } کے دستخط ہیں
core-signature-not-microsoft-unnamed = اس پر Microsoft کے بجائے کسی اور کے دستخط ہیں
# { $status } is Windows' own word for the signature's state, such as HashMismatch or NotSigned, which is not translated.
core-signature-status = اس کے دستخط کی حیثیت { $status } ہے

## Warnings a package records when it is built from an app, which an installer shows later, maybe on another PC, in
## that PC's language. "This app" is the app being packaged.

core-warning-build-program-needs-unreadable = یہ پڑھا نہیں جا سکا کہ اسے Windows سے کیا درکار ہے، اس لیے یہ جانچ نہیں کی جائے گی کہ آیا یہ دوسرے PC پر چل سکتا ہے۔
core-warning-build-external-manifest = بیرونی ایپلیکیشن مینی فیسٹ ملا؛ side-by-side انحصارات کی تشریح مؤخر کر دی گئی ہے۔
core-warning-build-non-exe = منتخب فائل .exe ایکسٹینشن استعمال نہیں کرتی؛ PE اسکیننگ جاری رہے گی۔
core-warning-build-stand-in-program = کوئی عام ایپلیکیشن قابل عمل فائل نہیں ملی، اس لیے یہ فائل اس کی جگہ استعمال ہو رہی ہے۔ آرکیٹیکچر اور انحصار کے نتائج اس فائل کو بیان کرتے ہیں، اور ہو سکتا ہے ایپلیکیشن کو بیان نہ کریں۔
core-warning-build-other-apps-files-unknown = Windows Installer کے پاس دوسری ایپس کے بارے میں جو کچھ ہے وہ سب پڑھا نہیں جا سکا، اس لیے اس ایپ کے ریکارڈ میں درج کسی فولڈر میں کسی دوسری ایپ کی فائلیں ہو سکتی ہیں اور یہاں اس کا کوئی اشارہ نہیں ہوگا۔
core-warning-build-app-folder-has-another-apps-file = Windows Installer کے مطابق جس فولڈر میں اس ایپ کا پروگرام ہے اس میں کسی دوسری ایپ کی ایک فائل ہے، اور وہ بھی ساتھ آتی ہے، کیونکہ وہ پورا فولڈر لیا جاتا ہے۔
# "App Paths" is the name of a Windows registry key; keep it as it is.
core-warning-build-app-paths-folder-left-out = اس ایپ کا App Paths اندراج کہتا ہے کہ یہ ایک ایسے فولڈر میں دیکھتی ہے جو اس کا اپنا نہیں ہے، اس لیے وہ فولڈر ساتھ نہیں آتا۔ ہو سکتا ہے یہ ایسے PC پر شروع نہ ہو جہاں وہاں کی چیزیں موجود نہ ہوں۔
core-warning-build-uninstaller-folder-left-out = جس فولڈر میں اس ایپ کا ان انسٹالر ہے اسے خارج کر دیا گیا، کیونکہ Windows Installer کے مطابق وہاں کسی دوسری ایپ کی ایک فائل ہے۔
# { $why } is never shown: it chooses the reason. Translate each reason as the end of the sentence.
core-warning-build-recorded-folder-left-out =
    اس ایپ کے ریکارڈ میں موجود کوئی چیز ایک ایسے فولڈر کا نام لیتی ہے جو خارج کر دیا گیا، کیونکہ { $why ->
        [unreadable] اس کا پاتھ ڈرائیو سے باہر جاتا ہے یا کسی لنک سے گزرتا ہے، اس لیے اسے محفوظ طریقے سے پڑھا نہیں جا سکتا
        [missing] وہ موجود نہیں ہے
        [link] وہ کسی لنک سے گزرتا ہے
        [short-name] وہ اس طرح نہیں لکھا گیا جیسا اس کا اصل نام ہے
        [another-account] وہ کسی دوسرے اکاؤنٹ کے فولڈرز میں ہے
        [one-account] یہ ایپ ایک اکاؤنٹ کے لیے انسٹال کی گئی تھی، اور وہ اس اکاؤنٹ کا نہیں ہے
        [shared] اس میں مشترکہ چیزیں ہیں، یا وہ Windows کا حصہ ہے
        [generic-name] اس کے نام سے پتا نہیں چلتا کہ وہ کس کا ہے
        [publisher] اس کا نام صرف ناشر کے نام پر ہے، اور اس میں ناشر کی دوسری مصنوعات بھی ہو سکتی ہیں
        [holds-the-app] اس میں اس ایپ کا اپنا فولڈر ہے، اور اس سے زیادہ بھی ہو سکتا ہے
        [another-apps-name] وہ کسی دوسری ایپ کے نام سے ہے
        [another-apps-record] اس کا تنصیب کا ریکارڈ کسی دوسری ایپ کا ہے
        [in-another-apps-folder] وہ کسی دوسری ایپ کے فولڈر میں ہے
       *[another-apps] وہ کسی دوسری ایپ کا ہے
    }۔
core-warning-build-app-folder-holds-another-app = اس PC پر ایک اور ایپ ریکارڈ کے مطابق اس ایپ کے فولڈر کے اندر ہے، اور وہ بھی اس کے ساتھ آتی ہے، کیونکہ وہ پورا فولڈر لیا جاتا ہے۔
# "Program Files (x86)" is the name of a Windows folder; keep it as Windows calls it.
core-warning-build-from-32-bit-windows = یہ ایپ 32 بٹ Windows پر تھی۔ اس کی رجسٹری کلیدیں اس طرح ترتیب دی گئی ہیں جیسے 64 بٹ Windows کسی 32 بٹ ایپ کی کلیدیں رکھتا ہے، اور انسٹالر اس کے لیے Program Files (x86) پیش کرتا ہے۔
# { $dll } is a file's name, such as MSVCP140D.dll.
core-warning-build-debug-runtime = ایک پروگرام کو { $dll } درکار ہے، جو Visual C++ رن ٹائم کا ڈیبگ بلڈ ہے۔ Microsoft ان کا کوئی انسٹالر فراہم نہیں کرتا، اور ممکن ہے انہیں آگے منتقل کرنے کی اجازت نہ دے، اس لیے یہ Visual Studio کے بغیر کسی PC پر نہیں چلے گا۔
# { $program } is a program's file name, such as Tool.exe.
core-warning-build-single-file-program = { $program } ایک فائل میں پیک کیا گیا .NET پروگرام ہے، جو یہ نہیں بتاتا کہ اسے کون سا .NET درکار ہے، اس لیے اس کے لیے کوئی .NET شامل نہیں کیا گیا۔ اگر اسے کسی کی ضرورت ہے، تو یہ اس کے بغیر کسی PC پر شروع نہیں ہوگا۔
# { $count } is how many of the app's programs.
core-warning-build-programs-not-inspected =
    { $count ->
        [one] ایپ کے { $count } پروگرام کی یہ جانچ نہیں کی گئی کہ اسے کون سے رن ٹائمز درکار ہیں، کیونکہ ایپ کے پروگرام بہت زیادہ یا بہت بڑے ہیں۔
       *[other] ایپ کے { $count } پروگراموں کی یہ جانچ نہیں کی گئی کہ انہیں کون سے رن ٹائمز درکار ہیں، کیونکہ ایپ کے پروگرام بہت زیادہ یا بہت بڑے ہیں۔
    }
# { $runtime } is a runtime's name, such as Java Runtime Environment.
core-warning-build-runtime-no-installer = ایپ کو { $runtime } درکار ہے، اور یہاں اس کا کوئی انسٹالر نہیں ہے، اس لیے کوئی انسٹالر نہیں چلایا جاتا۔ اسے دوسرے PC پر خود انسٹال کریں۔
# { $why } is why something was left out, such as "its name says what it holds".
core-warning-build-left-out-of-package = پیکیج سے خارج: { $why }۔
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-missing-dependency = انحصار حل نہیں کیا جا سکا: { $name }
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-dependency-only-here = { $name } صرف اس PC کے سرچ پاتھ کے ذریعے ملا، اور یہ ایپ کا حصہ نہیں ہے، اس لیے یہ ساتھ نہیں آتا۔ ہو سکتا ہے ایپ اس کے بغیر کسی PC پر شروع نہ ہو۔
core-warning-build-driver-detected = ڈرائیور یا کرنل جزو ملا۔ ایپ کا اپنا ڈرائیور شامل کیا جاتا ہے اور دوبارہ رجسٹر کیا جاتا ہے؛ وینڈر یا Windows کے ساتھ آنے والا ڈرائیور آگے تقسیم نہیں کیا جاتا اور اسے اس کے دستخط شدہ وینڈر پیکیج سے دوبارہ انسٹال کرنا ضروری ہے۔
core-warning-build-near-no-longer-offered = "یہ بھی اس کا حصہ ہو سکتا ہے" والی فہرست میں نشان لگائی گئی کوئی چیز اب اس فہرست میں نہیں ہے، اس لیے اسے شامل نہیں کیا گیا۔ فہرست کو دوبارہ دیکھیں۔
# { $app_name } is the name of a part of the app, as the list of apps calls it. { $why } is never shown: it chooses the
# reason. Translate each reason as the end of the sentence.
core-warning-build-near-part-left-out =
    { $app_name } جس چیز کو اپنا کہتی ہے اس کا ایک حصہ خارج کر دیا گیا: { $why ->
        [unreadable] اس کا پاتھ ڈرائیو سے باہر جاتا ہے یا کسی لنک سے گزرتا ہے، اس لیے اسے محفوظ طریقے سے پڑھا نہیں جا سکتا
        [missing] وہ موجود نہیں ہے
        [link] وہ کسی لنک سے گزرتا ہے
        [short-name] وہ اس طرح نہیں لکھا گیا جیسا اس کا اصل نام ہے
        [another-account] وہ کسی دوسرے اکاؤنٹ کے فولڈرز میں ہے
        [one-account] یہ ایپ ایک اکاؤنٹ کے لیے انسٹال کی گئی تھی، اور وہ اس اکاؤنٹ کا نہیں ہے
        [shared] اس میں مشترکہ چیزیں ہیں، یا وہ Windows کا حصہ ہے
        [generic-name] اس کے نام سے پتا نہیں چلتا کہ وہ کس کا ہے
        [publisher] اس کا نام صرف ناشر کے نام پر ہے، اور اس میں ناشر کی دوسری مصنوعات بھی ہو سکتی ہیں
        [holds-the-app] اس میں اس ایپ کا اپنا فولڈر ہے، اور اس سے زیادہ بھی ہو سکتا ہے
        [another-apps-name] وہ کسی دوسری ایپ کے نام سے ہے
        [another-apps-record] اس کا تنصیب کا ریکارڈ کسی دوسری ایپ کا ہے
        [in-another-apps-folder] وہ کسی دوسری ایپ کے فولڈر میں ہے
       *[another-apps] وہ کسی دوسری ایپ کا ہے
    }۔
# { $app_name } is the name of a part of the app, as the list of apps calls it.
core-warning-build-near-keys-left-out = { $app_name } کے تنصیب کے ریکارڈ میں درج رجسٹری کلیدیں خارج کر دی گئیں، کیونکہ وہ ریکارڈ ایسی چیزوں کا بھی نام لیتا ہے جو کسی دوسری ایپ کی ہیں۔
# "Windows keeps what an app wrote for one account" is Windows' VirtualStore folder.
core-warning-build-virtual-store-program = ایک پروگرام اس فولڈر میں تھا جہاں Windows وہ چیزیں رکھتا ہے جو کسی ایپ نے ایک اکاؤنٹ کے لیے لکھیں۔ Windows وہاں کبھی کوئی پروگرام نہیں رکھتا، اس لیے اسے کسی اور چیز نے وہاں رکھا، اور اسے خارج کر دیا گیا۔
# { $path } is a file or folder, such as C:\Users\Sam\AppData\Roaming\Acme.
core-warning-build-file-left-out = { $path } کو خارج کر دیا گیا: یہ کسی منسلک ڈرائیو پر لنک سے گزرتا ہے، یا یہ اس اکاؤنٹ کے علاوہ کسی اور اکاؤنٹ کا ہے جس کا ڈیٹا شامل کیا جا رہا ہے۔
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as D:\Sam\Documents.
core-warning-build-user-folder-not-on-drive = { $user } کا { $folder } فولڈر { $path } پر تھا، جو اس ڈرائیو پر نہیں ہے، اس لیے وہاں موجود چیزیں شامل نہیں کی گئیں۔
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as \\server\home\Sam\Documents.
core-warning-build-user-folder-not-on-this-pc = { $user } کا { $folder } فولڈر { $path } پر تھا، جو اس PC کی کسی ڈسک پر نہیں ہے، اس لیے وہاں موجود چیزیں شامل نہیں کی گئیں۔
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-base-unreadable = { $path } پڑھا نہیں جا سکا ({ $error })؛ وہاں محفوظ ایپلیکیشن ڈیٹا شامل نہیں کیا گیا۔
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-folder-unreadable = { $path } پڑھا نہیں جا سکا ({ $error })؛ اس کے اندر موجود ایپلیکیشن ڈیٹا غائب ہو سکتا ہے۔
# { $error } is what went wrong.
core-warning-build-entry-unreadable = فائل سسٹم کا ایک اندراج پڑھا نہیں جا سکا: { $error }
# { $count } is how many links. { $names } lists some of them. { $more } is how many more there are, which are not
# named; when it is 0, the sentence ends after the list.
core-warning-build-links-not-followed =
    { $count ->
        [one] اس فولڈر میں { $count } لنک، یا کچھ اور کھول دینے والے نام، کی پیروی نہیں کی گئی، اس لیے جس چیز کی طرف وہ اشارہ کرتا ہے وہ انسٹالر میں نہیں ہے: { $names }
       *[other] اس فولڈر میں { $count } لنکس، یا کچھ اور کھول دینے والے ناموں، کی پیروی نہیں کی گئی، اس لیے جن چیزوں کی طرف وہ اشارہ کرتے ہیں وہ انسٹالر میں نہیں ہیں: { $names }
    }{ $more ->
        [0] ۔
       *[other] ، اور مزید { $more }۔
    }
# { $count } is how many folders. { $depth } is how many levels down a folder can be. { $names } lists some of them.
# { $more } is how many more there are, which are not named; when it is 0, the sentence ends after the list.
core-warning-build-folders-too-deep =
    { $count ->
        [one] { $count } فولڈر { $depth } سطحوں سے زیادہ نیچے ہے، اس لیے اس میں موجود چیزیں انسٹالر میں نہیں ہیں: { $names }
       *[other] { $count } فولڈرز { $depth } سطحوں سے زیادہ نیچے ہیں، اس لیے ان میں موجود چیزیں انسٹالر میں نہیں ہیں: { $names }
    }{ $more ->
        [0] ۔
       *[other] ، اور مزید { $more }۔
    }
# { $error } is what went wrong.
core-warning-build-metadata-unreadable = فائل کا میٹا ڈیٹا پڑھا نہیں جا سکا: { $error }
core-warning-build-reserved-name = اس فائل کا نام Windows ڈیوائس کے لیے مخصوص ہے، اس لیے اسے کاپی نہیں کیا گیا۔
# { $error } is what went wrong.
core-warning-build-hash-failed = فائل کا ہیش نہیں بنایا جا سکا: { $error }
core-warning-build-cloud-placeholder = صرف آن لائن دستیاب کلاؤڈ فائل (مثلاً OneDrive Files On-Demand) کا کوئی مقامی ڈیٹا نہیں ہے؛ بیک اپ سے پہلے اسے پن یا ڈاؤن لوڈ کریں۔ اس کی اطلاع دی گئی لیکن اسے کاپی نہیں کیا گیا۔
# { $file_name } is a driver's file name, such as acmeflt.sys.
core-warning-build-driver-file = ڈرائیور/کرنل فائل "{ $file_name }" عام فائل کے طور پر کاپی نہیں کی جاتی۔ اگر یہ اس ایپ کی کسی رجسٹرڈ ڈرائیور سروس کی بنیاد ہے، تو انسٹالر اسے شامل کرتا ہے اور ڈرائیور کو خود بخود دوبارہ رجسٹر کرتا ہے؛ ورنہ اسے وینڈر کے دستخط شدہ ڈرائیور پیکیج سے دوبارہ انسٹال کریں۔
core-warning-build-file-gone = جب ایپ کا جائزہ لیا گیا تھا تو یہ موجود تھی اور اب غائب ہے، اس لیے یہ انسٹالر میں نہیں ہے۔
# { $error } is what went wrong.
core-warning-build-file-copy-failed = فائل پیکیج میں کاپی نہیں کی جا سکی: { $error }
# { $count } is how many files. { $names } lists them.
core-warning-build-not-looked-through =
    { $count ->
        [one] اس پیکیج میں ایک فائل ایسی ہے جسے یہ دیکھنے کے لیے نہیں کھنگالا جا سکا کہ یہ پیکیج کس کے پاس سے آیا ہے: { $names }۔ ممکن ہے اس کے مندرجات سے یہ پتا چلتا ہو۔
       *[other] اس پیکیج میں کچھ فائلیں ایسی ہیں جنہیں یہ دیکھنے کے لیے نہیں کھنگالا جا سکا کہ یہ پیکیج کس کے پاس سے آیا ہے: { $names }۔ ممکن ہے ان کے مندرجات سے یہ پتا چلتا ہو۔
    }
# { $count } is the most empty folders an installer makes, such as 10,000.
core-warning-build-empty-folders-truncated = اس ایپ میں { $count } سے زیادہ خالی فولڈرز ہیں، اور صرف اتنے ہی دوبارہ بنائے جائیں گے۔
core-warning-build-credential-by-now = پیکیج سے خارج: اب یہ اسناد جیسا لگتا ہے۔
core-warning-build-no-package-path = کاپی کے قابل فائل کا پیکیج کے اندر کوئی نسبتی پاتھ نہیں تھا۔
# { $file_name } is the runtime installer's file name, such as vc_redist.x64.exe. { $error } is what went wrong.
core-warning-build-prerequisite-path-invalid = شامل کردہ پیشگی ضرورت { $file_name } نہیں رکھی جا سکی: { $error }
# { $runtime } is a runtime's name, such as Microsoft Visual C++ 2015+ Redistributable (x64). { $error } is what went
# wrong. winget is the name of a Windows command; keep it as it is.
core-warning-build-prerequisite-download-failed = { $runtime } ڈاؤن لوڈ نہیں کیا جا سکا ({ $error })؛ انسٹالر ہدف پر winget آزمائے گا۔
# { $runtime } is a runtime's name. { $reason } is why the check failed, such as "it is signed by Contoso Ltd, not
# Microsoft".
core-warning-build-prerequisite-signature-failed = { $runtime } کا ڈاؤن لوڈ اپنے دستخط کی جانچ میں ناکام رہا، اس لیے اسے شامل نہیں کیا گیا: { $reason }۔ انسٹالر ہدف پر winget آزمائے گا۔
# { $runtime } is a runtime's name. { $error } is what went wrong.
core-warning-build-prerequisite-hash-failed = { $runtime } ڈاؤن لوڈ ہو گیا لیکن اس کا ہیش نہیں بنایا جا سکا ({ $error })؛ اس کے بجائے انسٹالر ہدف پر winget آزمائے گا۔
# { $service } is a driver's service name, such as AcmeFlt. { $error } is what went wrong.
core-warning-build-driver-copy-failed = "{ $service }" کے لیے ڈرائیور امیج شامل نہیں کی جا سکی: { $error }
core-warning-build-license-not-copied = لائسنس کا ڈیٹا کاپی نہیں کیا جا سکا (موجود نہیں یا پڑھا نہیں جا سکا)؛ صرف میٹا ڈیٹا کے طور پر ریکارڈ کیا گیا۔
core-warning-build-license-file-credential = ایک لائسنس فائل اسناد جیسی لگتی ہے، اور اسے خارج کر دیا گیا۔
# { $error } is what went wrong.
core-warning-build-license-store-walk-failed = لائسنس اسٹور کے ایک حصے کی فہرست نہیں بنائی جا سکی: { $error }
core-warning-build-license-store-credential = لائسنس اسٹور کی ایک فائل اسناد جیسی لگتی ہے، اور اسے خارج کر دیا گیا۔
# { $error } is what went wrong.
core-warning-build-license-store-file-skipped = لائسنس اسٹور کی ایک فائل چھوڑ دی گئی: { $error }
