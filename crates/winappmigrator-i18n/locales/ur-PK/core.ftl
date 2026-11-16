# Words the core writes for people: warnings, refusals, errors, and the installer's list of what it sets up.
# Message names start with core-.


## Warnings a package records when it is built, which an installer shows later, maybe on another PC, in that PC's
## language.

# { $count } is how many shortcuts.
core-warning-shortcuts-unreadable =
    { $count ->
        [one] اس ڈرائیو پر { $count } شارٹ کٹ کسی فائل کا نام نہیں لیتا، اس لیے یہ معلوم نہیں ہو سکا کہ آیا وہ اس ایپ کو کھولتا ہے۔
       *[other] اس ڈرائیو پر { $count } شارٹ کٹس کسی فائل کا نام نہیں لیتے، اس لیے یہ معلوم نہیں ہو سکا کہ آیا وہ اس ایپ کو کھولتے ہیں۔
    }

## Undoing an install: what an uninstall, or a failed install, says it put back, left, or could not undo. Each line is
## shown on its own, in a list.

# { $app_name } is the app's name.
core-rollback-record-damaged = { $app_name } کے ریکارڈ کا کچھ حصہ پڑھا نہیں جا سکا، اس لیے اس میں بیان کردہ ہر چیز ابھی تک اپنی جگہ موجود ہے، اور ہو سکتا ہے جس فائل کی جگہ اس نے لی تھی وہ واپس نہ آئے۔
core-rollback-settings-entry-stays = ترتیبات میں ایپ کا اندراج باقی رہتا ہے، تاکہ اسے وہاں سے دوبارہ چلایا جا سکے۔
# { $name } is a runtime's name, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-rollback-prerequisite-stays = { $name } انسٹال کیا گیا تھا، یا شاید کیا گیا ہو، اور یہ باقی رہتا ہے، کیونکہ ہو سکتا ہے دوسرے پروگرام اسے استعمال کر رہے ہوں۔
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-drive-not-connected = اس تنصیب نے { $folder } میں جو کچھ رکھا وہ ایسی ڈرائیو پر ہے جو منسلک نہیں ہے۔ اسے منسلک کریں، تو یہ کام مکمل کیا جا سکتا ہے۔
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-record-part-unread = اس تنصیب نے { $folder } میں جو کچھ رکھا وہ باقی رہتا ہے، کیونکہ ریکارڈ کا وہ حصہ پڑھا نہیں جا سکا۔

## Why a file or folder was left, shown after "was left:" in the messages below them. Each is part of a sentence, so
## starts with a small letter and has no full stop.

# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-being-changed = اس کے چلنے کے دوران کوئی چیز { $folder } کو تبدیل کر رہی تھی، اس لیے اس میں مزید کچھ نہیں کیا گیا۔ جو چیز ایسا کر رہی ہے اسے بند کریں، اور یہ دوبارہ چلائیں
# { $folder } is a folder; { $error } is what Windows said went wrong.
core-rollback-reason-not-opened = { $folder } کھولا نہیں جا سکا: { $error }
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-no-record = اس کا کوئی ریکارڈ نہیں کہ { $folder } کہاں تھا
# { $folder } is a folder; { $actual } is the folder it now leads to.
core-rollback-reason-moved = { $folder } اب { $actual } کی طرف لے جاتا ہے، نہ کہ وہاں جہاں یہ تنصیب کے دوران تھا
# Why a service was not stopped.
core-rollback-reason-did-not-stop = یہ وقت پر نہیں رکی

## Files and folders

# { $path } is a folder; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-folder-left = فولڈر { $path } چھوڑ دیا گیا: { $reason }۔
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-removed = فولڈر { $path } ہٹا دیا گیا
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-holds-others = فولڈر { $path } باقی رہتا ہے، کیونکہ اس میں ایسی فائلیں ہیں جو اس تنصیب نے وہاں نہیں رکھیں۔
# { $path } is a folder; { $error } is what Windows said went wrong.
core-rollback-folder-not-removed = فولڈر { $path } ہٹایا نہیں جا سکا: { $error }۔
# { $path } is a file; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-file-left = فائل { $path } چھوڑ دی گئی: { $reason }۔
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-folder-gone = { $path } واپس نہیں رکھا جا رہا، کیونکہ جس فولڈر کے تحت یہ تھا وہ اب موجود نہیں ہے۔
# { $path } is a file; { $error } is what went wrong.
core-rollback-partial-not-removed = ایک نامکمل فائل ہٹائی نہیں جا سکی: { $path } ({ $error })۔
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-checked = { $path } کی جانچ نہیں کی جا سکی: { $error }۔
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-removed = { $path } ہٹا دیا گیا
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-removed = { $path } ہٹایا نہیں جا سکا: { $error }۔
# { $path } is a file this install had replaced, and has put back.
core-rollback-file-put-back = { $path } واپس رکھ دیا گیا
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-put-back = { $path } واپس نہیں رکھا جا سکا: { $error }۔
# Why a file could not be put back.
core-rollback-backup-gone = بیک اپ اب موجود نہیں ہے
# { $path } is a file.
core-rollback-no-parent-folder = { $path } کا کوئی پیرنٹ فولڈر نہیں ہے

## A file that stays. { $why } says why: still-used (something that stays still uses it), changed (it has changed since
## it was installed), or not-asked (it is someone's data, and they did not ask for it to go). { $path } is the file.

core-rollback-file-stays =
    { $why ->
        [still-used] { $path } باقی رہتا ہے، کیونکہ باقی رہنے والی کوئی چیز اب بھی اسے استعمال کرتی ہے۔
        [changed] { $path } باقی رہتا ہے، کیونکہ انسٹال ہونے کے بعد سے یہ تبدیل ہو چکا ہے۔
       *[not-asked] { $path } باقی رہتا ہے، کیونکہ اسے ہٹانے کے لیے نہیں کہا گیا تھا۔
    }
# The file it replaced was to be put beside it, under another name, and every name was taken.
core-rollback-file-stays-no-name-beside =
    { $why ->
        [still-used] { $path } باقی رہتا ہے، کیونکہ باقی رہنے والی کوئی چیز اب بھی اسے استعمال کرتی ہے، اور جس فائل کی جگہ اس نے لی تھی اسے اس کے ساتھ نہیں رکھا جا سکا: اس کے لیے ہر نام پہلے سے استعمال میں ہے۔
        [changed] { $path } باقی رہتا ہے، کیونکہ انسٹال ہونے کے بعد سے یہ تبدیل ہو چکا ہے، اور جس فائل کی جگہ اس نے لی تھی اسے اس کے ساتھ نہیں رکھا جا سکا: اس کے لیے ہر نام پہلے سے استعمال میں ہے۔
       *[not-asked] { $path } باقی رہتا ہے، کیونکہ اسے ہٹانے کے لیے نہیں کہا گیا تھا، اور جس فائل کی جگہ اس نے لی تھی اسے اس کے ساتھ نہیں رکھا جا سکا: اس کے لیے ہر نام پہلے سے استعمال میں ہے۔
    }
# { $name } is the name the file it replaced now has, such as settings.ini.before-install.
core-rollback-file-stays-beside =
    { $why ->
        [still-used] { $path } باقی رہتا ہے، کیونکہ باقی رہنے والی کوئی چیز اب بھی اسے استعمال کرتی ہے۔ جس فائل کی جگہ اس نے لی تھی وہ اس کے ساتھ { $name } کے نام سے موجود ہے۔
        [changed] { $path } باقی رہتا ہے، کیونکہ انسٹال ہونے کے بعد سے یہ تبدیل ہو چکا ہے۔ جس فائل کی جگہ اس نے لی تھی وہ اس کے ساتھ { $name } کے نام سے موجود ہے۔
       *[not-asked] { $path } باقی رہتا ہے، کیونکہ اسے ہٹانے کے لیے نہیں کہا گیا تھا۔ جس فائل کی جگہ اس نے لی تھی وہ اس کے ساتھ { $name } کے نام سے موجود ہے۔
    }
# { $error } is what went wrong.
core-rollback-file-stays-not-beside =
    { $why ->
        [still-used] { $path } باقی رہتا ہے، کیونکہ باقی رہنے والی کوئی چیز اب بھی اسے استعمال کرتی ہے، اور جس فائل کی جگہ اس نے لی تھی اسے اس کے ساتھ نہیں رکھا جا سکا: { $error }۔
        [changed] { $path } باقی رہتا ہے، کیونکہ انسٹال ہونے کے بعد سے یہ تبدیل ہو چکا ہے، اور جس فائل کی جگہ اس نے لی تھی اسے اس کے ساتھ نہیں رکھا جا سکا: { $error }۔
       *[not-asked] { $path } باقی رہتا ہے، کیونکہ اسے ہٹانے کے لیے نہیں کہا گیا تھا، اور جس فائل کی جگہ اس نے لی تھی اسے اس کے ساتھ نہیں رکھا جا سکا: { $error }۔
    }

## The registry and environment variables. { $kind } says what { $name } is: key (a registry key, such as
## HKLM\Software\App), value (a registry value, such as HKLM\Software\App\Version), user-variable or machine-variable
## (an environment variable of the user's or of the whole PC, such as APP_HOME), or user-list or machine-list (a
## variable that is a list of folders, such as Path). { $error } is what Windows said went wrong.

# How a registry key's unnamed value is shown, as Registry Editor shows it.
core-registry-default-value-name = (ڈیفالٹ)
core-rollback-setting-other-account =
    { $kind ->
        [key] کلید { $name } باقی رہتی ہے، کیونکہ یہ اس اکاؤنٹ کی ہے جس نے اسے انسٹال کیا تھا، اور وہ سائن ان نہیں ہے۔
        [user-variable] صارف کا ماحولیاتی متغیر { $name } باقی رہتا ہے، کیونکہ یہ اس اکاؤنٹ کا ہے جس نے اسے انسٹال کیا تھا، اور وہ سائن ان نہیں ہے۔
        [machine-variable] مشین کا ماحولیاتی متغیر { $name } باقی رہتا ہے، کیونکہ یہ اس اکاؤنٹ کا ہے جس نے اسے انسٹال کیا تھا، اور وہ سائن ان نہیں ہے۔
        [user-list] صارف کا { $name } باقی رہتا ہے، کیونکہ یہ اس اکاؤنٹ کا ہے جس نے اسے انسٹال کیا تھا، اور وہ سائن ان نہیں ہے۔
        [machine-list] مشین کا { $name } باقی رہتا ہے، کیونکہ یہ اس اکاؤنٹ کا ہے جس نے اسے انسٹال کیا تھا، اور وہ سائن ان نہیں ہے۔
       *[value] رجسٹری قدر { $name } باقی رہتی ہے، کیونکہ یہ اس اکاؤنٹ کی ہے جس نے اسے انسٹال کیا تھا، اور وہ سائن ان نہیں ہے۔
    }
core-rollback-setting-not-reached =
    { $kind ->
        [key] کلید { $name } تک نہیں پہنچا جا سکا: { $error }۔
        [user-variable] صارف کے ماحولیاتی متغیر { $name } تک نہیں پہنچا جا سکا: { $error }۔
        [machine-variable] مشین کے ماحولیاتی متغیر { $name } تک نہیں پہنچا جا سکا: { $error }۔
        [user-list] صارف کے { $name } تک نہیں پہنچا جا سکا: { $error }۔
        [machine-list] مشین کے { $name } تک نہیں پہنچا جا سکا: { $error }۔
       *[value] رجسٹری قدر { $name } تک نہیں پہنچا جا سکا: { $error }۔
    }
core-rollback-setting-not-read =
    { $kind ->
        [key] کلید { $name } پڑھی نہیں جا سکی: { $error }۔
        [user-variable] صارف کا ماحولیاتی متغیر { $name } پڑھا نہیں جا سکا: { $error }۔
        [machine-variable] مشین کا ماحولیاتی متغیر { $name } پڑھا نہیں جا سکا: { $error }۔
        [user-list] صارف کا { $name } پڑھا نہیں جا سکا: { $error }۔
        [machine-list] مشین کا { $name } پڑھا نہیں جا سکا: { $error }۔
       *[value] رجسٹری قدر { $name } پڑھی نہیں جا سکی: { $error }۔
    }
core-rollback-setting-changed =
    { $kind ->
        [user-variable] صارف کا ماحولیاتی متغیر { $name } باقی رہتا ہے، کیونکہ انسٹال ہونے کے بعد سے یہ تبدیل ہو چکا ہے۔
        [machine-variable] مشین کا ماحولیاتی متغیر { $name } باقی رہتا ہے، کیونکہ انسٹال ہونے کے بعد سے یہ تبدیل ہو چکا ہے۔
        [user-list] صارف کا { $name } باقی رہتا ہے، کیونکہ انسٹال ہونے کے بعد سے یہ تبدیل ہو چکا ہے۔
        [machine-list] مشین کا { $name } باقی رہتا ہے، کیونکہ انسٹال ہونے کے بعد سے یہ تبدیل ہو چکا ہے۔
       *[value] رجسٹری قدر { $name } باقی رہتی ہے، کیونکہ انسٹال ہونے کے بعد سے یہ تبدیل ہو چکی ہے۔
    }
# Said when it is back as it was before the install.
core-rollback-setting-put-back =
    { $kind ->
        [user-variable] صارف کا ماحولیاتی متغیر { $name } پہلے جیسا کر دیا گیا
        [machine-variable] مشین کا ماحولیاتی متغیر { $name } پہلے جیسا کر دیا گیا
        [user-list] صارف کا { $name } پہلے جیسا کر دیا گیا
        [machine-list] مشین کا { $name } پہلے جیسا کر دیا گیا
       *[value] رجسٹری قدر { $name } پہلے جیسی کر دی گئی
    }
core-rollback-setting-not-put-back =
    { $kind ->
        [user-variable] صارف کا ماحولیاتی متغیر { $name } پہلے جیسا نہیں کیا جا سکا: { $error }۔
        [machine-variable] مشین کا ماحولیاتی متغیر { $name } پہلے جیسا نہیں کیا جا سکا: { $error }۔
        [user-list] صارف کا { $name } پہلے جیسا نہیں کیا جا سکا: { $error }۔
        [machine-list] مشین کا { $name } پہلے جیسا نہیں کیا جا سکا: { $error }۔
       *[value] رجسٹری قدر { $name } پہلے جیسی نہیں کی جا سکی: { $error }۔
    }
# A registry value the install wrote, which is the user's data, and stays unless they asked for it to go.
core-rollback-value-kept-by-request = رجسٹری قدر { $name } باقی رہتی ہے، کیونکہ اسے ہٹانے کے لیے نہیں کہا گیا تھا۔
core-rollback-key-holds-others = کلید { $name } باقی رہتی ہے، کیونکہ اس میں ایسی چیزیں ہیں جو اس تنصیب نے وہاں نہیں رکھیں۔
core-rollback-key-removed = کلید { $name } ہٹا دی گئی
core-rollback-key-not-removed = کلید { $name } ہٹائی نہیں جا سکی: { $error }۔
# { $kind } is user-list or machine-list.
core-rollback-list-entries-removed =
    { $kind ->
        [user-list] اس تنصیب کے اندراجات صارف کے { $name } سے نکال دیے گئے
       *[machine-list] اس تنصیب کے اندراجات مشین کے { $name } سے نکال دیے گئے
    }
core-rollback-list-entries-not-removed =
    { $kind ->
        [user-list] اس تنصیب کے اندراجات صارف کے { $name } سے نہیں نکالے جا سکے: { $error }۔
       *[machine-list] اس تنصیب کے اندراجات مشین کے { $name } سے نہیں نکالے جا سکے: { $error }۔
    }

## Services and drivers. { $kind } is service or driver; { $name } is its name, such as AppUpdater. { $error } and
## { $reason } are what Windows said went wrong.

core-rollback-service-not-read =
    { $kind ->
        [driver] ڈرائیور { $name } پڑھا نہیں جا سکا: { $error }۔
       *[service] سروس { $name } پڑھی نہیں جا سکی: { $error }۔
    }
core-rollback-service-changed =
    { $kind ->
        [driver] ڈرائیور { $name } باقی رہتا ہے، کیونکہ انسٹال ہونے کے بعد سے یہ تبدیل ہو چکا ہے۔
       *[service] سروس { $name } باقی رہتی ہے، کیونکہ انسٹال ہونے کے بعد سے یہ تبدیل ہو چکی ہے۔
    }
core-rollback-service-not-stopped =
    { $kind ->
        [driver] ڈرائیور { $name } کو روکا نہیں جا سکا ({ $reason })، اس لیے جو کچھ یہ استعمال کر رہا ہے وہ PC کے دوبارہ شروع ہونے تک باقی رہ سکتا ہے۔
       *[service] سروس { $name } کو روکا نہیں جا سکا ({ $reason })، اس لیے جو کچھ یہ استعمال کر رہی ہے وہ PC کے دوبارہ شروع ہونے تک باقی رہ سکتا ہے۔
    }
core-rollback-service-stopped-for-this =
    { $kind ->
        [driver] یہ کرنے کے لیے ڈرائیور { $name } کو روکا گیا۔ جب آپ کو اس کی ضرورت ہو تو اسے دوبارہ شروع کریں۔
       *[service] یہ کرنے کے لیے سروس { $name } کو روکا گیا۔ جب آپ کو اس کی ضرورت ہو تو اسے دوبارہ شروع کریں۔
    }
core-rollback-service-not-removed =
    { $kind ->
        [driver] ڈرائیور { $name } ہٹایا نہیں جا سکا: { $reason }۔
       *[service] سروس { $name } ہٹائی نہیں جا سکی: { $reason }۔
    }
core-rollback-service-marked-for-removal =
    { $kind ->
        [driver] ڈرائیور { $name } ہٹانے کے لیے نشان زد ہے، اور PC کے دوبارہ شروع ہونے پر، یا جب اسے کھولے رکھنے والی چیز اسے چھوڑ دے، ہٹ جائے گا۔
       *[service] سروس { $name } ہٹانے کے لیے نشان زد ہے، اور PC کے دوبارہ شروع ہونے پر، یا جب اسے کھولے رکھنے والی چیز اسے چھوڑ دے، ہٹ جائے گی۔
    }
core-rollback-service-removed =
    { $kind ->
        [driver] ڈرائیور { $name } ہٹا دیا گیا
       *[service] سروس { $name } ہٹا دی گئی
    }
core-rollback-service-not-restored =
    { $kind ->
        [driver] ڈرائیور { $name } کو پہلے جیسا نہیں کیا جا سکا: { $reason }۔
       *[service] سروس { $name } کو پہلے جیسا نہیں کیا جا سکا: { $reason }۔
    }
core-rollback-service-restored =
    { $kind ->
        [driver] ڈرائیور { $name } کی ترتیبات پہلے جیسی کر دی گئیں
       *[service] سروس { $name } کی ترتیبات پہلے جیسی کر دی گئیں
    }
core-rollback-service-not-restarted = سروس { $name } اس تنصیب سے پہلے چل رہی تھی، اور دوبارہ شروع نہیں ہوئی: { $reason }۔
# { $account } is the account the service ran as, such as CONTOSO\backup.
core-rollback-service-account-again = سروس { $name } { $account } کے طور پر چلتی تھی۔ اس کا پاس ورڈ محفوظ نہیں رکھا جاتا، اس لیے سروسز میں اس کا اکاؤنٹ دوبارہ سیٹ کریں۔

## Firewall rules. { $name } is the rule's name; { $error } is what went wrong.

core-rollback-firewall-not-read = فائر وال قاعدہ { $name } پڑھا نہیں جا سکا: { $error }۔
core-rollback-firewall-says-otherwise = فائر وال قاعدہ { $name } باقی رہتا ہے، کیونکہ اس نام سے جو قاعدہ موجود ہے وہ کچھ اور کہتا ہے۔
core-rollback-firewall-changed = فائر وال قاعدہ { $name } باقی رہتا ہے، کیونکہ انسٹال ہونے کے بعد سے یہ تبدیل ہو چکا ہے۔
# The capital "The" is as the app has always said it.
core-rollback-firewall-removed = فائر وال قاعدہ { $name } ہٹا دیا گیا
core-rollback-firewall-not-removed = فائر وال قاعدہ { $name } ہٹایا نہیں جا سکا: { $error }۔

## Scheduled tasks. { $path } is a folder of tasks, and { $name } a task with its folder, such as \App\Nightly Update.
## { $error } and { $reason } are what went wrong.

core-rollback-task-folder-holds-others = ٹاسک فولڈر { $path } باقی رہتا ہے، کیونکہ اس میں ایسے ٹاسکس ہیں جو اس تنصیب نے وہاں نہیں رکھے۔
core-rollback-task-folder-not-removed = ٹاسک فولڈر { $path } ہٹایا نہیں جا سکا: { $error }۔
core-rollback-task-not-read = ٹاسک { $name } پڑھا نہیں جا سکا: { $error }۔
core-rollback-task-changed = ٹاسک { $name } باقی رہتا ہے، کیونکہ انسٹال ہونے کے بعد سے یہ تبدیل ہو چکا ہے۔
core-rollback-task-removed = ٹاسک { $name } ہٹا دیا گیا
core-rollback-task-not-removed = ٹاسک { $name } ہٹایا نہیں جا سکا: { $reason }۔
core-rollback-task-put-back = ٹاسک { $name } واپس رکھ دیا گیا
core-rollback-task-not-put-back = ٹاسک { $name } واپس نہیں رکھا جا سکا: { $reason }۔

## When an install cannot be undone at all

# { $error } is what went wrong.
core-rollback-record-unreadable = اس تنصیب کا ریکارڈ پڑھا نہیں جا سکا، اس لیے کچھ کالعدم نہیں کیا گیا: { $error }۔
# { $error } is what went wrong.
core-rollback-nothing-undone = کچھ کالعدم نہیں کیا گیا: { $error }۔
# One app's part of the message below. { $app_name } is the app's name; { $failures } is what could not be undone, a
# sentence or more; { $folder } is the folder that holds the install's record.
core-rollback-earlier-install-stuck = { $app_name }: { $failures } اگر اسے کبھی کالعدم نہ کیا جا سکے، تو کوئی منتظم { $folder } کو ہٹا کر اسے ترک کر سکتا ہے۔
# Why an install did not start. { $details } is one or more of the message above.
core-rollback-earlier-installs-not-undone = ایک پچھلی تنصیب، جو کبھی مکمل نہیں ہوئی، پوری طرح کالعدم نہیں کی جا سکی، اس لیے کچھ انسٹال نہیں کیا گیا۔ جو چیز بھی انہیں استعمال کر رہی ہے اسے بند کریں اور دوبارہ کوشش کریں۔ { $details }
core-rollback-install-busy = اس تنصیب کو کوئی اور انسٹالر یا ان انسٹالر تبدیل کر رہا ہے

## The summary an uninstall writes on the command line, one line each, with what each says below it. { $count } is how
## many.

core-rollback-count-undone = کالعدم کیا گیا: { $count }
core-rollback-count-failed = کالعدم نہیں کیا جا سکا: { $count }
# What was left on purpose: it has changed since the install, or it is the user's.
core-rollback-count-kept = اپنی جگہ چھوڑ دیا گیا: { $count }
# What no uninstall can take back, such as a runtime that was installed, which other programs may use.
core-rollback-count-left-behind = پیچھے رہ گیا: { $count }

## Which Windows a PC or drive has, as the app shows it beside the drive.

# { $name } is the Windows' name, such as Windows 7 Professional Service Pack 1.
core-system-windows-32-bit = { $name }، 32 بٹ
# { $name } is the Windows' name, such as Windows 11 Pro.
core-system-windows-64-bit = { $name }، 64 بٹ
# A drive's Windows, when it only says it is older than Vista.
core-system-windows-xp-or-older = Windows XP یا اس سے پرانا

## Reading a connected drive, or another account's settings. Each is why it could not be done, and some start with a
## small letter, as they follow other words.

core-system-needs-administrator = منسلک ڈرائیو یا کسی دوسرے اکاؤنٹ کی ترتیبات پڑھنے کے لیے منتظم کے حقوق درکار ہیں
core-system-no-program-data = اس PC کا ProgramData فولڈر نہیں مل سکا
# { $path } is a file, such as D:\Windows\System32\config\SOFTWARE.
core-system-not-a-hive-file = ہائیو فائل نہیں ہے: { $path }
# { $path } is a file; { $size } is its size in bytes.
core-system-hive-too-large = { $path } کا سائز { $size } بائٹس ہے، جو رجسٹری ہائیو کے لیے بہت زیادہ ہے، یا اس ڈرائیو کے تمام ہائیوز کے مجموعی سائز سے بھی زیادہ ہے
# { $key } is a registry key, such as Software\Microsoft\Windows\CurrentVersion.
core-system-hive-no-mapping = آف لائن ہائیو میں HKLM\{ $key } کی کوئی میپنگ نہیں ہے
# { $key } is where a drive's registry was to be loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said.
# "reg load" is a command's name.
core-system-hive-not-loaded = reg load { $key } ناکام ہو گیا: { $error }
# { $key } is where a drive's registry was loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said. "reg
# unload" is a command's name.
core-system-hive-not-unloaded = reg unload { $key } ناکام ہو گیا: { $error }
# { $drive } is a drive, such as D:\.
core-system-not-a-system-drive = { $drive } Windows سسٹم ڈرائیو نہیں ہے (اس میں \Windows فولڈر نہیں ہے)۔
# { $drive } is a drive, such as C:\. "This PC" is the name of a choice in the app.
core-system-own-system-drive = { $drive } اس PC کی اپنی سسٹم ڈرائیو ہے۔ اس PC کو پڑھنے کے لیے "یہ PC" منتخب کریں۔
# { $drive } is a drive, such as D:\.
core-system-registry-through-link = { $drive } Windows سسٹم ڈرائیو نہیں ہے: اس کی رجسٹری تک کا راستہ کسی لنک سے گزرتا ہے۔

## Warnings a scan records about whose data it read.

# { $account } is an account's name or id, such as alice or S-1-5-21-...-1001.
core-warning-scan-account-not-on-record = اکاؤنٹ { $account } یہاں ریکارڈ میں نہیں ہے، اس لیے کسی کا بھی فی صارف ڈیٹا یا ترتیبات شامل نہیں کی گئیں۔
# { $account } is an account's name, such as CONTOSO\alice.
core-warning-scan-signed-in-account-no-profile = یہاں سائن ان اکاؤنٹ، { $account }، کی کوئی پروفائل ریکارڈ میں نہیں ہے، اس لیے کسی کا بھی فی صارف ڈیٹا یا ترتیبات شامل نہیں کی گئیں۔
# { $reason } is why Windows could not say.
core-warning-scan-signed-in-account-unknown = یہ معلوم نہیں ہو سکا کہ کون سائن ان ہے ({ $reason })، اس لیے کسی کا بھی فی صارف ڈیٹا یا ترتیبات شامل نہیں کی گئیں۔ منتخب کریں کہ کس کا ڈیٹا شامل کرنا ہے۔
core-warning-scan-last-account-unknown = یہ ڈرائیو نہیں بتاتی کہ آخری بار کون سائن ان ہوا تھا، اور اس میں ایک سے زیادہ اکاؤنٹس ہیں، اس لیے کسی کا بھی فی صارف ڈیٹا یا ترتیبات شامل نہیں کی گئیں۔ منتخب کریں کہ کس کا ڈیٹا شامل کرنا ہے۔
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-settings-unreadable = { $account } کی ترتیبات پڑھی نہیں جا سکیں، اس لیے انہیں شامل نہیں کیا گیا: { $error }
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-classes-unreadable = { $account } کے لیے رجسٹر کردہ COM کلاسز اور فائل کی اقسام پڑھی نہیں جا سکیں، اس لیے انہیں شامل نہیں کیا گیا: { $error }

## Paths a scan did not follow. { $count } is how many; { $paths } is some of them, separated by semicolons; { $more } is
## how many more there are, not shown.

core-warning-scan-drive-paths-not-followed =
    { $count ->
        [one] اس ڈرائیو پر ریکارڈ شدہ ایک پاتھ پڑھا نہیں جا سکا، اس لیے وہاں کی کوئی چیز شامل نہیں کی گئی۔ یہ کسی دوسری ڈسک پر ہے، مکمل پاتھ نہیں ہے، یا کسی ایسے لنک سے گزرتا ہے جو ڈرائیو سے باہر لے جائے گا: { $paths }
       *[other] اس ڈرائیو پر ریکارڈ شدہ کچھ پاتھ پڑھے نہیں جا سکے، اس لیے وہاں کی کوئی چیز شامل نہیں کی گئی۔ ان میں سے ہر ایک کسی دوسری ڈسک پر ہے، مکمل پاتھ نہیں ہے، یا کسی ایسے لنک سے گزرتا ہے جو ڈرائیو سے باہر لے جائے گا: { $paths }
    }
core-warning-scan-drive-paths-not-followed-more = اس ڈرائیو پر ریکارڈ شدہ کچھ پاتھ پڑھے نہیں جا سکے، اس لیے وہاں کی کوئی چیز شامل نہیں کی گئی۔ ان میں سے ہر ایک کسی دوسری ڈسک پر ہے، مکمل پاتھ نہیں ہے، یا کسی ایسے لنک سے گزرتا ہے جو ڈرائیو سے باہر لے جائے گا: { $paths } اور مزید { $more }
core-warning-scan-network-paths-not-followed =
    { $count ->
        [one] اس PC پر ریکارڈ شدہ ایک پاتھ پڑھا نہیں جا سکا، اس لیے وہاں کی کوئی چیز شامل نہیں کی گئی۔ یہ اس PC کی کسی ڈسک پر نہیں ہے، جہاں اسے دیکھنے سے اس سے رابطہ قائم ہو جائے گا، یا اس کا نام ایسا ہے جو کچھ اور کھول دے گا: { $paths }
       *[other] اس PC پر ریکارڈ شدہ کچھ پاتھ پڑھے نہیں جا سکے، اس لیے وہاں کی کوئی چیز شامل نہیں کی گئی۔ ان میں سے ہر ایک اس PC کی کسی ڈسک پر نہیں ہے، جہاں اسے دیکھنے سے اس سے رابطہ قائم ہو جائے گا، یا اس کا نام ایسا ہے جو کچھ اور کھول دے گا: { $paths }
    }
core-warning-scan-network-paths-not-followed-more = اس PC پر ریکارڈ شدہ کچھ پاتھ پڑھے نہیں جا سکے، اس لیے وہاں کی کوئی چیز شامل نہیں کی گئی۔ ان میں سے ہر ایک اس PC کی کسی ڈسک پر نہیں ہے، جہاں اسے دیکھنے سے اس سے رابطہ قائم ہو جائے گا، یا اس کا نام ایسا ہے جو کچھ اور کھول دے گا: { $paths } اور مزید { $more }

## What a scan with none of an account's data leaves out of a package, in the app's list of what was left out. Each
## has a name, and below it why.

core-account-left-out-shortcuts = اس اکاؤنٹ کے اپنے فولڈرز میں موجود شارٹ کٹس
# { $count } is how many shortcuts.
core-account-left-out-shortcuts-why =
    { $count ->
        [one] { $count } شارٹ کٹ، جو صرف صارف کے ڈیٹا کے اختیار کے ساتھ جاتا ہے
       *[other] { $count } شارٹ کٹس، جو صرف صارف کے ڈیٹا کے اختیار کے ساتھ جاتے ہیں
    }
core-account-left-out-variables = اس اکاؤنٹ کے ماحولیاتی متغیرات
# { $count } is how many variables.
core-account-left-out-variables-why =
    { $count ->
        [one] { $count } متغیر، جو صرف صارف کے ڈیٹا کے اختیار کے ساتھ جاتا ہے
       *[other] { $count } متغیرات، جو صرف صارف کے ڈیٹا کے اختیار کے ساتھ جاتے ہیں
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-account-left-out-task = شیڈول کردہ ٹاسک { $task }
core-account-task-works-on-account-folder = یہ کسی اکاؤنٹ کے فولڈرز میں موجود کسی جگہ پر کام کرتا ہے، یا اس کا نام کسی ایسی جگہ کے نام پر ہے، اور صرف صارف کے ڈیٹا کے اختیار کے ساتھ جاتا ہے
core-account-task-runs-from-account-folder = یہ کسی اکاؤنٹ کے فولڈرز سے جو کچھ اور بھی چلاتا ہے، وہ صرف صارف کے ڈیٹا کے اختیار کے ساتھ جاتا ہے
# { $service } is a service's name, such as AppUpdater.
core-account-left-out-service = سروس { $service }
core-account-service-works-on-account-folder = یہ کسی اکاؤنٹ کے فولڈرز میں موجود کسی جگہ پر کام کرتی ہے، اور صرف صارف کے ڈیٹا کے اختیار کے ساتھ جاتی ہے

## What a package was found to still say of whose PC it came from, after a build with none of an account's data.

# The parts of a package, as the warning below names them.
core-account-part-files = ایپ کی اپنی فائلوں کے پاتھ
core-account-part-registry = رجسٹری اقدار
core-account-part-settings-entry = ترتیبات میں ایپ کا اندراج
core-account-part-main-program = اس کے مرکزی پروگرام کا مقام
core-account-part-shortcuts = شارٹ کٹس
core-account-part-tasks = شیڈول کردہ ٹاسکس
core-account-part-services = سروسز
core-account-part-variables = ماحولیاتی متغیرات
# One place in the list below. { $place } is one of the parts above, or a file's name; { $count } is how many times.
core-account-mention-place = { $place } ({ $count })
# { $kind } is what was found: account (an account's name), folder (where an account's folder is), computer (the PC's
# name), or id (an account's id). { $places } is a list of where, from the message above.
core-warning-scan-package-mentions =
    { $kind ->
        [account] یہ پیکیج { $places } میں کسی اکاؤنٹ کے نام کا ذکر کرتا ہے۔ اسے کسی بھی اکاؤنٹ کے ڈیٹا کے بغیر بنایا گیا تھا، اور جس کے پاس بھی انسٹالر ہو وہ یہ پڑھ سکتا ہے۔
        [folder] یہ پیکیج { $places } میں ذکر کرتا ہے کہ کسی اکاؤنٹ کا فولڈر کہاں ہے۔ اسے کسی بھی اکاؤنٹ کے ڈیٹا کے بغیر بنایا گیا تھا، اور جس کے پاس بھی انسٹالر ہو وہ یہ پڑھ سکتا ہے۔
        [computer] یہ پیکیج { $places } میں PC کے نام کا ذکر کرتا ہے۔ اسے کسی بھی اکاؤنٹ کے ڈیٹا کے بغیر بنایا گیا تھا، اور جس کے پاس بھی انسٹالر ہو وہ یہ پڑھ سکتا ہے۔
       *[id] یہ پیکیج { $places } میں کسی اکاؤنٹ کی ID کا ذکر کرتا ہے۔ اسے کسی بھی اکاؤنٹ کے ڈیٹا کے بغیر بنایا گیا تھا، اور جس کے پاس بھی انسٹالر ہو وہ یہ پڑھ سکتا ہے۔
    }
# { $count } is how many folders.
core-warning-scan-folders-of-others-unreadable =
    { $count ->
        [one] ایپ سے باہر کا { $count } فولڈر پڑھا نہیں جا سکا، اس لیے وہاں موجود ایپ کا ڈیٹا غائب ہو سکتا ہے۔
       *[other] ایپ سے باہر کے { $count } فولڈرز پڑھے نہیں جا سکے، اس لیے وہاں موجود ایپ کا ڈیٹا غائب ہو سکتا ہے۔
    }

## What a scan leaves out because it looks like a credential, or is a license with the License option off, in the app's
## list of what was left out. Each has a name, and below it why.

core-secret-why-credential = یہ اسناد جیسا لگتا ہے
core-secret-why-license-option-off = لائسنس اور فعال کاری کے ڈیٹا کا اختیار آف ہے
# { $count } is how many values of a registry key.
core-secret-why-account-values =
    { $count ->
        [one] { $count } قدر، جو صرف صارف کے ڈیٹا کے اختیار کے ساتھ جاتی ہے
       *[other] { $count } اقدار، جو صرف صارف کے ڈیٹا کے اختیار کے ساتھ جاتی ہیں
    }
# Shown in place of a registry value's name when its name itself looks like a password or a key.
core-secret-name-holds-credential = (ایسی قدر جس کے نام میں اسناد ہیں)
# Shown in place of an option's name, for an argument on a command line that has no name.
core-secret-unnamed-argument = اسناد پر مشتمل ایک آرگیومنٹ
# { $option } is an option taken out of a command line, such as /password, or the message above. { $place } says
# where the command line was: service, task, shortcut, opener (what opens a file type), opener-unnamed (the same, of a
# type with no name), or uninstall. { $name } is the service, task, shortcut, file type, or app.
core-secret-taken-from =
    { $place ->
        [service] { $name } کو چلانے کے طریقے میں { $option }
        [task] ٹاسک { $name } کو چلانے کے طریقے میں { $option }
        [shortcut] شارٹ کٹ { $name } میں { $option }
        [opener] { $name } کو کھولنے والی چیز میں { $option }
        [opener-unnamed] فائل کی کسی قسم کو کھولنے والی چیز میں { $option }
       *[uninstall] { $name } کو ان انسٹال کرنے کے طریقے میں { $option }
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-secret-task-definition-unreadable = ٹاسک { $task } کی تعریف، جسے جانچ کے لیے پڑھا نہیں جا سکا
# { $name } is an environment variable's name, such as APP_TOKEN.
core-secret-left-out-variable = ماحولیاتی متغیر { $name }
# { $class } is a COM class's id, such as {00000000-0000-0000-0000-000000000000}.
core-secret-left-out-class-server = وہ چیز جسے کلاس { $class } چلانے کا کہتی ہے
# { $rule } is a firewall rule's name.
core-secret-left-out-firewall-description = فائر وال قاعدہ { $rule } کی تفصیل

## Licenses a scan found, as the package's report describes each.

# { $name } is a file's name, such as license.lic.
core-license-detail-file = لائسنس فائل ملی: { $name }
# { $framework } is a licensing product's name, such as FlexLM / FlexNet.
core-license-detail-store = { $framework } لائسنس اسٹور موجود ہے
# { $name } is an environment variable's name, such as LM_LICENSE_FILE.
core-license-detail-variable = لائسنس کا ماحولیاتی متغیر { $name }
# { $name } is a registry value's name, such as SerialNumber.
core-license-detail-registry-value = رجسٹری قدر سے لائسنسنگ ڈیٹا کا اشارہ ملتا ہے: { $name }
# Said of a registry value in an account's key, which is not named.
core-license-detail-registry-value-of-an-account = رجسٹری قدر سے لائسنسنگ ڈیٹا کا اشارہ ملتا ہے
# { $key } is a registry key, such as HKLM\Software\Contoso\License.
core-license-detail-registry-key = رجسٹری کلید سے لائسنسنگ ڈیٹا کا اشارہ ملتا ہے: { $key }

## Warnings a scan records about licenses.

core-warning-scan-license-data = لائسنسنگ یا فعال کاری کا ڈیٹا ملا۔ لائسنس اور فعال کاری کے ڈیٹا کا اختیار آن ہونے پر، انسٹالر وہ سب بحال کرتا ہے جو منتقل کیا جا سکتا ہے؛ پوری مشین کی اقدار اور ہارڈویئر سے بندھے اسٹورز کی صرف اطلاع دی جاتی ہے۔ WinAppMigrator کبھی ڈیٹا کو ڈکرپٹ نہیں کرتا، فعال کاری کو بائی پاس نہیں کرتا، اور ڈونگلز کی نقل نہیں کرتا۔
core-warning-scan-license-machine-bound = کچھ لائسنس مشین سے بندھے ہوئے ہیں (ہارڈویئر ڈونگل، مشین سے مقفل فعال کاری، یا مرموز)۔ انسٹالر جو بحال کر سکتا ہے، کرتا ہے؛ ہو سکتا ہے ان کے لیے نئے PC پر پھر بھی دوبارہ فعال کاری درکار ہو۔

## Warnings a scan records about the registry. What could not be read is not named in them: it is kept beside them.

# { $error } is what Windows said went wrong. App Paths is a registry key's name.
core-warning-scan-app-paths-key-unreadable = App Paths کی ذیلی کلید پڑھی نہیں جا سکی: { $error }
core-warning-scan-registry-limit = رجسٹری کا حصول مقررہ حفاظتی حد تک پہنچ گیا۔
core-warning-scan-registry-subtree-limit = رجسٹری کی ذیلی شاخ کا حصول مقررہ حفاظتی حد تک پہنچ گیا۔
# { $error } is what Windows said went wrong.
core-warning-scan-registry-key-unreadable = رجسٹری کلید پڑھی نہیں جا سکی: { $error }
core-warning-scan-registry-value-too-large = رجسٹری قدر حصول کے سائز کی مقررہ حد سے تجاوز کر گئی۔

## Scheduled tasks

core-warning-scan-tasks-folder-through-link = اس ڈرائیو پر شیڈول کردہ ٹاسکس کا فولڈر کسی لنک سے گزرتا ہے، اس لیے اس کے ٹاسکس پڑھے نہیں گئے۔
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-warning-scan-task-left-out = شیڈول کردہ ٹاسک { $task } کو خارج کر دیا گیا: یہ اس اکاؤنٹ کے علاوہ کسی اور اکاؤنٹ کے طور پر چلتا ہے جس کی ترتیبات شامل کی جا رہی ہیں، منتظمین کے علاوہ دوسرے اس کی فائل تبدیل کر سکتے ہیں، یا ایپ کے ٹاسکس رکھنے کے لیے بہت زیادہ ہیں۔
# { $error } is what Windows said went wrong.
core-warning-scan-tasks-folder-unreadable = شیڈول کردہ ٹاسکس کا ایک حصہ پڑھا نہیں جا سکا: { $error }
# { $path } is a task's file, such as C:\Windows\System32\Tasks\App\Nightly Sync; { $reason } is one of the reasons
# below, or what Windows said went wrong.
core-warning-scan-task-unreadable = شیڈول کردہ ٹاسک { $path } پڑھا نہیں جا سکا: { $reason }۔ اگر یہ ایپ چلاتا ہے، تو اسے شامل نہیں کیا گیا۔
# Why a task's file could not be read. { $size } is its size in bytes.
core-task-too-large = اس کا سائز { $size } بائٹس ہے، جو ٹاسک ہونے کے لیے بہت بڑا ہے
# Why a task's file could not be read.
core-task-ends-mid-character = یہ کسی حرف کے درمیان میں ختم ہو جاتا ہے
# Why a task's file could not be read. { $error } says where.
core-task-not-utf16 = یہ درست UTF-16 نہیں ہے: { $error }
# Why a task's file could not be read. { $error } says where.
core-task-not-text = یہ درست UTF-8 یا UTF-16 نہیں ہے: { $error }
# What a package records of how a task is put back. Register-ScheduledTask is a command's name.
core-task-restore-instructions = بحالی کے لیے محفوظ شدہ XML تعریف کے ساتھ Register-ScheduledTask استعمال کیا جاتا ہے۔

## Reading a program's file. Each is why it could not be read, after the program's name.

# { $reason } is what went wrong.
core-pe-unreadable = اسے پڑھا نہیں جا سکا: { $reason }
core-pe-headers-misaligned = اس کے ہیڈرز ان حدود پر نہیں ہیں جن پر کسی پروگرام کے ہیڈرز کا ہونا ضروری ہے
# { $size } is the file's size in bytes.
core-pe-too-large = اس کا سائز { $size } بائٹس ہے، جو پروگرام ہونے کے لیے بہت بڑا ہے
# PE is the name of the format Windows programs are in.
core-pe-no-headers = اس میں PE ہیڈرز نہیں ہیں

## Warnings a scan records about what a program needs. Which program is kept beside each.

# { $error } is what went wrong.
core-warning-scan-imports-unreadable = امپورٹ ٹیبل پڑھا نہیں جا سکا: { $error }
# { $error } is what went wrong.
core-warning-scan-import-name-unreadable = امپورٹ ڈسکرپٹر کا نام پڑھا نہیں جا سکا: { $error }
# { $count } is how many were skipped.
core-warning-scan-legacy-delay-imports =
    { $count ->
        [one] { $count } پرانا (v1) ڈیلے لوڈ ڈسکرپٹر چھوڑ دیا گیا؛ صرف جدید RVA پر مبنی ڈیلے امپورٹس پارس کیے جاتے ہیں۔
       *[other] { $count } پرانے (v1) ڈیلے لوڈ ڈسکرپٹرز چھوڑ دیے گئے؛ صرف جدید RVA پر مبنی ڈیلے امپورٹس پارس کیے جاتے ہیں۔
    }

## The DLLs a program needs. Which DLL or program is kept beside each warning.

# { $path } is a program, such as C:\Program Files\App\app.exe.
core-dll-program-has-no-folder = قابل عمل فائل کا کوئی پیرنٹ فولڈر نہیں ہے: { $path }
# Why a DLL is among the app's files, in the package's list of them.
core-dll-beside-program = DLL منتخب قابل عمل فائل کے ساتھ موجود ہے
# { $error } is what went wrong.
core-warning-scan-dependency-unscanned = ذیلی انحصار اسکین نہیں کیا جا سکا: { $error }
# { $name } is a DLL's name, such as d3dx9_43.dll.
core-warning-scan-dependency-not-in-newer-windows = { $name } اس PC کے Windows فولڈر میں تھا جس پر یہ ایپ تھی، اور Windows 10 اور 11 کے ساتھ یہ نہیں آتا۔ اسے وہاں کسی اور چیز نے انسٹال کیا تھا، اس لیے یہ ایپ کی اپنی فائلوں کا حصہ نہیں ہے اور ساتھ نہیں آتا۔ ہو سکتا ہے ایپ اس وقت تک شروع نہ ہو جب تک اسے فراہم کرنے والی چیز انسٹال نہ کی جائے۔
# { $error } is what went wrong.
core-warning-scan-dependency-not-hashed = انحصار کا ہیش نہیں بنایا جا سکا: { $error }
# SystemRoot and windir are the names of environment variables.
core-warning-scan-windows-folder-unknown = SystemRoot/windir ماحولیاتی متغیرات دستیاب نہیں ہیں؛ سسٹم DLL کی تلاش محدود ہے۔
# { $error } is what went wrong.
core-warning-scan-dependency-architecture-unreadable = انحصار کا آرکیٹیکچر پڑھا نہیں جا سکا: { $error }

## Services, drivers, shortcuts, and environment variables a scan reads. Where it was reading is kept beside each
## warning.

# { $error } is what Windows said went wrong.
core-warning-scan-services-unreadable = سروسز کی رجسٹری کلید پڑھی نہیں جا سکی: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-drivers-unreadable = ڈرائیورز تلاش کرنے کے لیے سروسز کی رجسٹری کلید پڑھی نہیں جا سکی: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-queried = شارٹ کٹس کے بارے میں معلومات حاصل نہیں کی جا سکیں: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-parsed = شارٹ کٹس کی معلومات کا آؤٹ پٹ پارس نہیں کیا جا سکا: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-environment-unreadable = مشین کے ماحولیاتی متغیرات کی رجسٹری کلید پڑھی نہیں جا سکی: { $error }

## What a package records of how a driver is put back, which an installer shows. { $kind } is the driver's type as
## Windows names it (kernel, file-system, recognizer, or unknown-type); { $name } is its service's name; { $path } is
## its file. { $signed } is yes when { $signer } names who signed it, and no when that is not known.

core-service-driver-bundled =
    { $signed ->
        [yes] ایپلیکیشن کا اپنا { $kind } ڈرائیور "{ $name }" ({ $path }، دستخط کنندہ: { $signer })۔ انسٹالر اس کی .sys فائل شامل کرتا ہے اور ڈرائیور سروس کو خود بخود دوبارہ رجسٹر کرتا ہے (sc create، منتظم کے حقوق درکار ہیں)، پھر تصدیق کرتا ہے کہ سروس رجسٹر ہو گئی ہے۔
       *[no] ایپلیکیشن کا اپنا { $kind } ڈرائیور "{ $name }" ({ $path }، دستخط نامعلوم)۔ انسٹالر اس کی .sys فائل شامل کرتا ہے اور ڈرائیور سروس کو خود بخود دوبارہ رجسٹر کرتا ہے (sc create، منتظم کے حقوق درکار ہیں)، پھر تصدیق کرتا ہے کہ سروس رجسٹر ہو گئی ہے۔
    }
core-service-driver-from-vendor =
    { $signed ->
        [yes] وینڈر یا Windows کے ساتھ آنے والا { $kind } ڈرائیور "{ $name }" ({ $path }، دستخط کنندہ: { $signer })۔ WinAppMigrator تیسرے فریق کے ڈرائیورز آگے تقسیم نہیں کرتا؛ ایپ چلانے سے پہلے اسے ہدف پر وینڈر کے دستخط شدہ ڈرائیور پیکیج سے دوبارہ انسٹال کریں۔
       *[no] وینڈر یا Windows کے ساتھ آنے والا { $kind } ڈرائیور "{ $name }" ({ $path }، دستخط نامعلوم)۔ WinAppMigrator تیسرے فریق کے ڈرائیورز آگے تقسیم نہیں کرتا؛ ایپ چلانے سے پہلے اسے ہدف پر وینڈر کے دستخط شدہ ڈرائیور پیکیج سے دوبارہ انسٹال کریں۔
    }

## Runtimes an app needs, such as the Visual C++ runtime or .NET: how each is provided, as the Dependencies list says.

core-runtime-handling-built-in = Windows میں شامل ہے
core-runtime-handling-ships-with-app = ایپ کے ساتھ آتا ہے
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-handling-windows-feature = Windows خصوصیت: { $feature }
core-runtime-handling-included = شامل ہے
# { $id } is the runtime's name in winget, such as Microsoft.VCRedist.2015+.x64.
core-runtime-handling-winget = winget سے انسٹال کیا جاتا ہے: { $id }
core-runtime-handling-manual = دستی طور پر انسٹال کریں

## Downloading a runtime's installer, when an installer is built. Each is why it could not be, and starts with a small
## letter, as it follows other words. { $url } is a web address; { $error } is what went wrong.

core-runtime-download-not-https = { $url } ڈاؤن لوڈ کرنے سے انکار کیا گیا: یہ HTTPS نہیں ہے
# TLS is what keeps a download private.
core-runtime-tls-failed = TLS شروع کرنا ناکام ہو گیا: { $error }
core-runtime-download-failed = { $url } کا ڈاؤن لوڈ ناکام ہو گیا: { $error }
core-runtime-download-unreadable = { $url } پڑھنا ناکام ہو گیا: { $error }
core-runtime-download-empty = { $url } کے ڈاؤن لوڈ سے کوئی ڈیٹا نہیں ملا
# { $came } and { $expected } are numbers of bytes.
core-runtime-download-cut-short = { $url } کا ڈاؤن لوڈ ادھورا رہ گیا: { $expected } میں سے { $came } بائٹس آئے

## Installing a runtime: why it was not installed. Each starts with a small letter, as it follows other words.

core-runtime-no-known-installer = اس رن ٹائم کا کوئی معلوم انسٹالر نہیں ہے
# { $path } is where in the package, such as prerequisites\vc_redist.x64.exe.
core-runtime-payload-misplaced = پیکیج { $path } کو اس رن ٹائم کا انسٹالر بتاتا ہے، جو اپنی درست جگہ پر نہیں ہے، اس لیے اسے نہیں چلایا گیا
# { $path } is a file.
core-runtime-payload-missing = شامل کردہ انسٹالر موجود نہیں ہے: { $path }
# { $error } is what went wrong.
core-runtime-payload-path-invalid = شامل کردہ انسٹالر کا پاتھ غلط ہے: { $error }
# { $url } is the web address it can be downloaded from. winget is a program's name.
core-runtime-no-installer-route = نہ کوئی شامل کردہ انسٹالر ہے نہ winget ID؛ اسے { $url } سے حاصل کریں
# { $error } is what went wrong.
core-runtime-no-temp-folder = اسے نکالنے کے لیے فولڈر نہیں بنایا جا سکا: { $error }
# { $reason } is what the check found.
core-runtime-payload-signature-failed = شامل کردہ انسٹالر اپنے دستخط کی جانچ میں ناکام رہا اور اسے نہیں چلایا گیا: { $reason }
core-runtime-payload-timed-out = شامل کردہ انسٹالر مقررہ وقت میں مکمل نہیں ہوا اور اسے ختم کر دیا گیا
# { $error } is what went wrong.
core-runtime-payload-not-started = شامل کردہ انسٹالر شروع نہیں کیا جا سکا: { $error }
core-runtime-installer-killed = انسٹالر کو ایگزٹ کوڈ کے بغیر ختم کر دیا گیا
# { $code } is the number it ended with, such as 1603.
core-runtime-installer-failed = انسٹالر کوڈ { $code } کے ساتھ ختم ہوا
# { $feature } is a Windows feature's name, such as NetFx3; { $code } is the number DISM ended with. Windows Features is
# the name of a Windows window. Keep the command as it is.
core-runtime-feature-failed = DISM { $feature } Windows خصوصیت کو فعال نہیں کر سکا (ایگزٹ { $code })؛ غالباً پے لوڈ ڈاؤن لوڈ نہیں ہو سکا۔ اسے Windows خصوصیات کے ذریعے فعال کریں، یا `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess` چلائیں۔
# The same, when DISM gave no number.
core-runtime-feature-failed-no-code = DISM { $feature } Windows خصوصیت کو فعال نہیں کر سکا (ایگزٹ نامعلوم)؛ غالباً پے لوڈ ڈاؤن لوڈ نہیں ہو سکا۔ اسے Windows خصوصیات کے ذریعے فعال کریں، یا `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess` چلائیں۔
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-feature-timed-out = { $feature } Windows خصوصیت کو فعال کرنا مقررہ وقت میں مکمل نہیں ہوا اور اسے ختم کر دیا گیا
# { $feature } is a Windows feature's name; { $error } is what went wrong.
core-runtime-feature-no-dism = { $feature } Windows خصوصیت کو فعال کرنے کے لیے DISM دستیاب نہیں ہے: { $error }

## Installing a runtime with winget. { $id } is its name in winget, such as Microsoft.VCRedist.2015+.x64.

# { $error } is what went wrong.
core-runtime-winget-not-looked-for = winget تلاش نہیں کیا جا سکا: { $error }
core-runtime-winget-not-installed = winget انسٹال نہیں ہے
# { $path } is where winget is.
core-runtime-winget-misplaced = { $path } وہ جگہ نہیں ہے جہاں Windows اپنے پیکیجز رکھتا ہے، اس لیے اسے نہیں چلایا گیا
# { $path } is where winget is; { $reason } is what the check found.
core-runtime-winget-signature-failed = { $path } اپنے دستخط کی جانچ میں ناکام رہا اور اسے نہیں چلایا گیا: { $reason }
# { $reason } is one of the two messages above, or what went wrong.
core-runtime-winget-unusable = { $id } انسٹال کرنے کے لیے winget استعمال نہیں کیا جا سکا: { $reason }
# { $code } is the number winget ended with.
core-runtime-winget-failed = winget install { $id } کوڈ { $code } کے ساتھ ختم ہوا
core-runtime-winget-failed-no-code = winget install { $id } ایک نامعلوم کوڈ کے ساتھ ختم ہوا
core-runtime-winget-timed-out = winget install { $id } مقررہ وقت میں مکمل نہیں ہوا اور اسے ختم کر دیا گیا
# { $error } is what went wrong.
core-runtime-winget-not-started = { $id } انسٹال کرنے کے لیے winget دستیاب نہیں ہے: { $error }

## What told a scan how an app was installed, as the package's report lists it. The names of installers and files are
## as their makers write them.

# { $path } is an installer's record of what it installed, such as C:\Program Files\App\unins000.dat.
core-source-record-not-believed = { $path } استعمال نہیں کیا گیا: منتظمین کے علاوہ دوسرے اسے تبدیل کر سکتے ہیں
# { $count } is how many Windows Installer packages the bundle holds.
core-source-burn-bundle =
    { $count ->
        [one] { $count } Windows Installer پیکیج کا Burn بنڈل
       *[other] { $count } Windows Installer پیکیجز کا Burn بنڈل
    }
# { $count } is how many paths Windows Installer recorded.
core-source-msi-database =
    { $count ->
        [one] Windows Installer ڈیٹا بیس: { $count } جزو کا کلیدی پاتھ
       *[other] Windows Installer ڈیٹا بیس: { $count } اجزاء کے کلیدی پاتھ
    }
core-source-inno-log = Inno Setup ان انسٹال لاگ (unins000.dat)
# { $count } is how many paths the log records.
core-source-inno-log-paths =
    { $count ->
        [one] Inno Setup ان انسٹال لاگ: { $count } ریکارڈ شدہ پاتھ
       *[other] Inno Setup ان انسٹال لاگ: { $count } ریکارڈ شدہ پاتھ
    }
core-source-nsis = NSIS (Nullsoft) ان انسٹالر کی شناختی علامت
# { $count } is how many paths the log records.
core-source-nsis-log-paths =
    { $count ->
        [one] NSIS AdvUninstLog uninstall.log: { $count } ریکارڈ شدہ پاتھ
       *[other] NSIS AdvUninstLog uninstall.log: { $count } ریکارڈ شدہ پاتھ
    }
core-source-squirrel = Squirrel کی فی صارف ساخت (Update.exe / app-*)
# { $version } is a folder's name that says the version, such as app-1.2.3.
core-source-squirrel-version = Squirrel کا موجودہ ورژن: { $version }
core-source-chocolatey = Chocolatey پیکیج میٹا ڈیٹا
core-source-msix = MSIX / Appx پیکیج
# { $identity } is a package's name, such as Contoso.App.
core-source-appx-identity = AppxManifest شناخت: { $identity }
# { $count } is how many folders.
core-source-msix-data =
    { $count ->
        [one] MSIX فی صارف پیکیج ڈیٹا: { $count } مقام
       *[other] MSIX فی صارف پیکیج ڈیٹا: { $count } مقامات
    }
# App Paths is a registry key's name.
core-source-app-paths = App Paths رجسٹریشن

## Why a runtime is needed, as the package's report says it.

# { $program } is a program's file name, such as app.exe; { $dll } is a DLL's, such as msvcp140.dll.
core-runtime-program-imports = { $program } { $dll } امپورٹ کرتا ہے

## What else of an app's a scan finds in the registry: fonts and certificates, and why each is the app's.

# Why a font is among the app's files, in the package's list of them.
core-registry-font-recorded = ایک فونٹ جس کا نام ایپ کے تنصیب کے ریکارڈ میں ہے
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-named-for-the-app = اس کا نام ایپ یا اس کے ناشر کے نام پر ہے
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-recorded = ایپ کے تنصیب کے ریکارڈ میں اس کا نام ہے

## Who is signed in, for the Whose Data choice. Each of the first six is why it could not be told, and starts with a
## small letter, as it is shown inside other words. An account is shown by its id, such as S-1-5-21-...-1001.

# { $error } is what Windows said.
core-profile-desktop-owner-not-said = Windows نے نہیں بتایا کہ یہ ڈیسک ٹاپ کس کا ہے: { $error }
# { $error } is what went wrong.
core-profile-desktop-owner-not-asked = Windows سے یہ نہیں پوچھا جا سکا کہ یہ ڈیسک ٹاپ کس کا ہے: { $error }
core-profile-signed-in-account-no-profile = یہاں سائن ان اکاؤنٹ، { $account }، کی کوئی پروفائل ریکارڈ میں نہیں ہے
core-profile-desktop-not-a-person = یہاں کا ڈیسک ٹاپ { $account } کا ہے، جو کسی شخص کا اکاؤنٹ نہیں ہے
core-profile-nobody-signed-in-running-account-no-profile = یہاں کوئی سائن ان نہیں ہے، اور اسے چلانے والے اکاؤنٹ، { $account }، کی کوئی پروفائل ریکارڈ میں نہیں ہے
core-profile-nobody-signed-in-running-not-a-person = یہاں کوئی سائن ان نہیں ہے، اور { $account }، جو اسے چلا رہا ہے، کسی شخص کا اکاؤنٹ نہیں ہے
# SID is the name Windows gives an account's id.
core-profile-nobody-signed-in-no-sid = یہاں کوئی سائن ان نہیں ہے، اور اس اکاؤنٹ کا کوئی SID نہیں ہے
# { $accounts } is a list of accounts.
core-profile-several-desktops = اس سیشن میں ایک سے زیادہ اکاؤنٹس کا ڈیسک ٹاپ ہے: { $accounts }
# Under the Whose Data choice. { $reason } is one of the messages above.
core-profile-signed-in-unknown = یہ معلوم نہیں ہو سکا کہ کون سائن ان ہے ({ $reason })۔
# Under the Whose Data choice, when there are accounts to choose from. { $reason } is one of the messages above.
core-profile-signed-in-unknown-choose = یہ معلوم نہیں ہو سکا کہ کون سائن ان ہے ({ $reason })۔ منتخب کریں کہ کس کا ڈیٹا شامل کرنا ہے۔
