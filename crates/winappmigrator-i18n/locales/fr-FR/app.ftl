# Words of the app, WinAppMigrator itself: its one window, with the installed apps listed on the left and, on the
# right, what goes in an installer of the one picked and the button that builds it; and the steps to read another PC
# through WinAppMigrator Client. Message names start with app-.


## The window and its header

# The program's name, in the window's title bar and at the top of the window. Keep it as it is unless your language
# writes product names in a script of its own.
app-name = WinAppMigrator
app-tagline = Transformez une application installée en programme d'installation auto-extractible
app-theme-to-light = Passer au thème clair
app-theme-to-dark = Passer au thème sombre
app-reload-list = Recharger la liste des applications
app-list-loading = Chargement
# { $count } is how many apps are listed.
app-list-count =
    { $count ->
        [one] { $count } application
       *[other] { $count } applications
    }
app-list-load-failed = Échec du chargement
# Shown when the window cannot open at all. { $error } is what went wrong, in the words of Windows or a graphics driver.
app-window-could-not-open = La fenêtre n'a pas pu s'ouvrir : { $error }

## The list of installed apps

app-list-search = Rechercher par nom ou éditeur  (Ctrl+F)
app-list-filter-all = Toutes
# Whether an app is installed for one account or for everyone on the PC.
app-scope-per-user = Par utilisateur
app-scope-per-machine = Par ordinateur
app-scope-unknown = Inconnue
# An app whose processor architecture (x86, x64, ARM64) is not known.
app-arch-unknown = Inconnue
app-list-loading-apps = Chargement des applications installées…
# { $error } is why the apps could not be listed.
app-list-could-not-list = Impossible de répertorier les applications : { $error }
app-list-no-matches = Aucune application correspondante
app-list-no-matches-hint = Modifiez le texte de recherche ou le filtre d'étendue.
app-list-worker-disconnected = Le processus d'inventaire s'est déconnecté.
# What a screen reader says of an app's row. { $name } is the app's name, { $publisher } who made it, and { $version } its
# version, such as 11.4.2, or — when either is not known.
app-list-row = { $name }, { $publisher }, version { $version }

## The list's columns, and what a screen reader says each column's heading does when clicked

app-column-name = Nom
app-column-name-sort = Trier par nom
app-column-publisher = Éditeur
app-column-publisher-sort = Trier par éditeur
app-column-version = Version
app-column-version-sort = Trier par version
app-column-scope = Étendue
app-column-scope-sort = Trier par étendue
# Short for architecture: x86, x64, or ARM64. The column is narrow.
app-column-arch = Arch.
app-column-arch-sort = Trier par architecture

## Where apps are read from: this PC, a drive from another PC, a disk image of one, or another PC on the network

app-source = Source de l'analyse
app-source-this-pc = Ce PC
app-source-drive = Lecteur connecté…
app-source-image = Image disque…
app-source-network = PC du réseau…
app-source-attaching-image = Montage de l'image en lecture seule…
# { $computer } is the other PC's name, such as OLD-PC, and { $address } its network address, such as 192.168.1.20.
app-source-network-pc = { $computer } via le réseau ({ $address })
app-source-disconnect = Déconnecter
app-source-change = Modifier…
# The title of the window that picks a disk image. .vhd and .vhdx are file extensions.
app-source-pick-image-title = Sélectionner une image disque de l'autre PC (.vhd ou .vhdx)
app-source-pick-image-filter = Images disque
# { $path } is the disk image file.
app-source-image-has-no-windows = { $path } ne contient aucun volume sur lequel Windows est installé.
app-source-image-not-attached = L'image disque n'a pas pu être montée.
app-source-pick-drive-title = Sélectionner la racine du lecteur connecté (par ex. E:\)
# { $path } is the folder picked, such as E:\. \Windows and \Users are folder names, as Windows writes them.
app-source-not-a-windows-drive = { $path } ne semble pas être un lecteur système Windows (ni \Windows ni \Users). Choisissez la racine du lecteur provenant de l'autre PC.
# Said of a drive or disk image with a Windows older than Windows 10. { $windows } is its name, such as "Windows 7
# Professional Service Pack 1, 32-bit". { $is_32_bit } is "yes" when that Windows is 32-bit, which adds the last
# sentence, and "no" when it is not.
app-source-older-windows =
    Ce disque contient { $windows }. Les programmes d'installation générés à partir de celui-ci sont conçus pour Windows 10 et 11. Avec l'option Données utilisateur, les paramètres que Windows a discrètement mis de côté pour une ancienne application sont repris, et les runtimes absents de Windows 11, comme DirectX 9, sont installés avec l'application.{ $is_32_bit ->
        [yes] { " " }Ses applications sont en 32 bits : elles vont donc dans Program Files (x86), où Windows 64 bits les range.
       *[no] { "" }
    }
# The same, of a Windows whose name could not be read. { $is_32_bit } is as above.
app-source-older-windows-unnamed =
    Ce disque contient une version antérieure de Windows. Les programmes d'installation générés à partir de celui-ci sont conçus pour Windows 10 et 11. Avec l'option Données utilisateur, les paramètres que Windows a discrètement mis de côté pour une ancienne application sont repris, et les runtimes absents de Windows 11, comme DirectX 9, sont installés avec l'application.{ $is_32_bit ->
        [yes] { " " }Ses applications sont en 32 bits : elles vont donc dans Program Files (x86), où Windows 64 bits les range.
       *[no] { "" }
    }

## The details panel, before an app is picked, and the app picked

app-details-pair-first = Associer l'ancien PC
app-details-pair-first-hint = Une fois WinAppMigrator Client lancé sur l'ancien PC et son code entré, ses applications s'affichent ici, et vous générez un programme d'installation à partir de l'une d'elles comme depuis ce PC.
app-details-pick-an-app = Choisir une application
app-details-pick-an-app-hint = Sélectionnez une application dans la liste pour choisir ce qu'il faut inclure et générer son programme d'installation.
# { $publisher } is who made the app, and { $version } its version, such as 11.4.2.
app-details-publisher-and-version = { $publisher }, version { $version }
# { $version } is the app's version, such as 11.4.2.
app-details-unknown-publisher-and-version = Éditeur inconnu, version { $version }
app-details-unknown-publisher = Éditeur inconnu

## What goes in besides the program

app-option-license = Données de licence et d'activation
app-option-license-hint = Peuvent être liées à ce PC et nécessiter une réactivation sur le nouveau.
app-option-user-data = Données utilisateur
# Documents, Saved Games, and ProgramData are folders of Windows: use the names Windows gives them in your language,
# except ProgramData, which keeps its name.
app-option-user-data-hint = Paramètres et historique, Documents, Parties enregistrées et ProgramData.
app-option-whose-data = Compte des données
app-option-whose-data-hint = Un programme d'installation contient les données d'un seul compte. Elles vont à la personne connectée sur le PC où il s'exécute.

## Building the installer

app-output-folder = Dossier de sortie
# An example of a folder, shown in the empty field. "You" stands for the name of an account.
app-output-folder-example = C:\Users\Vous\WinAppMigrator
app-output-folder-not-full = Entrez un chemin d'accès complet, par exemple D:\Installations.
app-build-create = Créer le programme d'installation
app-build-create-again = Recréer le programme d'installation
app-build-cancel = Annuler
app-build-cancelling = Annulation après l'étape en cours…
app-build-cancelled = Génération annulée. Aucun programme d'installation n'a été écrit.
# winappmigrator-installer.exe is a file name.
app-build-no-stub = Le stub du programme d'installation (winappmigrator-installer.exe) est introuvable à côté de cette application, un programme d'installation auto-extractible ne peut donc pas être généré.
app-build-select-an-app-first = Sélectionnez d'abord une application à migrer.
app-build-look-again-first = Cette application doit être réexaminée avant que son programme d'installation puisse être généré.
# { $reason } is what went wrong, in the program's own words, often in English.
app-build-stopped-unexpectedly = La génération s'est arrêtée de façon inattendue : { $reason }. Si elle a laissé un programme d'installation dans le dossier de sortie, ne l'utilisez pas.
app-build-worker-disconnected = Le processus de migration s'est déconnecté.
# { $path } is the output folder, and { $error } what Windows said went wrong.
app-build-no-output-folder = Impossible de créer le dossier de sortie { $path } : { $error }
# { $error } is what went wrong.
app-build-no-staging-folder = Impossible de créer un dossier de préparation protégé, rien n'a donc été généré : { $error }
# { $error } is what went wrong, as a sentence.
app-build-could-not-build = Le programme d'installation n'a pas pu être généré. { $error }
app-build-no-installer = La génération s'est terminée sans programme d'installation.
# { $error } is what went wrong.
app-build-hash-check-did-not-run = Le programme d'installation a été écrit, mais sa vérification du hachage n'a pas pu s'exécuter : { $error }. Ne l'utilisez pas.

## How far a build has got

app-progress-preparing = Détermination du contenu…
app-progress-checking-room = Vérification de l'espace disponible…
app-progress-bundling = Regroupement des runtimes et des pilotes…
# { $done } is how many files have been read so far, and { $total } how many there are.
app-progress-packing = Lecture et compression des fichiers : { $done } sur { $total }
# { $done } is how many of the installer's records have been looked over so far, and { $total } how many there are.
app-progress-looking-over = Vérification du contenu du programme d'installation : { $done } sur { $total }…
app-progress-writing = Écriture du programme d'installation…
app-progress-verifying = Vérification des hachages…

## What a build made

app-result-failed-hash-check = N'utilisez pas ce programme d'installation : il a échoué à sa vérification du hachage.
# { $count } is how many files were gone when the build came to read them.
app-result-files-gone =
    { $count ->
        [one] Programme d'installation généré, mais { $count } fichier avait disparu depuis l'examen de l'application, il n'y figure donc pas. Consultez Avertissements ci-dessous.
       *[other] Programme d'installation généré, mais { $count } fichiers avaient disparu depuis l'examen de l'application, ils n'y figurent donc pas. Consultez Avertissements ci-dessous.
    }
# { $count } is how many files could not be read into the installer.
app-result-files-not-captured =
    { $count ->
        [one] Programme d'installation généré, mais { $count } fichier n'a pas pu être capturé, généralement parce qu'il était verrouillé ou illisible. L'application risque de ne pas fonctionner sans lui. Consultez Avertissements ci-dessous.
       *[other] Programme d'installation généré, mais { $count } fichiers n'ont pas pu être capturés, généralement parce qu'ils étaient verrouillés ou illisibles. L'application risque de ne pas fonctionner sans eux. Consultez Avertissements ci-dessous.
    }
app-result-mentions-who = Programme d'installation généré, mais il contient encore des indications sur sa provenance. Lisez ce qui est indiqué ci-dessous avant de le donner à qui que ce soit.
app-result-ready = Programme d'installation prêt. Son package a réussi sa vérification du hachage.
app-result-files = Fichiers
app-result-installer-size = Taille du programme d'installation
app-result-warnings = Avertissements
app-result-log = Journal
app-result-copy-error = Copier l'erreur
app-result-installer = Programme d'installation
app-result-data-file = Fichier de données
app-result-open-folder = Ouvrir le dossier
app-result-copy-path = Copier le chemin d'accès
# { $error } is what Windows said went wrong.
app-result-could-not-open-folder = Impossible d'ouvrir le dossier : { $error }
# { $installer } and { $data_file } are file names, such as App_installer.exe and App_installer.wamdata.
app-result-two-files = Cette application est trop volumineuse pour un seul exécutable, son programme d'installation se compose donc de deux fichiers, { $installer } et { $data_file }. Ils doivent rester ensemble : copiez-les tous les deux sur l'autre PC, dans le même dossier, sous ces noms. Si vous signez le programme d'installation, cette signature couvre aussi le fichier de données.
# { $app_name } is the app's name. "Unknown publisher" is what Windows shows when it asks to run an unsigned program: use
# Windows' own words for it in your language.
app-result-run-it = Exécutez-le sur l'autre PC pour réinstaller { $app_name }. Il vérifie ses fichiers par SHA-256. Windows affiche « Éditeur inconnu », car les programmes d'installation que vous générez ne sont pas signés.
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-verification-errors = Erreurs de vérification ({ $count })
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-warnings-heading = Avertissements ({ $count })
# After the first of a long list of warnings. { $count } is how many more there are.
app-warnings-and-more =
    { $count ->
        [one] et { $count } autre.
       *[other] et { $count } autres.
    }

## Looking at an app, to see what an installer of it would hold

app-scan-getting-ready = Préparation de l'examen de cette application…
app-scan-install-record = Lecture de la façon dont elle a été installée…
app-scan-near-matches = Recherche de ce qui pourrait aussi en faire partie…
app-scan-system-items = Recherche de ses services, tâches et raccourcis…
app-scan-files = Parcours de ses fichiers…
app-scan-registry = Lecture de ce qui l'intègre à Windows…
app-scan-finishing = Finalisation…
app-scan-look-again = Réexaminer
app-scan-stop = Arrêter
app-scan-running-hint = Vous pouvez choisir une autre application ou modifier ce qui est inclus pendant cette opération.
# { $error } is why, in words that follow a colon.
app-scan-failed = Cette application n'a pas pu être examinée : { $error }
app-scan-try-again = Réessayer
app-scan-stopped = Arrêté. Un programme d'installation peut être généré une fois cette application examinée.
# { $reason } is what went wrong, in the program's own words, often in English.
app-scan-stopped-unexpectedly = L'examen s'est arrêté de façon inattendue : { $reason }.

## How big an installer can be, and whether there is room for it

app-room-files = Fichiers
# Over how big the installer can be: up to that size.
app-room-up-to = Jusqu'à
app-room-free = Libre
# { $drive } is a drive, such as C:.
app-room-free-on = Libre sur { $drive }
# How much room a drive has, when it would not say.
app-room-free-unknown = Inconnu
app-room-most-it-could-need = Le maximum qui pourrait être nécessaire, avant compression.
# { $drive } is a drive, such as C:, { $needed } how much room it needs, such as 1.2 GB, and { $free } how much it has.
app-room-drive-short = { $drive } nécessite jusqu'à { $needed } d'espace libre pour cette génération, et dispose de { $free }.
# { $drives } is one sentence like the one above for each drive without the room, one after another.
app-room-not-enough = L'espace est insuffisant. { $drives } Choisissez un dossier de sortie sur un autre lecteur, ou excluez certains éléments.
# { $note } is a sentence, without its full stop, about a drive whose room could not be checked. Use your language's
# full stop.
app-room-could-not-tell = { $note }.

## What goes in the installer, part by part

app-footprint-heading = Contenu du programme d'installation
app-footprint-working-out = Évaluation de ce qui l'accompagne…
# { $error } is why.
app-footprint-would-not-make-a-package = Ce qui reste ne suffirait pas à constituer un package : { $error }
# { $list } is the names of what else is left out, such as a service and a shortcut.
app-footprint-also-left-out = Cela exclut aussi { $list }, car ce qu'ils exécutent ou désignent serait absent. Ils ne seront pas configurés sur l'autre PC.
# { $list } is the names of the first few of what else is left out, and { $count } how many more there are.
app-footprint-also-left-out-and-more =
    Cela exclut aussi { $list } et { $count ->
        [one] { $count } autre élément
       *[other] { $count } autres éléments
    }, car ce qu'ils exécutent ou désignent serait absent. Ils ne seront pas configurés sur l'autre PC.
# { $list } is the names of license files. The option is the one named "License and Activation Data".
app-footprint-licenses-still-go = Les données de licence comprises dans ce que vous avez exclu sont tout de même incluses, car elles relèvent de l'option Données de licence et d'activation : { $list }.
# { $elapsed } is how long ago, in minutes and seconds such as 45:00, or hours, minutes, and seconds such as 1:05:00.
# "Look Again" names the button of that name.
app-footprint-looked-at-long-ago = Cette application a été examinée il y a { $elapsed }. Les fichiers sont lus au moment de la génération : une modification est donc prise en compte, et tout ce qui a disparu d'ici là est signalé. Utilisez Réexaminer pour prendre en compte tout nouvel élément.
# Headings of the parts of the list, each with a line that says what is in it.
app-footprint-program-files = Fichiers du programme
app-footprint-data-folders = Dossiers de données
# What could go in with the User Data option, when nothing else is in this part. { $size } is how much, such as 1.2 GB.
app-footprint-folders-with-user-data = { $size } avec l'option Données utilisateur
# { $included } is how many folders go in, { $count } how many there are, and { $size } how much those that go in hold.
app-footprint-folders-summary =
    { $count ->
        [one] { $included } sur { $count } dossier, { $size }
       *[other] { $included } sur { $count } dossiers, { $size }
    }
# { $size } is how much, such as 1.2 GB, and { $count } in how many files.
app-footprint-data-only-with-user-data =
    { $count ->
        [one] { $size } de données de cette application, dans { $count } fichier, ne seront incluses qu'avec l'option Données utilisateur.
       *[other] { $size } de données de cette application, réparties dans { $count } fichiers, ne seront incluses qu'avec l'option Données utilisateur.
    }
# A folder's row. { $count } is how many files it holds, and { $detail } where it is or why it cannot be left out.
app-footprint-folder-row =
    { $count ->
        [one] { $count } fichier. { $detail }
       *[other] { $count } fichiers. { $detail }
    }
app-footprint-services = Services
app-footprint-startup = Démarre automatiquement
app-footprint-tasks = Tâches planifiées
app-footprint-shortcuts = Raccourcis
# { $included } is how many of the part's things go in, and { $count } how many there are.
app-footprint-items-summary = { $included } sur { $count }
# { $why } is what else was left out that this goes with.
app-footprint-left-out-too = Également exclu : { $why }
# A row's detail. { $kind } is what it is, such as Service, and { $detail } what it does.
app-footprint-change-detail = { $kind }. { $detail }
app-footprint-integrations = Ce qui l'intègre à Windows
# { $included } is how many kinds go in, such as file types and firewall rules, and { $count } how many kinds there are.
app-footprint-integrations-summary =
    { $count ->
        [one] { $included } sur { $count } type
       *[other] { $included } sur { $count } types
    }
app-footprint-integration-nothing-left = Plus rien, en raison de ce que vous avez exclu
# { $count } is how many things of this kind the installer sets up.
app-footprint-integration-things =
    { $count ->
        [one] { $count } élément configuré
       *[other] { $count } éléments configurés
    }
app-footprint-prerequisites = Ce dont elle a besoin pour s'exécuter
# { $count } is how many runtimes, such as Visual C++ Redistributable.
app-footprint-prerequisites-summary =
    { $count ->
        [one] { $count } runtime
       *[other] { $count } runtimes
    }
app-footprint-near-matches = Pourrait aussi en faire partie
# { $ticked } is how many were ticked, and { $count } how many there are.
app-footprint-near-matches-summary = Ajoutés : { $ticked } sur { $count }
app-footprint-near-matches-hint = Non inclus dans le programme d'installation, sauf si vous les cochez. En cocher un relance l'examen de l'application, avec les mêmes vérifications que pour le reste : ce qui ressemble à un mot de passe reste donc exclu.
# { $why } is why it might be part of the app, as a sentence.
app-footprint-near-match-with-user-data = Inclus avec l'option Données utilisateur. { $why }
# How much a folder holds, when not all of it could be measured. { $size } is how much was, such as 1.2 GB.
app-footprint-near-match-over = plus de { $size }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-folder = Dossier : { $place }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-hidden = Partie masquée d'une application : { $place }
app-footprint-nothing-left-out = Rien n'a été exclu parce qu'il ressemblait à un mot de passe, une licence ou un certificat.
app-footprint-left-out = Éléments exclus et raisons
# { $count } is how many things were left out.
app-footprint-left-out-summary =
    { $count ->
        [one] { $count } élément
       *[other] { $count } éléments
    }
# Above a list of things left out for one reason. { $title } is the reason.
app-footprint-left-out-group = { $title } :
app-footprint-refused = Aucun programme d'installation ne serait autorisé à configurer ces éléments :
# A heading over a list that opens and closes. { $count } is how many are in it.
app-footprint-worth-knowing = Bon à savoir ({ $count })
# { $list } is what kinds of things were looked for and none found, from the five below.
app-none-found = Aucun élément trouvé : { $list }.
app-none-found-services = services
app-none-found-startup = éléments de démarrage
app-none-found-tasks = tâches planifiées
app-none-found-shortcuts = raccourcis
app-none-found-integrations = intégrations à Windows

## Network PC: reading another PC through WinAppMigrator Client, in three steps. "WinAppMigrator Client" is the name of a
## program, and the old PC is the one the apps are moved off.

app-network-step-client = 1. Placer le client sur l'ancien PC
app-network-step-client-hint = WinAppMigrator Client s'exécute sur le PC où se trouve actuellement l'application, sous Windows 7 ou version ultérieure. Il permet uniquement à ce PC de lire l'ancien, de façon chiffrée, grâce au code qu'il affiche.
# { $drive } is a USB drive, such as E:\.
app-network-save-to-usb = Enregistrer sur le lecteur USB { $drive }
# { $drive } is a USB drive, such as E:\, and { $name } the name it was given, such as BACKUP.
app-network-save-to-named-usb = Enregistrer sur le lecteur USB { $drive } ({ $name })
app-network-save-to-folder = Enregistrer dans un dossier…
app-network-look-for-usb-again = Rechercher à nouveau les lecteurs USB
app-network-no-usb = Aucun lecteur USB n'est branché. Branchez-en un, ou enregistrez le client dans un dossier accessible depuis l'ancien PC, comme un dossier partagé sur votre réseau.
# { $path } is where the client was saved, such as E:\WinAppMigrator Client.exe.
app-network-client-saved = Enregistré sous { $path }. Sur l'ancien PC, fermez l'application que vous déplacez, puis double-cliquez sur ce fichier et autorisez-le à apporter des modifications. Il affiche un code.
app-network-step-pick = 2. Choisir l'ancien PC
app-network-looking = Recherche sur votre réseau de PC exécutant le client…
app-network-type-name = Ou entrer son nom ou son adresse
# An example, shown in the empty field: a PC's name, or its network address.
app-network-type-name-example = ANCIEN-PC ou 192.168.1.20
app-network-step-code = 3. Entrer le code qu'il affiche
app-network-code = Code d'association
app-network-connect = Se connecter
app-network-pairing = Association…
# Why pairing did not work, shown on its own.
app-network-pairing-stopped = l'association s'est arrêtée avant d'aboutir
app-network-wrong-code = ce n'est pas le code affiché par l'ancien PC. Vérifiez-le, puis réessayez.
# { $error } is what went wrong.
app-network-not-reached = l'ancien PC n'a pas pu être joint : { $error }
app-network-pick-folder-title = Choisir un dossier pour WinAppMigrator Client
# { $path } is the folder, and { $error } what Windows said went wrong.
app-client-could-not-save = le client n'a pas pu être enregistré dans { $path } : { $error }
# scripts\build-client.ps1 is a file name.
app-client-not-carried = cette version de WinAppMigrator ne contient pas le client. Générez-le avec scripts\build-client.ps1, puis générez à nouveau l'application.
# { $error } is what Windows said went wrong.
app-client-could-not-save-note = la note qui l'accompagne n'a pas pu être enregistrée : { $error }
# A note saved as a text file beside the client, for whoever runs it on the old PC. Keep its lines short, as it is read in
# Notepad. "WinAppMigrator Client.exe" is a file name. "Network PC..." names the Scan Source button: use its words in your
# language.
app-client-note =
    WinAppMigrator Client
    
    Exécutez ce programme sur le PC dont vous déplacez les applications :
    Windows 7, 8, 10 ou 11.
    
    1. Fermez l'application que vous déplacez.
    2. Double-cliquez sur « WinAppMigrator Client.exe » et autorisez-le à
       apporter des modifications.
    3. Il affiche un code d'association à 6 chiffres. Dans WinAppMigrator,
       sur votre nouveau PC, choisissez PC du réseau…, sélectionnez ce PC,
       puis entrez le code.
    4. Quand le nouveau PC a terminé, appuyez sur Entrée dans la fenêtre du
       client, ou fermez-la.
    
    Il se contente de lire. Pendant son exécution, il conserve un cliché
    instantané du lecteur Windows et laisse passer ses deux ports dans le
    Pare-feu Windows depuis votre réseau local, puis il supprime les deux à
    son arrêt. Ce qui transite par le réseau est chiffré, et seul un PC qui
    possède le code peut lire quoi que ce soit.
