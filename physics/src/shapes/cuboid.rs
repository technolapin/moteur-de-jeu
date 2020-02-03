use crate::misc::Coordinates;
use nphysics3d::object::{RigidBodyDesc, RigidBody};
use ncollide3d::shape;
use ncollide3d::shape::ShapeHandle;
use na::Vector3;



// We implement the Clone trait to the structure
#[derive(Clone)]
pub struct Cuboid
{
    pub vector: Vector3<f32>
}

impl Cuboid{
    /// Creates and returns a RigidBody corresponding to the 'Cuboid' type
    pub fn process_cuboid(cuboid: Cuboid, position: Coordinates) -> (RigidBody<f32>, ShapeHandle<f32>){
        // Coordinates and vector of the Cuboid
        let x = position.x;
        let y = position.y;
        let z = position.z;
        let vector = cuboid.vector;

        // Creation of a Cuboid we'll need later to make a Collider
        let cub = ShapeHandle::new(shape::Cuboid::new(vector));

        // Creation of the Cuboid's RigidBody
        let rb = RigidBodyDesc::new()
            .translation(Vector3::new(x, y, z))
            .build();

        return (rb, cub);
    }
}