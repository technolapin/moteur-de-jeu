use crate::shapes::*;
use nphysics3d::object::{DefaultBodySet, DefaultColliderSet, RigidBody, BodyPartHandle, ColliderDesc, BodyStatus};
use nphysics3d::material::{MaterialHandle, BasicMaterial};
use ncollide3d::shape::ShapeHandle;



// We implement the Copy trait to the structure
#[derive(Copy, Clone)]
pub struct Coordinates{
    pub x: f32,
    pub y: f32,
    pub z: f32
}

// We implement the Clone trait to the structure
#[derive(Clone)]
/// Different types of shape an object can take
pub enum ShapeType {
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

/// An object with different features
pub struct Object {
    pub position: Coordinates,
    //speed: f32,
    //mass: f32,
    // Peut servir pour le BodyStatus
    //can_move: bool,
    pub shape: ShapeType,
    pub density: f32,
    pub restitution: f32,
    pub friction: f32,
    pub margin: f32,
    pub linear_prediction: f32,
    pub angular_prediction: f32,
    pub sensor: bool,
    pub user_data: usize,
}

/// A set that contains many 'Object'
pub struct ObjSet{
    pub tab: Vec<Object>
}

impl ObjSet{
    /// Creates an empty Vec that can store 'Object'
    pub fn build_object_table() -> Vec<Object>{
        let tab = Vec::new();
        return tab;
    }

    /// Creates an 'ObjSet' with the tab given as parameter
    pub fn build_obj_set(tab: Vec<Object>,) -> ObjSet{
        ObjSet {
            tab
        }
    }
}



/// Creates and returns a Tuple containing: 1. A RigidBody corresponding to the object's shape; 2. The position of the RigidBody
pub fn process_shape(event: ShapeType, object: &Object) -> (RigidBody<f32>, ShapeHandle<f32>) {
    match event {
        ShapeType::Ball(ball) => return Ball::process_ball(ball, object.position),
        ShapeType::Capsule(capsule) => return Capsule::process_capsule(capsule, object.position),
        ShapeType::Compound(compound) => return Compound::process_compound(compound, object.position),
        ShapeType::ConvexHull(convexhull) => return ConvexHull::process_convexhull(convexhull, object.position),
        ShapeType::Cuboid(cuboid) => return Cuboid::process_cuboid(cuboid, object.position),
        ShapeType::HeightField(heightfield) => return HeightField::process_heightfield(heightfield, object.position),
        ShapeType::Plane(plane) => return Plane::process_plane(plane, object.position),
        ShapeType::Polyline(polyline) => return Polyline::process_polyline(polyline, object.position),
        ShapeType::Segment(segment) => return Segment::process_segment(segment, object.position),
        ShapeType::TriMesh(trimesh) => return TriMesh::process_trimesh(trimesh, object.position),
        ShapeType::Triangle(triangle) => return Triangle::process_triangle(triangle, object.position),
    }
}

/// Creates the Collider of every object in the ObjSet given in parameter, store them in a ColliderSet and a Vector<Collider> and returns it
pub fn build_colliders(obj_set: ObjSet) -> (DefaultBodySet<f32>, DefaultColliderSet<f32>, Vec<generational_arena::Index>){

    // Where we store all the RigidBody object
    let mut bodies = DefaultBodySet::new();

    // Where we store all the Collider object
    let mut colliders = DefaultColliderSet::<f32>::new();

    // Where we store the handle of every collider so we can get their position and material later (used for testing only at the moment)
    let mut coll_tab = Vec::new();

    // For every object in obj_set
    for object in &obj_set.tab{

        let tuple = process_shape(object.shape.clone(), object);

        // The RigidBody associated to the object is at position 0 of the tuple
        let rb = tuple.0; 

        // We add the RigidBody to the RigidBodySet
        let rb_handle = bodies.insert(rb);

        // ### FOR TESTING PURPOSE ONLY ###
        // We only have the ground at y = 0 at the moment
        if object.position.y == 0 as f32 {

            // We get the ground's RigidBody 
            let rb_ground = bodies.get_mut(rb_handle).expect("Rigid-body not found.");

            /*
             "BodyStatus::Kinematic" indicates the body velocity must not be altered by the physics engine
             It can be set to Disabled to indicate that the body should be completely ignored by the physics engine - Default: Dynamic 
            */
            rb_ground.set_status(BodyStatus::Kinematic);
        }
        // ### END OF TESTING ###

        // We create the Collider relative to the fields of 'object'
        // The shape (Ball, Triangle, ...) associated to the object is at position 1 of the tuple
        let collider = ColliderDesc::new(tuple.1)
        /*
        // The collider translation wrt. the body part it is attached to - Default: zero vector
        .translation(Vector3::y() * 5.0)
        // The collider rotation wrt. the body part it is attached to - Default: no rotation
        .rotation(Vector3::y() * 5.0)
        */
        // If non-zero the collider's mass and angular inertia will be added to the inertial properties of the body part it is attached to - Default: 0.0
        .density(object.density)
        // Allows to define if the object bounces for example (restitution, friction) - Default: (0.0, 0.5)
        .material(MaterialHandle::new(BasicMaterial::new(object.restitution, object.friction)))
        // Solid margin surrounding the collider (should always be non-zero) - Default: 0.01
        .margin(object.margin)
        // The distance tolerance for predictive contacts generation - Default: 0.002
        .linear_prediction(object.linear_prediction)
        // The angular tolerance for predictive contacts generation - Default: PI / 180.0 * 5.0
        .angular_prediction(object.angular_prediction)
        // Whether this collider is a sensor, i.e., generate only proximity events - Default: false
        .sensor(object.sensor)
        // Arbitrary user-defined data associated to the rigid body to be built - Default: no associated data
        .user_data(object.user_data)
        // Build the collider into the world
        .build(BodyPartHandle(rb_handle, 0));
        
        // We add the Collider to the set of colliders
        let coll_handle = colliders.insert(collider);

        // Wa add the handle to the coll_tab
        coll_tab.push(coll_handle);
    }
    return (bodies, colliders, coll_tab);
}