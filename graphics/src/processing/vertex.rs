use glium::implement_vertex;


#[derive(Copy, Clone)]
/**
The format of the Vertexes that will be passed to the GPU
*/
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub texture: [f32; 2],
}

implement_vertex!(Vertex, position, normal, texture);

