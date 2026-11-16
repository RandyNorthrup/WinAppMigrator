# Words the core says about what an install may do: why an install location or something in a package is refused,
# the installer's list of what an app sets up beyond its files, the checks of a package and of an install, and whether
# there is room for either. Message names start with core-.
#
# Many messages here are a reason, the second half of a sentence that starts "... was left out:" or "that install
# location cannot be used:", so they start with a small letter and have no full stop.


## Why an install location cannot be used. Shown in red under the Install Location field.

core-review-location-state-root = es donde las instalaciones guardan su historial
core-review-location-has-files = ya contiene archivos y no hay ninguna instalación registrada aquí, así que usa una carpeta vacía
core-policy-location-not-full-path = debe ser una ruta de acceso completa
core-policy-location-parent-step = no debe contener ".."
core-policy-location-not-plain = tiene que ser una ruta de acceso normal en una unidad de este PC
core-policy-location-whole-drive = no puede ser una unidad entera
core-policy-location-windows = no puede ser la carpeta de Windows
core-policy-location-runs-by-itself = es una carpeta que pertenece a Windows, o cuyo contenido se ejecuta o se carga por sí solo
core-policy-location-shared = contiene otros elementos uno junto a otro, así que la aplicación necesita una carpeta dentro de ella

## Whether an app can run on this PC at all, said before anything is installed.

# { $reason } is why, as the second half of a sentence, such as "App.exe is built for Arm64, and this PC is x64, so
# Windows would not start it".
core-review-cannot-run-here = esta aplicación no puede ejecutarse en este PC, así que no se ha cambiado nada: { $reason }
core-policy-architecture-x86 = x86 de 32 bits
core-policy-architecture-x64 = x64
core-policy-architecture-arm64 = Arm64
core-policy-architecture-arm32 = Arm de 32 bits
core-policy-architecture-unknown = una arquitectura desconocida
# { $code } is the number Windows gives a kind of processor, such as 0x01c4.
core-policy-machine-type = tipo de máquina { $code }
# { $names } is a list of program names, such as "a.exe, b.exe, c.exe", and { $count } how many more there are.
core-policy-names-and-more = { $names } y { $count } más
core-policy-fit-programs-not-said = Este instalador no indica qué necesitan sus programas, así que no se ha comprobado si pueden ejecutarse en este PC.
core-policy-fit-architecture-not-told = No se ha podido saber qué arquitectura tiene este PC, así que no se ha comprobado si los programas de esta aplicación pueden ejecutarse en él.
core-policy-fit-main-not-said = Este instalador no indica qué necesita su programa principal, así que lo que se indica aquí se basa en el resto de lo que contiene.
# { $driver } is a driver's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-driver = su controlador { $driver } está compilado para { $built_for }, y este PC es { $here }. Un controlador tiene que coincidir exactamente con Windows, así que esta aplicación no puede ejecutarse aquí
# { $program } is a program's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-program = { $program } está compilado para { $built_for }, y este PC es { $here }, así que Windows no lo iniciaría
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-none-start = ninguno de sus programas puede iniciarse en este PC, que es { $here }: { $programs }
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-some-other-architecture = Algunos de los programas de esta aplicación están compilados para otra arquitectura y no se iniciarán en este PC, que es { $here }: { $programs }. El resto de la aplicación sí puede, así que puede que esto no importe.
# { $here } is a kind of processor, such as "Arm64". { $programs } is a list of program names.
core-policy-fit-emulated = Este PC es { $here }, así que Windows ejecutaría estos programas mediante un emulador: { $programs }. La mayoría de los programas funcionan bien así, pero algunos no.
# { $program } is a program's file name. { $needs } and { $runs } are versions of Windows, such as 10.0 and 6.1.
core-policy-fit-needs-newer = { $program } indica que necesita Windows { $needs } o posterior, y este PC ejecuta { $runs }, así que Windows no lo iniciaría
# { $programs } is a list of program names.
core-policy-fit-some-need-newer = Algunos de los programas de esta aplicación indican que necesitan una versión de Windows más reciente que esta y no se iniciarán aquí: { $programs }.
# { $from_build } and { $this_build } are Windows build numbers, such as 22631, written as Windows writes them.
core-policy-fit-older-build = Esta aplicación estaba en la compilación { $from_build } de Windows, y este PC ejecuta la compilación { $this_build }. Sus programas indican que pueden iniciarse en este Windows, así que puede que funcione, pero puede que aquí no esté lo que necesitaba allí.
core-policy-fit-build-not-told = No se ha podido saber qué compilación de Windows es esta, así que no se ha comparado con la que tenía esta aplicación.

## Room on a drive for a build or an install.

# Each of these ends the sentence core-space-shortfall, after "free for".
core-space-for-unpacking = desempaquetar el paquete
core-space-for-install-record = el historial de la instalación
core-space-for-app-files = los archivos de la aplicación
core-space-for-what-is-replaced = guardar lo que reemplaza
core-space-for-build-work = los datos con los que trabaja una creación
core-space-for-installer = el instalador
core-space-for-package = el paquete
core-space-for-unpacking-and-installing = desempaquetar el paquete e instalarlo
# What goes between two of those, as in "unpacking the package and the app's files". Keep the quotes: they keep the
# spaces.
core-space-purposes-joiner = { " y " }
# { $drive } is a drive, such as C:, or a folder. { $needed } and { $free } are sizes, such as 1.2 GB. { $purposes } is
# what the room is for, such as "unpacking the package and the app's files".
core-space-shortfall = { $drive } necesita hasta { $needed } libres para { $purposes }, y tiene { $free }
# { $drive } is a drive, such as C:, or a folder. { $needed } is a size, such as 1.2 GB.
core-space-not-told = { $drive } no indica cuánto espacio tiene, así que no se ha comprobado si caben los { $needed } que necesita
# One drive in a list of where room is needed. { $needed } and { $free } are sizes, and { $drive } a drive, such as C:.
core-space-summary = hasta { $needed } en { $drive } ({ $free } libres)
# { $needed } is a size, and { $drive } a drive, such as C:.
core-space-summary-not-told = hasta { $needed } en { $drive } (no se ha podido saber cuánto espacio queda libre)
# What goes between two sentences of core-space-shortfall inside core-review-no-room. Keep the quotes: they keep the
# space.
core-review-sentence-joiner = { ". " }
# { $shortfalls } is one or more sentences of core-space-shortfall.
core-review-no-room = no hay espacio suficiente para esto, así que no se ha cambiado nada. { $shortfalls }. Libera espacio ahí o elige una ubicación de instalación en otra unidad.

## The installer's list of what an app sets up beyond its files: what each kind is called.

core-review-kind-service = Servicio
core-review-kind-driver = Controlador
core-review-kind-scheduled-task = Tarea programada
core-review-kind-autostart = Se inicia automáticamente
core-review-kind-shortcut = Acceso directo
core-review-kind-path = PATH
core-review-kind-environment-variable = Variable de entorno
core-review-kind-shell-extension = Extensión del shell
core-review-kind-menu-command = Comando de menú
core-review-kind-link-type = Tipo de vínculo
core-review-kind-file-type = Tipo de archivo
core-review-kind-com = COM
core-review-kind-firewall-rule = Regla de firewall
core-review-kind-event-log-source = Origen del registro de eventos
core-review-kind-font = Fuente
core-review-kind-odbc-driver = Controlador ODBC
core-review-kind-odbc-source = Origen de datos ODBC
core-review-kind-compatibility-setting = Configuración de compatibilidad
core-review-kind-default-apps = Aplicaciones predeterminadas
core-review-kind-program-name = Nombre de programa
core-review-kind-other-settings = Otra configuración

## The installer's list of what an app sets up: what each line says of it.

# { $file } is a program's file name. { $account } is the Windows account it runs as, such as LocalSystem. { $starts }
# says when it starts, and is not shown itself; for "other", { $start } is one of the core-review-start- words below.
core-review-service-detail =
    { $starts ->
        [automatic] Ejecuta { $file } como { $account }, se inicia ahora y cada vez que se inicia este PC
        [delayed] Ejecuta { $file } como { $account }, se inicia ahora y poco después de cada inicio de este PC
        [manual] Ejecuta { $file } como { $account }, solo se inicia cuando algo lo solicita
        [disabled] Ejecuta { $file } como { $account }, está deshabilitado
        [other] Ejecuta { $file } como { $account }, { $start }
       *[unknown] Ejecuta { $file } como { $account }
    }
# { $account } is a Windows account, such as LocalService.
core-policy-account-custom = { $account } (cuenta personalizada no restaurada)
# { $file } is a driver's file name.
core-review-driver-detail = Carga { $file } en el propio Windows
# { $file } is a driver's file name. { $start } is when Windows starts it: one of the core-review-start- words below.
core-review-driver-detail-starts = Carga { $file } en el propio Windows, { $start }
# When a service or driver starts, ending a sentence such as "Loads disk.sys into Windows itself, starts …".
core-review-start-boot = se inicia con Windows, antes que todo lo demás
core-review-start-system = se inicia con Windows, durante el arranque
core-review-start-automatic = se inicia cada vez que arranca este PC
core-review-start-delayed = se inicia poco después de cada arranque de este PC
core-review-start-manual = solo se inicia cuando algo lo solicita
core-review-start-disabled = no se inicia nunca, porque está deshabilitado
core-review-start-unknown = se inicia de una forma que Windows no indica
# { $account } is the account a task runs as: an account's name, or core-review-task-runs-as-you, or
# core-review-task-runs-as-creator.
core-review-task-detail = Se ejecuta como { $account }
# { $account } is the account a task runs as, as for core-review-task-detail.
core-review-task-detail-highest = Se ejecuta como { $account }, con los privilegios más altos que pueda tener
core-review-task-runs-as-you = tú, quien haya iniciado sesión
core-review-task-runs-as-creator = el usuario que la creó
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-autostart-detail =
    Se ejecuta { $scope ->
        [everyone] para todos los que inicien sesión
       *[you] cuando inicias sesión
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shortcut-startup-detail =
    Un acceso directo en la carpeta Inicio, { $scope ->
        [everyone] para todos los usuarios de este PC
       *[you] para ti
    }
# { $file } is the file name of what the shortcut opens. { $scope } is whom it is set up for: everyone on this PC,
# or you. It is not shown itself.
core-review-shortcut-detail =
    Abre { $file }, { $scope ->
        [everyone] para todos los usuarios de este PC
       *[you] para ti
    }
# { $name } is an environment variable's name, such as PATH.
core-review-path-title = Se agrega a { $name }
core-review-path-install-folder = La carpeta de instalación
# { $path } is a folder inside the install folder, such as bin\tools.
core-review-path-below-install-folder = La carpeta de instalación\{ $path }
# { $value } is the variable's value, as it will be set.
core-review-variable-detail = Se establece en { $value }
# { $count } is how many registry values.
core-review-other-settings-title =
    { $count ->
        [one] { $count } valor de configuración fuera de las claves propias de la aplicación
       *[other] { $count } valores de configuración fuera de las claves propias de la aplicación
    }
# { $keys } is a list of registry keys, such as HKCU\Software\Vendor.
core-review-other-settings-detail = En { $keys }
# { $keys } is a list of registry keys, and { $count } how many more there are.
core-review-other-settings-detail-more = En { $keys } y { $count } más
# { $way } says whether the rule allows or blocks, and incoming or outgoing connections; it is not shown itself.
# { $ports } is a list of port numbers, and { $port_count } how many. { $program } is the program's file name.
# { $networks } is a list of the kinds of network Windows names in English (domain, private, public), and
# { $network_count } how many.
# The kinds of network a firewall rule is for, as Windows Defender Firewall names its profiles in your language. They
# fill { $networks } in core-review-firewall-detail, joined into a list.
core-review-network-domain = Dominio
core-review-network-private = Privado
core-review-network-public = Público
core-review-firewall-detail =
    { $way ->
        [allow-in] Permite las conexiones entrantes
        [allow-out] Permite las conexiones salientes
        [block-in] Bloquea las conexiones entrantes
       *[block-out] Bloquea las conexiones salientes
    }{ $port_count ->
        [0] { "" }
        [one] { " " }en el puerto { $ports }
       *[other] { " " }en los puertos { $ports }
    } para { $program }, en { $network_count ->
        [0] todas las redes
        [one] el perfil { $networks }
       *[other] los perfiles { $networks }
    }
# { $server } is the file name of what Explorer loads. { $scope } is whom it is set up for: everyone on this PC, or
# you. It is not shown itself.
core-review-shell-extension-detail =
    Carga { $server } en el Explorador de archivos, { $scope ->
        [everyone] para todos los usuarios de este PC
       *[you] para ti
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shell-extension-detail-own-file =
    Carga en el Explorador de archivos un archivo que instala, { $scope ->
        [everyone] para todos los usuarios de este PC
       *[you] para ti
    }
# The name of a shell extension: { $handler } is its name in the registry, and { $class } the kind of file or folder it
# is for, such as * or Directory.
core-policy-shell-extension-name = { $handler } en { $class }
# { $on } is the kind of file or folder whose menu it is, as the registry names it, such as * or
# Directory. { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-menu-command-detail =
    Agrega un comando al menú de { $on }, { $scope ->
        [everyone] para todos los usuarios de este PC
       *[you] para ti
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-default-verb-detail =
    Cambia lo que ocurre al abrir uno de estos, { $scope ->
        [everyone] para todos los usuarios de este PC
       *[you] para ti
    }
core-review-client-entry-detail = Se puede elegir como programa para esto. No se cambia nada para que se use
core-review-com-permissions-title = Permisos de COM
core-review-com-permissions-detail = Indica quién puede iniciar sus clases COM, o con qué cuenta se ejecutan
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-link-type-detail =
    Los vínculos de este tipo se abren con la aplicación, { $scope ->
        [everyone] para todos los usuarios de este PC
       *[you] para ti
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-file-type-detail =
    Los archivos de este tipo se abren con la aplicación, { $scope ->
        [everyone] para todos los usuarios de este PC
       *[you] para ti
    }
# { $log } is the name of a Windows event log, such as Application.
core-review-event-log-source-detail = Puede escribir en el registro { $log }, con mensajes de sus propios archivos
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-font-detail =
    Se carga desde la carpeta de instalación al iniciar sesión, { $scope ->
        [everyone] para todos los usuarios de este PC
       *[you] para ti
    }
core-review-odbc-driver-detail = Se carga en los programas que usan ODBC para acceder a una base de datos
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-odbc-source-detail =
    Una conexión a una base de datos, { $scope ->
        [everyone] para todos los usuarios de este PC
       *[you] para ti
    }
core-review-default-apps-detail = Se puede elegir en Aplicaciones predeterminadas. No se cambia nada para que se use
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-program-name-detail =
    Ejecutar y otros programas pueden encontrarlo por este nombre, { $scope ->
        [everyone] para todos los usuarios de este PC
       *[you] para ti
    }
# { $server } is the file name of what serves the classes.
core-review-com-title = COM en { $server }
core-review-com-title-own-file = COM en un archivo que instala
# { $count } is how many registry values.
core-review-com-detail =
    { $count ->
        [one] { $count } valor de configuración de las clases que pueden iniciar otros programas
       *[other] { $count } valores de configuración de las clases que pueden iniciar otros programas
    }
core-review-com-support-title = Bibliotecas de tipos, interfaces y AppID de COM
# { $libraries }, { $interfaces }, and { $app_ids } are how many registry values there are for its type libraries, its
# interfaces, and its AppIDs.
core-review-com-support-detail = Valores de configuración de sus bibliotecas de tipos ({ $libraries }), interfaces ({ $interfaces }) y AppID ({ $app_ids }), que acompañan a sus clases
core-review-compatibility-title = Cómo se inician sus programas
# { $count } is how many registry values.
core-review-compatibility-detail =
    { $count ->
        [one] { $count } valor de configuración de compatibilidad para los programas que instala
       *[other] { $count } valores de configuración de compatibilidad para los programas que instala
    }

## The installer's notes on whose folders and settings these are, and what is not coming.

# { $account } is the name of a Windows account, such as alice.
core-review-account-signed-in = Los datos de usuario y la configuración por usuario van a { $account }, que ha iniciado sesión aquí, y no al administrador con el que se ejecuta esto.
# { $account } is the name of a Windows account, such as alice.
core-review-account-no-desktop = No hay ningún usuario conectado a esta sesión, así que los datos de usuario y la configuración por usuario van a { $account }, la cuenta con la que se ejecuta esto.
# { $reason } is what Windows said, in its own words.
core-review-account-not-told = No se ha podido saber quién ha iniciado sesión ({ $reason }), así que se excluye la configuración por usuario, y los datos de usuario van dentro de la ubicación de instalación.
# { $reason } is what Windows said, in its own words.
core-review-signed-in-unknown = no se ha podido saber quién ha iniciado sesión: { $reason }
# Headings of what is left out, shared by the app's list and the installer's notes.
core-review-left-out-credentials = No se incluyen en este instalador porque parecen credenciales
core-review-left-out-settings = Configuración e historial que una cuenta guarda para la aplicación, que solo se incluyen con la opción Datos de usuario
core-review-left-out-certificates = Parece que la aplicación depende de certificados que eran de confianza en su PC, y no se instalan
core-review-left-out-fonts = Estas fuentes llevan el nombre de la aplicación y nada indica que sean suyas, así que no se instalan
# { $names } is a list of names, such as the names of saved passwords.
core-review-note-credentials = No se incluyen en este instalador porque parecen credenciales: { $names }. Lo que haya de ellas en este PC se queda como está, y puede que tengas que volver a escribirlas.
# { $names } is a list of names of license data.
core-review-note-licenses = No se incluyen en este instalador porque la opción Datos de licencia y activación estaba desactivada cuando se creó: { $names }.
# { $names } is a list of places an account keeps settings, such as folders.
core-review-note-settings = No se incluyen en este instalador porque la opción Datos de usuario estaba desactivada cuando se creó: la configuración y el historial que una cuenta guarda para la aplicación, en { $names }. La aplicación se inicia aquí como lo haría para alguien que la usa por primera vez.
# { $names } is a list of certificates, each as core-review-certificate.
core-review-note-certificates = Parece que la aplicación depende de certificados que eran de confianza en su PC, y no se instalan: { $names }. Tú decides si confiar en ellos aquí.
# { $names } is a list of font names.
core-review-note-fonts = Estas fuentes llevan el nombre de la aplicación y nada indica que sean suyas, así que no se instalan: { $names }.
# { $names } is a list of names.
core-review-note-left-out-by-choice = Excluido por quien creó este instalador: { $names }.
# { $names } is a list of names.
core-review-note-added-by-choice = Agregado por quien creó este instalador, porque lo consideró parte de la aplicación: { $names }.
# { $name } is a certificate's subject, such as CN=Contoso, and { $store } the certificate store it was in, such as
# Root.
core-review-certificate = { $name } en { $store }
# { $names } is a list of names, and { $count } how many more there are.
core-review-names-and-more = { $names } y { $count } más

## Why something in a package is left out. Each is the second half of a sentence of core-warning-review-left-out.

core-review-no-account = pertenece a la configuración de una cuenta, y no se ha podido saber quién ha iniciado sesión
core-review-name-not-accepted = Windows no aceptaría su nombre
# { $error } is what Windows said, in its own words.
core-review-service-unreadable = no se ha podido leer su configuración actual: { $error }
core-review-service-dependency = depende de un servicio con un nombre no válido
# { $path } is the program the service runs.
core-review-service-runs-other = ejecuta { $path }, que este paquete no instala
core-review-service-settings = su configuración nombra algo que este paquete no instala, o una clave que no es suya
core-review-driver-no-hash = no tiene ningún hash con el que comprobarlo
core-review-license-for-the-machine = una licencia guardada para todo el equipo solo se indica
core-review-license-starts-something = una licencia no tiene por qué iniciar nada
core-review-task-not-captured = no se capturó su definición
# { $error } is what was wrong with the task's XML, in English.
core-review-task-unreadable = no se ha podido leer su definición: { $error }
core-review-task-no-account = se ejecuta como quien la instale, y no se ha podido saber quién ha iniciado sesión aquí
# { $path } is what the shortcut opens.
core-review-shortcut-opens-other = abre { $path }, que este paquete no instala
# { $service } is the service's name, and { $path } the program it runs now.
core-policy-service-exists = { $service } ya existe en este PC y ejecuta { $path }, que no procede de una instalación anterior aquí. Si es una copia antigua de esta aplicación, desinstálala primero
core-policy-key-control-characters = el nombre de su clave contiene caracteres de control
core-policy-value-control-characters = el nombre de su valor contiene caracteres de control
core-policy-registry-link = es un vínculo del Registro
# { $reason } is why the key it stands for is left out, such as "no app install should change this key".
core-policy-virtual-store = representa una clave del PC, y { $reason }
core-policy-registry-part = ninguna instalación de una aplicación debería escribir en esta parte del Registro
core-policy-key-refused = ninguna instalación de una aplicación debería cambiar esta clave
core-policy-net-debugger = nombraría un depurador para todos los programas .NET
core-policy-net-install-root = cambiaría el lugar desde el que todos los programas .NET cargan .NET
core-policy-class-redirects = cambiaría cómo se abre un tipo de programa o de carpeta
core-policy-app-path-name = nombra un programa con algo que no es un nombre de archivo
core-policy-app-path-not-installed = registraría un nombre de programa para un archivo que este paquete no instala
core-policy-shared-default-verb = cambiaría lo que ocurre al abrir cualquier archivo o carpeta de un tipo
core-policy-command-runs-other = agrega un comando que ejecuta algo que este paquete no instala
core-policy-autostart-not-command = se inicia automáticamente, y su valor no es un comando
core-policy-autostart-runs-other = se inicia automáticamente y ejecuta algo que este paquete no instala
core-policy-environment-own-rules = las variables de entorno se restauran según sus propias reglas
core-policy-service-not-restored = pertenece a un servicio que este paquete no restaura
core-policy-service-own-rules = la configuración propia de un servicio solo se restaura mediante sus reglas
core-policy-service-dll = su ServiceDll no es un archivo que instale este paquete
core-policy-event-log-source-only = solo se pueden restaurar los valores propios de un origen del registro de eventos
core-policy-security-log = ninguna aplicación escribe en el registro de seguridad
core-policy-event-log-messages = sus mensajes procederían de un archivo que este paquete no instala
core-policy-type-library = su biblioteca de tipos no es un archivo que instale este paquete
# { $name } is a program's file name, such as app.exe.
core-policy-application-opened = cambiaría cómo se abre { $name }, que este paquete no instala
core-policy-event-log-no-file = un origen del registro de eventos tiene que obtener sus mensajes de un archivo
core-policy-com-runs-other = ejecuta algo que no es un archivo que instale este paquete
core-policy-com-redirect = convertiría esta clase en algo que este paquete no instala
core-policy-com-server = ejecuta algo que no es ni un archivo que instale este paquete ni parte de Windows
core-policy-interface = decidiría cómo se hacen las llamadas a una interfaz, con algo que este paquete no instala
# { $name } is a program's file name, such as app.exe.
core-policy-app-id-started = cambiaría cómo se inicia { $name }, que este paquete no instala
core-policy-app-id-surrogate = lo hospedaría un programa que este paquete no instala
core-policy-app-id-service = se ejecutaría en un servicio que este paquete no restaura
core-policy-app-id-run-as = se ejecutaría con una cuenta propia
core-policy-app-id-link = no nombra ningún AppID
core-policy-app-id-remote = se ejecutaría en otro lugar que no es este PC
core-policy-odbc-own-driver = es una configuración propia de ODBC y no un controlador
core-policy-odbc-driver-not-installed = su controlador no es un archivo que instale este paquete
core-policy-loads-other-file = cargaría un archivo que este paquete no instala
core-policy-odbc-own-source = es una configuración propia de ODBC y no un origen de datos
core-policy-odbc-source-missing = enumera un origen de datos que este paquete no tiene
core-policy-odbc-source-driver = su controlador no es ni un archivo que instale este paquete ni parte de Windows
core-policy-shell-extension = cargaría en el Explorador de archivos una extensión del shell que no es un archivo que instale este paquete
core-policy-not-text = su contenido no es el texto que se leería
core-policy-network-place = nombra un lugar de la red
core-policy-font = registraría una fuente que este paquete no instala, o una de la carpeta de Windows
core-policy-compatibility = cambiaría cómo se ejecuta un programa que este paquete no instala
core-policy-capabilities = enumera una aplicación cuyos detalles no están en este paquete
core-policy-default-program = cambiaría qué programa abre las cosas de forma predeterminada
core-policy-url-protocol = indica que es un tipo de vínculo, y no se abre con nada que instale este paquete
core-policy-command-handler = agrega un comando que gestiona algo que este paquete no instala
core-policy-installer-command = ejecutaría algo que solo Windows Installer puede leer
core-policy-creates-class = crearía una clase que este paquete no instala
core-policy-opens-links = abre vínculos con algo que este paquete no instala
core-policy-opens-files = abre archivos con algo que este paquete no instala
core-policy-firewall-name = su nombre no es uno que se pueda transferir
core-policy-firewall-description = su descripción no es una que se pueda transferir
core-policy-firewall-every-program = se aplica a todos los programas, y una regla solo puede ser para uno
# { $path } is the program the rule is for.
core-policy-firewall-program = es para { $path }, que este paquete no instala
core-policy-firewall-direction = no indica una dirección que se pueda establecer
core-policy-firewall-action = invalidaría otras reglas, algo que no debería hacer la regla de ninguna aplicación
# { $setting } is a part of a firewall rule, as the scan of its PC described it.
core-policy-firewall-not-carried = indica { $setting }, que no se podía establecer tal cual o que debilitaría lo que hacen otras reglas
core-policy-firewall-service = es para un servicio que este paquete no restaura
core-policy-firewall-parts = una parte no es algo que pueda indicar una regla de firewall
core-policy-folder-already-there = esa carpeta ya existía, fuera de la ubicación de instalación, y nada indica que sea de esta aplicación, así que no se coloca ningún programa en ella
core-policy-variable-name = su nombre no es un nombre de variable válido
core-policy-variable-relied-on = Windows y otros programas dependen de esta variable
core-policy-task-name = su nombre no es uno que acepte Windows
core-policy-task-folder = su carpeta no es una que acepte Windows
core-policy-task-windows = reemplazaría una de las tareas propias de Windows
core-policy-task-other-action = hace algo distinto de ejecutar un programa
core-policy-task-no-program = no nombra ningún programa que ejecutar
# { $path } is the program the task runs.
core-policy-task-runs-other = ejecuta algo que este paquete no instala: { $path }
core-policy-task-exists-unreadable = ya existe una tarea con este nombre y no se ha podido leer
core-policy-task-exists-other = ya existe una tarea con este nombre y no ejecuta la aplicación
# { $path } is the program the task already on this PC runs.
core-policy-task-exists-runs = ya existe una tarea con este nombre y ejecuta { $path }, que no procede de una instalación anterior aquí
core-policy-shortcut-not-lnk = no es un acceso directo .lnk
core-policy-shortcut-place = no está en un menú Inicio ni en un escritorio
core-policy-short-name = su ruta de acceso usa un nombre que no se puede comprobar
core-policy-shortcut-no-folder = este PC no tiene una carpeta así para él
core-policy-data-startup = iría en una carpeta Inicio y se ejecutaría en cada inicio de sesión
core-policy-data-powershell = iría donde PowerShell ejecuta lo que encuentra al iniciarse
core-policy-data-windows = iría en una carpeta que pertenece a Windows
core-policy-license-outside = está fuera de la ubicación de instalación y de todas las carpetas de datos conocidas
core-rewrite-cdata = contiene una sección CDATA
core-rewrite-too-much-to-look-through = este paquete contiene demasiado en su configuración para comprobar lo que nombra

## Warnings a review gives, which the installer shows with what it sets up.

# { $kind } says what was left out, and is not shown itself. { $item } is its path or name, or a registry value such
# as HKLM\Software\App\Name. { $reason } is why, as the second half of the sentence, such as "it cannot be a whole
# drive".
core-warning-review-left-out =
    { $kind ->
        [file] Se ha excluido el archivo { $item }: { $reason }.
        [folder] Se ha excluido la carpeta { $item }: { $reason }.
        [service] Se ha excluido el servicio { $item }: { $reason }.
        [driver] Se ha excluido el controlador { $item }: { $reason }.
        [firewall-rule] Se ha excluido la regla de firewall { $item }: { $reason }.
        [license-data] Se han excluido los datos de licencia de { $item }: { $reason }.
        [license-value] Se ha excluido el valor de licencia { $item }: { $reason }.
        [scheduled-task] Se ha excluido la tarea programada { $item }: { $reason }.
        [shortcut] Se ha excluido el acceso directo { $item }: { $reason }.
        [environment-variable] Se ha excluido la variable de entorno { $item }: { $reason }.
       *[value] Se ha excluido { $item }: { $reason }.
    }
core-warning-review-package-path-missing = Un archivo que se puede copiar no tiene ruta de acceso relativa en el paquete.
# { $count } is the most empty folders a package makes.
core-warning-review-too-many-empty-folders = Este paquete enumera más de { $count } carpetas vacías, y solo se crea esa cantidad.
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Documents folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-documents-elsewhere =
    { $count ->
        [one] { $count } archivo de Documentos va en su lugar dentro de { $target }, en una carpeta llamada _external, porque la carpeta Documentos de este PC está en { $folder }, y solo se escribe en una carpeta que esté dentro de tu perfil. Para usarlo, muévelo tú mismo a tu carpeta Documentos.
       *[other] { $count } archivos de Documentos van en su lugar dentro de { $target }, en una carpeta llamada _external, porque la carpeta Documentos de este PC está en { $folder }, y solo se escribe en una carpeta que esté dentro de tu perfil. Para usarlos, muévelos tú mismo a tu carpeta Documentos.
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-documents-unknown =
    { $count ->
        [one] { $count } archivo de Documentos va en su lugar dentro de { $target }, en una carpeta llamada _external, porque no se ha podido saber dónde está la carpeta Documentos de este PC. Para usarlo, muévelo tú mismo a tu carpeta Documentos.
       *[other] { $count } archivos de Documentos van en su lugar dentro de { $target }, en una carpeta llamada _external, porque no se ha podido saber dónde está la carpeta Documentos de este PC. Para usarlos, muévelos tú mismo a tu carpeta Documentos.
    }
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Saved Games folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-elsewhere =
    { $count ->
        [one] { $count } archivo de Juegos guardados va en su lugar dentro de { $target }, en una carpeta llamada _external, porque la carpeta Juegos guardados de este PC está en { $folder }, y solo se escribe en una carpeta que esté dentro de tu perfil. Para usarlo, muévelo tú mismo a tu carpeta Juegos guardados.
       *[other] { $count } archivos de Juegos guardados van en su lugar dentro de { $target }, en una carpeta llamada _external, porque la carpeta Juegos guardados de este PC está en { $folder }, y solo se escribe en una carpeta que esté dentro de tu perfil. Para usarlos, muévelos tú mismo a tu carpeta Juegos guardados.
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-unknown =
    { $count ->
        [one] { $count } archivo de Juegos guardados va en su lugar dentro de { $target }, en una carpeta llamada _external, porque no se ha podido saber dónde está la carpeta Juegos guardados de este PC. Para usarlo, muévelo tú mismo a tu carpeta Juegos guardados.
       *[other] { $count } archivos de Juegos guardados van en su lugar dentro de { $target }, en una carpeta llamada _external, porque no se ha podido saber dónde está la carpeta Juegos guardados de este PC. Para usarlos, muévelos tú mismo a tu carpeta Juegos guardados.
    }

## Errors when a folder that only administrators may change cannot be made so, or checked.

# { $path } is a folder.
core-privilege-not-a-folder = { $path } es un vínculo o un archivo, no una carpeta
# { $path } is a folder. { $error } is what Windows said, in its own words.
core-privilege-not-made-here = La carpeta { $path } no la ha creado WinAppMigrator, y otros pueden cambiar su contenido. Un administrador debería revisarla y quitarla: { $error }
# { $path } is a folder.
core-privilege-others-put-something-in = La carpeta { $path } contiene algo que no ha creado WinAppMigrator, y otros pueden cambiarlo. Un administrador debería revisarla y quitarla
# { $path } is a folder.
core-privilege-became-a-link-in-use = { $path } se ha convertido en un vínculo mientras se usaba
# { $path } is a folder.
core-privilege-became-a-link-looked-at = { $path } se ha convertido en un vínculo mientras se examinaba
# { $path } is a file. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-users-cannot-start = no se ha podido permitir que los usuarios inicien { $path }: { $said }
core-privilege-elevation-unknown = no se ha podido saber si este proceso tiene permisos elevados
# { $path } is a folder. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-cannot-restrict = no se ha podido restringir { $path } a los administradores: { $said }
# { $path } is a folder. { $said } is the folder's permissions as Windows wrote them, and what else it printed.
core-privilege-not-restricted = { $path } no se ha podido restringir a los administradores, y puede que su unidad no conserve los permisos: { $said }
# { $path } is a folder.
core-privilege-written-to-first = otro proceso ha escrito en { $path } antes de que se pudiera proteger
core-privilege-no-program-data = Windows no indica dónde está ProgramData

## Checks of a package's files, and of what an install put in place. Each is said of one file, value, service, or the
## like, which the report names beside it.

# { $path } is the installer's file.
core-verify-no-package = { $path } no contiene ningún paquete
# { $path } is a folder.
core-verify-not-unpacked = el paquete de este instalador no se ha desempaquetado en { $path }
core-verify-schema-supported = El esquema del manifiesto del paquete es compatible.
core-verify-packaged-file-missing = Falta el archivo empaquetado.
core-verify-no-package-path = Un archivo que se puede copiar no tiene ruta de acceso relativa en el paquete.
core-verify-folder-there = La carpeta vacía existe.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-unreadable = No se ha podido leer para comprobarlo: { $error }
core-verify-restored-file-missing = Falta el archivo restaurado.
# { $expected } and { $actual } are sizes in bytes.
core-verify-size-mismatch = El tamaño del archivo no coincide. Se esperaban { $expected } bytes y hay { $actual } bytes.
core-verify-no-hash = El manifiesto no registra ningún hash para este archivo, así que no se puede comprobar.
core-verify-hash-mismatch = El hash SHA-256 no coincide.
core-verify-file-matches = El tamaño y el hash del archivo coinciden con el manifiesto.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-payload-failed = Error al verificar la carga útil de la licencia: { $error }
core-verify-license-payload-missing = Falta la carga útil de la licencia empaquetada.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-failed = Error al verificar el almacén de licencias: { $error }
core-verify-license-store-file-missing = Falta el archivo empaquetado del almacén de licencias.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-path-invalid = La ruta de acceso del almacén de licencias empaquetado no es válida: { $error }
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-prerequisite-failed = Error al verificar la carga útil del requisito previo: { $error }
core-verify-prerequisite-missing = Falta la carga útil del requisito previo incluido.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-failed = Error al verificar la imagen del controlador: { $error }
core-verify-bundled-driver-missing = Falta la imagen del controlador incluido.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-path-invalid = La ruta de acceso del controlador incluido no es válida: { $error }
core-verify-prerequisite-present = El requisito previo de entorno de ejecución está presente en el equipo de destino.
core-verify-prerequisite-not-detected = No se ha detectado el requisito previo de entorno de ejecución; puede que necesite un reinicio o que haya que completarlo manualmente (consulta el archivo de registro de la restauración para ver el resultado definitivo).
core-verify-prerequisite-not-probed = La presencia del requisito previo no se puede comprobar en el Registro; la instalación se ha hecho en la medida de lo posible.
core-verify-license-file-missing = Falta el archivo de licencia restaurado.
core-verify-license-store-restored-missing = Falta el archivo restaurado del almacén de licencias.
core-verify-registry-key-there = La clave del Registro existe.
core-verify-registry-value-matches = El valor del Registro coincide con los datos archivados.
core-verify-registry-value-differs = El valor del Registro no coincide con los datos archivados.
core-verify-firewall-rule-there = El Firewall de Windows tiene esta regla.
core-verify-variable-matches = El valor de la variable de entorno coincide con los datos archivados.
core-verify-variable-differs = El valor de la variable de entorno no coincide con los datos archivados.
# { $error } is what Windows said, in its own words.
core-verify-variable-unreadable = La variable de entorno falta o no se puede leer: { $error }
core-verify-service-runs-other = El servicio existe, pero no ejecuta el binario restaurado con sus argumentos.
core-verify-service-other-account = El servicio existe, pero no se ejecuta con la cuenta esperada.
core-verify-service-matches = El servicio existe y ejecuta el binario restaurado con sus argumentos y su cuenta.
core-verify-service-missing = Falta el servicio.
# { $error } is what Windows said, in its own words.
core-verify-service-unreadable = No se puede leer el servicio: { $error }
core-verify-driver-other-image = El servicio del controlador no apunta a la imagen restaurada.
core-verify-driver-not-registered = El servicio del controlador incluido no está registrado después de la restauración.
# { $error } is what Windows said, in its own words.
core-verify-driver-unreadable = No se puede leer el servicio del controlador: { $error }
core-verify-driver-image-missing = Falta la imagen del controlador.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-unchecked = No se ha podido comprobar la imagen del controlador: { $error }
core-verify-task-matches = La tarea programada existe y ejecuta los programas restaurados.
core-verify-task-runs-other = La tarea programada existe, pero no ejecuta los programas restaurados.
core-verify-task-missing = Falta la tarea programada.
# { $error } is what Windows said, in its own words.
core-verify-task-unreadable = Error al consultar la tarea programada: { $error }
core-verify-shortcut-matches = El acceso directo existe y su ruta de destino coincide con el destino restaurado.
core-verify-shortcut-opens-other = El acceso directo existe, pero su ruta de destino no coincide con el destino restaurado.
core-verify-shortcut-missing = Falta el acceso directo.
# { $error } is what Windows said, in its own words.
core-verify-shortcut-unreadable = Error al consultar el acceso directo: { $error }

## Warnings and errors a check gives, which the app and the installer show.

core-warning-verify-schema-unsupported = El esquema del manifiesto del paquete no es compatible.
# { $error } is what went wrong, in Windows' or the program's own words.
core-warning-verify-package-file-failed = Error al verificar el archivo del paquete: { $error }
core-warning-verify-folder-missing = Falta la carpeta vacía.
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-key-missing = La clave del Registro falta o no se puede leer: { $error }
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-value-missing = El valor del Registro falta o no se puede leer: { $error }
core-warning-verify-registry-kind-missing = Falta el tipo del valor del Registro.
core-warning-verify-firewall-rule-missing = El Firewall de Windows no tiene esta regla.
core-warning-verify-service-scope = La verificación de servicios lee la configuración de servicios de HKLM y no inicia servicios.
core-warning-verify-driver-manual = WinAppMigrator no redistribuye uno o varios controladores de fabricantes o integrados en Windows; reinstálalos con el paquete de controlador firmado del fabricante en el equipo de destino.
core-warning-verify-task-scope = La verificación de tareas programadas solo comprueba que estén registradas y a qué comando hacen referencia.

## The verification report written beside a package, verification_report.md. Its headings follow a "#".

core-verify-report-title = Informe de verificación de WinAppMigrator
# { $time } is when, as a date and time in UTC, such as 2026-11-15T10:00:00Z.
core-verify-report-verified = Verificado (UTC): { $time }
# { $valid } is true or false, which are not translated.
core-verify-report-package-valid = Paquete válido: { $valid }
# { $valid } is true or false, which are not translated.
core-verify-report-restore-valid = Restauración válida: { $valid }
core-verify-report-restore-not-run = Restauración válida: no ejecutada
core-verify-report-items = Elementos
# { $message } is what a check found, and { $source } the file or setting it looked at.
core-verify-report-item-from = { $message } Origen: { $source }
core-verify-report-warnings = Advertencias
core-verify-report-errors = Errores
core-verify-report-none = Ninguno.
