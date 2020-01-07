extern crate nalgebra as na;

////use std::env; // Pour récupérer les arguments de la commande
use std::vec::Vec;
use na::Vector3;
use na::base::{Unit, DMatrix};
use na::geometry::{Point2, Point3};
use nphysics3d::object::{DefaultBodySet, DefaultColliderSet};
use nphysics3d::force_generator::DefaultForceGeneratorSet;
use nphysics3d::joint::DefaultJointConstraintSet;
use nphysics3d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use ncollide3d::shape::ShapeHandle;
use ncollide3d::shape;
use nphysics3d::object::{RigidBodyDesc, RigidBody};
use ncollide3d::math::{Point, Isometry};


struct Ball
{
    radius: f32
}

struct Capsule
{
    half_height: f32,
    radius: f32
}

struct Compound
{
    shapes: Vec<(Isometry<f32>, ShapeHandle<f32>)>
}

struct ConvexHull
{
    points: Vec<Point<f32>>
}

struct Cuboid
{
    vector: Vector3<f32>
}

struct HeightField
{
    heights: DMatrix<f32>,
    scale: Vector3<f32>
}

struct Plane
{
    normal: Unit<Vector3<f32>>
}

struct Polyline
{
    points: Vec<Point<f32>>,
    indices: Option<Vec<Point2<usize>>>
}

struct Segment
{
    a: Point<f32>,
    b: Point<f32>
}

struct TriMesh
{
    points: Vec<Point<f32>>,
    indices: Vec<Point3<usize>>,
    uvs: Option<Vec<Point2<f32>>>
}

struct Triangle
{
    a: Point<f32>,
    b: Point<f32>,
    c: Point<f32>
}

// On fait une énum pour répertorier tous les types de Mesh qu'on peut créer avec ncollide
enum MeshType {
    Ball(Ball),
    Capsule(Capsule),
    Compound(Compound),
    ConvexHull(ConvexHull),
    Cuboid(Cuboid),
    HeightField(HeightField),
    Plane(Plane),
    Polyline(Polyline),
    Segment(Segment),
    TriMesh(TriMesh),
    Triangle(Triangle)
}

// On implémente le trait Copy à la structure
#[derive(Copy, Clone)]
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
 Retourne un RigidBody correspondant à la Ball
 */
fn process_ball(ball: Ball, position: Coordinates) -> RigidBody<f32>{
    // Coordonnées et rayon de la Ball
    let x = position.x;
    let y = position.y;
    let z = position.z;
    let radius = ball.radius;

    // Création d'une Ball
    let Ball = ShapeHandle::new(shape::Ball::new(radius));

    // Création du RigidBody de la Ball
    let rb = RigidBodyDesc::new()
        .translation(Vector3::new(x, y, z))
        .build();

    return rb;
}

fn process_capsule(capsule: Capsule, position: Coordinates) -> RigidBody<f32>{
    // Coordonnées et rayon de la Capsule
    let x = position.x;
    let y = position.y;
    let z = position.z;
    let half_height = capsule.half_height;
    let radius = capsule.radius;

    // Création d'une Capsule
    let Capsule = ShapeHandle::new(shape::Capsule::new(half_height, radius));

    // Création du RigidBody de la Capsule
    let rb = RigidBodyDesc::new()
        .translation(Vector3::new(x, y, z))
        .build();

    return rb;
}

fn process_compound(compound: Compound, position: Coordinates) -> RigidBody<f32>{
    // Coordonnées et rayon du Compound
    let x = position.x;
    let y = position.y;
    let z = position.z;
    let shapes = compound.shapes; 

    // Création d'un Compound
    let Compound = ShapeHandle::new(shape::Compound::new(shapes));

    // Création du RigidBody du Compound
    let rb = RigidBodyDesc::new()
        .translation(Vector3::new(x, y, z))
        .build();

    return rb;
}

fn process_convexhull(convexhull: ConvexHull, position: Coordinates) -> RigidBody<f32>{
    // Coordonnées et rayon du ConvexHull
    let x = position.x;
    let y = position.y;
    let z = position.z;
    let points = convexhull.points;

    // Création d'un ConvexHull
    let ConvexHull = ShapeHandle::new(shape::ConvexHull::try_from_points(&points).unwrap());

    // Création du RigidBody du ConvexHull
    let rb = RigidBodyDesc::new()
        .translation(Vector3::new(x, y, z))
        .build();

    return rb;
}

fn process_cuboid(cuboid: Cuboid, position: Coordinates) -> RigidBody<f32>{
    // Coordonnées et rayon du Cuboid
    let x = position.x;
    let y = position.y;
    let z = position.z;
    let vector = cuboid.vector;

    // Création d'un Cuboid
    let Cuboid = ShapeHandle::new(shape::Cuboid::new(vector));

    // Création du RigidBody du Cuboid
    let rb = RigidBodyDesc::new()
        .translation(Vector3::new(x, y, z))
        .build();

    return rb;
}

fn process_heightfield(heightfield: HeightField, position: Coordinates) -> RigidBody<f32>{
    // Coordonnées et rayon du HeightField
    let x = position.x;
    let y = position.y;
    let z = position.z;
    let heights = heightfield.heights;
    let scale = heightfield.scale;

    // Création d'un HeightField
    let HeightField = ShapeHandle::new(shape::HeightField::new(heights, scale));

    // Création du RigidBody du HeightField
    let rb = RigidBodyDesc::new()
        .translation(Vector3::new(x, y, z))
        .build();

    return rb;
}

fn process_plane(plane: Plane, position: Coordinates) -> RigidBody<f32>{
    // Coordonnées et rayon du Plane
    let x = position.x;
    let y = position.y;
    let z = position.z;
    let normal = plane.normal;

    // Création d'un Plane
    let Plane = ShapeHandle::new(shape::Plane::new(normal));

    // Création du RigidBody du Plane
    let rb = RigidBodyDesc::new()
        .translation(Vector3::new(x, y, z))
        .build();

    return rb;
}

fn process_polyline(polyline: Polyline, position: Coordinates) -> RigidBody<f32>{
    // Coordonnées et rayon de la Polyline
    let x = position.x;
    let y = position.y;
    let z = position.z;
    let points = polyline.points;
    let indices = polyline.indices;

    // Création d'une Polyline
    let Polyline = ShapeHandle::new(shape::Polyline::new(points, indices));

    // Création du RigidBody de la Polyline
    let rb = RigidBodyDesc::new()
        .translation(Vector3::new(x, y, z))
        .build();

    return rb;
}

fn process_segment(segment: Segment, position: Coordinates) -> RigidBody<f32>{
    // Coordonnées et rayon du Segment
    let x = position.x;
    let y = position.y;
    let z = position.z;
    let a = segment.a;
    let b = segment.b; 

    // Création d'un Segment
    let Segment = ShapeHandle::new(shape::Segment::new(a, b));

    // Création du RigidBody du Segment
    let rb = RigidBodyDesc::new()
        .translation(Vector3::new(x, y, z))
        .build();

    return rb;
}

fn process_trimesh(trimesh: TriMesh, position: Coordinates) -> RigidBody<f32>{
    // Coordonnées et rayon de la TriMesh
    let x = position.x;
    let y = position.y;
    let z = position.z;
    let points = trimesh.points;
    let indices = trimesh.indices;
    let uvs = trimesh.uvs;

    // Création d'une TriMesh
    let TriMesh = ShapeHandle::new(shape::TriMesh::new(points, indices, uvs));

    // Création du RigidBody de la TriMesh
    let rb = RigidBodyDesc::new()
        .translation(Vector3::new(x, y, z))
        .build();

    return rb;
}

fn process_triangle(triangle: Triangle, position: Coordinates) -> RigidBody<f32>{
    // Coordonnées et rayon du Triangle
    let x = position.x;
    let y = position.y;
    let z = position.z;
    let a = triangle.a;
    let b = triangle.b;
    let c = triangle.c;

    // Création d'un Triangle
    let Triangle = ShapeHandle::new(shape::Triangle::new(a, b, c));

    // Création du RigidBody du Triangle
    let rb = RigidBodyDesc::new()
        .translation(Vector3::new(x, y, z))
        .build();

    return rb;
}

// Print shit atm
fn process_mesh(event: MeshType, objet: &Objet) -> RigidBody<f32> {
    match event {
        MeshType::Ball(Ball) => return process_ball(Ball, objet.position),
        MeshType::Capsule(Capsule) => return process_capsule(Capsule, objet.position),
        MeshType::Compound(Compound) => return process_compound(Compound, objet.position),
        MeshType::ConvexHull(ConvexHull) => return process_convexhull(ConvexHull, objet.position),
        MeshType::Cuboid(Cuboid) => return process_cuboid(Cuboid, objet.position),
        MeshType::HeightField(HeightField) => return process_heightfield(HeightField, objet.position),
        MeshType::Plane(Plane) => return process_plane(Plane, objet.position),
        MeshType::Polyline(Polyline) => return process_polyline(Polyline, objet.position),
        MeshType::Segment(Segment) => return process_segment(Segment, objet.position),
        MeshType::TriMesh(TriMesh) => return process_trimesh(TriMesh, objet.position),
        MeshType::Triangle(Triangle) => return process_triangle(Triangle, objet.position),
    }
}

// Fait rien pour le moment
fn main() {
    // MechanicalWorld with a gravity vector
    let mut mechanical_world = DefaultMechanicalWorld::new(Vector3::new(0.0, -9.81, 0.0));
    let mut geometrical_world = DefaultGeometricalWorld::<f32>::new();

    let mut bodies = DefaultBodySet::new();
    let mut colliders = DefaultColliderSet::<f32>::new();
    let mut joint_constraints = DefaultJointConstraintSet::<f32>::new();
    let mut force_generators = DefaultForceGeneratorSet::<f32>::new();


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
    for objet in &obj_set.tab{
        let rb = process_mesh(objet.mesh, objet); 
        // Ajout du RigidBody au set de RigidBody
        let rb_handle = bodies.insert(rb);
    }

    



    //let itsaball  = MeshType::Ball(Ball{radius: 1 as f32});

    // Il faut aussi passer un objet
    //process_mesh(itsaball );
}
