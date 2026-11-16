# Words of the installer that every built installer carries, which runs on the PC an app is moved to and speaks its
# language, and of the uninstaller it leaves behind for Settings. Message names start with installer- for the
# installer's window, installer-cli- for its command line, and uninstaller- for the uninstaller.
#
# The command lines (installer-cli-, uninstaller-cli-) are always shown in English, as scripts read what they print.
# Translate them all the same. Keep their command-line flags, such as --apply and --target-root, exactly as they are.


## The installer's window: its title and header

installer-window-title = نصب‌کننده WinAppMigrator
# The window's title in the taskbar. { $app_name } is the name of the app it installs.
installer-window-title-app = نصب { $app_name }
installer-tagline = برنامه‌ای را که از رایانه دیگری گرفته شده است دوباره نصب می‌کند
# What an app is called when its package gives it no name.
installer-unnamed-app = برنامه

## When the installer's window cannot open. Each is a paragraph of a message box.

# { $error } is what went wrong, as Windows or the graphics driver says it.
installer-window-failed = پنجره نصب‌کننده باز نشد: { $error }
installer-window-failed-without-one = همچنان می‌توان بدون پنجره نصب کرد: این فایل را از خط فرمانی با حقوق سرپرستی با --apply اجرا کنید و برای انتخاب محل، --target-root را به‌همراه یک پوشه بدهید.

## Checking the installer when it opens

installer-checking = در حال بررسی نصب‌کننده…
installer-stub-no-package = این یک پایه نصب‌کننده WinAppMigrator است و هیچ بسته برنامه‌ای در خود ندارد. با WinAppMigrator یک نصب‌کننده بسازید، سپس فایل .exe ساخته‌شده را اینجا اجرا کنید.
installer-check-stopped = بررسی نصب‌کننده متوقف شد.
# { $error } is what went wrong, as Windows says it.
installer-locate-failed = نصب‌کننده پیدا نشد: { $error }
# { $error } is what went wrong reading it.
installer-read-failed = نصب‌کننده خوانده نشد: { $error }
# { $error } is why, such as that the app needs a newer Windows.
installer-cannot-be-used-here = از بسته این نصب‌کننده نمی‌توان در این رایانه استفاده کرد: { $error }

## The app, where it goes, and whether the installer is signed

installer-captured-from = گرفته‌شده از
installer-install-location = محل نصب
# The title of the folder picker.
installer-choose-location = انتخاب محل نصب
installer-folder-has-files = این پوشه از قبل فایل‌هایی دارد. فایل‌های هم‌نام جایگزین می‌شوند و اگر نصب را حذف کنید، بازمی‌گردند.
installer-room-needed = فضای مورد نیاز
# { $shortfall } says which drive is short of room, such as "C: needs up to 824.0 MB for the app's files, and has 300.0 MB free".
installer-room-short = شاید فضای کافی در اینجا نباشد: { $shortfall }. می‌توان بسته را در درایو دیگری استخراج کرد، اما فایل‌های برنامه همچنان باید جا شوند.
# { $app_name } is the name of an app whose earlier install was cut short.
installer-interrupted = نصب قبلی { $app_name } هرگز تمام نشد. «نصب» ابتدا آن را برمی‌گرداند.
# Beside "Installer", for whether it proves who made it.
installer-signature = نصب‌کننده
installer-signature-checking = در حال بررسی امضای آن…
installer-signature-unsigned = امضا نشده: ناشر ناشناخته
# { $publisher } is the name on the installer's certificate, such as "Fabrikam, Inc.".
installer-signature-signed-by = امضاشده توسط { $publisher }
installer-signature-unnamed = امضاشده توسط ناشری بی‌نام
# { $reason } is Windows' own word for what is wrong, such as HashMismatch.
installer-signature-invalid = امضای این نصب‌کننده معتبر نیست ({ $reason }). ممکن است پس از امضا تغییر کرده باشد.

## Why the install location cannot be used. Install and Preview Changes stay off while one of these is shown.

# { $app_name } is the name of the app being installed.
installer-location-app-already-there = { $app_name } از قبل در این پوشه نصب شده است، بنابراین این نسخه نمی‌تواند اینجا نصب شود. برای نصب آن در اینجا، ابتدا نسخه موجود در این رایانه را از تنظیمات Windows، در بخش «برنامه‌ها»، حذف نصب کنید و سپس این نصب‌کننده را دوباره باز کنید. یا یک پوشه جدید و خالی در بالا وارد کنید.
# { $other_app } is the name of another app Windows lists as installed in the folder. { $app_name } is the name of the app being installed.
installer-location-other-app-there = { $other_app } در این پوشه نصب شده است، بنابراین { $app_name } نمی‌تواند اینجا نصب شود. یک پوشه جدید و خالی در بالا وارد کنید. یا اگر دیگر به { $other_app } نیاز ندارید، ابتدا آن را از تنظیمات Windows، در بخش «برنامه‌ها»، حذف نصب کنید و سپس این نصب‌کننده را دوباره باز کنید.
installer-location-has-files = این پوشه از قبل فایل‌هایی دارد که WinAppMigrator آن‌ها را نصب نکرده است. شاید متعلق به برنامه دیگری باشند، بنابراین چیزی روی آن‌ها نصب نمی‌شود. یک پوشه جدید و خالی در بالا وارد کنید.
# { $reason } is why, such as that it cannot be the Windows folder.
installer-location-refused = از این پوشه نمی‌توان استفاده کرد: { $reason }. پوشه دیگری در بالا وارد کنید.

## What the install sets up beyond its files

installer-beyond-files = آنچه علاوه بر فایل‌ها راه‌اندازی می‌شود
# Under the heading above. It is part of what a person agrees to when they click Install.
installer-beyond-files-agreed = همه این کارها با حقوق سرپرستی انجام می‌شود. اگر نصب ناموفق باشد، تغییرات آن بازگردانده می‌شود و هر چه بازگردانده نشود به شما گفته می‌شود. همچنین می‌توانید بعداً این نصب را از تنظیمات، در بخش «برنامه‌های نصب‌شده»، برگردانید. فایل‌هایی که جایگزین می‌کند بازمی‌گردند و زمان‌های اجرایی که نصب می‌کند باقی می‌مانند.
installer-beyond-files-none = هیچ: فقط فایل‌ها و تنظیمات موجود در کلیدهای رجیستری خود برنامه را کپی می‌کند.
# { $problem } is why, such as that the app cannot run on this PC.
installer-cannot-go-here = این بسته نمی‌تواند در این محل نصب شود: { $problem }
installer-working-out = در حال تعیین آنچه این بسته در این محل راه‌اندازی می‌کند…
# Said as the reason after "This package cannot go to this location:".
installer-list-stopped = تعیین کارهایی که انجام می‌داد متوقف شد
# A heading over a list of what else the package asked for and will not get. { $count } is how many.
installer-more-left-out =
    { $count ->
        [one] { $count } مورد دیگر کنار گذاشته شد
       *[other] { $count } مورد دیگر کنار گذاشته شد
    }

## Options

installer-options = گزینه‌ها
installer-user-data = داده‌های کاربر
installer-user-data-detail = روشن: تنظیمات، اسناد و بازی‌های ذخیره‌شده به پوشه‌های معمول کاربری می‌روند که وارد سیستم شده است. خاموش: همه آن‌ها زیر پوشه نصب قرار می‌گیرند و چیزی برای اجرا از آنجا راه‌اندازی نمی‌شود.
# { $account } is the name of the account on the other PC whose data this is.
installer-user-data-whose = این بسته داده‌های { $account } را در خود دارد.
installer-license = مجوز
installer-license-detail = فایل‌های فعال‌سازی و مجوز، و مقادیر رجیستری که مجوزها را در خود دارند. برخی شاید در اینجا به فعال‌سازی دوباره نیاز داشته باشند.

## Dependencies: what else the app needs, and how each is provided

installer-dependencies = وابستگی‌ها
installer-dependencies-detail = آنچه این برنامه نیاز دارد و نحوه تأمین هر کدام.
installer-dependencies-none = هیچ وابستگی زمان اجرای دیگری شناسایی نشد.
# { $feature } is the name of an optional part of Windows, such as NetFx3.
installer-dependency-windows-feature = ویژگی Windows: { $feature }
installer-dependency-included = گنجانده‌شده
# { $id } is the package's name in winget, such as Microsoft.VCRedist.2015+.x64.
installer-dependency-winget = نصب با winget: { $id }
installer-dependency-manual = نصب دستی
installer-dependency-built-in = جزئی از Windows
installer-dependency-ships-with-app = همراه برنامه

## Buttons, and what is said beside them

installer-install = نصب
installer-preview = پیش‌نمایش تغییرات
installer-cancel = لغو
installer-close = بستن
installer-open-folder = باز کردن پوشه نصب
# { $error } is what went wrong, as Windows says it.
installer-open-folder-failed = پوشه باز نشد: { $error }
installer-location-blocks = «نصب» و «پیش‌نمایش تغییرات» تا زمانی که بتوان از محل نصب استفاده کرد غیرفعال هستند. دلیل آن در بالا، زیر محل نصب آمده است.
installer-package-blocks = «نصب» غیرفعال است، زیرا این بسته نمی‌تواند در این محل نصب شود. دلیل آن در بالا آمده است.
installer-list-changed = آنچه این نصب در اینجا انجام می‌داد تغییر کرده است، بنابراین چیزی نصب نشد. فهرست زیر را مرور کنید و اگر همان چیزی است که می‌خواهید، دوباره روی «نصب» کلیک کنید.
installer-cancelled-early = لغو شد. چیزی تغییر نکرد.

## How far an install has got, beside its progress bar

# { $step } is one of the steps below. { $done } and { $total } are how many of its items are done, and how many there are.
installer-step-counted = { $step }: { $done } از { $total }…
# { $step } is one of the steps below, when it does not count its items.
installer-step-uncounted = { $step }…
installer-step-getting-ready = در حال آماده شدن…
installer-step-cancelling = در حال لغو پس از مرحله فعلی…
installer-step-putting-back = در حال بازگرداندن تغییرات
installer-step-unpacking = در حال استخراج
installer-step-checking-package = در حال بررسی بسته
installer-step-recovering = در حال برگرداندن نصب قبلی که هرگز تمام نشد
# What a preview is doing, whatever step it is on.
installer-step-previewing = در حال تعیین آنچه تغییر خواهد کرد
installer-step-runtimes = در حال نصب زمان‌های اجرا
installer-step-files = در حال کپی کردن فایل‌ها
installer-step-services = در حال راه‌اندازی سرویس‌ها
installer-step-drivers = در حال راه‌اندازی درایورها
installer-step-registry = در حال نوشتن تنظیمات رجیستری
installer-step-license = در حال بازیابی داده‌های مجوز
installer-step-tasks = در حال راه‌اندازی کارهای زمان‌بندی‌شده
installer-step-shortcuts = در حال ایجاد میان‌برها
installer-step-environment = در حال تنظیم متغیرهای محیطی
installer-step-settings-entry = در حال افزودن برنامه به تنظیمات
installer-step-verifying = در حال بررسی آنچه نصب شد
installer-step-undoing = در حال برگرداندن

## When an install cannot go ahead or stops

installer-carries-no-package = این نصب‌کننده هیچ بسته‌ای ندارد.
installer-stopped-unexpectedly = نصب به‌طور غیرمنتظره متوقف شد.
# { $reason } is what went wrong, as the program says it.
installer-stopped-unexpectedly-because = نصب به‌طور غیرمنتظره متوقف شد: { $reason }. اگر نصب آغاز شده بود، اجرای دوباره یک نصب‌کننده ابتدا کارهای انجام‌شده را برمی‌گرداند.
# { $error } is what went wrong reading it.
installer-package-read-failed = بسته خوانده نشد: { $error }
installer-no-programdata = پوشه ProgramData این رایانه پیدا نشد، بنابراین چیزی نصب نشد: هر نصب، سابقه‌ای را که با آن برگردانده می‌شود در آنجا نگه می‌دارد.
# { $error } is why, such as that there is nowhere only administrators can get into.
installer-staging-failed = پوشه آماده‌سازی ایجاد نشد: { $error }
# { $error } is what went wrong unpacking it.
installer-unpack-failed = بسته استخراج نشد: { $error }
installer-staging-nowhere = هیچ جایی برای استخراج بسته نیست که فقط سرپرستان به آن دسترسی داشته باشند
# { $failure } is a folder and what went wrong there, such as "C:\ProgramData\WinAppMigrator: Access is denied.".
installer-staging-nowhere-look-at = هیچ جایی برای استخراج بسته نیست که فقط سرپرستان به آن دسترسی داشته باشند. جایی که باید بررسی شود: { $failure }

## What came of an install, or of a preview

installer-result = نتیجه نصب
installer-result-preview = پیش‌نمایش: هنوز چیزی تغییر نکرده است
installer-result-installed = برنامه نصب شد و از بررسی هش با موفقیت گذشت.
installer-result-installed-without-left-out = برنامه بدون مواردی که در زیر به‌عنوان کنارگذاشته فهرست شده‌اند نصب شد و از بررسی هش با موفقیت گذشت.
installer-result-not-all-put-back = نصب ناموفق بود و همه تغییرات آن بازگردانده نشد. موارد باقی‌مانده در زیر فهرست شده‌اند و نصب بعدی دوباره تلاش می‌کند.
installer-result-cancelled-put-back = نصب لغو شد و تغییرات آن بازگردانده شد.
installer-result-failed-put-back = نصب ناموفق بود و تغییرات آن بازگردانده شد.
installer-result-not-clean = نصب به‌طور کامل و بی‌نقص تمام نشد. مشکلات زیر را مرور کنید.
installer-result-changes-put-back = تغییرات بازگردانده‌شده
installer-result-changes-made = تغییرات اعمال‌شده
installer-result-changes-planned = تغییرات برنامه‌ریزی‌شده
installer-result-warnings = هشدارها
installer-result-errors = خطاها
installer-result-package-valid = هش‌های بسته معتبرند
installer-result-package-invalid = هش‌های بسته نامعتبرند
installer-result-files-valid = فایل‌های نصب‌شده معتبرند
installer-result-files-invalid = فایل‌های نصب‌شده نامعتبرند
installer-result-reboot = یک وابستگی برای تکمیل به راه‌اندازی مجدد نیاز دارد. Windows را دوباره راه‌اندازی کنید، سپس برنامه را اجرا کنید.
# A heading over the list of what was changed. { $count } is how many.
installer-result-changes = تغییرات ({ $count })
# After the first part of a long list. { $count } is how many more there are.
installer-result-changes-more = و { $count } مورد دیگر.
# Over the path of the install's log file.
installer-log = گزارش

## The installer's command line. Shown in English: scripts read it. A { $count } is plain digits, with no separators.

installer-cli-stub = پایه نصب‌کننده WinAppMigrator — هیچ بسته برنامه‌ای در خود ندارد.
installer-cli-stub-how = با برنامه WinAppMigrator یک نصب‌کننده بسازید، سپس فایل .exe ساخته‌شده را روی رایانه مقصد اجرا کنید.
installer-cli-usage-title = نصب‌کننده خوداستخراج WinAppMigrator.
installer-cli-usage = آن را بدون آرگومان اجرا کنید تا نصب‌کننده گرافیکی باز شود، یا:
installer-cli-usage-preview = پیش‌نمایش نصب مجدد: --dry-run [--target-root <پوشه>]
installer-cli-usage-apply = نصب مجدد:           --apply [--target-root <پوشه>]
installer-cli-no-package = نصب‌کننده هیچ بسته جاسازی‌شده‌ای ندارد
installer-cli-no-programdata = پوشه ProgramData این رایانه پیدا نشد، بنابراین چیزی نصب نشد: هر نصب، سابقه‌ای را که با آن برگردانده می‌شود در آنجا نگه می‌دارد
# { $path } is the folder asked for, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-folder-has-files = از { $path } نمی‌توان استفاده کرد: { $problem }. فقط اگر می‌دانید که متعلق به همین برنامه است، از --force استفاده کنید
installer-cli-no-location = این بسته نمی‌گوید برنامه‌اش کجا بوده است، بنابراین از --target-root استفاده کنید
# { $path } is where the app was on the other PC, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-location-refused = برنامه این بسته در { $path } بوده است و از آنجا نمی‌توان استفاده کرد: { $problem }. از --target-root استفاده کنید
# { $summary } is how much room it needs where, such as "up to 824.0 MB on C: (356.0 GB free)".
installer-cli-room = فضا: { $summary }
# { $note } is something to know about the install.
installer-cli-note = توجه: { $note }
# { $count } is how many things it sets up beyond its files.
installer-cli-sets-up = آنچه راه‌اندازی می‌کند: { $count }
installer-cli-applied = نصب مجدد اعمال شد.
installer-cli-previewed = پیش‌نمایش نصب مجدد انجام شد.
# { $count } is how many changes the install logged.
installer-cli-changes-logged = تغییرات ثبت‌شده در گزارش: { $count }
# { $count } is how many warnings.
installer-cli-warnings = هشدارها: { $count }
# { $count } is how many errors.
installer-cli-errors = خطاها: { $count }
# { $count } is how many things the package asked for and did not get.
installer-cli-refused = ردشده: { $count }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-package-valid = معتبر بودن هش‌های بسته: { $valid }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-reinstall-valid = معتبر بودن هش‌های نصب مجدد: { $valid }
# { $id } is the install's id, such as wam-0f3a….
installer-cli-install-id = شناسه نصب: { $id }
installer-cli-put-back = نصب ناموفق بود، بنابراین تغییرات آن بازگردانده شد.
# { $path } is the log file.
installer-cli-log = گزارش: { $path }
# { $path } is the log file. Printed before an error.
installer-cli-error-log = گزارش: { $path }
# { $error } is what went wrong.
installer-cli-error = خطا: { $error }

## The uninstaller's window

uninstaller-window-title = حذف نصب
# The window's title, and its heading. { $app_name } is the name of the app it uninstalls.
uninstaller-window-title-app = حذف نصب { $app_name }
uninstaller-tagline = کارهای نصب‌کننده خود را برمی‌گرداند
uninstaller-reading = در حال خواندن سابقه این نصب…
uninstaller-stopped-unexpectedly = حذف نصب به‌طور غیرمنتظره متوقف شد.
uninstaller-what-will-be-removed = آنچه حذف خواهد شد
uninstaller-install-location = محل نصب
uninstaller-files = فایل‌ها
uninstaller-registry-values = مقادیر رجیستری
uninstaller-services = سرویس‌ها
uninstaller-drivers = درایورها
uninstaller-scheduled-tasks = کارهای زمان‌بندی‌شده
uninstaller-environment = متغیرهای محیطی
uninstaller-plan-stays = فایل‌هایی که اضافه یا تغییر داده‌اید، همراه با پوشه‌هایی که در آن‌ها هستند، باقی می‌مانند.
# { $count } is how many files the install replaced.
uninstaller-plan-coming-back =
    { $count ->
        [one] { $count } فایلی که این نصب جایگزین کرد بازمی‌گردد، یا اگر فایلی که جای آن را گرفت از آن زمان تغییر کرده باشد، کنار آن قرار می‌گیرد.
       *[other] { $count } فایلی که این نصب جایگزین کرد بازمی‌گردند، یا اگر فایل‌هایی که جای آن‌ها را گرفتند از آن زمان تغییر کرده باشند، کنار آن‌ها قرار می‌گیرند.
    }
# { $runtimes } is a list of runtimes, such as "Microsoft Visual C++ Redistributable, .NET Desktop Runtime 8".
uninstaller-plan-runtimes = زمان‌های اجرایی که این نصب نصب کرد باقی می‌مانند، زیرا شاید برنامه‌های دیگر از آن‌ها استفاده کنند: { $runtimes }.
uninstaller-options = گزینه‌ها
uninstaller-remove-user-data = حذف داده‌های کاربر و داده‌های مجوز نیز
# { $count } is how many items of the app's settings, data, and license there are.
uninstaller-remove-user-data-detail =
    { $count ->
        [one] { $count } مورد از تنظیمات، داده‌ها و مجوز آن. تا این گزینه روشن نباشد، باقی می‌ماند. آنچه جایگزین کرده بود بازمی‌گردد، یا اگر این مورد از آن زمان تغییر کرده باشد، کنار آن قرار می‌گیرد.
       *[other] { $count } مورد از تنظیمات، داده‌ها و مجوز آن. تا این گزینه روشن نباشد، باقی می‌مانند. آنچه جایگزین کرده بودند بازمی‌گردد، یا اگر این موارد از آن زمان تغییر کرده باشند، کنار آن‌ها قرار می‌گیرد.
    }
uninstaller-uninstall = حذف نصب
uninstaller-cancel = لغو
uninstaller-close = بستن
# { $done } and { $total } are how many of the install's items are undone, and how many there are.
uninstaller-step-counted = در حال برگرداندن نصب: { $done } از { $total }…
uninstaller-step = در حال برگرداندن نصب…
uninstaller-done = برنامه حذف نصب شد.
uninstaller-done-except = برنامه حذف نصب شد، به‌جز مواردی که در زیر فهرست شده‌اند.
uninstaller-waits-for-drive = بخشی از آن روی درایوی است که متصل نیست. آن را متصل کنید و دوباره از تنظیمات حذف نصب کنید.
uninstaller-not-all-undone = همه آن برگردانده نشد. هر چه را از آن استفاده می‌کند ببندید و دوباره از تنظیمات حذف نصب کنید.
# Over the path of the uninstall's log file.
uninstaller-log = گزارش
uninstaller-undone = برگردانده‌شده
uninstaller-left-in-place = در جای خود باقی‌مانده
uninstaller-could-not-be-undone = برگردانده نشد
# After the first part of a long list. { $count } is how many more there are.
uninstaller-and-more = و { $count } مورد دیگر.
# Said as the reason when there is no record of the install to read.
uninstaller-no-programdata = پوشه ProgramData این رایانه پیدا نشد، بنابراین سابقه‌ای برای خواندن وجود ندارد

## When the uninstaller's window cannot open. Each is a paragraph of a message box, followed by a command to type.

uninstaller-window-failed-title = برنامه حذف نصب WinAppMigrator
# { $error } is what went wrong, as Windows or the graphics driver says it.
uninstaller-window-failed = پنجره برنامه حذف نصب باز نشد: { $error }
uninstaller-window-failed-quiet = همچنان می‌توان بدون پنجره، از خط فرمانی با حقوق سرپرستی، حذف نصب کرد:
# Stands in for this program's own path in that command, if it cannot be found.
uninstaller-this-program = این برنامه

## The uninstaller's command line (--quiet). Shown in English: scripts read it.

# { $app_name } is the name of the app.
uninstaller-cli-uninstalling = در حال حذف نصب { $app_name }.
# { $path } is the log file.
uninstaller-cli-log = گزارش: { $path }
# { $error } is what went wrong.
uninstaller-cli-error = خطا: { $error }
