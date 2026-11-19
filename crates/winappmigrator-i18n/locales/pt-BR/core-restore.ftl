# Words the core writes for people as it installs an app from a package: why an install will not start, the list of
# what it did (or for a preview, what it would do) that the installer window shows, its warnings, the record it keeps of
# each install so that it can be undone, and the core's errors. Message names start with core-.
#
# The list of what an install did is short and plain, one line for each thing, as the installer window lists them:
# "copied", "restored", "would copy". Words such as --force and --apply are command-line options, and LocalService,
# SYSTEM, ProgramData, and %AppData% are as Windows names them: keep them as they are.


## Errors: what went wrong, when the core cannot go on. Each is shown on its own, or after words that say what was being
## done.

core-error-list-changed = o que esta instalação faria mudou desde que foi aceito, então nada foi instalado
# { $path } is a file or folder, such as C:\Program Files\App\app.exe, or words that name one, such as "the installer".
# { $error } is what Windows said went wrong.
core-error-io = falha de E/S ({ $path }): { $error }
# { $detail } is what went wrong.
core-error-registry = falha na leitura do Registro: { $detail }
# { $path } is a program file. { $detail } is what was wrong with it.
core-error-pe = falha ao interpretar o arquivo PE { $path }: { $detail }
# { $detail } is what was wrong, already in words.
core-error-invalid-input = entrada inválida: { $detail }
# { $detail } is what went wrong.
core-error-serialization = falha na serialização do relatório: { $detail }
# { $detail } is what went wrong.
core-error-archive = falha na operação com o arquivo compactado: { $detail }
# { $detail } is what went wrong.
core-error-network = falha na operação de rede: { $detail }
# { $detail } is what went wrong.
core-error-time-format = falha na formatação de data e hora: { $detail }
core-error-cancelled = a operação foi cancelada
# What is said when part of the program stopped unexpectedly and gave no reason.
core-error-panic-no-reason = nenhum motivo foi informado

## The record of each install, kept so that it can be undone.

# { $kind } is a number that says what type of registry value it is.
core-journal-unknown-value-type = tipo de valor do Registro desconhecido: { $kind }
core-journal-not-elevated = o processo não está elevado
core-journal-needs-admin = instalar exige direitos de administrador, para guardar o histórico da instalação onde outras pessoas não possam alterá-lo
# { $path } is a folder.
core-journal-not-an-install-folder = esta não é a pasta de uma instalação: { $path }
# { $path } is the file the record is kept in.
core-journal-not-written = não foi possível gravar o histórico desta instalação: { $path }
# { $path } is a folder.
core-journal-path-escapes-root = o caminho sai da raiz pretendida: { $path }
# { $path } is a folder.
core-journal-nothing-exists = nada disso existe neste PC: { $path }
core-journal-busy = outra instalação ou desinstalação está em execução. Aguarde até que ela termine e tente novamente
# Said of a file that was to get back the permissions it had.
core-journal-no-folder = ele não tem uma pasta
# Said of the permissions a file had, which were kept to give back to it.
core-journal-permissions-unreadable = o que foi guardado não pôde ser lido
# { $folder } is the folder of one install's record.
core-journal-unreadable-empty = o histórico da instalação em { $folder } não pode ser usado: ele está vazio
# { $folder } is the folder of one install's record.
core-journal-unreadable-no-header = o histórico da instalação em { $folder } não pode ser usado: ele não começa dizendo a que se refere
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-version = o histórico da instalação em { $folder } não pode ser usado: ele foi gravado por outra versão deste programa
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-install = o histórico da instalação em { $folder } não pode ser usado: ele diz que pertence a outra instalação
# { $id } is an install's id, such as wam-1a2b3c.
core-journal-not-an-install-id = este não é o ID de uma instalação: { $id }

## Why an install will not start, or stopped before it changed anything.

core-restore-apply-and-dry-run = --apply e --dry-run não podem ser usados juntos
# { $path } is the folder that was chosen to install in.
core-restore-location-not-full-path = o local de instalação deve ser um caminho completo: “{ $path }”
# { $problem } says what is wrong with the folder, already in words. { $path } is the folder.
core-restore-location-unusable = esse local de instalação não pode ser usado: { $problem }: “{ $path }”
core-restore-needs-admin-plan = o plano de restauração inclui alterações que só um administrador pode fazer: no Registro, em serviços, tarefas agendadas, atalhos ou destinos protegidos
# { $path } is the folder that was chosen to install in.
core-restore-needs-admin-target = a raiz de destino exige privilégios elevados: { $path }
core-restore-needs-admin-user-data = restaurar dados do usuário em ProgramData ou em outros locais protegidos exige privilégios elevados
# { $error } is why the install stopped. { $failed } is what could not be put back, as sentences of their own.
core-restore-not-all-put-back = { $error }. Nem tudo o que tinha sido feito pôde ser revertido: { $failed }
# { $error } is what went wrong.
core-restore-nowhere-to-keep-record = não há um lugar seguro para guardar o histórico que permitiria desfazer esta instalação, então nada foi instalado: { $error }
# { $path } is a file the app would have put there.
core-restore-already-exists = { $path } já existe, então nada foi instalado. Use --force para substituí-lo
# { $root } is a folder the app has files for. { $link } is that folder, or one it is in.
core-restore-root-under-link = este aplicativo tem itens a restaurar em { $root }, e { $link } é um link para outro lugar, que outras pessoas podem conseguir alterar, então nada foi instalado. Se o que vai para lá são dados do usuário ou dados de licença, instale-o sem os dados do usuário e de licença. Se for um atalho em uma pasta Inicializar, devolva essa pasta ao lugar onde ela estava
core-restore-hash-check-failed = o pacote falhou na verificação de hash e nada foi instalado
# { $detail } is the first thing found wrong, already in words.
core-restore-hash-check-failed-because = o pacote falhou na verificação de hash e nada foi instalado: { $detail }
# { $path } is a file or folder.
core-restore-refuse-records-folder = não é permitido gravar onde as instalações guardam os históricos: { $path }
# { $path } is a file or folder.
core-restore-refuse-outside = não é permitido gravar fora do local de instalação e das pastas de dados conhecidas: { $path }
# { $path } is a file.
core-restore-no-parent-folder = o destino não tem pasta pai: { $path }
# { $path } is where a file was to go.
core-restore-folder-in-the-way = há uma pasta onde este arquivo precisa ficar: { $path }
# { $path } is a file in the package.
core-restore-file-changed = { $path } não é mais o que o pacote contém, então não foi instalado
# { $path } is a file in the package.
core-restore-source-is-link = { $path } é um link, e links não são seguidos
# { $path } is a file in the package.
core-restore-source-became-link = { $path } se tornou um link enquanto era aberto
# Why a service did not start, after "did not start:".
core-restore-service-stopped-by-itself = ele parou de novo sozinho
# Why part of the app was left out, after "was left out:".
core-restore-com-class-someone-elses = esta classe COM já existe neste PC e pertence a outro programa

## What an install did, one line for each thing, or for a preview, what it would do. { $source } is a file in the
## package, and { $destination } and { $path } are where it goes on this PC. { $key } is a registry key, such as
## HKLM\Software\App.

# { $package_valid } is true or false, and { $restore_valid } is true, false, or not_run: keep them as they are.
core-restore-verification-result = verificação pós-restauração package_valid={ $package_valid } restore_valid={ $restore_valid }
# { $name } is a service's name.
core-restore-started-service = iniciou o serviço { $name }
core-restore-would-copy = copiaria { $source } -> { $destination }
core-restore-copied = copiou { $source } -> { $destination }
core-restore-would-create-empty-folder = criaria a pasta vazia { $path }
core-restore-created-empty-folder = criou a pasta vazia { $path }
# { $value } is the name of a value in the key.
core-restore-would-restore-registry-value = restauraria o valor do Registro { $key } { $value }
core-restore-would-restore-registry-key = restauraria a chave do Registro { $key }
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-would-add-firewall-rule = adicionaria a regra de firewall { $name } para { $program }
# { $name } is a firewall rule's name.
core-restore-firewall-rule-already-there = a regra de firewall { $name } já existe
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-added-firewall-rule = adicionou a regra de firewall { $name } para { $program }
# { $name } is a service's name, { $command } what it runs, and { $account } the account it runs as, such as LocalService.
core-restore-would-restore-service = restauraria o serviço { $name } -> { $command } (executado como { $account })
# { $name } is a runtime, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-restore-prerequisite-present = pré-requisito já presente: { $name }
# { $name } is a runtime that Windows has as a Windows feature.
core-restore-would-install-prerequisite-feature = instalaria o pré-requisito { $name } (recurso do Windows)
# { $name } is a runtime whose installer is in the package.
core-restore-would-install-prerequisite-bundled = instalaria o pré-requisito { $name } (instalador incluído)
# { $name } is a runtime that would come through winget, which is a program's name.
core-restore-would-install-prerequisite-winget = instalaria o pré-requisito { $name } (winget)
# { $name } is a runtime.
core-restore-installed-prerequisite = instalou o pré-requisito { $name }
# { $name } is a runtime.
core-restore-installed-prerequisite-reboot = instalou o pré-requisito { $name } (é necessário reiniciar)
core-restore-would-restore-license = restauraria a licença { $source } -> { $destination }
core-restore-restored-license = restaurou a licença { $source } -> { $destination }
core-restore-would-restore-license-registry = restauraria a licença no Registro { $key }
# { $name } is a driver's name.
core-restore-driver-not-restored = driver não restaurado automaticamente: { $name }
# { $name } is a driver's name.
core-restore-would-install-driver = instalaria o driver { $name } -> { $destination }
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-would-restore-task = restauraria a tarefa agendada { $name }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-startup-shortcut = restauraria o atalho de inicialização { $shortcut } -> { $target }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-shortcut = restauraria o atalho { $shortcut } -> { $target }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-would-restore-variable =
    { $scope ->
        [user] restauraria a variável de ambiente do usuário { $name }
       *[machine] restauraria a variável de ambiente do sistema { $name }
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-left-variable =
    { $scope ->
        [user] manteve a variável de ambiente do usuário { $name } como está: nada nela pertence a este aplicativo
       *[machine] manteve a variável de ambiente do sistema { $name } como está: nada nela pertence a este aplicativo
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-restored-variable =
    { $scope ->
        [user] restaurou a variável de ambiente do usuário { $name }
       *[machine] restaurou a variável de ambiente do sistema { $name }
    }
core-restore-set-uninstaller = definiu o desinstalador para { $key }
core-restore-placed-driver-image = colocou a imagem do driver { $source } -> { $destination }
# { $name } is a driver's name.
core-restore-configured-driver = configurou o driver { $name }
# { $name } is a driver's name.
core-restore-created-driver = criou o driver { $name }
# { $name } is a driver's name.
core-restore-started-driver = iniciou o driver { $name }
core-restore-ensured-registry-key = garantiu que a chave do Registro { $key } existe
# { $value } is the name of a value in the key.
core-restore-restored-registry-value = restaurou o valor do Registro { $key } { $value }
# "(Default)" is how Windows names the value a key has with no name: write it as Registry Editor does.
core-restore-restored-registry-default = restaurou o valor do Registro { $key } (Padrão)
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-configured-service = configurou o serviço { $name } (executado como { $account })
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-created-service = criou o serviço { $name } (executado como { $account })
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-restored-task = restaurou a tarefa agendada { $name }
core-restore-restored-shortcut = restaurou o atalho { $path }

## Warnings an install gives, and why it failed. They are shown after it, on the PC it ran on.

core-warning-restore-earlier-not-trusted = O histórico de uma instalação que não foi concluída foi deixado como está, porque outras pessoas podem alterá-lo. O que essa instalação alterou continua no lugar.
# { $app_name } is the app's name. { $count } is how many changes were put back.
core-warning-restore-earlier-undone =
    { $count ->
        [0] Uma instalação anterior de { $app_name } não foi concluída, então ela foi desfeita primeiro. { $count } alterações foram revertidas.
        [one] Uma instalação anterior de { $app_name } não foi concluída, então ela foi desfeita primeiro. { $count } alteração foi revertida.
       *[other] Uma instalação anterior de { $app_name } não foi concluída, então ela foi desfeita primeiro. { $count } alterações foram revertidas.
    }
# { $app_name } is the app's name.
core-warning-restore-earlier-never-finished = Uma instalação anterior de { $app_name } não foi concluída. Ao instalar, ela é desfeita primeiro.
core-warning-restore-user-data-real-locations = Os dados do usuário são restaurados nos locais reais %AppData%/%ProgramData% desta máquina, e não na raiz de destino.
# { $error } is what went wrong.
core-warning-restore-commit-failed = Não foi possível concluir o histórico desta instalação: { $error }
# { $name } is a service's name.
core-warning-restore-service-still-starting = Foi solicitado que o serviço { $name } fosse iniciado, e ele ainda não terminou de iniciar. Isso ainda pode acontecer. Ele também é iniciado sozinho na próxima inicialização deste PC.
# { $name } is a service's name. { $reason } is why it did not start, as Windows said it.
core-warning-restore-service-not-started = O serviço { $name } está instalado e não foi iniciado: { $reason }. O aplicativo pode não funcionar até que ele seja iniciado. O Windows tentará novamente na próxima inicialização deste PC.
# { $name } is a service's name. { $reason } is why it could not be stopped.
core-warning-restore-service-not-stopped = Não foi possível parar o serviço { $name } antes: { $reason }. Ele pode continuar executando o programa antigo até a próxima inicialização deste PC.
core-warning-restore-install-cancelled = A instalação foi cancelada.
core-warning-restore-preview-cancelled = A visualização foi cancelada.
# { $item } is what was left out, such as a file or a service's name. { $reason } is why, already in words.
core-warning-restore-left-out = { $item } foi deixado de fora: { $reason }.
# { $error } is what went wrong.
core-warning-restore-verification-failed = A verificação pós-restauração não pôde ser executada: { $error }
# { $runner } says what relies on the file: a service, a scheduled task, a firewall rule, a shortcut, or a registry value.
# { $path } is the file. { $name } is the name of what relies on it. { $refusal } is why the file was not replaced,
# already in words.
core-warning-restore-program-would-run =
    { $runner ->
        [service] Não cabe a este pacote substituir { $path }, e o serviço { $name } depende dele, então nada foi instalado: { $refusal }
        [task] Não cabe a este pacote substituir { $path }, e a tarefa agendada { $name } depende dele, então nada foi instalado: { $refusal }
        [firewall] Não cabe a este pacote substituir { $path }, e a regra de firewall { $name } depende dele, então nada foi instalado: { $refusal }
        [shortcut] Não cabe a este pacote substituir { $path }, e o atalho { $name } depende dele, então nada foi instalado: { $refusal }
       *[registry] Não cabe a este pacote substituir { $path }, e o valor do Registro { $name } depende dele, então nada foi instalado: { $refusal }
    }
core-warning-restore-destination-exists = O destino já existe; use --force para substituí-lo.
# { $path } is a file. { $error } is what went wrong.
core-warning-restore-file-not-written = Não foi possível gravar { $path }: { $error }
core-warning-restore-not-a-folder = Há algo que não é uma pasta onde esta pasta vazia deve ficar.
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-not-looked-at = Não foi possível examinar { $path }: { $error }
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-folder-not-made = Não foi possível criar { $path }: { $error }
# Why part of the app was left out, after "was left out:".
core-warning-restore-registration-exists = este PC já tem isto, definido de outra forma, e uma instalação não assume o controle disso
core-warning-restore-registry-review = Os valores do Registro foram restaurados a partir dos dados estruturados arquivados, com os caminhos conhecidos do aplicativo reescritos para a raiz de destino.
# { $error } is what went wrong.
core-warning-restore-firewall-unreadable = Não foi possível ler as regras do Firewall do Windows: { $error }
# Why a firewall rule was left out, after "was left out:".
core-warning-restore-firewall-rule-exists = uma regra com este nome para este programa já existe, definida de outra forma, e uma instalação não a altera
# { $name } is a runtime.
core-warning-restore-prerequisite-unknown = { $name }: não há nenhum instalador conhecido para este runtime.
# { $name } is a runtime.
core-warning-restore-prerequisite-needs-admin = Instalar { $name } exige elevação; execute o instalador novamente como administrador.
# { $name } is a runtime.
core-warning-restore-prerequisite-reboot = { $name } foi instalado, mas é necessário reiniciar para concluir a instalação.
# { $error } is what went wrong.
core-warning-restore-skipped-not-recorded = Não foi possível registrar no histórico um runtime que foi ignorado: { $error }
# { $name } is a runtime. { $reason } is why it was skipped, already in words.
core-warning-restore-prerequisite-skipped = { $name }: { $reason }
# { $name } is a runtime. { $reason } is why it could not be installed. { $url } is a web address to get it from.
core-warning-restore-prerequisite-failed = Não foi possível instalar { $name }: { $reason }. Obtenha-o em { $url }.
# { $error } is what went wrong.
core-warning-restore-runtime-not-recorded = Não foi possível registrar no histórico um runtime instalado: { $error }
# { $error } is what went wrong.
core-warning-restore-license-source-invalid = O caminho da licença no pacote é inválido: { $error }
core-warning-restore-license-needs-admin = Restaurar dados de licença em um local protegido exige elevação.
core-warning-restore-license-exists = O destino da licença já existe; use --force para substituí-lo.
# { $error } is what went wrong.
core-warning-restore-license-failed = Não foi possível restaurar os dados de licença: { $error }
core-warning-restore-driver-needs-admin = Registrar o driver exige elevação; execute o instalador novamente como administrador.
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-warning-restore-task-runs-as-system = A tarefa agendada { $name } é executada como SYSTEM, com direitos totais sobre este PC.
core-warning-restore-shortcut-exists = O atalho já existe; use --force para substituí-lo.
core-warning-restore-shortcut-all-users = Um ou mais atalhos restaurados apontam para a área de trabalho pública ou para o menu Iniciar comum.
core-warning-restore-environment-refresh = As alterações nas variáveis de ambiente podem só aparecer em um novo processo, em um novo shell ou depois de entrar novamente.
# { $name } is a service's name. { $account } is the account it ran as on the PC it came from.
core-warning-restore-service-account = O serviço { $name } era executado como { $account } na origem. A senha dele não está no instalador, então o serviço é configurado como LocalService. Se ele não funcionar, defina a conta e a senha dele em Serviços.
# { $name } is a runtime.
core-warning-restore-prerequisite-pending-reboot = { $name } ainda não está registrado; ele deve estar ativo depois de reiniciar.
# { $name } is a runtime.
core-warning-restore-prerequisite-not-verified = { $name } informou sucesso, mas não foi registrado; a instalação pode ter falhado.
# { $error } is what went wrong.
core-warning-restore-driver-source-invalid = O caminho do driver incluído é inválido: { $error }
# { $error } is what went wrong.
core-warning-restore-driver-not-read = Não foi possível ler as configurações atuais do driver: { $error }
core-warning-restore-driver-exists = O serviço do driver já existe; use --force para reconfigurá-lo.
core-warning-restore-driver-image-exists = A imagem do driver já existe; use --force para substituí-la.
core-warning-restore-driver-hash-bundled = A imagem do driver incluída não corresponde ao hash registrado dela, então não foi instalada.
# { $error } is what went wrong.
core-warning-restore-driver-not-placed = Não foi possível colocar a imagem do driver: { $error }
core-warning-restore-driver-hash-placed = A imagem do driver foi alterada depois de ser colocada, então não foi registrada.
# { $error } is what went wrong.
core-warning-restore-key-not-created = Não foi possível criar a chave do Registro: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-read = Não foi possível ler o valor que isto substituiria: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-written = Não foi possível gravar o valor do Registro: { $error }
core-warning-restore-value-exists = O valor do Registro já existe; use --force para substituí-lo.
# { $error } is what went wrong.
core-warning-restore-environment-key-not-created = Não foi possível criar a chave do Registro de ambiente: { $error }
core-warning-restore-variable-exists = A variável de ambiente já existe; use --force para substituí-la.
# { $error } is what went wrong.
core-warning-restore-variable-not-written = Não foi possível gravar a variável de ambiente: { $error }
core-warning-restore-value-kind-missing = O tipo do valor do Registro está ausente.
# { $error } is what went wrong.
core-warning-restore-service-not-read = Não foi possível ler as configurações atuais do serviço: { $error }
core-warning-restore-service-exists = O serviço já existe; use --force para reconfigurá-lo.
# { $error } is what went wrong.
core-warning-restore-task-not-queried = Falha ao consultar a tarefa agendada: { $error }
core-warning-restore-task-exists = A tarefa agendada já existe; use --force para substituí-la.
# { $error } is what went wrong.
core-warning-restore-task-folder-not-looked-at = Não foi possível examinar a pasta da tarefa: { $error }
# { $error } is what went wrong.
core-warning-restore-shortcut-not-put-back = Não foi possível criar o atalho, e o atalho que ele substituiu não pôde ser reposto: { $error }
# { $error } is what went wrong.
core-warning-restore-command-not-started = Falha ao iniciar o comando: { $error }
# { $output } is what the command wrote, as it wrote it.
core-warning-restore-command-failed = O comando falhou: { $output }
# { $error } is what went wrong.
core-warning-restore-powershell-not-started = Falha ao iniciar o PowerShell: { $error }
