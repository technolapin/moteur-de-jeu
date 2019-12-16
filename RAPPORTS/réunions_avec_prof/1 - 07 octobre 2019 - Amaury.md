# Réunion du 07 octobre 2019

Réunion avec Nabil MUSTAFA.

## Notes de Amaury

### Introduction

Project goes to may.

OpenGL : Library for video games. We will make something like this. Computer graphic.

Nabil will discuss with Sylvain about the language. But he think it's better to use C++.

Nabil has a course about computer graphic in E5. He will send us his course and his code.

Computer graphic is general. Animation, collision detection, particles, interactions...

3D or 2D, but Nabil prefers 3D. If you can do 3D you can 2D. Conclusion : 3D game engine.

On va voir de l'alègre linéaire. AAARRRGGGHHH !

### Maths Stuffs

#### Points and vectors

A point is reprezented by 3 numbers : X, Y and Z. Same for a vector.
Un vecteur est une "flèche" dans l'espace. C'est-à-dire la notiion de vecteur vue au lycée.
Un vecteur n'est pas fixé à un point de l'espace.

Le vecteur XYZ est la flèche qui va de l'origine au point XYZ.

Point : Position in space.
Vector : A direction.

A point + A vector = A point

A vector + A vector =  A vector (A direction)

A point + A point means nothing.

#### Dot product (Between two vectors) = Produit scalaire

V1 ∙ V2 = (X1, Y1, Z1) ∙ (X2, Y2, Z2) = X1 X2 + Y1 Y2 + Z1 Z2 = |V1| * |V2| * cos ( angle between V1 and V2 )

|vector| = length of the vector

#### Cross product (Between two vectors) = Produit vectoriel

Donne un vecteur perpendiculaire entre les deux vecteurs.
Si on inverse l'ordre de ces deux vecteurs, donne le même vecteur mais dans l'autre sens.

#### Types of objects

3 types of things : Ray (Point + vector), segment (Between two points), and line (Two points).

A plane : 3 points (But it's not a good idea, not flexible). Or a point and a normal unit vector (Vecteur de longueur 1). This unit vector is normal to the plane.

Is the point Q in the plane defined by point P and vector V : (vector PQ) ∙ V = 0

Vertex = Un sommet du mesh.

### 3D Scene

Objects, textures, lights, shadows, and you (Camera). The most basic thing is the object.

#### Object

It is called a **mesh** !

A mesh is made of points, segments ("arètes"), and faces.

If you have a hundred chairs, you store one chair and store a hundred transformations.

#### Camera

Is a point and two vectors : One for direction and another for rotation (Foward and up).

### Maths stuffs again

Use 4D vectors, add homogenous coordinate. Produit matriciel plus simple / efficace / j'ai pas bien compris.

(X, Y, Z, 1) = (2X, 2Y, 2Z, 2) = (10X, 10Y, 10Z, 10)

### Conclusion

We will make a 3D engine from scratch.

Prefer that we make something ambitious and fail. But work hard on it.
