# Réunion avec Nabil, le 23 janvier 2020

Date limite pour rendre le rapport : **5 février 2019**

Ce rapport est présent sur Google Docs, et partagé avec les membres du groupe

## OpenGL (Barbara & Maylis)

FV = Float variable, dans nom de fonction d’OpenGL

Il faut qu’on sache comment fonctionne le pipeline d’OpenGL, un peu plus que les grandes étapes.

## Physique : Nphysics + Ncollide (Alexandre)

Librairie choisie : Nphysics, choix collectif. Même si Nabil avait proposé Bullet.

Nphysics est opensource. Il y a des wrappers Bullet pour Rust, mais ils ne sont plus maintenus depuis 2 ans.

Ncollide, va avec Nphysics, pour gérer les collisions. Les deux sont entièrement en Rust, grosse doc, facile à utiliser.

Alexandre crée une structure object, définit la shape / forme de l’objet. Crée un « rigid body », et utilise le « collider » de Ncollide qui permet de détecter les collisions.

Geometrical world et mechanical world.

Nabil : 3 objets ? Dans le processeur, dans la carte graphique, et dans le « monde physics » ?

Le rigid body sert uniquement à récupérer le collider, qui contient la « hitbox » de l’objet.

On donne l’objet 3D a la librairie physique, sert à créer le collider « qui a la même forme ». Et crée un rigid body autour de cet objet.

Nabil veut savoir si la mesh de l’objet est copiée ou pas ?

Comment synchroniser physique avec le dessin ?
Pour le moment, pas de sync, on voit juste les coordonnées bouger, mais pas à l’écran.

Comment on va faire le lien ?
Clément : Déjà, pour faire la physique, c’est une hitbox, une mesh simplifié. Comment on fait le lien entre les deux ? Le CPU donne au GPU les coordonnées des objets. On n’a pas encore regardé comment faire.

Nabil : Dans Bullet, le lien entre le GPU et le CPU est une matrice : La « model matrix », une des 3 matrices : « view-matrix », « projection matrix ». La view matrix pour la caméra, et la projection matrix pour mettre en 2D.
Tout ce qu’on a à faire, c’est demander à la librairie la model matrix !
Google : « nphysics get model matrix »

Structure ? Architecture actuelle ? Différences class et enum ?
Alexandre : Une enum composée de plusieurs structures, pour traiter facilement les cas, contient tous les cas de hitbox différentes. Permet de créer n’importe quel rigid body.

Comment on sait que tel objet est une balle par exemple ? On doit préciser le type !
L’enum sert à distinguer plusieurs cas.

Explique son code, tout dans le `main.rs` pour le moment, a tout écrit lui-même.

Objet « set » qui contient tous les objets dans le moteur physique.

Tout ceci n’est pas intégré à ce que font Barbara et Maylis.

Clément : La partie graphique et la partie physique sont designées pour être comme des librairies qu’on importe ensemble dans la partie ECS.

Nabil : Il nous faudrait un truc global pour savoir qui interagit avec qui, etc…

On doit faire un graphe représentant l’architecture globale ! A faire pour la semaine prochaine ! **Il faut lui donner une date où on discute de l’architecture, prochaine réunion.**

## Interface & interaction (Nicolas)

Input, et tout ce qui va actionner les fonctions (Cliquer sur une boule en jeu par exemple), et l’interface (i.e. menus).

Nabil : Ce sont deux choses différentes : Interface et interaction.

Interface : Au début on avait parlé de SDL2, etc…

Pour inputs : Glutin, Nabil trouve que c’est trop basique, Clément : On utilise Glutin uniquement pour les entrées clavier et souris. Ce n’est pas une librairie d’interface.

Pour l’UI : Pour le moment, pas de partie graphique, on doit le faire, mais pour le moment.

## ECS + gestion de projet (Clément)

A fait toute la base de la démo + A fait la gestion des ressources + A fait l’ECS

Pour le moment, c’est dans `graphics/processing/`.

Quand le jeu charge un fichier, il est mis dans le « ModelHolder ». A la base c’était pour les modèles 3D, mais c’est plus général maintenant.

A un joli graphique dans `RAPPORTS/diagrammes/graphics.png`.

Clément passe au tableau.

Il faut une classe qui ouvre les fichiers objets, textures, etc… Appelée par les objets lorsqu’on les crée, donc sait ce qu’il charge. « FileHandler »

Enum OneMaterial. Contient les données de tous les matériaux. C’est comme une classe.
Textured ou NonTextured…

One object, one material. Et un a une scène avec une liste d’objets.

OneGroup = VBO + OneMaterial

OneObject = List of OneGroup

OneScene = List of tuples OneObject + InstancesVBO

*Non mais là je ne suis plus du tout. x)*

Le ModelHolder (Qui devrait s’appeler « RessourceHolder ») contient les OneGroup, rangés par objet.

**Entre nous, il faut qu’on fasse un document qui définit l’architecture, un pour chaque partie du moteur, puis un général !**

## Soutenance

**Pour la soutenance finale avec Sylvain, il faut un diapo.**
