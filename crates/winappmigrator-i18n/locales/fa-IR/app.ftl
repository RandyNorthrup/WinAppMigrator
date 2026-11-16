# Words of the app, WinAppMigrator itself: its one window, with the installed apps listed on the left and, on the
# right, what goes in an installer of the one picked and the button that builds it; and the steps to read another PC
# through WinAppMigrator Client. Message names start with app-.


## The window and its header

# The program's name, in the window's title bar and at the top of the window. Keep it as it is unless your language
# writes product names in a script of its own.
app-name = WinAppMigrator
app-tagline = یک برنامه نصب‌شده را به نصب‌کننده خوداستخراج تبدیل کنید
app-theme-to-light = تغییر به طرح زمینه روشن
app-theme-to-dark = تغییر به طرح زمینه تیره
app-reload-list = بارگذاری دوباره فهرست برنامه‌ها
app-list-loading = در حال بارگذاری
# { $count } is how many apps are listed.
app-list-count =
    { $count ->
        [one] { $count } برنامه
       *[other] { $count } برنامه
    }
app-list-load-failed = بارگذاری ناموفق بود
# Shown when the window cannot open at all. { $error } is what went wrong, in the words of Windows or a graphics driver.
app-window-could-not-open = پنجره باز نشد: { $error }

## The list of installed apps

app-list-search = جستجو بر اساس نام یا ناشر  (Ctrl+F)
app-list-filter-all = همه
# Whether an app is installed for one account or for everyone on the PC.
app-scope-per-user = ویژه کاربر
app-scope-per-machine = ویژه رایانه
app-scope-unknown = نامشخص
# An app whose processor architecture (x86, x64, ARM64) is not known.
app-arch-unknown = نامشخص
app-list-loading-apps = در حال بارگذاری برنامه‌های نصب‌شده…
# { $error } is why the apps could not be listed.
app-list-could-not-list = فهرست برنامه‌ها تهیه نشد: { $error }
app-list-no-matches = برنامه منطبقی وجود ندارد
app-list-no-matches-hint = متن جستجو یا فیلتر دامنه را تغییر دهید.
app-list-worker-disconnected = ارتباط با فرایند فهرست‌برداری قطع شد.
# What a screen reader says of an app's row. { $name } is the app's name, { $publisher } who made it, and { $version } its
# version, such as 11.4.2, or — when either is not known.
app-list-row = { $name }، { $publisher }، نسخه { $version }

## The list's columns, and what a screen reader says each column's heading does when clicked

app-column-name = نام
app-column-name-sort = مرتب‌سازی بر اساس نام
app-column-publisher = ناشر
app-column-publisher-sort = مرتب‌سازی بر اساس ناشر
app-column-version = نسخه
app-column-version-sort = مرتب‌سازی بر اساس نسخه
app-column-scope = دامنه
app-column-scope-sort = مرتب‌سازی بر اساس دامنه
# Short for architecture: x86, x64, or ARM64. The column is narrow.
app-column-arch = معماری
app-column-arch-sort = مرتب‌سازی بر اساس معماری

## Where apps are read from: this PC, a drive from another PC, a disk image of one, or another PC on the network

app-source = منبع اسکن
app-source-this-pc = این رایانه
app-source-drive = درایو متصل…
app-source-image = تصویر دیسک…
app-source-network = رایانه شبکه…
app-source-attaching-image = در حال پیوست کردن تصویر به‌صورت فقط‌خواندنی…
# { $computer } is the other PC's name, such as OLD-PC, and { $address } its network address, such as 192.168.1.20.
app-source-network-pc = { $computer } از طریق شبکه ({ $address })
app-source-disconnect = قطع اتصال
app-source-change = تغییر…
# The title of the window that picks a disk image. .vhd and .vhdx are file extensions.
app-source-pick-image-title = انتخاب تصویر دیسک رایانه دیگر (.vhd یا .vhdx)
app-source-pick-image-filter = تصاویر دیسک
# { $path } is the disk image file.
app-source-image-has-no-windows = { $path } هیچ حجمی ندارد که Windows روی آن باشد.
app-source-image-not-attached = تصویر دیسک پیوست نشد.
app-source-pick-drive-title = انتخاب ریشه درایو متصل (مثلاً E:\)
# { $path } is the folder picked, such as E:\. \Windows and \Users are folder names, as Windows writes them.
app-source-not-a-windows-drive = { $path } شبیه درایو سیستمی Windows نیست (\Windows و \Users ندارد). ریشه درایوی را انتخاب کنید که از رایانه دیگر آمده است.
# Said of a drive or disk image with a Windows older than Windows 10. { $windows } is its name, such as "Windows 7
# Professional Service Pack 1, 32-bit". { $is_32_bit } is "yes" when that Windows is 32-bit, which adds the last
# sentence, and "no" when it is not.
app-source-older-windows =
    روی این درایو { $windows } نصب است. نصب‌کننده‌هایی که از آن ساخته شوند برای Windows 10 و 11 هستند. با «داده‌های کاربر»، تنظیماتی که Windows بی‌سروصدا برای یک برنامه قدیمی‌تر جداگانه نگه داشته بود نیز منتقل می‌شوند، و زمان‌های اجرایی که Windows 11 ندارد، مانند DirectX 9، همراه برنامه نصب می‌شوند.{ $is_32_bit ->
        [yes] { " " }برنامه‌های آن 32 بیتی هستند، بنابراین به Program Files (x86) می‌روند، جایی که Windows 64 بیتی آن‌ها را نگه می‌دارد.
       *[no] { "" }
    }
# The same, of a Windows whose name could not be read. { $is_32_bit } is as above.
app-source-older-windows-unnamed =
    روی این درایو یک Windows قدیمی‌تر نصب است. نصب‌کننده‌هایی که از آن ساخته شوند برای Windows 10 و 11 هستند. با «داده‌های کاربر»، تنظیماتی که Windows بی‌سروصدا برای یک برنامه قدیمی‌تر جداگانه نگه داشته بود نیز منتقل می‌شوند، و زمان‌های اجرایی که Windows 11 ندارد، مانند DirectX 9، همراه برنامه نصب می‌شوند.{ $is_32_bit ->
        [yes] { " " }برنامه‌های آن 32 بیتی هستند، بنابراین به Program Files (x86) می‌روند، جایی که Windows 64 بیتی آن‌ها را نگه می‌دارد.
       *[no] { "" }
    }

## The details panel, before an app is picked, and the app picked

app-details-pair-first = با رایانه قدیمی جفت شوید
app-details-pair-first-hint = وقتی WinAppMigrator Client در آنجا اجرا شد و کد آن را تایپ کردید، برنامه‌های آن در اینجا فهرست می‌شوند و می‌توانید مانند برنامه‌های این رایانه، از هر کدام نصب‌کننده بسازید.
app-details-pick-an-app = یک برنامه انتخاب کنید
app-details-pick-an-app-hint = برنامه‌ای را از فهرست انتخاب کنید تا مشخص کنید چه چیزهایی گنجانده شود و نصب‌کننده آن را بسازید.
# { $publisher } is who made the app, and { $version } its version, such as 11.4.2.
app-details-publisher-and-version = { $publisher }، نسخه { $version }
# { $version } is the app's version, such as 11.4.2.
app-details-unknown-publisher-and-version = ناشر ناشناخته، نسخه { $version }
app-details-unknown-publisher = ناشر ناشناخته

## What goes in besides the program

app-option-license = داده‌های مجوز و فعال‌سازی
app-option-license-hint = شاید به این رایانه وابسته باشد و در رایانه جدید به فعال‌سازی دوباره نیاز داشته باشد.
app-option-user-data = داده‌های کاربر
# Documents, Saved Games, and ProgramData are folders of Windows: use the names Windows gives them in your language,
# except ProgramData, which keeps its name.
app-option-user-data-hint = تنظیمات و تاریخچه، اسناد، بازی‌های ذخیره‌شده و ProgramData.
app-option-whose-data = داده‌های چه کسی
app-option-whose-data-hint = هر نصب‌کننده داده‌های یک حساب را در خود دارد. این داده‌ها به کسی می‌رسد که هنگام اجرای آن وارد سیستم شده است.

## Building the installer

app-output-folder = پوشه خروجی
# An example of a folder, shown in the empty field. "You" stands for the name of an account.
app-output-folder-example = C:\Users\You\WinAppMigrator
app-output-folder-not-full = یک مسیر کامل پوشه وارد کنید، مانند D:\Installers.
app-build-create = ایجاد نصب‌کننده
app-build-create-again = ایجاد دوباره نصب‌کننده
app-build-cancel = لغو
app-build-cancelling = در حال لغو پس از مرحله فعلی…
app-build-cancelled = ساخت لغو شد. هیچ نصب‌کننده‌ای نوشته نشد.
# winappmigrator-installer.exe is a file name.
app-build-no-stub = پایه نصب‌کننده (winappmigrator-installer.exe) کنار این برنامه پیدا نشد، بنابراین نمی‌توان نصب‌کننده خوداستخراج ساخت.
app-build-select-an-app-first = ابتدا برنامه‌ای را برای انتقال انتخاب کنید.
app-build-look-again-first = این برنامه باید پیش از ساخت دوباره بررسی شود.
# { $reason } is what went wrong, in the program's own words, often in English.
app-build-stopped-unexpectedly = ساخت به‌طور غیرمنتظره متوقف شد: { $reason }. اگر نصب‌کننده‌ای در پوشه خروجی باقی گذاشته است، از آن استفاده نکنید.
app-build-worker-disconnected = ارتباط با فرایند انتقال قطع شد.
# { $path } is the output folder, and { $error } what Windows said went wrong.
app-build-no-output-folder = پوشه خروجی { $path } ایجاد نشد: { $error }
# { $error } is what went wrong.
app-build-no-staging-folder = پوشه آماده‌سازی محافظت‌شده ایجاد نشد، بنابراین چیزی ساخته نشد: { $error }
# { $error } is what went wrong, as a sentence.
app-build-could-not-build = نصب‌کننده ساخته نشد. { $error }
app-build-no-installer = ساخت بدون هیچ نصب‌کننده‌ای پایان یافت.
# { $error } is what went wrong.
app-build-hash-check-did-not-run = نصب‌کننده نوشته شد، اما بررسی هش آن اجرا نشد: { $error }. از آن استفاده نکنید.

## How far a build has got

app-progress-preparing = در حال تعیین آنچه گنجانده می‌شود…
app-progress-checking-room = در حال بررسی فضای کافی…
app-progress-bundling = در حال بسته‌بندی زمان‌های اجرا و درایورها…
# { $done } is how many files have been read so far, and { $total } how many there are.
app-progress-packing = در حال خواندن و فشرده‌سازی فایل‌ها: { $done } از { $total }
# { $done } is how many of the installer's records have been looked over so far, and { $total } how many there are.
app-progress-looking-over = در حال مرور آنچه نصب‌کننده می‌گوید: { $done } از { $total }…
app-progress-writing = در حال نوشتن نصب‌کننده…
app-progress-verifying = در حال تأیید هش‌ها…

## What a build made

app-result-failed-hash-check = از این نصب‌کننده استفاده نکنید: در بررسی هش رد شد.
# { $count } is how many files were gone when the build came to read them.
app-result-files-gone =
    { $count ->
        [one] نصب‌کننده ساخته شد، اما { $count } فایل از زمان بررسی برنامه دیگر وجود نداشت، بنابراین در آن نیست. «هشدارها» را در زیر ببینید.
       *[other] نصب‌کننده ساخته شد، اما { $count } فایل از زمان بررسی برنامه دیگر وجود نداشتند، بنابراین در آن نیستند. «هشدارها» را در زیر ببینید.
    }
# { $count } is how many files could not be read into the installer.
app-result-files-not-captured =
    { $count ->
        [one] نصب‌کننده ساخته شد، اما { $count } فایل گرفته نشد، معمولاً به این دلیل که قفل یا غیرقابل خواندن بود. شاید برنامه بدون آن کار نکند. «هشدارها» را در زیر ببینید.
       *[other] نصب‌کننده ساخته شد، اما { $count } فایل گرفته نشدند، معمولاً به این دلیل که قفل یا غیرقابل خواندن بودند. شاید برنامه بدون آن‌ها کار نکند. «هشدارها» را در زیر ببینید.
    }
app-result-mentions-who = نصب‌کننده ساخته شد و هنوز نشانه‌ای از اینکه از چه کسی آمده است در خود دارد. پیش از اینکه آن را به کسی بدهید، آنچه را در زیر آمده است بخوانید.
app-result-ready = نصب‌کننده آماده است. بسته آن از بررسی هش با موفقیت گذشت.
app-result-files = فایل‌ها
app-result-installer-size = اندازه نصب‌کننده
app-result-warnings = هشدارها
app-result-log = گزارش
app-result-copy-error = کپی خطا
app-result-installer = نصب‌کننده
app-result-data-file = فایل داده
app-result-open-folder = باز کردن پوشه
app-result-copy-path = کپی مسیر
# { $error } is what Windows said went wrong.
app-result-could-not-open-folder = پوشه باز نشد: { $error }
# { $installer } and { $data_file } are file names, such as App_installer.exe and App_installer.wamdata.
app-result-two-files = این برنامه برای یک فایل اجرایی بیش از حد بزرگ است، بنابراین نصب‌کننده آن دو فایل است: { $installer } و { $data_file }. این دو باید کنار هم بمانند: هر دو را با همین نام‌ها در یک پوشه روی رایانه دیگر کپی کنید. اگر نصب‌کننده را امضا کنید، آن امضا فایل داده را نیز پوشش می‌دهد.
# { $app_name } is the app's name. "Unknown publisher" is what Windows shows when it asks to run an unsigned program: use
# Windows' own words for it in your language.
app-result-run-it = آن را روی رایانه دیگر اجرا کنید تا { $app_name } دوباره نصب شود. فایل‌هایش را با SHA-256 بررسی می‌کند. Windows «ناشر ناشناخته» را نشان می‌دهد، زیرا نصب‌کننده‌هایی که می‌سازید امضا نشده‌اند.
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-verification-errors = خطاهای تأیید ({ $count })
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-warnings-heading = هشدارها ({ $count })
# After the first of a long list of warnings. { $count } is how many more there are.
app-warnings-and-more = و { $count } مورد دیگر.

## Looking at an app, to see what an installer of it would hold

app-scan-getting-ready = در حال آماده شدن برای بررسی این برنامه…
app-scan-install-record = در حال خواندن نحوه نصب آن…
app-scan-near-matches = در حال جستجوی مواردی که شاید بخشی از آن هم باشند…
app-scan-system-items = در حال جستجوی سرویس‌ها، کارها و میان‌برهای آن…
app-scan-files = در حال بررسی فایل‌های آن…
app-scan-registry = در حال خواندن آنچه آن را با Windows یکپارچه می‌کند…
app-scan-finishing = در حال اتمام…
app-scan-look-again = بررسی دوباره
app-scan-stop = توقف
app-scan-running-hint = در حین اجرای این کار می‌توانید برنامه دیگری انتخاب کنید یا موارد گنجانده‌شده را تغییر دهید.
# { $error } is why, in words that follow a colon.
app-scan-failed = این برنامه بررسی نشد: { $error }
app-scan-try-again = تلاش دوباره
app-scan-stopped = متوقف شد. پس از بررسی این برنامه می‌توان نصب‌کننده ساخت.
# { $reason } is what went wrong, in the program's own words, often in English.
app-scan-stopped-unexpectedly = به‌طور غیرمنتظره متوقف شد: { $reason }.

## How big an installer can be, and whether there is room for it

app-room-files = فایل‌ها
# Over how big the installer can be: up to that size.
app-room-up-to = حداکثر
app-room-free = فضای آزاد
# { $drive } is a drive, such as C:.
app-room-free-on = فضای آزاد در { $drive }
# How much room a drive has, when it would not say.
app-room-free-unknown = نامشخص
app-room-most-it-could-need = بیشترین فضایی که شاید نیاز داشته باشد، پیش از فشرده‌سازی.
# { $drive } is a drive, such as C:, { $needed } how much room it needs, such as 1.2 GB, and { $free } how much it has.
app-room-drive-short = { $drive } برای ساخت این نصب‌کننده به حداکثر { $needed } فضای آزاد نیاز دارد و { $free } دارد.
# { $drives } is one sentence like the one above for each drive without the room, one after another.
app-room-not-enough = فضای کافی وجود ندارد. { $drives } یک پوشه خروجی در درایو دیگری انتخاب کنید، یا چیزی را کنار بگذارید.
# { $note } is a sentence, without its full stop, about a drive whose room could not be checked. Use your language's
# full stop.
app-room-could-not-tell = { $note }.

## What goes in the installer, part by part

app-footprint-heading = آنچه در نصب‌کننده قرار می‌گیرد
app-footprint-working-out = در حال تعیین آنچه همراه آن می‌رود…
# { $error } is why.
app-footprint-would-not-make-a-package = آنچه باقی مانده است بسته‌ای تشکیل نمی‌دهد: { $error }
# { $list } is the names of what else is left out, such as a service and a shortcut.
app-footprint-also-left-out = این کار { $list } را نیز کنار می‌گذارد، زیرا آنچه آن‌ها اجرا می‌کنند یا نام می‌برند وجود نخواهد داشت. آن‌ها در رایانه دیگر راه‌اندازی نخواهند شد.
# { $list } is the names of the first few of what else is left out, and { $count } how many more there are.
app-footprint-also-left-out-and-more = این کار { $list } و { $count } مورد دیگر را نیز کنار می‌گذارد، زیرا آنچه آن‌ها اجرا می‌کنند یا نام می‌برند وجود نخواهد داشت. آن‌ها در رایانه دیگر راه‌اندازی نخواهند شد.
# { $list } is the names of license files. The option is the one named "License and Activation Data".
app-footprint-licenses-still-go = داده‌های مجوز در مواردی که کنار گذاشتید همچنان گنجانده می‌شود، زیرا تابع گزینه «داده‌های مجوز و فعال‌سازی» است: { $list }.
# { $elapsed } is how long ago, in minutes and seconds such as 45:00, or hours, minutes, and seconds such as 1:05:00.
# "Look Again" names the button of that name.
app-footprint-looked-at-long-ago = این برنامه { $elapsed } پیش بررسی شد. فایل‌ها هنگام ساخت خوانده می‌شوند، بنابراین تغییرات لحاظ می‌شوند و هر چه تا آن زمان از بین رفته باشد فهرست می‌شود. برای یافتن موارد جدید، «بررسی دوباره» را بزنید.
# Headings of the parts of the list, each with a line that says what is in it.
app-footprint-program-files = فایل‌های برنامه
app-footprint-data-folders = پوشه‌های داده
# What could go in with the User Data option, when nothing else is in this part. { $size } is how much, such as 1.2 GB.
app-footprint-folders-with-user-data = { $size } با گزینه «داده‌های کاربر»
# { $included } is how many folders go in, { $count } how many there are, and { $size } how much those that go in hold.
app-footprint-folders-summary =
    { $count ->
        [one] { $included } از { $count } پوشه، { $size }
       *[other] { $included } از { $count } پوشه، { $size }
    }
# { $size } is how much, such as 1.2 GB, and { $count } in how many files.
app-footprint-data-only-with-user-data =
    { $count ->
        [one] { $size } در { $count } فایل از داده‌های این برنامه فقط با گزینه «داده‌های کاربر» گنجانده می‌شود.
       *[other] { $size } در { $count } فایل از داده‌های این برنامه فقط با گزینه «داده‌های کاربر» گنجانده می‌شود.
    }
# A folder's row. { $count } is how many files it holds, and { $detail } where it is or why it cannot be left out.
app-footprint-folder-row =
    { $count ->
        [one] { $count } فایل. { $detail }
       *[other] { $count } فایل. { $detail }
    }
app-footprint-services = سرویس‌ها
app-footprint-startup = شروع خودکار
app-footprint-tasks = کارهای زمان‌بندی‌شده
app-footprint-shortcuts = میان‌برها
# { $included } is how many of the part's things go in, and { $count } how many there are.
app-footprint-items-summary = { $included } از { $count }
# { $why } is what else was left out that this goes with.
app-footprint-left-out-too = این نیز کنار گذاشته شد: { $why }
# A row's detail. { $kind } is what it is, such as Service, and { $detail } what it does.
app-footprint-change-detail = { $kind }. { $detail }
app-footprint-integrations = یکپارچگی با Windows
# { $included } is how many kinds go in, such as file types and firewall rules, and { $count } how many kinds there are.
app-footprint-integrations-summary =
    { $count ->
        [one] { $included } از { $count } نوع
       *[other] { $included } از { $count } نوع
    }
app-footprint-integration-nothing-left = به دلیل مواردی که کنار گذاشتید، چیزی از آن باقی نمانده است
# { $count } is how many things of this kind the installer sets up.
app-footprint-integration-things =
    { $count ->
        [one] { $count } مورد راه‌اندازی می‌شود
       *[other] { $count } مورد راه‌اندازی می‌شود
    }
app-footprint-prerequisites = آنچه برای اجرا نیاز دارد
# { $count } is how many runtimes, such as Visual C++ Redistributable.
app-footprint-prerequisites-summary =
    { $count ->
        [one] { $count } زمان اجرا
       *[other] { $count } زمان اجرا
    }
app-footprint-near-matches = شاید بخشی از آن هم باشد
# { $ticked } is how many were ticked, and { $count } how many there are.
app-footprint-near-matches-summary = { $ticked } از { $count } اضافه شد
app-footprint-near-matches-hint = تا آن‌ها را علامت نزنید، در نصب‌کننده قرار نمی‌گیرند. علامت زدن هر مورد، برنامه را دوباره با همان بررسی‌های بقیه موارد بررسی می‌کند، بنابراین هر چه شبیه گذرواژه باشد همچنان بیرون می‌ماند.
# { $why } is why it might be part of the app, as a sentence.
app-footprint-near-match-with-user-data = همراه گزینه «داده‌های کاربر» می‌آید. { $why }
# How much a folder holds, when not all of it could be measured. { $size } is how much was, such as 1.2 GB.
app-footprint-near-match-over = بیش از { $size }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-folder = پوشه: { $place }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-hidden = بخش پنهان یک برنامه: { $place }
app-footprint-nothing-left-out = هیچ چیزی به دلیل شباهت به گذرواژه، مجوز یا گواهی کنار گذاشته نشد.
app-footprint-left-out = موارد کنارگذاشته و دلیل آن‌ها
# { $count } is how many things were left out.
app-footprint-left-out-summary =
    { $count ->
        [one] { $count } مورد
       *[other] { $count } مورد
    }
# Above a list of things left out for one reason. { $title } is the reason.
app-footprint-left-out-group = { $title }:
app-footprint-refused = هیچ نصب‌کننده‌ای اجازه راه‌اندازی این موارد را ندارد:
# A heading over a list that opens and closes. { $count } is how many are in it.
app-footprint-worth-knowing = نکات قابل توجه ({ $count })
# { $list } is what kinds of things were looked for and none found, from the five below.
app-none-found = هیچ موردی یافت نشد: { $list }.
app-none-found-services = سرویس‌ها
app-none-found-startup = موارد راه‌اندازی
app-none-found-tasks = کارهای زمان‌بندی‌شده
app-none-found-shortcuts = میان‌برها
app-none-found-integrations = هر چیزی که آن را با Windows یکپارچه کند

## Network PC: reading another PC through WinAppMigrator Client, in three steps. "WinAppMigrator Client" is the name of a
## program, and the old PC is the one the apps are moved off.

app-network-step-client = 1. کلاینت را روی رایانه قدیمی قرار دهید
app-network-step-client-hint = WinAppMigrator Client روی رایانه‌ای اجرا می‌شود که اکنون برنامه روی آن است، با Windows 7 یا جدیدتر. فقط به این رایانه اجازه می‌دهد آن را به‌صورت رمزگذاری‌شده و با کدی که نشان می‌دهد بخواند.
# { $drive } is a USB drive, such as E:\.
app-network-save-to-usb = ذخیره در درایو USB { $drive }
# { $drive } is a USB drive, such as E:\, and { $name } the name it was given, such as BACKUP.
app-network-save-to-named-usb = ذخیره در درایو USB { $drive } ({ $name })
app-network-save-to-folder = ذخیره در یک پوشه…
app-network-look-for-usb-again = جستجوی دوباره درایوهای USB
app-network-no-usb = هیچ درایو USB متصل نیست. یکی را متصل کنید، یا در پوشه‌ای ذخیره کنید که از رایانه قدیمی به آن دسترسی دارید، مانند یک پوشه اشتراکی در شبکه.
# { $path } is where the client was saved, such as E:\WinAppMigrator Client.exe.
app-network-client-saved = به‌عنوان { $path } ذخیره شد. در رایانه قدیمی، برنامه‌ای را که منتقل می‌کنید ببندید، سپس روی کلاینت دوبار کلیک کنید و اجازه دهید تغییرات ایجاد کند. کلاینت یک کد نشان می‌دهد.
app-network-step-pick = 2. رایانه قدیمی را انتخاب کنید
app-network-looking = در حال جستجوی رایانه‌هایی در شبکه که کلاینت را اجرا می‌کنند…
app-network-type-name = یا نام یا نشانی آن را تایپ کنید
# An example, shown in the empty field: a PC's name, or its network address.
app-network-type-name-example = OLD-PC یا 192.168.1.20
app-network-step-code = 3. کدی را که نشان می‌دهد تایپ کنید
app-network-code = کد جفت‌سازی
app-network-connect = اتصال
app-network-pairing = در حال جفت شدن…
# Why pairing did not work, shown on its own.
app-network-pairing-stopped = جفت شدن پیش از تکمیل متوقف شد
app-network-wrong-code = این کدی نیست که رایانه قدیمی نشان می‌دهد. آن را بررسی کنید و دوباره امتحان کنید.
# { $error } is what went wrong.
app-network-not-reached = دسترسی به رایانه قدیمی ممکن نشد: { $error }
app-network-pick-folder-title = انتخاب پوشه برای WinAppMigrator Client
# { $path } is the folder, and { $error } what Windows said went wrong.
app-client-could-not-save = کلاینت در { $path } ذخیره نشد: { $error }
# scripts\build-client.ps1 is a file name.
app-client-not-carried = این نسخه از WinAppMigrator کلاینت را در خود ندارد. آن را با scripts\build-client.ps1 بسازید، سپس برنامه را دوباره بسازید.
# { $error } is what Windows said went wrong.
app-client-could-not-save-note = یادداشت کنار آن ذخیره نشد: { $error }
# A note saved as a text file beside the client, for whoever runs it on the old PC. Keep its lines short, as it is read in
# Notepad. "WinAppMigrator Client.exe" is a file name. "Network PC..." names the Scan Source button: use its words in your
# language.
app-client-note =
    WinAppMigrator Client
    
    این را روی رایانه‌ای اجرا کنید که برنامه‌ها را از آن منتقل می‌کنید:
    Windows 7، 8، 10 یا 11.
    
    1. برنامه‌ای را که منتقل می‌کنید ببندید.
    2. روی «WinAppMigrator Client.exe» دوبار کلیک کنید و اجازه دهید
       تغییرات ایجاد کند.
    3. یک کد جفت‌سازی 6 رقمی نشان می‌دهد. در WinAppMigrator روی رایانه
       جدید، «رایانه شبکه...» را انتخاب کنید، این رایانه را برگزینید و کد
       را تایپ کنید.
    4. وقتی کار رایانه جدید تمام شد، در پنجره کلاینت Enter را فشار دهید
       یا پنجره را ببندید.
    
    فقط می‌خواند. در حین اجرا یک تصویر لحظه‌ای از درایو Windows نگه
    می‌دارد و دو درگاه خود را در دیوار آتش Windows برای شبکه محلی شما باز
    می‌کند، و هنگام توقف هر دو را حذف می‌کند. آنچه از شبکه عبور می‌کند
    رمزگذاری‌شده است و فقط رایانه‌ای که کد را دارد می‌تواند چیزی بخواند.
