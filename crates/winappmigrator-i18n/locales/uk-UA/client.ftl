# Words of WinAppMigrator Client, the console program run on the old PC, the one an app is moving off, so that
# WinAppMigrator on the new PC can read it over the network. Message names start with client-.
#
# Its window is a console, which on Windows 7 is 80 columns wide. Each line break in a message is one on the screen:
# keep every line shorter than 80 characters, and break lines where your language needs to, not only where English
# does.


## Starting

# Shown only in English, in place of a right-to-left language the console cannot show. { $language } is that
# language's name in English, such as Arabic.
client-console-rtl-fallback = Ця консоль не може відображати мову { $language }, тому клієнт працює англійською.
# The client copies itself to a folder in Program Files before it starts. { $error } is what Windows said went wrong.
client-safe-folder-failed =
    Не вдалося запустити WinAppMigrator Client з папки, яку можуть змінювати
    лише адміністратори: { $error }
client-press-enter-to-close = Натисніть клавішу Enter, щоб закрити це вікно.
# The first line of its screen. { $version } is the client's version, such as 0.6.2.
client-heading = WinAppMigrator Client { $version }
client-intro =
    Завдяки цьому WinAppMigrator на новому ПК може читати програми цього ПК
    через вашу мережу. Тут нічого не змінюється: клієнт читає знімок диска
    Windows і видаляє його, коли ви зупиняєте клієнт.
# { $error } is why: one of the messages under "Why it stopped" below, or what Windows said.
client-stopped = Клієнт зупинився: { $error }
# The caretaker is a copy of the client with no window that cleans up after it. { $error } is what Windows said.
client-caretaker-failed =
    Якщо закрити це вікно, а не зупинити клієнт, знімок і правила брандмауера
    залишаться до наступного запуску клієнта на цьому ПК ({ $error }).
# Shown while the snapshot is taken, followed on the same line by client-snapshot-taken. Keep the quotes: they keep
# the space before it.
client-snapshot-taking = Створення знімка диска Windows...{ " " }
client-snapshot-taken = готово.
# The old PC answers the new one's search for it on the network. { $error } is what Windows said went wrong.
client-discovery-failed =
    Цей ПК не з'явиться в мережі автоматично ({ $error }).
    На новому ПК натомість введіть ім'я або адресу цього ПК.

## This PC and its pairing code. The three lines are lined up in two columns, behind two spaces; keep the quotes, which
## keep those spaces.

# { $computer_name } is this PC's name, such as OLD-PC.
client-this-pc = { "  " }Цей ПК:           { $computer_name }
# { $windows_name } is this PC's Windows, such as Windows 7 Professional Service Pack 1. { $bits } is 32 or 64.
client-windows = { "  " }Windows:          { $windows_name }, { $bits }-розрядна
# { $code } is six digits in two groups, such as 482 913.
client-pairing-code = { "  " }Код зв'язування:  { $code }
# Network PC... is the button of that name in WinAppMigrator: use the name it has there in your language.
client-pairing-steps =
    На новому ПК у WinAppMigrator виберіть «Мережевий ПК...» і введіть цей код.
    Спершу закрийте програму, яку переносите, щоб її файли були такими,
    як вона їх залишила.
    Щоб зупинити клієнт, натисніть тут Enter або закрийте це вікно.

## While it runs. { $from } is the network address of the PC that connected, such as 192.168.1.20, or "unknown" when
## it could not be told, which the [unknown] variant words.

client-paired =
    Зв'язано з { $from ->
        [unknown] іншим ПК
       *[address] { $from }
    }. Триває читання потрібних даних...
# { $count } is how many requests it answered.
client-builder-done =
    Роботу з { $from ->
        [unknown] іншим ПК
       *[address] { $from }
    } завершено (оброблено { $count ->
        [one] { $count } запит
        [few] { $count } запити
        [many] { $count } запитів
       *[other] { $count } запитів
    }).
# { $error } is what went wrong, often in Windows' words.
client-link-ended =
    З'єднання з { $from ->
        [unknown] іншим ПК
       *[address] { $from }
    } розірвано: { $error }
client-wrong-code =
    З { $from ->
        [unknown] іншого ПК
       *[address] { $from }
    } введено код, який не відповідає цьому.
# { $error } is what went wrong, often in Windows' words.
client-pair-failed =
    Не вдалося зв'язатися з { $from ->
        [unknown] іншим ПК
       *[address] { $from }
    }: { $error }
client-too-many-wrong-codes =
    Забагато спроб із неправильним кодом, тому тепер жоден ПК не може
    зв'язатися з цим. Можливо, щось у вашій мережі намагається підібрати код.
    Щоб зв'язати ПК, зупиніть клієнт, запустіть його знову й введіть новий код,
    який він покаже.
# { $code } is six digits in two groups, such as 482 913.
client-new-code = Забагато неправильних кодів. Новий код зв'язування: { $code }

## Stopping

client-stopping = Зупинка: видалення знімка й правил брандмауера...
client-stop-done = Готово.

## Why it stopped, shown after "The client stopped: " (client-stopped), and written to follow it.

client-already-running = клієнт уже працює на цьому ПК або ще виконує очищення після клієнта, якого було закрито. Скористайтеся ним або трохи зачекайте й запустіть цей клієнт знову.
# { $drive } is the drive Windows is on, such as C:. { $details } is what Windows said, in its own words.
client-snapshot-failed = Windows не вдалося створити знімок диска { $drive }: { $details }
# { $details } is what Windows Firewall said, in its own words.
client-firewall-failed = Брандмауер Windows не дозволив клієнту прослуховувати підключення: { $details }

## What it takes, shown for --help. The options, such as --lang, are typed as they are and are not translated; keep
## their descriptions lined up.

client-usage =
    WinAppMigrator Client підтримує такі параметри:
    
      --lang <мова>      Використовувати цю мову, наприклад it-IT, якщо вона є
                         в клієнті, замість мови інтерфейсу Windows.
      --code <код>       Почати з цим шестизначним кодом зв'язування (для
                         сценарію, що запускає WinAppMigrator і на новому ПК).
      --help             Показати цю довідку.
