# Words of the app, WinAppMigrator itself: its one window, with the installed apps listed on the left and, on the
# right, what goes in an installer of the one picked and the button that builds it; and the steps to read another PC
# through WinAppMigrator Client. Message names start with app-.


## The window and its header

# The program's name, in the window's title bar and at the top of the window. Keep it as it is unless your language
# writes product names in a script of its own.
app-name = WinAppMigrator
app-tagline = Transforme um aplicativo instalado em um instalador autoextraível
app-theme-to-light = Mudar para o tema claro
app-theme-to-dark = Mudar para o tema escuro
app-reload-list = Recarregar a lista de aplicativos
app-list-loading = Carregando
# { $count } is how many apps are listed.
app-list-count =
    { $count ->
        [0] { $count } aplicativos
        [one] { $count } aplicativo
       *[other] { $count } aplicativos
    }
app-list-load-failed = Falha ao carregar
# Shown when the window cannot open at all. { $error } is what went wrong, in the words of Windows or a graphics driver.
app-window-could-not-open = Não foi possível abrir a janela: { $error }

## The list of installed apps

app-list-search = Pesquisar por nome ou editor  (Ctrl+F)
app-list-filter-all = Todos
# Whether an app is installed for one account or for everyone on the PC.
app-scope-per-user = Por usuário
app-scope-per-machine = Por máquina
app-scope-unknown = Desconhecido
# An app whose processor architecture (x86, x64, ARM64) is not known.
app-arch-unknown = Desconhecida
app-list-loading-apps = Carregando os aplicativos instalados…
# { $error } is why the apps could not be listed.
app-list-could-not-list = Não foi possível listar os aplicativos: { $error }
app-list-no-matches = Nenhum aplicativo correspondente
app-list-no-matches-hint = Ajuste o texto da pesquisa ou o filtro de escopo.
app-list-worker-disconnected = O processo de inventário foi desconectado.
# What a screen reader says of an app's row. { $name } is the app's name, { $publisher } who made it, and { $version } its
# version, such as 11.4.2, or — when either is not known.
app-list-row = { $name }, { $publisher }, versão { $version }

## The list's columns, and what a screen reader says each column's heading does when clicked

app-column-name = Nome
app-column-name-sort = Classificar por nome
app-column-publisher = Editor
app-column-publisher-sort = Classificar por editor
app-column-version = Versão
app-column-version-sort = Classificar por versão
app-column-scope = Escopo
app-column-scope-sort = Classificar por escopo
# Short for architecture: x86, x64, or ARM64. The column is narrow.
app-column-arch = Arq.
app-column-arch-sort = Classificar por arquitetura

## Where apps are read from: this PC, a drive from another PC, a disk image of one, or another PC on the network

app-source = Origem da análise
app-source-this-pc = Este PC
app-source-drive = Unidade conectada…
app-source-image = Imagem de disco…
app-source-network = PC na rede…
app-source-attaching-image = Anexando a imagem, somente leitura…
# { $computer } is the other PC's name, such as OLD-PC, and { $address } its network address, such as 192.168.1.20.
app-source-network-pc = { $computer } pela rede ({ $address })
app-source-disconnect = Desconectar
app-source-change = Alterar…
# The title of the window that picks a disk image. .vhd and .vhdx are file extensions.
app-source-pick-image-title = Selecionar uma imagem de disco do outro PC (.vhd ou .vhdx)
app-source-pick-image-filter = Imagens de disco
# { $path } is the disk image file.
app-source-image-has-no-windows = { $path } não tem nenhum volume com o Windows.
app-source-image-not-attached = Não foi possível anexar a imagem de disco.
app-source-pick-drive-title = Selecionar a raiz da unidade conectada (por exemplo, E:\)
# { $path } is the folder picked, such as E:\. \Windows and \Users are folder names, as Windows writes them.
app-source-not-a-windows-drive = { $path } não parece ser uma unidade do sistema Windows (não tem \Windows nem \Users). Escolha a raiz da unidade do outro PC.
# Said of a drive or disk image with a Windows older than Windows 10. { $windows } is its name, such as "Windows 7
# Professional Service Pack 1, 32-bit". { $is_32_bit } is "yes" when that Windows is 32-bit, which adds the last
# sentence, and "no" when it is not.
app-source-older-windows =
    Este disco tem o { $windows }. Os instaladores criados a partir dele são feitos para o Windows 10 e 11. Com a opção Dados do usuário, as configurações que o Windows guardou discretamente à parte para um aplicativo mais antigo vêm junto, e os runtimes que o Windows 11 não tem, como o DirectX 9, são instalados com o aplicativo.{ $is_32_bit ->
        [yes] { " " }Os aplicativos dele são de 32 bits, então vão para Program Files (x86), onde o Windows de 64 bits os mantém.
       *[no] { "" }
    }
# The same, of a Windows whose name could not be read. { $is_32_bit } is as above.
app-source-older-windows-unnamed =
    Este disco tem um Windows mais antigo. Os instaladores criados a partir dele são feitos para o Windows 10 e 11. Com a opção Dados do usuário, as configurações que o Windows guardou discretamente à parte para um aplicativo mais antigo vêm junto, e os runtimes que o Windows 11 não tem, como o DirectX 9, são instalados com o aplicativo.{ $is_32_bit ->
        [yes] { " " }Os aplicativos dele são de 32 bits, então vão para Program Files (x86), onde o Windows de 64 bits os mantém.
       *[no] { "" }
    }

## The details panel, before an app is picked, and the app picked

app-details-pair-first = Emparelhe com o PC antigo
app-details-pair-first-hint = Quando o WinAppMigrator Client estiver em execução lá e você tiver digitado o código dele, os aplicativos desse PC serão listados aqui, e você poderá criar um instalador de um deles como faria neste PC.
app-details-pick-an-app = Escolha um aplicativo
app-details-pick-an-app-hint = Selecione um aplicativo na lista para escolher o que incluir e criar o instalador dele.
# { $publisher } is who made the app, and { $version } its version, such as 11.4.2.
app-details-publisher-and-version = { $publisher }, versão { $version }
# { $version } is the app's version, such as 11.4.2.
app-details-unknown-publisher-and-version = Editor desconhecido, versão { $version }
app-details-unknown-publisher = Editor desconhecido

## What goes in besides the program

app-option-license = Dados de licença e ativação
app-option-license-hint = Podem estar vinculados a este PC e precisar de reativação no novo.
app-option-user-data = Dados do usuário
# Documents, Saved Games, and ProgramData are folders of Windows: use the names Windows gives them in your language,
# except ProgramData, which keeps its name.
app-option-user-data-hint = Configurações e histórico, Documentos, Jogos Salvos e ProgramData.
app-option-whose-data = Dados de quem
app-option-whose-data-hint = Um instalador contém os dados de uma conta. Eles vão para quem estiver conectado onde ele for executado.

## Building the installer

app-output-folder = Pasta de saída
# An example of a folder, shown in the empty field. "You" stands for the name of an account.
app-output-folder-example = C:\Users\Você\WinAppMigrator
app-output-folder-not-full = Digite o caminho completo de uma pasta, como D:\Instaladores.
app-build-create = Criar instalador
app-build-create-again = Criar instalador novamente
app-build-cancel = Cancelar
app-build-cancelling = Cancelando após a etapa atual…
app-build-cancelled = Criação cancelada. Nenhum instalador foi gravado.
# winappmigrator-installer.exe is a file name.
app-build-no-stub = O stub do instalador (winappmigrator-installer.exe) não foi encontrado ao lado deste aplicativo, então não é possível criar um instalador autoextraível.
app-build-select-an-app-first = Selecione primeiro um aplicativo para migrar.
app-build-look-again-first = Este aplicativo precisa ser analisado novamente antes que o instalador possa ser criado.
# { $reason } is what went wrong, in the program's own words, often in English.
app-build-stopped-unexpectedly = A criação parou inesperadamente: { $reason }. Se ela deixou um instalador na pasta de saída, não o use.
app-build-worker-disconnected = O processo de migração foi desconectado.
# { $path } is the output folder, and { $error } what Windows said went wrong.
app-build-no-output-folder = Não foi possível criar a pasta de saída { $path }: { $error }
# { $error } is what went wrong.
app-build-no-staging-folder = Não foi possível criar uma pasta de preparação protegida, então nada foi criado: { $error }
# { $error } is what went wrong, as a sentence.
app-build-could-not-build = Não foi possível criar o instalador. { $error }
app-build-no-installer = A criação terminou sem nenhum instalador.
# { $error } is what went wrong.
app-build-hash-check-did-not-run = O instalador foi gravado, mas a verificação de hash dele não pôde ser executada: { $error }. Não o use.

## How far a build has got

app-progress-preparing = Calculando o que entra…
app-progress-checking-room = Verificando se há espaço…
app-progress-bundling = Empacotando runtimes e drivers…
# { $done } is how many files have been read so far, and { $total } how many there are.
app-progress-packing = Lendo e compactando arquivos: { $done } de { $total }
# { $done } is how many of the installer's records have been looked over so far, and { $total } how many there are.
app-progress-looking-over = Revisando o que o instalador diz: { $done } de { $total }…
app-progress-writing = Gravando o instalador…
app-progress-verifying = Verificando hashes…

## What a build made

app-result-failed-hash-check = Não use este instalador: ele falhou na verificação de hash.
# { $count } is how many files were gone when the build came to read them.
app-result-files-gone =
    { $count ->
        [one] Instalador criado, mas { $count } arquivo deixou de existir desde que o aplicativo foi analisado, então ele não está incluído. Veja os avisos abaixo.
       *[other] Instalador criado, mas { $count } arquivos deixaram de existir desde que o aplicativo foi analisado, então eles não estão incluídos. Veja os avisos abaixo.
    }
# { $count } is how many files could not be read into the installer.
app-result-files-not-captured =
    { $count ->
        [one] Instalador criado, mas não foi possível capturar { $count } arquivo, geralmente porque ele estava bloqueado ou ilegível. O aplicativo pode não funcionar sem ele. Veja os avisos abaixo.
       *[other] Instalador criado, mas não foi possível capturar { $count } arquivos, geralmente porque eles estavam bloqueados ou ilegíveis. O aplicativo pode não funcionar sem eles. Veja os avisos abaixo.
    }
app-result-mentions-who = Instalador criado, mas ele ainda revela algo sobre a origem dele. Leia o que ele diz abaixo antes de entregá-lo a alguém.
app-result-ready = Instalador pronto. O pacote dele passou na verificação de hash.
app-result-files = Arquivos
app-result-installer-size = Tamanho do instalador
app-result-warnings = Avisos
app-result-log = Log
app-result-copy-error = Copiar erro
app-result-installer = Instalador
app-result-data-file = Arquivo de dados
app-result-open-folder = Abrir pasta
app-result-copy-path = Copiar caminho
# { $error } is what Windows said went wrong.
app-result-could-not-open-folder = Não foi possível abrir a pasta: { $error }
# { $installer } and { $data_file } are file names, such as App_installer.exe and App_installer.wamdata.
app-result-two-files = Este aplicativo é grande demais para um único executável, então o instalador dele tem dois arquivos, { $installer } e { $data_file }. Eles precisam ficar juntos: copie os dois para o outro PC, na mesma pasta, com estes nomes. Se você assinar o instalador, essa assinatura também abrange o arquivo de dados.
# { $app_name } is the app's name. "Unknown publisher" is what Windows shows when it asks to run an unsigned program: use
# Windows' own words for it in your language.
app-result-run-it = Execute-o no outro PC para reinstalar { $app_name }. Ele verifica os próprios arquivos por SHA-256. O Windows mostra “Editor desconhecido” porque os instaladores que você cria não são assinados.
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-verification-errors = Erros de verificação ({ $count })
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-warnings-heading = Avisos ({ $count })
# After the first of a long list of warnings. { $count } is how many more there are.
app-warnings-and-more = e mais { $count }.

## Looking at an app, to see what an installer of it would hold

app-scan-getting-ready = Preparando para analisar este aplicativo…
app-scan-install-record = Lendo como ele foi instalado…
app-scan-near-matches = Procurando o que também pode fazer parte dele…
app-scan-system-items = Procurando os serviços, as tarefas e os atalhos dele…
app-scan-files = Percorrendo os arquivos dele…
app-scan-registry = Lendo o que o integra ao Windows…
app-scan-finishing = Concluindo…
app-scan-look-again = Analisar novamente
app-scan-stop = Parar
app-scan-running-hint = Você pode escolher outro aplicativo ou alterar o que está incluído enquanto isso é executado.
# { $error } is why, in words that follow a colon.
app-scan-failed = Não foi possível analisar este aplicativo: { $error }
app-scan-try-again = Tentar novamente
app-scan-stopped = Interrompido. Um instalador pode ser criado depois que este aplicativo for analisado.
# { $reason } is what went wrong, in the program's own words, often in English.
app-scan-stopped-unexpectedly = A análise parou inesperadamente: { $reason }.

## How big an installer can be, and whether there is room for it

app-room-files = Arquivos
# Over how big the installer can be: up to that size.
app-room-up-to = Até
app-room-free = Livre
# { $drive } is a drive, such as C:.
app-room-free-on = Livre em { $drive }
# How much room a drive has, when it would not say.
app-room-free-unknown = Desconhecido
app-room-most-it-could-need = O máximo de que ele poderia precisar, antes da compactação.
# { $drive } is a drive, such as C:, { $needed } how much room it needs, such as 1.2 GB, and { $free } how much it has.
app-room-drive-short = { $drive } precisa de até { $needed } livres para criar isto e tem { $free }.
# { $drives } is one sentence like the one above for each drive without the room, one after another.
app-room-not-enough = Não há espaço suficiente. { $drives } Escolha uma pasta de saída em outra unidade ou deixe algo de fora.
# { $note } is a sentence, without its full stop, about a drive whose room could not be checked. Use your language's
# full stop.
app-room-could-not-tell = { $note }.

## What goes in the installer, part by part

app-footprint-heading = O que entra no instalador
app-footprint-working-out = Calculando o que vai junto com isso…
# { $error } is why.
app-footprint-would-not-make-a-package = O que sobrou não formaria um pacote: { $error }
# { $list } is the names of what else is left out, such as a service and a shortcut.
app-footprint-also-left-out = Isso também deixa de fora { $list }, porque o que eles executam ou indicam não estaria lá. Eles não serão configurados no outro PC.
# { $list } is the names of the first few of what else is left out, and { $count } how many more there are.
app-footprint-also-left-out-and-more = Isso também deixa de fora { $list } e mais { $count }, porque o que eles executam ou indicam não estaria lá. Eles não serão configurados no outro PC.
# { $list } is the names of license files. The option is the one named "License and Activation Data".
app-footprint-licenses-still-go = Os dados de licença no que você deixou de fora ainda entram, porque eles dependem da opção Dados de licença e ativação: { $list }.
# { $elapsed } is how long ago, in minutes and seconds such as 45:00, or hours, minutes, and seconds such as 1:05:00.
# "Look Again" names the button of that name.
app-footprint-looked-at-long-ago = Isto foi analisado há { $elapsed }. Os arquivos são lidos quando você cria o instalador, então uma alteração é levada em conta, e o que tiver deixado de existir até lá é listado. Use Analisar novamente para incluir o que for novo.
# Headings of the parts of the list, each with a line that says what is in it.
app-footprint-program-files = Arquivos do programa
app-footprint-data-folders = Pastas de dados
# What could go in with the User Data option, when nothing else is in this part. { $size } is how much, such as 1.2 GB.
app-footprint-folders-with-user-data = { $size } com a opção Dados do usuário
# { $included } is how many folders go in, { $count } how many there are, and { $size } how much those that go in hold.
app-footprint-folders-summary =
    { $count ->
        [0] { $included } de { $count } pastas, { $size }
        [one] { $included } de { $count } pasta, { $size }
       *[other] { $included } de { $count } pastas, { $size }
    }
# { $size } is how much, such as 1.2 GB, and { $count } in how many files.
app-footprint-data-only-with-user-data =
    { $count ->
        [one] Os dados deste aplicativo em { $count } arquivo ({ $size }) só entram com a opção Dados do usuário.
       *[other] Os dados deste aplicativo em { $count } arquivos ({ $size }) só entram com a opção Dados do usuário.
    }
# A folder's row. { $count } is how many files it holds, and { $detail } where it is or why it cannot be left out.
app-footprint-folder-row =
    { $count ->
        [0] { $count } arquivos. { $detail }
        [one] { $count } arquivo. { $detail }
       *[other] { $count } arquivos. { $detail }
    }
app-footprint-services = Serviços
app-footprint-startup = Inicia automaticamente
app-footprint-tasks = Tarefas agendadas
app-footprint-shortcuts = Atalhos
# { $included } is how many of the part's things go in, and { $count } how many there are.
app-footprint-items-summary = { $included } de { $count }
# { $why } is what else was left out that this goes with.
app-footprint-left-out-too = Também deixado de fora: { $why }
# A row's detail. { $kind } is what it is, such as Service, and { $detail } what it does.
app-footprint-change-detail = { $kind }. { $detail }
app-footprint-integrations = O que o integra ao Windows
# { $included } is how many kinds go in, such as file types and firewall rules, and { $count } how many kinds there are.
app-footprint-integrations-summary =
    { $count ->
        [0] { $included } de { $count } tipos
        [one] { $included } de { $count } tipo
       *[other] { $included } de { $count } tipos
    }
app-footprint-integration-nothing-left = Nada sobrou disso, por causa do que você deixou de fora
# { $count } is how many things of this kind the installer sets up.
app-footprint-integration-things =
    { $count ->
        [0] { $count } itens configurados
        [one] { $count } item configurado
       *[other] { $count } itens configurados
    }
app-footprint-prerequisites = Do que ele precisa para ser executado
# { $count } is how many runtimes, such as Visual C++ Redistributable.
app-footprint-prerequisites-summary =
    { $count ->
        [0] { $count } runtimes
        [one] { $count } runtime
       *[other] { $count } runtimes
    }
app-footprint-near-matches = Também pode fazer parte dele
# { $ticked } is how many were ticked, and { $count } how many there are.
app-footprint-near-matches-summary = { $ticked } de { $count } adicionados
app-footprint-near-matches-hint = Não entram no instalador, a menos que você os marque. Marcar um deles analisa o aplicativo novamente, com as mesmas verificações do resto, então o que parece uma senha continua de fora.
# { $why } is why it might be part of the app, as a sentence.
app-footprint-near-match-with-user-data = Vem com a opção Dados do usuário. { $why }
# How much a folder holds, when not all of it could be measured. { $size } is how much was, such as 1.2 GB.
app-footprint-near-match-over = mais de { $size }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-folder = Pasta: { $place }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-hidden = Parte oculta de um aplicativo: { $place }
app-footprint-nothing-left-out = Nada foi deixado de fora por parecer uma senha, uma licença ou um certificado.
app-footprint-left-out = O que foi deixado de fora e por quê
# { $count } is how many things were left out.
app-footprint-left-out-summary =
    { $count ->
        [0] { $count } itens
        [one] { $count } item
       *[other] { $count } itens
    }
# Above a list of things left out for one reason. { $title } is the reason.
app-footprint-left-out-group = { $title }:
app-footprint-refused = Nenhum instalador teria permissão para configurar estes itens:
# A heading over a list that opens and closes. { $count } is how many are in it.
app-footprint-worth-knowing = Vale saber ({ $count })
# { $list } is what kinds of things were looked for and none found, from the five below.
app-none-found = Não foram encontrados: { $list }.
app-none-found-services = serviços
app-none-found-startup = itens de inicialização
app-none-found-tasks = tarefas agendadas
app-none-found-shortcuts = atalhos
app-none-found-integrations = itens que o integrem ao Windows

## Network PC: reading another PC through WinAppMigrator Client, in three steps. "WinAppMigrator Client" is the name of a
## program, and the old PC is the one the apps are moved off.

app-network-step-client = 1. Coloque o cliente no PC antigo
app-network-step-client-hint = O WinAppMigrator Client é executado no PC em que o aplicativo está agora, com Windows 7 ou posterior. Ele só permite que este PC o leia, com criptografia, por meio de um código que ele mostra.
# { $drive } is a USB drive, such as E:\.
app-network-save-to-usb = Salvar na unidade USB { $drive }
# { $drive } is a USB drive, such as E:\, and { $name } the name it was given, such as BACKUP.
app-network-save-to-named-usb = Salvar na unidade USB { $drive } ({ $name })
app-network-save-to-folder = Salvar em uma pasta…
app-network-look-for-usb-again = Procurar unidades USB novamente
app-network-no-usb = Nenhuma unidade USB está conectada. Conecte uma ou salve em uma pasta que você consiga acessar do PC antigo, como uma pasta compartilhada na sua rede.
# { $path } is where the client was saved, such as E:\WinAppMigrator Client.exe.
app-network-client-saved = Salvo como { $path }. No PC antigo, feche o aplicativo que você está transferindo, clique duas vezes no cliente e permita que ele faça alterações. Ele mostrará um código.
app-network-step-pick = 2. Escolha o PC antigo
app-network-looking = Procurando na sua rede PCs que executam o cliente…
app-network-type-name = Ou digite o nome ou o endereço dele
# An example, shown in the empty field: a PC's name, or its network address.
app-network-type-name-example = PC-ANTIGO ou 192.168.1.20
app-network-step-code = 3. Digite o código que ele mostra
app-network-code = Código de emparelhamento
app-network-connect = Conectar
app-network-pairing = Emparelhando…
# Why pairing did not work, shown on its own.
app-network-pairing-stopped = o emparelhamento foi interrompido antes de terminar
app-network-wrong-code = esse não é o código que o PC antigo mostra. Verifique-o e tente novamente.
# { $error } is what went wrong.
app-network-not-reached = não foi possível acessar o PC antigo: { $error }
app-network-pick-folder-title = Escolher uma pasta para o WinAppMigrator Client
# { $path } is the folder, and { $error } what Windows said went wrong.
app-client-could-not-save = não foi possível salvar o cliente em { $path }: { $error }
# scripts\build-client.ps1 is a file name.
app-client-not-carried = esta compilação do WinAppMigrator não contém o cliente. Compile-o com scripts\build-client.ps1 e depois compile o aplicativo novamente.
# { $error } is what Windows said went wrong.
app-client-could-not-save-note = não foi possível salvar a nota ao lado dele: { $error }
# A note saved as a text file beside the client, for whoever runs it on the old PC. Keep its lines short, as it is read in
# Notepad. "WinAppMigrator Client.exe" is a file name. "Network PC..." names the Scan Source button: use its words in your
# language.
app-client-note =
    WinAppMigrator Client
    
    Execute isto no PC do qual você está transferindo aplicativos: Windows 7, 8,
    10 ou 11.
    
    1. Feche o aplicativo que você está transferindo.
    2. Clique duas vezes em “WinAppMigrator Client.exe” e permita que ele faça
       alterações.
    3. Ele mostra um código de emparelhamento de 6 dígitos. No WinAppMigrator,
       no seu PC novo, escolha PC na rede..., escolha este PC e digite o código.
    4. Quando o PC novo terminar, pressione Enter na janela do cliente ou
       feche-a.
    
    Ele apenas lê. Enquanto é executado, ele mantém um instantâneo da unidade do
    Windows e libera as duas portas dele no Firewall do Windows para a sua rede
    local, e remove os dois quando é interrompido. O que passa pela rede é
    criptografado, e só um PC que tenha o código pode ler alguma coisa.
