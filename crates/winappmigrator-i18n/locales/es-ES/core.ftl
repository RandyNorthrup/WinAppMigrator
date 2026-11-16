# Words the core writes for people: warnings, refusals, errors, and the installer's list of what it sets up.
# Message names start with core-.


## Warnings a package records when it is built, which an installer shows later, maybe on another PC, in that PC's
## language.

# { $count } is how many shortcuts.
core-warning-shortcuts-unreadable =
    { $count ->
        [one] { $count } acceso directo de esta unidad no nombra ningún archivo, así que no se ha podido saber si abre esta aplicación.
       *[other] { $count } accesos directos de esta unidad no nombran ningún archivo, así que no se ha podido saber si abren esta aplicación.
    }

## Undoing an install: what an uninstall, or a failed install, says it put back, left, or could not undo. Each line is
## shown on its own, in a list.

# { $app_name } is the app's name.
core-rollback-record-damaged = No se ha podido leer parte del historial de { $app_name }, así que todo lo que describía sigue en su lugar, y puede que un archivo que reemplazó no vuelva a su lugar.
core-rollback-settings-entry-stays = La entrada de la aplicación en Configuración se queda, para que esto se pueda volver a ejecutar desde ahí.
# { $name } is a runtime's name, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-rollback-prerequisite-stays = { $name } se instaló, o puede que se instalara, y se queda, porque otros programas pueden estar usándolo.
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-drive-not-connected = Lo que esta instalación puso en { $folder } está en una unidad que no está conectada. Conéctala y esto se podrá terminar.
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-record-part-unread = Lo que esta instalación puso en { $folder } se queda, porque no se ha podido leer esa parte del historial.

## Why a file or folder was left, shown after "was left:" in the messages below them. Each is part of a sentence, so
## starts with a small letter and has no full stop.

# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-being-changed = algo estaba cambiando { $folder } mientras se ejecutaba esto, así que no se ha hecho nada más en esa carpeta. Cierra lo que lo esté haciendo y vuelve a ejecutar esto
# { $folder } is a folder; { $error } is what Windows said went wrong.
core-rollback-reason-not-opened = { $folder } no se ha podido abrir: { $error }
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-no-record = no hay constancia de dónde estaba { $folder }
# { $folder } is a folder; { $actual } is the folder it now leads to.
core-rollback-reason-moved = { $folder } ahora lleva a { $actual }, no a donde estaba durante la instalación
# Why a service was not stopped.
core-rollback-reason-did-not-stop = no se ha detenido a tiempo

## Files and folders

# { $path } is a folder; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-folder-left = Se ha dejado la carpeta { $path }: { $reason }.
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-removed = Se ha quitado la carpeta { $path }
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-holds-others = La carpeta { $path } se queda, porque contiene archivos que esta instalación no puso ahí.
# { $path } is a folder; { $error } is what Windows said went wrong.
core-rollback-folder-not-removed = No se ha podido quitar la carpeta { $path }: { $error }.
# { $path } is a file; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-file-left = Se ha dejado el archivo { $path }: { $reason }.
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-folder-gone = { $path } no se restablece, porque la carpeta en la que estaba ya no existe.
# { $path } is a file; { $error } is what went wrong.
core-rollback-partial-not-removed = No se ha podido quitar un archivo parcial: { $path } ({ $error }).
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-checked = No se ha podido comprobar { $path }: { $error }.
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-removed = Se ha quitado { $path }
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-removed = No se ha podido quitar { $path }: { $error }.
# { $path } is a file this install had replaced, and has put back.
core-rollback-file-put-back = Se ha restablecido { $path }
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-put-back = No se ha podido restablecer { $path }: { $error }.
# Why a file could not be put back.
core-rollback-backup-gone = la copia de seguridad ya no existe
# { $path } is a file.
core-rollback-no-parent-folder = { $path } no tiene carpeta principal

## A file that stays. { $why } says why: still-used (something that stays still uses it), changed (it has changed since
## it was installed), or not-asked (it is someone's data, and they did not ask for it to go). { $path } is the file.

core-rollback-file-stays =
    { $why ->
        [still-used] { $path } se queda, porque algo que se queda todavía lo usa.
        [changed] { $path } se queda, porque ha cambiado desde que se instaló.
       *[not-asked] { $path } se queda, ya que no se pidió quitarlo.
    }
# The file it replaced was to be put beside it, under another name, and every name was taken.
core-rollback-file-stays-no-name-beside =
    { $why ->
        [still-used] { $path } se queda, porque algo que se queda todavía lo usa, y el archivo que reemplazó no se ha podido colocar a su lado: todos los nombres posibles están ocupados.
        [changed] { $path } se queda, porque ha cambiado desde que se instaló, y el archivo que reemplazó no se ha podido colocar a su lado: todos los nombres posibles están ocupados.
       *[not-asked] { $path } se queda, ya que no se pidió quitarlo, y el archivo que reemplazó no se ha podido colocar a su lado: todos los nombres posibles están ocupados.
    }
# { $name } is the name the file it replaced now has, such as settings.ini.before-install.
core-rollback-file-stays-beside =
    { $why ->
        [still-used] { $path } se queda, porque algo que se queda todavía lo usa. El archivo que reemplazó está a su lado, como { $name }.
        [changed] { $path } se queda, porque ha cambiado desde que se instaló. El archivo que reemplazó está a su lado, como { $name }.
       *[not-asked] { $path } se queda, ya que no se pidió quitarlo. El archivo que reemplazó está a su lado, como { $name }.
    }
# { $error } is what went wrong.
core-rollback-file-stays-not-beside =
    { $why ->
        [still-used] { $path } se queda, porque algo que se queda todavía lo usa, y el archivo que reemplazó no se ha podido colocar a su lado: { $error }.
        [changed] { $path } se queda, porque ha cambiado desde que se instaló, y el archivo que reemplazó no se ha podido colocar a su lado: { $error }.
       *[not-asked] { $path } se queda, ya que no se pidió quitarlo, y el archivo que reemplazó no se ha podido colocar a su lado: { $error }.
    }

## The registry and environment variables. { $kind } says what { $name } is: key (a registry key, such as
## HKLM\Software\App), value (a registry value, such as HKLM\Software\App\Version), user-variable or machine-variable
## (an environment variable of the user's or of the whole PC, such as APP_HOME), or user-list or machine-list (a
## variable that is a list of folders, such as Path). { $error } is what Windows said went wrong.

# How a registry key's unnamed value is shown, as Registry Editor shows it.
core-registry-default-value-name = (Predeterminado)
core-rollback-setting-other-account =
    { $kind ->
        [key] La clave { $name } se queda, porque pertenece a la cuenta que instaló esto, que no ha iniciado sesión.
        [user-variable] La variable de entorno de usuario { $name } se queda, porque pertenece a la cuenta que instaló esto, que no ha iniciado sesión.
        [machine-variable] La variable de entorno del sistema { $name } se queda, porque pertenece a la cuenta que instaló esto, que no ha iniciado sesión.
        [user-list] La variable { $name } del usuario se queda, porque pertenece a la cuenta que instaló esto, que no ha iniciado sesión.
        [machine-list] La variable { $name } del sistema se queda, porque pertenece a la cuenta que instaló esto, que no ha iniciado sesión.
       *[value] El valor del Registro { $name } se queda, porque pertenece a la cuenta que instaló esto, que no ha iniciado sesión.
    }
core-rollback-setting-not-reached =
    { $kind ->
        [key] No se ha podido acceder a la clave { $name }: { $error }.
        [user-variable] No se ha podido acceder a la variable de entorno de usuario { $name }: { $error }.
        [machine-variable] No se ha podido acceder a la variable de entorno del sistema { $name }: { $error }.
        [user-list] No se ha podido acceder a la variable { $name } del usuario: { $error }.
        [machine-list] No se ha podido acceder a la variable { $name } del sistema: { $error }.
       *[value] No se ha podido acceder al valor del Registro { $name }: { $error }.
    }
core-rollback-setting-not-read =
    { $kind ->
        [key] No se ha podido leer la clave { $name }: { $error }.
        [user-variable] No se ha podido leer la variable de entorno de usuario { $name }: { $error }.
        [machine-variable] No se ha podido leer la variable de entorno del sistema { $name }: { $error }.
        [user-list] No se ha podido leer la variable { $name } del usuario: { $error }.
        [machine-list] No se ha podido leer la variable { $name } del sistema: { $error }.
       *[value] No se ha podido leer el valor del Registro { $name }: { $error }.
    }
core-rollback-setting-changed =
    { $kind ->
        [user-variable] La variable de entorno de usuario { $name } se queda, porque ha cambiado desde que se instaló.
        [machine-variable] La variable de entorno del sistema { $name } se queda, porque ha cambiado desde que se instaló.
        [user-list] La variable { $name } del usuario se queda, porque ha cambiado desde que se instaló.
        [machine-list] La variable { $name } del sistema se queda, porque ha cambiado desde que se instaló.
       *[value] El valor del Registro { $name } se queda, porque ha cambiado desde que se instaló.
    }
# Said when it is back as it was before the install.
core-rollback-setting-put-back =
    { $kind ->
        [user-variable] Se ha restablecido la variable de entorno de usuario { $name }
        [machine-variable] Se ha restablecido la variable de entorno del sistema { $name }
        [user-list] Se ha restablecido la variable { $name } del usuario
        [machine-list] Se ha restablecido la variable { $name } del sistema
       *[value] Se ha restablecido el valor del Registro { $name }
    }
core-rollback-setting-not-put-back =
    { $kind ->
        [user-variable] No se ha podido restablecer la variable de entorno de usuario { $name }: { $error }.
        [machine-variable] No se ha podido restablecer la variable de entorno del sistema { $name }: { $error }.
        [user-list] No se ha podido restablecer la variable { $name } del usuario: { $error }.
        [machine-list] No se ha podido restablecer la variable { $name } del sistema: { $error }.
       *[value] No se ha podido restablecer el valor del Registro { $name }: { $error }.
    }
# A registry value the install wrote, which is the user's data, and stays unless they asked for it to go.
core-rollback-value-kept-by-request = El valor del Registro { $name } se queda, ya que no se pidió quitarlo.
core-rollback-key-holds-others = La clave { $name } se queda, porque contiene elementos que esta instalación no puso ahí.
core-rollback-key-removed = Se ha quitado la clave { $name }
core-rollback-key-not-removed = No se ha podido quitar la clave { $name }: { $error }.
# { $kind } is user-list or machine-list.
core-rollback-list-entries-removed =
    { $kind ->
        [user-list] Se han quitado las entradas de esta instalación de la variable { $name } del usuario
       *[machine-list] Se han quitado las entradas de esta instalación de la variable { $name } del sistema
    }
core-rollback-list-entries-not-removed =
    { $kind ->
        [user-list] No se han podido quitar las entradas de esta instalación de la variable { $name } del usuario: { $error }.
       *[machine-list] No se han podido quitar las entradas de esta instalación de la variable { $name } del sistema: { $error }.
    }

## Services and drivers. { $kind } is service or driver; { $name } is its name, such as AppUpdater. { $error } and
## { $reason } are what Windows said went wrong.

core-rollback-service-not-read =
    { $kind ->
        [driver] No se ha podido leer el controlador { $name }: { $error }.
       *[service] No se ha podido leer el servicio { $name }: { $error }.
    }
core-rollback-service-changed =
    { $kind ->
        [driver] El controlador { $name } se queda, porque ha cambiado desde que se instaló.
       *[service] El servicio { $name } se queda, porque ha cambiado desde que se instaló.
    }
core-rollback-service-not-stopped =
    { $kind ->
        [driver] No se ha podido detener el controlador { $name } ({ $reason }), así que puede que lo que está usando se quede hasta que se reinicie el PC.
       *[service] No se ha podido detener el servicio { $name } ({ $reason }), así que puede que lo que está usando se quede hasta que se reinicie el PC.
    }
core-rollback-service-stopped-for-this =
    { $kind ->
        [driver] El controlador { $name } se ha detenido para hacer esto. Vuelve a iniciarlo cuando lo necesites.
       *[service] El servicio { $name } se ha detenido para hacer esto. Vuelve a iniciarlo cuando lo necesites.
    }
core-rollback-service-not-removed =
    { $kind ->
        [driver] No se ha podido quitar el controlador { $name }: { $reason }.
       *[service] No se ha podido quitar el servicio { $name }: { $reason }.
    }
core-rollback-service-marked-for-removal =
    { $kind ->
        [driver] El controlador { $name } está marcado para quitarse, y se quitará cuando se reinicie el PC o cuando lo libere lo que lo tenga abierto.
       *[service] El servicio { $name } está marcado para quitarse, y se quitará cuando se reinicie el PC o cuando lo libere lo que lo tenga abierto.
    }
core-rollback-service-removed =
    { $kind ->
        [driver] Se ha quitado el controlador { $name }
       *[service] Se ha quitado el servicio { $name }
    }
core-rollback-service-not-restored =
    { $kind ->
        [driver] No se ha podido restablecer el controlador { $name } como estaba: { $reason }.
       *[service] No se ha podido restablecer el servicio { $name } como estaba: { $reason }.
    }
core-rollback-service-restored =
    { $kind ->
        [driver] Se ha restablecido la configuración del controlador { $name }
       *[service] Se ha restablecido la configuración del servicio { $name }
    }
core-rollback-service-not-restarted = El servicio { $name } se estaba ejecutando antes de esta instalación y no se ha vuelto a iniciar: { $reason }.
# { $account } is the account the service ran as, such as CONTOSO\backup.
core-rollback-service-account-again = El servicio { $name } se ejecutaba como { $account }. Su contraseña no se guarda, así que vuelve a establecer su cuenta en Servicios.

## Firewall rules. { $name } is the rule's name; { $error } is what went wrong.

core-rollback-firewall-not-read = No se ha podido leer la regla de firewall { $name }: { $error }.
core-rollback-firewall-says-otherwise = La regla de firewall { $name } se queda, porque lo que hay con ese nombre indica otra cosa.
core-rollback-firewall-changed = La regla de firewall { $name } se queda, porque ha cambiado desde que se instaló.
# The capital "The" is as the app has always said it.
core-rollback-firewall-removed = Se ha quitado la regla de firewall { $name }
core-rollback-firewall-not-removed = No se ha podido quitar la regla de firewall { $name }: { $error }.

## Scheduled tasks. { $path } is a folder of tasks, and { $name } a task with its folder, such as \App\Nightly Update.
## { $error } and { $reason } are what went wrong.

core-rollback-task-folder-holds-others = La carpeta de tareas { $path } se queda, porque contiene tareas que esta instalación no puso ahí.
core-rollback-task-folder-not-removed = No se ha podido quitar la carpeta de tareas { $path }: { $error }.
core-rollback-task-not-read = No se ha podido leer la tarea { $name }: { $error }.
core-rollback-task-changed = La tarea { $name } se queda, porque ha cambiado desde que se instaló.
core-rollback-task-removed = Se ha quitado la tarea { $name }
core-rollback-task-not-removed = No se ha podido quitar la tarea { $name }: { $reason }.
core-rollback-task-put-back = Se ha restablecido la tarea { $name }
core-rollback-task-not-put-back = No se ha podido restablecer la tarea { $name }: { $reason }.

## When an install cannot be undone at all

# { $error } is what went wrong.
core-rollback-record-unreadable = No se ha podido leer el historial de esta instalación, así que no se ha deshecho nada: { $error }.
# { $error } is what went wrong.
core-rollback-nothing-undone = No se ha deshecho nada: { $error }.
# One app's part of the message below. { $app_name } is the app's name; { $failures } is what could not be undone, a
# sentence or more; { $folder } is the folder that holds the install's record.
core-rollback-earlier-install-stuck = { $app_name }: { $failures } Si nunca se puede deshacer, un administrador puede descartarla quitando { $folder }.
# Why an install did not start. { $details } is one or more of the message above.
core-rollback-earlier-installs-not-undone = una instalación anterior que no llegó a terminar no se ha podido deshacer por completo, así que no se ha instalado nada. Cierra lo que esté usando estos elementos y vuelve a intentarlo. { $details }
core-rollback-install-busy = otro instalador o desinstalador está cambiando esta instalación

## The summary an uninstall writes on the command line, one line each, with what each says below it. { $count } is how
## many.

core-rollback-count-undone = Deshecho: { $count }
core-rollback-count-failed = No se ha podido deshacer: { $count }
# What was left on purpose: it has changed since the install, or it is the user's.
core-rollback-count-kept = Se queda en su lugar: { $count }
# What no uninstall can take back, such as a runtime that was installed, which other programs may use.
core-rollback-count-left-behind = No reversible: { $count }

## Which Windows a PC or drive has, as the app shows it beside the drive.

# { $name } is the Windows' name, such as Windows 7 Professional Service Pack 1.
core-system-windows-32-bit = { $name }, 32 bits
# { $name } is the Windows' name, such as Windows 11 Pro.
core-system-windows-64-bit = { $name }, 64 bits
# A drive's Windows, when it only says it is older than Vista.
core-system-windows-xp-or-older = Windows XP o anterior

## Reading a connected drive, or another account's settings. Each is why it could not be done, and some start with a
## small letter, as they follow other words.

core-system-needs-administrator = leer una unidad conectada o la configuración de otra cuenta requiere permisos de administrador
core-system-no-program-data = no se ha encontrado la carpeta ProgramData de este PC
# { $path } is a file, such as D:\Windows\System32\config\SOFTWARE.
core-system-not-a-hive-file = no es un archivo de subárbol: { $path }
# { $path } is a file; { $size } is its size in bytes.
core-system-hive-too-large = { $path } ocupa { $size } bytes, demasiado para ser un subárbol del Registro, o más de lo que podrían sumar los subárboles de esta unidad
# { $key } is a registry key, such as Software\Microsoft\Windows\CurrentVersion.
core-system-hive-no-mapping = el subárbol sin conexión no tiene ninguna asignación para HKLM\{ $key }
# { $key } is where a drive's registry was to be loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said.
# "reg load" is a command's name.
core-system-hive-not-loaded = error de reg load { $key }: { $error }
# { $key } is where a drive's registry was loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said. "reg
# unload" is a command's name.
core-system-hive-not-unloaded = error de reg unload { $key }: { $error }
# { $drive } is a drive, such as D:\.
core-system-not-a-system-drive = { $drive } no es una unidad de sistema de Windows (no tiene carpeta \Windows).
# { $drive } is a drive, such as C:\. "This PC" is the name of a choice in the app.
core-system-own-system-drive = { $drive } es la propia unidad de sistema de este PC. Elige Este PC para leer este PC.
# { $drive } is a drive, such as D:\.
core-system-registry-through-link = { $drive } no es una unidad de sistema de Windows: el camino a su Registro pasa por un vínculo.

## Warnings a scan records about whose data it read.

# { $account } is an account's name or id, such as alice or S-1-5-21-...-1001.
core-warning-scan-account-not-on-record = La cuenta { $account } no está registrada aquí, así que no se han incluido datos ni configuración por usuario de nadie.
# { $account } is an account's name, such as CONTOSO\alice.
core-warning-scan-signed-in-account-no-profile = La cuenta que ha iniciado sesión aquí, { $account }, no tiene ningún perfil registrado, así que no se han incluido datos ni configuración por usuario de nadie.
# { $reason } is why Windows could not say.
core-warning-scan-signed-in-account-unknown = No se ha podido saber quién ha iniciado sesión ({ $reason }), así que no se han incluido datos ni configuración por usuario de nadie. Elige de quién incluirlos.
core-warning-scan-last-account-unknown = Esta unidad no indica quién inició sesión por última vez, y tiene más de una cuenta, así que no se han incluido datos ni configuración por usuario de nadie. Elige de quién incluirlos.
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-settings-unreadable = No se ha podido leer la configuración de { $account }, así que no se ha incluido: { $error }
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-classes-unreadable = No se han podido leer las clases COM ni los tipos de archivo registrados para { $account }, así que no se han incluido: { $error }

## Paths a scan did not follow. { $count } is how many; { $paths } is some of them, separated by semicolons; { $more } is
## how many more there are, not shown.

core-warning-scan-drive-paths-not-followed =
    { $count ->
        [one] No se ha podido leer una ruta de acceso registrada en esta unidad, así que no se ha incluido nada de ahí. Está en otro disco, no es una ruta de acceso completa o pasa por un vínculo, que saldría de la unidad: { $paths }
       *[other] No se han podido leer algunas rutas de acceso registradas en esta unidad, así que no se ha incluido nada de ahí. Cada una está en otro disco, no es una ruta de acceso completa o pasa por un vínculo, que saldría de la unidad: { $paths }
    }
core-warning-scan-drive-paths-not-followed-more = No se han podido leer algunas rutas de acceso registradas en esta unidad, así que no se ha incluido nada de ahí. Cada una está en otro disco, no es una ruta de acceso completa o pasa por un vínculo, que saldría de la unidad: { $paths } y { $more } más
core-warning-scan-network-paths-not-followed =
    { $count ->
        [one] No se ha podido leer una ruta de acceso registrada en este PC, así que no se ha incluido nada de ahí. Está fuera de los discos de este PC, donde examinarla supondría conectarse a ella, o tiene un nombre que abriría otra cosa: { $paths }
       *[other] No se han podido leer algunas rutas de acceso registradas en este PC, así que no se ha incluido nada de ahí. Cada una está fuera de los discos de este PC, donde examinarla supondría conectarse a ella, o tiene un nombre que abriría otra cosa: { $paths }
    }
core-warning-scan-network-paths-not-followed-more = No se han podido leer algunas rutas de acceso registradas en este PC, así que no se ha incluido nada de ahí. Cada una está fuera de los discos de este PC, donde examinarla supondría conectarse a ella, o tiene un nombre que abriría otra cosa: { $paths } y { $more } más

## What a scan with none of an account's data leaves out of a package, in the app's list of what was left out. Each
## has a name, and below it why.

core-account-left-out-shortcuts = Accesos directos de las carpetas propias de esta cuenta
# { $count } is how many shortcuts.
core-account-left-out-shortcuts-why =
    { $count ->
        [one] { $count } acceso directo, que solo se incluye con la opción Datos de usuario
       *[other] { $count } accesos directos, que solo se incluyen con la opción Datos de usuario
    }
core-account-left-out-variables = Variables de entorno de esta cuenta
# { $count } is how many variables.
core-account-left-out-variables-why =
    { $count ->
        [one] { $count } variable, que solo se incluye con la opción Datos de usuario
       *[other] { $count } variables, que solo se incluyen con la opción Datos de usuario
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-account-left-out-task = La tarea programada { $task }
core-account-task-works-on-account-folder = Trabaja con un lugar de las carpetas de una cuenta, o lleva el nombre de uno, y solo se incluye con la opción Datos de usuario
core-account-task-runs-from-account-folder = Lo que también ejecuta desde las carpetas de una cuenta solo se incluye con la opción Datos de usuario
# { $service } is a service's name, such as AppUpdater.
core-account-left-out-service = El servicio { $service }
core-account-service-works-on-account-folder = Trabaja con un lugar de las carpetas de una cuenta, y solo se incluye con la opción Datos de usuario

## What a package was found to still say of whose PC it came from, after a build with none of an account's data.

# The parts of a package, as the warning below names them.
core-account-part-files = las rutas de acceso de los archivos propios de la aplicación
core-account-part-registry = valores del Registro
core-account-part-settings-entry = la entrada de la aplicación en Configuración
core-account-part-main-program = dónde está su programa principal
core-account-part-shortcuts = accesos directos
core-account-part-tasks = tareas programadas
core-account-part-services = servicios
core-account-part-variables = variables de entorno
# One place in the list below. { $place } is one of the parts above, or a file's name; { $count } is how many times.
core-account-mention-place = { $place } ({ $count })
# { $kind } is what was found: account (an account's name), folder (where an account's folder is), computer (the PC's
# name), or id (an account's id). { $places } is a list of where, from the message above.
core-warning-scan-package-mentions =
    { $kind ->
        [account] Este paquete menciona el nombre de una cuenta en { $places }. Se creó sin datos de ninguna cuenta, y cualquiera que tenga el instalador puede leer esto.
        [folder] Este paquete menciona dónde está la carpeta de una cuenta en { $places }. Se creó sin datos de ninguna cuenta, y cualquiera que tenga el instalador puede leer esto.
        [computer] Este paquete menciona el nombre del PC en { $places }. Se creó sin datos de ninguna cuenta, y cualquiera que tenga el instalador puede leer esto.
       *[id] Este paquete menciona el id. de una cuenta en { $places }. Se creó sin datos de ninguna cuenta, y cualquiera que tenga el instalador puede leer esto.
    }
# { $count } is how many folders.
core-warning-scan-folders-of-others-unreadable =
    { $count ->
        [one] No se ha podido leer { $count } carpeta fuera de la aplicación, así que puede que falten datos de la aplicación que haya ahí.
       *[other] No se han podido leer { $count } carpetas fuera de la aplicación, así que puede que falten datos de la aplicación que haya ahí.
    }

## What a scan leaves out because it looks like a credential, or is a license with the License option off, in the app's
## list of what was left out. Each has a name, and below it why.

core-secret-why-credential = parece una credencial
core-secret-why-license-option-off = la opción Datos de licencia y activación está desactivada
# { $count } is how many values of a registry key.
core-secret-why-account-values =
    { $count ->
        [one] { $count } valor, que solo se incluye con la opción Datos de usuario
       *[other] { $count } valores, que solo se incluyen con la opción Datos de usuario
    }
# Shown in place of a registry value's name when its name itself looks like a password or a key.
core-secret-name-holds-credential = (un valor cuyo nombre contiene una credencial)
# Shown in place of an option's name, for an argument on a command line that has no name.
core-secret-unnamed-argument = un argumento que contiene una credencial
# { $option } is an option taken out of a command line, such as /password, or the message above. { $place } says
# where the command line was: service, task, shortcut, opener (what opens a file type), opener-unnamed (the same, of a
# type with no name), or uninstall. { $name } is the service, task, shortcut, file type, or app.
core-secret-taken-from =
    { $place ->
        [service] { $option } en cómo se ejecuta { $name }
        [task] { $option } en cómo se ejecuta la tarea { $name }
        [shortcut] { $option } en el acceso directo { $name }
        [opener] { $option } en lo que abre { $name }
        [opener-unnamed] { $option } en lo que abre un tipo de archivo
       *[uninstall] { $option } en cómo se desinstala { $name }
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-secret-task-definition-unreadable = La definición de la tarea { $task }, que no se ha podido leer para comprobarla
# { $name } is an environment variable's name, such as APP_TOKEN.
core-secret-left-out-variable = La variable de entorno { $name }
# { $class } is a COM class's id, such as {00000000-0000-0000-0000-000000000000}.
core-secret-left-out-class-server = Lo que la clase { $class } indica que ejecuta
# { $rule } is a firewall rule's name.
core-secret-left-out-firewall-description = La descripción de la regla de firewall { $rule }

## Licenses a scan found, as the package's report describes each.

# { $name } is a file's name, such as license.lic.
core-license-detail-file = Archivo de licencia detectado: { $name }
# { $framework } is a licensing product's name, such as FlexLM / FlexNet.
core-license-detail-store = Almacén de licencias de { $framework } presente
# { $name } is an environment variable's name, such as LM_LICENSE_FILE.
core-license-detail-variable = Variable de entorno de licencia { $name }
# { $name } is a registry value's name, such as SerialNumber.
core-license-detail-registry-value = Valor del Registro que parece contener datos de licencia: { $name }
# Said of a registry value in an account's key, which is not named.
core-license-detail-registry-value-of-an-account = Valor del Registro que parece contener datos de licencia
# { $key } is a registry key, such as HKLM\Software\Contoso\License.
core-license-detail-registry-key = Clave del Registro que parece contener datos de licencia: { $key }

## Warnings a scan records about licenses.

core-warning-scan-license-data = Se han detectado datos de licencia o activación. Con la opción Datos de licencia y activación activada, el instalador restaura lo que se puede trasladar; los valores de todo el equipo y los almacenes vinculados al hardware solo se indican. WinAppMigrator nunca descifra, omite la activación ni emula llaves de hardware.
core-warning-scan-license-machine-bound = Algunas licencias están vinculadas al equipo (llave de hardware, activación bloqueada al equipo o cifrado). El instalador restaura lo que puede; puede que estas sigan necesitando una nueva activación en el PC nuevo.

## Warnings a scan records about the registry. What could not be read is not named in them: it is kept beside them.

# { $error } is what Windows said went wrong. App Paths is a registry key's name.
core-warning-scan-app-paths-key-unreadable = No se ha podido leer una subclave de App Paths: { $error }
core-warning-scan-registry-limit = La captura del Registro ha alcanzado el límite de seguridad configurado.
core-warning-scan-registry-subtree-limit = La captura de una rama del Registro ha alcanzado el límite de seguridad configurado.
# { $error } is what Windows said went wrong.
core-warning-scan-registry-key-unreadable = No se ha podido leer la clave del Registro: { $error }
core-warning-scan-registry-value-too-large = Un valor del Registro ha superado el límite de tamaño de captura configurado.

## Scheduled tasks

core-warning-scan-tasks-folder-through-link = La carpeta de tareas programadas de esta unidad pasa por un vínculo, así que no se han leído sus tareas.
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-warning-scan-task-left-out = Se ha excluido la tarea programada { $task }: se ejecuta con una cuenta distinta de aquella cuya configuración se incluye, otros usuarios aparte de los administradores pueden cambiar su archivo, o la aplicación tiene demasiadas para conservarlas.
# { $error } is what Windows said went wrong.
core-warning-scan-tasks-folder-unreadable = No se ha podido leer parte de las tareas programadas: { $error }
# { $path } is a task's file, such as C:\Windows\System32\Tasks\App\Nightly Sync; { $reason } is one of the reasons
# below, or what Windows said went wrong.
core-warning-scan-task-unreadable = No se ha podido leer la tarea programada { $path }: { $reason }. Si ejecuta la aplicación, no se ha incluido.
# Why a task's file could not be read. { $size } is its size in bytes.
core-task-too-large = ocupa { $size } bytes, demasiado para ser una tarea
# Why a task's file could not be read.
core-task-ends-mid-character = termina a mitad de un carácter
# Why a task's file could not be read. { $error } says where.
core-task-not-utf16 = no es UTF-16 válido: { $error }
# Why a task's file could not be read. { $error } says where.
core-task-not-text = no es UTF-8 ni UTF-16 válido: { $error }
# What a package records of how a task is put back. Register-ScheduledTask is a command's name.
core-task-restore-instructions = La restauración usa Register-ScheduledTask con la definición XML archivada.

## Reading a program's file. Each is why it could not be read, after the program's name.

# { $reason } is what went wrong.
core-pe-unreadable = no se ha podido leer: { $reason }
core-pe-headers-misaligned = sus encabezados no están en los límites en los que deben estar los de un programa
# { $size } is the file's size in bytes.
core-pe-too-large = ocupa { $size } bytes, demasiado para ser un programa
# PE is the name of the format Windows programs are in.
core-pe-no-headers = no tiene encabezados PE

## Warnings a scan records about what a program needs. Which program is kept beside each.

# { $error } is what went wrong.
core-warning-scan-imports-unreadable = No se ha podido leer la tabla de importaciones: { $error }
# { $error } is what went wrong.
core-warning-scan-import-name-unreadable = No se ha podido leer el nombre de un descriptor de importación: { $error }
# { $count } is how many were skipped.
core-warning-scan-legacy-delay-imports =
    { $count ->
        [one] Se ha omitido { $count } descriptor de carga diferida heredado (v1); solo se analizan las importaciones diferidas modernas basadas en RVA.
       *[other] Se han omitido { $count } descriptores de carga diferida heredados (v1); solo se analizan las importaciones diferidas modernas basadas en RVA.
    }

## The DLLs a program needs. Which DLL or program is kept beside each warning.

# { $path } is a program, such as C:\Program Files\App\app.exe.
core-dll-program-has-no-folder = el ejecutable no tiene directorio principal: { $path }
# Why a DLL is among the app's files, in the package's list of them.
core-dll-beside-program = DLL situada junto al ejecutable seleccionado
# { $error } is what went wrong.
core-warning-scan-dependency-unscanned = No se ha podido analizar una dependencia recursiva: { $error }
# { $name } is a DLL's name, such as d3dx9_43.dll.
core-warning-scan-dependency-not-in-newer-windows = { $name } estaba en la carpeta de Windows del PC en el que estaba esta aplicación, y Windows 10 y 11 no la incluyen. Algo la instaló ahí, así que no forma parte de los archivos propios de la aplicación y no se incluye. Puede que la aplicación no se inicie hasta que se instale lo que la proporciona.
# { $error } is what went wrong.
core-warning-scan-dependency-not-hashed = No se ha podido calcular el hash de la dependencia: { $error }
# SystemRoot and windir are the names of environment variables.
core-warning-scan-windows-folder-unknown = Las variables de entorno SystemRoot/windir no están disponibles; la búsqueda de DLL del sistema es limitada.
# { $error } is what went wrong.
core-warning-scan-dependency-architecture-unreadable = No se ha podido leer la arquitectura de la dependencia: { $error }

## Services, drivers, shortcuts, and environment variables a scan reads. Where it was reading is kept beside each
## warning.

# { $error } is what Windows said went wrong.
core-warning-scan-services-unreadable = No se ha podido leer la clave del Registro de servicios: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-drivers-unreadable = No se ha podido leer la clave del Registro de servicios para detectar controladores: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-queried = No se han podido consultar los accesos directos: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-parsed = No se ha podido analizar el resultado de la consulta de accesos directos: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-environment-unreadable = No se ha podido leer la clave del Registro del entorno del sistema: { $error }

## What a package records of how a driver is put back, which an installer shows. { $kind } is the driver's type as
## Windows names it (kernel, file-system, recognizer, or unknown-type); { $name } is its service's name; { $path } is
## its file. { $signed } is yes when { $signer } names who signed it, and no when that is not known.

core-service-driver-bundled =
    { $signed ->
        [yes] Controlador { $kind } propio de la aplicación "{ $name }" ({ $path }, firmado por { $signer }). El instalador incluye su .sys y vuelve a registrar el servicio del controlador automáticamente (sc create, requiere permisos elevados), y después confirma que el servicio está registrado.
       *[no] Controlador { $kind } propio de la aplicación "{ $name }" ({ $path }, firma desconocida). El instalador incluye su .sys y vuelve a registrar el servicio del controlador automáticamente (sc create, requiere permisos elevados), y después confirma que el servicio está registrado.
    }
core-service-driver-from-vendor =
    { $signed ->
        [yes] Controlador { $kind } de fabricante o integrado en Windows "{ $name }" ({ $path }, firmado por { $signer }). WinAppMigrator no redistribuye controladores de terceros; reinstálalo con el paquete de controlador firmado del fabricante en el equipo de destino antes de abrir la aplicación.
       *[no] Controlador { $kind } de fabricante o integrado en Windows "{ $name }" ({ $path }, firma desconocida). WinAppMigrator no redistribuye controladores de terceros; reinstálalo con el paquete de controlador firmado del fabricante en el equipo de destino antes de abrir la aplicación.
    }

## Runtimes an app needs, such as the Visual C++ runtime or .NET: how each is provided, as the Dependencies list says.

core-runtime-handling-built-in = Integrado en Windows
core-runtime-handling-ships-with-app = Viene con la aplicación
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-handling-windows-feature = Característica de Windows: { $feature }
core-runtime-handling-included = Incluido
# { $id } is the runtime's name in winget, such as Microsoft.VCRedist.2015+.x64.
core-runtime-handling-winget = Se instala con winget: { $id }
core-runtime-handling-manual = Instalar manualmente

## Downloading a runtime's installer, when an installer is built. Each is why it could not be, and starts with a small
## letter, as it follows other words. { $url } is a web address; { $error } is what went wrong.

core-runtime-download-not-https = no se descarga { $url }: no es HTTPS
# TLS is what keeps a download private.
core-runtime-tls-failed = error al inicializar TLS: { $error }
core-runtime-download-failed = error al descargar { $url }: { $error }
core-runtime-download-unreadable = error al leer { $url }: { $error }
core-runtime-download-empty = la descarga de { $url } no ha devuelto datos
# { $came } and { $expected } are numbers of bytes.
core-runtime-download-cut-short = la descarga de { $url } se ha interrumpido: han llegado { $came } de { $expected } bytes

## Installing a runtime: why it was not installed. Each starts with a small letter, as it follows other words.

core-runtime-no-known-installer = no hay ningún instalador conocido para este entorno de ejecución
# { $path } is where in the package, such as prerequisites\vc_redist.x64.exe.
core-runtime-payload-misplaced = el paquete indica que { $path } es el instalador de este entorno de ejecución, y no está donde le corresponde, así que no se ha ejecutado
# { $path } is a file.
core-runtime-payload-missing = falta el instalador incluido: { $path }
# { $error } is what went wrong.
core-runtime-payload-path-invalid = la ruta de acceso del instalador incluido no es válida: { $error }
# { $url } is the web address it can be downloaded from. winget is a program's name.
core-runtime-no-installer-route = no hay instalador incluido ni id. de winget; consíguelo en { $url }
# { $error } is what went wrong.
core-runtime-no-temp-folder = no se ha podido crear una carpeta para desempaquetarlo: { $error }
# { $reason } is what the check found.
core-runtime-payload-signature-failed = el instalador incluido no ha superado la comprobación de firma y no se ha ejecutado: { $reason }
core-runtime-payload-timed-out = el instalador incluido no ha terminado dentro del tiempo de espera y se ha finalizado
# { $error } is what went wrong.
core-runtime-payload-not-started = no se ha podido iniciar el instalador incluido: { $error }
core-runtime-installer-killed = el instalador se ha finalizado sin código de salida
# { $code } is the number it ended with, such as 1603.
core-runtime-installer-failed = el instalador ha terminado con el código { $code }
# { $feature } is a Windows feature's name, such as NetFx3; { $code } is the number DISM ended with. Windows Features is
# the name of a Windows window. Keep the command as it is.
core-runtime-feature-failed = DISM no ha podido habilitar la característica de Windows { $feature } (código de salida { $code }); probablemente no se ha podido descargar la carga útil. Habilítala en Características de Windows o ejecuta `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`.
# The same, when DISM gave no number.
core-runtime-feature-failed-no-code = DISM no ha podido habilitar la característica de Windows { $feature } (código de salida desconocido); probablemente no se ha podido descargar la carga útil. Habilítala en Características de Windows o ejecuta `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`.
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-feature-timed-out = la habilitación de la característica de Windows { $feature } no ha terminado dentro del tiempo de espera y se ha finalizado
# { $feature } is a Windows feature's name; { $error } is what went wrong.
core-runtime-feature-no-dism = DISM no está disponible para habilitar la característica de Windows { $feature }: { $error }

## Installing a runtime with winget. { $id } is its name in winget, such as Microsoft.VCRedist.2015+.x64.

# { $error } is what went wrong.
core-runtime-winget-not-looked-for = no se ha podido buscar winget: { $error }
core-runtime-winget-not-installed = winget no está instalado
# { $path } is where winget is.
core-runtime-winget-misplaced = { $path } no está donde Windows guarda sus propios paquetes, así que no se ha ejecutado
# { $path } is where winget is; { $reason } is what the check found.
core-runtime-winget-signature-failed = { $path } no ha superado la comprobación de firma y no se ha ejecutado: { $reason }
# { $reason } is one of the two messages above, or what went wrong.
core-runtime-winget-unusable = no se ha podido usar winget para instalar { $id }: { $reason }
# { $code } is the number winget ended with.
core-runtime-winget-failed = winget install { $id } ha terminado con el código { $code }
core-runtime-winget-failed-no-code = winget install { $id } ha terminado con un código desconocido
core-runtime-winget-timed-out = winget install { $id } no ha terminado dentro del tiempo de espera y se ha finalizado
# { $error } is what went wrong.
core-runtime-winget-not-started = winget no está disponible para instalar { $id }: { $error }

## What told a scan how an app was installed, as the package's report lists it. The names of installers and files are
## as their makers write them.

# { $path } is an installer's record of what it installed, such as C:\Program Files\App\unins000.dat.
core-source-record-not-believed = { $path } no se ha usado: otros usuarios aparte de los administradores pueden cambiarlo
# { $count } is how many Windows Installer packages the bundle holds.
core-source-burn-bundle =
    { $count ->
        [one] Lote de Burn con { $count } paquete de Windows Installer
       *[other] Lote de Burn con { $count } paquetes de Windows Installer
    }
# { $count } is how many paths Windows Installer recorded.
core-source-msi-database =
    { $count ->
        [one] Base de datos de Windows Installer: { $count } ruta de acceso de clave de componente
       *[other] Base de datos de Windows Installer: { $count } rutas de acceso de clave de componente
    }
core-source-inno-log = Archivo de registro de desinstalación de Inno Setup (unins000.dat)
# { $count } is how many paths the log records.
core-source-inno-log-paths =
    { $count ->
        [one] Archivo de registro de desinstalación de Inno Setup: { $count } ruta de acceso registrada
       *[other] Archivo de registro de desinstalación de Inno Setup: { $count } rutas de acceso registradas
    }
core-source-nsis = Firma del desinstalador de NSIS (Nullsoft)
# { $count } is how many paths the log records.
core-source-nsis-log-paths =
    { $count ->
        [one] uninstall.log de NSIS AdvUninstLog: { $count } ruta de acceso registrada
       *[other] uninstall.log de NSIS AdvUninstLog: { $count } rutas de acceso registradas
    }
core-source-squirrel = Estructura por usuario de Squirrel (Update.exe / app-*)
# { $version } is a folder's name that says the version, such as app-1.2.3.
core-source-squirrel-version = Versión actual de Squirrel: { $version }
core-source-chocolatey = Metadatos del paquete de Chocolatey
core-source-msix = Paquete MSIX / Appx
# { $identity } is a package's name, such as Contoso.App.
core-source-appx-identity = Identidad de AppxManifest: { $identity }
# { $count } is how many folders.
core-source-msix-data =
    { $count ->
        [one] Datos por usuario del paquete MSIX: { $count } ubicación
       *[other] Datos por usuario del paquete MSIX: { $count } ubicaciones
    }
# App Paths is a registry key's name.
core-source-app-paths = Entrada de App Paths

## Why a runtime is needed, as the package's report says it.

# { $program } is a program's file name, such as app.exe; { $dll } is a DLL's, such as msvcp140.dll.
core-runtime-program-imports = { $program } importa { $dll }

## What else of an app's a scan finds in the registry: fonts and certificates, and why each is the app's.

# Why a font is among the app's files, in the package's list of them.
core-registry-font-recorded = Una fuente que nombra la información de instalación de la aplicación
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-named-for-the-app = lleva el nombre de la aplicación o de su editor
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-recorded = la información de instalación de la aplicación lo nombra

## Who is signed in, for the Whose Data choice. Each of the first six is why it could not be told, and starts with a
## small letter, as it is shown inside other words. An account is shown by its id, such as S-1-5-21-...-1001.

# { $error } is what Windows said.
core-profile-desktop-owner-not-said = Windows no ha indicado de quién es este escritorio: { $error }
# { $error } is what went wrong.
core-profile-desktop-owner-not-asked = no se ha podido preguntar a Windows de quién es este escritorio: { $error }
core-profile-signed-in-account-no-profile = la cuenta que ha iniciado sesión aquí, { $account }, no tiene ningún perfil registrado
core-profile-desktop-not-a-person = el escritorio de aquí pertenece a { $account }, que no es la cuenta de una persona
core-profile-nobody-signed-in-running-account-no-profile = nadie ha iniciado sesión aquí, y la cuenta que ejecuta esto, { $account }, no tiene ningún perfil registrado
core-profile-nobody-signed-in-running-not-a-person = nadie ha iniciado sesión aquí, y { $account }, que ejecuta esto, no es la cuenta de una persona
# SID is the name Windows gives an account's id.
core-profile-nobody-signed-in-no-sid = nadie ha iniciado sesión aquí, y esta cuenta no tiene SID
# { $accounts } is a list of accounts.
core-profile-several-desktops = más de una cuenta tiene un escritorio en esta sesión: { $accounts }
# Under the Whose Data choice. { $reason } is one of the messages above.
core-profile-signed-in-unknown = No se ha podido saber quién ha iniciado sesión ({ $reason }).
# Under the Whose Data choice, when there are accounts to choose from. { $reason } is one of the messages above.
core-profile-signed-in-unknown-choose = No se ha podido saber quién ha iniciado sesión ({ $reason }). Elige de quién son los datos que se incluyen.
