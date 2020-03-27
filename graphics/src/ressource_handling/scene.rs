use crate::misc::{Similarity, new_vertexbuffer};
use super::Object;
use crate::engine::{Graphical, Frame};
/**
A scene (contain references to the RessourcesHolder)
*/
pub struct Scene {
    objects: Vec<(Vec<Object>, Vec<Similarity>)>,
}


impl Scene {
    /// creates a scene
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    /// Adds some objects to the scene
    pub fn add(&mut self, meshes: Vec<Object>, instances: Vec<Similarity>) {
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

    pub fn render(&self, gr: &Graphical, frame: &mut Frame)
    {
        self.objects.iter().for_each(|(objects, instances)| {
            let vbo = new_vertexbuffer(&gr.display, instances);
            objects
                .iter()
                .for_each(|ob|
                          frame.draw(&gr, &ob, &vbo)
                )
        });

    }
    
}
