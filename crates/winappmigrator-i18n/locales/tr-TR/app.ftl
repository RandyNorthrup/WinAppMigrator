# Words of the app, WinAppMigrator itself: its one window, with the installed apps listed on the left and, on the
# right, what goes in an installer of the one picked and the button that builds it; and the steps to read another PC
# through WinAppMigrator Client. Message names start with app-.


## The window and its header

# The program's name, in the window's title bar and at the top of the window. Keep it as it is unless your language
# writes product names in a script of its own.
app-name = WinAppMigrator
app-tagline = Yüklü bir uygulamayı kendiliğinden ayıklanan bir yükleyiciye dönüştürür
app-theme-to-light = Açık temaya geç
app-theme-to-dark = Koyu temaya geç
app-reload-list = Uygulama listesini yenile
app-list-loading = Okunuyor
# { $count } is how many apps are listed.
app-list-count =
    { $count ->
        [one] { $count } uygulama
       *[other] { $count } uygulama
    }
app-list-load-failed = Okunamadı
# Shown when the window cannot open at all. { $error } is what went wrong, in the words of Windows or a graphics driver.
app-window-could-not-open = Pencere açılamadı: { $error }

## The list of installed apps

app-list-search = Ada veya yayımcıya göre ara  (Ctrl+F)
app-list-filter-all = Tümü
# Whether an app is installed for one account or for everyone on the PC.
app-scope-per-user = Kullanıcı başına
app-scope-per-machine = Makine başına
app-scope-unknown = Bilinmiyor
# An app whose processor architecture (x86, x64, ARM64) is not known.
app-arch-unknown = Bilinmiyor
app-list-loading-apps = Yüklü uygulamalar okunuyor…
# { $error } is why the apps could not be listed.
app-list-could-not-list = Uygulamalar listelenemedi: { $error }
app-list-no-matches = Eşleşen uygulama yok
app-list-no-matches-hint = Arama metnini veya kapsam filtresini değiştirin.
app-list-worker-disconnected = Uygulama listesini okuyan işlemle bağlantı kesildi.
# What a screen reader says of an app's row. { $name } is the app's name, { $publisher } who made it, and { $version } its
# version, such as 11.4.2, or — when either is not known.
app-list-row = { $name }, { $publisher }, sürüm { $version }

## The list's columns, and what a screen reader says each column's heading does when clicked

app-column-name = Ad
app-column-name-sort = Ada göre sırala
app-column-publisher = Yayımcı
app-column-publisher-sort = Yayımcıya göre sırala
app-column-version = Sürüm
app-column-version-sort = Sürüme göre sırala
app-column-scope = Kapsam
app-column-scope-sort = Kapsama göre sırala
# Short for architecture: x86, x64, or ARM64. The column is narrow.
app-column-arch = Mimari
app-column-arch-sort = Mimariye göre sırala

## Where apps are read from: this PC, a drive from another PC, a disk image of one, or another PC on the network

app-source = Tarama kaynağı
app-source-this-pc = Bu bilgisayar
app-source-drive = Bağlı sürücü…
app-source-image = Disk görüntüsü…
app-source-network = Ağ bilgisayarı…
app-source-attaching-image = Görüntü salt okunur olarak bağlanıyor…
# { $computer } is the other PC's name, such as OLD-PC, and { $address } its network address, such as 192.168.1.20.
app-source-network-pc = Ağ üzerinden { $computer } ({ $address })
app-source-disconnect = Bağlantıyı kes
app-source-change = Değiştir…
# The title of the window that picks a disk image. .vhd and .vhdx are file extensions.
app-source-pick-image-title = Diğer bilgisayarın disk görüntüsünü seçin (.vhd veya .vhdx)
app-source-pick-image-filter = Disk görüntüleri
# { $path } is the disk image file.
app-source-image-has-no-windows = { $path } dosyasında Windows içeren bir birim yok.
app-source-image-not-attached = Disk görüntüsü bağlanamadı.
app-source-pick-drive-title = Bağlı sürücünün kökünü seçin (örn. E:\)
# { $path } is the folder picked, such as E:\. \Windows and \Users are folder names, as Windows writes them.
app-source-not-a-windows-drive = { $path } konumu bir Windows sistem sürücüsüne benzemiyor (\Windows ve \Users yok). Diğer bilgisayardan gelen sürücünün kökünü seçin.
# Said of a drive or disk image with a Windows older than Windows 10. { $windows } is its name, such as "Windows 7
# Professional Service Pack 1, 32-bit". { $is_32_bit } is "yes" when that Windows is 32-bit, which adds the last
# sentence, and "no" when it is not.
app-source-older-windows =
    Burada { $windows } yüklü. Buradan oluşturulan yükleyiciler Windows 10 ve 11 için hazırlanır. Kullanıcı verileri seçeneğiyle, Windows'un eski bir uygulama için sessizce ayrı tuttuğu ayarlar da gelir ve DirectX 9 gibi Windows 11'de bulunmayan çalışma zamanları uygulamayla birlikte yüklenir.{ $is_32_bit ->
        [yes] { " " }Buradaki uygulamalar 32 bit olduğundan, 64 bit Windows'un bu tür uygulamaları tuttuğu Program Files (x86) klasörüne gider.
       *[no] { "" }
    }
# The same, of a Windows whose name could not be read. { $is_32_bit } is as above.
app-source-older-windows-unnamed =
    Burada eski bir Windows yüklü. Buradan oluşturulan yükleyiciler Windows 10 ve 11 için hazırlanır. Kullanıcı verileri seçeneğiyle, Windows'un eski bir uygulama için sessizce ayrı tuttuğu ayarlar da gelir ve DirectX 9 gibi Windows 11'de bulunmayan çalışma zamanları uygulamayla birlikte yüklenir.{ $is_32_bit ->
        [yes] { " " }Buradaki uygulamalar 32 bit olduğundan, 64 bit Windows'un bu tür uygulamaları tuttuğu Program Files (x86) klasörüne gider.
       *[no] { "" }
    }

## The details panel, before an app is picked, and the app picked

app-details-pair-first = Eski bilgisayarla eşleştirin
app-details-pair-first-hint = WinAppMigrator Client orada çalıştıktan ve kodunu yazdıktan sonra oradaki uygulamalar burada listelenir. Bunlardan birinin yükleyicisini, bu bilgisayardaki gibi oluşturursunuz.
app-details-pick-an-app = Bir uygulama seçin
app-details-pick-an-app-hint = Nelerin ekleneceğini seçmek ve yükleyicisini oluşturmak için listeden bir uygulama seçin.
# { $publisher } is who made the app, and { $version } its version, such as 11.4.2.
app-details-publisher-and-version = { $publisher }, sürüm { $version }
# { $version } is the app's version, such as 11.4.2.
app-details-unknown-publisher-and-version = Bilinmeyen yayımcı, sürüm { $version }
app-details-unknown-publisher = Bilinmeyen yayımcı

## What goes in besides the program

app-option-license = Lisans ve etkinleştirme verileri
app-option-license-hint = Bu bilgisayara bağlı olabilir ve yeni bilgisayarda yeniden etkinleştirme gerektirebilir.
app-option-user-data = Kullanıcı verileri
# Documents, Saved Games, and ProgramData are folders of Windows: use the names Windows gives them in your language,
# except ProgramData, which keeps its name.
app-option-user-data-hint = Uygulama ayarları ve geçmişi, Belgeler, Kayıtlı Oyunlar ve ProgramData.
app-option-whose-data = Kimin verileri
app-option-whose-data-hint = Bir yükleyici tek bir hesabın verilerini içerir. Bu veriler, yükleyicinin çalıştığı yerde oturum açmış kişiye gider.

## Building the installer

app-output-folder = Çıkış klasörü
# An example of a folder, shown in the empty field. "You" stands for the name of an account.
app-output-folder-example = C:\Users\Kullanıcı\WinAppMigrator
app-output-folder-not-full = D:\Yükleyiciler gibi tam bir klasör yolu girin.
app-build-create = Yükleyici oluştur
app-build-create-again = Yükleyiciyi yeniden oluştur
app-build-cancel = İptal
app-build-cancelling = Geçerli adımdan sonra iptal ediliyor…
app-build-cancelled = Oluşturma iptal edildi. Hiçbir yükleyici yazılmadı.
# winappmigrator-installer.exe is a file name.
app-build-no-stub = Yükleyici gövdesi (winappmigrator-installer.exe) bu uygulamanın yanında bulunamadı, bu nedenle kendiliğinden ayıklanan bir yükleyici oluşturulamaz.
app-build-select-an-app-first = Önce taşınacak bir uygulama seçin.
app-build-look-again-first = Yükleyicisinin oluşturulabilmesi için bu uygulamanın yeniden incelenmesi gerekiyor.
# { $reason } is what went wrong, in the program's own words, often in English.
app-build-stopped-unexpectedly = Oluşturma beklenmedik şekilde durdu: { $reason }. Çıkış klasöründe bir yükleyici kaldıysa onu kullanmayın.
app-build-worker-disconnected = Taşıma işlemiyle bağlantı kesildi.
# { $path } is the output folder, and { $error } what Windows said went wrong.
app-build-no-output-folder = { $path } çıkış klasörü oluşturulamadı: { $error }
# { $error } is what went wrong.
app-build-no-staging-folder = Korumalı bir hazırlık klasörü oluşturulamadı, bu nedenle hiçbir şey oluşturulmadı: { $error }
# { $error } is what went wrong, as a sentence.
app-build-could-not-build = Yükleyici oluşturulamadı. { $error }
app-build-no-installer = Oluşturma bitti, ancak ortada bir yükleyici yok.
# { $error } is what went wrong.
app-build-hash-check-did-not-run = Yükleyici yazıldı, ancak karma denetimi çalıştırılamadı: { $error }. Bu yükleyiciyi kullanmayın.

## How far a build has got

app-progress-preparing = Nelerin ekleneceği belirleniyor…
app-progress-checking-room = Yeterli alan olup olmadığı denetleniyor…
app-progress-bundling = Çalışma zamanları ve sürücüler pakete ekleniyor…
# { $done } is how many files have been read so far, and { $total } how many there are.
app-progress-packing = Dosyalar okunuyor ve sıkıştırılıyor: { $done } / { $total }
# { $done } is how many of the installer's records have been looked over so far, and { $total } how many there are.
app-progress-looking-over = Yükleyicinin kayıtları gözden geçiriliyor: { $done } / { $total }…
app-progress-writing = Yükleyici yazılıyor…
app-progress-verifying = Karmalar doğrulanıyor…

## What a build made

app-result-failed-hash-check = Bu yükleyiciyi kullanmayın: Karma denetiminden geçemedi.
# { $count } is how many files were gone when the build came to read them.
app-result-files-gone =
    { $count ->
        [one] Yükleyici oluşturuldu, ancak uygulama incelendikten sonra { $count } dosya kaybolduğu için yükleyicide yer almıyor. Aşağıdaki Uyarılar bölümüne bakın.
       *[other] Yükleyici oluşturuldu, ancak uygulama incelendikten sonra { $count } dosya kaybolduğu için yükleyicide yer almıyor. Aşağıdaki Uyarılar bölümüne bakın.
    }
# { $count } is how many files could not be read into the installer.
app-result-files-not-captured =
    { $count ->
        [one] Yükleyici oluşturuldu, ancak { $count } dosya alınamadı; genellikle bunun nedeni dosyanın kilitli veya okunamaz olmasıdır. Uygulama bu dosya olmadan çalışmayabilir. Aşağıdaki Uyarılar bölümüne bakın.
       *[other] Yükleyici oluşturuldu, ancak { $count } dosya alınamadı; genellikle bunun nedeni dosyaların kilitli veya okunamaz olmasıdır. Uygulama bu dosyalar olmadan çalışmayabilir. Aşağıdaki Uyarılar bölümüne bakın.
    }
app-result-mentions-who = Yükleyici oluşturuldu, ancak hâlâ kimden geldiğine dair bilgiler içeriyor. Kimseye vermeden önce aşağıda yazılanları okuyun.
app-result-ready = Yükleyici hazır. Paketi karma denetiminden geçti.
app-result-files = Dosyalar
app-result-installer-size = Yükleyici boyutu
app-result-warnings = Uyarılar
app-result-log = Günlük
app-result-copy-error = Hatayı kopyala
app-result-installer = Yükleyici
app-result-data-file = Veri dosyası
app-result-open-folder = Klasörü aç
app-result-copy-path = Yolu kopyala
# { $error } is what Windows said went wrong.
app-result-could-not-open-folder = Klasör açılamadı: { $error }
# { $installer } and { $data_file } are file names, such as App_installer.exe and App_installer.wamdata.
app-result-two-files = Bu uygulama tek bir yürütülebilir dosya için fazla büyük, bu nedenle yükleyicisi iki dosyadan oluşur: { $installer } ve { $data_file }. Bu dosyalar birlikte kalmalıdır: İkisini de diğer bilgisayarda aynı klasöre, bu adlarla kopyalayın. Yükleyiciyi imzalarsanız bu imza veri dosyasını da kapsar.
# { $app_name } is the app's name. "Unknown publisher" is what Windows shows when it asks to run an unsigned program: use
# Windows' own words for it in your language.
app-result-run-it = { $app_name } uygulamasını yeniden yüklemek için bunu diğer bilgisayarda çalıştırın. Dosyalarını SHA-256 ile denetler. Oluşturduğunuz yükleyiciler imzalı olmadığı için Windows “Bilinmeyen yayımcı” uyarısı gösterir.
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-verification-errors = Doğrulama hataları ({ $count })
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-warnings-heading = Uyarılar ({ $count })
# After the first of a long list of warnings. { $count } is how many more there are.
app-warnings-and-more = ve { $count } tane daha.

## Looking at an app, to see what an installer of it would hold

app-scan-getting-ready = Bu uygulamayı incelemek için hazırlanılıyor…
app-scan-install-record = Nasıl yüklendiği okunuyor…
app-scan-near-matches = Uygulamanın parçası olabilecek başka şeyler aranıyor…
app-scan-system-items = Uygulamanın hizmetleri, görevleri ve kısayolları aranıyor…
app-scan-files = Uygulamanın dosyaları inceleniyor…
app-scan-registry = Uygulamayı Windows'a bağlayan öğeler okunuyor…
app-scan-finishing = Tamamlanıyor…
app-scan-look-again = Yeniden incele
app-scan-stop = Durdur
app-scan-running-hint = Bu işlem sürerken başka bir uygulama seçebilir veya eklenecekleri değiştirebilirsiniz.
# { $error } is why, in words that follow a colon.
app-scan-failed = Bu uygulama incelenemedi: { $error }
app-scan-try-again = Yeniden dene
# "Look Again" names the button of that name.
app-scan-stopped = Durduruldu. Bu uygulama incelendikten sonra yükleyici oluşturulabilir.
# { $reason } is what went wrong, in the program's own words, often in English.
app-scan-stopped-unexpectedly = Beklenmedik şekilde durdu: { $reason }.

## How big an installer can be, and whether there is room for it

app-room-files = Dosyalar
# Over how big the installer can be: up to that size.
app-room-up-to = En fazla
app-room-free = Boş
# { $drive } is a drive, such as C:.
app-room-free-on = { $drive } sürücüsünde boş
# How much room a drive has, when it would not say.
app-room-free-unknown = Bilinmiyor
app-room-most-it-could-need = Sıkıştırmadan önce gerekebilecek en fazla alan.
# { $drive } is a drive, such as C:, { $needed } how much room it needs, such as 1.2 GB, and { $free } how much it has.
app-room-drive-short = Bunu oluşturmak için { $drive } sürücüsünde en fazla { $needed } boş alan gerekiyor; sürücüde { $free } boş alan var.
# { $drives } is one sentence like the one above for each drive without the room, one after another.
app-room-not-enough = Yeterli alan yok. { $drives } Başka bir sürücüde bir çıkış klasörü seçin veya bazı öğeleri dışarıda bırakın.
# { $note } is a sentence, without its full stop, about a drive whose room could not be checked. Use your language's
# full stop.
app-room-could-not-tell = { $note }.

## What goes in the installer, part by part

app-footprint-heading = Yükleyiciye girenler
app-footprint-working-out = Bununla birlikte nelerin gideceği belirleniyor…
# { $error } is why.
app-footprint-would-not-make-a-package = Kalanlarla bir paket oluşturulamaz: { $error }
# { $list } is the names of what else is left out, such as a service and a shortcut.
app-footprint-also-left-out = Bu, şunları da dışarıda bırakır: { $list }. Bunların çalıştırdığı veya adlandırdığı şey orada olmayacağından diğer bilgisayarda ayarlanmayacaklar.
# { $list } is the names of the first few of what else is left out, and { $count } how many more there are.
app-footprint-also-left-out-and-more = Bu, şunları da dışarıda bırakır: { $list } ve { $count } öğe daha. Bunların çalıştırdığı veya adlandırdığı şey orada olmayacağından diğer bilgisayarda ayarlanmayacaklar.
# { $list } is the names of license files. The option is the one named "License and Activation Data".
app-footprint-licenses-still-go = Dışarıda bıraktıklarınızın içindeki lisans verileri yine de eklenir, çünkü bunlar Lisans ve etkinleştirme verileri seçeneğine bağlıdır: { $list }.
# { $elapsed } is how long ago, in minutes and seconds such as 45:00, or hours, minutes, and seconds such as 1:05:00.
# "Look Again" names the button of that name.
app-footprint-looked-at-long-ago = Bu uygulama { $elapsed } önce incelendi. Dosyalar oluşturma sırasında okunduğundan değişiklikler de alınır ve o zamana kadar kaybolanlar listelenir. Yeni eklenenler için Yeniden incele düğmesini kullanın.
# Headings of the parts of the list, each with a line that says what is in it.
app-footprint-program-files = Program dosyaları
app-footprint-data-folders = Veri klasörleri
# What could go in with the User Data option, when nothing else is in this part. { $size } is how much, such as 1.2 GB.
app-footprint-folders-with-user-data = Kullanıcı verileri seçeneğiyle { $size }
# { $included } is how many folders go in, { $count } how many there are, and { $size } how much those that go in hold.
app-footprint-folders-summary =
    { $count ->
        [one] { $included } / { $count } klasör, { $size }
       *[other] { $included } / { $count } klasör, { $size }
    }
# { $size } is how much, such as 1.2 GB, and { $count } in how many files.
app-footprint-data-only-with-user-data =
    { $count ->
        [one] Bu uygulamanın verilerinden { $count } dosyadaki { $size } yalnızca Kullanıcı verileri seçeneğiyle eklenir.
       *[other] Bu uygulamanın verilerinden { $count } dosyadaki { $size } yalnızca Kullanıcı verileri seçeneğiyle eklenir.
    }
# A folder's row. { $count } is how many files it holds, and { $detail } where it is or why it cannot be left out.
app-footprint-folder-row =
    { $count ->
        [one] { $count } dosya. { $detail }
       *[other] { $count } dosya. { $detail }
    }
app-footprint-services = Hizmetler
app-footprint-startup = Otomatik başlayanlar
app-footprint-tasks = Zamanlanmış görevler
app-footprint-shortcuts = Kısayollar
# { $included } is how many of the part's things go in, and { $count } how many there are.
app-footprint-items-summary = { $included } / { $count }
# { $why } is what else was left out that this goes with.
app-footprint-left-out-too = Bu da dışarıda bırakıldı: { $why }
# A row's detail. { $kind } is what it is, such as Service, and { $detail } what it does.
app-footprint-change-detail = { $kind }. { $detail }
app-footprint-integrations = Windows ile bağlantıları
# { $included } is how many kinds go in, such as file types and firewall rules, and { $count } how many kinds there are.
app-footprint-integrations-summary =
    { $count ->
        [one] { $included } / { $count } tür
       *[other] { $included } / { $count } tür
    }
app-footprint-integration-nothing-left = Dışarıda bıraktıklarınız nedeniyle bundan hiçbir şey kalmadı
# { $count } is how many things of this kind the installer sets up.
app-footprint-integration-things =
    { $count ->
        [one] { $count } öğe ayarlanır
       *[other] { $count } öğe ayarlanır
    }
app-footprint-prerequisites = Çalışması için gerekenler
# { $count } is how many runtimes, such as Visual C++ Redistributable.
app-footprint-prerequisites-summary =
    { $count ->
        [one] { $count } çalışma zamanı
       *[other] { $count } çalışma zamanı
    }
app-footprint-near-matches = Bunun da parçası olabilir
# { $ticked } is how many were ticked, and { $count } how many there are.
app-footprint-near-matches-summary = { $ticked } / { $count } eklendi
app-footprint-near-matches-hint = İşaretlemediğiniz sürece yükleyiciye eklenmez. Birini işaretlediğinizde uygulama, diğerleriyle aynı denetimlerle yeniden incelenir; bu nedenle parolaya benzeyen bir şey yine dışarıda kalır.
# { $why } is why it might be part of the app, as a sentence.
app-footprint-near-match-with-user-data = Kullanıcı verileri seçeneğiyle gelir. { $why }
# How much a folder holds, when not all of it could be measured. { $size } is how much was, such as 1.2 GB.
app-footprint-near-match-over = { $size } üzeri
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-folder = Klasör: { $place }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-hidden = Bir uygulamanın gizli parçası: { $place }
app-footprint-nothing-left-out = Parolaya, lisansa veya sertifikaya benzediği için dışarıda bırakılan bir şey yok.
app-footprint-left-out = Dışarıda bırakılanlar ve nedenleri
# { $count } is how many things were left out.
app-footprint-left-out-summary =
    { $count ->
        [one] { $count } öğe
       *[other] { $count } öğe
    }
# Above a list of things left out for one reason. { $title } is the reason.
app-footprint-left-out-group = { $title }:
app-footprint-refused = Hiçbir yükleyicinin bunları ayarlamasına izin verilmez:
# A heading over a list that opens and closes. { $count } is how many are in it.
app-footprint-worth-knowing = Bilinmesi gerekenler ({ $count })
# { $list } is what kinds of things were looked for and none found, from the five below.
app-none-found = Hiçbiri bulunamadı: { $list }.
app-none-found-services = hizmetler
app-none-found-startup = başlangıç öğeleri
app-none-found-tasks = zamanlanmış görevler
app-none-found-shortcuts = kısayollar
app-none-found-integrations = uygulamayı Windows'a bağlayan herhangi bir şey

## Network PC: reading another PC through WinAppMigrator Client, in three steps. "WinAppMigrator Client" is the name of a
## program, and the old PC is the one the apps are moved off.

app-network-step-client = 1. İstemciyi eski bilgisayara koyun
app-network-step-client-hint = WinAppMigrator Client, uygulamanın şu anda bulunduğu bilgisayarda (Windows 7 veya üstü) çalışır. Yalnızca bu bilgisayarın, gösterdiği bir kodla ve şifrelenmiş olarak orayı okumasına izin verir.
# { $drive } is a USB drive, such as E:\.
app-network-save-to-usb = { $drive } USB sürücüsüne kaydet
# { $drive } is a USB drive, such as E:\, and { $name } the name it was given, such as BACKUP.
app-network-save-to-named-usb = { $drive } USB sürücüsüne kaydet ({ $name })
app-network-save-to-folder = Bir klasöre kaydet…
app-network-look-for-usb-again = USB sürücülerini yeniden ara
app-network-no-usb = Takılı bir USB sürücüsü yok. Bir tane takın veya bir klasöre kaydedin: Eski bilgisayardan erişebileceğiniz bir klasör, örneğin ağınızdaki paylaşılan bir klasör.
# { $path } is where the client was saved, such as E:\WinAppMigrator Client.exe.
app-network-client-saved = { $path } olarak kaydedildi. Eski bilgisayarda taşıdığınız uygulamayı kapatın, ardından bu dosyaya çift tıklayın ve değişiklik yapmasına izin verin. Bir kod gösterecektir.
app-network-step-pick = 2. Eski bilgisayarı seçin
app-network-looking = Ağınızda istemciyi çalıştıran bilgisayarlar aranıyor…
app-network-type-name = Ya da adını veya adresini yazın
# An example, shown in the empty field: a PC's name, or its network address.
app-network-type-name-example = ESKI-PC veya 192.168.1.20
app-network-step-code = 3. Gösterdiği kodu yazın
app-network-code = Eşleştirme kodu
app-network-connect = Bağlan
app-network-pairing = Eşleştiriliyor…
# Why pairing did not work, shown on its own.
app-network-pairing-stopped = eşleştirme tamamlanmadan durdu
app-network-wrong-code = bu, eski bilgisayarın gösterdiği kod değil. Kodu denetleyip yeniden deneyin.
# { $error } is what went wrong.
app-network-not-reached = eski bilgisayara ulaşılamadı: { $error }
app-network-pick-folder-title = WinAppMigrator Client için bir klasör seçin
# { $path } is the folder, and { $error } what Windows said went wrong.
app-client-could-not-save = istemci { $path } klasörüne kaydedilemedi: { $error }
# scripts\build-client.ps1 is a file name.
app-client-not-carried = bu WinAppMigrator derlemesi istemciyi içermiyor. İstemciyi scripts\build-client.ps1 ile derleyin, ardından uygulamayı yeniden derleyin.
# { $error } is what Windows said went wrong.
app-client-could-not-save-note = yanındaki not kaydedilemedi: { $error }
# A note saved as a text file beside the client, for whoever runs it on the old PC. Keep its lines short, as it is read in
# Notepad. "WinAppMigrator Client.exe" is a file name. "Network PC..." names the Scan Source button: use its words in your
# language.
app-client-note =
    WinAppMigrator Client
    
    Bunu, uygulamaları başka bir bilgisayara taşıdığınız bilgisayarda
    çalıştırın: Windows 7, 8, 10 veya 11.
    
    1. Taşıdığınız uygulamayı kapatın.
    2. “WinAppMigrator Client.exe” dosyasına çift tıklayın ve değişiklik
       yapmasına izin verin.
    3. 6 haneli bir eşleştirme kodu gösterir. Yeni bilgisayarınızdaki
       WinAppMigrator'da Ağ bilgisayarı... düğmesini seçin, bu bilgisayarı
       seçin ve kodu yazın.
    4. Yeni bilgisayar işini bitirdiğinde istemcinin penceresinde Enter
       tuşuna basın veya pencereyi kapatın.
    
    Yalnızca okur. Çalışırken Windows sürücüsünün anlık görüntüsünü tutar ve
    yerel ağınızdan kendi iki bağlantı noktasına gelen bağlantılara Windows
    Güvenlik Duvarı'nda izin verir; durduğunda ikisini de kaldırır. Ağ
    üzerinden geçen her şey şifrelenir ve yalnızca kodu bilen bir bilgisayar
    bir şey okuyabilir.
