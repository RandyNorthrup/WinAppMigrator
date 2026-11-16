# Words the core says about what an install may do: why an install location or something in a package is refused,
# the installer's list of what an app sets up beyond its files, the checks of a package and of an install, and whether
# there is room for either. Message names start with core-.
#
# Many messages here are a reason, the second half of a sentence that starts "... was left out:" or "that install
# location cannot be used:", so they start with a small letter and have no full stop.


## Why an install location cannot be used. Shown in red under the Install Location field.

core-review-location-state-root = здесь установки хранят свои записи
core-review-location-has-files = в ней уже есть файлы, а записи об установке сюда нет, поэтому используйте пустую папку
core-policy-location-not-full-path = нужно указать полный путь
core-policy-location-parent-step = путь не должен содержать «..»
core-policy-location-not-plain = это должен быть обычный путь на диске этого компьютера
core-policy-location-whole-drive = это не может быть весь диск
core-policy-location-windows = это не может быть папка Windows
core-policy-location-runs-by-itself = эта папка принадлежит Windows, или ее содержимое запускается или загружается автоматически
core-policy-location-shared = в ней хранится и другое содержимое, поэтому приложению нужна вложенная папка

## Whether an app can run on this PC at all, said before anything is installed.

# { $reason } is why, as the second half of a sentence, such as "App.exe is built for Arm64, and this PC is x64, so
# Windows would not start it".
core-review-cannot-run-here = это приложение не может работать на этом компьютере, поэтому ничего не изменено: { $reason }
core-policy-architecture-x86 = x86 (32 бита)
core-policy-architecture-x64 = x64
core-policy-architecture-arm64 = Arm64
core-policy-architecture-arm32 = Arm (32 бита)
core-policy-architecture-unknown = неизвестного типа
# { $code } is the number Windows gives a kind of processor, such as 0x01c4.
core-policy-machine-type = с кодом типа машины { $code }
# { $names } is a list of program names, such as "a.exe, b.exe, c.exe", and { $count } how many more there are.
core-policy-names-and-more = { $names } и еще { $count }
core-policy-fit-programs-not-said = Этот установщик не сообщает, что нужно его программам, поэтому возможность их запуска на этом компьютере не проверялась.
core-policy-fit-architecture-not-told = Не удалось определить архитектуру этого компьютера, поэтому возможность запуска программ этого приложения на нем не проверялась.
core-policy-fit-main-not-said = Этот установщик не сообщает, что нужно его основной программе, поэтому сведения здесь основаны на остальном его содержимом.
# { $driver } is a driver's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-driver = его драйвер { $driver } создан для архитектуры { $built_for }, а архитектура этого компьютера — { $here }. Драйвер должен точно соответствовать Windows, поэтому это приложение не может здесь работать
# { $program } is a program's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-program = программа { $program } создана для архитектуры { $built_for }, а архитектура этого компьютера — { $here }, поэтому Windows ее не запустит
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-none-start = ни одна из его программ не может запуститься на этом компьютере (архитектура { $here }): { $programs }
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-some-other-architecture = Некоторые программы этого приложения созданы для другой архитектуры и не запустятся на этом компьютере (архитектура { $here }): { $programs }. Остальная часть приложения запустится, поэтому это может быть неважно.
# { $here } is a kind of processor, such as "Arm64". { $programs } is a list of program names.
core-policy-fit-emulated = Архитектура этого компьютера — { $here }, поэтому Windows будет запускать эти программы через эмулятор: { $programs }. Большинство программ при этом работает нормально, но некоторые — нет.
# { $program } is a program's file name. { $needs } and { $runs } are versions of Windows, such as 10.0 and 6.1.
core-policy-fit-needs-newer = программа { $program } требует Windows { $needs } или более поздней версии, а на этом компьютере установлена версия { $runs }, поэтому Windows ее не запустит
# { $programs } is a list of program names.
core-policy-fit-some-need-newer = Некоторым программам этого приложения нужна более новая версия Windows, чем эта, и здесь они не запустятся: { $programs }.
# { $from_build } and { $this_build } are Windows build numbers, such as 22631, written as Windows writes them.
core-policy-fit-older-build = Это приложение работало в Windows сборки { $from_build }, а на этом компьютере установлена сборка { $this_build }. Его программы сообщают, что могут запускаться в этой версии Windows, поэтому оно может работать, но то, на что оно полагалось там, здесь может отсутствовать.
core-policy-fit-build-not-told = Не удалось определить сборку этой Windows, поэтому она не сравнивалась со сборкой, в которой работало приложение.

## Room on a drive for a build or an install.

# Each of these ends the sentence core-space-shortfall, after "free for".
core-space-for-unpacking = распаковки пакета
core-space-for-install-record = записи об установке
core-space-for-app-files = файлов приложения
core-space-for-what-is-replaced = хранения того, что заменяется
core-space-for-build-work = рабочих данных при создании
core-space-for-installer = установщика
core-space-for-package = пакета
core-space-for-unpacking-and-installing = распаковки и установки пакета
# What goes between two of those, as in "unpacking the package and the app's files". Keep the quotes: they keep the
# spaces.
core-space-purposes-joiner = { " и " }
# { $drive } is a drive, such as C:, or a folder. { $needed } and { $free } are sizes, such as 1.2 GB. { $purposes } is
# what the room is for, such as "unpacking the package and the app's files".
core-space-shortfall = { $drive } — нужно до { $needed } свободного места для { $purposes }, а доступно { $free }
# { $drive } is a drive, such as C:, or a folder. { $needed } is a size, such as 1.2 GB.
core-space-not-told = { $drive } — объем свободного места не сообщается, поэтому не проверено, поместятся ли нужные { $needed }
# One drive in a list of where room is needed. { $needed } and { $free } are sizes, and { $drive } a drive, such as C:.
core-space-summary = до { $needed } на { $drive } (свободно { $free })
# { $needed } is a size, and { $drive } a drive, such as C:.
core-space-summary-not-told = до { $needed } на { $drive } (объем свободного места определить не удалось)
# What goes between two sentences of core-space-shortfall inside core-review-no-room. Keep the quotes: they keep the
# space.
core-review-sentence-joiner = { ". " }
# { $shortfalls } is one or more sentences of core-space-shortfall.
core-review-no-room = недостаточно места, поэтому ничего не изменено. { $shortfalls }. Освободите там место или выберите папку установки на другом диске.

## The installer's list of what an app sets up beyond its files: what each kind is called.

core-review-kind-service = Служба
core-review-kind-driver = Драйвер
core-review-kind-scheduled-task = Задача планировщика
core-review-kind-autostart = Запускается автоматически
core-review-kind-shortcut = Ярлык
core-review-kind-path = PATH
core-review-kind-environment-variable = Переменная среды
core-review-kind-shell-extension = Расширение оболочки
core-review-kind-menu-command = Команда меню
core-review-kind-link-type = Тип ссылки
core-review-kind-file-type = Тип файла
core-review-kind-com = COM
core-review-kind-firewall-rule = Правило брандмауэра
core-review-kind-event-log-source = Источник журнала событий
core-review-kind-font = Шрифт
core-review-kind-odbc-driver = Драйвер ODBC
core-review-kind-odbc-source = Источник данных ODBC
core-review-kind-compatibility-setting = Параметр совместимости
core-review-kind-default-apps = Приложения по умолчанию
core-review-kind-program-name = Имя программы
core-review-kind-other-settings = Другие параметры

## The installer's list of what an app sets up: what each line says of it.

# { $file } is a program's file name. { $account } is the Windows account it runs as, such as LocalSystem. { $starts }
# says when it starts, and is not shown itself; for "other", { $start } is one of the core-review-start- words below.
core-review-service-detail =
    { $starts ->
        [automatic] Выполняет { $file } от имени { $account }, запускается сейчас и при каждом запуске компьютера
        [delayed] Выполняет { $file } от имени { $account }, запускается сейчас и вскоре после каждого запуска компьютера
        [manual] Выполняет { $file } от имени { $account }, запускается только по запросу
        [disabled] Выполняет { $file } от имени { $account }, отключена
        [other] Выполняет { $file } от имени { $account }, { $start }
       *[unknown] Выполняет { $file } от имени { $account }
    }
# { $account } is a Windows account, such as LocalService.
core-policy-account-custom = { $account } (особая учетная запись не восстанавливается)
# { $file } is a driver's file name.
core-review-driver-detail = Загружает { $file } непосредственно в Windows
# { $file } is a driver's file name. { $start } is when Windows starts it: one of the core-review-start- words below.
core-review-driver-detail-starts = Загружает { $file } непосредственно в Windows, { $start }
# When a service or driver starts, ending a sentence such as "Loads disk.sys into Windows itself, starts …".
core-review-start-boot = запускается вместе с Windows раньше всего остального
core-review-start-system = запускается вместе с Windows во время ее загрузки
core-review-start-automatic = запускается при каждом запуске компьютера
core-review-start-delayed = запускается вскоре после каждого запуска компьютера
core-review-start-manual = запускается только по запросу
core-review-start-disabled = никогда не запускается: запуск отключен
core-review-start-unknown = запускается способом, который Windows не указывает
# { $account } is the account a task runs as: an account's name, or core-review-task-runs-as-you, or
# core-review-task-runs-as-creator.
core-review-task-detail = Выполняется от имени: { $account }
# { $account } is the account a task runs as, as for core-review-task-detail.
core-review-task-detail-highest = Выполняется от имени: { $account }, с наивысшими правами
core-review-task-runs-as-you = вы (пользователь, вошедший в систему)
core-review-task-runs-as-creator = пользователь, создавший задачу
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-autostart-detail =
    Запускается { $scope ->
        [everyone] при входе в систему любого пользователя
       *[you] при вашем входе в систему
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shortcut-startup-detail =
    Ярлык в папке автозагрузки { $scope ->
        [everyone] для всех пользователей этого компьютера
       *[you] для вас
    }
# { $file } is the file name of what the shortcut opens. { $scope } is whom it is set up for: everyone on this PC,
# or you. It is not shown itself.
core-review-shortcut-detail =
    Открывает { $file } { $scope ->
        [everyone] для всех пользователей этого компьютера
       *[you] для вас
    }
# { $name } is an environment variable's name, such as PATH.
core-review-path-title = Добавляет в { $name }
core-review-path-install-folder = Папка установки
# { $path } is a folder inside the install folder, such as bin\tools.
core-review-path-below-install-folder = Папка установки\{ $path }
# { $value } is the variable's value, as it will be set.
core-review-variable-detail = Значение: { $value }
# { $count } is how many registry values.
core-review-other-settings-title =
    { $count ->
        [one] { $count } параметр вне собственных разделов приложения
        [few] { $count } параметра вне собственных разделов приложения
        [many] { $count } параметров вне собственных разделов приложения
       *[other] { $count } параметра вне собственных разделов приложения
    }
# { $keys } is a list of registry keys, such as HKCU\Software\Vendor.
core-review-other-settings-detail = В { $keys }
# { $keys } is a list of registry keys, and { $count } how many more there are.
core-review-other-settings-detail-more = В { $keys } и еще { $count }
# { $way } says whether the rule allows or blocks, and incoming or outgoing connections; it is not shown itself.
# { $ports } is a list of port numbers, and { $port_count } how many. { $program } is the program's file name.
# { $networks } is a list of the kinds of network Windows names in English (domain, private, public), and
# { $network_count } how many.
# The kinds of network a firewall rule is for, as Windows Defender Firewall names its profiles in your language. They
# fill { $networks } in core-review-firewall-detail, joined into a list.
core-review-network-domain = доменных
core-review-network-private = частных
core-review-network-public = общедоступных
core-review-firewall-detail =
    { $way ->
        [allow-in] Разрешает входящие
        [allow-out] Разрешает исходящие
        [block-in] Блокирует входящие
       *[block-out] Блокирует исходящие
    } { $port_count ->
        [0] подключения
        [1] подключения через порт { $ports }
        [one] подключения через порты { $ports }
        [few] подключения через порты { $ports }
        [many] подключения через порты { $ports }
       *[other] подключения через порты { $ports }
    } для { $program } { $network_count ->
        [0] во всех сетях
        [one] в { $networks } сетях
        [few] в { $networks } сетях
        [many] в { $networks } сетях
       *[other] в { $networks } сетях
    }
# { $server } is the file name of what Explorer loads. { $scope } is whom it is set up for: everyone on this PC, or
# you. It is not shown itself.
core-review-shell-extension-detail =
    Загружает { $server } в Проводник { $scope ->
        [everyone] для всех пользователей этого компьютера
       *[you] для вас
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shell-extension-detail-own-file =
    Загружает в Проводник устанавливаемый им файл { $scope ->
        [everyone] для всех пользователей этого компьютера
       *[you] для вас
    }
# The name of a shell extension: { $handler } is its name in the registry, and { $class } the kind of file or folder it
# is for, such as * or Directory.
core-policy-shell-extension-name = { $handler } для { $class }
# { $on } is the kind of file or folder whose menu it is, as the registry names it, such as * or
# Directory. { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-menu-command-detail =
    Добавляет команду в меню объектов { $on } { $scope ->
        [everyone] для всех пользователей этого компьютера
       *[you] для вас
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-default-verb-detail =
    Изменяет действие при открытии таких объектов { $scope ->
        [everyone] для всех пользователей этого компьютера
       *[you] для вас
    }
core-review-client-entry-detail = Можно выбрать как программу для этого действия. Сам выбор не меняется
core-review-com-permissions-title = Разрешения COM
core-review-com-permissions-detail = Определяет, кто может запускать его классы COM или от чьего имени они выполняются
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-link-type-detail =
    Ссылки этого типа открываются в приложении { $scope ->
        [everyone] для всех пользователей этого компьютера
       *[you] для вас
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-file-type-detail =
    Файлы этого типа открываются в приложении { $scope ->
        [everyone] для всех пользователей этого компьютера
       *[you] для вас
    }
# { $log } is the name of a Windows event log, such as Application.
core-review-event-log-source-detail = Может записывать в журнал { $log } сообщения из собственных файлов
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-font-detail =
    Загружается из папки установки при входе в систему { $scope ->
        [everyone] для всех пользователей этого компьютера
       *[you] для вас
    }
core-review-odbc-driver-detail = Загружается в программы, которые подключаются к базе данных через ODBC
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-odbc-source-detail =
    Подключение к базе данных { $scope ->
        [everyone] для всех пользователей этого компьютера
       *[you] для вас
    }
core-review-default-apps-detail = Можно выбрать в разделе «Приложения по умолчанию». Сам выбор не меняется
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-program-name-detail =
    Окно «Выполнить» и другие программы могут найти его по этому имени { $scope ->
        [everyone] для всех пользователей этого компьютера
       *[you] для вас
    }
# { $server } is the file name of what serves the classes.
core-review-com-title = COM в { $server }
core-review-com-title-own-file = COM в устанавливаемом им файле
# { $count } is how many registry values.
core-review-com-detail =
    { $count ->
        [one] { $count } параметр классов, которые могут запускать другие программы
        [few] { $count } параметра классов, которые могут запускать другие программы
        [many] { $count } параметров классов, которые могут запускать другие программы
       *[other] { $count } параметра классов, которые могут запускать другие программы
    }
core-review-com-support-title = Библиотеки типов, интерфейсы и AppID COM
# { $libraries }, { $interfaces }, and { $app_ids } are how many registry values there are for its type libraries, its
# interfaces, and its AppIDs.
core-review-com-support-detail = Параметры его библиотек типов ({ $libraries }), интерфейсов ({ $interfaces }) и AppID ({ $app_ids }), относящиеся к его классам
core-review-compatibility-title = Как запускаются его программы
# { $count } is how many registry values.
core-review-compatibility-detail =
    { $count ->
        [one] { $count } параметр совместимости для устанавливаемых им программ
        [few] { $count } параметра совместимости для устанавливаемых им программ
        [many] { $count } параметров совместимости для устанавливаемых им программ
       *[other] { $count } параметра совместимости для устанавливаемых им программ
    }

## The installer's notes on whose folders and settings these are, and what is not coming.

# { $account } is the name of a Windows account, such as alice.
core-review-account-signed-in = Данные пользователя и пользовательские параметры переносятся пользователю { $account }, который вошел в систему здесь, а не администратору, от имени которого выполняется установка.
# { $account } is the name of a Windows account, such as alice.
core-review-account-no-desktop = В этом сеансе никто не вошел в систему, поэтому данные пользователя и пользовательские параметры переносятся учетной записи { $account }, от имени которой выполняется установка.
# { $reason } is what Windows said, in its own words.
core-review-account-not-told = Не удалось определить, кто вошел в систему ({ $reason }), поэтому пользовательские параметры исключены, а данные пользователя помещаются в папку установки.
# { $reason } is what Windows said, in its own words.
core-review-signed-in-unknown = не удалось определить, кто вошел в систему: { $reason }
# Headings of what is left out, shared by the app's list and the installer's notes.
core-review-left-out-credentials = Не входят в этот установщик, так как похожи на учетные данные
core-review-left-out-settings = Параметры и история приложения в учетной записи, которые включаются только с параметром «Данные пользователя»
core-review-left-out-certificates = Приложение, по-видимому, использует сертификаты, которые были доверенными на его компьютере, и они не устанавливаются
core-review-left-out-fonts = Эти шрифты названы по имени приложения, но ничто не указывает, что они принадлежат ему, поэтому они не устанавливаются
# { $names } is a list of names, such as the names of saved passwords.
core-review-note-credentials = Не входят в этот установщик, так как похожи на учетные данные: { $names }. То, что из этого есть на этом компьютере, остается без изменений, и, возможно, вам придется ввести эти данные снова.
# { $names } is a list of names of license data.
core-review-note-licenses = Не входят в этот установщик, так как при его создании параметр «Данные лицензии и активации» был выключен: { $names }.
# { $names } is a list of places an account keeps settings, such as folders.
core-review-note-settings = Не входят в этот установщик, так как при его создании параметр «Данные пользователя» был выключен: параметры и история приложения в учетной записи ({ $names }). Здесь приложение запустится так же, как у нового пользователя.
# { $names } is a list of certificates, each as core-review-certificate.
core-review-note-certificates = Приложение, по-видимому, использует сертификаты, которые были доверенными на его компьютере, и они не устанавливаются: { $names }. Доверять ли им здесь, решать вам.
# { $names } is a list of font names.
core-review-note-fonts = Эти шрифты названы по имени приложения, но ничто не указывает, что они принадлежат ему, поэтому они не устанавливаются: { $names }.
# { $names } is a list of names.
core-review-note-left-out-by-choice = Исключено создателем этого установщика: { $names }.
# { $names } is a list of names.
core-review-note-added-by-choice = Добавлено создателем этого установщика, так как он счел это частью приложения: { $names }.
# { $name } is a certificate's subject, such as CN=Contoso, and { $store } the certificate store it was in, such as
# Root.
core-review-certificate = { $name } в хранилище { $store }
# { $names } is a list of names, and { $count } how many more there are.
core-review-names-and-more = { $names } и еще { $count }

## Why something in a package is left out. Each is the second half of a sentence of core-warning-review-left-out.

core-review-no-account = это относится к параметрам одной учетной записи, а определить, кто вошел в систему, не удалось
core-review-name-not-accepted = Windows не принимает такое имя
# { $error } is what Windows said, in its own words.
core-review-service-unreadable = не удалось прочитать текущие параметры службы: { $error }
core-review-service-dependency = она зависит от службы с недопустимым именем
# { $path } is the program the service runs.
core-review-service-runs-other = она запускает { $path }, а этот пакет такой файл не устанавливает
core-review-service-settings = ее параметры указывают на то, что этот пакет не устанавливает, или на чужой раздел реестра
core-review-driver-no-hash = нет хэша, по которому его можно проверить
core-review-license-for-the-machine = лицензия, сохраненная для всего компьютера, только указывается в отчете
core-review-license-starts-something = лицензии незачем что-либо запускать
core-review-task-not-captured = определение задачи не было сохранено
# { $error } is what was wrong with the task's XML, in English.
core-review-task-unreadable = не удалось прочитать определение задачи: { $error }
core-review-task-no-account = задача выполняется от имени того, кто ее устанавливает, а определить, кто вошел в систему здесь, не удалось
# { $path } is what the shortcut opens.
core-review-shortcut-opens-other = он открывает { $path }, а этот пакет такой файл не устанавливает
# { $service } is the service's name, and { $path } the program it runs now.
core-policy-service-exists = служба { $service } уже существует на этом компьютере и запускает { $path }, а это не результат предыдущей установки здесь. Если это более старая копия этого приложения, сначала удалите ее
core-policy-key-control-characters = имя раздела содержит управляющие символы
core-policy-value-control-characters = имя параметра содержит управляющие символы
core-policy-registry-link = это ссылка реестра
# { $reason } is why the key it stands for is left out, such as "no app install should change this key".
core-policy-virtual-store = это замена раздела реестра всего компьютера, а { $reason }
core-policy-registry-part = ни одна установка приложения не должна записывать в эту часть реестра
core-policy-key-refused = ни одна установка приложения не должна изменять этот раздел
core-policy-net-debugger = это назначило бы отладчик для всех программ .NET
core-policy-net-install-root = это изменило бы расположение, из которого все программы .NET загружают .NET
core-policy-class-redirects = это изменило бы способ открытия определенного вида программ или папок
core-policy-app-path-name = программа в нем указана не по имени файла
core-policy-app-path-not-installed = это зарегистрировало бы имя программы для файла, который этот пакет не устанавливает
core-policy-shared-default-verb = это изменило бы действие при открытии всех файлов или папок определенного типа
core-policy-command-runs-other = это добавляет команду, которая запускает то, что этот пакет не устанавливает
core-policy-autostart-not-command = это запускается автоматически, а его значение не является командой
core-policy-autostart-runs-other = это запускается автоматически и запускает то, что этот пакет не устанавливает
core-policy-environment-own-rules = переменные среды восстанавливаются по собственным правилам
core-policy-service-not-restored = это относится к службе, которую этот пакет не восстанавливает
core-policy-service-own-rules = собственные параметры службы восстанавливаются только по ее правилам
core-policy-service-dll = его ServiceDll не является файлом, который устанавливает этот пакет
core-policy-event-log-source-only = восстановить можно только собственные значения источника журнала событий
core-policy-security-log = ни одно приложение не записывает в журнал «Безопасность»
core-policy-event-log-messages = его сообщения брались бы из файла, который этот пакет не устанавливает
core-policy-type-library = его библиотека типов не является файлом, который устанавливает этот пакет
# { $name } is a program's file name, such as app.exe.
core-policy-application-opened = это изменило бы способ открытия { $name }, а этот пакет такую программу не устанавливает
core-policy-event-log-no-file = источник журнала событий должен брать сообщения из файла
core-policy-com-runs-other = это запускает то, что не является файлом, который устанавливает этот пакет
core-policy-com-redirect = это сделало бы этот класс тем, что этот пакет не устанавливает
core-policy-com-server = это запускает то, что не является ни файлом из этого пакета, ни частью Windows
core-policy-interface = это определяло бы способ вызова интерфейса с помощью того, что этот пакет не устанавливает
# { $name } is a program's file name, such as app.exe.
core-policy-app-id-started = это изменило бы способ запуска { $name }, а этот пакет такую программу не устанавливает
core-policy-app-id-surrogate = это размещалось бы в программе, которую этот пакет не устанавливает
core-policy-app-id-service = это выполнялось бы в службе, которую этот пакет не восстанавливает
core-policy-app-id-run-as = это выполнялось бы от имени собственной учетной записи
core-policy-app-id-link = это не указывает на AppID
core-policy-app-id-remote = это выполнялось бы не на этом компьютере
core-policy-odbc-own-driver = это собственный параметр ODBC, а не драйвер
core-policy-odbc-driver-not-installed = его драйвер не является файлом, который устанавливает этот пакет
core-policy-loads-other-file = это загрузило бы файл, который этот пакет не устанавливает
core-policy-odbc-own-source = это собственный параметр ODBC, а не источник данных
core-policy-odbc-source-missing = в нем указан источник данных, которого нет в этом пакете
core-policy-odbc-source-driver = его драйвер не является ни файлом из этого пакета, ни частью Windows
core-policy-shell-extension = это загрузило бы в Проводник расширение оболочки, которое не является файлом из этого пакета
core-policy-not-text = его содержимое не является текстом, хотя должно читаться как текст
core-policy-network-place = это указывает на расположение в сети
core-policy-font = это зарегистрировало бы шрифт, который этот пакет не устанавливает, или шрифт из папки Windows
core-policy-compatibility = это изменило бы способ запуска программы, которую этот пакет не устанавливает
core-policy-capabilities = в нем указано приложение, сведений о котором нет в этом пакете
core-policy-default-program = это изменило бы программу, которая открывает объекты по умолчанию
core-policy-url-protocol = это объявлено как тип ссылки, но не открывается ничем из того, что устанавливает этот пакет
core-policy-command-handler = это добавляет команду, которую обрабатывает то, что этот пакет не устанавливает
core-policy-installer-command = это запустило бы то, что может прочитать только установщик Windows
core-policy-creates-class = это создало бы класс, который этот пакет не устанавливает
core-policy-opens-links = это открывает ссылки с помощью того, что этот пакет не устанавливает
core-policy-opens-files = это открывает файлы с помощью того, что этот пакет не устанавливает
core-policy-firewall-name = его имя нельзя передать
core-policy-firewall-description = его описание нельзя передать
core-policy-firewall-every-program = оно применяется ко всем программам, а правило может относиться только к одной
# { $path } is the program the rule is for.
core-policy-firewall-program = оно предназначено для { $path }, а этот пакет такой файл не устанавливает
core-policy-firewall-direction = в нем не указано направление, которое можно задать
core-policy-firewall-action = оно переопределяло бы другие правила, чего не должно делать правило ни одного приложения
# { $setting } is a part of a firewall rule, as the scan of its PC described it.
core-policy-firewall-not-carried = в нем указано: { $setting }, а это нельзя задать в исходном виде или это ослабило бы действие других правил
core-policy-firewall-service = оно предназначено для службы, которую этот пакет не восстанавливает
core-policy-firewall-parts = часть его содержимого нельзя выразить в правиле брандмауэра
core-policy-folder-already-there = эта папка уже существовала вне папки установки, и ничто не указывает, что она принадлежит этому приложению, поэтому программы в нее не помещаются
core-policy-variable-name = ее имя не является допустимым именем переменной
core-policy-variable-relied-on = от этой переменной зависят Windows и другие программы
core-policy-task-name = Windows не принимает такое имя задачи
core-policy-task-folder = Windows не принимает такую папку задачи
core-policy-task-windows = она заменила бы одну из собственных задач Windows
core-policy-task-other-action = она выполняет не запуск программы, а другое действие
core-policy-task-no-program = в ней не указана программа для запуска
# { $path } is the program the task runs.
core-policy-task-runs-other = она запускает то, что этот пакет не устанавливает: { $path }
core-policy-task-exists-unreadable = задача с этим именем уже существует, и ее не удалось прочитать
core-policy-task-exists-other = задача с этим именем уже существует и не запускает приложение
# { $path } is the program the task already on this PC runs.
core-policy-task-exists-runs = задача с этим именем уже существует и запускает { $path }, а это не результат предыдущей установки здесь
core-policy-shortcut-not-lnk = это не ярлык .lnk
core-policy-shortcut-place = он находится не в меню «Пуск» и не на рабочем столе
core-policy-short-name = в его пути используется имя, которое нельзя проверить
core-policy-shortcut-no-folder = на этом компьютере нет подходящей для него папки
core-policy-data-startup = это попало бы в папку автозагрузки и запускалось бы при каждом входе в систему
core-policy-data-powershell = это попало бы туда, откуда PowerShell при запуске выполняет все, что найдет
core-policy-data-windows = это попало бы в папку, принадлежащую Windows
core-policy-license-outside = это находится вне папки установки и всех известных папок данных
core-rewrite-cdata = это содержит раздел CDATA
core-rewrite-too-much-to-look-through = в параметрах этого пакета слишком много данных, чтобы проверить все, на что они указывают

## Warnings a review gives, which the installer shows with what it sets up.

# { $kind } says what was left out, and is not shown itself. { $item } is its path or name, or a registry value such
# as HKLM\Software\App\Name. { $reason } is why, as the second half of the sentence, such as "it cannot be a whole
# drive".
core-warning-review-left-out =
    { $kind ->
        [file] Файл { $item } исключен: { $reason }.
        [folder] Папка { $item } исключена: { $reason }.
        [service] Служба { $item } исключена: { $reason }.
        [driver] Драйвер { $item } исключен: { $reason }.
        [firewall-rule] Правило брандмауэра { $item } исключено: { $reason }.
        [license-data] Данные лицензии в { $item } исключены: { $reason }.
        [license-value] Значение лицензии { $item } исключено: { $reason }.
        [scheduled-task] Задача планировщика { $item } исключена: { $reason }.
        [shortcut] Ярлык { $item } исключен: { $reason }.
        [environment-variable] Переменная среды { $item } исключена: { $reason }.
       *[value] Параметр { $item } исключен: { $reason }.
    }
core-warning-review-package-path-missing = У копируемого файла нет относительного пути в пакете.
# { $count } is the most empty folders a package makes.
core-warning-review-too-many-empty-folders = В этом пакете слишком много пустых папок (больше { $count }); создано будет только { $count }.
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Documents folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-documents-elsewhere =
    { $count ->
        [one] { $count } файл из папки «Документы» вместо этого помещается в папку _external в { $target }, так как папка «Документы» этого компьютера находится в { $folder }, а запись выполняется только в папки внутри вашего профиля. Чтобы использовать этот файл, самостоятельно переместите его в папку «Документы».
        [few] { $count } файла из папки «Документы» вместо этого помещаются в папку _external в { $target }, так как папка «Документы» этого компьютера находится в { $folder }, а запись выполняется только в папки внутри вашего профиля. Чтобы использовать эти файлы, самостоятельно переместите их в папку «Документы».
        [many] { $count } файлов из папки «Документы» вместо этого помещаются в папку _external в { $target }, так как папка «Документы» этого компьютера находится в { $folder }, а запись выполняется только в папки внутри вашего профиля. Чтобы использовать эти файлы, самостоятельно переместите их в папку «Документы».
       *[other] { $count } файла из папки «Документы» вместо этого помещаются в папку _external в { $target }, так как папка «Документы» этого компьютера находится в { $folder }, а запись выполняется только в папки внутри вашего профиля. Чтобы использовать эти файлы, самостоятельно переместите их в папку «Документы».
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-documents-unknown =
    { $count ->
        [one] { $count } файл из папки «Документы» вместо этого помещается в папку _external в { $target }, так как не удалось определить, где находится папка «Документы» этого компьютера. Чтобы использовать этот файл, самостоятельно переместите его в папку «Документы».
        [few] { $count } файла из папки «Документы» вместо этого помещаются в папку _external в { $target }, так как не удалось определить, где находится папка «Документы» этого компьютера. Чтобы использовать эти файлы, самостоятельно переместите их в папку «Документы».
        [many] { $count } файлов из папки «Документы» вместо этого помещаются в папку _external в { $target }, так как не удалось определить, где находится папка «Документы» этого компьютера. Чтобы использовать эти файлы, самостоятельно переместите их в папку «Документы».
       *[other] { $count } файла из папки «Документы» вместо этого помещаются в папку _external в { $target }, так как не удалось определить, где находится папка «Документы» этого компьютера. Чтобы использовать эти файлы, самостоятельно переместите их в папку «Документы».
    }
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Saved Games folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-elsewhere =
    { $count ->
        [one] { $count } файл из папки «Сохраненные игры» вместо этого помещается в папку _external в { $target }, так как папка «Сохраненные игры» этого компьютера находится в { $folder }, а запись выполняется только в папки внутри вашего профиля. Чтобы использовать этот файл, самостоятельно переместите его в папку «Сохраненные игры».
        [few] { $count } файла из папки «Сохраненные игры» вместо этого помещаются в папку _external в { $target }, так как папка «Сохраненные игры» этого компьютера находится в { $folder }, а запись выполняется только в папки внутри вашего профиля. Чтобы использовать эти файлы, самостоятельно переместите их в папку «Сохраненные игры».
        [many] { $count } файлов из папки «Сохраненные игры» вместо этого помещаются в папку _external в { $target }, так как папка «Сохраненные игры» этого компьютера находится в { $folder }, а запись выполняется только в папки внутри вашего профиля. Чтобы использовать эти файлы, самостоятельно переместите их в папку «Сохраненные игры».
       *[other] { $count } файла из папки «Сохраненные игры» вместо этого помещаются в папку _external в { $target }, так как папка «Сохраненные игры» этого компьютера находится в { $folder }, а запись выполняется только в папки внутри вашего профиля. Чтобы использовать эти файлы, самостоятельно переместите их в папку «Сохраненные игры».
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-unknown =
    { $count ->
        [one] { $count } файл из папки «Сохраненные игры» вместо этого помещается в папку _external в { $target }, так как не удалось определить, где находится папка «Сохраненные игры» этого компьютера. Чтобы использовать этот файл, самостоятельно переместите его в папку «Сохраненные игры».
        [few] { $count } файла из папки «Сохраненные игры» вместо этого помещаются в папку _external в { $target }, так как не удалось определить, где находится папка «Сохраненные игры» этого компьютера. Чтобы использовать эти файлы, самостоятельно переместите их в папку «Сохраненные игры».
        [many] { $count } файлов из папки «Сохраненные игры» вместо этого помещаются в папку _external в { $target }, так как не удалось определить, где находится папка «Сохраненные игры» этого компьютера. Чтобы использовать эти файлы, самостоятельно переместите их в папку «Сохраненные игры».
       *[other] { $count } файла из папки «Сохраненные игры» вместо этого помещаются в папку _external в { $target }, так как не удалось определить, где находится папка «Сохраненные игры» этого компьютера. Чтобы использовать эти файлы, самостоятельно переместите их в папку «Сохраненные игры».
    }

## Errors when a folder that only administrators may change cannot be made so, or checked.

# { $path } is a folder.
core-privilege-not-a-folder = { $path } — это ссылка или файл, а не папка
# { $path } is a folder. { $error } is what Windows said, in its own words.
core-privilege-not-made-here = папка { $path } создана не WinAppMigrator, и другие могут изменять ее содержимое. Администратору следует проверить и удалить ее: { $error }
# { $path } is a folder.
core-privilege-others-put-something-in = в папке { $path } есть то, что создано не WinAppMigrator, и другие могут это изменить. Администратору следует проверить и удалить это
# { $path } is a folder.
core-privilege-became-a-link-in-use = папка { $path } стала ссылкой во время использования
# { $path } is a folder.
core-privilege-became-a-link-looked-at = папка { $path } стала ссылкой во время проверки
# { $path } is a file. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-users-cannot-start = не удалось разрешить пользователям запускать { $path }: { $said }
core-privilege-elevation-unknown = не удалось определить, выполняется ли этот процесс с повышенными правами
# { $path } is a folder. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-cannot-restrict = не удалось ограничить доступ к { $path } только администраторами: { $said }
# { $path } is a folder. { $said } is the folder's permissions as Windows wrote them, and what else it printed.
core-privilege-not-restricted = не удалось ограничить доступ к { $path } только администраторами; возможно, этот диск не хранит разрешения: { $said }
# { $path } is a folder.
core-privilege-written-to-first = другой процесс записал данные в { $path } до того, как папку удалось защитить
core-privilege-no-program-data = Windows не сообщает, где находится ProgramData

## Checks of a package's files, and of what an install put in place. Each is said of one file, value, service, or the
## like, which the report names beside it.

# { $path } is the installer's file.
core-verify-no-package = в файле { $path } нет пакета
# { $path } is a folder.
core-verify-not-unpacked = пакет этого установщика не был распакован в { $path }
core-verify-schema-supported = Схема манифеста пакета поддерживается.
core-verify-packaged-file-missing = Файл из пакета отсутствует.
core-verify-no-package-path = У копируемого файла нет относительного пути в пакете.
core-verify-folder-there = Пустая папка на месте.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-unreadable = Не удалось прочитать для проверки: { $error }
core-verify-restored-file-missing = Восстановленный файл отсутствует.
# { $expected } and { $actual } are sizes in bytes.
core-verify-size-mismatch =
    Размер файла не совпадает. Ожидалось { $expected ->
        [one] { $expected } байт
        [few] { $expected } байта
        [many] { $expected } байт
       *[other] { $expected } байта
    }, обнаружено { $actual ->
        [one] { $actual } байт
        [few] { $actual } байта
        [many] { $actual } байт
       *[other] { $actual } байта
    }.
core-verify-no-hash = В манифесте не записан хэш этого файла, поэтому проверить его невозможно.
core-verify-hash-mismatch = Хэш SHA-256 не совпадает.
core-verify-file-matches = Размер и хэш файла совпадают с манифестом.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-payload-failed = Ошибка проверки содержимого лицензии: { $error }
core-verify-license-payload-missing = Содержимое лицензии в пакете отсутствует.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-failed = Ошибка проверки хранилища лицензий: { $error }
core-verify-license-store-file-missing = Файл хранилища лицензий в пакете отсутствует.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-path-invalid = Недопустимый путь к хранилищу лицензий в пакете: { $error }
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-prerequisite-failed = Ошибка проверки необходимого компонента: { $error }
core-verify-prerequisite-missing = Включенный в пакет необходимый компонент отсутствует.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-failed = Ошибка проверки образа драйвера: { $error }
core-verify-bundled-driver-missing = Включенный в пакет образ драйвера отсутствует.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-path-invalid = Недопустимый путь к драйверу в пакете: { $error }
core-verify-prerequisite-present = Необходимая среда выполнения есть на целевом компьютере.
core-verify-prerequisite-not-detected = Необходимая среда выполнения не обнаружена; возможно, требуется перезагрузка или завершение установки вручную (точный результат см. в журнале восстановления).
core-verify-prerequisite-not-probed = Наличие необходимого компонента нельзя проверить по реестру; установка выполнена без гарантии результата.
core-verify-license-file-missing = Восстановленный файл лицензии отсутствует.
core-verify-license-store-restored-missing = Восстановленный файл хранилища лицензий отсутствует.
core-verify-registry-key-there = Раздел реестра существует.
core-verify-registry-value-matches = Параметр реестра совпадает с сохраненными данными.
core-verify-registry-value-differs = Параметр реестра не совпадает с сохраненными данными.
core-verify-firewall-rule-there = Это правило есть в Брандмауэре Windows.
core-verify-variable-matches = Значение переменной среды совпадает с сохраненными данными.
core-verify-variable-differs = Значение переменной среды не совпадает с сохраненными данными.
# { $error } is what Windows said, in its own words.
core-verify-variable-unreadable = Переменная среды отсутствует или недоступна для чтения: { $error }
core-verify-service-runs-other = Служба существует, но не запускает восстановленный исполняемый файл с его аргументами.
core-verify-service-other-account = Служба существует, но выполняется не от имени ожидаемой учетной записи.
core-verify-service-matches = Служба существует и запускает восстановленный исполняемый файл с его аргументами от имени нужной учетной записи.
core-verify-service-missing = Служба отсутствует.
# { $error } is what Windows said, in its own words.
core-verify-service-unreadable = Не удалось прочитать службу: { $error }
core-verify-driver-other-image = Служба драйвера не указывает на восстановленный образ.
core-verify-driver-not-registered = Служба драйвера из пакета не зарегистрирована после восстановления.
# { $error } is what Windows said, in its own words.
core-verify-driver-unreadable = Не удалось прочитать службу драйвера: { $error }
core-verify-driver-image-missing = Образ драйвера отсутствует.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-unchecked = Не удалось проверить образ драйвера: { $error }
core-verify-task-matches = Задача планировщика существует и запускает восстановленные программы.
core-verify-task-runs-other = Задача планировщика существует, но не запускает восстановленные программы.
core-verify-task-missing = Задача планировщика отсутствует.
# { $error } is what Windows said, in its own words.
core-verify-task-unreadable = Ошибка запроса задачи планировщика: { $error }
core-verify-shortcut-matches = Ярлык существует, и его целевой путь совпадает с восстановленным.
core-verify-shortcut-opens-other = Ярлык существует, но его целевой путь не совпадает с восстановленным.
core-verify-shortcut-missing = Ярлык отсутствует.
# { $error } is what Windows said, in its own words.
core-verify-shortcut-unreadable = Ошибка запроса ярлыка: { $error }

## Warnings and errors a check gives, which the app and the installer show.

core-warning-verify-schema-unsupported = Схема манифеста пакета не поддерживается.
# { $error } is what went wrong, in Windows' or the program's own words.
core-warning-verify-package-file-failed = Ошибка проверки файла пакета: { $error }
core-warning-verify-folder-missing = Пустая папка отсутствует.
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-key-missing = Раздел реестра отсутствует или недоступен для чтения: { $error }
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-value-missing = Параметр реестра отсутствует или недоступен для чтения: { $error }
core-warning-verify-registry-kind-missing = Не указан тип параметра реестра.
core-warning-verify-firewall-rule-missing = В Брандмауэре Windows нет этого правила.
core-warning-verify-service-scope = При проверке служб считывается их конфигурация в HKLM, а сами службы не запускаются.
core-warning-verify-driver-manual = WinAppMigrator не распространяет один или несколько драйверов поставщика или встроенных драйверов Windows; переустановите их на целевом компьютере из подписанного пакета драйвера поставщика.
core-warning-verify-task-scope = При проверке задач планировщика проверяются только регистрация и ссылка на команду.

## The verification report written beside a package, verification_report.md. Its headings follow a "#".

core-verify-report-title = Отчет о проверке WinAppMigrator
# { $time } is when, as a date and time in UTC, such as 2026-11-15T10:00:00Z.
core-verify-report-verified = Проверено (UTC): { $time }
# { $valid } is true or false, which are not translated.
core-verify-report-package-valid = Пакет верен: { $valid }
# { $valid } is true or false, which are not translated.
core-verify-report-restore-valid = Восстановление верно: { $valid }
core-verify-report-restore-not-run = Восстановление верно: не выполнялось
core-verify-report-items = Элементы
# { $message } is what a check found, and { $source } the file or setting it looked at.
core-verify-report-item-from = { $message } Источник: { $source }
core-verify-report-warnings = Предупреждения
core-verify-report-errors = Ошибки
core-verify-report-none = Нет.
