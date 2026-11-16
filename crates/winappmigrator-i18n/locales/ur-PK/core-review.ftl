# Words the core says about what an install may do: why an install location or something in a package is refused,
# the installer's list of what an app sets up beyond its files, the checks of a package and of an install, and whether
# there is room for either. Message names start with core-.
#
# Many messages here are a reason, the second half of a sentence that starts "... was left out:" or "that install
# location cannot be used:", so they start with a small letter and have no full stop.


## Why an install location cannot be used. Shown in red under the Install Location field.

core-review-location-state-root = یہاں تنصیبات اپنے ریکارڈ رکھتی ہیں
core-review-location-has-files = اس میں پہلے سے فائلیں ہیں، اور یہاں کوئی تنصیب ریکارڈ میں نہیں ہے، اس لیے خالی فولڈر استعمال کریں
core-policy-location-not-full-path = یہ مکمل پاتھ ہونا چاہیے
core-policy-location-parent-step = اس میں ".." نہیں ہونا چاہیے
core-policy-location-not-plain = یہ اس PC کی کسی ڈرائیو پر ایک سادہ پاتھ ہونا چاہیے
core-policy-location-whole-drive = یہ پوری ڈرائیو نہیں ہو سکتا
core-policy-location-windows = یہ Windows فولڈر نہیں ہو سکتا
core-policy-location-runs-by-itself = یہ ایسا فولڈر ہے جو Windows کا ہے، یا جس کے مندرجات خود بخود چلتے یا لوڈ ہوتے ہیں
core-policy-location-shared = اس میں دوسری چیزیں ساتھ ساتھ موجود ہیں، اس لیے ایپ کو اس کے اندر ایک فولڈر درکار ہے

## Whether an app can run on this PC at all, said before anything is installed.

# { $reason } is why, as the second half of a sentence, such as "App.exe is built for Arm64, and this PC is x64, so
# Windows would not start it".
core-review-cannot-run-here = یہ ایپ اس PC پر نہیں چل سکتی، اس لیے کچھ تبدیل نہیں کیا گیا: { $reason }
core-policy-architecture-x86 = 32 بٹ x86
core-policy-architecture-x64 = x64
core-policy-architecture-arm64 = Arm64
core-policy-architecture-arm32 = 32 بٹ Arm
core-policy-architecture-unknown = ایک نامعلوم آرکیٹیکچر
# { $code } is the number Windows gives a kind of processor, such as 0x01c4.
core-policy-machine-type = مشین کی قسم { $code }
# { $names } is a list of program names, such as "a.exe, b.exe, c.exe", and { $count } how many more there are.
core-policy-names-and-more = { $names }، اور مزید { $count }
core-policy-fit-programs-not-said = یہ انسٹالر نہیں بتاتا کہ اس کے پروگراموں کو کیا درکار ہے، اس لیے یہ جانچ نہیں کی گئی کہ آیا وہ اس PC پر چل سکتے ہیں۔
core-policy-fit-architecture-not-told = یہ معلوم نہیں ہو سکا کہ اس PC کا آرکیٹیکچر کیا ہے، اس لیے یہ جانچ نہیں کی گئی کہ آیا اس ایپ کے پروگرام اس پر چل سکتے ہیں۔
core-policy-fit-main-not-said = یہ انسٹالر نہیں بتاتا کہ اس کے مرکزی پروگرام کو کیا درکار ہے، اس لیے یہاں جو کہا گیا ہے وہ اس میں موجود باقی چیزوں کی بنیاد پر ہے۔
# { $driver } is a driver's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-driver = اس کا ڈرائیور { $driver } { $built_for } کے لیے بنا ہے، اور یہ PC { $here } ہے۔ ڈرائیور کا Windows سے بالکل مماثل ہونا ضروری ہے، اس لیے یہ ایپ یہاں نہیں چل سکتی
# { $program } is a program's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-program = { $program } { $built_for } کے لیے بنا ہے، اور یہ PC { $here } ہے، اس لیے Windows اسے شروع نہیں کرے گا
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-none-start = اس کا کوئی بھی پروگرام اس PC پر شروع نہیں ہو سکتا، جو { $here } ہے: { $programs }
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-some-other-architecture = اس ایپ کے کچھ پروگرام کسی دوسرے آرکیٹیکچر کے لیے بنے ہیں اور اس PC پر شروع نہیں ہوں گے، جو { $here } ہے: { $programs }۔ باقی ایپ چل سکتی ہے، اس لیے شاید اس سے فرق نہ پڑے۔
# { $here } is a kind of processor, such as "Arm64". { $programs } is a list of program names.
core-policy-fit-emulated = یہ PC { $here } ہے، اس لیے Windows انہیں ایمولیٹر کے ذریعے چلائے گا: { $programs }۔ زیادہ تر پروگرام اس طرح ٹھیک چلتے ہیں، اور کچھ نہیں چلتے۔
# { $program } is a program's file name. { $needs } and { $runs } are versions of Windows, such as 10.0 and 6.1.
core-policy-fit-needs-newer = { $program } کہتا ہے کہ اسے Windows { $needs } یا اس سے نیا درکار ہے، اور یہ PC { $runs } چلاتا ہے، اس لیے Windows اسے شروع نہیں کرے گا
# { $programs } is a list of program names.
core-policy-fit-some-need-newer = اس ایپ کے کچھ پروگرام کہتے ہیں کہ انہیں اس سے نیا Windows درکار ہے اور وہ یہاں شروع نہیں ہوں گے: { $programs }۔
# { $from_build } and { $this_build } are Windows build numbers, such as 22631, written as Windows writes them.
core-policy-fit-older-build = یہ ایپ Windows بلڈ { $from_build } پر تھی، اور یہ PC بلڈ { $this_build } چلاتا ہے۔ اس کے پروگرام کہتے ہیں کہ وہ اس Windows پر شروع ہو سکتے ہیں، اس لیے شاید یہ کام کرے، لیکن ہو سکتا ہے جن چیزوں پر یہ وہاں انحصار کرتی تھی وہ یہاں نہ ہوں۔
core-policy-fit-build-not-told = یہ معلوم نہیں ہو سکا کہ یہ Windows کا کون سا بلڈ ہے، اس لیے اس کا موازنہ اس بلڈ سے نہیں کیا گیا جس پر یہ ایپ تھی۔

## Room on a drive for a build or an install.

# Each of these ends the sentence core-space-shortfall, after "free for".
core-space-for-unpacking = پیکیج نکالنے
core-space-for-install-record = تنصیب کے ریکارڈ
core-space-for-app-files = ایپ کی فائلوں
core-space-for-what-is-replaced = تبدیل ہونے والی چیزیں محفوظ رکھنے
core-space-for-build-work = بلڈ کے دوران استعمال ہونے والی چیزوں
core-space-for-installer = انسٹالر
core-space-for-package = پیکیج
core-space-for-unpacking-and-installing = پیکیج نکالنے اور اسے انسٹال کرنے
# What goes between two of those, as in "unpacking the package and the app's files". Keep the quotes: they keep the
# spaces.
core-space-purposes-joiner = { " اور " }
# { $drive } is a drive, such as C:, or a folder. { $needed } and { $free } are sizes, such as 1.2 GB. { $purposes } is
# what the room is for, such as "unpacking the package and the app's files".
core-space-shortfall = { $drive } پر { $purposes } کے لیے زیادہ سے زیادہ { $needed } خالی جگہ درکار ہے، اور اس پر { $free } ہے
# { $drive } is a drive, such as C:, or a folder. { $needed } is a size, such as 1.2 GB.
core-space-not-told = { $drive } نے یہ نہیں بتایا کہ اس پر کتنی جگہ ہے، اس لیے یہ جانچ نہیں کی گئی کہ آیا درکار { $needed } کی گنجائش ہے
# One drive in a list of where room is needed. { $needed } and { $free } are sizes, and { $drive } a drive, such as C:.
core-space-summary = { $drive } پر زیادہ سے زیادہ { $needed } ({ $free } خالی)
# { $needed } is a size, and { $drive } a drive, such as C:.
core-space-summary-not-told = { $drive } پر زیادہ سے زیادہ { $needed } (کتنی جگہ خالی ہے یہ معلوم نہیں ہو سکا)
# What goes between two sentences of core-space-shortfall inside core-review-no-room. Keep the quotes: they keep the
# space.
core-review-sentence-joiner = { "۔ " }
# { $shortfalls } is one or more sentences of core-space-shortfall.
core-review-no-room = اس کے لیے کافی جگہ نہیں ہے، اس لیے کچھ تبدیل نہیں کیا گیا۔ { $shortfalls }۔ وہاں کچھ جگہ خالی کریں، یا کسی دوسری ڈرائیو پر تنصیب کا مقام منتخب کریں۔

## The installer's list of what an app sets up beyond its files: what each kind is called.

core-review-kind-service = سروس
core-review-kind-driver = ڈرائیور
core-review-kind-scheduled-task = شیڈول کردہ ٹاسک
core-review-kind-autostart = خود بخود شروع ہوتا ہے
core-review-kind-shortcut = شارٹ کٹ
core-review-kind-path = PATH
core-review-kind-environment-variable = ماحولیاتی متغیر
core-review-kind-shell-extension = شیل ایکسٹینشن
core-review-kind-menu-command = مینیو کمانڈ
core-review-kind-link-type = لنک کی قسم
core-review-kind-file-type = فائل کی قسم
core-review-kind-com = COM
core-review-kind-firewall-rule = فائر وال قاعدہ
core-review-kind-event-log-source = ایونٹ لاگ کا ماخذ
core-review-kind-font = فونٹ
core-review-kind-odbc-driver = ODBC ڈرائیور
core-review-kind-odbc-source = ODBC ڈیٹا ماخذ
core-review-kind-compatibility-setting = مطابقت کی ترتیب
core-review-kind-default-apps = ڈیفالٹ ایپس
core-review-kind-program-name = پروگرام کا نام
core-review-kind-other-settings = دیگر ترتیبات

## The installer's list of what an app sets up: what each line says of it.

# { $file } is a program's file name. { $account } is the Windows account it runs as, such as LocalSystem. { $starts }
# says when it starts, and is not shown itself; for "other", { $start } is one of the core-review-start- words below.
core-review-service-detail =
    { $starts ->
        [automatic] { $file } کو { $account } کے طور پر چلاتی ہے، ابھی اور ہر بار یہ PC شروع ہونے پر شروع ہوتی ہے
        [delayed] { $file } کو { $account } کے طور پر چلاتی ہے، ابھی اور ہر بار یہ PC شروع ہونے کے کچھ دیر بعد شروع ہوتی ہے
        [manual] { $file } کو { $account } کے طور پر چلاتی ہے، صرف اس وقت شروع ہوتی ہے جب کوئی چیز اسے کہے
        [disabled] { $file } کو { $account } کے طور پر چلاتی ہے، غیر فعال ہے
        [other] { $file } کو { $account } کے طور پر چلاتی ہے، شروع ہوتی ہے: { $start }
       *[unknown] { $file } کو { $account } کے طور پر چلاتی ہے
    }
# { $account } is a Windows account, such as LocalService.
core-policy-account-custom = { $account } (حسب ضرورت اکاؤنٹ بحال نہیں کیا گیا)
# { $file } is a driver's file name.
core-review-driver-detail = { $file } کو خود Windows میں لوڈ کرتا ہے
# { $file } is a driver's file name. { $start } is when Windows starts it: one of the core-review-start- words below.
core-review-driver-detail-starts = { $file } کو خود Windows میں لوڈ کرتا ہے، شروع ہوتا ہے: { $start }
# When a service or driver starts, ending a sentence such as "Loads disk.sys into Windows itself, starts …".
core-review-start-boot = Windows کے ساتھ، باقی ہر چیز سے پہلے
core-review-start-system = Windows کے ساتھ، اس کے شروع ہوتے وقت
core-review-start-automatic = ہر بار یہ PC شروع ہونے پر
core-review-start-delayed = ہر بار یہ PC شروع ہونے کے کچھ دیر بعد
core-review-start-manual = صرف اس وقت جب کوئی چیز اسے کہے
core-review-start-disabled = کبھی نہیں، کیونکہ یہ غیر فعال ہے
core-review-start-unknown = ایسے طریقے سے جو Windows نہیں بتاتا
# { $account } is the account a task runs as: an account's name, or core-review-task-runs-as-you, or
# core-review-task-runs-as-creator.
core-review-task-detail = اس اکاؤنٹ سے چلتا ہے: { $account }
# { $account } is the account a task runs as, as for core-review-task-detail.
core-review-task-detail-highest = اس اکاؤنٹ سے، زیادہ سے زیادہ ممکنہ حقوق کے ساتھ چلتا ہے: { $account }
core-review-task-runs-as-you = آپ، جو بھی سائن ان ہو
core-review-task-runs-as-creator = وہ صارف جس نے اسے بنایا
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-autostart-detail =
    { $scope ->
        [everyone] سائن ان کرنے والے ہر فرد کے لیے چلتا ہے
       *[you] جب آپ سائن ان کریں تب چلتا ہے
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shortcut-startup-detail =
    اسٹارٹ اپ فولڈر میں ایک شارٹ کٹ، { $scope ->
        [everyone] اس PC پر سب کے لیے
       *[you] آپ کے لیے
    }
# { $file } is the file name of what the shortcut opens. { $scope } is whom it is set up for: everyone on this PC,
# or you. It is not shown itself.
core-review-shortcut-detail =
    { $file } کھولتا ہے، { $scope ->
        [everyone] اس PC پر سب کے لیے
       *[you] آپ کے لیے
    }
# { $name } is an environment variable's name, such as PATH.
core-review-path-title = { $name } میں شامل کرتا ہے
core-review-path-install-folder = تنصیب کا فولڈر
# { $path } is a folder inside the install folder, such as bin\tools.
core-review-path-below-install-folder = تنصیب کا فولڈر\{ $path }
# { $value } is the variable's value, as it will be set.
core-review-variable-detail = { $value } پر سیٹ کیا جاتا ہے
# { $count } is how many registry values.
core-review-other-settings-title =
    { $count ->
        [one] ایپ کی اپنی کلیدوں سے باہر { $count } ترتیب
       *[other] ایپ کی اپنی کلیدوں سے باہر { $count } ترتیبات
    }
# { $keys } is a list of registry keys, such as HKCU\Software\Vendor.
core-review-other-settings-detail = { $keys } میں
# { $keys } is a list of registry keys, and { $count } how many more there are.
core-review-other-settings-detail-more = { $keys }، اور مزید { $count } میں
# { $way } says whether the rule allows or blocks, and incoming or outgoing connections; it is not shown itself.
# { $ports } is a list of port numbers, and { $port_count } how many. { $program } is the program's file name.
# { $networks } is a list of the kinds of network Windows names in English (domain, private, public), and
# { $network_count } how many.
# The kinds of network a firewall rule is for, as Windows Defender Firewall names its profiles in your language. They
# fill { $networks } in core-review-firewall-detail, joined into a list.
core-review-network-domain = ڈومین
core-review-network-private = نجی
core-review-network-public = عوامی
core-review-firewall-detail =
    { $program } کے لیے، { $network_count ->
        [0] ہر نیٹ ورک پر
       *[other] { $networks } نیٹ ورکس پر
    }، { $port_count ->
        [0] { "" }
        [one] پورٹ { $ports } پر{ " " }
       *[other] پورٹس { $ports } پر{ " " }
    }{ $way ->
        [allow-in] آنے والے کنکشنز کی اجازت دیتا ہے
        [allow-out] جانے والے کنکشنز کی اجازت دیتا ہے
        [block-in] آنے والے کنکشنز کو بلاک کرتا ہے
       *[block-out] جانے والے کنکشنز کو بلاک کرتا ہے
    }
# { $server } is the file name of what Explorer loads. { $scope } is whom it is set up for: everyone on this PC, or
# you. It is not shown itself.
core-review-shell-extension-detail =
    { $server } کو فائل ایکسپلورر میں لوڈ کرتا ہے، { $scope ->
        [everyone] اس PC پر سب کے لیے
       *[you] آپ کے لیے
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shell-extension-detail-own-file =
    اپنی انسٹال کردہ ایک فائل کو فائل ایکسپلورر میں لوڈ کرتا ہے، { $scope ->
        [everyone] اس PC پر سب کے لیے
       *[you] آپ کے لیے
    }
# The name of a shell extension: { $handler } is its name in the registry, and { $class } the kind of file or folder it
# is for, such as * or Directory.
core-policy-shell-extension-name = { $class } پر { $handler }
# { $on } is the kind of file or folder whose menu it is, as the registry names it, such as * or
# Directory. { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-menu-command-detail =
    { $on } کے مینیو میں ایک کمانڈ شامل کرتا ہے، { $scope ->
        [everyone] اس PC پر سب کے لیے
       *[you] آپ کے لیے
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-default-verb-detail =
    ان میں سے کسی کو کھولنے پر جو ہوتا ہے اسے تبدیل کرتا ہے، { $scope ->
        [everyone] اس PC پر سب کے لیے
       *[you] آپ کے لیے
    }
core-review-client-entry-detail = اس کام کے پروگرام کے طور پر منتخب کیا جا سکتا ہے۔ اس میں کچھ تبدیل نہیں کیا جاتا
core-review-com-permissions-title = COM اجازتیں
core-review-com-permissions-detail = بتاتا ہے کہ اس کی COM کلاسز کو کون شروع کر سکتا ہے، یا وہ کس کے طور پر چلتی ہیں
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-link-type-detail =
    اس قسم کے لنکس ایپ کے ساتھ کھلتے ہیں، { $scope ->
        [everyone] اس PC پر سب کے لیے
       *[you] آپ کے لیے
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-file-type-detail =
    اس قسم کی فائلیں ایپ کے ساتھ کھلتی ہیں، { $scope ->
        [everyone] اس PC پر سب کے لیے
       *[you] آپ کے لیے
    }
# { $log } is the name of a Windows event log, such as Application.
core-review-event-log-source-detail = { $log } لاگ میں، اپنی فائلوں کے پیغامات کے ساتھ لکھ سکتا ہے
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-font-detail =
    جب آپ سائن ان کریں تو تنصیب کے فولڈر سے لوڈ ہوتا ہے، { $scope ->
        [everyone] اس PC پر سب کے لیے
       *[you] آپ کے لیے
    }
core-review-odbc-driver-detail = ان پروگراموں میں لوڈ ہوتا ہے جو کسی ڈیٹا بیس تک پہنچنے کے لیے ODBC استعمال کرتے ہیں
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-odbc-source-detail =
    ایک ڈیٹا بیس کنکشن، { $scope ->
        [everyone] اس PC پر سب کے لیے
       *[you] آپ کے لیے
    }
core-review-default-apps-detail = ڈیفالٹ ایپس میں منتخب کیا جا سکتا ہے۔ اس میں کچھ تبدیل نہیں کیا جاتا
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-program-name-detail =
    "چلائیں" اور دوسرے پروگرام اسے اس نام سے تلاش کر سکتے ہیں، { $scope ->
        [everyone] اس PC پر سب کے لیے
       *[you] آپ کے لیے
    }
# { $server } is the file name of what serves the classes.
core-review-com-title = { $server } میں COM
core-review-com-title-own-file = اپنی انسٹال کردہ ایک فائل میں COM
# { $count } is how many registry values.
core-review-com-detail =
    { $count ->
        [one] ان کلاسز کی { $count } ترتیب جنہیں دوسرے پروگرام شروع کر سکتے ہیں
       *[other] ان کلاسز کی { $count } ترتیبات جنہیں دوسرے پروگرام شروع کر سکتے ہیں
    }
core-review-com-support-title = COM ٹائپ لائبریریاں، انٹرفیسز، اور AppIDs
# { $libraries }, { $interfaces }, and { $app_ids } are how many registry values there are for its type libraries, its
# interfaces, and its AppIDs.
core-review-com-support-detail = اس کی ٹائپ لائبریریوں ({ $libraries })، انٹرفیسز ({ $interfaces }) اور AppIDs ({ $app_ids }) کی ترتیبات، جو اس کی کلاسز کے ساتھ جاتی ہیں
core-review-compatibility-title = اس کے پروگرام کیسے شروع ہوتے ہیں
# { $count } is how many registry values.
core-review-compatibility-detail =
    { $count ->
        [one] اس کے انسٹال کردہ پروگراموں کے لیے { $count } مطابقت کی ترتیب
       *[other] اس کے انسٹال کردہ پروگراموں کے لیے { $count } مطابقت کی ترتیبات
    }

## The installer's notes on whose folders and settings these are, and what is not coming.

# { $account } is the name of a Windows account, such as alice.
core-review-account-signed-in = صارف کا ڈیٹا اور فی صارف ترتیبات { $account } کو جاتی ہیں، جو یہاں سائن ان ہے، نہ کہ اس منتظم کو جس کے طور پر یہ چل رہا ہے۔
# { $account } is the name of a Windows account, such as alice.
core-review-account-no-desktop = اس سیشن میں کوئی سائن ان نہیں ہے، اس لیے صارف کا ڈیٹا اور فی صارف ترتیبات { $account } کو جاتی ہیں، جس اکاؤنٹ کے طور پر یہ چل رہا ہے۔
# { $reason } is what Windows said, in its own words.
core-review-account-not-told = یہ معلوم نہیں ہو سکا کہ کون سائن ان ہے ({ $reason })، اس لیے فی صارف ترتیبات خارج کر دی گئی ہیں، اور صارف کا ڈیٹا تنصیب کے مقام کے تحت جاتا ہے۔
# { $reason } is what Windows said, in its own words.
core-review-signed-in-unknown = یہ معلوم نہیں ہو سکا کہ کون سائن ان ہے: { $reason }
# Headings of what is left out, shared by the app's list and the installer's notes.
core-review-left-out-credentials = اس انسٹالر میں شامل نہیں، کیونکہ یہ اسناد جیسے لگتے ہیں
core-review-left-out-settings = وہ ترتیبات اور سرگزشت جو کوئی اکاؤنٹ ایپ کے لیے رکھتا ہے، جو صرف صارف کے ڈیٹا کے اختیار کے ساتھ شامل ہوتی ہیں
core-review-left-out-certificates = لگتا ہے ایپ ایسے سرٹیفکیٹس پر انحصار کرتی ہے جن پر اس کے PC پر بھروسا کیا گیا تھا، اور وہ انسٹال نہیں کیے جاتے
core-review-left-out-fonts = یہ فونٹس ایپ کے نام سے ہیں، اور کہیں درج نہیں کہ یہ اس کے اپنے ہیں، اس لیے یہ انسٹال نہیں کیے جاتے
# { $names } is a list of names, such as the names of saved passwords.
core-review-note-credentials = اس انسٹالر میں شامل نہیں، کیونکہ یہ اسناد جیسے لگتے ہیں: { $names }۔ اس PC پر ان کی جو چیزیں موجود ہیں وہ جوں کی توں رہتی ہیں، اور ہو سکتا ہے آپ کو انہیں دوبارہ درج کرنا پڑے۔
# { $names } is a list of names of license data.
core-review-note-licenses = اس انسٹالر میں شامل نہیں، کیونکہ اسے بناتے وقت لائسنس اور فعال کاری کے ڈیٹا کا اختیار آف تھا: { $names }۔
# { $names } is a list of places an account keeps settings, such as folders.
core-review-note-settings = اس انسٹالر میں شامل نہیں، کیونکہ اسے بناتے وقت صارف کے ڈیٹا کا اختیار آف تھا: وہ ترتیبات اور سرگزشت جو کوئی اکاؤنٹ ایپ کے لیے { $names } میں رکھتا ہے۔ ایپ یہاں اسی طرح شروع ہوتی ہے جیسے کسی ایسے شخص کے لیے جو اسے پہلی بار استعمال کر رہا ہو۔
# { $names } is a list of certificates, each as core-review-certificate.
core-review-note-certificates = لگتا ہے ایپ ایسے سرٹیفکیٹس پر انحصار کرتی ہے جن پر اس کے PC پر بھروسا کیا گیا تھا، اور وہ انسٹال نہیں کیے جاتے: { $names }۔ یہاں ان پر بھروسا کرنا ہے یا نہیں، یہ فیصلہ آپ کا ہے۔
# { $names } is a list of font names.
core-review-note-fonts = یہ فونٹس ایپ کے نام سے ہیں، اور کہیں درج نہیں کہ یہ اس کے اپنے ہیں، اس لیے یہ انسٹال نہیں کیے جاتے: { $names }۔
# { $names } is a list of names.
core-review-note-left-out-by-choice = اس انسٹالر کو بنانے والے نے انہیں خارج کیا: { $names }۔
# { $names } is a list of names.
core-review-note-added-by-choice = اس انسٹالر کو بنانے والے نے انہیں شامل کیا، کیونکہ اس نے انہیں ایپ کا حصہ سمجھا: { $names }۔
# { $name } is a certificate's subject, such as CN=Contoso, and { $store } the certificate store it was in, such as
# Root.
core-review-certificate = { $store } میں { $name }
# { $names } is a list of names, and { $count } how many more there are.
core-review-names-and-more = { $names }، اور مزید { $count }

## Why something in a package is left out. Each is the second half of a sentence of core-warning-review-left-out.

core-review-no-account = یہ کسی ایک اکاؤنٹ کی ترتیبات کا حصہ ہے، اور یہ معلوم نہیں ہو سکا کہ کون سائن ان ہے
core-review-name-not-accepted = Windows اس کا نام قبول نہیں کرے گا
# { $error } is what Windows said, in its own words.
core-review-service-unreadable = اس کی موجودہ ترتیبات پڑھی نہیں جا سکیں: { $error }
core-review-service-dependency = یہ ایسی سروس پر منحصر ہے جس کا نام غلط ہے
# { $path } is the program the service runs.
core-review-service-runs-other = یہ { $path } چلاتی ہے، جو یہ پیکیج انسٹال نہیں کرتا
core-review-service-settings = اس کی ترتیبات ایسی چیز کا نام لیتی ہیں جو یہ پیکیج انسٹال نہیں کرتا، یا ایسی کلید کا جو اس کی اپنی نہیں ہے
core-review-driver-no-hash = اس کی جانچ کے لیے اس کا کوئی ہیش نہیں ہے
core-review-license-for-the-machine = پوری مشین کے لیے محفوظ کیے گئے لائسنس کی صرف اطلاع دی جاتی ہے
core-review-license-starts-something = لائسنس کے پاس کچھ بھی شروع کرنے کی کوئی وجہ نہیں ہوتی
core-review-task-not-captured = اس کی تعریف حاصل نہیں کی گئی
# { $error } is what was wrong with the task's XML, in English.
core-review-task-unreadable = اس کی تعریف پڑھی نہیں جا سکی: { $error }
core-review-task-no-account = یہ اسے انسٹال کرنے والے کے طور پر چلتا ہے، اور یہ معلوم نہیں ہو سکا کہ یہاں کون سائن ان ہے
# { $path } is what the shortcut opens.
core-review-shortcut-opens-other = یہ { $path } کھولتا ہے، جو یہ پیکیج انسٹال نہیں کرتا
# { $service } is the service's name, and { $path } the program it runs now.
core-policy-service-exists = { $service } اس PC پر پہلے سے موجود ہے اور { $path } چلاتی ہے، جو یہاں کی کسی پچھلی تنصیب سے نہیں ہے۔ اگر یہ اس ایپ کی کوئی پرانی کاپی ہے، تو پہلے اسے ان انسٹال کریں
core-policy-key-control-characters = اس کی کلید کے نام میں کنٹرول حروف ہیں
core-policy-value-control-characters = اس کی قدر کے نام میں کنٹرول حروف ہیں
core-policy-registry-link = یہ ایک رجسٹری لنک ہے
# { $reason } is why the key it stands for is left out, such as "no app install should change this key".
core-policy-virtual-store = یہ PC کی ایک کلید کی نمائندگی کرتا ہے، اور { $reason }
core-policy-registry-part = کسی ایپ کی تنصیب کو رجسٹری کے اس حصے میں نہیں لکھنا چاہیے
core-policy-key-refused = کسی ایپ کی تنصیب کو یہ کلید تبدیل نہیں کرنی چاہیے
core-policy-net-debugger = یہ ہر .NET پروگرام کے لیے ڈیبگر مقرر کر دے گا
core-policy-net-install-root = یہ تبدیل کر دے گا کہ ہر .NET پروگرام .NET کہاں سے لوڈ کرتا ہے
core-policy-class-redirects = یہ تبدیل کر دے گا کہ کسی قسم کا پروگرام یا فولڈر کیسے کھولا جاتا ہے
core-policy-app-path-name = یہ کسی پروگرام کا نام فائل کے نام کے علاوہ کسی اور چیز سے لیتا ہے
core-policy-app-path-not-installed = یہ ایسی فائل کے لیے پروگرام کا نام رجسٹر کرے گا جو یہ پیکیج انسٹال نہیں کرتا
core-policy-shared-default-verb = یہ تبدیل کر دے گا کہ کسی قسم کی ہر فائل یا فولڈر کو کھولنے پر کیا ہوتا ہے
core-policy-command-runs-other = یہ ایک ایسی کمانڈ شامل کرتا ہے جو ایسی چیز چلاتی ہے جو یہ پیکیج انسٹال نہیں کرتا
core-policy-autostart-not-command = یہ خود بخود شروع ہوتا ہے، اور اس کی قدر کوئی کمانڈ نہیں ہے
core-policy-autostart-runs-other = یہ خود بخود شروع ہوتا ہے، اور ایسی چیز چلاتا ہے جو یہ پیکیج انسٹال نہیں کرتا
core-policy-environment-own-rules = ماحولیاتی متغیرات اپنے قواعد کے مطابق بحال کیے جاتے ہیں
core-policy-service-not-restored = یہ ایسی سروس کا حصہ ہے جسے یہ پیکیج بحال نہیں کرتا
core-policy-service-own-rules = کسی سروس کی اپنی ترتیبات صرف اس کے قواعد کے ذریعے بحال کی جاتی ہیں
core-policy-service-dll = اس کی ServiceDll ایسی فائل نہیں ہے جو یہ پیکیج انسٹال کرتا ہے
core-policy-event-log-source-only = ایونٹ لاگ کے ماخذ کی صرف اپنی اقدار بحال کی جا سکتی ہیں
core-policy-security-log = کوئی ایپ Security لاگ میں نہیں لکھتی
core-policy-event-log-messages = اس کے پیغامات ایسی فائل سے آئیں گے جو یہ پیکیج انسٹال نہیں کرتا
core-policy-type-library = اس کی ٹائپ لائبریری ایسی فائل نہیں ہے جو یہ پیکیج انسٹال کرتا ہے
# { $name } is a program's file name, such as app.exe.
core-policy-application-opened = یہ تبدیل کر دے گا کہ { $name } کیسے کھولا جاتا ہے، جسے یہ پیکیج انسٹال نہیں کرتا
core-policy-event-log-no-file = ایونٹ لاگ کے ماخذ کو اپنے پیغامات کسی فائل سے لینے ہوتے ہیں
core-policy-com-runs-other = یہ ایسی چیز چلاتا ہے جو اس پیکیج کی انسٹال کردہ فائل نہیں ہے
core-policy-com-redirect = یہ اس کلاس کو ایسی چیز بنا دے گا جو یہ پیکیج انسٹال نہیں کرتا
core-policy-com-server = یہ ایسی چیز چلاتا ہے جو نہ اس پیکیج کی انسٹال کردہ فائل ہے اور نہ Windows کا حصہ
core-policy-interface = یہ ایسی چیز کے ذریعے، جو یہ پیکیج انسٹال نہیں کرتا، طے کرے گا کہ کسی انٹرفیس کو کالز کیسے کی جاتی ہیں
# { $name } is a program's file name, such as app.exe.
core-policy-app-id-started = یہ تبدیل کر دے گا کہ { $name } کیسے شروع کیا جاتا ہے، جسے یہ پیکیج انسٹال نہیں کرتا
core-policy-app-id-surrogate = اسے ایک ایسا پروگرام ہوسٹ کرے گا جو یہ پیکیج انسٹال نہیں کرتا
core-policy-app-id-service = یہ ایسی سروس میں چلے گا جسے یہ پیکیج بحال نہیں کرتا
core-policy-app-id-run-as = یہ اپنے الگ اکاؤنٹ کے طور پر چلے گا
core-policy-app-id-link = یہ کسی AppID کا نام نہیں لیتا
core-policy-app-id-remote = یہ اس PC کے علاوہ کسی اور جگہ چلے گا
core-policy-odbc-own-driver = یہ ODBC کی اپنی ترتیب ہے، ڈرائیور نہیں
core-policy-odbc-driver-not-installed = اس کا ڈرائیور ایسی فائل نہیں ہے جو یہ پیکیج انسٹال کرتا ہے
core-policy-loads-other-file = یہ ایسی فائل لوڈ کرے گا جو یہ پیکیج انسٹال نہیں کرتا
core-policy-odbc-own-source = یہ ODBC کی اپنی ترتیب ہے، ڈیٹا ماخذ نہیں
core-policy-odbc-source-missing = یہ ایسا ڈیٹا ماخذ درج کرتا ہے جو اس پیکیج میں نہیں ہے
core-policy-odbc-source-driver = اس کا ڈرائیور نہ اس پیکیج کی انسٹال کردہ فائل ہے اور نہ Windows کا حصہ
core-policy-shell-extension = یہ فائل ایکسپلورر میں ایسی شیل ایکسٹینشن لوڈ کرے گا جو اس پیکیج کی انسٹال کردہ فائل نہیں ہے
core-policy-not-text = اس میں موجود مواد وہ متن نہیں جس کے طور پر اسے پڑھا جائے گا
core-policy-network-place = یہ نیٹ ورک پر کسی جگہ کا نام لیتا ہے
core-policy-font = یہ ایسا فونٹ رجسٹر کرے گا جو یہ پیکیج انسٹال نہیں کرتا، یا Windows فولڈر میں موجود کوئی فونٹ
core-policy-compatibility = یہ تبدیل کر دے گا کہ ایسا پروگرام کیسے چلتا ہے جو یہ پیکیج انسٹال نہیں کرتا
core-policy-capabilities = یہ ایسی ایپ درج کرتا ہے جس کی تفصیلات اس پیکیج میں نہیں ہیں
core-policy-default-program = یہ تبدیل کر دے گا کہ کون سا پروگرام چیزوں کو بطور ڈیفالٹ کھولتا ہے
core-policy-url-protocol = یہ کہتا ہے کہ یہ لنک کی ایک قسم ہے، اور ایسی کسی چیز سے نہیں کھلتا جو یہ پیکیج انسٹال کرتا ہے
core-policy-command-handler = یہ ایک ایسی کمانڈ شامل کرتا ہے جسے ایسی چیز سنبھالتی ہے جو یہ پیکیج انسٹال نہیں کرتا
core-policy-installer-command = یہ ایسی چیز چلائے گا جسے صرف Windows Installer پڑھ سکتا ہے
core-policy-creates-class = یہ ایسی کلاس بنائے گا جو یہ پیکیج انسٹال نہیں کرتا
core-policy-opens-links = یہ لنکس کو ایسی چیز سے کھولتا ہے جو یہ پیکیج انسٹال نہیں کرتا
core-policy-opens-files = یہ فائلوں کو ایسی چیز سے کھولتا ہے جو یہ پیکیج انسٹال نہیں کرتا
core-policy-firewall-name = اس کا نام ایسا نہیں جسے یہ آگے منتقل کر سکے
core-policy-firewall-description = اس کی تفصیل ایسی نہیں جسے یہ آگے منتقل کر سکے
core-policy-firewall-every-program = یہ ہر پروگرام پر لاگو ہوتا ہے، جبکہ قاعدہ صرف ایک پروگرام کے لیے ہو سکتا ہے
# { $path } is the program the rule is for.
core-policy-firewall-program = یہ { $path } کے لیے ہے، جو یہ پیکیج انسٹال نہیں کرتا
core-policy-firewall-direction = یہ کوئی ایسی سمت نہیں بتاتا جو یہ سیٹ کر سکے
core-policy-firewall-action = یہ دوسرے قواعد پر غالب آ جائے گا، جو کسی ایپ کے قاعدے کو نہیں کرنا چاہیے
# { $setting } is a part of a firewall rule, as the scan of its PC described it.
core-policy-firewall-not-carried = اس میں { $setting } درج ہے، جسے ویسا سیٹ نہیں کیا جا سکا جیسا یہ تھا، یا جو دوسرے قواعد کے اثر کو کمزور کر دے گا
core-policy-firewall-service = یہ ایسی سروس کے لیے ہے جسے یہ پیکیج بحال نہیں کرتا
core-policy-firewall-parts = اس کا کچھ حصہ ایسا ہے جو فائر وال قاعدے میں بیان نہیں ہو سکتا
core-policy-folder-already-there = وہ فولڈر تنصیب کے مقام سے باہر پہلے سے موجود تھا، اور کہیں درج نہیں کہ یہ اس ایپ کا ہے، اس لیے اس میں کوئی پروگرام نہیں رکھا جاتا
core-policy-variable-name = اس کا نام درست متغیر نام نہیں ہے
core-policy-variable-relied-on = Windows اور دوسرے پروگرام اس متغیر پر انحصار کرتے ہیں
core-policy-task-name = اس کا نام ایسا نہیں جسے Windows قبول کرے
core-policy-task-folder = اس کا فولڈر ایسا نہیں جسے Windows قبول کرے
core-policy-task-windows = یہ Windows کے اپنے ٹاسکس میں سے کسی ایک کی جگہ لے لے گا
core-policy-task-other-action = یہ پروگرام چلانے کے علاوہ کچھ اور کرتا ہے
core-policy-task-no-program = یہ چلانے کے لیے کسی پروگرام کا نام نہیں لیتا
# { $path } is the program the task runs.
core-policy-task-runs-other = یہ ایسی چیز چلاتا ہے جو یہ پیکیج انسٹال نہیں کرتا: { $path }
core-policy-task-exists-unreadable = اس نام کا ایک ٹاسک پہلے سے موجود ہے اور اسے پڑھا نہیں جا سکا
core-policy-task-exists-other = اس نام کا ایک ٹاسک پہلے سے موجود ہے اور وہ ایپ نہیں چلاتا
# { $path } is the program the task already on this PC runs.
core-policy-task-exists-runs = اس نام کا ایک ٹاسک پہلے سے موجود ہے اور { $path } چلاتا ہے، جو یہاں کی کسی پچھلی تنصیب سے نہیں ہے
core-policy-shortcut-not-lnk = یہ .lnk شارٹ کٹ نہیں ہے
core-policy-shortcut-place = یہ کسی اسٹارٹ مینیو میں یا ڈیسک ٹاپ پر نہیں ہے
core-policy-short-name = اس کا پاتھ ایسا نام استعمال کرتا ہے جس کی جانچ نہیں کی جا سکتی
core-policy-shortcut-no-folder = اس PC پر اس کے لیے ایسا کوئی فولڈر نہیں ہے
core-policy-data-startup = یہ کسی اسٹارٹ اپ فولڈر میں جائے گا، اور ہر سائن ان پر چلے گا
core-policy-data-powershell = یہ وہاں جائے گا جہاں سے PowerShell شروع ہوتے وقت ملنے والی چیزیں چلاتا ہے
core-policy-data-windows = یہ ایسے فولڈر میں جائے گا جو Windows کا ہے
core-policy-license-outside = یہ تنصیب کے مقام اور ہر معلوم ڈیٹا فولڈر سے باہر ہے
core-rewrite-cdata = اس میں CDATA سیکشن ہے
core-rewrite-too-much-to-look-through = اس پیکیج کی ترتیبات میں اتنا کچھ ہے کہ ان میں درج چیزوں کی جانچ نہیں کی جا سکتی

## Warnings a review gives, which the installer shows with what it sets up.

# { $kind } says what was left out, and is not shown itself. { $item } is its path or name, or a registry value such
# as HKLM\Software\App\Name. { $reason } is why, as the second half of the sentence, such as "it cannot be a whole
# drive".
core-warning-review-left-out =
    { $kind ->
        [file] فائل { $item } کو خارج کر دیا گیا: { $reason }۔
        [folder] فولڈر { $item } کو خارج کر دیا گیا: { $reason }۔
        [service] سروس { $item } کو خارج کر دیا گیا: { $reason }۔
        [driver] ڈرائیور { $item } کو خارج کر دیا گیا: { $reason }۔
        [firewall-rule] فائر وال قاعدہ { $item } کو خارج کر دیا گیا: { $reason }۔
        [license-data] { $item } پر موجود لائسنس کا ڈیٹا خارج کر دیا گیا: { $reason }۔
        [license-value] لائسنس کی قدر { $item } کو خارج کر دیا گیا: { $reason }۔
        [scheduled-task] شیڈول کردہ ٹاسک { $item } کو خارج کر دیا گیا: { $reason }۔
        [shortcut] شارٹ کٹ { $item } کو خارج کر دیا گیا: { $reason }۔
        [environment-variable] ماحولیاتی متغیر { $item } کو خارج کر دیا گیا: { $reason }۔
       *[value] { $item } کو خارج کر دیا گیا: { $reason }۔
    }
core-warning-review-package-path-missing = کاپی کے قابل ایک فائل کا پیکیج کے اندر کوئی نسبتی پاتھ نہیں ہے۔
# { $count } is the most empty folders a package makes.
core-warning-review-too-many-empty-folders = یہ پیکیج { $count } سے زیادہ خالی فولڈرز درج کرتا ہے، اور صرف اتنے ہی بنائے جاتے ہیں۔
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Documents folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-documents-elsewhere =
    { $count ->
        [one] دستاویزات کی { $count } فائل اس کے بجائے { $target } کے تحت، _external نامی فولڈر میں جاتی ہے، کیونکہ اس PC کا دستاویزات فولڈر { $folder } پر ہے، اور صرف آپ کی پروفائل کے اندر موجود فولڈر میں لکھا جاتا ہے۔ اسے استعمال کرنے کے لیے خود اپنے دستاویزات فولڈر میں منتقل کریں۔
       *[other] دستاویزات کی { $count } فائلیں اس کے بجائے { $target } کے تحت، _external نامی فولڈر میں جاتی ہیں، کیونکہ اس PC کا دستاویزات فولڈر { $folder } پر ہے، اور صرف آپ کی پروفائل کے اندر موجود فولڈر میں لکھا جاتا ہے۔ انہیں استعمال کرنے کے لیے خود اپنے دستاویزات فولڈر میں منتقل کریں۔
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-documents-unknown =
    { $count ->
        [one] دستاویزات کی { $count } فائل اس کے بجائے { $target } کے تحت، _external نامی فولڈر میں جاتی ہے، کیونکہ یہ معلوم نہیں ہو سکا کہ اس PC کا دستاویزات فولڈر کہاں ہے۔ اسے استعمال کرنے کے لیے خود اپنے دستاویزات فولڈر میں منتقل کریں۔
       *[other] دستاویزات کی { $count } فائلیں اس کے بجائے { $target } کے تحت، _external نامی فولڈر میں جاتی ہیں، کیونکہ یہ معلوم نہیں ہو سکا کہ اس PC کا دستاویزات فولڈر کہاں ہے۔ انہیں استعمال کرنے کے لیے خود اپنے دستاویزات فولڈر میں منتقل کریں۔
    }
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Saved Games folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-elsewhere =
    { $count ->
        [one] محفوظ کردہ گیمز کی { $count } فائل اس کے بجائے { $target } کے تحت، _external نامی فولڈر میں جاتی ہے، کیونکہ اس PC کا محفوظ کردہ گیمز فولڈر { $folder } پر ہے، اور صرف آپ کی پروفائل کے اندر موجود فولڈر میں لکھا جاتا ہے۔ اسے استعمال کرنے کے لیے خود اپنے محفوظ کردہ گیمز فولڈر میں منتقل کریں۔
       *[other] محفوظ کردہ گیمز کی { $count } فائلیں اس کے بجائے { $target } کے تحت، _external نامی فولڈر میں جاتی ہیں، کیونکہ اس PC کا محفوظ کردہ گیمز فولڈر { $folder } پر ہے، اور صرف آپ کی پروفائل کے اندر موجود فولڈر میں لکھا جاتا ہے۔ انہیں استعمال کرنے کے لیے خود اپنے محفوظ کردہ گیمز فولڈر میں منتقل کریں۔
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-unknown =
    { $count ->
        [one] محفوظ کردہ گیمز کی { $count } فائل اس کے بجائے { $target } کے تحت، _external نامی فولڈر میں جاتی ہے، کیونکہ یہ معلوم نہیں ہو سکا کہ اس PC کا محفوظ کردہ گیمز فولڈر کہاں ہے۔ اسے استعمال کرنے کے لیے خود اپنے محفوظ کردہ گیمز فولڈر میں منتقل کریں۔
       *[other] محفوظ کردہ گیمز کی { $count } فائلیں اس کے بجائے { $target } کے تحت، _external نامی فولڈر میں جاتی ہیں، کیونکہ یہ معلوم نہیں ہو سکا کہ اس PC کا محفوظ کردہ گیمز فولڈر کہاں ہے۔ انہیں استعمال کرنے کے لیے خود اپنے محفوظ کردہ گیمز فولڈر میں منتقل کریں۔
    }

## Errors when a folder that only administrators may change cannot be made so, or checked.

# { $path } is a folder.
core-privilege-not-a-folder = { $path } لنک یا فائل ہے، فولڈر نہیں
# { $path } is a folder. { $error } is what Windows said, in its own words.
core-privilege-not-made-here = { $path } کو WinAppMigrator نے نہیں بنایا، اور دوسرے اس کے مندرجات تبدیل کر سکتے ہیں۔ کسی منتظم کو اس کی جانچ کر کے اسے ہٹا دینا چاہیے: { $error }
# { $path } is a folder.
core-privilege-others-put-something-in = { $path } میں کوئی ایسی چیز ہے جو WinAppMigrator نے نہیں بنائی، اور دوسرے اسے تبدیل کر سکتے ہیں۔ کسی منتظم کو اس کی جانچ کر کے اسے ہٹا دینا چاہیے
# { $path } is a folder.
core-privilege-became-a-link-in-use = { $path } استعمال کے دوران لنک بن گیا
# { $path } is a folder.
core-privilege-became-a-link-looked-at = { $path } دیکھے جانے کے دوران لنک بن گیا
# { $path } is a file. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-users-cannot-start = صارفین کو { $path } شروع کرنے کی اجازت نہیں دی جا سکی: { $said }
core-privilege-elevation-unknown = یہ معلوم نہیں ہو سکا کہ آیا یہ پروسیس منتظم کے حقوق کے ساتھ چل رہا ہے
# { $path } is a folder. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-cannot-restrict = { $path } کو صرف منتظمین تک محدود نہیں کیا جا سکا: { $said }
# { $path } is a folder. { $said } is the folder's permissions as Windows wrote them, and what else it printed.
core-privilege-not-restricted = { $path } کو صرف منتظمین تک محدود نہیں کیا جا سکا، اور ممکن ہے اس کی ڈرائیو اجازتیں محفوظ نہ رکھتی ہو: { $said }
# { $path } is a folder.
core-privilege-written-to-first = اس سے پہلے کہ { $path } کو محفوظ کیا جا سکتا، کسی دوسرے پروسیس نے اس میں لکھ دیا
core-privilege-no-program-data = Windows یہ نہیں بتاتا کہ ProgramData کہاں ہے

## Checks of a package's files, and of what an install put in place. Each is said of one file, value, service, or the
## like, which the report names beside it.

# { $path } is the installer's file.
core-verify-no-package = { $path } میں کوئی پیکیج نہیں ہے
# { $path } is a folder.
core-verify-not-unpacked = اس انسٹالر کا پیکیج { $path } میں نہیں نکالا گیا
core-verify-schema-supported = پیکیج مینی فیسٹ کا اسکیما معاون ہے۔
core-verify-packaged-file-missing = پیکیج کی فائل موجود نہیں ہے۔
core-verify-no-package-path = کاپی کے قابل فائل کا پیکیج کے اندر کوئی نسبتی پاتھ نہیں ہے۔
core-verify-folder-there = خالی فولڈر موجود ہے۔
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-unreadable = جانچ کے لیے اسے پڑھا نہیں جا سکا: { $error }
core-verify-restored-file-missing = بحال کردہ فائل موجود نہیں ہے۔
# { $expected } and { $actual } are sizes in bytes.
core-verify-size-mismatch = فائل کا سائز مماثل نہیں ہے۔ متوقع { $expected } بائٹس، ملے { $actual } بائٹس۔
core-verify-no-hash = مینی فیسٹ میں اس فائل کا کوئی ہیش درج نہیں، اس لیے اس کی جانچ نہیں کی جا سکتی۔
core-verify-hash-mismatch = SHA-256 ہیش مماثل نہیں ہے۔
core-verify-file-matches = فائل کا سائز اور ہیش مینی فیسٹ سے مماثل ہیں۔
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-payload-failed = لائسنس کے پے لوڈ کی تصدیق ناکام ہو گئی: { $error }
core-verify-license-payload-missing = پیکیج میں لائسنس کا پے لوڈ موجود نہیں ہے۔
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-failed = لائسنس اسٹور کی تصدیق ناکام ہو گئی: { $error }
core-verify-license-store-file-missing = پیکیج میں لائسنس اسٹور کی فائل موجود نہیں ہے۔
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-path-invalid = پیکیج میں لائسنس اسٹور کا پاتھ غلط ہے: { $error }
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-prerequisite-failed = پیشگی ضرورت کے پے لوڈ کی تصدیق ناکام ہو گئی: { $error }
core-verify-prerequisite-missing = شامل کردہ پیشگی ضرورت کا پے لوڈ موجود نہیں ہے۔
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-failed = ڈرائیور امیج کی تصدیق ناکام ہو گئی: { $error }
core-verify-bundled-driver-missing = شامل کردہ ڈرائیور امیج موجود نہیں ہے۔
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-path-invalid = شامل کردہ ڈرائیور کا پاتھ غلط ہے: { $error }
core-verify-prerequisite-present = رن ٹائم کی پیشگی ضرورت ہدف پر موجود ہے۔
core-verify-prerequisite-not-detected = رن ٹائم کی پیشگی ضرورت نہیں ملی؛ ہو سکتا ہے اس کے لیے دوبارہ شروع کرنا یا دستی طور پر مکمل کرنا درکار ہو (حتمی نتیجے کے لیے بحالی کا لاگ دیکھیں)۔
core-verify-prerequisite-not-probed = پیشگی ضرورت کی موجودگی رجسٹری سے جانچی نہیں جا سکتی؛ تنصیب حتی الامکان کوشش کے ساتھ کی گئی۔
core-verify-license-file-missing = بحال کردہ لائسنس فائل موجود نہیں ہے۔
core-verify-license-store-restored-missing = بحال کردہ لائسنس اسٹور فائل موجود نہیں ہے۔
core-verify-registry-key-there = رجسٹری کلید موجود ہے۔
core-verify-registry-value-matches = رجسٹری قدر محفوظ شدہ ڈیٹا سے مماثل ہے۔
core-verify-registry-value-differs = رجسٹری قدر محفوظ شدہ ڈیٹا سے مماثل نہیں ہے۔
core-verify-firewall-rule-there = Windows فائر وال میں یہ قاعدہ موجود ہے۔
core-verify-variable-matches = ماحولیاتی متغیر کی قدر محفوظ شدہ ڈیٹا سے مماثل ہے۔
core-verify-variable-differs = ماحولیاتی متغیر کی قدر محفوظ شدہ ڈیٹا سے مماثل نہیں ہے۔
# { $error } is what Windows said, in its own words.
core-verify-variable-unreadable = ماحولیاتی متغیر موجود نہیں یا پڑھا نہیں جا سکتا: { $error }
core-verify-service-runs-other = سروس موجود ہے لیکن بحال کردہ بائنری کو اس کے آرگیومنٹس کے ساتھ نہیں چلاتی۔
core-verify-service-other-account = سروس موجود ہے لیکن متوقع اکاؤنٹ کے طور پر نہیں چلتی۔
core-verify-service-matches = سروس موجود ہے، اور بحال کردہ بائنری کو اس کے آرگیومنٹس اور اکاؤنٹ کے ساتھ چلاتی ہے۔
core-verify-service-missing = سروس موجود نہیں ہے۔
# { $error } is what Windows said, in its own words.
core-verify-service-unreadable = سروس پڑھی نہیں جا سکتی: { $error }
core-verify-driver-other-image = ڈرائیور سروس بحال کردہ امیج کی طرف اشارہ نہیں کرتی۔
core-verify-driver-not-registered = بحالی کے بعد شامل کردہ ڈرائیور سروس رجسٹرڈ نہیں ہے۔
# { $error } is what Windows said, in its own words.
core-verify-driver-unreadable = ڈرائیور سروس پڑھی نہیں جا سکتی: { $error }
core-verify-driver-image-missing = ڈرائیور امیج موجود نہیں ہے۔
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-unchecked = ڈرائیور امیج کی جانچ نہیں کی جا سکی: { $error }
core-verify-task-matches = شیڈول کردہ ٹاسک موجود ہے اور بحال کردہ پروگرام چلاتا ہے۔
core-verify-task-runs-other = شیڈول کردہ ٹاسک موجود ہے لیکن بحال کردہ پروگرام نہیں چلاتا۔
core-verify-task-missing = شیڈول کردہ ٹاسک موجود نہیں ہے۔
# { $error } is what Windows said, in its own words.
core-verify-task-unreadable = شیڈول کردہ ٹاسک کی معلومات حاصل نہیں کی جا سکیں: { $error }
core-verify-shortcut-matches = شارٹ کٹ موجود ہے اور اس کا ہدف پاتھ بحال کردہ ہدف سے مماثل ہے۔
core-verify-shortcut-opens-other = شارٹ کٹ موجود ہے لیکن اس کا ہدف پاتھ بحال کردہ ہدف سے مماثل نہیں ہے۔
core-verify-shortcut-missing = شارٹ کٹ موجود نہیں ہے۔
# { $error } is what Windows said, in its own words.
core-verify-shortcut-unreadable = شارٹ کٹ کی معلومات حاصل نہیں کی جا سکیں: { $error }

## Warnings and errors a check gives, which the app and the installer show.

core-warning-verify-schema-unsupported = پیکیج مینی فیسٹ کا اسکیما معاون نہیں ہے۔
# { $error } is what went wrong, in Windows' or the program's own words.
core-warning-verify-package-file-failed = پیکیج فائل کی تصدیق ناکام ہو گئی: { $error }
core-warning-verify-folder-missing = خالی فولڈر موجود نہیں ہے۔
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-key-missing = رجسٹری کلید موجود نہیں یا پڑھی نہیں جا سکتی: { $error }
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-value-missing = رجسٹری قدر موجود نہیں یا پڑھی نہیں جا سکتی: { $error }
core-warning-verify-registry-kind-missing = رجسٹری قدر کی قسم موجود نہیں ہے۔
core-warning-verify-firewall-rule-missing = Windows فائر وال میں یہ قاعدہ موجود نہیں ہے۔
core-warning-verify-service-scope = سروس کی تصدیق HKLM میں سروس کی کنفیگریشن پڑھتی ہے اور سروسز شروع نہیں کرتی۔
core-warning-verify-driver-manual = وینڈر یا Windows کے ساتھ آنے والے ایک یا زیادہ ڈرائیورز WinAppMigrator آگے تقسیم نہیں کرتا؛ انہیں ہدف پر وینڈر کے دستخط شدہ ڈرائیور پیکیج سے دوبارہ انسٹال کریں۔
core-warning-verify-task-scope = شیڈول کردہ ٹاسک کی تصدیق صرف رجسٹریشن اور کمانڈ کے حوالے کی جانچ کرتی ہے۔

## The verification report written beside a package, verification_report.md. Its headings follow a "#".

core-verify-report-title = WinAppMigrator تصدیقی رپورٹ
# { $time } is when, as a date and time in UTC, such as 2026-11-15T10:00:00Z.
core-verify-report-verified = تصدیق کا وقت (UTC): { $time }
# { $valid } is true or false, which are not translated.
core-verify-report-package-valid = پیکیج درست: { $valid }
# { $valid } is true or false, which are not translated.
core-verify-report-restore-valid = بحالی درست: { $valid }
core-verify-report-restore-not-run = بحالی درست: نہیں چلائی گئی
core-verify-report-items = آئٹمز
# { $message } is what a check found, and { $source } the file or setting it looked at.
core-verify-report-item-from = { $message } ماخذ: { $source }
core-verify-report-warnings = انتباہات
core-verify-report-errors = خرابیاں
core-verify-report-none = کوئی نہیں۔
