# Words of the installer that every built installer carries, which runs on the PC an app is moved to and speaks its
# language, and of the uninstaller it leaves behind for Settings. Message names start with installer- for the
# installer's window, installer-cli- for its command line, and uninstaller- for the uninstaller.
#
# The command lines (installer-cli-, uninstaller-cli-) are always shown in English, as scripts read what they print.
# Translate them all the same. Keep their command-line flags, such as --apply and --target-root, exactly as they are.


## The installer's window: its title and header

installer-window-title = WinAppMigrator इंस्टॉलर
# The window's title in the taskbar. { $app_name } is the name of the app it installs.
installer-window-title-app = { $app_name } इंस्टॉल करें
installer-tagline = किसी अन्य PC पर कैप्चर किए गए एप्लिकेशन को फिर से इंस्टॉल करता है
# What an app is called when its package gives it no name.
installer-unnamed-app = एप्लिकेशन

## When the installer's window cannot open. Each is a paragraph of a message box.

# { $error } is what went wrong, as Windows or the graphics driver says it.
installer-window-failed = इंस्टॉलर की विंडो नहीं खुल सकी: { $error }
installer-window-failed-without-one = यह विंडो के बिना भी इंस्टॉल कर सकता है: इस फ़ाइल को व्यवस्थापक कमांड प्रॉम्प्ट से --apply के साथ चलाएँ, और स्थान चुनने के लिए --target-root के बाद एक फ़ोल्डर दें।

## Checking the installer when it opens

installer-checking = इंस्टॉलर की जाँच की जा रही है…
installer-stub-no-package = यह एक WinAppMigrator इंस्टॉलर स्टब है, और इसमें कोई एप्लिकेशन पैकेज नहीं है। WinAppMigrator से एक इंस्टॉलर बनाएँ, फिर बनाई गई .exe फ़ाइल यहाँ चलाएँ।
installer-check-stopped = इंस्टॉलर की जाँच रुक गई।
# { $error } is what went wrong, as Windows says it.
installer-locate-failed = इंस्टॉलर का पता नहीं लगाया जा सका: { $error }
# { $error } is what went wrong reading it.
installer-read-failed = इंस्टॉलर पढ़ा नहीं जा सका: { $error }
# { $error } is why, such as that the app needs a newer Windows.
installer-cannot-be-used-here = इस इंस्टॉलर के पैकेज का उपयोग इस PC पर नहीं किया जा सकता: { $error }

## The app, where it goes, and whether the installer is signed

installer-captured-from = कैप्चर का स्रोत
installer-install-location = इंस्टॉल स्थान
# The title of the folder picker.
installer-choose-location = इंस्टॉल स्थान चुनें
installer-folder-has-files = इस फ़ोल्डर में पहले से फ़ाइलें हैं। समान नाम वाली फ़ाइलें प्रतिस्थापित की जाती हैं, और अगर आप अनइंस्टॉल करते हैं, तो वे वापस आ जाती हैं।
installer-room-needed = आवश्यक डिस्क स्थान
# { $shortfall } says which drive is short of room, such as "C: needs up to 824.0 MB for the app's files, and has 300.0 MB free".
installer-room-short = हो सकता है कि यहाँ पर्याप्त स्थान न हो: { $shortfall }। पैकेज किसी अन्य ड्राइव पर अनपैक हो सकता है, लेकिन ऐप की फ़ाइलों के लिए फिर भी यहाँ पर्याप्त स्थान होना चाहिए।
# { $app_name } is the name of an app whose earlier install was cut short.
installer-interrupted = { $app_name } का एक पिछला इंस्टॉल कभी पूरा नहीं हुआ। इंस्टॉल करने पर पहले उसे पूर्ववत किया जाता है।
# Beside "Installer", for whether it proves who made it.
installer-signature = इंस्टॉलर
installer-signature-checking = इसके हस्ताक्षर की जाँच की जा रही है…
installer-signature-unsigned = हस्ताक्षरित नहीं: प्रकाशक अज्ञात
# { $publisher } is the name on the installer's certificate, such as "Fabrikam, Inc.".
installer-signature-signed-by = { $publisher } द्वारा हस्ताक्षरित
installer-signature-unnamed = किसी अनाम प्रकाशक द्वारा हस्ताक्षरित
# { $reason } is Windows' own word for what is wrong, such as HashMismatch.
installer-signature-invalid = इस इंस्टॉलर का हस्ताक्षर मान्य नहीं है ({ $reason })। हो सकता है कि हस्ताक्षर किए जाने के बाद इसे बदला गया हो।

## Why the install location cannot be used. Install and Preview Changes stay off while one of these is shown.

# { $app_name } is the name of the app being installed.
installer-location-app-already-there = { $app_name } इस फ़ोल्डर में पहले से इंस्टॉल है, इसलिए यह प्रति यहाँ नहीं जा सकती। इसे यहाँ इंस्टॉल करने के लिए, पहले इस PC पर मौजूद प्रति को Windows सेटिंग्स में ऐप्स के अंतर्गत अनइंस्टॉल करें, और फिर यह इंस्टॉलर फिर से खोलें। या ऊपर एक नया, खाली फ़ोल्डर दर्ज करें।
# { $other_app } is the name of another app Windows lists as installed in the folder. { $app_name } is the name of the app being installed.
installer-location-other-app-there = { $other_app } इस फ़ोल्डर में इंस्टॉल है, इसलिए { $app_name } यहाँ नहीं जा सकता। ऊपर एक नया, खाली फ़ोल्डर दर्ज करें। या, अगर आपको अब { $other_app } की आवश्यकता नहीं है, तो पहले उसे Windows सेटिंग्स में ऐप्स के अंतर्गत अनइंस्टॉल करें, और फिर यह इंस्टॉलर फिर से खोलें।
installer-location-has-files = इस फ़ोल्डर में पहले से ऐसी फ़ाइलें हैं जिन्हें WinAppMigrator ने इंस्टॉल नहीं किया था। वे किसी अन्य ऐप की हो सकती हैं, इसलिए उनके ऊपर कुछ भी इंस्टॉल नहीं किया जाता। ऊपर एक नया, खाली फ़ोल्डर दर्ज करें।
# { $reason } is why, such as that it cannot be the Windows folder.
installer-location-refused = इस फ़ोल्डर का उपयोग नहीं किया जा सकता: { $reason }। ऊपर कोई अन्य फ़ोल्डर दर्ज करें।

## What the install sets up beyond its files

installer-beyond-files = यह फ़ाइलों के अलावा क्या सेट अप करता है
# Under the heading above. It is part of what a person agrees to when they click Install.
installer-beyond-files-agreed = यह सब व्यवस्थापक अधिकारों के साथ किया जाता है। अगर इंस्टॉल विफल होता है, तो इसके द्वारा किए गए परिवर्तन पहले जैसे कर दिए जाते हैं, और जो पहले जैसा नहीं किया जा सका, उसके बारे में आपको बताया जाता है। आप बाद में सेटिंग्स में, इंस्टॉल किए गए ऐप्स के अंतर्गत, इंस्टॉल को पूर्ववत भी कर सकते हैं। इसके द्वारा प्रतिस्थापित की गई फ़ाइलें वापस आ जाती हैं, और इसके द्वारा इंस्टॉल किए गए रनटाइम बने रहते हैं।
installer-beyond-files-none = कुछ नहीं: यह केवल फ़ाइलें, और ऐप की अपनी रजिस्ट्री कुंजियों में सेटिंग्स कॉपी करता है।
# { $problem } is why, such as that the app cannot run on this PC.
installer-cannot-go-here = यह पैकेज इस स्थान पर नहीं जा सकता: { $problem }
installer-working-out = पता लगाया जा रहा है कि यह इस स्थान पर क्या सेट अप करेगा…
# Said as the reason after "This package cannot go to this location:".
installer-list-stopped = यह पता लगाना रुक गया कि यह क्या करेगा
# A heading over a list of what else the package asked for and will not get. { $count } is how many.
installer-more-left-out =
    { $count ->
        [one] { $count } और आइटम छोड़ा गया
       *[other] { $count } और आइटम छोड़े गए
    }

## Options

installer-options = विकल्प
installer-user-data = उपयोगकर्ता डेटा
installer-user-data-detail = चालू: सेटिंग्स, दस्तावेज़, और सहेजे गए गेम उस व्यक्ति के सामान्य फ़ोल्डरों में जाते हैं जो साइन इन है। बंद: यह सब इंस्टॉल फ़ोल्डर के अंतर्गत जाता है, और वहाँ से कुछ भी चलाने के लिए सेट अप नहीं किया जाता।
# { $account } is the name of the account on the other PC whose data this is.
installer-user-data-whose = इसमें { $account } का डेटा है।
installer-license = लाइसेंस
installer-license-detail = सक्रियण और लाइसेंस फ़ाइलें, और लाइसेंस रखने वाले रजिस्ट्री मान। कुछ को यहाँ फिर से सक्रिय करने की आवश्यकता हो सकती है।

## Dependencies: what else the app needs, and how each is provided

installer-dependencies = निर्भरताएँ
installer-dependencies-detail = इस ऐप को क्या चाहिए, और हर एक कैसे उपलब्ध कराया जाता है।
installer-dependencies-none = कोई अतिरिक्त रनटाइम निर्भरता नहीं मिली।
# { $feature } is the name of an optional part of Windows, such as NetFx3.
installer-dependency-windows-feature = Windows सुविधा: { $feature }
installer-dependency-included = शामिल
# { $id } is the package's name in winget, such as Microsoft.VCRedist.2015+.x64.
installer-dependency-winget = winget से इंस्टॉल किया जाता है: { $id }
installer-dependency-manual = मैन्युअल रूप से इंस्टॉल करें
installer-dependency-built-in = Windows में अंतर्निहित
installer-dependency-ships-with-app = ऐप के साथ आता है

## Buttons, and what is said beside them

installer-install = इंस्टॉल करें
installer-preview = परिवर्तनों का पूर्वावलोकन करें
installer-cancel = रद्द करें
installer-close = बंद करें
installer-open-folder = इंस्टॉल फ़ोल्डर खोलें
# { $error } is what went wrong, as Windows says it.
installer-open-folder-failed = फ़ोल्डर नहीं खोला जा सका: { $error }
installer-location-blocks = जब तक इंस्टॉल स्थान का उपयोग नहीं किया जा सकता, तब तक "इंस्टॉल करें" और "परिवर्तनों का पूर्वावलोकन करें" बंद रहते हैं। कारण ऊपर, उस फ़ील्ड के नीचे दिया गया है।
installer-package-blocks = "इंस्टॉल करें" बंद है, क्योंकि यह पैकेज इस स्थान पर नहीं जा सकता। कारण ऊपर दिया गया है।
installer-list-changed = यहाँ यह जो करेगा, वह बदल गया है, इसलिए कुछ भी इंस्टॉल नहीं किया गया। नीचे दी गई सूची देखें, और अगर आप यही चाहते हैं, तो फिर से "इंस्टॉल करें" पर क्लिक करें।
installer-cancelled-early = रद्द किया गया। कुछ भी नहीं बदला गया।

## How far an install has got, beside its progress bar

# { $step } is one of the steps below. { $done } and { $total } are how many of its items are done, and how many there are.
installer-step-counted = { $step }: { $total } में से { $done }…
# { $step } is one of the steps below, when it does not count its items.
installer-step-uncounted = { $step }…
installer-step-getting-ready = तैयारी की जा रही है…
installer-step-cancelling = वर्तमान चरण के बाद रद्द किया जा रहा है…
installer-step-putting-back = किए गए परिवर्तनों को पहले जैसा किया जा रहा है
installer-step-unpacking = अनपैक किया जा रहा है
installer-step-checking-package = पैकेज की जाँच की जा रही है
installer-step-recovering = एक पिछले इंस्टॉल को पूर्ववत किया जा रहा है जो कभी पूरा नहीं हुआ
# What a preview is doing, whatever step it is on.
installer-step-previewing = पता लगाया जा रहा है कि क्या बदलेगा
installer-step-runtimes = रनटाइम इंस्टॉल किए जा रहे हैं
installer-step-files = फ़ाइलें कॉपी की जा रही हैं
installer-step-services = सेवाएँ सेट अप की जा रही हैं
installer-step-drivers = ड्राइवर सेट अप किए जा रहे हैं
installer-step-registry = रजिस्ट्री सेटिंग्स लिखी जा रही हैं
installer-step-license = लाइसेंस डेटा पुनर्स्थापित किया जा रहा है
installer-step-tasks = शेड्यूल किए गए कार्य सेट अप किए जा रहे हैं
installer-step-shortcuts = शॉर्टकट बनाए जा रहे हैं
installer-step-environment = परिवेश चर सेट किए जा रहे हैं
installer-step-settings-entry = ऐप को सेटिंग्स में जोड़ा जा रहा है
installer-step-verifying = जो इंस्टॉल किया गया, उसकी जाँच की जा रही है
installer-step-undoing = पूर्ववत किया जा रहा है

## When an install cannot go ahead or stops

installer-carries-no-package = इस इंस्टॉलर में कोई पैकेज नहीं है।
installer-stopped-unexpectedly = इंस्टॉल अनपेक्षित रूप से रुक गया।
# { $reason } is what went wrong, as the program says it.
installer-stopped-unexpectedly-because = इंस्टॉल अनपेक्षित रूप से रुक गया: { $reason }। अगर यह शुरू हो चुका था, तो इंस्टॉलर को फिर से चलाने पर पहले वह पूर्ववत किया जाता है जो इसने किया था।
# { $error } is what went wrong reading it.
installer-package-read-failed = पैकेज पढ़ा नहीं जा सका: { $error }
installer-no-programdata = इस PC का ProgramData फ़ोल्डर नहीं मिला, इसलिए कुछ भी इंस्टॉल नहीं किया गया: इंस्टॉल को पूर्ववत करने के लिए उसका रिकॉर्ड वहीं रखा जाता है।
# { $error } is why, such as that there is nowhere only administrators can get into.
installer-staging-failed = स्टेजिंग फ़ोल्डर नहीं बनाया जा सका: { $error }
# { $error } is what went wrong unpacking it.
installer-unpack-failed = पैकेज अनपैक नहीं किया जा सका: { $error }
installer-staging-nowhere = पैकेज को अनपैक करने के लिए ऐसा कोई स्थान नहीं है जहाँ केवल व्यवस्थापक पहुँच सकें
# { $failure } is a folder and what went wrong there, such as "C:\ProgramData\WinAppMigrator: Access is denied.".
installer-staging-nowhere-look-at = पैकेज को अनपैक करने के लिए ऐसा कोई स्थान नहीं है जहाँ केवल व्यवस्थापक पहुँच सकें। इसे जाँचें: { $failure }

## What came of an install, or of a preview

installer-result = इंस्टॉल का परिणाम
installer-result-preview = पूर्वावलोकन: अभी तक कुछ भी नहीं बदला गया है
installer-result-installed = एप्लिकेशन इंस्टॉल हो गया और उसने अपनी हैश जाँच पास कर ली।
installer-result-installed-without-left-out = एप्लिकेशन इंस्टॉल हो गया और उसने अपनी हैश जाँच पास कर ली। नीचे जिन चीज़ों को छोड़ा गया बताया गया है, वे इसमें शामिल नहीं हैं।
installer-result-not-all-put-back = इंस्टॉल विफल रहा, और इसके द्वारा किए गए सभी परिवर्तन पहले जैसे नहीं किए जा सके। जो बचा है, वह नीचे सूचीबद्ध है, और अगला इंस्टॉल फिर से प्रयास करता है।
installer-result-cancelled-put-back = इंस्टॉल रद्द कर दिया गया, और इसके द्वारा किए गए परिवर्तन पहले जैसे कर दिए गए।
installer-result-failed-put-back = इंस्टॉल विफल रहा, और इसके द्वारा किए गए परिवर्तन पहले जैसे कर दिए गए।
installer-result-not-clean = इंस्टॉल ठीक से पूरा नहीं हुआ। नीचे दी गई समस्याओं की समीक्षा करें।
installer-result-changes-put-back = पहले जैसे किए गए परिवर्तन
installer-result-changes-made = किए गए परिवर्तन
installer-result-changes-planned = नियोजित परिवर्तन
installer-result-warnings = चेतावनियाँ
installer-result-errors = त्रुटियाँ
installer-result-package-valid = पैकेज हैश मान्य हैं
installer-result-package-invalid = पैकेज हैश अमान्य हैं
installer-result-files-valid = इंस्टॉल की गई फ़ाइलें मान्य हैं
installer-result-files-invalid = इंस्टॉल की गई फ़ाइलें अमान्य हैं
installer-result-reboot = किसी निर्भरता को पूरा होने के लिए पुनः प्रारंभ की आवश्यकता है। Windows को पुनः प्रारंभ करें, फिर ऐप लॉन्च करें।
# A heading over the list of what was changed. { $count } is how many.
installer-result-changes = परिवर्तन ({ $count })
# After the first part of a long list. { $count } is how many more there are.
installer-result-changes-more = और { $count } अन्य।
# Over the path of the install's log file.
installer-log = लॉग

## The installer's command line. Shown in English: scripts read it. A { $count } is plain digits, with no separators.

installer-cli-stub = WinAppMigrator इंस्टॉलर स्टब — इसमें कोई एप्लिकेशन पैकेज नहीं है।
installer-cli-stub-how = WinAppMigrator ऐप से एक इंस्टॉलर बनाएँ, फिर बनाई गई .exe फ़ाइल को लक्ष्य PC पर चलाएँ।
installer-cli-usage-title = WinAppMigrator सेल्फ़-एक्सट्रैक्टिंग इंस्टॉलर।
installer-cli-usage = ग्राफ़िकल इंस्टॉलर खोलने के लिए इसे बिना किसी तर्क के चलाएँ, या:
installer-cli-usage-preview = फिर से इंस्टॉल का पूर्वावलोकन करें: --dry-run [--target-root <DIR>]
installer-cli-usage-apply = फिर से इंस्टॉल करें:              --apply [--target-root <DIR>]
installer-cli-no-package = इंस्टॉलर में कोई एम्बेड किया गया पैकेज नहीं है
installer-cli-no-programdata = इस PC का ProgramData फ़ोल्डर नहीं मिला, इसलिए कुछ भी इंस्टॉल नहीं किया गया: इंस्टॉल को पूर्ववत करने के लिए उसका रिकॉर्ड वहीं रखा जाता है
# { $path } is the folder asked for, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-folder-has-files = { $path } का उपयोग नहीं किया जा सकता: { $problem }। --force केवल तभी दें जब आप जानते हों कि यह इसी ऐप का है
installer-cli-no-location = यह पैकेज नहीं बताता कि इसका ऐप कहाँ था, इसलिए --target-root दें
# { $path } is where the app was on the other PC, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-location-refused = इस पैकेज का ऐप { $path } में था, और उसका उपयोग नहीं किया जा सकता: { $problem }। --target-root दें
# { $summary } is how much room it needs where, such as "up to 824.0 MB on C: (356.0 GB free)".
installer-cli-room = डिस्क स्थान: { $summary }
# { $note } is something to know about the install.
installer-cli-note = नोट: { $note }
# { $count } is how many things it sets up beyond its files.
installer-cli-sets-up = सेट अप करता है: { $count }
installer-cli-applied = फिर से इंस्टॉल लागू किया गया।
installer-cli-previewed = फिर से इंस्टॉल का पूर्वावलोकन किया गया।
# { $count } is how many changes the install logged.
installer-cli-changes-logged = लॉग किए गए परिवर्तन: { $count }
# { $count } is how many warnings.
installer-cli-warnings = चेतावनियाँ: { $count }
# { $count } is how many errors.
installer-cli-errors = त्रुटियाँ: { $count }
# { $count } is how many things the package asked for and did not get.
installer-cli-refused = अस्वीकृत: { $count }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-package-valid = पैकेज हैश मान्य: { $valid }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-reinstall-valid = फिर से इंस्टॉल हैश मान्य: { $valid }
# { $id } is the install's id, such as wam-0f3a….
installer-cli-install-id = इंस्टॉल ID: { $id }
installer-cli-put-back = इंस्टॉल विफल रहा, इसलिए इसके द्वारा किए गए परिवर्तन पहले जैसे कर दिए गए।
# { $path } is the log file.
installer-cli-log = लॉग: { $path }
# { $path } is the log file. Printed before an error.
installer-cli-error-log = लॉग: { $path }
# { $error } is what went wrong.
installer-cli-error = त्रुटि: { $error }

## The uninstaller's window

uninstaller-window-title = अनइंस्टॉल करें
# The window's title, and its heading. { $app_name } is the name of the app it uninstalls.
uninstaller-window-title-app = { $app_name } अनइंस्टॉल करें
uninstaller-tagline = इसके इंस्टॉलर ने जो किया था, उसे पूर्ववत करता है
uninstaller-reading = इस इंस्टॉल का रिकॉर्ड पढ़ा जा रहा है…
uninstaller-stopped-unexpectedly = अनइंस्टॉल अनपेक्षित रूप से रुक गया।
uninstaller-what-will-be-removed = क्या हटाया जाएगा
uninstaller-install-location = इंस्टॉल स्थान
uninstaller-files = फ़ाइलें
uninstaller-registry-values = रजिस्ट्री मान
uninstaller-services = सेवाएँ
uninstaller-drivers = ड्राइवर
uninstaller-scheduled-tasks = शेड्यूल किए गए कार्य
uninstaller-environment = परिवेश
uninstaller-plan-stays = आपके द्वारा जोड़ी या बदली गई फ़ाइलें, उनके फ़ोल्डरों सहित, बनी रहती हैं।
# { $count } is how many files the install replaced.
uninstaller-plan-coming-back =
    { $count ->
        [one] इस इंस्टॉल द्वारा प्रतिस्थापित की गई { $count } फ़ाइल वापस आती है, या अगर उसकी जगह वाली फ़ाइल तब से बदल गई है, तो उसके बगल में रखी जाती है।
       *[other] इस इंस्टॉल द्वारा प्रतिस्थापित की गई { $count } फ़ाइलें वापस आती हैं, या अगर उनकी जगह वाली फ़ाइल तब से बदल गई है, तो उसके बगल में रखी जाती हैं।
    }
# { $runtimes } is a list of runtimes, such as "Microsoft Visual C++ Redistributable, .NET Desktop Runtime 8".
uninstaller-plan-runtimes = इसके द्वारा इंस्टॉल किए गए रनटाइम बने रहते हैं, क्योंकि अन्य प्रोग्राम उनका उपयोग कर सकते हैं: { $runtimes }।
uninstaller-options = विकल्प
uninstaller-remove-user-data = उपयोगकर्ता डेटा और लाइसेंस डेटा भी हटाएँ
# { $count } is how many items of the app's settings, data, and license there are.
uninstaller-remove-user-data-detail =
    { $count ->
        [one] इसकी सेटिंग्स, डेटा और लाइसेंस का { $count } आइटम। जब तक यह विकल्प चालू न हो, यह बना रहता है। इसने जो प्रतिस्थापित किया था, वह वापस आता है, या अगर यह तब से बदल गया है, तो इसके बगल में रखा जाता है।
       *[other] इसकी सेटिंग्स, डेटा और लाइसेंस के { $count } आइटम। जब तक यह विकल्प चालू न हो, वे बने रहते हैं। उन्होंने जो प्रतिस्थापित किया था, वह वापस आता है, या अगर वे तब से बदल गए हैं, तो उनके बगल में रखा जाता है।
    }
uninstaller-uninstall = अनइंस्टॉल करें
uninstaller-cancel = रद्द करें
uninstaller-close = बंद करें
# { $done } and { $total } are how many of the install's items are undone, and how many there are.
uninstaller-step-counted = इंस्टॉल पूर्ववत किया जा रहा है: { $total } में से { $done }…
uninstaller-step = इंस्टॉल पूर्ववत किया जा रहा है…
uninstaller-done = ऐप अनइंस्टॉल कर दिया गया।
uninstaller-done-except = नीचे सूचीबद्ध चीज़ों को छोड़कर, ऐप अनइंस्टॉल कर दिया गया।
uninstaller-waits-for-drive = इसका कुछ भाग ऐसी ड्राइव पर है जो कनेक्ट नहीं है। उसे कनेक्ट करें, और सेटिंग्स से फिर से अनइंस्टॉल करें।
uninstaller-not-all-undone = इसका सब कुछ पूर्ववत नहीं किया जा सका। जो इसका उपयोग कर रहा है, उसे बंद करें, और सेटिंग्स से फिर से अनइंस्टॉल करें।
# Over the path of the uninstall's log file.
uninstaller-log = लॉग
uninstaller-undone = पूर्ववत किया गया
uninstaller-left-in-place = यथास्थान छोड़ा गया
uninstaller-could-not-be-undone = पूर्ववत नहीं किया जा सका
# After the first part of a long list. { $count } is how many more there are.
uninstaller-and-more = और { $count } अन्य।
# Said as the reason when there is no record of the install to read.
uninstaller-no-programdata = इस PC का ProgramData फ़ोल्डर नहीं मिला, इसलिए पढ़ने के लिए कोई रिकॉर्ड नहीं है

## When the uninstaller's window cannot open. Each is a paragraph of a message box, followed by a command to type.

uninstaller-window-failed-title = WinAppMigrator अनइंस्टॉलर
# { $error } is what went wrong, as Windows or the graphics driver says it.
uninstaller-window-failed = अनइंस्टॉलर की विंडो नहीं खुल सकी: { $error }
uninstaller-window-failed-quiet = यह विंडो के बिना भी, व्यवस्थापक कमांड प्रॉम्प्ट से अनइंस्टॉल कर सकता है:
# Stands in for this program's own path in that command, if it cannot be found.
uninstaller-this-program = यह प्रोग्राम

## The uninstaller's command line (--quiet). Shown in English: scripts read it.

# { $app_name } is the name of the app.
uninstaller-cli-uninstalling = { $app_name } अनइंस्टॉल किया जा रहा है।
# { $path } is the log file.
uninstaller-cli-log = लॉग: { $path }
# { $error } is what went wrong.
uninstaller-cli-error = त्रुटि: { $error }
