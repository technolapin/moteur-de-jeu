use super::material::*;
use super::programs_register::ProgramsRegister;
use super::wavefronts_holder::WavefrontsHolder;
use super::Object;
use crate::engine::programs::ProgramId;
use crate::engine::Display;
use base::EngineError;
use std::path::Path;

/**
Owns the all Objects imported.
This is what the user is supposed to use.
 */
#[derive(Debug)]
pub struct RessourcesHolder {
    wavefronts: WavefrontsHolder,
    programs: ProgramsRegister,
}

impl RessourcesHolder {
    /// Create a new RessourcesHolder
    pub fn new() -> Self {
        Self {
            wavefronts: WavefrontsHolder::new(),
            programs: ProgramsRegister::new(),
        }
    }

    pub fn register_program(&mut self, pgrm: ProgramId, name: String) {
        self.programs.register(pgrm, name);
    }
    pub fn get_program(&mut self, name: String) -> Result<ProgramId, EngineError> {
        self.programs.get(name)
    }

    /// Fetch a displayable object from a wavefront file name and the name of the requested object.
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

    /// Like get_object, but the object fetched is the union of all the objects of the wavefront file.
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

    /// Loads a wavefront file
    pub fn load_wavefront(
        &mut self,
        disp: &Display,
        filename: &str,
        ressources_path: &Path,
    ) -> Result<(), &'static str> {
        self.wavefronts
            .load_wavefront(disp, filename, ressources_path)
    }

    /// Unloads a wavefront file.
    pub fn unload(&mut self, filename: &str) -> Result<(), &'static str> {
        self.wavefronts.unload(filename)
    }
}
