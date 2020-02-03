use crate::misc::Coordinates;
use nphysics3d::object::{RigidBodyDesc, RigidBody};
use ncollide3d::shape;
use ncollide3d::shape::ShapeHandle;
use ncollide3d::math::Point;
use na::Vector3;



// We implement the Clone trait to the structure
#[derive(Clone)]
pub struct ConvexHull
{
    pub points: Vec<Point<f32>>
}

impl ConvexHull{
    /// Creates and returns a RigidBody corresponding to the 'ConvexHull' type
    pub fn process_convexhull(convexhull: ConvexHull, position: Coordinates) -> (RigidBody<f32>, ShapeHandle<f32>){
        // Coordinates and points of the ConvexHull
        let x = position.x;
        let y = position.y;
        let z = position.z;
        let points = convexhull.points;

        // Creation of a ConvexHull we'll need later to make a Collider
        let convexh = ShapeHandle::new(shape::ConvexHull::try_from_points(&points).unwrap());

        // Creation of the ConvexHull's RigidBody
        let rb = RigidBodyDesc::new()
            .translation(Vector3::new(x, y, z))
            .build();

        return (rb, convexh);
    }
}