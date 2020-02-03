use ncollide3d::shape;
use ncollide3d::shape::ShapeHandle;



// We implement the Clone trait to the structure
#[derive(Clone)]
pub struct Ball
{
    pub radius: f32
}

impl Ball{
    /// Creates and returns a RigidBody corresponding to the 'Ball' type
    pub fn process_ball(ball: Ball) -> ShapeHandle<f32>{
        // Radius of the Ball
        let radius = ball.radius;

        // Creation of a Ball we'll need later to make a RigidBody and Collider
        let bal = ShapeHandle::new(shape::Ball::new(radius));

        return bal;
    }
}