# Words the core writes for people: warnings, refusals, errors, and the installer's list of what it sets up.
# Message names start with core-.


## Warnings a package records when it is built, which an installer shows later, maybe on another PC, in that PC's
## language.

# { $count } is how many shortcuts.
core-warning-shortcuts-unreadable =
    { $count ->
        [one] Bu sürücüdeki { $count } kısayol hiçbir dosyayı göstermiyor, bu nedenle bu uygulamayı açıp açmadığı belirlenemedi.
       *[other] Bu sürücüdeki { $count } kısayol hiçbir dosyayı göstermiyor, bu nedenle bu uygulamayı açıp açmadıkları belirlenemedi.
    }

## Undoing an install: what an uninstall, or a failed install, says it put back, left, or could not undo. Each line is
## shown on its own, in a list.

# { $app_name } is the app's name.
core-rollback-record-damaged = { $app_name } uygulamasının kaydının bir kısmı okunamadı; bu nedenle o kısmın açıkladığı her şey yerinde kalıyor ve üzerine yazılan bir dosya geri gelmeyebilir.
core-rollback-settings-entry-stays = Uygulamanın Ayarlar'daki girdisi kalır; böylece bu işlem oradan yeniden çalıştırılabilir.
# { $name } is a runtime's name, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-rollback-prerequisite-stays = { $name } yüklendi veya yüklenmiş olabilir; başka programlar kullanıyor olabileceğinden kalır.
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-drive-not-connected = Bu yüklemenin { $folder } klasörüne koyduğu şeyler bağlı olmayan bir sürücüde. Sürücüyü bağlayın; böylece bu işlem tamamlanabilir.
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-record-part-unread = Bu yüklemenin { $folder } klasörüne koyduğu şeyler kalır, çünkü kaydın o kısmı okunamadı.

## Why a file or folder was left, shown after "was left:" in the messages below them. Each is part of a sentence, so
## starts with a small letter and has no full stop.

# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-being-changed = bu işlem çalışırken bir şey { $folder } klasörünü değiştiriyordu, bu nedenle orada başka bir şey yapılmadı. Bunu yapan şeyi kapatın ve bu işlemi yeniden çalıştırın
# { $folder } is a folder; { $error } is what Windows said went wrong.
core-rollback-reason-not-opened = { $folder } klasörü açılamadı: { $error }
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-no-record = { $folder } klasörünün nerede olduğuna dair bir kayıt yok
# { $folder } is a folder; { $actual } is the folder it now leads to.
core-rollback-reason-moved = { $folder } klasörü artık yükleme sırasındaki yere değil, { $actual } konumuna gidiyor
# Why a service was not stopped.
core-rollback-reason-did-not-stop = zamanında durmadı

## Files and folders

# { $path } is a folder; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-folder-left = { $path } klasörü yerinde bırakıldı: { $reason }.
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-removed = { $path } klasörü kaldırıldı
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-holds-others = { $path } klasörü kalır, çünkü bu yüklemenin oraya koymadığı dosyalar içeriyor.
# { $path } is a folder; { $error } is what Windows said went wrong.
core-rollback-folder-not-removed = { $path } klasörü kaldırılamadı: { $error }.
# { $path } is a file; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-file-left = { $path } dosyası yerinde bırakıldı: { $reason }.
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-folder-gone = { $path } eski haline getirilmiyor, çünkü altında bulunduğu klasör artık yok.
# { $path } is a file; { $error } is what went wrong.
core-rollback-partial-not-removed = Yarım kalmış bir dosya kaldırılamadı: { $path } ({ $error }).
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-checked = { $path } denetlenemedi: { $error }.
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-removed = { $path } kaldırıldı
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-removed = { $path } kaldırılamadı: { $error }.
# { $path } is a file this install had replaced, and has put back.
core-rollback-file-put-back = { $path } eski haline getirildi
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-put-back = { $path } eski haline getirilemedi: { $error }.
# Why a file could not be put back.
core-rollback-backup-gone = yedek artık yok
# { $path } is a file.
core-rollback-no-parent-folder = { $path } için üst klasör yok

## A file that stays. { $why } says why: still-used (something that stays still uses it), changed (it has changed since
## it was installed), or not-asked (it is someone's data, and they did not ask for it to go). { $path } is the file.

core-rollback-file-stays =
    { $why ->
        [still-used] { $path } kalır, çünkü kalan bir şey onu hâlâ kullanıyor.
        [changed] { $path } kalır, çünkü yüklendikten sonra değişti.
       *[not-asked] { $path } kalır, çünkü kaldırılması istenmedi.
    }
# The file it replaced was to be put beside it, under another name, and every name was taken.
core-rollback-file-stays-no-name-beside =
    { $why ->
        [still-used] { $path } kalır, çünkü kalan bir şey onu hâlâ kullanıyor; üzerine yazdığı dosya da yanına konamadı: O dosya için kullanılabilecek tüm adlar alınmış.
        [changed] { $path } kalır, çünkü yüklendikten sonra değişti; üzerine yazdığı dosya da yanına konamadı: O dosya için kullanılabilecek tüm adlar alınmış.
       *[not-asked] { $path } kalır, çünkü kaldırılması istenmedi; üzerine yazdığı dosya da yanına konamadı: O dosya için kullanılabilecek tüm adlar alınmış.
    }
# { $name } is the name the file it replaced now has, such as settings.ini.before-install.
core-rollback-file-stays-beside =
    { $why ->
        [still-used] { $path } kalır, çünkü kalan bir şey onu hâlâ kullanıyor. Üzerine yazdığı dosya, { $name } adıyla yanında duruyor.
        [changed] { $path } kalır, çünkü yüklendikten sonra değişti. Üzerine yazdığı dosya, { $name } adıyla yanında duruyor.
       *[not-asked] { $path } kalır, çünkü kaldırılması istenmedi. Üzerine yazdığı dosya, { $name } adıyla yanında duruyor.
    }
# { $error } is what went wrong.
core-rollback-file-stays-not-beside =
    { $why ->
        [still-used] { $path } kalır, çünkü kalan bir şey onu hâlâ kullanıyor; üzerine yazdığı dosya da yanına konamadı: { $error }.
        [changed] { $path } kalır, çünkü yüklendikten sonra değişti; üzerine yazdığı dosya da yanına konamadı: { $error }.
       *[not-asked] { $path } kalır, çünkü kaldırılması istenmedi; üzerine yazdığı dosya da yanına konamadı: { $error }.
    }

## The registry and environment variables. { $kind } says what { $name } is: key (a registry key, such as
## HKLM\Software\App), value (a registry value, such as HKLM\Software\App\Version), user-variable or machine-variable
## (an environment variable of the user's or of the whole PC, such as APP_HOME), or user-list or machine-list (a
## variable that is a list of folders, such as Path). { $error } is what Windows said went wrong.

# How a registry key's unnamed value is shown, as Registry Editor shows it.
core-registry-default-value-name = (Varsayılan)
core-rollback-setting-other-account =
    { $kind ->
        [key] { $name } anahtarı kalır, çünkü bunu yükleyen ve şu anda oturum açmamış olan hesaba ait.
        [user-variable] { $name } kullanıcı ortam değişkeni kalır, çünkü bunu yükleyen ve şu anda oturum açmamış olan hesaba ait.
        [machine-variable] { $name } sistem ortam değişkeni kalır, çünkü bunu yükleyen ve şu anda oturum açmamış olan hesaba ait.
        [user-list] { $name } kullanıcı değişkeni kalır, çünkü bunu yükleyen ve şu anda oturum açmamış olan hesaba ait.
        [machine-list] { $name } sistem değişkeni kalır, çünkü bunu yükleyen ve şu anda oturum açmamış olan hesaba ait.
       *[value] { $name } kayıt defteri değeri kalır, çünkü bunu yükleyen ve şu anda oturum açmamış olan hesaba ait.
    }
core-rollback-setting-not-reached =
    { $kind ->
        [key] { $name } anahtarına erişilemedi: { $error }.
        [user-variable] { $name } kullanıcı ortam değişkenine erişilemedi: { $error }.
        [machine-variable] { $name } sistem ortam değişkenine erişilemedi: { $error }.
        [user-list] { $name } kullanıcı değişkenine erişilemedi: { $error }.
        [machine-list] { $name } sistem değişkenine erişilemedi: { $error }.
       *[value] { $name } kayıt defteri değerine erişilemedi: { $error }.
    }
core-rollback-setting-not-read =
    { $kind ->
        [key] { $name } anahtarı okunamadı: { $error }.
        [user-variable] { $name } kullanıcı ortam değişkeni okunamadı: { $error }.
        [machine-variable] { $name } sistem ortam değişkeni okunamadı: { $error }.
        [user-list] { $name } kullanıcı değişkeni okunamadı: { $error }.
        [machine-list] { $name } sistem değişkeni okunamadı: { $error }.
       *[value] { $name } kayıt defteri değeri okunamadı: { $error }.
    }
core-rollback-setting-changed =
    { $kind ->
        [user-variable] { $name } kullanıcı ortam değişkeni kalır, çünkü yüklendikten sonra değişti.
        [machine-variable] { $name } sistem ortam değişkeni kalır, çünkü yüklendikten sonra değişti.
        [user-list] { $name } kullanıcı değişkeni kalır, çünkü yüklendikten sonra değişti.
        [machine-list] { $name } sistem değişkeni kalır, çünkü yüklendikten sonra değişti.
       *[value] { $name } kayıt defteri değeri kalır, çünkü yüklendikten sonra değişti.
    }
# Said when it is back as it was before the install. The capital "The" is as the app has always said it.
core-rollback-setting-put-back =
    { $kind ->
        [user-variable] { $name } kullanıcı ortam değişkeni eski haline getirildi
        [machine-variable] { $name } sistem ortam değişkeni eski haline getirildi
        [user-list] { $name } kullanıcı değişkeni eski haline getirildi
        [machine-list] { $name } sistem değişkeni eski haline getirildi
       *[value] { $name } kayıt defteri değeri eski haline getirildi
    }
core-rollback-setting-not-put-back =
    { $kind ->
        [user-variable] { $name } kullanıcı ortam değişkeni eski haline getirilemedi: { $error }.
        [machine-variable] { $name } sistem ortam değişkeni eski haline getirilemedi: { $error }.
        [user-list] { $name } kullanıcı değişkeni eski haline getirilemedi: { $error }.
        [machine-list] { $name } sistem değişkeni eski haline getirilemedi: { $error }.
       *[value] { $name } kayıt defteri değeri eski haline getirilemedi: { $error }.
    }
# A registry value the install wrote, which is the user's data, and stays unless they asked for it to go.
core-rollback-value-kept-by-request = { $name } kayıt defteri değeri kalır, çünkü kaldırılması istenmedi.
core-rollback-key-holds-others = { $name } anahtarı kalır, çünkü bu yüklemenin oraya koymadığı şeyler içeriyor.
core-rollback-key-removed = { $name } anahtarı kaldırıldı
core-rollback-key-not-removed = { $name } anahtarı kaldırılamadı: { $error }.
# { $kind } is user-list or machine-list. The capital "The" is as the app has always said it.
core-rollback-list-entries-removed =
    { $kind ->
        [user-list] Bu yüklemenin girdileri { $name } kullanıcı değişkeninden çıkarıldı
       *[machine-list] Bu yüklemenin girdileri { $name } sistem değişkeninden çıkarıldı
    }
core-rollback-list-entries-not-removed =
    { $kind ->
        [user-list] Bu yüklemenin girdileri { $name } kullanıcı değişkeninden çıkarılamadı: { $error }.
       *[machine-list] Bu yüklemenin girdileri { $name } sistem değişkeninden çıkarılamadı: { $error }.
    }

## Services and drivers. { $kind } is service or driver; { $name } is its name, such as AppUpdater. { $error } and
## { $reason } are what Windows said went wrong.

core-rollback-service-not-read =
    { $kind ->
        [driver] { $name } sürücüsü okunamadı: { $error }.
       *[service] { $name } hizmeti okunamadı: { $error }.
    }
core-rollback-service-changed =
    { $kind ->
        [driver] { $name } sürücüsü kalır, çünkü yüklendikten sonra değişti.
       *[service] { $name } hizmeti kalır, çünkü yüklendikten sonra değişti.
    }
core-rollback-service-not-stopped =
    { $kind ->
        [driver] { $name } sürücüsü durdurulamadı ({ $reason }), bu nedenle kullandığı şeyler bilgisayar yeniden başlatılana kadar kalabilir.
       *[service] { $name } hizmeti durdurulamadı ({ $reason }), bu nedenle kullandığı şeyler bilgisayar yeniden başlatılana kadar kalabilir.
    }
core-rollback-service-stopped-for-this =
    { $kind ->
        [driver] { $name } sürücüsü bu işlem için durduruldu. Gerektiğinde yeniden başlatın.
       *[service] { $name } hizmeti bu işlem için durduruldu. Gerektiğinde yeniden başlatın.
    }
core-rollback-service-not-removed =
    { $kind ->
        [driver] { $name } sürücüsü kaldırılamadı: { $reason }.
       *[service] { $name } hizmeti kaldırılamadı: { $reason }.
    }
core-rollback-service-marked-for-removal =
    { $kind ->
        [driver] { $name } sürücüsü kaldırılmak üzere işaretlendi; bilgisayar yeniden başlatıldığında veya onu açık tutan şey onu bıraktığında kaldırılır.
       *[service] { $name } hizmeti kaldırılmak üzere işaretlendi; bilgisayar yeniden başlatıldığında veya onu açık tutan şey onu bıraktığında kaldırılır.
    }
core-rollback-service-removed =
    { $kind ->
        [driver] { $name } sürücüsü kaldırıldı
       *[service] { $name } hizmeti kaldırıldı
    }
core-rollback-service-not-restored =
    { $kind ->
        [driver] { $name } sürücüsü eski haline getirilemedi: { $reason }.
       *[service] { $name } hizmeti eski haline getirilemedi: { $reason }.
    }
core-rollback-service-restored =
    { $kind ->
        [driver] { $name } sürücüsünün ayarları eski haline getirildi
       *[service] { $name } hizmetinin ayarları eski haline getirildi
    }
core-rollback-service-not-restarted = { $name } hizmeti bu yüklemeden önce çalışıyordu ve yeniden başlamadı: { $reason }.
# { $account } is the account the service ran as, such as CONTOSO\backup.
core-rollback-service-account-again = { $name } hizmeti { $account } olarak çalışıyordu. Parolası saklanmadığından hesabını Hizmetler'de yeniden ayarlayın.

## Firewall rules. { $name } is the rule's name; { $error } is what went wrong.

core-rollback-firewall-not-read = { $name } güvenlik duvarı kuralı okunamadı: { $error }.
core-rollback-firewall-says-otherwise = { $name } güvenlik duvarı kuralı kalır, çünkü orada bu adla bulunan kural başka bir şey belirtiyor.
core-rollback-firewall-changed = { $name } güvenlik duvarı kuralı kalır, çünkü yüklendikten sonra değişti.
# The capital "The" is as the app has always said it.
core-rollback-firewall-removed = { $name } güvenlik duvarı kuralı kaldırıldı
core-rollback-firewall-not-removed = { $name } güvenlik duvarı kuralı kaldırılamadı: { $error }.

## Scheduled tasks. { $path } is a folder of tasks, and { $name } a task with its folder, such as \App\Nightly Update.
## { $error } and { $reason } are what went wrong.

core-rollback-task-folder-holds-others = { $path } görev klasörü kalır, çünkü bu yüklemenin oraya koymadığı görevler içeriyor.
core-rollback-task-folder-not-removed = { $path } görev klasörü kaldırılamadı: { $error }.
core-rollback-task-not-read = { $name } görevi okunamadı: { $error }.
core-rollback-task-changed = { $name } görevi kalır, çünkü yüklendikten sonra değişti.
core-rollback-task-removed = { $name } görevi kaldırıldı
core-rollback-task-not-removed = { $name } görevi kaldırılamadı: { $reason }.
core-rollback-task-put-back = { $name } görevi eski haline getirildi
core-rollback-task-not-put-back = { $name } görevi eski haline getirilemedi: { $reason }.

## When an install cannot be undone at all

# { $error } is what went wrong.
core-rollback-record-unreadable = Bu yüklemenin kaydı okunamadı, bu nedenle hiçbir şey geri alınmadı: { $error }.
# { $error } is what went wrong.
core-rollback-nothing-undone = Hiçbir şey geri alınmadı: { $error }.
# One app's part of the message below. { $app_name } is the app's name; { $failures } is what could not be undone, a
# sentence or more; { $folder } is the folder that holds the install's record.
core-rollback-earlier-install-stuck = { $app_name }: { $failures } Hiçbir zaman geri alınamazsa, bir yönetici { $folder } klasörünü kaldırarak bu yüklemeden vazgeçebilir.
# Why an install did not start. { $details } is one or more of the message above.
core-rollback-earlier-installs-not-undone = hiç tamamlanmamış önceki bir yükleme tamamen geri alınamadı, bu nedenle hiçbir şey yüklenmedi. Bunları kullanan her şeyi kapatın ve yeniden deneyin. { $details }
core-rollback-install-busy = bu yükleme başka bir yükleyici veya kaldırma programı tarafından değiştiriliyor

## The summary an uninstall writes on the command line, one line each, with what each says below it. { $count } is how
## many.

core-rollback-count-undone = Geri alındı: { $count }
core-rollback-count-failed = Geri alınamadı: { $count }
core-rollback-count-kept = Yerinde bırakıldı: { $count }
core-rollback-count-left-behind = Geride bırakıldı: { $count }

## Which Windows a PC or drive has, as the app shows it beside the drive.

# { $name } is the Windows' name, such as Windows 7 Professional Service Pack 1.
core-system-windows-32-bit = { $name }, 32 bit
# { $name } is the Windows' name, such as Windows 11 Pro.
core-system-windows-64-bit = { $name }, 64 bit
# A drive's Windows, when it only says it is older than Vista.
core-system-windows-xp-or-older = Windows XP veya daha eski

## Reading a connected drive, or another account's settings. Each is why it could not be done, and some start with a
## small letter, as they follow other words.

core-system-needs-administrator = bağlı bir sürücüyü veya başka bir hesabın ayarlarını okumak için yönetici hakları gerekir
core-system-no-program-data = bu bilgisayarın ProgramData klasörü bulunamadı
# { $path } is a file, such as D:\Windows\System32\config\SOFTWARE.
core-system-not-a-hive-file = bir kovan dosyası değil: { $path }
# { $path } is a file; { $size } is its size in bytes.
core-system-hive-too-large = { $path } dosyasının boyutu { $size } bayt; bir kayıt defteri kovanı için fazla büyük veya bu sürücünün kovanlarının toplamından fazla
# { $key } is a registry key, such as Software\Microsoft\Windows\CurrentVersion.
core-system-hive-no-mapping = çevrimdışı kovanda HKLM\{ $key } için eşleme yok
# { $key } is where a drive's registry was to be loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said.
# "reg load" is a command's name.
core-system-hive-not-loaded = reg load { $key } başarısız oldu: { $error }
# { $key } is where a drive's registry was loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said. "reg
# unload" is a command's name.
core-system-hive-not-unloaded = reg unload { $key } başarısız oldu: { $error }
# { $drive } is a drive, such as D:\.
core-system-not-a-system-drive = { $drive } bir Windows sistem sürücüsü değil (\Windows klasörü yok).
# { $drive } is a drive, such as C:\. "This PC" is the name of a choice in the app.
core-system-own-system-drive = { $drive } bu bilgisayarın kendi sistem sürücüsü. Bu bilgisayarı okumak için Bu bilgisayar seçeneğini belirleyin.
# { $drive } is a drive, such as D:\.
core-system-registry-through-link = { $drive } bir Windows sistem sürücüsü değil: Kayıt defterine giden yol bir bağlantı üzerinden geçiyor.

## Warnings a scan records about whose data it read.

# { $account } is an account's name or id, such as alice or S-1-5-21-...-1001.
core-warning-scan-account-not-on-record = { $account } hesabı burada kayıtlı değil, bu nedenle hiç kimsenin kullanıcı başına verileri veya ayarları eklenmedi.
# { $account } is an account's name, such as CONTOSO\alice.
core-warning-scan-signed-in-account-no-profile = Burada oturum açmış olan { $account } hesabının kayıtlı bir profili yok, bu nedenle hiç kimsenin kullanıcı başına verileri veya ayarları eklenmedi.
# { $reason } is why Windows could not say.
core-warning-scan-signed-in-account-unknown = Kimin oturum açtığı belirlenemedi ({ $reason }), bu nedenle hiç kimsenin kullanıcı başına verileri veya ayarları eklenmedi. Kimin verilerinin ekleneceğini seçin.
core-warning-scan-last-account-unknown = Bu sürücü en son kimin oturum açtığını belirtmiyor ve birden fazla hesap içeriyor, bu nedenle hiç kimsenin kullanıcı başına verileri veya ayarları eklenmedi. Kimin verilerinin ekleneceğini seçin.
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-settings-unreadable = { $account } hesabının ayarları okunamadı, bu nedenle eklenmedi: { $error }
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-classes-unreadable = { $account } hesabı için kaydedilen COM sınıfları ve dosya türleri okunamadı, bu nedenle eklenmedi: { $error }

## Paths a scan did not follow. { $count } is how many; { $paths } is some of them, separated by semicolons; { $more } is
## how many more there are, not shown.

core-warning-scan-drive-paths-not-followed =
    { $count ->
        [one] Bu sürücüde kayıtlı bir yol okunamadı, bu nedenle oradaki hiçbir şey eklenmedi. Bu yol başka bir diskte, tam bir yol değil veya sürücünün dışına çıkacak bir bağlantı üzerinden gidiyor: { $paths }
       *[other] Bu sürücüde kayıtlı bazı yollar okunamadı, bu nedenle oralardaki hiçbir şey eklenmedi. Bu yolların her biri başka bir diskte, tam bir yol değil veya sürücünün dışına çıkacak bir bağlantı üzerinden gidiyor: { $paths }
    }
core-warning-scan-drive-paths-not-followed-more = Bu sürücüde kayıtlı bazı yollar okunamadı, bu nedenle oralardaki hiçbir şey eklenmedi. Bu yolların her biri başka bir diskte, tam bir yol değil veya sürücünün dışına çıkacak bir bağlantı üzerinden gidiyor: { $paths } ve { $more } tane daha
core-warning-scan-network-paths-not-followed =
    { $count ->
        [one] Bu bilgisayarda kayıtlı bir yol okunamadı, bu nedenle oradaki hiçbir şey eklenmedi. Bu yol, bu bilgisayarın bir diskinde değil (ona bakmak oraya bağlanmak anlamına gelir) veya başka bir şeyi açacak bir ada sahip: { $paths }
       *[other] Bu bilgisayarda kayıtlı bazı yollar okunamadı, bu nedenle oralardaki hiçbir şey eklenmedi. Bu yolların her biri bu bilgisayarın bir diskinde değil (onlara bakmak oraya bağlanmak anlamına gelir) veya başka bir şeyi açacak bir ada sahip: { $paths }
    }
core-warning-scan-network-paths-not-followed-more = Bu bilgisayarda kayıtlı bazı yollar okunamadı, bu nedenle oralardaki hiçbir şey eklenmedi. Bu yolların her biri bu bilgisayarın bir diskinde değil (onlara bakmak oraya bağlanmak anlamına gelir) veya başka bir şeyi açacak bir ada sahip: { $paths } ve { $more } tane daha

## What a scan with none of an account's data leaves out of a package, in the app's list of what was left out. Each
## has a name, and below it why.

core-account-left-out-shortcuts = Bu hesabın kendi klasörlerindeki kısayollar
# { $count } is how many shortcuts.
core-account-left-out-shortcuts-why =
    { $count ->
        [one] Yalnızca Kullanıcı verileri seçeneğiyle eklenen { $count } kısayol
       *[other] Yalnızca Kullanıcı verileri seçeneğiyle eklenen { $count } kısayol
    }
core-account-left-out-variables = Bu hesaba ait ortam değişkenleri
# { $count } is how many variables.
core-account-left-out-variables-why =
    { $count ->
        [one] Yalnızca Kullanıcı verileri seçeneğiyle eklenen { $count } değişken
       *[other] Yalnızca Kullanıcı verileri seçeneğiyle eklenen { $count } değişken
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-account-left-out-task = { $task } zamanlanmış görevi
core-account-task-works-on-account-folder = Bir hesabın klasörlerindeki bir yer üzerinde çalışıyor veya böyle bir yerin adını taşıyor; yalnızca Kullanıcı verileri seçeneğiyle eklenir
core-account-task-runs-from-account-folder = Bir hesabın klasörlerinden ayrıca çalıştırdıkları yalnızca Kullanıcı verileri seçeneğiyle eklenir
# { $service } is a service's name, such as AppUpdater.
core-account-left-out-service = { $service } hizmeti
core-account-service-works-on-account-folder = Bir hesabın klasörlerindeki bir yer üzerinde çalışıyor; yalnızca Kullanıcı verileri seçeneğiyle eklenir

## What a package was found to still say of whose PC it came from, after a build with none of an account's data.

# The parts of a package, as the warning below names them.
core-account-part-files = uygulamanın kendi dosyalarının yolları
core-account-part-registry = kayıt defteri değerleri
core-account-part-settings-entry = uygulamanın Ayarlar'daki girdisi
core-account-part-main-program = ana programının konumu
core-account-part-shortcuts = kısayollar
core-account-part-tasks = zamanlanmış görevler
core-account-part-services = hizmetler
core-account-part-variables = ortam değişkenleri
# One place in the list below. { $place } is one of the parts above, or a file's name; { $count } is how many times.
core-account-mention-place = { $place } ({ $count })
# { $kind } is what was found: account (an account's name), folder (where an account's folder is), computer (the PC's
# name), or id (an account's id). { $places } is a list of where, from the message above.
core-warning-scan-package-mentions =
    { $kind ->
        [account] Bu paket şu yerlerde bir hesabın adını içeriyor: { $places }. Paket hiçbir hesabın verileri olmadan oluşturuldu ve yükleyiciye sahip olan herkes bunu okuyabilir.
        [folder] Bu paket şu yerlerde bir hesabın klasörünün konumunu içeriyor: { $places }. Paket hiçbir hesabın verileri olmadan oluşturuldu ve yükleyiciye sahip olan herkes bunu okuyabilir.
        [computer] Bu paket şu yerlerde bilgisayarın adını içeriyor: { $places }. Paket hiçbir hesabın verileri olmadan oluşturuldu ve yükleyiciye sahip olan herkes bunu okuyabilir.
       *[id] Bu paket şu yerlerde bir hesabın kimliğini içeriyor: { $places }. Paket hiçbir hesabın verileri olmadan oluşturuldu ve yükleyiciye sahip olan herkes bunu okuyabilir.
    }
# { $count } is how many folders.
core-warning-scan-folders-of-others-unreadable =
    { $count ->
        [one] Uygulamanın dışındaki { $count } klasör okunamadı, bu nedenle uygulamanın oradaki verileri eksik olabilir.
       *[other] Uygulamanın dışındaki { $count } klasör okunamadı, bu nedenle uygulamanın oralardaki verileri eksik olabilir.
    }

## What a scan leaves out because it looks like a credential, or is a license with the License option off, in the app's
## list of what was left out. Each has a name, and below it why.

core-secret-why-credential = kimlik bilgisine benziyor
core-secret-why-license-option-off = Lisans ve etkinleştirme verileri seçeneği kapalı
# { $count } is how many values of a registry key.
core-secret-why-account-values =
    { $count ->
        [one] Yalnızca Kullanıcı verileri seçeneğiyle eklenen { $count } değer
       *[other] Yalnızca Kullanıcı verileri seçeneğiyle eklenen { $count } değer
    }
# Shown in place of a registry value's name when its name itself looks like a password or a key.
core-secret-name-holds-credential = (adı kimlik bilgisi içeren bir değer)
# Shown in place of an option's name, for an argument on a command line that has no name.
core-secret-unnamed-argument = kimlik bilgisi içeren bir bağımsız değişken
# { $option } is an option taken out of a command line, such as /password, or the message above. { $place } says
# where the command line was: service, task, shortcut, opener (what opens a file type), opener-unnamed (the same, of a
# type with no name), or uninstall. { $name } is the service, task, shortcut, file type, or app.
core-secret-taken-from =
    { $place ->
        [service] { $option } ({ $name } hizmetinin çalıştırılma komutunda)
        [task] { $option } ({ $name } görevinin çalıştırılma komutunda)
        [shortcut] { $option } ({ $name } kısayolunda)
        [opener] { $option } ({ $name } türünü açan komutta)
        [opener-unnamed] { $option } (bir dosya türünü açan komutta)
       *[uninstall] { $option } ({ $name } uygulamasının kaldırma komutunda)
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-secret-task-definition-unreadable = { $task } görevinin, denetlemek için okunamayan tanımı
# { $name } is an environment variable's name, such as APP_TOKEN.
core-secret-left-out-variable = { $name } ortam değişkeni
# { $class } is a COM class's id, such as {00000000-0000-0000-0000-000000000000}.
core-secret-left-out-class-server = { $class } sınıfının çalıştırdığını belirttiği şey
# { $rule } is a firewall rule's name.
core-secret-left-out-firewall-description = { $rule } güvenlik duvarı kuralının açıklaması

## Licenses a scan found, as the package's report describes each.

# { $name } is a file's name, such as license.lic.
core-license-detail-file = Lisans dosyası algılandı: { $name }
# { $framework } is a licensing product's name, such as FlexLM / FlexNet.
core-license-detail-store = { $framework } lisans deposu mevcut
# { $name } is an environment variable's name, such as LM_LICENSE_FILE.
core-license-detail-variable = Lisans ortam değişkeni: { $name }
# { $name } is a registry value's name, such as SerialNumber.
core-license-detail-registry-value = Kayıt defteri değeri lisans verilerine işaret ediyor: { $name }
# Said of a registry value in an account's key, which is not named.
core-license-detail-registry-value-of-an-account = Kayıt defteri değeri lisans verilerine işaret ediyor
# { $key } is a registry key, such as HKLM\Software\Contoso\License.
core-license-detail-registry-key = Kayıt defteri anahtarı lisans verilerine işaret ediyor: { $key }

## Warnings a scan records about licenses.

core-warning-scan-license-data = Lisans/etkinleştirme verileri algılandı. Lisans ve etkinleştirme verileri seçeneği açıkken yükleyici taşınabilecek olanları geri yükler; makine genelindeki değerler ve donanıma bağlı depolar yalnızca bildirilir. WinAppMigrator hiçbir zaman şifre çözmez, etkinleştirmeyi atlatmaz veya donanım kilitlerine (dongle) öykünmez.
core-warning-scan-license-machine-bound = Bazı lisanslar makineye bağlı (donanım kilidi, makineye kilitli etkinleştirme veya şifrelenmiş). Yükleyici yapabildiğini geri yükler; bunlar yeni bilgisayarda yine de yeniden etkinleştirme gerektirebilir.

## Warnings a scan records about the registry. What could not be read is not named in them: it is kept beside them.

# { $error } is what Windows said went wrong. App Paths is a registry key's name.
core-warning-scan-app-paths-key-unreadable = App Paths alt anahtarı okunamadı: { $error }
core-warning-scan-registry-limit = Kayıt defterinin alınması, yapılandırılmış güvenlik sınırına ulaştı.
core-warning-scan-registry-subtree-limit = Kayıt defteri alt ağacının alınması, yapılandırılmış güvenlik sınırına ulaştı.
# { $error } is what Windows said went wrong.
core-warning-scan-registry-key-unreadable = Kayıt defteri anahtarı okunamadı: { $error }
core-warning-scan-registry-value-too-large = Kayıt defteri değeri, yapılandırılmış alma boyutu sınırını aştı.

## Scheduled tasks

core-warning-scan-tasks-folder-through-link = Bu sürücüdeki zamanlanmış görevler klasörü bir bağlantı üzerinden gidiyor, bu nedenle görevleri okunmadı.
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-warning-scan-task-left-out = { $task } zamanlanmış görevi dışarıda bırakıldı: Ayarları eklenen hesaptan başka bir hesapla çalışıyor, dosyasını yöneticiler dışındakiler değiştirebiliyor veya uygulamanın saklanamayacak kadar çok görevi var.
# { $error } is what Windows said went wrong.
core-warning-scan-tasks-folder-unreadable = Zamanlanmış görevlerin bir kısmı okunamadı: { $error }
# { $path } is a task's file, such as C:\Windows\System32\Tasks\App\Nightly Sync; { $reason } is one of the reasons
# below, or what Windows said went wrong.
core-warning-scan-task-unreadable = { $path } zamanlanmış görevi okunamadı: { $reason }. Uygulamayı çalıştırıyorsa eklenmedi.
# Why a task's file could not be read. { $size } is its size in bytes.
core-task-too-large = boyutu { $size } bayt; bir görev olamayacak kadar büyük
# Why a task's file could not be read.
core-task-ends-mid-character = bir karakterin ortasında bitiyor
# Why a task's file could not be read. { $error } says where.
core-task-not-utf16 = geçerli UTF-16 değil: { $error }
# Why a task's file could not be read. { $error } says where.
core-task-not-text = geçerli UTF-8 veya UTF-16 değil: { $error }
# What a package records of how a task is put back. Register-ScheduledTask is a command's name.
core-task-restore-instructions = Geri yükleme, arşivlenen XML tanımıyla Register-ScheduledTask komutunu kullanır.

## Reading a program's file. Each is why it could not be read, after the program's name.

# { $reason } is what went wrong.
core-pe-unreadable = okunamadı: { $reason }
core-pe-headers-misaligned = üst bilgileri, bir programınkilerin olması gereken sınırlarda değil
# { $size } is the file's size in bytes.
core-pe-too-large = boyutu { $size } bayt; bir program olamayacak kadar büyük
# PE is the name of the format Windows programs are in.
core-pe-no-headers = PE üst bilgileri yok

## Warnings a scan records about what a program needs. Which program is kept beside each.

# { $error } is what went wrong.
core-warning-scan-imports-unreadable = İçeri aktarma tablosu okunamadı: { $error }
# { $error } is what went wrong.
core-warning-scan-import-name-unreadable = Bir içeri aktarma tanımlayıcısının adı okunamadı: { $error }
# { $count } is how many were skipped.
core-warning-scan-legacy-delay-imports =
    { $count ->
        [one] { $count } eski (v1) gecikmeli yükleme tanımlayıcısı atlandı; yalnızca modern, RVA tabanlı gecikmeli içeri aktarmalar ayrıştırılır.
       *[other] { $count } eski (v1) gecikmeli yükleme tanımlayıcısı atlandı; yalnızca modern, RVA tabanlı gecikmeli içeri aktarmalar ayrıştırılır.
    }

## The DLLs a program needs. Which DLL or program is kept beside each warning.

# { $path } is a program, such as C:\Program Files\App\app.exe.
core-dll-program-has-no-folder = yürütülebilir dosyanın üst dizini yok: { $path }
# Why a DLL is among the app's files, in the package's list of them.
core-dll-beside-program = DLL, seçilen yürütülebilir dosyanın yanında bulunuyor
# { $error } is what went wrong.
core-warning-scan-dependency-unscanned = Özyinelemeli bağımlılık taranamadı: { $error }
# { $name } is a DLL's name, such as d3dx9_43.dll.
core-warning-scan-dependency-not-in-newer-windows = { $name }, bu uygulamanın bulunduğu bilgisayarın Windows klasöründeydi ve Windows 10 ile 11 bu dosyayla birlikte gelmez. Onu oraya başka bir şey yüklemiş; bu nedenle uygulamanın kendi dosyalarından biri değil ve gelmez. Bu dosyayı sağlayan şey yüklenene kadar uygulama başlamayabilir.
# { $error } is what went wrong.
core-warning-scan-dependency-not-hashed = Bağımlılığın karması hesaplanamadı: { $error }
# SystemRoot and windir are the names of environment variables.
core-warning-scan-windows-folder-unknown = SystemRoot/windir ortam değişkenleri kullanılamıyor; sistem DLL araması sınırlı.
# { $error } is what went wrong.
core-warning-scan-dependency-architecture-unreadable = Bağımlılığın mimarisi okunamadı: { $error }

## Services, drivers, shortcuts, and environment variables a scan reads. Where it was reading is kept beside each
## warning.

# { $error } is what Windows said went wrong.
core-warning-scan-services-unreadable = Hizmetlerin kayıt defteri anahtarı okunamadı: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-drivers-unreadable = Sürücüleri bulmak için hizmetlerin kayıt defteri anahtarı okunamadı: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-queried = Kısayollar sorgulanamadı: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-parsed = Kısayol sorgusunun çıktısı ayrıştırılamadı: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-environment-unreadable = Sistem ortam değişkenlerinin kayıt defteri anahtarı okunamadı: { $error }

## What a package records of how a driver is put back, which an installer shows. { $kind } is the driver's type as
## Windows names it (kernel, file-system, recognizer, or unknown-type); { $name } is its service's name; { $path } is
## its file. { $signed } is yes when { $signer } names who signed it, and no when that is not known.

core-service-driver-bundled =
    { $signed ->
        [yes] Uygulamaya ait { $kind } sürücüsü “{ $name }” ({ $path }, imzalayan: { $signer }). Yükleyici .sys dosyasını pakete ekler ve sürücü hizmetini otomatik olarak yeniden kaydeder (sc create, yükseltme gerektirir), ardından hizmetin kaydedildiğini doğrular.
       *[no] Uygulamaya ait { $kind } sürücüsü “{ $name }” ({ $path }, imza bilinmiyor). Yükleyici .sys dosyasını pakete ekler ve sürücü hizmetini otomatik olarak yeniden kaydeder (sc create, yükseltme gerektirir), ardından hizmetin kaydedildiğini doğrular.
    }
core-service-driver-from-vendor =
    { $signed ->
        [yes] Üreticiye ait veya Windows ile gelen { $kind } sürücüsü “{ $name }” ({ $path }, imzalayan: { $signer }). WinAppMigrator üçüncü taraf sürücülerini dağıtmaz; uygulamayı başlatmadan önce sürücüyü hedefte üreticinin imzalı sürücü paketiyle yeniden yükleyin.
       *[no] Üreticiye ait veya Windows ile gelen { $kind } sürücüsü “{ $name }” ({ $path }, imza bilinmiyor). WinAppMigrator üçüncü taraf sürücülerini dağıtmaz; uygulamayı başlatmadan önce sürücüyü hedefte üreticinin imzalı sürücü paketiyle yeniden yükleyin.
    }

## Runtimes an app needs, such as the Visual C++ runtime or .NET: how each is provided, as the Dependencies list says.

core-runtime-handling-built-in = Windows'ta yerleşik
core-runtime-handling-ships-with-app = Uygulamayla birlikte gelir
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-handling-windows-feature = Windows özelliği: { $feature }
core-runtime-handling-included = Dahil
# { $id } is the runtime's name in winget, such as Microsoft.VCRedist.2015+.x64.
core-runtime-handling-winget = winget ile yüklenir: { $id }
core-runtime-handling-manual = El ile yükleyin

## Downloading a runtime's installer, when an installer is built. Each is why it could not be, and starts with a small
## letter, as it follows other words. { $url } is a web address; { $error } is what went wrong.

core-runtime-download-not-https = { $url } indirilmiyor: HTTPS değil
# TLS is what keeps a download private.
core-runtime-tls-failed = TLS başlatılamadı: { $error }
core-runtime-download-failed = { $url } indirilemedi: { $error }
core-runtime-download-unreadable = { $url } okunamadı: { $error }
core-runtime-download-empty = { $url } indirmesi hiç veri döndürmedi
# { $came } and { $expected } are numbers of bytes.
core-runtime-download-cut-short = { $url } indirmesi yarıda kesildi: { $expected } bayttan yalnızca { $came } bayt geldi

## Installing a runtime: why it was not installed. Each starts with a small letter, as it follows other words.

core-runtime-no-known-installer = bu çalışma zamanı için bilinen bir yükleyici yok
# { $path } is where in the package, such as prerequisites\vc_redist.x64.exe.
core-runtime-payload-misplaced = paket, bu çalışma zamanının yükleyicisi olarak { $path } yolunu belirtiyor; bu, olması gereken yer değil, bu nedenle çalıştırılmadı
# { $path } is a file.
core-runtime-payload-missing = paketteki yükleyici eksik: { $path }
# { $error } is what went wrong.
core-runtime-payload-path-invalid = paketteki yükleyicinin yolu geçersiz: { $error }
# { $url } is the web address it can be downloaded from. winget is a program's name.
core-runtime-no-installer-route = pakette yükleyici veya winget kimliği yok; şu adresten edinin: { $url }
# { $error } is what went wrong.
core-runtime-no-temp-folder = içinde açılabileceği bir klasör oluşturulamadı: { $error }
# { $reason } is what the check found.
core-runtime-payload-signature-failed = paketteki yükleyici imza denetiminden geçemedi ve çalıştırılmadı: { $reason }
core-runtime-payload-timed-out = paketteki yükleyici zaman aşımı süresi içinde bitmedi ve sonlandırıldı
# { $error } is what went wrong.
core-runtime-payload-not-started = paketteki yükleyici başlatılamadı: { $error }
core-runtime-installer-killed = yükleyici bir çıkış kodu olmadan sonlandırıldı
# { $code } is the number it ended with, such as 1603.
core-runtime-installer-failed = yükleyici { $code } koduyla çıktı
# { $feature } is a Windows feature's name, such as NetFx3; { $code } is the number DISM ended with. Windows Features is
# the name of a Windows window. Keep the command as it is.
core-runtime-feature-failed = DISM, { $feature } Windows özelliğini etkinleştiremedi (çıkış kodu { $code }); yük büyük olasılıkla indirilemedi. Özelliği Windows Özellikleri penceresinden etkinleştirin veya şunu çalıştırın: `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`.
# The same, when DISM gave no number.
core-runtime-feature-failed-no-code = DISM, { $feature } Windows özelliğini etkinleştiremedi (çıkış kodu bilinmiyor); yük büyük olasılıkla indirilemedi. Özelliği Windows Özellikleri penceresinden etkinleştirin veya şunu çalıştırın: `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`.
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-feature-timed-out = { $feature } Windows özelliğinin etkinleştirilmesi zaman aşımı süresi içinde bitmedi ve sonlandırıldı
# { $feature } is a Windows feature's name; { $error } is what went wrong.
core-runtime-feature-no-dism = { $feature } Windows özelliğini etkinleştirmek için DISM kullanılamıyor: { $error }

## Installing a runtime with winget. { $id } is its name in winget, such as Microsoft.VCRedist.2015+.x64.

# { $error } is what went wrong.
core-runtime-winget-not-looked-for = winget aranamadı: { $error }
core-runtime-winget-not-installed = winget yüklü değil
# { $path } is where winget is.
core-runtime-winget-misplaced = { $path }, Windows'un kendi paketlerini tuttuğu yerde değil, bu nedenle çalıştırılmadı
# { $path } is where winget is; { $reason } is what the check found.
core-runtime-winget-signature-failed = { $path } imza denetiminden geçemedi ve çalıştırılmadı: { $reason }
# { $reason } is one of the two messages above, or what went wrong.
core-runtime-winget-unusable = { $id } paketini yüklemek için winget kullanılamadı: { $reason }
# { $code } is the number winget ended with.
core-runtime-winget-failed = winget install { $id } komutu { $code } koduyla çıktı
core-runtime-winget-failed-no-code = winget install { $id } komutu bilinmeyen bir kodla çıktı
core-runtime-winget-timed-out = winget install { $id } komutu zaman aşımı süresi içinde bitmedi ve sonlandırıldı
# { $error } is what went wrong.
core-runtime-winget-not-started = { $id } paketini yüklemek için winget kullanılamıyor: { $error }

## What told a scan how an app was installed, as the package's report lists it. The names of installers and files are
## as their makers write them.

# { $path } is an installer's record of what it installed, such as C:\Program Files\App\unins000.dat.
core-source-record-not-believed = { $path } kullanılmadı: Yöneticiler dışındakiler onu değiştirebilir
# { $count } is how many Windows Installer packages the bundle holds.
core-source-burn-bundle =
    { $count ->
        [one] Burn paket grubu ({ $count } Windows Installer paketi)
       *[other] Burn paket grubu ({ $count } Windows Installer paketi)
    }
# { $count } is how many paths Windows Installer recorded.
core-source-msi-database =
    { $count ->
        [one] Windows Installer veritabanı: { $count } bileşen anahtar yolu
       *[other] Windows Installer veritabanı: { $count } bileşen anahtar yolu
    }
core-source-inno-log = Inno Setup kaldırma günlüğü (unins000.dat)
# { $count } is how many paths the log records.
core-source-inno-log-paths =
    { $count ->
        [one] Inno Setup kaldırma günlüğü: { $count } kayıtlı yol
       *[other] Inno Setup kaldırma günlüğü: { $count } kayıtlı yol
    }
core-source-nsis = NSIS (Nullsoft) kaldırma programı imzası
# { $count } is how many paths the log records.
core-source-nsis-log-paths =
    { $count ->
        [one] NSIS AdvUninstLog uninstall.log: { $count } kayıtlı yol
       *[other] NSIS AdvUninstLog uninstall.log: { $count } kayıtlı yol
    }
core-source-squirrel = Squirrel kullanıcı başına düzeni (Update.exe / app-*)
# { $version } is a folder's name that says the version, such as app-1.2.3.
core-source-squirrel-version = Squirrel geçerli sürümü: { $version }
core-source-chocolatey = Chocolatey paket meta verileri
core-source-msix = MSIX / Appx paketi
# { $identity } is a package's name, such as Contoso.App.
core-source-appx-identity = AppxManifest kimliği: { $identity }
# { $count } is how many folders.
core-source-msix-data =
    { $count ->
        [one] MSIX kullanıcı başına paket verileri: { $count } konum
       *[other] MSIX kullanıcı başına paket verileri: { $count } konum
    }
# App Paths is a registry key's name.
core-source-app-paths = App Paths kaydı

## Why a runtime is needed, as the package's report says it.

# { $program } is a program's file name, such as app.exe; { $dll } is a DLL's, such as msvcp140.dll.
core-runtime-program-imports = { $program }, { $dll } dosyasını içeri aktarıyor

## What else of an app's a scan finds in the registry: fonts and certificates, and why each is the app's.

# Why a font is among the app's files, in the package's list of them.
core-registry-font-recorded = Uygulamanın yükleme kaydında adı geçen bir yazı tipi
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-named-for-the-app = uygulamanın veya yayımcısının adını taşıyor
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-recorded = uygulamanın yükleme kaydında adı geçiyor

## Who is signed in, for the Whose Data choice. Each of the first six is why it could not be told, and starts with a
## small letter, as it is shown inside other words. An account is shown by its id, such as S-1-5-21-...-1001.

# { $error } is what Windows said.
core-profile-desktop-owner-not-said = Windows bu masaüstünün kime ait olduğunu söylemedi: { $error }
# { $error } is what went wrong.
core-profile-desktop-owner-not-asked = bu masaüstünün kime ait olduğu Windows'a sorulamadı: { $error }
core-profile-signed-in-account-no-profile = burada oturum açmış olan { $account } hesabının kayıtlı bir profili yok
core-profile-desktop-not-a-person = buradaki masaüstü, bir kişinin hesabı olmayan { $account } hesabına ait
core-profile-nobody-signed-in-running-account-no-profile = burada kimse oturum açmamış ve bunu çalıştıran { $account } hesabının kayıtlı bir profili yok
core-profile-nobody-signed-in-running-not-a-person = burada kimse oturum açmamış ve bunu çalıştıran { $account } hesabı bir kişinin hesabı değil
# SID is the name Windows gives an account's id.
core-profile-nobody-signed-in-no-sid = burada kimse oturum açmamış ve bu hesabın SID'si yok
# { $accounts } is a list of accounts.
core-profile-several-desktops = bu oturumda birden fazla hesabın masaüstü var: { $accounts }
# Under the Whose Data choice. { $reason } is one of the messages above.
core-profile-signed-in-unknown = Kimin oturum açtığı belirlenemedi ({ $reason }).
# Under the Whose Data choice, when there are accounts to choose from. { $reason } is one of the messages above.
core-profile-signed-in-unknown-choose = Kimin oturum açtığı belirlenemedi ({ $reason }). Kimin verilerinin ekleneceğini seçin.
