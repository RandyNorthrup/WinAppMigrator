# Words the core writes for people as it installs an app from a package: why an install will not start, the list of
# what it did (or for a preview, what it would do) that the installer window shows, its warnings, the record it keeps of
# each install so that it can be undone, and the core's errors. Message names start with core-.
#
# The list of what an install did is short and plain, one line for each thing, as the installer window lists them:
# "copied", "restored", "would copy". Words such as --force and --apply are command-line options, and LocalService,
# SYSTEM, ProgramData, and %AppData% are as Windows names them: keep them as they are.


## Errors: what went wrong, when the core cannot go on. Each is shown on its own, or after words that say what was being
## done.

core-error-list-changed = यह इंस्टॉल जो करेगा, वह सहमति दिए जाने के बाद से बदल गया है, इसलिए कुछ भी इंस्टॉल नहीं किया गया
# { $path } is a file or folder, such as C:\Program Files\App\app.exe, or words that name one, such as "the installer".
# { $error } is what Windows said went wrong.
core-error-io = { $path } के लिए I/O विफल रहा: { $error }
# { $detail } is what went wrong.
core-error-registry = रजिस्ट्री पढ़ना विफल रहा: { $detail }
# { $path } is a program file. { $detail } is what was wrong with it.
core-error-pe = { $path } के लिए PE पार्स करना विफल रहा: { $detail }
# { $detail } is what was wrong, already in words.
core-error-invalid-input = अमान्य इनपुट: { $detail }
# { $detail } is what went wrong.
core-error-serialization = रिपोर्ट को सीरियलाइज़ करना विफल रहा: { $detail }
# { $detail } is what went wrong.
core-error-archive = संग्रह कार्रवाई विफल रही: { $detail }
# { $detail } is what went wrong.
core-error-network = नेटवर्क कार्रवाई विफल रही: { $detail }
# { $detail } is what went wrong.
core-error-time-format = समय का स्वरूपण विफल रहा: { $detail }
core-error-cancelled = इसे रद्द कर दिया गया
# What is said when part of the program stopped unexpectedly and gave no reason.
core-error-panic-no-reason = कोई कारण नहीं दिया गया

## The record of each install, kept so that it can be undone.

# { $kind } is a number that says what type of registry value it is.
core-journal-unknown-value-type = अज्ञात रजिस्ट्री मान प्रकार { $kind }
core-journal-not-elevated = व्यवस्थापक अधिकारों के साथ नहीं चल रहा
core-journal-needs-admin = इंस्टॉल करने के लिए व्यवस्थापक अधिकारों की आवश्यकता है, ताकि इसका रिकॉर्ड ऐसी जगह रखा जा सके जहाँ अन्य लोग उसे बदल न सकें
# { $path } is a folder.
core-journal-not-an-install-folder = यह किसी इंस्टॉल का फ़ोल्डर नहीं है: { $path }
# { $path } is the file the record is kept in.
core-journal-not-written = इस इंस्टॉल का रिकॉर्ड नहीं लिखा जा सका: { $path }
# { $path } is a folder.
core-journal-path-escapes-root = पथ इच्छित रूट से बाहर जाता है: { $path }
# { $path } is a folder.
core-journal-nothing-exists = इसमें से कुछ भी इस PC पर मौजूद नहीं है: { $path }
core-journal-busy = कोई अन्य इंस्टॉल या अनइंस्टॉल चल रहा है। उसके पूरा होने की प्रतीक्षा करें, और फिर से प्रयास करें
# Said of a file that was to get back the permissions it had.
core-journal-no-folder = इसका कोई फ़ोल्डर नहीं है
# Said of the permissions a file had, which were kept to give back to it.
core-journal-permissions-unreadable = जो सहेजा गया था, उसे पढ़ा नहीं जा सका
# { $folder } is the folder of one install's record.
core-journal-unreadable-empty = { $folder } में मौजूद इंस्टॉल के रिकॉर्ड का उपयोग नहीं किया जा सकता: यह खाली है
# { $folder } is the folder of one install's record.
core-journal-unreadable-no-header = { $folder } में मौजूद इंस्टॉल के रिकॉर्ड का उपयोग नहीं किया जा सकता: इसकी शुरुआत में यह नहीं लिखा है कि यह किस बारे में है
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-version = { $folder } में मौजूद इंस्टॉल के रिकॉर्ड का उपयोग नहीं किया जा सकता: इसे इस प्रोग्राम के किसी अन्य संस्करण ने लिखा था
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-install = { $folder } में मौजूद इंस्टॉल के रिकॉर्ड का उपयोग नहीं किया जा सकता: इसके अनुसार यह किसी अन्य इंस्टॉल का है
# { $id } is an install's id, such as wam-1a2b3c.
core-journal-not-an-install-id = यह किसी इंस्टॉल की ID नहीं है: { $id }

## Why an install will not start, or stopped before it changed anything.

core-restore-apply-and-dry-run = --apply और --dry-run दोनों एक साथ नहीं दिए जा सकते
# { $path } is the folder that was chosen to install in.
core-restore-location-not-full-path = इंस्टॉल स्थान एक पूरा पथ होना चाहिए: "{ $path }"
# { $problem } says what is wrong with the folder, already in words. { $path } is the folder.
core-restore-location-unusable = उस इंस्टॉल स्थान का उपयोग नहीं किया जा सकता: { $problem }: "{ $path }"
core-restore-needs-admin-plan = पुनर्स्थापना योजना में ऐसे रजिस्ट्री, सेवा, शेड्यूल किए गए कार्य, शॉर्टकट, या सुरक्षित लक्ष्य परिवर्तन शामिल हैं जिन्हें केवल व्यवस्थापक कर सकते हैं
# { $path } is the folder that was chosen to install in.
core-restore-needs-admin-target = लक्ष्य रूट के लिए व्यवस्थापक अधिकारों की आवश्यकता है: { $path }
core-restore-needs-admin-user-data = ProgramData या अन्य सुरक्षित स्थानों में उपयोगकर्ता डेटा पुनर्स्थापित करने के लिए व्यवस्थापक अधिकारों की आवश्यकता है
# { $error } is why the install stopped. { $failed } is what could not be put back, as sentences of their own.
core-restore-not-all-put-back = { $error }। जो किया गया था, वह सब पहले जैसा नहीं किया जा सका: { $failed }
# { $error } is what went wrong.
core-restore-nowhere-to-keep-record = वह रिकॉर्ड रखने के लिए कोई सुरक्षित जगह नहीं है जिससे यह इंस्टॉल पूर्ववत किया जा सके, इसलिए कुछ भी इंस्टॉल नहीं किया गया: { $error }
# { $path } is a file the app would have put there.
core-restore-already-exists = { $path } पहले से मौजूद है, इसलिए कुछ भी इंस्टॉल नहीं किया गया। इसे प्रतिस्थापित करने के लिए --force का उपयोग करें
# { $root } is a folder the app has files for. { $link } is that folder, or one it is in.
core-restore-root-under-link = इस ऐप के पास { $root } में पुनर्स्थापित करने के लिए चीज़ें हैं, और { $link } किसी अन्य जगह का लिंक है, जिसे अन्य लोग बदल सकते हैं, इसलिए कुछ भी इंस्टॉल नहीं किया गया। अगर वहाँ जाने वाली चीज़ उपयोगकर्ता डेटा या लाइसेंस डेटा है, तो इसे इसके उपयोगकर्ता डेटा और लाइसेंस डेटा के बिना इंस्टॉल करें। अगर यह किसी स्टार्टअप फ़ोल्डर में मौजूद शॉर्टकट है, तो उस फ़ोल्डर को वापस वहीं रखें जहाँ वह था
core-restore-hash-check-failed = पैकेज अपनी हैश जाँच में विफल रहा और कुछ भी इंस्टॉल नहीं किया गया
# { $detail } is the first thing found wrong, already in words.
core-restore-hash-check-failed-because = पैकेज अपनी हैश जाँच में विफल रहा और कुछ भी इंस्टॉल नहीं किया गया: { $detail }
# { $path } is a file or folder.
core-restore-refuse-records-folder = वहाँ लिखने से इनकार किया जा रहा है जहाँ इंस्टॉल अपने रिकॉर्ड रखते हैं: { $path }
# { $path } is a file or folder.
core-restore-refuse-outside = इंस्टॉल स्थान और ज्ञात डेटा फ़ोल्डरों के बाहर लिखने से इनकार किया जा रहा है: { $path }
# { $path } is a file.
core-restore-no-parent-folder = गंतव्य का कोई पैरेंट फ़ोल्डर नहीं है: { $path }
# { $path } is where a file was to go.
core-restore-folder-in-the-way = जहाँ इस फ़ाइल को जाना है, वहाँ एक फ़ोल्डर है: { $path }
# { $path } is a file in the package.
core-restore-file-changed = { $path } अब वैसी नहीं है जैसी पैकेज में है, इसलिए इसे इंस्टॉल नहीं किया गया
# { $path } is a file in the package.
core-restore-source-is-link = { $path } एक लिंक है, और इसका अनुसरण नहीं किया जाना है
# { $path } is a file in the package.
core-restore-source-became-link = खोले जाते समय { $path } एक लिंक बन गया
# Why a service did not start, after "did not start:".
core-restore-service-stopped-by-itself = यह अपने आप फिर से रुक गई
# Why part of the app was left out, after "was left out:".
core-restore-com-class-someone-elses = यह COM क्लास इस PC पर पहले से किसी और चीज़ की है

## What an install did, one line for each thing, or for a preview, what it would do. { $source } is a file in the
## package, and { $destination } and { $path } are where it goes on this PC. { $key } is a registry key, such as
## HKLM\Software\App.

# { $package_valid } is true or false, and { $restore_valid } is true, false, or not_run: keep them as they are.
core-restore-verification-result = पुनर्स्थापना के बाद का सत्यापन package_valid={ $package_valid } restore_valid={ $restore_valid }
# { $name } is a service's name.
core-restore-started-service = सेवा { $name } प्रारंभ की गई
core-restore-would-copy = { $source } -> { $destination } कॉपी किया जाएगा
core-restore-copied = { $source } -> { $destination } कॉपी किया गया
core-restore-would-create-empty-folder = खाली फ़ोल्डर { $path } बनाया जाएगा
core-restore-created-empty-folder = खाली फ़ोल्डर { $path } बनाया गया
# { $value } is the name of a value in the key.
core-restore-would-restore-registry-value = रजिस्ट्री { $key } { $value } पुनर्स्थापित किया जाएगा
core-restore-would-restore-registry-key = रजिस्ट्री { $key } (कुंजी) पुनर्स्थापित की जाएगी
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-would-add-firewall-rule = { $program } के लिए फ़ायरवॉल नियम { $name } जोड़ा जाएगा
# { $name } is a firewall rule's name.
core-restore-firewall-rule-already-there = फ़ायरवॉल नियम { $name } पहले से मौजूद है
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-added-firewall-rule = { $program } के लिए फ़ायरवॉल नियम { $name } जोड़ा गया
# { $name } is a service's name, { $command } what it runs, and { $account } the account it runs as, such as LocalService.
core-restore-would-restore-service = सेवा { $name } -> { $command } पुनर्स्थापित की जाएगी ({ $account } के रूप में चलती है)
# { $name } is a runtime, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-restore-prerequisite-present = पूर्वापेक्षा पहले से मौजूद है: { $name }
# { $name } is a runtime that Windows has as a Windows feature.
core-restore-would-install-prerequisite-feature = पूर्वापेक्षा { $name } इंस्टॉल की जाएगी (Windows सुविधा)
# { $name } is a runtime whose installer is in the package.
core-restore-would-install-prerequisite-bundled = पूर्वापेक्षा { $name } इंस्टॉल की जाएगी (बंडल किया गया इंस्टॉलर)
# { $name } is a runtime that would come through winget, which is a program's name.
core-restore-would-install-prerequisite-winget = पूर्वापेक्षा { $name } इंस्टॉल की जाएगी (winget)
# { $name } is a runtime.
core-restore-installed-prerequisite = पूर्वापेक्षा { $name } इंस्टॉल की गई
# { $name } is a runtime.
core-restore-installed-prerequisite-reboot = पूर्वापेक्षा { $name } इंस्टॉल की गई (पुनः प्रारंभ आवश्यक)
core-restore-would-restore-license = लाइसेंस { $source } -> { $destination } पुनर्स्थापित किया जाएगा
core-restore-restored-license = लाइसेंस { $source } -> { $destination } पुनर्स्थापित किया गया
core-restore-would-restore-license-registry = लाइसेंस रजिस्ट्री { $key } पुनर्स्थापित की जाएगी
# { $name } is a driver's name.
core-restore-driver-not-restored = ड्राइवर स्वचालित रूप से पुनर्स्थापित नहीं किया गया: { $name }
# { $name } is a driver's name.
core-restore-would-install-driver = ड्राइवर { $name } -> { $destination } इंस्टॉल किया जाएगा
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-would-restore-task = शेड्यूल किया गया कार्य { $name } पुनर्स्थापित किया जाएगा
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-startup-shortcut = स्टार्टअप शॉर्टकट { $shortcut } -> { $target } पुनर्स्थापित किया जाएगा
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-shortcut = शॉर्टकट { $shortcut } -> { $target } पुनर्स्थापित किया जाएगा
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-would-restore-variable =
    { $scope ->
        [user] उपयोगकर्ता परिवेश चर { $name } पुनर्स्थापित किया जाएगा
       *[machine] मशीन परिवेश चर { $name } पुनर्स्थापित किया जाएगा
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-left-variable =
    { $scope ->
        [user] उपयोगकर्ता परिवेश चर { $name } को जैसा था वैसा ही छोड़ा गया: इसका कोई भी भाग इस ऐप का नहीं है
       *[machine] मशीन परिवेश चर { $name } को जैसा था वैसा ही छोड़ा गया: इसका कोई भी भाग इस ऐप का नहीं है
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-restored-variable =
    { $scope ->
        [user] उपयोगकर्ता परिवेश चर { $name } पुनर्स्थापित किया गया
       *[machine] मशीन परिवेश चर { $name } पुनर्स्थापित किया गया
    }
core-restore-set-uninstaller = { $key } के लिए अनइंस्टॉलर सेट किया गया
core-restore-placed-driver-image = ड्राइवर इमेज { $source } -> { $destination } रखी गई
# { $name } is a driver's name.
core-restore-configured-driver = ड्राइवर { $name } कॉन्फ़िगर किया गया
# { $name } is a driver's name.
core-restore-created-driver = ड्राइवर { $name } बनाया गया
core-restore-ensured-registry-key = रजिस्ट्री कुंजी { $key } का होना सुनिश्चित किया गया
# { $value } is the name of a value in the key.
core-restore-restored-registry-value = रजिस्ट्री { $key } { $value } पुनर्स्थापित किया गया
# "(Default)" is how Windows names the value a key has with no name: write it as Registry Editor does.
core-restore-restored-registry-default = रजिस्ट्री { $key } (डिफ़ॉल्ट) पुनर्स्थापित किया गया
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-configured-service = सेवा { $name } कॉन्फ़िगर की गई ({ $account } के रूप में चलती है)
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-created-service = सेवा { $name } बनाई गई ({ $account } के रूप में चलती है)
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-restored-task = शेड्यूल किया गया कार्य { $name } पुनर्स्थापित किया गया
core-restore-restored-shortcut = शॉर्टकट { $path } पुनर्स्थापित किया गया

## Warnings an install gives, and why it failed. They are shown after it, on the PC it ran on.

core-warning-restore-earlier-not-trusted = कभी पूरे न हुए एक इंस्टॉल का रिकॉर्ड जैसा था वैसा ही छोड़ दिया गया, क्योंकि अन्य लोग उसे बदल सकते हैं। उस इंस्टॉल ने जो भी बदला था, वह अब भी मौजूद है।
# { $app_name } is the app's name. { $count } is how many changes were put back.
core-warning-restore-earlier-undone =
    { $count ->
        [one] { $app_name } का एक पिछला इंस्टॉल कभी पूरा नहीं हुआ, इसलिए पहले उसे पूर्ववत किया गया। { $count } परिवर्तन पहले जैसा किया गया।
       *[other] { $app_name } का एक पिछला इंस्टॉल कभी पूरा नहीं हुआ, इसलिए पहले उसे पूर्ववत किया गया। { $count } परिवर्तन पहले जैसे किए गए।
    }
# { $app_name } is the app's name.
core-warning-restore-earlier-never-finished = { $app_name } का एक पिछला इंस्टॉल कभी पूरा नहीं हुआ। इंस्टॉल करने पर पहले उसे पूर्ववत किया जाता है।
core-warning-restore-user-data-real-locations = उपयोगकर्ता डेटा लक्ष्य रूट के बजाय इस मशीन के वास्तविक %AppData%/%ProgramData% स्थानों पर पुनर्स्थापित किया जाता है।
# { $error } is what went wrong.
core-warning-restore-commit-failed = इस इंस्टॉल का रिकॉर्ड पूरा नहीं किया जा सका: { $error }
# { $name } is a service's name.
core-warning-restore-service-still-starting = सेवा { $name } को प्रारंभ होने के लिए कहा गया था, और वह अभी पूरी तरह प्रारंभ नहीं हुई है। हो सकता है कि वह अभी प्रारंभ हो जाए। जब यह PC अगली बार प्रारंभ होगा, तब भी वह अपने आप प्रारंभ हो जाएगी।
# { $name } is a service's name. { $reason } is why it did not start, as Windows said it.
core-warning-restore-service-not-started = सेवा { $name } इंस्टॉल है, लेकिन प्रारंभ नहीं हुई: { $reason }। जब तक यह प्रारंभ नहीं होती, तब तक हो सकता है कि ऐप काम न करे। जब यह PC अगली बार प्रारंभ होगा, तब Windows फिर से प्रयास करेगा।
# { $name } is a service's name. { $reason } is why it could not be stopped.
core-warning-restore-service-not-stopped = सेवा { $name } को पहले रोका नहीं जा सका: { $reason }। हो सकता है कि यह PC अगली बार प्रारंभ होने तक यह पुराना प्रोग्राम चलाती रहे।
core-warning-restore-install-cancelled = इंस्टॉल रद्द कर दिया गया।
core-warning-restore-preview-cancelled = पूर्वावलोकन रद्द कर दिया गया।
# { $item } is what was left out, such as a file or a service's name. { $reason } is why, already in words.
core-warning-restore-left-out = { $item } छोड़ा गया: { $reason }।
# { $error } is what went wrong.
core-warning-restore-verification-failed = पुनर्स्थापना के बाद का सत्यापन नहीं चल सका: { $error }
# { $runner } says what relies on the file: a service, a scheduled task, a firewall rule, a shortcut, or a registry value.
# { $path } is the file. { $name } is the name of what relies on it. { $refusal } is why the file was not replaced,
# already in words.
core-warning-restore-program-would-run =
    { $runner ->
        [service] इस पैकेज को { $path } प्रतिस्थापित करने का अधिकार नहीं है, और सेवा { $name } इस पर निर्भर है, इसलिए कुछ भी इंस्टॉल नहीं किया गया: { $refusal }
        [task] इस पैकेज को { $path } प्रतिस्थापित करने का अधिकार नहीं है, और शेड्यूल किया गया कार्य { $name } इस पर निर्भर है, इसलिए कुछ भी इंस्टॉल नहीं किया गया: { $refusal }
        [firewall] इस पैकेज को { $path } प्रतिस्थापित करने का अधिकार नहीं है, और फ़ायरवॉल नियम { $name } इस पर निर्भर है, इसलिए कुछ भी इंस्टॉल नहीं किया गया: { $refusal }
        [shortcut] इस पैकेज को { $path } प्रतिस्थापित करने का अधिकार नहीं है, और शॉर्टकट { $name } इस पर निर्भर है, इसलिए कुछ भी इंस्टॉल नहीं किया गया: { $refusal }
       *[registry] इस पैकेज को { $path } प्रतिस्थापित करने का अधिकार नहीं है, और रजिस्ट्री मान { $name } इस पर निर्भर है, इसलिए कुछ भी इंस्टॉल नहीं किया गया: { $refusal }
    }
core-warning-restore-destination-exists = गंतव्य मौजूद है; अधिलेखित करने के लिए --force का उपयोग करें।
# { $path } is a file. { $error } is what went wrong.
core-warning-restore-file-not-written = { $path } नहीं लिखा जा सका: { $error }
core-warning-restore-not-a-folder = जहाँ यह खाली फ़ोल्डर जाना है, वहाँ फ़ोल्डर के अलावा कुछ और है।
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-not-looked-at = { $path } को देखा नहीं जा सका: { $error }
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-folder-not-made = { $path } नहीं बनाया जा सका: { $error }
# Why part of the app was left out, after "was left out:".
core-warning-restore-registration-exists = इस PC पर यह पहले से है और किसी और चीज़ पर सेट है, जिसे कोई इंस्टॉल अपने नियंत्रण में नहीं लेता
core-warning-restore-registry-review = रजिस्ट्री मान संग्रहीत संरचित डेटा से पुनर्स्थापित किए गए, और ज्ञात ऐप पथ लक्ष्य रूट के अनुसार फिर से लिखे गए।
# { $error } is what went wrong.
core-warning-restore-firewall-unreadable = Windows फ़ायरवॉल के नियम पढ़े नहीं जा सके: { $error }
# Why a firewall rule was left out, after "was left out:".
core-warning-restore-firewall-rule-exists = इस प्रोग्राम के लिए इस नाम का एक नियम पहले से मौजूद है और किसी और चीज़ पर सेट है, जिसे कोई इंस्टॉल नहीं बदलता
# { $name } is a runtime.
core-warning-restore-prerequisite-unknown = { $name }: इस रनटाइम के लिए कोई ज्ञात इंस्टॉलर नहीं है।
# { $name } is a runtime.
core-warning-restore-prerequisite-needs-admin = { $name } को इंस्टॉल करने के लिए व्यवस्थापक अधिकारों की आवश्यकता है; इंस्टॉलर को व्यवस्थापक के रूप में फिर से चलाएँ।
# { $name } is a runtime.
core-warning-restore-prerequisite-reboot = { $name } इंस्टॉल किया गया, लेकिन इसे पूरा करने के लिए पुनः प्रारंभ आवश्यक है।
# { $error } is what went wrong.
core-warning-restore-skipped-not-recorded = छोड़े गए रनटाइम को रिकॉर्ड नहीं किया जा सका: { $error }
# { $name } is a runtime. { $reason } is why it was skipped, already in words.
core-warning-restore-prerequisite-skipped = { $name }: { $reason }
# { $name } is a runtime. { $reason } is why it could not be installed. { $url } is a web address to get it from.
core-warning-restore-prerequisite-failed = { $name } इंस्टॉल नहीं किया जा सका: { $reason }। इसे { $url } से प्राप्त करें।
# { $error } is what went wrong.
core-warning-restore-runtime-not-recorded = इंस्टॉल किए गए रनटाइम को रिकॉर्ड नहीं किया जा सका: { $error }
# { $error } is what went wrong.
core-warning-restore-license-source-invalid = पैकेज किया गया लाइसेंस पथ अमान्य है: { $error }
core-warning-restore-license-needs-admin = लाइसेंस डेटा को किसी सुरक्षित स्थान पर पुनर्स्थापित करने के लिए व्यवस्थापक अधिकारों की आवश्यकता है।
core-warning-restore-license-exists = लाइसेंस गंतव्य मौजूद है; अधिलेखित करने के लिए --force का उपयोग करें।
# { $error } is what went wrong.
core-warning-restore-license-failed = लाइसेंस डेटा पुनर्स्थापित नहीं किया जा सका: { $error }
core-warning-restore-driver-needs-admin = ड्राइवर पंजीकरण के लिए व्यवस्थापक अधिकारों की आवश्यकता है; इंस्टॉलर को व्यवस्थापक के रूप में फिर से चलाएँ।
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-warning-restore-task-runs-as-system = शेड्यूल किया गया कार्य { $name } SYSTEM के रूप में, इस PC पर पूर्ण अधिकारों के साथ चलता है।
core-warning-restore-shortcut-exists = शॉर्टकट मौजूद है; अधिलेखित करने के लिए --force का उपयोग करें।
core-warning-restore-shortcut-all-users = एक या अधिक पुनर्स्थापित शॉर्टकट किसी सार्वजनिक डेस्कटॉप या साझा प्रारंभ मेनू स्थान को लक्षित करते हैं।
core-warning-restore-environment-refresh = परिवेश चर में किए गए परिवर्तन दिखने के लिए किसी नई प्रक्रिया, शेल, या साइन-इन सत्र की आवश्यकता हो सकती है।
# { $name } is a service's name. { $account } is the account it ran as on the PC it came from.
core-warning-restore-service-account = स्रोत PC पर सेवा { $name } { $account } के रूप में चलती थी। इसका पासवर्ड इंस्टॉलर में नहीं है, इसलिए सेवा LocalService के रूप में सेट अप की गई है। अगर यह काम नहीं करती, तो "सेवाएँ" में इसका खाता और पासवर्ड सेट करें।
# { $name } is a runtime.
core-warning-restore-prerequisite-pending-reboot = { $name } अभी पंजीकृत नहीं है; पुनः प्रारंभ के बाद इसे सक्रिय हो जाना चाहिए।
# { $name } is a runtime.
core-warning-restore-prerequisite-not-verified = { $name } ने सफलता की सूचना दी, लेकिन पंजीकृत नहीं हुआ; हो सकता है कि इंस्टॉल विफल रहा हो।
# { $error } is what went wrong.
core-warning-restore-driver-source-invalid = बंडल किया गया ड्राइवर पथ अमान्य है: { $error }
# { $error } is what went wrong.
core-warning-restore-driver-not-read = ड्राइवर की वर्तमान सेटिंग्स पढ़ी नहीं जा सकीं: { $error }
core-warning-restore-driver-exists = ड्राइवर सेवा मौजूद है; इसे फिर से कॉन्फ़िगर करने के लिए --force का उपयोग करें।
core-warning-restore-driver-image-exists = ड्राइवर इमेज मौजूद है; अधिलेखित करने के लिए --force का उपयोग करें।
core-warning-restore-driver-hash-bundled = बंडल की गई ड्राइवर इमेज अपने रिकॉर्ड किए गए हैश से मेल नहीं खाती, इसलिए इसे इंस्टॉल नहीं किया गया।
# { $error } is what went wrong.
core-warning-restore-driver-not-placed = ड्राइवर इमेज नहीं रखी जा सकी: { $error }
core-warning-restore-driver-hash-placed = रखे जाने के बाद ड्राइवर इमेज बदल गई, इसलिए इसे पंजीकृत नहीं किया गया।
# { $error } is what went wrong.
core-warning-restore-key-not-created = रजिस्ट्री कुंजी नहीं बनाई जा सकी: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-read = वह मान पढ़ा नहीं जा सका जिसे यह प्रतिस्थापित करेगा: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-written = रजिस्ट्री मान नहीं लिखा जा सका: { $error }
core-warning-restore-value-exists = रजिस्ट्री मान मौजूद है; अधिलेखित करने के लिए --force का उपयोग करें।
# { $error } is what went wrong.
core-warning-restore-environment-key-not-created = परिवेश की रजिस्ट्री कुंजी नहीं बनाई जा सकी: { $error }
core-warning-restore-variable-exists = परिवेश चर मौजूद है; अधिलेखित करने के लिए --force का उपयोग करें।
# { $error } is what went wrong.
core-warning-restore-variable-not-written = परिवेश चर नहीं लिखा जा सका: { $error }
core-warning-restore-value-kind-missing = रजिस्ट्री मान का प्रकार अनुपलब्ध है।
# { $error } is what went wrong.
core-warning-restore-service-not-read = सेवा की वर्तमान सेटिंग्स पढ़ी नहीं जा सकीं: { $error }
core-warning-restore-service-exists = सेवा मौजूद है; इसे फिर से कॉन्फ़िगर करने के लिए --force का उपयोग करें।
# { $error } is what went wrong.
core-warning-restore-task-not-queried = शेड्यूल किए गए कार्य की क्वेरी विफल रही: { $error }
core-warning-restore-task-exists = शेड्यूल किया गया कार्य मौजूद है; अधिलेखित करने के लिए --force का उपयोग करें।
# { $error } is what went wrong.
core-warning-restore-task-folder-not-looked-at = कार्य का फ़ोल्डर देखा नहीं जा सका: { $error }
# { $error } is what went wrong.
core-warning-restore-shortcut-not-put-back = शॉर्टकट नहीं बनाया जा सका, और जिस शॉर्टकट को इसने प्रतिस्थापित किया था, उसे वापस नहीं रखा जा सका: { $error }
# { $error } is what went wrong.
core-warning-restore-command-not-started = कमांड प्रारंभ नहीं किया जा सका: { $error }
# { $output } is what the command wrote, as it wrote it.
core-warning-restore-command-failed = कमांड विफल रहा: { $output }
# { $error } is what went wrong.
core-warning-restore-powershell-not-started = PowerShell प्रारंभ नहीं किया जा सका: { $error }
