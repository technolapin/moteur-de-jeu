use crate::misc::Coordinates;
use nphysics3d::object::{RigidBodyDesc, RigidBody};
use ncollide3d::shape;
use ncollide3d::shape::ShapeHandle;
use na::Vector3;



// We implement the Clone trait to the structure
#[derive(Clone)]
pub struct Ball
{
    pub radius: f32
}

impl Ball{
    /// Creates and returns a RigidBody corresponding to the 'Ball' type
    pub fn process_ball(ball: Ball, position: Coordinates) -> (RigidBody<f32>, ShapeHandle<f32>){
        // Coordinates and radius of the Ball
        let x = position.x;
        let y = position.y;
        let z = position.z;
        let radius = ball.radius;

        // Creation of a Ball we'll need later to make a Collider
        let bal = ShapeHandle::new(shape::Ball::new(radius));

        // Creation of the Ball's RigidBody
        let rb = RigidBodyDesc::new()
            .translation(Vector3::new(x, y, z))
            .build();

        return (rb, bal);
    }
}