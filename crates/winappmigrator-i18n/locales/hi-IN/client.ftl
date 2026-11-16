# Words of WinAppMigrator Client, the console program run on the old PC, the one an app is moving off, so that
# WinAppMigrator on the new PC can read it over the network. Message names start with client-.
#
# Its window is a console, which on Windows 7 is 80 columns wide. Each line break in a message is one on the screen:
# keep every line shorter than 80 characters, and break lines where your language needs to, not only where English
# does.


## Starting

# Shown only in English, in place of a right-to-left language the console cannot show. { $language } is that
# language's name in English, such as Arabic.
client-console-rtl-fallback = यह कंसोल { $language } नहीं दिखा सकता, इसलिए क्लाइंट अंग्रेज़ी में बोलता है।
# The client copies itself to a folder in Program Files before it starts. { $error } is what Windows said went wrong.
client-safe-folder-failed =
    WinAppMigrator Client ऐसे फ़ोल्डर से प्रारंभ नहीं हो सका जिसे केवल
    व्यवस्थापक बदल सकते हैं: { $error }
client-press-enter-to-close = यह विंडो बंद करने के लिए Enter दबाएँ।
# The first line of its screen. { $version } is the client's version, such as 0.6.2.
client-heading = WinAppMigrator Client { $version }
client-intro =
    इससे आपके नए PC पर WinAppMigrator आपके नेटवर्क के ज़रिए इस PC के
    ऐप्स पढ़ सकता है। यह यहाँ कुछ भी नहीं बदलता: यह Windows ड्राइव का
    एक स्नैपशॉट पढ़ता है, जिसे आपके इसे रोकने पर यह हटा देता है।
# { $error } is why: one of the messages under "Why it stopped" below, or what Windows said.
client-stopped = क्लाइंट रुक गया: { $error }
# The caretaker is a copy of the client with no window that cleans up after it. { $error } is what Windows said.
client-caretaker-failed =
    अगर यह विंडो रोकने के बजाय बंद की जाती है, तो स्नैपशॉट और फ़ायरवॉल
    नियम तब तक बने रहते हैं जब तक क्लाइंट अगली बार यहाँ नहीं चलाया जाता
    ({ $error })।
# Shown while the snapshot is taken, followed on the same line by client-snapshot-taken. Keep the quotes: they keep
# the space before it.
client-snapshot-taking = Windows ड्राइव का स्नैपशॉट लिया जा रहा है...{ " " }
client-snapshot-taken = हो गया।
# The old PC answers the new one's search for it on the network. { $error } is what Windows said went wrong.
client-discovery-failed =
    यह PC अपने आप नहीं दिखेगा ({ $error })।
    इसके बजाय, नए PC पर इस PC का नाम या पता टाइप करें।

## This PC and its pairing code. The three lines are lined up in two columns, behind two spaces; keep the quotes, which
## keep those spaces.

# { $computer_name } is this PC's name, such as OLD-PC.
client-this-pc = { "  " }यह PC:         { $computer_name }
# { $windows_name } is this PC's Windows, such as Windows 7 Professional Service Pack 1. { $bits } is 32 or 64.
client-windows = { "  " }Windows:       { $windows_name }, { $bits }-बिट
# { $code } is six digits in two groups, such as 482 913.
client-pairing-code = { "  " }युग्मन कोड:      { $code }
# Network PC... is the button of that name in WinAppMigrator: use the name it has there in your language.
client-pairing-steps =
    अपने नए PC पर, WinAppMigrator में, नेटवर्क PC... चुनें और यह कोड
    टाइप करें। जिस ऐप को आप ले जा रहे हैं, उसे पहले बंद करें, ताकि उसकी
    फ़ाइलें वैसी ही रहें जैसी उसने छोड़ी थीं।
    रोकने के लिए, यहाँ Enter दबाएँ या यह विंडो बंद करें।

## While it runs. { $from } is the network address of the PC that connected, such as 192.168.1.20, or "unknown" when
## it could not be told, which the [unknown] variant words.

client-paired =
    { $from ->
        [unknown] किसी अन्य PC
       *[address] { $from }
    } के साथ युग्मित हो गया। वह जो चाहिए उसे पढ़ रहा है...
# { $count } is how many requests it answered.
client-builder-done =
    { $from ->
        [unknown] किसी अन्य PC
       *[address] { $from }
    } का काम पूरा हो गया ({ $count ->
        [one] { $count } अनुरोध
       *[other] { $count } अनुरोधों
    } का उत्तर दिया गया)।
# { $error } is what went wrong, often in Windows' words.
client-link-ended =
    { $from ->
        [unknown] किसी अन्य PC
       *[address] { $from }
    } से कनेक्शन समाप्त हो गया: { $error }
client-wrong-code =
    { $from ->
        [unknown] किसी अन्य PC
       *[address] { $from }
    } ने ऐसा कोड आज़माया जो इस PC का कोड नहीं है।
# { $error } is what went wrong, often in Windows' words.
client-pair-failed =
    { $from ->
        [unknown] किसी अन्य PC
       *[address] { $from }
    } से युग्मन नहीं हो सका: { $error }
client-too-many-wrong-codes =
    बहुत अधिक गलत कोड आज़माए गए, इसलिए अब कोई भी PC इससे युग्मित नहीं
    हो सकता। हो सकता है कि आपके नेटवर्क पर कोई कोड का अनुमान लगा रहा हो।
    युग्मित करने के लिए, क्लाइंट को रोकें और फिर से प्रारंभ करें, और वह
    नया कोड टाइप करें जो यह दिखाता है।
# { $code } is six digits in two groups, such as 482 913.
client-new-code = बहुत अधिक गलत कोड। नया युग्मन कोड है: { $code }

## Stopping

client-stopping = रोका जा रहा है: स्नैपशॉट और फ़ायरवॉल नियम हटाए जा रहे हैं...
client-stop-done = हो गया।

## Why it stopped, shown after "The client stopped: " (client-stopped), and written to follow it.

client-already-running = क्लाइंट इस PC पर पहले से चल रहा है, या बंद किए गए किसी क्लाइंट के बाद सफ़ाई कर रहा है। उसी का उपयोग करें, या थोड़ी देर प्रतीक्षा करें और इसे फिर से प्रारंभ करें।
# { $drive } is the drive Windows is on, such as C:. { $details } is what Windows said, in its own words.
client-snapshot-failed = Windows ने { $drive } का स्नैपशॉट नहीं बनाया: { $details }
# { $details } is what Windows Firewall said, in its own words.
client-firewall-failed = Windows फ़ायरवॉल ने क्लाइंट को कनेक्शन सुनने नहीं दिया: { $details }

## What it takes, shown for --help. The options, such as --lang, are typed as they are and are not translated; keep
## their descriptions lined up.

client-usage =
    WinAppMigrator Client ये विकल्प लेता है:
    
      --lang <language>  Windows की प्रदर्शन भाषा के बजाय यह भाषा बोलें,
                         जैसे it-IT, अगर क्लाइंट में यह भाषा है।
      --code <code>      इस छह-अंकीय युग्मन कोड से प्रारंभ करें, ऐसी
                         स्क्रिप्ट के लिए जो नए PC पर WinAppMigrator भी
                         प्रारंभ करती है।
      --help             यह जानकारी दिखाएँ।
