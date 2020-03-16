use super::{Object, Wavefront};
use crate::engine::Graphical;

use std::collections::HashMap;
use std::path::Path;

/**
Owns the all Objects imported.
This is what the user is supposed to use.
 */
#[derive(Debug)]
pub struct RessourcesHolder {
    wavefronts: WavefrontsHolder,
}

#[derive(Debug)]
pub struct WavefrontsHolder(HashMap<String, Wavefront>);

impl<'a> WavefrontsHolder {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /**
    Check if the corresponding source of object is known and then tries to fetch an Object struct.
     */
    pub fn get_object(&self, file: &str, model_name: &str) -> Option<Object> {
        match self.0.get(file) {
            None => None,
            Some(wavefront) => wavefront.get_object_checked(model_name.to_string()),
        }
    }

    pub fn get_whole_content(&self, file: &str) -> Option<Vec<Object>> {
        match self.0.get(file) {
            None => None,
            Some(objects) => Some({
                objects
                    .objects
                    .keys()
                    .map(|obj_name| self.get_object(file, obj_name).unwrap())
                    .collect::<Vec<_>>()
            }),
        }
    }

    /**
    Tries to import a wavefront.
     */
    pub fn load_wavefront(
        &mut self,
        gr: &Graphical,
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
            Wavefront::new(gr, path_to_wavefront, &path_to_mtl, ressources_path),
        );
        Ok(())
    }

    /** remove from the graphical memory a file**/
    pub fn unload(&mut self, filename: &str) -> Result<(), &'static str> {
        if self.0.contains_key(filename) {
            self.0.remove(filename);
            return Ok(());
        } else {
            return Err("file not found");
        }
    }
}

impl RessourcesHolder {
    pub fn new() -> Self {
        Self {
            wavefronts: WavefrontsHolder::new(),
        }
    }

    pub fn get_object(&self, file: &str, model_name: &str) -> Option<Object> {
        self.wavefronts.get_object(file, model_name)
    }
    pub fn get_whole_content(&self, file: &str) -> Option<Vec<Object>> {
        self.wavefronts.get_whole_content(file)
    }
    pub fn load_wavefront(
        &mut self,
        gr: &Graphical,
        filename: &str,
        ressources_path: &Path,
    ) -> Result<(), &'static str> {
        self.wavefronts
            .load_wavefront(gr, filename, ressources_path)
    }

    pub fn unload(&mut self, filename: &str) -> Result<(), &'static str> {
        self.wavefronts.unload(filename)
    }
}
