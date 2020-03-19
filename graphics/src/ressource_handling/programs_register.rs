use std::collections::HashMap;
use base::EngineError;

use crate::engine::ProgramId;

/**
Map the names of shaders programs to their ProgramId
*/
#[derive(Debug)]
pub struct ProgramsRegister(HashMap<String, ProgramId>);


impl ProgramsRegister {
    /// Creates a new ProgramRegister
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Add an entry
    pub fn register(&mut self, id: ProgramId, name: String) {
        self.0.insert(name, id);
    }

    /// Returns the ProgramId corresponding to the String given
    pub fn get(&self, name: String) -> Result<ProgramId, EngineError> {
        match self.0.get(&name) {
            Some(thing) => Ok(*thing),
            None => Err(EngineError::NoneError),
        }
    }
}
