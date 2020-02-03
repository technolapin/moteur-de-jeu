use crate::misc::Coordinates;
use nphysics3d::object::{RigidBodyDesc, RigidBody};
use ncollide3d::shape;
use ncollide3d::shape::ShapeHandle;
use na::Vector3;
use na::base::DMatrix;



// We implement the Clone trait to the structure
#[derive(Clone)]
pub struct HeightField
{
    pub heights: DMatrix<f32>,
    pub scale: Vector3<f32>
}

impl HeightField{
    /// Creates and returns a RigidBody corresponding to the 'HeightField' type
    pub fn process_heightfield(heightfield: HeightField, position: Coordinates) -> (RigidBody<f32>, ShapeHandle<f32>){
        // Coordinates, height and scale of the HeightField
        let x = position.x;
        let y = position.y;
        let z = position.z;
        let heights = heightfield.heights;
        let scale = heightfield.scale;

        // Creation of a HeightField we'll need later to make a Collider
        let heightf = ShapeHandle::new(shape::HeightField::new(heights, scale));

        // Creation of the HeightField's RigidBody
        let rb = RigidBodyDesc::new()
            .translation(Vector3::new(x, y, z))
            .build();

        return (rb, heightf);
    }
}