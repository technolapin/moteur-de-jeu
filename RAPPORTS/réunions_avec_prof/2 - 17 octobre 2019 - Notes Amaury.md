TODO : Mettre au propre

# Réunion du 17 octobre 2019
126 rue de Lagny (Siège monde d'Ubisoft)
Studio Montréal 5-7 mille

JV : Pas mal de couches,
La plus connue : Design etc... ==> GAMEPLAY, faite par jeux, même si c'est réutilisable entre des jeux d'une même série
En desosus : Ingénieurie software ==> Moteur de jeu

Le moteur : Entre le jeu et le matériel, prend en compte spec du matériel
Permet d'abstraire le matériel pour les développeurs du gameplay

FRAMEWORKS (Plus que des moteurs) : Unity 3D, Unreal Engine

Moteur : Composant réutilisable, mais ça a ses limites 
Doit monter résultat moteur avec un jeu ==> Démo technique

JV = Simulation temps réel "doux", pas grave si on loupe un peu

Digital Foundry (Site web) : Décortiquent jeu de manière technique

Illusion de réel : Dès qu'on s'éloigne, les personnages bougent moins par exemple

Le moteur fait les illusions

Partie gestion du hardware pour nous : OpenGL
Les constructeurs de console offrent moins que ça

Comment est ce que les entités vivent dans ce monde et comment elles sont mises à jour ?

Il faut que l'architecture soit modulaire

Un moteur : Des couches les unes sur les autres

Gestion des ressources
Lire des assets complets
Comment est ce qu'on affiche un mesh ?
Physique / Collisions
Afficher textures / Afficher meshs

PDV de Nabil : PHYSIC + RENDERING + INTERACTION WITH USER

EN SACHANT QUE LE JOUEUR N'AIME PAS ATTENDRE !

Un moteur essaye de ne pas faire faire au CPU ce qui n'est pas nécéssaire pour le moment

Ne pas charger objects qui ne servent à rien
MAIS Préparer les objets qui vont être affichés après

Collision, on test en plusieurs étapes : 1. Est ce que dans la même région ? 2. Est ce que boite englobante se touchent ? 3. Est ce que faces se touchent ?

Une production d'un jeu est sur 3-4 ans. Pendant 2 ans, itérations sur des idées.


COnseil :
Partir sur une idée de scène, et qu'est ce qu'on a besoin ?
Afficher, physique (Même minimaliste), input (Joueur) et output (Ecran et son).

Pour la physique, on peut utiliser "Bullet" (Librairie). ==> Comment Bullet va être interfacé au reste ?

IL FAUT UNE ABSTRACTION ! Pour que, par exemple, on puisse remplacer à un moment Bullet par une autre lib.
Dans un JV, il faut un maximum d'abstraction pour remplacer des choses sans casser tout le reste.


JV est pas l'industrie la plus transparente.
Réécrivent leurs allocateurs mémoires, donc utilisaient pas la STL C++, utilisaient la leur
Car mal optimisé
N'utilisent pas Boost car le code template c'est trop long à compiler

Ca l'intéresse qu'on utilise Rust, voir comment on interface ça à Bullet par exemple
Préférence de Nabil : C++, peut nous aider
Sylvain : Intérêt à voir des gens faire ça en Rust, par contre, plus gros challenge
==> Discuter pour faire notre choix

Tous les moteurs d'Ubi ont au moins 15 ans
Développer un moteur coûte super cher

Rust est linkable à toutes les libs


Microsoft : DirectX, etc...
Couche d'abstraction au dessus de ça
Moteur de rendu est une partie du moteur de jeu

Partie haute du moteur graphique est commune à la plateforme, la partie basse change
==> Pas optimisable de la même manière
Hui : Xbox et PS4 sont proches niveau hardware
Par contre, par rapport à la Switch, change tout le moteur de rendu


Partir simple :
Affichage statique,
puis faire bouger objets
faire bouger

==> Y aller graduellement


Dans le dév d'un jeu : Constamment en train de réajuster, tout change en permanance


Parallélisme : Une première structure, pas de parallélisme possible.


Moteur de jeu publique / opensource et facile à lire ?


Partie méthodiquement sur afficher un écran, mettre à jour, charger ressources, puis en déduire ce qui doit interagir


IDE ? Visual Studio CODE est pas mal, crossplateforme, c'est pas mal
Un IDE c'est pratique parce que ça analyse le code


Trouver une librarie pour le son
Moteur de son


C'est bien que ça soit crossplateforme


Testgame : Ne pas aller dans un truc gigantesque
Il faut qu'il y ait des interactions entre des trucs qui bougent
