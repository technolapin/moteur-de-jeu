# COURS RUST PAR CLEMENT

## INTRODUCTION

* Extension d'un fichier Rust : `.rs`

* Un **Projet** en Rust, c'est construit :

```
Fichier
| Cargo.toml
| src \ 
|         |   main.rs 
|         |   truc.rs 
| target \ 
|         |   release
```

* Pour compiler & lancer : `cargo run`

* Pour seulement compiler : `cargo build`

* Pour compiler & optimiser : `cargo build --release`

*Remarque* : c'est `rustop` qui compile mais `cargo run` le fait à notre place

* Ouvre la documentation de Rust : `rustop --doc`

* Créer un projet de base (hello world par défaut) : `cargo new nom_projet`


## FONCTIONS

* Ecrire une fonction en Rust : `fn nom_fonction(param1 : type_param1, param2 : type_param2) -> type_retour`

* Exemple avec HelloWorld :

```
fn main()
{	println!("Hello World !") ;	}
```

* `println!` : n'est pas une fonction mais une macro `!` veut dire que c'est une macro
* `println!("test {}", a) ;` : équivalent de `printf("blabla %d",a) ;`
* `println!("test {:?}", cube) ;` : afficher ce qui implémente le trait débug


## TYPES PRIMITIFS

* Primitives (types très simples) :
	* En C++ : int, double...
	* En Rust, il faut préciser sur combien de bits sont stockés les variables, nombres, pour les types

		8	16	32	64
unsigned	u	OK	OK	**OK**	OK		EX : `a : u32`
signed	i	OK	OK	**OK**	OK		EX : `a : i32`
float	f	OK	OK	**OK**	OK		EX : `a : f32`


*Remarque* : Flottants écrits avec un 2.8 => Si 0 c'est un flottant il faut écrire `0.0`

* Chaines de caractère : on utilise la classe `String` : tableau de `char` (`char` en unicode)

* Booleans : `bool` : prend 1 bit (comme dans les langages modernes) au lieu de 8 en C

* Type vide (void) écrit `()`


## DECLARATION DE VARIABLES

### Constantes
* `let a = 1 ;` : presque équivalent au `auto` en C++ mais trouve tout seul le type de unsigned qu'il faut en regardant dans la suite du code comment la variable est utilisée
* `let` mot clé obligatoire pour créer une variable
* Si on veut définir une nouvelle variable dont on connait le type : `let variable:type_variable = valeur;`

### Variables
* `let mut variable : type_variable = valeur;`
* => C'est le contraire de `const` en C++, en gros de base tout est en `const` et on précise seulement si c'est une variable


## CAST ENTRE LES TYPES PRIMITIFS
* `variable as u32` : strictement équivalent à `(int_u32) variable` pour le cast


## BOUCLES
* `for variable in iterator {}` -> avec un itérator
* `for variable in 0..n {}` -> pour une variable allant de 0 à n
* `v.iter()` = permet de créer un itérateur sur un objet itérable comme un vecteur...

## RETOUR DES FONCTIONS
* On peut utiliser `return`
* Ou bien `variable` sans `;` et ça retourne ce truc qui correpond au type de retour de la fonction

## USIZE
* `usize` : type d'entier dont la taille dépend si on est en Windows 32 ou 64 bits (dépend de notre matériel)

## TUPLES
* On peut faire des tuples de types : `(u8, f32)`

## TABLEAUX
* Déclaration de type : `[type_variables_du_tableau ; nombre_d_elements]`
* Déclarer un tableau : `let a : [u16 ; 8] = [1,2,3,4,5,6,7,8];`
* Déclarer un tableau modifiable : `let mut a : [u16 ; 8] = [1,2,3,4,5,6,7,8];`
* Pour faire un tableau de 8 éléments à 4 : `let a : [u8;8]=[4 ; 8];`

## SLICE
* pointeur vers un morceau de tableau
* On connait sa taille
* `&[type_variable]`
* `let s : &[u16] = &a ;`  => Les éléments sur lesquels on pointe sont des éléments de a, et en l'occurence pour le moment le premier élément

## BORROW
* `borrow` : emprunter la propriété d'une variable, il n'y a qu'une seule variable qui a le droit de modifier une variable, on emprunte le droit de le faire
```
let a : usize=2;
let b : &usize = &a;  // => Emprunt non mutable n°1
let c : &usize = &a; //  => Emprunt non mutable n°2
```

```
let mut truc : usize=2; //  => Déclaration variable mutable
let coucou : &usize = &mut truc; // ==> Emprunt mutable, on ne met par `machin` en mutable, c'est pas lui qui est modifiable mais ce sur quoi ça pointe
```
* => On a le droit d'avoir une seule référence mutable par objet

* Si on écrit `let mut machin` ça veut dire que machin est une variable
* En écrivant `=&mut truc` ça veut dire que ça pointe vers un objet qui est une variable


## STRUCTURE & ENUMS

### STRUCTURE
* Struct : se fait en dehors des fonctions
* Nomenclature pour les noms : il faut commencer par une majuscule pour le nom des Struct
```
struct Nom 
{   a : u8 ,
      b : u8 ,
      attribut : T
}
```

* Pour accéder à un attribut de la structure, on écrit `Nom.a` ou `Nom.attribut`

### ENUM
* Définit un type et dont les éléments du type sont les éléments Cas_a, Cas_b, Cas_c
* Nomenclature pour les noms : il faut commencer par une majuscule pour le nom des Struct

```
enum Nom
{   Cas_a,
     Cas_b,
     Cas_c
}
```

**J'ai pas noté du tout, sowi, à rattraper, toute l'explication des enums**

* Pour Nom un enum : `let a : Nom = Cas_a(4) ;`

* `Option<T>` => comme le Template
* Some(T)
* None


**Exemple de code :** 

```
struct Michel
{   a:usize  }

impl Michel // Implémenter nos propres méthodes
{	fn get_a(&self) -> usize //&self remplace 'this'
	{	self.a		}
	fn set_a(p:) -> ()
	{	self.a=p ; 		}
}
```

## CONDITIONS ==> **Pas suivi**
* Pour avoir la valeur de sortie du if il faut assigner le if à une variable

```
if a==b
{	
}
```


## MATCH

* comme un if, il faut assigner le résultat à une variable
```
match a // avec a:u8 par exemple
{	
	0 => 2,
	1 => 8,
}
```

## GENERICITE EN RUST

```
fn foo<T> ( a : T ) {
	a
}
```

Comme les templates en C, sauf que là, c'est pas que du copié-collé.

```
struct<T> S<T> {
	champ : T
}
```

## LIFETIME

```
struct S<'a> {
	champs
}
```

Ou on peut utiliser `static` pour qu'elle ne soit jamais supprimée.
```
