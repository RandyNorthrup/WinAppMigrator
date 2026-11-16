# Words the core writes for people while it builds a package from an installed app: what a package records for an
# installer to show later, errors shown in the app, the list of what goes in and what is left out, opening a disk
# image or another PC over the network, and what an installer says when the package it carries cannot be read.
# Message names start with core-.


## Scanning an app, before a package is built. Shown in the app when a scan cannot go on.

# { $path } is the app's program, such as D:\Program Files\Acme\Acme.exe, on a drive taken from another PC.
core-scan-program-not-on-the-drive = { $path } passe par un lien ou ne se trouve pas sur le lecteur connecté, il n'a donc pas été lu.
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-not-on-this-pc = { $path } ne se trouve pas sur un disque de ce PC, ou porte un nom qui ouvrirait autre chose, il n'a donc pas été lu.
# { $path } is the app's program.
core-scan-program-someone-elses = { $path } se trouve dans les dossiers d'un autre compte ou n'appartient pas à ce compte, et l'enregistrement propre à cette application n'indique pas qu'il y a sa place, il n'a donc pas été lu.
# { $app_name } is the app's name, as the list of apps calls it.
core-scan-no-executable = impossible de trouver un exécutable pour { $app_name }. Un pilote, un runtime ou un complément peut ne pas avoir de programme propre à empaqueter.
# { $path } is the file that was picked.
core-scan-not-a-file = la cible de l'analyse n'est pas un fichier : { $path }
# Names something left out of a package, in a list of what was left out and why. { $name } is the variable's name,
# such as ACME_LICENSE_KEY.
core-scan-environment-variable-item = La variable d'environnement { $name }

## Folders and parts that might also be the app's, offered in a list to tick. Each says why it is offered.

core-near-starts-with-app-name = Son nom commence par le nom de cette application.
core-near-has-app-name = Son nom contient le nom de cette application.
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-starts-with-publisher = Son nom commence par le nom de l'éditeur, { $publisher }.
# { $folder } is a folder's name, such as Program Files.
core-near-hidden-part-beside = Une partie masquée dans la liste des applications, installée à côté de cette application dans { $folder }.
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-hidden-part-publisher = Une partie masquée dans la liste des applications, du même éditeur, { $publisher }.

## What a scan found, as a person chooses from it: the folders that go in, and what is left out, and why

# Why a folder cannot be left out, beside its number of files.
core-footprint-locked-app-folder = Ces éléments accompagnent toujours le programme principal
core-footprint-locked-main-program = Contient le programme principal, qui est toujours inclus
core-footprint-locked-needed = Contient ce sans quoi le programme principal ne peut pas démarrer
core-footprint-locked-font = Accompagne sa police, sous Ce qui l'intègre à Windows
# Where a folder is. "AppData" and "ProgramData" are the names of Windows folders; keep them as they are.
core-footprint-place-roaming = Paramètres dans votre profil (AppData)
core-footprint-place-local = Paramètres propres à ce PC dans votre profil (AppData)
core-footprint-place-program-data = Partagé par tous les utilisateurs du PC (ProgramData)
core-footprint-place-documents = Dans vos Documents
core-footprint-place-saved-games = Dans vos Parties enregistrées
core-footprint-place-public-documents = Dans Documents publics
core-footprint-place-app-folder = Dans le dossier de l'application
# { $folder } is a folder, such as C:\Users\Sam\AppData\Roaming.
core-footprint-place-in = Dans { $folder }
# A row for the files directly in the app's folder. { $folder } is that folder's name, such as Acme.
core-footprint-files-directly-in = Fichiers situés directement dans { $folder }
# Headings over what is left out, and why each item is.
core-footprint-licenses-left-out = Non inclus dans ce programme d'installation, car l'option Données de licence et d'activation est désactivée
core-footprint-font-stays-behind = Elle reste sur place.
core-footprint-drivers-left-out = Pilotes appartenant à Windows ou à un fournisseur, qui ne peuvent pas être redistribués : réinstallez-les sur l'autre PC
core-footprint-driver-reinstall = Réinstallez-le à partir du package de son fournisseur.
# Why something else is left out along with what the person left out. { $service } is a service's name, such as AcmeSync.
core-footprint-needs-left-out = il a besoin de { $service }, qui est exclu
core-footprint-told-to-use-left-out = il est configuré pour utiliser un élément d'un dossier que vous avez exclu
core-footprint-file-in-left-out = son fichier se trouve dans un dossier que vous avez exclu
core-footprint-program-in-left-out = le programme auquel il est destiné se trouve dans un dossier que vous avez exclu
core-footprint-names-left-out = il désigne un dossier que vous avez exclu
# Why something is left out, as the installer tells whoever installs it.
core-footprint-builder-left-it-out = la personne qui a généré ce programme d'installation l'a exclu
# Something the person left out, by kind and name, such as "Service Acme Updater". { $kind } is the kind, such as
# Service or Scheduled Task; { $title } is its name.
core-footprint-left-out-item = { $kind } { $title }

## Building a package and an installer from a scan. Shown in the app when a build cannot go on.

core-package-scan-of-another-app = cette analyse porte sur une autre application que celle en cours de génération, rien n'a donc été généré
core-package-scan-brought-in-more = cette analyse a apporté des données que cette génération n'a pas demandées, rien n'a donc été généré
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-package-program-gone = { $path } n'existe plus, rien n'a donc été généré. Réexaminez l'application.
core-package-needs-stub = une archive auto-extractible nécessite un stub de programme d'installation
# { $reason } is what went wrong.
core-package-staged-file-failed = un fichier du dossier de préparation n'a pas pu être ajouté au package : { $reason }
# { $shortfalls } says which drives are short of room and by how much, in sentences of its own.
core-package-not-enough-room = l'espace est insuffisant pour générer ce programme d'installation, rien n'a donc été lu ni écrit. { $shortfalls }. Libérez de l'espace à cet endroit, ou choisissez un dossier de sortie sur un autre lecteur.
# { $path } is a file's path, on a drive taken from another PC.
core-package-not-read-off-the-drive = { $path } passe par un lien ou ne se trouve pas sur le lecteur connecté, il n'a donc pas été lu
# { $path } is a file's path.
core-package-not-read-not-local = { $path } ne se trouve pas sur un disque de ce PC, il n'a donc pas été lu
# { $path } is a file's path.
core-package-not-read-other-account = { $path } mène aux dossiers d'un autre compte, il n'a donc pas été lu
# { $path } is a file's path, as the PC it came from called it.
core-package-not-on-the-drive = { $path } ne se trouve pas sur le lecteur connecté, il n'a donc pas été lu
# { $path } is a file's path.
core-package-backup-copy-no-parent = impossible d'effectuer une copie de sauvegarde d'un fichier sans répertoire parent : { $path }
# { $path } is a file's path.
core-package-backup-copy-destination-no-parent = la destination de la copie de sauvegarde n'a pas de répertoire parent : { $path }
# robocopy is the name of a Windows command; keep it as it is. { $code } is how it ended, such as Some(8).
core-package-robocopy-failed = échec de la copie robocopy en mode sauvegarde (code de sortie { $code })
# Stands for a folder where a file's name would be, as in "I/O failed for temporary package folder: …".
core-package-temporary-folder = dossier temporaire du package
# Stands for the installer being written where a file's name would be, as in "I/O failed for the installer: …".
core-package-the-installer = le programme d'installation
# { $path } is a file's path, or its name in the package. { $error } is what went wrong with it.
core-package-path-error = { $path } : { $error }
# { $name } is a file's name in the package, such as app/bin/App.exe.
core-package-same-place = un autre fichier occupe la même place dans le package : { $name }
# { $name } is a file's name in the package.
core-package-changed-in-protected-folder = { $name } a changé dans le dossier protégé pendant la création du package
core-package-worker-died = un thread de compression de fichiers s'est arrêté de façon inattendue, aucun programme d'installation n'a donc été écrit
# { $path } is a file's path.
core-package-link-not-read = { $path } est un lien, il n'a donc pas été lu
# { $path } is a file's path. { $error } is what went wrong.
core-package-not-compressed = { $path } n'a pas pu être compressé : { $error }
# { $path } is a file's path. { $reason } is what went wrong.
core-package-not-read-back = { $path } n'a pas pu être relu : { $reason }

## Where an app's files are, and where an installer may write them. Shown when a path is refused.

# { $path } is a folder, such as C:\Program Files\Acme.
core-discovery-root-not-full-path = le dossier dans lequel écrire doit être un chemin d'accès complet : { $path }
# { $path } is a folder's path.
core-discovery-root-not-a-folder = le dossier dans lequel écrire n'est pas un dossier : { $path }
# { $path } is a file's path.
core-discovery-escapes-root = le chemin sort de la racine prévue : { $path }
# { $path } is the link's path.
core-discovery-refusing-to-write-through-link = écriture refusée à travers un lien : { $path }
# { $path } is the link's path.
core-discovery-refusing-link = passage refusé à travers un lien : { $path }
# { $error } is what Windows said went wrong.
core-discovery-refusing-link-because = passage refusé à travers un lien : { $error }
# { $path } is the link's path.
core-discovery-link-to-somewhere-else = { $path } est un lien vers un autre emplacement, et d'autres peuvent modifier sa cible. Utilisez l'emplacement vers lequel il pointe
# { $path } is a folder's path.
core-discovery-no-parent-folder = { $path } n'a pas de dossier parent
# { $path } is the install location as it was given. Keep the quotes, or use your language's own.
core-discovery-install-location-not-full-path = l'emplacement d'installation doit être un chemin d'accès complet : « { $path } »
# { $path } is the install location.
core-discovery-install-location-a-link = l'emplacement d'installation est un lien vers un autre emplacement : { $path }
# { $path } is a file's path.
core-discovery-device-name = le chemin utilise un nom réservé à un périphérique Windows : { $path }
# { $path } is a file's path.
core-discovery-device-name-on-older-windows = { $path } porte un nom que cette version de Windows ouvre comme un périphérique et non comme un fichier, rien n'y a donc été écrit. Il peut être installé sur Windows 11
# { $path } is a file's path.
core-discovery-unsafe-path = chemin non sécurisé : { $path }
# { $path } is a file's path.
core-discovery-no-package-path = le fichier n'a pas de chemin relatif dans le package : { $path }

## Writing an installer, and reading the package inside one. Most of these are shown by an installer, on the PC it
## runs on, when it cannot read what it carries. "The package" is the app's files and settings that an installer
## carries; "the stub" is the program part of an installer, before the package; "the data file" is the .wamdata file
## that holds the package of a very large installer, beside it.

# Stands for the package where a file's name would be, as in "I/O failed for installer package: …".
core-archive-installer-package = package du programme d'installation
# Stands for the installer program built into WinAppMigrator, where a file's name would be.
core-archive-embedded-stub = stub du programme d'installation intégré
# { $index } is the number of a file in the package. { $reason } is what went wrong.
core-archive-entry-unreadable = l'entrée { $index } du package n'a pas pu être lue : { $reason }
# { $name } is a file's name inside the package, such as app/bin/App.exe.
core-archive-entry-unsafe-path = une entrée zip a un chemin non sécurisé : { $name }
# { $name } is a file's name inside the package.
core-archive-holds-more = { $name } contient plus qu'il ne l'indique
# { $name } is a file's name inside the package.
core-archive-name-too-long = ce nom est trop long pour un fichier zip : { $name }
core-archive-changed-while-copied = le package a changé pendant son ajout à la suite du stub, aucun programme d'installation n'a donc été écrit
core-archive-stub-not-a-program = la partie de ce programme d'installation qui précède son package n'est pas un programme
core-archive-damaged = ce programme d'installation est endommagé ou a été modifié après sa génération, rien n'a donc été lu dans son package. Copiez-le à nouveau depuis l'emplacement où il a été généré.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-missing = ce programme d'installation conserve son package dans un fichier nommé { $name }, dans le même dossier, et ce fichier est absent. Les deux fichiers doivent rester ensemble : copiez ici { $name } depuis l'emplacement où le programme d'installation a été généré, puis relancez celui-ci.
# { $name } is the data file's name, such as App_installer.wamdata. { $error } is what Windows said went wrong.
core-archive-data-file-unopened = le fichier de données de ce programme d'installation, { $name }, n'a pas pu être ouvert : { $error }. Les deux fichiers doivent rester ensemble, dans un dossier que ce PC peut lire.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-a-link = { $name } est un lien ou un dossier, alors que le fichier de données de ce programme d'installation doit être un fichier à part entière, juste à côté de lui, rien n'y a donc été lu. Copiez ici le fichier réel.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-wrong = { $name } n'est pas le fichier de données de ce programme d'installation, ou il est endommagé ou a été modifié après sa génération, rien n'y a donc été lu. Copiez à nouveau les deux fichiers depuis l'emplacement où ils ont été générés.
core-archive-stub-signed = le stub du programme d'installation est signé ; il doit rester non signé, signez donc plutôt le programme d'installation terminé
# { $size } is a size, such as 4.2 GB.
core-archive-too-big-to-run = le programme d'installation ferait { $size }, et Windows ne peut pas exécuter un exécutable de 4 Go ou plus ; rien n'a été écrit
core-archive-outside-signature = le programme d'installation contient des données hors de sa signature et a peut-être été falsifié
core-archive-footer-length = la longueur du pied de l'archive auto-extractible n'est pas valide
core-archive-seek-outside = positionnement en dehors du package
# { $path } is a file's path.
core-archive-unsafe-package-path = chemin de package non sécurisé : { $path }
# { $path } is a file's path.
core-archive-empty-package-path = chemin de package vide : { $path }

## Opening a disk image of another PC (a .vhd or .vhdx file) to read apps from it

# { $path } is the disk image file, such as D:\Backups\OldPC.vhdx.
core-image-not-attachable = { $path } n'est pas une image disque que Windows peut monter (.vhd ou .vhdx)
core-image-needs-administrator = le montage d'une image disque nécessite des droits d'administrateur
# { $path } is the disk image file the person picked.
core-image-not-a-file = { $path } n'est pas un fichier
# { $path } is the disk image file. "Edit Disk" is the name of a Hyper-V Manager command.
core-image-differencing = { $path } est un disque de différenciation, qui ne contient que ce qui a changé depuis le disque à partir duquel il a été créé, et Windows ouvrirait aussi ce disque, où qu'il indique se trouver. Fusionnez-le d'abord dans ce disque avec Modifier le disque dans Hyper-V, puis choisissez ce dernier
# { $path } is the disk image file.
core-image-unreadable = { $path } n'est pas un fichier .vhd ou .vhdx lisible
# ProgramData is the name of a Windows folder; keep it as it is.
core-image-no-program-data = le dossier ProgramData de ce PC est introuvable
# The reason given in core-image-could-not-attach when Windows already has the image open.
core-image-already-attached = Windows l'a déjà monté, et ce qui l'a monté est peut-être en train d'y écrire. Démontez-le d'abord, ou choisissez son lecteur en tant que lecteur connecté
# { $path } is the disk image file. { $reason } is why, such as core-image-already-attached or what Windows said.
core-image-could-not-attach = { $path } n'a pas pu être monté : { $reason }
# { $path } is the disk image file.
core-image-no-windows-volume = { $path } ne contient aucun volume lisible sur lequel Windows est installé. Un volume chiffré avec BitLocker doit d'abord être déverrouillé.
# Given as the reason when Windows failed without saying anything.
core-image-windows-did-not-say = Windows n'a pas indiqué pourquoi

## Reading another PC over the network, through the client running on it. Shown as why something on that PC
## could not be read. "The client" is WinAppMigrator Client, the small program on the other PC.

core-remote-too-slow = l'autre PC a mis trop de temps à répondre
core-remote-closed = la connexion à l'autre PC est fermée
core-remote-failed = la connexion à l'autre PC a échoué
core-remote-answered-something-else = le client a répondu autre chose
# { $name } is the PC's name or address as the person typed it, such as OLDPC or 192.168.1.20.
core-remote-not-found = { $name } est introuvable sur le réseau
core-remote-did-not-say-what-it-is = le client n'a pas indiqué ce qu'il est
core-remote-ordinary-names-only = seul un chemin sous la racine de l'autre PC, composé de noms ordinaires, peut être demandé
core-remote-more-but-none = l'autre PC a indiqué qu'un dossier contenait davantage d'éléments, sans en fournir aucun
# { $count } is the most things one folder may hold, such as 250,000.
core-remote-too-many-listed = l'autre PC a répertorié plus de { $count } éléments dans un seul dossier
core-remote-sent-too-much = l'autre PC a envoyé une plus grande partie d'un fichier que celle demandée
core-remote-name-not-text = un nom qui n'est pas du texte ne peut pas être demandé
core-remote-below-root-only = seul un chemin sous la racine de l'autre PC peut être demandé
core-remote-file-ended-early = le fichier de l'autre PC s'est terminé prématurément
core-remote-link-not-followed = un lien sur un autre PC n'est pas suivi
core-remote-only-files-opened = seul un fichier peut être ouvert sur un autre PC
core-remote-listing-said-nothing = la liste n'en disait rien
core-remote-change-time-unknown = la date de modification est inconnue
core-remote-seek-before-start = un positionnement avant le début
# { $size } is how many bytes a file is; { $limit } is the most that were expected. Both are numbers.
core-remote-too-large =
    { $size ->
        [one] sa taille est de { $size } octet, alors que { $limit } au plus étaient attendus
       *[other] sa taille est de { $size } octets, alors que { $limit } au plus étaient attendus
    }

## Checking that a download is signed by Microsoft. Each is given as the reason a check failed.

# { $error } is what Windows said went wrong.
core-signature-no-powershell = impossible de démarrer PowerShell : { $error }
# { $error } is what went wrong reading PowerShell's answer.
core-signature-unreadable = impossible de lire la signature : { $error }
# { $signer } is who signed the file, such as Contoso Ltd.
core-signature-not-microsoft = il est signé par { $signer }, et non par Microsoft
core-signature-not-microsoft-unnamed = il est signé par quelqu'un d'autre que Microsoft
# { $status } is Windows' own word for the signature's state, such as HashMismatch or NotSigned, which is not translated.
core-signature-status = l'état de sa signature est { $status }

## Warnings a package records when it is built from an app, which an installer shows later, maybe on another PC, in
## that PC's language. "This app" is the app being packaged.

core-warning-build-program-needs-unreadable = Les éléments de Windows dont ce programme a besoin n'ont pas pu être lus, sa capacité à s'exécuter sur l'autre PC ne sera donc pas vérifiée.
core-warning-build-external-manifest = Manifeste d'application externe détecté ; l'interprétation des dépendances côte à côte est reportée.
core-warning-build-non-exe = Le fichier sélectionné n'utilise pas l'extension .exe ; l'analyse PE va se poursuivre.
core-warning-build-stand-in-program = Aucun exécutable d'application ordinaire n'a été trouvé, ce fichier le remplace donc. Les résultats d'architecture et de dépendances décrivent ce fichier, et peuvent ne pas décrire l'application.
core-warning-build-other-apps-files-unknown = Les données de Windows Installer sur les autres applications n'ont pas toutes pu être lues : un dossier enregistré pour cette application peut donc contenir les fichiers d'une autre application sans que rien ici ne le signale.
core-warning-build-app-folder-has-another-apps-file = Windows Installer indique qu'une autre application a un fichier dans le dossier où se trouve le programme de cette application, et ce fichier est inclus, car l'intégralité de ce dossier est reprise.
# "App Paths" is the name of a Windows registry key; keep it as it is.
core-warning-build-app-paths-folder-left-out = L'entrée App Paths de cette application indique qu'elle cherche dans un dossier qui ne lui appartient pas, ce dossier n'est donc pas inclus. L'application risque de ne pas démarrer sur un PC qui ne dispose pas de son contenu.
core-warning-build-uninstaller-folder-left-out = Le dossier contenant le programme de désinstallation de cette application a été exclu, car Windows Installer indique qu'une autre application y a un fichier.
# { $why } is never shown: it chooses the reason. Translate each reason as the end of the sentence.
core-warning-build-recorded-folder-left-out =
    Un élément enregistré pour cette application désigne un dossier qui a été exclu, car { $why ->
        [unreadable] son chemin sort du lecteur ou passe par un lien, ce qui ne permet pas de le lire en toute sécurité
        [missing] il n'existe pas
        [link] il passe par un lien
        [short-name] il n'est pas écrit sous son vrai nom
        [another-account] il se trouve dans les dossiers d'un autre compte
        [one-account] cette application a été installée pour un seul compte, et ce dossier n'appartient pas à ce compte
        [shared] il contient des éléments partagés, ou fait partie de Windows
        [generic-name] son nom n'indique rien sur son propriétaire
        [publisher] il porte seulement le nom de l'éditeur, et peut contenir ses autres produits
        [holds-the-app] il contient le dossier propre à cette application, et peut contenir davantage
        [another-apps-name] il porte le nom d'une autre application
        [another-apps-record] son enregistrement d'installation appartient à une autre application
        [in-another-apps-folder] il se trouve dans le dossier d'une autre application
       *[another-apps] il appartient à une autre application
    }.
core-warning-build-app-folder-holds-another-app = Une autre application de ce PC est enregistrée comme se trouvant dans le dossier de cette application, et elle est incluse avec celle-ci, car l'intégralité de ce dossier est reprise.
# "Program Files (x86)" is the name of a Windows folder; keep it as Windows calls it.
core-warning-build-from-32-bit-windows = Cette application se trouvait sur un Windows 32 bits. Ses clés de Registre sont organisées comme Windows 64 bits range celles d'une application 32 bits, et le programme d'installation propose Program Files (x86) pour elle.
# { $dll } is a file's name, such as MSVCP140D.dll.
core-warning-build-debug-runtime = Un programme a besoin de { $dll }, une version de débogage du runtime Visual C++. Microsoft ne fournit pas de programme d'installation pour ces versions et n'autorise peut-être pas leur redistribution : ce programme ne s'exécutera donc pas sur un PC sans Visual Studio.
# { $program } is a program's file name, such as Tool.exe.
core-warning-build-single-file-program = { $program } est un programme .NET empaqueté en un seul fichier, qui n'indique pas de quel .NET il a besoin, aucun n'est donc prévu pour lui. S'il en a besoin d'un, il ne démarrera pas sur un PC qui n'en dispose pas.
# { $count } is how many of the app's programs.
core-warning-build-programs-not-inspected =
    { $count ->
        [one] { $count } des programmes de l'application n'a pas été examiné pour déterminer les runtimes dont il a besoin, car elle en compte trop ou ils sont trop volumineux.
       *[other] { $count } des programmes de l'application n'ont pas été examinés pour déterminer les runtimes dont ils ont besoin, car elle en compte trop ou ils sont trop volumineux.
    }
# { $runtime } is a runtime's name, such as Java Runtime Environment.
core-warning-build-runtime-no-installer = L'application a besoin de { $runtime }, et aucun programme d'installation n'est disponible ici pour celui-ci, aucun n'est donc exécuté. Installez-le vous-même sur l'autre PC.
# { $why } is why something was left out, such as "its name says what it holds".
core-warning-build-left-out-of-package = Exclu du package : { $why }.
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-missing-dependency = Impossible de résoudre la dépendance : { $name }
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-dependency-only-here = { $name } n'a été trouvé que via le chemin de recherche de ce PC, et ne fait pas partie de l'application, il n'est donc pas inclus. L'application risque de ne pas démarrer sur un PC qui ne l'a pas.
core-warning-build-driver-detected = Pilote ou composant noyau détecté. Un pilote appartenant à l'application est inclus et réinscrit ; un pilote de fournisseur ou fourni avec Windows n'est pas redistribué et doit être réinstallé à partir du package signé de son fournisseur.
core-warning-build-near-no-longer-offered = Un élément coché dans la liste de ce qui pourrait aussi appartenir à cette application ne figure plus dans cette liste, il n'a donc pas été ajouté. Consultez à nouveau la liste.
# { $app_name } is the name of a part of the app, as the list of apps calls it. { $why } is never shown: it chooses the
# reason. Translate each reason as the end of the sentence.
core-warning-build-near-part-left-out =
    Une partie de ce que { $app_name } déclare comme sien a été exclue : { $why ->
        [unreadable] son chemin sort du lecteur ou passe par un lien, ce qui ne permet pas de la lire en toute sécurité
        [missing] elle n'existe pas
        [link] elle passe par un lien
        [short-name] elle n'est pas écrite sous son vrai nom
        [another-account] elle se trouve dans les dossiers d'un autre compte
        [one-account] cette application a été installée pour un seul compte, et cette partie n'appartient pas à ce compte
        [shared] elle contient des éléments partagés, ou fait partie de Windows
        [generic-name] son nom n'indique rien sur son propriétaire
        [publisher] elle porte seulement le nom de l'éditeur, et peut contenir ses autres produits
        [holds-the-app] elle contient le dossier propre à cette application, et peut contenir davantage
        [another-apps-name] elle porte le nom d'une autre application
        [another-apps-record] son enregistrement d'installation appartient à une autre application
        [in-another-apps-folder] elle se trouve dans le dossier d'une autre application
       *[another-apps] elle appartient à une autre application
    }.
# { $app_name } is the name of a part of the app, as the list of apps calls it.
core-warning-build-near-keys-left-out = Les clés de Registre répertoriées par l'enregistrement d'installation de { $app_name } ont été exclues, car cet enregistrement désigne aussi des éléments appartenant à une autre application.
# "Windows keeps what an app wrote for one account" is Windows' VirtualStore folder.
core-warning-build-virtual-store-program = Un programme se trouvait dans le dossier où Windows conserve ce qu'une application a écrit pour un compte. Windows n'y place jamais de programme : quelque chose d'autre l'y a donc mis, et il a été exclu.
# { $path } is a file or folder, such as C:\Users\Sam\AppData\Roaming\Acme.
core-warning-build-file-left-out = { $path } a été exclu : il passe par un lien sur un lecteur connecté, ou il appartient à un autre compte que celui dont les données sont incluses.
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as D:\Sam\Documents.
core-warning-build-user-folder-not-on-drive = Le dossier { $folder } de { $user } se trouvait dans { $path }, qui n'est pas sur ce lecteur, son contenu n'a donc pas été inclus.
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as \\server\home\Sam\Documents.
core-warning-build-user-folder-not-on-this-pc = Le dossier { $folder } de { $user } se trouvait dans { $path }, qui n'est pas sur un disque de ce PC, son contenu n'a donc pas été inclus.
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-base-unreadable = Impossible de lire { $path } ({ $error }) ; les données d'application qui y sont stockées n'ont pas été incluses.
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-folder-unreadable = Impossible de lire { $path } ({ $error }) ; des données d'application imbriquées peuvent manquer.
# { $error } is what went wrong.
core-warning-build-entry-unreadable = Impossible de lire une entrée du système de fichiers : { $error }
# { $count } is how many links. { $names } lists some of them. { $more } is how many more there are, which are not
# named; when it is 0, the sentence ends after the list.
core-warning-build-links-not-followed =
    { $count ->
        [one] { $count } lien, ou nom qui ouvrirait autre chose, n'a pas été suivi dans ce dossier, sa cible ne figure donc pas dans le programme d'installation : { $names }
       *[other] { $count } liens, ou noms qui ouvriraient autre chose, n'ont pas été suivis dans ce dossier, leurs cibles ne figurent donc pas dans le programme d'installation : { $names }
    }{ $more ->
        [0] .
        [one] { " " }et { $more } autre.
       *[other] { " " }et { $more } autres.
    }
# { $count } is how many folders. { $depth } is how many levels down a folder can be. { $names } lists some of them.
# { $more } is how many more there are, which are not named; when it is 0, the sentence ends after the list.
core-warning-build-folders-too-deep =
    { $count ->
        [one] { $count } dossier se trouve à plus de { $depth } niveaux de profondeur, son contenu ne figure donc pas dans le programme d'installation : { $names }
       *[other] { $count } dossiers se trouvent à plus de { $depth } niveaux de profondeur, leur contenu ne figure donc pas dans le programme d'installation : { $names }
    }{ $more ->
        [0] .
        [one] { " " }et { $more } autre.
       *[other] { " " }et { $more } autres.
    }
# { $error } is what went wrong.
core-warning-build-metadata-unreadable = Impossible de lire les métadonnées du fichier : { $error }
core-warning-build-reserved-name = Le nom de ce fichier est réservé à un périphérique Windows, il n'a donc pas été copié.
# { $error } is what went wrong.
core-warning-build-hash-failed = Impossible de calculer le hachage du fichier : { $error }
core-warning-build-cloud-placeholder = Fichier cloud disponible uniquement en ligne (par ex. Fichiers à la demande OneDrive) sans données locales ; épinglez-le ou téléchargez-le avant la sauvegarde. Signalé, mais non copié.
# { $file_name } is a driver's file name, such as acmeflt.sys.
core-warning-build-driver-file = Le fichier pilote/noyau « { $file_name } » n'est pas copié comme un fichier ordinaire. S'il sert à un service de pilote inscrit pour cette application, le programme d'installation l'inclut et réinscrit automatiquement le pilote ; sinon, réinstallez-le avec le package de pilote signé du fournisseur.
core-warning-build-file-gone = Cet élément était présent lors de l'examen de l'application et a disparu depuis, il ne figure donc pas dans le programme d'installation.
# { $error } is what went wrong.
core-warning-build-file-copy-failed = Impossible de copier le fichier dans le package : { $error }
# { $count } is how many files. { $names } lists them.
core-warning-build-not-looked-through =
    { $count ->
        [one] Ce package contient un fichier qui n'a pas pu être parcouru à la recherche d'indications sur sa provenance : { $names }. Son contenu peut en révéler.
       *[other] Ce package contient des fichiers qui n'ont pas pu être parcourus à la recherche d'indications sur leur provenance : { $names }. Leur contenu peut en révéler.
    }
# { $count } is the most empty folders an installer makes, such as 10,000.
core-warning-build-empty-folders-truncated = Cette application compte plus de { $count } dossiers vides, et seul ce nombre sera recréé.
core-warning-build-credential-by-now = Exclu du package : il ressemble désormais à des informations d'identification.
core-warning-build-no-package-path = Un fichier copiable n'avait pas de chemin relatif dans le package.
# { $file_name } is the runtime installer's file name, such as vc_redist.x64.exe. { $error } is what went wrong.
core-warning-build-prerequisite-path-invalid = Impossible de placer le prérequis inclus { $file_name } : { $error }
# { $runtime } is a runtime's name, such as Microsoft Visual C++ 2015+ Redistributable (x64). { $error } is what went
# wrong. winget is the name of a Windows command; keep it as it is.
core-warning-build-prerequisite-download-failed = Impossible de télécharger { $runtime } ({ $error }) ; le programme d'installation essaiera winget sur l'ordinateur cible.
# { $runtime } is a runtime's name. { $reason } is why the check failed, such as "it is signed by Contoso Ltd, not
# Microsoft".
core-warning-build-prerequisite-signature-failed = Le téléchargement de { $runtime } a échoué à sa vérification de signature, il n'a donc pas été inclus : { $reason }. Le programme d'installation essaiera winget sur l'ordinateur cible.
# { $runtime } is a runtime's name. { $error } is what went wrong.
core-warning-build-prerequisite-hash-failed = { $runtime } a été téléchargé, mais son hachage n'a pas pu être calculé ({ $error }) ; le programme d'installation essaiera plutôt winget sur l'ordinateur cible.
# { $service } is a driver's service name, such as AcmeFlt. { $error } is what went wrong.
core-warning-build-driver-copy-failed = Impossible d'inclure l'image du pilote pour « { $service } » : { $error }
core-warning-build-license-not-copied = Les données de licence n'ont pas pu être copiées (manquantes ou illisibles) ; elles sont enregistrées uniquement sous forme de métadonnées.
core-warning-build-license-file-credential = Un fichier de licence ressemble à des informations d'identification, et a été exclu.
# { $error } is what went wrong.
core-warning-build-license-store-walk-failed = Impossible d'énumérer une partie d'un magasin de licences : { $error }
core-warning-build-license-store-credential = Un fichier d'un magasin de licences ressemble à des informations d'identification, et a été exclu.
# { $error } is what went wrong.
core-warning-build-license-store-file-skipped = Fichier de magasin de licences ignoré : { $error }
