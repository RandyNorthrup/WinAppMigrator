# Words of the installer that every built installer carries, which runs on the PC an app is moved to and speaks its
# language, and of the uninstaller it leaves behind for Settings. Message names start with installer- for the
# installer's window, installer-cli- for its command line, and uninstaller- for the uninstaller.
#
# The command lines (installer-cli-, uninstaller-cli-) are always shown in English, as scripts read what they print.
# Translate them all the same. Keep their command-line flags, such as --apply and --target-root, exactly as they are.


## The installer's window: its title and header

installer-window-title = Programme d'installation de WinAppMigrator
# The window's title in the taskbar. { $app_name } is the name of the app it installs.
installer-window-title-app = Installer { $app_name }
installer-tagline = Réinstalle une application capturée sur un autre PC
# What an app is called when its package gives it no name.
installer-unnamed-app = Application

## When the installer's window cannot open. Each is a paragraph of a message box.

# { $error } is what went wrong, as Windows or the graphics driver says it.
installer-window-failed = La fenêtre du programme d'installation n'a pas pu s'ouvrir : { $error }
installer-window-failed-without-one = Il peut tout de même installer sans fenêtre : exécutez ce fichier depuis une invite de commandes ouverte en tant qu'administrateur avec --apply, et --target-root suivi d'un dossier pour choisir l'emplacement.

## Checking the installer when it opens

installer-checking = Vérification du programme d'installation…
installer-stub-no-package = Ceci est un stub de programme d'installation WinAppMigrator, et il ne contient aucun package d'application. Générez un programme d'installation avec WinAppMigrator, puis exécutez ici le fichier .exe produit.
installer-check-stopped = La vérification du programme d'installation s'est arrêtée.
# { $error } is what went wrong, as Windows says it.
installer-locate-failed = Impossible de localiser le programme d'installation : { $error }
# { $error } is what went wrong reading it.
installer-read-failed = Impossible de lire le programme d'installation : { $error }
# { $error } is why, such as that the app needs a newer Windows.
installer-cannot-be-used-here = Le package de ce programme d'installation ne peut pas être utilisé sur ce PC : { $error }

## The app, where it goes, and whether the installer is signed

installer-captured-from = Capturée sur
installer-install-location = Emplacement d'installation
# The title of the folder picker.
installer-choose-location = Choisir l'emplacement d'installation
installer-folder-has-files = Ce dossier contient déjà des fichiers. Les fichiers portant le même nom sont remplacés, et sont rétablis si vous désinstallez.
installer-room-needed = Espace requis
# { $shortfall } says which drive is short of room, such as "C: needs up to 824.0 MB for the app's files, and has 300.0 MB free".
installer-room-short = L'espace risque d'être insuffisant ici : { $shortfall }. Le package peut être décompressé sur un autre lecteur, mais les fichiers de l'application doivent tout de même tenir ici.
# { $app_name } is the name of an app whose earlier install was cut short.
installer-interrupted = Une installation antérieure de { $app_name } n'a jamais abouti. Elle sera d'abord annulée lorsque vous cliquerez sur Installer.
# Beside "Installer", for whether it proves who made it.
installer-signature = Programme d'installation
installer-signature-checking = Vérification de sa signature…
installer-signature-unsigned = Non signé : éditeur inconnu
# { $publisher } is the name on the installer's certificate, such as "Fabrikam, Inc.".
installer-signature-signed-by = Signé par { $publisher }
installer-signature-unnamed = Signé par un éditeur sans nom
# { $reason } is Windows' own word for what is wrong, such as HashMismatch.
installer-signature-invalid = La signature de ce programme d'installation n'est pas valide ({ $reason }). Il a peut-être été modifié depuis sa signature.

## Why the install location cannot be used. Install and Preview Changes stay off while one of these is shown.

# { $app_name } is the name of the app being installed.
installer-location-app-already-there = { $app_name } est déjà installé dans ce dossier, cette copie ne peut donc pas y être installée. Pour l'installer ici, désinstallez d'abord la copie présente sur ce PC, dans Paramètres Windows sous Applications, puis rouvrez ce programme d'installation. Ou entrez un nouveau dossier vide ci-dessus.
# { $other_app } is the name of another app Windows lists as installed in the folder. { $app_name } is the name of the app being installed.
installer-location-other-app-there = { $other_app } est installé dans ce dossier, { $app_name } ne peut donc pas y être installé. Entrez un nouveau dossier vide ci-dessus. Ou, si vous n'avez plus besoin de { $other_app }, désinstallez-le d'abord, dans Paramètres Windows sous Applications, puis rouvrez ce programme d'installation.
installer-location-has-files = Ce dossier contient déjà des fichiers qui n'ont pas été installés par WinAppMigrator. Ils appartiennent peut-être à une autre application, rien n'est donc installé par-dessus. Entrez un nouveau dossier vide ci-dessus.
# { $reason } is why, such as that it cannot be the Windows folder.
installer-location-refused = Ce dossier ne peut pas être utilisé : { $reason }. Entrez un autre dossier ci-dessus.

## What the install sets up beyond its files

installer-beyond-files = Ce qui est configuré en plus des fichiers
# Under the heading above. It is part of what a person agrees to when they click Install.
installer-beyond-files-agreed = Tout ceci est effectué avec des droits d'administrateur. Si l'installation échoue, ce qu'elle a modifié est rétabli, et vous êtes informé de tout ce qui n'a pas pu l'être. Vous pouvez aussi désinstaller l'application plus tard depuis Paramètres, sous Applications installées. Les fichiers qu'elle remplace sont alors rétablis, et les runtimes qu'elle installe restent en place.
installer-beyond-files-none = Rien : l'installation copie uniquement des fichiers, ainsi que des paramètres dans les propres clés de Registre de l'application.
# { $problem } is why, such as that the app cannot run on this PC.
installer-cannot-go-here = Ce package ne peut pas être installé à cet emplacement : { $problem }
installer-working-out = Évaluation de ce que l'installation configurerait à cet emplacement…
# Said as the reason after "This package cannot go to this location:".
installer-list-stopped = l'évaluation de ce que l'installation ferait s'est arrêtée
# A heading over a list of what else the package asked for and will not get. { $count } is how many.
installer-more-left-out =
    { $count ->
        [one] { $count } autre élément a été exclu
       *[other] { $count } autres éléments ont été exclus
    }

## Options

installer-options = Options
installer-user-data = Données utilisateur
installer-user-data-detail = Activé : les paramètres, Documents et Parties enregistrées vont dans les dossiers habituels de la personne connectée. Désactivé : tout est placé sous le dossier d'installation, et rien n'est configuré pour s'exécuter à partir de là.
# { $account } is the name of the account on the other PC whose data this is.
installer-user-data-whose = Ceci contient les données de { $account }.
installer-license = Licence
installer-license-detail = Fichiers d'activation et de licence, et valeurs de Registre qui contiennent des licences. Certaines licences peuvent nécessiter une réactivation ici.

## Dependencies: what else the app needs, and how each is provided

installer-dependencies = Dépendances
installer-dependencies-detail = Ce dont cette application a besoin, et comment chaque élément est fourni.
installer-dependencies-none = Aucune dépendance d'exécution supplémentaire n'a été détectée.
# { $feature } is the name of an optional part of Windows, such as NetFx3.
installer-dependency-windows-feature = Fonctionnalité Windows : { $feature }
installer-dependency-included = Inclus
# { $id } is the package's name in winget, such as Microsoft.VCRedist.2015+.x64.
installer-dependency-winget = Installé avec winget : { $id }
installer-dependency-manual = Installer manuellement
installer-dependency-built-in = Intégré à Windows
installer-dependency-ships-with-app = Fourni avec l'application

## Buttons, and what is said beside them

installer-install = Installer
installer-preview = Aperçu des modifications
installer-cancel = Annuler
installer-close = Fermer
installer-open-folder = Ouvrir le dossier d'installation
# { $error } is what went wrong, as Windows says it.
installer-open-folder-failed = Impossible d'ouvrir le dossier : { $error }
installer-location-blocks = Installer et Aperçu des modifications restent désactivés tant que l'emplacement d'installation ne peut pas être utilisé. La raison est indiquée ci-dessus, sous celui-ci.
installer-package-blocks = Installer est désactivé, car ce package ne peut pas être installé à cet emplacement. La raison est indiquée ci-dessus.
installer-list-changed = Ce que l'installation ferait ici a changé, rien n'a donc été installé. Vérifiez la liste ci-dessous, puis cliquez à nouveau sur Installer si elle vous convient.
installer-cancelled-early = Annulé. Rien n'a été modifié.

## How far an install has got, beside its progress bar

# { $step } is one of the steps below. { $done } and { $total } are how many of its items are done, and how many there are.
installer-step-counted = { $step } : { $done } sur { $total }…
# { $step } is one of the steps below, when it does not count its items.
installer-step-uncounted = { $step }…
installer-step-getting-ready = Préparation…
installer-step-cancelling = Annulation après l'étape en cours…
installer-step-putting-back = Rétablissement de ce qui a été modifié
installer-step-unpacking = Décompression
installer-step-checking-package = Vérification du package
installer-step-recovering = Annulation d'une installation antérieure inachevée
# What a preview is doing, whatever step it is on.
installer-step-previewing = Évaluation de ce qui changerait
installer-step-runtimes = Installation des runtimes
installer-step-files = Copie des fichiers
installer-step-services = Configuration des services
installer-step-drivers = Configuration des pilotes
installer-step-registry = Écriture des paramètres du Registre
installer-step-license = Restauration des données de licence
installer-step-tasks = Configuration des tâches planifiées
installer-step-shortcuts = Création des raccourcis
installer-step-environment = Définition des variables d'environnement
installer-step-settings-entry = Ajout de l'application à Paramètres
installer-step-verifying = Vérification de ce qui a été installé
installer-step-undoing = Annulation

## When an install cannot go ahead or stops

installer-carries-no-package = Ce programme d'installation ne contient aucun package.
installer-stopped-unexpectedly = L'installation s'est arrêtée de façon inattendue.
# { $reason } is what went wrong, as the program says it.
installer-stopped-unexpectedly-because = L'installation s'est arrêtée de façon inattendue : { $reason }. Si elle avait commencé, relancer un programme d'installation annule d'abord ce qu'elle a fait.
# { $error } is what went wrong reading it.
installer-package-read-failed = Impossible de lire le package : { $error }
installer-no-programdata = Le dossier ProgramData de ce PC est introuvable, rien n'a donc été installé : une installation y conserve l'enregistrement qui permet de l'annuler.
# { $error } is why, such as that there is nowhere only administrators can get into.
installer-staging-failed = Impossible de créer un dossier de préparation : { $error }
# { $error } is what went wrong unpacking it.
installer-unpack-failed = Impossible de décompresser le package : { $error }
installer-staging-nowhere = il n'existe aucun emplacement accessible uniquement aux administrateurs où décompresser le package
# { $failure } is a folder and what went wrong there, such as "C:\ProgramData\WinAppMigrator: Access is denied.".
installer-staging-nowhere-look-at = il n'existe aucun emplacement accessible uniquement aux administrateurs où décompresser le package. L'emplacement à vérifier est { $failure }

## What came of an install, or of a preview

installer-result = Résultat de l'installation
installer-result-preview = Aperçu : rien n'a encore été modifié
installer-result-installed = L'application a été installée et a réussi sa vérification du hachage.
installer-result-installed-without-left-out = L'application a été installée et a réussi sa vérification du hachage, sans les éléments indiqués comme exclus ci-dessous.
installer-result-not-all-put-back = L'installation a échoué, et tout ce qu'elle a modifié n'a pas pu être rétabli. Ce qui reste est indiqué ci-dessous, et la prochaine installation réessaiera.
installer-result-cancelled-put-back = L'installation a été annulée, et ce qu'elle a modifié a été rétabli.
installer-result-failed-put-back = L'installation a échoué, et ce qu'elle a modifié a été rétabli.
installer-result-not-clean = L'installation ne s'est pas terminée correctement. Examinez les problèmes ci-dessous.
installer-result-changes-put-back = Modifications rétablies
installer-result-changes-made = Modifications effectuées
installer-result-changes-planned = Modifications prévues
installer-result-warnings = Avertissements
installer-result-errors = Erreurs
installer-result-package-valid = Hachages du package valides
installer-result-package-invalid = Hachages du package NON VALIDES
installer-result-files-valid = Fichiers installés valides
installer-result-files-invalid = Fichiers installés NON VALIDES
installer-result-reboot = Une dépendance nécessite un redémarrage pour terminer son installation. Redémarrez Windows, puis lancez l'application.
# A heading over the list of what was changed. { $count } is how many.
installer-result-changes = Modifications ({ $count })
# After the first part of a long list. { $count } is how many more there are.
installer-result-changes-more =
    { $count ->
        [one] et { $count } autre.
       *[other] et { $count } autres.
    }
# Over the path of the install's log file.
installer-log = Journal

## The installer's command line. Shown in English: scripts read it. A { $count } is plain digits, with no separators.

installer-cli-stub = Stub de programme d'installation WinAppMigrator — il ne contient aucun package d'application.
installer-cli-stub-how = Générez un programme d'installation avec l'application WinAppMigrator, puis exécutez le fichier .exe produit sur l'ordinateur cible.
installer-cli-usage-title = Programme d'installation auto-extractible WinAppMigrator.
installer-cli-usage = Exécutez-le sans argument pour ouvrir le programme d'installation graphique, ou :
installer-cli-usage-preview = aperçu de la réinstallation : --dry-run [--target-root <DOSSIER>]
installer-cli-usage-apply = réinstallation :              --apply [--target-root <DOSSIER>]
installer-cli-no-package = le programme d'installation ne contient pas de package intégré
installer-cli-no-programdata = le dossier ProgramData de ce PC est introuvable, rien n'a donc été installé : une installation y conserve l'enregistrement qui permet de l'annuler
# { $path } is the folder asked for, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-folder-has-files = { $path } ne peut pas être utilisé : { $problem }. N'indiquez --force que si vous savez qu'il s'agit du dossier de cette application
installer-cli-no-location = ce package n'indique pas où se trouvait son application, indiquez donc un --target-root
# { $path } is where the app was on the other PC, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-location-refused = l'application de ce package se trouvait dans { $path }, et cet emplacement ne peut pas être utilisé : { $problem }. Indiquez un --target-root
# { $summary } is how much room it needs where, such as "up to 824.0 MB on C: (356.0 GB free)".
installer-cli-room = Espace : { $summary }
# { $note } is something to know about the install.
installer-cli-note = Remarque : { $note }
# { $count } is how many things it sets up beyond its files.
installer-cli-sets-up = Éléments configurés : { $count }
installer-cli-applied = Réinstallation appliquée.
installer-cli-previewed = Aperçu de la réinstallation effectué.
# { $count } is how many changes the install logged.
installer-cli-changes-logged = Modifications journalisées : { $count }
# { $count } is how many warnings.
installer-cli-warnings = Avertissements : { $count }
# { $count } is how many errors.
installer-cli-errors = Erreurs : { $count }
# { $count } is how many things the package asked for and did not get.
installer-cli-refused = Éléments refusés : { $count }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-package-valid = Hachages du package valides : { $valid }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-reinstall-valid = Hachages de la réinstallation valides : { $valid }
# { $id } is the install's id, such as wam-0f3a….
installer-cli-install-id = ID d'installation : { $id }
installer-cli-put-back = L'installation a échoué, ce qu'elle a modifié a donc été rétabli.
# { $path } is the log file.
installer-cli-log = Journal : { $path }
# { $path } is the log file. Printed before an error.
installer-cli-error-log = journal : { $path }
# { $error } is what went wrong.
installer-cli-error = erreur : { $error }

## The uninstaller's window

uninstaller-window-title = Désinstaller
# The window's title, and its heading. { $app_name } is the name of the app it uninstalls.
uninstaller-window-title-app = Désinstaller { $app_name }
uninstaller-tagline = Annule ce que son programme d'installation a fait
uninstaller-reading = Lecture de l'enregistrement de cette installation…
uninstaller-stopped-unexpectedly = La désinstallation s'est arrêtée de façon inattendue.
uninstaller-what-will-be-removed = Éléments qui seront supprimés
uninstaller-install-location = Emplacement d'installation
uninstaller-files = Fichiers
uninstaller-registry-values = Valeurs du Registre
uninstaller-services = Services
uninstaller-drivers = Pilotes
uninstaller-scheduled-tasks = Tâches planifiées
uninstaller-environment = Environnement
uninstaller-plan-stays = Les fichiers que vous avez ajoutés ou modifiés restent en place, avec les dossiers qui les contiennent.
# { $count } is how many files the install replaced.
uninstaller-plan-coming-back =
    { $count ->
        [one] { $count } fichier remplacé par cette installation est rétabli, ou placé à côté d'un fichier qui a changé depuis.
       *[other] { $count } fichiers remplacés par cette installation sont rétablis, ou placés à côté d'un fichier qui a changé depuis.
    }
# { $runtimes } is a list of runtimes, such as "Microsoft Visual C++ Redistributable, .NET Desktop Runtime 8".
uninstaller-plan-runtimes = Les runtimes qu'elle a installés restent en place, car d'autres programmes peuvent les utiliser : { $runtimes }.
uninstaller-options = Options
uninstaller-remove-user-data = Supprimer aussi les données utilisateur et les données de licence
# { $count } is how many items of the app's settings, data, and license there are.
uninstaller-remove-user-data-detail =
    { $count ->
        [one] { $count } élément de ses paramètres, données et licence. Il reste en place, sauf si cette option est activée. Ce qu'il a remplacé est rétabli, ou placé à côté de lui s'il a changé depuis.
       *[other] { $count } éléments de ses paramètres, données et licence. Ils restent en place, sauf si cette option est activée. Ce qu'ils ont remplacé est rétabli, ou placé à côté d'eux s'ils ont changé depuis.
    }
uninstaller-uninstall = Désinstaller
uninstaller-cancel = Annuler
uninstaller-close = Fermer
# { $done } and { $total } are how many of the install's items are undone, and how many there are.
uninstaller-step-counted = Annulation de l'installation : { $done } sur { $total }…
uninstaller-step = Annulation de l'installation…
uninstaller-done = L'application a été désinstallée.
uninstaller-done-except = L'application a été désinstallée, à l'exception des éléments indiqués ci-dessous.
uninstaller-waits-for-drive = Une partie se trouve sur un lecteur qui n'est pas connecté. Connectez-le, puis désinstallez à nouveau depuis Paramètres.
uninstaller-not-all-undone = Tout n'a pas pu être annulé. Fermez ce qui l'utilise, puis désinstallez à nouveau depuis Paramètres.
# Over the path of the uninstall's log file.
uninstaller-log = Journal
uninstaller-undone = Annulé
uninstaller-left-in-place = Laissé en place
uninstaller-could-not-be-undone = Impossible à annuler
# After the first part of a long list. { $count } is how many more there are.
uninstaller-and-more =
    { $count ->
        [one] et { $count } autre.
       *[other] et { $count } autres.
    }
# Said as the reason when there is no record of the install to read.
uninstaller-no-programdata = le dossier ProgramData de ce PC est introuvable, il n'y a donc aucun enregistrement à lire

## When the uninstaller's window cannot open. Each is a paragraph of a message box, followed by a command to type.

uninstaller-window-failed-title = Programme de désinstallation de WinAppMigrator
# { $error } is what went wrong, as Windows or the graphics driver says it.
uninstaller-window-failed = La fenêtre du programme de désinstallation n'a pas pu s'ouvrir : { $error }
uninstaller-window-failed-quiet = Il peut tout de même désinstaller sans fenêtre, depuis une invite de commandes ouverte en tant qu'administrateur :
# Stands in for this program's own path in that command, if it cannot be found.
uninstaller-this-program = ce programme

## The uninstaller's command line (--quiet). Shown in English: scripts read it.

# { $app_name } is the name of the app.
uninstaller-cli-uninstalling = Désinstallation de { $app_name }.
# { $path } is the log file.
uninstaller-cli-log = Journal : { $path }
# { $error } is what went wrong.
uninstaller-cli-error = erreur : { $error }
