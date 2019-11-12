# Ce que raconte Clément, le 12 novembre 2019

## Introduction aux ECS

On fait de l'orienté composants.
Pas de l'orienté objets, c'est pas du tout adapté au jeu-vidéo.

ECS : Entity Component System, c'est un Design Pattern.
Données = EC, Code = System.

Les données sont comme un gros tableau, ou une base de données SQL. Grosso modo.
Le tableau représente tous nos objets en mémoire.

Position va être un type (Une structure).
Idem pour Sprite...
Etc...

```
struct Position {
	x: f32,
	y: f32,
	z; f32
}
```

On aura une sorte de bibliothèque de composants.

```
struct World {
	
}
```

Une entité peut avoir plusieurs composants.
On va avoir des tableaux de composants : Tableau des vitesses, tableau des positions...
L'entité est l'indice des tableaux qui correspond à notre objet.

Une entité n'est qu'un entier.

Toutes les struct sont dans des tableaux.

Il y a une structure globale qui a l'ownership de tout le monde.

Pour les structures qui n'ont pas besoin de certaines colonnes du tableau, on met rien dedans.
Par exemple, la case "IA" d'un caillou est vide.

Nicolas veut absolument qu'on puisse dabber.

Une ressource est une donnée qui n'est pas redondante. L'adresse mémoire d'un sprite par exemple est une ressource.

## Encore une chose

**Note : On va utiliser OpenGL au début, et après on verra.**
