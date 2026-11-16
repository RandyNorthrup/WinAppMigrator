# Words of WinAppMigrator Client, the console program run on the old PC, the one an app is moving off, so that
# WinAppMigrator on the new PC can read it over the network. Message names start with client-.
#
# Its window is a console, which on Windows 7 is 80 columns wide. Each line break in a message is one on the screen:
# keep every line shorter than 80 characters, and break lines where your language needs to, not only where English
# does.


## Starting

# Shown only in English, in place of a right-to-left language the console cannot show. { $language } is that
# language's name in English, such as Arabic.
client-console-rtl-fallback = Esta consola no puede mostrar { $language }, así que el cliente usa el inglés.
# The client copies itself to a folder in Program Files before it starts. { $error } is what Windows said went wrong.
client-safe-folder-failed =
    WinAppMigrator Client no ha podido iniciarse desde una carpeta que solo
    pueden cambiar los administradores: { $error }
client-press-enter-to-close = Presiona Entrar para cerrar esta ventana.
# The first line of its screen. { $version } is the client's version, such as 0.6.2.
client-heading = WinAppMigrator Client { $version }
client-intro =
    Este programa permite que WinAppMigrator, en tu PC nuevo, lea a través de la
    red las aplicaciones de este PC. Aquí no cambia nada: lee una instantánea de
    la unidad de Windows, que elimina cuando lo detienes.
# { $error } is why: one of the messages under "Why it stopped" below, or what Windows said.
client-stopped = El cliente se ha detenido: { $error }
# The caretaker is a copy of the client with no window that cleans up after it. { $error } is what Windows said.
client-caretaker-failed =
    Si se cierra esta ventana en lugar de detener el cliente, la instantánea y las
    reglas de firewall permanecen hasta la próxima vez que se ejecute el cliente
    aquí ({ $error }).
# Shown while the snapshot is taken, followed on the same line by client-snapshot-taken. Keep the quotes: they keep
# the space before it.
client-snapshot-taking = Creando una instantánea de la unidad de Windows...{ " " }
client-snapshot-taken = listo.
# The old PC answers the new one's search for it on the network. { $error } is what Windows said went wrong.
client-discovery-failed =
    Este PC no aparecerá por sí solo ({ $error }).
    En el PC nuevo, escribe en su lugar el nombre o la dirección de este PC.

## This PC and its pairing code. The three lines are lined up in two columns, behind two spaces; keep the quotes, which
## keep those spaces.

# { $computer_name } is this PC's name, such as OLD-PC.
client-this-pc = { "  " }Este PC:                   { $computer_name }
# { $windows_name } is this PC's Windows, such as Windows 7 Professional Service Pack 1. { $bits } is 32 or 64.
client-windows = { "  " }Windows:                   { $windows_name }, { $bits } bits
# { $code } is six digits in two groups, such as 482 913.
client-pairing-code = { "  " }Código de emparejamiento:  { $code }
# Network PC... is the button of that name in WinAppMigrator: use the name it has there in your language.
client-pairing-steps =
    En tu PC nuevo, en WinAppMigrator, elige PC en red... y escribe este código.
    Cierra primero la aplicación que vas a mover, para que sus archivos queden
    tal como los dejó.
    Para detener el cliente, presiona Entrar aquí o cierra esta ventana.

## While it runs. { $from } is the network address of the PC that connected, such as 192.168.1.20, or "unknown" when
## it could not be told, which the [unknown] variant words.

client-paired =
    Emparejado con { $from ->
        [unknown] otro PC
       *[address] { $from }
    }. Está leyendo lo que necesita...
# { $count } is how many requests it answered.
client-builder-done =
    { $from ->
        [unknown] Otro PC
       *[address] { $from }
    } ha terminado ({ $count ->
        [one] { $count } solicitud respondida
       *[other] { $count } solicitudes respondidas
    }).
# { $error } is what went wrong, often in Windows' words.
client-link-ended =
    Ha terminado la conexión con { $from ->
        [unknown] otro PC
       *[address] { $from }
    }: { $error }
client-wrong-code =
    { $from ->
        [unknown] Otro PC
       *[address] { $from }
    } ha probado un código que no es este.
# { $error } is what went wrong, often in Windows' words.
client-pair-failed =
    { $from ->
        [unknown] Otro PC
       *[address] { $from }
    } no ha podido emparejarse: { $error }
client-too-many-wrong-codes =
    Se han probado demasiados códigos incorrectos, así que ahora ningún PC puede
    emparejarse con este. Puede que algo en tu red esté intentando adivinarlo.
    Para emparejar, detén el cliente, vuelve a iniciarlo y escribe el nuevo
    código que muestre.
# { $code } is six digits in two groups, such as 482 913.
client-new-code = Demasiados códigos incorrectos. El nuevo código de emparejamiento es: { $code }

## Stopping

client-stopping = Deteniendo: eliminando la instantánea y las reglas de firewall...
client-stop-done = Listo.

## Why it stopped, shown after "The client stopped: " (client-stopped), and written to follow it.

client-already-running = el cliente ya se está ejecutando en este PC, o está limpiando lo que dejó otro que se cerró. Usa ese o espera un momento y vuelve a iniciar este.
# { $drive } is the drive Windows is on, such as C:. { $details } is what Windows said, in its own words.
client-snapshot-failed = Windows no ha creado una instantánea de { $drive }: { $details }
# { $details } is what Windows Firewall said, in its own words.
client-firewall-failed = el Firewall de Windows no ha permitido que el cliente escuche: { $details }

## What it takes, shown for --help. The options, such as --lang, are typed as they are and are not translated; keep
## their descriptions lined up.

client-usage =
    WinAppMigrator Client admite estas opciones:
    
      --lang <idioma>    Usa este idioma, como it-IT, si el cliente lo tiene,
                         en lugar del idioma de presentación de Windows.
      --code <código>    Empieza con este código de emparejamiento de seis
                         dígitos, para un script que inicie también WinAppMigrator
                         en el PC nuevo.
      --help             Muestra esta ayuda.
