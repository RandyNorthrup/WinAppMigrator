# Words of WinAppMigrator Client, the console program run on the old PC, the one an app is moving off, so that
# WinAppMigrator on the new PC can read it over the network. Message names start with client-.
#
# Its window is a console, which on Windows 7 is 80 columns wide. Each line break in a message is one on the screen:
# keep every line shorter than 80 characters, and break lines where your language needs to, not only where English
# does.


## Starting

# Shown only in English, in place of a right-to-left language the console cannot show. { $language } is that
# language's name in English, such as Arabic.
client-console-rtl-fallback = Эта консоль не может отображать язык { $language }, поэтому клиент использует английский.
# The client copies itself to a folder in Program Files before it starts. { $error } is what Windows said went wrong.
client-safe-folder-failed =
    Не удалось запустить WinAppMigrator Client из папки, изменять которую
    могут только администраторы: { $error }
client-press-enter-to-close = Нажмите клавишу ВВОД, чтобы закрыть это окно.
# The first line of its screen. { $version } is the client's version, such as 0.6.2.
client-heading = WinAppMigrator Client { $version }
client-intro =
    Эта программа позволяет WinAppMigrator на новом компьютере считывать
    приложения с этого компьютера по сети. Здесь она ничего не меняет: она
    считывает снимок диска Windows и удаляет его, когда вы ее останавливаете.
# { $error } is why: one of the messages under "Why it stopped" below, or what Windows said.
client-stopped = Работа клиента остановлена: { $error }
# The caretaker is a copy of the client with no window that cleans up after it. { $error } is what Windows said.
client-caretaker-failed =
    Если закрыть это окно, а не остановить клиент, снимок и правила
    брандмауэра останутся до следующего запуска клиента на этом компьютере
    ({ $error }).
# Shown while the snapshot is taken, followed on the same line by client-snapshot-taken. Keep the quotes: they keep
# the space before it.
client-snapshot-taking = Создание снимка диска Windows...{ " " }
client-snapshot-taken = готово.
# The old PC answers the new one's search for it on the network. { $error } is what Windows said went wrong.
client-discovery-failed =
    Этот компьютер не будет обнаружен в сети автоматически ({ $error }).
    На новом компьютере введите имя или адрес этого компьютера вручную.

## This PC and its pairing code. The three lines are lined up in two columns, behind two spaces; keep the quotes, which
## keep those spaces.

# { $computer_name } is this PC's name, such as OLD-PC.
client-this-pc = { "  " }Этот компьютер:  { $computer_name }
# { $windows_name } is this PC's Windows, such as Windows 7 Professional Service Pack 1. { $bits } is 32 or 64.
client-windows = { "  " }Windows:         { $windows_name }, { $bits }-разрядная
# { $code } is six digits in two groups, such as 482 913.
client-pairing-code = { "  " }Код сопряжения:  { $code }
# Network PC... is the button of that name in WinAppMigrator: use the name it has there in your language.
client-pairing-steps =
    На новом компьютере в WinAppMigrator выберите «Сетевой компьютер...»
    и введите этот код. Сначала закройте переносимое приложение, чтобы его
    файлы остались в том виде, в каком оно их сохранило.
    Чтобы остановить клиент, нажмите здесь клавишу ВВОД или закройте это окно.

## While it runs. { $from } is the network address of the PC that connected, such as 192.168.1.20, or "unknown" when
## it could not be told, which the [unknown] variant words.

client-paired =
    Сопряжение с { $from ->
        [unknown] другим компьютером
       *[address] компьютером { $from }
    } выполнено. Идет считывание нужных данных...
# { $count } is how many requests it answered.
client-builder-done =
    { $from ->
        [unknown] Другой компьютер
       *[address] Компьютер { $from }
    } завершил работу ({ $count ->
        [one] обработан { $count } запрос
        [few] обработано { $count } запроса
        [many] обработано { $count } запросов
       *[other] обработано { $count } запроса
    }).
# { $error } is what went wrong, often in Windows' words.
client-link-ended =
    Связь с { $from ->
        [unknown] другим компьютером
       *[address] компьютером { $from }
    } прервана: { $error }
client-wrong-code =
    { $from ->
        [unknown] Другой компьютер
       *[address] Компьютер { $from }
    } ввел код, который не совпадает с кодом этого компьютера.
# { $error } is what went wrong, often in Windows' words.
client-pair-failed =
    { $from ->
        [unknown] Другому компьютеру
       *[address] Компьютеру { $from }
    } не удалось выполнить сопряжение: { $error }
client-too-many-wrong-codes =
    Введено слишком много неверных кодов, поэтому сейчас ни один компьютер
    не может выполнить сопряжение с этим. Возможно, что-то в вашей сети
    подбирает код. Чтобы выполнить сопряжение, остановите клиент, запустите
    его снова и введите новый код, который он покажет.
# { $code } is six digits in two groups, such as 482 913.
client-new-code = Слишком много неверных кодов. Новый код сопряжения: { $code }

## Stopping

client-stopping = Остановка: удаление снимка и правил брандмауэра...
client-stop-done = Готово.

## Why it stopped, shown after "The client stopped: " (client-stopped), and written to follow it.

client-already-running = клиент уже запущен на этом компьютере или завершает очистку после закрытого экземпляра. Используйте запущенный клиент или подождите немного и запустите этот снова.
# { $drive } is the drive Windows is on, such as C:. { $details } is what Windows said, in its own words.
client-snapshot-failed = Windows не удалось создать снимок диска { $drive }: { $details }
# { $details } is what Windows Firewall said, in its own words.
client-firewall-failed = Брандмауэр Windows не разрешил клиенту принимать подключения: { $details }

## What it takes, shown for --help. The options, such as --lang, are typed as they are and are not translated; keep
## their descriptions lined up.

client-usage =
    WinAppMigrator Client поддерживает следующие параметры:
    
      --lang <язык>      Использовать этот язык, например it-IT, если он есть
                         у клиента, вместо языка интерфейса Windows.
      --code <код>       Начать с этим шестизначным кодом сопряжения (для
                         сценария, который также запускает WinAppMigrator
                         на новом компьютере).
      --help             Показать эту справку.
