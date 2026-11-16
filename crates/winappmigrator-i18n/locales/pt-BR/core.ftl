# Words the core writes for people: warnings, refusals, errors, and the installer's list of what it sets up.
# Message names start with core-.


## Warnings a package records when it is built, which an installer shows later, maybe on another PC, in that PC's
## language.

# { $count } is how many shortcuts.
core-warning-shortcuts-unreadable =
    { $count ->
        [one] { $count } atalho nesta unidade não indica nenhum arquivo, então não foi possível saber se ele abre este aplicativo.
       *[other] { $count } atalhos nesta unidade não indicam nenhum arquivo, então não foi possível saber se eles abrem este aplicativo.
    }

## Undoing an install: what an uninstall, or a failed install, says it put back, left, or could not undo. Each line is
## shown on its own, in a list.

# { $app_name } is the app's name.
core-rollback-record-damaged = Parte do histórico de { $app_name } não pôde ser lida, então tudo o que essa parte descrevia continua no lugar, e um arquivo substituído pode não voltar.
core-rollback-settings-entry-stays = A entrada do aplicativo em Configurações permanece, para que isto possa ser executado novamente a partir de lá.
# { $name } is a runtime's name, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-rollback-prerequisite-stays = { $name } foi instalado, ou pode ter sido, e permanece, porque outros programas podem estar usando-o.
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-drive-not-connected = O que esta instalação colocou em { $folder } está em uma unidade que não está conectada. Conecte-a, e isto poderá ser concluído.
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-record-part-unread = O que esta instalação colocou em { $folder } permanece, porque essa parte do histórico não pôde ser lida.

## Why a file or folder was left, shown after "was left:" in the messages below them. Each is part of a sentence, so
## starts with a small letter and has no full stop.

# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-being-changed = algo estava alterando { $folder } enquanto isto era executado, então nada mais foi feito nessa pasta. Feche o que está fazendo isso e execute isto novamente
# { $folder } is a folder; { $error } is what Windows said went wrong.
core-rollback-reason-not-opened = não foi possível abrir { $folder }: { $error }
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-no-record = não há informação de onde { $folder } estava
# { $folder } is a folder; { $actual } is the folder it now leads to.
core-rollback-reason-moved = { $folder } agora leva a { $actual }, e não ao local onde estava durante a instalação
# Why a service was not stopped.
core-rollback-reason-did-not-stop = não parou a tempo

## Files and folders

# { $path } is a folder; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-folder-left = A pasta { $path } foi mantida: { $reason }.
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-removed = Pasta removida: { $path }
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-holds-others = A pasta { $path } permanece, porque contém arquivos que esta instalação não colocou lá.
# { $path } is a folder; { $error } is what Windows said went wrong.
core-rollback-folder-not-removed = Não foi possível remover a pasta { $path }: { $error }.
# { $path } is a file; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-file-left = O arquivo { $path } foi mantido: { $reason }.
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-folder-gone = { $path } não está sendo reposto, porque a pasta em que ele estava deixou de existir.
# { $path } is a file; { $error } is what went wrong.
core-rollback-partial-not-removed = Não foi possível remover um arquivo parcial: { $path } ({ $error }).
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-checked = Não foi possível verificar { $path }: { $error }.
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-removed = Removido: { $path }
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-removed = Não foi possível remover { $path }: { $error }.
# { $path } is a file this install had replaced, and has put back.
core-rollback-file-put-back = Reposto: { $path }
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-put-back = Não foi possível repor { $path }: { $error }.
# Why a file could not be put back.
core-rollback-backup-gone = o backup deixou de existir
# { $path } is a file.
core-rollback-no-parent-folder = { $path } não tem pasta pai

## A file that stays. { $why } says why: still-used (something that stays still uses it), changed (it has changed since
## it was installed), or not-asked (it is someone's data, and they did not ask for it to go). { $path } is the file.

core-rollback-file-stays =
    { $why ->
        [still-used] { $path } permanece, porque algo que permanece ainda o usa.
        [changed] { $path } permanece, porque foi alterado desde que foi instalado.
       *[not-asked] { $path } permanece, já que a remoção dele não foi solicitada.
    }
# The file it replaced was to be put beside it, under another name, and every name was taken.
core-rollback-file-stays-no-name-beside =
    { $why ->
        [still-used] { $path } permanece, porque algo que permanece ainda o usa, e o arquivo que ele substituiu não pôde ser colocado ao lado dele: todos os nomes possíveis já estão em uso.
        [changed] { $path } permanece, porque foi alterado desde que foi instalado, e o arquivo que ele substituiu não pôde ser colocado ao lado dele: todos os nomes possíveis já estão em uso.
       *[not-asked] { $path } permanece, já que a remoção dele não foi solicitada, e o arquivo que ele substituiu não pôde ser colocado ao lado dele: todos os nomes possíveis já estão em uso.
    }
# { $name } is the name the file it replaced now has, such as settings.ini.before-install.
core-rollback-file-stays-beside =
    { $why ->
        [still-used] { $path } permanece, porque algo que permanece ainda o usa. O arquivo que ele substituiu está ao lado dele, como { $name }.
        [changed] { $path } permanece, porque foi alterado desde que foi instalado. O arquivo que ele substituiu está ao lado dele, como { $name }.
       *[not-asked] { $path } permanece, já que a remoção dele não foi solicitada. O arquivo que ele substituiu está ao lado dele, como { $name }.
    }
# { $error } is what went wrong.
core-rollback-file-stays-not-beside =
    { $why ->
        [still-used] { $path } permanece, porque algo que permanece ainda o usa, e o arquivo que ele substituiu não pôde ser colocado ao lado dele: { $error }.
        [changed] { $path } permanece, porque foi alterado desde que foi instalado, e o arquivo que ele substituiu não pôde ser colocado ao lado dele: { $error }.
       *[not-asked] { $path } permanece, já que a remoção dele não foi solicitada, e o arquivo que ele substituiu não pôde ser colocado ao lado dele: { $error }.
    }

## The registry and environment variables. { $kind } says what { $name } is: key (a registry key, such as
## HKLM\Software\App), value (a registry value, such as HKLM\Software\App\Version), user-variable or machine-variable
## (an environment variable of the user's or of the whole PC, such as APP_HOME), or user-list or machine-list (a
## variable that is a list of folders, such as Path). { $error } is what Windows said went wrong.

# How a registry key's unnamed value is shown, as Registry Editor shows it.
core-registry-default-value-name = (Padrão)
core-rollback-setting-other-account =
    { $kind ->
        [key] A chave { $name } permanece, porque pertence à conta que instalou isto, que não está conectada.
        [user-variable] A variável de ambiente do usuário { $name } permanece, porque pertence à conta que instalou isto, que não está conectada.
        [machine-variable] A variável de ambiente do sistema { $name } permanece, porque pertence à conta que instalou isto, que não está conectada.
        [user-list] A variável { $name } do usuário permanece, porque pertence à conta que instalou isto, que não está conectada.
        [machine-list] A variável { $name } do sistema permanece, porque pertence à conta que instalou isto, que não está conectada.
       *[value] O valor do Registro { $name } permanece, porque pertence à conta que instalou isto, que não está conectada.
    }
core-rollback-setting-not-reached =
    { $kind ->
        [key] Não foi possível acessar a chave { $name }: { $error }.
        [user-variable] Não foi possível acessar a variável de ambiente do usuário { $name }: { $error }.
        [machine-variable] Não foi possível acessar a variável de ambiente do sistema { $name }: { $error }.
        [user-list] Não foi possível acessar a variável { $name } do usuário: { $error }.
        [machine-list] Não foi possível acessar a variável { $name } do sistema: { $error }.
       *[value] Não foi possível acessar o valor do Registro { $name }: { $error }.
    }
core-rollback-setting-not-read =
    { $kind ->
        [key] Não foi possível ler a chave { $name }: { $error }.
        [user-variable] Não foi possível ler a variável de ambiente do usuário { $name }: { $error }.
        [machine-variable] Não foi possível ler a variável de ambiente do sistema { $name }: { $error }.
        [user-list] Não foi possível ler a variável { $name } do usuário: { $error }.
        [machine-list] Não foi possível ler a variável { $name } do sistema: { $error }.
       *[value] Não foi possível ler o valor do Registro { $name }: { $error }.
    }
core-rollback-setting-changed =
    { $kind ->
        [user-variable] A variável de ambiente do usuário { $name } permanece, porque foi alterada desde que foi instalada.
        [machine-variable] A variável de ambiente do sistema { $name } permanece, porque foi alterada desde que foi instalada.
        [user-list] A variável { $name } do usuário permanece, porque foi alterada desde que foi instalada.
        [machine-list] A variável { $name } do sistema permanece, porque foi alterada desde que foi instalada.
       *[value] O valor do Registro { $name } permanece, porque foi alterado desde que foi instalado.
    }
# Said when it is back as it was before the install.
core-rollback-setting-put-back =
    { $kind ->
        [user-variable] Variável de ambiente do usuário reposta: { $name }
        [machine-variable] Variável de ambiente do sistema reposta: { $name }
        [user-list] Variável do usuário reposta: { $name }
        [machine-list] Variável do sistema reposta: { $name }
       *[value] Valor do Registro reposto: { $name }
    }
core-rollback-setting-not-put-back =
    { $kind ->
        [user-variable] Não foi possível repor a variável de ambiente do usuário { $name }: { $error }.
        [machine-variable] Não foi possível repor a variável de ambiente do sistema { $name }: { $error }.
        [user-list] Não foi possível repor a variável { $name } do usuário: { $error }.
        [machine-list] Não foi possível repor a variável { $name } do sistema: { $error }.
       *[value] Não foi possível repor o valor do Registro { $name }: { $error }.
    }
# A registry value the install wrote, which is the user's data, and stays unless they asked for it to go.
core-rollback-value-kept-by-request = O valor do Registro { $name } permanece, já que a remoção dele não foi solicitada.
core-rollback-key-holds-others = A chave { $name } permanece, porque contém o que esta instalação não colocou lá.
core-rollback-key-removed = Chave removida: { $name }
core-rollback-key-not-removed = Não foi possível remover a chave { $name }: { $error }.
# { $kind } is user-list or machine-list.
core-rollback-list-entries-removed =
    { $kind ->
        [user-list] Entradas desta instalação removidas da variável { $name } do usuário
       *[machine-list] Entradas desta instalação removidas da variável { $name } do sistema
    }
core-rollback-list-entries-not-removed =
    { $kind ->
        [user-list] Não foi possível remover as entradas desta instalação da variável { $name } do usuário: { $error }.
       *[machine-list] Não foi possível remover as entradas desta instalação da variável { $name } do sistema: { $error }.
    }

## Services and drivers. { $kind } is service or driver; { $name } is its name, such as AppUpdater. { $error } and
## { $reason } are what Windows said went wrong.

core-rollback-service-not-read =
    { $kind ->
        [driver] Não foi possível ler o driver { $name }: { $error }.
       *[service] Não foi possível ler o serviço { $name }: { $error }.
    }
core-rollback-service-changed =
    { $kind ->
        [driver] O driver { $name } permanece, porque foi alterado desde que foi instalado.
       *[service] O serviço { $name } permanece, porque foi alterado desde que foi instalado.
    }
core-rollback-service-not-stopped =
    { $kind ->
        [driver] Não foi possível parar o driver { $name } ({ $reason }), então o que ele está usando pode permanecer até o PC ser reiniciado.
       *[service] Não foi possível parar o serviço { $name } ({ $reason }), então o que ele está usando pode permanecer até o PC ser reiniciado.
    }
core-rollback-service-stopped-for-this =
    { $kind ->
        [driver] O driver { $name } foi parado para fazer isto. Inicie-o novamente quando precisar dele.
       *[service] O serviço { $name } foi parado para fazer isto. Inicie-o novamente quando precisar dele.
    }
core-rollback-service-not-removed =
    { $kind ->
        [driver] Não foi possível remover o driver { $name }: { $reason }.
       *[service] Não foi possível remover o serviço { $name }: { $reason }.
    }
core-rollback-service-marked-for-removal =
    { $kind ->
        [driver] O driver { $name } está marcado para remoção e será removido quando o PC for reiniciado ou quando o que o mantém aberto o liberar.
       *[service] O serviço { $name } está marcado para remoção e será removido quando o PC for reiniciado ou quando o que o mantém aberto o liberar.
    }
core-rollback-service-removed =
    { $kind ->
        [driver] Driver removido: { $name }
       *[service] Serviço removido: { $name }
    }
core-rollback-service-not-restored =
    { $kind ->
        [driver] Não foi possível repor o driver { $name } como estava: { $reason }.
       *[service] Não foi possível repor o serviço { $name } como estava: { $reason }.
    }
core-rollback-service-restored =
    { $kind ->
        [driver] Configurações do driver repostas: { $name }
       *[service] Configurações do serviço repostas: { $name }
    }
core-rollback-service-not-restarted = O serviço { $name } estava em execução antes desta instalação e não foi iniciado novamente: { $reason }.
# { $account } is the account the service ran as, such as CONTOSO\backup.
core-rollback-service-account-again = O serviço { $name } era executado como { $account }. A senha dele não é guardada, então defina a conta dele novamente em Serviços.

## Firewall rules. { $name } is the rule's name; { $error } is what went wrong.

core-rollback-firewall-not-read = Não foi possível ler a regra de firewall { $name }: { $error }.
core-rollback-firewall-says-otherwise = A regra de firewall { $name } permanece, porque a regra com esse nome que está lá diz outra coisa.
core-rollback-firewall-changed = A regra de firewall { $name } permanece, porque foi alterada desde que foi instalada.
# The capital "The" is as the app has always said it.
core-rollback-firewall-removed = Regra de firewall removida: { $name }
core-rollback-firewall-not-removed = Não foi possível remover a regra de firewall { $name }: { $error }.

## Scheduled tasks. { $path } is a folder of tasks, and { $name } a task with its folder, such as \App\Nightly Update.
## { $error } and { $reason } are what went wrong.

core-rollback-task-folder-holds-others = A pasta de tarefas { $path } permanece, porque contém tarefas que esta instalação não colocou lá.
core-rollback-task-folder-not-removed = Não foi possível remover a pasta de tarefas { $path }: { $error }.
core-rollback-task-not-read = Não foi possível ler a tarefa { $name }: { $error }.
core-rollback-task-changed = A tarefa { $name } permanece, porque foi alterada desde que foi instalada.
core-rollback-task-removed = Tarefa removida: { $name }
core-rollback-task-not-removed = Não foi possível remover a tarefa { $name }: { $reason }.
core-rollback-task-put-back = Tarefa reposta: { $name }
core-rollback-task-not-put-back = Não foi possível repor a tarefa { $name }: { $reason }.

## When an install cannot be undone at all

# { $error } is what went wrong.
core-rollback-record-unreadable = Não foi possível ler o histórico desta instalação, então nada foi desfeito: { $error }.
# { $error } is what went wrong.
core-rollback-nothing-undone = Nada foi desfeito: { $error }.
# One app's part of the message below. { $app_name } is the app's name; { $failures } is what could not be undone, a
# sentence or more; { $folder } is the folder that holds the install's record.
core-rollback-earlier-install-stuck = { $app_name }: { $failures } Se isso nunca puder ser desfeito, um administrador pode desistir dessa instalação removendo { $folder }.
# Why an install did not start. { $details } is one or more of the message above.
core-rollback-earlier-installs-not-undone = uma instalação anterior que não foi concluída não pôde ser totalmente desfeita, então nada foi instalado. Feche o que estiver usando estes itens e tente novamente. { $details }
core-rollback-install-busy = esta instalação está sendo alterada por outro instalador ou desinstalador

## The summary an uninstall writes on the command line, one line each, with what each says below it. { $count } is how
## many.

core-rollback-count-undone = Desfeitos: { $count }
core-rollback-count-failed = Não foi possível desfazer: { $count }
# What was left on purpose: it has changed since the install, or it is the user's.
core-rollback-count-kept = Mantidos no lugar: { $count }
# What no uninstall can take back, such as a runtime that was installed, which other programs may use.
core-rollback-count-left-behind = Deixados para trás: { $count }

## Which Windows a PC or drive has, as the app shows it beside the drive.

# { $name } is the Windows' name, such as Windows 7 Professional Service Pack 1.
core-system-windows-32-bit = { $name }, 32 bits
# { $name } is the Windows' name, such as Windows 11 Pro.
core-system-windows-64-bit = { $name }, 64 bits
# A drive's Windows, when it only says it is older than Vista.
core-system-windows-xp-or-older = Windows XP ou anterior

## Reading a connected drive, or another account's settings. Each is why it could not be done, and some start with a
## small letter, as they follow other words.

core-system-needs-administrator = ler uma unidade conectada ou as configurações de outra conta exige direitos de administrador
core-system-no-program-data = a pasta ProgramData deste PC não foi encontrada
# { $path } is a file, such as D:\Windows\System32\config\SOFTWARE.
core-system-not-a-hive-file = não é um arquivo de hive: { $path }
# { $path } is a file; { $size } is its size in bytes.
core-system-hive-too-large = { $path } tem { $size } bytes, o que é grande demais para um hive do Registro ou mais do que os hives desta unidade poderiam somar
# { $key } is a registry key, such as Software\Microsoft\Windows\CurrentVersion.
core-system-hive-no-mapping = o hive offline não tem mapeamento para HKLM\{ $key }
# { $key } is where a drive's registry was to be loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said.
# "reg load" is a command's name.
core-system-hive-not-loaded = reg load { $key } falhou: { $error }
# { $key } is where a drive's registry was loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said. "reg
# unload" is a command's name.
core-system-hive-not-unloaded = reg unload { $key } falhou: { $error }
# { $drive } is a drive, such as D:\.
core-system-not-a-system-drive = { $drive } não é uma unidade do sistema Windows (não há pasta \Windows).
# { $drive } is a drive, such as C:\. "This PC" is the name of a choice in the app.
core-system-own-system-drive = { $drive } é a própria unidade do sistema deste PC. Escolha Este PC para ler este PC.
# { $drive } is a drive, such as D:\.
core-system-registry-through-link = { $drive } não é uma unidade do sistema Windows: o caminho até o Registro dela passa por um link.

## Warnings a scan records about whose data it read.

# { $account } is an account's name or id, such as alice or S-1-5-21-...-1001.
core-warning-scan-account-not-on-record = A conta { $account } não consta aqui, então nenhum dado ou configuração por usuário foi incluído.
# { $account } is an account's name, such as CONTOSO\alice.
core-warning-scan-signed-in-account-no-profile = A conta conectada aqui, { $account }, não tem perfil registrado, então nenhum dado ou configuração por usuário foi incluído.
# { $reason } is why Windows could not say.
core-warning-scan-signed-in-account-unknown = Não foi possível determinar quem está conectado ({ $reason }), então nenhum dado ou configuração por usuário foi incluído. Escolha de quem incluir os dados.
core-warning-scan-last-account-unknown = Esta unidade não informa quem entrou por último, e ela tem mais de uma conta, então nenhum dado ou configuração por usuário foi incluído. Escolha de quem incluir os dados.
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-settings-unreadable = Não foi possível ler as configurações de { $account }, então elas não foram incluídas: { $error }
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-classes-unreadable = Não foi possível ler as classes COM e os tipos de arquivo registrados para { $account }, então eles não foram incluídos: { $error }

## Paths a scan did not follow. { $count } is how many; { $paths } is some of them, separated by semicolons; { $more } is
## how many more there are, not shown.

core-warning-scan-drive-paths-not-followed =
    { $count ->
        [one] Um caminho registrado nesta unidade não pôde ser lido, então nada do que está lá foi incluído. Ele está em outro disco, não é um caminho completo ou passa por um link que levaria para fora da unidade: { $paths }
       *[other] Alguns caminhos registrados nesta unidade não puderam ser lidos, então nada do que está lá foi incluído. Cada um deles está em outro disco, não é um caminho completo ou passa por um link que levaria para fora da unidade: { $paths }
    }
core-warning-scan-drive-paths-not-followed-more = Alguns caminhos registrados nesta unidade não puderam ser lidos, então nada do que está lá foi incluído. Cada um deles está em outro disco, não é um caminho completo ou passa por um link que levaria para fora da unidade: { $paths } e mais { $more }
core-warning-scan-network-paths-not-followed =
    { $count ->
        [one] Um caminho registrado neste PC não pôde ser lido, então nada do que está lá foi incluído. Ele não está em um disco deste PC, e examiná-lo faria uma conexão com ele, ou tem um nome que abriria outra coisa: { $paths }
       *[other] Alguns caminhos registrados neste PC não puderam ser lidos, então nada do que está lá foi incluído. Cada um deles não está em um disco deste PC, e examiná-lo faria uma conexão com ele, ou tem um nome que abriria outra coisa: { $paths }
    }
core-warning-scan-network-paths-not-followed-more = Alguns caminhos registrados neste PC não puderam ser lidos, então nada do que está lá foi incluído. Cada um deles não está em um disco deste PC, e examiná-lo faria uma conexão com ele, ou tem um nome que abriria outra coisa: { $paths } e mais { $more }

## What a scan with none of an account's data leaves out of a package, in the app's list of what was left out. Each
## has a name, and below it why.

core-account-left-out-shortcuts = Atalhos nas próprias pastas desta conta
# { $count } is how many shortcuts.
core-account-left-out-shortcuts-why =
    { $count ->
        [one] { $count } atalho, que só entra com a opção Dados do usuário
       *[other] { $count } atalhos, que só entram com a opção Dados do usuário
    }
core-account-left-out-variables = Variáveis de ambiente desta conta
# { $count } is how many variables.
core-account-left-out-variables-why =
    { $count ->
        [one] { $count } variável, que só entra com a opção Dados do usuário
       *[other] { $count } variáveis, que só entram com a opção Dados do usuário
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-account-left-out-task = A tarefa agendada { $task }
core-account-task-works-on-account-folder = Ela trabalha em um local nas pastas de uma conta, ou tem o nome de um, e só entra com a opção Dados do usuário
core-account-task-runs-from-account-folder = O que ela também executa a partir das pastas de uma conta só entra com a opção Dados do usuário
# { $service } is a service's name, such as AppUpdater.
core-account-left-out-service = O serviço { $service }
core-account-service-works-on-account-folder = Ele trabalha em um local nas pastas de uma conta e só entra com a opção Dados do usuário

## What a package was found to still say of whose PC it came from, after a build with none of an account's data.

# The parts of a package, as the warning below names them.
core-account-part-files = os caminhos dos próprios arquivos do aplicativo
core-account-part-registry = valores do Registro
core-account-part-settings-entry = a entrada do aplicativo em Configurações
core-account-part-main-program = o local do programa principal
core-account-part-shortcuts = atalhos
core-account-part-tasks = tarefas agendadas
core-account-part-services = serviços
core-account-part-variables = variáveis de ambiente
# One place in the list below. { $place } is one of the parts above, or a file's name; { $count } is how many times.
core-account-mention-place = { $place } ({ $count })
# { $kind } is what was found: account (an account's name), folder (where an account's folder is), computer (the PC's
# name), or id (an account's id). { $places } is a list of where, from the message above.
core-warning-scan-package-mentions =
    { $kind ->
        [account] Este pacote menciona o nome de uma conta em { $places }. Ele foi criado sem os dados de nenhuma conta, e qualquer pessoa que tenha o instalador pode ler isso.
        [folder] Este pacote menciona onde fica a pasta de uma conta em { $places }. Ele foi criado sem os dados de nenhuma conta, e qualquer pessoa que tenha o instalador pode ler isso.
        [computer] Este pacote menciona o nome do PC em { $places }. Ele foi criado sem os dados de nenhuma conta, e qualquer pessoa que tenha o instalador pode ler isso.
       *[id] Este pacote menciona o ID de uma conta em { $places }. Ele foi criado sem os dados de nenhuma conta, e qualquer pessoa que tenha o instalador pode ler isso.
    }
# { $count } is how many folders.
core-warning-scan-folders-of-others-unreadable =
    { $count ->
        [one] { $count } pasta fora do aplicativo não pôde ser lida, então podem faltar dados do aplicativo que estejam nela.
       *[other] { $count } pastas fora do aplicativo não puderam ser lidas, então podem faltar dados do aplicativo que estejam nelas.
    }

## What a scan leaves out because it looks like a credential, or is a license with the License option off, in the app's
## list of what was left out. Each has a name, and below it why.

core-secret-why-credential = parece uma credencial
core-secret-why-license-option-off = a opção Dados de licença e ativação está desativada
# { $count } is how many values of a registry key.
core-secret-why-account-values =
    { $count ->
        [one] { $count } valor, que só entra com a opção Dados do usuário
       *[other] { $count } valores, que só entram com a opção Dados do usuário
    }
# Shown in place of a registry value's name when its name itself looks like a password or a key.
core-secret-name-holds-credential = (um valor cujo nome contém uma credencial)
# Shown in place of an option's name, for an argument on a command line that has no name.
core-secret-unnamed-argument = um argumento que contém uma credencial
# { $option } is an option taken out of a command line, such as /password, or the message above. { $place } says
# where the command line was: service, task, shortcut, opener (what opens a file type), opener-unnamed (the same, of a
# type with no name), or uninstall. { $name } is the service, task, shortcut, file type, or app.
core-secret-taken-from =
    { $place ->
        [service] { $option } na forma como { $name } é executado
        [task] { $option } na forma como a tarefa { $name } é executada
        [shortcut] { $option } no atalho { $name }
        [opener] { $option } no que abre { $name }
        [opener-unnamed] { $option } no que abre um tipo de arquivo
       *[uninstall] { $option } na forma como { $name } é desinstalado
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-secret-task-definition-unreadable = A definição da tarefa { $task }, que não pôde ser lida para ser verificada
# { $name } is an environment variable's name, such as APP_TOKEN.
core-secret-left-out-variable = A variável de ambiente { $name }
# { $class } is a COM class's id, such as {00000000-0000-0000-0000-000000000000}.
core-secret-left-out-class-server = O que a classe { $class } diz executar
# { $rule } is a firewall rule's name.
core-secret-left-out-firewall-description = A descrição da regra de firewall { $rule }

## Licenses a scan found, as the package's report describes each.

# { $name } is a file's name, such as license.lic.
core-license-detail-file = Arquivo de licença detectado: { $name }
# { $framework } is a licensing product's name, such as FlexLM / FlexNet.
core-license-detail-store = Repositório de licenças { $framework } presente
# { $name } is an environment variable's name, such as LM_LICENSE_FILE.
core-license-detail-variable = Variável de ambiente de licença { $name }
# { $name } is a registry value's name, such as SerialNumber.
core-license-detail-registry-value = Valor do Registro que sugere dados de licenciamento: { $name }
# Said of a registry value in an account's key, which is not named.
core-license-detail-registry-value-of-an-account = Valor do Registro que sugere dados de licenciamento
# { $key } is a registry key, such as HKLM\Software\Contoso\License.
core-license-detail-registry-key = Chave do Registro que sugere dados de licenciamento: { $key }

## Warnings a scan records about licenses.

core-warning-scan-license-data = Foram detectados dados de licenciamento/ativação. Com a opção Dados de licença e ativação, o instalador restaura o que pode ser transferido; valores de toda a máquina e repositórios vinculados ao hardware são apenas relatados. O WinAppMigrator nunca descriptografa, contorna ativações nem emula dongles.
core-warning-scan-license-machine-bound = Algumas licenças estão vinculadas à máquina (dongle de hardware, ativação vinculada à máquina ou criptografia). O instalador restaura o que pode; elas ainda podem exigir reativação no PC novo.

## Warnings a scan records about the registry. What could not be read is not named in them: it is kept beside them.

# { $error } is what Windows said went wrong. App Paths is a registry key's name.
core-warning-scan-app-paths-key-unreadable = Não foi possível ler uma subchave de App Paths: { $error }
core-warning-scan-registry-limit = A captura do Registro atingiu o limite de segurança configurado.
core-warning-scan-registry-subtree-limit = A captura de uma subárvore do Registro atingiu o limite de segurança configurado.
# { $error } is what Windows said went wrong.
core-warning-scan-registry-key-unreadable = Não foi possível ler a chave do Registro: { $error }
core-warning-scan-registry-value-too-large = Um valor do Registro excedeu o limite de tamanho de captura configurado.

## Scheduled tasks

core-warning-scan-tasks-folder-through-link = A pasta de tarefas agendadas desta unidade passa por um link, então as tarefas dela não foram lidas.
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-warning-scan-task-left-out = A tarefa agendada { $task } foi deixada de fora: ela é executada com uma conta diferente daquela cujas configurações estão sendo incluídas, outras pessoas além dos administradores podem alterar o arquivo dela, ou o aplicativo tem tarefas demais para manter.
# { $error } is what Windows said went wrong.
core-warning-scan-tasks-folder-unreadable = Não foi possível ler parte das tarefas agendadas: { $error }
# { $path } is a task's file, such as C:\Windows\System32\Tasks\App\Nightly Sync; { $reason } is one of the reasons
# below, or what Windows said went wrong.
core-warning-scan-task-unreadable = Não foi possível ler a tarefa agendada { $path }: { $reason }. Se ela executa o aplicativo, não foi incluída.
# Why a task's file could not be read. { $size } is its size in bytes.
core-task-too-large = tem { $size } bytes, o que é grande demais para uma tarefa
# Why a task's file could not be read.
core-task-ends-mid-character = termina no meio de um caractere
# Why a task's file could not be read. { $error } says where.
core-task-not-utf16 = não é UTF-16 válido: { $error }
# Why a task's file could not be read. { $error } says where.
core-task-not-text = não é UTF-8 nem UTF-16 válido: { $error }
# What a package records of how a task is put back. Register-ScheduledTask is a command's name.
core-task-restore-instructions = A restauração usa Register-ScheduledTask com a definição XML arquivada.

## Reading a program's file. Each is why it could not be read, after the program's name.

# { $reason } is what went wrong.
core-pe-unreadable = não pôde ser lido: { $reason }
core-pe-headers-misaligned = os cabeçalhos não estão nos limites em que os de um programa precisam estar
# { $size } is the file's size in bytes.
core-pe-too-large = tem { $size } bytes, o que é grande demais para um programa
# PE is the name of the format Windows programs are in.
core-pe-no-headers = não tem cabeçalhos PE

## Warnings a scan records about what a program needs. Which program is kept beside each.

# { $error } is what went wrong.
core-warning-scan-imports-unreadable = Não foi possível ler a tabela de importação: { $error }
# { $error } is what went wrong.
core-warning-scan-import-name-unreadable = Não foi possível ler o nome de um descritor de importação: { $error }
# { $count } is how many were skipped.
core-warning-scan-legacy-delay-imports =
    { $count ->
        [one] { $count } descritor de carregamento atrasado legado (v1) foi ignorado; só as importações de carregamento atrasado modernas, baseadas em RVA, são interpretadas.
       *[other] { $count } descritores de carregamento atrasado legados (v1) foram ignorados; só as importações de carregamento atrasado modernas, baseadas em RVA, são interpretadas.
    }

## The DLLs a program needs. Which DLL or program is kept beside each warning.

# { $path } is a program, such as C:\Program Files\App\app.exe.
core-dll-program-has-no-folder = o executável não tem diretório pai: { $path }
# Why a DLL is among the app's files, in the package's list of them.
core-dll-beside-program = DLL localizada ao lado do executável selecionado
# { $error } is what went wrong.
core-warning-scan-dependency-unscanned = Não foi possível analisar a dependência recursiva: { $error }
# { $name } is a DLL's name, such as d3dx9_43.dll.
core-warning-scan-dependency-not-in-newer-windows = { $name } estava na pasta do Windows do PC em que este aplicativo estava, e o Windows 10 e 11 não vêm com ela. Algo a instalou lá, então ela não faz parte dos próprios arquivos do aplicativo e não vem junto. O aplicativo pode não iniciar até que o que a fornece seja instalado.
# { $error } is what went wrong.
core-warning-scan-dependency-not-hashed = Não foi possível calcular o hash da dependência: { $error }
# SystemRoot and windir are the names of environment variables.
core-warning-scan-windows-folder-unknown = As variáveis de ambiente SystemRoot/windir não estão disponíveis; a pesquisa de DLLs do sistema é limitada.
# { $error } is what went wrong.
core-warning-scan-dependency-architecture-unreadable = Não foi possível ler a arquitetura da dependência: { $error }

## Services, drivers, shortcuts, and environment variables a scan reads. Where it was reading is kept beside each
## warning.

# { $error } is what Windows said went wrong.
core-warning-scan-services-unreadable = Não foi possível ler a chave do Registro de serviços: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-drivers-unreadable = Não foi possível ler a chave do Registro de serviços para detectar drivers: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-queried = Não foi possível consultar os atalhos: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-parsed = Não foi possível interpretar o resultado da consulta de atalhos: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-environment-unreadable = Não foi possível ler a chave do Registro de ambiente do sistema: { $error }

## What a package records of how a driver is put back, which an installer shows. { $kind } is the driver's type as
## Windows names it (kernel, file-system, recognizer, or unknown-type); { $name } is its service's name; { $path } is
## its file. { $signed } is yes when { $signer } names who signed it, and no when that is not known.

core-service-driver-bundled =
    { $signed ->
        [yes] Driver { $kind } do próprio aplicativo “{ $name }” ({ $path }, assinado por { $signer }). O instalador inclui o .sys dele e registra novamente o serviço do driver de forma automática (sc create, exige elevação) e depois confirma que o serviço está registrado.
       *[no] Driver { $kind } do próprio aplicativo “{ $name }” ({ $path }, assinatura desconhecida). O instalador inclui o .sys dele e registra novamente o serviço do driver de forma automática (sc create, exige elevação) e depois confirma que o serviço está registrado.
    }
core-service-driver-from-vendor =
    { $signed ->
        [yes] Driver { $kind } de fabricante ou nativo do Windows “{ $name }” ({ $path }, assinado por { $signer }). O WinAppMigrator não redistribui drivers de terceiros; reinstale-o no destino com o pacote de driver assinado do fabricante antes de abrir o aplicativo.
       *[no] Driver { $kind } de fabricante ou nativo do Windows “{ $name }” ({ $path }, assinatura desconhecida). O WinAppMigrator não redistribui drivers de terceiros; reinstale-o no destino com o pacote de driver assinado do fabricante antes de abrir o aplicativo.
    }

## Runtimes an app needs, such as the Visual C++ runtime or .NET: how each is provided, as the Dependencies list says.

core-runtime-handling-built-in = Integrado ao Windows
core-runtime-handling-ships-with-app = Acompanha o aplicativo
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-handling-windows-feature = Recurso do Windows: { $feature }
core-runtime-handling-included = Incluído
# { $id } is the runtime's name in winget, such as Microsoft.VCRedist.2015+.x64.
core-runtime-handling-winget = Instalado com o winget: { $id }
core-runtime-handling-manual = Instalar manualmente

## Downloading a runtime's installer, when an installer is built. Each is why it could not be, and starts with a small
## letter, as it follows other words. { $url } is a web address; { $error } is what went wrong.

core-runtime-download-not-https = download de { $url } recusado: não é HTTPS
# TLS is what keeps a download private.
core-runtime-tls-failed = falha ao inicializar o TLS: { $error }
core-runtime-download-failed = falha no download de { $url }: { $error }
core-runtime-download-unreadable = falha ao ler { $url }: { $error }
core-runtime-download-empty = o download de { $url } não retornou dados
# { $came } and { $expected } are numbers of bytes.
core-runtime-download-cut-short = o download de { $url } foi interrompido: chegaram { $came } de { $expected } bytes

## Installing a runtime: why it was not installed. Each starts with a small letter, as it follows other words.

core-runtime-no-known-installer = não há nenhum instalador conhecido para este runtime
# { $path } is where in the package, such as prerequisites\vc_redist.x64.exe.
core-runtime-payload-misplaced = o pacote indica { $path } como instalador deste runtime, e esse não é o lugar dele, então ele não foi executado
# { $path } is a file.
core-runtime-payload-missing = o instalador incluído está ausente: { $path }
# { $error } is what went wrong.
core-runtime-payload-path-invalid = o caminho do instalador incluído é inválido: { $error }
# { $url } is the web address it can be downloaded from. winget is a program's name.
core-runtime-no-installer-route = não há instalador incluído nem ID do winget; obtenha-o em { $url }
# { $error } is what went wrong.
core-runtime-no-temp-folder = não foi possível criar uma pasta em que descompactá-lo: { $error }
# { $reason } is what the check found.
core-runtime-payload-signature-failed = o instalador incluído falhou na verificação de assinatura e não foi executado: { $reason }
core-runtime-payload-timed-out = o instalador incluído não terminou dentro do tempo limite e foi encerrado
# { $error } is what went wrong.
core-runtime-payload-not-started = não foi possível iniciar o instalador incluído: { $error }
core-runtime-installer-killed = o instalador foi encerrado sem código de saída
# { $code } is the number it ended with, such as 1603.
core-runtime-installer-failed = o instalador terminou com o código { $code }
# { $feature } is a Windows feature's name, such as NetFx3; { $code } is the number DISM ended with. Windows Features is
# the name of a Windows window. Keep the command as it is.
core-runtime-feature-failed = o DISM não conseguiu ativar o recurso do Windows { $feature } (saída { $code }); provavelmente não foi possível baixar os arquivos necessários. Ative-o em Recursos do Windows ou execute `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`.
# The same, when DISM gave no number.
core-runtime-feature-failed-no-code = o DISM não conseguiu ativar o recurso do Windows { $feature } (saída desconhecida); provavelmente não foi possível baixar os arquivos necessários. Ative-o em Recursos do Windows ou execute `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`.
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-feature-timed-out = a ativação do recurso do Windows { $feature } não terminou dentro do tempo limite e foi encerrada
# { $feature } is a Windows feature's name; { $error } is what went wrong.
core-runtime-feature-no-dism = o DISM não está disponível para ativar o recurso do Windows { $feature }: { $error }

## Installing a runtime with winget. { $id } is its name in winget, such as Microsoft.VCRedist.2015+.x64.

# { $error } is what went wrong.
core-runtime-winget-not-looked-for = não foi possível procurar o winget: { $error }
core-runtime-winget-not-installed = o winget não está instalado
# { $path } is where winget is.
core-runtime-winget-misplaced = { $path } não está onde o Windows mantém os próprios pacotes, então não foi executado
# { $path } is where winget is; { $reason } is what the check found.
core-runtime-winget-signature-failed = { $path } falhou na verificação de assinatura e não foi executado: { $reason }
# { $reason } is one of the two messages above, or what went wrong.
core-runtime-winget-unusable = não foi possível usar o winget para instalar { $id }: { $reason }
# { $code } is the number winget ended with.
core-runtime-winget-failed = winget install { $id } terminou com { $code }
core-runtime-winget-failed-no-code = winget install { $id } terminou com um código desconhecido
core-runtime-winget-timed-out = winget install { $id } não terminou dentro do tempo limite e foi encerrado
# { $error } is what went wrong.
core-runtime-winget-not-started = o winget não está disponível para instalar { $id }: { $error }

## What told a scan how an app was installed, as the package's report lists it. The names of installers and files are
## as their makers write them.

# { $path } is an installer's record of what it installed, such as C:\Program Files\App\unins000.dat.
core-source-record-not-believed = { $path } não foi usado: outras pessoas além dos administradores podem alterá-lo
# { $count } is how many Windows Installer packages the bundle holds.
core-source-burn-bundle =
    { $count ->
        [0] Bundle Burn com { $count } pacotes do Windows Installer
        [one] Bundle Burn com { $count } pacote do Windows Installer
       *[other] Bundle Burn com { $count } pacotes do Windows Installer
    }
# { $count } is how many paths Windows Installer recorded.
core-source-msi-database =
    { $count ->
        [0] Banco de dados do Windows Installer: { $count } caminhos de chave de componente
        [one] Banco de dados do Windows Installer: { $count } caminho de chave de componente
       *[other] Banco de dados do Windows Installer: { $count } caminhos de chave de componente
    }
core-source-inno-log = Log de desinstalação do Inno Setup (unins000.dat)
# { $count } is how many paths the log records.
core-source-inno-log-paths =
    { $count ->
        [0] Log de desinstalação do Inno Setup: { $count } caminhos registrados
        [one] Log de desinstalação do Inno Setup: { $count } caminho registrado
       *[other] Log de desinstalação do Inno Setup: { $count } caminhos registrados
    }
core-source-nsis = Assinatura do desinstalador NSIS (Nullsoft)
# { $count } is how many paths the log records.
core-source-nsis-log-paths =
    { $count ->
        [0] uninstall.log do NSIS AdvUninstLog: { $count } caminhos registrados
        [one] uninstall.log do NSIS AdvUninstLog: { $count } caminho registrado
       *[other] uninstall.log do NSIS AdvUninstLog: { $count } caminhos registrados
    }
core-source-squirrel = Layout por usuário do Squirrel (Update.exe / app-*)
# { $version } is a folder's name that says the version, such as app-1.2.3.
core-source-squirrel-version = Versão atual do Squirrel: { $version }
core-source-chocolatey = Metadados do pacote Chocolatey
core-source-msix = Pacote MSIX / Appx
# { $identity } is a package's name, such as Contoso.App.
core-source-appx-identity = Identidade no AppxManifest: { $identity }
# { $count } is how many folders.
core-source-msix-data =
    { $count ->
        [0] Dados por usuário do pacote MSIX: { $count } locais
        [one] Dados por usuário do pacote MSIX: { $count } local
       *[other] Dados por usuário do pacote MSIX: { $count } locais
    }
# App Paths is a registry key's name.
core-source-app-paths = Entrada em App Paths

## Why a runtime is needed, as the package's report says it.

# { $program } is a program's file name, such as app.exe; { $dll } is a DLL's, such as msvcp140.dll.
core-runtime-program-imports = { $program } importa { $dll }

## What else of an app's a scan finds in the registry: fonts and certificates, and why each is the app's.

# Why a font is among the app's files, in the package's list of them.
core-registry-font-recorded = Uma fonte indicada nos dados de instalação do aplicativo
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-named-for-the-app = tem o nome do aplicativo ou do editor dele
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-recorded = os dados de instalação do aplicativo o indicam

## Who is signed in, for the Whose Data choice. Each of the first six is why it could not be told, and starts with a
## small letter, as it is shown inside other words. An account is shown by its id, such as S-1-5-21-...-1001.

# { $error } is what Windows said.
core-profile-desktop-owner-not-said = o Windows não informou de quem é esta área de trabalho: { $error }
# { $error } is what went wrong.
core-profile-desktop-owner-not-asked = não foi possível perguntar ao Windows de quem é esta área de trabalho: { $error }
core-profile-signed-in-account-no-profile = a conta conectada aqui, { $account }, não tem perfil registrado
core-profile-desktop-not-a-person = a área de trabalho aqui pertence a { $account }, que não é a conta de uma pessoa
core-profile-nobody-signed-in-running-account-no-profile = ninguém está conectado aqui, e a conta que executa isto, { $account }, não tem perfil registrado
core-profile-nobody-signed-in-running-not-a-person = ninguém está conectado aqui, e { $account }, que executa isto, não é a conta de uma pessoa
# SID is the name Windows gives an account's id.
core-profile-nobody-signed-in-no-sid = ninguém está conectado aqui, e esta conta não tem SID
# { $accounts } is a list of accounts.
core-profile-several-desktops = mais de uma conta tem uma área de trabalho nesta sessão: { $accounts }
# Under the Whose Data choice. { $reason } is one of the messages above.
core-profile-signed-in-unknown = Não foi possível determinar quem está conectado ({ $reason }).
# Under the Whose Data choice, when there are accounts to choose from. { $reason } is one of the messages above.
core-profile-signed-in-unknown-choose = Não foi possível determinar quem está conectado ({ $reason }). Escolha de quem incluir os dados.
