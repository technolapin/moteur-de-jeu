Nous avons choisis Rust pour ses avantages comparés à C++.
Ne disposant pas d'une équipe de développeur chevronés, nous ne pouvons nous permettre de perdre du temps sur des erreures de pointeur, d'allocation mémoire, ou de messages peu clairs de la part du compilateur.
Rust est, de plus, plus facile à aborder que C++, car quelques features du langage couvrent de nombreuses autres features de C++, ainsi que pour les raisons citées précedements.
De plus, des gens font déjà du jeu vidéo en rust depuis plusieures années, et Clément étudiait déjà Amethyst, un moteur de jeu open source en Rust.
Le point négatif de Rust est sa jeunesse, qui restreint un peu le catalogue de libs disponibles. Néanmoins, tout ce dont on a besoin existe (car comme mentionné précedement, des gens font déjà du jeu vidéo en rust depuis longtemps).

---

Nous avons hésité assez longuement entre opengl et vulkan (enfin plutôt, entre opengl et rendy qui est basé sur vulkan).
Le premier est vieux et largement délaissé au profit de vulkan et de ses dérivés, mais d'un autre côté nous disposons d'un professeur compétent en OpenGL.
Personne dans le groupe n'ayant de réelles compétences en l'un ou l'autre, nous avons décidé d'opter pour opengl.
Nous avons perdu beaucoup de temps sur une lib mal documentée et surtout pas prévue pour être utilisée sérieusement, puis nous avons basculé sur glium. Cela nous a semblé être la meilleure alternative, étant donné sa simplicité d'usage que la qualité de sa documentation.
Glium n'est d'ailleurs plus maintennue par son propriétaire (qui est parti faire du vulkan), mais une communauté assez active de développeurs continue à mettre à jour et expandre cette lib, et est plutôt réactive quant au traîtement des issues.

---

Nous avons choisis de baser notre moteur sur une ECS.
Clément a d'abord tenté d'en implémenter une ex nihilo, mais il s'est avéré qu'il était illusoire qu'une personne seule arrive à produire une alternative viable à ce qui existe déjà, notamment SPECS (que Clément connaissait déjà), qui est bien connu dans le milieux.
Ainsi nous utilisons SPECS.

---

Pour la physique, nous avons comparé les bindings bullet avec nphysics.
Il se trouve qu'il n'existe pas de binding pour bullet disposant d'une doc suffisante pour être utilisé, et les rares projets que nous avons trouvés semblaient plus être des ébauches qu'autre chose en plus de ne pas avoir été mis à jour depuis plusieurs années.
nphysics, par contre, est une librairie modulaire extrêmement complète disposant d'une excellente documentation et était largement utilisée et maintenue par une communauté conséquente (et est notement utilisée par Amethyst)
Nous avons donc choisi d'utiliser nphysics, ncollide et nalgebra.

---

Pour les inputs, nous avons d'abord tenté d'utiliser une librairie de binding de sdl2 que nous avions déjà un peu utilisée auparavant.
Nous avons cependant rencontré un problème: les zones d'affichage de glium ne sont pas compatibles avec cette lib, tout simplement car les devs ne l'ont pas prévu.
Nous avons alors choisit pour l'instant d'utiliser glutin.
Glutin est une lib de relativement bas niveau sur laquelle s'appuie glium (beaucoup d'objets de glium comme le contexte sont de fait des objets de glutin).
Il se trouve que Glutin permet assez facilement de récupérer et filtrer les évènements. Assez facilement pour que chercher plus facile perde de de son intérêt.
Nous utilisons donc Glutin pour l'instant.

---

Gestion audio : Librairie "ears".
Installation : `sudo apt install libopenal-dev libsndfile1-dev`
Doc : https://docs.rs/ears/0.7.0/ears/struct.Music.html
Choix avec rg3d_sounds, écarté car ne possède pas de doc suffisante.
