# Words the core writes for people as it installs an app from a package: why an install will not start, the list of
# what it did (or for a preview, what it would do) that the installer window shows, its warnings, the record it keeps of
# each install so that it can be undone, and the core's errors. Message names start with core-.
#
# The list of what an install did is short and plain, one line for each thing, as the installer window lists them:
# "copied", "restored", "would copy". Words such as --force and --apply are command-line options, and LocalService,
# SYSTEM, ProgramData, and %AppData% are as Windows names them: keep them as they are.


## Errors: what went wrong, when the core cannot go on. Each is shown on its own, or after words that say what was being
## done.

core-error-list-changed = تغيّر ما سيفعله هذا التثبيت منذ الموافقة عليه، لذا لم يتم تثبيت أي شيء
# { $path } is a file or folder, such as C:\Program Files\App\app.exe, or words that name one, such as "the installer".
# { $error } is what Windows said went wrong.
core-error-io = فشلت عملية الإدخال/الإخراج لـ { $path }: { $error }
# { $detail } is what went wrong.
core-error-registry = فشلت قراءة السجل: { $detail }
# { $path } is a program file. { $detail } is what was wrong with it.
core-error-pe = فشل تحليل PE لـ { $path }: { $detail }
# { $detail } is what was wrong, already in words.
core-error-invalid-input = إدخال غير صالح: { $detail }
# { $detail } is what went wrong.
core-error-serialization = فشل تسلسل التقرير: { $detail }
# { $detail } is what went wrong.
core-error-archive = فشلت عملية الأرشيف: { $detail }
# { $detail } is what went wrong.
core-error-network = فشلت عملية الشبكة: { $detail }
# { $detail } is what went wrong.
core-error-time-format = فشل تنسيق الوقت: { $detail }
core-error-cancelled = تم إلغاؤه
# What is said when part of the program stopped unexpectedly and gave no reason.
core-error-panic-no-reason = لم يتم ذكر أي سبب

## The record of each install, kept so that it can be undone.

# { $kind } is a number that says what type of registry value it is.
core-journal-unknown-value-type = نوع قيمة سجل غير معروف { $kind }
core-journal-not-elevated = لا يعمل بامتيازات مرتفعة
core-journal-needs-admin = يتطلب التثبيت حقوق المسؤول، للاحتفاظ بسجله حيث لا يمكن للآخرين تغييره
# { $path } is a folder.
core-journal-not-an-install-folder = هذا ليس مجلد تثبيت: { $path }
# { $path } is the file the record is kept in.
core-journal-not-written = تعذرت كتابة سجل هذا التثبيت: { $path }
# { $path } is a folder.
core-journal-path-escapes-root = المسار يخرج عن الجذر المقصود: { $path }
# { $path } is a folder.
core-journal-nothing-exists = لا يوجد أي من هذا على هذا الكمبيوتر: { $path }
core-journal-busy = هناك عملية تثبيت أو إلغاء تثبيت أخرى قيد التشغيل. انتظر حتى تنتهي، ثم حاول مرة أخرى
# Said of a file that was to get back the permissions it had.
core-journal-no-folder = ليس له مجلد
# Said of the permissions a file had, which were kept to give back to it.
core-journal-permissions-unreadable = تعذرت قراءة ما تم الاحتفاظ به
# { $folder } is the folder of one install's record.
core-journal-unreadable-empty = لا يمكن استخدام سجل التثبيت في { $folder }: إنه فارغ
# { $folder } is the folder of one install's record.
core-journal-unreadable-no-header = لا يمكن استخدام سجل التثبيت في { $folder }: لا يبدأ بما يحدد موضوعه
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-version = لا يمكن استخدام سجل التثبيت في { $folder }: كتبه إصدار مختلف من هذا البرنامج
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-install = لا يمكن استخدام سجل التثبيت في { $folder }: يذكر أنه يخص تثبيتًا مختلفًا
# { $id } is an install's id, such as wam-1a2b3c.
core-journal-not-an-install-id = هذا ليس معرّف تثبيت: { $id }

## Why an install will not start, or stopped before it changed anything.

core-restore-apply-and-dry-run = لا يمكن تحديد --apply و--dry-run معًا
# { $path } is the folder that was chosen to install in.
core-restore-location-not-full-path = يجب أن يكون موقع التثبيت مسارًا كاملاً: "{ $path }"
# { $problem } says what is wrong with the folder, already in words. { $path } is the folder.
core-restore-location-unusable = لا يمكن استخدام موقع التثبيت هذا: { $problem }: "{ $path }"
core-restore-needs-admin-plan = تتضمن خطة الاستعادة تغييرات لا يجريها إلا المسؤول في السجل أو الخدمات أو المهام المجدولة أو الاختصارات أو في أهداف محمية
# { $path } is the folder that was chosen to install in.
core-restore-needs-admin-target = يتطلب الجذر الهدف امتيازات مرتفعة: { $path }
core-restore-needs-admin-user-data = تتطلب استعادة بيانات المستخدم إلى ProgramData أو إلى مواقع محمية أخرى امتيازات مرتفعة
# { $error } is why the install stopped. { $failed } is what could not be put back, as sentences of their own.
core-restore-not-all-put-back = { $error }. تعذرت إعادة كل ما تم تنفيذه إلى ما كان عليه: { $failed }
# { $error } is what went wrong.
core-restore-nowhere-to-keep-record = لا يوجد مكان آمن للاحتفاظ بالسجل الذي يتيح التراجع عن هذا التثبيت، لذا لم يتم تثبيت أي شيء: { $error }
# { $path } is a file the app would have put there.
core-restore-already-exists = { $path } موجود بالفعل، لذا لم يتم تثبيت أي شيء. استخدم --force لاستبداله
# { $root } is a folder the app has files for. { $link } is that folder, or one it is in.
core-restore-root-under-link = لدى هذا التطبيق عناصر لاستعادتها إلى { $root }، و{ $link } ارتباط إلى مكان آخر قد يتمكن الآخرون من تغييره، لذا لم يتم تثبيت أي شيء. إذا كان ما يذهب إلى هناك بيانات مستخدم أو بيانات ترخيص، فثبّته بدون بيانات المستخدم وبيانات الترخيص. إذا كان اختصارًا في مجلد بدء التشغيل، فأعد ذلك المجلد إلى مكانه السابق
core-restore-hash-check-failed = فشلت الحزمة في التحقق من التجزئة ولم يتم تثبيت أي شيء
# { $detail } is the first thing found wrong, already in words.
core-restore-hash-check-failed-because = فشلت الحزمة في التحقق من التجزئة ولم يتم تثبيت أي شيء: { $detail }
# { $path } is a file or folder.
core-restore-refuse-records-folder = تم رفض الكتابة في المكان الذي تحتفظ فيه عمليات التثبيت بسجلاتها: { $path }
# { $path } is a file or folder.
core-restore-refuse-outside = تم رفض الكتابة خارج موقع التثبيت ومجلدات البيانات المعروفة: { $path }
# { $path } is a file.
core-restore-no-parent-folder = لا يوجد مجلد أصل للوجهة: { $path }
# { $path } is where a file was to go.
core-restore-folder-in-the-way = يوجد مجلد في المكان الذي يجب أن يذهب إليه هذا الملف: { $path }
# { $path } is a file in the package.
core-restore-file-changed = لم يعد { $path } مطابقًا لما تحتويه الحزمة، لذا لم يتم تثبيته
# { $path } is a file in the package.
core-restore-source-is-link = { $path } ارتباط، ولا يجوز اتباعه
# { $path } is a file in the package.
core-restore-source-became-link = أصبح { $path } ارتباطًا أثناء فتحه
# Why a service did not start, after "did not start:".
core-restore-service-stopped-by-itself = توقفت مرة أخرى من تلقاء نفسها
# Why part of the app was left out, after "was left out:".
core-restore-com-class-someone-elses = فئة COM هذه موجودة بالفعل على هذا الكمبيوتر، وتخص شيئًا آخر

## What an install did, one line for each thing, or for a preview, what it would do. { $source } is a file in the
## package, and { $destination } and { $path } are where it goes on this PC. { $key } is a registry key, such as
## HKLM\Software\App.

# { $package_valid } is true or false, and { $restore_valid } is true, false, or not_run: keep them as they are.
core-restore-verification-result = التحقق بعد الاستعادة package_valid={ $package_valid } restore_valid={ $restore_valid }
# { $name } is a service's name.
core-restore-started-service = تم بدء الخدمة { $name }
core-restore-would-copy = سيتم نسخ { $source } -> { $destination }
core-restore-copied = تم نسخ { $source } -> { $destination }
core-restore-would-create-empty-folder = سيتم إنشاء المجلد الفارغ { $path }
core-restore-created-empty-folder = تم إنشاء المجلد الفارغ { $path }
# { $value } is the name of a value in the key.
core-restore-would-restore-registry-value = ستتم استعادة السجل { $key } { $value }
core-restore-would-restore-registry-key = ستتم استعادة السجل { $key } (مفتاح)
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-would-add-firewall-rule = ستتم إضافة قاعدة جدار الحماية { $name } للبرنامج { $program }
# { $name } is a firewall rule's name.
core-restore-firewall-rule-already-there = قاعدة جدار الحماية { $name } موجودة بالفعل
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-added-firewall-rule = تمت إضافة قاعدة جدار الحماية { $name } للبرنامج { $program }
# { $name } is a service's name, { $command } what it runs, and { $account } the account it runs as, such as LocalService.
core-restore-would-restore-service = ستتم استعادة الخدمة { $name } -> { $command } (تعمل باسم { $account })
# { $name } is a runtime, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-restore-prerequisite-present = المتطلب الأساسي موجود بالفعل: { $name }
# { $name } is a runtime that Windows has as a Windows feature.
core-restore-would-install-prerequisite-feature = سيتم تثبيت المتطلب الأساسي { $name } (ميزة Windows)
# { $name } is a runtime whose installer is in the package.
core-restore-would-install-prerequisite-bundled = سيتم تثبيت المتطلب الأساسي { $name } (برنامج تثبيت مجمّع)
# { $name } is a runtime that would come through winget, which is a program's name.
core-restore-would-install-prerequisite-winget = سيتم تثبيت المتطلب الأساسي { $name } (winget)
# { $name } is a runtime.
core-restore-installed-prerequisite = تم تثبيت المتطلب الأساسي { $name }
# { $name } is a runtime.
core-restore-installed-prerequisite-reboot = تم تثبيت المتطلب الأساسي { $name } (إعادة التشغيل مطلوبة)
core-restore-would-restore-license = ستتم استعادة الترخيص { $source } -> { $destination }
core-restore-restored-license = تمت استعادة الترخيص { $source } -> { $destination }
core-restore-would-restore-license-registry = ستتم استعادة الترخيص في السجل { $key }
# { $name } is a driver's name.
core-restore-driver-not-restored = لم تتم استعادة برنامج التشغيل تلقائيًا: { $name }
# { $name } is a driver's name.
core-restore-would-install-driver = سيتم تثبيت برنامج التشغيل { $name } -> { $destination }
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-would-restore-task = ستتم استعادة المهمة المجدولة { $name }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-startup-shortcut = ستتم استعادة اختصار بدء التشغيل { $shortcut } -> { $target }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-shortcut = ستتم استعادة الاختصار { $shortcut } -> { $target }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-would-restore-variable =
    { $scope ->
        [user] ستتم استعادة متغير بيئة المستخدم { $name }
       *[machine] ستتم استعادة متغير بيئة الجهاز { $name }
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-left-variable =
    { $scope ->
        [user] تُرك متغير بيئة المستخدم { $name } كما هو: لا يخص أي جزء منه هذا التطبيق
       *[machine] تُرك متغير بيئة الجهاز { $name } كما هو: لا يخص أي جزء منه هذا التطبيق
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-restored-variable =
    { $scope ->
        [user] تمت استعادة متغير بيئة المستخدم { $name }
       *[machine] تمت استعادة متغير بيئة الجهاز { $name }
    }
core-restore-set-uninstaller = تم تعيين برنامج إلغاء التثبيت لـ { $key }
core-restore-placed-driver-image = تم وضع صورة برنامج التشغيل { $source } -> { $destination }
# { $name } is a driver's name.
core-restore-configured-driver = تم تكوين برنامج التشغيل { $name }
# { $name } is a driver's name.
core-restore-created-driver = تم إنشاء برنامج التشغيل { $name }
core-restore-ensured-registry-key = تم التأكد من وجود مفتاح السجل { $key }
# { $value } is the name of a value in the key.
core-restore-restored-registry-value = تمت استعادة السجل { $key } { $value }
# "(Default)" is how Windows names the value a key has with no name: write it as Registry Editor does.
core-restore-restored-registry-default = تمت استعادة السجل { $key } (افتراضي)
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-configured-service = تم تكوين الخدمة { $name } (تعمل باسم { $account })
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-created-service = تم إنشاء الخدمة { $name } (تعمل باسم { $account })
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-restored-task = تمت استعادة المهمة المجدولة { $name }
core-restore-restored-shortcut = تمت استعادة الاختصار { $path }

## Warnings an install gives, and why it failed. They are shown after it, on the PC it ran on.

core-warning-restore-earlier-not-trusted = تم ترك سجل لتثبيت لم يكتمل دون تغيير، لأن الآخرين يمكنهم تغييره. كل ما غيّره ذلك التثبيت لا يزال موجودًا.
# { $app_name } is the app's name. { $count } is how many changes were put back.
core-warning-restore-earlier-undone =
    { $count ->
        [zero] لم يكتمل تثبيت سابق لـ { $app_name }، لذا تم التراجع عنه أولاً. تم التراجع عن { $count } تغيير.
        [one] لم يكتمل تثبيت سابق لـ { $app_name }، لذا تم التراجع عنه أولاً. تم التراجع عن تغيير واحد.
        [two] لم يكتمل تثبيت سابق لـ { $app_name }، لذا تم التراجع عنه أولاً. تم التراجع عن تغييرين.
        [few] لم يكتمل تثبيت سابق لـ { $app_name }، لذا تم التراجع عنه أولاً. تم التراجع عن { $count } تغييرات.
        [many] لم يكتمل تثبيت سابق لـ { $app_name }، لذا تم التراجع عنه أولاً. تم التراجع عن { $count } تغييرًا.
       *[other] لم يكتمل تثبيت سابق لـ { $app_name }، لذا تم التراجع عنه أولاً. تم التراجع عن { $count } تغيير.
    }
# { $app_name } is the app's name.
core-warning-restore-earlier-never-finished = لم يكتمل تثبيت سابق لـ { $app_name }. سيتراجع التثبيت عنه أولاً.
core-warning-restore-user-data-real-locations = تتم استعادة بيانات المستخدم إلى مواقع %AppData%/%ProgramData% الفعلية على هذا الجهاز بدلاً من الجذر الهدف.
# { $error } is what went wrong.
core-warning-restore-commit-failed = تعذر إكمال سجل هذا التثبيت: { $error }
# { $name } is a service's name.
core-warning-restore-service-still-starting = طُلب بدء الخدمة { $name } ولم تنتهِ من البدء بعد. قد تبدأ لاحقًا. كما أنها تبدأ من تلقاء نفسها عند بدء تشغيل هذا الكمبيوتر في المرة التالية.
# { $name } is a service's name. { $reason } is why it did not start, as Windows said it.
core-warning-restore-service-not-started = الخدمة { $name } مثبتة ولكنها لم تبدأ: { $reason }. قد لا يعمل التطبيق حتى تبدأ. سيحاول Windows مرة أخرى عند بدء تشغيل هذا الكمبيوتر في المرة التالية.
# { $name } is a service's name. { $reason } is why it could not be stopped.
core-warning-restore-service-not-stopped = تعذر إيقاف الخدمة { $name } أولاً: { $reason }. قد تستمر في تشغيل البرنامج القديم حتى بدء تشغيل هذا الكمبيوتر في المرة التالية.
core-warning-restore-install-cancelled = تم إلغاء التثبيت.
core-warning-restore-preview-cancelled = تم إلغاء المعاينة.
# { $item } is what was left out, such as a file or a service's name. { $reason } is why, already in words.
core-warning-restore-left-out = تم استبعاد { $item }: { $reason }.
# { $error } is what went wrong.
core-warning-restore-verification-failed = تعذر تشغيل التحقق بعد الاستعادة: { $error }
# { $runner } says what relies on the file: a service, a scheduled task, a firewall rule, a shortcut, or a registry value.
# { $path } is the file. { $name } is the name of what relies on it. { $refusal } is why the file was not replaced,
# already in words.
core-warning-restore-program-would-run =
    { $runner ->
        [service] لا يحق لهذه الحزمة استبدال { $path }، والخدمة { $name } تعتمد عليه، لذا لم يتم تثبيت أي شيء: { $refusal }
        [task] لا يحق لهذه الحزمة استبدال { $path }، والمهمة المجدولة { $name } تعتمد عليه، لذا لم يتم تثبيت أي شيء: { $refusal }
        [firewall] لا يحق لهذه الحزمة استبدال { $path }، وقاعدة جدار الحماية { $name } تعتمد عليه، لذا لم يتم تثبيت أي شيء: { $refusal }
        [shortcut] لا يحق لهذه الحزمة استبدال { $path }، والاختصار { $name } يعتمد عليه، لذا لم يتم تثبيت أي شيء: { $refusal }
       *[registry] لا يحق لهذه الحزمة استبدال { $path }، وقيمة السجل { $name } تعتمد عليه، لذا لم يتم تثبيت أي شيء: { $refusal }
    }
core-warning-restore-destination-exists = الوجهة موجودة؛ استخدم --force للكتابة فوقها.
# { $path } is a file. { $error } is what went wrong.
core-warning-restore-file-not-written = تعذرت كتابة { $path }: { $error }
core-warning-restore-not-a-folder = يوجد شيء غير مجلد في المكان الذي يذهب إليه هذا المجلد الفارغ.
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-not-looked-at = تعذر فحص { $path }: { $error }
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-folder-not-made = تعذر إنشاء { $path }: { $error }
# Why part of the app was left out, after "was left out:".
core-warning-restore-registration-exists = هذا موجود بالفعل على هذا الكمبيوتر، ومعيّن إلى شيء آخر، والتثبيت لا يستولي عليه
core-warning-restore-registry-review = تمت استعادة قيم السجل من البيانات المنظمة المؤرشفة مع إعادة كتابة مسارات التطبيق المعروفة إلى الجذر الهدف.
# { $error } is what went wrong.
core-warning-restore-firewall-unreadable = تعذرت قراءة قواعد جدار حماية Windows: { $error }
# Why a firewall rule was left out, after "was left out:".
core-warning-restore-firewall-rule-exists = توجد بالفعل قاعدة بهذا الاسم لهذا البرنامج، معيّنة إلى شيء آخر، والتثبيت لا يغيّرها
# { $name } is a runtime.
core-warning-restore-prerequisite-unknown = { $name }: لا يوجد برنامج تثبيت معروف لمكتبة وقت التشغيل هذه.
# { $name } is a runtime.
core-warning-restore-prerequisite-needs-admin = يتطلب تثبيت { $name } امتيازات مرتفعة؛ أعد تشغيل برنامج التثبيت كمسؤول.
# { $name } is a runtime.
core-warning-restore-prerequisite-reboot = تم تثبيت { $name } ولكن يلزم إعادة التشغيل لإكماله.
# { $error } is what went wrong.
core-warning-restore-skipped-not-recorded = تعذر تسجيل مكتبة وقت تشغيل تم تخطيها: { $error }
# { $name } is a runtime. { $reason } is why it was skipped, already in words.
core-warning-restore-prerequisite-skipped = { $name }: { $reason }
# { $name } is a runtime. { $reason } is why it could not be installed. { $url } is a web address to get it from.
core-warning-restore-prerequisite-failed = تعذر تثبيت { $name }: { $reason }. احصل عليه من { $url }.
# { $error } is what went wrong.
core-warning-restore-runtime-not-recorded = تعذر تسجيل مكتبة وقت تشغيل مثبتة: { $error }
# { $error } is what went wrong.
core-warning-restore-license-source-invalid = مسار الترخيص المضمّن في الحزمة غير صالح: { $error }
core-warning-restore-license-needs-admin = تتطلب استعادة بيانات الترخيص إلى موقع محمي امتيازات مرتفعة.
core-warning-restore-license-exists = وجهة الترخيص موجودة؛ استخدم --force للكتابة فوقها.
# { $error } is what went wrong.
core-warning-restore-license-failed = تعذرت استعادة بيانات الترخيص: { $error }
core-warning-restore-driver-needs-admin = يتطلب تسجيل برنامج التشغيل امتيازات مرتفعة؛ أعد تشغيل برنامج التثبيت كمسؤول.
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-warning-restore-task-runs-as-system = تعمل المهمة المجدولة { $name } باسم SYSTEM، مع حقوق كاملة على هذا الكمبيوتر.
core-warning-restore-shortcut-exists = الاختصار موجود؛ استخدم --force للكتابة فوقه.
core-warning-restore-shortcut-all-users = يستهدف اختصار واحد أو أكثر من الاختصارات المستعادة سطح مكتب عامًا أو موقعًا مشتركًا في قائمة "ابدأ".
core-warning-restore-environment-refresh = قد تتطلب تغييرات متغيرات البيئة عملية جديدة أو واجهة أوامر جديدة أو جلسة تسجيل دخول جديدة حتى تظهر.
# { $name } is a service's name. { $account } is the account it ran as on the PC it came from.
core-warning-restore-service-account = كانت الخدمة { $name } تعمل باسم { $account } على الكمبيوتر المصدر. كلمة المرور الخاصة بها غير موجودة في برنامج التثبيت، لذا تم إعداد الخدمة لتعمل باسم LocalService. إذا لم تعمل، فعيّن حسابها وكلمة المرور الخاصة بها في "الخدمات".
# { $name } is a runtime.
core-warning-restore-prerequisite-pending-reboot = لم يتم تسجيل { $name } بعد؛ يُفترض أن يصبح نشطًا بعد إعادة التشغيل.
# { $name } is a runtime.
core-warning-restore-prerequisite-not-verified = أفاد { $name } بالنجاح لكنه لم يُسجَّل؛ ربما فشل التثبيت.
# { $error } is what went wrong.
core-warning-restore-driver-source-invalid = مسار برنامج التشغيل المجمّع غير صالح: { $error }
# { $error } is what went wrong.
core-warning-restore-driver-not-read = تعذرت قراءة الإعدادات الحالية لبرنامج التشغيل: { $error }
core-warning-restore-driver-exists = خدمة برنامج التشغيل موجودة؛ استخدم --force لإعادة تكوينها.
core-warning-restore-driver-image-exists = صورة برنامج التشغيل موجودة؛ استخدم --force للكتابة فوقها.
core-warning-restore-driver-hash-bundled = لا تطابق صورة برنامج التشغيل المجمّعة التجزئة المسجلة لها، لذا لم يتم تثبيتها.
# { $error } is what went wrong.
core-warning-restore-driver-not-placed = تعذر وضع صورة برنامج التشغيل: { $error }
core-warning-restore-driver-hash-placed = تغيّرت صورة برنامج التشغيل بعد وضعها، لذا لم يتم تسجيلها.
# { $error } is what went wrong.
core-warning-restore-key-not-created = تعذر إنشاء مفتاح السجل: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-read = تعذرت قراءة القيمة التي سيستبدلها هذا: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-written = تعذرت كتابة قيمة السجل: { $error }
core-warning-restore-value-exists = قيمة السجل موجودة؛ استخدم --force للكتابة فوقها.
# { $error } is what went wrong.
core-warning-restore-environment-key-not-created = تعذر إنشاء مفتاح سجل البيئة: { $error }
core-warning-restore-variable-exists = متغير البيئة موجود؛ استخدم --force للكتابة فوقه.
# { $error } is what went wrong.
core-warning-restore-variable-not-written = تعذرت كتابة متغير البيئة: { $error }
core-warning-restore-value-kind-missing = نوع قيمة السجل مفقود.
# { $error } is what went wrong.
core-warning-restore-service-not-read = تعذرت قراءة الإعدادات الحالية للخدمة: { $error }
core-warning-restore-service-exists = الخدمة موجودة؛ استخدم --force لإعادة تكوينها.
# { $error } is what went wrong.
core-warning-restore-task-not-queried = فشل الاستعلام عن المهمة المجدولة: { $error }
core-warning-restore-task-exists = المهمة المجدولة موجودة؛ استخدم --force للكتابة فوقها.
# { $error } is what went wrong.
core-warning-restore-task-folder-not-looked-at = تعذر فحص مجلد المهمة: { $error }
# { $error } is what went wrong.
core-warning-restore-shortcut-not-put-back = تعذر إنشاء الاختصار، وتعذرت إعادة الاختصار الذي استبدله: { $error }
# { $error } is what went wrong.
core-warning-restore-command-not-started = فشل بدء الأمر: { $error }
# { $output } is what the command wrote, as it wrote it.
core-warning-restore-command-failed = فشل الأمر: { $output }
# { $error } is what went wrong.
core-warning-restore-powershell-not-started = فشل بدء PowerShell: { $error }
