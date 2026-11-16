<a id="troubleshooting"></a>

# Solução de problemas

Encontre abaixo o que você está vendo, pelo que acontece ou pelas palavras na tela. As palavras entre aspas são o que o programa mostra, às vezes com um nome ou um número no meio.

Se nada aqui ajudar, consulte [Como obter ajuda](#getting-help) no final.

<a id="the-window-could-not-open"></a>

## Não foi possível abrir a janela

**O que você vê.** Uma caixa de mensagem em vez da janela:

- do aplicativo: “Não foi possível abrir a janela” e “É necessário o Direct3D 12 para desenhar esta janela, e um driver gráfico antigo ou uma máquina virtual pode não oferecê-lo.”
- de um instalador: o mesmo, e “Ainda é possível instalar sem ela: execute este arquivo em um prompt de comando de administrador com --apply e, para escolher onde, com --target-root seguido de uma pasta.”
- de um desinstalador: o mesmo, e “Ainda é possível desinstalar sem ela, em um prompt de comando de administrador:” com o comando exato.

**Por quê.** As janelas são desenhadas com o [Direct3D 12](Glossary#direct3d-12), e não há outra forma de desenhá-las.

**O que fazer.**

- Atualize o driver gráfico do PC.
- Para um instalador ou um desinstalador, use a linha de comando. Consulte [Linha de comando](Command-Line).
- O aplicativo não tem linha de comando. Crie o instalador em outro PC: você pode conectar a unidade do PC antigo a ele. Consulte [Transferir de uma unidade conectada](Moving-from-a-Connected-Drive). Esse PC também pode ler o antigo pela rede. Consulte [Transferir de um PC pela rede](Moving-over-the-Network).

<a id="starting-the-app"></a>

## Ao iniciar o aplicativo

| O que você vê | O que significa | O que fazer |
| --- | --- | --- |
| A solicitação do UAC diz “Editor desconhecido” para o aplicativo | A partir da 0.5.0, o aplicativo é assinado. Um aplicativo sem assinatura não é o arquivo da versão, ou é anterior à 0.5.0. | Baixe-o novamente da [versão mais recente](https://github.com/RandyNorthrup/WinAppMigrator/releases/latest) e [verifique-o](Getting-Started#check-the-download). |
| O SmartScreen diz “O Windows protegeu o computador” | O SmartScreen não viu esse arquivo muitas vezes | Escolha **Mais informações**, verifique se o editor é Randy Northrup e depois escolha **Executar assim mesmo**. |
| O rótulo na parte superior diz **Falha ao carregar**, e a lista diz “Não foi possível listar os aplicativos” | Não foi possível ler a lista de aplicativos | Clique na seta circular para recarregar. No caso de uma unidade conectada, verifique se ela ainda está conectada. |
| O aplicativo que você quer não está na lista | É um aplicativo da Microsoft Store, uma parte oculta de outro aplicativo, ou não tem nenhuma entrada em Configurações, como um aplicativo portátil | Aplicativos da Store não podem ser transferidos. Uma parte oculta pode ser oferecida em **Também pode fazer parte dele** quando você escolher o aplicativo principal. Um aplicativo sem entrada em Configurações não pode ser escolhido. |
| “Nenhum aplicativo correspondente” | A pesquisa ou o filtro oculta tudo | Limpe a caixa de pesquisa e escolha **Todos**. |

<a id="building"></a>

## Criação do instalador

<a id="picking-a-drive-or-a-disk-image"></a>

### Escolher uma unidade ou uma imagem de disco

| O que você vê | O que fazer |
| --- | --- |
| A unidade “não parece ser uma unidade do sistema Windows (não tem \Windows nem \Users). Escolha a raiz da unidade do outro PC.” | Escolha o nível superior da unidade, como `E:\`, não uma pasta nela. Verifique se é a unidade em que o Windows estava instalado. |
| Uma imagem “não é uma imagem de disco que o Windows consiga anexar (.vhd ou .vhdx)” | Escolha um arquivo `.vhd` ou `.vhdx`. Salve antes um disco do VMware ou do VirtualBox em um desses formatos. Para um ponto de verificação do Hyper-V, `.avhdx`, exporte o disco da máquina virtual. |
| “Não foi possível anexar” uma imagem: “o Windows já está com ela anexada, e o que a anexou pode estar gravando nela.” | Ela está aberta em outro lugar, por exemplo montada no Explorador de Arquivos. Escolha a unidade dela em **Unidade conectada…**, ou desanexe-a e escolha-a novamente. |
| Uma imagem “não tem nenhum volume legível com o Windows. Um volume criptografado com o BitLocker precisa ser desbloqueado primeiro.” | Verifique se é uma imagem da unidade em que o Windows estava instalado. Se ela estiver bloqueada com o [BitLocker](Glossary#bitlocker), monte-a você mesmo no Windows, desbloqueie-a e escolha a unidade dela em **Unidade conectada…**. |
| Uma imagem “é um disco de diferenciação, que só contém o que mudou desde o disco a partir do qual foi criado” | Mescle-a no disco a partir do qual ela foi criada, com Editar Disco do Hyper-V, e escolha esse disco. |
| “Não foi possível anexar” uma imagem, com outro motivo | O motivo é do próprio Windows. Leia-o, verifique se nada mais está usando a imagem e tente novamente. |

<a id="reading-a-pc-over-the-network"></a>

### Ler um PC pela rede

A lista completa, incluindo o que o cliente diz no PC antigo, está em [Transferir de um PC pela rede](Moving-over-the-Network#if-something-goes-wrong).

| O que você vê | O que fazer |
| --- | --- |
| O PC antigo não aparece em **2. Escolha o PC antigo** | Verifique se o cliente está em execução nele e mostra um código, e se os dois PCs estão na mesma rede. Depois, digite o nome ou o endereço dele. Se houver um roteador entre eles, digite o endereço. Um firewall que não seja o do próprio Windows precisa permitir a passagem de TCP 47732 e UDP 47731. |
| Um nome “não foi encontrado na rede” | Digite o endereço do PC antigo. |
| “não foi possível acessar o PC antigo:” e um motivo | Verifique se o cliente ainda está em execução e se o nome ou o endereço está correto. |
| “esse não é o código que o PC antigo mostra. Verifique-o e tente novamente.” | Digite o código que o cliente mostra agora. Depois de cinco códigos errados, ele gera um novo. Depois de 20, ele não permite nenhum emparelhamento até ser iniciado novamente. |
| No PC antigo, o Windows diz que o editor do cliente é desconhecido | Isso só é esperado em um Windows 7 sem o certificado raiz de 2020 da Microsoft, que não consegue verificar a assinatura do cliente. Nesse caso, escolha **Sim** somente para um cliente que você mesmo salvou a partir do aplicativo. Em qualquer outro caso, a solicitação deve indicar Randy Northrup. Se não indicar, não execute essa cópia. |
| O aplicativo não consegue mais ler o PC antigo, depois de uma longa pausa | O cliente encerra uma conexão à qual o aplicativo não fez nenhum pedido por 30 minutos. Clique em **Desconectar** e emparelhe novamente. |

<a id="looking-at-the-app"></a>

### Analisar o aplicativo

| O que você vê | O que significa | O que fazer |
| --- | --- | --- |
| “Não foi possível analisar este aplicativo:” e um motivo | A análise falhou. Um motivo comum é uma entrada sem nenhum programa, como um pacote de driver. | Clique em **Tentar novamente**. Se falhar da mesma forma, a entrada talvez não seja um aplicativo que o WinAppMigrator consiga transferir. |
| “Interrompido. Um instalador pode ser criado depois que este aplicativo for analisado.” | Você clicou em **Parar** | Clique em **Analisar novamente**. |
| A análise demora muito | Um aplicativo grande, uma unidade lenta ou com falha, ou muitas contas | Se a unidade puder estar falhando, clique em **Parar**, crie uma [imagem de disco](Glossary#disk-image) dela e escolha a imagem, pois a leitura de uma unidade com falha pode acabar de danificá-la. A partir da 0.6.0, escolha-a com **Imagem de disco…**. Na 0.5.0, monte-a e escolha a unidade dela. Caso contrário, aguarde. Enquanto isso, você pode escolher outro aplicativo. |
| Uma linha que você quer está esmaecida e marcada | Ela contém o programa principal, ou o que ele precisa para iniciar | Ela sempre entra. Consulte [as linhas que você não pode desmarcar](Building-an-Installer#the-groups). |
| Uma linha diz “Também deixado de fora:” | Algo que você desmarcou a levou junto | Marque esse item novamente, se você quiser esta linha. |
| “O que sobrou não formaria um pacote” | O que você desmarcou não deixa nada que possa ser criado | Marque algo novamente. |
| “Os dados de licença no que você deixou de fora ainda entram” | As licenças seguem somente a opção **Dados de licença e ativação** | Desmarque essa opção se quiser deixá-las de fora. |

<a id="create-installer-stays-greyed-out"></a>

### Criar instalador continua esmaecido

Ele aguarda até que tudo isto seja verdade:

- a análise do aplicativo terminou, para as opções como estão agora (“Calculando o que vai junto com isso…” significa que ele ainda está calculando o que o último item que você desmarcou leva junto);
- a **Pasta de saída** é um caminho completo, como `D:\Installers` (“Digite o caminho completo de uma pasta, como D:\Instaladores.”);
- há espaço nessa unidade.

<a id="room"></a>

### Espaço

| O que você vê | O que fazer |
| --- | --- |
| “Não há espaço suficiente.” e quanto uma unidade precisa e tem | Escolha uma pasta de saída em outra unidade ou deixe algo de fora. O valor é o máximo que uma criação poderia ocupar, antes da compactação, então pode ser maior que o instalador. |
| Uma unidade “não informou quanto espaço tem” | A criação continua sem verificar essa unidade. Verifique se ela tem espaço. |
| Uma unidade FAT32 e um aplicativo grande | O FAT32 não consegue armazenar um arquivo de 4 GiB. Use uma unidade NTFS ou exFAT. |

<a id="the-done-screen"></a>

### A tela de conclusão

| O que você vê | O que fazer |
| --- | --- |
| “Instalador criado, mas” alguns arquivos “deixaram de existir desde que o aplicativo foi analisado” | Clique em **Analisar novamente** e depois em **Criar instalador novamente**, se esses arquivos importarem. |
| “Instalador criado, mas não foi possível capturar” alguns arquivos, “geralmente porque eles estavam bloqueados ou ilegíveis.” | Feche o aplicativo e tudo o que estiver usando os arquivos dele e crie o instalador novamente. **Avisos** lista quais arquivos. |
| “Instalador criado, mas ele ainda revela algo sobre a origem dele.” | Leia as mensagens em âmbar abaixo dele. Consulte [a última verificação](Building-an-Installer#the-last-look). |
| **Avisos** diz “Não foi possível baixar” um runtime, e “o instalador tentará usar o winget no destino.” | Esse runtime não está dentro do instalador, então o PC novo precisará de internet para ele. Crie o instalador novamente enquanto estiver online. Consulte [Runtimes](Runtimes#at-build-time). |
| “Não use este instalador: ele falhou na verificação de hash.” | Não o use. Crie-o novamente. Se isso acontecer de novo, a unidade pode estar falhando. |
| “O instalador foi gravado, mas a verificação de hash dele não pôde ser executada” | Não o use. Crie-o novamente. |
| “Criação cancelada. Nenhum instalador foi gravado.” | Você clicou em **Cancelar**. |
| “Não foi possível criar o instalador.” e um motivo | Leia o motivo. **Copiar erro** o copia, e **Log** mostra onde está o log. |
| “Não foi possível criar a pasta de saída” | Escolha uma pasta em que você possa gravar. |
| “Não foi possível criar uma pasta de preparação protegida, então nada foi criado” | A criação não conseguiu criar a pasta de trabalho privada dela ao lado da pasta de saída nem na pasta temporária do Windows. Escolha uma pasta de saída em outra unidade. |
| “A criação parou inesperadamente” | Algo deu errado de um jeito que nada previa. “Se ela deixou um instalador na pasta de saída, não o use.” [Relate o problema](#getting-help). |

<a id="installing"></a>

## Instalação

<a id="when-the-installer-opens"></a>

### Quando o instalador abre

| O que você vê | O que significa | O que fazer |
| --- | --- | --- |
| “Verificando o instalador…” por muito tempo | Ele lê o pacote inteiro para verificar o hash | Aguarde. Um instalador grande demora um pouco. |
| “Não foi possível ler o instalador:” e “este instalador está danificado ou foi alterado depois de criado, então nada foi lido do pacote dele. Copie-o novamente do local onde ele foi criado.” | O arquivo está incompleto ou foi alterado | Copie-o novamente. |
| “este instalador guarda o pacote em um arquivo chamado” um arquivo `.wamdata`, “na mesma pasta, e esse arquivo não está lá” | Falta o arquivo de dados de um instalador de dois arquivos | Copie o arquivo `.wamdata` para a mesma pasta. Consulte [Aplicativos grandes](Big-Apps). |
| o arquivo de dados “não é o arquivo de dados deste instalador, ou está danificado ou foi alterado depois de criado” | O arquivo de dados não corresponde | Copie os dois arquivos novamente, do local onde eles foram criados. |
| “O pacote deste instalador não pode ser usado neste PC:” e “este aplicativo não pode ser executado neste PC, então nada foi alterado” | Os programas do aplicativo foram compilados para outro processador, ou um programa diz que precisa de um Windows mais recente | Instale em um PC parecido com o antigo. Nada foi alterado. |
| “Este é um stub de instalador do WinAppMigrator e não contém nenhum pacote de aplicativo.” | Este é o stub vazio, não um instalador criado | Crie um instalador com o aplicativo e execute esse instalador. |
| “A assinatura deste instalador não é válida”, com o motivo do Windows | Ele foi assinado, mas o Windows não aceita a assinatura. Ele pode ter sido alterado desde então, ou o certificado dele não é confiável neste PC. | Não o instale. Fale com quem o criou. |
| A solicitação do UAC diz “Editor desconhecido” | O instalador não está assinado. Isso é normal para um instalador que você criou. | Execute-o somente se você confiar na origem dele. |

<a id="the-install-location"></a>

### O local de instalação

Quando **Instalar** e **Visualizar alterações** estão esmaecidos, um aviso em vermelho abaixo de **Local de instalação** diz por quê. O motivo mais comum é que o aplicativo já está instalado nessa pasta neste PC, como costuma acontecer com um navegador da Web, um player de mídia ou o Office: primeiro desinstale essa cópia, em Configurações do Windows, em **Aplicativos**, e abra o instalador novamente, ou digite uma pasta nova e vazia. Os motivos, e o que fazer, estão em [Instalação no PC novo](Installing-on-the-New-PC#when-the-location-cannot-be-used).

<a id="before-or-during-the-install"></a>

### Antes ou durante a instalação

| O que você vê | O que significa | O que fazer |
| --- | --- | --- |
| “Talvez não haja espaço suficiente aqui” | A unidade parece ter pouco espaço | Libere espaço ou escolha um local em outra unidade. |
| “não há espaço suficiente para isto, então nada foi alterado” | Uma unidade em que ele grava está cheia demais | Libere espaço nela ou escolha um local de instalação em outra unidade. |
| “O que isto faria aqui mudou, então nada foi instalado.” | Algo no PC mudou depois que a lista foi calculada | Leia a lista novamente e clique em **Instalar** de novo se for o que você deseja. |
| “Uma instalação anterior de” um aplicativo “não foi concluída. Ao instalar, ela é desfeita primeiro.” | Uma instalação anterior foi interrompida | Clique em **Instalar**. Ela desfaz aquela primeiro. |
| “outra instalação ou desinstalação está em execução. Aguarde até que ela termine e tente novamente” | Só uma é executada por vez | Aguarde e tente novamente. |
| Um arquivo “tem um nome que esta versão do Windows abre como dispositivo, e não como arquivo” | O aplicativo tem um arquivo com um nome como `aux.h`, que o Windows anterior ao Windows 11 não consegue armazenar | Instale no Windows 11. |
| “Não foi possível criar uma pasta de preparação” | Não há nenhum lugar privado, com espaço, para descompactar o pacote | Libere espaço na unidade do local de instalação ou na unidade do sistema. |
| Uma pasta de dados como `%AppData%` é indicada como um link | O instalador não grava por meio de um [link](Glossary#link) | Desmarque **Dados do usuário** e **Licença** e instale sem eles. |
| “A pasta ProgramData deste PC não foi encontrada, então nada foi instalado” | A informação do próprio Windows sobre onde fica ProgramData não pôde ser lida | O instalador não instala sem um lugar seguro para guardar o histórico da instalação. |

<a id="the-result"></a>

### O resultado

| O que você vê | O que fazer |
| --- | --- |
| “A instalação falhou, e o que ela alterou foi revertido.” | Leia as mensagens em vermelho para saber por quê. O PC está como estava, exceto por qualquer runtime que tenha sido instalado. |
| “A instalação falhou, e nem tudo o que ela alterou pôde ser revertido.” | Leia o que está listado. Feche o que possa estar usando esses arquivos e execute um instalador novamente: ele tenta reverter isso primeiro. |
| “Não foi possível instalar” um runtime | Não foi possível instalar o runtime, então a instalação foi revertida. No caso do DirectX End-User Runtime, o PC novo precisa de internet. Consulte [Runtimes](Runtimes#with-no-internet-on-the-new-pc). |
| “Uma dependência precisa que o PC seja reiniciado para concluir. Reinicie o Windows e abra o aplicativo.” | Reinicie e depois inicie o aplicativo. |
| O serviço “era executado como” uma conta “na origem. A senha dele não está no instalador, então o serviço é configurado como LocalService.” | Abra Serviços e defina a conta e a senha do serviço, se ele não funcionar como está. |
| Alguns arquivos vão, “em vez disso, para” a pasta de instalação, “em uma pasta chamada _external” | Este PC mantém Documentos ou Jogos Salvos fora do seu perfil. Mova você mesmo esses arquivos para essa pasta. |
| Algo “foi deixado de fora”, com um motivo | As regras o recusaram, ou ele já pertence a outro aplicativo aqui. O restante é instalado. Se o aplicativo precisar dele, veja o motivo. |
| **Hashes do pacote INVÁLIDOS** ou **Arquivos instalados INVÁLIDOS** | Algo não correspondeu. Copie o instalador novamente e tente mais uma vez. |

<a id="after-installing"></a>

### Após a instalação

| O que você vê | O que fazer |
| --- | --- |
| O aplicativo não inicia | Verifique em **Dependências** se há um runtime marcado como **Instalar manualmente** e instale-o. Verifique os avisos no resultado. |
| Um aplicativo do Windows 7 não inicia, e um aviso na criação dizia, sobre uma DLL, que “o Windows 10 e 11 não vêm com ela” | Essa DLL foi colocada nas pastas do próprio Windows antigo por outro programa, então não veio junto. Instale o que a fornece e inicie o aplicativo novamente. Consulte [Aplicativos do Windows 7, Vista, 8 ou 8.1](Moving-from-a-Connected-Drive#apps-from-windows-7-vista-8-or-81). |
| O aplicativo inicia como se fosse novo | Isso é esperado com um instalador criado com **Dados do usuário** desativado. Também acontece quando **Dados do usuário** foi desmarcado na instalação, porque os dados vão então para a pasta de instalação, e quando o instalador não conseguiu determinar quem estava conectado, o que ele informa. |
| O aplicativo pede ativação | Uma licença vinculada ao PC antigo precisa ser ativada novamente. |
| O atualizador do próprio aplicativo acha que ele não está instalado | O Windows Installer não tem registro dele aqui. Consulte [Limitações](Limitations). |
| Um tipo de arquivo ou tipo de link não abre com o aplicativo | Outro aplicativo já era dono dele aqui, então ele foi deixado como estava. Escolha o aplicativo em Configurações, em Aplicativos padrão. |
| Uma nova entrada de `Path` não é vista | Abra um novo PowerShell ou prompt de comando. Alguns programas só veem uma alteração depois que você sai e entra novamente. |

<a id="uninstalling"></a>

## Desinstalação

| O que você vê | O que fazer |
| --- | --- |
| “Parte dele está em uma unidade que não está conectada.” | Conecte essa unidade e desinstale novamente em Configurações. |
| “Nem tudo pôde ser desfeito. Feche o que o estiver usando e desinstale novamente em Configurações.” | Feche o aplicativo e tudo o que estiver usando os arquivos dele. Se um serviço não parar, reinicie o PC. Depois, desinstale novamente. |
| “this is not an install's id” (a linha de comando responde em inglês) | A ID informada para `--uninstall` está errada. Copie-a da entrada em Configurações. Consulte [Linha de comando](Command-Line#finding-the-id). |
| Apareceu um arquivo chamado `.before-install` | É o original de um arquivo que a instalação substituiu e que você tinha alterado desde então. Fique com o que preferir. |
| Um runtime continua instalado | Os runtimes permanecem, porque outros programas podem usá-los. Deixe-o, a menos que você tenha certeza de que nada mais o usa. |

<a id="scripts"></a>

## Scripts

| O que você vê | O que fazer |
| --- | --- |
| “A operação solicitada requer elevação” | Execute o PowerShell como administrador. |
| Nada é exibido, e `$LASTEXITCODE` não é definido | Canalize a saída do instalador, por exemplo, para `Out-String`. Consulte [Linha de comando](Command-Line). |
| Código de saída `2` | Um parâmetro está errado, ou `--apply` e `--dry-run` foram usados juntos. |
| “this package does not say where its app was, so give a --target-root” (o pacote não diz onde o aplicativo estava) | Adicione `--target-root` com um caminho completo. |
| O aplicativo “was in” uma pasta, “and that cannot be used” (estava em uma pasta que não pode ser usada) | Adicione `--target-root` com outra pasta. |
| Uma pasta “cannot be used: it already has files” “Give --force only if you know it is this app's” (já contém arquivos) | Escolha uma pasta vazia. Use `--force` somente para uma pasta que você sabe que é deste aplicativo. |

<a id="language-and-text"></a>

## Idioma e texto

- **O idioma errado.** Escolha outro em **Idioma**, no canto superior direito do aplicativo ou da primeira tela de um instalador. O aplicativo se lembra da escolha; um instalador a mantém durante aquela execução. Se nenhuma escolha tiver sido feita, cada um usa o primeiro dos idiomas de exibição do Windows que tiver, ou então inglês.
- **Uma tradução está errada.** Tudo, exceto o inglês, foi traduzido automaticamente. Em **Idioma**, escolha **Relatar um problema de tradução**, que abre um formulário com o seu idioma já preenchido. Diga o que a tela mostra e o que ela deveria dizer.
- **Letras aparecem como caixas vazias.** A janela usa as fontes que vêm com o Windows para cada sistema de escrita, de `C:\Windows\Fonts`. Um Windows do qual algumas delas foram removidas desenha essas letras como caixas; os nomes de outros aplicativos na lista também podem precisar delas.
- **Palavras cortadas.** Um rótulo que não cabe termina em `…`; passe o mouse sobre ele, ou vá até ele com a tecla Tab, para ver o texto completo. Relate o problema informando o idioma e a tela, para que a janela possa ser ajustada.
- **O WinAppMigrator Client exibe inglês em um Windows da direita para a esquerda.** O console em que ele é executado não consegue unir nem ordenar letras árabes ou hebraicas, então ele exibe inglês e diz por quê. `--lang` escolhe qualquer idioma que ele tenha.

<a id="getting-help"></a>

## Como obter ajuda

1. Encontre o [log](Glossary#log). O caminho dele é mostrado como **Log** quando há uma falha, ou exibido como `Log:` na linha de comando. Os logs ficam em `%ProgramData%\WinAppMigrator\logs`. Eles contêm caminhos e nomes de chaves, e nunca o conteúdo de uma configuração, de um arquivo de licença ou de uma variável de ambiente. Mesmo assim, podem conter os nomes das suas pastas, então leia um log antes de compartilhá-lo.
2. [Abra um problema (issue)](https://github.com/RandyNorthrup/WinAppMigrator/issues) com a versão, o que você fez e o que a tela disse. **Copiar erro**, na tela de conclusão do aplicativo, copia a mensagem.
3. **Nunca anexe um instalador a um problema.** Ele pode conter dados particulares.
4. Para um problema de segurança, relate-o de forma privada, por meio das [consultorias de segurança do GitHub](https://github.com/RandyNorthrup/WinAppMigrator/security/advisories/new).
