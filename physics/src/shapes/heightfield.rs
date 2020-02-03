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
    pub fn process_heightfield(heightfield: HeightField) -> ShapeHandle<f32>{
        // Height and scale of the HeightField
        let heights = heightfield.heights;
        let scale = heightfield.scale;

        // Creation of a HeightField we'll need later to make a RigidBody and Collider
        let heightf = ShapeHandle::new(shape::HeightField::new(heights, scale));

        return heightf;
    }
}