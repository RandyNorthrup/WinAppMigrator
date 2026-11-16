# Words of the app, WinAppMigrator itself: its one window, with the installed apps listed on the left and, on the
# right, what goes in an installer of the one picked and the button that builds it; and the steps to read another PC
# through WinAppMigrator Client. Message names start with app-.


## The window and its header

# The program's name, in the window's title bar and at the top of the window. Keep it as it is unless your language
# writes product names in a script of its own.
app-name = WinAppMigrator
app-tagline = ইনস্টল করা একটি অ্যাপকে সেলফ-এক্সট্র্যাক্টিং ইনস্টলারে পরিণত করুন
app-theme-to-light = হালকা থিমে যান
app-theme-to-dark = গাঢ় থিমে যান
app-reload-list = অ্যাপ্লিকেশনের তালিকা পুনরায় লোড করুন
app-list-loading = লোড হচ্ছে
# { $count } is how many apps are listed.
app-list-count =
    { $count ->
        [one] { $count }টি অ্যাপ
       *[other] { $count }টি অ্যাপ
    }
app-list-load-failed = লোড ব্যর্থ হয়েছে
# Shown when the window cannot open at all. { $error } is what went wrong, in the words of Windows or a graphics driver.
app-window-could-not-open = উইন্ডোটি খোলা যায়নি: { $error }

## The list of installed apps

app-list-search = নাম বা প্রকাশক দিয়ে অনুসন্ধান করুন  (Ctrl+F)
app-list-filter-all = সব
# Whether an app is installed for one account or for everyone on the PC.
app-scope-per-user = ব্যবহারকারী
app-scope-per-machine = মেশিন
app-scope-unknown = অজানা
# An app whose processor architecture (x86, x64, ARM64) is not known.
app-arch-unknown = অজানা
app-list-loading-apps = ইনস্টল করা অ্যাপ্লিকেশনগুলো লোড হচ্ছে…
# { $error } is why the apps could not be listed.
app-list-could-not-list = অ্যাপ্লিকেশনগুলোর তালিকা তৈরি করা যায়নি: { $error }
app-list-no-matches = মিলে যায় এমন কোনো অ্যাপ্লিকেশন নেই
app-list-no-matches-hint = অনুসন্ধানের লেখা বা পরিসরের ফিল্টার বদলে দেখুন।
app-list-worker-disconnected = অ্যাপের তালিকা তৈরির প্রক্রিয়ার সঙ্গে সংযোগ বিচ্ছিন্ন হয়েছে।
# What a screen reader says of an app's row. { $name } is the app's name, { $publisher } who made it, and { $version } its
# version, such as 11.4.2, or — when either is not known.
app-list-row = { $name }, { $publisher }, সংস্করণ { $version }

## The list's columns, and what a screen reader says each column's heading does when clicked

app-column-name = নাম
app-column-name-sort = নাম অনুসারে সাজান
app-column-publisher = প্রকাশক
app-column-publisher-sort = প্রকাশক অনুসারে সাজান
app-column-version = সংস্করণ
app-column-version-sort = সংস্করণ অনুসারে সাজান
app-column-scope = পরিসর
app-column-scope-sort = পরিসর অনুসারে সাজান
# Short for architecture: x86, x64, or ARM64. The column is narrow.
app-column-arch = আর্কি.
app-column-arch-sort = আর্কিটেকচার অনুসারে সাজান

## Where apps are read from: this PC, a drive from another PC, a disk image of one, or another PC on the network

app-source = স্ক্যানের উৎস
app-source-this-pc = এই পিসি
app-source-drive = সংযুক্ত ড্রাইভ…
app-source-image = ডিস্ক ইমেজ…
app-source-network = নেটওয়ার্ক পিসি…
app-source-attaching-image = ইমেজটি কেবল-পঠনযোগ্য হিসেবে সংযুক্ত করা হচ্ছে…
# { $computer } is the other PC's name, such as OLD-PC, and { $address } its network address, such as 192.168.1.20.
app-source-network-pc = নেটওয়ার্কের মাধ্যমে { $computer } ({ $address })
app-source-disconnect = সংযোগ বিচ্ছিন্ন করুন
app-source-change = পরিবর্তন করুন…
# The title of the window that picks a disk image. .vhd and .vhdx are file extensions.
app-source-pick-image-title = অন্য পিসির একটি ডিস্ক ইমেজ নির্বাচন করুন (.vhd বা .vhdx)
app-source-pick-image-filter = ডিস্ক ইমেজ
# { $path } is the disk image file.
app-source-image-has-no-windows = { $path }-এ Windows আছে এমন কোনো ভলিউম নেই।
app-source-image-not-attached = ডিস্ক ইমেজটি সংযুক্ত করা যায়নি।
app-source-pick-drive-title = সংযুক্ত ড্রাইভের রুট নির্বাচন করুন (যেমন E:\)
# { $path } is the folder picked, such as E:\. \Windows and \Users are folder names, as Windows writes them.
app-source-not-a-windows-drive = { $path } দেখে Windows সিস্টেম ড্রাইভ বলে মনে হচ্ছে না (\Windows ও \Users নেই)। অন্য পিসি থেকে আনা ড্রাইভটির রুট বেছে নিন।
# Said of a drive or disk image with a Windows older than Windows 10. { $windows } is its name, such as "Windows 7
# Professional Service Pack 1, 32-bit". { $is_32_bit } is "yes" when that Windows is 32-bit, which adds the last
# sentence, and "no" when it is not.
app-source-older-windows =
    এতে { $windows } আছে। এটি থেকে তৈরি ইনস্টলারগুলো Windows 10 ও 11-এর জন্য তৈরি। ব্যবহারকারীর ডেটা বিকল্প চালু থাকলে, কোনো পুরোনো অ্যাপের জন্য Windows নীরবে যে সেটিংস আলাদা করে রেখেছিল সেগুলোও সঙ্গে আসে, এবং Windows 11-এ নেই এমন রানটাইম, যেমন DirectX 9, অ্যাপের সঙ্গে ইনস্টল করা হয়।{ $is_32_bit ->
        [yes] { " " }এর অ্যাপগুলো 32-বিট, তাই সেগুলো Program Files (x86)-এ যায়, যেখানে 64-বিট Windows সেগুলো রাখে।
       *[no] { "" }
    }
# The same, of a Windows whose name could not be read. { $is_32_bit } is as above.
app-source-older-windows-unnamed =
    এতে একটি পুরোনো Windows আছে। এটি থেকে তৈরি ইনস্টলারগুলো Windows 10 ও 11-এর জন্য তৈরি। ব্যবহারকারীর ডেটা বিকল্প চালু থাকলে, কোনো পুরোনো অ্যাপের জন্য Windows নীরবে যে সেটিংস আলাদা করে রেখেছিল সেগুলোও সঙ্গে আসে, এবং Windows 11-এ নেই এমন রানটাইম, যেমন DirectX 9, অ্যাপের সঙ্গে ইনস্টল করা হয়।{ $is_32_bit ->
        [yes] { " " }এর অ্যাপগুলো 32-বিট, তাই সেগুলো Program Files (x86)-এ যায়, যেখানে 64-বিট Windows সেগুলো রাখে।
       *[no] { "" }
    }

## The details panel, before an app is picked, and the app picked

app-details-pair-first = পুরোনো পিসির সঙ্গে পেয়ার করুন
app-details-pair-first-hint = সেখানে WinAppMigrator Client চালু হলে এবং আপনি এর কোড টাইপ করলে, এর অ্যাপগুলো এখানে তালিকাভুক্ত হবে, এবং এই পিসির মতোই সেগুলোর যেকোনোটি থেকে আপনি ইনস্টলার তৈরি করতে পারবেন।
app-details-pick-an-app = একটি অ্যাপ্লিকেশন বেছে নিন
app-details-pick-an-app-hint = কী কী অন্তর্ভুক্ত হবে তা বেছে নিতে এবং ইনস্টলার তৈরি করতে তালিকা থেকে একটি অ্যাপ নির্বাচন করুন।
# { $publisher } is who made the app, and { $version } its version, such as 11.4.2.
app-details-publisher-and-version = { $publisher }, সংস্করণ { $version }
# { $version } is the app's version, such as 11.4.2.
app-details-unknown-publisher-and-version = অজানা প্রকাশক, সংস্করণ { $version }
app-details-unknown-publisher = অজানা প্রকাশক

## What goes in besides the program

app-option-license = লাইসেন্স ও অ্যাক্টিভেশন ডেটা
app-option-license-hint = এটি এই পিসির সঙ্গে আবদ্ধ থাকতে পারে, ফলে নতুন পিসিতে আবার অ্যাক্টিভেট করতে হতে পারে।
app-option-user-data = ব্যবহারকারীর ডেটা
# Documents, Saved Games, and ProgramData are folders of Windows: use the names Windows gives them in your language,
# except ProgramData, which keeps its name.
app-option-user-data-hint = সেটিংস ও ইতিহাস, নথিগুলো, সংরক্ষিত গেমগুলো এবং ProgramData।
app-option-whose-data = কার ডেটা
app-option-whose-data-hint = একটি ইনস্টলারে একটিমাত্র অ্যাকাউন্টের ডেটা থাকে। যে পিসিতে এটি চালানো হয়, সেখানে যিনি সাইন ইন করে আছেন তাঁর কাছে ডেটা যায়।

## Building the installer

app-output-folder = আউটপুট ফোল্ডার
# An example of a folder, shown in the empty field. "You" stands for the name of an account.
app-output-folder-example = C:\Users\You\WinAppMigrator
app-output-folder-not-full = একটি সম্পূর্ণ ফোল্ডার পাথ লিখুন, যেমন D:\Installers।
app-build-create = ইনস্টলার তৈরি করুন
app-build-create-again = আবার ইনস্টলার তৈরি করুন
app-build-cancel = বাতিল করুন
app-build-cancelling = বর্তমান ধাপের পরে বাতিল করা হচ্ছে…
app-build-cancelled = বিল্ড বাতিল করা হয়েছে। কোনো ইনস্টলার লেখা হয়নি।
# winappmigrator-installer.exe is a file name.
app-build-no-stub = এই অ্যাপের পাশে ইনস্টলার স্টাব (winappmigrator-installer.exe) পাওয়া যায়নি, তাই সেলফ-এক্সট্র্যাক্টিং ইনস্টলার তৈরি করা যাবে না।
app-build-select-an-app-first = প্রথমে স্থানান্তরের জন্য একটি অ্যাপ্লিকেশন নির্বাচন করুন।
app-build-look-again-first = ইনস্টলার তৈরি করার আগে এই অ্যাপটি আবার স্ক্যান করতে হবে।
# { $reason } is what went wrong, in the program's own words, often in English.
app-build-stopped-unexpectedly = বিল্ড অপ্রত্যাশিতভাবে থেমে গেছে: { $reason }। এটি আউটপুট ফোল্ডারে কোনো ইনস্টলার রেখে গিয়ে থাকলে, সেটি ব্যবহার করবেন না।
app-build-worker-disconnected = স্থানান্তর প্রক্রিয়ার সঙ্গে সংযোগ বিচ্ছিন্ন হয়েছে।
# { $path } is the output folder, and { $error } what Windows said went wrong.
app-build-no-output-folder = আউটপুট ফোল্ডার { $path } তৈরি করা যায়নি: { $error }
# { $error } is what went wrong.
app-build-no-staging-folder = একটি সুরক্ষিত স্টেজিং ফোল্ডার তৈরি করা যায়নি, তাই কিছুই তৈরি করা হয়নি: { $error }
# { $error } is what went wrong, as a sentence.
app-build-could-not-build = ইনস্টলারটি তৈরি করা যায়নি। { $error }
app-build-no-installer = বিল্ড শেষ হয়েছে, কিন্তু কোনো ইনস্টলার তৈরি হয়নি।
# { $error } is what went wrong.
app-build-hash-check-did-not-run = ইনস্টলারটি লেখা হয়েছে, কিন্তু এর হ্যাশ যাচাই চালানো যায়নি: { $error }। এটি ব্যবহার করবেন না।

## How far a build has got

app-progress-preparing = কী কী অন্তর্ভুক্ত হবে তা নির্ণয় করা হচ্ছে…
app-progress-checking-room = যথেষ্ট জায়গা আছে কি না যাচাই করা হচ্ছে…
app-progress-bundling = রানটাইম ও ড্রাইভার একত্র করা হচ্ছে…
# { $done } is how many files have been read so far, and { $total } how many there are.
app-progress-packing = ফাইল পড়া ও সংকুচিত করা হচ্ছে: { $total }টির মধ্যে { $done }টি
# { $done } is how many of the installer's records have been looked over so far, and { $total } how many there are.
app-progress-looking-over = ইনস্টলারের তথ্য পরীক্ষা করা হচ্ছে: { $total }টির মধ্যে { $done }টি…
app-progress-writing = ইনস্টলার লেখা হচ্ছে…
app-progress-verifying = হ্যাশ যাচাই করা হচ্ছে…

## What a build made

app-result-failed-hash-check = এই ইনস্টলারটি ব্যবহার করবেন না: এটি হ্যাশ যাচাইয়ে ব্যর্থ হয়েছে।
# { $count } is how many files were gone when the build came to read them.
app-result-files-gone =
    { $count ->
        [one] ইনস্টলার তৈরি হয়েছে, কিন্তু অ্যাপটি স্ক্যান করার পরে { $count }টি ফাইল আর পাওয়া যায়নি, তাই সেটি এতে নেই। নিচে “সতর্কতা” দেখুন।
       *[other] ইনস্টলার তৈরি হয়েছে, কিন্তু অ্যাপটি স্ক্যান করার পরে { $count }টি ফাইল আর পাওয়া যায়নি, তাই সেগুলো এতে নেই। নিচে “সতর্কতা” দেখুন।
    }
# { $count } is how many files could not be read into the installer.
app-result-files-not-captured =
    { $count ->
        [one] ইনস্টলার তৈরি হয়েছে, কিন্তু { $count }টি ফাইল সংগ্রহ করা যায়নি, সাধারণত সেটি লক করা বা অপঠনযোগ্য ছিল বলে। এটি ছাড়া অ্যাপটি হয়তো কাজ করবে না। নিচে “সতর্কতা” দেখুন।
       *[other] ইনস্টলার তৈরি হয়েছে, কিন্তু { $count }টি ফাইল সংগ্রহ করা যায়নি, সাধারণত সেগুলো লক করা বা অপঠনযোগ্য ছিল বলে। সেগুলো ছাড়া অ্যাপটি হয়তো কাজ করবে না। নিচে “সতর্কতা” দেখুন।
    }
app-result-mentions-who = ইনস্টলার তৈরি হয়েছে, তবে এতে এখনো এমন কিছু তথ্য আছে যা থেকে বোঝা যায় এটি কার কাছ থেকে এসেছে। কাউকে দেওয়ার আগে নিচে কী লেখা আছে তা পড়ুন।
app-result-ready = ইনস্টলার প্রস্তুত। এর প্যাকেজ হ্যাশ যাচাইয়ে উত্তীর্ণ হয়েছে।
app-result-files = ফাইল
app-result-installer-size = ইনস্টলারের আকার
app-result-warnings = সতর্কতা
app-result-log = লগ
app-result-copy-error = ত্রুটি অনুলিপি করুন
app-result-installer = ইনস্টলার
app-result-data-file = ডেটা ফাইল
app-result-open-folder = ফোল্ডার খুলুন
app-result-copy-path = পাথ অনুলিপি করুন
# { $error } is what Windows said went wrong.
app-result-could-not-open-folder = ফোল্ডারটি খোলা যায়নি: { $error }
# { $installer } and { $data_file } are file names, such as App_installer.exe and App_installer.wamdata.
app-result-two-files = এই অ্যাপটি একটিমাত্র এক্সিকিউটেবল ফাইলের জন্য খুব বড়, তাই এর ইনস্টলার দুটি ফাইলে: { $installer } ও { $data_file }। এগুলো একসঙ্গে রাখতে হবে: দুটিই অন্য পিসির একই ফোল্ডারে, এই নামেই অনুলিপি করুন। আপনি ইনস্টলারে স্বাক্ষর করলে, সেই স্বাক্ষর ডেটা ফাইলটিকেও অন্তর্ভুক্ত করে।
# { $app_name } is the app's name. "Unknown publisher" is what Windows shows when it asks to run an unsigned program: use
# Windows' own words for it in your language.
app-result-run-it = { $app_name } পুনরায় ইনস্টল করতে অন্য পিসিতে এটি চালান। এটি SHA-256 দিয়ে নিজের ফাইলগুলো যাচাই করে। Windows “অজানা প্রকাশক” দেখায়, কারণ আপনার তৈরি ইনস্টলারগুলো স্বাক্ষরিত নয়।
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-verification-errors = যাচাইয়ের ত্রুটি ({ $count })
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-warnings-heading = সতর্কতা ({ $count })
# After the first of a long list of warnings. { $count } is how many more there are.
app-warnings-and-more = এবং আরও { $count }টি।

## Looking at an app, to see what an installer of it would hold

app-scan-getting-ready = এই অ্যাপটি স্ক্যান করার প্রস্তুতি নেওয়া হচ্ছে…
app-scan-install-record = এটি কীভাবে ইনস্টল করা হয়েছিল তা পড়া হচ্ছে…
app-scan-near-matches = এর অংশ হতে পারে এমন আর কী আছে তা খোঁজা হচ্ছে…
app-scan-system-items = এর সেবা, কার্য ও শর্টকাট খোঁজা হচ্ছে…
app-scan-files = এর ফাইলগুলো দেখা হচ্ছে…
app-scan-registry = যা একে Windows-এর সঙ্গে যুক্ত করে তা পড়া হচ্ছে…
app-scan-finishing = শেষ করা হচ্ছে…
app-scan-look-again = আবার স্ক্যান করুন
app-scan-stop = থামান
app-scan-running-hint = এটি চলার সময় আপনি অন্য একটি অ্যাপ বেছে নিতে পারেন, বা কী অন্তর্ভুক্ত হবে তা বদলাতে পারেন।
# { $error } is why, as a sentence.
app-scan-failed = এই অ্যাপটি স্ক্যান করা যায়নি: { $error }
app-scan-try-again = আবার চেষ্টা করুন
# "Look Again" names the button of that name.
app-scan-stopped = থামানো হয়েছে। এই অ্যাপটি স্ক্যান করা হলে তবেই ইনস্টলার তৈরি করা যাবে।
# { $reason } is what went wrong, in the program's own words, often in English.
app-scan-stopped-unexpectedly = এটি অপ্রত্যাশিতভাবে থেমে গেছে: { $reason }।

## How big an installer can be, and whether there is room for it

app-room-files = ফাইল
# Over how big the installer can be: up to that size.
app-room-up-to = সর্বোচ্চ
app-room-free = খালি
# { $drive } is a drive, such as C:.
app-room-free-on = { $drive }-এ খালি
# How much room a drive has, when it would not say.
app-room-free-unknown = অজানা
app-room-most-it-could-need = সংকুচিত করার আগে, সর্বোচ্চ যতটা জায়গা লাগতে পারে।
# { $drive } is a drive, such as C:, { $needed } how much room it needs, such as 1.2 GB, and { $free } how much it has.
app-room-drive-short = এটি তৈরি করতে { $drive }-এ সর্বোচ্চ { $needed } খালি জায়গা প্রয়োজন, কিন্তু আছে { $free }।
# { $drives } is one sentence like the one above for each drive without the room, one after another.
app-room-not-enough = যথেষ্ট জায়গা নেই। { $drives } অন্য ড্রাইভে একটি আউটপুট ফোল্ডার বেছে নিন, অথবা কিছু বাদ দিন।
# { $note } is a sentence, without its full stop, about a drive whose room could not be checked. Use your language's
# full stop.
app-room-could-not-tell = { $note }।

## What goes in the installer, part by part

app-footprint-heading = ইনস্টলারে যা যাবে
app-footprint-working-out = এর সঙ্গে আর কী যাবে তা নির্ণয় করা হচ্ছে…
# { $error } is why.
app-footprint-would-not-make-a-package = যা বাকি আছে তা দিয়ে প্যাকেজ তৈরি হবে না: { $error }
# { $list } is the names of what else is left out, such as a service and a shortcut.
app-footprint-also-left-out = এর ফলে { $list }-ও বাদ পড়ে, কারণ সেগুলো যা চালায় বা যার নাম উল্লেখ করে তা থাকবে না। অন্য পিসিতে সেগুলো সেট আপ করা হবে না।
# { $list } is the names of the first few of what else is left out, and { $count } how many more there are.
app-footprint-also-left-out-and-more = এর ফলে { $list } এবং আরও { $count }টি জিনিসও বাদ পড়ে, কারণ সেগুলো যা চালায় বা যার নাম উল্লেখ করে তা থাকবে না। অন্য পিসিতে সেগুলো সেট আপ করা হবে না।
# { $list } is the names of license files. "License" names the License and Activation Data option.
app-footprint-licenses-still-go = আপনি যা বাদ দিয়েছেন তার মধ্যে থাকা লাইসেন্স ডেটা তবুও যাবে, কারণ তা লাইসেন্স ও অ্যাক্টিভেশন ডেটা বিকল্প অনুযায়ী যায়: { $list }।
# { $elapsed } is how long ago, in minutes and seconds such as 45:00, or hours, minutes, and seconds such as 1:05:00.
# "Look Again" names the button of that name.
app-footprint-looked-at-long-ago = এটি { $elapsed } আগে স্ক্যান করা হয়েছিল। ফাইলগুলো তৈরির সময় পড়া হয়, তাই কোনো পরিবর্তন হলে তা ধরা পড়বে, এবং ততক্ষণে যা সরে গেছে তা তালিকায় দেখানো হবে। নতুন কিছু থাকলে তার জন্য “আবার স্ক্যান করুন” ক্লিক করুন।
# Headings of the parts of the list, each with a line that says what is in it.
app-footprint-program-files = প্রোগ্রামের ফাইল
app-footprint-data-folders = ডেটা ফোল্ডার
# What could go in with the User Data option, when nothing else is in this part. { $size } is how much, such as 1.2 GB.
app-footprint-folders-with-user-data = ব্যবহারকারীর ডেটা বিকল্পের সঙ্গে { $size }
# { $included } is how many folders go in, { $count } how many there are, and { $size } how much those that go in hold.
app-footprint-folders-summary =
    { $count ->
        [one] { $count }টি ফোল্ডারের মধ্যে { $included }টি, { $size }
       *[other] { $count }টি ফোল্ডারের মধ্যে { $included }টি, { $size }
    }
# { $size } is how much, such as 1.2 GB, and { $count } in how many files.
app-footprint-data-only-with-user-data =
    { $count ->
        [one] এই অ্যাপের ডেটার { $count }টি ফাইলে থাকা { $size } কেবল ব্যবহারকারীর ডেটা বিকল্পের সঙ্গে যাবে।
       *[other] এই অ্যাপের ডেটার { $count }টি ফাইলে থাকা { $size } কেবল ব্যবহারকারীর ডেটা বিকল্পের সঙ্গে যাবে।
    }
# A folder's row. { $count } is how many files it holds, and { $detail } where it is or why it cannot be left out.
app-footprint-folder-row =
    { $count ->
        [one] { $count }টি ফাইল। { $detail }
       *[other] { $count }টি ফাইল। { $detail }
    }
app-footprint-services = সেবা
app-footprint-startup = স্বয়ংক্রিয়ভাবে শুরু হয়
app-footprint-tasks = নির্ধারিত কার্য
app-footprint-shortcuts = শর্টকাট
# { $included } is how many of the part's things go in, and { $count } how many there are.
app-footprint-items-summary = { $count }টির মধ্যে { $included }টি
# { $why } is what else was left out that this goes with.
app-footprint-left-out-too = এটিও বাদ পড়েছে: { $why }
# A row's detail. { $kind } is what it is, such as Service, and { $detail } what it does.
app-footprint-change-detail = { $kind }। { $detail }
app-footprint-integrations = যা একে Windows-এর সঙ্গে যুক্ত করে
# { $included } is how many kinds go in, such as file types and firewall rules, and { $count } how many kinds there are.
app-footprint-integrations-summary =
    { $count ->
        [one] { $count }টি ধরনের মধ্যে { $included }টি
       *[other] { $count }টি ধরনের মধ্যে { $included }টি
    }
app-footprint-integration-nothing-left = আপনি যা বাদ দিয়েছেন তার কারণে এর কিছুই বাকি নেই
# { $count } is how many things of this kind the installer sets up.
app-footprint-integration-things =
    { $count ->
        [one] { $count }টি জিনিস সেট আপ করা হয়
       *[other] { $count }টি জিনিস সেট আপ করা হয়
    }
app-footprint-prerequisites = চলার জন্য এর যা প্রয়োজন
# { $count } is how many runtimes, such as Visual C++ Redistributable.
app-footprint-prerequisites-summary =
    { $count ->
        [one] { $count }টি রানটাইম
       *[other] { $count }টি রানটাইম
    }
app-footprint-near-matches = এটিরও অংশ হতে পারে
# { $ticked } is how many were ticked, and { $count } how many there are.
app-footprint-near-matches-summary = { $count }টির মধ্যে { $ticked }টি যোগ করা হয়েছে
app-footprint-near-matches-hint = আপনি টিক না দিলে এগুলো ইনস্টলারে থাকবে না। কোনোটিতে টিক দিলে বাকিগুলোর মতো একই যাচাইসহ অ্যাপটি আবার স্ক্যান করা হয়, তাই পাসওয়ার্ডের মতো দেখায় এমন কিছু তখনো বাদ থাকে।
# { $why } is why it might be part of the app, as a sentence.
app-footprint-near-match-with-user-data = ব্যবহারকারীর ডেটা বিকল্পের সঙ্গে আসে। { $why }
# How much a folder holds, when not all of it could be measured. { $size } is how much was, such as 1.2 GB.
app-footprint-near-match-over = { $size }-এর বেশি
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-folder = ফোল্ডার: { $place }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-hidden = কোনো অ্যাপের লুকানো অংশ: { $place }
app-footprint-nothing-left-out = পাসওয়ার্ড, লাইসেন্স বা সার্টিফিকেটের মতো দেখানোর কারণে কিছুই বাদ দেওয়া হয়নি।
app-footprint-left-out = যা বাদ দেওয়া হয়েছে, এবং কেন
# { $count } is how many things were left out.
app-footprint-left-out-summary =
    { $count ->
        [one] { $count }টি জিনিস
       *[other] { $count }টি জিনিস
    }
# Above a list of things left out for one reason. { $title } is the reason.
app-footprint-left-out-group = { $title }:
app-footprint-refused = কোনো ইনস্টলারকেই এগুলো সেট আপ করতে দেওয়া হবে না:
# A heading over a list that opens and closes. { $count } is how many are in it.
app-footprint-worth-knowing = জেনে রাখা ভালো ({ $count })
# { $list } is what kinds of things were looked for and none found, from the five below.
app-none-found = কিছুই পাওয়া যায়নি: { $list }।
app-none-found-services = সেবা
app-none-found-startup = স্টার্টআপ আইটেম
app-none-found-tasks = নির্ধারিত কার্য
app-none-found-shortcuts = শর্টকাট
app-none-found-integrations = Windows-এর সঙ্গে একে যুক্ত করে এমন কিছু

## Network PC: reading another PC through WinAppMigrator Client, in three steps. "WinAppMigrator Client" is the name of a
## program, and the old PC is the one the apps are moved off.

app-network-step-client = 1. পুরোনো পিসিতে ক্লায়েন্টটি রাখুন
app-network-step-client-hint = অ্যাপটি এখন যে পিসিতে আছে, WinAppMigrator Client সেই পিসিতে চলে (Windows 7 বা তার পরের সংস্করণ)। এটি কেবল এই পিসিকেই সেটি পড়তে দেয়, এনক্রিপ্ট করা অবস্থায়, এবং কেবল এর দেখানো একটি কোড দিয়ে।
# { $drive } is a USB drive, such as E:\.
app-network-save-to-usb = USB ড্রাইভ { $drive }-এ সংরক্ষণ করুন
# { $drive } is a USB drive, such as E:\, and { $name } the name it was given, such as BACKUP.
app-network-save-to-named-usb = USB ড্রাইভ { $drive } ({ $name })-এ সংরক্ষণ করুন
app-network-save-to-folder = একটি ফোল্ডারে সংরক্ষণ করুন…
app-network-look-for-usb-again = আবার USB ড্রাইভ খুঁজুন
app-network-no-usb = কোনো USB ড্রাইভ লাগানো নেই। একটি লাগান, অথবা এমন একটি ফোল্ডারে সংরক্ষণ করুন যেখানে পুরোনো পিসি থেকে পৌঁছানো যায়, যেমন আপনার নেটওয়ার্কে ভাগ করা একটি ফোল্ডার।
# { $path } is where the client was saved, such as E:\WinAppMigrator Client.exe.
app-network-client-saved = { $path } হিসেবে সংরক্ষণ করা হয়েছে। পুরোনো পিসিতে, যে অ্যাপটি সরাচ্ছেন সেটি বন্ধ করুন, তারপর এটিতে ডাবল-ক্লিক করুন এবং পরিবর্তন করার অনুমতি দিন। এটি একটি কোড দেখাবে।
app-network-step-pick = 2. পুরোনো পিসিটি বেছে নিন
app-network-looking = আপনার নেটওয়ার্কে ক্লায়েন্ট চলছে এমন পিসি খোঁজা হচ্ছে…
app-network-type-name = অথবা এর নাম বা ঠিকানা টাইপ করুন
# An example, shown in the empty field: a PC's name, or its network address.
app-network-type-name-example = OLD-PC বা 192.168.1.20
app-network-step-code = 3. এটি যে কোড দেখায় তা টাইপ করুন
app-network-code = পেয়ারিং কোড
app-network-connect = সংযোগ করুন
app-network-pairing = পেয়ার করা হচ্ছে…
# Why pairing did not work, shown on its own.
app-network-pairing-stopped = পেয়ার করা শেষ হওয়ার আগেই থেমে গেছে
app-network-wrong-code = পুরোনো পিসি যে কোড দেখাচ্ছে, এটি সেই কোড নয়। কোডটি মিলিয়ে দেখে আবার চেষ্টা করুন।
# { $error } is what went wrong.
app-network-not-reached = পুরোনো পিসিতে পৌঁছানো যায়নি: { $error }
app-network-pick-folder-title = WinAppMigrator Client-এর জন্য একটি ফোল্ডার বেছে নিন
# { $path } is the folder, and { $error } what Windows said went wrong.
app-client-could-not-save = ক্লায়েন্টটি { $path }-এ সংরক্ষণ করা যায়নি: { $error }
# scripts\build-client.ps1 is a file name.
app-client-not-carried = WinAppMigrator-এর এই বিল্ডে ক্লায়েন্টটি নেই। scripts\build-client.ps1 দিয়ে সেটি তৈরি করুন, তারপর অ্যাপটি আবার তৈরি করুন।
# { $error } is what Windows said went wrong.
app-client-could-not-save-note = এর পাশের নোটটি সংরক্ষণ করা যায়নি: { $error }
# A note saved as a text file beside the client, for whoever runs it on the old PC. Keep its lines short, as it is read in
# Notepad. "WinAppMigrator Client.exe" is a file name. "Network PC..." names the Scan Source button: use its words in your
# language.
app-client-note =
    WinAppMigrator Client
    
    যে পিসি থেকে অ্যাপ সরাচ্ছেন, সেখানে এটি চালান: Windows 7, 8, 10 বা 11।
    
    1. যে অ্যাপটি সরাচ্ছেন সেটি বন্ধ করুন।
    2. “WinAppMigrator Client.exe”-এ ডাবল-ক্লিক করুন এবং পরিবর্তন করার
       অনুমতি দিন।
    3. এটি একটি 6-অঙ্কের পেয়ারিং কোড দেখাবে। আপনার নতুন পিসির
       WinAppMigrator-এ নেটওয়ার্ক পিসি... বেছে নিন, এই পিসিটি নির্বাচন
       করুন, এবং কোডটি টাইপ করুন।
    4. নতুন পিসির কাজ শেষ হলে, ক্লায়েন্টের উইন্ডোতে Enter চাপুন, অথবা
       উইন্ডোটি বন্ধ করুন।
    
    এটি কেবল পড়ে। চলার সময় এটি Windows ড্রাইভের একটি স্ন্যাপশট রাখে,
    এবং আপনার স্থানীয় নেটওয়ার্ক থেকে আসা সংযোগের জন্য Windows
    ফায়ারওয়ালে এর নিজের দুটি পোর্ট খুলে রাখে; থামলে এটি দুটিই সরিয়ে দেয়।
    নেটওয়ার্কের মধ্য দিয়ে যা যায় তা এনক্রিপ্ট করা থাকে, এবং কেবল কোডটি
    জানা পিসিই কিছু পড়তে পারে।
