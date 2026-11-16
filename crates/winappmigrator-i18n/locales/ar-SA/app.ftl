# Words of the app, WinAppMigrator itself: its one window, with the installed apps listed on the left and, on the
# right, what goes in an installer of the one picked and the button that builds it; and the steps to read another PC
# through WinAppMigrator Client. Message names start with app-.


## The window and its header

# The program's name, in the window's title bar and at the top of the window. Keep it as it is unless your language
# writes product names in a script of its own.
app-name = WinAppMigrator
app-tagline = تحويل تطبيق مثبت إلى برنامج تثبيت ذاتي الاستخراج
app-theme-to-light = التبديل إلى النسق الفاتح
app-theme-to-dark = التبديل إلى النسق الداكن
app-reload-list = إعادة تحميل قائمة التطبيقات
app-list-loading = جارٍ التحميل
# { $count } is how many apps are listed.
app-list-count =
    { $count ->
        [zero] { $count } تطبيق
        [one] تطبيق واحد
        [two] تطبيقان
        [few] { $count } تطبيقات
        [many] { $count } تطبيقًا
       *[other] { $count } تطبيق
    }
app-list-load-failed = فشل التحميل
# Shown when the window cannot open at all. { $error } is what went wrong, in the words of Windows or a graphics driver.
app-window-could-not-open = تعذر فتح النافذة: { $error }

## The list of installed apps

app-list-search = البحث حسب الاسم أو الناشر  (Ctrl+F)
app-list-filter-all = الكل
# Whether an app is installed for one account or for everyone on the PC.
app-scope-per-user = لكل مستخدم
app-scope-per-machine = لكل جهاز
app-scope-unknown = غير معروف
# An app whose processor architecture (x86, x64, ARM64) is not known.
app-arch-unknown = غير معروفة
app-list-loading-apps = جارٍ تحميل التطبيقات المثبتة…
# { $error } is why the apps could not be listed.
app-list-could-not-list = تعذر سرد التطبيقات: { $error }
app-list-no-matches = لا توجد تطبيقات مطابقة
app-list-no-matches-hint = عدّل نص البحث أو عامل تصفية النطاق.
app-list-worker-disconnected = انقطع الاتصال بعملية جرد التطبيقات.
# What a screen reader says of an app's row. { $name } is the app's name, { $publisher } who made it, and { $version } its
# version, such as 11.4.2, or — when either is not known.
app-list-row = { $name }، { $publisher }، الإصدار { $version }

## The list's columns, and what a screen reader says each column's heading does when clicked

app-column-name = الاسم
app-column-name-sort = فرز حسب الاسم
app-column-publisher = الناشر
app-column-publisher-sort = فرز حسب الناشر
app-column-version = الإصدار
app-column-version-sort = فرز حسب الإصدار
app-column-scope = النطاق
app-column-scope-sort = فرز حسب النطاق
# Short for architecture: x86, x64, or ARM64. The column is narrow.
app-column-arch = البنية
app-column-arch-sort = فرز حسب البنية

## Where apps are read from: this PC, a drive from another PC, a disk image of one, or another PC on the network

app-source = مصدر الفحص
app-source-this-pc = هذا الكمبيوتر
app-source-drive = محرك أقراص متصل…
app-source-image = صورة قرص…
app-source-network = كمبيوتر على الشبكة…
app-source-attaching-image = جارٍ إرفاق الصورة للقراءة فقط…
# { $computer } is the other PC's name, such as OLD-PC, and { $address } its network address, such as 192.168.1.20.
app-source-network-pc = { $computer } عبر الشبكة ({ $address })
app-source-disconnect = قطع الاتصال
app-source-change = تغيير…
# The title of the window that picks a disk image. .vhd and .vhdx are file extensions.
app-source-pick-image-title = تحديد صورة قرص للكمبيوتر الآخر (.vhd أو .vhdx)
app-source-pick-image-filter = صور الأقراص
# { $path } is the disk image file.
app-source-image-has-no-windows = لا يحتوي { $path } على وحدة تخزين عليها Windows.
app-source-image-not-attached = تعذر إرفاق صورة القرص.
app-source-pick-drive-title = تحديد جذر محرك الأقراص المتصل (مثل E:\)
# { $path } is the folder picked, such as E:\. \Windows and \Users are folder names, as Windows writes them.
app-source-not-a-windows-drive = لا يبدو أن { $path } محرك أقراص نظام Windows (لا يحتوي على \Windows ولا على \Users). اختر جذر محرك الأقراص من الكمبيوتر الآخر.
# Said of a drive or disk image with a Windows older than Windows 10. { $windows } is its name, such as "Windows 7
# Professional Service Pack 1, 32-bit". { $is_32_bit } is "yes" when that Windows is 32-bit, which adds the last
# sentence, and "no" when it is not.
app-source-older-windows =
    يحتوي هذا على { $windows }. برامج التثبيت التي يتم إنشاؤها منه مخصصة لـ Windows 10 و11. مع "بيانات المستخدم"، تنتقل الإعدادات التي احتفظ بها Windows جانبًا دون إشعار لتطبيق قديم، وتُثبَّت مع التطبيق مكتبات وقت التشغيل غير الموجودة في Windows 11، مثل DirectX 9.{ $is_32_bit ->
        [yes] { " " }تطبيقاته مصممة لـ 32 بت، لذا تذهب إلى Program Files (x86)، حيث يحتفظ بها Windows 64 بت.
       *[no] { "" }
    }
# The same, of a Windows whose name could not be read. { $is_32_bit } is as above.
app-source-older-windows-unnamed =
    يحتوي هذا على إصدار أقدم من Windows. برامج التثبيت التي يتم إنشاؤها منه مخصصة لـ Windows 10 و11. مع "بيانات المستخدم"، تنتقل الإعدادات التي احتفظ بها Windows جانبًا دون إشعار لتطبيق قديم، وتُثبَّت مع التطبيق مكتبات وقت التشغيل غير الموجودة في Windows 11، مثل DirectX 9.{ $is_32_bit ->
        [yes] { " " }تطبيقاته مصممة لـ 32 بت، لذا تذهب إلى Program Files (x86)، حيث يحتفظ بها Windows 64 بت.
       *[no] { "" }
    }

## The details panel, before an app is picked, and the app picked

app-details-pair-first = الاقتران بالكمبيوتر القديم
app-details-pair-first-hint = بمجرد تشغيل WinAppMigrator Client هناك وكتابة الرمز الذي يعرضه، تُدرج تطبيقاته هنا، ويمكنك إنشاء برنامج تثبيت من أحدها كما تفعل من هذا الكمبيوتر.
app-details-pick-an-app = اختيار تطبيق
app-details-pick-an-app-hint = حدد تطبيقًا من القائمة لاختيار ما سيتم تضمينه وإنشاء برنامج التثبيت الخاص به.
# { $publisher } is who made the app, and { $version } its version, such as 11.4.2.
app-details-publisher-and-version = { $publisher }، الإصدار { $version }
# { $version } is the app's version, such as 11.4.2.
app-details-unknown-publisher-and-version = ناشر غير معروف، الإصدار { $version }
app-details-unknown-publisher = ناشر غير معروف

## What goes in besides the program

app-option-license = بيانات الترخيص والتنشيط
app-option-license-hint = قد تكون مرتبطة بهذا الكمبيوتر وتحتاج إلى إعادة التنشيط على الكمبيوتر الجديد.
app-option-user-data = بيانات المستخدم
# Documents, Saved Games, and ProgramData are folders of Windows: use the names Windows gives them in your language,
# except ProgramData, which keeps its name.
app-option-user-data-hint = الإعدادات والمحفوظات، و"المستندات"، و"الألعاب المحفوظة"، وProgramData.
app-option-whose-data = بيانات أي حساب
app-option-whose-data-hint = يحتوي برنامج التثبيت على بيانات حساب واحد. تذهب هذه البيانات إلى المستخدم الذي سجّل الدخول حيث يتم تشغيله.

## Building the installer

app-output-folder = مجلد الإخراج
# An example of a folder, shown in the empty field. "You" stands for the name of an account.
app-output-folder-example = C:\Users\You\WinAppMigrator
app-output-folder-not-full = أدخل مسار مجلد كاملاً، مثل D:\Installers.
app-build-create = إنشاء برنامج التثبيت
app-build-create-again = إنشاء برنامج التثبيت مرة أخرى
app-build-cancel = إلغاء الأمر
app-build-cancelling = جارٍ الإلغاء بعد الخطوة الحالية…
app-build-cancelled = تم إلغاء الإنشاء. لم تتم كتابة أي برنامج تثبيت.
# winappmigrator-installer.exe is a file name.
app-build-no-stub = لم يتم العثور على قالب برنامج التثبيت (winappmigrator-installer.exe) بجانب هذا التطبيق، لذا لا يمكن إنشاء برنامج تثبيت ذاتي الاستخراج.
app-build-select-an-app-first = حدد أولاً تطبيقًا لترحيله.
app-build-look-again-first = يجب فحص هذا التطبيق مرة أخرى قبل أن يمكن إنشاء برنامج التثبيت الخاص به.
# { $reason } is what went wrong, in the program's own words, often in English.
app-build-stopped-unexpectedly = توقف الإنشاء بشكل غير متوقع: { $reason }. إذا ترك برنامج تثبيت في مجلد الإخراج، فلا تستخدمه.
app-build-worker-disconnected = انقطع الاتصال بعملية الترحيل.
# { $path } is the output folder, and { $error } what Windows said went wrong.
app-build-no-output-folder = تعذر إنشاء مجلد الإخراج { $path }: { $error }
# { $error } is what went wrong.
app-build-no-staging-folder = تعذر إنشاء مجلد تجهيز محمي، لذا لم يتم إنشاء أي شيء: { $error }
# { $error } is what went wrong, as a sentence.
app-build-could-not-build = تعذر إنشاء برنامج التثبيت. { $error }
app-build-no-installer = انتهى الإنشاء دون أي برنامج تثبيت.
# { $error } is what went wrong.
app-build-hash-check-did-not-run = تمت كتابة برنامج التثبيت، لكن تعذر تشغيل التحقق من التجزئة الخاص به: { $error }. لا تستخدمه.

## How far a build has got

app-progress-preparing = جارٍ تحديد ما سيتم تضمينه…
app-progress-checking-room = جارٍ التحقق من توفر مساحة…
app-progress-bundling = جارٍ تجميع مكتبات وقت التشغيل وبرامج التشغيل…
# { $done } is how many files have been read so far, and { $total } how many there are.
app-progress-packing = جارٍ قراءة الملفات وضغطها: { $done } من { $total }
# { $done } is how many of the installer's records have been looked over so far, and { $total } how many there are.
app-progress-looking-over = جارٍ مراجعة ما يذكره برنامج التثبيت: { $done } من { $total }…
app-progress-writing = جارٍ كتابة برنامج التثبيت…
app-progress-verifying = جارٍ التحقق من التجزئات…

## What a build made

app-result-failed-hash-check = لا تستخدم برنامج التثبيت هذا: فقد فشل في التحقق من التجزئة.
# { $count } is how many files were gone when the build came to read them.
app-result-files-gone =
    { $count ->
        [zero] تم إنشاء برنامج التثبيت، لكن { $count } ملف لم تعد موجودة منذ فحص التطبيق، لذا فهي غير مضمّنة فيه. راجع "التحذيرات" أدناه.
        [one] تم إنشاء برنامج التثبيت، لكن ملفًا واحدًا لم يعد موجودًا منذ فحص التطبيق، لذا فهو غير مضمّن فيه. راجع "التحذيرات" أدناه.
        [two] تم إنشاء برنامج التثبيت، لكن ملفين لم يعودا موجودين منذ فحص التطبيق، لذا فهما غير مضمّنين فيه. راجع "التحذيرات" أدناه.
        [few] تم إنشاء برنامج التثبيت، لكن { $count } ملفات لم تعد موجودة منذ فحص التطبيق، لذا فهي غير مضمّنة فيه. راجع "التحذيرات" أدناه.
        [many] تم إنشاء برنامج التثبيت، لكن { $count } ملفًا لم تعد موجودة منذ فحص التطبيق، لذا فهي غير مضمّنة فيه. راجع "التحذيرات" أدناه.
       *[other] تم إنشاء برنامج التثبيت، لكن { $count } ملف لم تعد موجودة منذ فحص التطبيق، لذا فهي غير مضمّنة فيه. راجع "التحذيرات" أدناه.
    }
# { $count } is how many files could not be read into the installer.
app-result-files-not-captured =
    { $count ->
        [zero] تم إنشاء برنامج التثبيت، لكن تعذر التقاط { $count } ملف، عادةً لأنها كانت مقفلة أو غير قابلة للقراءة. قد لا يعمل التطبيق بدونها. راجع "التحذيرات" أدناه.
        [one] تم إنشاء برنامج التثبيت، لكن تعذر التقاط ملف واحد، عادةً لأنه كان مقفلاً أو غير قابل للقراءة. قد لا يعمل التطبيق بدونه. راجع "التحذيرات" أدناه.
        [two] تم إنشاء برنامج التثبيت، لكن تعذر التقاط ملفين، عادةً لأنهما كانا مقفلين أو غير قابلين للقراءة. قد لا يعمل التطبيق بدونهما. راجع "التحذيرات" أدناه.
        [few] تم إنشاء برنامج التثبيت، لكن تعذر التقاط { $count } ملفات، عادةً لأنها كانت مقفلة أو غير قابلة للقراءة. قد لا يعمل التطبيق بدونها. راجع "التحذيرات" أدناه.
        [many] تم إنشاء برنامج التثبيت، لكن تعذر التقاط { $count } ملفًا، عادةً لأنها كانت مقفلة أو غير قابلة للقراءة. قد لا يعمل التطبيق بدونها. راجع "التحذيرات" أدناه.
       *[other] تم إنشاء برنامج التثبيت، لكن تعذر التقاط { $count } ملف، عادةً لأنها كانت مقفلة أو غير قابلة للقراءة. قد لا يعمل التطبيق بدونها. راجع "التحذيرات" أدناه.
    }
app-result-mentions-who = تم إنشاء برنامج التثبيت، ولا يزال يتضمن ما يدل على مصدره. اقرأ ما يذكره أدناه قبل أن تعطيه لأي شخص.
app-result-ready = برنامج التثبيت جاهز. اجتازت حزمته التحقق من التجزئة.
app-result-files = الملفات
app-result-installer-size = حجم برنامج التثبيت
app-result-warnings = التحذيرات
app-result-log = ملف السجل
app-result-copy-error = نسخ الخطأ
app-result-installer = برنامج التثبيت
app-result-data-file = ملف البيانات
app-result-open-folder = فتح المجلد
app-result-copy-path = نسخ المسار
# { $error } is what Windows said went wrong.
app-result-could-not-open-folder = تعذر فتح المجلد: { $error }
# { $installer } and { $data_file } are file names, such as App_installer.exe and App_installer.wamdata.
app-result-two-files = هذا التطبيق أكبر من أن يتسع له ملف تنفيذي واحد، لذا يتكون برنامج التثبيت الخاص به من ملفين، { $installer } و{ $data_file }. يجب أن يبقيا معًا: انسخ كليهما إلى الكمبيوتر الآخر، في المجلد نفسه، بهذين الاسمين. إذا وقّعت برنامج التثبيت، فإن هذا التوقيع يشمل ملف البيانات أيضًا.
# { $app_name } is the app's name. "Unknown publisher" is what Windows shows when it asks to run an unsigned program: use
# Windows' own words for it in your language.
app-result-run-it = شغّله على الكمبيوتر الآخر لإعادة تثبيت { $app_name }. يتحقق من ملفاته باستخدام SHA-256. يعرض Windows "ناشر غير معروف" لأن برامج التثبيت التي تنشئها غير موقّعة.
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-verification-errors = أخطاء التحقق ({ $count })
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-warnings-heading = التحذيرات ({ $count })
# After the first of a long list of warnings. { $count } is how many more there are.
app-warnings-and-more = و{ $count } غيرها.

## Looking at an app, to see what an installer of it would hold

app-scan-getting-ready = جارٍ الاستعداد لفحص هذا التطبيق…
app-scan-install-record = جارٍ قراءة كيفية تثبيته…
app-scan-near-matches = جارٍ البحث عما قد يكون جزءًا منه أيضًا…
app-scan-system-items = جارٍ البحث عن خدماته ومهامه واختصاراته…
app-scan-files = جارٍ فحص ملفاته…
app-scan-registry = جارٍ قراءة ما يربطه بـ Windows…
app-scan-finishing = جارٍ الإنهاء…
app-scan-look-again = إعادة الفحص
app-scan-stop = إيقاف
app-scan-running-hint = يمكنك اختيار تطبيق آخر أو تغيير ما يتم تضمينه أثناء تشغيل هذا الفحص.
# { $error } is why, in words that follow a colon.
app-scan-failed = تعذر فحص هذا التطبيق: { $error }
app-scan-try-again = إعادة المحاولة
app-scan-stopped = تم الإيقاف. يمكن إنشاء برنامج تثبيت بعد اكتمال فحص هذا التطبيق.
# { $reason } is what went wrong, in the program's own words, often in English.
app-scan-stopped-unexpectedly = توقف بشكل غير متوقع: { $reason }.

## How big an installer can be, and whether there is room for it

app-room-files = الملفات
# Over how big the installer can be: up to that size.
app-room-up-to = الحد الأقصى
app-room-free = المساحة الحرة
# { $drive } is a drive, such as C:.
app-room-free-on = المساحة الحرة على { $drive }
# How much room a drive has, when it would not say.
app-room-free-unknown = غير معروفة
app-room-most-it-could-need = أقصى ما قد يحتاج إليه، قبل الضغط.
# { $drive } is a drive, such as C:, { $needed } how much room it needs, such as 1.2 GB, and { $free } how much it has.
app-room-drive-short = يحتاج { $drive } إلى مساحة حرة تصل إلى { $needed } لإنشاء هذا، والمتوفر { $free }.
# { $drives } is one sentence like the one above for each drive without the room, one after another.
app-room-not-enough = لا توجد مساحة كافية. { $drives } اختر مجلد إخراج على محرك أقراص آخر، أو استبعد شيئًا.
# { $note } is a sentence, without its full stop, about a drive whose room could not be checked. Use your language's
# full stop.
app-room-could-not-tell = { $note }.

## What goes in the installer, part by part

app-footprint-heading = ما يدخل في برنامج التثبيت
app-footprint-working-out = جارٍ تحديد ما يرافق ذلك…
# { $error } is why.
app-footprint-would-not-make-a-package = ما تبقى لا يكوّن حزمة: { $error }
# { $list } is the names of what else is left out, such as a service and a shortcut.
app-footprint-also-left-out = يؤدي ذلك أيضًا إلى استبعاد { $list }، لأن ما تشغّله أو تسمّيه لن يكون موجودًا. لن يتم إعدادها على الكمبيوتر الآخر.
# { $list } is the names of the first few of what else is left out, and { $count } how many more there are.
app-footprint-also-left-out-and-more = يؤدي ذلك أيضًا إلى استبعاد { $list } و{ $count } غيرها، لأن ما تشغّله أو تسمّيه لن يكون موجودًا. لن يتم إعدادها على الكمبيوتر الآخر.
# { $list } is the names of license files. The option is the one named "License and Activation Data".
app-footprint-licenses-still-go = بيانات الترخيص الموجودة فيما استبعدته لا تزال مضمّنة، لأنها تخضع لخيار "بيانات الترخيص والتنشيط": { $list }.
# { $elapsed } is how long ago, in minutes and seconds such as 45:00, or hours, minutes, and seconds such as 1:05:00.
# "Look Again" names the button of that name.
app-footprint-looked-at-long-ago = تم فحص هذا منذ { $elapsed }. تُقرأ الملفات عند الإنشاء، لذا يتم التقاط أي تغيير، ويُدرج كل ما لم يعد موجودًا حينها. انقر فوق "إعادة الفحص" لالتقاط أي شيء جديد.
# Headings of the parts of the list, each with a line that says what is in it.
app-footprint-program-files = ملفات البرنامج
app-footprint-data-folders = مجلدات البيانات
# What could go in with the User Data option, when nothing else is in this part. { $size } is how much, such as 1.2 GB.
app-footprint-folders-with-user-data = { $size } مع خيار "بيانات المستخدم"
# { $included } is how many folders go in, { $count } how many there are, and { $size } how much those that go in hold.
app-footprint-folders-summary =
    { $count ->
        [zero] { $included } من أصل { $count } مجلد، { $size }
        [one] { $included } من أصل مجلد واحد، { $size }
        [two] { $included } من أصل مجلدين، { $size }
        [few] { $included } من أصل { $count } مجلدات، { $size }
        [many] { $included } من أصل { $count } مجلدًا، { $size }
       *[other] { $included } من أصل { $count } مجلد، { $size }
    }
# { $size } is how much, such as 1.2 GB, and { $count } in how many files.
app-footprint-data-only-with-user-data =
    { $count ->
        [zero] لن تُضمّن بيانات هذا التطبيق البالغة { $size } في { $count } ملف إلا مع خيار "بيانات المستخدم".
        [one] لن تُضمّن بيانات هذا التطبيق البالغة { $size } في ملف واحد إلا مع خيار "بيانات المستخدم".
        [two] لن تُضمّن بيانات هذا التطبيق البالغة { $size } في ملفين إلا مع خيار "بيانات المستخدم".
        [few] لن تُضمّن بيانات هذا التطبيق البالغة { $size } في { $count } ملفات إلا مع خيار "بيانات المستخدم".
        [many] لن تُضمّن بيانات هذا التطبيق البالغة { $size } في { $count } ملفًا إلا مع خيار "بيانات المستخدم".
       *[other] لن تُضمّن بيانات هذا التطبيق البالغة { $size } في { $count } ملف إلا مع خيار "بيانات المستخدم".
    }
# A folder's row. { $count } is how many files it holds, and { $detail } where it is or why it cannot be left out.
app-footprint-folder-row =
    { $count ->
        [zero] { $count } ملف. { $detail }
        [one] ملف واحد. { $detail }
        [two] ملفان. { $detail }
        [few] { $count } ملفات. { $detail }
        [many] { $count } ملفًا. { $detail }
       *[other] { $count } ملف. { $detail }
    }
app-footprint-services = الخدمات
app-footprint-startup = يبدأ تلقائيًا
app-footprint-tasks = المهام المجدولة
app-footprint-shortcuts = الاختصارات
# { $included } is how many of the part's things go in, and { $count } how many there are.
app-footprint-items-summary = { $included } من أصل { $count }
# { $why } is what else was left out that this goes with.
app-footprint-left-out-too = مستبعد أيضًا: { $why }
# A row's detail. { $kind } is what it is, such as Service, and { $detail } what it does.
app-footprint-change-detail = { $kind }. { $detail }
app-footprint-integrations = ما يربطه بـ Windows
# { $included } is how many kinds go in, such as file types and firewall rules, and { $count } how many kinds there are.
app-footprint-integrations-summary =
    { $count ->
        [zero] { $included } من أصل { $count } نوع
        [one] { $included } من أصل نوع واحد
        [two] { $included } من أصل نوعين
        [few] { $included } من أصل { $count } أنواع
        [many] { $included } من أصل { $count } نوعًا
       *[other] { $included } من أصل { $count } نوع
    }
app-footprint-integration-nothing-left = لم يتبقَّ منه شيء، بسبب ما استبعدته
# { $count } is how many things of this kind the installer sets up.
app-footprint-integration-things =
    { $count ->
        [zero] { $count } عنصر يتم إعداده
        [one] عنصر واحد يتم إعداده
        [two] عنصران يتم إعدادهما
        [few] { $count } عناصر يتم إعدادها
        [many] { $count } عنصرًا يتم إعدادها
       *[other] { $count } عنصر يتم إعدادها
    }
app-footprint-prerequisites = ما يحتاج إليه للتشغيل
# { $count } is how many runtimes, such as Visual C++ Redistributable.
app-footprint-prerequisites-summary =
    { $count ->
        [zero] { $count } مكتبة وقت تشغيل
        [one] مكتبة وقت تشغيل واحدة
        [two] مكتبتا وقت تشغيل
        [few] { $count } مكتبات وقت تشغيل
        [many] { $count } مكتبة وقت تشغيل
       *[other] { $count } مكتبة وقت تشغيل
    }
app-footprint-near-matches = قد يكون جزءًا منه أيضًا
# { $ticked } is how many were ticked, and { $count } how many there are.
app-footprint-near-matches-summary = تمت إضافة { $ticked } من أصل { $count }
app-footprint-near-matches-hint = لا تُضمّن في برنامج التثبيت ما لم تحددها. يؤدي تحديد أحدها إلى فحص التطبيق مرة أخرى، بالفحوصات نفسها المطبقة على الباقي، لذا يبقى ما يبدو ككلمة مرور مستبعدًا.
# { $why } is why it might be part of the app, as a sentence.
app-footprint-near-match-with-user-data = يأتي مع خيار "بيانات المستخدم". { $why }
# How much a folder holds, when not all of it could be measured. { $size } is how much was, such as 1.2 GB.
app-footprint-near-match-over = أكثر من { $size }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-folder = المجلد: { $place }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-hidden = جزء مخفي من تطبيق: { $place }
app-footprint-nothing-left-out = لم يُستبعد أي شيء لكونه يبدو ككلمة مرور أو ترخيص أو شهادة.
app-footprint-left-out = ما تم استبعاده وسبب ذلك
# { $count } is how many things were left out.
app-footprint-left-out-summary =
    { $count ->
        [zero] { $count } عنصر
        [one] عنصر واحد
        [two] عنصران
        [few] { $count } عناصر
        [many] { $count } عنصرًا
       *[other] { $count } عنصر
    }
# Above a list of things left out for one reason. { $title } is the reason.
app-footprint-left-out-group = { $title }:
app-footprint-refused = لن يُسمح لأي برنامج تثبيت بإعداد هذه العناصر:
# A heading over a list that opens and closes. { $count } is how many are in it.
app-footprint-worth-knowing = معلومات مفيدة ({ $count })
# { $list } is what kinds of things were looked for and none found, from the five below.
app-none-found = لم يتم العثور على: { $list }.
app-none-found-services = خدمات
app-none-found-startup = عناصر بدء التشغيل
app-none-found-tasks = مهام مجدولة
app-none-found-shortcuts = اختصارات
app-none-found-integrations = أي شيء يربطه بـ Windows

## Network PC: reading another PC through WinAppMigrator Client, in three steps. "WinAppMigrator Client" is the name of a
## program, and the old PC is the one the apps are moved off.

app-network-step-client = 1. وضع العميل على الكمبيوتر القديم
app-network-step-client-hint = يعمل WinAppMigrator Client على الكمبيوتر الذي يوجد عليه التطبيق الآن، بنظام Windows 7 أو أحدث. ولا يسمح إلا لهذا الكمبيوتر بقراءته، بشكل مشفّر، بواسطة رمز يعرضه.
# { $drive } is a USB drive, such as E:\.
app-network-save-to-usb = الحفظ على محرك أقراص USB { $drive }
# { $drive } is a USB drive, such as E:\, and { $name } the name it was given, such as BACKUP.
app-network-save-to-named-usb = الحفظ على محرك أقراص USB { $drive } ({ $name })
app-network-save-to-folder = الحفظ في مجلد…
app-network-look-for-usb-again = البحث عن محركات أقراص USB مرة أخرى
app-network-no-usb = لا يوجد محرك أقراص USB متصل. قم بتوصيل محرك، أو احفظ في مجلد: مجلد يمكنك الوصول إليه من الكمبيوتر القديم، مثل مجلد مشترك على الشبكة.
# { $path } is where the client was saved, such as E:\WinAppMigrator Client.exe.
app-network-client-saved = تم الحفظ باسم { $path }. على الكمبيوتر القديم، أغلق التطبيق الذي تنقله، ثم انقر نقرًا مزدوجًا فوق العميل واسمح له بإجراء تغييرات. سيعرض رمزًا.
app-network-step-pick = 2. اختيار الكمبيوتر القديم
app-network-looking = جارٍ البحث في الشبكة عن أجهزة كمبيوتر يعمل عليها العميل…
app-network-type-name = أو اكتب اسمه أو عنوانه
# An example, shown in the empty field: a PC's name, or its network address.
app-network-type-name-example = OLD-PC أو 192.168.1.20
app-network-step-code = 3. كتابة الرمز الذي يعرضه
app-network-code = رمز الاقتران
app-network-connect = اتصال
app-network-pairing = جارٍ الاقتران…
# Why pairing did not work, shown on its own.
app-network-pairing-stopped = توقف الاقتران قبل اكتماله
app-network-wrong-code = هذا ليس الرمز الذي يعرضه الكمبيوتر القديم. تحقق منه وحاول مرة أخرى.
# { $error } is what went wrong.
app-network-not-reached = تعذر الوصول إلى الكمبيوتر القديم: { $error }
app-network-pick-folder-title = اختيار مجلد لـ WinAppMigrator Client
# { $path } is the folder, and { $error } what Windows said went wrong.
app-client-could-not-save = تعذر حفظ العميل في { $path }: { $error }
# scripts\build-client.ps1 is a file name.
app-client-not-carried = إصدار WinAppMigrator هذا لا يحمل العميل. أنشئ العميل باستخدام scripts\build-client.ps1، ثم أنشئ التطبيق مرة أخرى.
# { $error } is what Windows said went wrong.
app-client-could-not-save-note = تعذر حفظ الملاحظة المرافقة له: { $error }
# A note saved as a text file beside the client, for whoever runs it on the old PC. Keep its lines short, as it is read in
# Notepad. "WinAppMigrator Client.exe" is a file name. "Network PC..." names the Scan Source button: use its words in your
# language.
app-client-note =
    WinAppMigrator Client
    
    شغّل هذا على الكمبيوتر الذي تنقل التطبيقات منه: Windows 7 أو 8 أو 10 أو 11.
    
    1. أغلق التطبيق الذي تنقله.
    2. انقر نقرًا مزدوجًا فوق "WinAppMigrator Client.exe" واسمح له بإجراء تغييرات.
    3. يعرض رمز اقتران من 6 أرقام. في WinAppMigrator على الكمبيوتر الجديد،
       اختر "كمبيوتر على الشبكة..."، ثم اختر هذا الكمبيوتر، واكتب الرمز.
    4. عند انتهاء الكمبيوتر الجديد، اضغط على Enter في نافذة العميل، أو أغلقها.
    
    يقتصر عمله على القراءة. أثناء تشغيله يحتفظ بلقطة لمحرك أقراص Windows
    ويسمح بمرور منفذيه عبر جدار حماية Windows من شبكتك المحلية، ويزيل
    كليهما عند توقفه. كل ما يعبر الشبكة مشفّر، ولا يمكن قراءة أي شيء إلا
    لكمبيوتر لديه الرمز.
