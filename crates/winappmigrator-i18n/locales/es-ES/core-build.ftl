# Words the core writes for people while it builds a package from an installed app: what a package records for an
# installer to show later, errors shown in the app, the list of what goes in and what is left out, opening a disk
# image or another PC over the network, and what an installer says when the package it carries cannot be read.
# Message names start with core-.


## Scanning an app, before a package is built. Shown in the app when a scan cannot go on.

# { $path } is the app's program, such as D:\Program Files\Acme\Acme.exe, on a drive taken from another PC.
core-scan-program-not-on-the-drive = { $path } pasa por un vínculo o no está en la unidad conectada, así que no se ha leído.
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-not-on-this-pc = { $path } no está en un disco de este PC, o tiene un nombre que abriría otra cosa, así que no se ha leído.
# { $path } is the app's program.
core-scan-program-someone-elses = { $path } está en las carpetas de otra cuenta o no es de esta cuenta, y la propia información de instalación de esta aplicación no indica que deba estar ahí, así que no se ha leído.
# { $app_name } is the app's name, as the list of apps calls it.
core-scan-nothing-to-package = No se ha encontrado ningún programa propio de { $app_name }, así que no hay nada que poner en un instalador. Suele ocurrir con los controladores, los entornos de ejecución y los complementos: en el PC nuevo, instala el controlador desde su fabricante o desde Windows Update, y el entorno de ejecución o el complemento junto con la aplicación que lo usa.
# { $app_name } is the app's name, as the list of apps calls it. A web app is a website installed as an app, with
# a window and a Start menu entry of its own.
core-scan-web-app = { $app_name } es una aplicación web que un navegador instaló desde un sitio web. No tiene un programa propio, así que no hay nada que poner en un instalador. En el PC nuevo, abre el sitio web en ese navegador e instala la aplicación desde allí.
# { $path } is the program the app's own record names, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-missing = { $path } ya no está ahí, aunque la información de instalación de esta aplicación lo indica como su programa. Puede que la aplicación se haya movido o quitado en parte: repararla o reinstalarla en este PC debería devolverlo a su sitio.
# { $path } is the file that was picked.
core-scan-not-a-file = el destino del análisis no es un archivo: { $path }
# Names something left out of a package, in a list of what was left out and why. { $name } is the variable's name,
# such as ACME_LICENSE_KEY.
core-scan-environment-variable-item = La variable de entorno { $name }

## Folders and parts that might also be the app's, offered in a list to tick. Each says why it is offered.

core-near-starts-with-app-name = Su nombre empieza por el nombre de esta aplicación.
core-near-has-app-name = Su nombre contiene el nombre de esta aplicación.
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-starts-with-publisher = Su nombre empieza por el nombre del editor, { $publisher }.
# { $folder } is a folder's name, such as Program Files.
core-near-hidden-part-beside = Una parte oculta en la lista de aplicaciones, instalada junto a esta aplicación en { $folder }.
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-hidden-part-publisher = Una parte oculta en la lista de aplicaciones, del mismo editor, { $publisher }.

## What a scan found, as a person chooses from it: the folders that go in, and what is left out, and why

# Why a folder cannot be left out, beside its number of files.
core-footprint-locked-app-folder = Siempre van con el programa principal
core-footprint-locked-main-program = Contiene el programa principal, que siempre se incluye
core-footprint-locked-needed = Contiene lo que el programa principal necesita para iniciarse
core-footprint-locked-font = Va con su fuente, en Qué la integra en Windows
# Where a folder is. "AppData" and "ProgramData" are the names of Windows folders; keep them as they are.
core-footprint-place-roaming = Configuración en tu perfil (AppData)
core-footprint-place-local = Configuración de este PC en tu perfil (AppData)
core-footprint-place-program-data = Compartido por todos los usuarios del PC (ProgramData)
core-footprint-place-documents = En tu carpeta Documentos
core-footprint-place-saved-games = En tu carpeta Juegos guardados
core-footprint-place-public-documents = En Documentos públicos
core-footprint-place-app-folder = En la carpeta de la aplicación
# { $folder } is a folder, such as C:\Users\Sam\AppData\Roaming.
core-footprint-place-in = En { $folder }
# A row for the files directly in the app's folder. { $folder } is that folder's name, such as Acme.
core-footprint-files-directly-in = Archivos situados directamente en { $folder }
# Headings over what is left out, and why each item is.
core-footprint-licenses-left-out = No se incluyen en este instalador porque la opción Datos de licencia y activación está desactivada
core-footprint-font-stays-behind = Se queda en el PC de origen.
core-footprint-drivers-left-out = Controladores que pertenecen a Windows o a un fabricante, que no se pueden redistribuir; reinstálalos en el otro PC
core-footprint-driver-reinstall = Reinstálalo desde el paquete del propio fabricante.
# Why something else is left out along with what the person left out. { $service } is a service's name, such as AcmeSync.
core-footprint-needs-left-out = necesita { $service }, que está excluido
core-footprint-told-to-use-left-out = está configurado para usar algo de una carpeta que has excluido
core-footprint-file-in-left-out = su archivo está en una carpeta que has excluido
core-footprint-program-in-left-out = el programa al que corresponde está en una carpeta que has excluido
core-footprint-names-left-out = nombra una carpeta que has excluido
# Why something is left out, as the installer tells whoever installs it.
core-footprint-builder-left-it-out = quien creó este instalador lo excluyó
# Something the person left out, by kind and name, such as "Service Acme Updater". { $kind } is the kind, such as
# Service or Scheduled Task; { $title } is its name.
core-footprint-left-out-item = { $kind } { $title }

## Building a package and an installer from a scan. Shown in the app when a build cannot go on.

core-package-scan-of-another-app = este análisis es de una aplicación distinta de la que se está creando, así que no se ha creado nada
core-package-scan-brought-in-more = este análisis ha traído datos que no se pidieron para esta creación, así que no se ha creado nada
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-package-program-gone = { $path } ya no está, así que no se ha creado nada. Vuelve a analizar la aplicación.
core-package-needs-stub = un archivo autoextraíble requiere el programa base de un instalador
# { $reason } is what went wrong.
core-package-staged-file-failed = un archivo de la carpeta de preparación no se ha podido incluir en el paquete: { $reason }
# { $shortfalls } says which drives are short of room and by how much, in sentences of its own.
core-package-not-enough-room = no hay espacio suficiente para crear este instalador, así que no se ha leído ni escrito nada. { $shortfalls }. Libera espacio ahí o elige una carpeta de salida en otra unidad.
# { $path } is a file's path, on a drive taken from another PC.
core-package-not-read-off-the-drive = { $path } pasa por un vínculo o no está en la unidad conectada, así que no se ha leído
# { $path } is a file's path.
core-package-not-read-not-local = { $path } no está en un disco de este PC, así que no se ha leído
# { $path } is a file's path.
core-package-not-read-other-account = { $path } lleva a las carpetas de otra cuenta, así que no se ha leído
# { $path } is a file's path, as the PC it came from called it.
core-package-not-on-the-drive = { $path } no está en la unidad conectada, así que no se ha leído
# { $path } is a file's path.
core-package-backup-copy-no-parent = no se puede hacer una copia de seguridad de un archivo sin directorio principal: { $path }
# { $path } is a file's path.
core-package-backup-copy-destination-no-parent = el destino de la copia de seguridad no tiene directorio principal: { $path }
# robocopy is the name of a Windows command; keep it as it is. { $code } is how it ended, such as Some(8).
core-package-robocopy-failed = error en la copia de robocopy en modo de copia de seguridad (código de salida { $code })
# Stands for a folder where a file's name would be, as in "I/O failed for temporary package folder: …".
core-package-temporary-folder = carpeta temporal del paquete
# Stands for the installer being written where a file's name would be, as in "I/O failed for the installer: …".
core-package-the-installer = el instalador
# { $path } is a file's path, or its name in the package. { $error } is what went wrong with it.
core-package-path-error = { $path }: { $error }
# { $name } is a file's name in the package, such as app/bin/App.exe.
core-package-same-place = otro archivo ocupa el mismo lugar en el paquete: { $name }
# { $name } is a file's name in the package.
core-package-changed-in-protected-folder = { $name } ha cambiado en la carpeta protegida mientras se creaba el paquete
core-package-worker-died = un subproceso que comprimía archivos se ha detenido de forma inesperada, así que no se ha escrito ningún instalador
# { $path } is a file's path.
core-package-link-not-read = { $path } es un vínculo, así que no se ha leído
# { $path } is a file's path. { $error } is what went wrong.
core-package-not-compressed = { $path } no se ha podido comprimir: { $error }
# { $path } is a file's path. { $reason } is what went wrong.
core-package-not-read-back = { $path } no se ha podido volver a leer: { $reason }

## Where an app's files are, and where an installer may write them. Shown when a path is refused.

# { $path } is a folder, such as C:\Program Files\Acme.
core-discovery-root-not-full-path = la carpeta en la que se escribe debe ser una ruta de acceso completa: { $path }
# { $path } is a folder's path.
core-discovery-root-not-a-folder = la carpeta en la que se escribe no es una carpeta: { $path }
# { $path } is a file's path.
core-discovery-escapes-root = la ruta de acceso sale de la raíz prevista: { $path }
# { $path } is the link's path.
core-discovery-refusing-to-write-through-link = no se escribe a través de un vínculo: { $path }
# { $path } is the link's path.
core-discovery-refusing-link = no se pasa por un vínculo: { $path }
# { $error } is what Windows said went wrong.
core-discovery-refusing-link-because = no se pasa por un vínculo: { $error }
# { $path } is the link's path.
core-discovery-link-to-somewhere-else = { $path } es un vínculo a otro lugar, y otros podrían cambiar adónde lleva. Usa el lugar al que lleva
# { $path } is a folder's path.
core-discovery-no-parent-folder = { $path } no tiene carpeta principal
# { $path } is the install location as it was given. Keep the quotes, or use your language's own.
core-discovery-install-location-not-full-path = la ubicación de instalación debe ser una ruta de acceso completa: "{ $path }"
# { $path } is the install location.
core-discovery-install-location-a-link = la ubicación de instalación es un vínculo a otro lugar: { $path }
# { $path } is a file's path.
core-discovery-device-name = la ruta de acceso usa un nombre reservado para un dispositivo de Windows: { $path }
# { $path } is a file's path.
core-discovery-device-name-on-older-windows = { $path } tiene un nombre que esta versión de Windows abre como dispositivo y no como archivo, así que no se ha escrito nada en él. Se puede instalar en Windows 11
# { $path } is a file's path.
core-discovery-unsafe-path = ruta de acceso no segura: { $path }
# { $path } is a file's path.
core-discovery-no-package-path = el archivo no tiene ruta de acceso relativa en el paquete: { $path }

## Writing an installer, and reading the package inside one. Most of these are shown by an installer, on the PC it
## runs on, when it cannot read what it carries. "The package" is the app's files and settings that an installer
## carries; "the stub" is the program part of an installer, before the package; "the data file" is the .wamdata file
## that holds the package of a very large installer, beside it.

# Stands for the package where a file's name would be, as in "I/O failed for installer package: …".
core-archive-installer-package = paquete del instalador
# Stands for the installer program built into WinAppMigrator, where a file's name would be.
core-archive-embedded-stub = programa base del instalador incrustado
# { $index } is the number of a file in the package. { $reason } is what went wrong.
core-archive-entry-unreadable = no se ha podido leer la entrada { $index } del paquete: { $reason }
# { $name } is a file's name inside the package, such as app/bin/App.exe.
core-archive-entry-unsafe-path = la entrada del zip tiene una ruta de acceso no segura: { $name }
# { $name } is a file's name inside the package.
core-archive-holds-more = { $name } contiene más de lo que indica
# { $name } is a file's name inside the package.
core-archive-name-too-long = este nombre es demasiado largo para un zip: { $name }
core-archive-changed-while-copied = el paquete ha cambiado mientras se colocaba detrás del programa base, así que no se ha escrito ningún instalador
core-archive-stub-not-a-program = la parte de este instalador anterior a su paquete no es un programa
core-archive-damaged = este instalador está dañado o se ha modificado después de crearse, así que no se ha leído nada de su paquete. Vuelve a copiarlo desde donde se creó.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-missing = este instalador guarda su paquete en un archivo llamado { $name }, en la misma carpeta, y ese archivo no está. Los dos archivos tienen que ir juntos: copia aquí { $name } desde donde se creó el instalador y vuelve a ejecutar esto.
# { $name } is the data file's name, such as App_installer.wamdata. { $error } is what Windows said went wrong.
core-archive-data-file-unopened = el archivo de datos de este instalador, { $name }, no se ha podido abrir: { $error }. Los dos archivos tienen que ir juntos, en una carpeta que este PC pueda leer.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-a-link = { $name } es un vínculo o una carpeta, y el archivo de datos de este instalador tiene que ser un archivo independiente, justo a su lado, así que no se ha leído nada de él. Copia aquí el archivo real.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-wrong = { $name } no es el archivo de datos de este instalador, o está dañado o se ha modificado después de crearse, así que no se ha leído nada de él. Vuelve a copiar los dos archivos desde donde se crearon.
core-archive-stub-signed = el programa base del instalador está firmado; debe quedar sin firmar, así que firma en su lugar el instalador terminado
# { $size } is a size, such as 4.2 GB.
core-archive-too-big-to-run = el instalador ocuparía { $size }, y Windows no puede ejecutar un ejecutable de 4 GB o más; no se ha escrito nada
core-archive-outside-signature = el instalador tiene datos fuera de su firma y puede que se haya manipulado
core-archive-footer-length = la longitud del pie del archivo autoextraíble no es válida
core-archive-seek-outside = posicionamiento fuera del paquete
# { $path } is a file's path.
core-archive-unsafe-package-path = ruta de acceso del paquete no segura: { $path }
# { $path } is a file's path.
core-archive-empty-package-path = ruta de acceso del paquete vacía: { $path }

## Opening a disk image of another PC (a .vhd or .vhdx file) to read apps from it

# { $path } is the disk image file, such as D:\Backups\OldPC.vhdx.
core-image-not-attachable = { $path } no es una imagen de disco que Windows pueda montar (.vhd o .vhdx)
core-image-needs-administrator = montar una imagen de disco requiere permisos de administrador
# { $path } is the disk image file the person picked.
core-image-not-a-file = { $path } no es un archivo
# { $path } is the disk image file. "Edit Disk" is the name of a Hyper-V Manager command.
core-image-differencing = { $path } es un disco de diferenciación, que solo contiene lo que ha cambiado desde el disco a partir del que se creó, y Windows abriría también ese disco, esté donde esté. Combínalo primero con ese disco, con Editar disco de Hyper-V, y elige ese disco
# { $path } is the disk image file.
core-image-unreadable = { $path } no es un archivo .vhd o .vhdx que se pueda leer
# ProgramData is the name of a Windows folder; keep it as it is.
core-image-no-program-data = no se ha encontrado la carpeta ProgramData de este PC
# The reason given in core-image-could-not-attach when Windows already has the image open.
core-image-already-attached = Windows ya la tiene montada, y lo que la haya montado podría estar escribiendo en ella. Desmóntala primero o elige su unidad como unidad conectada
# { $path } is the disk image file. { $reason } is why, such as core-image-already-attached or what Windows said.
core-image-could-not-attach = { $path } no se ha podido montar: { $reason }
# { $path } is the disk image file.
core-image-no-windows-volume = { $path } no tiene ningún volumen con Windows que se pueda leer. Un volumen cifrado con BitLocker se tiene que desbloquear primero.
# Given as the reason when Windows failed without saying anything.
core-image-windows-did-not-say = Windows no ha indicado el motivo

## Reading another PC over the network, through the client running on it. Shown as why something on that PC
## could not be read. "The client" is WinAppMigrator Client, the small program on the other PC.

core-remote-too-slow = el otro PC ha tardado demasiado en responder
core-remote-closed = la conexión con el otro PC está cerrada
core-remote-failed = la conexión con el otro PC ha fallado
core-remote-answered-something-else = el cliente ha respondido otra cosa
# { $name } is the PC's name or address as the person typed it, such as OLDPC or 192.168.1.20.
core-remote-not-found = no se ha encontrado { $name } en la red
core-remote-did-not-say-what-it-is = el cliente no ha indicado qué es
core-remote-ordinary-names-only = solo se pregunta por rutas de acceso dentro de la raíz del otro PC, con nombres normales
core-remote-more-but-none = el otro PC ha indicado que una carpeta contenía más elementos, pero no ha enviado ninguno
# { $count } is the most things one folder may hold, such as 250,000.
core-remote-too-many-listed = el otro PC ha enumerado más de { $count } elementos en una carpeta
core-remote-sent-too-much = el otro PC ha enviado más datos de un archivo de los que se pidieron
core-remote-name-not-text = no se puede preguntar por un nombre que no es texto
core-remote-below-root-only = solo se puede preguntar por rutas de acceso dentro de la raíz del otro PC
core-remote-file-ended-early = el archivo del otro PC ha terminado antes de tiempo
core-remote-link-not-followed = no se siguen los vínculos de otro PC
core-remote-only-files-opened = en otro PC solo se abren archivos
core-remote-listing-said-nothing = la lista no decía nada de ello
core-remote-change-time-unknown = no se sabe cuándo se modificó
core-remote-seek-before-start = un posicionamiento antes del inicio
# { $size } is how many bytes a file is; { $limit } is the most that were expected. Both are numbers.
core-remote-too-large =
    { $size ->
        [one] ocupa { $size } byte, y no se esperaban más de { $limit }
       *[other] ocupa { $size } bytes, y no se esperaban más de { $limit }
    }

## Checking that a download is signed by Microsoft. Each is given as the reason a check failed.

# { $error } is what Windows said went wrong.
core-signature-no-powershell = no se ha podido iniciar PowerShell: { $error }
# { $error } is what went wrong reading PowerShell's answer.
core-signature-unreadable = no se ha podido leer la firma: { $error }
# { $signer } is who signed the file, such as Contoso Ltd.
core-signature-not-microsoft = está firmado por { $signer }, no por Microsoft
core-signature-not-microsoft-unnamed = está firmado por otro, no por Microsoft
# { $status } is Windows' own word for the signature's state, such as HashMismatch or NotSigned, which is not translated.
core-signature-status = el estado de su firma es { $status }

## Warnings a package records when it is built from an app, which an installer shows later, maybe on another PC, in
## that PC's language. "This app" is the app being packaged.

core-warning-build-program-needs-unreadable = No se ha podido leer lo que esto necesita de Windows, así que no se comprobará si puede ejecutarse en el otro PC.
core-warning-build-external-manifest = Se ha detectado un manifiesto de aplicación externo; la interpretación de las dependencias side-by-side se aplaza.
core-warning-build-non-exe = El archivo seleccionado no usa la extensión .exe; el análisis PE continuará.
core-warning-build-stand-in-program = No se ha encontrado ningún ejecutable normal de la aplicación, así que este archivo ocupa su lugar. Los resultados de arquitectura y dependencias describen este archivo, y puede que no describan la aplicación.
core-warning-build-other-apps-files-unknown = No se ha podido leer todo lo que Windows Installer tiene de otras aplicaciones, así que una carpeta que esta aplicación tiene registrada podría contener archivos de otra aplicación sin que nada aquí lo indique.
core-warning-build-app-folder-has-another-apps-file = Windows Installer indica que otra aplicación tiene un archivo en la carpeta en la que está el programa de esta aplicación, y ese archivo se incluye, ya que se toma toda esa carpeta.
# "App Paths" is the name of a Windows registry key; keep it as it is.
core-warning-build-app-paths-folder-left-out = La entrada App Paths de esta aplicación indica que busca en una carpeta que no es suya, así que esa carpeta no se incluye. Puede que no se inicie en un PC sin lo que hay ahí.
core-warning-build-uninstaller-folder-left-out = Se ha excluido la carpeta en la que está el desinstalador de esta aplicación, porque Windows Installer indica que otra aplicación tiene un archivo ahí.
# { $why } is never shown: it chooses the reason. Translate each reason as the end of the sentence.
core-warning-build-recorded-folder-left-out =
    Algo registrado para esta aplicación nombra una carpeta que se ha excluido, porque { $why ->
        [unreadable] su ruta de acceso sale de la unidad o pasa por un vínculo, así que no se puede leer de forma segura
        [missing] no existe
        [link] pasa por un vínculo
        [short-name] no está escrita con su nombre real
        [another-account] está en las carpetas de otra cuenta
        [one-account] esta aplicación se instaló para una cuenta, y esto no es de esa cuenta
        [shared] contiene elementos compartidos o forma parte de Windows
        [generic-name] su nombre no dice nada de a quién pertenece
        [publisher] solo lleva el nombre del editor, y puede contener sus otros productos
        [holds-the-app] contiene la propia carpeta de esta aplicación, y puede contener más
        [another-apps-name] lleva el nombre de otra aplicación
        [another-apps-record] su información de instalación es de otra aplicación
        [in-another-apps-folder] está en la carpeta de otra aplicación
       *[another-apps] es de otra aplicación
    }.
core-warning-build-app-folder-holds-another-app = Otra aplicación de este PC está registrada dentro de la carpeta de esta aplicación, y se incluye con ella, ya que se toma toda esa carpeta.
# "Program Files (x86)" is the name of a Windows folder; keep it as Windows calls it.
core-warning-build-from-32-bit-windows = Esta aplicación estaba en un Windows de 32 bits. Sus claves del Registro se organizan como Windows de 64 bits guarda las de una aplicación de 32 bits, y el instalador ofrece Program Files (x86) para ella.
# { $dll } is a file's name, such as MSVCP140D.dll.
core-warning-build-debug-runtime = Un programa necesita { $dll }, una compilación de depuración del entorno de ejecución de Visual C++. Microsoft no tiene ningún instalador para ellas y puede que no permita redistribuirlas, así que no se ejecutará en un PC sin Visual Studio.
# { $program } is a program's file name, such as Tool.exe.
core-warning-build-single-file-program = { $program } es un programa .NET empaquetado en un solo archivo, que no indica qué .NET necesita, así que no se prevé ninguno para él. Si necesita uno, no se iniciará en un PC que no lo tenga.
# { $count } is how many of the app's programs.
core-warning-build-programs-not-inspected =
    { $count ->
        [one] No se ha examinado { $count } de los programas de la aplicación para ver qué entornos de ejecución necesita, porque tiene demasiados o son demasiado grandes.
       *[other] No se han examinado { $count } de los programas de la aplicación para ver qué entornos de ejecución necesitan, porque tiene demasiados o son demasiado grandes.
    }
# { $runtime } is a runtime's name, such as Java Runtime Environment.
core-warning-build-runtime-no-installer = La aplicación necesita { $runtime }, y aquí no hay ningún instalador para él, así que no se ejecuta ninguno. Instálalo tú mismo en el otro PC.
# { $why } is why something was left out, such as "its name says what it holds".
core-warning-build-left-out-of-package = Excluido del paquete: { $why }.
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-missing-dependency = No se ha podido resolver la dependencia: { $name }
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-dependency-only-here = { $name } solo se ha encontrado mediante la ruta de búsqueda de este PC y no forma parte de la aplicación, así que no se incluye. Puede que la aplicación no se inicie en un PC sin él.
core-warning-build-driver-detected = Se ha detectado un controlador o un componente del kernel. Un controlador propio de la aplicación se incluye y se vuelve a registrar; un controlador de un fabricante o integrado en Windows no se redistribuye y se debe reinstalar desde el paquete firmado del fabricante.
core-warning-build-near-no-longer-offered = Algo que se marcó en la lista de lo que también podría ser de esta aplicación ya no está en esa lista, así que no se ha agregado. Vuelve a revisar la lista.
# { $app_name } is the name of a part of the app, as the list of apps calls it. { $why } is never shown: it chooses the
# reason. Translate each reason as the end of the sentence.
core-warning-build-near-part-left-out =
    Se ha excluido una parte de lo que { $app_name } indica que es suyo: { $why ->
        [unreadable] su ruta de acceso sale de la unidad o pasa por un vínculo, así que no se puede leer de forma segura
        [missing] no existe
        [link] pasa por un vínculo
        [short-name] no está escrita con su nombre real
        [another-account] está en las carpetas de otra cuenta
        [one-account] esta aplicación se instaló para una cuenta, y esto no es de esa cuenta
        [shared] contiene elementos compartidos o forma parte de Windows
        [generic-name] su nombre no dice nada de a quién pertenece
        [publisher] solo lleva el nombre del editor, y puede contener sus otros productos
        [holds-the-app] contiene la propia carpeta de esta aplicación, y puede contener más
        [another-apps-name] lleva el nombre de otra aplicación
        [another-apps-record] su información de instalación es de otra aplicación
        [in-another-apps-folder] está en la carpeta de otra aplicación
       *[another-apps] es de otra aplicación
    }.
# { $app_name } is the name of a part of the app, as the list of apps calls it.
core-warning-build-near-keys-left-out = Se han excluido las claves del Registro que enumera la información de instalación de { $app_name }, ya que esa información también nombra elementos de otra aplicación.
# "Windows keeps what an app wrote for one account" is Windows' VirtualStore folder.
core-warning-build-virtual-store-program = Había un programa en la carpeta donde Windows guarda lo que una aplicación escribió para una cuenta. Windows nunca pone ninguno ahí, así que lo puso otra cosa, y se ha excluido.
# { $path } is a file or folder, such as C:\Users\Sam\AppData\Roaming\Acme.
core-warning-build-file-left-out = Se ha excluido { $path }: pasa por un vínculo en una unidad conectada, o pertenece a una cuenta distinta de aquella cuyos datos se incluyen.
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as D:\Sam\Documents.
core-warning-build-user-folder-not-on-drive = La carpeta { $folder } de { $user } estaba en { $path }, que no está en esta unidad, así que no se ha incluido lo que hay ahí.
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as \\server\home\Sam\Documents.
core-warning-build-user-folder-not-on-this-pc = La carpeta { $folder } de { $user } estaba en { $path }, que no está en un disco de este PC, así que no se ha incluido lo que hay ahí.
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-base-unreadable = No se ha podido leer { $path } ({ $error }); no se han incluido los datos de la aplicación guardados ahí.
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-folder-unreadable = No se ha podido leer { $path } ({ $error }); puede que falten datos de la aplicación guardados en subcarpetas.
# { $error } is what went wrong.
core-warning-build-entry-unreadable = No se ha podido leer una entrada del sistema de archivos: { $error }
# { $count } is how many links. { $names } lists some of them. { $more } is how many more there are, which are not
# named; when it is 0, the sentence ends after the list.
core-warning-build-links-not-followed =
    { $count ->
        [one] No se ha seguido { $count } vínculo, o nombre que abriría otra cosa, en esta carpeta, así que aquello a lo que apunta no está en el instalador: { $names }
       *[other] No se han seguido { $count } vínculos, o nombres que abrirían otra cosa, en esta carpeta, así que aquello a lo que apuntan no está en el instalador: { $names }
    }{ $more ->
        [0] .
       *[other] { " " }y { $more } más.
    }
# { $count } is how many folders. { $depth } is how many levels down a folder can be. { $names } lists some of them.
# { $more } is how many more there are, which are not named; when it is 0, the sentence ends after the list.
core-warning-build-folders-too-deep =
    { $count ->
        [one] { $count } carpeta está a más de { $depth } niveles de profundidad, así que su contenido no está en el instalador: { $names }
       *[other] { $count } carpetas están a más de { $depth } niveles de profundidad, así que su contenido no está en el instalador: { $names }
    }{ $more ->
        [0] .
       *[other] { " " }y { $more } más.
    }
# { $error } is what went wrong.
core-warning-build-metadata-unreadable = No se han podido leer los metadatos del archivo: { $error }
core-warning-build-reserved-name = El nombre de este archivo está reservado para un dispositivo de Windows, así que no se ha copiado.
# { $error } is what went wrong.
core-warning-build-hash-failed = No se ha podido calcular el hash del archivo: { $error }
core-warning-build-cloud-placeholder = Archivo en la nube solo en línea (por ejemplo, Archivos a petición de OneDrive) sin datos locales; ánclalo o descárgalo antes de la copia de seguridad. Se indica, pero no se copia.
# { $file_name } is a driver's file name, such as acmeflt.sys.
core-warning-build-driver-file = El archivo de controlador o del kernel "{ $file_name }" no se copia como un archivo normal. Si respalda un servicio de controlador registrado para esta aplicación, el instalador lo incluye y vuelve a registrar el controlador automáticamente; si no, reinstálalo con el paquete de controlador firmado del fabricante.
core-warning-build-file-gone = Esto estaba ahí cuando se analizó la aplicación y ya no está, así que no está en el instalador.
# { $error } is what went wrong.
core-warning-build-file-copy-failed = No se ha podido copiar el archivo en el paquete: { $error }
# { $count } is how many files. { $names } lists them.
core-warning-build-not-looked-through =
    { $count ->
        [one] Este paquete tiene un archivo en el que no se ha podido buscar de quién procede: { $names }. Puede que su contenido lo revele.
       *[other] Este paquete tiene archivos en los que no se ha podido buscar de quién proceden: { $names }. Puede que su contenido lo revele.
    }
# { $count } is the most empty folders an installer makes, such as 10,000.
core-warning-build-empty-folders-truncated = Esta aplicación tiene más de { $count } carpetas vacías, y solo se volverá a crear esa cantidad.
core-warning-build-credential-by-now = Excluido del paquete: ahora parece una credencial.
core-warning-build-no-package-path = Un archivo que se puede copiar no tenía ruta de acceso relativa en el paquete.
# { $file_name } is the runtime installer's file name, such as vc_redist.x64.exe. { $error } is what went wrong.
core-warning-build-prerequisite-path-invalid = No se ha podido colocar el requisito previo incluido { $file_name }: { $error }
# { $runtime } is a runtime's name, such as Microsoft Visual C++ 2015+ Redistributable (x64). { $error } is what went
# wrong. winget is the name of a Windows command; keep it as it is.
core-warning-build-prerequisite-download-failed = No se ha podido descargar { $runtime } ({ $error }); el instalador probará con winget en el equipo de destino.
# { $runtime } is a runtime's name. { $reason } is why the check failed, such as "it is signed by Contoso Ltd, not
# Microsoft".
core-warning-build-prerequisite-signature-failed = La descarga de { $runtime } no ha superado la comprobación de firma, así que no se ha incluido: { $reason }. El instalador probará con winget en el equipo de destino.
# { $runtime } is a runtime's name. { $error } is what went wrong.
core-warning-build-prerequisite-hash-failed = Se ha descargado { $runtime }, pero no se ha podido calcular su hash ({ $error }); el instalador probará con winget en el equipo de destino.
# { $service } is a driver's service name, such as AcmeFlt. { $error } is what went wrong.
core-warning-build-driver-copy-failed = No se ha podido incluir la imagen del controlador de "{ $service }": { $error }
core-warning-build-license-not-copied = No se han podido copiar los datos de licencia (faltan o no se pueden leer); solo se han registrado como metadatos.
core-warning-build-license-file-credential = Un archivo de licencia parece una credencial y se ha excluido.
# { $error } is what went wrong.
core-warning-build-license-store-walk-failed = No se ha podido enumerar parte de un almacén de licencias: { $error }
core-warning-build-license-store-credential = Un archivo de un almacén de licencias parece una credencial y se ha excluido.
# { $error } is what went wrong.
core-warning-build-license-store-file-skipped = Se ha omitido un archivo de un almacén de licencias: { $error }
