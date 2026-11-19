# Words the core writes for people while it builds a package from an installed app: what a package records for an
# installer to show later, errors shown in the app, the list of what goes in and what is left out, opening a disk
# image or another PC over the network, and what an installer says when the package it carries cannot be read.
# Message names start with core-.


## Scanning an app, before a package is built. Shown in the app when a scan cannot go on.

# { $path } is the app's program, such as D:\Program Files\Acme\Acme.exe, on a drive taken from another PC.
core-scan-program-not-on-the-drive = Программа { $path } доступна через ссылку или находится не на подключенном диске, поэтому она не прочитана.
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-not-on-this-pc = Программа { $path } находится не на диске этого компьютера или имеет имя, которое открыло бы что-то другое, поэтому она не прочитана.
# { $path } is the app's program.
core-scan-program-someone-elses = Программа { $path } находится в папках другой учетной записи или не принадлежит этой учетной записи, а в собственной записи этого приложения не указано, что она должна там находиться, поэтому она не прочитана.
# { $app_name } is the app's name, as the list of apps calls it.
core-scan-nothing-to-package = Собственная программа { $app_name } не найдена, поэтому в установщик нечего поместить. Так часто бывает с драйверами, средами выполнения и надстройками: на новом компьютере установите драйвер от производителя или через Центр обновления Windows, а среду выполнения или надстройку — вместе с приложением, которое ее использует.
# { $app_name } is the app's name, as the list of apps calls it. A web app is a website installed as an app, with
# a window and a Start menu entry of its own.
core-scan-web-app = { $app_name } — это веб-приложение, которое браузер установил с веб-сайта. У него нет собственной программы, поэтому в установщик нечего поместить. На новом компьютере откройте этот веб-сайт в том же браузере и установите приложение оттуда.
# { $path } is the program the app's own record names, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-missing = Программы { $path } больше нет, хотя в записи этого приложения она указана как его программа. Возможно, приложение было перемещено или частично удалено: восстановление или повторная установка на этом компьютере должны ее вернуть.
# { $path } is the file that was picked.
core-scan-not-a-file = объект сканирования не является файлом: { $path }
# Names something left out of a package, in a list of what was left out and why. { $name } is the variable's name,
# such as ACME_LICENSE_KEY.
core-scan-environment-variable-item = Переменная среды { $name }

## Folders and parts that might also be the app's, offered in a list to tick. Each says why it is offered.

core-near-starts-with-app-name = Имя начинается с имени этого приложения.
core-near-has-app-name = Имя содержит имя этого приложения.
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-starts-with-publisher = Имя начинается с имени издателя: { $publisher }.
# { $folder } is a folder's name, such as Program Files.
core-near-hidden-part-beside = Скрытый из списка приложений компонент, установленный рядом с этим приложением в папке { $folder }.
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-hidden-part-publisher = Скрытый из списка приложений компонент того же издателя: { $publisher }.

## What a scan found, as a person chooses from it: the folders that go in, and what is left out, and why

# Why a folder cannot be left out, beside its number of files.
core-footprint-locked-app-folder = Всегда включаются вместе с основной программой
core-footprint-locked-main-program = Содержит основную программу, которая включается всегда
core-footprint-locked-needed = Содержит то, без чего основная программа не запустится
core-footprint-locked-font = Включается вместе со своим шрифтом в разделе «Что связывает его с Windows»
# Where a folder is. "AppData" and "ProgramData" are the names of Windows folders; keep them as they are.
core-footprint-place-roaming = Параметры в вашем профиле (AppData)
core-footprint-place-local = Параметры этого компьютера в вашем профиле (AppData)
core-footprint-place-program-data = Общие для всех пользователей компьютера (ProgramData)
core-footprint-place-documents = В папке «Документы»
core-footprint-place-saved-games = В папке «Сохраненные игры»
core-footprint-place-public-documents = В папке «Общие документы»
core-footprint-place-app-folder = В папке приложения
# { $folder } is a folder, such as C:\Users\Sam\AppData\Roaming.
core-footprint-place-in = В папке { $folder }
# A row for the files directly in the app's folder. { $folder } is that folder's name, such as Acme.
core-footprint-files-directly-in = Файлы непосредственно в папке { $folder }
# Headings over what is left out, and why each item is.
core-footprint-licenses-left-out = Не входит в этот установщик, так как параметр «Данные лицензии и активации» выключен
core-footprint-font-stays-behind = Остается на этом компьютере.
core-footprint-drivers-left-out = Драйверы, принадлежащие Windows или поставщику: их нельзя передавать, поэтому переустановите их на другом компьютере
core-footprint-driver-reinstall = Переустановите его из собственного пакета поставщика.
# Why something else is left out along with what the person left out. { $service } is a service's name, such as AcmeSync.
core-footprint-needs-left-out = требуется служба { $service }, которая исключена
core-footprint-told-to-use-left-out = настроено на использование содержимого папки, которую вы исключили
core-footprint-file-in-left-out = файл находится в папке, которую вы исключили
core-footprint-program-in-left-out = программа, к которой это относится, находится в папке, которую вы исключили
core-footprint-names-left-out = указывает на папку, которую вы исключили
# Why something is left out, as the installer tells whoever installs it.
core-footprint-builder-left-it-out = исключено тем, кто создал этот установщик
# Something the person left out, by kind and name, such as "Service Acme Updater". { $kind } is the kind, such as
# Service or Scheduled Task; { $title } is its name.
core-footprint-left-out-item = { $kind } { $title }

## Building a package and an installer from a scan. Shown in the app when a build cannot go on.

core-package-scan-of-another-app = это сканирование относится не к создаваемому приложению, а к другому, поэтому ничего не создано
core-package-scan-brought-in-more = это сканирование включило данные, которые для этого создания не запрашивались, поэтому ничего не создано
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-package-program-gone = программы { $path } больше нет, поэтому ничего не создано. Проверьте приложение снова.
core-package-needs-stub = для самораспаковывающегося архива требуется заготовка установщика
# { $reason } is what went wrong.
core-package-staged-file-failed = файл из промежуточной папки не удалось добавить в пакет: { $reason }
# { $shortfalls } says which drives are short of room and by how much, in sentences of its own.
core-package-not-enough-room = недостаточно места для создания этого установщика, поэтому ничего не прочитано и не записано. { $shortfalls }. Освободите там место или выберите выходную папку на другом диске.
# { $path } is a file's path, on a drive taken from another PC.
core-package-not-read-off-the-drive = файл { $path } доступен через ссылку или находится не на подключенном диске, поэтому он не прочитан
# { $path } is a file's path.
core-package-not-read-not-local = файл { $path } находится не на диске этого компьютера, поэтому он не прочитан
# { $path } is a file's path.
core-package-not-read-other-account = путь { $path } ведет в папки другой учетной записи, поэтому файл не прочитан
# { $path } is a file's path, as the PC it came from called it.
core-package-not-on-the-drive = файла { $path } нет на подключенном диске, поэтому он не прочитан
# { $path } is a file's path.
core-package-backup-copy-no-parent = невозможно выполнить резервное копирование файла без родительской папки: { $path }
# { $path } is a file's path.
core-package-backup-copy-destination-no-parent = у места назначения резервного копирования нет родительской папки: { $path }
# robocopy is the name of a Windows command; keep it as it is. { $code } is how it ended, such as Some(8).
core-package-robocopy-failed = ошибка копирования robocopy в режиме резервного копирования (код выхода { $code })
# Stands for a folder where a file's name would be, as in "I/O failed for temporary package folder: …".
core-package-temporary-folder = временная папка пакета
# Stands for the installer being written where a file's name would be, as in "I/O failed for the installer: …".
core-package-the-installer = установщик
# { $path } is a file's path, or its name in the package. { $error } is what went wrong with it.
core-package-path-error = { $path }: { $error }
# { $name } is a file's name in the package, such as app/bin/App.exe.
core-package-same-place = другой файл занимает то же место в пакете: { $name }
# { $name } is a file's name in the package.
core-package-changed-in-protected-folder = файл { $name } изменился в защищенной папке во время создания пакета
core-package-worker-died = поток сжатия файлов неожиданно остановился, поэтому установщик не записан
# { $path } is a file's path.
core-package-link-not-read = { $path } — это ссылка, поэтому она не прочитана
# { $path } is a file's path. { $error } is what went wrong.
core-package-not-compressed = не удалось сжать { $path }: { $error }
# { $path } is a file's path. { $reason } is what went wrong.
core-package-not-read-back = не удалось повторно прочитать { $path }: { $reason }

## Where an app's files are, and where an installer may write them. Shown when a path is refused.

# { $path } is a folder, such as C:\Program Files\Acme.
core-discovery-root-not-full-path = папка для записи должна быть указана полным путем: { $path }
# { $path } is a folder's path.
core-discovery-root-not-a-folder = путь для записи не является папкой: { $path }
# { $path } is a file's path.
core-discovery-escapes-root = путь выходит за пределы предусмотренной корневой папки: { $path }
# { $path } is the link's path.
core-discovery-refusing-to-write-through-link = отказано в записи через ссылку: { $path }
# { $path } is the link's path.
core-discovery-refusing-link = отказано в переходе по ссылке: { $path }
# { $error } is what Windows said went wrong.
core-discovery-refusing-link-because = отказано в переходе по ссылке: { $error }
# { $path } is the link's path.
core-discovery-link-to-somewhere-else = { $path } — это ссылка на другое расположение, и другие могут изменить то, куда она ведет. Используйте расположение, на которое она указывает
# { $path } is a folder's path.
core-discovery-no-parent-folder = у { $path } нет родительской папки
# { $path } is the install location as it was given. Keep the quotes, or use your language's own.
core-discovery-install-location-not-full-path = для папки установки нужно указать полный путь: «{ $path }»
# { $path } is the install location.
core-discovery-install-location-a-link = папка установки — это ссылка на другое расположение: { $path }
# { $path } is a file's path.
core-discovery-device-name = в пути используется имя, зарезервированное для устройства Windows: { $path }
# { $path } is a file's path.
core-discovery-device-name-on-older-windows = имя { $path } эта версия Windows открывает как устройство, а не как файл, поэтому в него ничего не записано. Этот файл можно установить в Windows 11
# { $path } is a file's path.
core-discovery-unsafe-path = небезопасный путь: { $path }
# { $path } is a file's path.
core-discovery-no-package-path = у файла нет относительного пути в пакете: { $path }

## Writing an installer, and reading the package inside one. Most of these are shown by an installer, on the PC it
## runs on, when it cannot read what it carries. "The package" is the app's files and settings that an installer
## carries; "the stub" is the program part of an installer, before the package; "the data file" is the .wamdata file
## that holds the package of a very large installer, beside it.

# Stands for the package where a file's name would be, as in "I/O failed for installer package: …".
core-archive-installer-package = пакет установщика
# Stands for the installer program built into WinAppMigrator, where a file's name would be.
core-archive-embedded-stub = встроенная заготовка установщика
# { $index } is the number of a file in the package. { $reason } is what went wrong.
core-archive-entry-unreadable = не удалось прочитать элемент { $index } пакета: { $reason }
# { $name } is a file's name inside the package, such as app/bin/App.exe.
core-archive-entry-unsafe-path = у элемента ZIP небезопасный путь: { $name }
# { $name } is a file's name inside the package.
core-archive-holds-more = элемент { $name } содержит больше данных, чем указано
# { $name } is a file's name inside the package.
core-archive-name-too-long = это имя слишком длинное для ZIP: { $name }
core-archive-changed-while-copied = пакет изменился, пока добавлялся после заготовки, поэтому установщик не записан
core-archive-stub-not-a-program = часть этого установщика перед пакетом не является программой
core-archive-damaged = этот установщик поврежден или был изменен после создания, поэтому из его пакета ничего не прочитано. Скопируйте его снова оттуда, где он был создан.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-missing = этот установщик хранит свой пакет в файле { $name } в той же папке, но этого файла там нет. Два файла должны храниться вместе: скопируйте сюда { $name } оттуда, где был создан установщик, а затем запустите его снова.
# { $name } is the data file's name, such as App_installer.wamdata. { $error } is what Windows said went wrong.
core-archive-data-file-unopened = не удалось открыть файл данных этого установщика { $name }: { $error }. Два файла должны храниться вместе в папке, доступной для чтения на этом компьютере.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-a-link = { $name } — это ссылка или папка, а файл данных этого установщика должен быть отдельным файлом рядом с ним, поэтому из него ничего не прочитано. Скопируйте сюда настоящий файл.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-wrong = { $name } не является файлом данных этого установщика либо поврежден или изменен после создания, поэтому из него ничего не прочитано. Скопируйте оба файла снова оттуда, где они были созданы.
core-archive-stub-signed = заготовка установщика подписана; она должна оставаться неподписанной, поэтому подписывайте готовый установщик
# { $size } is a size, such as 4.2 GB.
core-archive-too-big-to-run = размер установщика составил бы { $size }, а Windows не может запустить исполняемый файл размером 4 ГБ или больше; ничего не записано
core-archive-outside-signature = в установщике есть данные за пределами подписи; возможно, он был злонамеренно изменен
core-archive-footer-length = недопустимая длина завершающего блока самораспаковывающегося архива
core-archive-seek-outside = переход за пределы пакета
# { $path } is a file's path.
core-archive-unsafe-package-path = небезопасный путь в пакете: { $path }
# { $path } is a file's path.
core-archive-empty-package-path = пустой путь в пакете: { $path }

## Opening a disk image of another PC (a .vhd or .vhdx file) to read apps from it

# { $path } is the disk image file, such as D:\Backups\OldPC.vhdx.
core-image-not-attachable = { $path } не является образом диска, который Windows может подключить (.vhd или .vhdx)
core-image-needs-administrator = для подключения образа диска нужны права администратора
# { $path } is the disk image file the person picked.
core-image-not-a-file = { $path } не является файлом
# { $path } is the disk image file. "Edit Disk" is the name of a Hyper-V Manager command.
core-image-differencing = { $path } — это разностный диск: он содержит только изменения относительно диска, из которого создан, и Windows открыла бы также и тот диск, где бы он ни находился по указанному пути. Сначала выполните его слияние с тем диском с помощью команды «Изменить диск» в Hyper-V и выберите тот диск
# { $path } is the disk image file.
core-image-unreadable = { $path } не является файлом .vhd или .vhdx, доступным для чтения
# ProgramData is the name of a Windows folder; keep it as it is.
core-image-no-program-data = не удалось найти папку ProgramData этого компьютера
# The reason given in core-image-could-not-attach when Windows already has the image open.
core-image-already-attached = этот образ уже подключен в Windows, и то, что его подключило, может записывать в него данные. Сначала отключите его или выберите его диск как подключенный диск
# { $path } is the disk image file. { $reason } is why, such as core-image-already-attached or what Windows said.
core-image-could-not-attach = не удалось подключить { $path }: { $reason }
# { $path } is the disk image file.
core-image-no-windows-volume = в образе { $path } нет тома с Windows, доступного для чтения. Том, зашифрованный с помощью BitLocker, сначала нужно разблокировать.
# Given as the reason when Windows failed without saying anything.
core-image-windows-did-not-say = Windows не сообщила причину

## Reading another PC over the network, through the client running on it. Shown as why something on that PC
## could not be read. "The client" is WinAppMigrator Client, the small program on the other PC.

core-remote-too-slow = другой компьютер слишком долго не отвечал
core-remote-closed = связь с другим компьютером закрыта
core-remote-failed = сбой связи с другим компьютером
core-remote-answered-something-else = клиент дал неожиданный ответ
# { $name } is the PC's name or address as the person typed it, such as OLDPC or 192.168.1.20.
core-remote-not-found = компьютер { $name } не найден в сети
core-remote-did-not-say-what-it-is = клиент не сообщил, что он собой представляет
core-remote-ordinary-names-only = запрашивать можно только путь внутри корневой папки другого компьютера, состоящий из обычных имен
core-remote-more-but-none = другой компьютер сообщил, что в папке есть еще содержимое, но не передал его
# { $count } is the most things one folder may hold, such as 250,000.
core-remote-too-many-listed = другой компьютер перечислил в одной папке слишком много элементов (больше { $count })
core-remote-sent-too-much = другой компьютер передал больше данных файла, чем было запрошено
core-remote-name-not-text = нельзя запросить имя, которое не является текстом
core-remote-below-root-only = запрашивать можно только путь внутри корневой папки другого компьютера
core-remote-file-ended-early = файл на другом компьютере закончился раньше ожидаемого
core-remote-link-not-followed = переход по ссылкам на другом компьютере не выполняется
core-remote-only-files-opened = на другом компьютере открываются только файлы
core-remote-listing-said-nothing = в списке содержимого об этом ничего не сказано
core-remote-change-time-unknown = время изменения неизвестно
core-remote-seek-before-start = переход к позиции перед началом
# { $size } is how many bytes a file is; { $limit } is the most that were expected. Both are numbers.
core-remote-too-large =
    { $size ->
        [one] размер составляет { $size } байт, а ожидалось не более { $limit }
        [few] размер составляет { $size } байта, а ожидалось не более { $limit }
        [many] размер составляет { $size } байт, а ожидалось не более { $limit }
       *[other] размер составляет { $size } байта, а ожидалось не более { $limit }
    }

## Checking that a download is signed by Microsoft. Each is given as the reason a check failed.

# { $error } is what Windows said went wrong.
core-signature-no-powershell = не удалось запустить PowerShell: { $error }
# { $error } is what went wrong reading PowerShell's answer.
core-signature-unreadable = не удалось прочитать подпись: { $error }
# { $signer } is who signed the file, such as Contoso Ltd.
core-signature-not-microsoft = файл подписан издателем { $signer }, а не Microsoft
core-signature-not-microsoft-unnamed = файл подписан не Microsoft, а кем-то другим
# { $status } is Windows' own word for the signature's state, such as HashMismatch or NotSigned, which is not translated.
core-signature-status = состояние подписи: { $status }

## Warnings a package records when it is built from an app, which an installer shows later, maybe on another PC, in
## that PC's language. "This app" is the app being packaged.

core-warning-build-program-needs-unreadable = Не удалось прочитать, что этой программе нужно от Windows, поэтому возможность ее запуска на другом компьютере не будет проверена.
core-warning-build-external-manifest = Обнаружен внешний манифест приложения; анализ параллельных (side-by-side) зависимостей отложен.
core-warning-build-non-exe = Выбранный файл не имеет расширения .exe; анализ PE продолжится.
core-warning-build-stand-in-program = Обычный исполняемый файл приложения не найден, поэтому вместо него используется этот файл. Результаты по архитектуре и зависимостям описывают этот файл и могут не относиться к приложению.
core-warning-build-other-apps-files-unknown = Не все сведения установщика Windows о других приложениях удалось прочитать, поэтому записанная для этого приложения папка может содержать файлы другого приложения, хотя здесь об этом ничего не будет сказано.
core-warning-build-app-folder-has-another-apps-file = По данным установщика Windows, в папке программы этого приложения есть файл другого приложения, и он переносится вместе с ней, так как берется вся эта папка.
# "App Paths" is the name of a Windows registry key; keep it as it is.
core-warning-build-app-paths-folder-left-out = Запись App Paths этого приложения указывает, что оно ищет файлы в чужой папке, поэтому эта папка не переносится. Без ее содержимого приложение может не запуститься на другом компьютере.
core-warning-build-uninstaller-folder-left-out = Папка программы удаления этого приложения исключена, так как, по данным установщика Windows, в ней есть файл другого приложения.
# { $why } is never shown: it chooses the reason. Translate each reason as the end of the sentence.
core-warning-build-recorded-folder-left-out =
    В записях этого приложения указана папка, которая была исключена, так как { $why ->
        [unreadable] путь к ней выходит за пределы диска или проходит через ссылку, поэтому читать ее небезопасно
        [missing] ее нет
        [link] путь к ней проходит через ссылку
        [short-name] ее путь записан не так, как она на самом деле называется
        [another-account] она находится в папках другой учетной записи
        [one-account] это приложение установлено для одной учетной записи, а она относится к другой
        [shared] в ней хранятся общие данные или она является частью Windows
        [generic-name] ее имя ничего не говорит о том, кому она принадлежит
        [publisher] она названа только по имени издателя и может содержать другие его продукты
        [holds-the-app] она содержит собственную папку этого приложения и может содержать что-то еще
        [another-apps-name] она носит имя другого приложения
        [another-apps-record] по записи об установке она принадлежит другому приложению
        [in-another-apps-folder] она находится в папке другого приложения
       *[another-apps] она принадлежит другому приложению
    }.
core-warning-build-app-folder-holds-another-app = Согласно записям, другое приложение на этом компьютере находится внутри папки этого приложения, и оно переносится вместе с ней, так как берется вся эта папка.
# "Program Files (x86)" is the name of a Windows folder; keep it as Windows calls it.
core-warning-build-from-32-bit-windows = Это приложение было установлено в 32-разрядной Windows. Его разделы реестра размещены так, как 64-разрядная Windows хранит разделы 32-разрядных приложений, а установщик предлагает для него папку Program Files (x86).
# { $dll } is a file's name, such as MSVCP140D.dll.
core-warning-build-debug-runtime = Программе нужен { $dll } — отладочная сборка среды выполнения Visual C++. У Microsoft нет установщика для таких сборок, и их распространение может быть запрещено, поэтому программа не запустится на компьютере без Visual Studio.
# { $program } is a program's file name, such as Tool.exe.
core-warning-build-single-file-program = { $program } — это программа .NET, упакованная в один файл, по которому нельзя определить, какая версия .NET ей нужна, поэтому для нее ничего не запланировано. Если ей нужна .NET, она не запустится на компьютере без нее.
# { $count } is how many of the app's programs.
core-warning-build-programs-not-inspected =
    { $count ->
        [one] { $count } программа приложения не проверена на нужные ей среды выполнения, так как программ слишком много или они слишком велики.
        [few] { $count } программы приложения не проверены на нужные им среды выполнения, так как программ слишком много или они слишком велики.
        [many] { $count } программ приложения не проверены на нужные им среды выполнения, так как программ слишком много или они слишком велики.
       *[other] { $count } программы приложения не проверены на нужные им среды выполнения, так как программ слишком много или они слишком велики.
    }
# { $runtime } is a runtime's name, such as Java Runtime Environment.
core-warning-build-runtime-no-installer = Приложению нужен компонент { $runtime }, а установщика для него здесь нет, поэтому он не устанавливается. Установите его на другом компьютере самостоятельно.
# { $why } is why something was left out, such as "its name says what it holds".
core-warning-build-left-out-of-package = Исключено из пакета: { $why }.
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-missing-dependency = Не удалось разрешить зависимость: { $name }
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-dependency-only-here = Файл { $name } найден только по пути поиска этого компьютера и не входит в приложение, поэтому он не переносится. Без него приложение может не запуститься на другом компьютере.
core-warning-build-driver-detected = Обнаружен драйвер или компонент ядра. Драйвер, принадлежащий приложению, включается в пакет и регистрируется повторно; драйвер поставщика или встроенный драйвер Windows не распространяется, и его нужно переустановить из подписанного пакета поставщика.
core-warning-build-near-no-longer-offered = Элемент, отмеченный в списке того, что может также относиться к этому приложению, больше не входит в этот список, поэтому он не добавлен. Просмотрите список снова.
# { $app_name } is the name of a part of the app, as the list of apps calls it. { $why } is never shown: it chooses the
# reason. Translate each reason as the end of the sentence.
core-warning-build-near-part-left-out =
    Часть того, что { $app_name } считает своим, исключена: { $why ->
        [unreadable] путь к ней выходит за пределы диска или проходит через ссылку, поэтому читать ее небезопасно
        [missing] ее нет
        [link] путь к ней проходит через ссылку
        [short-name] ее путь записан не так, как она на самом деле называется
        [another-account] она находится в папках другой учетной записи
        [one-account] это приложение установлено для одной учетной записи, а она относится к другой
        [shared] в ней хранятся общие данные или она является частью Windows
        [generic-name] ее имя ничего не говорит о том, кому она принадлежит
        [publisher] она названа только по имени издателя и может содержать другие его продукты
        [holds-the-app] она содержит собственную папку этого приложения и может содержать что-то еще
        [another-apps-name] она носит имя другого приложения
        [another-apps-record] по записи об установке она принадлежит другому приложению
        [in-another-apps-folder] она находится в папке другого приложения
       *[another-apps] она принадлежит другому приложению
    }.
# { $app_name } is the name of a part of the app, as the list of apps calls it.
core-warning-build-near-keys-left-out = Разделы реестра, указанные в записи об установке { $app_name }, исключены, так как в этой записи указано и то, что принадлежит другому приложению.
# "Windows keeps what an app wrote for one account" is Windows' VirtualStore folder.
core-warning-build-virtual-store-program = Программа находилась в папке, где Windows хранит то, что приложение записало для одной учетной записи. Windows никогда не помещает туда программы, значит, это сделало что-то другое, и программа исключена.
# { $path } is a file or folder, such as C:\Users\Sam\AppData\Roaming\Acme.
core-warning-build-file-left-out = { $path }: исключено, так как путь проходит через ссылку на подключенном диске или принадлежит другой учетной записи, а не той, чьи данные включаются.
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as D:\Sam\Documents.
core-warning-build-user-folder-not-on-drive = Папка «{ $folder }» пользователя { $user } находилась в расположении { $path }, которое не на этом диске, поэтому ее содержимое не включено.
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as \\server\home\Sam\Documents.
core-warning-build-user-folder-not-on-this-pc = Папка «{ $folder }» пользователя { $user } находилась в расположении { $path }, которое не на диске этого компьютера, поэтому ее содержимое не включено.
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-base-unreadable = Не удалось прочитать { $path } ({ $error }); хранящиеся там данные приложений не включены.
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-folder-unreadable = Не удалось прочитать { $path } ({ $error }); вложенные данные приложений из этой папки могут отсутствовать.
# { $error } is what went wrong.
core-warning-build-entry-unreadable = Не удалось прочитать элемент файловой системы: { $error }
# { $count } is how many links. { $names } lists some of them. { $more } is how many more there are, which are not
# named; when it is 0, the sentence ends after the list.
core-warning-build-links-not-followed =
    { $count ->
        [one] В этой папке не обработана { $count } ссылка или имя, ведущее в другое место, поэтому то, на что указывает этот элемент, не входит в установщик: { $names }
        [few] В этой папке не обработаны { $count } ссылки или имени, ведущих в другое место, поэтому то, на что они указывают, не входит в установщик: { $names }
        [many] В этой папке не обработано { $count } ссылок или имен, ведущих в другое место, поэтому то, на что они указывают, не входит в установщик: { $names }
       *[other] В этой папке не обработано { $count } ссылки или имени, ведущих в другое место, поэтому то, на что они указывают, не входит в установщик: { $names }
    }{ $more ->
        [0] .
        [one] { " " }и еще { $more }.
        [few] { " " }и еще { $more }.
        [many] { " " }и еще { $more }.
       *[other] { " " }и еще { $more }.
    }
# { $count } is how many folders. { $depth } is how many levels down a folder can be. { $names } lists some of them.
# { $more } is how many more there are, which are not named; when it is 0, the sentence ends after the list.
core-warning-build-folders-too-deep =
    { $count ->
        [one] { $count } папка имеет уровень вложенности больше { $depth }, поэтому ее содержимое не входит в установщик: { $names }
        [few] { $count } папки имеют уровень вложенности больше { $depth }, поэтому их содержимое не входит в установщик: { $names }
        [many] { $count } папок имеют уровень вложенности больше { $depth }, поэтому их содержимое не входит в установщик: { $names }
       *[other] { $count } папки имеют уровень вложенности больше { $depth }, поэтому их содержимое не входит в установщик: { $names }
    }{ $more ->
        [0] .
        [one] { " " }и еще { $more }.
        [few] { " " }и еще { $more }.
        [many] { " " }и еще { $more }.
       *[other] { " " }и еще { $more }.
    }
# { $error } is what went wrong.
core-warning-build-metadata-unreadable = Не удалось прочитать метаданные файла: { $error }
core-warning-build-reserved-name = Имя этого файла зарезервировано для устройства Windows, поэтому файл не скопирован.
# { $error } is what went wrong.
core-warning-build-hash-failed = Не удалось вычислить хэш файла: { $error }
core-warning-build-cloud-placeholder = У облачного файла нет локальной копии (например, при использовании функции «Файлы по запросу» OneDrive); перед резервным копированием сделайте его всегда доступным на этом устройстве или скачайте его. Файл указан в отчете, но не скопирован.
# { $file_name } is a driver's file name, such as acmeflt.sys.
core-warning-build-driver-file = Файл драйвера или ядра «{ $file_name }» не копируется как обычный файл. Если он относится к зарегистрированной службе драйвера этого приложения, установщик включает его в пакет и автоматически регистрирует драйвер повторно; в противном случае переустановите его из подписанного пакета драйвера от поставщика.
core-warning-build-file-gone = Этот файл был на месте при проверке приложения, но сейчас его нет, поэтому он не входит в установщик.
# { $error } is what went wrong.
core-warning-build-file-copy-failed = Не удалось скопировать файл в пакет: { $error }
# { $count } is how many files. { $names } lists them.
core-warning-build-not-looked-through =
    { $count ->
        [1] В этом пакете есть файл, который не удалось проверить на сведения о том, от кого получен пакет: { $names }. В его содержимом такие сведения могут быть.
        [one] В этом пакете есть файлы, которые не удалось проверить на сведения о том, от кого получен пакет: { $names }. В их содержимом такие сведения могут быть.
        [few] В этом пакете есть файлы, которые не удалось проверить на сведения о том, от кого получен пакет: { $names }. В их содержимом такие сведения могут быть.
        [many] В этом пакете есть файлы, которые не удалось проверить на сведения о том, от кого получен пакет: { $names }. В их содержимом такие сведения могут быть.
       *[other] В этом пакете есть файлы, которые не удалось проверить на сведения о том, от кого получен пакет: { $names }. В их содержимом такие сведения могут быть.
    }
# { $count } is the most empty folders an installer makes, such as 10,000.
core-warning-build-empty-folders-truncated = В этом приложении слишком много пустых папок (больше { $count }); заново будет создано только { $count }.
core-warning-build-credential-by-now = Исключено из пакета: теперь это похоже на учетные данные.
core-warning-build-no-package-path = У копируемого файла нет относительного пути в пакете.
# { $file_name } is the runtime installer's file name, such as vc_redist.x64.exe. { $error } is what went wrong.
core-warning-build-prerequisite-path-invalid = Не удалось разместить включенный в пакет необходимый компонент { $file_name }: { $error }
# { $runtime } is a runtime's name, such as Microsoft Visual C++ 2015+ Redistributable (x64). { $error } is what went
# wrong. winget is the name of a Windows command; keep it as it is.
core-warning-build-prerequisite-download-failed = Не удалось скачать { $runtime } ({ $error }); установщик попробует использовать winget на целевом компьютере.
# { $runtime } is a runtime's name. { $reason } is why the check failed, such as "it is signed by Contoso Ltd, not
# Microsoft".
core-warning-build-prerequisite-signature-failed = Скачанный компонент { $runtime } не прошел проверку подписи, поэтому он не включен в пакет: { $reason }. Установщик попробует использовать winget на целевом компьютере.
# { $runtime } is a runtime's name. { $error } is what went wrong.
core-warning-build-prerequisite-hash-failed = Компонент { $runtime } скачан, но вычислить его хэш не удалось ({ $error }); вместо этого установщик попробует использовать winget на целевом компьютере.
# { $service } is a driver's service name, such as AcmeFlt. { $error } is what went wrong.
core-warning-build-driver-copy-failed = Не удалось включить в пакет образ драйвера для «{ $service }»: { $error }
core-warning-build-license-not-copied = Не удалось скопировать данные лицензии (они отсутствуют или недоступны для чтения); записаны только метаданные.
core-warning-build-license-file-credential = Файл лицензии похож на учетные данные и был исключен.
# { $error } is what went wrong.
core-warning-build-license-store-walk-failed = Не удалось перечислить часть хранилища лицензий: { $error }
core-warning-build-license-store-credential = Файл в хранилище лицензий похож на учетные данные и был исключен.
# { $error } is what went wrong.
core-warning-build-license-store-file-skipped = Пропущен файл хранилища лицензий: { $error }
