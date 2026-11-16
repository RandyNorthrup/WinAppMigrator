<a id="troubleshooting"></a>

# Solución de problemas

Busca abajo lo que ves, por lo que ocurre o por las palabras que aparecen en la pantalla. Las palabras entre comillas son las que muestra el programa, a veces con un nombre o un número en medio. La línea de comandos de un instalador siempre responde en inglés, así que en la sección Scripts esas palabras aparecen en inglés.

Si nada de esto te ayuda, consulta [Obtener ayuda](#getting-help) al final.

<a id="the-window-could-not-open"></a>

## No se ha podido abrir la ventana

**Lo que ves.** Un cuadro de mensaje en lugar de la ventana:

- de la aplicación: "No se ha podido abrir la ventana" y "Se necesita Direct3D 12 para dibujar la ventana, y es posible que un controlador de gráficos antiguo o una máquina virtual no lo tengan."
- de un instalador: lo mismo, y "Aun así, se puede instalar sin ella: ejecuta este archivo desde un símbolo del sistema como administrador con --apply y, para elegir dónde, con --target-root seguido de una carpeta."
- de un desinstalador: lo mismo, y "Aun así, se puede desinstalar sin ella, desde un símbolo del sistema como administrador:" con el comando exacto.

**Por qué.** Las ventanas se dibujan con [Direct3D 12](Glossary#direct3d-12), y no hay ninguna otra forma de dibujarlas.

**Qué hacer.**

- Actualiza el controlador de gráficos del PC.
- Para un instalador o un desinstalador, usa la línea de comandos. Consulta [Línea de comandos](Command-Line).
- La aplicación no tiene línea de comandos. Crea el instalador en otro PC: puedes conectar a ese PC la unidad del PC antiguo. Consulta [Mover desde una unidad conectada](Moving-from-a-Connected-Drive). Ese PC también puede leer el antiguo a través de la red. Consulta [Mover desde un PC a través de la red](Moving-over-the-Network).

<a id="starting-the-app"></a>

## Iniciar la aplicación

| Lo que ves | Qué significa | Qué hacer |
| --- | --- | --- |
| La solicitud de UAC indica "Editor desconocido" para la aplicación | Desde la versión 0.5.0, la aplicación está firmada. Una aplicación sin firmar no es el archivo publicado, o es anterior a la 0.5.0. | Vuelve a descargarla de la [última versión](https://github.com/RandyNorthrup/WinAppMigrator/releases/latest) y [compruébala](Getting-Started#check-the-download). |
| SmartScreen indica "Windows protegió tu PC" | SmartScreen no ha visto ese archivo a menudo | Elige **Más información**, comprueba que el editor es Randy Northrup y, después, elige **Ejecutar de todas formas**. |
| La etiqueta de la parte superior indica **Error al cargar**, y la lista indica "No se han podido enumerar las aplicaciones" | No se ha podido leer la lista de aplicaciones | Haz clic en la flecha circular para volver a cargarla. Si es una unidad conectada, comprueba que sigue conectada. |
| La aplicación que quieres no está en la lista | Es una aplicación de Microsoft Store, una parte oculta de otra aplicación, o no tiene ninguna entrada en Configuración, como una aplicación portátil | Las aplicaciones de Store no se pueden mover. Una parte oculta puede aparecer en **También podría formar parte de ella** al elegir la aplicación principal. Una aplicación sin entrada en Configuración no se puede elegir. |
| "No hay aplicaciones que coincidan" | La búsqueda o el filtro lo ocultan todo | Borra el cuadro de búsqueda y elige **Todas**. |

<a id="building"></a>

## Crear el instalador

<a id="picking-a-drive-or-a-disk-image"></a>

### Elegir una unidad o una imagen de disco

| Lo que ves | Qué hacer |
| --- | --- |
| La unidad "no parece una unidad de sistema de Windows (no tiene \Windows ni \Users). Elige la raíz de la unidad del otro PC." | Elige la parte superior de la unidad, como `E:\`, no una carpeta de ella. Comprueba que es la unidad en la que estaba instalado Windows. |
| Una imagen "no es una imagen de disco que Windows pueda montar (.vhd o .vhdx)" | Elige un `.vhd` o un `.vhdx`. Antes, guarda un disco de VMware o VirtualBox como uno de esos. Para un punto de control de Hyper-V, `.avhdx`, exporta el disco de la máquina virtual. |
| Una imagen "no se ha podido montar: Windows ya la tiene montada, y lo que la haya montado podría estar escribiendo en ella." | Está abierta en otro lugar, por ejemplo montada en el Explorador de archivos. Elige en su lugar su unidad en **Unidad conectada…**, o desmóntala y vuelve a elegirla. |
| Una imagen "no tiene ningún volumen con Windows que se pueda leer. Un volumen cifrado con BitLocker se tiene que desbloquear primero." | Comprueba que es una imagen de la unidad en la que estaba instalado Windows. Si está bloqueada con [BitLocker](Glossary#bitlocker), móntala tú en Windows, desbloquéala y elige su unidad en **Unidad conectada…**. |
| Una imagen "es un disco de diferenciación, que solo contiene lo que ha cambiado desde el disco a partir del que se creó" | Combínala con el disco a partir del que se creó, con Editar disco de Hyper-V, y elige ese disco. |
| Una imagen "no se ha podido montar:" y otro motivo | El motivo es el que da Windows. Léelo, comprueba que nada más esté usando la imagen y vuelve a intentarlo. |

<a id="reading-a-pc-over-the-network"></a>

### Leer un PC a través de la red

La lista completa, incluido lo que indica el cliente en el PC antiguo, está en [Mover desde un PC a través de la red](Moving-over-the-Network#if-something-goes-wrong).

| Lo que ves | Qué hacer |
| --- | --- |
| El PC antiguo no aparece en **2. Elige el PC antiguo** | Comprueba que el cliente se está ejecutando allí y muestra un código, y que los dos PC están en la misma red. Después, escribe su nombre o dirección. Si hay un enrutador entre ellos, escribe su dirección. Un firewall que no sea el de Windows tiene que dejar pasar TCP 47732 y UDP 47731. |
| "no se ha encontrado" un nombre "en la red" | Escribe en su lugar la dirección del PC antiguo. |
| "no se ha podido conectar con el PC antiguo:" y un motivo | Comprueba que el cliente sigue ejecutándose y que el nombre o la dirección son correctos. |
| "ese no es el código que muestra el PC antiguo. Compruébalo y vuelve a intentarlo." | Escribe el código que muestra ahora el cliente. Después de cinco códigos incorrectos, genera uno nuevo. Después de 20, no deja emparejar nada hasta que lo vuelvas a iniciar. |
| En el PC antiguo, Windows indica que el editor del cliente es desconocido | Solo es lo esperado en un Windows 7 sin el certificado raíz de 2020 de Microsoft, que no puede comprobar la firma del cliente. Ahí, elige **Sí** solo para un cliente que hayas guardado tú desde la aplicación. En cualquier otro caso, la solicitud debería indicar Randy Northrup. Si no es así, no ejecutes esa copia. |
| La aplicación ya no puede leer el PC antiguo, tras una pausa larga | El cliente cierra una conexión a la que la aplicación no ha pedido nada durante 30 minutos. Haz clic en **Desconectar** y vuelve a emparejar. |

<a id="looking-at-the-app"></a>

### Analizar la aplicación

| Lo que ves | Qué significa | Qué hacer |
| --- | --- | --- |
| "No se ha podido analizar esta aplicación:" y un motivo | El análisis ha fallado. Un motivo habitual es una entrada que no tiene ningún programa, como un paquete de controladores. | Haz clic en **Volver a intentarlo**. Si vuelve a fallar igual, puede que la entrada no sea una aplicación que WinAppMigrator pueda mover. |
| "Detenido. Se podrá crear un instalador cuando se haya analizado esta aplicación." | Has hecho clic en **Detener** | Haz clic en **Volver a analizar**. |
| El análisis tarda mucho | Una aplicación grande, una unidad lenta o que está fallando, o muchas cuentas | Si la unidad puede estar fallando, haz clic en **Detener**, crea una [imagen de disco](Glossary#disk-image) de ella y elígela en su lugar, porque leer una unidad que está fallando puede acabar de estropearla. A partir de la 0.6.0, elígela con **Imagen de disco…**. Con la 0.5.0, móntala y elige su unidad. Si no, espera. Mientras tanto puedes elegir otra aplicación. |
| Una fila que quieres está atenuada y marcada | Contiene el programa principal, o lo que necesita para iniciarse | Siempre se incluye. Consulta [las filas que no se pueden desmarcar](Building-an-Installer#the-groups). |
| Una fila indica "También excluido:" | Algo que desmarcaste se la ha llevado consigo | Vuelve a marcar eso, si quieres incluir esta fila. |
| "Lo que queda no formaría un paquete" | Lo que has desmarcado no deja nada con lo que crear un paquete | Vuelve a marcar algo. |
| "Los datos de licencia de lo que has excluido se incluyen igualmente" | Las licencias solo dependen de la opción **Datos de licencia y activación** | Desmarca esa opción si no quieres incluirlas. |

<a id="create-installer-stays-greyed-out"></a>

### Crear instalador sigue atenuado

Espera hasta que se cumpla todo esto:

- el análisis de la aplicación ha terminado, con las opciones tal como están ahora ("Calculando qué va con eso…" significa que todavía está calculando lo que se lleva lo último que has desmarcado);
- la **Carpeta de salida** es una ruta de acceso completa, como `D:\Installers` ("Escribe la ruta de acceso completa de una carpeta, como D:\Instaladores.");
- hay espacio en esa unidad.

<a id="room"></a>

### Espacio

| Lo que ves | Qué hacer |
| --- | --- |
| "No hay espacio suficiente." y cuánto necesita y cuánto tiene una unidad | Elige una carpeta de salida en otra unidad o excluye algo. La cifra es lo máximo que podría ocupar una creación, antes de comprimir, así que puede ser más de lo que ocupará el instalador. |
| Una unidad "no indica cuánto espacio tiene" | La creación sigue adelante sin comprobar esa unidad. Asegúrate de que tiene espacio. |
| Una unidad FAT32 y una aplicación grande | FAT32 no admite archivos de 4 GiB. Usa una unidad NTFS o exFAT. |

<a id="the-done-screen"></a>

### La pantalla final

| Lo que ves | Qué hacer |
| --- | --- |
| "Instalador creado, pero" algunos archivos "habían desaparecido desde que se analizó la aplicación" | Haz clic en **Volver a analizar** y, después, en **Volver a crear el instalador**, si esos archivos importan. |
| "Instalador creado, pero no se han podido capturar" algunos archivos, "normalmente porque estaban bloqueados o no se podían leer." | Cierra la aplicación, y cualquier otra cosa que use sus archivos, y vuelve a crear el instalador. **Advertencias** indica qué archivos. |
| "Instalador creado, pero todavía revela algo de quién procede." | Lee los mensajes en ámbar de debajo. Consulta [la última revisión](Building-an-Installer#the-last-look). |
| **Advertencias** indica "No se ha podido descargar" un entorno de ejecución, y "el instalador probará con winget en el equipo de destino." | Ese entorno de ejecución no está dentro del instalador, así que el PC nuevo necesitará internet para obtenerlo. Vuelve a crear el instalador con conexión. Consulta [Entornos de ejecución](Runtimes#at-build-time). |
| "No uses este instalador: no ha superado su comprobación de hash." | No lo uses. Vuelve a crearlo. Si vuelve a ocurrir, puede que la unidad esté fallando. |
| "Se ha escrito el instalador, pero no se ha podido ejecutar su comprobación de hash" | No lo uses. Vuelve a crearlo. |
| "Creación cancelada. No se ha escrito ningún instalador." | Has hecho clic en **Cancelar**. |
| "No se ha podido crear el instalador." y un motivo | Lee el motivo. **Copiar error** lo copia, y **Archivo de registro** muestra dónde está el archivo de registro. |
| "No se ha podido crear la carpeta de salida" | Elige una carpeta en la que puedas escribir. |
| "No se ha podido crear una carpeta de preparación protegida, así que no se ha creado nada" | La creación no ha podido crear su carpeta de trabajo privada junto a la carpeta de salida, ni en la carpeta temporal de Windows. Elige una carpeta de salida en otra unidad. |
| "La creación se ha detenido de forma inesperada" | Ha fallado algo que nada preveía. "Si ha dejado un instalador en la carpeta de salida, no lo uses." Por favor, [notifícalo](#getting-help). |

<a id="installing"></a>

## Instalar

<a id="when-the-installer-opens"></a>

### Cuando se abre el instalador

| Lo que ves | Qué significa | Qué hacer |
| --- | --- | --- |
| "Comprobando el instalador…" durante mucho tiempo | Lee todo el paquete para comprobar su hash | Espera. Un instalador grande tarda un rato. |
| "No se ha podido leer el instalador:" y "este instalador está dañado o se ha modificado después de crearse, así que no se ha leído nada de su paquete. Vuelve a copiarlo desde donde se creó." | El archivo está incompleto o se ha modificado | Vuelve a copiarlo. |
| "este instalador guarda su paquete en un archivo llamado" un archivo `.wamdata`, "en la misma carpeta, y ese archivo no está" | A un instalador de dos archivos le falta su archivo de datos | Copia el archivo `.wamdata` en la misma carpeta. Consulta [Aplicaciones grandes](Big-Apps). |
| el archivo de datos "no es el archivo de datos de este instalador, o está dañado o se ha modificado después de crearse" | El archivo de datos no coincide | Vuelve a copiar los dos archivos desde donde se crearon. |
| "El paquete de este instalador no se puede usar en este PC:" y "esta aplicación no puede ejecutarse en este PC, así que no se ha cambiado nada" | Los programas de la aplicación están compilados para otro procesador, o un programa indica que necesita una versión más reciente de Windows | Instala en un PC como el antiguo. No se ha cambiado nada. |
| "Este es el programa base de un instalador de WinAppMigrator y no contiene ningún paquete de aplicación." | Es el programa base sin más, no un instalador creado | Crea un instalador con la aplicación y ejecuta ese. |
| "La firma de este instalador no es válida", con el motivo que da Windows | Estaba firmado, pero Windows no acepta la firma. Puede que haya cambiado después, o que su certificado no sea de confianza en este PC. | No lo instales. Consúltalo con quien lo creó. |
| La solicitud de UAC indica "Editor desconocido" | El instalador no está firmado. Es normal en uno que hayas creado tú. | Ejecútalo solo si confías en su procedencia. |

<a id="the-install-location"></a>

### La ubicación de instalación

Cuando **Instalar** y **Vista previa de los cambios** están atenuados, un aviso en rojo debajo de **Ubicación de instalación** indica por qué. El motivo más habitual es que la aplicación ya está instalada en esa carpeta en este PC, como suele ocurrir con un navegador web, un reproductor multimedia u Office: desinstala primero esa copia, en Configuración de Windows, en **Aplicaciones**, y vuelve a abrir el instalador, o escribe una carpeta nueva y vacía. Los motivos, y qué hacer, están en [Instalar en el PC nuevo](Installing-on-the-New-PC#when-the-location-cannot-be-used).

<a id="before-or-during-the-install"></a>

### Antes o durante la instalación

| Lo que ves | Qué significa | Qué hacer |
| --- | --- | --- |
| "Puede que aquí no haya espacio suficiente" | Parece que la unidad no tiene espacio suficiente | Libera espacio o elige una ubicación en otra unidad. |
| "no hay espacio suficiente para esto, así que no se ha cambiado nada" | Una unidad en la que escribe está demasiado llena | Libera espacio ahí o elige una ubicación de instalación en otra unidad. |
| "Lo que esto haría aquí ha cambiado, así que no se ha instalado nada." | Algo ha cambiado en el PC desde que se calculó la lista | Vuelve a leer la lista y haz clic de nuevo en **Instalar** si es lo que quieres. |
| "Una instalación anterior de" una aplicación "no llegó a terminar. Instalar la deshace primero." | Una instalación anterior se interrumpió | Haz clic en **Instalar**. Primero la deshace. |
| "se está ejecutando otra instalación o desinstalación. Espera a que termine y vuelve a intentarlo" | Solo se ejecuta una a la vez | Espera y vuelve a intentarlo. |
| Un archivo "tiene un nombre que esta versión de Windows abre como dispositivo y no como archivo" | La aplicación tiene un archivo con un nombre como `aux.h`, que las versiones de Windows anteriores a Windows 11 no admiten | Instala en Windows 11. |
| "No se ha podido crear una carpeta de preparación" | No hay ningún lugar privado, con espacio, donde desempaquetar el paquete | Libera espacio en la unidad de la ubicación de instalación o en la unidad del sistema. |
| Se indica que una carpeta de datos como `%AppData%` es un vínculo | El instalador no escribe a través de un [vínculo](Glossary#link) | Desmarca **Datos de usuario** y **Licencia**, e instala sin ellos. |
| "No se ha encontrado la carpeta ProgramData de este PC, así que no se ha instalado nada: una instalación guarda ahí el historial que permite deshacerla." | No se ha podido leer el dato propio de Windows que indica dónde está ProgramData | El instalador no instala sin un lugar seguro donde guardar su historial. |

<a id="the-result"></a>

### El resultado

| Lo que ves | Qué hacer |
| --- | --- |
| "La instalación ha fallado y se ha revertido lo que cambió." | Lee los mensajes en rojo para saber por qué. El PC está como estaba, excepto los entornos de ejecución que haya instalado. |
| "La instalación ha fallado y no se ha podido revertir todo lo que cambió." | Lee lo que aparece en la lista. Cierra lo que pueda estar usando esos archivos y vuelve a ejecutar un instalador: primero vuelve a intentar revertirlo. |
| "No se ha podido instalar" un entorno de ejecución | No se ha podido instalar el entorno de ejecución, así que se ha revertido la instalación. Para el DirectX End-User Runtime, el PC nuevo necesita internet. Consulta [Entornos de ejecución](Runtimes#with-no-internet-on-the-new-pc). |
| "Una dependencia necesita un reinicio para terminar. Reinicia Windows y, después, abre la aplicación." | Reinicia y, después, inicia la aplicación. |
| El servicio "se ejecutaba como" una cuenta "en el PC de origen. Su contraseña no está en el instalador, así que el servicio se configura como LocalService." | Abre Servicios y establece la cuenta y la contraseña del servicio, si no funciona tal como está. |
| Algunos archivos van "en su lugar dentro de" la carpeta de instalación, "en una carpeta llamada _external" | Este PC tiene Documentos o Juegos guardados fuera de tu perfil. Mueve tú esos archivos a esa carpeta. |
| "Se ha excluido" algo, con un motivo | Las reglas lo han rechazado, o aquí ya pertenece a otra aplicación. Lo demás se ha instalado. Si la aplicación lo necesita, consulta el motivo. |
| **Hashes del paquete NO VÁLIDOS** o **Archivos instalados NO VÁLIDOS** | Algo no coincide. Vuelve a copiar el instalador e inténtalo una vez más. |

<a id="after-installing"></a>

### Después de instalar

| Lo que ves | Qué hacer |
| --- | --- |
| La aplicación no se inicia | Busca en **Dependencias** un entorno de ejecución marcado como **Instalar manualmente** e instálalo. Revisa las advertencias del resultado. |
| Una aplicación de Windows 7 no se inicia, y al crear el instalador una advertencia decía de una DLL que "Windows 10 y 11 no la incluyen" | Esa DLL la colocó otra cosa en las carpetas propias del Windows antiguo, así que no se ha incluido. Instala lo que la proporcione y vuelve a iniciar la aplicación. Consulta [Aplicaciones de Windows 7, Vista, 8 u 8.1](Moving-from-a-Connected-Drive#apps-from-windows-7-vista-8-or-81). |
| La aplicación se inicia como si fuera nueva | Es lo esperado con un instalador creado con **Datos de usuario** desactivado. También ocurre cuando se desmarcó **Datos de usuario** al instalar, porque entonces los datos van dentro de la carpeta de instalación, y cuando el instalador no pudo saber quién había iniciado sesión, cosa que indica. |
| La aplicación pide que se active | Hay que volver a activar una licencia vinculada al PC antiguo. |
| El actualizador propio de la aplicación cree que no está instalada | Windows Installer no tiene aquí constancia de ella. Consulta [Limitaciones](Limitations). |
| Un tipo de archivo o tipo de vínculo no se abre con la aplicación | Aquí ya era de otra aplicación, así que se ha dejado como estaba. Elige la aplicación en Configuración, en Aplicaciones predeterminadas. |
| No se ve una nueva entrada de `Path` | Abre una nueva ventana de PowerShell o del símbolo del sistema. Algunos programas solo ven un cambio después de cerrar la sesión y volver a iniciarla. |

<a id="uninstalling"></a>

## Desinstalar

| Lo que ves | Qué hacer |
| --- | --- |
| "Una parte está en una unidad que no está conectada." | Conecta esa unidad y vuelve a desinstalar desde Configuración. |
| "No se ha podido deshacer todo. Cierra lo que lo esté usando y vuelve a desinstalar desde Configuración." | Cierra la aplicación y cualquier cosa que use sus archivos. Si un servicio no se detiene, reinicia el PC. Después, vuelve a desinstalar. |
| "este no es el id. de una instalación" (desde la línea de comandos, "this is not an install's id") | El id. que se ha dado a `--uninstall` es incorrecto. Cópialo de la entrada de Configuración. Consulta [Línea de comandos](Command-Line#finding-the-id). |
| Ha aparecido un archivo llamado `.before-install` | Es el original de un archivo que reemplazó la instalación y que tú habías cambiado desde entonces. Quédate con el que prefieras. |
| Un entorno de ejecución sigue instalado | Los entornos de ejecución se quedan, porque otros programas pueden usarlos. Déjalo, a menos que estés seguro de que nada más lo usa. |

<a id="scripts"></a>

## Scripts

| Lo que ves | Qué hacer |
| --- | --- |
| "La operación solicitada requiere elevación" | Ejecuta PowerShell como administrador. |
| No se muestra nada y `$LASTEXITCODE` no se establece | Canaliza la salida del instalador, por ejemplo a `Out-String`. Consulta [Línea de comandos](Command-Line). |
| Código de salida `2` | Una opción es incorrecta, o se han usado `--apply` y `--dry-run` a la vez. |
| "this package does not say where its app was, so give a --target-root" (el paquete no indica dónde estaba su aplicación) | Agrega `--target-root` con una ruta de acceso completa. |
| La aplicación "was in" una carpeta, "and that cannot be used" (estaba en una carpeta que no se puede usar) | Agrega `--target-root` con otra carpeta. |
| Una carpeta "cannot be used: it already has files" "Give --force only if you know it is this app's" (ya contiene archivos; usa --force solo si sabes que es de esta aplicación) | Elige una carpeta vacía. Usa `--force` solo con una carpeta que sepas que es de esta aplicación. |

<a id="language-and-text"></a>

## Idioma y texto

- **El idioma equivocado.** Elige otro en **Idioma**, en la parte superior derecha de la aplicación o de la primera pantalla de un instalador. La aplicación lo recuerda; un instalador lo mantiene durante esa ejecución. Si no se ha elegido ninguno, cada uno usa el primero de los idiomas de presentación de Windows que tenga, o si no, inglés.
- **Una traducción se lee mal.** Todo, salvo el inglés, se ha traducido automáticamente. En **Idioma**, elige **Notificar un problema de traducción**, que abre un formulario con tu idioma ya rellenado. Indica qué muestra la pantalla y qué debería decir.
- **Las letras aparecen como cuadros vacíos.** La ventana usa las fuentes que incluye Windows para cada sistema de escritura, de `C:\Windows\Fonts`. Un Windows al que se le hayan quitado algunas dibuja esas letras como cuadros; los nombres de otras aplicaciones de la lista también pueden necesitarlas.
- **Las palabras aparecen cortadas.** Una etiqueta que no cabe termina en `…`; coloca el puntero sobre ella, o ve a ella con la tecla Tab, para verla entera. Notifícalo indicando el idioma y la pantalla, para que se pueda ajustar la ventana.
- **WinAppMigrator Client muestra el texto en inglés en un Windows de derecha a izquierda.** La consola en la que se ejecuta no puede unir ni ordenar las letras árabes o hebreas, así que muestra el texto en inglés e indica por qué. `--lang` elige cualquier idioma que tenga.

<a id="getting-help"></a>

## Obtener ayuda

1. Busca el [archivo de registro](Glossary#log). Su ruta de acceso aparece como **Archivo de registro** cuando algo falla, o se muestra como `Log:` desde la línea de comandos. Los archivos de registro están en `%ProgramData%\WinAppMigrator\logs`. Contienen rutas de acceso y nombres de claves, y nunca el contenido de un valor de configuración, un archivo de licencia o una variable de entorno. Aun así, pueden contener los nombres de tus carpetas, así que lee uno antes de compartirlo.
2. [Abre una incidencia](https://github.com/RandyNorthrup/WinAppMigrator/issues) con la versión, lo que hiciste y lo que decía la pantalla. **Copiar error**, en la pantalla final de la aplicación, copia el mensaje.
3. **Nunca adjuntes un instalador a una incidencia.** Puede contener datos privados.
4. Si se trata de un problema de seguridad, notifícalo en privado, a través de los [avisos de seguridad de GitHub](https://github.com/RandyNorthrup/WinAppMigrator/security/advisories/new).
