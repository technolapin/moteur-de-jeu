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

Ajouter le fichier : `git add leFichier.ext`.
On peut ajouter plusieurs fichiers, ou des répertoires.

Commit ses modifications : `git commit -m "Qu'avez vous fait ?"`.
On peut ne faire que `git commit`. A ce moment là, votre éditeur de texte favoris s'ouvrira pour vous demander le message. La première ligne de ce message sera le titre sur GitHub.

Envoyer sur le serveur ses modifications : `git push`.
C'est comme `git pull`, mais dans l'autre sens.
