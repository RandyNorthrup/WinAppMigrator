# Words the core writes for people while it builds a package from an installed app: what a package records for an
# installer to show later, errors shown in the app, the list of what goes in and what is left out, opening a disk
# image or another PC over the network, and what an installer says when the package it carries cannot be read.
# Message names start with core-.


## Scanning an app, before a package is built. Shown in the app when a scan cannot go on.

# { $path } is the app's program, such as D:\Program Files\Acme\Acme.exe, on a drive taken from another PC.
core-scan-program-not-on-the-drive = { $path } bir bağlantı üzerinden gidiyor veya bağlı sürücüde değil, bu nedenle okunmadı.
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-not-on-this-pc = { $path } bu bilgisayarın bir diskinde değil veya başka bir şeyi açacak bir ada sahip, bu nedenle okunmadı.
# { $path } is the app's program.
core-scan-program-someone-elses = { $path } başka bir hesabın klasörlerinde veya bu hesaba ait değil ve bu uygulamanın kendi kaydı orada olması gerektiğini belirtmiyor, bu nedenle okunmadı.
# { $app_name } is the app's name, as the list of apps calls it.
core-scan-nothing-to-package = { $app_name } uygulamasına ait bir program bulunamadı, bu nedenle yükleyiciye konacak bir şey yok. Sürücüler, çalışma zamanları ve eklentiler genellikle böyledir: yeni bilgisayarda sürücüyü üreticisinden veya Windows Update'ten, çalışma zamanını ya da eklentiyi ise onu kullanan uygulamayla birlikte yükleyin.
# { $app_name } is the app's name, as the list of apps calls it. A web app is a website installed as an app, with
# a window and a Start menu entry of its own.
core-scan-web-app = { $app_name }, bir tarayıcının bir web sitesinden yüklediği bir web uygulamasıdır. Kendine ait bir programı yoktur, bu nedenle yükleyiciye konacak bir şey yok. Yeni bilgisayarda web sitesini aynı tarayıcıda açın ve uygulamayı oradan yükleyin.
# { $path } is the program the app's own record names, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-missing = Bu uygulamanın kaydı { $path } dosyasını uygulamanın programı olarak gösterse de dosya artık orada değil. Uygulama taşınmış veya kısmen kaldırılmış olabilir: uygulamayı bu bilgisayarda onarmak veya yeniden yüklemek dosyayı geri getirmelidir.
# { $path } is the file that was picked.
core-scan-not-a-file = tarama hedefi bir dosya değil: { $path }
# Names something left out of a package, in a list of what was left out and why. { $name } is the variable's name,
# such as ACME_LICENSE_KEY.
core-scan-environment-variable-item = { $name } ortam değişkeni

## Folders and parts that might also be the app's, offered in a list to tick. Each says why it is offered.

core-near-starts-with-app-name = Adı bu uygulamanın adıyla başlıyor.
core-near-has-app-name = Adında bu uygulamanın adı geçiyor.
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-starts-with-publisher = Adı yayımcının adıyla başlıyor: { $publisher }.
# { $folder } is a folder's name, such as Program Files.
core-near-hidden-part-beside = Uygulama listesinde gizlenen ve bu uygulamanın yanına, { $folder } klasörüne yüklenmiş bir parça.
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-hidden-part-publisher = Uygulama listesinde gizlenen, aynı yayımcıdan bir parça: { $publisher }.

## What a scan found, as a person chooses from it: the folders that go in, and what is left out, and why

# Why a folder cannot be left out, beside its number of files.
core-footprint-locked-app-folder = Bunlar her zaman ana programla birlikte gider
core-footprint-locked-main-program = Her zaman eklenen ana programı içerir
core-footprint-locked-needed = Ana programın onsuz başlayamayacağı şeyleri içerir
core-footprint-locked-font = Yazı tipiyle birlikte gider (Windows ile bağlantıları altında)
# Where a folder is. "AppData" and "ProgramData" are the names of Windows folders; keep them as they are.
core-footprint-place-roaming = Profilinizdeki ayarlar (AppData)
core-footprint-place-local = Profilinizdeki, bu bilgisayara özel ayarlar (AppData)
core-footprint-place-program-data = Bilgisayardaki herkesle paylaşılan (ProgramData)
core-footprint-place-documents = Belgeler klasörünüzde
core-footprint-place-saved-games = Kayıtlı Oyunlar klasörünüzde
core-footprint-place-public-documents = Genel Belgeler klasöründe
core-footprint-place-app-folder = Uygulamanın klasöründe
# { $folder } is a folder, such as C:\Users\Sam\AppData\Roaming.
core-footprint-place-in = { $folder } içinde
# A row for the files directly in the app's folder. { $folder } is that folder's name, such as Acme.
core-footprint-files-directly-in = Doğrudan { $folder } klasöründeki dosyalar
# Headings over what is left out, and why each item is.
core-footprint-licenses-left-out = Lisans ve etkinleştirme verileri seçeneği kapalı olduğu için bu yükleyicide yok
core-footprint-font-stays-behind = Geride kalır.
core-footprint-drivers-left-out = Windows'a veya bir üreticiye ait olan ve aktarılamayan sürücüler; bunları diğer bilgisayarda yeniden yükleyin
core-footprint-driver-reinstall = Üreticisinin kendi paketinden yeniden yükleyin.
# Why something else is left out along with what the person left out. { $service } is a service's name, such as AcmeSync.
core-footprint-needs-left-out = dışarıda bırakılan { $service } hizmetine ihtiyaç duyuyor
core-footprint-told-to-use-left-out = dışarıda bıraktığınız bir klasördeki bir şeyi kullanacak şekilde ayarlanmış
core-footprint-file-in-left-out = dosyası, dışarıda bıraktığınız bir klasörde
core-footprint-program-in-left-out = ait olduğu program, dışarıda bıraktığınız bir klasörde
core-footprint-names-left-out = dışarıda bıraktığınız bir klasörü adlandırıyor
# Why something is left out, as the installer tells whoever installs it.
core-footprint-builder-left-it-out = bu yükleyiciyi oluşturan kişi bunu dışarıda bıraktı
# Something the person left out, by kind and name, such as "Service Acme Updater". { $kind } is the kind, such as
# Service or Scheduled Task; { $title } is its name.
core-footprint-left-out-item = { $kind }: { $title }

## Building a package and an installer from a scan. Shown in the app when a build cannot go on.

core-package-scan-of-another-app = bu tarama, oluşturulan uygulamadan farklı bir uygulamaya ait, bu nedenle hiçbir şey oluşturulmadı
core-package-scan-brought-in-more = bu tarama, bu oluşturma işleminin istemediği verileri de getirdi, bu nedenle hiçbir şey oluşturulmadı
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-package-program-gone = { $path } artık yok, bu nedenle hiçbir şey oluşturulmadı. Uygulamayı yeniden inceleyin.
core-package-needs-stub = kendiliğinden ayıklanan bir arşiv için yükleyici gövdesi gerekir
# { $reason } is what went wrong.
core-package-staged-file-failed = hazırlık klasöründeki bir dosya pakete eklenemedi: { $reason }
# { $shortfalls } says which drives are short of room and by how much, in sentences of its own.
core-package-not-enough-room = bu yükleyiciyi oluşturmak için yeterli alan yok, bu nedenle hiçbir şey okunmadı veya yazılmadı. { $shortfalls }. Orada biraz yer açın veya başka bir sürücüde bir çıkış klasörü seçin.
# { $path } is a file's path, on a drive taken from another PC.
core-package-not-read-off-the-drive = { $path } bir bağlantı üzerinden gidiyor veya bağlı sürücüde değil, bu nedenle okunmadı
# { $path } is a file's path.
core-package-not-read-not-local = { $path } bu bilgisayarın bir diskinde değil, bu nedenle okunmadı
# { $path } is a file's path.
core-package-not-read-other-account = { $path } başka bir hesabın klasörlerine çıkıyor, bu nedenle okunmadı
# { $path } is a file's path, as the PC it came from called it.
core-package-not-on-the-drive = { $path } bağlı sürücüde değil, bu nedenle okunmadı
# { $path } is a file's path.
core-package-backup-copy-no-parent = üst dizini olmayan bir dosyanın yedekleme kopyası alınamaz: { $path }
# { $path } is a file's path.
core-package-backup-copy-destination-no-parent = yedekleme kopyasının hedefinin üst dizini yok: { $path }
# robocopy is the name of a Windows command; keep it as it is. { $code } is how it ended, such as Some(8).
core-package-robocopy-failed = robocopy ile yedekleme modunda kopyalama başarısız oldu (çıkış kodu { $code })
# Stands for a folder where a file's name would be, as in "I/O failed for temporary package folder: …".
core-package-temporary-folder = geçici paket klasörü
# Stands for the installer being written where a file's name would be, as in "I/O failed for the installer: …".
core-package-the-installer = yükleyici
# { $path } is a file's path, or its name in the package. { $error } is what went wrong with it.
core-package-path-error = { $path }: { $error }
# { $name } is a file's name in the package, such as app/bin/App.exe.
core-package-same-place = pakette aynı yerde başka bir dosya var: { $name }
# { $name } is a file's name in the package.
core-package-changed-in-protected-folder = { $name }, paket oluşturulurken korumalı klasörde değişti
core-package-worker-died = dosyaları sıkıştıran bir iş parçacığı beklenmedik şekilde durdu, bu nedenle hiçbir yükleyici yazılmadı
# { $path } is a file's path.
core-package-link-not-read = { $path } bir bağlantı, bu nedenle okunmadı
# { $path } is a file's path. { $error } is what went wrong.
core-package-not-compressed = { $path } sıkıştırılamadı: { $error }
# { $path } is a file's path. { $reason } is what went wrong.
core-package-not-read-back = { $path } geri okunamadı: { $reason }

## Where an app's files are, and where an installer may write them. Shown when a path is refused.

# { $path } is a folder, such as C:\Program Files\Acme.
core-discovery-root-not-full-path = altına yazılacak klasör tam bir yol olmalıdır: { $path }
# { $path } is a folder's path.
core-discovery-root-not-a-folder = altına yazılacak klasör bir klasör değil: { $path }
# { $path } is a file's path.
core-discovery-escapes-root = yol, amaçlanan kökün dışına çıkıyor: { $path }
# { $path } is the link's path.
core-discovery-refusing-to-write-through-link = bir bağlantı üzerinden yazılmıyor: { $path }
# { $path } is the link's path.
core-discovery-refusing-link = bir bağlantı üzerinden gidilmiyor: { $path }
# { $error } is what Windows said went wrong.
core-discovery-refusing-link-because = bir bağlantı üzerinden gidilmiyor: { $error }
# { $path } is the link's path.
core-discovery-link-to-somewhere-else = { $path } başka bir yere giden bir bağlantı ve nereye gittiğini başkaları değiştirebilir. Bağlantının gittiği yeri kullanın
# { $path } is a folder's path.
core-discovery-no-parent-folder = { $path } için üst klasör yok
# { $path } is the install location as it was given. Keep the quotes, or use your language's own.
core-discovery-install-location-not-full-path = yükleme konumu tam bir yol olmalıdır: “{ $path }”
# { $path } is the install location.
core-discovery-install-location-a-link = yükleme konumu başka bir yere giden bir bağlantı: { $path }
# { $path } is a file's path.
core-discovery-device-name = yol, bir Windows aygıtı için ayrılmış bir ad kullanıyor: { $path }
# { $path } is a file's path.
core-discovery-device-name-on-older-windows = { $path } yolu, bu Windows sürümünün dosya olarak değil aygıt olarak açtığı bir ad taşıyor, bu nedenle buraya hiçbir şey yazılmadı. Windows 11 üzerine yüklenebilir
# { $path } is a file's path.
core-discovery-unsafe-path = güvenli olmayan yol: { $path }
# { $path } is a file's path.
core-discovery-no-package-path = dosyanın pakete göre yolu yok: { $path }

## Writing an installer, and reading the package inside one. Most of these are shown by an installer, on the PC it
## runs on, when it cannot read what it carries. "The package" is the app's files and settings that an installer
## carries; "the stub" is the program part of an installer, before the package; "the data file" is the .wamdata file
## that holds the package of a very large installer, beside it.

# Stands for the package where a file's name would be, as in "I/O failed for installer package: …".
core-archive-installer-package = yükleyici paketi
# Stands for the installer program built into WinAppMigrator, where a file's name would be.
core-archive-embedded-stub = gömülü yükleyici gövdesi
# { $index } is the number of a file in the package. { $reason } is what went wrong.
core-archive-entry-unreadable = paketin { $index }. girdisi okunamadı: { $reason }
# { $name } is a file's name inside the package, such as app/bin/App.exe.
core-archive-entry-unsafe-path = zip girdisinin yolu güvenli değil: { $name }
# { $name } is a file's name inside the package.
core-archive-holds-more = { $name } belirttiğinden fazlasını içeriyor
# { $name } is a file's name inside the package.
core-archive-name-too-long = bu ad bir zip için fazla uzun: { $name }
core-archive-changed-while-copied = paket, gövdenin arkasına eklenirken değişti, bu nedenle hiçbir yükleyici yazılmadı
core-archive-stub-not-a-program = bu yükleyicinin paketinden önceki kısmı bir program değil
core-archive-damaged = bu yükleyici hasarlı veya oluşturulduktan sonra değiştirilmiş, bu nedenle paketinden hiçbir şey okunmadı. Oluşturulduğu yerden yeniden kopyalayın.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-missing = bu yükleyici paketini aynı klasördeki { $name } adlı bir dosyada tutuyor ve bu dosya orada yok. İki dosya birlikte kalmalıdır: { $name } dosyasını yükleyicinin oluşturulduğu yerden buraya kopyalayın, ardından bunu yeniden çalıştırın.
# { $name } is the data file's name, such as App_installer.wamdata. { $error } is what Windows said went wrong.
core-archive-data-file-unopened = bu yükleyicinin veri dosyası ({ $name }) açılamadı: { $error }. İki dosya, bu bilgisayarın okuyabileceği bir klasörde birlikte kalmalıdır.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-a-link = { $name } bir bağlantı veya klasör; bu yükleyicinin veri dosyası ise hemen yanında, kendi başına bir dosya olmalıdır, bu nedenle ondan hiçbir şey okunmadı. Gerçek dosyayı buraya kopyalayın.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-wrong = { $name } bu yükleyicinin veri dosyası değil ya da hasarlı veya oluşturulduktan sonra değiştirilmiş, bu nedenle ondan hiçbir şey okunmadı. İki dosyayı da oluşturuldukları yerden yeniden kopyalayın.
core-archive-stub-signed = yükleyici gövdesi imzalı; imzasız kalması gerekir, bu nedenle bunun yerine tamamlanmış yükleyiciyi imzalayın
# { $size } is a size, such as 4.2 GB.
core-archive-too-big-to-run = yükleyicinin boyutu { $size } olacaktı ve Windows 4 GB veya daha büyük bir yürütülebilir dosyayı çalıştıramaz; hiçbir şey yazılmadı
core-archive-outside-signature = yükleyicide imzasının dışında veriler var ve kurcalanmış olabilir
core-archive-footer-length = kendiliğinden ayıklanan arşivin alt bilgi uzunluğu geçersiz
core-archive-seek-outside = paketin dışına konumlanma
# { $path } is a file's path.
core-archive-unsafe-package-path = güvenli olmayan paket yolu: { $path }
# { $path } is a file's path.
core-archive-empty-package-path = boş paket yolu: { $path }

## Opening a disk image of another PC (a .vhd or .vhdx file) to read apps from it

# { $path } is the disk image file, such as D:\Backups\OldPC.vhdx.
core-image-not-attachable = { $path }, Windows'un bağlayabileceği bir disk görüntüsü (.vhd veya .vhdx) değil
core-image-needs-administrator = disk görüntüsü bağlamak için yönetici hakları gerekir
# { $path } is the disk image file the person picked.
core-image-not-a-file = { $path } bir dosya değil
# { $path } is the disk image file. "Edit Disk" is the name of a Hyper-V Manager command.
core-image-differencing = { $path } bir fark diski; yalnızca oluşturulduğu diskten bu yana değişenleri tutar ve Windows o diski de, nerede olduğunu belirtiyorsa orada açar. Önce Hyper-V'nin Diski Düzenle komutuyla bu diski o diskle birleştirin ve o diski seçin
# { $path } is the disk image file.
core-image-unreadable = { $path } okunabilen bir .vhd veya .vhdx değil
# ProgramData is the name of a Windows folder; keep it as it is.
core-image-no-program-data = bu bilgisayarın ProgramData klasörü bulunamadı
# The reason given in core-image-could-not-attach when Windows already has the image open.
core-image-already-attached = Windows onu zaten bağlamış ve onu bağlayan şey ona yazıyor olabilir. Önce ayırın veya sürücüsünü bağlı sürücü olarak seçin
# { $path } is the disk image file. { $reason } is why, such as core-image-already-attached or what Windows said.
core-image-could-not-attach = { $path } bağlanamadı: { $reason }
# { $path } is the disk image file.
core-image-no-windows-volume = { $path } dosyasında okunabilen ve Windows içeren bir birim yok. BitLocker ile şifrelenmiş bir birimin önce kilidinin açılması gerekir.
# Given as the reason when Windows failed without saying anything.
core-image-windows-did-not-say = Windows nedenini belirtmedi

## Reading another PC over the network, through the client running on it. Shown as why something on that PC
## could not be read. "The client" is WinAppMigrator Client, the small program on the other PC.

core-remote-too-slow = diğer bilgisayarın yanıt vermesi çok uzun sürdü
core-remote-closed = diğer bilgisayarla bağlantı kapalı
core-remote-failed = diğer bilgisayarla bağlantı başarısız oldu
core-remote-answered-something-else = istemci başka bir yanıt verdi
# { $name } is the PC's name or address as the person typed it, such as OLDPC or 192.168.1.20.
core-remote-not-found = { $name } ağda bulunamadı
core-remote-did-not-say-what-it-is = istemci ne olduğunu belirtmedi
core-remote-ordinary-names-only = yalnızca diğer bilgisayarın kökünün altındaki, sıradan adlardan oluşan bir yol sorulur
core-remote-more-but-none = diğer bilgisayar bir klasörde daha fazla öğe olduğunu söyledi, ancak hiçbirini göndermedi
# { $count } is the most things one folder may hold, such as 250,000.
core-remote-too-many-listed = diğer bilgisayar tek bir klasörde { $count } öğeden fazlasını listeledi
core-remote-sent-too-much = diğer bilgisayar bir dosyadan istenenden fazlasını gönderdi
core-remote-name-not-text = metin olmayan bir ad sorulamaz
core-remote-below-root-only = yalnızca diğer bilgisayarın kökünün altındaki bir yol sorulabilir
core-remote-file-ended-early = diğer bilgisayardaki dosya erken bitti
core-remote-link-not-followed = başka bir bilgisayardaki bağlantı izlenmez
core-remote-only-files-opened = başka bir bilgisayarda yalnızca dosya açılır
core-remote-listing-said-nothing = listede bununla ilgili hiçbir bilgi yoktu
core-remote-change-time-unknown = ne zaman değiştirildiği bilinmiyor
core-remote-seek-before-start = başlangıçtan önceye konumlanma
# { $size } is how many bytes a file is; { $limit } is the most that were expected. Both are numbers.
core-remote-too-large =
    { $size ->
        [one] boyutu { $size } bayt, ancak en fazla { $limit } bayt bekleniyordu
       *[other] boyutu { $size } bayt, ancak en fazla { $limit } bayt bekleniyordu
    }

## Checking that a download is signed by Microsoft. Each is given as the reason a check failed.

# { $error } is what Windows said went wrong.
core-signature-no-powershell = PowerShell başlatılamadı: { $error }
# { $error } is what went wrong reading PowerShell's answer.
core-signature-unreadable = imza okunamadı: { $error }
# { $signer } is who signed the file, such as Contoso Ltd.
core-signature-not-microsoft = Microsoft tarafından değil, { $signer } tarafından imzalanmış
core-signature-not-microsoft-unnamed = Microsoft tarafından değil, başka biri tarafından imzalanmış
# { $status } is Windows' own word for the signature's state, such as HashMismatch or NotSigned, which is not translated.
core-signature-status = imza durumu: { $status }

## Warnings a package records when it is built from an app, which an installer shows later, maybe on another PC, in
## that PC's language. "This app" is the app being packaged.

core-warning-build-program-needs-unreadable = Bunun Windows'tan neye ihtiyaç duyduğu okunamadı, bu nedenle diğer bilgisayarda çalışıp çalışamayacağı denetlenmeyecek.
core-warning-build-external-manifest = Harici uygulama bildirimi algılandı; yan yana bağımlılıkların yorumlanması ertelendi.
core-warning-build-non-exe = Seçilen dosya .exe uzantısını kullanmıyor; PE taraması devam edecek.
core-warning-build-stand-in-program = Sıradan bir uygulama yürütülebilir dosyası bulunamadı, bu nedenle onun yerine bu dosya kullanılıyor. Mimari ve bağımlılık sonuçları bu dosyayı açıklar ve uygulamayı açıklamayabilir.
core-warning-build-other-apps-files-unknown = Windows Installer'ın diğer uygulamalarla ilgili bilgilerinin tümü okunamadı; bu nedenle bu uygulamaya ait kayıtlı bir klasör, burada bunu belirten hiçbir şey olmadan başka bir uygulamanın dosyalarını içerebilir.
core-warning-build-app-folder-has-another-apps-file = Windows Installer'a göre, bu uygulamanın programının bulunduğu klasörde başka bir uygulamanın bir dosyası var; klasörün tamamı alındığı için bu dosya da gelir.
# "App Paths" is the name of a Windows registry key; keep it as it is.
core-warning-build-app-paths-folder-left-out = Bu uygulamanın App Paths girdisi, kendisine ait olmayan bir klasöre baktığını belirtiyor, bu nedenle o klasör gelmez. Uygulama, oradakiler olmayan bir bilgisayarda başlamayabilir.
core-warning-build-uninstaller-folder-left-out = Windows Installer'a göre başka bir uygulamanın orada bir dosyası olduğundan, bu uygulamanın kaldırma programının bulunduğu klasör dışarıda bırakıldı.
# { $why } is never shown: it chooses the reason. Translate each reason as the end of the sentence.
core-warning-build-recorded-folder-left-out =
    Bu uygulama için kayıtlı bir öğe, dışarıda bırakılan bir klasörü adlandırıyor. Klasör şu nedenle dışarıda bırakıldı: { $why ->
        [unreadable] yolu sürücünün dışına çıkıyor veya bir bağlantı üzerinden gidiyor, bu nedenle güvenle okunamaz
        [missing] orada değil
        [link] bir bağlantı üzerinden gidiyor
        [short-name] gerçek adıyla yazılmamış
        [another-account] başka bir hesabın klasörlerinde
        [one-account] bu uygulama tek bir hesap için yüklenmiş ve bu klasör o hesaba ait değil
        [shared] paylaşılan öğeleri içeriyor veya Windows'un bir parçası
        [generic-name] adı, kime ait olduğu hakkında hiçbir şey söylemiyor
        [publisher] yalnızca yayımcının adını taşıyor ve yayımcının diğer ürünlerini de içerebilir
        [holds-the-app] bu uygulamanın kendi klasörünü içeriyor ve daha fazlasını da içerebilir
        [another-apps-name] başka bir uygulamanın adını taşıyor
        [another-apps-record] yükleme kaydı başka bir uygulamaya ait
        [in-another-apps-folder] başka bir uygulamanın klasöründe
       *[another-apps] başka bir uygulamaya ait
    }.
core-warning-build-app-folder-holds-another-app = Bu bilgisayardaki başka bir uygulama, bu uygulamanın klasörünün içinde kayıtlı; klasörün tamamı alındığı için o uygulama da birlikte gelir.
# "Program Files (x86)" is the name of a Windows folder; keep it as Windows calls it.
core-warning-build-from-32-bit-windows = Bu uygulama 32 bit Windows üzerindeydi. Kayıt defteri anahtarları, 64 bit Windows'un 32 bit uygulamaların anahtarlarını tuttuğu düzende yerleştirilir ve yükleyici bunun için Program Files (x86) klasörünü önerir.
# { $dll } is a file's name, such as MSVCP140D.dll.
core-warning-build-debug-runtime = Bir program, Visual C++ çalışma zamanının hata ayıklama derlemesi olan { $dll } dosyasına ihtiyaç duyuyor. Microsoft'un bunlar için bir yükleyicisi yoktur ve bunların dağıtılmasına izin vermeyebilir; bu nedenle program, Visual Studio olmayan bir bilgisayarda çalışmaz.
# { $program } is a program's file name, such as Tool.exe.
core-warning-build-single-file-program = { $program }, tek bir dosyaya paketlenmiş bir .NET programı ve hangi .NET sürümüne ihtiyaç duyduğunu belirtmiyor, bu nedenle onun için hiçbir sürüm planlanmadı. Bir sürüme ihtiyacı varsa, o sürüm olmayan bir bilgisayarda başlamaz.
# { $count } is how many of the app's programs.
core-warning-build-programs-not-inspected =
    { $count ->
        [one] Uygulamanın çok fazla programı olduğundan veya programlar çok büyük olduğundan, programlardan { $count } tanesi ihtiyaç duyduğu çalışma zamanları açısından incelenmedi.
       *[other] Uygulamanın çok fazla programı olduğundan veya programlar çok büyük olduğundan, programlardan { $count } tanesi ihtiyaç duydukları çalışma zamanları açısından incelenmedi.
    }
# { $runtime } is a runtime's name, such as Java Runtime Environment.
core-warning-build-runtime-no-installer = Uygulamanın { $runtime } çalışma zamanına ihtiyacı var ve burada bunun için bir yükleyici olmadığından hiçbir yükleyici çalıştırılmayacak. Bunu diğer bilgisayara kendiniz yükleyin.
# { $why } is why something was left out, such as "its name says what it holds".
core-warning-build-left-out-of-package = Paketin dışında bırakıldı: { $why }.
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-missing-dependency = Bağımlılık çözümlenemedi: { $name }
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-dependency-only-here = { $name } yalnızca bu bilgisayarın arama yolunda bulundu ve uygulamanın bir parçası değil, bu nedenle gelmez. Uygulama, bu dosya olmayan bir bilgisayarda başlamayabilir.
core-warning-build-driver-detected = Sürücü veya çekirdek bileşeni algılandı. Uygulamaya ait bir sürücü pakete eklenir ve yeniden kaydedilir; üreticiye ait veya Windows ile gelen bir sürücü ise dağıtılmaz ve üreticinin imzalı paketinden yeniden yüklenmelidir.
core-warning-build-near-no-longer-offered = Bu uygulamanın parçası olabilecekler listesinde işaretlenen bir öğe artık o listede değil, bu nedenle eklenmedi. Listeye yeniden bakın.
# { $app_name } is the name of a part of the app, as the list of apps calls it. { $why } is never shown: it chooses the
# reason. Translate each reason as the end of the sentence.
core-warning-build-near-part-left-out =
    { $app_name } uygulamasının kendisine ait olduğunu söylediği şeylerin bir kısmı dışarıda bırakıldı: { $why ->
        [unreadable] yolu sürücünün dışına çıkıyor veya bir bağlantı üzerinden gidiyor, bu nedenle güvenle okunamaz
        [missing] orada değil
        [link] bir bağlantı üzerinden gidiyor
        [short-name] gerçek adıyla yazılmamış
        [another-account] başka bir hesabın klasörlerinde
        [one-account] bu uygulama tek bir hesap için yüklenmiş ve bu kısım o hesaba ait değil
        [shared] paylaşılan öğeleri içeriyor veya Windows'un bir parçası
        [generic-name] adı, kime ait olduğu hakkında hiçbir şey söylemiyor
        [publisher] yalnızca yayımcının adını taşıyor ve yayımcının diğer ürünlerini de içerebilir
        [holds-the-app] bu uygulamanın kendi klasörünü içeriyor ve daha fazlasını da içerebilir
        [another-apps-name] başka bir uygulamanın adını taşıyor
        [another-apps-record] yükleme kaydı başka bir uygulamaya ait
        [in-another-apps-folder] başka bir uygulamanın klasöründe
       *[another-apps] başka bir uygulamaya ait
    }.
# { $app_name } is the name of a part of the app, as the list of apps calls it.
core-warning-build-near-keys-left-out = { $app_name } uygulamasının yükleme kaydında listelenen kayıt defteri anahtarları dışarıda bırakıldı, çünkü bu kayıt başka bir uygulamaya ait olanları da adlandırıyor.
# "Windows keeps what an app wrote for one account" is Windows' VirtualStore folder.
core-warning-build-virtual-store-program = Windows'un bir uygulamanın tek bir hesap için yazdıklarını tuttuğu klasörde bir program vardı. Windows oraya hiçbir zaman program koymaz; bu nedenle onu başka bir şey koydu ve program dışarıda bırakıldı.
# { $path } is a file or folder, such as C:\Users\Sam\AppData\Roaming\Acme.
core-warning-build-file-left-out = { $path } dışarıda bırakıldı: Bağlı bir sürücüdeki bir bağlantı üzerinden gidiyor veya verileri eklenen hesaptan başka bir hesaba ait.
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as D:\Sam\Documents.
core-warning-build-user-folder-not-on-drive = { $user } hesabının { $folder } klasörü { $path } konumundaydı; bu konum bu sürücüde olmadığından oradakiler eklenmedi.
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as \\server\home\Sam\Documents.
core-warning-build-user-folder-not-on-this-pc = { $user } hesabının { $folder } klasörü { $path } konumundaydı; bu konum bu bilgisayarın bir diskinde olmadığından oradakiler eklenmedi.
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-base-unreadable = { $path } okunamadı ({ $error }); orada depolanan uygulama verileri eklenmedi.
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-folder-unreadable = { $path } okunamadı ({ $error }); oradaki iç içe uygulama verileri eksik olabilir.
# { $error } is what went wrong.
core-warning-build-entry-unreadable = Bir dosya sistemi girdisi okunamadı: { $error }
# { $count } is how many links. { $names } lists some of them. { $more } is how many more there are, which are not
# named; when it is 0, the sentence ends after the list.
core-warning-build-links-not-followed =
    { $count ->
        [one] Bu klasörde { $count } bağlantı veya başka bir şeyi açacak bir ad izlenmedi, bu nedenle işaret ettiği şey yükleyicide yok: { $names }
       *[other] Bu klasörde { $count } bağlantı veya başka bir şeyi açacak ad izlenmedi, bu nedenle işaret ettikleri şeyler yükleyicide yok: { $names }
    }{ $more ->
        [0] .
       *[other] { " " }ve { $more } tane daha.
    }
# { $count } is how many folders. { $depth } is how many levels down a folder can be. { $names } lists some of them.
# { $more } is how many more there are, which are not named; when it is 0, the sentence ends after the list.
core-warning-build-folders-too-deep =
    { $count ->
        [one] { $count } klasör { $depth } düzeyden daha derinde, bu nedenle içindekiler yükleyicide yok: { $names }
       *[other] { $count } klasör { $depth } düzeyden daha derinde, bu nedenle içlerindekiler yükleyicide yok: { $names }
    }{ $more ->
        [0] .
       *[other] { " " }ve { $more } tane daha.
    }
# { $error } is what went wrong.
core-warning-build-metadata-unreadable = Dosya meta verileri okunamadı: { $error }
core-warning-build-reserved-name = Bu dosyanın adı bir Windows aygıtı için ayrılmış, bu nedenle kopyalanmadı.
# { $error } is what went wrong.
core-warning-build-hash-failed = Dosyanın karması hesaplanamadı: { $error }
core-warning-build-cloud-placeholder = Yalnızca çevrimiçi bulut dosyasının (ör. OneDrive İsteğe Bağlı Dosyalar) yerel verisi yok; yedeklemeden önce sabitleyin veya indirin. Bildirildi ancak kopyalanmadı.
# { $file_name } is a driver's file name, such as acmeflt.sys.
core-warning-build-driver-file = “{ $file_name }” sürücü/çekirdek dosyası düz bir dosya olarak kopyalanmaz. Bu uygulama için kayıtlı bir sürücü hizmetinin dosyasıysa yükleyici onu pakete ekler ve sürücüyü otomatik olarak yeniden kaydeder; değilse üreticinin imzalı sürücü paketiyle yeniden yükleyin.
core-warning-build-file-gone = Bu, uygulama incelendiğinde oradaydı ve artık yok, bu nedenle yükleyicide yer almıyor.
# { $error } is what went wrong.
core-warning-build-file-copy-failed = Dosya pakete kopyalanamadı: { $error }
# { $count } is how many files. { $names } lists them.
core-warning-build-not-looked-through =
    { $count ->
        [one] Bu pakette, kimden geldiği açısından incelenemeyen bir dosya var: { $names }. İçeriği bunu belirtiyor olabilir.
       *[other] Bu pakette, kimden geldiği açısından incelenemeyen dosyalar var: { $names }. İçerikleri bunu belirtiyor olabilir.
    }
# { $count } is the most empty folders an installer makes, such as 10,000.
core-warning-build-empty-folders-truncated = Bu uygulamada { $count } taneden fazla boş klasör var ve yalnızca bu kadarı yeniden oluşturulacak.
core-warning-build-credential-by-now = Paketin dışında bırakıldı: Artık bir kimlik bilgisine benziyor.
core-warning-build-no-package-path = Kopyalanabilir dosyanın pakete göre yolu yoktu.
# { $file_name } is the runtime installer's file name, such as vc_redist.x64.exe. { $error } is what went wrong.
core-warning-build-prerequisite-path-invalid = Paketteki { $file_name } önkoşulu yerleştirilemedi: { $error }
# { $runtime } is a runtime's name, such as Microsoft Visual C++ 2015+ Redistributable (x64). { $error } is what went
# wrong. winget is the name of a Windows command; keep it as it is.
core-warning-build-prerequisite-download-failed = { $runtime } indirilemedi ({ $error }); yükleyici hedef bilgisayarda winget ile yüklemeyi deneyecek.
# { $runtime } is a runtime's name. { $reason } is why the check failed, such as "it is signed by Contoso Ltd, not
# Microsoft".
core-warning-build-prerequisite-signature-failed = İndirilen { $runtime } imza denetiminden geçemedi, bu nedenle pakete eklenmedi: { $reason }. Yükleyici hedef bilgisayarda winget ile yüklemeyi deneyecek.
# { $runtime } is a runtime's name. { $error } is what went wrong.
core-warning-build-prerequisite-hash-failed = { $runtime } indirildi, ancak karması hesaplanamadı ({ $error }); yükleyici bunun yerine hedef bilgisayarda winget ile yüklemeyi deneyecek.
# { $service } is a driver's service name, such as AcmeFlt. { $error } is what went wrong.
core-warning-build-driver-copy-failed = “{ $service }” için sürücü görüntüsü pakete eklenemedi: { $error }
core-warning-build-license-not-copied = Lisans verileri kopyalanamadı (eksik veya okunamıyor); yalnızca meta veri olarak kaydedildi.
core-warning-build-license-file-credential = Bir lisans dosyası kimlik bilgisine benziyor ve dışarıda bırakıldı.
# { $error } is what went wrong.
core-warning-build-license-store-walk-failed = Bir lisans deposunun bir kısmı listelenemedi: { $error }
core-warning-build-license-store-credential = Bir lisans deposundaki bir dosya kimlik bilgisine benziyor ve dışarıda bırakıldı.
# { $error } is what went wrong.
core-warning-build-license-store-file-skipped = Bir lisans deposu dosyası atlandı: { $error }
