use crate::misc::Coordinates;
use nphysics3d::object::{RigidBodyDesc, RigidBody};
use ncollide3d::shape;
use ncollide3d::shape::ShapeHandle;
use ncollide3d::math::Point;
use na::Vector3;



// We implement the Clone trait to the structure
#[derive(Clone)]
pub struct Segment
{
    pub a: Point<f32>,
    pub b: Point<f32>
}

impl Segment{
    /// Creates and returns a RigidBody corresponding to the 'Segment' type
    pub fn process_segment(segment: Segment, position: Coordinates) -> (RigidBody<f32>, ShapeHandle<f32>){
        // Coordinates and points of the Segment
        let x = position.x;
        let y = position.y;
        let z = position.z;
        let a = segment.a;
        let b = segment.b; 

        // Creation of a Segment we'll need later to make a Collider
        let seg = ShapeHandle::new(shape::Segment::new(a, b));

        // Creation of the Segment's RigidBody
        let rb = RigidBodyDesc::new()
            .translation(Vector3::new(x, y, z))
            .build();

        return (rb, seg);
    }
}