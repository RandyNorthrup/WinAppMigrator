<a id="getting-started"></a>

# 入门

本页带你从下载开始，一直到看到 WinAppMigrator 的第一个屏幕。

<a id="what-you-need"></a>

## 所需条件

- **Windows 10 或 Windows 11，64 位**。WinAppMigrator 是 x64 程序。基于 Arm 的 Windows 11 通过[仿真](Glossary#emulation)运行它。基于 Arm 的 Windows 10 无法运行 x64 程序，32 位 Windows 则完全无法运行它。项目的测试使用 Windows Server 2022 和 2025 以及基于 Arm 的 Windows 11，尚未在 x64 上的桌面版 Windows 10 和 11 中运行过。请参阅[要求和测试](Requirements-and-Testing)。
- 电脑上的**管理员权限**。WinAppMigrator 每次启动时都会请求管理员权限。请参阅[管理员权限](Glossary#administrator-rights)。
- **Direct3D 12**，Windows 用它来绘制 WinAppMigrator 的窗口。Windows 10 和 11 通常都有。旧版显卡驱动程序或虚拟机可能没有。请参阅 [Direct3D 12](Glossary#direct3d-12)。
- 驱动器上要有足够的**空间**来存放你生成的安装程序。WinAppMigrator 会在开始之前告诉你需要多少空间。
- **旧电脑上不一定要有 Internet 连接**。如果生成时能联网，WinAppMigrator 会下载它可以包含的 Microsoft [运行时](Glossary#runtime)，并将它们放入安装程序中。请参阅[运行时](Runtimes)。

更多详细信息，以及哪些内容已经测试、哪些尚未测试，请参阅[要求和测试](Requirements-and-Testing)。

<a id="download"></a>

## 下载

请转到[最新版本](https://github.com/RandyNorthrup/WinAppMigrator/releases/latest)。其中有三个文件：

| 文件 | 它是什么 | 适用情况 |
| --- | --- | --- |
| `WinAppMigrator-v0.7.3.exe` | WinAppMigrator 本身。只有一个文件，无需安装。 | 你只是想使用它。大多数人需要的就是这个文件。 |
| `WinAppMigrator-v0.7.3-windows-x64.zip` | 同一个程序，文件名为 `winappmigrator.exe`，附带 README、更改日志、许可证和第三方许可证 | 你还想要这些附带文档 |
| `SHA256SUMS.txt` | 上面两个文件的校验和 | 你想检查下载的文件 |

装入每个安装程序的那部分安装程序，即[安装程序存根](Glossary#installer-stub)，从不单独发布。它已经包含在 WinAppMigrator 中。

<a id="check-the-download"></a>

## 检查下载的文件

这一步是可选的，但值得一做。下面的每项检查都相互独立。请在 PowerShell 中，在保存该文件的文件夹里运行它们。

**1. 校验和**。[哈希](Glossary#hash)就是文件的指纹。哪怕只改动一个字节，它也会改变。

```powershell
Get-FileHash .\WinAppMigrator-v0.7.3.exe -Algorithm SHA256
Get-Content .\SHA256SUMS.txt
```

`Get-FileHash` 输出的长数字应当与该文件那一行中的数字一致。`Get-FileHash` 输出的是大写字母，而该文件使用小写字母。这没有关系：只需比较字母和数字。

**2. 签名**。[签名](Glossary#signature)说明文件是谁制作的，以及文件此后没有被更改过。

```powershell
$signature = Get-AuthenticodeSignature .\WinAppMigrator-v0.7.3.exe
$signature.Status                     # should say Valid
$signature.SignerCertificate.Subject  # should include CN=Randy Northrup
```

你也可以右键单击该文件，选择**属性**，然后查看**数字签名**选项卡。0.5.0 版是第一个对应用进行签名的版本。更早版本的下载文件未签名。

**3. 在哪里生成**。从 0.5.0 起，每个版本都会记录它是由 GitHub 自己的发布工作流从此存储库生成的。这一记录称为[证明](Glossary#attestation)。如果你装有 [GitHub CLI](https://cli.github.com/)，请运行：

```powershell
gh attestation verify .\WinAppMigrator-v0.7.3.exe --repo RandyNorthrup/WinAppMigrator
```

对 zip 文件也可以这样检查。

<a id="start-it"></a>

## 启动

1. **双击** `WinAppMigrator-v0.7.3.exe`。无需安装。它在你保存它的位置直接运行。
2. **SmartScreen**。对于不常见的文件，Windows [SmartScreen](Glossary#smartscreen) 可能会显示“Windows 已保护你的电脑”屏幕。如果出现此屏幕，请选择**更多信息**，确认发布者是 Randy Northrup，然后再选择**仍要运行**。
3. **管理员提示**。Windows 会询问是否允许此应用进行更改。这就是 [UAC 提示](Glossary#uac-prompt)。WinAppMigrator 每次都会请求，因为读取另一台电脑的驱动器以及只有管理员才能读取的文件都需要管理员权限。对于已签名的下载文件，提示中会将 **Randy Northrup** 显示为已验证的发布者。如果此文件的提示显示“未知发布者”，那么它不是已签名的发布文件，请不要运行它。
4. **窗口打开**。如果你看到的是一条消息，说无法打开窗口、需要 Direct3D 12，请参阅[疑难解答](Troubleshooting#the-window-could-not-open)。WinAppMigrator 没有窗口就无法运行。

<a id="first-run"></a>

## 首次运行

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/app-pick-dark.png">
    <img alt="已选中 Fabrikam Data Sync 的 WinAppMigrator" src="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/app-pick-light.png" width="860">
  </picture>
</p>

- 已安装的应用列在左侧。加载期间，右上方的小标签显示**正在加载**，然后显示找到的应用数，例如 **5 个应用**。
- 窗口启动时采用浅色或深色模式，与 Windows 当前的模式一致。右上方的太阳或月亮按钮可以切换模式，WinAppMigrator 会记住你的选择。请参阅[它在电脑上保留的文件](Files-It-Keeps-on-a-PC)。在 Windows 对比度主题下，它使用该主题的颜色，并且没有这个按钮。
- 如果你的 Windows 显示语言是它支持的 24 种语言之一，它就使用该语言，否则使用英语。右上方的**语言**以各语言自己的名称列出这些语言，选择后窗口会立即切换；WinAppMigrator 会记住你的选择。阿拉伯语、波斯语、乌尔都语和希伯来语采用从右到左的布局。请参阅[语言和辅助功能](Languages-and-Accessibility)。
- 单击一个应用。右侧会显示其安装程序中将包含的内容，并立即开始确定这些内容。在你单击**创建安装程序**之前，不会生成任何内容。

下一步：[生成安装程序](Building-an-Installer)。
