# Words the core says about what an install may do: why an install location or something in a package is refused,
# the installer's list of what an app sets up beyond its files, the checks of a package and of an install, and whether
# there is room for either. Message names start with core-.
#
# Many messages here are a reason, the second half of a sentence that starts "... was left out:" or "that install
# location cannot be used:", so they start with a small letter and have no full stop.


## Why an install location cannot be used. Shown in red under the Install Location field.

core-review-location-state-root = yüklemelerin kayıtlarını tuttuğu yer burası
core-review-location-has-files = burada zaten dosyalar var ve kayıtlı bir yükleme yok; bu nedenle boş bir klasör kullanın
core-policy-location-not-full-path = tam bir yol olmalıdır
core-policy-location-parent-step = “..” içermemelidir
core-policy-location-not-plain = bu bilgisayarın bir sürücüsündeki düz bir yol olmalıdır
core-policy-location-whole-drive = sürücünün tamamı olamaz
core-policy-location-windows = Windows klasörü olamaz
core-policy-location-runs-by-itself = Windows'a ait bir klasör veya içeriği kendiliğinden çalışan ya da yüklenen bir klasör
core-policy-location-shared = başka şeyleri de yan yana barındırıyor, bu nedenle uygulamanın bunun içinde bir klasöre ihtiyacı var

## Whether an app can run on this PC at all, said before anything is installed.

# { $reason } is why, as the second half of a sentence, such as "App.exe is built for Arm64, and this PC is x64, so
# Windows would not start it".
core-review-cannot-run-here = bu uygulama bu bilgisayarda çalışamaz, bu nedenle hiçbir şey değiştirilmedi: { $reason }
core-policy-architecture-x86 = 32 bit x86
core-policy-architecture-x64 = x64
core-policy-architecture-arm64 = Arm64
core-policy-architecture-arm32 = 32 bit Arm
core-policy-architecture-unknown = bilinmeyen bir mimari
# { $code } is the number Windows gives a kind of processor, such as 0x01c4.
core-policy-machine-type = makine türü { $code }
# { $names } is a list of program names, such as "a.exe, b.exe, c.exe", and { $count } how many more there are.
core-policy-names-and-more = { $names } ve { $count } tane daha
core-policy-fit-programs-not-said = Bu yükleyici, programlarının neye ihtiyaç duyduğunu belirtmiyor; bu nedenle bu bilgisayarda çalışıp çalışamayacakları denetlenmedi.
core-policy-fit-architecture-not-told = Bu bilgisayarın mimarisi belirlenemedi; bu nedenle bu uygulamanın programlarının burada çalışıp çalışamayacağı denetlenmedi.
core-policy-fit-main-not-said = Bu yükleyici, ana programının neye ihtiyaç duyduğunu belirtmiyor; bu nedenle burada söylenenler, yükleyicinin içerdiği diğer bilgilere dayanıyor.
# { $driver } is a driver's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-driver = { $driver } sürücüsü { $built_for } için derlenmiş; bu bilgisayarın mimarisi ise { $here }. Sürücünün Windows ile tam olarak eşleşmesi gerektiğinden bu uygulama burada çalışamaz
# { $program } is a program's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-program = { $program }, { $built_for } için derlenmiş; bu bilgisayarın mimarisi ise { $here }. Bu nedenle Windows onu başlatmaz
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-none-start = programlarının hiçbiri bu bilgisayarda başlayamaz (bu bilgisayarın mimarisi: { $here }): { $programs }
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-some-other-architecture = Bu uygulamanın bazı programları başka bir mimari için derlenmiş ve bu bilgisayarda başlamaz (bu bilgisayarın mimarisi: { $here }): { $programs }. Uygulamanın geri kalanı başlayabildiğinden bu önemli olmayabilir.
# { $here } is a kind of processor, such as "Arm64". { $programs } is a list of program names.
core-policy-fit-emulated = Bu bilgisayarın mimarisi { $here }, bu nedenle Windows şunları bir öykünücü üzerinden çalıştırır: { $programs }. Çoğu program bu şekilde sorunsuz çalışır, bazıları çalışmaz.
# { $program } is a program's file name. { $needs } and { $runs } are versions of Windows, such as 10.0 and 6.1.
core-policy-fit-needs-newer = { $program }, Windows { $needs } veya daha yeni bir sürüm gerektirdiğini belirtiyor ve bu bilgisayarda Windows { $runs } çalışıyor; bu nedenle Windows onu başlatmaz
# { $programs } is a list of program names.
core-policy-fit-some-need-newer = Bu uygulamanın bazı programları, buradakinden daha yeni bir Windows gerektirdiğini belirtiyor ve burada başlamaz: { $programs }.
# { $from_build } and { $this_build } are Windows build numbers, such as 22631, written as Windows writes them.
core-policy-fit-older-build = Bu uygulama Windows'un { $from_build } derlemesindeydi ve bu bilgisayarda { $this_build } derlemesi çalışıyor. Programları bu Windows'ta başlayabileceklerini belirtiyor, bu nedenle uygulama çalışabilir; ancak orada dayandığı şeyler burada olmayabilir.
core-policy-fit-build-not-told = Bunun hangi Windows derlemesi olduğu belirlenemedi; bu nedenle bu uygulamanın bulunduğu derlemeyle karşılaştırılmadı.

## Room on a drive for a build or an install.

# Each of these ends the sentence core-space-shortfall, after "free for".
core-space-for-unpacking = paketin açılması
core-space-for-install-record = yükleme kaydı
core-space-for-app-files = uygulamanın dosyaları
core-space-for-what-is-replaced = üzerine yazılanların saklanması
core-space-for-build-work = oluşturma işleminin kullandığı dosyalar
core-space-for-installer = yükleyici
core-space-for-package = paket
core-space-for-unpacking-and-installing = paketin açılması ve yüklenmesi
# What goes between two of those, as in "unpacking the package and the app's files". Keep the quotes: they keep the
# spaces.
core-space-purposes-joiner = { " ve " }
# { $drive } is a drive, such as C:, or a folder. { $needed } and { $free } are sizes, such as 1.2 GB. { $purposes } is
# what the room is for, such as "unpacking the package and the app's files".
core-space-shortfall = { $drive } üzerinde, { $purposes } için en fazla { $needed } boş alan gerekiyor; şu anda { $free } boş alan var
# { $drive } is a drive, such as C:, or a folder. { $needed } is a size, such as 1.2 GB.
core-space-not-told = { $drive } ne kadar boş alanı olduğunu bildirmedi; bu nedenle gereken { $needed } alanın sığıp sığmadığı denetlenmedi
# One drive in a list of where room is needed. { $needed } and { $free } are sizes, and { $drive } a drive, such as C:.
core-space-summary = { $drive } üzerinde en fazla { $needed } ({ $free } boş)
# { $needed } is a size, and { $drive } a drive, such as C:.
core-space-summary-not-told = { $drive } üzerinde en fazla { $needed } (ne kadar boş alan olduğu belirlenemedi)
# What goes between two sentences of core-space-shortfall inside core-review-no-room. Keep the quotes: they keep the
# space.
core-review-sentence-joiner = { ". " }
# { $shortfalls } is one or more sentences of core-space-shortfall.
core-review-no-room = bunun için yeterli alan yok, bu nedenle hiçbir şey değiştirilmedi. { $shortfalls }. Orada biraz yer açın veya başka bir sürücüde bir yükleme konumu seçin.

## The installer's list of what an app sets up beyond its files: what each kind is called.

core-review-kind-service = Hizmet
core-review-kind-driver = Sürücü
core-review-kind-scheduled-task = Zamanlanmış görev
core-review-kind-autostart = Otomatik başlayan
core-review-kind-shortcut = Kısayol
core-review-kind-path = PATH
core-review-kind-environment-variable = Ortam değişkeni
core-review-kind-shell-extension = Kabuk uzantısı
core-review-kind-menu-command = Menü komutu
core-review-kind-link-type = Bağlantı türü
core-review-kind-file-type = Dosya türü
core-review-kind-com = COM
core-review-kind-firewall-rule = Güvenlik duvarı kuralı
core-review-kind-event-log-source = Olay günlüğü kaynağı
core-review-kind-font = Yazı tipi
core-review-kind-odbc-driver = ODBC sürücüsü
core-review-kind-odbc-source = ODBC veri kaynağı
core-review-kind-compatibility-setting = Uyumluluk ayarı
core-review-kind-default-apps = Varsayılan uygulamalar
core-review-kind-program-name = Program adı
core-review-kind-other-settings = Diğer ayarlar

## The installer's list of what an app sets up: what each line says of it.

# { $file } is a program's file name. { $account } is the Windows account it runs as, such as LocalSystem. { $starts }
# says when it starts, and is not shown itself; for "other", { $start } is one of the core-review-start- words below.
core-review-service-detail =
    { $starts ->
        [automatic] { $file } dosyasını { $account } olarak çalıştırır; şimdi ve bu bilgisayar her başladığında başlar
        [delayed] { $file } dosyasını { $account } olarak çalıştırır; şimdi ve bu bilgisayar her başladıktan kısa süre sonra başlar
        [manual] { $file } dosyasını { $account } olarak çalıştırır; yalnızca bir şey istediğinde başlar
        [disabled] { $file } dosyasını { $account } olarak çalıştırır; devre dışı
        [other] { $file } dosyasını { $account } olarak çalıştırır; { $start }
       *[unknown] { $file } dosyasını { $account } olarak çalıştırır
    }
# { $account } is a Windows account, such as LocalService.
core-policy-account-custom = { $account } (özel hesap geri yüklenmedi)
# { $file } is a driver's file name.
core-review-driver-detail = { $file } dosyasını doğrudan Windows'a yükler
# { $file } is a driver's file name. { $start } is when Windows starts it: one of the core-review-start- words below.
core-review-driver-detail-starts = { $file } dosyasını doğrudan Windows'a yükler; { $start }
# When a service or driver starts, ending a sentence such as "Loads disk.sys into Windows itself, starts …".
core-review-start-boot = Windows ile birlikte, her şeyden önce başlar
core-review-start-system = Windows açılırken onunla birlikte başlar
core-review-start-automatic = bu bilgisayar her başladığında başlar
core-review-start-delayed = bu bilgisayar her başladıktan kısa süre sonra başlar
core-review-start-manual = yalnızca bir şey istediğinde başlar
core-review-start-disabled = devre dışı olduğu için hiçbir zaman başlamaz
core-review-start-unknown = Windows'un belirtmediği bir şekilde başlar
# { $account } is the account a task runs as: an account's name, or core-review-task-runs-as-you, or
# core-review-task-runs-as-creator.
core-review-task-detail = Çalıştığı hesap: { $account }
# { $account } is the account a task runs as, as for core-review-task-detail.
core-review-task-detail-highest = Çalıştığı hesap: { $account }, alabileceği en yüksek haklarla
core-review-task-runs-as-you = siz (oturum açmış kişi)
core-review-task-runs-as-creator = onu oluşturan kullanıcı
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-autostart-detail =
    { $scope ->
        [everyone] Oturum açan herkes için çalışır
       *[you] Oturum açtığınızda çalışır
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shortcut-startup-detail =
    Başlangıç klasöründe bir kısayol, { $scope ->
        [everyone] bu bilgisayardaki herkes için
       *[you] sizin için
    }
# { $file } is the file name of what the shortcut opens. { $scope } is whom it is set up for: everyone on this PC,
# or you. It is not shown itself.
core-review-shortcut-detail =
    { $file } dosyasını açar, { $scope ->
        [everyone] bu bilgisayardaki herkes için
       *[you] sizin için
    }
# { $name } is an environment variable's name, such as PATH.
core-review-path-title = { $name } değişkenine ekler
core-review-path-install-folder = Yükleme klasörü
# { $path } is a folder inside the install folder, such as bin\tools.
core-review-path-below-install-folder = Yükleme klasörü\{ $path }
# { $value } is the variable's value, as it will be set.
core-review-variable-detail = Şu değere ayarlanır: { $value }
# { $count } is how many registry values.
core-review-other-settings-title =
    { $count ->
        [one] Uygulamanın kendi anahtarları dışında { $count } ayar
       *[other] Uygulamanın kendi anahtarları dışında { $count } ayar
    }
# { $keys } is a list of registry keys, such as HKCU\Software\Vendor.
core-review-other-settings-detail = Konum: { $keys }
# { $keys } is a list of registry keys, and { $count } how many more there are.
core-review-other-settings-detail-more = Konum: { $keys } ve { $count } tane daha
# { $way } says whether the rule allows or blocks, and incoming or outgoing connections; it is not shown itself.
# { $ports } is a list of port numbers, and { $port_count } how many. { $program } is the program's file name.
# { $networks } is a list of the kinds of network Windows names in English (domain, private, public), and
# { $network_count } how many.
# The kinds of network a firewall rule is for, as Windows Defender Firewall names its profiles in your language. They
# fill { $networks } in core-review-firewall-detail, joined into a list.
core-review-network-domain = Etki alanı
core-review-network-private = Özel
core-review-network-public = Ortak
core-review-firewall-detail =
    { $program } için { $network_count ->
        [0] tüm ağlarda
       *[other] { $networks } türündeki ağlarda
    }{ $port_count ->
        [0] { "" }
        [one] { " " }{ $ports } bağlantı noktasında
       *[other] { " " }{ $ports } bağlantı noktalarında
    } { $way ->
        [allow-in] gelen bağlantılara izin verir
        [allow-out] giden bağlantılara izin verir
        [block-in] gelen bağlantıları engeller
       *[block-out] giden bağlantıları engeller
    }
# { $server } is the file name of what Explorer loads. { $scope } is whom it is set up for: everyone on this PC, or
# you. It is not shown itself.
core-review-shell-extension-detail =
    { $server } dosyasını Dosya Gezgini'ne yükler, { $scope ->
        [everyone] bu bilgisayardaki herkes için
       *[you] sizin için
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shell-extension-detail-own-file =
    Kendi yüklediği bir dosyayı Dosya Gezgini'ne yükler, { $scope ->
        [everyone] bu bilgisayardaki herkes için
       *[you] sizin için
    }
# The name of a shell extension: { $handler } is its name in the registry, and { $class } the kind of file or folder it
# is for, such as * or Directory.
core-policy-shell-extension-name = { $handler } ({ $class } için)
# { $on } is the kind of file or folder whose menu it is, as the registry names it, such as * or
# Directory. { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-menu-command-detail =
    { $on } menüsüne bir komut ekler, { $scope ->
        [everyone] bu bilgisayardaki herkes için
       *[you] sizin için
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-default-verb-detail =
    Bunlardan birini açmanın ne yapacağını değiştirir, { $scope ->
        [everyone] bu bilgisayardaki herkes için
       *[you] sizin için
    }
core-review-client-entry-detail = Bunun için program olarak seçilebilir. Mevcut ayarda hiçbir şey değiştirilmez
core-review-com-permissions-title = COM izinleri
core-review-com-permissions-detail = COM sınıflarını kimlerin başlatabileceğini veya bunların hangi hesapla çalışacağını belirtir
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-link-type-detail =
    Bu türdeki bağlantılar uygulamayla açılır, { $scope ->
        [everyone] bu bilgisayardaki herkes için
       *[you] sizin için
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-file-type-detail =
    Bu türdeki dosyalar uygulamayla açılır, { $scope ->
        [everyone] bu bilgisayardaki herkes için
       *[you] sizin için
    }
# { $log } is the name of a Windows event log, such as Application.
core-review-event-log-source-detail = { $log } günlüğüne, kendi dosyalarındaki iletilerle yazabilir
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-font-detail =
    Oturum açıldığında yükleme klasöründen yüklenir, { $scope ->
        [everyone] bu bilgisayardaki herkes için
       *[you] sizin için
    }
core-review-odbc-driver-detail = Bir veritabanına erişmek için ODBC kullanan programlara yüklenir
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-odbc-source-detail =
    Bir veritabanı bağlantısı, { $scope ->
        [everyone] bu bilgisayardaki herkes için
       *[you] sizin için
    }
core-review-default-apps-detail = Varsayılan uygulamalar bölümünde seçilebilir. Mevcut ayarda hiçbir şey değiştirilmez
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-program-name-detail =
    Çalıştır ve diğer programlar onu bu adla bulabilir, { $scope ->
        [everyone] bu bilgisayardaki herkes için
       *[you] sizin için
    }
# { $server } is the file name of what serves the classes.
core-review-com-title = { $server } içinde COM
core-review-com-title-own-file = Yüklediği bir dosyada COM
# { $count } is how many registry values.
core-review-com-detail =
    { $count ->
        [one] Diğer programların başlatabileceği sınıflara ait { $count } ayar
       *[other] Diğer programların başlatabileceği sınıflara ait { $count } ayar
    }
core-review-com-support-title = COM tür kitaplıkları, arabirimler ve AppID'ler
# { $libraries }, { $interfaces }, and { $app_ids } are how many registry values there are for its type libraries, its
# interfaces, and its AppIDs.
core-review-com-support-detail = Sınıflarıyla birlikte gelen tür kitaplığı ({ $libraries }), arabirim ({ $interfaces }) ve AppID ({ $app_ids }) ayarları
core-review-compatibility-title = Programlarının nasıl başladığı
# { $count } is how many registry values.
core-review-compatibility-detail =
    { $count ->
        [one] Yüklediği programlar için { $count } uyumluluk ayarı
       *[other] Yüklediği programlar için { $count } uyumluluk ayarı
    }

## The installer's notes on whose folders and settings these are, and what is not coming.

# { $account } is the name of a Windows account, such as alice.
core-review-account-signed-in = Kullanıcı verileri ve kullanıcı başına ayarlar, bunun çalıştığı yönetici hesabına değil, burada oturum açmış olan { $account } hesabına gider.
# { $account } is the name of a Windows account, such as alice.
core-review-account-no-desktop = Bu oturumda kimse oturum açmamış, bu nedenle kullanıcı verileri ve kullanıcı başına ayarlar, bunun çalıştığı { $account } hesabına gider.
# { $reason } is what Windows said, in its own words.
core-review-account-not-told = Kimin oturum açtığı belirlenemedi ({ $reason }), bu nedenle kullanıcı başına ayarlar dışarıda bırakılır ve kullanıcı verileri yükleme konumunun altına gider.
# { $reason } is what Windows said, in its own words.
core-review-signed-in-unknown = kimin oturum açtığı belirlenemedi: { $reason }
# Headings of what is left out, shared by the app's list and the installer's notes.
core-review-left-out-credentials = Kimlik bilgisine benzedikleri için bu yükleyicide yok
core-review-left-out-settings = Bir hesabın uygulama için tuttuğu, yalnızca Kullanıcı verileri seçeneğiyle eklenen ayarlar ve geçmiş
core-review-left-out-certificates = Uygulama, kendi bilgisayarında güvenilen sertifikalara dayanıyor gibi görünüyor ve bunlar yüklenmez
core-review-left-out-fonts = Bu yazı tipleri uygulamanın adını taşıyor, ancak uygulamaya ait olduklarını gösteren bir şey yok; bu nedenle yüklenmez
# { $names } is a list of names, such as the names of saved passwords.
core-review-note-credentials = Kimlik bilgisine benzedikleri için bu yükleyicide yok: { $names }. Bu bilgisayardaki karşılıkları olduğu gibi kalır ve bunları yeniden girmeniz gerekebilir.
# { $names } is a list of names of license data.
core-review-note-licenses = Yükleyici oluşturulurken Lisans ve etkinleştirme verileri seçeneği kapalı olduğu için bu yükleyicide yok: { $names }.
# { $names } is a list of places an account keeps settings, such as folders.
core-review-note-settings = Yükleyici oluşturulurken Kullanıcı verileri seçeneği kapalı olduğu için bu yükleyicide yok: Bir hesabın uygulama için tuttuğu ayarlar ve geçmiş ({ $names } içinde). Uygulama burada, onu ilk kez kullanan biri için olduğu gibi başlar.
# { $names } is a list of certificates, each as core-review-certificate.
core-review-note-certificates = Uygulama, kendi bilgisayarında güvenilen sertifikalara dayanıyor gibi görünüyor ve bunlar yüklenmez: { $names }. Bunlara burada güvenip güvenmemek size kalmış.
# { $names } is a list of font names.
core-review-note-fonts = Bu yazı tipleri uygulamanın adını taşıyor, ancak uygulamaya ait olduklarını gösteren bir şey yok; bu nedenle yüklenmez: { $names }.
# { $names } is a list of names.
core-review-note-left-out-by-choice = Bu yükleyiciyi oluşturan kişi tarafından dışarıda bırakıldı: { $names }.
# { $names } is a list of names.
core-review-note-added-by-choice = Bu yükleyiciyi oluşturan kişi tarafından, uygulamanın parçası olduğu düşünülerek eklendi: { $names }.
# { $name } is a certificate's subject, such as CN=Contoso, and { $store } the certificate store it was in, such as
# Root.
core-review-certificate = { $name } ({ $store } deposunda)
# { $names } is a list of names, and { $count } how many more there are.
core-review-names-and-more = { $names } ve { $count } tane daha

## Why something in a package is left out. Each is the second half of a sentence of core-warning-review-left-out.

core-review-no-account = tek bir hesabın ayarlarına ait ve kimin oturum açtığı belirlenemedi
core-review-name-not-accepted = Windows adını kabul etmedi
# { $error } is what Windows said, in its own words.
core-review-service-unreadable = geçerli ayarları okunamadı: { $error }
core-review-service-dependency = geçersiz adlı bir hizmete bağlı
# { $path } is the program the service runs.
core-review-service-runs-other = bu paketin yüklemediği { $path } dosyasını çalıştırıyor
core-review-service-settings = ayarları, bu paketin yüklemediği bir şeyi veya kendisine ait olmayan bir anahtarı adlandırıyor
core-review-driver-no-hash = denetlenebileceği bir karması yok
core-review-license-for-the-machine = tüm makine için depolanan bir lisans yalnızca bildirilir
core-review-license-starts-something = bir lisansın herhangi bir şeyi başlatması için bir neden yok
core-review-task-not-captured = tanımı alınmadı
# { $error } is what was wrong with the task's XML, in English.
core-review-task-unreadable = tanımı okunamadı: { $error }
core-review-task-no-account = onu yükleyen kişi olarak çalışıyor ve burada kimin oturum açtığı belirlenemedi
# { $path } is what the shortcut opens.
core-review-shortcut-opens-other = bu paketin yüklemediği { $path } öğesini açıyor
# { $service } is the service's name, and { $path } the program it runs now.
core-policy-service-exists = { $service } bu bilgisayarda zaten var ve buradaki önceki bir yüklemeden gelmeyen { $path } dosyasını çalıştırıyor. Bu uygulamanın eski bir kopyasıysa önce onu kaldırın
core-policy-key-control-characters = anahtar adı denetim karakterleri içeriyor
core-policy-value-control-characters = değerinin adı denetim karakterleri içeriyor
core-policy-registry-link = bir kayıt defteri bağlantısı
# { $reason } is why the key it stands for is left out, such as "no app install should change this key".
core-policy-virtual-store = bilgisayara ait bir anahtarın yerini tutuyor ve { $reason }
core-policy-registry-part = hiçbir uygulama yüklemesi kayıt defterinin bu bölümüne yazmamalı
core-policy-key-refused = hiçbir uygulama yüklemesi bu anahtarı değiştirmemeli
core-policy-net-debugger = her .NET programı için bir hata ayıklayıcı belirleyecekti
core-policy-net-install-root = her .NET programının .NET'i nereden yüklediğini değiştirecekti
core-policy-class-redirects = bir tür programın veya klasörün nasıl açıldığını değiştirecekti
core-policy-app-path-name = bir programı dosya adından başka bir şeyle adlandırıyor
core-policy-app-path-not-installed = bu paketin yüklemediği bir dosya için program adı kaydedecekti
core-policy-shared-default-verb = belirli bir türdeki her dosyayı veya klasörü açmanın ne yapacağını değiştirecekti
core-policy-command-runs-other = bu paketin yüklemediği bir şeyi çalıştıran bir komut ekliyor
core-policy-autostart-not-command = otomatik olarak başlıyor ve değeri bir komut değil
core-policy-autostart-runs-other = otomatik olarak başlıyor ve bu paketin yüklemediği bir şeyi çalıştırıyor
core-policy-environment-own-rules = ortam değişkenleri kendi kurallarına göre geri yüklenir
core-policy-service-not-restored = bu paketin geri yüklemediği bir hizmete ait
core-policy-service-own-rules = bir hizmetin kendi ayarları yalnızca hizmet kurallarına göre geri yüklenir
core-policy-service-dll = ServiceDll değeri bu paketin yüklediği bir dosya değil
core-policy-event-log-source-only = yalnızca bir olay günlüğü kaynağının kendi değerleri geri yüklenebilir
core-policy-security-log = hiçbir uygulama Güvenlik günlüğüne yazmaz
core-policy-event-log-messages = iletileri bu paketin yüklemediği bir dosyadan gelecekti
core-policy-type-library = tür kitaplığı bu paketin yüklediği bir dosya değil
# { $name } is a program's file name, such as app.exe.
core-policy-application-opened = bu paketin yüklemediği { $name } dosyasının nasıl açıldığını değiştirecekti
core-policy-event-log-no-file = bir olay günlüğü kaynağı iletilerini bir dosyadan almalıdır
core-policy-com-runs-other = bu paketin yüklediği bir dosya olmayan bir şeyi çalıştırıyor
core-policy-com-redirect = bu sınıfı, bu paketin yüklemediği bir şeye dönüştürecekti
core-policy-com-server = ne bu paketin yüklediği bir dosya ne de Windows'un bir parçası olan bir şeyi çalıştırıyor
core-policy-interface = bir arabirime yapılan çağrıların nasıl yapılacağını, bu paketin yüklemediği bir şeyle belirleyecekti
# { $name } is a program's file name, such as app.exe.
core-policy-app-id-started = bu paketin yüklemediği { $name } dosyasının nasıl başlatıldığını değiştirecekti
core-policy-app-id-surrogate = bu paketin yüklemediği bir program tarafından barındırılacaktı
core-policy-app-id-service = bu paketin geri yüklemediği bir hizmette çalışacaktı
core-policy-app-id-run-as = kendine ait bir hesapla çalışacaktı
core-policy-app-id-link = bir AppID belirtmiyor
core-policy-app-id-remote = bu bilgisayardan başka bir yerde çalışacaktı
core-policy-odbc-own-driver = bu, ODBC'nin kendi ayarı; bir sürücü değil
core-policy-odbc-driver-not-installed = sürücüsü bu paketin yüklediği bir dosya değil
core-policy-loads-other-file = bu paketin yüklemediği bir dosyayı yükleyecekti
core-policy-odbc-own-source = bu, ODBC'nin kendi ayarı; bir veri kaynağı değil
core-policy-odbc-source-missing = bu pakette olmayan bir veri kaynağını listeliyor
core-policy-odbc-source-driver = sürücüsü ne bu paketin yüklediği bir dosya ne de Windows'un bir parçası
core-policy-shell-extension = bu paketin yüklediği bir dosya olmayan bir kabuk uzantısını Dosya Gezgini'ne yükleyecekti
core-policy-not-text = içerdiği şey, okunacağı türden bir metin değil
core-policy-network-place = ağdaki bir yeri adlandırıyor
core-policy-font = bu paketin yüklemediği veya Windows klasöründe bulunan bir yazı tipini kaydedecekti
core-policy-compatibility = bu paketin yüklemediği bir programın nasıl çalıştığını değiştirecekti
core-policy-capabilities = ayrıntıları bu pakette olmayan bir uygulamayı listeliyor
core-policy-default-program = öğeleri varsayılan olarak hangi programın açtığını değiştirecekti
core-policy-url-protocol = bir bağlantı türü olduğunu belirtiyor ve bu paketin yüklediği hiçbir şeyle açılmıyor
core-policy-command-handler = bu paketin yüklemediği bir şeyin işlediği bir komut ekliyor
core-policy-installer-command = yalnızca Windows Installer'ın okuyabildiği bir şeyi çalıştıracaktı
core-policy-creates-class = bu paketin yüklemediği bir sınıf oluşturacaktı
core-policy-opens-links = bağlantıları bu paketin yüklemediği bir şeyle açıyor
core-policy-opens-files = dosyaları bu paketin yüklemediği bir şeyle açıyor
core-policy-firewall-name = adı, buradan aktarılabilecek bir ad değil
core-policy-firewall-description = açıklaması, buradan aktarılabilecek bir açıklama değil
core-policy-firewall-every-program = her program için geçerli, oysa bir kural yalnızca bir program için olabilir
# { $path } is the program the rule is for.
core-policy-firewall-program = bu paketin yüklemediği { $path } için
core-policy-firewall-direction = burada ayarlanabilecek bir yön belirtmiyor
core-policy-firewall-action = diğer kuralları geçersiz kılacaktı ve hiçbir uygulamanın kuralı bunu yapmamalı
# { $setting } is a part of a firewall rule, as the scan of its PC described it.
core-policy-firewall-not-carried = şunu belirtiyor: { $setting }; bu, olduğu gibi ayarlanamadı veya diğer kuralların etkisini zayıflatacaktı
core-policy-firewall-service = bu paketin geri yüklemediği bir hizmet için
core-policy-firewall-parts = bir kısmı, bir güvenlik duvarı kuralının belirtebileceği bir şey değil
core-policy-folder-already-there = bu klasör yükleme konumunun dışında zaten vardı ve bu uygulamaya ait olduğunu gösteren bir şey yok; bu nedenle içine hiçbir program konmaz
core-policy-variable-name = adı geçerli bir değişken adı değil
core-policy-variable-relied-on = Windows ve diğer programlar bu değişkene dayanır
core-policy-task-name = adı Windows'un kabul ettiği bir ad değil
core-policy-task-folder = klasörü Windows'un kabul ettiği bir klasör değil
core-policy-task-windows = Windows'un kendi görevlerinden birinin yerine geçecekti
core-policy-task-other-action = program çalıştırmaktan başka bir şey yapıyor
core-policy-task-no-program = çalıştırılacak bir program belirtmiyor
# { $path } is the program the task runs.
core-policy-task-runs-other = bu paketin yüklemediği bir şeyi çalıştırıyor: { $path }
core-policy-task-exists-unreadable = bu adda bir görev zaten var ve okunamadı
core-policy-task-exists-other = bu adda bir görev zaten var ve uygulamayı çalıştırmıyor
# { $path } is the program the task already on this PC runs.
core-policy-task-exists-runs = bu adda bir görev zaten var ve buradaki önceki bir yüklemeden gelmeyen { $path } dosyasını çalıştırıyor
core-policy-shortcut-not-lnk = bir .lnk kısayolu değil
core-policy-shortcut-place = Başlat menüsünde veya masaüstünde değil
core-policy-short-name = yolu, denetlenemeyen bir ad kullanıyor
core-policy-shortcut-no-folder = bu bilgisayarda bunun için böyle bir klasör yok
core-policy-data-startup = bir Başlangıç klasörüne gidecek ve her oturum açılışında çalışacaktı
core-policy-data-powershell = PowerShell'in başlarken bulduklarını çalıştırdığı yere gidecekti
core-policy-data-windows = Windows'a ait bir klasöre gidecekti
core-policy-license-outside = yükleme konumunun ve bilinen tüm veri klasörlerinin dışında
core-rewrite-cdata = bir CDATA bölümü içeriyor
core-rewrite-too-much-to-look-through = bu paketin ayarlarında, adlandırdıkları şeylerin denetlenemeyeceği kadar çok şey var

## Warnings a review gives, which the installer shows with what it sets up.

# { $kind } says what was left out, and is not shown itself. { $item } is its path or name, or a registry value such
# as HKLM\Software\App\Name. { $reason } is why, as the second half of the sentence, such as "it cannot be a whole
# drive".
core-warning-review-left-out =
    { $kind ->
        [file] { $item } dosyası dışarıda bırakıldı: { $reason }.
        [folder] { $item } klasörü dışarıda bırakıldı: { $reason }.
        [service] { $item } hizmeti dışarıda bırakıldı: { $reason }.
        [driver] { $item } sürücüsü dışarıda bırakıldı: { $reason }.
        [firewall-rule] { $item } güvenlik duvarı kuralı dışarıda bırakıldı: { $reason }.
        [license-data] { $item } konumundaki lisans verileri dışarıda bırakıldı: { $reason }.
        [license-value] { $item } lisans değeri dışarıda bırakıldı: { $reason }.
        [scheduled-task] { $item } zamanlanmış görevi dışarıda bırakıldı: { $reason }.
        [shortcut] { $item } kısayolu dışarıda bırakıldı: { $reason }.
        [environment-variable] { $item } ortam değişkeni dışarıda bırakıldı: { $reason }.
       *[value] { $item } dışarıda bırakıldı: { $reason }.
    }
core-warning-review-package-path-missing = Kopyalanabilir bir dosyanın pakete göre yolu yok.
# { $count } is the most empty folders a package makes.
core-warning-review-too-many-empty-folders = Bu paket { $count } taneden fazla boş klasör listeliyor ve yalnızca bu kadarı oluşturulur.
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Documents folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-documents-elsewhere =
    { $count ->
        [one] Belgeler klasöründeki { $count } dosya bunun yerine { $target } altında, _external adlı bir klasöre gider; çünkü bu bilgisayarın Belgeler klasörü { $folder } konumunda ve yalnızca profilinizin içindeki bir klasöre yazılır. Bu dosyayı kullanmak için onu kendiniz Belgeler klasörünüze taşıyın.
       *[other] Belgeler klasöründeki { $count } dosya bunun yerine { $target } altında, _external adlı bir klasöre gider; çünkü bu bilgisayarın Belgeler klasörü { $folder } konumunda ve yalnızca profilinizin içindeki bir klasöre yazılır. Bu dosyaları kullanmak için onları kendiniz Belgeler klasörünüze taşıyın.
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-documents-unknown =
    { $count ->
        [one] Belgeler klasöründeki { $count } dosya bunun yerine { $target } altında, _external adlı bir klasöre gider; çünkü bu bilgisayarın Belgeler klasörünün nerede olduğu belirlenemedi. Bu dosyayı kullanmak için onu kendiniz Belgeler klasörünüze taşıyın.
       *[other] Belgeler klasöründeki { $count } dosya bunun yerine { $target } altında, _external adlı bir klasöre gider; çünkü bu bilgisayarın Belgeler klasörünün nerede olduğu belirlenemedi. Bu dosyaları kullanmak için onları kendiniz Belgeler klasörünüze taşıyın.
    }
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Saved Games folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-elsewhere =
    { $count ->
        [one] Kayıtlı Oyunlar klasöründeki { $count } dosya bunun yerine { $target } altında, _external adlı bir klasöre gider; çünkü bu bilgisayarın Kayıtlı Oyunlar klasörü { $folder } konumunda ve yalnızca profilinizin içindeki bir klasöre yazılır. Bu dosyayı kullanmak için onu kendiniz Kayıtlı Oyunlar klasörünüze taşıyın.
       *[other] Kayıtlı Oyunlar klasöründeki { $count } dosya bunun yerine { $target } altında, _external adlı bir klasöre gider; çünkü bu bilgisayarın Kayıtlı Oyunlar klasörü { $folder } konumunda ve yalnızca profilinizin içindeki bir klasöre yazılır. Bu dosyaları kullanmak için onları kendiniz Kayıtlı Oyunlar klasörünüze taşıyın.
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-unknown =
    { $count ->
        [one] Kayıtlı Oyunlar klasöründeki { $count } dosya bunun yerine { $target } altında, _external adlı bir klasöre gider; çünkü bu bilgisayarın Kayıtlı Oyunlar klasörünün nerede olduğu belirlenemedi. Bu dosyayı kullanmak için onu kendiniz Kayıtlı Oyunlar klasörünüze taşıyın.
       *[other] Kayıtlı Oyunlar klasöründeki { $count } dosya bunun yerine { $target } altında, _external adlı bir klasöre gider; çünkü bu bilgisayarın Kayıtlı Oyunlar klasörünün nerede olduğu belirlenemedi. Bu dosyaları kullanmak için onları kendiniz Kayıtlı Oyunlar klasörünüze taşıyın.
    }

## Errors when a folder that only administrators may change cannot be made so, or checked.

# { $path } is a folder.
core-privilege-not-a-folder = { $path } bir klasör değil, bir bağlantı veya dosya
# { $path } is a folder. { $error } is what Windows said, in its own words.
core-privilege-not-made-here = { $path } WinAppMigrator tarafından oluşturulmadı ve içindekileri başkaları değiştirebilir. Bir yönetici bunu denetleyip kaldırmalıdır: { $error }
# { $path } is a folder.
core-privilege-others-put-something-in = { $path } içinde WinAppMigrator tarafından oluşturulmamış bir şey var ve başkaları onu değiştirebilir. Bir yönetici bunu denetleyip kaldırmalıdır
# { $path } is a folder.
core-privilege-became-a-link-in-use = { $path } kullanılırken bir bağlantıya dönüştü
# { $path } is a folder.
core-privilege-became-a-link-looked-at = { $path } incelenirken bir bağlantıya dönüştü
# { $path } is a file. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-users-cannot-start = kullanıcıların { $path } dosyasını başlatmasına izin verilemedi: { $said }
core-privilege-elevation-unknown = bu işlemin yükseltilmiş olup olmadığı belirlenemedi
# { $path } is a folder. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-cannot-restrict = { $path } klasörüne erişim yalnızca yöneticilerle sınırlandırılamadı: { $said }
# { $path } is a folder. { $said } is the folder's permissions as Windows wrote them, and what else it printed.
core-privilege-not-restricted = { $path } klasörüne erişim yalnızca yöneticilerle sınırlandırılamadı ve sürücüsü izinleri tutmuyor olabilir: { $said }
# { $path } is a folder.
core-privilege-written-to-first = başka bir işlem, { $path } klasörü korunmadan önce oraya yazdı
core-privilege-no-program-data = Windows, ProgramData klasörünün nerede olduğunu belirtmiyor

## Checks of a package's files, and of what an install put in place. Each is said of one file, value, service, or the
## like, which the report names beside it.

# { $path } is the installer's file.
core-verify-no-package = { $path } içinde paket yok
# { $path } is a folder.
core-verify-not-unpacked = bu yükleyicinin paketi { $path } konumuna açılmadı
core-verify-schema-supported = Paket bildirimi şeması destekleniyor.
core-verify-packaged-file-missing = Paketlenmiş dosya eksik.
core-verify-no-package-path = Kopyalanabilir dosyanın pakete göre yolu yok.
core-verify-folder-there = Boş klasör mevcut.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-unreadable = Denetlemek için okunamadı: { $error }
core-verify-restored-file-missing = Geri yüklenen dosya eksik.
# { $expected } and { $actual } are sizes in bytes.
core-verify-size-mismatch = Dosya boyutu eşleşmiyor. Beklenen: { $expected } bayt, bulunan: { $actual } bayt.
core-verify-no-hash = Bildirim bu dosya için karma kaydetmiyor, bu nedenle dosya denetlenemez.
core-verify-hash-mismatch = SHA-256 karması eşleşmiyor.
core-verify-file-matches = Dosya boyutu ve karması bildirimle eşleşiyor.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-payload-failed = Lisans yükü doğrulaması başarısız oldu: { $error }
core-verify-license-payload-missing = Paketlenmiş lisans yükü eksik.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-failed = Lisans deposu doğrulaması başarısız oldu: { $error }
core-verify-license-store-file-missing = Paketlenmiş lisans deposu dosyası eksik.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-path-invalid = Paketlenmiş lisans deposu yolu geçersiz: { $error }
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-prerequisite-failed = Önkoşul yükü doğrulaması başarısız oldu: { $error }
core-verify-prerequisite-missing = Paketteki önkoşul yükü eksik.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-failed = Sürücü görüntüsü doğrulaması başarısız oldu: { $error }
core-verify-bundled-driver-missing = Paketteki sürücü görüntüsü eksik.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-path-invalid = Paketteki sürücü yolu geçersiz: { $error }
core-verify-prerequisite-present = Çalışma zamanı önkoşulu hedefte mevcut.
core-verify-prerequisite-not-detected = Çalışma zamanı önkoşulu algılanmadı; yeniden başlatma veya el ile tamamlama gerekebilir (kesin sonuç için geri yükleme günlüğüne bakın).
core-verify-prerequisite-not-probed = Önkoşulun varlığı kayıt defterinden denetlenemiyor; yükleme en iyi çabayla yapıldı.
core-verify-license-file-missing = Geri yüklenen lisans dosyası eksik.
core-verify-license-store-restored-missing = Geri yüklenen lisans deposu dosyası eksik.
core-verify-registry-key-there = Kayıt defteri anahtarı mevcut.
core-verify-registry-value-matches = Kayıt defteri değeri arşivlenen verilerle eşleşiyor.
core-verify-registry-value-differs = Kayıt defteri değeri arşivlenen verilerle eşleşmiyor.
core-verify-firewall-rule-there = Windows Güvenlik Duvarı'nda bu kural var.
core-verify-variable-matches = Ortam değişkeninin değeri arşivlenen verilerle eşleşiyor.
core-verify-variable-differs = Ortam değişkeninin değeri arşivlenen verilerle eşleşmiyor.
# { $error } is what Windows said, in its own words.
core-verify-variable-unreadable = Ortam değişkeni eksik veya okunamıyor: { $error }
core-verify-service-runs-other = Hizmet mevcut, ancak geri yüklenen ikili dosyayı bağımsız değişkenleriyle çalıştırmıyor.
core-verify-service-other-account = Hizmet mevcut, ancak beklenen hesapla çalışmıyor.
core-verify-service-matches = Hizmet mevcut ve geri yüklenen ikili dosyayı bağımsız değişkenleri ve hesabıyla çalıştırıyor.
core-verify-service-missing = Hizmet eksik.
# { $error } is what Windows said, in its own words.
core-verify-service-unreadable = Hizmet okunamıyor: { $error }
core-verify-driver-other-image = Sürücü hizmeti, geri yüklenen görüntüyü göstermiyor.
core-verify-driver-not-registered = Paketteki sürücü hizmeti geri yüklemeden sonra kayıtlı değil.
# { $error } is what Windows said, in its own words.
core-verify-driver-unreadable = Sürücü hizmeti okunamıyor: { $error }
core-verify-driver-image-missing = Sürücü görüntüsü eksik.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-unchecked = Sürücü görüntüsü denetlenemedi: { $error }
core-verify-task-matches = Zamanlanmış görev mevcut ve geri yüklenen programları çalıştırıyor.
core-verify-task-runs-other = Zamanlanmış görev mevcut, ancak geri yüklenen programları çalıştırmıyor.
core-verify-task-missing = Zamanlanmış görev eksik.
# { $error } is what Windows said, in its own words.
core-verify-task-unreadable = Zamanlanmış görev sorgulanamadı: { $error }
core-verify-shortcut-matches = Kısayol mevcut ve hedef yolu geri yüklenen hedefle eşleşiyor.
core-verify-shortcut-opens-other = Kısayol mevcut, ancak hedef yolu geri yüklenen hedefle eşleşmiyor.
core-verify-shortcut-missing = Kısayol eksik.
# { $error } is what Windows said, in its own words.
core-verify-shortcut-unreadable = Kısayol sorgulanamadı: { $error }

## Warnings and errors a check gives, which the app and the installer show.

core-warning-verify-schema-unsupported = Paket bildirimi şeması desteklenmiyor.
# { $error } is what went wrong, in Windows' or the program's own words.
core-warning-verify-package-file-failed = Paket dosyası doğrulaması başarısız oldu: { $error }
core-warning-verify-folder-missing = Boş klasör eksik.
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-key-missing = Kayıt defteri anahtarı eksik veya okunamıyor: { $error }
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-value-missing = Kayıt defteri değeri eksik veya okunamıyor: { $error }
core-warning-verify-registry-kind-missing = Kayıt defteri değerinin türü eksik.
core-warning-verify-firewall-rule-missing = Windows Güvenlik Duvarı'nda bu kural yok.
core-warning-verify-service-scope = Hizmet doğrulaması HKLM hizmet yapılandırmasını okur ve hizmetleri başlatmaz.
core-warning-verify-driver-manual = Üreticiye ait veya Windows ile gelen bir ya da daha fazla sürücü WinAppMigrator tarafından dağıtılmaz; bunları hedefte üreticinin imzalı sürücü paketiyle yeniden yükleyin.
core-warning-verify-task-scope = Zamanlanmış görev doğrulaması yalnızca kaydı ve komut başvurusunu denetler.

## The verification report written beside a package, verification_report.md. Its headings follow a "#".

core-verify-report-title = WinAppMigrator Doğrulama Raporu
# { $time } is when, as a date and time in UTC, such as 2026-11-15T10:00:00Z.
core-verify-report-verified = Doğrulama zamanı (UTC): { $time }
# { $valid } is true or false, which are not translated.
core-verify-report-package-valid = Paket geçerli: { $valid }
# { $valid } is true or false, which are not translated.
core-verify-report-restore-valid = Geri yükleme geçerli: { $valid }
core-verify-report-restore-not-run = Geri yükleme geçerli: çalıştırılmadı
core-verify-report-items = Öğeler
# { $message } is what a check found, and { $source } the file or setting it looked at.
core-verify-report-item-from = { $message } Kaynak: { $source }
core-verify-report-warnings = Uyarılar
core-verify-report-errors = Hatalar
core-verify-report-none = Yok.
