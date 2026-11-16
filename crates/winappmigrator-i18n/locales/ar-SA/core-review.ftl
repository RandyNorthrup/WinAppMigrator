# Words the core says about what an install may do: why an install location or something in a package is refused,
# the installer's list of what an app sets up beyond its files, the checks of a package and of an install, and whether
# there is room for either. Message names start with core-.
#
# Many messages here are a reason, the second half of a sentence that starts "... was left out:" or "that install
# location cannot be used:", so they start with a small letter and have no full stop.


## Why an install location cannot be used. Shown in red under the Install Location field.

core-review-location-state-root = إنه المكان الذي تحتفظ فيه عمليات التثبيت بسجلاتها
core-review-location-has-files = يحتوي بالفعل على ملفات، ولا يوجد تثبيت مسجل هنا، لذا استخدم مجلدًا فارغًا
core-policy-location-not-full-path = يجب أن يكون مسارًا كاملاً
core-policy-location-parent-step = يجب ألا يحتوي على ".."
core-policy-location-not-plain = يجب أن يكون مسارًا عاديًا على محرك أقراص في هذا الكمبيوتر
core-policy-location-whole-drive = لا يمكن أن يكون محرك أقراص بأكمله
core-policy-location-windows = لا يمكن أن يكون مجلد Windows
core-policy-location-runs-by-itself = إنه مجلد يخص Windows، أو مجلد يتم تشغيل محتوياته أو تحميلها تلقائيًا
core-policy-location-shared = يحتوي على عناصر أخرى جنبًا إلى جنب، لذا يحتاج التطبيق إلى مجلد بداخله

## Whether an app can run on this PC at all, said before anything is installed.

# { $reason } is why, as the second half of a sentence, such as "App.exe is built for Arm64, and this PC is x64, so
# Windows would not start it".
core-review-cannot-run-here = لا يمكن تشغيل هذا التطبيق على هذا الكمبيوتر، لذا لم يتم تغيير أي شيء: { $reason }
core-policy-architecture-x86 = x86 (32 بت)
core-policy-architecture-x64 = x64
core-policy-architecture-arm64 = Arm64
core-policy-architecture-arm32 = Arm (32 بت)
core-policy-architecture-unknown = غير معروفة
# { $code } is the number Windows gives a kind of processor, such as 0x01c4.
core-policy-machine-type = ذات رمز المعالج { $code }
# { $names } is a list of program names, such as "a.exe, b.exe, c.exe", and { $count } how many more there are.
core-policy-names-and-more = { $names }، و{ $count } غيرها
core-policy-fit-programs-not-said = لا يذكر برنامج التثبيت هذا ما تحتاج إليه برامجه، لذا لم يتم التحقق مما إذا كان يمكن تشغيلها على هذا الكمبيوتر.
core-policy-fit-architecture-not-told = تعذر تحديد بنية هذا الكمبيوتر، لذا لم يتم التحقق مما إذا كان يمكن تشغيل برامج هذا التطبيق عليه.
core-policy-fit-main-not-said = لا يذكر برنامج التثبيت هذا ما يحتاج إليه برنامجه الرئيسي، لذا فإن ما يُذكر هنا يستند إلى بقية ما يحتوي عليه.
# { $driver } is a driver's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-driver = برنامج التشغيل { $driver } الخاص به مصمم لبنية { $built_for }، وبنية هذا الكمبيوتر { $here }. يجب أن يطابق برنامج التشغيل Windows تمامًا، لذا لا يمكن تشغيل هذا التطبيق هنا
# { $program } is a program's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-program = { $program } مصمم لبنية { $built_for }، وبنية هذا الكمبيوتر { $here }، لذا لن يبدأ Windows تشغيله
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-none-start = لا يمكن بدء تشغيل أي من برامجه على هذا الكمبيوتر، الذي بنيته { $here }: { $programs }
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-some-other-architecture = بعض برامج هذا التطبيق مصممة لبنية أخرى ولن تبدأ على هذا الكمبيوتر، الذي بنيته { $here }: { $programs }. يمكن لبقية التطبيق أن تعمل، لذا قد لا يكون لهذا أهمية.
# { $here } is a kind of processor, such as "Arm64". { $programs } is a list of program names.
core-policy-fit-emulated = بنية هذا الكمبيوتر { $here }، لذا سيشغّل Windows هذه البرامج عبر محاكي: { $programs }. تعمل معظم البرامج جيدًا بهذه الطريقة، وبعضها لا يعمل.
# { $program } is a program's file name. { $needs } and { $runs } are versions of Windows, such as 10.0 and 6.1.
core-policy-fit-needs-newer = يذكر { $program } أنه يحتاج إلى Windows { $needs } أو أحدث، وهذا الكمبيوتر يعمل بالإصدار { $runs }، لذا لن يبدأ Windows تشغيله
# { $programs } is a list of program names.
core-policy-fit-some-need-newer = تذكر بعض برامج هذا التطبيق أنها تحتاج إلى إصدار Windows أحدث من هذا ولن تبدأ هنا: { $programs }.
# { $from_build } and { $this_build } are Windows build numbers, such as 22631, written as Windows writes them.
core-policy-fit-older-build = كان هذا التطبيق على Windows برقم الإصدار { $from_build }، وهذا الكمبيوتر يعمل برقم الإصدار { $this_build }. تذكر برامجه أنها يمكن أن تبدأ على إصدار Windows هذا، لذا قد يعمل، لكن ما كان يعتمد عليه هناك قد لا يكون موجودًا هنا.
core-policy-fit-build-not-told = تعذر تحديد رقم إصدار Windows هذا، لذا لم تتم مقارنته برقم الإصدار الذي كان عليه هذا التطبيق.

## Room on a drive for a build or an install.

# Each of these ends the sentence core-space-shortfall, after "free for".
core-space-for-unpacking = فك ضغط الحزمة
core-space-for-install-record = سجل التثبيت
core-space-for-app-files = ملفات التطبيق
core-space-for-what-is-replaced = الاحتفاظ بما يستبدله
core-space-for-build-work = ما تعمل عليه عملية الإنشاء
core-space-for-installer = برنامج التثبيت
core-space-for-package = الحزمة
core-space-for-unpacking-and-installing = فك ضغط الحزمة وتثبيتها
# What goes between two of those, as in "unpacking the package and the app's files". Keep the quotes: they keep the
# spaces.
core-space-purposes-joiner = { " و" }
# { $drive } is a drive, such as C:, or a folder. { $needed } and { $free } are sizes, such as 1.2 GB. { $purposes } is
# what the room is for, such as "unpacking the package and the app's files".
core-space-shortfall = يحتاج { $drive } إلى مساحة حرة تصل إلى { $needed } من أجل { $purposes }، والمتوفر { $free }
# { $drive } is a drive, such as C:, or a folder. { $needed } is a size, such as 1.2 GB.
core-space-not-told = لم يذكر { $drive } مقدار المساحة المتوفرة فيه، لذا لم يتم التحقق مما إذا كانت المساحة التي يحتاج إليها ({ $needed }) كافية
# One drive in a list of where room is needed. { $needed } and { $free } are sizes, and { $drive } a drive, such as C:.
core-space-summary = حتى { $needed } على { $drive } ({ $free } متوفرة)
# { $needed } is a size, and { $drive } a drive, such as C:.
core-space-summary-not-told = حتى { $needed } على { $drive } (تعذر تحديد المساحة المتوفرة)
# What goes between two sentences of core-space-shortfall inside core-review-no-room. Keep the quotes: they keep the
# space.
core-review-sentence-joiner = { ". " }
# { $shortfalls } is one or more sentences of core-space-shortfall.
core-review-no-room = لا توجد مساحة كافية لهذا، لذا لم يتم تغيير أي شيء. { $shortfalls }. حرر بعض المساحة هناك، أو اختر موقع تثبيت على محرك أقراص آخر.

## The installer's list of what an app sets up beyond its files: what each kind is called.

core-review-kind-service = خدمة
core-review-kind-driver = برنامج تشغيل
core-review-kind-scheduled-task = مهمة مجدولة
core-review-kind-autostart = يبدأ تلقائيًا
core-review-kind-shortcut = اختصار
core-review-kind-path = PATH
core-review-kind-environment-variable = متغير البيئة
core-review-kind-shell-extension = ملحق Shell
core-review-kind-menu-command = أمر قائمة
core-review-kind-link-type = نوع الارتباط
core-review-kind-file-type = نوع الملف
core-review-kind-com = COM
core-review-kind-firewall-rule = قاعدة جدار الحماية
core-review-kind-event-log-source = مصدر سجل الأحداث
core-review-kind-font = خط
core-review-kind-odbc-driver = برنامج تشغيل ODBC
core-review-kind-odbc-source = مصدر بيانات ODBC
core-review-kind-compatibility-setting = إعداد التوافق
core-review-kind-default-apps = التطبيقات الافتراضية
core-review-kind-program-name = اسم البرنامج
core-review-kind-other-settings = إعدادات أخرى

## The installer's list of what an app sets up: what each line says of it.

# { $file } is a program's file name. { $account } is the Windows account it runs as, such as LocalSystem. { $starts }
# says when it starts, and is not shown itself; for "other", { $start } is one of the core-review-start- words below.
core-review-service-detail =
    { $starts ->
        [automatic] تشغّل { $file } باسم { $account }، وتبدأ الآن وفي كل مرة يبدأ فيها تشغيل هذا الكمبيوتر
        [delayed] تشغّل { $file } باسم { $account }، وتبدأ الآن وبعد فترة وجيزة من كل بدء تشغيل لهذا الكمبيوتر
        [manual] تشغّل { $file } باسم { $account }، ولا تبدأ إلا عندما يطلب منها شيء ذلك
        [disabled] تشغّل { $file } باسم { $account }، وهي معطلة
        [other] تشغّل { $file } باسم { $account }، ويكون بدء تشغيلها { $start }
       *[unknown] تشغّل { $file } باسم { $account }
    }
# { $account } is a Windows account, such as LocalService.
core-policy-account-custom = { $account } (لم تتم استعادة الحساب المخصص)
# { $file } is a driver's file name.
core-review-driver-detail = يحمّل { $file } في Windows نفسه
# { $file } is a driver's file name. { $start } is when Windows starts it: one of the core-review-start- words below.
core-review-driver-detail-starts = يحمّل { $file } في Windows نفسه، ويكون بدء تشغيله { $start }
# When a service or driver starts, ending a sentence such as "Loads disk.sys into Windows itself, starts …".
core-review-start-boot = مع Windows، قبل أي شيء آخر
core-review-start-system = مع Windows، أثناء بدء تشغيله
core-review-start-automatic = في كل مرة يبدأ فيها تشغيل هذا الكمبيوتر
core-review-start-delayed = بعد فترة وجيزة من كل بدء تشغيل لهذا الكمبيوتر
core-review-start-manual = فقط عندما يطلب شيء ذلك
core-review-start-disabled = معطّلًا، فلا يحدث أبدًا
core-review-start-unknown = بطريقة لا يذكرها Windows
# { $account } is the account a task runs as: an account's name, or core-review-task-runs-as-you, or
# core-review-task-runs-as-creator.
core-review-task-detail = تعمل باسم { $account }
# { $account } is the account a task runs as, as for core-review-task-detail.
core-review-task-detail-highest = تعمل باسم { $account }، بأعلى الامتيازات الممكنة
core-review-task-runs-as-you = حسابك، أي حساب من سجّل الدخول
core-review-task-runs-as-creator = المستخدم الذي أنشأها
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-autostart-detail =
    يعمل { $scope ->
        [everyone] عند تسجيل دخول أي مستخدم
       *[you] عند تسجيل دخولك
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shortcut-startup-detail =
    اختصار في مجلد بدء التشغيل، { $scope ->
        [everyone] لكل مستخدمي هذا الكمبيوتر
       *[you] لك
    }
# { $file } is the file name of what the shortcut opens. { $scope } is whom it is set up for: everyone on this PC,
# or you. It is not shown itself.
core-review-shortcut-detail =
    يفتح { $file }، { $scope ->
        [everyone] لكل مستخدمي هذا الكمبيوتر
       *[you] لك
    }
# { $name } is an environment variable's name, such as PATH.
core-review-path-title = يضيف إلى { $name }
core-review-path-install-folder = مجلد التثبيت
# { $path } is a folder inside the install folder, such as bin\tools.
core-review-path-below-install-folder = مجلد التثبيت\{ $path }
# { $value } is the variable's value, as it will be set.
core-review-variable-detail = يتم تعيينه إلى { $value }
# { $count } is how many registry values.
core-review-other-settings-title =
    { $count ->
        [zero] { $count } إعداد خارج مفاتيح التطبيق نفسه
        [one] إعداد واحد خارج مفاتيح التطبيق نفسه
        [two] إعدادان خارج مفاتيح التطبيق نفسه
        [few] { $count } إعدادات خارج مفاتيح التطبيق نفسه
        [many] { $count } إعدادًا خارج مفاتيح التطبيق نفسه
       *[other] { $count } إعداد خارج مفاتيح التطبيق نفسه
    }
# { $keys } is a list of registry keys, such as HKCU\Software\Vendor.
core-review-other-settings-detail = في { $keys }
# { $keys } is a list of registry keys, and { $count } how many more there are.
core-review-other-settings-detail-more = في { $keys }، و{ $count } غيرها
# { $way } says whether the rule allows or blocks, and incoming or outgoing connections; it is not shown itself.
# { $ports } is a list of port numbers, and { $port_count } how many. { $program } is the program's file name.
# { $networks } is a list of the kinds of network Windows names in English (domain, private, public), and
# { $network_count } how many.
# The kinds of network a firewall rule is for, as Windows Defender Firewall names its profiles in your language. They
# fill { $networks } in core-review-firewall-detail, joined into a list.
core-review-network-domain = شبكة المجال
core-review-network-private = الشبكة الخاصة
core-review-network-public = الشبكة العامة
core-review-firewall-detail =
    { $way ->
        [allow-in] يسمح بالاتصالات الواردة
        [allow-out] يسمح بالاتصالات الصادرة
        [block-in] يحظر الاتصالات الواردة
       *[block-out] يحظر الاتصالات الصادرة
    } { $port_count ->
        [0] للبرنامج { $program }
        [zero] للبرنامج { $program }
        [one] على المنفذ { $ports } للبرنامج { $program }
        [two] على المنفذين { $ports } للبرنامج { $program }
        [few] على المنافذ { $ports } للبرنامج { $program }
        [many] على المنافذ { $ports } للبرنامج { $program }
       *[other] على المنافذ { $ports } للبرنامج { $program }
    }، { $network_count ->
        [0] على كل الشبكات
        [zero] على كل الشبكات
        [one] على { $networks }
        [two] على { $networks }
        [few] على { $networks }
        [many] على { $networks }
       *[other] على { $networks }
    }
# { $server } is the file name of what Explorer loads. { $scope } is whom it is set up for: everyone on this PC, or
# you. It is not shown itself.
core-review-shell-extension-detail =
    يحمّل { $server } في مستكشف الملفات، { $scope ->
        [everyone] لكل مستخدمي هذا الكمبيوتر
       *[you] لك
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shell-extension-detail-own-file =
    يحمّل في مستكشف الملفات ملفًا يثبّته، { $scope ->
        [everyone] لكل مستخدمي هذا الكمبيوتر
       *[you] لك
    }
# The name of a shell extension: { $handler } is its name in the registry, and { $class } the kind of file or folder it
# is for, such as * or Directory.
core-policy-shell-extension-name = { $handler } على { $class }
# { $on } is the kind of file or folder whose menu it is, as the registry names it, such as * or
# Directory. { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-menu-command-detail =
    يضيف أمرًا إلى قائمة { $on }، { $scope ->
        [everyone] لكل مستخدمي هذا الكمبيوتر
       *[you] لك
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-default-verb-detail =
    يغيّر ما يحدث عند فتح أحد هذه العناصر، { $scope ->
        [everyone] لكل مستخدمي هذا الكمبيوتر
       *[you] لك
    }
core-review-client-entry-detail = يمكن اختياره كبرنامج لهذا الغرض. لا يتم تحويل أي شيء إليه
core-review-com-permissions-title = أذونات COM
core-review-com-permissions-detail = يحدد من يمكنه بدء فئات COM الخاصة به، أو الحساب الذي تعمل باسمه
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-link-type-detail =
    تُفتح الارتباطات من هذا النوع باستخدام التطبيق، { $scope ->
        [everyone] لكل مستخدمي هذا الكمبيوتر
       *[you] لك
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-file-type-detail =
    تُفتح الملفات من هذا النوع باستخدام التطبيق، { $scope ->
        [everyone] لكل مستخدمي هذا الكمبيوتر
       *[you] لك
    }
# { $log } is the name of a Windows event log, such as Application.
core-review-event-log-source-detail = يمكنه الكتابة إلى سجل { $log }، برسائل من ملفاته الخاصة
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-font-detail =
    يُحمَّل من مجلد التثبيت عند تسجيل الدخول، { $scope ->
        [everyone] لكل مستخدمي هذا الكمبيوتر
       *[you] لك
    }
core-review-odbc-driver-detail = يُحمَّل في البرامج التي تستخدم ODBC للوصول إلى قاعدة بيانات
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-odbc-source-detail =
    اتصال بقاعدة بيانات، { $scope ->
        [everyone] لكل مستخدمي هذا الكمبيوتر
       *[you] لك
    }
core-review-default-apps-detail = يمكن اختياره في "التطبيقات الافتراضية". لا يتم تحويل أي شيء إليه
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-program-name-detail =
    يمكن لمربع الحوار "تشغيل" والبرامج الأخرى العثور عليه بهذا الاسم، { $scope ->
        [everyone] لكل مستخدمي هذا الكمبيوتر
       *[you] لك
    }
# { $server } is the file name of what serves the classes.
core-review-com-title = COM في { $server }
core-review-com-title-own-file = COM في ملف يثبّته
# { $count } is how many registry values.
core-review-com-detail =
    { $count ->
        [zero] { $count } إعداد للفئات التي يمكن للبرامج الأخرى بدؤها
        [one] إعداد واحد للفئات التي يمكن للبرامج الأخرى بدؤها
        [two] إعدادان للفئات التي يمكن للبرامج الأخرى بدؤها
        [few] { $count } إعدادات للفئات التي يمكن للبرامج الأخرى بدؤها
        [many] { $count } إعدادًا للفئات التي يمكن للبرامج الأخرى بدؤها
       *[other] { $count } إعداد للفئات التي يمكن للبرامج الأخرى بدؤها
    }
core-review-com-support-title = مكتبات أنواع COM وواجهاتها ومعرّفات AppID
# { $libraries }, { $interfaces }, and { $app_ids } are how many registry values there are for its type libraries, its
# interfaces, and its AppIDs.
core-review-com-support-detail = إعدادات ترافق فئاته: { $libraries } لمكتبات الأنواع، و{ $interfaces } للواجهات، و{ $app_ids } لمعرّفات AppID
core-review-compatibility-title = كيفية بدء برامجه
# { $count } is how many registry values.
core-review-compatibility-detail =
    { $count ->
        [zero] { $count } إعداد توافق للبرامج التي يثبّتها
        [one] إعداد توافق واحد للبرامج التي يثبّتها
        [two] إعدادا توافق للبرامج التي يثبّتها
        [few] { $count } إعدادات توافق للبرامج التي يثبّتها
        [many] { $count } إعداد توافق للبرامج التي يثبّتها
       *[other] { $count } إعداد توافق للبرامج التي يثبّتها
    }

## The installer's notes on whose folders and settings these are, and what is not coming.

# { $account } is the name of a Windows account, such as alice.
core-review-account-signed-in = تذهب بيانات المستخدم والإعدادات الخاصة بكل مستخدم إلى { $account }، الذي سجّل الدخول هنا، وليس إلى المسؤول الذي يعمل هذا باسمه.
# { $account } is the name of a Windows account, such as alice.
core-review-account-no-desktop = لم يسجّل أحد الدخول إلى هذه الجلسة، لذا تذهب بيانات المستخدم والإعدادات الخاصة بكل مستخدم إلى { $account }، وهو الحساب الذي يعمل هذا باسمه.
# { $reason } is what Windows said, in its own words.
core-review-account-not-told = تعذر تحديد من سجّل الدخول ({ $reason })، لذا تم استبعاد الإعدادات الخاصة بكل مستخدم، وتذهب بيانات المستخدم إلى موقع التثبيت.
# { $reason } is what Windows said, in its own words.
core-review-signed-in-unknown = تعذر تحديد من سجّل الدخول: { $reason }
# Headings of what is left out, shared by the app's list and the installer's notes.
core-review-left-out-credentials = غير مضمّنة في برنامج التثبيت هذا، لأنها تبدو كبيانات اعتماد
core-review-left-out-settings = الإعدادات والمحفوظات التي يحتفظ بها حساب للتطبيق، والتي لا تُضمّن إلا مع خيار "بيانات المستخدم"
core-review-left-out-certificates = يبدو أن التطبيق يعتمد على شهادات كانت موثوقة على الكمبيوتر الذي كان عليه، ولم يتم تثبيتها
core-review-left-out-fonts = تحمل هذه الخطوط اسم التطبيق، ولا شيء يدل على أنها خاصة به، لذا لم يتم تثبيتها
# { $names } is a list of names, such as the names of saved passwords.
core-review-note-credentials = غير مضمّنة في برنامج التثبيت هذا، لأنها تبدو كبيانات اعتماد: { $names }. يبقى ما يوجد منها على هذا الكمبيوتر كما هو، وقد تحتاج إلى إدخالها مرة أخرى.
# { $names } is a list of names of license data.
core-review-note-licenses = غير مضمّنة في برنامج التثبيت هذا، لأن خيار "بيانات الترخيص والتنشيط" كان متوقفًا عند إنشائه: { $names }.
# { $names } is a list of places an account keeps settings, such as folders.
core-review-note-settings = غير مضمّنة في برنامج التثبيت هذا، لأن خيار "بيانات المستخدم" كان متوقفًا عند إنشائه: الإعدادات والمحفوظات التي يحتفظ بها حساب للتطبيق، في { $names }. يبدأ التطبيق هنا كما لو كان يُستخدم لأول مرة.
# { $names } is a list of certificates, each as core-review-certificate.
core-review-note-certificates = يبدو أن التطبيق يعتمد على شهادات كانت موثوقة على الكمبيوتر الذي كان عليه، ولم يتم تثبيتها: { $names }. القرار لك في ما إذا كنت ستثق بها هنا.
# { $names } is a list of font names.
core-review-note-fonts = تحمل هذه الخطوط اسم التطبيق، ولا شيء يدل على أنها خاصة به، لذا لم يتم تثبيتها: { $names }.
# { $names } is a list of names.
core-review-note-left-out-by-choice = استبعدها من أنشأ برنامج التثبيت هذا: { $names }.
# { $names } is a list of names.
core-review-note-added-by-choice = أضافها من أنشأ برنامج التثبيت هذا، لأنه اعتبرها جزءًا من التطبيق: { $names }.
# { $name } is a certificate's subject, such as CN=Contoso, and { $store } the certificate store it was in, such as
# Root.
core-review-certificate = { $name } في { $store }
# { $names } is a list of names, and { $count } how many more there are.
core-review-names-and-more = { $names }، و{ $count } غيرها

## Why something in a package is left out. Each is the second half of a sentence of core-warning-review-left-out.

core-review-no-account = ينتمي إلى إعدادات حساب واحد، وتعذر تحديد من سجّل الدخول
core-review-name-not-accepted = لن يقبل Windows اسمه
# { $error } is what Windows said, in its own words.
core-review-service-unreadable = تعذرت قراءة إعداداتها الحالية: { $error }
core-review-service-dependency = تعتمد على خدمة ذات اسم غير صالح
# { $path } is the program the service runs.
core-review-service-runs-other = تشغّل { $path }، الذي لا تثبّته هذه الحزمة
core-review-service-settings = تذكر إعداداتها شيئًا لا تثبّته هذه الحزمة، أو مفتاحًا ليس خاصًا بها
core-review-driver-no-hash = ليس له تجزئة للتحقق منه بها
core-review-license-for-the-machine = الترخيص المخزن على مستوى الجهاز بأكمله يتم الإبلاغ عنه فقط
core-review-license-starts-something = لا يوجد سبب يدعو الترخيص إلى بدء تشغيل أي شيء
core-review-task-not-captured = لم يتم التقاط تعريفها
# { $error } is what was wrong with the task's XML, in English.
core-review-task-unreadable = تعذرت قراءة تعريفها: { $error }
core-review-task-no-account = تعمل باسم من يثبّتها، وتعذر تحديد من سجّل الدخول هنا
# { $path } is what the shortcut opens.
core-review-shortcut-opens-other = يفتح { $path }، الذي لا تثبّته هذه الحزمة
# { $service } is the service's name, and { $path } the program it runs now.
core-policy-service-exists = { $service } موجودة بالفعل على هذا الكمبيوتر وتشغّل { $path }، وهو ليس من تثبيت سابق هنا. إذا كان نسخة أقدم من هذا التطبيق، فألغِ تثبيتها أولاً
core-policy-key-control-characters = يحتوي اسم مفتاحه على أحرف تحكم
core-policy-value-control-characters = يحتوي اسم قيمته على أحرف تحكم
core-policy-registry-link = إنه ارتباط في السجل
# { $reason } is why the key it stands for is left out, such as "no app install should change this key".
core-policy-virtual-store = يمثل مفتاحًا خاصًا بالكمبيوتر، و{ $reason }
core-policy-registry-part = لا ينبغي لأي تثبيت تطبيق الكتابة في هذا الجزء من السجل
core-policy-key-refused = لا ينبغي لأي تثبيت تطبيق تغيير هذا المفتاح
core-policy-net-debugger = سيحدد مصحح أخطاء لكل برامج .NET
core-policy-net-install-root = سيغيّر المكان الذي تحمّل منه كل برامج .NET مكتبات .NET
core-policy-class-redirects = سيغيّر طريقة فتح نوع من البرامج أو المجلدات
core-policy-app-path-name = يسمّي برنامجًا بشيء غير اسم ملف
core-policy-app-path-not-installed = سيسجّل اسم برنامج لملف لا تثبّته هذه الحزمة
core-policy-shared-default-verb = سيغيّر ما يحدث عند فتح أي ملف أو مجلد من نوع ما
core-policy-command-runs-other = يضيف أمرًا يشغّل شيئًا لا تثبّته هذه الحزمة
core-policy-autostart-not-command = يبدأ تلقائيًا، وقيمته ليست أمرًا
core-policy-autostart-runs-other = يبدأ تلقائيًا، ويشغّل شيئًا لا تثبّته هذه الحزمة
core-policy-environment-own-rules = تتم استعادة متغيرات البيئة وفقًا لقواعدها الخاصة
core-policy-service-not-restored = ينتمي إلى خدمة لا تستعيدها هذه الحزمة
core-policy-service-own-rules = لا تتم استعادة الإعدادات الخاصة بالخدمة إلا وفقًا لقواعدها
core-policy-service-dll = قيمة ServiceDll الخاصة به ليست ملفًا تثبّته هذه الحزمة
core-policy-event-log-source-only = لا يمكن استعادة سوى القيم الخاصة بمصدر سجل الأحداث
core-policy-security-log = لا يكتب أي تطبيق إلى سجل الأمان
core-policy-event-log-messages = ستأتي رسائله من ملف لا تثبّته هذه الحزمة
core-policy-type-library = مكتبة الأنواع الخاصة به ليست ملفًا تثبّته هذه الحزمة
# { $name } is a program's file name, such as app.exe.
core-policy-application-opened = سيغيّر طريقة فتح { $name }، الذي لا تثبّته هذه الحزمة
core-policy-event-log-no-file = يجب أن يأخذ مصدر سجل الأحداث رسائله من ملف
core-policy-com-runs-other = يشغّل شيئًا ليس ملفًا تثبّته هذه الحزمة
core-policy-com-redirect = سيجعل هذه الفئة شيئًا لا تثبّته هذه الحزمة
core-policy-com-server = يشغّل شيئًا ليس ملفًا تثبّته هذه الحزمة ولا جزءًا من Windows
core-policy-interface = سيحدد طريقة إجراء الاستدعاءات إلى واجهة، باستخدام شيء لا تثبّته هذه الحزمة
# { $name } is a program's file name, such as app.exe.
core-policy-app-id-started = سيغيّر طريقة بدء { $name }، الذي لا تثبّته هذه الحزمة
core-policy-app-id-surrogate = ستتم استضافته بواسطة برنامج لا تثبّته هذه الحزمة
core-policy-app-id-service = سيعمل في خدمة لا تستعيدها هذه الحزمة
core-policy-app-id-run-as = سيعمل باسم حساب خاص به
core-policy-app-id-link = لا يسمّي AppID
core-policy-app-id-remote = سيعمل في مكان غير هذا الكمبيوتر
core-policy-odbc-own-driver = هذا إعداد خاص بـ ODBC نفسه وليس برنامج تشغيل
core-policy-odbc-driver-not-installed = برنامج التشغيل الخاص به ليس ملفًا تثبّته هذه الحزمة
core-policy-loads-other-file = سيحمّل ملفًا لا تثبّته هذه الحزمة
core-policy-odbc-own-source = هذا إعداد خاص بـ ODBC نفسه وليس مصدر بيانات
core-policy-odbc-source-missing = يسرد مصدر بيانات غير موجود في هذه الحزمة
core-policy-odbc-source-driver = برنامج التشغيل الخاص به ليس ملفًا تثبّته هذه الحزمة ولا جزءًا من Windows
core-policy-shell-extension = سيحمّل في مستكشف الملفات ملحق Shell ليس ملفًا تثبّته هذه الحزمة
core-policy-not-text = محتواه ليس نصًا صالحًا بالشكل الذي سيُقرأ به
core-policy-network-place = يسمّي مكانًا على الشبكة
core-policy-font = سيسجّل خطًا لا تثبّته هذه الحزمة، أو خطًا موجودًا في مجلد Windows
core-policy-compatibility = سيغيّر طريقة تشغيل برنامج لا تثبّته هذه الحزمة
core-policy-capabilities = يسرد تطبيقًا ليست تفاصيله في هذه الحزمة
core-policy-default-program = سيغيّر البرنامج الذي يفتح العناصر بشكل افتراضي
core-policy-url-protocol = يذكر أنه نوع ارتباط، ولا يُفتح بأي شيء تثبّته هذه الحزمة
core-policy-command-handler = يضيف أمرًا يعالجه شيء لا تثبّته هذه الحزمة
core-policy-installer-command = سيشغّل شيئًا لا يستطيع قراءته سوى Windows Installer
core-policy-creates-class = سينشئ فئة لا تثبّتها هذه الحزمة
core-policy-opens-links = يفتح الارتباطات بشيء لا تثبّته هذه الحزمة
core-policy-opens-files = يفتح الملفات بشيء لا تثبّته هذه الحزمة
core-policy-firewall-name = اسمها لا يمكن نقله
core-policy-firewall-description = وصفها لا يمكن نقله
core-policy-firewall-every-program = تنطبق على كل البرامج، والقاعدة لا يمكن أن تكون إلا لبرنامج واحد
# { $path } is the program the rule is for.
core-policy-firewall-program = إنها مخصصة لـ { $path }، الذي لا تثبّته هذه الحزمة
core-policy-firewall-direction = لا تذكر اتجاهًا يمكن تعيينه
core-policy-firewall-action = ستتجاوز قواعد أخرى، وهذا ما لا ينبغي لقاعدة أي تطبيق فعله
# { $setting } is a part of a firewall rule, as the scan of its PC described it.
core-policy-firewall-not-carried = تذكر { $setting }، وتعذر تعيين ذلك كما كان، أو كان سيُضعف ما تفعله القواعد الأخرى
core-policy-firewall-service = إنها مخصصة لخدمة لا تستعيدها هذه الحزمة
core-policy-firewall-parts = جزء منها ليس مما يمكن أن تذكره قاعدة جدار الحماية
core-policy-folder-already-there = كان ذلك المجلد موجودًا بالفعل، خارج موقع التثبيت، ولا شيء يدل على أنه خاص بهذا التطبيق، لذا لا يوضع فيه أي برنامج
core-policy-variable-name = اسمه ليس اسم متغير صالحًا
core-policy-variable-relied-on = يعتمد Windows والبرامج الأخرى على هذا المتغير
core-policy-task-name = اسمها ليس اسمًا يقبله Windows
core-policy-task-folder = مجلدها ليس مجلدًا يقبله Windows
core-policy-task-windows = ستستبدل إحدى مهام Windows الخاصة
core-policy-task-other-action = تفعل شيئًا غير تشغيل برنامج
core-policy-task-no-program = لا تسمّي أي برنامج لتشغيله
# { $path } is the program the task runs.
core-policy-task-runs-other = تشغّل شيئًا لا تثبّته هذه الحزمة: { $path }
core-policy-task-exists-unreadable = توجد بالفعل مهمة بهذا الاسم وتعذرت قراءتها
core-policy-task-exists-other = توجد بالفعل مهمة بهذا الاسم ولا تشغّل التطبيق
# { $path } is the program the task already on this PC runs.
core-policy-task-exists-runs = توجد بالفعل مهمة بهذا الاسم وتشغّل { $path }، وهو ليس من تثبيت سابق هنا
core-policy-shortcut-not-lnk = ليس اختصار .lnk
core-policy-shortcut-place = ليس في قائمة "ابدأ" أو على سطح المكتب
core-policy-short-name = يستخدم مساره اسمًا لا يمكن التحقق منه
core-policy-shortcut-no-folder = لا يوجد على هذا الكمبيوتر مجلد كهذا له
core-policy-data-startup = سيوضع في مجلد بدء التشغيل، ويعمل عند كل تسجيل دخول
core-policy-data-powershell = سيوضع حيث يشغّل PowerShell ما يجده عند بدء تشغيله
core-policy-data-windows = سيوضع في مجلد يخص Windows
core-policy-license-outside = إنه خارج موقع التثبيت وكل مجلدات البيانات المعروفة
core-rewrite-cdata = يحتوي على قسم CDATA
core-rewrite-too-much-to-look-through = تحتوي إعدادات هذه الحزمة على أكثر مما يمكن فحصه للتحقق مما تسمّيه

## Warnings a review gives, which the installer shows with what it sets up.

# { $kind } says what was left out, and is not shown itself. { $item } is its path or name, or a registry value such
# as HKLM\Software\App\Name. { $reason } is why, as the second half of the sentence, such as "it cannot be a whole
# drive".
core-warning-review-left-out =
    { $kind ->
        [file] تم استبعاد الملف { $item }: { $reason }.
        [folder] تم استبعاد المجلد { $item }: { $reason }.
        [service] تم استبعاد الخدمة { $item }: { $reason }.
        [driver] تم استبعاد برنامج التشغيل { $item }: { $reason }.
        [firewall-rule] تم استبعاد قاعدة جدار الحماية { $item }: { $reason }.
        [license-data] تم استبعاد بيانات الترخيص الموجودة في { $item }: { $reason }.
        [license-value] تم استبعاد قيمة الترخيص { $item }: { $reason }.
        [scheduled-task] تم استبعاد المهمة المجدولة { $item }: { $reason }.
        [shortcut] تم استبعاد الاختصار { $item }: { $reason }.
        [environment-variable] تم استبعاد متغير البيئة { $item }: { $reason }.
       *[value] تم استبعاد { $item }: { $reason }.
    }
core-warning-review-package-path-missing = ليس لملف قابل للنسخ مسار نسبي في الحزمة.
# { $count } is the most empty folders a package makes.
core-warning-review-too-many-empty-folders = تسرد هذه الحزمة أكثر من { $count } من المجلدات الفارغة، ولا يتم إنشاء سوى هذا العدد منها.
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Documents folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-documents-elsewhere =
    { $count ->
        [zero] يذهب { $count } ملف من "المستندات" إلى { $target } بدلاً من ذلك، في مجلد باسم _external، لأن مجلد "المستندات" على هذا الكمبيوتر موجود في { $folder }، ولا تتم الكتابة إلا في مجلد داخل ملف التعريف الخاص بك. لاستخدامها، انقلها إلى مجلد "المستندات" بنفسك.
        [one] يذهب ملف واحد من "المستندات" إلى { $target } بدلاً من ذلك، في مجلد باسم _external، لأن مجلد "المستندات" على هذا الكمبيوتر موجود في { $folder }، ولا تتم الكتابة إلا في مجلد داخل ملف التعريف الخاص بك. لاستخدامه، انقله إلى مجلد "المستندات" بنفسك.
        [two] يذهب ملفان من "المستندات" إلى { $target } بدلاً من ذلك، في مجلد باسم _external، لأن مجلد "المستندات" على هذا الكمبيوتر موجود في { $folder }، ولا تتم الكتابة إلا في مجلد داخل ملف التعريف الخاص بك. لاستخدامهما، انقلهما إلى مجلد "المستندات" بنفسك.
        [few] يذهب { $count } ملفات من "المستندات" إلى { $target } بدلاً من ذلك، في مجلد باسم _external، لأن مجلد "المستندات" على هذا الكمبيوتر موجود في { $folder }، ولا تتم الكتابة إلا في مجلد داخل ملف التعريف الخاص بك. لاستخدامها، انقلها إلى مجلد "المستندات" بنفسك.
        [many] يذهب { $count } ملفًا من "المستندات" إلى { $target } بدلاً من ذلك، في مجلد باسم _external، لأن مجلد "المستندات" على هذا الكمبيوتر موجود في { $folder }، ولا تتم الكتابة إلا في مجلد داخل ملف التعريف الخاص بك. لاستخدامها، انقلها إلى مجلد "المستندات" بنفسك.
       *[other] يذهب { $count } ملف من "المستندات" إلى { $target } بدلاً من ذلك، في مجلد باسم _external، لأن مجلد "المستندات" على هذا الكمبيوتر موجود في { $folder }، ولا تتم الكتابة إلا في مجلد داخل ملف التعريف الخاص بك. لاستخدامها، انقلها إلى مجلد "المستندات" بنفسك.
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-documents-unknown =
    { $count ->
        [zero] يذهب { $count } ملف من "المستندات" إلى { $target } بدلاً من ذلك، في مجلد باسم _external، لأنه تعذر تحديد موقع مجلد "المستندات" على هذا الكمبيوتر. لاستخدامها، انقلها إلى مجلد "المستندات" بنفسك.
        [one] يذهب ملف واحد من "المستندات" إلى { $target } بدلاً من ذلك، في مجلد باسم _external، لأنه تعذر تحديد موقع مجلد "المستندات" على هذا الكمبيوتر. لاستخدامه، انقله إلى مجلد "المستندات" بنفسك.
        [two] يذهب ملفان من "المستندات" إلى { $target } بدلاً من ذلك، في مجلد باسم _external، لأنه تعذر تحديد موقع مجلد "المستندات" على هذا الكمبيوتر. لاستخدامهما، انقلهما إلى مجلد "المستندات" بنفسك.
        [few] يذهب { $count } ملفات من "المستندات" إلى { $target } بدلاً من ذلك، في مجلد باسم _external، لأنه تعذر تحديد موقع مجلد "المستندات" على هذا الكمبيوتر. لاستخدامها، انقلها إلى مجلد "المستندات" بنفسك.
        [many] يذهب { $count } ملفًا من "المستندات" إلى { $target } بدلاً من ذلك، في مجلد باسم _external، لأنه تعذر تحديد موقع مجلد "المستندات" على هذا الكمبيوتر. لاستخدامها، انقلها إلى مجلد "المستندات" بنفسك.
       *[other] يذهب { $count } ملف من "المستندات" إلى { $target } بدلاً من ذلك، في مجلد باسم _external، لأنه تعذر تحديد موقع مجلد "المستندات" على هذا الكمبيوتر. لاستخدامها، انقلها إلى مجلد "المستندات" بنفسك.
    }
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Saved Games folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-elsewhere =
    { $count ->
        [zero] يذهب { $count } ملف من "الألعاب المحفوظة" إلى { $target } بدلاً من ذلك، في مجلد باسم _external، لأن مجلد "الألعاب المحفوظة" على هذا الكمبيوتر موجود في { $folder }، ولا تتم الكتابة إلا في مجلد داخل ملف التعريف الخاص بك. لاستخدامها، انقلها إلى مجلد "الألعاب المحفوظة" بنفسك.
        [one] يذهب ملف واحد من "الألعاب المحفوظة" إلى { $target } بدلاً من ذلك، في مجلد باسم _external، لأن مجلد "الألعاب المحفوظة" على هذا الكمبيوتر موجود في { $folder }، ولا تتم الكتابة إلا في مجلد داخل ملف التعريف الخاص بك. لاستخدامه، انقله إلى مجلد "الألعاب المحفوظة" بنفسك.
        [two] يذهب ملفان من "الألعاب المحفوظة" إلى { $target } بدلاً من ذلك، في مجلد باسم _external، لأن مجلد "الألعاب المحفوظة" على هذا الكمبيوتر موجود في { $folder }، ولا تتم الكتابة إلا في مجلد داخل ملف التعريف الخاص بك. لاستخدامهما، انقلهما إلى مجلد "الألعاب المحفوظة" بنفسك.
        [few] يذهب { $count } ملفات من "الألعاب المحفوظة" إلى { $target } بدلاً من ذلك، في مجلد باسم _external، لأن مجلد "الألعاب المحفوظة" على هذا الكمبيوتر موجود في { $folder }، ولا تتم الكتابة إلا في مجلد داخل ملف التعريف الخاص بك. لاستخدامها، انقلها إلى مجلد "الألعاب المحفوظة" بنفسك.
        [many] يذهب { $count } ملفًا من "الألعاب المحفوظة" إلى { $target } بدلاً من ذلك، في مجلد باسم _external، لأن مجلد "الألعاب المحفوظة" على هذا الكمبيوتر موجود في { $folder }، ولا تتم الكتابة إلا في مجلد داخل ملف التعريف الخاص بك. لاستخدامها، انقلها إلى مجلد "الألعاب المحفوظة" بنفسك.
       *[other] يذهب { $count } ملف من "الألعاب المحفوظة" إلى { $target } بدلاً من ذلك، في مجلد باسم _external، لأن مجلد "الألعاب المحفوظة" على هذا الكمبيوتر موجود في { $folder }، ولا تتم الكتابة إلا في مجلد داخل ملف التعريف الخاص بك. لاستخدامها، انقلها إلى مجلد "الألعاب المحفوظة" بنفسك.
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-unknown =
    { $count ->
        [zero] يذهب { $count } ملف من "الألعاب المحفوظة" إلى { $target } بدلاً من ذلك، في مجلد باسم _external، لأنه تعذر تحديد موقع مجلد "الألعاب المحفوظة" على هذا الكمبيوتر. لاستخدامها، انقلها إلى مجلد "الألعاب المحفوظة" بنفسك.
        [one] يذهب ملف واحد من "الألعاب المحفوظة" إلى { $target } بدلاً من ذلك، في مجلد باسم _external، لأنه تعذر تحديد موقع مجلد "الألعاب المحفوظة" على هذا الكمبيوتر. لاستخدامه، انقله إلى مجلد "الألعاب المحفوظة" بنفسك.
        [two] يذهب ملفان من "الألعاب المحفوظة" إلى { $target } بدلاً من ذلك، في مجلد باسم _external، لأنه تعذر تحديد موقع مجلد "الألعاب المحفوظة" على هذا الكمبيوتر. لاستخدامهما، انقلهما إلى مجلد "الألعاب المحفوظة" بنفسك.
        [few] يذهب { $count } ملفات من "الألعاب المحفوظة" إلى { $target } بدلاً من ذلك، في مجلد باسم _external، لأنه تعذر تحديد موقع مجلد "الألعاب المحفوظة" على هذا الكمبيوتر. لاستخدامها، انقلها إلى مجلد "الألعاب المحفوظة" بنفسك.
        [many] يذهب { $count } ملفًا من "الألعاب المحفوظة" إلى { $target } بدلاً من ذلك، في مجلد باسم _external، لأنه تعذر تحديد موقع مجلد "الألعاب المحفوظة" على هذا الكمبيوتر. لاستخدامها، انقلها إلى مجلد "الألعاب المحفوظة" بنفسك.
       *[other] يذهب { $count } ملف من "الألعاب المحفوظة" إلى { $target } بدلاً من ذلك، في مجلد باسم _external، لأنه تعذر تحديد موقع مجلد "الألعاب المحفوظة" على هذا الكمبيوتر. لاستخدامها، انقلها إلى مجلد "الألعاب المحفوظة" بنفسك.
    }

## Errors when a folder that only administrators may change cannot be made so, or checked.

# { $path } is a folder.
core-privilege-not-a-folder = { $path } ارتباط أو ملف، وليس مجلدًا
# { $path } is a folder. { $error } is what Windows said, in its own words.
core-privilege-not-made-here = لم يُنشئ WinAppMigrator المجلد { $path }، ويمكن للآخرين تغيير محتوياته. يجب أن يتحقق منه مسؤول ويزيله: { $error }
# { $path } is a folder.
core-privilege-others-put-something-in = يحتوي { $path } على شيء لم يُنشئه WinAppMigrator، ويمكن للآخرين تغييره. يجب أن يتحقق منه مسؤول ويزيله
# { $path } is a folder.
core-privilege-became-a-link-in-use = أصبح { $path } ارتباطًا أثناء استخدامه
# { $path } is a folder.
core-privilege-became-a-link-looked-at = أصبح { $path } ارتباطًا أثناء فحصه
# { $path } is a file. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-users-cannot-start = تعذر السماح للمستخدمين ببدء تشغيل { $path }: { $said }
core-privilege-elevation-unknown = تعذر تحديد ما إذا كانت هذه العملية تعمل بامتيازات مرتفعة
# { $path } is a folder. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-cannot-restrict = تعذر قصر الوصول إلى { $path } على المسؤولين: { $said }
# { $path } is a folder. { $said } is the folder's permissions as Windows wrote them, and what else it printed.
core-privilege-not-restricted = تعذر قصر الوصول إلى { $path } على المسؤولين، وقد لا يحتفظ محرك الأقراص الخاص به بالأذونات: { $said }
# { $path } is a folder.
core-privilege-written-to-first = كتبت عملية أخرى إلى { $path } قبل أن تتم حمايته
core-privilege-no-program-data = لا يذكر Windows مكان ProgramData

## Checks of a package's files, and of what an install put in place. Each is said of one file, value, service, or the
## like, which the report names beside it.

# { $path } is the installer's file.
core-verify-no-package = لا يحتوي { $path } على حزمة
# { $path } is a folder.
core-verify-not-unpacked = لم يتم فك ضغط حزمة برنامج التثبيت هذا إلى { $path }
core-verify-schema-supported = مخطط بيان الحزمة مدعوم.
core-verify-packaged-file-missing = الملف المضمّن في الحزمة مفقود.
core-verify-no-package-path = ليس للملف القابل للنسخ مسار نسبي في الحزمة.
core-verify-folder-there = المجلد الفارغ موجود.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-unreadable = تعذرت قراءته للتحقق منه: { $error }
core-verify-restored-file-missing = الملف المستعاد مفقود.
# { $expected } and { $actual } are sizes in bytes.
core-verify-size-mismatch = عدم تطابق حجم الملف. المتوقع { $expected } بايت، والموجود { $actual } بايت.
core-verify-no-hash = لا يسجل البيان أي تجزئة لهذا الملف، لذا لا يمكن التحقق منه.
core-verify-hash-mismatch = عدم تطابق تجزئة SHA-256.
core-verify-file-matches = يطابق حجم الملف وتجزئته ما في البيان.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-payload-failed = فشل التحقق من حمولة الترخيص: { $error }
core-verify-license-payload-missing = حمولة الترخيص المضمّنة في الحزمة مفقودة.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-failed = فشل التحقق من مخزن الترخيص: { $error }
core-verify-license-store-file-missing = ملف مخزن الترخيص المضمّن في الحزمة مفقود.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-path-invalid = مسار مخزن الترخيص المضمّن في الحزمة غير صالح: { $error }
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-prerequisite-failed = فشل التحقق من حمولة المتطلب الأساسي: { $error }
core-verify-prerequisite-missing = حمولة المتطلب الأساسي المجمّعة مفقودة.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-failed = فشل التحقق من صورة برنامج التشغيل: { $error }
core-verify-bundled-driver-missing = صورة برنامج التشغيل المجمّعة مفقودة.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-path-invalid = مسار برنامج التشغيل المجمّع غير صالح: { $error }
core-verify-prerequisite-present = المتطلب الأساسي لوقت التشغيل موجود على الجهاز الهدف.
core-verify-prerequisite-not-detected = لم يتم الكشف عن المتطلب الأساسي لوقت التشغيل؛ قد يتطلب إعادة التشغيل أو إكمالاً يدويًا (راجع سجل الاستعادة لمعرفة النتيجة المعتمدة).
core-verify-prerequisite-not-probed = لا يمكن التحقق من وجود المتطلب الأساسي عبر السجل؛ تم التثبيت بأفضل جهد ممكن.
core-verify-license-file-missing = ملف الترخيص المستعاد مفقود.
core-verify-license-store-restored-missing = ملف مخزن الترخيص المستعاد مفقود.
core-verify-registry-key-there = مفتاح السجل موجود.
core-verify-registry-value-matches = تطابق قيمة السجل البيانات المؤرشفة.
core-verify-registry-value-differs = لا تطابق قيمة السجل البيانات المؤرشفة.
core-verify-firewall-rule-there = يحتوي جدار حماية Windows على هذه القاعدة.
core-verify-variable-matches = تطابق قيمة متغير البيئة البيانات المؤرشفة.
core-verify-variable-differs = لا تطابق قيمة متغير البيئة البيانات المؤرشفة.
# { $error } is what Windows said, in its own words.
core-verify-variable-unreadable = متغير البيئة مفقود أو غير قابل للقراءة: { $error }
core-verify-service-runs-other = الخدمة موجودة ولكنها لا تشغّل الملف الثنائي المستعاد مع وسيطاته.
core-verify-service-other-account = الخدمة موجودة ولكنها لا تعمل باسم الحساب المتوقع.
core-verify-service-matches = الخدمة موجودة، وتشغّل الملف الثنائي المستعاد مع وسيطاته وحسابه.
core-verify-service-missing = الخدمة مفقودة.
# { $error } is what Windows said, in its own words.
core-verify-service-unreadable = تتعذر قراءة الخدمة: { $error }
core-verify-driver-other-image = لا تشير خدمة برنامج التشغيل إلى الصورة المستعادة.
core-verify-driver-not-registered = خدمة برنامج التشغيل المجمّعة غير مسجلة بعد الاستعادة.
# { $error } is what Windows said, in its own words.
core-verify-driver-unreadable = تتعذر قراءة خدمة برنامج التشغيل: { $error }
core-verify-driver-image-missing = صورة برنامج التشغيل مفقودة.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-unchecked = تعذر التحقق من صورة برنامج التشغيل: { $error }
core-verify-task-matches = المهمة المجدولة موجودة وتشغّل البرامج المستعادة.
core-verify-task-runs-other = المهمة المجدولة موجودة ولكنها لا تشغّل البرامج المستعادة.
core-verify-task-missing = المهمة المجدولة مفقودة.
# { $error } is what Windows said, in its own words.
core-verify-task-unreadable = فشل الاستعلام عن المهمة المجدولة: { $error }
core-verify-shortcut-matches = الاختصار موجود ومسار هدفه يطابق الهدف المستعاد.
core-verify-shortcut-opens-other = الاختصار موجود ولكن مسار هدفه لا يطابق الهدف المستعاد.
core-verify-shortcut-missing = الاختصار مفقود.
# { $error } is what Windows said, in its own words.
core-verify-shortcut-unreadable = فشل الاستعلام عن الاختصار: { $error }

## Warnings and errors a check gives, which the app and the installer show.

core-warning-verify-schema-unsupported = مخطط بيان الحزمة غير مدعوم.
# { $error } is what went wrong, in Windows' or the program's own words.
core-warning-verify-package-file-failed = فشل التحقق من ملف الحزمة: { $error }
core-warning-verify-folder-missing = المجلد الفارغ مفقود.
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-key-missing = مفتاح السجل مفقود أو غير قابل للقراءة: { $error }
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-value-missing = قيمة السجل مفقودة أو غير قابلة للقراءة: { $error }
core-warning-verify-registry-kind-missing = نوع قيمة السجل مفقود.
core-warning-verify-firewall-rule-missing = لا يحتوي جدار حماية Windows على هذه القاعدة.
core-warning-verify-service-scope = يقرأ التحقق من الخدمات تكوين الخدمات في HKLM ولا يبدأ تشغيل الخدمات.
core-warning-verify-driver-manual = لا يعيد WinAppMigrator توزيع برنامج تشغيل واحد أو أكثر من برامج التشغيل الخاصة بالمورّدين أو المضمّنة في Windows؛ أعد تثبيتها على الجهاز الهدف باستخدام حزمة برنامج التشغيل الموقّعة من المورّد.
core-warning-verify-task-scope = يقتصر التحقق من المهام المجدولة على التسجيل ومرجع الأمر.

## The verification report written beside a package, verification_report.md. Its headings follow a "#".

core-verify-report-title = تقرير التحقق من WinAppMigrator
# { $time } is when, as a date and time in UTC, such as 2026-11-15T10:00:00Z.
core-verify-report-verified = وقت التحقق (UTC): { $time }
# { $valid } is true or false, which are not translated.
core-verify-report-package-valid = الحزمة صالحة: { $valid }
# { $valid } is true or false, which are not translated.
core-verify-report-restore-valid = الاستعادة صالحة: { $valid }
core-verify-report-restore-not-run = الاستعادة صالحة: لم يتم التشغيل
core-verify-report-items = العناصر
# { $message } is what a check found, and { $source } the file or setting it looked at.
core-verify-report-item-from = { $message } المصدر: { $source }
core-verify-report-warnings = التحذيرات
core-verify-report-errors = الأخطاء
core-verify-report-none = لا شيء.
