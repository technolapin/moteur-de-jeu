use crate::misc::Coordinates;
use nphysics3d::object::{RigidBodyDesc, RigidBody};
use ncollide3d::shape;
use ncollide3d::shape::ShapeHandle;
use na::Vector3;



// We implement the Clone trait to the structure
#[derive(Clone)]
pub struct Capsule
{
    pub half_height: f32,
    pub radius: f32
}

impl Capsule{
    /// Creates and returns a RigidBody corresponding to the 'Capsule' type
    pub fn process_capsule(capsule: Capsule, position: Coordinates) -> (RigidBody<f32>, ShapeHandle<f32>){
        // Coordinates, half-height and radius of the Capsule
        let x = position.x;
        let y = position.y;
        let z = position.z;
        let half_height = capsule.half_height;
        let radius = capsule.radius;

        // Creation of a Capsule we'll need later to make a Collider
        let caps = ShapeHandle::new(shape::Capsule::new(half_height, radius));

        // Creation of the Capsule's RigidBody
        let rb = RigidBodyDesc::new()
            .translation(Vector3::new(x, y, z))
            .build();

        return (rb, caps); 
    }
}