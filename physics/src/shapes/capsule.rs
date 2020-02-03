use ncollide3d::shape;
use ncollide3d::shape::ShapeHandle;



// We implement the Clone trait to the structure
#[derive(Clone)]
pub struct Capsule
{
    pub half_height: f32,
    pub radius: f32
}

impl Capsule{
    /// Creates and returns a RigidBody corresponding to the 'Capsule' type
    pub fn process_capsule(capsule: Capsule) -> ShapeHandle<f32>{
        // Half-height and radius of the Capsule
        let half_height = capsule.half_height;
        let radius = capsule.radius;

        // Creation of a Capsule we'll need later to make a RigidBody and Collider
        let caps = ShapeHandle::new(shape::Capsule::new(half_height, radius));

        return caps; 
    }
}