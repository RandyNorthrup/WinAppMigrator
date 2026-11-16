# Words the core says about what an install may do: why an install location or something in a package is refused,
# the installer's list of what an app sets up beyond its files, the checks of a package and of an install, and whether
# there is room for either. Message names start with core-.
#
# Many messages here are a reason, the second half of a sentence that starts "... was left out:" or "that install
# location cannot be used:", so they start with a small letter and have no full stop.


## Why an install location cannot be used. Shown in red under the Install Location field.

core-review-location-state-root = এখানে ইনস্টলগুলো তাদের রেকর্ড রাখে
core-review-location-has-files = এতে আগে থেকেই ফাইল আছে, এবং এখানে কোনো ইনস্টলের রেকর্ড নেই, তাই একটি খালি ফোল্ডার ব্যবহার করুন
core-policy-location-not-full-path = এটি অবশ্যই একটি সম্পূর্ণ পাথ হতে হবে
core-policy-location-parent-step = এতে “..” থাকা চলবে না
core-policy-location-not-plain = এটিকে এই পিসির কোনো ড্রাইভের একটি সাধারণ পাথ হতে হবে
core-policy-location-whole-drive = এটি একটি সম্পূর্ণ ড্রাইভ হতে পারে না
core-policy-location-windows = এটি Windows ফোল্ডার হতে পারে না
core-policy-location-runs-by-itself = এটি এমন একটি ফোল্ডার যা Windows-এর, অথবা যার ভিতরের জিনিস নিজে থেকেই চলে বা লোড হয়
core-policy-location-shared = এতে পাশাপাশি অন্যান্য জিনিস থাকে, তাই অ্যাপের জন্য এর ভিতরে একটি ফোল্ডার প্রয়োজন

## Whether an app can run on this PC at all, said before anything is installed.

# { $reason } is why, as the second half of a sentence, such as "App.exe is built for Arm64, and this PC is x64, so
# Windows would not start it".
core-review-cannot-run-here = এই অ্যাপটি এই পিসিতে চলতে পারবে না, তাই কিছুই পরিবর্তন করা হয়নি: { $reason }
core-policy-architecture-x86 = 32-বিট x86
core-policy-architecture-x64 = x64
core-policy-architecture-arm64 = Arm64
core-policy-architecture-arm32 = 32-বিট Arm
core-policy-architecture-unknown = একটি অজানা আর্কিটেকচার
# { $code } is the number Windows gives a kind of processor, such as 0x01c4.
core-policy-machine-type = মেশিনের ধরন { $code }
# { $names } is a list of program names, such as "a.exe, b.exe, c.exe", and { $count } how many more there are.
core-policy-names-and-more = { $names }, এবং আরও { $count }টি
core-policy-fit-programs-not-said = এই ইনস্টলারে বলা নেই এর প্রোগ্রামগুলোর কী প্রয়োজন, তাই সেগুলো এই পিসিতে চলতে পারবে কি না তা যাচাই করা হয়নি।
core-policy-fit-architecture-not-told = এই পিসির আর্কিটেকচার কী তা জানা যায়নি, তাই এই অ্যাপের প্রোগ্রামগুলো এতে চলতে পারবে কি না তা যাচাই করা হয়নি।
core-policy-fit-main-not-said = এই ইনস্টলারে বলা নেই এর মূল প্রোগ্রামের কী প্রয়োজন, তাই এখানে যা বলা হয়েছে তা এর বাকি প্রোগ্রামগুলোর ভিত্তিতে।
# { $driver } is a driver's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-driver = এর ড্রাইভার { $driver } { $built_for }-এর জন্য তৈরি, আর এই পিসি { $here }। ড্রাইভারকে Windows-এর সঙ্গে হুবহু মিলতে হয়, তাই এই অ্যাপটি এখানে চলতে পারবে না
# { $program } is a program's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-program = { $program } { $built_for }-এর জন্য তৈরি, আর এই পিসি { $here }, তাই Windows এটি চালু করবে না
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-none-start = এর কোনো প্রোগ্রামই এই পিসিতে ({ $here }) চালু হতে পারবে না: { $programs }
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-some-other-architecture = এই অ্যাপের কিছু প্রোগ্রাম অন্য আর্কিটেকচারের জন্য তৈরি, এবং এই পিসিতে ({ $here }) চালু হবে না: { $programs }। অ্যাপের বাকি অংশ চালু হতে পারে, তাই এতে হয়তো কোনো সমস্যা হবে না।
# { $here } is a kind of processor, such as "Arm64". { $programs } is a list of program names.
core-policy-fit-emulated = এই পিসি { $here }, তাই Windows এগুলো একটি এমুলেটরের মাধ্যমে চালাবে: { $programs }। বেশিরভাগ প্রোগ্রাম এভাবে ঠিকঠাক চলে, তবে কিছু চলে না।
# { $program } is a program's file name. { $needs } and { $runs } are versions of Windows, such as 10.0 and 6.1.
core-policy-fit-needs-newer = { $program }-এ বলা আছে এর Windows { $needs } বা তার চেয়ে নতুন সংস্করণ প্রয়োজন, আর এই পিসিতে চলছে { $runs }, তাই Windows এটি চালু করবে না
# { $programs } is a list of program names.
core-policy-fit-some-need-newer = এই অ্যাপের কিছু প্রোগ্রামে বলা আছে সেগুলোর এর চেয়ে নতুন Windows প্রয়োজন, তাই সেগুলো এখানে চালু হবে না: { $programs }।
# { $from_build } and { $this_build } are Windows build numbers, such as 22631, written as Windows writes them.
core-policy-fit-older-build = এই অ্যাপটি Windows বিল্ড { $from_build }-এ ছিল, আর এই পিসিতে চলছে বিল্ড { $this_build }। এর প্রোগ্রামগুলোতে বলা আছে সেগুলো এই Windows-এ চালু হতে পারে, তাই এটি কাজ করতে পারে, কিন্তু সেখানে এটি যার উপর নির্ভর করত তা হয়তো এখানে নেই।
core-policy-fit-build-not-told = এটি Windows-এর কোন বিল্ড তা জানা যায়নি, তাই এই অ্যাপটি যে বিল্ডে ছিল তার সঙ্গে তুলনা করা হয়নি।

## Room on a drive for a build or an install.

# Each of these ends the sentence core-space-shortfall, after "free for".
core-space-for-unpacking = প্যাকেজটি আনপ্যাক করার
core-space-for-install-record = ইনস্টলের রেকর্ডের
core-space-for-app-files = অ্যাপের ফাইলগুলোর
core-space-for-what-is-replaced = এটি যা প্রতিস্থাপন করে তা রেখে দেওয়ার
core-space-for-build-work = বিল্ডের কাজে লাগা ফাইলগুলোর
core-space-for-installer = ইনস্টলারের
core-space-for-package = প্যাকেজের
core-space-for-unpacking-and-installing = প্যাকেজটি আনপ্যাক ও ইনস্টল করার
# What goes between two of those, as in "unpacking the package and the app's files". Keep the quotes: they keep the
# spaces.
core-space-purposes-joiner = { " ও " }
# { $drive } is a drive, such as C:, or a folder. { $needed } and { $free } are sizes, such as 1.2 GB. { $purposes } is
# what the room is for, such as "unpacking the package and the app's files".
core-space-shortfall = { $purposes } জন্য { $drive }-এ সর্বোচ্চ { $needed } খালি জায়গা প্রয়োজন, কিন্তু আছে { $free }
# { $drive } is a drive, such as C:, or a folder. { $needed } is a size, such as 1.2 GB.
core-space-not-told = { $drive } কতটা জায়গা খালি আছে তা জানায়নি, তাই এর প্রয়োজনীয় { $needed } জায়গা সেখানে আঁটবে কি না তা যাচাই করা হয়নি
# One drive in a list of where room is needed. { $needed } and { $free } are sizes, and { $drive } a drive, such as C:.
core-space-summary = { $drive }-এ সর্বোচ্চ { $needed } ({ $free } খালি)
# { $needed } is a size, and { $drive } a drive, such as C:.
core-space-summary-not-told = { $drive }-এ সর্বোচ্চ { $needed } (কতটা খালি আছে তা জানা যায়নি)
# What goes between two sentences of core-space-shortfall inside core-review-no-room. Keep the quotes: they keep the
# space.
core-review-sentence-joiner = { "। " }
# { $shortfalls } is one or more sentences of core-space-shortfall.
core-review-no-room = এর জন্য যথেষ্ট জায়গা নেই, তাই কিছুই পরিবর্তন করা হয়নি। { $shortfalls }। সেখানে কিছু জায়গা খালি করুন, অথবা অন্য ড্রাইভে ইনস্টলের অবস্থান বেছে নিন।

## The installer's list of what an app sets up beyond its files: what each kind is called.

core-review-kind-service = সেবা
core-review-kind-driver = ড্রাইভার
core-review-kind-scheduled-task = নির্ধারিত কার্য
core-review-kind-autostart = স্বয়ংক্রিয়ভাবে শুরু হয়
core-review-kind-shortcut = শর্টকাট
core-review-kind-path = PATH
core-review-kind-environment-variable = এনভায়রনমেন্ট ভেরিয়েবল
core-review-kind-shell-extension = শেল এক্সটেনশন
core-review-kind-menu-command = মেনু কমান্ড
core-review-kind-link-type = লিঙ্কের ধরন
core-review-kind-file-type = ফাইলের ধরন
core-review-kind-com = COM
core-review-kind-firewall-rule = ফায়ারওয়াল নিয়ম
core-review-kind-event-log-source = ইভেন্ট লগের উৎস
core-review-kind-font = ফন্ট
core-review-kind-odbc-driver = ODBC ড্রাইভার
core-review-kind-odbc-source = ODBC ডেটা উৎস
core-review-kind-compatibility-setting = সামঞ্জস্যতার সেটিং
core-review-kind-default-apps = ডিফল্ট অ্যাপস
core-review-kind-program-name = প্রোগ্রামের নাম
core-review-kind-other-settings = অন্যান্য সেটিংস

## The installer's list of what an app sets up: what each line says of it.

# { $file } is a program's file name. { $account } is the Windows account it runs as, such as LocalSystem. { $starts }
# says when it starts, and is not shown itself; for "other", { $start } is one of the core-review-start- words below.
core-review-service-detail =
    { $starts ->
        [automatic] { $account } হিসেবে { $file } চালায়, এখনই এবং এই পিসি প্রতিবার চালু হলে শুরু হয়
        [delayed] { $account } হিসেবে { $file } চালায়, এখনই এবং এই পিসি প্রতিবার চালু হওয়ার কিছুক্ষণ পরে শুরু হয়
        [manual] { $account } হিসেবে { $file } চালায়, কেবল কিছু চাইলে তবেই শুরু হয়
        [disabled] { $account } হিসেবে { $file } চালায়, নিষ্ক্রিয় করা আছে
        [other] { $account } হিসেবে { $file } চালায়, শুরু হয়: { $start }
       *[unknown] { $account } হিসেবে { $file } চালায়
    }
# { $account } is a Windows account, such as LocalService.
core-policy-account-custom = { $account } (কাস্টম অ্যাকাউন্ট পুনরুদ্ধার করা হয়নি)
# { $file } is a driver's file name.
core-review-driver-detail = { $file } সরাসরি Windows-এর ভিতরে লোড করে
# { $file } is a driver's file name. { $start } is when Windows starts it: one of the core-review-start- words below.
core-review-driver-detail-starts = { $file } সরাসরি Windows-এর ভিতরে লোড করে, শুরু হয়: { $start }
# When a service or driver starts, ending a sentence such as "Loads disk.sys into Windows itself, starts …".
core-review-start-boot = Windows-এর সঙ্গে, অন্য সবকিছুর আগে
core-review-start-system = Windows-এর সঙ্গে, Windows চালু হওয়ার সময়
core-review-start-automatic = এই পিসি প্রতিবার চালু হলে
core-review-start-delayed = এই পিসি প্রতিবার চালু হওয়ার কিছুক্ষণ পরে
core-review-start-manual = কেবল কিছু চাইলে তবেই
core-review-start-disabled = কখনোই না, কারণ এটি নিষ্ক্রিয় করা আছে
core-review-start-unknown = এমনভাবে, যা Windows জানায় না
# { $account } is the account a task runs as: an account's name, or core-review-task-runs-as-you, or
# core-review-task-runs-as-creator.
core-review-task-detail = যে অ্যাকাউন্ট হিসেবে চলে: { $account }
# { $account } is the account a task runs as, as for core-review-task-detail.
core-review-task-detail-highest = যে অ্যাকাউন্ট হিসেবে চলে: { $account }, সম্ভাব্য সর্বোচ্চ অধিকার নিয়ে
core-review-task-runs-as-you = আপনি, অর্থাৎ যিনিই সাইন ইন করে থাকুন
core-review-task-runs-as-creator = যে ব্যবহারকারী এটি তৈরি করেছেন
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-autostart-detail =
    { $scope ->
        [everyone] যিনিই সাইন ইন করেন, তাঁর জন্য চলে
       *[you] আপনি সাইন ইন করলে চলে
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shortcut-startup-detail =
    স্টার্টআপ ফোল্ডারে একটি শর্টকাট, { $scope ->
        [everyone] এই পিসির সবার জন্য
       *[you] আপনার জন্য
    }
# { $file } is the file name of what the shortcut opens. { $scope } is whom it is set up for: everyone on this PC,
# or you. It is not shown itself.
core-review-shortcut-detail =
    { $file } খোলে, { $scope ->
        [everyone] এই পিসির সবার জন্য
       *[you] আপনার জন্য
    }
# { $name } is an environment variable's name, such as PATH.
core-review-path-title = { $name }-এ যোগ করে
core-review-path-install-folder = ইনস্টল ফোল্ডার
# { $path } is a folder inside the install folder, such as bin\tools.
core-review-path-below-install-folder = ইনস্টল ফোল্ডার\{ $path }
# { $value } is the variable's value, as it will be set.
core-review-variable-detail = এই মানে সেট করা হয়: { $value }
# { $count } is how many registry values.
core-review-other-settings-title =
    { $count ->
        [one] অ্যাপের নিজস্ব কীগুলোর বাইরে { $count }টি সেটিং
       *[other] অ্যাপের নিজস্ব কীগুলোর বাইরে { $count }টি সেটিং
    }
# { $keys } is a list of registry keys, such as HKCU\Software\Vendor.
core-review-other-settings-detail = { $keys }-এ
# { $keys } is a list of registry keys, and { $count } how many more there are.
core-review-other-settings-detail-more = { $keys }-এ, এবং আরও { $count }টিতে
# { $way } says whether the rule allows or blocks, and incoming or outgoing connections; it is not shown itself.
# { $ports } is a list of port numbers, and { $port_count } how many. { $program } is the program's file name.
# { $networks } is a list of the kinds of network Windows names in English (domain, private, public), and
# { $network_count } how many.
# The kinds of network a firewall rule is for, as Windows Defender Firewall names its profiles in your language. They
# fill { $networks } in core-review-firewall-detail, joined into a list.
core-review-network-domain = ডোমেন
core-review-network-private = ব্যক্তিগত
core-review-network-public = সর্বজনীন
core-review-firewall-detail =
    { $program }-এর জন্য { $network_count ->
        [0] সব নেটওয়ার্কে
       *[other] { $networks } নেটওয়ার্কে
    }{ $port_count ->
        [0] { "" }
       *[other] { " " }{ $ports } পোর্টে
    } { $way ->
        [allow-in] অন্তর্মুখী সংযোগের অনুমতি দেয়
        [allow-out] বহির্মুখী সংযোগের অনুমতি দেয়
        [block-in] অন্তর্মুখী সংযোগ ব্লক করে
       *[block-out] বহির্মুখী সংযোগ ব্লক করে
    }
# { $server } is the file name of what Explorer loads. { $scope } is whom it is set up for: everyone on this PC, or
# you. It is not shown itself.
core-review-shell-extension-detail =
    ফাইল এক্সপ্লোরারে { $server } লোড করে, { $scope ->
        [everyone] এই পিসির সবার জন্য
       *[you] আপনার জন্য
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shell-extension-detail-own-file =
    ফাইল এক্সপ্লোরারে এর নিজের ইনস্টল করা একটি ফাইল লোড করে, { $scope ->
        [everyone] এই পিসির সবার জন্য
       *[you] আপনার জন্য
    }
# The name of a shell extension: { $handler } is its name in the registry, and { $class } the kind of file or folder it
# is for, such as * or Directory.
core-policy-shell-extension-name = { $class }-এ { $handler }
# { $on } is the kind of file or folder whose menu it is, as the registry names it, such as * or
# Directory. { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-menu-command-detail =
    { $on }-এর মেনুতে একটি কমান্ড যোগ করে, { $scope ->
        [everyone] এই পিসির সবার জন্য
       *[you] আপনার জন্য
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-default-verb-detail =
    এগুলোর কোনোটি খুললে কী হয় তা বদলে দেয়, { $scope ->
        [everyone] এই পিসির সবার জন্য
       *[you] আপনার জন্য
    }
core-review-client-entry-detail = এই কাজের প্রোগ্রাম হিসেবে বেছে নেওয়া যায়। নিজে থেকে কিছুই এতে বদলানো হয় না
core-review-com-permissions-title = COM অনুমতি
core-review-com-permissions-detail = কে এর COM ক্লাসগুলো চালু করতে পারে, বা সেগুলো কোন অ্যাকাউন্ট হিসেবে চলে, তা নির্ধারণ করে
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-link-type-detail =
    এই ধরনের লিঙ্ক অ্যাপটি দিয়ে খোলে, { $scope ->
        [everyone] এই পিসির সবার জন্য
       *[you] আপনার জন্য
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-file-type-detail =
    এই ধরনের ফাইল অ্যাপটি দিয়ে খোলে, { $scope ->
        [everyone] এই পিসির সবার জন্য
       *[you] আপনার জন্য
    }
# { $log } is the name of a Windows event log, such as Application.
core-review-event-log-source-detail = এর নিজস্ব ফাইলের বার্তা দিয়ে { $log } লগে লিখতে পারে
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-font-detail =
    আপনি সাইন ইন করলে ইনস্টল ফোল্ডার থেকে লোড হয়, { $scope ->
        [everyone] এই পিসির সবার জন্য
       *[you] আপনার জন্য
    }
core-review-odbc-driver-detail = যে প্রোগ্রামগুলো ডেটাবেসে পৌঁছাতে ODBC ব্যবহার করে, সেগুলোতে লোড হয়
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-odbc-source-detail =
    একটি ডেটাবেস সংযোগ, { $scope ->
        [everyone] এই পিসির সবার জন্য
       *[you] আপনার জন্য
    }
core-review-default-apps-detail = ডিফল্ট অ্যাপস-এ বেছে নেওয়া যায়। নিজে থেকে কিছুই এতে বদলানো হয় না
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-program-name-detail =
    “চালান” ও অন্যান্য প্রোগ্রাম এই নামে এটি খুঁজে পায়, { $scope ->
        [everyone] এই পিসির সবার জন্য
       *[you] আপনার জন্য
    }
# { $server } is the file name of what serves the classes.
core-review-com-title = { $server }-এ COM
core-review-com-title-own-file = এর ইনস্টল করা একটি ফাইলে COM
# { $count } is how many registry values.
core-review-com-detail =
    { $count ->
        [one] অন্য প্রোগ্রাম চালু করতে পারে এমন ক্লাসগুলোর { $count }টি সেটিং
       *[other] অন্য প্রোগ্রাম চালু করতে পারে এমন ক্লাসগুলোর { $count }টি সেটিং
    }
core-review-com-support-title = COM টাইপ লাইব্রেরি, ইন্টারফেস ও AppID
# { $libraries }, { $interfaces }, and { $app_ids } are how many registry values there are of each.
core-review-com-support-detail = এর ক্লাসগুলোর সঙ্গে যাওয়া সেটিং: টাইপ লাইব্রেরির { $libraries }টি, ইন্টারফেসের { $interfaces }টি ও AppID-এর { $app_ids }টি
core-review-compatibility-title = এর প্রোগ্রামগুলো কীভাবে চালু হয়
# { $count } is how many registry values.
core-review-compatibility-detail =
    { $count ->
        [one] এর ইনস্টল করা প্রোগ্রামগুলোর জন্য { $count }টি সামঞ্জস্যতার সেটিং
       *[other] এর ইনস্টল করা প্রোগ্রামগুলোর জন্য { $count }টি সামঞ্জস্যতার সেটিং
    }

## The installer's notes on whose folders and settings these are, and what is not coming.

# { $account } is the name of a Windows account, such as alice.
core-review-account-signed-in = ব্যবহারকারীর ডেটা ও প্রতি-ব্যবহারকারী সেটিংস যাবে { $account }-এর কাছে, যিনি এখানে সাইন ইন করে আছেন; এটি যে প্রশাসক হিসেবে চলছে তাঁর কাছে নয়।
# { $account } is the name of a Windows account, such as alice.
core-review-account-no-desktop = এই সেশনে কেউ সাইন ইন করে নেই, তাই ব্যবহারকারীর ডেটা ও প্রতি-ব্যবহারকারী সেটিংস যাবে { $account }-এর কাছে, যে অ্যাকাউন্ট হিসেবে এটি চলছে।
# { $reason } is what Windows said, in its own words.
core-review-account-not-told = কে সাইন ইন করে আছেন তা জানা যায়নি ({ $reason }), তাই প্রতি-ব্যবহারকারী সেটিংস বাদ দেওয়া হয়েছে, এবং ব্যবহারকারীর ডেটা ইনস্টলের অবস্থানের ভিতরে যাবে।
# { $reason } is what Windows said, in its own words.
core-review-signed-in-unknown = কে সাইন ইন করে আছেন তা জানা যায়নি: { $reason }
# Headings of what is left out, shared by the app's list and the installer's notes.
core-review-left-out-credentials = এই ইনস্টলারে নেই, কারণ এগুলোকে ক্রেডেনশিয়ালের মতো দেখায়
core-review-left-out-settings = কোনো অ্যাকাউন্ট অ্যাপটির জন্য যে সেটিংস ও ইতিহাস রাখে, যা কেবল ব্যবহারকারীর ডেটা বিকল্পের সঙ্গে যায়
core-review-left-out-certificates = অ্যাপটি মনে হয় তার পিসিতে বিশ্বস্ত হিসেবে রাখা কিছু সার্টিফিকেটের উপর নির্ভর করে, এবং সেগুলো ইনস্টল করা হয় না
core-review-left-out-fonts = এই ফন্টগুলো অ্যাপের নামে পরিচিত, কিন্তু কোথাও বলা নেই যে এগুলো এর নিজস্ব, তাই এগুলো ইনস্টল করা হয় না
# { $names } is a list of names, such as the names of saved passwords.
core-review-note-credentials = এই ইনস্টলারে নেই, কারণ এগুলোকে ক্রেডেনশিয়ালের মতো দেখায়: { $names }। এই পিসিতে এগুলোর যা আছে তা যেমন আছে তেমনই থাকবে, এবং আপনাকে হয়তো এগুলো আবার লিখতে হবে।
# { $names } is a list of names of license data.
core-review-note-licenses = এই ইনস্টলারে নেই, কারণ তৈরির সময় লাইসেন্স ও অ্যাক্টিভেশন ডেটা বিকল্পটি বন্ধ ছিল: { $names }।
# { $names } is a list of places an account keeps settings, such as folders.
core-review-note-settings = এই ইনস্টলারে নেই, কারণ তৈরির সময় ব্যবহারকারীর ডেটা বিকল্পটি বন্ধ ছিল: কোনো অ্যাকাউন্ট অ্যাপটির জন্য { $names }-এ যে সেটিংস ও ইতিহাস রাখে। অ্যাপটি এখানে এমনভাবে শুরু হবে, যেন কেউ প্রথমবার এটি ব্যবহার করছেন।
# { $names } is a list of certificates, each as core-review-certificate.
core-review-note-certificates = অ্যাপটি মনে হয় তার পিসিতে বিশ্বস্ত হিসেবে রাখা কিছু সার্টিফিকেটের উপর নির্ভর করে, এবং সেগুলো ইনস্টল করা হয় না: { $names }। এখানে সেগুলোকে বিশ্বাস করবেন কি না, তা আপনার সিদ্ধান্ত।
# { $names } is a list of font names.
core-review-note-fonts = এই ফন্টগুলো অ্যাপের নামে পরিচিত, কিন্তু কোথাও বলা নেই যে এগুলো এর নিজস্ব, তাই এগুলো ইনস্টল করা হয় না: { $names }।
# { $names } is a list of names.
core-review-note-left-out-by-choice = যিনি এই ইনস্টলারটি তৈরি করেছেন তিনি বাদ দিয়েছেন: { $names }।
# { $names } is a list of names.
core-review-note-added-by-choice = যিনি এই ইনস্টলারটি তৈরি করেছেন তিনি যোগ করেছেন, কারণ তিনি এগুলোকে অ্যাপের অংশ বলে ধরে নিয়েছেন: { $names }।
# { $name } is a certificate's subject, such as CN=Contoso, and { $store } the certificate store it was in, such as
# Root.
core-review-certificate = { $store }-এ { $name }
# { $names } is a list of names, and { $count } how many more there are.
core-review-names-and-more = { $names }, এবং আরও { $count }টি

## Why something in a package is left out. Each is the second half of a sentence of core-warning-review-left-out.

core-review-no-account = এটি একটি অ্যাকাউন্টের সেটিংসের অংশ, এবং কে সাইন ইন করে আছেন তা জানা যায়নি
core-review-name-not-accepted = Windows এর নাম গ্রহণ করবে না
# { $error } is what Windows said, in its own words.
core-review-service-unreadable = এর বর্তমান সেটিংস পড়া যায়নি: { $error }
core-review-service-dependency = এটি অবৈধ নামের একটি সেবার উপর নির্ভর করে
# { $path } is the program the service runs.
core-review-service-runs-other = এটি { $path } চালায়, যা এই প্যাকেজ ইনস্টল করে না
core-review-service-settings = এর সেটিংসে এমন কিছুর নাম আছে যা এই প্যাকেজ ইনস্টল করে না, অথবা এমন একটি কী যা এর নিজের নয়
core-review-driver-no-hash = এটি যাচাই করার মতো কোনো হ্যাশ এর নেই
core-review-license-for-the-machine = পুরো মেশিনের জন্য রাখা লাইসেন্স কেবল জানানো হয়
core-review-license-starts-something = কোনো কিছু চালু করার কোনো কারণ লাইসেন্সের নেই
core-review-task-not-captured = এর সংজ্ঞা সংগ্রহ করা হয়নি
# { $error } is what was wrong with the task's XML, in English.
core-review-task-unreadable = এর সংজ্ঞা পড়া যায়নি: { $error }
core-review-task-no-account = যিনি এটি ইনস্টল করেন এটি তাঁর হিসেবে চলে, এবং এখানে কে সাইন ইন করে আছেন তা জানা যায়নি
# { $path } is what the shortcut opens.
core-review-shortcut-opens-other = এটি { $path } খোলে, যা এই প্যাকেজ ইনস্টল করে না
# { $service } is the service's name, and { $path } the program it runs now.
core-policy-service-exists = { $service } এই পিসিতে আগে থেকেই আছে এবং { $path } চালায়, যা এখানে আগের কোনো ইনস্টল থেকে আসেনি। এটি এই অ্যাপের পুরোনো কোনো অনুলিপি হলে, প্রথমে সেটি আনইনস্টল করুন
core-policy-key-control-characters = এর কী-এর নামে কন্ট্রোল ক্যারেক্টার আছে
core-policy-value-control-characters = এর মানের নামে কন্ট্রোল ক্যারেক্টার আছে
core-policy-registry-link = এটি একটি রেজিস্ট্রি লিঙ্ক
# { $reason } is why the key it stands for is left out, such as "no app install should change this key".
core-policy-virtual-store = এটি পিসির একটি কী-এর প্রতিনিধিত্ব করে, এবং { $reason }
core-policy-registry-part = কোনো অ্যাপের ইনস্টলেরই রেজিস্ট্রির এই অংশে লেখা উচিত নয়
core-policy-key-refused = কোনো অ্যাপের ইনস্টলেরই এই কী পরিবর্তন করা উচিত নয়
core-policy-net-debugger = এটি প্রতিটি .NET প্রোগ্রামের জন্য একটি ডিবাগার নির্ধারণ করবে
core-policy-net-install-root = প্রতিটি .NET প্রোগ্রাম কোথা থেকে .NET লোড করে, এটি তা বদলে দেবে
core-policy-class-redirects = এক ধরনের প্রোগ্রাম বা ফোল্ডার কীভাবে খোলে, এটি তা বদলে দেবে
core-policy-app-path-name = এটি একটি প্রোগ্রামকে ফাইলের নাম ছাড়া অন্য কিছু দিয়ে নির্দেশ করে
core-policy-app-path-not-installed = এটি এমন একটি ফাইলের জন্য প্রোগ্রামের নাম নিবন্ধন করবে, যা এই প্যাকেজ ইনস্টল করে না
core-policy-shared-default-verb = একই ধরনের প্রতিটি ফাইল বা ফোল্ডার খুললে কী হয়, এটি তা বদলে দেবে
core-policy-command-runs-other = এটি একটি কমান্ড যোগ করে, যা এমন কিছু চালায় যা এই প্যাকেজ ইনস্টল করে না
core-policy-autostart-not-command = এটি স্বয়ংক্রিয়ভাবে শুরু হয়, এবং এর মান কোনো কমান্ড নয়
core-policy-autostart-runs-other = এটি স্বয়ংক্রিয়ভাবে শুরু হয়, এবং এমন কিছু চালায় যা এই প্যাকেজ ইনস্টল করে না
core-policy-environment-own-rules = এনভায়রনমেন্ট ভেরিয়েবল তাদের নিজস্ব নিয়মে পুনরুদ্ধার করা হয়
core-policy-service-not-restored = এটি এমন একটি সেবার অংশ, যা এই প্যাকেজ পুনরুদ্ধার করে না
core-policy-service-own-rules = সেবার নিজস্ব সেটিংস কেবল এর নিয়ম অনুযায়ীই পুনরুদ্ধার করা হয়
core-policy-service-dll = এর ServiceDll এমন কোনো ফাইল নয় যা এই প্যাকেজ ইনস্টল করে
core-policy-event-log-source-only = কেবল ইভেন্ট লগের উৎসের নিজস্ব মানগুলোই পুনরুদ্ধার করা যায়
core-policy-security-log = কোনো অ্যাপ নিরাপত্তা লগে লেখে না
core-policy-event-log-messages = এর বার্তাগুলো এমন একটি ফাইল থেকে আসবে, যা এই প্যাকেজ ইনস্টল করে না
core-policy-type-library = এর টাইপ লাইব্রেরি এমন কোনো ফাইল নয় যা এই প্যাকেজ ইনস্টল করে
# { $name } is a program's file name, such as app.exe.
core-policy-application-opened = { $name } কীভাবে খোলে, এটি তা বদলে দেবে, অথচ { $name } এই প্যাকেজ ইনস্টল করে না
core-policy-event-log-no-file = ইভেন্ট লগের উৎসকে তার বার্তাগুলো কোনো ফাইল থেকে নিতে হয়
core-policy-com-runs-other = এটি এমন কিছু চালায় যা এই প্যাকেজের ইনস্টল করা কোনো ফাইল নয়
core-policy-com-redirect = এটি এই ক্লাসটিকে এমন কিছুতে পরিণত করবে, যা এই প্যাকেজ ইনস্টল করে না
core-policy-com-server = এটি এমন কিছু চালায় যা এই প্যাকেজের ইনস্টল করা কোনো ফাইলও নয়, Windows-এর অংশও নয়
core-policy-interface = কোনো ইন্টারফেসে কল কীভাবে করা হয়, এটি তা এমন কিছু দিয়ে নির্ধারণ করবে যা এই প্যাকেজ ইনস্টল করে না
# { $name } is a program's file name, such as app.exe.
core-policy-app-id-started = { $name } কীভাবে চালু হয়, এটি তা বদলে দেবে, অথচ { $name } এই প্যাকেজ ইনস্টল করে না
core-policy-app-id-surrogate = এটি এমন একটি প্রোগ্রামের মধ্যে চলবে, যা এই প্যাকেজ ইনস্টল করে না
core-policy-app-id-service = এটি এমন একটি সেবার মধ্যে চলবে, যা এই প্যাকেজ পুনরুদ্ধার করে না
core-policy-app-id-run-as = এটি নিজস্ব একটি অ্যাকাউন্ট হিসেবে চলবে
core-policy-app-id-link = এতে কোনো AppID-এর নাম উল্লেখ নেই
core-policy-app-id-remote = এটি এই পিসি ছাড়া অন্য কোথাও চলবে
core-policy-odbc-own-driver = এটি ODBC-র নিজস্ব সেটিং, কোনো ড্রাইভার নয়
core-policy-odbc-driver-not-installed = এর ড্রাইভার এমন কোনো ফাইল নয় যা এই প্যাকেজ ইনস্টল করে
core-policy-loads-other-file = এটি এমন একটি ফাইল লোড করবে, যা এই প্যাকেজ ইনস্টল করে না
core-policy-odbc-own-source = এটি ODBC-র নিজস্ব সেটিং, কোনো ডেটা উৎস নয়
core-policy-odbc-source-missing = এটি এমন একটি ডেটা উৎস তালিকাভুক্ত করে, যা এই প্যাকেজে নেই
core-policy-odbc-source-driver = এর ড্রাইভার এই প্যাকেজের ইনস্টল করা কোনো ফাইলও নয়, Windows-এর অংশও নয়
core-policy-shell-extension = এটি ফাইল এক্সপ্লোরারে এমন একটি শেল এক্সটেনশন লোড করবে, যা এই প্যাকেজের ইনস্টল করা কোনো ফাইল নয়
core-policy-not-text = এতে যা আছে তা সেই ধরনের টেক্সট নয়, যা হিসেবে এটি পড়া হবে
core-policy-network-place = এটি নেটওয়ার্কের কোনো জায়গার নাম উল্লেখ করে
core-policy-font = এটি এমন একটি ফন্ট নিবন্ধন করবে যা এই প্যাকেজ ইনস্টল করে না, অথবা Windows ফোল্ডারে থাকা কোনো ফন্ট
core-policy-compatibility = এই প্যাকেজ ইনস্টল করে না এমন একটি প্রোগ্রাম কীভাবে চলে, এটি তা বদলে দেবে
core-policy-capabilities = এটি এমন একটি অ্যাপ তালিকাভুক্ত করে, যার বিবরণ এই প্যাকেজে নেই
core-policy-default-program = কোন প্রোগ্রাম ডিফল্টভাবে বিভিন্ন জিনিস খোলে, এটি তা বদলে দেবে
core-policy-url-protocol = এটি নিজেকে একটি লিঙ্কের ধরন বলে জানায়, অথচ এই প্যাকেজের ইনস্টল করা কোনো কিছু দিয়ে খোলে না
core-policy-command-handler = এটি এমন একটি কমান্ড যোগ করে, যা এই প্যাকেজের ইনস্টল না করা কিছু পরিচালনা করে
core-policy-installer-command = এটি এমন কিছু চালাবে, যা কেবল Windows Installer পড়তে পারে
core-policy-creates-class = এটি এমন একটি ক্লাস তৈরি করবে, যা এই প্যাকেজ ইনস্টল করে না
core-policy-opens-links = এটি এমন কিছু দিয়ে লিঙ্ক খোলে, যা এই প্যাকেজ ইনস্টল করে না
core-policy-opens-files = এটি এমন কিছু দিয়ে ফাইল খোলে, যা এই প্যাকেজ ইনস্টল করে না
core-policy-firewall-name = এর নাম এমন যে তা অন্য পিসিতে নেওয়া যায় না
core-policy-firewall-description = এর বিবরণ এমন যে তা অন্য পিসিতে নেওয়া যায় না
core-policy-firewall-every-program = এটি প্রতিটি প্রোগ্রামের উপর প্রযোজ্য, অথচ একটি নিয়ম কেবল একটি প্রোগ্রামের জন্য হতে পারে
# { $path } is the program the rule is for.
core-policy-firewall-program = এটি { $path }-এর জন্য, যা এই প্যাকেজ ইনস্টল করে না
core-policy-firewall-direction = এতে এমন কোনো দিক বলা নেই যা এখানে সেট করা যায়
core-policy-firewall-action = এটি অন্য নিয়মগুলোকে অগ্রাহ্য করবে, যা কোনো অ্যাপের নিয়মেরই করা উচিত নয়
# { $setting } is a part of a firewall rule, as the scan of its PC described it.
core-policy-firewall-not-carried = এতে বলা আছে { $setting }, যা যেমন ছিল তেমন সেট করা যায়নি, অথবা যা অন্য নিয়মগুলোর কাজ দুর্বল করে দেবে
core-policy-firewall-service = এটি এমন একটি সেবার জন্য, যা এই প্যাকেজ পুনরুদ্ধার করে না
core-policy-firewall-parts = এর একটি অংশ এমন কিছু, যা ফায়ারওয়াল নিয়মে বলা যায় না
core-policy-folder-already-there = সেই ফোল্ডারটি ইনস্টলের অবস্থানের বাইরে আগে থেকেই ছিল, এবং কোথাও বলা নেই যে এটি এই অ্যাপের, তাই এতে কোনো প্রোগ্রাম রাখা হবে না
core-policy-variable-name = এর নাম কোনো বৈধ ভেরিয়েবলের নাম নয়
core-policy-variable-relied-on = Windows ও অন্যান্য প্রোগ্রাম এই ভেরিয়েবলের উপর নির্ভর করে
core-policy-task-name = এর নাম এমন নয় যা Windows গ্রহণ করে
core-policy-task-folder = এর ফোল্ডার এমন নয় যা Windows গ্রহণ করে
core-policy-task-windows = এটি Windows-এর নিজস্ব কোনো কার্য প্রতিস্থাপন করবে
core-policy-task-other-action = এটি প্রোগ্রাম চালানো ছাড়া অন্য কিছু করে
core-policy-task-no-program = এতে চালানোর মতো কোনো প্রোগ্রামের নাম নেই
# { $path } is the program the task runs.
core-policy-task-runs-other = এটি এমন কিছু চালায় যা এই প্যাকেজ ইনস্টল করে না: { $path }
core-policy-task-exists-unreadable = এই নামের একটি কার্য আগে থেকেই আছে, এবং সেটি পড়া যায়নি
core-policy-task-exists-other = এই নামের একটি কার্য আগে থেকেই আছে, এবং সেটি অ্যাপটি চালায় না
# { $path } is the program the task already on this PC runs.
core-policy-task-exists-runs = এই নামের একটি কার্য আগে থেকেই আছে এবং { $path } চালায়, যা এখানে আগের কোনো ইনস্টল থেকে আসেনি
core-policy-shortcut-not-lnk = এটি কোনো .lnk শর্টকাট নয়
core-policy-shortcut-place = এটি কোনো স্টার্ট মেনুতে বা ডেস্কটপে নেই
core-policy-short-name = এর পাথে এমন একটি নাম আছে যা যাচাই করা যায় না
core-policy-shortcut-no-folder = এই পিসিতে এর জন্য এমন কোনো ফোল্ডার নেই
core-policy-data-startup = এটি একটি স্টার্টআপ ফোল্ডারে যাবে, এবং প্রতিবার সাইন ইন করলে চলবে
core-policy-data-powershell = এটি এমন জায়গায় যাবে, যেখানে যা পাওয়া যায় PowerShell চালু হওয়ার সময় তা চালায়
core-policy-data-windows = এটি Windows-এর একটি ফোল্ডারে যাবে
core-policy-license-outside = এটি ইনস্টলের অবস্থান ও প্রতিটি পরিচিত ডেটা ফোল্ডারের বাইরে আছে
core-rewrite-cdata = এতে একটি CDATA অংশ আছে
core-rewrite-too-much-to-look-through = এই প্যাকেজের সেটিংসে এত বেশি কিছু আছে যে সেগুলোতে কীসের নাম আছে তা যাচাই করা সম্ভব নয়

## Warnings a review gives, which the installer shows with what it sets up.

# { $kind } says what was left out, and is not shown itself. { $item } is its path or name, or a registry value such
# as HKLM\Software\App\Name. { $reason } is why, as the second half of the sentence, such as "it cannot be a whole
# drive".
core-warning-review-left-out =
    { $kind ->
        [file] { $item } ফাইলটি বাদ দেওয়া হয়েছে: { $reason }।
        [folder] { $item } ফোল্ডারটি বাদ দেওয়া হয়েছে: { $reason }।
        [service] { $item } সেবাটি বাদ দেওয়া হয়েছে: { $reason }।
        [driver] { $item } ড্রাইভারটি বাদ দেওয়া হয়েছে: { $reason }।
        [firewall-rule] { $item } ফায়ারওয়াল নিয়মটি বাদ দেওয়া হয়েছে: { $reason }।
        [license-data] { $item }-এ থাকা লাইসেন্স ডেটা বাদ দেওয়া হয়েছে: { $reason }।
        [license-value] { $item } লাইসেন্স মানটি বাদ দেওয়া হয়েছে: { $reason }।
        [scheduled-task] { $item } নির্ধারিত কার্যটি বাদ দেওয়া হয়েছে: { $reason }।
        [shortcut] { $item } শর্টকাটটি বাদ দেওয়া হয়েছে: { $reason }।
        [environment-variable] { $item } এনভায়রনমেন্ট ভেরিয়েবলটি বাদ দেওয়া হয়েছে: { $reason }।
       *[value] { $item } বাদ দেওয়া হয়েছে: { $reason }।
    }
core-warning-review-package-path-missing = একটি অনুলিপিযোগ্য ফাইলের প্যাকেজের ভিতরের কোনো আপেক্ষিক পাথ নেই।
# { $count } is the most empty folders a package makes.
core-warning-review-too-many-empty-folders = এই প্যাকেজে { $count }টির বেশি খালি ফোল্ডার তালিকাভুক্ত আছে, এবং কেবল ততগুলোই তৈরি করা হয়।
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Documents folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-documents-elsewhere =
    { $count ->
        [one] নথিগুলো থেকে { $count }টি ফাইল এর বদলে { $target }-এর ভিতরে _external নামের একটি ফোল্ডারে যাবে, কারণ এই পিসির নথিগুলো ফোল্ডার { $folder }-এ আছে, এবং কেবল আপনার প্রোফাইলের ভিতরের ফোল্ডারেই লেখা হয়। এটি ব্যবহার করতে, নিজে এটি আপনার নথিগুলো ফোল্ডারে সরিয়ে নিন।
       *[other] নথিগুলো থেকে { $count }টি ফাইল এর বদলে { $target }-এর ভিতরে _external নামের একটি ফোল্ডারে যাবে, কারণ এই পিসির নথিগুলো ফোল্ডার { $folder }-এ আছে, এবং কেবল আপনার প্রোফাইলের ভিতরের ফোল্ডারেই লেখা হয়। সেগুলো ব্যবহার করতে, নিজে সেগুলো আপনার নথিগুলো ফোল্ডারে সরিয়ে নিন।
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-documents-unknown =
    { $count ->
        [one] নথিগুলো থেকে { $count }টি ফাইল এর বদলে { $target }-এর ভিতরে _external নামের একটি ফোল্ডারে যাবে, কারণ এই পিসির নথিগুলো ফোল্ডার কোথায় তা জানা যায়নি। এটি ব্যবহার করতে, নিজে এটি আপনার নথিগুলো ফোল্ডারে সরিয়ে নিন।
       *[other] নথিগুলো থেকে { $count }টি ফাইল এর বদলে { $target }-এর ভিতরে _external নামের একটি ফোল্ডারে যাবে, কারণ এই পিসির নথিগুলো ফোল্ডার কোথায় তা জানা যায়নি। সেগুলো ব্যবহার করতে, নিজে সেগুলো আপনার নথিগুলো ফোল্ডারে সরিয়ে নিন।
    }
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Saved Games folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-elsewhere =
    { $count ->
        [one] সংরক্ষিত গেমগুলো থেকে { $count }টি ফাইল এর বদলে { $target }-এর ভিতরে _external নামের একটি ফোল্ডারে যাবে, কারণ এই পিসির সংরক্ষিত গেমগুলো ফোল্ডার { $folder }-এ আছে, এবং কেবল আপনার প্রোফাইলের ভিতরের ফোল্ডারেই লেখা হয়। এটি ব্যবহার করতে, নিজে এটি আপনার সংরক্ষিত গেমগুলো ফোল্ডারে সরিয়ে নিন।
       *[other] সংরক্ষিত গেমগুলো থেকে { $count }টি ফাইল এর বদলে { $target }-এর ভিতরে _external নামের একটি ফোল্ডারে যাবে, কারণ এই পিসির সংরক্ষিত গেমগুলো ফোল্ডার { $folder }-এ আছে, এবং কেবল আপনার প্রোফাইলের ভিতরের ফোল্ডারেই লেখা হয়। সেগুলো ব্যবহার করতে, নিজে সেগুলো আপনার সংরক্ষিত গেমগুলো ফোল্ডারে সরিয়ে নিন।
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-unknown =
    { $count ->
        [one] সংরক্ষিত গেমগুলো থেকে { $count }টি ফাইল এর বদলে { $target }-এর ভিতরে _external নামের একটি ফোল্ডারে যাবে, কারণ এই পিসির সংরক্ষিত গেমগুলো ফোল্ডার কোথায় তা জানা যায়নি। এটি ব্যবহার করতে, নিজে এটি আপনার সংরক্ষিত গেমগুলো ফোল্ডারে সরিয়ে নিন।
       *[other] সংরক্ষিত গেমগুলো থেকে { $count }টি ফাইল এর বদলে { $target }-এর ভিতরে _external নামের একটি ফোল্ডারে যাবে, কারণ এই পিসির সংরক্ষিত গেমগুলো ফোল্ডার কোথায় তা জানা যায়নি। সেগুলো ব্যবহার করতে, নিজে সেগুলো আপনার সংরক্ষিত গেমগুলো ফোল্ডারে সরিয়ে নিন।
    }

## Errors when a folder that only administrators may change cannot be made so, or checked.

# { $path } is a folder.
core-privilege-not-a-folder = { $path } একটি লিঙ্ক বা ফাইল, ফোল্ডার নয়
# { $path } is a folder. { $error } is what Windows said, in its own words.
core-privilege-not-made-here = { $path } WinAppMigrator তৈরি করেনি, এবং অন্যরা এর ভিতরের জিনিস পরিবর্তন করতে পারে। একজন প্রশাসকের এটি পরীক্ষা করে সরিয়ে ফেলা উচিত: { $error }
# { $path } is a folder.
core-privilege-others-put-something-in = { $path }-এ এমন কিছু আছে যা WinAppMigrator তৈরি করেনি, এবং অন্যরা তা পরিবর্তন করতে পারে। একজন প্রশাসকের এটি পরীক্ষা করে সরিয়ে ফেলা উচিত
# { $path } is a folder.
core-privilege-became-a-link-in-use = ব্যবহারের সময় { $path } একটি লিঙ্কে পরিণত হয়েছে
# { $path } is a folder.
core-privilege-became-a-link-looked-at = পরীক্ষা করার সময় { $path } একটি লিঙ্কে পরিণত হয়েছে
# { $path } is a file. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-users-cannot-start = ব্যবহারকারীদের { $path } চালু করার অনুমতি দেওয়া যায়নি: { $said }
core-privilege-elevation-unknown = এই প্রসেসটি প্রশাসকের অধিকার নিয়ে চলছে কি না তা জানা যায়নি
# { $path } is a folder. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-cannot-restrict = { $path } কেবল প্রশাসকদের জন্য সীমাবদ্ধ করা যায়নি: { $said }
# { $path } is a folder. { $said } is the folder's permissions as Windows wrote them, and what else it printed.
core-privilege-not-restricted = { $path } কেবল প্রশাসকদের জন্য সীমাবদ্ধ করা যায়নি, এবং এর ড্রাইভ হয়তো অনুমতি সংরক্ষণ করে না: { $said }
# { $path } is a folder.
core-privilege-written-to-first = সুরক্ষিত করার আগেই অন্য একটি প্রসেস { $path }-এ লিখেছে
core-privilege-no-program-data = ProgramData কোথায় তা Windows জানায় না

## Checks of a package's files, and of what an install put in place. Each is said of one file, value, service, or the
## like, which the report names beside it.

# { $path } is the installer's file.
core-verify-no-package = { $path }-এ কোনো প্যাকেজ নেই
# { $path } is a folder.
core-verify-not-unpacked = এই ইনস্টলারের প্যাকেজ { $path }-এ আনপ্যাক করা হয়নি
core-verify-schema-supported = প্যাকেজ ম্যানিফেস্টের স্কিমা সমর্থিত।
core-verify-packaged-file-missing = প্যাকেজে থাকা ফাইলটি নেই।
core-verify-no-package-path = অনুলিপিযোগ্য ফাইলটির প্যাকেজের ভিতরের কোনো আপেক্ষিক পাথ নেই।
core-verify-folder-there = খালি ফোল্ডারটি আছে।
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-unreadable = যাচাই করার জন্য এটি পড়া যায়নি: { $error }
core-verify-restored-file-missing = পুনরুদ্ধার করা ফাইলটি নেই।
# { $expected } and { $actual } are sizes in bytes.
core-verify-size-mismatch = ফাইলের আকার মেলেনি। প্রত্যাশিত { $expected } বাইট, পাওয়া গেছে { $actual } বাইট।
core-verify-no-hash = ম্যানিফেস্টে এই ফাইলের কোনো হ্যাশ রেকর্ড করা নেই, তাই এটি যাচাই করা যায় না।
core-verify-hash-mismatch = SHA-256 হ্যাশ মেলেনি।
core-verify-file-matches = ফাইলের আকার ও হ্যাশ ম্যানিফেস্টের সঙ্গে মিলেছে।
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-payload-failed = লাইসেন্স পেলোড যাচাই ব্যর্থ হয়েছে: { $error }
core-verify-license-payload-missing = প্যাকেজে থাকা লাইসেন্স পেলোডটি নেই।
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-failed = লাইসেন্স স্টোর যাচাই ব্যর্থ হয়েছে: { $error }
core-verify-license-store-file-missing = প্যাকেজে থাকা লাইসেন্স স্টোরের ফাইলটি নেই।
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-path-invalid = প্যাকেজে থাকা লাইসেন্স স্টোরের পাথ অবৈধ: { $error }
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-prerequisite-failed = পূর্বশর্ত রানটাইমের পেলোড যাচাই ব্যর্থ হয়েছে: { $error }
core-verify-prerequisite-missing = সঙ্গে দেওয়া পূর্বশর্ত রানটাইমের পেলোডটি নেই।
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-failed = ড্রাইভার ইমেজ যাচাই ব্যর্থ হয়েছে: { $error }
core-verify-bundled-driver-missing = সঙ্গে দেওয়া ড্রাইভার ইমেজটি নেই।
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-path-invalid = সঙ্গে দেওয়া ড্রাইভারের পাথ অবৈধ: { $error }
core-verify-prerequisite-present = পূর্বশর্ত রানটাইমটি লক্ষ্য পিসিতে আছে।
core-verify-prerequisite-not-detected = পূর্বশর্ত রানটাইমটি পাওয়া যায়নি; এর জন্য হয়তো পুনরায় চালু করা বা নিজে সম্পূর্ণ করা প্রয়োজন (নিশ্চিত ফলাফলের জন্য পুনরুদ্ধারের লগ দেখুন)।
core-verify-prerequisite-not-probed = পূর্বশর্ত রানটাইমটি আছে কি না তা রেজিস্ট্রি থেকে যাচাই করা যায় না; ইনস্টলের জন্য যথাসাধ্য চেষ্টা করা হয়েছে।
core-verify-license-file-missing = পুনরুদ্ধার করা লাইসেন্স ফাইলটি নেই।
core-verify-license-store-restored-missing = পুনরুদ্ধার করা লাইসেন্স স্টোরের ফাইলটি নেই।
core-verify-registry-key-there = রেজিস্ট্রি কীটি আছে।
core-verify-registry-value-matches = রেজিস্ট্রি মানটি আর্কাইভ করা ডেটার সঙ্গে মিলেছে।
core-verify-registry-value-differs = রেজিস্ট্রি মানটি আর্কাইভ করা ডেটার সঙ্গে মেলেনি।
core-verify-firewall-rule-there = Windows ফায়ারওয়ালে এই নিয়মটি আছে।
core-verify-variable-matches = এনভায়রনমেন্ট ভেরিয়েবলের মান আর্কাইভ করা ডেটার সঙ্গে মিলেছে।
core-verify-variable-differs = এনভায়রনমেন্ট ভেরিয়েবলের মান আর্কাইভ করা ডেটার সঙ্গে মেলেনি।
# { $error } is what Windows said, in its own words.
core-verify-variable-unreadable = এনভায়রনমেন্ট ভেরিয়েবলটি নেই বা পড়া যায় না: { $error }
core-verify-service-runs-other = সেবাটি আছে, কিন্তু এটি পুনরুদ্ধার করা প্রোগ্রামটি তার আর্গুমেন্টসহ চালায় না।
core-verify-service-other-account = সেবাটি আছে, কিন্তু এটি প্রত্যাশিত অ্যাকাউন্ট হিসেবে চলে না।
core-verify-service-matches = সেবাটি আছে, এবং এটি পুনরুদ্ধার করা প্রোগ্রামটি তার আর্গুমেন্ট ও অ্যাকাউন্টসহ চালায়।
core-verify-service-missing = সেবাটি নেই।
# { $error } is what Windows said, in its own words.
core-verify-service-unreadable = সেবাটি পড়া যায় না: { $error }
core-verify-driver-other-image = ড্রাইভার সেবাটি পুনরুদ্ধার করা ইমেজটি নির্দেশ করে না।
core-verify-driver-not-registered = পুনরুদ্ধারের পরে সঙ্গে দেওয়া ড্রাইভার সেবাটি নিবন্ধিত নয়।
# { $error } is what Windows said, in its own words.
core-verify-driver-unreadable = ড্রাইভার সেবাটি পড়া যায় না: { $error }
core-verify-driver-image-missing = ড্রাইভার ইমেজটি নেই।
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-unchecked = ড্রাইভার ইমেজটি যাচাই করা যায়নি: { $error }
core-verify-task-matches = নির্ধারিত কার্যটি আছে এবং পুনরুদ্ধার করা প্রোগ্রামগুলো চালায়।
core-verify-task-runs-other = নির্ধারিত কার্যটি আছে, কিন্তু পুনরুদ্ধার করা প্রোগ্রামগুলো চালায় না।
core-verify-task-missing = নির্ধারিত কার্যটি নেই।
# { $error } is what Windows said, in its own words.
core-verify-task-unreadable = নির্ধারিত কার্যের তথ্য জানা যায়নি: { $error }
core-verify-shortcut-matches = শর্টকাটটি আছে, এবং এর লক্ষ্য পাথ পুনরুদ্ধার করা লক্ষ্যের সঙ্গে মিলেছে।
core-verify-shortcut-opens-other = শর্টকাটটি আছে, কিন্তু এর লক্ষ্য পাথ পুনরুদ্ধার করা লক্ষ্যের সঙ্গে মেলেনি।
core-verify-shortcut-missing = শর্টকাটটি নেই।
# { $error } is what Windows said, in its own words.
core-verify-shortcut-unreadable = শর্টকাটের তথ্য জানা যায়নি: { $error }

## Warnings and errors a check gives, which the app and the installer show.

core-warning-verify-schema-unsupported = প্যাকেজ ম্যানিফেস্টের স্কিমা সমর্থিত নয়।
# { $error } is what went wrong, in Windows' or the program's own words.
core-warning-verify-package-file-failed = প্যাকেজের ফাইল যাচাই ব্যর্থ হয়েছে: { $error }
core-warning-verify-folder-missing = খালি ফোল্ডারটি নেই।
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-key-missing = রেজিস্ট্রি কীটি নেই বা পড়া যায় না: { $error }
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-value-missing = রেজিস্ট্রি মানটি নেই বা পড়া যায় না: { $error }
core-warning-verify-registry-kind-missing = রেজিস্ট্রি মানের ধরন উল্লেখ নেই।
core-warning-verify-firewall-rule-missing = Windows ফায়ারওয়ালে এই নিয়মটি নেই।
core-warning-verify-service-scope = সেবা যাচাই HKLM-এর সেবা কনফিগারেশন পড়ে, এবং কোনো সেবা চালু করে না।
core-warning-verify-driver-manual = বিক্রেতার বা Windows-এর অন্তর্নির্মিত এক বা একাধিক ড্রাইভার WinAppMigrator পুনর্বিতরণ করে না; লক্ষ্য পিসিতে বিক্রেতার স্বাক্ষরিত ড্রাইভার প্যাকেজ দিয়ে সেগুলো পুনরায় ইনস্টল করুন।
core-warning-verify-task-scope = নির্ধারিত কার্যের যাচাই কেবল নিবন্ধন ও কমান্ডের উল্লেখ পরীক্ষা করে।

## The verification report written beside a package, verification_report.md. Its headings follow a "#".

core-verify-report-title = WinAppMigrator যাচাই রিপোর্ট
# { $time } is when, as a date and time in UTC, such as 2026-11-15T10:00:00Z.
core-verify-report-verified = যাচাইয়ের সময় (UTC): { $time }
# { $valid } is true or false, which are not translated.
core-verify-report-package-valid = প্যাকেজ বৈধ: { $valid }
# { $valid } is true or false, which are not translated.
core-verify-report-restore-valid = পুনরুদ্ধার বৈধ: { $valid }
core-verify-report-restore-not-run = পুনরুদ্ধার বৈধ: চালানো হয়নি
core-verify-report-items = আইটেম
# { $message } is what a check found, and { $source } the file or setting it looked at.
core-verify-report-item-from = { $message } উৎস: { $source }
core-verify-report-warnings = সতর্কতা
core-verify-report-errors = ত্রুটি
core-verify-report-none = কিছু নেই।
