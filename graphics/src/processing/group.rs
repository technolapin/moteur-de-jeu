use glium::vertex::VertexBufferAny;


/**
 * This structure represents a group of vertex
 * and the name their associated material.
 */
#[derive(Debug)]
pub struct Group
{
    pub voxels: VertexBufferAny,
    pub material: Option<String>
}
