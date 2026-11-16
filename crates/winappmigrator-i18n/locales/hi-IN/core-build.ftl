# Words the core writes for people while it builds a package from an installed app: what a package records for an
# installer to show later, errors shown in the app, the list of what goes in and what is left out, opening a disk
# image or another PC over the network, and what an installer says when the package it carries cannot be read.
# Message names start with core-.


## Scanning an app, before a package is built. Shown in the app when a scan cannot go on.

# { $path } is the app's program, such as D:\Program Files\Acme\Acme.exe, on a drive taken from another PC.
core-scan-program-not-on-the-drive = { $path } किसी लिंक से होकर जाता है या कनेक्ट की गई ड्राइव पर नहीं है, इसलिए इसे पढ़ा नहीं गया।
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-not-on-this-pc = { $path } इस PC की किसी डिस्क पर नहीं है, या इसका नाम ऐसा है जो कुछ और खोलेगा, इसलिए इसे पढ़ा नहीं गया।
# { $path } is the app's program.
core-scan-program-someone-elses = { $path } किसी अन्य खाते के फ़ोल्डरों में है या इस खाते का नहीं है, और इस ऐप का अपना रिकॉर्ड यह नहीं कहता कि यह वहाँ होना चाहिए, इसलिए इसे पढ़ा नहीं गया।
# { $app_name } is the app's name, as the list of apps calls it.
core-scan-no-executable = { $app_name } के लिए कोई निष्पादन योग्य फ़ाइल नहीं मिली। हो सकता है कि किसी ड्राइवर, रनटाइम, या ऐड-इन में पैकेज करने लायक अपना कोई प्रोग्राम न हो।
# { $path } is the file that was picked.
core-scan-not-a-file = स्कैन लक्ष्य कोई फ़ाइल नहीं है: { $path }
# Names something left out of a package, in a list of what was left out and why. { $name } is the variable's name,
# such as ACME_LICENSE_KEY.
core-scan-environment-variable-item = परिवेश चर { $name }

## Folders and parts that might also be the app's, offered in a list to tick. Each says why it is offered.

core-near-starts-with-app-name = इसका नाम इस ऐप के नाम से शुरू होता है।
core-near-has-app-name = इसके नाम में इस ऐप का नाम है।
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-starts-with-publisher = इसका नाम प्रकाशक के नाम, { $publisher }, से शुरू होता है।
# { $folder } is a folder's name, such as Program Files.
core-near-hidden-part-beside = ऐप्स की सूची से छिपा एक भाग, जो इस ऐप के बगल में { $folder } में इंस्टॉल है।
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-hidden-part-publisher = ऐप्स की सूची से छिपा एक भाग, जो उसी प्रकाशक, { $publisher }, का है।

## What a scan found, as a person chooses from it: the folders that go in, and what is left out, and why

# Why a folder cannot be left out, beside its number of files.
core-footprint-locked-app-folder = ये हमेशा मुख्य प्रोग्राम के साथ जाते हैं
core-footprint-locked-main-program = इसमें मुख्य प्रोग्राम है, जो हमेशा शामिल होता है
core-footprint-locked-needed = इसमें वह है जिसके बिना मुख्य प्रोग्राम प्रारंभ नहीं हो सकता
core-footprint-locked-font = यह अपने फ़ॉन्ट के साथ जाता है, "Windows से इसे क्या जोड़ता है" के अंतर्गत
# Where a folder is. "AppData" and "ProgramData" are the names of Windows folders; keep them as they are.
core-footprint-place-roaming = आपकी प्रोफ़ाइल में सेटिंग्स (AppData)
core-footprint-place-local = आपकी प्रोफ़ाइल में इस PC के लिए सेटिंग्स (AppData)
core-footprint-place-program-data = PC पर सभी के द्वारा साझा (ProgramData)
core-footprint-place-documents = आपके दस्तावेज़ में
core-footprint-place-saved-games = आपके सहेजे गए गेम में
core-footprint-place-public-documents = सार्वजनिक दस्तावेज़ में
core-footprint-place-app-folder = ऐप के फ़ोल्डर में
# { $folder } is a folder, such as C:\Users\Sam\AppData\Roaming.
core-footprint-place-in = { $folder } में
# A row for the files directly in the app's folder. { $folder } is that folder's name, such as Acme.
core-footprint-files-directly-in = सीधे { $folder } में मौजूद फ़ाइलें
# Headings over what is left out, and why each item is.
core-footprint-licenses-left-out = इस इंस्टॉलर में नहीं हैं, क्योंकि "लाइसेंस और सक्रियण डेटा" विकल्प बंद है
core-footprint-font-stays-behind = यह पीछे रह जाता है।
core-footprint-drivers-left-out = Windows या किसी विक्रेता के ड्राइवर, जिन्हें आगे नहीं दिया जा सकता, इसलिए उन्हें दूसरे PC पर फिर से इंस्टॉल करें
core-footprint-driver-reinstall = इसे इसके विक्रेता के अपने पैकेज से फिर से इंस्टॉल करें।
# Why something else is left out along with what the person left out. { $service } is a service's name, such as AcmeSync.
core-footprint-needs-left-out = इसे { $service } की आवश्यकता है, जिसे छोड़ा गया है
core-footprint-told-to-use-left-out = इसे आपके द्वारा छोड़े गए किसी फ़ोल्डर में मौजूद किसी चीज़ का उपयोग करने के लिए कहा गया है
core-footprint-file-in-left-out = इसकी फ़ाइल आपके द्वारा छोड़े गए फ़ोल्डर में है
core-footprint-program-in-left-out = जिस प्रोग्राम के लिए यह है, वह आपके द्वारा छोड़े गए फ़ोल्डर में है
core-footprint-names-left-out = यह आपके द्वारा छोड़े गए किसी फ़ोल्डर का नाम लेता है
# Why something is left out, as the installer tells whoever installs it.
core-footprint-builder-left-it-out = इस इंस्टॉलर को बनाने वाले ने इसे छोड़ दिया
# Something the person left out, by kind and name, such as "Service Acme Updater". { $kind } is the kind, such as
# Service or Scheduled Task; { $title } is its name.
core-footprint-left-out-item = { $kind } { $title }

## Building a package and an installer from a scan. Shown in the app when a build cannot go on.

core-package-scan-of-another-app = यह स्कैन बनाए जा रहे ऐप का नहीं, बल्कि किसी अन्य ऐप का है, इसलिए कुछ भी नहीं बनाया गया
core-package-scan-brought-in-more = यह स्कैन ऐसा डेटा लाया जिसके लिए इस बिल्ड ने नहीं कहा था, इसलिए कुछ भी नहीं बनाया गया
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-package-program-gone = { $path } अब मौजूद नहीं है, इसलिए कुछ भी नहीं बनाया गया। ऐप को फिर से स्कैन करें।
core-package-needs-stub = सेल्फ़-एक्सट्रैक्टिंग संग्रह के लिए इंस्टॉलर स्टब आवश्यक है
# { $reason } is what went wrong.
core-package-staged-file-failed = स्टेजिंग फ़ोल्डर की एक फ़ाइल पैकेज में नहीं जा सकी: { $reason }
# { $shortfalls } says which drives are short of room and by how much, in sentences of its own.
core-package-not-enough-room = इस इंस्टॉलर को बनाने के लिए पर्याप्त स्थान नहीं है, इसलिए कुछ भी पढ़ा या लिखा नहीं गया। { $shortfalls }। वहाँ कुछ स्थान खाली करें, या किसी अन्य ड्राइव पर आउटपुट फ़ोल्डर चुनें।
# { $path } is a file's path, on a drive taken from another PC.
core-package-not-read-off-the-drive = { $path } किसी लिंक से होकर जाता है या कनेक्ट की गई ड्राइव पर नहीं है, इसलिए इसे पढ़ा नहीं गया
# { $path } is a file's path.
core-package-not-read-not-local = { $path } इस PC की किसी डिस्क पर नहीं है, इसलिए इसे पढ़ा नहीं गया
# { $path } is a file's path.
core-package-not-read-other-account = { $path } किसी अन्य खाते के फ़ोल्डरों में ले जाता है, इसलिए इसे पढ़ा नहीं गया
# { $path } is a file's path, as the PC it came from called it.
core-package-not-on-the-drive = { $path } कनेक्ट की गई ड्राइव पर नहीं है, इसलिए इसे पढ़ा नहीं गया
# { $path } is a file's path.
core-package-backup-copy-no-parent = पैरेंट फ़ोल्डर के बिना किसी फ़ाइल की बैकअप-कॉपी नहीं की जा सकती: { $path }
# { $path } is a file's path.
core-package-backup-copy-destination-no-parent = बैकअप-कॉपी के गंतव्य का कोई पैरेंट फ़ोल्डर नहीं है: { $path }
# robocopy is the name of a Windows command; keep it as it is. { $code } is how it ended, such as Some(8).
core-package-robocopy-failed = robocopy बैकअप-मोड कॉपी विफल रही (निकास { $code })
# Stands for a folder where a file's name would be, as in "I/O failed for temporary package folder: …".
core-package-temporary-folder = अस्थायी पैकेज फ़ोल्डर
# Stands for the installer being written where a file's name would be, as in "I/O failed for the installer: …".
core-package-the-installer = इंस्टॉलर
# { $path } is a file's path, or its name in the package. { $error } is what went wrong with it.
core-package-path-error = { $path }: { $error }
# { $name } is a file's name in the package, such as app/bin/App.exe.
core-package-same-place = पैकेज में किसी अन्य फ़ाइल का स्थान भी यही है: { $name }
# { $name } is a file's name in the package.
core-package-changed-in-protected-folder = पैकेज बनाए जाते समय सुरक्षित फ़ोल्डर में { $name } में परिवर्तन हुआ
core-package-worker-died = फ़ाइलें संपीड़ित करने वाला एक थ्रेड अनपेक्षित रूप से रुक गया, इसलिए कोई इंस्टॉलर नहीं लिखा गया
# { $path } is a file's path.
core-package-link-not-read = { $path } एक लिंक है, इसलिए इसे पढ़ा नहीं गया
# { $path } is a file's path. { $error } is what went wrong.
core-package-not-compressed = { $path } को संपीड़ित नहीं किया जा सका: { $error }
# { $path } is a file's path. { $reason } is what went wrong.
core-package-not-read-back = { $path } को वापस पढ़ा नहीं जा सका: { $reason }

## Where an app's files are, and where an installer may write them. Shown when a path is refused.

# { $path } is a folder, such as C:\Program Files\Acme.
core-discovery-root-not-full-path = जिस फ़ोल्डर के अंतर्गत लिखना है, वह एक पूरा पथ होना चाहिए: { $path }
# { $path } is a folder's path.
core-discovery-root-not-a-folder = जिस फ़ोल्डर के अंतर्गत लिखना है, वह फ़ोल्डर नहीं है: { $path }
# { $path } is a file's path.
core-discovery-escapes-root = पथ इच्छित रूट से बाहर जाता है: { $path }
# { $path } is the link's path.
core-discovery-refusing-to-write-through-link = किसी लिंक के माध्यम से लिखने से इनकार किया जा रहा है: { $path }
# { $path } is the link's path.
core-discovery-refusing-link = किसी लिंक से होकर जाने से इनकार किया जा रहा है: { $path }
# { $error } is what Windows said went wrong.
core-discovery-refusing-link-because = किसी लिंक से होकर जाने से इनकार किया जा रहा है: { $error }
# { $path } is the link's path.
core-discovery-link-to-somewhere-else = { $path } किसी अन्य जगह का लिंक है, और अन्य लोग बदल सकते हैं कि यह कहाँ ले जाता है। उस जगह का उपयोग करें जहाँ यह ले जाता है
# { $path } is a folder's path.
core-discovery-no-parent-folder = { $path } का कोई पैरेंट फ़ोल्डर नहीं है
# { $path } is the install location as it was given. Keep the quotes, or use your language's own.
core-discovery-install-location-not-full-path = इंस्टॉल स्थान एक पूरा पथ होना चाहिए: "{ $path }"
# { $path } is the install location.
core-discovery-install-location-a-link = इंस्टॉल स्थान किसी अन्य जगह का लिंक है: { $path }
# { $path } is a file's path.
core-discovery-device-name = पथ में Windows डिवाइस के लिए आरक्षित नाम का उपयोग किया गया है: { $path }
# { $path } is a file's path.
core-discovery-device-name-on-older-windows = { $path } का नाम ऐसा है जिसे Windows का यह संस्करण फ़ाइल के रूप में नहीं, बल्कि डिवाइस के रूप में खोलता है, इसलिए इसमें कुछ भी नहीं लिखा गया। इसे Windows 11 पर इंस्टॉल किया जा सकता है
# { $path } is a file's path.
core-discovery-unsafe-path = असुरक्षित पथ: { $path }
# { $path } is a file's path.
core-discovery-no-package-path = फ़ाइल का कोई पैकेज-सापेक्ष पथ नहीं है: { $path }

## Writing an installer, and reading the package inside one. Most of these are shown by an installer, on the PC it
## runs on, when it cannot read what it carries. "The package" is the app's files and settings that an installer
## carries; "the stub" is the program part of an installer, before the package; "the data file" is the .wamdata file
## that holds the package of a very large installer, beside it.

# Stands for the package where a file's name would be, as in "I/O failed for installer package: …".
core-archive-installer-package = इंस्टॉलर पैकेज
# Stands for the installer program built into WinAppMigrator, where a file's name would be.
core-archive-embedded-stub = एम्बेड किया गया इंस्टॉलर स्टब
# { $index } is the number of a file in the package. { $reason } is what went wrong.
core-archive-entry-unreadable = पैकेज की प्रविष्टि { $index } पढ़ी नहीं जा सकी: { $reason }
# { $name } is a file's name inside the package, such as app/bin/App.exe.
core-archive-entry-unsafe-path = zip प्रविष्टि का पथ असुरक्षित है: { $name }
# { $name } is a file's name inside the package.
core-archive-holds-more = { $name } में उससे अधिक है जितना यह बताता है
# { $name } is a file's name inside the package.
core-archive-name-too-long = यह नाम zip के लिए बहुत लंबा है: { $name }
core-archive-changed-while-copied = स्टब के बाद रखे जाते समय पैकेज बदल गया, इसलिए कोई इंस्टॉलर नहीं लिखा गया
core-archive-stub-not-a-program = इस इंस्टॉलर में पैकेज से पहले वाला भाग कोई प्रोग्राम नहीं है
core-archive-damaged = यह इंस्टॉलर क्षतिग्रस्त है या बनाए जाने के बाद बदला गया था, इसलिए इसके पैकेज से कुछ भी पढ़ा नहीं गया। इसे वहाँ से फिर से कॉपी करें जहाँ इसे बनाया गया था।
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-missing = यह इंस्टॉलर अपना पैकेज उसी फ़ोल्डर में { $name } नाम की फ़ाइल में रखता है, और वह फ़ाइल वहाँ नहीं है। दोनों फ़ाइलों को साथ रहना होगा: जहाँ इंस्टॉलर बनाया गया था, वहाँ से { $name } को यहाँ कॉपी करें, फिर इसे फिर से चलाएँ।
# { $name } is the data file's name, such as App_installer.wamdata. { $error } is what Windows said went wrong.
core-archive-data-file-unopened = इस इंस्टॉलर की डेटा फ़ाइल, { $name }, नहीं खोली जा सकी: { $error }। दोनों फ़ाइलों को साथ, ऐसे फ़ोल्डर में रहना होगा जिसे यह PC पढ़ सके।
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-a-link = { $name } एक लिंक या फ़ोल्डर है, जबकि इस इंस्टॉलर की डेटा फ़ाइल को इसके ठीक बगल में एक अलग फ़ाइल होना चाहिए, इसलिए इससे कुछ भी पढ़ा नहीं गया। असली फ़ाइल यहाँ कॉपी करें।
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-wrong = { $name } इस इंस्टॉलर की डेटा फ़ाइल नहीं है, या यह क्षतिग्रस्त है या बनाए जाने के बाद बदली गई थी, इसलिए इससे कुछ भी पढ़ा नहीं गया। दोनों फ़ाइलें वहाँ से फिर से कॉपी करें जहाँ उन्हें बनाया गया था।
core-archive-stub-signed = इंस्टॉलर स्टब हस्ताक्षरित है; इसे बिना हस्ताक्षर के रहना चाहिए, इसलिए इसके बजाय तैयार इंस्टॉलर पर हस्ताक्षर करें
# { $size } is a size, such as 4.2 GB.
core-archive-too-big-to-run = इंस्टॉलर { $size } का होगा, और Windows 4 GB या उससे बड़ी निष्पादन योग्य फ़ाइल नहीं चला सकता; कुछ भी नहीं लिखा गया
core-archive-outside-signature = इंस्टॉलर में उसके हस्ताक्षर के बाहर डेटा है, और हो सकता है कि उसके साथ छेड़छाड़ की गई हो
core-archive-footer-length = सेल्फ़-एक्सट्रैक्टिंग संग्रह के फ़ुटर की लंबाई अमान्य है
core-archive-seek-outside = पैकेज के बाहर सीक करने का प्रयास
# { $path } is a file's path.
core-archive-unsafe-package-path = असुरक्षित पैकेज पथ: { $path }
# { $path } is a file's path.
core-archive-empty-package-path = खाली पैकेज पथ: { $path }

## Opening a disk image of another PC (a .vhd or .vhdx file) to read apps from it

# { $path } is the disk image file, such as D:\Backups\OldPC.vhdx.
core-image-not-attachable = { $path } ऐसी डिस्क छवि नहीं है जिसे Windows संलग्न कर सके (.vhd या .vhdx)
core-image-needs-administrator = डिस्क छवि संलग्न करने के लिए व्यवस्थापक अधिकारों की आवश्यकता है
# { $path } is the disk image file the person picked.
core-image-not-a-file = { $path } कोई फ़ाइल नहीं है
# { $path } is the disk image file. "Edit Disk" is the name of a Hyper-V Manager command.
core-image-differencing = { $path } एक डिफ़रेंसिंग डिस्क है, जिसमें केवल वह होता है जो उस डिस्क से बनाए जाने के बाद बदला है, और Windows वह डिस्क भी खोलेगा, इस डिस्क में उसका स्थान जो भी लिखा हो। पहले इसे Hyper-V के Edit Disk से उस डिस्क में मर्ज करें, और फिर वही डिस्क चुनें
# { $path } is the disk image file.
core-image-unreadable = { $path } ऐसी .vhd या .vhdx फ़ाइल नहीं है जिसे पढ़ा जा सके
# ProgramData is the name of a Windows folder; keep it as it is.
core-image-no-program-data = इस PC का ProgramData फ़ोल्डर नहीं मिला
# The reason given in core-image-could-not-attach when Windows already has the image open.
core-image-already-attached = Windows ने इसे पहले से संलग्न किया हुआ है, और जिसने भी इसे संलग्न किया है, वह इसमें लिख रहा हो सकता है। पहले इसे अलग करें, या इसकी ड्राइव को कनेक्ट की गई ड्राइव के रूप में चुनें
# { $path } is the disk image file. { $reason } is why, such as core-image-already-attached or what Windows said.
core-image-could-not-attach = { $path } को संलग्न नहीं किया जा सका: { $reason }
# { $path } is the disk image file.
core-image-no-windows-volume = { $path } में Windows वाला ऐसा कोई वॉल्यूम नहीं है जिसे पढ़ा जा सके। BitLocker से एन्क्रिप्ट किए गए वॉल्यूम को पहले अनलॉक करना होगा।
# Given as the reason when Windows failed without saying anything.
core-image-windows-did-not-say = Windows ने कारण नहीं बताया

## Reading another PC over the network, through the client running on it. Shown as why something on that PC
## could not be read. "The client" is WinAppMigrator Client, the small program on the other PC.

core-remote-too-slow = दूसरे PC ने उत्तर देने में बहुत अधिक समय लिया
core-remote-closed = दूसरे PC से कनेक्शन बंद है
core-remote-failed = दूसरे PC से कनेक्शन विफल रहा
core-remote-answered-something-else = क्लाइंट ने कुछ और उत्तर दिया
# { $name } is the PC's name or address as the person typed it, such as OLDPC or 192.168.1.20.
core-remote-not-found = { $name } नेटवर्क पर नहीं मिला
core-remote-did-not-say-what-it-is = क्लाइंट ने यह नहीं बताया कि वह क्या है
core-remote-ordinary-names-only = केवल दूसरे PC के रूट के नीचे के, सामान्य नामों वाले पथ के बारे में ही पूछा जाता है
core-remote-more-but-none = दूसरे PC ने कहा कि एक फ़ोल्डर में और भी चीज़ें हैं, लेकिन उनमें से कुछ भी नहीं भेजा
# { $count } is the most things one folder may hold, such as 250,000.
core-remote-too-many-listed = दूसरे PC ने एक फ़ोल्डर में { $count } से अधिक चीज़ें सूचीबद्ध कीं
core-remote-sent-too-much = दूसरे PC ने किसी फ़ाइल का उससे अधिक भाग भेजा जितना माँगा गया था
core-remote-name-not-text = ऐसे नाम के बारे में नहीं पूछा जा सकता जो टेक्स्ट नहीं है
core-remote-below-root-only = केवल दूसरे PC के रूट के नीचे के पथ के बारे में ही पूछा जा सकता है
core-remote-file-ended-early = दूसरे PC की फ़ाइल समय से पहले समाप्त हो गई
core-remote-link-not-followed = दूसरे PC पर किसी लिंक का अनुसरण नहीं किया जाता
core-remote-only-files-opened = दूसरे PC पर केवल फ़ाइल ही खोली जाती है
core-remote-listing-said-nothing = सूची में इसके बारे में कुछ नहीं था
core-remote-change-time-unknown = यह कब बदला गया, यह ज्ञात नहीं है
core-remote-seek-before-start = शुरुआत से पहले की स्थिति पर सीक करने का प्रयास
# { $size } is how many bytes a file is; { $limit } is the most that were expected. Both are numbers.
core-remote-too-large =
    { $size ->
        [one] यह { $size } बाइट है, और { $limit } से अधिक की अपेक्षा नहीं थी
       *[other] यह { $size } बाइट है, और { $limit } से अधिक की अपेक्षा नहीं थी
    }

## Checking that a download is signed by Microsoft. Each is given as the reason a check failed.

# { $error } is what Windows said went wrong.
core-signature-no-powershell = PowerShell प्रारंभ नहीं किया जा सका: { $error }
# { $error } is what went wrong reading PowerShell's answer.
core-signature-unreadable = हस्ताक्षर पढ़ा नहीं जा सका: { $error }
# { $signer } is who signed the file, such as Contoso Ltd.
core-signature-not-microsoft = यह Microsoft द्वारा नहीं, बल्कि { $signer } द्वारा हस्ताक्षरित है
core-signature-not-microsoft-unnamed = यह Microsoft द्वारा नहीं, बल्कि किसी और द्वारा हस्ताक्षरित है
# { $status } is Windows' own word for the signature's state, such as HashMismatch or NotSigned, which is not translated.
core-signature-status = इसके हस्ताक्षर की स्थिति { $status } है

## Warnings a package records when it is built from an app, which an installer shows later, maybe on another PC, in
## that PC's language. "This app" is the app being packaged.

core-warning-build-program-needs-unreadable = इसे Windows से क्या चाहिए, यह पढ़ा नहीं जा सका, इसलिए यह जाँचा नहीं जाएगा कि यह दूसरे PC पर चल सकता है या नहीं।
core-warning-build-external-manifest = बाहरी एप्लिकेशन मेनिफ़ेस्ट मिला; साइड-बाय-साइड निर्भरता की व्याख्या स्थगित की गई है।
core-warning-build-non-exe = चुनी गई फ़ाइल .exe एक्सटेंशन का उपयोग नहीं करती; PE स्कैनिंग जारी रहेगी।
core-warning-build-stand-in-program = कोई सामान्य एप्लिकेशन निष्पादन योग्य फ़ाइल नहीं मिली, इसलिए उसकी जगह इस फ़ाइल का उपयोग किया जा रहा है। आर्किटेक्चर और निर्भरता के परिणाम इस फ़ाइल का वर्णन करते हैं, और हो सकता है कि वे एप्लिकेशन का वर्णन न करें।
core-warning-build-other-apps-files-unknown = Windows Installer के पास अन्य ऐप्स के बारे में जो कुछ है, वह सब पढ़ा नहीं जा सका, इसलिए इस ऐप के रिकॉर्ड में दर्ज किसी फ़ोल्डर में किसी अन्य ऐप की फ़ाइलें हो सकती हैं, और यहाँ ऐसा बताने वाला कुछ नहीं होगा।
core-warning-build-app-folder-has-another-apps-file = Windows Installer के अनुसार, जिस फ़ोल्डर में इस ऐप का प्रोग्राम है, उसमें किसी अन्य ऐप की एक फ़ाइल है, और वह भी साथ आती है, क्योंकि वह पूरा फ़ोल्डर लिया जाता है।
# "App Paths" is the name of a Windows registry key; keep it as it is.
core-warning-build-app-paths-folder-left-out = इस ऐप की App Paths प्रविष्टि के अनुसार यह ऐसे फ़ोल्डर में देखता है जो इसका अपना नहीं है, इसलिए वह फ़ोल्डर साथ नहीं आता। हो सकता है कि वहाँ मौजूद चीज़ों के बिना यह किसी PC पर प्रारंभ न हो।
core-warning-build-uninstaller-folder-left-out = जिस फ़ोल्डर में इस ऐप का अनइंस्टॉलर है, उसे छोड़ दिया गया, क्योंकि Windows Installer के अनुसार वहाँ किसी अन्य ऐप की एक फ़ाइल है।
# { $why } is never shown: it chooses the reason. Translate each reason as the end of the sentence.
core-warning-build-recorded-folder-left-out =
    इस ऐप के रिकॉर्ड में मौजूद कोई चीज़ ऐसे फ़ोल्डर का नाम लेती है जिसे छोड़ दिया गया, क्योंकि { $why ->
        [unreadable] इसका पथ ड्राइव से बाहर जाता है या किसी लिंक से होकर जाता है, इसलिए इसे सुरक्षित रूप से पढ़ा नहीं जा सकता
        [missing] यह मौजूद नहीं है
        [link] यह किसी लिंक से होकर जाता है
        [short-name] यह उस तरह नहीं लिखा गया है जैसा इसका वास्तविक नाम है
        [another-account] यह किसी अन्य खाते के फ़ोल्डरों में है
        [one-account] यह ऐप एक खाते के लिए इंस्टॉल किया गया था, और यह फ़ोल्डर उस खाते का नहीं है
        [shared] इसमें साझा चीज़ें हैं, या यह Windows का भाग है
        [generic-name] इसके नाम से पता नहीं चलता कि यह किसका है
        [publisher] इसका नाम केवल प्रकाशक के नाम पर है, और इसमें प्रकाशक के अन्य उत्पाद हो सकते हैं
        [holds-the-app] इसमें इस ऐप का अपना फ़ोल्डर है, और इसमें और भी चीज़ें हो सकती हैं
        [another-apps-name] यह किसी अन्य ऐप के नाम से है
        [another-apps-record] इसका इंस्टॉल रिकॉर्ड किसी अन्य ऐप का है
        [in-another-apps-folder] यह किसी अन्य ऐप के फ़ोल्डर में है
       *[another-apps] यह किसी अन्य ऐप का है
    }।
core-warning-build-app-folder-holds-another-app = रिकॉर्ड के अनुसार इस PC पर एक अन्य ऐप इस ऐप के फ़ोल्डर के अंदर है, और वह भी इसके साथ आता है, क्योंकि वह पूरा फ़ोल्डर लिया जाता है।
# "Program Files (x86)" is the name of a Windows folder; keep it as Windows calls it.
core-warning-build-from-32-bit-windows = यह ऐप 32-बिट Windows पर था। इसकी रजिस्ट्री कुंजियाँ उसी तरह व्यवस्थित की गई हैं जैसे 64-बिट Windows किसी 32-बिट ऐप की कुंजियाँ रखता है, और इंस्टॉलर इसके लिए Program Files (x86) सुझाता है।
# { $dll } is a file's name, such as MSVCP140D.dll.
core-warning-build-debug-runtime = एक प्रोग्राम को { $dll } की आवश्यकता है, जो Visual C++ रनटाइम का डीबग बिल्ड है। Microsoft इनके लिए कोई इंस्टॉलर नहीं देता, और हो सकता है कि इन्हें आगे देने की अनुमति भी न दे, इसलिए यह Visual Studio के बिना किसी PC पर नहीं चलेगा।
# { $program } is a program's file name, such as Tool.exe.
core-warning-build-single-file-program = { $program } एक फ़ाइल में पैक किया गया .NET प्रोग्राम है, जो यह नहीं बताता कि उसे कौन-सा .NET चाहिए, इसलिए इसके लिए कोई .NET नियोजित नहीं है। अगर इसे किसी .NET की आवश्यकता है, तो यह उसके बिना किसी PC पर प्रारंभ नहीं होगा।
# { $count } is how many of the app's programs.
core-warning-build-programs-not-inspected =
    { $count ->
        [one] ऐप के प्रोग्रामों में से { $count } प्रोग्राम की यह जाँच नहीं की गई कि उसे किन रनटाइम की आवश्यकता है, क्योंकि ऐप में बहुत अधिक प्रोग्राम हैं या वे बहुत बड़े हैं।
       *[other] ऐप के प्रोग्रामों में से { $count } प्रोग्रामों की यह जाँच नहीं की गई कि उन्हें किन रनटाइम की आवश्यकता है, क्योंकि ऐप में बहुत अधिक प्रोग्राम हैं या वे बहुत बड़े हैं।
    }
# { $runtime } is a runtime's name, such as Java Runtime Environment.
core-warning-build-runtime-no-installer = ऐप को { $runtime } की आवश्यकता है, और यहाँ उसके लिए कोई इंस्टॉलर नहीं है, इसलिए कोई इंस्टॉलर नहीं चलाया जाता। इसे दूसरे PC पर स्वयं इंस्टॉल करें।
# { $why } is why something was left out, such as "its name says what it holds".
core-warning-build-left-out-of-package = पैकेज से छोड़ा गया: { $why }।
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-missing-dependency = निर्भरता का समाधान नहीं किया जा सका: { $name }
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-dependency-only-here = { $name } केवल इस PC के खोज पथ से मिला, और यह ऐप का भाग नहीं है, इसलिए यह साथ नहीं आता। हो सकता है कि इसके बिना ऐप किसी PC पर प्रारंभ न हो।
core-warning-build-driver-detected = ड्राइवर या कर्नेल घटक मिला। ऐप का अपना ड्राइवर बंडल किया जाता है और फिर से पंजीकृत किया जाता है; किसी विक्रेता या इनबॉक्स ड्राइवर को फिर से वितरित नहीं किया जाता, और उसे उसके हस्ताक्षरित विक्रेता पैकेज से फिर से इंस्टॉल करना होगा।
core-warning-build-near-no-longer-offered = "यह भी इसका भाग हो सकता है" सूची में चुनी गई कोई चीज़ अब उस सूची में नहीं है, इसलिए उसे जोड़ा नहीं गया। सूची को फिर से देखें।
# { $app_name } is the name of a part of the app, as the list of apps calls it. { $why } is never shown: it chooses the
# reason. Translate each reason as the end of the sentence.
core-warning-build-near-part-left-out =
    { $app_name } जिसे अपना बताता है, उसका एक भाग छोड़ दिया गया: { $why ->
        [unreadable] इसका पथ ड्राइव से बाहर जाता है या किसी लिंक से होकर जाता है, इसलिए इसे सुरक्षित रूप से पढ़ा नहीं जा सकता
        [missing] यह मौजूद नहीं है
        [link] यह किसी लिंक से होकर जाता है
        [short-name] यह उस तरह नहीं लिखा गया है जैसा इसका वास्तविक नाम है
        [another-account] यह किसी अन्य खाते के फ़ोल्डरों में है
        [one-account] यह ऐप एक खाते के लिए इंस्टॉल किया गया था, और यह भाग उस खाते का नहीं है
        [shared] इसमें साझा चीज़ें हैं, या यह Windows का भाग है
        [generic-name] इसके नाम से पता नहीं चलता कि यह किसका है
        [publisher] इसका नाम केवल प्रकाशक के नाम पर है, और इसमें प्रकाशक के अन्य उत्पाद हो सकते हैं
        [holds-the-app] इसमें इस ऐप का अपना फ़ोल्डर है, और इसमें और भी चीज़ें हो सकती हैं
        [another-apps-name] यह किसी अन्य ऐप के नाम से है
        [another-apps-record] इसका इंस्टॉल रिकॉर्ड किसी अन्य ऐप का है
        [in-another-apps-folder] यह किसी अन्य ऐप के फ़ोल्डर में है
       *[another-apps] यह किसी अन्य ऐप का है
    }।
# { $app_name } is the name of a part of the app, as the list of apps calls it.
core-warning-build-near-keys-left-out = { $app_name } के इंस्टॉल रिकॉर्ड में सूचीबद्ध रजिस्ट्री कुंजियाँ छोड़ दी गईं, क्योंकि वह रिकॉर्ड किसी अन्य ऐप की चीज़ों का भी नाम लेता है।
# "Windows keeps what an app wrote for one account" is Windows' VirtualStore folder.
core-warning-build-virtual-store-program = एक प्रोग्राम उस फ़ोल्डर में था जहाँ Windows किसी एक खाते के लिए ऐप द्वारा लिखी गई चीज़ें रखता है। Windows वहाँ कभी कोई प्रोग्राम नहीं रखता, इसलिए उसे किसी और ने वहाँ रखा, और उसे छोड़ दिया गया।
# { $path } is a file or folder, such as C:\Users\Sam\AppData\Roaming\Acme.
core-warning-build-file-left-out = { $path } को छोड़ दिया गया: यह कनेक्ट की गई ड्राइव पर किसी लिंक से होकर जाता है, या यह उस खाते के अलावा किसी अन्य खाते का है जिसका डेटा शामिल किया जा रहा है।
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as D:\Sam\Documents.
core-warning-build-user-folder-not-on-drive = { $user } का { $folder } फ़ोल्डर { $path } पर था, जो इस ड्राइव पर नहीं है, इसलिए वहाँ मौजूद चीज़ें शामिल नहीं की गईं।
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as \\server\home\Sam\Documents.
core-warning-build-user-folder-not-on-this-pc = { $user } का { $folder } फ़ोल्डर { $path } पर था, जो इस PC की किसी डिस्क पर नहीं है, इसलिए वहाँ मौजूद चीज़ें शामिल नहीं की गईं।
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-base-unreadable = { $path } को पढ़ा नहीं जा सका ({ $error }); वहाँ संग्रहीत एप्लिकेशन डेटा शामिल नहीं किया गया।
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-folder-unreadable = { $path } को पढ़ा नहीं जा सका ({ $error }); वहाँ के अंदर के फ़ोल्डरों का एप्लिकेशन डेटा अनुपलब्ध हो सकता है।
# { $error } is what went wrong.
core-warning-build-entry-unreadable = फ़ाइल सिस्टम की एक प्रविष्टि पढ़ी नहीं जा सकी: { $error }
# { $count } is how many links. { $names } lists some of them. { $more } is how many more there are, which are not
# named; when it is 0, the sentence ends after the list.
core-warning-build-links-not-followed =
    { $count ->
        [one] इस फ़ोल्डर में { $count } लिंक, या कुछ और खोलने वाले नाम, का अनुसरण नहीं किया गया, इसलिए वह जिस चीज़ की ओर इंगित करता है, वह इंस्टॉलर में नहीं है: { $names }
       *[other] इस फ़ोल्डर में { $count } लिंक, या कुछ और खोलने वाले नामों, का अनुसरण नहीं किया गया, इसलिए वे जिन चीज़ों की ओर इंगित करते हैं, वे इंस्टॉलर में नहीं हैं: { $names }
    }{ $more ->
        [0] ।
       *[other] , और { $more } अन्य।
    }
# { $count } is how many folders. { $depth } is how many levels down a folder can be. { $names } lists some of them.
# { $more } is how many more there are, which are not named; when it is 0, the sentence ends after the list.
core-warning-build-folders-too-deep =
    { $count ->
        [one] { $count } फ़ोल्डर { $depth } स्तर से अधिक नीचे है, इसलिए उसके अंदर की चीज़ें इंस्टॉलर में नहीं हैं: { $names }
       *[other] { $count } फ़ोल्डर { $depth } स्तर से अधिक नीचे हैं, इसलिए उनके अंदर की चीज़ें इंस्टॉलर में नहीं हैं: { $names }
    }{ $more ->
        [0] ।
       *[other] , और { $more } अन्य।
    }
# { $error } is what went wrong.
core-warning-build-metadata-unreadable = फ़ाइल मेटाडेटा पढ़ा नहीं जा सका: { $error }
core-warning-build-reserved-name = इस फ़ाइल का नाम Windows डिवाइस के लिए आरक्षित है, इसलिए इसे कॉपी नहीं किया गया।
# { $error } is what went wrong.
core-warning-build-hash-failed = फ़ाइल का हैश नहीं बनाया जा सका: { $error }
core-warning-build-cloud-placeholder = केवल-ऑनलाइन क्लाउड फ़ाइल (जैसे OneDrive Files On-Demand) का कोई स्थानीय डेटा नहीं है; बैकअप से पहले इसे पिन करें या डाउनलोड करें। इसकी रिपोर्ट की गई, लेकिन इसे कॉपी नहीं किया गया।
# { $file_name } is a driver's file name, such as acmeflt.sys.
core-warning-build-driver-file = ड्राइवर/कर्नेल फ़ाइल '{ $file_name }' को सामान्य फ़ाइल के रूप में कॉपी नहीं किया जाता। अगर यह इस ऐप की किसी पंजीकृत ड्राइवर सेवा के लिए है, तो इंस्टॉलर इसे बंडल करता है और ड्राइवर को स्वचालित रूप से फिर से पंजीकृत करता है; अन्यथा इसे विक्रेता के हस्ताक्षरित ड्राइवर पैकेज से फिर से इंस्टॉल करें।
core-warning-build-file-gone = ऐप को स्कैन करते समय यह मौजूद था, लेकिन अब नहीं है, इसलिए यह इंस्टॉलर में नहीं है।
# { $error } is what went wrong.
core-warning-build-file-copy-failed = फ़ाइल को पैकेज में कॉपी नहीं किया जा सका: { $error }
# { $count } is how many files. { $names } lists them.
core-warning-build-not-looked-through =
    { $count ->
        [one] इस पैकेज में एक फ़ाइल है जिसकी यह जाँच नहीं की जा सकी कि उसमें यह लिखा है या नहीं कि पैकेज किससे आया है: { $names }। हो सकता है कि उसमें यह लिखा हो।
       *[other] इस पैकेज में ऐसी फ़ाइलें हैं जिनकी यह जाँच नहीं की जा सकी कि उनमें यह लिखा है या नहीं कि पैकेज किससे आया है: { $names }। हो सकता है कि उनमें यह लिखा हो।
    }
# { $count } is the most empty folders an installer makes, such as 10,000.
core-warning-build-empty-folders-truncated = इस ऐप में { $count } से अधिक खाली फ़ोल्डर हैं, और केवल इतने ही फिर से बनाए जाएँगे।
core-warning-build-credential-by-now = पैकेज से छोड़ा गया: यह अब क्रेडेंशियल जैसा दिखता है।
core-warning-build-no-package-path = कॉपी की जा सकने वाली फ़ाइल का कोई पैकेज-सापेक्ष पथ नहीं था।
# { $file_name } is the runtime installer's file name, such as vc_redist.x64.exe. { $error } is what went wrong.
core-warning-build-prerequisite-path-invalid = बंडल की गई पूर्वापेक्षा { $file_name } नहीं रखी जा सकी: { $error }
# { $runtime } is a runtime's name, such as Microsoft Visual C++ 2015+ Redistributable (x64). { $error } is what went
# wrong. winget is the name of a Windows command; keep it as it is.
core-warning-build-prerequisite-download-failed = { $runtime } डाउनलोड नहीं किया जा सका ({ $error }); इंस्टॉलर लक्ष्य PC पर winget आज़माएगा।
# { $runtime } is a runtime's name. { $reason } is why the check failed, such as "it is signed by Contoso Ltd, not
# Microsoft".
core-warning-build-prerequisite-signature-failed = { $runtime } का डाउनलोड हस्ताक्षर जाँच में विफल रहा, इसलिए इसे बंडल नहीं किया गया: { $reason }। इंस्टॉलर लक्ष्य PC पर winget आज़माएगा।
# { $runtime } is a runtime's name. { $error } is what went wrong.
core-warning-build-prerequisite-hash-failed = { $runtime } डाउनलोड किया गया, लेकिन उसका हैश नहीं बनाया जा सका ({ $error }); इसके बजाय इंस्टॉलर लक्ष्य PC पर winget आज़माएगा।
# { $service } is a driver's service name, such as AcmeFlt. { $error } is what went wrong.
core-warning-build-driver-copy-failed = '{ $service }' के लिए ड्राइवर इमेज बंडल नहीं की जा सकी: { $error }
core-warning-build-license-not-copied = लाइसेंस डेटा कॉपी नहीं किया जा सका (अनुपलब्ध है या पढ़ा नहीं जा सकता); केवल मेटाडेटा के रूप में रिकॉर्ड किया गया।
core-warning-build-license-file-credential = एक लाइसेंस फ़ाइल क्रेडेंशियल जैसी दिखती है, और उसे छोड़ दिया गया।
# { $error } is what went wrong.
core-warning-build-license-store-walk-failed = लाइसेंस स्टोर के एक भाग की गणना नहीं की जा सकी: { $error }
core-warning-build-license-store-credential = लाइसेंस स्टोर की एक फ़ाइल क्रेडेंशियल जैसी दिखती है, और उसे छोड़ दिया गया।
# { $error } is what went wrong.
core-warning-build-license-store-file-skipped = एक लाइसेंस स्टोर फ़ाइल छोड़ी गई: { $error }
