use crate::misc::Coordinates;
use nphysics3d::object::{RigidBodyDesc, RigidBody};
use ncollide3d::shape;
use ncollide3d::shape::ShapeHandle;
use ncollide3d::math::Isometry;
use na::Vector3;



// We implement the Clone trait to the structure
#[derive(Clone)]
pub struct Compound
{
    pub shapes: Vec<(Isometry<f32>, ShapeHandle<f32>)>
}

impl Compound{
    /// Creates and returns a RigidBody corresponding to the 'Compound' type
    pub fn process_compound(compound: Compound, position: Coordinates) -> (RigidBody<f32>, ShapeHandle<f32>){
        // Coordinates and shapes of the Compound
        let x = position.x;
        let y = position.y;
        let z = position.z;
        let shapes = compound.shapes; 

        // Creation of a Compound we'll need later to make a Collider
        let comp = ShapeHandle::new(shape::Compound::new(shapes));

        // Creation of the Compound's RigidBody
        let rb = RigidBodyDesc::new()
            .translation(Vector3::new(x, y, z))
            .build();

        return (rb, comp);
    }
}