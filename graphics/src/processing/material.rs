use glium::texture::Texture2d;

#[derive(Debug)]
pub enum Material
{
    Textured // incomplets
    {
        texture: Texture2d,
        specular_color: [f32; 3],
        specular_exponent: f32,
        opacity: f32
    },
    NonTextured
    {
        ambiant_color: [f32; 3],
        diffuse_color: [f32; 3],
        specular_color: [f32; 3],
        specular_exponent: f32,
        emission_color: [f32; 3],
        opacity: f32
        
    },
    Default
}
