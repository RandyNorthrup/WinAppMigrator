# Words of WinAppMigrator Client, the console program run on the old PC, the one an app is moving off, so that
# WinAppMigrator on the new PC can read it over the network. Message names start with client-.
#
# Its window is a console, which on Windows 7 is 80 columns wide. Each line break in a message is one on the screen:
# keep every line shorter than 80 characters, and break lines where your language needs to, not only where English
# does.


## Starting

# Shown only in English, in place of a right-to-left language the console cannot show. { $language } is that
# language's name in English, such as Arabic.
client-console-rtl-fallback = Bu konsol { $language } dilini gösteremediği için istemci İngilizce kullanıyor.
# The client copies itself to a folder in Program Files before it starts. { $error } is what Windows said went wrong.
client-safe-folder-failed =
    WinAppMigrator Client, yalnızca yöneticilerin değiştirebildiği bir klasörden
    başlatılamadı: { $error }
client-press-enter-to-close = Bu pencereyi kapatmak için Enter tuşuna basın.
# The first line of its screen. { $version } is the client's version, such as 0.6.2.
client-heading = WinAppMigrator Client { $version }
client-intro =
    Bu araç sayesinde yeni bilgisayarınızdaki WinAppMigrator, bu bilgisayardaki
    uygulamaları ağınız üzerinden okuyabilir. Burada hiçbir şeyi değiştirmez:
    Windows sürücüsünün anlık görüntüsünü okur ve siz durdurduğunuzda bu anlık
    görüntüyü kaldırır.
# { $error } is why: one of the messages under "Why it stopped" below, or what Windows said.
client-stopped = İstemci durdu: { $error }
# The caretaker is a copy of the client with no window that cleans up after it. { $error } is what Windows said.
client-caretaker-failed =
    Bu pencere durdurulmak yerine kapatılırsa, anlık görüntü ve güvenlik duvarı
    kuralları istemci burada yeniden çalıştırılana kadar kalır ({ $error }).
# Shown while the snapshot is taken, followed on the same line by client-snapshot-taken. Keep the quotes: they keep
# the space before it.
client-snapshot-taking = Windows sürücüsünün anlık görüntüsü alınıyor...{ " " }
client-snapshot-taken = tamamlandı.
# The old PC answers the new one's search for it on the network. { $error } is what Windows said went wrong.
client-discovery-failed =
    Bu bilgisayar ağda kendiliğinden görünmeyecek ({ $error }).
    Bunun yerine yeni bilgisayarda bu bilgisayarın adını veya adresini yazın.

## This PC and its pairing code. The three lines are lined up in two columns, behind two spaces; keep the quotes, which
## keep those spaces.

# { $computer_name } is this PC's name, such as OLD-PC.
client-this-pc = { "  " }Bu bilgisayar:    { $computer_name }
# { $windows_name } is this PC's Windows, such as Windows 7 Professional Service Pack 1. { $bits } is 32 or 64.
client-windows = { "  " }Windows:          { $windows_name }, { $bits } bit
# { $code } is six digits in two groups, such as 482 913.
client-pairing-code = { "  " }Eşleştirme kodu:  { $code }
# Network PC... is the button of that name in WinAppMigrator: use the name it has there in your language.
client-pairing-steps =
    Yeni bilgisayarınızda, WinAppMigrator'da Ağ bilgisayarı... düğmesini seçin
    ve bu kodu yazın.
    Önce taşıdığınız uygulamayı kapatın; böylece dosyaları bıraktığı gibi kalır.
    Durdurmak için burada Enter tuşuna basın veya bu pencereyi kapatın.

## While it runs. { $from } is the network address of the PC that connected, such as 192.168.1.20, or "unknown" when
## it could not be told, which the [unknown] variant words.

client-paired =
    { $from ->
        [unknown] Başka bir bilgisayar
       *[address] { $from }
    } ile eşleştirildi. Gerekenler okunuyor...
# { $count } is how many requests it answered.
client-builder-done =
    { $from ->
        [unknown] Başka bir bilgisayar
       *[address] { $from }
    } işini tamamladı ({ $count ->
        [one] { $count } istek
       *[other] { $count } istek
    } yanıtlandı).
# { $error } is what went wrong, often in Windows' words.
client-link-ended =
    { $from ->
        [unknown] Başka bir bilgisayar
       *[address] { $from }
    } ile bağlantı sona erdi: { $error }
client-wrong-code =
    { $from ->
        [unknown] Başka bir bilgisayar
       *[address] { $from }
    } bu bilgisayarın kodu olmayan bir kod denedi.
# { $error } is what went wrong, often in Windows' words.
client-pair-failed =
    { $from ->
        [unknown] Başka bir bilgisayar
       *[address] { $from }
    } ile eşleştirme yapılamadı: { $error }
client-too-many-wrong-codes =
    Çok fazla yanlış kod denendi; bu nedenle şu anda hiçbir bilgisayar bununla
    eşleştirilemez. Ağınızdaki bir şey kodu tahmin etmeye çalışıyor olabilir.
    Eşleştirmek için istemciyi durdurup yeniden başlatın ve gösterdiği yeni kodu
    yazın.
# { $code } is six digits in two groups, such as 482 913.
client-new-code = Çok fazla yanlış kod denendi. Yeni eşleştirme kodu: { $code }

## Stopping

client-stopping = Durduruluyor: anlık görüntü ve güvenlik duvarı kuralları kaldırılıyor...
client-stop-done = Tamamlandı.

## Why it stopped, shown after "The client stopped: " (client-stopped), and written to follow it.

client-already-running = istemci bu bilgisayarda zaten çalışıyor veya kapatılmış bir istemcinin ardından temizlik yapıyor. O istemciyi kullanın ya da biraz bekleyip bunu yeniden başlatın.
# { $drive } is the drive Windows is on, such as C:. { $details } is what Windows said, in its own words.
client-snapshot-failed = Windows, { $drive } sürücüsünün anlık görüntüsünü almadı: { $details }
# { $details } is what Windows Firewall said, in its own words.
client-firewall-failed = Windows Güvenlik Duvarı, istemcinin bağlantıları dinlemesine izin vermedi: { $details }

## What it takes, shown for --help. The options, such as --lang, are typed as they are and are not translated; keep
## their descriptions lined up.

client-usage =
    WinAppMigrator Client şu seçenekleri alır:
    
      --lang <dil>       İstemcide varsa, Windows görüntüleme dili yerine bu dili
                         (örneğin it-IT) kullanır.
      --code <kod>       Bu altı haneli eşleştirme koduyla başlar. Yeni
                         bilgisayarda WinAppMigrator uygulamasını da başlatan
                         betikler içindir.
      --help             Bu yardımı gösterir.
