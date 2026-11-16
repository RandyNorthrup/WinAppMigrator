# Words of the installer that every built installer carries, which runs on the PC an app is moved to and speaks its
# language, and of the uninstaller it leaves behind for Settings. Message names start with installer- for the
# installer's window, installer-cli- for its command line, and uninstaller- for the uninstaller.
#
# The command lines (installer-cli-, uninstaller-cli-) are always shown in English, as scripts read what they print.
# Translate them all the same. Keep their command-line flags, such as --apply and --target-root, exactly as they are.


## The installer's window: its title and header

installer-window-title = WinAppMigrator Yükleyicisi
# The window's title in the taskbar. { $app_name } is the name of the app it installs.
installer-window-title-app = { $app_name } uygulamasını yükle
installer-tagline = Başka bir bilgisayardan alınan bir uygulamayı yeniden yükler
# What an app is called when its package gives it no name.
installer-unnamed-app = Uygulama

## When the installer's window cannot open. Each is a paragraph of a message box.

# { $error } is what went wrong, as Windows or the graphics driver says it.
installer-window-failed = Yükleyicinin penceresi açılamadı: { $error }
installer-window-failed-without-one = Pencere olmadan da yükleme yapabilir: Bu dosyayı yönetici komut isteminden --apply ile çalıştırın. Nereye yükleneceğini seçmek için --target-root ve ardından bir klasör ekleyin.

## Checking the installer when it opens

installer-checking = Yükleyici denetleniyor…
installer-stub-no-package = Bu bir WinAppMigrator yükleyici gövdesidir ve uygulama paketi taşımaz. WinAppMigrator ile bir yükleyici oluşturun, ardından oluşturulan .exe dosyasını burada çalıştırın.
installer-check-stopped = Yükleyicinin denetimi durdu.
# { $error } is what went wrong, as Windows says it.
installer-locate-failed = Yükleyici bulunamadı: { $error }
# { $error } is what went wrong reading it.
installer-read-failed = Yükleyici okunamadı: { $error }
# { $error } is why, such as that the app needs a newer Windows.
installer-cannot-be-used-here = Bu yükleyicinin paketi bu bilgisayarda kullanılamaz: { $error }

## The app, where it goes, and whether the installer is signed

installer-captured-from = Alındığı bilgisayar
installer-install-location = Yükleme konumu
# The title of the folder picker.
installer-choose-location = Yükleme konumunu seçin
installer-folder-has-files = Bu klasörde zaten dosyalar var. Aynı adlı dosyaların üzerine yazılır; uygulamayı kaldırırsanız eski dosyalar geri gelir.
installer-room-needed = Gereken alan
# { $shortfall } says which drive is short of room, such as "C: needs up to 824.0 MB for the app's files, and has 300.0 MB free".
installer-room-short = Burada yeterli alan olmayabilir: { $shortfall }. Paket başka bir sürücüde açılabilir, ancak uygulamanın dosyalarının yine de sığması gerekir.
# { $app_name } is the name of an app whose earlier install was cut short.
installer-interrupted = { $app_name } uygulamasının daha önceki bir yüklemesi hiç tamamlanmadı. Yükle seçildiğinde önce o yükleme geri alınır.
# Beside "Installer", for whether it proves who made it.
installer-signature = Yükleyici
installer-signature-checking = İmzası denetleniyor…
installer-signature-unsigned = İmzalanmamış: yayımcı bilinmiyor
# { $publisher } is the name on the installer's certificate, such as "Fabrikam, Inc.".
installer-signature-signed-by = İmzalayan: { $publisher }
installer-signature-unnamed = Adı belirtilmemiş bir yayımcı tarafından imzalanmış
# { $reason } is Windows' own word for what is wrong, such as HashMismatch.
installer-signature-invalid = Bu yükleyicinin imzası geçerli değil ({ $reason }). İmzalandıktan sonra değiştirilmiş olabilir.

## Why the install location cannot be used. Install and Preview Changes stay off while one of these is shown.

# { $app_name } is the name of the app being installed.
installer-location-app-already-there = { $app_name } bu klasörde zaten yüklü, bu nedenle bu kopya buraya yüklenemez. Buraya yüklemek için önce bu bilgisayardaki kopyayı Windows Ayarları'nda Uygulamalar bölümünden kaldırın, ardından bu yükleyiciyi yeniden açın. Ya da yukarıya yeni ve boş bir klasör girin.
# { $other_app } is the name of another app Windows lists as installed in the folder. { $app_name } is the name of the app being installed.
installer-location-other-app-there = { $other_app } bu klasörde yüklü, bu nedenle { $app_name } buraya yüklenemez. Yukarıya yeni ve boş bir klasör girin. Ya da { $other_app } artık gerekmiyorsa önce onu Windows Ayarları'nda Uygulamalar bölümünden kaldırın, ardından bu yükleyiciyi yeniden açın.
installer-location-has-files = Bu klasörde WinAppMigrator tarafından yüklenmemiş dosyalar zaten var. Bunlar başka bir uygulamaya ait olabileceğinden üzerlerine hiçbir şey yüklenmez. Yukarıya yeni ve boş bir klasör girin.
# { $reason } is why, such as that it cannot be the Windows folder.
installer-location-refused = Bu klasör kullanılamaz: { $reason }. Yukarıya başka bir klasör girin.

## What the install sets up beyond its files

installer-beyond-files = Dosyalar dışında ayarlananlar
# Under the heading above. It is part of what a person agrees to when they click Install.
installer-beyond-files-agreed = Bunların tümü yönetici haklarıyla yapılır. Yükleme başarısız olursa değiştirdikleri eski haline getirilir ve eski haline getirilemeyen her şey size bildirilir. Yüklemeyi daha sonra Ayarlar'da Yüklü uygulamalar bölümünden de geri alabilirsiniz. Üzerine yazdığı dosyalar geri gelir; yüklediği çalışma zamanları kalır.
installer-beyond-files-none = Hiçbir şey: Yalnızca dosyaları ve uygulamanın kendi kayıt defteri anahtarlarındaki ayarları kopyalar.
# { $problem } is why, such as that the app cannot run on this PC.
installer-cannot-go-here = Bu paket bu konuma yüklenemez: { $problem }
installer-working-out = Bunun bu konumda neleri ayarlayacağı belirleniyor…
# Said as the reason after "This package cannot go to this location:".
installer-list-stopped = yapacaklarının belirlenmesi durdu
# A heading over a list of what else the package asked for and will not get. { $count } is how many.
installer-more-left-out =
    { $count ->
        [one] { $count } öğe daha dışarıda bırakıldı
       *[other] { $count } öğe daha dışarıda bırakıldı
    }

## Options

installer-options = Seçenekler
installer-user-data = Kullanıcı verileri
installer-user-data-detail = Açık: Uygulama ayarları, Belgeler ve Kayıtlı Oyunlar, oturum açmış kişinin olağan klasörlerine gider. Kapalı: Tümü yükleme klasörünün altına gider ve oradan çalışacak hiçbir şey ayarlanmaz.
# { $account } is the name of the account on the other PC whose data this is.
installer-user-data-whose = Bu, { $account } hesabının verilerini içerir.
installer-license = Lisans
installer-license-detail = Etkinleştirme ve lisans dosyaları ile lisans bilgilerini tutan kayıt defteri değerleri. Bazılarının burada yeniden etkinleştirilmesi gerekebilir.

## Dependencies: what else the app needs, and how each is provided

installer-dependencies = Bağımlılıklar
installer-dependencies-detail = Bu uygulamanın gereksinimleri ve her birinin nasıl sağlandığı.
installer-dependencies-none = Ek çalışma zamanı bağımlılığı algılanmadı.
# { $feature } is the name of an optional part of Windows, such as NetFx3.
installer-dependency-windows-feature = Windows özelliği: { $feature }
installer-dependency-included = Dahil
# { $id } is the package's name in winget, such as Microsoft.VCRedist.2015+.x64.
installer-dependency-winget = winget ile yüklenir: { $id }
installer-dependency-manual = El ile yükleyin
installer-dependency-built-in = Windows'ta yerleşik
installer-dependency-ships-with-app = Uygulamayla birlikte gelir

## Buttons, and what is said beside them

installer-install = Yükle
installer-preview = Değişiklikleri önizle
installer-cancel = İptal
installer-close = Kapat
installer-open-folder = Yükleme klasörünü aç
# { $error } is what went wrong, as Windows says it.
installer-open-folder-failed = Klasör açılamadı: { $error }
installer-location-blocks = Yükleme konumu kullanılabilir olana kadar Yükle ve Değişiklikleri önizle kapalıdır. Nedeni yukarıda, Yükleme konumunun altında yazıyor.
installer-package-blocks = Bu paket bu konuma yüklenemediği için Yükle kapalıdır. Nedeni yukarıda yazıyor.
installer-list-changed = Bunun burada yapacakları değişti, bu nedenle hiçbir şey yüklenmedi. Aşağıdaki listeyi gözden geçirin ve istediğiniz buysa Yükle düğmesini yeniden seçin.
installer-cancelled-early = İptal edildi. Hiçbir şey değiştirilmedi.

## How far an install has got, beside its progress bar

# { $step } is one of the steps below. { $done } and { $total } are how many of its items are done, and how many there are.
installer-step-counted = { $step }: { $done } / { $total }…
# { $step } is one of the steps below, when it does not count its items.
installer-step-uncounted = { $step }…
installer-step-getting-ready = Hazırlanıyor…
installer-step-cancelling = Geçerli adımdan sonra iptal ediliyor…
installer-step-putting-back = Değiştirilenler eski haline getiriliyor
installer-step-unpacking = Paket açılıyor
installer-step-checking-package = Paket denetleniyor
installer-step-recovering = Tamamlanmamış önceki bir yükleme geri alınıyor
# What a preview is doing, whatever step it is on.
installer-step-previewing = Nelerin değişeceği belirleniyor
installer-step-runtimes = Çalışma zamanları yükleniyor
installer-step-files = Dosyalar kopyalanıyor
installer-step-services = Hizmetler ayarlanıyor
installer-step-drivers = Sürücüler ayarlanıyor
installer-step-registry = Kayıt defteri ayarları yazılıyor
installer-step-license = Lisans verileri geri yükleniyor
installer-step-tasks = Zamanlanmış görevler ayarlanıyor
installer-step-shortcuts = Kısayollar oluşturuluyor
installer-step-environment = Ortam değişkenleri ayarlanıyor
installer-step-settings-entry = Uygulama Ayarlar'a ekleniyor
installer-step-verifying = Yüklenenler denetleniyor
installer-step-undoing = Geri alınıyor

## When an install cannot go ahead or stops

installer-carries-no-package = Bu yükleyici paket taşımıyor.
installer-stopped-unexpectedly = Yükleme beklenmedik şekilde durdu.
# { $reason } is what went wrong, as the program says it.
installer-stopped-unexpectedly-because = Yükleme beklenmedik şekilde durdu: { $reason }. Yükleme başlamışsa, bir yükleyiciyi yeniden çalıştırdığınızda önce onun yaptıkları geri alınır.
# { $error } is what went wrong reading it.
installer-package-read-failed = Paket okunamadı: { $error }
installer-no-programdata = Bu bilgisayarın ProgramData klasörü bulunamadı, bu nedenle hiçbir şey yüklenmedi: Bir yükleme, geri alınmasını sağlayan kaydı orada tutar.
# { $error } is why, such as that there is nowhere only administrators can get into.
installer-staging-failed = Hazırlık klasörü oluşturulamadı: { $error }
# { $error } is what went wrong unpacking it.
installer-unpack-failed = Paket açılamadı: { $error }
installer-staging-nowhere = paketin açılabileceği, yalnızca yöneticilerin erişebildiği bir yer yok
# { $failure } is a folder and what went wrong there, such as "C:\ProgramData\WinAppMigrator: Access is denied.".
installer-staging-nowhere-look-at = paketin açılabileceği, yalnızca yöneticilerin erişebildiği bir yer yok. Bakılması gereken yer: { $failure }

## What came of an install, or of a preview

installer-result = Yükleme sonucu
installer-result-preview = Önizleme: Henüz hiçbir şey değiştirilmedi
installer-result-installed = Uygulama yüklendi ve karma denetimini geçti.
installer-result-installed-without-left-out = Uygulama, aşağıda dışarıda bırakıldığı belirtilenler olmadan yüklendi ve karma denetimini geçti.
installer-result-not-all-put-back = Yükleme başarısız oldu ve değiştirdiklerinin tümü eski haline getirilemedi. Kalanlar aşağıda listeleniyor; bir sonraki yükleme bunları yeniden dener.
installer-result-cancelled-put-back = Yükleme iptal edildi ve değiştirdikleri eski haline getirildi.
installer-result-failed-put-back = Yükleme başarısız oldu ve değiştirdikleri eski haline getirildi.
installer-result-not-clean = Yükleme sorunsuz tamamlanmadı. Aşağıdaki sorunları gözden geçirin.
installer-result-changes-put-back = Eski haline getirilen değişiklikler
installer-result-changes-made = Yapılan değişiklikler
installer-result-changes-planned = Planlanan değişiklikler
installer-result-warnings = Uyarılar
installer-result-errors = Hatalar
installer-result-package-valid = Paket karmaları geçerli
installer-result-package-invalid = Paket karmaları GEÇERSİZ
installer-result-files-valid = Yüklenen dosyalar geçerli
installer-result-files-invalid = Yüklenen dosyalar GEÇERSİZ
installer-result-reboot = Bir bağımlılığın tamamlanması için yeniden başlatma gerekiyor. Windows'u yeniden başlatın, ardından uygulamayı açın.
# A heading over the list of what was changed. { $count } is how many.
installer-result-changes = Değişiklikler ({ $count })
# After the first part of a long list. { $count } is how many more there are.
installer-result-changes-more = ve { $count } tane daha.
# Over the path of the install's log file.
installer-log = Günlük

## The installer's command line. Shown in English: scripts read it. A { $count } is plain digits, with no separators.

installer-cli-stub = WinAppMigrator yükleyici gövdesi — uygulama paketi taşımıyor.
installer-cli-stub-how = WinAppMigrator uygulamasıyla bir yükleyici oluşturun, ardından oluşturulan .exe dosyasını hedef bilgisayarda çalıştırın.
installer-cli-usage-title = WinAppMigrator kendiliğinden ayıklanan yükleyicisi.
installer-cli-usage = Grafik yükleyiciyi açmak için bağımsız değişken vermeden çalıştırın veya:
installer-cli-usage-preview = yeniden yüklemeyi önizle: --dry-run [--target-root <KLASÖR>]
installer-cli-usage-apply = yeniden yükle:            --apply [--target-root <KLASÖR>]
installer-cli-no-package = yükleyici gömülü bir paket içermiyor
installer-cli-no-programdata = bu bilgisayarın ProgramData klasörü bulunamadı, bu nedenle hiçbir şey yüklenmedi: Bir yükleme, geri alınmasını sağlayan kaydı orada tutar
# { $path } is the folder asked for, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-folder-has-files = { $path } kullanılamaz: { $problem }. --force seçeneğini yalnızca klasörün bu uygulamaya ait olduğunu biliyorsanız verin
installer-cli-no-location = bu paket, uygulamasının nerede olduğunu belirtmiyor; bu nedenle bir --target-root verin
# { $path } is where the app was on the other PC, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-location-refused = bu paketin uygulaması { $path } konumundaydı ve orası kullanılamaz: { $problem }. Bir --target-root verin
# { $summary } is how much room it needs where, such as "up to 824.0 MB on C: (356.0 GB free)".
installer-cli-room = Alan: { $summary }
# { $note } is something to know about the install.
installer-cli-note = Not: { $note }
# { $count } is how many things it sets up beyond its files.
installer-cli-sets-up = Ayarlananlar: { $count }
installer-cli-applied = Yeniden yükleme uygulandı.
installer-cli-previewed = Yeniden yükleme önizlendi.
# { $count } is how many changes the install logged.
installer-cli-changes-logged = Günlüğe kaydedilen değişiklikler: { $count }
# { $count } is how many warnings.
installer-cli-warnings = Uyarılar: { $count }
# { $count } is how many errors.
installer-cli-errors = Hatalar: { $count }
# { $count } is how many things the package asked for and did not get.
installer-cli-refused = Reddedilenler: { $count }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-package-valid = Paket karmaları geçerli: { $valid }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-reinstall-valid = Yeniden yükleme karmaları geçerli: { $valid }
# { $id } is the install's id, such as wam-0f3a….
installer-cli-install-id = Yükleme kimliği: { $id }
installer-cli-put-back = Yükleme başarısız oldu, bu nedenle değiştirdikleri eski haline getirildi.
# { $path } is the log file.
installer-cli-log = Günlük: { $path }
# { $path } is the log file. Printed before an error.
installer-cli-error-log = günlük: { $path }
# { $error } is what went wrong.
installer-cli-error = hata: { $error }

## The uninstaller's window

uninstaller-window-title = Kaldır
# The window's title, and its heading. { $app_name } is the name of the app it uninstalls.
uninstaller-window-title-app = { $app_name } uygulamasını kaldır
uninstaller-tagline = Yükleyicisinin yaptıklarını geri alır
uninstaller-reading = Bu yüklemenin kaydı okunuyor…
uninstaller-stopped-unexpectedly = Kaldırma işlemi beklenmedik şekilde durdu.
uninstaller-what-will-be-removed = Kaldırılacaklar
uninstaller-install-location = Yükleme konumu
uninstaller-files = Dosyalar
uninstaller-registry-values = Kayıt defteri değerleri
uninstaller-services = Hizmetler
uninstaller-drivers = Sürücüler
uninstaller-scheduled-tasks = Zamanlanmış görevler
uninstaller-environment = Ortam
uninstaller-plan-stays = Eklediğiniz veya değiştirdiğiniz dosyalar, bulundukları klasörlerle birlikte kalır.
# { $count } is how many files the install replaced.
uninstaller-plan-coming-back =
    { $count ->
        [one] Bu yüklemenin üzerine yazdığı { $count } dosya geri gelir ya da o zamandan beri değişmiş olan dosyanın yanına konur.
       *[other] Bu yüklemenin üzerine yazdığı { $count } dosya geri gelir ya da o zamandan beri değişmiş olan dosyaların yanına konur.
    }
# { $runtimes } is a list of runtimes, such as "Microsoft Visual C++ Redistributable, .NET Desktop Runtime 8".
uninstaller-plan-runtimes = Yüklediği çalışma zamanları, başka programlar da kullanıyor olabileceğinden kalır: { $runtimes }.
uninstaller-options = Seçenekler
uninstaller-remove-user-data = Kullanıcı verilerini ve lisans verilerini de kaldır
# { $count } is how many items of the app's settings, data, and license there are.
uninstaller-remove-user-data-detail =
    { $count ->
        [one] Ayarlarına, verilerine ve lisansına ait { $count } öğe. Bu seçenek açık olmadıkça kalır. Üzerine yazdığı şey geri gelir; öğe o zamandan beri değiştiyse onun yanına konur.
       *[other] Ayarlarına, verilerine ve lisansına ait { $count } öğe. Bu seçenek açık olmadıkça kalırlar. Üzerine yazdıkları şeyler geri gelir; öğeler o zamandan beri değiştiyse onların yanına konur.
    }
uninstaller-uninstall = Kaldır
uninstaller-cancel = İptal
uninstaller-close = Kapat
# { $done } and { $total } are how many of the install's items are undone, and how many there are.
uninstaller-step-counted = Yükleme geri alınıyor: { $done } / { $total }…
uninstaller-step = Yükleme geri alınıyor…
uninstaller-done = Uygulama kaldırıldı.
uninstaller-done-except = Uygulama, aşağıda listelenenler dışında kaldırıldı.
uninstaller-waits-for-drive = Bir kısmı bağlı olmayan bir sürücüde. Sürücüyü bağlayın ve Ayarlar'dan yeniden kaldırın.
uninstaller-not-all-undone = Tümü geri alınamadı. Onu kullanan programları kapatın ve Ayarlar'dan yeniden kaldırın.
# Over the path of the uninstall's log file.
uninstaller-log = Günlük
uninstaller-undone = Geri alındı
uninstaller-left-in-place = Yerinde bırakıldı
uninstaller-could-not-be-undone = Geri alınamadı
# After the first part of a long list. { $count } is how many more there are.
uninstaller-and-more = ve { $count } tane daha.
# Said as the reason when there is no record of the install to read.
uninstaller-no-programdata = bu bilgisayarın ProgramData klasörü bulunamadı, bu nedenle okunacak kayıt yok

## When the uninstaller's window cannot open. Each is a paragraph of a message box, followed by a command to type.

uninstaller-window-failed-title = WinAppMigrator Kaldırma Programı
# { $error } is what went wrong, as Windows or the graphics driver says it.
uninstaller-window-failed = Kaldırma programının penceresi açılamadı: { $error }
uninstaller-window-failed-quiet = Pencere olmadan da yönetici komut isteminden kaldırma yapabilir:
# Stands in for this program's own path in that command, if it cannot be found.
uninstaller-this-program = bu program

## The uninstaller's command line (--quiet). Shown in English: scripts read it.

# { $app_name } is the name of the app.
uninstaller-cli-uninstalling = { $app_name } uygulaması kaldırılıyor.
# { $path } is the log file.
uninstaller-cli-log = Günlük: { $path }
# { $error } is what went wrong.
uninstaller-cli-error = hata: { $error }
