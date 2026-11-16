# Words the core writes for people as it installs an app from a package: why an install will not start, the list of
# what it did (or for a preview, what it would do) that the installer window shows, its warnings, the record it keeps of
# each install so that it can be undone, and the core's errors. Message names start with core-.
#
# The list of what an install did is short and plain, one line for each thing, as the installer window lists them:
# "copied", "restored", "would copy". Words such as --force and --apply are command-line options, and LocalService,
# SYSTEM, ProgramData, and %AppData% are as Windows names them: keep them as they are.


## Errors: what went wrong, when the core cannot go on. Each is shown on its own, or after words that say what was being
## done.

core-error-list-changed = то, что сделает эта установка, изменилось после того, как вы с этим согласились, поэтому ничего не установлено
# { $path } is a file or folder, such as C:\Program Files\App\app.exe, or words that name one, such as "the installer".
# { $error } is what Windows said went wrong.
core-error-io = ошибка ввода-вывода ({ $path }): { $error }
# { $detail } is what went wrong.
core-error-registry = ошибка чтения реестра: { $detail }
# { $path } is a program file. { $detail } is what was wrong with it.
core-error-pe = ошибка разбора PE-файла { $path }: { $detail }
# { $detail } is what was wrong, already in words.
core-error-invalid-input = недопустимые входные данные: { $detail }
# { $detail } is what went wrong.
core-error-serialization = ошибка сериализации отчета: { $detail }
# { $detail } is what went wrong.
core-error-archive = ошибка операции с архивом: { $detail }
# { $detail } is what went wrong.
core-error-network = ошибка сетевой операции: { $detail }
# { $detail } is what went wrong.
core-error-time-format = ошибка форматирования времени: { $detail }
core-error-cancelled = операция отменена
# What is said when part of the program stopped unexpectedly and gave no reason.
core-error-panic-no-reason = причина не указана

## The record of each install, kept so that it can be undone.

# { $kind } is a number that says what type of registry value it is.
core-journal-unknown-value-type = неизвестный тип параметра реестра { $kind }
core-journal-not-elevated = нет повышенных прав
core-journal-needs-admin = для установки нужны права администратора, чтобы хранить запись о ней там, где другие не могут ее изменить
# { $path } is a folder.
core-journal-not-an-install-folder = это не папка установки: { $path }
# { $path } is the file the record is kept in.
core-journal-not-written = не удалось сохранить запись об этой установке: { $path }
# { $path } is a folder.
core-journal-path-escapes-root = путь выходит за пределы предусмотренной корневой папки: { $path }
# { $path } is a folder.
core-journal-nothing-exists = ничего из этого нет на этом компьютере: { $path }
core-journal-busy = выполняется другая установка или удаление. Дождитесь завершения и повторите попытку
# Said of a file that was to get back the permissions it had.
core-journal-no-folder = у него нет папки
# Said of the permissions a file had, which were kept to give back to it.
core-journal-permissions-unreadable = сохраненные разрешения не удалось прочитать
# { $folder } is the folder of one install's record.
core-journal-unreadable-empty = запись об установке в папке { $folder } нельзя использовать: она пуста
# { $folder } is the folder of one install's record.
core-journal-unreadable-no-header = запись об установке в папке { $folder } нельзя использовать: в ее начале не указано, к чему она относится
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-version = запись об установке в папке { $folder } нельзя использовать: ее создала другая версия этой программы
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-install = запись об установке в папке { $folder } нельзя использовать: в ней указано, что она относится к другой установке
# { $id } is an install's id, such as wam-1a2b3c.
core-journal-not-an-install-id = это не идентификатор установки: { $id }

## Why an install will not start, or stopped before it changed anything.

core-restore-apply-and-dry-run = нельзя указывать одновременно --apply и --dry-run
# { $path } is the folder that was chosen to install in.
core-restore-location-not-full-path = для папки установки нужно указать полный путь: «{ $path }»
# { $problem } says what is wrong with the folder, already in words. { $path } is the folder.
core-restore-location-unusable = эту папку установки нельзя использовать: { $problem }: «{ $path }»
core-restore-needs-admin-plan = план восстановления включает изменения реестра, служб, задач планировщика, ярлыков или защищенных расположений, для которых нужны права администратора
# { $path } is the folder that was chosen to install in.
core-restore-needs-admin-target = для целевой корневой папки нужны повышенные права: { $path }
core-restore-needs-admin-user-data = для восстановления данных пользователя в ProgramData или другие защищенные расположения нужны повышенные права
# { $error } is why the install stopped. { $failed } is what could not be put back, as sentences of their own.
core-restore-not-all-put-back = { $error }. Не все сделанное удалось вернуть: { $failed }
# { $error } is what went wrong.
core-restore-nowhere-to-keep-record = нет безопасного места для записи, которая позволила бы отменить эту установку, поэтому ничего не установлено: { $error }
# { $path } is a file the app would have put there.
core-restore-already-exists = { $path } уже существует, поэтому ничего не установлено. Чтобы заменить, используйте --force
# { $root } is a folder the app has files for. { $link } is that folder, or one it is in.
core-restore-root-under-link = у этого приложения есть данные для восстановления в { $root }, а { $link } — это ссылка на другое расположение, которое могут изменить другие, поэтому ничего не установлено. Если туда помещаются данные пользователя или данные лицензии, установите приложение без данных пользователя и данных лицензии. Если это ярлык в папке автозагрузки, верните эту папку на прежнее место
core-restore-hash-check-failed = пакет не прошел проверку хэша, и ничего не установлено
# { $detail } is the first thing found wrong, already in words.
core-restore-hash-check-failed-because = пакет не прошел проверку хэша, и ничего не установлено: { $detail }
# { $path } is a file or folder.
core-restore-refuse-records-folder = отказано в записи туда, где хранятся записи об установках: { $path }
# { $path } is a file or folder.
core-restore-refuse-outside = отказано в записи за пределами папки установки и известных папок данных: { $path }
# { $path } is a file.
core-restore-no-parent-folder = у места назначения нет родительской папки: { $path }
# { $path } is where a file was to go.
core-restore-folder-in-the-way = на месте этого файла находится папка: { $path }
# { $path } is a file in the package.
core-restore-file-changed = файл { $path } больше не совпадает с файлом в пакете, поэтому он не установлен
# { $path } is a file in the package.
core-restore-source-is-link = { $path } — это ссылка, и переход по ней не выполняется
# { $path } is a file in the package.
core-restore-source-became-link = файл { $path } стал ссылкой во время открытия
# Why a service did not start, after "did not start:".
core-restore-service-stopped-by-itself = она снова остановилась сама
# Why part of the app was left out, after "was left out:".
core-restore-com-class-someone-elses = этот класс COM уже есть на этом компьютере и принадлежит другому приложению

## What an install did, one line for each thing, or for a preview, what it would do. { $source } is a file in the
## package, and { $destination } and { $path } are where it goes on this PC. { $key } is a registry key, such as
## HKLM\Software\App.

# { $package_valid } is true or false, and { $restore_valid } is true, false, or not_run: keep them as they are.
core-restore-verification-result = проверка после восстановления: package_valid={ $package_valid } restore_valid={ $restore_valid }
# { $name } is a service's name.
core-restore-started-service = запущена служба { $name }
core-restore-would-copy = будет скопировано: { $source } -> { $destination }
core-restore-copied = скопировано: { $source } -> { $destination }
core-restore-would-create-empty-folder = будет создана пустая папка { $path }
core-restore-created-empty-folder = создана пустая папка { $path }
# { $value } is the name of a value in the key.
core-restore-would-restore-registry-value = будет восстановлен параметр реестра { $key } { $value }
core-restore-would-restore-registry-key = будет восстановлен раздел реестра { $key }
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-would-add-firewall-rule = будет добавлено правило брандмауэра { $name } для { $program }
# { $name } is a firewall rule's name.
core-restore-firewall-rule-already-there = правило брандмауэра { $name } уже существует
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-added-firewall-rule = добавлено правило брандмауэра { $name } для { $program }
# { $name } is a service's name, { $command } what it runs, and { $account } the account it runs as, such as LocalService.
core-restore-would-restore-service = будет восстановлена служба { $name } -> { $command } (запускается от имени { $account })
# { $name } is a runtime, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-restore-prerequisite-present = необходимый компонент уже установлен: { $name }
# { $name } is a runtime that Windows has as a Windows feature.
core-restore-would-install-prerequisite-feature = будет установлен необходимый компонент { $name } (компонент Windows)
# { $name } is a runtime whose installer is in the package.
core-restore-would-install-prerequisite-bundled = будет установлен необходимый компонент { $name } (установщик из пакета)
# { $name } is a runtime that would come through winget, which is a program's name.
core-restore-would-install-prerequisite-winget = будет установлен необходимый компонент { $name } (winget)
# { $name } is a runtime.
core-restore-installed-prerequisite = установлен необходимый компонент { $name }
# { $name } is a runtime.
core-restore-installed-prerequisite-reboot = установлен необходимый компонент { $name } (требуется перезагрузка)
core-restore-would-restore-license = будет восстановлена лицензия: { $source } -> { $destination }
core-restore-restored-license = восстановлена лицензия: { $source } -> { $destination }
core-restore-would-restore-license-registry = будет восстановлена лицензия в реестре: { $key }
# { $name } is a driver's name.
core-restore-driver-not-restored = драйвер не восстановлен автоматически: { $name }
# { $name } is a driver's name.
core-restore-would-install-driver = будет установлен драйвер { $name } -> { $destination }
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-would-restore-task = будет восстановлена задача планировщика { $name }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-startup-shortcut = будет восстановлен ярлык автозагрузки { $shortcut } -> { $target }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-shortcut = будет восстановлен ярлык { $shortcut } -> { $target }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-would-restore-variable =
    { $scope ->
        [user] будет восстановлена переменная среды пользователя { $name }
       *[machine] будет восстановлена системная переменная среды { $name }
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-left-variable =
    { $scope ->
        [user] переменная среды пользователя { $name } оставлена без изменений: ничего в ней не относится к этому приложению
       *[machine] системная переменная среды { $name } оставлена без изменений: ничего в ней не относится к этому приложению
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-restored-variable =
    { $scope ->
        [user] восстановлена переменная среды пользователя { $name }
       *[machine] восстановлена системная переменная среды { $name }
    }
core-restore-set-uninstaller = настроена программа удаления для { $key }
core-restore-placed-driver-image = размещен образ драйвера { $source } -> { $destination }
# { $name } is a driver's name.
core-restore-configured-driver = настроен драйвер { $name }
# { $name } is a driver's name.
core-restore-created-driver = создан драйвер { $name }
core-restore-ensured-registry-key = обеспечено наличие раздела реестра { $key }
# { $value } is the name of a value in the key.
core-restore-restored-registry-value = восстановлен параметр реестра { $key } { $value }
# "(Default)" is how Windows names the value a key has with no name: write it as Registry Editor does.
core-restore-restored-registry-default = восстановлен параметр реестра { $key } (По умолчанию)
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-configured-service = настроена служба { $name } (запускается от имени { $account })
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-created-service = создана служба { $name } (запускается от имени { $account })
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-restored-task = восстановлена задача планировщика { $name }
core-restore-restored-shortcut = восстановлен ярлык { $path }

## Warnings an install gives, and why it failed. They are shown after it, on the PC it ran on.

core-warning-restore-earlier-not-trusted = Запись о незавершенной установке оставлена без изменений, так как ее могут изменить другие. Все, что изменила та установка, по-прежнему на месте.
# { $app_name } is the app's name. { $count } is how many changes were put back.
core-warning-restore-earlier-undone =
    { $count ->
        [one] Предыдущая установка { $app_name } не была завершена, поэтому сначала она была отменена. Возвращено { $count } изменение.
        [few] Предыдущая установка { $app_name } не была завершена, поэтому сначала она была отменена. Возвращено { $count } изменения.
        [many] Предыдущая установка { $app_name } не была завершена, поэтому сначала она была отменена. Возвращено { $count } изменений.
       *[other] Предыдущая установка { $app_name } не была завершена, поэтому сначала она была отменена. Возвращено { $count } изменения.
    }
# { $app_name } is the app's name.
core-warning-restore-earlier-never-finished = Предыдущая установка { $app_name } не была завершена. Перед установкой она будет отменена.
core-warning-restore-user-data-real-locations = Данные пользователя восстанавливаются в настоящие расположения %AppData%/%ProgramData% этого компьютера, а не в целевую корневую папку.
# { $error } is what went wrong.
core-warning-restore-commit-failed = Не удалось завершить запись об этой установке: { $error }
# { $name } is a service's name.
core-warning-restore-service-still-starting = Службе { $name } отправлен запрос на запуск, но она еще не запустилась. Возможно, она запустится позже. Кроме того, она запустится сама при следующем запуске этого компьютера.
# { $name } is a service's name. { $reason } is why it did not start, as Windows said it.
core-warning-restore-service-not-started = Служба { $name } установлена, но не запустилась: { $reason }. Пока она не запустится, приложение может не работать. Windows повторит попытку при следующем запуске этого компьютера.
# { $name } is a service's name. { $reason } is why it could not be stopped.
core-warning-restore-service-not-stopped = Не удалось сначала остановить службу { $name }: { $reason }. До следующего запуска этого компьютера она может продолжать выполнять старую программу.
core-warning-restore-install-cancelled = Установка отменена.
core-warning-restore-preview-cancelled = Предварительный просмотр отменен.
# { $item } is what was left out, such as a file or a service's name. { $reason } is why, already in words.
core-warning-restore-left-out = Элемент { $item } исключен: { $reason }.
# { $error } is what went wrong.
core-warning-restore-verification-failed = Не удалось выполнить проверку после восстановления: { $error }
# { $runner } says what relies on the file: a service, a scheduled task, a firewall rule, a shortcut, or a registry value.
# { $path } is the file. { $name } is the name of what relies on it. { $refusal } is why the file was not replaced,
# already in words.
core-warning-restore-program-would-run =
    { $runner ->
        [service] Этот пакет не может заменить { $path }, а от этого файла зависит служба { $name }, поэтому ничего не установлено: { $refusal }
        [task] Этот пакет не может заменить { $path }, а от этого файла зависит задача планировщика { $name }, поэтому ничего не установлено: { $refusal }
        [firewall] Этот пакет не может заменить { $path }, а от этого файла зависит правило брандмауэра { $name }, поэтому ничего не установлено: { $refusal }
        [shortcut] Этот пакет не может заменить { $path }, а от этого файла зависит ярлык { $name }, поэтому ничего не установлено: { $refusal }
       *[registry] Этот пакет не может заменить { $path }, а от этого файла зависит параметр реестра { $name }, поэтому ничего не установлено: { $refusal }
    }
core-warning-restore-destination-exists = Место назначения уже существует; чтобы перезаписать его, используйте --force.
# { $path } is a file. { $error } is what went wrong.
core-warning-restore-file-not-written = Не удалось записать { $path }: { $error }
core-warning-restore-not-a-folder = Там, где должна быть эта пустая папка, находится не папка.
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-not-looked-at = Не удалось проверить { $path }: { $error }
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-folder-not-made = Не удалось создать { $path }: { $error }
# Why part of the app was left out, after "was left out:".
core-warning-restore-registration-exists = на этом компьютере это уже настроено иначе, а установка не перехватывает чужие настройки
core-warning-restore-registry-review = Параметры реестра восстановлены из сохраненных структурированных данных, а известные пути приложения заменены путями в целевой корневой папке.
# { $error } is what went wrong.
core-warning-restore-firewall-unreadable = Не удалось прочитать правила Брандмауэра Windows: { $error }
# Why a firewall rule was left out, after "was left out:".
core-warning-restore-firewall-rule-exists = правило с этим именем для этой программы уже существует с другими настройками, а установка его не изменяет
# { $name } is a runtime.
core-warning-restore-prerequisite-unknown = { $name }: для этой среды выполнения нет известного установщика.
# { $name } is a runtime.
core-warning-restore-prerequisite-needs-admin = Для установки { $name } требуются повышенные права; запустите установщик снова от имени администратора.
# { $name } is a runtime.
core-warning-restore-prerequisite-reboot = Компонент { $name } установлен, но для завершения установки требуется перезагрузка.
# { $error } is what went wrong.
core-warning-restore-skipped-not-recorded = Не удалось записать сведения о пропущенной среде выполнения: { $error }
# { $name } is a runtime. { $reason } is why it was skipped, already in words.
core-warning-restore-prerequisite-skipped = { $name }: { $reason }
# { $name } is a runtime. { $reason } is why it could not be installed. { $url } is a web address to get it from.
core-warning-restore-prerequisite-failed = Не удалось установить { $name }: { $reason }. Скачайте этот компонент отсюда: { $url }.
# { $error } is what went wrong.
core-warning-restore-runtime-not-recorded = Не удалось записать сведения об установленной среде выполнения: { $error }
# { $error } is what went wrong.
core-warning-restore-license-source-invalid = Недопустимый путь к лицензии в пакете: { $error }
core-warning-restore-license-needs-admin = Для восстановления данных лицензии в защищенное расположение требуются повышенные права.
core-warning-restore-license-exists = Лицензия в месте назначения уже существует; чтобы перезаписать ее, используйте --force.
# { $error } is what went wrong.
core-warning-restore-license-failed = Не удалось восстановить данные лицензии: { $error }
core-warning-restore-driver-needs-admin = Для регистрации драйвера требуются повышенные права; запустите установщик снова от имени администратора.
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-warning-restore-task-runs-as-system = Задача планировщика { $name } выполняется от имени SYSTEM, с полными правами на этом компьютере.
core-warning-restore-shortcut-exists = Ярлык уже существует; чтобы перезаписать его, используйте --force.
core-warning-restore-shortcut-all-users = Один или несколько восстановленных ярлыков помещены на общий рабочий стол или в общее меню «Пуск».
core-warning-restore-environment-refresh = Изменения переменных среды могут стать видны только в новом процессе, в новом окне командной оболочки или после повторного входа в систему.
# { $name } is a service's name. { $account } is the account it ran as on the PC it came from.
core-warning-restore-service-account = На исходном компьютере служба { $name } выполнялась от имени { $account }. Ее пароля нет в установщике, поэтому служба настроена на запуск от имени LocalService. Если она не работает, укажите ее учетную запись и пароль в оснастке «Службы».
# { $name } is a runtime.
core-warning-restore-prerequisite-pending-reboot = Компонент { $name } еще не зарегистрирован; он должен заработать после перезагрузки.
# { $name } is a runtime.
core-warning-restore-prerequisite-not-verified = Компонент { $name } сообщил об успешной установке, но не зарегистрировался; возможно, установка не удалась.
# { $error } is what went wrong.
core-warning-restore-driver-source-invalid = Недопустимый путь к драйверу в пакете: { $error }
# { $error } is what went wrong.
core-warning-restore-driver-not-read = Не удалось прочитать текущие параметры драйвера: { $error }
core-warning-restore-driver-exists = Служба драйвера уже существует; чтобы перенастроить ее, используйте --force.
core-warning-restore-driver-image-exists = Образ драйвера уже существует; чтобы перезаписать его, используйте --force.
core-warning-restore-driver-hash-bundled = Образ драйвера из пакета не совпадает с записанным хэшем, поэтому он не установлен.
# { $error } is what went wrong.
core-warning-restore-driver-not-placed = Не удалось разместить образ драйвера: { $error }
core-warning-restore-driver-hash-placed = Образ драйвера изменился после размещения, поэтому он не зарегистрирован.
# { $error } is what went wrong.
core-warning-restore-key-not-created = Не удалось создать раздел реестра: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-read = Не удалось прочитать значение, которое будет заменено: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-written = Не удалось записать параметр реестра: { $error }
core-warning-restore-value-exists = Параметр реестра уже существует; чтобы перезаписать его, используйте --force.
# { $error } is what went wrong.
core-warning-restore-environment-key-not-created = Не удалось создать раздел реестра для переменных среды: { $error }
core-warning-restore-variable-exists = Переменная среды уже существует; чтобы перезаписать ее, используйте --force.
# { $error } is what went wrong.
core-warning-restore-variable-not-written = Не удалось записать переменную среды: { $error }
core-warning-restore-value-kind-missing = Не указан тип параметра реестра.
# { $error } is what went wrong.
core-warning-restore-service-not-read = Не удалось прочитать текущие параметры службы: { $error }
core-warning-restore-service-exists = Служба уже существует; чтобы перенастроить ее, используйте --force.
# { $error } is what went wrong.
core-warning-restore-task-not-queried = Не удалось запросить задачу планировщика: { $error }
core-warning-restore-task-exists = Задача планировщика уже существует; чтобы перезаписать ее, используйте --force.
# { $error } is what went wrong.
core-warning-restore-task-folder-not-looked-at = Не удалось проверить папку задачи: { $error }
# { $error } is what went wrong.
core-warning-restore-shortcut-not-put-back = Не удалось создать ярлык и вернуть ярлык, который он заменил: { $error }
# { $error } is what went wrong.
core-warning-restore-command-not-started = Не удалось запустить команду: { $error }
# { $output } is what the command wrote, as it wrote it.
core-warning-restore-command-failed = Ошибка команды: { $output }
# { $error } is what went wrong.
core-warning-restore-powershell-not-started = Не удалось запустить PowerShell: { $error }
