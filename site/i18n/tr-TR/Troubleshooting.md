<a id="troubleshooting"></a>

# Sorun giderme

Gördüğünüz şeyi aşağıda, ne olduğuna veya ekrandaki sözcüklere göre bulun. Tırnak içindeki sözcükler programın gösterdikleridir; bazen ortalarında bir ad veya sayı bulunur.

Burada hiçbir şey yardımcı olmazsa sondaki [Yardım alma](#getting-help) bölümüne bakın.

<a id="the-window-could-not-open"></a>

## Pencere açılamadı

**Gördüğünüz.** Pencere yerine bir ileti kutusu:

- uygulamadan: "Pencere açılamadı" ve "Bu programın penceresini çizmek için Direct3D 12 gerekir. Eski bir grafik sürücüsü veya bir sanal makine bunu sağlamayabilir."
- bir yükleyiciden: aynısı ve "Pencere olmadan da yükleme yapabilir: Bu dosyayı yönetici komut isteminden --apply ile çalıştırın. Nereye yükleneceğini seçmek için --target-root ve ardından bir klasör ekleyin."
- bir kaldırma programından: aynısı ve tam komutla birlikte "Pencere olmadan da yönetici komut isteminden kaldırma yapabilir:".

**Neden.** Pencereler [Direct3D 12](Glossary#direct3d-12) ile çizilir ve onları çizebilecek başka bir şey yoktur.

**Ne yapmalı.**

- Bilgisayarın grafik sürücüsünü güncelleştirin.
- Bir yükleyici veya kaldırma programı için komut satırını kullanın. Bkz. [Komut satırı](Command-Line).
- Uygulamanın komut satırı yoktur. Yükleyiciyi başka bir bilgisayarda oluşturun: Eski bilgisayarın sürücüsünü o bilgisayara bağlayabilirsiniz. Bkz. [Bağlı bir sürücüden taşıma](Moving-from-a-Connected-Drive). O bilgisayar eski bilgisayarı ağ üzerinden de okuyabilir. Bkz. [Ağ üzerinden bir bilgisayardan taşıma](Moving-over-the-Network).

<a id="starting-the-app"></a>

## Uygulamayı başlatma

| Gördüğünüz | Anlamı | Ne yapmalı |
| --- | --- | --- |
| UAC isteminde uygulama için "Bilinmeyen yayımcı" yazıyor | 0.5.0 ve sonrasında uygulama imzalıdır. İmzasız bir uygulama ya sürüm dosyası değildir ya da 0.5.0'dan öncesine aittir. | [En son sürümden](https://github.com/RandyNorthrup/WinAppMigrator/releases/latest) yeniden indirin ve [denetleyin](Getting-Started#check-the-download). |
| SmartScreen "Windows kişisel bilgisayarınızı korudu" diyor | SmartScreen o dosyayı sık görmemiş | **Ek bilgi** seçeneğini seçin, yayımcının Randy Northrup olduğunu denetleyin, ardından **Yine de çalıştır** düğmesini seçin. |
| Üstteki etikette **Okunamadı**, listede de "Uygulamalar listelenemedi" yazıyor | Uygulama listesi okunamadı | Listeyi yenilemek için dairesel oka tıklayın. Bağlı bir sürücüde, sürücünün hâlâ bağlı olduğunu denetleyin. |
| İstediğiniz uygulama listede yok | Bir Microsoft Store uygulamasıdır, başka bir uygulamanın gizli bir parçasıdır ya da taşınabilir bir uygulama gibi Ayarlar'da hiç girdisi yoktur | Store uygulamaları taşınamaz. Gizli bir parça, ana uygulamayı seçtiğinizde **Bunun da parçası olabilir** altında önerilebilir. Ayarlar'da girdisi olmayan bir uygulama seçilemez. |
| "Eşleşen uygulama yok" | Arama veya filtre her şeyi gizliyor | Arama kutusunu temizleyin ve **Tümü** seçeneğini seçin. |

<a id="building"></a>

## Oluşturma

<a id="picking-a-drive-or-a-disk-image"></a>

### Bir sürücü veya disk görüntüsü seçme

| Gördüğünüz | Ne yapmalı |
| --- | --- |
| Sürücü için "… konumu bir Windows sistem sürücüsüne benzemiyor (\Windows ve \Users yok). Diğer bilgisayardan gelen sürücünün kökünü seçin." | Sürücüdeki bir klasörü değil, `E:\` gibi sürücünün en üstünü seçin. Bunun, Windows'un yüklü olduğu sürücü olduğunu denetleyin. |
| Bir görüntü için "…, Windows'un bağlayabileceği bir disk görüntüsü (.vhd veya .vhdx) değil" | Bir `.vhd` veya `.vhdx` dosyası seçin. VMware veya VirtualBox diskini önce bunlardan biri olarak kaydedin. Bir Hyper-V denetim noktası (`.avhdx`) için sanal makinenin diskini dışarı aktarın. |
| Bir görüntü için "… bağlanamadı: Windows onu zaten bağlamış ve onu bağlayan şey ona yazıyor olabilir." | Görüntü başka bir yerde açık; örneğin Dosya Gezgini'nde bağlanmış. Bunun yerine sürücüsünü **Bağlı sürücü…** altında seçin ya da görüntüyü ayırın ve yeniden seçin. |
| Bir görüntü için "… dosyasında okunabilen ve Windows içeren bir birim yok. BitLocker ile şifrelenmiş bir birimin önce kilidinin açılması gerekir." | Bunun, Windows'un yüklü olduğu sürücünün görüntüsü olduğunu denetleyin. [BitLocker](Glossary#bitlocker) ile kilitliyse görüntüyü Windows'ta kendiniz bağlayın, kilidini açın ve sürücüsünü **Bağlı sürücü…** altında seçin. |
| Bir görüntü için "… bir fark diski; yalnızca oluşturulduğu diskten bu yana değişenleri tutar" | Hyper-V'nin Diski Düzenle komutuyla onu oluşturulduğu diskle birleştirin ve o diski seçin. |
| Bir görüntü için "… bağlanamadı:" ve başka bir neden | Neden, Windows'un kendi nedenidir. Okuyun, görüntüyü başka hiçbir şeyin kullanmadığını denetleyin ve yeniden deneyin. |

<a id="reading-a-pc-over-the-network"></a>

### Ağ üzerinden bir bilgisayarı okuma

Eski bilgisayarda istemcinin söyledikleri de dahil tam liste [Ağ üzerinden bir bilgisayardan taşıma](Moving-over-the-Network#if-something-goes-wrong) sayfasındadır.

| Gördüğünüz | Ne yapmalı |
| --- | --- |
| Eski bilgisayar **2. Eski bilgisayarı seçin** altında listelenmiyor | İstemcinin orada çalıştığını ve bir kod gösterdiğini, iki bilgisayarın da aynı ağda olduğunu denetleyin. Ardından bilgisayarın adını veya adresini yazın. Bilgisayar bir yönlendiricinin ötesindeyse adresini yazın. Windows'un kendi güvenlik duvarı dışındaki bir güvenlik duvarının TCP 47732 ve UDP 47731 bağlantı noktalarına izin vermesi gerekir. |
| Bir ad için "… ağda bulunamadı" | Bunun yerine eski bilgisayarın adresini yazın. |
| "eski bilgisayara ulaşılamadı:" ve bir neden | İstemcinin hâlâ çalıştığını ve adın veya adresin doğru olduğunu denetleyin. |
| "bu, eski bilgisayarın gösterdiği kod değil. Kodu denetleyip yeniden deneyin." | İstemcinin şu anda gösterdiği kodu yazın. Beş yanlış koddan sonra istemci yeni bir kod oluşturur. 20 yanlış koddan sonra, siz istemciyi yeniden başlatana kadar hiçbir eşleştirmeye izin vermez. |
| Eski bilgisayarda Windows, istemcinin yayımcısının bilinmediğini söylüyor | Bu yalnızca Microsoft'un 2020 kök sertifikası olmayan ve bu nedenle istemcinin imzasını denetleyemeyen bir Windows 7'de beklenen bir durumdur. Orada **Evet** düğmesini yalnızca uygulamadan kendiniz kaydettiğiniz bir istemci için seçin. Diğer her yerde istemde Randy Northrup adı görünmelidir. Görünmüyorsa o kopyayı çalıştırmayın. |
| Uzun bir aradan sonra uygulama eski bilgisayarı artık okuyamıyor | İstemci, uygulamanın 30 dakika boyunca hiçbir şey istemediği bir bağlantıyı sonlandırır. **Bağlantıyı kes** düğmesine tıklayın ve yeniden eşleştirin. |

<a id="looking-at-the-app"></a>

### Uygulamayı inceleme

| Gördüğünüz | Anlamı | Ne yapmalı |
| --- | --- | --- |
| "Bu uygulama incelenemedi:" ve bir neden | İnceleme başarısız oldu. | **Yeniden dene** düğmesine tıklayın. Aynı şekilde başarısız olursa girdi, WinAppMigrator'ın taşıyabileceği bir uygulama olmayabilir. |
| "Bu uygulamanın kaydı … dosyasını uygulamanın programı olarak gösterse de dosya artık orada değil" | Uygulamanın uygulama listesindeki girdisi, taşınmış veya kaldırılmış bir programı gösteriyor | Uygulamayı bu bilgisayarda onarın veya yeniden yükleyin, ardından onu yeniden seçin. |
| Bir uygulama için "… uygulamasına ait bir program bulunamadı, bu nedenle yükleyiciye konacak bir şey yok" | Girdinin kendine ait bir programı yok; grafik sürücüsü gibi sürücü paketlerinde, çalışma zamanlarında veya eklentilerde bu sık görülür | Yeni bilgisayarda sürücüyü üreticisinden veya Windows Update'ten, çalışma zamanını ya da eklentiyi ise onu kullanan uygulamayla birlikte yükleyin. |
| Bir uygulama için "…, bir tarayıcının bir web sitesinden yüklediği bir web uygulamasıdır" | Chrome Remote Desktop gibi, uygulama olarak yüklenmiş ve kendine ait bir programı olmayan bir web sitesi | Yeni bilgisayarda web sitesini aynı tarayıcıda açın ve uygulamayı oradan yükleyin. |
| "Durduruldu. Bu uygulama incelendikten sonra yükleyici oluşturulabilir." | **Durdur** düğmesine tıkladınız | **Yeniden incele** düğmesine tıklayın. |
| İnceleme uzun sürüyor | Büyük bir uygulama, yavaş veya bozulmakta olan bir sürücü ya da çok sayıda hesap | Sürücü bozuluyor olabilirse **Durdur** düğmesine tıklayın, sürücünün bir [disk görüntüsünü](Glossary#disk-image) alın ve bunun yerine onu seçin; çünkü bozulmakta olan bir sürücüyü okumak onu tamamen bozabilir. 0.6.0'dan itibaren görüntüyü **Disk görüntüsü…** ile seçin. 0.5.0'da görüntüyü bağlayıp sürücüsünü seçin. Aksi halde bekleyin. Bu sırada başka bir uygulama seçebilirsiniz. |
| İstediğiniz bir satır soluk ve işaretli | Ana programı veya başlaması için gerekenleri içeriyor | Her zaman yükleyiciye girer. Bkz. [işareti kaldırılamayan satırlar](Building-an-Installer#the-groups). |
| Bir satırda "Bu da dışarıda bırakıldı:" yazıyor | İşaretini kaldırdığınız bir şey onu da beraberinde götürdü | Bunu istiyorsanız o şeyi yeniden işaretleyin. |
| "Kalanlarla bir paket oluşturulamaz" | İşaretini kaldırdıklarınız, oluşturulabilecek hiçbir şey bırakmıyor | Bir şeyi yeniden işaretleyin. |
| "Dışarıda bıraktıklarınızın içindeki lisans verileri yine de eklenir" | Lisanslar yalnızca **Lisans ve etkinleştirme verileri** seçeneğine bağlıdır | Lisansların dışarıda kalmasını istiyorsanız bu seçeneğin işaretini kaldırın. |

<a id="create-installer-stays-greyed-out"></a>

### Yükleyici oluştur düğmesi soluk kalıyor

Düğme, şunların tümü doğru olana kadar bekler:

- uygulamanın incelenmesi, seçeneklerin şu anki haliyle bitmiştir ("Bununla birlikte nelerin gideceği belirleniyor…", son kaldırdığınız işaretin neleri beraberinde götürdüğünün hâlâ belirlendiği anlamına gelir);
- **Çıkış klasörü**, `D:\Installers` gibi tam bir yoldur ("D:\Yükleyiciler gibi tam bir klasör yolu girin.");
- o sürücüde yer vardır.

<a id="room"></a>

### Alan

| Gördüğünüz | Ne yapmalı |
| --- | --- |
| "Yeterli alan yok." ve bir sürücünün ne kadar alana ihtiyacı olduğu ve ne kadar boş alanı olduğu | Başka bir sürücüde bir çıkış klasörü seçin veya bazı öğeleri dışarıda bırakın. Bu rakam, bir oluşturma işleminin sıkıştırmadan önce kullanabileceği en fazla alandır; bu nedenle yükleyicinin boyutundan büyük olabilir. |
| Bir sürücü için "… ne kadar boş alanı olduğunu bildirmedi" | Oluşturma işlemi o sürücüyü denetlemeden devam eder. Sürücüde yer olduğundan emin olun. |
| FAT32 bir sürücü ve büyük bir uygulama | FAT32, 4 GiB büyüklüğünde bir dosyayı tutamaz. NTFS veya exFAT bir sürücü kullanın. |

<a id="the-done-screen"></a>

### Tamamlandı ekranı

| Gördüğünüz | Ne yapmalı |
| --- | --- |
| "Yükleyici oluşturuldu, ancak uygulama incelendikten sonra" birkaç dosya "kaybolduğu için yükleyicide yer almıyor" | Bu dosyalar önemliyse **Yeniden incele**, ardından **Yükleyiciyi yeniden oluştur** düğmesine tıklayın. |
| "Yükleyici oluşturuldu, ancak" birkaç dosya "alınamadı; genellikle bunun nedeni dosyaların kilitli veya okunamaz olmasıdır." | Uygulamayı ve dosyalarını kullanan diğer her şeyi kapatın, ardından yeniden oluşturun. Hangi dosyalar olduğu **Uyarılar** bölümünde listelenir. |
| "Yükleyici oluşturuldu, ancak hâlâ kimden geldiğine dair bilgiler içeriyor." | Altındaki sarı iletileri okuyun. Bkz. [son kontrol](Building-an-Installer#the-last-look). |
| **Uyarılar** bölümünde bir çalışma zamanı için "… indirilemedi" ve "yükleyici hedef bilgisayarda winget ile yüklemeyi deneyecek." yazıyor | Bu çalışma zamanı yükleyicinin içinde değildir, bu nedenle yeni bilgisayarın onun için internete ihtiyacı olacak. Çevrimiçiyken yeniden oluşturun. Bkz. [Çalışma zamanları](Runtimes#at-build-time). |
| "Bu yükleyiciyi kullanmayın: Karma denetiminden geçemedi." | Kullanmayın. Yeniden oluşturun. Yine olursa sürücü bozuluyor olabilir. |
| "Yükleyici yazıldı, ancak karma denetimi çalıştırılamadı" | Kullanmayın. Yeniden oluşturun. |
| "Oluşturma iptal edildi. Hiçbir yükleyici yazılmadı." | **İptal** düğmesine tıkladınız. |
| "Yükleyici oluşturulamadı." ve bir neden | Nedeni okuyun. **Hatayı kopyala** onu kopyalar, **Günlük** de günlüğün nerede olduğunu gösterir. |
| "… çıkış klasörü oluşturulamadı" | Yazabileceğiniz bir klasör seçin. |
| "Korumalı bir hazırlık klasörü oluşturulamadı, bu nedenle hiçbir şey oluşturulmadı" | Oluşturma işlemi, çıkış klasörünün yanında veya Windows'un geçici klasöründe kendi özel çalışma klasörünü oluşturamadı. Başka bir sürücüde bir çıkış klasörü seçin. |
| "Oluşturma beklenmedik şekilde durdu" | Hiçbir şeyin beklemediği bir sorun oluştu. "Çıkış klasöründe bir yükleyici kaldıysa onu kullanmayın." Lütfen [bildirin](#getting-help). |

<a id="installing"></a>

## Yükleme

<a id="when-the-installer-opens"></a>

### Yükleyici açıldığında

| Gördüğünüz | Anlamı | Ne yapmalı |
| --- | --- | --- |
| Uzun süre "Yükleyici denetleniyor…" | Karmasını denetlemek için paketin tamamını okuyor | Bekleyin. Büyük bir yükleyici biraz zaman alır. |
| "Yükleyici okunamadı:" ve "bu yükleyici hasarlı veya oluşturulduktan sonra değiştirilmiş, bu nedenle paketinden hiçbir şey okunmadı. Oluşturulduğu yerden yeniden kopyalayın." | Dosya kesik veya değiştirilmiş | Yeniden kopyalayın. |
| "bu yükleyici paketini aynı klasördeki", bir `.wamdata` dosyasının adı ve "adlı bir dosyada tutuyor ve bu dosya orada yok" | İki dosyalı bir yükleyicinin veri dosyası eksik | `.wamdata` dosyasını aynı klasöre kopyalayın. Bkz. [Büyük uygulamalar](Big-Apps). |
| Veri dosyası için "… bu yükleyicinin veri dosyası değil ya da hasarlı veya oluşturulduktan sonra değiştirilmiş" | Veri dosyası eşleşmiyor | İki dosyayı da oluşturuldukları yerden yeniden kopyalayın. |
| "Bu yükleyicinin paketi bu bilgisayarda kullanılamaz:" ve "bu uygulama bu bilgisayarda çalışamaz, bu nedenle hiçbir şey değiştirilmedi" | Uygulamanın programları başka bir işlemci için derlenmiş ya da bir program daha yeni bir Windows gerektirdiğini belirtiyor | Eskisine benzeyen bir bilgisayara yükleyin. Hiçbir şey değiştirilmedi. |
| "Bu bir WinAppMigrator yükleyici gövdesidir ve uygulama paketi taşımaz." | Bu, oluşturulmuş bir yükleyici değil, yalın gövdedir | Uygulamayla bir yükleyici oluşturun ve onu çalıştırın. |
| Windows'un nedeniyle birlikte "Bu yükleyicinin imzası geçerli değil" | İmzalanmış, ancak Windows imzayı kabul etmiyor. O zamandan beri değişmiş olabilir ya da sertifikasına bu bilgisayarda güvenilmiyor. | Yüklemeyin. Onu oluşturan kişiye danışın. |
| UAC isteminde "Bilinmeyen yayımcı" yazıyor | Yükleyici imzalı değil. Kendi oluşturduğunuz bir yükleyici için bu normaldir. | Yalnızca nereden geldiğine güveniyorsanız çalıştırın. |

<a id="the-install-location"></a>

### Yükleme konumu

**Yükle** ve **Değişiklikleri önizle** soluk olduğunda, **Yükleme konumu** altındaki kırmızı bir bildirim nedenini söyler. En sık neden, uygulamanın bu bilgisayarda o klasörde zaten yüklü olmasıdır; web tarayıcıları, medya oynatıcılar veya Office sık sık böyledir: Önce o kopyayı Windows Ayarları'nda **Uygulamalar** bölümünden kaldırın ve yükleyiciyi yeniden açın ya da yeni ve boş bir klasör yazın. Nedenler ve ne yapılacağı [Yeni bilgisayara yükleme](Installing-on-the-New-PC#when-the-location-cannot-be-used) sayfasındadır.

<a id="before-or-during-the-install"></a>

### Yüklemeden önce veya yükleme sırasında

| Gördüğünüz | Anlamı | Ne yapmalı |
| --- | --- | --- |
| "Burada yeterli alan olmayabilir" | Sürücüde yeterli alan yok gibi görünüyor | Yer açın veya başka bir sürücüde bir konum seçin. |
| "bunun için yeterli alan yok, bu nedenle hiçbir şey değiştirilmedi" | Yazacağı bir sürücü çok dolu | Orada yer açın veya başka bir sürücüde bir yükleme konumu seçin. |
| "Bunun burada yapacakları değişti, bu nedenle hiçbir şey yüklenmedi." | Liste belirlendikten sonra bilgisayarda bir şey değişti | Listeyi yeniden okuyun ve istediğiniz buysa **Yükle** düğmesine yeniden tıklayın. |
| Bir uygulama için "… uygulamasının daha önceki bir yüklemesi hiç tamamlanmadı. Yükle seçildiğinde önce o yükleme geri alınır." | Önceki bir yükleme yarıda kaldı | **Yükle** düğmesine tıklayın. Önce o yüklemeyi geri alır. |
| "başka bir yükleme veya kaldırma işlemi çalışıyor. Bitmesini bekleyin, ardından yeniden deneyin" | Aynı anda yalnızca biri çalışır | Bekleyin, ardından yeniden deneyin. |
| Bir dosya için "… yolu, bu Windows sürümünün dosya olarak değil aygıt olarak açtığı bir ad taşıyor" | Uygulamada `aux.h` gibi adlandırılmış, Windows 11'den önceki Windows sürümlerinin tutamadığı bir dosya var | Windows 11'e yükleyin. |
| "Hazırlık klasörü oluşturulamadı" | Paketin açılabileceği, yeterli alanı olan özel bir yer yok | Yükleme konumunun sürücüsünde veya sistem sürücüsünde yer açın. |
| `%AppData%` gibi bir veri klasörünün bir bağlantı olduğu söyleniyor | Yükleyici bir [bağlantı](Glossary#link) üzerinden yazmaz | **Kullanıcı verileri** ve **Lisans** seçeneklerinin işaretini kaldırın ve onlar olmadan yükleyin. |
| "Bu bilgisayarın ProgramData klasörü bulunamadı, bu nedenle hiçbir şey yüklenmedi" | Windows'un ProgramData klasörünün nerede olduğuna dair kendi kaydı okunamadı | Yükleyici, kaydını tutacak güvenli bir yer olmadan yükleme yapmaz. |

<a id="the-result"></a>

### Sonuç

| Gördüğünüz | Ne yapmalı |
| --- | --- |
| "Yükleme başarısız oldu ve değiştirdikleri eski haline getirildi." | Nedenini kırmızı iletilerde okuyun. Yüklediği çalışma zamanları dışında bilgisayar eskisi gibidir. |
| "Yükleme başarısız oldu ve değiştirdiklerinin tümü eski haline getirilemedi." | Listelenenleri okuyun. Bu dosyaları kullanıyor olabilecek programları kapatın ve bir yükleyiciyi yeniden çalıştırın: Önce bunu yeniden dener. |
| Bir çalışma zamanı için "… yüklenemedi" | Çalışma zamanı yüklenemedi, bu nedenle yükleme eski haline getirildi. DirectX End-User Runtime için yeni bilgisayarın internete ihtiyacı vardır. Bkz. [Çalışma zamanları](Runtimes#with-no-internet-on-the-new-pc). |
| "Bir bağımlılığın tamamlanması için yeniden başlatma gerekiyor. Windows'u yeniden başlatın, ardından uygulamayı açın." | Yeniden başlatın, ardından uygulamayı başlatın. |
| "… hizmeti kaynak bilgisayarda … olarak çalışıyordu. Parolası yükleyicide olmadığından hizmet LocalService olarak ayarlandı." | Hizmet bu haliyle çalışmıyorsa Hizmetler'i açın ve hizmetin hesabını ve parolasını ayarlayın. |
| Bazı dosyalar "bunun yerine" yükleme klasörünün "altında, _external adlı bir klasöre gider" | Bu bilgisayar Belgeler veya Kayıtlı Oyunlar klasörünü profilinizin dışında tutuyor. Bu dosyaları o klasöre kendiniz taşıyın. |
| Bir şey bir nedenle "dışarıda bırakıldı" | Kurallar onu reddetti ya da burada zaten başka bir uygulamaya ait. Geri kalanı yüklendi. Uygulamanın buna ihtiyacı varsa nedene bakın. |
| **Paket karmaları GEÇERSİZ** veya **Yüklenen dosyalar GEÇERSİZ** | Bir şey eşleşmedi. Yükleyiciyi yeniden kopyalayın ve bir kez daha deneyin. |

<a id="after-installing"></a>

### Yüklemeden sonra

| Gördüğünüz | Ne yapmalı |
| --- | --- |
| Uygulama başlamıyor | **Bağımlılıklar** bölümünde **El ile yükleyin** olarak işaretlenmiş bir çalışma zamanı olup olmadığına bakın ve onu yükleyin. Sonuçtaki uyarıları denetleyin. |
| Windows 7'den gelen bir uygulama başlamıyor ve oluşturulurken bir uyarı, "Windows 10 ile 11 bu dosyayla birlikte gelmez" denen bir DLL'yi adlandırmıştı | O DLL eski Windows'un kendi klasörlerine başka bir şey tarafından konmuştu, bu nedenle uygulamayla birlikte gelmedi. Onu sağlayan şeyi yükleyin, ardından uygulamayı yeniden başlatın. Bkz. [Windows 7, Vista, 8 veya 8.1'deki uygulamalar](Moving-from-a-Connected-Drive#apps-from-windows-7-vista-8-or-81). |
| Uygulama yeniymiş gibi başlıyor | **Kullanıcı verileri** kapalıyken oluşturulmuş bir yükleyicide bu beklenen bir durumdur. Yükleme sırasında **Kullanıcı verileri** işareti kaldırıldıysa da olur, çünkü veriler o zaman yükleme klasörünün altına gider; yükleyici kimin oturum açtığını belirleyemediğinde de olur ve yükleyici bunu söyler. |
| Uygulama etkinleştirilmeyi istiyor | Eski bilgisayara bağlı bir lisansın yeniden etkinleştirilmesi gerekir. |
| Uygulamanın kendi güncelleyicisi uygulamanın yüklü olmadığını düşünüyor | Windows Installer'ın burada onunla ilgili bir kaydı yok. Bkz. [Sınırlamalar](Limitations). |
| Bir dosya türü veya bağlantı türü uygulamayla açılmıyor | Burada başka bir uygulama zaten ona sahipti, bu nedenle ona dokunulmadı. Uygulamayı Ayarlar'da, Varsayılan uygulamalar altında seçin. |
| Yeni bir `Path` girdisi görülmüyor | Yeni bir PowerShell penceresi veya komut istemi açın. Bazı programlar bir değişikliği ancak oturumu kapatıp yeniden açtıktan sonra görür. |

<a id="uninstalling"></a>

## Kaldırma

| Gördüğünüz | Ne yapmalı |
| --- | --- |
| "Bir kısmı bağlı olmayan bir sürücüde." | O sürücüyü bağlayın ve Ayarlar'dan yeniden kaldırın. |
| "Tümü geri alınamadı. Onu kullanan programları kapatın ve Ayarlar'dan yeniden kaldırın." | Uygulamayı ve dosyalarını kullanan her şeyi kapatın. Bir hizmet durmuyorsa bilgisayarı yeniden başlatın. Ardından yeniden kaldırın. |
| "bu bir yükleme kimliği değil" (komut satırında: "this is not an install's id") | `--uninstall` için verilen kimlik yanlış. Kimliği Ayarlar'daki girdiden kopyalayın. Bkz. [Komut satırı](Command-Line#finding-the-id). |
| `.before-install` adlı bir dosya ortaya çıktı | Bu, yüklemenin üzerine yazdığı ve sizin o zamandan beri değiştirdiğiniz bir dosyanın aslıdır. Hangisini istiyorsanız onu saklayın. |
| Bir çalışma zamanı hâlâ yüklü | Başka programlar kullanıyor olabileceğinden çalışma zamanları kalır. Başka hiçbir şeyin kullanmadığından emin değilseniz dokunmayın. |

<a id="scripts"></a>

## Betikler

| Gördüğünüz | Ne yapmalı |
| --- | --- |
| "İstenen işlem yükseltme gerektiriyor" | PowerShell'i yönetici olarak çalıştırın. |
| Hiçbir şey yazdırılmıyor ve `$LASTEXITCODE` ayarlanmıyor | Yükleyicinin çıktısını, örneğin `Out-String` komutuna yönlendirin. Bkz. [Komut satırı](Command-Line). |
| Çıkış kodu `2` | Bir seçenek yanlış ya da `--apply` ve `--dry-run` birlikte kullanılmış. |
| "this package does not say where its app was, so give a --target-root" (komut satırı her zaman İngilizce yazar) | Tam bir yolla birlikte `--target-root` ekleyin. |
| Uygulamanın bulunduğu klasörle birlikte "was in" ve "and that cannot be used" | Başka bir klasörle birlikte `--target-root` ekleyin. |
| Bir klasör için "cannot be used: it already has files" ve "Give --force only if you know it is this app's" | Boş bir klasör seçin. `--force` seçeneğini yalnızca bu uygulamaya ait olduğunu bildiğiniz bir klasör için kullanın. |

<a id="language-and-text"></a>

## Dil ve metin

- **Yanlış dil.** Uygulamanın veya bir yükleyicinin ilk ekranının sağ üstündeki **Dil** altında başka bir dil seçin. Uygulama seçiminizi hatırlar; yükleyici ise o çalıştırma boyunca korur. Seçim yapılmamışsa her biri, Windows görüntüleme dilleri arasından sahip olduğu ilk dili, yoksa İngilizceyi konuşur.
- **Bir çeviri yanlış okunuyor.** İngilizce dışındaki her şey makine tarafından çevrildi. **Dil** altında, dilinizin önceden doldurulduğu bir form açan **Çeviri sorunu bildir** seçeneğini seçin. Ekranın ne gösterdiğini ve ne demesi gerektiğini yazın.
- **Harfler boş kutular olarak görünüyor.** Pencere, her yazı sistemi için Windows'la birlikte gelen yazı tiplerini `C:\Windows\Fonts` klasöründen kullanır. Bunların bazıları kaldırılmış bir Windows bu harfleri kutu olarak çizer; listedeki diğer uygulamaların adları da bu yazı tiplerine ihtiyaç duyabilir.
- **Sözcükler kesiliyor.** Sığmayan bir etiket `…` ile biter; tamamını görmek için üzerine gelin veya Sekme tuşuyla ona gidin. Pencerenin sığacak hale getirilebilmesi için bunu dil ve ekranla birlikte bildirin.
- **WinAppMigrator Client, sağdan sola yazılan bir Windows'ta İngilizce yazdırıyor.** Çalıştığı konsol Arapça veya İbranice harfleri birleştiremez ya da sıralayamaz; bu nedenle İngilizce yazdırır ve nedenini söyler. `--lang` ile sahip olduğu herhangi bir dili seçebilirsiniz.

<a id="getting-help"></a>

## Yardım alma

1. [Günlüğü](Glossary#log) bulun. Yolu, bir başarısızlıkta **Günlük** olarak gösterilir ya da komut satırından `Log:` olarak yazdırılır. Günlükler `%ProgramData%\WinAppMigrator\logs` klasöründedir. Yolları ve anahtar adlarını içerirler; bir ayarın, lisans dosyasının veya ortam değişkeninin içeriğini asla içermezler. Yine de klasör adlarınızı içerebilirler, bu nedenle paylaşmadan önce okuyun.
2. Sürümü, ne yaptığınızı ve ekranın ne dediğini belirterek [bir sorun kaydı açın](https://github.com/RandyNorthrup/WinAppMigrator/issues). Uygulamanın tamamlandı ekranındaki **Hatayı kopyala** iletiyi kopyalar.
3. **Bir sorun kaydına asla yükleyici eklemeyin.** Özel veriler içerebilir.
4. Bir güvenlik sorununu ise [GitHub'ın güvenlik bildirimleri (security advisories)](https://github.com/RandyNorthrup/WinAppMigrator/security/advisories/new) aracılığıyla gizli olarak bildirin.
