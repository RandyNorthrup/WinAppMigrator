# Words of WinAppMigrator Client, the console program run on the old PC, the one an app is moving off, so that
# WinAppMigrator on the new PC can read it over the network. Message names start with client-.
#
# Its window is a console, which on Windows 7 is 80 columns wide. Each line break in a message is one on the screen:
# keep every line shorter than 80 characters, and break lines where your language needs to, not only where English
# does.


## Starting

# Shown only in English, in place of a right-to-left language the console cannot show. { $language } is that
# language's name in English, such as Arabic.
client-console-rtl-fallback = Cette console ne peut pas afficher la langue { $language }, le client utilise donc l'anglais.
# The client copies itself to a folder in Program Files before it starts. { $error } is what Windows said went wrong.
client-safe-folder-failed =
    WinAppMigrator Client n'a pas pu démarrer depuis un dossier que seuls les
    administrateurs peuvent modifier : { $error }
client-press-enter-to-close = Appuyez sur Entrée pour fermer cette fenêtre.
# The first line of its screen. { $version } is the client's version, such as 0.6.2.
client-heading = WinAppMigrator Client { $version }
client-intro =
    Ce programme permet à WinAppMigrator, sur votre nouveau PC, de lire les
    applications de ce PC via votre réseau. Il ne modifie rien ici : il lit un
    cliché instantané du lecteur Windows, qu'il supprime quand vous l'arrêtez.
# { $error } is why: one of the messages under "Why it stopped" below, or what Windows said.
client-stopped = Le client s'est arrêté : { $error }
# The caretaker is a copy of the client with no window that cleans up after it. { $error } is what Windows said.
client-caretaker-failed =
    Si cette fenêtre est fermée sans arrêter le client, le cliché instantané
    et les règles de pare-feu restent en place jusqu'à la prochaine exécution
    du client sur ce PC ({ $error }).
# Shown while the snapshot is taken, followed on the same line by client-snapshot-taken. Keep the quotes: they keep
# the space before it.
client-snapshot-taking = Création d'un cliché instantané du lecteur Windows...{ " " }
client-snapshot-taken = terminé.
# The old PC answers the new one's search for it on the network. { $error } is what Windows said went wrong.
client-discovery-failed =
    Ce PC n'apparaîtra pas automatiquement ({ $error }).
    Sur le nouveau PC, entrez plutôt le nom ou l'adresse de ce PC.

## This PC and its pairing code. The three lines are lined up in two columns, behind two spaces; keep the quotes, which
## keep those spaces.

# { $computer_name } is this PC's name, such as OLD-PC.
client-this-pc = { "  " }Ce PC :               { $computer_name }
# { $windows_name } is this PC's Windows, such as Windows 7 Professional Service Pack 1. { $bits } is 32 or 64.
client-windows = { "  " }Windows :             { $windows_name }, { $bits } bits
# { $code } is six digits in two groups, such as 482 913.
client-pairing-code = { "  " }Code d'association :  { $code }
# Network PC... is the button of that name in WinAppMigrator: use the name it has there in your language.
client-pairing-steps =
    Sur votre nouveau PC, dans WinAppMigrator, choisissez PC du réseau... et
    entrez ce code. Fermez d'abord l'application que vous déplacez, pour que
    ses fichiers restent tels qu'elle les a laissés.
    Pour arrêter, appuyez sur Entrée ici ou fermez cette fenêtre.

## While it runs. { $from } is the network address of the PC that connected, such as 192.168.1.20, or "unknown" when
## it could not be told, which the [unknown] variant words.

client-paired =
    Associé à { $from ->
        [unknown] un autre PC
       *[address] { $from }
    }. Celui-ci lit ce dont il a besoin...
# { $count } is how many requests it answered.
client-builder-done =
    { $from ->
        [unknown] Un autre PC
       *[address] { $from }
    } a terminé ({ $count ->
        [one] { $count } requête traitée
       *[other] { $count } requêtes traitées
    }).
# { $error } is what went wrong, often in Windows' words.
client-link-ended =
    La connexion avec { $from ->
        [unknown] un autre PC
       *[address] { $from }
    } a pris fin : { $error }
client-wrong-code =
    { $from ->
        [unknown] Un autre PC
       *[address] { $from }
    } a essayé un code qui n'est pas le bon.
# { $error } is what went wrong, often in Windows' words.
client-pair-failed =
    { $from ->
        [unknown] Un autre PC
       *[address] { $from }
    } n'a pas pu s'associer : { $error }
client-too-many-wrong-codes =
    Trop de codes erronés ont été essayés, aucun PC ne peut donc plus
    s'associer à celui-ci. Quelque chose sur votre réseau tente peut-être de
    deviner le code. Pour associer un PC, arrêtez le client, relancez-le, puis
    entrez le nouveau code qu'il affiche.
# { $code } is six digits in two groups, such as 482 913.
client-new-code = Trop de codes erronés. Le nouveau code d'association est : { $code }

## Stopping

client-stopping = Arrêt : suppression du cliché instantané et des règles de pare-feu...
client-stop-done = Terminé.

## Why it stopped, shown after "The client stopped: " (client-stopped), and written to follow it.

client-already-running = le client est déjà en cours d'exécution sur ce PC, ou termine le nettoyage après un client qui a été fermé. Utilisez celui-là, ou patientez un instant et relancez celui-ci.
# { $drive } is the drive Windows is on, such as C:. { $details } is what Windows said, in its own words.
client-snapshot-failed = Windows n'a pas créé de cliché instantané de { $drive } : { $details }
# { $details } is what Windows Firewall said, in its own words.
client-firewall-failed = le Pare-feu Windows n'a pas autorisé le client à écouter : { $details }

## What it takes, shown for --help. The options, such as --lang, are typed as they are and are not translated; keep
## their descriptions lined up.

client-usage =
    WinAppMigrator Client accepte ces options :
    
      --lang <langue>    Utiliser cette langue, par ex. it-IT, si le client la
                         propose, au lieu de la langue d'affichage de Windows.
      --code <code>      Démarrer avec ce code d'association à six chiffres,
                         pour un script qui démarre aussi WinAppMigrator sur
                         le nouveau PC.
      --help             Afficher cette aide.
