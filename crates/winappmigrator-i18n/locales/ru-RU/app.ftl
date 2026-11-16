# Words of the app, WinAppMigrator itself: its one window, with the installed apps listed on the left and, on the
# right, what goes in an installer of the one picked and the button that builds it; and the steps to read another PC
# through WinAppMigrator Client. Message names start with app-.


## The window and its header

# The program's name, in the window's title bar and at the top of the window. Keep it as it is unless your language
# writes product names in a script of its own.
app-name = WinAppMigrator
app-tagline = Создание самораспаковывающегося установщика из установленного приложения
app-theme-to-light = Переключиться на светлую тему
app-theme-to-dark = Переключиться на темную тему
app-reload-list = Обновить список приложений
app-list-loading = Загрузка
# { $count } is how many apps are listed.
app-list-count =
    { $count ->
        [one] { $count } приложение
        [few] { $count } приложения
        [many] { $count } приложений
       *[other] { $count } приложения
    }
app-list-load-failed = Ошибка загрузки
# Shown when the window cannot open at all. { $error } is what went wrong, in the words of Windows or a graphics driver.
app-window-could-not-open = Не удалось открыть окно: { $error }

## The list of installed apps

app-list-search = Поиск по имени или издателю  (Ctrl+F)
app-list-filter-all = Все
# Whether an app is installed for one account or for everyone on the PC.
app-scope-per-user = Для пользователя
app-scope-per-machine = Для всех пользователей
app-scope-unknown = Неизвестно
# An app whose processor architecture (x86, x64, ARM64) is not known.
app-arch-unknown = Неизвестно
app-list-loading-apps = Загрузка установленных приложений…
# { $error } is why the apps could not be listed.
app-list-could-not-list = Не удалось получить список приложений: { $error }
app-list-no-matches = Нет подходящих приложений
app-list-no-matches-hint = Измените текст поиска или фильтр по области.
app-list-worker-disconnected = Связь с процессом поиска приложений потеряна.
# What a screen reader says of an app's row. { $name } is the app's name, { $publisher } who made it, and { $version } its
# version, such as 11.4.2, or — when either is not known.
app-list-row = { $name }, { $publisher }, версия { $version }

## The list's columns, and what a screen reader says each column's heading does when clicked

app-column-name = Имя
app-column-name-sort = Сортировать по имени
app-column-publisher = Издатель
app-column-publisher-sort = Сортировать по издателю
app-column-version = Версия
app-column-version-sort = Сортировать по версии
app-column-scope = Область
app-column-scope-sort = Сортировать по области
# Short for architecture: x86, x64, or ARM64. The column is narrow.
app-column-arch = Архит.
app-column-arch-sort = Сортировать по архитектуре

## Where apps are read from: this PC, a drive from another PC, a disk image of one, or another PC on the network

app-source = Источник сканирования
app-source-this-pc = Этот компьютер
app-source-drive = Подключенный диск…
app-source-image = Образ диска…
app-source-network = Сетевой компьютер…
app-source-attaching-image = Подключение образа только для чтения…
# { $computer } is the other PC's name, such as OLD-PC, and { $address } its network address, such as 192.168.1.20.
app-source-network-pc = { $computer } по сети ({ $address })
app-source-disconnect = Отключить
app-source-change = Изменить…
# The title of the window that picks a disk image. .vhd and .vhdx are file extensions.
app-source-pick-image-title = Выберите образ диска другого компьютера (.vhd или .vhdx)
app-source-pick-image-filter = Образы дисков
# { $path } is the disk image file.
app-source-image-has-no-windows = В образе { $path } нет тома с Windows.
app-source-image-not-attached = Не удалось подключить образ диска.
app-source-pick-drive-title = Выберите корневую папку подключенного диска (например, E:\)
# { $path } is the folder picked, such as E:\. \Windows and \Users are folder names, as Windows writes them.
app-source-not-a-windows-drive = Папка { $path } не похожа на системный диск Windows (в ней нет \Windows и \Users). Выберите корневую папку диска с другого компьютера.
# Said of a drive or disk image with a Windows older than Windows 10. { $windows } is its name, such as "Windows 7
# Professional Service Pack 1, 32-bit". { $is_32_bit } is "yes" when that Windows is 32-bit, which adds the last
# sentence, and "no" when it is not.
app-source-older-windows =
    Здесь установлена { $windows }. Установщики, созданные из этого источника, предназначены для Windows 10 и 11. С параметром «Данные пользователя» переносятся и параметры, которые Windows незаметно хранила отдельно для старого приложения, а среды выполнения, которых нет в Windows 11, например DirectX 9, устанавливаются вместе с приложением.{ $is_32_bit ->
        [yes] { " " }Приложения здесь 32-разрядные, поэтому они устанавливаются в Program Files (x86), где их хранит 64-разрядная Windows.
       *[no] { "" }
    }
# The same, of a Windows whose name could not be read. { $is_32_bit } is as above.
app-source-older-windows-unnamed =
    Здесь установлена более старая версия Windows. Установщики, созданные из этого источника, предназначены для Windows 10 и 11. С параметром «Данные пользователя» переносятся и параметры, которые Windows незаметно хранила отдельно для старого приложения, а среды выполнения, которых нет в Windows 11, например DirectX 9, устанавливаются вместе с приложением.{ $is_32_bit ->
        [yes] { " " }Приложения здесь 32-разрядные, поэтому они устанавливаются в Program Files (x86), где их хранит 64-разрядная Windows.
       *[no] { "" }
    }

## The details panel, before an app is picked, and the app picked

app-details-pair-first = Выполните сопряжение со старым компьютером
app-details-pair-first-hint = Когда на нем будет запущен WinAppMigrator Client и вы введете его код, здесь появится список его приложений, и вы сможете создать установщик любого из них так же, как на этом компьютере.
app-details-pick-an-app = Выберите приложение
app-details-pick-an-app-hint = Выберите приложение в списке, чтобы указать, что включить в установщик, и создать его.
# { $publisher } is who made the app, and { $version } its version, such as 11.4.2.
app-details-publisher-and-version = { $publisher }, версия { $version }
# { $version } is the app's version, such as 11.4.2.
app-details-unknown-publisher-and-version = Неизвестный издатель, версия { $version }
app-details-unknown-publisher = Неизвестный издатель

## What goes in besides the program

app-option-license = Данные лицензии и активации
app-option-license-hint = Могут быть привязаны к этому компьютеру и потребовать повторной активации на новом.
app-option-user-data = Данные пользователя
# Documents, Saved Games, and ProgramData are folders of Windows: use the names Windows gives them in your language,
# except ProgramData, which keeps its name.
app-option-user-data-hint = Параметры и история, папки «Документы», «Сохраненные игры» и ProgramData.
app-option-whose-data = Чьи данные
app-option-whose-data-hint = Установщик содержит данные одной учетной записи. Они переносятся пользователю, который вошел в систему на компьютере, где запущен установщик.

## Building the installer

app-output-folder = Выходная папка
# An example of a folder, shown in the empty field. "You" stands for the name of an account.
app-output-folder-example = C:\Users\Имя\WinAppMigrator
app-output-folder-not-full = Укажите полный путь к папке, например D:\Установщики.
app-build-create = Создать установщик
app-build-create-again = Создать установщик снова
app-build-cancel = Отмена
app-build-cancelling = Отмена после текущего шага…
app-build-cancelled = Создание отменено. Установщик не записан.
# winappmigrator-installer.exe is a file name.
app-build-no-stub = Заготовка установщика (winappmigrator-installer.exe) не найдена рядом с этим приложением, поэтому создать самораспаковывающийся установщик невозможно.
app-build-select-an-app-first = Сначала выберите приложение для переноса.
app-build-look-again-first = Прежде чем создать установщик, это приложение нужно проверить снова.
# { $reason } is what went wrong, in the program's own words, often in English.
app-build-stopped-unexpectedly = Создание неожиданно прервалось: { $reason }. Если в выходной папке остался установщик, не используйте его.
app-build-worker-disconnected = Связь с процессом переноса потеряна.
# { $path } is the output folder, and { $error } what Windows said went wrong.
app-build-no-output-folder = Не удалось создать выходную папку { $path }: { $error }
# { $error } is what went wrong.
app-build-no-staging-folder = Не удалось создать защищенную промежуточную папку, поэтому ничего не создано: { $error }
# { $error } is what went wrong, as a sentence.
app-build-could-not-build = Не удалось создать установщик. { $error }
app-build-no-installer = Создание завершилось, но установщик не получен.
# { $error } is what went wrong.
app-build-hash-check-did-not-run = Установщик записан, но выполнить проверку его хэша не удалось: { $error }. Не используйте его.

## How far a build has got

app-progress-preparing = Определение содержимого…
app-progress-checking-room = Проверка свободного места…
app-progress-bundling = Упаковка сред выполнения и драйверов…
# { $done } is how many files have been read so far, and { $total } how many there are.
app-progress-packing = Чтение и сжатие файлов: { $done } из { $total }
# { $done } is how many of the installer's records have been looked over so far, and { $total } how many there are.
app-progress-looking-over = Проверка содержимого установщика: { $done } из { $total }…
app-progress-writing = Запись установщика…
app-progress-verifying = Проверка хэшей…

## What a build made

app-result-failed-hash-check = Не используйте этот установщик: он не прошел проверку хэша.
# { $count } is how many files were gone when the build came to read them.
app-result-files-gone =
    { $count ->
        [one] Установщик создан, но { $count } файл исчез после проверки приложения, поэтому его нет в установщике. См. раздел «Предупреждения» ниже.
        [few] Установщик создан, но { $count } файла исчезли после проверки приложения, поэтому их нет в установщике. См. раздел «Предупреждения» ниже.
        [many] Установщик создан, но { $count } файлов исчезли после проверки приложения, поэтому их нет в установщике. См. раздел «Предупреждения» ниже.
       *[other] Установщик создан, но { $count } файла исчезли после проверки приложения, поэтому их нет в установщике. См. раздел «Предупреждения» ниже.
    }
# { $count } is how many files could not be read into the installer.
app-result-files-not-captured =
    { $count ->
        [one] Установщик создан, но { $count } файл не удалось включить в него, обычно потому, что файл заблокирован или недоступен для чтения. Без него приложение может не работать. См. раздел «Предупреждения» ниже.
        [few] Установщик создан, но { $count } файла не удалось включить в него, обычно потому, что они заблокированы или недоступны для чтения. Без них приложение может не работать. См. раздел «Предупреждения» ниже.
        [many] Установщик создан, но { $count } файлов не удалось включить в него, обычно потому, что они заблокированы или недоступны для чтения. Без них приложение может не работать. См. раздел «Предупреждения» ниже.
       *[other] Установщик создан, но { $count } файла не удалось включить в него, обычно потому, что они заблокированы или недоступны для чтения. Без них приложение может не работать. См. раздел «Предупреждения» ниже.
    }
app-result-mentions-who = Установщик создан, но в нем все еще есть сведения о том, от кого он получен. Прочитайте их ниже, прежде чем кому-либо его передавать.
app-result-ready = Установщик готов. Его пакет прошел проверку хэша.
app-result-files = Файлы
app-result-installer-size = Размер установщика
app-result-warnings = Предупреждения
app-result-log = Журнал
app-result-copy-error = Копировать ошибку
app-result-installer = Установщик
app-result-data-file = Файл данных
app-result-open-folder = Открыть папку
app-result-copy-path = Копировать путь
# { $error } is what Windows said went wrong.
app-result-could-not-open-folder = Не удалось открыть папку: { $error }
# { $installer } and { $data_file } are file names, such as App_installer.exe and App_installer.wamdata.
app-result-two-files = Это приложение слишком велико для одного исполняемого файла, поэтому его установщик состоит из двух файлов: { $installer } и { $data_file }. Они должны храниться вместе: скопируйте оба файла на другой компьютер в одну папку, не меняя их имен. Если вы подпишете установщик, подпись распространится и на файл данных.
# { $app_name } is the app's name. "Unknown publisher" is what Windows shows when it asks to run an unsigned program: use
# Windows' own words for it in your language.
app-result-run-it = Запустите его на другом компьютере, чтобы переустановить { $app_name }. Он проверяет свои файлы по SHA-256. Windows показывает «Неизвестный издатель», так как создаваемые вами установщики не подписаны.
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-verification-errors = Ошибки проверки ({ $count })
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-warnings-heading = Предупреждения ({ $count })
# After the first of a long list of warnings. { $count } is how many more there are.
app-warnings-and-more = и еще { $count }.

## Looking at an app, to see what an installer of it would hold

app-scan-getting-ready = Подготовка к проверке приложения…
app-scan-install-record = Чтение сведений о том, как оно было установлено…
app-scan-near-matches = Поиск того, что тоже может к нему относиться…
app-scan-system-items = Поиск его служб, задач и ярлыков…
app-scan-files = Просмотр его файлов…
app-scan-registry = Чтение того, что связывает его с Windows…
app-scan-finishing = Завершение…
app-scan-look-again = Проверить снова
app-scan-stop = Остановить
app-scan-running-hint = Пока идет проверка, можно выбрать другое приложение или изменить состав включаемого.
# { $error } is why, in words that follow a colon.
app-scan-failed = Не удалось проверить это приложение: { $error }
app-scan-try-again = Повторить попытку
app-scan-stopped = Остановлено. Установщик можно будет создать после проверки этого приложения.
# { $reason } is what went wrong, in the program's own words, often in English.
app-scan-stopped-unexpectedly = Проверка неожиданно прервалась: { $reason }.

## How big an installer can be, and whether there is room for it

app-room-files = Файлы
# Over how big the installer can be: up to that size.
app-room-up-to = Максимум
app-room-free = Свободно
# { $drive } is a drive, such as C:.
app-room-free-on = Свободно на диске { $drive }
# How much room a drive has, when it would not say.
app-room-free-unknown = Неизвестно
app-room-most-it-could-need = Максимальный объем, который может потребоваться, до сжатия.
# { $drive } is a drive, such as C:, { $needed } how much room it needs, such as 1.2 GB, and { $free } how much it has.
app-room-drive-short = Для создания на диске { $drive } нужно до { $needed } свободного места, а доступно { $free }.
# { $drives } is one sentence like the one above for each drive without the room, one after another.
app-room-not-enough = Недостаточно места. { $drives } Выберите выходную папку на другом диске или исключите что-нибудь.
# { $note } is a sentence, without its full stop, about a drive whose room could not be checked. Use your language's
# full stop.
app-room-could-not-tell = { $note }.

## What goes in the installer, part by part

app-footprint-heading = Что входит в установщик
app-footprint-working-out = Определение того, что с этим связано…
# { $error } is why.
app-footprint-would-not-make-a-package = Из оставшегося нельзя составить пакет: { $error }
# { $list } is the names of what else is left out, such as a service and a shortcut.
app-footprint-also-left-out = Из-за этого также исключается следующее: { $list }, так как то, что они запускают или на что ссылаются, будет отсутствовать. На другом компьютере они не будут настроены.
# { $list } is the names of the first few of what else is left out, and { $count } how many more there are.
app-footprint-also-left-out-and-more = Из-за этого также исключается следующее: { $list } и еще { $count }, так как то, что они запускают или на что ссылаются, будет отсутствовать. На другом компьютере они не будут настроены.
# { $list } is the names of license files. The option is the one named "License and Activation Data".
app-footprint-licenses-still-go = Данные лицензии среди исключенного все равно включаются, так как их включение определяет параметр «Данные лицензии и активации»: { $list }.
# { $elapsed } is how long ago, in minutes and seconds such as 45:00, or hours, minutes, and seconds such as 1:05:00.
# "Look Again" names the button of that name.
app-footprint-looked-at-long-ago = Проверка выполнялась { $elapsed } назад. Файлы считываются при создании установщика, поэтому изменения будут учтены, а все, что к тому времени исчезнет, будет указано в списке. Чтобы учесть новое, нажмите «Проверить снова».
# Headings of the parts of the list, each with a line that says what is in it.
app-footprint-program-files = Файлы программы
app-footprint-data-folders = Папки данных
# What could go in with the User Data option, when nothing else is in this part. { $size } is how much, such as 1.2 GB.
app-footprint-folders-with-user-data = { $size } с параметром «Данные пользователя»
# { $included } is how many folders go in, { $count } how many there are, and { $size } how much those that go in hold.
app-footprint-folders-summary =
    { $count ->
        [one] { $included } из { $count } папки, { $size }
        [few] { $included } из { $count } папок, { $size }
        [many] { $included } из { $count } папок, { $size }
       *[other] { $included } из { $count } папки, { $size }
    }
# { $size } is how much, such as 1.2 GB, and { $count } in how many files.
app-footprint-data-only-with-user-data =
    { $count ->
        [one] { $size } данных этого приложения в { $count } файле будут включены только с параметром «Данные пользователя».
        [few] { $size } данных этого приложения в { $count } файлах будут включены только с параметром «Данные пользователя».
        [many] { $size } данных этого приложения в { $count } файлах будут включены только с параметром «Данные пользователя».
       *[other] { $size } данных этого приложения в { $count } файла будут включены только с параметром «Данные пользователя».
    }
# A folder's row. { $count } is how many files it holds, and { $detail } where it is or why it cannot be left out.
app-footprint-folder-row =
    { $count ->
        [one] { $count } файл. { $detail }
        [few] { $count } файла. { $detail }
        [many] { $count } файлов. { $detail }
       *[other] { $count } файла. { $detail }
    }
app-footprint-services = Службы
app-footprint-startup = Запускается автоматически
app-footprint-tasks = Задачи планировщика
app-footprint-shortcuts = Ярлыки
# { $included } is how many of the part's things go in, and { $count } how many there are.
app-footprint-items-summary = { $included } из { $count }
# { $why } is what else was left out that this goes with.
app-footprint-left-out-too = Исключено вместе с: { $why }
# A row's detail. { $kind } is what it is, such as Service, and { $detail } what it does.
app-footprint-change-detail = { $kind }. { $detail }
app-footprint-integrations = Что связывает его с Windows
# { $included } is how many kinds go in, such as file types and firewall rules, and { $count } how many kinds there are.
app-footprint-integrations-summary =
    { $count ->
        [one] { $included } из { $count } типа
        [few] { $included } из { $count } типов
        [many] { $included } из { $count } типов
       *[other] { $included } из { $count } типа
    }
app-footprint-integration-nothing-left = Ничего не осталось из-за того, что вы исключили
# { $count } is how many things of this kind the installer sets up.
app-footprint-integration-things =
    { $count ->
        [one] Настраивается { $count } элемент
        [few] Настраиваются { $count } элемента
        [many] Настраивается { $count } элементов
       *[other] Настраивается { $count } элемента
    }
app-footprint-prerequisites = Что нужно для работы
# { $count } is how many runtimes, such as Visual C++ Redistributable.
app-footprint-prerequisites-summary =
    { $count ->
        [one] { $count } среда выполнения
        [few] { $count } среды выполнения
        [many] { $count } сред выполнения
       *[other] { $count } среды выполнения
    }
app-footprint-near-matches = Может также относиться к нему
# { $ticked } is how many were ticked, and { $count } how many there are.
app-footprint-near-matches-summary = Добавлено: { $ticked } из { $count }
app-footprint-near-matches-hint = Не включаются в установщик, если вы их не отметите. После отметки приложение проверяется снова теми же проверками, что и остальное, поэтому все, что похоже на пароль, по-прежнему не включается.
# { $why } is why it might be part of the app, as a sentence.
app-footprint-near-match-with-user-data = Включается с параметром «Данные пользователя». { $why }
# How much a folder holds, when not all of it could be measured. { $size } is how much was, such as 1.2 GB.
app-footprint-near-match-over = более { $size }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-folder = Папка: { $place }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-hidden = Скрытая часть приложения: { $place }
app-footprint-nothing-left-out = Ничего не исключено из-за сходства с паролем, лицензией или сертификатом.
app-footprint-left-out = Что исключено и почему
# { $count } is how many things were left out.
app-footprint-left-out-summary =
    { $count ->
        [one] { $count } элемент
        [few] { $count } элемента
        [many] { $count } элементов
       *[other] { $count } элемента
    }
# Above a list of things left out for one reason. { $title } is the reason.
app-footprint-left-out-group = { $title }:
app-footprint-refused = Ни одному установщику не разрешено настраивать следующее:
# A heading over a list that opens and closes. { $count } is how many are in it.
app-footprint-worth-knowing = Полезно знать ({ $count })
# { $list } is what kinds of things were looked for and none found, from the five below.
app-none-found = Не найдено: { $list }.
app-none-found-services = службы
app-none-found-startup = элементы автозагрузки
app-none-found-tasks = задачи планировщика
app-none-found-shortcuts = ярлыки
app-none-found-integrations = связи с Windows

## Network PC: reading another PC through WinAppMigrator Client, in three steps. "WinAppMigrator Client" is the name of a
## program, and the old PC is the one the apps are moved off.

app-network-step-client = 1. Скопируйте клиент на старый компьютер
app-network-step-client-hint = WinAppMigrator Client запускается на компьютере, где сейчас находится приложение (Windows 7 или более поздней версии). Он позволяет считывать данные только этому компьютеру, в зашифрованном виде и по коду, который он показывает.
# { $drive } is a USB drive, such as E:\.
app-network-save-to-usb = Сохранить на USB-накопитель { $drive }
# { $drive } is a USB drive, such as E:\, and { $name } the name it was given, such as BACKUP.
app-network-save-to-named-usb = Сохранить на USB-накопитель { $drive } ({ $name })
app-network-save-to-folder = Сохранить в папку…
app-network-look-for-usb-again = Повторить поиск USB-накопителей
app-network-no-usb = USB-накопитель не подключен. Подключите его или сохраните клиент в папку, доступную со старого компьютера, например в общую папку в вашей сети.
# { $path } is where the client was saved, such as E:\WinAppMigrator Client.exe.
app-network-client-saved = Сохранено как { $path }. На старом компьютере закройте переносимое приложение, затем дважды щелкните этот файл и разрешите ему вносить изменения. Он покажет код.
app-network-step-pick = 2. Выберите старый компьютер
app-network-looking = Поиск компьютеров с запущенным клиентом в вашей сети…
app-network-type-name = Или введите его имя или адрес
# An example, shown in the empty field: a PC's name, or its network address.
app-network-type-name-example = OLD-PC или 192.168.1.20
app-network-step-code = 3. Введите показанный код
app-network-code = Код сопряжения
app-network-connect = Подключиться
app-network-pairing = Сопряжение…
# Why pairing did not work, shown on its own.
app-network-pairing-stopped = сопряжение прервано до завершения
app-network-wrong-code = это не тот код, который показывает старый компьютер. Проверьте его и повторите попытку.
# { $error } is what went wrong.
app-network-not-reached = не удалось связаться со старым компьютером: { $error }
app-network-pick-folder-title = Выберите папку для WinAppMigrator Client
# { $path } is the folder, and { $error } what Windows said went wrong.
app-client-could-not-save = не удалось сохранить клиент в папке { $path }: { $error }
# scripts\build-client.ps1 is a file name.
app-client-not-carried = эта сборка WinAppMigrator не содержит клиента. Соберите его с помощью scripts\build-client.ps1, а затем снова соберите приложение.
# { $error } is what Windows said went wrong.
app-client-could-not-save-note = не удалось сохранить рядом с ним файл с инструкцией: { $error }
# A note saved as a text file beside the client, for whoever runs it on the old PC. Keep its lines short, as it is read in
# Notepad. "WinAppMigrator Client.exe" is a file name. "Network PC..." names the Scan Source button: use its words in your
# language.
app-client-note =
    WinAppMigrator Client
    
    Запустите эту программу на компьютере, с которого переносите приложения:
    Windows 7, 8, 10 или 11.
    
    1. Закройте переносимое приложение.
    2. Дважды щелкните «WinAppMigrator Client.exe» и разрешите ему вносить
       изменения.
    3. Он покажет шестизначный код сопряжения. В WinAppMigrator на новом
       компьютере выберите «Сетевой компьютер...», выберите этот компьютер
       и введите код.
    4. Когда новый компьютер закончит работу, нажмите клавишу ВВОД в окне
       клиента или закройте его.
    
    Клиент только считывает данные. Пока он работает, он хранит снимок диска
    Windows и разрешает в Брандмауэре Windows подключения к двум своим портам
    из локальной сети, а при остановке убирает и то, и другое. Данные по сети
    передаются в зашифрованном виде, и прочитать их может только компьютер,
    у которого есть код.
