# Words the core writes for people as it installs an app from a package: why an install will not start, the list of
# what it did (or for a preview, what it would do) that the installer window shows, its warnings, the record it keeps of
# each install so that it can be undone, and the core's errors. Message names start with core-.
#
# The list of what an install did is short and plain, one line for each thing, as the installer window lists them:
# "copied", "restored", "would copy". Words such as --force and --apply are command-line options, and LocalService,
# SYSTEM, ProgramData, and %AppData% are as Windows names them: keep them as they are.


## Errors: what went wrong, when the core cannot go on. Each is shown on its own, or after words that say what was being
## done.

core-error-list-changed = lo que haría esta instalación ha cambiado desde que se aceptó, así que no se ha instalado nada
# { $path } is a file or folder, such as C:\Program Files\App\app.exe, or words that name one, such as "the installer".
# { $error } is what Windows said went wrong.
core-error-io = error de E/S en { $path }: { $error }
# { $detail } is what went wrong.
core-error-registry = error al leer el Registro: { $detail }
# { $path } is a program file. { $detail } is what was wrong with it.
core-error-pe = error al analizar el formato PE de { $path }: { $detail }
# { $detail } is what was wrong, already in words.
core-error-invalid-input = entrada no válida: { $detail }
# { $detail } is what went wrong.
core-error-serialization = error al serializar el informe: { $detail }
# { $detail } is what went wrong.
core-error-archive = error en la operación con el archivo comprimido: { $detail }
# { $detail } is what went wrong.
core-error-network = error en la operación de red: { $detail }
# { $detail } is what went wrong.
core-error-time-format = error al dar formato a la hora: { $detail }
core-error-cancelled = se ha cancelado
# What is said when part of the program stopped unexpectedly and gave no reason.
core-error-panic-no-reason = no se ha indicado ningún motivo

## The record of each install, kept so that it can be undone.

# { $kind } is a number that says what type of registry value it is.
core-journal-unknown-value-type = tipo de valor del Registro desconocido: { $kind }
core-journal-not-elevated = sin permisos elevados
core-journal-needs-admin = para instalar se necesitan permisos de administrador, para guardar su historial donde otros no puedan cambiarlo
# { $path } is a folder.
core-journal-not-an-install-folder = esta no es la carpeta de una instalación: { $path }
# { $path } is the file the record is kept in.
core-journal-not-written = no se ha podido escribir el historial de esta instalación: { $path }
# { $path } is a folder.
core-journal-path-escapes-root = la ruta de acceso sale de la raíz prevista: { $path }
# { $path } is a folder.
core-journal-nothing-exists = nada de esto existe en este PC: { $path }
core-journal-busy = se está ejecutando otra instalación o desinstalación. Espera a que termine y vuelve a intentarlo
# Said of a file that was to get back the permissions it had.
core-journal-no-folder = no tiene carpeta
# Said of the permissions a file had, which were kept to give back to it.
core-journal-permissions-unreadable = no se ha podido leer lo que se guardó
# { $folder } is the folder of one install's record.
core-journal-unreadable-empty = el historial de la instalación en { $folder } no se puede usar: está vacío
# { $folder } is the folder of one install's record.
core-journal-unreadable-no-header = el historial de la instalación en { $folder } no se puede usar: no empieza indicando a qué corresponde
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-version = el historial de la instalación en { $folder } no se puede usar: lo escribió otra versión de este programa
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-install = el historial de la instalación en { $folder } no se puede usar: indica que pertenece a otra instalación
# { $id } is an install's id, such as wam-1a2b3c.
core-journal-not-an-install-id = este no es el id. de una instalación: { $id }

## Why an install will not start, or stopped before it changed anything.

core-restore-apply-and-dry-run = no se pueden indicar --apply y --dry-run a la vez
# { $path } is the folder that was chosen to install in.
core-restore-location-not-full-path = la ubicación de instalación debe ser una ruta de acceso completa: "{ $path }"
# { $problem } says what is wrong with the folder, already in words. { $path } is the folder.
core-restore-location-unusable = esa ubicación de instalación no se puede usar: { $problem }: "{ $path }"
core-restore-needs-admin-plan = el plan de restauración incluye cambios en el Registro, servicios, tareas programadas, accesos directos o destinos protegidos que solo puede hacer un administrador
# { $path } is the folder that was chosen to install in.
core-restore-needs-admin-target = la raíz de destino requiere permisos elevados: { $path }
core-restore-needs-admin-user-data = restaurar datos de usuario en ProgramData u otras ubicaciones protegidas requiere permisos elevados
# { $error } is why the install stopped. { $failed } is what could not be put back, as sentences of their own.
core-restore-not-all-put-back = { $error }. No se ha podido revertir todo lo que se había hecho: { $failed }
# { $error } is what went wrong.
core-restore-nowhere-to-keep-record = no hay ningún lugar seguro donde guardar el historial que permitiría deshacer esta instalación, así que no se ha instalado nada: { $error }
# { $path } is a file the app would have put there.
core-restore-already-exists = { $path } ya existe, así que no se ha instalado nada. Usa --force para reemplazarlo
# { $root } is a folder the app has files for. { $link } is that folder, or one it is in.
core-restore-root-under-link = esta aplicación tiene elementos que restaurar en { $root }, y { $link } es un vínculo a otro lugar que otros podrían cambiar, así que no se ha instalado nada. Si lo que va ahí son datos de usuario o de licencia, instálala sin sus datos de usuario ni sus datos de licencia. Si es un acceso directo de una carpeta Inicio, devuelve esa carpeta a donde estaba
core-restore-hash-check-failed = el paquete no ha superado la comprobación de hash y no se ha instalado nada
# { $detail } is the first thing found wrong, already in words.
core-restore-hash-check-failed-because = el paquete no ha superado la comprobación de hash y no se ha instalado nada: { $detail }
# { $path } is a file or folder.
core-restore-refuse-records-folder = no se escribe donde las instalaciones guardan su historial: { $path }
# { $path } is a file or folder.
core-restore-refuse-outside = no se escribe fuera de la ubicación de instalación y de las carpetas de datos conocidas: { $path }
# { $path } is a file.
core-restore-no-parent-folder = el destino no tiene carpeta principal: { $path }
# { $path } is where a file was to go.
core-restore-folder-in-the-way = hay una carpeta donde tiene que ir este archivo: { $path }
# { $path } is a file in the package.
core-restore-file-changed = { $path } ya no es lo que contiene el paquete, así que no se ha instalado
# { $path } is a file in the package.
core-restore-source-is-link = { $path } es un vínculo, y no se debe seguir
# { $path } is a file in the package.
core-restore-source-became-link = { $path } se ha convertido en un vínculo mientras se abría
# Why a service did not start, after "did not start:".
core-restore-service-stopped-by-itself = se ha vuelto a detener por sí solo
# Why part of the app was left out, after "was left out:".
core-restore-com-class-someone-elses = esta clase COM ya está en este PC y pertenece a otra cosa

## What an install did, one line for each thing, or for a preview, what it would do. { $source } is a file in the
## package, and { $destination } and { $path } are where it goes on this PC. { $key } is a registry key, such as
## HKLM\Software\App.

# { $package_valid } is true or false, and { $restore_valid } is true, false, or not_run: keep them as they are.
core-restore-verification-result = verificación posterior a la restauración package_valid={ $package_valid } restore_valid={ $restore_valid }
# { $name } is a service's name.
core-restore-started-service = se ha iniciado el servicio { $name }
core-restore-would-copy = se copiaría { $source } -> { $destination }
core-restore-copied = se ha copiado { $source } -> { $destination }
core-restore-would-create-empty-folder = se crearía la carpeta vacía { $path }
core-restore-created-empty-folder = se ha creado la carpeta vacía { $path }
# { $value } is the name of a value in the key.
core-restore-would-restore-registry-value = se restauraría en el Registro { $key } { $value }
core-restore-would-restore-registry-key = se restauraría en el Registro { $key } (clave)
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-would-add-firewall-rule = se agregaría la regla de firewall { $name } para { $program }
# { $name } is a firewall rule's name.
core-restore-firewall-rule-already-there = la regla de firewall { $name } ya existe
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-added-firewall-rule = se ha agregado la regla de firewall { $name } para { $program }
# { $name } is a service's name, { $command } what it runs, and { $account } the account it runs as, such as LocalService.
core-restore-would-restore-service = se restauraría el servicio { $name } -> { $command } (se ejecuta como { $account })
# { $name } is a runtime, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-restore-prerequisite-present = el requisito previo ya está presente: { $name }
# { $name } is a runtime that Windows has as a Windows feature.
core-restore-would-install-prerequisite-feature = se instalaría el requisito previo { $name } (característica de Windows)
# { $name } is a runtime whose installer is in the package.
core-restore-would-install-prerequisite-bundled = se instalaría el requisito previo { $name } (instalador incluido)
# { $name } is a runtime that would come through winget, which is a program's name.
core-restore-would-install-prerequisite-winget = se instalaría el requisito previo { $name } (winget)
# { $name } is a runtime.
core-restore-installed-prerequisite = se ha instalado el requisito previo { $name }
# { $name } is a runtime.
core-restore-installed-prerequisite-reboot = se ha instalado el requisito previo { $name } (es necesario reiniciar)
core-restore-would-restore-license = se restauraría la licencia { $source } -> { $destination }
core-restore-restored-license = se ha restaurado la licencia { $source } -> { $destination }
core-restore-would-restore-license-registry = se restauraría la licencia en el Registro { $key }
# { $name } is a driver's name.
core-restore-driver-not-restored = controlador no restaurado automáticamente: { $name }
# { $name } is a driver's name.
core-restore-would-install-driver = se instalaría el controlador { $name } -> { $destination }
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-would-restore-task = se restauraría la tarea programada { $name }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-startup-shortcut = se restauraría el acceso directo de inicio { $shortcut } -> { $target }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-shortcut = se restauraría el acceso directo { $shortcut } -> { $target }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-would-restore-variable =
    { $scope ->
        [user] se restauraría la variable de entorno de usuario { $name }
       *[machine] se restauraría la variable de entorno del sistema { $name }
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-left-variable =
    { $scope ->
        [user] se ha dejado como estaba la variable de entorno de usuario { $name }: nada de ella pertenece a esta aplicación
       *[machine] se ha dejado como estaba la variable de entorno del sistema { $name }: nada de ella pertenece a esta aplicación
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-restored-variable =
    { $scope ->
        [user] se ha restaurado la variable de entorno de usuario { $name }
       *[machine] se ha restaurado la variable de entorno del sistema { $name }
    }
core-restore-set-uninstaller = se ha configurado el desinstalador de { $key }
core-restore-placed-driver-image = se ha colocado la imagen del controlador { $source } -> { $destination }
# { $name } is a driver's name.
core-restore-configured-driver = se ha configurado el controlador { $name }
# { $name } is a driver's name.
core-restore-created-driver = se ha creado el controlador { $name }
core-restore-ensured-registry-key = se ha comprobado que existe la clave del Registro { $key }
# { $value } is the name of a value in the key.
core-restore-restored-registry-value = se ha restaurado en el Registro { $key } { $value }
# "(Default)" is how Windows names the value a key has with no name: write it as Registry Editor does.
core-restore-restored-registry-default = se ha restaurado en el Registro { $key } (Predeterminado)
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-configured-service = se ha configurado el servicio { $name } (se ejecuta como { $account })
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-created-service = se ha creado el servicio { $name } (se ejecuta como { $account })
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-restored-task = se ha restaurado la tarea programada { $name }
core-restore-restored-shortcut = se ha restaurado el acceso directo { $path }

## Warnings an install gives, and why it failed. They are shown after it, on the PC it ran on.

core-warning-restore-earlier-not-trusted = Se ha dejado sin tocar el historial de una instalación que no llegó a terminar, porque otros pueden cambiarlo. Lo que cambió esa instalación sigue ahí.
# { $app_name } is the app's name. { $count } is how many changes were put back.
core-warning-restore-earlier-undone =
    { $count ->
        [one] Una instalación anterior de { $app_name } no llegó a terminar, así que primero se ha deshecho. Se ha revertido { $count } cambio.
       *[other] Una instalación anterior de { $app_name } no llegó a terminar, así que primero se ha deshecho. Se han revertido { $count } cambios.
    }
# { $app_name } is the app's name.
core-warning-restore-earlier-never-finished = Una instalación anterior de { $app_name } no llegó a terminar. Al instalar, primero se deshace.
core-warning-restore-user-data-real-locations = Los datos de usuario se restauran en las ubicaciones reales %AppData%/%ProgramData% de este equipo en lugar de en la raíz de destino.
# { $error } is what went wrong.
core-warning-restore-commit-failed = No se ha podido completar el historial de esta instalación: { $error }
# { $name } is a service's name.
core-warning-restore-service-still-starting = Se ha pedido que se inicie el servicio { $name } y todavía no ha terminado de iniciarse. Puede que aún lo haga. También se inicia por sí solo la próxima vez que se inicie este PC.
# { $name } is a service's name. { $reason } is why it did not start, as Windows said it.
core-warning-restore-service-not-started = El servicio { $name } está instalado y no se ha iniciado: { $reason }. Puede que la aplicación no funcione hasta que se inicie. Windows lo volverá a intentar la próxima vez que se inicie este PC.
# { $name } is a service's name. { $reason } is why it could not be stopped.
core-warning-restore-service-not-stopped = No se ha podido detener primero el servicio { $name }: { $reason }. Puede que siga ejecutando el programa antiguo hasta la próxima vez que se inicie este PC.
core-warning-restore-install-cancelled = La instalación se ha cancelado.
core-warning-restore-preview-cancelled = La vista previa se ha cancelado.
# { $item } is what was left out, such as a file or a service's name. { $reason } is why, already in words.
core-warning-restore-left-out = Se ha excluido { $item }: { $reason }.
# { $error } is what went wrong.
core-warning-restore-verification-failed = No se ha podido ejecutar la verificación posterior a la restauración: { $error }
# { $runner } says what relies on the file: a service, a scheduled task, a firewall rule, a shortcut, or a registry value.
# { $path } is the file. { $name } is the name of what relies on it. { $refusal } is why the file was not replaced,
# already in words.
core-warning-restore-program-would-run =
    { $runner ->
        [service] Este paquete no debe reemplazar { $path }, y el servicio { $name } depende de ese archivo, así que no se ha instalado nada: { $refusal }
        [task] Este paquete no debe reemplazar { $path }, y la tarea programada { $name } depende de ese archivo, así que no se ha instalado nada: { $refusal }
        [firewall] Este paquete no debe reemplazar { $path }, y la regla de firewall { $name } depende de ese archivo, así que no se ha instalado nada: { $refusal }
        [shortcut] Este paquete no debe reemplazar { $path }, y el acceso directo { $name } depende de ese archivo, así que no se ha instalado nada: { $refusal }
       *[registry] Este paquete no debe reemplazar { $path }, y el valor del Registro { $name } depende de ese archivo, así que no se ha instalado nada: { $refusal }
    }
core-warning-restore-destination-exists = El destino ya existe; usa --force para sobrescribirlo.
# { $path } is a file. { $error } is what went wrong.
core-warning-restore-file-not-written = No se ha podido escribir { $path }: { $error }
core-warning-restore-not-a-folder = Donde va esta carpeta vacía hay algo que no es una carpeta.
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-not-looked-at = No se ha podido examinar { $path }: { $error }
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-folder-not-made = No se ha podido crear { $path }: { $error }
# Why part of the app was left out, after "was left out:".
core-warning-restore-registration-exists = este PC ya tiene esto, configurado de otra forma, y una instalación no se apropia de ello
core-warning-restore-registry-review = Los valores del Registro se han restaurado a partir de los datos estructurados archivados, con las rutas de acceso conocidas de la aplicación reescritas para apuntar a la raíz de destino.
# { $error } is what went wrong.
core-warning-restore-firewall-unreadable = No se han podido leer las reglas del Firewall de Windows: { $error }
# Why a firewall rule was left out, after "was left out:".
core-warning-restore-firewall-rule-exists = ya hay una regla con este nombre para este programa, configurada de otra forma, y una instalación no la cambia
# { $name } is a runtime.
core-warning-restore-prerequisite-unknown = { $name }: no hay ningún instalador conocido para este entorno de ejecución.
# { $name } is a runtime.
core-warning-restore-prerequisite-needs-admin = Instalar { $name } requiere permisos elevados; vuelve a ejecutar el instalador como administrador.
# { $name } is a runtime.
core-warning-restore-prerequisite-reboot = { $name } se ha instalado, pero es necesario reiniciar para completar la instalación.
# { $error } is what went wrong.
core-warning-restore-skipped-not-recorded = No se ha podido anotar un entorno de ejecución que se ha omitido: { $error }
# { $name } is a runtime. { $reason } is why it was skipped, already in words.
core-warning-restore-prerequisite-skipped = { $name }: { $reason }
# { $name } is a runtime. { $reason } is why it could not be installed. { $url } is a web address to get it from.
core-warning-restore-prerequisite-failed = No se ha podido instalar { $name }: { $reason }. Descárgalo de { $url }.
# { $error } is what went wrong.
core-warning-restore-runtime-not-recorded = No se ha podido anotar un entorno de ejecución instalado: { $error }
# { $error } is what went wrong.
core-warning-restore-license-source-invalid = La ruta de acceso de la licencia empaquetada no es válida: { $error }
core-warning-restore-license-needs-admin = Restaurar datos de licencia en una ubicación protegida requiere permisos elevados.
core-warning-restore-license-exists = El destino de la licencia ya existe; usa --force para sobrescribirlo.
# { $error } is what went wrong.
core-warning-restore-license-failed = No se han podido restaurar los datos de licencia: { $error }
core-warning-restore-driver-needs-admin = Registrar el controlador requiere permisos elevados; vuelve a ejecutar el instalador como administrador.
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-warning-restore-task-runs-as-system = La tarea programada { $name } se ejecuta como SYSTEM, con control total sobre este PC.
core-warning-restore-shortcut-exists = El acceso directo ya existe; usa --force para sobrescribirlo.
core-warning-restore-shortcut-all-users = Uno o varios accesos directos restaurados apuntan al escritorio público o al menú Inicio común.
core-warning-restore-environment-refresh = Puede que los cambios en las variables de entorno no se vean hasta que se inicie un nuevo proceso, un nuevo shell o una nueva sesión.
# { $name } is a service's name. { $account } is the account it ran as on the PC it came from.
core-warning-restore-service-account = El servicio { $name } se ejecutaba como { $account } en el PC de origen. Su contraseña no está en el instalador, así que el servicio se configura como LocalService. Si no funciona, establece su cuenta y su contraseña en Servicios.
# { $name } is a runtime.
core-warning-restore-prerequisite-pending-reboot = { $name } aún no está registrado; debería estar activo después de reiniciar.
# { $name } is a runtime.
core-warning-restore-prerequisite-not-verified = { $name } ha indicado que se ha completado correctamente, pero no se ha registrado; puede que la instalación haya fallado.
# { $error } is what went wrong.
core-warning-restore-driver-source-invalid = La ruta de acceso del controlador incluido no es válida: { $error }
# { $error } is what went wrong.
core-warning-restore-driver-not-read = No se ha podido leer la configuración actual del controlador: { $error }
core-warning-restore-driver-exists = El servicio del controlador ya existe; usa --force para volver a configurarlo.
core-warning-restore-driver-image-exists = La imagen del controlador ya existe; usa --force para sobrescribirla.
core-warning-restore-driver-hash-bundled = La imagen del controlador incluida no coincide con su hash registrado, así que no se ha instalado.
# { $error } is what went wrong.
core-warning-restore-driver-not-placed = No se ha podido colocar la imagen del controlador: { $error }
core-warning-restore-driver-hash-placed = La imagen del controlador ha cambiado después de colocarse, así que no se ha registrado.
# { $error } is what went wrong.
core-warning-restore-key-not-created = No se ha podido crear la clave del Registro: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-read = No se ha podido leer el valor que esto reemplazaría: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-written = No se ha podido escribir el valor del Registro: { $error }
core-warning-restore-value-exists = El valor del Registro ya existe; usa --force para sobrescribirlo.
# { $error } is what went wrong.
core-warning-restore-environment-key-not-created = No se ha podido crear la clave del Registro de las variables de entorno: { $error }
core-warning-restore-variable-exists = La variable de entorno ya existe; usa --force para sobrescribirla.
# { $error } is what went wrong.
core-warning-restore-variable-not-written = No se ha podido escribir la variable de entorno: { $error }
core-warning-restore-value-kind-missing = Falta el tipo del valor del Registro.
# { $error } is what went wrong.
core-warning-restore-service-not-read = No se ha podido leer la configuración actual del servicio: { $error }
core-warning-restore-service-exists = El servicio ya existe; usa --force para volver a configurarlo.
# { $error } is what went wrong.
core-warning-restore-task-not-queried = Error al consultar la tarea programada: { $error }
core-warning-restore-task-exists = La tarea programada ya existe; usa --force para sobrescribirla.
# { $error } is what went wrong.
core-warning-restore-task-folder-not-looked-at = No se ha podido examinar la carpeta de la tarea: { $error }
# { $error } is what went wrong.
core-warning-restore-shortcut-not-put-back = No se ha podido crear el acceso directo ni restablecer el que reemplazaba: { $error }
# { $error } is what went wrong.
core-warning-restore-command-not-started = Error al iniciar el comando: { $error }
# { $output } is what the command wrote, as it wrote it.
core-warning-restore-command-failed = Error del comando: { $output }
# { $error } is what went wrong.
core-warning-restore-powershell-not-started = Error al iniciar PowerShell: { $error }
