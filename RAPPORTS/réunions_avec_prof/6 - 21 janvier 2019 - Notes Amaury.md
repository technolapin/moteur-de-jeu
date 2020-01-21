# Réunion avec Nabil, le 21 janvier 2020

## Organisation soutenance / évaluation, le jeudi 23 janvier 2020

La soutenance est devant le coach uniquement, Nabil. Il nous dit donc de ne surtout pas stresser, il s'en fiche un peu, pour lui l'important, c'est l'avancement du projet.

Clément : « Ce n’est pas quelque chose de très réglementé. Ça dépend des groupes. Certains ont de grands trucs devant plusieurs profs. »

Plus : Rapport à faire. Liste des choses à y mettre dans un mail de Corine Berland.

Le rapport doit être suffisamment détaillé pour permettre à un élève de rejoindre le projet en cours.

Clément : Beaucoup de blabla dans le rapport car c’est assez spécial ce que on fait, un gros tas de fonctionnalités.

## Gestion des équipes

Nouveaux arrivants : Emeric, et je sais plus.

Barbara et Maylis ne partent pas tout de suite : Mi-mars pour Maylis, mi-février pour Barbara. Mais ne n’ont plus cours, ne sont plus obligées de continuer le projet, mais font quand même continuer. C’est adorable de leur part. :3

## Soutenance

OpenGL : Barbara et Maylis, et un peu Alexandre.

Interface : Nicolas.

Physique : Alexandre.

Tout le reste, ECS, structure du code, organisation, grands choix, etc… : Clément.

Debugs à la noix : Amaury.

## Qu'est-ce qu'on va dire à cette soutenance ?

Chacun explique ce qu'il a fait.

Partie graphique, dans la caméra : `aspect_ratio` est le ratio du rectangle.

Dans OpenGL, la caméra est fixée à l’origine, « forward » = orientation aussi. Le « forward » est toujours vers -Z.

Pour se déplacer, tout le monde se déplace, via la « view matrix ».

Toutes les choses qui sont dans le champ de vision sont projeté sur l’écran. L’écran est fixé. L’écran est orthogonal à l’axe X. Le champ est calculé à partir de la position de la caméra, c’est-à-dire l’origine. Le champ de vision est donc fixé.

FOV = Field Of Vision. Variable qui fixe la longueur et l’angle entre l’écran et l’origine. On peut alors calculer le reste à partir de ces variables.

`get_view_matrix()` : Fonction qu’on a écrit au tout tout début pour avoir quelque chose qui affiche.
Pourquoi c’est exactement ça ? Parce que cette fonction est repompée sur Internet… En fait non, Clément l’a écrit… Mais je n’ai pas compris. x)

3 options :
- Ecrire le code,
- Utiliser librairie, exemple : GLM (OpenGL Math), librairie pour faire les calculs maths, mais on utilise Naglebra, qui est la librairie standard en Rust,
- Repomper sur Internet, Nabil n’aime pas trop. (Perso, Stack Overflow, c’est la vie. :D ) => On prend des pièces de construction en ligne. Nabil veut qu’on comprenne le code, c’est logique.

On n’a pas le temps pour aller jusqu’au bout, c’est pour ça qu’on doit comprendre l’architecture. Mais les trucs détaillés, pas besoin de le comprendre ni d’écrire le code.

Pour les maths, on a utilisé Nalgebra. Nalgebra-GLM est une extension qui est inspirée par GLM.
Il faut justifier le choix de Nalebra.

Nabil : « On est en voyage d’exploration. » Il faut justifier nos choix.

Nabil cherche fonction dans Nalgebra pour calculer la view matrix.
Préfère du coup qu’on utilise une fonction qui calcule la view matrix pour nous.

Clément : On n’a pas trop touché à la caméra, car on a fait un truc vite fait au début pour que ça marche. Mais on peut utiliser Nalgebra-GLM.

N’aime pas qu’on a hardcodé les matrices… Etc…

Pense qu’on devrait faire propre avec une classe caméra et une classe écran. => Pleins de choses à changer.

Quand on passe des choses du CPU au GPU, on utilise des API de OpenGL. Ne peut pas envoyer une matrice Nalgebra au GPU.
En C++ : S’en fiche, veut juste où sont les datas, donc un pointeur, et le nombre d’octets à compter. Il faut juste faire attention à l’ordre des données.

Je ne comprends plus rien. x)

La suite de cette réunion dans le prochain épisode !
**RDV jeudi à 14h ! Et rapport dans 1 ou 2 semaines.**

Pas besoin de faire les changements dans le rapport qu'il a demandé de faire aujourd'hui.

Aimerait bien qu'on regarder moteurs de jeu opensources pour comprendre.
