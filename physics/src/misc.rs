use crate::shapes::*;

use nphysics3d::object::{DefaultBodySet, DefaultColliderSet, RigidBodyDesc, BodyPartHandle, ColliderDesc, BodyStatus};
use nphysics3d::material::{MaterialHandle, BasicMaterial};

use ncollide3d::shape::ShapeHandle;

use na::Vector3;
use na::Matrix3;
use na::geometry::Point3;

use nphysics3d::algebra::Velocity3;


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

/// Data needed to create a 'RigidBody'
pub struct RbData{
    pub translation: Vector3<f32>, // The rigid body translation - Default: zero vector
    pub rotation: Vector3<f32>, // The rigid body rotation - Default: no rotation
    pub gravity_enabled: bool, // Whether or not this rigid body is affected by gravity - Default: true
    pub bodystatus: BodyStatus, // The status of this rigid body. It can be Disabled, Kinematic or Dynamic - Default: BodyStatus::Dynamic
    pub linear_velocity: Vector3<f32>, // The velocity of this body - Default: zero velocity
    pub angular_velocity: Vector3<f32>, // The velocity of this body - Default: zero velocity
    pub linear_damping: f32, // The linear damping applied to this rigid body velocity to slow it down automatically - Default: zero (no damping at all)
    pub angular_damping: f32, // The angular damping applied to this rigid body velocity to slow down its rotation automatically - Default: zero (no damping at all)
    pub max_linear_velocity: f32, // The maximum linear velocity this rigid body can reach - Default: f32::max_value() or f64::max_value() (no limit)
    pub max_angular_velocity: f32, // The maximum angular velocity this rigid body can reach - Default: f32::max_value() or f64::max_value() (no limit)
    pub angular_inertia: f32, // The angular inertia tensor of this rigid body, expressed on its local-space - Default: the zero matrix
    pub mass: f32, // The rigid body mass - Default: 0.0
    pub local_center_of_mass: Point3<f32>, // The center of mass of this rigid body expressed in its local-space - Default: the origin
    pub sleep_threshold: f32, // The threshold for putting this rigid body to sleep - Default: Some(ActivationStatus::default_threshold())
    pub kinematic_translations: Vector3<bool>, // The translations that will be locked for this rigid body - Default: nothing is locked (false everywhere)
    pub kinematic_rotations: Vector3<bool>, // The rotations that will be locked for this rigid body - Default: nothing is locked (false everywhere)
    pub user_data: usize, // Arbitrary user-defined data associated to the rigid body to be built - Default: no associated data
    pub enable_linear_motion_interpolation: bool // Whether this rigid body motion should be interpolated linearly during CCD resolution - Default: false (which implies non-linear interpolation)
}

/// Data needed to create a 'Collider'
pub struct ColData{
    pub translation: Vector3<f32>, // The collider translation wrt. the body part it is attached to - Default: zero vector
    pub rotation: Vector3<f32>, // The collider rotation wrt. the body part it is attached to - Default: no rotation
    pub density: f32, // If non-zero the collider's mass and angular inertia will be added to the inertial properties of the body part it is attached to - Default: 0.0
    pub restitution: f32, // Restitution of the collider - Default 0.0
    pub friction: f32, // Friction of the collider - Default: 0.5
    pub margin: f32, // Solid margin surrounding the collider (should always be non-zero) - Default: 0.01
    pub linear_prediction: f32, // The distance tolerance for predictive contacts generation - Default: 0.002
    pub angular_prediction: f32, // The angular tolerance for predictive contacts generation - Default: PI / 180.0 * 5.0
    pub sensor: bool, // Whether this collider is a sensor, i.e., generate only proximity events - Default: false
    pub user_data: usize // Arbitrary user-defined data associated to the rigid body to be built - Default: no associated data
}

/// An object with different features
pub struct Object {
    pub position: Coordinates,
    pub shape: ShapeType,
    pub rbdata: RbData,
    pub coldata: ColData
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



/// Creates and returns a RigidBody corresponding to the object's shape
pub fn process_shape(event: ShapeType) -> ShapeHandle<f32>{
    match event {
        ShapeType::Ball(ball) => return Ball::process_ball(ball),
        ShapeType::Capsule(capsule) => return Capsule::process_capsule(capsule),
        ShapeType::Compound(compound) => return Compound::process_compound(compound),
        ShapeType::ConvexHull(convexhull) => return ConvexHull::process_convexhull(convexhull),
        ShapeType::Cuboid(cuboid) => return Cuboid::process_cuboid(cuboid),
        ShapeType::HeightField(heightfield) => return HeightField::process_heightfield(heightfield),
        ShapeType::Plane(plane) => return Plane::process_plane(plane),
        ShapeType::Polyline(polyline) => return Polyline::process_polyline(polyline),
        ShapeType::Segment(segment) => return Segment::process_segment(segment),
        ShapeType::TriMesh(trimesh) => return TriMesh::process_trimesh(trimesh),
        ShapeType::Triangle(triangle) => return Triangle::process_triangle(triangle),
    }
}



/// Creates the RigidBody and Collider of every object in the ObjSet given in parameter, store them in a ColliderSet and a Vector<Collider> and returns it
pub fn build_rb_col(obj_set: ObjSet) -> (DefaultBodySet<f32>, DefaultColliderSet<f32>, Vec<generational_arena::Index>){

    // Where we store all the RigidBody object
    let mut bodies = DefaultBodySet::new();

    // Where we store all the Collider object
    let mut colliders = DefaultColliderSet::<f32>::new();

    // Where we store the handle of every collider so we can get their position and material later (used for testing only at the moment)
    let mut coll_tab = Vec::new();

    // For every object in obj_set
    for object in &obj_set.tab{

        let shape = process_shape(object.shape.clone());
        
        // We create the RigidBody relative to the field rbdata of 'object'
        let mut rb = RigidBodyDesc::new()
        .translation(object.rbdata.translation)
        .rotation(object.rbdata.rotation)
        .gravity_enabled(object.rbdata.gravity_enabled)
        .status(object.rbdata.bodystatus)
        .velocity(Velocity3::new(object.rbdata.linear_velocity, object.rbdata.angular_velocity))
        .linear_damping(object.rbdata.linear_damping)
        .angular_damping(object.rbdata.angular_damping)
        .max_linear_velocity(object.rbdata.max_linear_velocity)
        .max_angular_velocity(object.rbdata.max_angular_velocity)
        .angular_inertia(Matrix3::from_diagonal_element(object.rbdata.angular_inertia))
        .mass(object.rbdata.mass)
        .local_center_of_mass(object.rbdata.local_center_of_mass)
        .sleep_threshold(Some(object.rbdata.sleep_threshold))
        .kinematic_translations(object.rbdata.kinematic_translations)
        .kinematic_rotations(object.rbdata.kinematic_rotations)
        .user_data(object.rbdata.user_data)
        .build(); // Build the rigid-body

        rb.enable_linear_motion_interpolation(object.rbdata.enable_linear_motion_interpolation);

        // We add the RigidBody to the RigidBodySet
        let rb_handle = bodies.insert(rb);


        // We create the Collider relative to the field coldata of 'object'
        let collider = ColliderDesc::new(shape)
        .translation(object.coldata.translation)
        .rotation(object.rbdata.rotation)
        .density(object.coldata.density)
        .material(MaterialHandle::new(BasicMaterial::new(object.coldata.restitution, object.coldata.friction)))
        .margin(object.coldata.margin)
        .linear_prediction(object.coldata.linear_prediction)
        .angular_prediction(object.coldata.angular_prediction)
        .sensor(object.coldata.sensor)
        .user_data(object.coldata.user_data)
        .build(BodyPartHandle(rb_handle, 0)); // Build the collider into the world
        
        // We add the Collider to the set of colliders
        let coll_handle = colliders.insert(collider);

        // Wa add the handle to the coll_tab
        coll_tab.push(coll_handle);
    }
    return (bodies, colliders, coll_tab);
}