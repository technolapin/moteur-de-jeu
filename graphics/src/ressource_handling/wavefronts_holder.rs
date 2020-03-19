use std::collections::HashMap;
use base::EngineError;
use std::path::Path;
use super::objects::Wavefront;
use super::material::Material;
use crate::engine::display::Display;
use glium::vertex::VertexBufferAny;


/// Own the wavefront files datas
#[derive(Debug)]
pub struct WavefrontsHolder(HashMap<String, Wavefront>);

impl<'a, 'b> WavefrontsHolder {
    /// Creates a new WavefrontsHolder
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /**
    Fetch the data of a 3D object
     */
    pub fn get_object(
        &self,
        file: &str,
        model_name: &str,
    ) -> Result<Vec<(&VertexBufferAny, &Material)>, EngineError> {
        match self.0.get(file) {
            Some(wavefront) => wavefront.get_object_checked(model_name.to_string()),
            None => Err(EngineError::NoneError),
        }
    }

    /**
    Fetch the data of all 3D objects in a file
     */
    pub fn get_whole_content(
        &self,
        file: &str,
    ) -> Result<Vec<(&VertexBufferAny, &Material)>, EngineError> {
        match self.0.get(file) {
            None => Err(EngineError::NoneError),
            Some(wavefront) => Ok({
                wavefront
                    .objects
                    .keys()
                    .map(|obj_name| self.get_object(file, obj_name).unwrap())
                    .flatten()
                    .collect::<Vec<_>>()
            }),
        }
    }

    /**
    Tries to import a wavefront.
     */
    pub fn load_wavefront(
        &mut self,
        disp: &Display,
        filename: &str,
        ressources_path: &Path,
    ) -> Result<(), &'static str> {
        let path_to_wavefront = Path::new(filename);
        let stem = match path_to_wavefront.file_stem() {
            Some(st) => st,
            _ => return Err("cannot parse stem, is the address empty?"),
        };
        let stem = match stem.to_str() {
            Some(st) => st,
            _ => return Err("cannot parse stem, is the address empty?"),
        };
        let stem = stem.to_string();
        let path_to_mtl = path_to_wavefront.with_extension("mtl");

        println!("stem: {:?}", stem);
        println!("mtl: {:?}", path_to_mtl);

        self.0.insert(
            stem.clone(),
            Wavefront::new(disp, path_to_wavefront, &path_to_mtl, ressources_path),
        );
        Ok(())
    }

    /** remove a file from the graphical memory**/
    pub fn unload(&mut self, filename: &str) -> Result<(), &'static str> {
        if self.0.contains_key(filename) {
            self.0.remove(filename);
            return Ok(());
        } else {
            return Err("file not found");
        }
    }
}
