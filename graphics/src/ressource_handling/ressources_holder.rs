use super::group::*;
use super::material::*;
use super::{Object, Wavefront};
use crate::engine::Display;
use base::Base;
use base::EngineError;
use std::collections::HashMap;
use std::path::Path;

use crate::engine::programs::ProgramsHolder;
use glium::vertex::{VertexBuffer, VertexBufferAny};

/**
Owns the all Objects imported.
This is what the user is supposed to use.
 */
#[derive(Debug)]
pub struct RessourcesHolder {
    wavefronts: WavefrontsHolder,
    pub programs: ProgramsRegister,
}

#[derive(Debug)]
pub struct WavefrontsHolder(HashMap<String, Wavefront>);

impl<'a, 'b> WavefrontsHolder {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /**
    Check if the corresponding source of object is known and then tries to fetch an Object struct.
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
            programs: ProgramsRegister::new(),
        }
    }

    pub fn get_object(&self, file: &str, model_name: &str) -> Result<Object, EngineError> {
        let shape_data = self.wavefronts.get_object(file, model_name);
        let whole_data = shape_data?
            .iter()
            .map(|(group, material)| {
                (
                    *group,
                    *material,
                    match material {
                        Material::Default => self.programs.get("default".to_string()).unwrap(),
                        Material::Textured { .. } => {
                            self.programs.get("textured".to_string()).unwrap()
                        }
                        Material::NonTextured { .. } => {
                            self.programs.get("nontextured".to_string()).unwrap()
                        }
                    },
                )
            })
            .collect::<Vec<_>>();
        Ok(Object { groups: whole_data })
    }

    pub fn get_whole_content(&self, file: &str) -> Result<Object, EngineError> {
        let shape_data = self.wavefronts.get_whole_content(file);
        let whole_data = shape_data?
            .iter()
            .map(|(group, material)| {
                (
                    *group,
                    *material,
                    match material {
                        Material::Default => self.programs.get("default".to_string()).unwrap(),
                        Material::Textured { .. } => {
                            self.programs.get("textured".to_string()).unwrap()
                        }
                        Material::NonTextured { .. } => {
                            self.programs.get("nontextured".to_string()).unwrap()
                        }
                    },
                )
            })
            .collect::<Vec<_>>();
        Ok(Object { groups: whole_data })
    }

    pub fn load_wavefront(
        &mut self,
        disp: &Display,
        filename: &str,
        ressources_path: &Path,
    ) -> Result<(), &'static str> {
        self.wavefronts
            .load_wavefront(disp, filename, ressources_path)
    }

    pub fn unload(&mut self, filename: &str) -> Result<(), &'static str> {
        self.wavefronts.unload(filename)
    }
}

pub use crate::engine::ProgramId;
#[derive(Debug)]
pub struct ProgramsRegister(HashMap<String, ProgramId>);

impl ProgramsRegister {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn register(&mut self, id: ProgramId, name: String) {
        self.0.insert(name, id);
    }

    pub fn get(&self, name: String) -> Result<ProgramId, EngineError> {
        match self.0.get(&name) {
            Some(thing) => Ok(*thing),
            None => Err(EngineError::NoneError),
        }
    }
}
