<a id="troubleshooting"></a>

# Résolution des problèmes

Recherchez ci-dessous ce que vous voyez, d'après ce qui se passe ou d'après les mots affichés à l'écran. Les mots entre guillemets sont ceux qu'affiche le programme, parfois avec un nom ou un nombre au milieu.

Si rien ici ne vous aide, consultez [Obtenir de l'aide](#getting-help) à la fin.

<a id="the-window-could-not-open"></a>

## La fenêtre n'a pas pu s'ouvrir

**Ce que vous voyez.** Une boîte de message au lieu de la fenêtre :

- de l'application : « La fenêtre n'a pas pu s'ouvrir », et « Ce programme a besoin de Direct3D 12 pour afficher sa fenêtre. Un ancien pilote graphique ou une machine virtuelle peut ne pas le prendre en charge. »
- d'un programme d'installation : le même message, et « Il peut tout de même installer sans fenêtre : exécutez ce fichier depuis une invite de commandes ouverte en tant qu'administrateur avec --apply, et --target-root suivi d'un dossier pour choisir l'emplacement. »
- d'un programme de désinstallation : le même message, et « Il peut tout de même désinstaller sans fenêtre, depuis une invite de commandes ouverte en tant qu'administrateur : » suivi de la commande exacte.

**Pourquoi.** Les fenêtres sont affichées avec [Direct3D 12](Glossary#direct3d-12), et rien d'autre ne permet de les afficher.

**Que faire.**

- Mettez à jour le pilote graphique du PC.
- Pour un programme d'installation ou de désinstallation, utilisez la ligne de commande. Voir [Ligne de commande](Command-Line).
- L'application n'a pas de ligne de commande. Générez le programme d'installation sur un autre PC : vous pouvez y connecter le lecteur de l'ancien PC. Voir [Déplacer depuis un lecteur connecté](Moving-from-a-Connected-Drive). Ce PC peut aussi lire l'ancien via le réseau. Voir [Déplacer depuis un PC via le réseau](Moving-over-the-Network).

<a id="starting-the-app"></a>

## Démarrage de l'application

| Ce que vous voyez | Signification | Que faire |
| --- | --- | --- |
| L'invite UAC indique « Éditeur inconnu » pour l'application | À partir de la version 0.5.0, l'application est signée. Une application non signée n'est pas le fichier de la version publiée, ou est antérieure à la version 0.5.0. | Téléchargez-la à nouveau depuis la [dernière version](https://github.com/RandyNorthrup/WinAppMigrator/releases/latest), et [vérifiez-la](Getting-Started#check-the-download). |
| SmartScreen indique « Windows a protégé votre ordinateur » | SmartScreen n'a pas souvent vu ce fichier | Choisissez **Informations complémentaires**, vérifiez que l'éditeur est Randy Northrup, puis choisissez **Exécuter quand même**. |
| L'étiquette en haut indique **Échec du chargement**, et la liste indique « Impossible de répertorier les applications » | La liste des applications n'a pas pu être lue | Cliquez sur la flèche circulaire pour recharger la liste. Pour un lecteur connecté, vérifiez qu'il est toujours connecté. |
| L'application voulue ne figure pas dans la liste | Il s'agit d'une application du Microsoft Store, d'une partie masquée d'une autre application, ou d'une application sans entrée dans Paramètres, comme une application portable | Les applications du Store ne peuvent pas être déplacées. Une partie masquée peut être proposée sous **Pourrait aussi en faire partie** lorsque vous choisissez l'application principale. Une application sans entrée dans Paramètres ne peut pas être choisie. |
| « Aucune application correspondante » | La recherche ou le filtre masque tout | Effacez la zone de recherche, et choisissez **Toutes**. |

<a id="building"></a>

## Génération

<a id="picking-a-drive-or-a-disk-image"></a>

### Choix d'un lecteur ou d'une image disque

| Ce que vous voyez | Que faire |
| --- | --- |
| Le lecteur « ne semble pas être un lecteur système Windows (ni \Windows ni \Users). Choisissez la racine du lecteur provenant de l'autre PC. » | Choisissez la racine du lecteur, par exemple `E:\`, et non un dossier qu'il contient. Vérifiez qu'il s'agit du lecteur sur lequel Windows était installé. |
| Le fichier image « n'est pas une image disque que Windows peut monter (.vhd ou .vhdx) » | Choisissez un fichier `.vhd` ou `.vhdx`. Enregistrez d'abord un disque VMware ou VirtualBox dans l'un de ces formats. Pour un point de contrôle Hyper-V, `.avhdx`, exportez le disque de la machine virtuelle. |
| Le fichier image « n'a pas pu être monté : Windows l'a déjà monté, et ce qui l'a monté est peut-être en train d'y écrire. » | L'image est ouverte ailleurs, par exemple montée dans l'Explorateur de fichiers. Choisissez plutôt son lecteur sous **Lecteur connecté…**, ou démontez l'image, puis choisissez-la de nouveau. |
| Le fichier image « ne contient aucun volume lisible sur lequel Windows est installé. Un volume chiffré avec BitLocker doit d'abord être déverrouillé. » | Vérifiez qu'il s'agit d'une image du lecteur sur lequel Windows était installé. Si elle est verrouillée avec [BitLocker](Glossary#bitlocker), montez-la vous-même dans Windows, déverrouillez-la, puis choisissez son lecteur sous **Lecteur connecté…**. |
| Le fichier image « est un disque de différenciation, qui ne contient que ce qui a changé depuis le disque à partir duquel il a été créé » | Fusionnez-le dans le disque à partir duquel il a été créé, avec Modifier le disque dans Hyper-V, puis choisissez ce dernier. |
| Le fichier image « n'a pas pu être monté : » suivi d'une autre raison | La raison est donnée par Windows. Lisez-la, vérifiez que rien d'autre n'utilise l'image, puis réessayez. |

<a id="reading-a-pc-over-the-network"></a>

### Lecture d'un PC via le réseau

La liste complète, y compris ce que le client indique sur l'ancien PC, figure dans [Déplacer depuis un PC via le réseau](Moving-over-the-Network#if-something-goes-wrong).

| Ce que vous voyez | Que faire |
| --- | --- |
| L'ancien PC ne figure pas sous **2. Choisir l'ancien PC** | Vérifiez que le client s'y exécute et affiche un code, et que les deux PC sont sur le même réseau. Entrez ensuite son nom ou son adresse. À travers un routeur, entrez son adresse. Un pare-feu autre que celui de Windows doit laisser passer TCP 47732 et UDP 47731. |
| Un nom « est introuvable sur le réseau » | Entrez plutôt l'adresse de l'ancien PC. |
| « l'ancien PC n'a pas pu être joint : » et une raison | Vérifiez que le client est toujours en cours d'exécution, et que le nom ou l'adresse est correct. |
| « ce n'est pas le code affiché par l'ancien PC. Vérifiez-le, puis réessayez. » | Entrez le code que le client affiche maintenant. Après cinq codes erronés, il en génère un nouveau. Après 20, il n'autorise plus aucune association jusqu'à ce que vous le redémarriez. |
| Sur l'ancien PC, Windows indique que l'éditeur du client est inconnu | C'est normal uniquement sur un Windows 7 qui n'a pas le certificat racine 2020 de Microsoft, et qui ne peut donc pas vérifier la signature du client. Dans ce cas, choisissez **Oui** uniquement pour un client que vous avez vous-même enregistré depuis l'application. Partout ailleurs, l'invite doit indiquer Randy Northrup. Si ce n'est pas le cas, n'exécutez pas cette copie. |
| L'application ne peut plus lire l'ancien PC, après une longue pause | Le client met fin à une connexion à laquelle l'application n'a rien demandé depuis 30 minutes. Cliquez sur **Déconnecter**, puis associez de nouveau les PC. |

<a id="looking-at-the-app"></a>

### Examen de l'application

| Ce que vous voyez | Signification | Que faire |
| --- | --- | --- |
| « Cette application n'a pas pu être examinée : » et une raison | L'examen a échoué. Une raison courante est une entrée sans aucun programme associé, comme un package de pilote. | Cliquez sur **Réessayer**. Si l'échec se reproduit de la même façon, l'entrée n'est peut-être pas une application que WinAppMigrator peut déplacer. |
| « Arrêté. Un programme d'installation peut être généré une fois cette application examinée. » | Vous avez cliqué sur **Arrêter** | Cliquez sur **Réexaminer**. |
| L'examen prend beaucoup de temps | Une application volumineuse, un lecteur lent ou défaillant, ou de nombreux comptes | Si le lecteur est peut-être défaillant, cliquez sur **Arrêter**, créez une [image disque](Glossary#disk-image) de celui-ci, et choisissez-la à la place, car la lecture d'un lecteur défaillant peut l'achever. À partir de la version 0.6.0, choisissez-la avec **Image disque…**. Avec la version 0.5.0, montez-la et choisissez son lecteur. Sinon, patientez. Vous pouvez choisir une autre application en attendant. |
| Une ligne que vous voulez est grisée et cochée | Elle contient le programme principal, ou ce dont il a besoin pour démarrer | Elle est toujours incluse. Voir [les lignes que vous ne pouvez pas décocher](Building-an-Installer#the-groups). |
| Une ligne indique « Également exclu : » | Un élément que vous avez décoché l'a entraînée avec lui | Recochez cet élément si vous voulez conserver cette ligne. |
| « Ce qui reste ne suffirait pas à constituer un package » | Ce que vous avez décoché ne laisse rien qui puisse être généré | Recochez un élément. |
| « Les données de licence comprises dans ce que vous avez exclu sont tout de même incluses » | Les licences dépendent uniquement de l'option **Données de licence et d'activation** | Décochez cette option si vous voulez les exclure. |

<a id="create-installer-stays-greyed-out"></a>

### Le bouton Créer le programme d'installation reste grisé

Il attend que toutes ces conditions soient remplies :

- l'examen de l'application est terminé, pour les options telles qu'elles sont actuellement (« Évaluation de ce qui l'accompagne… » signifie qu'il est encore en train de déterminer ce qu'entraîne le dernier élément que vous avez décoché) ;
- le **Dossier de sortie** est un chemin complet, par exemple `D:\Installers` (« Entrez un chemin d'accès complet, par exemple D:\Installations. ») ;
- il y a assez d'espace sur ce lecteur.

<a id="room"></a>

### Espace

| Ce que vous voyez | Que faire |
| --- | --- |
| « L'espace est insuffisant. » suivi de l'espace dont un lecteur a besoin et dont il dispose | Choisissez un dossier de sortie sur un autre lecteur, ou excluez certains éléments. Ce chiffre est le maximum qu'une génération pourrait occuper, avant compression : il peut donc être supérieur à la taille du programme d'installation. |
| Un lecteur « n'a pas indiqué son espace disponible » | La génération se poursuit sans vérifier ce lecteur. Assurez-vous qu'il dispose d'assez d'espace. |
| Un lecteur FAT32 et une application volumineuse | FAT32 ne peut pas contenir un fichier de 4 Gio. Utilisez un lecteur NTFS ou exFAT. |

<a id="the-done-screen"></a>

### L'écran de fin

| Ce que vous voyez | Que faire |
| --- | --- |
| « Programme d'installation généré, mais » certains fichiers « avaient disparu depuis l'examen de l'application » | Cliquez sur **Réexaminer**, puis sur **Recréer le programme d'installation**, si ces fichiers sont importants. |
| « Programme d'installation généré, mais » certains fichiers « n'ont pas pu être capturés, généralement parce qu'ils étaient verrouillés ou illisibles. » | Fermez l'application, et tout ce qui utilise ses fichiers, puis relancez la génération. **Avertissements** indique de quels fichiers il s'agit. |
| « Programme d'installation généré, mais il contient encore des indications sur sa provenance. » | Lisez les messages en orange en dessous. Voir [le dernier examen](Building-an-Installer#the-last-look). |
| **Avertissements** indique « Impossible de télécharger » un runtime, et « le programme d'installation essaiera winget sur l'ordinateur cible. » | Ce runtime ne se trouve pas dans le programme d'installation : le nouveau PC aura donc besoin d'Internet pour l'obtenir. Relancez la génération en étant connecté à Internet. Voir [Runtimes](Runtimes#at-build-time). |
| « N'utilisez pas ce programme d'installation : il a échoué à sa vérification du hachage. » | Ne l'utilisez pas. Relancez la génération. Si cela se reproduit, le lecteur est peut-être défaillant. |
| « Le programme d'installation a été écrit, mais sa vérification du hachage n'a pas pu s'exécuter » | Ne l'utilisez pas. Relancez la génération. |
| « Génération annulée. Aucun programme d'installation n'a été écrit. » | Vous avez cliqué sur **Annuler**. |
| « Le programme d'installation n'a pas pu être généré. » et une raison | Lisez la raison. **Copier l'erreur** la copie, et **Journal** indique où se trouve le journal. |
| « Impossible de créer le dossier de sortie » | Choisissez un dossier dans lequel vous pouvez écrire. |
| « Impossible de créer un dossier de préparation protégé, rien n'a donc été généré » | La génération n'a pas pu créer son dossier de travail privé à côté du dossier de sortie, ni dans le dossier temporaire de Windows. Choisissez un dossier de sortie sur un autre lecteur. |
| « La génération s'est arrêtée de façon inattendue » | Un problème que rien ne prévoyait s'est produit. « Si elle a laissé un programme d'installation dans le dossier de sortie, ne l'utilisez pas. » Merci de [le signaler](#getting-help). |

<a id="installing"></a>

## Installation

<a id="when-the-installer-opens"></a>

### À l'ouverture du programme d'installation

| Ce que vous voyez | Signification | Que faire |
| --- | --- | --- |
| « Vérification du programme d'installation… » pendant longtemps | Il lit l'intégralité du package pour vérifier son hachage | Patientez. Un programme d'installation volumineux prend un certain temps. |
| « Impossible de lire le programme d'installation : » et « ce programme d'installation est endommagé ou a été modifié après sa génération, rien n'a donc été lu dans son package. Copiez-le à nouveau depuis l'emplacement où il a été généré. » | Le fichier est tronqué ou modifié | Copiez-le à nouveau. |
| « ce programme d'installation conserve son package dans un fichier nommé » suivi d'un fichier `.wamdata`, « dans le même dossier, et ce fichier est absent » | Il manque son fichier de données à un programme d'installation en deux fichiers | Copiez le fichier `.wamdata` dans le même dossier. Voir [Applications volumineuses](Big-Apps). |
| le fichier de données « n'est pas le fichier de données de ce programme d'installation, ou il est endommagé ou a été modifié après sa génération » | Le fichier de données ne correspond pas | Copiez à nouveau les deux fichiers, depuis l'emplacement où ils ont été générés. |
| « Le package de ce programme d'installation ne peut pas être utilisé sur ce PC : » et « cette application ne peut pas s'exécuter sur ce PC, rien n'a donc été modifié » | Les programmes de l'application sont conçus pour un autre processeur, ou un programme indique qu'il nécessite une version plus récente de Windows | Installez sur un PC semblable à l'ancien. Rien n'a été modifié. |
| « Ceci est un stub de programme d'installation WinAppMigrator, et il ne contient aucun package d'application. » | Il s'agit du stub seul, et non d'un programme d'installation généré | Générez un programme d'installation avec l'application, et exécutez celui-ci. |
| « La signature de ce programme d'installation n'est pas valide », avec la raison donnée par Windows | Il a été signé, mais Windows n'accepte pas la signature. Il a peut-être été modifié depuis, ou son certificat n'est pas approuvé sur ce PC. | Ne l'installez pas. Renseignez-vous auprès de la personne qui l'a généré. |
| L'invite UAC indique « Éditeur inconnu » | Le programme d'installation n'est pas signé. C'est normal pour un programme d'installation que vous avez généré. | Exécutez-le uniquement si vous faites confiance à sa provenance. |

<a id="the-install-location"></a>

### L'emplacement d'installation

Lorsque **Installer** et **Aperçu des modifications** sont grisés, un avis en rouge sous **Emplacement d'installation** en indique la raison. La raison la plus courante est que l'application est déjà installée dans ce dossier sur ce PC, comme c'est souvent le cas d'un navigateur web, d'un lecteur multimédia ou d'Office : désinstallez d'abord cette copie, dans Paramètres Windows sous **Applications**, puis rouvrez le programme d'installation, ou entrez un nouveau dossier vide. Les raisons, et ce qu'il faut faire, figurent dans [Installer sur le nouveau PC](Installing-on-the-New-PC#when-the-location-cannot-be-used).

<a id="before-or-during-the-install"></a>

### Avant ou pendant l'installation

| Ce que vous voyez | Signification | Que faire |
| --- | --- | --- |
| « L'espace risque d'être insuffisant ici » | Le lecteur semble manquer d'espace | Libérez de l'espace, ou choisissez un emplacement sur un autre lecteur. |
| « l'espace est insuffisant pour cette opération, rien n'a donc été modifié » | Un lecteur sur lequel il écrit est trop plein | Libérez de l'espace sur ce lecteur, ou choisissez un emplacement d'installation sur un autre lecteur. |
| « Ce que l'installation ferait ici a changé, rien n'a donc été installé. » | Quelque chose a changé sur le PC depuis l'établissement de la liste | Relisez la liste, puis cliquez à nouveau sur **Installer** si elle vous convient. |
| « Une installation antérieure de » une application « n'a jamais abouti. Elle sera d'abord annulée lorsque vous cliquerez sur Installer. » | Une installation précédente a été interrompue | Cliquez sur **Installer**. Il l'annule d'abord. |
| « une autre installation ou désinstallation est en cours. Attendez qu'elle se termine, puis réessayez » | Une seule s'exécute à la fois | Patientez, puis réessayez. |
| Un fichier « porte un nom que cette version de Windows ouvre comme un périphérique et non comme un fichier » | L'application contient un fichier nommé par exemple `aux.h`, que les versions de Windows antérieures à Windows 11 ne peuvent pas contenir | Installez sur Windows 11. |
| « Impossible de créer un dossier de préparation » | Il n'existe aucun emplacement privé, avec assez d'espace, où décompresser le package | Libérez de l'espace sur le lecteur de l'emplacement d'installation ou sur le lecteur système. |
| Un dossier de données comme `%AppData%` est signalé comme étant un lien | Le programme d'installation n'écrit pas à travers un [lien](Glossary#link) | Décochez **Données utilisateur** et **Licence**, et installez sans ces données. |
| « Le dossier ProgramData de ce PC est introuvable, rien n'a donc été installé » | L'enregistrement de Windows qui indique l'emplacement de ProgramData n'a pas pu être lu | Le programme d'installation n'installe rien sans un emplacement sûr où conserver son enregistrement. |

<a id="the-result"></a>

### Le résultat

| Ce que vous voyez | Que faire |
| --- | --- |
| « L'installation a échoué, et ce qu'elle a modifié a été rétabli. » | Lisez les messages en rouge pour en connaître la raison. Le PC est dans l'état où il était, à l'exception des runtimes installés. |
| « L'installation a échoué, et tout ce qu'elle a modifié n'a pas pu être rétabli. » | Lisez ce qui est indiqué. Fermez ce qui pourrait utiliser ces fichiers, puis exécutez de nouveau un programme d'installation : il réessaie d'abord de les rétablir. |
| « Impossible d'installer » un runtime | Le runtime n'a pas pu être installé, l'installation a donc été annulée. Pour DirectX End-User Runtime, le nouveau PC a besoin d'Internet. Voir [Runtimes](Runtimes#with-no-internet-on-the-new-pc). |
| « Une dépendance nécessite un redémarrage pour terminer son installation. Redémarrez Windows, puis lancez l'application. » | Redémarrez, puis démarrez l'application. |
| Le service « s'exécutait en tant que » un compte « sur l'ordinateur source. Son mot de passe ne figure pas dans le programme d'installation, le service est donc configuré en tant que LocalService. » | Ouvrez Services, et définissez le compte et le mot de passe du service, s'il ne fonctionne pas tel quel. |
| Certains fichiers vont « plutôt sous » le dossier d'installation, « dans un dossier nommé _external » | Ce PC conserve Documents ou Parties enregistrées en dehors de votre profil. Déplacez vous-même ces fichiers vers ce dossier. |
| Un élément est « exclu », avec une raison | Les règles l'ont refusé, ou il appartient déjà à une autre application ici. Le reste est installé. Si l'application en a besoin, consultez la raison. |
| **Hachages du package NON VALIDES** ou **Fichiers installés NON VALIDES** | Quelque chose ne correspondait pas. Copiez à nouveau le programme d'installation, et réessayez une fois. |

<a id="after-installing"></a>

### Après l'installation

| Ce que vous voyez | Que faire |
| --- | --- |
| L'application ne démarre pas | Recherchez dans **Dépendances** un runtime marqué **Installer manuellement**, et installez-le. Consultez les avertissements du résultat. |
| Une application provenant de Windows 7 ne démarre pas, et un avertissement lors de la génération a nommé une DLL que « Windows 10 et 11 ne fournissent pas » | Cette DLL a été placée dans les propres dossiers de l'ancien Windows par un autre programme : elle n'a donc pas été incluse. Installez ce qui la fournit, puis redémarrez l'application. Voir [Applications de Windows 7, Vista, 8 ou 8.1](Moving-from-a-Connected-Drive#apps-from-windows-7-vista-8-or-81). |
| L'application démarre comme si elle venait d'être installée pour la première fois | C'est normal avec un programme d'installation généré avec l'option **Données utilisateur** désactivée. Cela se produit aussi lorsque **Données utilisateur** a été décochée lors de l'installation, car les données vont alors sous le dossier d'installation, et lorsque le programme d'installation n'a pas pu déterminer qui était connecté, ce qu'il indique. |
| L'application demande à être activée | Une licence liée à l'ancien PC doit être réactivée. |
| Le programme de mise à jour de l'application pense qu'elle n'est pas installée | Windows Installer n'en a aucun enregistrement ici. Voir [Limites](Limitations). |
| Un type de fichier ou de lien ne s'ouvre pas avec l'application | Une autre application le possédait déjà ici : il a donc été laissé tel quel. Choisissez l'application dans Paramètres, sous Applications par défaut. |
| Une nouvelle entrée `Path` n'est pas prise en compte | Ouvrez une nouvelle fenêtre PowerShell ou d'invite de commandes. Certains programmes ne voient une modification qu'après une déconnexion puis une reconnexion. |

<a id="uninstalling"></a>

## Désinstallation

| Ce que vous voyez | Que faire |
| --- | --- |
| « Une partie se trouve sur un lecteur qui n'est pas connecté. » | Connectez ce lecteur, puis désinstallez à nouveau depuis Paramètres. |
| « Tout n'a pas pu être annulé. Fermez ce qui l'utilise, puis désinstallez à nouveau depuis Paramètres. » | Fermez l'application et tout ce qui utilise ses fichiers. Si un service refuse de s'arrêter, redémarrez le PC. Désinstallez ensuite à nouveau. |
| « ce n'est pas l'ID d'une installation » (avec `--quiet`, en anglais : « this is not an install's id ») | L'ID indiqué à `--uninstall` est incorrect. Copiez-le depuis l'entrée dans Paramètres. Voir [Ligne de commande](Command-Line#finding-the-id). |
| Un fichier nommé `.before-install` est apparu | Il s'agit de l'original d'un fichier que l'installation a remplacé, et que vous aviez modifié depuis. Conservez celui que vous voulez. |
| Un runtime est toujours installé | Les runtimes restent en place, car d'autres programmes peuvent les utiliser. Laissez-le, sauf si vous êtes sûr que rien d'autre ne l'utilise. |

<a id="scripts"></a>

## Scripts

| Ce que vous voyez | Que faire |
| --- | --- |
| « L'opération demandée nécessite une élévation » | Exécutez PowerShell en tant qu'administrateur. |
| Rien ne s'affiche, et `$LASTEXITCODE` n'est pas défini | Redirigez la sortie du programme d'installation, par exemple vers `Out-String`. Voir [Ligne de commande](Command-Line). |
| Code de sortie `2` | Un argument est incorrect, ou `--apply` et `--dry-run` ont été utilisés ensemble. |
| « this package does not say where its app was, so give a --target-root » (ce package n'indique pas où se trouvait son application) | Ajoutez `--target-root` avec un chemin complet. |
| L'application « was in » un dossier, « and that cannot be used » (se trouvait dans un dossier qui ne peut pas être utilisé) | Ajoutez `--target-root` avec un autre dossier. |
| Un dossier « cannot be used: it already has files » « Give --force only if you know it is this app's » (le dossier contient déjà des fichiers) | Choisissez un dossier vide. Utilisez `--force` uniquement pour un dossier dont vous savez qu'il appartient à cette application. |

<a id="language-and-text"></a>

## Langue et texte

- **La langue n'est pas la bonne.** Choisissez-en une autre sous **Langue**, en haut à droite de l'application ou du premier écran d'un programme d'installation. L'application la mémorise ; un programme d'installation la conserve pour cette exécution. Si aucun choix n'a été fait, chacun utilise la première des langues d'affichage de Windows dont il dispose, et sinon l'anglais.
- **Une traduction semble incorrecte.** Tout, sauf l'anglais, a été traduit automatiquement. Sous **Langue**, choisissez **Signaler un problème de traduction**, qui ouvre un formulaire où votre langue est déjà renseignée. Indiquez ce que l'écran affiche et ce qu'il devrait indiquer.
- **Des lettres s'affichent sous forme de cases vides.** La fenêtre utilise les polices fournies avec Windows pour chaque écriture, dans `C:\Windows\Fonts`. Un Windows dont certaines de ces polices ont été supprimées affiche ces lettres sous forme de cases ; les noms d'autres applications de la liste peuvent aussi en avoir besoin.
- **Des mots sont tronqués.** Une étiquette trop longue se termine par `…` ; pointez dessus, ou atteignez-la avec la touche Tab, pour la voir en entier. Signalez-le en indiquant la langue et l'écran, pour que la fenêtre puisse être adaptée.
- **WinAppMigrator Client affiche de l'anglais sur un Windows qui s'écrit de droite à gauche.** La console dans laquelle il s'exécute ne peut ni lier ni ordonner les lettres arabes ou hébraïques : il affiche donc de l'anglais et en indique la raison. `--lang` permet de choisir n'importe laquelle de ses langues.

<a id="getting-help"></a>

## Obtenir de l'aide

1. Trouvez le [journal](Glossary#log). Son chemin est affiché sous **Journal** en cas d'échec, ou sous la forme `Log:` depuis une ligne de commande. Les journaux se trouvent dans `%ProgramData%\WinAppMigrator\logs`. Ils contiennent des chemins et des noms de clés, mais jamais le contenu d'un paramètre, d'un fichier de licence ou d'une variable d'environnement. Ils peuvent tout de même contenir les noms de vos dossiers : lisez-en un avant de le partager.
2. [Ouvrez un ticket (issue)](https://github.com/RandyNorthrup/WinAppMigrator/issues) en indiquant la version, ce que vous avez fait et ce que l'écran affichait. **Copier l'erreur**, sur l'écran de fin de l'application, copie le message.
3. **Ne joignez jamais un programme d'installation à un ticket.** Il peut contenir des données privées.
4. Pour un problème de sécurité, signalez-le plutôt en privé, via les [avis de sécurité de GitHub](https://github.com/RandyNorthrup/WinAppMigrator/security/advisories/new).
