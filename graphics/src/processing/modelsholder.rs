use super::{Object, Objects};
use crate::engine::Graphical;

use std::collections::HashMap;
use std::path::Path;

/**
Owns the all Objects imported.
This is what the user is supposed to use.
 */
#[derive(Debug)]
pub struct ModelsHolder {
    wavefronts: HashMap<String, Objects>,
}

impl<'a> ModelsHolder {
    pub fn new() -> Self {
        Self {
            wavefronts: HashMap::new(),
        }
    }

    /**
    Check if the corresponding source of object is known and then tries to fetch an Object struct.
     */
    pub fn get(&self, file: &str, model_name: &str) -> Option<Object> {
        match self.wavefronts.get(file) {
            None => None,
            Some(wavefront) => wavefront.get_object_checked(model_name.to_string()),
        }
    }

    pub fn get_whole_file(&self, file: &str) -> Option<Vec<Object>> {
        match self.wavefronts.get(file) {
            None => None,
            Some(objects) => Some({
                objects
                    .objects
                    .keys()
                    .map(|obj_name| self.get(file, obj_name).unwrap())
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

        self.wavefronts.insert(
            stem.clone(),
            Objects::new(gr, path_to_wavefront, &path_to_mtl, ressources_path),
        );
        Ok(())
    }

    /** remove from the graphical memory a file**/
    pub fn unload(&mut self, filename: &str) -> Result<(), &'static str> {
        if self.wavefronts.contains_key(filename) {
            self.wavefronts.remove(filename);
            return Ok(());
        } else {
            return Err("file not found");
        }
    }
}
