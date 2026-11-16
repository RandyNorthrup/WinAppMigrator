# Words the core says about what an install may do: why an install location or something in a package is refused,
# the installer's list of what an app sets up beyond its files, the checks of a package and of an install, and whether
# there is room for either. Message names start with core-.
#
# Many messages here are a reason, the second half of a sentence that starts "... was left out:" or "that install
# location cannot be used:", so they start with a small letter and have no full stop.


## Why an install location cannot be used. Shown in red under the Install Location field.

core-review-location-state-root = यह वह जगह है जहाँ इंस्टॉल अपने रिकॉर्ड रखते हैं
core-review-location-has-files = इसमें पहले से फ़ाइलें हैं, और यहाँ कोई इंस्टॉल रिकॉर्ड में दर्ज नहीं है, इसलिए किसी खाली फ़ोल्डर का उपयोग करें
core-policy-location-not-full-path = यह एक पूरा पथ होना चाहिए
core-policy-location-parent-step = इसमें ".." नहीं होना चाहिए
core-policy-location-not-plain = इसे इस PC की किसी ड्राइव पर एक सामान्य पथ होना चाहिए
core-policy-location-whole-drive = यह पूरी ड्राइव नहीं हो सकता
core-policy-location-windows = यह Windows फ़ोल्डर नहीं हो सकता
core-policy-location-runs-by-itself = यह ऐसा फ़ोल्डर है जो Windows का है, या जिसकी सामग्री अपने आप चलती या लोड होती है
core-policy-location-shared = इसमें अन्य चीज़ें साथ-साथ रखी जाती हैं, इसलिए ऐप को इसके अंदर एक फ़ोल्डर चाहिए

## Whether an app can run on this PC at all, said before anything is installed.

# { $reason } is why, as the second half of a sentence, such as "App.exe is built for Arm64, and this PC is x64, so
# Windows would not start it".
core-review-cannot-run-here = यह ऐप इस PC पर नहीं चल सकता, इसलिए कुछ भी नहीं बदला गया: { $reason }
core-policy-architecture-x86 = 32-बिट x86
core-policy-architecture-x64 = x64
core-policy-architecture-arm64 = Arm64
core-policy-architecture-arm32 = 32-बिट Arm
core-policy-architecture-unknown = एक अज्ञात आर्किटेक्चर
# { $code } is the number Windows gives a kind of processor, such as 0x01c4.
core-policy-machine-type = मशीन प्रकार { $code }
# { $names } is a list of program names, such as "a.exe, b.exe, c.exe", and { $count } how many more there are.
core-policy-names-and-more = { $names }, और { $count } अन्य
core-policy-fit-programs-not-said = यह इंस्टॉलर नहीं बताता कि इसके प्रोग्रामों को क्या चाहिए, इसलिए यह जाँचा नहीं गया कि वे इस PC पर चल सकते हैं या नहीं।
core-policy-fit-architecture-not-told = यह पता नहीं चल सका कि यह PC किस आर्किटेक्चर का है, इसलिए यह जाँचा नहीं गया कि इस ऐप के प्रोग्राम इस पर चल सकते हैं या नहीं।
core-policy-fit-main-not-said = यह इंस्टॉलर नहीं बताता कि इसके मुख्य प्रोग्राम को क्या चाहिए, इसलिए यहाँ जो बताया गया है, वह इसमें मौजूद बाकी चीज़ों के आधार पर है।
# { $driver } is a driver's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-driver = इसका ड्राइवर { $driver } { $built_for } के लिए बना है, और यह PC { $here } है। ड्राइवर का Windows से बिल्कुल मेल खाना ज़रूरी है, इसलिए यह ऐप यहाँ नहीं चल सकता
# { $program } is a program's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-program = { $program } { $built_for } के लिए बना है, और यह PC { $here } है, इसलिए Windows इसे प्रारंभ नहीं करेगा
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-none-start = इसका कोई भी प्रोग्राम इस PC पर प्रारंभ नहीं हो सकता, जो { $here } है: { $programs }
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-some-other-architecture = इस ऐप के कुछ प्रोग्राम किसी अन्य आर्किटेक्चर के लिए बने हैं और इस PC पर प्रारंभ नहीं होंगे, जो { $here } है: { $programs }। ऐप का बाकी भाग प्रारंभ हो सकता है, इसलिए हो सकता है कि इससे कोई फ़र्क न पड़े।
# { $here } is a kind of processor, such as "Arm64". { $programs } is a list of program names.
core-policy-fit-emulated = यह PC { $here } है, इसलिए Windows इन्हें एमुलेटर के ज़रिए चलाएगा: { $programs }। अधिकतर प्रोग्राम इस तरह ठीक चलते हैं, लेकिन कुछ नहीं चलते।
# { $program } is a program's file name. { $needs } and { $runs } are versions of Windows, such as 10.0 and 6.1.
core-policy-fit-needs-newer = { $program } के अनुसार इसे Windows { $needs } या उससे नए की आवश्यकता है, और यह PC { $runs } चलाता है, इसलिए Windows इसे प्रारंभ नहीं करेगा
# { $programs } is a list of program names.
core-policy-fit-some-need-newer = इस ऐप के कुछ प्रोग्रामों के अनुसार उन्हें इससे नए Windows की आवश्यकता है, और वे यहाँ प्रारंभ नहीं होंगे: { $programs }।
# { $from_build } and { $this_build } are Windows build numbers, such as 22631, written as Windows writes them.
core-policy-fit-older-build = यह ऐप Windows बिल्ड { $from_build } पर था, और यह PC बिल्ड { $this_build } चलाता है। इसके प्रोग्रामों के अनुसार वे इस Windows पर प्रारंभ हो सकते हैं, इसलिए यह काम कर सकता है, लेकिन हो सकता है कि वहाँ यह जिन चीज़ों पर निर्भर था, वे यहाँ न हों।
core-policy-fit-build-not-told = यह पता नहीं चल सका कि यह Windows का कौन-सा बिल्ड है, इसलिए इसकी तुलना उस बिल्ड से नहीं की गई जिस पर यह ऐप था।

## Room on a drive for a build or an install.

# Each of these ends the sentence core-space-shortfall, after "free for".
core-space-for-unpacking = पैकेज को अनपैक करने
core-space-for-install-record = इंस्टॉल के रिकॉर्ड
core-space-for-app-files = ऐप की फ़ाइलों
core-space-for-what-is-replaced = प्रतिस्थापित होने वाली चीज़ों को रखने
core-space-for-build-work = बिल्ड में उपयोग होने वाली चीज़ों
core-space-for-installer = इंस्टॉलर
core-space-for-package = पैकेज
core-space-for-unpacking-and-installing = पैकेज को अनपैक करने और उसे इंस्टॉल करने
# What goes between two of those, as in "unpacking the package and the app's files". Keep the quotes: they keep the
# spaces.
core-space-purposes-joiner = { " और " }
# { $drive } is a drive, such as C:, or a folder. { $needed } and { $free } are sizes, such as 1.2 GB. { $purposes } is
# what the room is for, such as "unpacking the package and the app's files".
core-space-shortfall = { $drive } पर { $purposes } के लिए अधिकतम { $needed } रिक्त स्थान चाहिए, और उस पर { $free } है
# { $drive } is a drive, such as C:, or a folder. { $needed } is a size, such as 1.2 GB.
core-space-not-told = { $drive } ने यह नहीं बताया कि उस पर कितना स्थान है, इसलिए यह जाँचा नहीं गया कि उसे जो { $needed } चाहिए, वह उसमें समाएगा या नहीं
# One drive in a list of where room is needed. { $needed } and { $free } are sizes, and { $drive } a drive, such as C:.
core-space-summary = { $drive } पर अधिकतम { $needed } ({ $free } रिक्त)
# { $needed } is a size, and { $drive } a drive, such as C:.
core-space-summary-not-told = { $drive } पर अधिकतम { $needed } (कितना रिक्त है, यह पता नहीं चल सका)
# What goes between two sentences of core-space-shortfall inside core-review-no-room. Keep the quotes: they keep the
# space.
core-review-sentence-joiner = { "। " }
# { $shortfalls } is one or more sentences of core-space-shortfall.
core-review-no-room = इसके लिए पर्याप्त स्थान नहीं है, इसलिए कुछ भी नहीं बदला गया। { $shortfalls }। वहाँ कुछ स्थान खाली करें, या किसी अन्य ड्राइव पर इंस्टॉल स्थान चुनें।

## The installer's list of what an app sets up beyond its files: what each kind is called.

core-review-kind-service = सेवा
core-review-kind-driver = ड्राइवर
core-review-kind-scheduled-task = शेड्यूल किया गया कार्य
core-review-kind-autostart = स्वचालित रूप से प्रारंभ होता है
core-review-kind-shortcut = शॉर्टकट
core-review-kind-path = PATH
core-review-kind-environment-variable = परिवेश चर
core-review-kind-shell-extension = शेल एक्सटेंशन
core-review-kind-menu-command = मेनू कमांड
core-review-kind-link-type = लिंक प्रकार
core-review-kind-file-type = फ़ाइल प्रकार
core-review-kind-com = COM
core-review-kind-firewall-rule = फ़ायरवॉल नियम
core-review-kind-event-log-source = इवेंट लॉग स्रोत
core-review-kind-font = फ़ॉन्ट
core-review-kind-odbc-driver = ODBC ड्राइवर
core-review-kind-odbc-source = ODBC डेटा स्रोत
core-review-kind-compatibility-setting = संगतता सेटिंग
core-review-kind-default-apps = डिफ़ॉल्ट ऐप्स
core-review-kind-program-name = प्रोग्राम का नाम
core-review-kind-other-settings = अन्य सेटिंग्स

## The installer's list of what an app sets up: what each line says of it.

# { $file } is a program's file name. { $account } is the Windows account it runs as, such as LocalSystem. { $starts }
# says when it starts, and is not shown itself; for "other", { $start } is one of the core-review-start- words below.
core-review-service-detail =
    { $starts ->
        [automatic] { $file } को { $account } के रूप में चलाती है, अभी और हर बार यह PC प्रारंभ होने पर प्रारंभ होती है
        [delayed] { $file } को { $account } के रूप में चलाती है, अभी और हर बार यह PC प्रारंभ होने के कुछ देर बाद प्रारंभ होती है
        [manual] { $file } को { $account } के रूप में चलाती है, केवल तभी प्रारंभ होती है जब कोई चीज़ इसे प्रारंभ करने के लिए कहती है
        [disabled] { $file } को { $account } के रूप में चलाती है, अक्षम है
        [other] { $file } को { $account } के रूप में चलाती है, प्रारंभ होती है: { $start }
       *[unknown] { $file } को { $account } के रूप में चलाती है
    }
# { $account } is a Windows account, such as LocalService.
core-policy-account-custom = { $account } (कस्टम खाता पुनर्स्थापित नहीं किया गया)
# { $file } is a driver's file name.
core-review-driver-detail = { $file } को स्वयं Windows में लोड करता है
# { $file } is a driver's file name. { $start } is when Windows starts it: one of the core-review-start- words below.
core-review-driver-detail-starts = { $file } को स्वयं Windows में लोड करता है, प्रारंभ होता है: { $start }
# When a service or driver starts, ending a sentence such as "Loads disk.sys into Windows itself, starts …".
core-review-start-boot = Windows के साथ, बाकी सब से पहले
core-review-start-system = Windows के साथ, उसके प्रारंभ होते समय
core-review-start-automatic = हर बार यह PC प्रारंभ होने पर
core-review-start-delayed = हर बार यह PC प्रारंभ होने के कुछ देर बाद
core-review-start-manual = केवल तब, जब कोई चीज़ इसके लिए कहती है
core-review-start-disabled = कभी नहीं, क्योंकि यह अक्षम है
core-review-start-unknown = ऐसे तरीके से जिसे Windows नहीं बताता
# { $account } is the account a task runs as: an account's name, or core-review-task-runs-as-you, or
# core-review-task-runs-as-creator.
core-review-task-detail = इस रूप में चलता है: { $account }
# { $account } is the account a task runs as, as for core-review-task-detail.
core-review-task-detail-highest = उच्चतम संभव अधिकारों के साथ, इस रूप में चलता है: { $account }
core-review-task-runs-as-you = आप, जो भी साइन इन है
core-review-task-runs-as-creator = वह उपयोगकर्ता जिसने इसे बनाया
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-autostart-detail =
    { $scope ->
        [everyone] साइन इन करने वाले हर व्यक्ति के लिए चलता है
       *[you] आपके साइन इन करने पर चलता है
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shortcut-startup-detail =
    स्टार्टअप फ़ोल्डर में एक शॉर्टकट, { $scope ->
        [everyone] इस PC पर सभी के लिए
       *[you] आपके लिए
    }
# { $file } is the file name of what the shortcut opens. { $scope } is whom it is set up for: everyone on this PC,
# or you. It is not shown itself.
core-review-shortcut-detail =
    { $file } खोलता है, { $scope ->
        [everyone] इस PC पर सभी के लिए
       *[you] आपके लिए
    }
# { $name } is an environment variable's name, such as PATH.
core-review-path-title = { $name } में जोड़ता है
core-review-path-install-folder = इंस्टॉल फ़ोल्डर
# { $path } is a folder inside the install folder, such as bin\tools.
core-review-path-below-install-folder = इंस्टॉल फ़ोल्डर\{ $path }
# { $value } is the variable's value, as it will be set.
core-review-variable-detail = { $value } पर सेट किया जाता है
# { $count } is how many registry values.
core-review-other-settings-title =
    { $count ->
        [one] ऐप की अपनी कुंजियों के बाहर { $count } सेटिंग
       *[other] ऐप की अपनी कुंजियों के बाहर { $count } सेटिंग्स
    }
# { $keys } is a list of registry keys, such as HKCU\Software\Vendor.
core-review-other-settings-detail = { $keys } में
# { $keys } is a list of registry keys, and { $count } how many more there are.
core-review-other-settings-detail-more = { $keys }, और { $count } अन्य में
# { $way } says whether the rule allows or blocks, and incoming or outgoing connections; it is not shown itself.
# { $ports } is a list of port numbers, and { $port_count } how many. { $program } is the program's file name.
# { $networks } is a list of the kinds of network Windows names in English (domain, private, public), and
# { $network_count } how many.
# The kinds of network a firewall rule is for, as Windows Defender Firewall names its profiles in your language. They
# fill { $networks } in core-review-firewall-detail, joined into a list.
core-review-network-domain = डोमेन
core-review-network-private = निजी
core-review-network-public = सार्वजनिक
core-review-firewall-detail =
    { $program } के लिए, { $network_count ->
        [0] हर नेटवर्क पर
       *[other] { $networks } नेटवर्क पर
    }{ $port_count ->
        [0] { "" }
       *[other] { " " }पोर्ट { $ports } पर
    } { $way ->
        [allow-in] आने वाले कनेक्शन की अनुमति देता है
        [allow-out] बाहर जाने वाले कनेक्शन की अनुमति देता है
        [block-in] आने वाले कनेक्शन अवरोधित करता है
       *[block-out] बाहर जाने वाले कनेक्शन अवरोधित करता है
    }
# { $server } is the file name of what Explorer loads. { $scope } is whom it is set up for: everyone on this PC, or
# you. It is not shown itself.
core-review-shell-extension-detail =
    { $server } को फ़ाइल एक्सप्लोरर में लोड करता है, { $scope ->
        [everyone] इस PC पर सभी के लिए
       *[you] आपके लिए
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shell-extension-detail-own-file =
    अपने द्वारा इंस्टॉल की गई एक फ़ाइल को फ़ाइल एक्सप्लोरर में लोड करता है, { $scope ->
        [everyone] इस PC पर सभी के लिए
       *[you] आपके लिए
    }
# The name of a shell extension: { $handler } is its name in the registry, and { $class } the kind of file or folder it
# is for, such as * or Directory.
core-policy-shell-extension-name = { $class } पर { $handler }
# { $on } is the kind of file or folder whose menu it is, as the registry names it, such as * or
# Directory. { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-menu-command-detail =
    { $on } के मेनू में एक कमांड जोड़ता है, { $scope ->
        [everyone] इस PC पर सभी के लिए
       *[you] आपके लिए
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-default-verb-detail =
    बदलता है कि इनमें से किसी एक को खोलने पर क्या होता है, { $scope ->
        [everyone] इस PC पर सभी के लिए
       *[you] आपके लिए
    }
core-review-client-entry-detail = इसके लिए प्रोग्राम के रूप में चुना जा सकता है। कुछ भी इस पर सेट नहीं किया जाता
core-review-com-permissions-title = COM अनुमतियाँ
core-review-com-permissions-detail = बताता है कि इसकी COM क्लास कौन प्रारंभ कर सकता है, या वे किस रूप में चलती हैं
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-link-type-detail =
    इस प्रकार के लिंक ऐप से खुलते हैं, { $scope ->
        [everyone] इस PC पर सभी के लिए
       *[you] आपके लिए
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-file-type-detail =
    इस प्रकार की फ़ाइलें ऐप से खुलती हैं, { $scope ->
        [everyone] इस PC पर सभी के लिए
       *[you] आपके लिए
    }
# { $log } is the name of a Windows event log, such as Application.
core-review-event-log-source-detail = अपनी फ़ाइलों के संदेशों के साथ, { $log } लॉग में लिख सकता है
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-font-detail =
    आपके साइन इन करने पर इंस्टॉल फ़ोल्डर से लोड होता है, { $scope ->
        [everyone] इस PC पर सभी के लिए
       *[you] आपके लिए
    }
core-review-odbc-driver-detail = उन प्रोग्रामों में लोड होता है जो किसी डेटाबेस तक पहुँचने के लिए ODBC का उपयोग करते हैं
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-odbc-source-detail =
    एक डेटाबेस कनेक्शन, { $scope ->
        [everyone] इस PC पर सभी के लिए
       *[you] आपके लिए
    }
core-review-default-apps-detail = डिफ़ॉल्ट ऐप्स में चुना जा सकता है। कुछ भी इस पर सेट नहीं किया जाता
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-program-name-detail =
    "चलाएँ" और अन्य प्रोग्राम इसे इस नाम से ढूँढ सकते हैं, { $scope ->
        [everyone] इस PC पर सभी के लिए
       *[you] आपके लिए
    }
# { $server } is the file name of what serves the classes.
core-review-com-title = { $server } में COM
core-review-com-title-own-file = अपने द्वारा इंस्टॉल की गई फ़ाइल में COM
# { $count } is how many registry values.
core-review-com-detail =
    { $count ->
        [one] उन क्लास की { $count } सेटिंग जिन्हें अन्य प्रोग्राम प्रारंभ कर सकते हैं
       *[other] उन क्लास की { $count } सेटिंग्स जिन्हें अन्य प्रोग्राम प्रारंभ कर सकते हैं
    }
core-review-com-support-title = COM टाइप लाइब्रेरी, इंटरफ़ेस, और AppID
# { $libraries }, { $interfaces }, and { $app_ids } are how many registry values there are for its type libraries, its
# interfaces, and its AppIDs.
core-review-com-support-detail = इसकी टाइप लाइब्रेरी ({ $libraries }), इंटरफ़ेस ({ $interfaces }) और AppID ({ $app_ids }) की सेटिंग्स, जो इसकी क्लास के साथ जाती हैं
core-review-compatibility-title = इसके प्रोग्राम कैसे प्रारंभ होते हैं
# { $count } is how many registry values.
core-review-compatibility-detail =
    { $count ->
        [one] इसके द्वारा इंस्टॉल किए जाने वाले प्रोग्रामों के लिए { $count } संगतता सेटिंग
       *[other] इसके द्वारा इंस्टॉल किए जाने वाले प्रोग्रामों के लिए { $count } संगतता सेटिंग्स
    }

## The installer's notes on whose folders and settings these are, and what is not coming.

# { $account } is the name of a Windows account, such as alice.
core-review-account-signed-in = उपयोगकर्ता डेटा और प्रति-उपयोगकर्ता सेटिंग्स { $account } के पास जाती हैं, जो यहाँ साइन इन है, न कि उस व्यवस्थापक के पास जिसके रूप में यह चल रहा है।
# { $account } is the name of a Windows account, such as alice.
core-review-account-no-desktop = इस सत्र में कोई साइन इन नहीं है, इसलिए उपयोगकर्ता डेटा और प्रति-उपयोगकर्ता सेटिंग्स { $account } के पास जाती हैं, जो वह खाता है जिसके रूप में यह चल रहा है।
# { $reason } is what Windows said, in its own words.
core-review-account-not-told = यह पता नहीं चल सका कि कौन साइन इन है ({ $reason }), इसलिए प्रति-उपयोगकर्ता सेटिंग्स छोड़ दी गईं, और उपयोगकर्ता डेटा इंस्टॉल स्थान के अंतर्गत जाता है।
# { $reason } is what Windows said, in its own words.
core-review-signed-in-unknown = यह पता नहीं चल सका कि कौन साइन इन है: { $reason }
# Headings of what is left out, shared by the app's list and the installer's notes.
core-review-left-out-credentials = इस इंस्टॉलर में नहीं हैं, क्योंकि ये क्रेडेंशियल जैसे दिखते हैं
core-review-left-out-settings = ऐप के लिए किसी खाते द्वारा रखी गई सेटिंग्स और इतिहास, जो केवल उपयोगकर्ता डेटा विकल्प के साथ शामिल होते हैं
core-review-left-out-certificates = ऐप ऐसे प्रमाणपत्रों पर निर्भर लगता है जिन पर इसके PC पर भरोसा किया गया था, और वे इंस्टॉल नहीं किए जाते
core-review-left-out-fonts = ये फ़ॉन्ट ऐप के नाम से हैं, और कुछ भी नहीं बताता कि ये इसी के हैं, इसलिए इन्हें इंस्टॉल नहीं किया जाता
# { $names } is a list of names, such as the names of saved passwords.
core-review-note-credentials = इस इंस्टॉलर में नहीं हैं, क्योंकि ये क्रेडेंशियल जैसे दिखते हैं: { $names }। इस PC पर इनमें से जो कुछ है, वह जैसा है वैसा ही रहता है, और आपको इन्हें फिर से दर्ज करना पड़ सकता है।
# { $names } is a list of names of license data.
core-review-note-licenses = इस इंस्टॉलर में नहीं हैं, क्योंकि इसे बनाते समय "लाइसेंस और सक्रियण डेटा" विकल्प बंद था: { $names }।
# { $names } is a list of places an account keeps settings, such as folders.
core-review-note-settings = इस इंस्टॉलर में नहीं हैं, क्योंकि इसे बनाते समय उपयोगकर्ता डेटा विकल्प बंद था: ऐप के लिए किसी खाते द्वारा { $names } में रखी गई सेटिंग्स और इतिहास। ऐप यहाँ वैसे ही प्रारंभ होता है जैसे किसी ऐसे व्यक्ति के लिए जिसने इसे पहले कभी उपयोग नहीं किया।
# { $names } is a list of certificates, each as core-review-certificate.
core-review-note-certificates = ऐप ऐसे प्रमाणपत्रों पर निर्भर लगता है जिन पर इसके PC पर भरोसा किया गया था, और वे इंस्टॉल नहीं किए जाते: { $names }। यहाँ उन पर भरोसा करना है या नहीं, यह आपको तय करना है।
# { $names } is a list of font names.
core-review-note-fonts = ये फ़ॉन्ट ऐप के नाम से हैं, और कुछ भी नहीं बताता कि ये इसी के हैं, इसलिए इन्हें इंस्टॉल नहीं किया जाता: { $names }।
# { $names } is a list of names.
core-review-note-left-out-by-choice = इस इंस्टॉलर को बनाने वाले ने इन्हें छोड़ा: { $names }।
# { $names } is a list of names.
core-review-note-added-by-choice = इस इंस्टॉलर को बनाने वाले ने इन्हें जोड़ा, क्योंकि उन्होंने इन्हें ऐप का भाग माना: { $names }।
# { $name } is a certificate's subject, such as CN=Contoso, and { $store } the certificate store it was in, such as
# Root.
core-review-certificate = { $store } में { $name }
# { $names } is a list of names, and { $count } how many more there are.
core-review-names-and-more = { $names }, और { $count } अन्य

## Why something in a package is left out. Each is the second half of a sentence of core-warning-review-left-out.

core-review-no-account = यह किसी एक खाते की सेटिंग्स का है, और यह पता नहीं चल सका कि कौन साइन इन है
core-review-name-not-accepted = Windows इसका नाम स्वीकार नहीं करेगा
# { $error } is what Windows said, in its own words.
core-review-service-unreadable = इसकी वर्तमान सेटिंग्स पढ़ी नहीं जा सकीं: { $error }
core-review-service-dependency = यह अमान्य नाम वाली किसी सेवा पर निर्भर है
# { $path } is the program the service runs.
core-review-service-runs-other = यह { $path } चलाती है, जिसे यह पैकेज इंस्टॉल नहीं करता
core-review-service-settings = इसकी सेटिंग्स किसी ऐसी चीज़ का नाम लेती हैं जिसे यह पैकेज इंस्टॉल नहीं करता, या किसी ऐसी कुंजी का जो इसकी अपनी नहीं है
core-review-driver-no-hash = इसकी जाँच के लिए इसका कोई हैश नहीं है
core-review-license-for-the-machine = पूरी मशीन के लिए संग्रहीत लाइसेंस की केवल रिपोर्ट की जाती है
core-review-license-starts-something = किसी लाइसेंस के पास कुछ भी प्रारंभ करने का कोई कारण नहीं है
core-review-task-not-captured = इसकी परिभाषा कैप्चर नहीं की गई
# { $error } is what was wrong with the task's XML, in English.
core-review-task-unreadable = इसकी परिभाषा पढ़ी नहीं जा सकी: { $error }
core-review-task-no-account = यह इसे इंस्टॉल करने वाले व्यक्ति के रूप में चलता है, और यह पता नहीं चल सका कि यहाँ कौन साइन इन है
# { $path } is what the shortcut opens.
core-review-shortcut-opens-other = यह { $path } खोलता है, जिसे यह पैकेज इंस्टॉल नहीं करता
# { $service } is the service's name, and { $path } the program it runs now.
core-policy-service-exists = { $service } इस PC पर पहले से मौजूद है और { $path } चलाती है, जो यहाँ किसी पिछले इंस्टॉल से नहीं है। अगर यह इस ऐप की पुरानी प्रति है, तो पहले उसे अनइंस्टॉल करें
core-policy-key-control-characters = इसकी कुंजी के नाम में नियंत्रण वर्ण हैं
core-policy-value-control-characters = इसके मान के नाम में नियंत्रण वर्ण हैं
core-policy-registry-link = यह एक रजिस्ट्री लिंक है
# { $reason } is why the key it stands for is left out, such as "no app install should change this key".
core-policy-virtual-store = यह PC की किसी कुंजी का प्रतिनिधित्व करता है, और { $reason }
core-policy-registry-part = किसी भी ऐप इंस्टॉल को रजिस्ट्री के इस भाग में नहीं लिखना चाहिए
core-policy-key-refused = किसी भी ऐप इंस्टॉल को यह कुंजी नहीं बदलनी चाहिए
core-policy-net-debugger = यह हर .NET प्रोग्राम के लिए एक डीबगर निर्धारित करेगा
core-policy-net-install-root = यह बदल देगा कि हर .NET प्रोग्राम .NET को कहाँ से लोड करता है
core-policy-class-redirects = यह बदल देगा कि किसी प्रकार का प्रोग्राम या फ़ोल्डर कैसे खोला जाता है
core-policy-app-path-name = यह किसी प्रोग्राम को फ़ाइल नाम के अलावा किसी और नाम से बताता है
core-policy-app-path-not-installed = यह ऐसी फ़ाइल के लिए प्रोग्राम का नाम पंजीकृत करेगा जिसे यह पैकेज इंस्टॉल नहीं करता
core-policy-shared-default-verb = यह बदल देगा कि किसी प्रकार की हर फ़ाइल या फ़ोल्डर को खोलने पर क्या होता है
core-policy-command-runs-other = यह ऐसा कमांड जोड़ता है जो कुछ ऐसा चलाता है जिसे यह पैकेज इंस्टॉल नहीं करता
core-policy-autostart-not-command = यह स्वचालित रूप से प्रारंभ होता है, और इसका मान कोई कमांड नहीं है
core-policy-autostart-runs-other = यह स्वचालित रूप से प्रारंभ होता है, और कुछ ऐसा चलाता है जिसे यह पैकेज इंस्टॉल नहीं करता
core-policy-environment-own-rules = परिवेश चर उनके अपने नियमों से पुनर्स्थापित किए जाते हैं
core-policy-service-not-restored = यह ऐसी सेवा का है जिसे यह पैकेज पुनर्स्थापित नहीं करता
core-policy-service-own-rules = किसी सेवा की अपनी सेटिंग्स केवल उसके नियमों के माध्यम से पुनर्स्थापित की जाती हैं
core-policy-service-dll = इसका ServiceDll ऐसी फ़ाइल नहीं है जिसे यह पैकेज इंस्टॉल करता है
core-policy-event-log-source-only = केवल इवेंट लॉग स्रोत के अपने मान ही पुनर्स्थापित किए जा सकते हैं
core-policy-security-log = कोई भी ऐप सुरक्षा लॉग में नहीं लिखता
core-policy-event-log-messages = इसके संदेश ऐसी फ़ाइल से आएँगे जिसे यह पैकेज इंस्टॉल नहीं करता
core-policy-type-library = इसकी टाइप लाइब्रेरी ऐसी फ़ाइल नहीं है जिसे यह पैकेज इंस्टॉल करता है
# { $name } is a program's file name, such as app.exe.
core-policy-application-opened = यह बदल देगा कि { $name } को कैसे खोला जाता है, जिसे यह पैकेज इंस्टॉल नहीं करता
core-policy-event-log-no-file = इवेंट लॉग स्रोत को अपने संदेश किसी फ़ाइल से लेने होते हैं
core-policy-com-runs-other = यह कुछ ऐसा चलाता है जो ऐसी फ़ाइल नहीं है जिसे यह पैकेज इंस्टॉल करता है
core-policy-com-redirect = यह इस क्लास को किसी ऐसी चीज़ में बदल देगा जिसे यह पैकेज इंस्टॉल नहीं करता
core-policy-com-server = यह कुछ ऐसा चलाता है जो न तो इस पैकेज द्वारा इंस्टॉल की जाने वाली फ़ाइल है, न ही Windows का भाग
core-policy-interface = यह तय करेगा कि किसी इंटरफ़ेस को कॉल कैसे किए जाते हैं, किसी ऐसी चीज़ से जिसे यह पैकेज इंस्टॉल नहीं करता
# { $name } is a program's file name, such as app.exe.
core-policy-app-id-started = यह बदल देगा कि { $name } को कैसे प्रारंभ किया जाता है, जिसे यह पैकेज इंस्टॉल नहीं करता
core-policy-app-id-surrogate = इसे ऐसा प्रोग्राम होस्ट करेगा जिसे यह पैकेज इंस्टॉल नहीं करता
core-policy-app-id-service = यह ऐसी सेवा में चलेगा जिसे यह पैकेज पुनर्स्थापित नहीं करता
core-policy-app-id-run-as = यह अपने अलग खाते के रूप में चलेगा
core-policy-app-id-link = यह किसी AppID का नाम नहीं लेता
core-policy-app-id-remote = यह इस PC के अलावा कहीं और चलेगा
core-policy-odbc-own-driver = वह ODBC की अपनी सेटिंग है, कोई ड्राइवर नहीं
core-policy-odbc-driver-not-installed = इसका ड्राइवर ऐसी फ़ाइल नहीं है जिसे यह पैकेज इंस्टॉल करता है
core-policy-loads-other-file = यह ऐसी फ़ाइल लोड करेगा जिसे यह पैकेज इंस्टॉल नहीं करता
core-policy-odbc-own-source = वह ODBC की अपनी सेटिंग है, कोई डेटा स्रोत नहीं
core-policy-odbc-source-missing = यह ऐसे डेटा स्रोत को सूचीबद्ध करता है जो इस पैकेज में नहीं है
core-policy-odbc-source-driver = इसका ड्राइवर न तो इस पैकेज द्वारा इंस्टॉल की जाने वाली फ़ाइल है, न ही Windows का भाग
core-policy-shell-extension = यह फ़ाइल एक्सप्लोरर में ऐसा शेल एक्सटेंशन लोड करेगा जो इस पैकेज द्वारा इंस्टॉल की जाने वाली फ़ाइल नहीं है
core-policy-not-text = इसमें जो है, वह वैसा टेक्स्ट नहीं है जैसा इसे पढ़ा जाएगा
core-policy-network-place = यह नेटवर्क पर किसी स्थान का नाम लेता है
core-policy-font = यह ऐसा फ़ॉन्ट पंजीकृत करेगा जिसे यह पैकेज इंस्टॉल नहीं करता, या Windows फ़ोल्डर में मौजूद कोई फ़ॉन्ट
core-policy-compatibility = यह बदल देगा कि ऐसा प्रोग्राम कैसे चलता है जिसे यह पैकेज इंस्टॉल नहीं करता
core-policy-capabilities = यह ऐसे ऐप को सूचीबद्ध करता है जिसका विवरण इस पैकेज में नहीं है
core-policy-default-program = यह बदल देगा कि चीज़ें डिफ़ॉल्ट रूप से किस प्रोग्राम से खुलती हैं
core-policy-url-protocol = यह कहता है कि यह एक लिंक प्रकार है, और ऐसी किसी चीज़ से नहीं खुलता जिसे यह पैकेज इंस्टॉल करता है
core-policy-command-handler = यह ऐसा कमांड जोड़ता है जिसे ऐसी चीज़ संभालती है जिसे यह पैकेज इंस्टॉल नहीं करता
core-policy-installer-command = यह वह चलाएगा जिसे केवल Windows Installer पढ़ सकता है
core-policy-creates-class = यह ऐसी क्लास बनाएगा जिसे यह पैकेज इंस्टॉल नहीं करता
core-policy-opens-links = यह लिंक ऐसी चीज़ से खोलता है जिसे यह पैकेज इंस्टॉल नहीं करता
core-policy-opens-files = यह फ़ाइलें ऐसी चीज़ से खोलता है जिसे यह पैकेज इंस्टॉल नहीं करता
core-policy-firewall-name = इसका नाम ऐसा नहीं है जिसे यह आगे दे सके
core-policy-firewall-description = इसका विवरण ऐसा नहीं है जिसे यह आगे दे सके
core-policy-firewall-every-program = यह हर प्रोग्राम पर लागू होता है, और एक नियम केवल एक प्रोग्राम के लिए हो सकता है
# { $path } is the program the rule is for.
core-policy-firewall-program = यह { $path } के लिए है, जिसे यह पैकेज इंस्टॉल नहीं करता
core-policy-firewall-direction = यह ऐसी दिशा नहीं बताता जिसे यह सेट कर सके
core-policy-firewall-action = यह अन्य नियमों को ओवरराइड करेगा, जो किसी भी ऐप के नियम को नहीं करना चाहिए
# { $setting } is a part of a firewall rule, as the scan of its PC described it.
core-policy-firewall-not-carried = इसमें { $setting } लिखा है, जिसे वैसा ही सेट नहीं किया जा सका, या जो अन्य नियमों के प्रभाव को कमज़ोर कर देगा
core-policy-firewall-service = यह ऐसी सेवा के लिए है जिसे यह पैकेज पुनर्स्थापित नहीं करता
core-policy-firewall-parts = इसका एक भाग ऐसा है जो फ़ायरवॉल नियम में नहीं हो सकता
core-policy-folder-already-there = वह फ़ोल्डर इंस्टॉल स्थान के बाहर पहले से मौजूद था, और कुछ भी नहीं बताता कि यह इस ऐप का है, इसलिए उसमें कोई प्रोग्राम नहीं जाता
core-policy-variable-name = इसका नाम मान्य चर नाम नहीं है
core-policy-variable-relied-on = Windows और अन्य प्रोग्राम इस चर पर निर्भर हैं
core-policy-task-name = इसका नाम ऐसा नहीं है जिसे Windows स्वीकार करता है
core-policy-task-folder = इसका फ़ोल्डर ऐसा नहीं है जिसे Windows स्वीकार करता है
core-policy-task-windows = यह Windows के अपने कार्यों में से किसी एक को प्रतिस्थापित करेगा
core-policy-task-other-action = यह प्रोग्राम चलाने के अलावा कुछ और करता है
core-policy-task-no-program = यह चलाने के लिए किसी प्रोग्राम का नाम नहीं देता
# { $path } is the program the task runs.
core-policy-task-runs-other = यह कुछ ऐसा चलाता है जिसे यह पैकेज इंस्टॉल नहीं करता: { $path }
core-policy-task-exists-unreadable = इस नाम का एक कार्य पहले से मौजूद है और उसे पढ़ा नहीं जा सका
core-policy-task-exists-other = इस नाम का एक कार्य पहले से मौजूद है और वह यह ऐप नहीं चलाता
# { $path } is the program the task already on this PC runs.
core-policy-task-exists-runs = इस नाम का एक कार्य पहले से मौजूद है और { $path } चलाता है, जो यहाँ किसी पिछले इंस्टॉल से नहीं है
core-policy-shortcut-not-lnk = यह .lnk शॉर्टकट नहीं है
core-policy-shortcut-place = यह प्रारंभ मेनू में या डेस्कटॉप पर नहीं है
core-policy-short-name = इसके पथ में ऐसे नाम का उपयोग किया गया है जिसकी जाँच नहीं की जा सकती
core-policy-shortcut-no-folder = इस PC पर इसके लिए ऐसा कोई फ़ोल्डर नहीं है
core-policy-data-startup = यह किसी स्टार्टअप फ़ोल्डर में जाएगा, और हर साइन इन पर चलेगा
core-policy-data-powershell = यह वहाँ जाएगा जहाँ PowerShell प्रारंभ होते समय जो पाता है, उसे चलाता है
core-policy-data-windows = यह ऐसे फ़ोल्डर में जाएगा जो Windows का है
core-policy-license-outside = यह इंस्टॉल स्थान और हर ज्ञात डेटा फ़ोल्डर के बाहर है
core-rewrite-cdata = इसमें एक CDATA अनुभाग है
core-rewrite-too-much-to-look-through = इस पैकेज की सेटिंग्स में इतना अधिक है कि यह जाँचा नहीं जा सकता कि वे किन चीज़ों का नाम लेती हैं

## Warnings a review gives, which the installer shows with what it sets up.

# { $kind } says what was left out, and is not shown itself. { $item } is its path or name, or a registry value such
# as HKLM\Software\App\Name. { $reason } is why, as the second half of the sentence, such as "it cannot be a whole
# drive".
core-warning-review-left-out =
    { $kind ->
        [file] फ़ाइल { $item } छोड़ी गई: { $reason }।
        [folder] फ़ोल्डर { $item } छोड़ा गया: { $reason }।
        [service] सेवा { $item } छोड़ी गई: { $reason }।
        [driver] ड्राइवर { $item } छोड़ा गया: { $reason }।
        [firewall-rule] फ़ायरवॉल नियम { $item } छोड़ा गया: { $reason }।
        [license-data] { $item } पर मौजूद लाइसेंस डेटा छोड़ा गया: { $reason }।
        [license-value] लाइसेंस मान { $item } छोड़ा गया: { $reason }।
        [scheduled-task] शेड्यूल किया गया कार्य { $item } छोड़ा गया: { $reason }।
        [shortcut] शॉर्टकट { $item } छोड़ा गया: { $reason }।
        [environment-variable] परिवेश चर { $item } छोड़ा गया: { $reason }।
       *[value] { $item } छोड़ा गया: { $reason }।
    }
core-warning-review-package-path-missing = कॉपी की जा सकने वाली एक फ़ाइल का कोई पैकेज-सापेक्ष पथ नहीं है।
# { $count } is the most empty folders a package makes.
core-warning-review-too-many-empty-folders = यह पैकेज { $count } से अधिक खाली फ़ोल्डर सूचीबद्ध करता है, और केवल इतने ही बनाए जाते हैं।
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Documents folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-documents-elsewhere =
    { $count ->
        [one] दस्तावेज़ फ़ोल्डर की { $count } फ़ाइल इसके बजाय { $target } के अंतर्गत, _external नाम के फ़ोल्डर में जाती है, क्योंकि इस PC का दस्तावेज़ फ़ोल्डर { $folder } पर है, और केवल आपकी प्रोफ़ाइल के अंदर के फ़ोल्डर में ही लिखा जाता है। इसका उपयोग करने के लिए, इसे स्वयं अपने दस्तावेज़ फ़ोल्डर में ले जाएँ।
       *[other] दस्तावेज़ फ़ोल्डर की { $count } फ़ाइलें इसके बजाय { $target } के अंतर्गत, _external नाम के फ़ोल्डर में जाती हैं, क्योंकि इस PC का दस्तावेज़ फ़ोल्डर { $folder } पर है, और केवल आपकी प्रोफ़ाइल के अंदर के फ़ोल्डर में ही लिखा जाता है। इनका उपयोग करने के लिए, इन्हें स्वयं अपने दस्तावेज़ फ़ोल्डर में ले जाएँ।
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-documents-unknown =
    { $count ->
        [one] दस्तावेज़ फ़ोल्डर की { $count } फ़ाइल इसके बजाय { $target } के अंतर्गत, _external नाम के फ़ोल्डर में जाती है, क्योंकि यह पता नहीं चल सका कि इस PC का दस्तावेज़ फ़ोल्डर कहाँ है। इसका उपयोग करने के लिए, इसे स्वयं अपने दस्तावेज़ फ़ोल्डर में ले जाएँ।
       *[other] दस्तावेज़ फ़ोल्डर की { $count } फ़ाइलें इसके बजाय { $target } के अंतर्गत, _external नाम के फ़ोल्डर में जाती हैं, क्योंकि यह पता नहीं चल सका कि इस PC का दस्तावेज़ फ़ोल्डर कहाँ है। इनका उपयोग करने के लिए, इन्हें स्वयं अपने दस्तावेज़ फ़ोल्डर में ले जाएँ।
    }
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Saved Games folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-elsewhere =
    { $count ->
        [one] सहेजे गए गेम फ़ोल्डर की { $count } फ़ाइल इसके बजाय { $target } के अंतर्गत, _external नाम के फ़ोल्डर में जाती है, क्योंकि इस PC का सहेजे गए गेम फ़ोल्डर { $folder } पर है, और केवल आपकी प्रोफ़ाइल के अंदर के फ़ोल्डर में ही लिखा जाता है। इसका उपयोग करने के लिए, इसे स्वयं अपने सहेजे गए गेम फ़ोल्डर में ले जाएँ।
       *[other] सहेजे गए गेम फ़ोल्डर की { $count } फ़ाइलें इसके बजाय { $target } के अंतर्गत, _external नाम के फ़ोल्डर में जाती हैं, क्योंकि इस PC का सहेजे गए गेम फ़ोल्डर { $folder } पर है, और केवल आपकी प्रोफ़ाइल के अंदर के फ़ोल्डर में ही लिखा जाता है। इनका उपयोग करने के लिए, इन्हें स्वयं अपने सहेजे गए गेम फ़ोल्डर में ले जाएँ।
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-unknown =
    { $count ->
        [one] सहेजे गए गेम फ़ोल्डर की { $count } फ़ाइल इसके बजाय { $target } के अंतर्गत, _external नाम के फ़ोल्डर में जाती है, क्योंकि यह पता नहीं चल सका कि इस PC का सहेजे गए गेम फ़ोल्डर कहाँ है। इसका उपयोग करने के लिए, इसे स्वयं अपने सहेजे गए गेम फ़ोल्डर में ले जाएँ।
       *[other] सहेजे गए गेम फ़ोल्डर की { $count } फ़ाइलें इसके बजाय { $target } के अंतर्गत, _external नाम के फ़ोल्डर में जाती हैं, क्योंकि यह पता नहीं चल सका कि इस PC का सहेजे गए गेम फ़ोल्डर कहाँ है। इनका उपयोग करने के लिए, इन्हें स्वयं अपने सहेजे गए गेम फ़ोल्डर में ले जाएँ।
    }

## Errors when a folder that only administrators may change cannot be made so, or checked.

# { $path } is a folder.
core-privilege-not-a-folder = { $path } एक लिंक या फ़ाइल है, फ़ोल्डर नहीं
# { $path } is a folder. { $error } is what Windows said, in its own words.
core-privilege-not-made-here = { $path } को WinAppMigrator ने नहीं बनाया था, और अन्य लोग इसकी सामग्री बदल सकते हैं। किसी व्यवस्थापक को इसकी जाँच करके इसे हटाना चाहिए: { $error }
# { $path } is a folder.
core-privilege-others-put-something-in = { $path } में कुछ ऐसा है जिसे WinAppMigrator ने नहीं बनाया था, और अन्य लोग उसे बदल सकते हैं। किसी व्यवस्थापक को इसकी जाँच करके इसे हटाना चाहिए
# { $path } is a folder.
core-privilege-became-a-link-in-use = उपयोग किए जाते समय { $path } एक लिंक बन गया
# { $path } is a folder.
core-privilege-became-a-link-looked-at = जाँचे जाते समय { $path } एक लिंक बन गया
# { $path } is a file. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-users-cannot-start = उपयोगकर्ताओं को { $path } प्रारंभ करने की अनुमति नहीं दी जा सकी: { $said }
core-privilege-elevation-unknown = यह पता नहीं चल सका कि यह प्रक्रिया व्यवस्थापक अधिकारों के साथ चल रही है या नहीं
# { $path } is a folder. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-cannot-restrict = { $path } को केवल व्यवस्थापकों तक सीमित नहीं किया जा सका: { $said }
# { $path } is a folder. { $said } is the folder's permissions as Windows wrote them, and what else it printed.
core-privilege-not-restricted = { $path } को केवल व्यवस्थापकों तक सीमित नहीं किया जा सका, और हो सकता है कि इसकी ड्राइव अनुमतियाँ न रखती हो: { $said }
# { $path } is a folder.
core-privilege-written-to-first = सुरक्षित किए जाने से पहले ही किसी अन्य प्रक्रिया ने { $path } में लिख दिया
core-privilege-no-program-data = Windows यह नहीं बताता कि ProgramData कहाँ है

## Checks of a package's files, and of what an install put in place. Each is said of one file, value, service, or the
## like, which the report names beside it.

# { $path } is the installer's file.
core-verify-no-package = { $path } में कोई पैकेज नहीं है
# { $path } is a folder.
core-verify-not-unpacked = इस इंस्टॉलर का पैकेज { $path } में अनपैक नहीं किया गया
core-verify-schema-supported = पैकेज मेनिफ़ेस्ट स्कीमा समर्थित है।
core-verify-packaged-file-missing = पैकेज की गई फ़ाइल अनुपलब्ध है।
core-verify-no-package-path = कॉपी की जा सकने वाली फ़ाइल का कोई पैकेज-सापेक्ष पथ नहीं है।
core-verify-folder-there = खाली फ़ोल्डर मौजूद है।
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-unreadable = इसकी जाँच के लिए इसे पढ़ा नहीं जा सका: { $error }
core-verify-restored-file-missing = पुनर्स्थापित फ़ाइल अनुपलब्ध है।
# { $expected } and { $actual } are sizes in bytes.
core-verify-size-mismatch = फ़ाइल का आकार मेल नहीं खाता। अपेक्षित { $expected } बाइट, मिले { $actual } बाइट।
core-verify-no-hash = मेनिफ़ेस्ट में इस फ़ाइल का कोई हैश दर्ज नहीं है, इसलिए इसकी जाँच नहीं की जा सकती।
core-verify-hash-mismatch = SHA-256 हैश मेल नहीं खाता।
core-verify-file-matches = फ़ाइल का आकार और हैश मेनिफ़ेस्ट से मेल खाते हैं।
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-payload-failed = लाइसेंस पेलोड का सत्यापन विफल रहा: { $error }
core-verify-license-payload-missing = पैकेज किया गया लाइसेंस पेलोड अनुपलब्ध है।
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-failed = लाइसेंस स्टोर का सत्यापन विफल रहा: { $error }
core-verify-license-store-file-missing = पैकेज की गई लाइसेंस स्टोर फ़ाइल अनुपलब्ध है।
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-path-invalid = पैकेज किया गया लाइसेंस स्टोर पथ अमान्य है: { $error }
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-prerequisite-failed = पूर्वापेक्षा पेलोड का सत्यापन विफल रहा: { $error }
core-verify-prerequisite-missing = बंडल किया गया पूर्वापेक्षा पेलोड अनुपलब्ध है।
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-failed = ड्राइवर इमेज का सत्यापन विफल रहा: { $error }
core-verify-bundled-driver-missing = बंडल की गई ड्राइवर इमेज अनुपलब्ध है।
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-path-invalid = बंडल किया गया ड्राइवर पथ अमान्य है: { $error }
core-verify-prerequisite-present = रनटाइम पूर्वापेक्षा लक्ष्य PC पर मौजूद है।
core-verify-prerequisite-not-detected = रनटाइम पूर्वापेक्षा का पता नहीं चला; इसके लिए पुनः प्रारंभ या मैन्युअल रूप से पूरा करने की आवश्यकता हो सकती है (आधिकारिक परिणाम के लिए पुनर्स्थापना लॉग देखें)।
core-verify-prerequisite-not-probed = पूर्वापेक्षा की उपस्थिति रजिस्ट्री से जाँची नहीं जा सकती; इंस्टॉल सर्वोत्तम प्रयास के आधार पर किया गया।
core-verify-license-file-missing = पुनर्स्थापित लाइसेंस फ़ाइल अनुपलब्ध है।
core-verify-license-store-restored-missing = पुनर्स्थापित लाइसेंस स्टोर फ़ाइल अनुपलब्ध है।
core-verify-registry-key-there = रजिस्ट्री कुंजी मौजूद है।
core-verify-registry-value-matches = रजिस्ट्री मान संग्रहीत डेटा से मेल खाता है।
core-verify-registry-value-differs = रजिस्ट्री मान संग्रहीत डेटा से मेल नहीं खाता।
core-verify-firewall-rule-there = Windows फ़ायरवॉल में यह नियम है।
core-verify-variable-matches = परिवेश चर का मान संग्रहीत डेटा से मेल खाता है।
core-verify-variable-differs = परिवेश चर का मान संग्रहीत डेटा से मेल नहीं खाता।
# { $error } is what Windows said, in its own words.
core-verify-variable-unreadable = परिवेश चर अनुपलब्ध है या पढ़ा नहीं जा सकता: { $error }
core-verify-service-runs-other = सेवा मौजूद है, लेकिन पुनर्स्थापित बाइनरी को उसके तर्कों के साथ नहीं चलाती।
core-verify-service-other-account = सेवा मौजूद है, लेकिन अपेक्षित खाते के रूप में नहीं चलती।
core-verify-service-matches = सेवा मौजूद है, और पुनर्स्थापित बाइनरी को उसके तर्कों और खाते के साथ चलाती है।
core-verify-service-missing = सेवा अनुपलब्ध है।
# { $error } is what Windows said, in its own words.
core-verify-service-unreadable = सेवा पढ़ी नहीं जा सकती: { $error }
core-verify-driver-other-image = ड्राइवर सेवा पुनर्स्थापित इमेज की ओर इंगित नहीं करती।
core-verify-driver-not-registered = पुनर्स्थापना के बाद बंडल की गई ड्राइवर सेवा पंजीकृत नहीं है।
# { $error } is what Windows said, in its own words.
core-verify-driver-unreadable = ड्राइवर सेवा पढ़ी नहीं जा सकती: { $error }
core-verify-driver-image-missing = ड्राइवर इमेज अनुपलब्ध है।
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-unchecked = ड्राइवर इमेज की जाँच नहीं की जा सकी: { $error }
core-verify-task-matches = शेड्यूल किया गया कार्य मौजूद है और पुनर्स्थापित प्रोग्राम चलाता है।
core-verify-task-runs-other = शेड्यूल किया गया कार्य मौजूद है, लेकिन पुनर्स्थापित प्रोग्राम नहीं चलाता।
core-verify-task-missing = शेड्यूल किया गया कार्य अनुपलब्ध है।
# { $error } is what Windows said, in its own words.
core-verify-task-unreadable = शेड्यूल किए गए कार्य की क्वेरी विफल रही: { $error }
core-verify-shortcut-matches = शॉर्टकट मौजूद है और उसका लक्ष्य पथ पुनर्स्थापित लक्ष्य से मेल खाता है।
core-verify-shortcut-opens-other = शॉर्टकट मौजूद है, लेकिन उसका लक्ष्य पथ पुनर्स्थापित लक्ष्य से मेल नहीं खाता।
core-verify-shortcut-missing = शॉर्टकट अनुपलब्ध है।
# { $error } is what Windows said, in its own words.
core-verify-shortcut-unreadable = शॉर्टकट की क्वेरी विफल रही: { $error }

## Warnings and errors a check gives, which the app and the installer show.

core-warning-verify-schema-unsupported = पैकेज मेनिफ़ेस्ट स्कीमा समर्थित नहीं है।
# { $error } is what went wrong, in Windows' or the program's own words.
core-warning-verify-package-file-failed = पैकेज फ़ाइल का सत्यापन विफल रहा: { $error }
core-warning-verify-folder-missing = खाली फ़ोल्डर अनुपलब्ध है।
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-key-missing = रजिस्ट्री कुंजी अनुपलब्ध है या पढ़ी नहीं जा सकती: { $error }
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-value-missing = रजिस्ट्री मान अनुपलब्ध है या पढ़ा नहीं जा सकता: { $error }
core-warning-verify-registry-kind-missing = रजिस्ट्री मान का प्रकार अनुपलब्ध है।
core-warning-verify-firewall-rule-missing = Windows फ़ायरवॉल में यह नियम नहीं है।
core-warning-verify-service-scope = सेवा सत्यापन HKLM सेवा कॉन्फ़िगरेशन पढ़ता है और सेवाएँ प्रारंभ नहीं करता।
core-warning-verify-driver-manual = एक या अधिक विक्रेता/इनबॉक्स ड्राइवर WinAppMigrator द्वारा फिर से वितरित नहीं किए जाते; उन्हें लक्ष्य PC पर विक्रेता के हस्ताक्षरित ड्राइवर पैकेज से फिर से इंस्टॉल करें।
core-warning-verify-task-scope = शेड्यूल किए गए कार्य का सत्यापन केवल पंजीकरण और कमांड संदर्भ की जाँच करता है।

## The verification report written beside a package, verification_report.md. Its headings follow a "#".

core-verify-report-title = WinAppMigrator सत्यापन रिपोर्ट
# { $time } is when, as a date and time in UTC, such as 2026-11-15T10:00:00Z.
core-verify-report-verified = सत्यापन का समय (UTC): { $time }
# { $valid } is true or false, which are not translated.
core-verify-report-package-valid = पैकेज मान्य: { $valid }
# { $valid } is true or false, which are not translated.
core-verify-report-restore-valid = पुनर्स्थापना मान्य: { $valid }
core-verify-report-restore-not-run = पुनर्स्थापना मान्य: नहीं चलाया गया
core-verify-report-items = आइटम
# { $message } is what a check found, and { $source } the file or setting it looked at.
core-verify-report-item-from = { $message } स्रोत: { $source }
core-verify-report-warnings = चेतावनियाँ
core-verify-report-errors = त्रुटियाँ
core-verify-report-none = कोई नहीं।
