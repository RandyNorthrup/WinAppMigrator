# Words the core writes for people while it builds a package from an installed app: what a package records for an
# installer to show later, errors shown in the app, the list of what goes in and what is left out, opening a disk
# image or another PC over the network, and what an installer says when the package it carries cannot be read.
# Message names start with core-.


## Scanning an app, before a package is built. Shown in the app when a scan cannot go on.

# { $path } is the app's program, such as D:\Program Files\Acme\Acme.exe, on a drive taken from another PC.
core-scan-program-not-on-the-drive = Шлях { $path } проходить через посилання або веде не на підключений диск, тому файл не прочитано.
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-not-on-this-pc = Файл { $path } розміщено не на диску цього ПК, або він має ім'я, яке відкрило б щось інше, тому його не прочитано.
# { $path } is the app's program.
core-scan-program-someone-elses = Файл { $path } розміщено в папках іншого облікового запису або він не належить цьому обліковому запису, а власний запис цієї програми не вказує, що він має бути там, тому його не прочитано.
# { $app_name } is the app's name, as the list of apps calls it.
core-scan-no-executable = не вдалося знайти виконуваний файл для { $app_name }. Драйвер, середовище виконання або надбудова можуть не мати власного виконуваного файлу, який можна запакувати.
# { $path } is the file that was picked.
core-scan-not-a-file = об'єкт сканування не є файлом: { $path }
# Names something left out of a package, in a list of what was left out and why. { $name } is the variable's name,
# such as ACME_LICENSE_KEY.
core-scan-environment-variable-item = Змінна середовища { $name }

## Folders and parts that might also be the app's, offered in a list to tick. Each says why it is offered.

core-near-starts-with-app-name = Назва починається з назви цієї програми.
core-near-has-app-name = Назва містить назву цієї програми.
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-starts-with-publisher = Назва починається з назви видавця — { $publisher }.
# { $folder } is a folder's name, such as Program Files.
core-near-hidden-part-beside = Частина, прихована зі списку програм та інстальована поруч із цією програмою в { $folder }.
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-hidden-part-publisher = Частина, прихована зі списку програм, від того самого видавця — { $publisher }.

## What a scan found, as a person chooses from it: the folders that go in, and what is left out, and why

# Why a folder cannot be left out, beside its number of files.
core-footprint-locked-app-folder = Завжди включаються разом з основним виконуваним файлом
core-footprint-locked-main-program = Містить основний виконуваний файл, який включається завжди
core-footprint-locked-needed = Містить те, без чого основний виконуваний файл не запуститься
core-footprint-locked-font = Включається разом зі своїм шрифтом у розділі «Що пов'язує програму з Windows»
# Where a folder is. "AppData" and "ProgramData" are the names of Windows folders; keep them as they are.
core-footprint-place-roaming = Параметри у вашому профілі (AppData)
core-footprint-place-local = Параметри для цього ПК у вашому профілі (AppData)
core-footprint-place-program-data = Спільні для всіх користувачів ПК (ProgramData)
core-footprint-place-documents = У папці «Документи»
core-footprint-place-saved-games = У папці «Збережені ігри»
core-footprint-place-public-documents = У папці «Загальні документи»
core-footprint-place-app-folder = У папці програми
# { $folder } is a folder, such as C:\Users\Sam\AppData\Roaming.
core-footprint-place-in = У { $folder }
# A row for the files directly in the app's folder. { $folder } is that folder's name, such as Acme.
core-footprint-files-directly-in = Файли безпосередньо в { $folder }
# Headings over what is left out, and why each item is.
core-footprint-licenses-left-out = Не включено до цього інсталятора, оскільки параметр «Дані ліцензії та активації» вимкнуто
core-footprint-font-stays-behind = Залишається на цьому ПК.
core-footprint-drivers-left-out = Драйвери, що належать Windows або постачальнику й не можуть передаватися, тому інсталюйте їх на іншому ПК повторно
core-footprint-driver-reinstall = Інсталюйте його повторно з власного пакета постачальника.
# Why something else is left out along with what the person left out. { $service } is a service's name, such as AcmeSync.
core-footprint-needs-left-out = потребує служби { $service }, яку не включено
core-footprint-told-to-use-left-out = налаштовано використовувати щось із папки, яку ви виключили
core-footprint-file-in-left-out = файл розміщено в папці, яку ви виключили
core-footprint-program-in-left-out = програма, для якої це призначено, розміщена в папці, яку ви виключили
core-footprint-names-left-out = вказує на папку, яку ви виключили
# Why something is left out, as the installer tells whoever installs it.
core-footprint-builder-left-it-out = той, хто створив цей інсталятор, не включив це
# Something the person left out, by kind and name, such as "Service Acme Updater". { $kind } is the kind, such as
# Service or Scheduled Task; { $title } is its name.
core-footprint-left-out-item = { $kind } { $title }

## Building a package and an installer from a scan. Shown in the app when a build cannot go on.

core-package-scan-of-another-app = це сканування стосується іншої програми, ніж та, для якої створюється інсталятор, тому нічого не створено
core-package-scan-brought-in-more = це сканування додало дані, яких не запитувало це створення, тому нічого не створено
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-package-program-gone = { $path } більше не існує, тому нічого не створено. Проскануйте програму знову.
core-package-needs-stub = для саморозпакувального архіву потрібна заглушка інсталятора
# { $reason } is what went wrong.
core-package-staged-file-failed = не вдалося додати файл із проміжної папки до пакета: { $reason }
# { $shortfalls } says which drives are short of room and by how much, in sentences of its own.
core-package-not-enough-room = недостатньо місця для створення цього інсталятора, тому нічого не прочитано й не записано. { $shortfalls }. Звільніть там місце або виберіть папку для збереження на іншому диску.
# { $path } is a file's path, on a drive taken from another PC.
core-package-not-read-off-the-drive = { $path } проходить через посилання або веде не на підключений диск, тому його не прочитано
# { $path } is a file's path.
core-package-not-read-not-local = { $path } розміщено не на диску цього ПК, тому його не прочитано
# { $path } is a file's path.
core-package-not-read-other-account = { $path } веде до папок іншого облікового запису, тому його не прочитано
# { $path } is a file's path, as the PC it came from called it.
core-package-not-on-the-drive = { $path } немає на підключеному диску, тому його не прочитано
# { $path } is a file's path.
core-package-backup-copy-no-parent = неможливо створити резервну копію файлу без батьківської папки: { $path }
# { $path } is a file's path.
core-package-backup-copy-destination-no-parent = місце призначення резервної копії не має батьківської папки: { $path }
# robocopy is the name of a Windows command; keep it as it is. { $code } is how it ended, such as Some(8).
core-package-robocopy-failed = помилка копіювання robocopy в режимі резервного копіювання (код виходу { $code })
# Stands for a folder where a file's name would be, as in "I/O failed for temporary package folder: …".
core-package-temporary-folder = тимчасова папка пакета
# Stands for the installer being written where a file's name would be, as in "I/O failed for the installer: …".
core-package-the-installer = інсталятор
# { $path } is a file's path, or its name in the package. { $error } is what went wrong with it.
core-package-path-error = { $path }: { $error }
# { $name } is a file's name in the package, such as app/bin/App.exe.
core-package-same-place = інший файл має те саме місце в пакеті: { $name }
# { $name } is a file's name in the package.
core-package-changed-in-protected-folder = { $name } змінився в захищеній папці під час створення пакета
core-package-worker-died = потік стиснення файлів неочікувано зупинився, тому інсталятор не записано
# { $path } is a file's path.
core-package-link-not-read = { $path } — це посилання, тому його не прочитано
# { $path } is a file's path. { $error } is what went wrong.
core-package-not-compressed = не вдалося стиснути { $path }: { $error }
# { $path } is a file's path. { $reason } is what went wrong.
core-package-not-read-back = не вдалося повторно прочитати { $path }: { $reason }

## Where an app's files are, and where an installer may write them. Shown when a path is refused.

# { $path } is a folder, such as C:\Program Files\Acme.
core-discovery-root-not-full-path = папку, у якій виконується запис, має бути вказано повним шляхом: { $path }
# { $path } is a folder's path.
core-discovery-root-not-a-folder = шлях, у якому виконується запис, не є папкою: { $path }
# { $path } is a file's path.
core-discovery-escapes-root = шлях виходить за межі призначеної кореневої папки: { $path }
# { $path } is the link's path.
core-discovery-refusing-to-write-through-link = відмовлено в записі через посилання: { $path }
# { $path } is the link's path.
core-discovery-refusing-link = відмовлено в переході за посиланням: { $path }
# { $error } is what Windows said went wrong.
core-discovery-refusing-link-because = відмовлено в переході за посиланням: { $error }
# { $path } is the link's path.
core-discovery-link-to-somewhere-else = { $path } — це посилання на інше місце, і, можливо, інші можуть змінити, куди воно веде. Використайте місце, на яке воно вказує
# { $path } is a folder's path.
core-discovery-no-parent-folder = { $path } не має батьківської папки
# { $path } is the install location as it was given. Keep the quotes, or use your language's own.
core-discovery-install-location-not-full-path = розташування інсталяції має бути повним шляхом: «{ $path }»
# { $path } is the install location.
core-discovery-install-location-a-link = розташування інсталяції — це посилання на інше місце: { $path }
# { $path } is a file's path.
core-discovery-device-name = шлях використовує ім'я, зарезервоване для пристрою Windows: { $path }
# { $path } is a file's path.
core-discovery-device-name-on-older-windows = { $path } має ім'я, яке ця версія Windows відкриває як пристрій, а не як файл, тому в нього нічого не записано. Цю програму можна інсталювати в Windows 11
# { $path } is a file's path.
core-discovery-unsafe-path = небезпечний шлях: { $path }
# { $path } is a file's path.
core-discovery-no-package-path = файл не має відносного шляху в пакеті: { $path }

## Writing an installer, and reading the package inside one. Most of these are shown by an installer, on the PC it
## runs on, when it cannot read what it carries. "The package" is the app's files and settings that an installer
## carries; "the stub" is the program part of an installer, before the package; "the data file" is the .wamdata file
## that holds the package of a very large installer, beside it.

# Stands for the package where a file's name would be, as in "I/O failed for installer package: …".
core-archive-installer-package = пакет інсталятора
# Stands for the installer program built into WinAppMigrator, where a file's name would be.
core-archive-embedded-stub = вбудована заглушка інсталятора
# { $index } is the number of a file in the package. { $reason } is what went wrong.
core-archive-entry-unreadable = не вдалося прочитати елемент { $index } пакета: { $reason }
# { $name } is a file's name inside the package, such as app/bin/App.exe.
core-archive-entry-unsafe-path = елемент zip має небезпечний шлях: { $name }
# { $name } is a file's name inside the package.
core-archive-holds-more = { $name } містить більше, ніж заявлено
# { $name } is a file's name inside the package.
core-archive-name-too-long = це ім'я задовге для zip: { $name }
core-archive-changed-while-copied = пакет змінився під час додавання після заглушки, тому інсталятор не записано
core-archive-stub-not-a-program = частина цього інсталятора перед пакетом не є програмою
core-archive-damaged = цей інсталятор пошкоджено або змінено після створення, тому з його пакета нічого не прочитано. Скопіюйте його знову з місця, де його створено.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-missing = цей інсталятор зберігає свій пакет у файлі { $name } у тій самій папці, але цього файлу там немає. Обидва файли мають бути разом: скопіюйте сюди { $name } з місця, де створено інсталятор, і запустіть його знову.
# { $name } is the data file's name, such as App_installer.wamdata. { $error } is what Windows said went wrong.
core-archive-data-file-unopened = не вдалося відкрити файл даних цього інсталятора { $name }: { $error }. Обидва файли мають бути разом, у папці, яку може читати цей ПК.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-a-link = { $name } — це посилання або папка, а файл даних цього інсталятора має бути окремим файлом безпосередньо поруч із ним, тому з нього нічого не прочитано. Скопіюйте сюди справжній файл.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-wrong = { $name } не є файлом даних цього інсталятора або його пошкоджено чи змінено після створення, тому з нього нічого не прочитано. Знову скопіюйте обидва файли з місця, де їх створено.
core-archive-stub-signed = заглушку інсталятора підписано, але вона має залишатися непідписаною, тому підписуйте готовий інсталятор
# { $size } is a size, such as 4.2 GB.
core-archive-too-big-to-run = розмір інсталятора становив би { $size }, а Windows не може запускати виконувані файли розміром 4 ГБ або більше; нічого не записано
core-archive-outside-signature = інсталятор містить дані поза своїм підписом і, можливо, був змінений сторонніми
core-archive-footer-length = неприпустима довжина кінцевого блоку саморозпакувального архіву
core-archive-seek-outside = позиціонування за межами пакета
# { $path } is a file's path.
core-archive-unsafe-package-path = небезпечний шлях у пакеті: { $path }
# { $path } is a file's path.
core-archive-empty-package-path = порожній шлях у пакеті: { $path }

## Opening a disk image of another PC (a .vhd or .vhdx file) to read apps from it

# { $path } is the disk image file, such as D:\Backups\OldPC.vhdx.
core-image-not-attachable = { $path } не є образом диска, який Windows може підключити (.vhd або .vhdx)
core-image-needs-administrator = для підключення образу диска потрібні права адміністратора
# { $path } is the disk image file the person picked.
core-image-not-a-file = { $path } не є файлом
# { $path } is the disk image file. "Edit Disk" is the name of a Hyper-V Manager command.
core-image-differencing = { $path } — це диференційний диск, який містить лише зміни відносно диска, з якого його створено, і Windows відкрила б і той диск, хоч би де він, за його даними, був. Спершу об'єднайте його з тим диском за допомогою команди «Редагувати диск» у Hyper-V і виберіть той диск
# { $path } is the disk image file.
core-image-unreadable = { $path } не є файлом .vhd або .vhdx, який вдалося прочитати
# ProgramData is the name of a Windows folder; keep it as it is.
core-image-no-program-data = не вдалося знайти папку ProgramData на цьому ПК
# The reason given in core-image-could-not-attach when Windows already has the image open.
core-image-already-attached = образ уже підключено у Windows, і те, що його підключило, може в нього записувати. Спершу відключіть його або виберіть його диск як підключений диск
# { $path } is the disk image file. { $reason } is why, such as core-image-already-attached or what Windows said.
core-image-could-not-attach = не вдалося підключити { $path }: { $reason }
# { $path } is the disk image file.
core-image-no-windows-volume = { $path } не містить тому з Windows, який вдалося б прочитати. Том, зашифрований за допомогою BitLocker, потрібно спершу розблокувати.
# Given as the reason when Windows failed without saying anything.
core-image-windows-did-not-say = Windows не повідомила причину

## Reading another PC over the network, through the client running on it. Shown as why something on that PC
## could not be read. "The client" is WinAppMigrator Client, the small program on the other PC.

core-remote-too-slow = інший ПК задовго не відповідав
core-remote-closed = з'єднання з іншим ПК закрито
core-remote-failed = помилка з'єднання з іншим ПК
core-remote-answered-something-else = клієнт надіслав іншу відповідь
# { $name } is the PC's name or address as the person typed it, such as OLDPC or 192.168.1.20.
core-remote-not-found = { $name } не знайдено в мережі
core-remote-did-not-say-what-it-is = клієнт не повідомив, що він таке
core-remote-ordinary-names-only = запитувати можна лише шлях у межах кореневої папки іншого ПК, що складається зі звичайних імен
core-remote-more-but-none = інший ПК повідомив, що в папці є ще вміст, але нічого з нього не надіслав
# { $count } is the most things one folder may hold, such as 250,000.
core-remote-too-many-listed = інший ПК перелічив в одній папці понад { $count } елементів
core-remote-sent-too-much = інший ПК надіслав більше даних файлу, ніж запитувалося
core-remote-name-not-text = ім'я, яке не є текстом, не можна запитати
core-remote-below-root-only = запитувати можна лише шлях у межах кореневої папки іншого ПК
core-remote-file-ended-early = файл на іншому ПК закінчився передчасно
core-remote-link-not-followed = за посиланнями на іншому ПК перехід не виконується
core-remote-only-files-opened = на іншому ПК відкриваються лише файли
core-remote-listing-said-nothing = у переліку про це нічого не сказано
core-remote-change-time-unknown = час зміни невідомий
core-remote-seek-before-start = позиціонування перед початком
# { $size } is how many bytes a file is; { $limit } is the most that were expected. Both are numbers.
core-remote-too-large =
    { $size ->
        [one] його розмір — { $size } байт, а очікувалося не більше ніж { $limit }
        [few] його розмір — { $size } байти, а очікувалося не більше ніж { $limit }
        [many] його розмір — { $size } байтів, а очікувалося не більше ніж { $limit }
       *[other] його розмір — { $size } байтів, а очікувалося не більше ніж { $limit }
    }

## Checking that a download is signed by Microsoft. Each is given as the reason a check failed.

# { $error } is what Windows said went wrong.
core-signature-no-powershell = не вдалося запустити PowerShell: { $error }
# { $error } is what went wrong reading PowerShell's answer.
core-signature-unreadable = не вдалося прочитати підпис: { $error }
# { $signer } is who signed the file, such as Contoso Ltd.
core-signature-not-microsoft = підписувач — { $signer }, а не Microsoft
core-signature-not-microsoft-unnamed = його підписав хтось інший, а не Microsoft
# { $status } is Windows' own word for the signature's state, such as HashMismatch or NotSigned, which is not translated.
core-signature-status = стан його підпису: { $status }

## Warnings a package records when it is built from an app, which an installer shows later, maybe on another PC, in
## that PC's language. "This app" is the app being packaged.

core-warning-build-program-needs-unreadable = Не вдалося прочитати, що цьому виконуваному файлу потрібно від Windows, тому не буде перевірено, чи зможе він працювати на іншому ПК.
core-warning-build-external-manifest = Виявлено зовнішній маніфест програми; інтерпретацію залежностей side-by-side відкладено.
core-warning-build-non-exe = Вибраний файл не має розширення .exe; сканування PE буде продовжено.
core-warning-build-stand-in-program = Звичайного виконуваного файлу програми не знайдено, тому замість нього використовується цей файл. Відомості про архітектуру й залежності описують цей файл і можуть не відповідати програмі.
core-warning-build-other-apps-files-unknown = Не вдалося прочитати всі відомості Windows Installer про інші програми, тому папка, записана для цієї програми, може містити файли іншої програми, і тут про це нічого не буде вказано.
core-warning-build-app-folder-has-another-apps-file = За даними Windows Installer, у папці з виконуваним файлом цієї програми є файл іншої програми, і його також перенесено, оскільки береться вся папка.
# "App Paths" is the name of a Windows registry key; keep it as it is.
core-warning-build-app-paths-folder-left-out = Запис App Paths цієї програми вказує, що вона шукає файли в папці, яка їй не належить, тому цю папку не перенесено. Без її вмісту програма може не запуститися на іншому ПК.
core-warning-build-uninstaller-folder-left-out = Папку з програмою видалення цієї програми не включено, оскільки, за даними Windows Installer, там є файл іншої програми.
# { $why } is never shown: it chooses the reason. Translate each reason as the end of the sentence.
core-warning-build-recorded-folder-left-out =
    Щось, записане для цієї програми, вказує на папку, яку не включено, оскільки { $why ->
        [unreadable] її шлях виводить за межі диска або веде через посилання, тому читати її небезпечно
        [missing] її немає
        [link] вона веде через посилання
        [short-name] її записано не під справжнім іменем
        [another-account] вона розміщена в папках іншого облікового запису
        [one-account] цю програму інстальовано для одного облікового запису, а ця папка йому не належить
        [shared] вона містить спільні дані або є частиною Windows
        [generic-name] її назва не вказує, кому вона належить
        [publisher] вона має лише назву видавця й може містити інші його продукти
        [holds-the-app] вона містить власну папку цієї програми й може містити ще щось
        [another-apps-name] вона має назву іншої програми
        [another-apps-record] її запис інсталяції належить іншій програмі
        [in-another-apps-folder] вона розміщена в папці іншої програми
       *[another-apps] вона належить іншій програмі
    }.
core-warning-build-app-folder-holds-another-app = Інша програма на цьому ПК, за записами, розміщена в папці цієї програми, і її перенесено разом із нею, оскільки береться вся папка.
# "Program Files (x86)" is the name of a Windows folder; keep it as Windows calls it.
core-warning-build-from-32-bit-windows = Ця програма працювала в 32-розрядній Windows. Її розділи реєстру розміщено так, як 64-розрядна Windows зберігає розділи 32-розрядних програм, а інсталятор пропонує для неї Program Files (x86).
# { $dll } is a file's name, such as MSVCP140D.dll.
core-warning-build-debug-runtime = Одному з виконуваних файлів потрібен { $dll } — налагоджувальна збірка середовища виконання Visual C++. Microsoft не має інсталятора для таких збірок і може не дозволяти їх поширювати, тому цей файл не запуститься на ПК без Visual Studio.
# { $program } is a program's file name, such as Tool.exe.
core-warning-build-single-file-program = { $program } — це програма .NET, запакована в один файл, яка не вказує, яка версія .NET їй потрібна, тому для неї нічого не заплановано. Якщо їй потрібна .NET, вона не запуститься на ПК без неї.
# { $count } is how many of the app's programs.
core-warning-build-programs-not-inspected =
    { $count ->
        [one] { $count } виконуваний файл програми не перевірено на потрібні йому середовища виконання, оскільки їх дуже багато або вони дуже великі.
        [few] { $count } виконувані файли програми не перевірено на потрібні їм середовища виконання, оскільки їх дуже багато або вони дуже великі.
        [many] { $count } виконуваних файлів програми не перевірено на потрібні їм середовища виконання, оскільки їх дуже багато або вони дуже великі.
       *[other] { $count } виконуваних файлів програми не перевірено на потрібні їм середовища виконання, оскільки їх дуже багато або вони дуже великі.
    }
# { $runtime } is a runtime's name, such as Java Runtime Environment.
core-warning-build-runtime-no-installer = Програма потребує { $runtime }, а інсталятора для цього середовища виконання тут немає, тому нічого не запускається. Інсталюйте його на іншому ПК самостійно.
# { $why } is why something was left out, such as "its name says what it holds".
core-warning-build-left-out-of-package = Не включено до пакета: { $why }.
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-missing-dependency = Не вдалося визначити залежність: { $name }
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-dependency-only-here = { $name } знайдено лише через шлях пошуку цього ПК, і він не є частиною програми, тому його не перенесено. Без нього програма може не запуститися на іншому ПК.
core-warning-build-driver-detected = Виявлено драйвер або компонент ядра. Драйвер, що належить програмі, включається до пакета й реєструється повторно; драйвер постачальника або вбудований драйвер Windows не поширюється, і його потрібно інсталювати повторно з підписаного пакета постачальника.
core-warning-build-near-no-longer-offered = Елемент, позначений у списку того, що також може належати цій програмі, більше не входить до цього списку, тому його не додано. Перегляньте список знову.
# { $app_name } is the name of a part of the app, as the list of apps calls it. { $why } is never shown: it chooses the
# reason. Translate each reason as the end of the sentence.
core-warning-build-near-part-left-out =
    Частину того, що, за даними { $app_name }, належить їй, не включено: { $why ->
        [unreadable] її шлях виводить за межі диска або веде через посилання, тому читати її небезпечно
        [missing] її немає
        [link] вона веде через посилання
        [short-name] її записано не під справжнім іменем
        [another-account] вона розміщена в папках іншого облікового запису
        [one-account] цю програму інстальовано для одного облікового запису, а ця частина йому не належить
        [shared] вона містить спільні дані або є частиною Windows
        [generic-name] її назва не вказує, кому вона належить
        [publisher] вона має лише назву видавця й може містити інші його продукти
        [holds-the-app] вона містить власну папку цієї програми й може містити ще щось
        [another-apps-name] вона має назву іншої програми
        [another-apps-record] її запис інсталяції належить іншій програмі
        [in-another-apps-folder] вона розміщена в папці іншої програми
       *[another-apps] вона належить іншій програмі
    }.
# { $app_name } is the name of a part of the app, as the list of apps calls it.
core-warning-build-near-keys-left-out = Розділи реєстру, перелічені в записі інсталяції { $app_name }, не включено, оскільки цей запис також вказує на те, що належить іншій програмі.
# "Windows keeps what an app wrote for one account" is Windows' VirtualStore folder.
core-warning-build-virtual-store-program = У папці, де Windows зберігає те, що програма записала для одного облікового запису, був виконуваний файл. Windows ніколи не розміщує їх там, тож це зробило щось інше, і цей файл не включено.
# { $path } is a file or folder, such as C:\Users\Sam\AppData\Roaming\Acme.
core-warning-build-file-left-out = { $path } не включено: шлях веде через посилання на підключеному диску або належить іншому обліковому запису, ніж той, чиї дані включаються.
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as D:\Sam\Documents.
core-warning-build-user-folder-not-on-drive = Папка «{ $folder }» користувача { $user } розміщувалася в { $path }, а це не на цьому диску, тому її вміст не включено.
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as \\server\home\Sam\Documents.
core-warning-build-user-folder-not-on-this-pc = Папка «{ $folder }» користувача { $user } розміщувалася в { $path }, а це не на диску цього ПК, тому її вміст не включено.
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-base-unreadable = Не вдалося прочитати { $path } ({ $error }); дані програм, що там зберігаються, не включено.
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-folder-unreadable = Не вдалося прочитати { $path } ({ $error }); вкладених даних програм звідти може бракувати.
# { $error } is what went wrong.
core-warning-build-entry-unreadable = Не вдалося прочитати елемент файлової системи: { $error }
# { $count } is how many links. { $names } lists some of them. { $more } is how many more there are, which are not
# named; when it is 0, the sentence ends after the list.
core-warning-build-links-not-followed =
    { $count ->
        [one] У цій папці не виконано перехід за { $count } посиланням або ім'ям, яке відкрило б щось інше, тому те, на що воно вказує, не включено до інсталятора: { $names }
        [few] У цій папці не виконано перехід за { $count } посиланнями або іменами, які відкрили б щось інше, тому те, на що вони вказують, не включено до інсталятора: { $names }
        [many] У цій папці не виконано перехід за { $count } посиланнями або іменами, які відкрили б щось інше, тому те, на що вони вказують, не включено до інсталятора: { $names }
       *[other] У цій папці не виконано перехід за { $count } посиланнями або іменами, які відкрили б щось інше, тому те, на що вони вказують, не включено до інсталятора: { $names }
    }{ $more ->
        [0] .
        [one] { " " }та ще { $more }.
        [few] { " " }та ще { $more }.
        [many] { " " }та ще { $more }.
       *[other] { " " }та ще { $more }.
    }
# { $count } is how many folders. { $depth } is how many levels down a folder can be. { $names } lists some of them.
# { $more } is how many more there are, which are not named; when it is 0, the sentence ends after the list.
core-warning-build-folders-too-deep =
    { $count ->
        [one] { $count } папка має глибину вкладеності понад { $depth }, тому її вміст не включено до інсталятора: { $names }
        [few] { $count } папки мають глибину вкладеності понад { $depth }, тому їхній вміст не включено до інсталятора: { $names }
        [many] { $count } папок мають глибину вкладеності понад { $depth }, тому їхній вміст не включено до інсталятора: { $names }
       *[other] { $count } папок мають глибину вкладеності понад { $depth }, тому їхній вміст не включено до інсталятора: { $names }
    }{ $more ->
        [0] .
        [one] { " " }та ще { $more }.
        [few] { " " }та ще { $more }.
        [many] { " " }та ще { $more }.
       *[other] { " " }та ще { $more }.
    }
# { $error } is what went wrong.
core-warning-build-metadata-unreadable = Не вдалося прочитати метадані файлу: { $error }
core-warning-build-reserved-name = Ім'я цього файлу зарезервовано для пристрою Windows, тому його не скопійовано.
# { $error } is what went wrong.
core-warning-build-hash-failed = Не вдалося обчислити хеш файлу: { $error }
core-warning-build-cloud-placeholder = Хмарний файл, доступний лише в Інтернеті (наприклад, файли OneDrive на вимогу), не має локальних даних; закріпіть або завантажте його перед резервним копіюванням. Про нього повідомлено, але його не скопійовано.
# { $file_name } is a driver's file name, such as acmeflt.sys.
core-warning-build-driver-file = Файл драйвера або ядра «{ $file_name }» не копіюється як звичайний файл. Якщо він забезпечує роботу зареєстрованої служби драйвера цієї програми, інсталятор включає його до пакета й автоматично повторно реєструє драйвер; інакше інсталюйте його повторно з підписаного пакета драйвера постачальника.
core-warning-build-file-gone = Цей файл був під час сканування програми, але зараз його немає, тому його не включено до інсталятора.
# { $error } is what went wrong.
core-warning-build-file-copy-failed = Не вдалося скопіювати файл у пакет: { $error }
# { $count } is how many files. { $names } lists them.
core-warning-build-not-looked-through =
    { $count ->
        [1] Цей пакет містить файл, який не вдалося перевірити на відомості про те, від кого походить пакет: { $names }. Вміст цього файлу може містити такі відомості.
        [one] Цей пакет містить файли, які не вдалося перевірити на відомості про те, від кого походить пакет: { $names }. Вміст цих файлів може містити такі відомості.
        [few] Цей пакет містить файли, які не вдалося перевірити на відомості про те, від кого походить пакет: { $names }. Вміст цих файлів може містити такі відомості.
        [many] Цей пакет містить файли, які не вдалося перевірити на відомості про те, від кого походить пакет: { $names }. Вміст цих файлів може містити такі відомості.
       *[other] Цей пакет містить файли, які не вдалося перевірити на відомості про те, від кого походить пакет: { $names }. Вміст цих файлів може містити такі відомості.
    }
# { $count } is the most empty folders an installer makes, such as 10,000.
core-warning-build-empty-folders-truncated = Ця програма має понад { $count } порожніх папок, і буде створено лише стільки.
core-warning-build-credential-by-now = Не включено до пакета: тепер це схоже на облікові дані.
core-warning-build-no-package-path = Файл для копіювання не має відносного шляху в пакеті.
# { $file_name } is the runtime installer's file name, such as vc_redist.x64.exe. { $error } is what went wrong.
core-warning-build-prerequisite-path-invalid = Не вдалося розмістити вбудований необхідний компонент { $file_name }: { $error }
# { $runtime } is a runtime's name, such as Microsoft Visual C++ 2015+ Redistributable (x64). { $error } is what went
# wrong. winget is the name of a Windows command; keep it as it is.
core-warning-build-prerequisite-download-failed = Не вдалося завантажити { $runtime } ({ $error }); інсталятор спробує скористатися winget на цільовому ПК.
# { $runtime } is a runtime's name. { $reason } is why the check failed, such as "it is signed by Contoso Ltd, not
# Microsoft".
core-warning-build-prerequisite-signature-failed = Завантажений файл { $runtime } не пройшов перевірку підпису, тому його не включено: { $reason }. Інсталятор спробує скористатися winget на цільовому ПК.
# { $runtime } is a runtime's name. { $error } is what went wrong.
core-warning-build-prerequisite-hash-failed = { $runtime } завантажено, але не вдалося обчислити його хеш ({ $error }); натомість інсталятор спробує скористатися winget на цільовому ПК.
# { $service } is a driver's service name, such as AcmeFlt. { $error } is what went wrong.
core-warning-build-driver-copy-failed = Не вдалося включити образ драйвера для «{ $service }»: { $error }
core-warning-build-license-not-copied = Не вдалося скопіювати дані ліцензії (їх немає або вони недоступні для читання); записано лише метадані.
core-warning-build-license-file-credential = Файл ліцензії схожий на облікові дані, тому його не включено.
# { $error } is what went wrong.
core-warning-build-license-store-walk-failed = Не вдалося перелічити частину сховища ліцензій: { $error }
core-warning-build-license-store-credential = Файл у сховищі ліцензій схожий на облікові дані, тому його не включено.
# { $error } is what went wrong.
core-warning-build-license-store-file-skipped = Пропущено файл сховища ліцензій: { $error }
