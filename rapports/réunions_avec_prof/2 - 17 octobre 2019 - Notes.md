# Réunion du 17 Octobre 2019 - UBISOFT 

## PARTICIPANTS

### Elèves
* Amaury BARUZY
* Alexandre LEBLON
* Clément CHOMICKY
* Barbara PARE 
* Maylis MONTANI

### Encadrants 
* Nabil MUSTAFA
* Sylvain GLAIZE


## REUNION

### Qu'est ce qu'un jeu vidéo ?
* Un peu flou comme concept
* Plusieurs couches dans les jeux vidéos
	* La plus connue : Design => Gameplay pour la vue caméra et graphismes, faite pour chaque jeu, même si réutilisable entre les jeux d'une même série
	* En dessous : Ingénierie software => Moteur de jeu

### Qu'est ce qu'un moteur de jeu vidéo ?
* Entre le jeu et le matériel, permet de prendre en compte la spécificité du matériel
* Permet de développer plusieurs styles de jeux
* Abstraction d'un certain nombre de services pour offrir aux développeurs du jeu le moyen de ne pas se préocuper d'un certain nombre de choses
* Dépend du style de jeu à faire (ex : quand on a un grand terrain pour représenter la Bolivie ou des montagnes...)
* Permet de gérer l'illusion du temps réel : Dès qu'on s'éloigne de certains objets, les personnages bougent moins par exemple, cela n'influe pas la perception du joueur et permet de gagner du temps processeur 
* Un moteur contient des couches les unes au dessus des autres : gère la mémoire, l'organisateur de tâches, lit des fichiers, lis les ressources et les mesh...
	* Gestion des ressources
	* Lire des assets complets
	* Comment afficher un mesh
	* Physique / collisions
	* Afficher des textures / Afficher des mesh
* Ne doit pas s'arrêter pour charger (l'utilisateur n'aime pas ça)
* Gérer les collisions :
	* Est-ce que c'est dans la même région ?
	* Est-ce que des boîtes englobantes se touchent ?
	* Est-ce que des faces se touchent ?
* Partie haute du moteur graphique commune à la plateforme, la partie basse change => Pas optimisable de la même manière
	* HUI Xbox et PS4 proches au niveau hardware, par contre la Switch => Change tout le moteur de rendu


### Que doit-on faire ?
* Nous devons décider d'un jeu, en partant d'une idée de scène :
	* Affichage
	* Mouvements (faire bouger des entités, physics, même minimaliste)
	* Input (joueur) et Output (écran et son)
	* Avoir un personnage principal
* Réaliser un moteur de jeu vidéo (démonstration technique) avec des limites de temps (simulation temps réel) au sens mou (chacune des étapes doit se terminer dans un temps à peu près)
* Ordonnanceur : nous devons nous demander comment les entités vivent dans le monde et se mettent à jour
* L'architecture doit être modulaire
* Niveau moyen de rendering : 
	* Physique : gérer les collisions
	* Rentrer dans une zone pour déclencher un dialogue par exemple
	* Entité : mesh + comportement + son
* Il faut faire une première passe sur les entités pour éliminer les éléments non nécessaires à l'affichage, et pré-charger les éléménts de la prochaine scène
* Premier affichage statique d'une scène, faire bouger les objets, bouger la caméra, bouger par quelqu'un, voir ce qui peut être travaillé en parallèle en fonction de l'avancement sur chaque étapes
	* Tout ce qui est input peut être fait en parallèle
	* Structurellement, physics et rendering en deux parties mais on peut moyennement le faire en parallèle
* Dans le développement d'un jeu, constamment entrain de réajuster, tout change en permanence
* Commencer par une première structure tous ensemble, sans parallélisme possible
* Partir méthodiquement pour affihcer un écran, mettre à jour, charger des ressources, déduire ce qui doit intéragir
* Nous devons trouver une librairie pour gérer le son, moteur de son
* Test game : ne doit pas être un truc de fou, il faut qu'il y ait des interractions entre les trucs qui bougent, point !
* On doit pouvoir utiliser notre moteur de jeu sous Linux et Windows


### Outils existants
* Frameworks (+ que des moteurs de jeux) : Unity 3D, Unreal Engine
* Digital Fonderie (site web) : Décortique le fonctionnement des jeux de manière technique
* Gestion du hardware : OpenGL, DirectX (sous Windows et X Box) => On doit écrire une couche d'abstraction
* Gestion de la physique : librairie Bullet => Comment l'interfacer au reste (comment prendre un moteur physique existant et le rendre visible par le moteur) ? Mais il faut un abstraction pour qu'on puisse remplacer à tout moment sans tout casser !
* Librairie standard : std => Dans le domaine du jeu vidéo, en général ils réécrivent tout (Assassin's Creed : doivent savoior à chaque instant où se trouvent les informations dans la mémoire, la leur est plus optimisée pour ce qu'ils font)
* NE SURTOUT PAS UTILISER "boost" : le code template est trop long à compiler
* Sylvain est ok pour le Rust mais dit qu'on aura probablement beaucoup de problèmes d'interfaçage, et plus gros challenge car pas grand chose qui existe déjà en Rust ; Nabil pense que c'est pas une bonne idée car il ne pourra pas nous aider en Rust, mais il peut nous aider en C++ et nous a déjà préparé des codes pour démarrer en C++
* Rust est linkable à toutes les librairies
* Utiliser cargo pour compiler le Rust
* IDE Visual Studio Code : écrire le code, en crossplateforme, c'est pas mal pour l'analyse du code


## ORGANISATION DES REUNIONS
* Réunion toutes les semaines avec Nabil (?)
* Réunions une fois par mois avec Sylvain


## MAIL SYLVAIN

## REUNION ETUDIANTS