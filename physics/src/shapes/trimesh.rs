use crate::misc::Coordinates;
use nphysics3d::object::{RigidBodyDesc, RigidBody};
use ncollide3d::shape;
use ncollide3d::shape::ShapeHandle;
use ncollide3d::math::Point;
use na::Vector3;
use na::geometry::{Point2, Point3};



// We implement the Clone trait to the structure
#[derive(Clone)]
pub struct TriMesh
{
    pub points: Vec<Point<f32>>,
    pub indices: Vec<Point3<usize>>,
    pub uvs: Option<Vec<Point2<f32>>>
}

impl TriMesh{
    /// Creates and returns a RigidBody corresponding to the 'TriMesh' type
    pub fn process_trimesh(trimesh: TriMesh, position: Coordinates) -> (RigidBody<f32>, ShapeHandle<f32>){
        // Coordinates, points, indices and uvs of the TriMesh
        let x = position.x;
        let y = position.y;
        let z = position.z;
        let points = trimesh.points;
        let indices = trimesh.indices;
        let uvs = trimesh.uvs;

        // Creation of a TriMesh we'll need later to make a Collider
        let trim = ShapeHandle::new(shape::TriMesh::new(points, indices, uvs));

        // Creation of the TriMesh's RigidBody
        let rb = RigidBodyDesc::new()
            .translation(Vector3::new(x, y, z))
            .build();

        return (rb, trim);
    }
}