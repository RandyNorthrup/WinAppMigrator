# Words the core writes for people as it installs an app from a package: why an install will not start, the list of
# what it did (or for a preview, what it would do) that the installer window shows, its warnings, the record it keeps of
# each install so that it can be undone, and the core's errors. Message names start with core-.
#
# The list of what an install did is short and plain, one line for each thing, as the installer window lists them:
# "copied", "restored", "would copy". Words such as --force and --apply are command-line options, and LocalService,
# SYSTEM, ProgramData, and %AppData% are as Windows names them: keep them as they are.


## Errors: what went wrong, when the core cannot go on. Each is shown on its own, or after words that say what was being
## done.

core-error-list-changed = bu yüklemenin yapacakları onaylandıktan sonra değişti, bu nedenle hiçbir şey yüklenmedi
# { $path } is a file or folder, such as C:\Program Files\App\app.exe. { $error } is what Windows said went wrong.
core-error-io = { $path } için G/Ç işlemi başarısız oldu: { $error }
# { $detail } is what went wrong.
core-error-registry = kayıt defteri okunamadı: { $detail }
# { $path } is a program file. { $detail } is what was wrong with it.
core-error-pe = { $path } için PE ayrıştırması başarısız oldu: { $detail }
# { $detail } is what was wrong, already in words.
core-error-invalid-input = geçersiz giriş: { $detail }
# { $detail } is what went wrong.
core-error-serialization = rapor serileştirilemedi: { $detail }
# { $detail } is what went wrong.
core-error-archive = arşiv işlemi başarısız oldu: { $detail }
# { $detail } is what went wrong.
core-error-network = ağ işlemi başarısız oldu: { $detail }
# { $detail } is what went wrong.
core-error-time-format = zaman biçimlendirilemedi: { $detail }
core-error-cancelled = iptal edildi
# What is said when part of the program stopped unexpectedly and gave no reason.
core-error-panic-no-reason = bir neden belirtilmedi

## The record of each install, kept so that it can be undone.

# { $kind } is a number that says what type of registry value it is.
core-journal-unknown-value-type = bilinmeyen kayıt defteri değeri türü: { $kind }
core-journal-not-elevated = yükseltilmiş değil
core-journal-needs-admin = kaydının başkalarının değiştiremeyeceği bir yerde tutulması için yükleme, yönetici hakları gerektirir
# { $path } is a folder.
core-journal-not-an-install-folder = bu bir yükleme klasörü değil: { $path }
# { $path } is the file the record is kept in.
core-journal-not-written = bu yüklemenin kaydı yazılamadı: { $path }
# { $path } is a folder.
core-journal-path-escapes-root = yol, amaçlanan kökün dışına çıkıyor: { $path }
# { $path } is a folder.
core-journal-nothing-exists = bunların hiçbiri bu bilgisayarda yok: { $path }
core-journal-busy = başka bir yükleme veya kaldırma işlemi çalışıyor. Bitmesini bekleyin, ardından yeniden deneyin
# Said of a file that was to get back the permissions it had.
core-journal-no-folder = klasörü yok
# Said of the permissions a file had, which were kept to give back to it.
core-journal-permissions-unreadable = saklanan izinler okunamadı
# { $folder } is the folder of one install's record.
core-journal-unreadable-empty = { $folder } klasöründeki yükleme kaydı kullanılamaz: kayıt boş
# { $folder } is the folder of one install's record.
core-journal-unreadable-no-header = { $folder } klasöründeki yükleme kaydı kullanılamaz: neyle ilgili olduğunu belirten bilgiyle başlamıyor
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-version = { $folder } klasöründeki yükleme kaydı kullanılamaz: bu programın farklı bir sürümü tarafından yazılmış
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-install = { $folder } klasöründeki yükleme kaydı kullanılamaz: farklı bir yüklemeye ait olduğunu belirtiyor
# { $id } is an install's id, such as wam-1a2b3c.
core-journal-not-an-install-id = bu bir yükleme kimliği değil: { $id }

## Why an install will not start, or stopped before it changed anything.

core-restore-apply-and-dry-run = --apply ve --dry-run birlikte verilemez
# { $path } is the folder that was chosen to install in.
core-restore-location-not-full-path = yükleme konumu tam bir yol olmalıdır: “{ $path }”
# { $problem } says what is wrong with the folder, already in words. { $path } is the folder.
core-restore-location-unusable = bu yükleme konumu kullanılamaz: { $problem }: “{ $path }”
core-restore-needs-admin-plan = geri yükleme planı, yalnızca yöneticinin yapabileceği kayıt defteri, hizmet, zamanlanmış görev, kısayol veya korumalı hedef değişiklikleri içeriyor
# { $path } is the folder that was chosen to install in.
core-restore-needs-admin-target = hedef kök yükseltilmiş ayrıcalıklar gerektiriyor: { $path }
core-restore-needs-admin-user-data = kullanıcı verilerini ProgramData klasörüne veya diğer korumalı konumlara geri yüklemek yükseltilmiş ayrıcalıklar gerektirir
# { $error } is why the install stopped. { $failed } is what could not be put back, as sentences of their own.
core-restore-not-all-put-back = { $error }. Yapılanların tümü eski haline getirilemedi: { $failed }
# { $error } is what went wrong.
core-restore-nowhere-to-keep-record = bu yüklemenin geri alınmasını sağlayacak kaydı tutmak için güvenli bir yer yok, bu nedenle hiçbir şey yüklenmedi: { $error }
# { $path } is a file the app would have put there.
core-restore-already-exists = { $path } zaten var, bu nedenle hiçbir şey yüklenmedi. Üzerine yazmak için --force kullanın
# { $root } is a folder the app has files for. { $link } is that folder, or one it is in.
core-restore-root-under-link = bu uygulamanın { $root } konumuna geri yüklenecek öğeleri var ve { $link } başka bir yere giden, başkalarının değiştirebileceği bir bağlantı; bu nedenle hiçbir şey yüklenmedi. Oraya gidenler kullanıcı verileri veya lisans verileriyse uygulamayı kullanıcı verileri ve lisans verileri olmadan yükleyin. Başlangıç klasöründeki bir kısayolsa o klasörü eski yerine geri taşıyın
core-restore-hash-check-failed = paket karma denetiminden geçemedi ve hiçbir şey yüklenmedi
# { $detail } is the first thing found wrong, already in words.
core-restore-hash-check-failed-because = paket karma denetiminden geçemedi ve hiçbir şey yüklenmedi: { $detail }
# { $path } is a file or folder.
core-restore-refuse-records-folder = yükleme kayıtlarının tutulduğu yere yazılmıyor: { $path }
# { $path } is a file or folder.
core-restore-refuse-outside = yükleme konumunun ve bilinen veri klasörlerinin dışına yazılmıyor: { $path }
# { $path } is a file.
core-restore-no-parent-folder = hedefin üst klasörü yok: { $path }
# { $path } is where a file was to go.
core-restore-folder-in-the-way = bu dosyanın gitmesi gereken yerde bir klasör var: { $path }
# { $path } is a file in the package.
core-restore-file-changed = { $path } artık paketin içerdiği dosyayla aynı değil, bu nedenle yüklenmedi
# { $path } is a file in the package.
core-restore-source-is-link = { $path } bir bağlantı ve izlenmeyecek
# { $path } is a file in the package.
core-restore-source-became-link = { $path } açılırken bir bağlantıya dönüştü
# Why a service did not start, after "did not start:".
core-restore-service-stopped-by-itself = kendiliğinden yeniden durdu
# Why part of the app was left out, after "was left out:".
core-restore-com-class-someone-elses = bu COM sınıfı bu bilgisayarda zaten var ve başka bir şeye ait

## What an install did, one line for each thing, or for a preview, what it would do. { $source } is a file in the
## package, and { $destination } and { $path } are where it goes on this PC. { $key } is a registry key, such as
## HKLM\Software\App.

# { $package_valid } is true or false, and { $restore_valid } is true, false, or not_run: keep them as they are.
core-restore-verification-result = geri yükleme sonrası doğrulama package_valid={ $package_valid } restore_valid={ $restore_valid }
# { $name } is a service's name.
core-restore-started-service = hizmet başlatıldı: { $name }
core-restore-would-copy = kopyalanacak: { $source } -> { $destination }
core-restore-copied = kopyalandı: { $source } -> { $destination }
core-restore-would-create-empty-folder = boş klasör oluşturulacak: { $path }
core-restore-created-empty-folder = boş klasör oluşturuldu: { $path }
# { $value } is the name of a value in the key.
core-restore-would-restore-registry-value = kayıt defteri geri yüklenecek: { $key } { $value }
core-restore-would-restore-registry-key = kayıt defteri geri yüklenecek: { $key } (anahtar)
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-would-add-firewall-rule = güvenlik duvarı kuralı eklenecek: { $name } ({ $program } için)
# { $name } is a firewall rule's name.
core-restore-firewall-rule-already-there = güvenlik duvarı kuralı zaten var: { $name }
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-added-firewall-rule = güvenlik duvarı kuralı eklendi: { $name } ({ $program } için)
# { $name } is a service's name, { $command } what it runs, and { $account } the account it runs as, such as LocalService.
core-restore-would-restore-service = hizmet geri yüklenecek: { $name } -> { $command } (çalıştığı hesap: { $account })
# { $name } is a runtime, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-restore-prerequisite-present = önkoşul zaten var: { $name }
# { $name } is a runtime that Windows has as a Windows feature.
core-restore-would-install-prerequisite-feature = önkoşul yüklenecek: { $name } (Windows özelliği)
# { $name } is a runtime whose installer is in the package.
core-restore-would-install-prerequisite-bundled = önkoşul yüklenecek: { $name } (paketteki yükleyici)
# { $name } is a runtime that would come through winget, which is a program's name.
core-restore-would-install-prerequisite-winget = önkoşul yüklenecek: { $name } (winget)
# { $name } is a runtime.
core-restore-installed-prerequisite = önkoşul yüklendi: { $name }
# { $name } is a runtime.
core-restore-installed-prerequisite-reboot = önkoşul yüklendi: { $name } (yeniden başlatma gerekiyor)
core-restore-would-restore-license = lisans geri yüklenecek: { $source } -> { $destination }
core-restore-restored-license = lisans geri yüklendi: { $source } -> { $destination }
core-restore-would-restore-license-registry = lisans kayıt defteri geri yüklenecek: { $key }
# { $name } is a driver's name.
core-restore-driver-not-restored = sürücü otomatik olarak geri yüklenmedi: { $name }
# { $name } is a driver's name.
core-restore-would-install-driver = sürücü yüklenecek: { $name } -> { $destination }
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-would-restore-task = zamanlanmış görev geri yüklenecek: { $name }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-startup-shortcut = başlangıç kısayolu geri yüklenecek: { $shortcut } -> { $target }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-shortcut = kısayol geri yüklenecek: { $shortcut } -> { $target }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-would-restore-variable =
    { $scope ->
        [user] kullanıcı ortam değişkeni geri yüklenecek: { $name }
       *[machine] sistem ortam değişkeni geri yüklenecek: { $name }
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-left-variable =
    { $scope ->
        [user] { $name } kullanıcı ortam değişkeni olduğu gibi bırakıldı: hiçbir kısmı bu uygulamaya ait değil
       *[machine] { $name } sistem ortam değişkeni olduğu gibi bırakıldı: hiçbir kısmı bu uygulamaya ait değil
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-restored-variable =
    { $scope ->
        [user] kullanıcı ortam değişkeni geri yüklendi: { $name }
       *[machine] sistem ortam değişkeni geri yüklendi: { $name }
    }
core-restore-set-uninstaller = kaldırma programı ayarlandı: { $key }
core-restore-placed-driver-image = sürücü görüntüsü yerleştirildi: { $source } -> { $destination }
# { $name } is a driver's name.
core-restore-configured-driver = sürücü yapılandırıldı: { $name }
# { $name } is a driver's name.
core-restore-created-driver = sürücü oluşturuldu: { $name }
core-restore-ensured-registry-key = kayıt defteri anahtarının var olması sağlandı: { $key }
# { $value } is the name of a value in the key.
core-restore-restored-registry-value = kayıt defteri geri yüklendi: { $key } { $value }
# "(Default)" is how Windows names the value a key has with no name: write it as Registry Editor does.
core-restore-restored-registry-default = kayıt defteri geri yüklendi: { $key } (Varsayılan)
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-configured-service = hizmet yapılandırıldı: { $name } (çalıştığı hesap: { $account })
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-created-service = hizmet oluşturuldu: { $name } (çalıştığı hesap: { $account })
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-restored-task = zamanlanmış görev geri yüklendi: { $name }
core-restore-restored-shortcut = kısayol geri yüklendi: { $path }

## Warnings an install gives, and why it failed. They are shown after it, on the PC it ran on.

core-warning-restore-earlier-not-trusted = Hiç tamamlanmamış bir yüklemenin kaydına, başkaları değiştirebildiği için dokunulmadı. O yüklemenin değiştirdiği her şey hâlâ yerinde.
# { $app_name } is the app's name. { $count } is how many changes were put back.
core-warning-restore-earlier-undone =
    { $count ->
        [one] { $app_name } uygulamasının daha önceki bir yüklemesi hiç tamamlanmadığı için önce o geri alındı. { $count } değişiklik eski haline getirildi.
       *[other] { $app_name } uygulamasının daha önceki bir yüklemesi hiç tamamlanmadığı için önce o geri alındı. { $count } değişiklik eski haline getirildi.
    }
# { $app_name } is the app's name.
core-warning-restore-earlier-never-finished = { $app_name } uygulamasının daha önceki bir yüklemesi hiç tamamlanmadı. Yükleme önce onu geri alır.
core-warning-restore-user-data-real-locations = Kullanıcı verileri, hedef kök yerine bu makinenin gerçek %AppData%/%ProgramData% konumlarına geri yüklenir.
# { $error } is what went wrong.
core-warning-restore-commit-failed = Bu yüklemenin kaydı tamamlanamadı: { $error }
# { $name } is a service's name.
core-warning-restore-service-still-starting = { $name } hizmetinin başlatılması istendi, ancak hizmet henüz başlamayı bitirmedi. Biraz sonra başlayabilir. Bu bilgisayar bir sonraki açılışında da kendiliğinden başlar.
# { $name } is a service's name. { $reason } is why it did not start, as Windows said it.
core-warning-restore-service-not-started = { $name } hizmeti yüklendi, ancak başlamadı: { $reason }. Hizmet başlayana kadar uygulama çalışmayabilir. Windows, bu bilgisayarın bir sonraki açılışında yeniden dener.
# { $name } is a service's name. { $reason } is why it could not be stopped.
core-warning-restore-service-not-stopped = { $name } hizmeti önce durdurulamadı: { $reason }. Bu bilgisayarın bir sonraki açılışına kadar eski programı çalıştırmaya devam edebilir.
core-warning-restore-install-cancelled = Yükleme iptal edildi.
core-warning-restore-preview-cancelled = Önizleme iptal edildi.
# { $item } is what was left out, such as a file or a service's name. { $reason } is why, already in words.
core-warning-restore-left-out = { $item } dışarıda bırakıldı: { $reason }.
# { $error } is what went wrong.
core-warning-restore-verification-failed = Geri yükleme sonrası doğrulama çalıştırılamadı: { $error }
# { $runner } says what relies on the file: a service, a scheduled task, a firewall rule, a shortcut, or a registry value.
# { $path } is the file. { $name } is the name of what relies on it. { $refusal } is why the file was not replaced,
# already in words.
core-warning-restore-program-would-run =
    { $runner ->
        [service] { $path } bu paketin üzerine yazabileceği bir dosya değil ve { $name } hizmeti buna bağlı, bu nedenle hiçbir şey yüklenmedi: { $refusal }
        [task] { $path } bu paketin üzerine yazabileceği bir dosya değil ve { $name } zamanlanmış görevi buna bağlı, bu nedenle hiçbir şey yüklenmedi: { $refusal }
        [firewall] { $path } bu paketin üzerine yazabileceği bir dosya değil ve { $name } güvenlik duvarı kuralı buna bağlı, bu nedenle hiçbir şey yüklenmedi: { $refusal }
        [shortcut] { $path } bu paketin üzerine yazabileceği bir dosya değil ve { $name } kısayolu buna bağlı, bu nedenle hiçbir şey yüklenmedi: { $refusal }
       *[registry] { $path } bu paketin üzerine yazabileceği bir dosya değil ve { $name } kayıt defteri değeri buna bağlı, bu nedenle hiçbir şey yüklenmedi: { $refusal }
    }
core-warning-restore-destination-exists = Hedef zaten var; üzerine yazmak için --force kullanın.
# { $path } is a file. { $error } is what went wrong.
core-warning-restore-file-not-written = { $path } dosyası yazılamadı: { $error }
core-warning-restore-not-a-folder = Bu boş klasörün gideceği yerde klasör olmayan bir şey var.
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-not-looked-at = { $path } klasörüne bakılamadı: { $error }
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-folder-not-made = { $path } klasörü oluşturulamadı: { $error }
# Why part of the app was left out, after "was left out:".
core-warning-restore-registration-exists = bu bilgisayarda bu kayıt zaten var ve başka bir şeye ayarlı; bir yükleme bunu devralmaz
core-warning-restore-registry-review = Kayıt defteri değerleri arşivlenmiş yapılandırılmış verilerden geri yüklendi; bilinen uygulama yolları hedef köke göre yeniden yazıldı.
# { $error } is what went wrong.
core-warning-restore-firewall-unreadable = Windows Güvenlik Duvarı kuralları okunamadı: { $error }
# Why a firewall rule was left out, after "was left out:".
core-warning-restore-firewall-rule-exists = bu program için bu adda bir kural zaten var ve başka bir şeye ayarlı; bir yükleme bunu değiştirmez
# { $name } is a runtime.
core-warning-restore-prerequisite-unknown = { $name }: bu çalışma zamanı için bilinen bir yükleyici yok.
# { $name } is a runtime.
core-warning-restore-prerequisite-needs-admin = { $name } çalışma zamanının yüklenmesi yükseltme gerektiriyor; yükleyiciyi yönetici olarak yeniden çalıştırın.
# { $name } is a runtime.
core-warning-restore-prerequisite-reboot = { $name } yüklendi, ancak tamamlanması için yeniden başlatma gerekiyor.
# { $error } is what went wrong.
core-warning-restore-skipped-not-recorded = Atlanan bir çalışma zamanı kaydedilemedi: { $error }
# { $name } is a runtime. { $reason } is why it was skipped, already in words.
core-warning-restore-prerequisite-skipped = { $name }: { $reason }
# { $name } is a runtime. { $reason } is why it could not be installed. { $url } is a web address to get it from.
core-warning-restore-prerequisite-failed = { $name } yüklenemedi: { $reason }. Şu adresten edinin: { $url }.
# { $error } is what went wrong.
core-warning-restore-runtime-not-recorded = Yüklenen bir çalışma zamanı kaydedilemedi: { $error }
# { $error } is what went wrong.
core-warning-restore-license-source-invalid = Paketteki lisans yolu geçersiz: { $error }
core-warning-restore-license-needs-admin = Lisans verilerini korumalı bir konuma geri yüklemek yükseltme gerektirir.
core-warning-restore-license-exists = Lisans hedefi zaten var; üzerine yazmak için --force kullanın.
# { $error } is what went wrong.
core-warning-restore-license-failed = Lisans verileri geri yüklenemedi: { $error }
core-warning-restore-driver-needs-admin = Sürücü kaydı yükseltme gerektirir; yükleyiciyi yönetici olarak yeniden çalıştırın.
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-warning-restore-task-runs-as-system = { $name } zamanlanmış görevi SYSTEM olarak, bu bilgisayar üzerinde tam haklarla çalışır.
core-warning-restore-shortcut-exists = Kısayol zaten var; üzerine yazmak için --force kullanın.
core-warning-restore-shortcut-all-users = Geri yüklenen kısayollardan en az biri ortak masaüstünü veya ortak Başlat menüsü konumunu hedefliyor.
core-warning-restore-environment-refresh = Ortam değişkeni değişikliklerinin görülmesi için yeni bir işlem, kabuk veya oturum gerekebilir.
# { $name } is a service's name. { $account } is the account it ran as on the PC it came from.
core-warning-restore-service-account = { $name } hizmeti kaynak bilgisayarda { $account } olarak çalışıyordu. Parolası yükleyicide olmadığından hizmet LocalService olarak ayarlandı. Çalışmazsa hesabını ve parolasını Hizmetler'de ayarlayın.
# { $name } is a runtime.
core-warning-restore-prerequisite-pending-reboot = { $name } henüz kaydedilmedi; yeniden başlatmadan sonra etkin olması beklenir.
# { $name } is a runtime.
core-warning-restore-prerequisite-not-verified = { $name } başarılı olduğunu bildirdi ancak kaydolmadı; yükleme başarısız olmuş olabilir.
# { $error } is what went wrong.
core-warning-restore-driver-source-invalid = Paketteki sürücü yolu geçersiz: { $error }
# { $error } is what went wrong.
core-warning-restore-driver-not-read = Sürücünün geçerli ayarları okunamadı: { $error }
core-warning-restore-driver-exists = Sürücü hizmeti zaten var; yeniden yapılandırmak için --force kullanın.
core-warning-restore-driver-image-exists = Sürücü görüntüsü zaten var; üzerine yazmak için --force kullanın.
core-warning-restore-driver-hash-bundled = Paketteki sürücü görüntüsü kayıtlı karmasıyla eşleşmiyor, bu nedenle yüklenmedi.
# { $error } is what went wrong.
core-warning-restore-driver-not-placed = Sürücü görüntüsü yerleştirilemedi: { $error }
core-warning-restore-driver-hash-placed = Sürücü görüntüsü yerleştirildikten sonra değişti, bu nedenle kaydedilmedi.
# { $error } is what went wrong.
core-warning-restore-key-not-created = Kayıt defteri anahtarı oluşturulamadı: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-read = Bunun üzerine yazacağı değer okunamadı: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-written = Kayıt defteri değeri yazılamadı: { $error }
core-warning-restore-value-exists = Kayıt defteri değeri zaten var; üzerine yazmak için --force kullanın.
# { $error } is what went wrong.
core-warning-restore-environment-key-not-created = Ortam değişkenleri için kayıt defteri anahtarı oluşturulamadı: { $error }
core-warning-restore-variable-exists = Ortam değişkeni zaten var; üzerine yazmak için --force kullanın.
# { $error } is what went wrong.
core-warning-restore-variable-not-written = Ortam değişkeni yazılamadı: { $error }
core-warning-restore-value-kind-missing = Kayıt defteri değerinin türü eksik.
# { $error } is what went wrong.
core-warning-restore-service-not-read = Hizmetin geçerli ayarları okunamadı: { $error }
core-warning-restore-service-exists = Hizmet zaten var; yeniden yapılandırmak için --force kullanın.
# { $error } is what went wrong.
core-warning-restore-task-not-queried = Zamanlanmış görev sorgulanamadı: { $error }
core-warning-restore-task-exists = Zamanlanmış görev zaten var; üzerine yazmak için --force kullanın.
# { $error } is what went wrong.
core-warning-restore-task-folder-not-looked-at = Görevin klasörüne bakılamadı: { $error }
# { $error } is what went wrong.
core-warning-restore-shortcut-not-put-back = Kısayol oluşturulamadı ve üzerine yazdığı kısayol eski haline getirilemedi: { $error }
# { $error } is what went wrong.
core-warning-restore-command-not-started = Komut başlatılamadı: { $error }
# { $output } is what the command wrote, as it wrote it.
core-warning-restore-command-failed = Komut başarısız oldu: { $output }
# { $error } is what went wrong.
core-warning-restore-powershell-not-started = PowerShell başlatılamadı: { $error }
