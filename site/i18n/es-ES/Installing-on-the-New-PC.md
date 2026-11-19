<a id="installing-on-the-new-pc"></a>

# Instalar en el PC nuevo

El instalador es una sola ventana. Se comprueba a sí mismo, te muestra dónde va a instalar y todo lo que va a configurar, y solo instala cuando haces clic en **Instalar**. Esta página lo recorre de arriba abajo, con las palabras que aparecen en la pantalla.

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/installer-dark.png">
    <img alt="El instalador de Fabrikam Data Sync, que muestra que no está firmado, la ubicación de instalación, el espacio necesario y lo que configura además de los archivos" src="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/installer-light.png" width="600">
  </picture>
</p>

<a id="before-you-start"></a>

## Antes de empezar

- **Copia el instalador completo.** Si la creación te dio un archivo `.wamdata` además del `.exe`, copia los dos en la misma carpeta y conserva sus nombres. Consulta [Aplicaciones grandes](Big-Apps).
- **Ejecuta solo instaladores de confianza.** Un instalador se ejecuta con [permisos de administrador](Glossary#administrator-rights) completos. Trátalo como cualquier otro programa: ejecútalo solo si lo has creado tú o si te lo ha dado alguien de confianza. Consulta [Privacidad y seguridad](Privacy-and-Security).
- **Necesitas permisos de administrador** en este PC, y [Direct3D 12](Glossary#direct3d-12) para la ventana. Sin Direct3D 12 puedes instalar igualmente desde un símbolo del sistema. Consulta [Línea de comandos](Command-Line).

<a id="open-it"></a>

## Abrirlo

Haz doble clic en el instalador. Windows muestra la [solicitud de UAC](Glossary#uac-prompt). Un instalador no está firmado a menos que quien lo creó lo haya firmado, así que la solicitud suele indicar **Editor desconocido**. Es lo esperado. Consulta [Privacidad y seguridad](Privacy-and-Security#signing).

La ventana, "Instalador de WinAppMigrator", "Reinstala una aplicación capturada en otro PC", se abre al instante, sea cual sea el tamaño del instalador. Indica "Comprobando el instalador…" mientras comprueba el [hash](Glossary#hash) de todo su paquete. Para ello lo lee entero, así que un instalador grande tarda un rato. Desde ese momento, el instalador mantiene abierto su propio archivo, y su archivo de datos si lo tiene, para que ninguno de los dos se pueda cambiar ni sustituir mientras la ventana esté abierta.

La comprobación puede detenerse aquí, con un mensaje en rojo. Los más habituales son:

- El instalador está dañado o se ha modificado después de crearse: vuelve a copiarlo desde donde se creó.
- Falta su archivo de datos, o no es el correcto: vuelve a copiar los dos archivos en una misma carpeta.
- "El paquete de este instalador no se puede usar en este PC", por ejemplo porque los programas de la aplicación están compilados para otro tipo de procesador o necesitan una versión más reciente de Windows. No se ha cambiado nada.

Todos se explican en [Solución de problemas](Troubleshooting#installing).

Cuando la comprobación termina bien, el título de la ventana pasa a ser "Instalar" y el nombre de la aplicación.

<a id="its-language"></a>

### Su idioma

Un instalador habla el idioma del PC en el que se ejecuta, no el del PC en el que se creó: el idioma de presentación de Windows si es uno de los 24 que tiene WinAppMigrator, y si no, inglés. **Idioma**, en la parte superior derecha, lo cambia para esta ejecución. Lo que muestra de la aplicación, incluidas las notas que escribió la aplicación al crearlo, aparece también en ese idioma. Un instalador creado con WinAppMigrator 0.6 o anterior muestra esas notas en inglés, tal como se escribieron. Una [instalación silenciosa desde la línea de comandos](Command-Line) siempre responde en inglés, para que la lean los scripts.

<a id="the-app-and-where-it-goes"></a>

## La aplicación y dónde va

La primera tarjeta tiene el nombre de la aplicación, con el editor y la versión debajo, por ejemplo "Fabrikam Inc. · 1.7.3". Después:

| Fila | Qué indica |
| --- | --- |
| **Capturada en** | El nombre del PC del que procede la aplicación. Solo aparece si el instalador se creó con la opción **Datos de usuario** activada. |
| **Instalador** | Si este archivo de instalador está [firmado](Glossary#signature): "Comprobando su firma…" y, después, "Sin firmar: editor desconocido", o "Firmado por" y el nombre del firmante. Si la firma no es válida, aparece en su lugar un mensaje en rojo: "La firma de este instalador no es válida", con el motivo que da Windows, y "Puede que se haya modificado después de firmarse." |
| **Ubicación de instalación** | La carpeta en la que va la aplicación, con **Examinar** para elegir una. Al principio es la carpeta en la que estaba la aplicación en el PC antiguo. Una aplicación de un Windows de 32 bits empieza en `Program Files (x86)`, donde Windows de 64 bits guarda las aplicaciones de 32 bits. Si esa carpeta no se puede usar aquí, un aviso en rojo debajo indica por qué. |
| **Espacio necesario** | Cuánto espacio necesita la instalación en esa unidad y cuánto hay libre, por ejemplo "hasta 824,0 MB en C: (356,0 GB libres)". Cuenta el paquete desempaquetado además de los archivos de la aplicación. |

El nombre del editor de la primera línea procede del paquete, así que cualquiera podría haberlo escrito. La fila **Instalador** procede de Windows, y es la que merece confianza.

<a id="when-the-location-cannot-be-used"></a>

### Cuando no se puede usar la ubicación

Si la carpeta de **Ubicación de instalación** no se puede usar, un aviso en rojo debajo indica por qué. **Instalar** y **Vista previa de los cambios** siguen atenuados hasta que elijas otra, y una línea junto a ellos indica "Instalar y Vista previa de los cambios están desactivados hasta que se pueda usar la ubicación de instalación."

El motivo más habitual es que la carpeta ya contiene archivos. Una carpeta con archivos solo se usa si los colocó ahí una instalación anterior de WinAppMigrator, para que nunca se reemplacen los archivos de otra aplicación:

| Aviso | Qué significa | Qué hacer |
| --- | --- | --- |
| "Ya hay una instalación de", el nombre de la aplicación, y "en esta carpeta, así que esta copia no puede ir aquí." | La aplicación ya está instalada en este PC, a menudo con su propio instalador: su programa principal está en esa carpeta, o la lista de aplicaciones instaladas de Windows la sitúa ahí. Ocurre con aplicaciones que ya tienen muchos PC, como un navegador web, un reproductor multimedia u Office. | Desinstala primero la copia de este PC, en Configuración de Windows, en **Aplicaciones**, y vuelve a abrir el instalador. O escribe una carpeta nueva y vacía en **Ubicación de instalación** para instalar esta copia junto a la otra. |
| "En esta carpeta está instalado" y el nombre de otra aplicación, y que esta aplicación "no puede ir aquí." | La lista de aplicaciones instaladas de Windows sitúa otra aplicación en esa carpeta, como otra edición de Office. | Escribe una carpeta nueva y vacía en **Ubicación de instalación**. O, si ya no necesitas la otra aplicación, desinstálala primero, en Configuración de Windows, en **Aplicaciones**, y vuelve a abrir el instalador. |
| "Esta carpeta ya contiene archivos que no instaló WinAppMigrator." | Hay otra cosa en la carpeta, y no figura ninguna aplicación instalada ahí. Puede ser de otra aplicación. | Escribe una carpeta nueva y vacía en **Ubicación de instalación**. Si los archivos son restos de una aplicación que has desinstalado, puedes eliminar la carpeta y volver a abrir el instalador. |

Por cualquier otro motivo, el aviso indica "Esta carpeta no se puede usar:", el motivo y "Escribe otra carpeta arriba." Los motivos son:

| Motivo en pantalla | Qué hacer |
| --- | --- |
| "debe ser una ruta de acceso completa" | Escribe una ruta de acceso que empiece por una letra de unidad, como `C:\Program Files\Fabrikam Data Sync`. |
| "no puede ser una unidad entera" | Usa una carpeta de la unidad, no la propia unidad. |
| "no puede ser la carpeta de Windows" | Elige una carpeta fuera de `C:\Windows`. |
| "contiene otros elementos uno junto a otro, así que la aplicación necesita una carpeta dentro de ella" | Agrega una carpeta para la aplicación, como `C:\Program Files\Fabrikam Data Sync` en lugar de `C:\Program Files`. |
| "es una carpeta que pertenece a Windows, o cuyo contenido se ejecuta o se carga por sí solo" | Elige una carpeta normal. Por ejemplo, se rechaza una carpeta Inicio. |
| "tiene que ser una ruta de acceso normal en una unidad de este PC" | Se rechaza un [recurso compartido de red](Glossary#network-share), y también un nombre que Windows interpretaría como otra cosa. |
| que "es un vínculo a otro lugar, y otros podrían cambiar adónde lleva" | Usa la carpeta a la que lleva el [vínculo](Glossary#link). |
| "es donde las instalaciones guardan su historial" | Elige una carpeta fuera de `%ProgramData%\WinAppMigrator`. |

Si la carpeta contiene archivos y **sí** consta ahí una instalación anterior de este proyecto, el instalador la usará, y advierte: "Esta carpeta ya contiene archivos. Los archivos con el mismo nombre se reemplazan y vuelven a su lugar si desinstalas."

Una vez aceptada una carpeta, la ventana reemplaza un archivo, valor del Registro, servicio, tarea, acceso directo o variable con el mismo nombre, como hace `--force` en la [línea de comandos](Command-Line#what---force-does). Lo que ya pertenece a otra aplicación, como su servicio o su tipo de archivo, sigue siendo de esa aplicación.

Si parece que la unidad no tiene espacio suficiente, una nota en ámbar indica "Puede que aquí no haya espacio suficiente" y lo que se necesita. El paquete se puede desempaquetar en otra unidad, pero los archivos de la aplicación tienen que caber igualmente en esta. La propia instalación vuelve a comprobar el espacio antes de empezar.

<a id="an-earlier-install-that-never-finished"></a>

### Una instalación anterior que no llegó a terminar

Si una instalación anterior con cualquiera de estos instaladores se interrumpió, por un bloqueo o un corte de corriente, una nota en ámbar indica "Una instalación anterior de" esa aplicación "no llegó a terminar. Instalar la deshace primero." Esto ocurre automáticamente al hacer clic en **Instalar**.

<a id="what-this-sets-up-beyond-files"></a>

## Qué configura esto además de los archivos

Esta tarjeta es la lista que aceptas al hacer clic en **Instalar**. Debajo de su título indica:

> Todo esto se hace con permisos de administrador. Si la instalación falla, se revierte lo que haya cambiado y se te indica lo que no se haya podido revertir. También puedes deshacer la instalación más adelante desde Configuración, en Aplicaciones instaladas. Los archivos que reemplace vuelven a su lugar, y los entornos de ejecución que instale se quedan.

Cada fila tiene un nombre, una etiqueta a la derecha que indica qué tipo de elemento es y una línea que indica lo que hace. Una etiqueta en el azul habitual es normal. Una etiqueta en **ámbar** marca algo que se ejecuta con todos los privilegios, como un servicio que se ejecuta como LocalSystem, un controlador o una regla de firewall que permite conexiones entrantes. Revísalas dos veces.

Si la lista está vacía, indica "Nada: solo copia archivos y configuración en las claves del Registro propias de la aplicación."

La lista se calcula para la ubicación que hay en el cuadro. Si cambias la ubicación, indica "Calculando qué configuraría esto en esta ubicación…" durante un momento, y **Instalar** espera.

<a id="what-each-label-means"></a>

### Qué significa cada etiqueta

| Etiqueta | Qué es | Ejemplo de la línea de debajo |
| --- | --- | --- |
| **Servicio** | Un [servicio](Glossary#service): un programa que Windows ejecuta en segundo plano | "Ejecuta FabrikamSync.exe como NT AUTHORITY\LocalService, se inicia ahora y cada vez que se inicia este PC". Otros indican "se inicia ahora y poco después de cada inicio de este PC", "solo se inicia cuando algo lo solicita" o "está deshabilitado". |
| **Controlador** | Un [controlador](Glossary#driver) propio de la aplicación | "Carga" un archivo "en el propio Windows". Siempre en ámbar. Uno que se inicia con Windows se inicia justo después de la instalación. El de VeraCrypt se cargó así a mano en un PC de prueba; la CI no carga ninguno. |
| **Tarea programada** | Una [tarea programada](Glossary#scheduled-task) | "Se ejecuta como SYSTEM", o como una cuenta. "con los privilegios más altos que pueda tener" se marca en ámbar. Una tarea que en el PC antiguo se ejecutaba como una persona se ejecuta como quien la instale. |
| **Se inicia automáticamente** | Un [elemento de inicio](Glossary#startup-item) | "Se ejecuta cuando inicias sesión", "Se ejecuta para todos los que inicien sesión" o "Un acceso directo en la carpeta Inicio, para ti" |
| **Acceso directo** | Un [acceso directo](Glossary#shortcut) en el escritorio o en el menú Inicio | "Abre Fabrikam.exe, para todos los usuarios de este PC" o "para ti" |
| **PATH** | Una entrada agregada a la [variable Path](Glossary#path-variable) | "Se agrega a Path" y "La carpeta de instalación\bin". Se agrega a lo que ya hay, nunca lo sustituye. |
| **Variable de entorno** | Una [variable de entorno](Glossary#environment-variable) | "Se establece en" su valor |
| **Extensión del shell** | Código que se carga en el Explorador de archivos | "Carga" un archivo "en el Explorador de archivos, para todos los usuarios de este PC". En ámbar. |
| **Comando de menú** | Un comando del menú contextual | "Agrega un comando al menú de" un tipo de archivo. "Cambia lo que ocurre al abrir uno de estos" se marca en ámbar. |
| **Tipo de vínculo** | Un [protocolo de vínculo](Glossary#link-protocol), como `fabrikam:` | "Los vínculos de este tipo se abren con la aplicación" |
| **Tipo de archivo** | Un [tipo de archivo](Glossary#file-type), como `.fds` | "Los archivos de este tipo se abren con la aplicación" |
| **COM** | [Clases COM](Glossary#com-class) que pueden iniciar otros programas | "COM en" un archivo, con cuántos valores de configuración. "Permisos de COM" se marca en ámbar. |
| **Regla de firewall** | Una [regla de firewall](Glossary#firewall-rule) para uno de sus programas | "Permite las conexiones entrantes en el puerto 8080 para FabrikamSync.exe, en todas las redes". Permitir conexiones entrantes se marca en ámbar. |
| **Origen del registro de eventos** | Un nombre con el que la aplicación escribe en los registros de eventos de Windows | "Puede escribir en el registro Application, con mensajes de sus propios archivos" |
| **Fuente** | Una fuente de la carpeta de instalación | "Se carga desde la carpeta de instalación al iniciar sesión" |
| **Controlador ODBC** | Un controlador de base de datos para [ODBC](Glossary#odbc) | "Se carga en los programas que usan ODBC para acceder a una base de datos". En ámbar. |
| **Origen de datos ODBC** | Una conexión a una base de datos guardada | "Una conexión a una base de datos, para todos los usuarios de este PC" |
| **Configuración de compatibilidad** | Cómo inicia Windows algunos de sus programas | "Cómo se inician sus programas" |
| **Aplicaciones predeterminadas** | Una entrada que permite elegir la aplicación en Aplicaciones predeterminadas | "Se puede elegir en Aplicaciones predeterminadas. No se cambia nada para que se use" |
| **Nombre de programa** | Un nombre con el que Ejecutar y otros programas pueden encontrarla | "Ejecutar y otros programas pueden encontrarlo por este nombre" |
| **Otra configuración** | Valores del [Registro](Glossary#registry) fuera de las claves propias de la aplicación y de los tipos anteriores | "3 valores de configuración fuera de las claves propias de la aplicación", "En" las claves. En ámbar si alguno está en claves de Microsoft. |

Algunas cosas solo se pueden valorar cuando la instalación llega a ellas, como un servicio con el mismo nombre que ya pertenece a otra aplicación. Esas siguen en la lista tal como se solicitaron. Cuando la instalación llega ahí, las deja como están y lo indica.

<a id="notes-and-what-is-left-out"></a>

### Notas y lo que se excluye

Debajo de las filas, unas notas en ámbar indican lo que conviene que sepas, por ejemplo:

- que las credenciales no se han incluido en el instalador, así que puede que tengas que volver a escribirlas;
- que la opción **Datos de licencia y activación** o **Datos de usuario** estaba desactivada cuando se creó, así que esos datos no están incluidos;
- qué [certificados](Glossary#certificate) parecía necesitar la aplicación, que nunca se instalan;
- lo que excluyó quien creó el instalador, y lo que agregó;
- que puede que la aplicación no funcione aquí: "Esta aplicación estaba en la compilación" de Windows, una más reciente que la de este PC, o que Windows ejecutaría algunos de sus programas "mediante un emulador";
- que los archivos de Documentos o Juegos guardados irán "dentro de" la carpeta de instalación, "en una carpeta llamada _external", porque la carpeta de este PC está fuera de tu perfil.

Después, en ámbar, cada cosa que excluyen las reglas de seguridad, con el motivo. Las 20 primeras se muestran completas. Las demás están bajo "Se han excluido 12 elementos más" (con el número que corresponda), que puedes abrir. El resto de la instalación sigue adelante sin ellas. Lo que se rechaza, y por qué, está en [Qué incluye un instalador](What-Goes-in-an-Installer#what-the-installer-refuses).

Si el paquete no puede ir de ninguna manera a la ubicación elegida, un mensaje en rojo indica "Este paquete no puede ir a esta ubicación" y por qué.

<a id="options"></a>

## Opciones

Esta tarjeta solo aparece cuando el instalador contiene datos de usuario o datos de licencia. Cada casilla empieza marcada.

| Opción | Qué indica |
| --- | --- |
| **Datos de usuario** | "Activado: la configuración, Documentos y Juegos guardados van a las carpetas habituales de quien haya iniciado sesión. Desactivado: todo va dentro de la carpeta de instalación y no se configura nada para ejecutarse desde ahí." Si el instalador indica de quién son los datos que contiene, una línea debajo dice "Contiene los datos de" esa cuenta. |
| **Licencia** | "Archivos de activación y licencia, y los valores del Registro que contienen licencias. Puede que algunas tengan que volver a activarse aquí." |

Al cambiar una casilla, la lista de arriba cambia al instante para coincidir.

Los datos de usuario siempre van a **quien haya iniciado sesión** en Windows, aunque en la solicitud de UAC se haya escrito el nombre de otro administrador. Si el instalador no puede saber quién es, lo indica, excluye la configuración por usuario y coloca los datos de usuario dentro de la carpeta de instalación en lugar de adivinar. Si nadie ha iniciado sesión en esa sesión, los datos de usuario y la configuración por usuario van a la cuenta con la que se ejecuta el instalador, y también lo indica.

<a id="dependencies"></a>

## Dependencias

"Qué necesita esta aplicación y cómo se proporciona cada cosa." Cada [entorno de ejecución](Glossary#runtime) tiene una etiqueta:

| Etiqueta | Significado |
| --- | --- |
| **Incluido** | Dentro del instalador. Se instala sin internet, excepto el DirectX End-User Runtime, cuyo instalador lo descarga de Microsoft. De estos, solo el entorno de ejecución de Visual C++ x64 se ha instalado de verdad en las pruebas del proyecto. |
| **Se instala con winget:** y un id. | No se pudo descargar cuando se creó el instalador, así que el instalador usa [winget](Glossary#winget), que necesita internet. Esta vía no se ha probado en las pruebas del proyecto. |
| **Característica de Windows: NetFx3** | .NET Framework 3.5, activado como [característica de Windows](Glossary#windows-feature). Puede necesitar internet. Esta vía no se ha probado en las pruebas del proyecto. |
| **Integrado en Windows** | .NET Framework 4, que Windows 10 y 11 ya tienen |
| **Viene con la aplicación** | La aplicación lleva su propia copia |
| **Instalar manualmente** | Aquí no hay ningún instalador para él. Instálalo tú. |

Si no hay ninguno, indica "No se han detectado más dependencias de entornos de ejecución." Consulta [Entornos de ejecución](Runtimes).

<a id="preview-changes"></a>

## Vista previa de los cambios

**Vista previa de los cambios** calcula exactamente lo que haría una instalación, sin cambiar nada. Comprueba que haya espacio, desempaqueta el paquete en una carpeta privada, calcula los cambios y vuelve a quitar esa carpeta. La tarjeta del resultado tiene el título "Vista previa: todavía no se ha cambiado nada", con **Cambios previstos** y una lista de los cambios, con los servicios, las tareas y los accesos directos primero. La ventana muestra los 300 primeros y, después, cuántos más hay.

Una vista previa no comprueba el hash de cada archivo. El hash de todo el paquete ya se comprobó al abrirse la ventana.

<a id="install"></a>

## Instalar

**Instalar** sigue atenuado hasta que se puede usar la ubicación y la lista corresponde a esa ubicación. Si el paquete no puede ir de ninguna manera a esa ubicación, una línea junto al botón indica "Instalar está desactivado porque este paquete no puede ir a esta ubicación", y el motivo aparece en rojo en **Qué configura esto además de los archivos**. Al hacer clic en él:

1. Vuelve a calcular la lista. Si lo que haría ha cambiado desde que la revisaste, no instala nada, muestra la nueva lista e indica "Lo que esto haría aquí ha cambiado, así que no se ha instalado nada. Revisa la lista de abajo y vuelve a hacer clic en Instalar si es lo que quieres."
2. Comprueba que haya espacio en cada unidad en la que escribe.
3. Desempaqueta el paquete en una carpeta que solo pueden usar los administradores, y comprueba el hash de cada archivo a medida que lo extrae.
4. Instala, y antes de cada cambio lo anota en un [diario](Glossary#journal), para poder deshacerlo.
5. Vuelve a leer cada archivo que ha instalado y comprueba su hash.
6. Inicia los servicios configurados para iniciarse solos.

En la parte inferior ves lo que está haciendo, un recuento cuando lo hay, el tiempo que lleva, una barra de progreso y **Cancelar**. Los pasos que puedes ver son "Preparando…", "Desempaquetando", "Comprobando el paquete", "Deshaciendo una instalación anterior que no llegó a terminar", "Instalando entornos de ejecución", "Copiando archivos", "Configurando servicios", "Configurando controladores", "Escribiendo la configuración del Registro", "Restaurando los datos de licencia", "Configurando tareas programadas", "Creando accesos directos", "Estableciendo variables de entorno", "Agregando la aplicación a Configuración" y "Comprobando lo que se ha instalado".

**Cancelar** muestra "Cancelando después del paso actual…" y, después, "Revirtiendo lo que se ha cambiado". Cerrar la ventana durante la instalación equivale a **Cancelar**: la ventana sigue abierta hasta que se ha revertido todo. Si todavía no se había cambiado nada, indica "Cancelado. No se ha cambiado nada."

<a id="the-hash-check"></a>

## La comprobación de hash

El instalador comprueba los [hashes](Glossary#hash) tres veces:

1. **Al abrirse la ventana**: el hash de todo el paquete. Una copia truncada, o una modificada después de crearse, se rechaza antes de leer nada de su contenido.
2. **Al desempaquetar**: el hash de cada archivo, antes de escribir nada en su lugar.
3. **Después de instalar**: vuelve a leer cada archivo que ha instalado y lo comprueba de nuevo.

Los hashes detectan los daños, y los cambios de cualquiera que no actualice también los hashes. No demuestran quién creó un instalador. Eso solo lo hace una [firma](Glossary#signature).

<a id="the-result"></a>

## El resultado

La tarjeta del resultado aparece en la parte superior, con el título "Resultado de la instalación".

| Mensaje | Qué significa |
| --- | --- |
| "La aplicación se ha instalado y ha superado la comprobación de hash." | Todo está bien. Si las reglas excluyeron algo, agrega "sin lo que aparece abajo como excluido". |
| "La instalación se ha cancelado y se ha revertido lo que cambió." | Has cancelado. |
| "La instalación ha fallado y se ha revertido lo que cambió." | Algo ha fallado. Se ha revertido lo que cambió, excepto los entornos de ejecución que haya instalado. Lo que aparezca en ámbar debajo se ha quedado, con el motivo, como un servicio que solo desaparece cuando se reinicia el PC. |
| "La instalación ha fallado y no se ha podido revertir todo lo que cambió. Lo que queda aparece abajo, y la próxima instalación lo volverá a intentar." | Lee la lista. Al volver a ejecutar un instalador, se vuelve a intentar revertirlo. |
| "La instalación no ha terminado correctamente. Revisa los problemas de abajo." | Lee los mensajes de debajo. |

Después hay tres recuadros: **Cambios realizados** (o **Cambios revertidos** tras un error), **Advertencias** y **Errores**. Las etiquetas indican **Hashes del paquete válidos** y **Archivos instalados válidos**, o **NO VÁLIDOS** en rojo. Se muestran como mensajes hasta cinco advertencias y todos los errores. "Una dependencia necesita un reinicio para terminar. Reinicia Windows y, después, abre la aplicación." significa que un entorno de ejecución ha pedido un reinicio.

**Cambios**, con un recuento, abre la lista de lo que se ha hecho, hasta 300 elementos. Si la instalación no ha terminado correctamente, la ruta de acceso de su [archivo de registro](Glossary#log) aparece como **Archivo de registro**.

Después de una instalación correcta, los botones de la parte inferior pasan a ser **Cerrar** y **Abrir carpeta de instalación**.

<a id="where-things-end-up"></a>

## Dónde acaba cada cosa

- **Los archivos de programa** van en la ubicación de instalación.
- **Datos de usuario**, con esa casilla marcada, van a las carpetas AppData, Documentos y Juegos guardados de la cuenta que ha iniciado sesión, y a ProgramData. No se escribe en una carpeta Documentos o Juegos guardados que este PC tenga fuera de tu perfil. Esos archivos van dentro de la ubicación de instalación, en `_external`, y la lista lo indica en primer lugar. Con la casilla desmarcada, todos los datos de usuario van dentro de la ubicación de instalación.
- **Los accesos directos** van al escritorio y al menú Inicio de este PC: los tuyos, o los de todos los usuarios si es ahí donde estaban.
- **Path** solo recibe las entradas de la aplicación. No cambia nada más en esa variable.
- **Lo que ya tiene aquí otra aplicación sigue siendo de esa aplicación.** Un tipo de archivo, tipo de vínculo, clase COM, servicio, origen del registro de eventos, fuente o cualquier cosa similar que ya pertenezca a otra cosa se deja como está, y el instalador lo indica. Una aplicación que esperaba abrir los vínculos `http`, por ejemplo, tiene que volver a establecerse como predeterminada en Configuración.
- **Configuración** recibe una entrada para la aplicación, para que puedas desinstalarla como cualquier otra. Consulta [Deshacer una instalación](Undoing-an-Install).

<a id="common-reasons-it-refuses"></a>

## Motivos habituales de rechazo

| Lo que ves | Por qué | Qué hacer |
| --- | --- | --- |
| "esta aplicación no puede ejecutarse en este PC, así que no se ha cambiado nada" | Sus programas están compilados para otro tipo de procesador, o necesitan una versión más reciente de Windows | Instala en un PC que coincida con el antiguo |
| "no hay espacio suficiente para esto, así que no se ha cambiado nada" | No hay suficiente espacio libre en una unidad en la que escribe | Libera espacio o elige una ubicación en otra unidad |
| "se está ejecutando otra instalación o desinstalación" | Solo se ejecuta una a la vez | Espera a que termine y vuelve a intentarlo |
| Un archivo "tiene un nombre que esta versión de Windows abre como dispositivo y no como archivo" | La aplicación tiene un archivo con un nombre como `aux.h`, que las versiones de Windows anteriores a Windows 11 no admiten | Instala en Windows 11 |
| Una carpeta de datos como `%AppData%` está debajo de un vínculo | El instalador no escribe a través de un vínculo | Desmarca **Datos de usuario** y **Licencia**, e instala sin ellos |

Hay más en [Solución de problemas](Troubleshooting#installing).
