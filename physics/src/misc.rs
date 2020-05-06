extern crate nalgebra as na;

use crate::shapes::*;

use nphysics3d::object::{DefaultBodySet, DefaultColliderSet, RigidBodyDesc, BodyPartHandle, ColliderDesc, BodyStatus, ActivationStatus};
use nphysics3d::material::{MaterialHandle, BasicMaterial};

use ncollide3d::shape::ShapeHandle;

use na::Vector3;
use na::Matrix3;
use na::geometry::Point3;

use nphysics3d::algebra::Velocity3;

use graphics::Scene;
use std::f32::consts::PI;
use std::f32::INFINITY;

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
    pub bodystatus: BodyStatus, // The status of this rigid body. It can be Disabled, Static, Kinematic or Dynamic - Default: BodyStatus::Dynamic
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

impl RbData{
    pub fn new(
        translation: Vector3<f32>, 
        rotation: Vector3<f32>, 
        gravity_enabled: bool, 
        bodystatus: BodyStatus, 
        linear_velocity: Vector3<f32>, 
        angular_velocity: Vector3<f32>, 
        linear_damping: f32, 
        angular_damping: f32, 
        max_linear_velocity: f32, 
        max_angular_velocity: f32, 
        angular_inertia: f32, 
        mass: f32, 
        local_center_of_mass: Point3<f32>, 
        sleep_threshold: f32, 
        kinematic_translations: Vector3<bool>, 
        kinematic_rotations: Vector3<bool>, 
        user_data: usize, 
        enable_linear_motion_interpolation: bool) -> RbData{

        RbData{
            translation: translation, 
            rotation: rotation, 
            gravity_enabled: gravity_enabled, 
            bodystatus: bodystatus, 
            linear_velocity: linear_velocity, 
            angular_velocity: angular_velocity, 
            linear_damping: linear_damping, 
            angular_damping: angular_damping, 
            max_linear_velocity: max_linear_velocity, 
            max_angular_velocity: max_angular_velocity, 
            angular_inertia: angular_inertia, 
            mass: mass, 
            local_center_of_mass: local_center_of_mass, 
            sleep_threshold: sleep_threshold, 
            kinematic_translations: kinematic_translations, 
            kinematic_rotations: kinematic_rotations, 
            user_data: user_data, 
            enable_linear_motion_interpolation: enable_linear_motion_interpolation 
        }
    }
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

impl ColData{
    pub fn new(
        translation: Vector3<f32>, 
        rotation: Vector3<f32>,
        density: f32,
        restitution: f32,
        friction: f32,
        margin: f32,
        linear_prediction: f32,
        angular_prediction: f32,
        sensor: bool,
        user_data: usize) -> ColData{

        ColData{
            translation: translation, 
            rotation: rotation,
            density: density,
            restitution: restitution,
            friction: friction,
            margin: margin,
            linear_prediction: linear_prediction,
            angular_prediction: angular_prediction,
            sensor: sensor,
            user_data: user_data
        }
    }
}

/// An object with different features
pub struct Object {
    pub shape: ShapeType,
    pub rbdata: RbData,
    pub coldata: ColData
}

impl Object{
    pub fn new(shape: ShapeType, rbdata: RbData, coldata: ColData) -> Object{
        Object{
            shape: shape, 
            rbdata: rbdata, 
            coldata: coldata
        }
    }
}

/// A set that contains many 'Object'
pub struct ObjSet{
    pub tab: Vec<Object>
}

impl ObjSet{
    /// Creates an 'ObjSet'
    pub fn new() -> ObjSet{

        ObjSet {
            tab: Vec::new()
        }
    }

    /// Puts tha 'Object' given in parameter in the tab of the 'ObjSet'
    pub fn push(&mut self, obj: Object){

        &mut self.tab.push(obj);
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
        .rotation(object.coldata.rotation)
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



pub fn make_objects(scene: &Scene) -> ObjSet{
    let mut obj_set = ObjSet::new();

    for object in scene.objects.iter() {
        for similarity in object.1.iter() {
            let trs = similarity.deconstruct();
            let translation = trs.0;
            let rotation = trs.1;
            let scale = trs.2;
            let grav;
            let shape;
            let mut stat = BodyStatus::Static;

            // ### A CHANGER !!! ###
            // On ne fait que des Cuboid pour le moment
            // Il faut adapter en choisissant la bonne forme ou en ne faisant que des TriMesh (voir physics/src/shapes/trimesh.rs)
            if translation[0] == 0. && translation[1] == 0. && translation[2] == 0.{
                grav = false;
                shape = ShapeType::Cuboid(Cuboid::new(Vector3::new(20.,0.1,20.)));
            }
            else{
                grav = true;
                shape = ShapeType::Cuboid(Cuboid::new(Vector3::new(scale,scale,scale)));
                stat = BodyStatus::Dynamic;
            }
            // ### A CHANGER !!! ###

            let rb_data = RbData::new(
                translation,                            // translation
                rotation,                               // rotation
                grav,                                   // gravity_enabled
                stat,                                   // bodystatus
                Vector3::new(0.0, 0.0, 0.0),            // linear_velocity
                Vector3::new(0.0, 0.0, 0.0),            // angular_velocity
                0.0,                                    // linear_damping
                0.0,                                    // angular_damping
                INFINITY,                               // max_linear_velocity
                INFINITY,                               // max_angular_velocity
                0.0,                                    // angular_inertia
                2000.0,                                 // mass
                Point3::new(0.0, 0.0, 0.0),             // local_center_of_mass
                ActivationStatus::default_threshold(),  // sleep_threshold
                Vector3::new(false, false, false),      // kinematic_translations
                Vector3::new(false, false, false),      // kinematic_rotations
                0,                                      // user_data
                true                                    // enable_linear_motion_interpolation
            );

            let col_data = ColData::new(
                Vector3::new(0.0, 0.0, 0.0),            // translation
                Vector3::new(0.0, 0.0, 0.0),            // rotation
                0.0,                                    // density
                0.5,                                    // restitution
                0.2,                                    // friction
                0.01,                                   // margin
                0.002,                                  // linear_prediction
                PI / 180.0 * 5.0,                       // angular_prediction
                false,                                  // sensor
                0                                       // user_data
            );

            let handle = Object::new(shape, rb_data, col_data); // CHANGER LE NOM DE 'Object' (optionnel mais préférable)
            obj_set.push(handle);
        }
    }
    return obj_set;
}




use ncollide3d::query::{Ray, RayCast};
use nalgebra::geometry::Isometry3;
use super::super::game_state;

// Partie a mettre dans la fonction game_logic de l'exemple
/*
if devices.key_continous(Key::F) {
    take_obj(game_state, devices);
}

if devices.key_continous(Key::E) {
    throw_obj(game_state, devices);
}
*/

/// Returns None if the ray given in parameter doesn't intersect with the object given in parameter. Returns the shape of the object otherwise.
pub fn do_intersect(object: Object, ray: Ray<f32>) -> Option<(Object, f32)> {

    // We get the shape of the object (of type ShapeType)
    let shape_type = object.shape;

    // We get the shape of the object (of type ncollide3d::shape::ShapeHandle) to apply toi_with_ray on it
    let shape = process_shape(shape_type);

    /* 
     Result of the intersection between the Ray and a TriMesh 
     0.0 means no intersection
     otherwise the value is the distance from the Ray origin at which the intersection is
     Ex: Ray origin = (0.0, 0.0, 0.0) and intersection at (0.0, 0.0, 5.0) -> intersection = 5.0
    */
    let intersection = shape.toi_with_ray(&Isometry3::identity(), &ray, false).unwrap(); 

    // S'il y a intersection, on retourne l'objet pour lequel il y a intersection
    if intersection != 0.0 {
        return Some((object, intersection));
    }
    else {
        return None;
    }
}

pub fn nearest_obj(game_state: &mut GameState) -> Object {

    // Position of the camera
    let camera_pos = game_state.scene.camera.position.to_point();

    // Direction the camera is looking at
    let camera_dir = game_state.scene.camera.orientation;

    // A Ray
    let ray = Ray::new(camera_pos, camera_dir);

    // A Vec containing every object the Ray intersects with
    let inters = Vec::new();

    // An f32 if there's an intersection between a certain object and the Ray, None if there's no intersection
    let result;

    for object in obj_set {
        result = do_intersect(object, ray);
        if !result.is_none() {
            inters.push(result);
        }
    }

    let d = INFINITY;
    let ob;

    for tuple in inters {
        if let Some((a,b)) = tuple {
            if b < d {
                d = b;
                ob = a;
            }
        }
    }
    return ob;
}