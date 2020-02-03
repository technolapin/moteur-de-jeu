use crate::misc::Coordinates;
use nphysics3d::object::{RigidBodyDesc, RigidBody};
use ncollide3d::shape;
use ncollide3d::shape::ShapeHandle;
use ncollide3d::math::Point;
use na::Vector3;



// We implement the Clone trait to the structure
#[derive(Clone)]
pub struct Triangle
{
    pub a: Point<f32>,
    pub b: Point<f32>,
    pub c: Point<f32>
}

impl Triangle{
    /// Creates and returns a RigidBody corresponding to the 'Triangle' type
    pub fn process_triangle(triangle: Triangle, position: Coordinates) -> (RigidBody<f32>, ShapeHandle<f32>){
        // Coordinates and points of the Triangle
        let x = position.x;
        let y = position.y;
        let z = position.z;
        let a = triangle.a;
        let b = triangle.b;
        let c = triangle.c;

        // Creation of a Triangle we'll need later to make a Collider
        let tri = ShapeHandle::new(shape::Triangle::new(a, b, c));

        // Creation of the Triangle's RigidBody
        let rb = RigidBodyDesc::new()
            .translation(Vector3::new(x, y, z))
            .build();

        return (rb, tri);
    }
}