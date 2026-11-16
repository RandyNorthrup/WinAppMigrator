<a id="getting-started"></a>

# 開始使用

本頁說明從下載到應用程式第一個畫面的過程。

<a id="what-you-need"></a>

## 需要的項目

- **Windows 10 或 Windows 11，64 位元**。此應用程式是 x64 程式。Windows 11 on Arm 會透過[模擬](Glossary#emulation)執行它。Windows 10 on Arm 無法執行 x64 程式，32 位元 Windows 則完全無法執行它。x64 上的桌面版 Windows 10 和 11 尚未在專案的測試中執行過，這些測試使用 Windows Server 2022 和 2025，以及 Windows 11 on Arm。請參閱[需求與測試](Requirements-and-Testing)。
- 這台電腦的**系統管理員權限**。應用程式每次啟動時都會要求這個權限。請參閱[系統管理員權限](Glossary#administrator-rights)。
- **Direct3D 12**，Windows 用它來繪製應用程式的視窗。Windows 10 和 11 通常都有。舊的顯示卡驅動程式或虛擬機器可能沒有。請參閱 [Direct3D 12](Glossary#direct3d-12)。
- **磁碟機空間**，用來存放您建置的安裝程式。開始之前，應用程式會告訴您需要多少空間。
- **舊電腦不一定要能連上網際網路**。如果建置時可以連線，應用程式會下載它可以包含的 Microsoft [執行階段](Glossary#runtime)，並放進安裝程式中。請參閱[執行階段](Runtimes)。

更多詳細資料，以及哪些項目已測試、哪些尚未測試，請參閱[需求與測試](Requirements-and-Testing)。

<a id="download"></a>

## 下載

前往[最新版本](https://github.com/RandyNorthrup/WinAppMigrator/releases/latest)。其中有三個檔案：

| 檔案 | 說明 | 適合下載的情況 |
| --- | --- | --- |
| `WinAppMigrator-v0.7.0.exe` | 應用程式本身。只有一個檔案，不需要安裝。 | 您只想使用它。大多數人需要的就是這個檔案。 |
| `WinAppMigrator-v0.7.0-windows-x64.zip` | 同一個應用程式（檔名為 `winappmigrator.exe`），另附 README、變更記錄、授權條款和第三方授權條款 | 您也想要這些文件資料 |
| `SHA256SUMS.txt` | 上面兩個檔案的總和檢查碼 | 您想檢查下載的檔案 |

每個安裝程式內部都有的安裝程式部分，也就是[安裝程式空殼](Glossary#installer-stub)，從不單獨發佈。它已經包含在應用程式中。

<a id="check-the-download"></a>

## 檢查下載的檔案

這是選擇性的步驟，但值得一做。下列每項檢查都各自獨立。請在 PowerShell 中，於您儲存檔案的資料夾內執行。

**1. 總和檢查碼**。[雜湊](Glossary#hash)就像檔案的指紋。只要有一個位元組改變，雜湊就會改變。

```powershell
Get-FileHash .\WinAppMigrator-v0.7.0.exe -Algorithm SHA256
Get-Content .\SHA256SUMS.txt
```

`Get-FileHash` 顯示的一長串數字，應該與檔案中該檔案那一行的數字相符。`Get-FileHash` 會以大寫字母顯示，而檔案中使用的是小寫字母。這沒有關係：只要比對字母和數字即可。

**2. 簽章**。[簽章](Glossary#signature)說明檔案是誰製作的，以及檔案在那之後沒有被變更。

```powershell
$signature = Get-AuthenticodeSignature .\WinAppMigrator-v0.7.0.exe
$signature.Status                     # should say Valid
$signature.SignerCertificate.Subject  # should include CN=Randy Northrup
```

您也可以用滑鼠右鍵按一下檔案，選擇**內容**，然後查看**數位簽章**索引標籤。0.5.0 版是第一個應用程式有簽署的版本。較早版本的下載檔案都沒有簽署。

**3. 建置的位置**。從 0.5.0 起的每個版本，都會記錄它是由 GitHub 本身的發行工作流程，從這個存放庫建置而成。這項記錄稱為[證明](Glossary#attestation)。如果您有 [GitHub CLI](https://cli.github.com/)，請執行：

```powershell
gh attestation verify .\WinAppMigrator-v0.7.0.exe --repo RandyNorthrup/WinAppMigrator
```

zip 檔也可以用相同的方式檢查。

<a id="start-it"></a>

## 啟動應用程式

1. **按兩下** `WinAppMigrator-v0.7.0.exe`。不需要安裝。它會直接從您儲存它的位置執行。
2. **SmartScreen**。對於不常見的檔案，Windows [SmartScreen](Glossary#smartscreen) 可能會顯示「Windows 已保護您的電腦」畫面。如果出現這個畫面，請選擇**其他資訊**，並在選擇**仍要執行**之前，確認發行者是 Randy Northrup。
3. **系統管理員提示**。Windows 會詢問是否允許此應用程式進行變更。這就是 [UAC 提示](Glossary#uac-prompt)。應用程式每次都會要求，因為讀取另一台電腦的磁碟機，以及只有系統管理員才能讀取的檔案，都需要系統管理員權限。如果是已簽署的下載檔案，提示會將 **Randy Northrup** 列為已驗證的發行者。如果此檔案的提示顯示「未知的發行者」，表示它不是已簽署的發行檔案，請不要執行。
4. **視窗開啟**。如果您看到的是無法開啟視窗、需要 Direct3D 12 的訊息，請參閱[疑難排解](Troubleshooting#the-window-could-not-open)。應用程式沒有視窗就無法執行。

<a id="first-run"></a>

## 第一次執行

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/app-pick-dark.png">
    <img alt="已選取 Fabrikam Data Sync 的應用程式畫面" src="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/app-pick-light.png" width="860">
  </picture>
</p>

- 已安裝的應用程式清單位於左側。清單載入時，右上角的小標籤會顯示**正在載入**，然後顯示找到的應用程式數目，例如 **5 個應用程式**。
- 視窗一開始會使用淺色或深色模式，與 Windows 目前的模式相同。右上角的太陽或月亮按鈕可以切換模式，應用程式會記住您的選擇。請參閱[它在電腦上保存的檔案](Files-It-Keeps-on-a-PC)。在 Windows 對比佈景主題中，它會使用佈景主題的色彩，而且沒有這個按鈕。
- 如果您的 Windows 顯示語言是它支援的 24 種語言之一，它就會使用該語言，否則使用英文。右上角的**語言**會以各語言本身的名稱列出這些語言，選擇後視窗會立即變更；應用程式會記住您的選擇。阿拉伯文、波斯文、烏爾都文和希伯來文會由右至左排列。請參閱[語言與協助工具](Languages-and-Accessibility)。
- 按一下應用程式。右側會顯示其安裝程式將包含的內容，並立即開始判斷。在您按一下**建立安裝程式**之前，不會建置任何內容。

下一步：[建置安裝程式](Building-an-Installer)。
