# Words the core writes for people: warnings, refusals, errors, and the installer's list of what it sets up.
# Message names start with core-.


## Warnings a package records when it is built, which an installer shows later, maybe on another PC, in that PC's
## language.

# { $count } is how many shortcuts.
core-warning-shortcuts-unreadable =
    { $count ->
        [one] এই ড্রাইভের { $count }টি শর্টকাটে কোনো ফাইলের নাম নেই, তাই সেটি এই অ্যাপটি খোলে কি না তা জানা যায়নি।
       *[other] এই ড্রাইভের { $count }টি শর্টকাটে কোনো ফাইলের নাম নেই, তাই সেগুলো এই অ্যাপটি খোলে কি না তা জানা যায়নি।
    }

## Undoing an install: what an uninstall, or a failed install, says it put back, left, or could not undo. Each line is
## shown on its own, in a list.

# { $app_name } is the app's name.
core-rollback-record-damaged = { $app_name }-এর রেকর্ডের একটি অংশ পড়া যায়নি, তাই সেখানে বর্ণিত সবকিছু এখনো যেমন ছিল তেমনই আছে, এবং এর প্রতিস্থাপন করা কোনো ফাইল হয়তো ফিরে আসবে না।
core-rollback-settings-entry-stays = সেটিংসে অ্যাপটির এন্ট্রি রয়ে গেছে, যাতে সেখান থেকে এটি আবার চালানো যায়।
# { $name } is a runtime's name, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-rollback-prerequisite-stays = { $name } ইনস্টল করা হয়েছিল, বা হয়ে থাকতে পারে, এবং এটি রয়ে গেছে, কারণ অন্য প্রোগ্রাম হয়তো এটি ব্যবহার করছে।
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-drive-not-connected = এই ইনস্টল { $folder }-এ যা রেখেছিল তা এমন একটি ড্রাইভে আছে যা সংযুক্ত নেই। ড্রাইভটি সংযুক্ত করুন, তাহলে এটি শেষ করা যাবে।
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-record-part-unread = এই ইনস্টল { $folder }-এ যা রেখেছিল তা রয়ে গেছে, কারণ রেকর্ডের সেই অংশটি পড়া যায়নি।

## Why a file or folder was left, shown after "was left:" in the messages below them. Each is part of a sentence, so
## starts with a small letter and has no full stop.

# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-being-changed = এটি চলার সময় অন্য কিছু { $folder } পরিবর্তন করছিল, তাই এতে আর কিছু করা হয়নি। যা এটি করছে তা বন্ধ করুন, এবং এটি আবার চালান
# { $folder } is a folder; { $error } is what Windows said went wrong.
core-rollback-reason-not-opened = { $folder } খোলা যায়নি: { $error }
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-no-record = { $folder } কোথায় ছিল তার কোনো রেকর্ড নেই
# { $folder } is a folder; { $actual } is the folder it now leads to.
core-rollback-reason-moved = { $folder } এখন { $actual }-এ নিয়ে যায়, ইনস্টলের সময় যেখানে নিয়ে যেত সেখানে নয়
# Why a service was not stopped.
core-rollback-reason-did-not-stop = এটি সময়মতো থামেনি

## Files and folders

# { $path } is a folder; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-folder-left = { $path } ফোল্ডারটি রেখে দেওয়া হয়েছে: { $reason }।
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-removed = { $path } ফোল্ডারটি সরানো হয়েছে
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-holds-others = { $path } ফোল্ডারটি রয়ে গেছে, কারণ এতে এমন ফাইল আছে যা এই ইনস্টল সেখানে রাখেনি।
# { $path } is a folder; { $error } is what Windows said went wrong.
core-rollback-folder-not-removed = { $path } ফোল্ডারটি সরানো যায়নি: { $error }।
# { $path } is a file; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-file-left = { $path } ফাইলটি রেখে দেওয়া হয়েছে: { $reason }।
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-folder-gone = { $path } আগের মতো ফিরিয়ে আনা হচ্ছে না, কারণ এটি যে ফোল্ডারের ভিতরে ছিল সেটি আর নেই।
# { $path } is a file; { $error } is what went wrong.
core-rollback-partial-not-removed = একটি আংশিক ফাইল সরানো যায়নি: { $path } ({ $error })।
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-checked = { $path } যাচাই করা যায়নি: { $error }।
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-removed = { $path } সরানো হয়েছে
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-removed = { $path } সরানো যায়নি: { $error }।
# { $path } is a file this install had replaced, and has put back.
core-rollback-file-put-back = { $path } আগের মতো ফিরিয়ে আনা হয়েছে
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-put-back = { $path } আগের মতো ফিরিয়ে আনা যায়নি: { $error }।
# Why a file could not be put back.
core-rollback-backup-gone = ব্যাকআপটি আর নেই
# { $path } is a file.
core-rollback-no-parent-folder = { $path }-এর কোনো প্যারেন্ট ফোল্ডার নেই

## A file that stays. { $why } says why: still-used (something that stays still uses it), changed (it has changed since
## it was installed), or not-asked (it is someone's data, and they did not ask for it to go). { $path } is the file.

core-rollback-file-stays =
    { $why ->
        [still-used] { $path } রয়ে গেছে, কারণ রয়ে যাওয়া অন্য কিছু এখনো এটি ব্যবহার করে।
        [changed] { $path } রয়ে গেছে, কারণ ইনস্টল করার পরে এটি পরিবর্তিত হয়েছে।
       *[not-asked] { $path } রয়ে গেছে, কারণ এটি সরাতে বলা হয়নি।
    }
# The file it replaced was to be put beside it, under another name, and every name was taken.
core-rollback-file-stays-no-name-beside =
    { $why ->
        [still-used] { $path } রয়ে গেছে, কারণ রয়ে যাওয়া অন্য কিছু এখনো এটি ব্যবহার করে, এবং এটি যে ফাইলটি প্রতিস্থাপন করেছিল সেটি এর পাশে রাখা যায়নি: এর জন্য সম্ভাব্য সব নামই ব্যবহৃত।
        [changed] { $path } রয়ে গেছে, কারণ ইনস্টল করার পরে এটি পরিবর্তিত হয়েছে, এবং এটি যে ফাইলটি প্রতিস্থাপন করেছিল সেটি এর পাশে রাখা যায়নি: এর জন্য সম্ভাব্য সব নামই ব্যবহৃত।
       *[not-asked] { $path } রয়ে গেছে, কারণ এটি সরাতে বলা হয়নি, এবং এটি যে ফাইলটি প্রতিস্থাপন করেছিল সেটি এর পাশে রাখা যায়নি: এর জন্য সম্ভাব্য সব নামই ব্যবহৃত।
    }
# { $name } is the name the file it replaced now has, such as settings.ini.before-install.
core-rollback-file-stays-beside =
    { $why ->
        [still-used] { $path } রয়ে গেছে, কারণ রয়ে যাওয়া অন্য কিছু এখনো এটি ব্যবহার করে। এটি যে ফাইলটি প্রতিস্থাপন করেছিল সেটি এর পাশে { $name } নামে আছে।
        [changed] { $path } রয়ে গেছে, কারণ ইনস্টল করার পরে এটি পরিবর্তিত হয়েছে। এটি যে ফাইলটি প্রতিস্থাপন করেছিল সেটি এর পাশে { $name } নামে আছে।
       *[not-asked] { $path } রয়ে গেছে, কারণ এটি সরাতে বলা হয়নি। এটি যে ফাইলটি প্রতিস্থাপন করেছিল সেটি এর পাশে { $name } নামে আছে।
    }
# { $error } is what went wrong.
core-rollback-file-stays-not-beside =
    { $why ->
        [still-used] { $path } রয়ে গেছে, কারণ রয়ে যাওয়া অন্য কিছু এখনো এটি ব্যবহার করে, এবং এটি যে ফাইলটি প্রতিস্থাপন করেছিল সেটি এর পাশে রাখা যায়নি: { $error }।
        [changed] { $path } রয়ে গেছে, কারণ ইনস্টল করার পরে এটি পরিবর্তিত হয়েছে, এবং এটি যে ফাইলটি প্রতিস্থাপন করেছিল সেটি এর পাশে রাখা যায়নি: { $error }।
       *[not-asked] { $path } রয়ে গেছে, কারণ এটি সরাতে বলা হয়নি, এবং এটি যে ফাইলটি প্রতিস্থাপন করেছিল সেটি এর পাশে রাখা যায়নি: { $error }।
    }

## The registry and environment variables. { $kind } says what { $name } is: key (a registry key, such as
## HKLM\Software\App), value (a registry value, such as HKLM\Software\App\Version), user-variable or machine-variable
## (an environment variable of the user's or of the whole PC, such as APP_HOME), or user-list or machine-list (a
## variable that is a list of folders, such as Path). { $error } is what Windows said went wrong.

# How a registry key's unnamed value is shown, as Registry Editor shows it.
core-registry-default-value-name = (ডিফল্ট)
core-rollback-setting-other-account =
    { $kind ->
        [key] { $name } কীটি রয়ে গেছে, কারণ এটি সেই অ্যাকাউন্টের, যা থেকে এটি ইনস্টল করা হয়েছিল এবং যা এখন সাইন ইন করা নেই।
        [user-variable] ব্যবহারকারীর এনভায়রনমেন্ট ভেরিয়েবল { $name } রয়ে গেছে, কারণ এটি সেই অ্যাকাউন্টের, যা থেকে এটি ইনস্টল করা হয়েছিল এবং যা এখন সাইন ইন করা নেই।
        [machine-variable] মেশিনের এনভায়রনমেন্ট ভেরিয়েবল { $name } রয়ে গেছে, কারণ এটি সেই অ্যাকাউন্টের, যা থেকে এটি ইনস্টল করা হয়েছিল এবং যা এখন সাইন ইন করা নেই।
        [user-list] ব্যবহারকারীর { $name } রয়ে গেছে, কারণ এটি সেই অ্যাকাউন্টের, যা থেকে এটি ইনস্টল করা হয়েছিল এবং যা এখন সাইন ইন করা নেই।
        [machine-list] মেশিনের { $name } রয়ে গেছে, কারণ এটি সেই অ্যাকাউন্টের, যা থেকে এটি ইনস্টল করা হয়েছিল এবং যা এখন সাইন ইন করা নেই।
       *[value] রেজিস্ট্রি মান { $name } রয়ে গেছে, কারণ এটি সেই অ্যাকাউন্টের, যা থেকে এটি ইনস্টল করা হয়েছিল এবং যা এখন সাইন ইন করা নেই।
    }
core-rollback-setting-not-reached =
    { $kind ->
        [key] { $name } কীটিতে পৌঁছানো যায়নি: { $error }।
        [user-variable] ব্যবহারকারীর এনভায়রনমেন্ট ভেরিয়েবল { $name }-এ পৌঁছানো যায়নি: { $error }।
        [machine-variable] মেশিনের এনভায়রনমেন্ট ভেরিয়েবল { $name }-এ পৌঁছানো যায়নি: { $error }।
        [user-list] ব্যবহারকারীর { $name }-এ পৌঁছানো যায়নি: { $error }।
        [machine-list] মেশিনের { $name }-এ পৌঁছানো যায়নি: { $error }।
       *[value] রেজিস্ট্রি মান { $name }-এ পৌঁছানো যায়নি: { $error }।
    }
core-rollback-setting-not-read =
    { $kind ->
        [key] { $name } কীটি পড়া যায়নি: { $error }।
        [user-variable] ব্যবহারকারীর এনভায়রনমেন্ট ভেরিয়েবল { $name } পড়া যায়নি: { $error }।
        [machine-variable] মেশিনের এনভায়রনমেন্ট ভেরিয়েবল { $name } পড়া যায়নি: { $error }।
        [user-list] ব্যবহারকারীর { $name } পড়া যায়নি: { $error }।
        [machine-list] মেশিনের { $name } পড়া যায়নি: { $error }।
       *[value] রেজিস্ট্রি মান { $name } পড়া যায়নি: { $error }।
    }
core-rollback-setting-changed =
    { $kind ->
        [user-variable] ব্যবহারকারীর এনভায়রনমেন্ট ভেরিয়েবল { $name } রয়ে গেছে, কারণ ইনস্টল করার পরে এটি পরিবর্তিত হয়েছে।
        [machine-variable] মেশিনের এনভায়রনমেন্ট ভেরিয়েবল { $name } রয়ে গেছে, কারণ ইনস্টল করার পরে এটি পরিবর্তিত হয়েছে।
        [user-list] ব্যবহারকারীর { $name } রয়ে গেছে, কারণ ইনস্টল করার পরে এটি পরিবর্তিত হয়েছে।
        [machine-list] মেশিনের { $name } রয়ে গেছে, কারণ ইনস্টল করার পরে এটি পরিবর্তিত হয়েছে।
       *[value] রেজিস্ট্রি মান { $name } রয়ে গেছে, কারণ ইনস্টল করার পরে এটি পরিবর্তিত হয়েছে।
    }
# Said when it is back as it was before the install. The capital "The" is as the app has always said it.
core-rollback-setting-put-back =
    { $kind ->
        [user-variable] ব্যবহারকারীর এনভায়রনমেন্ট ভেরিয়েবল { $name } আগের মতো ফিরিয়ে আনা হয়েছে
        [machine-variable] মেশিনের এনভায়রনমেন্ট ভেরিয়েবল { $name } আগের মতো ফিরিয়ে আনা হয়েছে
        [user-list] ব্যবহারকারীর { $name } আগের মতো ফিরিয়ে আনা হয়েছে
        [machine-list] মেশিনের { $name } আগের মতো ফিরিয়ে আনা হয়েছে
       *[value] রেজিস্ট্রি মান { $name } আগের মতো ফিরিয়ে আনা হয়েছে
    }
core-rollback-setting-not-put-back =
    { $kind ->
        [user-variable] ব্যবহারকারীর এনভায়রনমেন্ট ভেরিয়েবল { $name } আগের মতো ফিরিয়ে আনা যায়নি: { $error }।
        [machine-variable] মেশিনের এনভায়রনমেন্ট ভেরিয়েবল { $name } আগের মতো ফিরিয়ে আনা যায়নি: { $error }।
        [user-list] ব্যবহারকারীর { $name } আগের মতো ফিরিয়ে আনা যায়নি: { $error }।
        [machine-list] মেশিনের { $name } আগের মতো ফিরিয়ে আনা যায়নি: { $error }।
       *[value] রেজিস্ট্রি মান { $name } আগের মতো ফিরিয়ে আনা যায়নি: { $error }।
    }
# A registry value the install wrote, which is the user's data, and stays unless they asked for it to go.
core-rollback-value-kept-by-request = রেজিস্ট্রি মান { $name } রয়ে গেছে, কারণ এটি সরাতে বলা হয়নি।
core-rollback-key-holds-others = { $name } কীটি রয়ে গেছে, কারণ এতে এমন কিছু আছে যা এই ইনস্টল সেখানে রাখেনি।
core-rollback-key-removed = { $name } কীটি সরানো হয়েছে
core-rollback-key-not-removed = { $name } কীটি সরানো যায়নি: { $error }।
# { $kind } is user-list or machine-list. The capital "The" is as the app has always said it.
core-rollback-list-entries-removed =
    { $kind ->
        [user-list] ব্যবহারকারীর { $name } থেকে এই ইনস্টলের এন্ট্রিগুলো সরানো হয়েছে
       *[machine-list] মেশিনের { $name } থেকে এই ইনস্টলের এন্ট্রিগুলো সরানো হয়েছে
    }
core-rollback-list-entries-not-removed =
    { $kind ->
        [user-list] ব্যবহারকারীর { $name } থেকে এই ইনস্টলের এন্ট্রিগুলো সরানো যায়নি: { $error }।
       *[machine-list] মেশিনের { $name } থেকে এই ইনস্টলের এন্ট্রিগুলো সরানো যায়নি: { $error }।
    }

## Services and drivers. { $kind } is service or driver; { $name } is its name, such as AppUpdater. { $error } and
## { $reason } are what Windows said went wrong.

core-rollback-service-not-read =
    { $kind ->
        [driver] { $name } ড্রাইভারটি পড়া যায়নি: { $error }।
       *[service] { $name } সেবাটি পড়া যায়নি: { $error }।
    }
core-rollback-service-changed =
    { $kind ->
        [driver] { $name } ড্রাইভারটি রয়ে গেছে, কারণ ইনস্টল করার পরে এটি পরিবর্তিত হয়েছে।
       *[service] { $name } সেবাটি রয়ে গেছে, কারণ ইনস্টল করার পরে এটি পরিবর্তিত হয়েছে।
    }
core-rollback-service-not-stopped =
    { $kind ->
        [driver] { $name } ড্রাইভারটি থামানো যায়নি ({ $reason }), তাই এটি যা ব্যবহার করছে তা পিসি পুনরায় চালু না হওয়া পর্যন্ত থেকে যেতে পারে।
       *[service] { $name } সেবাটি থামানো যায়নি ({ $reason }), তাই এটি যা ব্যবহার করছে তা পিসি পুনরায় চালু না হওয়া পর্যন্ত থেকে যেতে পারে।
    }
core-rollback-service-stopped-for-this =
    { $kind ->
        [driver] এটি করার জন্য { $name } ড্রাইভারটি থামানো হয়েছে। প্রয়োজন হলে এটি আবার চালু করুন।
       *[service] এটি করার জন্য { $name } সেবাটি থামানো হয়েছে। প্রয়োজন হলে এটি আবার চালু করুন।
    }
core-rollback-service-not-removed =
    { $kind ->
        [driver] { $name } ড্রাইভারটি সরানো যায়নি: { $reason }।
       *[service] { $name } সেবাটি সরানো যায়নি: { $reason }।
    }
core-rollback-service-marked-for-removal =
    { $kind ->
        [driver] { $name } ড্রাইভারটি সরানোর জন্য চিহ্নিত করা হয়েছে, এবং পিসি পুনরায় চালু হলে বা যা এটি খুলে রেখেছে তা এটি ছেড়ে দিলে এটি সরে যাবে।
       *[service] { $name } সেবাটি সরানোর জন্য চিহ্নিত করা হয়েছে, এবং পিসি পুনরায় চালু হলে বা যা এটি খুলে রেখেছে তা এটি ছেড়ে দিলে এটি সরে যাবে।
    }
core-rollback-service-removed =
    { $kind ->
        [driver] { $name } ড্রাইভারটি সরানো হয়েছে
       *[service] { $name } সেবাটি সরানো হয়েছে
    }
core-rollback-service-not-restored =
    { $kind ->
        [driver] { $name } ড্রাইভারটি আগের অবস্থায় ফিরিয়ে আনা যায়নি: { $reason }।
       *[service] { $name } সেবাটি আগের অবস্থায় ফিরিয়ে আনা যায়নি: { $reason }।
    }
core-rollback-service-restored =
    { $kind ->
        [driver] { $name } ড্রাইভারের সেটিংস আগের মতো ফিরিয়ে আনা হয়েছে
       *[service] { $name } সেবার সেটিংস আগের মতো ফিরিয়ে আনা হয়েছে
    }
core-rollback-service-not-restarted = { $name } সেবাটি এই ইনস্টলের আগে চলছিল, কিন্তু আবার চালু হয়নি: { $reason }।
# { $account } is the account the service ran as, such as CONTOSO\backup.
core-rollback-service-account-again = { $name } সেবাটি { $account } হিসেবে চলত। এর পাসওয়ার্ড রাখা হয় না, তাই “সেবাগুলো”-তে এর অ্যাকাউন্ট আবার সেট করুন।

## Firewall rules. { $name } is the rule's name; { $error } is what went wrong.

core-rollback-firewall-not-read = { $name } ফায়ারওয়াল নিয়মটি পড়া যায়নি: { $error }।
core-rollback-firewall-says-otherwise = { $name } ফায়ারওয়াল নিয়মটি রয়ে গেছে, কারণ এই নামে যা আছে তাতে অন্য কিছু বলা আছে।
core-rollback-firewall-changed = { $name } ফায়ারওয়াল নিয়মটি রয়ে গেছে, কারণ ইনস্টল করার পরে এটি পরিবর্তিত হয়েছে।
# The capital "The" is as the app has always said it.
core-rollback-firewall-removed = { $name } ফায়ারওয়াল নিয়মটি সরানো হয়েছে
core-rollback-firewall-not-removed = { $name } ফায়ারওয়াল নিয়মটি সরানো যায়নি: { $error }।

## Scheduled tasks. { $path } is a folder of tasks, and { $name } a task with its folder, such as \App\Nightly Update.
## { $error } and { $reason } are what went wrong.

core-rollback-task-folder-holds-others = { $path } কার্য ফোল্ডারটি রয়ে গেছে, কারণ এতে এমন কার্য আছে যা এই ইনস্টল সেখানে রাখেনি।
core-rollback-task-folder-not-removed = { $path } কার্য ফোল্ডারটি সরানো যায়নি: { $error }।
core-rollback-task-not-read = { $name } কার্যটি পড়া যায়নি: { $error }।
core-rollback-task-changed = { $name } কার্যটি রয়ে গেছে, কারণ ইনস্টল করার পরে এটি পরিবর্তিত হয়েছে।
core-rollback-task-removed = { $name } কার্যটি সরানো হয়েছে
core-rollback-task-not-removed = { $name } কার্যটি সরানো যায়নি: { $reason }।
core-rollback-task-put-back = { $name } কার্যটি আগের মতো ফিরিয়ে আনা হয়েছে
core-rollback-task-not-put-back = { $name } কার্যটি আগের মতো ফিরিয়ে আনা যায়নি: { $reason }।

## When an install cannot be undone at all

# { $error } is what went wrong.
core-rollback-record-unreadable = এই ইনস্টলের রেকর্ড পড়া যায়নি, তাই কিছুই পূর্বাবস্থায় ফেরানো হয়নি: { $error }।
# { $error } is what went wrong.
core-rollback-nothing-undone = কিছুই পূর্বাবস্থায় ফেরানো হয়নি: { $error }।
# One app's part of the message below. { $app_name } is the app's name; { $failures } is what could not be undone, a
# sentence or more; { $folder } is the folder that holds the install's record.
core-rollback-earlier-install-stuck = { $app_name }: { $failures } এটি কখনোই পূর্বাবস্থায় ফেরানো না গেলে, একজন প্রশাসক { $folder } সরিয়ে দিয়ে এটি পরিত্যাগ করতে পারেন।
# Why an install did not start. { $details } is one or more of the message above.
core-rollback-earlier-installs-not-undone = শেষ না হওয়া একটি আগের ইনস্টল পুরোপুরি পূর্বাবস্থায় ফেরানো যায়নি, তাই কিছুই ইনস্টল করা হয়নি। এগুলো যা ব্যবহার করছে তা বন্ধ করে আবার চেষ্টা করুন। { $details }
core-rollback-install-busy = এই ইনস্টলটি অন্য একটি ইনস্টলার বা আনইনস্টলার পরিবর্তন করছে

## The summary an uninstall writes on the command line, one line each, with what each says below it. { $count } is how
## many.

core-rollback-count-undone = পূর্বাবস্থায় ফেরানো হয়েছে: { $count }
core-rollback-count-failed = পূর্বাবস্থায় ফেরানো যায়নি: { $count }
core-rollback-count-kept = যেমন ছিল তেমন রাখা হয়েছে: { $count }
core-rollback-count-left-behind = পেছনে রয়ে গেছে: { $count }

## Which Windows a PC or drive has, as the app shows it beside the drive.

# { $name } is the Windows' name, such as Windows 7 Professional Service Pack 1.
core-system-windows-32-bit = { $name }, 32-বিট
# { $name } is the Windows' name, such as Windows 11 Pro.
core-system-windows-64-bit = { $name }, 64-বিট
# A drive's Windows, when it only says it is older than Vista.
core-system-windows-xp-or-older = Windows XP বা তার চেয়ে পুরোনো

## Reading a connected drive, or another account's settings. Each is why it could not be done, and some start with a
## small letter, as they follow other words.

core-system-needs-administrator = সংযুক্ত ড্রাইভ বা অন্য অ্যাকাউন্টের সেটিংস পড়তে প্রশাসকের অধিকার প্রয়োজন
core-system-no-program-data = এই পিসির ProgramData ফোল্ডার পাওয়া যায়নি
# { $path } is a file, such as D:\Windows\System32\config\SOFTWARE.
core-system-not-a-hive-file = এটি কোনো হাইভ ফাইল নয়: { $path }
# { $path } is a file; { $size } is its size in bytes.
core-system-hive-too-large = { $path } { $size } বাইট, যা রেজিস্ট্রি হাইভ হওয়ার জন্য অনেক বড়, অথবা এই ড্রাইভের সব হাইভ মিলিয়ে যত হতে পারে তার চেয়ে বেশি
# { $key } is a registry key, such as Software\Microsoft\Windows\CurrentVersion.
core-system-hive-no-mapping = অফলাইন হাইভে HKLM\{ $key }-এর জন্য কোনো ম্যাপিং নেই
# { $key } is where a drive's registry was to be loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said.
# "reg load" is a command's name.
core-system-hive-not-loaded = reg load { $key } ব্যর্থ হয়েছে: { $error }
# { $key } is where a drive's registry was loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said. "reg
# unload" is a command's name.
core-system-hive-not-unloaded = reg unload { $key } ব্যর্থ হয়েছে: { $error }
# { $drive } is a drive, such as D:\.
core-system-not-a-system-drive = { $drive } কোনো Windows সিস্টেম ড্রাইভ নয় (\Windows ফোল্ডার নেই)।
# { $drive } is a drive, such as C:\. "This PC" is the name of a choice in the app.
core-system-own-system-drive = { $drive } এই পিসির নিজস্ব সিস্টেম ড্রাইভ। এই পিসি পড়তে “এই পিসি” বেছে নিন।
# { $drive } is a drive, such as D:\.
core-system-registry-through-link = { $drive } কোনো Windows সিস্টেম ড্রাইভ নয়: এর রেজিস্ট্রিতে যাওয়ার পথ একটি লিঙ্কের মধ্য দিয়ে যায়।

## Warnings a scan records about whose data it read.

# { $account } is an account's name or id, such as alice or S-1-5-21-...-1001.
core-warning-scan-account-not-on-record = { $account } অ্যাকাউন্টটি এখানে রেকর্ডে নেই, তাই কারও প্রতি-ব্যবহারকারী ডেটা বা সেটিংস অন্তর্ভুক্ত করা হয়নি।
# { $account } is an account's name, such as CONTOSO\alice.
core-warning-scan-signed-in-account-no-profile = এখানে সাইন ইন করা অ্যাকাউন্ট { $account }-এর কোনো প্রোফাইল রেকর্ডে নেই, তাই কারও প্রতি-ব্যবহারকারী ডেটা বা সেটিংস অন্তর্ভুক্ত করা হয়নি।
# { $reason } is why Windows could not say.
core-warning-scan-signed-in-account-unknown = কে সাইন ইন করে আছেন তা জানা যায়নি ({ $reason }), তাই কারও প্রতি-ব্যবহারকারী ডেটা বা সেটিংস অন্তর্ভুক্ত করা হয়নি। কার ডেটা অন্তর্ভুক্ত হবে তা বেছে নিন।
core-warning-scan-last-account-unknown = এই ড্রাইভে বলা নেই শেষবার কে সাইন ইন করেছিলেন, এবং এতে একাধিক অ্যাকাউন্ট আছে, তাই কারও প্রতি-ব্যবহারকারী ডেটা বা সেটিংস অন্তর্ভুক্ত করা হয়নি। কার ডেটা অন্তর্ভুক্ত হবে তা বেছে নিন।
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-settings-unreadable = { $account }-এর সেটিংস পড়া যায়নি, তাই সেগুলো অন্তর্ভুক্ত করা হয়নি: { $error }
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-classes-unreadable = { $account }-এর জন্য নিবন্ধিত COM ক্লাস ও ফাইলের ধরন পড়া যায়নি, তাই সেগুলো অন্তর্ভুক্ত করা হয়নি: { $error }

## Paths a scan did not follow. { $count } is how many; { $paths } is some of them, separated by semicolons; { $more } is
## how many more there are, not shown.

core-warning-scan-drive-paths-not-followed =
    { $count ->
        [one] এই ড্রাইভে রেকর্ড করা একটি পাথ পড়া যায়নি, তাই সেখানকার কিছুই অন্তর্ভুক্ত করা হয়নি। এমন প্রতিটি পাথ হয় অন্য ডিস্কে আছে, নয়তো সম্পূর্ণ পাথ নয়, নয়তো এমন একটি লিঙ্কের মধ্য দিয়ে যায় যা ড্রাইভের বাইরে নিয়ে যেত: { $paths }
       *[other] এই ড্রাইভে রেকর্ড করা কিছু পাথ পড়া যায়নি, তাই সেখানকার কিছুই অন্তর্ভুক্ত করা হয়নি। এমন প্রতিটি পাথ হয় অন্য ডিস্কে আছে, নয়তো সম্পূর্ণ পাথ নয়, নয়তো এমন একটি লিঙ্কের মধ্য দিয়ে যায় যা ড্রাইভের বাইরে নিয়ে যেত: { $paths }
    }
core-warning-scan-drive-paths-not-followed-more = এই ড্রাইভে রেকর্ড করা কিছু পাথ পড়া যায়নি, তাই সেখানকার কিছুই অন্তর্ভুক্ত করা হয়নি। এমন প্রতিটি পাথ হয় অন্য ডিস্কে আছে, নয়তো সম্পূর্ণ পাথ নয়, নয়তো এমন একটি লিঙ্কের মধ্য দিয়ে যায় যা ড্রাইভের বাইরে নিয়ে যেত: { $paths } এবং আরও { $more }টি
core-warning-scan-network-paths-not-followed =
    { $count ->
        [one] এই পিসিতে রেকর্ড করা একটি পাথ পড়া যায়নি, তাই সেখানকার কিছুই অন্তর্ভুক্ত করা হয়নি। এমন প্রতিটি পাথ হয় এই পিসির কোনো ডিস্কে নেই, ফলে সেটি দেখতে গেলে সেখানে সংযোগ করতে হতো, নয়তো এর নাম এমন যা অন্য কিছু খুলবে: { $paths }
       *[other] এই পিসিতে রেকর্ড করা কিছু পাথ পড়া যায়নি, তাই সেখানকার কিছুই অন্তর্ভুক্ত করা হয়নি। এমন প্রতিটি পাথ হয় এই পিসির কোনো ডিস্কে নেই, ফলে সেটি দেখতে গেলে সেখানে সংযোগ করতে হতো, নয়তো এর নাম এমন যা অন্য কিছু খুলবে: { $paths }
    }
core-warning-scan-network-paths-not-followed-more = এই পিসিতে রেকর্ড করা কিছু পাথ পড়া যায়নি, তাই সেখানকার কিছুই অন্তর্ভুক্ত করা হয়নি। এমন প্রতিটি পাথ হয় এই পিসির কোনো ডিস্কে নেই, ফলে সেটি দেখতে গেলে সেখানে সংযোগ করতে হতো, নয়তো এর নাম এমন যা অন্য কিছু খুলবে: { $paths } এবং আরও { $more }টি

## What a scan with none of an account's data leaves out of a package, in the app's list of what was left out. Each
## has a name, and below it why.

core-account-left-out-shortcuts = এই অ্যাকাউন্টের নিজস্ব ফোল্ডারে থাকা শর্টকাট
# { $count } is how many shortcuts.
core-account-left-out-shortcuts-why =
    { $count ->
        [one] { $count }টি শর্টকাট, যা কেবল ব্যবহারকারীর ডেটা বিকল্পের সঙ্গে যায়
       *[other] { $count }টি শর্টকাট, যেগুলো কেবল ব্যবহারকারীর ডেটা বিকল্পের সঙ্গে যায়
    }
core-account-left-out-variables = এই অ্যাকাউন্টের এনভায়রনমেন্ট ভেরিয়েবল
# { $count } is how many variables.
core-account-left-out-variables-why =
    { $count ->
        [one] { $count }টি ভেরিয়েবল, যা কেবল ব্যবহারকারীর ডেটা বিকল্পের সঙ্গে যায়
       *[other] { $count }টি ভেরিয়েবল, যেগুলো কেবল ব্যবহারকারীর ডেটা বিকল্পের সঙ্গে যায়
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-account-left-out-task = { $task } নির্ধারিত কার্যটি
core-account-task-works-on-account-folder = এটি কোনো অ্যাকাউন্টের ফোল্ডারের একটি জায়গায় কাজ করে, বা এর নাম সেই জায়গার নামে, এবং কেবল ব্যবহারকারীর ডেটা বিকল্পের সঙ্গে যায়
core-account-task-runs-from-account-folder = এটি কোনো অ্যাকাউন্টের ফোল্ডার থেকে আর যা চালায়, তা কেবল ব্যবহারকারীর ডেটা বিকল্পের সঙ্গে যায়
# { $service } is a service's name, such as AppUpdater.
core-account-left-out-service = { $service } সেবাটি
core-account-service-works-on-account-folder = এটি কোনো অ্যাকাউন্টের ফোল্ডারের একটি জায়গায় কাজ করে, এবং কেবল ব্যবহারকারীর ডেটা বিকল্পের সঙ্গে যায়

## What a package was found to still say of whose PC it came from, after a build with none of an account's data.

# The parts of a package, as the warning below names them.
core-account-part-files = অ্যাপের নিজস্ব ফাইলগুলোর পাথ
core-account-part-registry = রেজিস্ট্রি মান
core-account-part-settings-entry = সেটিংসে অ্যাপটির এন্ট্রি
core-account-part-main-program = এর মূল প্রোগ্রাম কোথায় আছে
core-account-part-shortcuts = শর্টকাট
core-account-part-tasks = নির্ধারিত কার্য
core-account-part-services = সেবা
core-account-part-variables = এনভায়রনমেন্ট ভেরিয়েবল
# One place in the list below. { $place } is one of the parts above, or a file's name; { $count } is how many times.
core-account-mention-place = { $place } ({ $count })
# { $kind } is what was found: account (an account's name), folder (where an account's folder is), computer (the PC's
# name), or id (an account's id). { $places } is a list of where, from the message above.
core-warning-scan-package-mentions =
    { $kind ->
        [account] এই প্যাকেজের এই জায়গাগুলোতে একটি অ্যাকাউন্টের নাম উল্লেখ আছে: { $places }। এটি কোনো অ্যাকাউন্টের ডেটা ছাড়াই তৈরি করা হয়েছে, এবং যার কাছে ইনস্টলারটি আছে তিনিই এটি পড়তে পারবেন।
        [folder] এই প্যাকেজের এই জায়গাগুলোতে কোনো অ্যাকাউন্টের ফোল্ডার কোথায় তা উল্লেখ আছে: { $places }। এটি কোনো অ্যাকাউন্টের ডেটা ছাড়াই তৈরি করা হয়েছে, এবং যার কাছে ইনস্টলারটি আছে তিনিই এটি পড়তে পারবেন।
        [computer] এই প্যাকেজের এই জায়গাগুলোতে পিসির নাম উল্লেখ আছে: { $places }। এটি কোনো অ্যাকাউন্টের ডেটা ছাড়াই তৈরি করা হয়েছে, এবং যার কাছে ইনস্টলারটি আছে তিনিই এটি পড়তে পারবেন।
       *[id] এই প্যাকেজের এই জায়গাগুলোতে একটি অ্যাকাউন্টের আইডি উল্লেখ আছে: { $places }। এটি কোনো অ্যাকাউন্টের ডেটা ছাড়াই তৈরি করা হয়েছে, এবং যার কাছে ইনস্টলারটি আছে তিনিই এটি পড়তে পারবেন।
    }
# { $count } is how many folders.
core-warning-scan-folders-of-others-unreadable =
    { $count ->
        [one] অ্যাপের বাইরের { $count }টি ফোল্ডার পড়া যায়নি, তাই সেখানে থাকা অ্যাপের ডেটা হয়তো বাদ পড়েছে।
       *[other] অ্যাপের বাইরের { $count }টি ফোল্ডার পড়া যায়নি, তাই সেখানে থাকা অ্যাপের ডেটা হয়তো বাদ পড়েছে।
    }

## What a scan leaves out because it looks like a credential, or is a license with the License option off, in the app's
## list of what was left out. Each has a name, and below it why.

core-secret-why-credential = এটিকে ক্রেডেনশিয়ালের মতো দেখায়
core-secret-why-license-option-off = লাইসেন্স ও অ্যাক্টিভেশন ডেটা বিকল্পটি বন্ধ
# { $count } is how many values of a registry key.
core-secret-why-account-values =
    { $count ->
        [one] { $count }টি মান, যা কেবল ব্যবহারকারীর ডেটা বিকল্পের সঙ্গে যায়
       *[other] { $count }টি মান, যেগুলো কেবল ব্যবহারকারীর ডেটা বিকল্পের সঙ্গে যায়
    }
# Shown in place of a registry value's name when its name itself looks like a password or a key.
core-secret-name-holds-credential = (এমন একটি মান, যার নামেই ক্রেডেনশিয়াল আছে)
# Shown in place of an option's name, for an argument on a command line that has no name.
core-secret-unnamed-argument = ক্রেডেনশিয়াল থাকা একটি আর্গুমেন্ট
# { $option } is an option taken out of a command line, such as /password, or the message above. { $place } says
# where the command line was: service, task, shortcut, opener (what opens a file type), opener-unnamed (the same, of a
# type with no name), or uninstall. { $name } is the service, task, shortcut, file type, or app.
core-secret-taken-from =
    { $place ->
        [service] { $name } যেভাবে চালানো হয় তাতে থাকা { $option }
        [task] { $name } কার্যটি যেভাবে চালানো হয় তাতে থাকা { $option }
        [shortcut] { $name } শর্টকাটে থাকা { $option }
        [opener] { $name } যা দিয়ে খোলে তাতে থাকা { $option }
        [opener-unnamed] একটি ফাইলের ধরন যা দিয়ে খোলে তাতে থাকা { $option }
       *[uninstall] { $name } যেভাবে আনইনস্টল করা হয় তাতে থাকা { $option }
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-secret-task-definition-unreadable = { $task } কার্যটির সংজ্ঞা, যা যাচাইয়ের জন্য পড়া যায়নি
# { $name } is an environment variable's name, such as APP_TOKEN.
core-secret-left-out-variable = এনভায়রনমেন্ট ভেরিয়েবল { $name }
# { $class } is a COM class's id, such as {00000000-0000-0000-0000-000000000000}.
core-secret-left-out-class-server = { $class } ক্লাসটি যা চালায় বলে জানায়
# { $rule } is a firewall rule's name.
core-secret-left-out-firewall-description = { $rule } ফায়ারওয়াল নিয়মের বিবরণ

## Licenses a scan found, as the package's report describes each.

# { $name } is a file's name, such as license.lic.
core-license-detail-file = লাইসেন্স ফাইল পাওয়া গেছে: { $name }
# { $framework } is a licensing product's name, such as FlexLM / FlexNet.
core-license-detail-store = { $framework } লাইসেন্স স্টোর আছে
# { $name } is an environment variable's name, such as LM_LICENSE_FILE.
core-license-detail-variable = লাইসেন্সের এনভায়রনমেন্ট ভেরিয়েবল { $name }
# { $name } is a registry value's name, such as SerialNumber.
core-license-detail-registry-value = রেজিস্ট্রি মানে লাইসেন্সের ডেটা থাকার ইঙ্গিত আছে: { $name }
# Said of a registry value in an account's key, which is not named.
core-license-detail-registry-value-of-an-account = রেজিস্ট্রি মানে লাইসেন্সের ডেটা থাকার ইঙ্গিত আছে
# { $key } is a registry key, such as HKLM\Software\Contoso\License.
core-license-detail-registry-key = রেজিস্ট্রি কীতে লাইসেন্সের ডেটা থাকার ইঙ্গিত আছে: { $key }

## Warnings a scan records about licenses.

core-warning-scan-license-data = লাইসেন্স/অ্যাক্টিভেশন ডেটা পাওয়া গেছে। লাইসেন্স ও অ্যাক্টিভেশন ডেটা বিকল্প চালু থাকলে, ইনস্টলার যা স্থানান্তর করা যায় তা পুনরুদ্ধার করে; পুরো মেশিনের মান ও হার্ডওয়্যারের সঙ্গে আবদ্ধ স্টোর কেবল জানানো হয়। WinAppMigrator কখনো ডিক্রিপ্ট করে না, অ্যাক্টিভেশন এড়িয়ে যায় না, বা ডংগল অনুকরণ করে না।
core-warning-scan-license-machine-bound = কিছু লাইসেন্স মেশিনের সঙ্গে আবদ্ধ (হার্ডওয়্যার ডংগল, মেশিনে লক করা অ্যাক্টিভেশন, বা এনক্রিপ্ট করা)। ইনস্টলার যতটা পারে পুনরুদ্ধার করে; তবুও নতুন পিসিতে এগুলো হয়তো আবার অ্যাক্টিভেট করতে হবে।

## Warnings a scan records about the registry. What could not be read is not named in them: it is kept beside them.

# { $error } is what Windows said went wrong. App Paths is a registry key's name.
core-warning-scan-app-paths-key-unreadable = App Paths-এর চাইল্ড কী পড়া যায়নি: { $error }
core-warning-scan-registry-limit = রেজিস্ট্রি সংগ্রহ নির্ধারিত নিরাপত্তা সীমায় পৌঁছেছে।
core-warning-scan-registry-subtree-limit = রেজিস্ট্রির সাবট্রি সংগ্রহ নির্ধারিত নিরাপত্তা সীমায় পৌঁছেছে।
# { $error } is what Windows said went wrong.
core-warning-scan-registry-key-unreadable = রেজিস্ট্রি কী পড়া যায়নি: { $error }
core-warning-scan-registry-value-too-large = রেজিস্ট্রি মান সংগ্রহের নির্ধারিত আকার-সীমা ছাড়িয়ে গেছে।

## Scheduled tasks

core-warning-scan-tasks-folder-through-link = এই ড্রাইভের নির্ধারিত কার্যের ফোল্ডারটি একটি লিঙ্কের মধ্য দিয়ে যায়, তাই এর কার্যগুলো পড়া হয়নি।
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-warning-scan-task-left-out = { $task } নির্ধারিত কার্যটি বাদ দেওয়া হয়েছে: এটি যে অ্যাকাউন্টের সেটিংস অন্তর্ভুক্ত করা হচ্ছে তা ছাড়া অন্য অ্যাকাউন্ট হিসেবে চলে, অথবা প্রশাসক ছাড়া অন্যরা এর ফাইল পরিবর্তন করতে পারে, অথবা অ্যাপটির এত বেশি কার্য আছে যে সব রাখা যায় না।
# { $error } is what Windows said went wrong.
core-warning-scan-tasks-folder-unreadable = নির্ধারিত কার্যগুলোর একটি অংশ পড়া যায়নি: { $error }
# { $path } is a task's file, such as C:\Windows\System32\Tasks\App\Nightly Sync; { $reason } is one of the reasons
# below, or what Windows said went wrong.
core-warning-scan-task-unreadable = { $path } নির্ধারিত কার্যটি পড়া যায়নি: { $reason }। এটি অ্যাপটি চালালে, তা অন্তর্ভুক্ত করা হয়নি।
# Why a task's file could not be read. { $size } is its size in bytes.
core-task-too-large = এটি { $size } বাইট, যা একটি কার্য হওয়ার জন্য অনেক বড়
# Why a task's file could not be read.
core-task-ends-mid-character = এটি একটি অক্ষরের মাঝখানে শেষ হয়েছে
# Why a task's file could not be read. { $error } says where.
core-task-not-utf16 = এটি বৈধ UTF-16 নয়: { $error }
# Why a task's file could not be read. { $error } says where.
core-task-not-text = এটি বৈধ UTF-8 বা UTF-16 নয়: { $error }
# What a package records of how a task is put back. Register-ScheduledTask is a command's name.
core-task-restore-instructions = আর্কাইভ করা XML সংজ্ঞা দিয়ে Register-ScheduledTask ব্যবহার করে পুনরুদ্ধার করা হয়।

## Reading a program's file. Each is why it could not be read, after the program's name.

# { $reason } is what went wrong.
core-pe-unreadable = এটি পড়া যায়নি: { $reason }
core-pe-headers-misaligned = এর হেডারগুলো একটি প্রোগ্রামের হেডারের জন্য প্রয়োজনীয় সীমানায় নেই
# { $size } is the file's size in bytes.
core-pe-too-large = এটি { $size } বাইট, যা একটি প্রোগ্রাম হওয়ার জন্য অনেক বড়
# PE is the name of the format Windows programs are in.
core-pe-no-headers = এতে কোনো PE হেডার নেই

## Warnings a scan records about what a program needs. Which program is kept beside each.

# { $error } is what went wrong.
core-warning-scan-imports-unreadable = ইমপোর্ট টেবিল পড়া যায়নি: { $error }
# { $error } is what went wrong.
core-warning-scan-import-name-unreadable = একটি ইমপোর্ট ডেসক্রিপ্টরের নাম পড়া যায়নি: { $error }
# { $count } is how many were skipped.
core-warning-scan-legacy-delay-imports =
    { $count ->
        [one] { $count }টি পুরোনো (v1) ডিলে-লোড ডেসক্রিপ্টর এড়িয়ে যাওয়া হয়েছে; কেবল আধুনিক RVA-ভিত্তিক ডিলে ইমপোর্ট পার্স করা হয়।
       *[other] { $count }টি পুরোনো (v1) ডিলে-লোড ডেসক্রিপ্টর এড়িয়ে যাওয়া হয়েছে; কেবল আধুনিক RVA-ভিত্তিক ডিলে ইমপোর্ট পার্স করা হয়।
    }

## The DLLs a program needs. Which DLL or program is kept beside each warning.

# { $path } is a program, such as C:\Program Files\App\app.exe.
core-dll-program-has-no-folder = এক্সিকিউটেবলের কোনো প্যারেন্ট ফোল্ডার নেই: { $path }
# Why a DLL is among the app's files, in the package's list of them.
core-dll-beside-program = নির্বাচিত এক্সিকিউটেবলের পাশে থাকা DLL
# { $error } is what went wrong.
core-warning-scan-dependency-unscanned = পরোক্ষ নির্ভরতা স্ক্যান করা যায়নি: { $error }
# { $name } is a DLL's name, such as d3dx9_43.dll.
core-warning-scan-dependency-not-in-newer-windows = এই অ্যাপটি যে পিসিতে ছিল তার Windows ফোল্ডারে { $name } ছিল, এবং Windows 10 ও 11-এর সঙ্গে এটি আসে না। অন্য কিছু এটি সেখানে ইনস্টল করেছিল, তাই এটি অ্যাপের নিজস্ব ফাইলের অংশ নয় এবং সঙ্গে যাচ্ছে না। যা এটি সরবরাহ করে তা ইনস্টল না করা পর্যন্ত অ্যাপটি হয়তো চালু হবে না।
# { $error } is what went wrong.
core-warning-scan-dependency-not-hashed = নির্ভরতার হ্যাশ নির্ণয় করা যায়নি: { $error }
# SystemRoot and windir are the names of environment variables.
core-warning-scan-windows-folder-unknown = SystemRoot/windir এনভায়রনমেন্ট ভেরিয়েবল পাওয়া যাচ্ছে না; সিস্টেম DLL খোঁজা সীমিত থাকবে।
# { $error } is what went wrong.
core-warning-scan-dependency-architecture-unreadable = নির্ভরতার আর্কিটেকচার পড়া যায়নি: { $error }

## Services, drivers, shortcuts, and environment variables a scan reads. Where it was reading is kept beside each
## warning.

# { $error } is what Windows said went wrong.
core-warning-scan-services-unreadable = সেবার রেজিস্ট্রি কী পড়া যায়নি: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-drivers-unreadable = ড্রাইভার খোঁজার জন্য সেবার রেজিস্ট্রি কী পড়া যায়নি: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-queried = শর্টকাটের তথ্য জানা যায়নি: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-parsed = শর্টকাট অনুসন্ধানের আউটপুট পার্স করা যায়নি: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-environment-unreadable = মেশিনের এনভায়রনমেন্টের রেজিস্ট্রি কী পড়া যায়নি: { $error }

## What a package records of how a driver is put back, which an installer shows. { $kind } is the driver's type as
## Windows names it (kernel, file-system, recognizer, or unknown-type); { $name } is its service's name; { $path } is
## its file. { $signed } is yes when { $signer } names who signed it, and no when that is not known.

core-service-driver-bundled =
    { $signed ->
        [yes] অ্যাপ্লিকেশনের নিজস্ব { $kind } ড্রাইভার “{ $name }” ({ $path }, { $signer } দ্বারা স্বাক্ষরিত)। ইনস্টলার এর .sys ফাইল সঙ্গে নেয় এবং ড্রাইভার সেবাটি স্বয়ংক্রিয়ভাবে পুনরায় নিবন্ধন করে (sc create, প্রশাসকের অধিকার প্রয়োজন), তারপর সেবাটি নিবন্ধিত হয়েছে কি না তা নিশ্চিত করে।
       *[no] অ্যাপ্লিকেশনের নিজস্ব { $kind } ড্রাইভার “{ $name }” ({ $path }, স্বাক্ষর অজানা)। ইনস্টলার এর .sys ফাইল সঙ্গে নেয় এবং ড্রাইভার সেবাটি স্বয়ংক্রিয়ভাবে পুনরায় নিবন্ধন করে (sc create, প্রশাসকের অধিকার প্রয়োজন), তারপর সেবাটি নিবন্ধিত হয়েছে কি না তা নিশ্চিত করে।
    }
core-service-driver-from-vendor =
    { $signed ->
        [yes] বিক্রেতার বা Windows-এর অন্তর্নির্মিত { $kind } ড্রাইভার “{ $name }” ({ $path }, { $signer } দ্বারা স্বাক্ষরিত)। WinAppMigrator তৃতীয় পক্ষের ড্রাইভার পুনর্বিতরণ করে না; অ্যাপটি চালু করার আগে লক্ষ্য পিসিতে বিক্রেতার স্বাক্ষরিত ড্রাইভার প্যাকেজ দিয়ে এটি পুনরায় ইনস্টল করুন।
       *[no] বিক্রেতার বা Windows-এর অন্তর্নির্মিত { $kind } ড্রাইভার “{ $name }” ({ $path }, স্বাক্ষর অজানা)। WinAppMigrator তৃতীয় পক্ষের ড্রাইভার পুনর্বিতরণ করে না; অ্যাপটি চালু করার আগে লক্ষ্য পিসিতে বিক্রেতার স্বাক্ষরিত ড্রাইভার প্যাকেজ দিয়ে এটি পুনরায় ইনস্টল করুন।
    }

## Runtimes an app needs, such as the Visual C++ runtime or .NET: how each is provided, as the Dependencies list says.

core-runtime-handling-built-in = Windows-এ অন্তর্নির্মিত
core-runtime-handling-ships-with-app = অ্যাপের সঙ্গে আসে
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-handling-windows-feature = Windows বৈশিষ্ট্য: { $feature }
core-runtime-handling-included = অন্তর্ভুক্ত
# { $id } is the runtime's name in winget, such as Microsoft.VCRedist.2015+.x64.
core-runtime-handling-winget = winget দিয়ে ইনস্টল করা হয়: { $id }
core-runtime-handling-manual = নিজে ইনস্টল করুন

## Downloading a runtime's installer, when an installer is built. Each is why it could not be, and starts with a small
## letter, as it follows other words. { $url } is a web address; { $error } is what went wrong.

core-runtime-download-not-https = { $url } ডাউনলোড করতে অস্বীকার করা হয়েছে: এটি HTTPS নয়
# TLS is what keeps a download private.
core-runtime-tls-failed = TLS চালু করা ব্যর্থ হয়েছে: { $error }
core-runtime-download-failed = { $url } ডাউনলোড ব্যর্থ হয়েছে: { $error }
core-runtime-download-unreadable = { $url } পড়া ব্যর্থ হয়েছে: { $error }
core-runtime-download-empty = { $url } ডাউনলোডে কোনো ডেটা আসেনি
# { $came } and { $expected } are numbers of bytes.
core-runtime-download-cut-short = { $url } ডাউনলোড মাঝপথে থেমে গেছে: { $expected } বাইটের মধ্যে { $came } বাইট এসেছে

## Installing a runtime: why it was not installed. Each starts with a small letter, as it follows other words.

core-runtime-no-known-installer = এই রানটাইমের জন্য কোনো পরিচিত ইনস্টলার নেই
# { $path } is where in the package, such as prerequisites\vc_redist.x64.exe.
core-runtime-payload-misplaced = প্যাকেজটি { $path }-কে এই রানটাইমের ইনস্টলার হিসেবে উল্লেখ করে, যা এর সঠিক জায়গা নয়, তাই এটি চালানো হয়নি
# { $path } is a file.
core-runtime-payload-missing = সঙ্গে দেওয়া ইনস্টলারটি নেই: { $path }
# { $error } is what went wrong.
core-runtime-payload-path-invalid = সঙ্গে দেওয়া ইনস্টলারের পাথ অবৈধ: { $error }
# { $url } is the web address it can be downloaded from. winget is a program's name.
core-runtime-no-installer-route = সঙ্গে দেওয়া কোনো ইনস্টলার বা winget আইডি নেই; এটি { $url } থেকে সংগ্রহ করুন
# { $error } is what went wrong.
core-runtime-no-temp-folder = এটি আনপ্যাক করার জন্য একটি ফোল্ডার তৈরি করা যায়নি: { $error }
# { $reason } is what the check found.
core-runtime-payload-signature-failed = সঙ্গে দেওয়া ইনস্টলারটি স্বাক্ষর যাচাইয়ে ব্যর্থ হয়েছে এবং চালানো হয়নি: { $reason }
core-runtime-payload-timed-out = সঙ্গে দেওয়া ইনস্টলারটি নির্ধারিত সময়ের মধ্যে শেষ হয়নি, তাই এটি বন্ধ করে দেওয়া হয়েছে
# { $error } is what went wrong.
core-runtime-payload-not-started = সঙ্গে দেওয়া ইনস্টলারটি চালু করা যায়নি: { $error }
core-runtime-installer-killed = ইনস্টলারটি কোনো এক্সিট কোড ছাড়াই বন্ধ করে দেওয়া হয়েছে
# { $code } is the number it ended with, such as 1603.
core-runtime-installer-failed = ইনস্টলারটি { $code } কোড দিয়ে শেষ হয়েছে
# { $feature } is a Windows feature's name, such as NetFx3; { $code } is the number DISM ended with. Windows Features is
# the name of a Windows window. Keep the command as it is.
core-runtime-feature-failed = DISM { $feature } Windows বৈশিষ্ট্যটি চালু করতে পারেনি (এক্সিট { $code }); সম্ভবত পেলোডটি ডাউনলোড করা যায়নি। “Windows বৈশিষ্ট্যগুলো” থেকে এটি চালু করুন, অথবা এটি চালান: `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`
# The same, when DISM gave no number.
core-runtime-feature-failed-no-code = DISM { $feature } Windows বৈশিষ্ট্যটি চালু করতে পারেনি (এক্সিট অজানা); সম্ভবত পেলোডটি ডাউনলোড করা যায়নি। “Windows বৈশিষ্ট্যগুলো” থেকে এটি চালু করুন, অথবা এটি চালান: `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-feature-timed-out = { $feature } Windows বৈশিষ্ট্যটি চালু করা নির্ধারিত সময়ের মধ্যে শেষ হয়নি, তাই এটি বন্ধ করে দেওয়া হয়েছে
# { $feature } is a Windows feature's name; { $error } is what went wrong.
core-runtime-feature-no-dism = { $feature } Windows বৈশিষ্ট্যটি চালু করার জন্য DISM পাওয়া যাচ্ছে না: { $error }

## Installing a runtime with winget. { $id } is its name in winget, such as Microsoft.VCRedist.2015+.x64.

# { $error } is what went wrong.
core-runtime-winget-not-looked-for = winget খোঁজা যায়নি: { $error }
core-runtime-winget-not-installed = winget ইনস্টল করা নেই
# { $path } is where winget is.
core-runtime-winget-misplaced = { $path } সেই জায়গা নয় যেখানে Windows নিজের প্যাকেজ রাখে, তাই এটি চালানো হয়নি
# { $path } is where winget is; { $reason } is what the check found.
core-runtime-winget-signature-failed = { $path } স্বাক্ষর যাচাইয়ে ব্যর্থ হয়েছে এবং চালানো হয়নি: { $reason }
# { $reason } is one of the two messages above, or what went wrong.
core-runtime-winget-unusable = { $id } ইনস্টল করতে winget ব্যবহার করা যায়নি: { $reason }
# { $code } is the number winget ended with.
core-runtime-winget-failed = winget install { $id } { $code } কোড দিয়ে শেষ হয়েছে
core-runtime-winget-failed-no-code = winget install { $id } একটি অজানা কোড দিয়ে শেষ হয়েছে
core-runtime-winget-timed-out = winget install { $id } নির্ধারিত সময়ের মধ্যে শেষ হয়নি, তাই এটি বন্ধ করে দেওয়া হয়েছে
# { $error } is what went wrong.
core-runtime-winget-not-started = { $id } ইনস্টল করার জন্য winget পাওয়া যাচ্ছে না: { $error }

## What told a scan how an app was installed, as the package's report lists it. The names of installers and files are
## as their makers write them.

# { $path } is an installer's record of what it installed, such as C:\Program Files\App\unins000.dat.
core-source-record-not-believed = { $path } ব্যবহার করা হয়নি: প্রশাসক ছাড়া অন্যরাও এটি পরিবর্তন করতে পারে
# { $count } is how many Windows Installer packages the bundle holds.
core-source-burn-bundle =
    { $count ->
        [one] { $count }টি Windows Installer প্যাকেজের Burn বান্ডেল
       *[other] { $count }টি Windows Installer প্যাকেজের Burn বান্ডেল
    }
# { $count } is how many paths Windows Installer recorded.
core-source-msi-database =
    { $count ->
        [one] Windows Installer ডেটাবেস: { $count }টি কম্পোনেন্ট কী পাথ
       *[other] Windows Installer ডেটাবেস: { $count }টি কম্পোনেন্ট কী পাথ
    }
core-source-inno-log = Inno Setup আনইনস্টল লগ (unins000.dat)
# { $count } is how many paths the log records.
core-source-inno-log-paths =
    { $count ->
        [one] Inno Setup আনইনস্টল লগ: রেকর্ড করা { $count }টি পাথ
       *[other] Inno Setup আনইনস্টল লগ: রেকর্ড করা { $count }টি পাথ
    }
core-source-nsis = NSIS (Nullsoft) আনইনস্টলারের স্বাক্ষর
# { $count } is how many paths the log records.
core-source-nsis-log-paths =
    { $count ->
        [one] NSIS AdvUninstLog uninstall.log: রেকর্ড করা { $count }টি পাথ
       *[other] NSIS AdvUninstLog uninstall.log: রেকর্ড করা { $count }টি পাথ
    }
core-source-squirrel = Squirrel প্রতি-ব্যবহারকারী বিন্যাস (Update.exe / app-*)
# { $version } is a folder's name that says the version, such as app-1.2.3.
core-source-squirrel-version = Squirrel-এর বর্তমান সংস্করণ: { $version }
core-source-chocolatey = Chocolatey প্যাকেজের মেটাডেটা
core-source-msix = MSIX / Appx প্যাকেজ
# { $identity } is a package's name, such as Contoso.App.
core-source-appx-identity = AppxManifest-এর পরিচয়: { $identity }
# { $count } is how many folders.
core-source-msix-data =
    { $count ->
        [one] MSIX প্রতি-ব্যবহারকারী প্যাকেজ ডেটা: { $count }টি অবস্থান
       *[other] MSIX প্রতি-ব্যবহারকারী প্যাকেজ ডেটা: { $count }টি অবস্থান
    }
# App Paths is a registry key's name.
core-source-app-paths = App Paths নিবন্ধন

## Why a runtime is needed, as the package's report says it.

# { $program } is a program's file name, such as app.exe; { $dll } is a DLL's, such as msvcp140.dll.
core-runtime-program-imports = { $program } { $dll } ইমপোর্ট করে

## What else of an app's a scan finds in the registry: fonts and certificates, and why each is the app's.

# Why a font is among the app's files, in the package's list of them.
core-registry-font-recorded = অ্যাপের ইনস্টল রেকর্ডে উল্লেখ করা একটি ফন্ট
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-named-for-the-app = এর নাম অ্যাপটির বা এর প্রকাশকের নামে
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-recorded = অ্যাপের ইনস্টল রেকর্ডে এর উল্লেখ আছে

## Who is signed in, for the Whose Data choice. Each of the first six is why it could not be told, and starts with a
## small letter, as it is shown inside other words. An account is shown by its id, such as S-1-5-21-...-1001.

# { $error } is what Windows said.
core-profile-desktop-owner-not-said = এই ডেস্কটপ কার, Windows তা জানায়নি: { $error }
# { $error } is what went wrong.
core-profile-desktop-owner-not-asked = এই ডেস্কটপ কার, তা Windows-কে জিজ্ঞাসা করা যায়নি: { $error }
core-profile-signed-in-account-no-profile = এখানে সাইন ইন করা অ্যাকাউন্ট { $account }-এর কোনো প্রোফাইল রেকর্ডে নেই
core-profile-desktop-not-a-person = এখানকার ডেস্কটপটি { $account }-এর, যা কোনো ব্যক্তির অ্যাকাউন্ট নয়
core-profile-nobody-signed-in-running-account-no-profile = এখানে কেউ সাইন ইন করে নেই, এবং যে অ্যাকাউন্ট এটি চালাচ্ছে, { $account }, তার কোনো প্রোফাইল রেকর্ডে নেই
core-profile-nobody-signed-in-running-not-a-person = এখানে কেউ সাইন ইন করে নেই, এবং যে { $account } এটি চালাচ্ছে, সেটি কোনো ব্যক্তির অ্যাকাউন্ট নয়
# SID is the name Windows gives an account's id.
core-profile-nobody-signed-in-no-sid = এখানে কেউ সাইন ইন করে নেই, এবং এই অ্যাকাউন্টের কোনো SID নেই
# { $accounts } is a list of accounts.
core-profile-several-desktops = এই সেশনে একাধিক অ্যাকাউন্টের ডেস্কটপ আছে: { $accounts }
# Under the Whose Data choice. { $reason } is one of the messages above.
core-profile-signed-in-unknown = কে সাইন ইন করে আছেন তা জানা যায়নি ({ $reason })।
# Under the Whose Data choice, when there are accounts to choose from. { $reason } is one of the messages above.
core-profile-signed-in-unknown-choose = কে সাইন ইন করে আছেন তা জানা যায়নি ({ $reason })। কার ডেটা অন্তর্ভুক্ত হবে তা বেছে নিন।
