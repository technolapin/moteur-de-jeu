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
    pub fn process_plane(plane: Plane) -> ShapeHandle<f32>{
        // Normal of the Plane
        let normal = plane.normal;

        // Creation of a Plane we'll need later to make a RigidBody and Collider
        let pla = ShapeHandle::new(shape::Plane::new(normal));

        return pla;
    }
}