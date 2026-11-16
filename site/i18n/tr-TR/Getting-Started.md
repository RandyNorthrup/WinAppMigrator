<a id="getting-started"></a>

# Başlarken

Bu sayfa sizi indirmeden uygulamanın ilk ekranına kadar götürür.

<a id="what-you-need"></a>

## Gerekenler

- **Windows 10 veya Windows 11, 64 bit.** Uygulama bir x64 programıdır. Arm üzerinde Windows 11 onu [öykünme](Glossary#emulation) yoluyla çalıştırır. Arm üzerinde Windows 10 x64 programlarını çalıştıramaz; 32 bit Windows ise onu hiç çalıştıramaz. x64 üzerinde masaüstü Windows 10 ve 11, Windows Server 2022 ve 2025 ile Arm üzerinde Windows 11 kullanan proje testlerinde çalıştırılmadı. Bkz. [Gereksinimler ve testler](Requirements-and-Testing).
- **Bilgisayarda yönetici hakları.** Uygulama başlarken bunları her zaman ister. Bkz. [Yönetici hakları](Glossary#administrator-rights).
- **Direct3D 12**: Windows, uygulamanın penceresini çizmek için bunu kullanır. Windows 10 ve 11'de normalde bulunur. Eski bir grafik sürücüsünde veya bir sanal makinede bulunmayabilir. Bkz. [Direct3D 12](Glossary#direct3d-12).
- **Oluşturacağınız yükleyici için bir sürücüde yer.** Uygulama, başlamadan önce ne kadar yer gerektiğini size söyler.
- **Eski bilgisayarda internet isteğe bağlıdır.** Yükleyiciyi oluşturduğunuz sırada internet varsa uygulama, ekleyebileceği Microsoft [çalışma zamanlarını](Glossary#runtime) indirir ve yükleyicinin içine koyar. Bkz. [Çalışma zamanları](Runtimes).

Daha fazla ayrıntı ve nelerin test edilip nelerin edilmediği [Gereksinimler ve testler](Requirements-and-Testing) sayfasındadır.

<a id="download"></a>

## İndirme

[En son sürüme](https://github.com/RandyNorthrup/WinAppMigrator/releases/latest) gidin. Bu sürümde üç dosya vardır:

| Dosya | Nedir | Şu durumda alın |
| --- | --- | --- |
| `WinAppMigrator-v0.7.0.exe` | Uygulamanın kendisi. Tek bir dosyadır ve yüklenecek bir şey yoktur. | Yalnızca kullanmak istiyorsanız. Çoğu kişinin istediği budur. |
| `WinAppMigrator-v0.7.0-windows-x64.zip` | `winappmigrator.exe` olarak aynı uygulama; README, değişiklik günlüğü, lisans ve üçüncü taraf lisanslarıyla birlikte | Bu belgeleri de istiyorsanız |
| `SHA256SUMS.txt` | Yukarıdaki iki dosyanın sağlama toplamları | İndirdiğiniz dosyayı denetlemek istiyorsanız |

Her yükleyicinin içine giren yükleyici parçası, yani [yükleyici gövdesi](Glossary#installer-stub), hiçbir zaman tek başına yayımlanmaz. Zaten uygulamanın içindedir.

<a id="check-the-download"></a>

## İndirilen dosyayı denetleme

Bu isteğe bağlıdır, ama yapmaya değer. Aşağıdaki denetimlerin her biri diğerlerinden bağımsızdır. Bunları PowerShell'de, dosyayı kaydettiğiniz klasörde çalıştırın.

**1. Sağlama toplamı.** [Karma](Glossary#hash), bir dosyanın parmak izidir. Dosyanın tek bir baytı bile değişse karma da değişir.

```powershell
Get-FileHash .\WinAppMigrator-v0.7.0.exe -Algorithm SHA256
Get-Content .\SHA256SUMS.txt
```

`Get-FileHash` komutunun verdiği uzun sayı, `SHA256SUMS.txt` dosyasında o dosyanın satırındaki sayıyla eşleşmelidir. `Get-FileHash` büyük harf yazdırır, dosyada ise küçük harfler kullanılır. Bu sorun değildir: Harfleri ve rakamları karşılaştırın.

**2. İmza.** [İmza](Glossary#signature), dosyayı kimin yaptığını ve o zamandan beri değişmediğini gösterir.

```powershell
$signature = Get-AuthenticodeSignature .\WinAppMigrator-v0.7.0.exe
$signature.Status                     # should say Valid
$signature.SignerCertificate.Subject  # should include CN=Randy Northrup
```

Dosyaya sağ tıklayıp **Özellikler** komutunu seçerek **Dijital İmzalar** sekmesine de bakabilirsiniz. Uygulaması imzalı ilk sürüm 0.5.0'dır. Daha önceki sürümlerden indirilen dosyalar imzalı değildir.

**3. Nerede oluşturulduğu.** 0.5.0 ve sonraki her sürüm, onu GitHub'ın kendi yayın iş akışının bu depodan oluşturduğunu kaydeder. Bu kayda [kanıtlama](Glossary#attestation) (attestation) denir. [GitHub CLI](https://cli.github.com/) yüklüyse şunu çalıştırın:

```powershell
gh attestation verify .\WinAppMigrator-v0.7.0.exe --repo RandyNorthrup/WinAppMigrator
```

Aynısı zip dosyası için de geçerlidir.

<a id="start-it"></a>

## Başlatma

1. `WinAppMigrator-v0.7.0.exe` dosyasına **çift tıklayın**. Yüklenecek bir şey yoktur. Dosyayı nereye kaydettiyseniz oradan çalışır.
2. **SmartScreen.** Windows [SmartScreen](Glossary#smartscreen), sık görmediği bir dosya için "Windows kişisel bilgisayarınızı korudu" ekranını gösterebilir. Gösterirse **Ek bilgi** seçeneğini seçin ve **Yine de çalıştır** düğmesini seçmeden önce yayımcının Randy Northrup olduğunu denetleyin.
3. **Yönetici istemi.** Windows, uygulamanın değişiklik yapmasına izin verilip verilmeyeceğini sorar. Bu, [UAC istemidir](Glossary#uac-prompt). Uygulama bunu her zaman ister, çünkü başka bir bilgisayarın sürücüsünü ve yalnızca yöneticilerin okuyabildiği dosyaları okumak yönetici hakları gerektirir. İmzalı indirmede istem, doğrulanmış yayımcı olarak **Randy Northrup** adını gösterir. Bu dosya için "Bilinmeyen yayımcı" yazıyorsa dosya imzalı sürüm dosyası değildir; çalıştırmayın.
4. **Pencere açılır.** Bunun yerine pencerenin açılamadığını ve Direct3D 12 gerektiğini söyleyen bir ileti görürseniz bkz. [Sorun giderme](Troubleshooting#the-window-could-not-open). Uygulamanın penceresi olmadan çalışmasının bir yolu yoktur.

<a id="first-run"></a>

## İlk çalıştırma

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/app-pick-dark.png">
    <img alt="Fabrikam Data Sync seçiliyken uygulama" src="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/app-pick-light.png" width="860">
  </picture>
</p>

- Yüklü uygulamaların listesi soldadır. Liste yüklenirken sağ üstteki küçük etikette **Okunuyor** yazar, ardından **5 uygulama** gibi kaç uygulama bulduğu gösterilir.
- Pencere, Windows hangi moddaysa açık veya koyu modda başlar. Sağ üstteki güneş ya da ay düğmesi modu değiştirir ve uygulama seçiminizi hatırlar. Bkz. [Bir bilgisayarda tuttuğu dosyalar](Files-It-Keeps-on-a-PC). Bir Windows karşıtlık temasında temanın renklerini kullanır ve böyle bir düğmesi olmaz.
- Windows görüntüleme diliniz uygulamanın 24 dilinden biriyse o dili, değilse İngilizceyi konuşur. Sağ üstteki **Dil**, dilleri kendi adlarıyla listeler ve pencereyi hemen değiştirir; uygulama seçiminizi hatırlar. Arapça, Farsça, Urduca ve İbranice sağdan sola düzenlenir. Bkz. [Diller ve erişilebilirlik](Languages-and-Accessibility).
- Bir uygulamaya tıklayın. Sağ taraf, o uygulamanın yükleyicisine nelerin gireceğini gösterir ve bunu hemen belirlemeye başlar. **Yükleyici oluştur** düğmesine tıklayana kadar hiçbir şey oluşturulmaz.

Sonraki adım: [Yükleyici oluşturma](Building-an-Installer).
