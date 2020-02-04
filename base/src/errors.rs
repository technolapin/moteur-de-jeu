use std::io;
//use std::option;

#[derive(Debug)]
pub enum EngineError
{
    IOError(io::Error),
    NoneError // experimental, not yet used
    
}


impl From<io::Error> for EngineError {
    fn from(error: io::Error) -> Self {
        Self::IOError(error)
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
