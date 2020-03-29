use glium::implement_vertex;


#[derive(Copy, Clone, Default)]
/**
The format of the Vertexes that will be passed to the GPU
*/
pub struct Vertex {
    /// The position of the vertex relative to the center of the object
    pub position: [f32; 3],
    /// The normal of the vertex
    pub normal: [f32; 3],
    /// The coordinates of the vertex in its eventual texture map
    pub texture: [f32; 2],
}

implement_vertex!(Vertex, position, normal, texture);

