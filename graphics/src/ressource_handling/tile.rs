use std::path::PathBuf;
use glium::texture::{RawImage2d, Texture2d};
use std::sync::Arc;

use super::Material;
use crate::engine::Display;
use base::{Base, EngineError};

/**
 A tile is a 2D rectangle with an image on it.
Stores the proportions of the image and the image itselve.
*/
#[derive(Debug, Clone)]
pub struct Tile
{
    pub texture: Arc<Material>,
    pub dims: (f32, f32)
}


impl Tile
{
    /// Creates a new Tile from the given image path
    pub fn new(base: &Base, display: &Display, image_path: PathBuf) -> Result<Self, EngineError>
    {
        let image = base.open_image(image_path)?
	    .to_rgba();
        
        let (x, y) = image.dimensions();
        let max = x.max(y) as f32;
        let dims = ((x as f32)/max, (y as f32)/max);
        let image =
            RawImage2d::from_raw_rgba_reversed(
                &image.into_raw(),
                (x, y)
            );
        
        let texture = Texture2d::new(
            &display.display,
            image
        )?;
        let mat = Material::Textured
        {
            texture: texture,
            specular_color: [0.; 3],
            specular_exponent: 0.,
            opacity: 0.
        };
        Ok(
            Self
            {
                texture: Arc::new(mat),
                dims: dims
            }

        )

    }
}
