# Words the core writes for people: warnings, refusals, errors, and the installer's list of what it sets up.
# Message names start with core-.


## Warnings a package records when it is built, which an installer shows later, maybe on another PC, in that PC's
## language.

# { $count } is how many shortcuts.
core-warning-shortcuts-unreadable =
    { $count ->
        [one] { $count } ярлык на этом диске не указывает ни на какой файл, поэтому не удалось определить, открывает ли он это приложение.
        [few] { $count } ярлыка на этом диске не указывают ни на какой файл, поэтому не удалось определить, открывают ли они это приложение.
        [many] { $count } ярлыков на этом диске не указывают ни на какой файл, поэтому не удалось определить, открывают ли они это приложение.
       *[other] { $count } ярлыка на этом диске не указывают ни на какой файл, поэтому не удалось определить, открывают ли они это приложение.
    }

## Undoing an install: what an uninstall, or a failed install, says it put back, left, or could not undo. Each line is
## shown on its own, in a list.

# { $app_name } is the app's name.
core-rollback-record-damaged = Часть записи об установке { $app_name } не удалось прочитать, поэтому все, что в ней описано, осталось на месте, а замененный файл может не вернуться.
core-rollback-settings-entry-stays = Запись приложения в параметрах Windows остается, чтобы эту операцию можно было снова запустить оттуда.
# { $name } is a runtime's name, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-rollback-prerequisite-stays = Компонент { $name } был установлен (или мог быть установлен) и остается, так как его могут использовать другие программы.
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-drive-not-connected = То, что эта установка поместила в { $folder }, находится на диске, который не подключен. Подключите его, и эту операцию можно будет завершить.
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-record-part-unread = То, что эта установка поместила в { $folder }, остается, так как эту часть записи не удалось прочитать.

## Why a file or folder was left, shown after "was left:" in the messages below them. Each is part of a sentence, so
## starts with a small letter and has no full stop.

# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-being-changed = во время выполнения что-то изменяло { $folder }, поэтому больше ничего в ней не делалось. Закройте то, что это делает, и запустите операцию снова
# { $folder } is a folder; { $error } is what Windows said went wrong.
core-rollback-reason-not-opened = не удалось открыть { $folder }: { $error }
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-no-record = нет записи о том, где находилась папка { $folder }
# { $folder } is a folder; { $actual } is the folder it now leads to.
core-rollback-reason-moved = папка { $folder } теперь указывает на { $actual }, а не на то расположение, что было во время установки
# Why a service was not stopped.
core-rollback-reason-did-not-stop = служба не остановилась вовремя

## Files and folders

# { $path } is a folder; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-folder-left = Папка { $path } оставлена: { $reason }.
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-removed = Удалена папка { $path }
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-holds-others = Папка { $path } остается, так как в ней есть файлы, помещенные туда не этой установкой.
# { $path } is a folder; { $error } is what Windows said went wrong.
core-rollback-folder-not-removed = Не удалось удалить папку { $path }: { $error }.
# { $path } is a file; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-file-left = Файл { $path } оставлен: { $reason }.
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-folder-gone = Файл { $path } не возвращается, так как папки, в которой он находился, больше нет.
# { $path } is a file; { $error } is what went wrong.
core-rollback-partial-not-removed = Не удалось удалить частично записанный файл: { $path } ({ $error }).
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-checked = Не удалось проверить { $path }: { $error }.
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-removed = Удален файл { $path }
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-removed = Не удалось удалить { $path }: { $error }.
# { $path } is a file this install had replaced, and has put back.
core-rollback-file-put-back = Возвращен файл { $path }
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-put-back = Не удалось вернуть { $path }: { $error }.
# Why a file could not be put back.
core-rollback-backup-gone = резервной копии больше нет
# { $path } is a file.
core-rollback-no-parent-folder = у { $path } нет родительской папки

## A file that stays. { $why } says why: still-used (something that stays still uses it), changed (it has changed since
## it was installed), or not-asked (it is someone's data, and they did not ask for it to go). { $path } is the file.

core-rollback-file-stays =
    { $why ->
        [still-used] Файл { $path } остается, так как его использует то, что остается.
        [changed] Файл { $path } остается, так как он изменился после установки.
       *[not-asked] Файл { $path } остается, так как его удаление не запрашивалось.
    }
# The file it replaced was to be put beside it, under another name, and every name was taken.
core-rollback-file-stays-no-name-beside =
    { $why ->
        [still-used] Файл { $path } остается, так как его использует то, что остается, а замененный им файл не удалось поместить рядом: все возможные имена для него заняты.
        [changed] Файл { $path } остается, так как он изменился после установки, а замененный им файл не удалось поместить рядом: все возможные имена для него заняты.
       *[not-asked] Файл { $path } остается, так как его удаление не запрашивалось, а замененный им файл не удалось поместить рядом: все возможные имена для него заняты.
    }
# { $name } is the name the file it replaced now has, such as settings.ini.before-install.
core-rollback-file-stays-beside =
    { $why ->
        [still-used] Файл { $path } остается, так как его использует то, что остается. Замененный им файл находится рядом под именем { $name }.
        [changed] Файл { $path } остается, так как он изменился после установки. Замененный им файл находится рядом под именем { $name }.
       *[not-asked] Файл { $path } остается, так как его удаление не запрашивалось. Замененный им файл находится рядом под именем { $name }.
    }
# { $error } is what went wrong.
core-rollback-file-stays-not-beside =
    { $why ->
        [still-used] Файл { $path } остается, так как его использует то, что остается, а замененный им файл не удалось поместить рядом: { $error }.
        [changed] Файл { $path } остается, так как он изменился после установки, а замененный им файл не удалось поместить рядом: { $error }.
       *[not-asked] Файл { $path } остается, так как его удаление не запрашивалось, а замененный им файл не удалось поместить рядом: { $error }.
    }

## The registry and environment variables. { $kind } says what { $name } is: key (a registry key, such as
## HKLM\Software\App), value (a registry value, such as HKLM\Software\App\Version), user-variable or machine-variable
## (an environment variable of the user's or of the whole PC, such as APP_HOME), or user-list or machine-list (a
## variable that is a list of folders, such as Path). { $error } is what Windows said went wrong.

# How a registry key's unnamed value is shown, as Registry Editor shows it.
core-registry-default-value-name = (По умолчанию)
core-rollback-setting-other-account =
    { $kind ->
        [key] Раздел реестра { $name } остается, так как он принадлежит учетной записи, которая выполняла установку и сейчас не вошла в систему.
        [user-variable] Переменная среды пользователя { $name } остается, так как она принадлежит учетной записи, которая выполняла установку и сейчас не вошла в систему.
        [machine-variable] Системная переменная среды { $name } остается, так как она принадлежит учетной записи, которая выполняла установку и сейчас не вошла в систему.
        [user-list] Переменная пользователя { $name } остается, так как она принадлежит учетной записи, которая выполняла установку и сейчас не вошла в систему.
        [machine-list] Системная переменная { $name } остается, так как она принадлежит учетной записи, которая выполняла установку и сейчас не вошла в систему.
       *[value] Параметр реестра { $name } остается, так как он принадлежит учетной записи, которая выполняла установку и сейчас не вошла в систему.
    }
core-rollback-setting-not-reached =
    { $kind ->
        [key] Не удалось получить доступ к разделу реестра { $name }: { $error }.
        [user-variable] Не удалось получить доступ к переменной среды пользователя { $name }: { $error }.
        [machine-variable] Не удалось получить доступ к системной переменной среды { $name }: { $error }.
        [user-list] Не удалось получить доступ к переменной пользователя { $name }: { $error }.
        [machine-list] Не удалось получить доступ к системной переменной { $name }: { $error }.
       *[value] Не удалось получить доступ к параметру реестра { $name }: { $error }.
    }
core-rollback-setting-not-read =
    { $kind ->
        [key] Не удалось прочитать раздел реестра { $name }: { $error }.
        [user-variable] Не удалось прочитать переменную среды пользователя { $name }: { $error }.
        [machine-variable] Не удалось прочитать системную переменную среды { $name }: { $error }.
        [user-list] Не удалось прочитать переменную пользователя { $name }: { $error }.
        [machine-list] Не удалось прочитать системную переменную { $name }: { $error }.
       *[value] Не удалось прочитать параметр реестра { $name }: { $error }.
    }
core-rollback-setting-changed =
    { $kind ->
        [user-variable] Переменная среды пользователя { $name } остается, так как она изменилась после установки.
        [machine-variable] Системная переменная среды { $name } остается, так как она изменилась после установки.
        [user-list] Переменная пользователя { $name } остается, так как она изменилась после установки.
        [machine-list] Системная переменная { $name } остается, так как она изменилась после установки.
       *[value] Параметр реестра { $name } остается, так как он изменился после установки.
    }
# Said when it is back as it was before the install.
core-rollback-setting-put-back =
    { $kind ->
        [user-variable] Возвращена переменная среды пользователя { $name }
        [machine-variable] Возвращена системная переменная среды { $name }
        [user-list] Возвращена переменная пользователя { $name }
        [machine-list] Возвращена системная переменная { $name }
       *[value] Возвращен параметр реестра { $name }
    }
core-rollback-setting-not-put-back =
    { $kind ->
        [user-variable] Не удалось вернуть переменную среды пользователя { $name }: { $error }.
        [machine-variable] Не удалось вернуть системную переменную среды { $name }: { $error }.
        [user-list] Не удалось вернуть переменную пользователя { $name }: { $error }.
        [machine-list] Не удалось вернуть системную переменную { $name }: { $error }.
       *[value] Не удалось вернуть параметр реестра { $name }: { $error }.
    }
# A registry value the install wrote, which is the user's data, and stays unless they asked for it to go.
core-rollback-value-kept-by-request = Параметр реестра { $name } остается, так как его удаление не запрашивалось.
core-rollback-key-holds-others = Раздел реестра { $name } остается, так как в нем есть то, что поместила туда не эта установка.
core-rollback-key-removed = Удален раздел реестра { $name }
core-rollback-key-not-removed = Не удалось удалить раздел реестра { $name }: { $error }.
# { $kind } is user-list or machine-list.
core-rollback-list-entries-removed =
    { $kind ->
        [user-list] Из переменной пользователя { $name } удалены элементы этой установки
       *[machine-list] Из системной переменной { $name } удалены элементы этой установки
    }
core-rollback-list-entries-not-removed =
    { $kind ->
        [user-list] Не удалось удалить элементы этой установки из переменной пользователя { $name }: { $error }.
       *[machine-list] Не удалось удалить элементы этой установки из системной переменной { $name }: { $error }.
    }

## Services and drivers. { $kind } is service or driver; { $name } is its name, such as AppUpdater. { $error } and
## { $reason } are what Windows said went wrong.

core-rollback-service-not-read =
    { $kind ->
        [driver] Не удалось прочитать драйвер { $name }: { $error }.
       *[service] Не удалось прочитать службу { $name }: { $error }.
    }
core-rollback-service-changed =
    { $kind ->
        [driver] Драйвер { $name } остается, так как он изменился после установки.
       *[service] Служба { $name } остается, так как она изменилась после установки.
    }
core-rollback-service-not-stopped =
    { $kind ->
        [driver] Не удалось остановить драйвер { $name } ({ $reason }), поэтому то, что он использует, может оставаться до перезагрузки компьютера.
       *[service] Не удалось остановить службу { $name } ({ $reason }), поэтому то, что она использует, может оставаться до перезагрузки компьютера.
    }
core-rollback-service-stopped-for-this =
    { $kind ->
        [driver] Драйвер { $name } был остановлен для выполнения этой операции. Запустите его снова, когда он понадобится.
       *[service] Служба { $name } была остановлена для выполнения этой операции. Запустите ее снова, когда она понадобится.
    }
core-rollback-service-not-removed =
    { $kind ->
        [driver] Не удалось удалить драйвер { $name }: { $reason }.
       *[service] Не удалось удалить службу { $name }: { $reason }.
    }
core-rollback-service-marked-for-removal =
    { $kind ->
        [driver] Драйвер { $name } помечен для удаления и будет удален при перезагрузке компьютера или когда его освободит то, что его открыло.
       *[service] Служба { $name } помечена для удаления и будет удалена при перезагрузке компьютера или когда ее освободит то, что ее открыло.
    }
core-rollback-service-removed =
    { $kind ->
        [driver] Удален драйвер { $name }
       *[service] Удалена служба { $name }
    }
core-rollback-service-not-restored =
    { $kind ->
        [driver] Не удалось вернуть драйвер { $name } в прежнее состояние: { $reason }.
       *[service] Не удалось вернуть службу { $name } в прежнее состояние: { $reason }.
    }
core-rollback-service-restored =
    { $kind ->
        [driver] Возвращены параметры драйвера { $name }
       *[service] Возвращены параметры службы { $name }
    }
core-rollback-service-not-restarted = Служба { $name } работала до этой установки, но не запустилась снова: { $reason }.
# { $account } is the account the service ran as, such as CONTOSO\backup.
core-rollback-service-account-again = Служба { $name } выполнялась от имени { $account }. Ее пароль не сохраняется, поэтому снова укажите ее учетную запись в оснастке «Службы».

## Firewall rules. { $name } is the rule's name; { $error } is what went wrong.

core-rollback-firewall-not-read = Не удалось прочитать правило брандмауэра { $name }: { $error }.
core-rollback-firewall-says-otherwise = Правило брандмауэра { $name } остается, так как существующее правило с этим именем задает другое.
core-rollback-firewall-changed = Правило брандмауэра { $name } остается, так как оно изменилось после установки.
# The capital "The" is as the app has always said it.
core-rollback-firewall-removed = Удалено правило брандмауэра { $name }
core-rollback-firewall-not-removed = Не удалось удалить правило брандмауэра { $name }: { $error }.

## Scheduled tasks. { $path } is a folder of tasks, and { $name } a task with its folder, such as \App\Nightly Update.
## { $error } and { $reason } are what went wrong.

core-rollback-task-folder-holds-others = Папка задач { $path } остается, так как в ней есть задачи, созданные не этой установкой.
core-rollback-task-folder-not-removed = Не удалось удалить папку задач { $path }: { $error }.
core-rollback-task-not-read = Не удалось прочитать задачу { $name }: { $error }.
core-rollback-task-changed = Задача { $name } остается, так как она изменилась после установки.
core-rollback-task-removed = Удалена задача { $name }
core-rollback-task-not-removed = Не удалось удалить задачу { $name }: { $reason }.
core-rollback-task-put-back = Возвращена задача { $name }
core-rollback-task-not-put-back = Не удалось вернуть задачу { $name }: { $reason }.

## When an install cannot be undone at all

# { $error } is what went wrong.
core-rollback-record-unreadable = Не удалось прочитать запись об этой установке, поэтому ничего не отменено: { $error }.
# { $error } is what went wrong.
core-rollback-nothing-undone = Ничего не отменено: { $error }.
# One app's part of the message below. { $app_name } is the app's name; { $failures } is what could not be undone, a
# sentence or more; { $folder } is the folder that holds the install's record.
core-rollback-earlier-install-stuck = { $app_name }: { $failures } Если эту установку не удастся отменить, администратор может отказаться от нее, удалив { $folder }.
# Why an install did not start. { $details } is one or more of the message above.
core-rollback-earlier-installs-not-undone = предыдущую незавершенную установку не удалось полностью отменить, поэтому ничего не установлено. Закройте все, что использует эти элементы, и повторите попытку. { $details }
core-rollback-install-busy = эту установку изменяет другой установщик или программа удаления

## The summary an uninstall writes on the command line, one line each, with what each says below it. { $count } is how
## many.

core-rollback-count-undone = Отменено: { $count }
core-rollback-count-failed = Не удалось отменить: { $count }
# What was left on purpose: it has changed since the install, or it is the user's.
core-rollback-count-kept = Оставлено на месте: { $count }
# What no uninstall can take back, such as a runtime that was installed, which other programs may use.
core-rollback-count-left-behind = Осталось: { $count }

## Which Windows a PC or drive has, as the app shows it beside the drive.

# { $name } is the Windows' name, such as Windows 7 Professional Service Pack 1.
core-system-windows-32-bit = { $name }, 32-разрядная
# { $name } is the Windows' name, such as Windows 11 Pro.
core-system-windows-64-bit = { $name }, 64-разрядная
# A drive's Windows, when it only says it is older than Vista.
core-system-windows-xp-or-older = Windows XP или более ранняя версия

## Reading a connected drive, or another account's settings. Each is why it could not be done, and some start with a
## small letter, as they follow other words.

core-system-needs-administrator = для чтения подключенного диска или параметров другой учетной записи нужны права администратора
core-system-no-program-data = не удалось найти папку ProgramData этого компьютера
# { $path } is a file, such as D:\Windows\System32\config\SOFTWARE.
core-system-not-a-hive-file = это не файл куста реестра: { $path }
# { $path } is a file; { $size } is its size in bytes.
core-system-hive-too-large =
    размер файла { $path } составляет { $size ->
        [one] { $size } байт
        [few] { $size } байта
        [many] { $size } байт
       *[other] { $size } байта
    }: это слишком много для куста реестра или больше, чем могут занимать все кусты этого диска вместе
# { $key } is a registry key, such as Software\Microsoft\Windows\CurrentVersion.
core-system-hive-no-mapping = у автономного куста нет сопоставления для HKLM\{ $key }
# { $key } is where a drive's registry was to be loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said.
# "reg load" is a command's name.
core-system-hive-not-loaded = ошибка reg load { $key }: { $error }
# { $key } is where a drive's registry was loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said. "reg
# unload" is a command's name.
core-system-hive-not-unloaded = ошибка reg unload { $key }: { $error }
# { $drive } is a drive, such as D:\.
core-system-not-a-system-drive = Диск { $drive } не является системным диском Windows (нет папки \Windows).
# { $drive } is a drive, such as C:\. "This PC" is the name of a choice in the app.
core-system-own-system-drive = Диск { $drive } — системный диск этого компьютера. Чтобы прочитать этот компьютер, выберите «Этот компьютер».
# { $drive } is a drive, such as D:\.
core-system-registry-through-link = Диск { $drive } не является системным диском Windows: путь к его реестру проходит через ссылку.

## Warnings a scan records about whose data it read.

# { $account } is an account's name or id, such as alice or S-1-5-21-...-1001.
core-warning-scan-account-not-on-record = Учетная запись { $account } здесь не зарегистрирована, поэтому ничьи пользовательские данные и параметры не включены.
# { $account } is an account's name, such as CONTOSO\alice.
core-warning-scan-signed-in-account-no-profile = У учетной записи { $account }, которая вошла в систему здесь, нет зарегистрированного профиля, поэтому ничьи пользовательские данные и параметры не включены.
# { $reason } is why Windows could not say.
core-warning-scan-signed-in-account-unknown = Не удалось определить, кто вошел в систему ({ $reason }), поэтому ничьи пользовательские данные и параметры не включены. Выберите, чьи данные включить.
core-warning-scan-last-account-unknown = На этом диске не указано, кто последним входил в систему, а учетных записей на нем несколько, поэтому ничьи пользовательские данные и параметры не включены. Выберите, чьи данные включить.
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-settings-unreadable = Не удалось прочитать параметры учетной записи { $account }, поэтому они не включены: { $error }
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-classes-unreadable = Не удалось прочитать классы COM и типы файлов, зарегистрированные для { $account }, поэтому они не включены: { $error }

## Paths a scan did not follow. { $count } is how many; { $paths } is some of them, separated by semicolons; { $more } is
## how many more there are, not shown.

core-warning-scan-drive-paths-not-followed =
    { $count ->
        [1] Не удалось прочитать путь, записанный на этом диске, поэтому ничего оттуда не включено. Этот путь находится на другом диске, не является полным или проходит через ссылку, которая вела бы за пределы диска: { $paths }
        [one] Не удалось прочитать некоторые пути, записанные на этом диске, поэтому ничего оттуда не включено. Каждый из них находится на другом диске, не является полным или проходит через ссылку, которая вела бы за пределы диска: { $paths }
        [few] Не удалось прочитать некоторые пути, записанные на этом диске, поэтому ничего оттуда не включено. Каждый из них находится на другом диске, не является полным или проходит через ссылку, которая вела бы за пределы диска: { $paths }
        [many] Не удалось прочитать некоторые пути, записанные на этом диске, поэтому ничего оттуда не включено. Каждый из них находится на другом диске, не является полным или проходит через ссылку, которая вела бы за пределы диска: { $paths }
       *[other] Не удалось прочитать некоторые пути, записанные на этом диске, поэтому ничего оттуда не включено. Каждый из них находится на другом диске, не является полным или проходит через ссылку, которая вела бы за пределы диска: { $paths }
    }
core-warning-scan-drive-paths-not-followed-more = Не удалось прочитать некоторые пути, записанные на этом диске, поэтому ничего оттуда не включено. Каждый из них находится на другом диске, не является полным или проходит через ссылку, которая вела бы за пределы диска: { $paths } и еще { $more }
core-warning-scan-network-paths-not-followed =
    { $count ->
        [1] Не удалось прочитать путь, записанный на этом компьютере, поэтому ничего оттуда не включено. Этот путь находится не на диске этого компьютера (обращение к нему привело бы к подключению) или имеет имя, которое открыло бы что-то другое: { $paths }
        [one] Не удалось прочитать некоторые пути, записанные на этом компьютере, поэтому ничего оттуда не включено. Каждый из них находится не на диске этого компьютера (обращение к нему привело бы к подключению) или имеет имя, которое открыло бы что-то другое: { $paths }
        [few] Не удалось прочитать некоторые пути, записанные на этом компьютере, поэтому ничего оттуда не включено. Каждый из них находится не на диске этого компьютера (обращение к нему привело бы к подключению) или имеет имя, которое открыло бы что-то другое: { $paths }
        [many] Не удалось прочитать некоторые пути, записанные на этом компьютере, поэтому ничего оттуда не включено. Каждый из них находится не на диске этого компьютера (обращение к нему привело бы к подключению) или имеет имя, которое открыло бы что-то другое: { $paths }
       *[other] Не удалось прочитать некоторые пути, записанные на этом компьютере, поэтому ничего оттуда не включено. Каждый из них находится не на диске этого компьютера (обращение к нему привело бы к подключению) или имеет имя, которое открыло бы что-то другое: { $paths }
    }
core-warning-scan-network-paths-not-followed-more = Не удалось прочитать некоторые пути, записанные на этом компьютере, поэтому ничего оттуда не включено. Каждый из них находится не на диске этого компьютера (обращение к нему привело бы к подключению) или имеет имя, которое открыло бы что-то другое: { $paths } и еще { $more }

## What a scan with none of an account's data leaves out of a package, in the app's list of what was left out. Each
## has a name, and below it why.

core-account-left-out-shortcuts = Ярлыки в собственных папках этой учетной записи
# { $count } is how many shortcuts.
core-account-left-out-shortcuts-why =
    { $count ->
        [one] { $count } ярлык, который включается только с параметром «Данные пользователя»
        [few] { $count } ярлыка, которые включаются только с параметром «Данные пользователя»
        [many] { $count } ярлыков, которые включаются только с параметром «Данные пользователя»
       *[other] { $count } ярлыка, которые включаются только с параметром «Данные пользователя»
    }
core-account-left-out-variables = Переменные среды этой учетной записи
# { $count } is how many variables.
core-account-left-out-variables-why =
    { $count ->
        [one] { $count } переменная, которая включается только с параметром «Данные пользователя»
        [few] { $count } переменные, которые включаются только с параметром «Данные пользователя»
        [many] { $count } переменных, которые включаются только с параметром «Данные пользователя»
       *[other] { $count } переменной, которые включаются только с параметром «Данные пользователя»
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-account-left-out-task = Задача планировщика { $task }
core-account-task-works-on-account-folder = Она работает с расположением в папках учетной записи или названа по нему и включается только с параметром «Данные пользователя»
core-account-task-runs-from-account-folder = То, что она также запускает из папок учетной записи, включается только с параметром «Данные пользователя»
# { $service } is a service's name, such as AppUpdater.
core-account-left-out-service = Служба { $service }
core-account-service-works-on-account-folder = Она работает с расположением в папках учетной записи и включается только с параметром «Данные пользователя»

## What a package was found to still say of whose PC it came from, after a build with none of an account's data.

# The parts of a package, as the warning below names them.
core-account-part-files = пути к собственным файлам приложения
core-account-part-registry = параметры реестра
core-account-part-settings-entry = запись приложения в параметрах Windows
core-account-part-main-program = расположение основной программы
core-account-part-shortcuts = ярлыки
core-account-part-tasks = задачи планировщика
core-account-part-services = службы
core-account-part-variables = переменные среды
# One place in the list below. { $place } is one of the parts above, or a file's name; { $count } is how many times.
core-account-mention-place = { $place } ({ $count })
# { $kind } is what was found: account (an account's name), folder (where an account's folder is), computer (the PC's
# name), or id (an account's id). { $places } is a list of where, from the message above.
core-warning-scan-package-mentions =
    { $kind ->
        [account] В этом пакете имя учетной записи упоминается в следующих местах: { $places }. Пакет создан без данных учетных записей, но это может прочитать любой, у кого есть установщик.
        [folder] В этом пакете расположение папки учетной записи упоминается в следующих местах: { $places }. Пакет создан без данных учетных записей, но это может прочитать любой, у кого есть установщик.
        [computer] В этом пакете имя компьютера упоминается в следующих местах: { $places }. Пакет создан без данных учетных записей, но это может прочитать любой, у кого есть установщик.
       *[id] В этом пакете идентификатор учетной записи упоминается в следующих местах: { $places }. Пакет создан без данных учетных записей, но это может прочитать любой, у кого есть установщик.
    }
# { $count } is how many folders.
core-warning-scan-folders-of-others-unreadable =
    { $count ->
        [one] Не удалось прочитать { $count } папку вне приложения, поэтому данные приложения в ней могут отсутствовать.
        [few] Не удалось прочитать { $count } папки вне приложения, поэтому данные приложения в них могут отсутствовать.
        [many] Не удалось прочитать { $count } папок вне приложения, поэтому данные приложения в них могут отсутствовать.
       *[other] Не удалось прочитать { $count } папки вне приложения, поэтому данные приложения в них могут отсутствовать.
    }

## What a scan leaves out because it looks like a credential, or is a license with the License option off, in the app's
## list of what was left out. Each has a name, and below it why.

core-secret-why-credential = похоже на учетные данные
core-secret-why-license-option-off = параметр «Данные лицензии и активации» выключен
# { $count } is how many values of a registry key.
core-secret-why-account-values =
    { $count ->
        [one] { $count } значение, которое включается только с параметром «Данные пользователя»
        [few] { $count } значения, которые включаются только с параметром «Данные пользователя»
        [many] { $count } значений, которые включаются только с параметром «Данные пользователя»
       *[other] { $count } значения, которые включаются только с параметром «Данные пользователя»
    }
# Shown in place of a registry value's name when its name itself looks like a password or a key.
core-secret-name-holds-credential = (параметр, имя которого содержит учетные данные)
# Shown in place of an option's name, for an argument on a command line that has no name.
core-secret-unnamed-argument = аргумент, содержащий учетные данные
# { $option } is an option taken out of a command line, such as /password, or the message above. { $place } says
# where the command line was: service, task, shortcut, opener (what opens a file type), opener-unnamed (the same, of a
# type with no name), or uninstall. { $name } is the service, task, shortcut, file type, or app.
core-secret-taken-from =
    { $place ->
        [service] { $option } в команде запуска { $name }
        [task] { $option } в команде запуска задачи { $name }
        [shortcut] { $option } в ярлыке { $name }
        [opener] { $option } в программе, которая открывает { $name }
        [opener-unnamed] { $option } в программе, которая открывает тип файлов
       *[uninstall] { $option } в команде удаления { $name }
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-secret-task-definition-unreadable = Определение задачи { $task }, которое не удалось прочитать для проверки
# { $name } is an environment variable's name, such as APP_TOKEN.
core-secret-left-out-variable = Переменная среды { $name }
# { $class } is a COM class's id, such as {00000000-0000-0000-0000-000000000000}.
core-secret-left-out-class-server = То, что запускает класс { $class }
# { $rule } is a firewall rule's name.
core-secret-left-out-firewall-description = Описание правила брандмауэра { $rule }

## Licenses a scan found, as the package's report describes each.

# { $name } is a file's name, such as license.lic.
core-license-detail-file = Обнаружен файл лицензии: { $name }
# { $framework } is a licensing product's name, such as FlexLM / FlexNet.
core-license-detail-store = Обнаружено хранилище лицензий { $framework }
# { $name } is an environment variable's name, such as LM_LICENSE_FILE.
core-license-detail-variable = Переменная среды лицензии { $name }
# { $name } is a registry value's name, such as SerialNumber.
core-license-detail-registry-value = Параметр реестра, похожий на данные лицензии: { $name }
# Said of a registry value in an account's key, which is not named.
core-license-detail-registry-value-of-an-account = Параметр реестра, похожий на данные лицензии
# { $key } is a registry key, such as HKLM\Software\Contoso\License.
core-license-detail-registry-key = Раздел реестра, похожий на данные лицензии: { $key }

## Warnings a scan records about licenses.

core-warning-scan-license-data = Обнаружены данные лицензии или активации. Если параметр «Данные лицензии и активации» включен, установщик восстанавливает то, что можно перенести; значения уровня компьютера и хранилища, привязанные к оборудованию, только указываются в отчете. WinAppMigrator никогда не расшифровывает данные, не обходит активацию и не эмулирует аппаратные ключи.
core-warning-scan-license-machine-bound = Некоторые лицензии привязаны к компьютеру (аппаратный ключ, активация с привязкой к компьютеру или шифрование). Установщик восстанавливает то, что может; для них все равно может потребоваться повторная активация на новом компьютере.

## Warnings a scan records about the registry. What could not be read is not named in them: it is kept beside them.

# { $error } is what Windows said went wrong. App Paths is a registry key's name.
core-warning-scan-app-paths-key-unreadable = Не удалось прочитать дочерний раздел App Paths: { $error }
core-warning-scan-registry-limit = Считывание реестра достигло установленного защитного ограничения.
core-warning-scan-registry-subtree-limit = Считывание поддерева реестра достигло установленного защитного ограничения.
# { $error } is what Windows said went wrong.
core-warning-scan-registry-key-unreadable = Не удалось прочитать раздел реестра: { $error }
core-warning-scan-registry-value-too-large = Параметр реестра превысил установленное ограничение размера при считывании.

## Scheduled tasks

core-warning-scan-tasks-folder-through-link = Папка задач планировщика на этом диске доступна через ссылку, поэтому ее задачи не прочитаны.
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-warning-scan-task-left-out = Задача планировщика { $task } исключена: она выполняется от имени другой учетной записи, а не той, чьи параметры включаются, ее файл могут изменять не только администраторы, или у приложения слишком много задач, чтобы сохранить их все.
# { $error } is what Windows said went wrong.
core-warning-scan-tasks-folder-unreadable = Не удалось прочитать часть задач планировщика: { $error }
# { $path } is a task's file, such as C:\Windows\System32\Tasks\App\Nightly Sync; { $reason } is one of the reasons
# below, or what Windows said went wrong.
core-warning-scan-task-unreadable = Не удалось прочитать задачу планировщика { $path }: { $reason }. Если она запускает приложение, она не включена.
# Why a task's file could not be read. { $size } is its size in bytes.
core-task-too-large =
    размер файла составляет { $size ->
        [one] { $size } байт
        [few] { $size } байта
        [many] { $size } байт
       *[other] { $size } байта
    }, это слишком много для задачи
# Why a task's file could not be read.
core-task-ends-mid-character = файл обрывается посреди символа
# Why a task's file could not be read. { $error } says where.
core-task-not-utf16 = файл не является допустимым текстом UTF-16: { $error }
# Why a task's file could not be read. { $error } says where.
core-task-not-text = файл не является допустимым текстом UTF-8 или UTF-16: { $error }
# What a package records of how a task is put back. Register-ScheduledTask is a command's name.
core-task-restore-instructions = Для восстановления используется Register-ScheduledTask с сохраненным XML-определением.

## Reading a program's file. Each is why it could not be read, after the program's name.

# { $reason } is what went wrong.
core-pe-unreadable = не удалось прочитать файл: { $reason }
core-pe-headers-misaligned = его заголовки не выровнены по границам, обязательным для программ
# { $size } is the file's size in bytes.
core-pe-too-large =
    размер файла составляет { $size ->
        [one] { $size } байт
        [few] { $size } байта
        [many] { $size } байт
       *[other] { $size } байта
    }, это слишком много для программы
# PE is the name of the format Windows programs are in.
core-pe-no-headers = в нем нет заголовков PE

## Warnings a scan records about what a program needs. Which program is kept beside each.

# { $error } is what went wrong.
core-warning-scan-imports-unreadable = Не удалось прочитать таблицу импорта: { $error }
# { $error } is what went wrong.
core-warning-scan-import-name-unreadable = Не удалось прочитать имя дескриптора импорта: { $error }
# { $count } is how many were skipped.
core-warning-scan-legacy-delay-imports =
    { $count ->
        [one] Пропущен { $count } устаревший дескриптор отложенной загрузки (v1); анализируются только современные отложенные импорты на основе RVA.
        [few] Пропущены { $count } устаревших дескриптора отложенной загрузки (v1); анализируются только современные отложенные импорты на основе RVA.
        [many] Пропущено { $count } устаревших дескрипторов отложенной загрузки (v1); анализируются только современные отложенные импорты на основе RVA.
       *[other] Пропущено { $count } устаревшего дескриптора отложенной загрузки (v1); анализируются только современные отложенные импорты на основе RVA.
    }

## The DLLs a program needs. Which DLL or program is kept beside each warning.

# { $path } is a program, such as C:\Program Files\App\app.exe.
core-dll-program-has-no-folder = у исполняемого файла нет родительской папки: { $path }
# Why a DLL is among the app's files, in the package's list of them.
core-dll-beside-program = DLL находится рядом с выбранным исполняемым файлом
# { $error } is what went wrong.
core-warning-scan-dependency-unscanned = Не удалось выполнить рекурсивное сканирование зависимости: { $error }
# { $name } is a DLL's name, such as d3dx9_43.dll.
core-warning-scan-dependency-not-in-newer-windows = Файл { $name } находился в папке Windows на компьютере, где было это приложение, а в Windows 10 и 11 его нет. Его туда установила какая-то программа, поэтому он не относится к собственным файлам приложения и не переносится. Приложение может не запуститься, пока не будет установлено то, что его предоставляет.
# { $error } is what went wrong.
core-warning-scan-dependency-not-hashed = Не удалось вычислить хэш зависимости: { $error }
# SystemRoot and windir are the names of environment variables.
core-warning-scan-windows-folder-unknown = Переменные среды SystemRoot/windir недоступны; поиск системных DLL ограничен.
# { $error } is what went wrong.
core-warning-scan-dependency-architecture-unreadable = Не удалось прочитать архитектуру зависимости: { $error }

## Services, drivers, shortcuts, and environment variables a scan reads. Where it was reading is kept beside each
## warning.

# { $error } is what Windows said went wrong.
core-warning-scan-services-unreadable = Не удалось прочитать раздел реестра служб: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-drivers-unreadable = Не удалось прочитать раздел реестра служб для поиска драйверов: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-queried = Не удалось запросить ярлыки: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-parsed = Не удалось разобрать результат запроса ярлыков: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-environment-unreadable = Не удалось прочитать раздел реестра системных переменных среды: { $error }

## What a package records of how a driver is put back, which an installer shows. { $kind } is the driver's type as
## Windows names it (kernel, file-system, recognizer, or unknown-type); { $name } is its service's name; { $path } is
## its file. { $signed } is yes when { $signer } names who signed it, and no when that is not known.

core-service-driver-bundled =
    { $signed ->
        [yes] Драйвер приложения «{ $name }» (тип { $kind }, { $path }, подписан: { $signer }). Установщик включает его файл .sys в пакет и автоматически регистрирует службу драйвера повторно (sc create, требуются повышенные права), а затем проверяет, что служба зарегистрирована.
       *[no] Драйвер приложения «{ $name }» (тип { $kind }, { $path }, подпись неизвестна). Установщик включает его файл .sys в пакет и автоматически регистрирует службу драйвера повторно (sc create, требуются повышенные права), а затем проверяет, что служба зарегистрирована.
    }
core-service-driver-from-vendor =
    { $signed ->
        [yes] Драйвер поставщика или встроенный драйвер Windows «{ $name }» (тип { $kind }, { $path }, подписан: { $signer }). WinAppMigrator не распространяет сторонние драйверы; перед запуском приложения переустановите драйвер на целевом компьютере из подписанного пакета драйвера поставщика.
       *[no] Драйвер поставщика или встроенный драйвер Windows «{ $name }» (тип { $kind }, { $path }, подпись неизвестна). WinAppMigrator не распространяет сторонние драйверы; перед запуском приложения переустановите драйвер на целевом компьютере из подписанного пакета драйвера поставщика.
    }

## Runtimes an app needs, such as the Visual C++ runtime or .NET: how each is provided, as the Dependencies list says.

core-runtime-handling-built-in = Встроено в Windows
core-runtime-handling-ships-with-app = Поставляется с приложением
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-handling-windows-feature = Компонент Windows: { $feature }
core-runtime-handling-included = Включено
# { $id } is the runtime's name in winget, such as Microsoft.VCRedist.2015+.x64.
core-runtime-handling-winget = Устанавливается с помощью winget: { $id }
core-runtime-handling-manual = Установите вручную

## Downloading a runtime's installer, when an installer is built. Each is why it could not be, and starts with a small
## letter, as it follows other words. { $url } is a web address; { $error } is what went wrong.

core-runtime-download-not-https = отказано в скачивании { $url }: это не HTTPS
# TLS is what keeps a download private.
core-runtime-tls-failed = ошибка инициализации TLS: { $error }
core-runtime-download-failed = ошибка скачивания { $url }: { $error }
core-runtime-download-unreadable = ошибка чтения { $url }: { $error }
core-runtime-download-empty = при скачивании { $url } не получено данных
# { $came } and { $expected } are numbers of bytes.
core-runtime-download-cut-short =
    скачивание { $url } прервано: получено { $came } из { $expected ->
        [one] { $expected } байта
        [few] { $expected } байт
        [many] { $expected } байт
       *[other] { $expected } байта
    }

## Installing a runtime: why it was not installed. Each starts with a small letter, as it follows other words.

core-runtime-no-known-installer = для этой среды выполнения нет известного установщика
# { $path } is where in the package, such as prerequisites\vc_redist.x64.exe.
core-runtime-payload-misplaced = пакет указывает { $path } как установщик этой среды выполнения, но этот файл находится не на своем месте, поэтому он не запущен
# { $path } is a file.
core-runtime-payload-missing = установщик из пакета отсутствует: { $path }
# { $error } is what went wrong.
core-runtime-payload-path-invalid = недопустимый путь к установщику из пакета: { $error }
# { $url } is the web address it can be downloaded from. winget is a program's name.
core-runtime-no-installer-route = нет ни установщика в пакете, ни идентификатора winget; скачайте компонент отсюда: { $url }
# { $error } is what went wrong.
core-runtime-no-temp-folder = не удалось создать папку для распаковки: { $error }
# { $reason } is what the check found.
core-runtime-payload-signature-failed = установщик из пакета не прошел проверку подписи и не запущен: { $reason }
core-runtime-payload-timed-out = установщик из пакета не завершил работу за отведенное время и был остановлен
# { $error } is what went wrong.
core-runtime-payload-not-started = не удалось запустить установщик из пакета: { $error }
core-runtime-installer-killed = установщик был остановлен без кода выхода
# { $code } is the number it ended with, such as 1603.
core-runtime-installer-failed = установщик завершил работу с кодом { $code }
# { $feature } is a Windows feature's name, such as NetFx3; { $code } is the number DISM ended with. Windows Features is
# the name of a Windows window. Keep the command as it is.
core-runtime-feature-failed = DISM не удалось включить компонент Windows { $feature } (код выхода { $code }); вероятно, не удалось скачать его файлы. Включите его в окне «Компоненты Windows» или выполните `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`.
# The same, when DISM gave no number.
core-runtime-feature-failed-no-code = DISM не удалось включить компонент Windows { $feature } (код выхода неизвестен); вероятно, не удалось скачать его файлы. Включите его в окне «Компоненты Windows» или выполните `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`.
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-feature-timed-out = включение компонента Windows { $feature } не завершилось за отведенное время и было остановлено
# { $feature } is a Windows feature's name; { $error } is what went wrong.
core-runtime-feature-no-dism = DISM недоступен для включения компонента Windows { $feature }: { $error }

## Installing a runtime with winget. { $id } is its name in winget, such as Microsoft.VCRedist.2015+.x64.

# { $error } is what went wrong.
core-runtime-winget-not-looked-for = не удалось выполнить поиск winget: { $error }
core-runtime-winget-not-installed = winget не установлен
# { $path } is where winget is.
core-runtime-winget-misplaced = { $path } находится не там, где Windows хранит свои пакеты, поэтому он не запущен
# { $path } is where winget is; { $reason } is what the check found.
core-runtime-winget-signature-failed = { $path } не прошел проверку подписи и не запущен: { $reason }
# { $reason } is one of the two messages above, or what went wrong.
core-runtime-winget-unusable = не удалось использовать winget для установки { $id }: { $reason }
# { $code } is the number winget ended with.
core-runtime-winget-failed = winget install { $id } завершился с кодом { $code }
core-runtime-winget-failed-no-code = winget install { $id } завершился с неизвестным кодом
core-runtime-winget-timed-out = winget install { $id } не завершился за отведенное время и был остановлен
# { $error } is what went wrong.
core-runtime-winget-not-started = winget недоступен для установки { $id }: { $error }

## What told a scan how an app was installed, as the package's report lists it. The names of installers and files are
## as their makers write them.

# { $path } is an installer's record of what it installed, such as C:\Program Files\App\unins000.dat.
core-source-record-not-believed = файл { $path } не использован: его могут изменять не только администраторы
# { $count } is how many Windows Installer packages the bundle holds.
core-source-burn-bundle =
    { $count ->
        [one] Набор Burn из { $count } пакета установщика Windows
        [few] Набор Burn из { $count } пакетов установщика Windows
        [many] Набор Burn из { $count } пакетов установщика Windows
       *[other] Набор Burn из { $count } пакета установщика Windows
    }
# { $count } is how many paths Windows Installer recorded.
core-source-msi-database =
    { $count ->
        [one] База данных установщика Windows: { $count } ключевой путь компонента
        [few] База данных установщика Windows: { $count } ключевых пути компонентов
        [many] База данных установщика Windows: { $count } ключевых путей компонентов
       *[other] База данных установщика Windows: { $count } ключевого пути компонентов
    }
core-source-inno-log = Журнал удаления Inno Setup (unins000.dat)
# { $count } is how many paths the log records.
core-source-inno-log-paths =
    { $count ->
        [one] Журнал удаления Inno Setup: { $count } записанный путь
        [few] Журнал удаления Inno Setup: { $count } записанных пути
        [many] Журнал удаления Inno Setup: { $count } записанных путей
       *[other] Журнал удаления Inno Setup: { $count } записанного пути
    }
core-source-nsis = Сигнатура программы удаления NSIS (Nullsoft)
# { $count } is how many paths the log records.
core-source-nsis-log-paths =
    { $count ->
        [one] NSIS AdvUninstLog uninstall.log: { $count } записанный путь
        [few] NSIS AdvUninstLog uninstall.log: { $count } записанных пути
        [many] NSIS AdvUninstLog uninstall.log: { $count } записанных путей
       *[other] NSIS AdvUninstLog uninstall.log: { $count } записанного пути
    }
core-source-squirrel = Структура Squirrel для отдельного пользователя (Update.exe / app-*)
# { $version } is a folder's name that says the version, such as app-1.2.3.
core-source-squirrel-version = Текущая версия Squirrel: { $version }
core-source-chocolatey = Метаданные пакета Chocolatey
core-source-msix = Пакет MSIX / Appx
# { $identity } is a package's name, such as Contoso.App.
core-source-appx-identity = Удостоверение AppxManifest: { $identity }
# { $count } is how many folders.
core-source-msix-data =
    { $count ->
        [one] Данные пакета MSIX для пользователя: { $count } расположение
        [few] Данные пакета MSIX для пользователя: { $count } расположения
        [many] Данные пакета MSIX для пользователя: { $count } расположений
       *[other] Данные пакета MSIX для пользователя: { $count } расположения
    }
# App Paths is a registry key's name.
core-source-app-paths = Регистрация App Paths

## Why a runtime is needed, as the package's report says it.

# { $program } is a program's file name, such as app.exe; { $dll } is a DLL's, such as msvcp140.dll.
core-runtime-program-imports = { $program } импортирует { $dll }

## What else of an app's a scan finds in the registry: fonts and certificates, and why each is the app's.

# Why a font is among the app's files, in the package's list of them.
core-registry-font-recorded = Шрифт, указанный в записи об установке приложения
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-named-for-the-app = он назван по имени приложения или его издателя
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-recorded = он указан в записи об установке приложения

## Who is signed in, for the Whose Data choice. Each of the first six is why it could not be told, and starts with a
## small letter, as it is shown inside other words. An account is shown by its id, such as S-1-5-21-...-1001.

# { $error } is what Windows said.
core-profile-desktop-owner-not-said = Windows не сообщила, чей это рабочий стол: { $error }
# { $error } is what went wrong.
core-profile-desktop-owner-not-asked = не удалось запросить у Windows, чей это рабочий стол: { $error }
core-profile-signed-in-account-no-profile = у учетной записи { $account }, которая вошла в систему здесь, нет зарегистрированного профиля
core-profile-desktop-not-a-person = рабочий стол здесь принадлежит учетной записи { $account }, которая не является учетной записью человека
core-profile-nobody-signed-in-running-account-no-profile = здесь никто не вошел в систему, а у учетной записи { $account }, от имени которой выполняется программа, нет зарегистрированного профиля
core-profile-nobody-signed-in-running-not-a-person = здесь никто не вошел в систему, а учетная запись { $account }, от имени которой выполняется программа, не является учетной записью человека
# SID is the name Windows gives an account's id.
core-profile-nobody-signed-in-no-sid = здесь никто не вошел в систему, а у этой учетной записи нет SID
# { $accounts } is a list of accounts.
core-profile-several-desktops = в этом сеансе рабочий стол есть у нескольких учетных записей: { $accounts }
# Under the Whose Data choice. { $reason } is one of the messages above.
core-profile-signed-in-unknown = Не удалось определить, кто вошел в систему ({ $reason }).
# Under the Whose Data choice, when there are accounts to choose from. { $reason } is one of the messages above.
core-profile-signed-in-unknown-choose = Не удалось определить, кто вошел в систему ({ $reason }). Выберите, чьи данные включить.
