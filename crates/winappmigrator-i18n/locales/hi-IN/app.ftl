# Words of the app, WinAppMigrator itself: its one window, with the installed apps listed on the left and, on the
# right, what goes in an installer of the one picked and the button that builds it; and the steps to read another PC
# through WinAppMigrator Client. Message names start with app-.


## The window and its header

# The program's name, in the window's title bar and at the top of the window. Keep it as it is unless your language
# writes product names in a script of its own.
app-name = WinAppMigrator
app-tagline = किसी इंस्टॉल किए गए ऐप को सेल्फ़-एक्सट्रैक्टिंग इंस्टॉलर में बदलें
app-theme-to-light = हल्की थीम पर स्विच करें
app-theme-to-dark = गहरी थीम पर स्विच करें
app-reload-list = एप्लिकेशन सूची फिर से लोड करें
app-list-loading = लोड हो रहा है
# { $count } is how many apps are listed.
app-list-count =
    { $count ->
        [one] { $count } ऐप
       *[other] { $count } ऐप्स
    }
app-list-load-failed = लोड विफल रहा
# Shown when the window cannot open at all. { $error } is what went wrong, in the words of Windows or a graphics driver.
app-window-could-not-open = विंडो नहीं खुल सकी: { $error }

## The list of installed apps

app-list-search = नाम या प्रकाशक से खोजें  (Ctrl+F)
app-list-filter-all = सभी
# Whether an app is installed for one account or for everyone on the PC.
app-scope-per-user = प्रति-उपयोगकर्ता
app-scope-per-machine = प्रति-मशीन
app-scope-unknown = अज्ञात
# An app whose processor architecture (x86, x64, ARM64) is not known.
app-arch-unknown = अज्ञात
app-list-loading-apps = इंस्टॉल किए गए एप्लिकेशन लोड हो रहे हैं…
# { $error } is why the apps could not be listed.
app-list-could-not-list = एप्लिकेशन सूचीबद्ध नहीं किए जा सके: { $error }
app-list-no-matches = कोई मेल खाने वाला एप्लिकेशन नहीं
app-list-no-matches-hint = खोज टेक्स्ट या दायरा फ़िल्टर बदलें।
app-list-worker-disconnected = इन्वेंटरी वर्कर डिस्कनेक्ट हो गया।
# What a screen reader says of an app's row. { $name } is the app's name, { $publisher } who made it, and { $version } its
# version, such as 11.4.2, or — when either is not known.
app-list-row = { $name }, { $publisher }, संस्करण { $version }

## The list's columns, and what a screen reader says each column's heading does when clicked

app-column-name = नाम
app-column-name-sort = नाम के अनुसार सॉर्ट करें
app-column-publisher = प्रकाशक
app-column-publisher-sort = प्रकाशक के अनुसार सॉर्ट करें
app-column-version = संस्करण
app-column-version-sort = संस्करण के अनुसार सॉर्ट करें
app-column-scope = दायरा
app-column-scope-sort = दायरे के अनुसार सॉर्ट करें
# Short for architecture: x86, x64, or ARM64. The column is narrow.
app-column-arch = आर्क
app-column-arch-sort = आर्किटेक्चर के अनुसार सॉर्ट करें

## Where apps are read from: this PC, a drive from another PC, a disk image of one, or another PC on the network

app-source = स्कैन स्रोत
app-source-this-pc = यह PC
app-source-drive = कनेक्ट की गई ड्राइव…
app-source-image = डिस्क छवि…
app-source-network = नेटवर्क PC…
app-source-attaching-image = छवि को केवल-पढ़ने के लिए संलग्न किया जा रहा है…
# { $computer } is the other PC's name, such as OLD-PC, and { $address } its network address, such as 192.168.1.20.
app-source-network-pc = नेटवर्क पर { $computer } ({ $address })
app-source-disconnect = डिस्कनेक्ट करें
app-source-change = बदलें…
# The title of the window that picks a disk image. .vhd and .vhdx are file extensions.
app-source-pick-image-title = दूसरे PC की डिस्क छवि चुनें (.vhd या .vhdx)
app-source-pick-image-filter = डिस्क छवियाँ
# { $path } is the disk image file.
app-source-image-has-no-windows = { $path } में Windows वाला कोई वॉल्यूम नहीं है।
app-source-image-not-attached = डिस्क छवि संलग्न नहीं की जा सकी।
app-source-pick-drive-title = कनेक्ट की गई ड्राइव का रूट चुनें (जैसे E:\)
# { $path } is the folder picked, such as E:\. \Windows and \Users are folder names, as Windows writes them.
app-source-not-a-windows-drive = { $path } Windows सिस्टम ड्राइव जैसी नहीं दिखती (\Windows और \Users नहीं हैं)। दूसरे PC से लाई गई ड्राइव का रूट चुनें।
# Said of a drive or disk image with a Windows older than Windows 10. { $windows } is its name, such as "Windows 7
# Professional Service Pack 1, 32-bit". { $is_32_bit } is "yes" when that Windows is 32-bit, which adds the last
# sentence, and "no" when it is not.
app-source-older-windows =
    इसमें { $windows } है। इससे बनाए गए इंस्टॉलर Windows 10 और 11 के लिए होते हैं। उपयोगकर्ता डेटा के साथ, वे सेटिंग्स भी आती हैं जिन्हें Windows ने किसी पुराने ऐप के लिए चुपचाप अलग रखा था, और ऐसे रनटाइम, जो Windows 11 में नहीं हैं, जैसे DirectX 9, ऐप के साथ इंस्टॉल किए जाते हैं।{ $is_32_bit ->
        [yes] { " " }इसके ऐप्स 32-बिट हैं, इसलिए वे Program Files (x86) में जाते हैं, जहाँ 64-बिट Windows उन्हें रखता है।
       *[no] { "" }
    }
# The same, of a Windows whose name could not be read. { $is_32_bit } is as above.
app-source-older-windows-unnamed =
    इसमें Windows का कोई पुराना संस्करण है। इससे बनाए गए इंस्टॉलर Windows 10 और 11 के लिए होते हैं। उपयोगकर्ता डेटा के साथ, वे सेटिंग्स भी आती हैं जिन्हें Windows ने किसी पुराने ऐप के लिए चुपचाप अलग रखा था, और ऐसे रनटाइम, जो Windows 11 में नहीं हैं, जैसे DirectX 9, ऐप के साथ इंस्टॉल किए जाते हैं।{ $is_32_bit ->
        [yes] { " " }इसके ऐप्स 32-बिट हैं, इसलिए वे Program Files (x86) में जाते हैं, जहाँ 64-बिट Windows उन्हें रखता है।
       *[no] { "" }
    }

## The details panel, before an app is picked, and the app picked

app-details-pair-first = पुराने PC से युग्मित करें
app-details-pair-first-hint = जब वहाँ WinAppMigrator Client चल रहा हो और आप उसका कोड टाइप कर चुके हों, तो उसके ऐप्स यहाँ सूचीबद्ध होते हैं, और आप उनमें से किसी का इंस्टॉलर वैसे ही बनाते हैं जैसे इस PC के किसी ऐप का।
app-details-pick-an-app = कोई एप्लिकेशन चुनें
app-details-pick-an-app-hint = क्या शामिल करना है, यह चुनने और उसका इंस्टॉलर बनाने के लिए सूची से कोई ऐप चुनें।
# { $publisher } is who made the app, and { $version } its version, such as 11.4.2.
app-details-publisher-and-version = { $publisher }, संस्करण { $version }
# { $version } is the app's version, such as 11.4.2.
app-details-unknown-publisher-and-version = अज्ञात प्रकाशक, संस्करण { $version }
app-details-unknown-publisher = अज्ञात प्रकाशक

## What goes in besides the program

app-option-license = लाइसेंस और सक्रियण डेटा
app-option-license-hint = यह इस PC से जुड़ा हो सकता है, और नए PC पर इसे फिर से सक्रिय करने की आवश्यकता हो सकती है।
app-option-user-data = उपयोगकर्ता डेटा
# Documents, Saved Games, and ProgramData are folders of Windows: use the names Windows gives them in your language,
# except ProgramData, which keeps its name.
app-option-user-data-hint = सेटिंग्स और इतिहास, दस्तावेज़, सहेजे गए गेम, और ProgramData।
app-option-whose-data = किसका डेटा
app-option-whose-data-hint = एक इंस्टॉलर में एक खाते का डेटा होता है। इंस्टॉलर जहाँ चलता है, वहाँ जो भी साइन इन है, यह डेटा उसके पास जाता है।

## Building the installer

app-output-folder = आउटपुट फ़ोल्डर
# An example of a folder, shown in the empty field. "You" stands for the name of an account.
app-output-folder-example = C:\Users\आप\WinAppMigrator
app-output-folder-not-full = पूरा फ़ोल्डर पथ दर्ज करें, जैसे D:\Installers।
app-build-create = इंस्टॉलर बनाएँ
app-build-create-again = इंस्टॉलर फिर से बनाएँ
app-build-cancel = रद्द करें
app-build-cancelling = वर्तमान चरण के बाद रद्द किया जा रहा है…
app-build-cancelled = बिल्ड रद्द किया गया। कोई इंस्टॉलर नहीं लिखा गया।
# winappmigrator-installer.exe is a file name.
app-build-no-stub = इंस्टॉलर स्टब (winappmigrator-installer.exe) इस ऐप के बगल में नहीं मिला, इसलिए सेल्फ़-एक्सट्रैक्टिंग इंस्टॉलर नहीं बनाया जा सकता।
app-build-select-an-app-first = पहले माइग्रेट करने के लिए कोई एप्लिकेशन चुनें।
app-build-look-again-first = इसे बनाने से पहले इस ऐप को फिर से स्कैन करना होगा।
# { $reason } is what went wrong, in the program's own words, often in English.
app-build-stopped-unexpectedly = बिल्ड अनपेक्षित रूप से रुक गया: { $reason }। अगर इसने आउटपुट फ़ोल्डर में कोई इंस्टॉलर छोड़ा है, तो उसका उपयोग न करें।
app-build-worker-disconnected = माइग्रेशन वर्कर डिस्कनेक्ट हो गया।
# { $path } is the output folder, and { $error } what Windows said went wrong.
app-build-no-output-folder = आउटपुट फ़ोल्डर { $path } नहीं बनाया जा सका: { $error }
# { $error } is what went wrong.
app-build-no-staging-folder = सुरक्षित स्टेजिंग फ़ोल्डर नहीं बनाया जा सका, इसलिए कुछ भी नहीं बनाया गया: { $error }
# { $error } is what went wrong, as a sentence.
app-build-could-not-build = इंस्टॉलर नहीं बनाया जा सका। { $error }
app-build-no-installer = बिल्ड बिना किसी इंस्टॉलर के समाप्त हो गया।
# { $error } is what went wrong.
app-build-hash-check-did-not-run = इंस्टॉलर लिखा गया, लेकिन उसकी हैश जाँच नहीं चल सकी: { $error }। इसका उपयोग न करें।

## How far a build has got

app-progress-preparing = पता लगाया जा रहा है कि इसमें क्या जाएगा…
app-progress-checking-room = जाँचा जा रहा है कि पर्याप्त स्थान है या नहीं…
app-progress-bundling = रनटाइम और ड्राइवर बंडल किए जा रहे हैं…
# { $done } is how many files have been read so far, and { $total } how many there are.
app-progress-packing = फ़ाइलें पढ़ी और संपीड़ित की जा रही हैं: { $total } में से { $done }
# { $done } is how many of the installer's records have been looked over so far, and { $total } how many there are.
app-progress-looking-over = इंस्टॉलर में लिखी बातों की जाँच की जा रही है: { $total } में से { $done }…
app-progress-writing = इंस्टॉलर लिखा जा रहा है…
app-progress-verifying = हैश सत्यापित किए जा रहे हैं…

## What a build made

app-result-failed-hash-check = इस इंस्टॉलर का उपयोग न करें: यह अपनी हैश जाँच में विफल रहा।
# { $count } is how many files were gone when the build came to read them.
app-result-files-gone =
    { $count ->
        [one] इंस्टॉलर बन गया, लेकिन ऐप को स्कैन किए जाने के बाद से { $count } फ़ाइल गायब हो गई, इसलिए वह इसमें नहीं है। नीचे चेतावनियाँ देखें।
       *[other] इंस्टॉलर बन गया, लेकिन ऐप को स्कैन किए जाने के बाद से { $count } फ़ाइलें गायब हो गईं, इसलिए वे इसमें नहीं हैं। नीचे चेतावनियाँ देखें।
    }
# { $count } is how many files could not be read into the installer.
app-result-files-not-captured =
    { $count ->
        [one] इंस्टॉलर बन गया, लेकिन { $count } फ़ाइल कैप्चर नहीं की जा सकी, आमतौर पर इसलिए क्योंकि वह लॉक थी या पढ़ी नहीं जा सकती थी। हो सकता है कि उसके बिना ऐप काम न करे। नीचे चेतावनियाँ देखें।
       *[other] इंस्टॉलर बन गया, लेकिन { $count } फ़ाइलें कैप्चर नहीं की जा सकीं, आमतौर पर इसलिए क्योंकि वे लॉक थीं या पढ़ी नहीं जा सकती थीं। हो सकता है कि उनके बिना ऐप काम न करे। नीचे चेतावनियाँ देखें।
    }
app-result-mentions-who = इंस्टॉलर बन गया, और इसमें अब भी कुछ ऐसा है जो बताता है कि यह किससे आया है। इसे किसी को देने से पहले नीचे लिखी बातें पढ़ें।
app-result-ready = इंस्टॉलर तैयार है। इसके पैकेज ने अपनी हैश जाँच पास कर ली।
app-result-files = फ़ाइलें
app-result-installer-size = इंस्टॉलर का आकार
app-result-warnings = चेतावनियाँ
app-result-log = लॉग
app-result-copy-error = त्रुटि कॉपी करें
app-result-installer = इंस्टॉलर
app-result-data-file = डेटा फ़ाइल
app-result-open-folder = फ़ोल्डर खोलें
app-result-copy-path = पथ कॉपी करें
# { $error } is what Windows said went wrong.
app-result-could-not-open-folder = फ़ोल्डर नहीं खोला जा सका: { $error }
# { $installer } and { $data_file } are file names, such as App_installer.exe and App_installer.wamdata.
app-result-two-files = यह ऐप एक निष्पादन योग्य फ़ाइल के लिए बहुत बड़ा है, इसलिए इसका इंस्टॉलर दो फ़ाइलों में है: { $installer } और { $data_file }। इन्हें साथ रहना होगा: दोनों को दूसरे PC पर, एक ही फ़ोल्डर में, इन्हीं नामों से कॉपी करें। अगर आप इंस्टॉलर पर हस्ताक्षर करते हैं, तो वह हस्ताक्षर डेटा फ़ाइल पर भी लागू होता है।
# { $app_name } is the app's name. "Unknown publisher" is what Windows shows when it asks to run an unsigned program: use
# Windows' own words for it in your language.
app-result-run-it = { $app_name } को फिर से इंस्टॉल करने के लिए इसे दूसरे PC पर चलाएँ। यह SHA-256 से अपनी फ़ाइलों की जाँच करता है। Windows "अज्ञात प्रकाशक" दिखाता है, क्योंकि आपके द्वारा बनाए गए इंस्टॉलर हस्ताक्षरित नहीं होते।
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-verification-errors = सत्यापन त्रुटियाँ ({ $count })
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-warnings-heading = चेतावनियाँ ({ $count })
# After the first of a long list of warnings. { $count } is how many more there are.
app-warnings-and-more = और { $count } अन्य।

## Looking at an app, to see what an installer of it would hold

app-scan-getting-ready = इस ऐप को स्कैन करने की तैयारी की जा रही है…
app-scan-install-record = यह पढ़ा जा रहा है कि इसे कैसे इंस्टॉल किया गया था…
app-scan-near-matches = यह खोजा जा रहा है कि और क्या इसका भाग हो सकता है…
app-scan-system-items = इसकी सेवाएँ, कार्य, और शॉर्टकट खोजे जा रहे हैं…
app-scan-files = इसकी फ़ाइलें देखी जा रही हैं…
app-scan-registry = यह पढ़ा जा रहा है कि Windows से इसे क्या जोड़ता है…
app-scan-finishing = समाप्त किया जा रहा है…
app-scan-look-again = फिर से स्कैन करें
app-scan-stop = रोकें
app-scan-running-hint = जब तक यह चल रहा है, आप कोई अन्य ऐप चुन सकते हैं या शामिल की जाने वाली चीज़ें बदल सकते हैं।
# { $error } is why, in words that follow a colon.
app-scan-failed = इस ऐप को स्कैन नहीं किया जा सका: { $error }
app-scan-try-again = फिर से प्रयास करें
app-scan-stopped = रोका गया। इस ऐप को स्कैन कर लेने के बाद ही इंस्टॉलर बनाया जा सकता है।
# { $reason } is what went wrong, in the program's own words, often in English.
app-scan-stopped-unexpectedly = यह अनपेक्षित रूप से रुक गया: { $reason }।

## How big an installer can be, and whether there is room for it

app-room-files = फ़ाइलें
# Over how big the installer can be: up to that size.
app-room-up-to = अधिकतम
app-room-free = रिक्त
# { $drive } is a drive, such as C:.
app-room-free-on = { $drive } पर रिक्त
# How much room a drive has, when it would not say.
app-room-free-unknown = अज्ञात
app-room-most-it-could-need = संपीड़न से पहले, इसे अधिकतम इतने स्थान की आवश्यकता हो सकती है।
# { $drive } is a drive, such as C:, { $needed } how much room it needs, such as 1.2 GB, and { $free } how much it has.
app-room-drive-short = इसे बनाने के लिए { $drive } पर अधिकतम { $needed } रिक्त स्थान चाहिए, और उस पर { $free } है।
# { $drives } is one sentence like the one above for each drive without the room, one after another.
app-room-not-enough = पर्याप्त स्थान नहीं है। { $drives } किसी अन्य ड्राइव पर आउटपुट फ़ोल्डर चुनें, या कुछ चीज़ें छोड़ दें।
# { $note } is a sentence, without its full stop, about a drive whose room could not be checked. Use your language's
# full stop.
app-room-could-not-tell = { $note }।

## What goes in the installer, part by part

app-footprint-heading = इंस्टॉलर में क्या जाता है
app-footprint-working-out = पता लगाया जा रहा है कि उसके साथ क्या जाता है…
# { $error } is why.
app-footprint-would-not-make-a-package = जो बचा है, उससे पैकेज नहीं बनेगा: { $error }
# { $list } is the names of what else is left out, such as a service and a shortcut.
app-footprint-also-left-out = इससे { $list } भी छूट जाते हैं, क्योंकि वे जो चलाते हैं या जिसका नाम लेते हैं, वह वहाँ नहीं होगा। उन्हें दूसरे PC पर सेट अप नहीं किया जाएगा।
# { $list } is the names of the first few of what else is left out, and { $count } how many more there are.
app-footprint-also-left-out-and-more = इससे { $list }, और { $count } अन्य भी छूट जाते हैं, क्योंकि वे जो चलाते हैं या जिसका नाम लेते हैं, वह वहाँ नहीं होगा। उन्हें दूसरे PC पर सेट अप नहीं किया जाएगा।
# { $list } is the names of license files. The option is the one named "License and Activation Data".
app-footprint-licenses-still-go = आपके द्वारा छोड़ी गई चीज़ों में मौजूद लाइसेंस डेटा फिर भी शामिल होता है, क्योंकि यह "लाइसेंस और सक्रियण डेटा" विकल्प से तय होता है: { $list }।
# { $elapsed } is how long ago, in minutes and seconds such as 45:00, or hours, minutes, and seconds such as 1:05:00.
# "Look Again" names the button of that name.
app-footprint-looked-at-long-ago = इसे { $elapsed } पहले स्कैन किया गया था। फ़ाइलें इंस्टॉलर बनाते समय पढ़ी जाती हैं, इसलिए कोई भी परिवर्तन शामिल हो जाता है, और तब तक जो कुछ गायब हो चुका हो, वह सूचीबद्ध होता है। कुछ भी नया शामिल करने के लिए फिर से स्कैन करें।
# Headings of the parts of the list, each with a line that says what is in it.
app-footprint-program-files = प्रोग्राम फ़ाइलें
app-footprint-data-folders = डेटा फ़ोल्डर
# What could go in with the User Data option, when nothing else is in this part. { $size } is how much, such as 1.2 GB.
app-footprint-folders-with-user-data = उपयोगकर्ता डेटा विकल्प के साथ { $size }
# { $included } is how many folders go in, { $count } how many there are, and { $size } how much those that go in hold.
app-footprint-folders-summary =
    { $count ->
        [one] { $count } में से { $included } फ़ोल्डर, { $size }
       *[other] { $count } में से { $included } फ़ोल्डर, { $size }
    }
# { $size } is how much, such as 1.2 GB, and { $count } in how many files.
app-footprint-data-only-with-user-data =
    { $count ->
        [one] इस ऐप के डेटा की { $count } फ़ाइल में मौजूद { $size } केवल उपयोगकर्ता डेटा विकल्प के साथ शामिल होगा।
       *[other] इस ऐप के डेटा की { $count } फ़ाइलों में मौजूद { $size } केवल उपयोगकर्ता डेटा विकल्प के साथ शामिल होगा।
    }
# A folder's row. { $count } is how many files it holds, and { $detail } where it is or why it cannot be left out.
app-footprint-folder-row =
    { $count ->
        [one] { $count } फ़ाइल। { $detail }
       *[other] { $count } फ़ाइलें। { $detail }
    }
app-footprint-services = सेवाएँ
app-footprint-startup = स्वचालित रूप से प्रारंभ होता है
app-footprint-tasks = शेड्यूल किए गए कार्य
app-footprint-shortcuts = शॉर्टकट
# { $included } is how many of the part's things go in, and { $count } how many there are.
app-footprint-items-summary = { $count } में से { $included }
# { $why } is what else was left out that this goes with.
app-footprint-left-out-too = यह भी छोड़ा गया: { $why }
# A row's detail. { $kind } is what it is, such as Service, and { $detail } what it does.
app-footprint-change-detail = { $kind }। { $detail }
app-footprint-integrations = Windows से इसे क्या जोड़ता है
# { $included } is how many kinds go in, such as file types and firewall rules, and { $count } how many kinds there are.
app-footprint-integrations-summary =
    { $count ->
        [one] { $count } में से { $included } प्रकार
       *[other] { $count } में से { $included } प्रकार
    }
app-footprint-integration-nothing-left = आपके द्वारा छोड़ी गई चीज़ों के कारण, इसमें से कुछ भी नहीं बचा
# { $count } is how many things of this kind the installer sets up.
app-footprint-integration-things =
    { $count ->
        [one] { $count } चीज़ सेट अप होती है
       *[other] { $count } चीज़ें सेट अप होती हैं
    }
app-footprint-prerequisites = इसे चलने के लिए क्या चाहिए
# { $count } is how many runtimes, such as Visual C++ Redistributable.
app-footprint-prerequisites-summary =
    { $count ->
        [one] { $count } रनटाइम
       *[other] { $count } रनटाइम
    }
app-footprint-near-matches = यह भी इसका भाग हो सकता है
# { $ticked } is how many were ticked, and { $count } how many there are.
app-footprint-near-matches-summary = { $count } में से { $ticked } जोड़े गए
app-footprint-near-matches-hint = जब तक आप इन्हें नहीं चुनते, ये इंस्टॉलर में नहीं हैं। किसी एक को चुनने पर ऐप फिर से स्कैन होता है, बाकी चीज़ों जैसी ही जाँचों के साथ, इसलिए जो पासवर्ड जैसा दिखता है, वह फिर भी बाहर रहता है।
# { $why } is why it might be part of the app, as a sentence.
app-footprint-near-match-with-user-data = उपयोगकर्ता डेटा विकल्प के साथ आता है। { $why }
# How much a folder holds, when not all of it could be measured. { $size } is how much was, such as 1.2 GB.
app-footprint-near-match-over = { $size } से अधिक
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-folder = फ़ोल्डर: { $place }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-hidden = किसी ऐप का छिपा हुआ भाग: { $place }
app-footprint-nothing-left-out = पासवर्ड, लाइसेंस, या प्रमाणपत्र जैसा दिखने के कारण कुछ भी नहीं छोड़ा गया।
app-footprint-left-out = क्या छोड़ा गया, और क्यों
# { $count } is how many things were left out.
app-footprint-left-out-summary =
    { $count ->
        [one] { $count } चीज़
       *[other] { $count } चीज़ें
    }
# Above a list of things left out for one reason. { $title } is the reason.
app-footprint-left-out-group = { $title }:
app-footprint-refused = किसी भी इंस्टॉलर को इन्हें सेट अप करने की अनुमति नहीं होगी:
# A heading over a list that opens and closes. { $count } is how many are in it.
app-footprint-worth-knowing = जानने योग्य बातें ({ $count })
# { $list } is what kinds of things were looked for and none found, from the five below.
app-none-found = कोई नहीं मिला: { $list }।
app-none-found-services = सेवाएँ
app-none-found-startup = स्टार्टअप आइटम
app-none-found-tasks = शेड्यूल किए गए कार्य
app-none-found-shortcuts = शॉर्टकट
app-none-found-integrations = Windows से इसे जोड़ने वाली कोई भी चीज़

## Network PC: reading another PC through WinAppMigrator Client, in three steps. "WinAppMigrator Client" is the name of a
## program, and the old PC is the one the apps are moved off.

app-network-step-client = 1. क्लाइंट को पुराने PC पर रखें
app-network-step-client-hint = WinAppMigrator Client उस PC पर चलता है जिस पर ऐप अभी है, जिसमें Windows 7 या उसके बाद का संस्करण हो। यह केवल इस PC को ही उसे पढ़ने देता है, एन्क्रिप्ट किए गए कनेक्शन पर, और उस कोड से जो वह दिखाता है।
# { $drive } is a USB drive, such as E:\.
app-network-save-to-usb = USB ड्राइव { $drive } में सहेजें
# { $drive } is a USB drive, such as E:\, and { $name } the name it was given, such as BACKUP.
app-network-save-to-named-usb = USB ड्राइव { $drive } ({ $name }) में सहेजें
app-network-save-to-folder = किसी फ़ोल्डर में सहेजें…
app-network-look-for-usb-again = USB ड्राइव फिर से खोजें
app-network-no-usb = कोई USB ड्राइव प्लग इन नहीं है। एक प्लग इन करें, या किसी फ़ोल्डर में सहेजें: ऐसा फ़ोल्डर जिस तक आप पुराने PC से पहुँच सकें, जैसे आपके नेटवर्क पर कोई साझा फ़ोल्डर।
# { $path } is where the client was saved, such as E:\WinAppMigrator Client.exe.
app-network-client-saved = { $path } के रूप में सहेजा गया। पुराने PC पर, जिस ऐप को आप ले जा रहे हैं उसे बंद करें, फिर इस पर डबल-क्लिक करें और इसे परिवर्तन करने की अनुमति दें। यह एक कोड दिखाता है।
app-network-step-pick = 2. पुराना PC चुनें
app-network-looking = आपके नेटवर्क पर क्लाइंट चला रहे PC खोजे जा रहे हैं…
app-network-type-name = या इसका नाम या पता टाइप करें
# An example, shown in the empty field: a PC's name, or its network address.
app-network-type-name-example = OLD-PC या 192.168.1.20
app-network-step-code = 3. इसके द्वारा दिखाया गया कोड टाइप करें
app-network-code = युग्मन कोड
app-network-connect = कनेक्ट करें
app-network-pairing = युग्मित किया जा रहा है…
# Why pairing did not work, shown on its own.
app-network-pairing-stopped = युग्मन पूरा होने से पहले रुक गया
app-network-wrong-code = यह वह कोड नहीं है जो पुराना PC दिखाता है। इसे जाँचें और फिर से प्रयास करें।
# { $error } is what went wrong.
app-network-not-reached = पुराने PC तक नहीं पहुँचा जा सका: { $error }
app-network-pick-folder-title = WinAppMigrator Client के लिए एक फ़ोल्डर चुनें
# { $path } is the folder, and { $error } what Windows said went wrong.
app-client-could-not-save = क्लाइंट को { $path } में सहेजा नहीं जा सका: { $error }
# scripts\build-client.ps1 is a file name.
app-client-not-carried = WinAppMigrator के इस बिल्ड में क्लाइंट नहीं है। इसे scripts\build-client.ps1 से बनाएँ, फिर ऐप को फिर से बनाएँ।
# { $error } is what Windows said went wrong.
app-client-could-not-save-note = इसके बगल वाला नोट सहेजा नहीं जा सका: { $error }
# A note saved as a text file beside the client, for whoever runs it on the old PC. Keep its lines short, as it is read in
# Notepad. "WinAppMigrator Client.exe" is a file name. "Network PC..." names the Scan Source button: use its words in your
# language.
app-client-note =
    WinAppMigrator Client
    
    इसे उस PC पर चलाएँ जिससे आप ऐप्स ले जा रहे हैं: Windows 7, 8, 10 या 11।
    
    1. जिस ऐप को आप ले जा रहे हैं, उसे बंद करें।
    2. "WinAppMigrator Client.exe" पर डबल-क्लिक करें और इसे परिवर्तन करने
       की अनुमति दें।
    3. यह 6-अंकीय युग्मन कोड दिखाता है। अपने नए PC पर WinAppMigrator में,
       नेटवर्क PC... चुनें, यह PC चुनें, और कोड टाइप करें।
    4. जब नए PC का काम पूरा हो जाए, तो क्लाइंट की विंडो में Enter दबाएँ, या
       उसे बंद करें।
    
    यह केवल पढ़ता है। चलते समय यह Windows ड्राइव का एक स्नैपशॉट रखता है, और
    Windows फ़ायरवॉल में आपके स्थानीय नेटवर्क के लिए अपने दो पोर्ट खोलता है,
    और रुकने पर इन दोनों को हटा देता है। नेटवर्क पर जो भी जाता है, वह
    एन्क्रिप्ट किया जाता है, और केवल वही PC कुछ भी पढ़ सकता है जिसके पास
    कोड है।
