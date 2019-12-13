use glium::vertex::VertexBufferAny;

#[derive(Debug)]
pub struct Group
{
    pub voxels: VertexBufferAny,
    pub material: Option<String>
}
