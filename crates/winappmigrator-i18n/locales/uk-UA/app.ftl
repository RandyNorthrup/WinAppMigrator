# Words of the app, WinAppMigrator itself: its one window, with the installed apps listed on the left and, on the
# right, what goes in an installer of the one picked and the button that builds it; and the steps to read another PC
# through WinAppMigrator Client. Message names start with app-.


## The window and its header

# The program's name, in the window's title bar and at the top of the window. Keep it as it is unless your language
# writes product names in a script of its own.
app-name = WinAppMigrator
app-tagline = Перетворіть інстальовану програму на саморозпакувальний інсталятор
app-theme-to-light = Перейти на світлу тему
app-theme-to-dark = Перейти на темну тему
app-reload-list = Оновити список програм
app-list-loading = Завантаження
# { $count } is how many apps are listed.
app-list-count =
    { $count ->
        [one] { $count } програма
        [few] { $count } програми
        [many] { $count } програм
       *[other] { $count } програм
    }
app-list-load-failed = Не вдалося завантажити
# Shown when the window cannot open at all. { $error } is what went wrong, in the words of Windows or a graphics driver.
app-window-could-not-open = Не вдалося відкрити вікно: { $error }

## The list of installed apps

app-list-search = Пошук за назвою або видавцем  (Ctrl+F)
app-list-filter-all = Усі
# Whether an app is installed for one account or for everyone on the PC.
app-scope-per-user = Для користувача
app-scope-per-machine = Для всього ПК
app-scope-unknown = Невідомо
# An app whose processor architecture (x86, x64, ARM64) is not known.
app-arch-unknown = Невідомо
app-list-loading-apps = Завантаження інстальованих програм…
# { $error } is why the apps could not be listed.
app-list-could-not-list = Не вдалося отримати список програм: { $error }
app-list-no-matches = Немає відповідних програм
app-list-no-matches-hint = Змініть текст пошуку або фільтр області.
app-list-worker-disconnected = Робочий процес отримання списку програм відключився.
# What a screen reader says of an app's row. { $name } is the app's name, { $publisher } who made it, and { $version } its
# version, such as 11.4.2, or — when either is not known.
app-list-row = { $name }, { $publisher }, версія { $version }

## The list's columns, and what a screen reader says each column's heading does when clicked

app-column-name = Назва
app-column-name-sort = Сортувати за назвою
app-column-publisher = Видавець
app-column-publisher-sort = Сортувати за видавцем
app-column-version = Версія
app-column-version-sort = Сортувати за версією
app-column-scope = Область
app-column-scope-sort = Сортувати за областю
# Short for architecture: x86, x64, or ARM64. The column is narrow.
app-column-arch = Арх.
app-column-arch-sort = Сортувати за архітектурою

## Where apps are read from: this PC, a drive from another PC, a disk image of one, or another PC on the network

app-source = Джерело сканування
app-source-this-pc = Цей ПК
app-source-drive = Підключений диск…
app-source-image = Образ диска…
app-source-network = Мережевий ПК…
app-source-attaching-image = Підключення образу лише для читання…
# { $computer } is the other PC's name, such as OLD-PC, and { $address } its network address, such as 192.168.1.20.
app-source-network-pc = { $computer } у мережі ({ $address })
app-source-disconnect = Відключити
app-source-change = Змінити…
# The title of the window that picks a disk image. .vhd and .vhdx are file extensions.
app-source-pick-image-title = Виберіть образ диска іншого ПК (.vhd або .vhdx)
app-source-pick-image-filter = Образи дисків
# { $path } is the disk image file.
app-source-image-has-no-windows = { $path } не містить тому з Windows.
app-source-image-not-attached = Не вдалося підключити образ диска.
app-source-pick-drive-title = Виберіть кореневу папку підключеного диска (наприклад, E:\)
# { $path } is the folder picked, such as E:\. \Windows and \Users are folder names, as Windows writes them.
app-source-not-a-windows-drive = { $path } не схожий на системний диск Windows (немає \Windows і \Users). Виберіть кореневу папку диска з іншого ПК.
# Said of a drive or disk image with a Windows older than Windows 10. { $windows } is its name, such as "Windows 7
# Professional Service Pack 1, 32-bit". { $is_32_bit } is "yes" when that Windows is 32-bit, which adds the last
# sentence, and "no" when it is not.
app-source-older-windows =
    Тут інстальовано { $windows }. Інсталятори, створені звідси, призначено для Windows 10 і 11. Якщо ввімкнуто «Дані користувача», разом із програмою переносяться параметри, які Windows непомітно зберігала окремо для старих програм, а середовища виконання, яких немає в Windows 11 (наприклад, DirectX 9), інсталюються разом із програмою.{ $is_32_bit ->
        [yes] { " " }Програми звідси 32-розрядні, тому вони потрапляють у Program Files (x86), де їх зберігає 64-розрядна Windows.
       *[no] { "" }
    }
# The same, of a Windows whose name could not be read. { $is_32_bit } is as above.
app-source-older-windows-unnamed =
    Тут інстальовано старішу версію Windows. Інсталятори, створені звідси, призначено для Windows 10 і 11. Якщо ввімкнуто «Дані користувача», разом із програмою переносяться параметри, які Windows непомітно зберігала окремо для старих програм, а середовища виконання, яких немає в Windows 11 (наприклад, DirectX 9), інсталюються разом із програмою.{ $is_32_bit ->
        [yes] { " " }Програми звідси 32-розрядні, тому вони потрапляють у Program Files (x86), де їх зберігає 64-розрядна Windows.
       *[no] { "" }
    }

## The details panel, before an app is picked, and the app picked

app-details-pair-first = Зв'язування зі старим ПК
app-details-pair-first-hint = Коли на ньому буде запущено WinAppMigrator Client і ви введете його код, тут з'являться програми того ПК, і ви зможете створити інсталятор будь-якої з них так само, як на цьому ПК.
app-details-pick-an-app = Виберіть програму
app-details-pick-an-app-hint = Виберіть програму в списку, щоб указати, що включити, і створити її інсталятор.
# { $publisher } is who made the app, and { $version } its version, such as 11.4.2.
app-details-publisher-and-version = { $publisher }, версія { $version }
# { $version } is the app's version, such as 11.4.2.
app-details-unknown-publisher-and-version = Невідомий видавець, версія { $version }
app-details-unknown-publisher = Невідомий видавець

## What goes in besides the program

app-option-license = Дані ліцензії та активації
app-option-license-hint = Можуть бути прив'язані до цього ПК і потребувати повторної активації на новому.
app-option-user-data = Дані користувача
# Documents, Saved Games, and ProgramData are folders of Windows: use the names Windows gives them in your language,
# except ProgramData, which keeps its name.
app-option-user-data-hint = Параметри й історія, «Документи», «Збережені ігри» та ProgramData.
app-option-whose-data = Чиї дані
app-option-whose-data-hint = Інсталятор містить дані одного облікового запису. Вони потрапляють до того, хто ввійшов у систему там, де інсталятор запущено.

## Building the installer

app-output-folder = Папка для збереження
# An example of a folder, shown in the empty field. "You" stands for the name of an account.
app-output-folder-example = C:\Users\Ви\WinAppMigrator
app-output-folder-not-full = Введіть повний шлях до папки, наприклад D:\Інсталятори.
app-build-create = Створити інсталятор
app-build-create-again = Створити інсталятор ще раз
app-build-cancel = Скасувати
app-build-cancelling = Скасування після поточного кроку…
app-build-cancelled = Створення скасовано. Інсталятор не записано.
# winappmigrator-installer.exe is a file name.
app-build-no-stub = Поруч із цією програмою не знайдено заглушку інсталятора (winappmigrator-installer.exe), тому саморозпакувальний інсталятор створити не можна.
app-build-select-an-app-first = Спершу виберіть програму для перенесення.
app-build-look-again-first = Перш ніж створювати інсталятор, цю програму потрібно просканувати ще раз.
# { $reason } is what went wrong, in the program's own words, often in English.
app-build-stopped-unexpectedly = Створення неочікувано зупинилося: { $reason }. Якщо в папці для збереження залишився інсталятор, не використовуйте його.
app-build-worker-disconnected = Робочий процес перенесення відключився.
# { $path } is the output folder, and { $error } what Windows said went wrong.
app-build-no-output-folder = Не вдалося створити папку для збереження { $path }: { $error }
# { $error } is what went wrong.
app-build-no-staging-folder = Не вдалося створити захищену проміжну папку, тому нічого не створено: { $error }
# { $error } is what went wrong, as a sentence.
app-build-could-not-build = Не вдалося створити інсталятор. { $error }
app-build-no-installer = Створення завершилося, але інсталятора немає.
# { $error } is what went wrong.
app-build-hash-check-did-not-run = Інсталятор записано, але не вдалося виконати перевірку його хешів: { $error }. Не використовуйте його.

## How far a build has got

app-progress-preparing = Визначення вмісту…
app-progress-checking-room = Перевірка вільного місця…
app-progress-bundling = Додавання середовищ виконання й драйверів…
# { $done } is how many files have been read so far, and { $total } how many there are.
app-progress-packing = Читання й стиснення файлів: { $done } з { $total }
# { $done } is how many of the installer's records have been looked over so far, and { $total } how many there are.
app-progress-looking-over = Перевірка записів інсталятора: { $done } з { $total }…
app-progress-writing = Запис інсталятора…
app-progress-verifying = Перевірка хешів…

## What a build made

app-result-failed-hash-check = Не використовуйте цей інсталятор: він не пройшов перевірку хешів.
# { $count } is how many files were gone when the build came to read them.
app-result-files-gone =
    { $count ->
        [one] Інсталятор створено, але { $count } файл зник після сканування програми, тому його не включено. Докладніше — у розділі «Попередження» нижче.
        [few] Інсталятор створено, але { $count } файли зникли після сканування програми, тому їх не включено. Докладніше — у розділі «Попередження» нижче.
        [many] Інсталятор створено, але { $count } файлів зникли після сканування програми, тому їх не включено. Докладніше — у розділі «Попередження» нижче.
       *[other] Інсталятор створено, але { $count } файлів зникли після сканування програми, тому їх не включено. Докладніше — у розділі «Попередження» нижче.
    }
# { $count } is how many files could not be read into the installer.
app-result-files-not-captured =
    { $count ->
        [one] Інсталятор створено, але не вдалося отримати { $count } файл, зазвичай тому, що він заблокований або недоступний для читання. Без нього програма може не працювати. Докладніше — у розділі «Попередження» нижче.
        [few] Інсталятор створено, але не вдалося отримати { $count } файли, зазвичай тому, що вони заблоковані або недоступні для читання. Без них програма може не працювати. Докладніше — у розділі «Попередження» нижче.
        [many] Інсталятор створено, але не вдалося отримати { $count } файлів, зазвичай тому, що вони заблоковані або недоступні для читання. Без них програма може не працювати. Докладніше — у розділі «Попередження» нижче.
       *[other] Інсталятор створено, але не вдалося отримати { $count } файлів, зазвичай тому, що вони заблоковані або недоступні для читання. Без них програма може не працювати. Докладніше — у розділі «Попередження» нижче.
    }
app-result-mentions-who = Інсталятор створено, але в ньому досі є відомості про те, від кого він походить. Перш ніж комусь його передавати, прочитайте наведене нижче.
app-result-ready = Інсталятор готовий. Його пакет пройшов перевірку хешів.
app-result-files = Файли
app-result-installer-size = Розмір інсталятора
app-result-warnings = Попередження
app-result-log = Журнал
app-result-copy-error = Копіювати помилку
app-result-installer = Інсталятор
app-result-data-file = Файл даних
app-result-open-folder = Відкрити папку
app-result-copy-path = Копіювати шлях
# { $error } is what Windows said went wrong.
app-result-could-not-open-folder = Не вдалося відкрити папку: { $error }
# { $installer } and { $data_file } are file names, such as App_installer.exe and App_installer.wamdata.
app-result-two-files = Ця програма завелика для одного виконуваного файлу, тому її інсталятор складається з двох файлів: { $installer } і { $data_file }. Вони мають бути разом: скопіюйте обидва на інший ПК в одну папку з цими самими іменами. Якщо ви підпишете інсталятор, цей підпис охоплюватиме й файл даних.
# { $app_name } is the app's name. "Unknown publisher" is what Windows shows when it asks to run an unsigned program: use
# Windows' own words for it in your language.
app-result-run-it = Запустіть його на іншому ПК, щоб повторно інсталювати { $app_name }. Він перевіряє свої файли за SHA-256. Windows показує «Невідомий видавець», оскільки створені вами інсталятори не підписано.
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-verification-errors = Помилки перевірки ({ $count })
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-warnings-heading = Попередження ({ $count })
# After the first of a long list of warnings. { $count } is how many more there are.
app-warnings-and-more = і ще { $count }.

## Looking at an app, to see what an installer of it would hold

app-scan-getting-ready = Підготовка до сканування цієї програми…
app-scan-install-record = Визначення способу інсталяції…
app-scan-near-matches = Пошук того, що також може бути її частиною…
app-scan-system-items = Пошук її служб, завдань і ярликів…
app-scan-files = Перегляд її файлів…
app-scan-registry = Читання того, що пов'язує її з Windows…
app-scan-finishing = Завершення…
app-scan-look-again = Сканувати знову
app-scan-stop = Зупинити
app-scan-running-hint = Поки триває сканування, можна вибрати іншу програму або змінити, що включати.
# { $error } is why, in words that follow a colon.
app-scan-failed = Не вдалося просканувати цю програму: { $error }
app-scan-try-again = Повторити спробу
app-scan-stopped = Зупинено. Інсталятор можна буде створити, коли цю програму буде проскановано.
# { $reason } is what went wrong, in the program's own words, often in English.
app-scan-stopped-unexpectedly = Сканування неочікувано зупинилося: { $reason }.

## How big an installer can be, and whether there is room for it

app-room-files = Файли
# Over how big the installer can be: up to that size.
app-room-up-to = До
app-room-free = Вільно
# { $drive } is a drive, such as C:.
app-room-free-on = Вільно на { $drive }
# How much room a drive has, when it would not say.
app-room-free-unknown = Невідомо
app-room-most-it-could-need = Максимальний потрібний обсяг до стиснення.
# { $drive } is a drive, such as C:, { $needed } how much room it needs, such as 1.2 GB, and { $free } how much it has.
app-room-drive-short = Щоб створити цей інсталятор, на { $drive } потрібно до { $needed } вільного місця, а є { $free }.
# { $drives } is one sentence like the one above for each drive without the room, one after another.
app-room-not-enough = Недостатньо місця. { $drives } Виберіть папку для збереження на іншому диску або виключіть щось.
# { $note } is a sentence, without its full stop, about a drive whose room could not be checked. Use your language's
# full stop.
app-room-could-not-tell = { $note }.

## What goes in the installer, part by part

app-footprint-heading = Що входить до інсталятора
app-footprint-working-out = Визначення того, що з цим пов'язано…
# { $error } is why.
app-footprint-would-not-make-a-package = З того, що залишилося, не можна створити пакет: { $error }
# { $list } is the names of what else is left out, such as a service and a shortcut.
app-footprint-also-left-out = Через це також не включено { $list }, оскільки того, що вони запускають або на що вказують, не буде. Їх не буде налаштовано на іншому ПК.
# { $list } is the names of the first few of what else is left out, and { $count } how many more there are.
app-footprint-also-left-out-and-more = Через це також не включено { $list } і ще { $count }, оскільки того, що вони запускають або на що вказують, не буде. Їх не буде налаштовано на іншому ПК.
# { $list } is the names of license files. The option is the one named "License and Activation Data".
app-footprint-licenses-still-go = Дані ліцензії з того, що ви виключили, усе одно включено, оскільки вони залежать від параметра «Дані ліцензії та активації»: { $list }.
# { $elapsed } is how long ago, in minutes and seconds such as 45:00, or hours, minutes, and seconds such as 1:05:00.
# "Look Again" names the button of that name.
app-footprint-looked-at-long-ago = Програму проскановано { $elapsed } тому. Файли читаються під час створення, тому зміни буде враховано, а все, що на той час зникне, буде вказано. Щоб побачити нове, натисніть «Сканувати знову».
# Headings of the parts of the list, each with a line that says what is in it.
app-footprint-program-files = Файли програми
app-footprint-data-folders = Папки даних
# What could go in with the User Data option, when nothing else is in this part. { $size } is how much, such as 1.2 GB.
app-footprint-folders-with-user-data = { $size } з параметром «Дані користувача»
# { $included } is how many folders go in, { $count } how many there are, and { $size } how much those that go in hold.
app-footprint-folders-summary =
    { $count ->
        [one] { $included } з { $count } папки, { $size }
        [few] { $included } з { $count } папок, { $size }
        [many] { $included } з { $count } папок, { $size }
       *[other] { $included } з { $count } папок, { $size }
    }
# { $size } is how much, such as 1.2 GB, and { $count } in how many files.
app-footprint-data-only-with-user-data =
    { $count ->
        [one] { $size } даних цієї програми в { $count } файлі буде включено лише з параметром «Дані користувача».
        [few] { $size } даних цієї програми в { $count } файлах буде включено лише з параметром «Дані користувача».
        [many] { $size } даних цієї програми в { $count } файлах буде включено лише з параметром «Дані користувача».
       *[other] { $size } даних цієї програми в { $count } файлах буде включено лише з параметром «Дані користувача».
    }
# A folder's row. { $count } is how many files it holds, and { $detail } where it is or why it cannot be left out.
app-footprint-folder-row =
    { $count ->
        [one] { $count } файл. { $detail }
        [few] { $count } файли. { $detail }
        [many] { $count } файлів. { $detail }
       *[other] { $count } файлів. { $detail }
    }
app-footprint-services = Служби
app-footprint-startup = Автоматичний запуск
app-footprint-tasks = Заплановані завдання
app-footprint-shortcuts = Ярлики
# { $included } is how many of the part's things go in, and { $count } how many there are.
app-footprint-items-summary = { $included } з { $count }
# { $why } is what else was left out that this goes with.
app-footprint-left-out-too = Також не включено: { $why }
# A row's detail. { $kind } is what it is, such as Service, and { $detail } what it does.
app-footprint-change-detail = { $kind }. { $detail }
app-footprint-integrations = Що пов'язує програму з Windows
# { $included } is how many kinds go in, such as file types and firewall rules, and { $count } how many kinds there are.
app-footprint-integrations-summary =
    { $count ->
        [one] { $included } з { $count } типу
        [few] { $included } з { $count } типів
        [many] { $included } з { $count } типів
       *[other] { $included } з { $count } типів
    }
app-footprint-integration-nothing-left = Нічого не залишилося через те, що ви виключили
# { $count } is how many things of this kind the installer sets up.
app-footprint-integration-things =
    { $count ->
        [one] { $count } налаштований елемент
        [few] { $count } налаштовані елементи
        [many] { $count } налаштованих елементів
       *[other] { $count } налаштованих елементів
    }
app-footprint-prerequisites = Що потрібно для запуску
# { $count } is how many runtimes, such as Visual C++ Redistributable.
app-footprint-prerequisites-summary =
    { $count ->
        [one] { $count } середовище виконання
        [few] { $count } середовища виконання
        [many] { $count } середовищ виконання
       *[other] { $count } середовищ виконання
    }
app-footprint-near-matches = Також може бути частиною програми
# { $ticked } is how many were ticked, and { $count } how many there are.
app-footprint-near-matches-summary = Додано { $ticked } з { $count }
app-footprint-near-matches-hint = Не включаються до інсталятора, якщо їх не позначити. Якщо позначити елемент, програму буде проскановано знову з тими самими перевірками, що й решту, тож те, що схоже на пароль, усе одно не потрапить до інсталятора.
# { $why } is why it might be part of the app, as a sentence.
app-footprint-near-match-with-user-data = Включається з параметром «Дані користувача». { $why }
# How much a folder holds, when not all of it could be measured. { $size } is how much was, such as 1.2 GB.
app-footprint-near-match-over = понад { $size }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-folder = Папка: { $place }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-hidden = Прихована частина програми: { $place }
app-footprint-nothing-left-out = Нічого не виключено через схожість на пароль, ліцензію чи сертифікат.
app-footprint-left-out = Що не включено і чому
# { $count } is how many things were left out.
app-footprint-left-out-summary =
    { $count ->
        [one] { $count } елемент
        [few] { $count } елементи
        [many] { $count } елементів
       *[other] { $count } елементів
    }
# Above a list of things left out for one reason. { $title } is the reason.
app-footprint-left-out-group = { $title }:
app-footprint-refused = Жодному інсталятору не дозволено налаштовувати таке:
# A heading over a list that opens and closes. { $count } is how many are in it.
app-footprint-worth-knowing = Варто знати ({ $count })
# { $list } is what kinds of things were looked for and none found, from the five below.
app-none-found = Не знайдено { $list }.
app-none-found-services = служб
app-none-found-startup = елементів автозавантаження
app-none-found-tasks = запланованих завдань
app-none-found-shortcuts = ярликів
app-none-found-integrations = елементів, що пов'язують програму з Windows

## Network PC: reading another PC through WinAppMigrator Client, in three steps. "WinAppMigrator Client" is the name of a
## program, and the old PC is the one the apps are moved off.

app-network-step-client = 1. Скопіюйте клієнт на старий ПК
app-network-step-client-hint = WinAppMigrator Client запускається на ПК, де зараз розміщена програма (Windows 7 або новіша). Він дозволяє читати дані лише цьому ПК — у зашифрованому вигляді та за кодом, який показує.
# { $drive } is a USB drive, such as E:\.
app-network-save-to-usb = Зберегти на USB-накопичувач { $drive }
# { $drive } is a USB drive, such as E:\, and { $name } the name it was given, such as BACKUP.
app-network-save-to-named-usb = Зберегти на USB-накопичувач { $drive } ({ $name })
app-network-save-to-folder = Зберегти в папку…
app-network-look-for-usb-again = Знову шукати USB-накопичувачі
app-network-no-usb = USB-накопичувач не підключено. Підключіть його або збережіть клієнт у папку, доступну зі старого ПК, наприклад у спільну мережеву папку.
# { $path } is where the client was saved, such as E:\WinAppMigrator Client.exe.
app-network-client-saved = Збережено як { $path }. На старому ПК закрийте програму, яку переносите, потім двічі клацніть цей файл і дозвольте йому вносити зміни. Він покаже код.
app-network-step-pick = 2. Виберіть старий ПК
app-network-looking = Пошук у мережі ПК, на яких запущено клієнт…
app-network-type-name = Або введіть його ім'я чи адресу
# An example, shown in the empty field: a PC's name, or its network address.
app-network-type-name-example = OLD-PC або 192.168.1.20
app-network-step-code = 3. Введіть код, який він показує
app-network-code = Код зв'язування
app-network-connect = Підключитися
app-network-pairing = Зв'язування…
# Why pairing did not work, shown on its own.
app-network-pairing-stopped = зв'язування зупинилося, не завершившись
app-network-wrong-code = це не той код, який показує старий ПК. Перевірте його та повторіть спробу.
# { $error } is what went wrong.
app-network-not-reached = не вдалося підключитися до старого ПК: { $error }
app-network-pick-folder-title = Виберіть папку для WinAppMigrator Client
# { $path } is the folder, and { $error } what Windows said went wrong.
app-client-could-not-save = не вдалося зберегти клієнт у { $path }: { $error }
# scripts\build-client.ps1 is a file name.
app-client-not-carried = ця збірка WinAppMigrator не містить клієнта. Зберіть його за допомогою scripts\build-client.ps1, а потім знову зберіть програму.
# { $error } is what Windows said went wrong.
app-client-could-not-save-note = не вдалося зберегти примітку поруч із ним: { $error }
# A note saved as a text file beside the client, for whoever runs it on the old PC. Keep its lines short, as it is read in
# Notepad. "WinAppMigrator Client.exe" is a file name. "Network PC..." names the Scan Source button: use its words in your
# language.
app-client-note =
    WinAppMigrator Client
    
    Запустіть цей файл на ПК, з якого переносите програми:
    Windows 7, 8, 10 або 11.
    
    1. Закрийте програму, яку переносите.
    2. Двічі клацніть «WinAppMigrator Client.exe» і дозвольте йому
       вносити зміни.
    3. Він покаже 6-значний код зв'язування. У WinAppMigrator на новому ПК
       виберіть «Мережевий ПК...», виберіть цей ПК і введіть код.
    4. Коли на новому ПК усе буде готово, натисніть Enter у вікні клієнта
       або закрийте його.
    
    Клієнт лише читає дані. Поки він працює, він зберігає знімок диска
    Windows і відкриває у брандмауері Windows два свої порти для вашої
    локальної мережі, а коли зупиняється, видаляє знімок і закриває
    порти. Усе, що передається мережею, зашифровано, і прочитати щось
    може лише ПК, який має код.
