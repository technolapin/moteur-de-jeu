use std::io;
//use std::option;

#[derive(Debug)]
pub enum EngineError
{
    IOError(io::Error),
    Misc(String),
    NoneError // experimental, not yet used
    
}


impl From<io::Error> for EngineError {
    fn from(error: io::Error) -> Self {
        Self::IOError(error)
    }
}

impl From<glium::ProgramCreationError> for EngineError {
    fn from(error: glium::ProgramCreationError) -> Self {
        Self::Misc(format!("{:?}", error))
    }
}
impl From<image::error::ImageError> for EngineError {
    fn from(error: image::error::ImageError) -> Self {
        Self::Misc(format!("{:?}", error))
    }
}

/*
// only in nightly rust
impl From<option::NoneError> for EngineError {
    fn from(error: option::NoneError) -> Self {
        Self::NoneError
    }
}
*/

/// Converts option to results, permit to use the ? notation in non-nighly rust
pub fn option_unwrap<T>(option: Option<T>) -> Result<T, EngineError>
{
    match option
    {
        Some(thing) => Ok(thing),
        None => Err(EngineError::NoneError)
    }
}
