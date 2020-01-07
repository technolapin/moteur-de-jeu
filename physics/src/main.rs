extern crate nalgebra as na;

////use std::env; // Pour récupérer les arguments de la commande
use std::vec::Vec;

use na::Vector3;
use na::base::{Unit, DMatrix};
use na::geometry::{Point2, Point3};

use nphysics3d::object::{DefaultBodySet, DefaultColliderSet, RigidBodyDesc, RigidBody, BodyPartHandle, ColliderDesc};
use nphysics3d::force_generator::DefaultForceGeneratorSet;
use nphysics3d::joint::DefaultJointConstraintSet;
use nphysics3d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};

use ncollide3d::shape::ShapeHandle;
use ncollide3d::shape;
use ncollide3d::math::{Point, Isometry};



// We implement the Clone trait to the structure
#[derive(Clone)]
struct Ball
{
    radius: f32
}

// We implement the Clone trait to the structure
#[derive(Clone)]
struct Capsule
{
    half_height: f32,
    radius: f32
}

// We implement the Clone trait to the structure
#[derive(Clone)]
struct Compound
{
    shapes: Vec<(Isometry<f32>, ShapeHandle<f32>)>
}

// We implement the Clone trait to the structure
#[derive(Clone)]
struct ConvexHull
{
    points: Vec<Point<f32>>
}

// We implement the Clone trait to the structure
#[derive(Clone)]
struct Cuboid
{
    vector: Vector3<f32>
}

// We implement the Clone trait to the structure
#[derive(Clone)]
struct HeightField
{
    heights: DMatrix<f32>,
    scale: Vector3<f32>
}

// We implement the Clone trait to the structure
#[derive(Clone)]
struct Plane
{
    normal: Unit<Vector3<f32>>
}

// We implement the Clone trait to the structure
#[derive(Clone)]
struct Polyline
{
    points: Vec<Point<f32>>,
    indices: Option<Vec<Point2<usize>>>
}

// We implement the Clone trait to the structure
#[derive(Clone)]
struct Segment
{
    a: Point<f32>,
    b: Point<f32>
}

// We implement the Clone trait to the structure
#[derive(Clone)]
struct TriMesh
{
    points: Vec<Point<f32>>,
    indices: Vec<Point3<usize>>,
    uvs: Option<Vec<Point2<f32>>>
}

// We implement the Clone trait to the structure
#[derive(Clone)]
struct Triangle
{
    a: Point<f32>,
    b: Point<f32>,
    c: Point<f32>
}

// We create an enum with all the shapes of mesh we can create with ncollide3d
#[derive(Clone)]
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

// We implement the Copy trait to the structure
#[derive(Copy, Clone)]
struct Coordinates{
    x: f32,
    y: f32,
    z: f32
}

struct Object {
    position: Coordinates,
    speed: f32,
    mass: f32,
    can_move: bool,
    mesh: MeshType,
    density: f32
}

struct ObjSet{
    tab: Vec<Object>,
    length: f32
}



// ### À compléter ###
fn build_object_table() -> Vec<Object>{
    // Fo mèt du cod
    let tab = Vec::new();
    return tab;
}

fn build_obj_set(tab: Vec<Object>, length: f32) -> ObjSet{
    ObjSet {
        tab,
        length
    }
}



/* 
 This function will be called by process_mesh and returns a RigidBody
 corresponding to the Ball
 */
fn process_ball(ball: Ball, position: Coordinates) -> (RigidBody<f32>, ShapeHandle<f32>){
    // Coordinates and radius of the Ball
    let x = position.x;
    let y = position.y;
    let z = position.z;
    let radius = ball.radius;

    // Creation of a Ball
    let Ball = ShapeHandle::new(shape::Ball::new(radius));

    // Creation of the Ball's RigidBody
    let rb = RigidBodyDesc::new()
        .translation(Vector3::new(x, y, z))
        .build();

    return (rb, Ball);
}

/* 
 This function will be called by process_mesh and returns a RigidBody
 corresponding to the Capsule
 */
fn process_capsule(capsule: Capsule, position: Coordinates) -> (RigidBody<f32>, ShapeHandle<f32>){
    // Coordinates, half-height and radius of the Capsule
    let x = position.x;
    let y = position.y;
    let z = position.z;
    let half_height = capsule.half_height;
    let radius = capsule.radius;

    // Creation of a Capsule
    let Capsule = ShapeHandle::new(shape::Capsule::new(half_height, radius));

    // Creation of the Capsule's RigidBody
    let rb = RigidBodyDesc::new()
        .translation(Vector3::new(x, y, z))
        .build();

    return (rb, Capsule);
}

/* 
 This function will be called by process_mesh and returns a RigidBody
 corresponding to the Compound
 */
fn process_compound(compound: Compound, position: Coordinates) -> (RigidBody<f32>, ShapeHandle<f32>){
    // Coordinates and shapes of the Compound
    let x = position.x;
    let y = position.y;
    let z = position.z;
    let shapes = compound.shapes; 

    // Creation of a Compound
    let Compound = ShapeHandle::new(shape::Compound::new(shapes));

    // Creation of the Compound's RigidBody
    let rb = RigidBodyDesc::new()
        .translation(Vector3::new(x, y, z))
        .build();

    return (rb, Compound);
}

/* 
 This function will be called by process_mesh and returns a RigidBody
 corresponding to the ConvexHull
 */
fn process_convexhull(convexhull: ConvexHull, position: Coordinates) -> (RigidBody<f32>, ShapeHandle<f32>){
    // Coordonnées and points of the ConvexHull
    let x = position.x;
    let y = position.y;
    let z = position.z;
    let points = convexhull.points;

    // Creation of a ConvexHull
    let ConvexHull = ShapeHandle::new(shape::ConvexHull::try_from_points(&points).unwrap());

    // Creation of the ConvexHull's RigidBody
    let rb = RigidBodyDesc::new()
        .translation(Vector3::new(x, y, z))
        .build();

    return (rb, ConvexHull);
}

/* 
 This function will be called by process_mesh and returns a RigidBody
 corresponding to the Cuboid
 */
fn process_cuboid(cuboid: Cuboid, position: Coordinates) -> (RigidBody<f32>, ShapeHandle<f32>){
    // Coordonnées and vector of the Cuboid
    let x = position.x;
    let y = position.y;
    let z = position.z;
    let vector = cuboid.vector;

    // Creation of a Cuboid
    let Cuboid = ShapeHandle::new(shape::Cuboid::new(vector));

    // Creation of the Cuboid's RigidBody
    let rb = RigidBodyDesc::new()
        .translation(Vector3::new(x, y, z))
        .build();

    return (rb, Cuboid);
}

/* 
 This function will be called by process_mesh and returns a RigidBody
 corresponding to the HeightField
 */
fn process_heightfield(heightfield: HeightField, position: Coordinates) -> (RigidBody<f32>, ShapeHandle<f32>){
    // Coordinates, height and scale of the HeightField
    let x = position.x;
    let y = position.y;
    let z = position.z;
    let heights = heightfield.heights;
    let scale = heightfield.scale;

    // Creation of a HeightField
    let HeightField = ShapeHandle::new(shape::HeightField::new(heights, scale));

    // Creation of the HeightField's RigidBody
    let rb = RigidBodyDesc::new()
        .translation(Vector3::new(x, y, z))
        .build();

    return (rb, HeightField);
}

/* 
 This function will be called by process_mesh and returns a RigidBody
 corresponding to the Plane
 */
fn process_plane(plane: Plane, position: Coordinates) -> (RigidBody<f32>, ShapeHandle<f32>){
    // Coordinates and normal of the Plane
    let x = position.x;
    let y = position.y;
    let z = position.z;
    let normal = plane.normal;

    // Creation of a Plane
    let Plane = ShapeHandle::new(shape::Plane::new(normal));

    // Creation of the Plane's RigidBody
    let rb = RigidBodyDesc::new()
        .translation(Vector3::new(x, y, z))
        .build();

    return (rb, Plane);
}

/* 
 This function will be called by process_mesh and returns a RigidBody
 corresponding to the Polyline
 */
fn process_polyline(polyline: Polyline, position: Coordinates) -> (RigidBody<f32>, ShapeHandle<f32>){
    // Coordinates, points and indices of the Polyline
    let x = position.x;
    let y = position.y;
    let z = position.z;
    let points = polyline.points;
    let indices = polyline.indices;

    // Creation of a Polyline
    let Polyline = ShapeHandle::new(shape::Polyline::new(points, indices));

    // Creation of the Polyline's RigidBody
    let rb = RigidBodyDesc::new()
        .translation(Vector3::new(x, y, z))
        .build();

    return (rb, Polyline);
}

/* 
 This function will be called by process_mesh and returns a RigidBody
 corresponding to the Segment
 */
fn process_segment(segment: Segment, position: Coordinates) -> (RigidBody<f32>, ShapeHandle<f32>){
    // Coordinates and points of the Segment
    let x = position.x;
    let y = position.y;
    let z = position.z;
    let a = segment.a;
    let b = segment.b; 

    // Creation of a Segment
    let Segment = ShapeHandle::new(shape::Segment::new(a, b));

    // Creation of the Segment's RigidBody
    let rb = RigidBodyDesc::new()
        .translation(Vector3::new(x, y, z))
        .build();

    return (rb, Segment);
}

/* 
 This function will be called by process_mesh and returns a RigidBody
 corresponding to the TriMesh
 */
fn process_trimesh(trimesh: TriMesh, position: Coordinates) -> (RigidBody<f32>, ShapeHandle<f32>){
    // Coordinates, points, indices and uvs of the TriMesh
    let x = position.x;
    let y = position.y;
    let z = position.z;
    let points = trimesh.points;
    let indices = trimesh.indices;
    let uvs = trimesh.uvs;

    // Creation of a TriMesh
    let TriMesh = ShapeHandle::new(shape::TriMesh::new(points, indices, uvs));

    // Creation of the TriMesh's RigidBody
    let rb = RigidBodyDesc::new()
        .translation(Vector3::new(x, y, z))
        .build();

    return (rb, TriMesh);
}

/* 
 This function will be called by process_mesh and returns a RigidBody
 corresponding to the Triangle
 */
fn process_triangle(triangle: Triangle, position: Coordinates) -> (RigidBody<f32>, ShapeHandle<f32>){
    // Coordinates and points of the Triangle
    let x = position.x;
    let y = position.y;
    let z = position.z;
    let a = triangle.a;
    let b = triangle.b;
    let c = triangle.c;

    // Creation of a Triangle
    let Triangle = ShapeHandle::new(shape::Triangle::new(a, b, c));

    // Creation of the Triangle's RigidBody
    let rb = RigidBodyDesc::new()
        .translation(Vector3::new(x, y, z))
        .build();

    return (rb, Triangle);
}

// Print shit at the moment
fn process_mesh(event: MeshType, objet: &Object) -> (RigidBody<f32>, ShapeHandle<f32>) {
    match event {
        MeshType::Ball(ball) => return process_ball(ball, objet.position),
        MeshType::Capsule(capsule) => return process_capsule(capsule, objet.position),
        MeshType::Compound(compound) => return process_compound(compound, objet.position),
        MeshType::ConvexHull(convexhull) => return process_convexhull(convexhull, objet.position),
        MeshType::Cuboid(cuboid) => return process_cuboid(cuboid, objet.position),
        MeshType::HeightField(heightfield) => return process_heightfield(heightfield, objet.position),
        MeshType::Plane(plane) => return process_plane(plane, objet.position),
        MeshType::Polyline(polyline) => return process_polyline(polyline, objet.position),
        MeshType::Segment(segment) => return process_segment(segment, objet.position),
        MeshType::TriMesh(trimesh) => return process_trimesh(trimesh, objet.position),
        MeshType::Triangle(triangle) => return process_triangle(triangle, objet.position),
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

    let tab = build_object_table();
    let length = 0 as f32; // Demander à Clément comment on a la taille d'un vec
    let obj_set = build_obj_set(tab, length); 



    // For every object in obj_set
    for object in &obj_set.tab{
        let tuple = process_mesh(object.mesh.clone(), object);
        let rb = tuple.0; 
        // We add the RigidBody to the RigidBodySet
        let rb_handle = bodies.insert(rb);
        let collider = ColliderDesc::new(tuple.1)
        .density(object.density)
        .build(BodyPartHandle(rb_handle, 0));
        
        colliders.insert(collider);
    }

    loop {
        // The universe is now running/ticking
        mechanical_world.step(
            &mut geometrical_world,
            &mut bodies,
            &mut colliders,
            &mut joint_constraints,
            &mut force_generators
        );
    }
}
