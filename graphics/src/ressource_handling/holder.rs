use std::collections::HashMap;

#[derive(Debug)]
pub struct Holder<T> (HashMap<String, T>);

impl<T> Holder<T>
{
    pub fn new() -> Self
    {
        Self(HashMap::new())
    }

    pub fn get(&self, s: &str) -> Option<&T>
    {
        self.0.get(s)
    }

    pub fn insert(&mut self, s: String, el: T)
    {
        self.0.insert(s, el);
    }
    
    
}

use super::Wavefront;

use std::path::Path;
use base::EngineError;
use super::Group;
use crate::engine::Display;
impl Holder<Wavefront>
{
     /**
    Fetch the data of a 3D object
     */
    pub fn get_object(
        &self,
        file: &str,
        model_name: &str,
    ) -> Result<Vec<(Group)>, EngineError> {
        match self.get(file) {
            Some(wavefront) => wavefront.get_object(model_name.to_string()),
            None => EngineError::new("file doesn't exist!"),
        }
    }

    /**
    Fetch the data of all 3D objects in a file
     */
    pub fn get_whole_content(
        &self,
        file: &str,
    ) -> Result<Vec<Group>, EngineError> {
        match self.get(file) {
            None => EngineError::new("file doesn't exist!"),
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

        self.insert(
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

