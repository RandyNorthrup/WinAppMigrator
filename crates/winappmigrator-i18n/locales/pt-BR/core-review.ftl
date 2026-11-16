# Words the core says about what an install may do: why an install location or something in a package is refused,
# the installer's list of what an app sets up beyond its files, the checks of a package and of an install, and whether
# there is room for either. Message names start with core-.
#
# Many messages here are a reason, the second half of a sentence that starts "... was left out:" or "that install
# location cannot be used:", so they start with a small letter and have no full stop.


## Why an install location cannot be used. Shown in red under the Install Location field.

core-review-location-state-root = é onde as instalações guardam os históricos
core-review-location-has-files = já contém arquivos, e nenhuma instalação consta aqui, então use uma pasta vazia
core-policy-location-not-full-path = deve ser um caminho completo
core-policy-location-parent-step = não deve conter “..”
core-policy-location-not-plain = precisa ser um caminho simples em uma unidade deste PC
core-policy-location-whole-drive = não pode ser uma unidade inteira
core-policy-location-windows = não pode ser a pasta do Windows
core-policy-location-runs-by-itself = é uma pasta que pertence ao Windows ou cujo conteúdo é executado ou carregado automaticamente
core-policy-location-shared = contém outros itens lado a lado, então o aplicativo precisa de uma subpasta

## Whether an app can run on this PC at all, said before anything is installed.

# { $reason } is why, as the second half of a sentence, such as "App.exe is built for Arm64, and this PC is x64, so
# Windows would not start it".
core-review-cannot-run-here = este aplicativo não pode ser executado neste PC, então nada foi alterado: { $reason }
core-policy-architecture-x86 = x86 de 32 bits
core-policy-architecture-x64 = x64
core-policy-architecture-arm64 = Arm64
core-policy-architecture-arm32 = Arm de 32 bits
core-policy-architecture-unknown = uma arquitetura desconhecida
# { $code } is the number Windows gives a kind of processor, such as 0x01c4.
core-policy-machine-type = tipo de máquina { $code }
# { $names } is a list of program names, such as "a.exe, b.exe, c.exe", and { $count } how many more there are.
core-policy-names-and-more = { $names } e mais { $count }
core-policy-fit-programs-not-said = Este instalador não diz do que os programas dele precisam, então não foi verificado se eles podem ser executados neste PC.
core-policy-fit-architecture-not-told = Não foi possível determinar a arquitetura deste PC, então não foi verificado se os programas deste aplicativo podem ser executados nele.
core-policy-fit-main-not-said = Este instalador não diz do que o programa principal dele precisa, então o que é dito aqui se baseia no restante do que ele contém.
# { $driver } is a driver's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-driver = o driver { $driver } dele foi compilado para { $built_for }, e este PC é { $here }. Um driver precisa corresponder exatamente ao Windows, então este aplicativo não pode ser executado aqui
# { $program } is a program's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-program = { $program } foi compilado para { $built_for }, e este PC é { $here }, então o Windows não o iniciaria
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-none-start = nenhum dos programas dele pode ser iniciado neste PC, que é { $here }: { $programs }
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-some-other-architecture = Alguns programas deste aplicativo foram compilados para outra arquitetura e não serão iniciados neste PC, que é { $here }: { $programs }. O restante do aplicativo pode ser iniciado, então isso talvez não importe.
# { $here } is a kind of processor, such as "Arm64". { $programs } is a list of program names.
core-policy-fit-emulated = Este PC é { $here }, então o Windows executaria estes programas por meio de um emulador: { $programs }. A maioria dos programas funciona bem assim, mas alguns não.
# { $program } is a program's file name. { $needs } and { $runs } are versions of Windows, such as 10.0 and 6.1.
core-policy-fit-needs-newer = { $program } diz que precisa do Windows { $needs } ou mais recente, e este PC executa a versão { $runs }, então o Windows não o iniciaria
# { $programs } is a list of program names.
core-policy-fit-some-need-newer = Alguns programas deste aplicativo dizem precisar de um Windows mais recente que este e não serão iniciados aqui: { $programs }.
# { $from_build } and { $this_build } are Windows build numbers, such as 22631, written as Windows writes them.
core-policy-fit-older-build = Este aplicativo estava na compilação { $from_build } do Windows, e este PC executa a compilação { $this_build }. Os programas dele dizem que podem ser iniciados neste Windows, então ele pode funcionar, mas aquilo de que ele dependia lá pode não estar aqui.
core-policy-fit-build-not-told = Não foi possível determinar qual compilação do Windows é esta, então ela não foi comparada com aquela em que este aplicativo estava.

## Room on a drive for a build or an install.

# Each of these ends the sentence core-space-shortfall, after "free for".
core-space-for-unpacking = a descompactação do pacote
core-space-for-install-record = o histórico da instalação
core-space-for-app-files = os arquivos do aplicativo
core-space-for-what-is-replaced = o backup do que a instalação substitui
core-space-for-build-work = os arquivos de trabalho da criação
core-space-for-installer = o instalador
core-space-for-package = o pacote
core-space-for-unpacking-and-installing = a descompactação e a instalação do pacote
# What goes between two of those, as in "unpacking the package and the app's files". Keep the quotes: they keep the
# spaces.
core-space-purposes-joiner = { " e " }
# { $drive } is a drive, such as C:, or a folder. { $needed } and { $free } are sizes, such as 1.2 GB. { $purposes } is
# what the room is for, such as "unpacking the package and the app's files".
core-space-shortfall = { $drive } precisa de até { $needed } livres para { $purposes } e tem { $free }
# { $drive } is a drive, such as C:, or a folder. { $needed } is a size, such as 1.2 GB.
core-space-not-told = { $drive } não informou quanto espaço tem, então não foi verificado se os { $needed } necessários cabem
# One drive in a list of where room is needed. { $needed } and { $free } are sizes, and { $drive } a drive, such as C:.
core-space-summary = até { $needed } em { $drive } ({ $free } livres)
# { $needed } is a size, and { $drive } a drive, such as C:.
core-space-summary-not-told = até { $needed } em { $drive } (não foi possível saber quanto está livre)
# What goes between two sentences of core-space-shortfall inside core-review-no-room. Keep the quotes: they keep the
# space.
core-review-sentence-joiner = { ". " }
# { $shortfalls } is one or more sentences of core-space-shortfall.
core-review-no-room = não há espaço suficiente para isto, então nada foi alterado. { $shortfalls }. Libere espaço nesse local ou escolha um local de instalação em outra unidade.

## The installer's list of what an app sets up beyond its files: what each kind is called.

core-review-kind-service = Serviço
core-review-kind-driver = Driver
core-review-kind-scheduled-task = Tarefa agendada
core-review-kind-autostart = Inicia automaticamente
core-review-kind-shortcut = Atalho
core-review-kind-path = PATH
core-review-kind-environment-variable = Variável de ambiente
core-review-kind-shell-extension = Extensão do shell
core-review-kind-menu-command = Comando de menu
core-review-kind-link-type = Tipo de link
core-review-kind-file-type = Tipo de arquivo
core-review-kind-com = COM
core-review-kind-firewall-rule = Regra de firewall
core-review-kind-event-log-source = Origem do log de eventos
core-review-kind-font = Fonte
core-review-kind-odbc-driver = Driver ODBC
core-review-kind-odbc-source = Fonte de dados ODBC
core-review-kind-compatibility-setting = Configuração de compatibilidade
core-review-kind-default-apps = Aplicativos padrão
core-review-kind-program-name = Nome do programa
core-review-kind-other-settings = Outras configurações

## The installer's list of what an app sets up: what each line says of it.

# { $file } is a program's file name. { $account } is the Windows account it runs as, such as LocalSystem. { $starts }
# says when it starts, and is not shown itself; for "other", { $start } is one of the core-review-start- words below.
core-review-service-detail =
    { $starts ->
        [automatic] Executa { $file } como { $account }, inicia agora e sempre que este PC é iniciado
        [delayed] Executa { $file } como { $account }, inicia agora e logo após cada inicialização deste PC
        [manual] Executa { $file } como { $account }, inicia só quando algo o solicita
        [disabled] Executa { $file } como { $account }, está desativado
        [other] Executa { $file } como { $account }, início: { $start }
       *[unknown] Executa { $file } como { $account }
    }
# { $account } is a Windows account, such as LocalService.
core-policy-account-custom = { $account } (conta personalizada não restaurada)
# { $file } is a driver's file name.
core-review-driver-detail = Carrega { $file } no próprio Windows
# { $file } is a driver's file name. { $start } is when Windows starts it: one of the core-review-start- words below.
core-review-driver-detail-starts = Carrega { $file } no próprio Windows, início: { $start }
# When a service or driver starts, ending a sentence such as "Loads disk.sys into Windows itself, starts …".
core-review-start-boot = com o Windows, antes de qualquer outra coisa
core-review-start-system = com o Windows, durante a inicialização dele
core-review-start-automatic = sempre que este PC é iniciado
core-review-start-delayed = logo após cada inicialização deste PC
core-review-start-manual = só quando algo o solicita
core-review-start-disabled = nunca, pois está desativado
core-review-start-unknown = de um modo que o Windows não informa
# { $account } is the account a task runs as: an account's name, or core-review-task-runs-as-you, or
# core-review-task-runs-as-creator.
core-review-task-detail = É executada como { $account }
# { $account } is the account a task runs as, as for core-review-task-detail.
core-review-task-detail-highest = É executada como { $account }, com privilégios mais altos
core-review-task-runs-as-you = você, quem estiver conectado
core-review-task-runs-as-creator = o usuário que a criou
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-autostart-detail =
    É executado { $scope ->
        [everyone] para todos que entrarem
       *[you] quando você entrar
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shortcut-startup-detail =
    Um atalho na pasta Inicializar, { $scope ->
        [everyone] para todos neste PC
       *[you] para você
    }
# { $file } is the file name of what the shortcut opens. { $scope } is whom it is set up for: everyone on this PC,
# or you. It is not shown itself.
core-review-shortcut-detail =
    Abre { $file }, { $scope ->
        [everyone] para todos neste PC
       *[you] para você
    }
# { $name } is an environment variable's name, such as PATH.
core-review-path-title = Adiciona a { $name }
core-review-path-install-folder = A pasta de instalação
# { $path } is a folder inside the install folder, such as bin\tools.
core-review-path-below-install-folder = A pasta de instalação\{ $path }
# { $value } is the variable's value, as it will be set.
core-review-variable-detail = Definida como { $value }
# { $count } is how many registry values.
core-review-other-settings-title =
    { $count ->
        [0] { $count } configurações fora das chaves do próprio aplicativo
        [one] { $count } configuração fora das chaves do próprio aplicativo
       *[other] { $count } configurações fora das chaves do próprio aplicativo
    }
# { $keys } is a list of registry keys, such as HKCU\Software\Vendor.
core-review-other-settings-detail = Em { $keys }
# { $keys } is a list of registry keys, and { $count } how many more there are.
core-review-other-settings-detail-more = Em { $keys } e mais { $count }
# { $way } says whether the rule allows or blocks, and incoming or outgoing connections; it is not shown itself.
# { $ports } is a list of port numbers, and { $port_count } how many. { $program } is the program's file name.
# { $networks } is a list of the kinds of network Windows names in English (domain, private, public), and
# { $network_count } how many.
# The kinds of network a firewall rule is for, as Windows Defender Firewall names its profiles in your language. They
# fill { $networks } in core-review-firewall-detail, joined into a list.
core-review-network-domain = Domínio
core-review-network-private = Privado
core-review-network-public = Público
core-review-firewall-detail =
    { $way ->
        [allow-in] Permite conexões de entrada
        [allow-out] Permite conexões de saída
        [block-in] Bloqueia conexões de entrada
       *[block-out] Bloqueia conexões de saída
    }{ $port_count ->
        [0] { "" }
       *[other] { " " }nas portas { $ports }
    } para { $program }, { $network_count ->
        [0] em todas as redes
        [one] no perfil de rede { $networks }
       *[other] nos perfis de rede { $networks }
    }
# { $server } is the file name of what Explorer loads. { $scope } is whom it is set up for: everyone on this PC, or
# you. It is not shown itself.
core-review-shell-extension-detail =
    Carrega { $server } no Explorador de Arquivos, { $scope ->
        [everyone] para todos neste PC
       *[you] para você
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shell-extension-detail-own-file =
    Carrega no Explorador de Arquivos um arquivo que ele instala, { $scope ->
        [everyone] para todos neste PC
       *[you] para você
    }
# The name of a shell extension: { $handler } is its name in the registry, and { $class } the kind of file or folder it
# is for, such as * or Directory.
core-policy-shell-extension-name = { $handler } em { $class }
# { $on } is the kind of file or folder whose menu it is, as the registry names it, such as * or
# Directory. { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-menu-command-detail =
    Adiciona um comando ao menu de { $on }, { $scope ->
        [everyone] para todos neste PC
       *[you] para você
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-default-verb-detail =
    Altera o que acontece ao abrir itens deste tipo, { $scope ->
        [everyone] para todos neste PC
       *[you] para você
    }
core-review-client-entry-detail = Pode ser escolhido como o programa para isto. Nada passa a usá-lo automaticamente
core-review-com-permissions-title = Permissões COM
core-review-com-permissions-detail = Define quem pode iniciar as classes COM dele ou com qual conta elas são executadas
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-link-type-detail =
    Links deste tipo abrem com o aplicativo, { $scope ->
        [everyone] para todos neste PC
       *[you] para você
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-file-type-detail =
    Arquivos deste tipo abrem com o aplicativo, { $scope ->
        [everyone] para todos neste PC
       *[you] para você
    }
# { $log } is the name of a Windows event log, such as Application.
core-review-event-log-source-detail = Pode gravar no log { $log }, com mensagens dos próprios arquivos
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-font-detail =
    É carregada da pasta de instalação quando você entra, { $scope ->
        [everyone] para todos neste PC
       *[you] para você
    }
core-review-odbc-driver-detail = É carregado em programas que usam ODBC para acessar um banco de dados
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-odbc-source-detail =
    Uma conexão de banco de dados, { $scope ->
        [everyone] para todos neste PC
       *[you] para você
    }
core-review-default-apps-detail = Pode ser escolhido em Aplicativos padrão. Nada passa a usá-lo automaticamente
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-program-name-detail =
    A caixa Executar e outros programas podem encontrá-lo por este nome, { $scope ->
        [everyone] para todos neste PC
       *[you] para você
    }
# { $server } is the file name of what serves the classes.
core-review-com-title = COM em { $server }
core-review-com-title-own-file = COM em um arquivo que ele instala
# { $count } is how many registry values.
core-review-com-detail =
    { $count ->
        [0] { $count } configurações das classes que outros programas podem iniciar
        [one] { $count } configuração das classes que outros programas podem iniciar
       *[other] { $count } configurações das classes que outros programas podem iniciar
    }
core-review-com-support-title = Bibliotecas de tipos, interfaces e AppIDs COM
# { $libraries }, { $interfaces }, and { $app_ids } are how many registry values there are for its type libraries, its
# interfaces, and its AppIDs.
core-review-com-support-detail = Configurações das bibliotecas de tipos ({ $libraries }), das interfaces ({ $interfaces }) e dos AppIDs ({ $app_ids }) que acompanham as classes dele
core-review-compatibility-title = Como os programas dele são iniciados
# { $count } is how many registry values.
core-review-compatibility-detail =
    { $count ->
        [0] { $count } configurações de compatibilidade para programas que ele instala
        [one] { $count } configuração de compatibilidade para programas que ele instala
       *[other] { $count } configurações de compatibilidade para programas que ele instala
    }

## The installer's notes on whose folders and settings these are, and what is not coming.

# { $account } is the name of a Windows account, such as alice.
core-review-account-signed-in = Os dados do usuário e as configurações por usuário vão para { $account }, que está conectado aqui, e não para o administrador com o qual isto está sendo executado.
# { $account } is the name of a Windows account, such as alice.
core-review-account-no-desktop = Ninguém está conectado nesta sessão, então os dados do usuário e as configurações por usuário vão para { $account }, a conta com a qual isto está sendo executado.
# { $reason } is what Windows said, in its own words.
core-review-account-not-told = Não foi possível determinar quem está conectado ({ $reason }), então as configurações por usuário são deixadas de fora, e os dados do usuário vão para o local de instalação.
# { $reason } is what Windows said, in its own words.
core-review-signed-in-unknown = não foi possível determinar quem está conectado: { $reason }
# Headings of what is left out, shared by the app's list and the installer's notes.
core-review-left-out-credentials = Fora deste instalador, porque parecem credenciais
core-review-left-out-settings = Configurações e histórico que uma conta mantém para o aplicativo, que só entram com a opção Dados do usuário
core-review-left-out-certificates = O aplicativo parece depender de certificados que eram confiáveis no PC dele, e eles não são instalados
core-review-left-out-fonts = Estas fontes têm o nome do aplicativo, e nada indica que sejam dele, então não são instaladas
# { $names } is a list of names, such as the names of saved passwords.
core-review-note-credentials = Fora deste instalador, porque parecem credenciais: { $names }. O que houver delas neste PC continua como está, e talvez você precise digitá-las novamente.
# { $names } is a list of names of license data.
core-review-note-licenses = Fora deste instalador, porque a opção Dados de licença e ativação estava desativada quando ele foi criado: { $names }.
# { $names } is a list of places an account keeps settings, such as folders.
core-review-note-settings = Fora deste instalador, porque a opção Dados do usuário estava desativada quando ele foi criado: as configurações e o histórico que uma conta mantém para o aplicativo, em { $names }. O aplicativo inicia aqui como iniciaria para alguém que nunca o usou.
# { $names } is a list of certificates, each as core-review-certificate.
core-review-note-certificates = O aplicativo parece depender de certificados que eram confiáveis no PC dele, e eles não são instalados: { $names }. Cabe a você decidir se deve confiar neles aqui.
# { $names } is a list of font names.
core-review-note-fonts = Estas fontes têm o nome do aplicativo, e nada indica que sejam dele, então não são instaladas: { $names }.
# { $names } is a list of names.
core-review-note-left-out-by-choice = Deixado de fora por quem criou este instalador: { $names }.
# { $names } is a list of names.
core-review-note-added-by-choice = Adicionado por quem criou este instalador, por considerar que faz parte do aplicativo: { $names }.
# { $name } is a certificate's subject, such as CN=Contoso, and { $store } the certificate store it was in, such as
# Root.
core-review-certificate = { $name } em { $store }
# { $names } is a list of names, and { $count } how many more there are.
core-review-names-and-more = { $names } e mais { $count }

## Why something in a package is left out. Each is the second half of a sentence of core-warning-review-left-out.

core-review-no-account = pertence às configurações de uma conta, e não foi possível determinar quem está conectado
core-review-name-not-accepted = o Windows não aceitaria o nome
# { $error } is what Windows said, in its own words.
core-review-service-unreadable = não foi possível ler as configurações atuais: { $error }
core-review-service-dependency = depende de um serviço com nome inválido
# { $path } is the program the service runs.
core-review-service-runs-other = executa { $path }, que este pacote não instala
core-review-service-settings = as configurações indicam algo que este pacote não instala, ou uma chave que não é do próprio serviço
core-review-driver-no-hash = não tem um hash com o qual ser verificado
core-review-license-for-the-machine = uma licença armazenada para a máquina inteira é apenas relatada
core-review-license-starts-something = uma licença não tem motivo para iniciar nada
core-review-task-not-captured = a definição não foi capturada
# { $error } is what was wrong with the task's XML, in English.
core-review-task-unreadable = não foi possível ler a definição: { $error }
core-review-task-no-account = é executada como quem a instalar, e não foi possível determinar quem está conectado aqui
# { $path } is what the shortcut opens.
core-review-shortcut-opens-other = abre { $path }, que este pacote não instala
# { $service } is the service's name, and { $path } the program it runs now.
core-policy-service-exists = { $service } já existe neste PC e executa { $path }, que não é de uma instalação anterior feita aqui. Se for uma cópia mais antiga deste aplicativo, desinstale-a primeiro
core-policy-key-control-characters = o nome da chave contém caracteres de controle
core-policy-value-control-characters = o nome do valor contém caracteres de controle
core-policy-registry-link = é um link do Registro
# { $reason } is why the key it stands for is left out, such as "no app install should change this key".
core-policy-virtual-store = representa uma chave do PC, e { $reason }
core-policy-registry-part = nenhuma instalação de aplicativo deveria gravar nesta parte do Registro
core-policy-key-refused = nenhuma instalação de aplicativo deveria alterar esta chave
core-policy-net-debugger = definiria um depurador para todos os programas .NET
core-policy-net-install-root = alteraria o local de onde todos os programas .NET carregam o .NET
core-policy-class-redirects = alteraria como um tipo de programa ou pasta é aberto
core-policy-app-path-name = indica um programa por algo que não é o nome de um arquivo
core-policy-app-path-not-installed = registraria um nome de programa para um arquivo que este pacote não instala
core-policy-shared-default-verb = alteraria o que acontece ao abrir qualquer arquivo ou pasta de um tipo
core-policy-command-runs-other = adiciona um comando que executa algo que este pacote não instala
core-policy-autostart-not-command = inicia automaticamente, e o valor não é um comando
core-policy-autostart-runs-other = inicia automaticamente e executa algo que este pacote não instala
core-policy-environment-own-rules = as variáveis de ambiente são restauradas por regras próprias
core-policy-service-not-restored = pertence a um serviço que este pacote não restaura
core-policy-service-own-rules = as configurações próprias de um serviço só são restauradas pelas regras dele
core-policy-service-dll = o ServiceDll não é um arquivo que este pacote instala
core-policy-event-log-source-only = só os valores próprios de uma origem do log de eventos podem ser restaurados
core-policy-security-log = nenhum aplicativo grava no log de Segurança
core-policy-event-log-messages = as mensagens viriam de um arquivo que este pacote não instala
core-policy-type-library = a biblioteca de tipos não é um arquivo que este pacote instala
# { $name } is a program's file name, such as app.exe.
core-policy-application-opened = alteraria como { $name } é aberto, e este pacote não o instala
core-policy-event-log-no-file = uma origem do log de eventos precisa obter as mensagens de um arquivo
core-policy-com-runs-other = executa algo que não é um arquivo que este pacote instala
core-policy-com-redirect = transformaria esta classe em algo que este pacote não instala
core-policy-com-server = executa algo que não é um arquivo que este pacote instala nem parte do Windows
core-policy-interface = decidiria como são feitas as chamadas a uma interface, com algo que este pacote não instala
# { $name } is a program's file name, such as app.exe.
core-policy-app-id-started = alteraria como { $name } é iniciado, e este pacote não o instala
core-policy-app-id-surrogate = seria hospedado por um programa que este pacote não instala
core-policy-app-id-service = seria executado em um serviço que este pacote não restaura
core-policy-app-id-run-as = seria executado com uma conta própria
core-policy-app-id-link = não indica um AppID
core-policy-app-id-remote = seria executado em outro lugar que não este PC
core-policy-odbc-own-driver = essa é uma configuração do próprio ODBC, e não um driver
core-policy-odbc-driver-not-installed = o driver não é um arquivo que este pacote instala
core-policy-loads-other-file = carregaria um arquivo que este pacote não instala
core-policy-odbc-own-source = essa é uma configuração do próprio ODBC, e não uma fonte de dados
core-policy-odbc-source-missing = lista uma fonte de dados que este pacote não tem
core-policy-odbc-source-driver = o driver não é um arquivo que este pacote instala nem parte do Windows
core-policy-shell-extension = carregaria no Explorador de Arquivos uma extensão do shell que não é um arquivo que este pacote instala
core-policy-not-text = o conteúdo não é um texto válido no formato em que seria lido
core-policy-network-place = indica um local na rede
core-policy-font = registraria uma fonte que este pacote não instala, ou uma que está na pasta do Windows
core-policy-compatibility = alteraria como é executado um programa que este pacote não instala
core-policy-capabilities = lista um aplicativo cujos detalhes não estão neste pacote
core-policy-default-program = alteraria qual programa abre itens por padrão
core-policy-url-protocol = diz ser um tipo de link e não abre com nada que este pacote instala
core-policy-command-handler = adiciona um comando tratado por algo que este pacote não instala
core-policy-installer-command = executaria o que só o Windows Installer consegue ler
core-policy-creates-class = criaria uma classe que este pacote não instala
core-policy-opens-links = abre links com algo que este pacote não instala
core-policy-opens-files = abre arquivos com algo que este pacote não instala
core-policy-firewall-name = o nome não pode ser repassado
core-policy-firewall-description = a descrição não pode ser repassada
core-policy-firewall-every-program = aplica-se a todos os programas, e uma regra só pode ser para um
# { $path } is the program the rule is for.
core-policy-firewall-program = é para { $path }, que este pacote não instala
core-policy-firewall-direction = não indica uma direção que possa ser definida
core-policy-firewall-action = substituiria outras regras, o que a regra de nenhum aplicativo deveria fazer
# { $setting } is a part of a firewall rule, as the scan of its PC described it.
core-policy-firewall-not-carried = diz { $setting }, o que não pôde ser definido como estava ou enfraqueceria o que outras regras fazem
core-policy-firewall-service = é para um serviço que este pacote não restaura
core-policy-firewall-parts = parte do conteúdo não é algo que uma regra de firewall possa expressar
core-policy-folder-already-there = essa pasta já existia, fora do local de instalação, e nada indica que seja deste aplicativo, então nenhum programa vai para ela
core-policy-variable-name = o nome não é um nome de variável válido
core-policy-variable-relied-on = o Windows e outros programas dependem desta variável
core-policy-task-name = o nome não é aceito pelo Windows
core-policy-task-folder = a pasta não é aceita pelo Windows
core-policy-task-windows = substituiria uma das tarefas do próprio Windows
core-policy-task-other-action = faz algo além de executar um programa
core-policy-task-no-program = não indica nenhum programa para executar
# { $path } is the program the task runs.
core-policy-task-runs-other = executa algo que este pacote não instala: { $path }
core-policy-task-exists-unreadable = já existe uma tarefa com este nome, e ela não pôde ser lida
core-policy-task-exists-other = já existe uma tarefa com este nome, e ela não executa o aplicativo
# { $path } is the program the task already on this PC runs.
core-policy-task-exists-runs = já existe uma tarefa com este nome, e ela executa { $path }, que não é de uma instalação anterior feita aqui
core-policy-shortcut-not-lnk = não é um atalho .lnk
core-policy-shortcut-place = não está em um menu Iniciar nem em uma área de trabalho
core-policy-short-name = o caminho usa um nome que não pode ser verificado
core-policy-shortcut-no-folder = este PC não tem essa pasta
core-policy-data-startup = iria para uma pasta Inicializar e seria executado a cada entrada no Windows
core-policy-data-powershell = iria para onde o PowerShell executa o que encontra ao iniciar
core-policy-data-windows = iria para uma pasta que pertence ao Windows
core-policy-license-outside = está fora do local de instalação e de todas as pastas de dados conhecidas
core-rewrite-cdata = contém uma seção CDATA
core-rewrite-too-much-to-look-through = este pacote contém dados demais nas configurações para que o que elas indicam seja verificado

## Warnings a review gives, which the installer shows with what it sets up.

# { $kind } says what was left out, and is not shown itself. { $item } is its path or name, or a registry value such
# as HKLM\Software\App\Name. { $reason } is why, as the second half of the sentence, such as "it cannot be a whole
# drive".
core-warning-review-left-out =
    { $kind ->
        [file] O arquivo { $item } foi deixado de fora: { $reason }.
        [folder] A pasta { $item } foi deixada de fora: { $reason }.
        [service] O serviço { $item } foi deixado de fora: { $reason }.
        [driver] O driver { $item } foi deixado de fora: { $reason }.
        [firewall-rule] A regra de firewall { $item } foi deixada de fora: { $reason }.
        [license-data] Os dados de licença em { $item } foram deixados de fora: { $reason }.
        [license-value] O valor de licença { $item } foi deixado de fora: { $reason }.
        [scheduled-task] A tarefa agendada { $item } foi deixada de fora: { $reason }.
        [shortcut] O atalho { $item } foi deixado de fora: { $reason }.
        [environment-variable] A variável de ambiente { $item } foi deixada de fora: { $reason }.
       *[value] { $item } foi deixado de fora: { $reason }.
    }
core-warning-review-package-path-missing = Um arquivo copiável não tem caminho relativo no pacote.
# { $count } is the most empty folders a package makes.
core-warning-review-too-many-empty-folders = Este pacote lista mais de { $count } pastas vazias, e só essa quantidade é criada.
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Documents folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-documents-elsewhere =
    { $count ->
        [one] { $count } arquivo de Documentos vai, em vez disso, para { $target }, em uma pasta chamada _external, porque a pasta Documentos deste PC está em { $folder }, e só são gravados arquivos em pastas dentro do seu perfil. Para usá-lo, mova-o você mesmo para a sua pasta Documentos.
       *[other] { $count } arquivos de Documentos vão, em vez disso, para { $target }, em uma pasta chamada _external, porque a pasta Documentos deste PC está em { $folder }, e só são gravados arquivos em pastas dentro do seu perfil. Para usá-los, mova-os você mesmo para a sua pasta Documentos.
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-documents-unknown =
    { $count ->
        [one] { $count } arquivo de Documentos vai, em vez disso, para { $target }, em uma pasta chamada _external, porque não foi possível determinar onde fica a pasta Documentos deste PC. Para usá-lo, mova-o você mesmo para a sua pasta Documentos.
       *[other] { $count } arquivos de Documentos vão, em vez disso, para { $target }, em uma pasta chamada _external, porque não foi possível determinar onde fica a pasta Documentos deste PC. Para usá-los, mova-os você mesmo para a sua pasta Documentos.
    }
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Saved Games folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-elsewhere =
    { $count ->
        [one] { $count } arquivo de Jogos Salvos vai, em vez disso, para { $target }, em uma pasta chamada _external, porque a pasta Jogos Salvos deste PC está em { $folder }, e só são gravados arquivos em pastas dentro do seu perfil. Para usá-lo, mova-o você mesmo para a sua pasta Jogos Salvos.
       *[other] { $count } arquivos de Jogos Salvos vão, em vez disso, para { $target }, em uma pasta chamada _external, porque a pasta Jogos Salvos deste PC está em { $folder }, e só são gravados arquivos em pastas dentro do seu perfil. Para usá-los, mova-os você mesmo para a sua pasta Jogos Salvos.
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-unknown =
    { $count ->
        [one] { $count } arquivo de Jogos Salvos vai, em vez disso, para { $target }, em uma pasta chamada _external, porque não foi possível determinar onde fica a pasta Jogos Salvos deste PC. Para usá-lo, mova-o você mesmo para a sua pasta Jogos Salvos.
       *[other] { $count } arquivos de Jogos Salvos vão, em vez disso, para { $target }, em uma pasta chamada _external, porque não foi possível determinar onde fica a pasta Jogos Salvos deste PC. Para usá-los, mova-os você mesmo para a sua pasta Jogos Salvos.
    }

## Errors when a folder that only administrators may change cannot be made so, or checked.

# { $path } is a folder.
core-privilege-not-a-folder = { $path } é um link ou um arquivo, e não uma pasta
# { $path } is a folder. { $error } is what Windows said, in its own words.
core-privilege-not-made-here = { $path } não foi criado pelo WinAppMigrator, e outras pessoas podem alterar o conteúdo dele. Um administrador deve verificá-lo e removê-lo: { $error }
# { $path } is a folder.
core-privilege-others-put-something-in = { $path } contém algo que não foi criado pelo WinAppMigrator, e outras pessoas podem alterá-lo. Um administrador deve verificá-lo e removê-lo
# { $path } is a folder.
core-privilege-became-a-link-in-use = { $path } se tornou um link enquanto estava em uso
# { $path } is a folder.
core-privilege-became-a-link-looked-at = { $path } se tornou um link enquanto era examinado
# { $path } is a file. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-users-cannot-start = não foi possível permitir que os usuários iniciem { $path }: { $said }
core-privilege-elevation-unknown = não foi possível determinar se este processo está elevado
# { $path } is a folder. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-cannot-restrict = não foi possível restringir { $path } aos administradores: { $said }
# { $path } is a folder. { $said } is the folder's permissions as Windows wrote them, and what else it printed.
core-privilege-not-restricted = { $path } não pôde ser restrito aos administradores, e a unidade dele talvez não mantenha permissões: { $said }
# { $path } is a folder.
core-privilege-written-to-first = outro processo gravou em { $path } antes que ele pudesse ser protegido
core-privilege-no-program-data = o Windows não informa onde fica a pasta ProgramData

## Checks of a package's files, and of what an install put in place. Each is said of one file, value, service, or the
## like, which the report names beside it.

# { $path } is the installer's file.
core-verify-no-package = { $path } não contém nenhum pacote
# { $path } is a folder.
core-verify-not-unpacked = o pacote deste instalador não foi descompactado em { $path }
core-verify-schema-supported = O esquema do manifesto do pacote é compatível.
core-verify-packaged-file-missing = O arquivo do pacote está ausente.
core-verify-no-package-path = O arquivo copiável não tem caminho relativo no pacote.
core-verify-folder-there = A pasta vazia existe.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-unreadable = Não foi possível lê-lo para verificá-lo: { $error }
core-verify-restored-file-missing = O arquivo restaurado está ausente.
# { $expected } and { $actual } are sizes in bytes.
core-verify-size-mismatch = O tamanho do arquivo não corresponde. Esperado: { $expected } bytes; encontrado: { $actual } bytes.
core-verify-no-hash = O manifesto não registra nenhum hash para este arquivo, então ele não pode ser verificado.
core-verify-hash-mismatch = O hash SHA-256 não corresponde.
core-verify-file-matches = O tamanho e o hash do arquivo correspondem ao manifesto.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-payload-failed = Falha na verificação dos dados da licença: { $error }
core-verify-license-payload-missing = Os dados da licença no pacote estão ausentes.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-failed = Falha na verificação do repositório de licenças: { $error }
core-verify-license-store-file-missing = O arquivo do repositório de licenças no pacote está ausente.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-path-invalid = O caminho do repositório de licenças no pacote é inválido: { $error }
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-prerequisite-failed = Falha na verificação do instalador do pré-requisito: { $error }
core-verify-prerequisite-missing = O instalador do pré-requisito incluído está ausente.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-failed = Falha na verificação da imagem do driver: { $error }
core-verify-bundled-driver-missing = A imagem do driver incluída está ausente.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-path-invalid = O caminho do driver incluído é inválido: { $error }
core-verify-prerequisite-present = O pré-requisito de runtime está presente no destino.
core-verify-prerequisite-not-detected = Pré-requisito de runtime não detectado; talvez seja necessário reiniciar ou concluir manualmente (veja no log de restauração o resultado definitivo).
core-verify-prerequisite-not-probed = A presença do pré-requisito não pode ser verificada pelo Registro; a instalação foi feita na base do melhor esforço.
core-verify-license-file-missing = O arquivo de licença restaurado está ausente.
core-verify-license-store-restored-missing = O arquivo do repositório de licenças restaurado está ausente.
core-verify-registry-key-there = A chave do Registro existe.
core-verify-registry-value-matches = O valor do Registro corresponde aos dados arquivados.
core-verify-registry-value-differs = O valor do Registro não corresponde aos dados arquivados.
core-verify-firewall-rule-there = O Firewall do Windows tem esta regra.
core-verify-variable-matches = O valor da variável de ambiente corresponde aos dados arquivados.
core-verify-variable-differs = O valor da variável de ambiente não corresponde aos dados arquivados.
# { $error } is what Windows said, in its own words.
core-verify-variable-unreadable = A variável de ambiente está ausente ou ilegível: { $error }
core-verify-service-runs-other = O serviço existe, mas não executa o binário restaurado com os argumentos dele.
core-verify-service-other-account = O serviço existe, mas não é executado com a conta esperada.
core-verify-service-matches = O serviço existe e executa o binário restaurado com os argumentos e a conta dele.
core-verify-service-missing = O serviço está ausente.
# { $error } is what Windows said, in its own words.
core-verify-service-unreadable = O serviço está ilegível: { $error }
core-verify-driver-other-image = O serviço do driver não aponta para a imagem restaurada.
core-verify-driver-not-registered = O serviço do driver incluído não está registrado após a restauração.
# { $error } is what Windows said, in its own words.
core-verify-driver-unreadable = O serviço do driver está ilegível: { $error }
core-verify-driver-image-missing = A imagem do driver está ausente.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-unchecked = Não foi possível verificar a imagem do driver: { $error }
core-verify-task-matches = A tarefa agendada existe e executa os programas restaurados.
core-verify-task-runs-other = A tarefa agendada existe, mas não executa os programas restaurados.
core-verify-task-missing = A tarefa agendada está ausente.
# { $error } is what Windows said, in its own words.
core-verify-task-unreadable = Falha na consulta da tarefa agendada: { $error }
core-verify-shortcut-matches = O atalho existe, e o caminho de destino corresponde ao destino restaurado.
core-verify-shortcut-opens-other = O atalho existe, mas o caminho de destino não corresponde ao destino restaurado.
core-verify-shortcut-missing = O atalho está ausente.
# { $error } is what Windows said, in its own words.
core-verify-shortcut-unreadable = Falha na consulta do atalho: { $error }

## Warnings and errors a check gives, which the app and the installer show.

core-warning-verify-schema-unsupported = O esquema do manifesto do pacote não é compatível.
# { $error } is what went wrong, in Windows' or the program's own words.
core-warning-verify-package-file-failed = Falha na verificação de um arquivo do pacote: { $error }
core-warning-verify-folder-missing = A pasta vazia está ausente.
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-key-missing = A chave do Registro está ausente ou ilegível: { $error }
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-value-missing = O valor do Registro está ausente ou ilegível: { $error }
core-warning-verify-registry-kind-missing = O tipo do valor do Registro está ausente.
core-warning-verify-firewall-rule-missing = O Firewall do Windows não tem esta regra.
core-warning-verify-service-scope = A verificação de serviços lê a configuração dos serviços em HKLM e não inicia serviços.
core-warning-verify-driver-manual = Um ou mais drivers de fabricante ou nativos do Windows não são redistribuídos pelo WinAppMigrator; reinstale-os no destino com o pacote de driver assinado do fabricante.
core-warning-verify-task-scope = A verificação de tarefas agendadas confere somente se a tarefa está registrada e qual comando ela indica.

## The verification report written beside a package, verification_report.md. Its headings follow a "#".

core-verify-report-title = Relatório de verificação do WinAppMigrator
# { $time } is when, as a date and time in UTC, such as 2026-11-15T10:00:00Z.
core-verify-report-verified = Verificado (UTC): { $time }
# { $valid } is true or false, which are not translated.
core-verify-report-package-valid = Pacote válido: { $valid }
# { $valid } is true or false, which are not translated.
core-verify-report-restore-valid = Restauração válida: { $valid }
core-verify-report-restore-not-run = Restauração válida: não executada
core-verify-report-items = Itens
# { $message } is what a check found, and { $source } the file or setting it looked at.
core-verify-report-item-from = { $message } Origem: { $source }
core-verify-report-warnings = Avisos
core-verify-report-errors = Erros
core-verify-report-none = Nenhum.
