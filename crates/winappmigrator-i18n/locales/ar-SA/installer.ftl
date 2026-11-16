# Words of the installer that every built installer carries, which runs on the PC an app is moved to and speaks its
# language, and of the uninstaller it leaves behind for Settings. Message names start with installer- for the
# installer's window, installer-cli- for its command line, and uninstaller- for the uninstaller.
#
# The command lines (installer-cli-, uninstaller-cli-) are always shown in English, as scripts read what they print.
# Translate them all the same. Keep their command-line flags, such as --apply and --target-root, exactly as they are.


## The installer's window: its title and header

installer-window-title = برنامج تثبيت WinAppMigrator
# The window's title in the taskbar. { $app_name } is the name of the app it installs.
installer-window-title-app = تثبيت { $app_name }
installer-tagline = يعيد تثبيت تطبيق تم التقاطه على كمبيوتر آخر
# What an app is called when its package gives it no name.
installer-unnamed-app = تطبيق

## When the installer's window cannot open. Each is a paragraph of a message box.

# { $error } is what went wrong, as Windows or the graphics driver says it.
installer-window-failed = تعذر فتح نافذة برنامج التثبيت: { $error }
installer-window-failed-without-one = لا يزال بإمكانه التثبيت بدونها: شغّل هذا الملف من موجه أوامر يعمل كمسؤول مع --apply، ومع --target-root متبوعًا بمجلد لاختيار مكان التثبيت.

## Checking the installer when it opens

installer-checking = جارٍ التحقق من برنامج التثبيت…
installer-stub-no-package = هذا قالب برنامج تثبيت من WinAppMigrator، ولا يحمل أي حزمة تطبيق. أنشئ برنامج تثبيت باستخدام WinAppMigrator، ثم شغّل ملف .exe الناتج هنا.
installer-check-stopped = توقف التحقق من برنامج التثبيت.
# { $error } is what went wrong, as Windows says it.
installer-locate-failed = تعذر تحديد موقع برنامج التثبيت: { $error }
# { $error } is what went wrong reading it.
installer-read-failed = تعذرت قراءة برنامج التثبيت: { $error }
# { $error } is why, such as that the app needs a newer Windows.
installer-cannot-be-used-here = لا يمكن استخدام حزمة برنامج التثبيت هذا على هذا الكمبيوتر: { $error }

## The app, where it goes, and whether the installer is signed

installer-captured-from = تم الالتقاط من
installer-install-location = موقع التثبيت
# The title of the folder picker.
installer-choose-location = اختيار موقع التثبيت
installer-folder-has-files = يحتوي هذا المجلد على ملفات بالفعل. يتم استبدال الملفات التي تحمل الأسماء نفسها، وتعود إذا ألغيت التثبيت.
installer-room-needed = المساحة المطلوبة
# { $shortfall } says which drive is short of room, such as "C: needs up to 824.0 MB for the app's files, and has 300.0 MB free".
installer-room-short = قد لا تتوفر مساحة كافية هنا: { $shortfall }. يمكن فك ضغط الحزمة على محرك أقراص آخر، لكن يجب أن تتسع ملفات التطبيق هنا رغم ذلك.
# { $app_name } is the name of an app whose earlier install was cut short.
installer-interrupted = لم يكتمل تثبيت سابق لـ { $app_name } مطلقًا. سيتم التراجع عنه أولاً عند النقر فوق "تثبيت".
# Beside "Installer", for whether it proves who made it.
installer-signature = برنامج التثبيت
installer-signature-checking = جارٍ التحقق من توقيعه…
installer-signature-unsigned = غير موقّع: الناشر غير معروف
# { $publisher } is the name on the installer's certificate, such as "Fabrikam, Inc.".
installer-signature-signed-by = موقّع من { $publisher }
installer-signature-unnamed = موقّع من ناشر غير مسمى
# { $reason } is Windows' own word for what is wrong, such as HashMismatch.
installer-signature-invalid = توقيع برنامج التثبيت هذا غير صالح ({ $reason }). ربما تم تغييره بعد توقيعه.

## Why the install location cannot be used. Install and Preview Changes stay off while one of these is shown.

# { $app_name } is the name of the app being installed.
installer-location-app-already-there = { $app_name } مثبت بالفعل في هذا المجلد، لذا لا يمكن وضع هذه النسخة هنا. لتثبيته هنا، ألغِ أولاً تثبيت النسخة الموجودة على هذا الكمبيوتر، من إعدادات Windows ضمن "التطبيقات"، ثم افتح برنامج التثبيت هذا مرة أخرى. أو أدخل مجلدًا جديدًا فارغًا أعلاه.
# { $other_app } is the name of another app Windows lists as installed in the folder. { $app_name } is the name of the app being installed.
installer-location-other-app-there = { $other_app } مثبت في هذا المجلد، لذا لا يمكن وضع { $app_name } هنا. أدخل مجلدًا جديدًا فارغًا أعلاه. أو، إذا لم تعد بحاجة إلى { $other_app }، فألغِ تثبيته أولاً، من إعدادات Windows ضمن "التطبيقات"، ثم افتح برنامج التثبيت هذا مرة أخرى.
installer-location-has-files = يحتوي هذا المجلد بالفعل على ملفات لم يثبّتها WinAppMigrator. قد تكون تابعة لتطبيق آخر، لذا لا يتم تثبيت أي شيء فوقها. أدخل مجلدًا جديدًا فارغًا أعلاه.
# { $reason } is why, such as that it cannot be the Windows folder.
installer-location-refused = لا يمكن استخدام هذا المجلد: { $reason }. أدخل مجلدًا آخر أعلاه.

## What the install sets up beyond its files

installer-beyond-files = ما يُعدّه هذا إلى جانب الملفات
# Under the heading above. It is part of what a person agrees to when they click Install.
installer-beyond-files-agreed = يتم تنفيذ كل ذلك بحقوق المسؤول. إذا فشل التثبيت، تتم إعادة ما غيّره إلى ما كان عليه، ويتم إعلامك بأي شيء تعذرت إعادته. يمكنك أيضًا التراجع عن التثبيت لاحقًا من "الإعدادات"، ضمن "التطبيقات المثبتة". تعود الملفات التي يستبدلها، وتبقى مكتبات وقت التشغيل التي يثبّتها.
installer-beyond-files-none = لا شيء: يقتصر على نسخ الملفات، والإعدادات الموجودة في مفاتيح السجل الخاصة بالتطبيق.
# { $problem } is why, such as that the app cannot run on this PC.
installer-cannot-go-here = لا يمكن وضع هذه الحزمة في هذا الموقع: { $problem }
installer-working-out = جارٍ تحديد ما سيُعدّه هذا في هذا الموقع…
# Said as the reason after "This package cannot go to this location:".
installer-list-stopped = توقف تحديد ما سيفعله
# A heading over a list of what else the package asked for and will not get. { $count } is how many.
installer-more-left-out =
    { $count ->
        [zero] تم استبعاد { $count } عنصر آخر
        [one] تم استبعاد عنصر واحد آخر
        [two] تم استبعاد عنصرين آخرين
        [few] تم استبعاد { $count } عناصر أخرى
        [many] تم استبعاد { $count } عنصرًا آخر
       *[other] تم استبعاد { $count } عنصر آخر
    }

## Options

installer-options = الخيارات
installer-user-data = بيانات المستخدم
installer-user-data-detail = عند تشغيله: تذهب الإعدادات و"المستندات" و"الألعاب المحفوظة" إلى المجلدات المعتادة للمستخدم الذي سجّل الدخول. عند إيقافه: يذهب كل ذلك إلى مجلد التثبيت، ولا يتم إعداد أي شيء ليعمل منه.
# { $account } is the name of the account on the other PC whose data this is.
installer-user-data-whose = يحتوي هذا على بيانات { $account }.
installer-license = الترخيص
installer-license-detail = ملفات التنشيط والترخيص، وقيم السجل التي تحتفظ بالتراخيص. قد يحتاج بعضها إلى إعادة التنشيط هنا.

## Dependencies: what else the app needs, and how each is provided

installer-dependencies = التبعيات
installer-dependencies-detail = ما يحتاج إليه هذا التطبيق، وكيف يتم توفير كل منها.
installer-dependencies-none = لم يتم الكشف عن أي تبعيات إضافية لوقت التشغيل.
# { $feature } is the name of an optional part of Windows, such as NetFx3.
installer-dependency-windows-feature = ميزة Windows: { $feature }
installer-dependency-included = مضمّنة
# { $id } is the package's name in winget, such as Microsoft.VCRedist.2015+.x64.
installer-dependency-winget = يتم تثبيتها باستخدام winget: { $id }
installer-dependency-manual = تثبيت يدوي
installer-dependency-built-in = مضمّنة في Windows
installer-dependency-ships-with-app = تأتي مع التطبيق

## Buttons, and what is said beside them

installer-install = تثبيت
installer-preview = معاينة التغييرات
installer-cancel = إلغاء الأمر
installer-close = إغلاق
installer-open-folder = فتح مجلد التثبيت
# { $error } is what went wrong, as Windows says it.
installer-open-folder-failed = تعذر فتح المجلد: { $error }
installer-location-blocks = يبقى الزران "تثبيت" و"معاينة التغييرات" معطلين حتى يصبح بالإمكان استخدام "موقع التثبيت". السبب مذكور أعلاه، أسفل الحقل.
installer-package-blocks = الزر "تثبيت" معطل لأنه لا يمكن وضع هذه الحزمة في هذا الموقع. السبب مذكور أعلاه.
installer-list-changed = تغيّر ما سيفعله هذا هنا، لذا لم يتم تثبيت أي شيء. راجع القائمة أدناه، وانقر فوق "تثبيت" مرة أخرى إذا كان هذا ما تريده.
installer-cancelled-early = تم الإلغاء. لم يتم تغيير أي شيء.

## How far an install has got, beside its progress bar

# { $step } is one of the steps below. { $done } and { $total } are how many of its items are done, and how many there are.
installer-step-counted = جارٍ { $step }: { $done } من { $total }…
# { $step } is one of the steps below, when it does not count its items.
installer-step-uncounted = جارٍ { $step }…
installer-step-getting-ready = جارٍ التجهيز…
installer-step-cancelling = جارٍ الإلغاء بعد الخطوة الحالية…
installer-step-putting-back = إعادة ما تم تغييره
installer-step-unpacking = فك الضغط
installer-step-checking-package = التحقق من الحزمة
installer-step-recovering = التراجع عن تثبيت سابق لم يكتمل
# What a preview is doing, whatever step it is on.
installer-step-previewing = تحديد ما سيتغير
installer-step-runtimes = تثبيت مكتبات وقت التشغيل
installer-step-files = نسخ الملفات
installer-step-services = إعداد الخدمات
installer-step-drivers = إعداد برامج التشغيل
installer-step-registry = كتابة إعدادات السجل
installer-step-license = استعادة بيانات الترخيص
installer-step-tasks = إعداد المهام المجدولة
installer-step-shortcuts = إنشاء الاختصارات
installer-step-environment = تعيين متغيرات البيئة
installer-step-settings-entry = إضافة التطبيق إلى "الإعدادات"
installer-step-verifying = التحقق مما تم تثبيته
installer-step-undoing = التراجع

## When an install cannot go ahead or stops

installer-carries-no-package = لا يحمل برنامج التثبيت هذا أي حزمة.
installer-stopped-unexpectedly = توقف التثبيت بشكل غير متوقع.
# { $reason } is what went wrong, as the program says it.
installer-stopped-unexpectedly-because = توقف التثبيت بشكل غير متوقع: { $reason }. إذا كان التثبيت قد بدأ، فإن تشغيل برنامج تثبيت مرة أخرى يتراجع أولاً عما فعله.
# { $error } is what went wrong reading it.
installer-package-read-failed = تعذرت قراءة الحزمة: { $error }
installer-no-programdata = تعذر العثور على مجلد ProgramData على هذا الكمبيوتر، لذا لم يتم تثبيت أي شيء: يحتفظ التثبيت هناك بالسجل الذي يُستخدم للتراجع عنه.
# { $error } is why, such as that there is nowhere only administrators can get into.
installer-staging-failed = تعذر إنشاء مجلد تجهيز: { $error }
# { $error } is what went wrong unpacking it.
installer-unpack-failed = تعذر فك ضغط الحزمة: { $error }
installer-staging-nowhere = لا يوجد مكان لفك ضغط الحزمة لا يستطيع الوصول إليه سوى المسؤولين
# { $failure } is a folder and what went wrong there, such as "C:\ProgramData\WinAppMigrator: Access is denied.".
installer-staging-nowhere-look-at = لا يوجد مكان لفك ضغط الحزمة لا يستطيع الوصول إليه سوى المسؤولين. المكان الذي يجب فحصه هو { $failure }

## What came of an install, or of a preview

installer-result = نتيجة التثبيت
installer-result-preview = معاينة: لم يتم تغيير أي شيء بعد
installer-result-installed = تم تثبيت التطبيق واجتاز التحقق من التجزئة.
installer-result-installed-without-left-out = تم تثبيت التطبيق واجتاز التحقق من التجزئة، باستثناء ما هو مدرج أدناه على أنه مستبعد.
installer-result-not-all-put-back = فشل التثبيت، وتعذرت إعادة كل ما غيّره إلى ما كان عليه. ما تبقى مدرج أدناه، وسيحاول التثبيت التالي مرة أخرى.
installer-result-cancelled-put-back = تم إلغاء التثبيت، وأُعيد ما غيّره إلى ما كان عليه.
installer-result-failed-put-back = فشل التثبيت، وأُعيد ما غيّره إلى ما كان عليه.
installer-result-not-clean = لم يكتمل التثبيت بشكل سليم. راجع المشكلات أدناه.
installer-result-changes-put-back = التغييرات التي تمت إعادتها
installer-result-changes-made = التغييرات التي تم إجراؤها
installer-result-changes-planned = التغييرات المخطط لها
installer-result-warnings = التحذيرات
installer-result-errors = الأخطاء
installer-result-package-valid = تجزئات الحزمة صالحة
installer-result-package-invalid = تجزئات الحزمة غير صالحة
installer-result-files-valid = الملفات المثبتة صالحة
installer-result-files-invalid = الملفات المثبتة غير صالحة
installer-result-reboot = تحتاج إحدى التبعيات إلى إعادة التشغيل لإكمال تثبيتها. أعد تشغيل Windows، ثم شغّل التطبيق.
# A heading over the list of what was changed. { $count } is how many.
installer-result-changes = التغييرات ({ $count })
# After the first part of a long list. { $count } is how many more there are.
installer-result-changes-more = و{ $count } غيرها.
# Over the path of the install's log file.
installer-log = ملف السجل

## The installer's command line. Shown in English: scripts read it. A { $count } is plain digits, with no separators.

installer-cli-stub = قالب برنامج تثبيت WinAppMigrator — لا يحمل أي حزمة تطبيق.
installer-cli-stub-how = أنشئ برنامج تثبيت باستخدام تطبيق WinAppMigrator، ثم شغّل ملف .exe الناتج على الجهاز الهدف.
installer-cli-usage-title = برنامج تثبيت WinAppMigrator ذاتي الاستخراج.
installer-cli-usage = شغّله بدون وسيطات لفتح برنامج التثبيت الرسومي، أو:
installer-cli-usage-preview = معاينة إعادة التثبيت: --dry-run [--target-root <DIR>]
installer-cli-usage-apply = إعادة التثبيت:        --apply [--target-root <DIR>]
installer-cli-no-package = لا يحتوي برنامج التثبيت على حزمة مضمّنة
installer-cli-no-programdata = تعذر العثور على مجلد ProgramData على هذا الكمبيوتر، لذا لم يتم تثبيت أي شيء: يحتفظ التثبيت هناك بالسجل الذي يُستخدم للتراجع عنه
# { $path } is the folder asked for, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-folder-has-files = لا يمكن استخدام { $path }: { $problem }. لا تستخدم --force إلا إذا كنت تعلم أنه خاص بهذا التطبيق
installer-cli-no-location = لا تذكر هذه الحزمة أين كان تطبيقها، لذا حدد --target-root
# { $path } is where the app was on the other PC, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-location-refused = كان تطبيق هذه الحزمة في { $path }، ولا يمكن استخدام ذلك: { $problem }. حدد --target-root
# { $summary } is how much room it needs where, such as "up to 824.0 MB on C: (356.0 GB free)".
installer-cli-room = المساحة: { $summary }
# { $note } is something to know about the install.
installer-cli-note = ملاحظة: { $note }
# { $count } is how many things it sets up beyond its files.
installer-cli-sets-up = ما يتم إعداده: { $count }
installer-cli-applied = تم تطبيق إعادة التثبيت.
installer-cli-previewed = تمت معاينة إعادة التثبيت.
# { $count } is how many changes the install logged.
installer-cli-changes-logged = التغييرات المسجلة: { $count }
# { $count } is how many warnings.
installer-cli-warnings = التحذيرات: { $count }
# { $count } is how many errors.
installer-cli-errors = الأخطاء: { $count }
# { $count } is how many things the package asked for and did not get.
installer-cli-refused = العناصر المرفوضة: { $count }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-package-valid = تجزئات الحزمة صالحة: { $valid }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-reinstall-valid = تجزئات إعادة التثبيت صالحة: { $valid }
# { $id } is the install's id, such as wam-0f3a….
installer-cli-install-id = معرّف التثبيت: { $id }
installer-cli-put-back = فشل التثبيت، لذا أُعيد ما غيّره إلى ما كان عليه.
# { $path } is the log file.
installer-cli-log = ملف السجل: { $path }
# { $path } is the log file. Printed before an error.
installer-cli-error-log = ملف السجل: { $path }
# { $error } is what went wrong.
installer-cli-error = خطأ: { $error }

## The uninstaller's window

uninstaller-window-title = إلغاء التثبيت
# The window's title, and its heading. { $app_name } is the name of the app it uninstalls.
uninstaller-window-title-app = إلغاء تثبيت { $app_name }
uninstaller-tagline = يتراجع عما فعله برنامج التثبيت الخاص به
uninstaller-reading = جارٍ قراءة سجل هذا التثبيت…
uninstaller-stopped-unexpectedly = توقف إلغاء التثبيت بشكل غير متوقع.
uninstaller-what-will-be-removed = ما ستتم إزالته
uninstaller-install-location = موقع التثبيت
uninstaller-files = الملفات
uninstaller-registry-values = قيم السجل
uninstaller-services = الخدمات
uninstaller-drivers = برامج التشغيل
uninstaller-scheduled-tasks = المهام المجدولة
uninstaller-environment = البيئة
uninstaller-plan-stays = تبقى الملفات التي أضفتها أو غيّرتها، مع المجلدات التي توجد فيها.
# { $count } is how many files the install replaced.
uninstaller-plan-coming-back =
    { $count ->
        [zero] يُعاد { $count } ملف استبدلها هذا التثبيت، أو يوضع كل منها بجانب الملف الحالي إذا كان قد تغيّر منذ ذلك الحين.
        [one] يُعاد ملف واحد استبدله هذا التثبيت، أو يوضع بجانب الملف الحالي إذا كان قد تغيّر منذ ذلك الحين.
        [two] يُعاد ملفان استبدلهما هذا التثبيت، أو يوضع كل منهما بجانب الملف الحالي إذا كان قد تغيّر منذ ذلك الحين.
        [few] يُعاد { $count } ملفات استبدلها هذا التثبيت، أو يوضع كل منها بجانب الملف الحالي إذا كان قد تغيّر منذ ذلك الحين.
        [many] يُعاد { $count } ملفًا استبدلها هذا التثبيت، أو يوضع كل منها بجانب الملف الحالي إذا كان قد تغيّر منذ ذلك الحين.
       *[other] يُعاد { $count } ملف استبدلها هذا التثبيت، أو يوضع كل منها بجانب الملف الحالي إذا كان قد تغيّر منذ ذلك الحين.
    }
# { $runtimes } is a list of runtimes, such as "Microsoft Visual C++ Redistributable, .NET Desktop Runtime 8".
uninstaller-plan-runtimes = تبقى مكتبات وقت التشغيل التي ثبّتها، لأن برامج أخرى قد تستخدمها: { $runtimes }.
uninstaller-options = الخيارات
uninstaller-remove-user-data = إزالة بيانات المستخدم وبيانات الترخيص أيضًا
# { $count } is how many items of the app's settings, data, and license there are.
uninstaller-remove-user-data-detail =
    { $count ->
        [zero] { $count } عنصر من إعداداته وبياناته وترخيصه. تبقى ما لم يتم تشغيل هذا الخيار. يعود ما استبدلته، أو يوضع بجانبها إذا كانت قد تغيّرت منذ ذلك الحين.
        [one] عنصر واحد من إعداداته وبياناته وترخيصه. يبقى ما لم يتم تشغيل هذا الخيار. يعود ما استبدله، أو يوضع بجانبه إذا كان قد تغيّر منذ ذلك الحين.
        [two] عنصران من إعداداته وبياناته وترخيصه. يبقيان ما لم يتم تشغيل هذا الخيار. يعود ما استبدلاه، أو يوضع بجانبهما إذا كانا قد تغيّرا منذ ذلك الحين.
        [few] { $count } عناصر من إعداداته وبياناته وترخيصه. تبقى ما لم يتم تشغيل هذا الخيار. يعود ما استبدلته، أو يوضع بجانبها إذا كانت قد تغيّرت منذ ذلك الحين.
        [many] { $count } عنصرًا من إعداداته وبياناته وترخيصه. تبقى ما لم يتم تشغيل هذا الخيار. يعود ما استبدلته، أو يوضع بجانبها إذا كانت قد تغيّرت منذ ذلك الحين.
       *[other] { $count } عنصر من إعداداته وبياناته وترخيصه. تبقى ما لم يتم تشغيل هذا الخيار. يعود ما استبدلته، أو يوضع بجانبها إذا كانت قد تغيّرت منذ ذلك الحين.
    }
uninstaller-uninstall = إلغاء التثبيت
uninstaller-cancel = إلغاء الأمر
uninstaller-close = إغلاق
# { $done } and { $total } are how many of the install's items are undone, and how many there are.
uninstaller-step-counted = جارٍ التراجع عن التثبيت: { $done } من { $total }…
uninstaller-step = جارٍ التراجع عن التثبيت…
uninstaller-done = تم إلغاء تثبيت التطبيق.
uninstaller-done-except = تم إلغاء تثبيت التطبيق، باستثناء ما هو مدرج أدناه.
uninstaller-waits-for-drive = جزء منه موجود على محرك أقراص غير متصل. قم بتوصيله، ثم ألغِ التثبيت مرة أخرى من "الإعدادات".
uninstaller-not-all-undone = تعذر التراجع عن كل شيء. أغلق ما يستخدمه، ثم ألغِ التثبيت مرة أخرى من "الإعدادات".
# Over the path of the uninstall's log file.
uninstaller-log = ملف السجل
uninstaller-undone = تم التراجع عنه
uninstaller-left-in-place = تُرك في مكانه
uninstaller-could-not-be-undone = تعذر التراجع عنه
# After the first part of a long list. { $count } is how many more there are.
uninstaller-and-more = و{ $count } غيرها.
# Said as the reason when there is no record of the install to read.
uninstaller-no-programdata = تعذر العثور على مجلد ProgramData على هذا الكمبيوتر، لذا لا يوجد سجل لقراءته

## When the uninstaller's window cannot open. Each is a paragraph of a message box, followed by a command to type.

uninstaller-window-failed-title = برنامج إلغاء تثبيت WinAppMigrator
# { $error } is what went wrong, as Windows or the graphics driver says it.
uninstaller-window-failed = تعذر فتح نافذة برنامج إلغاء التثبيت: { $error }
uninstaller-window-failed-quiet = لا يزال بإمكانه إلغاء التثبيت بدونها، من موجه أوامر يعمل كمسؤول:
# Stands in for this program's own path in that command, if it cannot be found.
uninstaller-this-program = هذا البرنامج

## The uninstaller's command line (--quiet). Shown in English: scripts read it.

# { $app_name } is the name of the app.
uninstaller-cli-uninstalling = جارٍ إلغاء تثبيت { $app_name }.
# { $path } is the log file.
uninstaller-cli-log = ملف السجل: { $path }
# { $error } is what went wrong.
uninstaller-cli-error = خطأ: { $error }
