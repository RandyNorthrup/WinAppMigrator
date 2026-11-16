# Words of WinAppMigrator Client, the console program run on the old PC, the one an app is moving off, so that
# WinAppMigrator on the new PC can read it over the network. Message names start with client-.
#
# Its window is a console, which on Windows 7 is 80 columns wide. Each line break in a message is one on the screen:
# keep every line shorter than 80 characters, and break lines where your language needs to, not only where English
# does.


## Starting

# Shown only in English, in place of a right-to-left language the console cannot show. { $language } is that
# language's name in English, such as Arabic.
client-console-rtl-fallback = Este console não consegue exibir { $language }, então o cliente usará inglês.
# The client copies itself to a folder in Program Files before it starts. { $error } is what Windows said went wrong.
client-safe-folder-failed =
    O WinAppMigrator Client não pôde ser iniciado de uma pasta que só
    administradores podem alterar: { $error }
client-press-enter-to-close = Pressione Enter para fechar esta janela.
# The first line of its screen. { $version } is the client's version, such as 0.6.2.
client-heading = WinAppMigrator Client { $version }
client-intro =
    Isto permite que o WinAppMigrator no seu PC novo leia pela rede os
    aplicativos deste PC. Nada é alterado aqui: ele lê um instantâneo da unidade
    do Windows, que é removido quando você o interrompe.
# { $error } is why: one of the messages under "Why it stopped" below, or what Windows said.
client-stopped = O cliente parou: { $error }
# The caretaker is a copy of the client with no window that cleans up after it. { $error } is what Windows said.
client-caretaker-failed =
    Se esta janela for fechada em vez de interrompida, o instantâneo e as regras
    de firewall continuam aqui até a próxima vez que o cliente for executado
    neste PC ({ $error }).
# Shown while the snapshot is taken, followed on the same line by client-snapshot-taken. Keep the quotes: they keep
# the space before it.
client-snapshot-taking = Criando um instantâneo da unidade do Windows...{ " " }
client-snapshot-taken = concluído.
# The old PC answers the new one's search for it on the network. { $error } is what Windows said went wrong.
client-discovery-failed =
    Este PC não vai aparecer sozinho ({ $error }).
    No PC novo, digite o nome ou o endereço deste PC.

## This PC and its pairing code. The three lines are lined up in two columns, behind two spaces; keep the quotes, which
## keep those spaces.

# { $computer_name } is this PC's name, such as OLD-PC.
client-this-pc = { "  " }Este PC:                   { $computer_name }
# { $windows_name } is this PC's Windows, such as Windows 7 Professional Service Pack 1. { $bits } is 32 or 64.
client-windows = { "  " }Windows:                   { $windows_name }, { $bits } bits
# { $code } is six digits in two groups, such as 482 913.
client-pairing-code = { "  " }Código de emparelhamento:  { $code }
# Network PC... is the button of that name in WinAppMigrator: use the name it has there in your language.
client-pairing-steps =
    No seu PC novo, no WinAppMigrator, escolha PC na rede... e digite este
    código. Antes, feche o aplicativo que você está transferindo, para que os
    arquivos dele fiquem como ele os deixou.
    Para interromper, pressione Enter aqui ou feche esta janela.

## While it runs. { $from } is the network address of the PC that connected, such as 192.168.1.20, or "unknown" when
## it could not be told, which the [unknown] variant words.

client-paired =
    Emparelhado com { $from ->
        [unknown] outro PC
       *[address] { $from }
    }. Lendo o que ele precisa...
# { $count } is how many requests it answered.
client-builder-done =
    { $from ->
        [unknown] Outro PC
       *[address] { $from }
    } terminou ({ $count ->
        [0] { $count } solicitações respondidas
        [one] { $count } solicitação respondida
       *[other] { $count } solicitações respondidas
    }).
# { $error } is what went wrong, often in Windows' words.
client-link-ended =
    A conexão com { $from ->
        [unknown] outro PC
       *[address] { $from }
    } terminou: { $error }
client-wrong-code =
    { $from ->
        [unknown] Outro PC
       *[address] { $from }
    } tentou um código que não é este.
# { $error } is what went wrong, often in Windows' words.
client-pair-failed =
    { $from ->
        [unknown] Outro PC
       *[address] { $from }
    } não conseguiu emparelhar: { $error }
client-too-many-wrong-codes =
    Foram tentados códigos errados demais, então nenhum PC pode se emparelhar
    com este agora. Algo na sua rede pode estar tentando adivinhar. Para
    emparelhar, interrompa o cliente, inicie-o novamente e digite o novo código
    que ele mostrar.
# { $code } is six digits in two groups, such as 482 913.
client-new-code = Códigos errados demais. O novo código de emparelhamento é: { $code }

## Stopping

client-stopping = Interrompendo: removendo o instantâneo e as regras de firewall...
client-stop-done = Concluído.

## Why it stopped, shown after "The client stopped: " (client-stopped), and written to follow it.

client-already-running = o cliente já está em execução neste PC ou está fazendo a limpeza depois de um que foi fechado. Use esse ou aguarde um momento e inicie este novamente.
# { $drive } is the drive Windows is on, such as C:. { $details } is what Windows said, in its own words.
client-snapshot-failed = o Windows não criou um instantâneo de { $drive }: { $details }
# { $details } is what Windows Firewall said, in its own words.
client-firewall-failed = o Firewall do Windows não permitiu que o cliente recebesse conexões: { $details }

## What it takes, shown for --help. The options, such as --lang, are typed as they are and are not translated; keep
## their descriptions lined up.

client-usage =
    O WinAppMigrator Client aceita estas opções:
    
      --lang <idioma>    Usar este idioma, como it-IT, se o cliente o tiver,
                         em vez do idioma de exibição do Windows.
      --code <código>    Começar com este código de emparelhamento de seis
                         dígitos, para um script que também inicia o
                         WinAppMigrator no PC novo.
      --help             Mostrar isto.
