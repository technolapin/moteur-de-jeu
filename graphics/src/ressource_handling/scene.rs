use crate::misc::Similarity;
use glium::vertex::VertexBuffer;
use super::Object;
/**
A scene (contain references to the RessourcesHolder)
*/
pub struct Scene<'a> {
    pub objects: Vec<(Vec<Object<'a>>, VertexBuffer<Similarity>)>,
}

impl<'a> Scene<'a> {
    /// creates a scene
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    /// Adds some objects to the scene
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
