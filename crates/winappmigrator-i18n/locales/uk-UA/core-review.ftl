# Words the core says about what an install may do: why an install location or something in a package is refused,
# the installer's list of what an app sets up beyond its files, the checks of a package and of an install, and whether
# there is room for either. Message names start with core-.
#
# Many messages here are a reason, the second half of a sentence that starts "... was left out:" or "that install
# location cannot be used:", so they start with a small letter and have no full stop.


## Why an install location cannot be used. Shown in red under the Install Location field.

core-review-location-state-root = тут інсталяції зберігають свої записи
core-review-location-has-files = у цій папці вже є файли, а запису про інсталяцію тут немає, тому використайте порожню папку
core-policy-location-not-full-path = це має бути повний шлях
core-policy-location-parent-step = шлях не може містити «..»
core-policy-location-not-plain = це має бути звичайний шлях на диску цього ПК
core-policy-location-whole-drive = це не може бути весь диск
core-policy-location-windows = це не може бути папка Windows
core-policy-location-runs-by-itself = це папка, яка належить Windows або вміст якої запускається чи завантажується автоматично
core-policy-location-shared = у цій папці поруч зберігаються інші речі, тому програмі потрібна окрема папка всередині неї

## Whether an app can run on this PC at all, said before anything is installed.

# { $reason } is why, as the second half of a sentence, such as "App.exe is built for Arm64, and this PC is x64, so
# Windows would not start it".
core-review-cannot-run-here = ця програма не може працювати на цьому ПК, тому нічого не змінено: { $reason }
core-policy-architecture-x86 = 32-розрядна x86
core-policy-architecture-x64 = x64
core-policy-architecture-arm64 = Arm64
core-policy-architecture-arm32 = 32-розрядна Arm
core-policy-architecture-unknown = невідома архітектура
# { $code } is the number Windows gives a kind of processor, such as 0x01c4.
core-policy-machine-type = тип машини { $code }
# { $names } is a list of program names, such as "a.exe, b.exe, c.exe", and { $count } how many more there are.
core-policy-names-and-more = { $names } і ще { $count }
core-policy-fit-programs-not-said = Цей інсталятор не вказує, що потрібно його виконуваним файлам, тому не перевірено, чи можуть вони працювати на цьому ПК.
core-policy-fit-architecture-not-told = Не вдалося визначити архітектуру цього ПК, тому не перевірено, чи можуть на ньому працювати виконувані файли цієї програми.
core-policy-fit-main-not-said = Цей інсталятор не вказує, що потрібно його основному виконуваному файлу, тому наведені тут відомості ґрунтуються на решті його вмісту.
# { $driver } is a driver's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-driver = архітектура її драйвера { $driver } — { $built_for }, а цього ПК — { $here }. Драйвер має точно відповідати Windows, тому ця програма не може тут працювати
# { $program } is a program's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-program = архітектура { $program } — { $built_for }, а цього ПК — { $here }, тому Windows не запустить цей файл
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-none-start = жоден із її виконуваних файлів не може запуститися на цьому ПК (архітектура { $here }): { $programs }
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-some-other-architecture = Деякі виконувані файли цієї програми створено для іншої архітектури, і вони не запустяться на цьому ПК (архітектура { $here }): { $programs }. Решта програми може запуститися, тому це може не мати значення.
# { $here } is a kind of processor, such as "Arm64". { $programs } is a list of program names.
core-policy-fit-emulated = Архітектура цього ПК — { $here }, тому Windows запускатиме ці файли через емулятор: { $programs }. Більшість програм так працюють нормально, але деякі — ні.
# { $program } is a program's file name. { $needs } and { $runs } are versions of Windows, such as 10.0 and 6.1.
core-policy-fit-needs-newer = { $program } потребує Windows { $needs } або новішої версії, а на цьому ПК працює версія { $runs }, тому Windows не запустить цей файл
# { $programs } is a list of program names.
core-policy-fit-some-need-newer = Деякі виконувані файли цієї програми потребують новішої версії Windows, ніж ця, і тут не запустяться: { $programs }.
# { $from_build } and { $this_build } are Windows build numbers, such as 22631, written as Windows writes them.
core-policy-fit-older-build = Ця програма працювала в збірці Windows { $from_build }, а на цьому ПК працює збірка { $this_build }. Її виконувані файли вказують, що можуть запускатися в цій версії Windows, тож програма може працювати, але того, на що вона покладалася там, тут може не бути.
core-policy-fit-build-not-told = Не вдалося визначити збірку Windows, тому її не порівняно зі збіркою, у якій працювала ця програма.

## Room on a drive for a build or an install.

# Each of these ends the sentence core-space-shortfall, after "free for".
core-space-for-unpacking = розпакування пакета
core-space-for-install-record = запису інсталяції
core-space-for-app-files = файлів програми
core-space-for-what-is-replaced = збереження того, що буде замінено
core-space-for-build-work = робочих файлів створення
core-space-for-installer = інсталятора
core-space-for-package = пакета
core-space-for-unpacking-and-installing = розпакування та інсталяції пакета
# What goes between two of those, as in "unpacking the package and the app's files". Keep the quotes: they keep the
# spaces.
core-space-purposes-joiner = { " і " }
# { $drive } is a drive, such as C:, or a folder. { $needed } and { $free } are sizes, such as 1.2 GB. { $purposes } is
# what the room is for, such as "unpacking the package and the app's files".
core-space-shortfall = На { $drive } потрібно до { $needed } вільного місця для { $purposes }, а вільно { $free }
# { $drive } is a drive, such as C:, or a folder. { $needed } is a size, such as 1.2 GB.
core-space-not-told = Не вдалося дізнатися, скільки вільного місця на { $drive }, тому не перевірено, чи вміститься потрібний обсяг { $needed }
# One drive in a list of where room is needed. { $needed } and { $free } are sizes, and { $drive } a drive, such as C:.
core-space-summary = до { $needed } на { $drive } (вільно { $free })
# { $needed } is a size, and { $drive } a drive, such as C:.
core-space-summary-not-told = до { $needed } на { $drive } (не вдалося визначити, скільки вільно)
# What goes between two sentences of core-space-shortfall inside core-review-no-room. Keep the quotes: they keep the
# space.
core-review-sentence-joiner = { ". " }
# { $shortfalls } is one or more sentences of core-space-shortfall.
core-review-no-room = недостатньо місця для цього, тому нічого не змінено. { $shortfalls }. Звільніть там місце або виберіть розташування інсталяції на іншому диску.

## The installer's list of what an app sets up beyond its files: what each kind is called.

core-review-kind-service = Служба
core-review-kind-driver = Драйвер
core-review-kind-scheduled-task = Заплановане завдання
core-review-kind-autostart = Автоматичний запуск
core-review-kind-shortcut = Ярлик
core-review-kind-path = PATH
core-review-kind-environment-variable = Змінна середовища
core-review-kind-shell-extension = Розширення оболонки
core-review-kind-menu-command = Команда меню
core-review-kind-link-type = Тип посилання
core-review-kind-file-type = Тип файлу
core-review-kind-com = COM
core-review-kind-firewall-rule = Правило брандмауера
core-review-kind-event-log-source = Джерело журналу подій
core-review-kind-font = Шрифт
core-review-kind-odbc-driver = Драйвер ODBC
core-review-kind-odbc-source = Джерело даних ODBC
core-review-kind-compatibility-setting = Параметр сумісності
core-review-kind-default-apps = Програми за замовчуванням
core-review-kind-program-name = Ім'я програми
core-review-kind-other-settings = Інші параметри

## The installer's list of what an app sets up: what each line says of it.

# { $file } is a program's file name. { $account } is the Windows account it runs as, such as LocalSystem. { $starts }
# says when it starts, and is not shown itself; for "other", { $start } is one of the core-review-start- words below.
core-review-service-detail =
    { $starts ->
        [automatic] Виконує { $file } від імені { $account }, запускається зараз і щоразу під час запуску цього ПК
        [delayed] Виконує { $file } від імені { $account }, запускається зараз і невдовзі після кожного запуску цього ПК
        [manual] Виконує { $file } від імені { $account }, запускається лише на запит
        [disabled] Виконує { $file } від імені { $account }, вимкнуто
        [other] Виконує { $file } від імені { $account }, { $start }
       *[unknown] Виконує { $file } від імені { $account }
    }
# { $account } is a Windows account, such as LocalService.
core-policy-account-custom = { $account } (власний обліковий запис не відновлюється)
# { $file } is a driver's file name.
core-review-driver-detail = Завантажує { $file } безпосередньо в Windows
# { $file } is a driver's file name. { $start } is when Windows starts it: one of the core-review-start- words below.
core-review-driver-detail-starts = Завантажує { $file } безпосередньо в Windows, { $start }
# When a service or driver starts, ending a sentence such as "Loads disk.sys into Windows itself, starts …".
core-review-start-boot = запускається разом із Windows, раніше за все інше
core-review-start-system = запускається разом із Windows, під час її завантаження
core-review-start-automatic = запускається щоразу під час запуску цього ПК
core-review-start-delayed = запускається невдовзі після кожного запуску цього ПК
core-review-start-manual = запускається лише на запит
core-review-start-disabled = ніколи не запускається (вимкнуто)
core-review-start-unknown = запускається у спосіб, який Windows не вказує
# { $account } is the account a task runs as: an account's name, or core-review-task-runs-as-you, or
# core-review-task-runs-as-creator.
core-review-task-detail = Обліковий запис: { $account }
# { $account } is the account a task runs as, as for core-review-task-detail.
core-review-task-detail-highest = Обліковий запис: { $account }, з найвищими можливими правами
core-review-task-runs-as-you = ви (той, хто ввійшов у систему)
core-review-task-runs-as-creator = користувач, який створив завдання
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-autostart-detail =
    Запускається { $scope ->
        [everyone] для кожного, хто входить у систему
       *[you] під час вашого входу в систему
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shortcut-startup-detail =
    Ярлик у папці «Автозавантаження» { $scope ->
        [everyone] для всіх користувачів цього ПК
       *[you] для вас
    }
# { $file } is the file name of what the shortcut opens. { $scope } is whom it is set up for: everyone on this PC,
# or you. It is not shown itself.
core-review-shortcut-detail =
    Відкриває { $file }, { $scope ->
        [everyone] для всіх користувачів цього ПК
       *[you] для вас
    }
# { $name } is an environment variable's name, such as PATH.
core-review-path-title = Додає до { $name }
core-review-path-install-folder = Папка інсталяції
# { $path } is a folder inside the install folder, such as bin\tools.
core-review-path-below-install-folder = Папка інсталяції\{ $path }
# { $value } is the variable's value, as it will be set.
core-review-variable-detail = Значення: { $value }
# { $count } is how many registry values.
core-review-other-settings-title =
    { $count ->
        [one] { $count } параметр поза власними розділами програми
        [few] { $count } параметри поза власними розділами програми
        [many] { $count } параметрів поза власними розділами програми
       *[other] { $count } параметрів поза власними розділами програми
    }
# { $keys } is a list of registry keys, such as HKCU\Software\Vendor.
core-review-other-settings-detail = У { $keys }
# { $keys } is a list of registry keys, and { $count } how many more there are.
core-review-other-settings-detail-more = У { $keys } і ще { $count }
# { $way } says whether the rule allows or blocks, and incoming or outgoing connections; it is not shown itself.
# { $ports } is a list of port numbers, and { $port_count } how many. { $program } is the program's file name.
# { $networks } is a list of the kinds of network Windows names in English (domain, private, public), and
# { $network_count } how many.
# The kinds of network a firewall rule is for, as Windows Defender Firewall names its profiles in your language. They
# fill { $networks } in core-review-firewall-detail, joined into a list.
core-review-network-domain = доменна
core-review-network-private = приватна
core-review-network-public = загальнодоступна
core-review-firewall-detail =
    { $way ->
        [allow-in] Дозволяє вхідні
        [allow-out] Дозволяє вихідні
        [block-in] Блокує вхідні
       *[block-out] Блокує вихідні
    } { $port_count ->
        [0] підключення
        [one] підключення на порту { $ports }
        [few] підключення на портах { $ports }
        [many] підключення на портах { $ports }
       *[other] підключення на портах { $ports }
    } для { $program }, { $network_count ->
        [0] в усіх мережах
        [one] тип мережі: { $networks }
        [few] типи мереж: { $networks }
        [many] типи мереж: { $networks }
       *[other] типи мереж: { $networks }
    }
# { $server } is the file name of what Explorer loads. { $scope } is whom it is set up for: everyone on this PC, or
# you. It is not shown itself.
core-review-shell-extension-detail =
    Завантажує { $server } у Провідник файлів { $scope ->
        [everyone] для всіх користувачів цього ПК
       *[you] для вас
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shell-extension-detail-own-file =
    Завантажує у Провідник файлів файл, який інсталює, { $scope ->
        [everyone] для всіх користувачів цього ПК
       *[you] для вас
    }
# The name of a shell extension: { $handler } is its name in the registry, and { $class } the kind of file or folder it
# is for, such as * or Directory.
core-policy-shell-extension-name = { $handler } для { $class }
# { $on } is the kind of file or folder whose menu it is, as the registry names it, such as * or
# Directory. { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-menu-command-detail =
    Додає команду до меню { $on }, { $scope ->
        [everyone] для всіх користувачів цього ПК
       *[you] для вас
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-default-verb-detail =
    Змінює дію під час відкриття таких об'єктів, { $scope ->
        [everyone] для всіх користувачів цього ПК
       *[you] для вас
    }
core-review-client-entry-detail = Можна вибрати як програму для цього. Нічого не змінюється
core-review-com-permissions-title = Дозволи COM
core-review-com-permissions-detail = Визначає, хто може запускати її класи COM або від чийого імені вони виконуються
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-link-type-detail =
    Посилання цього типу відкриваються в програмі, { $scope ->
        [everyone] для всіх користувачів цього ПК
       *[you] для вас
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-file-type-detail =
    Файли цього типу відкриваються в програмі, { $scope ->
        [everyone] для всіх користувачів цього ПК
       *[you] для вас
    }
# { $log } is the name of a Windows event log, such as Application.
core-review-event-log-source-detail = Може записувати в журнал «{ $log }» повідомлення з власних файлів
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-font-detail =
    Завантажується з папки інсталяції під час входу в систему, { $scope ->
        [everyone] для всіх користувачів цього ПК
       *[you] для вас
    }
core-review-odbc-driver-detail = Завантажується в програми, які використовують ODBC для доступу до бази даних
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-odbc-source-detail =
    Підключення до бази даних, { $scope ->
        [everyone] для всіх користувачів цього ПК
       *[you] для вас
    }
core-review-default-apps-detail = Можна вибрати в розділі «Програми за замовчуванням». Нічого не змінюється
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-program-name-detail =
    Вікно «Виконати» та інші програми можуть знайти її за цим іменем, { $scope ->
        [everyone] для всіх користувачів цього ПК
       *[you] для вас
    }
# { $server } is the file name of what serves the classes.
core-review-com-title = COM у { $server }
core-review-com-title-own-file = COM у файлі, який вона інсталює
# { $count } is how many registry values.
core-review-com-detail =
    { $count ->
        [one] { $count } параметр класів, які можуть запускати інші програми
        [few] { $count } параметри класів, які можуть запускати інші програми
        [many] { $count } параметрів класів, які можуть запускати інші програми
       *[other] { $count } параметрів класів, які можуть запускати інші програми
    }
core-review-com-support-title = Бібліотеки типів, інтерфейси та AppID для COM
# { $libraries }, { $interfaces }, and { $app_ids } are how many registry values there are for its type libraries, its
# interfaces, and its AppIDs.
core-review-com-support-detail = Параметри її бібліотек типів ({ $libraries }), інтерфейсів ({ $interfaces }) і AppID ({ $app_ids }), пов'язані з її класами
core-review-compatibility-title = Як запускаються її виконувані файли
# { $count } is how many registry values.
core-review-compatibility-detail =
    { $count ->
        [one] { $count } параметр сумісності для виконуваних файлів, які вона інсталює
        [few] { $count } параметри сумісності для виконуваних файлів, які вона інсталює
        [many] { $count } параметрів сумісності для виконуваних файлів, які вона інсталює
       *[other] { $count } параметрів сумісності для виконуваних файлів, які вона інсталює
    }

## The installer's notes on whose folders and settings these are, and what is not coming.

# { $account } is the name of a Windows account, such as alice.
core-review-account-signed-in = Дані користувача й параметри окремих користувачів потрапляють до облікового запису { $account }, який увійшов тут у систему, а не до адміністратора, від імені якого це запущено.
# { $account } is the name of a Windows account, such as alice.
core-review-account-no-desktop = У цьому сеансі ніхто не ввійшов у систему, тому дані користувача й параметри окремих користувачів потрапляють до { $account } — облікового запису, від імені якого це запущено.
# { $reason } is what Windows said, in its own words.
core-review-account-not-told = Не вдалося визначити, хто ввійшов у систему ({ $reason }), тому параметри окремих користувачів не включено, а дані користувача розміщуються в розташуванні інсталяції.
# { $reason } is what Windows said, in its own words.
core-review-signed-in-unknown = не вдалося визначити, хто ввійшов у систему: { $reason }
# Headings of what is left out, shared by the app's list and the installer's notes.
core-review-left-out-credentials = Не включено до цього інсталятора, оскільки це схоже на облікові дані
core-review-left-out-settings = Параметри та історія програми, які зберігає обліковий запис і які включаються лише з параметром «Дані користувача»
core-review-left-out-certificates = Програма, імовірно, покладається на сертифікати, яким довіряли на її ПК, але їх не інстальовано
core-review-left-out-fonts = Ці шрифти названо за назвою програми, але ніщо не вказує, що вони належать їй, тому їх не інстальовано
# { $names } is a list of names, such as the names of saved passwords.
core-review-note-credentials = Не включено до цього інсталятора, оскільки це схоже на облікові дані: { $names }. Те, що з цього є на цьому ПК, залишається без змін, і, можливо, вам доведеться ввести ці дані знову.
# { $names } is a list of names of license data.
core-review-note-licenses = Не включено до цього інсталятора, оскільки під час його створення параметр «Дані ліцензії та активації» був вимкнутий: { $names }.
# { $names } is a list of places an account keeps settings, such as folders.
core-review-note-settings = Не включено до цього інсталятора, оскільки під час його створення параметр «Дані користувача» був вимкнутий: параметри та історія програми, які зберігає обліковий запис, у { $names }. Програма запуститься тут так, ніби ви використовуєте її вперше.
# { $names } is a list of certificates, each as core-review-certificate.
core-review-note-certificates = Програма, імовірно, покладається на сертифікати, яким довіряли на її ПК, але їх не інстальовано: { $names }. Чи довіряти їм тут, вирішувати вам.
# { $names } is a list of font names.
core-review-note-fonts = Ці шрифти названо за назвою програми, але ніщо не вказує, що вони належать їй, тому їх не інстальовано: { $names }.
# { $names } is a list of names.
core-review-note-left-out-by-choice = Не включено тим, хто створив цей інсталятор: { $names }.
# { $names } is a list of names.
core-review-note-added-by-choice = Додано тим, хто створив цей інсталятор, оскільки він вважав це частиною програми: { $names }.
# { $name } is a certificate's subject, such as CN=Contoso, and { $store } the certificate store it was in, such as
# Root.
core-review-certificate = { $name } у сховищі { $store }
# { $names } is a list of names, and { $count } how many more there are.
core-review-names-and-more = { $names } і ще { $count }

## Why something in a package is left out. Each is the second half of a sentence of core-warning-review-left-out.

core-review-no-account = це належить до параметрів одного облікового запису, а не вдалося визначити, хто ввійшов у систему
core-review-name-not-accepted = Windows не приймає таке ім'я
# { $error } is what Windows said, in its own words.
core-review-service-unreadable = не вдалося прочитати її поточні параметри: { $error }
core-review-service-dependency = вона залежить від служби з неприпустимим іменем
# { $path } is the program the service runs.
core-review-service-runs-other = вона запускає { $path }, який цей пакет не інсталює
core-review-service-settings = її параметри вказують на те, що цей пакет не інсталює, або на розділ, який їй не належить
core-review-driver-no-hash = для нього немає хешу, за яким його можна перевірити
core-review-license-for-the-machine = про ліцензію, збережену для всього комп'ютера, лише повідомляється
core-review-license-starts-something = ліцензія не має причин щось запускати
core-review-task-not-captured = його визначення не отримано
# { $error } is what was wrong with the task's XML, in English.
core-review-task-unreadable = не вдалося прочитати його визначення: { $error }
core-review-task-no-account = воно виконується від імені того, хто його інсталює, а не вдалося визначити, хто тут увійшов у систему
# { $path } is what the shortcut opens.
core-review-shortcut-opens-other = він відкриває { $path }, який цей пакет не інсталює
# { $service } is the service's name, and { $path } the program it runs now.
core-policy-service-exists = { $service } уже існує на цьому ПК і запускає { $path }, який не належить до попередньої інсталяції тут. Якщо це старіша копія цієї програми, спершу видаліть її
core-policy-key-control-characters = ім'я розділу містить керувальні символи
core-policy-value-control-characters = ім'я параметра містить керувальні символи
core-policy-registry-link = це посилання реєстру
# { $reason } is why the key it stands for is left out, such as "no app install should change this key".
core-policy-virtual-store = це замінник розділу комп'ютера, а { $reason }
core-policy-registry-part = жодна інсталяція програми не має записувати в цю частину реєстру
core-policy-key-refused = жодна інсталяція програми не має змінювати цей розділ
core-policy-net-debugger = це призначило б налагоджувач для кожної програми .NET
core-policy-net-install-root = це змінило б, звідки кожна програма .NET завантажує .NET
core-policy-class-redirects = це змінило б, як відкривається певний тип програм або папок
core-policy-app-path-name = це називає програму не за іменем файлу
core-policy-app-path-not-installed = це зареєструвало б ім'я програми для файлу, який цей пакет не інсталює
core-policy-shared-default-verb = це змінило б дію під час відкриття всіх файлів або папок певного типу
core-policy-command-runs-other = це додає команду, яка запускає те, що цей пакет не інсталює
core-policy-autostart-not-command = це запускається автоматично, а значення не є командою
core-policy-autostart-runs-other = це запускається автоматично й запускає те, що цей пакет не інсталює
core-policy-environment-own-rules = змінні середовища відновлюються за власними правилами
core-policy-service-not-restored = це належить службі, яку цей пакет не відновлює
core-policy-service-own-rules = власні параметри служби відновлюються лише за її правилами
core-policy-service-dll = її ServiceDll не є файлом, який інсталює цей пакет
core-policy-event-log-source-only = відновити можна лише власні параметри джерела журналу подій
core-policy-security-log = жодна програма не записує в журнал «Безпека»
core-policy-event-log-messages = його повідомлення надходили б із файлу, який цей пакет не інсталює
core-policy-type-library = його бібліотека типів не є файлом, який інсталює цей пакет
# { $name } is a program's file name, such as app.exe.
core-policy-application-opened = це змінило б, як відкривається { $name }, який цей пакет не інсталює
core-policy-event-log-no-file = джерело журналу подій має брати повідомлення з файлу
core-policy-com-runs-other = це запускає щось, що не є файлом, який інсталює цей пакет
core-policy-com-redirect = це перетворило б цей клас на те, що цей пакет не інсталює
core-policy-com-server = це запускає щось, що не є ні файлом, який інсталює цей пакет, ні частиною Windows
core-policy-interface = це визначало б, як виконуються виклики інтерфейсу, за допомогою того, що цей пакет не інсталює
# { $name } is a program's file name, such as app.exe.
core-policy-app-id-started = це змінило б, як запускається { $name }, який цей пакет не інсталює
core-policy-app-id-surrogate = це розміщувалося б у програмі, яку цей пакет не інсталює
core-policy-app-id-service = це виконувалося б у службі, яку цей пакет не відновлює
core-policy-app-id-run-as = це виконувалося б від імені окремого облікового запису
core-policy-app-id-link = це не вказує на AppID
core-policy-app-id-remote = це виконувалося б не на цьому ПК
core-policy-odbc-own-driver = це власний параметр ODBC, а не драйвер
core-policy-odbc-driver-not-installed = його драйвер не є файлом, який інсталює цей пакет
core-policy-loads-other-file = це завантажило б файл, який цей пакет не інсталює
core-policy-odbc-own-source = це власний параметр ODBC, а не джерело даних
core-policy-odbc-source-missing = тут указано джерело даних, якого немає в цьому пакеті
core-policy-odbc-source-driver = його драйвер не є ні файлом, який інсталює цей пакет, ні частиною Windows
core-policy-shell-extension = це завантажило б у Провідник файлів розширення оболонки, яке не є файлом, що інсталює цей пакет
core-policy-not-text = вміст не є текстом, як очікувалося
core-policy-network-place = це вказує на розташування в мережі
core-policy-font = це зареєструвало б шрифт, який цей пакет не інсталює, або шрифт у папці Windows
core-policy-compatibility = це змінило б, як працює програма, яку цей пакет не інсталює
core-policy-capabilities = тут указано програму, відомостей про яку немає в цьому пакеті
core-policy-default-program = це змінило б, яка програма відкриває об'єкти за замовчуванням
core-policy-url-protocol = це позначено як тип посилання, але воно не відкривається нічим, що інсталює цей пакет
core-policy-command-handler = це додає команду, яку обробляє те, що цей пакет не інсталює
core-policy-installer-command = це запускало б те, що може прочитати лише Windows Installer
core-policy-creates-class = це створило б клас, який цей пакет не інсталює
core-policy-opens-links = це відкриває посилання за допомогою того, що цей пакет не інсталює
core-policy-opens-files = це відкриває файли за допомогою того, що цей пакет не інсталює
core-policy-firewall-name = його назву не можна передати
core-policy-firewall-description = його опис не можна передати
core-policy-firewall-every-program = воно застосовується до всіх програм, а правило може стосуватися лише однієї
# { $path } is the program the rule is for.
core-policy-firewall-program = воно призначене для { $path }, який цей пакет не інсталює
core-policy-firewall-direction = у ньому не вказано напрямок, який можна налаштувати
core-policy-firewall-action = воно мало б пріоритет над іншими правилами, а правило жодної програми не повинне цього робити
# { $setting } is a part of a firewall rule, as the scan of its PC described it.
core-policy-firewall-not-carried = у ньому вказано { $setting }, а це не можна налаштувати так само або це послабило б дію інших правил
core-policy-firewall-service = воно призначене для служби, яку цей пакет не відновлює
core-policy-firewall-parts = частина його вмісту не відповідає тому, що може містити правило брандмауера
core-policy-folder-already-there = ця папка вже існувала поза розташуванням інсталяції, і ніщо не вказує, що вона належить цій програмі, тому жоден виконуваний файл у неї не розміщується
core-policy-variable-name = її ім'я не є допустимим іменем змінної
core-policy-variable-relied-on = від цієї змінної залежать Windows та інші програми
core-policy-task-name = Windows не приймає таке ім'я завдання
core-policy-task-folder = Windows не приймає таку папку завдання
core-policy-task-windows = воно замінило б одне з власних завдань Windows
core-policy-task-other-action = воно робить щось інше, а не запускає програму
core-policy-task-no-program = у ньому не вказано програму для запуску
# { $path } is the program the task runs.
core-policy-task-runs-other = воно запускає те, що цей пакет не інсталює: { $path }
core-policy-task-exists-unreadable = завдання з такою назвою вже існує, і його не вдалося прочитати
core-policy-task-exists-other = завдання з такою назвою вже існує й не запускає цю програму
# { $path } is the program the task already on this PC runs.
core-policy-task-exists-runs = завдання з такою назвою вже існує й запускає { $path }, який не належить до попередньої інсталяції тут
core-policy-shortcut-not-lnk = це не ярлик .lnk
core-policy-shortcut-place = він розміщений не в меню «Пуск» і не на робочому столі
core-policy-short-name = у шляху використано ім'я, яке неможливо перевірити
core-policy-shortcut-no-folder = на цьому ПК немає такої папки для нього
core-policy-data-startup = це потрапило б у папку «Автозавантаження» й запускалося б під час кожного входу в систему
core-policy-data-powershell = це потрапило б туди, звідки PowerShell під час запуску виконує все, що знаходить
core-policy-data-windows = це потрапило б у папку, яка належить Windows
core-policy-license-outside = це розміщено поза розташуванням інсталяції та всіма відомими папками даних
core-rewrite-cdata = це містить розділ CDATA
core-rewrite-too-much-to-look-through = цей пакет містить у своїх параметрах забагато даних, щоб перевірити, на що вони вказують

## Warnings a review gives, which the installer shows with what it sets up.

# { $kind } says what was left out, and is not shown itself. { $item } is its path or name, or a registry value such
# as HKLM\Software\App\Name. { $reason } is why, as the second half of the sentence, such as "it cannot be a whole
# drive".
core-warning-review-left-out =
    { $kind ->
        [file] Файл { $item } не включено: { $reason }.
        [folder] Папку { $item } не включено: { $reason }.
        [service] Службу { $item } не включено: { $reason }.
        [driver] Драйвер { $item } не включено: { $reason }.
        [firewall-rule] Правило брандмауера { $item } не включено: { $reason }.
        [license-data] Дані ліцензії в { $item } не включено: { $reason }.
        [license-value] Параметр ліцензії { $item } не включено: { $reason }.
        [scheduled-task] Заплановане завдання { $item } не включено: { $reason }.
        [shortcut] Ярлик { $item } не включено: { $reason }.
        [environment-variable] Змінну середовища { $item } не включено: { $reason }.
       *[value] { $item } не включено: { $reason }.
    }
core-warning-review-package-path-missing = Файл для копіювання не має відносного шляху в пакеті.
# { $count } is the most empty folders a package makes.
core-warning-review-too-many-empty-folders = Цей пакет містить понад { $count } порожніх папок, і буде створено лише стільки.
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Documents folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-documents-elsewhere =
    { $count ->
        [one] { $count } файл із папки «Документи» натомість розміщується в { $target }, у папці _external, оскільки папка «Документи» на цьому ПК розташована в { $folder }, а запис виконується лише в папку всередині вашого профілю. Щоб скористатися цим файлом, самостійно перемістіть його в папку «Документи».
        [few] { $count } файли з папки «Документи» натомість розміщуються в { $target }, у папці _external, оскільки папка «Документи» на цьому ПК розташована в { $folder }, а запис виконується лише в папку всередині вашого профілю. Щоб скористатися цими файлами, самостійно перемістіть їх у папку «Документи».
        [many] { $count } файлів із папки «Документи» натомість розміщуються в { $target }, у папці _external, оскільки папка «Документи» на цьому ПК розташована в { $folder }, а запис виконується лише в папку всередині вашого профілю. Щоб скористатися цими файлами, самостійно перемістіть їх у папку «Документи».
       *[other] { $count } файлів із папки «Документи» натомість розміщуються в { $target }, у папці _external, оскільки папка «Документи» на цьому ПК розташована в { $folder }, а запис виконується лише в папку всередині вашого профілю. Щоб скористатися цими файлами, самостійно перемістіть їх у папку «Документи».
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-documents-unknown =
    { $count ->
        [one] { $count } файл із папки «Документи» натомість розміщується в { $target }, у папці _external, оскільки не вдалося визначити, де розташована папка «Документи» на цьому ПК. Щоб скористатися цим файлом, самостійно перемістіть його в папку «Документи».
        [few] { $count } файли з папки «Документи» натомість розміщуються в { $target }, у папці _external, оскільки не вдалося визначити, де розташована папка «Документи» на цьому ПК. Щоб скористатися цими файлами, самостійно перемістіть їх у папку «Документи».
        [many] { $count } файлів із папки «Документи» натомість розміщуються в { $target }, у папці _external, оскільки не вдалося визначити, де розташована папка «Документи» на цьому ПК. Щоб скористатися цими файлами, самостійно перемістіть їх у папку «Документи».
       *[other] { $count } файлів із папки «Документи» натомість розміщуються в { $target }, у папці _external, оскільки не вдалося визначити, де розташована папка «Документи» на цьому ПК. Щоб скористатися цими файлами, самостійно перемістіть їх у папку «Документи».
    }
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Saved Games folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-elsewhere =
    { $count ->
        [one] { $count } файл із папки «Збережені ігри» натомість розміщується в { $target }, у папці _external, оскільки папка «Збережені ігри» на цьому ПК розташована в { $folder }, а запис виконується лише в папку всередині вашого профілю. Щоб скористатися цим файлом, самостійно перемістіть його в папку «Збережені ігри».
        [few] { $count } файли з папки «Збережені ігри» натомість розміщуються в { $target }, у папці _external, оскільки папка «Збережені ігри» на цьому ПК розташована в { $folder }, а запис виконується лише в папку всередині вашого профілю. Щоб скористатися цими файлами, самостійно перемістіть їх у папку «Збережені ігри».
        [many] { $count } файлів із папки «Збережені ігри» натомість розміщуються в { $target }, у папці _external, оскільки папка «Збережені ігри» на цьому ПК розташована в { $folder }, а запис виконується лише в папку всередині вашого профілю. Щоб скористатися цими файлами, самостійно перемістіть їх у папку «Збережені ігри».
       *[other] { $count } файлів із папки «Збережені ігри» натомість розміщуються в { $target }, у папці _external, оскільки папка «Збережені ігри» на цьому ПК розташована в { $folder }, а запис виконується лише в папку всередині вашого профілю. Щоб скористатися цими файлами, самостійно перемістіть їх у папку «Збережені ігри».
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-unknown =
    { $count ->
        [one] { $count } файл із папки «Збережені ігри» натомість розміщується в { $target }, у папці _external, оскільки не вдалося визначити, де розташована папка «Збережені ігри» на цьому ПК. Щоб скористатися цим файлом, самостійно перемістіть його в папку «Збережені ігри».
        [few] { $count } файли з папки «Збережені ігри» натомість розміщуються в { $target }, у папці _external, оскільки не вдалося визначити, де розташована папка «Збережені ігри» на цьому ПК. Щоб скористатися цими файлами, самостійно перемістіть їх у папку «Збережені ігри».
        [many] { $count } файлів із папки «Збережені ігри» натомість розміщуються в { $target }, у папці _external, оскільки не вдалося визначити, де розташована папка «Збережені ігри» на цьому ПК. Щоб скористатися цими файлами, самостійно перемістіть їх у папку «Збережені ігри».
       *[other] { $count } файлів із папки «Збережені ігри» натомість розміщуються в { $target }, у папці _external, оскільки не вдалося визначити, де розташована папка «Збережені ігри» на цьому ПК. Щоб скористатися цими файлами, самостійно перемістіть їх у папку «Збережені ігри».
    }

## Errors when a folder that only administrators may change cannot be made so, or checked.

# { $path } is a folder.
core-privilege-not-a-folder = { $path } — це посилання або файл, а не папка
# { $path } is a folder. { $error } is what Windows said, in its own words.
core-privilege-not-made-here = { $path } створено не програмою WinAppMigrator, і інші можуть змінювати вміст цієї папки. Адміністратор має перевірити й видалити її: { $error }
# { $path } is a folder.
core-privilege-others-put-something-in = { $path } містить щось, створене не програмою WinAppMigrator, і інші можуть це змінювати. Адміністратор має перевірити й видалити цю папку
# { $path } is a folder.
core-privilege-became-a-link-in-use = { $path } перетворився на посилання під час використання
# { $path } is a folder.
core-privilege-became-a-link-looked-at = { $path } перетворився на посилання під час перевірки
# { $path } is a file. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-users-cannot-start = не вдалося дозволити користувачам запускати { $path }: { $said }
core-privilege-elevation-unknown = не вдалося визначити, чи має цей процес підвищені права
# { $path } is a folder. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-cannot-restrict = не вдалося обмежити доступ до { $path } лише адміністраторами: { $said }
# { $path } is a folder. { $said } is the folder's permissions as Windows wrote them, and what else it printed.
core-privilege-not-restricted = доступ до { $path } не вдалося обмежити лише адміністраторами, і диск, можливо, не зберігає дозволи: { $said }
# { $path } is a folder.
core-privilege-written-to-first = інший процес записав дані в { $path }, перш ніж папку вдалося захистити
core-privilege-no-program-data = Windows не повідомляє, де розташована папка ProgramData

## Checks of a package's files, and of what an install put in place. Each is said of one file, value, service, or the
## like, which the report names beside it.

# { $path } is the installer's file.
core-verify-no-package = { $path } не містить пакета
# { $path } is a folder.
core-verify-not-unpacked = пакет цього інсталятора не розпаковано в { $path }
core-verify-schema-supported = Схема маніфесту пакета підтримується.
core-verify-packaged-file-missing = Файлу з пакета немає.
core-verify-no-package-path = Файл для копіювання не має відносного шляху в пакеті.
core-verify-folder-there = Порожня папка існує.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-unreadable = Не вдалося прочитати для перевірки: { $error }
core-verify-restored-file-missing = Відновленого файлу немає.
# { $expected } and { $actual } are sizes in bytes.
core-verify-size-mismatch = Розмір файлу не збігається. Очікувалося байтів: { $expected }, виявлено: { $actual }.
core-verify-no-hash = У маніфесті не записано хеш цього файлу, тому його неможливо перевірити.
core-verify-hash-mismatch = Хеш SHA-256 не збігається.
core-verify-file-matches = Розмір і хеш файлу відповідають маніфесту.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-payload-failed = Помилка перевірки вмісту ліцензії: { $error }
core-verify-license-payload-missing = Вмісту ліцензії з пакета немає.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-failed = Помилка перевірки сховища ліцензій: { $error }
core-verify-license-store-file-missing = Файлу сховища ліцензій із пакета немає.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-path-invalid = Неприпустимий шлях до сховища ліцензій у пакеті: { $error }
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-prerequisite-failed = Помилка перевірки вмісту необхідного компонента: { $error }
core-verify-prerequisite-missing = Вбудованого вмісту необхідного компонента немає.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-failed = Помилка перевірки образу драйвера: { $error }
core-verify-bundled-driver-missing = Вбудованого образу драйвера немає.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-path-invalid = Неприпустимий шлях до вбудованого драйвера: { $error }
core-verify-prerequisite-present = Необхідне середовище виконання є на цільовому ПК.
core-verify-prerequisite-not-detected = Необхідне середовище виконання не виявлено; можливо, потрібне перезавантаження або завершення вручну (остаточний результат див. у журналі відновлення).
core-verify-prerequisite-not-probed = Наявність необхідного компонента неможливо перевірити за реєстром; інсталяцію виконано без гарантії результату.
core-verify-license-file-missing = Відновленого файлу ліцензії немає.
core-verify-license-store-restored-missing = Відновленого файлу сховища ліцензій немає.
core-verify-registry-key-there = Розділ реєстру існує.
core-verify-registry-value-matches = Параметр реєстру відповідає архівованим даним.
core-verify-registry-value-differs = Параметр реєстру не відповідає архівованим даним.
core-verify-firewall-rule-there = Це правило є в брандмауері Windows.
core-verify-variable-matches = Значення змінної середовища відповідає архівованим даним.
core-verify-variable-differs = Значення змінної середовища не відповідає архівованим даним.
# { $error } is what Windows said, in its own words.
core-verify-variable-unreadable = Змінна середовища відсутня або недоступна для читання: { $error }
core-verify-service-runs-other = Служба існує, але не запускає відновлений виконуваний файл із його аргументами.
core-verify-service-other-account = Служба існує, але працює не від імені очікуваного облікового запису.
core-verify-service-matches = Служба існує й запускає відновлений виконуваний файл із його аргументами від імені потрібного облікового запису.
core-verify-service-missing = Служби немає.
# { $error } is what Windows said, in its own words.
core-verify-service-unreadable = Не вдалося прочитати службу: { $error }
core-verify-driver-other-image = Служба драйвера не вказує на відновлений образ.
core-verify-driver-not-registered = Службу вбудованого драйвера не зареєстровано після відновлення.
# { $error } is what Windows said, in its own words.
core-verify-driver-unreadable = Не вдалося прочитати службу драйвера: { $error }
core-verify-driver-image-missing = Образу драйвера немає.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-unchecked = Не вдалося перевірити образ драйвера: { $error }
core-verify-task-matches = Заплановане завдання існує й запускає відновлені програми.
core-verify-task-runs-other = Заплановане завдання існує, але не запускає відновлені програми.
core-verify-task-missing = Запланованого завдання немає.
# { $error } is what Windows said, in its own words.
core-verify-task-unreadable = Не вдалося отримати відомості про заплановане завдання: { $error }
core-verify-shortcut-matches = Ярлик існує, і шлях до його об'єкта збігається з відновленим.
core-verify-shortcut-opens-other = Ярлик існує, але шлях до його об'єкта не збігається з відновленим.
core-verify-shortcut-missing = Ярлика немає.
# { $error } is what Windows said, in its own words.
core-verify-shortcut-unreadable = Не вдалося отримати відомості про ярлик: { $error }

## Warnings and errors a check gives, which the app and the installer show.

core-warning-verify-schema-unsupported = Схема маніфесту пакета не підтримується.
# { $error } is what went wrong, in Windows' or the program's own words.
core-warning-verify-package-file-failed = Помилка перевірки файлу пакета: { $error }
core-warning-verify-folder-missing = Порожньої папки немає.
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-key-missing = Розділ реєстру відсутній або недоступний для читання: { $error }
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-value-missing = Параметр реєстру відсутній або недоступний для читання: { $error }
core-warning-verify-registry-kind-missing = Не вказано тип параметра реєстру.
core-warning-verify-firewall-rule-missing = Цього правила немає в брандмауері Windows.
core-warning-verify-service-scope = Перевірка служб читає конфігурацію служб у HKLM і не запускає служби.
core-warning-verify-driver-manual = WinAppMigrator не поширює один або кілька драйверів постачальників чи вбудованих драйверів Windows; інсталюйте їх повторно на цільовому ПК з підписаного пакета драйвера постачальника.
core-warning-verify-task-scope = Перевірка запланованих завдань перевіряє лише реєстрацію та посилання на команду.

## The verification report written beside a package, verification_report.md. Its headings follow a "#".

core-verify-report-title = Звіт про перевірку WinAppMigrator
# { $time } is when, as a date and time in UTC, such as 2026-11-15T10:00:00Z.
core-verify-report-verified = Перевірено (UTC): { $time }
# { $valid } is true or false, which are not translated.
core-verify-report-package-valid = Пакет дійсний: { $valid }
# { $valid } is true or false, which are not translated.
core-verify-report-restore-valid = Відновлення дійсне: { $valid }
core-verify-report-restore-not-run = Відновлення дійсне: не виконувалося
core-verify-report-items = Елементи
# { $message } is what a check found, and { $source } the file or setting it looked at.
core-verify-report-item-from = { $message } Джерело: { $source }
core-verify-report-warnings = Попередження
core-verify-report-errors = Помилки
core-verify-report-none = Немає.
