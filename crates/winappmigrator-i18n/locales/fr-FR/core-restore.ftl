# Words the core writes for people as it installs an app from a package: why an install will not start, the list of
# what it did (or for a preview, what it would do) that the installer window shows, its warnings, the record it keeps of
# each install so that it can be undone, and the core's errors. Message names start with core-.
#
# The list of what an install did is short and plain, one line for each thing, as the installer window lists them:
# "copied", "restored", "would copy". Words such as --force and --apply are command-line options, and LocalService,
# SYSTEM, ProgramData, and %AppData% are as Windows names them: keep them as they are.


## Errors: what went wrong, when the core cannot go on. Each is shown on its own, or after words that say what was being
## done.

core-error-list-changed = ce que cette installation ferait a changé depuis qu'elle a été acceptée, rien n'a donc été installé
# { $path } is a file or folder, such as C:\Program Files\App\app.exe, or words that name one, such as "the installer".
# { $error } is what Windows said went wrong.
core-error-io = échec d'E/S pour { $path } : { $error }
# { $detail } is what went wrong.
core-error-registry = échec de la lecture du Registre : { $detail }
# { $path } is a program file. { $detail } is what was wrong with it.
core-error-pe = échec de l'analyse PE de { $path } : { $detail }
# { $detail } is what was wrong, already in words.
core-error-invalid-input = entrée non valide : { $detail }
# { $detail } is what went wrong.
core-error-serialization = échec de la sérialisation du rapport : { $detail }
# { $detail } is what went wrong.
core-error-archive = échec de l'opération d'archive : { $detail }
# { $detail } is what went wrong.
core-error-network = échec de l'opération réseau : { $detail }
# { $detail } is what went wrong.
core-error-time-format = échec de la mise en forme de l'heure : { $detail }
core-error-cancelled = l'opération a été annulée
# What is said when part of the program stopped unexpectedly and gave no reason.
core-error-panic-no-reason = aucune raison n'a été donnée

## The record of each install, kept so that it can be undone.

# { $kind } is a number that says what type of registry value it is.
core-journal-unknown-value-type = type de valeur de Registre inconnu { $kind }
core-journal-not-elevated = privilèges non élevés
core-journal-needs-admin = l'installation nécessite des droits d'administrateur, afin de conserver son enregistrement là où d'autres ne peuvent pas le modifier
# { $path } is a folder.
core-journal-not-an-install-folder = ce n'est pas le dossier d'une installation : { $path }
# { $path } is the file the record is kept in.
core-journal-not-written = l'enregistrement de cette installation n'a pas pu être écrit : { $path }
# { $path } is a folder.
core-journal-path-escapes-root = le chemin sort de la racine prévue : { $path }
# { $path } is a folder.
core-journal-nothing-exists = rien de tout cela n'existe sur ce PC : { $path }
core-journal-busy = une autre installation ou désinstallation est en cours. Attendez qu'elle se termine, puis réessayez
# Said of a file that was to get back the permissions it had.
core-journal-no-folder = il n'a pas de dossier
# Said of the permissions a file had, which were kept to give back to it.
core-journal-permissions-unreadable = ce qui a été conservé n'a pas pu être lu
# { $folder } is the folder of one install's record.
core-journal-unreadable-empty = l'enregistrement de l'installation dans { $folder } est inutilisable : il est vide
# { $folder } is the folder of one install's record.
core-journal-unreadable-no-header = l'enregistrement de l'installation dans { $folder } est inutilisable : il ne commence pas par ce qu'il concerne
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-version = l'enregistrement de l'installation dans { $folder } est inutilisable : une autre version de ce programme l'a écrit
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-install = l'enregistrement de l'installation dans { $folder } est inutilisable : il indique appartenir à une autre installation
# { $id } is an install's id, such as wam-1a2b3c.
core-journal-not-an-install-id = ce n'est pas l'ID d'une installation : { $id }

## Why an install will not start, or stopped before it changed anything.

core-restore-apply-and-dry-run = --apply et --dry-run ne peuvent pas être indiqués ensemble
# { $path } is the folder that was chosen to install in.
core-restore-location-not-full-path = l'emplacement d'installation doit être un chemin d'accès complet : « { $path } »
# { $problem } says what is wrong with the folder, already in words. { $path } is the folder.
core-restore-location-unusable = cet emplacement d'installation ne peut pas être utilisé : { $problem } : « { $path } »
core-restore-needs-admin-plan = le plan de restauration comprend des modifications réservées aux administrateurs (Registre, service, tâche planifiée, raccourci ou cible protégée)
# { $path } is the folder that was chosen to install in.
core-restore-needs-admin-target = la racine cible nécessite des privilèges élevés : { $path }
core-restore-needs-admin-user-data = la restauration des données utilisateur dans ProgramData ou dans d'autres emplacements protégés nécessite des privilèges élevés
# { $error } is why the install stopped. { $failed } is what could not be put back, as sentences of their own.
core-restore-not-all-put-back = { $error }. Ce qui avait été fait n'a pas pu être entièrement rétabli : { $failed }
# { $error } is what went wrong.
core-restore-nowhere-to-keep-record = il n'existe aucun emplacement sûr où conserver l'enregistrement qui permettrait d'annuler cette installation, rien n'a donc été installé : { $error }
# { $path } is a file the app would have put there.
core-restore-already-exists = { $path } existe déjà, rien n'a donc été installé. Utilisez --force pour le remplacer
# { $root } is a folder the app has files for. { $link } is that folder, or one it is in.
core-restore-root-under-link = cette application a des éléments à restaurer dans { $root }, et { $link } est un lien vers un autre emplacement, que d'autres peuvent modifier, rien n'a donc été installé. Si ce qui va là est constitué de données utilisateur ou de données de licence, installez-la sans ses données utilisateur ni ses données de licence. S'il s'agit d'un raccourci dans un dossier Démarrage, remettez ce dossier à son emplacement d'origine
core-restore-hash-check-failed = le package a échoué à sa vérification du hachage et rien n'a été installé
# { $detail } is the first thing found wrong, already in words.
core-restore-hash-check-failed-because = le package a échoué à sa vérification du hachage et rien n'a été installé : { $detail }
# { $path } is a file or folder.
core-restore-refuse-records-folder = écriture refusée à l'emplacement où les installations conservent leurs enregistrements : { $path }
# { $path } is a file or folder.
core-restore-refuse-outside = écriture refusée en dehors de l'emplacement d'installation et des dossiers de données connus : { $path }
# { $path } is a file.
core-restore-no-parent-folder = la destination n'a pas de dossier parent : { $path }
# { $path } is where a file was to go.
core-restore-folder-in-the-way = un dossier se trouve là où ce fichier doit aller : { $path }
# { $path } is a file in the package.
core-restore-file-changed = { $path } ne correspond plus au contenu du package, il n'a donc pas été installé
# { $path } is a file in the package.
core-restore-source-is-link = { $path } est un lien, et il ne doit pas être suivi
# { $path } is a file in the package.
core-restore-source-became-link = { $path } est devenu un lien pendant son ouverture
# Why a service did not start, after "did not start:".
core-restore-service-stopped-by-itself = il s'est de nouveau arrêté de lui-même
# Why part of the app was left out, after "was left out:".
core-restore-com-class-someone-elses = cette classe COM existe déjà sur ce PC, pour un autre élément

## What an install did, one line for each thing, or for a preview, what it would do. { $source } is a file in the
## package, and { $destination } and { $path } are where it goes on this PC. { $key } is a registry key, such as
## HKLM\Software\App.

# { $package_valid } is true or false, and { $restore_valid } is true, false, or not_run: keep them as they are.
core-restore-verification-result = vérification post-restauration package_valid={ $package_valid } restore_valid={ $restore_valid }
# { $name } is a service's name.
core-restore-started-service = a démarré le service { $name }
core-restore-would-copy = copierait { $source } -> { $destination }
core-restore-copied = a copié { $source } -> { $destination }
core-restore-would-create-empty-folder = créerait le dossier vide { $path }
core-restore-created-empty-folder = a créé le dossier vide { $path }
# { $value } is the name of a value in the key.
core-restore-would-restore-registry-value = restaurerait la valeur de Registre { $key } { $value }
core-restore-would-restore-registry-key = restaurerait la clé de Registre { $key }
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-would-add-firewall-rule = ajouterait la règle de pare-feu { $name } pour { $program }
# { $name } is a firewall rule's name.
core-restore-firewall-rule-already-there = la règle de pare-feu { $name } existe déjà
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-added-firewall-rule = a ajouté la règle de pare-feu { $name } pour { $program }
# { $name } is a service's name, { $command } what it runs, and { $account } the account it runs as, such as LocalService.
core-restore-would-restore-service = restaurerait le service { $name } -> { $command } (s'exécute en tant que { $account })
# { $name } is a runtime, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-restore-prerequisite-present = prérequis déjà présent : { $name }
# { $name } is a runtime that Windows has as a Windows feature.
core-restore-would-install-prerequisite-feature = installerait le prérequis { $name } (fonctionnalité Windows)
# { $name } is a runtime whose installer is in the package.
core-restore-would-install-prerequisite-bundled = installerait le prérequis { $name } (programme d'installation inclus)
# { $name } is a runtime that would come through winget, which is a program's name.
core-restore-would-install-prerequisite-winget = installerait le prérequis { $name } (winget)
# { $name } is a runtime.
core-restore-installed-prerequisite = a installé le prérequis { $name }
# { $name } is a runtime.
core-restore-installed-prerequisite-reboot = a installé le prérequis { $name } (redémarrage requis)
core-restore-would-restore-license = restaurerait la licence { $source } -> { $destination }
core-restore-restored-license = a restauré la licence { $source } -> { $destination }
core-restore-would-restore-license-registry = restaurerait la licence dans le Registre { $key }
# { $name } is a driver's name.
core-restore-driver-not-restored = pilote non restauré automatiquement : { $name }
# { $name } is a driver's name.
core-restore-would-install-driver = installerait le pilote { $name } -> { $destination }
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-would-restore-task = restaurerait la tâche planifiée { $name }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-startup-shortcut = restaurerait le raccourci de démarrage { $shortcut } -> { $target }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-shortcut = restaurerait le raccourci { $shortcut } -> { $target }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-would-restore-variable =
    { $scope ->
        [user] restaurerait la variable d'environnement utilisateur { $name }
       *[machine] restaurerait la variable d'environnement système { $name }
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-left-variable =
    { $scope ->
        [user] a laissé la variable d'environnement utilisateur { $name } telle quelle : rien n'y appartient à cette application
       *[machine] a laissé la variable d'environnement système { $name } telle quelle : rien n'y appartient à cette application
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-restored-variable =
    { $scope ->
        [user] a restauré la variable d'environnement utilisateur { $name }
       *[machine] a restauré la variable d'environnement système { $name }
    }
core-restore-set-uninstaller = a défini le programme de désinstallation pour { $key }
core-restore-placed-driver-image = a placé l'image du pilote { $source } -> { $destination }
# { $name } is a driver's name.
core-restore-configured-driver = a configuré le pilote { $name }
# { $name } is a driver's name.
core-restore-created-driver = a créé le pilote { $name }
# { $name } is a driver's name.
core-restore-started-driver = a démarré le pilote { $name }
core-restore-ensured-registry-key = a vérifié la présence de la clé de Registre { $key }
# { $value } is the name of a value in the key.
core-restore-restored-registry-value = a restauré la valeur de Registre { $key } { $value }
# "(Default)" is how Windows names the value a key has with no name: write it as Registry Editor does.
core-restore-restored-registry-default = a restauré la valeur de Registre { $key } (par défaut)
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-configured-service = a configuré le service { $name } (s'exécute en tant que { $account })
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-created-service = a créé le service { $name } (s'exécute en tant que { $account })
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-restored-task = a restauré la tâche planifiée { $name }
core-restore-restored-shortcut = a restauré le raccourci { $path }

## Warnings an install gives, and why it failed. They are shown after it, on the PC it ran on.

core-warning-restore-earlier-not-trusted = L'enregistrement d'une installation inachevée a été laissé tel quel, car d'autres peuvent le modifier. Tout ce que cette installation avait modifié est toujours en place.
# { $app_name } is the app's name. { $count } is how many changes were put back.
core-warning-restore-earlier-undone =
    { $count ->
        [one] Une installation antérieure de { $app_name } n'avait jamais abouti, elle a donc d'abord été annulée. { $count } modification a été rétablie.
       *[other] Une installation antérieure de { $app_name } n'avait jamais abouti, elle a donc d'abord été annulée. { $count } modifications ont été rétablies.
    }
# { $app_name } is the app's name.
core-warning-restore-earlier-never-finished = Une installation antérieure de { $app_name } n'a jamais abouti. L'installation l'annule d'abord.
core-warning-restore-user-data-real-locations = Les données utilisateur sont restaurées dans les emplacements réels %AppData%/%ProgramData% de cet ordinateur, et non dans la racine cible.
# { $error } is what went wrong.
core-warning-restore-commit-failed = L'enregistrement de cette installation n'a pas pu être finalisé : { $error }
# { $name } is a service's name.
core-warning-restore-service-still-starting = Le démarrage du service { $name } a été demandé et n'est pas terminé. Il peut encore aboutir. Le service démarre aussi de lui-même au prochain démarrage de ce PC.
# { $name } is a service's name. { $reason } is why it did not start, as Windows said it.
core-warning-restore-service-not-started = Le service { $name } est installé mais n'a pas démarré : { $reason }. L'application risque de ne pas fonctionner tant qu'il n'a pas démarré. Windows réessaiera au prochain démarrage de ce PC.
# { $name } is a service's name. { $reason } is why it could not be stopped.
core-warning-restore-service-not-stopped = Le service { $name } n'a pas pu être arrêté au préalable : { $reason }. Il risque de continuer à exécuter l'ancien programme jusqu'au prochain démarrage de ce PC.
core-warning-restore-install-cancelled = L'installation a été annulée.
core-warning-restore-preview-cancelled = L'aperçu a été annulé.
# { $item } is what was left out, such as a file or a service's name. { $reason } is why, already in words.
core-warning-restore-left-out = { $item } a été exclu : { $reason }.
# { $error } is what went wrong.
core-warning-restore-verification-failed = La vérification post-restauration n'a pas pu s'exécuter : { $error }
# { $runner } says what relies on the file: a service, a scheduled task, a firewall rule, a shortcut, or a registry value.
# { $path } is the file. { $name } is the name of what relies on it. { $refusal } is why the file was not replaced,
# already in words.
core-warning-restore-program-would-run =
    { $runner ->
        [service] Ce package n'a pas à remplacer { $path }, et le service { $name } en dépend, rien n'a donc été installé : { $refusal }
        [task] Ce package n'a pas à remplacer { $path }, et la tâche planifiée { $name } en dépend, rien n'a donc été installé : { $refusal }
        [firewall] Ce package n'a pas à remplacer { $path }, et la règle de pare-feu { $name } en dépend, rien n'a donc été installé : { $refusal }
        [shortcut] Ce package n'a pas à remplacer { $path }, et le raccourci { $name } en dépend, rien n'a donc été installé : { $refusal }
       *[registry] Ce package n'a pas à remplacer { $path }, et la valeur de Registre { $name } en dépend, rien n'a donc été installé : { $refusal }
    }
core-warning-restore-destination-exists = La destination existe ; utilisez --force pour la remplacer.
# { $path } is a file. { $error } is what went wrong.
core-warning-restore-file-not-written = Impossible d'écrire { $path } : { $error }
core-warning-restore-not-a-folder = Un élément autre qu'un dossier se trouve à l'emplacement de ce dossier vide.
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-not-looked-at = Impossible d'examiner { $path } : { $error }
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-folder-not-made = Impossible de créer { $path } : { $error }
# Why part of the app was left out, after "was left out:".
core-warning-restore-registration-exists = ce PC possède déjà cet élément, configuré différemment, et une installation ne le reprend pas
core-warning-restore-registry-review = Les valeurs de Registre ont été restaurées à partir des données structurées archivées, avec les chemins d'application connus réécrits vers la racine cible.
# { $error } is what went wrong.
core-warning-restore-firewall-unreadable = Les règles du Pare-feu Windows n'ont pas pu être lues : { $error }
# Why a firewall rule was left out, after "was left out:".
core-warning-restore-firewall-rule-exists = une règle de ce nom pour ce programme existe déjà, configurée différemment, et une installation ne la modifie pas
# { $name } is a runtime.
core-warning-restore-prerequisite-unknown = { $name } : il n'existe aucun programme d'installation connu pour ce runtime.
# { $name } is a runtime.
core-warning-restore-prerequisite-needs-admin = L'installation de { $name } nécessite une élévation de privilèges ; relancez le programme d'installation en tant qu'administrateur.
# { $name } is a runtime.
core-warning-restore-prerequisite-reboot = { $name } a été installé, mais un redémarrage est nécessaire pour terminer l'installation.
# { $error } is what went wrong.
core-warning-restore-skipped-not-recorded = Impossible d'enregistrer un runtime qui a été ignoré : { $error }
# { $name } is a runtime. { $reason } is why it was skipped, already in words.
core-warning-restore-prerequisite-skipped = { $name } : { $reason }
# { $name } is a runtime. { $reason } is why it could not be installed. { $url } is a web address to get it from.
core-warning-restore-prerequisite-failed = Impossible d'installer { $name } : { $reason }. Procurez-le-vous sur { $url }.
# { $error } is what went wrong.
core-warning-restore-runtime-not-recorded = Impossible d'enregistrer un runtime installé : { $error }
# { $error } is what went wrong.
core-warning-restore-license-source-invalid = Le chemin de la licence empaquetée n'est pas valide : { $error }
core-warning-restore-license-needs-admin = La restauration des données de licence dans un emplacement protégé nécessite une élévation de privilèges.
core-warning-restore-license-exists = La destination de la licence existe ; utilisez --force pour la remplacer.
# { $error } is what went wrong.
core-warning-restore-license-failed = Impossible de restaurer les données de licence : { $error }
core-warning-restore-driver-needs-admin = L'inscription du pilote nécessite une élévation de privilèges ; relancez le programme d'installation en tant qu'administrateur.
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-warning-restore-task-runs-as-system = La tâche planifiée { $name } s'exécute en tant que SYSTEM, avec tous les droits sur ce PC.
core-warning-restore-shortcut-exists = Le raccourci existe ; utilisez --force pour le remplacer.
core-warning-restore-shortcut-all-users = Un ou plusieurs raccourcis restaurés ciblent le Bureau public ou un emplacement commun du menu Démarrer.
core-warning-restore-environment-refresh = Les modifications des variables d'environnement peuvent ne prendre effet que dans un nouveau processus, un nouvel interpréteur de commandes ou une nouvelle session.
# { $name } is a service's name. { $account } is the account it ran as on the PC it came from.
core-warning-restore-service-account = Le service { $name } s'exécutait en tant que { $account } sur l'ordinateur source. Son mot de passe ne figure pas dans le programme d'installation, le service est donc configuré en tant que LocalService. S'il ne fonctionne pas, définissez son compte et son mot de passe dans Services.
# { $name } is a runtime.
core-warning-restore-prerequisite-pending-reboot = { $name } n'est pas encore inscrit ; il devrait être actif après un redémarrage.
# { $name } is a runtime.
core-warning-restore-prerequisite-not-verified = { $name } a signalé une réussite, mais ne s'est pas inscrit ; l'installation a peut-être échoué.
# { $error } is what went wrong.
core-warning-restore-driver-source-invalid = Le chemin du pilote inclus n'est pas valide : { $error }
# { $error } is what went wrong.
core-warning-restore-driver-not-read = Impossible de lire les paramètres actuels du pilote : { $error }
core-warning-restore-driver-exists = Le service du pilote existe ; utilisez --force pour le reconfigurer.
core-warning-restore-driver-image-exists = L'image du pilote existe ; utilisez --force pour la remplacer.
core-warning-restore-driver-hash-bundled = L'image du pilote incluse ne correspond pas à son hachage enregistré, elle n'a donc pas été installée.
# { $error } is what went wrong.
core-warning-restore-driver-not-placed = Impossible de placer l'image du pilote : { $error }
core-warning-restore-driver-hash-placed = L'image du pilote a changé après avoir été placée, elle n'a donc pas été inscrite.
# { $error } is what went wrong.
core-warning-restore-key-not-created = Impossible de créer la clé de Registre : { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-read = Impossible de lire la valeur que ceci remplacerait : { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-written = Impossible d'écrire la valeur de Registre : { $error }
core-warning-restore-value-exists = La valeur de Registre existe ; utilisez --force pour la remplacer.
# { $error } is what went wrong.
core-warning-restore-environment-key-not-created = Impossible de créer la clé de Registre de l'environnement : { $error }
core-warning-restore-variable-exists = La variable d'environnement existe ; utilisez --force pour la remplacer.
# { $error } is what went wrong.
core-warning-restore-variable-not-written = Impossible d'écrire la variable d'environnement : { $error }
core-warning-restore-value-kind-missing = Le type de la valeur de Registre est manquant.
# { $error } is what went wrong.
core-warning-restore-service-not-read = Impossible de lire les paramètres actuels du service : { $error }
core-warning-restore-service-exists = Le service existe ; utilisez --force pour le reconfigurer.
# { $error } is what went wrong.
core-warning-restore-task-not-queried = Échec de l'interrogation de la tâche planifiée : { $error }
core-warning-restore-task-exists = La tâche planifiée existe ; utilisez --force pour la remplacer.
# { $error } is what went wrong.
core-warning-restore-task-folder-not-looked-at = Impossible d'examiner le dossier de la tâche : { $error }
# { $error } is what went wrong.
core-warning-restore-shortcut-not-put-back = Le raccourci n'a pas pu être créé, et celui qu'il remplaçait n'a pas pu être rétabli : { $error }
# { $error } is what went wrong.
core-warning-restore-command-not-started = Échec du démarrage de la commande : { $error }
# { $output } is what the command wrote, as it wrote it.
core-warning-restore-command-failed = Échec de la commande : { $output }
# { $error } is what went wrong.
core-warning-restore-powershell-not-started = Échec du démarrage de PowerShell : { $error }
