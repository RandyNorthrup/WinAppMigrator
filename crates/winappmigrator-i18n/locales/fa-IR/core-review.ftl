# Words the core says about what an install may do: why an install location or something in a package is refused,
# the installer's list of what an app sets up beyond its files, the checks of a package and of an install, and whether
# there is room for either. Message names start with core-.
#
# Many messages here are a reason, the second half of a sentence that starts "... was left out:" or "that install
# location cannot be used:", so they start with a small letter and have no full stop.


## Why an install location cannot be used. Shown in red under the Install Location field.

core-review-location-state-root = جایی است که نصب‌ها سوابق خود را نگه می‌دارند
core-review-location-has-files = از قبل فایل‌هایی دارد و هیچ نصبی در اینجا ثبت نشده است، بنابراین از یک پوشه خالی استفاده کنید
core-policy-location-not-full-path = باید یک مسیر کامل باشد
core-policy-location-parent-step = نباید شامل «..» باشد
core-policy-location-not-plain = باید یک مسیر ساده روی درایوی از این رایانه باشد
core-policy-location-whole-drive = نمی‌تواند کل یک درایو باشد
core-policy-location-windows = نمی‌تواند پوشه Windows باشد
core-policy-location-runs-by-itself = پوشه‌ای است که متعلق به Windows است، یا محتوای آن خودبه‌خود اجرا یا بارگذاری می‌شود
core-policy-location-shared = چیزهای دیگری را کنار هم در خود دارد، بنابراین برنامه به پوشه‌ای درون آن نیاز دارد

## Whether an app can run on this PC at all, said before anything is installed.

# { $reason } is why, as the second half of a sentence, such as "App.exe is built for Arm64, and this PC is x64, so
# Windows would not start it".
core-review-cannot-run-here = این برنامه نمی‌تواند در این رایانه اجرا شود، بنابراین چیزی تغییر نکرد: { $reason }
core-policy-architecture-x86 = x86 32 بیتی
core-policy-architecture-x64 = x64
core-policy-architecture-arm64 = Arm64
core-policy-architecture-arm32 = Arm 32 بیتی
core-policy-architecture-unknown = یک معماری ناشناخته
# { $code } is the number Windows gives a kind of processor, such as 0x01c4.
core-policy-machine-type = نوع ماشین { $code }
# { $names } is a list of program names, such as "a.exe, b.exe, c.exe", and { $count } how many more there are.
core-policy-names-and-more = { $names } و { $count } مورد دیگر
core-policy-fit-programs-not-said = این نصب‌کننده نمی‌گوید فایل‌های اجرایی‌اش به چه چیزی نیاز دارند، بنابراین بررسی نشد که آیا می‌توانند در این رایانه اجرا شوند.
core-policy-fit-architecture-not-told = معماری این رایانه مشخص نشد، بنابراین بررسی نشد که آیا فایل‌های اجرایی این برنامه می‌توانند روی آن اجرا شوند.
core-policy-fit-main-not-said = این نصب‌کننده نمی‌گوید فایل اجرایی اصلی‌اش به چه چیزی نیاز دارد، بنابراین آنچه اینجا گفته می‌شود بر اساس بقیه محتوای آن است.
# { $driver } is a driver's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-driver = درایور آن، { $driver }، برای { $built_for } ساخته شده است و این رایانه { $here } است. درایور باید دقیقاً با Windows مطابقت داشته باشد، بنابراین این برنامه نمی‌تواند اینجا اجرا شود
# { $program } is a program's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-program = { $program } برای { $built_for } ساخته شده است و این رایانه { $here } است، بنابراین Windows آن را اجرا نمی‌کند
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-none-start = هیچ‌کدام از فایل‌های اجرایی آن نمی‌توانند در این رایانه، که { $here } است، شروع شوند: { $programs }
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-some-other-architecture = برخی از فایل‌های اجرایی این برنامه برای معماری دیگری ساخته شده‌اند و در این رایانه، که { $here } است، شروع نمی‌شوند: { $programs }. بقیه برنامه می‌تواند شروع شود، بنابراین شاید این مهم نباشد.
# { $here } is a kind of processor, such as "Arm64". { $programs } is a list of program names.
core-policy-fit-emulated = این رایانه { $here } است، بنابراین Windows این‌ها را از طریق شبیه‌ساز اجرا می‌کند: { $programs }. بیشتر برنامه‌ها به این شکل مشکلی ندارند، اما برخی دارند.
# { $program } is a program's file name. { $needs } and { $runs } are versions of Windows, such as 10.0 and 6.1.
core-policy-fit-needs-newer = { $program } می‌گوید به Windows { $needs } یا جدیدتر نیاز دارد و این رایانه { $runs } را اجرا می‌کند، بنابراین Windows آن را اجرا نمی‌کند
# { $programs } is a list of program names.
core-policy-fit-some-need-newer = برخی از فایل‌های اجرایی این برنامه می‌گویند به Windows جدیدتری نیاز دارند و اینجا شروع نمی‌شوند: { $programs }.
# { $from_build } and { $this_build } are Windows build numbers, such as 22631, written as Windows writes them.
core-policy-fit-older-build = این برنامه روی Windows با شماره ساخت { $from_build } بود و این رایانه ساخت { $this_build } را اجرا می‌کند. فایل‌های اجرایی آن می‌گویند می‌توانند روی این Windows شروع شوند، بنابراین شاید کار کند، اما شاید آنچه در آنجا به آن متکی بود اینجا نباشد.
core-policy-fit-build-not-told = شماره ساخت این Windows مشخص نشد، بنابراین با ساختی که این برنامه روی آن بود مقایسه نشد.

## Room on a drive for a build or an install.

# Each of these ends the sentence core-space-shortfall, after "free for".
core-space-for-unpacking = استخراج بسته
core-space-for-install-record = سابقه نصب
core-space-for-app-files = فایل‌های برنامه
core-space-for-what-is-replaced = نگه‌داشتن آنچه جایگزین می‌شود
core-space-for-build-work = فایل‌هایی که ساخت با آن‌ها کار می‌کند
core-space-for-installer = نصب‌کننده
core-space-for-package = بسته
core-space-for-unpacking-and-installing = استخراج بسته و نصب آن
# What goes between two of those, as in "unpacking the package and the app's files". Keep the quotes: they keep the
# spaces.
core-space-purposes-joiner = { " و " }
# { $drive } is a drive, such as C:, or a folder. { $needed } and { $free } are sizes, such as 1.2 GB. { $purposes } is
# what the room is for, such as "unpacking the package and the app's files".
core-space-shortfall = { $drive } برای { $purposes } به حداکثر { $needed } فضای آزاد نیاز دارد و { $free } دارد
# { $drive } is a drive, such as C:, or a folder. { $needed } is a size, such as 1.2 GB.
core-space-not-told = { $drive } نمی‌گوید چه مقدار فضا دارد، بنابراین بررسی نشد که آیا { $needed } مورد نیاز جا می‌شود
# One drive in a list of where room is needed. { $needed } and { $free } are sizes, and { $drive } a drive, such as C:.
core-space-summary = حداکثر { $needed } در { $drive } ({ $free } آزاد)
# { $needed } is a size, and { $drive } a drive, such as C:.
core-space-summary-not-told = حداکثر { $needed } در { $drive } (فضای آزاد مشخص نشد)
# What goes between two sentences of core-space-shortfall inside core-review-no-room. Keep the quotes: they keep the
# space.
core-review-sentence-joiner = { ". " }
# { $shortfalls } is one or more sentences of core-space-shortfall.
core-review-no-room = فضای کافی برای این کار وجود ندارد، بنابراین چیزی تغییر نکرد. { $shortfalls }. در آنجا مقداری فضا آزاد کنید، یا محل نصبی در درایو دیگری انتخاب کنید.

## The installer's list of what an app sets up beyond its files: what each kind is called.

core-review-kind-service = سرویس
core-review-kind-driver = درایور
core-review-kind-scheduled-task = کار زمان‌بندی‌شده
core-review-kind-autostart = شروع خودکار
core-review-kind-shortcut = میان‌بر
core-review-kind-path = PATH
core-review-kind-environment-variable = متغیر محیطی
core-review-kind-shell-extension = افزونه پوسته
core-review-kind-menu-command = فرمان منو
core-review-kind-link-type = نوع پیوند
core-review-kind-file-type = نوع فایل
core-review-kind-com = COM
core-review-kind-firewall-rule = قانون دیوار آتش
core-review-kind-event-log-source = منبع گزارش رویداد
core-review-kind-font = قلم
core-review-kind-odbc-driver = درایور ODBC
core-review-kind-odbc-source = منبع داده ODBC
core-review-kind-compatibility-setting = تنظیم سازگاری
core-review-kind-default-apps = برنامه‌های پیش‌فرض
core-review-kind-program-name = نام برنامه
core-review-kind-other-settings = تنظیمات دیگر

## The installer's list of what an app sets up: what each line says of it.

# { $file } is a program's file name. { $account } is the Windows account it runs as, such as LocalSystem. { $starts }
# says when it starts, and is not shown itself; for "other", { $start } is one of the core-review-start- words below.
core-review-service-detail =
    { $starts ->
        [automatic] { $file } را به‌عنوان { $account } اجرا می‌کند، اکنون و هر بار که این رایانه راه‌اندازی شود شروع می‌شود
        [delayed] { $file } را به‌عنوان { $account } اجرا می‌کند، اکنون و کمی پس از هر بار راه‌اندازی این رایانه شروع می‌شود
        [manual] { $file } را به‌عنوان { $account } اجرا می‌کند، فقط وقتی چیزی درخواست کند شروع می‌شود
        [disabled] { $file } را به‌عنوان { $account } اجرا می‌کند، غیرفعال است
        [other] { $file } را به‌عنوان { $account } اجرا می‌کند، زمان شروع: { $start }
       *[unknown] { $file } را به‌عنوان { $account } اجرا می‌کند
    }
# { $account } is a Windows account, such as LocalService.
core-policy-account-custom = { $account } (حساب سفارشی بازیابی نمی‌شود)
# { $file } is a driver's file name.
core-review-driver-detail = { $file } را در خود Windows بارگذاری می‌کند
# { $file } is a driver's file name. { $start } is when Windows starts it: one of the core-review-start- words below.
core-review-driver-detail-starts = { $file } را در خود Windows بارگذاری می‌کند، زمان شروع: { $start }
# When a service or driver starts, ending a sentence such as "Loads disk.sys into Windows itself, starts …".
core-review-start-boot = همراه با Windows، پیش از هر چیز دیگر
core-review-start-system = همراه با Windows، هنگام شروع آن
core-review-start-automatic = هر بار که این رایانه راه‌اندازی می‌شود
core-review-start-delayed = کمی پس از هر بار راه‌اندازی این رایانه
core-review-start-manual = فقط وقتی چیزی درخواست کند
core-review-start-disabled = هرگز، زیرا غیرفعال است
core-review-start-unknown = در زمانی که Windows مشخص نمی‌کند
# { $account } is the account a task runs as: an account's name, or core-review-task-runs-as-you, or
# core-review-task-runs-as-creator.
core-review-task-detail = به‌عنوان { $account } اجرا می‌شود
# { $account } is the account a task runs as, as for core-review-task-detail.
core-review-task-detail-highest = به‌عنوان { $account } و با بالاترین حقوقی که می‌تواند داشته باشد اجرا می‌شود
core-review-task-runs-as-you = شما، هر کسی که وارد سیستم شده باشد
core-review-task-runs-as-creator = کاربری که آن را ایجاد کرد
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-autostart-detail =
    { $scope ->
        [everyone] برای هر کسی که وارد سیستم شود اجرا می‌شود
       *[you] وقتی شما وارد سیستم شوید اجرا می‌شود
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shortcut-startup-detail =
    میان‌بری در پوشه راه‌اندازی، { $scope ->
        [everyone] برای همه کاربران این رایانه
       *[you] برای شما
    }
# { $file } is the file name of what the shortcut opens. { $scope } is whom it is set up for: everyone on this PC,
# or you. It is not shown itself.
core-review-shortcut-detail =
    { $file } را باز می‌کند، { $scope ->
        [everyone] برای همه کاربران این رایانه
       *[you] برای شما
    }
# { $name } is an environment variable's name, such as PATH.
core-review-path-title = به { $name } اضافه می‌کند
core-review-path-install-folder = پوشه نصب
# { $path } is a folder inside the install folder, such as bin\tools.
core-review-path-below-install-folder = پوشه نصب\{ $path }
# { $value } is the variable's value, as it will be set.
core-review-variable-detail = روی { $value } تنظیم می‌شود
# { $count } is how many registry values.
core-review-other-settings-title =
    { $count ->
        [one] { $count } تنظیم بیرون از کلیدهای خود برنامه
       *[other] { $count } تنظیم بیرون از کلیدهای خود برنامه
    }
# { $keys } is a list of registry keys, such as HKCU\Software\Vendor.
core-review-other-settings-detail = در { $keys }
# { $keys } is a list of registry keys, and { $count } how many more there are.
core-review-other-settings-detail-more = در { $keys } و { $count } مورد دیگر
# { $way } says whether the rule allows or blocks, and incoming or outgoing connections; it is not shown itself.
# { $ports } is a list of port numbers, and { $port_count } how many. { $program } is the program's file name.
# { $networks } is a list of the kinds of network Windows names in English (domain, private, public), and
# { $network_count } how many.
# The kinds of network a firewall rule is for, as Windows Defender Firewall names its profiles in your language. They
# fill { $networks } in core-review-firewall-detail, joined into a list.
core-review-network-domain = دامنه
core-review-network-private = خصوصی
core-review-network-public = عمومی
core-review-firewall-detail =
    { $way ->
        [allow-in] اجازه دادن به اتصال‌های ورودی
        [allow-out] اجازه دادن به اتصال‌های خروجی
        [block-in] مسدود کردن اتصال‌های ورودی
       *[block-out] مسدود کردن اتصال‌های خروجی
    }{ $port_count ->
        [0] { "" }
       *[other] { " " }روی درگاه { $ports }
    } برای { $program }، در { $network_count ->
        [0] همه شبکه‌ها
       *[other] شبکه‌های { $networks }
    }
# { $server } is the file name of what Explorer loads. { $scope } is whom it is set up for: everyone on this PC, or
# you. It is not shown itself.
core-review-shell-extension-detail =
    { $server } را در کاوشگر فایل بارگذاری می‌کند، { $scope ->
        [everyone] برای همه کاربران این رایانه
       *[you] برای شما
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shell-extension-detail-own-file =
    فایلی را که نصب می‌کند در کاوشگر فایل بارگذاری می‌کند، { $scope ->
        [everyone] برای همه کاربران این رایانه
       *[you] برای شما
    }
# The name of a shell extension: { $handler } is its name in the registry, and { $class } the kind of file or folder it
# is for, such as * or Directory.
core-policy-shell-extension-name = { $handler } روی { $class }
# { $on } is the kind of file or folder whose menu it is, as the registry names it, such as * or
# Directory. { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-menu-command-detail =
    فرمانی به منوی { $on } اضافه می‌کند، { $scope ->
        [everyone] برای همه کاربران این رایانه
       *[you] برای شما
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-default-verb-detail =
    آنچه با باز کردن یکی از این‌ها انجام می‌شود را تغییر می‌دهد، { $scope ->
        [everyone] برای همه کاربران این رایانه
       *[you] برای شما
    }
core-review-client-entry-detail = می‌توان آن را به‌عنوان برنامه این کار انتخاب کرد. هیچ پیش‌فرضی به آن تغییر داده نمی‌شود
core-review-com-permissions-title = مجوزهای COM
core-review-com-permissions-detail = تعیین می‌کند چه کسی می‌تواند کلاس‌های COM آن را شروع کند، یا آن کلاس‌ها با چه حسابی اجرا شوند
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-link-type-detail =
    پیوندهای این نوع با این برنامه باز می‌شوند، { $scope ->
        [everyone] برای همه کاربران این رایانه
       *[you] برای شما
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-file-type-detail =
    فایل‌های این نوع با این برنامه باز می‌شوند، { $scope ->
        [everyone] برای همه کاربران این رایانه
       *[you] برای شما
    }
# { $log } is the name of a Windows event log, such as Application.
core-review-event-log-source-detail = می‌تواند در گزارش { $log } بنویسد، با پیام‌هایی از فایل‌های خودش
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-font-detail =
    هنگام ورود به سیستم از پوشه نصب بارگذاری می‌شود، { $scope ->
        [everyone] برای همه کاربران این رایانه
       *[you] برای شما
    }
core-review-odbc-driver-detail = در برنامه‌هایی بارگذاری می‌شود که برای دسترسی به پایگاه داده از ODBC استفاده می‌کنند
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-odbc-source-detail =
    یک اتصال پایگاه داده، { $scope ->
        [everyone] برای همه کاربران این رایانه
       *[you] برای شما
    }
core-review-default-apps-detail = می‌توان آن را در «برنامه‌های پیش‌فرض» انتخاب کرد. هیچ پیش‌فرضی به آن تغییر داده نمی‌شود
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-program-name-detail =
    «اجرا» و برنامه‌های دیگر می‌توانند آن را با این نام پیدا کنند، { $scope ->
        [everyone] برای همه کاربران این رایانه
       *[you] برای شما
    }
# { $server } is the file name of what serves the classes.
core-review-com-title = COM در { $server }
core-review-com-title-own-file = COM در فایلی که نصب می‌کند
# { $count } is how many registry values.
core-review-com-detail =
    { $count ->
        [one] { $count } تنظیم برای کلاس‌هایی که برنامه‌های دیگر می‌توانند شروع کنند
       *[other] { $count } تنظیم برای کلاس‌هایی که برنامه‌های دیگر می‌توانند شروع کنند
    }
core-review-com-support-title = کتابخانه‌های نوع، رابط‌ها و AppIDهای COM
# { $libraries }, { $interfaces }, and { $app_ids } are how many registry values there are for its type libraries, its
# interfaces, and its AppIDs.
core-review-com-support-detail = تنظیمات کتابخانه‌های نوع ({ $libraries })، رابط‌ها ({ $interfaces }) و AppIDها ({ $app_ids })، که همراه کلاس‌های آن هستند
core-review-compatibility-title = نحوه شروع فایل‌های اجرایی آن
# { $count } is how many registry values.
core-review-compatibility-detail =
    { $count ->
        [one] { $count } تنظیم سازگاری برای فایل‌های اجرایی که نصب می‌کند
       *[other] { $count } تنظیم سازگاری برای فایل‌های اجرایی که نصب می‌کند
    }

## The installer's notes on whose folders and settings these are, and what is not coming.

# { $account } is the name of a Windows account, such as alice.
core-review-account-signed-in = داده‌های کاربر و تنظیمات ویژه هر کاربر به { $account } می‌رسد، که در اینجا وارد سیستم شده است، و نه به سرپرستی که این برنامه با حساب او اجرا می‌شود.
# { $account } is the name of a Windows account, such as alice.
core-review-account-no-desktop = هیچ کسی به این نشست وارد نشده است، بنابراین داده‌های کاربر و تنظیمات ویژه هر کاربر به { $account } می‌رسد، حسابی که این برنامه با آن اجرا می‌شود.
# { $reason } is what Windows said, in its own words.
core-review-account-not-told = مشخص نشد چه کسی وارد سیستم شده است ({ $reason })، بنابراین تنظیمات ویژه هر کاربر کنار گذاشته می‌شود و داده‌های کاربر زیر محل نصب قرار می‌گیرد.
# { $reason } is what Windows said, in its own words.
core-review-signed-in-unknown = مشخص نشد چه کسی وارد سیستم شده است: { $reason }
# Headings of what is left out, shared by the app's list and the installer's notes.
core-review-left-out-credentials = در این نصب‌کننده نیست، زیرا شبیه اعتبارنامه است
core-review-left-out-settings = تنظیمات و تاریخچه‌ای که یک حساب برای برنامه نگه می‌دارد، که فقط با گزینه «داده‌های کاربر» گنجانده می‌شود
core-review-left-out-certificates = به نظر می‌رسد برنامه به گواهی‌هایی متکی است که در رایانه‌اش مورد اعتماد بودند، و آن‌ها نصب نمی‌شوند
core-review-left-out-fonts = این قلم‌ها به نام برنامه هستند و چیزی نمی‌گوید که متعلق به خود آن باشند، بنابراین نصب نمی‌شوند
# { $names } is a list of names, such as the names of saved passwords.
core-review-note-credentials = در این نصب‌کننده نیست، زیرا شبیه اعتبارنامه است: { $names }. آنچه از آن‌ها در این رایانه هست همان‌طور که هست می‌ماند، و شاید لازم باشد آن‌ها را دوباره وارد کنید.
# { $names } is a list of names of license data.
core-review-note-licenses = در این نصب‌کننده نیست، زیرا گزینه «داده‌های مجوز و فعال‌سازی» هنگام ساخت آن خاموش بود: { $names }.
# { $names } is a list of places an account keeps settings, such as folders.
core-review-note-settings = در این نصب‌کننده نیست، زیرا گزینه «داده‌های کاربر» هنگام ساخت آن خاموش بود: تنظیمات و تاریخچه‌ای که یک حساب برای برنامه نگه می‌دارد، در { $names }. برنامه در اینجا مانند اولین استفاده یک کاربر تازه شروع می‌شود.
# { $names } is a list of certificates, each as core-review-certificate.
core-review-note-certificates = به نظر می‌رسد برنامه به گواهی‌هایی متکی است که در رایانه‌اش مورد اعتماد بودند، و آن‌ها نصب نمی‌شوند: { $names }. تصمیم به اعتماد کردن به آن‌ها در اینجا با شماست.
# { $names } is a list of font names.
core-review-note-fonts = این قلم‌ها به نام برنامه هستند و چیزی نمی‌گوید که متعلق به خود آن باشند، بنابراین نصب نمی‌شوند: { $names }.
# { $names } is a list of names.
core-review-note-left-out-by-choice = کسی که این نصب‌کننده را ساخت این موارد را کنار گذاشت: { $names }.
# { $names } is a list of names.
core-review-note-added-by-choice = کسی که این نصب‌کننده را ساخت این موارد را اضافه کرد، زیرا آن‌ها را بخشی از برنامه دانست: { $names }.
# { $name } is a certificate's subject, such as CN=Contoso, and { $store } the certificate store it was in, such as
# Root.
core-review-certificate = { $name } در { $store }
# { $names } is a list of names, and { $count } how many more there are.
core-review-names-and-more = { $names } و { $count } مورد دیگر

## Why something in a package is left out. Each is the second half of a sentence of core-warning-review-left-out.

core-review-no-account = متعلق به تنظیمات یک حساب است و مشخص نشد چه کسی وارد سیستم شده است
core-review-name-not-accepted = Windows نام آن را نمی‌پذیرد
# { $error } is what Windows said, in its own words.
core-review-service-unreadable = تنظیمات فعلی آن خوانده نشد: { $error }
core-review-service-dependency = به سرویسی با نام نامعتبر وابسته است
# { $path } is the program the service runs.
core-review-service-runs-other = { $path } را اجرا می‌کند، که این بسته آن را نصب نمی‌کند
core-review-service-settings = تنظیمات آن چیزی را نام می‌برد که این بسته نصب نمی‌کند، یا کلیدی را که متعلق به خودش نیست
core-review-driver-no-hash = هشی برای بررسی آن ندارد
core-review-license-for-the-machine = مجوزی که برای کل رایانه ذخیره شده است فقط گزارش می‌شود
core-review-license-starts-something = مجوز دلیلی برای شروع کردن چیزی ندارد
core-review-task-not-captured = تعریف آن گرفته نشد
# { $error } is what was wrong with the task's XML, in English.
core-review-task-unreadable = تعریف آن خوانده نشد: { $error }
core-review-task-no-account = با حساب هر کسی که آن را نصب کند اجرا می‌شود، و مشخص نشد چه کسی در اینجا وارد سیستم شده است
# { $path } is what the shortcut opens.
core-review-shortcut-opens-other = { $path } را باز می‌کند، که این بسته آن را نصب نمی‌کند
# { $service } is the service's name, and { $path } the program it runs now.
core-policy-service-exists = { $service } از قبل در این رایانه وجود دارد و { $path } را اجرا می‌کند، که از یک نصب قبلی در اینجا نیست. اگر نسخه قدیمی‌تری از این برنامه است، ابتدا آن را حذف نصب کنید
core-policy-key-control-characters = نام کلید آن نویسه‌های کنترلی دارد
core-policy-value-control-characters = نام مقدار آن نویسه‌های کنترلی دارد
core-policy-registry-link = یک پیوند رجیستری است
# { $reason } is why the key it stands for is left out, such as "no app install should change this key".
core-policy-virtual-store = جانشین یکی از کلیدهای رایانه است و { $reason }
core-policy-registry-part = هیچ نصب برنامه‌ای نباید در این بخش از رجیستری بنویسد
core-policy-key-refused = هیچ نصب برنامه‌ای نباید این کلید را تغییر دهد
core-policy-net-debugger = برای همه برنامه‌های .NET یک اشکال‌زدا تعیین می‌کند
core-policy-net-install-root = محلی را که همه برنامه‌های .NET از آن .NET را بارگذاری می‌کنند تغییر می‌دهد
core-policy-class-redirects = نحوه باز شدن نوعی برنامه یا پوشه را تغییر می‌دهد
core-policy-app-path-name = برنامه‌ای را با چیزی غیر از نام یک فایل نام‌گذاری می‌کند
core-policy-app-path-not-installed = نام برنامه‌ای را برای فایلی ثبت می‌کند که این بسته نصب نمی‌کند
core-policy-shared-default-verb = آنچه با باز کردن هر فایل یا پوشه از یک نوع انجام می‌شود را تغییر می‌دهد
core-policy-command-runs-other = فرمانی اضافه می‌کند که چیزی را اجرا می‌کند که این بسته نصب نمی‌کند
core-policy-autostart-not-command = به‌طور خودکار شروع می‌شود و مقدار آن یک فرمان نیست
core-policy-autostart-runs-other = به‌طور خودکار شروع می‌شود و چیزی را اجرا می‌کند که این بسته نصب نمی‌کند
core-policy-environment-own-rules = متغیرهای محیطی با قواعد خودشان بازیابی می‌شوند
core-policy-service-not-restored = متعلق به سرویسی است که این بسته بازیابی نمی‌کند
core-policy-service-own-rules = تنظیمات خود یک سرویس فقط از طریق قواعد آن بازیابی می‌شوند
core-policy-service-dll = ServiceDll آن فایلی نیست که این بسته نصب کند
core-policy-event-log-source-only = فقط مقادیر خود یک منبع گزارش رویداد قابل بازیابی است
core-policy-security-log = هیچ برنامه‌ای در گزارش امنیتی نمی‌نویسد
core-policy-event-log-messages = پیام‌های آن از فایلی می‌آید که این بسته نصب نمی‌کند
core-policy-type-library = کتابخانه نوع آن فایلی نیست که این بسته نصب کند
# { $name } is a program's file name, such as app.exe.
core-policy-application-opened = نحوه باز شدن { $name } را تغییر می‌دهد، که این بسته آن را نصب نمی‌کند
core-policy-event-log-no-file = منبع گزارش رویداد باید پیام‌هایش را از یک فایل بگیرد
core-policy-com-runs-other = چیزی را اجرا می‌کند که فایلی نیست که این بسته نصب کند
core-policy-com-redirect = این کلاس را به چیزی تبدیل می‌کند که این بسته نصب نمی‌کند
core-policy-com-server = چیزی را اجرا می‌کند که نه فایلی است که این بسته نصب می‌کند و نه بخشی از Windows
core-policy-interface = نحوه انجام فراخوانی‌های یک رابط را با چیزی تعیین می‌کند که این بسته نصب نمی‌کند
# { $name } is a program's file name, such as app.exe.
core-policy-app-id-started = نحوه شروع { $name } را تغییر می‌دهد، که این بسته آن را نصب نمی‌کند
core-policy-app-id-surrogate = توسط برنامه‌ای میزبانی می‌شود که این بسته نصب نمی‌کند
core-policy-app-id-service = در سرویسی اجرا می‌شود که این بسته بازیابی نمی‌کند
core-policy-app-id-run-as = با حساب مخصوص خود اجرا می‌شود
core-policy-app-id-link = به هیچ AppID اشاره نمی‌کند
core-policy-app-id-remote = جایی غیر از این رایانه اجرا می‌شود
core-policy-odbc-own-driver = آن تنظیم خود ODBC است و درایور نیست
core-policy-odbc-driver-not-installed = درایور آن فایلی نیست که این بسته نصب کند
core-policy-loads-other-file = فایلی را بارگذاری می‌کند که این بسته نصب نمی‌کند
core-policy-odbc-own-source = آن تنظیم خود ODBC است و منبع داده نیست
core-policy-odbc-source-missing = منبع داده‌ای را فهرست می‌کند که این بسته ندارد
core-policy-odbc-source-driver = درایور آن نه فایلی است که این بسته نصب می‌کند و نه بخشی از Windows
core-policy-shell-extension = افزونه پوسته‌ای را در کاوشگر فایل بارگذاری می‌کند که فایلی نیست که این بسته نصب کند
core-policy-not-text = محتوای آن متنی نیست که بتوان آن را به شکل مورد انتظار خواند
core-policy-network-place = مکانی در شبکه را نام می‌برد
core-policy-font = قلمی را ثبت می‌کند که این بسته نصب نمی‌کند، یا قلمی در پوشه Windows
core-policy-compatibility = نحوه اجرای برنامه‌ای را تغییر می‌دهد که این بسته نصب نمی‌کند
core-policy-capabilities = برنامه‌ای را فهرست می‌کند که جزئیات آن در این بسته نیست
core-policy-default-program = برنامه‌ای را که به‌طور پیش‌فرض چیزها را باز می‌کند تغییر می‌دهد
core-policy-url-protocol = می‌گوید یک نوع پیوند است، اما با هیچ چیزی که این بسته نصب می‌کند باز نمی‌شود
core-policy-command-handler = فرمانی اضافه می‌کند که چیزی آن را پردازش می‌کند که این بسته نصب نمی‌کند
core-policy-installer-command = چیزی را اجرا می‌کند که فقط Windows Installer می‌تواند بخواند
core-policy-creates-class = کلاسی ایجاد می‌کند که این بسته نصب نمی‌کند
core-policy-opens-links = پیوندها را با چیزی باز می‌کند که این بسته نصب نمی‌کند
core-policy-opens-files = فایل‌ها را با چیزی باز می‌کند که این بسته نصب نمی‌کند
core-policy-firewall-name = نام آن از نام‌هایی نیست که این برنامه بتواند منتقل کند
core-policy-firewall-description = توضیح آن از توضیحاتی نیست که این برنامه بتواند منتقل کند
core-policy-firewall-every-program = برای همه برنامه‌ها اعمال می‌شود، در حالی که یک قانون فقط می‌تواند برای یک برنامه باشد
# { $path } is the program the rule is for.
core-policy-firewall-program = برای { $path } است، که این بسته آن را نصب نمی‌کند
core-policy-firewall-direction = جهتی را که این برنامه بتواند تنظیم کند مشخص نمی‌کند
core-policy-firewall-action = قوانین دیگر را نادیده می‌گیرد، کاری که قانون هیچ برنامه‌ای نباید انجام دهد
# { $setting } is a part of a firewall rule, as the scan of its PC described it.
core-policy-firewall-not-carried = می‌گوید { $setting }، که نمی‌شد همان‌طور تنظیم کرد، یا کار قوانین دیگر را تضعیف می‌کرد
core-policy-firewall-service = برای سرویسی است که این بسته بازیابی نمی‌کند
core-policy-firewall-parts = بخشی از آن چیزی نیست که یک قانون دیوار آتش بتواند بیان کند
core-policy-folder-already-there = آن پوشه از قبل، بیرون از محل نصب، وجود داشت و چیزی نمی‌گوید که متعلق به این برنامه است، بنابراین هیچ فایل اجرایی در آن قرار نمی‌گیرد
core-policy-variable-name = نام آن یک نام متغیر معتبر نیست
core-policy-variable-relied-on = Windows و برنامه‌های دیگر به این متغیر وابسته‌اند
core-policy-task-name = نام آن از نام‌هایی نیست که Windows می‌پذیرد
core-policy-task-folder = پوشه آن از پوشه‌هایی نیست که Windows می‌پذیرد
core-policy-task-windows = یکی از کارهای خود Windows را جایگزین می‌کند
core-policy-task-other-action = کاری غیر از اجرای یک برنامه انجام می‌دهد
core-policy-task-no-program = هیچ برنامه‌ای را برای اجرا نام نمی‌برد
# { $path } is the program the task runs.
core-policy-task-runs-other = چیزی را اجرا می‌کند که این بسته نصب نمی‌کند: { $path }
core-policy-task-exists-unreadable = کاری با این نام از قبل وجود دارد و خوانده نشد
core-policy-task-exists-other = کاری با این نام از قبل وجود دارد و برنامه را اجرا نمی‌کند
# { $path } is the program the task already on this PC runs.
core-policy-task-exists-runs = کاری با این نام از قبل وجود دارد و { $path } را اجرا می‌کند، که از یک نصب قبلی در اینجا نیست
core-policy-shortcut-not-lnk = یک میان‌بر .lnk نیست
core-policy-shortcut-place = در منوی شروع یا روی میز کار نیست
core-policy-short-name = مسیر آن از نامی استفاده می‌کند که قابل بررسی نیست
core-policy-shortcut-no-folder = این رایانه چنین پوشه‌ای برای آن ندارد
core-policy-data-startup = در پوشه راه‌اندازی قرار می‌گیرد و هنگام هر بار ورود به سیستم اجرا می‌شود
core-policy-data-powershell = در جایی قرار می‌گیرد که PowerShell هنگام شروع، هر چه را آنجا پیدا کند اجرا می‌کند
core-policy-data-windows = در پوشه‌ای قرار می‌گیرد که متعلق به Windows است
core-policy-license-outside = بیرون از محل نصب و همه پوشه‌های داده شناخته‌شده است
core-rewrite-cdata = یک بخش CDATA دارد
core-rewrite-too-much-to-look-through = این بسته آن‌قدر تنظیمات دارد که نمی‌توان بررسی کرد چه چیزهایی را نام می‌برند

## Warnings a review gives, which the installer shows with what it sets up.

# { $kind } says what was left out, and is not shown itself. { $item } is its path or name, or a registry value such
# as HKLM\Software\App\Name. { $reason } is why, as the second half of the sentence, such as "it cannot be a whole
# drive".
core-warning-review-left-out =
    { $kind ->
        [file] فایل { $item } کنار گذاشته شد: { $reason }.
        [folder] پوشه { $item } کنار گذاشته شد: { $reason }.
        [service] سرویس { $item } کنار گذاشته شد: { $reason }.
        [driver] درایور { $item } کنار گذاشته شد: { $reason }.
        [firewall-rule] قانون دیوار آتش { $item } کنار گذاشته شد: { $reason }.
        [license-data] داده‌های مجوز در { $item } کنار گذاشته شد: { $reason }.
        [license-value] مقدار مجوز { $item } کنار گذاشته شد: { $reason }.
        [scheduled-task] کار زمان‌بندی‌شده { $item } کنار گذاشته شد: { $reason }.
        [shortcut] میان‌بر { $item } کنار گذاشته شد: { $reason }.
        [environment-variable] متغیر محیطی { $item } کنار گذاشته شد: { $reason }.
       *[value] { $item } کنار گذاشته شد: { $reason }.
    }
core-warning-review-package-path-missing = یک فایل قابل کپی مسیر نسبی در بسته ندارد.
# { $count } is the most empty folders a package makes.
core-warning-review-too-many-empty-folders = این بسته بیش از { $count } پوشه خالی را فهرست می‌کند و فقط همین تعداد ایجاد می‌شود.
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Documents folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-documents-elsewhere =
    { $count ->
        [one] { $count } فایل از اسناد در عوض زیر { $target }، در پوشه‌ای به نام _external، قرار می‌گیرد، زیرا پوشه اسناد این رایانه در { $folder } است و فقط در پوشه‌ای درون نمایه شما نوشته می‌شود. برای استفاده از آن، خودتان آن را به پوشه اسناد خود منتقل کنید.
       *[other] { $count } فایل از اسناد در عوض زیر { $target }، در پوشه‌ای به نام _external، قرار می‌گیرند، زیرا پوشه اسناد این رایانه در { $folder } است و فقط در پوشه‌ای درون نمایه شما نوشته می‌شود. برای استفاده از آن‌ها، خودتان آن‌ها را به پوشه اسناد خود منتقل کنید.
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-documents-unknown =
    { $count ->
        [one] { $count } فایل از اسناد در عوض زیر { $target }، در پوشه‌ای به نام _external، قرار می‌گیرد، زیرا مشخص نشد پوشه اسناد این رایانه کجاست. برای استفاده از آن، خودتان آن را به پوشه اسناد خود منتقل کنید.
       *[other] { $count } فایل از اسناد در عوض زیر { $target }، در پوشه‌ای به نام _external، قرار می‌گیرند، زیرا مشخص نشد پوشه اسناد این رایانه کجاست. برای استفاده از آن‌ها، خودتان آن‌ها را به پوشه اسناد خود منتقل کنید.
    }
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Saved Games folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-elsewhere =
    { $count ->
        [one] { $count } فایل از بازی‌های ذخیره‌شده در عوض زیر { $target }، در پوشه‌ای به نام _external، قرار می‌گیرد، زیرا پوشه بازی‌های ذخیره‌شده این رایانه در { $folder } است و فقط در پوشه‌ای درون نمایه شما نوشته می‌شود. برای استفاده از آن، خودتان آن را به پوشه بازی‌های ذخیره‌شده خود منتقل کنید.
       *[other] { $count } فایل از بازی‌های ذخیره‌شده در عوض زیر { $target }، در پوشه‌ای به نام _external، قرار می‌گیرند، زیرا پوشه بازی‌های ذخیره‌شده این رایانه در { $folder } است و فقط در پوشه‌ای درون نمایه شما نوشته می‌شود. برای استفاده از آن‌ها، خودتان آن‌ها را به پوشه بازی‌های ذخیره‌شده خود منتقل کنید.
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-unknown =
    { $count ->
        [one] { $count } فایل از بازی‌های ذخیره‌شده در عوض زیر { $target }، در پوشه‌ای به نام _external، قرار می‌گیرد، زیرا مشخص نشد پوشه بازی‌های ذخیره‌شده این رایانه کجاست. برای استفاده از آن، خودتان آن را به پوشه بازی‌های ذخیره‌شده خود منتقل کنید.
       *[other] { $count } فایل از بازی‌های ذخیره‌شده در عوض زیر { $target }، در پوشه‌ای به نام _external، قرار می‌گیرند، زیرا مشخص نشد پوشه بازی‌های ذخیره‌شده این رایانه کجاست. برای استفاده از آن‌ها، خودتان آن‌ها را به پوشه بازی‌های ذخیره‌شده خود منتقل کنید.
    }

## Errors when a folder that only administrators may change cannot be made so, or checked.

# { $path } is a folder.
core-privilege-not-a-folder = { $path } یک پیوند یا فایل است، نه پوشه
# { $path } is a folder. { $error } is what Windows said, in its own words.
core-privilege-not-made-here = { $path } را WinAppMigrator ایجاد نکرده است و دیگران می‌توانند محتوای آن را تغییر دهند. یک سرپرست باید آن را بررسی و حذف کند: { $error }
# { $path } is a folder.
core-privilege-others-put-something-in = { $path } چیزی در خود دارد که WinAppMigrator ایجاد نکرده است و دیگران می‌توانند آن را تغییر دهند. یک سرپرست باید آن را بررسی و حذف کند
# { $path } is a folder.
core-privilege-became-a-link-in-use = { $path } در حین استفاده به پیوند تبدیل شد
# { $path } is a folder.
core-privilege-became-a-link-looked-at = { $path } در حین بررسی به پیوند تبدیل شد
# { $path } is a file. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-users-cannot-start = اجازه شروع { $path } به کاربران داده نشد: { $said }
core-privilege-elevation-unknown = مشخص نشد که آیا این فرایند با حقوق سرپرستی اجرا می‌شود یا نه
# { $path } is a folder. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-cannot-restrict = دسترسی به { $path } به سرپرستان محدود نشد: { $said }
# { $path } is a folder. { $said } is the folder's permissions as Windows wrote them, and what else it printed.
core-privilege-not-restricted = دسترسی به { $path } به سرپرستان محدود نشد و شاید درایو آن مجوزها را نگه ندارد: { $said }
# { $path } is a folder.
core-privilege-written-to-first = فرایند دیگری پیش از محافظت از { $path } در آن نوشت
core-privilege-no-program-data = Windows نمی‌گوید ProgramData کجاست

## Checks of a package's files, and of what an install put in place. Each is said of one file, value, service, or the
## like, which the report names beside it.

# { $path } is the installer's file.
core-verify-no-package = { $path } هیچ بسته‌ای در خود ندارد
# { $path } is a folder.
core-verify-not-unpacked = بسته این نصب‌کننده در { $path } استخراج نشد
core-verify-schema-supported = طرح‌واره مانیفست بسته پشتیبانی می‌شود.
core-verify-packaged-file-missing = فایل بسته‌بندی‌شده وجود ندارد.
core-verify-no-package-path = فایل قابل کپی مسیر نسبی در بسته ندارد.
core-verify-folder-there = پوشه خالی وجود دارد.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-unreadable = برای بررسی خوانده نشد: { $error }
core-verify-restored-file-missing = فایل بازیابی‌شده وجود ندارد.
# { $expected } and { $actual } are sizes in bytes.
core-verify-size-mismatch = اندازه فایل مطابقت ندارد. انتظار می‌رفت { $expected } بایت باشد، اما { $actual } بایت است.
core-verify-no-hash = مانیفست هیچ هشی برای این فایل ثبت نکرده است، بنابراین نمی‌توان آن را بررسی کرد.
core-verify-hash-mismatch = هش SHA-256 مطابقت ندارد.
core-verify-file-matches = اندازه و هش فایل با مانیفست مطابقت دارد.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-payload-failed = تأیید محتوای مجوز ناموفق بود: { $error }
core-verify-license-payload-missing = محتوای مجوز بسته‌بندی‌شده وجود ندارد.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-failed = تأیید انبار مجوز ناموفق بود: { $error }
core-verify-license-store-file-missing = فایل انبار مجوز بسته‌بندی‌شده وجود ندارد.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-path-invalid = مسیر انبار مجوز بسته‌بندی‌شده نامعتبر است: { $error }
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-prerequisite-failed = تأیید محتوای پیش‌نیاز ناموفق بود: { $error }
core-verify-prerequisite-missing = محتوای پیش‌نیاز همراه بسته وجود ندارد.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-failed = تأیید تصویر درایور ناموفق بود: { $error }
core-verify-bundled-driver-missing = تصویر درایور همراه بسته وجود ندارد.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-path-invalid = مسیر درایور همراه بسته نامعتبر است: { $error }
core-verify-prerequisite-present = پیش‌نیاز زمان اجرا در رایانه مقصد موجود است.
core-verify-prerequisite-not-detected = پیش‌نیاز زمان اجرا شناسایی نشد؛ شاید به راه‌اندازی مجدد یا تکمیل دستی نیاز داشته باشد (برای نتیجه قطعی، گزارش بازیابی را ببینید).
core-verify-prerequisite-not-probed = وجود پیش‌نیاز از طریق رجیستری قابل بررسی نیست؛ نصب با بیشترین تلاش ممکن انجام شد.
core-verify-license-file-missing = فایل مجوز بازیابی‌شده وجود ندارد.
core-verify-license-store-restored-missing = فایل انبار مجوز بازیابی‌شده وجود ندارد.
core-verify-registry-key-there = کلید رجیستری وجود دارد.
core-verify-registry-value-matches = مقدار رجیستری با داده‌های بایگانی‌شده مطابقت دارد.
core-verify-registry-value-differs = مقدار رجیستری با داده‌های بایگانی‌شده مطابقت ندارد.
core-verify-firewall-rule-there = دیوار آتش Windows این قانون را دارد.
core-verify-variable-matches = مقدار متغیر محیطی با داده‌های بایگانی‌شده مطابقت دارد.
core-verify-variable-differs = مقدار متغیر محیطی با داده‌های بایگانی‌شده مطابقت ندارد.
# { $error } is what Windows said, in its own words.
core-verify-variable-unreadable = متغیر محیطی وجود ندارد یا قابل خواندن نیست: { $error }
core-verify-service-runs-other = سرویس وجود دارد اما فایل اجرایی بازیابی‌شده را با آرگومان‌هایش اجرا نمی‌کند.
core-verify-service-other-account = سرویس وجود دارد اما با حساب مورد انتظار اجرا نمی‌شود.
core-verify-service-matches = سرویس وجود دارد و فایل اجرایی بازیابی‌شده را با آرگومان‌ها و حساب مورد انتظار اجرا می‌کند.
core-verify-service-missing = سرویس وجود ندارد.
# { $error } is what Windows said, in its own words.
core-verify-service-unreadable = سرویس قابل خواندن نیست: { $error }
core-verify-driver-other-image = سرویس درایور به تصویر بازیابی‌شده اشاره نمی‌کند.
core-verify-driver-not-registered = سرویس درایور همراه بسته پس از بازیابی ثبت نشده است.
# { $error } is what Windows said, in its own words.
core-verify-driver-unreadable = سرویس درایور قابل خواندن نیست: { $error }
core-verify-driver-image-missing = تصویر درایور وجود ندارد.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-unchecked = تصویر درایور بررسی نشد: { $error }
core-verify-task-matches = کار زمان‌بندی‌شده وجود دارد و برنامه‌های بازیابی‌شده را اجرا می‌کند.
core-verify-task-runs-other = کار زمان‌بندی‌شده وجود دارد اما برنامه‌های بازیابی‌شده را اجرا نمی‌کند.
core-verify-task-missing = کار زمان‌بندی‌شده وجود ندارد.
# { $error } is what Windows said, in its own words.
core-verify-task-unreadable = پرس‌وجوی کار زمان‌بندی‌شده ناموفق بود: { $error }
core-verify-shortcut-matches = میان‌بر وجود دارد و مسیر مقصد آن با مقصد بازیابی‌شده مطابقت دارد.
core-verify-shortcut-opens-other = میان‌بر وجود دارد اما مسیر مقصد آن با مقصد بازیابی‌شده مطابقت ندارد.
core-verify-shortcut-missing = میان‌بر وجود ندارد.
# { $error } is what Windows said, in its own words.
core-verify-shortcut-unreadable = پرس‌وجوی میان‌بر ناموفق بود: { $error }

## Warnings and errors a check gives, which the app and the installer show.

core-warning-verify-schema-unsupported = طرح‌واره مانیفست بسته پشتیبانی نمی‌شود.
# { $error } is what went wrong, in Windows' or the program's own words.
core-warning-verify-package-file-failed = تأیید فایل بسته ناموفق بود: { $error }
core-warning-verify-folder-missing = پوشه خالی وجود ندارد.
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-key-missing = کلید رجیستری وجود ندارد یا قابل خواندن نیست: { $error }
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-value-missing = مقدار رجیستری وجود ندارد یا قابل خواندن نیست: { $error }
core-warning-verify-registry-kind-missing = نوع مقدار رجیستری مشخص نیست.
core-warning-verify-firewall-rule-missing = دیوار آتش Windows این قانون را ندارد.
core-warning-verify-service-scope = تأیید سرویس، پیکربندی سرویس را از HKLM می‌خواند و سرویس‌ها را شروع نمی‌کند.
core-warning-verify-driver-manual = WinAppMigrator یک یا چند درایور سازنده یا داخلی Windows را توزیع نمی‌کند؛ آن‌ها را با بسته درایور امضاشده سازنده در رایانه مقصد دوباره نصب کنید.
core-warning-verify-task-scope = تأیید کار زمان‌بندی‌شده فقط ثبت و ارجاع فرمان را بررسی می‌کند.

## The verification report written beside a package, verification_report.md. Its headings follow a "#".

core-verify-report-title = گزارش تأیید WinAppMigrator
# { $time } is when, as a date and time in UTC, such as 2026-11-15T10:00:00Z.
core-verify-report-verified = زمان تأیید (UTC): { $time }
# { $valid } is true or false, which are not translated.
core-verify-report-package-valid = بسته معتبر: { $valid }
# { $valid } is true or false, which are not translated.
core-verify-report-restore-valid = بازیابی معتبر: { $valid }
core-verify-report-restore-not-run = بازیابی معتبر: اجرا نشد
core-verify-report-items = موارد
# { $message } is what a check found, and { $source } the file or setting it looked at.
core-verify-report-item-from = { $message } منبع: { $source }
core-verify-report-warnings = هشدارها
core-verify-report-errors = خطاها
core-verify-report-none = هیچ.
