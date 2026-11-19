<a id="getting-started"></a>

# Primeiros passos

Esta página leva você do download até a primeira tela do aplicativo.

<a id="what-you-need"></a>

## O que você precisa

- **Windows 10 ou Windows 11, 64 bits.** O aplicativo é um programa x64. O Windows 11 em Arm o executa por meio de [emulação](Glossary#emulation). O Windows 10 em Arm não executa programas x64, e o Windows de 32 bits não consegue executá-lo de forma alguma. O Windows 10 e 11 para desktop em x64 não foram executados nos testes do projeto, que usam o Windows Server 2022 e 2025 e o Windows 11 em Arm. Consulte [Requisitos e testes](Requirements-and-Testing).
- **Direitos de administrador** no PC. O aplicativo sempre os solicita ao ser iniciado. Consulte [Direitos de administrador](Glossary#administrator-rights).
- **Direct3D 12**, que o Windows usa para desenhar a janela do aplicativo. O Windows 10 e 11 normalmente o têm. Um driver gráfico antigo ou uma máquina virtual pode não ter. Consulte [Direct3D 12](Glossary#direct3d-12).
- **Espaço em uma unidade** para o instalador que você criar. O aplicativo informa quanto antes de você começar.
- **Internet no PC antigo é opcional.** Se ela estiver disponível quando você criar o instalador, o aplicativo baixa os [runtimes](Glossary#runtime) da Microsoft que pode incluir e os coloca dentro do instalador. Consulte [Runtimes](Runtimes).

Mais detalhes, e o que foi e o que não foi testado, estão em [Requisitos e testes](Requirements-and-Testing).

<a id="download"></a>

## Download

Acesse a [versão mais recente](https://github.com/RandyNorthrup/WinAppMigrator/releases/latest). Ela tem três arquivos:

| Arquivo | O que é | Baixe-o se |
| --- | --- | --- |
| `WinAppMigrator-v0.7.3.exe` | O próprio aplicativo. Um único arquivo, sem nada para instalar. | Você só quer usá-lo. É o que a maioria das pessoas quer. |
| `WinAppMigrator-v0.7.3-windows-x64.zip` | O mesmo aplicativo, como `winappmigrator.exe`, com o README, o changelog, a licença e as licenças de terceiros | Você também quer a documentação |
| `SHA256SUMS.txt` | As somas de verificação dos dois arquivos acima | Você quer verificar o download |

A parte do instalador que vai dentro de cada instalador, o [stub do instalador](Glossary#installer-stub), nunca é publicada separadamente. Ela já está dentro do aplicativo.

<a id="check-the-download"></a>

## Verificar o download

Isto é opcional, e vale a pena. Cada verificação abaixo é independente. Execute-as no PowerShell, na pasta em que você salvou o arquivo.

**1. A soma de verificação.** Um [hash](Glossary#hash) é uma impressão digital de um arquivo. Se um único byte mudar, ele muda.

```powershell
Get-FileHash .\WinAppMigrator-v0.7.3.exe -Algorithm SHA256
Get-Content .\SHA256SUMS.txt
```

O número longo exibido por `Get-FileHash` deve ser igual ao da linha desse arquivo. `Get-FileHash` exibe letras maiúsculas, e o arquivo usa minúsculas. Não tem problema: compare as letras e os dígitos.

**2. A assinatura.** Uma [assinatura](Glossary#signature) diz quem criou o arquivo e que ele não foi alterado desde então.

```powershell
$signature = Get-AuthenticodeSignature .\WinAppMigrator-v0.7.3.exe
$signature.Status                     # should say Valid
$signature.SignerCertificate.Subject  # should include CN=Randy Northrup
```

Você também pode clicar com o botão direito do mouse no arquivo, escolher **Propriedades** e abrir a guia **Assinaturas Digitais**. A versão 0.5.0 é a primeira em que o aplicativo é assinado. Downloads de versões anteriores não são assinados.

**3. Onde ele foi compilado.** Cada versão a partir da 0.5.0 registra que o próprio fluxo de trabalho de lançamento do GitHub a compilou, a partir deste repositório. Esse registro se chama [atestado](Glossary#attestation). Se você tiver a [GitHub CLI](https://cli.github.com/), execute:

```powershell
gh attestation verify .\WinAppMigrator-v0.7.3.exe --repo RandyNorthrup/WinAppMigrator
```

O mesmo funciona para o zip.

<a id="start-it"></a>

## Iniciar o aplicativo

1. **Clique duas vezes** em `WinAppMigrator-v0.7.3.exe`. Não há nada para instalar. Ele é executado de onde quer que você o tenha salvo.
2. **SmartScreen.** O [SmartScreen](Glossary#smartscreen) do Windows pode mostrar uma tela “O Windows protegeu o computador” para um arquivo que ele não viu muitas vezes. Se isso acontecer, escolha **Mais informações** e verifique se o editor é Randy Northrup antes de escolher **Executar assim mesmo**.
3. **A solicitação de administrador.** O Windows pergunta se deseja permitir que o aplicativo faça alterações. Essa é a [solicitação do UAC](Glossary#uac-prompt). O aplicativo sempre pergunta, porque ler a unidade de outro PC, e arquivos que só administradores podem ler, exige direitos de administrador. No download assinado, a solicitação indica **Randy Northrup** como editor verificado. Se ela disser “Editor desconhecido” para este arquivo, ele não é o arquivo assinado da versão, então não o execute.
4. **A janela se abre.** Se, em vez disso, você vir uma mensagem dizendo que não foi possível abrir a janela e que é necessário o Direct3D 12, consulte [Solução de problemas](Troubleshooting#the-window-could-not-open). O aplicativo não tem como ser executado sem a janela.

<a id="first-run"></a>

## Primeira execução

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/app-pick-dark.png">
    <img alt="O aplicativo com o Fabrikam Data Sync selecionado" src="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/app-pick-light.png" width="860">
  </picture>
</p>

- A lista de aplicativos instalados fica à esquerda. Enquanto ela carrega, o pequeno rótulo no canto superior direito diz **Carregando** e, depois, quantos aplicativos encontrou, como **5 aplicativos**.
- A janela começa no modo claro ou escuro, conforme o Windows estiver. O botão de sol ou lua no canto superior direito alterna o modo, e o aplicativo se lembra da sua escolha. Consulte [Arquivos que ele mantém em um PC](Files-It-Keeps-on-a-PC). Em um tema de contraste do Windows, ele usa as cores do tema e não tem esse botão.
- Ele usa o idioma de exibição do Windows se for um dos 24 idiomas dele, e inglês caso contrário. **Idioma**, no canto superior direito, lista os idiomas pelo nome de cada um no próprio idioma e muda a janela na hora; o aplicativo se lembra da sua escolha. Árabe, persa, urdu e hebraico são exibidos da direita para a esquerda. Consulte [Idiomas e acessibilidade](Languages-and-Accessibility).
- Clique em um aplicativo. O lado direito mostra o que iria para o instalador dele e começa a calcular isso na hora. Nada é criado até você clicar em **Criar instalador**.

Próximo: [Criar um instalador](Building-an-Installer).
