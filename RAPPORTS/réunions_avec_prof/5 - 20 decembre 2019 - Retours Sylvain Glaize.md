# Retour général

Suite à notre rencontre du 20 décembre et à votre présentation, j'ai pu me pencher plus en détail sur le travail accompli jusque-là.

Dans ce rapport, je vous présente mes retours sur différents aspects. J'ai essayé de structurer car comme c'est mon premier retour
complet détaillé, c'est touffu.

## Méthodologie

Vous avez fait plusieurs projets avec des avancées diverses sur les différents aspects
que vous développez. Je pense que c'est une bonne chose : cela permet de faire des essais sans
se marcher dessus, cela permet aussi de documenter les différents essais, découvertes et travaux,
ce qu'il sera intéressant à aborder dans le rapport de projet.

Les notes présentes dans `RAPPORTS` ont aussi de la valeur pour le suivi. De manière générale,
on oublie vite les discussions et réflexions que l'on a pendant le développement d'un
projet. Pouvoir se référer ultérieurement aux raisons d'un choix, au cheminement sauve du
temps. Et d'expérience, on regrette toujours de ne pas assez noter.

### Git

Quelque soit le gestionnaire de versions utilisé, un certain nombre de pratiques restent
importantes à respecter. Et cela quelque soit le projet.

  * Choisir un flow : quelles sont les règles pour archiver quoi. Aujourd'hui, ce qui en ressort
  est que c'est un développement sur `master`. C'est valable : c'est clair et direct. Cela
  signifie en revanche que tout l'historique intermédiaire est visible, ce qui amène au
  point suivant.
  * Faire des descriptions avec du sens : lorsqu'on lit un historique de dépôt, on devrait
  pouvoir reconstituer l'évolution du logiciel dans les grandes lignes. Cela signifie
  qu'il faut décrire les changements et trouver la bonne balance. Ici, vous pouvez
  améliorer avec les pistes suivantes :
    * choisir une langue commune : il y a un mélange de français et d'anglais pour le moment
    * choisir un format, une forme d'expression : une préconisation classique en anglais
    est d'utiliser un verbe au présent sans pronom : "Create a document to explain our choices",
    "Add an ECS external crate dependency"
    * chaque `commit` doit contenir un ensemble de changements cohérents
    * rester factuel : qu'est-ce qui a été fait.
    * des annotations peuvent être ajoutées et doivent être cohérentes d'un `commit` à l'autre.
    Par exemple, on peut choisir que [WIP] après une description de changement indique que le
    système n'est pas encore fonctionnel (Ex: « Add the base system to read keyboard keys [WIP] »).
    Mais attention de ne pas en abuser. L'exemple précédent même sans [WIP] indique clairement que le système
    n'est pas complet.
  * Archiver ses propres changements régulièrement : la mise en commun au plus tôt permet
  à l'ensemble du groupe de voir les avancées (et moi aussi au passage). Savoir qui travaille
  sur quoi permet aussi de s'adresser aux bonnes personnes.

### Code

Au début d'un projet, le ration d'écriture/lecture du code est en faveur de l'écriture.
Cette tendance s'inverse rapidement et penche en faveur de la lecture très fortement.
Écrire du code, c'est avant tout en lire.

 Il est donc extrêmement important d'avoir une « copie propre » dans la base de code
 commune.

 Voici certains points que j'ai relevé qui doivent être améliorés pour vous éviter de la
 fatigue de lecture et de compréhension.

 Je m'attarde essentiellement sur `graphics` pour mes exemples car c'est la partie la plus développée.

 #### Typos

 ```rust
    let mut holden = ModelsHolder::new();

    holden.load_wavefront(&graphics, "textured_cube.obj", &ressources_path);
    holden.load_wavefront(&graphics, "reds.obj", &ressources_path);
    holden.load_wavefront(&graphics, "transparent_sphere.obj", &ressources_path);

```

Je comprends `Holder`, je comprends moins `holden`
(d'après le Collins: *archaic or dialect a past participle of hold*),
et cela ressemble plus à une faute de frappe. Si c'est vraiment le participe passé,
alors je ne comprends pas le sens, je reviendrai dessus dans la partie sur l'expressivité.

Il y a pas mal de tpos dans les messages de `commit` sur Git. Les messages sur Git sont
aussi importants que le code lui-même. Ils servent à transmettre des messages et doivent
être corrects.

La solution pour améliorer est d'utiliser un dictionnaire (certains IDE soulignent les
mots mal orthographiés dans le code) et de la relecture.

#### Cohérence des nommages

Dans les commentaires, il y a de l'anglais et du français. Choisissez l'un des deux.
Lire une ligne dans une langue et la ligne suivante dans l'autre apporte un peu de
fatigue qui n'est pas nécessaire.

Cohérence des commentaires avec le code : j'en ai parlé dans notre entrevue. Les commentaires
ont ce défaut qu'ils ne sont pas compilés. Ils nécessitent donc une attention particulière
et doivent décrire un fonctionnement sans le paraphraser.

Par exemple : `// list of teapots with position and direction`

La ligne qui suit ne correspond pas, suivant ma compréhension, au commentaire. Ce qui
suit serait pour moi :

```
// creation of a vector of tuples containing the individual positions, directions and sizes
// for teapots. The size of this vector will drive the number of instances created in the world.
```


C'est long, ça encombre, à peu près personne ne va lire le commentaire dans le futur.
Je reviendrai dessus dans l'expressivité des nommages plus bas.

### Conclusion

En terme de méthodologie, faites des choix et documentez-les. Lors d'un rapport
de projet, les jury aiment généralement poser des questions autour de la méthodologie
qui est une partie souvent oubliée dans les rapports.


## Technique

Dans ce paragraphe, je vais aborder les points plus techniques de la réalisation
du programme. Des choses s'appliquent à Rust en particulier, d'autre s'applique
à n'importe quel projet.

#### Expressivité des nommages

Je reviens sur `holden`. Si ce n'est pas une typo, je lis que cela représente quelque chose
qui est « gardé » par le `Holder`. Ce qui n'est pas le cas. `holder` est meilleur. J'irai même
franchement jusqu'à `modelHolder` : avoir l'instance locale (et la seule instance dans
ce cas) porter le même nom que son type, à la majuscule initiale prêt, est une convention
de nommage que j'aime asser (et qui est assez courante).

Je reviens sur le commentaire de `teapots`. Je disais que le commentaire était long, mais
il peut en fait se retrouve en grande partie dans le nommage de la variable. Par exemple :
```let mut teapotTransformParameters```.

Avec un nommage similaire à celui-ci, le seul commentaire que je vois nécessaire
est alors d'avertir que la taille de ce `Vector` implique le nombre d'instance de
teapots.

D'autres nommages me semblent peu explicites ou trompeurs : `per_instance` n'indique
pas à la lecture ce qu'est cette données. Il m'a fallu un certain temps à tracer le
code pour comprendre qu'il s'agissait d'un buffer de matrices de transformations
pour les teapots. `let mut teapotTransforms` aurait permis une compréhension plus
rapide.

La solution pour améliorer les nommages est de se poser la question : est-ce que
le nommage indique ce à quoi sert ce binding (variable, fonction, structure...).
Une bonne manière de faire est de faire relire par quelqu'un d'autre (sans explication
préalable autre que le message de `commit` qui y serait associé !)


#### Les Warnings

Au moment où j'écris ce document, il y a 21 warnings lors de la compilation de `graphics`.
Je suis assez strict sur les warnings. Pour moi, c'est 21 de trop.

La raison en est simple : les warnings indiquent que quelque chose dévie de ce qui
est attendue. Le résultat du programme ne devrait pas en être affecté (pas toujours
du moins, et moins en Rust qu'en C), mais le compilateur essaie d'attirer l'attention
sur quelque chose qui reste important.

Si ces warnings ne sont pas traités, ils vont se multiplier rapidement, et brouiller
l'information que le compilateur envoie (lorsque le 22ième warning apparaîtra, qui
le verra ?) et il indique aussi que l'on travail sur du code plus complexe que
nécessaire, ou pas tout à fait comme on l'attendrait. Cela signifie que ce qui est
écrit ensuite se base sur quelque chose de peut-être erroné, ou de plus complexe qu'il
n'y parait, et donc plus fatiguant.

Parenthèse : je parle souvent de fatigue inutile. C'est un concept important à mes
yeux. L'énergie dépensée lors de la construction d'un programme (mais cela fonctionne
pour toute autre activité de construction) doit essentiellement servir à cette
construction. Une analogie mécanique est qu'il faut éviter les frottements. Tout ce
qui provoque du frottement fatigue plus rapidement que nécessaire, et dégrade le
résultat, ou nous fait baisser les bras parce que 'c'est trop dur .

Fin de la parenthèse.

Dans les warnings, je vois :

  * des `unused` essentiellement. Que ce soit des  `import` ou des `variables`.
  Remède : enlevez tout ce qui est `unused`. Vous perdez plus d'énergie à le
  conserver qu'à le réécrire le jour où cela sera à nouveau nécessaire.
  * de la mutabilité inutile (mais souvent parce que la variable est `unused`).
  * de la redondance de nommage de paramètre lors de l'initialisation de
  structures. Un soucis ici est que la redondance est partiel, du fait de nommages
  non cohérents entre les noms des champs et les paramètres.

Ce n'est pas long à corriger, et ça nettoiera le code à peu de frais.

### Unwraps

En Rust, `unwrap()`, ou en tout cas sa multiplication, est un code smell (https://fr.wikipedia.org/wiki/Code_smell).

Il y en a vraiment beaucoup actuellement dans le code, et j'ai essayé de comprendre pourquoi.
D'après mon analyse, ils sont là sous plusieurs formes.

Je vais en prendre quelques-uns.

#### crate_path

```rust
    let crate_path = executable_path
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();
```

Dans ce passage, même quelqu'un qui ne connaît pas Rust va probablement tiquer et se poser
les questions : pourquoi cette succession de `parent().unwrap()` (peu importe ce que
cela fait) et pourquoi trois répétitions ?

D'un point de vue plus Rust, si un des unwrap() ne passe pas (si le working directory n'a
pas au moins trois niveaux de profondeurs), le program va lancer un panic() sans trop
de contexte.

Deux choses à améliorer selon moi.

Le but ici est d'obtenir le `resource_path`, les autres variables sont intermédiaires.
Je m'attends donc à avoir une seule ligne qui ressemblerait à : `let resource_path = get_resource_path(std::env::args().collect())`
(il y a plusieurs de choix discutables dans ma proposition).

La manière dont le path est obtenu n'est pas de la responsabilité de `main`. La seule
chose qui importe est que cette fonction a besoin des arguments (actuellement uniquement
le premier éléments, mais on pourrait imaginer que des paramètres de ligne de commande
donnent des indications sur le resource_path).

C'est un guide général qui ne concerne pas les unwrap() : séparer les responsabilités.
Des lignes qui effectuent un traitement qui ne sont pas de la responsabilité locale
doivent aller ailleurs.

La seconde amélioration est sur le unwrap(). Puisqu'il s'agit d'itérer, il vaut mieux utiliser
les nombreuses possibilités d'itérations de Rust. Et plutôt que de passer par Vec<String> via
un `collect()`, autant utiliser `ancestors()` qu'offre Path pour cette utilisation.

Voici un exemple de proposition (avec de nombreux choix discutable). J'attire l'attention
sur les deux `panic!`.

Celui dans la fonction `get_resource_path` est là car, structurellement, la fonction
ne peut fonctionner que si les arguments ont le format attendus. C'est donc un
contrat au niveau de la fonction (un prérequis du système, on s'attend à ce que
le système envoie le chemin de l'exécutable en premier élément).

Le `panic!()` à l'extérieur résulte d'une erreur retournée par la fonction. Le contrat
est donc : si les prérequis sont bons, je m'attends à pouvoir trouver le resource path.
Si je ne le peux pas *à cause des données*, alors c'est une erreur utilisateur.

Ce contrat est un choix que j'ai fais et qui est complètement discutable. Ce sont des
questions à se poser à l'écriture de chaque fonction.

```rust
use std::path::{Path, PathBuf};

fn get_resource_path(mut args: std::env::Args) -> Result<PathBuf, String> {
    const UP_LEVEL_COUNT: usize = 3;
    match args.next() {
        Some(parameter) => {
            let executable_path = Path::new(&parameter);
            let maybe_crate_path = executable_path.ancestors().nth(UP_LEVEL_COUNT);

            match maybe_crate_path {
                Some(crate_path) => Ok(crate_path.join(Path::new("ressources"))),
                _ => Err(format!("Cannot go up {} levels in the executable path.", UP_LEVEL_COUNT))
            }
        },
        _ => panic!("Argument list to the executable is empty, missing the executable path.")
    }
}

fn main() {
    let resource_path = match get_resource_path(std::env::args()) {
        Ok(path) => path,
        Err(reason) => panic!("Program cannot work... {}", reason)
    };

    println!("Resource path is {:?}.", resource_path);
}
```

Autre note : le nombre de niveaux à monter est sous forme de constante. Cela explicite
ce que l'on veut faire et permet de changer la valeur tout en gardant la cohérence.


#### Retours d'appels à glium

Les `unwrap()` sur les appels à glium sont des unwrap() de `Result`. On peut effectivement
considérer de passer en `panic` si ça ne passe pas dans un premier temps. Mais ça n'est
pas solide sur le long terme.

Ça en fait quand même beaucoup et, relié à ce que je traiterai plus tard sur l'architecture,
il n'est pas normal d'avoir des appels à **glium** directement dans `main.rs`. Du coup,
encapsuler ça avec un appel de fonction renvoyant une seule erreur globale et forwarder
les erreurs de glium via l'utilisation de l'opérateur '?' assainirait un peu les
choses.

#### Option vs. Result

Les deux structures sont similaires mais ont des sémantiques différentes.

`ModelHolder` renvoie des types `Option` sur les `get`. Je verrais plutôt des types `Error`.
De même pendant lors de l'appel d'un `load()` sur `ModelHolder`.

Ensuite, il faut choisir l'endroit, la « cloison » entre l'espace incertain des erreurs et
des options, et l'espace certain où les données sont présentes et sans erreur.

Cet endroit est actuellement dans `main.rs`, ce qui est beaucoup trop haut. `main` ne
devrait pas à avoir à se soucier de la validité des données en faisant les `unwrap` à
ce moment-là. Au maximum, `main` peut demander à un autre système de charger les donner
et prendre une seule décision qui est, en fonction d'erreurs éventuelles, est-ce que le
programme continue ou pas.

#### Unwrap dans les couches basses

Lorsque la boucle moteur est lancée et qu'on est dans les couches les plus basses, comme
la construction de la frame, ce n'est plus le moment de vérifier la validité des données.

D'abord parce que c'est lent, et ensuite car ces données peuvent venir de loin et que
lors d'un debug, il est compliqué de remonter la chaîne de transformation des données
pour trouver l'erreur.

Les `self.frame.draw( (vertex_buffer, per_instance.per_instance().unwrap()),` se retrouvent
à trois endroits, dans les couches basses, avec à chaque fois un `unwrap()` sur l'erreur
potentielle renvoyée par `per_instance())`.

`per_instance()` renvoie une erreur si l'instanciation n'est pas supporté par la carte
graphique et ne devrait pas (cf. le commentaire dans la fonction)... mais puisque
c'est le cas et qu'il faut vivre avec, alors il vaut mieux appeler `per_instance()` plus
haut, ou au pire en début de fonction à un seul endroit.

Il n'y a aucune chance que la matériel se mette à ne plus supporter l'instanciation
pendant une frame, donc il n'est pas nécessaire de traiter l'erreur à chaque fois.

Côté .unwrap() de `draw`... il vaudrait mieux que la fonction draw() de `Frame` renvoie aussi un Result<> et
utiliser le chaînage d'erreurs avec l'opérateur '?'. Ça allégerait l'écriture et c'est
fait pour.


#### Matrices

Dans `main`, il y a différentes utilisations qui représentent à chaque fois une matrice 4x4.
Une unification et une encapsulation serait les bienvenues.

Avez vous essayé d'utiliser Matrix4 pour la définition des vertices ? Si c'est possible,
ça unifierait le type utilisé pour les matrices.

Si ce n'est pas le cas, alors un type simple de matrice pour vos usages serait un minimum.
Cela évite d'écrite in extenso : `world_transformation: [[0.; 4]; 4],` mais plutôt un
`world_transformation: matrix!(null)` par exemple. De même, mieux vaut écrire une
macro *passe plat* qui permettrait d'écrire :

```rust
        &vec![Attr {
            world_transformation: matrix!([
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, -1.0, 0.0, 1.0],
            ]),
        }],
```

La différence peut paraître minime (la macro se contente de réécrire son contenu) mais
la différence est énorme et terme d'évolution et de lecture. D'abord, vous indiquez
qu'il s'agit d'une matrice. Ensuite, si le format change, il suffira de changer
la macro à un seul endroit.

Note : cependant, au final dans un moteur, ces données devraient probablement venir d'un fichier
de données.

#### Simplicité

Attention aux structures alambiquées à base d'iterateurs et de collect() à la fin.
Surtout pour l'initialisation.

```rust
        let data = teapots
            .iter()
            .map(|_| Attr {
                world_transformation: [[0.; 4]; 4],
            })
            .collect::<Vec<_>>();
```

peut s'écrire

```rust
        let data = vec![Attr{world_transformation: [[0.; 4]; 4]}; teapots.len()];
```

Ou, pour être un peu plus explicite, par exemple :

```rust
        let data = vec![Attr::null(); teapots.len()];
```

Ou en utilisant un constructeur sur `Attr`, ce qui serait le mieux si cette initialisation
a du sens plus largement.

Ce qui m'a attiré le regard ici était initialement l'utilisation de `map` en ignorant le
paramètre de la closure. C'est louche.


## Architecture

À présent que vous avez un bon morceau d'affichage qui tourne, avec une gestion
des inputs, vous allez pouvoir regarder si une architecture se dégage afin de trier
les différents éléments.

Il y a déjà un début d'architecture avec une séparation entre `engine` et `processing`, qu'il
va falloir pousser plus loin.

Par exemple, une question à se poser est sur la manière dont est représentée la scène.
On voit apparaître des choses dans le `main` de graphics qu'il va falloir extraire sous
des concepts plus génériques.

### Responsabilités

Comme je le disais lors de notre réunion, une méthode peut être de se poser la question
de la responsabilité lorsque l'on extrait une *partie* (une fonction, une structure, un module).

Ce nombre de responsabilités devrait être limité, idéalement 1 (ce n'est pas toujours
si facile en pratique).

Par exemple : quelle est la responsabilité de `Frame`. Je tente un : *`Frame` est
responsable de la construction d'une image à partir d'objets graphiques*. Ça colle
dans les grandes lignes, sans s'occuper des contrats sur comment sont présentés
ces objets. C'est ok.

La responsabilité de `Camera` me semble assez facilement trouvable.
La responsabilité de `Graphical` est un peu plus obscure pour moi.

### Dépendances

Un autre aspect important est celui des dépendances. Plus une construction a de
dépendances, plus elle va être sensible à des changements qui ne la concerne pas,
ou très indirectement. Et plus le programme sera complexe à maintenir.

Il est important de minimiser les dépendances.

Il faut aussi éviter un cycle de dépendance, car alors cela forme un groupe
compact extrêmement complexe a maintenir. Je n'ai pas regardé s'il y en avait.

### main.rs

`main.rs` a pour le moment énormément de dépendances et énormément de responsabilités.
Et ces responsabilités sont de différents niveaux : du la gestion de la boucle à la
création de vertex buffers.

Une des prochains étapes pour moi est de découper tout ça.

Au final, `main()` devrait être une toute petite fonction qui se charge de
démarrer les systèmes (ou même un seul système, le moteur, avec des paramètres
de départ).

À titre de référence, je mets juste après l'intégralité du fichier `main.cpp`
d'un projet à visé de formation que j'ai écrit il y quelques mois. C'est du C++,
mais le volume de code reste similaire.

```c++
#include "engine/Engine.h"

#include "actors/StartupActor.h"

int main()
{
    auto startupScript = std::make_unique<StartupActor>();
    Engine engine(std::move(startupScript));

    while (engine.is_running())
    {
        engine.loop();
    }

    return 0;
}
```

## Les différents crates

Dans cette partie, je fais un commentaire global sur chacun des crates.

### Graphics

C'est la partie la plus avancée du projet et probablement celle qui va donner naissance
au moteur en soi. Il y a déjà un petit morceau d'architecture et on voit apparaître
des structures (principalement dans la fonction main()) qui devraient diriger la suite.

Attention de ne pas leaker les structures (et même parfois les concepts) des librairies
utilisées dans le code de haut niveau.

### ECS

L'utilisation d'une bibliothèque externe me semble un bon choix. Il y a eu pas mal
de temps passé en début de projet sur l'implémentation d'un système d'entités. Ça
sera intéressant d'en parler dans le rapport je pense.

Construire un ECS est une tâche complexe, j'en reparlerai dans la partie sur les
choix.

### Physics

Ma compréhension est que pour le moment, il s'agit avant tout de tests de prise en
main de la bibliothèque. Une fois que c'est fait, il faudra se poser la question
de l'intégration au moteur en général.

### Test_ui2

Si j'ai bien compris, il s'agit du reliquat des tests faits avec SDL2 pour le
traitement des événements. J'imagine que cela ne changera plus et que le crate
est là à titre d'archive.

## Les choix

Puisque `choix.md` vient d'apparaître, ce qui est une très bonne chose, voici mes
commentaires sur les choix et leurs justifications, dans l'ordre où ils apparaissent.

### Rust

Je conseille de tourner la justification vers les avantages intrinsèques de Rust, sa
philosophie. La comparaison avec le C++ est un peu dangereuse si vous devez la défendre,
tout en disant ne pas être une équipe de développeurs chevronnés.

La notion de perte de temps sur les pointeurs peut être complexe à défendre. On peut
y opposer qu'à l'inverse, Rust étant intransigeant sur son modèle mémoire, il y a une
perte de temps à trouver les bonnes manière de faire, avec la bonne syntaxe.

On peut présenter ses avantages (réels) et justifier le choix par
une volonté d'étudier un langage qui s'appuie sur les expériences et les connaissances
acquises par de nombreux langages, dont le C++.

Les avantages peuvent être :
  * une gestion rigoureuse de l'ownership des structures qui sous tend une gestion
  automatique de la durée de vie des structures et donc de leurs allocation sans avoir
  recours à un garbage collector.
  * un effort au niveau du compilateur pour présenter des messages d'erreurs les plus
  clairs possibles ainsi que des propositions de correction.
  * une grande disponibilité de bibliothèques et d'outils grâce à un grand dynamisme autour
  de ce nouveau langage (de nombreux langages inventés souffrent en premier lieu d'un
  manque de bibliothèque et d'outils).

De manière générale dans un rapport ou une justification de choix, ne partez jamais sur
une critique négative de quelque chose que vous ne maîtrisez pas à fond. Il y aura
toujours le risque d'être challengé sur ce point, surtout sur quelque chose d'aussi
connu que le C++.

Plutôt que de travailler une partie « critique du C++ », travaillez sur une justification
positive du choix du Rust. Ainsi, à la question évidente : « pourquoi ne pas avoir choisi C++, qui
est le standard dans le domaine ? », vous pouvez répondre que c'est l'envie d'aller
chercher la nouveauté, pour éventuellement comprendre ce que signifie faire du jeu
vidéo en Rust, et s'appuyer sur le fait que cela m'intéresse d'un point de vue
industriel.

Le fait d'avoir un membre du groupe ayant une connaissance préalable du langage choisi
est aussi un bon atout et peut faire partie de la justification.


### OpenGL

Le choix est bon et ici encore, ne dévalorisez pas OpenGL. OpenGL est encore
vastement utilisé et « être vieux » peut aussi signifier « être solide », « avoir
beaucoup de support » ainsi que bon nombre de choses positives.

L'âge n'est donc pas un contre-argument. Et Vulkan est le petit nouveau dans le secteur,
il n'est pas majoritaire encore. Et il y a une bonne double raison : il est beaucoup
plus complexe à maîtriser (et il y a moins de recul, moins d'expérience général), et
son support n'est pas disponible sur toutes les plateformes.

Le passage sur le changement de choix dans la bibliothèque est intéressant.

Le fait que le créateur initial soit parti me semble peu intéressant si la
bibliothèque est maintenue derrière.

Ce que je comprends, c'est que le choix d'OpenGL est du à :
  * Une connaissance de la part du professeur, qui permet de vous guider si nécessaire
  * Avoir trouvé une bibliothèque active et documentée, wrapper Rust d'OpenGL
  offrant des abstractions « rustiennes » cachant la tuyauterie d'OpenGL (ex: choisir
  les bonnes extensions, gérer les capabilities, les formats de vertex,...)
  * À son support multi-plateforme.

Il est intéressant de présenter les autres choix possibles de manière simple :
  * DirectX : limité à Windows alors que vous voulez que votre projet fonctionnent au moins sous Windows et Linux
  * Vulkan : concepts plus complexes qui vous auraient demandés plus de temps à comprendre et mettre au point

### ECS

Pour le moment, ce choix ne me paraît pas être fait ailleurs que sur papier. Je vais
tenter d'expliquer pourquoi.

La partie Entity Component d'un ECS est simple à implémenter et à comprendre.

La difficulté de conception se trouve dans deux parties.

La première est le découpage en composants. Comment faire le découpage ? Comment
gérer les dépendances entre composants ? Choisir entre un découpage trop fin et une
grosse granularité... Tout cela est plus facile si on a déjà implémenté un jeu.

La deuxième est au niveau des systèmes. Qu'est-ce qu'un système ? De quoi est-il
responsable ? De quelles données a-t-il besoin ? Cela a une dépendance et une implication
forte aux composants.

Les ECS sont des architectures orientées performances, mais ne le sont que si l'analyse
des données, la découpe des composants et des systèmes, est bien faite. Dans le cas
contraire, penser sous forme d'ECS apporte une forte contrainte et le gain est plus flou.
On peut y voir un gain d'architecture aussi mais...

Parenthèse à ce sujet : j'ai lu l'article pris en référence sur le Closing Keynote
de RustConf 2018. Il y a de bonnes choses... et des plus sujettes à discussion. Le cheminement qui
s'appuie sur un rejet de la POO a exactement l'effet que j'indique plus haut : il
est attaquable. Du coup, quelques conclusions sont bonnes, mais partent de prémisses
pas totalement justes.

Les problématiques qui sont apparues sur les dépendances entre les objets n'impliquent pas
qu'il faut passer à un ECS. L'auteur du jeu « Celeste » avait publié le code source de
la gestion principale du jeu et avait montré un choix radicalement différent (qui
avait été très critiqué, mais qui a ses avantages) qui réglait tous ces problèmes
sans ECS.

Et d'expérience, j'ai vu des projets régler ces problèmes de dépendance dans un système
de POO sans soucis. Mais avec une architecture plus fouillée.

Fin de la parenthèse.

Ma conclusion est double :
  * un ECS ne résous pas magiquement des problèmes, c'est un principe d'architecture,
  * en tant que principe, je déconseille de _choisir_ une architecture ECS tant qu'elle s'est pas
  dégagée d'elle-même de la conception.

C'est pour cela que je proposais lors de la première réunion de partir sur un cube qui
tourne, puis qui est manipulé, et de construire des briques jusqu'à en dégager les
concepts.

C'est à ce moment-là, quand les concepts du moteur sont apparents, que l'on peut choisir
d'architecturer un ECS autour, si c'est le bon schéma.

Penser en amont que l'on va dans cette direction aide à y aller, bien entendu. Et c'est
en ce sens que je disais en début de paragraphe que ce choix ne me semblait pas
fait ailleurs que sur le papier.

Et c'est aussi pour cela que je posais la question à la dernière rencontre sur comment
vous voyiez la rencontre entre "ecs" et "graphics". Pour le moment, elle ne me semble
pas évidente. Elle devrait l'être lorsque vous aurez fait se rencontrer un concept
d'entité, de son affichage et de sa représentation physique.

Et en ce sens, il est bon d'avoir choisi une bibliothèque externe, qui va mettre de côté les
détails d'implémentation pour se concentrer sur le « comment » mettre ça en place.

### Physics

Je n'ai pas encore regardé cette partie. Les arguments de documentation et d'activité
sur la bibliothèque sont pour moi recevables à défaut d'autres comparaisons qui
nécessiteraient des essais sur une base de moteur existant (et du temps).

L'argument du temps pour refaire des bindings vers Bullet est aussi recevable.
Vous choisissez quelque chose qui fonctionne pour vous concentrer sur l'intégration
au moteur d'un système physique.

Comme ce choix a été challengé par le professeur pendant la réunion, et qu'il y a donc
des chances qu'il le soit de manière générale, il me semble important de bien
encapsuler son utilisation.

Dans l'intégration de la physique au moteur, si vous montrez que l'architecture isole
la bibliothèque du reste (ce qui devrait normalement être fait pour chaque bibliothèque),
vous montrez que ce choix peut être changé si les arguments évoluent.

Ainsi, si le moteur devait évoluer car cette bibliothèque n'est pas performante, changer
dans le futur (hypothétique) vers un binding bullet par exemple serait un travail
sans impact sur le reste.

### Inputs

Indiquer votre cheminement est intéressant. Le choix de Glutin semble naturel
après avoir choisi Glium.

Si vous gardez « pour l'instant », cela signifie que vous allez faire un effort
d'encapsulation de Glutin à part de Glium. Je serais plutôt d'avis que vous
considériez Glium + Glutin comme un tout, encapsulé ensemble vis à vis du
reste du moteur.
