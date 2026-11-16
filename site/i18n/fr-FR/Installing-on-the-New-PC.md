<a id="installing-on-the-new-pc"></a>

# Installer sur le nouveau PC

Le programme d'installation tient en une seule fenêtre. Il se vérifie lui-même, vous montre où il va installer l'application et tout ce qu'il va configurer, et n'installe que lorsque vous cliquez sur **Installer**. Cette page le parcourt de haut en bas, avec les mots affichés à l'écran.

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/installer-dark.png">
    <img alt="Le programme d'installation de Fabrikam Data Sync, indiquant qu'il n'est pas signé, l'emplacement d'installation, l'espace requis et ce qu'il configure en plus des fichiers" src="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/installer-light.png" width="600">
  </picture>
</p>

<a id="before-you-start"></a>

## Avant de commencer

- **Copiez l'intégralité du programme d'installation.** Si la génération vous a fourni un fichier `.wamdata` en plus du fichier `.exe`, copiez les deux dans le même dossier, et conservez leurs noms. Voir [Applications volumineuses](Big-Apps).
- **N'exécutez que des programmes d'installation de confiance.** Un programme d'installation s'exécute avec tous les [droits d'administrateur](Glossary#administrator-rights). Traitez-le comme n'importe quel autre programme : exécutez-le uniquement si vous l'avez généré vous-même, ou si vous l'avez obtenu d'une personne de confiance. Voir [Confidentialité et sécurité](Privacy-and-Security).
- **Vous avez besoin de droits d'administrateur** sur ce PC, et de [Direct3D 12](Glossary#direct3d-12) pour la fenêtre. Sans Direct3D 12, vous pouvez quand même installer depuis une invite de commandes. Voir [Ligne de commande](Command-Line).

<a id="open-it"></a>

## Ouvrir le programme d'installation

Double-cliquez sur le programme d'installation. Windows affiche l'[invite du Contrôle de compte d'utilisateur](Glossary#uac-prompt) (UAC). Un programme d'installation n'est pas signé, sauf si la personne qui l'a généré l'a signé : l'invite indique donc généralement **Éditeur inconnu**. C'est normal. Voir [Confidentialité et sécurité](Privacy-and-Security#signing).

La fenêtre, « Programme d'installation de WinAppMigrator », « Réinstalle une application capturée sur un autre PC », s'ouvre immédiatement, quelle que soit la taille du programme d'installation. Elle affiche « Vérification du programme d'installation… » pendant qu'il vérifie le [hachage](Glossary#hash) de l'ensemble de son package. Cette vérification lit tout le package : un programme d'installation volumineux prend donc un certain temps. À partir de ce moment, le programme d'installation garde son propre fichier ouvert, ainsi que son fichier de données s'il en a un, pour qu'aucun des deux ne puisse être modifié ou remplacé tant que la fenêtre est ouverte.

La vérification peut s'arrêter ici, avec un message en rouge. Les plus courants sont les suivants :

- Le programme d'installation est endommagé ou a été modifié après sa génération : copiez-le à nouveau depuis l'emplacement où il a été généré.
- Son fichier de données est absent, ou n'est pas le bon : copiez à nouveau les deux fichiers, dans un même dossier.
- « Le package de ce programme d'installation ne peut pas être utilisé sur ce PC », par exemple parce que les programmes de l'application sont conçus pour un autre type de processeur, ou nécessitent une version plus récente de Windows. Rien n'a été modifié.

Tous ces cas sont expliqués dans [Résolution des problèmes](Troubleshooting#installing).

Une fois la vérification réussie, le titre de la fenêtre devient « Installer » suivi du nom de l'application.

<a id="its-language"></a>

### Sa langue

Un programme d'installation utilise la langue du PC sur lequel il s'exécute, et non celle du PC où il a été généré : la langue d'affichage de Windows si c'est l'une des 24 langues de WinAppMigrator, et l'anglais sinon. **Langue**, en haut à droite, la change pour cette exécution. Ce qu'il affiche de l'application, y compris les notes rédigées par l'application lors de la génération, apparaît aussi dans cette langue. Un programme d'installation généré avec WinAppMigrator 0.6 ou une version antérieure affiche ces notes en anglais, telles qu'elles ont été rédigées. Une [installation silencieuse depuis une ligne de commande](Command-Line) répond toujours en anglais, pour que les scripts puissent la lire.

<a id="the-app-and-where-it-goes"></a>

## L'application et son emplacement

La première carte contient le nom de l'application, avec l'éditeur et la version en dessous, par exemple « Fabrikam Inc. · 1.7.3 ». Ensuite :

| Ligne | Ce qu'elle indique |
| --- | --- |
| **Capturée sur** | Le nom du PC d'où provient l'application. Elle n'apparaît que si le programme d'installation a été généré avec l'option **Données utilisateur** activée. |
| **Programme d'installation** | Si ce fichier de programme d'installation est [signé](Glossary#signature) : « Vérification de sa signature… », puis « Non signé : éditeur inconnu », ou « Signé par » suivi du nom du signataire. Une signature qui ne résiste pas à la vérification affiche un message en rouge à la place : « La signature de ce programme d'installation n'est pas valide », avec la raison donnée par Windows, et « Il a peut-être été modifié depuis sa signature. » |
| **Emplacement d'installation** | Le dossier dans lequel va l'application, avec **Parcourir** pour en choisir un. Il correspond au départ au dossier où se trouvait l'application sur l'ancien PC. Une application provenant d'un Windows 32 bits démarre plutôt dans `Program Files (x86)`, où Windows 64 bits range les applications 32 bits. Si ce dossier ne peut pas être utilisé ici, un avis en rouge en dessous en indique la raison. |
| **Espace requis** | L'espace nécessaire à l'installation sur ce lecteur, et l'espace libre, par exemple « jusqu'à 824,0 Mo sur C: (356,0 Go libres) ». Il tient compte du package décompressé ainsi que des fichiers de l'application. |

Le nom de l'éditeur sur la première ligne provient du package : n'importe qui aurait pu l'écrire. La ligne **Programme d'installation** provient de Windows, et c'est elle qui fait foi.

<a id="when-the-location-cannot-be-used"></a>

### Quand l'emplacement ne peut pas être utilisé

Si le dossier indiqué dans **Emplacement d'installation** ne peut pas être utilisé, un avis en rouge en dessous en indique la raison. **Installer** et **Aperçu des modifications** restent grisés tant que vous n'en choisissez pas un autre, et une ligne à côté indique « Installer et Aperçu des modifications restent désactivés tant que l'emplacement d'installation ne peut pas être utilisé. »

La raison la plus courante est que le dossier contient déjà des fichiers. Un dossier qui contient des fichiers n'est utilisé que si une installation antérieure de WinAppMigrator les y a placés, afin que les fichiers d'une autre application ne soient jamais remplacés :

| Avis | Signification | Que faire |
| --- | --- | --- |
| Le nom de l'application, puis « est déjà installé dans ce dossier, cette copie ne peut donc pas y être installée. » | L'application est déjà installée sur ce PC, souvent par son propre programme d'installation : son programme principal se trouve dans ce dossier, ou la liste des applications installées de Windows l'y indique. C'est le cas des applications que de nombreux PC possèdent déjà, comme un navigateur web, un lecteur multimédia ou Office. | Désinstallez d'abord la copie présente sur ce PC, dans Paramètres Windows sous **Applications**, puis rouvrez le programme d'installation. Ou entrez un nouveau dossier vide dans **Emplacement d'installation** pour installer cette copie à côté. |
| Le nom d'une autre application, puis « est installé dans ce dossier », et que cette application « ne peut donc pas y être installé ». | La liste des applications installées de Windows indique une autre application dans ce dossier, par exemple une autre édition d'Office. | Entrez un nouveau dossier vide dans **Emplacement d'installation**. Ou, si vous n'avez plus besoin de l'autre application, désinstallez-la d'abord, dans Paramètres Windows sous **Applications**, puis rouvrez le programme d'installation. |
| « Ce dossier contient déjà des fichiers qui n'ont pas été installés par WinAppMigrator. » | Le dossier contient autre chose, et aucune application installée n'y est répertoriée. Ces fichiers appartiennent peut-être à une autre application. | Entrez un nouveau dossier vide dans **Emplacement d'installation**. Si les fichiers sont des restes d'une application que vous avez désinstallée, vous pouvez supprimer le dossier, puis rouvrir le programme d'installation. |

Pour toute autre raison, l'avis indique « Ce dossier ne peut pas être utilisé : », la raison, et « Entrez un autre dossier ci-dessus. » Les raisons sont les suivantes :

| Raison affichée | Que faire |
| --- | --- |
| « il doit s'agir d'un chemin d'accès complet » | Entrez un chemin qui commence par une lettre de lecteur, par exemple `C:\Program Files\Fabrikam Data Sync`. |
| « il ne peut pas s'agir d'un lecteur entier » | Utilisez un dossier du lecteur, et non le lecteur lui-même. |
| « il ne peut pas s'agir du dossier Windows » | Choisissez un dossier en dehors de `C:\Windows`. |
| « il contient d'autres éléments côte à côte, l'application a donc besoin d'un dossier à l'intérieur » | Ajoutez un dossier pour l'application, par exemple `C:\Program Files\Fabrikam Data Sync` au lieu de `C:\Program Files`. |
| « il s'agit d'un dossier qui appartient à Windows, ou dont le contenu s'exécute ou se charge de lui-même » | Choisissez un dossier ordinaire. Un dossier Démarrage, par exemple, est refusé. |
| « il doit s'agir d'un chemin simple sur un lecteur de ce PC » | Un [dossier partagé sur le réseau](Glossary#network-share) est refusé, de même qu'un nom que Windows interpréterait comme autre chose. |
| que le dossier « est un lien vers un autre emplacement, et d'autres peuvent modifier sa cible » | Utilisez le dossier vers lequel pointe le [lien](Glossary#link). |
| « c'est l'emplacement où les installations conservent leurs enregistrements » | Choisissez un dossier en dehors de `%ProgramData%\WinAppMigrator`. |

Si le dossier contient des fichiers et qu'une installation antérieure de ce projet **y est** enregistrée, le programme d'installation l'utilise, et vous avertit : « Ce dossier contient déjà des fichiers. Les fichiers portant le même nom sont remplacés, et sont rétablis si vous désinstallez. »

Une fois un dossier accepté, la fenêtre remplace un fichier, une valeur de Registre, un service, une tâche, un raccourci ou une variable du même nom, comme le fait `--force` sur la [ligne de commande](Command-Line#what---force-does). Ce qui appartient déjà à une autre application, comme son service ou son type de fichier, reste à cette application.

Si le lecteur semble manquer d'espace, une note en orange indique « L'espace risque d'être insuffisant ici » et l'espace nécessaire. Le package peut être décompressé sur un autre lecteur, mais les fichiers de l'application doivent tout de même tenir sur celui-ci. L'installation elle-même vérifie de nouveau l'espace avant de commencer.

<a id="an-earlier-install-that-never-finished"></a>

### Une installation antérieure qui n'a jamais abouti

Si une installation précédente effectuée par l'un de ces programmes d'installation a été interrompue, par un plantage ou une coupure de courant, une note en orange indique « Une installation antérieure de » cette application « n'a jamais abouti. Elle sera d'abord annulée lorsque vous cliquerez sur Installer. » Cela se fait automatiquement lorsque vous cliquez sur **Installer**.

<a id="what-this-sets-up-beyond-files"></a>

## Ce qui est configuré en plus des fichiers

Cette carte est la liste que vous acceptez lorsque vous cliquez sur **Installer**. Sous son titre, elle indique :

> Tout ceci est effectué avec des droits d'administrateur. Si l'installation échoue, ce qu'elle a modifié est rétabli, et vous êtes informé de tout ce qui n'a pas pu l'être. Vous pouvez aussi désinstaller l'application plus tard depuis Paramètres, sous Applications installées. Les fichiers qu'elle remplace sont alors rétablis, et les runtimes qu'elle installe restent en place.

Chaque ligne comporte un nom, une étiquette à droite indiquant de quel type d'élément il s'agit, et une ligne indiquant ce qu'il fait. Une étiquette dans le bleu habituel est ordinaire. Une étiquette en **orange** signale un élément qui s'exécute avec tous les droits, comme un service qui s'exécute en tant que LocalSystem, un pilote ou une règle de pare-feu qui autorise les connexions entrantes. Examinez-les attentivement.

Si la liste est vide, elle indique « Rien : l'installation copie uniquement des fichiers, ainsi que des paramètres dans les propres clés de Registre de l'application. »

La liste est établie pour l'emplacement indiqué dans la zone. Si vous changez d'emplacement, elle indique « Évaluation de ce que l'installation configurerait à cet emplacement… » pendant un instant, et **Installer** attend.

<a id="what-each-label-means"></a>

### Signification de chaque étiquette

| Étiquette | De quoi il s'agit | Exemple de la ligne en dessous |
| --- | --- | --- |
| **Service** | Un [service](Glossary#service) : un programme que Windows exécute en arrière-plan | « Exécute FabrikamSync.exe en tant que NT AUTHORITY\LocalService, démarre maintenant et à chaque démarrage de ce PC ». D'autres indiquent « démarre maintenant et peu après chaque démarrage de ce PC », « démarre uniquement à la demande » ou « est désactivé ». |
| **Pilote** | Un [pilote](Glossary#driver) qui appartient à l'application | « Charge » un fichier « dans Windows même ». Toujours en orange. Aucun test du projet n'a chargé de pilote. |
| **Tâche planifiée** | Une [tâche planifiée](Glossary#scheduled-task) | « S'exécute en tant que SYSTEM », ou en tant qu'un autre compte. « avec les autorisations maximales » est signalé en orange. Une tâche qui s'exécutait sous le compte d'une personne sur l'ancien PC s'exécute sous le compte de la personne qui l'installe. |
| **Démarre automatiquement** | Un [élément de démarrage](Glossary#startup-item) | « S'exécute quand vous vous connectez », « S'exécute pour toute personne qui se connecte » ou « Un raccourci dans le dossier Démarrage, pour vous » |
| **Raccourci** | Un [raccourci](Glossary#shortcut) sur le Bureau ou dans le menu Démarrer | « Ouvre Fabrikam.exe, pour tous les utilisateurs de ce PC » ou « pour vous » |
| **PATH** | Une entrée ajoutée à la [variable Path](Glossary#path-variable) | « Ajoute à Path » et « Le dossier d'installation\bin ». Elle est ajoutée au contenu existant, sans jamais le remplacer. |
| **Variable d'environnement** | Une [variable d'environnement](Glossary#environment-variable) | « Définie sur » sa valeur |
| **Extension de shell** | Du code qui se charge dans l'Explorateur de fichiers | « Charge » un fichier « dans l'Explorateur de fichiers, pour tous les utilisateurs de ce PC ». En orange. |
| **Commande de menu** | Une commande du menu contextuel (clic droit) | « Ajoute une commande au menu de » un type de fichier. « Modifie l'action effectuée à l'ouverture de ces éléments » est en orange. |
| **Type de lien** | Un [protocole de lien](Glossary#link-protocol), comme `fabrikam:` | « Les liens de ce type s'ouvrent avec l'application » |
| **Type de fichier** | Un [type de fichier](Glossary#file-type), comme `.fds` | « Les fichiers de ce type s'ouvrent avec l'application » |
| **COM** | Des [classes COM](Glossary#com-class) que d'autres programmes peuvent démarrer | « COM dans » un fichier, avec le nombre de paramètres. « Autorisations COM » est en orange. |
| **Règle de pare-feu** | Une [règle de pare-feu](Glossary#firewall-rule) pour l'un de ses programmes | « Autorise les connexions entrantes sur le port 8080 pour FabrikamSync.exe, sur tous les réseaux ». L'autorisation des connexions entrantes est signalée en orange. |
| **Source du journal des événements** | Un nom sous lequel l'application écrit dans les journaux d'événements de Windows | « Peut écrire dans le journal Application, avec des messages provenant de ses propres fichiers » |
| **Police** | Une police située dans le dossier d'installation | « Se charge depuis le dossier d'installation quand vous vous connectez » |
| **Pilote ODBC** | Un pilote de base de données pour [ODBC](Glossary#odbc) | « Se charge dans les programmes qui utilisent ODBC pour accéder à une base de données ». En orange. |
| **Source de données ODBC** | Une connexion à une base de données enregistrée | « Une connexion à une base de données, pour tous les utilisateurs de ce PC » |
| **Paramètre de compatibilité** | La façon dont Windows démarre certains de ses programmes | « Mode de démarrage de ses programmes » |
| **Applications par défaut** | Une entrée qui vous permet de choisir l'application dans Applications par défaut | « Peut être choisie dans Applications par défaut. Aucune modification n'y est apportée » |
| **Nom du programme** | Un nom sous lequel Exécuter et d'autres programmes peuvent la trouver | « Exécuter et d'autres programmes peuvent la trouver sous ce nom » |
| **Autres paramètres** | Des valeurs de [Registre](Glossary#registry) situées en dehors des propres clés de l'application et des types ci-dessus | « 3 paramètres hors des clés propres à l'application », « Dans » les clés. En orange si certaines se trouvent dans les clés de Microsoft. |

Certains éléments ne peuvent être évalués qu'au moment où l'installation les atteint, comme un service du même nom qui appartient déjà à une autre application. Ils restent dans la liste tels que demandés. Lorsque l'installation les atteint, elle les laisse tels quels, et l'indique.

<a id="notes-and-what-is-left-out"></a>

### Notes et éléments exclus

Sous les lignes, des notes en orange indiquent ce que vous devez savoir, par exemple :

- que des informations d'identification n'ont pas été placées dans le programme d'installation, et que vous devrez peut-être les saisir à nouveau ;
- que l'option **Licence** ou **Données utilisateur** était désactivée lors de la génération, et que ces données n'y figurent donc pas ;
- quels [certificats](Glossary#certificate) l'application semblait utiliser, ces certificats n'étant jamais installés ;
- ce que la personne qui a généré le programme d'installation a exclu, et ce qu'elle a ajouté ;
- que l'application risque de ne pas fonctionner ici : « Cette application se trouvait sur la build » d'un Windows plus récent, ou Windows exécuterait certains de ses programmes « par émulation » ;
- que des fichiers de Documents ou de Parties enregistrées iront « plutôt sous » le dossier d'installation, « dans un dossier nommé _external », car le dossier de ce PC se trouve en dehors de votre profil.

Ensuite, en orange, chaque élément exclu par les règles de sécurité, avec la raison. Les 20 premiers sont affichés en entier. Les autres se trouvent sous « autres éléments ont été exclus », que vous pouvez développer. Le reste de l'installation se poursuit sans eux. Ce qui est refusé, et pourquoi, est expliqué dans [Contenu d'un programme d'installation](What-Goes-in-an-Installer#what-the-installer-refuses).

Si le package ne peut pas du tout être installé à l'emplacement choisi, un message en rouge indique « Ce package ne peut pas être installé à cet emplacement » et la raison.

<a id="options"></a>

## Options

Cette carte n'apparaît que si le programme d'installation contient des données utilisateur ou des données de licence. Chaque case est cochée au départ.

| Option | Ce qu'elle indique |
| --- | --- |
| **Données utilisateur** | « Activé : les paramètres, Documents et Parties enregistrées vont dans les dossiers habituels de la personne connectée. Désactivé : tout est placé sous le dossier d'installation, et rien n'est configuré pour s'exécuter à partir de là. » Si le programme d'installation indique à qui appartiennent ses données, une ligne en dessous indique « Ceci contient les données de » ce compte. |
| **Licence** | « Fichiers d'activation et de licence, et valeurs de Registre qui contiennent des licences. Certaines licences peuvent nécessiter une réactivation ici. » |

Cocher ou décocher une case met immédiatement à jour la liste ci-dessus.

Les données utilisateur vont toujours à **la personne connectée** à Windows, même si le nom d'un autre administrateur a été saisi dans l'invite UAC. Si le programme d'installation ne peut pas déterminer de qui il s'agit, il l'indique, exclut les paramètres par utilisateur et place les données utilisateur sous le dossier d'installation plutôt que de deviner. Si personne n'est connecté à cette session, les données utilisateur et les paramètres par utilisateur vont au compte sous lequel le programme d'installation s'exécute, et il l'indique aussi.

<a id="dependencies"></a>

## Dépendances

« Ce dont cette application a besoin, et comment chaque élément est fourni. » Chaque [runtime](Glossary#runtime) a une étiquette :

| Étiquette | Signification |
| --- | --- |
| **Inclus** | Dans le programme d'installation. Il s'installe sans Internet, sauf DirectX End-User Runtime, dont le programme d'installation le télécharge auprès de Microsoft. Parmi ces runtimes, seul le runtime Visual C++ x64 a réellement été installé dans les tests du projet. |
| **Installé avec winget :** et un ID | Il n'a pas pu être téléchargé lors de la génération du programme d'installation : le programme d'installation utilise donc [winget](Glossary#winget), qui nécessite Internet. Cette méthode n'a pas été utilisée dans les tests du projet. |
| **Fonctionnalité Windows : NetFx3** | .NET Framework 3.5, activé en tant que [fonctionnalité Windows](Glossary#windows-feature). Cela peut nécessiter Internet. Cette méthode n'a pas été utilisée dans les tests du projet. |
| **Intégré à Windows** | .NET Framework 4, que Windows 10 et 11 incluent déjà |
| **Fourni avec l'application** | L'application contient sa propre copie |
| **Installer manuellement** | Il n'existe aucun programme d'installation pour ce runtime ici. Installez-le vous-même. |

S'il n'y en a aucune, elle indique « Aucune dépendance d'exécution supplémentaire n'a été détectée. » Voir [Runtimes](Runtimes).

<a id="preview-changes"></a>

## Aperçu des modifications

**Aperçu des modifications** détermine exactement ce que ferait une installation, sans rien modifier. Il vérifie qu'il y a assez d'espace, décompresse le package dans un dossier privé, détermine les modifications, puis supprime ce dossier. La carte de résultat a pour titre « Aperçu : rien n'a encore été modifié », avec **Modifications prévues** et une liste des modifications, en commençant par les services, les tâches et les raccourcis. La fenêtre répertorie les 300 premières, puis indique combien il en reste.

Un aperçu ne vérifie pas le hachage de chaque fichier. Le hachage de l'ensemble du package a été vérifié à l'ouverture de la fenêtre.

<a id="install"></a>

## Installer

**Installer** reste grisé tant que l'emplacement ne peut pas être utilisé et que la liste ne correspond pas à cet emplacement. Si le package ne peut pas du tout être installé à cet emplacement, une ligne à côté indique « Installer est désactivé, car ce package ne peut pas être installé à cet emplacement », et la raison apparaît en rouge dans **Ce qui est configuré en plus des fichiers**. Lorsque vous cliquez dessus :

1. Il établit la liste une dernière fois. Si ce qu'il ferait a changé depuis que vous l'avez consultée, il n'installe rien, affiche la nouvelle liste et indique « Ce que l'installation ferait ici a changé, rien n'a donc été installé. Vérifiez la liste ci-dessous, puis cliquez à nouveau sur Installer si elle vous convient. »
2. Il vérifie qu'il y a assez d'espace sur chaque lecteur sur lequel il écrit.
3. Il décompresse le package dans un dossier accessible uniquement aux administrateurs, en vérifiant le hachage de chaque fichier extrait.
4. Il installe, en consignant d'abord chaque modification dans un [journal](Glossary#journal), afin qu'elle puisse être annulée.
5. Il relit chaque fichier qu'il a installé et vérifie son hachage.
6. Il démarre les services configurés pour démarrer d'eux-mêmes.

En bas, vous voyez ce qu'il est en train de faire, un compteur le cas échéant, le temps écoulé, une barre de progression et **Annuler**. Les étapes que vous pouvez voir sont « Préparation… », « Décompression », « Vérification du package », « Annulation d'une installation antérieure inachevée », « Installation des runtimes », « Copie des fichiers », « Configuration des services », « Configuration des pilotes », « Écriture des paramètres du Registre », « Restauration des données de licence », « Configuration des tâches planifiées », « Création des raccourcis », « Définition des variables d'environnement », « Ajout de l'application à Paramètres » et « Vérification de ce qui a été installé ».

**Annuler** affiche « Annulation après l'étape en cours… », puis « Rétablissement de ce qui a été modifié ». Fermer la fenêtre pendant l'installation a le même effet que **Annuler** : la fenêtre reste ouverte jusqu'à ce que tout soit rétabli. Si rien n'avait encore été modifié, elle indique « Annulé. Rien n'a été modifié. »

<a id="the-hash-check"></a>

## La vérification du hachage

Le programme d'installation vérifie les [hachages](Glossary#hash) trois fois :

1. **À l'ouverture de la fenêtre** : le hachage de l'ensemble du package. Une copie tronquée, ou modifiée après sa génération, est refusée avant que quoi que ce soit n'y soit lu.
2. **Pendant la décompression** : le hachage de chaque fichier, avant que quoi que ce soit ne soit écrit à son emplacement.
3. **Après l'installation** : il relit chaque fichier installé et le vérifie de nouveau.

Les hachages détectent les dommages, ainsi que les modifications faites par toute personne qui ne met pas aussi à jour les hachages. Ils ne prouvent pas qui a créé un programme d'installation. Seule une [signature](Glossary#signature) le prouve.

<a id="the-result"></a>

## Le résultat

La carte de résultat apparaît en haut, avec le titre « Résultat de l'installation ».

| Message | Signification |
| --- | --- |
| « L'application a été installée et a réussi sa vérification du hachage. » | Tout va bien. Si les règles ont exclu quelque chose, le message se poursuit par « sans les éléments indiqués comme exclus ci-dessous. » |
| « L'installation a été annulée, et ce qu'elle a modifié a été rétabli. » | Vous avez annulé. |
| « L'installation a échoué, et ce qu'elle a modifié a été rétabli. » | Une erreur s'est produite. Ce qu'elle a modifié a été rétabli, sauf les runtimes qu'elle a installés. Tout élément indiqué en orange en dessous a été laissé en place, avec la raison, par exemple un service qui ne disparaît qu'au redémarrage du PC. |
| « L'installation a échoué, et tout ce qu'elle a modifié n'a pas pu être rétabli. Ce qui reste est indiqué ci-dessous, et la prochaine installation réessaiera. » | Lisez la liste. Relancer un programme d'installation tente à nouveau de le rétablir. |
| « L'installation ne s'est pas terminée correctement. Examinez les problèmes ci-dessous. » | Lisez les messages en dessous. |

Viennent ensuite trois vignettes : **Modifications effectuées** (ou **Modifications rétablies** après un échec), **Avertissements** et **Erreurs**. Des étiquettes indiquent **Hachages du package valides** et **Fichiers installés valides**, ou **NON VALIDES** en rouge. Jusqu'à cinq avertissements et toutes les erreurs sont affichés sous forme de messages. « Une dépendance nécessite un redémarrage pour terminer son installation. Redémarrez Windows, puis lancez l'application. » signifie qu'un runtime a demandé un redémarrage.

**Modifications**, avec un nombre, ouvre la liste de ce qui a été fait, jusqu'à 300 éléments. Si l'installation ne s'est pas terminée correctement, le chemin de son [journal](Glossary#log) est affiché sous **Journal**.

Après une installation réussie, les boutons du bas deviennent **Fermer** et **Ouvrir le dossier d'installation**.

<a id="where-things-end-up"></a>

## Où vont les éléments

- **Les fichiers du programme** vont dans l'emplacement d'installation.
- **Les données utilisateur**, avec la case **Données utilisateur** cochée, vont dans les dossiers AppData, Documents et Parties enregistrées du compte connecté, ainsi que dans ProgramData. Rien n'est écrit dans un dossier Documents ou Parties enregistrées que ce PC conserve en dehors de votre profil. Ces fichiers vont sous l'emplacement d'installation, dans `_external`, et la liste l'indique en premier. Avec la case décochée, toutes les données utilisateur vont sous l'emplacement d'installation.
- **Les raccourcis** vont sur le Bureau et dans le menu Démarrer de ce PC : les vôtres, ou ceux de tous les utilisateurs si c'est là qu'ils se trouvaient.
- **Path** reçoit uniquement les entrées de l'application. Rien d'autre n'y change.
- **Ce qu'une autre application possède déjà ici reste à cette application.** Un type de fichier, un type de lien, une classe COM, un service, une source de journal des événements, une police ou tout élément semblable qui appartient déjà à autre chose est laissé tel quel, et le programme d'installation l'indique. Une application qui s'attendait à ouvrir les liens `http`, par exemple, doit de nouveau être définie comme application par défaut dans Paramètres.
- **Paramètres** reçoit une entrée pour l'application, pour que vous puissiez la désinstaller comme n'importe quelle autre. Voir [Annuler une installation](Undoing-an-Install).

<a id="common-reasons-it-refuses"></a>

## Raisons courantes d'un refus

| Ce que vous voyez | Pourquoi | Que faire |
| --- | --- | --- |
| « cette application ne peut pas s'exécuter sur ce PC, rien n'a donc été modifié » | Ses programmes sont conçus pour un autre type de processeur, ou nécessitent une version plus récente de Windows | Installez sur un PC qui correspond à l'ancien |
| « l'espace est insuffisant pour cette opération, rien n'a donc été modifié » | L'espace libre est insuffisant sur un lecteur sur lequel il écrit | Libérez de l'espace, ou choisissez un emplacement sur un autre lecteur |
| « une autre installation ou désinstallation est en cours » | Une seule s'exécute à la fois | Attendez qu'elle se termine, puis réessayez |
| Un fichier « porte un nom que cette version de Windows ouvre comme un périphérique et non comme un fichier » | L'application contient un fichier nommé par exemple `aux.h`, que les versions de Windows antérieures à Windows 11 ne peuvent pas contenir | Installez sur Windows 11 |
| Un dossier de données comme `%AppData%` se trouve sous un lien | Le programme d'installation n'écrit pas à travers un lien | Décochez **Données utilisateur** et **Licence**, et installez sans ces données |

D'autres cas figurent dans [Résolution des problèmes](Troubleshooting#installing).
