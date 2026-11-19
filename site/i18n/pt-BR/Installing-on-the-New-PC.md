<a id="installing-on-the-new-pc"></a>

# Instalação no PC novo

O instalador é uma única janela. Ele verifica a si mesmo, mostra onde vai instalar e tudo o que vai configurar, e só instala quando você clica em **Instalar**. Esta página percorre a janela de cima para baixo, usando as palavras da tela.

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/installer-dark.png">
    <img alt="O instalador do Fabrikam Data Sync, mostrando que ele não está assinado, o local de instalação, o espaço necessário e o que ele configura além dos arquivos" src="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/installer-light.png" width="600">
  </picture>
</p>

<a id="before-you-start"></a>

## Antes de começar

- **Copie o instalador inteiro.** Se a criação gerou um arquivo `.wamdata` além do `.exe`, copie os dois para a mesma pasta e mantenha os nomes deles. Consulte [Aplicativos grandes](Big-Apps).
- **Execute somente instaladores em que você confia.** Um instalador é executado com [direitos de administrador](Glossary#administrator-rights) totais. Trate-o como qualquer outro programa: execute-o somente se você o criou ou se o recebeu de alguém em quem confia. Consulte [Privacidade e segurança](Privacy-and-Security).
- **Você precisa de direitos de administrador** neste PC, e do [Direct3D 12](Glossary#direct3d-12) para a janela. Sem o Direct3D 12, ainda é possível instalar a partir de um prompt de comando. Consulte [Linha de comando](Command-Line).

<a id="open-it"></a>

## Abrir o instalador

Clique duas vezes no instalador. O Windows mostra a [solicitação do UAC](Glossary#uac-prompt). Um instalador não é assinado, a menos que quem o criou o tenha assinado, então a solicitação geralmente diz **Editor desconhecido**. Isso é esperado. Consulte [Privacidade e segurança](Privacy-and-Security#signing).

A janela, “Instalador do WinAppMigrator”, “Reinstala um aplicativo capturado em outro PC”, abre na hora, seja qual for o tamanho do instalador. Ela diz “Verificando o instalador…” enquanto verifica o [hash](Glossary#hash) do pacote inteiro. Isso lê o pacote todo, então um instalador grande demora um pouco. A partir desse momento, o instalador mantém o próprio arquivo aberto, e também o arquivo de dados, se tiver um, para que nenhum dos dois possa ser alterado ou trocado enquanto a janela estiver aberta.

A verificação pode parar aqui, com uma mensagem em vermelho. As mais comuns são:

- O instalador está danificado ou foi alterado depois de criado: copie-o novamente do local onde ele foi criado.
- O arquivo de dados dele está ausente ou não é o correto: copie os dois arquivos novamente, para uma mesma pasta.
- “O pacote deste instalador não pode ser usado neste PC”, por exemplo porque os programas do aplicativo foram compilados para outro tipo de processador ou precisam de um Windows mais recente. Nada foi alterado.

Todas elas são explicadas em [Solução de problemas](Troubleshooting#installing).

Quando a verificação é aprovada, o título da janela passa a ser “Instalar” e o nome do aplicativo.

<a id="its-language"></a>

### O idioma do instalador

Um instalador fala o idioma do PC em que é executado, não o do PC em que foi criado: o idioma de exibição do Windows, se for um dos 24 que o WinAppMigrator tem, e inglês caso contrário. **Idioma**, no canto superior direito, muda o idioma para esta execução. O que ele lista do aplicativo, incluindo as observações que o aplicativo escreveu ao criá-lo, também vem nesse idioma. Um instalador criado com o WinAppMigrator 0.6 ou anterior mostra essas observações em inglês, como foram escritas. Uma [instalação silenciosa pela linha de comando](Command-Line) sempre responde em inglês, para que scripts possam ler.

<a id="the-app-and-where-it-goes"></a>

## O aplicativo e para onde ele vai

O primeiro cartão tem o nome do aplicativo, com o editor e a versão abaixo, como “Fabrikam Inc. · 1.7.3”. Em seguida:

| Linha | O que ela diz |
| --- | --- |
| **Capturado de** | O nome do PC de onde o aplicativo veio. Só aparece quando o instalador foi criado com a opção **Dados do usuário** ativada. |
| **Instalador** | Se este arquivo de instalador está [assinado](Glossary#signature): “Verificando a assinatura…”, depois “Não assinado: editor desconhecido” ou “Assinado por” e o nome de quem assinou. Uma assinatura que não se sustenta mostra, em vez disso, uma mensagem em vermelho: “A assinatura deste instalador não é válida”, com o motivo do Windows, e “Ele pode ter sido alterado depois de ser assinado.” |
| **Local de instalação** | A pasta para onde o aplicativo vai, com **Procurar** para escolher uma. Começa como a pasta em que o aplicativo estava no PC antigo. Um aplicativo de um Windows de 32 bits começa em `Program Files (x86)`, onde o Windows de 64 bits guarda um aplicativo de 32 bits. Se ela não puder ser usada aqui, um aviso em vermelho abaixo dela diz por quê. |
| **Espaço necessário** | Quanto espaço a instalação precisa nessa unidade e quanto está livre, como “até 824,0 MB em C: (356,0 GB livres)”. Conta o pacote descompactado e também os arquivos do aplicativo. |

O nome do editor na primeira linha vem do pacote, então qualquer pessoa poderia tê-lo escrito. A linha **Instalador** vem do Windows e é a que merece confiança.

<a id="when-the-location-cannot-be-used"></a>

### Quando o local não pode ser usado

Se a pasta em **Local de instalação** não puder ser usada, um aviso em vermelho abaixo dela diz por quê. **Instalar** e **Visualizar alterações** ficam esmaecidos até você escolher outra, e uma linha ao lado deles diz “Instalar e Visualizar alterações ficam desativados até que o Local de instalação possa ser usado.”

O motivo mais comum é que a pasta já contém arquivos. Uma pasta com arquivos só é usada se uma instalação anterior do WinAppMigrator os colocou lá, para que os arquivos de outro aplicativo nunca sejam substituídos:

| Aviso | O que significa | O que fazer |
| --- | --- | --- |
| O nome do aplicativo, seguido de “já está instalado nesta pasta, então esta cópia não pode ir para cá.” | O aplicativo já está instalado neste PC, muitas vezes pelo próprio instalador dele: o programa principal está nessa pasta, ou a lista de aplicativos instalados do Windows o indica lá. Isso acontece com aplicativos que muitos PCs já têm, como um navegador da Web, um player de mídia ou o Office. | Primeiro desinstale a cópia que está neste PC, em Configurações do Windows, em **Aplicativos**, e abra o instalador novamente. Ou digite em **Local de instalação** uma pasta nova e vazia para instalar esta cópia ao lado da outra. |
| O nome de outro aplicativo, seguido de “está instalado nesta pasta”, e que este aplicativo “não pode ir para cá.” | A lista de aplicativos instalados do Windows indica outro aplicativo nessa pasta, como outra edição do Office. | Digite em **Local de instalação** uma pasta nova e vazia. Ou, se você não precisa mais do outro aplicativo, desinstale-o primeiro, em Configurações do Windows, em **Aplicativos**, e abra o instalador novamente. |
| “Esta pasta já contém arquivos que não foram instalados pelo WinAppMigrator.” | Há outra coisa na pasta, e nenhum aplicativo instalado está listado lá. Pode ser de outro aplicativo. | Digite em **Local de instalação** uma pasta nova e vazia. Se os arquivos sobraram de um aplicativo que você desinstalou, você pode excluir a pasta e abrir o instalador novamente. |

Por qualquer outro motivo, o aviso diz “Esta pasta não pode ser usada:”, o motivo e “Digite outra pasta acima.” Os motivos são:

| Motivo na tela | O que fazer |
| --- | --- |
| “deve ser um caminho completo” | Digite um caminho que comece com uma letra de unidade, como `C:\Program Files\Fabrikam Data Sync`. |
| “não pode ser uma unidade inteira” | Use uma pasta na unidade, não a própria unidade. |
| “não pode ser a pasta do Windows” | Escolha uma pasta fora de `C:\Windows`. |
| “contém outros itens lado a lado, então o aplicativo precisa de uma subpasta” | Adicione uma pasta para o aplicativo, como `C:\Program Files\Fabrikam Data Sync` em vez de `C:\Program Files`. |
| “é uma pasta que pertence ao Windows ou cujo conteúdo é executado ou carregado automaticamente” | Escolha uma pasta comum. Uma pasta Inicializar, por exemplo, é recusada. |
| “precisa ser um caminho simples em uma unidade deste PC” | Um [compartilhamento de rede](Glossary#network-share) é recusado, assim como um nome que o Windows interpretaria como outra coisa. |
| que o caminho “é um link para outro lugar, e outras pessoas podem conseguir alterar para onde ele leva” | Use a pasta para onde o [link](Glossary#link) leva. |
| “é onde as instalações guardam os históricos” | Escolha uma pasta fora de `%ProgramData%\WinAppMigrator`. |

Se a pasta tiver arquivos e uma instalação anterior deste projeto **estiver** registrada nela, o instalador a usará, e avisa: “Esta pasta já contém arquivos. Arquivos com os mesmos nomes são substituídos e voltam se você desinstalar.”

Depois que uma pasta é aceita, a janela substitui um arquivo, valor do Registro, serviço, tarefa, atalho ou variável com o mesmo nome, como `--force` faz na [linha de comando](Command-Line#what---force-does). O que já pertence a outro aplicativo, como o serviço ou o tipo de arquivo dele, continua sendo desse aplicativo.

Se a unidade parecer ter pouco espaço, uma observação em âmbar diz “Talvez não haja espaço suficiente aqui” e o que é necessário. O pacote pode ser descompactado em outra unidade, mas os arquivos do aplicativo ainda precisam caber nesta. A própria instalação verifica o espaço novamente antes de começar.

<a id="an-earlier-install-that-never-finished"></a>

### Uma instalação anterior que não foi concluída

Se uma instalação anterior feita por qualquer um destes instaladores foi interrompida, por uma falha ou uma queda de energia, uma observação em âmbar diz “Uma instalação anterior de” esse aplicativo “não foi concluída. Ao instalar, ela é desfeita primeiro.” Isso acontece automaticamente quando você clica em **Instalar**.

<a id="what-this-sets-up-beyond-files"></a>

## O que isto configura além dos arquivos

Este cartão é a lista com a qual você concorda ao clicar em **Instalar**. Abaixo do título, ele diz:

> Tudo isso é feito com direitos de administrador. Se a instalação falhar, o que ela alterou é revertido, e você é informado do que não puder ser. Você também pode desfazer a instalação depois em Configurações, em Aplicativos instalados. Os arquivos que ela substitui voltam, e os runtimes que ela instala permanecem.

Cada linha tem um nome, um rótulo à direita dizendo que tipo de item é, e uma linha dizendo o que ele faz. Um rótulo no azul habitual é comum. Um rótulo em **âmbar** marca algo que é executado com direitos totais, como um serviço executado como LocalSystem, um driver ou uma regra de firewall que permite conexões de entrada. Examine esses itens com atenção redobrada.

Se a lista estiver vazia, ela diz “Nada: ele apenas copia arquivos e configurações nas próprias chaves do Registro do aplicativo.”

A lista é calculada para o local que está na caixa. Se você mudar o local, ela diz “Calculando o que isto configuraria neste local…” por um momento, e **Instalar** aguarda.

<a id="what-each-label-means"></a>

### O que cada rótulo significa

| Rótulo | O que é | Exemplo da linha abaixo dele |
| --- | --- | --- |
| **Serviço** | Um [serviço](Glossary#service): um programa que o Windows executa em segundo plano | “Executa FabrikamSync.exe como NT AUTHORITY\LocalService, inicia agora e sempre que este PC é iniciado”. Outros dizem “inicia agora e logo após cada inicialização deste PC”, “inicia só quando algo o solicita” ou “está desativado”. |
| **Driver** | Um [driver](Glossary#driver) que pertence ao aplicativo | “Carrega” um arquivo “no próprio Windows”. Sempre em âmbar. Um driver que inicia com o Windows é iniciado logo após a instalação. O do VeraCrypt foi carregado dessa forma, manualmente, em um PC de teste; a CI não carrega nenhum. |
| **Tarefa agendada** | Uma [tarefa agendada](Glossary#scheduled-task) | “É executada como SYSTEM”, ou como uma conta. “com privilégios mais altos” fica em âmbar. Uma tarefa que era executada como uma pessoa no PC antigo é executada como quem a instalar. |
| **Inicia automaticamente** | Um [item de inicialização](Glossary#startup-item) | “É executado quando você entrar”, “É executado para todos que entrarem” ou “Um atalho na pasta Inicializar, para você” |
| **Atalho** | Um [atalho](Glossary#shortcut) na área de trabalho ou no menu Iniciar | “Abre Fabrikam.exe, para todos neste PC” ou “para você” |
| **PATH** | Uma entrada adicionada à [variável Path](Glossary#path-variable) | “Adiciona a Path” e “A pasta de instalação\bin”. Ela é adicionada ao que já existe, nunca colocada no lugar disso. |
| **Variável de ambiente** | Uma [variável de ambiente](Glossary#environment-variable) | “Definida como” o valor dela |
| **Extensão do shell** | Código que é carregado no Explorador de Arquivos | “Carrega” um arquivo “no Explorador de Arquivos, para todos neste PC”. Em âmbar. |
| **Comando de menu** | Um comando no menu que se abre com o botão direito do mouse | “Adiciona um comando ao menu de” um tipo de arquivo. “Altera o que acontece ao abrir itens deste tipo” fica em âmbar. |
| **Tipo de link** | Um [protocolo de link](Glossary#link-protocol), como `fabrikam:` | “Links deste tipo abrem com o aplicativo” |
| **Tipo de arquivo** | Um [tipo de arquivo](Glossary#file-type), como `.fds` | “Arquivos deste tipo abrem com o aplicativo” |
| **COM** | [Classes COM](Glossary#com-class) que outros programas podem iniciar | “COM em” um arquivo, com quantas configurações. “Permissões COM” fica em âmbar. |
| **Regra de firewall** | Uma [regra de firewall](Glossary#firewall-rule) para um dos programas dele | “Permite conexões de entrada nas portas 8080 para FabrikamSync.exe, em todas as redes”. Permitir conexões de entrada fica em âmbar. |
| **Origem do log de eventos** | Um nome sob o qual o aplicativo grava nos logs de eventos do Windows | “Pode gravar no log Application, com mensagens dos próprios arquivos” |
| **Fonte** | Uma fonte na pasta de instalação | “É carregada da pasta de instalação quando você entra” |
| **Driver ODBC** | Um driver de banco de dados para [ODBC](Glossary#odbc) | “É carregado em programas que usam ODBC para acessar um banco de dados”. Em âmbar. |
| **Fonte de dados ODBC** | Uma conexão de banco de dados salva | “Uma conexão de banco de dados, para todos neste PC” |
| **Configuração de compatibilidade** | Como o Windows inicia alguns dos programas dele | “Como os programas dele são iniciados” |
| **Aplicativos padrão** | Uma entrada que permite escolher o aplicativo em Aplicativos padrão | “Pode ser escolhido em Aplicativos padrão. Nada passa a usá-lo automaticamente” |
| **Nome do programa** | Um nome pelo qual a caixa Executar e outros programas podem encontrá-lo | “A caixa Executar e outros programas podem encontrá-lo por este nome” |
| **Outras configurações** | Valores do [Registro](Glossary#registry) fora das chaves do próprio aplicativo e dos tipos acima | “3 configurações fora das chaves do próprio aplicativo”, “Em” as chaves. Em âmbar se alguma estiver nas chaves da Microsoft. |

Algumas coisas só podem ser avaliadas quando a instalação chega a elas, como um serviço com o mesmo nome que já pertence a outro aplicativo. Elas ficam na lista como foram solicitadas. Quando a instalação chega lá, ela as deixa como estão e informa isso.

<a id="notes-and-what-is-left-out"></a>

### Observações e o que fica de fora

Abaixo das linhas, observações em âmbar dizem o que você deve saber, por exemplo:

- que credenciais não foram colocadas no instalador, então talvez você precise digitá-las novamente;
- que a opção **Dados de licença e ativação** ou **Dados do usuário** estava desativada quando ele foi criado, então esses dados não estão nele;
- de quais [certificados](Glossary#certificate) o aplicativo parecia depender, que nunca são instalados;
- o que quem criou o instalador deixou de fora e o que adicionou;
- que o aplicativo pode não funcionar aqui: “Este aplicativo estava na compilação” mais recente “do Windows”, ou que o Windows executaria alguns dos programas dele “por meio de um emulador”;
- que arquivos de Documentos ou Jogos Salvos vão “para” a pasta de instalação, “em uma pasta chamada _external”, porque a pasta deste PC fica fora do seu perfil.

Em seguida, em âmbar, cada item que as regras de segurança deixam de fora, com o motivo. Os primeiros 20 são mostrados por completo. Os demais ficam na linha que diz quantos mais “ficaram de fora”, que você pode abrir. O restante da instalação continua sem eles. O que é recusado, e por quê, está em [O que vai em um instalador](What-Goes-in-an-Installer#what-the-installer-refuses).

Se o pacote não puder ir para o local escolhido de forma alguma, uma mensagem em vermelho diz “Este pacote não pode ir para este local” e por quê.

<a id="options"></a>

## Opções

Este cartão só aparece quando o instalador contém dados do usuário ou dados de licença. Cada caixa começa marcada.

| Opção | O que ela diz |
| --- | --- |
| **Dados do usuário** | “Ativado: configurações, Documentos e Jogos Salvos vão para as pastas habituais de quem estiver conectado. Desativado: tudo vai para a pasta de instalação, e nada é configurado para ser executado a partir dela.” Se o instalador disser de quem são os dados que contém, uma linha abaixo diz “Contém os dados de” essa conta. |
| **Licença** | “Arquivos de ativação e licença, e os valores do Registro que guardam licenças. Alguns podem precisar de reativação aqui.” |

Alterar uma caixa atualiza a lista acima na hora.

Os dados do usuário sempre vão para **quem estiver conectado** ao Windows, mesmo que o nome de outro administrador tenha sido digitado na solicitação do UAC. Se o instalador não conseguir determinar quem é, ele informa isso, deixa de fora as configurações por usuário e coloca os dados do usuário na pasta de instalação em vez de adivinhar. Se ninguém estiver conectado nessa sessão, os dados do usuário e as configurações por usuário vão para a conta com a qual o instalador é executado, e ele também informa isso.

<a id="dependencies"></a>

## Dependências

“Do que este aplicativo precisa e como cada item é fornecido.” Cada [runtime](Glossary#runtime) tem um rótulo:

| Rótulo | Significado |
| --- | --- |
| **Incluído** | Dentro do instalador. É instalado sem internet, exceto o DirectX End-User Runtime, cujo instalador o baixa da Microsoft. Destes, só o runtime x64 do Visual C++ foi instalado de verdade nos testes do projeto. |
| **Instalado com o winget:** e uma ID | Não foi possível baixá-lo quando o instalador foi criado, então o instalador usa o [winget](Glossary#winget), que precisa de internet. Esse caminho não foi executado nos testes do projeto. |
| **Recurso do Windows: NetFx3** | .NET Framework 3.5, ativado como um [recurso do Windows](Glossary#windows-feature). Isso pode precisar de internet. Esse caminho não foi executado nos testes do projeto. |
| **Integrado ao Windows** | .NET Framework 4, que o Windows 10 e 11 já têm |
| **Acompanha o aplicativo** | O aplicativo traz a própria cópia |
| **Instalar manualmente** | Não há instalador para ele aqui. Instale-o você mesmo. |

Se não houver nenhum, ele diz “Nenhuma dependência de runtime adicional foi detectada.” Consulte [Runtimes](Runtimes).

<a id="preview-changes"></a>

## Visualizar alterações

**Visualizar alterações** calcula exatamente o que uma instalação faria, sem alterar nada. Ela verifica se há espaço, descompacta o pacote em uma pasta privada, calcula as alterações e remove essa pasta novamente. O cartão de resultado tem o título “Visualização: nada foi alterado ainda”, com **Alterações planejadas**, e uma lista das alterações, com serviços, tarefas e atalhos primeiro. A janela lista as primeiras 300 e depois diz quantas mais há.

Uma visualização não verifica o hash de cada arquivo. O hash do pacote inteiro foi verificado quando a janela foi aberta.

<a id="install"></a>

## Instalar

**Instalar** fica esmaecido até que o local possa ser usado e a lista seja a desse local. Se o pacote não puder ir para esse local de forma alguma, uma linha ao lado diz “Instalar está desativado porque este pacote não pode ir para este local”, e o motivo aparece em vermelho em **O que isto configura além dos arquivos**. Quando você clica nele:

1. Ele calcula a lista mais uma vez. Se o que ele faria mudou desde que você olhou, ele não instala nada, mostra a nova lista e diz “O que isto faria aqui mudou, então nada foi instalado. Revise a lista abaixo e clique em Instalar novamente se for o que você deseja.”
2. Ele verifica se há espaço em todas as unidades em que grava.
3. Ele descompacta o pacote em uma pasta que só administradores podem usar, verificando o hash de cada arquivo à medida que ele sai.
4. Ele instala, registrando antes cada alteração em um [diário](Glossary#journal), para que ela possa ser desfeita.
5. Ele relê cada arquivo que instalou e verifica o hash dele.
6. Ele inicia os serviços configurados para iniciar sozinhos.

Na parte inferior, você vê o que ele está fazendo, uma contagem quando houver, o tempo decorrido, uma barra de progresso e **Cancelar**. As etapas que você pode ver são “Preparando…”, “Descompactando”, “Verificando o pacote”, “Desfazendo uma instalação anterior que não foi concluída”, “Instalando runtimes”, “Copiando arquivos”, “Configurando serviços”, “Configurando drivers”, “Gravando configurações no Registro”, “Restaurando dados de licença”, “Configurando tarefas agendadas”, “Criando atalhos”, “Definindo variáveis de ambiente”, “Adicionando o aplicativo em Configurações” e “Verificando o que foi instalado”.

**Cancelar** mostra “Cancelando após a etapa atual…” e depois “Revertendo o que foi alterado”. Fechar a janela durante a instalação faz o mesmo que **Cancelar**: a janela permanece até que tudo seja revertido. Se nada tiver sido alterado ainda, ela diz “Cancelado. Nada foi alterado.”

<a id="the-hash-check"></a>

## A verificação de hash

O instalador verifica [hashes](Glossary#hash) três vezes:

1. **Quando a janela abre**: o hash do pacote inteiro. Uma cópia truncada, ou editada depois de criada, é recusada antes que qualquer coisa nela seja lida.
2. **Durante a descompactação**: o hash de cada arquivo, antes que algo seja gravado no lugar dele.
3. **Depois da instalação**: ele relê cada arquivo que instalou e o verifica novamente.

Os hashes detectam danos e alterações feitas por qualquer pessoa que não atualize também os hashes. Eles não provam quem criou um instalador. Só uma [assinatura](Glossary#signature) faz isso.

<a id="the-result"></a>

## O resultado

O cartão de resultado aparece na parte superior, com o título “Resultado da instalação”.

| Mensagem | O que significa |
| --- | --- |
| “O aplicativo foi instalado e passou na verificação de hash.” | Está tudo certo. Se as regras deixaram algo de fora, ela acrescenta “sem o que está listado abaixo como deixado de fora.” |
| “A instalação foi cancelada, e o que ela alterou foi revertido.” | Você cancelou. |
| “A instalação falhou, e o que ela alterou foi revertido.” | Algo falhou. O que ela alterou foi revertido, exceto qualquer runtime que tenha instalado. O que estiver listado em âmbar abaixo ficou como estava, e diz por quê, como um serviço que só sai quando o PC reinicia. |
| “A instalação falhou, e nem tudo o que ela alterou pôde ser revertido. O que restou está listado abaixo, e a próxima instalação tentará de novo.” | Leia a lista. Executar um instalador novamente tenta reverter isso de novo. |
| “A instalação não foi concluída sem problemas. Revise os problemas abaixo.” | Leia as mensagens abaixo dela. |

Em seguida, três blocos: **Alterações feitas** (ou **Alterações revertidas** após uma falha), **Avisos** e **Erros**. Rótulos dizem **Hashes do pacote válidos** e **Arquivos instalados válidos**, ou **INVÁLIDOS** em vermelho. Até cinco avisos e todos os erros são mostrados como mensagens. “Uma dependência precisa que o PC seja reiniciado para concluir. Reinicie o Windows e abra o aplicativo.” significa que um runtime pediu uma reinicialização.

**Alterações**, com uma contagem, abre a lista do que foi feito, até 300 itens. Se a instalação não foi concluída sem problemas, o caminho do [log](Glossary#log) dela é mostrado como **Log**.

Após uma instalação sem problemas, os botões na parte inferior passam a ser **Fechar** e **Abrir pasta de instalação**.

<a id="where-things-end-up"></a>

## Onde as coisas ficam

- **Os arquivos do programa** vão para o local de instalação.
- **Dados do usuário**, com essa caixa marcada, vão para as pastas AppData, Documentos e Jogos Salvos da própria conta conectada, e para ProgramData. Uma pasta Documentos ou Jogos Salvos que este PC mantém fora do seu perfil não recebe gravações. Esses arquivos vão para o local de instalação, em `_external`, e a lista informa isso primeiro. Com a caixa desmarcada, todos os dados do usuário vão para o local de instalação.
- **Os atalhos** vão para a área de trabalho e o menu Iniciar deste PC: os seus, ou os de todos, se era lá que estavam.
- **Path** recebe apenas as entradas do aplicativo. Nada mais nela muda.
- **O que outro aplicativo já tem aqui continua sendo desse aplicativo.** Um tipo de arquivo, tipo de link, classe COM, serviço, origem do log de eventos, fonte ou qualquer coisa parecida que já pertença a outro programa fica como está, e o instalador informa isso. Um aplicativo que esperava abrir links `http`, por exemplo, precisa ser definido como padrão novamente em Configurações.
- **Configurações** recebe uma entrada para o aplicativo, para que você possa desinstalá-lo como qualquer outro. Consulte [Desfazer uma instalação](Undoing-an-Install).

<a id="common-reasons-it-refuses"></a>

## Motivos comuns de recusa

| O que você vê | Por quê | O que fazer |
| --- | --- | --- |
| “este aplicativo não pode ser executado neste PC, então nada foi alterado” | Os programas dele foram compilados para outro tipo de processador ou precisam de um Windows mais recente | Instale em um PC compatível com o antigo |
| “não há espaço suficiente para isto, então nada foi alterado” | Não há espaço livre suficiente em uma unidade em que ele grava | Libere espaço ou escolha um local em outra unidade |
| “outra instalação ou desinstalação está em execução” | Só uma é executada por vez | Aguarde até que ela termine e tente novamente |
| Um arquivo “tem um nome que esta versão do Windows abre como dispositivo, e não como arquivo” | O aplicativo tem um arquivo com um nome como `aux.h`, que o Windows anterior ao Windows 11 não consegue armazenar | Instale no Windows 11 |
| Uma pasta de dados como `%AppData%` está sob um link | O instalador não grava por meio de um link | Desmarque **Dados do usuário** e **Licença** e instale sem eles |

Mais motivos estão em [Solução de problemas](Troubleshooting#installing).
