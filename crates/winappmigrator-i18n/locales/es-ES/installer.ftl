# Words of the installer that every built installer carries, which runs on the PC an app is moved to and speaks its
# language, and of the uninstaller it leaves behind for Settings. Message names start with installer- for the
# installer's window, installer-cli- for its command line, and uninstaller- for the uninstaller.
#
# The command lines (installer-cli-, uninstaller-cli-) are always shown in English, as scripts read what they print.
# Translate them all the same. Keep their command-line flags, such as --apply and --target-root, exactly as they are.


## The installer's window: its title and header

installer-window-title = Instalador de WinAppMigrator
# The window's title in the taskbar. { $app_name } is the name of the app it installs.
installer-window-title-app = Instalar { $app_name }
installer-tagline = Reinstala una aplicación capturada en otro PC
# What an app is called when its package gives it no name.
installer-unnamed-app = Aplicación

## When the installer's window cannot open. Each is a paragraph of a message box.

# { $error } is what went wrong, as Windows or the graphics driver says it.
installer-window-failed = No se ha podido abrir la ventana del instalador: { $error }
installer-window-failed-without-one = Aun así, se puede instalar sin ella: ejecuta este archivo desde un símbolo del sistema como administrador con --apply y, para elegir dónde, con --target-root seguido de una carpeta.

## Checking the installer when it opens

installer-checking = Comprobando el instalador…
installer-stub-no-package = Este es el programa base de un instalador de WinAppMigrator y no contiene ningún paquete de aplicación. Crea un instalador con WinAppMigrator y ejecuta aquí el .exe que se genere.
installer-check-stopped = La comprobación del instalador se ha detenido.
# { $error } is what went wrong, as Windows says it.
installer-locate-failed = No se ha podido encontrar el instalador: { $error }
# { $error } is what went wrong reading it.
installer-read-failed = No se ha podido leer el instalador: { $error }
# { $error } is why, such as that the app needs a newer Windows.
installer-cannot-be-used-here = El paquete de este instalador no se puede usar en este PC: { $error }

## The app, where it goes, and whether the installer is signed

installer-captured-from = Capturada en
installer-install-location = Ubicación de instalación
# The title of the folder picker.
installer-choose-location = Elige la ubicación de instalación
installer-folder-has-files = Esta carpeta ya contiene archivos. Los archivos con el mismo nombre se reemplazan y vuelven a su lugar si desinstalas.
installer-room-needed = Espacio necesario
# { $shortfall } says which drive is short of room, such as "C: needs up to 824.0 MB for the app's files, and has 300.0 MB free".
installer-room-short = Puede que aquí no haya espacio suficiente: { $shortfall }. Se puede desempaquetar en otra unidad, pero los archivos de la aplicación tienen que caber igualmente.
# { $app_name } is the name of an app whose earlier install was cut short.
installer-interrupted = Una instalación anterior de { $app_name } no llegó a terminar. Instalar la deshace primero.
# Beside "Installer", for whether it proves who made it.
installer-signature = Instalador
installer-signature-checking = Comprobando su firma…
installer-signature-unsigned = Sin firmar: editor desconocido
# { $publisher } is the name on the installer's certificate, such as "Fabrikam, Inc.".
installer-signature-signed-by = Firmado por { $publisher }
installer-signature-unnamed = Firmado por un editor sin nombre
# { $reason } is Windows' own word for what is wrong, such as HashMismatch.
installer-signature-invalid = La firma de este instalador no es válida ({ $reason }). Puede que se haya modificado después de firmarse.

## Why the install location cannot be used. Install and Preview Changes stay off while one of these is shown.

# { $app_name } is the name of the app being installed.
installer-location-app-already-there = Ya hay una instalación de { $app_name } en esta carpeta, así que esta copia no puede ir aquí. Para instalarla aquí, desinstala primero la copia de este PC, en Configuración de Windows, en Aplicaciones, y vuelve a abrir este instalador. O bien escribe arriba una carpeta nueva y vacía.
# { $other_app } is the name of another app Windows lists as installed in the folder. { $app_name } is the name of the app being installed.
installer-location-other-app-there = En esta carpeta está instalado { $other_app }, así que { $app_name } no puede ir aquí. Escribe arriba una carpeta nueva y vacía. O bien, si ya no necesitas { $other_app }, desinstálalo primero, en Configuración de Windows, en Aplicaciones, y vuelve a abrir este instalador.
installer-location-has-files = Esta carpeta ya contiene archivos que no instaló WinAppMigrator. Pueden pertenecer a otra aplicación, así que no se instala nada encima de ellos. Escribe arriba una carpeta nueva y vacía.
# { $reason } is why, such as that it cannot be the Windows folder.
installer-location-refused = Esta carpeta no se puede usar: { $reason }. Escribe otra carpeta arriba.

## What the install sets up beyond its files

installer-beyond-files = Qué configura esto además de los archivos
# Under the heading above. It is part of what a person agrees to when they click Install.
installer-beyond-files-agreed = Todo esto se hace con permisos de administrador. Si la instalación falla, se revierte lo que haya cambiado y se te indica lo que no se haya podido revertir. También puedes deshacer la instalación más adelante desde Configuración, en Aplicaciones instaladas. Los archivos que reemplace vuelven a su lugar, y los entornos de ejecución que instale se quedan.
installer-beyond-files-none = Nada: solo copia archivos y configuración en las claves del Registro propias de la aplicación.
# { $problem } is why, such as that the app cannot run on this PC.
installer-cannot-go-here = Este paquete no puede ir a esta ubicación: { $problem }
installer-working-out = Calculando qué configuraría esto en esta ubicación…
# Said as the reason after "This package cannot go to this location:".
installer-list-stopped = se ha detenido el cálculo de lo que haría
# A heading over a list of what else the package asked for and will not get. { $count } is how many.
installer-more-left-out =
    { $count ->
        [one] Se ha excluido { $count } elemento más
       *[other] Se han excluido { $count } elementos más
    }

## Options

installer-options = Opciones
installer-user-data = Datos de usuario
installer-user-data-detail = Activado: la configuración, Documentos y Juegos guardados van a las carpetas habituales de quien haya iniciado sesión. Desactivado: todo va dentro de la carpeta de instalación y no se configura nada para ejecutarse desde ahí.
# { $account } is the name of the account on the other PC whose data this is.
installer-user-data-whose = Contiene los datos de { $account }.
installer-license = Licencia
installer-license-detail = Archivos de activación y licencia, y los valores del Registro que contienen licencias. Puede que algunas tengan que volver a activarse aquí.

## Dependencies: what else the app needs, and how each is provided

installer-dependencies = Dependencias
installer-dependencies-detail = Qué necesita esta aplicación y cómo se proporciona cada cosa.
installer-dependencies-none = No se han detectado más dependencias de entornos de ejecución.
# { $feature } is the name of an optional part of Windows, such as NetFx3.
installer-dependency-windows-feature = Característica de Windows: { $feature }
installer-dependency-included = Incluido
# { $id } is the package's name in winget, such as Microsoft.VCRedist.2015+.x64.
installer-dependency-winget = Se instala con winget: { $id }
installer-dependency-manual = Instalar manualmente
installer-dependency-built-in = Integrado en Windows
installer-dependency-ships-with-app = Viene con la aplicación

## Buttons, and what is said beside them

installer-install = Instalar
installer-preview = Vista previa de los cambios
installer-cancel = Cancelar
installer-close = Cerrar
installer-open-folder = Abrir carpeta de instalación
# { $error } is what went wrong, as Windows says it.
installer-open-folder-failed = No se ha podido abrir la carpeta: { $error }
installer-location-blocks = Instalar y Vista previa de los cambios están desactivados hasta que se pueda usar la ubicación de instalación. El motivo aparece arriba, debajo de ella.
installer-package-blocks = Instalar está desactivado porque este paquete no puede ir a esta ubicación. El motivo aparece arriba.
installer-list-changed = Lo que esto haría aquí ha cambiado, así que no se ha instalado nada. Revisa la lista de abajo y vuelve a hacer clic en Instalar si es lo que quieres.
installer-cancelled-early = Cancelado. No se ha cambiado nada.

## How far an install has got, beside its progress bar

# { $step } is one of the steps below. { $done } and { $total } are how many of its items are done, and how many there are.
installer-step-counted = { $step }: { $done } de { $total }…
# { $step } is one of the steps below, when it does not count its items.
installer-step-uncounted = { $step }…
installer-step-getting-ready = Preparando…
installer-step-cancelling = Cancelando después del paso actual…
installer-step-putting-back = Revirtiendo lo que se ha cambiado
installer-step-unpacking = Desempaquetando
installer-step-checking-package = Comprobando el paquete
installer-step-recovering = Deshaciendo una instalación anterior que no llegó a terminar
# What a preview is doing, whatever step it is on.
installer-step-previewing = Calculando qué cambiaría
installer-step-runtimes = Instalando entornos de ejecución
installer-step-files = Copiando archivos
installer-step-services = Configurando servicios
installer-step-drivers = Configurando controladores
installer-step-registry = Escribiendo la configuración del Registro
installer-step-license = Restaurando los datos de licencia
installer-step-tasks = Configurando tareas programadas
installer-step-shortcuts = Creando accesos directos
installer-step-environment = Estableciendo variables de entorno
installer-step-settings-entry = Agregando la aplicación a Configuración
installer-step-verifying = Comprobando lo que se ha instalado
installer-step-undoing = Deshaciendo

## When an install cannot go ahead or stops

installer-carries-no-package = Este instalador no contiene ningún paquete.
installer-stopped-unexpectedly = La instalación se ha detenido de forma inesperada.
# { $reason } is what went wrong, as the program says it.
installer-stopped-unexpectedly-because = La instalación se ha detenido de forma inesperada: { $reason }. Si ya había empezado, al volver a ejecutar un instalador primero se deshace lo que hizo.
# { $error } is what went wrong reading it.
installer-package-read-failed = No se ha podido leer el paquete: { $error }
installer-no-programdata = No se ha encontrado la carpeta ProgramData de este PC, así que no se ha instalado nada: una instalación guarda ahí el historial que permite deshacerla.
# { $error } is why, such as that there is nowhere only administrators can get into.
installer-staging-failed = No se ha podido crear una carpeta de preparación: { $error }
# { $error } is what went wrong unpacking it.
installer-unpack-failed = No se ha podido desempaquetar el paquete: { $error }
installer-staging-nowhere = no hay ningún lugar donde desempaquetar el paquete al que solo puedan acceder los administradores
# { $failure } is a folder and what went wrong there, such as "C:\ProgramData\WinAppMigrator: Access is denied.".
installer-staging-nowhere-look-at = no hay ningún lugar donde desempaquetar el paquete al que solo puedan acceder los administradores. El que hay que revisar es { $failure }

## What came of an install, or of a preview

installer-result = Resultado de la instalación
installer-result-preview = Vista previa: todavía no se ha cambiado nada
installer-result-installed = La aplicación se ha instalado y ha superado la comprobación de hash.
installer-result-installed-without-left-out = La aplicación se ha instalado y ha superado la comprobación de hash, sin lo que aparece abajo como excluido.
installer-result-not-all-put-back = La instalación ha fallado y no se ha podido revertir todo lo que cambió. Lo que queda aparece abajo, y la próxima instalación lo volverá a intentar.
installer-result-cancelled-put-back = La instalación se ha cancelado y se ha revertido lo que cambió.
installer-result-failed-put-back = La instalación ha fallado y se ha revertido lo que cambió.
installer-result-not-clean = La instalación no ha terminado correctamente. Revisa los problemas de abajo.
installer-result-changes-put-back = Cambios revertidos
installer-result-changes-made = Cambios realizados
installer-result-changes-planned = Cambios previstos
installer-result-warnings = Advertencias
installer-result-errors = Errores
installer-result-package-valid = Hashes del paquete válidos
installer-result-package-invalid = Hashes del paquete NO VÁLIDOS
installer-result-files-valid = Archivos instalados válidos
installer-result-files-invalid = Archivos instalados NO VÁLIDOS
installer-result-reboot = Una dependencia necesita un reinicio para terminar. Reinicia Windows y, después, abre la aplicación.
# A heading over the list of what was changed. { $count } is how many.
installer-result-changes = Cambios ({ $count })
# After the first part of a long list. { $count } is how many more there are.
installer-result-changes-more = y { $count } más.
# Over the path of the install's log file.
installer-log = Archivo de registro

## The installer's command line. Shown in English: scripts read it. A { $count } is plain digits, with no separators.

installer-cli-stub = Programa base del instalador de WinAppMigrator: no contiene ningún paquete de aplicación.
installer-cli-stub-how = Crea un instalador con la aplicación WinAppMigrator y ejecuta el .exe generado en el equipo de destino.
installer-cli-usage-title = Instalador autoextraíble de WinAppMigrator.
installer-cli-usage = Ejecútalo sin argumentos para abrir el instalador gráfico, o bien:
installer-cli-usage-preview = vista previa de la reinstalación: --dry-run [--target-root <DIR>]
installer-cli-usage-apply = reinstalar:                       --apply [--target-root <DIR>]
installer-cli-no-package = el instalador no contiene un paquete incrustado
installer-cli-no-programdata = no se ha encontrado la carpeta ProgramData de este PC, así que no se ha instalado nada: una instalación guarda ahí el historial que permite deshacerla
# { $path } is the folder asked for, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-folder-has-files = { $path } no se puede usar: { $problem }. Indica --force solo si sabes que es de esta aplicación
installer-cli-no-location = este paquete no indica dónde estaba su aplicación, así que indica un --target-root
# { $path } is where the app was on the other PC, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-location-refused = la aplicación de este paquete estaba en { $path }, y esa ubicación no se puede usar: { $problem }. Indica un --target-root
# { $summary } is how much room it needs where, such as "up to 824.0 MB on C: (356.0 GB free)".
installer-cli-room = Espacio: { $summary }
# { $note } is something to know about the install.
installer-cli-note = Nota: { $note }
# { $count } is how many things it sets up beyond its files.
installer-cli-sets-up = Configura: { $count }
installer-cli-applied = Reinstalación aplicada.
installer-cli-previewed = Vista previa de la reinstalación completada.
# { $count } is how many changes the install logged.
installer-cli-changes-logged = Cambios registrados: { $count }
# { $count } is how many warnings.
installer-cli-warnings = Advertencias: { $count }
# { $count } is how many errors.
installer-cli-errors = Errores: { $count }
# { $count } is how many things the package asked for and did not get.
installer-cli-refused = Rechazados: { $count }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-package-valid = Hashes del paquete válidos: { $valid }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-reinstall-valid = Hashes de la reinstalación válidos: { $valid }
# { $id } is the install's id, such as wam-0f3a….
installer-cli-install-id = Id. de la instalación: { $id }
installer-cli-put-back = La instalación ha fallado, así que se ha revertido lo que cambió.
# { $path } is the log file.
installer-cli-log = Archivo de registro: { $path }
# { $path } is the log file. Printed before an error.
installer-cli-error-log = archivo de registro: { $path }
# { $error } is what went wrong.
installer-cli-error = error: { $error }

## The uninstaller's window

uninstaller-window-title = Desinstalar
# The window's title, and its heading. { $app_name } is the name of the app it uninstalls.
uninstaller-window-title-app = Desinstalar { $app_name }
uninstaller-tagline = Deshace lo que hizo su instalador
uninstaller-reading = Leyendo el historial de esta instalación…
uninstaller-stopped-unexpectedly = La desinstalación se ha detenido de forma inesperada.
uninstaller-what-will-be-removed = Qué se quitará
uninstaller-install-location = Ubicación de instalación
uninstaller-files = Archivos
uninstaller-registry-values = Valores del Registro
uninstaller-services = Servicios
uninstaller-drivers = Controladores
uninstaller-scheduled-tasks = Tareas programadas
uninstaller-environment = Entorno
uninstaller-plan-stays = Los archivos que hayas agregado o cambiado se quedan, junto con las carpetas que los contienen.
# { $count } is how many files the install replaced.
uninstaller-plan-coming-back =
    { $count ->
        [one] { $count } archivo que reemplazó esta instalación vuelve a su lugar o se coloca junto a un archivo que ha cambiado desde entonces.
       *[other] { $count } archivos que reemplazó esta instalación vuelven a su lugar o se colocan junto a un archivo que ha cambiado desde entonces.
    }
# { $runtimes } is a list of runtimes, such as "Microsoft Visual C++ Redistributable, .NET Desktop Runtime 8".
uninstaller-plan-runtimes = Los entornos de ejecución que instaló se quedan, porque otros programas pueden usarlos: { $runtimes }.
uninstaller-options = Opciones
uninstaller-remove-user-data = Quitar también los datos de usuario y los datos de licencia
# { $count } is how many items of the app's settings, data, and license there are.
uninstaller-remove-user-data-detail =
    { $count ->
        [one] { $count } elemento de su configuración, sus datos y su licencia. Se queda a menos que esto esté activado. Lo que reemplazó vuelve a su lugar o, si ha cambiado desde entonces, se coloca junto a él.
       *[other] { $count } elementos de su configuración, sus datos y su licencia. Se quedan a menos que esto esté activado. Lo que reemplazaron vuelve a su lugar o, si ha cambiado desde entonces, se coloca junto a ellos.
    }
uninstaller-uninstall = Desinstalar
uninstaller-cancel = Cancelar
uninstaller-close = Cerrar
# { $done } and { $total } are how many of the install's items are undone, and how many there are.
uninstaller-step-counted = Deshaciendo la instalación: { $done } de { $total }…
uninstaller-step = Deshaciendo la instalación…
uninstaller-done = La aplicación se ha desinstalado.
uninstaller-done-except = La aplicación se ha desinstalado, excepto lo que aparece abajo.
uninstaller-waits-for-drive = Una parte está en una unidad que no está conectada. Conéctala y vuelve a desinstalar desde Configuración.
uninstaller-not-all-undone = No se ha podido deshacer todo. Cierra lo que lo esté usando y vuelve a desinstalar desde Configuración.
# Over the path of the uninstall's log file.
uninstaller-log = Archivo de registro
uninstaller-undone = Deshecho
uninstaller-left-in-place = Se queda en su lugar
uninstaller-could-not-be-undone = No se ha podido deshacer
# After the first part of a long list. { $count } is how many more there are.
uninstaller-and-more = y { $count } más.
# Said as the reason when there is no record of the install to read.
uninstaller-no-programdata = no se ha encontrado la carpeta ProgramData de este PC, así que no hay ningún historial que leer

## When the uninstaller's window cannot open. Each is a paragraph of a message box, followed by a command to type.

uninstaller-window-failed-title = Desinstalador de WinAppMigrator
# { $error } is what went wrong, as Windows or the graphics driver says it.
uninstaller-window-failed = No se ha podido abrir la ventana del desinstalador: { $error }
uninstaller-window-failed-quiet = Aun así, se puede desinstalar sin ella, desde un símbolo del sistema como administrador:
# Stands in for this program's own path in that command, if it cannot be found.
uninstaller-this-program = este programa

## The uninstaller's command line (--quiet). Shown in English: scripts read it.

# { $app_name } is the name of the app.
uninstaller-cli-uninstalling = Desinstalando { $app_name }.
# { $path } is the log file.
uninstaller-cli-log = Archivo de registro: { $path }
# { $error } is what went wrong.
uninstaller-cli-error = error: { $error }
