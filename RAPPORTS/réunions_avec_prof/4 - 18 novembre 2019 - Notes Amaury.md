# Réunion avec Nabil, le 18 novembre 2019

## Participants

### Elèves

* Alexandre LEBLON
* Clément CHOMICKY
* Amaury BARUZY
* Maylis MONTANI
* Nicolas GOISLARD

### Encadrants

* Nabil MUSTAFA

## Réunion

Notre grosse partie, c'est de faire le pipeline graphique ! Il y a le tutoriel envoyé par Sylvain à suivre.

On a déjà avancé sur cet article.

Regarder la librairie "bullet" ! On est 6, on peut diviser le travail.

Pour Clément, sur ce quoi il bosse : ECS System. Component System.
Concept of entity and component. 
"User" = The person who will use the engine we are making.

Pour Nabil : Faire quelque chose de flexible pour l'utilisateur lui semble "too much", sachant qu'on a déjà beaucoup de choses à faire.
Pour Clément, non.

Nabil : Fait le moteur en premier, faire le jeu, et après voir comment on peut rendre le moteur flexible.
Problem : On n'a aucune démo OpenGL.

On doit le connecter avec OpenGL... Je comprend plus rien, Maylis, revient m'aider à prendre des notes STP !

J'ai ajouté Nabil au repos.

Pb Maylis à compiler ce qu'elle fait avec OpenGL, surement à cause du fait qu'elle utilise une machine virtuelle.

J'ai dû rater des étapes à noter.

Clément explique ce qu'il est en train de faire avec son **ECS**. Et là il va dessiner au tableau.

Nos objets sont une liste de composants.

Classes qu'il a :
* Component, something that is stored in a storage.
* Entities, entities are stored as and ID in the index.
* Storage<Component>, that stores components. Each storage stores one type of component.
* World.

Ex of Component : `struct Position (x, y);`
`struct ToDraw;`
`struct Speed(x, y);`

There are components which stores nothing, but are here like as a flag.


Ce que propose Nabil : On peut l'utiliser, s'en inspirer, ou pas.

OBJETS :
* Dans le moteur, les objets seront des "meshes" ! Chaque mesh est fait de "vertexs", "edges" et "faces".
* A coté, il y a "lights"... ==> Rendering engine.
* Camera.
* Assets = Textures, images.
* Materiels = Physics of materials, how do you represent materials ?
* State of objects.
AU MILIEU :
* Rendering Engine, utilise les éléments ci-desus pour faire le rendu, donner l'image.
* Physics Engine, gère la position des objets.
* UI (User Interface) Engine, gère les événements (Clique sur un bouton, etc...).
* AI (Artificial Inteligence) Engine.

UI & AI Engines -> Physics Engine -> Rendering Engine.

Physics Engine : USE BULLET ! Takes care of the reality of the object. Give an object, what is its position now ?

Duplique sur son propre système de mémoire.

Comment mettre toutes ces choses dans l'ECS ?

Clément : ECS gère les ressources.

(Je suis crevée punaise, milles excuses si mes notes sont incomplètes)

Advice Nabil : 2 personnes prennent le rendering engine, 2 sur le physics, etc... Pour avoir un système basique qui fonctionne.
Après, on peut reculer, redesigner, et mettre dans l'ECS.
C'est ainsi qu'il aurait fait.

UI takes a lot of time.

Voir shaders pour OpenGL, bosser là dessus !

Send email when we want to meet ! We have to contact him !
Bye ! Good luck !

Will find online free simple game engines ! Have a look at them.
