# Projet E4 : Construction d'un moteur de Jeu Vidéo

Encadrant·e·s :
- Sylvain GLAIZE, UBISOFT
- Nabil MUSTAFA, ESIEE Paris

Participant·e·s aux semestres 1 et 2 :
- Clément CHOMICKI `Chef de projet`
- Alexandre LEBLON
- Amaury BARUZY
- Nicolas GOISLARD
- Barbara PARE

Participant·e·s uniquement au semestre 1 :
- Maÿlis MONTANI

Participant·e·s uniquement au semestre 2 :
- Sébastien ZHOU
- Aymeric SALLET

## Sujet du Projet

L'objectif du projet est la construction d’un moteur de jeu video, et d’un jeu "test" pour prouver son fonctionnement.

Ce moteur de jeu devra être capable de :
1. Rendre une scène à l'écran.
2. Pouvoir y déplacer un élément contrôlé par un joueur.
3. Y faire se déplacer des éléments contrôlés par le moteur.
4. Résoudre des collisions entre les éléments de jeu.
5. Plus largement résoudre des interactions entre les éléments de jeu.
6. Afficher des éléments de signes et feedbacks au joueur (Texte et/ou particules, son).

En fonction du temps et de l'avancée, le moteur peut aussi intégrer (Par ordre de complexité) :
1. Une gestion de musique déambiance.
2. Gérer un écran de titre et différents niveaux.
3. Des éléments animés par déformation.
4. De la communication entre deux moteurs (Jeu en réseau).

Les étudiants devront entre autre :
1. Identifier les différentes parties constituantes d'un moteur de jeu vidéo.
2. Impl ́ementer ces parties en veillant à leur modularité / architecture.
3. En architecture, déterminer en particulier sur quel modèle les entités du monde virtuel sont gérées.
4. Optimiser le programme (CPU/GPU ainsi que mémoire).
5. Décider et implémenter un "jeu test" pour montrer les possibilités du moteur.

## Installation sous Linux
Vraiment, utilisez Linux. C'est plus simple.
Ou alors, utilisez Windows Sub-Linux (WSL), mais on risque d'être vite limité.

### Installer les paquets nécessaires (Ici, avec APT)

```
sudo apt install cargo
sudo apt install libsdl2-dev
```

### Cloner le dépot

Voir `AIDE_AVEC_GIT.md`.

### Compiler et lancer avec Cargo

Voir `COURS_RUST.md`.
