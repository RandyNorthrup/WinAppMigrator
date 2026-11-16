# Words of the installer that every built installer carries, which runs on the PC an app is moved to and speaks its
# language, and of the uninstaller it leaves behind for Settings. Message names start with installer- for the
# installer's window, installer-cli- for its command line, and uninstaller- for the uninstaller.
#
# The command lines (installer-cli-, uninstaller-cli-) are always shown in English, as scripts read what they print.
# Translate them all the same. Keep their command-line flags, such as --apply and --target-root, exactly as they are.


## The installer's window: its title and header

installer-window-title = WinAppMigrator ইনস্টলার
# The window's title in the taskbar. { $app_name } is the name of the app it installs.
installer-window-title-app = { $app_name } ইনস্টল
installer-tagline = অন্য একটি পিসি থেকে সংগ্রহ করা অ্যাপ্লিকেশন পুনরায় ইনস্টল করে
# What an app is called when its package gives it no name.
installer-unnamed-app = অ্যাপ্লিকেশন

## When the installer's window cannot open. Each is a paragraph of a message box.

# { $error } is what went wrong, as Windows or the graphics driver says it.
installer-window-failed = ইনস্টলারের উইন্ডো খোলা যায়নি: { $error }
installer-window-failed-without-one = উইন্ডো ছাড়াও এটি ইনস্টল করতে পারে: প্রশাসকের কমান্ড প্রম্পট থেকে --apply দিয়ে এই ফাইলটি চালান, আর কোথায় ইনস্টল হবে তা বেছে নিতে --target-root-এর পরে একটি ফোল্ডার দিন।

## Checking the installer when it opens

installer-checking = ইনস্টলারটি যাচাই করা হচ্ছে…
installer-stub-no-package = এটি একটি WinAppMigrator ইনস্টলার স্টাব, এবং এতে কোনো অ্যাপ্লিকেশন প্যাকেজ নেই। WinAppMigrator দিয়ে একটি ইনস্টলার তৈরি করুন, তারপর তৈরি হওয়া .exe ফাইলটি এখানে চালান।
installer-check-stopped = ইনস্টলারের যাচাই থেমে গেছে।
# { $error } is what went wrong, as Windows says it.
installer-locate-failed = ইনস্টলারটি খুঁজে পাওয়া যায়নি: { $error }
# { $error } is what went wrong reading it.
installer-read-failed = ইনস্টলারটি পড়া যায়নি: { $error }
# { $error } is why, such as that the app needs a newer Windows.
installer-cannot-be-used-here = এই ইনস্টলারের প্যাকেজ এই পিসিতে ব্যবহার করা যাবে না: { $error }

## The app, where it goes, and whether the installer is signed

installer-captured-from = যেখান থেকে সংগ্রহ করা হয়েছে
installer-install-location = ইনস্টলের অবস্থান
# The title of the folder picker.
installer-choose-location = ইনস্টলের অবস্থান বেছে নিন
installer-folder-has-files = এই ফোল্ডারে আগে থেকেই ফাইল আছে। একই নামের ফাইলগুলো প্রতিস্থাপিত হবে, এবং আপনি আনইনস্টল করলে সেগুলো ফিরে আসবে।
installer-room-needed = প্রয়োজনীয় জায়গা
# { $shortfall } says which drive is short of room, such as "C: needs up to 824.0 MB for the app's files, and has 300.0 MB free".
installer-room-short = এখানে হয়তো যথেষ্ট জায়গা নেই: { $shortfall }। প্যাকেজটি অন্য ড্রাইভে আনপ্যাক হতে পারে, কিন্তু অ্যাপের ফাইলগুলোর জন্য এখানে জায়গা থাকতেই হবে।
# { $app_name } is the name of an app whose earlier install was cut short.
installer-interrupted = { $app_name }-এর একটি আগের ইনস্টল কখনো শেষ হয়নি। ইনস্টল করলে প্রথমে সেটি পূর্বাবস্থায় ফেরানো হবে।
# Beside "Installer", for whether it proves who made it.
installer-signature = ইনস্টলার
installer-signature-checking = এর স্বাক্ষর যাচাই করা হচ্ছে…
installer-signature-unsigned = স্বাক্ষরিত নয়: প্রকাশক অজানা
# { $publisher } is the name on the installer's certificate, such as "Fabrikam, Inc.".
installer-signature-signed-by = { $publisher } দ্বারা স্বাক্ষরিত
installer-signature-unnamed = নামহীন একজন প্রকাশক দ্বারা স্বাক্ষরিত
# { $reason } is Windows' own word for what is wrong, such as HashMismatch.
installer-signature-invalid = এই ইনস্টলারের স্বাক্ষর বৈধ নয় ({ $reason })। স্বাক্ষর করার পরে এটি হয়তো পরিবর্তন করা হয়েছে।

## Why the install location cannot be used. Install and Preview Changes stay off while one of these is shown.

# { $app_name } is the name of the app being installed.
installer-location-app-already-there = { $app_name } এই ফোল্ডারে আগে থেকেই ইনস্টল করা আছে, তাই এই অনুলিপিটি এখানে যেতে পারবে না। এখানে ইনস্টল করতে, প্রথমে Windows সেটিংসের অ্যাপস থেকে এই পিসির অনুলিপিটি আনইনস্টল করুন, তারপর এই ইনস্টলারটি আবার খুলুন। অথবা উপরে একটি নতুন, খালি ফোল্ডার লিখুন।
# { $other_app } is the name of another app Windows lists as installed in the folder. { $app_name } is the name of the app being installed.
installer-location-other-app-there = এই ফোল্ডারে { $other_app } ইনস্টল করা আছে, তাই { $app_name } এখানে যেতে পারবে না। উপরে একটি নতুন, খালি ফোল্ডার লিখুন। অথবা, { $other_app } যদি আপনার আর প্রয়োজন না হয়, তবে প্রথমে Windows সেটিংসের অ্যাপস থেকে সেটি আনইনস্টল করুন, তারপর এই ইনস্টলারটি আবার খুলুন।
installer-location-has-files = এই ফোল্ডারে আগে থেকেই এমন ফাইল আছে যা WinAppMigrator ইনস্টল করেনি। সেগুলো অন্য কোনো অ্যাপের হতে পারে, তাই সেগুলোর উপরে কিছুই ইনস্টল করা হবে না। উপরে একটি নতুন, খালি ফোল্ডার লিখুন।
# { $reason } is why, such as that it cannot be the Windows folder.
installer-location-refused = এই ফোল্ডারটি ব্যবহার করা যাবে না: { $reason }। উপরে অন্য একটি ফোল্ডার লিখুন।

## What the install sets up beyond its files

installer-beyond-files = ফাইলের বাইরে এটি যা সেট আপ করে
# Under the heading above. It is part of what a person agrees to when they click Install.
installer-beyond-files-agreed = এই সবকিছু প্রশাসকের অধিকার নিয়ে করা হয়। ইনস্টল ব্যর্থ হলে, এটি যা পরিবর্তন করেছে তা আগের মতো ফিরিয়ে আনা হয়, এবং যা ফিরিয়ে আনা যায়নি তা আপনাকে জানানো হয়। পরে সেটিংসের ইনস্টল করা অ্যাপস থেকেও আপনি ইনস্টলটি পূর্বাবস্থায় ফেরাতে পারবেন। এটি যে ফাইলগুলো প্রতিস্থাপন করে সেগুলো ফিরে আসে, আর এটি যে রানটাইমগুলো ইনস্টল করে সেগুলো থেকে যায়।
installer-beyond-files-none = কিছুই না: এটি কেবল ফাইল অনুলিপি করে, এবং অ্যাপের নিজস্ব রেজিস্ট্রি কীগুলোতে সেটিংস লেখে।
# { $problem } is why, such as that the app cannot run on this PC.
installer-cannot-go-here = এই প্যাকেজটি এই অবস্থানে যেতে পারবে না: { $problem }
installer-working-out = এই অবস্থানে এটি কী কী সেট আপ করবে তা নির্ণয় করা হচ্ছে…
# Said as the reason after "This package cannot go to this location:".
installer-list-stopped = এটি কী করবে তা নির্ণয় করা মাঝপথে থেমে গেছে
# A heading over a list of what else the package asked for and will not get. { $count } is how many.
installer-more-left-out =
    { $count ->
        [one] আরও { $count }টি বাদ দেওয়া হয়েছে
       *[other] আরও { $count }টি বাদ দেওয়া হয়েছে
    }

## Options

installer-options = বিকল্পগুলো
installer-user-data = ব্যবহারকারীর ডেটা
installer-user-data-detail = চালু: সেটিংস, নথিগুলো ও সংরক্ষিত গেমগুলো যিনি সাইন ইন করে আছেন তাঁর সাধারণ ফোল্ডারগুলোতে যায়। বন্ধ: এর সবকিছু ইনস্টল ফোল্ডারের ভিতরে যায়, এবং সেখান থেকে কোনো কিছুই চালানোর জন্য সেট আপ করা হয় না।
# { $account } is the name of the account on the other PC whose data this is.
installer-user-data-whose = এতে { $account }-এর ডেটা আছে।
installer-license = লাইসেন্স
installer-license-detail = অ্যাক্টিভেশন ও লাইসেন্সের ফাইল, এবং যেসব রেজিস্ট্রি মানে লাইসেন্স রাখা থাকে। কিছু ক্ষেত্রে এখানে আবার অ্যাক্টিভেট করতে হতে পারে।

## Dependencies: what else the app needs, and how each is provided

installer-dependencies = নির্ভরতা
installer-dependencies-detail = এই অ্যাপের কী কী প্রয়োজন, এবং প্রতিটি কীভাবে দেওয়া হয়।
installer-dependencies-none = অতিরিক্ত কোনো রানটাইম নির্ভরতা পাওয়া যায়নি।
# { $feature } is the name of an optional part of Windows, such as NetFx3.
installer-dependency-windows-feature = Windows বৈশিষ্ট্য: { $feature }
installer-dependency-included = অন্তর্ভুক্ত
# { $id } is the package's name in winget, such as Microsoft.VCRedist.2015+.x64.
installer-dependency-winget = winget দিয়ে ইনস্টল করা হয়: { $id }
installer-dependency-manual = নিজে ইনস্টল করুন
installer-dependency-built-in = Windows-এ অন্তর্নির্মিত
installer-dependency-ships-with-app = অ্যাপের সঙ্গে আসে

## Buttons, and what is said beside them

installer-install = ইনস্টল করুন
installer-preview = পরিবর্তনের পূর্বরূপ দেখুন
installer-cancel = বাতিল করুন
installer-close = বন্ধ করুন
installer-open-folder = ইনস্টল ফোল্ডার খুলুন
# { $error } is what went wrong, as Windows says it.
installer-open-folder-failed = ফোল্ডারটি খোলা যায়নি: { $error }
installer-location-blocks = ইনস্টলের অবস্থান ব্যবহারযোগ্য না হওয়া পর্যন্ত “ইনস্টল করুন” ও “পরিবর্তনের পূর্বরূপ দেখুন” বন্ধ থাকবে। কারণটি উপরে, ইনস্টলের অবস্থানের নিচে দেখানো আছে।
installer-package-blocks = “ইনস্টল করুন” বন্ধ আছে, কারণ এই প্যাকেজটি এই অবস্থানে যেতে পারবে না। কারণটি উপরে দেখানো আছে।
installer-list-changed = এখানে এটি যা করবে তা বদলে গেছে, তাই কিছুই ইনস্টল করা হয়নি। নিচের তালিকাটি দেখে নিন, এবং আপনি এটিই চাইলে আবার “ইনস্টল করুন” ক্লিক করুন।
installer-cancelled-early = বাতিল করা হয়েছে। কিছুই পরিবর্তন করা হয়নি।

## How far an install has got, beside its progress bar

# { $step } is one of the steps below. { $done } and { $total } are how many of its items are done, and how many there are.
installer-step-counted = { $step }: { $total }টির মধ্যে { $done }টি…
# { $step } is one of the steps below, when it does not count its items.
installer-step-uncounted = { $step }…
installer-step-getting-ready = প্রস্তুত হচ্ছে…
installer-step-cancelling = বর্তমান ধাপের পরে বাতিল করা হচ্ছে…
installer-step-putting-back = পরিবর্তনগুলো আগের মতো ফিরিয়ে আনা হচ্ছে
installer-step-unpacking = আনপ্যাক করা হচ্ছে
installer-step-checking-package = প্যাকেজ যাচাই করা হচ্ছে
installer-step-recovering = শেষ না হওয়া একটি আগের ইনস্টল পূর্বাবস্থায় ফেরানো হচ্ছে
# What a preview is doing, whatever step it is on.
installer-step-previewing = কী কী পরিবর্তন হবে তা নির্ণয় করা হচ্ছে
installer-step-runtimes = রানটাইম ইনস্টল করা হচ্ছে
installer-step-files = ফাইল অনুলিপি করা হচ্ছে
installer-step-services = সেবা সেট আপ করা হচ্ছে
installer-step-drivers = ড্রাইভার সেট আপ করা হচ্ছে
installer-step-registry = রেজিস্ট্রি সেটিংস লেখা হচ্ছে
installer-step-license = লাইসেন্স ডেটা পুনরুদ্ধার করা হচ্ছে
installer-step-tasks = নির্ধারিত কার্য সেট আপ করা হচ্ছে
installer-step-shortcuts = শর্টকাট তৈরি করা হচ্ছে
installer-step-environment = এনভায়রনমেন্ট ভেরিয়েবল সেট করা হচ্ছে
installer-step-settings-entry = সেটিংসে অ্যাপটি যোগ করা হচ্ছে
installer-step-verifying = যা ইনস্টল হয়েছে তা যাচাই করা হচ্ছে
installer-step-undoing = পূর্বাবস্থায় ফেরানো হচ্ছে

## When an install cannot go ahead or stops

installer-carries-no-package = এই ইনস্টলারে কোনো প্যাকেজ নেই।
installer-stopped-unexpectedly = ইনস্টল অপ্রত্যাশিতভাবে থেমে গেছে।
# { $reason } is what went wrong, as the program says it.
installer-stopped-unexpectedly-because = ইনস্টল অপ্রত্যাশিতভাবে থেমে গেছে: { $reason }। এটি শুরু হয়ে থাকলে, আবার কোনো ইনস্টলার চালালে সেটি প্রথমে এর করা কাজ পূর্বাবস্থায় ফেরাবে।
# { $error } is what went wrong reading it.
installer-package-read-failed = প্যাকেজটি পড়া যায়নি: { $error }
installer-no-programdata = এই পিসির ProgramData ফোল্ডার পাওয়া যায়নি, তাই কিছুই ইনস্টল করা হয়নি: যে রেকর্ড দিয়ে একটি ইনস্টল পূর্বাবস্থায় ফেরানো হয়, তা সেখানেই রাখা হয়।
# { $error } is why, such as that there is nowhere only administrators can get into.
installer-staging-failed = একটি স্টেজিং ফোল্ডার তৈরি করা যায়নি: { $error }
# { $error } is what went wrong unpacking it.
installer-unpack-failed = প্যাকেজটি আনপ্যাক করা যায়নি: { $error }
installer-staging-nowhere = প্যাকেজটি আনপ্যাক করার এমন কোনো জায়গা নেই, যেখানে কেবল প্রশাসকেরা প্রবেশ করতে পারেন
# { $failure } is a folder and what went wrong there, such as "C:\ProgramData\WinAppMigrator: Access is denied.".
installer-staging-nowhere-look-at = প্যাকেজটি আনপ্যাক করার এমন কোনো জায়গা নেই, যেখানে কেবল প্রশাসকেরা প্রবেশ করতে পারেন। যেটি পরীক্ষা করে দেখতে হবে: { $failure }

## What came of an install, or of a preview

installer-result = ইনস্টলের ফলাফল
installer-result-preview = পূর্বরূপ: এখনো কিছুই পরিবর্তন করা হয়নি
installer-result-installed = অ্যাপ্লিকেশনটি ইনস্টল করা হয়েছে এবং এর হ্যাশ যাচাইয়ে উত্তীর্ণ হয়েছে।
installer-result-installed-without-left-out = নিচে বাদ দেওয়া হিসেবে তালিকাভুক্ত জিনিসগুলো ছাড়া অ্যাপ্লিকেশনটি ইনস্টল করা হয়েছে, এবং এর হ্যাশ যাচাইয়ে উত্তীর্ণ হয়েছে।
installer-result-not-all-put-back = ইনস্টল ব্যর্থ হয়েছে, এবং এটি যা পরিবর্তন করেছিল তার সবকিছু আগের মতো ফিরিয়ে আনা যায়নি। যা রয়ে গেছে তা নিচে তালিকাভুক্ত, এবং পরের ইনস্টল আবার চেষ্টা করবে।
installer-result-cancelled-put-back = ইনস্টল বাতিল করা হয়েছে, এবং এটি যা পরিবর্তন করেছিল তা আগের মতো ফিরিয়ে আনা হয়েছে।
installer-result-failed-put-back = ইনস্টল ব্যর্থ হয়েছে, এবং এটি যা পরিবর্তন করেছিল তা আগের মতো ফিরিয়ে আনা হয়েছে।
installer-result-not-clean = ইনস্টল সুষ্ঠুভাবে শেষ হয়নি। নিচের সমস্যাগুলো দেখে নিন।
installer-result-changes-put-back = আগের মতো ফিরিয়ে আনা পরিবর্তনগুলো
installer-result-changes-made = করা পরিবর্তনগুলো
installer-result-changes-planned = পরিকল্পিত পরিবর্তনগুলো
installer-result-warnings = সতর্কতা
installer-result-errors = ত্রুটি
installer-result-package-valid = প্যাকেজের হ্যাশ বৈধ
installer-result-package-invalid = প্যাকেজের হ্যাশ অবৈধ
installer-result-files-valid = ইনস্টল করা ফাইল বৈধ
installer-result-files-invalid = ইনস্টল করা ফাইল অবৈধ
installer-result-reboot = একটি নির্ভরতার কাজ শেষ করতে পুনরায় চালু করা প্রয়োজন। Windows পুনরায় চালু করুন, তারপর অ্যাপটি চালু করুন।
# A heading over the list of what was changed. { $count } is how many.
installer-result-changes = পরিবর্তন ({ $count })
# After the first part of a long list. { $count } is how many more there are.
installer-result-changes-more = এবং আরও { $count }টি।
# Over the path of the install's log file.
installer-log = লগ

## The installer's command line. Shown in English: scripts read it. A { $count } is plain digits, with no separators.

installer-cli-stub = WinAppMigrator ইনস্টলার স্টাব — এতে কোনো অ্যাপ্লিকেশন প্যাকেজ নেই।
installer-cli-stub-how = WinAppMigrator অ্যাপ দিয়ে একটি ইনস্টলার তৈরি করুন, তারপর তৈরি হওয়া .exe ফাইলটি লক্ষ্য মেশিনে চালান।
installer-cli-usage-title = WinAppMigrator সেলফ-এক্সট্র্যাক্টিং ইনস্টলার।
installer-cli-usage = গ্রাফিক্যাল ইনস্টলার খুলতে কোনো আর্গুমেন্ট ছাড়া এটি চালান, অথবা:
installer-cli-usage-preview = পুনরায় ইনস্টলের পূর্বরূপ: --dry-run [--target-root <DIR>]
installer-cli-usage-apply = পুনরায় ইনস্টল:            --apply [--target-root <DIR>]
installer-cli-no-package = ইনস্টলারে কোনো এমবেড করা প্যাকেজ নেই
installer-cli-no-programdata = এই পিসির ProgramData ফোল্ডার পাওয়া যায়নি, তাই কিছুই ইনস্টল করা হয়নি: যে রেকর্ড দিয়ে একটি ইনস্টল পূর্বাবস্থায় ফেরানো হয়, তা সেখানেই রাখা হয়
# { $path } is the folder asked for, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-folder-has-files = { $path } ব্যবহার করা যাবে না: { $problem }। কেবল যদি নিশ্চিত হন যে এটি এই অ্যাপেরই, তবেই --force দিন
installer-cli-no-location = এই প্যাকেজে বলা নেই এর অ্যাপটি কোথায় ছিল, তাই একটি --target-root দিন
# { $path } is where the app was on the other PC, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-location-refused = এই প্যাকেজের অ্যাপটি { $path }-এ ছিল, এবং সেটি ব্যবহার করা যাবে না: { $problem }। একটি --target-root দিন
# { $summary } is how much room it needs where, such as "up to 824.0 MB on C: (356.0 GB free)".
installer-cli-room = জায়গা: { $summary }
# { $note } is something to know about the install.
installer-cli-note = দ্রষ্টব্য: { $note }
# { $count } is how many things it sets up beyond its files.
installer-cli-sets-up = সেট আপ করে: { $count }
installer-cli-applied = পুনরায় ইনস্টল সম্পন্ন হয়েছে।
installer-cli-previewed = পুনরায় ইনস্টলের পূর্বরূপ দেখা হয়েছে।
# { $count } is how many changes the install logged.
installer-cli-changes-logged = লগ করা পরিবর্তন: { $count }
# { $count } is how many warnings.
installer-cli-warnings = সতর্কতা: { $count }
# { $count } is how many errors.
installer-cli-errors = ত্রুটি: { $count }
# { $count } is how many things the package asked for and did not get.
installer-cli-refused = প্রত্যাখ্যাত: { $count }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-package-valid = প্যাকেজের হ্যাশ বৈধ: { $valid }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-reinstall-valid = পুনরায় ইনস্টলের হ্যাশ বৈধ: { $valid }
# { $id } is the install's id, such as wam-0f3a….
installer-cli-install-id = ইনস্টল আইডি: { $id }
installer-cli-put-back = ইনস্টল ব্যর্থ হয়েছে, তাই এটি যা পরিবর্তন করেছিল তা আগের মতো ফিরিয়ে আনা হয়েছে।
# { $path } is the log file.
installer-cli-log = লগ: { $path }
# { $path } is the log file. Printed before an error.
installer-cli-error-log = লগ: { $path }
# { $error } is what went wrong.
installer-cli-error = ত্রুটি: { $error }

## The uninstaller's window

uninstaller-window-title = আনইনস্টল
# The window's title, and its heading. { $app_name } is the name of the app it uninstalls.
uninstaller-window-title-app = { $app_name } আনইনস্টল
uninstaller-tagline = এর ইনস্টলার যা করেছিল তা পূর্বাবস্থায় ফেরায়
uninstaller-reading = এই ইনস্টলের রেকর্ড পড়া হচ্ছে…
uninstaller-stopped-unexpectedly = আনইনস্টল অপ্রত্যাশিতভাবে থেমে গেছে।
uninstaller-what-will-be-removed = যা সরানো হবে
uninstaller-install-location = ইনস্টলের অবস্থান
uninstaller-files = ফাইল
uninstaller-registry-values = রেজিস্ট্রি মান
uninstaller-services = সেবা
uninstaller-drivers = ড্রাইভার
uninstaller-scheduled-tasks = নির্ধারিত কার্য
uninstaller-environment = এনভায়রনমেন্ট ভেরিয়েবল
uninstaller-plan-stays = আপনি যে ফাইলগুলো যোগ বা পরিবর্তন করেছেন সেগুলো, যে ফোল্ডারগুলোতে আছে সেগুলোসহ, থেকে যাবে।
# { $count } is how many files the install replaced.
uninstaller-plan-coming-back =
    { $count ->
        [one] এই ইনস্টল যে { $count }টি ফাইল প্রতিস্থাপন করেছিল সেটি ফিরে আসবে, অথবা ফাইলটি এর মধ্যে পরিবর্তিত হয়ে থাকলে তার পাশে রাখা হবে।
       *[other] এই ইনস্টল যে { $count }টি ফাইল প্রতিস্থাপন করেছিল সেগুলো ফিরে আসবে, অথবা কোনো ফাইল এর মধ্যে পরিবর্তিত হয়ে থাকলে তার পাশে রাখা হবে।
    }
# { $runtimes } is a list of runtimes, such as "Microsoft Visual C++ Redistributable, .NET Desktop Runtime 8".
uninstaller-plan-runtimes = এটি যে রানটাইমগুলো ইনস্টল করেছিল সেগুলো থেকে যাবে, কারণ অন্য প্রোগ্রামও সেগুলো ব্যবহার করতে পারে: { $runtimes }।
uninstaller-options = বিকল্পগুলো
uninstaller-remove-user-data = ব্যবহারকারীর ডেটা ও লাইসেন্স ডেটাও সরান
# { $count } is how many items of the app's settings, data, and license there are.
uninstaller-remove-user-data-detail =
    { $count ->
        [one] এর সেটিংস, ডেটা ও লাইসেন্সের { $count }টি আইটেম। এই বিকল্পটি চালু না করলে এটি থেকে যাবে। এটি যা প্রতিস্থাপন করেছিল তা ফিরে আসবে, অথবা এটি এর মধ্যে পরিবর্তিত হয়ে থাকলে এর পাশে রাখা হবে।
       *[other] এর সেটিংস, ডেটা ও লাইসেন্সের { $count }টি আইটেম। এই বিকল্পটি চালু না করলে এগুলো থেকে যাবে। এগুলো যা প্রতিস্থাপন করেছিল তা ফিরে আসবে, অথবা এগুলো এর মধ্যে পরিবর্তিত হয়ে থাকলে এগুলোর পাশে রাখা হবে।
    }
uninstaller-uninstall = আনইনস্টল করুন
uninstaller-cancel = বাতিল করুন
uninstaller-close = বন্ধ করুন
# { $done } and { $total } are how many of the install's items are undone, and how many there are.
uninstaller-step-counted = ইনস্টলটি পূর্বাবস্থায় ফেরানো হচ্ছে: { $total }টির মধ্যে { $done }টি…
uninstaller-step = ইনস্টলটি পূর্বাবস্থায় ফেরানো হচ্ছে…
uninstaller-done = অ্যাপটি আনইনস্টল করা হয়েছে।
uninstaller-done-except = নিচে তালিকাভুক্ত জিনিসগুলো ছাড়া অ্যাপটি আনইনস্টল করা হয়েছে।
uninstaller-waits-for-drive = এর একটি অংশ এমন একটি ড্রাইভে আছে যা সংযুক্ত নেই। ড্রাইভটি সংযুক্ত করুন, এবং সেটিংস থেকে আবার আনইনস্টল করুন।
uninstaller-not-all-undone = এর সবকিছু পূর্বাবস্থায় ফেরানো যায়নি। যা এটি ব্যবহার করছে তা বন্ধ করুন, এবং সেটিংস থেকে আবার আনইনস্টল করুন।
# Over the path of the uninstall's log file.
uninstaller-log = লগ
uninstaller-undone = পূর্বাবস্থায় ফেরানো হয়েছে
uninstaller-left-in-place = যেমন ছিল তেমন রাখা হয়েছে
uninstaller-could-not-be-undone = পূর্বাবস্থায় ফেরানো যায়নি
# After the first part of a long list. { $count } is how many more there are.
uninstaller-and-more = এবং আরও { $count }টি।
# Said as the reason when there is no record of the install to read.
uninstaller-no-programdata = এই পিসির ProgramData ফোল্ডার পাওয়া যায়নি, তাই পড়ার মতো কোনো রেকর্ড নেই

## When the uninstaller's window cannot open. Each is a paragraph of a message box, followed by a command to type.

uninstaller-window-failed-title = WinAppMigrator আনইনস্টলার
# { $error } is what went wrong, as Windows or the graphics driver says it.
uninstaller-window-failed = আনইনস্টলারের উইন্ডো খোলা যায়নি: { $error }
uninstaller-window-failed-quiet = উইন্ডো ছাড়াও এটি প্রশাসকের কমান্ড প্রম্পট থেকে আনইনস্টল করতে পারে:
# Stands in for this program's own path in that command, if it cannot be found.
uninstaller-this-program = এই প্রোগ্রাম

## The uninstaller's command line (--quiet). Shown in English: scripts read it.

# { $app_name } is the name of the app.
uninstaller-cli-uninstalling = { $app_name } আনইনস্টল করা হচ্ছে।
# { $path } is the log file.
uninstaller-cli-log = লগ: { $path }
# { $error } is what went wrong.
uninstaller-cli-error = ত্রুটি: { $error }
