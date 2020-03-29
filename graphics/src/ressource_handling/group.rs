use glium::vertex::VertexBufferAny;
use std::sync::Arc;
use super::Material;

/**
This structure represents a group of vertex and their associated material.
It doesn't own the date, rather it owns a shared pointer to the data.
 */
#[derive(Debug, Clone)]
pub struct Group {
    pub vertexes: Arc<VertexBufferAny>,
    pub material: Arc<Material>,
}
