# Words the core writes for people while it builds a package from an installed app: what a package records for an
# installer to show later, errors shown in the app, the list of what goes in and what is left out, opening a disk
# image or another PC over the network, and what an installer says when the package it carries cannot be read.
# Message names start with core-.


## Scanning an app, before a package is built. Shown in the app when a scan cannot go on.

# { $path } is the app's program, such as D:\Program Files\Acme\Acme.exe, on a drive taken from another PC.
core-scan-program-not-on-the-drive = { $path } از یک پیوند عبور می‌کند یا روی درایو متصل نیست، بنابراین خوانده نشد.
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-not-on-this-pc = { $path } روی دیسکی از این رایانه نیست، یا نامی دارد که چیز دیگری را باز می‌کند، بنابراین خوانده نشد.
# { $path } is the app's program.
core-scan-program-someone-elses = { $path } در پوشه‌های حساب دیگری است یا متعلق به این حساب نیست، و سابقه خود این برنامه نمی‌گوید که به آنجا تعلق دارد، بنابراین خوانده نشد.
# { $app_name } is the app's name, as the list of apps calls it.
core-scan-nothing-to-package = هیچ فایل اجرایی متعلق به خود { $app_name } پیدا نشد، بنابراین چیزی برای قرار دادن در نصب‌کننده وجود ندارد. درایورها، زمان‌های اجرا و افزونه‌ها اغلب چنین‌اند: در رایانه جدید، درایور را از سازنده آن یا Windows Update نصب کنید، و زمان اجرا یا افزونه را همراه با برنامه‌ای که از آن استفاده می‌کند نصب کنید.
# { $app_name } is the app's name, as the list of apps calls it. A web app is a website installed as an app, with
# a window and a Start menu entry of its own.
core-scan-web-app = { $app_name } یک برنامه وب است که یک مرورگر از یک وب‌سایت نصب کرده است. فایل اجرایی خودش را ندارد، بنابراین چیزی برای قرار دادن در نصب‌کننده وجود ندارد. در رایانه جدید، آن وب‌سایت را در همان مرورگر باز کنید و برنامه را از همان‌جا نصب کنید.
# { $path } is the program the app's own record names, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-missing = { $path } دیگر آنجا نیست، هرچند سابقه این برنامه آن را فایل اجرایی برنامه معرفی می‌کند. شاید برنامه جابه‌جا شده یا بخشی از آن حذف شده باشد: تعمیر یا نصب دوباره آن در این رایانه باید آن را بازگرداند.
# { $path } is the file that was picked.
core-scan-not-a-file = مقصد اسکن یک فایل نیست: { $path }
# Names something left out of a package, in a list of what was left out and why. { $name } is the variable's name,
# such as ACME_LICENSE_KEY.
core-scan-environment-variable-item = متغیر محیطی { $name }

## Folders and parts that might also be the app's, offered in a list to tick. Each says why it is offered.

core-near-starts-with-app-name = نام آن با نام این برنامه شروع می‌شود.
core-near-has-app-name = نام این برنامه در نام آن آمده است.
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-starts-with-publisher = نام آن با نام ناشر، { $publisher }، شروع می‌شود.
# { $folder } is a folder's name, such as Program Files.
core-near-hidden-part-beside = بخشی پنهان از فهرست برنامه‌ها، که کنار این برنامه در { $folder } نصب شده است.
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-hidden-part-publisher = بخشی پنهان از فهرست برنامه‌ها، از همان ناشر، { $publisher }.

## What a scan found, as a person chooses from it: the folders that go in, and what is left out, and why

# Why a folder cannot be left out, beside its number of files.
core-footprint-locked-app-folder = این‌ها همیشه همراه فایل اجرایی اصلی می‌روند
core-footprint-locked-main-program = فایل اجرایی اصلی را در خود دارد، که همیشه گنجانده می‌شود
core-footprint-locked-needed = چیزی را در خود دارد که فایل اجرایی اصلی بدون آن شروع نمی‌شود
core-footprint-locked-font = همراه قلم خود، در بخش یکپارچگی با Windows، می‌رود
# Where a folder is. "AppData" and "ProgramData" are the names of Windows folders; keep them as they are.
core-footprint-place-roaming = تنظیمات در نمایه شما (AppData)
core-footprint-place-local = تنظیمات ویژه این رایانه در نمایه شما (AppData)
core-footprint-place-program-data = مشترک بین همه کاربران این رایانه (ProgramData)
core-footprint-place-documents = در اسناد شما
core-footprint-place-saved-games = در بازی‌های ذخیره‌شده شما
core-footprint-place-public-documents = در اسناد عمومی
core-footprint-place-app-folder = در پوشه برنامه
# { $folder } is a folder, such as C:\Users\Sam\AppData\Roaming.
core-footprint-place-in = در { $folder }
# A row for the files directly in the app's folder. { $folder } is that folder's name, such as Acme.
core-footprint-files-directly-in = فایل‌هایی که مستقیماً در { $folder } هستند
# Headings over what is left out, and why each item is.
core-footprint-licenses-left-out = در این نصب‌کننده نیست، زیرا گزینه «داده‌های مجوز و فعال‌سازی» خاموش است
core-footprint-font-stays-behind = جا می‌ماند.
core-footprint-drivers-left-out = درایورهایی که متعلق به Windows یا یک سازنده‌اند و نمی‌توان آن‌ها را منتقل کرد؛ آن‌ها را در رایانه دیگر دوباره نصب کنید
core-footprint-driver-reinstall = آن را از بسته خود سازنده‌اش دوباره نصب کنید.
# Why something else is left out along with what the person left out. { $service } is a service's name, such as AcmeSync.
core-footprint-needs-left-out = به { $service } نیاز دارد، که کنار گذاشته شده است
core-footprint-told-to-use-left-out = تنظیم شده است تا از چیزی در پوشه‌ای استفاده کند که کنار گذاشتید
core-footprint-file-in-left-out = فایل آن در پوشه‌ای است که کنار گذاشتید
core-footprint-program-in-left-out = فایل اجرایی مربوط به آن در پوشه‌ای است که کنار گذاشتید
core-footprint-names-left-out = پوشه‌ای را نام می‌برد که کنار گذاشتید
# Why something is left out, as the installer tells whoever installs it.
core-footprint-builder-left-it-out = کسی که این نصب‌کننده را ساخت آن را کنار گذاشت
# Something the person left out, by kind and name, such as "Service Acme Updater". { $kind } is the kind, such as
# Service or Scheduled Task; { $title } is its name.
core-footprint-left-out-item = { $kind } { $title }

## Building a package and an installer from a scan. Shown in the app when a build cannot go on.

core-package-scan-of-another-app = این اسکن مربوط به برنامه‌ای غیر از برنامه در حال ساخت است، بنابراین چیزی ساخته نشد
core-package-scan-brought-in-more = این اسکن داده‌هایی آورد که از این ساخت خواسته نشده بود، بنابراین چیزی ساخته نشد
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-package-program-gone = { $path } دیگر وجود ندارد، بنابراین چیزی ساخته نشد. برنامه را دوباره بررسی کنید.
core-package-needs-stub = بایگانی خوداستخراج به پایه نصب‌کننده نیاز دارد
# { $reason } is what went wrong.
core-package-staged-file-failed = فایلی در پوشه آماده‌سازی نتوانست در بسته قرار گیرد: { $reason }
# { $shortfalls } says which drives are short of room and by how much, in sentences of its own.
core-package-not-enough-room = فضای کافی برای ساخت این نصب‌کننده وجود ندارد، بنابراین چیزی خوانده یا نوشته نشد. { $shortfalls }. در آنجا مقداری فضا آزاد کنید، یا یک پوشه خروجی در درایو دیگری انتخاب کنید.
# { $path } is a file's path, on a drive taken from another PC.
core-package-not-read-off-the-drive = { $path } از یک پیوند عبور می‌کند یا روی درایو متصل نیست، بنابراین خوانده نشد
# { $path } is a file's path.
core-package-not-read-not-local = { $path } روی دیسکی از این رایانه نیست، بنابراین خوانده نشد
# { $path } is a file's path.
core-package-not-read-other-account = { $path } به پوشه‌های حساب دیگری می‌رسد، بنابراین خوانده نشد
# { $path } is a file's path, as the PC it came from called it.
core-package-not-on-the-drive = { $path } روی درایو متصل نیست، بنابراین خوانده نشد
# { $path } is a file's path.
core-package-backup-copy-no-parent = نمی‌توان از فایلی که پوشه والد ندارد کپی پشتیبان گرفت: { $path }
# { $path } is a file's path.
core-package-backup-copy-destination-no-parent = مقصد کپی پشتیبان پوشه والد ندارد: { $path }
# robocopy is the name of a Windows command; keep it as it is. { $code } is how it ended, such as Some(8).
core-package-robocopy-failed = کپی robocopy در حالت پشتیبان‌گیری ناموفق بود (کد خروج { $code })
# Stands for a folder where a file's name would be, as in "I/O failed for temporary package folder: …".
core-package-temporary-folder = پوشه موقت بسته
# Stands for the installer being written where a file's name would be, as in "I/O failed for the installer: …".
core-package-the-installer = نصب‌کننده
# { $path } is a file's path, or its name in the package. { $error } is what went wrong with it.
core-package-path-error = { $path }: { $error }
# { $name } is a file's name in the package, such as app/bin/App.exe.
core-package-same-place = فایل دیگری همین جایگاه را در بسته دارد: { $name }
# { $name } is a file's name in the package.
core-package-changed-in-protected-folder = { $name } در حین ساخت بسته، در پوشه محافظت‌شده تغییر کرد
core-package-worker-died = رشته‌ای که فایل‌ها را فشرده می‌کرد به‌طور غیرمنتظره متوقف شد، بنابراین هیچ نصب‌کننده‌ای نوشته نشد
# { $path } is a file's path.
core-package-link-not-read = { $path } یک پیوند است، بنابراین خوانده نشد
# { $path } is a file's path. { $error } is what went wrong.
core-package-not-compressed = { $path } فشرده نشد: { $error }
# { $path } is a file's path. { $reason } is what went wrong.
core-package-not-read-back = { $path } بازخوانی نشد: { $reason }

## Where an app's files are, and where an installer may write them. Shown when a path is refused.

# { $path } is a folder, such as C:\Program Files\Acme.
core-discovery-root-not-full-path = پوشه‌ای که باید زیر آن نوشته شود باید یک مسیر کامل باشد: { $path }
# { $path } is a folder's path.
core-discovery-root-not-a-folder = پوشه‌ای که باید زیر آن نوشته شود، پوشه نیست: { $path }
# { $path } is a file's path.
core-discovery-escapes-root = مسیر از ریشه مورد نظر بیرون می‌رود: { $path }
# { $path } is the link's path.
core-discovery-refusing-to-write-through-link = نوشتن از طریق پیوند رد شد: { $path }
# { $path } is the link's path.
core-discovery-refusing-link = عبور از پیوند رد شد: { $path }
# { $error } is what Windows said went wrong.
core-discovery-refusing-link-because = عبور از پیوند رد شد: { $error }
# { $path } is the link's path.
core-discovery-link-to-somewhere-else = { $path } پیوندی به جای دیگری است و شاید دیگران بتوانند مقصد آن را تغییر دهند. از جایی استفاده کنید که پیوند به آن می‌رسد
# { $path } is a folder's path.
core-discovery-no-parent-folder = { $path } پوشه والد ندارد
# { $path } is the install location as it was given. Keep the quotes, or use your language's own.
core-discovery-install-location-not-full-path = محل نصب باید یک مسیر کامل باشد: «{ $path }»
# { $path } is the install location.
core-discovery-install-location-a-link = محل نصب پیوندی به جای دیگری است: { $path }
# { $path } is a file's path.
core-discovery-device-name = مسیر از نامی استفاده می‌کند که برای یک دستگاه Windows رزرو شده است: { $path }
# { $path } is a file's path.
core-discovery-device-name-on-older-windows = { $path } نامی دارد که این نسخه از Windows آن را به‌عنوان دستگاه باز می‌کند و نه به‌عنوان فایل، بنابراین چیزی در آن نوشته نشد. می‌توان آن را روی Windows 11 نصب کرد
# { $path } is a file's path.
core-discovery-unsafe-path = مسیر ناامن: { $path }
# { $path } is a file's path.
core-discovery-no-package-path = فایل مسیر نسبی در بسته ندارد: { $path }

## Writing an installer, and reading the package inside one. Most of these are shown by an installer, on the PC it
## runs on, when it cannot read what it carries. "The package" is the app's files and settings that an installer
## carries; "the stub" is the program part of an installer, before the package; "the data file" is the .wamdata file
## that holds the package of a very large installer, beside it.

# Stands for the package where a file's name would be, as in "I/O failed for installer package: …".
core-archive-installer-package = بسته نصب‌کننده
# Stands for the installer program built into WinAppMigrator, where a file's name would be.
core-archive-embedded-stub = پایه نصب‌کننده جاسازی‌شده
# { $index } is the number of a file in the package. { $reason } is what went wrong.
core-archive-entry-unreadable = مورد شماره { $index } بسته خوانده نشد: { $reason }
# { $name } is a file's name inside the package, such as app/bin/App.exe.
core-archive-entry-unsafe-path = ورودی zip مسیر ناامن دارد: { $name }
# { $name } is a file's name inside the package.
core-archive-holds-more = { $name } بیش از آنچه می‌گوید در خود دارد
# { $name } is a file's name inside the package.
core-archive-name-too-long = این نام برای zip بیش از حد طولانی است: { $name }
core-archive-changed-while-copied = بسته در حین قرار گرفتن پس از پایه نصب‌کننده تغییر کرد، بنابراین هیچ نصب‌کننده‌ای نوشته نشد
core-archive-stub-not-a-program = بخش پیش از بسته در این نصب‌کننده یک فایل اجرایی نیست
core-archive-damaged = این نصب‌کننده آسیب دیده است یا پس از ساخت تغییر کرده است، بنابراین چیزی از بسته آن خوانده نشد. آن را دوباره از جایی که ساخته شد کپی کنید.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-missing = این نصب‌کننده بسته خود را در فایلی به نام { $name }، در همین پوشه، نگه می‌دارد و آن فایل اینجا نیست. این دو فایل باید کنار هم بمانند: { $name } را از جایی که نصب‌کننده ساخته شد به اینجا کپی کنید، سپس دوباره این را اجرا کنید.
# { $name } is the data file's name, such as App_installer.wamdata. { $error } is what Windows said went wrong.
core-archive-data-file-unopened = فایل داده این نصب‌کننده، { $name }، باز نشد: { $error }. این دو فایل باید کنار هم، در پوشه‌ای بمانند که این رایانه بتواند آن را بخواند.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-a-link = { $name } یک پیوند یا پوشه است، و فایل داده این نصب‌کننده باید یک فایل مستقل، درست کنار آن باشد، بنابراین چیزی از آن خوانده نشد. فایل واقعی را اینجا کپی کنید.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-wrong = { $name } فایل داده این نصب‌کننده نیست، یا آسیب دیده است یا پس از ساخت تغییر کرده است، بنابراین چیزی از آن خوانده نشد. هر دو فایل را دوباره از جایی که ساخته شدند کپی کنید.
core-archive-stub-signed = پایه نصب‌کننده امضا شده است؛ باید بدون امضا بماند، پس به‌جای آن نصب‌کننده نهایی را امضا کنید
# { $size } is a size, such as 4.2 GB.
core-archive-too-big-to-run = اندازه نصب‌کننده { $size } می‌شد و Windows نمی‌تواند فایل اجرایی 4 گیگابایتی یا بزرگ‌تر را اجرا کند؛ چیزی نوشته نشد
core-archive-outside-signature = نصب‌کننده داده‌هایی بیرون از امضای خود دارد و شاید دست‌کاری شده باشد
core-archive-footer-length = طول پانویس بایگانی خوداستخراج نامعتبر است
core-archive-seek-outside = جابه‌جایی به بیرون از بسته
# { $path } is a file's path.
core-archive-unsafe-package-path = مسیر ناامن در بسته: { $path }
# { $path } is a file's path.
core-archive-empty-package-path = مسیر خالی در بسته: { $path }

## Opening a disk image of another PC (a .vhd or .vhdx file) to read apps from it

# { $path } is the disk image file, such as D:\Backups\OldPC.vhdx.
core-image-not-attachable = { $path } تصویر دیسکی نیست که Windows بتواند پیوست کند (.vhd یا .vhdx)
core-image-needs-administrator = پیوست کردن تصویر دیسک به حقوق سرپرستی نیاز دارد
# { $path } is the disk image file the person picked.
core-image-not-a-file = { $path } یک فایل نیست
# { $path } is the disk image file. "Edit Disk" is the name of a Hyper-V Manager command.
core-image-differencing = { $path } یک دیسک تفاضلی است که فقط تغییرات نسبت به دیسک مبنای خود را نگه می‌دارد، و Windows آن دیسک را نیز، هر جا که بگوید هست، باز می‌کند. ابتدا آن را با فرمان Edit Disk در Hyper-V با آن دیسک ادغام کنید و آن دیسک را انتخاب کنید
# { $path } is the disk image file.
core-image-unreadable = { $path } یک فایل .vhd یا .vhdx قابل خواندن نیست
# ProgramData is the name of a Windows folder; keep it as it is.
core-image-no-program-data = پوشه ProgramData این رایانه پیدا نشد
# The reason given in core-image-could-not-attach when Windows already has the image open.
core-image-already-attached = Windows از قبل آن را پیوست کرده است و هر چه آن را پیوست کرده، شاید در حال نوشتن در آن باشد. ابتدا آن را جدا کنید، یا درایو آن را به‌عنوان درایو متصل انتخاب کنید
# { $path } is the disk image file. { $reason } is why, such as core-image-already-attached or what Windows said.
core-image-could-not-attach = { $path } پیوست نشد: { $reason }
# { $path } is the disk image file.
core-image-no-windows-volume = { $path } هیچ حجم قابل خواندنی ندارد که Windows روی آن باشد. قفل حجمی که با BitLocker رمزگذاری شده است باید ابتدا باز شود.
# Given as the reason when Windows failed without saying anything.
core-image-windows-did-not-say = Windows دلیل آن را نگفت

## Reading another PC over the network, through the client running on it. Shown as why something on that PC
## could not be read. "The client" is WinAppMigrator Client, the small program on the other PC.

core-remote-too-slow = پاسخ رایانه دیگر بیش از حد طول کشید
core-remote-closed = اتصال با رایانه دیگر بسته است
core-remote-failed = اتصال با رایانه دیگر ناموفق بود
core-remote-answered-something-else = کلاینت پاسخ دیگری داد
# { $name } is the PC's name or address as the person typed it, such as OLDPC or 192.168.1.20.
core-remote-not-found = { $name } در شبکه پیدا نشد
core-remote-did-not-say-what-it-is = کلاینت نگفت چه برنامه‌ای است
core-remote-ordinary-names-only = فقط درباره مسیری زیر ریشه رایانه دیگر، با نام‌های معمولی، پرسیده می‌شود
core-remote-more-but-none = رایانه دیگر گفت پوشه‌ای موارد بیشتری دارد، اما هیچ‌کدام را نفرستاد
# { $count } is the most things one folder may hold, such as 250,000.
core-remote-too-many-listed = رایانه دیگر بیش از { $count } مورد را در یک پوشه فهرست کرد
core-remote-sent-too-much = رایانه دیگر بیش از آنچه خواسته شده بود از یک فایل فرستاد
core-remote-name-not-text = نمی‌توان درباره نامی که متن نیست پرسید
core-remote-below-root-only = فقط درباره مسیری زیر ریشه رایانه دیگر می‌توان پرسید
core-remote-file-ended-early = فایل رایانه دیگر زودتر از انتظار تمام شد
core-remote-link-not-followed = پیوندی در رایانه دیگر دنبال نمی‌شود
core-remote-only-files-opened = در رایانه دیگر فقط فایل باز می‌شود
core-remote-listing-said-nothing = فهرست چیزی درباره آن نگفت
core-remote-change-time-unknown = زمان تغییر آن مشخص نیست
core-remote-seek-before-start = جابه‌جایی به پیش از ابتدا
# { $size } is how many bytes a file is; { $limit } is the most that were expected. Both are numbers.
core-remote-too-large =
    { $size ->
        [one] اندازه آن { $size } بایت است و انتظار نمی‌رفت بیش از { $limit } باشد
       *[other] اندازه آن { $size } بایت است و انتظار نمی‌رفت بیش از { $limit } باشد
    }

## Checking that a download is signed by Microsoft. Each is given as the reason a check failed.

# { $error } is what Windows said went wrong.
core-signature-no-powershell = PowerShell اجرا نشد: { $error }
# { $error } is what went wrong reading PowerShell's answer.
core-signature-unreadable = امضا خوانده نشد: { $error }
# { $signer } is who signed the file, such as Contoso Ltd.
core-signature-not-microsoft = توسط { $signer } امضا شده است، نه Microsoft
core-signature-not-microsoft-unnamed = توسط شخص دیگری امضا شده است، نه Microsoft
# { $status } is Windows' own word for the signature's state, such as HashMismatch or NotSigned, which is not translated.
core-signature-status = وضعیت امضای آن { $status } است

## Warnings a package records when it is built from an app, which an installer shows later, maybe on another PC, in
## that PC's language. "This app" is the app being packaged.

core-warning-build-program-needs-unreadable = آنچه این برنامه از Windows نیاز دارد خوانده نشد، بنابراین بررسی نمی‌شود که آیا می‌تواند در رایانه دیگر اجرا شود.
core-warning-build-external-manifest = مانیفست خارجی برنامه شناسایی شد؛ تفسیر وابستگی‌های اجرای کنار هم (side-by-side) به بعد موکول شد.
core-warning-build-non-exe = فایل انتخاب‌شده پسوند .exe ندارد؛ اسکن PE ادامه می‌یابد.
core-warning-build-stand-in-program = هیچ فایل اجرایی معمولی برای برنامه پیدا نشد، بنابراین این فایل جای آن را می‌گیرد. نتایج معماری و وابستگی این فایل را توصیف می‌کنند و شاید برنامه را توصیف نکنند.
core-warning-build-other-apps-files-unknown = همه اطلاعاتی که Windows Installer درباره برنامه‌های دیگر دارد خوانده نشد، بنابراین شاید پوشه‌ای که در سابقه این برنامه آمده است فایل‌های برنامه دیگری را داشته باشد، بی‌آنکه چیزی در اینجا آن را نشان دهد.
core-warning-build-app-folder-has-another-apps-file = Windows Installer می‌گوید برنامه دیگری فایلی در پوشه‌ای دارد که فایل اجرایی این برنامه در آن است، و آن فایل نیز همراه می‌آید، زیرا کل آن پوشه برداشته می‌شود.
# "App Paths" is the name of a Windows registry key; keep it as it is.
core-warning-build-app-paths-folder-left-out = ورودی App Paths این برنامه می‌گوید در پوشه‌ای جستجو می‌کند که متعلق به خودش نیست، بنابراین آن پوشه همراه نمی‌آید. شاید برنامه در رایانه‌ای که محتوای آن پوشه را ندارد شروع نشود.
core-warning-build-uninstaller-folder-left-out = پوشه‌ای که برنامه حذف نصب این برنامه در آن است کنار گذاشته شد، زیرا Windows Installer می‌گوید برنامه دیگری فایلی در آنجا دارد.
# { $why } is never shown: it chooses the reason. Translate each reason as the end of the sentence.
core-warning-build-recorded-folder-left-out =
    چیزی در سابقه این برنامه پوشه‌ای را نام می‌برد که کنار گذاشته شد، زیرا { $why ->
        [unreadable] مسیر آن از درایو بیرون می‌رود یا از یک پیوند عبور می‌کند، بنابراین خواندن آن ایمن نیست
        [missing] وجود ندارد
        [link] از یک پیوند عبور می‌کند
        [short-name] به همان شکلی که واقعاً نام دارد نوشته نشده است
        [another-account] در پوشه‌های حساب دیگری است
        [one-account] این برنامه برای یک حساب نصب شده است و این پوشه متعلق به آن حساب نیست
        [shared] چیزهای مشترک را در خود دارد یا بخشی از Windows است
        [generic-name] نام آن چیزی درباره صاحبش نمی‌گوید
        [publisher] فقط به نام ناشر است و شاید محصولات دیگر او را هم در خود داشته باشد
        [holds-the-app] پوشه خود این برنامه را در خود دارد و شاید چیزهای بیشتری هم داشته باشد
        [another-apps-name] به نام برنامه دیگری است
        [another-apps-record] سابقه نصب آن متعلق به برنامه دیگری است
        [in-another-apps-folder] در پوشه برنامه دیگری است
       *[another-apps] متعلق به برنامه دیگری است
    }.
core-warning-build-app-folder-holds-another-app = طبق سوابق، برنامه دیگری در این رایانه داخل پوشه این برنامه است و همراه آن می‌آید، زیرا کل آن پوشه برداشته می‌شود.
# "Program Files (x86)" is the name of a Windows folder; keep it as Windows calls it.
core-warning-build-from-32-bit-windows = این برنامه روی Windows 32 بیتی بود. کلیدهای رجیستری آن به همان شکلی چیده شده‌اند که Windows 64 بیتی کلیدهای یک برنامه 32 بیتی را نگه می‌دارد، و نصب‌کننده Program Files (x86) را برای آن پیشنهاد می‌کند.
# { $dll } is a file's name, such as MSVCP140D.dll.
core-warning-build-debug-runtime = یک فایل اجرایی به { $dll } نیاز دارد، که نسخه اشکال‌زدایی زمان اجرای Visual C++ است. Microsoft نصب‌کننده‌ای برای این نسخه‌ها ندارد و شاید اجازه توزیع آن‌ها را ندهد، بنابراین در رایانه‌ای بدون Visual Studio اجرا نمی‌شود.
# { $program } is a program's file name, such as Tool.exe.
core-warning-build-single-file-program = { $program } یک برنامه .NET است که در یک فایل بسته‌بندی شده و چیزی درباره اینکه به کدام .NET نیاز دارد نمی‌گوید، بنابراین هیچ‌کدام برای آن در نظر گرفته نشده است. اگر به یکی نیاز داشته باشد، در رایانه‌ای بدون آن شروع نمی‌شود.
# { $count } is how many of the app's programs.
core-warning-build-programs-not-inspected =
    { $count ->
        [one] { $count } مورد از فایل‌های اجرایی برنامه از نظر زمان‌های اجرای مورد نیاز بررسی نشد، زیرا برنامه فایل‌های اجرایی بسیار زیاد یا بسیار بزرگی دارد.
       *[other] { $count } مورد از فایل‌های اجرایی برنامه از نظر زمان‌های اجرای مورد نیاز بررسی نشدند، زیرا برنامه فایل‌های اجرایی بسیار زیاد یا بسیار بزرگی دارد.
    }
# { $runtime } is a runtime's name, such as Java Runtime Environment.
core-warning-build-runtime-no-installer = برنامه به { $runtime } نیاز دارد و نصب‌کننده‌ای برای آن در اینجا نیست، بنابراین هیچ نصب‌کننده‌ای اجرا نمی‌شود. خودتان آن را در رایانه دیگر نصب کنید.
# { $why } is why something was left out, such as "its name says what it holds".
core-warning-build-left-out-of-package = از بسته کنار گذاشته شد: { $why }.
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-missing-dependency = وابستگی پیدا نشد: { $name }
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-dependency-only-here = { $name } فقط از طریق مسیر جستجوی این رایانه پیدا شد و بخشی از برنامه نیست، بنابراین همراه آن نمی‌آید. شاید برنامه در رایانه‌ای بدون آن شروع نشود.
core-warning-build-driver-detected = درایور یا مؤلفه هسته شناسایی شد. درایوری که متعلق به برنامه است همراه بسته می‌شود و دوباره ثبت می‌شود؛ درایور سازنده یا درایور داخلی Windows توزیع نمی‌شود و باید از بسته امضاشده سازنده‌اش دوباره نصب شود.
core-warning-build-near-no-longer-offered = چیزی که در فهرست موارد احتمالی این برنامه علامت زده شده بود دیگر در آن فهرست نیست، بنابراین اضافه نشد. فهرست را دوباره ببینید.
# { $app_name } is the name of a part of the app, as the list of apps calls it. { $why } is never shown: it chooses the
# reason. Translate each reason as the end of the sentence.
core-warning-build-near-part-left-out =
    بخشی از آنچه { $app_name } متعلق به خود می‌داند کنار گذاشته شد: { $why ->
        [unreadable] مسیر آن از درایو بیرون می‌رود یا از یک پیوند عبور می‌کند، بنابراین خواندن آن ایمن نیست
        [missing] وجود ندارد
        [link] از یک پیوند عبور می‌کند
        [short-name] به همان شکلی که واقعاً نام دارد نوشته نشده است
        [another-account] در پوشه‌های حساب دیگری است
        [one-account] این برنامه برای یک حساب نصب شده است و این مورد متعلق به آن حساب نیست
        [shared] چیزهای مشترک را در خود دارد یا بخشی از Windows است
        [generic-name] نام آن چیزی درباره صاحبش نمی‌گوید
        [publisher] فقط به نام ناشر است و شاید محصولات دیگر او را هم در خود داشته باشد
        [holds-the-app] پوشه خود این برنامه را در خود دارد و شاید چیزهای بیشتری هم داشته باشد
        [another-apps-name] به نام برنامه دیگری است
        [another-apps-record] سابقه نصب آن متعلق به برنامه دیگری است
        [in-another-apps-folder] در پوشه برنامه دیگری است
       *[another-apps] متعلق به برنامه دیگری است
    }.
# { $app_name } is the name of a part of the app, as the list of apps calls it.
core-warning-build-near-keys-left-out = کلیدهای رجیستری که سابقه نصب { $app_name } فهرست می‌کند کنار گذاشته شدند، زیرا آن سابقه چیزهایی متعلق به برنامه دیگری را نیز نام می‌برد.
# "Windows keeps what an app wrote for one account" is Windows' VirtualStore folder.
core-warning-build-virtual-store-program = یک فایل اجرایی در پوشه‌ای بود که Windows آنچه را یک برنامه برای یک حساب نوشته است در آن نگه می‌دارد. Windows هرگز فایل اجرایی در آنجا نمی‌گذارد، پس چیز دیگری آن را گذاشته است، و کنار گذاشته شد.
# { $path } is a file or folder, such as C:\Users\Sam\AppData\Roaming\Acme.
core-warning-build-file-left-out = { $path } کنار گذاشته شد: از یک پیوند روی درایو متصل عبور می‌کند، یا متعلق به حسابی غیر از حسابی است که داده‌هایش گنجانده می‌شود.
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as D:\Sam\Documents.
core-warning-build-user-folder-not-on-drive = پوشه { $folder } کاربر { $user } در { $path } بود که روی این درایو نیست، بنابراین محتوای آن گنجانده نشد.
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as \\server\home\Sam\Documents.
core-warning-build-user-folder-not-on-this-pc = پوشه { $folder } کاربر { $user } در { $path } بود که روی دیسکی از این رایانه نیست، بنابراین محتوای آن گنجانده نشد.
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-base-unreadable = { $path } خوانده نشد ({ $error })؛ داده‌های برنامه که در آنجا ذخیره شده بود گنجانده نشد.
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-folder-unreadable = { $path } خوانده نشد ({ $error })؛ شاید بخشی از داده‌های برنامه در زیرپوشه‌های آن جا مانده باشد.
# { $error } is what went wrong.
core-warning-build-entry-unreadable = یک ورودی سیستم فایل خوانده نشد: { $error }
# { $count } is how many links. { $names } lists some of them. { $more } is how many more there are, which are not
# named; when it is 0, the sentence ends after the list.
core-warning-build-links-not-followed =
    { $count ->
        [one] { $count } پیوند، یا نامی که چیز دیگری را باز می‌کند، در این پوشه دنبال نشد، بنابراین آنچه به آن اشاره می‌کند در نصب‌کننده نیست: { $names }
       *[other] { $count } پیوند، یا نامی که چیز دیگری را باز می‌کند، در این پوشه دنبال نشدند، بنابراین آنچه به آن‌ها اشاره می‌کنند در نصب‌کننده نیست: { $names }
    }{ $more ->
        [0] .
       *[other] ، و { $more } مورد دیگر.
    }
# { $count } is how many folders. { $depth } is how many levels down a folder can be. { $names } lists some of them.
# { $more } is how many more there are, which are not named; when it is 0, the sentence ends after the list.
core-warning-build-folders-too-deep =
    { $count ->
        [one] { $count } پوشه بیش از { $depth } سطح پایین‌تر است، بنابراین محتوای آن در نصب‌کننده نیست: { $names }
       *[other] { $count } پوشه بیش از { $depth } سطح پایین‌تر هستند، بنابراین محتوای آن‌ها در نصب‌کننده نیست: { $names }
    }{ $more ->
        [0] .
       *[other] ، و { $more } مورد دیگر.
    }
# { $error } is what went wrong.
core-warning-build-metadata-unreadable = فراداده فایل خوانده نشد: { $error }
core-warning-build-reserved-name = نام این فایل برای یک دستگاه Windows رزرو شده است، بنابراین کپی نشد.
# { $error } is what went wrong.
core-warning-build-hash-failed = هش فایل محاسبه نشد: { $error }
core-warning-build-cloud-placeholder = فایل ابری فقط‌آنلاین (مثلاً OneDrive Files On-Demand) داده محلی ندارد؛ پیش از پشتیبان‌گیری آن را سنجاق کنید یا بارگیری کنید. گزارش شد اما کپی نشد.
# { $file_name } is a driver's file name, such as acmeflt.sys.
core-warning-build-driver-file = فایل درایور/هسته «{ $file_name }» به‌عنوان یک فایل معمولی کپی نمی‌شود. اگر پشتوانه یک سرویس درایور ثبت‌شده برای این برنامه باشد، نصب‌کننده آن را همراه بسته می‌کند و درایور را به‌طور خودکار دوباره ثبت می‌کند؛ در غیر این صورت، آن را با بسته درایور امضاشده سازنده دوباره نصب کنید.
core-warning-build-file-gone = این مورد هنگام بررسی برنامه وجود داشت و اکنون از بین رفته است، بنابراین در نصب‌کننده نیست.
# { $error } is what went wrong.
core-warning-build-file-copy-failed = فایل در بسته کپی نشد: { $error }
# { $count } is how many files. { $names } lists them.
core-warning-build-not-looked-through =
    { $count ->
        [one] این بسته فایلی دارد که نمی‌شد آن را برای یافتن نشانه‌ای از اینکه از چه کسی آمده است بررسی کرد: { $names }. شاید محتوای آن چنین نشانه‌ای داشته باشد.
       *[other] این بسته فایل‌هایی دارد که نمی‌شد آن‌ها را برای یافتن نشانه‌ای از اینکه از چه کسی آمده‌اند بررسی کرد: { $names }. شاید محتوای آن‌ها چنین نشانه‌ای داشته باشد.
    }
# { $count } is the most empty folders an installer makes, such as 10,000.
core-warning-build-empty-folders-truncated = این برنامه بیش از { $count } پوشه خالی دارد و فقط همین تعداد دوباره ساخته می‌شود.
core-warning-build-credential-by-now = از بسته کنار گذاشته شد: اکنون شبیه اعتبارنامه است.
core-warning-build-no-package-path = فایل قابل کپی مسیر نسبی در بسته نداشت.
# { $file_name } is the runtime installer's file name, such as vc_redist.x64.exe. { $error } is what went wrong.
core-warning-build-prerequisite-path-invalid = پیش‌نیاز همراه { $file_name } قرار داده نشد: { $error }
# { $runtime } is a runtime's name, such as Microsoft Visual C++ 2015+ Redistributable (x64). { $error } is what went
# wrong. winget is the name of a Windows command; keep it as it is.
core-warning-build-prerequisite-download-failed = { $runtime } بارگیری نشد ({ $error })؛ نصب‌کننده در رایانه مقصد winget را امتحان می‌کند.
# { $runtime } is a runtime's name. { $reason } is why the check failed, such as "it is signed by Contoso Ltd, not
# Microsoft".
core-warning-build-prerequisite-signature-failed = فایل بارگیری‌شده { $runtime } در بررسی امضا رد شد، بنابراین همراه بسته نشد: { $reason }. نصب‌کننده در رایانه مقصد winget را امتحان می‌کند.
# { $runtime } is a runtime's name. { $error } is what went wrong.
core-warning-build-prerequisite-hash-failed = { $runtime } بارگیری شد اما هش آن محاسبه نشد ({ $error })؛ نصب‌کننده به‌جای آن در رایانه مقصد winget را امتحان می‌کند.
# { $service } is a driver's service name, such as AcmeFlt. { $error } is what went wrong.
core-warning-build-driver-copy-failed = تصویر درایور برای «{ $service }» همراه بسته نشد: { $error }
core-warning-build-license-not-copied = داده‌های مجوز کپی نشد (وجود ندارد یا قابل خواندن نیست)؛ فقط به‌صورت فراداده ثبت شد.
core-warning-build-license-file-credential = یک فایل مجوز شبیه اعتبارنامه است و کنار گذاشته شد.
# { $error } is what went wrong.
core-warning-build-license-store-walk-failed = بخشی از یک انبار مجوز شمارش نشد: { $error }
core-warning-build-license-store-credential = فایلی در یک انبار مجوز شبیه اعتبارنامه است و کنار گذاشته شد.
# { $error } is what went wrong.
core-warning-build-license-store-file-skipped = از یک فایل انبار مجوز صرف‌نظر شد: { $error }
