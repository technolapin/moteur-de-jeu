use glium::vertex::VertexBufferAny;
use std::sync::Arc;
use super::Material;

/**
 * This structure represents a group of vertex
 * and the name their associated material.
 */
#[derive(Debug, Clone)]
pub struct Group {
    pub vertexes: Arc<VertexBufferAny>,
    pub material: Arc<Material>,
}
