# Words of WinAppMigrator Client, the console program run on the old PC, the one an app is moving off, so that
# WinAppMigrator on the new PC can read it over the network. Message names start with client-.
#
# Its window is a console, which on Windows 7 is 80 columns wide. Each line break in a message is one on the screen:
# keep every line shorter than 80 characters, and break lines where your language needs to, not only where English
# does.


## Starting

# Shown only in English, in place of a right-to-left language the console cannot show. { $language } is that
# language's name in English, such as Arabic.
client-console-rtl-fallback = این کنسول نمی‌تواند { $language } را نشان دهد، بنابراین کلاینت به زبان انگلیسی است.
# The client copies itself to a folder in Program Files before it starts. { $error } is what Windows said went wrong.
client-safe-folder-failed =
    WinAppMigrator Client نتوانست از پوشه‌ای اجرا شود که فقط سرپرستان
    می‌توانند آن را تغییر دهند: { $error }
client-press-enter-to-close = برای بستن این پنجره، Enter را فشار دهید.
# The first line of its screen. { $version } is the client's version, such as 0.6.2.
client-heading = WinAppMigrator Client { $version }
client-intro =
    این برنامه به WinAppMigrator روی رایانه جدید شما اجازه می‌دهد
    برنامه‌های این رایانه را از طریق شبکه بخواند. اینجا چیزی را تغییر
    نمی‌دهد: از یک تصویر لحظه‌ای درایو Windows می‌خواند و وقتی آن را
    متوقف کنید، تصویر را حذف می‌کند.
# { $error } is why: one of the messages under "Why it stopped" below, or what Windows said.
client-stopped = کلاینت متوقف شد: { $error }
# The caretaker is a copy of the client with no window that cleans up after it. { $error } is what Windows said.
client-caretaker-failed =
    اگر این پنجره به‌جای متوقف کردن بسته شود، تصویر لحظه‌ای و قوانین
    دیوار آتش تا اجرای بعدی کلاینت در اینجا باقی می‌مانند ({ $error }).
# Shown while the snapshot is taken, followed on the same line by client-snapshot-taken. Keep the quotes: they keep
# the space before it.
client-snapshot-taking = در حال گرفتن تصویر لحظه‌ای از درایو Windows...{ " " }
client-snapshot-taken = انجام شد.
# The old PC answers the new one's search for it on the network. { $error } is what Windows said went wrong.
client-discovery-failed =
    این رایانه خودبه‌خود در شبکه پیدا نمی‌شود ({ $error }).
    در رایانه جدید، به‌جای آن نام یا نشانی این رایانه را تایپ کنید.

## This PC and its pairing code. The three lines are lined up in two columns, behind two spaces; keep the quotes, which
## keep those spaces.

# { $computer_name } is this PC's name, such as OLD-PC.
client-this-pc = { "  " }این رایانه:    { $computer_name }
# { $windows_name } is this PC's Windows, such as Windows 7 Professional Service Pack 1. { $bits } is 32 or 64.
client-windows = { "  " }Windows:       { $windows_name }، { $bits } بیتی
# { $code } is six digits in two groups, such as 482 913.
client-pairing-code = { "  " }کد جفت‌سازی:    { $code }
# Network PC... is the button of that name in WinAppMigrator: use the name it has there in your language.
client-pairing-steps =
    در رایانه جدید، در WinAppMigrator گزینه «رایانه شبکه...» را انتخاب
    کنید و این کد را تایپ کنید. ابتدا برنامه‌ای را که منتقل می‌کنید
    ببندید تا فایل‌هایش در همان حالتی بمانند که آن را رها کرده است.
    برای توقف، اینجا Enter را فشار دهید یا این پنجره را ببندید.

## While it runs. { $from } is the network address of the PC that connected, such as 192.168.1.20, or "unknown" when
## it could not be told, which the [unknown] variant words.

client-paired =
    با { $from ->
        [unknown] رایانه‌ای دیگر
       *[address] { $from }
    } جفت شد. در حال خواندن آنچه نیاز دارد...
# { $count } is how many requests it answered.
client-builder-done =
    { $from ->
        [unknown] رایانه‌ای دیگر
       *[address] { $from }
    } کار خود را تمام کرد (به { $count ->
        [one] { $count } درخواست
       *[other] { $count } درخواست
    } پاسخ داده شد).
# { $error } is what went wrong, often in Windows' words.
client-link-ended =
    اتصال با { $from ->
        [unknown] رایانه‌ای دیگر
       *[address] { $from }
    } قطع شد: { $error }
client-wrong-code =
    { $from ->
        [unknown] رایانه‌ای دیگر
       *[address] { $from }
    } کدی را امتحان کرد که کد این رایانه نیست.
# { $error } is what went wrong, often in Windows' words.
client-pair-failed =
    { $from ->
        [unknown] رایانه‌ای دیگر
       *[address] { $from }
    } نتوانست جفت شود: { $error }
client-too-many-wrong-codes =
    کدهای اشتباه زیادی امتحان شد، بنابراین اکنون هیچ رایانه‌ای نمی‌تواند
    با این رایانه جفت شود. شاید چیزی در شبکه شما در حال حدس زدن کد باشد.
    برای جفت شدن، کلاینت را متوقف کنید و دوباره اجرا کنید، و کد جدیدی را
    که نشان می‌دهد تایپ کنید.
# { $code } is six digits in two groups, such as 482 913.
client-new-code = تعداد کدهای اشتباه بیش از حد است. کد جفت‌سازی جدید: { $code }

## Stopping

client-stopping = در حال توقف: حذف تصویر لحظه‌ای و قوانین دیوار آتش...
client-stop-done = انجام شد.

## Why it stopped, shown after "The client stopped: " (client-stopped), and written to follow it.

client-already-running = کلاینت از قبل روی این رایانه در حال اجراست، یا در حال پاک‌سازی پس از کلاینتی است که بسته شد. از همان استفاده کنید، یا لحظه‌ای صبر کنید و دوباره این را اجرا کنید.
# { $drive } is the drive Windows is on, such as C:. { $details } is what Windows said, in its own words.
client-snapshot-failed = Windows از { $drive } تصویر لحظه‌ای تهیه نکرد: { $details }
# { $details } is what Windows Firewall said, in its own words.
client-firewall-failed = دیوار آتش Windows اجازه نداد کلاینت منتظر اتصال بماند: { $details }

## What it takes, shown for --help. The options, such as --lang, are typed as they are and are not translated; keep
## their descriptions lined up.

client-usage =
    WinAppMigrator Client این گزینه‌ها را می‌پذیرد:
    
      --lang <زبان>      به‌جای زبان نمایش Windows، به این زبان، مانند it-IT،
                         نمایش دهد، اگر کلاینت آن را داشته باشد.
      --code <کد>        با این کد جفت‌سازی 6 رقمی شروع شود، برای اسکریپتی
                         که WinAppMigrator را روی رایانه جدید هم اجرا می‌کند.
      --help             نمایش همین راهنما.
