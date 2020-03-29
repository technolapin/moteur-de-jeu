use glium::texture::Texture2d;

/**
This structure represents a material in the wavefront sense.
*/
#[derive(Debug)]
pub enum Material
{
    /// The materials owning a texture
    Textured // incomplets
    {
        texture: Texture2d,
        specular_color: [f32; 3],
        specular_exponent: f32,
        opacity: f32
    },
    /// The materials not owning a texture
    NonTextured
    {
        ambiant_color: [f32; 3],
        diffuse_color: [f32; 3],
        specular_color: [f32; 3],
        specular_exponent: f32,
        emission_color: [f32; 3],
        opacity: f32
        
    },
    /// Used to replace unrecognized materials.
    Default
}
