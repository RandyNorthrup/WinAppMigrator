# Words of the app, WinAppMigrator itself: its one window, with the installed apps listed on the left and, on the
# right, what goes in an installer of the one picked and the button that builds it; and the steps to read another PC
# through WinAppMigrator Client. Message names start with app-.


## The window and its header

# The program's name, in the window's title bar and at the top of the window. Keep it as it is unless your language
# writes product names in a script of its own.
app-name = WinAppMigrator
app-tagline = Convierte una aplicación instalada en un instalador autoextraíble
app-theme-to-light = Cambiar al tema claro
app-theme-to-dark = Cambiar al tema oscuro
app-reload-list = Volver a cargar la lista de aplicaciones
app-list-loading = Cargando
# { $count } is how many apps are listed.
app-list-count =
    { $count ->
        [one] { $count } aplicación
       *[other] { $count } aplicaciones
    }
app-list-load-failed = Error al cargar
# Shown when the window cannot open at all. { $error } is what went wrong, in the words of Windows or a graphics driver.
app-window-could-not-open = No se ha podido abrir la ventana: { $error }

## The list of installed apps

app-list-search = Buscar por nombre o editor  (Ctrl+F)
app-list-filter-all = Todas
# Whether an app is installed for one account or for everyone on the PC.
app-scope-per-user = Por usuario
app-scope-per-machine = Por equipo
app-scope-unknown = Desconocido
# An app whose processor architecture (x86, x64, ARM64) is not known.
app-arch-unknown = Desconocida
app-list-loading-apps = Cargando las aplicaciones instaladas…
# { $error } is why the apps could not be listed.
app-list-could-not-list = No se han podido enumerar las aplicaciones: { $error }
app-list-no-matches = No hay aplicaciones que coincidan
app-list-no-matches-hint = Cambia el texto de búsqueda o el filtro de ámbito.
app-list-worker-disconnected = Se ha desconectado el proceso de inventario.
# What a screen reader says of an app's row. { $name } is the app's name, { $publisher } who made it, and { $version } its
# version, such as 11.4.2, or — when either is not known.
app-list-row = { $name }, { $publisher }, versión { $version }

## The list's columns, and what a screen reader says each column's heading does when clicked

app-column-name = Nombre
app-column-name-sort = Ordenar por nombre
app-column-publisher = Editor
app-column-publisher-sort = Ordenar por editor
app-column-version = Versión
app-column-version-sort = Ordenar por versión
app-column-scope = Ámbito
app-column-scope-sort = Ordenar por ámbito
# Short for architecture: x86, x64, or ARM64. The column is narrow.
app-column-arch = Arq.
app-column-arch-sort = Ordenar por arquitectura

## Where apps are read from: this PC, a drive from another PC, a disk image of one, or another PC on the network

app-source = Origen del análisis
app-source-this-pc = Este PC
app-source-drive = Unidad conectada…
app-source-image = Imagen de disco…
app-source-network = PC en red…
app-source-attaching-image = Montando la imagen en modo de solo lectura…
# { $computer } is the other PC's name, such as OLD-PC, and { $address } its network address, such as 192.168.1.20.
app-source-network-pc = { $computer } a través de la red ({ $address })
app-source-disconnect = Desconectar
app-source-change = Cambiar…
# The title of the window that picks a disk image. .vhd and .vhdx are file extensions.
app-source-pick-image-title = Selecciona una imagen de disco del otro PC (.vhd o .vhdx)
app-source-pick-image-filter = Imágenes de disco
# { $path } is the disk image file.
app-source-image-has-no-windows = { $path } no tiene ningún volumen con Windows.
app-source-image-not-attached = No se ha podido montar la imagen de disco.
app-source-pick-drive-title = Selecciona la raíz de la unidad conectada (por ejemplo, E:\)
# { $path } is the folder picked, such as E:\. \Windows and \Users are folder names, as Windows writes them.
app-source-not-a-windows-drive = { $path } no parece una unidad de sistema de Windows (no tiene \Windows ni \Users). Elige la raíz de la unidad del otro PC.
# Said of a drive or disk image with a Windows older than Windows 10. { $windows } is its name, such as "Windows 7
# Professional Service Pack 1, 32-bit". { $is_32_bit } is "yes" when that Windows is 32-bit, which adds the last
# sentence, and "no" when it is not.
app-source-older-windows =
    Contiene { $windows }. Los instaladores creados a partir de ella están hechos para Windows 10 y 11. Con la opción Datos de usuario, se incluye la configuración que Windows apartó discretamente para una aplicación antigua, y los entornos de ejecución que Windows 11 no tiene, como DirectX 9, se instalan con la aplicación.{ $is_32_bit ->
        [yes] { " " }Sus aplicaciones son de 32 bits, así que van a Program Files (x86), donde las guarda Windows de 64 bits.
       *[no] { "" }
    }
# The same, of a Windows whose name could not be read. { $is_32_bit } is as above.
app-source-older-windows-unnamed =
    Contiene una versión anterior de Windows. Los instaladores creados a partir de ella están hechos para Windows 10 y 11. Con la opción Datos de usuario, se incluye la configuración que Windows apartó discretamente para una aplicación antigua, y los entornos de ejecución que Windows 11 no tiene, como DirectX 9, se instalan con la aplicación.{ $is_32_bit ->
        [yes] { " " }Sus aplicaciones son de 32 bits, así que van a Program Files (x86), donde las guarda Windows de 64 bits.
       *[no] { "" }
    }

## The details panel, before an app is picked, and the app picked

app-details-pair-first = Emparejar con el PC antiguo
app-details-pair-first-hint = Cuando WinAppMigrator Client se esté ejecutando allí y hayas escrito su código, sus aplicaciones aparecerán aquí y podrás crear un instalador de una de ellas igual que desde este PC.
app-details-pick-an-app = Elige una aplicación
app-details-pick-an-app-hint = Selecciona una aplicación de la lista para elegir qué incluir y crear su instalador.
# { $publisher } is who made the app, and { $version } its version, such as 11.4.2.
app-details-publisher-and-version = { $publisher }, versión { $version }
# { $version } is the app's version, such as 11.4.2.
app-details-unknown-publisher-and-version = Editor desconocido, versión { $version }
app-details-unknown-publisher = Editor desconocido

## What goes in besides the program

app-option-license = Datos de licencia y activación
app-option-license-hint = Pueden estar vinculados a este PC y necesitar una nueva activación en el PC nuevo.
app-option-user-data = Datos de usuario
# Documents, Saved Games, and ProgramData are folders of Windows: use the names Windows gives them in your language,
# except ProgramData, which keeps its name.
app-option-user-data-hint = Configuración e historial, Documentos, Juegos guardados y ProgramData.
app-option-whose-data = De quién son los datos
app-option-whose-data-hint = Un instalador contiene los datos de una sola cuenta. Van a quien haya iniciado sesión en el PC donde se ejecute.

## Building the installer

app-output-folder = Carpeta de salida
# An example of a folder, shown in the empty field. "You" stands for the name of an account.
app-output-folder-example = C:\Users\Usuario\WinAppMigrator
app-output-folder-not-full = Escribe la ruta de acceso completa de una carpeta, como D:\Instaladores.
app-build-create = Crear instalador
app-build-create-again = Volver a crear el instalador
app-build-cancel = Cancelar
app-build-cancelling = Cancelando después del paso actual…
app-build-cancelled = Creación cancelada. No se ha escrito ningún instalador.
# winappmigrator-installer.exe is a file name.
app-build-no-stub = No se ha encontrado el programa base del instalador (winappmigrator-installer.exe) junto a esta aplicación, así que no se puede crear un instalador autoextraíble.
app-build-select-an-app-first = Primero selecciona una aplicación para migrar.
app-build-look-again-first = Hay que volver a analizar esta aplicación antes de poder crear su instalador.
# { $reason } is what went wrong, in the program's own words, often in English.
app-build-stopped-unexpectedly = La creación se ha detenido de forma inesperada: { $reason }. Si ha dejado un instalador en la carpeta de salida, no lo uses.
app-build-worker-disconnected = Se ha desconectado el proceso de migración.
# { $path } is the output folder, and { $error } what Windows said went wrong.
app-build-no-output-folder = No se ha podido crear la carpeta de salida { $path }: { $error }
# { $error } is what went wrong.
app-build-no-staging-folder = No se ha podido crear una carpeta de preparación protegida, así que no se ha creado nada: { $error }
# { $error } is what went wrong, as a sentence.
app-build-could-not-build = No se ha podido crear el instalador. { $error }
app-build-no-installer = La creación ha terminado sin ningún instalador.
# { $error } is what went wrong.
app-build-hash-check-did-not-run = Se ha escrito el instalador, pero no se ha podido ejecutar su comprobación de hash: { $error }. No lo uses.

## How far a build has got

app-progress-preparing = Calculando qué se incluye…
app-progress-checking-room = Comprobando que hay espacio…
app-progress-bundling = Empaquetando entornos de ejecución y controladores…
# { $done } is how many files have been read so far, and { $total } how many there are.
app-progress-packing = Leyendo y comprimiendo archivos: { $done } de { $total }
# { $done } is how many of the installer's records have been looked over so far, and { $total } how many there are.
app-progress-looking-over = Revisando lo que dice el instalador: { $done } de { $total }…
app-progress-writing = Escribiendo el instalador…
app-progress-verifying = Comprobando hashes…

## What a build made

app-result-failed-hash-check = No uses este instalador: no ha superado su comprobación de hash.
# { $count } is how many files were gone when the build came to read them.
app-result-files-gone =
    { $count ->
        [one] Instalador creado, pero { $count } archivo había desaparecido desde que se analizó la aplicación, así que no está incluido. Consulta Advertencias más abajo.
       *[other] Instalador creado, pero { $count } archivos habían desaparecido desde que se analizó la aplicación, así que no están incluidos. Consulta Advertencias más abajo.
    }
# { $count } is how many files could not be read into the installer.
app-result-files-not-captured =
    { $count ->
        [one] Instalador creado, pero no se ha podido capturar { $count } archivo, normalmente porque estaba bloqueado o no se podía leer. Puede que la aplicación no funcione sin él. Consulta Advertencias más abajo.
       *[other] Instalador creado, pero no se han podido capturar { $count } archivos, normalmente porque estaban bloqueados o no se podían leer. Puede que la aplicación no funcione sin ellos. Consulta Advertencias más abajo.
    }
app-result-mentions-who = Instalador creado, pero todavía revela algo de quién procede. Lee lo que dice abajo antes de dárselo a nadie.
app-result-ready = Instalador listo. Su paquete ha superado la comprobación de hash.
app-result-files = Archivos
app-result-installer-size = Tamaño del instalador
app-result-warnings = Advertencias
app-result-log = Archivo de registro
app-result-copy-error = Copiar error
app-result-installer = Instalador
app-result-data-file = Archivo de datos
app-result-open-folder = Abrir carpeta
app-result-copy-path = Copiar ruta de acceso
# { $error } is what Windows said went wrong.
app-result-could-not-open-folder = No se ha podido abrir la carpeta: { $error }
# { $installer } and { $data_file } are file names, such as App_installer.exe and App_installer.wamdata.
app-result-two-files = Esta aplicación es demasiado grande para un solo ejecutable, así que su instalador son dos archivos: { $installer } y { $data_file }. Tienen que ir juntos: copia los dos al otro PC, en la misma carpeta y con estos nombres. Si firmas el instalador, esa firma también cubre el archivo de datos.
# { $app_name } is the app's name. "Unknown publisher" is what Windows shows when it asks to run an unsigned program: use
# Windows' own words for it in your language.
app-result-run-it = Ejecútalo en el otro PC para reinstalar { $app_name }. Comprueba sus archivos con SHA-256. Windows muestra "Editor desconocido" porque los instaladores que creas no están firmados.
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-verification-errors = Errores de verificación ({ $count })
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-warnings-heading = Advertencias ({ $count })
# After the first of a long list of warnings. { $count } is how many more there are.
app-warnings-and-more = y { $count } más.

## Looking at an app, to see what an installer of it would hold

app-scan-getting-ready = Preparando el análisis de esta aplicación…
app-scan-install-record = Leyendo cómo se instaló…
app-scan-near-matches = Buscando qué más podría formar parte de ella…
app-scan-system-items = Buscando sus servicios, tareas y accesos directos…
app-scan-files = Revisando sus archivos…
app-scan-registry = Leyendo qué la integra en Windows…
app-scan-finishing = Finalizando…
app-scan-look-again = Volver a analizar
app-scan-stop = Detener
app-scan-running-hint = Mientras se ejecuta, puedes elegir otra aplicación o cambiar lo que se incluye.
# { $error } is why, in words that follow a colon.
app-scan-failed = No se ha podido analizar esta aplicación: { $error }
app-scan-try-again = Volver a intentarlo
app-scan-stopped = Detenido. Se podrá crear un instalador cuando se haya analizado esta aplicación.
# { $reason } is what went wrong, in the program's own words, often in English.
app-scan-stopped-unexpectedly = Se ha detenido de forma inesperada: { $reason }.

## How big an installer can be, and whether there is room for it

app-room-files = Archivos
# Over how big the installer can be: up to that size.
app-room-up-to = Hasta
app-room-free = Libre
# { $drive } is a drive, such as C:.
app-room-free-on = Libre en { $drive }
# How much room a drive has, when it would not say.
app-room-free-unknown = Desconocido
app-room-most-it-could-need = Lo máximo que podría necesitar, antes de comprimir.
# { $drive } is a drive, such as C:, { $needed } how much room it needs, such as 1.2 GB, and { $free } how much it has.
app-room-drive-short = { $drive } necesita hasta { $needed } libres para crear esto y tiene { $free }.
# { $drives } is one sentence like the one above for each drive without the room, one after another.
app-room-not-enough = No hay espacio suficiente. { $drives } Elige una carpeta de salida en otra unidad o excluye algo.
# { $note } is a sentence, without its full stop, about a drive whose room could not be checked. Use your language's
# full stop.
app-room-could-not-tell = { $note }.

## What goes in the installer, part by part

app-footprint-heading = Qué incluye el instalador
app-footprint-working-out = Calculando qué va con eso…
# { $error } is why.
app-footprint-would-not-make-a-package = Lo que queda no formaría un paquete: { $error }
# { $list } is the names of what else is left out, such as a service and a shortcut.
app-footprint-also-left-out = Eso también excluye { $list }, porque lo que ejecutan o nombran no estaría ahí. No se configurarán en el otro PC.
# { $list } is the names of the first few of what else is left out, and { $count } how many more there are.
app-footprint-also-left-out-and-more = Eso también excluye { $list } y { $count } más, porque lo que ejecutan o nombran no estaría ahí. No se configurarán en el otro PC.
# { $list } is the names of license files. The option is the one named "License and Activation Data".
app-footprint-licenses-still-go = Los datos de licencia de lo que has excluido se incluyen igualmente, porque dependen de la opción Datos de licencia y activación: { $list }.
# { $elapsed } is how long ago, in minutes and seconds such as 45:00, or hours, minutes, and seconds such as 1:05:00.
# "Look Again" names the button of that name.
app-footprint-looked-at-long-ago = Esto se analizó hace { $elapsed }. Los archivos se leen al crear el instalador, así que se recogen los cambios y se indica lo que haya desaparecido. Usa Volver a analizar para lo que sea nuevo.
# Headings of the parts of the list, each with a line that says what is in it.
app-footprint-program-files = Archivos de programa
app-footprint-data-folders = Carpetas de datos
# What could go in with the User Data option, when nothing else is in this part. { $size } is how much, such as 1.2 GB.
app-footprint-folders-with-user-data = { $size } con la opción Datos de usuario
# { $included } is how many folders go in, { $count } how many there are, and { $size } how much those that go in hold.
app-footprint-folders-summary =
    { $count ->
        [one] { $included } de { $count } carpeta, { $size }
       *[other] { $included } de { $count } carpetas, { $size }
    }
# { $size } is how much, such as 1.2 GB, and { $count } in how many files.
app-footprint-data-only-with-user-data =
    { $count ->
        [one] { $size } en { $count } archivo de datos de esta aplicación solo se incluirán con la opción Datos de usuario.
       *[other] { $size } en { $count } archivos de datos de esta aplicación solo se incluirán con la opción Datos de usuario.
    }
# A folder's row. { $count } is how many files it holds, and { $detail } where it is or why it cannot be left out.
app-footprint-folder-row =
    { $count ->
        [one] { $count } archivo. { $detail }
       *[other] { $count } archivos. { $detail }
    }
app-footprint-services = Servicios
app-footprint-startup = Se inicia automáticamente
app-footprint-tasks = Tareas programadas
app-footprint-shortcuts = Accesos directos
# { $included } is how many of the part's things go in, and { $count } how many there are.
app-footprint-items-summary = { $included } de { $count }
# { $why } is what else was left out that this goes with.
app-footprint-left-out-too = También excluido: { $why }
# A row's detail. { $kind } is what it is, such as Service, and { $detail } what it does.
app-footprint-change-detail = { $kind }. { $detail }
app-footprint-integrations = Qué la integra en Windows
# { $included } is how many kinds go in, such as file types and firewall rules, and { $count } how many kinds there are.
app-footprint-integrations-summary =
    { $count ->
        [one] { $included } de { $count } tipo
       *[other] { $included } de { $count } tipos
    }
app-footprint-integration-nothing-left = No queda nada, debido a lo que has excluido
# { $count } is how many things of this kind the installer sets up.
app-footprint-integration-things =
    { $count ->
        [one] { $count } elemento configurado
       *[other] { $count } elementos configurados
    }
app-footprint-prerequisites = Qué necesita para ejecutarse
# { $count } is how many runtimes, such as Visual C++ Redistributable.
app-footprint-prerequisites-summary =
    { $count ->
        [one] { $count } entorno de ejecución
       *[other] { $count } entornos de ejecución
    }
app-footprint-near-matches = También podría formar parte de ella
# { $ticked } is how many were ticked, and { $count } how many there are.
app-footprint-near-matches-summary = { $ticked } de { $count } agregados
app-footprint-near-matches-hint = No se incluyen en el instalador a menos que los marques. Al marcar uno, se vuelve a analizar la aplicación con las mismas comprobaciones que el resto, así que lo que parezca una contraseña sigue quedando fuera.
# { $why } is why it might be part of the app, as a sentence.
app-footprint-near-match-with-user-data = Se incluye con la opción Datos de usuario. { $why }
# How much a folder holds, when not all of it could be measured. { $size } is how much was, such as 1.2 GB.
app-footprint-near-match-over = más de { $size }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-folder = Carpeta: { $place }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-hidden = Parte oculta de una aplicación: { $place }
app-footprint-nothing-left-out = No se ha excluido nada por parecer una contraseña, una licencia o un certificado.
app-footprint-left-out = Qué se ha excluido y por qué
# { $count } is how many things were left out.
app-footprint-left-out-summary =
    { $count ->
        [one] { $count } elemento
       *[other] { $count } elementos
    }
# Above a list of things left out for one reason. { $title } is the reason.
app-footprint-left-out-group = { $title }:
app-footprint-refused = Ningún instalador tendría permiso para configurar esto:
# A heading over a list that opens and closes. { $count } is how many are in it.
app-footprint-worth-knowing = Conviene saber ({ $count })
# { $list } is what kinds of things were looked for and none found, from the five below.
app-none-found = No se han encontrado: { $list }.
app-none-found-services = servicios
app-none-found-startup = elementos de inicio
app-none-found-tasks = tareas programadas
app-none-found-shortcuts = accesos directos
app-none-found-integrations = elementos que la integren en Windows

## Network PC: reading another PC through WinAppMigrator Client, in three steps. "WinAppMigrator Client" is the name of a
## program, and the old PC is the one the apps are moved off.

app-network-step-client = 1. Copia el cliente en el PC antiguo
app-network-step-client-hint = WinAppMigrator Client se ejecuta en el PC donde está ahora la aplicación, con Windows 7 o posterior. Solo permite que este PC lo lea, de forma cifrada, mediante un código que muestra.
# { $drive } is a USB drive, such as E:\.
app-network-save-to-usb = Guardar en la unidad USB { $drive }
# { $drive } is a USB drive, such as E:\, and { $name } the name it was given, such as BACKUP.
app-network-save-to-named-usb = Guardar en la unidad USB { $drive } ({ $name })
app-network-save-to-folder = Guardar en una carpeta…
app-network-look-for-usb-again = Volver a buscar unidades USB
app-network-no-usb = No hay ninguna unidad USB conectada. Conecta una o guarda el cliente en una carpeta a la que puedas acceder desde el PC antiguo, como una carpeta compartida de tu red.
# { $path } is where the client was saved, such as E:\WinAppMigrator Client.exe.
app-network-client-saved = Guardado como { $path }. En el PC antiguo, cierra la aplicación que vas a mover, haz doble clic en este archivo y permite que haga cambios. Mostrará un código.
app-network-step-pick = 2. Elige el PC antiguo
app-network-looking = Buscando en tu red equipos que ejecuten el cliente…
app-network-type-name = O escribe su nombre o dirección
# An example, shown in the empty field: a PC's name, or its network address.
app-network-type-name-example = PC-ANTIGUO o 192.168.1.20
app-network-step-code = 3. Escribe el código que muestra
app-network-code = Código de emparejamiento
app-network-connect = Conectar
app-network-pairing = Emparejando…
# Why pairing did not work, shown on its own.
app-network-pairing-stopped = el emparejamiento se ha detenido antes de terminar
app-network-wrong-code = ese no es el código que muestra el PC antiguo. Compruébalo y vuelve a intentarlo.
# { $error } is what went wrong.
app-network-not-reached = no se ha podido conectar con el PC antiguo: { $error }
app-network-pick-folder-title = Elige una carpeta para WinAppMigrator Client
# { $path } is the folder, and { $error } what Windows said went wrong.
app-client-could-not-save = no se ha podido guardar el cliente en { $path }: { $error }
# scripts\build-client.ps1 is a file name.
app-client-not-carried = esta compilación de WinAppMigrator no incluye el cliente. Compílalo con scripts\build-client.ps1 y vuelve a compilar la aplicación.
# { $error } is what Windows said went wrong.
app-client-could-not-save-note = no se ha podido guardar la nota que lo acompaña: { $error }
# A note saved as a text file beside the client, for whoever runs it on the old PC. Keep its lines short, as it is read in
# Notepad. "WinAppMigrator Client.exe" is a file name. "Network PC..." names the Scan Source button: use its words in your
# language.
app-client-note =
    WinAppMigrator Client
    
    Ejecútalo en el PC del que vas a mover las aplicaciones: Windows 7, 8, 10 u 11.
    
    1. Cierra la aplicación que vas a mover.
    2. Haz doble clic en "WinAppMigrator Client.exe" y permite que haga cambios.
    3. Muestra un código de emparejamiento de 6 dígitos. En WinAppMigrator, en tu
       PC nuevo, elige PC en red..., selecciona este PC y escribe el código.
    4. Cuando el PC nuevo termine, presiona Entrar en la ventana del cliente o
       ciérrala.
    
    Solo lee. Mientras se ejecuta, mantiene una instantánea de la unidad de
    Windows y deja pasar sus dos puertos por el Firewall de Windows desde tu red
    local, y elimina ambas cosas cuando se detiene. Lo que pasa por la red va
    cifrado, y solo un PC que tenga el código puede leer algo.
