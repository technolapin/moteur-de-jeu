use crate::misc::Coordinates;
use nphysics3d::object::{RigidBodyDesc, RigidBody};
use ncollide3d::shape;
use ncollide3d::shape::ShapeHandle;
use ncollide3d::math::Point;
use na::Vector3;
use na::geometry::Point2;



// We implement the Clone trait to the structure
#[derive(Clone)]
pub struct Polyline
{
    pub points: Vec<Point<f32>>,
    pub indices: Option<Vec<Point2<usize>>>
}

impl Polyline{
    /// Creates and returns a RigidBody corresponding to the 'Polyline' type
    pub fn process_polyline(polyline: Polyline, position: Coordinates) -> (RigidBody<f32>, ShapeHandle<f32>){
        // Coordinates, points and indices of the Polyline
        let x = position.x;
        let y = position.y;
        let z = position.z;
        let points = polyline.points;
        let indices = polyline.indices;

        // Creation of a Polyline we'll need later to make a Collider
        let polyl = ShapeHandle::new(shape::Polyline::new(points, indices));

        // Creation of the Polyline's RigidBody
        let rb = RigidBodyDesc::new()
            .translation(Vector3::new(x, y, z))
            .build();

        return (rb, polyl);
    }
}