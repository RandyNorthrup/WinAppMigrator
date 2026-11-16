# Words the core writes for people as it installs an app from a package: why an install will not start, the list of
# what it did (or for a preview, what it would do) that the installer window shows, its warnings, the record it keeps of
# each install so that it can be undone, and the core's errors. Message names start with core-.
#
# The list of what an install did is short and plain, one line for each thing, as the installer window lists them:
# "copied", "restored", "would copy". Words such as --force and --apply are command-line options, and LocalService,
# SYSTEM, ProgramData, and %AppData% are as Windows names them: keep them as they are.


## Errors: what went wrong, when the core cannot go on. Each is shown on its own, or after words that say what was being
## done.

core-error-list-changed = সম্মতি দেওয়ার পরে এই ইনস্টল যা করবে তা বদলে গেছে, তাই কিছুই ইনস্টল করা হয়নি
# { $path } is a file or folder, such as C:\Program Files\App\app.exe. { $error } is what Windows said went wrong.
core-error-io = { $path }-এ I/O ব্যর্থ হয়েছে: { $error }
# { $detail } is what went wrong.
core-error-registry = রেজিস্ট্রি পড়া ব্যর্থ হয়েছে: { $detail }
# { $path } is a program file. { $detail } is what was wrong with it.
core-error-pe = { $path }-এর PE পার্স করা ব্যর্থ হয়েছে: { $detail }
# { $detail } is what was wrong, already in words.
core-error-invalid-input = অবৈধ ইনপুট: { $detail }
# { $detail } is what went wrong.
core-error-serialization = রিপোর্ট সিরিয়ালাইজ করা ব্যর্থ হয়েছে: { $detail }
# { $detail } is what went wrong.
core-error-archive = আর্কাইভের কাজ ব্যর্থ হয়েছে: { $detail }
# { $detail } is what went wrong.
core-error-network = নেটওয়ার্কের কাজ ব্যর্থ হয়েছে: { $detail }
# { $detail } is what went wrong.
core-error-time-format = সময় ফরম্যাট করা ব্যর্থ হয়েছে: { $detail }
core-error-cancelled = এটি বাতিল করা হয়েছে
# What is said when part of the program stopped unexpectedly and gave no reason.
core-error-panic-no-reason = কোনো কারণ জানানো হয়নি

## The record of each install, kept so that it can be undone.

# { $kind } is a number that says what type of registry value it is.
core-journal-unknown-value-type = অজানা রেজিস্ট্রি মানের ধরন { $kind }
core-journal-not-elevated = প্রশাসকের অধিকার নিয়ে চালানো হয়নি
core-journal-needs-admin = ইনস্টল করতে প্রশাসকের অধিকার প্রয়োজন, যাতে এর রেকর্ড এমন জায়গায় রাখা যায় যেখানে অন্যরা তা পরিবর্তন করতে পারে না
# { $path } is a folder.
core-journal-not-an-install-folder = এটি কোনো ইনস্টলের ফোল্ডার নয়: { $path }
# { $path } is the file the record is kept in.
core-journal-not-written = এই ইনস্টলের রেকর্ড লেখা যায়নি: { $path }
# { $path } is a folder.
core-journal-path-escapes-root = পাথটি নির্ধারিত রুটের বাইরে চলে যায়: { $path }
# { $path } is a folder.
core-journal-nothing-exists = এর কোনোটিই এই পিসিতে নেই: { $path }
core-journal-busy = আরেকটি ইনস্টল বা আনইনস্টল চলছে। সেটি শেষ হওয়া পর্যন্ত অপেক্ষা করুন, তারপর আবার চেষ্টা করুন
# Said of a file that was to get back the permissions it had.
core-journal-no-folder = এর কোনো ফোল্ডার নেই
# Said of the permissions a file had, which were kept to give back to it.
core-journal-permissions-unreadable = যা রাখা হয়েছিল তা পড়া যায়নি
# { $folder } is the folder of one install's record.
core-journal-unreadable-empty = { $folder }-এ থাকা ইনস্টলের রেকর্ড ব্যবহার করা যাবে না: এটি খালি
# { $folder } is the folder of one install's record.
core-journal-unreadable-no-header = { $folder }-এ থাকা ইনস্টলের রেকর্ড ব্যবহার করা যাবে না: এর শুরুতে বলা নেই এটি কী নিয়ে
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-version = { $folder }-এ থাকা ইনস্টলের রেকর্ড ব্যবহার করা যাবে না: এই প্রোগ্রামের অন্য একটি সংস্করণ এটি লিখেছে
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-install = { $folder }-এ থাকা ইনস্টলের রেকর্ড ব্যবহার করা যাবে না: এতে বলা আছে এটি অন্য একটি ইনস্টলের
# { $id } is an install's id, such as wam-1a2b3c.
core-journal-not-an-install-id = এটি কোনো ইনস্টলের আইডি নয়: { $id }

## Why an install will not start, or stopped before it changed anything.

core-restore-apply-and-dry-run = --apply ও --dry-run দুটি একসঙ্গে দেওয়া যাবে না
# { $path } is the folder that was chosen to install in.
core-restore-location-not-full-path = ইনস্টলের অবস্থান অবশ্যই একটি সম্পূর্ণ পাথ হতে হবে: “{ $path }”
# { $problem } says what is wrong with the folder, already in words. { $path } is the folder.
core-restore-location-unusable = এই ইনস্টলের অবস্থান ব্যবহার করা যাবে না: { $problem }: “{ $path }”
core-restore-needs-admin-plan = পুনরুদ্ধারের পরিকল্পনায় রেজিস্ট্রি, সেবা, নির্ধারিত কার্য, শর্টকাট বা সুরক্ষিত লক্ষ্যের এমন পরিবর্তন আছে যা কেবল প্রশাসক করতে পারেন
# { $path } is the folder that was chosen to install in.
core-restore-needs-admin-target = লক্ষ্য রুটের জন্য প্রশাসকের অধিকার প্রয়োজন: { $path }
core-restore-needs-admin-user-data = ProgramData বা অন্য কোনো সুরক্ষিত অবস্থানে ব্যবহারকারীর ডেটা পুনরুদ্ধার করতে প্রশাসকের অধিকার প্রয়োজন
# { $error } is why the install stopped. { $failed } is what could not be put back, as sentences of their own.
core-restore-not-all-put-back = { $error }। যা করা হয়েছিল তার সবকিছু আগের মতো ফিরিয়ে আনা যায়নি: { $failed }
# { $error } is what went wrong.
core-restore-nowhere-to-keep-record = এই ইনস্টল পূর্বাবস্থায় ফেরানোর জন্য প্রয়োজনীয় রেকর্ড রাখার মতো কোনো নিরাপদ জায়গা নেই, তাই কিছুই ইনস্টল করা হয়নি: { $error }
# { $path } is a file the app would have put there.
core-restore-already-exists = { $path } আগে থেকেই আছে, তাই কিছুই ইনস্টল করা হয়নি। এটি প্রতিস্থাপন করতে --force ব্যবহার করুন
# { $root } is a folder the app has files for. { $link } is that folder, or one it is in.
core-restore-root-under-link = এই অ্যাপের কিছু জিনিস { $root }-এ পুনরুদ্ধার করার আছে, এবং { $link } অন্য কোথাও যাওয়ার একটি লিঙ্ক, যা অন্যরা হয়তো পরিবর্তন করতে পারে, তাই কিছুই ইনস্টল করা হয়নি। সেখানে যা যায় তা ব্যবহারকারীর ডেটা বা লাইসেন্স ডেটা হলে, এটি ব্যবহারকারীর ডেটা ও লাইসেন্স ডেটা ছাড়া ইনস্টল করুন। সেটি কোনো স্টার্টআপ ফোল্ডারের শর্টকাট হলে, সেই ফোল্ডারটি আগে যেখানে ছিল সেখানে ফিরিয়ে আনুন
core-restore-hash-check-failed = প্যাকেজটি হ্যাশ যাচাইয়ে ব্যর্থ হয়েছে, এবং কিছুই ইনস্টল করা হয়নি
# { $detail } is the first thing found wrong, already in words.
core-restore-hash-check-failed-because = প্যাকেজটি হ্যাশ যাচাইয়ে ব্যর্থ হয়েছে, এবং কিছুই ইনস্টল করা হয়নি: { $detail }
# { $path } is a file or folder.
core-restore-refuse-records-folder = ইনস্টলগুলো যেখানে তাদের রেকর্ড রাখে, সেখানে লিখতে অস্বীকার করা হয়েছে: { $path }
# { $path } is a file or folder.
core-restore-refuse-outside = ইনস্টলের অবস্থান ও পরিচিত ডেটা ফোল্ডারগুলোর বাইরে লিখতে অস্বীকার করা হয়েছে: { $path }
# { $path } is a file.
core-restore-no-parent-folder = গন্তব্যের কোনো প্যারেন্ট ফোল্ডার নেই: { $path }
# { $path } is where a file was to go.
core-restore-folder-in-the-way = এই ফাইলটি যেখানে যাওয়ার কথা, সেখানে একটি ফোল্ডার আছে: { $path }
# { $path } is a file in the package.
core-restore-file-changed = { $path } আর প্যাকেজে থাকা ফাইলের মতো নেই, তাই এটি ইনস্টল করা হয়নি
# { $path } is a file in the package.
core-restore-source-is-link = { $path } একটি লিঙ্ক, এবং এটি অনুসরণ করা হবে না
# { $path } is a file in the package.
core-restore-source-became-link = খোলার সময় { $path } একটি লিঙ্কে পরিণত হয়েছে
# Why a service did not start, after "did not start:".
core-restore-service-stopped-by-itself = এটি নিজে থেকেই আবার থেমে গেছে
# Why part of the app was left out, after "was left out:".
core-restore-com-class-someone-elses = এই COM ক্লাসটি এই পিসিতে আগে থেকেই আছে, অন্য কিছুর অংশ হিসেবে

## What an install did, one line for each thing, or for a preview, what it would do. { $source } is a file in the
## package, and { $destination } and { $path } are where it goes on this PC. { $key } is a registry key, such as
## HKLM\Software\App.

# { $package_valid } is true or false, and { $restore_valid } is true, false, or not_run: keep them as they are.
core-restore-verification-result = পুনরুদ্ধারের পরের যাচাই: package_valid={ $package_valid } restore_valid={ $restore_valid }
# { $name } is a service's name.
core-restore-started-service = সেবা চালু করা হয়েছে: { $name }
core-restore-would-copy = অনুলিপি করা হবে: { $source } -> { $destination }
core-restore-copied = অনুলিপি করা হয়েছে: { $source } -> { $destination }
core-restore-would-create-empty-folder = খালি ফোল্ডার তৈরি করা হবে: { $path }
core-restore-created-empty-folder = খালি ফোল্ডার তৈরি করা হয়েছে: { $path }
# { $value } is the name of a value in the key.
core-restore-would-restore-registry-value = রেজিস্ট্রি পুনরুদ্ধার করা হবে: { $key } { $value }
core-restore-would-restore-registry-key = রেজিস্ট্রি পুনরুদ্ধার করা হবে: { $key } (কী)
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-would-add-firewall-rule = ফায়ারওয়াল নিয়ম যোগ করা হবে: { $name }, { $program }-এর জন্য
# { $name } is a firewall rule's name.
core-restore-firewall-rule-already-there = ফায়ারওয়াল নিয়ম আগে থেকেই আছে: { $name }
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-added-firewall-rule = ফায়ারওয়াল নিয়ম যোগ করা হয়েছে: { $name }, { $program }-এর জন্য
# { $name } is a service's name, { $command } what it runs, and { $account } the account it runs as, such as LocalService.
core-restore-would-restore-service = সেবা পুনরুদ্ধার করা হবে: { $name } -> { $command } ({ $account } হিসেবে চলে)
# { $name } is a runtime, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-restore-prerequisite-present = পূর্বশর্ত রানটাইম আগে থেকেই আছে: { $name }
# { $name } is a runtime that Windows has as a Windows feature.
core-restore-would-install-prerequisite-feature = পূর্বশর্ত রানটাইম ইনস্টল করা হবে: { $name } (Windows বৈশিষ্ট্য)
# { $name } is a runtime whose installer is in the package.
core-restore-would-install-prerequisite-bundled = পূর্বশর্ত রানটাইম ইনস্টল করা হবে: { $name } (সঙ্গে দেওয়া ইনস্টলার)
# { $name } is a runtime that would come through winget, which is a program's name.
core-restore-would-install-prerequisite-winget = পূর্বশর্ত রানটাইম ইনস্টল করা হবে: { $name } (winget)
# { $name } is a runtime.
core-restore-installed-prerequisite = পূর্বশর্ত রানটাইম ইনস্টল করা হয়েছে: { $name }
# { $name } is a runtime.
core-restore-installed-prerequisite-reboot = পূর্বশর্ত রানটাইম ইনস্টল করা হয়েছে: { $name } (পুনরায় চালু করা প্রয়োজন)
core-restore-would-restore-license = লাইসেন্স পুনরুদ্ধার করা হবে: { $source } -> { $destination }
core-restore-restored-license = লাইসেন্স পুনরুদ্ধার করা হয়েছে: { $source } -> { $destination }
core-restore-would-restore-license-registry = লাইসেন্সের রেজিস্ট্রি পুনরুদ্ধার করা হবে: { $key }
# { $name } is a driver's name.
core-restore-driver-not-restored = ড্রাইভার স্বয়ংক্রিয়ভাবে পুনরুদ্ধার করা হয়নি: { $name }
# { $name } is a driver's name.
core-restore-would-install-driver = ড্রাইভার ইনস্টল করা হবে: { $name } -> { $destination }
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-would-restore-task = নির্ধারিত কার্য পুনরুদ্ধার করা হবে: { $name }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-startup-shortcut = স্টার্টআপ শর্টকাট পুনরুদ্ধার করা হবে: { $shortcut } -> { $target }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-shortcut = শর্টকাট পুনরুদ্ধার করা হবে: { $shortcut } -> { $target }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-would-restore-variable =
    { $scope ->
        [user] ব্যবহারকারীর এনভায়রনমেন্ট ভেরিয়েবল পুনরুদ্ধার করা হবে: { $name }
       *[machine] মেশিনের এনভায়রনমেন্ট ভেরিয়েবল পুনরুদ্ধার করা হবে: { $name }
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-left-variable =
    { $scope ->
        [user] ব্যবহারকারীর এনভায়রনমেন্ট ভেরিয়েবল { $name } যেমন ছিল তেমন রাখা হয়েছে: এর কোনো অংশই এই অ্যাপের নয়
       *[machine] মেশিনের এনভায়রনমেন্ট ভেরিয়েবল { $name } যেমন ছিল তেমন রাখা হয়েছে: এর কোনো অংশই এই অ্যাপের নয়
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-restored-variable =
    { $scope ->
        [user] ব্যবহারকারীর এনভায়রনমেন্ট ভেরিয়েবল পুনরুদ্ধার করা হয়েছে: { $name }
       *[machine] মেশিনের এনভায়রনমেন্ট ভেরিয়েবল পুনরুদ্ধার করা হয়েছে: { $name }
    }
core-restore-set-uninstaller = আনইনস্টলার সেট করা হয়েছে: { $key }
core-restore-placed-driver-image = ড্রাইভার ইমেজ রাখা হয়েছে: { $source } -> { $destination }
# { $name } is a driver's name.
core-restore-configured-driver = ড্রাইভার কনফিগার করা হয়েছে: { $name }
# { $name } is a driver's name.
core-restore-created-driver = ড্রাইভার তৈরি করা হয়েছে: { $name }
core-restore-ensured-registry-key = রেজিস্ট্রি কী আছে তা নিশ্চিত করা হয়েছে: { $key }
# { $value } is the name of a value in the key.
core-restore-restored-registry-value = রেজিস্ট্রি পুনরুদ্ধার করা হয়েছে: { $key } { $value }
# "(Default)" is how Windows names the value a key has with no name: write it as Registry Editor does.
core-restore-restored-registry-default = রেজিস্ট্রি পুনরুদ্ধার করা হয়েছে: { $key } (ডিফল্ট)
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-configured-service = সেবা কনফিগার করা হয়েছে: { $name } ({ $account } হিসেবে চলে)
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-created-service = সেবা তৈরি করা হয়েছে: { $name } ({ $account } হিসেবে চলে)
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-restored-task = নির্ধারিত কার্য পুনরুদ্ধার করা হয়েছে: { $name }
core-restore-restored-shortcut = শর্টকাট পুনরুদ্ধার করা হয়েছে: { $path }

## Warnings an install gives, and why it failed. They are shown after it, on the PC it ran on.

core-warning-restore-earlier-not-trusted = শেষ না হওয়া একটি ইনস্টলের রেকর্ড যেমন ছিল তেমন রাখা হয়েছে, কারণ অন্যরা সেটি পরিবর্তন করতে পারে। সেই ইনস্টল যা পরিবর্তন করেছিল তা এখনো রয়ে গেছে।
# { $app_name } is the app's name. { $count } is how many changes were put back.
core-warning-restore-earlier-undone =
    { $count ->
        [one] { $app_name }-এর একটি আগের ইনস্টল কখনো শেষ হয়নি, তাই প্রথমে সেটি পূর্বাবস্থায় ফেরানো হয়েছে। { $count }টি পরিবর্তন আগের মতো ফিরিয়ে আনা হয়েছে।
       *[other] { $app_name }-এর একটি আগের ইনস্টল কখনো শেষ হয়নি, তাই প্রথমে সেটি পূর্বাবস্থায় ফেরানো হয়েছে। { $count }টি পরিবর্তন আগের মতো ফিরিয়ে আনা হয়েছে।
    }
# { $app_name } is the app's name.
core-warning-restore-earlier-never-finished = { $app_name }-এর একটি আগের ইনস্টল কখনো শেষ হয়নি। ইনস্টল করলে প্রথমে সেটি পূর্বাবস্থায় ফেরানো হবে।
core-warning-restore-user-data-real-locations = ব্যবহারকারীর ডেটা লক্ষ্য রুটের বদলে এই মেশিনের আসল %AppData%/%ProgramData% অবস্থানে পুনরুদ্ধার করা হয়।
# { $error } is what went wrong.
core-warning-restore-commit-failed = এই ইনস্টলের রেকর্ড সম্পূর্ণ করা যায়নি: { $error }
# { $name } is a service's name.
core-warning-restore-service-still-starting = { $name } সেবাটিকে চালু হতে বলা হয়েছে, কিন্তু এটি এখনো চালু হওয়া শেষ করেনি। হয়তো একটু পরে হবে। এই পিসি পরের বার চালু হলেও এটি নিজে থেকে চালু হয়।
# { $name } is a service's name. { $reason } is why it did not start, as Windows said it.
core-warning-restore-service-not-started = { $name } সেবাটি ইনস্টল করা হয়েছে, কিন্তু চালু হয়নি: { $reason }। এটি চালু না হওয়া পর্যন্ত অ্যাপটি হয়তো কাজ করবে না। এই পিসি পরের বার চালু হলে Windows আবার চেষ্টা করবে।
# { $name } is a service's name. { $reason } is why it could not be stopped.
core-warning-restore-service-not-stopped = { $name } সেবাটি প্রথমে থামানো যায়নি: { $reason }। এই পিসি পরের বার চালু না হওয়া পর্যন্ত এটি হয়তো পুরোনো প্রোগ্রামটিই চালাতে থাকবে।
core-warning-restore-install-cancelled = ইনস্টল বাতিল করা হয়েছে।
core-warning-restore-preview-cancelled = পূর্বরূপ বাতিল করা হয়েছে।
# { $item } is what was left out, such as a file or a service's name. { $reason } is why, already in words.
core-warning-restore-left-out = { $item } বাদ দেওয়া হয়েছে: { $reason }।
# { $error } is what went wrong.
core-warning-restore-verification-failed = পুনরুদ্ধারের পরের যাচাই চালানো যায়নি: { $error }
# { $runner } says what relies on the file: a service, a scheduled task, a firewall rule, a shortcut, or a registry value.
# { $path } is the file. { $name } is the name of what relies on it. { $refusal } is why the file was not replaced,
# already in words.
core-warning-restore-program-would-run =
    { $runner ->
        [service] { $path } প্রতিস্থাপন করার অধিকার এই প্যাকেজের নেই, এবং { $name } সেবাটি এর উপর নির্ভর করে, তাই কিছুই ইনস্টল করা হয়নি: { $refusal }
        [task] { $path } প্রতিস্থাপন করার অধিকার এই প্যাকেজের নেই, এবং { $name } নির্ধারিত কার্যটি এর উপর নির্ভর করে, তাই কিছুই ইনস্টল করা হয়নি: { $refusal }
        [firewall] { $path } প্রতিস্থাপন করার অধিকার এই প্যাকেজের নেই, এবং { $name } ফায়ারওয়াল নিয়মটি এর উপর নির্ভর করে, তাই কিছুই ইনস্টল করা হয়নি: { $refusal }
        [shortcut] { $path } প্রতিস্থাপন করার অধিকার এই প্যাকেজের নেই, এবং { $name } শর্টকাটটি এর উপর নির্ভর করে, তাই কিছুই ইনস্টল করা হয়নি: { $refusal }
       *[registry] { $path } প্রতিস্থাপন করার অধিকার এই প্যাকেজের নেই, এবং { $name } রেজিস্ট্রি মানটি এর উপর নির্ভর করে, তাই কিছুই ইনস্টল করা হয়নি: { $refusal }
    }
core-warning-restore-destination-exists = গন্তব্যে আগে থেকেই আছে; ওভাররাইট করতে --force ব্যবহার করুন।
# { $path } is a file. { $error } is what went wrong.
core-warning-restore-file-not-written = { $path } লেখা যায়নি: { $error }
core-warning-restore-not-a-folder = এই খালি ফোল্ডারটি যেখানে যাবে, সেখানে ফোল্ডার নয় এমন কিছু আছে।
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-not-looked-at = { $path } পরীক্ষা করা যায়নি: { $error }
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-folder-not-made = { $path } তৈরি করা যায়নি: { $error }
# Why part of the app was left out, after "was left out:".
core-warning-restore-registration-exists = এই পিসিতে এটি আগে থেকেই আছে, অন্য কিছুতে সেট করা অবস্থায়, এবং কোনো ইনস্টল তা নিজের করে নেয় না
core-warning-restore-registry-review = আর্কাইভ করা কাঠামোবদ্ধ ডেটা থেকে রেজিস্ট্রি মানগুলো পুনরুদ্ধার করা হয়েছে, এবং অ্যাপের পরিচিত পাথগুলো লক্ষ্য রুট অনুযায়ী নতুন করে লেখা হয়েছে।
# { $error } is what went wrong.
core-warning-restore-firewall-unreadable = Windows ফায়ারওয়ালের নিয়মগুলো পড়া যায়নি: { $error }
# Why a firewall rule was left out, after "was left out:".
core-warning-restore-firewall-rule-exists = এই প্রোগ্রামের জন্য এই নামের একটি নিয়ম আগে থেকেই আছে, অন্যভাবে সেট করা অবস্থায়, এবং কোনো ইনস্টল তা পরিবর্তন করে না
# { $name } is a runtime.
core-warning-restore-prerequisite-unknown = { $name }: এই রানটাইমের জন্য কোনো পরিচিত ইনস্টলার নেই।
# { $name } is a runtime.
core-warning-restore-prerequisite-needs-admin = { $name } ইনস্টল করতে প্রশাসকের অধিকার প্রয়োজন; প্রশাসক হিসেবে ইনস্টলারটি আবার চালান।
# { $name } is a runtime.
core-warning-restore-prerequisite-reboot = { $name } ইনস্টল করা হয়েছে, তবে সম্পূর্ণ করতে পুনরায় চালু করা প্রয়োজন।
# { $error } is what went wrong.
core-warning-restore-skipped-not-recorded = এড়িয়ে যাওয়া একটি রানটাইম রেকর্ড করা যায়নি: { $error }
# { $name } is a runtime. { $reason } is why it was skipped, already in words.
core-warning-restore-prerequisite-skipped = { $name }: { $reason }
# { $name } is a runtime. { $reason } is why it could not be installed. { $url } is a web address to get it from.
core-warning-restore-prerequisite-failed = { $name } ইনস্টল করা যায়নি: { $reason }। এটি { $url } থেকে সংগ্রহ করুন।
# { $error } is what went wrong.
core-warning-restore-runtime-not-recorded = ইনস্টল করা একটি রানটাইম রেকর্ড করা যায়নি: { $error }
# { $error } is what went wrong.
core-warning-restore-license-source-invalid = প্যাকেজে থাকা লাইসেন্সের পাথ অবৈধ: { $error }
core-warning-restore-license-needs-admin = সুরক্ষিত অবস্থানে লাইসেন্স ডেটা পুনরুদ্ধার করতে প্রশাসকের অধিকার প্রয়োজন।
core-warning-restore-license-exists = লাইসেন্সের গন্তব্যে আগে থেকেই আছে; ওভাররাইট করতে --force ব্যবহার করুন।
# { $error } is what went wrong.
core-warning-restore-license-failed = লাইসেন্স ডেটা পুনরুদ্ধার করা যায়নি: { $error }
core-warning-restore-driver-needs-admin = ড্রাইভার নিবন্ধন করতে প্রশাসকের অধিকার প্রয়োজন; প্রশাসক হিসেবে ইনস্টলারটি আবার চালান।
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-warning-restore-task-runs-as-system = { $name } নির্ধারিত কার্যটি SYSTEM হিসেবে চলে, এই পিসির উপর পূর্ণ অধিকার নিয়ে।
core-warning-restore-shortcut-exists = শর্টকাট আগে থেকেই আছে; ওভাররাইট করতে --force ব্যবহার করুন।
core-warning-restore-shortcut-all-users = পুনরুদ্ধার করা এক বা একাধিক শর্টকাট সর্বজনীন ডেস্কটপে বা সবার স্টার্ট মেনুতে রাখা হয়েছে।
core-warning-restore-environment-refresh = এনভায়রনমেন্ট ভেরিয়েবলের পরিবর্তন কার্যকর দেখতে নতুন প্রসেস, শেল বা সাইন-ইন সেশন প্রয়োজন হতে পারে।
# { $name } is a service's name. { $account } is the account it ran as on the PC it came from.
core-warning-restore-service-account = উৎস পিসিতে { $name } সেবাটি { $account } হিসেবে চলত। এর পাসওয়ার্ড ইনস্টলারে নেই, তাই সেবাটি LocalService হিসেবে সেট আপ করা হয়েছে। এটি কাজ না করলে, “সেবাগুলো”-তে এর অ্যাকাউন্ট ও পাসওয়ার্ড সেট করুন।
# { $name } is a runtime.
core-warning-restore-prerequisite-pending-reboot = { $name } এখনো নিবন্ধিত হয়নি; পুনরায় চালু করার পরে এটি সক্রিয় হওয়ার কথা।
# { $name } is a runtime.
core-warning-restore-prerequisite-not-verified = { $name } সফল হয়েছে বলে জানিয়েছে, কিন্তু নিবন্ধিত হয়নি; ইনস্টল হয়তো ব্যর্থ হয়েছে।
# { $error } is what went wrong.
core-warning-restore-driver-source-invalid = সঙ্গে দেওয়া ড্রাইভারের পাথ অবৈধ: { $error }
# { $error } is what went wrong.
core-warning-restore-driver-not-read = ড্রাইভারের বর্তমান সেটিংস পড়া যায়নি: { $error }
core-warning-restore-driver-exists = ড্রাইভার সেবা আগে থেকেই আছে; এটি নতুন করে কনফিগার করতে --force ব্যবহার করুন।
core-warning-restore-driver-image-exists = ড্রাইভার ইমেজ আগে থেকেই আছে; ওভাররাইট করতে --force ব্যবহার করুন।
core-warning-restore-driver-hash-bundled = সঙ্গে দেওয়া ড্রাইভার ইমেজটি এর রেকর্ড করা হ্যাশের সঙ্গে মেলে না, তাই এটি ইনস্টল করা হয়নি।
# { $error } is what went wrong.
core-warning-restore-driver-not-placed = ড্রাইভার ইমেজ রাখা যায়নি: { $error }
core-warning-restore-driver-hash-placed = রাখার পরে ড্রাইভার ইমেজটি পরিবর্তিত হয়েছে, তাই এটি নিবন্ধন করা হয়নি।
# { $error } is what went wrong.
core-warning-restore-key-not-created = রেজিস্ট্রি কী তৈরি করা যায়নি: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-read = এটি যে মান প্রতিস্থাপন করবে তা পড়া যায়নি: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-written = রেজিস্ট্রি মান লেখা যায়নি: { $error }
core-warning-restore-value-exists = রেজিস্ট্রি মান আগে থেকেই আছে; ওভাররাইট করতে --force ব্যবহার করুন।
# { $error } is what went wrong.
core-warning-restore-environment-key-not-created = এনভায়রনমেন্টের রেজিস্ট্রি কী তৈরি করা যায়নি: { $error }
core-warning-restore-variable-exists = এনভায়রনমেন্ট ভেরিয়েবল আগে থেকেই আছে; ওভাররাইট করতে --force ব্যবহার করুন।
# { $error } is what went wrong.
core-warning-restore-variable-not-written = এনভায়রনমেন্ট ভেরিয়েবল লেখা যায়নি: { $error }
core-warning-restore-value-kind-missing = রেজিস্ট্রি মানের ধরন উল্লেখ নেই।
# { $error } is what went wrong.
core-warning-restore-service-not-read = সেবার বর্তমান সেটিংস পড়া যায়নি: { $error }
core-warning-restore-service-exists = সেবা আগে থেকেই আছে; এটি নতুন করে কনফিগার করতে --force ব্যবহার করুন।
# { $error } is what went wrong.
core-warning-restore-task-not-queried = নির্ধারিত কার্যের তথ্য জানা যায়নি: { $error }
core-warning-restore-task-exists = নির্ধারিত কার্য আগে থেকেই আছে; ওভাররাইট করতে --force ব্যবহার করুন।
# { $error } is what went wrong.
core-warning-restore-task-folder-not-looked-at = কার্যের ফোল্ডারটি পরীক্ষা করা যায়নি: { $error }
# { $error } is what went wrong.
core-warning-restore-shortcut-not-put-back = শর্টকাটটি তৈরি করা যায়নি, এবং এটি যেটিকে প্রতিস্থাপন করেছিল সেটিও আগের মতো ফিরিয়ে আনা যায়নি: { $error }
# { $error } is what went wrong.
core-warning-restore-command-not-started = কমান্ড চালু করা যায়নি: { $error }
# { $output } is what the command wrote, as it wrote it.
core-warning-restore-command-failed = কমান্ড ব্যর্থ হয়েছে: { $output }
# { $error } is what went wrong.
core-warning-restore-powershell-not-started = PowerShell চালু করা যায়নি: { $error }
