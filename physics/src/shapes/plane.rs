use crate::misc::Coordinates;
use nphysics3d::object::{RigidBodyDesc, RigidBody};
use ncollide3d::shape;
use ncollide3d::shape::ShapeHandle;
use na::Vector3;
use na::base::Unit;



// We implement the Clone trait to the structure
#[derive(Clone)]
pub struct Plane
{
    pub normal: Unit<Vector3<f32>>
}

impl Plane{
    /// Creates and returns a RigidBody corresponding to the 'Plane' type
    pub fn process_plane(plane: Plane, position: Coordinates) -> (RigidBody<f32>, ShapeHandle<f32>){
        // Coordinates and normal of the Plane
        let x = position.x;
        let y = position.y;
        let z = position.z;
        let normal = plane.normal;

        // Creation of a Plane we'll need later to make a Collider
        let pla = ShapeHandle::new(shape::Plane::new(normal));

        // Creation of the Plane's RigidBody
        let rb = RigidBodyDesc::new()
            .translation(Vector3::new(x, y, z))
            .build();

        return (rb, pla);
    }
}