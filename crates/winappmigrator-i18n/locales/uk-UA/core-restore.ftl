# Words the core writes for people as it installs an app from a package: why an install will not start, the list of
# what it did (or for a preview, what it would do) that the installer window shows, its warnings, the record it keeps of
# each install so that it can be undone, and the core's errors. Message names start with core-.
#
# The list of what an install did is short and plain, one line for each thing, as the installer window lists them:
# "copied", "restored", "would copy". Words such as --force and --apply are command-line options, and LocalService,
# SYSTEM, ProgramData, and %AppData% are as Windows names them: keep them as they are.


## Errors: what went wrong, when the core cannot go on. Each is shown on its own, or after words that say what was being
## done.

core-error-list-changed = дії цієї інсталяції змінилися після того, як їх було погоджено, тому нічого не інстальовано
# { $path } is a file or folder, such as C:\Program Files\App\app.exe, or words that name one, such as "the installer".
# { $error } is what Windows said went wrong.
core-error-io = помилка введення-виведення — { $path }: { $error }
# { $detail } is what went wrong.
core-error-registry = не вдалося прочитати реєстр: { $detail }
# { $path } is a program file. { $detail } is what was wrong with it.
core-error-pe = не вдалося проаналізувати PE-файл { $path }: { $detail }
# { $detail } is what was wrong, already in words.
core-error-invalid-input = неприпустимі вхідні дані: { $detail }
# { $detail } is what went wrong.
core-error-serialization = не вдалося серіалізувати звіт: { $detail }
# { $detail } is what went wrong.
core-error-archive = помилка операції з архівом: { $detail }
# { $detail } is what went wrong.
core-error-network = помилка мережевої операції: { $detail }
# { $detail } is what went wrong.
core-error-time-format = не вдалося відформатувати час: { $detail }
core-error-cancelled = операцію скасовано
# What is said when part of the program stopped unexpectedly and gave no reason.
core-error-panic-no-reason = причину не вказано

## The record of each install, kept so that it can be undone.

# { $kind } is a number that says what type of registry value it is.
core-journal-unknown-value-type = невідомий тип параметра реєстру: { $kind }
core-journal-not-elevated = без підвищених прав
core-journal-needs-admin = для інсталяції потрібні права адміністратора, щоб її запис зберігався там, де інші не можуть його змінити
# { $path } is a folder.
core-journal-not-an-install-folder = це не папка інсталяції: { $path }
# { $path } is the file the record is kept in.
core-journal-not-written = не вдалося зберегти запис цієї інсталяції: { $path }
# { $path } is a folder.
core-journal-path-escapes-root = шлях виходить за межі призначеної кореневої папки: { $path }
# { $path } is a folder.
core-journal-nothing-exists = нічого з цього немає на цьому ПК: { $path }
core-journal-busy = виконується інша інсталяція або видалення. Дочекайтеся завершення й повторіть спробу
# Said of a file that was to get back the permissions it had.
core-journal-no-folder = у нього немає папки
# Said of the permissions a file had, which were kept to give back to it.
core-journal-permissions-unreadable = не вдалося прочитати збережене
# { $folder } is the folder of one install's record.
core-journal-unreadable-empty = запис інсталяції в { $folder } не можна використати: він порожній
# { $folder } is the folder of one install's record.
core-journal-unreadable-no-header = запис інсталяції в { $folder } не можна використати: він не починається з опису того, чого стосується
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-version = запис інсталяції в { $folder } не можна використати: його записала інша версія цієї програми
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-install = запис інсталяції в { $folder } не можна використати: у ньому вказано, що він належить іншій інсталяції
# { $id } is an install's id, such as wam-1a2b3c.
core-journal-not-an-install-id = це не ідентифікатор інсталяції: { $id }

## Why an install will not start, or stopped before it changed anything.

core-restore-apply-and-dry-run = не можна вказувати --apply і --dry-run одночасно
# { $path } is the folder that was chosen to install in.
core-restore-location-not-full-path = розташування інсталяції має бути повним шляхом: «{ $path }»
# { $problem } says what is wrong with the folder, already in words. { $path } is the folder.
core-restore-location-unusable = це розташування інсталяції не можна використати: { $problem }: «{ $path }»
core-restore-needs-admin-plan = план відновлення містить зміни реєстру, служб, запланованих завдань, ярликів або захищених розташувань, для яких потрібні права адміністратора
# { $path } is the folder that was chosen to install in.
core-restore-needs-admin-target = для цільової кореневої папки потрібні підвищені права: { $path }
core-restore-needs-admin-user-data = для відновлення даних користувача в ProgramData або інші захищені розташування потрібні підвищені права
# { $error } is why the install stopped. { $failed } is what could not be put back, as sentences of their own.
core-restore-not-all-put-back = { $error }. Не все зроблене вдалося повернути: { $failed }
# { $error } is what went wrong.
core-restore-nowhere-to-keep-record = немає безпечного місця для запису, за яким цю інсталяцію можна було б скасувати, тому нічого не інстальовано: { $error }
# { $path } is a file the app would have put there.
core-restore-already-exists = { $path } уже існує, тому нічого не інстальовано. Щоб замінити його, використайте --force
# { $root } is a folder the app has files for. { $link } is that folder, or one it is in.
core-restore-root-under-link = ця програма має дані для відновлення в { $root }, а { $link } — це посилання на інше місце, яке, можливо, можуть змінити інші, тому нічого не інстальовано. Якщо туди потрапляють дані користувача або дані ліцензії, інсталюйте програму без них. Якщо це ярлик у папці «Автозавантаження», поверніть цю папку на її попереднє місце
core-restore-hash-check-failed = пакет не пройшов перевірку хешів, тому нічого не інстальовано
# { $detail } is the first thing found wrong, already in words.
core-restore-hash-check-failed-because = пакет не пройшов перевірку хешів, тому нічого не інстальовано: { $detail }
# { $path } is a file or folder.
core-restore-refuse-records-folder = відмовлено в записі туди, де інсталяції зберігають свої записи: { $path }
# { $path } is a file or folder.
core-restore-refuse-outside = відмовлено в записі за межами розташування інсталяції та відомих папок даних: { $path }
# { $path } is a file.
core-restore-no-parent-folder = місце призначення не має батьківської папки: { $path }
# { $path } is where a file was to go.
core-restore-folder-in-the-way = там, куди має потрапити цей файл, є папка: { $path }
# { $path } is a file in the package.
core-restore-file-changed = { $path } уже не відповідає вмісту пакета, тому його не інстальовано
# { $path } is a file in the package.
core-restore-source-is-link = { $path } — це посилання, за яким не можна переходити
# { $path } is a file in the package.
core-restore-source-became-link = { $path } перетворився на посилання під час відкриття
# Why a service did not start, after "did not start:".
core-restore-service-stopped-by-itself = вона знову зупинилася сама
# Why part of the app was left out, after "was left out:".
core-restore-com-class-someone-elses = цей клас COM уже є на цьому ПК і належить іншому компоненту

## What an install did, one line for each thing, or for a preview, what it would do. { $source } is a file in the
## package, and { $destination } and { $path } are where it goes on this PC. { $key } is a registry key, such as
## HKLM\Software\App.

# { $package_valid } is true or false, and { $restore_valid } is true, false, or not_run: keep them as they are.
core-restore-verification-result = перевірка після відновлення package_valid={ $package_valid } restore_valid={ $restore_valid }
# { $name } is a service's name.
core-restore-started-service = запущено службу { $name }
core-restore-would-copy = буде скопійовано { $source } -> { $destination }
core-restore-copied = скопійовано { $source } -> { $destination }
core-restore-would-create-empty-folder = буде створено порожню папку { $path }
core-restore-created-empty-folder = створено порожню папку { $path }
# { $value } is the name of a value in the key.
core-restore-would-restore-registry-value = буде відновлено параметр реєстру { $key } { $value }
core-restore-would-restore-registry-key = буде відновлено розділ реєстру { $key }
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-would-add-firewall-rule = буде додано правило брандмауера { $name } для { $program }
# { $name } is a firewall rule's name.
core-restore-firewall-rule-already-there = правило брандмауера { $name } уже існує
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-added-firewall-rule = додано правило брандмауера { $name } для { $program }
# { $name } is a service's name, { $command } what it runs, and { $account } the account it runs as, such as LocalService.
core-restore-would-restore-service = буде відновлено службу { $name } -> { $command } (запускається від імені { $account })
# { $name } is a runtime, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-restore-prerequisite-present = необхідний компонент уже є: { $name }
# { $name } is a runtime that Windows has as a Windows feature.
core-restore-would-install-prerequisite-feature = буде інстальовано необхідний компонент { $name } (компонент Windows)
# { $name } is a runtime whose installer is in the package.
core-restore-would-install-prerequisite-bundled = буде інстальовано необхідний компонент { $name } (вбудований інсталятор)
# { $name } is a runtime that would come through winget, which is a program's name.
core-restore-would-install-prerequisite-winget = буде інстальовано необхідний компонент { $name } (winget)
# { $name } is a runtime.
core-restore-installed-prerequisite = інстальовано необхідний компонент { $name }
# { $name } is a runtime.
core-restore-installed-prerequisite-reboot = інстальовано необхідний компонент { $name } (потрібне перезавантаження)
core-restore-would-restore-license = буде відновлено ліцензію { $source } -> { $destination }
core-restore-restored-license = відновлено ліцензію { $source } -> { $destination }
core-restore-would-restore-license-registry = буде відновлено розділ реєстру ліцензії { $key }
# { $name } is a driver's name.
core-restore-driver-not-restored = драйвер не відновлено автоматично: { $name }
# { $name } is a driver's name.
core-restore-would-install-driver = буде інстальовано драйвер { $name } -> { $destination }
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-would-restore-task = буде відновлено заплановане завдання { $name }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-startup-shortcut = буде відновлено ярлик автозавантаження { $shortcut } -> { $target }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-shortcut = буде відновлено ярлик { $shortcut } -> { $target }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-would-restore-variable =
    { $scope ->
        [user] буде відновлено змінну середовища користувача { $name }
       *[machine] буде відновлено системну змінну середовища { $name }
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-left-variable =
    { $scope ->
        [user] змінну середовища користувача { $name } залишено без змін: жодна її частина не належить цій програмі
       *[machine] системну змінну середовища { $name } залишено без змін: жодна її частина не належить цій програмі
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-restored-variable =
    { $scope ->
        [user] відновлено змінну середовища користувача { $name }
       *[machine] відновлено системну змінну середовища { $name }
    }
core-restore-set-uninstaller = налаштовано програму видалення для { $key }
core-restore-placed-driver-image = розміщено образ драйвера { $source } -> { $destination }
# { $name } is a driver's name.
core-restore-configured-driver = налаштовано драйвер { $name }
# { $name } is a driver's name.
core-restore-created-driver = створено драйвер { $name }
# { $name } is a driver's name.
core-restore-started-driver = запущено драйвер { $name }
core-restore-ensured-registry-key = забезпечено наявність розділу реєстру { $key }
# { $value } is the name of a value in the key.
core-restore-restored-registry-value = відновлено параметр реєстру { $key } { $value }
# "(Default)" is how Windows names the value a key has with no name: write it as Registry Editor does.
core-restore-restored-registry-default = відновлено параметр реєстру { $key } (За замовчуванням)
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-configured-service = налаштовано службу { $name } (запускається від імені { $account })
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-created-service = створено службу { $name } (запускається від імені { $account })
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-restored-task = відновлено заплановане завдання { $name }
core-restore-restored-shortcut = відновлено ярлик { $path }

## Warnings an install gives, and why it failed. They are shown after it, on the PC it ran on.

core-warning-restore-earlier-not-trusted = Запис незавершеної інсталяції не використано, оскільки його можуть змінити інші. Усі зміни, внесені тією інсталяцією, залишилися.
# { $app_name } is the app's name. { $count } is how many changes were put back.
core-warning-restore-earlier-undone =
    { $count ->
        [one] Попередню інсталяцію програми { $app_name } не було завершено, тому її спершу скасовано. Повернуто { $count } зміну.
        [few] Попередню інсталяцію програми { $app_name } не було завершено, тому її спершу скасовано. Повернуто { $count } зміни.
        [many] Попередню інсталяцію програми { $app_name } не було завершено, тому її спершу скасовано. Повернуто { $count } змін.
       *[other] Попередню інсталяцію програми { $app_name } не було завершено, тому її спершу скасовано. Повернуто { $count } змін.
    }
# { $app_name } is the app's name.
core-warning-restore-earlier-never-finished = Попередню інсталяцію програми { $app_name } не було завершено. Під час інсталяції її буде спершу скасовано.
core-warning-restore-user-data-real-locations = Дані користувача відновлюються у фактичні розташування %AppData%/%ProgramData% цього комп'ютера, а не в цільову кореневу папку.
# { $error } is what went wrong.
core-warning-restore-commit-failed = Не вдалося завершити запис цієї інсталяції: { $error }
# { $name } is a service's name.
core-warning-restore-service-still-starting = Службі { $name } надіслано команду запуску, але вона ще не завершила запуск. Можливо, вона ще запуститься. Також вона запуститься автоматично під час наступного запуску цього ПК.
# { $name } is a service's name. { $reason } is why it did not start, as Windows said it.
core-warning-restore-service-not-started = Службу { $name } інстальовано, але вона не запустилася: { $reason }. Доки вона не запуститься, програма може не працювати. Windows спробує запустити її знову під час наступного запуску цього ПК.
# { $name } is a service's name. { $reason } is why it could not be stopped.
core-warning-restore-service-not-stopped = Не вдалося спершу зупинити службу { $name }: { $reason }. До наступного запуску цього ПК вона може й далі виконувати стару програму.
core-warning-restore-install-cancelled = Інсталяцію скасовано.
core-warning-restore-preview-cancelled = Попередній перегляд скасовано.
# { $item } is what was left out, such as a file or a service's name. { $reason } is why, already in words.
core-warning-restore-left-out = { $item } не включено: { $reason }.
# { $error } is what went wrong.
core-warning-restore-verification-failed = Не вдалося виконати перевірку після відновлення: { $error }
# { $runner } says what relies on the file: a service, a scheduled task, a firewall rule, a shortcut, or a registry value.
# { $path } is the file. { $name } is the name of what relies on it. { $refusal } is why the file was not replaced,
# already in words.
core-warning-restore-program-would-run =
    { $runner ->
        [service] Цей пакет не може замінювати { $path }, а від нього залежить служба { $name }, тому нічого не інстальовано: { $refusal }
        [task] Цей пакет не може замінювати { $path }, а від нього залежить заплановане завдання { $name }, тому нічого не інстальовано: { $refusal }
        [firewall] Цей пакет не може замінювати { $path }, а від нього залежить правило брандмауера { $name }, тому нічого не інстальовано: { $refusal }
        [shortcut] Цей пакет не може замінювати { $path }, а від нього залежить ярлик { $name }, тому нічого не інстальовано: { $refusal }
       *[registry] Цей пакет не може замінювати { $path }, а від нього залежить параметр реєстру { $name }, тому нічого не інстальовано: { $refusal }
    }
core-warning-restore-destination-exists = Місце призначення вже існує; щоб перезаписати його, використайте --force.
# { $path } is a file. { $error } is what went wrong.
core-warning-restore-file-not-written = Не вдалося записати { $path }: { $error }
core-warning-restore-not-a-folder = Там, де має бути ця порожня папка, міститься щось інше.
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-not-looked-at = Не вдалося перевірити { $path }: { $error }
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-folder-not-made = Не вдалося створити { $path }: { $error }
# Why part of the app was left out, after "was left out:".
core-warning-restore-registration-exists = на цьому ПК це вже є з іншим значенням, і інсталяція не перебирає його на себе
core-warning-restore-registry-review = Параметри реєстру відновлено з архівованих структурованих даних, а відомі шляхи програми замінено на цільову кореневу папку.
# { $error } is what went wrong.
core-warning-restore-firewall-unreadable = Не вдалося прочитати правила брандмауера Windows: { $error }
# Why a firewall rule was left out, after "was left out:".
core-warning-restore-firewall-rule-exists = правило з такою назвою для цієї програми вже існує з іншими параметрами, а інсталяція їх не змінює
# { $name } is a runtime.
core-warning-restore-prerequisite-unknown = { $name }: для цього середовища виконання немає відомого інсталятора.
# { $name } is a runtime.
core-warning-restore-prerequisite-needs-admin = Для інсталяції { $name } потрібні підвищені права; запустіть інсталятор знову від імені адміністратора.
# { $name } is a runtime.
core-warning-restore-prerequisite-reboot = { $name } інстальовано, але для завершення потрібно перезавантажити комп'ютер.
# { $error } is what went wrong.
core-warning-restore-skipped-not-recorded = Не вдалося записати відомості про пропущене середовище виконання: { $error }
# { $name } is a runtime. { $reason } is why it was skipped, already in words.
core-warning-restore-prerequisite-skipped = { $name }: { $reason }
# { $name } is a runtime. { $reason } is why it could not be installed. { $url } is a web address to get it from.
core-warning-restore-prerequisite-failed = Не вдалося інсталювати { $name }: { $reason }. Завантажте його звідси: { $url }.
# { $error } is what went wrong.
core-warning-restore-runtime-not-recorded = Не вдалося записати відомості про інстальоване середовище виконання: { $error }
# { $error } is what went wrong.
core-warning-restore-license-source-invalid = Неприпустимий шлях до ліцензії в пакеті: { $error }
core-warning-restore-license-needs-admin = Для відновлення даних ліцензії в захищене розташування потрібні підвищені права.
core-warning-restore-license-exists = Місце призначення ліцензії вже існує; щоб перезаписати його, використайте --force.
# { $error } is what went wrong.
core-warning-restore-license-failed = Не вдалося відновити дані ліцензії: { $error }
core-warning-restore-driver-needs-admin = Для реєстрації драйвера потрібні підвищені права; запустіть інсталятор знову від імені адміністратора.
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-warning-restore-task-runs-as-system = Заплановане завдання { $name } виконується від імені SYSTEM з повними правами на цьому ПК.
core-warning-restore-shortcut-exists = Ярлик уже існує; щоб перезаписати його, використайте --force.
core-warning-restore-shortcut-all-users = Один або кілька відновлених ярликів указують на загальний робочий стіл або загальне меню «Пуск».
core-warning-restore-environment-refresh = Щоб зміни змінних середовища набули сили, може знадобитися новий процес, нова оболонка або повторний вхід у систему.
# { $name } is a service's name. { $account } is the account it ran as on the PC it came from.
core-warning-restore-service-account = На ПК, з якого її перенесено, служба { $name } працювала від імені { $account }. Її пароля немає в інсталяторі, тому службу налаштовано для роботи від імені LocalService. Якщо вона не працюватиме, укажіть її обліковий запис і пароль у «Службах».
# { $name } is a runtime.
core-warning-restore-prerequisite-pending-reboot = { $name } ще не зареєстровано; цей компонент має запрацювати після перезавантаження.
# { $name } is a runtime.
core-warning-restore-prerequisite-not-verified = { $name } повідомив про успішне завершення, але не зареєструвався; можливо, інсталяція не вдалася.
# { $error } is what went wrong.
core-warning-restore-driver-source-invalid = Неприпустимий шлях до вбудованого драйвера: { $error }
# { $error } is what went wrong.
core-warning-restore-driver-not-read = Не вдалося прочитати поточні параметри драйвера: { $error }
core-warning-restore-driver-exists = Служба драйвера вже існує; щоб налаштувати її повторно, використайте --force.
core-warning-restore-driver-image-exists = Образ драйвера вже існує; щоб перезаписати його, використайте --force.
core-warning-restore-driver-hash-bundled = Вбудований образ драйвера не відповідає записаному хешу, тому його не інстальовано.
# { $error } is what went wrong.
core-warning-restore-driver-not-placed = Не вдалося розмістити образ драйвера: { $error }
core-warning-restore-driver-hash-placed = Образ драйвера змінився після розміщення, тому його не зареєстровано.
# { $error } is what went wrong.
core-warning-restore-key-not-created = Не вдалося створити розділ реєстру: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-read = Не вдалося прочитати значення, яке буде замінено: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-written = Не вдалося записати параметр реєстру: { $error }
core-warning-restore-value-exists = Параметр реєстру вже існує; щоб перезаписати його, використайте --force.
# { $error } is what went wrong.
core-warning-restore-environment-key-not-created = Не вдалося створити розділ реєстру для змінних середовища: { $error }
core-warning-restore-variable-exists = Змінна середовища вже існує; щоб перезаписати її, використайте --force.
# { $error } is what went wrong.
core-warning-restore-variable-not-written = Не вдалося записати змінну середовища: { $error }
core-warning-restore-value-kind-missing = Не вказано тип параметра реєстру.
# { $error } is what went wrong.
core-warning-restore-service-not-read = Не вдалося прочитати поточні параметри служби: { $error }
core-warning-restore-service-exists = Служба вже існує; щоб налаштувати її повторно, використайте --force.
# { $error } is what went wrong.
core-warning-restore-task-not-queried = Не вдалося отримати відомості про заплановане завдання: { $error }
core-warning-restore-task-exists = Заплановане завдання вже існує; щоб перезаписати його, використайте --force.
# { $error } is what went wrong.
core-warning-restore-task-folder-not-looked-at = Не вдалося перевірити папку завдання: { $error }
# { $error } is what went wrong.
core-warning-restore-shortcut-not-put-back = Не вдалося створити ярлик і повернути той, який він замінив: { $error }
# { $error } is what went wrong.
core-warning-restore-command-not-started = Не вдалося запустити команду: { $error }
# { $output } is what the command wrote, as it wrote it.
core-warning-restore-command-failed = Помилка виконання команди: { $output }
# { $error } is what went wrong.
core-warning-restore-powershell-not-started = Не вдалося запустити PowerShell: { $error }
