<a id="getting-started"></a>

# はじめに

このページでは、ダウンロードからアプリの最初の画面までを説明します。

<a id="what-you-need"></a>

## 必要なもの

- **Windows 10 または Windows 11（64 ビット）**。アプリは x64 プログラムです。Arm 版 Windows 11 では[エミュレーション](Glossary#emulation)によって実行されます。Arm 版 Windows 10 では x64 プログラムを実行できず、32 ビット版 Windows ではまったく実行できません。プロジェクトのテストでは Windows Server 2022 と 2025、および Arm 版 Windows 11 を使用しており、デスクトップ版の Windows 10 および 11（x64）では実行されていません。[要件とテスト](Requirements-and-Testing)を参照してください。
- PC の**管理者権限**。アプリは起動時に必ず管理者権限を求めます。[管理者権限](Glossary#administrator-rights)を参照してください。
- **Direct3D 12**。Windows はこれを使ってアプリのウィンドウを描画します。Windows 10 と 11 には通常含まれていますが、古いグラフィックス ドライバーや仮想マシンでは使用できない場合があります。[Direct3D 12](Glossary#direct3d-12) を参照してください。
- 作成するインストーラーを保存する**ドライブの空き領域**。必要な量は、作成を始める前にアプリに表示されます。
- **古い PC のインターネット接続は任意です**。作成時に接続されていれば、アプリは含めることができる Microsoft の[ランタイム](Glossary#runtime)をダウンロードして、インストーラーの中に入れます。[ランタイム](Runtimes)を参照してください。

詳細と、テスト済みの内容と未テストの内容については、[要件とテスト](Requirements-and-Testing)を参照してください。

<a id="download"></a>

## ダウンロード

[最新リリース](https://github.com/RandyNorthrup/WinAppMigrator/releases/latest)を開きます。3 つのファイルがあります。

| ファイル | 内容 | 次の場合に選択 |
| --- | --- | --- |
| `WinAppMigrator-v0.7.2.exe` | アプリ本体です。インストール不要の 1 つのファイルです。 | 使用するだけの場合。ほとんどの人にはこちらが適しています。 |
| `WinAppMigrator-v0.7.2-windows-x64.zip` | 同じアプリ（`winappmigrator.exe`）と、README、変更履歴、ライセンス、サードパーティのライセンス | 関連文書も必要な場合 |
| `SHA256SUMS.txt` | 上の 2 つのファイルのチェックサム | ダウンロードを確認したい場合 |

すべてのインストーラーの中に入るインストーラー部分である[インストーラー スタブ](Glossary#installer-stub)は、単独では公開されません。既にアプリの中に含まれています。

<a id="check-the-download"></a>

## ダウンロードの確認

これは任意ですが、行う価値があります。以下の確認は、それぞれ単独で行えます。ファイルを保存したフォルダーで、PowerShell を使用して実行してください。

**1. チェックサム**：[ハッシュ](Glossary#hash)は、ファイルの指紋のようなものです。1 バイトでも変わると、ハッシュも変わります。

```powershell
Get-FileHash .\WinAppMigrator-v0.7.2.exe -Algorithm SHA256
Get-Content .\SHA256SUMS.txt
```

`Get-FileHash` で表示される長い数値が、そのファイルの行にある数値と一致する必要があります。`Get-FileHash` は英字を大文字で表示し、ファイルには小文字で記載されていますが、問題ありません。英字と数字を比較してください。

**2. 署名**：[署名](Glossary#signature)は、誰がファイルを作成したか、そしてその後ファイルが変更されていないことを示します。

```powershell
$signature = Get-AuthenticodeSignature .\WinAppMigrator-v0.7.2.exe
$signature.Status                     # should say Valid
$signature.SignerCertificate.Subject  # should include CN=Randy Northrup
```

ファイルを右クリックして**プロパティ**を選択し、**デジタル署名**タブを確認することもできます。アプリが署名されている最初のリリースはバージョン 0.5.0 です。それより前のリリースのダウンロードは署名されていません。

**3. 作成された場所**：0.5.0 以降の各リリースには、このリポジトリから GitHub 自身のリリース ワークフローによって作成されたことが記録されています。この記録を[構成証明](Glossary#attestation)と呼びます。[GitHub CLI](https://cli.github.com/) がある場合は、次のコマンドを実行します。

```powershell
gh attestation verify .\WinAppMigrator-v0.7.2.exe --repo RandyNorthrup/WinAppMigrator
```

zip ファイルでも同じように確認できます。

<a id="start-it"></a>

## 起動

1. `WinAppMigrator-v0.7.2.exe` を**ダブルクリック**します。インストールは不要です。保存した場所からそのまま実行されます。
2. **SmartScreen**：あまり見かけないファイルの場合、Windows の [SmartScreen](Glossary#smartscreen) が「Windows によって PC が保護されました」という画面を表示することがあります。表示された場合は、**詳細情報**を選択し、発行元が Randy Northrup であることを確認してから、**実行**を選択してください。
3. **管理者の確認画面**：Windows は、アプリによる変更を許可するかどうかを確認します。これが [UAC プロンプト](Glossary#uac-prompt)です。別の PC のドライブや、管理者しか読み取れないファイルを読み取るには管理者権限が必要なため、アプリは必ず確認を求めます。署名済みのダウンロードでは、確認済みの発行元として **Randy Northrup** が表示されます。このファイルで「不明な発行元」と表示される場合は、署名済みのリリース ファイルではないため、実行しないでください。
4. **ウィンドウが開きます**。代わりに、ウィンドウを開けなかったことと Direct3D 12 が必要であることを示すメッセージが表示された場合は、[トラブルシューティング](Troubleshooting#the-window-could-not-open)を参照してください。アプリはウィンドウなしでは実行できません。

<a id="first-run"></a>

## 初回の実行

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/app-pick-dark.png">
    <img alt="Fabrikam Data Sync を選択したアプリ" src="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/app-pick-light.png" width="860">
  </picture>
</p>

- インストールされているアプリの一覧は左側にあります。読み込み中は、右上の小さなラベルに**読み込み中**と表示され、その後、見つかったアプリの数が **5 個のアプリ**のように表示されます。
- ウィンドウは、Windows に合わせてライト モードまたはダーク モードで起動します。右上の太陽または月のボタンで切り替えることができ、アプリはその選択を記憶します。[PC に保存されるファイル](Files-It-Keeps-on-a-PC)を参照してください。Windows のコントラスト テーマでは、テーマの色を使用し、このボタンは表示されません。
- Windows の表示言語が対応している 24 の言語のいずれかであればその言語で、そうでなければ英語で表示されます。右上の**言語**では、各言語がその言語自身の名前で一覧表示され、選択するとすぐにウィンドウが切り替わります。アプリはその選択を記憶します。アラビア語、ペルシア語、ウルドゥー語、ヘブライ語は右から左にレイアウトされます。[言語とアクセシビリティ](Languages-and-Accessibility)を参照してください。
- アプリをクリックします。右側に、そのインストーラーに含まれる内容が表示され、すぐにその確認が始まります。**インストーラーの作成**をクリックするまで、何も作成されません。

次へ：[インストーラーの作成](Building-an-Installer)。
