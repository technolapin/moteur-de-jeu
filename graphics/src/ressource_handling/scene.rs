use crate::misc::Similarity;
use glium::vertex::VertexBuffer;
use super::Object;
/**
All the meshes and their instancings.
*/
pub struct Scene<'a> {
    pub objects: Vec<(Vec<Object<'a>>, VertexBuffer<Similarity>)>,
}

impl<'a> Scene<'a> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, meshes: Vec<Object<'a>>, instances: VertexBuffer<Similarity>) {
        self.objects.push((meshes, instances));
    }

    /*
    pub fn add_2d(&mut self, (x, y, w, h): (f32, f32, f32, f32), depth: f32, instances: VertexBuffer<Similarity>)
    {
	let mut mesh = vec![
	    Vertex{position: [x, y, depth], .. Default::default()}
	];
    }
*/
}
