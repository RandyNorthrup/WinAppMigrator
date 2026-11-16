# Words the core says about what an install may do: why an install location or something in a package is refused,
# the installer's list of what an app sets up beyond its files, the checks of a package and of an install, and whether
# there is room for either. Message names start with core-.
#
# Many messages here are a reason, the second half of a sentence that starts "... was left out:" or "that install
# location cannot be used:", so they start with a small letter and have no full stop.


## Why an install location cannot be used. Shown in red under the Install Location field.

core-review-location-state-root = c'est l'emplacement où les installations conservent leurs enregistrements
core-review-location-has-files = il contient déjà des fichiers, et aucune installation n'y est enregistrée : utilisez un dossier vide
core-policy-location-not-full-path = il doit s'agir d'un chemin d'accès complet
core-policy-location-parent-step = il ne doit pas contenir « .. »
core-policy-location-not-plain = il doit s'agir d'un chemin simple sur un lecteur de ce PC
core-policy-location-whole-drive = il ne peut pas s'agir d'un lecteur entier
core-policy-location-windows = il ne peut pas s'agir du dossier Windows
core-policy-location-runs-by-itself = il s'agit d'un dossier qui appartient à Windows, ou dont le contenu s'exécute ou se charge de lui-même
core-policy-location-shared = il contient d'autres éléments côte à côte, l'application a donc besoin d'un dossier à l'intérieur

## Whether an app can run on this PC at all, said before anything is installed.

# { $reason } is why, as the second half of a sentence, such as "App.exe is built for Arm64, and this PC is x64, so
# Windows would not start it".
core-review-cannot-run-here = cette application ne peut pas s'exécuter sur ce PC, rien n'a donc été modifié : { $reason }
core-policy-architecture-x86 = x86 32 bits
core-policy-architecture-x64 = x64
core-policy-architecture-arm64 = Arm64
core-policy-architecture-arm32 = Arm 32 bits
core-policy-architecture-unknown = une architecture inconnue
# { $code } is the number Windows gives a kind of processor, such as 0x01c4.
core-policy-machine-type = type de machine { $code }
# { $names } is a list of program names, such as "a.exe, b.exe, c.exe", and { $count } how many more there are.
core-policy-names-and-more =
    { $names } et { $count ->
        [one] { $count } autre
       *[other] { $count } autres
    }
core-policy-fit-programs-not-said = Ce programme d'installation n'indique pas ce dont ses programmes ont besoin, leur capacité à s'exécuter sur ce PC n'a donc pas été vérifiée.
core-policy-fit-architecture-not-told = L'architecture de ce PC n'a pas pu être déterminée, la capacité des programmes de cette application à s'y exécuter n'a donc pas été vérifiée.
core-policy-fit-main-not-said = Ce programme d'installation n'indique pas ce dont son programme principal a besoin, les informations présentées ici reposent donc sur le reste de son contenu.
# { $driver } is a driver's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-driver = son pilote { $driver } est conçu pour { $built_for }, et ce PC est de type { $here }. Un pilote doit correspondre exactement à Windows, cette application ne peut donc pas s'exécuter ici
# { $program } is a program's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-program = { $program } est conçu pour { $built_for }, et ce PC est de type { $here }, Windows ne le démarrerait donc pas
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-none-start = aucun de ses programmes ne peut démarrer sur ce PC, qui est de type { $here } : { $programs }
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-some-other-architecture = Certains programmes de cette application sont conçus pour une autre architecture et ne démarreront pas sur ce PC, qui est de type { $here } : { $programs }. Le reste de l'application peut démarrer, ce n'est donc peut-être pas un problème.
# { $here } is a kind of processor, such as "Arm64". { $programs } is a list of program names.
core-policy-fit-emulated = Ce PC est de type { $here }, Windows exécuterait donc ces programmes par émulation : { $programs }. La plupart des programmes fonctionnent bien ainsi, mais pas tous.
# { $program } is a program's file name. { $needs } and { $runs } are versions of Windows, such as 10.0 and 6.1.
core-policy-fit-needs-newer = { $program } indique qu'il nécessite Windows { $needs } ou une version ultérieure, et ce PC exécute { $runs }, Windows ne le démarrerait donc pas
# { $programs } is a list of program names.
core-policy-fit-some-need-newer = Certains programmes de cette application indiquent qu'ils nécessitent une version de Windows plus récente que celle-ci et ne démarreront pas ici : { $programs }.
# { $from_build } and { $this_build } are Windows build numbers, such as 22631, written as Windows writes them.
core-policy-fit-older-build = Cette application se trouvait sur la build { $from_build } de Windows, et ce PC exécute la build { $this_build }. Ses programmes indiquent qu'ils peuvent démarrer sur cette version de Windows, elle peut donc fonctionner, mais ce dont elle dépendait là-bas n'est peut-être pas présent ici.
core-policy-fit-build-not-told = La build de Windows de ce PC n'a pas pu être déterminée, elle n'a donc pas été comparée à celle sur laquelle se trouvait cette application.

## Room on a drive for a build or an install.

# Each of these ends the sentence core-space-shortfall, after "free for".
core-space-for-unpacking = la décompression du package
core-space-for-install-record = l'enregistrement de l'installation
core-space-for-app-files = les fichiers de l'application
core-space-for-what-is-replaced = la conservation de ce qu'elle remplace
core-space-for-build-work = les fichiers de travail de la génération
core-space-for-installer = le programme d'installation
core-space-for-package = le package
core-space-for-unpacking-and-installing = la décompression du package et son installation
# What goes between two of those, as in "unpacking the package and the app's files". Keep the quotes: they keep the
# spaces.
core-space-purposes-joiner = { " et " }
# { $drive } is a drive, such as C:, or a folder. { $needed } and { $free } are sizes, such as 1.2 GB. { $purposes } is
# what the room is for, such as "unpacking the package and the app's files".
core-space-shortfall = { $drive } nécessite jusqu'à { $needed } d'espace libre pour { $purposes }, et dispose de { $free }
# { $drive } is a drive, such as C:, or a folder. { $needed } is a size, such as 1.2 GB.
core-space-not-told = { $drive } n'a pas indiqué son espace disponible, la place pour les { $needed } nécessaires n'a donc pas été vérifiée
# One drive in a list of where room is needed. { $needed } and { $free } are sizes, and { $drive } a drive, such as C:.
core-space-summary = jusqu'à { $needed } sur { $drive } ({ $free } libres)
# { $needed } is a size, and { $drive } a drive, such as C:.
core-space-summary-not-told = jusqu'à { $needed } sur { $drive } (espace libre inconnu)
# What goes between two sentences of core-space-shortfall inside core-review-no-room. Keep the quotes: they keep the
# space.
core-review-sentence-joiner = { ". " }
# { $shortfalls } is one or more sentences of core-space-shortfall.
core-review-no-room = l'espace est insuffisant pour cette opération, rien n'a donc été modifié. { $shortfalls }. Libérez de l'espace à cet endroit, ou choisissez un emplacement d'installation sur un autre lecteur.

## The installer's list of what an app sets up beyond its files: what each kind is called.

core-review-kind-service = Service
core-review-kind-driver = Pilote
core-review-kind-scheduled-task = Tâche planifiée
core-review-kind-autostart = Démarre automatiquement
core-review-kind-shortcut = Raccourci
core-review-kind-path = PATH
core-review-kind-environment-variable = Variable d'environnement
core-review-kind-shell-extension = Extension de shell
core-review-kind-menu-command = Commande de menu
core-review-kind-link-type = Type de lien
core-review-kind-file-type = Type de fichier
core-review-kind-com = COM
core-review-kind-firewall-rule = Règle de pare-feu
core-review-kind-event-log-source = Source du journal des événements
core-review-kind-font = Police
core-review-kind-odbc-driver = Pilote ODBC
core-review-kind-odbc-source = Source de données ODBC
core-review-kind-compatibility-setting = Paramètre de compatibilité
core-review-kind-default-apps = Applications par défaut
core-review-kind-program-name = Nom du programme
core-review-kind-other-settings = Autres paramètres

## The installer's list of what an app sets up: what each line says of it.

# { $file } is a program's file name. { $account } is the Windows account it runs as, such as LocalSystem. { $starts }
# says when it starts, and is not shown itself; for "other", { $start } is one of the core-review-start- words below.
core-review-service-detail =
    { $starts ->
        [automatic] Exécute { $file } en tant que { $account }, démarre maintenant et à chaque démarrage de ce PC
        [delayed] Exécute { $file } en tant que { $account }, démarre maintenant et peu après chaque démarrage de ce PC
        [manual] Exécute { $file } en tant que { $account }, démarre uniquement à la demande
        [disabled] Exécute { $file } en tant que { $account }, est désactivé
        [other] Exécute { $file } en tant que { $account }, { $start }
       *[unknown] Exécute { $file } en tant que { $account }
    }
# { $account } is a Windows account, such as LocalService.
core-policy-account-custom = { $account } (compte personnalisé non restauré)
# { $file } is a driver's file name.
core-review-driver-detail = Charge { $file } dans Windows même
# { $file } is a driver's file name. { $start } is when Windows starts it: one of the core-review-start- words below.
core-review-driver-detail-starts = Charge { $file } dans Windows même, { $start }
# When a service or driver starts, ending a sentence such as "Loads disk.sys into Windows itself, starts …".
core-review-start-boot = démarre avec Windows, avant tout le reste
core-review-start-system = démarre avec Windows, pendant son lancement
core-review-start-automatic = se lance à chaque démarrage de ce PC
core-review-start-delayed = se lance peu après chaque démarrage de ce PC
core-review-start-manual = démarre uniquement à la demande
core-review-start-disabled = ne démarre jamais, car il est désactivé
core-review-start-unknown = démarre d'une manière que Windows n'indique pas
# { $account } is the account a task runs as: an account's name, or core-review-task-runs-as-you, or
# core-review-task-runs-as-creator.
core-review-task-detail = S'exécute en tant que { $account }
# { $account } is the account a task runs as, as for core-review-task-detail.
core-review-task-detail-highest = S'exécute en tant que { $account }, avec les autorisations maximales
core-review-task-runs-as-you = l'utilisateur connecté (vous)
core-review-task-runs-as-creator = l'utilisateur qui l'a créée
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-autostart-detail =
    S'exécute { $scope ->
        [everyone] pour toute personne qui se connecte
       *[you] quand vous vous connectez
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shortcut-startup-detail =
    Un raccourci dans le dossier Démarrage, { $scope ->
        [everyone] pour tous les utilisateurs de ce PC
       *[you] pour vous
    }
# { $file } is the file name of what the shortcut opens. { $scope } is whom it is set up for: everyone on this PC,
# or you. It is not shown itself.
core-review-shortcut-detail =
    Ouvre { $file }, { $scope ->
        [everyone] pour tous les utilisateurs de ce PC
       *[you] pour vous
    }
# { $name } is an environment variable's name, such as PATH.
core-review-path-title = Ajoute à { $name }
core-review-path-install-folder = Le dossier d'installation
# { $path } is a folder inside the install folder, such as bin\tools.
core-review-path-below-install-folder = Le dossier d'installation\{ $path }
# { $value } is the variable's value, as it will be set.
core-review-variable-detail = Définie sur { $value }
# { $count } is how many registry values.
core-review-other-settings-title =
    { $count ->
        [one] { $count } paramètre hors des clés propres à l'application
       *[other] { $count } paramètres hors des clés propres à l'application
    }
# { $keys } is a list of registry keys, such as HKCU\Software\Vendor.
core-review-other-settings-detail = Dans { $keys }
# { $keys } is a list of registry keys, and { $count } how many more there are.
core-review-other-settings-detail-more =
    Dans { $keys } et { $count ->
        [one] { $count } autre
       *[other] { $count } autres
    }
# { $way } says whether the rule allows or blocks, and incoming or outgoing connections; it is not shown itself.
# { $ports } is a list of port numbers, and { $port_count } how many. { $program } is the program's file name.
# { $networks } is a list of the kinds of network Windows names in English (domain, private, public), and
# { $network_count } how many.
# The kinds of network a firewall rule is for, as Windows Defender Firewall names its profiles in your language. They
# fill { $networks } in core-review-firewall-detail, joined into a list.
core-review-network-domain = domaine
core-review-network-private = privé
core-review-network-public = public
core-review-firewall-detail =
    { $way ->
        [allow-in] Autorise les connexions entrantes
        [allow-out] Autorise les connexions sortantes
        [block-in] Bloque les connexions entrantes
       *[block-out] Bloque les connexions sortantes
    }{ $port_count ->
        [0] { "" }
        [one] { " " }sur le port { $ports }
       *[other] { " " }sur les ports { $ports }
    } pour { $program }, sur { $network_count ->
        [0] tous les réseaux
       *[other] les réseaux de type { $networks }
    }
# { $server } is the file name of what Explorer loads. { $scope } is whom it is set up for: everyone on this PC, or
# you. It is not shown itself.
core-review-shell-extension-detail =
    Charge { $server } dans l'Explorateur de fichiers, { $scope ->
        [everyone] pour tous les utilisateurs de ce PC
       *[you] pour vous
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shell-extension-detail-own-file =
    Charge dans l'Explorateur de fichiers un fichier qu'elle installe, { $scope ->
        [everyone] pour tous les utilisateurs de ce PC
       *[you] pour vous
    }
# The name of a shell extension: { $handler } is its name in the registry, and { $class } the kind of file or folder it
# is for, such as * or Directory.
core-policy-shell-extension-name = { $handler } sur { $class }
# { $on } is the kind of file or folder whose menu it is, as the registry names it, such as * or
# Directory. { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-menu-command-detail =
    Ajoute une commande au menu de { $on }, { $scope ->
        [everyone] pour tous les utilisateurs de ce PC
       *[you] pour vous
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-default-verb-detail =
    Modifie l'action effectuée à l'ouverture de ces éléments, { $scope ->
        [everyone] pour tous les utilisateurs de ce PC
       *[you] pour vous
    }
core-review-client-entry-detail = Peut être choisi comme programme pour cet usage. Aucune modification n'y est apportée
core-review-com-permissions-title = Autorisations COM
core-review-com-permissions-detail = Indique qui peut démarrer ses classes COM, ou sous quel compte elles s'exécutent
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-link-type-detail =
    Les liens de ce type s'ouvrent avec l'application, { $scope ->
        [everyone] pour tous les utilisateurs de ce PC
       *[you] pour vous
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-file-type-detail =
    Les fichiers de ce type s'ouvrent avec l'application, { $scope ->
        [everyone] pour tous les utilisateurs de ce PC
       *[you] pour vous
    }
# { $log } is the name of a Windows event log, such as Application.
core-review-event-log-source-detail = Peut écrire dans le journal { $log }, avec des messages provenant de ses propres fichiers
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-font-detail =
    Se charge depuis le dossier d'installation quand vous vous connectez, { $scope ->
        [everyone] pour tous les utilisateurs de ce PC
       *[you] pour vous
    }
core-review-odbc-driver-detail = Se charge dans les programmes qui utilisent ODBC pour accéder à une base de données
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-odbc-source-detail =
    Une connexion à une base de données, { $scope ->
        [everyone] pour tous les utilisateurs de ce PC
       *[you] pour vous
    }
core-review-default-apps-detail = Peut être choisie dans Applications par défaut. Aucune modification n'y est apportée
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-program-name-detail =
    Exécuter et d'autres programmes peuvent la trouver sous ce nom, { $scope ->
        [everyone] pour tous les utilisateurs de ce PC
       *[you] pour vous
    }
# { $server } is the file name of what serves the classes.
core-review-com-title = COM dans { $server }
core-review-com-title-own-file = COM dans un fichier qu'elle installe
# { $count } is how many registry values.
core-review-com-detail =
    { $count ->
        [one] { $count } paramètre des classes que d'autres programmes peuvent démarrer
       *[other] { $count } paramètres des classes que d'autres programmes peuvent démarrer
    }
core-review-com-support-title = Bibliothèques de types, interfaces et AppID COM
# { $libraries }, { $interfaces }, and { $app_ids } are how many registry values there are for its type libraries, its
# interfaces, and its AppIDs.
core-review-com-support-detail = Paramètres de ses bibliothèques de types ({ $libraries }), de ses interfaces ({ $interfaces }) et de ses AppID ({ $app_ids }), qui accompagnent ses classes
core-review-compatibility-title = Mode de démarrage de ses programmes
# { $count } is how many registry values.
core-review-compatibility-detail =
    { $count ->
        [one] { $count } paramètre de compatibilité pour les programmes qu'elle installe
       *[other] { $count } paramètres de compatibilité pour les programmes qu'elle installe
    }

## The installer's notes on whose folders and settings these are, and what is not coming.

# { $account } is the name of a Windows account, such as alice.
core-review-account-signed-in = Les données utilisateur et les paramètres par utilisateur vont à { $account }, qui est connecté ici, et non à l'administrateur sous le compte duquel ceci s'exécute.
# { $account } is the name of a Windows account, such as alice.
core-review-account-no-desktop = Personne n'est connecté à cette session, les données utilisateur et les paramètres par utilisateur vont donc à { $account }, le compte sous lequel ceci s'exécute.
# { $reason } is what Windows said, in its own words.
core-review-account-not-told = Impossible de déterminer qui est connecté ({ $reason }), les paramètres par utilisateur sont donc exclus, et les données utilisateur vont sous l'emplacement d'installation.
# { $reason } is what Windows said, in its own words.
core-review-signed-in-unknown = impossible de déterminer qui est connecté : { $reason }
# Headings of what is left out, shared by the app's list and the installer's notes.
core-review-left-out-credentials = Non inclus dans ce programme d'installation, car ils ressemblent à des informations d'identification
core-review-left-out-settings = Paramètres et historique qu'un compte conserve pour l'application, inclus uniquement avec l'option Données utilisateur
core-review-left-out-certificates = L'application semble dépendre de certificats approuvés sur son PC, et ils ne sont pas installés
core-review-left-out-fonts = Ces polices portent le nom de l'application, mais rien n'indique qu'elles lui appartiennent, elles ne sont donc pas installées
# { $names } is a list of names, such as the names of saved passwords.
core-review-note-credentials = Non inclus dans ce programme d'installation, car ils ressemblent à des informations d'identification : { $names }. Ce qui en existe sur ce PC reste tel quel, et vous devrez peut-être les saisir à nouveau.
# { $names } is a list of names of license data.
core-review-note-licenses = Non inclus dans ce programme d'installation, car l'option Données de licence et d'activation était désactivée lors de sa génération : { $names }.
# { $names } is a list of places an account keeps settings, such as folders.
core-review-note-settings = Non inclus dans ce programme d'installation, car l'option Données utilisateur était désactivée lors de sa génération : les paramètres et l'historique qu'un compte conserve pour l'application, dans { $names }. L'application démarre ici comme pour un nouvel utilisateur.
# { $names } is a list of certificates, each as core-review-certificate.
core-review-note-certificates = L'application semble dépendre de certificats approuvés sur son PC, et ils ne sont pas installés : { $names }. C'est à vous de décider si vous leur faites confiance ici.
# { $names } is a list of font names.
core-review-note-fonts = Ces polices portent le nom de l'application, mais rien n'indique qu'elles lui appartiennent, elles ne sont donc pas installées : { $names }.
# { $names } is a list of names.
core-review-note-left-out-by-choice = Exclu par la personne qui a généré ce programme d'installation : { $names }.
# { $names } is a list of names.
core-review-note-added-by-choice = Ajouté par la personne qui a généré ce programme d'installation, car elle l'a considéré comme faisant partie de l'application : { $names }.
# { $name } is a certificate's subject, such as CN=Contoso, and { $store } the certificate store it was in, such as
# Root.
core-review-certificate = { $name } dans { $store }
# { $names } is a list of names, and { $count } how many more there are.
core-review-names-and-more =
    { $names } et { $count ->
        [one] { $count } autre
       *[other] { $count } autres
    }

## Why something in a package is left out. Each is the second half of a sentence of core-warning-review-left-out.

core-review-no-account = cet élément appartient aux paramètres d'un compte, et la personne connectée n'a pas pu être déterminée
core-review-name-not-accepted = Windows n'accepterait pas son nom
# { $error } is what Windows said, in its own words.
core-review-service-unreadable = ses paramètres actuels n'ont pas pu être lus : { $error }
core-review-service-dependency = il dépend d'un service dont le nom n'est pas valide
# { $path } is the program the service runs.
core-review-service-runs-other = il exécute { $path }, que ce package n'installe pas
core-review-service-settings = ses paramètres désignent un élément que ce package n'installe pas, ou une clé qui ne lui appartient pas
core-review-driver-no-hash = il n'a pas de hachage permettant de le vérifier
core-review-license-for-the-machine = une licence stockée pour l'ensemble de l'ordinateur est seulement signalée
core-review-license-starts-something = une licence n'a aucune raison de démarrer quoi que ce soit
core-review-task-not-captured = sa définition n'a pas été capturée
# { $error } is what was wrong with the task's XML, in English.
core-review-task-unreadable = sa définition n'a pas pu être lue : { $error }
core-review-task-no-account = elle s'exécute en tant que la personne qui l'installe, et la personne connectée ici n'a pas pu être déterminée
# { $path } is what the shortcut opens.
core-review-shortcut-opens-other = il ouvre { $path }, que ce package n'installe pas
# { $service } is the service's name, and { $path } the program it runs now.
core-policy-service-exists = { $service } existe déjà sur ce PC et exécute { $path }, qui ne provient pas d'une installation antérieure effectuée ici. S'il s'agit d'une ancienne copie de cette application, désinstallez-la d'abord
core-policy-key-control-characters = le nom de sa clé contient des caractères de contrôle
core-policy-value-control-characters = le nom de sa valeur contient des caractères de contrôle
core-policy-registry-link = il s'agit d'un lien de Registre
# { $reason } is why the key it stands for is left out, such as "no app install should change this key".
core-policy-virtual-store = il représente une clé de l'ordinateur, et { $reason }
core-policy-registry-part = aucune installation d'application ne devrait écrire dans cette partie du Registre
core-policy-key-refused = aucune installation d'application ne devrait modifier cette clé
core-policy-net-debugger = il désignerait un débogueur pour chaque programme .NET
core-policy-net-install-root = il modifierait l'emplacement à partir duquel chaque programme .NET charge .NET
core-policy-class-redirects = il modifierait la façon dont un type de programme ou de dossier est ouvert
core-policy-app-path-name = il désigne un programme par autre chose qu'un nom de fichier
core-policy-app-path-not-installed = il inscrirait un nom de programme pour un fichier que ce package n'installe pas
core-policy-shared-default-verb = il modifierait l'action effectuée à l'ouverture de tous les fichiers ou dossiers d'un type
core-policy-command-runs-other = il ajoute une commande qui exécute un élément que ce package n'installe pas
core-policy-autostart-not-command = il démarre automatiquement, et sa valeur n'est pas une commande
core-policy-autostart-runs-other = il démarre automatiquement, et exécute un élément que ce package n'installe pas
core-policy-environment-own-rules = les variables d'environnement sont restaurées selon leurs propres règles
core-policy-service-not-restored = il appartient à un service que ce package ne restaure pas
core-policy-service-own-rules = les paramètres propres à un service ne sont restaurés que selon ses règles
core-policy-service-dll = sa valeur ServiceDll n'est pas un fichier que ce package installe
core-policy-event-log-source-only = seules les valeurs propres à une source du journal des événements peuvent être restaurées
core-policy-security-log = aucune application n'écrit dans le journal Sécurité
core-policy-event-log-messages = ses messages proviendraient d'un fichier que ce package n'installe pas
core-policy-type-library = sa bibliothèque de types n'est pas un fichier que ce package installe
# { $name } is a program's file name, such as app.exe.
core-policy-application-opened = il modifierait la façon dont { $name } est ouvert, alors que ce package ne l'installe pas
core-policy-event-log-no-file = une source du journal des événements doit tirer ses messages d'un fichier
core-policy-com-runs-other = il exécute un élément qui n'est pas un fichier que ce package installe
core-policy-com-redirect = il ferait de cette classe un élément que ce package n'installe pas
core-policy-com-server = il exécute un élément qui n'est ni un fichier que ce package installe, ni une partie de Windows
core-policy-interface = il déterminerait la façon dont les appels à une interface sont effectués, avec un élément que ce package n'installe pas
# { $name } is a program's file name, such as app.exe.
core-policy-app-id-started = il modifierait la façon dont { $name } est démarré, alors que ce package ne l'installe pas
core-policy-app-id-surrogate = il serait hébergé par un programme que ce package n'installe pas
core-policy-app-id-service = il s'exécuterait dans un service que ce package ne restaure pas
core-policy-app-id-run-as = il s'exécuterait sous un compte qui lui est propre
core-policy-app-id-link = il ne désigne pas d'AppID
core-policy-app-id-remote = il s'exécuterait ailleurs que sur ce PC
core-policy-odbc-own-driver = il s'agit d'un paramètre propre à ODBC et non d'un pilote
core-policy-odbc-driver-not-installed = son pilote n'est pas un fichier que ce package installe
core-policy-loads-other-file = il chargerait un fichier que ce package n'installe pas
core-policy-odbc-own-source = il s'agit d'un paramètre propre à ODBC et non d'une source de données
core-policy-odbc-source-missing = il répertorie une source de données que ce package ne contient pas
core-policy-odbc-source-driver = son pilote n'est ni un fichier que ce package installe, ni une partie de Windows
core-policy-shell-extension = il chargerait dans l'Explorateur de fichiers une extension de shell qui n'est pas un fichier que ce package installe
core-policy-not-text = son contenu n'est pas du texte, alors qu'il serait lu comme tel
core-policy-network-place = il désigne un emplacement sur le réseau
core-policy-font = il inscrirait une police que ce package n'installe pas, ou une police du dossier Windows
core-policy-compatibility = il modifierait le mode d'exécution d'un programme que ce package n'installe pas
core-policy-capabilities = il répertorie une application dont les détails ne figurent pas dans ce package
core-policy-default-program = il modifierait le programme qui ouvre les éléments par défaut
core-policy-url-protocol = il se présente comme un type de lien, et ne s'ouvre avec rien de ce que ce package installe
core-policy-command-handler = il ajoute une commande gérée par un élément que ce package n'installe pas
core-policy-installer-command = il exécuterait ce que seul Windows Installer peut lire
core-policy-creates-class = il créerait une classe que ce package n'installe pas
core-policy-opens-links = il ouvre les liens avec un élément que ce package n'installe pas
core-policy-opens-files = il ouvre les fichiers avec un élément que ce package n'installe pas
core-policy-firewall-name = son nom ne peut pas être transmis
core-policy-firewall-description = sa description ne peut pas être transmise
core-policy-firewall-every-program = elle s'applique à tous les programmes, alors qu'une règle ne peut concerner qu'un seul programme
# { $path } is the program the rule is for.
core-policy-firewall-program = elle concerne { $path }, que ce package n'installe pas
core-policy-firewall-direction = elle n'indique pas de direction qui puisse être définie
core-policy-firewall-action = elle passerait outre d'autres règles, ce qu'aucune règle d'application ne devrait faire
# { $setting } is a part of a firewall rule, as the scan of its PC described it.
core-policy-firewall-not-carried = elle indique { $setting }, qui n'a pas pu être défini tel quel, ou affaiblirait l'effet d'autres règles
core-policy-firewall-service = elle concerne un service que ce package ne restaure pas
core-policy-firewall-parts = une partie de son contenu ne correspond pas à ce qu'une règle de pare-feu peut indiquer
core-policy-folder-already-there = ce dossier existait déjà, en dehors de l'emplacement d'installation, et rien n'indique qu'il appartient à cette application, aucun programme n'y est donc placé
core-policy-variable-name = son nom n'est pas un nom de variable valide
core-policy-variable-relied-on = Windows et d'autres programmes dépendent de cette variable
core-policy-task-name = son nom n'est pas accepté par Windows
core-policy-task-folder = son dossier n'est pas accepté par Windows
core-policy-task-windows = elle remplacerait l'une des tâches propres à Windows
core-policy-task-other-action = elle fait autre chose qu'exécuter un programme
core-policy-task-no-program = elle ne désigne aucun programme à exécuter
# { $path } is the program the task runs.
core-policy-task-runs-other = elle exécute un élément que ce package n'installe pas : { $path }
core-policy-task-exists-unreadable = une tâche portant ce nom existe déjà et n'a pas pu être lue
core-policy-task-exists-other = une tâche portant ce nom existe déjà et n'exécute pas l'application
# { $path } is the program the task already on this PC runs.
core-policy-task-exists-runs = une tâche portant ce nom existe déjà et exécute { $path }, qui ne provient pas d'une installation antérieure effectuée ici
core-policy-shortcut-not-lnk = il ne s'agit pas d'un raccourci .lnk
core-policy-shortcut-place = il ne se trouve ni dans un menu Démarrer, ni sur un Bureau
core-policy-short-name = son chemin utilise un nom qui ne peut pas être vérifié
core-policy-shortcut-no-folder = ce PC n'a pas de dossier de ce type pour lui
core-policy-data-startup = cela irait dans un dossier Démarrage, et s'exécuterait à chaque connexion
core-policy-data-powershell = cela irait à un emplacement où PowerShell exécute ce qu'il trouve au démarrage
core-policy-data-windows = cela irait dans un dossier qui appartient à Windows
core-policy-license-outside = cela se trouve en dehors de l'emplacement d'installation et de tous les dossiers de données connus
core-rewrite-cdata = il contient une section CDATA
core-rewrite-too-much-to-look-through = ce package contient trop de paramètres pour que ce qu'ils désignent puisse être vérifié

## Warnings a review gives, which the installer shows with what it sets up.

# { $kind } says what was left out, and is not shown itself. { $item } is its path or name, or a registry value such
# as HKLM\Software\App\Name. { $reason } is why, as the second half of the sentence, such as "it cannot be a whole
# drive".
core-warning-review-left-out =
    { $kind ->
        [file] Le fichier { $item } a été exclu : { $reason }.
        [folder] Le dossier { $item } a été exclu : { $reason }.
        [service] Le service { $item } a été exclu : { $reason }.
        [driver] Le pilote { $item } a été exclu : { $reason }.
        [firewall-rule] La règle de pare-feu { $item } a été exclue : { $reason }.
        [license-data] Les données de licence situées dans { $item } ont été exclues : { $reason }.
        [license-value] La valeur de licence { $item } a été exclue : { $reason }.
        [scheduled-task] La tâche planifiée { $item } a été exclue : { $reason }.
        [shortcut] Le raccourci { $item } a été exclu : { $reason }.
        [environment-variable] La variable d'environnement { $item } a été exclue : { $reason }.
       *[value] { $item } a été exclu : { $reason }.
    }
core-warning-review-package-path-missing = Un fichier copiable n'a pas de chemin relatif dans le package.
# { $count } is the most empty folders a package makes.
core-warning-review-too-many-empty-folders = Ce package répertorie plus de { $count } dossiers vides, et seul ce nombre est créé.
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Documents folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-documents-elsewhere =
    { $count ->
        [one] { $count } fichier provenant de Documents va plutôt sous { $target }, dans un dossier nommé _external, car le dossier Documents de ce PC se trouve dans { $folder }, et l'écriture n'a lieu que dans un dossier situé dans votre profil. Pour l'utiliser, déplacez-le vous-même dans votre dossier Documents.
       *[other] { $count } fichiers provenant de Documents vont plutôt sous { $target }, dans un dossier nommé _external, car le dossier Documents de ce PC se trouve dans { $folder }, et l'écriture n'a lieu que dans un dossier situé dans votre profil. Pour les utiliser, déplacez-les vous-même dans votre dossier Documents.
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-documents-unknown =
    { $count ->
        [one] { $count } fichier provenant de Documents va plutôt sous { $target }, dans un dossier nommé _external, car l'emplacement du dossier Documents de ce PC n'a pas pu être déterminé. Pour l'utiliser, déplacez-le vous-même dans votre dossier Documents.
       *[other] { $count } fichiers provenant de Documents vont plutôt sous { $target }, dans un dossier nommé _external, car l'emplacement du dossier Documents de ce PC n'a pas pu être déterminé. Pour les utiliser, déplacez-les vous-même dans votre dossier Documents.
    }
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Saved Games folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-elsewhere =
    { $count ->
        [one] { $count } fichier provenant de Parties enregistrées va plutôt sous { $target }, dans un dossier nommé _external, car le dossier Parties enregistrées de ce PC se trouve dans { $folder }, et l'écriture n'a lieu que dans un dossier situé dans votre profil. Pour l'utiliser, déplacez-le vous-même dans votre dossier Parties enregistrées.
       *[other] { $count } fichiers provenant de Parties enregistrées vont plutôt sous { $target }, dans un dossier nommé _external, car le dossier Parties enregistrées de ce PC se trouve dans { $folder }, et l'écriture n'a lieu que dans un dossier situé dans votre profil. Pour les utiliser, déplacez-les vous-même dans votre dossier Parties enregistrées.
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-unknown =
    { $count ->
        [one] { $count } fichier provenant de Parties enregistrées va plutôt sous { $target }, dans un dossier nommé _external, car l'emplacement du dossier Parties enregistrées de ce PC n'a pas pu être déterminé. Pour l'utiliser, déplacez-le vous-même dans votre dossier Parties enregistrées.
       *[other] { $count } fichiers provenant de Parties enregistrées vont plutôt sous { $target }, dans un dossier nommé _external, car l'emplacement du dossier Parties enregistrées de ce PC n'a pas pu être déterminé. Pour les utiliser, déplacez-les vous-même dans votre dossier Parties enregistrées.
    }

## Errors when a folder that only administrators may change cannot be made so, or checked.

# { $path } is a folder.
core-privilege-not-a-folder = { $path } est un lien ou un fichier, et non un dossier
# { $path } is a folder. { $error } is what Windows said, in its own words.
core-privilege-not-made-here = { $path } n'a pas été créé par WinAppMigrator, et d'autres peuvent modifier son contenu. Un administrateur doit le vérifier et le supprimer : { $error }
# { $path } is a folder.
core-privilege-others-put-something-in = { $path } contient un élément qui n'a pas été créé par WinAppMigrator, et d'autres peuvent le modifier. Un administrateur doit le vérifier et le supprimer
# { $path } is a folder.
core-privilege-became-a-link-in-use = { $path } est devenu un lien pendant son utilisation
# { $path } is a folder.
core-privilege-became-a-link-looked-at = { $path } est devenu un lien pendant son examen
# { $path } is a file. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-users-cannot-start = impossible d'autoriser les utilisateurs à démarrer { $path } : { $said }
core-privilege-elevation-unknown = impossible de déterminer si ce processus dispose de privilèges élevés
# { $path } is a folder. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-cannot-restrict = impossible de restreindre l'accès à { $path } aux administrateurs : { $said }
# { $path } is a folder. { $said } is the folder's permissions as Windows wrote them, and what else it printed.
core-privilege-not-restricted = l'accès à { $path } n'a pas pu être restreint aux administrateurs, et son lecteur ne conserve peut-être pas les autorisations : { $said }
# { $path } is a folder.
core-privilege-written-to-first = un autre processus a écrit dans { $path } avant qu'il ait pu être protégé
core-privilege-no-program-data = Windows n'indique pas l'emplacement de ProgramData

## Checks of a package's files, and of what an install put in place. Each is said of one file, value, service, or the
## like, which the report names beside it.

# { $path } is the installer's file.
core-verify-no-package = { $path } ne contient aucun package
# { $path } is a folder.
core-verify-not-unpacked = le package de ce programme d'installation n'a pas été décompressé dans { $path }
core-verify-schema-supported = Le schéma du manifeste du package est pris en charge.
core-verify-packaged-file-missing = Le fichier empaqueté est manquant.
core-verify-no-package-path = Le fichier copiable n'a pas de chemin relatif dans le package.
core-verify-folder-there = Le dossier vide est présent.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-unreadable = Impossible de le lire pour le vérifier : { $error }
core-verify-restored-file-missing = Le fichier restauré est manquant.
# { $expected } and { $actual } are sizes in bytes.
core-verify-size-mismatch = Taille de fichier incorrecte. Attendu : { $expected } octets, trouvé : { $actual } octets.
core-verify-no-hash = Le manifeste n'enregistre aucun hachage pour ce fichier, il ne peut donc pas être vérifié.
core-verify-hash-mismatch = Le hachage SHA-256 ne correspond pas.
core-verify-file-matches = La taille et le hachage du fichier correspondent au manifeste.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-payload-failed = Échec de la vérification de la charge utile de licence : { $error }
core-verify-license-payload-missing = La charge utile de licence empaquetée est manquante.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-failed = Échec de la vérification du magasin de licences : { $error }
core-verify-license-store-file-missing = Le fichier de magasin de licences empaqueté est manquant.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-path-invalid = Le chemin du magasin de licences empaqueté n'est pas valide : { $error }
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-prerequisite-failed = Échec de la vérification de la charge utile du prérequis : { $error }
core-verify-prerequisite-missing = La charge utile du prérequis inclus est manquante.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-failed = Échec de la vérification de l'image du pilote : { $error }
core-verify-bundled-driver-missing = L'image du pilote incluse est manquante.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-path-invalid = Le chemin du pilote inclus n'est pas valide : { $error }
core-verify-prerequisite-present = Le prérequis d'exécution est présent sur l'ordinateur cible.
core-verify-prerequisite-not-detected = Prérequis d'exécution non détecté ; il nécessite peut-être un redémarrage ou une finalisation manuelle (consultez le journal de restauration pour le résultat qui fait foi).
core-verify-prerequisite-not-probed = La présence du prérequis ne peut pas être détectée dans le Registre ; l'installation a été effectuée au mieux.
core-verify-license-file-missing = Le fichier de licence restauré est manquant.
core-verify-license-store-restored-missing = Le fichier de magasin de licences restauré est manquant.
core-verify-registry-key-there = La clé de Registre existe.
core-verify-registry-value-matches = La valeur de Registre correspond aux données archivées.
core-verify-registry-value-differs = La valeur de Registre ne correspond pas aux données archivées.
core-verify-firewall-rule-there = Le Pare-feu Windows contient cette règle.
core-verify-variable-matches = La valeur de la variable d'environnement correspond aux données archivées.
core-verify-variable-differs = La valeur de la variable d'environnement ne correspond pas aux données archivées.
# { $error } is what Windows said, in its own words.
core-verify-variable-unreadable = La variable d'environnement est manquante ou illisible : { $error }
core-verify-service-runs-other = Le service existe, mais n'exécute pas le binaire restauré avec ses arguments.
core-verify-service-other-account = Le service existe, mais ne s'exécute pas sous le compte attendu.
core-verify-service-matches = Le service existe, et exécute le binaire restauré avec ses arguments et son compte.
core-verify-service-missing = Le service est manquant.
# { $error } is what Windows said, in its own words.
core-verify-service-unreadable = Le service est illisible : { $error }
core-verify-driver-other-image = Le service du pilote ne pointe pas vers l'image restaurée.
core-verify-driver-not-registered = Le service du pilote inclus n'est pas inscrit après la restauration.
# { $error } is what Windows said, in its own words.
core-verify-driver-unreadable = Le service du pilote est illisible : { $error }
core-verify-driver-image-missing = L'image du pilote est manquante.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-unchecked = L'image du pilote n'a pas pu être vérifiée : { $error }
core-verify-task-matches = La tâche planifiée existe et exécute les programmes restaurés.
core-verify-task-runs-other = La tâche planifiée existe, mais n'exécute pas les programmes restaurés.
core-verify-task-missing = La tâche planifiée est manquante.
# { $error } is what Windows said, in its own words.
core-verify-task-unreadable = Échec de l'interrogation de la tâche planifiée : { $error }
core-verify-shortcut-matches = Le raccourci existe et son chemin cible correspond à la cible restaurée.
core-verify-shortcut-opens-other = Le raccourci existe, mais son chemin cible ne correspond pas à la cible restaurée.
core-verify-shortcut-missing = Le raccourci est manquant.
# { $error } is what Windows said, in its own words.
core-verify-shortcut-unreadable = Échec de l'interrogation du raccourci : { $error }

## Warnings and errors a check gives, which the app and the installer show.

core-warning-verify-schema-unsupported = Le schéma du manifeste du package n'est pas pris en charge.
# { $error } is what went wrong, in Windows' or the program's own words.
core-warning-verify-package-file-failed = Échec de la vérification du fichier du package : { $error }
core-warning-verify-folder-missing = Le dossier vide est manquant.
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-key-missing = La clé de Registre est manquante ou illisible : { $error }
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-value-missing = La valeur de Registre est manquante ou illisible : { $error }
core-warning-verify-registry-kind-missing = Le type de la valeur de Registre est manquant.
core-warning-verify-firewall-rule-missing = Le Pare-feu Windows ne contient pas cette règle.
core-warning-verify-service-scope = La vérification des services lit la configuration des services dans HKLM et ne démarre pas les services.
core-warning-verify-driver-manual = Un ou plusieurs pilotes de fournisseur ou fournis avec Windows ne sont pas redistribués par WinAppMigrator ; réinstallez-les sur l'ordinateur cible avec le package de pilote signé du fournisseur.
core-warning-verify-task-scope = La vérification des tâches planifiées contrôle uniquement l'inscription et la référence de commande.

## The verification report written beside a package, verification_report.md. Its headings follow a "#".

core-verify-report-title = Rapport de vérification WinAppMigrator
# { $time } is when, as a date and time in UTC, such as 2026-11-15T10:00:00Z.
core-verify-report-verified = Vérifié (UTC) : { $time }
# { $valid } is true or false, which are not translated.
core-verify-report-package-valid = Package valide : { $valid }
# { $valid } is true or false, which are not translated.
core-verify-report-restore-valid = Restauration valide : { $valid }
core-verify-report-restore-not-run = Restauration valide : non exécutée
core-verify-report-items = Éléments
# { $message } is what a check found, and { $source } the file or setting it looked at.
core-verify-report-item-from = { $message } Source : { $source }
core-verify-report-warnings = Avertissements
core-verify-report-errors = Erreurs
core-verify-report-none = Aucun.
