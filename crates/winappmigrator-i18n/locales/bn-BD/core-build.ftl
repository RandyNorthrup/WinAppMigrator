# Words the core writes for people while it builds a package from an installed app: what a package records for an
# installer to show later, errors shown in the app, the list of what goes in and what is left out, opening a disk
# image or another PC over the network, and what an installer says when the package it carries cannot be read.
# Message names start with core-.


## Scanning an app, before a package is built. Shown in the app when a scan cannot go on.

# { $path } is the app's program, such as D:\Program Files\Acme\Acme.exe, on a drive taken from another PC.
core-scan-program-not-on-the-drive = { $path } একটি লিঙ্কের মধ্য দিয়ে যায় বা সংযুক্ত ড্রাইভে নেই, তাই এটি পড়া হয়নি।
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-not-on-this-pc = { $path } এই পিসির কোনো ডিস্কে নেই, অথবা এর নাম এমন যা অন্য কিছু খুলবে, তাই এটি পড়া হয়নি।
# { $path } is the app's program.
core-scan-program-someone-elses = { $path } অন্য একটি অ্যাকাউন্টের ফোল্ডারে আছে বা এই অ্যাকাউন্টের নয়, এবং এই অ্যাপের নিজস্ব রেকর্ডেও বলা নেই যে এটি সেখানে থাকার কথা, তাই এটি পড়া হয়নি।
# { $app_name } is the app's name, as the list of apps calls it.
core-scan-no-executable = { $app_name }-এর জন্য কোনো এক্সিকিউটেবল পাওয়া যায়নি। কোনো ড্রাইভার, রানটাইম বা অ্যাড-ইনের প্যাকেজ করার মতো নিজস্ব প্রোগ্রাম না-ও থাকতে পারে।
# { $path } is the file that was picked.
core-scan-not-a-file = স্ক্যানের লক্ষ্যটি কোনো ফাইল নয়: { $path }
# Names something left out of a package, in a list of what was left out and why. { $name } is the variable's name,
# such as ACME_LICENSE_KEY.
core-scan-environment-variable-item = এনভায়রনমেন্ট ভেরিয়েবল { $name }

## Folders and parts that might also be the app's, offered in a list to tick. Each says why it is offered.

core-near-starts-with-app-name = এর নাম এই অ্যাপের নাম দিয়ে শুরু হয়।
core-near-has-app-name = এর নামে এই অ্যাপের নাম আছে।
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-starts-with-publisher = এর নাম প্রকাশকের নাম, { $publisher }, দিয়ে শুরু হয়।
# { $folder } is a folder's name, such as Program Files.
core-near-hidden-part-beside = অ্যাপের তালিকা থেকে লুকানো একটি অংশ, যা { $folder }-এ এই অ্যাপের পাশে ইনস্টল করা আছে।
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-hidden-part-publisher = অ্যাপের তালিকা থেকে লুকানো একটি অংশ, একই প্রকাশক { $publisher }-এর।

## What a scan found, as a person chooses from it: the folders that go in, and what is left out, and why

# Why a folder cannot be left out, beside its number of files.
core-footprint-locked-app-folder = এগুলো সবসময় মূল প্রোগ্রামের সঙ্গে যায়
core-footprint-locked-main-program = এতে মূল প্রোগ্রাম আছে, যা সবসময় অন্তর্ভুক্ত হয়
core-footprint-locked-needed = এতে এমন জিনিস আছে যা ছাড়া মূল প্রোগ্রাম চালু হতে পারে না
core-footprint-locked-font = এটি এর ফন্টের সঙ্গে যায়, “যা একে Windows-এর সঙ্গে যুক্ত করে”-এর অধীনে
# Where a folder is. "AppData" and "ProgramData" are the names of Windows folders; keep them as they are.
core-footprint-place-roaming = আপনার প্রোফাইলের সেটিংস (AppData)
core-footprint-place-local = আপনার প্রোফাইলে এই পিসির জন্য সেটিংস (AppData)
core-footprint-place-program-data = পিসির সবাই মিলে ব্যবহার করে (ProgramData)
core-footprint-place-documents = আপনার নথিগুলোতে
core-footprint-place-saved-games = আপনার সংরক্ষিত গেমগুলোতে
core-footprint-place-public-documents = সর্বজনীন নথিগুলোতে
core-footprint-place-app-folder = অ্যাপের ফোল্ডারে
# { $folder } is a folder, such as C:\Users\Sam\AppData\Roaming.
core-footprint-place-in = { $folder }-এ
# A row for the files directly in the app's folder. { $folder } is that folder's name, such as Acme.
core-footprint-files-directly-in = সরাসরি { $folder }-এ থাকা ফাইল
# Headings over what is left out, and why each item is.
core-footprint-licenses-left-out = এই ইনস্টলারে নেই, কারণ লাইসেন্স ও অ্যাক্টিভেশন ডেটা বিকল্পটি বন্ধ
core-footprint-font-stays-behind = এটি থেকে যায়।
core-footprint-drivers-left-out = Windows বা কোনো বিক্রেতার ড্রাইভার, যা পুনর্বিতরণ করা যায় না, তাই অন্য পিসিতে সেগুলো পুনরায় ইনস্টল করুন
core-footprint-driver-reinstall = এর বিক্রেতার নিজস্ব প্যাকেজ থেকে এটি পুনরায় ইনস্টল করুন।
# Why something else is left out along with what the person left out. { $service } is a service's name, such as AcmeSync.
core-footprint-needs-left-out = এর { $service } প্রয়োজন, যা বাদ দেওয়া হয়েছে
core-footprint-told-to-use-left-out = এটিকে এমন একটি ফোল্ডারের কিছু ব্যবহার করতে বলা আছে, যা আপনি বাদ দিয়েছেন
core-footprint-file-in-left-out = এর ফাইলটি এমন একটি ফোল্ডারে আছে, যা আপনি বাদ দিয়েছেন
core-footprint-program-in-left-out = এটি যে প্রোগ্রামের জন্য, সেটি এমন একটি ফোল্ডারে আছে যা আপনি বাদ দিয়েছেন
core-footprint-names-left-out = এটি এমন একটি ফোল্ডারের নাম উল্লেখ করে, যা আপনি বাদ দিয়েছেন
# Why something is left out, as the installer tells whoever installs it.
core-footprint-builder-left-it-out = যিনি এই ইনস্টলারটি তৈরি করেছেন তিনি এটি বাদ দিয়েছেন
# Something the person left out, by kind and name, such as "Service Acme Updater". { $kind } is the kind, such as
# Service or Scheduled Task; { $title } is its name.
core-footprint-left-out-item = { $kind } { $title }

## Building a package and an installer from a scan. Shown in the app when a build cannot go on.

core-package-scan-of-another-app = এই স্ক্যানটি যে অ্যাপ তৈরি করা হচ্ছে তার নয়, অন্য একটি অ্যাপের, তাই কিছুই তৈরি করা হয়নি
core-package-scan-brought-in-more = এই স্ক্যানে এমন ডেটা এসেছে যা এই বিল্ডে চাওয়া হয়নি, তাই কিছুই তৈরি করা হয়নি
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-package-program-gone = { $path } আর নেই, তাই কিছুই তৈরি করা হয়নি। অ্যাপটি আবার স্ক্যান করুন।
core-package-needs-stub = সেলফ-এক্সট্র্যাক্টিং আর্কাইভের জন্য একটি ইনস্টলার স্টাব প্রয়োজন
# { $reason } is what went wrong.
core-package-staged-file-failed = স্টেজিং ফোল্ডারের একটি ফাইল প্যাকেজে যোগ করা যায়নি: { $reason }
# { $shortfalls } says which drives are short of room and by how much, in sentences of its own.
core-package-not-enough-room = এই ইনস্টলার তৈরি করার মতো যথেষ্ট জায়গা নেই, তাই কিছুই পড়া বা লেখা হয়নি। { $shortfalls }। সেখানে কিছু জায়গা খালি করুন, অথবা অন্য ড্রাইভে একটি আউটপুট ফোল্ডার বেছে নিন।
# { $path } is a file's path, on a drive taken from another PC.
core-package-not-read-off-the-drive = { $path } একটি লিঙ্কের মধ্য দিয়ে যায় বা সংযুক্ত ড্রাইভে নেই, তাই এটি পড়া হয়নি
# { $path } is a file's path.
core-package-not-read-not-local = { $path } এই পিসির কোনো ডিস্কে নেই, তাই এটি পড়া হয়নি
# { $path } is a file's path.
core-package-not-read-other-account = { $path } অন্য একটি অ্যাকাউন্টের ফোল্ডারে নিয়ে যায়, তাই এটি পড়া হয়নি
# { $path } is a file's path, as the PC it came from called it.
core-package-not-on-the-drive = { $path } সংযুক্ত ড্রাইভে নেই, তাই এটি পড়া হয়নি
# { $path } is a file's path.
core-package-backup-copy-no-parent = প্যারেন্ট ফোল্ডার ছাড়া কোনো ফাইলের ব্যাকআপ-অনুলিপি করা যায় না: { $path }
# { $path } is a file's path.
core-package-backup-copy-destination-no-parent = ব্যাকআপ-অনুলিপির গন্তব্যের কোনো প্যারেন্ট ফোল্ডার নেই: { $path }
# robocopy is the name of a Windows command; keep it as it is. { $code } is how it ended, such as Some(8).
core-package-robocopy-failed = robocopy দিয়ে ব্যাকআপ মোডে অনুলিপি ব্যর্থ হয়েছে (এক্সিট { $code })
# Stands for a folder where a file's name would be, as in "I/O failed for temporary package folder: …".
core-package-temporary-folder = অস্থায়ী প্যাকেজ ফোল্ডার
# Stands for the installer being written where a file's name would be, as in "I/O failed for the installer: …".
core-package-the-installer = ইনস্টলার
# { $path } is a file's path, or its name in the package. { $error } is what went wrong with it.
core-package-path-error = { $path }: { $error }
# { $name } is a file's name in the package, such as app/bin/App.exe.
core-package-same-place = প্যাকেজে একই জায়গায় আরেকটি ফাইল আছে: { $name }
# { $name } is a file's name in the package.
core-package-changed-in-protected-folder = প্যাকেজ তৈরির সময় সুরক্ষিত ফোল্ডারে { $name } পরিবর্তিত হয়েছে
core-package-worker-died = ফাইল সংকুচিত করার একটি থ্রেড অপ্রত্যাশিতভাবে থেমে গেছে, তাই কোনো ইনস্টলার লেখা হয়নি
# { $path } is a file's path.
core-package-link-not-read = { $path } একটি লিঙ্ক, তাই এটি পড়া হয়নি
# { $path } is a file's path. { $error } is what went wrong.
core-package-not-compressed = { $path } সংকুচিত করা যায়নি: { $error }
# { $path } is a file's path. { $reason } is what went wrong.
core-package-not-read-back = { $path } আবার পড়ে দেখা যায়নি: { $reason }

## Where an app's files are, and where an installer may write them. Shown when a path is refused.

# { $path } is a folder, such as C:\Program Files\Acme.
core-discovery-root-not-full-path = যে ফোল্ডারের ভিতরে লেখা হবে, সেটি অবশ্যই একটি সম্পূর্ণ পাথ হতে হবে: { $path }
# { $path } is a folder's path.
core-discovery-root-not-a-folder = যে ফোল্ডারের ভিতরে লেখা হবে, সেটি কোনো ফোল্ডার নয়: { $path }
# { $path } is a file's path.
core-discovery-escapes-root = পাথটি নির্ধারিত রুটের বাইরে চলে যায়: { $path }
# { $path } is the link's path.
core-discovery-refusing-to-write-through-link = লিঙ্কের মধ্য দিয়ে লিখতে অস্বীকার করা হয়েছে: { $path }
# { $path } is the link's path.
core-discovery-refusing-link = লিঙ্কের মধ্য দিয়ে যেতে অস্বীকার করা হয়েছে: { $path }
# { $error } is what Windows said went wrong.
core-discovery-refusing-link-because = লিঙ্কের মধ্য দিয়ে যেতে অস্বীকার করা হয়েছে: { $error }
# { $path } is the link's path.
core-discovery-link-to-somewhere-else = { $path } অন্য কোথাও যাওয়ার একটি লিঙ্ক, এবং এটি কোথায় নিয়ে যায় তা অন্যরা হয়তো পরিবর্তন করতে পারে। এটি যে জায়গায় নিয়ে যায়, সেটি ব্যবহার করুন
# { $path } is a folder's path.
core-discovery-no-parent-folder = { $path }-এর কোনো প্যারেন্ট ফোল্ডার নেই
# { $path } is the install location as it was given. Keep the quotes, or use your language's own.
core-discovery-install-location-not-full-path = ইনস্টলের অবস্থান অবশ্যই একটি সম্পূর্ণ পাথ হতে হবে: “{ $path }”
# { $path } is the install location.
core-discovery-install-location-a-link = ইনস্টলের অবস্থানটি অন্য কোথাও যাওয়ার একটি লিঙ্ক: { $path }
# { $path } is a file's path.
core-discovery-device-name = পাথে Windows ডিভাইসের জন্য সংরক্ষিত একটি নাম ব্যবহার করা হয়েছে: { $path }
# { $path } is a file's path.
core-discovery-device-name-on-older-windows = { $path }-এর নাম এমন যে Windows-এর এই সংস্করণ এটিকে ফাইল হিসেবে নয়, ডিভাইস হিসেবে খোলে, তাই এতে কিছুই লেখা হয়নি। এটি Windows 11-এ ইনস্টল করা যায়
# { $path } is a file's path.
core-discovery-unsafe-path = অনিরাপদ পাথ: { $path }
# { $path } is a file's path.
core-discovery-no-package-path = ফাইলটির প্যাকেজের ভিতরের কোনো আপেক্ষিক পাথ নেই: { $path }

## Writing an installer, and reading the package inside one. Most of these are shown by an installer, on the PC it
## runs on, when it cannot read what it carries. "The package" is the app's files and settings that an installer
## carries; "the stub" is the program part of an installer, before the package; "the data file" is the .wamdata file
## that holds the package of a very large installer, beside it.

# Stands for the package where a file's name would be, as in "I/O failed for installer package: …".
core-archive-installer-package = ইনস্টলার প্যাকেজ
# Stands for the installer program built into WinAppMigrator, where a file's name would be.
core-archive-embedded-stub = এমবেড করা ইনস্টলার স্টাব
# { $index } is the number of a file in the package. { $reason } is what went wrong.
core-archive-entry-unreadable = প্যাকেজের { $index } নম্বর এন্ট্রি পড়া যায়নি: { $reason }
# { $name } is a file's name inside the package, such as app/bin/App.exe.
core-archive-entry-unsafe-path = zip এন্ট্রির পাথ অনিরাপদ: { $name }
# { $name } is a file's name inside the package.
core-archive-holds-more = { $name }-এ যতটা থাকার কথা বলা আছে, তার চেয়ে বেশি আছে
# { $name } is a file's name inside the package.
core-archive-name-too-long = এই নামটি zip-এর জন্য খুব দীর্ঘ: { $name }
core-archive-changed-while-copied = স্টাবের পরে বসানোর সময় প্যাকেজটি পরিবর্তিত হয়েছে, তাই কোনো ইনস্টলার লেখা হয়নি
core-archive-stub-not-a-program = এই ইনস্টলারের প্যাকেজের আগের অংশটি কোনো প্রোগ্রাম নয়
core-archive-damaged = এই ইনস্টলারটি ক্ষতিগ্রস্ত, বা তৈরির পরে পরিবর্তন করা হয়েছে, তাই এর প্যাকেজ থেকে কিছুই পড়া হয়নি। যেখানে এটি তৈরি করা হয়েছিল, সেখান থেকে আবার অনুলিপি করুন।
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-missing = এই ইনস্টলার তার প্যাকেজ একই ফোল্ডারে { $name } নামের একটি ফাইলে রাখে, এবং সেই ফাইলটি সেখানে নেই। দুটি ফাইল একসঙ্গে থাকতে হবে: যেখানে ইনস্টলারটি তৈরি করা হয়েছিল সেখান থেকে { $name } এখানে অনুলিপি করুন, তারপর এটি আবার চালান।
# { $name } is the data file's name, such as App_installer.wamdata. { $error } is what Windows said went wrong.
core-archive-data-file-unopened = এই ইনস্টলারের ডেটা ফাইল { $name } খোলা যায়নি: { $error }। দুটি ফাইল একসঙ্গে, এই পিসি পড়তে পারে এমন একটি ফোল্ডারে থাকতে হবে।
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-a-link = { $name } একটি লিঙ্ক বা ফোল্ডার, অথচ এই ইনস্টলারের ডেটা ফাইলকে ঠিক এর পাশে থাকা একটি স্বতন্ত্র ফাইল হতে হবে, তাই এটি থেকে কিছুই পড়া হয়নি। আসল ফাইলটি এখানে অনুলিপি করুন।
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-wrong = { $name } এই ইনস্টলারের ডেটা ফাইল নয়, অথবা এটি ক্ষতিগ্রস্ত বা তৈরির পরে পরিবর্তন করা হয়েছে, তাই এটি থেকে কিছুই পড়া হয়নি। যেখানে ফাইল দুটি তৈরি করা হয়েছিল, সেখান থেকে দুটিই আবার অনুলিপি করুন।
core-archive-stub-signed = ইনস্টলার স্টাবটি স্বাক্ষরিত; এটিকে স্বাক্ষরবিহীন থাকতে হবে, তাই এর বদলে তৈরি হওয়া ইনস্টলারে স্বাক্ষর করুন
# { $size } is a size, such as 4.2 GB.
core-archive-too-big-to-run = ইনস্টলারটির আকার হতো { $size }, এবং Windows 4 GB বা তার বেশি আকারের এক্সিকিউটেবল চালাতে পারে না; কিছুই লেখা হয়নি
core-archive-outside-signature = ইনস্টলারের স্বাক্ষরের বাইরে ডেটা আছে, এবং এতে হয়তো কারসাজি করা হয়েছে
core-archive-footer-length = সেলফ-এক্সট্র্যাক্টিং আর্কাইভের ফুটারের দৈর্ঘ্য অবৈধ
core-archive-seek-outside = প্যাকেজের সীমার বাইরে সিক করার চেষ্টা
# { $path } is a file's path.
core-archive-unsafe-package-path = অনিরাপদ প্যাকেজ পাথ: { $path }
# { $path } is a file's path.
core-archive-empty-package-path = খালি প্যাকেজ পাথ: { $path }

## Opening a disk image of another PC (a .vhd or .vhdx file) to read apps from it

# { $path } is the disk image file, such as D:\Backups\OldPC.vhdx.
core-image-not-attachable = { $path } এমন কোনো ডিস্ক ইমেজ নয় যা Windows সংযুক্ত করতে পারে (.vhd বা .vhdx)
core-image-needs-administrator = ডিস্ক ইমেজ সংযুক্ত করতে প্রশাসকের অধিকার প্রয়োজন
# { $path } is the disk image file the person picked.
core-image-not-a-file = { $path } কোনো ফাইল নয়
# { $path } is the disk image file. "Edit Disk" is the name of a Hyper-V Manager command.
core-image-differencing = { $path } একটি ডিফারেন্সিং ডিস্ক, যাতে কেবল সেই পরিবর্তনগুলো থাকে যা এটি যে ডিস্ক থেকে তৈরি তার পরে হয়েছে, এবং Windows সেই ডিস্কটিও খুলবে, সেটি যেখানে আছে বলে এতে লেখা আছে সেখান থেকেই। প্রথমে Hyper-V-এর Edit Disk দিয়ে এটিকে সেই ডিস্কের সঙ্গে মার্জ করুন, এবং সেটি বেছে নিন
# { $path } is the disk image file.
core-image-unreadable = { $path } পড়া যায় এমন কোনো .vhd বা .vhdx নয়
# ProgramData is the name of a Windows folder; keep it as it is.
core-image-no-program-data = এই পিসির ProgramData ফোল্ডার পাওয়া যায়নি
# The reason given in core-image-could-not-attach when Windows already has the image open.
core-image-already-attached = Windows এটিকে আগে থেকেই সংযুক্ত করে রেখেছে, এবং যা এটিকে সংযুক্ত করেছে তা হয়তো এতে লিখছে। প্রথমে এটি বিচ্ছিন্ন করুন, অথবা এর ড্রাইভটিকে সংযুক্ত ড্রাইভ হিসেবে বেছে নিন
# { $path } is the disk image file. { $reason } is why, such as core-image-already-attached or what Windows said.
core-image-could-not-attach = { $path } সংযুক্ত করা যায়নি: { $reason }
# { $path } is the disk image file.
core-image-no-windows-volume = { $path }-এ Windows আছে এমন কোনো পাঠযোগ্য ভলিউম নেই। BitLocker দিয়ে এনক্রিপ্ট করা ভলিউম প্রথমে আনলক করতে হবে।
# Given as the reason when Windows failed without saying anything.
core-image-windows-did-not-say = Windows কারণ জানায়নি

## Reading another PC over the network, through the client running on it. Shown as why something on that PC
## could not be read. "The client" is WinAppMigrator Client, the small program on the other PC.

core-remote-too-slow = অন্য পিসিটি উত্তর দিতে অনেক বেশি সময় নিয়েছে
core-remote-closed = অন্য পিসির সঙ্গে সংযোগ বন্ধ হয়ে গেছে
core-remote-failed = অন্য পিসির সঙ্গে সংযোগ ব্যর্থ হয়েছে
core-remote-answered-something-else = ক্লায়েন্ট অপ্রত্যাশিত অন্য কিছু উত্তর দিয়েছে
# { $name } is the PC's name or address as the person typed it, such as OLDPC or 192.168.1.20.
core-remote-not-found = নেটওয়ার্কে { $name } পাওয়া যায়নি
core-remote-did-not-say-what-it-is = ক্লায়েন্ট জানায়নি এটি কী
core-remote-ordinary-names-only = কেবল অন্য পিসির রুটের নিচের, সাধারণ নামের পাথ সম্পর্কেই জানতে চাওয়া হয়
core-remote-more-but-none = অন্য পিসিটি বলেছে একটি ফোল্ডারে আরও কিছু আছে, কিন্তু তার কিছুই দেয়নি
# { $count } is the most things one folder may hold, such as 250,000.
core-remote-too-many-listed = অন্য পিসিটি একটি ফোল্ডারে { $count }টির বেশি জিনিস তালিকাভুক্ত করেছে
core-remote-sent-too-much = অন্য পিসিটি একটি ফাইলের যতটা চাওয়া হয়েছিল, তার চেয়ে বেশি পাঠিয়েছে
core-remote-name-not-text = টেক্সট নয় এমন কোনো নাম সম্পর্কে জানতে চাওয়া যায় না
core-remote-below-root-only = কেবল অন্য পিসির রুটের নিচের কোনো পাথ সম্পর্কেই জানতে চাওয়া যায়
core-remote-file-ended-early = অন্য পিসির ফাইলটি প্রত্যাশার আগেই শেষ হয়ে গেছে
core-remote-link-not-followed = অন্য পিসিতে থাকা লিঙ্ক অনুসরণ করা হয় না
core-remote-only-files-opened = অন্য পিসিতে কেবল ফাইলই খোলা হয়
core-remote-listing-said-nothing = তালিকায় এ সম্পর্কে কিছুই বলা নেই
core-remote-change-time-unknown = এটি কখন পরিবর্তন করা হয়েছিল তা জানা নেই
core-remote-seek-before-start = শুরুর আগের অবস্থানে সিক করার চেষ্টা
# { $size } is how many bytes a file is; { $limit } is the most that were expected. Both are numbers.
core-remote-too-large =
    { $size ->
        [one] এটি { $size } বাইট, অথচ সর্বোচ্চ { $limit } বাইট প্রত্যাশিত ছিল
       *[other] এটি { $size } বাইট, অথচ সর্বোচ্চ { $limit } বাইট প্রত্যাশিত ছিল
    }

## Checking that a download is signed by Microsoft. Each is given as the reason a check failed.

# { $error } is what Windows said went wrong.
core-signature-no-powershell = PowerShell চালু করা যায়নি: { $error }
# { $error } is what went wrong reading PowerShell's answer.
core-signature-unreadable = স্বাক্ষরটি পড়া যায়নি: { $error }
# { $signer } is who signed the file, such as Contoso Ltd.
core-signature-not-microsoft = এটি Microsoft দ্বারা নয়, { $signer } দ্বারা স্বাক্ষরিত
core-signature-not-microsoft-unnamed = এটি Microsoft দ্বারা নয়, অন্য কারও দ্বারা স্বাক্ষরিত
# { $status } is Windows' own word for the signature's state, such as HashMismatch or NotSigned, which is not translated.
core-signature-status = এর স্বাক্ষরের অবস্থা { $status }

## Warnings a package records when it is built from an app, which an installer shows later, maybe on another PC, in
## that PC's language. "This app" is the app being packaged.

core-warning-build-program-needs-unreadable = এটি Windows-এর কাছ থেকে কী চায় তা পড়া যায়নি, তাই এটি অন্য পিসিতে চলতে পারবে কি না তা যাচাই করা হবে না।
core-warning-build-external-manifest = বাহ্যিক অ্যাপ্লিকেশন ম্যানিফেস্ট পাওয়া গেছে; সাইড-বাই-সাইড নির্ভরতার ব্যাখ্যা স্থগিত রাখা হয়েছে।
core-warning-build-non-exe = নির্বাচিত ফাইলটির এক্সটেনশন .exe নয়; PE স্ক্যান চলতে থাকবে।
core-warning-build-stand-in-program = সাধারণ কোনো অ্যাপ্লিকেশন এক্সিকিউটেবল পাওয়া যায়নি, তাই এই ফাইলটিকে তার বিকল্প হিসেবে ধরা হয়েছে। আর্কিটেকচার ও নির্ভরতার ফলাফল এই ফাইলটির বিবরণ দেয়, এবং অ্যাপ্লিকেশনটির বিবরণ না-ও দিতে পারে।
core-warning-build-other-apps-files-unknown = Windows Installer-এ অন্য অ্যাপগুলোর যা আছে তার সবকিছু পড়া যায়নি, তাই এই অ্যাপের রেকর্ডে থাকা কোনো ফোল্ডারে অন্য অ্যাপের ফাইল থাকতে পারে, অথচ এখানে তা জানানোর মতো কিছু নেই।
core-warning-build-app-folder-has-another-apps-file = Windows Installer বলছে, এই অ্যাপের প্রোগ্রাম যে ফোল্ডারে আছে সেখানে অন্য একটি অ্যাপের একটি ফাইল আছে, এবং সেটিও সঙ্গে যাচ্ছে, কারণ পুরো ফোল্ডারটিই নেওয়া হয়।
# "App Paths" is the name of a Windows registry key; keep it as it is.
core-warning-build-app-paths-folder-left-out = এই অ্যাপের App Paths এন্ট্রি বলছে এটি নিজের নয় এমন একটি ফোল্ডারে খোঁজে, তাই সেই ফোল্ডারটি সঙ্গে যাচ্ছে না। সেখানে যা আছে তা ছাড়া কোনো পিসিতে এটি হয়তো চালু হবে না।
core-warning-build-uninstaller-folder-left-out = এই অ্যাপের আনইনস্টলার যে ফোল্ডারে আছে সেটি বাদ দেওয়া হয়েছে, কারণ Windows Installer বলছে সেখানে অন্য একটি অ্যাপের ফাইল আছে।
# { $why } is never shown: it chooses the reason. Translate each reason as the end of the sentence.
core-warning-build-recorded-folder-left-out =
    এই অ্যাপের রেকর্ডে থাকা কিছু একটি এমন ফোল্ডারের নাম উল্লেখ করে যা বাদ দেওয়া হয়েছে, কারণ { $why ->
        [unreadable] এর পাথ ড্রাইভের বাইরে চলে যায় বা একটি লিঙ্কের মধ্য দিয়ে যায়, তাই এটি নিরাপদে পড়া যায় না
        [missing] এটি সেখানে নেই
        [link] এটি একটি লিঙ্কের মধ্য দিয়ে যায়
        [short-name] এর আসল নাম যেভাবে, এটি সেভাবে লেখা নেই
        [another-account] এটি অন্য একটি অ্যাকাউন্টের ফোল্ডারে আছে
        [one-account] এই অ্যাপটি একটি অ্যাকাউন্টের জন্য ইনস্টল করা হয়েছিল, এবং এটি সেই অ্যাকাউন্টের নয়
        [shared] এতে সবার ভাগ করা জিনিস আছে, অথবা এটি Windows-এর অংশ
        [generic-name] এর নাম থেকে বোঝা যায় না এটি কার
        [publisher] এর নাম কেবল প্রকাশকের নামে, এবং এতে প্রকাশকের অন্য পণ্যও থাকতে পারে
        [holds-the-app] এতে এই অ্যাপের নিজস্ব ফোল্ডার আছে, এবং আরও কিছু থাকতে পারে
        [another-apps-name] এটি অন্য একটি অ্যাপের নামে পরিচিত
        [another-apps-record] এর ইনস্টল রেকর্ড অন্য একটি অ্যাপের
        [in-another-apps-folder] এটি অন্য একটি অ্যাপের ফোল্ডারে আছে
       *[another-apps] এটি অন্য একটি অ্যাপের
    }।
core-warning-build-app-folder-holds-another-app = রেকর্ড অনুযায়ী এই পিসির অন্য একটি অ্যাপ এই অ্যাপের ফোল্ডারের ভিতরে আছে, এবং সেটিও সঙ্গে যাচ্ছে, কারণ পুরো ফোল্ডারটিই নেওয়া হয়।
# "Program Files (x86)" is the name of a Windows folder; keep it as Windows calls it.
core-warning-build-from-32-bit-windows = এই অ্যাপটি 32-বিট Windows-এ ছিল। 64-বিট Windows যেভাবে কোনো 32-বিট অ্যাপের রেজিস্ট্রি কী সাজিয়ে রাখে, এর রেজিস্ট্রি কীগুলোও সেভাবে সাজানো হয়েছে, এবং ইনস্টলার এর জন্য Program Files (x86) প্রস্তাব করে।
# { $dll } is a file's name, such as MSVCP140D.dll.
core-warning-build-debug-runtime = একটি প্রোগ্রামের { $dll } প্রয়োজন, যা Visual C++ রানটাইমের একটি ডিবাগ বিল্ড। Microsoft এগুলোর জন্য কোনো ইনস্টলার দেয় না, এবং এগুলো পুনর্বিতরণের অনুমতি না-ও দিতে পারে, তাই Visual Studio নেই এমন পিসিতে এটি চলবে না।
# { $program } is a program's file name, such as Tool.exe.
core-warning-build-single-file-program = { $program } একটিমাত্র ফাইলে প্যাক করা একটি .NET প্রোগ্রাম, যাতে বলা নেই এর কোন .NET প্রয়োজন, তাই এর জন্য কোনোটিই পরিকল্পনায় রাখা হয়নি। এর কোনো .NET প্রয়োজন হলে, সেটি নেই এমন পিসিতে এটি চালু হবে না।
# { $count } is how many of the app's programs.
core-warning-build-programs-not-inspected =
    { $count ->
        [one] অ্যাপটির { $count }টি প্রোগ্রামের কোন কোন রানটাইম প্রয়োজন তা পরীক্ষা করা হয়নি, কারণ অ্যাপটির প্রোগ্রাম অনেক বেশি বা সেগুলো অনেক বড়।
       *[other] অ্যাপটির { $count }টি প্রোগ্রামের কোন কোন রানটাইম প্রয়োজন তা পরীক্ষা করা হয়নি, কারণ অ্যাপটির প্রোগ্রাম অনেক বেশি বা সেগুলো অনেক বড়।
    }
# { $runtime } is a runtime's name, such as Java Runtime Environment.
core-warning-build-runtime-no-installer = অ্যাপটির { $runtime } প্রয়োজন, এবং এখানে এর কোনো ইনস্টলার নেই, তাই কোনো ইনস্টলার চালানো হবে না। অন্য পিসিতে এটি নিজে ইনস্টল করুন।
# { $why } is why something was left out, such as "its name says what it holds".
core-warning-build-left-out-of-package = প্যাকেজ থেকে বাদ দেওয়া হয়েছে: { $why }।
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-missing-dependency = নির্ভরতা খুঁজে পাওয়া যায়নি: { $name }
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-dependency-only-here = { $name } কেবল এই পিসির সার্চ পাথের মাধ্যমে পাওয়া গেছে, এবং এটি অ্যাপের অংশ নয়, তাই এটি সঙ্গে যাচ্ছে না। এটি নেই এমন পিসিতে অ্যাপটি হয়তো চালু হবে না।
core-warning-build-driver-detected = ড্রাইভার বা কার্নেল উপাদান পাওয়া গেছে। অ্যাপের নিজস্ব ড্রাইভার সঙ্গে দেওয়া হয় এবং পুনরায় নিবন্ধন করা হয়; বিক্রেতার বা Windows-এর অন্তর্নির্মিত ড্রাইভার পুনর্বিতরণ করা হয় না, এবং সেটি বিক্রেতার স্বাক্ষরিত প্যাকেজ থেকে পুনরায় ইনস্টল করতে হবে।
core-warning-build-near-no-longer-offered = এই অ্যাপেরও হতে পারে এমন জিনিসের তালিকায় টিক দেওয়া একটি জিনিস আর সেই তালিকায় নেই, তাই সেটি যোগ করা হয়নি। তালিকাটি আবার দেখুন।
# { $app_name } is the name of a part of the app, as the list of apps calls it. { $why } is never shown: it chooses the
# reason. Translate each reason as the end of the sentence.
core-warning-build-near-part-left-out =
    { $app_name } যা নিজের বলে জানায় তার একটি অংশ বাদ দেওয়া হয়েছে: { $why ->
        [unreadable] এর পাথ ড্রাইভের বাইরে চলে যায় বা একটি লিঙ্কের মধ্য দিয়ে যায়, তাই এটি নিরাপদে পড়া যায় না
        [missing] এটি সেখানে নেই
        [link] এটি একটি লিঙ্কের মধ্য দিয়ে যায়
        [short-name] এর আসল নাম যেভাবে, এটি সেভাবে লেখা নেই
        [another-account] এটি অন্য একটি অ্যাকাউন্টের ফোল্ডারে আছে
        [one-account] এই অ্যাপটি একটি অ্যাকাউন্টের জন্য ইনস্টল করা হয়েছিল, এবং এটি সেই অ্যাকাউন্টের নয়
        [shared] এতে সবার ভাগ করা জিনিস আছে, অথবা এটি Windows-এর অংশ
        [generic-name] এর নাম থেকে বোঝা যায় না এটি কার
        [publisher] এর নাম কেবল প্রকাশকের নামে, এবং এতে প্রকাশকের অন্য পণ্যও থাকতে পারে
        [holds-the-app] এতে এই অ্যাপের নিজস্ব ফোল্ডার আছে, এবং আরও কিছু থাকতে পারে
        [another-apps-name] এটি অন্য একটি অ্যাপের নামে পরিচিত
        [another-apps-record] এর ইনস্টল রেকর্ড অন্য একটি অ্যাপের
        [in-another-apps-folder] এটি অন্য একটি অ্যাপের ফোল্ডারে আছে
       *[another-apps] এটি অন্য একটি অ্যাপের
    }।
# { $app_name } is the name of a part of the app, as the list of apps calls it.
core-warning-build-near-keys-left-out = { $app_name }-এর ইনস্টল রেকর্ডে তালিকাভুক্ত রেজিস্ট্রি কীগুলো বাদ দেওয়া হয়েছে, কারণ সেই রেকর্ডে অন্য অ্যাপের জিনিসেরও নাম আছে।
# "Windows keeps what an app wrote for one account" is Windows' VirtualStore folder.
core-warning-build-virtual-store-program = কোনো অ্যাপ একটি অ্যাকাউন্টের জন্য যা লেখে, Windows তা যে ফোল্ডারে রাখে, সেখানে একটি প্রোগ্রাম ছিল। Windows কখনো সেখানে প্রোগ্রাম রাখে না, তাই অন্য কিছু সেটি রেখেছে, এবং সেটি বাদ দেওয়া হয়েছে।
# { $path } is a file or folder, such as C:\Users\Sam\AppData\Roaming\Acme.
core-warning-build-file-left-out = { $path } বাদ দেওয়া হয়েছে: এটি কোনো সংযুক্ত ড্রাইভের একটি লিঙ্কের মধ্য দিয়ে যায়, অথবা এটি যে অ্যাকাউন্টের ডেটা অন্তর্ভুক্ত করা হচ্ছে সেটি ছাড়া অন্য একটি অ্যাকাউন্টের।
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as D:\Sam\Documents.
core-warning-build-user-folder-not-on-drive = { $user }-এর { $folder } ফোল্ডারটি { $path }-এ ছিল, যা এই ড্রাইভে নেই, তাই সেখানে যা আছে তা অন্তর্ভুক্ত করা হয়নি।
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as \\server\home\Sam\Documents.
core-warning-build-user-folder-not-on-this-pc = { $user }-এর { $folder } ফোল্ডারটি { $path }-এ ছিল, যা এই পিসির কোনো ডিস্কে নেই, তাই সেখানে যা আছে তা অন্তর্ভুক্ত করা হয়নি।
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-base-unreadable = { $path } পড়া যায়নি ({ $error }); সেখানে রাখা অ্যাপ্লিকেশন ডেটা অন্তর্ভুক্ত করা হয়নি।
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-folder-unreadable = { $path } পড়া যায়নি ({ $error }); এর ভিতরের অ্যাপ্লিকেশন ডেটা হয়তো বাদ পড়েছে।
# { $error } is what went wrong.
core-warning-build-entry-unreadable = ফাইল সিস্টেমের একটি এন্ট্রি পড়া যায়নি: { $error }
# { $count } is how many links. { $names } lists some of them. { $more } is how many more there are, which are not
# named; when it is 0, the sentence ends after the list.
core-warning-build-links-not-followed =
    { $count ->
        [one] এই ফোল্ডারে { $count }টি লিঙ্ক, বা অন্য কিছু খুলবে এমন নাম, অনুসরণ করা হয়নি, তাই সেটি যা নির্দেশ করে তা ইনস্টলারে নেই: { $names }
       *[other] এই ফোল্ডারে { $count }টি লিঙ্ক, বা অন্য কিছু খুলবে এমন নাম, অনুসরণ করা হয়নি, তাই সেগুলো যা নির্দেশ করে তা ইনস্টলারে নেই: { $names }
    }{ $more ->
        [0] ।
       *[other] , এবং আরও { $more }টি।
    }
# { $count } is how many folders. { $depth } is how many levels down a folder can be. { $names } lists some of them.
# { $more } is how many more there are, which are not named; when it is 0, the sentence ends after the list.
core-warning-build-folders-too-deep =
    { $count ->
        [one] { $count }টি ফোল্ডার { $depth } স্তরের চেয়ে বেশি গভীরে আছে, তাই এর ভিতরে যা আছে তা ইনস্টলারে নেই: { $names }
       *[other] { $count }টি ফোল্ডার { $depth } স্তরের চেয়ে বেশি গভীরে আছে, তাই সেগুলোর ভিতরে যা আছে তা ইনস্টলারে নেই: { $names }
    }{ $more ->
        [0] ।
       *[other] , এবং আরও { $more }টি।
    }
# { $error } is what went wrong.
core-warning-build-metadata-unreadable = ফাইলের মেটাডেটা পড়া যায়নি: { $error }
core-warning-build-reserved-name = এই ফাইলের নামটি Windows ডিভাইসের জন্য সংরক্ষিত, তাই এটি অনুলিপি করা হয়নি।
# { $error } is what went wrong.
core-warning-build-hash-failed = ফাইলের হ্যাশ নির্ণয় করা যায়নি: { $error }
core-warning-build-cloud-placeholder = কেবল অনলাইনে থাকা ক্লাউড ফাইলের (যেমন OneDrive Files On-Demand) কোনো স্থানীয় ডেটা নেই; ব্যাকআপের আগে এটি পিন বা ডাউনলোড করুন। এটি জানানো হয়েছে, কিন্তু অনুলিপি করা হয়নি।
# { $file_name } is a driver's file name, such as acmeflt.sys.
core-warning-build-driver-file = ড্রাইভার/কার্নেল ফাইল “{ $file_name }” সাধারণ ফাইল হিসেবে অনুলিপি করা হয় না। এটি যদি এই অ্যাপের নিবন্ধিত কোনো ড্রাইভার সেবার ফাইল হয়, তবে ইনস্টলার এটি সঙ্গে নেয় এবং ড্রাইভারটি স্বয়ংক্রিয়ভাবে পুনরায় নিবন্ধন করে; অন্যথায় বিক্রেতার স্বাক্ষরিত ড্রাইভার প্যাকেজ দিয়ে এটি পুনরায় ইনস্টল করুন।
core-warning-build-file-gone = অ্যাপটি স্ক্যান করার সময় এটি ছিল, কিন্তু এখন নেই, তাই এটি ইনস্টলারে নেই।
# { $error } is what went wrong.
core-warning-build-file-copy-failed = ফাইলটি প্যাকেজে অনুলিপি করা যায়নি: { $error }
# { $count } is how many files. { $names } lists them.
core-warning-build-not-looked-through =
    { $count ->
        [one] এই প্যাকেজে এমন একটি ফাইল আছে যাতে এটি কার কাছ থেকে এসেছে তার উল্লেখ আছে কি না, তা খতিয়ে দেখা যায়নি: { $names }। এর বিষয়বস্তুতে হয়তো তা বলা আছে।
       *[other] এই প্যাকেজে এমন কিছু ফাইল আছে যেগুলোতে এটি কার কাছ থেকে এসেছে তার উল্লেখ আছে কি না, তা খতিয়ে দেখা যায়নি: { $names }। সেগুলোর বিষয়বস্তুতে হয়তো তা বলা আছে।
    }
# { $count } is the most empty folders an installer makes, such as 10,000.
core-warning-build-empty-folders-truncated = এই অ্যাপে { $count }টির বেশি খালি ফোল্ডার আছে, এবং কেবল ততগুলোই আবার তৈরি করা হবে।
core-warning-build-credential-by-now = প্যাকেজ থেকে বাদ দেওয়া হয়েছে: এখন এটিকে ক্রেডেনশিয়ালের মতো দেখাচ্ছে।
core-warning-build-no-package-path = অনুলিপিযোগ্য ফাইলটির প্যাকেজের ভিতরের কোনো আপেক্ষিক পাথ ছিল না।
# { $file_name } is the runtime installer's file name, such as vc_redist.x64.exe. { $error } is what went wrong.
core-warning-build-prerequisite-path-invalid = সঙ্গে দেওয়া পূর্বশর্ত রানটাইম { $file_name } রাখা যায়নি: { $error }
# { $runtime } is a runtime's name, such as Microsoft Visual C++ 2015+ Redistributable (x64). { $error } is what went
# wrong. winget is the name of a Windows command; keep it as it is.
core-warning-build-prerequisite-download-failed = { $runtime } ডাউনলোড করা যায়নি ({ $error }); ইনস্টলার লক্ষ্য পিসিতে winget দিয়ে চেষ্টা করবে।
# { $runtime } is a runtime's name. { $reason } is why the check failed, such as "it is signed by Contoso Ltd, not
# Microsoft".
core-warning-build-prerequisite-signature-failed = { $runtime }-এর ডাউনলোড স্বাক্ষর যাচাইয়ে ব্যর্থ হয়েছে, তাই এটি সঙ্গে দেওয়া হয়নি: { $reason }। ইনস্টলার লক্ষ্য পিসিতে winget দিয়ে চেষ্টা করবে।
# { $runtime } is a runtime's name. { $error } is what went wrong.
core-warning-build-prerequisite-hash-failed = { $runtime } ডাউনলোড করা হয়েছে, কিন্তু এর হ্যাশ নির্ণয় করা যায়নি ({ $error }); এর বদলে ইনস্টলার লক্ষ্য পিসিতে winget দিয়ে চেষ্টা করবে।
# { $service } is a driver's service name, such as AcmeFlt. { $error } is what went wrong.
core-warning-build-driver-copy-failed = “{ $service }”-এর ড্রাইভার ইমেজ সঙ্গে দেওয়া যায়নি: { $error }
core-warning-build-license-not-copied = লাইসেন্স ডেটা অনুলিপি করা যায়নি (নেই বা পড়া যায় না); কেবল মেটাডেটা হিসেবে রেকর্ড করা হয়েছে।
core-warning-build-license-file-credential = একটি লাইসেন্স ফাইলকে ক্রেডেনশিয়ালের মতো দেখায়, তাই এটি বাদ দেওয়া হয়েছে।
# { $error } is what went wrong.
core-warning-build-license-store-walk-failed = লাইসেন্স স্টোরের একটি অংশের তালিকা তৈরি করা যায়নি: { $error }
core-warning-build-license-store-credential = লাইসেন্স স্টোরের একটি ফাইলকে ক্রেডেনশিয়ালের মতো দেখায়, তাই এটি বাদ দেওয়া হয়েছে।
# { $error } is what went wrong.
core-warning-build-license-store-file-skipped = লাইসেন্স স্টোরের একটি ফাইল এড়িয়ে যাওয়া হয়েছে: { $error }
