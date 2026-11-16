# Words of the installer that every built installer carries, which runs on the PC an app is moved to and speaks its
# language, and of the uninstaller it leaves behind for Settings. Message names start with installer- for the
# installer's window, installer-cli- for its command line, and uninstaller- for the uninstaller.
#
# The command lines (installer-cli-, uninstaller-cli-) are always shown in English, as scripts read what they print.
# Translate them all the same. Keep their command-line flags, such as --apply and --target-root, exactly as they are.


## The installer's window: its title and header

installer-window-title = Установщик WinAppMigrator
# The window's title in the taskbar. { $app_name } is the name of the app it installs.
installer-window-title-app = Установка { $app_name }
installer-tagline = Переустанавливает приложение, упакованное на другом компьютере
# What an app is called when its package gives it no name.
installer-unnamed-app = Приложение

## When the installer's window cannot open. Each is a paragraph of a message box.

# { $error } is what went wrong, as Windows or the graphics driver says it.
installer-window-failed = Не удалось открыть окно установщика: { $error }
installer-window-failed-without-one = Установить приложение можно и без окна: запустите этот файл из командной строки от имени администратора с параметром --apply, а чтобы выбрать папку, добавьте --target-root и путь к ней.

## Checking the installer when it opens

installer-checking = Проверка установщика…
installer-stub-no-package = Это заготовка установщика WinAppMigrator, и в ней нет пакета приложения. Создайте установщик в WinAppMigrator, а затем запустите здесь полученный файл .exe.
installer-check-stopped = Проверка установщика прервана.
# { $error } is what went wrong, as Windows says it.
installer-locate-failed = Не удалось найти установщик: { $error }
# { $error } is what went wrong reading it.
installer-read-failed = Не удалось прочитать установщик: { $error }
# { $error } is why, such as that the app needs a newer Windows.
installer-cannot-be-used-here = Пакет этого установщика нельзя использовать на этом компьютере: { $error }

## The app, where it goes, and whether the installer is signed

installer-captured-from = Источник
installer-install-location = Папка установки
# The title of the folder picker.
installer-choose-location = Выберите папку установки
installer-folder-has-files = В этой папке уже есть файлы. Файлы с такими же именами будут заменены и вернутся, если вы удалите приложение.
installer-room-needed = Требуемое место
# { $shortfall } says which drive is short of room, such as "C: needs up to 824.0 MB for the app's files, and has 300.0 MB free".
installer-room-short = Здесь может не хватить места: { $shortfall }. Пакет можно распаковать на другом диске, но файлы приложения все равно должны поместиться.
# { $app_name } is the name of an app whose earlier install was cut short.
installer-interrupted = Предыдущая установка { $app_name } не была завершена. Перед установкой она будет отменена.
# Beside "Installer", for whether it proves who made it.
installer-signature = Установщик
installer-signature-checking = Проверка подписи…
installer-signature-unsigned = Не подписан: издатель неизвестен
# { $publisher } is the name on the installer's certificate, such as "Fabrikam, Inc.".
installer-signature-signed-by = Подписан: { $publisher }
installer-signature-unnamed = Подписан издателем без имени
# { $reason } is Windows' own word for what is wrong, such as HashMismatch.
installer-signature-invalid = Подпись этого установщика недействительна ({ $reason }). Возможно, он был изменен после подписания.

## Why the install location cannot be used. Install and Preview Changes stay off while one of these is shown.

# { $app_name } is the name of the app being installed.
installer-location-app-already-there = Приложение { $app_name } уже установлено в этой папке, поэтому эту копию нельзя установить сюда. Чтобы установить ее здесь, сначала удалите копию на этом компьютере в параметрах Windows в разделе «Приложения», а затем снова откройте этот установщик. Или укажите выше новую пустую папку.
# { $other_app } is the name of another app Windows lists as installed in the folder. { $app_name } is the name of the app being installed.
installer-location-other-app-there = В этой папке установлено приложение { $other_app }, поэтому { $app_name } нельзя установить сюда. Укажите выше новую пустую папку. Или, если приложение { $other_app } вам больше не нужно, сначала удалите его в параметрах Windows в разделе «Приложения», а затем снова откройте этот установщик.
installer-location-has-files = В этой папке уже есть файлы, установленные не с помощью WinAppMigrator. Они могут принадлежать другому приложению, поэтому поверх них ничего не устанавливается. Укажите выше новую пустую папку.
# { $reason } is why, such as that it cannot be the Windows folder.
installer-location-refused = Эту папку нельзя использовать: { $reason }. Укажите выше другую папку.

## What the install sets up beyond its files

installer-beyond-files = Что настраивается помимо файлов
# Under the heading above. It is part of what a person agrees to when they click Install.
installer-beyond-files-agreed = Все это выполняется с правами администратора. Если установка завершится ошибкой, внесенные изменения будут возвращены, а вам сообщат обо всем, что вернуть не удалось. Позже установку также можно отменить в параметрах Windows в разделе «Установленные приложения». Замененные ею файлы будут возвращены, а установленные среды выполнения останутся.
installer-beyond-files-none = Ничего: копируются только файлы и параметры в собственных разделах реестра приложения.
# { $problem } is why, such as that the app cannot run on this PC.
installer-cannot-go-here = Этот пакет нельзя установить в это расположение: { $problem }
installer-working-out = Определение того, что будет настроено в этом расположении…
# Said as the reason after "This package cannot go to this location:".
installer-list-stopped = определение действий установки было прервано
# A heading over a list of what else the package asked for and will not get. { $count } is how many.
installer-more-left-out =
    { $count ->
        [one] Исключен еще { $count } элемент
        [few] Исключены еще { $count } элемента
        [many] Исключено еще { $count } элементов
       *[other] Исключено еще { $count } элемента
    }

## Options

installer-options = Параметры
installer-user-data = Данные пользователя
installer-user-data-detail = Вкл.: параметры, а также содержимое папок «Документы» и «Сохраненные игры» помещаются в обычные папки пользователя, который вошел в систему. Выкл.: все это помещается в папку установки, и ничего не настраивается на запуск оттуда.
# { $account } is the name of the account on the other PC whose data this is.
installer-user-data-whose = Здесь хранятся данные учетной записи { $account }.
installer-license = Лицензия
installer-license-detail = Файлы активации и лицензий, а также параметры реестра, в которых хранятся лицензии. Для некоторых из них на этом компьютере может потребоваться повторная активация.

## Dependencies: what else the app needs, and how each is provided

installer-dependencies = Зависимости
installer-dependencies-detail = Что требуется этому приложению и как это предоставляется.
installer-dependencies-none = Дополнительные зависимости от сред выполнения не обнаружены.
# { $feature } is the name of an optional part of Windows, such as NetFx3.
installer-dependency-windows-feature = Компонент Windows: { $feature }
installer-dependency-included = Включено
# { $id } is the package's name in winget, such as Microsoft.VCRedist.2015+.x64.
installer-dependency-winget = Устанавливается с помощью winget: { $id }
installer-dependency-manual = Установите вручную
installer-dependency-built-in = Встроено в Windows
installer-dependency-ships-with-app = Поставляется с приложением

## Buttons, and what is said beside them

installer-install = Установить
installer-preview = Предварительный просмотр изменений
installer-cancel = Отмена
installer-close = Закрыть
installer-open-folder = Открыть папку установки
# { $error } is what went wrong, as Windows says it.
installer-open-folder-failed = Не удалось открыть папку: { $error }
installer-location-blocks = Кнопки «Установить» и «Предварительный просмотр изменений» недоступны, пока папку установки нельзя использовать. Причина указана выше, под этим полем.
installer-package-blocks = Кнопка «Установить» недоступна, так как этот пакет нельзя установить в это расположение. Причина указана выше.
installer-list-changed = То, что будет сделано в этом расположении, изменилось, поэтому ничего не установлено. Просмотрите список ниже и снова нажмите «Установить», если все верно.
installer-cancelled-early = Отменено. Ничего не изменено.

## How far an install has got, beside its progress bar

# { $step } is one of the steps below. { $done } and { $total } are how many of its items are done, and how many there are.
installer-step-counted = { $step }: { $done } из { $total }…
# { $step } is one of the steps below, when it does not count its items.
installer-step-uncounted = { $step }…
installer-step-getting-ready = Подготовка…
installer-step-cancelling = Отмена после текущего шага…
installer-step-putting-back = Возврат внесенных изменений
installer-step-unpacking = Распаковка
installer-step-checking-package = Проверка пакета
installer-step-recovering = Отмена предыдущей незавершенной установки
# What a preview is doing, whatever step it is on.
installer-step-previewing = Определение изменений
installer-step-runtimes = Установка сред выполнения
installer-step-files = Копирование файлов
installer-step-services = Настройка служб
installer-step-drivers = Настройка драйверов
installer-step-registry = Запись параметров реестра
installer-step-license = Восстановление данных лицензии
installer-step-tasks = Настройка задач планировщика
installer-step-shortcuts = Создание ярлыков
installer-step-environment = Настройка переменных среды
installer-step-settings-entry = Добавление приложения в параметры Windows
installer-step-verifying = Проверка установленного
installer-step-undoing = Отмена изменений

## When an install cannot go ahead or stops

installer-carries-no-package = Этот установщик не содержит пакета.
installer-stopped-unexpectedly = Установка неожиданно прервалась.
# { $reason } is what went wrong, as the program says it.
installer-stopped-unexpectedly-because = Установка неожиданно прервалась: { $reason }. Если она успела начаться, при повторном запуске установщик сначала отменит сделанное.
# { $error } is what went wrong reading it.
installer-package-read-failed = Не удалось прочитать пакет: { $error }
installer-no-programdata = Не удалось найти папку ProgramData этого компьютера, поэтому ничего не установлено: в ней хранится запись, по которой установку можно отменить.
# { $error } is why, such as that there is nowhere only administrators can get into.
installer-staging-failed = Не удалось создать промежуточную папку: { $error }
# { $error } is what went wrong unpacking it.
installer-unpack-failed = Не удалось распаковать пакет: { $error }
installer-staging-nowhere = нет расположения, доступного только администраторам, куда можно распаковать пакет
# { $failure } is a folder and what went wrong there, such as "C:\ProgramData\WinAppMigrator: Access is denied.".
installer-staging-nowhere-look-at = нет расположения, доступного только администраторам, куда можно распаковать пакет. Проверьте это расположение: { $failure }

## What came of an install, or of a preview

installer-result = Результат установки
installer-result-preview = Предварительный просмотр: пока ничего не изменено
installer-result-installed = Приложение установлено и прошло проверку хэша.
installer-result-installed-without-left-out = Приложение установлено и прошло проверку хэша, но без того, что ниже указано как исключенное.
installer-result-not-all-put-back = Установка завершилась ошибкой, и не все внесенные изменения удалось вернуть. Оставшиеся изменения перечислены ниже; следующая установка попытается вернуть их снова.
installer-result-cancelled-put-back = Установка отменена, и внесенные изменения возвращены.
installer-result-failed-put-back = Установка завершилась ошибкой, и внесенные изменения возвращены.
installer-result-not-clean = Установка завершилась с проблемами. Просмотрите их ниже.
installer-result-changes-put-back = Возвращенные изменения
installer-result-changes-made = Внесенные изменения
installer-result-changes-planned = Запланированные изменения
installer-result-warnings = Предупреждения
installer-result-errors = Ошибки
installer-result-package-valid = Хэши пакета верны
installer-result-package-invalid = Хэши пакета НЕВЕРНЫ
installer-result-files-valid = Установленные файлы верны
installer-result-files-invalid = Установленные файлы НЕВЕРНЫ
installer-result-reboot = Для завершения установки зависимости требуется перезагрузка. Перезагрузите компьютер, а затем запустите приложение.
# A heading over the list of what was changed. { $count } is how many.
installer-result-changes = Изменения ({ $count })
# After the first part of a long list. { $count } is how many more there are.
installer-result-changes-more = и еще { $count }.
# Over the path of the install's log file.
installer-log = Журнал

## The installer's command line. Shown in English: scripts read it. A { $count } is plain digits, with no separators.

installer-cli-stub = Заготовка установщика WinAppMigrator — не содержит пакета приложения.
installer-cli-stub-how = Создайте установщик в приложении WinAppMigrator, а затем запустите полученный файл .exe на целевом компьютере.
installer-cli-usage-title = Самораспаковывающийся установщик WinAppMigrator.
installer-cli-usage = Запустите его без аргументов, чтобы открыть графический установщик, или:
installer-cli-usage-preview = предварительный просмотр переустановки: --dry-run [--target-root <ПАПКА>]
installer-cli-usage-apply = переустановка:                          --apply [--target-root <ПАПКА>]
installer-cli-no-package = установщик не содержит встроенного пакета
installer-cli-no-programdata = не удалось найти папку ProgramData этого компьютера, поэтому ничего не установлено: в ней хранится запись, по которой установку можно отменить
# { $path } is the folder asked for, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-folder-has-files = Папку { $path } нельзя использовать: { $problem }. Указывайте --force, только если точно знаете, что в ней это приложение
installer-cli-no-location = в этом пакете не указано, где находилось приложение, поэтому укажите --target-root
# { $path } is where the app was on the other PC, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-location-refused = приложение из этого пакета находилось в папке { $path }, и ее нельзя использовать: { $problem }. Укажите --target-root
# { $summary } is how much room it needs where, such as "up to 824.0 MB on C: (356.0 GB free)".
installer-cli-room = Место: { $summary }
# { $note } is something to know about the install.
installer-cli-note = Примечание: { $note }
# { $count } is how many things it sets up beyond its files.
installer-cli-sets-up = Настраивается: { $count }
installer-cli-applied = Переустановка выполнена.
installer-cli-previewed = Предварительный просмотр переустановки выполнен.
# { $count } is how many changes the install logged.
installer-cli-changes-logged = Изменений записано в журнал: { $count }
# { $count } is how many warnings.
installer-cli-warnings = Предупреждения: { $count }
# { $count } is how many errors.
installer-cli-errors = Ошибки: { $count }
# { $count } is how many things the package asked for and did not get.
installer-cli-refused = Отклонено: { $count }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-package-valid = Хэши пакета верны: { $valid }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-reinstall-valid = Хэши переустановки верны: { $valid }
# { $id } is the install's id, such as wam-0f3a….
installer-cli-install-id = Идентификатор установки: { $id }
installer-cli-put-back = Установка завершилась ошибкой, поэтому внесенные изменения возвращены.
# { $path } is the log file.
installer-cli-log = Журнал: { $path }
# { $path } is the log file. Printed before an error.
installer-cli-error-log = журнал: { $path }
# { $error } is what went wrong.
installer-cli-error = ошибка: { $error }

## The uninstaller's window

uninstaller-window-title = Удаление
# The window's title, and its heading. { $app_name } is the name of the app it uninstalls.
uninstaller-window-title-app = Удаление { $app_name }
uninstaller-tagline = Отменяет то, что сделал установщик
uninstaller-reading = Чтение записи об этой установке…
uninstaller-stopped-unexpectedly = Удаление неожиданно прервалось.
uninstaller-what-will-be-removed = Что будет удалено
uninstaller-install-location = Папка установки
uninstaller-files = Файлы
uninstaller-registry-values = Параметры реестра
uninstaller-services = Службы
uninstaller-drivers = Драйверы
uninstaller-scheduled-tasks = Задачи планировщика
uninstaller-environment = Переменные среды
uninstaller-plan-stays = Файлы, которые вы добавили или изменили, останутся вместе с папками, в которых они находятся.
# { $count } is how many files the install replaced.
uninstaller-plan-coming-back =
    { $count ->
        [one] { $count } файл, замененный этой установкой, будет возвращен или помещен рядом с файлом, который с тех пор изменился.
        [few] { $count } файла, замененных этой установкой, будут возвращены или помещены рядом с файлами, которые с тех пор изменились.
        [many] { $count } файлов, замененных этой установкой, будут возвращены или помещены рядом с файлами, которые с тех пор изменились.
       *[other] { $count } файла, замененного этой установкой, будут возвращены или помещены рядом с файлами, которые с тех пор изменились.
    }
# { $runtimes } is a list of runtimes, such as "Microsoft Visual C++ Redistributable, .NET Desktop Runtime 8".
uninstaller-plan-runtimes = Среды выполнения, установленные вместе с приложением, останутся, так как их могут использовать другие программы: { $runtimes }.
uninstaller-options = Параметры
uninstaller-remove-user-data = Также удалить данные пользователя и данные лицензии
# { $count } is how many items of the app's settings, data, and license there are.
uninstaller-remove-user-data-detail =
    { $count ->
        [one] { $count } элемент параметров, данных и лицензии приложения. Он останется, если этот параметр не включен. То, что он заменил, будет возвращено или помещено рядом с ним, если он с тех пор изменился.
        [few] { $count } элемента параметров, данных и лицензии приложения. Они останутся, если этот параметр не включен. То, что они заменили, будет возвращено или помещено рядом с ними, если они с тех пор изменились.
        [many] { $count } элементов параметров, данных и лицензии приложения. Они останутся, если этот параметр не включен. То, что они заменили, будет возвращено или помещено рядом с ними, если они с тех пор изменились.
       *[other] { $count } элемента параметров, данных и лицензии приложения. Они останутся, если этот параметр не включен. То, что они заменили, будет возвращено или помещено рядом с ними, если они с тех пор изменились.
    }
uninstaller-uninstall = Удалить
uninstaller-cancel = Отмена
uninstaller-close = Закрыть
# { $done } and { $total } are how many of the install's items are undone, and how many there are.
uninstaller-step-counted = Отмена установки: { $done } из { $total }…
uninstaller-step = Отмена установки…
uninstaller-done = Приложение удалено.
uninstaller-done-except = Приложение удалено, кроме перечисленного ниже.
uninstaller-waits-for-drive = Часть приложения находится на диске, который не подключен. Подключите этот диск и снова выполните удаление в параметрах Windows.
uninstaller-not-all-undone = Не все удалось отменить. Закройте программы, которые используют приложение, и снова выполните удаление в параметрах Windows.
# Over the path of the uninstall's log file.
uninstaller-log = Журнал
uninstaller-undone = Отменено
uninstaller-left-in-place = Оставлено на месте
uninstaller-could-not-be-undone = Не удалось отменить
# After the first part of a long list. { $count } is how many more there are.
uninstaller-and-more = и еще { $count }.
# Said as the reason when there is no record of the install to read.
uninstaller-no-programdata = не удалось найти папку ProgramData этого компьютера, поэтому прочитать запись об установке невозможно

## When the uninstaller's window cannot open. Each is a paragraph of a message box, followed by a command to type.

uninstaller-window-failed-title = Программа удаления WinAppMigrator
# { $error } is what went wrong, as Windows or the graphics driver says it.
uninstaller-window-failed = Не удалось открыть окно программы удаления: { $error }
uninstaller-window-failed-quiet = Удалить приложение можно и без окна, из командной строки от имени администратора:
# Stands in for this program's own path in that command, if it cannot be found.
uninstaller-this-program = эта программа

## The uninstaller's command line (--quiet). Shown in English: scripts read it.

# { $app_name } is the name of the app.
uninstaller-cli-uninstalling = Удаление { $app_name }.
# { $path } is the log file.
uninstaller-cli-log = Журнал: { $path }
# { $error } is what went wrong.
uninstaller-cli-error = ошибка: { $error }
