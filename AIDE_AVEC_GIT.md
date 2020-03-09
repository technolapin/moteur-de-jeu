# Aide rapide avec GIT

## A faire une seule fois

Télécharger GIT :
* Windows : https://git-scm.com/download/win
* Linux avec APT : `sudo apt install git`

Aller dans le répertoire voulu :
* Pour Windows, aller dans le répertoire voulu via l'explorateur de fichiers, faire un clique droit et sélectionner `Git Bash here`. Ca ouvre un joli terminal.
* Pour Linux, `cd chemin/vers/répertoire`, je ne vous apprend rien.

Git s'utilise via un terminal. Donc à partir de là, c'est pareil pour Windows ou Linux...

Cloner le dépôt : `git clone https://github.com/734F96/moteur_jeu_video.git`

Enregistrer son adresse mail pour les commits : `git config user.email "user@domain.tld"`.
**Bien mettre l'adresse associée à son compte GitHub pour que les commits soient associés à ce compte.**

Enregistrer son nom pour les commits :  `git config user.name "Nom"`
**Idem, bien mettre son nom d'utilisateur GitHub.**

## Mettre à jour

Pour mettre à jour depuis le dépôt distant : `git pull`.

A faire de temps en temps. Cela permet de récupérer toutes les modifications des collaborateurs.

## Envoyer ses modifications

Mettre à jour son dépôt local : `git fetch`

Ajouter le fichier : `git add leFichier.ext`.
On peut ajouter plusieurs fichiers, ou des répertoires.
**Faites attention à ne pas envoyer des fichiers temporaires ou générés automatiquement !**

Commit ses modifications : `git commit -m "Qu'avez vous fait ?"`.
On peut ne faire que `git commit`. A ce moment là, votre éditeur de texte favoris s'ouvrira pour vous demander le message. La première ligne de ce message sera le titre sur GitHub.

Envoyer sur le serveur ses modifications : `git push`.
C'est comme `git pull`, mais dans l'autre sens.

## Travailler dans une branche

Il peut être super intéressant de travailler dans sa branche, puis faire une "Pull Request" (Par exemple, il y a d'autres méthodes, comme `git cherry-pick` ou `git merge`) pour envoyer ses modifications sur la branche `master`.

Pour mettre son repos local sur une branche : `git checkout nom`.

Pour créer une branche et mettre son dépôt local dessus : `git branch nom`.

Un commit se fait sur la branche courrante de votre dépôt local. Par défaut, c'est la branche `master`.

Lorsque vous faites des modifications sur une branche, vous devez préciser lors de l'envoi au serveur que vous travaillez sur cette branche : `git push origin nom_de_la_branch`.

Une fois la branche sur le serveur, vous pouvez la fusionner dans master en ouvrant et en validant une "Pull Request" ! Il suffit d'aller sur l'interface web de la branche (`https://github.com/734F96/moteur_jeu_video/tree/nom_de_la_branche`) et de cliquer sur le bouton vert "Compare & Pull Request". Puis, validez là dans l'onglet "Pull requests".
**En la validant, pensez bien à faire "Squash and merge".**

La branche sera ensuite supprimée automatiquement de dépôt distant.

Sur votre dépôt local, retournez sur master et faites un `git pull origin master` pour la mettre à jour.
