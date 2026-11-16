# Words the core writes for people: warnings, refusals, errors, and the installer's list of what it sets up.
# Message names start with core-.


## Warnings a package records when it is built, which an installer shows later, maybe on another PC, in that PC's
## language.

# { $count } is how many shortcuts.
core-warning-shortcuts-unreadable =
    { $count ->
        [one] { $count } میان‌بر در این درایو هیچ فایلی را نام نمی‌برد، بنابراین مشخص نشد که آیا این برنامه را باز می‌کند یا نه.
       *[other] { $count } میان‌بر در این درایو هیچ فایلی را نام نمی‌برند، بنابراین مشخص نشد که آیا این برنامه را باز می‌کنند یا نه.
    }

## Undoing an install: what an uninstall, or a failed install, says it put back, left, or could not undo. Each line is
## shown on its own, in a list.

# { $app_name } is the app's name.
core-rollback-record-damaged = بخشی از سابقه { $app_name } خوانده نشد، بنابراین هر چه در آن شرح داده شده بود همچنان در جای خود است، و شاید فایلی که جایگزین کرده بود بازنگردد.
core-rollback-settings-entry-stays = ورودی برنامه در تنظیمات باقی می‌ماند تا بتوان این کار را دوباره از آنجا اجرا کرد.
# { $name } is a runtime's name, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-rollback-prerequisite-stays = { $name } نصب شد، یا شاید نصب شده باشد، و باقی می‌ماند، زیرا شاید برنامه‌های دیگر از آن استفاده کنند.
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-drive-not-connected = آنچه این نصب در { $folder } قرار داد روی درایوی است که متصل نیست. آن را متصل کنید تا این کار تمام شود.
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-record-part-unread = آنچه این نصب در { $folder } قرار داد باقی می‌ماند، زیرا آن بخش از سابقه خوانده نشد.

## Why a file or folder was left, shown after "was left:" in the messages below them. Each is part of a sentence, so
## starts with a small letter and has no full stop.

# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-being-changed = چیزی در حین اجرای این کار { $folder } را تغییر می‌داد، بنابراین کار دیگری در آن انجام نشد. هر چه این کار را می‌کند ببندید و این را دوباره اجرا کنید
# { $folder } is a folder; { $error } is what Windows said went wrong.
core-rollback-reason-not-opened = { $folder } باز نشد: { $error }
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-no-record = سابقه‌ای از محل { $folder } وجود ندارد
# { $folder } is a folder; { $actual } is the folder it now leads to.
core-rollback-reason-moved = { $folder } اکنون به { $actual } می‌رسد، نه جایی که هنگام نصب بود
# Why a service was not stopped.
core-rollback-reason-did-not-stop = به‌موقع متوقف نشد

## Files and folders

# { $path } is a folder; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-folder-left = پوشه { $path } باقی ماند: { $reason }.
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-removed = پوشه { $path } حذف شد
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-holds-others = پوشه { $path } باقی می‌ماند، زیرا فایل‌هایی دارد که این نصب در آن قرار نداده است.
# { $path } is a folder; { $error } is what Windows said went wrong.
core-rollback-folder-not-removed = پوشه { $path } حذف نشد: { $error }.
# { $path } is a file; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-file-left = فایل { $path } باقی ماند: { $reason }.
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-folder-gone = { $path } بازگردانده نمی‌شود، زیرا پوشه‌ای که در آن بود دیگر وجود ندارد.
# { $path } is a file; { $error } is what went wrong.
core-rollback-partial-not-removed = یک فایل ناقص حذف نشد: { $path } ({ $error }).
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-checked = { $path } بررسی نشد: { $error }.
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-removed = { $path } حذف شد
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-removed = { $path } حذف نشد: { $error }.
# { $path } is a file this install had replaced, and has put back.
core-rollback-file-put-back = { $path } بازگردانده شد
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-put-back = { $path } بازگردانده نشد: { $error }.
# Why a file could not be put back.
core-rollback-backup-gone = نسخه پشتیبان دیگر وجود ندارد
# { $path } is a file.
core-rollback-no-parent-folder = { $path } پوشه والد ندارد

## A file that stays. { $why } says why: still-used (something that stays still uses it), changed (it has changed since
## it was installed), or not-asked (it is someone's data, and they did not ask for it to go). { $path } is the file.

core-rollback-file-stays =
    { $why ->
        [still-used] { $path } باقی می‌ماند، زیرا چیزی که باقی می‌ماند هنوز از آن استفاده می‌کند.
        [changed] { $path } باقی می‌ماند، زیرا از زمان نصب تغییر کرده است.
       *[not-asked] { $path } باقی می‌ماند، زیرا درخواستی برای حذف آن نشده است.
    }
# The file it replaced was to be put beside it, under another name, and every name was taken.
core-rollback-file-stays-no-name-beside =
    { $why ->
        [still-used] { $path } باقی می‌ماند، زیرا چیزی که باقی می‌ماند هنوز از آن استفاده می‌کند، و فایلی که جایگزین کرده بود نتوانست کنار آن قرار گیرد: همه نام‌های ممکن برای آن گرفته شده است.
        [changed] { $path } باقی می‌ماند، زیرا از زمان نصب تغییر کرده است، و فایلی که جایگزین کرده بود نتوانست کنار آن قرار گیرد: همه نام‌های ممکن برای آن گرفته شده است.
       *[not-asked] { $path } باقی می‌ماند، زیرا درخواستی برای حذف آن نشده است، و فایلی که جایگزین کرده بود نتوانست کنار آن قرار گیرد: همه نام‌های ممکن برای آن گرفته شده است.
    }
# { $name } is the name the file it replaced now has, such as settings.ini.before-install.
core-rollback-file-stays-beside =
    { $why ->
        [still-used] { $path } باقی می‌ماند، زیرا چیزی که باقی می‌ماند هنوز از آن استفاده می‌کند. فایلی که جایگزین کرده بود با نام { $name } کنار آن است.
        [changed] { $path } باقی می‌ماند، زیرا از زمان نصب تغییر کرده است. فایلی که جایگزین کرده بود با نام { $name } کنار آن است.
       *[not-asked] { $path } باقی می‌ماند، زیرا درخواستی برای حذف آن نشده است. فایلی که جایگزین کرده بود با نام { $name } کنار آن است.
    }
# { $error } is what went wrong.
core-rollback-file-stays-not-beside =
    { $why ->
        [still-used] { $path } باقی می‌ماند، زیرا چیزی که باقی می‌ماند هنوز از آن استفاده می‌کند، و فایلی که جایگزین کرده بود نتوانست کنار آن قرار گیرد: { $error }.
        [changed] { $path } باقی می‌ماند، زیرا از زمان نصب تغییر کرده است، و فایلی که جایگزین کرده بود نتوانست کنار آن قرار گیرد: { $error }.
       *[not-asked] { $path } باقی می‌ماند، زیرا درخواستی برای حذف آن نشده است، و فایلی که جایگزین کرده بود نتوانست کنار آن قرار گیرد: { $error }.
    }

## The registry and environment variables. { $kind } says what { $name } is: key (a registry key, such as
## HKLM\Software\App), value (a registry value, such as HKLM\Software\App\Version), user-variable or machine-variable
## (an environment variable of the user's or of the whole PC, such as APP_HOME), or user-list or machine-list (a
## variable that is a list of folders, such as Path). { $error } is what Windows said went wrong.

# How a registry key's unnamed value is shown, as Registry Editor shows it.
core-registry-default-value-name = (پیش‌فرض)
core-rollback-setting-other-account =
    { $kind ->
        [key] کلید { $name } باقی می‌ماند، زیرا متعلق به حسابی است که این را نصب کرد و آن حساب وارد سیستم نشده است.
        [user-variable] متغیر کاربر { $name } باقی می‌ماند، زیرا متعلق به حسابی است که این را نصب کرد و آن حساب وارد سیستم نشده است.
        [machine-variable] متغیر سیستم { $name } باقی می‌ماند، زیرا متعلق به حسابی است که این را نصب کرد و آن حساب وارد سیستم نشده است.
        [user-list] متغیر { $name } کاربر باقی می‌ماند، زیرا متعلق به حسابی است که این را نصب کرد و آن حساب وارد سیستم نشده است.
        [machine-list] متغیر { $name } سیستم باقی می‌ماند، زیرا متعلق به حسابی است که این را نصب کرد و آن حساب وارد سیستم نشده است.
       *[value] مقدار رجیستری { $name } باقی می‌ماند، زیرا متعلق به حسابی است که این را نصب کرد و آن حساب وارد سیستم نشده است.
    }
core-rollback-setting-not-reached =
    { $kind ->
        [key] دسترسی به کلید { $name } ممکن نشد: { $error }.
        [user-variable] دسترسی به متغیر کاربر { $name } ممکن نشد: { $error }.
        [machine-variable] دسترسی به متغیر سیستم { $name } ممکن نشد: { $error }.
        [user-list] دسترسی به متغیر { $name } کاربر ممکن نشد: { $error }.
        [machine-list] دسترسی به متغیر { $name } سیستم ممکن نشد: { $error }.
       *[value] دسترسی به مقدار رجیستری { $name } ممکن نشد: { $error }.
    }
core-rollback-setting-not-read =
    { $kind ->
        [key] کلید { $name } خوانده نشد: { $error }.
        [user-variable] متغیر کاربر { $name } خوانده نشد: { $error }.
        [machine-variable] متغیر سیستم { $name } خوانده نشد: { $error }.
        [user-list] متغیر { $name } کاربر خوانده نشد: { $error }.
        [machine-list] متغیر { $name } سیستم خوانده نشد: { $error }.
       *[value] مقدار رجیستری { $name } خوانده نشد: { $error }.
    }
core-rollback-setting-changed =
    { $kind ->
        [user-variable] متغیر کاربر { $name } باقی می‌ماند، زیرا از زمان نصب تغییر کرده است.
        [machine-variable] متغیر سیستم { $name } باقی می‌ماند، زیرا از زمان نصب تغییر کرده است.
        [user-list] متغیر { $name } کاربر باقی می‌ماند، زیرا از زمان نصب تغییر کرده است.
        [machine-list] متغیر { $name } سیستم باقی می‌ماند، زیرا از زمان نصب تغییر کرده است.
       *[value] مقدار رجیستری { $name } باقی می‌ماند، زیرا از زمان نصب تغییر کرده است.
    }
# Said when it is back as it was before the install.
core-rollback-setting-put-back =
    { $kind ->
        [user-variable] متغیر کاربر { $name } بازگردانده شد
        [machine-variable] متغیر سیستم { $name } بازگردانده شد
        [user-list] متغیر { $name } کاربر بازگردانده شد
        [machine-list] متغیر { $name } سیستم بازگردانده شد
       *[value] مقدار رجیستری { $name } بازگردانده شد
    }
core-rollback-setting-not-put-back =
    { $kind ->
        [user-variable] متغیر کاربر { $name } بازگردانده نشد: { $error }.
        [machine-variable] متغیر سیستم { $name } بازگردانده نشد: { $error }.
        [user-list] متغیر { $name } کاربر بازگردانده نشد: { $error }.
        [machine-list] متغیر { $name } سیستم بازگردانده نشد: { $error }.
       *[value] مقدار رجیستری { $name } بازگردانده نشد: { $error }.
    }
# A registry value the install wrote, which is the user's data, and stays unless they asked for it to go.
core-rollback-value-kept-by-request = مقدار رجیستری { $name } باقی می‌ماند، زیرا درخواستی برای حذف آن نشده است.
core-rollback-key-holds-others = کلید { $name } باقی می‌ماند، زیرا چیزهایی دارد که این نصب در آن قرار نداده است.
core-rollback-key-removed = کلید { $name } حذف شد
core-rollback-key-not-removed = کلید { $name } حذف نشد: { $error }.
# { $kind } is user-list or machine-list.
core-rollback-list-entries-removed =
    { $kind ->
        [user-list] ورودی‌های این نصب از متغیر { $name } کاربر حذف شد
       *[machine-list] ورودی‌های این نصب از متغیر { $name } سیستم حذف شد
    }
core-rollback-list-entries-not-removed =
    { $kind ->
        [user-list] ورودی‌های این نصب از متغیر { $name } کاربر حذف نشد: { $error }.
       *[machine-list] ورودی‌های این نصب از متغیر { $name } سیستم حذف نشد: { $error }.
    }

## Services and drivers. { $kind } is service or driver; { $name } is its name, such as AppUpdater. { $error } and
## { $reason } are what Windows said went wrong.

core-rollback-service-not-read =
    { $kind ->
        [driver] درایور { $name } خوانده نشد: { $error }.
       *[service] سرویس { $name } خوانده نشد: { $error }.
    }
core-rollback-service-changed =
    { $kind ->
        [driver] درایور { $name } باقی می‌ماند، زیرا از زمان نصب تغییر کرده است.
       *[service] سرویس { $name } باقی می‌ماند، زیرا از زمان نصب تغییر کرده است.
    }
core-rollback-service-not-stopped =
    { $kind ->
        [driver] درایور { $name } متوقف نشد ({ $reason })، بنابراین شاید آنچه از آن استفاده می‌کند تا راه‌اندازی مجدد رایانه باقی بماند.
       *[service] سرویس { $name } متوقف نشد ({ $reason })، بنابراین شاید آنچه از آن استفاده می‌کند تا راه‌اندازی مجدد رایانه باقی بماند.
    }
core-rollback-service-stopped-for-this =
    { $kind ->
        [driver] درایور { $name } برای انجام این کار متوقف شد. هر وقت به آن نیاز داشتید، دوباره آن را شروع کنید.
       *[service] سرویس { $name } برای انجام این کار متوقف شد. هر وقت به آن نیاز داشتید، دوباره آن را شروع کنید.
    }
core-rollback-service-not-removed =
    { $kind ->
        [driver] درایور { $name } حذف نشد: { $reason }.
       *[service] سرویس { $name } حذف نشد: { $reason }.
    }
core-rollback-service-marked-for-removal =
    { $kind ->
        [driver] درایور { $name } برای حذف علامت‌گذاری شده است و وقتی رایانه دوباره راه‌اندازی شود یا هر چه آن را باز نگه داشته رهایش کند، حذف می‌شود.
       *[service] سرویس { $name } برای حذف علامت‌گذاری شده است و وقتی رایانه دوباره راه‌اندازی شود یا هر چه آن را باز نگه داشته رهایش کند، حذف می‌شود.
    }
core-rollback-service-removed =
    { $kind ->
        [driver] درایور { $name } حذف شد
       *[service] سرویس { $name } حذف شد
    }
core-rollback-service-not-restored =
    { $kind ->
        [driver] درایور { $name } به حالت قبلی بازگردانده نشد: { $reason }.
       *[service] سرویس { $name } به حالت قبلی بازگردانده نشد: { $reason }.
    }
core-rollback-service-restored =
    { $kind ->
        [driver] تنظیمات درایور { $name } بازگردانده شد
       *[service] تنظیمات سرویس { $name } بازگردانده شد
    }
core-rollback-service-not-restarted = سرویس { $name } پیش از این نصب در حال اجرا بود و دوباره شروع نشد: { $reason }.
# { $account } is the account the service ran as, such as CONTOSO\backup.
core-rollback-service-account-again = سرویس { $name } به‌عنوان { $account } اجرا می‌شد. گذرواژه آن نگه داشته نمی‌شود، بنابراین حساب آن را دوباره در «سرویس‌ها» تنظیم کنید.

## Firewall rules. { $name } is the rule's name; { $error } is what went wrong.

core-rollback-firewall-not-read = قانون دیوار آتش { $name } خوانده نشد: { $error }.
core-rollback-firewall-says-otherwise = قانون دیوار آتش { $name } باقی می‌ماند، زیرا آنچه با این نام وجود دارد چیز دیگری می‌گوید.
core-rollback-firewall-changed = قانون دیوار آتش { $name } باقی می‌ماند، زیرا از زمان نصب تغییر کرده است.
# The capital "The" is as the app has always said it.
core-rollback-firewall-removed = قانون دیوار آتش { $name } حذف شد
core-rollback-firewall-not-removed = قانون دیوار آتش { $name } حذف نشد: { $error }.

## Scheduled tasks. { $path } is a folder of tasks, and { $name } a task with its folder, such as \App\Nightly Update.
## { $error } and { $reason } are what went wrong.

core-rollback-task-folder-holds-others = پوشه کار { $path } باقی می‌ماند، زیرا کارهایی دارد که این نصب در آن قرار نداده است.
core-rollback-task-folder-not-removed = پوشه کار { $path } حذف نشد: { $error }.
core-rollback-task-not-read = کار { $name } خوانده نشد: { $error }.
core-rollback-task-changed = کار { $name } باقی می‌ماند، زیرا از زمان نصب تغییر کرده است.
core-rollback-task-removed = کار { $name } حذف شد
core-rollback-task-not-removed = کار { $name } حذف نشد: { $reason }.
core-rollback-task-put-back = کار { $name } بازگردانده شد
core-rollback-task-not-put-back = کار { $name } بازگردانده نشد: { $reason }.

## When an install cannot be undone at all

# { $error } is what went wrong.
core-rollback-record-unreadable = سابقه این نصب خوانده نشد، بنابراین چیزی برگردانده نشد: { $error }.
# { $error } is what went wrong.
core-rollback-nothing-undone = چیزی برگردانده نشد: { $error }.
# One app's part of the message below. { $app_name } is the app's name; { $failures } is what could not be undone, a
# sentence or more; { $folder } is the folder that holds the install's record.
core-rollback-earlier-install-stuck = { $app_name }: { $failures } اگر هرگز قابل برگرداندن نباشد، یک سرپرست می‌تواند با حذف { $folder } از آن صرف‌نظر کند.
# Why an install did not start. { $details } is one or more of the message above.
core-rollback-earlier-installs-not-undone = نصب قبلی که هرگز تمام نشد به‌طور کامل برگردانده نشد، بنابراین چیزی نصب نشد. هر چه از این‌ها استفاده می‌کند ببندید و دوباره امتحان کنید. { $details }
core-rollback-install-busy = نصب‌کننده یا برنامه حذف نصب دیگری در حال تغییر این نصب است

## The summary an uninstall writes on the command line, one line each, with what each says below it. { $count } is how
## many.

core-rollback-count-undone = برگردانده‌شده: { $count }
core-rollback-count-failed = برگردانده‌نشده: { $count }
# What was left on purpose: it has changed since the install, or it is the user's.
core-rollback-count-kept = در جای خود باقی‌مانده: { $count }
# What no uninstall can take back, such as a runtime that was installed, which other programs may use.
core-rollback-count-left-behind = جامانده: { $count }

## Which Windows a PC or drive has, as the app shows it beside the drive.

# { $name } is the Windows' name, such as Windows 7 Professional Service Pack 1.
core-system-windows-32-bit = { $name }، 32 بیتی
# { $name } is the Windows' name, such as Windows 11 Pro.
core-system-windows-64-bit = { $name }، 64 بیتی
# A drive's Windows, when it only says it is older than Vista.
core-system-windows-xp-or-older = Windows XP یا قدیمی‌تر

## Reading a connected drive, or another account's settings. Each is why it could not be done, and some start with a
## small letter, as they follow other words.

core-system-needs-administrator = خواندن یک درایو متصل یا تنظیمات حساب دیگر به حقوق سرپرستی نیاز دارد
core-system-no-program-data = پوشه ProgramData این رایانه پیدا نشد
# { $path } is a file, such as D:\Windows\System32\config\SOFTWARE.
core-system-not-a-hive-file = فایل کندوی رجیستری نیست: { $path }
# { $path } is a file; { $size } is its size in bytes.
core-system-hive-too-large = اندازه { $path } برابر { $size } بایت است، که برای یک کندوی رجیستری بیش از حد بزرگ است، یا از مجموع کندوهای این درایو بیشتر است
# { $key } is a registry key, such as Software\Microsoft\Windows\CurrentVersion.
core-system-hive-no-mapping = کندوی آفلاین هیچ نگاشتی برای HKLM\{ $key } ندارد
# { $key } is where a drive's registry was to be loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said.
# "reg load" is a command's name.
core-system-hive-not-loaded = reg load { $key } ناموفق بود: { $error }
# { $key } is where a drive's registry was loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said. "reg
# unload" is a command's name.
core-system-hive-not-unloaded = reg unload { $key } ناموفق بود: { $error }
# { $drive } is a drive, such as D:\.
core-system-not-a-system-drive = { $drive } درایو سیستمی Windows نیست (پوشه \Windows ندارد).
# { $drive } is a drive, such as C:\. "This PC" is the name of a choice in the app.
core-system-own-system-drive = { $drive } درایو سیستمی خود این رایانه است. برای خواندن این رایانه، «این رایانه» را انتخاب کنید.
# { $drive } is a drive, such as D:\.
core-system-registry-through-link = { $drive } درایو سیستمی Windows نیست: مسیر رسیدن به رجیستری آن از یک پیوند عبور می‌کند.

## Warnings a scan records about whose data it read.

# { $account } is an account's name or id, such as alice or S-1-5-21-...-1001.
core-warning-scan-account-not-on-record = حساب { $account } در اینجا ثبت نشده است، بنابراین داده‌ها یا تنظیمات ویژه هیچ کاربری گنجانده نشد.
# { $account } is an account's name, such as CONTOSO\alice.
core-warning-scan-signed-in-account-no-profile = حسابی که در اینجا وارد سیستم شده است، { $account }، نمایه ثبت‌شده‌ای ندارد، بنابراین داده‌ها یا تنظیمات ویژه هیچ کاربری گنجانده نشد.
# { $reason } is why Windows could not say.
core-warning-scan-signed-in-account-unknown = مشخص نشد چه کسی وارد سیستم شده است ({ $reason })، بنابراین داده‌ها یا تنظیمات ویژه هیچ کاربری گنجانده نشد. انتخاب کنید داده‌های چه کسی گنجانده شود.
core-warning-scan-last-account-unknown = این درایو نمی‌گوید آخرین بار چه کسی وارد سیستم شده است و بیش از یک حساب دارد، بنابراین داده‌ها یا تنظیمات ویژه هیچ کاربری گنجانده نشد. انتخاب کنید داده‌های چه کسی گنجانده شود.
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-settings-unreadable = تنظیمات { $account } خوانده نشد، بنابراین گنجانده نشد: { $error }
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-classes-unreadable = کلاس‌های COM و انواع فایلی که برای { $account } ثبت شده‌اند خوانده نشدند، بنابراین گنجانده نشدند: { $error }

## Paths a scan did not follow. { $count } is how many; { $paths } is some of them, separated by semicolons; { $more } is
## how many more there are, not shown.

core-warning-scan-drive-paths-not-followed =
    { $count ->
        [one] مسیری که در این درایو ثبت شده است خوانده نشد، بنابراین چیزی از آنجا گنجانده نشد. هر کدام روی دیسک دیگری است، مسیر کامل نیست، یا از پیوندی عبور می‌کند که به بیرون از درایو می‌رسد: { $paths }
       *[other] برخی مسیرهایی که در این درایو ثبت شده‌اند خوانده نشدند، بنابراین چیزی از آنجا گنجانده نشد. هر کدام روی دیسک دیگری است، مسیر کامل نیست، یا از پیوندی عبور می‌کند که به بیرون از درایو می‌رسد: { $paths }
    }
core-warning-scan-drive-paths-not-followed-more = برخی مسیرهایی که در این درایو ثبت شده‌اند خوانده نشدند، بنابراین چیزی از آنجا گنجانده نشد. هر کدام روی دیسک دیگری است، مسیر کامل نیست، یا از پیوندی عبور می‌کند که به بیرون از درایو می‌رسد: { $paths } و { $more } مورد دیگر
core-warning-scan-network-paths-not-followed =
    { $count ->
        [one] مسیری که در این رایانه ثبت شده است خوانده نشد، بنابراین چیزی از آنجا گنجانده نشد. هر کدام یا روی دیسکی از این رایانه نیست، که در این صورت بررسی آن باعث اتصال به آن می‌شود، یا نامی دارد که چیز دیگری را باز می‌کند: { $paths }
       *[other] برخی مسیرهایی که در این رایانه ثبت شده‌اند خوانده نشدند، بنابراین چیزی از آنجا گنجانده نشد. هر کدام یا روی دیسکی از این رایانه نیست، که در این صورت بررسی آن باعث اتصال به آن می‌شود، یا نامی دارد که چیز دیگری را باز می‌کند: { $paths }
    }
core-warning-scan-network-paths-not-followed-more = برخی مسیرهایی که در این رایانه ثبت شده‌اند خوانده نشدند، بنابراین چیزی از آنجا گنجانده نشد. هر کدام یا روی دیسکی از این رایانه نیست، که در این صورت بررسی آن باعث اتصال به آن می‌شود، یا نامی دارد که چیز دیگری را باز می‌کند: { $paths } و { $more } مورد دیگر

## What a scan with none of an account's data leaves out of a package, in the app's list of what was left out. Each
## has a name, and below it why.

core-account-left-out-shortcuts = میان‌برهای موجود در پوشه‌های خود این حساب
# { $count } is how many shortcuts.
core-account-left-out-shortcuts-why =
    { $count ->
        [one] { $count } میان‌بر، که فقط با گزینه «داده‌های کاربر» گنجانده می‌شود
       *[other] { $count } میان‌بر، که فقط با گزینه «داده‌های کاربر» گنجانده می‌شوند
    }
core-account-left-out-variables = متغیرهای محیطی این حساب
# { $count } is how many variables.
core-account-left-out-variables-why =
    { $count ->
        [one] { $count } متغیر، که فقط با گزینه «داده‌های کاربر» گنجانده می‌شود
       *[other] { $count } متغیر، که فقط با گزینه «داده‌های کاربر» گنجانده می‌شوند
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-account-left-out-task = کار زمان‌بندی‌شده { $task }
core-account-task-works-on-account-folder = روی مکانی در پوشه‌های یک حساب کار می‌کند، یا به نام یکی از آن‌ها نام‌گذاری شده است، و فقط با گزینه «داده‌های کاربر» گنجانده می‌شود
core-account-task-runs-from-account-folder = آنچه علاوه بر این از پوشه‌های یک حساب اجرا می‌کند فقط با گزینه «داده‌های کاربر» گنجانده می‌شود
# { $service } is a service's name, such as AppUpdater.
core-account-left-out-service = سرویس { $service }
core-account-service-works-on-account-folder = روی مکانی در پوشه‌های یک حساب کار می‌کند و فقط با گزینه «داده‌های کاربر» گنجانده می‌شود

## What a package was found to still say of whose PC it came from, after a build with none of an account's data.

# The parts of a package, as the warning below names them.
core-account-part-files = مسیرهای فایل‌های خود برنامه
core-account-part-registry = مقادیر رجیستری
core-account-part-settings-entry = ورودی برنامه در تنظیمات
core-account-part-main-program = محل فایل اجرایی اصلی آن
core-account-part-shortcuts = میان‌برها
core-account-part-tasks = کارهای زمان‌بندی‌شده
core-account-part-services = سرویس‌ها
core-account-part-variables = متغیرهای محیطی
# One place in the list below. { $place } is one of the parts above, or a file's name; { $count } is how many times.
core-account-mention-place = { $place } ({ $count })
# { $kind } is what was found: account (an account's name), folder (where an account's folder is), computer (the PC's
# name), or id (an account's id). { $places } is a list of where, from the message above.
core-warning-scan-package-mentions =
    { $kind ->
        [account] این بسته نام یک حساب را در { $places } ذکر می‌کند. بدون داده‌های هیچ حسابی ساخته شده است و هر کسی که نصب‌کننده را داشته باشد می‌تواند این را بخواند.
        [folder] این بسته محل پوشه یک حساب را در { $places } ذکر می‌کند. بدون داده‌های هیچ حسابی ساخته شده است و هر کسی که نصب‌کننده را داشته باشد می‌تواند این را بخواند.
        [computer] این بسته نام رایانه را در { $places } ذکر می‌کند. بدون داده‌های هیچ حسابی ساخته شده است و هر کسی که نصب‌کننده را داشته باشد می‌تواند این را بخواند.
       *[id] این بسته شناسه یک حساب را در { $places } ذکر می‌کند. بدون داده‌های هیچ حسابی ساخته شده است و هر کسی که نصب‌کننده را داشته باشد می‌تواند این را بخواند.
    }
# { $count } is how many folders.
core-warning-scan-folders-of-others-unreadable =
    { $count ->
        [one] { $count } پوشه بیرون از برنامه خوانده نشد، بنابراین شاید بخشی از داده‌های برنامه در آنجا جا مانده باشد.
       *[other] { $count } پوشه بیرون از برنامه خوانده نشدند، بنابراین شاید بخشی از داده‌های برنامه در آنجا جا مانده باشد.
    }

## What a scan leaves out because it looks like a credential, or is a license with the License option off, in the app's
## list of what was left out. Each has a name, and below it why.

core-secret-why-credential = شبیه اعتبارنامه است
core-secret-why-license-option-off = گزینه «داده‌های مجوز و فعال‌سازی» خاموش است
# { $count } is how many values of a registry key.
core-secret-why-account-values =
    { $count ->
        [one] { $count } مقدار، که فقط با گزینه «داده‌های کاربر» گنجانده می‌شود
       *[other] { $count } مقدار، که فقط با گزینه «داده‌های کاربر» گنجانده می‌شوند
    }
# Shown in place of a registry value's name when its name itself looks like a password or a key.
core-secret-name-holds-credential = (مقداری که نامش شامل اعتبارنامه است)
# Shown in place of an option's name, for an argument on a command line that has no name.
core-secret-unnamed-argument = آرگومانی که اعتبارنامه دارد
# { $option } is an option taken out of a command line, such as /password, or the message above. { $place } says
# where the command line was: service, task, shortcut, opener (what opens a file type), opener-unnamed (the same, of a
# type with no name), or uninstall. { $name } is the service, task, shortcut, file type, or app.
core-secret-taken-from =
    { $place ->
        [service] { $option } در نحوه اجرای { $name }
        [task] { $option } در نحوه اجرای کار { $name }
        [shortcut] { $option } در میان‌بر { $name }
        [opener] { $option } در آنچه { $name } را باز می‌کند
        [opener-unnamed] { $option } در آنچه یک نوع فایل را باز می‌کند
       *[uninstall] { $option } در نحوه حذف نصب { $name }
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-secret-task-definition-unreadable = تعریف کار { $task }، که برای بررسی خوانده نشد
# { $name } is an environment variable's name, such as APP_TOKEN.
core-secret-left-out-variable = متغیر محیطی { $name }
# { $class } is a COM class's id, such as {00000000-0000-0000-0000-000000000000}.
core-secret-left-out-class-server = آنچه کلاس { $class } می‌گوید اجرا می‌کند
# { $rule } is a firewall rule's name.
core-secret-left-out-firewall-description = توضیح قانون دیوار آتش { $rule }

## Licenses a scan found, as the package's report describes each.

# { $name } is a file's name, such as license.lic.
core-license-detail-file = فایل مجوز شناسایی شد: { $name }
# { $framework } is a licensing product's name, such as FlexLM / FlexNet.
core-license-detail-store = انبار مجوز { $framework } وجود دارد
# { $name } is an environment variable's name, such as LM_LICENSE_FILE.
core-license-detail-variable = متغیر محیطی مجوز { $name }
# { $name } is a registry value's name, such as SerialNumber.
core-license-detail-registry-value = مقدار رجیستری نشان‌دهنده داده‌های مجوز است: { $name }
# Said of a registry value in an account's key, which is not named.
core-license-detail-registry-value-of-an-account = مقدار رجیستری نشان‌دهنده داده‌های مجوز است
# { $key } is a registry key, such as HKLM\Software\Contoso\License.
core-license-detail-registry-key = کلید رجیستری نشان‌دهنده داده‌های مجوز است: { $key }

## Warnings a scan records about licenses.

core-warning-scan-license-data = داده‌های مجوز/فعال‌سازی شناسایی شد. با روشن بودن گزینه «داده‌های مجوز و فعال‌سازی»، نصب‌کننده آنچه را قابل انتقال است بازیابی می‌کند؛ مقادیر سراسری رایانه و انبارهای وابسته به سخت‌افزار فقط گزارش می‌شوند. WinAppMigrator هرگز رمزگشایی نمی‌کند، فعال‌سازی را دور نمی‌زند و قفل‌های سخت‌افزاری (dongle) را شبیه‌سازی نمی‌کند.
core-warning-scan-license-machine-bound = برخی مجوزها به رایانه وابسته‌اند (قفل سخت‌افزاری، فعال‌سازی وابسته به رایانه، یا رمزگذاری‌شده). نصب‌کننده آنچه را بتواند بازیابی می‌کند؛ شاید این‌ها همچنان در رایانه جدید به فعال‌سازی دوباره نیاز داشته باشند.

## Warnings a scan records about the registry. What could not be read is not named in them: it is kept beside them.

# { $error } is what Windows said went wrong. App Paths is a registry key's name.
core-warning-scan-app-paths-key-unreadable = زیرکلید App Paths خوانده نشد: { $error }
core-warning-scan-registry-limit = خواندن رجیستری به حد ایمنی تنظیم‌شده رسید.
core-warning-scan-registry-subtree-limit = خواندن زیرشاخه رجیستری به حد ایمنی تنظیم‌شده رسید.
# { $error } is what Windows said went wrong.
core-warning-scan-registry-key-unreadable = کلید رجیستری خوانده نشد: { $error }
core-warning-scan-registry-value-too-large = مقدار رجیستری از حد اندازه تنظیم‌شده برای خواندن بزرگ‌تر بود.

## Scheduled tasks

core-warning-scan-tasks-folder-through-link = پوشه کارهای زمان‌بندی‌شده در این درایو از یک پیوند عبور می‌کند، بنابراین کارهای آن خوانده نشد.
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-warning-scan-task-left-out = کار زمان‌بندی‌شده { $task } کنار گذاشته شد: با حسابی غیر از حسابی اجرا می‌شود که تنظیماتش گنجانده می‌شود، کسانی غیر از سرپرستان می‌توانند فایل آن را تغییر دهند، یا برنامه آن‌قدر کار دارد که همه نگه داشته نمی‌شوند.
# { $error } is what Windows said went wrong.
core-warning-scan-tasks-folder-unreadable = بخشی از کارهای زمان‌بندی‌شده خوانده نشد: { $error }
# { $path } is a task's file, such as C:\Windows\System32\Tasks\App\Nightly Sync; { $reason } is one of the reasons
# below, or what Windows said went wrong.
core-warning-scan-task-unreadable = کار زمان‌بندی‌شده { $path } خوانده نشد: { $reason }. اگر برنامه را اجرا کند، گنجانده نشده است.
# Why a task's file could not be read. { $size } is its size in bytes.
core-task-too-large = اندازه آن { $size } بایت است، که برای یک کار بیش از حد بزرگ است
# Why a task's file could not be read.
core-task-ends-mid-character = در میانه یک نویسه تمام می‌شود
# Why a task's file could not be read. { $error } says where.
core-task-not-utf16 = UTF-16 معتبر نیست: { $error }
# Why a task's file could not be read. { $error } says where.
core-task-not-text = UTF-8 یا UTF-16 معتبر نیست: { $error }
# What a package records of how a task is put back. Register-ScheduledTask is a command's name.
core-task-restore-instructions = بازیابی از Register-ScheduledTask با تعریف XML بایگانی‌شده استفاده می‌کند.

## Reading a program's file. Each is why it could not be read, after the program's name.

# { $reason } is what went wrong.
core-pe-unreadable = خوانده نشد: { $reason }
core-pe-headers-misaligned = سرآیندهای آن در مرزهایی که سرآیندهای یک فایل اجرایی باید باشند قرار ندارند
# { $size } is the file's size in bytes.
core-pe-too-large = اندازه آن { $size } بایت است، که برای یک فایل اجرایی بیش از حد بزرگ است
# PE is the name of the format Windows programs are in.
core-pe-no-headers = سرآیند PE ندارد

## Warnings a scan records about what a program needs. Which program is kept beside each.

# { $error } is what went wrong.
core-warning-scan-imports-unreadable = جدول واردات خوانده نشد: { $error }
# { $error } is what went wrong.
core-warning-scan-import-name-unreadable = نام یک توصیفگر واردات خوانده نشد: { $error }
# { $count } is how many were skipped.
core-warning-scan-legacy-delay-imports =
    { $count ->
        [one] { $count } توصیفگر بارگذاری تأخیری قدیمی (v1) نادیده گرفته شد؛ فقط واردات تأخیری جدید مبتنی بر RVA تجزیه می‌شوند.
       *[other] { $count } توصیفگر بارگذاری تأخیری قدیمی (v1) نادیده گرفته شدند؛ فقط واردات تأخیری جدید مبتنی بر RVA تجزیه می‌شوند.
    }

## The DLLs a program needs. Which DLL or program is kept beside each warning.

# { $path } is a program, such as C:\Program Files\App\app.exe.
core-dll-program-has-no-folder = فایل اجرایی پوشه والد ندارد: { $path }
# Why a DLL is among the app's files, in the package's list of them.
core-dll-beside-program = DLL کنار فایل اجرایی انتخاب‌شده قرار دارد
# { $error } is what went wrong.
core-warning-scan-dependency-unscanned = وابستگی تودرتو اسکن نشد: { $error }
# { $name } is a DLL's name, such as d3dx9_43.dll.
core-warning-scan-dependency-not-in-newer-windows = { $name } در پوشه Windows رایانه‌ای بود که این برنامه روی آن بود، و Windows 10 و 11 آن را همراه خود ندارند. چیزی آن را در آنجا نصب کرده است، بنابراین بخشی از فایل‌های خود برنامه نیست و همراه آن نمی‌آید. شاید برنامه تا زمانی که آنچه آن را فراهم می‌کند نصب نشود، شروع نشود.
# { $error } is what went wrong.
core-warning-scan-dependency-not-hashed = هش وابستگی محاسبه نشد: { $error }
# SystemRoot and windir are the names of environment variables.
core-warning-scan-windows-folder-unknown = متغیرهای محیطی SystemRoot/windir در دسترس نیستند؛ جستجوی DLLهای سیستم محدود است.
# { $error } is what went wrong.
core-warning-scan-dependency-architecture-unreadable = معماری وابستگی خوانده نشد: { $error }

## Services, drivers, shortcuts, and environment variables a scan reads. Where it was reading is kept beside each
## warning.

# { $error } is what Windows said went wrong.
core-warning-scan-services-unreadable = کلید رجیستری سرویس‌ها خوانده نشد: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-drivers-unreadable = کلید رجیستری سرویس‌ها برای شناسایی درایورها خوانده نشد: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-queried = پرس‌وجوی میان‌برها انجام نشد: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-parsed = خروجی پرس‌وجوی میان‌برها تجزیه نشد: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-environment-unreadable = کلید رجیستری متغیرهای سیستم خوانده نشد: { $error }

## What a package records of how a driver is put back, which an installer shows. { $kind } is the driver's type as
## Windows names it (kernel, file-system, recognizer, or unknown-type); { $name } is its service's name; { $path } is
## its file. { $signed } is yes when { $signer } names who signed it, and no when that is not known.

core-service-driver-bundled =
    { $signed ->
        [yes] درایور { $kind } متعلق به برنامه، «{ $name }» ({ $path }، امضاشده توسط { $signer }). نصب‌کننده فایل .sys آن را همراه بسته می‌کند و سرویس درایور را به‌طور خودکار دوباره ثبت می‌کند (sc create، که به حقوق سرپرستی نیاز دارد)، سپس ثبت شدن سرویس را تأیید می‌کند.
       *[no] درایور { $kind } متعلق به برنامه، «{ $name }» ({ $path }، امضای نامشخص). نصب‌کننده فایل .sys آن را همراه بسته می‌کند و سرویس درایور را به‌طور خودکار دوباره ثبت می‌کند (sc create، که به حقوق سرپرستی نیاز دارد)، سپس ثبت شدن سرویس را تأیید می‌کند.
    }
core-service-driver-from-vendor =
    { $signed ->
        [yes] درایور { $kind } سازنده یا داخلی Windows، «{ $name }» ({ $path }، امضاشده توسط { $signer }). WinAppMigrator درایورهای شخص ثالث را توزیع نمی‌کند؛ پیش از اجرای برنامه، آن را با بسته درایور امضاشده سازنده در رایانه مقصد دوباره نصب کنید.
       *[no] درایور { $kind } سازنده یا داخلی Windows، «{ $name }» ({ $path }، امضای نامشخص). WinAppMigrator درایورهای شخص ثالث را توزیع نمی‌کند؛ پیش از اجرای برنامه، آن را با بسته درایور امضاشده سازنده در رایانه مقصد دوباره نصب کنید.
    }

## Runtimes an app needs, such as the Visual C++ runtime or .NET: how each is provided, as the Dependencies list says.

core-runtime-handling-built-in = جزئی از Windows
core-runtime-handling-ships-with-app = همراه برنامه
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-handling-windows-feature = ویژگی Windows: { $feature }
core-runtime-handling-included = گنجانده‌شده
# { $id } is the runtime's name in winget, such as Microsoft.VCRedist.2015+.x64.
core-runtime-handling-winget = نصب با winget: { $id }
core-runtime-handling-manual = نصب دستی

## Downloading a runtime's installer, when an installer is built. Each is why it could not be, and starts with a small
## letter, as it follows other words. { $url } is a web address; { $error } is what went wrong.

core-runtime-download-not-https = بارگیری { $url } رد شد: HTTPS نیست
# TLS is what keeps a download private.
core-runtime-tls-failed = راه‌اندازی TLS ناموفق بود: { $error }
core-runtime-download-failed = بارگیری { $url } ناموفق بود: { $error }
core-runtime-download-unreadable = خواندن { $url } ناموفق بود: { $error }
core-runtime-download-empty = بارگیری { $url } هیچ داده‌ای برنگرداند
# { $came } and { $expected } are numbers of bytes.
core-runtime-download-cut-short = بارگیری { $url } نیمه‌کاره ماند: { $came } از { $expected } بایت دریافت شد

## Installing a runtime: why it was not installed. Each starts with a small letter, as it follows other words.

core-runtime-no-known-installer = هیچ نصب‌کننده شناخته‌شده‌ای برای این زمان اجرا وجود ندارد
# { $path } is where in the package, such as prerequisites\vc_redist.x64.exe.
core-runtime-payload-misplaced = بسته { $path } را به‌عنوان نصب‌کننده این زمان اجرا نام می‌برد، که جای درست آن نیست، بنابراین اجرا نشد
# { $path } is a file.
core-runtime-payload-missing = نصب‌کننده همراه بسته وجود ندارد: { $path }
# { $error } is what went wrong.
core-runtime-payload-path-invalid = مسیر نصب‌کننده همراه بسته نامعتبر است: { $error }
# { $url } is the web address it can be downloaded from. winget is a program's name.
core-runtime-no-installer-route = نه نصب‌کننده همراه بسته‌ای هست و نه شناسه winget؛ آن را از { $url } تهیه کنید
# { $error } is what went wrong.
core-runtime-no-temp-folder = پوشه‌ای برای استخراج آن ایجاد نشد: { $error }
# { $reason } is what the check found.
core-runtime-payload-signature-failed = نصب‌کننده همراه بسته در بررسی امضا رد شد و اجرا نشد: { $reason }
core-runtime-payload-timed-out = نصب‌کننده همراه بسته در مهلت مقرر تمام نشد و متوقف شد
# { $error } is what went wrong.
core-runtime-payload-not-started = نصب‌کننده همراه بسته اجرا نشد: { $error }
core-runtime-installer-killed = نصب‌کننده بدون کد خروج متوقف شد
# { $code } is the number it ended with, such as 1603.
core-runtime-installer-failed = نصب‌کننده با کد { $code } خارج شد
# { $feature } is a Windows feature's name, such as NetFx3; { $code } is the number DISM ended with. Windows Features is
# the name of a Windows window. Keep the command as it is.
core-runtime-feature-failed = DISM نتوانست ویژگی { $feature } در Windows را فعال کند (کد خروج { $code })؛ احتمالاً محتوای آن بارگیری نشد. آن را از طریق «ویژگی‌های Windows» فعال کنید، یا این فرمان را اجرا کنید: `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`.
# The same, when DISM gave no number.
core-runtime-feature-failed-no-code = DISM نتوانست ویژگی { $feature } در Windows را فعال کند (کد خروج نامشخص)؛ احتمالاً محتوای آن بارگیری نشد. آن را از طریق «ویژگی‌های Windows» فعال کنید، یا این فرمان را اجرا کنید: `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`.
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-feature-timed-out = فعال کردن ویژگی { $feature } در Windows در مهلت مقرر تمام نشد و متوقف شد
# { $feature } is a Windows feature's name; { $error } is what went wrong.
core-runtime-feature-no-dism = DISM برای فعال کردن ویژگی { $feature } در Windows در دسترس نیست: { $error }

## Installing a runtime with winget. { $id } is its name in winget, such as Microsoft.VCRedist.2015+.x64.

# { $error } is what went wrong.
core-runtime-winget-not-looked-for = جستجوی winget ممکن نشد: { $error }
core-runtime-winget-not-installed = winget نصب نیست
# { $path } is where winget is.
core-runtime-winget-misplaced = { $path } جایی نیست که Windows بسته‌های خودش را نگه می‌دارد، بنابراین اجرا نشد
# { $path } is where winget is; { $reason } is what the check found.
core-runtime-winget-signature-failed = { $path } در بررسی امضا رد شد و اجرا نشد: { $reason }
# { $reason } is one of the two messages above, or what went wrong.
core-runtime-winget-unusable = نمی‌توان از winget برای نصب { $id } استفاده کرد: { $reason }
# { $code } is the number winget ended with.
core-runtime-winget-failed = winget install { $id } با کد { $code } خارج شد
core-runtime-winget-failed-no-code = winget install { $id } با کدی نامشخص خارج شد
core-runtime-winget-timed-out = winget install { $id } در مهلت مقرر تمام نشد و متوقف شد
# { $error } is what went wrong.
core-runtime-winget-not-started = winget برای نصب { $id } در دسترس نیست: { $error }

## What told a scan how an app was installed, as the package's report lists it. The names of installers and files are
## as their makers write them.

# { $path } is an installer's record of what it installed, such as C:\Program Files\App\unins000.dat.
core-source-record-not-believed = از { $path } استفاده نشد: کسانی غیر از سرپرستان می‌توانند آن را تغییر دهند
# { $count } is how many Windows Installer packages the bundle holds.
core-source-burn-bundle =
    { $count ->
        [one] مجموعه Burn شامل { $count } بسته Windows Installer
       *[other] مجموعه Burn شامل { $count } بسته Windows Installer
    }
# { $count } is how many paths Windows Installer recorded.
core-source-msi-database =
    { $count ->
        [one] پایگاه داده Windows Installer: { $count } مسیر کلید مؤلفه
       *[other] پایگاه داده Windows Installer: { $count } مسیر کلید مؤلفه
    }
core-source-inno-log = گزارش حذف نصب Inno Setup (unins000.dat)
# { $count } is how many paths the log records.
core-source-inno-log-paths =
    { $count ->
        [one] گزارش حذف نصب Inno Setup: { $count } مسیر ثبت‌شده
       *[other] گزارش حذف نصب Inno Setup: { $count } مسیر ثبت‌شده
    }
core-source-nsis = امضای برنامه حذف نصب NSIS (Nullsoft)
# { $count } is how many paths the log records.
core-source-nsis-log-paths =
    { $count ->
        [one] NSIS AdvUninstLog uninstall.log: { $count } مسیر ثبت‌شده
       *[other] NSIS AdvUninstLog uninstall.log: { $count } مسیر ثبت‌شده
    }
core-source-squirrel = چیدمان ویژه کاربر Squirrel (Update.exe / app-*)
# { $version } is a folder's name that says the version, such as app-1.2.3.
core-source-squirrel-version = نسخه فعلی Squirrel: { $version }
core-source-chocolatey = فراداده بسته Chocolatey
core-source-msix = بسته MSIX / Appx
# { $identity } is a package's name, such as Contoso.App.
core-source-appx-identity = هویت AppxManifest: { $identity }
# { $count } is how many folders.
core-source-msix-data =
    { $count ->
        [one] داده‌های ویژه کاربر بسته MSIX: { $count } مکان
       *[other] داده‌های ویژه کاربر بسته MSIX: { $count } مکان
    }
# App Paths is a registry key's name.
core-source-app-paths = ثبت در App Paths

## Why a runtime is needed, as the package's report says it.

# { $program } is a program's file name, such as app.exe; { $dll } is a DLL's, such as msvcp140.dll.
core-runtime-program-imports = { $program } از { $dll } استفاده می‌کند

## What else of an app's a scan finds in the registry: fonts and certificates, and why each is the app's.

# Why a font is among the app's files, in the package's list of them.
core-registry-font-recorded = قلمی که سابقه نصب برنامه آن را نام می‌برد
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-named-for-the-app = به نام برنامه یا ناشر آن است
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-recorded = سابقه نصب برنامه آن را نام می‌برد

## Who is signed in, for the Whose Data choice. Each of the first six is why it could not be told, and starts with a
## small letter, as it is shown inside other words. An account is shown by its id, such as S-1-5-21-...-1001.

# { $error } is what Windows said.
core-profile-desktop-owner-not-said = Windows نگفت این میز کار متعلق به کیست: { $error }
# { $error } is what went wrong.
core-profile-desktop-owner-not-asked = نمی‌شد از Windows پرسید این میز کار متعلق به کیست: { $error }
core-profile-signed-in-account-no-profile = حسابی که در اینجا وارد سیستم شده است، { $account }، نمایه ثبت‌شده‌ای ندارد
core-profile-desktop-not-a-person = میز کار اینجا متعلق به { $account } است، که حساب یک شخص نیست
core-profile-nobody-signed-in-running-account-no-profile = هیچ کسی در اینجا وارد سیستم نشده است، و حسابی که این را اجرا می‌کند، { $account }، نمایه ثبت‌شده‌ای ندارد
core-profile-nobody-signed-in-running-not-a-person = هیچ کسی در اینجا وارد سیستم نشده است، و { $account }، که این را اجرا می‌کند، حساب یک شخص نیست
# SID is the name Windows gives an account's id.
core-profile-nobody-signed-in-no-sid = هیچ کسی در اینجا وارد سیستم نشده است، و این حساب SID ندارد
# { $accounts } is a list of accounts.
core-profile-several-desktops = بیش از یک حساب در این نشست میز کار دارد: { $accounts }
# Under the Whose Data choice. { $reason } is one of the messages above.
core-profile-signed-in-unknown = مشخص نشد چه کسی وارد سیستم شده است ({ $reason }).
# Under the Whose Data choice, when there are accounts to choose from. { $reason } is one of the messages above.
core-profile-signed-in-unknown-choose = مشخص نشد چه کسی وارد سیستم شده است ({ $reason }). انتخاب کنید داده‌های چه کسی گنجانده شود.
