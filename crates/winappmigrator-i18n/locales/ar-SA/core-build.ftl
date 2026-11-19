# Words the core writes for people while it builds a package from an installed app: what a package records for an
# installer to show later, errors shown in the app, the list of what goes in and what is left out, opening a disk
# image or another PC over the network, and what an installer says when the package it carries cannot be read.
# Message names start with core-.


## Scanning an app, before a package is built. Shown in the app when a scan cannot go on.

# { $path } is the app's program, such as D:\Program Files\Acme\Acme.exe, on a drive taken from another PC.
core-scan-program-not-on-the-drive = لم تتم قراءة { $path } لأنه يمر عبر ارتباط أو ليس على محرك الأقراص المتصل.
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-not-on-this-pc = لم تتم قراءة { $path } لأنه ليس على قرص في هذا الكمبيوتر، أو له اسم قد يفتح شيئًا آخر.
# { $path } is the app's program.
core-scan-program-someone-elses = لم تتم قراءة { $path } لأنه موجود في مجلدات حساب آخر أو لا يخص هذا الحساب، ولا يذكر سجل هذا التطبيق نفسه أنه ينتمي إلى هناك.
# { $app_name } is the app's name, as the list of apps calls it.
core-scan-nothing-to-package = لم يُعثر على برنامج خاص بـ { $app_name }، لذا لا يوجد ما يُوضع في برنامج التثبيت. هذا شائع في برامج التشغيل ومكتبات وقت التشغيل والوظائف الإضافية: على الكمبيوتر الجديد، ثبّت برنامج التشغيل من الشركة المصنّعة أو من Windows Update، وثبّت مكتبة وقت التشغيل أو الوظيفة الإضافية مع التطبيق الذي يستخدمها.
# { $app_name } is the app's name, as the list of apps calls it. A web app is a website installed as an app, with
# a window and a Start menu entry of its own.
core-scan-web-app = { $app_name } تطبيق ويب ثبّته مستعرض من موقع ويب. ليس له برنامج خاص به، لذا لا يوجد ما يُوضع في برنامج التثبيت. على الكمبيوتر الجديد، افتح موقع الويب في المستعرض نفسه وثبّت التطبيق من هناك.
# { $path } is the program the app's own record names, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-missing = لم يعد { $path } موجودًا، مع أن سجل هذا التطبيق يذكره على أنه برنامج التطبيق. ربما نُقل التطبيق أو أُزيل جزء منه: إصلاحه أو إعادة تثبيته على هذا الكمبيوتر يُفترض أن يعيده.
# { $path } is the file that was picked.
core-scan-not-a-file = هدف الفحص ليس ملفًا: { $path }
# Names something left out of a package, in a list of what was left out and why. { $name } is the variable's name,
# such as ACME_LICENSE_KEY.
core-scan-environment-variable-item = متغير البيئة { $name }

## Folders and parts that might also be the app's, offered in a list to tick. Each says why it is offered.

core-near-starts-with-app-name = يبدأ اسمه باسم هذا التطبيق.
core-near-has-app-name = يتضمن اسمه اسم هذا التطبيق.
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-starts-with-publisher = يبدأ اسمه باسم الناشر، { $publisher }.
# { $folder } is a folder's name, such as Program Files.
core-near-hidden-part-beside = جزء مخفي من قائمة التطبيقات، مثبت بجانب هذا التطبيق في { $folder }.
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-hidden-part-publisher = جزء مخفي من قائمة التطبيقات، من الناشر نفسه، { $publisher }.

## What a scan found, as a person chooses from it: the folders that go in, and what is left out, and why

# Why a folder cannot be left out, beside its number of files.
core-footprint-locked-app-folder = تُضمّن هذه دائمًا مع البرنامج الرئيسي
core-footprint-locked-main-program = يحتوي على البرنامج الرئيسي، الذي يُضمّن دائمًا
core-footprint-locked-needed = يحتوي على ما لا يمكن للبرنامج الرئيسي البدء بدونه
core-footprint-locked-font = يرافق الخط الخاص به، ضمن ما يربطه بـ Windows
# Where a folder is. "AppData" and "ProgramData" are the names of Windows folders; keep them as they are.
core-footprint-place-roaming = إعدادات في ملف التعريف الخاص بك (AppData)
core-footprint-place-local = إعدادات لهذا الكمبيوتر في ملف التعريف الخاص بك (AppData)
core-footprint-place-program-data = مشترك بين جميع مستخدمي الكمبيوتر (ProgramData)
core-footprint-place-documents = في "المستندات"
core-footprint-place-saved-games = في "الألعاب المحفوظة"
core-footprint-place-public-documents = في "المستندات العامة"
core-footprint-place-app-folder = في مجلد التطبيق
# { $folder } is a folder, such as C:\Users\Sam\AppData\Roaming.
core-footprint-place-in = في { $folder }
# A row for the files directly in the app's folder. { $folder } is that folder's name, such as Acme.
core-footprint-files-directly-in = الملفات الموجودة مباشرةً في { $folder }
# Headings over what is left out, and why each item is.
core-footprint-licenses-left-out = غير مضمّنة في برنامج التثبيت هذا، لأن خيار "بيانات الترخيص والتنشيط" متوقف
core-footprint-font-stays-behind = يبقى في مكانه ولا يُنقل.
core-footprint-drivers-left-out = برامج تشغيل تخص Windows أو أحد المورّدين، ولا يمكن إعادة توزيعها، لذا أعد تثبيتها على الكمبيوتر الآخر
core-footprint-driver-reinstall = أعد تثبيته من الحزمة الخاصة بمورّده.
# Why something else is left out along with what the person left out. { $service } is a service's name, such as AcmeSync.
core-footprint-needs-left-out = يحتاج إلى { $service }، وقد تم استبعادها
core-footprint-told-to-use-left-out = مُعدّ لاستخدام شيء في مجلد استبعدته
core-footprint-file-in-left-out = ملفه موجود في مجلد استبعدته
core-footprint-program-in-left-out = البرنامج المخصص له موجود في مجلد استبعدته
core-footprint-names-left-out = يذكر مجلدًا استبعدته
# Why something is left out, as the installer tells whoever installs it.
core-footprint-builder-left-it-out = استبعده من أنشأ برنامج التثبيت هذا
# Something the person left out, by kind and name, such as "Service Acme Updater". { $kind } is the kind, such as
# Service or Scheduled Task; { $title } is its name.
core-footprint-left-out-item = { $kind } { $title }

## Building a package and an installer from a scan. Shown in the app when a build cannot go on.

core-package-scan-of-another-app = هذا الفحص لتطبيق غير التطبيق الذي يتم إنشاؤه، لذا لم يتم إنشاء أي شيء
core-package-scan-brought-in-more = أدخل هذا الفحص بيانات لم يُطلب من عملية الإنشاء هذه تضمينها، لذا لم يتم إنشاء أي شيء
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-package-program-gone = لم يعد { $path } موجودًا، لذا لم يتم إنشاء أي شيء. افحص التطبيق مرة أخرى.
core-package-needs-stub = يتطلب الأرشيف ذاتي الاستخراج قالب برنامج تثبيت
# { $reason } is what went wrong.
core-package-staged-file-failed = تعذر إدخال ملف موجود في مجلد التجهيز إلى الحزمة: { $reason }
# { $shortfalls } says which drives are short of room and by how much, in sentences of its own.
core-package-not-enough-room = لا توجد مساحة كافية لإنشاء برنامج التثبيت هذا، لذا لم تتم قراءة أي شيء أو كتابته. { $shortfalls }. حرر بعض المساحة هناك، أو اختر مجلد إخراج على محرك أقراص آخر.
# { $path } is a file's path, on a drive taken from another PC.
core-package-not-read-off-the-drive = لم تتم قراءة { $path } لأنه يمر عبر ارتباط أو ليس على محرك الأقراص المتصل
# { $path } is a file's path.
core-package-not-read-not-local = لم تتم قراءة { $path } لأنه ليس على قرص في هذا الكمبيوتر
# { $path } is a file's path.
core-package-not-read-other-account = لم تتم قراءة { $path } لأنه يؤدي إلى مجلدات حساب آخر
# { $path } is a file's path, as the PC it came from called it.
core-package-not-on-the-drive = لم تتم قراءة { $path } لأنه ليس على محرك الأقراص المتصل
# { $path } is a file's path.
core-package-backup-copy-no-parent = لا يمكن إجراء نسخ احتياطي لملف ليس له دليل أصل: { $path }
# { $path } is a file's path.
core-package-backup-copy-destination-no-parent = لا يوجد دليل أصل لوجهة النسخ الاحتياطي: { $path }
# robocopy is the name of a Windows command; keep it as it is. { $code } is how it ended, such as Some(8).
core-package-robocopy-failed = فشل النسخ باستخدام robocopy في وضع النسخ الاحتياطي (رمز الخروج { $code })
# Stands for a folder where a file's name would be, as in "I/O failed for temporary package folder: …".
core-package-temporary-folder = مجلد الحزمة المؤقت
# Stands for the installer being written where a file's name would be, as in "I/O failed for the installer: …".
core-package-the-installer = برنامج التثبيت
# { $path } is a file's path, or its name in the package. { $error } is what went wrong with it.
core-package-path-error = { $path }: { $error }
# { $name } is a file's name in the package, such as app/bin/App.exe.
core-package-same-place = يوجد ملف آخر له المكان نفسه في الحزمة: { $name }
# { $name } is a file's name in the package.
core-package-changed-in-protected-folder = تغيّر { $name } في المجلد المحمي أثناء إنشاء الحزمة
core-package-worker-died = توقف مؤشر ترابط يضغط الملفات بشكل غير متوقع، لذا لم تتم كتابة أي برنامج تثبيت
# { $path } is a file's path.
core-package-link-not-read = { $path } ارتباط، لذا لم تتم قراءته
# { $path } is a file's path. { $error } is what went wrong.
core-package-not-compressed = تعذر ضغط { $path }: { $error }
# { $path } is a file's path. { $reason } is what went wrong.
core-package-not-read-back = تعذرت إعادة قراءة { $path }: { $reason }

## Where an app's files are, and where an installer may write them. Shown when a path is refused.

# { $path } is a folder, such as C:\Program Files\Acme.
core-discovery-root-not-full-path = يجب أن يكون المجلد المطلوب الكتابة ضمنه مسارًا كاملاً: { $path }
# { $path } is a folder's path.
core-discovery-root-not-a-folder = المجلد المطلوب الكتابة ضمنه ليس مجلدًا: { $path }
# { $path } is a file's path.
core-discovery-escapes-root = المسار يخرج عن الجذر المقصود: { $path }
# { $path } is the link's path.
core-discovery-refusing-to-write-through-link = تم رفض الكتابة عبر ارتباط: { $path }
# { $path } is the link's path.
core-discovery-refusing-link = تم رفض المرور عبر ارتباط: { $path }
# { $error } is what Windows said went wrong.
core-discovery-refusing-link-because = تم رفض المرور عبر ارتباط: { $error }
# { $path } is the link's path.
core-discovery-link-to-somewhere-else = { $path } ارتباط إلى مكان آخر، وقد يتمكن الآخرون من تغيير المكان الذي يؤدي إليه. استخدم المكان الذي يؤدي إليه
# { $path } is a folder's path.
core-discovery-no-parent-folder = ليس لـ { $path } مجلد أصل
# { $path } is the install location as it was given. Keep the quotes, or use your language's own.
core-discovery-install-location-not-full-path = يجب أن يكون موقع التثبيت مسارًا كاملاً: "{ $path }"
# { $path } is the install location.
core-discovery-install-location-a-link = موقع التثبيت ارتباط إلى مكان آخر: { $path }
# { $path } is a file's path.
core-discovery-device-name = يستخدم المسار اسمًا محجوزًا لجهاز Windows: { $path }
# { $path } is a file's path.
core-discovery-device-name-on-older-windows = لـ { $path } اسم يفتحه هذا الإصدار من Windows كجهاز وليس كملف، لذا لم تتم كتابة أي شيء إليه. يمكن تثبيته على Windows 11
# { $path } is a file's path.
core-discovery-unsafe-path = مسار غير آمن: { $path }
# { $path } is a file's path.
core-discovery-no-package-path = ليس للملف مسار نسبي في الحزمة: { $path }

## Writing an installer, and reading the package inside one. Most of these are shown by an installer, on the PC it
## runs on, when it cannot read what it carries. "The package" is the app's files and settings that an installer
## carries; "the stub" is the program part of an installer, before the package; "the data file" is the .wamdata file
## that holds the package of a very large installer, beside it.

# Stands for the package where a file's name would be, as in "I/O failed for installer package: …".
core-archive-installer-package = حزمة برنامج التثبيت
# Stands for the installer program built into WinAppMigrator, where a file's name would be.
core-archive-embedded-stub = قالب برنامج التثبيت المضمّن
# { $index } is the number of a file in the package. { $reason } is what went wrong.
core-archive-entry-unreadable = تعذرت قراءة الإدخال { $index } في الحزمة: { $reason }
# { $name } is a file's name inside the package, such as app/bin/App.exe.
core-archive-entry-unsafe-path = إدخال zip له مسار غير آمن: { $name }
# { $name } is a file's name inside the package.
core-archive-holds-more = يحتوي { $name } على أكثر مما يذكر
# { $name } is a file's name inside the package.
core-archive-name-too-long = هذا الاسم طويل جدًا بالنسبة إلى ملف zip: { $name }
core-archive-changed-while-copied = تغيّرت الحزمة أثناء وضعها بعد القالب، لذا لم تتم كتابة أي برنامج تثبيت
core-archive-stub-not-a-program = الجزء الذي يسبق الحزمة في برنامج التثبيت هذا ليس برنامجًا
core-archive-damaged = برنامج التثبيت هذا تالف أو تم تغييره بعد إنشائه، لذا لم تتم قراءة أي شيء من حزمته. انسخه مرة أخرى من المكان الذي تم إنشاؤه فيه.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-missing = يحتفظ برنامج التثبيت هذا بحزمته في ملف باسم { $name }، في المجلد نفسه، وهذا الملف غير موجود. يجب أن يبقى الملفان معًا: انسخ { $name } إلى هنا من المكان الذي تم إنشاء برنامج التثبيت فيه، ثم شغّل هذا مرة أخرى.
# { $name } is the data file's name, such as App_installer.wamdata. { $error } is what Windows said went wrong.
core-archive-data-file-unopened = تعذر فتح ملف بيانات برنامج التثبيت هذا، { $name }: { $error }. يجب أن يبقى الملفان معًا، في مجلد يمكن لهذا الكمبيوتر قراءته.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-a-link = { $name } ارتباط أو مجلد، ويجب أن يكون ملف بيانات برنامج التثبيت هذا ملفًا مستقلاً بجانبه مباشرةً، لذا لم تتم قراءة أي شيء منه. انسخ الملف الحقيقي إلى هنا.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-wrong = { $name } ليس ملف بيانات برنامج التثبيت هذا، أو أنه تالف أو تم تغييره بعد إنشائه، لذا لم تتم قراءة أي شيء منه. انسخ الملفين مرة أخرى من المكان الذي تم إنشاؤهما فيه.
core-archive-stub-signed = قالب برنامج التثبيت موقّع؛ يجب أن يبقى غير موقّع، لذا وقّع برنامج التثبيت النهائي بدلاً منه
# { $size } is a size, such as 4.2 GB.
core-archive-too-big-to-run = سيكون حجم برنامج التثبيت { $size }، ولا يمكن لـ Windows تشغيل ملف تنفيذي حجمه 4 غيغابايت أو أكثر؛ لم تتم كتابة أي شيء
core-archive-outside-signature = يحتوي برنامج التثبيت على بيانات خارج توقيعه وربما تم العبث به
core-archive-footer-length = طول تذييل الأرشيف ذاتي الاستخراج غير صالح
core-archive-seek-outside = انتقال إلى موضع خارج الحزمة
# { $path } is a file's path.
core-archive-unsafe-package-path = مسار حزمة غير آمن: { $path }
# { $path } is a file's path.
core-archive-empty-package-path = مسار حزمة فارغ: { $path }

## Opening a disk image of another PC (a .vhd or .vhdx file) to read apps from it

# { $path } is the disk image file, such as D:\Backups\OldPC.vhdx.
core-image-not-attachable = { $path } ليس صورة قرص يمكن لـ Windows إرفاقها (.vhd أو .vhdx)
core-image-needs-administrator = يتطلب إرفاق صورة قرص حقوق المسؤول
# { $path } is the disk image file the person picked.
core-image-not-a-file = { $path } ليس ملفًا
# { $path } is the disk image file. "Edit Disk" is the name of a Hyper-V Manager command.
core-image-differencing = { $path } قرص تفاضلي، لا يحتوي إلا على ما تغيّر منذ إنشائه من القرص الأصلي، وسيفتح Windows ذلك القرص أيضًا، أينما ذُكر أنه موجود. ادمجه في ذلك القرص أولاً، باستخدام "تحرير القرص" في Hyper-V، ثم اختر ذلك القرص
# { $path } is the disk image file.
core-image-unreadable = { $path } ليس ملف .vhd أو .vhdx يمكن قراءته
# ProgramData is the name of a Windows folder; keep it as it is.
core-image-no-program-data = تعذر العثور على مجلد ProgramData على هذا الكمبيوتر
# The reason given in core-image-could-not-attach when Windows already has the image open.
core-image-already-attached = الصورة مرفقة بالفعل في Windows، وقد يكون ما أرفقها يكتب إليها. افصلها أولاً، أو اختر محرك الأقراص الخاص بها كمحرك أقراص متصل
# { $path } is the disk image file. { $reason } is why, such as core-image-already-attached or what Windows said.
core-image-could-not-attach = تعذر إرفاق { $path }: { $reason }
# { $path } is the disk image file.
core-image-no-windows-volume = لا يحتوي { $path } على وحدة تخزين عليها Windows يمكن قراءتها. يجب أولاً إلغاء تأمين وحدة التخزين المشفرة باستخدام BitLocker.
# Given as the reason when Windows failed without saying anything.
core-image-windows-did-not-say = لم يذكر Windows السبب

## Reading another PC over the network, through the client running on it. Shown as why something on that PC
## could not be read. "The client" is WinAppMigrator Client, the small program on the other PC.

core-remote-too-slow = استغرق الكمبيوتر الآخر وقتًا طويلاً جدًا للرد
core-remote-closed = الاتصال بالكمبيوتر الآخر مغلق
core-remote-failed = فشل الاتصال بالكمبيوتر الآخر
core-remote-answered-something-else = رد العميل بشيء آخر
# { $name } is the PC's name or address as the person typed it, such as OLDPC or 192.168.1.20.
core-remote-not-found = لم يتم العثور على { $name } على الشبكة
core-remote-did-not-say-what-it-is = لم يذكر العميل ماهيته
core-remote-ordinary-names-only = لا يُسأل إلا عن مسار أسفل جذر الكمبيوتر الآخر، بأسماء عادية
core-remote-more-but-none = ذكر الكمبيوتر الآخر أن مجلدًا يحتوي على المزيد، ولم يرسل أيًا منه
# { $count } is the most things one folder may hold, such as 250,000.
core-remote-too-many-listed = سرد الكمبيوتر الآخر في مجلد واحد عناصر يزيد عددها على { $count }
core-remote-sent-too-much = أرسل الكمبيوتر الآخر من الملف أكثر مما طُلب
core-remote-name-not-text = لا يمكن السؤال عن اسم ليس نصًا
core-remote-below-root-only = لا يمكن السؤال إلا عن مسار أسفل جذر الكمبيوتر الآخر
core-remote-file-ended-early = انتهى ملف الكمبيوتر الآخر قبل الأوان
core-remote-link-not-followed = لا يتم اتباع ارتباط على كمبيوتر آخر
core-remote-only-files-opened = لا يُفتح على كمبيوتر آخر سوى الملفات
core-remote-listing-said-nothing = لم تذكر القائمة شيئًا عنه
core-remote-change-time-unknown = وقت تغييره غير معروف
core-remote-seek-before-start = انتقال إلى ما قبل البداية
# { $size } is how many bytes a file is; { $limit } is the most that were expected. Both are numbers.
core-remote-too-large =
    { $size ->
        [zero] حجمه { $size } بايت، ولم يكن متوقعًا أكثر من { $limit }
        [one] حجمه { $size } بايت، ولم يكن متوقعًا أكثر من { $limit }
        [two] حجمه { $size } بايت، ولم يكن متوقعًا أكثر من { $limit }
        [few] حجمه { $size } بايت، ولم يكن متوقعًا أكثر من { $limit }
        [many] حجمه { $size } بايت، ولم يكن متوقعًا أكثر من { $limit }
       *[other] حجمه { $size } بايت، ولم يكن متوقعًا أكثر من { $limit }
    }

## Checking that a download is signed by Microsoft. Each is given as the reason a check failed.

# { $error } is what Windows said went wrong.
core-signature-no-powershell = تعذر بدء PowerShell: { $error }
# { $error } is what went wrong reading PowerShell's answer.
core-signature-unreadable = تعذرت قراءة التوقيع: { $error }
# { $signer } is who signed the file, such as Contoso Ltd.
core-signature-not-microsoft = إنه موقّع من { $signer }، وليس من Microsoft
core-signature-not-microsoft-unnamed = إنه موقّع من جهة أخرى، وليس من Microsoft
# { $status } is Windows' own word for the signature's state, such as HashMismatch or NotSigned, which is not translated.
core-signature-status = حالة توقيعه هي { $status }

## Warnings a package records when it is built from an app, which an installer shows later, maybe on another PC, in
## that PC's language. "This app" is the app being packaged.

core-warning-build-program-needs-unreadable = تعذرت قراءة ما يحتاج إليه هذا من Windows، لذا لن يتم التحقق مما إذا كان يمكن تشغيله على الكمبيوتر الآخر.
core-warning-build-external-manifest = تم الكشف عن بيان تطبيق خارجي؛ تم تأجيل تفسير تبعيات التشغيل جنبًا إلى جنب.
core-warning-build-non-exe = لا يستخدم الملف المحدد الامتداد .exe؛ سيستمر فحص PE.
core-warning-build-stand-in-program = لم يتم العثور على ملف تنفيذي عادي للتطبيق، لذا يحل هذا الملف محله. تصف نتائج البنية والتبعيات هذا الملف، وقد لا تصف التطبيق.
core-warning-build-other-apps-files-unknown = تعذرت قراءة كل ما لدى Windows Installer عن التطبيقات الأخرى، لذا قد يحتوي مجلد مسجل لهذا التطبيق على ملفات تطبيق آخر دون أن يدل شيء هنا على ذلك.
core-warning-build-app-folder-has-another-apps-file = يذكر Windows Installer أن لتطبيق آخر ملفًا في المجلد الذي يوجد فيه برنامج هذا التطبيق، وسيُضمّن هذا الملف معه، لأنه يتم أخذ هذا المجلد بالكامل.
# "App Paths" is the name of a Windows registry key; keep it as it is.
core-warning-build-app-paths-folder-left-out = يذكر إدخال App Paths لهذا التطبيق أنه يبحث في مجلد ليس خاصًا به، لذا لن يُضمّن ذلك المجلد. قد لا يبدأ التطبيق على كمبيوتر لا يحتوي على ما فيه.
core-warning-build-uninstaller-folder-left-out = تم استبعاد المجلد الذي يوجد فيه برنامج إلغاء تثبيت هذا التطبيق، لأن Windows Installer يذكر أن لتطبيق آخر ملفًا هناك.
# { $why } is never shown: it chooses the reason. Translate each reason as the end of the sentence.
core-warning-build-recorded-folder-left-out =
    يذكر شيء مسجل لهذا التطبيق مجلدًا تم استبعاده، { $why ->
        [unreadable] لأن مساره يؤدي إلى خارج محرك الأقراص أو يمر عبر ارتباط، لذا لا يمكن قراءته بأمان
        [missing] لأنه غير موجود
        [link] لأنه يمر عبر ارتباط
        [short-name] لأنه غير مكتوب بالاسم الذي يُسمى به فعلاً
        [another-account] لأنه موجود في مجلدات حساب آخر
        [one-account] لأن هذا التطبيق مثبت لحساب واحد، وهذا المجلد لا يخص ذلك الحساب
        [shared] لأنه يحتوي على عناصر مشتركة، أو هو جزء من Windows
        [generic-name] لأن اسمه لا يدل على الجهة التي يخصها
        [publisher] لأنه مسمى باسم الناشر فقط، وقد يحتوي على منتجاته الأخرى
        [holds-the-app] لأنه يحتوي على مجلد هذا التطبيق نفسه، وقد يحتوي على المزيد
        [another-apps-name] لأنه يحمل اسم تطبيق آخر
        [another-apps-record] لأن سجل تثبيته يخص تطبيقًا آخر
        [in-another-apps-folder] لأنه موجود في مجلد تطبيق آخر
       *[another-apps] لأنه يخص تطبيقًا آخر
    }.
core-warning-build-app-folder-holds-another-app = تطبيق آخر على هذا الكمبيوتر مسجل على أنه داخل مجلد هذا التطبيق، وسيُضمّن معه، لأنه يتم أخذ هذا المجلد بالكامل.
# "Program Files (x86)" is the name of a Windows folder; keep it as Windows calls it.
core-warning-build-from-32-bit-windows = كان هذا التطبيق على Windows 32 بت. مفاتيح السجل الخاصة به مرتبة كما يحتفظ Windows 64 بت بمفاتيح تطبيقات 32 بت، ويقترح برنامج التثبيت Program Files (x86) له.
# { $dll } is a file's name, such as MSVCP140D.dll.
core-warning-build-debug-runtime = يحتاج أحد البرامج إلى { $dll }، وهو إصدار تصحيح من مكتبة وقت تشغيل Visual C++. لا توفر Microsoft برنامج تثبيت لهذه الإصدارات، وقد لا تسمح بإعادة توزيعها، لذا لن يعمل البرنامج على كمبيوتر لا يحتوي على Visual Studio.
# { $program } is a program's file name, such as Tool.exe.
core-warning-build-single-file-program = { $program } برنامج .NET مجمّع في ملف واحد، لا يذكر إصدار .NET الذي يحتاج إليه، لذا لم يُخطط لأي إصدار له. إذا كان يحتاج إلى إصدار، فلن يبدأ على كمبيوتر لا يحتوي عليه.
# { $count } is how many of the app's programs.
core-warning-build-programs-not-inspected =
    { $count ->
        [zero] لم يتم فحص { $count } برنامج من برامج التطبيق بحثًا عن مكتبات وقت التشغيل التي تحتاج إليها، لأن برامجه كثيرة جدًا أو كبيرة جدًا.
        [one] لم يتم فحص برنامج واحد من برامج التطبيق بحثًا عن مكتبات وقت التشغيل التي يحتاج إليها، لأن برامجه كثيرة جدًا أو كبيرة جدًا.
        [two] لم يتم فحص برنامجين من برامج التطبيق بحثًا عن مكتبات وقت التشغيل التي يحتاجان إليها، لأن برامجه كثيرة جدًا أو كبيرة جدًا.
        [few] لم يتم فحص { $count } برامج من برامج التطبيق بحثًا عن مكتبات وقت التشغيل التي تحتاج إليها، لأن برامجه كثيرة جدًا أو كبيرة جدًا.
        [many] لم يتم فحص { $count } برنامجًا من برامج التطبيق بحثًا عن مكتبات وقت التشغيل التي تحتاج إليها، لأن برامجه كثيرة جدًا أو كبيرة جدًا.
       *[other] لم يتم فحص { $count } برنامج من برامج التطبيق بحثًا عن مكتبات وقت التشغيل التي تحتاج إليها، لأن برامجه كثيرة جدًا أو كبيرة جدًا.
    }
# { $runtime } is a runtime's name, such as Java Runtime Environment.
core-warning-build-runtime-no-installer = يحتاج التطبيق إلى { $runtime }، ولا يوجد برنامج تثبيت له هنا، لذا لن يتم تشغيل أي برنامج تثبيت. ثبّته بنفسك على الكمبيوتر الآخر.
# { $why } is why something was left out, such as "its name says what it holds".
core-warning-build-left-out-of-package = تم استبعاده من الحزمة: { $why }.
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-missing-dependency = تعذر تحديد التبعية: { $name }
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-dependency-only-here = لم يتم العثور على { $name } إلا عبر مسار البحث في هذا الكمبيوتر، وهو ليس جزءًا من التطبيق، لذا لن يُضمّن. قد لا يبدأ التطبيق على كمبيوتر لا يحتوي عليه.
core-warning-build-driver-detected = تم الكشف عن برنامج تشغيل أو مكوّن نواة. يتم تجميع برنامج التشغيل الذي يملكه التطبيق وإعادة تسجيله؛ أما برنامج التشغيل الخاص بمورّد أو المضمّن في Windows فلا تتم إعادة توزيعه ويجب إعادة تثبيته من حزمة المورّد الموقّعة.
core-warning-build-near-no-longer-offered = عنصر تم تحديده في قائمة ما قد يكون أيضًا جزءًا من هذا التطبيق لم يعد موجودًا في تلك القائمة، لذا لم تتم إضافته. راجع القائمة مرة أخرى.
# { $app_name } is the name of a part of the app, as the list of apps calls it. { $why } is never shown: it chooses the
# reason. Translate each reason as the end of the sentence.
core-warning-build-near-part-left-out =
    تم استبعاد جزء مما يذكر { $app_name } أنه خاص به: { $why ->
        [unreadable] مساره يؤدي إلى خارج محرك الأقراص أو يمر عبر ارتباط، لذا لا يمكن قراءته بأمان
        [missing] غير موجود
        [link] يمر عبر ارتباط
        [short-name] غير مكتوب بالاسم الذي يُسمى به فعلاً
        [another-account] موجود في مجلدات حساب آخر
        [one-account] هذا التطبيق مثبت لحساب واحد، وهذا الجزء لا يخص ذلك الحساب
        [shared] يحتوي على عناصر مشتركة، أو هو جزء من Windows
        [generic-name] اسمه لا يدل على الجهة التي يخصها
        [publisher] مسمى باسم الناشر فقط، وقد يحتوي على منتجاته الأخرى
        [holds-the-app] يحتوي على مجلد هذا التطبيق نفسه، وقد يحتوي على المزيد
        [another-apps-name] يحمل اسم تطبيق آخر
        [another-apps-record] سجل تثبيته يخص تطبيقًا آخر
        [in-another-apps-folder] موجود في مجلد تطبيق آخر
       *[another-apps] يخص تطبيقًا آخر
    }.
# { $app_name } is the name of a part of the app, as the list of apps calls it.
core-warning-build-near-keys-left-out = تم استبعاد مفاتيح السجل التي يذكرها سجل تثبيت { $app_name }، لأن ذلك السجل يذكر أيضًا ما يخص تطبيقًا آخر.
# "Windows keeps what an app wrote for one account" is Windows' VirtualStore folder.
core-warning-build-virtual-store-program = كان هناك برنامج في المجلد الذي يحتفظ فيه Windows بما كتبه تطبيق لحساب واحد. لا يضع Windows برامج هناك أبدًا، لذا وضعه شيء آخر، وتم استبعاده.
# { $path } is a file or folder, such as C:\Users\Sam\AppData\Roaming\Acme.
core-warning-build-file-left-out = تم استبعاد { $path }: فهو يمر عبر ارتباط على محرك أقراص متصل، أو يخص حسابًا غير الحساب الذي يتم تضمين بياناته.
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as D:\Sam\Documents.
core-warning-build-user-folder-not-on-drive = كان المجلد { $folder } الخاص بـ { $user } في { $path }، وهو ليس على محرك الأقراص هذا، لذا لم يتم تضمين ما فيه.
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as \\server\home\Sam\Documents.
core-warning-build-user-folder-not-on-this-pc = كان المجلد { $folder } الخاص بـ { $user } في { $path }، وهو ليس على قرص في هذا الكمبيوتر، لذا لم يتم تضمين ما فيه.
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-base-unreadable = تعذرت قراءة { $path } ({ $error })؛ لم يتم تضمين بيانات التطبيق المخزنة هناك.
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-folder-unreadable = تعذرت قراءة { $path } ({ $error })؛ قد تكون بيانات التطبيق المتداخلة هناك مفقودة.
# { $error } is what went wrong.
core-warning-build-entry-unreadable = تعذرت قراءة إدخال في نظام الملفات: { $error }
# { $count } is how many links. { $names } lists some of them. { $more } is how many more there are, which are not
# named; when it is 0, the sentence ends after the list.
core-warning-build-links-not-followed =
    { $count ->
        [zero] لم يتم اتباع { $count } ارتباط، أو اسم قد يفتح شيئًا آخر، في هذا المجلد، لذا فإن ما تشير إليه غير مضمّن في برنامج التثبيت: { $names }
        [one] لم يتم اتباع ارتباط واحد، أو اسم قد يفتح شيئًا آخر، في هذا المجلد، لذا فإن ما يشير إليه غير مضمّن في برنامج التثبيت: { $names }
        [two] لم يتم اتباع ارتباطين، أو اسمين قد يفتحان شيئًا آخر، في هذا المجلد، لذا فإن ما يشيران إليه غير مضمّن في برنامج التثبيت: { $names }
        [few] لم يتم اتباع { $count } ارتباطات، أو أسماء قد تفتح شيئًا آخر، في هذا المجلد، لذا فإن ما تشير إليه غير مضمّن في برنامج التثبيت: { $names }
        [many] لم يتم اتباع { $count } ارتباطًا، أو اسمًا قد يفتح شيئًا آخر، في هذا المجلد، لذا فإن ما تشير إليه غير مضمّن في برنامج التثبيت: { $names }
       *[other] لم يتم اتباع { $count } ارتباط، أو اسم قد يفتح شيئًا آخر، في هذا المجلد، لذا فإن ما تشير إليه غير مضمّن في برنامج التثبيت: { $names }
    }{ $more ->
        [0] .
        [zero] .
        [one] ، وعنصر آخر.
        [two] ، وعنصران آخران.
        [few] ، و{ $more } عناصر أخرى.
        [many] ، و{ $more } عنصرًا آخر.
       *[other] ، و{ $more } عنصر آخر.
    }
# { $count } is how many folders. { $depth } is how many levels down a folder can be. { $names } lists some of them.
# { $more } is how many more there are, which are not named; when it is 0, the sentence ends after the list.
core-warning-build-folders-too-deep =
    { $count ->
        [zero] يوجد { $count } مجلد أعمق من المستوى { $depth }، لذا فإن ما بداخلها غير مضمّن في برنامج التثبيت: { $names }
        [one] يوجد مجلد واحد أعمق من المستوى { $depth }، لذا فإن ما بداخله غير مضمّن في برنامج التثبيت: { $names }
        [two] يوجد مجلدان أعمق من المستوى { $depth }، لذا فإن ما بداخلهما غير مضمّن في برنامج التثبيت: { $names }
        [few] توجد { $count } مجلدات أعمق من المستوى { $depth }، لذا فإن ما بداخلها غير مضمّن في برنامج التثبيت: { $names }
        [many] يوجد { $count } مجلدًا أعمق من المستوى { $depth }، لذا فإن ما بداخلها غير مضمّن في برنامج التثبيت: { $names }
       *[other] يوجد { $count } مجلد أعمق من المستوى { $depth }، لذا فإن ما بداخلها غير مضمّن في برنامج التثبيت: { $names }
    }{ $more ->
        [0] .
        [zero] .
        [one] ، ومجلد آخر.
        [two] ، ومجلدان آخران.
        [few] ، و{ $more } مجلدات أخرى.
        [many] ، و{ $more } مجلدًا آخر.
       *[other] ، و{ $more } مجلد آخر.
    }
# { $error } is what went wrong.
core-warning-build-metadata-unreadable = تعذرت قراءة بيانات تعريف الملف: { $error }
core-warning-build-reserved-name = اسم هذا الملف محجوز لجهاز Windows، لذا لم يتم نسخه.
# { $error } is what went wrong.
core-warning-build-hash-failed = تعذر حساب تجزئة الملف: { $error }
core-warning-build-cloud-placeholder = ملف سحابي متوفر عبر الإنترنت فقط (مثل "الملفات عند الطلب" في OneDrive) ليس له بيانات محلية؛ اجعله متوفرًا دائمًا على هذا الجهاز أو قم بتنزيله قبل النسخ الاحتياطي. تم الإبلاغ عنه ولكن لم يتم نسخه.
# { $file_name } is a driver's file name, such as acmeflt.sys.
core-warning-build-driver-file = لا يتم نسخ ملف برنامج التشغيل/النواة "{ $file_name }" كملف عادي. إذا كان يدعم خدمة برنامج تشغيل مسجلة لهذا التطبيق، فإن برنامج التثبيت يجمّعه ويعيد تسجيل برنامج التشغيل تلقائيًا؛ وإلا فأعد تثبيته باستخدام حزمة برنامج التشغيل الموقّعة من المورّد.
core-warning-build-file-gone = كان هذا موجودًا عند فحص التطبيق ولم يعد موجودًا الآن، لذا فهو غير مضمّن في برنامج التثبيت.
# { $error } is what went wrong.
core-warning-build-file-copy-failed = تعذر نسخ الملف إلى الحزمة: { $error }
# { $count } is how many files. { $names } lists them.
core-warning-build-not-looked-through =
    { $count ->
        [zero] تحتوي هذه الحزمة على ملفات تعذر فحصها بحثًا عما يدل على مصدرها: { $names }. قد يكشف محتواها ذلك.
        [one] تحتوي هذه الحزمة على ملف تعذر فحصه بحثًا عما يدل على مصدره: { $names }. قد يكشف محتواه ذلك.
        [two] تحتوي هذه الحزمة على ملفين تعذر فحصهما بحثًا عما يدل على مصدرهما: { $names }. قد يكشف محتواهما ذلك.
        [few] تحتوي هذه الحزمة على ملفات تعذر فحصها بحثًا عما يدل على مصدرها: { $names }. قد يكشف محتواها ذلك.
        [many] تحتوي هذه الحزمة على ملفات تعذر فحصها بحثًا عما يدل على مصدرها: { $names }. قد يكشف محتواها ذلك.
       *[other] تحتوي هذه الحزمة على ملفات تعذر فحصها بحثًا عما يدل على مصدرها: { $names }. قد يكشف محتواها ذلك.
    }
# { $count } is the most empty folders an installer makes, such as 10,000.
core-warning-build-empty-folders-truncated = يحتوي هذا التطبيق على أكثر من { $count } من المجلدات الفارغة، ولن يُعاد إنشاء سوى هذا العدد منها.
core-warning-build-credential-by-now = تم استبعاده من الحزمة: أصبح يبدو الآن كبيانات اعتماد.
core-warning-build-no-package-path = لم يكن للملف القابل للنسخ مسار نسبي في الحزمة.
# { $file_name } is the runtime installer's file name, such as vc_redist.x64.exe. { $error } is what went wrong.
core-warning-build-prerequisite-path-invalid = تعذر وضع المتطلب الأساسي المجمّع { $file_name }: { $error }
# { $runtime } is a runtime's name, such as Microsoft Visual C++ 2015+ Redistributable (x64). { $error } is what went
# wrong. winget is the name of a Windows command; keep it as it is.
core-warning-build-prerequisite-download-failed = تعذر تنزيل { $runtime } ({ $error })؛ سيحاول برنامج التثبيت استخدام winget على الجهاز الهدف.
# { $runtime } is a runtime's name. { $reason } is why the check failed, such as "it is signed by Contoso Ltd, not
# Microsoft".
core-warning-build-prerequisite-signature-failed = فشل تنزيل { $runtime } في التحقق من التوقيع، لذا لم يتم تجميعه: { $reason }. سيحاول برنامج التثبيت استخدام winget على الجهاز الهدف.
# { $runtime } is a runtime's name. { $error } is what went wrong.
core-warning-build-prerequisite-hash-failed = تم تنزيل { $runtime } ولكن تعذر حساب تجزئته ({ $error })؛ سيحاول برنامج التثبيت استخدام winget على الجهاز الهدف بدلاً من ذلك.
# { $service } is a driver's service name, such as AcmeFlt. { $error } is what went wrong.
core-warning-build-driver-copy-failed = تعذر تجميع صورة برنامج التشغيل لـ "{ $service }": { $error }
core-warning-build-license-not-copied = تعذر نسخ بيانات الترخيص (مفقودة أو غير قابلة للقراءة)؛ تم تسجيلها كبيانات تعريف فقط.
core-warning-build-license-file-credential = يبدو ملف ترخيص كبيانات اعتماد، وتم استبعاده.
# { $error } is what went wrong.
core-warning-build-license-store-walk-failed = تعذر تعداد جزء من مخزن ترخيص: { $error }
core-warning-build-license-store-credential = يبدو ملف في مخزن ترخيص كبيانات اعتماد، وتم استبعاده.
# { $error } is what went wrong.
core-warning-build-license-store-file-skipped = تم تخطي ملف في مخزن ترخيص: { $error }
