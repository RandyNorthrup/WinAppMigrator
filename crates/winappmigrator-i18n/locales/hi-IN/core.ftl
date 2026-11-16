# Words the core writes for people: warnings, refusals, errors, and the installer's list of what it sets up.
# Message names start with core-.


## Warnings a package records when it is built, which an installer shows later, maybe on another PC, in that PC's
## language.

# { $count } is how many shortcuts.
core-warning-shortcuts-unreadable =
    { $count ->
        [one] इस ड्राइव पर { $count } शॉर्टकट किसी फ़ाइल का नाम नहीं लेता, इसलिए यह पता नहीं चल सका कि वह इस ऐप को खोलता है या नहीं।
       *[other] इस ड्राइव पर { $count } शॉर्टकट किसी फ़ाइल का नाम नहीं लेते, इसलिए यह पता नहीं चल सका कि वे इस ऐप को खोलते हैं या नहीं।
    }

## Undoing an install: what an uninstall, or a failed install, says it put back, left, or could not undo. Each line is
## shown on its own, in a list.

# { $app_name } is the app's name.
core-rollback-record-damaged = { $app_name } के रिकॉर्ड का एक भाग पढ़ा नहीं जा सका, इसलिए उसमें वर्णित सब कुछ अब भी यथास्थान है, और हो सकता है कि इंस्टॉल द्वारा प्रतिस्थापित की गई कोई फ़ाइल वापस न आए।
core-rollback-settings-entry-stays = सेटिंग्स में ऐप की प्रविष्टि बनी रहती है, ताकि इसे वहाँ से फिर से चलाया जा सके।
# { $name } is a runtime's name, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-rollback-prerequisite-stays = { $name } इंस्टॉल किया गया था, या शायद किया गया हो, और यह बना रहता है, क्योंकि अन्य प्रोग्राम इसका उपयोग कर रहे हो सकते हैं।
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-drive-not-connected = इस इंस्टॉल ने { $folder } में जो रखा था, वह ऐसी ड्राइव पर है जो कनेक्ट नहीं है। उसे कनेक्ट करें, और यह कार्य पूरा किया जा सकता है।
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-record-part-unread = इस इंस्टॉल ने { $folder } में जो रखा था, वह बना रहता है, क्योंकि रिकॉर्ड का वह भाग पढ़ा नहीं जा सका।

## Why a file or folder was left, shown after "was left:" in the messages below them. Each is part of a sentence, so
## starts with a small letter and has no full stop.

# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-being-changed = इसके चलते समय कोई चीज़ { $folder } को बदल रही थी, इसलिए उसमें आगे कुछ नहीं किया गया। जो ऐसा कर रहा है, उसे बंद करें, और इसे फिर से चलाएँ
# { $folder } is a folder; { $error } is what Windows said went wrong.
core-rollback-reason-not-opened = { $folder } को खोला नहीं जा सका: { $error }
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-no-record = इसका कोई रिकॉर्ड नहीं है कि { $folder } कहाँ था
# { $folder } is a folder; { $actual } is the folder it now leads to.
core-rollback-reason-moved = { $folder } अब { $actual } की ओर ले जाता है, न कि वहाँ जहाँ यह इंस्टॉल के समय ले जाता था
# Why a service was not stopped.
core-rollback-reason-did-not-stop = यह समय पर नहीं रुकी

## Files and folders

# { $path } is a folder; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-folder-left = फ़ोल्डर { $path } यथास्थान छोड़ा गया: { $reason }।
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-removed = फ़ोल्डर { $path } हटाया गया
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-holds-others = फ़ोल्डर { $path } बना रहता है, क्योंकि इसमें ऐसी फ़ाइलें हैं जिन्हें इस इंस्टॉल ने वहाँ नहीं रखा था।
# { $path } is a folder; { $error } is what Windows said went wrong.
core-rollback-folder-not-removed = फ़ोल्डर { $path } हटाया नहीं जा सका: { $error }।
# { $path } is a file; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-file-left = फ़ाइल { $path } यथास्थान छोड़ी गई: { $reason }।
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-folder-gone = { $path } को वापस नहीं रखा जा रहा है, क्योंकि जिस फ़ोल्डर में यह थी, वह अब मौजूद नहीं है।
# { $path } is a file; { $error } is what went wrong.
core-rollback-partial-not-removed = एक अधूरी फ़ाइल हटाई नहीं जा सकी: { $path } ({ $error })।
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-checked = { $path } की जाँच नहीं की जा सकी: { $error }।
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-removed = { $path } को हटाया गया
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-removed = { $path } को हटाया नहीं जा सका: { $error }।
# { $path } is a file this install had replaced, and has put back.
core-rollback-file-put-back = { $path } को वापस रखा गया
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-put-back = { $path } को वापस नहीं रखा जा सका: { $error }।
# Why a file could not be put back.
core-rollback-backup-gone = बैकअप अब मौजूद नहीं है
# { $path } is a file.
core-rollback-no-parent-folder = { $path } का कोई पैरेंट फ़ोल्डर नहीं है

## A file that stays. { $why } says why: still-used (something that stays still uses it), changed (it has changed since
## it was installed), or not-asked (it is someone's data, and they did not ask for it to go). { $path } is the file.

core-rollback-file-stays =
    { $why ->
        [still-used] { $path } बनी रहती है, क्योंकि बनी रहने वाली कोई चीज़ अब भी इसका उपयोग करती है।
        [changed] { $path } बनी रहती है, क्योंकि इंस्टॉल किए जाने के बाद से यह बदल गई है।
       *[not-asked] { $path } बनी रहती है, क्योंकि इसे हटाने के लिए नहीं कहा गया था।
    }
# The file it replaced was to be put beside it, under another name, and every name was taken.
core-rollback-file-stays-no-name-beside =
    { $why ->
        [still-used] { $path } बनी रहती है, क्योंकि बनी रहने वाली कोई चीज़ अब भी इसका उपयोग करती है, और जिस फ़ाइल को इसने प्रतिस्थापित किया था, उसे इसके बगल में नहीं रखा जा सका: उसके लिए हर नाम पहले से लिया जा चुका है।
        [changed] { $path } बनी रहती है, क्योंकि इंस्टॉल किए जाने के बाद से यह बदल गई है, और जिस फ़ाइल को इसने प्रतिस्थापित किया था, उसे इसके बगल में नहीं रखा जा सका: उसके लिए हर नाम पहले से लिया जा चुका है।
       *[not-asked] { $path } बनी रहती है, क्योंकि इसे हटाने के लिए नहीं कहा गया था, और जिस फ़ाइल को इसने प्रतिस्थापित किया था, उसे इसके बगल में नहीं रखा जा सका: उसके लिए हर नाम पहले से लिया जा चुका है।
    }
# { $name } is the name the file it replaced now has, such as settings.ini.before-install.
core-rollback-file-stays-beside =
    { $why ->
        [still-used] { $path } बनी रहती है, क्योंकि बनी रहने वाली कोई चीज़ अब भी इसका उपयोग करती है। जिस फ़ाइल को इसने प्रतिस्थापित किया था, वह इसके बगल में { $name } नाम से है।
        [changed] { $path } बनी रहती है, क्योंकि इंस्टॉल किए जाने के बाद से यह बदल गई है। जिस फ़ाइल को इसने प्रतिस्थापित किया था, वह इसके बगल में { $name } नाम से है।
       *[not-asked] { $path } बनी रहती है, क्योंकि इसे हटाने के लिए नहीं कहा गया था। जिस फ़ाइल को इसने प्रतिस्थापित किया था, वह इसके बगल में { $name } नाम से है।
    }
# { $error } is what went wrong.
core-rollback-file-stays-not-beside =
    { $why ->
        [still-used] { $path } बनी रहती है, क्योंकि बनी रहने वाली कोई चीज़ अब भी इसका उपयोग करती है, और जिस फ़ाइल को इसने प्रतिस्थापित किया था, उसे इसके बगल में नहीं रखा जा सका: { $error }।
        [changed] { $path } बनी रहती है, क्योंकि इंस्टॉल किए जाने के बाद से यह बदल गई है, और जिस फ़ाइल को इसने प्रतिस्थापित किया था, उसे इसके बगल में नहीं रखा जा सका: { $error }।
       *[not-asked] { $path } बनी रहती है, क्योंकि इसे हटाने के लिए नहीं कहा गया था, और जिस फ़ाइल को इसने प्रतिस्थापित किया था, उसे इसके बगल में नहीं रखा जा सका: { $error }।
    }

## The registry and environment variables. { $kind } says what { $name } is: key (a registry key, such as
## HKLM\Software\App), value (a registry value, such as HKLM\Software\App\Version), user-variable or machine-variable
## (an environment variable of the user's or of the whole PC, such as APP_HOME), or user-list or machine-list (a
## variable that is a list of folders, such as Path). { $error } is what Windows said went wrong.

# How a registry key's unnamed value is shown, as Registry Editor shows it.
core-registry-default-value-name = (डिफ़ॉल्ट)
core-rollback-setting-other-account =
    { $kind ->
        [key] कुंजी { $name } बनी रहती है, क्योंकि यह उस खाते की है जिसने इसे इंस्टॉल किया था, और वह खाता साइन इन नहीं है।
        [user-variable] उपयोगकर्ता का परिवेश चर { $name } बना रहता है, क्योंकि यह उस खाते का है जिसने इसे इंस्टॉल किया था, और वह खाता साइन इन नहीं है।
        [machine-variable] मशीन का परिवेश चर { $name } बना रहता है, क्योंकि यह उस खाते का है जिसने इसे इंस्टॉल किया था, और वह खाता साइन इन नहीं है।
        [user-list] उपयोगकर्ता का { $name } बना रहता है, क्योंकि यह उस खाते का है जिसने इसे इंस्टॉल किया था, और वह खाता साइन इन नहीं है।
        [machine-list] मशीन का { $name } बना रहता है, क्योंकि यह उस खाते का है जिसने इसे इंस्टॉल किया था, और वह खाता साइन इन नहीं है।
       *[value] रजिस्ट्री मान { $name } बना रहता है, क्योंकि यह उस खाते का है जिसने इसे इंस्टॉल किया था, और वह खाता साइन इन नहीं है।
    }
core-rollback-setting-not-reached =
    { $kind ->
        [key] कुंजी { $name } तक नहीं पहुँचा जा सका: { $error }।
        [user-variable] उपयोगकर्ता के परिवेश चर { $name } तक नहीं पहुँचा जा सका: { $error }।
        [machine-variable] मशीन के परिवेश चर { $name } तक नहीं पहुँचा जा सका: { $error }।
        [user-list] उपयोगकर्ता के { $name } तक नहीं पहुँचा जा सका: { $error }।
        [machine-list] मशीन के { $name } तक नहीं पहुँचा जा सका: { $error }।
       *[value] रजिस्ट्री मान { $name } तक नहीं पहुँचा जा सका: { $error }।
    }
core-rollback-setting-not-read =
    { $kind ->
        [key] कुंजी { $name } पढ़ी नहीं जा सकी: { $error }।
        [user-variable] उपयोगकर्ता का परिवेश चर { $name } पढ़ा नहीं जा सका: { $error }।
        [machine-variable] मशीन का परिवेश चर { $name } पढ़ा नहीं जा सका: { $error }।
        [user-list] उपयोगकर्ता का { $name } पढ़ा नहीं जा सका: { $error }।
        [machine-list] मशीन का { $name } पढ़ा नहीं जा सका: { $error }।
       *[value] रजिस्ट्री मान { $name } पढ़ा नहीं जा सका: { $error }।
    }
core-rollback-setting-changed =
    { $kind ->
        [user-variable] उपयोगकर्ता का परिवेश चर { $name } बना रहता है, क्योंकि इंस्टॉल किए जाने के बाद से यह बदल गया है।
        [machine-variable] मशीन का परिवेश चर { $name } बना रहता है, क्योंकि इंस्टॉल किए जाने के बाद से यह बदल गया है।
        [user-list] उपयोगकर्ता का { $name } बना रहता है, क्योंकि इंस्टॉल किए जाने के बाद से यह बदल गया है।
        [machine-list] मशीन का { $name } बना रहता है, क्योंकि इंस्टॉल किए जाने के बाद से यह बदल गया है।
       *[value] रजिस्ट्री मान { $name } बना रहता है, क्योंकि इंस्टॉल किए जाने के बाद से यह बदल गया है।
    }
# Said when it is back as it was before the install.
core-rollback-setting-put-back =
    { $kind ->
        [user-variable] उपयोगकर्ता का परिवेश चर { $name } पहले जैसा किया गया
        [machine-variable] मशीन का परिवेश चर { $name } पहले जैसा किया गया
        [user-list] उपयोगकर्ता का { $name } पहले जैसा किया गया
        [machine-list] मशीन का { $name } पहले जैसा किया गया
       *[value] रजिस्ट्री मान { $name } पहले जैसा किया गया
    }
core-rollback-setting-not-put-back =
    { $kind ->
        [user-variable] उपयोगकर्ता का परिवेश चर { $name } पहले जैसा नहीं किया जा सका: { $error }।
        [machine-variable] मशीन का परिवेश चर { $name } पहले जैसा नहीं किया जा सका: { $error }।
        [user-list] उपयोगकर्ता का { $name } पहले जैसा नहीं किया जा सका: { $error }।
        [machine-list] मशीन का { $name } पहले जैसा नहीं किया जा सका: { $error }।
       *[value] रजिस्ट्री मान { $name } पहले जैसा नहीं किया जा सका: { $error }।
    }
# A registry value the install wrote, which is the user's data, and stays unless they asked for it to go.
core-rollback-value-kept-by-request = रजिस्ट्री मान { $name } बना रहता है, क्योंकि इसे हटाने के लिए नहीं कहा गया था।
core-rollback-key-holds-others = कुंजी { $name } बनी रहती है, क्योंकि इसमें ऐसी चीज़ें हैं जिन्हें इस इंस्टॉल ने वहाँ नहीं रखा था।
core-rollback-key-removed = कुंजी { $name } हटाई गई
core-rollback-key-not-removed = कुंजी { $name } हटाई नहीं जा सकी: { $error }।
# { $kind } is user-list or machine-list.
core-rollback-list-entries-removed =
    { $kind ->
        [user-list] उपयोगकर्ता के { $name } से इस इंस्टॉल की प्रविष्टियाँ हटाई गईं
       *[machine-list] मशीन के { $name } से इस इंस्टॉल की प्रविष्टियाँ हटाई गईं
    }
core-rollback-list-entries-not-removed =
    { $kind ->
        [user-list] उपयोगकर्ता के { $name } से इस इंस्टॉल की प्रविष्टियाँ हटाई नहीं जा सकीं: { $error }।
       *[machine-list] मशीन के { $name } से इस इंस्टॉल की प्रविष्टियाँ हटाई नहीं जा सकीं: { $error }।
    }

## Services and drivers. { $kind } is service or driver; { $name } is its name, such as AppUpdater. { $error } and
## { $reason } are what Windows said went wrong.

core-rollback-service-not-read =
    { $kind ->
        [driver] ड्राइवर { $name } पढ़ा नहीं जा सका: { $error }।
       *[service] सेवा { $name } पढ़ी नहीं जा सकी: { $error }।
    }
core-rollback-service-changed =
    { $kind ->
        [driver] ड्राइवर { $name } बना रहता है, क्योंकि इंस्टॉल किए जाने के बाद से यह बदल गया है।
       *[service] सेवा { $name } बनी रहती है, क्योंकि इंस्टॉल किए जाने के बाद से यह बदल गई है।
    }
core-rollback-service-not-stopped =
    { $kind ->
        [driver] ड्राइवर { $name } को रोका नहीं जा सका ({ $reason }), इसलिए हो सकता है कि यह जिन चीज़ों का उपयोग कर रहा है, वे PC के पुनः प्रारंभ होने तक बनी रहें।
       *[service] सेवा { $name } को रोका नहीं जा सका ({ $reason }), इसलिए हो सकता है कि यह जिन चीज़ों का उपयोग कर रही है, वे PC के पुनः प्रारंभ होने तक बनी रहें।
    }
core-rollback-service-stopped-for-this =
    { $kind ->
        [driver] ऐसा करने के लिए ड्राइवर { $name } को रोका गया। जब आपको इसकी आवश्यकता हो, तो इसे फिर से प्रारंभ करें।
       *[service] ऐसा करने के लिए सेवा { $name } को रोका गया। जब आपको इसकी आवश्यकता हो, तो इसे फिर से प्रारंभ करें।
    }
core-rollback-service-not-removed =
    { $kind ->
        [driver] ड्राइवर { $name } को हटाया नहीं जा सका: { $reason }।
       *[service] सेवा { $name } को हटाया नहीं जा सका: { $reason }।
    }
core-rollback-service-marked-for-removal =
    { $kind ->
        [driver] ड्राइवर { $name } को हटाने के लिए चिह्नित किया गया है, और यह PC के पुनः प्रारंभ होने पर, या जिस चीज़ ने इसे खोल रखा है उसके इसे छोड़ने पर, हट जाएगा।
       *[service] सेवा { $name } को हटाने के लिए चिह्नित किया गया है, और यह PC के पुनः प्रारंभ होने पर, या जिस चीज़ ने इसे खोल रखा है उसके इसे छोड़ने पर, हट जाएगी।
    }
core-rollback-service-removed =
    { $kind ->
        [driver] ड्राइवर { $name } हटाया गया
       *[service] सेवा { $name } हटाई गई
    }
core-rollback-service-not-restored =
    { $kind ->
        [driver] ड्राइवर { $name } को पहले जैसा नहीं किया जा सका: { $reason }।
       *[service] सेवा { $name } को पहले जैसा नहीं किया जा सका: { $reason }।
    }
core-rollback-service-restored =
    { $kind ->
        [driver] ड्राइवर { $name } की सेटिंग्स पहले जैसी की गईं
       *[service] सेवा { $name } की सेटिंग्स पहले जैसी की गईं
    }
core-rollback-service-not-restarted = सेवा { $name } इस इंस्टॉल से पहले चल रही थी, और फिर से प्रारंभ नहीं हुई: { $reason }।
# { $account } is the account the service ran as, such as CONTOSO\backup.
core-rollback-service-account-again = सेवा { $name } { $account } के रूप में चलती थी। इसका पासवर्ड नहीं रखा जाता, इसलिए "सेवाएँ" में इसका खाता फिर से सेट करें।

## Firewall rules. { $name } is the rule's name; { $error } is what went wrong.

core-rollback-firewall-not-read = फ़ायरवॉल नियम { $name } पढ़ा नहीं जा सका: { $error }।
core-rollback-firewall-says-otherwise = फ़ायरवॉल नियम { $name } बना रहता है, क्योंकि उस नाम से जो नियम मौजूद है, उसमें कुछ और लिखा है।
core-rollback-firewall-changed = फ़ायरवॉल नियम { $name } बना रहता है, क्योंकि इंस्टॉल किए जाने के बाद से यह बदल गया है।
# The capital "The" is as the app has always said it.
core-rollback-firewall-removed = फ़ायरवॉल नियम { $name } हटाया गया
core-rollback-firewall-not-removed = फ़ायरवॉल नियम { $name } हटाया नहीं जा सका: { $error }।

## Scheduled tasks. { $path } is a folder of tasks, and { $name } a task with its folder, such as \App\Nightly Update.
## { $error } and { $reason } are what went wrong.

core-rollback-task-folder-holds-others = कार्य फ़ोल्डर { $path } बना रहता है, क्योंकि इसमें ऐसे कार्य हैं जिन्हें इस इंस्टॉल ने वहाँ नहीं रखा था।
core-rollback-task-folder-not-removed = कार्य फ़ोल्डर { $path } हटाया नहीं जा सका: { $error }।
core-rollback-task-not-read = कार्य { $name } पढ़ा नहीं जा सका: { $error }।
core-rollback-task-changed = कार्य { $name } बना रहता है, क्योंकि इंस्टॉल किए जाने के बाद से यह बदल गया है।
core-rollback-task-removed = कार्य { $name } हटाया गया
core-rollback-task-not-removed = कार्य { $name } हटाया नहीं जा सका: { $reason }।
core-rollback-task-put-back = कार्य { $name } पहले जैसा किया गया
core-rollback-task-not-put-back = कार्य { $name } पहले जैसा नहीं किया जा सका: { $reason }।

## When an install cannot be undone at all

# { $error } is what went wrong.
core-rollback-record-unreadable = इस इंस्टॉल का रिकॉर्ड पढ़ा नहीं जा सका, इसलिए कुछ भी पूर्ववत नहीं किया गया: { $error }।
# { $error } is what went wrong.
core-rollback-nothing-undone = कुछ भी पूर्ववत नहीं किया गया: { $error }।
# One app's part of the message below. { $app_name } is the app's name; { $failures } is what could not be undone, a
# sentence or more; { $folder } is the folder that holds the install's record.
core-rollback-earlier-install-stuck = { $app_name }: { $failures } अगर इसे कभी पूर्ववत नहीं किया जा सकता, तो कोई व्यवस्थापक { $folder } को हटाकर इसे छोड़ सकता है।
# Why an install did not start. { $details } is one or more of the message above.
core-rollback-earlier-installs-not-undone = कभी पूरा न हुआ एक पिछला इंस्टॉल पूरी तरह पूर्ववत नहीं किया जा सका, इसलिए कुछ भी इंस्टॉल नहीं किया गया। जो भी इनका उपयोग कर रहा है, उसे बंद करें और फिर से प्रयास करें। { $details }
core-rollback-install-busy = इस इंस्टॉल को कोई अन्य इंस्टॉलर या अनइंस्टॉलर बदल रहा है

## The summary an uninstall writes on the command line, one line each, with what each says below it. { $count } is how
## many.

core-rollback-count-undone = पूर्ववत किया गया: { $count }
core-rollback-count-failed = पूर्ववत नहीं किया जा सका: { $count }
# What was left on purpose: it has changed since the install, or it is the user's.
core-rollback-count-kept = यथास्थान छोड़ा गया: { $count }
# What no uninstall can take back, such as a runtime that was installed, which other programs may use.
core-rollback-count-left-behind = पीछे छूट गया: { $count }

## Which Windows a PC or drive has, as the app shows it beside the drive.

# { $name } is the Windows' name, such as Windows 7 Professional Service Pack 1.
core-system-windows-32-bit = { $name }, 32-बिट
# { $name } is the Windows' name, such as Windows 11 Pro.
core-system-windows-64-bit = { $name }, 64-बिट
# A drive's Windows, when it only says it is older than Vista.
core-system-windows-xp-or-older = Windows XP या उससे पुराना

## Reading a connected drive, or another account's settings. Each is why it could not be done, and some start with a
## small letter, as they follow other words.

core-system-needs-administrator = कनेक्ट की गई ड्राइव या किसी अन्य खाते की सेटिंग्स पढ़ने के लिए व्यवस्थापक अधिकारों की आवश्यकता है
core-system-no-program-data = इस PC का ProgramData फ़ोल्डर नहीं मिला
# { $path } is a file, such as D:\Windows\System32\config\SOFTWARE.
core-system-not-a-hive-file = हाइव फ़ाइल नहीं है: { $path }
# { $path } is a file; { $size } is its size in bytes.
core-system-hive-too-large = { $path } { $size } बाइट की है, जो रजिस्ट्री हाइव होने के लिए बहुत बड़ी है, या इस ड्राइव के सभी हाइव के कुल आकार से भी अधिक है
# { $key } is a registry key, such as Software\Microsoft\Windows\CurrentVersion.
core-system-hive-no-mapping = ऑफ़लाइन हाइव में HKLM\{ $key } के लिए कोई मैपिंग नहीं है
# { $key } is where a drive's registry was to be loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said.
# "reg load" is a command's name.
core-system-hive-not-loaded = reg load { $key } विफल रहा: { $error }
# { $key } is where a drive's registry was loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said. "reg
# unload" is a command's name.
core-system-hive-not-unloaded = reg unload { $key } विफल रहा: { $error }
# { $drive } is a drive, such as D:\.
core-system-not-a-system-drive = { $drive } Windows सिस्टम ड्राइव नहीं है (कोई \Windows फ़ोल्डर नहीं है)।
# { $drive } is a drive, such as C:\. "This PC" is the name of a choice in the app.
core-system-own-system-drive = { $drive } इस PC की अपनी सिस्टम ड्राइव है। इस PC को पढ़ने के लिए "यह PC" चुनें।
# { $drive } is a drive, such as D:\.
core-system-registry-through-link = { $drive } Windows सिस्टम ड्राइव नहीं है: इसकी रजिस्ट्री तक का रास्ता किसी लिंक से होकर जाता है।

## Warnings a scan records about whose data it read.

# { $account } is an account's name or id, such as alice or S-1-5-21-...-1001.
core-warning-scan-account-not-on-record = खाता { $account } यहाँ रिकॉर्ड में नहीं है, इसलिए किसी का भी प्रति-उपयोगकर्ता डेटा या सेटिंग्स शामिल नहीं की गईं।
# { $account } is an account's name, such as CONTOSO\alice.
core-warning-scan-signed-in-account-no-profile = यहाँ साइन इन खाते, { $account }, की कोई प्रोफ़ाइल रिकॉर्ड में नहीं है, इसलिए किसी का भी प्रति-उपयोगकर्ता डेटा या सेटिंग्स शामिल नहीं की गईं।
# { $reason } is why Windows could not say.
core-warning-scan-signed-in-account-unknown = यह पता नहीं चल सका कि कौन साइन इन है ({ $reason }), इसलिए किसी का भी प्रति-उपयोगकर्ता डेटा या सेटिंग्स शामिल नहीं की गईं। चुनें कि किसका डेटा शामिल करना है।
core-warning-scan-last-account-unknown = यह ड्राइव नहीं बताती कि आखिरी बार किसने साइन इन किया था, और इस पर एक से अधिक खाते हैं, इसलिए किसी का भी प्रति-उपयोगकर्ता डेटा या सेटिंग्स शामिल नहीं की गईं। चुनें कि किसका डेटा शामिल करना है।
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-settings-unreadable = { $account } की सेटिंग्स पढ़ी नहीं जा सकीं, इसलिए उन्हें शामिल नहीं किया गया: { $error }
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-classes-unreadable = { $account } के लिए पंजीकृत COM क्लास और फ़ाइल प्रकार पढ़े नहीं जा सके, इसलिए उन्हें शामिल नहीं किया गया: { $error }

## Paths a scan did not follow. { $count } is how many; { $paths } is some of them, separated by semicolons; { $more } is
## how many more there are, not shown.

core-warning-scan-drive-paths-not-followed =
    { $count ->
        [one] इस ड्राइव पर रिकॉर्ड किया गया एक पथ पढ़ा नहीं जा सका, इसलिए वहाँ से कुछ भी शामिल नहीं किया गया। हर ऐसा पथ या तो किसी अन्य डिस्क पर है, या पूरा पथ नहीं है, या किसी ऐसे लिंक से होकर जाता है जो ड्राइव से बाहर ले जाएगा: { $paths }
       *[other] इस ड्राइव पर रिकॉर्ड किए गए कुछ पथ पढ़े नहीं जा सके, इसलिए वहाँ से कुछ भी शामिल नहीं किया गया। हर ऐसा पथ या तो किसी अन्य डिस्क पर है, या पूरा पथ नहीं है, या किसी ऐसे लिंक से होकर जाता है जो ड्राइव से बाहर ले जाएगा: { $paths }
    }
core-warning-scan-drive-paths-not-followed-more = इस ड्राइव पर रिकॉर्ड किए गए कुछ पथ पढ़े नहीं जा सके, इसलिए वहाँ से कुछ भी शामिल नहीं किया गया। हर ऐसा पथ या तो किसी अन्य डिस्क पर है, या पूरा पथ नहीं है, या किसी ऐसे लिंक से होकर जाता है जो ड्राइव से बाहर ले जाएगा: { $paths } और { $more } अन्य
core-warning-scan-network-paths-not-followed =
    { $count ->
        [one] इस PC पर रिकॉर्ड किया गया एक पथ पढ़ा नहीं जा सका, इसलिए वहाँ से कुछ भी शामिल नहीं किया गया। हर ऐसा पथ या तो इस PC की किसी डिस्क पर नहीं है, जहाँ उसे देखने भर से उससे कनेक्ट हो जाता, या उसका नाम ऐसा है जो कुछ और खोलेगा: { $paths }
       *[other] इस PC पर रिकॉर्ड किए गए कुछ पथ पढ़े नहीं जा सके, इसलिए वहाँ से कुछ भी शामिल नहीं किया गया। हर ऐसा पथ या तो इस PC की किसी डिस्क पर नहीं है, जहाँ उसे देखने भर से उससे कनेक्ट हो जाता, या उसका नाम ऐसा है जो कुछ और खोलेगा: { $paths }
    }
core-warning-scan-network-paths-not-followed-more = इस PC पर रिकॉर्ड किए गए कुछ पथ पढ़े नहीं जा सके, इसलिए वहाँ से कुछ भी शामिल नहीं किया गया। हर ऐसा पथ या तो इस PC की किसी डिस्क पर नहीं है, जहाँ उसे देखने भर से उससे कनेक्ट हो जाता, या उसका नाम ऐसा है जो कुछ और खोलेगा: { $paths } और { $more } अन्य

## What a scan with none of an account's data leaves out of a package, in the app's list of what was left out. Each
## has a name, and below it why.

core-account-left-out-shortcuts = इस खाते के अपने फ़ोल्डरों में मौजूद शॉर्टकट
# { $count } is how many shortcuts.
core-account-left-out-shortcuts-why =
    { $count ->
        [one] { $count } शॉर्टकट, जो केवल उपयोगकर्ता डेटा विकल्प के साथ शामिल होता है
       *[other] { $count } शॉर्टकट, जो केवल उपयोगकर्ता डेटा विकल्प के साथ शामिल होते हैं
    }
core-account-left-out-variables = इस खाते के परिवेश चर
# { $count } is how many variables.
core-account-left-out-variables-why =
    { $count ->
        [one] { $count } चर, जो केवल उपयोगकर्ता डेटा विकल्प के साथ शामिल होता है
       *[other] { $count } चर, जो केवल उपयोगकर्ता डेटा विकल्प के साथ शामिल होते हैं
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-account-left-out-task = शेड्यूल किया गया कार्य { $task }
core-account-task-works-on-account-folder = यह किसी खाते के फ़ोल्डरों में किसी स्थान पर काम करता है, या उसके नाम पर है, और केवल उपयोगकर्ता डेटा विकल्प के साथ शामिल होता है
core-account-task-runs-from-account-folder = यह किसी खाते के फ़ोल्डरों से जो कुछ और भी चलाता है, वह केवल उपयोगकर्ता डेटा विकल्प के साथ शामिल होता है
# { $service } is a service's name, such as AppUpdater.
core-account-left-out-service = सेवा { $service }
core-account-service-works-on-account-folder = यह किसी खाते के फ़ोल्डरों में किसी स्थान पर काम करती है, और केवल उपयोगकर्ता डेटा विकल्प के साथ शामिल होती है

## What a package was found to still say of whose PC it came from, after a build with none of an account's data.

# The parts of a package, as the warning below names them.
core-account-part-files = ऐप की अपनी फ़ाइलों के पथ
core-account-part-registry = रजिस्ट्री मान
core-account-part-settings-entry = सेटिंग्स में ऐप की प्रविष्टि
core-account-part-main-program = इसका मुख्य प्रोग्राम कहाँ है
core-account-part-shortcuts = शॉर्टकट
core-account-part-tasks = शेड्यूल किए गए कार्य
core-account-part-services = सेवाएँ
core-account-part-variables = परिवेश चर
# One place in the list below. { $place } is one of the parts above, or a file's name; { $count } is how many times.
core-account-mention-place = { $place } ({ $count })
# { $kind } is what was found: account (an account's name), folder (where an account's folder is), computer (the PC's
# name), or id (an account's id). { $places } is a list of where, from the message above.
core-warning-scan-package-mentions =
    { $kind ->
        [account] इस पैकेज में { $places } में किसी खाते के नाम का उल्लेख है। इसे किसी भी खाते के डेटा के बिना बनाया गया था, और जिसके पास भी इंस्टॉलर है, वह इसे पढ़ सकता है।
        [folder] इस पैकेज में { $places } में इस बात का उल्लेख है कि किसी खाते का फ़ोल्डर कहाँ है। इसे किसी भी खाते के डेटा के बिना बनाया गया था, और जिसके पास भी इंस्टॉलर है, वह इसे पढ़ सकता है।
        [computer] इस पैकेज में { $places } में PC के नाम का उल्लेख है। इसे किसी भी खाते के डेटा के बिना बनाया गया था, और जिसके पास भी इंस्टॉलर है, वह इसे पढ़ सकता है।
       *[id] इस पैकेज में { $places } में किसी खाते की ID का उल्लेख है। इसे किसी भी खाते के डेटा के बिना बनाया गया था, और जिसके पास भी इंस्टॉलर है, वह इसे पढ़ सकता है।
    }
# { $count } is how many folders.
core-warning-scan-folders-of-others-unreadable =
    { $count ->
        [one] ऐप के बाहर का { $count } फ़ोल्डर पढ़ा नहीं जा सका, इसलिए वहाँ मौजूद ऐप का डेटा अनुपलब्ध हो सकता है।
       *[other] ऐप के बाहर के { $count } फ़ोल्डर पढ़े नहीं जा सके, इसलिए वहाँ मौजूद ऐप का डेटा अनुपलब्ध हो सकता है।
    }

## What a scan leaves out because it looks like a credential, or is a license with the License option off, in the app's
## list of what was left out. Each has a name, and below it why.

core-secret-why-credential = यह क्रेडेंशियल जैसा दिखता है
core-secret-why-license-option-off = "लाइसेंस और सक्रियण डेटा" विकल्प बंद है
# { $count } is how many values of a registry key.
core-secret-why-account-values =
    { $count ->
        [one] { $count } मान, जो केवल उपयोगकर्ता डेटा विकल्प के साथ शामिल होता है
       *[other] { $count } मान, जो केवल उपयोगकर्ता डेटा विकल्प के साथ शामिल होते हैं
    }
# Shown in place of a registry value's name when its name itself looks like a password or a key.
core-secret-name-holds-credential = (ऐसा मान जिसके नाम में क्रेडेंशियल है)
# Shown in place of an option's name, for an argument on a command line that has no name.
core-secret-unnamed-argument = क्रेडेंशियल वाला एक तर्क
# { $option } is an option taken out of a command line, such as /password, or the message above. { $place } says
# where the command line was: service, task, shortcut, opener (what opens a file type), opener-unnamed (the same, of a
# type with no name), or uninstall. { $name } is the service, task, shortcut, file type, or app.
core-secret-taken-from =
    { $place ->
        [service] { $name } को चलाने के तरीके में { $option }
        [task] कार्य { $name } को चलाने के तरीके में { $option }
        [shortcut] शॉर्टकट { $name } में { $option }
        [opener] { $name } को खोलने वाली चीज़ में { $option }
        [opener-unnamed] किसी फ़ाइल प्रकार को खोलने वाली चीज़ में { $option }
       *[uninstall] { $name } को अनइंस्टॉल करने के तरीके में { $option }
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-secret-task-definition-unreadable = कार्य { $task } की परिभाषा, जिसे जाँचने के लिए पढ़ा नहीं जा सका
# { $name } is an environment variable's name, such as APP_TOKEN.
core-secret-left-out-variable = परिवेश चर { $name }
# { $class } is a COM class's id, such as {00000000-0000-0000-0000-000000000000}.
core-secret-left-out-class-server = वह चीज़ जिसे क्लास { $class } चलाने की बात कहती है
# { $rule } is a firewall rule's name.
core-secret-left-out-firewall-description = फ़ायरवॉल नियम { $rule } का विवरण

## Licenses a scan found, as the package's report describes each.

# { $name } is a file's name, such as license.lic.
core-license-detail-file = लाइसेंस फ़ाइल मिली: { $name }
# { $framework } is a licensing product's name, such as FlexLM / FlexNet.
core-license-detail-store = { $framework } लाइसेंस स्टोर मौजूद है
# { $name } is an environment variable's name, such as LM_LICENSE_FILE.
core-license-detail-variable = लाइसेंस परिवेश चर { $name }
# { $name } is a registry value's name, such as SerialNumber.
core-license-detail-registry-value = रजिस्ट्री मान से लाइसेंसिंग डेटा का संकेत मिलता है: { $name }
# Said of a registry value in an account's key, which is not named.
core-license-detail-registry-value-of-an-account = रजिस्ट्री मान से लाइसेंसिंग डेटा का संकेत मिलता है
# { $key } is a registry key, such as HKLM\Software\Contoso\License.
core-license-detail-registry-key = रजिस्ट्री कुंजी से लाइसेंसिंग डेटा का संकेत मिलता है: { $key }

## Warnings a scan records about licenses.

core-warning-scan-license-data = लाइसेंसिंग/सक्रियण डेटा मिला। "लाइसेंस और सक्रियण डेटा" विकल्प चालू होने पर, इंस्टॉलर वह पुनर्स्थापित करता है जिसे ले जाया जा सकता है; मशीन-व्यापी मानों और हार्डवेयर से बंधे स्टोर की केवल रिपोर्ट की जाती है। WinAppMigrator कभी भी डिक्रिप्ट नहीं करता, सक्रियण को बायपास नहीं करता, या डोंगल का अनुकरण नहीं करता।
core-warning-scan-license-machine-bound = कुछ लाइसेंस मशीन से बंधे हैं (हार्डवेयर डोंगल, मशीन-लॉक सक्रियण, या एन्क्रिप्ट किए गए)। इंस्टॉलर जो कर सकता है, वह पुनर्स्थापित करता है; हो सकता है कि इन्हें नए PC पर फिर से सक्रिय करना पड़े।

## Warnings a scan records about the registry. What could not be read is not named in them: it is kept beside them.

# { $error } is what Windows said went wrong. App Paths is a registry key's name.
core-warning-scan-app-paths-key-unreadable = App Paths की चाइल्ड कुंजी पढ़ी नहीं जा सकी: { $error }
core-warning-scan-registry-limit = रजिस्ट्री कैप्चर कॉन्फ़िगर की गई सुरक्षा सीमा तक पहुँच गया।
core-warning-scan-registry-subtree-limit = रजिस्ट्री सबट्री कैप्चर कॉन्फ़िगर की गई सुरक्षा सीमा तक पहुँच गया।
# { $error } is what Windows said went wrong.
core-warning-scan-registry-key-unreadable = रजिस्ट्री कुंजी पढ़ी नहीं जा सकी: { $error }
core-warning-scan-registry-value-too-large = रजिस्ट्री मान कॉन्फ़िगर की गई कैप्चर आकार सीमा से अधिक था।

## Scheduled tasks

core-warning-scan-tasks-folder-through-link = इस ड्राइव पर शेड्यूल किए गए कार्यों का फ़ोल्डर किसी लिंक से होकर जाता है, इसलिए इसके कार्य पढ़े नहीं गए।
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-warning-scan-task-left-out = शेड्यूल किया गया कार्य { $task } छोड़ा गया: यह उस खाते के अलावा किसी अन्य खाते के रूप में चलता है जिसकी सेटिंग्स शामिल की जा रही हैं, या व्यवस्थापकों के अलावा अन्य लोग इसकी फ़ाइल बदल सकते हैं, या ऐप के पास रखने के लिए बहुत अधिक कार्य हैं।
# { $error } is what Windows said went wrong.
core-warning-scan-tasks-folder-unreadable = शेड्यूल किए गए कार्यों का एक भाग पढ़ा नहीं जा सका: { $error }
# { $path } is a task's file, such as C:\Windows\System32\Tasks\App\Nightly Sync; { $reason } is one of the reasons
# below, or what Windows said went wrong.
core-warning-scan-task-unreadable = शेड्यूल किया गया कार्य { $path } पढ़ा नहीं जा सका: { $reason }। अगर यह ऐप चलाता है, तो इसे शामिल नहीं किया गया।
# Why a task's file could not be read. { $size } is its size in bytes.
core-task-too-large = यह { $size } बाइट का है, जो किसी कार्य के लिए बहुत बड़ा है
# Why a task's file could not be read.
core-task-ends-mid-character = यह किसी वर्ण के बीच में समाप्त होता है
# Why a task's file could not be read. { $error } says where.
core-task-not-utf16 = यह मान्य UTF-16 नहीं है: { $error }
# Why a task's file could not be read. { $error } says where.
core-task-not-text = यह मान्य UTF-8 या UTF-16 नहीं है: { $error }
# What a package records of how a task is put back. Register-ScheduledTask is a command's name.
core-task-restore-instructions = पुनर्स्थापना संग्रहीत XML परिभाषा के साथ Register-ScheduledTask का उपयोग करती है।

## Reading a program's file. Each is why it could not be read, after the program's name.

# { $reason } is what went wrong.
core-pe-unreadable = इसे पढ़ा नहीं जा सका: { $reason }
core-pe-headers-misaligned = इसके हेडर उन सीमाओं पर नहीं हैं जिन पर किसी प्रोग्राम के हेडर होने चाहिए
# { $size } is the file's size in bytes.
core-pe-too-large = यह { $size } बाइट का है, जो किसी प्रोग्राम के लिए बहुत बड़ा है
# PE is the name of the format Windows programs are in.
core-pe-no-headers = इसमें PE हेडर नहीं हैं

## Warnings a scan records about what a program needs. Which program is kept beside each.

# { $error } is what went wrong.
core-warning-scan-imports-unreadable = इंपोर्ट तालिका पढ़ी नहीं जा सकी: { $error }
# { $error } is what went wrong.
core-warning-scan-import-name-unreadable = किसी इंपोर्ट डिस्क्रिप्टर का नाम पढ़ा नहीं जा सका: { $error }
# { $count } is how many were skipped.
core-warning-scan-legacy-delay-imports =
    { $count ->
        [one] { $count } पुराना (v1) डिले-लोड डिस्क्रिप्टर छोड़ा गया; केवल आधुनिक RVA-आधारित डिले इंपोर्ट ही पार्स किए जाते हैं।
       *[other] { $count } पुराने (v1) डिले-लोड डिस्क्रिप्टर छोड़े गए; केवल आधुनिक RVA-आधारित डिले इंपोर्ट ही पार्स किए जाते हैं।
    }

## The DLLs a program needs. Which DLL or program is kept beside each warning.

# { $path } is a program, such as C:\Program Files\App\app.exe.
core-dll-program-has-no-folder = निष्पादन योग्य फ़ाइल का कोई पैरेंट फ़ोल्डर नहीं है: { $path }
# Why a DLL is among the app's files, in the package's list of them.
core-dll-beside-program = चुनी गई निष्पादन योग्य फ़ाइल के बगल में मिली DLL
# { $error } is what went wrong.
core-warning-scan-dependency-unscanned = पुनरावर्ती निर्भरता को स्कैन नहीं किया जा सका: { $error }
# { $name } is a DLL's name, such as d3dx9_43.dll.
core-warning-scan-dependency-not-in-newer-windows = { $name } उस PC के Windows फ़ोल्डर में थी जिस पर यह ऐप था, और Windows 10 और 11 के साथ यह नहीं आती। इसे किसी और चीज़ ने वहाँ इंस्टॉल किया था, इसलिए यह ऐप की अपनी फ़ाइलों का भाग नहीं है और साथ नहीं आती। हो सकता है कि जब तक इसे उपलब्ध कराने वाली चीज़ इंस्टॉल न हो, तब तक ऐप प्रारंभ न हो।
# { $error } is what went wrong.
core-warning-scan-dependency-not-hashed = निर्भरता का हैश नहीं बनाया जा सका: { $error }
# SystemRoot and windir are the names of environment variables.
core-warning-scan-windows-folder-unknown = SystemRoot/windir परिवेश चर उपलब्ध नहीं हैं; सिस्टम DLL की खोज सीमित है।
# { $error } is what went wrong.
core-warning-scan-dependency-architecture-unreadable = निर्भरता का आर्किटेक्चर पढ़ा नहीं जा सका: { $error }

## Services, drivers, shortcuts, and environment variables a scan reads. Where it was reading is kept beside each
## warning.

# { $error } is what Windows said went wrong.
core-warning-scan-services-unreadable = सेवाओं की रजिस्ट्री कुंजी पढ़ी नहीं जा सकी: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-drivers-unreadable = ड्राइवर खोजने के लिए सेवाओं की रजिस्ट्री कुंजी पढ़ी नहीं जा सकी: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-queried = शॉर्टकट की क्वेरी नहीं की जा सकी: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-parsed = शॉर्टकट क्वेरी का आउटपुट पार्स नहीं किया जा सका: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-environment-unreadable = मशीन परिवेश की रजिस्ट्री कुंजी पढ़ी नहीं जा सकी: { $error }

## What a package records of how a driver is put back, which an installer shows. { $kind } is the driver's type as
## Windows names it (kernel, file-system, recognizer, or unknown-type); { $name } is its service's name; { $path } is
## its file. { $signed } is yes when { $signer } names who signed it, and no when that is not known.

core-service-driver-bundled =
    { $signed ->
        [yes] एप्लिकेशन का अपना { $kind } ड्राइवर '{ $name }' ({ $path }, { $signer } द्वारा हस्ताक्षरित)। इंस्टॉलर इसकी .sys फ़ाइल बंडल करता है और ड्राइवर सेवा को स्वचालित रूप से फिर से पंजीकृत करता है (sc create, व्यवस्थापक अधिकार आवश्यक), फिर पुष्टि करता है कि सेवा पंजीकृत है।
       *[no] एप्लिकेशन का अपना { $kind } ड्राइवर '{ $name }' ({ $path }, हस्ताक्षर अज्ञात)। इंस्टॉलर इसकी .sys फ़ाइल बंडल करता है और ड्राइवर सेवा को स्वचालित रूप से फिर से पंजीकृत करता है (sc create, व्यवस्थापक अधिकार आवश्यक), फिर पुष्टि करता है कि सेवा पंजीकृत है।
    }
core-service-driver-from-vendor =
    { $signed ->
        [yes] विक्रेता/इनबॉक्स { $kind } ड्राइवर '{ $name }' ({ $path }, { $signer } द्वारा हस्ताक्षरित)। WinAppMigrator तृतीय-पक्ष ड्राइवरों को फिर से वितरित नहीं करता; ऐप लॉन्च करने से पहले इसे लक्ष्य PC पर विक्रेता के हस्ताक्षरित ड्राइवर पैकेज से फिर से इंस्टॉल करें।
       *[no] विक्रेता/इनबॉक्स { $kind } ड्राइवर '{ $name }' ({ $path }, हस्ताक्षर अज्ञात)। WinAppMigrator तृतीय-पक्ष ड्राइवरों को फिर से वितरित नहीं करता; ऐप लॉन्च करने से पहले इसे लक्ष्य PC पर विक्रेता के हस्ताक्षरित ड्राइवर पैकेज से फिर से इंस्टॉल करें।
    }

## Runtimes an app needs, such as the Visual C++ runtime or .NET: how each is provided, as the Dependencies list says.

core-runtime-handling-built-in = Windows में अंतर्निहित
core-runtime-handling-ships-with-app = ऐप के साथ आता है
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-handling-windows-feature = Windows सुविधा: { $feature }
core-runtime-handling-included = शामिल
# { $id } is the runtime's name in winget, such as Microsoft.VCRedist.2015+.x64.
core-runtime-handling-winget = winget से इंस्टॉल किया जाता है: { $id }
core-runtime-handling-manual = मैन्युअल रूप से इंस्टॉल करें

## Downloading a runtime's installer, when an installer is built. Each is why it could not be, and starts with a small
## letter, as it follows other words. { $url } is a web address; { $error } is what went wrong.

core-runtime-download-not-https = { $url } डाउनलोड करने से इनकार किया जा रहा है: यह HTTPS नहीं है
# TLS is what keeps a download private.
core-runtime-tls-failed = TLS प्रारंभ करना विफल रहा: { $error }
core-runtime-download-failed = { $url } डाउनलोड करना विफल रहा: { $error }
core-runtime-download-unreadable = { $url } पढ़ना विफल रहा: { $error }
core-runtime-download-empty = { $url } के डाउनलोड से कोई डेटा नहीं मिला
# { $came } and { $expected } are numbers of bytes.
core-runtime-download-cut-short = { $url } का डाउनलोड बीच में ही रुक गया: { $expected } में से { $came } बाइट आए

## Installing a runtime: why it was not installed. Each starts with a small letter, as it follows other words.

core-runtime-no-known-installer = इस रनटाइम के लिए कोई ज्ञात इंस्टॉलर नहीं है
# { $path } is where in the package, such as prerequisites\vc_redist.x64.exe.
core-runtime-payload-misplaced = पैकेज { $path } को इस रनटाइम का इंस्टॉलर बताता है, जो वहाँ नहीं है जहाँ उसे होना चाहिए, इसलिए इसे नहीं चलाया गया
# { $path } is a file.
core-runtime-payload-missing = बंडल किया गया इंस्टॉलर अनुपलब्ध है: { $path }
# { $error } is what went wrong.
core-runtime-payload-path-invalid = बंडल किए गए इंस्टॉलर का पथ अमान्य है: { $error }
# { $url } is the web address it can be downloaded from. winget is a program's name.
core-runtime-no-installer-route = कोई बंडल किया गया इंस्टॉलर या winget ID नहीं है; इसे { $url } से प्राप्त करें
# { $error } is what went wrong.
core-runtime-no-temp-folder = इसे अनपैक करने के लिए फ़ोल्डर नहीं बनाया जा सका: { $error }
# { $reason } is what the check found.
core-runtime-payload-signature-failed = बंडल किया गया इंस्टॉलर अपनी हस्ताक्षर जाँच में विफल रहा और उसे नहीं चलाया गया: { $reason }
core-runtime-payload-timed-out = बंडल किया गया इंस्टॉलर समय सीमा के भीतर पूरा नहीं हुआ और उसे समाप्त कर दिया गया
# { $error } is what went wrong.
core-runtime-payload-not-started = बंडल किया गया इंस्टॉलर प्रारंभ नहीं किया जा सका: { $error }
core-runtime-installer-killed = इंस्टॉलर को बिना निकास कोड के समाप्त कर दिया गया
# { $code } is the number it ended with, such as 1603.
core-runtime-installer-failed = इंस्टॉलर निकास कोड { $code } के साथ बंद हुआ
# { $feature } is a Windows feature's name, such as NetFx3; { $code } is the number DISM ended with. Windows Features is
# the name of a Windows window. Keep the command as it is.
core-runtime-feature-failed = DISM { $feature } Windows सुविधा को सक्षम नहीं कर सका (निकास कोड { $code }); संभवतः पेलोड डाउनलोड नहीं किया जा सका। इसे "Windows सुविधाएँ" से सक्षम करें, या `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess` चलाएँ।
# The same, when DISM gave no number.
core-runtime-feature-failed-no-code = DISM { $feature } Windows सुविधा को सक्षम नहीं कर सका (निकास कोड अज्ञात); संभवतः पेलोड डाउनलोड नहीं किया जा सका। इसे "Windows सुविधाएँ" से सक्षम करें, या `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess` चलाएँ।
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-feature-timed-out = { $feature } Windows सुविधा को सक्षम करना समय सीमा के भीतर पूरा नहीं हुआ और उसे समाप्त कर दिया गया
# { $feature } is a Windows feature's name; { $error } is what went wrong.
core-runtime-feature-no-dism = { $feature } Windows सुविधा को सक्षम करने के लिए DISM उपलब्ध नहीं है: { $error }

## Installing a runtime with winget. { $id } is its name in winget, such as Microsoft.VCRedist.2015+.x64.

# { $error } is what went wrong.
core-runtime-winget-not-looked-for = winget को खोजा नहीं जा सका: { $error }
core-runtime-winget-not-installed = winget इंस्टॉल नहीं है
# { $path } is where winget is.
core-runtime-winget-misplaced = { $path } वह स्थान नहीं है जहाँ Windows अपने पैकेज रखता है, इसलिए इसे नहीं चलाया गया
# { $path } is where winget is; { $reason } is what the check found.
core-runtime-winget-signature-failed = { $path } अपनी हस्ताक्षर जाँच में विफल रहा और उसे नहीं चलाया गया: { $reason }
# { $reason } is one of the two messages above, or what went wrong.
core-runtime-winget-unusable = { $id } इंस्टॉल करने के लिए winget का उपयोग नहीं किया जा सका: { $reason }
# { $code } is the number winget ended with.
core-runtime-winget-failed = winget install { $id } निकास कोड { $code } के साथ बंद हुआ
core-runtime-winget-failed-no-code = winget install { $id } किसी अज्ञात निकास कोड के साथ बंद हुआ
core-runtime-winget-timed-out = winget install { $id } समय सीमा के भीतर पूरा नहीं हुआ और उसे समाप्त कर दिया गया
# { $error } is what went wrong.
core-runtime-winget-not-started = { $id } इंस्टॉल करने के लिए winget उपलब्ध नहीं है: { $error }

## What told a scan how an app was installed, as the package's report lists it. The names of installers and files are
## as their makers write them.

# { $path } is an installer's record of what it installed, such as C:\Program Files\App\unins000.dat.
core-source-record-not-believed = { $path } का उपयोग नहीं किया गया: व्यवस्थापकों के अलावा अन्य लोग इसे बदल सकते हैं
# { $count } is how many Windows Installer packages the bundle holds.
core-source-burn-bundle =
    { $count ->
        [one] { $count } Windows Installer पैकेज का Burn बंडल
       *[other] { $count } Windows Installer पैकेजों का Burn बंडल
    }
# { $count } is how many paths Windows Installer recorded.
core-source-msi-database =
    { $count ->
        [one] Windows Installer डेटाबेस: { $count } घटक कुंजी पथ
       *[other] Windows Installer डेटाबेस: { $count } घटक कुंजी पथ
    }
core-source-inno-log = Inno Setup अनइंस्टॉल लॉग (unins000.dat)
# { $count } is how many paths the log records.
core-source-inno-log-paths =
    { $count ->
        [one] Inno Setup अनइंस्टॉल लॉग: { $count } रिकॉर्ड किया गया पथ
       *[other] Inno Setup अनइंस्टॉल लॉग: { $count } रिकॉर्ड किए गए पथ
    }
core-source-nsis = NSIS (Nullsoft) अनइंस्टॉलर हस्ताक्षर
# { $count } is how many paths the log records.
core-source-nsis-log-paths =
    { $count ->
        [one] NSIS AdvUninstLog uninstall.log: { $count } रिकॉर्ड किया गया पथ
       *[other] NSIS AdvUninstLog uninstall.log: { $count } रिकॉर्ड किए गए पथ
    }
core-source-squirrel = Squirrel प्रति-उपयोगकर्ता लेआउट (Update.exe / app-*)
# { $version } is a folder's name that says the version, such as app-1.2.3.
core-source-squirrel-version = Squirrel वर्तमान संस्करण: { $version }
core-source-chocolatey = Chocolatey पैकेज मेटाडेटा
core-source-msix = MSIX / Appx पैकेज
# { $identity } is a package's name, such as Contoso.App.
core-source-appx-identity = AppxManifest पहचान: { $identity }
# { $count } is how many folders.
core-source-msix-data =
    { $count ->
        [one] MSIX प्रति-उपयोगकर्ता पैकेज डेटा: { $count } स्थान
       *[other] MSIX प्रति-उपयोगकर्ता पैकेज डेटा: { $count } स्थान
    }
# App Paths is a registry key's name.
core-source-app-paths = App Paths पंजीकरण

## Why a runtime is needed, as the package's report says it.

# { $program } is a program's file name, such as app.exe; { $dll } is a DLL's, such as msvcp140.dll.
core-runtime-program-imports = { $program } { $dll } को इंपोर्ट करता है

## What else of an app's a scan finds in the registry: fonts and certificates, and why each is the app's.

# Why a font is among the app's files, in the package's list of them.
core-registry-font-recorded = ऐसा फ़ॉन्ट जिसका नाम ऐप का इंस्टॉल रिकॉर्ड लेता है
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-named-for-the-app = इसका नाम ऐप या उसके प्रकाशक के नाम पर है
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-recorded = ऐप का इंस्टॉल रिकॉर्ड इसका नाम लेता है

## Who is signed in, for the Whose Data choice. Each of the first six is why it could not be told, and starts with a
## small letter, as it is shown inside other words. An account is shown by its id, such as S-1-5-21-...-1001.

# { $error } is what Windows said.
core-profile-desktop-owner-not-said = Windows ने यह नहीं बताया कि यह डेस्कटॉप किसका है: { $error }
# { $error } is what went wrong.
core-profile-desktop-owner-not-asked = Windows से यह नहीं पूछा जा सका कि यह डेस्कटॉप किसका है: { $error }
core-profile-signed-in-account-no-profile = यहाँ साइन इन खाते, { $account }, की कोई प्रोफ़ाइल रिकॉर्ड में नहीं है
core-profile-desktop-not-a-person = यहाँ का डेस्कटॉप { $account } का है, जो किसी व्यक्ति का खाता नहीं है
core-profile-nobody-signed-in-running-account-no-profile = यहाँ कोई साइन इन नहीं है, और इसे चलाने वाले खाते, { $account }, की कोई प्रोफ़ाइल रिकॉर्ड में नहीं है
core-profile-nobody-signed-in-running-not-a-person = यहाँ कोई साइन इन नहीं है, और { $account }, जो इसे चला रहा है, किसी व्यक्ति का खाता नहीं है
# SID is the name Windows gives an account's id.
core-profile-nobody-signed-in-no-sid = यहाँ कोई साइन इन नहीं है, और इस खाते की कोई SID नहीं है
# { $accounts } is a list of accounts.
core-profile-several-desktops = इस सत्र में एक से अधिक खातों के डेस्कटॉप हैं: { $accounts }
# Under the Whose Data choice. { $reason } is one of the messages above.
core-profile-signed-in-unknown = यह पता नहीं चल सका कि कौन साइन इन है ({ $reason })।
# Under the Whose Data choice, when there are accounts to choose from. { $reason } is one of the messages above.
core-profile-signed-in-unknown-choose = यह पता नहीं चल सका कि कौन साइन इन है ({ $reason })। चुनें कि किसका डेटा शामिल करना है।
