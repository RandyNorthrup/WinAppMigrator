# Words the core writes for people: warnings, refusals, errors, and the installer's list of what it sets up.
# Message names start with core-.


## Warnings a package records when it is built, which an installer shows later, maybe on another PC, in that PC's
## language.

# { $count } is how many shortcuts.
core-warning-shortcuts-unreadable =
    { $count ->
        [one] { $count } ярлик на цьому диску не вказує на жоден файл, тому не вдалося визначити, чи відкриває він цю програму.
        [few] { $count } ярлики на цьому диску не вказують на жоден файл, тому не вдалося визначити, чи відкривають вони цю програму.
        [many] { $count } ярликів на цьому диску не вказують на жоден файл, тому не вдалося визначити, чи відкривають вони цю програму.
       *[other] { $count } ярликів на цьому диску не вказують на жоден файл, тому не вдалося визначити, чи відкривають вони цю програму.
    }

## Undoing an install: what an uninstall, or a failed install, says it put back, left, or could not undo. Each line is
## shown on its own, in a list.

# { $app_name } is the app's name.
core-rollback-record-damaged = Не вдалося прочитати частину запису програми { $app_name }, тому все, що в ній описано, залишилося на місці, а файл, який замінила інсталяція, може не повернутися.
core-rollback-settings-entry-stays = Запис програми в «Параметрах» залишається, щоб цю дію можна було знову запустити звідти.
# { $name } is a runtime's name, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-rollback-prerequisite-stays = { $name } інстальовано (або, можливо, інстальовано), і цей компонент залишається, оскільки його можуть використовувати інші програми.
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-drive-not-connected = Те, що ця інсталяція розмістила в { $folder }, міститься на диску, який не підключено. Підключіть його, і цю дію можна буде завершити.
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-record-part-unread = Те, що ця інсталяція розмістила в { $folder }, залишається, оскільки цю частину запису не вдалося прочитати.

## Why a file or folder was left, shown after "was left:" in the messages below them. Each is part of a sentence, so
## starts with a small letter and has no full stop.

# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-being-changed = під час роботи щось змінювало { $folder }, тому в цій папці більше нічого не робилося. Закрийте те, що це робить, і запустіть дію знову
# { $folder } is a folder; { $error } is what Windows said went wrong.
core-rollback-reason-not-opened = не вдалося відкрити { $folder }: { $error }
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-no-record = немає запису про те, де було розташовано { $folder }
# { $folder } is a folder; { $actual } is the folder it now leads to.
core-rollback-reason-moved = { $folder } тепер веде до { $actual }, а не туди, куди вело під час інсталяції
# Why a service was not stopped.
core-rollback-reason-did-not-stop = вона не зупинилася вчасно

## Files and folders

# { $path } is a folder; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-folder-left = Папку { $path } залишено: { $reason }.
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-removed = Видалено папку { $path }
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-holds-others = Папка { $path } залишається, оскільки містить файли, які розмістила там не ця інсталяція.
# { $path } is a folder; { $error } is what Windows said went wrong.
core-rollback-folder-not-removed = Не вдалося видалити папку { $path }: { $error }.
# { $path } is a file; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-file-left = Файл { $path } залишено: { $reason }.
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-folder-gone = { $path } не повертається, оскільки папки, у якій він був, уже немає.
# { $path } is a file; { $error } is what went wrong.
core-rollback-partial-not-removed = Не вдалося видалити частковий файл: { $path } ({ $error }).
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-checked = Не вдалося перевірити { $path }: { $error }.
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-removed = Видалено { $path }
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-removed = Не вдалося видалити { $path }: { $error }.
# { $path } is a file this install had replaced, and has put back.
core-rollback-file-put-back = Повернуто { $path }
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-put-back = Не вдалося повернути { $path }: { $error }.
# Why a file could not be put back.
core-rollback-backup-gone = резервної копії вже немає
# { $path } is a file.
core-rollback-no-parent-folder = { $path } не має батьківської папки

## A file that stays. { $why } says why: still-used (something that stays still uses it), changed (it has changed since
## it was installed), or not-asked (it is someone's data, and they did not ask for it to go). { $path } is the file.

core-rollback-file-stays =
    { $why ->
        [still-used] { $path } залишається, оскільки його досі використовує те, що залишається.
        [changed] { $path } залишається, оскільки його змінено після інсталяції.
       *[not-asked] { $path } залишається, оскільки його видалення не запитувалося.
    }
# The file it replaced was to be put beside it, under another name, and every name was taken.
core-rollback-file-stays-no-name-beside =
    { $why ->
        [still-used] { $path } залишається, оскільки його досі використовує те, що залишається, а файл, який він замінив, не вдалося розмістити поруч: усі можливі імена для нього зайнято.
        [changed] { $path } залишається, оскільки його змінено після інсталяції, а файл, який він замінив, не вдалося розмістити поруч: усі можливі імена для нього зайнято.
       *[not-asked] { $path } залишається, оскільки його видалення не запитувалося, а файл, який він замінив, не вдалося розмістити поруч: усі можливі імена для нього зайнято.
    }
# { $name } is the name the file it replaced now has, such as settings.ini.before-install.
core-rollback-file-stays-beside =
    { $why ->
        [still-used] { $path } залишається, оскільки його досі використовує те, що залишається. Файл, який він замінив, розміщено поруч під іменем { $name }.
        [changed] { $path } залишається, оскільки його змінено після інсталяції. Файл, який він замінив, розміщено поруч під іменем { $name }.
       *[not-asked] { $path } залишається, оскільки його видалення не запитувалося. Файл, який він замінив, розміщено поруч під іменем { $name }.
    }
# { $error } is what went wrong.
core-rollback-file-stays-not-beside =
    { $why ->
        [still-used] { $path } залишається, оскільки його досі використовує те, що залишається, а файл, який він замінив, не вдалося розмістити поруч: { $error }.
        [changed] { $path } залишається, оскільки його змінено після інсталяції, а файл, який він замінив, не вдалося розмістити поруч: { $error }.
       *[not-asked] { $path } залишається, оскільки його видалення не запитувалося, а файл, який він замінив, не вдалося розмістити поруч: { $error }.
    }

## The registry and environment variables. { $kind } says what { $name } is: key (a registry key, such as
## HKLM\Software\App), value (a registry value, such as HKLM\Software\App\Version), user-variable or machine-variable
## (an environment variable of the user's or of the whole PC, such as APP_HOME), or user-list or machine-list (a
## variable that is a list of folders, such as Path). { $error } is what Windows said went wrong.

# How a registry key's unnamed value is shown, as Registry Editor shows it.
core-registry-default-value-name = (За замовчуванням)
core-rollback-setting-other-account =
    { $kind ->
        [key] Розділ { $name } залишається, оскільки належить обліковому запису, який виконав цю інсталяцію і зараз не ввійшов у систему.
        [user-variable] Змінна середовища користувача { $name } залишається, оскільки належить обліковому запису, який виконав цю інсталяцію і зараз не ввійшов у систему.
        [machine-variable] Системна змінна середовища { $name } залишається, оскільки належить обліковому запису, який виконав цю інсталяцію і зараз не ввійшов у систему.
        [user-list] Змінна користувача { $name } залишається, оскільки належить обліковому запису, який виконав цю інсталяцію і зараз не ввійшов у систему.
        [machine-list] Системна змінна { $name } залишається, оскільки належить обліковому запису, який виконав цю інсталяцію і зараз не ввійшов у систему.
       *[value] Параметр реєстру { $name } залишається, оскільки належить обліковому запису, який виконав цю інсталяцію і зараз не ввійшов у систему.
    }
core-rollback-setting-not-reached =
    { $kind ->
        [key] Не вдалося отримати доступ до розділу { $name }: { $error }.
        [user-variable] Не вдалося отримати доступ до змінної середовища користувача { $name }: { $error }.
        [machine-variable] Не вдалося отримати доступ до системної змінної середовища { $name }: { $error }.
        [user-list] Не вдалося отримати доступ до змінної користувача { $name }: { $error }.
        [machine-list] Не вдалося отримати доступ до системної змінної { $name }: { $error }.
       *[value] Не вдалося отримати доступ до параметра реєстру { $name }: { $error }.
    }
core-rollback-setting-not-read =
    { $kind ->
        [key] Не вдалося прочитати розділ { $name }: { $error }.
        [user-variable] Не вдалося прочитати змінну середовища користувача { $name }: { $error }.
        [machine-variable] Не вдалося прочитати системну змінну середовища { $name }: { $error }.
        [user-list] Не вдалося прочитати змінну користувача { $name }: { $error }.
        [machine-list] Не вдалося прочитати системну змінну { $name }: { $error }.
       *[value] Не вдалося прочитати параметр реєстру { $name }: { $error }.
    }
core-rollback-setting-changed =
    { $kind ->
        [user-variable] Змінна середовища користувача { $name } залишається, оскільки її змінено після інсталяції.
        [machine-variable] Системна змінна середовища { $name } залишається, оскільки її змінено після інсталяції.
        [user-list] Змінна користувача { $name } залишається, оскільки її змінено після інсталяції.
        [machine-list] Системна змінна { $name } залишається, оскільки її змінено після інсталяції.
       *[value] Параметр реєстру { $name } залишається, оскільки його змінено після інсталяції.
    }
# Said when it is back as it was before the install.
core-rollback-setting-put-back =
    { $kind ->
        [user-variable] Повернуто змінну середовища користувача { $name }
        [machine-variable] Повернуто системну змінну середовища { $name }
        [user-list] Повернуто змінну користувача { $name }
        [machine-list] Повернуто системну змінну { $name }
       *[value] Повернуто параметр реєстру { $name }
    }
core-rollback-setting-not-put-back =
    { $kind ->
        [user-variable] Не вдалося повернути змінну середовища користувача { $name }: { $error }.
        [machine-variable] Не вдалося повернути системну змінну середовища { $name }: { $error }.
        [user-list] Не вдалося повернути змінну користувача { $name }: { $error }.
        [machine-list] Не вдалося повернути системну змінну { $name }: { $error }.
       *[value] Не вдалося повернути параметр реєстру { $name }: { $error }.
    }
# A registry value the install wrote, which is the user's data, and stays unless they asked for it to go.
core-rollback-value-kept-by-request = Параметр реєстру { $name } залишається, оскільки його видалення не запитувалося.
core-rollback-key-holds-others = Розділ { $name } залишається, оскільки містить те, що додала не ця інсталяція.
core-rollback-key-removed = Видалено розділ { $name }
core-rollback-key-not-removed = Не вдалося видалити розділ { $name }: { $error }.
# { $kind } is user-list or machine-list.
core-rollback-list-entries-removed =
    { $kind ->
        [user-list] Видалено записи цієї інсталяції зі змінної користувача { $name }
       *[machine-list] Видалено записи цієї інсталяції із системної змінної { $name }
    }
core-rollback-list-entries-not-removed =
    { $kind ->
        [user-list] Не вдалося видалити записи цієї інсталяції зі змінної користувача { $name }: { $error }.
       *[machine-list] Не вдалося видалити записи цієї інсталяції із системної змінної { $name }: { $error }.
    }

## Services and drivers. { $kind } is service or driver; { $name } is its name, such as AppUpdater. { $error } and
## { $reason } are what Windows said went wrong.

core-rollback-service-not-read =
    { $kind ->
        [driver] Не вдалося прочитати драйвер { $name }: { $error }.
       *[service] Не вдалося прочитати службу { $name }: { $error }.
    }
core-rollback-service-changed =
    { $kind ->
        [driver] Драйвер { $name } залишається, оскільки його змінено після інсталяції.
       *[service] Служба { $name } залишається, оскільки її змінено після інсталяції.
    }
core-rollback-service-not-stopped =
    { $kind ->
        [driver] Не вдалося зупинити драйвер { $name } ({ $reason }), тому те, що він використовує, може залишатися до перезавантаження ПК.
       *[service] Не вдалося зупинити службу { $name } ({ $reason }), тому те, що вона використовує, може залишатися до перезавантаження ПК.
    }
core-rollback-service-stopped-for-this =
    { $kind ->
        [driver] Драйвер { $name } зупинено для виконання цієї дії. Запустіть його знову, коли він знадобиться.
       *[service] Службу { $name } зупинено для виконання цієї дії. Запустіть її знову, коли вона знадобиться.
    }
core-rollback-service-not-removed =
    { $kind ->
        [driver] Не вдалося видалити драйвер { $name }: { $reason }.
       *[service] Не вдалося видалити службу { $name }: { $reason }.
    }
core-rollback-service-marked-for-removal =
    { $kind ->
        [driver] Драйвер { $name } позначено для видалення; його буде видалено після перезавантаження ПК або коли його звільнить те, що його використовує.
       *[service] Службу { $name } позначено для видалення; її буде видалено після перезавантаження ПК або коли її звільнить те, що її використовує.
    }
core-rollback-service-removed =
    { $kind ->
        [driver] Видалено драйвер { $name }
       *[service] Видалено службу { $name }
    }
core-rollback-service-not-restored =
    { $kind ->
        [driver] Не вдалося повернути драйвер { $name } до попереднього стану: { $reason }.
       *[service] Не вдалося повернути службу { $name } до попереднього стану: { $reason }.
    }
core-rollback-service-restored =
    { $kind ->
        [driver] Повернуто параметри драйвера { $name }
       *[service] Повернуто параметри служби { $name }
    }
core-rollback-service-not-restarted = Служба { $name } працювала до цієї інсталяції, але не запустилася знову: { $reason }.
# { $account } is the account the service ran as, such as CONTOSO\backup.
core-rollback-service-account-again = Служба { $name } працювала від імені { $account }. Її пароль не зберігається, тому знову вкажіть її обліковий запис у «Службах».

## Firewall rules. { $name } is the rule's name; { $error } is what went wrong.

core-rollback-firewall-not-read = Не вдалося прочитати правило брандмауера { $name }: { $error }.
core-rollback-firewall-says-otherwise = Правило брандмауера { $name } залишається, оскільки наявне правило з такою назвою містить інші параметри.
core-rollback-firewall-changed = Правило брандмауера { $name } залишається, оскільки його змінено після інсталяції.
# The capital "The" is as the app has always said it.
core-rollback-firewall-removed = Видалено правило брандмауера { $name }
core-rollback-firewall-not-removed = Не вдалося видалити правило брандмауера { $name }: { $error }.

## Scheduled tasks. { $path } is a folder of tasks, and { $name } a task with its folder, such as \App\Nightly Update.
## { $error } and { $reason } are what went wrong.

core-rollback-task-folder-holds-others = Папка завдань { $path } залишається, оскільки містить завдання, які додала не ця інсталяція.
core-rollback-task-folder-not-removed = Не вдалося видалити папку завдань { $path }: { $error }.
core-rollback-task-not-read = Не вдалося прочитати завдання { $name }: { $error }.
core-rollback-task-changed = Завдання { $name } залишається, оскільки його змінено після інсталяції.
core-rollback-task-removed = Видалено завдання { $name }
core-rollback-task-not-removed = Не вдалося видалити завдання { $name }: { $reason }.
core-rollback-task-put-back = Повернуто завдання { $name }
core-rollback-task-not-put-back = Не вдалося повернути завдання { $name }: { $reason }.

## When an install cannot be undone at all

# { $error } is what went wrong.
core-rollback-record-unreadable = Не вдалося прочитати запис цієї інсталяції, тому нічого не скасовано: { $error }.
# { $error } is what went wrong.
core-rollback-nothing-undone = Нічого не скасовано: { $error }.
# One app's part of the message below. { $app_name } is the app's name; { $failures } is what could not be undone, a
# sentence or more; { $folder } is the folder that holds the install's record.
core-rollback-earlier-install-stuck = { $app_name }: { $failures } Якщо цю інсталяцію ніколи не вдасться скасувати, адміністратор може відмовитися від неї, видаливши { $folder }.
# Why an install did not start. { $details } is one or more of the message above.
core-rollback-earlier-installs-not-undone = попередню незавершену інсталяцію не вдалося повністю скасувати, тому нічого не інстальовано. Закрийте все, що використовує ці елементи, і повторіть спробу. { $details }
core-rollback-install-busy = цю інсталяцію змінює інший інсталятор або інша програма видалення

## The summary an uninstall writes on the command line, one line each, with what each says below it. { $count } is how
## many.

core-rollback-count-undone = Скасовано: { $count }
core-rollback-count-failed = Не вдалося скасувати: { $count }
# What was left on purpose: it has changed since the install, or it is the user's.
core-rollback-count-kept = Залишено на місці: { $count }
# What no uninstall can take back, such as a runtime that was installed, which other programs may use.
core-rollback-count-left-behind = Залишається в системі: { $count }

## Which Windows a PC or drive has, as the app shows it beside the drive.

# { $name } is the Windows' name, such as Windows 7 Professional Service Pack 1.
core-system-windows-32-bit = { $name }, 32-розрядна
# { $name } is the Windows' name, such as Windows 11 Pro.
core-system-windows-64-bit = { $name }, 64-розрядна
# A drive's Windows, when it only says it is older than Vista.
core-system-windows-xp-or-older = Windows XP або старіша

## Reading a connected drive, or another account's settings. Each is why it could not be done, and some start with a
## small letter, as they follow other words.

core-system-needs-administrator = для читання підключеного диска або параметрів іншого облікового запису потрібні права адміністратора
core-system-no-program-data = не вдалося знайти папку ProgramData на цьому ПК
# { $path } is a file, such as D:\Windows\System32\config\SOFTWARE.
core-system-not-a-hive-file = це не файл куща реєстру: { $path }
# { $path } is a file; { $size } is its size in bytes.
core-system-hive-too-large = розмір { $path } у байтах — { $size }; це завелико для куща реєстру або більше, ніж можуть разом займати кущі цього диска
# { $key } is a registry key, such as Software\Microsoft\Windows\CurrentVersion.
core-system-hive-no-mapping = для автономного куща немає зіставлення для HKLM\{ $key }
# { $key } is where a drive's registry was to be loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said.
# "reg load" is a command's name.
core-system-hive-not-loaded = помилка reg load { $key }: { $error }
# { $key } is where a drive's registry was loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said. "reg
# unload" is a command's name.
core-system-hive-not-unloaded = помилка reg unload { $key }: { $error }
# { $drive } is a drive, such as D:\.
core-system-not-a-system-drive = { $drive } не є системним диском Windows (немає папки \Windows).
# { $drive } is a drive, such as C:\. "This PC" is the name of a choice in the app.
core-system-own-system-drive = { $drive } — власний системний диск цього ПК. Щоб прочитати цей ПК, виберіть «Цей ПК».
# { $drive } is a drive, such as D:\.
core-system-registry-through-link = { $drive } не є системним диском Windows: шлях до його реєстру проходить через посилання.

## Warnings a scan records about whose data it read.

# { $account } is an account's name or id, such as alice or S-1-5-21-...-1001.
core-warning-scan-account-not-on-record = Обліковий запис { $account } тут не зареєстровано, тому дані й параметри жодного користувача не включено.
# { $account } is an account's name, such as CONTOSO\alice.
core-warning-scan-signed-in-account-no-profile = Обліковий запис, який тут увійшов у систему ({ $account }), не має зареєстрованого профілю, тому дані й параметри жодного користувача не включено.
# { $reason } is why Windows could not say.
core-warning-scan-signed-in-account-unknown = Не вдалося визначити, хто ввійшов у систему ({ $reason }), тому дані й параметри жодного користувача не включено. Виберіть, чиї дані включити.
core-warning-scan-last-account-unknown = На цьому диску не вказано, хто останнім входив у систему, а облікових записів на ньому кілька, тому дані й параметри жодного користувача не включено. Виберіть, чиї дані включити.
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-settings-unreadable = Не вдалося прочитати параметри облікового запису { $account }, тому їх не включено: { $error }
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-classes-unreadable = Не вдалося прочитати класи COM і типи файлів, зареєстровані для { $account }, тому їх не включено: { $error }

## Paths a scan did not follow. { $count } is how many; { $paths } is some of them, separated by semicolons; { $more } is
## how many more there are, not shown.

core-warning-scan-drive-paths-not-followed =
    { $count ->
        [1] Не вдалося прочитати шлях, записаний на цьому диску, тому нічого звідти не включено. Такий шлях вказує на інший диск, не є повним або проходить через посилання, яке вивело б за межі диска: { $paths }
        [one] Не вдалося прочитати деякі шляхи, записані на цьому диску, тому нічого звідти не включено. Кожен із них вказує на інший диск, не є повним або проходить через посилання, яке вивело б за межі диска: { $paths }
        [few] Не вдалося прочитати деякі шляхи, записані на цьому диску, тому нічого звідти не включено. Кожен із них вказує на інший диск, не є повним або проходить через посилання, яке вивело б за межі диска: { $paths }
        [many] Не вдалося прочитати деякі шляхи, записані на цьому диску, тому нічого звідти не включено. Кожен із них вказує на інший диск, не є повним або проходить через посилання, яке вивело б за межі диска: { $paths }
       *[other] Не вдалося прочитати деякі шляхи, записані на цьому диску, тому нічого звідти не включено. Кожен із них вказує на інший диск, не є повним або проходить через посилання, яке вивело б за межі диска: { $paths }
    }
core-warning-scan-drive-paths-not-followed-more = Не вдалося прочитати деякі шляхи, записані на цьому диску, тому нічого звідти не включено. Кожен із них вказує на інший диск, не є повним або проходить через посилання, яке вивело б за межі диска: { $paths } і ще { $more }
core-warning-scan-network-paths-not-followed =
    { $count ->
        [1] Не вдалося прочитати шлях, записаний на цьому ПК, тому нічого звідти не включено. Такий шлях або вказує не на диск цього ПК (і спроба переглянути його спричинила б підключення), або має ім'я, яке відкрило б щось інше: { $paths }
        [one] Не вдалося прочитати деякі шляхи, записані на цьому ПК, тому нічого звідти не включено. Кожен із них або вказує не на диск цього ПК (і спроба переглянути його спричинила б підключення), або має ім'я, яке відкрило б щось інше: { $paths }
        [few] Не вдалося прочитати деякі шляхи, записані на цьому ПК, тому нічого звідти не включено. Кожен із них або вказує не на диск цього ПК (і спроба переглянути його спричинила б підключення), або має ім'я, яке відкрило б щось інше: { $paths }
        [many] Не вдалося прочитати деякі шляхи, записані на цьому ПК, тому нічого звідти не включено. Кожен із них або вказує не на диск цього ПК (і спроба переглянути його спричинила б підключення), або має ім'я, яке відкрило б щось інше: { $paths }
       *[other] Не вдалося прочитати деякі шляхи, записані на цьому ПК, тому нічого звідти не включено. Кожен із них або вказує не на диск цього ПК (і спроба переглянути його спричинила б підключення), або має ім'я, яке відкрило б щось інше: { $paths }
    }
core-warning-scan-network-paths-not-followed-more = Не вдалося прочитати деякі шляхи, записані на цьому ПК, тому нічого звідти не включено. Кожен із них або вказує не на диск цього ПК (і спроба переглянути його спричинила б підключення), або має ім'я, яке відкрило б щось інше: { $paths } і ще { $more }

## What a scan with none of an account's data leaves out of a package, in the app's list of what was left out. Each
## has a name, and below it why.

core-account-left-out-shortcuts = Ярлики у власних папках цього облікового запису
# { $count } is how many shortcuts.
core-account-left-out-shortcuts-why =
    { $count ->
        [one] { $count } ярлик, який включається лише з параметром «Дані користувача»
        [few] { $count } ярлики, які включаються лише з параметром «Дані користувача»
        [many] { $count } ярликів, які включаються лише з параметром «Дані користувача»
       *[other] { $count } ярликів, які включаються лише з параметром «Дані користувача»
    }
core-account-left-out-variables = Змінні середовища цього облікового запису
# { $count } is how many variables.
core-account-left-out-variables-why =
    { $count ->
        [one] { $count } змінна, яка включається лише з параметром «Дані користувача»
        [few] { $count } змінні, які включаються лише з параметром «Дані користувача»
        [many] { $count } змінних, які включаються лише з параметром «Дані користувача»
       *[other] { $count } змінних, які включаються лише з параметром «Дані користувача»
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-account-left-out-task = Заплановане завдання { $task }
core-account-task-works-on-account-folder = Воно працює з розташуванням у папках облікового запису або назване за ним, тому включається лише з параметром «Дані користувача»
core-account-task-runs-from-account-folder = Те, що воно також запускає з папок облікового запису, включається лише з параметром «Дані користувача»
# { $service } is a service's name, such as AppUpdater.
core-account-left-out-service = Служба { $service }
core-account-service-works-on-account-folder = Вона працює з розташуванням у папках облікового запису, тому включається лише з параметром «Дані користувача»

## What a package was found to still say of whose PC it came from, after a build with none of an account's data.

# The parts of a package, as the warning below names them.
core-account-part-files = шляхи до власних файлів програми
core-account-part-registry = параметри реєстру
core-account-part-settings-entry = запис програми в «Параметрах»
core-account-part-main-program = розташування основного виконуваного файлу
core-account-part-shortcuts = ярлики
core-account-part-tasks = заплановані завдання
core-account-part-services = служби
core-account-part-variables = змінні середовища
# One place in the list below. { $place } is one of the parts above, or a file's name; { $count } is how many times.
core-account-mention-place = { $place } ({ $count })
# { $kind } is what was found: account (an account's name), folder (where an account's folder is), computer (the PC's
# name), or id (an account's id). { $places } is a list of where, from the message above.
core-warning-scan-package-mentions =
    { $kind ->
        [account] Цей пакет містить ім'я облікового запису в таких місцях: { $places }. Його створено без даних жодного облікового запису, і будь-хто, хто має інсталятор, може це прочитати.
        [folder] Цей пакет містить розташування папки облікового запису в таких місцях: { $places }. Його створено без даних жодного облікового запису, і будь-хто, хто має інсталятор, може це прочитати.
        [computer] Цей пакет містить ім'я ПК у таких місцях: { $places }. Його створено без даних жодного облікового запису, і будь-хто, хто має інсталятор, може це прочитати.
       *[id] Цей пакет містить ідентифікатор облікового запису в таких місцях: { $places }. Його створено без даних жодного облікового запису, і будь-хто, хто має інсталятор, може це прочитати.
    }
# { $count } is how many folders.
core-warning-scan-folders-of-others-unreadable =
    { $count ->
        [one] Не вдалося прочитати { $count } папку поза програмою, тому даних програми звідти може бракувати.
        [few] Не вдалося прочитати { $count } папки поза програмою, тому даних програми звідти може бракувати.
        [many] Не вдалося прочитати { $count } папок поза програмою, тому даних програми звідти може бракувати.
       *[other] Не вдалося прочитати { $count } папок поза програмою, тому даних програми звідти може бракувати.
    }

## What a scan leaves out because it looks like a credential, or is a license with the License option off, in the app's
## list of what was left out. Each has a name, and below it why.

core-secret-why-credential = це схоже на облікові дані
core-secret-why-license-option-off = параметр «Дані ліцензії та активації» вимкнуто
# { $count } is how many values of a registry key.
core-secret-why-account-values =
    { $count ->
        [one] { $count } параметр, який включається лише з параметром «Дані користувача»
        [few] { $count } параметри, які включаються лише з параметром «Дані користувача»
        [many] { $count } параметрів, які включаються лише з параметром «Дані користувача»
       *[other] { $count } параметрів, які включаються лише з параметром «Дані користувача»
    }
# Shown in place of a registry value's name when its name itself looks like a password or a key.
core-secret-name-holds-credential = (параметр, ім'я якого містить облікові дані)
# Shown in place of an option's name, for an argument on a command line that has no name.
core-secret-unnamed-argument = аргумент, що містить облікові дані
# { $option } is an option taken out of a command line, such as /password, or the message above. { $place } says
# where the command line was: service, task, shortcut, opener (what opens a file type), opener-unnamed (the same, of a
# type with no name), or uninstall. { $name } is the service, task, shortcut, file type, or app.
core-secret-taken-from =
    { $place ->
        [service] { $option } у команді запуску { $name }
        [task] { $option } у команді завдання { $name }
        [shortcut] { $option } в ярлику { $name }
        [opener] { $option } у команді, яка відкриває { $name }
        [opener-unnamed] { $option } у команді, яка відкриває тип файлу
       *[uninstall] { $option } у команді видалення { $name }
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-secret-task-definition-unreadable = Визначення завдання { $task }, яке не вдалося прочитати для перевірки
# { $name } is an environment variable's name, such as APP_TOKEN.
core-secret-left-out-variable = Змінна середовища { $name }
# { $class } is a COM class's id, such as {00000000-0000-0000-0000-000000000000}.
core-secret-left-out-class-server = Те, що запускає клас { $class } за його власними даними
# { $rule } is a firewall rule's name.
core-secret-left-out-firewall-description = Опис правила брандмауера { $rule }

## Licenses a scan found, as the package's report describes each.

# { $name } is a file's name, such as license.lic.
core-license-detail-file = Виявлено файл ліцензії: { $name }
# { $framework } is a licensing product's name, such as FlexLM / FlexNet.
core-license-detail-store = Виявлено сховище ліцензій { $framework }
# { $name } is an environment variable's name, such as LM_LICENSE_FILE.
core-license-detail-variable = Змінна середовища ліцензії { $name }
# { $name } is a registry value's name, such as SerialNumber.
core-license-detail-registry-value = Параметр реєстру, імовірно, містить дані ліцензування: { $name }
# Said of a registry value in an account's key, which is not named.
core-license-detail-registry-value-of-an-account = Параметр реєстру, імовірно, містить дані ліцензування
# { $key } is a registry key, such as HKLM\Software\Contoso\License.
core-license-detail-registry-key = Розділ реєстру, імовірно, містить дані ліцензування: { $key }

## Warnings a scan records about licenses.

core-warning-scan-license-data = Виявлено дані ліцензування або активації. Якщо параметр «Дані ліцензії та активації» ввімкнуто, інсталятор відновлює те, що можна перенести; про значення для всього комп'ютера та сховища, прив'язані до обладнання, лише повідомляється. WinAppMigrator ніколи не розшифровує дані, не обходить активацію й не емулює апаратні ключі захисту.
core-warning-scan-license-machine-bound = Деякі ліцензії прив'язано до комп'ютера (апаратний ключ захисту, активація з прив'язкою до комп'ютера або шифрування). Інсталятор відновлює те, що може; на новому ПК для них усе одно може знадобитися повторна активація.

## Warnings a scan records about the registry. What could not be read is not named in them: it is kept beside them.

# { $error } is what Windows said went wrong. App Paths is a registry key's name.
core-warning-scan-app-paths-key-unreadable = Не вдалося прочитати дочірній розділ App Paths: { $error }
core-warning-scan-registry-limit = Отримання даних реєстру досягло налаштованого безпечного обмеження.
core-warning-scan-registry-subtree-limit = Отримання даних гілки реєстру досягло налаштованого безпечного обмеження.
# { $error } is what Windows said went wrong.
core-warning-scan-registry-key-unreadable = Не вдалося прочитати розділ реєстру: { $error }
core-warning-scan-registry-value-too-large = Параметр реєстру перевищив налаштоване обмеження розміру для отримання.

## Scheduled tasks

core-warning-scan-tasks-folder-through-link = Шлях до папки запланованих завдань на цьому диску проходить через посилання, тому завдання з неї не прочитано.
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-warning-scan-task-left-out = Заплановане завдання { $task } не включено: воно виконується від імені іншого облікового запису, ніж той, чиї параметри включаються, його файл можуть змінювати не лише адміністратори, або програма має забагато завдань, щоб зберегти всі.
# { $error } is what Windows said went wrong.
core-warning-scan-tasks-folder-unreadable = Не вдалося прочитати частину запланованих завдань: { $error }
# { $path } is a task's file, such as C:\Windows\System32\Tasks\App\Nightly Sync; { $reason } is one of the reasons
# below, or what Windows said went wrong.
core-warning-scan-task-unreadable = Не вдалося прочитати заплановане завдання { $path }: { $reason }. Якщо воно запускає програму, його не включено.
# Why a task's file could not be read. { $size } is its size in bytes.
core-task-too-large = розмір файлу в байтах ({ $size }) завеликий для завдання
# Why a task's file could not be read.
core-task-ends-mid-character = файл обривається посеред символу
# Why a task's file could not be read. { $error } says where.
core-task-not-utf16 = вміст не є допустимим UTF-16: { $error }
# Why a task's file could not be read. { $error } says where.
core-task-not-text = вміст не є допустимим UTF-8 або UTF-16: { $error }
# What a package records of how a task is put back. Register-ScheduledTask is a command's name.
core-task-restore-instructions = Під час відновлення використовується Register-ScheduledTask з архівованим XML-визначенням.

## Reading a program's file. Each is why it could not be read, after the program's name.

# { $reason } is what went wrong.
core-pe-unreadable = його не вдалося прочитати: { $reason }
core-pe-headers-misaligned = його заголовки не вирівняно за межами, обов'язковими для програм
# { $size } is the file's size in bytes.
core-pe-too-large = розмір файлу в байтах ({ $size }) завеликий для програми
# PE is the name of the format Windows programs are in.
core-pe-no-headers = у ньому немає заголовків PE

## Warnings a scan records about what a program needs. Which program is kept beside each.

# { $error } is what went wrong.
core-warning-scan-imports-unreadable = Не вдалося прочитати таблицю імпорту: { $error }
# { $error } is what went wrong.
core-warning-scan-import-name-unreadable = Не вдалося прочитати ім'я дескриптора імпорту: { $error }
# { $count } is how many were skipped.
core-warning-scan-legacy-delay-imports =
    { $count ->
        [one] Пропущено { $count } застарілий (v1) дескриптор відкладеного завантаження; аналізуються лише сучасні відкладені імпорти на основі RVA.
        [few] Пропущено { $count } застарілі (v1) дескриптори відкладеного завантаження; аналізуються лише сучасні відкладені імпорти на основі RVA.
        [many] Пропущено { $count } застарілих (v1) дескрипторів відкладеного завантаження; аналізуються лише сучасні відкладені імпорти на основі RVA.
       *[other] Пропущено { $count } застарілих (v1) дескрипторів відкладеного завантаження; аналізуються лише сучасні відкладені імпорти на основі RVA.
    }

## The DLLs a program needs. Which DLL or program is kept beside each warning.

# { $path } is a program, such as C:\Program Files\App\app.exe.
core-dll-program-has-no-folder = виконуваний файл не має батьківської папки: { $path }
# Why a DLL is among the app's files, in the package's list of them.
core-dll-beside-program = DLL розташована поруч із вибраним виконуваним файлом
# { $error } is what went wrong.
core-warning-scan-dependency-unscanned = Не вдалося просканувати рекурсивну залежність: { $error }
# { $name } is a DLL's name, such as d3dx9_43.dll.
core-warning-scan-dependency-not-in-newer-windows = { $name } був у папці Windows на ПК, де працювала ця програма, а Windows 10 і 11 не містять цього файлу. Його туди інсталювало щось інше, тому він не належить до власних файлів програми й не переноситься. Програма може не запуститися, доки не буде інстальовано те, що його надає.
# { $error } is what went wrong.
core-warning-scan-dependency-not-hashed = Не вдалося обчислити хеш залежності: { $error }
# SystemRoot and windir are the names of environment variables.
core-warning-scan-windows-folder-unknown = Змінні середовища SystemRoot/windir недоступні; пошук системних DLL обмежено.
# { $error } is what went wrong.
core-warning-scan-dependency-architecture-unreadable = Не вдалося прочитати архітектуру залежності: { $error }

## Services, drivers, shortcuts, and environment variables a scan reads. Where it was reading is kept beside each
## warning.

# { $error } is what Windows said went wrong.
core-warning-scan-services-unreadable = Не вдалося прочитати розділ реєстру служб: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-drivers-unreadable = Не вдалося прочитати розділ реєстру служб для виявлення драйверів: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-queried = Не вдалося отримати відомості про ярлики: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-parsed = Не вдалося проаналізувати результат запиту ярликів: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-environment-unreadable = Не вдалося прочитати розділ реєстру із системними змінними середовища: { $error }

## What a package records of how a driver is put back, which an installer shows. { $kind } is the driver's type as
## Windows names it (kernel, file-system, recognizer, or unknown-type); { $name } is its service's name; { $path } is
## its file. { $signed } is yes when { $signer } names who signed it, and no when that is not known.

core-service-driver-bundled =
    { $signed ->
        [yes] Драйвер типу { $kind } «{ $name }», що належить програмі ({ $path }, підписувач: { $signer }). Інсталятор включає його файл .sys і автоматично повторно реєструє службу драйвера (sc create, потрібні підвищені права), а потім перевіряє, чи службу зареєстровано.
       *[no] Драйвер типу { $kind } «{ $name }», що належить програмі ({ $path }, підпис невідомий). Інсталятор включає його файл .sys і автоматично повторно реєструє службу драйвера (sc create, потрібні підвищені права), а потім перевіряє, чи службу зареєстровано.
    }
core-service-driver-from-vendor =
    { $signed ->
        [yes] Драйвер постачальника або вбудований драйвер Windows типу { $kind } «{ $name }» ({ $path }, підписувач: { $signer }). WinAppMigrator не поширює сторонні драйвери; перш ніж запускати програму, інсталюйте його повторно на цільовому ПК з підписаного пакета драйвера постачальника.
       *[no] Драйвер постачальника або вбудований драйвер Windows типу { $kind } «{ $name }» ({ $path }, підпис невідомий). WinAppMigrator не поширює сторонні драйвери; перш ніж запускати програму, інсталюйте його повторно на цільовому ПК з підписаного пакета драйвера постачальника.
    }

## Runtimes an app needs, such as the Visual C++ runtime or .NET: how each is provided, as the Dependencies list says.

core-runtime-handling-built-in = Вбудовано в Windows
core-runtime-handling-ships-with-app = Постачається з програмою
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-handling-windows-feature = Компонент Windows: { $feature }
core-runtime-handling-included = Включено
# { $id } is the runtime's name in winget, such as Microsoft.VCRedist.2015+.x64.
core-runtime-handling-winget = Інсталюється за допомогою winget: { $id }
core-runtime-handling-manual = Інсталювати вручну

## Downloading a runtime's installer, when an installer is built. Each is why it could not be, and starts with a small
## letter, as it follows other words. { $url } is a web address; { $error } is what went wrong.

core-runtime-download-not-https = відмовлено в завантаженні { $url }: це не HTTPS
# TLS is what keeps a download private.
core-runtime-tls-failed = не вдалося ініціалізувати TLS: { $error }
core-runtime-download-failed = не вдалося завантажити { $url }: { $error }
core-runtime-download-unreadable = не вдалося прочитати { $url }: { $error }
core-runtime-download-empty = завантаження { $url } не повернуло даних
# { $came } and { $expected } are numbers of bytes.
core-runtime-download-cut-short = завантаження { $url } перервано: отримано байтів — { $came } з { $expected }

## Installing a runtime: why it was not installed. Each starts with a small letter, as it follows other words.

core-runtime-no-known-installer = для цього середовища виконання немає відомого інсталятора
# { $path } is where in the package, such as prerequisites\vc_redist.x64.exe.
core-runtime-payload-misplaced = пакет указує { $path } як інсталятор цього середовища виконання, але цей файл розміщено не там, де слід, тому його не запущено
# { $path } is a file.
core-runtime-payload-missing = вбудованого інсталятора немає: { $path }
# { $error } is what went wrong.
core-runtime-payload-path-invalid = неприпустимий шлях до вбудованого інсталятора: { $error }
# { $url } is the web address it can be downloaded from. winget is a program's name.
core-runtime-no-installer-route = немає ні вбудованого інсталятора, ні ідентифікатора winget; завантажте його звідси: { $url }
# { $error } is what went wrong.
core-runtime-no-temp-folder = не вдалося створити папку для розпакування: { $error }
# { $reason } is what the check found.
core-runtime-payload-signature-failed = вбудований інсталятор не пройшов перевірку підпису, і його не запущено: { $reason }
core-runtime-payload-timed-out = вбудований інсталятор не завершив роботу вчасно, і його примусово зупинено
# { $error } is what went wrong.
core-runtime-payload-not-started = не вдалося запустити вбудований інсталятор: { $error }
core-runtime-installer-killed = інсталятор примусово зупинено без коду виходу
# { $code } is the number it ended with, such as 1603.
core-runtime-installer-failed = інсталятор завершив роботу з кодом { $code }
# { $feature } is a Windows feature's name, such as NetFx3; { $code } is the number DISM ended with. Windows Features is
# the name of a Windows window. Keep the command as it is.
core-runtime-feature-failed = DISM не вдалося ввімкнути компонент Windows { $feature } (код виходу { $code }); імовірно, не вдалося завантажити його файли. Увімкніть його у вікні «Компоненти Windows» або виконайте команду `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`.
# The same, when DISM gave no number.
core-runtime-feature-failed-no-code = DISM не вдалося ввімкнути компонент Windows { $feature } (код виходу невідомий); імовірно, не вдалося завантажити його файли. Увімкніть його у вікні «Компоненти Windows» або виконайте команду `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`.
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-feature-timed-out = увімкнення компонента Windows { $feature } не завершилося вчасно, і його примусово зупинено
# { $feature } is a Windows feature's name; { $error } is what went wrong.
core-runtime-feature-no-dism = DISM недоступний для ввімкнення компонента Windows { $feature }: { $error }

## Installing a runtime with winget. { $id } is its name in winget, such as Microsoft.VCRedist.2015+.x64.

# { $error } is what went wrong.
core-runtime-winget-not-looked-for = не вдалося виконати пошук winget: { $error }
core-runtime-winget-not-installed = winget не інстальовано
# { $path } is where winget is.
core-runtime-winget-misplaced = { $path } розміщено не там, де Windows зберігає власні пакети, тому його не запущено
# { $path } is where winget is; { $reason } is what the check found.
core-runtime-winget-signature-failed = { $path } не пройшов перевірку підпису, і його не запущено: { $reason }
# { $reason } is one of the two messages above, or what went wrong.
core-runtime-winget-unusable = не вдалося використати winget для інсталяції { $id }: { $reason }
# { $code } is the number winget ended with.
core-runtime-winget-failed = winget install { $id } завершився з кодом { $code }
core-runtime-winget-failed-no-code = winget install { $id } завершився з невідомим кодом
core-runtime-winget-timed-out = winget install { $id } не завершився вчасно, і його примусово зупинено
# { $error } is what went wrong.
core-runtime-winget-not-started = winget недоступний для інсталяції { $id }: { $error }

## What told a scan how an app was installed, as the package's report lists it. The names of installers and files are
## as their makers write them.

# { $path } is an installer's record of what it installed, such as C:\Program Files\App\unins000.dat.
core-source-record-not-believed = { $path } не використано: його можуть змінювати не лише адміністратори
# { $count } is how many Windows Installer packages the bundle holds.
core-source-burn-bundle =
    { $count ->
        [one] Набір Burn з { $count } пакетом Windows Installer
        [few] Набір Burn з { $count } пакетами Windows Installer
        [many] Набір Burn з { $count } пакетами Windows Installer
       *[other] Набір Burn з { $count } пакетами Windows Installer
    }
# { $count } is how many paths Windows Installer recorded.
core-source-msi-database =
    { $count ->
        [one] База даних Windows Installer: { $count } ключовий шлях компонента
        [few] База даних Windows Installer: { $count } ключові шляхи компонентів
        [many] База даних Windows Installer: { $count } ключових шляхів компонентів
       *[other] База даних Windows Installer: { $count } ключових шляхів компонентів
    }
core-source-inno-log = Журнал видалення Inno Setup (unins000.dat)
# { $count } is how many paths the log records.
core-source-inno-log-paths =
    { $count ->
        [one] Журнал видалення Inno Setup: { $count } записаний шлях
        [few] Журнал видалення Inno Setup: { $count } записані шляхи
        [many] Журнал видалення Inno Setup: { $count } записаних шляхів
       *[other] Журнал видалення Inno Setup: { $count } записаних шляхів
    }
core-source-nsis = Підпис програми видалення NSIS (Nullsoft)
# { $count } is how many paths the log records.
core-source-nsis-log-paths =
    { $count ->
        [one] NSIS AdvUninstLog uninstall.log: { $count } записаний шлях
        [few] NSIS AdvUninstLog uninstall.log: { $count } записані шляхи
        [many] NSIS AdvUninstLog uninstall.log: { $count } записаних шляхів
       *[other] NSIS AdvUninstLog uninstall.log: { $count } записаних шляхів
    }
core-source-squirrel = Структура Squirrel для окремого користувача (Update.exe / app-*)
# { $version } is a folder's name that says the version, such as app-1.2.3.
core-source-squirrel-version = Поточна версія Squirrel: { $version }
core-source-chocolatey = Метадані пакета Chocolatey
core-source-msix = Пакет MSIX / Appx
# { $identity } is a package's name, such as Contoso.App.
core-source-appx-identity = Ідентифікатор AppxManifest: { $identity }
# { $count } is how many folders.
core-source-msix-data =
    { $count ->
        [one] Дані пакета MSIX для окремого користувача: { $count } розташування
        [few] Дані пакета MSIX для окремого користувача: { $count } розташування
        [many] Дані пакета MSIX для окремого користувача: { $count } розташувань
       *[other] Дані пакета MSIX для окремого користувача: { $count } розташувань
    }
# App Paths is a registry key's name.
core-source-app-paths = Реєстрація в App Paths

## Why a runtime is needed, as the package's report says it.

# { $program } is a program's file name, such as app.exe; { $dll } is a DLL's, such as msvcp140.dll.
core-runtime-program-imports = { $program } імпортує { $dll }

## What else of an app's a scan finds in the registry: fonts and certificates, and why each is the app's.

# Why a font is among the app's files, in the package's list of them.
core-registry-font-recorded = Шрифт, указаний у записі інсталяції програми
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-named-for-the-app = його названо за назвою програми або її видавця
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-recorded = його вказано в записі інсталяції програми

## Who is signed in, for the Whose Data choice. Each of the first six is why it could not be told, and starts with a
## small letter, as it is shown inside other words. An account is shown by its id, such as S-1-5-21-...-1001.

# { $error } is what Windows said.
core-profile-desktop-owner-not-said = Windows не повідомляє, чий це робочий стіл: { $error }
# { $error } is what went wrong.
core-profile-desktop-owner-not-asked = не вдалося запитати у Windows, чий це робочий стіл: { $error }
core-profile-signed-in-account-no-profile = обліковий запис, який тут увійшов у систему ({ $account }), не має зареєстрованого профілю
core-profile-desktop-not-a-person = робочий стіл тут належить обліковому запису { $account }, який не є обліковим записом людини
core-profile-nobody-signed-in-running-account-no-profile = тут ніхто не ввійшов у систему, а обліковий запис, від імені якого це запущено ({ $account }), не має зареєстрованого профілю
core-profile-nobody-signed-in-running-not-a-person = тут ніхто не ввійшов у систему, а { $account }, від імені якого це запущено, не є обліковим записом людини
# SID is the name Windows gives an account's id.
core-profile-nobody-signed-in-no-sid = тут ніхто не ввійшов у систему, а цей обліковий запис не має SID
# { $accounts } is a list of accounts.
core-profile-several-desktops = у цьому сеансі робочий стіл мають кілька облікових записів: { $accounts }
# Under the Whose Data choice. { $reason } is one of the messages above.
core-profile-signed-in-unknown = Не вдалося визначити, хто ввійшов у систему ({ $reason }).
# Under the Whose Data choice, when there are accounts to choose from. { $reason } is one of the messages above.
core-profile-signed-in-unknown-choose = Не вдалося визначити, хто ввійшов у систему ({ $reason }). Виберіть, чиї дані включити.
