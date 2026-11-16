# Words of the installer that every built installer carries, which runs on the PC an app is moved to and speaks its
# language, and of the uninstaller it leaves behind for Settings. Message names start with installer- for the
# installer's window, installer-cli- for its command line, and uninstaller- for the uninstaller.
#
# The command lines (installer-cli-, uninstaller-cli-) are always shown in English, as scripts read what they print.
# Translate them all the same. Keep their command-line flags, such as --apply and --target-root, exactly as they are.


## The installer's window: its title and header

installer-window-title = Інсталятор WinAppMigrator
# The window's title in the taskbar. { $app_name } is the name of the app it installs.
installer-window-title-app = Інсталяція програми { $app_name }
installer-tagline = Повторно інсталює програму, отриману з іншого ПК
# What an app is called when its package gives it no name.
installer-unnamed-app = Програма

## When the installer's window cannot open. Each is a paragraph of a message box.

# { $error } is what went wrong, as Windows or the graphics driver says it.
installer-window-failed = Не вдалося відкрити вікно інсталятора: { $error }
installer-window-failed-without-one = Інсталювати можна й без нього: запустіть цей файл із командного рядка, відкритого від імені адміністратора, з параметром --apply, а щоб вибрати папку, додайте --target-root і шлях до неї.

## Checking the installer when it opens

installer-checking = Перевірка інсталятора…
installer-stub-no-package = Це заглушка інсталятора WinAppMigrator, і вона не містить пакета програми. Створіть інсталятор у WinAppMigrator, а потім запустіть тут файл .exe, який він створить.
installer-check-stopped = Перевірку інсталятора зупинено.
# { $error } is what went wrong, as Windows says it.
installer-locate-failed = Не вдалося знайти інсталятор: { $error }
# { $error } is what went wrong reading it.
installer-read-failed = Не вдалося прочитати інсталятор: { $error }
# { $error } is why, such as that the app needs a newer Windows.
installer-cannot-be-used-here = Пакет цього інсталятора не можна використати на цьому ПК: { $error }

## The app, where it goes, and whether the installer is signed

installer-captured-from = Отримано з
installer-install-location = Розташування інсталяції
# The title of the folder picker.
installer-choose-location = Виберіть розташування інсталяції
installer-folder-has-files = У цій папці вже є файли. Файли з такими самими іменами буде замінено, а якщо видалити програму, вони повернуться.
installer-room-needed = Потрібне місце
# { $shortfall } says which drive is short of room, such as "C: needs up to 824.0 MB for the app's files, and has 300.0 MB free".
installer-room-short = Тут може не вистачити місця. { $shortfall }. Розпакувати пакет можна й на іншому диску, але файли програми все одно мають тут поміститися.
# { $app_name } is the name of an app whose earlier install was cut short.
installer-interrupted = Попередню інсталяцію програми { $app_name } не було завершено. Перед інсталяцією її буде скасовано.
# Beside "Installer", for whether it proves who made it.
installer-signature = Інсталятор
installer-signature-checking = Перевірка підпису…
installer-signature-unsigned = Не підписано: видавець невідомий
# { $publisher } is the name on the installer's certificate, such as "Fabrikam, Inc.".
installer-signature-signed-by = Підписано: { $publisher }
installer-signature-unnamed = Підписано видавцем без імені
# { $reason } is Windows' own word for what is wrong, such as HashMismatch.
installer-signature-invalid = Підпис цього інсталятора недійсний ({ $reason }). Можливо, інсталятор змінили після підписання.

## Why the install location cannot be used. Install and Preview Changes stay off while one of these is shown.

# { $app_name } is the name of the app being installed.
installer-location-app-already-there = Програму { $app_name } уже інстальовано в цій папці, тому цю копію не можна сюди інсталювати. Щоб інсталювати її тут, спершу видаліть копію з цього ПК у програмі «Параметри» в розділі «Програми», а потім знову відкрийте цей інсталятор. Або введіть вище нову порожню папку.
# { $other_app } is the name of another app Windows lists as installed in the folder. { $app_name } is the name of the app being installed.
installer-location-other-app-there = У цій папці інстальовано програму { $other_app }, тому програму { $app_name } не можна сюди інсталювати. Введіть вище нову порожню папку. Або, якщо програма { $other_app } вам більше не потрібна, спершу видаліть її в програмі «Параметри» в розділі «Програми», а потім знову відкрийте цей інсталятор.
installer-location-has-files = У цій папці вже є файли, які інсталював не WinAppMigrator. Вони можуть належати іншій програмі, тому поверх них нічого не інсталюється. Введіть вище нову порожню папку.
# { $reason } is why, such as that it cannot be the Windows folder.
installer-location-refused = Цю папку не можна використати: { $reason }. Введіть вище іншу папку.

## What the install sets up beyond its files

installer-beyond-files = Що ще налаштовується, крім файлів
# Under the heading above. It is part of what a person agrees to when they click Install.
installer-beyond-files-agreed = Усе це виконується з правами адміністратора. Якщо інсталяція не вдасться, внесені нею зміни буде повернуто, а вам повідомлять про все, що повернути не вдалося. Пізніше інсталяцію також можна скасувати в «Параметрах», у розділі «Інстальовані програми». Замінені нею файли повернуться, а інстальовані середовища виконання залишаться.
installer-beyond-files-none = Нічого: копіюються лише файли та параметри у власних розділах реєстру програми.
# { $problem } is why, such as that the app cannot run on this PC.
installer-cannot-go-here = Цей пакет не можна інсталювати в це розташування: { $problem }
installer-working-out = Визначення того, що буде налаштовано в цьому розташуванні…
# Said as the reason after "This package cannot go to this location:".
installer-list-stopped = визначення того, що буде зроблено, зупинилося
# A heading over a list of what else the package asked for and will not get. { $count } is how many.
installer-more-left-out =
    { $count ->
        [one] Не включено ще { $count } елемент
        [few] Не включено ще { $count } елементи
        [many] Не включено ще { $count } елементів
       *[other] Не включено ще { $count } елементів
    }

## Options

installer-options = Параметри
installer-user-data = Дані користувача
installer-user-data-detail = Якщо ввімкнуто: параметри, «Документи» та «Збережені ігри» потрапляють у звичайні папки того, хто ввійшов у систему. Якщо вимкнуто: усе це розміщується в папці інсталяції, і звідти нічого не налаштовується для запуску.
# { $account } is the name of the account on the other PC whose data this is.
installer-user-data-whose = Тут зберігаються дані облікового запису { $account }.
installer-license = Ліцензія
installer-license-detail = Файли активації та ліцензій, а також параметри реєстру, у яких зберігаються ліцензії. Деякі ліцензії, можливо, доведеться тут активувати повторно.

## Dependencies: what else the app needs, and how each is provided

installer-dependencies = Залежності
installer-dependencies-detail = Що потрібно цій програмі та як це надається.
installer-dependencies-none = Додаткових залежностей від середовищ виконання не виявлено.
# { $feature } is the name of an optional part of Windows, such as NetFx3.
installer-dependency-windows-feature = Компонент Windows: { $feature }
installer-dependency-included = Включено
# { $id } is the package's name in winget, such as Microsoft.VCRedist.2015+.x64.
installer-dependency-winget = Інсталюється за допомогою winget: { $id }
installer-dependency-manual = Інсталювати вручну
installer-dependency-built-in = Вбудовано в Windows
installer-dependency-ships-with-app = Постачається з програмою

## Buttons, and what is said beside them

installer-install = Інсталювати
installer-preview = Переглянути зміни
installer-cancel = Скасувати
installer-close = Закрити
installer-open-folder = Відкрити папку інсталяції
# { $error } is what went wrong, as Windows says it.
installer-open-folder-failed = Не вдалося відкрити папку: { $error }
installer-location-blocks = Кнопки «Інсталювати» та «Переглянути зміни» недоступні, доки не можна використати розташування інсталяції. Причину вказано вище, під цим полем.
installer-package-blocks = Кнопка «Інсталювати» недоступна, оскільки цей пакет не можна інсталювати в це розташування. Причину вказано вище.
installer-list-changed = Дії, які буде виконано тут, змінилися, тому нічого не інстальовано. Перегляньте список нижче й знову натисніть «Інсталювати», якщо вам потрібно саме це.
installer-cancelled-early = Скасовано. Нічого не змінено.

## How far an install has got, beside its progress bar

# { $step } is one of the steps below. { $done } and { $total } are how many of its items are done, and how many there are.
installer-step-counted = { $step }: { $done } з { $total }…
# { $step } is one of the steps below, when it does not count its items.
installer-step-uncounted = { $step }…
installer-step-getting-ready = Підготовка…
installer-step-cancelling = Скасування після поточного кроку…
installer-step-putting-back = Повернення внесених змін
installer-step-unpacking = Розпакування
installer-step-checking-package = Перевірка пакета
installer-step-recovering = Скасування попередньої незавершеної інсталяції
# What a preview is doing, whatever step it is on.
installer-step-previewing = Визначення змін
installer-step-runtimes = Інсталяція середовищ виконання
installer-step-files = Копіювання файлів
installer-step-services = Налаштування служб
installer-step-drivers = Налаштування драйверів
installer-step-registry = Запис параметрів реєстру
installer-step-license = Відновлення даних ліцензії
installer-step-tasks = Налаштування запланованих завдань
installer-step-shortcuts = Створення ярликів
installer-step-environment = Налаштування змінних середовища
installer-step-settings-entry = Додавання програми до «Параметрів»
installer-step-verifying = Перевірка інстальованого
installer-step-undoing = Скасування змін

## When an install cannot go ahead or stops

installer-carries-no-package = Цей інсталятор не містить пакета.
installer-stopped-unexpectedly = Інсталяцію неочікувано зупинено.
# { $reason } is what went wrong, as the program says it.
installer-stopped-unexpectedly-because = Інсталяцію неочікувано зупинено: { $reason }. Якщо вона вже почалася, повторний запуск інсталятора спершу скасує внесені нею зміни.
# { $error } is what went wrong reading it.
installer-package-read-failed = Не вдалося прочитати пакет: { $error }
installer-no-programdata = Не вдалося знайти папку ProgramData на цьому ПК, тому нічого не інстальовано: саме там інсталяція зберігає запис, за яким її скасовують.
# { $error } is why, such as that there is nowhere only administrators can get into.
installer-staging-failed = Не вдалося створити проміжну папку: { $error }
# { $error } is what went wrong unpacking it.
installer-unpack-failed = Не вдалося розпакувати пакет: { $error }
installer-staging-nowhere = немає папки для розпакування пакета, доступ до якої мають лише адміністратори
# { $failure } is a folder and what went wrong there, such as "C:\ProgramData\WinAppMigrator: Access is denied.".
installer-staging-nowhere-look-at = немає папки для розпакування пакета, доступ до якої мають лише адміністратори. Варто перевірити: { $failure }

## What came of an install, or of a preview

installer-result = Результат інсталяції
installer-result-preview = Попередній перегляд: ще нічого не змінено
installer-result-installed = Програму інстальовано, і вона пройшла перевірку хешів.
installer-result-installed-without-left-out = Програму інстальовано, і вона пройшла перевірку хешів, але без того, що нижче вказано як не включене.
installer-result-not-all-put-back = Інсталяція не вдалася, і не всі внесені нею зміни вдалося повернути. Те, що залишилося, наведено нижче; наступна інсталяція спробує повернути це знову.
installer-result-cancelled-put-back = Інсталяцію скасовано, і внесені нею зміни повернуто.
installer-result-failed-put-back = Інсталяція не вдалася, і внесені нею зміни повернуто.
installer-result-not-clean = Інсталяцію завершено з проблемами. Перегляньте їх нижче.
installer-result-changes-put-back = Повернуті зміни
installer-result-changes-made = Внесені зміни
installer-result-changes-planned = Заплановані зміни
installer-result-warnings = Попередження
installer-result-errors = Помилки
installer-result-package-valid = Хеші пакета дійсні
installer-result-package-invalid = Хеші пакета НЕДІЙСНІ
installer-result-files-valid = Інстальовані файли дійсні
installer-result-files-invalid = Інстальовані файли НЕДІЙСНІ
installer-result-reboot = Щоб завершити інсталяцію залежності, потрібне перезавантаження. Перезавантажте Windows, а потім запустіть програму.
# A heading over the list of what was changed. { $count } is how many.
installer-result-changes = Зміни ({ $count })
# After the first part of a long list. { $count } is how many more there are.
installer-result-changes-more = і ще { $count }.
# Over the path of the install's log file.
installer-log = Журнал

## The installer's command line. Shown in English: scripts read it. A { $count } is plain digits, with no separators.

installer-cli-stub = Заглушка інсталятора WinAppMigrator — вона не містить пакета програми.
installer-cli-stub-how = Створіть інсталятор у програмі WinAppMigrator, а потім запустіть створений файл .exe на цільовому комп'ютері.
installer-cli-usage-title = Саморозпакувальний інсталятор WinAppMigrator.
installer-cli-usage = Запустіть його без аргументів, щоб відкрити графічний інсталятор, або так:
installer-cli-usage-preview = переглянути зміни:    --dry-run [--target-root <ПАПКА>]
installer-cli-usage-apply = повторно інсталювати: --apply [--target-root <ПАПКА>]
installer-cli-no-package = інсталятор не містить вбудованого пакета
installer-cli-no-programdata = не вдалося знайти папку ProgramData на цьому ПК, тому нічого не інстальовано: саме там інсталяція зберігає запис, за яким її скасовують
# { $path } is the folder asked for, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-folder-has-files = { $path } не можна використати: { $problem }. Указуйте --force, лише якщо знаєте, що ця папка належить цій програмі
installer-cli-no-location = у цьому пакеті не вказано, де була його програма, тому вкажіть --target-root
# { $path } is where the app was on the other PC, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-location-refused = програма цього пакета була в { $path }, і цю папку не можна використати: { $problem }. Вкажіть --target-root
# { $summary } is how much room it needs where, such as "up to 824.0 MB on C: (356.0 GB free)".
installer-cli-room = Місце: { $summary }
# { $note } is something to know about the install.
installer-cli-note = Примітка: { $note }
# { $count } is how many things it sets up beyond its files.
installer-cli-sets-up = Налаштовує: { $count }
installer-cli-applied = Повторну інсталяцію виконано.
installer-cli-previewed = Попередній перегляд повторної інсталяції виконано.
# { $count } is how many changes the install logged.
installer-cli-changes-logged = Записано змін: { $count }
# { $count } is how many warnings.
installer-cli-warnings = Попередження: { $count }
# { $count } is how many errors.
installer-cli-errors = Помилки: { $count }
# { $count } is how many things the package asked for and did not get.
installer-cli-refused = Відхилено: { $count }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-package-valid = Хеші пакета дійсні: { $valid }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-reinstall-valid = Хеші повторної інсталяції дійсні: { $valid }
# { $id } is the install's id, such as wam-0f3a….
installer-cli-install-id = Ідентифікатор інсталяції: { $id }
installer-cli-put-back = Інсталяція не вдалася, тому внесені нею зміни повернуто.
# { $path } is the log file.
installer-cli-log = Журнал: { $path }
# { $path } is the log file. Printed before an error.
installer-cli-error-log = журнал: { $path }
# { $error } is what went wrong.
installer-cli-error = помилка: { $error }

## The uninstaller's window

uninstaller-window-title = Видалення
# The window's title, and its heading. { $app_name } is the name of the app it uninstalls.
uninstaller-window-title-app = Видалення програми { $app_name }
uninstaller-tagline = Скасовує те, що зробив її інсталятор
uninstaller-reading = Читання запису цієї інсталяції…
uninstaller-stopped-unexpectedly = Видалення неочікувано зупинено.
uninstaller-what-will-be-removed = Що буде видалено
uninstaller-install-location = Розташування інсталяції
uninstaller-files = Файли
uninstaller-registry-values = Параметри реєстру
uninstaller-services = Служби
uninstaller-drivers = Драйвери
uninstaller-scheduled-tasks = Заплановані завдання
uninstaller-environment = Змінні середовища
uninstaller-plan-stays = Файли, які ви додали або змінили, залишаться разом із папками, у яких вони містяться.
# { $count } is how many files the install replaced.
uninstaller-plan-coming-back =
    { $count ->
        [one] { $count } файл, замінений цією інсталяцією, повернеться або буде розміщений поруч із файлом, який відтоді змінився.
        [few] { $count } файли, замінені цією інсталяцією, повернуться або будуть розміщені поруч із файлами, які відтоді змінилися.
        [many] { $count } файлів, замінених цією інсталяцією, повернуться або будуть розміщені поруч із файлами, які відтоді змінилися.
       *[other] { $count } файлів, замінених цією інсталяцією, повернуться або будуть розміщені поруч із файлами, які відтоді змінилися.
    }
# { $runtimes } is a list of runtimes, such as "Microsoft Visual C++ Redistributable, .NET Desktop Runtime 8".
uninstaller-plan-runtimes = Інстальовані нею середовища виконання залишаться, бо їх можуть використовувати інші програми: { $runtimes }.
uninstaller-options = Параметри
uninstaller-remove-user-data = Також видалити дані користувача й дані ліцензії
# { $count } is how many items of the app's settings, data, and license there are.
uninstaller-remove-user-data-detail =
    { $count ->
        [one] { $count } елемент параметрів, даних і ліцензії програми. Він залишиться, якщо не ввімкнути цей параметр. Замінене ним повернеться або, якщо елемент відтоді змінився, буде розміщене поруч із ним.
        [few] { $count } елементи параметрів, даних і ліцензії програми. Вони залишаться, якщо не ввімкнути цей параметр. Замінене ними повернеться або, якщо вони відтоді змінилися, буде розміщене поруч із ними.
        [many] { $count } елементів параметрів, даних і ліцензії програми. Вони залишаться, якщо не ввімкнути цей параметр. Замінене ними повернеться або, якщо вони відтоді змінилися, буде розміщене поруч із ними.
       *[other] { $count } елементів параметрів, даних і ліцензії програми. Вони залишаться, якщо не ввімкнути цей параметр. Замінене ними повернеться або, якщо вони відтоді змінилися, буде розміщене поруч із ними.
    }
uninstaller-uninstall = Видалити
uninstaller-cancel = Скасувати
uninstaller-close = Закрити
# { $done } and { $total } are how many of the install's items are undone, and how many there are.
uninstaller-step-counted = Скасування інсталяції: { $done } з { $total }…
uninstaller-step = Скасування інсталяції…
uninstaller-done = Програму видалено.
uninstaller-done-except = Програму видалено, крім указаного нижче.
uninstaller-waits-for-drive = Частина програми розміщена на диску, який не підключено. Підключіть його та повторіть видалення в «Параметрах».
uninstaller-not-all-undone = Не все вдалося скасувати. Закрийте те, що використовує програму, і повторіть видалення в «Параметрах».
# Over the path of the uninstall's log file.
uninstaller-log = Журнал
uninstaller-undone = Скасовано
uninstaller-left-in-place = Залишено на місці
uninstaller-could-not-be-undone = Не вдалося скасувати
# After the first part of a long list. { $count } is how many more there are.
uninstaller-and-more = і ще { $count }.
# Said as the reason when there is no record of the install to read.
uninstaller-no-programdata = не вдалося знайти папку ProgramData на цьому ПК, тому немає запису, який можна прочитати

## When the uninstaller's window cannot open. Each is a paragraph of a message box, followed by a command to type.

uninstaller-window-failed-title = Програма видалення WinAppMigrator
# { $error } is what went wrong, as Windows or the graphics driver says it.
uninstaller-window-failed = Не вдалося відкрити вікно програми видалення: { $error }
uninstaller-window-failed-quiet = Видалити програму можна й без нього — з командного рядка, відкритого від імені адміністратора:
# Stands in for this program's own path in that command, if it cannot be found.
uninstaller-this-program = ця програма

## The uninstaller's command line (--quiet). Shown in English: scripts read it.

# { $app_name } is the name of the app.
uninstaller-cli-uninstalling = Видалення програми { $app_name }.
# { $path } is the log file.
uninstaller-cli-log = Журнал: { $path }
# { $error } is what went wrong.
uninstaller-cli-error = помилка: { $error }
