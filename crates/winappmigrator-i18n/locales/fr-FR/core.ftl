# Words the core writes for people: warnings, refusals, errors, and the installer's list of what it sets up.
# Message names start with core-.


## Warnings a package records when it is built, which an installer shows later, maybe on another PC, in that PC's
## language.

# { $count } is how many shortcuts.
core-warning-shortcuts-unreadable =
    { $count ->
        [one] { $count } raccourci de ce lecteur ne désigne aucun fichier, il n'a donc pas pu être déterminé s'il ouvre cette application.
       *[other] { $count } raccourcis de ce lecteur ne désignent aucun fichier, il n'a donc pas pu être déterminé s'ils ouvrent cette application.
    }

## Undoing an install: what an uninstall, or a failed install, says it put back, left, or could not undo. Each line is
## shown on its own, in a list.

# { $app_name } is the app's name.
core-rollback-record-damaged = Une partie de l'enregistrement de { $app_name } n'a pas pu être lue : tout ce qu'elle décrivait est donc toujours en place, et un fichier que l'installation a remplacé risque de ne pas être rétabli.
core-rollback-settings-entry-stays = L'entrée de l'application dans Paramètres reste en place, afin que cette opération puisse être relancée à partir de là.
# { $name } is a runtime's name, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-rollback-prerequisite-stays = { $name } a été installé, ou l'a peut-être été, et reste en place, car d'autres programmes l'utilisent peut-être.
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-drive-not-connected = Ce que cette installation a placé dans { $folder } se trouve sur un lecteur qui n'est pas connecté. Connectez-le, et cette opération pourra être terminée.
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-record-part-unread = Ce que cette installation a placé dans { $folder } reste en place, car cette partie de l'enregistrement n'a pas pu être lue.

## Why a file or folder was left, shown after "was left:" in the messages below them. Each is part of a sentence, so
## starts with a small letter and has no full stop.

# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-being-changed = un élément modifiait { $folder } pendant cette opération, rien de plus n'y a donc été fait. Fermez ce qui le modifie, puis relancez cette opération
# { $folder } is a folder; { $error } is what Windows said went wrong.
core-rollback-reason-not-opened = { $folder } n'a pas pu être ouvert : { $error }
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-no-record = l'emplacement de { $folder } n'a pas été enregistré
# { $folder } is a folder; { $actual } is the folder it now leads to.
core-rollback-reason-moved = { $folder } mène maintenant à { $actual }, et non à l'emplacement où il se trouvait pendant l'installation
# Why a service was not stopped.
core-rollback-reason-did-not-stop = il ne s'est pas arrêté à temps

## Files and folders

# { $path } is a folder; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-folder-left = Le dossier { $path } a été laissé en place : { $reason }.
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-removed = Dossier { $path } supprimé
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-holds-others = Le dossier { $path } reste en place, car il contient des fichiers que cette installation n'y a pas placés.
# { $path } is a folder; { $error } is what Windows said went wrong.
core-rollback-folder-not-removed = Le dossier { $path } n'a pas pu être supprimé : { $error }.
# { $path } is a file; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-file-left = Le fichier { $path } a été laissé en place : { $reason }.
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-folder-gone = { $path } n'est pas rétabli, car le dossier dans lequel il se trouvait a disparu.
# { $path } is a file; { $error } is what went wrong.
core-rollback-partial-not-removed = Un fichier partiel n'a pas pu être supprimé : { $path } ({ $error }).
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-checked = { $path } n'a pas pu être vérifié : { $error }.
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-removed = { $path } supprimé
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-removed = { $path } n'a pas pu être supprimé : { $error }.
# { $path } is a file this install had replaced, and has put back.
core-rollback-file-put-back = { $path } rétabli
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-put-back = { $path } n'a pas pu être rétabli : { $error }.
# Why a file could not be put back.
core-rollback-backup-gone = la sauvegarde a disparu
# { $path } is a file.
core-rollback-no-parent-folder = { $path } n'a pas de dossier parent

## A file that stays. { $why } says why: still-used (something that stays still uses it), changed (it has changed since
## it was installed), or not-asked (it is someone's data, and they did not ask for it to go). { $path } is the file.

core-rollback-file-stays =
    { $why ->
        [still-used] { $path } reste en place, car un élément qui reste l'utilise encore.
        [changed] { $path } reste en place, car il a changé depuis son installation.
       *[not-asked] { $path } reste en place, car sa suppression n'a pas été demandée.
    }
# The file it replaced was to be put beside it, under another name, and every name was taken.
core-rollback-file-stays-no-name-beside =
    { $why ->
        [still-used] { $path } reste en place, car un élément qui reste l'utilise encore, et le fichier qu'il a remplacé n'a pas pu être placé à côté : tous les noms possibles sont déjà pris.
        [changed] { $path } reste en place, car il a changé depuis son installation, et le fichier qu'il a remplacé n'a pas pu être placé à côté : tous les noms possibles sont déjà pris.
       *[not-asked] { $path } reste en place, car sa suppression n'a pas été demandée, et le fichier qu'il a remplacé n'a pas pu être placé à côté : tous les noms possibles sont déjà pris.
    }
# { $name } is the name the file it replaced now has, such as settings.ini.before-install.
core-rollback-file-stays-beside =
    { $why ->
        [still-used] { $path } reste en place, car un élément qui reste l'utilise encore. Le fichier qu'il a remplacé se trouve à côté, sous le nom { $name }.
        [changed] { $path } reste en place, car il a changé depuis son installation. Le fichier qu'il a remplacé se trouve à côté, sous le nom { $name }.
       *[not-asked] { $path } reste en place, car sa suppression n'a pas été demandée. Le fichier qu'il a remplacé se trouve à côté, sous le nom { $name }.
    }
# { $error } is what went wrong.
core-rollback-file-stays-not-beside =
    { $why ->
        [still-used] { $path } reste en place, car un élément qui reste l'utilise encore, et le fichier qu'il a remplacé n'a pas pu être placé à côté : { $error }.
        [changed] { $path } reste en place, car il a changé depuis son installation, et le fichier qu'il a remplacé n'a pas pu être placé à côté : { $error }.
       *[not-asked] { $path } reste en place, car sa suppression n'a pas été demandée, et le fichier qu'il a remplacé n'a pas pu être placé à côté : { $error }.
    }

## The registry and environment variables. { $kind } says what { $name } is: key (a registry key, such as
## HKLM\Software\App), value (a registry value, such as HKLM\Software\App\Version), user-variable or machine-variable
## (an environment variable of the user's or of the whole PC, such as APP_HOME), or user-list or machine-list (a
## variable that is a list of folders, such as Path). { $error } is what Windows said went wrong.

# How a registry key's unnamed value is shown, as Registry Editor shows it.
core-registry-default-value-name = (par défaut)
core-rollback-setting-other-account =
    { $kind ->
        [key] La clé { $name } reste en place, car elle appartient au compte qui a effectué cette installation, qui n'est pas connecté.
        [user-variable] La variable d'environnement utilisateur { $name } reste en place, car elle appartient au compte qui a effectué cette installation, qui n'est pas connecté.
        [machine-variable] La variable d'environnement système { $name } reste en place, car elle appartient au compte qui a effectué cette installation, qui n'est pas connecté.
        [user-list] La variable utilisateur { $name } reste en place, car elle appartient au compte qui a effectué cette installation, qui n'est pas connecté.
        [machine-list] La variable système { $name } reste en place, car elle appartient au compte qui a effectué cette installation, qui n'est pas connecté.
       *[value] La valeur de Registre { $name } reste en place, car elle appartient au compte qui a effectué cette installation, qui n'est pas connecté.
    }
core-rollback-setting-not-reached =
    { $kind ->
        [key] La clé { $name } n'a pas pu être atteinte : { $error }.
        [user-variable] La variable d'environnement utilisateur { $name } n'a pas pu être atteinte : { $error }.
        [machine-variable] La variable d'environnement système { $name } n'a pas pu être atteinte : { $error }.
        [user-list] La variable utilisateur { $name } n'a pas pu être atteinte : { $error }.
        [machine-list] La variable système { $name } n'a pas pu être atteinte : { $error }.
       *[value] La valeur de Registre { $name } n'a pas pu être atteinte : { $error }.
    }
core-rollback-setting-not-read =
    { $kind ->
        [key] La clé { $name } n'a pas pu être lue : { $error }.
        [user-variable] La variable d'environnement utilisateur { $name } n'a pas pu être lue : { $error }.
        [machine-variable] La variable d'environnement système { $name } n'a pas pu être lue : { $error }.
        [user-list] La variable utilisateur { $name } n'a pas pu être lue : { $error }.
        [machine-list] La variable système { $name } n'a pas pu être lue : { $error }.
       *[value] La valeur de Registre { $name } n'a pas pu être lue : { $error }.
    }
core-rollback-setting-changed =
    { $kind ->
        [user-variable] La variable d'environnement utilisateur { $name } reste en place, car elle a changé depuis son installation.
        [machine-variable] La variable d'environnement système { $name } reste en place, car elle a changé depuis son installation.
        [user-list] La variable utilisateur { $name } reste en place, car elle a changé depuis son installation.
        [machine-list] La variable système { $name } reste en place, car elle a changé depuis son installation.
       *[value] La valeur de Registre { $name } reste en place, car elle a changé depuis son installation.
    }
# Said when it is back as it was before the install.
core-rollback-setting-put-back =
    { $kind ->
        [user-variable] Variable d'environnement utilisateur { $name } rétablie
        [machine-variable] Variable d'environnement système { $name } rétablie
        [user-list] Variable utilisateur { $name } rétablie
        [machine-list] Variable système { $name } rétablie
       *[value] Valeur de Registre { $name } rétablie
    }
core-rollback-setting-not-put-back =
    { $kind ->
        [user-variable] La variable d'environnement utilisateur { $name } n'a pas pu être rétablie : { $error }.
        [machine-variable] La variable d'environnement système { $name } n'a pas pu être rétablie : { $error }.
        [user-list] La variable utilisateur { $name } n'a pas pu être rétablie : { $error }.
        [machine-list] La variable système { $name } n'a pas pu être rétablie : { $error }.
       *[value] La valeur de Registre { $name } n'a pas pu être rétablie : { $error }.
    }
# A registry value the install wrote, which is the user's data, and stays unless they asked for it to go.
core-rollback-value-kept-by-request = La valeur de Registre { $name } reste en place, car sa suppression n'a pas été demandée.
core-rollback-key-holds-others = La clé { $name } reste en place, car elle contient des éléments que cette installation n'y a pas placés.
core-rollback-key-removed = Clé { $name } supprimée
core-rollback-key-not-removed = La clé { $name } n'a pas pu être supprimée : { $error }.
# { $kind } is user-list or machine-list.
core-rollback-list-entries-removed =
    { $kind ->
        [user-list] Entrées de cette installation retirées de la variable utilisateur { $name }
       *[machine-list] Entrées de cette installation retirées de la variable système { $name }
    }
core-rollback-list-entries-not-removed =
    { $kind ->
        [user-list] Les entrées de cette installation n'ont pas pu être retirées de la variable utilisateur { $name } : { $error }.
       *[machine-list] Les entrées de cette installation n'ont pas pu être retirées de la variable système { $name } : { $error }.
    }

## Services and drivers. { $kind } is service or driver; { $name } is its name, such as AppUpdater. { $error } and
## { $reason } are what Windows said went wrong.

core-rollback-service-not-read =
    { $kind ->
        [driver] Le pilote { $name } n'a pas pu être lu : { $error }.
       *[service] Le service { $name } n'a pas pu être lu : { $error }.
    }
core-rollback-service-changed =
    { $kind ->
        [driver] Le pilote { $name } reste en place, car il a changé depuis son installation.
       *[service] Le service { $name } reste en place, car il a changé depuis son installation.
    }
core-rollback-service-not-stopped =
    { $kind ->
        [driver] Le pilote { $name } n'a pas pu être arrêté ({ $reason }), ce qu'il utilise risque donc de rester en place jusqu'au redémarrage du PC.
       *[service] Le service { $name } n'a pas pu être arrêté ({ $reason }), ce qu'il utilise risque donc de rester en place jusqu'au redémarrage du PC.
    }
core-rollback-service-stopped-for-this =
    { $kind ->
        [driver] Le pilote { $name } a été arrêté pour cette opération. Relancez-le quand vous en avez besoin.
       *[service] Le service { $name } a été arrêté pour cette opération. Relancez-le quand vous en avez besoin.
    }
core-rollback-service-not-removed =
    { $kind ->
        [driver] Le pilote { $name } n'a pas pu être supprimé : { $reason }.
       *[service] Le service { $name } n'a pas pu être supprimé : { $reason }.
    }
core-rollback-service-marked-for-removal =
    { $kind ->
        [driver] Le pilote { $name } est marqué pour suppression, et disparaîtra au redémarrage du PC ou lorsque ce qui le maintient ouvert le libérera.
       *[service] Le service { $name } est marqué pour suppression, et disparaîtra au redémarrage du PC ou lorsque ce qui le maintient ouvert le libérera.
    }
core-rollback-service-removed =
    { $kind ->
        [driver] Pilote { $name } supprimé
       *[service] Service { $name } supprimé
    }
core-rollback-service-not-restored =
    { $kind ->
        [driver] Le pilote { $name } n'a pas pu être rétabli dans son état d'origine : { $reason }.
       *[service] Le service { $name } n'a pas pu être rétabli dans son état d'origine : { $reason }.
    }
core-rollback-service-restored =
    { $kind ->
        [driver] Paramètres du pilote { $name } rétablis
       *[service] Paramètres du service { $name } rétablis
    }
core-rollback-service-not-restarted = Le service { $name } était en cours d'exécution avant cette installation, et n'a pas redémarré : { $reason }.
# { $account } is the account the service ran as, such as CONTOSO\backup.
core-rollback-service-account-again = Le service { $name } s'exécutait en tant que { $account }. Son mot de passe n'est pas conservé : redéfinissez donc son compte dans Services.

## Firewall rules. { $name } is the rule's name; { $error } is what went wrong.

core-rollback-firewall-not-read = La règle de pare-feu { $name } n'a pas pu être lue : { $error }.
core-rollback-firewall-says-otherwise = La règle de pare-feu { $name } reste en place, car la règle présente sous ce nom indique autre chose.
core-rollback-firewall-changed = La règle de pare-feu { $name } reste en place, car elle a changé depuis son installation.
# The capital "The" is as the app has always said it.
core-rollback-firewall-removed = Règle de pare-feu { $name } supprimée
core-rollback-firewall-not-removed = La règle de pare-feu { $name } n'a pas pu être supprimée : { $error }.

## Scheduled tasks. { $path } is a folder of tasks, and { $name } a task with its folder, such as \App\Nightly Update.
## { $error } and { $reason } are what went wrong.

core-rollback-task-folder-holds-others = Le dossier de tâches { $path } reste en place, car il contient des tâches que cette installation n'y a pas placées.
core-rollback-task-folder-not-removed = Le dossier de tâches { $path } n'a pas pu être supprimé : { $error }.
core-rollback-task-not-read = La tâche { $name } n'a pas pu être lue : { $error }.
core-rollback-task-changed = La tâche { $name } reste en place, car elle a changé depuis son installation.
core-rollback-task-removed = Tâche { $name } supprimée
core-rollback-task-not-removed = La tâche { $name } n'a pas pu être supprimée : { $reason }.
core-rollback-task-put-back = Tâche { $name } rétablie
core-rollback-task-not-put-back = La tâche { $name } n'a pas pu être rétablie : { $reason }.

## When an install cannot be undone at all

# { $error } is what went wrong.
core-rollback-record-unreadable = L'enregistrement de cette installation n'a pas pu être lu, rien n'a donc été annulé : { $error }.
# { $error } is what went wrong.
core-rollback-nothing-undone = Rien n'a été annulé : { $error }.
# One app's part of the message below. { $app_name } is the app's name; { $failures } is what could not be undone, a
# sentence or more; { $folder } is the folder that holds the install's record.
core-rollback-earlier-install-stuck = { $app_name } : { $failures } Si cette installation ne peut jamais être annulée, un administrateur peut y renoncer en supprimant { $folder }.
# Why an install did not start. { $details } is one or more of the message above.
core-rollback-earlier-installs-not-undone = une installation antérieure inachevée n'a pas pu être entièrement annulée, rien n'a donc été installé. Fermez tout ce qui utilise ces éléments, puis réessayez. { $details }
core-rollback-install-busy = cette installation est en cours de modification par un autre programme d'installation ou de désinstallation

## The summary an uninstall writes on the command line, one line each, with what each says below it. { $count } is how
## many.

core-rollback-count-undone = Annulés : { $count }
core-rollback-count-failed = Impossible à annuler : { $count }
# What was left on purpose: it has changed since the install, or it is the user's.
core-rollback-count-kept = Laissés en place : { $count }
# What no uninstall can take back, such as a runtime that was installed, which other programs may use.
core-rollback-count-left-behind = Restants : { $count }

## Which Windows a PC or drive has, as the app shows it beside the drive.

# { $name } is the Windows' name, such as Windows 7 Professional Service Pack 1.
core-system-windows-32-bit = { $name }, 32 bits
# { $name } is the Windows' name, such as Windows 11 Pro.
core-system-windows-64-bit = { $name }, 64 bits
# A drive's Windows, when it only says it is older than Vista.
core-system-windows-xp-or-older = Windows XP ou version antérieure

## Reading a connected drive, or another account's settings. Each is why it could not be done, and some start with a
## small letter, as they follow other words.

core-system-needs-administrator = la lecture d'un lecteur connecté ou des paramètres d'un autre compte nécessite des droits d'administrateur
core-system-no-program-data = le dossier ProgramData de ce PC est introuvable
# { $path } is a file, such as D:\Windows\System32\config\SOFTWARE.
core-system-not-a-hive-file = ce n'est pas un fichier de ruche : { $path }
# { $path } is a file; { $size } is its size in bytes.
core-system-hive-too-large = { $path } fait { $size } octets, ce qui est trop volumineux pour une ruche du Registre, ou plus que ce que les ruches de ce lecteur pourraient totaliser
# { $key } is a registry key, such as Software\Microsoft\Windows\CurrentVersion.
core-system-hive-no-mapping = la ruche hors connexion n'a pas de correspondance pour HKLM\{ $key }
# { $key } is where a drive's registry was to be loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said.
# "reg load" is a command's name.
core-system-hive-not-loaded = échec de reg load { $key } : { $error }
# { $key } is where a drive's registry was loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said. "reg
# unload" is a command's name.
core-system-hive-not-unloaded = échec de reg unload { $key } : { $error }
# { $drive } is a drive, such as D:\.
core-system-not-a-system-drive = { $drive } n'est pas un lecteur système Windows (pas de dossier \Windows).
# { $drive } is a drive, such as C:\. "This PC" is the name of a choice in the app.
core-system-own-system-drive = { $drive } est le lecteur système de ce PC. Choisissez Ce PC pour lire ce PC.
# { $drive } is a drive, such as D:\.
core-system-registry-through-link = { $drive } n'est pas un lecteur système Windows : l'accès à son Registre passe par un lien.

## Warnings a scan records about whose data it read.

# { $account } is an account's name or id, such as alice or S-1-5-21-...-1001.
core-warning-scan-account-not-on-record = Le compte { $account } n'est pas enregistré ici, aucune donnée ni aucun paramètre par utilisateur n'ont donc été inclus.
# { $account } is an account's name, such as CONTOSO\alice.
core-warning-scan-signed-in-account-no-profile = Le compte connecté ici, { $account }, n'a pas de profil enregistré, aucune donnée ni aucun paramètre par utilisateur n'ont donc été inclus.
# { $reason } is why Windows could not say.
core-warning-scan-signed-in-account-unknown = Impossible de déterminer qui est connecté ({ $reason }), aucune donnée ni aucun paramètre par utilisateur n'ont donc été inclus. Choisissez le compte dont les données doivent être incluses.
core-warning-scan-last-account-unknown = Ce lecteur n'indique pas qui s'est connecté en dernier, et il contient plusieurs comptes, aucune donnée ni aucun paramètre par utilisateur n'ont donc été inclus. Choisissez le compte dont les données doivent être incluses.
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-settings-unreadable = Les paramètres de { $account } n'ont pas pu être lus, ils n'ont donc pas été inclus : { $error }
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-classes-unreadable = Les classes COM et les types de fichiers inscrits pour { $account } n'ont pas pu être lus, ils n'ont donc pas été inclus : { $error }

## Paths a scan did not follow. { $count } is how many; { $paths } is some of them, separated by semicolons; { $more } is
## how many more there are, not shown.

core-warning-scan-drive-paths-not-followed =
    { $count ->
        [one] Un chemin enregistré sur ce lecteur n'a pas pu être lu, rien de ce qui s'y trouve n'a donc été inclus. Il se trouve sur un autre disque, n'est pas un chemin complet ou passe par un lien qui mènerait hors du lecteur : { $paths }
       *[other] Certains chemins enregistrés sur ce lecteur n'ont pas pu être lus, rien de ce qui s'y trouve n'a donc été inclus. Chacun d'eux se trouve sur un autre disque, n'est pas un chemin complet ou passe par un lien qui mènerait hors du lecteur : { $paths }
    }
core-warning-scan-drive-paths-not-followed-more =
    Certains chemins enregistrés sur ce lecteur n'ont pas pu être lus, rien de ce qui s'y trouve n'a donc été inclus. Chacun d'eux se trouve sur un autre disque, n'est pas un chemin complet ou passe par un lien qui mènerait hors du lecteur : { $paths } et { $more ->
        [one] { $more } autre
       *[other] { $more } autres
    }
core-warning-scan-network-paths-not-followed =
    { $count ->
        [one] Un chemin enregistré sur ce PC n'a pas pu être lu, rien de ce qui s'y trouve n'a donc été inclus. Soit il ne se trouve pas sur un disque de ce PC, auquel cas l'examiner établirait une connexion, soit il porte un nom qui ouvrirait autre chose : { $paths }
       *[other] Certains chemins enregistrés sur ce PC n'ont pas pu être lus, rien de ce qui s'y trouve n'a donc été inclus. Chacun d'eux soit ne se trouve pas sur un disque de ce PC, auquel cas l'examiner établirait une connexion, soit porte un nom qui ouvrirait autre chose : { $paths }
    }
core-warning-scan-network-paths-not-followed-more =
    Certains chemins enregistrés sur ce PC n'ont pas pu être lus, rien de ce qui s'y trouve n'a donc été inclus. Chacun d'eux soit ne se trouve pas sur un disque de ce PC, auquel cas l'examiner établirait une connexion, soit porte un nom qui ouvrirait autre chose : { $paths } et { $more ->
        [one] { $more } autre
       *[other] { $more } autres
    }

## What a scan with none of an account's data leaves out of a package, in the app's list of what was left out. Each
## has a name, and below it why.

core-account-left-out-shortcuts = Raccourcis dans les dossiers propres à ce compte
# { $count } is how many shortcuts.
core-account-left-out-shortcuts-why =
    { $count ->
        [one] { $count } raccourci, inclus uniquement avec l'option Données utilisateur
       *[other] { $count } raccourcis, inclus uniquement avec l'option Données utilisateur
    }
core-account-left-out-variables = Variables d'environnement de ce compte
# { $count } is how many variables.
core-account-left-out-variables-why =
    { $count ->
        [one] { $count } variable, incluse uniquement avec l'option Données utilisateur
       *[other] { $count } variables, incluses uniquement avec l'option Données utilisateur
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-account-left-out-task = La tâche planifiée { $task }
core-account-task-works-on-account-folder = Elle agit sur un emplacement situé dans les dossiers d'un compte, ou porte le nom de l'un d'eux, et n'est incluse qu'avec l'option Données utilisateur
core-account-task-runs-from-account-folder = Ce qu'elle exécute également depuis les dossiers d'un compte n'est inclus qu'avec l'option Données utilisateur
# { $service } is a service's name, such as AppUpdater.
core-account-left-out-service = Le service { $service }
core-account-service-works-on-account-folder = Il agit sur un emplacement situé dans les dossiers d'un compte, et n'est inclus qu'avec l'option Données utilisateur

## What a package was found to still say of whose PC it came from, after a build with none of an account's data.

# The parts of a package, as the warning below names them.
core-account-part-files = les chemins des fichiers propres à l'application
core-account-part-registry = les valeurs de Registre
core-account-part-settings-entry = l'entrée de l'application dans Paramètres
core-account-part-main-program = l'emplacement de son programme principal
core-account-part-shortcuts = les raccourcis
core-account-part-tasks = les tâches planifiées
core-account-part-services = les services
core-account-part-variables = les variables d'environnement
# One place in the list below. { $place } is one of the parts above, or a file's name; { $count } is how many times.
core-account-mention-place = { $place } ({ $count })
# { $kind } is what was found: account (an account's name), folder (where an account's folder is), computer (the PC's
# name), or id (an account's id). { $places } is a list of where, from the message above.
core-warning-scan-package-mentions =
    { $kind ->
        [account] Ce package mentionne le nom d'un compte dans { $places }. Il a été généré sans aucune donnée de compte, et toute personne qui possède le programme d'installation peut lire ceci.
        [folder] Ce package mentionne l'emplacement du dossier d'un compte dans { $places }. Il a été généré sans aucune donnée de compte, et toute personne qui possède le programme d'installation peut lire ceci.
        [computer] Ce package mentionne le nom du PC dans { $places }. Il a été généré sans aucune donnée de compte, et toute personne qui possède le programme d'installation peut lire ceci.
       *[id] Ce package mentionne l'ID d'un compte dans { $places }. Il a été généré sans aucune donnée de compte, et toute personne qui possède le programme d'installation peut lire ceci.
    }
# { $count } is how many folders.
core-warning-scan-folders-of-others-unreadable =
    { $count ->
        [one] { $count } dossier situé hors de l'application n'a pas pu être lu, des données de l'application qui s'y trouvent peuvent donc manquer.
       *[other] { $count } dossiers situés hors de l'application n'ont pas pu être lus, des données de l'application qui s'y trouvent peuvent donc manquer.
    }

## What a scan leaves out because it looks like a credential, or is a license with the License option off, in the app's
## list of what was left out. Each has a name, and below it why.

core-secret-why-credential = il ressemble à des informations d'identification
core-secret-why-license-option-off = l'option Données de licence et d'activation est désactivée
# { $count } is how many values of a registry key.
core-secret-why-account-values =
    { $count ->
        [one] { $count } valeur, incluse uniquement avec l'option Données utilisateur
       *[other] { $count } valeurs, incluses uniquement avec l'option Données utilisateur
    }
# Shown in place of a registry value's name when its name itself looks like a password or a key.
core-secret-name-holds-credential = (une valeur dont le nom contient des informations d'identification)
# Shown in place of an option's name, for an argument on a command line that has no name.
core-secret-unnamed-argument = un argument contenant des informations d'identification
# { $option } is an option taken out of a command line, such as /password, or the message above. { $place } says
# where the command line was: service, task, shortcut, opener (what opens a file type), opener-unnamed (the same, of a
# type with no name), or uninstall. { $name } is the service, task, shortcut, file type, or app.
core-secret-taken-from =
    { $place ->
        [service] { $option } dans la commande d'exécution de { $name }
        [task] { $option } dans la commande d'exécution de la tâche { $name }
        [shortcut] { $option } dans le raccourci { $name }
        [opener] { $option } dans ce qui ouvre { $name }
        [opener-unnamed] { $option } dans ce qui ouvre un type de fichier
       *[uninstall] { $option } dans la commande de désinstallation de { $name }
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-secret-task-definition-unreadable = La définition de la tâche { $task }, qui n'a pas pu être lue pour être vérifiée
# { $name } is an environment variable's name, such as APP_TOKEN.
core-secret-left-out-variable = La variable d'environnement { $name }
# { $class } is a COM class's id, such as {00000000-0000-0000-0000-000000000000}.
core-secret-left-out-class-server = Ce que la classe { $class } indique exécuter
# { $rule } is a firewall rule's name.
core-secret-left-out-firewall-description = La description de la règle de pare-feu { $rule }

## Licenses a scan found, as the package's report describes each.

# { $name } is a file's name, such as license.lic.
core-license-detail-file = Fichier de licence détecté : { $name }
# { $framework } is a licensing product's name, such as FlexLM / FlexNet.
core-license-detail-store = Magasin de licences { $framework } présent
# { $name } is an environment variable's name, such as LM_LICENSE_FILE.
core-license-detail-variable = Variable d'environnement de licence { $name }
# { $name } is a registry value's name, such as SerialNumber.
core-license-detail-registry-value = Valeur de Registre évoquant des données de licence : { $name }
# Said of a registry value in an account's key, which is not named.
core-license-detail-registry-value-of-an-account = Valeur de Registre évoquant des données de licence
# { $key } is a registry key, such as HKLM\Software\Contoso\License.
core-license-detail-registry-key = Clé de Registre évoquant des données de licence : { $key }

## Warnings a scan records about licenses.

core-warning-scan-license-data = Des données de licence/d'activation ont été détectées. Avec l'option Données de licence et d'activation activée, le programme d'installation restaure ce qui peut être déplacé ; les valeurs propres à l'ordinateur et les magasins liés au matériel sont seulement signalés. WinAppMigrator ne déchiffre jamais rien, ne contourne jamais l'activation et n'émule jamais de dongles.
core-warning-scan-license-machine-bound = Certaines licences sont liées à l'ordinateur (dongle matériel, activation verrouillée sur l'ordinateur ou données chiffrées). Le programme d'installation restaure ce qu'il peut ; elles peuvent tout de même nécessiter une réactivation sur le nouveau PC.

## Warnings a scan records about the registry. What could not be read is not named in them: it is kept beside them.

# { $error } is what Windows said went wrong. App Paths is a registry key's name.
core-warning-scan-app-paths-key-unreadable = Impossible de lire une sous-clé App Paths : { $error }
core-warning-scan-registry-limit = La capture du Registre a atteint la limite de sécurité configurée.
core-warning-scan-registry-subtree-limit = La capture d'une sous-arborescence du Registre a atteint la limite de sécurité configurée.
# { $error } is what Windows said went wrong.
core-warning-scan-registry-key-unreadable = Impossible de lire la clé de Registre : { $error }
core-warning-scan-registry-value-too-large = La valeur de Registre a dépassé la taille de capture maximale configurée.

## Scheduled tasks

core-warning-scan-tasks-folder-through-link = Le dossier des tâches planifiées de ce lecteur passe par un lien, ses tâches n'ont donc pas été lues.
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-warning-scan-task-left-out = La tâche planifiée { $task } a été exclue : elle s'exécute sous un autre compte que celui dont les paramètres sont inclus, d'autres utilisateurs que les administrateurs peuvent modifier son fichier, ou l'application en compte trop pour toutes les conserver.
# { $error } is what Windows said went wrong.
core-warning-scan-tasks-folder-unreadable = Impossible de lire une partie des tâches planifiées : { $error }
# { $path } is a task's file, such as C:\Windows\System32\Tasks\App\Nightly Sync; { $reason } is one of the reasons
# below, or what Windows said went wrong.
core-warning-scan-task-unreadable = Impossible de lire la tâche planifiée { $path } : { $reason }. Si elle exécute l'application, elle n'a pas été incluse.
# Why a task's file could not be read. { $size } is its size in bytes.
core-task-too-large = son fichier fait { $size } octets, ce qui est trop volumineux pour une tâche
# Why a task's file could not be read.
core-task-ends-mid-character = son fichier se termine au milieu d'un caractère
# Why a task's file could not be read. { $error } says where.
core-task-not-utf16 = son fichier n'est pas en UTF-16 valide : { $error }
# Why a task's file could not be read. { $error } says where.
core-task-not-text = son fichier n'est ni en UTF-8 ni en UTF-16 valide : { $error }
# What a package records of how a task is put back. Register-ScheduledTask is a command's name.
core-task-restore-instructions = La restauration utilise Register-ScheduledTask avec la définition XML archivée.

## Reading a program's file. Each is why it could not be read, after the program's name.

# { $reason } is what went wrong.
core-pe-unreadable = il n'a pas pu être lu : { $reason }
core-pe-headers-misaligned = ses en-têtes ne sont pas alignés comme doivent l'être ceux d'un programme
# { $size } is the file's size in bytes.
core-pe-too-large = il fait { $size } octets, ce qui est trop volumineux pour un programme
# PE is the name of the format Windows programs are in.
core-pe-no-headers = il n'a pas d'en-têtes PE

## Warnings a scan records about what a program needs. Which program is kept beside each.

# { $error } is what went wrong.
core-warning-scan-imports-unreadable = La table d'importation n'a pas pu être lue : { $error }
# { $error } is what went wrong.
core-warning-scan-import-name-unreadable = Le nom d'un descripteur d'importation n'a pas pu être lu : { $error }
# { $count } is how many were skipped.
core-warning-scan-legacy-delay-imports =
    { $count ->
        [one] { $count } descripteur de chargement différé hérité (v1) a été ignoré ; seules les importations différées modernes basées sur RVA sont analysées.
       *[other] { $count } descripteurs de chargement différé hérités (v1) ont été ignorés ; seules les importations différées modernes basées sur RVA sont analysées.
    }

## The DLLs a program needs. Which DLL or program is kept beside each warning.

# { $path } is a program, such as C:\Program Files\App\app.exe.
core-dll-program-has-no-folder = l'exécutable n'a pas de répertoire parent : { $path }
# Why a DLL is among the app's files, in the package's list of them.
core-dll-beside-program = DLL située à côté de l'exécutable sélectionné
# { $error } is what went wrong.
core-warning-scan-dependency-unscanned = Impossible d'analyser une dépendance récursive : { $error }
# { $name } is a DLL's name, such as d3dx9_43.dll.
core-warning-scan-dependency-not-in-newer-windows = { $name } se trouvait dans le dossier Windows du PC où était cette application, et Windows 10 et 11 ne le fournissent pas. Un autre programme l'y a installé, il ne fait donc pas partie des fichiers propres à l'application et n'est pas inclus. L'application risque de ne pas démarrer tant que ce qui le fournit n'est pas installé.
# { $error } is what went wrong.
core-warning-scan-dependency-not-hashed = Impossible de calculer le hachage de la dépendance : { $error }
# SystemRoot and windir are the names of environment variables.
core-warning-scan-windows-folder-unknown = Les variables d'environnement SystemRoot/windir ne sont pas disponibles ; la recherche de DLL système est limitée.
# { $error } is what went wrong.
core-warning-scan-dependency-architecture-unreadable = Impossible de lire l'architecture de la dépendance : { $error }

## Services, drivers, shortcuts, and environment variables a scan reads. Where it was reading is kept beside each
## warning.

# { $error } is what Windows said went wrong.
core-warning-scan-services-unreadable = Impossible de lire la clé de Registre des services : { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-drivers-unreadable = Impossible de lire la clé de Registre des services pour la détection des pilotes : { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-queried = Impossible d'interroger les raccourcis : { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-parsed = Impossible d'analyser le résultat de l'interrogation des raccourcis : { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-environment-unreadable = Impossible de lire la clé de Registre de l'environnement système : { $error }

## What a package records of how a driver is put back, which an installer shows. { $kind } is the driver's type as
## Windows names it (kernel, file-system, recognizer, or unknown-type); { $name } is its service's name; { $path } is
## its file. { $signed } is yes when { $signer } names who signed it, and no when that is not known.

core-service-driver-bundled =
    { $signed ->
        [yes] Pilote { $kind } « { $name } » appartenant à l'application ({ $path }, signé par { $signer }). Le programme d'installation inclut son fichier .sys et réinscrit automatiquement le service du pilote (sc create, nécessite une élévation de privilèges), puis vérifie que le service est inscrit.
       *[no] Pilote { $kind } « { $name } » appartenant à l'application ({ $path }, signature inconnue). Le programme d'installation inclut son fichier .sys et réinscrit automatiquement le service du pilote (sc create, nécessite une élévation de privilèges), puis vérifie que le service est inscrit.
    }
core-service-driver-from-vendor =
    { $signed ->
        [yes] Pilote { $kind } « { $name } » de fournisseur ou fourni avec Windows ({ $path }, signé par { $signer }). WinAppMigrator ne redistribue pas les pilotes tiers ; réinstallez-le sur l'ordinateur cible avec le package de pilote signé du fournisseur avant de lancer l'application.
       *[no] Pilote { $kind } « { $name } » de fournisseur ou fourni avec Windows ({ $path }, signature inconnue). WinAppMigrator ne redistribue pas les pilotes tiers ; réinstallez-le sur l'ordinateur cible avec le package de pilote signé du fournisseur avant de lancer l'application.
    }

## Runtimes an app needs, such as the Visual C++ runtime or .NET: how each is provided, as the Dependencies list says.

core-runtime-handling-built-in = Intégré à Windows
core-runtime-handling-ships-with-app = Fourni avec l'application
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-handling-windows-feature = Fonctionnalité Windows : { $feature }
core-runtime-handling-included = Inclus
# { $id } is the runtime's name in winget, such as Microsoft.VCRedist.2015+.x64.
core-runtime-handling-winget = Installé avec winget : { $id }
core-runtime-handling-manual = Installer manuellement

## Downloading a runtime's installer, when an installer is built. Each is why it could not be, and starts with a small
## letter, as it follows other words. { $url } is a web address; { $error } is what went wrong.

core-runtime-download-not-https = téléchargement de { $url } refusé : ce n'est pas du HTTPS
# TLS is what keeps a download private.
core-runtime-tls-failed = échec de l'initialisation TLS : { $error }
core-runtime-download-failed = échec du téléchargement de { $url } : { $error }
core-runtime-download-unreadable = échec de la lecture de { $url } : { $error }
core-runtime-download-empty = le téléchargement de { $url } n'a renvoyé aucune donnée
# { $came } and { $expected } are numbers of bytes.
core-runtime-download-cut-short = le téléchargement de { $url } a été interrompu : { $came } octets reçus sur { $expected }

## Installing a runtime: why it was not installed. Each starts with a small letter, as it follows other words.

core-runtime-no-known-installer = il n'existe aucun programme d'installation connu pour ce runtime
# { $path } is where in the package, such as prerequisites\vc_redist.x64.exe.
core-runtime-payload-misplaced = le package désigne { $path } comme programme d'installation de ce runtime, ce qui n'est pas son emplacement prévu, il n'a donc pas été exécuté
# { $path } is a file.
core-runtime-payload-missing = le programme d'installation inclus est manquant : { $path }
# { $error } is what went wrong.
core-runtime-payload-path-invalid = le chemin du programme d'installation inclus n'est pas valide : { $error }
# { $url } is the web address it can be downloaded from. winget is a program's name.
core-runtime-no-installer-route = aucun programme d'installation inclus ni ID winget ; procurez-le-vous sur { $url }
# { $error } is what went wrong.
core-runtime-no-temp-folder = impossible de créer un dossier dans lequel le décompresser : { $error }
# { $reason } is what the check found.
core-runtime-payload-signature-failed = le programme d'installation inclus a échoué à sa vérification de signature et n'a pas été exécuté : { $reason }
core-runtime-payload-timed-out = le programme d'installation inclus ne s'est pas terminé dans le délai imparti et a été arrêté
# { $error } is what went wrong.
core-runtime-payload-not-started = impossible de démarrer le programme d'installation inclus : { $error }
core-runtime-installer-killed = le programme d'installation a été arrêté sans code de sortie
# { $code } is the number it ended with, such as 1603.
core-runtime-installer-failed = le programme d'installation s'est terminé avec le code { $code }
# { $feature } is a Windows feature's name, such as NetFx3; { $code } is the number DISM ended with. Windows Features is
# the name of a Windows window. Keep the command as it is.
core-runtime-feature-failed = DISM n'a pas pu activer la fonctionnalité Windows { $feature } (code de sortie { $code }) ; la charge utile n'a probablement pas pu être téléchargée. Activez-la via Fonctionnalités de Windows, ou exécutez `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`.
# The same, when DISM gave no number.
core-runtime-feature-failed-no-code = DISM n'a pas pu activer la fonctionnalité Windows { $feature } (code de sortie inconnu) ; la charge utile n'a probablement pas pu être téléchargée. Activez-la via Fonctionnalités de Windows, ou exécutez `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`.
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-feature-timed-out = l'activation de la fonctionnalité Windows { $feature } ne s'est pas terminée dans le délai imparti et a été arrêtée
# { $feature } is a Windows feature's name; { $error } is what went wrong.
core-runtime-feature-no-dism = DISM n'est pas disponible pour activer la fonctionnalité Windows { $feature } : { $error }

## Installing a runtime with winget. { $id } is its name in winget, such as Microsoft.VCRedist.2015+.x64.

# { $error } is what went wrong.
core-runtime-winget-not-looked-for = impossible de rechercher winget : { $error }
core-runtime-winget-not-installed = winget n'est pas installé
# { $path } is where winget is.
core-runtime-winget-misplaced = { $path } ne se trouve pas là où Windows conserve ses propres packages, il n'a donc pas été exécuté
# { $path } is where winget is; { $reason } is what the check found.
core-runtime-winget-signature-failed = { $path } a échoué à sa vérification de signature et n'a pas été exécuté : { $reason }
# { $reason } is one of the two messages above, or what went wrong.
core-runtime-winget-unusable = impossible d'utiliser winget pour installer { $id } : { $reason }
# { $code } is the number winget ended with.
core-runtime-winget-failed = winget install { $id } s'est terminé avec le code { $code }
core-runtime-winget-failed-no-code = winget install { $id } s'est terminé avec un code inconnu
core-runtime-winget-timed-out = winget install { $id } ne s'est pas terminé dans le délai imparti et a été arrêté
# { $error } is what went wrong.
core-runtime-winget-not-started = winget n'est pas disponible pour installer { $id } : { $error }

## What told a scan how an app was installed, as the package's report lists it. The names of installers and files are
## as their makers write them.

# { $path } is an installer's record of what it installed, such as C:\Program Files\App\unins000.dat.
core-source-record-not-believed = { $path } n'a pas été utilisé : d'autres utilisateurs que les administrateurs peuvent le modifier
# { $count } is how many Windows Installer packages the bundle holds.
core-source-burn-bundle =
    { $count ->
        [one] Bundle Burn de { $count } package Windows Installer
       *[other] Bundle Burn de { $count } packages Windows Installer
    }
# { $count } is how many paths Windows Installer recorded.
core-source-msi-database =
    { $count ->
        [one] Base de données Windows Installer : { $count } chemin de clé de composant
       *[other] Base de données Windows Installer : { $count } chemins de clé de composant
    }
core-source-inno-log = Journal de désinstallation Inno Setup (unins000.dat)
# { $count } is how many paths the log records.
core-source-inno-log-paths =
    { $count ->
        [one] Journal de désinstallation Inno Setup : { $count } chemin enregistré
       *[other] Journal de désinstallation Inno Setup : { $count } chemins enregistrés
    }
core-source-nsis = Signature de programme de désinstallation NSIS (Nullsoft)
# { $count } is how many paths the log records.
core-source-nsis-log-paths =
    { $count ->
        [one] NSIS AdvUninstLog uninstall.log : { $count } chemin enregistré
       *[other] NSIS AdvUninstLog uninstall.log : { $count } chemins enregistrés
    }
core-source-squirrel = Disposition Squirrel par utilisateur (Update.exe / app-*)
# { $version } is a folder's name that says the version, such as app-1.2.3.
core-source-squirrel-version = Version actuelle de Squirrel : { $version }
core-source-chocolatey = Métadonnées de package Chocolatey
core-source-msix = Package MSIX / Appx
# { $identity } is a package's name, such as Contoso.App.
core-source-appx-identity = Identité AppxManifest : { $identity }
# { $count } is how many folders.
core-source-msix-data =
    { $count ->
        [one] Données de package MSIX par utilisateur : { $count } emplacement
       *[other] Données de package MSIX par utilisateur : { $count } emplacements
    }
# App Paths is a registry key's name.
core-source-app-paths = Inscription App Paths

## Why a runtime is needed, as the package's report says it.

# { $program } is a program's file name, such as app.exe; { $dll } is a DLL's, such as msvcp140.dll.
core-runtime-program-imports = { $program } importe { $dll }

## What else of an app's a scan finds in the registry: fonts and certificates, and why each is the app's.

# Why a font is among the app's files, in the package's list of them.
core-registry-font-recorded = Une police désignée par l'enregistrement d'installation de l'application
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-named-for-the-app = il porte le nom de l'application ou de son éditeur
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-recorded = l'enregistrement d'installation de l'application le désigne

## Who is signed in, for the Whose Data choice. Each of the first six is why it could not be told, and starts with a
## small letter, as it is shown inside other words. An account is shown by its id, such as S-1-5-21-...-1001.

# { $error } is what Windows said.
core-profile-desktop-owner-not-said = Windows n'a pas indiqué à qui appartient ce bureau : { $error }
# { $error } is what went wrong.
core-profile-desktop-owner-not-asked = Windows n'a pas pu être interrogé sur le propriétaire de ce bureau : { $error }
core-profile-signed-in-account-no-profile = le compte connecté ici, { $account }, n'a pas de profil enregistré
core-profile-desktop-not-a-person = le bureau appartient ici à { $account }, qui n'est pas le compte d'une personne
core-profile-nobody-signed-in-running-account-no-profile = personne n'est connecté ici, et le compte qui exécute ceci, { $account }, n'a pas de profil enregistré
core-profile-nobody-signed-in-running-not-a-person = personne n'est connecté ici, et { $account }, qui exécute ceci, n'est pas le compte d'une personne
# SID is the name Windows gives an account's id.
core-profile-nobody-signed-in-no-sid = personne n'est connecté ici, et ce compte n'a pas de SID
# { $accounts } is a list of accounts.
core-profile-several-desktops = plusieurs comptes ont un bureau dans cette session : { $accounts }
# Under the Whose Data choice. { $reason } is one of the messages above.
core-profile-signed-in-unknown = Impossible de déterminer qui est connecté ({ $reason }).
# Under the Whose Data choice, when there are accounts to choose from. { $reason } is one of the messages above.
core-profile-signed-in-unknown-choose = Impossible de déterminer qui est connecté ({ $reason }). Choisissez le compte dont les données doivent être incluses.
