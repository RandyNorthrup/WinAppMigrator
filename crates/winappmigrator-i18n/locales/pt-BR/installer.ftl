# Words of the installer that every built installer carries, which runs on the PC an app is moved to and speaks its
# language, and of the uninstaller it leaves behind for Settings. Message names start with installer- for the
# installer's window, installer-cli- for its command line, and uninstaller- for the uninstaller.
#
# The command lines (installer-cli-, uninstaller-cli-) are always shown in English, as scripts read what they print.
# Translate them all the same. Keep their command-line flags, such as --apply and --target-root, exactly as they are.


## The installer's window: its title and header

installer-window-title = Instalador do WinAppMigrator
# The window's title in the taskbar. { $app_name } is the name of the app it installs.
installer-window-title-app = Instalar { $app_name }
installer-tagline = Reinstala um aplicativo capturado em outro PC
# What an app is called when its package gives it no name.
installer-unnamed-app = Aplicativo

## When the installer's window cannot open. Each is a paragraph of a message box.

# { $error } is what went wrong, as Windows or the graphics driver says it.
installer-window-failed = Não foi possível abrir a janela do instalador: { $error }
installer-window-failed-without-one = Ainda é possível instalar sem ela: execute este arquivo em um prompt de comando de administrador com --apply e, para escolher onde, com --target-root seguido de uma pasta.

## Checking the installer when it opens

installer-checking = Verificando o instalador…
installer-stub-no-package = Este é um stub de instalador do WinAppMigrator e não contém nenhum pacote de aplicativo. Crie um instalador com o WinAppMigrator e execute aqui o .exe gerado.
installer-check-stopped = A verificação do instalador foi interrompida.
# { $error } is what went wrong, as Windows says it.
installer-locate-failed = Não foi possível localizar o instalador: { $error }
# { $error } is what went wrong reading it.
installer-read-failed = Não foi possível ler o instalador: { $error }
# { $error } is why, such as that the app needs a newer Windows.
installer-cannot-be-used-here = O pacote deste instalador não pode ser usado neste PC: { $error }

## The app, where it goes, and whether the installer is signed

installer-captured-from = Capturado de
installer-install-location = Local de instalação
# The title of the folder picker.
installer-choose-location = Escolher o local de instalação
installer-folder-has-files = Esta pasta já contém arquivos. Arquivos com os mesmos nomes são substituídos e voltam se você desinstalar.
installer-room-needed = Espaço necessário
# { $shortfall } says which drive is short of room, such as "C: needs up to 824.0 MB for the app's files, and has 300.0 MB free".
installer-room-short = Talvez não haja espaço suficiente aqui: { $shortfall }. O pacote pode ser descompactado em outra unidade, mas os arquivos do aplicativo ainda precisam caber.
# { $app_name } is the name of an app whose earlier install was cut short.
installer-interrupted = Uma instalação anterior de { $app_name } não foi concluída. Ao instalar, ela é desfeita primeiro.
# Beside "Installer", for whether it proves who made it.
installer-signature = Instalador
installer-signature-checking = Verificando a assinatura…
installer-signature-unsigned = Não assinado: editor desconhecido
# { $publisher } is the name on the installer's certificate, such as "Fabrikam, Inc.".
installer-signature-signed-by = Assinado por { $publisher }
installer-signature-unnamed = Assinado por um editor sem nome
# { $reason } is Windows' own word for what is wrong, such as HashMismatch.
installer-signature-invalid = A assinatura deste instalador não é válida ({ $reason }). Ele pode ter sido alterado depois de ser assinado.

## Why the install location cannot be used. Install and Preview Changes stay off while one of these is shown.

# { $app_name } is the name of the app being installed.
installer-location-app-already-there = { $app_name } já está instalado nesta pasta, então esta cópia não pode ir para cá. Para instalá-lo aqui, primeiro desinstale a cópia que está neste PC, em Configurações do Windows, em Aplicativos, e abra este instalador novamente. Ou digite acima uma pasta nova e vazia.
# { $other_app } is the name of another app Windows lists as installed in the folder. { $app_name } is the name of the app being installed.
installer-location-other-app-there = { $other_app } está instalado nesta pasta, então { $app_name } não pode ir para cá. Digite acima uma pasta nova e vazia. Ou, se você não precisa mais de { $other_app }, desinstale-o primeiro, em Configurações do Windows, em Aplicativos, e abra este instalador novamente.
installer-location-has-files = Esta pasta já contém arquivos que não foram instalados pelo WinAppMigrator. Eles podem pertencer a outro aplicativo, então nada é instalado por cima deles. Digite acima uma pasta nova e vazia.
# { $reason } is why, such as that it cannot be the Windows folder.
installer-location-refused = Esta pasta não pode ser usada: { $reason }. Digite outra pasta acima.

## What the install sets up beyond its files

installer-beyond-files = O que isto configura além dos arquivos
# Under the heading above. It is part of what a person agrees to when they click Install.
installer-beyond-files-agreed = Tudo isso é feito com direitos de administrador. Se a instalação falhar, o que ela alterou é revertido, e você é informado do que não puder ser. Você também pode desfazer a instalação depois em Configurações, em Aplicativos instalados. Os arquivos que ela substitui voltam, e os runtimes que ela instala permanecem.
installer-beyond-files-none = Nada: ele apenas copia arquivos e configurações nas próprias chaves do Registro do aplicativo.
# { $problem } is why, such as that the app cannot run on this PC.
installer-cannot-go-here = Este pacote não pode ir para este local: { $problem }
installer-working-out = Calculando o que isto configuraria neste local…
# Said as the reason after "This package cannot go to this location:".
installer-list-stopped = o cálculo do que ele faria foi interrompido
# A heading over a list of what else the package asked for and will not get. { $count } is how many.
installer-more-left-out =
    { $count ->
        [0] Mais { $count } itens ficaram de fora
        [one] Mais { $count } item ficou de fora
       *[other] Mais { $count } itens ficaram de fora
    }

## Options

installer-options = Opções
installer-user-data = Dados do usuário
installer-user-data-detail = Ativado: configurações, Documentos e Jogos Salvos vão para as pastas habituais de quem estiver conectado. Desativado: tudo vai para a pasta de instalação, e nada é configurado para ser executado a partir dela.
# { $account } is the name of the account on the other PC whose data this is.
installer-user-data-whose = Contém os dados de { $account }.
installer-license = Licença
installer-license-detail = Arquivos de ativação e licença, e os valores do Registro que guardam licenças. Alguns podem precisar de reativação aqui.

## Dependencies: what else the app needs, and how each is provided

installer-dependencies = Dependências
installer-dependencies-detail = Do que este aplicativo precisa e como cada item é fornecido.
installer-dependencies-none = Nenhuma dependência de runtime adicional foi detectada.
# { $feature } is the name of an optional part of Windows, such as NetFx3.
installer-dependency-windows-feature = Recurso do Windows: { $feature }
installer-dependency-included = Incluído
# { $id } is the package's name in winget, such as Microsoft.VCRedist.2015+.x64.
installer-dependency-winget = Instalado com o winget: { $id }
installer-dependency-manual = Instalar manualmente
installer-dependency-built-in = Integrado ao Windows
installer-dependency-ships-with-app = Acompanha o aplicativo

## Buttons, and what is said beside them

installer-install = Instalar
installer-preview = Visualizar alterações
installer-cancel = Cancelar
installer-close = Fechar
installer-open-folder = Abrir pasta de instalação
# { $error } is what went wrong, as Windows says it.
installer-open-folder-failed = Não foi possível abrir a pasta: { $error }
installer-location-blocks = Instalar e Visualizar alterações ficam desativados até que o Local de instalação possa ser usado. O motivo está acima, logo abaixo dele.
installer-package-blocks = Instalar está desativado porque este pacote não pode ir para este local. O motivo está acima.
installer-list-changed = O que isto faria aqui mudou, então nada foi instalado. Revise a lista abaixo e clique em Instalar novamente se for o que você deseja.
installer-cancelled-early = Cancelado. Nada foi alterado.

## How far an install has got, beside its progress bar

# { $step } is one of the steps below. { $done } and { $total } are how many of its items are done, and how many there are.
installer-step-counted = { $step }: { $done } de { $total }…
# { $step } is one of the steps below, when it does not count its items.
installer-step-uncounted = { $step }…
installer-step-getting-ready = Preparando…
installer-step-cancelling = Cancelando após a etapa atual…
installer-step-putting-back = Revertendo o que foi alterado
installer-step-unpacking = Descompactando
installer-step-checking-package = Verificando o pacote
installer-step-recovering = Desfazendo uma instalação anterior que não foi concluída
# What a preview is doing, whatever step it is on.
installer-step-previewing = Calculando o que mudaria
installer-step-runtimes = Instalando runtimes
installer-step-files = Copiando arquivos
installer-step-services = Configurando serviços
installer-step-drivers = Configurando drivers
installer-step-registry = Gravando configurações no Registro
installer-step-license = Restaurando dados de licença
installer-step-tasks = Configurando tarefas agendadas
installer-step-shortcuts = Criando atalhos
installer-step-environment = Definindo variáveis de ambiente
installer-step-settings-entry = Adicionando o aplicativo em Configurações
installer-step-verifying = Verificando o que foi instalado
installer-step-undoing = Desfazendo

## When an install cannot go ahead or stops

installer-carries-no-package = Este instalador não contém nenhum pacote.
installer-stopped-unexpectedly = A instalação parou inesperadamente.
# { $reason } is what went wrong, as the program says it.
installer-stopped-unexpectedly-because = A instalação parou inesperadamente: { $reason }. Se ela já tinha começado, executar um instalador novamente desfaz primeiro o que ela fez.
# { $error } is what went wrong reading it.
installer-package-read-failed = Não foi possível ler o pacote: { $error }
installer-no-programdata = A pasta ProgramData deste PC não foi encontrada, então nada foi instalado: é nela que uma instalação guarda o histórico usado para desfazê-la.
# { $error } is why, such as that there is nowhere only administrators can get into.
installer-staging-failed = Não foi possível criar uma pasta de preparação: { $error }
# { $error } is what went wrong unpacking it.
installer-unpack-failed = Não foi possível descompactar o pacote: { $error }
installer-staging-nowhere = não há nenhum lugar para descompactar o pacote ao qual só administradores tenham acesso
# { $failure } is a folder and what went wrong there, such as "C:\ProgramData\WinAppMigrator: Access is denied.".
installer-staging-nowhere-look-at = não há nenhum lugar para descompactar o pacote ao qual só administradores tenham acesso. O local a verificar é { $failure }

## What came of an install, or of a preview

installer-result = Resultado da instalação
installer-result-preview = Visualização: nada foi alterado ainda
installer-result-installed = O aplicativo foi instalado e passou na verificação de hash.
installer-result-installed-without-left-out = O aplicativo foi instalado e passou na verificação de hash, sem o que está listado abaixo como deixado de fora.
installer-result-not-all-put-back = A instalação falhou, e nem tudo o que ela alterou pôde ser revertido. O que restou está listado abaixo, e a próxima instalação tentará de novo.
installer-result-cancelled-put-back = A instalação foi cancelada, e o que ela alterou foi revertido.
installer-result-failed-put-back = A instalação falhou, e o que ela alterou foi revertido.
installer-result-not-clean = A instalação não foi concluída sem problemas. Revise os problemas abaixo.
installer-result-changes-put-back = Alterações revertidas
installer-result-changes-made = Alterações feitas
installer-result-changes-planned = Alterações planejadas
installer-result-warnings = Avisos
installer-result-errors = Erros
installer-result-package-valid = Hashes do pacote válidos
installer-result-package-invalid = Hashes do pacote INVÁLIDOS
installer-result-files-valid = Arquivos instalados válidos
installer-result-files-invalid = Arquivos instalados INVÁLIDOS
installer-result-reboot = Uma dependência precisa que o PC seja reiniciado para concluir. Reinicie o Windows e abra o aplicativo.
# A heading over the list of what was changed. { $count } is how many.
installer-result-changes = Alterações ({ $count })
# After the first part of a long list. { $count } is how many more there are.
installer-result-changes-more = e mais { $count }.
# Over the path of the install's log file.
installer-log = Log

## The installer's command line. Shown in English: scripts read it. A { $count } is plain digits, with no separators.

installer-cli-stub = Stub de instalador do WinAppMigrator — não contém nenhum pacote de aplicativo.
installer-cli-stub-how = Crie um instalador com o aplicativo WinAppMigrator e execute o .exe gerado na máquina de destino.
installer-cli-usage-title = Instalador autoextraível do WinAppMigrator.
installer-cli-usage = Execute-o sem argumentos para abrir o instalador gráfico, ou:
installer-cli-usage-preview = visualizar a reinstalação: --dry-run [--target-root <PASTA>]
installer-cli-usage-apply = reinstalar:                --apply [--target-root <PASTA>]
installer-cli-no-package = o instalador não contém um pacote incorporado
installer-cli-no-programdata = a pasta ProgramData deste PC não foi encontrada, então nada foi instalado: é nela que uma instalação guarda o histórico usado para desfazê-la
# { $path } is the folder asked for, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-folder-has-files = { $path } não pode ser usado: { $problem }. Use --force somente se você souber que é deste aplicativo
installer-cli-no-location = este pacote não diz onde o aplicativo estava, então informe um --target-root
# { $path } is where the app was on the other PC, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-location-refused = o aplicativo deste pacote estava em { $path }, e esse local não pode ser usado: { $problem }. Informe um --target-root
# { $summary } is how much room it needs where, such as "up to 824.0 MB on C: (356.0 GB free)".
installer-cli-room = Espaço: { $summary }
# { $note } is something to know about the install.
installer-cli-note = Observação: { $note }
# { $count } is how many things it sets up beyond its files.
installer-cli-sets-up = Configura: { $count }
installer-cli-applied = Reinstalação aplicada.
installer-cli-previewed = Reinstalação visualizada.
# { $count } is how many changes the install logged.
installer-cli-changes-logged = Alterações no log: { $count }
# { $count } is how many warnings.
installer-cli-warnings = Avisos: { $count }
# { $count } is how many errors.
installer-cli-errors = Erros: { $count }
# { $count } is how many things the package asked for and did not get.
installer-cli-refused = Recusados: { $count }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-package-valid = Hashes do pacote válidos: { $valid }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-reinstall-valid = Hashes da reinstalação válidos: { $valid }
# { $id } is the install's id, such as wam-0f3a….
installer-cli-install-id = ID da instalação: { $id }
installer-cli-put-back = A instalação falhou, então o que ela alterou foi revertido.
# { $path } is the log file.
installer-cli-log = Log: { $path }
# { $path } is the log file. Printed before an error.
installer-cli-error-log = log: { $path }
# { $error } is what went wrong.
installer-cli-error = erro: { $error }

## The uninstaller's window

uninstaller-window-title = Desinstalar
# The window's title, and its heading. { $app_name } is the name of the app it uninstalls.
uninstaller-window-title-app = Desinstalar { $app_name }
uninstaller-tagline = Desfaz o que o instalador fez
uninstaller-reading = Lendo o histórico desta instalação…
uninstaller-stopped-unexpectedly = A desinstalação parou inesperadamente.
uninstaller-what-will-be-removed = O que será removido
uninstaller-install-location = Local de instalação
uninstaller-files = Arquivos
uninstaller-registry-values = Valores do Registro
uninstaller-services = Serviços
uninstaller-drivers = Drivers
uninstaller-scheduled-tasks = Tarefas agendadas
uninstaller-environment = Ambiente
uninstaller-plan-stays = Os arquivos que você adicionou ou alterou permanecem, com as pastas em que estão.
# { $count } is how many files the install replaced.
uninstaller-plan-coming-back =
    { $count ->
        [0] { $count } arquivos que esta instalação substituiu voltam ou são colocados ao lado de um arquivo que foi alterado desde então.
        [one] { $count } arquivo que esta instalação substituiu volta ou é colocado ao lado de um arquivo que foi alterado desde então.
       *[other] { $count } arquivos que esta instalação substituiu voltam ou são colocados ao lado de um arquivo que foi alterado desde então.
    }
# { $runtimes } is a list of runtimes, such as "Microsoft Visual C++ Redistributable, .NET Desktop Runtime 8".
uninstaller-plan-runtimes = Os runtimes que ela instalou permanecem, porque outros programas podem usá-los: { $runtimes }.
uninstaller-options = Opções
uninstaller-remove-user-data = Remover também os dados do usuário e de licença
# { $count } is how many items of the app's settings, data, and license there are.
uninstaller-remove-user-data-detail =
    { $count ->
        [0] { $count } itens de configurações, dados e licença do aplicativo. Eles permanecem, a menos que esta opção esteja ativada. O que eles substituíram volta ou é colocado ao lado deles, se eles tiverem sido alterados desde então.
        [one] { $count } item de configurações, dados e licença do aplicativo. Ele permanece, a menos que esta opção esteja ativada. O que ele substituiu volta ou é colocado ao lado dele, se ele tiver sido alterado desde então.
       *[other] { $count } itens de configurações, dados e licença do aplicativo. Eles permanecem, a menos que esta opção esteja ativada. O que eles substituíram volta ou é colocado ao lado deles, se eles tiverem sido alterados desde então.
    }
uninstaller-uninstall = Desinstalar
uninstaller-cancel = Cancelar
uninstaller-close = Fechar
# { $done } and { $total } are how many of the install's items are undone, and how many there are.
uninstaller-step-counted = Desfazendo a instalação: { $done } de { $total }…
uninstaller-step = Desfazendo a instalação…
uninstaller-done = O aplicativo foi desinstalado.
uninstaller-done-except = O aplicativo foi desinstalado, exceto o que está listado abaixo.
uninstaller-waits-for-drive = Parte dele está em uma unidade que não está conectada. Conecte-a e desinstale novamente em Configurações.
uninstaller-not-all-undone = Nem tudo pôde ser desfeito. Feche o que o estiver usando e desinstale novamente em Configurações.
# Over the path of the uninstall's log file.
uninstaller-log = Log
uninstaller-undone = Desfeito
uninstaller-left-in-place = Mantido no lugar
uninstaller-could-not-be-undone = Não foi possível desfazer
# After the first part of a long list. { $count } is how many more there are.
uninstaller-and-more = e mais { $count }.
# Said as the reason when there is no record of the install to read.
uninstaller-no-programdata = a pasta ProgramData deste PC não foi encontrada, então não há histórico para ler

## When the uninstaller's window cannot open. Each is a paragraph of a message box, followed by a command to type.

uninstaller-window-failed-title = Desinstalador do WinAppMigrator
# { $error } is what went wrong, as Windows or the graphics driver says it.
uninstaller-window-failed = Não foi possível abrir a janela do desinstalador: { $error }
uninstaller-window-failed-quiet = Ainda é possível desinstalar sem ela, em um prompt de comando de administrador:
# Stands in for this program's own path in that command, if it cannot be found.
uninstaller-this-program = este programa

## The uninstaller's command line (--quiet). Shown in English: scripts read it.

# { $app_name } is the name of the app.
uninstaller-cli-uninstalling = Desinstalando { $app_name }.
# { $path } is the log file.
uninstaller-cli-log = Log: { $path }
# { $error } is what went wrong.
uninstaller-cli-error = erro: { $error }
