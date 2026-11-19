<a id="installing-on-the-new-pc"></a>

# Yeni bilgisayara yükleme

Yükleyici tek bir penceredir. Kendini denetler, nereye yükleyeceğini ve ayarlayacağı her şeyi size gösterir ve yalnızca **Yükle** düğmesine tıkladığınızda yükler. Bu sayfa, ekrandaki sözcükleri kullanarak yükleyiciyi yukarıdan aşağıya anlatır.

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/installer-dark.png">
    <img alt="Fabrikam Data Sync yükleyicisi; imzalı olmadığını, yükleme konumunu, gereken alanı ve dosyalar dışında neleri ayarladığını gösteriyor" src="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/installer-light.png" width="600">
  </picture>
</p>

<a id="before-you-start"></a>

## Başlamadan önce

- **Yükleyicinin tamamını kopyalayın.** Oluşturma işlemi size `.exe` dosyasının yanında bir `.wamdata` dosyası da verdiyse ikisini de aynı klasöre kopyalayın ve adlarını değiştirmeyin. Bkz. [Büyük uygulamalar](Big-Apps).
- **Yalnızca güvendiğiniz yükleyicileri çalıştırın.** Bir yükleyici tam [yönetici haklarıyla](Glossary#administrator-rights) çalışır. Ona diğer programlar gibi davranın: Yalnızca kendiniz oluşturduysanız veya güvendiğiniz birinden aldıysanız çalıştırın. Bkz. [Gizlilik ve güvenlik](Privacy-and-Security).
- **Yönetici hakları gerekir.** Bu bilgisayarda yönetici haklarına, pencere için de [Direct3D 12](Glossary#direct3d-12) desteğine ihtiyacınız var. Direct3D 12 olmadan da komut isteminden yükleme yapabilirsiniz. Bkz. [Komut satırı](Command-Line).

<a id="open-it"></a>

## Açma

Yükleyiciye çift tıklayın. Windows [UAC istemini](Glossary#uac-prompt) gösterir. Bir yükleyici, onu oluşturan kişi imzalamadıkça imzasızdır; bu nedenle istemde genellikle **Bilinmeyen yayımcı** yazar. Bu beklenen bir durumdur. Bkz. [Gizlilik ve güvenlik](Privacy-and-Security#signing).

"WinAppMigrator Yükleyicisi" penceresi, "Başka bir bilgisayardan alınan bir uygulamayı yeniden yükler" yazısıyla, yükleyici ne kadar büyük olursa olsun hemen açılır. Paketinin tamamının [karmasını](Glossary#hash) denetlerken "Yükleyici denetleniyor…" yazar. Bu denetim paketin tamamını okur, bu nedenle büyük bir yükleyicide biraz zaman alır. Bu andan itibaren yükleyici kendi dosyasını ve varsa veri dosyasını açık tutar; böylece pencere açıkken ikisi de değiştirilemez ya da başka bir dosyayla değiştirilemez.

Denetim burada kırmızı bir iletiyle durabilir. Sık görülenler şunlardır:

- Yükleyici hasarlı veya oluşturulduktan sonra değiştirilmiş: Oluşturulduğu yerden yeniden kopyalayın.
- Veri dosyası eksik veya doğru dosya değil: İki dosyayı da yeniden, tek bir klasöre kopyalayın.
- "Bu yükleyicinin paketi bu bilgisayarda kullanılamaz"; örneğin uygulamanın programları başka türde bir işlemci için derlendiğinden veya daha yeni bir Windows gerektirdiğinden. Hiçbir şey değiştirilmemiştir.

Bunların tümü [Sorun giderme](Troubleshooting#installing) sayfasında açıklanmıştır.

Denetim geçtiğinde pencerenin başlığı, uygulamanın adını taşıyan "… uygulamasını yükle" olur; örneğin "Fabrikam Data Sync uygulamasını yükle".

<a id="its-language"></a>

### Dili

Bir yükleyici, oluşturulduğu bilgisayarın değil, çalıştığı bilgisayarın dilini konuşur: Windows görüntüleme dili WinAppMigrator'ın 24 dilinden biriyse o dili, değilse İngilizceyi. Sağ üstteki **Dil**, bu çalıştırma için dili değiştirir. Uygulamayla ilgili listelediği her şey, uygulamanın yükleyiciyi oluştururken yazdığı notlar da dahil, o dilde gelir. WinAppMigrator 0.6 veya daha eski bir sürümle oluşturulmuş bir yükleyici bu notları yazıldıkları gibi İngilizce gösterir. [Komut satırından sessiz yükleme](Command-Line), betiklerin okuyabilmesi için her zaman İngilizce yanıt verir.

<a id="the-app-and-where-it-goes"></a>

## Uygulama ve gideceği yer

İlk kartta uygulamanın adı, altında da yayımcı ve sürüm yer alır; örneğin "Fabrikam Inc. · 1.7.3". Ardından şunlar gelir:

| Satır | Ne söyler |
| --- | --- |
| **Alındığı bilgisayar** | Uygulamanın geldiği bilgisayarın adı. Yalnızca yükleyici **Kullanıcı verileri** seçeneği açıkken oluşturulduysa görünür. |
| **Yükleyici** | Bu yükleyici dosyasının [imzalı](Glossary#signature) olup olmadığı: Önce "İmzası denetleniyor…", ardından "İmzalanmamış: yayımcı bilinmiyor" ya da "İmzalayan:" ve imzalayanın adı. Geçerliliği doğrulanamayan bir imza bunun yerine kırmızı bir ileti gösterir: Windows'un belirttiği nedenle birlikte "Bu yükleyicinin imzası geçerli değil" ve "İmzalandıktan sonra değiştirilmiş olabilir." |
| **Yükleme konumu** | Uygulamanın gireceği klasör; başka bir klasör seçmek için **Gözat** düğmesi de vardır. Başlangıçta uygulamanın eski bilgisayarda bulunduğu klasördür. 32 bit bir Windows'tan gelen uygulama ise bunun yerine, 64 bit Windows'un 32 bit uygulamaları tuttuğu `Program Files (x86)` içinde başlar. Bu konum burada kullanılamıyorsa altındaki kırmızı bir bildirim nedenini söyler. |
| **Gereken alan** | Yüklemenin o sürücüde ne kadar alana ihtiyaç duyduğu ve ne kadar boş alan olduğu; örneğin "C: üzerinde en fazla 824,0 MB (356,0 GB boş)". Uygulamanın dosyalarının yanı sıra açılmış paketi de hesaba katar. |

İlk satırdaki yayımcı adı paketten gelir, yani herkes yazmış olabilir. **Yükleyici** satırı ise Windows'tan gelir ve güvenilmesi gereken odur.

<a id="when-the-location-cannot-be-used"></a>

### Konum kullanılamadığında

**Yükleme konumu** içindeki klasör kullanılamıyorsa altındaki kırmızı bir bildirim nedenini söyler. Siz başka bir klasör seçene kadar **Yükle** ve **Değişiklikleri önizle** soluk kalır ve yanlarındaki bir satırda "Yükleme konumu kullanılabilir olana kadar Yükle ve Değişiklikleri önizle kapalıdır." yazar.

En sık neden, klasörde zaten dosyalar olmasıdır. İçinde dosya bulunan bir klasör yalnızca o dosyaları daha önceki bir WinAppMigrator yüklemesi koyduysa kullanılır; böylece başka bir uygulamanın dosyalarının üzerine hiçbir zaman yazılmaz:

| Bildirim | Anlamı | Ne yapmalı |
| --- | --- | --- |
| Uygulamanın adı, ardından "bu klasörde zaten yüklü, bu nedenle bu kopya buraya yüklenemez." | Uygulama bu bilgisayarda zaten yüklü; çoğu zaman kendi yükleyicisiyle yüklenmiştir: Ana programı o klasördedir ya da Windows'un yüklü uygulamalar listesi onu orada gösterir. Bu, web tarayıcısı, medya oynatıcı veya Office gibi birçok bilgisayarda zaten bulunan uygulamalarda olur. | Önce bu bilgisayardaki kopyayı Windows Ayarları'nda **Uygulamalar** bölümünden kaldırın, ardından yükleyiciyi yeniden açın. Ya da bu kopyayı diğerinin yanına yüklemek için **Yükleme konumu** kutusuna yeni ve boş bir klasör yazın. |
| Başka bir uygulamanın adı, ardından "bu klasörde yüklü" ve bu uygulamanın "buraya yüklenemez" olduğu. | Windows'un yüklü uygulamalar listesi o klasörde başka bir uygulamayı, örneğin Office'in başka bir sürümünü gösteriyor. | **Yükleme konumu** kutusuna yeni ve boş bir klasör yazın. Ya da diğer uygulamaya artık ihtiyacınız yoksa önce onu Windows Ayarları'nda **Uygulamalar** bölümünden kaldırın, ardından yükleyiciyi yeniden açın. |
| "Bu klasörde WinAppMigrator tarafından yüklenmemiş dosyalar zaten var." | Klasörde başka bir şey var ve orada listelenen yüklü bir uygulama yok. Başka bir uygulamaya ait olabilir. | **Yükleme konumu** kutusuna yeni ve boş bir klasör yazın. Dosyalar kaldırdığınız bir uygulamadan kalmışsa klasörü silebilir, ardından yükleyiciyi yeniden açabilirsiniz. |

Başka herhangi bir nedende bildirimde "Bu klasör kullanılamaz:", ardından neden ve "Yukarıya başka bir klasör girin." yazar. Nedenler şunlardır:

| Ekrandaki neden | Ne yapmalı |
| --- | --- |
| "tam bir yol olmalıdır" | `C:\Program Files\Fabrikam Data Sync` gibi bir sürücü harfiyle başlayan bir yol yazın. |
| "sürücünün tamamı olamaz" | Sürücünün kendisini değil, sürücüdeki bir klasörü kullanın. |
| "Windows klasörü olamaz" | `C:\Windows` dışında bir klasör seçin. |
| "başka şeyleri de yan yana barındırıyor, bu nedenle uygulamanın bunun içinde bir klasöre ihtiyacı var" | Uygulama için bir klasör ekleyin; örneğin `C:\Program Files` yerine `C:\Program Files\Fabrikam Data Sync`. |
| "Windows'a ait bir klasör veya içeriği kendiliğinden çalışan ya da yüklenen bir klasör" | Sıradan bir klasör seçin. Örneğin bir Başlangıç klasörü reddedilir. |
| "bu bilgisayarın bir sürücüsündeki düz bir yol olmalıdır" | Bir [ağ paylaşımı](Glossary#network-share) reddedilir; Windows'un başka bir şey olarak okuyacağı bir ad da reddedilir. |
| Klasörün "başka bir yere giden bir bağlantı ve nereye gittiğini başkaları değiştirebilir" olduğu | [Bağlantının](Glossary#link) gittiği klasörü kullanın. |
| "yüklemelerin kayıtlarını tuttuğu yer burası" | `%ProgramData%\WinAppMigrator` dışında bir klasör seçin. |

Klasörde dosyalar varsa ve orada bu projeden daha önceki bir yükleme **kayıtlıysa**, yükleyici klasörü kullanır ve şu uyarıyı verir: "Bu klasörde zaten dosyalar var. Aynı adlı dosyaların üzerine yazılır; uygulamayı kaldırırsanız eski dosyalar geri gelir."

Bir klasör kabul edildikten sonra pencere, [komut satırında](Command-Line#what---force-does) `--force` seçeneğinin yaptığı gibi, aynı adlı bir dosyanın, kayıt defteri değerinin, hizmetin, görevin, kısayolun veya değişkenin üzerine yazar. Başka bir uygulamaya zaten ait olanlar, örneğin onun hizmeti veya dosya türü, yine o uygulamanın olarak kalır.

Sürücüde yeterli alan yok gibi görünüyorsa sarı bir notta "Burada yeterli alan olmayabilir" yazar ve ne kadar alan gerektiği belirtilir. Paket başka bir sürücüde açılabilir, ancak uygulamanın dosyalarının yine de bu sürücüye sığması gerekir. Yüklemenin kendisi de başlamadan önce alanı yeniden denetler.

<a id="an-earlier-install-that-never-finished"></a>

### Hiç tamamlanmamış önceki bir yükleme

Bu yükleyicilerden herhangi birinin önceki bir yüklemesi bir çökme veya elektrik kesintisi nedeniyle yarıda kaldıysa, sarı bir notta uygulamanın adıyla birlikte "… uygulamasının daha önceki bir yüklemesi hiç tamamlanmadı. Yükle seçildiğinde önce o yükleme geri alınır." yazar. Bu, **Yükle** düğmesine tıkladığınızda otomatik olarak yapılır.

<a id="what-this-sets-up-beyond-files"></a>

## Dosyalar dışında ayarlananlar

Bu kart, **Yükle** düğmesine tıkladığınızda kabul ettiğiniz listedir. Başlığının altında şu yazar:

> Bunların tümü yönetici haklarıyla yapılır. Yükleme başarısız olursa değiştirdikleri eski haline getirilir ve eski haline getirilemeyen her şey size bildirilir. Yüklemeyi daha sonra Ayarlar'da Yüklü uygulamalar bölümünden de geri alabilirsiniz. Üzerine yazdığı dosyalar geri gelir; yüklediği çalışma zamanları kalır.

Her satırda bir ad, sağda neyin söz konusu olduğunu belirten bir etiket ve ne yaptığını anlatan bir satır bulunur. Her zamanki mavi renkteki etiket sıradandır. **Sarı** renkli bir etiket, tam haklarla çalışan bir şeyi işaret eder; örneğin LocalSystem olarak çalışan bir hizmet, bir sürücü veya gelen bağlantılara izin veren bir güvenlik duvarı kuralı. Bunlara iki kez bakın.

Liste boşsa "Hiçbir şey: Yalnızca dosyaları ve uygulamanın kendi kayıt defteri anahtarlarındaki ayarları kopyalar." yazar.

Liste, kutudaki konuma göre belirlenir. Konumu değiştirirseniz kısa bir süre "Bunun bu konumda neleri ayarlayacağı belirleniyor…" yazar ve **Yükle** bekler.

<a id="what-each-label-means"></a>

### Her etiketin anlamı

| Etiket | Nedir | Altındaki satıra örnek |
| --- | --- | --- |
| **Hizmet** | Bir [hizmet](Glossary#service): Windows'un arka planda çalıştırdığı bir program | "FabrikamSync.exe dosyasını NT AUTHORITY\LocalService olarak çalıştırır; şimdi ve bu bilgisayar her başladığında başlar". Diğerlerinde "şimdi ve bu bilgisayar her başladıktan kısa süre sonra başlar", "yalnızca bir şey istediğinde başlar" veya "devre dışı" yazar. |
| **Sürücü** | Uygulamaya ait bir [sürücü](Glossary#driver) | Bir dosya için "… dosyasını doğrudan Windows'a yükler". Her zaman sarıdır. Windows ile başlayan bir sürücü, yüklemeden hemen sonra başlatılır. VeraCrypt'in sürücüsü bir test bilgisayarında bu şekilde elle yüklendi; CI bir sürücü yüklemez. |
| **Zamanlanmış görev** | Bir [zamanlanmış görev](Glossary#scheduled-task) | "Çalıştığı hesap: SYSTEM" veya başka bir hesap. "alabileceği en yüksek haklarla" sarı ile işaretlenir. Eski bilgisayarda bir kişi olarak çalışan görev, onu yükleyen kişi olarak çalışır. |
| **Otomatik başlayan** | Bir [başlangıç öğesi](Glossary#startup-item) | "Oturum açtığınızda çalışır", "Oturum açan herkes için çalışır" veya "Başlangıç klasöründe bir kısayol, sizin için" |
| **Kısayol** | Masaüstünde veya Başlat menüsünde bir [kısayol](Glossary#shortcut) | "Fabrikam.exe dosyasını açar, bu bilgisayardaki herkes için" veya "sizin için" |
| **PATH** | [Path değişkenine](Glossary#path-variable) eklenen bir girdi | "Path değişkenine ekler" ve "Yükleme klasörü\bin". Var olanın yerine konmaz, her zaman ona eklenir. |
| **Ortam değişkeni** | Bir [ortam değişkeni](Glossary#environment-variable) | "Şu değere ayarlanır:" ve değeri |
| **Kabuk uzantısı** | Dosya Gezgini'ne yüklenen kod | Bir dosya için "… dosyasını Dosya Gezgini'ne yükler, bu bilgisayardaki herkes için". Sarıdır. |
| **Menü komutu** | Sağ tıklama menüsündeki bir komut | Bir dosya türü için "… menüsüne bir komut ekler". "Bunlardan birini açmanın ne yapacağını değiştirir" sarıdır. |
| **Bağlantı türü** | `fabrikam:` gibi bir [bağlantı protokolü](Glossary#link-protocol) | "Bu türdeki bağlantılar uygulamayla açılır" |
| **Dosya türü** | `.fds` gibi bir [dosya türü](Glossary#file-type) | "Bu türdeki dosyalar uygulamayla açılır" |
| **COM** | Diğer programların başlatabileceği [COM sınıfları](Glossary#com-class) | Bir dosya için "… içinde COM" ve kaç ayar olduğu. "COM izinleri" sarıdır. |
| **Güvenlik duvarı kuralı** | Programlarından biri için bir [güvenlik duvarı kuralı](Glossary#firewall-rule) | "FabrikamSync.exe için tüm ağlarda 8080 bağlantı noktasında gelen bağlantılara izin verir". Gelen bağlantılara izin vermek sarıdır. |
| **Olay günlüğü kaynağı** | Uygulamanın Windows olay günlüklerine yazarken kullandığı ad | "Application günlüğüne, kendi dosyalarındaki iletilerle yazabilir" |
| **Yazı tipi** | Yükleme klasöründeki bir yazı tipi | "Oturum açıldığında yükleme klasöründen yüklenir" |
| **ODBC sürücüsü** | [ODBC](Glossary#odbc) için bir veritabanı sürücüsü | "Bir veritabanına erişmek için ODBC kullanan programlara yüklenir". Sarıdır. |
| **ODBC veri kaynağı** | Kaydedilmiş bir veritabanı bağlantısı | "Bir veritabanı bağlantısı, bu bilgisayardaki herkes için" |
| **Uyumluluk ayarı** | Windows'un bazı programlarını nasıl başlattığı | "Programlarının nasıl başladığı" |
| **Varsayılan uygulamalar** | Uygulamayı Varsayılan uygulamalar bölümünde seçmenizi sağlayan bir girdi | "Varsayılan uygulamalar bölümünde seçilebilir. Mevcut ayarda hiçbir şey değiştirilmez" |
| **Program adı** | Çalıştır'ın ve diğer programların uygulamayı bulabileceği bir ad | "Çalıştır ve diğer programlar onu bu adla bulabilir" |
| **Diğer ayarlar** | Uygulamanın kendi anahtarları ve yukarıdaki türler dışındaki [kayıt defteri](Glossary#registry) değerleri | "Uygulamanın kendi anahtarları dışında 3 ayar" ve anahtarlarla birlikte "Konum:". Bunlardan herhangi biri Microsoft'un anahtarlarındaysa sarıdır. |

Bazı şeyler ancak yükleme onlara geldiğinde değerlendirilebilir; örneğin başka bir uygulamaya zaten ait olan aynı adlı bir hizmet. Bunlar listede istendiği gibi kalır. Yükleme oraya geldiğinde onlara dokunmaz ve bunu bildirir.

<a id="notes-and-what-is-left-out"></a>

### Notlar ve dışarıda bırakılanlar

Satırların altında sarı notlar, bilmeniz gerekenleri söyler; örneğin:

- kimlik bilgilerinin yükleyiciye konmadığını, bu nedenle bunları yeniden girmeniz gerekebileceğini;
- yükleyici oluşturulurken **Lisans** veya **Kullanıcı verileri** seçeneğinin kapalı olduğunu, bu nedenle o verilerin yükleyicide olmadığını;
- uygulamanın dayandığı görünen ve hiçbir zaman yüklenmeyen [sertifikaları](Glossary#certificate);
- yükleyiciyi oluşturan kişinin neleri dışarıda bıraktığını ve neleri eklediğini;
- uygulamanın burada çalışmayabileceğini: Daha yeni bir derleme için "Bu uygulama Windows'un … derlemesindeydi" ya da Windows'un bazı programlarını "bir öykünücü üzerinden" çalıştıracağını;
- Belgeler veya Kayıtlı Oyunlar klasöründeki dosyaların yükleme klasörünün "altında, _external adlı bir klasöre" gideceğini, çünkü bu bilgisayarın o klasörünün profilinizin dışında olduğunu.

Ardından, güvenlik kurallarının dışarıda bıraktığı her şey nedeniyle birlikte sarı renkte listelenir. İlk 20 tanesi tam olarak gösterilir. Geri kalanlar, açabileceğiniz "… öğe daha dışarıda bırakıldı" altındadır. Yüklemenin geri kalanı onlar olmadan yine de yapılır. Nelerin neden reddedildiği [Bir yükleyiciye neler girer](What-Goes-in-an-Installer#what-the-installer-refuses) sayfasındadır.

Paket seçilen konuma hiç yüklenemiyorsa kırmızı bir iletide "Bu paket bu konuma yüklenemez" ve nedeni yazar.

<a id="options"></a>

## Seçenekler

Bu kart yalnızca yükleyici kullanıcı verileri veya lisans verileri içerdiğinde görünür. Her kutu işaretli olarak başlar.

| Seçenek | Ne söyler |
| --- | --- |
| **Kullanıcı verileri** | "Açık: Uygulama ayarları, Belgeler ve Kayıtlı Oyunlar, oturum açmış kişinin olağan klasörlerine gider. Kapalı: Tümü yükleme klasörünün altına gider ve oradan çalışacak hiçbir şey ayarlanmaz." Yükleyici kimin verilerini içerdiğini belirtiyorsa altında "Bu, … hesabının verilerini içerir." satırı bulunur. |
| **Lisans** | "Etkinleştirme ve lisans dosyaları ile lisans bilgilerini tutan kayıt defteri değerleri. Bazılarının burada yeniden etkinleştirilmesi gerekebilir." |

Bir kutuyu değiştirdiğinizde yukarıdaki liste hemen buna göre değişir.

Kullanıcı verileri, UAC isteminde başka bir yöneticinin adı yazılmış olsa bile her zaman Windows'ta **oturum açmış kişiye** gider. Yükleyici bunun kim olduğunu belirleyemezse bunu söyler, kullanıcı başına ayarları dışarıda bırakır ve tahmin yürütmek yerine kullanıcı verilerini yükleme klasörünün altına koyar. O oturumda kimse oturum açmamışsa kullanıcı verileri ve kullanıcı başına ayarlar yükleyicinin çalıştığı hesaba gider ve yükleyici bunu da söyler.

<a id="dependencies"></a>

## Bağımlılıklar

"Bu uygulamanın gereksinimleri ve her birinin nasıl sağlandığı." Her [çalışma zamanının](Glossary#runtime) bir etiketi vardır:

| Etiket | Anlamı |
| --- | --- |
| **Dahil** | Yükleyicinin içindedir. İnternet olmadan yüklenir; yalnızca DirectX End-User Runtime bunun dışındadır, çünkü onun yükleyicisi çalışma zamanını Microsoft'tan indirir. Bunlardan yalnızca x64 Visual C++ çalışma zamanı projenin testlerinde gerçekten yüklendi. |
| **winget ile yüklenir:** ve bir kimlik | Yükleyici oluşturulurken indirilemediği için yükleyici, internet gerektiren [winget](Glossary#winget) aracını kullanır. Bu yol projenin testlerinde çalıştırılmadı. |
| **Windows özelliği: NetFx3** | Bir [Windows özelliği](Glossary#windows-feature) olarak açılan .NET Framework 3.5. Bu, internet gerektirebilir. Bu yol projenin testlerinde çalıştırılmadı. |
| **Windows'ta yerleşik** | Windows 10 ve 11'de zaten bulunan .NET Framework 4 |
| **Uygulamayla birlikte gelir** | Uygulama kendi kopyasını taşır |
| **El ile yükleyin** | Burada bunun için bir yükleyici yok. Kendiniz yükleyin. |

Hiç bağımlılık yoksa "Ek çalışma zamanı bağımlılığı algılanmadı." yazar. Bkz. [Çalışma zamanları](Runtimes).

<a id="preview-changes"></a>

## Değişiklikleri önizle

**Değişiklikleri önizle**, bir yüklemenin tam olarak ne yapacağını belirler ve hiçbir şeyi değiştirmez. Yeterli alan olup olmadığını denetler, paketi özel bir klasöre açar, değişiklikleri belirler ve o klasörü yeniden kaldırır. Sonuç kartının başlığı "Önizleme: Henüz hiçbir şey değiştirilmedi" olur; kartta **Planlanan değişiklikler** ve hizmetler, görevler ve kısayollar başta olmak üzere değişikliklerin bir listesi yer alır. Pencere ilk 300 değişikliği listeler, ardından kaç tane daha olduğunu söyler.

Önizleme her dosyanın karmasını denetlemez. Paketin tamamının karması pencere açıldığında denetlenmişti.

<a id="install"></a>

## Yükleme

**Yükle**, konum kullanılabilir olana ve liste o konuma göre belirlenene kadar soluk kalır. Paket o konuma hiç yüklenemiyorsa yanındaki bir satırda "Bu paket bu konuma yüklenemediği için Yükle kapalıdır." yazar ve neden, **Dosyalar dışında ayarlananlar** bölümünde kırmızı renkte gösterilir. Düğmeye tıkladığınızda:

1. Listeyi bir kez daha belirler. Siz baktıktan sonra yapacakları değiştiyse hiçbir şey yüklemez, yeni listeyi gösterir ve "Bunun burada yapacakları değişti, bu nedenle hiçbir şey yüklenmedi. Aşağıdaki listeyi gözden geçirin ve istediğiniz buysa Yükle düğmesini yeniden seçin." der.
2. Yazacağı her sürücüde yer olup olmadığını denetler.
3. Paketi yalnızca yöneticilerin kullanabildiği bir klasöre açar ve her dosya çıkarken karmasını denetler.
4. Yükler; geri alınabilmesi için her değişikliği önce bir [kayda](Glossary#journal) yazar.
5. Yüklediği her dosyayı geri okur ve karmasını denetler.
6. Kendiliğinden başlayacak şekilde ayarlanmış hizmetleri başlatır.

En altta ne yaptığını, varsa bir sayacı, geçen süreyi, bir ilerleme çubuğunu ve **İptal** düğmesini görürsünüz. Görebileceğiniz adımlar şunlardır: "Hazırlanıyor…", "Paket açılıyor", "Paket denetleniyor", "Tamamlanmamış önceki bir yükleme geri alınıyor", "Çalışma zamanları yükleniyor", "Dosyalar kopyalanıyor", "Hizmetler ayarlanıyor", "Sürücüler ayarlanıyor", "Kayıt defteri ayarları yazılıyor", "Lisans verileri geri yükleniyor", "Zamanlanmış görevler ayarlanıyor", "Kısayollar oluşturuluyor", "Ortam değişkenleri ayarlanıyor", "Uygulama Ayarlar'a ekleniyor" ve "Yüklenenler denetleniyor".

**İptal** önce "Geçerli adımdan sonra iptal ediliyor…", ardından "Değiştirilenler eski haline getiriliyor" gösterir. Yükleme sırasında pencereyi kapatmak da **İptal** ile aynı işi yapar: Her şey eski haline getirilene kadar pencere açık kalır. Henüz hiçbir şey değiştirilmemişse "İptal edildi. Hiçbir şey değiştirilmedi." yazar.

<a id="the-hash-check"></a>

## Karma denetimi

Yükleyici [karmaları](Glossary#hash) üç kez denetler:

1. **Pencere açıldığında**: paketin tamamının karması. Kesik bir kopya veya oluşturulduktan sonra düzenlenmiş bir kopya, içindeki herhangi bir şey okunmadan reddedilir.
2. **Paketi açarken**: her dosyanın karması, herhangi bir şey yerine yazılmadan önce.
3. **Yükledikten sonra**: Yüklediği her dosyayı geri okur ve yeniden denetler.

Karmalar hasarı ve karmaları da güncellemeyen herkesin yaptığı değişiklikleri yakalar. Bir yükleyiciyi kimin yaptığını kanıtlamazlar. Bunu yalnızca bir [imza](Glossary#signature) yapar.

<a id="the-result"></a>

## Sonuç

Sonuç kartı en üstte, "Yükleme sonucu" başlığıyla görünür.

| İleti | Anlamı |
| --- | --- |
| "Uygulama yüklendi ve karma denetimini geçti." | Her şey yolunda. Kurallar bir şeyi dışarıda bıraktıysa ileti "Uygulama, aşağıda dışarıda bırakıldığı belirtilenler olmadan yüklendi ve karma denetimini geçti." olur. |
| "Yükleme iptal edildi ve değiştirdikleri eski haline getirildi." | Siz iptal ettiniz. |
| "Yükleme başarısız oldu ve değiştirdikleri eski haline getirildi." | Bir şey başarısız oldu. Yüklediği çalışma zamanları dışında değiştirdikleri eski haline getirildi. Altında sarı renkte listelenen her şey olduğu gibi bırakıldı ve nedenini söyler; örneğin ancak bilgisayar yeniden başlatıldığında kaldırılabilen bir hizmet. |
| "Yükleme başarısız oldu ve değiştirdiklerinin tümü eski haline getirilemedi. Kalanlar aşağıda listeleniyor; bir sonraki yükleme bunları yeniden dener." | Listeyi okuyun. Bir yükleyiciyi yeniden çalıştırdığınızda bunları eski haline getirmeyi yeniden dener. |
| "Yükleme sorunsuz tamamlanmadı. Aşağıdaki sorunları gözden geçirin." | Altındaki iletileri okuyun. |

Ardından üç kutucuk gelir: **Yapılan değişiklikler** (başarısızlıktan sonra **Eski haline getirilen değişiklikler**), **Uyarılar** ve **Hatalar**. Etiketlerde **Paket karmaları geçerli** ve **Yüklenen dosyalar geçerli** ya da kırmızı renkte **GEÇERSİZ** yazar. En fazla beş uyarı ve tüm hatalar ileti olarak gösterilir. "Bir bağımlılığın tamamlanması için yeniden başlatma gerekiyor. Windows'u yeniden başlatın, ardından uygulamayı açın." iletisi, bir çalışma zamanının yeniden başlatma istediği anlamına gelir.

Sayıyla birlikte gösterilen **Değişiklikler**, yapılanların listesini (en fazla 300) açar. Yükleme sorunsuz tamamlanmadıysa [günlüğünün](Glossary#log) yolu **Günlük** olarak gösterilir.

Sorunsuz bir yüklemeden sonra alttaki düğmeler **Kapat** ve **Yükleme klasörünü aç** olur.

<a id="where-things-end-up"></a>

## Neler nereye gider

- **Program dosyaları** yükleme konumuna gider.
- **Kullanıcı verileri**, kutusu işaretliyken oturum açmış hesabın kendi AppData, Belgeler ve Kayıtlı Oyunlar klasörlerine ve ProgramData'ya gider. Bu bilgisayarın profilinizin dışında tuttuğu bir Belgeler veya Kayıtlı Oyunlar klasörüne yazılmaz. Bu dosyalar yükleme konumunun altında `_external` içine gider ve liste bunu en başta söyler. Kutunun işareti kaldırılmışsa tüm kullanıcı verileri yükleme konumunun altına gider.
- **Kısayollar** bu bilgisayarın masaüstüne ve Başlat menüsüne gider: Sizinkine ya da eskiden oradaysalar herkesinkine.
- **Path** değişkenine yalnızca uygulamanın girdileri eklenir. İçindeki başka hiçbir şey değişmez.
- **Burada başka bir uygulamanın zaten sahip olduğu şeyler o uygulamanın olarak kalır.** Başka bir şeye zaten ait olan bir dosya türüne, bağlantı türüne, COM sınıfına, hizmete, olay günlüğü kaynağına, yazı tipine veya benzeri bir şeye dokunulmaz ve yükleyici bunu söyler. Örneğin `http` bağlantılarını açmayı bekleyen bir uygulamanın Ayarlar'da yeniden varsayılan olarak ayarlanması gerekir.
- **Ayarlar** bölümüne uygulama için bir girdi eklenir; böylece onu diğer uygulamalar gibi kaldırabilirsiniz. Bkz. [Bir yüklemeyi geri alma](Undoing-an-Install).

<a id="common-reasons-it-refuses"></a>

## Sık görülen reddetme nedenleri

| Gördüğünüz | Neden | Ne yapmalı |
| --- | --- | --- |
| "bu uygulama bu bilgisayarda çalışamaz, bu nedenle hiçbir şey değiştirilmedi" | Programları başka türde bir işlemci için derlenmiş veya daha yeni bir Windows gerektiriyor | Eski bilgisayarla eşleşen bir bilgisayara yükleyin |
| "bunun için yeterli alan yok, bu nedenle hiçbir şey değiştirilmedi" | Yazacağı bir sürücüde yeterli boş alan yok | Yer açın veya başka bir sürücüde bir konum seçin |
| "başka bir yükleme veya kaldırma işlemi çalışıyor" | Aynı anda yalnızca biri çalışır | Bitmesini bekleyin, ardından yeniden deneyin |
| Bir dosya için "… yolu, bu Windows sürümünün dosya olarak değil aygıt olarak açtığı bir ad taşıyor" | Uygulamada `aux.h` gibi adlandırılmış, Windows 11'den önceki Windows sürümlerinin tutamadığı bir dosya var | Windows 11'e yükleyin |
| `%AppData%` gibi bir veri klasörü bir bağlantının altında | Yükleyici bir bağlantı üzerinden yazmaz | **Kullanıcı verileri** ve **Lisans** seçeneklerinin işaretini kaldırın ve onlar olmadan yükleyin |

Daha fazlası [Sorun giderme](Troubleshooting#installing) sayfasındadır.
