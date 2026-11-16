# Términos de es-ES (español de España)

Tratamiento: **tú**, como Windows 10 y Windows 11 en español ("¿Quieres permitir que esta aplicación haga cambios en el
dispositivo?"). Mayúsculas: solo la primera palabra de títulos, etiquetas y botones. Comillas: rectas ("..."), como en
los mensajes de Windows en español. "PC" es masculino: el PC, este PC. Lo que acaba de pasar va en pretérito perfecto
("No se ha podido...", "se ha instalado"), como en España. "Registro" con mayúscula es siempre el Registro de Windows;
"registro" en minúscula solo aparece en "archivo de registro" y "registro de eventos".

## Palabras de Windows

| English | es-ES |
| --- | --- |
| Settings | Configuración |
| Apps, Installed apps | Aplicaciones, Aplicaciones instaladas |
| Accessibility | Accesibilidad |
| Text size, Animation effects, Contrast themes | Tamaño del texto, Efectos de animación, Temas de contraste |
| Explorer (File Explorer) | Explorador de archivos |
| administrator rights | permisos de administrador |
| administrator | administrador |
| administrator's command prompt | símbolo del sistema como administrador |
| User Account Control | Control de cuentas de usuario |
| service, Services (console) | servicio, Servicios |
| disabled (a service's start type) | deshabilitado |
| scheduled task, Task Scheduler | tarea programada, Programador de tareas |
| shortcut | acceso directo |
| registry, registry key, registry value | Registro, clave del Registro, valor del Registro |
| Registry Editor, (Default) | Editor del Registro, (Predeterminado) |
| firewall rule, Windows Firewall | regla de firewall, Firewall de Windows |
| firewall profiles: domain, private, public | perfiles Dominio, Privado, Público (como en Opciones avanzadas de una regla del Firewall de Windows Defender) |
| network share; a shared folder on your network | recurso compartido de red; una carpeta compartida de tu red |
| Documents, Saved Games, Public Documents | Documentos, Juegos guardados, Documentos públicos |
| Startup folder, Start Menu, Desktop | carpeta Inicio, menú Inicio, escritorio |
| Default apps | Aplicaciones predeterminadas |
| Windows feature, Windows Features | característica de Windows, Características de Windows |
| environment variable | variable de entorno |
| user / machine (variable) | de usuario / del sistema |
| event log, Security log | registro de eventos, registro de seguridad |
| shell extension | extensión del shell |
| driver | controlador |
| Notepad | Bloc de notas |
| Run (dialog) | Ejecutar |
| Unknown publisher | Editor desconocido |
| publisher | editor |
| signed in, sign in | ha iniciado sesión, iniciar sesión |
| restart, reboot | reiniciar, reinicio |
| Light / Dark theme | tema claro / tema oscuro |
| snapshot (of a drive) | instantánea |
| link (symbolic link, junction) | vínculo |
| Browse | Examinar |
| open / closed (a heading, as Narrator says it) | expandido / contraído |
| Enter (key), display language | Entrar, idioma de presentación de Windows |

## Palabras de WinAppMigrator

| English | es-ES |
| --- | --- |
| installer | instalador |
| installer stub | programa base del instalador |
| package | paquete |
| Install Location | Ubicación de instalación |
| User Data | Datos de usuario |
| License; License and Activation Data | Licencia; Datos de licencia y activación |
| Preview Changes | Vista previa de los cambios |
| uninstall, uninstaller | desinstalar, desinstalación, desinstalador |
| runtime | entorno de ejecución |
| prerequisite | requisito previo |
| the old PC, the new PC, the other PC | el PC antiguo, el PC nuevo, el otro PC |
| Network PC | PC en red |
| pairing code, pair | código de emparejamiento, emparejar |
| WinAppMigrator Client; the client | WinAppMigrator Client; el cliente |

## Otras palabras repetidas

| English | es-ES |
| --- | --- |
| app, application | aplicación |
| program | programa |
| folder | carpeta |
| drive, connected drive, USB drive | unidad, unidad conectada, unidad USB |
| disk image | imagen de disco |
| scan, look at (an app), Look Again | análisis, analizar, Volver a analizar |
| Scan Source | Origen del análisis |
| build (an installer), Create Installer | crear, creación, Crear instalador |
| preview | vista previa |
| restore | restaurar |
| put back (what an install changed) | revertir (cambios), restablecer (un elemento) |
| undo | deshacer |
| left out | excluido, excluir |
| settings | configuración; valor de configuración (si se cuentan) |
| warning, error | advertencia, error |
| record of the install (WinAppMigrator's own, for undoing it) | historial de la instalación |
| install record (what an app's own installer recorded) | información de instalación |
| log (file) | archivo de registro |
| move (an app to the new PC) | mover |
| remove; delete (a snapshot) | quitar; eliminar |
| stays | se queda |
| hive; subtree | subárbol; rama |
| output folder, staging folder | carpeta de salida, carpeta de preparación |
| hash check | comprobación de hash |
| self-extracting | autoextraíble |
| credential, certificate | credencial, certificado |
| dependency | dependencia |
| account, profile | cuenta, perfil |
| room (free space) | espacio |
| What Ties It into Windows | Qué la integra en Windows |

## Notas de estructura

- `core-review-start-*` llevan su propio verbo ("se inicia con Windows...", "no se inicia nunca..."), porque "se inicia
  nunca" no es correcto; por eso `core-review-service-detail` (`[other]`) y `core-review-driver-detail-starts` terminan
  en `{ $start }` sin "se inicia" delante.
- `core-review-firewall-detail` dice "en el perfil { $networks }" / "en los perfiles { $networks }", con los nombres de
  perfil de `core-review-network-*`.
