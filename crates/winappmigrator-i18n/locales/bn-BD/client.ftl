# Words of WinAppMigrator Client, the console program run on the old PC, the one an app is moving off, so that
# WinAppMigrator on the new PC can read it over the network. Message names start with client-.
#
# Its window is a console, which on Windows 7 is 80 columns wide. Each line break in a message is one on the screen:
# keep every line shorter than 80 characters, and break lines where your language needs to, not only where English
# does.


## Starting

# Shown only in English, in place of a right-to-left language the console cannot show. { $language } is that
# language's name in English, such as Arabic.
client-console-rtl-fallback = এই কনসোল { $language } দেখাতে পারে না, তাই ক্লায়েন্ট ইংরেজি ব্যবহার করছে।
# The client copies itself to a folder in Program Files before it starts. { $error } is what Windows said went wrong.
client-safe-folder-failed =
    WinAppMigrator Client এমন একটি ফোল্ডার থেকে চালু হতে পারেনি, যা কেবল
    প্রশাসকেরা পরিবর্তন করতে পারেন: { $error }
client-press-enter-to-close = এই উইন্ডোটি বন্ধ করতে Enter চাপুন।
# The first line of its screen. { $version } is the client's version, such as 0.6.2.
client-heading = WinAppMigrator Client { $version }
client-intro =
    এটি আপনার নতুন পিসির WinAppMigrator-কে আপনার নেটওয়ার্কের মাধ্যমে এই
    পিসির অ্যাপগুলো পড়তে দেয়। এটি এখানে কিছুই পরিবর্তন করে না: এটি Windows
    ড্রাইভের একটি স্ন্যাপশট পড়ে, এবং আপনি এটি থামালে স্ন্যাপশটটি সরিয়ে দেয়।
# { $error } is why: one of the messages under "Why it stopped" below, or what Windows said.
client-stopped = ক্লায়েন্ট থেমে গেছে: { $error }
# The caretaker is a copy of the client with no window that cleans up after it. { $error } is what Windows said.
client-caretaker-failed =
    এই উইন্ডোটি থামানোর বদলে বন্ধ করে দিলে, পরের বার এখানে ক্লায়েন্ট
    চালানো পর্যন্ত স্ন্যাপশট ও ফায়ারওয়াল নিয়মগুলো থেকে যাবে ({ $error })।
# Shown while the snapshot is taken, followed on the same line by client-snapshot-taken. Keep the quotes: they keep
# the space before it.
client-snapshot-taking = Windows ড্রাইভের একটি স্ন্যাপশট নেওয়া হচ্ছে...{ " " }
client-snapshot-taken = সম্পন্ন।
# The old PC answers the new one's search for it on the network. { $error } is what Windows said went wrong.
client-discovery-failed =
    এই পিসি নিজে থেকে নেটওয়ার্কে দেখা যাবে না ({ $error })।
    এর বদলে নতুন পিসিতে এই পিসির নাম বা ঠিকানা টাইপ করুন।

## This PC and its pairing code. The three lines are lined up in two columns, behind two spaces; keep the quotes, which
## keep those spaces.

# { $computer_name } is this PC's name, such as OLD-PC.
client-this-pc = { "  " }এই পিসি:       { $computer_name }
# { $windows_name } is this PC's Windows, such as Windows 7 Professional Service Pack 1. { $bits } is 32 or 64.
client-windows = { "  " }Windows:       { $windows_name }, { $bits }-বিট
# { $code } is six digits in two groups, such as 482 913.
client-pairing-code = { "  " }পেয়ারিং কোড:  { $code }
# Network PC... is the button of that name in WinAppMigrator: use the name it has there in your language.
client-pairing-steps =
    আপনার নতুন পিসির WinAppMigrator-এ নেটওয়ার্ক পিসি... বেছে নিন এবং এই
    কোডটি টাইপ করুন। যে অ্যাপটি সরাচ্ছেন সেটি আগে বন্ধ করুন, যাতে এর
    ফাইলগুলো অ্যাপটি যেভাবে রেখে গেছে ঠিক সেভাবেই থাকে।
    থামাতে এখানে Enter চাপুন বা এই উইন্ডোটি বন্ধ করুন।

## While it runs. { $from } is the network address of the PC that connected, such as 192.168.1.20, or "unknown" when
## it could not be told, which the [unknown] variant words.

client-paired =
    { $from ->
        [unknown] অন্য একটি পিসির
       *[address] { $from }-এর
    } সঙ্গে পেয়ার হয়েছে। সেটি প্রয়োজনীয় সবকিছু পড়ছে...
# { $count } is how many requests it answered.
client-builder-done =
    { $from ->
        [unknown] অন্য একটি পিসির
       *[address] { $from }-এর
    } কাজ শেষ হয়েছে ({ $count ->
        [one] { $count }টি অনুরোধের উত্তর দেওয়া হয়েছে
       *[other] { $count }টি অনুরোধের উত্তর দেওয়া হয়েছে
    })।
# { $error } is what went wrong, often in Windows' words.
client-link-ended =
    { $from ->
        [unknown] অন্য একটি পিসির
       *[address] { $from }-এর
    } সঙ্গে সংযোগ বিচ্ছিন্ন হয়েছে: { $error }
client-wrong-code =
    { $from ->
        [unknown] অন্য একটি পিসি
       *[address] { $from }
    } এমন একটি কোড দিয়ে চেষ্টা করেছে, যা এখানকার কোড নয়।
# { $error } is what went wrong, often in Windows' words.
client-pair-failed =
    { $from ->
        [unknown] অন্য একটি পিসি
       *[address] { $from }
    } পেয়ার করতে পারেনি: { $error }
client-too-many-wrong-codes =
    অনেকবার ভুল কোড দিয়ে চেষ্টা করা হয়েছে, তাই এখন আর কোনো পিসি এই পিসির
    সঙ্গে পেয়ার করতে পারবে না। আপনার নেটওয়ার্কের কিছু হয়তো কোড অনুমান করার
    চেষ্টা করছে। পেয়ার করতে ক্লায়েন্টটি থামিয়ে আবার চালু করুন, এবং এটি যে
    নতুন কোড দেখায় সেটি টাইপ করুন।
# { $code } is six digits in two groups, such as 482 913.
client-new-code = অনেকবার ভুল কোড দেওয়া হয়েছে। নতুন পেয়ারিং কোড: { $code }

## Stopping

client-stopping = থামানো হচ্ছে: স্ন্যাপশট ও ফায়ারওয়াল নিয়মগুলো সরানো হচ্ছে...
client-stop-done = সম্পন্ন।

## Why it stopped, shown after "The client stopped: " (client-stopped), and written to follow it.

client-already-running = ক্লায়েন্টটি এই পিসিতে ইতিমধ্যে চলছে, অথবা বন্ধ করে দেওয়া একটি ক্লায়েন্টের রেখে যাওয়া জিনিস পরিষ্কার করছে। সেটিই ব্যবহার করুন, অথবা কিছুক্ষণ অপেক্ষা করে এটি আবার চালু করুন।
# { $drive } is the drive Windows is on, such as C:. { $details } is what Windows said, in its own words.
client-snapshot-failed = Windows { $drive }-এর স্ন্যাপশট তৈরি করেনি: { $details }
# { $details } is what Windows Firewall said, in its own words.
client-firewall-failed = Windows ফায়ারওয়াল ক্লায়েন্টকে সংযোগ গ্রহণ করতে দেয়নি: { $details }

## What it takes, shown for --help. The options, such as --lang, are typed as they are and are not translated; keep
## their descriptions lined up.

client-usage =
    WinAppMigrator Client এই বিকল্পগুলো গ্রহণ করে:
    
      --lang <ভাষা>      Windows-এর প্রদর্শন ভাষার বদলে এই ভাষা ব্যবহার করে,
                         যেমন it-IT, যদি ক্লায়েন্টে সেটি থাকে।
      --code <কোড>       এই ছয় অঙ্কের পেয়ারিং কোড দিয়ে শুরু করে, এমন
                         স্ক্রিপ্টের জন্য যা নতুন পিসিতেও WinAppMigrator চালু করে।
      --help             এটি দেখায়।
