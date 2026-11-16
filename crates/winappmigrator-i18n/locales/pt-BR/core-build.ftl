# Words the core writes for people while it builds a package from an installed app: what a package records for an
# installer to show later, errors shown in the app, the list of what goes in and what is left out, opening a disk
# image or another PC over the network, and what an installer says when the package it carries cannot be read.
# Message names start with core-.


## Scanning an app, before a package is built. Shown in the app when a scan cannot go on.

# { $path } is the app's program, such as D:\Program Files\Acme\Acme.exe, on a drive taken from another PC.
core-scan-program-not-on-the-drive = { $path } passa por um link ou não está na unidade conectada, então não foi lido.
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-not-on-this-pc = { $path } não está em um disco deste PC ou tem um nome que abriria outra coisa, então não foi lido.
# { $path } is the app's program.
core-scan-program-someone-elses = { $path } está nas pastas de outra conta ou não pertence a esta conta, e os próprios dados de instalação deste aplicativo não dizem que ele pertence a esse local, então não foi lido.
# { $app_name } is the app's name, as the list of apps calls it.
core-scan-no-executable = não foi possível encontrar um executável para { $app_name }. Um driver, runtime ou suplemento pode não ter um programa próprio para empacotar.
# { $path } is the file that was picked.
core-scan-not-a-file = o destino da análise não é um arquivo: { $path }
# Names something left out of a package, in a list of what was left out and why. { $name } is the variable's name,
# such as ACME_LICENSE_KEY.
core-scan-environment-variable-item = A variável de ambiente { $name }

## Folders and parts that might also be the app's, offered in a list to tick. Each says why it is offered.

core-near-starts-with-app-name = O nome começa com o nome deste aplicativo.
core-near-has-app-name = O nome contém o nome deste aplicativo.
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-starts-with-publisher = O nome começa com o nome do editor, { $publisher }.
# { $folder } is a folder's name, such as Program Files.
core-near-hidden-part-beside = Uma parte oculta da lista de aplicativos, instalada ao lado deste aplicativo em { $folder }.
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-hidden-part-publisher = Uma parte oculta da lista de aplicativos, do mesmo editor, { $publisher }.

## What a scan found, as a person chooses from it: the folders that go in, and what is left out, and why

# Why a folder cannot be left out, beside its number of files.
core-footprint-locked-app-folder = Estes sempre vão com o programa principal
core-footprint-locked-main-program = Contém o programa principal, que sempre entra
core-footprint-locked-needed = Contém aquilo sem o que o programa principal não inicia
core-footprint-locked-font = Vai com a fonte dele, em O que o integra ao Windows
# Where a folder is. "AppData" and "ProgramData" are the names of Windows folders; keep them as they are.
core-footprint-place-roaming = Configurações no seu perfil (AppData)
core-footprint-place-local = Configurações deste PC no seu perfil (AppData)
core-footprint-place-program-data = Compartilhado por todos no PC (ProgramData)
core-footprint-place-documents = Em Documentos
core-footprint-place-saved-games = Em Jogos Salvos
core-footprint-place-public-documents = Em Documentos Públicos
core-footprint-place-app-folder = Na pasta do aplicativo
# { $folder } is a folder, such as C:\Users\Sam\AppData\Roaming.
core-footprint-place-in = Em { $folder }
# A row for the files directly in the app's folder. { $folder } is that folder's name, such as Acme.
core-footprint-files-directly-in = Arquivos diretamente em { $folder }
# Headings over what is left out, and why each item is.
core-footprint-licenses-left-out = Fora deste instalador, porque a opção Dados de licença e ativação está desativada
core-footprint-font-stays-behind = Ela não vai junto.
core-footprint-drivers-left-out = Drivers que pertencem ao Windows ou a um fabricante, que não podem ser redistribuídos; reinstale-os no outro PC
core-footprint-driver-reinstall = Reinstale-o a partir do pacote do próprio fabricante.
# Why something else is left out along with what the person left out. { $service } is a service's name, such as AcmeSync.
core-footprint-needs-left-out = precisa de { $service }, que foi deixado de fora
core-footprint-told-to-use-left-out = deve usar algo em uma pasta que você deixou de fora
core-footprint-file-in-left-out = o arquivo está em uma pasta que você deixou de fora
core-footprint-program-in-left-out = o programa a que se destina está em uma pasta que você deixou de fora
core-footprint-names-left-out = indica uma pasta que você deixou de fora
# Why something is left out, as the installer tells whoever installs it.
core-footprint-builder-left-it-out = por escolha de quem criou este instalador
# Something the person left out, by kind and name, such as "Service Acme Updater". { $kind } is the kind, such as
# Service or Scheduled Task; { $title } is its name.
core-footprint-left-out-item = { $kind } { $title }

## Building a package and an installer from a scan. Shown in the app when a build cannot go on.

core-package-scan-of-another-app = esta análise é de outro aplicativo, diferente do que está sendo criado, então nada foi criado
core-package-scan-brought-in-more = esta análise trouxe dados que não foram solicitados para esta criação, então nada foi criado
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-package-program-gone = { $path } não existe mais, então nada foi criado. Analise o aplicativo novamente.
core-package-needs-stub = um arquivo autoextraível exige um stub de instalador
# { $reason } is what went wrong.
core-package-staged-file-failed = um arquivo na pasta de preparação não pôde entrar no pacote: { $reason }
# { $shortfalls } says which drives are short of room and by how much, in sentences of its own.
core-package-not-enough-room = não há espaço suficiente para criar este instalador, então nada foi lido nem gravado. { $shortfalls }. Libere espaço nesse local ou escolha uma pasta de saída em outra unidade.
# { $path } is a file's path, on a drive taken from another PC.
core-package-not-read-off-the-drive = { $path } passa por um link ou não está na unidade conectada, então não foi lido
# { $path } is a file's path.
core-package-not-read-not-local = { $path } não está em um disco deste PC, então não foi lido
# { $path } is a file's path.
core-package-not-read-other-account = { $path } leva às pastas de outra conta, então não foi lido
# { $path } is a file's path, as the PC it came from called it.
core-package-not-on-the-drive = { $path } não está na unidade conectada, então não foi lido
# { $path } is a file's path.
core-package-backup-copy-no-parent = não é possível fazer uma cópia de backup de um arquivo sem diretório pai: { $path }
# { $path } is a file's path.
core-package-backup-copy-destination-no-parent = o destino da cópia de backup não tem diretório pai: { $path }
# robocopy is the name of a Windows command; keep it as it is. { $code } is how it ended, such as Some(8).
core-package-robocopy-failed = falha na cópia do robocopy em modo de backup (saída { $code })
# Stands for a folder where a file's name would be, as in "I/O failed for temporary package folder: …".
core-package-temporary-folder = pasta temporária do pacote
# Stands for the installer being written where a file's name would be, as in "I/O failed for the installer: …".
core-package-the-installer = o instalador
# { $path } is a file's path, or its name in the package. { $error } is what went wrong with it.
core-package-path-error = { $path }: { $error }
# { $name } is a file's name in the package, such as app/bin/App.exe.
core-package-same-place = outro arquivo ocupa o mesmo lugar no pacote: { $name }
# { $name } is a file's name in the package.
core-package-changed-in-protected-folder = { $name } foi alterado na pasta protegida enquanto o pacote era criado
core-package-worker-died = um thread que compactava arquivos parou inesperadamente, então nenhum instalador foi gravado
# { $path } is a file's path.
core-package-link-not-read = { $path } é um link, então não foi lido
# { $path } is a file's path. { $error } is what went wrong.
core-package-not-compressed = { $path } não pôde ser compactado: { $error }
# { $path } is a file's path. { $reason } is what went wrong.
core-package-not-read-back = { $path } não pôde ser relido: { $reason }

## Where an app's files are, and where an installer may write them. Shown when a path is refused.

# { $path } is a folder, such as C:\Program Files\Acme.
core-discovery-root-not-full-path = a pasta em que gravar deve ser um caminho completo: { $path }
# { $path } is a folder's path.
core-discovery-root-not-a-folder = a pasta em que gravar não é uma pasta: { $path }
# { $path } is a file's path.
core-discovery-escapes-root = o caminho sai da raiz pretendida: { $path }
# { $path } is the link's path.
core-discovery-refusing-to-write-through-link = não é permitido gravar através de um link: { $path }
# { $path } is the link's path.
core-discovery-refusing-link = não é permitido passar por um link: { $path }
# { $error } is what Windows said went wrong.
core-discovery-refusing-link-because = não é permitido passar por um link: { $error }
# { $path } is the link's path.
core-discovery-link-to-somewhere-else = { $path } é um link para outro lugar, e outras pessoas podem conseguir alterar para onde ele leva. Use o local para onde ele leva
# { $path } is a folder's path.
core-discovery-no-parent-folder = { $path } não tem pasta pai
# { $path } is the install location as it was given. Keep the quotes, or use your language's own.
core-discovery-install-location-not-full-path = o local de instalação deve ser um caminho completo: “{ $path }”
# { $path } is the install location.
core-discovery-install-location-a-link = o local de instalação é um link para outro lugar: { $path }
# { $path } is a file's path.
core-discovery-device-name = o caminho usa um nome reservado para um dispositivo do Windows: { $path }
# { $path } is a file's path.
core-discovery-device-name-on-older-windows = { $path } tem um nome que esta versão do Windows abre como dispositivo, e não como arquivo, então nada foi gravado nele. Ele pode ser instalado no Windows 11
# { $path } is a file's path.
core-discovery-unsafe-path = caminho não seguro: { $path }
# { $path } is a file's path.
core-discovery-no-package-path = o arquivo não tem caminho relativo no pacote: { $path }

## Writing an installer, and reading the package inside one. Most of these are shown by an installer, on the PC it
## runs on, when it cannot read what it carries. "The package" is the app's files and settings that an installer
## carries; "the stub" is the program part of an installer, before the package; "the data file" is the .wamdata file
## that holds the package of a very large installer, beside it.

# Stands for the package where a file's name would be, as in "I/O failed for installer package: …".
core-archive-installer-package = pacote do instalador
# Stands for the installer program built into WinAppMigrator, where a file's name would be.
core-archive-embedded-stub = stub do instalador incorporado
# { $index } is the number of a file in the package. { $reason } is what went wrong.
core-archive-entry-unreadable = a entrada { $index } do pacote não pôde ser lida: { $reason }
# { $name } is a file's name inside the package, such as app/bin/App.exe.
core-archive-entry-unsafe-path = a entrada do zip tem um caminho não seguro: { $name }
# { $name } is a file's name inside the package.
core-archive-holds-more = { $name } contém mais do que diz conter
# { $name } is a file's name inside the package.
core-archive-name-too-long = este nome é longo demais para um zip: { $name }
core-archive-changed-while-copied = o pacote mudou enquanto era colocado depois do stub, então nenhum instalador foi gravado
core-archive-stub-not-a-program = a parte deste instalador antes do pacote não é um programa
core-archive-damaged = este instalador está danificado ou foi alterado depois de criado, então nada foi lido do pacote dele. Copie-o novamente do local onde ele foi criado.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-missing = este instalador guarda o pacote em um arquivo chamado { $name }, na mesma pasta, e esse arquivo não está lá. Os dois arquivos precisam ficar juntos: copie { $name } para cá do local onde o instalador foi criado e execute-o novamente.
# { $name } is the data file's name, such as App_installer.wamdata. { $error } is what Windows said went wrong.
core-archive-data-file-unopened = o arquivo de dados deste instalador, { $name }, não pôde ser aberto: { $error }. Os dois arquivos precisam ficar juntos, em uma pasta que este PC consiga ler.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-a-link = { $name } é um link ou uma pasta, e o arquivo de dados deste instalador precisa ser um arquivo próprio, logo ao lado dele, então nada foi lido dele. Copie o arquivo real para cá.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-wrong = { $name } não é o arquivo de dados deste instalador, ou está danificado ou foi alterado depois de criado, então nada foi lido dele. Copie os dois arquivos novamente do local onde eles foram criados.
core-archive-stub-signed = o stub do instalador está assinado; ele deve permanecer sem assinatura, então assine o instalador final
# { $size } is a size, such as 4.2 GB.
core-archive-too-big-to-run = o instalador teria { $size }, e o Windows não consegue executar um executável de 4 GB ou mais; nada foi gravado
core-archive-outside-signature = o instalador tem dados fora da assinatura e pode ter sido adulterado
core-archive-footer-length = o tamanho do rodapé do arquivo autoextraível é inválido
core-archive-seek-outside = posicionamento fora do pacote
# { $path } is a file's path.
core-archive-unsafe-package-path = caminho de pacote não seguro: { $path }
# { $path } is a file's path.
core-archive-empty-package-path = caminho de pacote vazio: { $path }

## Opening a disk image of another PC (a .vhd or .vhdx file) to read apps from it

# { $path } is the disk image file, such as D:\Backups\OldPC.vhdx.
core-image-not-attachable = { $path } não é uma imagem de disco que o Windows consiga anexar (.vhd ou .vhdx)
core-image-needs-administrator = anexar uma imagem de disco exige direitos de administrador
# { $path } is the disk image file the person picked.
core-image-not-a-file = { $path } não é um arquivo
# { $path } is the disk image file. "Edit Disk" is the name of a Hyper-V Manager command.
core-image-differencing = { $path } é um disco de diferenciação, que só contém o que mudou desde o disco a partir do qual foi criado, e o Windows abriria esse disco também, onde quer que ele diga que está. Mescle-o primeiro nesse disco, com Editar Disco do Hyper-V, e escolha esse disco
# { $path } is the disk image file.
core-image-unreadable = { $path } não é um .vhd ou .vhdx que pôde ser lido
# ProgramData is the name of a Windows folder; keep it as it is.
core-image-no-program-data = a pasta ProgramData deste PC não foi encontrada
# The reason given in core-image-could-not-attach when Windows already has the image open.
core-image-already-attached = o Windows já está com ela anexada, e o que a anexou pode estar gravando nela. Desanexe-a primeiro ou escolha a unidade dela como unidade conectada
# { $path } is the disk image file. { $reason } is why, such as core-image-already-attached or what Windows said.
core-image-could-not-attach = Não foi possível anexar { $path }: { $reason }
# { $path } is the disk image file.
core-image-no-windows-volume = { $path } não tem nenhum volume legível com o Windows. Um volume criptografado com o BitLocker precisa ser desbloqueado primeiro.
# Given as the reason when Windows failed without saying anything.
core-image-windows-did-not-say = o Windows não informou o motivo

## Reading another PC over the network, through the client running on it. Shown as why something on that PC
## could not be read. "The client" is WinAppMigrator Client, the small program on the other PC.

core-remote-too-slow = o outro PC demorou demais para responder
core-remote-closed = a conexão com o outro PC está fechada
core-remote-failed = a conexão com o outro PC falhou
core-remote-answered-something-else = o cliente respondeu outra coisa
# { $name } is the PC's name or address as the person typed it, such as OLDPC or 192.168.1.20.
core-remote-not-found = { $name } não foi encontrado na rede
core-remote-did-not-say-what-it-is = o cliente não disse o que é
core-remote-ordinary-names-only = só são consultados caminhos abaixo da raiz do outro PC, com nomes comuns
core-remote-more-but-none = o outro PC disse que uma pasta tinha mais itens, mas não enviou nenhum deles
# { $count } is the most things one folder may hold, such as 250,000.
core-remote-too-many-listed = o outro PC listou mais de { $count } itens em uma pasta
core-remote-sent-too-much = o outro PC enviou mais dados de um arquivo do que foi solicitado
core-remote-name-not-text = não é possível consultar um nome que não é texto
core-remote-below-root-only = só é possível consultar um caminho abaixo da raiz do outro PC
core-remote-file-ended-early = o arquivo do outro PC terminou antes do esperado
core-remote-link-not-followed = um link em outro PC não é seguido
core-remote-only-files-opened = só arquivos são abertos em outro PC
core-remote-listing-said-nothing = a listagem não dizia nada sobre ele
core-remote-change-time-unknown = não se sabe quando ele foi alterado
core-remote-seek-before-start = um posicionamento antes do início
# { $size } is how many bytes a file is; { $limit } is the most that were expected. Both are numbers.
core-remote-too-large =
    { $size ->
        [one] ele tem { $size } byte, e não eram esperados mais de { $limit }
       *[other] ele tem { $size } bytes, e não eram esperados mais de { $limit }
    }

## Checking that a download is signed by Microsoft. Each is given as the reason a check failed.

# { $error } is what Windows said went wrong.
core-signature-no-powershell = não foi possível iniciar o PowerShell: { $error }
# { $error } is what went wrong reading PowerShell's answer.
core-signature-unreadable = não foi possível ler a assinatura: { $error }
# { $signer } is who signed the file, such as Contoso Ltd.
core-signature-not-microsoft = ele está assinado por { $signer }, e não pela Microsoft
core-signature-not-microsoft-unnamed = ele está assinado por terceiros, e não pela Microsoft
# { $status } is Windows' own word for the signature's state, such as HashMismatch or NotSigned, which is not translated.
core-signature-status = o status da assinatura dele é { $status }

## Warnings a package records when it is built from an app, which an installer shows later, maybe on another PC, in
## that PC's language. "This app" is the app being packaged.

core-warning-build-program-needs-unreadable = Não foi possível ler o que isto precisa do Windows, então não será verificado se ele pode ser executado no outro PC.
core-warning-build-external-manifest = Manifesto de aplicativo externo detectado; a interpretação de dependências lado a lado foi adiada.
core-warning-build-non-exe = O arquivo selecionado não usa a extensão .exe; a análise PE continuará.
core-warning-build-stand-in-program = Nenhum executável comum do aplicativo foi encontrado, então este arquivo o substitui. Os resultados de arquitetura e dependências descrevem este arquivo e podem não descrever o aplicativo.
core-warning-build-other-apps-files-unknown = Nem tudo o que o Windows Installer tem de outros aplicativos pôde ser lido, então uma pasta que consta nos dados deste aplicativo pode conter arquivos de outro aplicativo sem que nada aqui indique isso.
core-warning-build-app-folder-has-another-apps-file = O Windows Installer diz que outro aplicativo tem um arquivo na pasta em que está o programa deste aplicativo, e ele vem junto, já que a pasta inteira é incluída.
# "App Paths" is the name of a Windows registry key; keep it as it is.
core-warning-build-app-paths-folder-left-out = A entrada App Paths deste aplicativo diz que ele procura em uma pasta que não é dele, então essa pasta não vem junto. Ele pode não iniciar em um PC sem o que há lá.
core-warning-build-uninstaller-folder-left-out = A pasta em que está o desinstalador deste aplicativo foi deixada de fora, porque o Windows Installer diz que outro aplicativo tem um arquivo lá.
# { $why } is never shown: it chooses the reason. Translate each reason as the end of the sentence.
core-warning-build-recorded-folder-left-out =
    Algo que consta nos dados deste aplicativo indica uma pasta que foi deixada de fora, porque { $why ->
        [unreadable] o caminho dela sai da unidade ou passa por um link, então não é seguro lê-la
        [missing] ela não existe
        [link] ela passa por um link
        [short-name] ela não está escrita com o nome verdadeiro dela
        [another-account] ela está nas pastas de outra conta
        [one-account] este aplicativo foi instalado para uma conta, e esta pasta não é dessa conta
        [shared] ela contém o que é compartilhado ou faz parte do Windows
        [generic-name] o nome dela não diz nada sobre a quem ela pertence
        [publisher] ela tem apenas o nome do editor e pode conter outros produtos dele
        [holds-the-app] ela contém a própria pasta deste aplicativo e pode conter mais
        [another-apps-name] ela leva o nome de outro aplicativo
        [another-apps-record] os dados de instalação dela são de outro aplicativo
        [in-another-apps-folder] ela está na pasta de outro aplicativo
       *[another-apps] ela é de outro aplicativo
    }.
core-warning-build-app-folder-holds-another-app = Outro aplicativo deste PC consta como estando dentro da pasta deste aplicativo e vem junto com ele, já que a pasta inteira é incluída.
# "Program Files (x86)" is the name of a Windows folder; keep it as Windows calls it.
core-warning-build-from-32-bit-windows = Este aplicativo estava em um Windows de 32 bits. As chaves do Registro dele são organizadas como o Windows de 64 bits mantém as de um aplicativo de 32 bits, e o instalador oferece Program Files (x86) para ele.
# { $dll } is a file's name, such as MSVCP140D.dll.
core-warning-build-debug-runtime = Um programa precisa de { $dll }, uma versão de depuração do runtime do Visual C++. A Microsoft não tem instalador para essas versões e pode não permitir que elas sejam redistribuídas, então ele não será executado em um PC sem o Visual Studio.
# { $program } is a program's file name, such as Tool.exe.
core-warning-build-single-file-program = { $program } é um programa .NET empacotado em um único arquivo, que não diz de qual .NET precisa, então nenhum foi previsto para ele. Se ele precisar de um, não iniciará em um PC que não o tenha.
# { $count } is how many of the app's programs.
core-warning-build-programs-not-inspected =
    { $count ->
        [one] { $count } dos programas do aplicativo não foi examinado em busca dos runtimes de que precisa, porque o aplicativo tem programas demais ou eles são grandes demais.
       *[other] { $count } dos programas do aplicativo não foram examinados em busca dos runtimes de que precisam, porque o aplicativo tem programas demais ou eles são grandes demais.
    }
# { $runtime } is a runtime's name, such as Java Runtime Environment.
core-warning-build-runtime-no-installer = O aplicativo precisa de { $runtime }, e não há instalador para ele aqui, então nenhum é executado. Instale-o você mesmo no outro PC.
# { $why } is why something was left out, such as "its name says what it holds".
core-warning-build-left-out-of-package = Deixado de fora do pacote: { $why }.
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-missing-dependency = Não foi possível resolver a dependência: { $name }
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-dependency-only-here = { $name } só foi encontrado pelo caminho de pesquisa deste PC e não faz parte do aplicativo, então não vem junto. O aplicativo pode não iniciar em um PC sem ele.
core-warning-build-driver-detected = Driver ou componente de kernel detectado. Um driver que pertence ao aplicativo é incluído e registrado novamente; um driver de fabricante ou nativo do Windows não é redistribuído e deve ser reinstalado a partir do pacote assinado do fabricante.
core-warning-build-near-no-longer-offered = Algo marcado na lista do que também pode ser deste aplicativo não está mais nessa lista, então não foi adicionado. Veja a lista novamente.
# { $app_name } is the name of a part of the app, as the list of apps calls it. { $why } is never shown: it chooses the
# reason. Translate each reason as the end of the sentence.
core-warning-build-near-part-left-out =
    Parte do que { $app_name } diz ser dele foi deixada de fora: { $why ->
        [unreadable] o caminho dela sai da unidade ou passa por um link, então não é seguro lê-la
        [missing] ela não existe
        [link] ela passa por um link
        [short-name] ela não está escrita com o nome verdadeiro dela
        [another-account] ela está nas pastas de outra conta
        [one-account] este aplicativo foi instalado para uma conta, e ela não é dessa conta
        [shared] ela contém o que é compartilhado ou faz parte do Windows
        [generic-name] o nome dela não diz nada sobre a quem ela pertence
        [publisher] ela tem apenas o nome do editor e pode conter outros produtos dele
        [holds-the-app] ela contém a própria pasta deste aplicativo e pode conter mais
        [another-apps-name] ela leva o nome de outro aplicativo
        [another-apps-record] os dados de instalação dela são de outro aplicativo
        [in-another-apps-folder] ela está na pasta de outro aplicativo
       *[another-apps] ela é de outro aplicativo
    }.
# { $app_name } is the name of a part of the app, as the list of apps calls it.
core-warning-build-near-keys-left-out = As chaves do Registro listadas nos dados de instalação de { $app_name } foram deixadas de fora, pois esses dados também indicam o que é de outro aplicativo.
# "Windows keeps what an app wrote for one account" is Windows' VirtualStore folder.
core-warning-build-virtual-store-program = Havia um programa na pasta em que o Windows guarda o que um aplicativo gravou para uma conta. O Windows nunca coloca um programa lá, então outra coisa o colocou, e ele foi deixado de fora.
# { $path } is a file or folder, such as C:\Users\Sam\AppData\Roaming\Acme.
core-warning-build-file-left-out = { $path } foi deixado de fora: ele passa por um link em uma unidade conectada ou pertence a uma conta diferente daquela cujos dados estão sendo incluídos.
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as D:\Sam\Documents.
core-warning-build-user-folder-not-on-drive = A pasta { $folder } de { $user } estava em { $path }, que não fica nesta unidade, então o que há lá não foi incluído.
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as \\server\home\Sam\Documents.
core-warning-build-user-folder-not-on-this-pc = A pasta { $folder } de { $user } estava em { $path }, que não fica em um disco deste PC, então o que há lá não foi incluído.
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-base-unreadable = Não foi possível ler { $path } ({ $error }); os dados de aplicativo armazenados lá não foram incluídos.
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-folder-unreadable = Não foi possível ler { $path } ({ $error }); podem faltar dados de aplicativo em subpastas dela.
# { $error } is what went wrong.
core-warning-build-entry-unreadable = Não foi possível ler uma entrada do sistema de arquivos: { $error }
# { $count } is how many links. { $names } lists some of them. { $more } is how many more there are, which are not
# named; when it is 0, the sentence ends after the list.
core-warning-build-links-not-followed =
    { $count ->
        [one] { $count } link, ou nome que abriria outra coisa, não foi seguido nesta pasta, então aquilo para o que ele aponta não está no instalador: { $names }
       *[other] { $count } links, ou nomes que abririam outra coisa, não foram seguidos nesta pasta, então aquilo para o que eles apontam não está no instalador: { $names }
    }{ $more ->
        [0] .
       *[other] , e mais { $more }.
    }
# { $count } is how many folders. { $depth } is how many levels down a folder can be. { $names } lists some of them.
# { $more } is how many more there are, which are not named; when it is 0, the sentence ends after the list.
core-warning-build-folders-too-deep =
    { $count ->
        [one] { $count } pasta está a mais de { $depth } níveis de profundidade, então o que há nela não está no instalador: { $names }
       *[other] { $count } pastas estão a mais de { $depth } níveis de profundidade, então o que há nelas não está no instalador: { $names }
    }{ $more ->
        [0] .
       *[other] , e mais { $more }.
    }
# { $error } is what went wrong.
core-warning-build-metadata-unreadable = Não foi possível ler os metadados do arquivo: { $error }
core-warning-build-reserved-name = O nome deste arquivo é reservado para um dispositivo do Windows, então ele não foi copiado.
# { $error } is what went wrong.
core-warning-build-hash-failed = Não foi possível calcular o hash do arquivo: { $error }
core-warning-build-cloud-placeholder = Arquivo de nuvem somente online (por exemplo, Arquivos Sob Demanda do OneDrive) sem dados locais; escolha Sempre manter neste dispositivo ou baixe-o antes do backup. Relatado, mas não copiado.
# { $file_name } is a driver's file name, such as acmeflt.sys.
core-warning-build-driver-file = O arquivo de driver/kernel “{ $file_name }” não é copiado como um arquivo comum. Se ele servir a um serviço de driver registrado deste aplicativo, o instalador o inclui e registra o driver novamente de forma automática; caso contrário, reinstale-o com o pacote de driver assinado do fabricante.
core-warning-build-file-gone = Isto existia quando o aplicativo foi analisado e não existe mais, então não está no instalador.
# { $error } is what went wrong.
core-warning-build-file-copy-failed = Não foi possível copiar o arquivo para o pacote: { $error }
# { $count } is how many files. { $names } lists them.
core-warning-build-not-looked-through =
    { $count ->
        [one] Este pacote tem um arquivo que não pôde ser examinado em busca de indícios da origem dele: { $names }. O conteúdo dele pode revelá-la.
       *[other] Este pacote tem arquivos que não puderam ser examinados em busca de indícios da origem deles: { $names }. O conteúdo deles pode revelá-la.
    }
# { $count } is the most empty folders an installer makes, such as 10,000.
core-warning-build-empty-folders-truncated = Este aplicativo tem mais de { $count } pastas vazias, e só essa quantidade será recriada.
core-warning-build-credential-by-now = Deixado de fora do pacote: agora ele parece uma credencial.
core-warning-build-no-package-path = Arquivo copiável sem caminho relativo no pacote.
# { $file_name } is the runtime installer's file name, such as vc_redist.x64.exe. { $error } is what went wrong.
core-warning-build-prerequisite-path-invalid = Não foi possível colocar o pré-requisito incluído { $file_name }: { $error }
# { $runtime } is a runtime's name, such as Microsoft Visual C++ 2015+ Redistributable (x64). { $error } is what went
# wrong. winget is the name of a Windows command; keep it as it is.
core-warning-build-prerequisite-download-failed = Não foi possível baixar { $runtime } ({ $error }); o instalador tentará usar o winget no destino.
# { $runtime } is a runtime's name. { $reason } is why the check failed, such as "it is signed by Contoso Ltd, not
# Microsoft".
core-warning-build-prerequisite-signature-failed = O download de { $runtime } falhou na verificação de assinatura, então não foi incluído: { $reason }. O instalador tentará usar o winget no destino.
# { $runtime } is a runtime's name. { $error } is what went wrong.
core-warning-build-prerequisite-hash-failed = { $runtime } foi baixado, mas não foi possível calcular o hash dele ({ $error }); o instalador tentará usar o winget no destino.
# { $service } is a driver's service name, such as AcmeFlt. { $error } is what went wrong.
core-warning-build-driver-copy-failed = Não foi possível incluir a imagem do driver de “{ $service }”: { $error }
core-warning-build-license-not-copied = Não foi possível copiar os dados de licença (ausentes ou ilegíveis); registrados somente como metadados.
core-warning-build-license-file-credential = Um arquivo de licença parece uma credencial e foi deixado de fora.
# { $error } is what went wrong.
core-warning-build-license-store-walk-failed = Não foi possível enumerar parte de um repositório de licenças: { $error }
core-warning-build-license-store-credential = Um arquivo em um repositório de licenças parece uma credencial e foi deixado de fora.
# { $error } is what went wrong.
core-warning-build-license-store-file-skipped = Um arquivo de um repositório de licenças foi ignorado: { $error }
