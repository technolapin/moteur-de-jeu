extern crate nalgebra as na;

use std::env; // Pou récupérer les arguments de la commande
use std::vec::Vec;
use na::Vector3;
use nphysics3d::object::{DefaultBodySet, DefaultColliderSet};
use nphysics3d::force_generator::DefaultForceGeneratorSet;
use nphysics3d::joint::DefaultJointConstraintSet;
use nphysics3d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use ncollide3d::shape::{Ball, Capsule, Compound, ConvexHull, Cuboid, 
                        HeightField, Plane, Polyline, Segment, TriMesh, 
                        Triangle, ShapeHandle};
use nphysics3d::object::{BodyPartHandle, ColliderDesc, RigidBodyDesc,};



// On fait une énum pour répertorier tous les types de Mesh qu'on peut créer avec ncollide
enum MeshType {
    Ball,
    Capsule,
    Compound,
    ConvexHull,
    Cuboid,
    HeightField,
    Plane,
    Polyline,
    Segment,
    TriMesh,
    Triangle
}

struct Coordinates{
    x: f32,
    y: f32,
    z: f32
}

struct Objet {
    position: Coordinates,
    speed: f32,
    mass: f32,
    can_move: bool,
    mesh: MeshType
}

struct ObjSet{
    tab: Vec<Objet>,
    length: f32
}



// ### À compléter ###
fn build_object_table() -> Vec<Objet>{
    // Fo mèt du cod
    let tab = Vec::new();
    return tab;
}

fn build_obj_set(tab: Vec<Objet>, length: f32) -> ObjSet{
    ObjSet {
        tab,
        length
    }
}



/* 
 Fonction appelée par process_mesh si c'est une Ball
 Retourne un RIgidBody correspondant à la Ball
 */
fn process_ball(objet: Objet) -> RigidBodyDesc<Ball>{
    // Coordonnées et rayon de la Ball
    let x = objet.position.x;
    let y = objet.position.y;
    let z = objet.position.y;
    let rad = 0 as f32;

    // Création d'une Ball
    let Ball = ShapeHandle::new(Ball::new(rad));

    // Création du RigidBody de la Ball
    let rb = RigidBodyDesc::new()
        .translation(Vector3::new(x, y, z))
        .build();

    return rb;
}



// Fait des trucs inutiles pour le moment
fn process_mesh(event: MeshType) {
    match event {
        MeshType::Ball => println!("Ball"),
        MeshType::Capsule => println!("Capsule"),
        MeshType::Compound => println!("Compound"),
        MeshType::ConvexHull => println!("ConvexHull"),
        MeshType::Cuboid => println!("Cuboid"),
        MeshType::HeightField => println!("HeightField"),
        MeshType::Plane => println!("Plane"),
        MeshType::Polyline => println!("Polyline"),
        MeshType::Segment => println!("Segment"),
        MeshType::TriMesh => println!("TriMesh"),
        MeshType::Triangle => println!("Triangle"),
    }
}

// Fait des trucs inutiles pour le moment
fn main() {
    let mut mechanical_world = DefaultMechanicalWorld::new(Vector3::new(0.0, -9.81, 0.0));
    let mut geometrical_world = DefaultGeometricalWorld::new();

    let mut bodies = DefaultBodySet::new();
    let mut colliders = DefaultColliderSet::new();
    let mut joint_constraints = DefaultJointConstraintSet::new();
    let mut force_generators = DefaultForceGeneratorSet::new();


    /*
    ### Marche pas ###

    // Vérifie qu'il y a le bon nombre d'arguments
    let mut c = 0;
    for argument in env::args() {
        c += 1;
        if c==1{
            let obj_set = env::args(c);
        }    
    }
    if c!=2{
        println!("Wrong number of argument: {} \n Expected 2", c);
    }
    */

    let tab = build_object_table();
    let length = 0 as f32; // Demander à Clément comment on a la taille d'un vec
    let obj_set = build_obj_set(tab, length); 



    // On itère sur le set d'objets
    for objet in &obj_set{
        let rb = process_mesh(&objet.mesh); 
    }

    // Ajout du RigidBody au set de RigidBody
    let rb_handle = bodies.insert(rb);




    let itsaball  = MeshType::Ball;
    /*let itsacapsule  = MeshType::Capsule;
    let itsacompound  = MeshType::Compound;
    let itsaconvexhull  = MeshType::ConvexHull;
    let itsacuboid  = MeshType::Cuboid;
    let itsaheightfield  = MeshType::HeightField;
    let itsaplane  = MeshType::Plane;
    let itsapolyline  = MeshType::Polyline;
    let itsasegment  = MeshType::Segment;
    let itsatrimesh  = MeshType::TriMesh;
    let itsatriangle  = MeshType::Triangle;*/

    process_mesh(itsaball);
    /*process_mesh(itsacapsule);
    process_mesh(itsacompound);
    process_mesh(itsaconvexhull);
    process_mesh(itsacuboid);
    process_mesh(itsaheightfield);
    process_mesh(itsaplane);
    process_mesh(itsapolyline);
    process_mesh(itsasegment);
    process_mesh(itsatrimesh);
    process_mesh(itsatriangle);*/
}
