# Words the core writes for people: warnings, refusals, errors, and the installer's list of what it sets up.
# Message names start with core-.


## Warnings a package records when it is built, which an installer shows later, maybe on another PC, in that PC's
## language.

# { $count } is how many shortcuts.
core-warning-shortcuts-unreadable =
    { $count ->
        [zero] لا يسمّي { $count } اختصار على محرك الأقراص هذا أي ملف، لذا تعذر تحديد ما إذا كانت تفتح هذا التطبيق.
        [one] لا يسمّي اختصار واحد على محرك الأقراص هذا أي ملف، لذا تعذر تحديد ما إذا كان يفتح هذا التطبيق.
        [two] لا يسمّي اختصاران على محرك الأقراص هذا أي ملف، لذا تعذر تحديد ما إذا كانا يفتحان هذا التطبيق.
        [few] لا يسمّي { $count } اختصارات على محرك الأقراص هذا أي ملف، لذا تعذر تحديد ما إذا كانت تفتح هذا التطبيق.
        [many] لا يسمّي { $count } اختصارًا على محرك الأقراص هذا أي ملف، لذا تعذر تحديد ما إذا كانت تفتح هذا التطبيق.
       *[other] لا يسمّي { $count } اختصار على محرك الأقراص هذا أي ملف، لذا تعذر تحديد ما إذا كانت تفتح هذا التطبيق.
    }

## Undoing an install: what an uninstall, or a failed install, says it put back, left, or could not undo. Each line is
## shown on its own, in a list.

# { $app_name } is the app's name.
core-rollback-record-damaged = تعذرت قراءة جزء من سجل { $app_name }، لذا فإن أي شيء وصفه لا يزال في مكانه، وقد لا يعود ملف استبدله.
core-rollback-settings-entry-stays = يبقى إدخال التطبيق في "الإعدادات"، حتى يمكن تشغيل هذا مرة أخرى من هناك.
# { $name } is a runtime's name, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-rollback-prerequisite-stays = تم تثبيت { $name }، أو ربما تم تثبيته، وسيبقى، لأن برامج أخرى قد تستخدمه.
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-drive-not-connected = ما وضعه هذا التثبيت في { $folder } موجود على محرك أقراص غير متصل. قم بتوصيله، ويمكن عندئذٍ إكمال هذا.
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-record-part-unread = يبقى ما وضعه هذا التثبيت في { $folder }، لأنه تعذرت قراءة ذلك الجزء من السجل.

## Why a file or folder was left, shown after "was left:" in the messages below them. Each is part of a sentence, so
## starts with a small letter and has no full stop.

# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-being-changed = كان شيء ما يغيّر { $folder } أثناء تشغيل هذا، لذا لم يتم فعل أي شيء آخر فيه. أغلق ما يفعل ذلك، وشغّل هذا مرة أخرى
# { $folder } is a folder; { $error } is what Windows said went wrong.
core-rollback-reason-not-opened = تعذر فتح { $folder }: { $error }
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-no-record = لا يوجد سجل لمكان { $folder }
# { $folder } is a folder; { $actual } is the folder it now leads to.
core-rollback-reason-moved = يؤدي { $folder } الآن إلى { $actual }، وليس إلى المكان الذي كان فيه أثناء التثبيت
# Why a service was not stopped.
core-rollback-reason-did-not-stop = لم يكتمل الإيقاف في الوقت المحدد

## Files and folders

# { $path } is a folder; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-folder-left = تُرك المجلد { $path } في مكانه: { $reason }.
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-removed = تمت إزالة المجلد { $path }
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-holds-others = يبقى المجلد { $path }، لأنه يحتوي على ملفات لم يضعها هذا التثبيت هناك.
# { $path } is a folder; { $error } is what Windows said went wrong.
core-rollback-folder-not-removed = تعذرت إزالة المجلد { $path }: { $error }.
# { $path } is a file; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-file-left = تُرك الملف { $path } في مكانه: { $reason }.
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-folder-gone = لن تتم إعادة { $path }، لأن المجلد الذي كان ضمنه لم يعد موجودًا.
# { $path } is a file; { $error } is what went wrong.
core-rollback-partial-not-removed = تعذرت إزالة ملف جزئي: { $path } ({ $error }).
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-checked = تعذر التحقق من { $path }: { $error }.
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-removed = تمت إزالة { $path }
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-removed = تعذرت إزالة { $path }: { $error }.
# { $path } is a file this install had replaced, and has put back.
core-rollback-file-put-back = تمت إعادة { $path }
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-put-back = تعذرت إعادة { $path }: { $error }.
# Why a file could not be put back.
core-rollback-backup-gone = لم تعد النسخة الاحتياطية موجودة
# { $path } is a file.
core-rollback-no-parent-folder = ليس لـ { $path } مجلد أصل

## A file that stays. { $why } says why: still-used (something that stays still uses it), changed (it has changed since
## it was installed), or not-asked (it is someone's data, and they did not ask for it to go). { $path } is the file.

core-rollback-file-stays =
    { $why ->
        [still-used] يبقى { $path }، لأن شيئًا باقيًا لا يزال يستخدمه.
        [changed] يبقى { $path }، لأنه تغيّر منذ تثبيته.
       *[not-asked] يبقى { $path }، لأنه لم يُطلب إزالته.
    }
# The file it replaced was to be put beside it, under another name, and every name was taken.
core-rollback-file-stays-no-name-beside =
    { $why ->
        [still-used] يبقى { $path }، لأن شيئًا باقيًا لا يزال يستخدمه، وتعذر وضع الملف الذي استبدله بجانبه: كل الأسماء المتاحة له مستخدمة.
        [changed] يبقى { $path }، لأنه تغيّر منذ تثبيته، وتعذر وضع الملف الذي استبدله بجانبه: كل الأسماء المتاحة له مستخدمة.
       *[not-asked] يبقى { $path }، لأنه لم يُطلب إزالته، وتعذر وضع الملف الذي استبدله بجانبه: كل الأسماء المتاحة له مستخدمة.
    }
# { $name } is the name the file it replaced now has, such as settings.ini.before-install.
core-rollback-file-stays-beside =
    { $why ->
        [still-used] يبقى { $path }، لأن شيئًا باقيًا لا يزال يستخدمه. الملف الذي استبدله موجود بجانبه، باسم { $name }.
        [changed] يبقى { $path }، لأنه تغيّر منذ تثبيته. الملف الذي استبدله موجود بجانبه، باسم { $name }.
       *[not-asked] يبقى { $path }، لأنه لم يُطلب إزالته. الملف الذي استبدله موجود بجانبه، باسم { $name }.
    }
# { $error } is what went wrong.
core-rollback-file-stays-not-beside =
    { $why ->
        [still-used] يبقى { $path }، لأن شيئًا باقيًا لا يزال يستخدمه، وتعذر وضع الملف الذي استبدله بجانبه: { $error }.
        [changed] يبقى { $path }، لأنه تغيّر منذ تثبيته، وتعذر وضع الملف الذي استبدله بجانبه: { $error }.
       *[not-asked] يبقى { $path }، لأنه لم يُطلب إزالته، وتعذر وضع الملف الذي استبدله بجانبه: { $error }.
    }

## The registry and environment variables. { $kind } says what { $name } is: key (a registry key, such as
## HKLM\Software\App), value (a registry value, such as HKLM\Software\App\Version), user-variable or machine-variable
## (an environment variable of the user's or of the whole PC, such as APP_HOME), or user-list or machine-list (a
## variable that is a list of folders, such as Path). { $error } is what Windows said went wrong.

# How a registry key's unnamed value is shown, as Registry Editor shows it.
core-registry-default-value-name = (افتراضي)
core-rollback-setting-other-account =
    { $kind ->
        [key] يبقى المفتاح { $name }، لأنه يخص الحساب الذي ثبّت هذا، وهو حساب لم يسجّل الدخول.
        [user-variable] يبقى متغير بيئة المستخدم { $name }، لأنه يخص الحساب الذي ثبّت هذا، وهو حساب لم يسجّل الدخول.
        [machine-variable] يبقى متغير بيئة الجهاز { $name }، لأنه يخص الحساب الذي ثبّت هذا، وهو حساب لم يسجّل الدخول.
        [user-list] يبقى { $name } الخاص بالمستخدم، لأنه يخص الحساب الذي ثبّت هذا، وهو حساب لم يسجّل الدخول.
        [machine-list] يبقى { $name } الخاص بالجهاز، لأنه يخص الحساب الذي ثبّت هذا، وهو حساب لم يسجّل الدخول.
       *[value] تبقى قيمة السجل { $name }، لأنها تخص الحساب الذي ثبّت هذا، وهو حساب لم يسجّل الدخول.
    }
core-rollback-setting-not-reached =
    { $kind ->
        [key] تعذر الوصول إلى المفتاح { $name }: { $error }.
        [user-variable] تعذر الوصول إلى متغير بيئة المستخدم { $name }: { $error }.
        [machine-variable] تعذر الوصول إلى متغير بيئة الجهاز { $name }: { $error }.
        [user-list] تعذر الوصول إلى { $name } الخاص بالمستخدم: { $error }.
        [machine-list] تعذر الوصول إلى { $name } الخاص بالجهاز: { $error }.
       *[value] تعذر الوصول إلى قيمة السجل { $name }: { $error }.
    }
core-rollback-setting-not-read =
    { $kind ->
        [key] تعذرت قراءة المفتاح { $name }: { $error }.
        [user-variable] تعذرت قراءة متغير بيئة المستخدم { $name }: { $error }.
        [machine-variable] تعذرت قراءة متغير بيئة الجهاز { $name }: { $error }.
        [user-list] تعذرت قراءة { $name } الخاص بالمستخدم: { $error }.
        [machine-list] تعذرت قراءة { $name } الخاص بالجهاز: { $error }.
       *[value] تعذرت قراءة قيمة السجل { $name }: { $error }.
    }
core-rollback-setting-changed =
    { $kind ->
        [user-variable] يبقى متغير بيئة المستخدم { $name }، لأنه تغيّر منذ تثبيته.
        [machine-variable] يبقى متغير بيئة الجهاز { $name }، لأنه تغيّر منذ تثبيته.
        [user-list] يبقى { $name } الخاص بالمستخدم، لأنه تغيّر منذ تثبيته.
        [machine-list] يبقى { $name } الخاص بالجهاز، لأنه تغيّر منذ تثبيته.
       *[value] تبقى قيمة السجل { $name }، لأنها تغيّرت منذ تثبيتها.
    }
# Said when it is back as it was before the install.
core-rollback-setting-put-back =
    { $kind ->
        [user-variable] تمت إعادة متغير بيئة المستخدم { $name }
        [machine-variable] تمت إعادة متغير بيئة الجهاز { $name }
        [user-list] تمت إعادة { $name } الخاص بالمستخدم
        [machine-list] تمت إعادة { $name } الخاص بالجهاز
       *[value] تمت إعادة قيمة السجل { $name }
    }
core-rollback-setting-not-put-back =
    { $kind ->
        [user-variable] تعذرت إعادة متغير بيئة المستخدم { $name }: { $error }.
        [machine-variable] تعذرت إعادة متغير بيئة الجهاز { $name }: { $error }.
        [user-list] تعذرت إعادة { $name } الخاص بالمستخدم: { $error }.
        [machine-list] تعذرت إعادة { $name } الخاص بالجهاز: { $error }.
       *[value] تعذرت إعادة قيمة السجل { $name }: { $error }.
    }
# A registry value the install wrote, which is the user's data, and stays unless they asked for it to go.
core-rollback-value-kept-by-request = تبقى قيمة السجل { $name }، لأنه لم يُطلب إزالتها.
core-rollback-key-holds-others = يبقى المفتاح { $name }، لأنه يحتوي على ما لم يضعه هذا التثبيت هناك.
core-rollback-key-removed = تمت إزالة المفتاح { $name }
core-rollback-key-not-removed = تعذرت إزالة المفتاح { $name }: { $error }.
# { $kind } is user-list or machine-list.
core-rollback-list-entries-removed =
    { $kind ->
        [user-list] تمت إزالة إدخالات هذا التثبيت من { $name } الخاص بالمستخدم
       *[machine-list] تمت إزالة إدخالات هذا التثبيت من { $name } الخاص بالجهاز
    }
core-rollback-list-entries-not-removed =
    { $kind ->
        [user-list] تعذرت إزالة إدخالات هذا التثبيت من { $name } الخاص بالمستخدم: { $error }.
       *[machine-list] تعذرت إزالة إدخالات هذا التثبيت من { $name } الخاص بالجهاز: { $error }.
    }

## Services and drivers. { $kind } is service or driver; { $name } is its name, such as AppUpdater. { $error } and
## { $reason } are what Windows said went wrong.

core-rollback-service-not-read =
    { $kind ->
        [driver] تعذرت قراءة برنامج التشغيل { $name }: { $error }.
       *[service] تعذرت قراءة الخدمة { $name }: { $error }.
    }
core-rollback-service-changed =
    { $kind ->
        [driver] يبقى برنامج التشغيل { $name }، لأنه تغيّر منذ تثبيته.
       *[service] تبقى الخدمة { $name }، لأنها تغيّرت منذ تثبيتها.
    }
core-rollback-service-not-stopped =
    { $kind ->
        [driver] تعذر إيقاف برنامج التشغيل { $name } ({ $reason })، لذا قد يبقى ما يستخدمه حتى إعادة تشغيل الكمبيوتر.
       *[service] تعذر إيقاف الخدمة { $name } ({ $reason })، لذا قد يبقى ما تستخدمه حتى إعادة تشغيل الكمبيوتر.
    }
core-rollback-service-stopped-for-this =
    { $kind ->
        [driver] تم إيقاف برنامج التشغيل { $name } لتنفيذ هذا. ابدأ تشغيله مرة أخرى عندما تحتاج إليه.
       *[service] تم إيقاف الخدمة { $name } لتنفيذ هذا. ابدأ تشغيلها مرة أخرى عندما تحتاج إليها.
    }
core-rollback-service-not-removed =
    { $kind ->
        [driver] تعذرت إزالة برنامج التشغيل { $name }: { $reason }.
       *[service] تعذرت إزالة الخدمة { $name }: { $reason }.
    }
core-rollback-service-marked-for-removal =
    { $kind ->
        [driver] تم وضع علامة على برنامج التشغيل { $name } لإزالته، وستتم إزالته عند إعادة تشغيل الكمبيوتر أو عندما يحرره ما يفتحه.
       *[service] تم وضع علامة على الخدمة { $name } لإزالتها، وستتم إزالتها عند إعادة تشغيل الكمبيوتر أو عندما يحررها ما يفتحها.
    }
core-rollback-service-removed =
    { $kind ->
        [driver] تمت إزالة برنامج التشغيل { $name }
       *[service] تمت إزالة الخدمة { $name }
    }
core-rollback-service-not-restored =
    { $kind ->
        [driver] تعذرت إعادة برنامج التشغيل { $name } إلى ما كان عليه: { $reason }.
       *[service] تعذرت إعادة الخدمة { $name } إلى ما كانت عليه: { $reason }.
    }
core-rollback-service-restored =
    { $kind ->
        [driver] تمت إعادة إعدادات برنامج التشغيل { $name }
       *[service] تمت إعادة إعدادات الخدمة { $name }
    }
core-rollback-service-not-restarted = كانت الخدمة { $name } قيد التشغيل قبل هذا التثبيت، ولم تبدأ مرة أخرى: { $reason }.
# { $account } is the account the service ran as, such as CONTOSO\backup.
core-rollback-service-account-again = كانت الخدمة { $name } تعمل باسم { $account }. لا يتم الاحتفاظ بكلمة المرور الخاصة بها، لذا عيّن حسابها مرة أخرى في "الخدمات".

## Firewall rules. { $name } is the rule's name; { $error } is what went wrong.

core-rollback-firewall-not-read = تعذرت قراءة قاعدة جدار الحماية { $name }: { $error }.
core-rollback-firewall-says-otherwise = تبقى قاعدة جدار الحماية { $name }، لأن ما يوجد بهذا الاسم يذكر شيئًا آخر.
core-rollback-firewall-changed = تبقى قاعدة جدار الحماية { $name }، لأنها تغيّرت منذ تثبيتها.
# The capital "The" is as the app has always said it.
core-rollback-firewall-removed = تمت إزالة قاعدة جدار الحماية { $name }
core-rollback-firewall-not-removed = تعذرت إزالة قاعدة جدار الحماية { $name }: { $error }.

## Scheduled tasks. { $path } is a folder of tasks, and { $name } a task with its folder, such as \App\Nightly Update.
## { $error } and { $reason } are what went wrong.

core-rollback-task-folder-holds-others = يبقى مجلد المهام { $path }، لأنه يحتوي على مهام لم يضعها هذا التثبيت هناك.
core-rollback-task-folder-not-removed = تعذرت إزالة مجلد المهام { $path }: { $error }.
core-rollback-task-not-read = تعذرت قراءة المهمة { $name }: { $error }.
core-rollback-task-changed = تبقى المهمة { $name }، لأنها تغيّرت منذ تثبيتها.
core-rollback-task-removed = تمت إزالة المهمة { $name }
core-rollback-task-not-removed = تعذرت إزالة المهمة { $name }: { $reason }.
core-rollback-task-put-back = تمت إعادة المهمة { $name }
core-rollback-task-not-put-back = تعذرت إعادة المهمة { $name }: { $reason }.

## When an install cannot be undone at all

# { $error } is what went wrong.
core-rollback-record-unreadable = تعذرت قراءة سجل هذا التثبيت، لذا لم يتم التراجع عن أي شيء: { $error }.
# { $error } is what went wrong.
core-rollback-nothing-undone = لم يتم التراجع عن أي شيء: { $error }.
# One app's part of the message below. { $app_name } is the app's name; { $failures } is what could not be undone, a
# sentence or more; { $folder } is the folder that holds the install's record.
core-rollback-earlier-install-stuck = { $app_name }: { $failures } إذا تعذر التراجع عنه نهائيًا، فيمكن للمسؤول التخلي عنه بإزالة { $folder }.
# Why an install did not start. { $details } is one or more of the message above.
core-rollback-earlier-installs-not-undone = تعذر التراجع بالكامل عن تثبيت سابق لم يكتمل، لذا لم يتم تثبيت أي شيء. أغلق كل ما يستخدم هذه العناصر وحاول مرة أخرى. { $details }
core-rollback-install-busy = يتم تغيير هذا التثبيت بواسطة برنامج تثبيت أو برنامج إلغاء تثبيت آخر

## The summary an uninstall writes on the command line, one line each, with what each says below it. { $count } is how
## many.

core-rollback-count-undone = تم التراجع عنه: { $count }
core-rollback-count-failed = تعذر التراجع عنه: { $count }
# What was left on purpose: it has changed since the install, or it is the user's.
core-rollback-count-kept = تُرك في مكانه: { $count }
# What no uninstall can take back, such as a runtime that was installed, which other programs may use.
core-rollback-count-left-behind = متبقٍ: { $count }

## Which Windows a PC or drive has, as the app shows it beside the drive.

# { $name } is the Windows' name, such as Windows 7 Professional Service Pack 1.
core-system-windows-32-bit = { $name }، 32 بت
# { $name } is the Windows' name, such as Windows 11 Pro.
core-system-windows-64-bit = { $name }، 64 بت
# A drive's Windows, when it only says it is older than Vista.
core-system-windows-xp-or-older = Windows XP أو أقدم

## Reading a connected drive, or another account's settings. Each is why it could not be done, and some start with a
## small letter, as they follow other words.

core-system-needs-administrator = تتطلب قراءة محرك أقراص متصل أو إعدادات حساب آخر حقوق المسؤول
core-system-no-program-data = تعذر العثور على مجلد ProgramData على هذا الكمبيوتر
# { $path } is a file, such as D:\Windows\System32\config\SOFTWARE.
core-system-not-a-hive-file = ليس ملف خلية سجل: { $path }
# { $path } is a file; { $size } is its size in bytes.
core-system-hive-too-large = حجم { $path } هو { $size } بايت، وهو أكبر من أن يكون خلية سجل، أو أكبر مما يمكن أن يبلغه مجموع خلايا السجل على محرك الأقراص هذا
# { $key } is a registry key, such as Software\Microsoft\Windows\CurrentVersion.
core-system-hive-no-mapping = لا يوجد تعيين في الخلية غير المتصلة لـ HKLM\{ $key }
# { $key } is where a drive's registry was to be loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said.
# "reg load" is a command's name.
core-system-hive-not-loaded = فشل reg load { $key }: { $error }
# { $key } is where a drive's registry was loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said. "reg
# unload" is a command's name.
core-system-hive-not-unloaded = فشل reg unload { $key }: { $error }
# { $drive } is a drive, such as D:\.
core-system-not-a-system-drive = { $drive } ليس محرك أقراص نظام Windows (لا يوجد مجلد \Windows).
# { $drive } is a drive, such as C:\. "This PC" is the name of a choice in the app.
core-system-own-system-drive = { $drive } هو محرك أقراص النظام الخاص بهذا الكمبيوتر. اختر "هذا الكمبيوتر" لقراءة هذا الكمبيوتر.
# { $drive } is a drive, such as D:\.
core-system-registry-through-link = { $drive } ليس محرك أقراص نظام Windows: الطريق إلى السجل الخاص به يمر عبر ارتباط.

## Warnings a scan records about whose data it read.

# { $account } is an account's name or id, such as alice or S-1-5-21-...-1001.
core-warning-scan-account-not-on-record = الحساب { $account } غير مسجل هنا، لذا لم يتم تضمين بيانات أو إعدادات أي مستخدم.
# { $account } is an account's name, such as CONTOSO\alice.
core-warning-scan-signed-in-account-no-profile = الحساب الذي سجّل الدخول هنا، { $account }، ليس له ملف تعريف مسجل، لذا لم يتم تضمين بيانات أو إعدادات أي مستخدم.
# { $reason } is why Windows could not say.
core-warning-scan-signed-in-account-unknown = تعذر تحديد من سجّل الدخول ({ $reason })، لذا لم يتم تضمين بيانات أو إعدادات أي مستخدم. اختر الحساب الذي تريد تضمين بياناته.
core-warning-scan-last-account-unknown = لا يذكر محرك الأقراص هذا من سجّل الدخول آخر مرة، ويحتوي على أكثر من حساب، لذا لم يتم تضمين بيانات أو إعدادات أي مستخدم. اختر الحساب الذي تريد تضمين بياناته.
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-settings-unreadable = تعذرت قراءة إعدادات { $account }، لذا لم يتم تضمينها: { $error }
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-classes-unreadable = تعذرت قراءة فئات COM وأنواع الملفات المسجلة لـ { $account }، لذا لم يتم تضمينها: { $error }

## Paths a scan did not follow. { $count } is how many; { $paths } is some of them, separated by semicolons; { $more } is
## how many more there are, not shown.

core-warning-scan-drive-paths-not-followed =
    { $count ->
        [zero] تعذرت قراءة بعض المسارات المسجلة على محرك الأقراص هذا، لذا لم يتم تضمين أي شيء فيها. كل منها إما على قرص آخر، أو ليس مسارًا كاملاً، أو يمر عبر ارتباط قد يؤدي إلى خارج محرك الأقراص: { $paths }
        [one] تعذرت قراءة مسار مسجل على محرك الأقراص هذا، لذا لم يتم تضمين أي شيء فيه. وهو إما على قرص آخر، أو ليس مسارًا كاملاً، أو يمر عبر ارتباط قد يؤدي إلى خارج محرك الأقراص: { $paths }
        [two] تعذرت قراءة مسارين مسجلين على محرك الأقراص هذا، لذا لم يتم تضمين أي شيء فيهما. كل منهما إما على قرص آخر، أو ليس مسارًا كاملاً، أو يمر عبر ارتباط قد يؤدي إلى خارج محرك الأقراص: { $paths }
        [few] تعذرت قراءة بعض المسارات المسجلة على محرك الأقراص هذا، لذا لم يتم تضمين أي شيء فيها. كل منها إما على قرص آخر، أو ليس مسارًا كاملاً، أو يمر عبر ارتباط قد يؤدي إلى خارج محرك الأقراص: { $paths }
        [many] تعذرت قراءة بعض المسارات المسجلة على محرك الأقراص هذا، لذا لم يتم تضمين أي شيء فيها. كل منها إما على قرص آخر، أو ليس مسارًا كاملاً، أو يمر عبر ارتباط قد يؤدي إلى خارج محرك الأقراص: { $paths }
       *[other] تعذرت قراءة بعض المسارات المسجلة على محرك الأقراص هذا، لذا لم يتم تضمين أي شيء فيها. كل منها إما على قرص آخر، أو ليس مسارًا كاملاً، أو يمر عبر ارتباط قد يؤدي إلى خارج محرك الأقراص: { $paths }
    }
core-warning-scan-drive-paths-not-followed-more = تعذرت قراءة بعض المسارات المسجلة على محرك الأقراص هذا، لذا لم يتم تضمين أي شيء فيها. كل منها إما على قرص آخر، أو ليس مسارًا كاملاً، أو يمر عبر ارتباط قد يؤدي إلى خارج محرك الأقراص: { $paths } و{ $more } غيرها
core-warning-scan-network-paths-not-followed =
    { $count ->
        [zero] تعذرت قراءة بعض المسارات المسجلة على هذا الكمبيوتر، لذا لم يتم تضمين أي شيء فيها. كل منها إما ليس على قرص في هذا الكمبيوتر، فيؤدي فحصه إلى الاتصال به، أو له اسم قد يفتح شيئًا آخر: { $paths }
        [one] تعذرت قراءة مسار مسجل على هذا الكمبيوتر، لذا لم يتم تضمين أي شيء فيه. وهو إما ليس على قرص في هذا الكمبيوتر، فيؤدي فحصه إلى الاتصال به، أو له اسم قد يفتح شيئًا آخر: { $paths }
        [two] تعذرت قراءة مسارين مسجلين على هذا الكمبيوتر، لذا لم يتم تضمين أي شيء فيهما. كل منهما إما ليس على قرص في هذا الكمبيوتر، فيؤدي فحصه إلى الاتصال به، أو له اسم قد يفتح شيئًا آخر: { $paths }
        [few] تعذرت قراءة بعض المسارات المسجلة على هذا الكمبيوتر، لذا لم يتم تضمين أي شيء فيها. كل منها إما ليس على قرص في هذا الكمبيوتر، فيؤدي فحصه إلى الاتصال به، أو له اسم قد يفتح شيئًا آخر: { $paths }
        [many] تعذرت قراءة بعض المسارات المسجلة على هذا الكمبيوتر، لذا لم يتم تضمين أي شيء فيها. كل منها إما ليس على قرص في هذا الكمبيوتر، فيؤدي فحصه إلى الاتصال به، أو له اسم قد يفتح شيئًا آخر: { $paths }
       *[other] تعذرت قراءة بعض المسارات المسجلة على هذا الكمبيوتر، لذا لم يتم تضمين أي شيء فيها. كل منها إما ليس على قرص في هذا الكمبيوتر، فيؤدي فحصه إلى الاتصال به، أو له اسم قد يفتح شيئًا آخر: { $paths }
    }
core-warning-scan-network-paths-not-followed-more = تعذرت قراءة بعض المسارات المسجلة على هذا الكمبيوتر، لذا لم يتم تضمين أي شيء فيها. كل منها إما ليس على قرص في هذا الكمبيوتر، فيؤدي فحصه إلى الاتصال به، أو له اسم قد يفتح شيئًا آخر: { $paths } و{ $more } غيرها

## What a scan with none of an account's data leaves out of a package, in the app's list of what was left out. Each
## has a name, and below it why.

core-account-left-out-shortcuts = الاختصارات الموجودة في المجلدات الخاصة بهذا الحساب
# { $count } is how many shortcuts.
core-account-left-out-shortcuts-why =
    { $count ->
        [zero] { $count } اختصار، لا تُضمّن إلا مع خيار "بيانات المستخدم"
        [one] اختصار واحد، لا يُضمّن إلا مع خيار "بيانات المستخدم"
        [two] اختصاران، لا يُضمّنان إلا مع خيار "بيانات المستخدم"
        [few] { $count } اختصارات، لا تُضمّن إلا مع خيار "بيانات المستخدم"
        [many] { $count } اختصارًا، لا تُضمّن إلا مع خيار "بيانات المستخدم"
       *[other] { $count } اختصار، لا تُضمّن إلا مع خيار "بيانات المستخدم"
    }
core-account-left-out-variables = متغيرات البيئة الخاصة بهذا الحساب
# { $count } is how many variables.
core-account-left-out-variables-why =
    { $count ->
        [zero] { $count } متغير، لا تُضمّن إلا مع خيار "بيانات المستخدم"
        [one] متغير واحد، لا يُضمّن إلا مع خيار "بيانات المستخدم"
        [two] متغيران، لا يُضمّنان إلا مع خيار "بيانات المستخدم"
        [few] { $count } متغيرات، لا تُضمّن إلا مع خيار "بيانات المستخدم"
        [many] { $count } متغيرًا، لا تُضمّن إلا مع خيار "بيانات المستخدم"
       *[other] { $count } متغير، لا تُضمّن إلا مع خيار "بيانات المستخدم"
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-account-left-out-task = المهمة المجدولة { $task }
core-account-task-works-on-account-folder = تعمل على مكان في مجلدات حساب، أو تحمل اسمه، ولا تُضمّن إلا مع خيار "بيانات المستخدم"
core-account-task-runs-from-account-folder = ما تشغّله أيضًا من مجلدات حساب لا يُضمّن إلا مع خيار "بيانات المستخدم"
# { $service } is a service's name, such as AppUpdater.
core-account-left-out-service = الخدمة { $service }
core-account-service-works-on-account-folder = تعمل على مكان في مجلدات حساب، ولا تُضمّن إلا مع خيار "بيانات المستخدم"

## What a package was found to still say of whose PC it came from, after a build with none of an account's data.

# The parts of a package, as the warning below names them.
core-account-part-files = مسارات ملفات التطبيق نفسه
core-account-part-registry = قيم السجل
core-account-part-settings-entry = إدخال التطبيق في "الإعدادات"
core-account-part-main-program = موقع برنامجه الرئيسي
core-account-part-shortcuts = الاختصارات
core-account-part-tasks = المهام المجدولة
core-account-part-services = الخدمات
core-account-part-variables = متغيرات البيئة
# One place in the list below. { $place } is one of the parts above, or a file's name; { $count } is how many times.
core-account-mention-place = { $place } ({ $count })
# { $kind } is what was found: account (an account's name), folder (where an account's folder is), computer (the PC's
# name), or id (an account's id). { $places } is a list of where, from the message above.
core-warning-scan-package-mentions =
    { $kind ->
        [account] تذكر هذه الحزمة اسم حساب في { $places }. تم إنشاؤها دون بيانات أي حساب، ويمكن لأي شخص لديه برنامج التثبيت قراءة ذلك.
        [folder] تذكر هذه الحزمة موقع مجلد حساب في { $places }. تم إنشاؤها دون بيانات أي حساب، ويمكن لأي شخص لديه برنامج التثبيت قراءة ذلك.
        [computer] تذكر هذه الحزمة اسم الكمبيوتر في { $places }. تم إنشاؤها دون بيانات أي حساب، ويمكن لأي شخص لديه برنامج التثبيت قراءة ذلك.
       *[id] تذكر هذه الحزمة معرّف حساب في { $places }. تم إنشاؤها دون بيانات أي حساب، ويمكن لأي شخص لديه برنامج التثبيت قراءة ذلك.
    }
# { $count } is how many folders.
core-warning-scan-folders-of-others-unreadable =
    { $count ->
        [zero] تعذرت قراءة { $count } مجلد خارج التطبيق، لذا قد تكون بيانات التطبيق الموجودة فيها مفقودة.
        [one] تعذرت قراءة مجلد واحد خارج التطبيق، لذا قد تكون بيانات التطبيق الموجودة فيه مفقودة.
        [two] تعذرت قراءة مجلدين خارج التطبيق، لذا قد تكون بيانات التطبيق الموجودة فيهما مفقودة.
        [few] تعذرت قراءة { $count } مجلدات خارج التطبيق، لذا قد تكون بيانات التطبيق الموجودة فيها مفقودة.
        [many] تعذرت قراءة { $count } مجلدًا خارج التطبيق، لذا قد تكون بيانات التطبيق الموجودة فيها مفقودة.
       *[other] تعذرت قراءة { $count } مجلد خارج التطبيق، لذا قد تكون بيانات التطبيق الموجودة فيها مفقودة.
    }

## What a scan leaves out because it looks like a credential, or is a license with the License option off, in the app's
## list of what was left out. Each has a name, and below it why.

core-secret-why-credential = يبدو كبيانات اعتماد
core-secret-why-license-option-off = خيار "بيانات الترخيص والتنشيط" متوقف
# { $count } is how many values of a registry key.
core-secret-why-account-values =
    { $count ->
        [zero] { $count } قيمة، لا تُضمّن إلا مع خيار "بيانات المستخدم"
        [one] قيمة واحدة، لا تُضمّن إلا مع خيار "بيانات المستخدم"
        [two] قيمتان، لا تُضمّنان إلا مع خيار "بيانات المستخدم"
        [few] { $count } قيم، لا تُضمّن إلا مع خيار "بيانات المستخدم"
        [many] { $count } قيمة، لا تُضمّن إلا مع خيار "بيانات المستخدم"
       *[other] { $count } قيمة، لا تُضمّن إلا مع خيار "بيانات المستخدم"
    }
# Shown in place of a registry value's name when its name itself looks like a password or a key.
core-secret-name-holds-credential = (قيمة يحتوي اسمها على بيانات اعتماد)
# Shown in place of an option's name, for an argument on a command line that has no name.
core-secret-unnamed-argument = وسيطة تحتوي على بيانات اعتماد
# { $option } is an option taken out of a command line, such as /password, or the message above. { $place } says
# where the command line was: service, task, shortcut, opener (what opens a file type), opener-unnamed (the same, of a
# type with no name), or uninstall. { $name } is the service, task, shortcut, file type, or app.
core-secret-taken-from =
    { $place ->
        [service] { $option } في طريقة تشغيل { $name }
        [task] { $option } في طريقة تشغيل المهمة { $name }
        [shortcut] { $option } في الاختصار { $name }
        [opener] { $option } فيما يفتح { $name }
        [opener-unnamed] { $option } فيما يفتح نوع ملف
       *[uninstall] { $option } في طريقة إلغاء تثبيت { $name }
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-secret-task-definition-unreadable = تعريف المهمة { $task }، الذي تعذرت قراءته للتحقق منه
# { $name } is an environment variable's name, such as APP_TOKEN.
core-secret-left-out-variable = متغير البيئة { $name }
# { $class } is a COM class's id, such as {00000000-0000-0000-0000-000000000000}.
core-secret-left-out-class-server = ما تذكر الفئة { $class } أنها تشغّله
# { $rule } is a firewall rule's name.
core-secret-left-out-firewall-description = وصف قاعدة جدار الحماية { $rule }

## Licenses a scan found, as the package's report describes each.

# { $name } is a file's name, such as license.lic.
core-license-detail-file = تم الكشف عن ملف ترخيص: { $name }
# { $framework } is a licensing product's name, such as FlexLM / FlexNet.
core-license-detail-store = مخزن ترخيص { $framework } موجود
# { $name } is an environment variable's name, such as LM_LICENSE_FILE.
core-license-detail-variable = متغير بيئة الترخيص { $name }
# { $name } is a registry value's name, such as SerialNumber.
core-license-detail-registry-value = تشير قيمة السجل إلى بيانات ترخيص: { $name }
# Said of a registry value in an account's key, which is not named.
core-license-detail-registry-value-of-an-account = تشير قيمة السجل إلى بيانات ترخيص
# { $key } is a registry key, such as HKLM\Software\Contoso\License.
core-license-detail-registry-key = يشير مفتاح السجل إلى بيانات ترخيص: { $key }

## Warnings a scan records about licenses.

core-warning-scan-license-data = تم الكشف عن بيانات ترخيص/تنشيط. عند تشغيل خيار "بيانات الترخيص والتنشيط"، يستعيد برنامج التثبيت ما يمكن نقله؛ أما القيم على مستوى الجهاز والمخازن المرتبطة بالأجهزة فيتم الإبلاغ عنها فقط. لا يفك WinAppMigrator التشفير أبدًا، ولا يتجاوز التنشيط، ولا يحاكي مفاتيح الحماية المادية.
core-warning-scan-license-machine-bound = بعض التراخيص مرتبطة بالجهاز (مفتاح حماية مادي، أو تنشيط مقفل على الجهاز، أو ترخيص مشفر). يستعيد برنامج التثبيت ما يستطيع؛ وقد تظل هذه التراخيص بحاجة إلى إعادة التنشيط على الكمبيوتر الجديد.

## Warnings a scan records about the registry. What could not be read is not named in them: it is kept beside them.

# { $error } is what Windows said went wrong. App Paths is a registry key's name.
core-warning-scan-app-paths-key-unreadable = تعذرت قراءة مفتاح فرعي في App Paths: { $error }
core-warning-scan-registry-limit = بلغ التقاط السجل حد الأمان المكوّن.
core-warning-scan-registry-subtree-limit = بلغ التقاط شجرة فرعية من السجل حد الأمان المكوّن.
# { $error } is what Windows said went wrong.
core-warning-scan-registry-key-unreadable = تعذرت قراءة مفتاح السجل: { $error }
core-warning-scan-registry-value-too-large = تجاوزت قيمة السجل حد حجم الالتقاط المكوّن.

## Scheduled tasks

core-warning-scan-tasks-folder-through-link = يمر مجلد المهام المجدولة على محرك الأقراص هذا عبر ارتباط، لذا لم تتم قراءة مهامه.
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-warning-scan-task-left-out = تم استبعاد المهمة المجدولة { $task }: إما أنها تعمل باسم حساب غير الحساب الذي يتم تضمين إعداداته، أو يمكن لغير المسؤولين تغيير ملفها، أو أن لدى التطبيق مهامًا أكثر من أن يُحتفظ بها.
# { $error } is what Windows said went wrong.
core-warning-scan-tasks-folder-unreadable = تعذرت قراءة جزء من المهام المجدولة: { $error }
# { $path } is a task's file, such as C:\Windows\System32\Tasks\App\Nightly Sync; { $reason } is one of the reasons
# below, or what Windows said went wrong.
core-warning-scan-task-unreadable = تعذرت قراءة المهمة المجدولة { $path }: { $reason }. إذا كانت تشغّل التطبيق، فلم يتم تضمينها.
# Why a task's file could not be read. { $size } is its size in bytes.
core-task-too-large = حجمه { $size } بايت، وهو أكبر من أن يكون ملف مهمة
# Why a task's file could not be read.
core-task-ends-mid-character = ينتهي في منتصف حرف
# Why a task's file could not be read. { $error } says where.
core-task-not-utf16 = ليس بترميز UTF-16 صالح: { $error }
# Why a task's file could not be read. { $error } says where.
core-task-not-text = ليس بترميز UTF-8 أو UTF-16 صالح: { $error }
# What a package records of how a task is put back. Register-ScheduledTask is a command's name.
core-task-restore-instructions = تستخدم الاستعادة Register-ScheduledTask مع تعريف XML المؤرشف.

## Reading a program's file. Each is why it could not be read, after the program's name.

# { $reason } is what went wrong.
core-pe-unreadable = تعذرت قراءته: { $reason }
core-pe-headers-misaligned = رؤوسه ليست على الحدود التي يجب أن تكون عليها رؤوس البرامج
# { $size } is the file's size in bytes.
core-pe-too-large = حجمه { $size } بايت، وهو أكبر من أن يكون برنامجًا
# PE is the name of the format Windows programs are in.
core-pe-no-headers = ليس له رؤوس PE

## Warnings a scan records about what a program needs. Which program is kept beside each.

# { $error } is what went wrong.
core-warning-scan-imports-unreadable = تعذرت قراءة جدول الاستيراد: { $error }
# { $error } is what went wrong.
core-warning-scan-import-name-unreadable = تعذرت قراءة اسم واصف استيراد: { $error }
# { $count } is how many were skipped.
core-warning-scan-legacy-delay-imports =
    { $count ->
        [zero] تم تخطي { $count } واصف تحميل مؤجل قديم (v1)؛ لا يتم تحليل سوى عمليات الاستيراد المؤجلة الحديثة المستندة إلى RVA.
        [one] تم تخطي واصف تحميل مؤجل قديم (v1) واحد؛ لا يتم تحليل سوى عمليات الاستيراد المؤجلة الحديثة المستندة إلى RVA.
        [two] تم تخطي واصفي تحميل مؤجل قديمين (v1)؛ لا يتم تحليل سوى عمليات الاستيراد المؤجلة الحديثة المستندة إلى RVA.
        [few] تم تخطي { $count } واصفات تحميل مؤجل قديمة (v1)؛ لا يتم تحليل سوى عمليات الاستيراد المؤجلة الحديثة المستندة إلى RVA.
        [many] تم تخطي { $count } واصفًا للتحميل المؤجل القديم (v1)؛ لا يتم تحليل سوى عمليات الاستيراد المؤجلة الحديثة المستندة إلى RVA.
       *[other] تم تخطي { $count } واصف تحميل مؤجل قديم (v1)؛ لا يتم تحليل سوى عمليات الاستيراد المؤجلة الحديثة المستندة إلى RVA.
    }

## The DLLs a program needs. Which DLL or program is kept beside each warning.

# { $path } is a program, such as C:\Program Files\App\app.exe.
core-dll-program-has-no-folder = ليس للملف التنفيذي دليل أصل: { $path }
# Why a DLL is among the app's files, in the package's list of them.
core-dll-beside-program = مكتبة DLL موجودة بجانب الملف التنفيذي المحدد
# { $error } is what went wrong.
core-warning-scan-dependency-unscanned = تعذر فحص تبعية متداخلة: { $error }
# { $name } is a DLL's name, such as d3dx9_43.dll.
core-warning-scan-dependency-not-in-newer-windows = كان { $name } في مجلد Windows على الكمبيوتر الذي كان عليه هذا التطبيق، ولا يأتي مع Windows 10 و11. لقد ثبّته شيء ما هناك، لذا فهو ليس جزءًا من ملفات التطبيق نفسه ولن يُضمّن. قد لا يبدأ التطبيق حتى يتم تثبيت ما يوفره.
# { $error } is what went wrong.
core-warning-scan-dependency-not-hashed = تعذر حساب تجزئة التبعية: { $error }
# SystemRoot and windir are the names of environment variables.
core-warning-scan-windows-folder-unknown = متغيرا البيئة SystemRoot/windir غير متوفرين؛ البحث عن مكتبات DLL الخاصة بالنظام محدود.
# { $error } is what went wrong.
core-warning-scan-dependency-architecture-unreadable = تعذرت قراءة بنية التبعية: { $error }

## Services, drivers, shortcuts, and environment variables a scan reads. Where it was reading is kept beside each
## warning.

# { $error } is what Windows said went wrong.
core-warning-scan-services-unreadable = تعذرت قراءة مفتاح سجل الخدمات: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-drivers-unreadable = تعذرت قراءة مفتاح سجل الخدمات لاكتشاف برامج التشغيل: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-queried = تعذر الاستعلام عن الاختصارات: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-parsed = تعذر تحليل مخرجات الاستعلام عن الاختصارات: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-environment-unreadable = تعذرت قراءة مفتاح سجل بيئة الجهاز: { $error }

## What a package records of how a driver is put back, which an installer shows. { $kind } is the driver's type as
## Windows names it (kernel, file-system, recognizer, or unknown-type); { $name } is its service's name; { $path } is
## its file. { $signed } is yes when { $signer } names who signed it, and no when that is not known.

core-service-driver-bundled =
    { $signed ->
        [yes] برنامج التشغيل "{ $name }" من نوع { $kind }، ويملكه التطبيق ({ $path }، موقّع من { $signer }). يجمّع برنامج التثبيت ملف .sys الخاص به ويعيد تسجيل خدمة برنامج التشغيل تلقائيًا (sc create، يتطلب امتيازات مرتفعة)، ثم يتأكد من تسجيل الخدمة.
       *[no] برنامج التشغيل "{ $name }" من نوع { $kind }، ويملكه التطبيق ({ $path }، التوقيع غير معروف). يجمّع برنامج التثبيت ملف .sys الخاص به ويعيد تسجيل خدمة برنامج التشغيل تلقائيًا (sc create، يتطلب امتيازات مرتفعة)، ثم يتأكد من تسجيل الخدمة.
    }
core-service-driver-from-vendor =
    { $signed ->
        [yes] برنامج التشغيل "{ $name }" من نوع { $kind }، وهو خاص بمورّد أو مضمّن في Windows ({ $path }، موقّع من { $signer }). لا يعيد WinAppMigrator توزيع برامج التشغيل الخاصة بجهات خارجية؛ أعد تثبيته على الجهاز الهدف باستخدام حزمة برنامج التشغيل الموقّعة من المورّد قبل تشغيل التطبيق.
       *[no] برنامج التشغيل "{ $name }" من نوع { $kind }، وهو خاص بمورّد أو مضمّن في Windows ({ $path }، التوقيع غير معروف). لا يعيد WinAppMigrator توزيع برامج التشغيل الخاصة بجهات خارجية؛ أعد تثبيته على الجهاز الهدف باستخدام حزمة برنامج التشغيل الموقّعة من المورّد قبل تشغيل التطبيق.
    }

## Runtimes an app needs, such as the Visual C++ runtime or .NET: how each is provided, as the Dependencies list says.

core-runtime-handling-built-in = مضمّنة في Windows
core-runtime-handling-ships-with-app = تأتي مع التطبيق
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-handling-windows-feature = ميزة Windows: { $feature }
core-runtime-handling-included = مضمّنة
# { $id } is the runtime's name in winget, such as Microsoft.VCRedist.2015+.x64.
core-runtime-handling-winget = يتم تثبيتها باستخدام winget: { $id }
core-runtime-handling-manual = تثبيت يدوي

## Downloading a runtime's installer, when an installer is built. Each is why it could not be, and starts with a small
## letter, as it follows other words. { $url } is a web address; { $error } is what went wrong.

core-runtime-download-not-https = تم رفض تنزيل { $url }: ليس HTTPS
# TLS is what keeps a download private.
core-runtime-tls-failed = فشلت تهيئة TLS: { $error }
core-runtime-download-failed = فشل تنزيل { $url }: { $error }
core-runtime-download-unreadable = فشلت قراءة { $url }: { $error }
core-runtime-download-empty = لم يُرجع تنزيل { $url } أي بيانات
# { $came } and { $expected } are numbers of bytes.
core-runtime-download-cut-short = انقطع تنزيل { $url } قبل اكتماله: وصل { $came } من أصل { $expected } بايت

## Installing a runtime: why it was not installed. Each starts with a small letter, as it follows other words.

core-runtime-no-known-installer = لا يوجد برنامج تثبيت معروف لمكتبة وقت التشغيل هذه
# { $path } is where in the package, such as prerequisites\vc_redist.x64.exe.
core-runtime-payload-misplaced = تذكر الحزمة { $path } كبرنامج تثبيت لمكتبة وقت التشغيل هذه، وهذا ليس مكانه الصحيح، لذا لم يتم تشغيله
# { $path } is a file.
core-runtime-payload-missing = برنامج التثبيت المجمّع مفقود: { $path }
# { $error } is what went wrong.
core-runtime-payload-path-invalid = مسار برنامج التثبيت المجمّع غير صالح: { $error }
# { $url } is the web address it can be downloaded from. winget is a program's name.
core-runtime-no-installer-route = لا يوجد برنامج تثبيت مجمّع ولا معرّف winget؛ احصل عليها من { $url }
# { $error } is what went wrong.
core-runtime-no-temp-folder = تعذر إنشاء مجلد لفك ضغطها فيه: { $error }
# { $reason } is what the check found.
core-runtime-payload-signature-failed = فشل برنامج التثبيت المجمّع في التحقق من التوقيع ولم يتم تشغيله: { $reason }
core-runtime-payload-timed-out = لم ينتهِ برنامج التثبيت المجمّع خلال المهلة المحددة وتم إنهاؤه
# { $error } is what went wrong.
core-runtime-payload-not-started = تعذر بدء برنامج التثبيت المجمّع: { $error }
core-runtime-installer-killed = تم إنهاء برنامج التثبيت دون رمز خروج
# { $code } is the number it ended with, such as 1603.
core-runtime-installer-failed = خرج برنامج التثبيت بالرمز { $code }
# { $feature } is a Windows feature's name, such as NetFx3; { $code } is the number DISM ended with. Windows Features is
# the name of a Windows window. Keep the command as it is.
core-runtime-feature-failed = تعذر على DISM تمكين ميزة Windows { $feature } (رمز الخروج { $code })؛ على الأرجح تعذر تنزيل الحمولة. مكّنها من "ميزات Windows"، أو شغّل `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`.
# The same, when DISM gave no number.
core-runtime-feature-failed-no-code = تعذر على DISM تمكين ميزة Windows { $feature } (رمز الخروج غير معروف)؛ على الأرجح تعذر تنزيل الحمولة. مكّنها من "ميزات Windows"، أو شغّل `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`.
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-feature-timed-out = لم ينتهِ تمكين ميزة Windows { $feature } خلال المهلة المحددة وتم إنهاؤه
# { $feature } is a Windows feature's name; { $error } is what went wrong.
core-runtime-feature-no-dism = DISM غير متوفر لتمكين ميزة Windows { $feature }: { $error }

## Installing a runtime with winget. { $id } is its name in winget, such as Microsoft.VCRedist.2015+.x64.

# { $error } is what went wrong.
core-runtime-winget-not-looked-for = تعذر البحث عن winget: { $error }
core-runtime-winget-not-installed = winget غير مثبت
# { $path } is where winget is.
core-runtime-winget-misplaced = { $path } ليس في المكان الذي يحتفظ فيه Windows بحزمه الخاصة، لذا لم يتم تشغيله
# { $path } is where winget is; { $reason } is what the check found.
core-runtime-winget-signature-failed = فشل { $path } في التحقق من التوقيع ولم يتم تشغيله: { $reason }
# { $reason } is one of the two messages above, or what went wrong.
core-runtime-winget-unusable = تعذر استخدام winget لتثبيت { $id }: { $reason }
# { $code } is the number winget ended with.
core-runtime-winget-failed = خرج winget install { $id } بالرمز { $code }
core-runtime-winget-failed-no-code = خرج winget install { $id } برمز غير معروف
core-runtime-winget-timed-out = لم ينتهِ winget install { $id } خلال المهلة المحددة وتم إنهاؤه
# { $error } is what went wrong.
core-runtime-winget-not-started = winget غير متوفر لتثبيت { $id }: { $error }

## What told a scan how an app was installed, as the package's report lists it. The names of installers and files are
## as their makers write them.

# { $path } is an installer's record of what it installed, such as C:\Program Files\App\unins000.dat.
core-source-record-not-believed = لم يتم استخدام { $path }: يمكن لغير المسؤولين تغييره
# { $count } is how many Windows Installer packages the bundle holds.
core-source-burn-bundle =
    { $count ->
        [zero] مجموعة Burn تضم { $count } حزمة Windows Installer
        [one] مجموعة Burn تضم حزمة Windows Installer واحدة
        [two] مجموعة Burn تضم حزمتي Windows Installer
        [few] مجموعة Burn تضم { $count } حزم Windows Installer
        [many] مجموعة Burn تضم { $count } حزمة Windows Installer
       *[other] مجموعة Burn تضم { $count } حزمة Windows Installer
    }
# { $count } is how many paths Windows Installer recorded.
core-source-msi-database =
    { $count ->
        [zero] قاعدة بيانات Windows Installer: { $count } مسار مفتاح مكوّن
        [one] قاعدة بيانات Windows Installer: مسار مفتاح مكوّن واحد
        [two] قاعدة بيانات Windows Installer: مسارا مفتاح مكوّن
        [few] قاعدة بيانات Windows Installer: { $count } مسارات مفاتيح مكوّنات
        [many] قاعدة بيانات Windows Installer: { $count } مسار مفتاح مكوّن
       *[other] قاعدة بيانات Windows Installer: { $count } مسار مفتاح مكوّن
    }
core-source-inno-log = سجل إلغاء تثبيت Inno Setup (unins000.dat)
# { $count } is how many paths the log records.
core-source-inno-log-paths =
    { $count ->
        [zero] سجل إلغاء تثبيت Inno Setup: { $count } مسار مسجل
        [one] سجل إلغاء تثبيت Inno Setup: مسار واحد مسجل
        [two] سجل إلغاء تثبيت Inno Setup: مساران مسجلان
        [few] سجل إلغاء تثبيت Inno Setup: { $count } مسارات مسجلة
        [many] سجل إلغاء تثبيت Inno Setup: { $count } مسارًا مسجلاً
       *[other] سجل إلغاء تثبيت Inno Setup: { $count } مسار مسجل
    }
core-source-nsis = توقيع برنامج إلغاء تثبيت NSIS (Nullsoft)
# { $count } is how many paths the log records.
core-source-nsis-log-paths =
    { $count ->
        [zero] NSIS AdvUninstLog uninstall.log: { $count } مسار مسجل
        [one] NSIS AdvUninstLog uninstall.log: مسار واحد مسجل
        [two] NSIS AdvUninstLog uninstall.log: مساران مسجلان
        [few] NSIS AdvUninstLog uninstall.log: { $count } مسارات مسجلة
        [many] NSIS AdvUninstLog uninstall.log: { $count } مسارًا مسجلاً
       *[other] NSIS AdvUninstLog uninstall.log: { $count } مسار مسجل
    }
core-source-squirrel = تخطيط Squirrel لكل مستخدم (Update.exe / app-*)
# { $version } is a folder's name that says the version, such as app-1.2.3.
core-source-squirrel-version = الإصدار الحالي لـ Squirrel: { $version }
core-source-chocolatey = بيانات تعريف حزمة Chocolatey
core-source-msix = حزمة MSIX / Appx
# { $identity } is a package's name, such as Contoso.App.
core-source-appx-identity = هوية AppxManifest: { $identity }
# { $count } is how many folders.
core-source-msix-data =
    { $count ->
        [zero] بيانات حزمة MSIX لكل مستخدم: { $count } موقع
        [one] بيانات حزمة MSIX لكل مستخدم: موقع واحد
        [two] بيانات حزمة MSIX لكل مستخدم: موقعان
        [few] بيانات حزمة MSIX لكل مستخدم: { $count } مواقع
        [many] بيانات حزمة MSIX لكل مستخدم: { $count } موقعًا
       *[other] بيانات حزمة MSIX لكل مستخدم: { $count } موقع
    }
# App Paths is a registry key's name.
core-source-app-paths = تسجيل App Paths

## Why a runtime is needed, as the package's report says it.

# { $program } is a program's file name, such as app.exe; { $dll } is a DLL's, such as msvcp140.dll.
core-runtime-program-imports = يستورد { $program } الملف { $dll }

## What else of an app's a scan finds in the registry: fonts and certificates, and why each is the app's.

# Why a font is among the app's files, in the package's list of them.
core-registry-font-recorded = خط يذكره سجل تثبيت التطبيق
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-named-for-the-app = يحمل اسم التطبيق أو اسم ناشره
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-recorded = يذكره سجل تثبيت التطبيق

## Who is signed in, for the Whose Data choice. Each of the first six is why it could not be told, and starts with a
## small letter, as it is shown inside other words. An account is shown by its id, such as S-1-5-21-...-1001.

# { $error } is what Windows said.
core-profile-desktop-owner-not-said = لم يذكر Windows لمن سطح المكتب هذا: { $error }
# { $error } is what went wrong.
core-profile-desktop-owner-not-asked = تعذر سؤال Windows عن صاحب سطح المكتب هذا: { $error }
core-profile-signed-in-account-no-profile = الحساب الذي سجّل الدخول هنا، { $account }، ليس له ملف تعريف مسجل
core-profile-desktop-not-a-person = سطح المكتب هنا يخص { $account }، وهو ليس حساب شخص
core-profile-nobody-signed-in-running-account-no-profile = لم يسجّل أحد الدخول هنا، والحساب الذي يشغّل هذا، { $account }، ليس له ملف تعريف مسجل
core-profile-nobody-signed-in-running-not-a-person = لم يسجّل أحد الدخول هنا، و{ $account }، الذي يشغّل هذا، ليس حساب شخص
# SID is the name Windows gives an account's id.
core-profile-nobody-signed-in-no-sid = لم يسجّل أحد الدخول هنا، وليس لهذا الحساب SID
# { $accounts } is a list of accounts.
core-profile-several-desktops = لدى أكثر من حساب سطح مكتب في هذه الجلسة: { $accounts }
# Under the Whose Data choice. { $reason } is one of the messages above.
core-profile-signed-in-unknown = تعذر تحديد من سجّل الدخول ({ $reason }).
# Under the Whose Data choice, when there are accounts to choose from. { $reason } is one of the messages above.
core-profile-signed-in-unknown-choose = تعذر تحديد من سجّل الدخول ({ $reason }). اختر الحساب الذي تريد تضمين بياناته.
