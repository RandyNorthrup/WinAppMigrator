<a id="getting-started"></a>

# 시작하기

이 페이지에서는 다운로드부터 앱의 첫 화면까지 안내합니다.

<a id="what-you-need"></a>

## 필요한 항목

- **Windows 10 또는 Windows 11, 64비트.** 이 앱은 x64 프로그램입니다. Windows 11 on Arm에서는 [에뮬레이션](Glossary#emulation)을 통해 실행됩니다. Windows 10 on Arm에서는 x64 프로그램을 실행할 수 없고, 32비트 Windows에서는 이 앱을 전혀 실행할 수 없습니다. 데스크톱용 Windows 10 및 11(x64)은 프로젝트 테스트에서 실행되지 않았습니다. 테스트에는 Windows Server 2022 및 2025와 Windows 11 on Arm을 사용합니다. [요구 사항 및 테스트](Requirements-and-Testing)를 참조하세요.
- **PC의 관리자 권한.** 앱은 시작할 때 항상 관리자 권한을 요청합니다. [관리자 권한](Glossary#administrator-rights)을 참조하세요.
- **Direct3D 12.** Windows에서 앱 창을 그리는 데 사용합니다. Windows 10과 11에는 보통 들어 있지만, 오래된 그래픽 드라이버나 가상 머신에는 없을 수 있습니다. [Direct3D 12](Glossary#direct3d-12)를 참조하세요.
- **드라이브 공간.** 빌드할 설치 관리자를 저장할 공간이 필요합니다. 시작하기 전에 앱에서 필요한 공간을 알려 줍니다.
- **이전 PC의 인터넷 연결은 선택 사항입니다.** 빌드할 때 인터넷에 연결되어 있으면 앱이 포함할 수 있는 Microsoft [런타임](Glossary#runtime)을 다운로드하여 설치 관리자에 넣습니다. [런타임](Runtimes)을 참조하세요.

자세한 내용과 테스트한 항목, 테스트하지 않은 항목은 [요구 사항 및 테스트](Requirements-and-Testing)에 나와 있습니다.

<a id="download"></a>

## 다운로드

[최신 릴리스](https://github.com/RandyNorthrup/WinAppMigrator/releases/latest)로 이동하세요. 파일 세 개가 있습니다.

| 파일 | 설명 | 이런 경우에 받으세요 |
| --- | --- | --- |
| `WinAppMigrator-v0.7.2.exe` | 앱 자체입니다. 설치할 필요가 없는 파일 하나입니다. | 앱을 사용하기만 하려는 경우. 대부분의 사용자에게 필요한 파일입니다. |
| `WinAppMigrator-v0.7.2-windows-x64.zip` | 같은 앱(`winappmigrator.exe`)과 README, 변경 로그, 라이선스, 타사 라이선스 | 관련 문서도 함께 받으려는 경우 |
| `SHA256SUMS.txt` | 위 두 파일의 체크섬 | 다운로드한 파일을 확인하려는 경우 |

모든 설치 관리자 안에 들어가는 설치 관리자 부분인 [설치 관리자 스텁](Glossary#installer-stub)은 따로 게시되지 않습니다. 이미 앱 안에 들어 있습니다.

<a id="check-the-download"></a>

## 다운로드 확인

선택 사항이지만 해 볼 만합니다. 아래의 각 확인은 서로 독립적입니다. 파일을 저장한 폴더에서 PowerShell로 실행하세요.

**1. 체크섬.** [해시](Glossary#hash)는 파일의 지문입니다. 1바이트만 바뀌어도 해시가 바뀝니다.

```powershell
Get-FileHash .\WinAppMigrator-v0.7.2.exe -Algorithm SHA256
Get-Content .\SHA256SUMS.txt
```

`Get-FileHash`가 출력하는 긴 숫자가 `SHA256SUMS.txt`에서 그 파일의 줄에 있는 숫자와 일치해야 합니다. `Get-FileHash`는 대문자로 출력하고 `SHA256SUMS.txt`에는 소문자로 적혀 있습니다. 문제없습니다. 문자와 숫자만 비교하세요.

**2. 서명.** [서명](Glossary#signature)은 파일을 누가 만들었는지, 그리고 그 후로 파일이 변경되지 않았는지 알려 줍니다.

```powershell
$signature = Get-AuthenticodeSignature .\WinAppMigrator-v0.7.2.exe
$signature.Status                     # should say Valid
$signature.SignerCertificate.Subject  # should include CN=Randy Northrup
```

`Status`에는 Valid가 표시되어야 하고, `Subject`에는 CN=Randy Northrup이 들어 있어야 합니다. 파일을 마우스 오른쪽 단추로 클릭하고 **속성**을 선택한 다음 **디지털 서명** 탭을 확인할 수도 있습니다. 앱이 서명된 첫 릴리스는 버전 0.5.0입니다. 이전 릴리스의 다운로드 파일은 서명되어 있지 않습니다.

**3. 빌드된 곳.** 0.5.0 이후의 각 릴리스에는 GitHub 자체 릴리스 워크플로가 이 리포지토리에서 빌드했다는 기록이 남습니다. 이 기록을 [증명](Glossary#attestation)이라고 합니다. [GitHub CLI](https://cli.github.com/)가 있으면 다음을 실행하세요.

```powershell
gh attestation verify .\WinAppMigrator-v0.7.2.exe --repo RandyNorthrup/WinAppMigrator
```

zip 파일도 같은 방법으로 확인할 수 있습니다.

<a id="start-it"></a>

## 시작

1. `WinAppMigrator-v0.7.2.exe`를 **두 번 클릭**합니다. 설치할 필요가 없습니다. 저장한 위치에서 바로 실행됩니다.
2. **SmartScreen.** 자주 본 적이 없는 파일이면 Windows [SmartScreen](Glossary#smartscreen)에 "Windows의 PC 보호" 화면이 표시될 수 있습니다. 이 화면이 표시되면 **추가 정보**를 선택하고, 게시자가 Randy Northrup인지 확인한 다음 **실행**을 선택하세요.
3. **관리자 확인 프롬프트.** Windows에서 앱이 변경할 수 있도록 허용할지 묻습니다. 이것이 [UAC 프롬프트](Glossary#uac-prompt)입니다. 다른 PC의 드라이브와 관리자만 읽을 수 있는 파일을 읽으려면 관리자 권한이 필요하므로 앱은 항상 이 프롬프트를 표시합니다. 서명된 다운로드 파일이면 프롬프트에 확인된 게시자로 **Randy Northrup**이 표시됩니다. 이 파일에 대해 "알 수 없는 게시자"라고 표시되면 서명된 릴리스 파일이 아니므로 실행하지 마세요.
4. **창이 열립니다.** 창 대신 창을 열 수 없으며 Direct3D 12가 필요하다는 메시지가 표시되면 [문제 해결](Troubleshooting#the-window-could-not-open)을 참조하세요. 이 앱은 창 없이 실행할 수 없습니다.

<a id="first-run"></a>

## 처음 실행

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/app-pick-dark.png">
    <img alt="Fabrikam Data Sync를 선택한 앱" src="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/app-pick-light.png" width="860">
  </picture>
</p>

- 설치된 앱 목록은 왼쪽에 있습니다. 목록을 로드하는 동안 오른쪽 위의 작은 레이블에 **로드 중**이 표시되고, 그다음에는 찾은 앱 수가 **앱 5개**와 같이 표시됩니다.
- 창은 Windows의 모드에 따라 밝은 모드 또는 어두운 모드로 시작됩니다. 오른쪽 위의 해 또는 달 단추로 전환할 수 있으며, 앱이 선택을 기억합니다. [PC에 보관되는 파일](Files-It-Keeps-on-a-PC)을 참조하세요. Windows 대비 테마에서는 테마 색을 사용하며 이 단추가 없습니다.
- Windows 표시 언어가 앱이 지원하는 24개 언어 중 하나이면 그 언어로 표시되고, 그렇지 않으면 영어로 표시됩니다. 오른쪽 위의 **언어**에는 각 언어가 해당 언어 이름으로 나열되며, 선택하면 창이 바로 바뀝니다. 앱은 선택을 기억합니다. 아랍어, 페르시아어, 우르두어, 히브리어는 오른쪽에서 왼쪽으로 배치됩니다. [언어 및 접근성](Languages-and-Accessibility)을 참조하세요.
- 앱을 클릭하세요. 오른쪽에 그 앱의 설치 관리자에 들어갈 항목이 표시되며, 바로 확인을 시작합니다. **설치 관리자 만들기**를 클릭할 때까지는 아무것도 빌드되지 않습니다.

다음: [설치 관리자 빌드](Building-an-Installer).
