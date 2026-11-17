<a id="getting-started"></a>

# Prise en main

Cette page vous accompagne du téléchargement jusqu'au premier écran de l'application.

<a id="what-you-need"></a>

## Ce dont vous avez besoin

- **Windows 10 ou Windows 11, 64 bits.** L'application est un programme x64. Windows 11 sur Arm l'exécute par [émulation](Glossary#emulation). Windows 10 sur Arm ne peut pas exécuter de programmes x64, et Windows 32 bits ne peut pas l'exécuter du tout. Les éditions de bureau de Windows 10 et 11 sur x64 n'ont pas été utilisées dans les tests du projet, qui utilisent Windows Server 2022 et 2025 et Windows 11 sur Arm. Voir [Configuration requise et tests](Requirements-and-Testing).
- **Des droits d'administrateur** sur le PC. L'application les demande toujours à son démarrage. Voir [Droits d'administrateur](Glossary#administrator-rights).
- **Direct3D 12**, que Windows utilise pour afficher la fenêtre de l'application. Windows 10 et 11 le prennent normalement en charge. Un ancien pilote graphique ou une machine virtuelle peut ne pas le prendre en charge. Voir [Direct3D 12](Glossary#direct3d-12).
- **De l'espace sur un lecteur** pour le programme d'installation que vous générez. L'application vous indique combien avant que vous commenciez.
- **Internet sur l'ancien PC est facultatif.** S'il est disponible lors de la génération, l'application télécharge les [runtimes](Glossary#runtime) Microsoft qu'elle peut inclure et les place dans le programme d'installation. Voir [Runtimes](Runtimes).

Plus de détails, ainsi que ce qui a été testé ou non, figurent dans [Configuration requise et tests](Requirements-and-Testing).

<a id="download"></a>

## Téléchargement

Accédez à la [dernière version](https://github.com/RandyNorthrup/WinAppMigrator/releases/latest). Elle contient trois fichiers :

| Fichier | De quoi il s'agit | À prendre si |
| --- | --- | --- |
| `WinAppMigrator-v0.7.2.exe` | L'application elle-même. Un seul fichier, sans rien à installer. | Vous voulez simplement l'utiliser. C'est celui que veulent la plupart des gens. |
| `WinAppMigrator-v0.7.2-windows-x64.zip` | La même application, sous le nom `winappmigrator.exe`, avec le fichier README, le journal des modifications, la licence et les licences tierces | Vous voulez aussi les documents qui l'accompagnent |
| `SHA256SUMS.txt` | Les sommes de contrôle des deux fichiers ci-dessus | Vous voulez vérifier votre téléchargement |

La partie qui se trouve dans chaque programme d'installation, le [stub du programme d'installation](Glossary#installer-stub), n'est jamais publiée seule. Elle est déjà incluse dans l'application.

<a id="check-the-download"></a>

## Vérifier le téléchargement

Cette étape est facultative, mais elle en vaut la peine. Chacune des vérifications ci-dessous est indépendante des autres. Exécutez-les dans PowerShell, dans le dossier où vous avez enregistré le fichier.

**1. La somme de contrôle.** Un [hachage](Glossary#hash) est l'empreinte d'un fichier. Si un seul octet du fichier change, le hachage change.

```powershell
Get-FileHash .\WinAppMigrator-v0.7.2.exe -Algorithm SHA256
Get-Content .\SHA256SUMS.txt
```

Le long nombre affiché par `Get-FileHash` doit correspondre à celui de la ligne de ce fichier. `Get-FileHash` affiche des lettres majuscules, et le fichier utilise des minuscules. Ce n'est pas un problème : comparez les lettres et les chiffres.

**2. La signature.** Une [signature](Glossary#signature) indique qui a créé le fichier, et qu'il n'a pas été modifié depuis.

```powershell
$signature = Get-AuthenticodeSignature .\WinAppMigrator-v0.7.2.exe
$signature.Status                     # should say Valid
$signature.SignerCertificate.Subject  # should include CN=Randy Northrup
```

Vous pouvez aussi cliquer avec le bouton droit sur le fichier, choisir **Propriétés**, puis consulter l'onglet **Signatures numériques**. La version 0.5.0 est la première dont l'application est signée. Les téléchargements des versions antérieures ne sont pas signés.

**3. Où il a été généré.** Chaque version à partir de la 0.5.0 enregistre qu'elle a été générée par le workflow de publication de GitHub lui-même, à partir de ce dépôt. Cet enregistrement s'appelle une [attestation](Glossary#attestation). Si vous avez [GitHub CLI](https://cli.github.com/), exécutez :

```powershell
gh attestation verify .\WinAppMigrator-v0.7.2.exe --repo RandyNorthrup/WinAppMigrator
```

La même commande fonctionne pour le fichier zip.

<a id="start-it"></a>

## Démarrer l'application

1. **Double-cliquez** sur `WinAppMigrator-v0.7.2.exe`. Il n'y a rien à installer. L'application s'exécute depuis l'emplacement où vous l'avez enregistrée.
2. **SmartScreen.** Windows [SmartScreen](Glossary#smartscreen) peut afficher un écran « Windows a protégé votre ordinateur » pour un fichier qu'il n'a pas souvent vu. Dans ce cas, choisissez **Informations complémentaires** et vérifiez que l'éditeur est Randy Northrup avant de choisir **Exécuter quand même**.
3. **L'invite d'administrateur.** Windows vous demande si vous voulez autoriser l'application à apporter des modifications. Il s'agit de l'[invite du Contrôle de compte d'utilisateur](Glossary#uac-prompt) (UAC). L'application affiche toujours cette invite, car la lecture du lecteur d'un autre PC, et des fichiers que seuls les administrateurs peuvent lire, nécessite des droits d'administrateur. Pour le téléchargement signé, l'invite indique **Randy Northrup** comme éditeur vérifié. Si elle indique « Éditeur inconnu » pour ce fichier, ce n'est pas le fichier signé de la version publiée : ne l'exécutez pas.
4. **La fenêtre s'ouvre.** Si vous voyez à la place un message indiquant que la fenêtre n'a pas pu s'ouvrir et qu'elle a besoin de Direct3D 12, consultez [Résolution des problèmes](Troubleshooting#the-window-could-not-open). L'application ne peut pas s'exécuter sans sa fenêtre.

<a id="first-run"></a>

## Première exécution

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/app-pick-dark.png">
    <img alt="L'application avec Fabrikam Data Sync sélectionnée" src="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/app-pick-light.png" width="860">
  </picture>
</p>

- La liste des applications installées se trouve à gauche. Pendant son chargement, la petite étiquette en haut à droite indique **Chargement**, puis le nombre d'applications trouvées, par exemple **5 applications**.
- La fenêtre démarre en mode clair ou sombre, comme Windows. Le bouton soleil ou lune en haut à droite permet de passer de l'un à l'autre, et l'application mémorise votre choix. Voir [Fichiers qu'elle conserve sur un PC](Files-It-Keeps-on-a-PC). Avec un thème de contraste Windows, elle utilise les couleurs du thème et n'affiche pas ce bouton.
- Elle utilise votre langue d'affichage Windows si c'est l'une de ses 24 langues, et l'anglais sinon. **Langue**, en haut à droite, les répertorie sous leur propre nom et change immédiatement la langue de la fenêtre ; l'application mémorise votre choix. L'arabe, le persan, l'ourdou et l'hébreu s'affichent de droite à gauche. Voir [Langues et accessibilité](Languages-and-Accessibility).
- Cliquez sur une application. La partie droite affiche ce qui irait dans son programme d'installation, et commence immédiatement à le déterminer. Rien n'est généré tant que vous ne cliquez pas sur **Créer le programme d'installation**.

Étape suivante : [Générer un programme d'installation](Building-an-Installer).
