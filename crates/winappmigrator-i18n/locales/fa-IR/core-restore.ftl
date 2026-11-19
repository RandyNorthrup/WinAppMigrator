# Words the core writes for people as it installs an app from a package: why an install will not start, the list of
# what it did (or for a preview, what it would do) that the installer window shows, its warnings, the record it keeps of
# each install so that it can be undone, and the core's errors. Message names start with core-.
#
# The list of what an install did is short and plain, one line for each thing, as the installer window lists them:
# "copied", "restored", "would copy". Words such as --force and --apply are command-line options, and LocalService,
# SYSTEM, ProgramData, and %AppData% are as Windows names them: keep them as they are.


## Errors: what went wrong, when the core cannot go on. Each is shown on its own, or after words that say what was being
## done.

core-error-list-changed = آنچه این نصب انجام می‌داد از زمان تأیید آن تغییر کرده است، بنابراین چیزی نصب نشد
# { $path } is a file or folder, such as C:\Program Files\App\app.exe, or words that name one, such as "the installer".
# { $error } is what Windows said went wrong.
core-error-io = عملیات ورودی/خروجی برای { $path } ناموفق بود: { $error }
# { $detail } is what went wrong.
core-error-registry = خواندن رجیستری ناموفق بود: { $detail }
# { $path } is a program file. { $detail } is what was wrong with it.
core-error-pe = تجزیه PE برای { $path } ناموفق بود: { $detail }
# { $detail } is what was wrong, already in words.
core-error-invalid-input = ورودی نامعتبر: { $detail }
# { $detail } is what went wrong.
core-error-serialization = سریال‌سازی گزارش ناموفق بود: { $detail }
# { $detail } is what went wrong.
core-error-archive = عملیات بایگانی ناموفق بود: { $detail }
# { $detail } is what went wrong.
core-error-network = عملیات شبکه ناموفق بود: { $detail }
# { $detail } is what went wrong.
core-error-time-format = قالب‌بندی زمان ناموفق بود: { $detail }
core-error-cancelled = لغو شد
# What is said when part of the program stopped unexpectedly and gave no reason.
core-error-panic-no-reason = هیچ دلیلی ارائه نشد

## The record of each install, kept so that it can be undone.

# { $kind } is a number that says what type of registry value it is.
core-journal-unknown-value-type = نوع مقدار رجیستری ناشناخته: { $kind }
core-journal-not-elevated = با حقوق سرپرستی اجرا نشده است
core-journal-needs-admin = نصب به حقوق سرپرستی نیاز دارد تا سابقه آن در جایی نگه داشته شود که دیگران نتوانند تغییرش دهند
# { $path } is a folder.
core-journal-not-an-install-folder = این پوشه یک نصب نیست: { $path }
# { $path } is the file the record is kept in.
core-journal-not-written = سابقه این نصب نوشته نشد: { $path }
# { $path } is a folder.
core-journal-path-escapes-root = مسیر از ریشه مورد نظر بیرون می‌رود: { $path }
# { $path } is a folder.
core-journal-nothing-exists = هیچ‌کدام از این‌ها در این رایانه وجود ندارد: { $path }
core-journal-busy = نصب یا حذف نصب دیگری در حال اجراست. صبر کنید تا تمام شود، سپس دوباره امتحان کنید
# Said of a file that was to get back the permissions it had.
core-journal-no-folder = پوشه‌ای ندارد
# Said of the permissions a file had, which were kept to give back to it.
core-journal-permissions-unreadable = آنچه نگه داشته شده بود خوانده نشد
# { $folder } is the folder of one install's record.
core-journal-unreadable-empty = از سابقه نصب در { $folder } نمی‌توان استفاده کرد: خالی است
# { $folder } is the folder of one install's record.
core-journal-unreadable-no-header = از سابقه نصب در { $folder } نمی‌توان استفاده کرد: با بیان اینکه درباره چیست شروع نمی‌شود
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-version = از سابقه نصب در { $folder } نمی‌توان استفاده کرد: نسخه دیگری از این برنامه آن را نوشته است
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-install = از سابقه نصب در { $folder } نمی‌توان استفاده کرد: می‌گوید متعلق به نصب دیگری است
# { $id } is an install's id, such as wam-1a2b3c.
core-journal-not-an-install-id = این شناسه یک نصب نیست: { $id }

## Why an install will not start, or stopped before it changed anything.

core-restore-apply-and-dry-run = --apply و --dry-run را نمی‌توان با هم به کار برد
# { $path } is the folder that was chosen to install in.
core-restore-location-not-full-path = محل نصب باید یک مسیر کامل باشد: «{ $path }»
# { $problem } says what is wrong with the folder, already in words. { $path } is the folder.
core-restore-location-unusable = از این محل نصب نمی‌توان استفاده کرد: { $problem }: «{ $path }»
core-restore-needs-admin-plan = طرح بازیابی شامل تغییراتی در رجیستری، سرویس، کار زمان‌بندی‌شده، میان‌بر یا مقصد محافظت‌شده است که فقط سرپرست می‌تواند انجام دهد
# { $path } is the folder that was chosen to install in.
core-restore-needs-admin-target = ریشه مقصد به حقوق سرپرستی نیاز دارد: { $path }
core-restore-needs-admin-user-data = بازیابی داده‌های کاربر در ProgramData یا دیگر مکان‌های محافظت‌شده به حقوق سرپرستی نیاز دارد
# { $error } is why the install stopped. { $failed } is what could not be put back, as sentences of their own.
core-restore-not-all-put-back = { $error }. همه کارهای انجام‌شده بازگردانده نشد: { $failed }
# { $error } is what went wrong.
core-restore-nowhere-to-keep-record = هیچ جای امنی برای نگه‌داشتن سابقه‌ای که برگرداندن این نصب را ممکن کند وجود ندارد، بنابراین چیزی نصب نشد: { $error }
# { $path } is a file the app would have put there.
core-restore-already-exists = { $path } از قبل وجود دارد، بنابراین چیزی نصب نشد. برای جایگزین کردن آن از --force استفاده کنید
# { $root } is a folder the app has files for. { $link } is that folder, or one it is in.
core-restore-root-under-link = این برنامه مواردی برای بازیابی در { $root } دارد، و { $link } پیوندی به جای دیگری است که شاید دیگران بتوانند آن را تغییر دهند، بنابراین چیزی نصب نشد. اگر آنچه به آنجا می‌رود داده‌های کاربر یا داده‌های مجوز است، برنامه را بدون داده‌های کاربر و داده‌های مجوز نصب کنید. اگر میان‌بری در پوشه راه‌اندازی است، آن پوشه را به جای قبلی‌اش برگردانید
core-restore-hash-check-failed = بسته در بررسی هش رد شد و چیزی نصب نشد
# { $detail } is the first thing found wrong, already in words.
core-restore-hash-check-failed-because = بسته در بررسی هش رد شد و چیزی نصب نشد: { $detail }
# { $path } is a file or folder.
core-restore-refuse-records-folder = نوشتن در جایی که نصب‌ها سوابق خود را نگه می‌دارند رد شد: { $path }
# { $path } is a file or folder.
core-restore-refuse-outside = نوشتن بیرون از محل نصب و پوشه‌های داده شناخته‌شده رد شد: { $path }
# { $path } is a file.
core-restore-no-parent-folder = مقصد پوشه والد ندارد: { $path }
# { $path } is where a file was to go.
core-restore-folder-in-the-way = یک پوشه در جایی است که این فایل باید برود: { $path }
# { $path } is a file in the package.
core-restore-file-changed = { $path } دیگر همانی نیست که بسته در خود دارد، بنابراین نصب نشد
# { $path } is a file in the package.
core-restore-source-is-link = { $path } یک پیوند است و نباید دنبال شود
# { $path } is a file in the package.
core-restore-source-became-link = { $path } هنگام باز شدن به پیوند تبدیل شد
# Why a service did not start, after "did not start:".
core-restore-service-stopped-by-itself = خودبه‌خود دوباره متوقف شد
# Why part of the app was left out, after "was left out:".
core-restore-com-class-someone-elses = این کلاس COM از قبل در این رایانه هست و متعلق به چیز دیگری است

## What an install did, one line for each thing, or for a preview, what it would do. { $source } is a file in the
## package, and { $destination } and { $path } are where it goes on this PC. { $key } is a registry key, such as
## HKLM\Software\App.

# { $package_valid } is true or false, and { $restore_valid } is true, false, or not_run: keep them as they are.
core-restore-verification-result = تأیید پس از بازیابی package_valid={ $package_valid } restore_valid={ $restore_valid }
# { $name } is a service's name.
core-restore-started-service = سرویس { $name } شروع شد
core-restore-would-copy = کپی خواهد شد: { $source } -> { $destination }
core-restore-copied = کپی شد: { $source } -> { $destination }
core-restore-would-create-empty-folder = پوشه خالی ایجاد خواهد شد: { $path }
core-restore-created-empty-folder = پوشه خالی ایجاد شد: { $path }
# { $value } is the name of a value in the key.
core-restore-would-restore-registry-value = رجیستری بازیابی خواهد شد: { $key } { $value }
core-restore-would-restore-registry-key = رجیستری بازیابی خواهد شد: { $key } (کلید)
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-would-add-firewall-rule = قانون دیوار آتش { $name } برای { $program } اضافه خواهد شد
# { $name } is a firewall rule's name.
core-restore-firewall-rule-already-there = قانون دیوار آتش { $name } از قبل وجود دارد
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-added-firewall-rule = قانون دیوار آتش { $name } برای { $program } اضافه شد
# { $name } is a service's name, { $command } what it runs, and { $account } the account it runs as, such as LocalService.
core-restore-would-restore-service = سرویس بازیابی خواهد شد: { $name } -> { $command } (اجرا به‌عنوان { $account })
# { $name } is a runtime, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-restore-prerequisite-present = پیش‌نیاز از قبل موجود است: { $name }
# { $name } is a runtime that Windows has as a Windows feature.
core-restore-would-install-prerequisite-feature = پیش‌نیاز { $name } نصب خواهد شد (ویژگی Windows)
# { $name } is a runtime whose installer is in the package.
core-restore-would-install-prerequisite-bundled = پیش‌نیاز { $name } نصب خواهد شد (نصب‌کننده همراه بسته)
# { $name } is a runtime that would come through winget, which is a program's name.
core-restore-would-install-prerequisite-winget = پیش‌نیاز { $name } نصب خواهد شد (winget)
# { $name } is a runtime.
core-restore-installed-prerequisite = پیش‌نیاز { $name } نصب شد
# { $name } is a runtime.
core-restore-installed-prerequisite-reboot = پیش‌نیاز { $name } نصب شد (راه‌اندازی مجدد لازم است)
core-restore-would-restore-license = مجوز بازیابی خواهد شد: { $source } -> { $destination }
core-restore-restored-license = مجوز بازیابی شد: { $source } -> { $destination }
core-restore-would-restore-license-registry = رجیستری مجوز بازیابی خواهد شد: { $key }
# { $name } is a driver's name.
core-restore-driver-not-restored = درایور به‌طور خودکار بازیابی نشد: { $name }
# { $name } is a driver's name.
core-restore-would-install-driver = درایور نصب خواهد شد: { $name } -> { $destination }
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-would-restore-task = کار زمان‌بندی‌شده بازیابی خواهد شد: { $name }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-startup-shortcut = میان‌بر راه‌اندازی بازیابی خواهد شد: { $shortcut } -> { $target }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-shortcut = میان‌بر بازیابی خواهد شد: { $shortcut } -> { $target }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-would-restore-variable =
    { $scope ->
        [user] متغیر کاربر { $name } بازیابی خواهد شد
       *[machine] متغیر سیستم { $name } بازیابی خواهد شد
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-left-variable =
    { $scope ->
        [user] متغیر کاربر { $name } همان‌طور که بود باقی ماند: هیچ بخشی از آن متعلق به این برنامه نیست
       *[machine] متغیر سیستم { $name } همان‌طور که بود باقی ماند: هیچ بخشی از آن متعلق به این برنامه نیست
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-restored-variable =
    { $scope ->
        [user] متغیر کاربر { $name } بازیابی شد
       *[machine] متغیر سیستم { $name } بازیابی شد
    }
core-restore-set-uninstaller = برنامه حذف نصب برای { $key } تنظیم شد
core-restore-placed-driver-image = تصویر درایور قرار داده شد: { $source } -> { $destination }
# { $name } is a driver's name.
core-restore-configured-driver = درایور { $name } پیکربندی شد
# { $name } is a driver's name.
core-restore-created-driver = درایور { $name } ایجاد شد
# { $name } is a driver's name.
core-restore-started-driver = درایور { $name } شروع شد
core-restore-ensured-registry-key = از وجود کلید رجیستری { $key } اطمینان حاصل شد
# { $value } is the name of a value in the key.
core-restore-restored-registry-value = رجیستری بازیابی شد: { $key } { $value }
# "(Default)" is how Windows names the value a key has with no name: write it as Registry Editor does.
core-restore-restored-registry-default = رجیستری بازیابی شد: { $key } (پیش‌فرض)
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-configured-service = سرویس { $name } پیکربندی شد (اجرا به‌عنوان { $account })
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-created-service = سرویس { $name } ایجاد شد (اجرا به‌عنوان { $account })
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-restored-task = کار زمان‌بندی‌شده بازیابی شد: { $name }
core-restore-restored-shortcut = میان‌بر بازیابی شد: { $path }

## Warnings an install gives, and why it failed. They are shown after it, on the PC it ran on.

core-warning-restore-earlier-not-trusted = سابقه نصبی که هرگز تمام نشد دست‌نخورده ماند، زیرا دیگران می‌توانند آن را تغییر دهند. هر تغییری که آن نصب ایجاد کرده بود همچنان وجود دارد.
# { $app_name } is the app's name. { $count } is how many changes were put back.
core-warning-restore-earlier-undone =
    { $count ->
        [one] نصب قبلی { $app_name } هرگز تمام نشد، بنابراین ابتدا برگردانده شد. { $count } تغییر بازگردانده شد.
       *[other] نصب قبلی { $app_name } هرگز تمام نشد، بنابراین ابتدا برگردانده شد. { $count } تغییر بازگردانده شد.
    }
# { $app_name } is the app's name.
core-warning-restore-earlier-never-finished = نصب قبلی { $app_name } هرگز تمام نشد. نصب ابتدا آن را برمی‌گرداند.
core-warning-restore-user-data-real-locations = داده‌های کاربر به‌جای ریشه مقصد، در مکان‌های واقعی %AppData%/%ProgramData% این رایانه بازیابی می‌شوند.
# { $error } is what went wrong.
core-warning-restore-commit-failed = سابقه این نصب تکمیل نشد: { $error }
# { $name } is a service's name.
core-warning-restore-service-still-starting = از سرویس { $name } خواسته شد شروع شود و هنوز شروع آن تمام نشده است. شاید بعداً شروع شود. همچنین در راه‌اندازی بعدی این رایانه خودبه‌خود شروع می‌شود.
# { $name } is a service's name. { $reason } is why it did not start, as Windows said it.
core-warning-restore-service-not-started = سرویس { $name } نصب شد و شروع نشد: { $reason }. شاید تا شروع آن، برنامه کار نکند. Windows در راه‌اندازی بعدی این رایانه دوباره تلاش می‌کند.
# { $name } is a service's name. { $reason } is why it could not be stopped.
core-warning-restore-service-not-stopped = سرویس { $name } ابتدا متوقف نشد: { $reason }. شاید تا راه‌اندازی بعدی این رایانه، همچنان برنامه قدیمی را اجرا کند.
core-warning-restore-install-cancelled = نصب لغو شد.
core-warning-restore-preview-cancelled = پیش‌نمایش لغو شد.
# { $item } is what was left out, such as a file or a service's name. { $reason } is why, already in words.
core-warning-restore-left-out = { $item } کنار گذاشته شد: { $reason }.
# { $error } is what went wrong.
core-warning-restore-verification-failed = تأیید پس از بازیابی اجرا نشد: { $error }
# { $runner } says what relies on the file: a service, a scheduled task, a firewall rule, a shortcut, or a registry value.
# { $path } is the file. { $name } is the name of what relies on it. { $refusal } is why the file was not replaced,
# already in words.
core-warning-restore-program-would-run =
    { $runner ->
        [service] جایگزین کردن { $path } در اختیار این بسته نیست و سرویس { $name } به آن وابسته است، بنابراین چیزی نصب نشد: { $refusal }
        [task] جایگزین کردن { $path } در اختیار این بسته نیست و کار زمان‌بندی‌شده { $name } به آن وابسته است، بنابراین چیزی نصب نشد: { $refusal }
        [firewall] جایگزین کردن { $path } در اختیار این بسته نیست و قانون دیوار آتش { $name } به آن وابسته است، بنابراین چیزی نصب نشد: { $refusal }
        [shortcut] جایگزین کردن { $path } در اختیار این بسته نیست و میان‌بر { $name } به آن وابسته است، بنابراین چیزی نصب نشد: { $refusal }
       *[registry] جایگزین کردن { $path } در اختیار این بسته نیست و مقدار رجیستری { $name } به آن وابسته است، بنابراین چیزی نصب نشد: { $refusal }
    }
core-warning-restore-destination-exists = مقصد وجود دارد؛ برای بازنویسی از --force استفاده کنید.
# { $path } is a file. { $error } is what went wrong.
core-warning-restore-file-not-written = { $path } نوشته نشد: { $error }
core-warning-restore-not-a-folder = چیزی غیر از پوشه در جایی است که این پوشه خالی باید قرار گیرد.
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-not-looked-at = { $path } بررسی نشد: { $error }
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-folder-not-made = { $path } ایجاد نشد: { $error }
# Why part of the app was left out, after "was left out:".
core-warning-restore-registration-exists = این رایانه از قبل این مورد را با تنظیم دیگری دارد و نصب آن را تصاحب نمی‌کند
core-warning-restore-registry-review = مقادیر رجیستری از داده‌های ساختاریافته بایگانی‌شده بازیابی شدند و مسیرهای شناخته‌شده برنامه به ریشه مقصد تغییر داده شدند.
# { $error } is what went wrong.
core-warning-restore-firewall-unreadable = قوانین دیوار آتش Windows خوانده نشد: { $error }
# Why a firewall rule was left out, after "was left out:".
core-warning-restore-firewall-rule-exists = قانونی با این نام برای این برنامه از قبل وجود دارد و تنظیم دیگری دارد، که نصب آن را تغییر نمی‌دهد
# { $name } is a runtime.
core-warning-restore-prerequisite-unknown = { $name }: هیچ نصب‌کننده شناخته‌شده‌ای برای این زمان اجرا وجود ندارد.
# { $name } is a runtime.
core-warning-restore-prerequisite-needs-admin = نصب { $name } به حقوق سرپرستی نیاز دارد؛ نصب‌کننده را دوباره به‌عنوان سرپرست اجرا کنید.
# { $name } is a runtime.
core-warning-restore-prerequisite-reboot = { $name } نصب شد، اما برای تکمیل آن راه‌اندازی مجدد لازم است.
# { $error } is what went wrong.
core-warning-restore-skipped-not-recorded = زمان اجرایی که از آن صرف‌نظر شد ثبت نشد: { $error }
# { $name } is a runtime. { $reason } is why it was skipped, already in words.
core-warning-restore-prerequisite-skipped = { $name }: { $reason }
# { $name } is a runtime. { $reason } is why it could not be installed. { $url } is a web address to get it from.
core-warning-restore-prerequisite-failed = { $name } نصب نشد: { $reason }. آن را از { $url } تهیه کنید.
# { $error } is what went wrong.
core-warning-restore-runtime-not-recorded = زمان اجرای نصب‌شده ثبت نشد: { $error }
# { $error } is what went wrong.
core-warning-restore-license-source-invalid = مسیر مجوز در بسته نامعتبر است: { $error }
core-warning-restore-license-needs-admin = بازیابی داده‌های مجوز در مکان محافظت‌شده به حقوق سرپرستی نیاز دارد.
core-warning-restore-license-exists = مقصد مجوز وجود دارد؛ برای بازنویسی از --force استفاده کنید.
# { $error } is what went wrong.
core-warning-restore-license-failed = داده‌های مجوز بازیابی نشد: { $error }
core-warning-restore-driver-needs-admin = ثبت درایور به حقوق سرپرستی نیاز دارد؛ نصب‌کننده را دوباره به‌عنوان سرپرست اجرا کنید.
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-warning-restore-task-runs-as-system = کار زمان‌بندی‌شده { $name } به‌عنوان SYSTEM و با حقوق کامل بر این رایانه اجرا می‌شود.
core-warning-restore-shortcut-exists = میان‌بر وجود دارد؛ برای بازنویسی از --force استفاده کنید.
core-warning-restore-shortcut-all-users = یک یا چند میان‌بر بازیابی‌شده در میز کار عمومی یا منوی شروع مشترک قرار دارند.
core-warning-restore-environment-refresh = شاید تغییرات متغیرهای محیطی فقط در یک فرایند، پوسته یا نشست ورود جدید دیده شوند.
# { $name } is a service's name. { $account } is the account it ran as on the PC it came from.
core-warning-restore-service-account = سرویس { $name } در رایانه مبدأ به‌عنوان { $account } اجرا می‌شد. گذرواژه آن در نصب‌کننده نیست، بنابراین سرویس به‌عنوان LocalService راه‌اندازی می‌شود. اگر کار نکرد، حساب و گذرواژه آن را در «سرویس‌ها» تنظیم کنید.
# { $name } is a runtime.
core-warning-restore-prerequisite-pending-reboot = { $name } هنوز ثبت نشده است؛ باید پس از راه‌اندازی مجدد فعال شود.
# { $name } is a runtime.
core-warning-restore-prerequisite-not-verified = { $name } موفقیت را گزارش کرد اما ثبت نشد؛ شاید نصب ناموفق بوده باشد.
# { $error } is what went wrong.
core-warning-restore-driver-source-invalid = مسیر درایور همراه بسته نامعتبر است: { $error }
# { $error } is what went wrong.
core-warning-restore-driver-not-read = تنظیمات فعلی درایور خوانده نشد: { $error }
core-warning-restore-driver-exists = سرویس درایور وجود دارد؛ برای پیکربندی دوباره آن از --force استفاده کنید.
core-warning-restore-driver-image-exists = تصویر درایور وجود دارد؛ برای بازنویسی از --force استفاده کنید.
core-warning-restore-driver-hash-bundled = تصویر درایور همراه بسته با هش ثبت‌شده آن مطابقت ندارد، بنابراین نصب نشد.
# { $error } is what went wrong.
core-warning-restore-driver-not-placed = تصویر درایور قرار داده نشد: { $error }
core-warning-restore-driver-hash-placed = تصویر درایور پس از قرار گرفتن تغییر کرد، بنابراین ثبت نشد.
# { $error } is what went wrong.
core-warning-restore-key-not-created = کلید رجیستری ایجاد نشد: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-read = مقداری که این جایگزینش می‌شد خوانده نشد: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-written = مقدار رجیستری نوشته نشد: { $error }
core-warning-restore-value-exists = مقدار رجیستری وجود دارد؛ برای بازنویسی از --force استفاده کنید.
# { $error } is what went wrong.
core-warning-restore-environment-key-not-created = کلید رجیستری متغیرهای محیطی ایجاد نشد: { $error }
core-warning-restore-variable-exists = متغیر محیطی وجود دارد؛ برای بازنویسی از --force استفاده کنید.
# { $error } is what went wrong.
core-warning-restore-variable-not-written = متغیر محیطی نوشته نشد: { $error }
core-warning-restore-value-kind-missing = نوع مقدار رجیستری مشخص نیست.
# { $error } is what went wrong.
core-warning-restore-service-not-read = تنظیمات فعلی سرویس خوانده نشد: { $error }
core-warning-restore-service-exists = سرویس وجود دارد؛ برای پیکربندی دوباره آن از --force استفاده کنید.
# { $error } is what went wrong.
core-warning-restore-task-not-queried = پرس‌وجوی کار زمان‌بندی‌شده ناموفق بود: { $error }
core-warning-restore-task-exists = کار زمان‌بندی‌شده وجود دارد؛ برای بازنویسی از --force استفاده کنید.
# { $error } is what went wrong.
core-warning-restore-task-folder-not-looked-at = پوشه کار بررسی نشد: { $error }
# { $error } is what went wrong.
core-warning-restore-shortcut-not-put-back = میان‌بر ایجاد نشد و میان‌بری که جایگزینش می‌شد بازگردانده نشد: { $error }
# { $error } is what went wrong.
core-warning-restore-command-not-started = اجرای فرمان ناموفق بود: { $error }
# { $output } is what the command wrote, as it wrote it.
core-warning-restore-command-failed = فرمان ناموفق بود: { $output }
# { $error } is what went wrong.
core-warning-restore-powershell-not-started = اجرای PowerShell ناموفق بود: { $error }
